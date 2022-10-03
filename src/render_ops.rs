#![allow(non_snake_case)]
//-----------------------------------------------------------------------------
// [SECTION] RENDER HELPERS
// Some of those (internal) functions are currently quite a legacy mess - their signature and behavior will change,
// we need a nicer separation between low-level functions and high-level functions relying on the ImGui context.
// Also see imgui_draw.cpp for some more which have been reworked to not rely on  context.
//-----------------------------------------------------------------------------

use std::ptr::{null, null_mut};
use libc::{c_char, c_float, c_int};
use crate::CallContextHooks;
use crate::color::{IM_COL32, IM_COL32_BLACK, IM_COL32_WHITE, ImGuiCol_Border, ImGuiCol_BorderShadow, ImGuiCol_NavHighlight, ImGuiCol_Text};
use crate::context_hook::{ImGuiContextHookType_RenderPost, ImGuiContextHookType_RenderPre};
use crate::draw_data::ImDrawData;
use crate::draw_data_ops::{AddDrawListToDrawData, AddRootWindowToDrawData};
use crate::draw_flags::ImDrawFlags_None;
use crate::draw_list::ImDrawList;
use crate::draw_list_ops::{GetBackgroundDrawList, GetForegroundDrawList};
use crate::font::ImFont;
use crate::font_atlas::ImFontAtlas;
use crate::font_glyph::ImFontGlyph;
use crate::frame_ops::EndFrame;
use crate::imgui::GImGui;
use crate::logging_ops::LogRenderedText;
use crate::mouse_cursor::{ImGuiMouseCursor, ImGuiMouseCursor_None};
use crate::nav_highlight_flags::{ImGuiNavHighlightFlags, ImGuiNavHighlightFlags_AlwaysDraw, ImGuiNavHighlightFlags_NoRounding, ImGuiNavHighlightFlags_TypeDefault, ImGuiNavHighlightFlags_TypeThin};
use crate::rect::ImRect;
use crate::string_ops::{ImTextCountUtf8BytesFromChar, ImTextCountUtf8BytesFromChar2, ImTextCountUtf8BytesFromStr};
use crate::style_ops::GetColorU32;
use crate::text_ops::CalcTextSize;
use crate::type_defs::{ImGuiID, ImTextureID, ImWchar};
use crate::utils::flag_set;
use crate::vec2::ImVec2;
use crate::vec4::ImVec4;
use crate::viewport::ImGuiViewport;
use crate::viewport_ops::SetupViewportDrawData;
use crate::window::ImGuiWindow;
use crate::window_flags::{ImGuiWindowFlags_ChildWindow, ImGuiWindowFlags_NoBringToFrontOnFocus};
use crate::window_ops::{IsWindowActiveAndVisible, RenderDimmedBackgrounds};

// *const char FindRenderedTextEnd(*const char text, *const char text_end)
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
// c_void RenderText(ImVec2 pos, *const char text, *const char text_end, hide_text_after_hash: bool)
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
        window.DrawList.AddText2(g.Font, g.FontSize, &pos, GetColorU32(ImGuiCol_Text, 0f32, ), text, text_display_end, 0f32, null());
        if g.LogEnabled {
            LogRenderedText(&pos, text, text_display_end);
        }
    }
}

// c_void RenderTextWrapped(ImVec2 pos, *const char text, *const char text_end, c_float wrap_width)
pub unsafe fn RenderTextWrapped(pos: ImVec2, text: *const c_char, mut text_end: *const c_char)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;

    if !text_end {
        text_end = text + libc::strlen(text);
    } // FIXME-OPT

    if text != text_end
    {
        window.DrawList.AddText2(g.Font, g.FontSize, &pos, GetColorU32(ImGuiCol_Text, 0f32, ), text, text_end, wrap_width, null());
        if g.LogEnabled {
            LogRenderedText(&pos, text, text_end);
        }
    }
}

// Default clip_rect uses (pos_min,pos_max)
// Handle clipping on CPU immediately (vs typically let the GPU clip the triangles that are overlapping the clipping rectangle edges)
// c_void RenderTextClippedEx(ImDrawList* draw_list, const ImVec2& pos_min, const ImVec2& pos_max, *const char text, *const char text_display_end, *const ImVec2 text_size_if_known, const ImVec2& align, *const ImRect clip_rect)
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
        draw_list.AddText2(null(), 0f32, &pos, GetColorU32(ImGuiCol_Text, 0f32, ), text, text_display_end, 0f32, &fine_clip_rect);
    } else {
        draw_list.AddText2(null_mut(), 0f32, &pos, GetColorU32(ImGuiCol_Text, 0f32, ), text, text_display_end, 0f32, null_mut());
    }
}

