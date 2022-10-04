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
use crate::window::ImGuiWindow;
use crate::window_ops::IsWindowContentHoverable;

// c_void MarkItemEdited(id: ImGuiID)
pub unsafe fn MarkItemEdited(id: ImGuiID) {
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
pub unsafe fn IsItemFocused() -> bool {
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
        // When the window is skipped/collapsed (SkipItems==true) that last item (always .MoveId submitted by Begin)
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
// bool ItemHoverable(const ImRect& bb, id: ImGuiID)
pub unsafe fn ItemHoverable(bb: &ImRect, id: ImGuiID) -> bool {
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
    if !IsWindowContentHoverable(window, ImGuiHoveredFlags_None) {
        g.HoveredIdDisabled = true;
        return false;
    }

    // We exceptionally allow this function to be called with id==0 to allow using it for easy high-level
    // hover test in widgets code. We could also decide to split this function is two.
    if id != 0 {
        SetHoveredID(id);
    }

    // When disabled we'll return false but still set HoveredId
    let mut item_flags: ImGuiItemFlags = if g.LastItemData.ID == id { g.LastItemData.InFlags } else { g.CurrentItemFlags };
    if item_flags & ImGuiItemFlags_Disabled {
        // Release active id if turning disabled
        if g.ActiveId == id {
            ClearActiveID();
        }
        g.HoveredIdDisabled = true;
        return false;
    }

    if id != 0 {
        // [DEBUG] Item Picker tool!
        // We perform the check here because SetHoveredID() is not frequently called (1~ time a frame), making
        // the cost of this tool near-zero. We can get slightly better call-stack and support picking non-hovered
        // items if we perform the test in ItemAdd(), but that would incur a small runtime cost.
        // #define IMGUI_DEBUG_TOOL_ITEM_PICKER_EX in imconfig.h if you want this check to also be performed in ItemAdd().
        if g.DebugItemPickerActive && g.HoveredIdPreviousFrame == id {
            GetForegroundDrawList(null_mut()).AddRect(&bb.Min, &bb.Max, IM_COL32(255, 255, 0, 255), 0f32, ImDrawFlags_None, 0f32, , );
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

// bool IsClippedEx(const ImRect& bb, id: ImGuiID)
pub unsafe fn IsClippedEx(bb: &mut ImRect, id: ImGuiID) -> bool {
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
// c_void SetLastItemData(item_id: ImGuiID, ImGuiItemFlags in_flags, ImGuiItemStatusFlags item_flags, const ImRect& item_rect)
pub unsafe fn SetLastItemData(item_id: ImGuiID, in_flags: ImGuiItemFlags, item_flags: ImGuiItemStatusFlags, item_rect: &ImRect) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.LastItemData.ID = item_id;
    g.LastItemData.InFlags = in_flags;
    g.LastItemData.StatusFlags = item_flags;
    g.LastItemData.Rect = item_rect.clone();
}

// c_float CalcWrapWidthForPos(const ImVec2& pos, c_float wrap_pos_x)
pub unsafe fn CalcWrapWidthForPos(pos: &ImVec2, mut wrap_pos_x: c_float) -> c_float {
    if wrap_pos_x < 0f32 {
        return 0f32;
    }

    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if wrap_pos_x == 0f32 {
        // We could decide to setup a default wrapping max point for auto-resizing windows,
        // or have auto-wrap (with unspecified wrapping pos) behave as a ContentSize extending function?
        //if (window.Hidden && (window.Flags & ImGuiWindowFlags_AlwaysAutoResize))
        //    wrap_pos_x = ImMax(window.WorkRect.Min.x + g.FontSize * 10f32, window.WorkRect.Max.x);
        //else
        wrap_pos_x = window.WorkRect.Max.x;
    } else if wrap_pos_x > 0f32 {
        wrap_pos_x += window.Pos.x - window.Scroll.x; // wrap_pos_x is provided is window local space
    }

    return ImMax(wrap_pos_x - pos.x, 1f32);
}


// bool IsItemActive()
pub unsafe fn IsItemActive() -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if g.ActiveId {
        return g.ActiveId == g.LastItemData.ID;
    }
    return false;
}

// bool IsItemActivated()
pub unsafe fn IsItemActivated() -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if g.ActiveId {
        if g.ActiveId == g.LastItemData.ID && g.ActiveIdPreviousFrame != g.LastItemData.ID {
            return true;
        }
    }
    return false;
}

// bool IsItemDeactivated()
pub unsafe fn IsItemDeactivated() -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if g.LastItemData.StatusFlags & ImGuiItemStatusFlags_HasDeactivated {
        return (g.LastItemData.StatusFlags & ImGuiItemStatusFlags_Deactivated) != 0;
    }
    return g.ActiveIdPreviousFrame == g.LastItemData.ID && g.ActiveIdPreviousFrame != 0 && g.ActiveId != g.LastItemData.ID;
}

// bool IsItemDeactivatedAfterEdit()
pub unsafe fn IsItemDeactivatedAfterEdit() -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return IsItemDeactivated() && (g.ActiveIdPreviousFrameHasBeenEditedBefore || (g.ActiveId == 0 && g.ActiveIdHasBeenEditedBefore));
}


// Important: this can be useful but it is NOT equivalent to the behavior of e.g.Button()!
// Most widgets have specific reactions based on mouse-up/down state, mouse position etc.
// bool IsItemClicked(ImGuiMouseButton mouse_button)
pub unsafe fn IsItemClicked(mouse_button: ImGuiMouseButton) -> bool {
    return IsMouseClicked(mouse_button, false) && IsItemHovered(ImGuiHoveredFlags_None);
}

// bool IsItemToggledOpen()
pub unsafe fn IsItemToggledOpen() -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return flag_set(g.LastItemData.StatusFlags, ImGuiItemStatusFlags_ToggledOpen);
}

