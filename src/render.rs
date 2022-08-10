use std::collections::HashSet;
use std::os::raw::c_char;
use std::ptr::null_mut;
use crate::{call_context_hooks, Context, ContextHookType, INVALID_ID, window};
use crate::color::{Color, COLOR32_A_MASK, alpha_blend_colors, IM_COL32_A_SHIFT, IM_COL32_BLACK, IM_COL32_WHITE, make_color_32, StyleColor};
use crate::draw::data::add_root_window_to_draw_data;
use crate::draw::DrawList;
use crate::draw::flags::{draw_flags_contains_round_corners, DRAW_FLAGS_EMPTY, DrawFlags, set_draw_flags_round_corners_default};
use crate::draw::list::{add_draw_list_to_draw_data, get_background_draw_list, foreground_draw_list};
use crate::frame::end_frame;
use crate::imgui_globals::GImGui;
use crate::imgui_h::Color;
use crate::imgui_style::{get_color_u32, GetColorU32_2, GetColorU32_3};
use crate::input::MouseCursor;
use crate::nav::NavHighlightFlags;
use crate::popup::get_top_most_and_visible_popup_modal;
use crate::style::{color_u32_from_style_color, color_u32_from_style_color_with_alpha};
use crate::text::{calc_text_size, text_count_utf8_bytes_from_char};
use crate::types::{Direction, Id32};
use crate::vectors::vector_2d::Vector2D;
use crate::viewport::setup_viewport_draw_data;
use crate::window::{get, Window, WindowFlags};
use crate::window::checks::{is_window_above, is_window_active_and_visible};
use crate::window::get::find_bottom_most_visible_window_with_begin_stack;
use crate::rect::Rect;
use crate::vectors::Vector4D;

// const char* ImGui::find_rendered_text_end(const char* text, const char* text_end)
// pub unsafe fn find_rendered_text_end(text: *const c_char, text_end: *const c_char) -> *const c_char {
//     // const char* text_display_end = text;
//     let mut text_display_end = text;
//     // if text_end.is_null() {
//     //     text_end = -1;
//     // }
//
//     while text_display_end < text_end && *text_display_end != '\0' as c_char && (text_display_end[0] != '#' || text_display_end[1] != '#') {
//         text_display_end += 1;
//     }
//     return text_display_end;
// }

// Internal ImGui functions to render text
pub fn render_text(
    g: &mut Context,
    pos: &Vector2D,
    text: &str,
    hide_text_after_hash: bool) {
    // ImGuiContext& g = *GImGui;
    // let g = GImGui;
    // Window* window = g.current_window;
    let window = g.current_window_mut();

    // Hide anything after a '##' string
    // const char* text_display_end;
    // let mut text_display_end: &str = null_mut();
    // if hide_text_after_hash
    // {
    //     text_display_end = find_rendered_text_end(text, text_end);
    // }
    // else
    // {
    //     if !text_end {
    //         text_end = text + strlen(text);
    //     }// FIXME-OPT
    //     text_display_end = text_end;
    // }

    // if text != text_display_end
    // {
    //     window.draw_list.AddText2(&g.font, g.font_size, pos, GetColorU32_3(Color::Text as u32), text, text_display_end, 0.0, None);
    //     if g.log_enabled {
    //         LogRenderedText(&pos, text, text_display_end);
    //     }
    // }
    let draw_list = g.draw_list_mut(window.draw_list_id);
    draw_list.add_text_2(Some(&g.font), g.font_size, pos, color_u32_from_style_color(g, StyleColor::Text), &String::from(text), 0.0, None);
    // window.draw_list.AddText2(&g.font, g.font_size, pos, GetColorU32_3(Color::Text as u32), text, text_display_end, 0.0, None);
    //     if g.log_enabled {
    //         LogRenderedText(&pos, text, text_display_end);
    //     }
}

// void ImGui::RenderTextWrapped(Vector2D pos, const char* text, const char* text_end, float wrap_width)
pub fn render_text_wrapped(
    g: &mut Context,
    pos: &Vector2D,
    text: &str,
    wrap_width: f32) {
    // ImGuiContext& g = *GImGui;
    // let g = GImGui;
    // Window* window = g.current_window;
    // let window = g.current_window;
    let window = g.current_window_mut();

    // if !text_end {
    //     text_end = text + strlen(text);
    // } // FIXME-OPT

    // if text != text_end
    // {
    //     window.draw_list.AddText2(&g.font, g.font_size, pos, GetColorU32_3(StyleColor::Text), text, text_end, wrap_width);
    //     if g.log_enabled {
    //         LogRenderedText(&pos, text, text_end);
    //     }
    // }
    let draw_list = g.draw_list_mut(window.draw_list_id);
    draw_list.add_text_2(Some(&g.font), g.font_size, pos, color_u32_from_style_color(g, StyleColor::Text), &String::from(text), wrap_width, None);
}

// Default clip_rect uses (pos_min,pos_max)
// Handle clipping on CPU immediately (vs typically let the GPU clip the triangles that are overlapping the clipping rectangle edges)
pub fn render_text_clipped_ex(
    g: &mut context,
    draw_list: &mut DrawList,
    pos_min: &Vector2D,
    pos_max: &Vector2D,
    text: &str,
    text_size_if_known: &Vector2D,
    align: &Vector2D,
    clip_rect: &Rect) {
    // Perform CPU side clipping for single clipped element to avoid using scissor state
    let mut pos = pos_min.clone();
    let text_size = if text_size_if_known != Vector2D::default() { text_size_if_known } else { calc_text_size(g, text, false, 0.0) };

    let clip_min = if clip_rect { &clip_rect.min } else { &pos_min };
    let clip_max = if clip_rect { &clip_rect.max } else { &pos_max };
    let mut need_clipping = (pos.x + text_size.x >= clip_max.x) || (pos.y + text_size.y >= clip_max.y);
    // If we had no explicit clipping rectangle then pos==clip_min
    if clip_rect != Rect::default() {
        need_clipping |= (pos.x < clip_min.x) || (pos.y < clip_min.y);
    }

    // Align whole block. We should defer that to the better rendering function when we'll have support for individual line alignment.
    if align.x > 0.0 { pos.x = f32::max(pos.x, pos.x + (pos_max.x - pos.x - text_size.x) * align.x); }
    if align.y > 0.0 { pos.y = f32::max(pos.y, pos.y + (pos_max.y - pos.y - text_size.y) * align.y); }

    // Render
    if need_clipping {
        let fine_clip_rect = Vector4D::new(clip_min.x, clip_min.y, clip_max.x, clip_max.y);
        draw_list.add_text_2(None, 0.0, &pos, color_u32_from_style_color(g, StyleColor::Text), &String::from(text), 0.0, Some(&fine_clip_rect));
    } else {
        draw_list.add_text_2(None, 0.0, &pos, color_u32_from_style_color(g, StyleColor::Text), &String::from(text), 0.0, None);
    }
}

