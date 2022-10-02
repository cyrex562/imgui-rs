#![allow(non_snake_case)]

use std::ptr::null_mut;
use libc::c_float;
use crate::color::IM_COL32;
use crate::draw_flags::ImDrawFlags_None;
use crate::draw_list_ops::GetForegroundDrawList;
use crate::hovered_flags::{ImGuiHoveredFlags, ImGuiHoveredFlags_AllowWhenBlockedByActiveItem, ImGuiHoveredFlags_AllowWhenDisabled, ImGuiHoveredFlags_AllowWhenOverlapped, ImGuiHoveredFlags_DelayNormal, ImGuiHoveredFlags_DelayShort, ImGuiHoveredFlags_NoNavOverride, ImGuiHoveredFlags_None, ImGuiHoveredFlags_NoSharedDelay};
use crate::id_ops::{ClearActiveID, SetHoveredID};
use crate::imgui::GImGui;
use crate::input_ops::{IsMouseClicked, IsMouseHoveringRect};
use crate::item_flags::{ImGuiItemFlags, ImGuiItemFlags_Disabled};
use crate::item_status_flags::{ImGuiItemStatusFlags, ImGuiItemStatusFlags_Deactivated, ImGuiItemStatusFlags_Edited, ImGuiItemStatusFlags_HasDeactivated, ImGuiItemStatusFlags_HoveredRect, ImGuiItemStatusFlags_HoveredWindow, ImGuiItemStatusFlags_ToggledOpen, ImGuiItemStatusFlags_ToggledSelection};
use crate::key::{ImGuiKey_MouseWheelX, ImGuiKey_MouseWheelY};
use crate::mouse_button::ImGuiMouseButton;
use crate::rect::ImRect;
use crate::type_defs::ImGuiID;
use crate::utils::flag_set;
use crate::vec2::ImVec2;
use crate::window_ops::IsWindowContentHoverable;

// c_void MarkItemEdited(ImGuiID id)
pub unsafe fn MarkItemEdited(id: ImGuiID)
{
    // This marking is solely to be able to provide info for IsItemDeactivatedAfterEdit().
    // ActiveId might have been released by the time we call this (as in the typical press/release button behavior) but still need need to fill the data.
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.ActiveId == id || g.ActiveId == 0 || g.DragDropActive);
    // IM_UNUSED(id); // Avoid unused variable warnings when asserts are compiled out.
    //IM_ASSERT(g.Currentwindow.DC.LastItemId == id);
    g.ActiveIdHasBeenEditedThisFrame = true;
    g.ActiveIdHasBeenEditedBefore = true;
    g.LastItemData.StatusFlags |= ImGuiItemStatusFlags_Edited;
}

// == GetItemID() == GetFocusID()
// bool IsItemFocused()
pub unsafe fn IsItemFocused() -> bool
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if g.NavId != g.LastItemData.ID || g.NavId == 0 {
        return false;
    }

    // Special handling for the dummy item after Begin() which represent the title bar or tab.
    // When the window is collapsed (SkipItems==true) that last item will never be overwritten so we need to detect the case.
    let mut window = g.CurrentWindow;
    if g.LastItemData.ID == window.ID && window.WriteAccessed {
        return false;
    }

    return true;
}


