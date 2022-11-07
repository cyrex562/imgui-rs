#![allow(non_snake_case)]

use crate::activate_flags::ImGuiActivateFlags_None;
use crate::color::color_u32_from_rgba;
use crate::content_ops::content_region_max_abs;
use crate::context::ImguiContext;
use crate::draw_flags::ImDrawFlags_None;
use crate::draw_list_ops::GetForegroundDrawList;
use crate::hovered_flags::{
    ImGuiHoveredFlags, ImGuiHoveredFlags_AllowWhenBlockedByActiveItem,
    ImGuiHoveredFlags_AllowWhenDisabled, ImGuiHoveredFlags_AllowWhenOverlapped,
    ImGuiHoveredFlags_DelayNormal, ImGuiHoveredFlags_DelayShort, ImGuiHoveredFlags_NoNavOverride,
    ImGuiHoveredFlags_NoSharedDelay, ImGuiHoveredFlags_None,
};
use crate::id_ops::{ClearActiveID, KeepAliveID, SetHoveredID};
use crate::imgui::GImGui;
use crate::input_ops::{IsMouseClicked, IsMouseHoveringRect};
use crate::item_flags::{ImGuiItemFlags, ImGuiItemFlags_Disabled};
use crate::item_status_flags::{
    ImGuiItemStatusFlags, ImGuiItemStatusFlags_Deactivated, ImGuiItemStatusFlags_Edited,
    ImGuiItemStatusFlags_HasDeactivated, ImGuiItemStatusFlags_HoveredRect,
    ImGuiItemStatusFlags_HoveredWindow, ImGuiItemStatusFlags_None,
    ImGuiItemStatusFlags_ToggledOpen, ImGuiItemStatusFlags_ToggledSelection,
};
use crate::key::{ImGuiKey_MouseWheelX, ImGuiKey_MouseWheelY};
use crate::layout_ops::same_line;
use crate::layout_type::ImGuiLayoutType_Horizontal;
use crate::math_ops::ImMax;
use crate::mouse_button::ImGuiMouseButton;
use crate::nav_ops::NavProcessItem;
use crate::next_item_data_flags::{ImGuiNextItemDataFlags_HasWidth, ImGuiNextItemDataFlags_None};
use crate::rect::ImRect;
use crate::type_defs::ImguiHandle;
use crate::utils::{flag_clear, flag_set};
use crate::vec2::ImVec2;
use crate::window::ops::{GetCurrentWindow, IsWindowContentHoverable};
use crate::window::window_flags::ImGuiWindowFlags_NavFlattened;
use crate::window::ImguiWindow;
use crate::window_flags::ImGuiWindowFlags_NavFlattened;
use crate::window_ops::IsWindowContentHoverable;
use libc::{c_float, c_int};
use std::ptr::null_mut;

// c_void MarkItemEdited(ImguiHandle id)
pub fn MarkItemEdited(g: &mut ImguiContext, id: ImguiHandle) {
    // This marking is solely to be able to provide info for IsItemDeactivatedAfterEdit().
    // ActiveId might have been released by the time we call this (as in the typical press/release button behavior) but still need need to fill the data.
    // IM_ASSERT(g.ActiveId == id || g.ActiveId == 0 || g.DragDropActive);
    // IM_UNUSED(id); // Avoid unused variable warnings when asserts are compiled out.
    //IM_ASSERT(g.Currentwindow.DC.LastItemId == id);
    g.ActiveIdHasBeenEditedThisFrame = true;
    g.ActiveIdHasBeenEditedBefore = true;
    g.last_item_data.status_flags |= ImGuiItemStatusFlags_Edited;
}

// == GetItemID() == GetFocusID()
// IsItemFocused: bool()
pub unsafe fn IsItemFocused() -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if g.NavId != g.LastItemData.ID || g.NavId == 0 {
        return false;
    }

    // Special handling for the dummy item after Begin() which represent the title bar or tab.
    // When the window is collapsed (SkipItems==true) that last item will never be overwritten so we need to detect the case.
    let mut window = g.current_window_mut().unwrap();
    if g.LastItemData.ID == window.ID && window.WriteAccessed {
        return false;
    }

    return true;
}

