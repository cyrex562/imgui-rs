use std::collections::HashSet;
use crate::{Context, INVALID_ID};
use crate::rect::Rect;
use crate::types::Id32;
use crate::window::{HoveredFlags, Window, WindowFlags};
use crate::window::get::{get_combined_root_window, is_window_child_of};

// bool ImGui::is_window_above(Window* potential_above, Window* potential_below)
pub fn is_window_above(g: &mut Context, potential_above: &mut Window, potential_below: &mut Window) -> bool
{
    // ImGuiContext& g = *GImGui;

    // It would be saner to ensure that display layer is always reflected in the g.windows[] order, which would likely requires altering all manipulations of that array
    let display_layer_delta = get_window_display_layer(potential_above) - get_window_display_layer(potential_below);
    if (display_layer_delta != 0)
        return display_layer_delta > 0;

    for (int i = g.windows.len() - 1; i >= 0; i -= 1 )
    {
        Window* candidate_window = g.windows[i];
        if (candidate_window == potential_above)
            return true;
        if (candidate_window == potential_below)
            return false;
    }
    return false;
}


// bool ImGui::IsWindowHovered(ImGuiHoveredFlags flags)
pub fn is_window_hovered(g: &mut Context, flags: &HashSet<HoveredFlags>) -> bool
{
    // IM_ASSERT((flags & (ImGuiHoveredFlags_AllowWhenOverlapped | ImGuiHoveredFlags_AllowWhenDisabled)) == 0);   // flags not supported by this function
    // ImGuiContext& g = *GImGui;
    // Window* ref_window = g.hovered_window;
    let ref_window_id = g.hovered_window_id;

    // Window* cur_window = g.current_window;
    let mut cur_window = g.current_window_mut();
    if ref_window_id == INVALID_ID {
        return false;
    }
    let ref_window = g.window_mut(ref_window_id);

    if (flags.contains(&HoveredFlags::AnyWindow)) == false
    {
        // IM_ASSERT(cur_window); // Not inside a Begin()/End()
        let popup_hierarchy = flags.contains(&HoveredFlags::NoPopupHierarchy) == 0;
        let dock_hierarchy = flags.contains(&HoveredFlags::DockHierarchy) != 0;
        if flags.contains( &HoveredFlags::RootWindow) {
            cur_window = get_combined_root_window(g, cur_window, popup_hierarchy, dock_hierarchy);
        }

        let mut result = false;
        if flags.contains(& HoveredFlags::ChildWindows) {
            result = is_window_child_of(g, ref_window, cur_window, popup_hierarchy, dock_hierarchy);
        }
        else {
            result = (ref_window == cur_window);
        }
        if !result {
            return false;
        }
    }

    if !is_window_content_hoverable(g, ref_window, flags) {
        return false;
    }
    if !flags.contains( & HoveredFlags::AllowWhenBlockedByActiveItem) {
        if g.active_id != 0 && !g.active_id_allow_overlap && g.active_id != ref_window.move_id {
            return false;
        }
    }
    return true;
}



// bool ImGui::IsWindowFocused(ImGuiFocusedFlags flags)
pub fn is_window_focused(g: &mut Context, flags: &mut HashSet<FocusedFlags>)
{
    // ImGuiContext& g = *GImGui;
    Window* ref_window = g.nav_window;
    Window* cur_window = g.current_window;

    if (ref_window == None)
        return false;
    if (flags & ImGuiFocusedFlags_AnyWindow)
        return true;

    // IM_ASSERT(cur_window); // Not inside a Begin()/End()
    let popup_hierarchy = (flags & ImGuiFocusedFlags_NoPopupHierarchy) == 0;
    let dock_hierarchy = (flags & ImGuiFocusedFlags_DockHierarchy) != 0;
    if (flags & HoveredFlags::RootWindow)
        cur_window = get_combined_root_window(cur_window, popup_hierarchy, dock_hierarchy);

    if (flags & HoveredFlags::ChildWindows)
        return is_window_child_of(ref_window, cur_window, popup_hierarchy, dock_hierarchy);
    else
        return (ref_window == cur_window);
}