// This is roughly matching the behavior of internal-facing ItemHoverable()
// - we allow hovering to be true when ActiveId==window.MoveID, so that clicking on non-interactive items such as a Text() item still returns true with IsItemHovered()
// - this should work even for non-interactive items that have no ID, so we cannot use LastItemId
// bool IsItemHovered(ImGuiHoveredFlags flags)
pub unsafe fn IsItemHovered(flags: ImGuiHoveredFlags) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if g.NavDisableMouseHover && !g.NavDisableHighlight && (flags & ImGuiHoveredFlags_NoNavOverride) == 0 {
        if (g.LastItemData.InFlags & ImGuiItemFlags_Disabled) && !(flags & ImGuiHoveredFlags_AllowWhenDisabled) {
            return false;
        }
        if !IsItemFocused() {
            return false;
        }
    } else {
        // Test for bounding box overlap, as updated as ItemAdd()
        let mut status_flags: ImGuiItemStatusFlags = g.LastItemData.StatusFlags;
        if !(status_flags & ImGuiItemStatusFlags_HoveredRect) {
            return false;
        }
        // IM_ASSERT((flags & (ImGuiHoveredFlags_AnyWindow | ImGuiHoveredFlags_RootWindow | ImGuiHoveredFlags_ChildWindows | ImGuiHoveredFlags_NoPopupHierarchy | ImGuiHoveredFlags_DockHierarchy)) == 0);   // Flags not supported by this function

        // Test if we are hovering the right window (our window could be behind another window)
        // [2021/03/02] Reworked / reverted the revert, finally. Note we want e.g. BeginGroup/ItemAdd/EndGroup to work as well. (#3851)
        // [2017/10/16] Reverted commit 344d48be3 and testing RootWindow instead. I believe it is correct to NOT test for RootWindow but this leaves us unable
        // to use IsItemHovered() after EndChild() itself. Until a solution is found I believe reverting to the test from 2017/09/27 is safe since this was
        // the test that has been running for a long while.
        if g.HoveredWindow != window && (status_flags & ImGuiItemStatusFlags_HoveredWindow) == 0 {
            if (flags & ImGuiHoveredFlags_AllowWhenOverlapped) == 0 {
                return false;
            }
        }

        // Test if another item is active (e.g. being dragged)
        if (flags & ImGuiHoveredFlags_AllowWhenBlockedByActiveItem) == 0 {
            if g.ActiveId != 0 && g.ActiveId != g.LastItemData.ID && !g.ActiveIdAllowOverlap {
                if g.ActiveId != window.MoveId && g.ActiveId != window.TabId {
                    return false;
                }
            }
        }

        // Test if interactions on this window are blocked by an active popup or modal.
        // The ImGuiHoveredFlags_AllowWhenBlockedByPopup flag will be tested here.
        if !IsWindowContentHoverable(window, flags) {
            return false;
        }

        // Test if the item is disabled
        if (g.LastItemData.InFlags & ImGuiItemFlags_Disabled) && !(flags & ImGuiHoveredFlags_AllowWhenDisabled) {
            return false;
        }

        // Special handling for calling after Begin() which represent the title bar or tab.
        // When the window is skipped/collapsed (SkipItems==true) that last item (always ->MoveId submitted by Begin)
        // will never be overwritten so we need to detect the case.
        if g.LastItemData.ID == window.MoveId && window.WriteAccessed {
            return false;
        }
    }

    // Handle hover delay
    // (some ideas: https://www.nngroup.com/articles/timing-exposing-content)
    let mut delay: c_float = 0f32;
    if flags & ImGuiHoveredFlags_DelayNormal {
        delay = g.IO.HoverDelayNormal;
    } else if flags & ImGuiHoveredFlags_DelayShort {
        delay = g.IO.HoverDelayShort;
    } else {
        delay = 0f32;
    }
    if delay > 0f32 {
        let mut hover_delay_id: ImGuiID = if g.LastItemData.ID != 0 { g.LastItemData.ID } else { window.GetIDFromRectangle(&g.LastItemData.Rect) };
        if (flags & ImGuiHoveredFlags_NoSharedDelay) != 0 && (g.HoverDelayIdPreviousFrame != hover_delay_id) {
            g.HoverDelayTimer = 0f32;
        }
        g.HoverDelayId = hover_delay_id;
        return g.HoverDelayTimer >= delay;
    }

    return true;
}

