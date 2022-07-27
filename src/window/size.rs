use crate::{Context, INVALID_ID, ViewportFlags};
use crate::axis::Axis;
use crate::border::get_resize_border_rect;
use crate::condition::Condition;
use crate::types::Direction;
use crate::globals::GImGui;
use crate::input::{InputSource, MouseCursor, NavLayer};
use crate::math::swap_f32;
use crate::nav::NAV_RESIZE_SPEED;
use crate::rect::Rect;
use crate::resize::{RESIZE_GRIP_DEF, ResizeGripDef};
use crate::size_callback_data::SizeCallbackData;
use crate::style::get_color_u32;
use crate::types::Id32;
use crate::vectors::two_d::Vector2D;
use crate::window::{calc_window_size_after_constraint, get, Window, WindowFlags, WINDOWS_HOVER_PADDING, WINDOWS_RESIZE_FROM_EDGES_FEEDBACK_TIMER};
use crate::window::next_window::NextWindowDataFlags;

// static Vector2D CalcWindowAutoFitSize(ImGuiWindow* window, const Vector2D& size_contents)
pub fn calc_window_auto_fit_size(g: &mut Context, window: &mut Window, size_contents: &Vector2D) -> Vector2D
{
    // ImGuiContext& g = *GImGui;
    // ImGuiStyle& style = g.style;
    let style = &mut g.style;
    // const float decoration_up_height = window.title_bar_height() + window.MenuBarHeight();
    let decoration_up_height = window.title_bar_height() + window.menu_bar_height();
    // Vector2D size_pad = window.WindowPadding * 2.0;
    let size_pad = window.window_padding.clone() * 2.0;
    // Vector2D size_desired = size_contents + size_pad + Vector2D::new(0.0, decoration_up_height);
    let size_desired = size_contents + size_pad + Vector2D::new(0.0, decoration_up_height);
    // if (window.flags & WindowFlags::Tooltip)
    if window.flags.contains(&WindowFlags::Tooltip)
    {
        // Tooltip always resize
        return size_desired;
    }
    else
    {
        // Maximum window size is determined by the viewport size or monitor size
        // const bool is_popup = (window.flags & WindowFlags::Popup) != 0;
        let is_popup = window.flags.contains(&WindowFlags::Popup);
        // const bool is_menu = (window.flags & WindowFlags::ChildMenu) != 0;
        let is_menu = window.flags.contains(&WindowFlags::ChildMenu);
        // Vector2D size_min = style.window_min_size;
        let size_min = style.window_min_size;
        if is_popup || is_menu { // Popups and menus bypass style.WindowMinSize by default, but we give then a non-zero minimum size to facilitate understanding problematic cases (e.g. empty popups)
            size_min = Vector2D::min(size_min, &Vector2D::new(4.0, 4.0));
        }

        // FIXME-VIEWPORT-WORKAREA: May want to use GetWorkSize() instead of size depending on the type of windows?
        // Vector2D avail_size = window.viewport.size;
        let mut avail_size= window.viewport.size;
        if window.viewport_owned {
            avail_size = Vector2D::new(f32::MAX, f32::MAX);
        }
        // const int monitor_idx = window.ViewportAllowPlatformMonitorExtend;
        let monitor_idx = window.viewport_allow_platform_monitor_extend;
        if monitor_idx >= 0 && monitor_idx < g.platform_io.monitors.size {
            avail_size = g.platform_io.monitors[monitor_idx].work_size;
        }
        // Vector2D size_auto_fit = ImClamp(size_desired, size_min, ImMax(size_min, avail_size - style.DisplaySafeAreaPadding * 2.0));
        let mut size_auto_fit = Vector2D::clamp(size_desired, size_min, Vector2D::max(size_min, avail_size - style.display_area_safe_padding * 2.0));

        // When the window cannot fit all contents (either because of constraints, either because screen is too small),
        // we are growing the size on the other axis to compensate for expected scrollbar. FIXME: Might turn bigger than ViewportSize-window_padding.
        // Vector2D size_auto_fit_after_constraint = CalcWindowSizeAfterConstraint(window, size_auto_fit);
        let size_auto_fit_after_constraint = calc_window_size_after_constraint(g, window, &size_auto_fit);

        // bool will_have_scrollbar_x = (size_auto_fit_after_constraint.x - size_pad.x - 0.0                 < size_contents.x && !(window.flags & WindowFlags::NoScrollbar) && (window.flags & WindowFlags::HorizontalScrollbar)) || (window.flags & WindowFlags::AlwaysHorizontalScrollbar);
        let will_have_scrollbar_x = (&size_auto_fit_after_constraint.x - &size_pad.x - 0.0) < size_contents.x ** !(window.flags.contains(&WindowFlags::NoScrollbar)) && window.flags.contains(&WindowFlags::HorizontalScrollbar) || window.flags.contains(&WindowFlags::AlwaysHorizontalScrollbar);

        // bool will_have_scrollbar_y = (size_auto_fit_after_constraint.y - size_pad.y - decoration_up_height < size_contents.y && !(window.flags & WindowFlags::NoScrollbar)) || (window.flags & WindowFlags::AlwaysVerticalScrollbar);
        let will_have_scrollbar_y = (&size_auto_fit_after_constraint.y - &size_pad.y - decoration_up_height) < size_contents.y && !window.flags.contains(&WindowFlags::NoScrollbar) || window.flags.contains(&WindowFlags::AlwaysVerticalScrollbar);

        if will_have_scrollbar_x {
            size_auto_fit.y += style.scrollbar_size;
        }
        if will_have_scrollbar_y {
            size_auto_fit.x += style.scrollbar_size;
        }
        return size_auto_fit;
    }
}

