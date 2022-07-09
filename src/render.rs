use std::os::raw::c_char;
use std::ptr::null_mut;
use crate::imgui_globals::GImGui;
use crate::imgui_h::ImGuiColor;
use crate::imgui_style::{GetColorU32, GetColorU32_2, GetColorU32_3};

// const char* ImGui::FindRenderedTextEnd(const char* text, const char* text_end)
pub unsafe fn FindRenderedTextEnd(text: *const c_char, text_end: *const c_char) -> *const c_char {
    // const char* text_display_end = text;
    let mut text_display_end = text;
    // if text_end.is_null() {
    //     text_end = -1;
    // }

    while text_display_end < text_end && *text_display_end != '\0' as c_char && (text_display_end[0] != '#' || text_display_end[1] != '#') {
        text_display_end += 1;
    }
    return text_display_end;
}

// Internal ImGui functions to render text
// render_text***() functions calls ImDrawList::add_text() calls ImBitmapFont::render_text()
// void ImGui::render_text(ImVec2 pos, const char* text, const char* text_end, bool hide_text_after_hash)
pub unsafe fn RenderText(pos: &ImVec2, text: *const c_char, mut text_end: *const c_char, hide_text_after_hash: bool)
{
    // ImGuiContext& g = *GImGui;
    let g = GImGui;
    // ImGuiWindow* window = g.current_window;
    let window = g.CurrentWindow;

    // Hide anything after a '##' string
    // const char* text_display_end;
    let mut text_display_end: *const c_char = null_mut();
    if hide_text_after_hash
    {
        text_display_end = FindRenderedTextEnd(text, text_end);
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
        window.DrawList.AddText2(&g.Font, g.FontSize, pos, GetColorU32_3(ImGuiColor::Text as u32), text, text_display_end, 0.0, None);
        if g.LogEnabled {
            LogRenderedText(&pos, text, text_display_end);
        }
    }
}

// void ImGui::RenderTextWrapped(ImVec2 pos, const char* text, const char* text_end, float wrap_width)
pub fn RenderTextWrapped(pos: &ImVec2, text: *const c_char, mut text_end: *const c_char, wrap_width: f32)
{
    // ImGuiContext& g = *GImGui;
    let g = GImGui;
    // ImGuiWindow* window = g.current_window;
    let window = g.CurrentWindow;

    if !text_end {
        text_end = text + strlen(text);
    } // FIXME-OPT

    if text != text_end
    {
        window.DrawList.AddText2(&g.Font, g.FontSize, pos, GetColorU32_3(ImGuiCol_Text), text, text_end, wrap_width);
        if g.LogEnabled {
            LogRenderedText(&pos, text, text_end);
        }
    }
}

// Default clip_rect uses (pos_min,pos_max)
// Handle clipping on CPU immediately (vs typically let the GPU clip the triangles that are overlapping the clipping rectangle edges)
void ImGui::RenderTextClippedEx(ImDrawList* draw_list, const ImVec2& pos_min, const ImVec2& pos_max, const char* text, const char* text_display_end, const ImVec2* text_size_if_known, const ImVec2& align, const ImRect* clip_rect)
{
    // Perform CPU side clipping for single clipped element to avoid using scissor state
    ImVec2 pos = pos_min;
    const ImVec2 text_size = text_size_if_known ? *text_size_if_known : CalcTextSize(text, text_display_end, false, 0.0);

    const ImVec2* clip_min = clip_rect ? &clip_rect->Min : &pos_min;
    const ImVec2* clip_max = clip_rect ? &clip_rect->Max : &pos_max;
    bool need_clipping = (pos.x + text_size.x >= clip_max->x) || (pos.y + text_size.y >= clip_max->y);
    if (clip_rect) // If we had no explicit clipping rectangle then pos==clip_min
        need_clipping |= (pos.x < clip_min->x) || (pos.y < clip_min->y);

    // Align whole block. We should defer that to the better rendering function when we'll have support for individual line alignment.
    if (align.x > 0.0) pos.x = ImMax(pos.x, pos.x + (pos_max.x - pos.x - text_size.x) * align.x);
    if (align.y > 0.0) pos.y = ImMax(pos.y, pos.y + (pos_max.y - pos.y - text_size.y) * align.y);

    // Render
    if (need_clipping)
    {
        ImVec4 fine_clip_rect(clip_min->x, clip_min->y, clip_max->x, clip_max->y);
        draw_list->AddText(NULL, 0.0, pos, GetColorU32(ImGuiCol_Text), text, text_display_end, 0.0, &fine_clip_rect);
    }
    else
    {
        draw_list->AddText(NULL, 0.0, pos, GetColorU32(ImGuiCol_Text), text, text_display_end, 0.0, NULL);
    }
}