// bool IsItemToggledSelection()
pub unsafe fn IsItemToggledSelectionm() -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return flag_set(g.LastItemData.StatusFlags, ImGuiItemStatusFlags_ToggledSelection);
}

// bool IsAnyItemHovered()
pub unsafe fn IsAnyItemHovered() -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.HoveredId != 0 || g.HoveredIdPreviousFrame != 0;
}

// bool IsAnyItemActive()
pub unsafe fn IsAnyItemActive() -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.ActiveId != 0;
}

// bool IsAnyItemFocused()
pub unsafe fn IsAnyItemFocused() -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.NavId != 0 && !g.NavDisableHighlight;
}

// bool IsItemVisible()
pub unsafe fn IsAnyItemVisible() -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.Currentwindow.ClipRect.Overlaps(&g.LastItemData.Rect);
}

// bool IsItemEdited()
pub unsafe fn IsItemEdited() -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return (g.LastItemData.StatusFlags & ImGuiItemStatusFlags_Edited) != 0;
}

// Allow last item to be overlapped by a subsequent item. Both may be activated during the same frame before the later one takes priority.
// FIXME: Although this is exposed, its interaction and ideal idiom with using ImGuiButtonFlags_AllowItemOverlap flag are extremely confusing, need rework.
// c_void SetItemAllowOverlap()
pub unsafe fn SetItemAllowedOverlap() {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut id: ImGuiID = g.LastItemData.ID;
    if g.HoveredId == id {
        g.HoveredIdAllowOverlap = true;
    }
    if g.ActiveId == id {
        g.ActiveIdAllowOverlap = true;
    }
}

// c_void SetItemUsingMouseWheel()
pub unsafe fn SetItemUsingMouseWheel() {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut id: ImGuiID = g.LastItemData.ID;
    if g.HoveredId == id {
        g.HoveredIdUsingMouseWheel = true;
    }
    if g.ActiveId == id {
        g.ActiveIdUsingKeyInputMask.SetBit(ImGuiKey_MouseWheelX);
        g.ActiveIdUsingKeyInputMask.SetBit(ImGuiKey_MouseWheelY);
    }
}


// ImVec2 GetItemRectMin()
pub unsafe fn GetItemRectMin() -> ImVec2 {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.LastItemData.Rect.Min.clone();
}

// ImVec2 GetItemRectMax()
pub unsafe fn GetItemRectMax() -> ImVec2 {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.LastItemData.Rect.Max.clone();
}