// Vector2D ImGui::CalcWindowNextAutoFitSize(ImGuiWindow* window)
pub fn calc_window_next_auto_fit_size(g: &mut Context, window: &mut Window) -> Vector2D
{
    // Vector2D size_contents_current;
    let mut size_contents_current = Vector2D::default();
    // Vector2D size_contents_ideal;
    let mut size_contents_ideal = Vector2D::default();
    // CalcWindowContentSizes(window, &size_contents_current, &size_contents_ideal);
    calc_window_content_sizes(g, window, &mut size_contents_current, &mut size_contents_ideal);
    // Vector2D size_auto_fit = CalcWindowAutoFitSize(window, size_contents_ideal);
    let size_auto_fit = calc_window_auto_fit_size(g, window, &mut size_contents_ideal);
    // Vector2D size_final = CalcWindowSizeAfterConstraint(window, size_auto_fit);
    let size_final = calc_window_size_after_constraint(g, window, &size_auto_fit);
    return size_final;
}

// static void CalcResizePosSizeFromAnyCorner(ImGuiWindow* window, const Vector2D& corner_target, const Vector2D& corner_norm, Vector2D* out_pos, Vector2D* out_size)
pub fn calc_resize_pos_size_from_any_corner(g: &mut Context, window: &mut Window, corner_target: &Vector2D, corner_norm: &Vector2D, out_pos: &mut Vector2D, out_size: &mut Vector2D)
{
    // Vector2D pos_min = ImLerp(corner_target, window.pos, corner_norm);                // Expected window upper-left
    let pos_min = Vector2D::lerp2(corner_target, &window.pos, &corner_norm);
    // Vector2D pos_max = ImLerp(window.pos + window.size, corner_target, corner_norm); // Expected window lower-right
    let pos_max = Vector2D::lerp2(&window.pos + &window.size, corner_target, &corner_norm);
    // Vector2D size_expected = pos_max - pos_min;
    let size_expected = pos_max - pos_min;
    // Vector2D size_constrained = CalcWindowSizeAfterConstraint(window, size_expected);
    let size_constrained = calc_window_size_after_constraint(g, window, &size_expected);
    // *out_pos = pos_min;
    *out_pos = pos_min.clone();
    if corner_norm.x == 0.0 {
        out_pos.x -= (size_constrained.x - size_expected.x);
    }
    if corner_norm.y == 0.0 {
        out_pos.y -= (size_constrained.y - size_expected.y);
    }
    *out_size = size_constrained.clone();
}