void ImGui::RenderTextClipped(const ImVec2& pos_min, const ImVec2& pos_max, const char* text, const char* text_end, const ImVec2* text_size_if_known, const ImVec2& align, const ImRect* clip_rect)
{
    // Hide anything after a '##' string
    const char* text_display_end = FindRenderedTextEnd(text, text_end);
    const int text_len = (text_display_end - text);
    if (text_len == 0)
        return;

    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;
    RenderTextClippedEx(window.DrawList, pos_min, pos_max, text, text_display_end, text_size_if_known, align, clip_rect);
    if (g.LogEnabled)
        LogRenderedText(&pos_min, text, text_display_end);
}


// Another overly complex function until we reorganize everything into a nice all-in-one helper.
// This is made more complex because we have dissociated the layout rectangle (pos_min..pos_max) which define _where_ the ellipsis is, from actual clipping of text and limit of the ellipsis display.
// This is because in the context of tabs we selectively hide part of the text when the Close Button appears, but we don't want the ellipsis to move.
void ImGui::RenderTextEllipsis(ImDrawList* draw_list, const ImVec2& pos_min, const ImVec2& pos_max, float clip_max_x, float ellipsis_max_x, const char* text, const char* text_end_full, const ImVec2* text_size_if_known)
{
    ImGuiContext& g = *GImGui;
    if (text_end_full == NULL)
        text_end_full = FindRenderedTextEnd(text);
    const ImVec2 text_size = text_size_if_known ? *text_size_if_known : CalcTextSize(text, text_end_full, false, 0.0);

    //draw_list->AddLine(ImVec2(pos_max.x, pos_min.y - 4), ImVec2(pos_max.x, pos_max.y + 4), IM_COL32(0, 0, 255, 255));
    //draw_list->AddLine(ImVec2(ellipsis_max_x, pos_min.y-2), ImVec2(ellipsis_max_x, pos_max.y+2), IM_COL32(0, 255, 0, 255));
    //draw_list->AddLine(ImVec2(clip_max_x, pos_min.y), ImVec2(clip_max_x, pos_max.y), IM_COL32(255, 0, 0, 255));
    // FIXME: We could technically remove (last_glyph->advance_x - last_glyph->x1) from text_size.x here and save a few pixels.
    if (text_size.x > pos_max.x - pos_min.x)
    {
        // Hello wo...
        // |       |   |
        // min   max   ellipsis_max
        //          <-> this is generally some padding value

        const ImFont* font = draw_list->_Data->Font;
        const float font_size = draw_list->_Data->FontSize;
        const char* text_end_ellipsis = NULL;

        ImWchar ellipsis_char = font->EllipsisChar;
        int ellipsis_char_count = 1;
        if (ellipsis_char == (ImWchar)-1)
        {
            ellipsis_char = font->DotChar;
            ellipsis_char_count = 3;
        }
        const ImFontGlyph* glyph = font->FindGlyph(ellipsis_char);

        float ellipsis_glyph_width = glyph->X1;                 // width of the glyph with no padding on either side
        float ellipsis_total_width = ellipsis_glyph_width;      // Full width of entire ellipsis

        if (ellipsis_char_count > 1)
        {
            // Full ellipsis size without free spacing after it.
            const float spacing_between_dots = 1.0 * (draw_list->_Data->FontSize / font->FontSize);
            ellipsis_glyph_width = glyph->X1 - glyph->X0 + spacing_between_dots;
            ellipsis_total_width = ellipsis_glyph_width * (float)ellipsis_char_count - spacing_between_dots;
        }

        // We can now claim the space between pos_max.x and ellipsis_max.x
        const float text_avail_width = ImMax((ImMax(pos_max.x, ellipsis_max_x) - ellipsis_total_width) - pos_min.x, 1.0);
        float text_size_clipped_x = font->CalcTextSizeA(font_size, text_avail_width, 0.0, text, text_end_full, &text_end_ellipsis).x;
        if (text == text_end_ellipsis && text_end_ellipsis < text_end_full)
        {
            // Always display at least 1 character if there's no room for character + ellipsis
            text_end_ellipsis = text + ImTextCountUtf8BytesFromChar(text, text_end_full);
            text_size_clipped_x = font->CalcTextSizeA(font_size, FLT_MAX, 0.0, text, text_end_ellipsis).x;
        }
        while (text_end_ellipsis > text && ImCharIsBlankA(text_end_ellipsis[-1]))
        {
            // Trim trailing space before ellipsis (FIXME: Supporting non-ascii blanks would be nice, for this we need a function to backtrack in UTF-8 text)
            text_end_ellipsis--;
            text_size_clipped_x -= font->CalcTextSizeA(font_size, FLT_MAX, 0.0, text_end_ellipsis, text_end_ellipsis + 1).x; // Ascii blanks are always 1 byte
        }

        // Render text, render ellipsis
        RenderTextClippedEx(draw_list, pos_min, DimgVec2D::new(clip_max_x, pos_max.y), text, text_end_ellipsis, &text_size, DimgVec2D::new(0.0, 0.0));
        float ellipsis_x = pos_min.x + text_size_clipped_x;
        if (ellipsis_x + ellipsis_total_width <= ellipsis_max_x)
            for (int i = 0; i < ellipsis_char_count; i += 1)
            {
                font->RenderChar(draw_list, font_size, DimgVec2D::new(ellipsis_x, pos_min.y), GetColorU32(ImGuiCol_Text), ellipsis_char);
                ellipsis_x += ellipsis_glyph_width;
            }
    }
    else
    {
        RenderTextClippedEx(draw_list, pos_min, DimgVec2D::new(clip_max_x, pos_max.y), text, text_end_full, &text_size, DimgVec2D::new(0.0, 0.0));
    }

    if (g.LogEnabled)
        LogRenderedText(&pos_min, text, text_end_full);
}