// ImVec2 GetItemRectSize()
pub unsafe fn GetItemRectSize() -> ImVec2 {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.LastItemData.Rect.GetSize();
}

// inline c_void             ItemSize( const ImRect & bb, let text_baseline_y: c_float = - 1f32) 
pub fn ItemSize2(bb: &ImRect, text_baseline_y: c_float) {
    ItemSize(bb.GetSize(), text_baseline_y);
} // FIXME: This is a misleading API since we expect CursorPos to be bb.Min.


// inline bool     FocusableItemRegister( * mut ImGuiWindow window, id: ImGuiID)
pub fn FocusableItemRegister(window: *mut ImGuiWindow, id: ImGuiID) -> bool {
    IM_ASSERT(0);
    IM_UNUSED(window);
    IM_UNUSED(id);
    return false;
} // -> pass ImGuiItemAddFlags_Inputable flag to ItemAdd()


// inline c_void     FocusableItemUnregister( * mut ImGuiWindow window)                      
pub fn FocusableItemUnregister(window: *mut ImGuiWindow) {
    IM_ASSERT(0);
    IM_UNUSED(window);
}                              // -> unnecessary:



c_void PushItemFlag(ImGuiItemFlags option, enabled: bool)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut item_flags: ImGuiItemFlags =  g.CurrentItemFlags;
    // IM_ASSERT(item_flags == g.ItemFlagsStack.back());
    if (enabled)
        item_flags |= option;
    else
        item_flags &= ~option;
    g.CurrentItemFlags = item_flags;
    g.ItemFlagsStack.push(item_flags);
}

c_void PopItemFlag()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.ItemFlagsStack.Size > 1); // Too many calls to PopItemFlag() - we always leave a 0 at the bottom of the stack.
    g.ItemFlagsStack.pop_back();
    g.CurrentItemFlags = g.ItemFlagsStack.last().unwrap();
}


c_void ActivateItem(id: ImGuiID)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NavNextActivateId = id;
    g.NavNextActivateFlags = ImGuiActivateFlags_None;
}



// Advance cursor given item size for layout.
// Register minimum needed size so it can extend the bounding box used for auto-fit calculation.
// See comments in ItemAdd() about how/why the size provided to ItemSize() vs ItemAdd() may often different.
c_void ItemSize(const ImVec2& size, c_float text_baseline_y)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if (window.SkipItems)
        return;

    // We increase the height in this function to accommodate for baseline offset.
    // In theory we should be offsetting the starting position (window.DC.CursorPos), that will be the topic of a larger refactor,
    // but since ItemSize() is not yet an API that moves the cursor (to handle e.g. wrapping) enlarging the height has the same effect.
    let offset_to_match_baseline_y: c_float =  (text_baseline_y >= 0) ? ImMax(0f32, window.DC.CurrLineTextBaseOffset - text_baseline_y) : 0f32;

    let line_y1: c_float =  window.DC.IsSameLine ? window.DC.CursorPosPrevLine.y : window.DC.CursorPos.y;
    let line_height: c_float =  ImMax(window.DC.CurrLineSize.y, /*ImMax(*/window.DC.CursorPos.y - line_y1/*, 0f32)*/ + size.y + offset_to_match_baseline_y);

    // Always align ourselves on pixel boundaries
    //if (g.IO.KeyAlt) window.DrawList.AddRect(window.DC.CursorPos, window.DC.CursorPos + ImVec2::new(size.x, line_height), IM_COL32(255,0,0,200)); // [DEBUG]
    window.DC.CursorPosPrevLine.x = window.DC.CursorPos.x + size.x;
    window.DC.CursorPosPrevLine.y = line_y1;
    window.DC.CursorPos.x = IM_FLOOR(window.Pos.x + window.DC.Indent.x + window.DC.ColumnsOffset.x);    // Next line
    window.DC.CursorPos.y = IM_FLOOR(line_y1 + line_height + g.Style.ItemSpacing.y);                    // Next line
    window.DC.CursorMaxPos.x = ImMax(window.DC.CursorMaxPos.x, window.DC.CursorPosPrevLine.x);
    window.DC.CursorMaxPos.y = ImMax(window.DC.CursorMaxPos.y, window.DC.CursorPos.y - g.Style.ItemSpacing.y);
    //if (g.IO.KeyAlt) window.DrawList.AddCircle(window.DC.CursorMaxPos, 3.0f32, IM_COL32(255,0,0,255), 4); // [DEBUG]

    window.DC.PrevLineSize.y = line_height;
    window.DC.CurrLineSize.y = 0f32;
    window.DC.PrevLineTextBaseOffset = ImMax(window.DC.CurrLineTextBaseOffset, text_baseline_y);
    window.DC.CurrLineTextBaseOffset = 0f32;
    window.DC.IsSameLine = window.DC.IsSetPos = false;

    // Horizontal layout mode
    if (window.DC.LayoutType == ImGuiLayoutType_Horizontal)
        SameLine();
}