// This is roughly matching the behavior of internal-facing ItemHoverable()
// - we allow hovering to be true when ActiveId==window.MoveID, so that clicking on non-interactive items such as a Text() item still returns true with IsItemHovered()
// - this should work even for non-interactive items that have no ID, so we cannot use LastItemId
// IsItemHovered: bool(flags: ImGuiHoveredFlags)
pub unsafe fn IsItemHovered(flags: ImGuiHoveredFlags) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.current_window_mut().unwrap();
    if g.NavDisableMouseHover
        && !g.NavDisableHighlight
        && flag_clear(flags, ImGuiHoveredFlags_NoNavOverride)
    {
        if flag_set(g.LastItemData.InFlags, ImGuiItemFlags_Disabled)
            && flag_clear(flags, ImGuiHoveredFlags_AllowWhenDisabled)
        {
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
            if flag_clear(flags, ImGuiHoveredFlags_AllowWhenOverlapped) {
                return false;
            }
        }

        // Test if another item is active (e.g. being dragged)
        if flag_clear(flags, ImGuiHoveredFlags_AllowWhenBlockedByActiveItem) {
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
        if flag_set(g.LastItemData.InFlags, ImGuiItemFlags_Disabled)
            && flag_clear(flags, ImGuiHoveredFlags_AllowWhenDisabled)
        {
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
    let mut delay: c_float = 0.0;
    if flags & ImGuiHoveredFlags_DelayNormal {
        delay = g.IO.HoverDelayNormal;
    } else if flags & ImGuiHoveredFlags_DelayShort {
        delay = g.IO.HoverDelayShort;
    } else {
        delay = 0.0;
    }
    if delay > 0.0 {
        let mut hover_delay_id: ImguiHandle = if g.LastItemData.ID != 0 {
            g.LastItemData.ID
        } else {
            window.GetIDFromRectangle(&g.LastItemData.Rect)
        };
        if flag_set(flags, ImGuiHoveredFlags_NoSharedDelay)
            && (g.HoverDelayIdPreviousFrame != hover_delay_id)
        {
            g.HoverDelayTimer = 0.0;
        }
        g.HoverDelayId = hover_delay_id;
        return g.HoverDelayTimer >= delay;
    }

    return true;
}

// Internal facing ItemHoverable() used when submitting widgets. Differs slightly from IsItemHovered().
// ItemHoverable: bool(const ImRect& bb, ImguiHandle id)
pub unsafe fn ItemHoverable(bb: &ImRect, id: ImguiHandle) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if g.HoveredId != 0 && g.HoveredId != id && !g.HoveredIdAllowOverlap {
        return false;
    }

    let mut window = g.current_window_mut().unwrap();
    if g.HoveredWindow != window {
        return false;
    }
    if g.ActiveId != 0 && g.ActiveId != id && !g.ActiveIdAllowOverlap {
        return false;
    }
    if !IsMouseHoveringRect(&bb.min, &bb.max, false) {
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
    let mut item_flags: ImGuiItemFlags = if g.LastItemData.ID == id {
        g.LastItemData.InFlags
    } else {
        g.CurrentItemFlags
    };
    if item_flags & ImGuiItemFlags_Disabled {
        // Release active id if turning disabled
        if g.ActiveId == id {
            ClearActiveID(g);
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
            GetForegroundDrawList(null_mut()).AddRect(
                &bb.min,
                &bb.max,
                color_u32_from_rgba(255, 255, 0, 255),
                0.0,
            );
        }
        if g.DebugItemPickerBreakId == id {
            // IM_DEBUG_BREAK();
        }
    }

    if g.NavDisableMouseHover {
        return false;
    }

    return true;
}

// IsClippedEx: bool(const ImRect& bb, ImguiHandle id)
pub unsafe fn IsClippedEx(bb: &mut ImRect, id: ImguiHandle) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.current_window_mut().unwrap();
    if !bb.Overlaps(&ImRect::from_vec4(&window.ClipRect)) {
        if id == 0 || (id != g.ActiveId && id != g.NavId) {
            if !g.LogEnabled {
                return true;
            }
        }
    }
    return false;
}

pub fn set_last_item_data(
    g: &mut ImguiContext,
    item_id: ImguiHandle,
    in_flags: ImGuiItemFlags,
    item_flags: ImGuiItemStatusFlags,
    item_rect: &ImRect,
) {
    g.last_item_data.id = item_id;
    g.last_item_data.in_flags = in_flags;
    g.last_item_data.status_flags = item_flags;
    g.last_item_data.rect = item_rect.clone();
}

pub fn calc_width_for_pos(g: &mut ImguiContext, pos: &ImVec2, mut wrap_pos_x: f32) -> f32 {
    if wrap_pos_x < 0.0 {
        return 0.0;
    }

    let mut window = g.current_window_mut().unwrap();
    if wrap_pos_x == 0.0 {
        // We could decide to setup a default wrapping max point for auto-resizing windows,
        // or have auto-wrap (with unspecified wrapping pos) behave as a ContentSize extending function?
        //if (window.Hidden && (window.Flags & ImGuiWindowFlags_AlwaysAutoResize))
        //    wrap_pos_x = ImMax(window.work_rect.Min.x + g.FontSize * 10.0, window.work_rect.Max.x);
        //else
        wrap_pos_x = window.work_rect.Max.x;
    } else if wrap_pos_x > 0.0 {
        // wrap_pos_x is provided is window local space
        wrap_pos_x += window.position.x - window.scroll.x;
    }

    return (wrap_pos_x - pos.x).max(1.0);
}

// IsItemActive: bool()
pub unsafe fn IsItemActive() -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if g.ActiveId {
        return g.ActiveId == g.LastItemData.ID;
    }
    return false;
}