pub fn render_text_clipped(
    g: &mut Context,
    pos_min: &Vector2D,
    pos_max: &Vector2D,
    text: &str,
    text_size_if_known: &Vector2D,
    align: &Vector2D,
    clip_rect: &Rect,
) {
    // Hide anything after a '##' string
    // const char* text_display_end = find_rendered_text_end(text, text_end);
    // let text_len = (text_display_end - text);
    // if (text_len == 0)
    //     return;

    // ImGuiContext& g = *GImGui;
    // Window* window = g.current_window;
    let window = g.current_window_mut();
    render_text_clipped_ex(g, g.draw_list_mut(window.draw_list_id), pos_min, pos_max, text, text_size_if_known, align, clip_rect);
    // if (g.log_enabled)
    //     LogRenderedText(&pos_min, text, text_display_end);
}


// Another overly complex function until we reorganize everything into a nice all-in-one helper.
// This is made more complex because we have dissociated the layout rectangle (pos_min..pos_max) which define _where_ the ellipsis is, from actual clipping of text and limit of the ellipsis display.
// This is because in the context of tabs we selectively hide part of the text when the Close Button appears, but we don't want the ellipsis to move.
// void ImGui::render_textEllipsis(ImDrawList* draw_list, const Vector2D& pos_min, const Vector2D& pos_max, float clip_max_x, float ellipsis_max_x, const char* text, const char* text_end_full, const Vector2D* text_size_if_known)
pub fn render_text_ellipsis(
    g: &mut Context,
    draw_list: &mut DrawList,
    pos_min: &Vector2D,
    pos_max: &Vector2D,
    clip_max_x: f32,
    ellipsis_max_x: f32,
    text: &str,
    text_size_if_known: &Vector2D,
) {
    // ImGuiContext& g = *GImGui;
    // if (text_end_full == None) {
    //     text_end_full = find_rendered_text_end(text);
    // }
    // const Vector2D text_size = text_size_if_known ? *text_size_if_known : calc_text_size(text, text_end_full, false, 0.0);
    let text_size = if text_size_if_known != Vector2D::default() {
        text_size_if_known.clone()
    } else {
        calc_text_size(g, text, false, 0.0);
    };

    //draw_list->add_line(Vector2D(pos_max.x, pos_min.y - 4), Vector2D(pos_max.x, pos_max.y + 4), IM_COL32(0, 0, 255, 255));
    //draw_list->add_line(Vector2D(ellipsis_max_x, pos_min.y-2), Vector2D(ellipsis_max_x, pos_max.y+2), IM_COL32(0, 255, 0, 255));
    //draw_list->add_line(Vector2D(clip_max_x, pos_min.y), Vector2D(clip_max_x, pos_max.y), IM_COL32(255, 0, 0, 255));
    // FIXME: We could technically remove (last_glyph->advance_x - last_glyph->x1) from text_size.x here and save a few pixels.
    if text_size.x > pos_max.x - pos_min.x {
        // Hello wo...
        // |       |   |
        // min   max   ellipsis_max
        //          <-> this is generally some padding value

        let font = draw_list.data.font;
        let font_size = draw_list.data.font_size;
        // const char* text_end_ellipsis = None;

        let mut ellipsis_char = font.ellipsis_char;
        let mut ellipsis_char_count = 1;
        if ellipsis_char == -1 {
            ellipsis_char = font.dot_char;
            ellipsis_char_count = 3;
        }
        let glyph = font.find_glyph(ellipsis_char);

        let mut ellipsis_glyph_width = glyph.x1;                 // width of the glyph with no padding on either side
        let mut ellipsis_total_width = ellipsis_glyph_width;      // Full width of entire ellipsis

        if ellipsis_char_count > 1 {
            // Full ellipsis size without free spacing after it.
            let spacing_between_dots = 1.0 * (draw_list.data.font_size / font.font_size);
            ellipsis_glyph_width = glyph.x1 - glyph.X0 + spacing_between_dots;
            ellipsis_total_width = ellipsis_glyph_width * ellipsis_char_count - spacing_between_dots;
        }

        // We can now claim the space between pos_max.x and ellipsis_max.x
        let text_avail_width = f32::max((f32::max(pos_max.x, ellipsis_max_x) - ellipsis_total_width) - pos_min.x, 1.0);
        let mut text_size_clipped_x = font.calc_text_size_a(font_size, text_avail_width, 0.0, text).x;
        if text == text_end_ellipsis && text_end_ellipsis < text_end_full {
            // Always display at least 1 character if there's no room for character + ellipsis
            text_end_ellipsis = text + text_count_utf8_bytes_from_char(text);
            text_size_clipped_x = font.calc_text_size_a(font_size, f32::MAX, 0.0, text).x;
        }
        while text_end_ellipsis > text && char_is_blank_a(text_end_ellipsis[-1]) {
            // Trim trailing space before ellipsis (FIXME: Supporting non-ascii blanks would be nice, for this we need a function to backtrack in UTF-8 text)
            text_end_ellipsis -= 1;
            text_size_clipped_x -= font.calc_text_size_a(font_size, f32::MAX, 0.0, text_end_ellipsis).x; // Ascii blanks are always 1 byte
        }

        // Render text, render ellipsis
        render_text_clipped_ex(g, draw_list, pos_min, &Vector2D::new(clip_max_x, pos_max.y), text, text_end_ellipsis, &text_size, &Rect::new(&Vector2D::new(0f32, 0f32), &Vector2D::new(0f32, 0f32)));
        let mut ellipsis_x = pos_min.x + text_size_clipped_x;
        if ellipsis_x + ellipsis_total_width <= ellipsis_max_x {
            // for (int i = 0; i < ellipsis_char_count; i += 1)
            for i in 0..ellipsis_char_count {
                font.render_char(draw_list, font_size, &Vector2D::new(ellipsis_x, pos_min.y), color_u32_from_style_color(g, StyleColor::Text), ellipsis_char);
                ellipsis_x += ellipsis_glyph_width;
            }
        }
    } else {
        render_text_clipped_ex(g, draw_list, pos_min, &Vector2D::new(clip_max_x, pos_max.y), text, text_end_full, &text_size, &Rect::new(&Vector2D::new(0f32, 0f32), &Vector2D::new(0f32, 0f32)));
    }

    // if (g.log_enabled) {
    //     LogRenderedText(&pos_min, text, text_end_full);
    // }
}

