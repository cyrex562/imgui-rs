use std::collections::HashSet;
use std::os::raw::c_char;
use std::ptr::null_mut;
use crate::{call_context_hooks, Context, INVALID_ID, window};
use crate::color::{IM_COL32_A_MASK, IM_COL32_BLACK, IM_COL32_WHITE, make_color_32};
use crate::draw::data::add_root_window_to_draw_data;
use crate::draw::list::{add_draw_list_to_draw_data, get_background_draw_list, get_foreground_draw_list};
use crate::frame::end_frame;
use crate::imgui_globals::GImGui;
use crate::imgui_h::Color;
use crate::imgui_style::{get_color_u32, GetColorU32_2, GetColorU32_3};
use crate::input::MouseCursor;
use crate::nav::NavHighlightFlags;
use crate::style::get_color_u32;
use crate::types::Id32;
use crate::vectors::two_d::Vector2D;
use crate::viewport::setup_viewport_draw_data;
use crate::window::{get, Window, WindowFlags};
use crate::window::checks::is_window_active_and_visible;
use crate::window::get::find_bottom_most_visible_window_with_begin_stack;

// const char* ImGui::FindRenderedTextEnd(const char* text, const char* text_end)
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
// render_text***() functions calls ImDrawList::add_text() calls ImBitmapFont::render_text()
// void ImGui::render_text(Vector2D pos, const char* text, const char* text_end, bool hide_text_after_hash)
pub unsafe fn RenderText(pos: &Vector2D, text: &str, mut text_end: &str, hide_text_after_hash: bool)
{
    // ImGuiContext& g = *GImGui;
    let g = GImGui;
    // ImGuiWindow* window = g.current_window;
    let window = g.current_window;

    // Hide anything after a '##' string
    // const char* text_display_end;
    let mut text_display_end: &str = null_mut();
    if hide_text_after_hash
    {
        text_display_end = find_rendered_text_end(text, text_end);
    }
    else
    {
        if !text_end {
            text_end = text + strlen(text);
        }// FIXME-OPT
        text_display_end = text_end;
    }

    if text != text_display_end
    {
        window.draw_list.AddText2(&g.font, g.font_size, pos, GetColorU32_3(Color::Text as u32), text, text_display_end, 0.0, None);
        if g.LogEnabled {
            LogRenderedText(&pos, text, text_display_end);
        }
    }
}

// void ImGui::RenderTextWrapped(Vector2D pos, const char* text, const char* text_end, float wrap_width)
pub fn RenderTextWrapped(pos: &Vector2D, text: &str, mut text_end: &str, wrap_width: f32)
{
    // ImGuiContext& g = *GImGui;
    let g = GImGui;
    // ImGuiWindow* window = g.current_window;
    let window = g.current_window;

    if !text_end {
        text_end = text + strlen(text);
    } // FIXME-OPT

    if text != text_end
    {
        window.draw_list.AddText2(&g.font, g.font_size, pos, GetColorU32_3(StyleColor::Text), text, text_end, wrap_width);
        if g.LogEnabled {
            LogRenderedText(&pos, text, text_end);
        }
    }
}

// Default clip_rect uses (pos_min,pos_max)
// Handle clipping on CPU immediately (vs typically let the GPU clip the triangles that are overlapping the clipping rectangle edges)
void ImGui::RenderTextClippedEx(ImDrawList* draw_list, const Vector2D& pos_min, const Vector2D& pos_max, const char* text, const char* text_display_end, const Vector2D* text_size_if_known, const Vector2D& align, const Rect* clip_rect)
{
    // Perform CPU side clipping for single clipped element to avoid using scissor state
    Vector2D pos = pos_min;
    const Vector2D text_size = text_size_if_known ? *text_size_if_known : CalcTextSize(text, text_display_end, false, 0.0);

    const Vector2D* clip_min = clip_rect ? &clip_rect.min : &pos_min;
    const Vector2D* clip_max = clip_rect ? &clip_rect.Max : &pos_max;
    bool need_clipping = (pos.x + text_size.x >= clip_max.x) || (pos.y + text_size.y >= clip_max.y);
    if (clip_rect) // If we had no explicit clipping rectangle then pos==clip_min
        need_clipping |= (pos.x < clip_min.x) || (pos.y < clip_min.y);

    // Align whole block. We should defer that to the better rendering function when we'll have support for individual line alignment.
    if (align.x > 0.0) pos.x = ImMax(pos.x, pos.x + (pos_max.x - pos.x - text_size.x) * align.x);
    if (align.y > 0.0) pos.y = ImMax(pos.y, pos.y + (pos_max.y - pos.y - text_size.y) * align.y);

    // Render
    if (need_clipping)
    {
        Vector4D fine_clip_rect(clip_min.x, clip_min.y, clip_max.x, clip_max.y);
        draw_list.AddText(None, 0.0, pos, get_color_u32(StyleColor::Text), text, text_display_end, 0.0, &fine_clip_rect);
    }
    else
    {
        draw_list.AddText(None, 0.0, pos, get_color_u32(StyleColor::Text), text, text_display_end, 0.0, None);
    }
}

