use crate::condition::Condition;
use crate::config::ConfigFlags;
use crate::context::{Context, set_active_id_using_nav_and_keys};
use crate::dock_node::{dock_node_get_root_node, DockNode};
use crate::id::set_active_id;
use crate::input::WINDOWS_MOUSE_WHEEL_SCROLL_LOCK_TIMER;
use crate::math::floor_vector_2d;
use crate::rect::Rect;
use crate::types::INVALID_ID;
use crate::utils::remove_hash_set_val;
use crate::vectors::ImLengthSqr;
use crate::{Viewport, ViewportFlags};
use crate::vectors::two_d::Vector2D;
use crate::window::{Window, WindowFlags};

/// Test if mouse cursor is hovering given rectangle
/// NB- Rectangle is clipped by our current clip setting
/// NB- Expand the rectangle to be generous on imprecise inputs systems (g.style.TouchExtraPadding)
/// bool ImGui::IsMouseHoveringRect(const Vector2D& r_min, const Vector2D& r_max, bool clip)
pub fn is_mouse_hovering_rect(g: &mut Context, r_min: &Vector2D, r_max: &Vector2D, clip: bool) -> bool {
    // ImGuiContext& g = *GImGui;

    // Clip
    // ImRect rect_clipped(r_min, r_max);
    let mut rect_clipped = Rect {
        min: r_min.clone(),
        max: r_max.clone(),
    };

    if clip {
        let curr_win = g.get_current_window()?;
        rect_clipped.ClipWith(&curr_win.clip_rect);
    }

    // Expand for touch input
    let min_1 = rect_clipped.min - g.style.touch_extra_padding;
    let max_1 = rect_clipped.max - g.style.touch_extra_padding;
    let rect_for_touch = Rect::new2(&min_1, &max_1);
    if !rect_for_touch.Contains(g.io.mouse_pos) {
        return false;
    }
    if !g.mouse_viewport_id.get_main_rect().Overlaps(&rect_clipped){
        return false;
    }
    return true;
}