// Declare item bounding box for clipping and interaction.
// Note that the size can be different than the one provided to ItemSize(). Typically, widgets that spread over available surface
// declare their minimum size requirement to ItemSize() and provide a larger region to ItemAdd() which is used drawing/interaction.
bool ItemAdd(const ImRect& bb, id: ImGuiID, *const ImRect nav_bb_arg, ImGuiItemFlags extra_flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;

    // Set item data
    // (DisplayRect is left untouched, made valid when ImGuiItemStatusFlags_HasDisplayRect is set)
    g.LastItemData.ID = id;
    g.LastItemData.Rect = bb;
    g.LastItemData.NavRect = nav_bb_arg ? *nav_bb_arg : bb;
    g.LastItemData.InFlags = g.CurrentItemFlags | extra_flags;
    g.LastItemData.StatusFlags = ImGuiItemStatusFlags_None;

    // Directional navigation processing
    if (id != 0)
    {
        KeepAliveID(id);

        // Runs prior to clipping early-out
        //  (a) So that NavInitRequest can be honored, for newly opened windows to select a default widget
        //  (b) So that we can scroll up/down past clipped items. This adds a small O(N) cost to regular navigation requests
        //      unfortunately, but it is still limited to one window. It may not scale very well for windows with ten of
        //      thousands of item, but at least NavMoveRequest is only set on user interaction, aka maximum once a frame.
        //      We could early out with "if (is_clipped && !g.NavInitRequest) return false;" but when we wouldn't be able
        //      to reach unclipped widgets. This would work if user had explicit scrolling control (e.g. mapped on a stick).
        // We intentionally don't check if g.NavWindow != NULL because g.NavAnyRequest should only be set when it is non null.
        // If we crash on a NULL g.NavWindow we need to fix the bug elsewhere.
        window.DC.NavLayersActiveMaskNext |= (1 << window.DC.NavLayerCurrent);
        if (g.NavId == id || g.NavAnyRequest)
            if (g.NavWindow.RootWindowForNav == window.RootWindowForNav)
                if (window == g.NavWindow || ((window.Flags | g.NavWindow.Flags) & ImGuiWindowFlags_NavFlattened))
                    NavProcessItem();

        // [DEBUG] People keep stumbling on this problem and using "" as identifier in the root of a window instead of "##something".
        // Empty identifier are valid and useful in a small amount of cases, but 99.9% of the time you want to use "##something".
        // READ THE FAQ: https://dearimgui.org/faq
        // IM_ASSERT(id != window.ID && "Cannot have an empty ID at the root of a window. If you need an empty label, use ## and read the FAQ about how the ID Stack works!");

        // [DEBUG] Item Picker tool, when enabling the "extended" version we perform the check in ItemAdd()
// #ifdef IMGUI_DEBUG_TOOL_ITEM_PICKER_EX
        if (id == g.DebugItemPickerBreakId)
        {
            IM_DEBUG_BREAK();
            g.DebugItemPickerBreakId = 0;
        }
// #endif
    }
    g.NextItemData.Flags = ImGuiNextItemDataFlags_None;

// #ifdef IMGUI_ENABLE_TEST_ENGINE
    if (id != 0)
        IMGUI_TEST_ENGINE_ITEM_ADD(nav_bb_arg ? *nav_bb_arg : bb, id);
// #endif

    // Clipping test
    let is_clipped: bool = IsClippedEx(bb, id);
    if (is_clipped)
        return false;
    //if (g.IO.KeyAlt) window.DrawList.AddRect(bb.Min, bb.Max, IM_COL32(255,255,0,120)); // [DEBUG]

    // We need to calculate this now to take account of the current clipping rectangle (as items like Selectable may change them)
    if (IsMouseHoveringRect(bb.Min, bb.Max))
        g.LastItemData.StatusFlags |= ImGuiItemStatusFlags_HoveredRect;
    return true;
}