// Render a rectangle shaped with optional rounding and borders
// void ImGui::RenderFrame(Vector2D p_min, Vector2D p_max, ImU32 fill_col, bool border, float rounding)
pub fn render_frame(g: &mut Context, p_min: &Vector2D, p_max: &Vector2D, fill_col: u32, border: bool, rounding: f32) {
    // ImGuiContext& g = *GImGui;
    let window = g.current_window_mut();
    window.draw_list.add_rect_filled(p_min, p_max, fill_col, rounding);
    let border_size = g.style.frame_border_size;
    if border && border_size > 0.0 {
        window.draw_list.add_rect(p_min + Vector2D::new(1f32, 1f32), p_max + Vector2D::new(1f32, 1f32), color_u32_from_style_color(g, StyleColor::BorderShadow), rounding, 0, border_size);
        window.draw_list.add_rect(p_min, p_max, color_u32_from_style_color(g, StyleColor::Border), rounding, 0, border_size);
    }
}

// void ImGui::RenderFrameBorder(Vector2D p_min, Vector2D p_max, float rounding)
pub fn render_frame_border(g: &mut Context, p_min: &Vector2D, p_max: &Vector2D, rounding: f32) {
    // ImGuiContext& g = *GImGui;
    let window = g.current_window_mut();
    let border_size = g.style.frame_border_size;
    if border_size > 0.0 {
        window.draw_list.add_rect(p_min + Vector2D::new(1f32, 1f32), p_max + Vector2D::new(1f32, 1f32), color_u32_from_style_color(g, StyleColor::BorderShadow), rounding, 0, border_size);
        window.draw_list.add_rect(p_min, p_max, color_u32_from_style_color(g, StyleColor::Border), rounding, 0, border_size);
    }
}

// void ImGui::render_nav_highlight(const Rect& bb, Id32 id, ImGuiNavHighlightFlags flags)
pub fn render_nav_highlight(
    g: &mut Context,
    bb: &Rect,
    id: Id32,
    flags: &HashSet<NavHighlightFlags>) {
    // ImGuiContext& g = *GImGui;
    if id != g.nav_id {
        return;
    }
    if g.nav_disable_highlight && !flags.contains(&NavHighlightFlags::AlwaysDraw) {
        return;
    }
    let window = g.current_window_mut();
    if window.dc.nav_hide_highlight_one_frame {
        return;
    }

    let rounding = if flags.contains(&NavHighlightFlags::NoRounding) { 0.0 } else { g.style.frame_rounding };
    let mut display_rect = bb.clone();
    display_rect.clip_width(&window.clip_rect);
    if flags.contains(&NavHighlightFlags::TypeDefault) {
        let THICKNESS = 2.0;
        let DISTANCE = 3.0 + THICKNESS * 0.5;
        display_rect.expand_vector(&Vector2D::new(DISTANCE, DISTANCE));
        let fully_visible = window.clip_rect.contains(display_rect);
        if !fully_visible {
            g.draw_list_mut(window.draw_list_id).push_clip_rect(&display_rect.min, &display_rect.max, false);
        }
        let draw_flags: HashSet<DrawFlags> = HashSet::new();
        g.draw_list_mut(window.draw_list_id).add_rect(&display_rect.min + Vector2D::new(THICKNESS * 0.5, THICKNESS * 0.5), &display_rect.max - Vector2D::new(THICKNESS * 0.5, THICKNESS * 0.5), color_u32_from_style_color(g, StyleColor::NavHighlight), rounding, &draw_flags, THICKNESS);
        if !fully_visible {
            g.draw_list_mut(window.draw_list_id).pop_clip_rect();
        }
    }
    if flags.contains(&NavHighlightingFlags::TypeThin) {
        let draw_flags: HashSet<DrawFlags> = HashSet::new();
        g.draw_list_mut(window.draw_list_id).add_rect(&display_rect.min, &display_rect.max, color_u32_from_style_color(g, StyleColor::NavHighlight), rounding, &draw_flags, 1.0);
    }
}

// void ImGui::render_mouse_cursor(Vector2D base_pos, float base_scale, ImGuiMouseCursor mouse_cursor, ImU32 col_fill, ImU32 col_border, ImU32 col_shadow)
pub fn render_mouse_cursor(
    g: &mut Context,
    base_pos: &Vector2D,
    base_scale: f32,
    mouse_cursor: MouseCursor,
    col_fill: u32,
    col_border: u32,
    col_shadow: u32) {
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(mouse_cursor > MouseCursor::None && mouse_cursor < ImGuiMouseCursor_COUNT);
    let mut font_atlas = g.draw_list_shared_data.font.container_atlas.unwrap();
    // for (int n = 0; n < g.viewports.size; n += 1)
    for n in 0..g.viewports.len() {
        // We scale cursor with current viewport/monitor, however windows 10 for its own hardware cursor seems to be using a different scale factor.
        // Vector2D offset, size, uv[4];
        let mut offset = Vector2D::default();
        let mut size = Vector2D::default();
        let mut uv: [Vector2D; 4] = [Vector2D::default(); 4];
        if !font_atlas.get_mouse_cursor_tex_data(&mouse_cursor, &mut offset, &mut size, &mut uv[0..1], &mut uv[2..3]) {
            continue;
        }
        let viewport = &mut g.viewports[n];
        let pos = base_pos - offset;
        let scale = base_scale * viewport.dpi_scale;
        if !viewport.get_main_rect().Overlaps(Rect::new(pos, pos + Vector2D::new(size.x + 2, size.y + 2) * scale)) {
            continue;
        }
        let draw_list = get_foreground_draw_list(viewport);
        let tex_id = font_atlas.tex_id;
        draw_list.push_texture_id(tex_id);
        draw_list.add_image(tex_id, pos + Vector2D::new(1f32, 0f32) * scale, pos + (Vector2D::new(1f32, 0f32) + size) * scale, uv[2], uv[3], col_shadow);
        draw_list.add_image(tex_id, pos + Vector2D::new(2f32, 0f32) * scale, pos + (Vector2D::new(2f32, 0f32) + size) * scale, uv[2], uv[3], col_shadow);
        draw_list.add_image(tex_id, pos, pos + size * scale, uv[2], uv[3], col_border);
        draw_list.add_image(tex_id, pos, pos + size * scale, uv[0], uv[1], col_fill);
        draw_list.pop_texture_id();
    }
}

