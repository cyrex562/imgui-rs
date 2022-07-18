use crate::Context;
use crate::types::Id32;
use crate::vectors::two_d::Vector2D;
use crate::window::{calc_window_content_sizes, calc_window_size_after_constraint, Window, WindowFlags};

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
    let mut id = if window.dock_is_active { window.dock_node.host_window } else { window.id};
    // id = ImHashStr("#RESIZE", 0, id);
    // id = hash_string("#RESIZE", 0, id);
    // // id = ImHashData(&n, sizeof, id);
    // id = hash_data(&n, )
    return id;
}