// 0..3: corners (Lower-right, Lower-left, Unused, Unused)
// ImGuiID ImGui::GetWindowResizeCornerID(ImGuiWindow* window, int n)
pub fn get_window_resize_corner_id(g: &mut Context, window: &mut Window, n: i32) -> Id32
{
    // IM_ASSERT(n >= 0 && n < 4);
    // ImGuiID id = window.dock_is_active ? window.DockNode.HostWindow.ID : window.id;
    let mut id = if window.dock_is_active { window.dock_node_id.host_window_id } else { window.id};
    // id = ImHashStr("#RESIZE", 0, id);
    // id = hash_string("#RESIZE", 0, id);
    // // id = ImHashData(&n, sizeof, id);
    // id = hash_data(&n, )
    return id;
}

// float ImGui::CalcWrapWidthForPos(const Vector2D& pos, float wrap_pos_x)
pub fn calc_wrap_width_for_pos(
    g: &mut Context,
    pos: &Vector2D,
    mut wrap_pos_x: f32,
) -> Result<f32, &'static str> {
    if wrap_pos_x < 0.0 {
        return Ok(0.0);
    }

    // ImGuiContext& g = *GImGui;
    // ImGuiWindow* window = g.CurrentWindow;
    let window = g.get_current_window()?;
    if wrap_pos_x == 0.0 {
        // We could decide to setup a default wrapping max point for auto-resizing windows,
        // or have auto-wrap (with unspecified wrapping pos) behave as a content_size extending function?
        //if (window->hidden && (window->flags & ImGuiWindowFlags_AlwaysAutoResize))
        //    wrap_pos_x = ImMax(window->work_rect.min.x + g.font_size * 10.0, window->work_rect.max.x);
        //else
        wrap_pos_x = window.work_rect.max.x;
    } else if wrap_pos_x > 0.0 {
        wrap_pos_x += window.pos.x - window.scroll.x; // wrap_pos_x is provided is window local space
    }

    let out = f32::max(wrap_pos_x - pos.x, 1.0);
    Ok(out)
}

// static Vector2D CalcWindowSizeAfterConstraint(ImGuiWindow* window, const Vector2D& size_desired)
pub fn calc_window_size_after_constraint(
    g: &mut Context,
    window: &mut Window,
    size_desired: &Vector2D,
) -> Vector2D {
    // ImGuiContext& g = *GImGui;
    // Vector2D new_size = size_desired;
    let mut new_size = size_desired.clone();
    if g.next_window_data.flags & NextWindowDataFlags::HasSizeConstraint {
        // Using -1,-1 on either x/Y axis to preserve the current size.
        // ImRect cr = g.next_window_data.sizeConstraintRect;
        let cr = g.next_window_data.size_constraint_rect;
        new_size.x = if cr.min.x >= 0 && cr.max.x >= 0 {
            f32::clamp(new_size.x, cr.min.x, cr.max.x)
        } else {
            window.size_full.x
        };
        new_size.y = if cr.min.y >= 0 && cr.max.y >= 0 {
            f32::clamp(new_size.y, cr.min.y, cr.max.y)
        } else {
            window.size_full.y
        };
        if g.next_window_data.sizeCallback {
            // ImGuiSizeCallbackData data;
            let mut data = SizeCallbackData::default();
            data.user_data = g.next_window_data.size_callback_user_data;
            data.pos = window.pos.clone();
            data.CurrentSize = window.size_full.clone();
            data.desired_size = new_size;
            g.next_window_data.sizeCallback(&data);
            new_size = data.desired_size;
        }
        new_size.x = f32::floor(new_size.x);
        new_size.y = f32::floor(new_size.y);
    }

    // Minimum size
    // if (!(window.flags.contains & (WindowFlags::ChildWindow | WindowFlags::AlwaysAutoResize)))
    if !window.flags.contains(&WindowFlags::ChildWindow)
        && !window.flags.contains(&WindowFlags::AlwaysAutoResize)
    {
        // ImGuiWindow* window_for_height = GetWindowForTitleAndMenuHeight(window);
        let window_for_height = get::get_window_for_title_and_menu_height(g, window);
        // const float decoration_up_height = window_for_height->TitleBarHeight() + window_for_height->MenuBarHeight();
        let decoration_up_height =
            window_for_height.title_bar_height() + window_for_height.menu_bar_height();
        new_size = Vector2D::max(new_size, g.style.window_min_size);
        new_size.y = f32::max(
            new_size.y,
            decoration_up_height + ImMax(0.0, g.style.WindowRounding - 1.0),
        ); // Reduce artifacts with very small windows
    }
    return new_size;
}