// static void ImGuis::RenderDimmedBackgrounds()
pub fn render_dimmed_backgrounds(g: &mut Context) {
    // ImGuiContext& g = *GImGui;
    // Window* modal_window = GetTopMostAndVisiblePopupModal();
    let modal_window: &mut Window = get_top_most_and_visible_popup_modal(g).unwrap();
    if g.dim_bg_ratio <= 0.0 && g.nav_windowing_highlight_alpha <= 0.0 {
        return;
    }
    // const bool dim_bg_for_modal = (modal_window != None);

    // const bool dim_bg_for_window_list = (g.NavWindowingTargetAnim != None && g.NavWindowingTargetAnim->Active);
    let dim_bg_for_window_list = (g.nav_windowing_target_anim != INVALID_ID && g.window_mut(g.nav_windowing_target_anim).active);
    let nav_win_tgt_anim = g.window_mut(g.nav_windowing_target_anim).unwrap();
    let dim_bg_for_window_list = g.nav_windowing_target_anim != INVALID_ID && nav_win_tgt_anim.active;
    if !dim_bg_for_modal && !dim_bg_for_window_list {
        return;
    }

    // ImGuiViewport* viewports_already_dimmed[2] = { None, None };
    let viewports_already_dimmedd: [Id32; 2] = [0, 0];
    if dim_bg_for_modal {
        // Draw dimming behind modal or a begin stack child, whichever comes first in draw order.
        // Window* dim_behind_window = FindBottomMostVisibleWindowWithinBeginStack(modal_window);
        let dim_behind_window = find_bottom_most_visible_window_with_begin_stack(g, modal_window);
        // RenderDimmedBackgroundBehindWindow(dim_behind_window, get_color_u32(ImGuiCol_ModalWindowDimBg, g.dim_bg_ration));
        render_dimmed_background_behind_window(g, dim_behind_window, color_u32_from_style_color_with_alpha(g, StyleColor::ModalWindowDimBg, g.dim_bg_ratio));
        viewports_already_dimmed[0] = modal_window.viewport_id;
    } else if dim_bg_for_window_list {
        // Draw dimming behind CTRL+Tab target window and behind CTRL+Tab UI window
        let nwta_win = g.window_mut(g.nav_windowing_target_anim).unwrap();
        let nwl_win = g.window_mut(g.nav_windowing_list_window_id).unwrap();
        render_dimmed_background_behind_window(g, g.NavWindowingTargetAnim, color_u32_from_style_color_with_alpha(g, StyleColor::NavWindowingDimBg, g.dim_bg_ratio));
        if g.nav_windowing_list_window_id != INVALID_ID {
            if nwl_win.viewport_id != INVALID_ID {
                if nwl_win.viewport_id != nwta_win.viewport_id {
                    render_dimmed_background_behind_window(g, nwl_win, color_u32_from_style_color_with_alpha(g, StyleColor::NavWindowingDimBg, g.dim_bg_ratio));
                }
            }
        }

        viewports_already_dimmed[0] = nwta_win.viewport_id;
        viewports_already_dimmed[1] = nwl_win.viewport_id;

        // Draw border around CTRL+Tab target window
        // Window * window = g.NavWindowingTargetAnim;
        let window = g.window_mut(g.nav_windowing_target_anim);
        // ImGuiViewport * viewport = window.viewport;
        let nwta_vp = g.viewport_mut(nwta_win.viewport_id).unwrap();
        let distance = g.font_size;
        let bb = nwta_win.rect();
        // float
        // distance = g.FontSize;
        // ImRect
        // bb = window.rect();
        bb.expand(distance);
        if bb.get_width() >= viewport.size.x && bb.get_height() >= viewport.size.y {
            bb.Expand(-distance - 1.0);
        }
        // If a window fits the entire viewport, adjust its highlight inward
        let draw_list = g.draw_list_mut(window.draw_list_id);
        if draw_list.cmd_buffer.is_empty() {
            draw_list.add_draw_cmd();
        }
        let draw_flags: HashSet<DrawFlags> = HashSet::new();
        draw_list.push_clip_rect(viewport.pos, viewport.pos + viewport.size, false);
        draw_list.add_rect(bb.min, bb.max, color_u32_from_style_color_with_alpha(g, StyleColor::NavWindowingHighlight, g.nav_windowing_highlight_alpha), window.window_rounding, &draw_flags, 3.0);
        draw_list.pop_clip_rect();
    }

    // Draw dimming background on _other_ viewports than the ones our windows are in
    // for (int viewport_n = 0; viewport_n < g.viewports.Size; viewport_n += 1)
    for viewport in g.viewports.iter_mut() {
        // ImGuiViewportP* viewport = g.viewports[viewport_n];
        if viewport.id == viewports_already_dimmed[0] || viewport == viewports_already_dimmed[1] {
            continue;
        }
        if modal_window.id != INVALID_ID && viewport.window_id != INVALID_ID && is_window_above(g, g.window_mut(viewport.window_id), modal_window) {
            continue;
        }
        // ImDrawList* draw_list = GetForegroundDrawList(viewport);
        let draw_list = foreground_draw_list(g, viewport);
        let dim_bg_color = color_u32_from_style_color_with_alpha(g, if dim_bg_for_modal { StyleColor::ModalWindowDimBg } else { StyleColor::NavWindowingDimBg }, g.dim_bg_ratio);
        let draw_flags: HashSet<DrawFlags> = HashSet::new();
        draw_list.add_rect_filled(&viewport.pos, &viewport.pos + &viewport.size, dim_bg_color, 0.0, &draw_flags);
    }
}