// static void ImGui::UpdateMouseInputs()
pub fn update_mouse_inputs(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    // ImGuiIO& io = g.io;
    let io = &mut g.io;

    // Round mouse position to avoid spreading non-rounded position (e.g. UpdateManualResize doesn't support them well)
    if is_mouse_pos_valid(&io.mouse_pos) {
        g.mouse_last_valid_pos =  floor_vector_2d(&io.mouse_pos);
        io.mouse_pos = g.mouse_last_valid_pos.clone();
    }

    // If mouse just appeared or disappeared (usually denoted by -FLT_MAX components) we cancel out movement in mouse_delta
    if is_mouse_pos_valid(&io.mouse_pos) && is_mouse_pos_valid(&io.mouse_pos_prev) {
        io.mouse_delta = &io.mouse_pos - &io.mouse_pos_prev;
    }
    else {
        io.mouse_delta = Vector2D::new(0.0, 0.0);
    }

    // If mouse moved we re-enable mouse hovering in case it was disabled by gamepad/keyboard. In theory should use a >0.0 threshold but would need to reset in everywhere we set this to true.
    if io.mouse_delta.x != 0.0 || io.mouse_delta.y != 0.0 {
        g.nav_disable_mouse_hover = false;
    }

    io.mouse_pos_prev = io.mouse_pos.clone();
    // for (int i = 0; i < IM_ARRAYSIZE(io.mouse_down); i += 1)
    for i in 0 .. io.mouse_down.len()
    {
        io.mouse_clicked[i] = io.mouse_down[i] && io.mouse_down_duration[i] < 0.0;
        io.mouse_clicked_count[i] = 0; // Will be filled below
        io.mouse_released[i] = !io.mouse_down[i] && io.mouse_down_duration[i] >= 0.0;
        io.mouse_down_duration_prev[i] = io.mouse_down_duration[i];
        io.mouse_down_duration[i] = if io.mouse_down[i] { (if io.mouse_down_duration[i] < 0.0 { 0.0 } else { io.mouse_down_duration[i] + io.delta_time })} else { -1.0 };
        if io.mouse_clicked[i]
        {
            let mut is_repeated_click = false;
            if (g.time - io.mouse_clicked_time[i]) < io.mouse_double_click_time
            {
                let delta_from_click_pos = if is_mouse_pos_valid(&io.mouse_pos) { (&io.mouse_pos - &io.mouse_clicked_pos[i]) } else { Vector2D::new(0.0, 0.0) };
                if ImLengthSqr(&delta_from_click_pos) < io.mouse_double_click_max_dist * io.mouse_double_click_max_dist {
                    is_repeated_click = true;
                }
            }
            if is_repeated_click {
                io.mouse_clicked_last_count[i] += 1;
            }
            else {
                io.mouse_clicked_last_count[i] = 1;
            }
            io.mouse_clicked_time[i] = g.time as f64;
            io.mouse_clicked_pos[i] = io.mouse_pos.clone();
            io.mouse_clicked_count[i] = io.mouse_clicked_last_count[i];
            io.mouse_drag_max_distance_abs[i] = Vector2D::new(0.0, 0.0);
            io.mouse_drag_max_distance_sqr[i] = 0.0;
        }
        else if io.mouse_down[i]
        {
            // Maintain the maximum distance we reaching from the initial click position, which is used with dragging threshold
            let delta_from_click_pos = if is_mouse_pos_valid(&io.mouse_pos) { (&io.mouse_pos - &io.mouse_clicked_pos[i]) } else {Vector2D::new(0.0, 0.0)};
            io.mouse_drag_max_distance_sqr[i] = f32::max(io.mouse_drag_max_distance_sqr[i], ImLengthSqr(&delta_from_click_pos));
            io.mouse_drag_max_distance_abs[i].x = f32::max(io.mouse_drag_max_distance_abs[i].x, if delta_from_click_pos.x < 0.0 { -delta_from_click_pos.x } else { delta_from_click_pos.x });
            io.mouse_drag_max_distance_abs[i].y = f32::max(io.mouse_drag_max_distance_abs[i].y, if delta_from_click_pos.y < 0.0 { -delta_from_click_pos.y } else { delta_from_click_pos.y });
        }

        // We provide io.mouse_double_clicked[] as a legacy service
        io.mouse_double_clicked[i] = (io.mouse_clicked_count[i] == 2);

        // Clicking any mouse button reactivate mouse hovering which may have been deactivated by gamepad/keyboard navigation
        if io.mouse_clicked[i] {
            g.nav_disable_mouse_hover = false;
        }
    }
}

// static void StartLockWheelingWindow(ImGuiWindow* window)
pub fn start_lock_wheeling_window(g: &mut Context, window: &Window)
{
    // ImGuiContext& g = *GImGui;
    let mut wheeling_window = g.get_window(g.wheeling_window_id).unwrap();
    if wheeling_window == window {
        return;
    }
    *wheeling_window = window.clone();
    g.wheeling_window_ref_mouse_pos = g.io.mouse_pos.clone();
    g.wheeling_window_timer = WINDOWS_MOUSE_WHEEL_SCROLL_LOCK_TIMER;
}

