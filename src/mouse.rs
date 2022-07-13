use crate::context::Context;
use crate::input::WINDOWS_MOUSE_WHEEL_SCROLL_LOCK_TIMER;
use crate::math::floor_vector_2d;
use crate::rect::Rect;
use crate::types::INVALID_ID;
use crate::vectors::{ImLengthSqr, Vector2D};
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
            let max_step = window.inner_rect.GetWidth() * 0.67;
            let scroll_step = f32::floor(f32::min(2 * window.CalcFontSize(), max_step));
            set_scroll_x(window, window.scroll.x - wheel_x * scroll_step);
        }
    }
}