// static void CalcWindowContentSizes(ImGuiWindow* window, Vector2D* content_size_current, Vector2D* content_size_ideal)
pub fn calc_window_content_sizes(
    g: &mut Context,
    window: &mut Window,
    content_size_current: &mut Vector2D,
    content_size_ideal: &mut Vector2D,
) {
    // bool preserve_old_content_sizes = false;
    let mut preserve_old_content_sizes = false;
    if window.collapsed && window.auto_fit_frames_x <= 0 && window.auto_fit_frames_y <= 0 {
        preserve_old_content_sizes = true;
    } else if window.hidden
        && window.hidden_frames_cannot_skip_items == 0
        && window.hidden_frames_can_skip_items > 0
    {
        preserve_old_content_sizes = true;
    }
    if preserve_old_content_sizes {
        *content_size_current = window.ContentSize;
        *content_size_ideal = window.ContentSizeIdeal;
        return;
    }

    content_size_current.x = if window.content_size_explicit.x != 0.0 {
        window.content_size_explicit.x
    } else {
        f32::floor(window.dc.cursor_max_pos.x - window.dc.cursor_start_pos.x)
    };
    content_size_current.y = if window.content_size_explicit.y != 0.0 {
        window.content_size_explicit.y
    } else {
        f32::floor(window.dc.cursor_max_pos.y - window.dc.cursor_start_pos.y)
    };
    content_size_ideal.x = if window.content_size_explicit.x != 0.0 {
        window.content_size_explicit.x
    } else {
        f32::floor(
            ImMax(window.dc.cursor_max_pos.x, window.dc.ideal_max_pos.x)
                - window.dc.cursor_start_pos.x,
        )
    };
    content_size_ideal.y = if window.content_size_explicit.y != 0.0 {
        window.content_size_explicit.y
    } else {
        f32::floor(
            ImMax(window.dc.cursor_max_pos.y, window.dc.ideal_max_pos.y)
                - window.dc.cursor_start_pos.y,
        )
    };
}

// static inline void ClampWindowRect(ImGuiWindow* window, const Rect& visibility_rect)
pub fn clamp_window_rect(g: &mut Context, window: &mut Window, visibility_rect: &Rect) {
    // ImGuiContext& g = *GImGui;
    // Vector2D size_for_clamping = window.size;
    let mut size_for_clamping = window.size.clone();
    // if g.io.config_windows_move_from_title_bar_only && (!(window.flags & WindowFlags::NoTitleBar) || window.DockNodeAsHost)
    if g.io.config_windows_move_from_title_bar_only
        && !window.flags.contains(&WindowFlags::NoTitleBar)
        || window.dock_node_as_host_id.id != INVALID_ID
    {
        // size_for_clamping.y = ImGui::GetFrameHeight();
        size_for_clamping.y = get_frame_height()
    } // Not using window->TitleBarHeight() as dock_node_as_host will report 0.0 here.
      // window.pos = ImClamp(window.pos, visibility_rect.min - size_for_clamping, visibility_rect.max);
    window.pos = Vector2D::clamp(
        &window.pos,
        &visibility_rect.min - size_for_clamping,
        &visibility_rect.max,
    );
}