/// void ImGui::UpdateMouseWheel()
pub fn update_mouse_wheel(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;

    // Reset the locked window if we move the mouse or after the timer elapses
    if g.wheeling_window_id != INVALID_ID
    {
        g.wheeling_window_timer -= g.io.delta_time;
        if is_mouse_pos_valid() && ImLengthSqr(&(&g.io.mouse_pos - &g.wheeling_window_ref_mouse_pos)) > g.io.mouse_drag_threshold * g.io.mouse_drag_threshold {
            g.wheeling_window_timer = 0.0;
        }
        if g.wheeling_window_timer <= 0.0
        {
            g.wheeling_window = NULL;
            g.wheeling_window_timer = 0.0;
        }
    }

    let mut wheel_x = g.io.mouse_wheel_h;
    let mut wheel_y = g.io.mouse_wheel;
    if wheel_x == 0.0 && wheel_y == 0.0 {
        return;
    }

    if (g.active_id != 0 && g.active_id_using_mouse_wheel) || (g.hovered_id_previous_frame != 0 && g.hovered_id_previous_frame_using_mouse_wheel) {
        return;
    }

    // ImGuiWindow* window = g.wheeling_window ? g.wheeling_window : g.hovered_window;

    let mut window = if g.wheeling_window_id != INVALID_ID {g.get_window(g.wheeling_window_id).unwrap()} else {
        g.get_window(g.hovered_window).unwrap()
    };
    // if (!window || window.collapsed) {
    if window.collapsed {
        return;
    }

    // Zoom / scale window
    // FIXME-OBSOLETE: This is an old feature, it still works but pretty much nobody is using it and may be best redesigned.
    if wheel_y != 0.0 && g.io.key_ctrl && g.io.font_allow_user_scaling
    {
        start_lock_wheeling_window(g, window);
        let new_font_scale = f32::clamp(window.font_window_scale + g.io.mouse_wheel * 0.10, 0.50, 2.50);
        let scale = new_font_scale / window.font_window_scale;
        window.font_window_scale = new_font_scale;
        if window.id == window.root_window_id
        {
            let offset = &window.size * (1.0 - scale) * (&g.io.mouse_pos - &window.pos) / &window.size;
            set_window_pos(window, &window.pos + offset, 0);
            window.size = Vector2D::floor(&window.size * scale);
            window.size_full = Vector2D::floor(&window.size_full * scale);
        }
        return;
    }

    // Mouse wheel scrolling
    // If a child window has the ImGuiWindowFlags_NoScrollWithMouse flag, we give a chance to scroll its parent
    if g.io.key_ctrl {
        return;
    }

    // As a standard behavior holding SHIFT while using Vertical Mouse Wheel triggers Horizontal scroll instead
    // (we avoid doing it on OSX as it the OS input layer handles this already)
    let swap_axis = g.io.key_shift && !g.io.config_mac_osx_behaviors;
    if swap_axis
    {
        wheel_x = wheel_y;
        wheel_y = 0.0;
    }

    // Vertical Mouse Wheel scrolling
    if wheel_y != 0.0
    {
        start_lock_wheeling_window(g, window);
        while (window.flags.contains(&WindowFlags::ChildWindow)) && ((window.scroll_max.y == 0.0) || ((window.flags.contains(&WindowFlags::NoScrollWithMouse)) && !(window.flags.contains(&WindowFlags::NoMouseInputs)))) {
            window = g.get_window(window.parent_window_id).unwrap();
        }
        if !(window.flags.contains(&WindowFlags::NoScrollWithMouse)) && !(window.flags.contains(&WindowFlags::NoMouseInputs))
        {
            let max_step = window.inner_rect.get_height() * 0.67;
            let scroll_step = f32::floor(f32::min(5 * window.calc_font_size(), max_step));
            set_scroll_y(window, window.scroll.y - wheel_y * scroll_step);
        }
    }

    // Horizontal Mouse Wheel scrolling, or Vertical Mouse Wheel w/ Shift held
    if wheel_x != 0.0
    {
        start_lock_wheeling_window(g, window);
        while (window.flags.contains(&WindowFlags::ChildWindow)) && ((window.scroll_max.x == 0.0) || ((window.flags.contains(&WindowFlags::NoScrollWithMouse)) && !(window.flags.contains( &WindowFlags::NoMouseInputs)))) {
            window = g.get_window(window.parent_window_id).unwrap();
        }
        if !(window.flags.contains(&WindowFlags::NoScrollWithMouse)) && !(window.flags.contains(&WindowFlags::NoMouseInputs))
        {
            let max_step = window.inner_rect.get_width() * 0.67;
            let scroll_step = f32::floor(f32::min(2 * window.CalcFontSize(), max_step));
            set_scroll_x(window, window.scroll.x - wheel_x * scroll_step);
        }
    }
}

