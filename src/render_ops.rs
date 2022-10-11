#![allow(non_snake_case)]
//-----------------------------------------------------------------------------
// [SECTION] RENDER HELPERS
// Some of those (internal) functions are currently quite a legacy mess - their signature and behavior will change,
// we need a nicer separation between low-level functions and high-level functions relying on the ImGui context.
// Also see imgui_draw.cpp for some more which have been reworked to not rely on ImGui:: context.
//-----------------------------------------------------------------------------

use std::ptr::null;
use libc::{c_char, c_float, c_int};
use crate::color::ImGuiCol_Text;
use crate::drawlist::ImDrawList;
use crate::font::ImFont;
use crate::imgui::GImGui;
use crate::logging_ops::LogRenderedText;
use crate::rect::ImRect;
use crate::style_ops::GetColorU32;
use crate::vec2::ImVec2;
use crate::vec4::ImVec4;

// *const char ImGui::FindRenderedTextEnd(*const char text, *const char text_end)
pub unsafe fn FindRenderedTextEnd(text: *const c_char, mut text_end: *const c_char) -> *const c_char {
    let mut text_display_end: *const c_char = text;
    if !text_end {
        text_end = null();
    }

    while text_display_end < text_end && *text_display_end != '\0' as c_char && (text_display_end[0] != '#' || text_display_end[1] != '#') {
        text_display_end += 1;
    }
    return text_display_end;
}

// Internal ImGui functions to render text
// RenderText***() functions calls ImDrawList::AddText() calls ImBitmapFont::RenderText()
// c_void ImGui::RenderText(ImVec2 pos, *const char text, *const char text_end, bool hide_text_after_hash)
pub unsafe fn RenderText(pos: ImVec2, text: *const c_char, mut text_end: *const c_char, hide_text_after_hash: bool) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;

    // Hide anything after a '##' string
    let text_display_end: *const c_char;
    if hide_text_after_hash {
        text_display_end = FindRenderedTextEnd(text, text_end);
    } else {
        if !text_end {
            text_end = text + libc::strlen(text);
        } // FIXME-OPT
        text_display_end = text_end;
    }

    if text != text_display_end {
        window.DrawList.AddText2(g.Font, g.FontSize, &pos, GetColorU32(ImGuiCol_Text, 0f32), text, text_display_end, 0f32, null());
        if g.LogEnabled {
            LogRenderedText(&pos, text, text_display_end);
        }
    }
}

// c_void ImGui::RenderTextWrapped(ImVec2 pos, *const char text, *const char text_end, c_float wrap_width)
pub unsafe fn RenderTextWrapped(pos: ImVec2, text: *const c_char, mut text_end: *const c_char)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;

    if !text_end {
        text_end = text + libc::strlen(text);
    } // FIXME-OPT

    if text != text_end
    {
        window.DrawList.AddText2(g.Font, g.FontSize, &pos, GetColorU32(ImGuiCol_Text, 0f32), text, text_end, wrap_width, null());
        if g.LogEnabled {
            LogRenderedText(&pos, text, text_end);
        }
    }
}