/// static void ScaleWindow(ImGuiWindow* window, float scale)
pub fn scale_window(window: &mut Window, scale: f32)
{
    // Vector2D origin = window.viewport.pos;
    let origin = window.viewport_id.pos;
    window.pos = Vector2D::floor((&window.pos - origin) * scale + origin);
    window.size = Vector2D::floor(&window.size * scale);
    window.size_full = Vector2D::floor(&window.size_full * scale);
    window.content_size = Vector2D::floor(window.ContentSize * scale);
}

// void ImGui::set_window_size(ImGuiWindow* window, const Vector2D& size, ImGuiCond cond)
pub fn set_window_size(g: &mut Context, window: &mut Window, size: &Vector2D, condition: Condition)
{
    // Test condition (NB: bit 0 is always true) and clear flags for next time
    if (cond && (window.set_window_size_allow_flags & cond) == 0)
        return;

    // IM_ASSERT(cond == 0 || ImIsPowerOfTwo(cond)); // Make sure the user doesn't attempt to combine multiple condition flags.
    window.set_window_size_allow_flags &= ~(ImGuiCond_Once | Cond::FirstUseEver | ImGuiCond_Appearing);

    // Set
    Vector2D old_size = window.size_full;
    window.auto_fit_frames_x = (size.x <= 0.0) ? 2 : 0;
    window.auto_fit_frames_y = (size.y <= 0.0) ? 2 : 0;
    if (size.x <= 0.0)
        window.auto_fit_only_grows = false;
    else
        window.size_full.x = f32::floor(size.x);
    if (size.y <= 0.0)
        window.auto_fit_only_grows = false;
    else
        window.size_full.y = f32::floor(size.y);
    if (old_size.x != window.size_full.x || old_size.y != window.size_full.y)
        mark_ini_settings_dirty(window);
}

// void ImGui::set_window_size(const Vector2D& size, ImGuiCond cond)
pub fn set_window_size2(g: &mut Context, size: &Vector2D, cond: Condition)
{
    set_window_size(g.current_window_id, size, cond);
}

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
    // push_id("#RESIZE");
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

            // Vector2D corner_target = g.io.mouse_pos - g.active_id_click_offset + ImLerp(def.inner_dir * grip_hover_outer_size, def.inner_dir * -grip_hover_inner_size, def.CornerPosN); // Corner of the window corresponding to our corner grip
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
        let axis = if border_n == Direction::Left || border_n == Direction::Right { Axis::X } else { Axis::Y};

        // bool hovered, held;
        let mut hovered = false;
        let mut held = false;
        // Rect border_rect = GetResizeBorderRect(window, border_n, grip_hover_inner_size, WINDOWS_HOVER_PADDING);
        let mut border_rect = get_resize_border_rect(window, border_n, grip_hover_inner_size, WINDOWS_HOVER_PADDING);
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
            // border_target[axis] = g.io.mouse_pos[axis] - g.active_id_click_offset[axis] + WINDOWS_HOVER_PADDING;
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
        mark_ini_settings_dirty(window);
    }
    if pos_target.x != f32::MAX
    {
        window.pos = Vector2D::floor(pos_target);
        mark_ini_settings_dirty(window);
    }

    window.size = window.size_full.clone();
    return ret_auto_fit;
}

// void ImGui::set_window_size(const char* name, const Vector2D& size, ImGuiCond cond)
pub fn set_window_size3(g: &mut Context, name: &str, size: &Vector2D, cond: Condition)
{
    if (ImGuiWindow* window = find_window_by_name(name)) {
        set_window_size(window, size, cond);
    }
}