// IsItemActivated: bool()
pub unsafe fn IsItemActivated() -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if g.ActiveId {
        if g.ActiveId == g.LastItemData.ID && g.ActiveIdPreviousFrame != g.LastItemData.ID {
            return true;
        }
    }
    return false;
}

// IsItemDeactivated: bool()
pub unsafe fn IsItemDeactivated() -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if g.LastItemData.StatusFlags & ImGuiItemStatusFlags_HasDeactivated {
        return (g.LastItemData.StatusFlags & ImGuiItemStatusFlags_Deactivated) != 0;
    }
    return g.ActiveIdPreviousFrame == g.LastItemData.ID
        && g.ActiveIdPreviousFrame != 0
        && g.ActiveId != g.LastItemData.ID;
}

// IsItemDeactivatedAfterEdit: bool()
pub unsafe fn IsItemDeactivatedAfterEdit() -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return IsItemDeactivated()
        && (g.ActiveIdPreviousFrameHasBeenEditedBefore
            || (g.ActiveId == 0 && g.ActiveIdHasBeenEditedBefore));
}

// Important: this can be useful but it is NOT equivalent to the behavior of e.g.Button()!
// Most widgets have specific reactions based on mouse-up/down state, mouse position etc.
// IsItemClicked: bool(ImGuiMouseButton mouse_button)
pub unsafe fn IsItemClicked(mouse_button: ImGuiMouseButton) -> bool {
    return IsMouseClicked(mouse_button, false) && IsItemHovered(ImGuiHoveredFlags_None);
}

// IsItemToggledOpen: bool()
pub unsafe fn IsItemToggledOpen() -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return flag_set(g.LastItemData.StatusFlags, ImGuiItemStatusFlags_ToggledOpen);
}

// IsItemToggledSelection: bool()
pub unsafe fn IsItemToggledSelectionm() -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return flag_set(
        g.LastItemData.StatusFlags,
        ImGuiItemStatusFlags_ToggledSelection,
    );
}

// IsAnyItemHovered: bool()
pub unsafe fn IsAnyItemHovered() -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.HoveredId != 0 || g.HoveredIdPreviousFrame != 0;
}

// IsAnyItemActive: bool()
pub unsafe fn IsAnyItemActive() -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.ActiveId != 0;
}

// IsAnyItemFocused: bool()
pub unsafe fn IsAnyItemFocused() -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.NavId != 0 && !g.NavDisableHighlight;
}

// IsItemVisible: bool()
pub unsafe fn IsAnyItemVisible() -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.Currentwindow.ClipRect.Overlaps(&g.LastItemData.Rect);
}

// IsItemEdited: bool()
pub unsafe fn IsItemEdited() -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return (g.LastItemData.StatusFlags & ImGuiItemStatusFlags_Edited) != 0;
}

// Allow last item to be overlapped by a subsequent item. Both may be activated during the same frame before the later one takes priority.
// FIXME: Although this is exposed, its interaction and ideal idiom with using ImGuiButtonFlags_AllowItemOverlap flag are extremely confusing, need rework.
// c_void SetItemAllowOverlap()
pub unsafe fn SetItemAllowedOverlap() {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut id: ImguiHandle = g.LastItemData.ID;
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
    let mut id: ImguiHandle = g.LastItemData.ID;
    if g.HoveredId == id {
        g.HoveredIdUsingMouseWheel = true;
    }
    if g.ActiveId == id {
        g.ActiveIdUsingKeyInputMask.SetBit(ImGuiKey_MouseWheelX);
        g.ActiveIdUsingKeyInputMask.SetBit(ImGuiKey_MouseWheelY);
    }
}