// Prepare the data for rendering so you can call GetDrawData()
// (As with anything within the ImGui:: namspace this doesn't touch your GPU or graphics API at all:
// it is the role of the ImGui_ImplXXXX_RenderDrawData() function provided by the renderer backend)
// void ImGui::Render()
pub fn render(g: &mut Context) {
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.initialized);

    if g.frame_count_ended != g.frame_count {
        end_frame(g);
    }
    let first_render_of_frame = (g.frame_count_rendered != g.frame_count);
    g.frame_count_rendered = g.frame_count;
    g.io.metrics_render_windows = 0;

    call_context_hooks(g, ContextHookType::RenderPre);

    // Add background ImDrawList (for each active viewport)
    // for (int n = 0; n != g.viewports.Size; n += 1)
    for viewport in g.viewports.iter_mut() {
        // ImGuiViewportP* viewport = g.viewports[n];
        viewport.draw_data_builder.clear();
        if viewport.draw_list_ids[0] != INVALID_ID {
            add_draw_list_to_draw_data(g, &mut viewport.draw_data_builder.layers[0], get_background_draw_list(g, viewport).id);
        }
    }

    // Add ImDrawList to render
    // Window* windows_to_render_top_most[2];
    let mut windows_to_render_top_most: [Id32; 2] = [
        if g.nav_windowing_target_id != INVALID_ID && !g.nav_windowing_target_id.flags.contains(WindowFlags::NoBringToFrontOnFocus) {
            let nwt_win = g.window_mut(g.nav_windowing_target_id).unwrap();
            nwt_win.root_window_dock_tree_id
        } else {
            INVALID_ID
        },
        if g.nav_windowing_target_id != INVALID_ID {
            g.nav_windowing_list_window_id
        } else {
            INVALID_ID
        }
    ];
    // windows_to_render_top_most[0] = (g.nav_windowing_target_id && !(g.nav_windowing_target_id.flags & WindowFlags_NoBringToFrontOnFocus)) ? g.nav_windowing_target_id ->root_window_dock_tree : None;
    // windows_to_render_top_most[1] = (g.nav_windowing_target_id? g.nav_windowing_list_window : None);
    // for (int n = 0; n != g.windows.Size; n += 1)
    for (win_id, window) in g.windows.iter_mut() {
        // Window* window = g.windows[n];
        // IM_MSVC_WARNING_SUPPRESS(6011); // Static Analysis false positive "warning C6011: Dereferencing None pointer 'window'"
        if is_window_active_and_visible(window) && (!window.flags.contains(&WindowFlags::ChildWindow)) && window.id != windows_to_render_top_most[0] && window.id != windows_to_render_top_most[1] {
            add_root_window_to_draw_data(g, window);
        }
    }
    // for (int n = 0; n < IM_ARRAYSIZE(windows_to_render_top_most); n += 1)
    for n in 0..windows_to_render_top_most.len() {
        if windows_to_render_top_most[n] != INVALID_ID && is_window_active_and_visible(g.window_mut(windows_to_render_top_most[n]).unwrap()) { // nav_windowing_target is always temporarily displayed as the top-most window
            add_root_window_to_draw_data(g, g.window_mut(windows_to_render_top_most[n]).unwrap());
        }
    }

    // Draw modal/window whitening backgrounds
    if first_render_of_frame {
        render_dimmed_backgrounds(g);
    }

    // Draw software mouse cursor if requested by io.mouse_draw_cursor flag
    if g.io.mouse_draw_cursor && first_render_of_frame && g.mouse_cursor != MouseCursor::None {
        render_mouse_cursor(g, &g.io.mouse_pos, g.style.mouse_cursor_scale, g.mouse_cursor.clone(), IM_COL32_WHITE, IM_COL32_BLACK, make_color_32(0, 0, 0, 48));
    }

    // Setup ImDrawData structures for end-user
    g.io.metrics_render_vertices = 0;
    g.io.metrics_render_indices = 0;
    // for (int n = 0; n < g.viewports.Size; n += 1)
    for viewport in g.viewports.iter_mut() {
        // ImGuiViewportP* viewport = g.viewports[n];
        // viewport->DrawDataBuilder.FlattenIntoSingleLayer();
        viewport.draw_data_builder.flatten_into_single_layer();

        // Add foreground ImDrawList (for each active viewport)
        if viewport.draw_list_ids[1] != INVALID_ID {
            add_draw_list_to_draw_data(g, &mut viewport.draw_data_builder.layers[0], foreground_draw_list(g, viewport).id);
        }
        setup_viewport_draw_data(g, viewport, &viewport.draw_data_builder.layers[0]);
        let draw_data = &viewport.draw_data;
        g.io.metrics_render_vertices += draw_data.total_vtx_count;
        g.io.metrics_render_indices += draw_data.total_idx_count;
    }

    call_context_hooks(g, ImGuiContextHookType_RenderPost);
}

// static void ImGui::RenderDimmedBackgroundBehindWindow(Window* window, ImU32 col)
pub fn render_dimmed_background_behind_window(g: &mut Context, window: &mut Window, color: u32) {
    if (color & COLOR32_A_MASK) == 0 {
        return;
    }

    // ImGuiViewportP* viewport = window.viewport;
    let viewport = g.viewport_mut(window.viewport_id).unwrap();
    // ImRect viewport_rect = viewport->get_main_rect();
    let viewport_rect = viewport.get_main_rect();

    // Draw behind window by moving the draw command at the FRONT of the draw list

    // We've already called AddWindowToDrawData() which called draw_list->ChannelsMerge() on DockNodeHost windows,
    // and draw list have been trimmed already, hence the explicit recreation of a draw command if missing.
    // FIXME: This is creating complication, might be simpler if we could inject a drawlist in drawdata at a given position and not attempt to manipulate ImDrawCmd order.
    // ImDrawList* draw_list = window.root_window_dock_tree->DrawList;
    let root_win_dock_tree_win = g.window_mut(window.root_window_dock_tree_id).unwrap();
    let draw_list = g.draw_list_mut(root_win_dock_tree_win.draw_list_id).unwrap();
    if draw_list.cmd_buffer.len() == 0 {
        draw_list.add_draw_cmd();
    }
    draw_list.push_clip_rect(viewport_rect.min - Vector2D::new(1.0, 1.0), viewport_rect.max + Vector2D::new(1.0, 1.0), false); // Ensure ImDrawCmd are not merged
    draw_list.add_rect_filled(viewport_rect.min, viewport_rect.max, color);
    // ImDrawCmd cmd = draw_list.cmd_buffer.back();
    let cmd = draw_list.cmd_buffer.last().unwrap();
    // IM_ASSERT(cmd.elem_count == 6);
    draw_list.cmd_buffer.pop_back();
    draw_list.cmd_buffer.push_front(cmd);
    draw_list.pop_clip_rect();
    draw_list.add_draw_cmd(); // We need to create a command as cmd_buffer.back().idx_offset won't be correct if we append to same command.


    // Draw over sibling docking nodes in a same docking tree
    let root_win = g.window_mut(window.root_window_id).unwrap();
    if root_win.dock_is_active {
        // ImDrawList* draw_list = FindFrontMostVisibleChildWindow(window.root_window_dock_tree)->DrawList;

        let draw_list = g.draw_list_mut(get::find_front_most_visible_child_window(g, root_win_dock_tree_win).draw_list_id).unwrap();
        if draw_list.cmd_buffer.len() == 0 {
            draw_list.add_draw_cmd();
        }
        draw_list.push_clip_rect(viewport_rect.min, viewport_rect.max, false);
        render_rect_filled_with_hole(draw_list, root_win_dock_tree_win.rect(), root_win.rect(), color, 0.0);// window->root_window_dock_tree->window_rounding);
        draw_list.pop_clip_rect();
    }
}