void ImGui::render_text_clipped(const Vector2D& pos_min, const Vector2D& pos_max, const char* text, const char* text_end, const Vector2D* text_size_if_known, const Vector2D& align, const Rect* clip_rect)
{
    // Hide anything after a '##' string
    const char* text_display_end = FindRenderedTextEnd(text, text_end);
    let text_len = (text_display_end - text);
    if (text_len == 0)
        return;

    // ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    RenderTextClippedEx(window.draw_list, pos_min, pos_max, text, text_display_end, text_size_if_known, align, clip_rect);
    if (g.LogEnabled)
        LogRenderedText(&pos_min, text, text_display_end);
}


// Another overly complex function until we reorganize everything into a nice all-in-one helper.
// This is made more complex because we have dissociated the layout rectangle (pos_min..pos_max) which define _where_ the ellipsis is, from actual clipping of text and limit of the ellipsis display.
// This is because in the context of tabs we selectively hide part of the text when the Close Button appears, but we don't want the ellipsis to move.
void ImGui::RenderTextEllipsis(ImDrawList* draw_list, const Vector2D& pos_min, const Vector2D& pos_max, float clip_max_x, float ellipsis_max_x, const char* text, const char* text_end_full, const Vector2D* text_size_if_known)
{
    // ImGuiContext& g = *GImGui;
    if (text_end_full == None)
        text_end_full = FindRenderedTextEnd(text);
    const Vector2D text_size = text_size_if_known ? *text_size_if_known : CalcTextSize(text, text_end_full, false, 0.0);

    //draw_list->add_line(Vector2D(pos_max.x, pos_min.y - 4), Vector2D(pos_max.x, pos_max.y + 4), IM_COL32(0, 0, 255, 255));
    //draw_list->add_line(Vector2D(ellipsis_max_x, pos_min.y-2), Vector2D(ellipsis_max_x, pos_max.y+2), IM_COL32(0, 255, 0, 255));
    //draw_list->add_line(Vector2D(clip_max_x, pos_min.y), Vector2D(clip_max_x, pos_max.y), IM_COL32(255, 0, 0, 255));
    // FIXME: We could technically remove (last_glyph->advance_x - last_glyph->x1) from text_size.x here and save a few pixels.
    if (text_size.x > pos_max.x - pos_min.x)
    {
        // Hello wo...
        // |       |   |
        // min   max   ellipsis_max
        //          <-> this is generally some padding value

        const ImFont* font = draw_list->_Data.Font;
        let font_size = draw_list->_Data.font_size;
        const char* text_end_ellipsis = None;

        ImWchar ellipsis_char = font.EllipsisChar;
        int ellipsis_char_count = 1;
        if (ellipsis_char == (ImWchar)-1)
        {
            ellipsis_char = font.DotChar;
            ellipsis_char_count = 3;
        }
        const ImFontGlyph* glyph = font.FindGlyph(ellipsis_char);

        let ellipsis_glyph_width =  glyph.X1;                 // width of the glyph with no padding on either side
        let ellipsis_total_width =  ellipsis_glyph_width;      // Full width of entire ellipsis

        if (ellipsis_char_count > 1)
        {
            // Full ellipsis size without free spacing after it.
            let spacing_between_dots = 1.0 * (draw_list->_Data.font_size / font.font_size);
            ellipsis_glyph_width = glyph.X1 - glyph.X0 + spacing_between_dots;
            ellipsis_total_width = ellipsis_glyph_width * ellipsis_char_count - spacing_between_dots;
        }

        // We can now claim the space between pos_max.x and ellipsis_max.x
        let text_avail_width = ImMax((ImMax(pos_max.x, ellipsis_max_x) - ellipsis_total_width) - pos_min.x, 1.0);
        let text_size_clipped_x =  font.CalcTextSizeA(font_size, text_avail_width, 0.0, text, text_end_full, &text_end_ellipsis).x;
        if (text == text_end_ellipsis && text_end_ellipsis < text_end_full)
        {
            // Always display at least 1 character if there's no room for character + ellipsis
            text_end_ellipsis = text + ImTextCountUtf8BytesFromChar(text, text_end_full);
            text_size_clipped_x = font.CalcTextSizeA(font_size, f32::MAX, 0.0, text, text_end_ellipsis).x;
        }
        while (text_end_ellipsis > text && ImCharIsBlankA(text_end_ellipsis[-1]))
        {
            // Trim trailing space before ellipsis (FIXME: Supporting non-ascii blanks would be nice, for this we need a function to backtrack in UTF-8 text)
            text_end_ellipsis--;
            text_size_clipped_x -= font.CalcTextSizeA(font_size, f32::MAX, 0.0, text_end_ellipsis, text_end_ellipsis + 1).x; // Ascii blanks are always 1 byte
        }

        // Render text, render ellipsis
        RenderTextClippedEx(draw_list, pos_min, Vector2D::new(clip_max_x, pos_max.y), text, text_end_ellipsis, &text_size, Vector2D::new(0.0, 0.0));
        let ellipsis_x =  pos_min.x + text_size_clipped_x;
        if (ellipsis_x + ellipsis_total_width <= ellipsis_max_x)
            for (int i = 0; i < ellipsis_char_count; i += 1)
            {
                font.RenderChar(draw_list, font_size, Vector2D::new(ellipsis_x, pos_min.y), get_color_u32(StyleColor::Text), ellipsis_char);
                ellipsis_x += ellipsis_glyph_width;
            }
    }
    else
    {
        RenderTextClippedEx(draw_list, pos_min, Vector2D::new(clip_max_x, pos_max.y), text, text_end_full, &text_size, Vector2D::new(0.0, 0.0));
    }

    if (g.LogEnabled)
        LogRenderedText(&pos_min, text, text_end_full);
}