// GetItemRectMin: ImVec2()
pub unsafe fn GetItemRectMin() -> ImVec2 {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.LastItemData.Rect.Min.clone();
}

// GetItemRectMax: ImVec2()
pub unsafe fn GetItemRectMax() -> ImVec2 {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.LastItemData.Rect.Max.clone();
}

// GetItemRectSize: ImVec2()
pub unsafe fn GetItemRectSize() -> ImVec2 {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.LastItemData.Rect.GetSize();
}

// Declare item bounding box for clipping and interaction.
// Note that the size can be different than the one provided to ItemSize(). Typically, widgets that spread over available surface
// declare their minimum size requirement to ItemSize() and provide a larger region to ItemAdd() which is used drawing/interaction.
pub fn ItemAdd(
    g: &mut ImguiContext,
    bb: &mut ImRect,
    id: ImguiHandle,
    nav_bb_arg: Option<&ImRect>,
    extra_flags: ImGuiItemFlags,
) -> bool {
    let mut window = g.current_window_mut().unwrap();

    // Set item data
    // (DisplayRect is left untouched, made valid when ImGuiItemStatusFlags_HasDisplayRect is set)
    g.last_item_data.id = id;
    g.last_item_data.rect = bb.clone();
    g.last_item_data.NavRect = nav_bb_arg.unwrap_or(bb).clone();
    g.last_item_data.in_flags = g.CurrentItemFlags | extra_flags;
    g.last_item_data.status_flags = ImGuiItemStatusFlags_None;

    // Directional navigation processing
    if id != 0 {
        KeepAliveID(g, id);

        // Runs prior to clipping early-out
        //  (a) So that NavInitRequest can be honored, for newly opened windows to select a default widget
        //  (b) So that we can scroll up/down past clipped items. This adds a small O(N) cost to regular navigation requests
        //      unfortunately, but it is still limited to one window. It may not scale very well for windows with ten of
        //      thousands of item, but at least NavMoveRequest is only set on user interaction, aka maximum once a frame.
        //      We could early out with "if (is_clipped && !g.NavInitRequest) return false;" but when we wouldn't be able
        //      to reach unclipped widgets. This would work if user had explicit scrolling control (e.g. mapped on a stick).
        // We intentionally don't check if g.NavWindow != NULL because g.NavAnyRequest should only be set when it is non null.
        // If we crash on a NULL g.NavWindow we need to fix the bug elsewhere.
        window.dc.NavLayersActiveMaskNext |= (1 << window.dc.NavLayerCurrent);
        if g.NavId == id || g.NavAnyRequest {
            if g.NavWindow.RootWindowForNav == window.RootWindowForNav {
                if window == g.NavWindow
                    || flag_set(
                        (window.Flags | g.NavWindow.Flags),
                        ImGuiWindowFlags_NavFlattened,
                    )
                {
                    NavProcessItem();
                }
            }
        }

        // [DEBUG] People keep stumbling on this problem and using "" as identifier in the root of a window instead of "##something".
        // Empty identifier are valid and useful in a small amount of cases, but 99.9% of the time you want to use "##something".
        // READ THE FAQ: https://dearimgui.org/faq
        // IM_ASSERT(id != window.ID && "Cannot have an empty ID at the root of a window. If you need an empty label, use ## and read the FAQ about how the ID Stack works!");

        // [DEBUG] Item Picker tool, when enabling the "extended" version we perform the check in ItemAdd()
        // #ifdef IMGUI_DEBUG_TOOL_ITEM_PICKER_EX
        if id == g.DebugItemPickerBreakId {
            // IM_DEBUG_BREAK();
            g.DebugItemPickerBreakId = 0;
        }
        // #endif
    }
    g.NextItemData.Flags = ImGuiNextItemDataFlags_None;

    // #ifdef IMGUI_ENABLE_TEST_ENGINE
    if (id != 0) {
        IMGUI_TEST_ENGINE_ITEM_ADD(nav_bb_arg.unwrap_or(bb), id);
    }
    // #endif

    // Clipping test
    let is_clipped: bool = IsClippedEx(bb, id);
    if is_clipped {
        return false;
    }
    //if (g.IO.KeyAlt) window.DrawList.AddRect(bb.Min, bb.Max, IM_COL32(255,255,0,120)); // [DEBUG]

    // We need to calculate this now to take account of the current clipping rectangle (as items like Selectable may change them)
    if IsMouseHoveringRect(&bb.min, &bb.max, false) {
        g.last_item_data.status_flags |= ImGuiItemStatusFlags_HoveredRect;
    }
    return true;
}