// Default clip_rect uses (pos_min,pos_max)
// Handle clipping on CPU immediately (vs typically let the GPU clip the triangles that are overlapping the clipping rectangle edges)
// c_void ImGui::RenderTextClippedEx(ImDrawList* draw_list, const ImVec2& pos_min, const ImVec2& pos_max, *const char text, *const char text_display_end, *const ImVec2 text_size_if_known, const ImVec2& align, *const ImRect clip_rect)
pub unsafe fn RenderTextClippedEx(mut draw_list: *mut ImDrawList, pos_min: &ImVec2, pos_max: &ImVec2, text: *const c_char, text_display_end: *const c_char, text_size_if_known: *const ImVec2, align: &ImVec2, clip_rect: *const ImRect) {
    // Perform CPU side clipping for single clipped element to avoid using scissor state
    let mut pos: ImVec2 = pos_min.clone();
    let text_size = if text_size_if_known { text_size_if_known.clone() } else { CalcTextSize(text, text_display_end, false, 0f32) };

    let clip_min: *const ImVec2 = if clip_rect { &clip_rect.Min } else { &pos_min };
    clip_max: *const ImVec2 = if clip_rect { &clip_rect.Max } else { &pos_max };
    let mut need_clipping: bool = (pos.x + text_size.x >= clip_max.x) || (pos.y + text_size.y >= clip_max.y);
    if clip_rect { // If we had no explicit clipping rectangle then pos==clip_min
        need_clipping |= (pos.x < clip_min.x) || (pos.y < clip_min.y);
    }

    // Align whole block. We should defer that to the better rendering function when we'll have support for individual line alignment.
    if align.x > 0f32 {
        pos.x = ImMax(pos.x, pos.x + (pos_max.x - pos.x - text_size.x) * align.x);
    }
    if align.y > 0f32 {
        pos.y = ImMax(pos.y, pos.y + (pos_max.y - pos.y - text_size.y) * align.y);
    }

    // Render
    if need_clipping {
        let mut fine_clip_rect = ImVec4::new2(clip_min.x, clip_min.y, clip_max.x, clip_max.y);
        draw_list.AddText2(null(), 0f32, &pos, GetColorU32(ImGuiCol_Text, 0f32), text, text_display_end, 0f32, &fine_clip_rect);
    } else {
        draw_list.AddText2(NULL, 0f32, &pos, GetColorU32(ImGuiCol_Text, 0f32), text, text_display_end, 0f32, NULL);
    }
}

// c_void ImGui::RenderTextClipped(const ImVec2& pos_min, const ImVec2& pos_max, *const char text, *const char text_end, *const ImVec2 text_size_if_known, const ImVec2& align, *const ImRect clip_rect)
pub unsafe fn RenderTextClipped(pos_min: &ImVec2, pos_max: &ImVec2, text: *const c_char, text_end: *const c_char, text_size_if_known: *const ImVec2, align: &ImVec2, clip_rect: *const ImRect)
{
    // Hide anything after a '##' string
    let mut  text_display_end: *const c_char = FindRenderedTextEnd(text, text_end);
    let text_len: c_int = (text_display_end - text);
    if text_len == 0 {
        return;
    }

    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    RenderTextClippedEx(window.DrawList, pos_min, pos_max, text, text_display_end, text_size_if_known, align, clip_rect);
    if g.LogEnabled {
        LogRenderedText(pos_min, text, text_display_end);
    }
}