/// void ImGui::start_mouse_moving_window(ImGuiWindow* window)
pub fn start_mouse_moving_window(g: &mut Context, window: &mut Window)
{
    // Set active_id even if the _NoMove flag is set. Without it, dragging away from a window with _NoMove would activate hover on other windows.
    // We _also_ call this when clicking in a window empty space when io.config_windows_move_from_title_bar_only is set, but clear g.moving_window afterward.
    // This is because we want active_id to be set even when the window is not permitted to move.
    // ImGuiContext& g = *GImGui;
    // focus_window(window);
    focus_window(window);
    // set_active_id(window.MoveId, window);
    set_active_id(g, window.id, window);
    g.nav_disable_highlight = true;
    g.active_id_click_offset = &g.io.mouse_clicked_pos[0] - window.root_window_dock_tree_id.pos;
    g.ActiveIdNoClearOnFocusLoss = true;
    // SetActiveIdUsingNavAndKeys();
    set_active_id_using_nav_and_keys(g);

    // bool can_move_window = true;
    let mut can_move_window= true;
    if window.flags.contains(&WindowFlags::NoMove) || window.root_window_dock_tree_id.flags.contains(&WindowFlags::NoMove) {
        can_move_window = false;
    }
    let node = &window.dock_node_as_host;
    if node.visible_window && (node.visible_window.flags.contains(WindowFlags::NoMove)) {
    can_move_window = false;}
    if can_move_window {
        g.moving_window_id = window.id;
    }
}

/// We use 'undock_floating_node == false' when dragging from title bar to allow moving groups of floating nodes without undocking them.
/// - undock_floating_node == true: when dragging from a floating node within a hierarchy, always undock the node.
/// - undock_floating_node == false: when dragging from a floating node within a hierarchy, move root window.
/// void ImGui::StartMouseMovingWindowOrNode(ImGuiWindow* window, ImGuiDockNode* node, bool undock_floating_node)
pub fn start_mouse_moving_window_or_node(g: &mut Context, window: &mut Window, node: &mut DockNode, undock_floating_node: bool)
{
    // ImGuiContext& g = *GImGui;
    // bool can_undock_node = false;
    let mut can_undock_node = false;
    // if (node != NULL && node->VisibleWindow && (node->VisibleWindow.flags & ImGuiWindowFlags_NoMove) == 0)
    if node.visible_window != INVALID_ID && node.visible_window.flags.contains(WindowFlags::NoMove) == false
    {
        // Can undock if:
        // - part of a floating node hierarchy with more than one visible node (if only one is visible, we'll just move the whole hierarchy)
        // - part of a dockspace node hierarchy (trivia: undocking from a fixed/central node will create a new node and copy windows)
        // ImGuiDockNode* root_node = DockNodeGetRootNode(node);
        let mut root_node = dock_node_get_root_node(g, node);
        //if (root_node->OnlyNodeWithWindows != node || root_node->CentralNode != NULL)
        if root_node.only_node_with_window != node.id || root_node.central_node != INVALID_ID
        {  // -V1051 PVS-Studio thinks node should be root_node and is wrong about that.
        // if (undock_floating_node || root_node -> IsDockSpace())
        if undock_floating_node || root_node.is_dock_space()
            {
            can_undock_node = true;
        }
    }
    }

    // const bool clicked = IsMouseClicked(0);
    let clicked = is_mouse_clicked(g, 0);
    // const bool dragging = IsMouseDragging(0, g.io.MouseDragThreshold * 1.70);
    let dragging = is_mouse_dragging(g, 0, g.io.mouse_drag_threshold * 1.70);
    if can_undock_node && dragging {
        dock_context_queue_undock_node(&g, node); // Will lead to DockNodeStartMouseMovingWindow() -> start_mouse_moving_window() being called next frame
    }
    else if !can_undock_node && (clicked || dragging) && g.moving_window_id != window.id {
        start_mouse_moving_window(g, window);
    }
}