// Advance cursor given item size for layout.
// Register minimum needed size so it can extend the bounding box used for auto-fit calculation.
// See comments in ItemAdd() about how/why the size provided to ItemSize() vs ItemAdd() may often different.
pub fn ItemSize(g: &mut ImguiContext, size: &ImVec2, text_baseline_y: f32) {
    let mut window = g.current_window_mut().unwrap();
    if window.skip_items {
        return;
    }

    // We increase the height in this function to accommodate for baseline offset.
    // In theory we should be offsetting the starting position (window.dc.cursor_pos), that will be the topic of a larger refactor,
    // but since ItemSize() is not yet an API that moves the cursor (to handle e.g. wrapping) enlarging the height has the same effect.
    let offset_to_match_baseline_y: f32 = if text_baseline_y >= 0.0 {
        (0.0).max(window.dc.curr_line_text_base_offset - text_baseline_y)
    } else {
        0.0
    };

    let line_y1: c_float = if window.dc.is_same_line {
        window.dc.cursor_pos_prev_line.y
    } else {
        window.dc.cursor_pos.y
    };
    let line_height: c_float = ImMax(
        window.dc.curr_line_size.y,
        /*ImMax(*/
        window.dc.cursor_pos.y - line_y1/*, 0.0)*/ + size.y + offset_to_match_baseline_y,
    );

    // Always align ourselves on pixel boundaries
    //if (g.IO.KeyAlt) window.DrawList.AddRect(window.dc.cursor_pos, window.dc.cursor_pos + ImVec2::new(size.x, line_height), IM_COL32(255,0,0,200)); // [DEBUG]
    window.dc.cursor_pos_prev_line.x = window.dc.cursor_pos.x + size.x;
    window.dc.cursor_pos_prev_line.y = line_y1;
    window.dc.cursor_pos.x =
        IM_FLOOR(window.position.x + window.dc.indent.x + window.dc.columns_offset.x); // Next line
    window.dc.cursor_pos.y = IM_FLOOR(line_y1 + line_height + g.style.item_spacing.y); // Next line
    window.dc.CursorMaxPos.x = ImMax(window.dc.CursorMaxPos.x, window.dc.cursor_pos_prev_line.x);
    window.dc.CursorMaxPos.y = ImMax(
        window.dc.CursorMaxPos.y,
        window.dc.cursor_pos.y - g.style.item_spacing.y,
    );
    //if (g.IO.KeyAlt) window.DrawList.AddCircle(window.dc.CursorMaxPos, 3.0, IM_COL32(255,0,0,255), 4); // [DEBUG]

    window.dc.prev_line_size.y = line_height;
    window.dc.curr_line_size.y = 0.0;
    window.dc.prev_line_text_base_offset =
        ImMax(window.dc.curr_line_text_base_offset, text_baseline_y);
    window.dc.curr_line_text_base_offset = 0.0;
    window.dc.is_same_line = false;
    window.dc.is_set_pos = false;

    // Horizontal layout mode
    if window.dc.LayoutType == ImGuiLayoutType_Horizontal {
        same_line(g, 0.0, 0.0);
    }
}

pub unsafe fn PushItemFlag(option: ImGuiItemFlags, enabled: bool) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut item_flags: ImGuiItemFlags = g.CurrentItemFlags;
    // IM_ASSERT(item_flags == g.ItemFlagsStack.back());
    if (enabled) {
        item_flags |= option;
    } else {
        item_flags &= !option;
    }
    g.CurrentItemFlags = item_flags;
    g.ItemFlagsStack.push(item_flags);
}

pub unsafe fn PopItemFlag() {
    let g = GImGui; // ImGuiContext& g = *GImGui;
                    // IM_ASSERT(g.ItemFlagsStack.Size > 1); // Too many calls to PopItemFlag() - we always leave a 0 at the bottom of the stack.
    g.ItemFlagsStack.pop_back();
    g.CurrentItemFlags = g.ItemFlagsStack.last().unwrap().clone();
}