// Render a rectangle shaped with optional rounding and borders
void ImGui::RenderFrame(Vector2D p_min, Vector2D p_max, ImU32 fill_col, bool border, float rounding)
{
    // ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    window.draw_list.add_rect_filled(p_min, p_max, fill_col, rounding);
    let border_size = g.style.frame_border_size;
    if (border && border_size > 0.0)
    {
        window.draw_list.add_rect(p_min + Vector2D::new(1, 1), p_max + Vector2D::new(1, 1), get_color_u32(StyleColor::BorderShadow), rounding, 0, border_size);
        window.draw_list.add_rect(p_min, p_max, get_color_u32(StyleColor::Border), rounding, 0, border_size);
    }
}

void ImGui::RenderFrameBorder(Vector2D p_min, Vector2D p_max, float rounding)
{
    // ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    let border_size = g.style.frame_border_size;
    if (border_size > 0.0)
    {
        window.draw_list.add_rect(p_min + Vector2D::new(1, 1), p_max + Vector2D::new(1, 1), get_color_u32(StyleColor::BorderShadow), rounding, 0, border_size);
        window.draw_list.add_rect(p_min, p_max, get_color_u32(StyleColor::Border), rounding, 0, border_size);
    }
}

// void ImGui::render_nav_highlight(const Rect& bb, ImGuiID id, ImGuiNavHighlightFlags flags)
pub fn render_nav_highlight(g: &mut Context, bb: &Rect, id: Id32, flags: Option<&HashSet<NavHighlightFlags>>)
{
    // ImGuiContext& g = *GImGui;
    if (id != g.nav_id)
        return;
    if (g.nav_disable_highlight && !(flags & ImGuiNavHighlightFlags_AlwaysDraw))
        return;
    ImGuiWindow* window = g.current_window;
    if (window.dc.NavHideHighlightOneFrame)
        return;

    let rounding =  (flags & ImGuiNavHighlightFlags_NoRounding) ? 0.0 : g.style.frame_rounding;
    Rect display_rect = bb;
    display_rect.clip_with(window.clip_rect);
    if (flags & ImGuiNavHighlightFlags_TypeDefault)
    {
        let THICKNESS = 2.0;
        let DISTANCE = 3.0 + THICKNESS * 0.5;
        display_rect.Expand(Vector2D::new(DISTANCE, DISTANCE));
        bool fully_visible = window.clip_rect.contains(display_rect);
        if (!fully_visible)
            window.draw_list.push_clip_rect(display_rect.min, display_rect.max);
        window.draw_list.add_rect(display_rect.min + Vector2D::new(THICKNESS * 0.5, THICKNESS * 0.5), display_rect.max - Vector2D::new(THICKNESS * 0.5, THICKNESS * 0.5), get_color_u32(StyleColor::NavHighlight), rounding, 0, THICKNESS);
        if (!fully_visible)
            window.draw_list.pop_clip_rect();
    }
    if (flags & NavHighlightingFlags::TypeThin)
    {
        window.draw_list.add_rect(display_rect.min, display_rect.max, get_color_u32(StyleColor::NavHighlight), rounding, 0, 1.0);
    }
}

void ImGui::render_mouse_cursor(Vector2D base_pos, float base_scale, ImGuiMouseCursor mouse_cursor, ImU32 col_fill, ImU32 col_border, ImU32 col_shadow)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(mouse_cursor > MouseCursor::None && mouse_cursor < ImGuiMouseCursor_COUNT);
    ImFontAtlas* font_atlas = g.draw_list_shared_data.font.container_atlas;
    for (int n = 0; n < g.viewports.size; n += 1)
    {
        // We scale cursor with current viewport/monitor, however windows 10 for its own hardware cursor seems to be using a different scale factor.
        Vector2D offset, size, uv[4];
        if (!font_atlas.GetMouseCursorTexData(mouse_cursor, &offset, &size, &uv[0], &uv[2]))
            continue;
        ImGuiViewportP* viewport = g.viewports[n];
        const Vector2D pos = base_pos - offset;
        let scale = base_scale * viewport.DpiScale;
        if (!viewport.get_main_rect().Overlaps(Rect(pos, pos + Vector2D::new(size.x + 2, size.y + 2) * scale)))
            continue;
        ImDrawList* draw_list = get_foreground_draw_list(viewport);
        ImTextureID tex_id = font_atlas.TexID;
        draw_list.PushTextureID(tex_id);
        draw_list.AddImage(tex_id, pos + Vector2D::new(1, 0) * scale, pos + (Vector2D::new(1, 0) + size) * scale, uv[2], uv[3], col_shadow);
        draw_list.AddImage(tex_id, pos + Vector2D::new(2, 0) * scale, pos + (Vector2D::new(2, 0) + size) * scale, uv[2], uv[3], col_shadow);
        draw_list.AddImage(tex_id, pos,                        pos + size * scale,                  uv[2], uv[3], col_border);
        draw_list.AddImage(tex_id, pos,                        pos + size * scale,                  uv[0], uv[1], col_fill);
        draw_list.PopTextureID();
    }
}