// c_void RenderTextClipped(const ImVec2& pos_min, const ImVec2& pos_max, *const char text, *const char text_end, *const ImVec2 text_size_if_known, const ImVec2& align, *const ImRect clip_rect)
pub unsafe fn RenderTextClipped(pos_min: &ImVec2, pos_max: &ImVec2, text: *const c_char, text_end: *const c_char, text_size_if_known: *const ImVec2, align: &ImVec2, clip_rect: *const ImRect) {
    // Hide anything after a '##' string
    let mut text_display_end: *const c_char = FindRenderedTextEnd(text, text_end);
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
// c_void RenderTextEllipsis(ImDrawList* draw_list, const ImVec2& pos_min, const ImVec2& pos_max, c_float clip_max_x, c_float ellipsis_max_x, *const char text, *const char text_end_full, *const ImVec2 text_size_if_known)
pub unsafe fn RenderTextEllipsis(draw_list: *mut ImDrawList, pos_min: &ImVec2, pos_max: &ImVec2, clip_max_x: c_float, ellipsis_max_x: c_float, text: *const c_char, mut text_end_full: *const c_char, text_size_if_known: *const ImVec2) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if text_end_full == null() {
        text_end_full = FindRenderedTextEnd(text, null());
    }
    let text_size: ImVec2 = if text_size_if_known { (*text_size_if_known).clone() } else { CalcTextSize(text, text_end_full, false, 0f32) };

    //draw_list.AddLine(ImVec2(pos_max.x, pos_min.y - 4), ImVec2(pos_max.x, pos_max.y + 4), IM_COL32(0, 0, 255, 255));
    //draw_list.AddLine(ImVec2(ellipsis_max_x, pos_min.y-2), ImVec2(ellipsis_max_x, pos_max.y+2), IM_COL32(0, 255, 0, 255));
    //draw_list.AddLine(ImVec2(clip_max_x, pos_min.y), ImVec2(clip_max_x, pos_max.y), IM_COL32(255, 0, 0, 255));
    // FIXME: We could technically remove (last_glyph.AdvanceX - last_glyph.X1) from text_size.x here and save a few pixels.
    if text_size.x > pos_max.x - pos_min.x {
        // Hello wo...
        // |       |   |
        // min   max   ellipsis_max
        //          <-> this is generally some padding value

        let mut font: *const ImFont = draw_list._Data.Font;
        let font_size: c_float = draw_list._Data.FontSize;
        let mut text_end_ellipsis: *const c_char = null();

        let mut ellipsis_char: ImWchar = font.EllipsisChar;
        let mut ellipsis_char_count: c_int = 1;
        if ellipsis_char == -1 {
            ellipsis_char = font.DotChar;
            ellipsis_char_count = 3;
        }
        let glyph: *const ImFontGlyph = font.FindGlyph(ellipsis_char);

        let mut ellipsis_glyph_width: c_float = glyph.X1;                 // Width of the glyph with no padding on either side
        let mut ellipsis_total_width: c_float = ellipsis_glyph_width;      // Full width of entire ellipsis

        if ellipsis_char_count > 1 {
            // Full ellipsis size without free spacing after it.
            let spacing_between_dots: c_float = 1f32 * (draw_list._Data.FontSize / font.FontSize);
            ellipsis_glyph_width = glyph.X1 - glyph.X0 + spacing_between_dots;
            ellipsis_total_width = ellipsis_glyph_width * ellipsis_char_count - spacing_between_dots;
        }

        // We can now claim the space between pos_max.x and ellipsis_max.x
        let text_avail_width: c_float = ImMax((ImMax(pos_max.x, ellipsis_max_x) - ellipsis_total_width) - pos_min.x, 1f32);
        let mut text_size_clipped_x: c_float = font.CalcTextSizeA(font_size, text_avail_width, 0f32, text, text_end_full, &mut text_end_ellipsis).x;
        if text == text_end_ellipsis && text_end_ellipsis < text_end_full {
            // Always display at least 1 character if there's no room for character + ellipsis
            text_end_ellipsis = text + ImTextCountUtf8BytesFromChar(text, text_end_full);
            text_size_clipped_x = font.CalcTextSizeA(font_size, f32::MAX, 0f32, text, text_end_ellipsis, null_mut()).x;
        }
        while text_end_ellipsis > text && ImCharIsBlankA(text_end_ellipsis[-1]) {
            // Trim trailing space before ellipsis (FIXME: Supporting non-ascii blanks would be nice, for this we need a function to backtrack in UTF-8 text)
            text_end_ellipsis -= 1;
            text_size_clipped_x -= font.CalcTextSizeA(font_size, f32::MAX, 0f32, text_end_ellipsis, text_end_ellipsis + 1, null_mut()).x; // Ascii blanks are always 1 byte
        }

        // Render text, render ellipsis
        RenderTextClippedEx(draw_list, pos_min, &ImVec2::new2(clip_max_x, pos_max.y), text, text_end_ellipsis, &text_size, &ImVec2::new2(0f32, 0f32), null_mut());
        let mut ellipsis_x: c_float = pos_min.x + text_size_clipped_x;
        if ellipsis_x + ellipsis_total_width <= ellipsis_max_x {
            // for (let i: c_int = 0; i < ellipsis_char_count; i+ +)
            for i in 0..ellipsis_char_count {
                font.RenderChar(draw_list, font_size, ImVec2(ellipsis_x, pos_min.y), GetColorU32(ImGuiCol_Text, 0f32, ), ellipsis_char);
                ellipsis_x += ellipsis_glyph_width;
            }
        }
    } else {
        RenderTextClippedEx(draw_list, pos_min, ImVec2(clip_max_x, pos_max.y), text, text_end_full, &text_size, ImVec2::new2(0f32, 0f32), null());
    }

    if g.LogEnabled {
        LogRenderedText(pos_min, text, text_end_full);
    }
}

