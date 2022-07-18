use crate::border::get_resize_border_rect;
use crate::color::StyleColor;
use crate::Context;
use crate::style::{get_color_u32, get_color_u32_no_alpha};
use crate::vectors::two_d::Vector2D;
use crate::window::{Window, WindowFlags};

// static void ImGui::RenderWindowOuterBorders(ImGuiWindow* window)
pub fn render_window_outer_borders(g: &mut Context, window: &mut Window)
{
    // ImGuiContext& g = *GImGui;
    // float rounding = window.WindowRounding;
    let mut rounding = window.window_rounding;
    // float border_size = window.WindowBorderSize;
    let mut border_size = window.window_border_size;
    // if border_size > 0.0 && !(window.flags & WindowFlags::NoBackground)
    if border_size > 0.0 && window.flags.contains(&WindowFlags::NoBackground) == false
    {
        window.draw_list.add_rect(&window.pos, &window.pos + &window.size, get_color_u32(StyleColor::Border, 0.0), rounding, 0, border_size);
    }

    // int border_held = window.ResizeBorderHeld;
    let mut border_held = window.resize_border_held;
    if border_held != -1
    {
        // const ImGuiResizeBorderDef& def = resize_border_def[border_held];
        let def = resize_border_def[border_held];
        // Rect border_r = GetResizeBorderRect(window, border_held, rounding, 0.0);
        let mut border_r = get_resize_border_rect(g, window, border_held as i32, rounding, 0.0);
        window.draw_listpath_arc_to(Vector2D::lerp2(&border_r.min, &border_r.max, def.segment_n1) + Vector2D::new(0.5, 0.5) + def.inner_dir * rounding, rounding, def.outer_angle - f32::PI * 0.25, def.outer_angle);
        window.draw_listpath_arc_to(Vector2D::lerp2(&border_r.min, &border_r.max, def.segment_n2) + Vector2D::new(0.5, 0.5) + def.inner_dir * rounding, rounding, def.outer_angle, def.outer_angle + f32::PI * 0.25);
        window.draw_list.path_stroke(get_color_u32_no_alpha(StyleColor::SeparatorActive), 0, ImMax(2.0, border_size)); // Thicker than usual
    }
    // if (g.style.frame_border_size > 0 && !(window.flags & WindowFlags::NoTitleBar) && !window.dock_is_active)
    if g.style.frame_border_size > 0 && window.flags.contains(&WindowFlags::NoTitleBar) == false && window.dock_is_active ==  false
    {
        let mut y = window.pos.y + window.title_bar_height() - 1;
        window.draw_list.add_line(Vector2D::new(window.pos.x + border_size, y), Vector2D::new(window.pos.x + window.size.x - border_size, y), get_color_u32(StyleColor::Border, 0.0), g.style.frame_border_size);
    }
}