// Another overly complex function until we reorganize everything into a nice all-in-one helper.
// This is made more complex because we have dissociated the layout rectangle (pos_min..pos_max) which define _where_ the ellipsis is, from actual clipping of text and limit of the ellipsis display.
// This is because in the context of tabs we selectively hide part of the text when the Close Button appears, but we don't want the ellipsis to move.
// c_void ImGui::RenderTextEllipsis(ImDrawList* draw_list, const ImVec2& pos_min, const ImVec2& pos_max, c_float clip_max_x, c_float ellipsis_max_x, *const char text, *const char text_end_full, *const ImVec2 text_size_if_known)
pub unsafe fn RenderTextEllipsis(draw_list: *mut ImDrawList, pos_min: &ImVec2, pos_max: &ImVec2, clip_max_x: c_float, ellipsis_max_x: c_float, text: *const c_char, mut text_end_full: *const c_char, text_size_if_known: *const ImVec2)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if text_end_full.is_null() {
        text_end_full = FindRenderedTextEnd(text, null());
    }
    let text_size: ImVec2 =  if text_size_if_known { text_size_if_known.clone() } else { CalcTextSize(text, text_end_full, false, 0f32) };

    //draw_list->AddLine(ImVec2(pos_max.x, pos_min.y - 4), ImVec2(pos_max.x, pos_max.y + 4), IM_COL32(0, 0, 255, 255));
    //draw_list->AddLine(ImVec2(ellipsis_max_x, pos_min.y-2), ImVec2(ellipsis_max_x, pos_max.y+2), IM_COL32(0, 255, 0, 255));
    //draw_list->AddLine(ImVec2(clip_max_x, pos_min.y), ImVec2(clip_max_x, pos_max.y), IM_COL32(255, 0, 0, 255));
    // FIXME: We could technically remove (last_glyph->AdvanceX - last_glyph->X1) from text_size.x here and save a few pixels.
    if text_size.x > pos_max.x - pos_min.x
    {
        // Hello wo...
        // |       |   |
        // min   max   ellipsis_max
        //          <-> this is generally some padding value

        let font: *const ImFont = draw_list._Data.Font;
        let         : c_float =  draw_list._Data.FontSize;
        let mut  text_end_ellipsis: *const c_char = None;

        ImWchar ellipsis_char = font.EllipsisChar;
        c_int ellipsis_char_count = 1;
        if (ellipsis_char == -1)
        {
            ellipsis_char = font.DotChar;
            ellipsis_char_count = 3;
        }
        glyph: *const ImFontGlyph = font.FindGlyph(ellipsis_char);

        c_float ellipsis_glyph_width = glyph.X1;                 // Width of the glyph with no padding on either side
        c_float ellipsis_total_width = ellipsis_glyph_width;      // Full width of entire ellipsis

        if (ellipsis_char_count > 1)
        {
            // Full ellipsis size without free spacing after it.
            let             : c_float =  1f32 * (draw_list._Data.FontSize / font.FontSize);
            ellipsis_glyph_width = glyph.X1 - glyph.X0 + spacing_between_dots;
            ellipsis_total_width = ellipsis_glyph_width * ellipsis_char_count - spacing_between_dots;
        }

        // We can now claim the space between pos_max.x and ellipsis_max.x
        let         : c_float =  ImMax((ImMax(pos_max.x, ellipsis_max_x) - ellipsis_total_width) - pos_min.x, 1f32);
        c_float text_size_clipped_x = font.CalcTextSizeA(font_size, text_avail_width, 0f32, text, text_end_full, &text_end_ellipsis).x;
        if (text == text_end_ellipsis && text_end_ellipsis < text_end_full)
        {
            // Always display at least 1 character if there's no room for character + ellipsis
            text_end_ellipsis = text + ImTextCountUtf8BytesFromChar(text, text_end_full);
            text_size_clipped_x = font.CalcTextSizeA(font_size, f32::MAX, 0f32, text, text_end_ellipsis).x;
        }
        while (text_end_ellipsis > text && ImCharIsBlankA(text_end_ellipsis[-1]))
        {
            // Trim trailing space before ellipsis (FIXME: Supporting non-ascii blanks would be nice, for this we need a function to backtrack in UTF-8 text)
            text_end_ellipsis-= 1;
            text_size_clipped_x -= font.CalcTextSizeA(font_size, f32::MAX, 0f32, text_end_ellipsis, text_end_ellipsis + 1).x; // Ascii blanks are always 1 byte
        }

        // Render text, render ellipsis
        RenderTextClippedEx(draw_list, pos_min, ImVec2(clip_max_x, pos_max.y), text, text_end_ellipsis, &text_size, ImVec2(0f32, 0f32));
        c_float ellipsis_x = pos_min.x + text_size_clipped_x;
        if (ellipsis_x + ellipsis_total_width <= ellipsis_max_x)
            for (c_int i = 0; i < ellipsis_char_count; i++)
            {
                font.RenderChar(draw_list, font_size, ImVec2(ellipsis_x, pos_min.y), GetColorU32(ImGuiCol_Text), ellipsis_char);
                ellipsis_x += ellipsis_glyph_width;
            }
    }
    else
    {
        RenderTextClippedEx(draw_list, pos_min, ImVec2(clip_max_x, pos_max.y), text, text_end_full, &text_size, ImVec2(0f32, 0f32));
    }

    if (g.LogEnabled)
        LogRenderedText(&pos_min, text, text_end_full);
}

// Render a rectangle shaped with optional rounding and borders
c_void ImGui::RenderFrame(ImVec2 p_min, ImVec2 p_max, u32 fill_col, bool border, c_float rounding)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    window.DrawList.AddRectFilled(p_min, p_max, fill_col, rounding);
    let     : c_float =  g.Style.FrameBorderSize;
    if (border && border_size > 0f32)
    {
        window.DrawList.AddRect(p_min + ImVec2(1, 1), p_max + ImVec2(1, 1), GetColorU32(ImGuiCol_BorderShadow), rounding, 0, border_size);
        window.DrawList.AddRect(p_min, p_max, GetColorU32(ImGuiCol_Border), rounding, 0, border_size);
    }
}

