use std::ptr::null_mut;
use libc::c_float;
use crate::focused_flags::ImGuiFocusedFlags;
use crate::GImGui;
use crate::hovered_flags::{ImGuiHoveredFlags, ImGuiHoveredFlags_AllowWhenBlockedByActiveItem, ImGuiHoveredFlags_AnyWindow, ImGuiHoveredFlags_ChildWindows, ImGuiHoveredFlags_DockHierarchy, ImGuiHoveredFlags_NoPopupHierarchy, ImGuiHoveredFlags_RootWindow};
use crate::type_defs::ImGuiID;
use crate::utils::{flag_clear, flag_set};
use crate::vec2::ImVec2;
use crate::window::find::{GetCombinedRootWindow, IsWindowChildOf};
use crate::window::ImGuiWindow;
use crate::window::ops::IsWindowContentHoverable;
use crate::window::window_flags::ImGuiWindowFlags_NoNavFocus;

pub unsafe fn IsWindowHovered(flags: ImGuiHoveredFlags) -> bool
{
    // IM_ASSERT((flags & (ImGuiHoveredFlags_AllowWhenOverlapped | ImGuiHoveredFlags_AllowWhenDisabled)) == 0);   // Flags not supported by this function
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut ref_window: *mut ImGuiWindow =  g.HoveredWindow;
    let mut cur_window: *mut ImGuiWindow =  g.CurrentWindow;
    if ref_window == null_mut() {
        return false;
    }

    if flag_clear(flags, ImGuiHoveredFlags_AnyWindow)
    {
        // IM_ASSERT(cur_window); // Not inside a Begin()/End()
        let popup_hierarchy: bool = flag_clear(flags, ImGuiHoveredFlags_NoPopupHierarchy);
        let dock_hierarchy: bool = flag_set(flags, ImGuiHoveredFlags_DockHierarchy);
        if flags & ImGuiHoveredFlags_RootWindow {
            cur_window = GetCombinedRootWindow(cur_window, popup_hierarchy, dock_hierarchy);
        }

        result: bool;
        if flags & ImGuiHoveredFlags_ChildWindows {
            result = IsWindowChildOf(ref_window, cur_window, popup_hierarchy, dock_hierarchy);
        }
        else {
            result = (ref_window == cur_window);
        }
        if !result {
            return false;
        }
    }

    if !IsWindowContentHoverable(ref_window, flags) {
        return false;
    }
    if flag_clear(flags, ImGuiHoveredFlags_AllowWhenBlockedByActiveItem) {
        if g.ActiveId != 0 && !g.ActiveIdAllowOverlap && g.ActiveId != ref_window.MoveId {
            return false;
        }
    }
    return true;
}



pub unsafe fn IsWindowFocused(flags: ImGuiFocusedFlags) -> bool
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut ref_window: *mut ImGuiWindow =  g.NavWindow;
    let mut cur_window: *mut ImGuiWindow =  g.CurrentWindow;

    if (ref_window == null_mut()) {
        return false;
    }
    if (flags & ImGuiFocusedFlags_AnyWindow) {
        return true;
    }

    // IM_ASSERT(cur_window); // Not inside a Begin()/End()
    let popup_hierarchy: bool = flag_clear(flags, ImGuiFocusedFlags_NoPopupHierarchy);
    let dock_hierarchy: bool = flag_set(flags, ImGuiFocusedFlags_DockHierarchy);
    if (flags & ImGuiHoveredFlags_RootWindow) {
        cur_window = GetCombinedRootWindow(cur_window, popup_hierarchy, dock_hierarchy);
    }

    if (flags & ImGuiHoveredFlags_ChildWindows) {
        return IsWindowChildOf(ref_window, cur_window, popup_hierarchy, dock_hierarchy);
    }
    else {
        return (ref_window == cur_window);
    }
}



pub unsafe fn GetWindowDockID() -> ImGuiID
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.Currentwindow.DockId;
}


pub unsafe fn IsWindowDocked() -> bool
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.Currentwindow.DockIsActive;
}


// Can we focus this window with CTRL+TAB (or PadMenu + PadFocusPrev/PadFocusNext)
// Note that NoNavFocus makes the window not reachable with CTRL+TAB but it can still be focused with mouse or programmatically.
// If you want a window to never be focused, you may use the e.g. NoInputs flag.
pub fn IsWindowNavFocusable(window: *mut ImGuiWindow) -> bool
{
    return window.WasActive && window == window.RootWindow && flag_clear(window.Flags, ImGuiWindowFlags_NoNavFocus);
}


pub unsafe fn GetWindowWidth() -> c_float {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window: *mut ImGuiWindow =  g.CurrentWindow;
    return window.Size.x;
}


pub unsafe fn GetWindowHeight() -> c_float
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window: *mut ImGuiWindow =  g.CurrentWindow;
    return window.Size.y;
}

pub unsafe fn GetWindowPos() -> ImVec2
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    return window.Pos;
}