// Internal facing ItemHoverable() used when submitting widgets. Differs slightly from IsItemHovered().
// bool ItemHoverable(const ImRect& bb, ImGuiID id)
pub unsafe fn ItemHoverable(bb: &ImRect, id: ImGuiID) -> bool
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if g.HoveredId != 0 && g.HoveredId != id && !g.HoveredIdAllowOverlap {
        return false;
    }

    let mut window = g.CurrentWindow;
    if g.HoveredWindow != window {
        return false;
    }
    if g.ActiveId != 0 && g.ActiveId != id && !g.ActiveIdAllowOverlap {
        return false;
    }
    if !IsMouseHoveringRect(&bb.Min, &bb.Max, false) {
        return false;
    }
    if !IsWindowContentHoverable(window, ImGuiHoveredFlags_None)
    {
        g.HoveredIdDisabled = true;
        return false;
    }

    // We exceptionally allow this function to be called with id==0 to allow using it for easy high-level
    // hover test in widgets code. We could also decide to split this function is two.
    if id != 0 {
        SetHoveredID(id);
    }

    // When disabled we'll return false but still set HoveredId
    let mut item_flags: ImGuiItemFlags =  if g.LastItemData.ID == id { g.LastItemData.InFlags } else { g.CurrentItemFlags };
    if item_flags & ImGuiItemFlags_Disabled
    {
        // Release active id if turning disabled
        if g.ActiveId == id {
            ClearActiveID();
        }
        g.HoveredIdDisabled = true;
        return false;
    }

    if id != 0
    {
        // [DEBUG] Item Picker tool!
        // We perform the check here because SetHoveredID() is not frequently called (1~ time a frame), making
        // the cost of this tool near-zero. We can get slightly better call-stack and support picking non-hovered
        // items if we perform the test in ItemAdd(), but that would incur a small runtime cost.
        // #define IMGUI_DEBUG_TOOL_ITEM_PICKER_EX in imconfig.h if you want this check to also be performed in ItemAdd().
        if g.DebugItemPickerActive && g.HoveredIdPreviousFrame == id {
            GetForegroundDrawList(null_mut()).AddRect(&bb.Min, &bb.Max, IM_COL32(255, 255, 0, 255), 0f32, ImDrawFlags_None, 0f32);
        }
        if g.DebugItemPickerBreakId == id {
            IM_DEBUG_BREAK();
        }
    }

    if g.NavDisableMouseHover {
        return false;
    }

    return true;
}

// bool IsClippedEx(const ImRect& bb, ImGuiID id)
pub unsafe fn IsClippedEx(bb: &mut ImRect, id: ImGuiID) -> bool
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if !bb.Overlaps(&window.ClipRect) {
        if id == 0 || (id != g.ActiveId && id != g.NavId) {
            if !g.LogEnabled {
                return true;
            }
        }
    }
    return false;
}


// This is also inlined in ItemAdd()
// Note: if ImGuiItemStatusFlags_HasDisplayRect is set, user needs to set window.DC.LastItemDisplayRect!
// c_void SetLastItemData(ImGuiID item_id, ImGuiItemFlags in_flags, ImGuiItemStatusFlags item_flags, const ImRect& item_rect)
pub unsafe fn SetLastItemData(item_id: ImGuiID, in_flags: ImGuiItemFlags, item_flags: ImGuiItemStatusFlags, item_rect: &ImRect) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.LastItemData.ID = item_id;
    g.LastItemData.InFlags = in_flags;
    g.LastItemData.StatusFlags = item_flags;
    g.LastItemData.Rect = item_rect.clone();
}

// c_float CalcWrapWidthForPos(const ImVec2& pos, c_float wrap_pos_x)
pub unsafe fn CalcWrapWidthForPos(pos: &ImVec2, mut wrap_pos_x: c_float) -> c_float
{
    if wrap_pos_x < 0f32 {
        return 0f32;
    }

    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if wrap_pos_x == 0f32
    {
        // We could decide to setup a default wrapping max point for auto-resizing windows,
        // or have auto-wrap (with unspecified wrapping pos) behave as a ContentSize extending function?
        //if (window.Hidden && (window.Flags & ImGuiWindowFlags_AlwaysAutoResize))
        //    wrap_pos_x = ImMax(window.WorkRect.Min.x + g.FontSize * 10f32, window.WorkRect.Max.x);
        //else
        wrap_pos_x = window.WorkRect.Max.x;
    }
    else if wrap_pos_x > 0f32
    {
        wrap_pos_x += window.Pos.x - window.Scroll.x; // wrap_pos_x is provided is window local space
    }

    return ImMax(wrap_pos_x - pos.x, 1f32);
}



// bool IsItemActive()
pub unsafe fn IsItemActive() -> bool
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if g.ActiveId {
        return g.ActiveId == g.LastItemData.ID;
    }
    return false;
}

// bool IsItemActivated()
pub unsafe fn IsItemActivated() -> bool
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if g.ActiveId {
        if g.ActiveId == g.LastItemData.ID && g.ActiveIdPreviousFrame != g.LastItemData.ID {
            return true;
        }
    }
    return false;
}