// Affect large frame+labels widgets only.
c_void SetNextItemWidth(c_float item_width)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NextItemData.Flags |= ImGuiNextItemDataFlags_HasWidth;
    g.NextItemData.Width = item_width;
}

// FIXME: Remove the == 0f32 behavior?
c_void PushItemWidth(c_float item_width)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    window.DC.ItemWidthStack.push(window.DC.ItemWidth); // Backup current width
    window.DC.ItemWidth = (item_width == 0f32 ? window.ItemWidthDefault : item_width);
    g.NextItemData.Flags &= ~ImGuiNextItemDataFlags_HasWidth;
}

c_void PushMultiItemsWidths(c_int components, c_float w_full)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    const let mut style = &mut g.Style;
    const c_float w_item_one  = ImMax(1f32, IM_FLOOR((w_full - (style.ItemInnerSpacing.x) * (components - 1)) / components));
    let w_item_last: c_float =  ImMax(1f32, IM_FLOOR(w_full - (w_item_one + style.ItemInnerSpacing.x) * (components - 1)));
    window.DC.ItemWidthStack.push(window.DC.ItemWidth); // Backup current width
    window.DC.ItemWidthStack.push(w_item_last);
    for (let i: c_int = 0; i < components - 2; i++)
        window.DC.ItemWidthStack.push(w_item_one);
    window.DC.ItemWidth = (components == 1) ? w_item_last : w_item_one;
    g.NextItemData.Flags &= ~ImGuiNextItemDataFlags_HasWidth;
}

c_void PopItemWidth()
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    window.DC.ItemWidth = window.DC.ItemWidthStack.last().unwrap();
    window.DC.ItemWidthStack.pop_back();
}

// Calculate default item width given value passed to PushItemWidth() or SetNextItemWidth().
// The SetNextItemWidth() data is generally cleared/consumed by ItemAdd() or NextItemData.ClearFlags()
c_float CalcItemWidth()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    let mut w: c_float = 0f32;
    if (g.NextItemData.Flags & ImGuiNextItemDataFlags_HasWidth)
        w = g.NextItemData.Width;
    else
        w = window.DC.ItemWidth;
    if (w < 0f32)
    {
        let region_max_x: c_float =  GetContentRegionMaxAbs().x;
        w = ImMax(1f32, region_max_x - window.DC.CursorPos.x + w);
    }
    w = IM_FLOOR(w);
    return w;
}

// [Internal] Calculate full item size given user provided 'size' parameter and default width/height. Default width is often == CalcItemWidth().
// Those two functions CalcItemWidth vs CalcItemSize are awkwardly named because they are not fully symmetrical.
// Note that only CalcItemWidth() is publicly exposed.
// The 4.0f32 here may be changed to match CalcItemWidth() and/or BeginChild() (right now we have a mismatch which is harmless but undesirable)
ImVec2 CalcItemSize(ImVec2 size, c_float default_w, c_float default_h)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;

    let mut region_max = ImVec2::default();
    if (size.x < 0f32 || size.y < 0f32)
        region_max = GetContentRegionMaxAbs();

    if (size.x == 0f32)
        size.x = default_w;
    else if (size.x < 0f32)
        size.x = ImMax(4.0f32, region_max.x - window.DC.CursorPos.x + size.x);

    if (size.y == 0f32)
        size.y = default_h;
    else if (size.y < 0f32)
        size.y = ImMax(4.0f32, region_max.y - window.DC.CursorPos.y + size.y);

    return size;
}