// Render a rectangle shaped with optional rounding and borders
// c_void RenderFrame(ImVec2 p_min, ImVec2 p_max, u32 fill_col, border: bool, c_float rounding)
pub unsafe fn RenderFrame(p_min: ImVec2, p_max: ImVec2, fill_col: u32, border: bool, rounding: c_float) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    window.DrawList.AddRectFilled(&p_min, &p_max, fill_col, rounding, ImDrawFlags_None);
    let border_size: c_float = g.Style.FrameBorderSize;
    if border && border_size > 0f32 {
        window.DrawList.AddRect(p_min + ImVec2::new2(1f32, 1f32), p_max + ImVec2::new2(1f32, 1f32), GetColorU32(ImGuiCol_BorderShadow, 0f32, ), rounding, 0, border_size, , );
        window.DrawList.AddRect(&p_min, &p_max, GetColorU32(ImGuiCol_Border, 0f32, ), rounding, 0, border_size, , );
    }
}

// c_void RenderFrameBorder(ImVec2 p_min, ImVec2 p_max, c_float rounding)
pub unsafe fn RenderFrameBorder(p_min: ImVec2, p_max: ImVec2, rounding: c_float) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    let border_size: c_float = g.Style.FrameBorderSize;
    if border_size > 0f32 {
        window.DrawList.AddRect(p_min + ImVec2::new2(1, 1), p_max + ImVec2::new2(1, 1), GetColorU32(ImGuiCol_BorderShadow, 0f32, ), rounding, 0, border_size, , );
        window.DrawList.AddRect(&p_min, &p_max, GetColorU32(ImGuiCol_Border, 0f32, ), rounding, 0, border_size, , );
    }
}

