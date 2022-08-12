use crate::{Context, INVALID_ID};
use crate::font::font::Font;
use crate::vectors::Vector2D;

// Important: this alone doesn't alter current ImDrawList state. This is called by PushFont/PopFont only.
pub fn set_current_font(g: &mut Context, font: &mut Font)
{
    g.font = font.clone();
    g.font_base_size = f32::max(1.0, g.io.font_global_scale * g.font.font_size * g.font.scale);
    g.font_size = if g.current_window_id != INVALID_ID { g.window_mut(g.current_window_id).calc_font_size() } else { 0.0 };

    let atlas = &mut g.font.container_atlas;
    g.draw_list_shared_data.tex_uv_white_pixel = atlas.tex_uv_white_pixel;
    g.draw_list_shared_data.tex_uv_lines = atlas.tex_uv_lines.clones();
    g.draw_list_shared_data.font = g.font.clone();
    g.draw_list_shared_data.font_size = g.font_size;
}

pub fn push_font(g: &mut Context, mut font: &mut Font)
{
    // ImGuiContext& g = *GImGui;
    if !font {
        font = get_default_font();
    }
    set_current_font(g, font);
    g.font_stack.push_back(font);
    g.draw_list_mut(g.current_window_mut().draw_list_id).push_texture_id(font.container_atlas.tex_id);
}

pub fn pop_font(g: &mut Context)
{
    g.draw_list_mut(g.current_window_mut().draw_list_id).pop_texture_id();
    g.font_stack.pop_back();
    set_current_font(g, if g.font_stack.is_empty() { get_default_font() } else { g.font_stack.back() });
}

// ImFont* GetFont()
pub fn get_font(g: &mut Context) -> &mut Font
{
    return &mut g.font;
}

// float GetFontSize()
pub fn get_font_size(g: &mut Context) -> f32
{
    return g.font_size;
}

// Vector2D GetFontTexUvWhitePixel()
pub fn get_font_tex_uv_white_pixel(g: &mut Context) -> Vector2D
{
    return g.draw_list_shared_data.tex_uv_white_pixel;
}

// void SetWindowFontScale(float scale)
pub fn set_window_font_scale(g: &mut Context, scale: f32)
{
    // IM_ASSERT(scale > 0.0);
    // ImGuiContext& g = *GImGui;
    let window = g.current_window_mut();
    window.font_window_scale = scale;
    g.draw_list_shared_data.font_size = window.calc_font_size();
    g.font_size =  g.draw_list_shared_data.font_size;
}