// Render a rectangle shaped with optional rounding and borders
void ImGui::RenderFrame(ImVec2 p_min, ImVec2 p_max, ImU32 fill_col, bool border, float rounding)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;
    window.DrawList->AddRectFilled(p_min, p_max, fill_col, rounding);
    const float border_size = g.Style.FrameBorderSize;
    if (border && border_size > 0.0)
    {
        window.DrawList->AddRect(p_min + DimgVec2D::new(1, 1), p_max + DimgVec2D::new(1, 1), GetColorU32(ImGuiCol_BorderShadow), rounding, 0, border_size);
        window.DrawList->AddRect(p_min, p_max, GetColorU32(ImGuiCol_Border), rounding, 0, border_size);
    }
}

void ImGui::RenderFrameBorder(ImVec2 p_min, ImVec2 p_max, float rounding)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.CurrentWindow;
    const float border_size = g.Style.FrameBorderSize;
    if (border_size > 0.0)
    {
        window.DrawList->AddRect(p_min + DimgVec2D::new(1, 1), p_max + DimgVec2D::new(1, 1), GetColorU32(ImGuiCol_BorderShadow), rounding, 0, border_size);
        window.DrawList->AddRect(p_min, p_max, GetColorU32(ImGuiCol_Border), rounding, 0, border_size);
    }
}