// c_void RenderNavHighlight(const ImRect& bb, id: ImGuiID, ImGuiNavHighlightFlags flags)
pub unsafe fn RenderNavHighlight(bb: &ImRect, id: ImGuiID, flags: ImGuiNavHighlightFlags) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if id != g.NavId {
        return;
    }
    if g.NavDisableHighlight && !(flags & ImGuiNavHighlightFlags_AlwaysDraw) == 0 {
        return;
    }
    let mut window = g.CurrentWindow;
    if window.DC.NavHideHighlightOneFrame {
        return;
    }

    let rounding: c_float = if (flags & ImGuiNavHighlightFlags_NoRounding) != 0 { 0f32 } else { g.Style.FrameRounding };
    let mut display_rect: ImRect = bb.clone();
    display_rect.ClipWith(&window.ClipRect);
    if flags & ImGuiNavHighlightFlags_TypeDefault {
        let THICKNESS: c_float = 2.0f32;
        let DISTANCE: c_float = 3.0f32 + THICKNESS * 0.5f32;
        display_rect.Expand(ImVec2(DISTANCE, DISTANCE));
        let mut fully_visible: bool = window.ClipRect.Contains2(&display_rect);
        if !fully_visible {
            window.DrawList.PushClipRect(&display_rect.Min, &display_rect.Max, false);
        }
        window.DrawList.AddRect(display_rect.Min + ImVec2(THICKNESS * 0.5f32, THICKNESS * 0.5f32), display_rect.Max - ImVec2(THICKNESS * 0.5f32, THICKNESS * 0.5f32), GetColorU32(ImGuiCol_NavHighlight, 0f32, ), rounding, 0, THICKNESS, , );
        if !fully_visible {
            window.DrawList.PopClipRect();
        }
    }
    if flags & ImGuiNavHighlightFlags_TypeThin {
        window.DrawList.AddRect(&display_rect.Min, &display_rect.Max, GetColorU32(ImGuiCol_NavHighlight, 0f32, ), rounding, 0, 1f32, , );
    }
}

// c_void RenderMouseCursor(ImVec2 base_pos, c_float base_scale, ImGuiMouseCursor mouse_cursor, u32 col_fill, u32 col_border, u32 col_shadow)
pub unsafe fn RenderMouseCursor(base_pos: ImVec2, base_scale: c_float, mouse_cursor: ImGuiMouseCursor, col_fill: u32, col_border: u32, col_shadow: u32) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(mouse_cursor > ImGuiMouseCursor_None && mouse_cursor < ImGuiMouseCursor_COUNT);
    let mut font_atlas: *mut ImFontAtlas = g.DrawListSharedData.Font.ContainerAtlas;
    // for (let n: c_int = 0; n < g.Viewports.Size; n++)
    for n in 0..g.Viewports.len() {
        // We scale cursor with current viewport/monitor, however Windows 10 for its own hardware cursor seems to be using a different scale factor.
        // ImVec2 offset, size, uv[4];
        let mut offset: ImVec2;
        let mut size: ImVec2 = ImVec2::new2(0f32, 0f32);
        let mut uv: [ImVec2; 4];
        let out_uv_border: [ImVec2; 2] = [uv[0].clone(), uv[1].clone()];
        let out_uv_fill: [ImVec2; 2] = [uv[2].clone(), uv[3].clone()];
        if !font_atlas.GetMouseCursorTexData(mouse_cursor, &mut offset, &mut size, out_uv_border, out_uv_fill) {
            continue;
        }
        let mut viewport: *mut ImGuiViewport = g.Viewports[n];
        let pos: ImVec2 = &base_pos - &offset;
        let scale: c_float = base_scale * viewport.DpiScale;
        if !viewport.GetMainRect().Overlaps(ImRect(pos.clone(), pos.clone() + ImVec2::new2(size.x + 2, size.y + 2) * scale)) {
            continue;
        }
        let mut draw_list: *mut ImDrawList = GetForegroundDrawList(viewport);
        let mut tex_id: ImTextureID = font_atlas.TexID;
        draw_list.PushTextureID(tex_id);
        draw_list.AddImage(tex_id, pos + ImVec2::new2(1, 0) * scale, &pos + (ImVec2::new2(1f32, 0f32) + size) * scale, &uv[2], &uv[3], col_shadow);
        draw_list.AddImage(tex_id, &pos + ImVec2::new2(2, 0) * scale, &pos + (ImVec2::new2(2, 0) + &size) * scale, &uv[2], &uv[3], col_shadow);
        draw_list.AddImage(tex_id, &pos, &pos + &size * scale, &uv[2], &uv[3], col_border);
        draw_list.AddImage(tex_id, &pos, &pos + &size * scale, &uv[0], &uv[1], col_fill);
        draw_list.PopTextureID();
    }
}