// static void ImGuis::RenderDimmedBackgrounds()
pub fn render_dimmed_backgrounds(g: &mut Context) {
    // ImGuiContext& g = *GImGui;
    // ImGuiWindow* modal_window = GetTopMostAndVisiblePopupModal();
    let modal_window: &mut Window = get_top_most_and_visible_popup_modal();
    if g.dim_bg_ration <= 0.0 && g.nav_windowing_highlight_alpha <= 0.0 {
        return;
    }
    // const bool dim_bg_for_modal = (modal_window != None);

    // const bool dim_bg_for_window_list = (g.NavWindowingTargetAnim != None && g.NavWindowingTargetAnim->Active);
    let dim_bg_for_window_list = true;
    let nav_win_tgt_anim = g.get_window(g.nav_windowing_target_anim).unwrap();
    let dim_bg_for_window_list = g.nav_windowing_target_anim != INVALID_ID && nav_win_tgt_anim.active;
    if !dim_bg_for_modal && !dim_bg_for_window_list {
        return;
    }

    // ImGuiViewport* viewports_already_dimmed[2] = { None, None };
    let viewports_already_dimmedd: [Id32; 2] = [0, 0];
    if dim_bg_for_modal {
        // Draw dimming behind modal or a begin stack child, whichever comes first in draw order.
        // ImGuiWindow* dim_behind_window = FindBottomMostVisibleWindowWithinBeginStack(modal_window);
        let dim_behind_window = find_bottom_most_visible_window_with_begin_stack(g, modal_window);
        // RenderDimmedBackgroundBehindWindow(dim_behind_window, get_color_u32(ImGuiCol_ModalWindowDimBg, g.dim_bg_ration));
        render_dimmed_background_behind_window(g, dim_behind_window, get_color_u32(StyleColor::ModalWindowDimBg, g.dim_bg_ratio));
        viewports_already_dimmed[0] = modal_window.viewport_id;
    } else if dim_bg_for_window_list {
        // Draw dimming behind CTRL+Tab target window and behind CTRL+Tab UI window
        let nwta_win = g.get_window(g.nav_windowing_target_anim).unwrap();
        let nwl_win = g.get_window(g.nav_windowing_list_window_id).unwrap();
        render_dimmed_background_behind_window(g, g.NavWindowingTargetAnim, get_color_u32(StyleColor::NavWindowingDimBg, g.dim_bg_ration));
        if g.nav_windowing_list_window_id != INVALID_ID {

            if nwl_win.viewport_id != INVALID_ID {
                if nwl_win.viewport_id != nwta_win.viewport_id{
                    render_dimmed_background_behind_window(g, nwl_win, get_color_u32(StyleColor::NavWindowingDimBg, g.dim_bg_ration));
                }
            }
        }

        viewports_already_dimmed[0] = nwta_win.viewport_id;
        viewports_already_dimmed[1] = nwl_win.viewport_id;

        // Draw border around CTRL+Tab target window
        // ImGuiWindow * window = g.NavWindowingTargetAnim;
        // ImGuiViewport * viewport = window.viewport;
        let nwta_vp = g.get_viewport(nwta_win.viewport_id).unwrap();
        let distance = g.font_size;
        let bb = nwta_win.rect();
        // float
        // distance = g.FontSize;
        // ImRect
        // bb = window.Rect();
        bb.expand(distance);
        if bb.get_width() >= viewport.size.x && bb.get_height() >= viewport.size.y){
            bb.Expand(-distance - 1.0);
        } // If a window fits the entire viewport, adjust its highlight inward
        if window.draw_list.cmd_buffer.size == 0 {
            window.draw_list.add_draw_cmd();
        }
        window.draw_list.push_clip_rect(viewport.pos, viewport.pos + viewport.size);
        window.draw_list.add_rect(bb.min, bb.max, get_color_u32(StyleColor::NavWindowingHighlight, g.nav_windowing_highlight_alpha), window.WindowRounding, 0, 3.0);
        window.draw_list.pop_clip_rect();
     }

    // Draw dimming background on _other_ viewports than the ones our windows are in
    // for (int viewport_n = 0; viewport_n < g.viewports.Size; viewport_n += 1)
    for vp_id in g.viewports.iter_mut()
    {
        // ImGuiViewportP* viewport = g.viewports[viewport_n];
        if viewport.id == viewports_already_dimmed[0] || viewport == viewports_already_dimmed[1] {
            continue;
        }
        if modal_window && viewport.window && is_window_above(viewport.window, modal_window){
            continue;
        }
        // ImDrawList* draw_list = GetForegroundDrawList(viewport);
        let draw_list = get_foreground_draw_list(g, viewport);
        let dim_bg_col = get_color_u32(if dim_bg_for_modal { StyleColor::ModalWindowDimBg } else { StyleColor::NavWindowingDimBg }, g.dim_bg_ration);
        draw_list.add_rect_filled(viewport.pos, viewport.pos + viewport.size, dim_bg_col, 0.0, 0.0);
    }
}