// Render an arrow aimed to be aligned with text (p_min is a position in the same space text would be positioned). To e.g. denote expanded/collapsed state
pub fn render_arrow(
    g: &mut Context,
    draw_list: &mut DrawList,
    pos: &Vector2D,
    col: u32,
    dir: Direction,
    scale: f32,
) {
    let h = draw_list.data.font_size * 1.00;
    let mut r = h * 0.40 * scale;
    let center = pos + Vector2D::new(h * 0.50, h * 0.50 * scale);

    // Vector2D a, b, c;
    let mut a = Vector2D::default();
    let mut b = Vector2D::default();
    let mut c = Vector2D::default();

    match dir {
        Direction::Up | Direction::Down => {
            if dir == Direction::Up { r = r * -1 };
            a = Vector2D::new(0.000, 0.750) * r;
            b = Vector2D::new(-0.866, -0.750) * r;
            c = Vector2D::new(0.866, -0.750) * r;
        }
        Direction::Left | Direction::Right => {
            if dir == Direction::Left {
                r = -r;
            }
            a = Vector2D::new(0.750, 0.000) * r;
            b = Vector2D::new(-0.750, 0.866) * r;
            c = Vector2D::new(-0.750, -0.866) * r;
        }
        _ => {}
    }

    draw_list.add_triangle_filled(center + a, center + b, center + c, col);
}

// void ImGui::render_bullet(ImDrawList* draw_list, Vector2D pos, ImU32 col)
pub fn render_bullet(g: &mut Context, draw_list: &mut DrawList, pos: &Vector2D, col: u32) {
    draw_list.add_circle_filled(pos, draw_list.data.font_size * 0.20, col, 8);
}

// void ImGui::RenderCheckMark(ImDrawList* draw_list, Vector2D pos, ImU32 col, float sz)
pub fn render_checkmark(draw_list: &mut DrawList, pos: &mut Vector2D, col: u32, mut sz: f32) {
    let thickness = f32::max(sz / 5.0, 1.0);
    sz -= thickness * 0.5;
    *pos += Vector2D::new(thickness * 0.25, thickness * 0.25);

    let third = sz / 3.0;
    let bx = pos.x + third;
    let by = pos.y + sz - third * 0.5;
    draw_list.path_line_to(&Vector2D::new(bx - third, by - third));
    draw_list.path_line_to(&Vector2D::new(bx, by));
    draw_list.path_line_to(&Vector2D::new(bx + third * 2.0, by - third * 2.0));
    let draw_flags: HashSet<DrawFlags> = HashSet::new();
    draw_list.path_stroke(col, &draw_flags, thickness);
}

// Render an arrow. 'pos' is position of the arrow tip. half_sz.x is length from base to tip. half_sz.y is length on each side.
// void ImGui::RenderArrowPointingAt(ImDrawList* draw_list, Vector2D pos, Vector2D half_sz, ImGuiDir direction, ImU32 col)
pub fn render_arrow_pointing_at(draw_list: &mut DrawList, pos: &Vector2D, half_sz: &Vector2D, direction: &Direction, col: u32) {
    match direction {
        Direction::Left => draw_list.add_triangle_filled(&Vector2D::new(pos.x + half_sz.x, pos.y - half_sz.y), &Vector2D::new(pos.x + half_sz.x, pos.y + half_sz.y), pos, col),
        Direction::Right => draw_list.add_triangle_filled(&Vector2D::new(pos.x - half_sz.x, pos.y + half_sz.y), &Vector2D::new(pos.x - half_sz.x, pos.y - half_sz.y), pos, col),
        Direction::Up => draw_list.add_triangle_filled(&Vector2D::new(pos.x + half_sz.x, pos.y + half_sz.y), &Vector2D::new(pos.x - half_sz.x, pos.y + half_sz.y), pos, col),
        Direction::Down => draw_list.add_triangle_filled(&Vector2D::new(pos.x - half_sz.x, pos.y - half_sz.y), &Vector2D::new(pos.x + half_sz.x, pos.y - half_sz.y), pos, col),
        _ => {} // Fix warnings
    }
}

// This is less wide than RenderArrow() and we use in dock nodes instead of the regular RenderArrow() to denote a change of functionality,
// and because the saved space means that the left-most tab label can stay at exactly the same position as the label of a loose window.
// void ImGui::RenderArrowDockMenu(ImDrawList* draw_list, Vector2D p_min, float sz, ImU32 col)
pub fn render_arrow_dock_menu(draw_list: &mut DrawList, p_min: &Vector2D, sz: f32, col: u32) {
    let draw_flags: HashSet<DrawFlags> = HashSet::new();
    draw_list.add_rect_filled(p_min + Vector2D::new(sz * 0.20, sz * 0.15), p_min + Vector2D::new(sz * 0.80, sz * 0.30), col, 0.0, &draw_flags);
    render_arrow_pointing_at(draw_list, p_min + Vector2D::new(sz * 0.50, sz * 0.85), &Vector2D::new(sz * 0.30, sz * 0.40), &Direction::Down, col);
}