void ImGui::RenderNavHighlight(const ImRect& bb, ImGuiID id, ImGuiNavHighlightFlags flags)
{
    ImGuiContext& g = *GImGui;
    if (id != g.NavId)
        return;
    if (g.NavDisableHighlight && !(flags & ImGuiNavHighlightFlags_AlwaysDraw))
        return;
    ImGuiWindow* window = g.CurrentWindow;
    if (window.DC.NavHideHighlightOneFrame)
        return;

    float rounding = (flags & ImGuiNavHighlightFlags_NoRounding) ? 0.0 : g.Style.FrameRounding;
    ImRect display_rect = bb;
    display_rect.ClipWith(window.ClipRect);
    if (flags & ImGuiNavHighlightFlags_TypeDefault)
    {
        const float THICKNESS = 2.0;
        const float DISTANCE = 3.0 + THICKNESS * 0.5;
        display_rect.Expand(DimgVec2D::new(DISTANCE, DISTANCE));
        bool fully_visible = window.ClipRect.Contains(display_rect);
        if (!fully_visible)
            window.DrawList->PushClipRect(display_rect.Min, display_rect.Max);
        window.DrawList->AddRect(display_rect.Min + DimgVec2D::new(THICKNESS * 0.5, THICKNESS * 0.5), display_rect.Max - DimgVec2D::new(THICKNESS * 0.5, THICKNESS * 0.5), GetColorU32(ImGuiCol_NavHighlight), rounding, 0, THICKNESS);
        if (!fully_visible)
            window.DrawList->PopClipRect();
    }
    if (flags & ImGuiNavHighlightFlags_TypeThin)
    {
        window.DrawList->AddRect(display_rect.Min, display_rect.Max, GetColorU32(ImGuiCol_NavHighlight), rounding, 0, 1.0);
    }
}

void ImGui::RenderMouseCursor(ImVec2 base_pos, float base_scale, ImGuiMouseCursor mouse_cursor, ImU32 col_fill, ImU32 col_border, ImU32 col_shadow)
{
    ImGuiContext& g = *GImGui;
    IM_ASSERT(mouse_cursor > ImGuiMouseCursor_None && mouse_cursor < ImGuiMouseCursor_COUNT);
    ImFontAtlas* font_atlas = g.DrawListSharedData.Font->ContainerAtlas;
    for (int n = 0; n < g.Viewports.Size; n += 1)
    {
        // We scale cursor with current viewport/monitor, however windows 10 for its own hardware cursor seems to be using a different scale factor.
        ImVec2 offset, size, uv[4];
        if (!font_atlas->GetMouseCursorTexData(mouse_cursor, &offset, &size, &uv[0], &uv[2]))
            continue;
        ImGuiViewportP* viewport = g.Viewports[n];
        const ImVec2 pos = base_pos - offset;
        const float scale = base_scale * viewport->DpiScale;
        if (!viewport->GetMainRect().Overlaps(ImRect(pos, pos + DimgVec2D::new(size.x + 2, size.y + 2) * scale)))
            continue;
        ImDrawList* draw_list = GetForegroundDrawList(viewport);
        ImTextureID tex_id = font_atlas->TexID;
        draw_list->PushTextureID(tex_id);
        draw_list->AddImage(tex_id, pos + DimgVec2D::new(1, 0) * scale, pos + (DimgVec2D::new(1, 0) + size) * scale, uv[2], uv[3], col_shadow);
        draw_list->AddImage(tex_id, pos + DimgVec2D::new(2, 0) * scale, pos + (DimgVec2D::new(2, 0) + size) * scale, uv[2], uv[3], col_shadow);
        draw_list->AddImage(tex_id, pos,                        pos + size * scale,                  uv[2], uv[3], col_border);
        draw_list->AddImage(tex_id, pos,                        pos + size * scale,                  uv[0], uv[1], col_fill);
        draw_list->PopTextureID();
    }
}