/// Handle mouse moving window
/// Note: moving window with the navigation keys (Square + d-pad / CTRL+TAB + Arrows) are processed in NavUpdateWindowing()
/// FIXME: We don't have strong guarantee that g.moving_window stay synched with g.active_id == g.moving_window->move_id.
/// This is currently enforced by the fact that BeginDragDropSource() is setting all g.ActiveIdUsingXXXX flags to inhibit navigation inputs,
/// but if we should more thoroughly test cases where g.active_id or g.moving_window gets changed and not the other.
/// void ImGui::UpdateMouseMovingWindowNewFrame()
pub fn update_mouse_moving_window_new_frame(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    if g.moving_window_id != INVALID_ID
    {
        // We actually want to move the root window. g.moving_window == window we clicked on (could be a child window).
        // We track it to preserve Focus and so that generally active_id_window == moving_window and active_id == moving_window->move_id for consistency.
        keep_alive_id(g.active_id);
        // IM_ASSERT(g.moving_window && g.moving_window->RootWindowDockTree);
        // ImGuiWindow* moving_window = g.moving_window->RootWindowDockTree;
        let moving_window_id = g.get_window(g.moving_window_id).unwrap().root_window_dock_tree_id;
        let moving_window = g.get_window(moving_window_id).unwrap();

        // When a window stop being submitted while being dragged, it may will its viewport until next Begin()
        // const bool window_disappared = ((!moving_window.WasActive && !moving_window.Active) || moving_window.viewport == NULL);
        let window_disappeared = (!moving_window.was_active && !moving_window.active) || moving_window.viewport_id == INVALID_ID;
        if g.io.mouse_down[0] && is_mouse_pos_valid(&g.io.mouse_pos) && !window_disappeared
        {
            // Vector2D pos = g.io.mouse_pos - g.ActiveIdClickOffset;
            let mut pos = g.io.mouse_pos.clone() - g.active_id_click_offset.clone();
            if moving_window.pos.x != pos.x || moving_window.pos.y != pos.y
            {
                set_window_pos(moving_window, pos, Condition::Always);
                if moving_window.viewport_owned // Synchronize viewport immediately because some overlays may relies on clipping rectangle before we Begin() into the window.
                {
                    moving_window.viewport_id.pos = pos.clone();
                    moving_window.viewport_id.update_work_rect();
                }
            }
            focus_window(g.moving_window_id);
        }
        else
        {
            if !window_disappeared
            {
                // Try to merge the window back into the main viewport.
                // This works because mouse_viewport should be != moving_window->viewport on release (as per code in UpdateViewports)
                if g.config_flags_curr_frame.contains(&ConfigFlags::ViewportsEnable) {
                    update_try_merge_window_into_host_viewport(moving_window, g.mouse_viewport_id);
                }

                // Restore the mouse viewport so that we don't hover the viewport _under_ the moved window during the frame we released the mouse button.
                if !is_drag_drop_payload_being_accepted() {
                    g.mouse_viewport_id = moving_window.viewport_id;
                }

                // clear the NoInput window flag set by the viewport system
                // moving_window.viewport.flags &= ~ViewportFlags::NoInputs; // FIXME-VIEWPORT: Test engine managed to crash here because viewport was NULL.
                let mut viewport: &mut Viewport = g.get_viewport(moving_window.viewport_id).unwrap();
                remove_hash_set_val(&mut viewport.flags, &ViewportFlags::NoInputs)
            }

            g.moving_window_id = INVALID_ID;
            clear_active_id();
        }
    }
    else
    {
        // When clicking/dragging from a window that has the _NoMove flag, we still set the active_id in order to prevent hovering others.
        if g.active_id_window_id != INVALID_ID // && g.active_id_window.move_id == g.active_id)
        {
            let active_id_win = g.get_window(g.active_id_window_id).unwrap();
            if active_id_win.move_id == g.active_id {
                keep_alive_id(g.active_id);
                if !g.io.mouse_down[0] {
                    clear_active_id();
                }
            }
        }
    }
}

