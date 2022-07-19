use crate::{Context, ViewportFlags};
use crate::axis::Axis;
use crate::border::get_resize_border_rect;
use crate::direction::Direction;
use crate::input::{InputSource, MouseCursor, NavLayer};
use crate::math::swap_f32;
use crate::nav::NAV_RESIZE_SPEED;
use crate::rect::Rect;
use crate::resize::{RESIZE_GRIP_DEF, ResizeGripDef};
use crate::style::get_color_u32;
use crate::vectors::two_d::Vector2D;
use crate::window::{calc_window_size_after_constraint, Window, WindowFlags, WINDOWS_HOVER_PADDING, WINDOWS_RESIZE_FROM_EDGES_FEEDBACK_TIMER};
use crate::window::size::calc_resize_pos_size_from_any_corner;

// Handle resize for: Resize Grips, Borders, Gamepad
// Return true when using auto-fit (double click on resize grip)
// static bool ImGui::UpdateWindowManualResize(ImGuiWindow* window, const Vector2D& size_auto_fit, int* border_held, int resize_grip_count, ImU32 resize_grip_col[4], const Rect& visibility_rect)
pub fn update_window_manual_resize(g: &mut Context, window: &mut Window, size_auto_fit: &Vector2D, border_held: &mut i32, resize_grip_count: i32, mut resize_grip_col: [u32;4], visibility_rect: &Rect) -> bool
{
    // ImGuiContext& g = *GImGui;
    // ImGuiWindowFlags flags = window.flags;
    let flags = &window.flags;

    if flags.contains(&WindowFlags::NoResize) || flags.contains(&WindowFlags::AlwaysAutoResize) || window.auto_fit_frames_x > 0 || window.auto_fit_frames_y > 0 {
        return false;
    }
    if window.was_active == false {// Early out to avoid running this code for e.g. an hidden implicit/fallback Debug window.
        return false;
    }

    let mut ret_auto_fit = false;
    // const int resize_border_count = g.io.ConfigWindowsResizeFromEdges ? 4 : 0;
    let resize_border_count = if g.io.config_windows_resize_from_edges { 4 } else {0};
    // const float grip_draw_size = f32::floor(ImMax(g.FontSize * 1.35, window.WindowRounding + 1.0 + g.FontSize * 0.2));
    let grip_draw_size = f32::floor(f32::max(g.font_size * 1.35, window.window_rounding + 1.0 + g.font_size * 0.2));

    // const float grip_hover_inner_size = f32::floor(grip_draw_size * 0.75);
    let grip_hover_inner_size = f32::floor(grip_draw_size * 0.75);
    // const float grip_hover_outer_size = g.io.ConfigWindowsResizeFromEdges ? WINDOWS_HOVER_PADDING : 0.0;
    let grip_hover_outer_size = if g.io.config_windows_resize_from_edges { WINDOWS_HOVER_PADDING} else { 0.0};

    // Vector2D pos_target(f32::MAX, f32::MAX);
    let mut pos_target = Vector2D::new(f32::MAX, f32::MAX);
    // Vector2D size_target(f32::MAX, f32::MAX);
    let mut size_target = Vector2D::new(f32::MAX, f32::MAX);

    // Clip mouse interaction rectangles within the viewport rectangle (in practice the narrowing is going to happen most of the time).
    // - Not narrowing would mostly benefit the situation where OS windows _without_ decoration have a threshold for hovering when outside their limits.
    //   This is however not the case with current backends under Win32, but a custom borderless window implementation would benefit from it.
    // - When decoration are enabled we typically benefit from that distance, but then our resize elements would be conflicting with OS resize elements, so we also narrow.
    // - Note that we are unable to tell if the platform setup allows hovering with a distance threshold (on Win32, decorated window have such threshold).
    // We only clip interaction so we overwrite window->clip_rect, cannot call push_clip_rect() yet as draw_list is not yet setup.
    // const bool clip_with_viewport_rect = !(g.io.backend_flags & ImGuiBackendFlags_HasMouseHoveredViewport) || (g.io.MouseHoveredViewport != window.viewport_id) || !(window.viewport.flags & ImGuiViewportFlags_NoDecoration);
    let clip_with_viewport_rect = !g.io.backend_flags.contains(&BackendFlags::HasMouseHoveredViewport) || g.io.mouse_hovered_viewport != window.viewport_id || !g.get_viewport(window.viewport_id).unwrap().flags.contains(&ViewportFlags::NoDecoration);
    if clip_with_viewport_rect {
        window.clip_rect = window.viewport.get_main_rect();
    }

    // Resize grips and borders are on layer 1
    window.dcnav_layer_current = NavLayer::Menu;

    // Manual resize grips
    // PushID("#RESIZE");
    push_id("#RESIZE");
    // for (int resize_grip_n = 0; resize_grip_n < resize_grip_count; resize_grip_n += 1)
    for resize_grip_n in 0 .. resize_grip_count
    {
        // const ImGuiResizeGripDef& def = resize_grip_def[resize_grip_n];
        let def: &ResizeGripDef = &RESIZE_GRIP_DEF[resize_grip_n];
        // const Vector2D corner = ImLerp(window.pos, window.pos + window.size, def.CornerPosN);
        let cornder = Vector2D::lerp2(&window.pos, &window.pos + &window.size, &def.corner_pos_n);

        // Using the FlattenChilds button flag we make the resize button accessible even if we are hovering over a child window
        // bool hovered, held;
        let mut hovered = false;
        let mut held = false;
        // Rect resize_rect(corner - def.inner_dir * grip_hover_outer_size, corner + def.inner_dir * grip_hover_inner_size);
        let mut resize_rect = Rect::new2(corner - &def.inner_dir * grip_hover_outer_size, corner + &def.inner_dir * grip_hover_inner_size);
        if resize_rect.min.x > resize_rect.max.x { swap_f32(&mut resize_rect.min.x, &mut resize_rect.max.x) };
        if resize_rect.min.y > resize_rect.max.y { swap_f32(&mut resize_rect.min.y, &mut resize_rect.max.y) };
        let resize_grip_id = window.get_id3(g, resize_grip_n); // == GetWindowResizeCornerID()
        keep_alive_id(resize_grip_id);
        button_behavior(resize_rect, resize_grip_id, &hovered, &held, ButtonFlags::FlattenChildren | ButtonFlags::NoNavFocus);
        //GetForegroundDrawList(window)->add_rect(resize_rect.min, resize_rect.max, IM_COL32(255, 255, 0, 255));
        if hovered || held {
            g.mouse_cursor = if resize_grip_n & 1 {
                ImGuiMouseCursor_ResizeNESW
            } else { ImGuiMouseCursor_ResizeNWSE };
        }

        if held && g.io.mouse_clicked_count[0] == 2 && resize_grip_n == 0
        {
            // Manual auto-fit when double-clicking
            size_target = calc_window_size_after_constraint(g, window, size_auto_fit);
            ret_auto_fit = true;
            clear_active_id();
        }
        else if held
        {
            // Resize from any of the four corners
            // We don't use an incremental mouse_delta but rather compute an absolute target size based on mouse position
            // Vector2D clamp_min = Vector2D::new(def.CornerPosN.x == 1.0 ? visibility_rect.min.x : -f32::MAX, def.CornerPosN.y == 1.0 ? visibility_rect.min.y : -f32::MAX);
            let in_x
            let clamp_min = Vector2D::new(if def.corner_pos_n.x == 1.0 { visibility_rect.min.x }  else {-f32::MAX}, if def.corner_pos_n.y == 1.0 { visibility_rect.min.y } else { -f32::MAX});

            // Vector2D clamp_max = Vector2D::new(def.CornerPosN.x == 0.0 ? visibility_rect.max.x : +f32::MAX, def.CornerPosN.y == 0.0 ? visibility_rect.max.y : +f32::MAX);
            let clamp_max = Vector2D::new(
                if def.corner_pos_n.x == 0.0 { visibility_rect.max.x } else {f32::MAX},
                if def.corner_pos_n.y == 0.0 { visibility_rect.max.y} else {f32::MAX}
            );

            // Vector2D corner_target = g.io.mouse_pos - g.ActiveIdClickOffset + ImLerp(def.inner_dir * grip_hover_outer_size, def.inner_dir * -grip_hover_inner_size, def.CornerPosN); // Corner of the window corresponding to our corner grip
            let corner_target = &g.io.mouse_pos - &g.active_id_click_offset + Vector2D::lerp2(&(&def.inner_dir * grip_hover_outer_size), (&def.inner_dir * -grop_hover_inner_size), &def.corner_pos_n);

            // corner_target = ImClamp(corner_target, clamp_min, clamp_max);
            let corner_target = Vector2D::clamp(corner_target, &clamp_min, &clamp_max);
            // CalcResizePosSizeFromAnyCorner(window, corner_target, def.CornerPosN, &pos_target, &size_target);
            calc_resize_pos_size_from_any_corner(g, window, &corner_target, &def.corner_pos_n, &mut pos_target, &mut size_target);
        }

        // Only lower-left grip is visible before hovering/activating
        if resize_grip_n == 0 || held || hovered {
            resize_grip_col[resize_grip_n] = get_color_u32(if held { StyleColor::ResizeGripActive } else { if hovered { StyleColor::ResizeGripHovered } else { StyleColor::ResizeGrip }}, 0.0);
        }
    }
    // for (int border_n = 0; border_n < resize_border_count; border_n += 1)
    for border_n in 0 .. resize_border_count
    {
        // const ImGuiResizeBorderDef& def = resize_border_def[border_n];
        let def = resize_border_def[border_n];
        // const ImGuiAxis axis = (border_n == Dir::Left || border_n == Dir::Right) ? Axis::X : Axis::Y;
        let axis = if border_n == Dir::Left || border_n == Dir::Right { Axis::X } else { Axis::Y};

        // bool hovered, held;
        let mut hovered = false;
        let mut held = false;
        // Rect border_rect = GetResizeBorderRect(window, border_n, grip_hover_inner_size, WINDOWS_HOVER_PADDING);
        let mut border_rect = get_resize_border_rect(g, window, border_n, grip_hover_inner_size, WINDOWS_HOVER_PADDING);
        // ImGuiID border_id = window.GetID(border_n + 4); // == GetWindowResizeBorderID()
        let border_id = window.get_id3(g, border_n + 4);
        keep_alive_id(border_id);
        button_behavior(border_rect, border_id, &hovered, &held, ButtonFlags::FlattenChildren | ButtonFlags::NoNavFocus);
        //GetForegroundDrawLists(window)->add_rect(border_rect.min, border_rect.max, IM_COL32(255, 255, 0, 255));
        if (hovered && g.hovered_id_timer > WINDOWS_RESIZE_FROM_EDGES_FEEDBACK_TIMER) || held
        {
            // g.mouse_cursor = (axis == Axis::X) ? ImGuiMouseCursor_ResizeEW : ImGuiMouseCursor_ResizeNS;
            g.mouse_cursor = if axis == Axis::X { MouseCursor::ResizeEW} else { MouseCurosr::ResizeNS};
            if held {
                *border_held = border_n;
            }
        }
        if held
        {
            // Vector2D clamp_min(border_n == Dir::Right ? visibility_rect.min.x : -f32::MAX, border_n == Dir::Down ? visibility_rect.min.y : -f32::MAX);
            let clamp_min = Vector2D::new(
                if border_n == Direction::Right as i32 { visibility_rect.min_x } else { -f32::MAX },
                if border_n == Direction::Down as i32 { visibility_rect.min_y } else { -f32::MAX }
            );

            // Vector2D clamp_max(border_n == Dir::Left  ? visibility_rect.max.x : +f32::MAX, border_n == Dir::Up   ? visibility_rect.max.y : +f32::MAX);
            let clamp_max = Vector2D::new(
                if border_n == Direction::Left as i32 { visibility_rect.max.x } else { f32::MAX},
                if border_n == Direction::Up as i32 { visibility_rect.max.y } else { f32::MAX}
            );

            // Vector2D border_target = window.pos;
            let mut border_target = window.pos.clone();
            // border_target[axis] = g.io.mouse_pos[axis] - g.ActiveIdClickOffset[axis] + WINDOWS_HOVER_PADDING;
            border_target[&axis] = g.io.mouse_pos[&axis] - g.active_id_click_offset[&axis] + WINDOWS_HOVER_PADDING;
            // border_target = ImClamp(border_target, clamp_min, clamp_max);
            border_target = Vector2D::clamp(&border_target, &clamp_min, &clamp_max);
            calc_resize_pos_size_from_any_corner(g, window, &border_target, &Vector2D::min(&def.segment_n1, &def.segment_n2), &mut pos_target, &mut size_target);
        }
    }
    pop_id();

    // Restore nav layer
    window.dcnav_layer_current = NavLayer::Main;

    // Navigation resize (keyboard/gamepad)
    if g.nav_windowing_target && g.nav_windowing_target.root_window_dock_tree == window
    {
        // Vector2D nav_resize_delta;
        let mut nav_resize_delta = Vector2D::default();
        if g.nav_input_source == InputSource::Keyboard && g.io.key_shift {
            nav_resize_delta = get_nav_input_amount_2d(NavDirSourceFlags::RawKeyboard, NavReadMode::Down);
        }
        if g.nav_input_source == InputSource::Gamepad {
            nav_resize_delta = get_nav_input_amount_2d(NavDirSourceFlags::PadDPad, NavReadMode::Down);
        }
        if nav_resize_delta.x != 0.0 || nav_resize_delta.y != 0.0
        {
            // const float NAV_RESIZE_SPEED = 600.0;

            nav_resize_delta *= f32::floor(NAV_RESIZE_SPEED * g.io.delta_time * ImMin(g.io.display_frame_buffer_scale.x, g.io.display_frame_buffer_scale.y));
            nav_resize_delta = ImMax(nav_resize_delta, &visibility_rect.min - &window.pos - &window.size);
            g.NavWindowingToggleLayer = false;
            g.nav_disable_mouse_hover = true;
            resize_grip_col[0] = get_color_u32(StyleColor::ResizeGripActive);
            // FIXME-NAV: Should store and accumulate into a separate size buffer to handle sizing constraints properly, right now a constraint will make us stuck.
            size_target = calc_window_size_after_constraint(g, window, &(&window.size_full + nav_resize_delta));
        }
    }

    // Apply back modified position/size to window
    if size_target.x != f32::MAX
    {
        window.size_full = size_target;
        MarkIniSettingsDirty(window);
    }
    if pos_target.x != f32::MAX
    {
        window.pos = Vector2D::floor(pos_target);
        MarkIniSettingsDirty(window);
    }

    window.size = window.size_full.clone();
    return ret_auto_fit;
}