// bool IsItemDeactivated()
pub unsafe fn IsItemDeactivated() -> bool
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if g.LastItemData.StatusFlags & ImGuiItemStatusFlags_HasDeactivated {
        return (g.LastItemData.StatusFlags & ImGuiItemStatusFlags_Deactivated) != 0;
    }
    return g.ActiveIdPreviousFrame == g.LastItemData.ID && g.ActiveIdPreviousFrame != 0 && g.ActiveId != g.LastItemData.ID;
}

// bool IsItemDeactivatedAfterEdit()
pub unsafe fn IsItemDeactivatedAfterEdit() -> bool
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return IsItemDeactivated() && (g.ActiveIdPreviousFrameHasBeenEditedBefore || (g.ActiveId == 0 && g.ActiveIdHasBeenEditedBefore));
}



// Important: this can be useful but it is NOT equivalent to the behavior of e.g.Button()!
// Most widgets have specific reactions based on mouse-up/down state, mouse position etc.
// bool IsItemClicked(ImGuiMouseButton mouse_button)
pub unsafe fn IsItemClicked(mouse_button: ImGuiMouseButton) -> bool
{
    return IsMouseClicked(mouse_button, false) && IsItemHovered(ImGuiHoveredFlags_None);
}

// bool IsItemToggledOpen()
pub unsafe fn IsItemToggledOpen() -> bool
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return flag_set(g.LastItemData.StatusFlags, ImGuiItemStatusFlags_ToggledOpen);
}

// bool IsItemToggledSelection()
pub unsafe fn IsItemToggledSelectionm() -> bool
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return flag_set(g.LastItemData.StatusFlags, ImGuiItemStatusFlags_ToggledSelection);
}

// bool IsAnyItemHovered()
pub unsafe fn IsAnyItemHovered() -> bool
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.HoveredId != 0 || g.HoveredIdPreviousFrame != 0;
}

// bool IsAnyItemActive()
pub unsafe fn IsAnyItemActive() -> bool
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.ActiveId != 0;
}

// bool IsAnyItemFocused()
pub unsafe fn IsAnyItemFocused() -> bool
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.NavId != 0 && !g.NavDisableHighlight;
}

// bool IsItemVisible()
pub unsafe fn IsAnyItemVisible() -> bool
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.Currentwindow.ClipRect.Overlaps(&g.LastItemData.Rect);
}

// bool IsItemEdited()
pub unsafe fn IsItemEdited() -> bool
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return (g.LastItemData.StatusFlags & ImGuiItemStatusFlags_Edited) != 0;
}

// Allow last item to be overlapped by a subsequent item. Both may be activated during the same frame before the later one takes priority.
// FIXME: Although this is exposed, its interaction and ideal idiom with using ImGuiButtonFlags_AllowItemOverlap flag are extremely confusing, need rework.
// c_void SetItemAllowOverlap()
pub unsafe fn SetItemAllowedOverlap()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut id: ImGuiID =  g.LastItemData.ID;
    if g.HoveredId == id {
        g.HoveredIdAllowOverlap = true;
    }
    if g.ActiveId == id {
        g.ActiveIdAllowOverlap = true;
    }
}

// c_void SetItemUsingMouseWheel()
pub unsafe fn SetItemUsingMouseWheel()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut id: ImGuiID =  g.LastItemData.ID;
    if g.HoveredId == id {
        g.HoveredIdUsingMouseWheel = true;
    }
    if g.ActiveId == id
    {
        g.ActiveIdUsingKeyInputMask.SetBit(ImGuiKey_MouseWheelX);
        g.ActiveIdUsingKeyInputMask.SetBit(ImGuiKey_MouseWheelY);
    }
}


// ImVec2 GetItemRectMin()
pub unsafe fn GetItemRectMin() -> ImVec2
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.LastItemData.Rect.Min.clone();
}

// ImVec2 GetItemRectMax()
pub unsafe fn GetItemRectMax() -> ImVec2
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.LastItemData.Rect.Max.clone();
}

// ImVec2 GetItemRectSize()
pub unsafe fn GetItemRectSize() -> ImVec2
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.LastItemData.Rect.GetSize();
}