// Prepare the data for rendering so you can call GetDrawData()
// (As with anything within the ImGui:: namspace this doesn't touch your GPU or graphics API at all:
// it is the role of the ImGui_ImplXXXX_RenderDrawData() function provided by the renderer backend)
// void ImGui::Render()
pub fn render(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.initialized);

    if (g.frame_count_ended != g.frame_count) {
        end_frame(g);
    }
    let first_render_of_frame = (g.frame_count_rendered != g.frame_count);
    g.frame_count_rendered = g.frame_count;
    g.io.metrics_render_windows = 0;

    call_context_hooks(g, ImGuiContextHookType_RenderPre);

    // Add background ImDrawList (for each active viewport)
    // for (int n = 0; n != g.viewports.Size; n += 1)
    for viewport in g.viewports.iter_mut()
    {
        // ImGuiViewportP* viewport = g.viewports[n];
        viewport.draw_data_builder.clear();
        if viewport.draw_lists[0] != INVALID_ID {
            add_draw_list_to_draw_data(g, &mut viewport.draw_data_builder.layers[0], get_background_draw_list(g, viewport).id);
        }
    }

    // Add ImDrawList to render
    // ImGuiWindow* windows_to_render_top_most[2];
    let mut windows_to_render_top_most: [Id32;2] = [
        if g.nav_windowing_target_id != INVALID_ID && !g.nav_windowing_target_id.flags.contains(WindowFlags::NoBringToFrontOnFocus) {
            let nwt_win = g.get_window(g.nav_windowing_target_id).unwrap();
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
    // windows_to_render_top_most[0] = (g.nav_windowing_target_id && !(g.nav_windowing_target_id.flags & ImGuiWindowFlags_NoBringToFrontOnFocus)) ? g.nav_windowing_target_id ->root_window_dock_tree : None;
    // windows_to_render_top_most[1] = (g.nav_windowing_target_id? g.nav_windowing_list_window : None);
    // for (int n = 0; n != g.windows.Size; n += 1)
    for (win_id, window) in g.windows.iter_mut()
    {
        // ImGuiWindow* window = g.windows[n];
        // IM_MSVC_WARNING_SUPPRESS(6011); // Static Analysis false positive "warning C6011: Dereferencing None pointer 'window'"
        if is_window_active_and_visible(window) && (!window.flags.contains(&WindowFlags::ChildWindow)) && window.id != windows_to_render_top_most[0] && window.id != windows_to_render_top_most[1] {
            add_root_window_to_draw_data(g, window);
        }
    }
    // for (int n = 0; n < IM_ARRAYSIZE(windows_to_render_top_most); n += 1)
    for n in 0 .. windows_to_render_top_most.len() {
        if windows_to_render_top_most[n] != INVALID_ID && is_window_active_and_visible(g.get_window(windows_to_render_top_most[n]).unwrap()) { // nav_windowing_target is always temporarily displayed as the top-most window
            add_root_window_to_draw_data(g, g.get_window(windows_to_render_top_most[n]).unwrap());
        }
    }

    // Draw modal/window whitening backgrounds
    if first_render_of_frame {
        render_dimmed_backgrounds();
    }

    // Draw software mouse cursor if requested by io.mouse_draw_cursor flag
    if g.io.mouse_draw_cursor && first_render_of_frame && g.mouse_cursor != MouseCursor::None {
        render_mouse_cursor(&g.io.mouse_pos, g.style.MouseCursorScale, &g.mouse_cursor, IM_COL32_WHITE, IM_COL32_BLACK, make_color_32(0, 0, 0, 48));
    }

    // Setup ImDrawData structures for end-user
    g.io.metrics_render_vertices = 0;
    g.io.metrics_render_indices = 0;
    // for (int n = 0; n < g.viewports.Size; n += 1)
    for viewport in g.viewports.iter_mut()
    {
        // ImGuiViewportP* viewport = g.viewports[n];
        // viewport->DrawDataBuilder.FlattenIntoSingleLayer();
        viewport.draw_data_builder.flatten_into_single_layer();

        // Add foreground ImDrawList (for each active viewport)
        if viewport.draw_lists[1] != INVALID_ID {
        add_draw_list_to_draw_data(g, &mut viewport.draw_data_builder.layers[0], get_foreground_draw_list(g,viewport));
    }
        setup_viewport_draw_data(g, viewport, &viewport.draw_data_builder.layers[0]);
        let draw_data = &viewport.draw_data;
        g.io.metrics_render_vertices += draw_data.total_vtx_count;
        g.io.metrics_render_indices += draw_data.total_idx_count;
    }

    call_context_hooks(g, ImGuiContextHookType_RenderPost);
}

// static void ImGui::RenderDimmedBackgroundBehindWindow(ImGuiWindow* window, ImU32 col)
pub fn render_dimmed_background_behind_window(g: &mut Context, window: &mut Window, color: u32)
{
    if (color & IM_COL32_A_MASK) == 0 {
        return;
    }

    // ImGuiViewportP* viewport = window.viewport;
    let viewport = g.get_viewport(window.viewport_id).unwrap();
    // ImRect viewport_rect = viewport->get_main_rect();
    let viewport_rect = viewport.get_main_rect();

    // Draw behind window by moving the draw command at the FRONT of the draw list

    // We've already called AddWindowToDrawData() which called draw_list->ChannelsMerge() on DockNodeHost windows,
    // and draw list have been trimmed already, hence the explicit recreation of a draw command if missing.
    // FIXME: This is creating complication, might be simpler if we could inject a drawlist in drawdata at a given position and not attempt to manipulate ImDrawCmd order.
    // ImDrawList* draw_list = window.root_window_dock_tree->DrawList;
    let root_win_dock_tree_win = g.get_window(window.root_window_dock_tree_id).unwrap();
    let draw_list = g.get_draw_list(root_win_dock_tree_win.draw_list_id).unwrap();
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
    let root_win = g.get_window(window.root_window_id).unwrap();
    if root_win.dock_is_active
    {
        // ImDrawList* draw_list = FindFrontMostVisibleChildWindow(window.root_window_dock_tree)->DrawList;

        let draw_list = g.get_draw_list(get::find_front_most_visible_child_window(g, root_win_dock_tree_win).draw_list_id).unwrap();
        if draw_list.cmd_buffer.len() == 0 {
            draw_list.add_draw_cmd();
        }
        draw_list.push_clip_rect(viewport_rect.min, viewport_rect.max, false);
        render_rect_filled_with_hole(draw_list, root_win_dock_tree_win.rect(), root_win.rect(), color, 0.0);// window->root_window_dock_tree->window_rounding);
        draw_list.pop_clip_rect();
    }
}


//-----------------------------------------------------------------------------
// [SECTION] ImGui Internal Render Helpers
//-----------------------------------------------------------------------------
// Vaguely redesigned to stop accessing ImGui global state:
// - RenderArrow()
// - RenderBullet()
// - RenderCheckMark()
// - RenderArrowDockMenu()
// - RenderArrowPointingAt()
// - RenderRectFilledRangeH()
// - RenderRectFilledWithHole()
//-----------------------------------------------------------------------------
// Function in need of a redesign (legacy mess)
// - RenderColorRectWithAlphaCheckerboard()
//-----------------------------------------------------------------------------

// Render an arrow aimed to be aligned with text (p_min is a position in the same space text would be positioned). To e.g. denote expanded/collapsed state
void ImGui::RenderArrow(ImDrawList* draw_list, Vector2D pos, ImU32 col, ImGuiDir dir, float scale)
{
    let h = draw_list->_Data.font_size * 1.00;
    let r =  h * 0.40 * scale;
    Vector2D center = pos + Vector2D::new(h * 0.50, h * 0.50 * scale);

    Vector2D a, b, c;
    switch (dir)
    {
    case Direction::Up:
    case Direction::Down:
        if (dir == Direction::Up) r = -r;
        a = Vector2D::new(+0.000, +0.750) * r;
        b = Vector2D::new(-0.866, -0.750) * r;
        c = Vector2D::new(+0.866, -0.750) * r;
        break;
    case Direction::Left:
    case Direction::Right:
        if (dir == Direction::Left) r = -r;
        a = Vector2D::new(+0.750, +0.000) * r;
        b = Vector2D::new(-0.750, +0.866) * r;
        c = Vector2D::new(-0.750, -0.866) * r;
        break;
    case Direction::None:
    case Direction::COUNT:
        // IM_ASSERT(0);
        break;
    }
    draw_list.add_triangle_filled(center + a, center + b, center + c, col);
}

void ImGui::render_bullet(ImDrawList* draw_list, Vector2D pos, ImU32 col)
{
    draw_list.AddCircleFilled(pos, draw_list->_Data.font_size * 0.20, col, 8);
}

void ImGui::RenderCheckMark(ImDrawList* draw_list, Vector2D pos, ImU32 col, float sz)
{
    let thickness =  ImMax(sz / 5.0, 1.0);
    sz -= thickness * 0.5;
    pos += Vector2D::new(thickness * 0.25, thickness * 0.25);

    let third =  sz / 3.0;
    let bx =  pos.x + third;
    let by =  pos.y + sz - third * 0.5;
    draw_list.path_line_to(Vector2D::new(bx - third, by - third));
    draw_list.path_line_to(Vector2D::new(bx, by));
    draw_list.path_line_to(Vector2D::new(bx + third * 2.0, by - third * 2.0));
    draw_list.path_stroke(col, 0, thickness);
}

// Render an arrow. 'pos' is position of the arrow tip. half_sz.x is length from base to tip. half_sz.y is length on each side.
void ImGui::RenderArrowPointingAt(ImDrawList* draw_list, Vector2D pos, Vector2D half_sz, ImGuiDir direction, ImU32 col)
{
    switch (direction)
    {
    case Direction::Left:  draw_list.add_triangle_filled(Vector2D::new(pos.x + half_sz.x, pos.y - half_sz.y), Vector2D::new(pos.x + half_sz.x, pos.y + half_sz.y), pos, col); return;
    case Direction::Right: draw_list.add_triangle_filled(Vector2D::new(pos.x - half_sz.x, pos.y + half_sz.y), Vector2D::new(pos.x - half_sz.x, pos.y - half_sz.y), pos, col); return;
    case Direction::Up:    draw_list.add_triangle_filled(Vector2D::new(pos.x + half_sz.x, pos.y + half_sz.y), Vector2D::new(pos.x - half_sz.x, pos.y + half_sz.y), pos, col); return;
    case Direction::Down:  draw_list.add_triangle_filled(Vector2D::new(pos.x - half_sz.x, pos.y - half_sz.y), Vector2D::new(pos.x + half_sz.x, pos.y - half_sz.y), pos, col); return;
    case Direction::None: case Direction::COUNT: break; // Fix warnings
    }
}

// This is less wide than RenderArrow() and we use in dock nodes instead of the regular RenderArrow() to denote a change of functionality,
// and because the saved space means that the left-most tab label can stay at exactly the same position as the label of a loose window.
void ImGui::RenderArrowDockMenu(ImDrawList* draw_list, Vector2D p_min, float sz, ImU32 col)
{
    draw_list.add_rect_filled(p_min + Vector2D::new(sz * 0.20, sz * 0.15), p_min + Vector2D::new(sz * 0.80, sz * 0.30), col);
    RenderArrowPointingAt(draw_list, p_min + Vector2D::new(sz * 0.50, sz * 0.85), Vector2D::new(sz * 0.30, sz * 0.40), Direction::Down, col);
}


// FIXME: Cleanup and move code to ImDrawList.
void ImGui::RenderRectFilledRangeH(ImDrawList* draw_list, const Rect& rect, ImU32 col, float x_start_norm, float x_end_norm, float rounding)
{
    if (x_end_norm == x_start_norm)
        return;
    if (x_start_norm > x_end_norm)
        ImSwap(x_start_norm, x_end_norm);

    Vector2D p0 = Vector2D::new(ImLerp(rect.min.x, rect.max.x, x_start_norm), rect.min.y);
    Vector2D p1 = Vector2D::new(ImLerp(rect.min.x, rect.max.x, x_end_norm), rect.max.y);
    if (rounding == 0.0)
    {
        draw_list.add_rect_filled(p0, p1, col, 0.0);
        return;
    }

    rounding = ImClamp(ImMin((rect.max.x - rect.min.x) * 0.5, (rect.max.y - rect.min.y) * 0.5) - 1.0, 0.0, rounding);
    let inv_rounding = 1.0 / rounding;
    let arc0_b = ImAcos01(1.0 - (p0.x - rect.min.x) * inv_rounding);
    let arc0_e = ImAcos01(1.0 - (p1.x - rect.min.x) * inv_rounding);
    let half_pi = f32::PI * 0.5; // We will == compare to this because we know this is the exact value ImAcos01 can return.
    let x0 = ImMax(p0.x, rect.min.x + rounding);
    if (arc0_b == arc0_e)
    {
        draw_list.path_line_to(Vector2D::new(x0, p1.y));
        draw_list.path_line_to(Vector2D::new(x0, p0.y));
    }
    else if (arc0_b == 0.0 && arc0_e == half_pi)
    {
        draw_list.path_arc_to_fast(Vector2D::new(x0, p1.y - rounding), rounding, 3, 6); // BL
        draw_list.path_arc_to_fast(Vector2D::new(x0, p0.y + rounding), rounding, 6, 9); // TR
    }
    else
    {
        draw_listpath_arc_to(Vector2D::new(x0, p1.y - rounding), rounding, f32::PI - arc0_e, f32::PI - arc0_b, 3); // BL
        draw_listpath_arc_to(Vector2D::new(x0, p0.y + rounding), rounding, f32::PI + arc0_b, f32::PI + arc0_e, 3); // TR
    }
    if (p1.x > rect.min.x + rounding)
    {
        let arc1_b = ImAcos01(1.0 - (rect.max.x - p1.x) * inv_rounding);
        let arc1_e = ImAcos01(1.0 - (rect.max.x - p0.x) * inv_rounding);
        let x1 = ImMin(p1.x, rect.max.x - rounding);
        if (arc1_b == arc1_e)
        {
            draw_list.path_line_to(Vector2D::new(x1, p0.y));
            draw_list.path_line_to(Vector2D::new(x1, p1.y));
        }
        else if (arc1_b == 0.0 && arc1_e == half_pi)
        {
            draw_list.path_arc_to_fast(Vector2D::new(x1, p0.y + rounding), rounding, 9, 12); // TR
            draw_list.path_arc_to_fast(Vector2D::new(x1, p1.y - rounding), rounding, 0, 3);  // BR
        }
        else
        {
            draw_listpath_arc_to(Vector2D::new(x1, p0.y + rounding), rounding, -arc1_e, -arc1_b, 3); // TR
            draw_listpath_arc_to(Vector2D::new(x1, p1.y - rounding), rounding, +arc1_b, +arc1_e, 3); // BR
        }
    }
    draw_list.path_fill_convex(col);
}

void ImGui::render_rect_filled_with_hole(ImDrawList* draw_list, const Rect& outer, const Rect& inner, ImU32 col, float rounding)
{
    const bool fill_L = (inner.min.x > outer.min.x);
    const bool fill_R = (inner.max.x < outer.max.x);
    const bool fill_U = (inner.min.y > outer.min.y);
    const bool fill_D = (inner.max.y < outer.max.y);
    if (fill_L) draw_list.add_rect_filled(Vector2D::new(outer.min.x, inner.min.y), Vector2D::new(inner.min.x, inner.max.y), col, rounding, DrawFlags::RoundCornersNone | (fill_U ? 0 : DrawFlags::RoundCornersTopLeft)    | (fill_D ? 0 : DrawFlags::RoundCornersBottomLeft));
    if (fill_R) draw_list.add_rect_filled(Vector2D::new(inner.max.x, inner.min.y), Vector2D::new(outer.max.x, inner.max.y), col, rounding, DrawFlags::RoundCornersNone | (fill_U ? 0 : DrawFlags::RoundCornersTopRight)   | (fill_D ? 0 : DrawFlags::RoundCornersBottomRight));
    if (fill_U) draw_list.add_rect_filled(Vector2D::new(inner.min.x, outer.min.y), Vector2D::new(inner.max.x, inner.min.y), col, rounding, DrawFlags::RoundCornersNone | (fill_L ? 0 : DrawFlags::RoundCornersTopLeft)    | (fill_R ? 0 : DrawFlags::RoundCornersTopRight));
    if (fill_D) draw_list.add_rect_filled(Vector2D::new(inner.min.x, inner.max.y), Vector2D::new(inner.max.x, outer.max.y), col, rounding, DrawFlags::RoundCornersNone | (fill_L ? 0 : DrawFlags::RoundCornersBottomLeft) | (fill_R ? 0 : DrawFlags::RoundCornersBottomRight));
    if (fill_L && fill_U) draw_list.add_rect_filled(Vector2D::new(outer.min.x, outer.min.y), Vector2D::new(inner.min.x, inner.min.y), col, rounding, DrawFlags::RoundCornersTopLeft);
    if (fill_R && fill_U) draw_list.add_rect_filled(Vector2D::new(inner.max.x, outer.min.y), Vector2D::new(outer.max.x, inner.min.y), col, rounding, DrawFlags::RoundCornersTopRight);
    if (fill_L && fill_D) draw_list.add_rect_filled(Vector2D::new(outer.min.x, inner.max.y), Vector2D::new(inner.min.x, outer.max.y), col, rounding, DrawFlags::RoundCornersBottomLeft);
    if (fill_R && fill_D) draw_list.add_rect_filled(Vector2D::new(inner.max.x, inner.max.y), Vector2D::new(outer.max.x, outer.max.y), col, rounding, DrawFlags::RoundCornersBottomRight);
}

ImDrawFlags ImGui::calc_rounding_flags_for_rect_in_rect(const Rect& r_in, const Rect& r_outer, float threshold)
{
    bool round_l = r_in.min.x <= r_outer.min.x + threshold;
    bool round_r = r_in.max.x >= r_outer.max.x - threshold;
    bool round_t = r_in.min.y <= r_outer.min.y + threshold;
    bool round_b = r_in.max.y >= r_outer.max.y - threshold;
    return DrawFlags::RoundCornersNone
        | ((round_t && round_l) ? DrawFlags::RoundCornersTopLeft : 0) | ((round_t && round_r) ? DrawFlags::RoundCornersTopRight : 0)
        | ((round_b && round_l) ? DrawFlags::RoundCornersBottomLeft : 0) | ((round_b && round_r) ? DrawFlags::RoundCornersBottomRight : 0);
}

// Helper for ColorPicker4()
// NB: This is rather brittle and will show artifact when rounding this enabled if rounded corners overlap multiple cells. Caller currently responsible for avoiding that.
// Spent a non reasonable amount of time trying to getting this right for ColorButton with rounding+anti-aliasing+ImGuiColorEditFlags_HalfAlphaPreview flag + various grid sizes and offsets, and eventually gave up... probably more reasonable to disable rounding altogether.
// FIXME: uses ImGui::get_color_u32
void ImGui::RenderColorRectWithAlphaCheckerboard(ImDrawList* draw_list, Vector2D p_min, Vector2D p_max, ImU32 col, float grid_step, Vector2D grid_off, float rounding, ImDrawFlags flags)
{
    if ((flags & DrawFlags::RoundCornersMask_) == 0)
        flags = DrawFlags::RoundCornersDefault_;
    if (((col & IM_COL32_A_MASK) >> IM_COL32_A_SHIFT) < 0xFF)
    {
        ImU32 col_bg1 = get_color_u32(ImAlphaBlendColors(IM_COL32(204, 204, 204, 255), col));
        ImU32 col_bg2 = get_color_u32(ImAlphaBlendColors(IM_COL32(128, 128, 128, 255), col));
        draw_list.add_rect_filled(p_min, p_max, col_bg1, rounding, flags);

        int yi = 0;
        for (let y =  p_min.y + grid_off.y; y < p_max.y; y += grid_step, yi += 1)
        {
            let y1 =  ImClamp(y, p_min.y, p_max.y), y2 = ImMin(y + grid_step, p_max.y);
            if (y2 <= y1)
                continue;
            for (let x =  p_min.x + grid_off.x + (yi & 1) * grid_step; x < p_max.x; x += grid_step * 2.0)
            {
                let x1 =  ImClamp(x, p_min.x, p_max.x), x2 = ImMin(x + grid_step, p_max.x);
                if (x2 <= x1)
                    continue;
                ImDrawFlags cell_flags = DrawFlags::RoundCornersNone;
                if (y1 <= p_min.y) { if (x1 <= p_min.x) cell_flags |= DrawFlags::RoundCornersTopLeft; if (x2 >= p_max.x) cell_flags |= DrawFlags::RoundCornersTopRight; }
                if (y2 >= p_max.y) { if (x1 <= p_min.x) cell_flags |= DrawFlags::RoundCornersBottomLeft; if (x2 >= p_max.x) cell_flags |= DrawFlags::RoundCornersBottomRight; }

                // Combine flags
                cell_flags = (flags == DrawFlags::RoundCornersNone || cell_flags == DrawFlags::RoundCornersNone) ? DrawFlags::RoundCornersNone : (cell_flags & flags);
                draw_list.add_rect_filled(Vector2D::new(x1, y1), Vector2D::new(x2, y2), col_bg2, rounding, cell_flags);
            }
        }
    }
    else
    {
        draw_list.add_rect_filled(p_min, p_max, col, rounding, flags);
    }
}