c_void ImGui::RenderFrameBorder(ImVec2 p_min, ImVec2 p_max, c_float rounding)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    let     : c_float =  g.Style.FrameBorderSize;
    if (border_size > 0f32)
    {
        window.DrawList.AddRect(p_min + ImVec2(1, 1), p_max + ImVec2(1, 1), GetColorU32(ImGuiCol_BorderShadow), rounding, 0, border_size);
        window.DrawList.AddRect(p_min, p_max, GetColorU32(ImGuiCol_Border), rounding, 0, border_size);
    }
}

c_void ImGui::RenderNavHighlight(const ImRect& bb, ImGuiID id, ImGuiNavHighlightFlags flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (id != g.NavId)
        return;
    if (g.NavDisableHighlight && !(flags & ImGuiNavHighlightFlags_AlwaysDraw))
        return;
    let mut window = g.CurrentWindow;
    if (window.DC.NavHideHighlightOneFrame)
        return;

    c_float rounding = (flags & ImGuiNavHighlightFlags_NoRounding) ? 0f32 : g.Style.FrameRounding;
    ImRect display_rect = bb;
    display_rect.ClipWith(window.ClipRect);
    if (flags & ImGuiNavHighlightFlags_TypeDefault)
    {
        let         : c_float =  2.0f32;
        let         : c_float =  3.0f32 + THICKNESS * 0.5f32;
        display_rect.Expand(ImVec2(DISTANCE, DISTANCE));
        let mut fully_visible: bool =  window.ClipRect.Contains(display_rect);
        if (!fully_visible)
            window.DrawList.PushClipRect(display_rect.Min, display_rect.Max);
        window.DrawList.AddRect(display_rect.Min + ImVec2(THICKNESS * 0.5f32, THICKNESS * 0.5f32), display_rect.Max - ImVec2(THICKNESS * 0.5f32, THICKNESS * 0.5f32), GetColorU32(ImGuiCol_NavHighlight), rounding, 0, THICKNESS);
        if (!fully_visible)
            window.DrawList.PopClipRect();
    }
    if (flags & ImGuiNavHighlightFlags_TypeThin)
    {
        window.DrawList.AddRect(display_rect.Min, display_rect.Max, GetColorU32(ImGuiCol_NavHighlight), rounding, 0, 1f32);
    }
}

c_void ImGui::RenderMouseCursor(ImVec2 base_pos, c_float base_scale, ImGuiMouseCursor mouse_cursor, u32 col_fill, u32 col_border, u32 col_shadow)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(mouse_cursor > ImGuiMouseCursor_None && mouse_cursor < ImGuiMouseCursor_COUNT);
    ImFontAtlas* font_atlas = g.DrawListSharedData.Font.ContainerAtlas;
    for (c_int n = 0; n < g.Viewports.Size; n++)
    {
        // We scale cursor with current viewport/monitor, however Windows 10 for its own hardware cursor seems to be using a different scale factor.
        ImVec2 offset, size, uv[4];
        if (!font_atlas.GetMouseCursorTexData(mouse_cursor, &offset, &size, &uv[0], &uv[2]))
            continue;
        *mut ImGuiViewportP viewport = g.Viewports[n];
        let pos: ImVec2 =  base_pos - offset;
        let         : c_float =  base_scale * viewport.DpiScale;
        if (!viewport.GetMainRect().Overlaps(ImRect(pos, pos + ImVec2(size.x + 2, size.y + 2) * scale)))
            continue;
        ImDrawList* draw_list = GetForegroundDrawList(viewport);
        ImTextureID tex_id = font_atlas.TexID;
        draw_list.PushTextureID(tex_id);
        draw_list.AddImage(tex_id, pos + ImVec2(1, 0) * scale, pos + (ImVec2(1, 0) + size) * scale, uv[2], uv[3], col_shadow);
        draw_list.AddImage(tex_id, pos + ImVec2(2, 0) * scale, pos + (ImVec2(2, 0) + size) * scale, uv[2], uv[3], col_shadow);
        draw_list.AddImage(tex_id, pos,                        pos + size * scale,                  uv[2], uv[3], col_border);
        draw_list.AddImage(tex_id, pos,                        pos + size * scale,                  uv[0], uv[1], col_fill);
        draw_list.PopTextureID();
    }
}