/// Initiate moving window when clicking on empty space or title bar.
/// Handle left-click and right-click focus.
/// void ImGui::UpdateMouseMovingWindowEndFrame()
pub fn update_mouse_moving_window_end_frame(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    if g.active_id != INVALID_ID || g.hovered_id != INVALID_ID {
        return;
    }

    // Unless we just made a window/popup appear
    // if (g.nav_window && g.nav_window.appearing) {
    //     return;
    // }
    if g.nav_window_id != INVALID_ID {
        let win = g.get_window(g.nav_window_id).unwrap();
        if win.appearing {
            return;
        }
    }

    // Click on empty space to focus window and start moving
    // (after we're done with all our widgets, so e.g. clicking on docking tab-bar which have set hovered_id already and not get us here!)
    if g.io.mouse_clicked[0]
    {
        // Handle the edge case of a popup being closed while clicking in its empty space.
        // If we try to focus it, focus_window() > close_popups_over_window() will accidentally close any parent popups because they are not linked together any more.
        // ImGuiWindow* RootWindow = g.hovered_window ? g.hovered_window->RootWindow : NULL;
        let root_window = if g.hovered_window_id != INVALID_ID {
            let hov_win = g.get_window(g.hovered_window_id).unwrap();
            Some(g.get_window(hov_win.root_window_id).unwrap())
        } else {
            None
        };
        // const bool is_closed_popup = RootWindow && (RootWindow.Flags & ImGuiWindowFlags_Popup) && !IsPopupOpen(RootWindow.PopupId, ImGuiPopupFlags_AnyPopupLevel);
        let is_closed_popup: bool = if root_window.is_some() {
            let root_win = root_window.unwrap();
            if root_win.flags.contains(&WindowFlags::Popup) {
                if is_popup_open(root_win.popup_id, PopupFlags::AnyPopupLevel) {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        };

        // if (RootWindow != NULL && !is_closed_popup)

        if root_window.is_some() && is_closed_popup == false
        {
            let root_win = root_window.unwrap();
            start_mouse_moving_window(g, g.hovered_window); //-V595

            // Cancel moving if clicked outside of title bar
            if g.io.config_windows_move_from_title_bar_only && (!(root_win.flags.contains(&WindowFlags::NoTitleBar)) || root_win.unwrap().dock_is_active) && !root_win.unwrap().title_bar_rect().Contains(&g.io.mouse_clicked_pos[0]) {
                g.moving_window_id = INVALID_ID
            }

            // Cancel moving if clicked over an item which was disabled or inhibited by popups (note that we know hovered_id == 0 already)
            if g.hovered_id_disabled {
                g.moving_window_id = INVALID_ID;
            }
        }
        else if root_window.is_none() && g.nav_window_id != INVALID_ID && get_top_most_popup_modal() == NULL
        {
            // Clicking on void disable focus
            focus_window();
        }
    }

    // With right mouse button we close popups without changing focus based on where the mouse is aimed
    // Instead, focus will be restored to the window under the bottom-most closed popup.
    // (The left mouse button path calls focus_window on the hovered window, which will lead NewFrame->close_popups_over_window to trigger)
    if g.io.mouse_clicked[1]
    {
        // Find the top-most window between hovered_window and the top-most Modal window.
        // This is where we can trim the popup stack.
        let modal = get_top_most_popup_modal();
        let hovered_window_above_modal = g.hovered_window && (modal == NULL || is_window_above(g.hovered_window, modal));
        close_popups_over_window(if hovered_window_above_modal { g.hovered_window } else { modal}, true);
    }
}