// FIXME: Cleanup and move code to ImDrawList.
// void ImGui::RenderRectFilledRangeH(ImDrawList* draw_list, const Rect& rect, ImU32 col, float x_start_norm, float x_end_norm, float rounding)
pub fn render_rect_filled_range_h(draw_list: &mut DrawList, rect: &Rect, col: u32, mut x_start_norm: f32, mut x_end_norm: f32, mut rounding: f32) {
    if x_end_norm == x_start_norm {
        return;
    }
    if x_start_norm > x_end_norm {
        // ImSwap(x_start_norm, x_end_norm);
        let x = x_end_norm;
        x_end_norm = x_start_norm;
        x_start_norm = x;
    }

    let p0 = Vector2D::new(ImLerp(rect.min.x, rect.max.x, x_start_norm), rect.min.y);
    let p1 = Vector2D::new(ImLerp(rect.min.x, rect.max.x, x_end_norm), rect.max.y);
    if rounding == 0.0 {
        draw_list.add_rect_filled(&p0, &p1, col, 0.0, &DRAW_FLAGS_EMPTY);
        return;
    }

    rounding = f32::clamp(f32::min((rect.max.x - rect.min.x) * 0.5, (rect.max.y - rect.min.y) * 0.5) - 1.0, 0.0, rounding);
    let inv_rounding = 1.0 / rounding;
    let arc0_b = ImAcos01(1.0 - (p0.x - rect.min.x) * inv_rounding);
    let arc0_e = ImAcos01(1.0 - (p1.x - rect.min.x) * inv_rounding);
    let half_pi = f32::PI * 0.5; // We will == compare to this because we know this is the exact value ImAcos01 can return.
    let x0 = ImMax(p0.x, rect.min.x + rounding);
    if arc0_b == arc0_e {
        draw_list.path_line_to(&Vector2D::new(x0, p1.y));
        draw_list.path_line_to(&Vector2D::new(x0, p0.y));
    } else if arc0_b == 0.0 && arc0_e == half_pi {
        draw_list.path_arc_to_fast(&Vector2D::new(x0, p1.y - rounding), rounding, 3, 6); // BL
        draw_list.path_arc_to_fast(&Vector2D::new(x0, p0.y + rounding), rounding, 6, 9); // TR
    } else {
        draw_list.path_arc_to(&Vector2D::new(x0, p1.y - rounding), rounding, f32::PI - arc0_e, f32::PI - arc0_b, 3); // BL
        draw_list.path_arc_to(&Vector2D::new(x0, p0.y + rounding), rounding, f32::PI + arc0_b, f32::PI + arc0_e, 3); // TR
    }
    if p1.x > rect.min.x + rounding {
        let arc1_b = ImAcos01(1.0 - (rect.max.x - p1.x) * inv_rounding);
        let arc1_e = ImAcos01(1.0 - (rect.max.x - p0.x) * inv_rounding);
        let x1 = ImMin(p1.x, rect.max.x - rounding);
        if arc1_b == arc1_e {
            draw_list.path_line_to(&Vector2D::new(x1, p0.y));
            draw_list.path_line_to(&Vector2D::new(x1, p1.y));
        } else if arc1_b == 0.0 && arc1_e == half_pi {
            draw_list.path_arc_to_fast(&Vector2D::new(x1, p0.y + rounding), rounding, 9, 12); // TR
            draw_list.path_arc_to_fast(&Vector2D::new(x1, p1.y - rounding), rounding, 0, 3);  // BR
        } else {
            draw_list.path_arc_to(&Vector2D::new(x1, p0.y + rounding), rounding, -arc1_e, -arc1_b, 3); // TR
            draw_list.path_arc_to(&Vector2D::new(x1, p1.y - rounding), rounding, arc1_b, arc1_e, 3); // BR
        }
    }
    draw_list.path_fill_convex(col);
}

// void ImGui::render_rect_filled_with_hole(ImDrawList* draw_list, const Rect& outer, const Rect& inner, ImU32 col, float rounding)
pub fn render_rect_filled_with_hole(draw_list: &mut DrawList, outer: &Rect, inner: &Rect, col: u32, rounding: f32) {
    let fill_l = (inner.min.x > outer.min.x);
    let fill_r = (inner.max.x < outer.max.x);
    let fill_u = (inner.min.y > outer.min.y);
    let fill_d = (inner.max.y < outer.max.y);
    let mut flags: HashSet<DrawFlags> = HashSet::from([DrawFlags::RoundCornersNone]);
    if fill_l {
        flags.clear();
        if !fill_u {
            flags.insert(DrawFlags::RoundCornersTopLeft);
        }
        if !fill_d {
            flags.insert(DrawFlags::RoundCornersBottomLeft);
        }
        draw_list.add_rect_filled(&Vector2D::new(outer.min.x, inner.min.y), &Vector2D::new(inner.min.x, inner.max.y), col, rounding, &flags);
    }
    if fill_r {
        flags.clear();
        if !fill_u {
            flags.insert(DrawFlags::RoundCornersTopRight);
        }
        if !fill_d {
            flags.insert(DrawFlags::RoundCornersBottomRight);
        }
        draw_list.add_rect_filled(&Vector2D::new(inner.max.x, inner.min.y), &Vector2D::new(outer.max.x, inner.max.y), col, rounding, &flags);
    }
    if fill_u {
        flags.clear();
        if !fill_l {
            flags.insert(DrawFlags::RoundCornersTopLeft);
        }
        if !fill_r {
            flags.insert(DrawFlags::RoundCornersTopRight);
        }
        draw_list.add_rect_filled(&Vector2D::new(inner.min.x, outer.min.y), &Vector2D::new(inner.max.x, inner.min.y), col, rounding, &flags);
    }
    if fill_d {
        flags.clear();
        if !fill_l {
            flags.insert(DrawFlags::RoundCornersBottomLeft);
        }
        if !fill_r {
            flags.insert(DrawFlags::RoundCornersBottomRight);
        }
        draw_list.add_rect_filled(&Vector2D::new(inner.min.x, inner.max.y), &Vector2D::new(inner.max.x, outer.max.y), col, rounding, &flags);
    }
    if fill_l && fill_u {
        flags.clear();
        flags.insert(DrawFlags::RoundCornersTopLeft);
        draw_list.add_rect_filled(&Vector2D::new(outer.min.x, outer.min.y), &Vector2D::new(inner.min.x, inner.min.y), col, rounding, &flags);
    }
    if fill_r && fill_u {
        flags.clear();
        flags.insert(DrawFlags::RoundCornersTopRight);
        draw_list.add_rect_filled(&Vector2D::new(inner.max.x, outer.min.y), &Vector2D::new(outer.max.x, inner.min.y), col, rounding, &flags);
    }
    if fill_l && fill_d {
        flags.clear();
        flags.insert(DrawFlags::RoundCornersBottomLeft);
        draw_list.add_rect_filled(&Vector2D::new(outer.min.x, inner.max.y), &Vector2D::new(inner.min.x, outer.max.y), col, rounding, &flags);
    }
    if fill_r && fill_d {
        flags.clear();
        flags.insert(DrawFlags::RoundCornersBottomRight);
        draw_list.add_rect_filled(&Vector2D::new(inner.max.x, inner.max.y), &Vector2D::new(outer.max.x, outer.max.y), col, rounding, &flags);
    }
}