pub unsafe fn ActivateItem(id: ImguiHandle) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NavNextActivateId = id;
    g.NavNextActivateFlags = ImGuiActivateFlags_None;
}

// Affect large frame+labels widgets only.
pub unsafe fn SetNextItemWidth(item_width: c_float) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NextItemData.Flags |= ImGuiNextItemDataFlags_HasWidth;
    g.NextItemData.Width = item_width;
}

// FIXME: Remove the == 0.0 behavior?
pub unsafe fn PushItemWidth(item_width: c_float) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.current_window_mut().unwrap();
    window.dc.ItemWidthStack.push(window.dc.ItemWidth); // Backup current width
    window.dc.ItemWidth = (if item_width == 0.0 {
        window.ItemWidthDefault
    } else {
        item_width
    });
    g.NextItemData.Flags &= !ImGuiNextItemDataFlags_HasWidth;
}

pub unsafe fn PushMultiItemsWidths(components: usize, w_full: c_float) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.current_window_mut().unwrap();
    let style = &mut g.style;
    w_item_one: c_float = ImMax(
        1.0,
        IM_FLOOR((w_full - (style.ItemInnerSpacing.x) * (components - 1)) / components),
    );
    let w_item_last: c_float = ImMax(
        1.0,
        IM_FLOOR(w_full - (w_item_one + style.ItemInnerSpacing.x) * (components - 1)),
    );
    window.dc.ItemWidthStack.push(window.dc.ItemWidth); // Backup current width
    window.dc.ItemWidthStack.push(w_item_last);
    // for (let i: c_int = 0; i < components - 2; i++)
    for i in 0..components - 2 {
        window.dc.ItemWidthStack.push(w_item_one);
    }
    window.dc.ItemWidth = if components == 1 {
        w_item_last
    } else {
        w_item_one
    };
    g.NextItemData.Flags &= !ImGuiNextItemDataFlags_HasWidth;
}

pub unsafe fn PopItemWidth() {
    let mut window = g.current_window_mut().unwrap();
    window.dc.ItemWidth = window.dc.ItemWidthStack.last().unwrap().clone();
    window.dc.ItemWidthStack.pop_back();
}

// Calculate default item width given value passed to PushItemWidth() or SetNextItemWidth().
// The SetNextItemWidth() data is generally cleared/consumed by ItemAdd() or NextItemData.ClearFlags()CalcItemWidth: c_float()
pub fn CalcItemWidth(g: &mut ImguiContext) -> f32 {
    let mut window = g.current_window_mut().unwrap();
    let mut w: c_float = 0.0;
    if g.next_item_data.flags & ImGuiNextItemDataFlags_HasWidth {
        w = g.next_item_data.width;
    } else {
        w = window.dc.item_width;
    }
    if (w < 0.0) {
        let region_max_x: c_float = content_region_max_abs(g).x;
        w = ImMax(1.0, region_max_x - window.dc.cursor_pos.x + w);
    }
    w = IM_FLOOR(w);
    return w;
}

// [Internal] Calculate full item size given user provided 'size' parameter and default width/height. Default width is often == CalcItemWidth().
// Those two functions CalcItemWidth vs CalcItemSize are awkwardly named because they are not fully symmetrical.
// Note that only CalcItemWidth() is publicly exposed.
// The 4.0 here may be changed to match CalcItemWidth() and/or BeginChild() (right now we have a mismatch which is harmless but undesirable)
// CalcItemSize: ImVec2(size: ImVec2,default_w: c_float,default_h: c_float)
pub unsafe fn CalcItemSize(mut size: ImVec2, default_w: c_float, default_h: c_float) -> ImVec2 {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.current_window_mut().unwrap();

    let mut region_max = ImVec2::default();
    if size.x < 0.0 || size.y < 0.0 {
        region_max = content_region_max_abs(g);
    }

    if size.x == 0.0 {
        size.x = default_w;
    } else if size.x < 0.0 {
        size.x = ImMax(4.0, region_max.x - window.dc.cursor_pos.x + size.x);
    }

    if size.y == 0.0 {
        size.y = default_h;
    } else if size.y < 0.0 {
        size.y = ImMax(4.0, region_max.y - window.dc.cursor_pos.y + size.y);
    }

    return size.clone();
}