// bool ImGui::IsWindowDocked()
pub fn is_window_docked(g: &mut Context) -> bool
{
    // ImGuiContext& g = *GImGui;
    return g.current_window.dock_is_active;
}

// Can we focus this window with CTRL+TAB (or PadMenu + PadFocusPrev/PadFocusNext)
// Note that NoNavFocus makes the window not reachable with CTRL+TAB but it can still be focused with mouse or programmatically.
// If you want a window to never be focused, you may use the e.g. NoInputs flag.
// bool ImGui::IsWindowNavFocusable(Window* window)
pub fn is_window_nav_focusable(g: &mut Context, window: &mut Window) -> bool
{
    return window.was_active && window == window.root_window && !(window.flags & WindowFlags::NoNavFocus);
}

// static inline bool IsWindowContentHoverable(Window* window, ImGuiHoveredFlags flags)
pub fn is_window_content_hoverable(
    g: &mut Context,
    window: &mut Window,
    flags: &HashSet<HoveredFlags>,
) -> bool {
    // An active popup disable hovering on other windows (apart from its own children)
    // FIXME-OPT: This could be cached/stored within the window.
    // ImGuiContext& g = *GImGui;
    if g.nav_window_id {
        let nav_win = g.window_mut(g.nav_window_id).unwrap();
        let focused_root_window = g.window_mut(nav_win.root_window_dock_tree_id).unwrap();
        if focused_root_window.was_active
            && focused_root_window.id != window.root_window_dock_tree_id
        {
            if focused_root_window.flags.contains(&WindowFlags::Modal) {
                return false;
            }
            if focused_root_window.flags.contains(&WindowFlags::Popup)
                && flags.contains(&HoveredFlags::AllowWhenBlockedByPopup)
            {
                return false;
            }
        }
        // if Window * focused_root_window = g.nav_window_id.root_window_dock_tree {
        //     if focused_root_window.was_active && focused_root_window != window.root_window_dock_tree_id {
        //         // For the purpose of those flags we differentiate "standard popup" from "modal popup"
        //         // NB: The order of those two tests is important because Modal windows are also Popups.
        //         if focused_root_window.flags & WindowFlags::Modal {
        //             return false;
        //         }
        //         if (focused_root_window.flags & WindowFlags::Popup) && !(flags & HoveredFlags::AllowWhenBlockedByPopup) {
        //             return false;
        //         }
        //     }
        // }
    }
    // Filter by viewport
    let moving_win = g.window_mut(g.moving_window_id).unwrap();
    if window.viewport_id != g.mouse_viewport_id
        && (g.moving_window_id == INVALID_ID
            || window.root_window_dock_tree_id != moving_win.root_window_dock_tree)
    {
        return false;
    } else {
    }

    return true;
}

// bool ImGui::is_clipped_ex(const ImRect& bb, Id32 id)
pub fn is_clipped_ex(g: &mut Context, bb: &Rect, id: Id32) -> Result<bool, &'static str> {
    // ImGuiContext& g = *GImGui;
    // Window* window = g.CurrentWindow;
    let window = g.current_window_mut()?;
    if !bb.Overlaps(&window.clip_rect) {
        if id == 0 || (id != g.active_id && id != g.nav_id) {
            if !g.log_enabled {
                return Ok(true);
            }
        }
    }
    return Ok(false);
}

// static bool IsWindowActiveAndVisible(Window* window)
pub fn is_window_active_and_visible(window: &mut Window) -> bool {
    return (window.active) && (!window.hidden);
}

/// static int IMGUI_CDECL ChildWindowComparer(const void* lhs, const void* rhs)
pub fn child_window_comparer(lhs: &Window, rhs: &Window) -> i32 {
    // const Window* const a = *(const Window* const *)lhs;
    // const Window* const b = *(const Window* const *)rhs;
    if lhs.flags.contains(&WindowFlags::Popup) - rhs.flags.contains(&WindowFlags::Popup) {
        return 1;
    }
    if lhs.flags.contains(&WindowFlags::Tooltip) - rhs.flags.contains(&WindowFlags::Tooltip) {
        return 1;
    }
    return (lhs.begin_order_within_parent - rhs.begin_order_within_parent) as i32;
}