pub fn calc_rounding_flags_for_rect_in_rect(r_in: &Rect, r_outer: &Rect, threshold: f32) -> HashSet<DrawFlags> {
    let mut draw_flags: HashSet<DrawFlags> = HashSet::new();
    let round_l = r_in.min.x <= r_outer.min.x + threshold;
    let round_r = r_in.max.x >= r_outer.max.x - threshold;
    let round_t = r_in.min.y <= r_outer.min.y + threshold;
    let round_b = r_in.max.y >= r_outer.max.y - threshold;
    // return DrawFlags::RoundCornersNone
    //     | ((round_t && round_l) ? DrawFlags::RoundCornersTopLeft : 0) | ((round_t && round_r) ? DrawFlags::RoundCornersTopRight : 0)
    //     | ((round_b && round_l) ? DrawFlags::RoundCornersBottomLeft : 0) | ((round_b && round_r) ? DrawFlags::RoundCornersBottomRight : 0);
    //
    if round_t && round_l {
        draw_flags.insert(DrawFlags::RoundCornersTopLeft);
    }
    if round_t && round_r {
        draw_flags.insert(DrawFlags::RoundCornersTopRight);
    }
    if round_b && round_l {
        draw_flags.insert(DrawFlags::RoundCornersBottomLeft);
    }
    if round_b && round_r {
        draw_flags.insert(DrawFlags::RoundCornersBottomRight)
    }
    return draw_flags;
}

// Helper for ColorPicker4()
// NB: This is rather brittle and will show artifact when rounding this enabled if rounded corners overlap multiple cells. Caller currently responsible for avoiding that.
// Spent a non reasonable amount of time trying to getting this right for ColorButton with rounding+anti-aliasing+ImGuiColorEditFlags_HalfAlphaPreview flag + various grid sizes and offsets, and eventually gave up... probably more reasonable to disable rounding altogether.
// FIXME: uses ImGui::get_color_u32
// void ImGui::RenderColorRectWithAlphaCheckerboard(ImDrawList* draw_list, Vector2D p_min, Vector2D p_max, ImU32 col, float grid_step, Vector2D grid_off, float rounding, ImDrawFlags flags)
pub fn render_color_rect_with_alpha_checkerboard(draw_list: &mut DrawList, p_min: &Vector2D, p_max: &Vector2D, col: u32, grid_step: f32, grid_off: &Vector2D, rounding: f32, flags: &HashSet<DrawFlags>) {
    let mut draw_flags: HashSet<DrawFlags> = HashSet::new();
    draw_flags = flags.clone();
    // if ((flags & DrawFlags::RoundCornersMask_) == 0) {
    //     flags = DrawFlags::RoundCornersDefault_;
    // }
    if draw_flags_contains_round_corners(flags) {}
    if draw_flags.contains(&DrawFlags::RoundCorners) {
        draw_flags.clear();
        set_draw_flags_round_corners_default(&mut draw_flags);
    }
    if ((col & COLOR32_A_MASK) >> IM_COL32_A_SHIFT) < 0xFF {
        let col_bg1 = get_color_u32(alpha_blend_colors(IM_COL32(204, 204, 204, 255), col));
        let col_bg2 = get_color_u32(alpha_blend_colors(IM_COL32(128, 128, 128, 255), col));
        draw_list.add_rect_filled(p_min, p_max, col_bg1, rounding, flags);

        let mut yi = 0;
        // for (let y =  p_min.y + grid_off.y; y < p_max.y; y += grid_step, yi += 1)
        for y in (p_min.y + grid_off.y..p_max.y).step(grid_step) {
            let y1 = f32::clamp(y, p_min.y, p_max.y);
            let y2 = f32::min(y + grid_step, p_max.y);
            if y2 <= y1 {
                continue;
            }
            // for (let x =  p_min.x + grid_off.x + (yi & 1) * grid_step; x < p_max.x; x += grid_step * 2.0)
            for x in (p_min.x + grid_off.x + (yi & 1) * grid_step..p_max.x).step(grid_step * 2.0) {
                let x1 = f32::clamp(x, p_min.x, p_max.x);
                let x2 = f32::clamp(x + grid_step, p_max.x);
                if x2 <= x1 {
                    continue;
                }
                let mut cell_flags: HashSet<DrawFlags> = HashSet::new();
                if y1 <= p_min.y {
                    if x1 <= p_min.x {
                        // cell_flags |= DrawFlags::RoundCornersTopLeft;
                        cell_flags.insert(DrawFlags::RoundCornersTopLeft);
                    }
                    if x2 >= p_max.x {
                        // cell_flags |= DrawFlags::RoundCornersTopRight;
                        cell_flags.insert(DrawFlags::RoundCornersTopRight);
                    }
                }
                if y2 >= p_max.y {
                    if x1 <= p_min.x {
                        // cell_flags |= DrawFlags::RoundCornersBottomLeft;
                        cell_flags.insert(DrawFlags::RoundCornersBottomLeft);
                    }
                    if x2 >= p_max.x {
                        // cell_flags |= DrawFlags::RoundCornersBottomRight;
                        cell_flags.insert(DrawFlags::RoundCornersBottomRight);
                    }
                }

                // Combine flags
                // cell_flags = (flags == DrawFlags::RoundCornersNone || cell_flags == DrawFlags::RoundCornersNone) ? DrawFlags::RoundCornersNone : (cell_flags & flags);
                if cell_flags.is_empty() || flags.is_empty() {
                    cell_flags.clear();
                } else {
                    cell_flags = cell_flags & flags;
                }
                draw_list.add_rect_filled(&Vector2D::new(x1, y1), &Vector2D::new(x2, y2), col_bg2, rounding, &cell_flags);
            }
            yi += 1;
        }
    } else {
        draw_list.add_rect_filled(p_min, p_max, col, rounding, flags);
    }
}