// Prepare the data for rendering so you can call GetDrawData()
// (As with anything within the  namspace this doesn't touch your GPU or graphics API at all:
// it is the role of the ImGui_ImplXXXX_RenderDrawData() function provided by the renderer backend)
// c_void Render()
pub unsafe fn Render()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.Initialized);

    if g.FrameCountEnded != g.FrameCount {
        EndFrame();
    }
    let first_render_of_frame: bool = (g.FrameCountRendered != g.FrameCount);
    g.FrameCountRendered = g.FrameCount;
    g.IO.MetricsRenderWindows = 0;

    CallContextHooks(g, ImGuiContextHookType_RenderPre);

    // Add background ImDrawList (for each active viewport)
    // for (let n: c_int = 0; n != g.Viewports.Size; n++)
    for n in 0 .. g.Viewports.len()
    {
        let mut viewport: *mut ImGuiViewport =  g.Viewports[n];
        viewport.DrawDataBuilder.Clear();
        if viewport.DrawLists[0] != null_mut() {
            AddDrawListToDrawData(&mut viewport.DrawDataBuilder.Layers[0], GetBackgroundDrawList(viewport));
        }
    }

    // Add ImDrawList to render
    let mut windows_to_render_top_most: [*mut ImGuiWindow;2] = [null_mut(),null_mut()];
    windows_to_render_top_most[0] = if g.NavWindowingTarget.is_null() == false && !flag_set(g.NavWindowingTarget.Flags, ImGuiWindowFlags_NoBringToFrontOnFocus) { g.NavWindowingTarget.RootWindowDockTree } else { null_mut() };
    windows_to_render_top_most[1] =  (if g.NavWindowingTarget { g.NavWindowingListWindow } else { null_mut() });
    // for (let n: c_int = 0; n != g.Windows.Size; n++)
    for n in 0 .. g.Windows.len()
    {
        let mut window: *mut ImGuiWindow =  g.Windows[n];
        IM_MSVC_WARNING_SUPPRESS(6011); // Static Analysis false positive "warning C6011: Dereferencing NULL pointer 'window'"
        if IsWindowActiveAndVisible(window) && (window.Flags & ImGuiWindowFlags_ChildWindow) == 0 && window != windows_to_render_top_most[0] && window != windows_to_render_top_most[1] {
            AddRootWindowToDrawData(window);
        }
    }
    // for (let n: c_int = 0; n < IM_ARRAYSIZE(windows_to_render_top_most); n++)
    for n in 0 .. windows_to_render_top_most.len()
    {
        if !(windows_to_render_top_most[n].is_null()) && IsWindowActiveAndVisible(windows_to_render_top_most[n]) { // NavWindowingTarget is always temporarily displayed as the top-most window
            AddRootWindowToDrawData(windows_to_render_top_most[n]);
        }
    }

    // Draw modal/window whitening backgrounds
    if first_render_of_frame {
        RenderDimmedBackgrounds();
    }

    // Draw software mouse cursor if requested by io.MouseDrawCursor flag
    if g.IO.MouseDrawCursor && first_render_of_frame && g.MouseCursor != ImGuiMouseCursor_None {
        RenderMouseCursor(g.IO.MousePos.clone(), g.Style.MouseCursorScale, g.MouseCursor, IM_COL32_WHITE, IM_COL32_BLACK, IM_COL32(0, 0, 0, 48));
    }

    // Setup ImDrawData structures for end-user
    g.IO.MetricsRenderVertices = 0;
    g.IO.MetricsRenderIndices = 0;
    // for (let n: c_int = 0; n < g.Viewports.Size; n++)
    for n in 0 .. g.Viewports.len()
    {
        let mut viewport: *mut ImGuiViewport =  g.Viewports[n];
        viewport.DrawDataBuilder.FlattenIntoSingleLayer();

        // Add foreground ImDrawList (for each active viewport)
        if viewport.DrawLists[1] != null_mut() {
            AddDrawListToDrawData(&mut viewport.DrawDataBuilder.Layers[0], GetForegroundDrawList(viewport));
        }

        SetupViewportDrawData(viewport, &mut viewport.DrawDataBuilder.Layers[0]);
        let draw_data = viewport.DrawData;
        g.IO.MetricsRenderVertices += draw_Data.TotalVtxCount;
        g.IO.MetricsRenderIndices += draw_Data.TotalIdxCount;
    }

    CallContextHooks(g, ImGuiContextHookType_RenderPost);
}
