#![allow(non_snake_case)]
//-----------------------------------------------------------------------------
// [SECTION] RENDER HELPERS
// Some of those (internal) functions are currently quite a legacy mess - their signature and behavior will change,
// we need a nicer separation between low-level functions and high-level functions relying on the ImGui context.
// Also see imgui_draw.cpp for some more which have been reworked to not rely on  context.
//-----------------------------------------------------------------------------

use crate::color::{
    ImGuiCol_Border, ImGuiCol_BorderShadow, ImGuiCol_NavHighlight, ImGuiCol_Text, IM_COL32,
    IM_COL32_A_MASK, IM_COL32_A_SHIFT, IM_COL32_BLACK, IM_COL32_WHITE,
};
use crate::context_hook::{ImGuiContextHookType_RenderPost, ImGuiContextHookType_RenderPre};
use crate::direction::{
    ImGuiDir, ImGuiDir_COUNT, ImGuiDir_Down, ImGuiDir_Left, ImGuiDir_None, ImGuiDir_Right,
    ImGuiDir_Up,
};
use crate::draw_data::ImDrawData;
use crate::draw_data_ops::{AddDrawListToDrawData, AddRootWindowToDrawData};
use crate::draw_flags::{
    ImDrawFlags, ImDrawFlags_None, ImDrawFlags_RoundCornersBottomLeft,
    ImDrawFlags_RoundCornersBottomRight, ImDrawFlags_RoundCornersDefault_,
    ImDrawFlags_RoundCornersMask_, ImDrawFlags_RoundCornersNone, ImDrawFlags_RoundCornersTopLeft,
    ImDrawFlags_RoundCornersTopRight,
};
use crate::draw_list::ImDrawList;
use crate::draw_list_ops::{GetBackgroundDrawList, GetForegroundDrawList};
use crate::font::ImFont;
use crate::font_atlas::ImFontAtlas;
use crate::font_glyph::ImFontGlyph;
use crate::frame_ops::EndFrame;
use crate::imgui::GImGui;
use crate::logging_ops::LogRenderedText;
use crate::math_ops::{ImAcosX, ImCharIsBlankA, ImClamp, ImLerp, ImMax, ImMin};
use crate::mouse_cursor::{ImGuiMouseCursor, ImGuiMouseCursor_None};
use crate::nav_highlight_flags::{
    ImGuiNavHighlightFlags, ImGuiNavHighlightFlags_AlwaysDraw, ImGuiNavHighlightFlags_NoRounding,
    ImGuiNavHighlightFlags_TypeDefault, ImGuiNavHighlightFlags_TypeThin,
};
use crate::rect::ImRect;
use crate::string_ops::{
    ImTextCountUtf8BytesFromChar, ImTextCountUtf8BytesFromChar2, ImTextCountUtf8BytesFromStr,
};
use crate::style_ops::GetColorU32;
use crate::text_ops::CalcTextSize;
use crate::type_defs::{ImGuiID, ImTextureID, ImWchar};
use crate::utils::{flag_clear, flag_set};
use crate::vec2::ImVec2;
use crate::vec4::ImVec4;
use crate::viewport::ImGuiViewport;
use crate::viewport_ops::SetupViewportDrawData;
use crate::window::ops::IsWindowActiveAndVisible;
use crate::window::render::RenderDimmedBackgrounds;
use crate::window::window_flags::{
    ImGuiWindowFlags_ChildWindow, ImGuiWindowFlags_NoBringToFrontOnFocus,
};
use crate::window::ImGuiWindow;
use crate::window_flags::{ImGuiWindowFlags_ChildWindow, ImGuiWindowFlags_NoBringToFrontOnFocus};
use crate::window_ops::{IsWindowActiveAndVisible, RenderDimmedBackgrounds};
use crate::CallContextHooks;
use libc::{c_char, c_float, c_int};
use std::mem::swap;
use std::ptr::{null, null_mut};

// FindRenderedTextEnd: *const c_char(text: *const c_char, text_end: *const c_char)
pub unsafe fn FindRenderedTextEnd(text: &str) -> usize {
    // let mut text_display_end: *const c_char = text;
    // if !text_end {
    //     text_end = null();
    // }
    //
    // while text_display_end < text_end && *text_display_end != '\0' as c_char && (text_display_end[0] != '#' || text_display_end[1] != '#') {
    //     text_display_end += 1;
    // }
    // return text_display_end;
    todo!()
}

// Internal ImGui functions to render text
// RenderText***() functions calls ImDrawList::AddText() calls ImBitmapFont::RenderText()
// c_void RenderText(pos: ImVec2, text: *const c_char, text_end: *const c_char, hide_text_after_hash: bool)
pub unsafe fn RenderText(pos: ImVec2, text: &str, hide_text_after_hash: bool) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;

    // Hide anything after a '##' string
    let mut text_display_end = 0usize;
    if hide_text_after_hash {
        text_display_end = FindRenderedTextEnd(text);
    } else {
        // if !text_end {
        //     text_end = text + libc::strlen(text);
        // } // FIXME-OPT
        text_display_end = text.len() - 1;
    }

    if text != text_display_end {
        window.DrawList.AddText2(
            g.Font,
            g.FontSize,
            &pos,
            GetColorU32(ImGuiCol_Text, 0.0),
            text,
            0.0,
            null(),
        );
        if g.LogEnabled {
            // LogRenderedText(&pos, text, text_display_end);
        }
    }
}

// c_void RenderTextWrapped(pos: ImVec2, text: *const c_char, text_end: *const c_char, c_float wrap_width)
pub unsafe fn RenderTextWrapped(pos: ImVec2, text: &str) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;

    // if !text_end {
    //     text_end = text + libc::strlen(text);
    // } // FIXME-OPT

    window.DrawList.AddText2(
        g.Font,
        g.FontSize,
        &pos,
        GetColorU32(ImGuiCol_Text, 0.0),
        text,
        wrap_width,
        null(),
    );
    if g.LogEnabled {
        // LogRenderedText(&pos, text, text_end);
    }
}

// Default clip_rect uses (pos_min,pos_max)
// Handle clipping on CPU immediately (vs typically let the GPU clip the triangles that are overlapping the clipping rectangle edges)
// c_void RenderTextClippedEx(draw_list: *mut ImDrawList, const pos_min: &mut ImVec2, const pos_max: &mut ImVec2, text: *const c_char, text_display_end: *const c_char, *const text_size_if_known: ImVec2, const align: &mut ImVec2, *const ImRect clip_rect)
pub unsafe fn RenderTextClippedEx(
    mut draw_list: *mut ImDrawList,
    pos_min: &ImVec2,
    pos_max: &ImVec2,
    text: &str,
    text_size_if_known: *const ImVec2,
    align: Option<&ImVec2>,
    clip_rect: *const ImRect,
) {
    // Perform CPU side clipping for single clipped element to avoid using scissor state
    let mut pos: ImVec2 = pos_min.clone();
    let text_size = if text_size_if_known {
        text_size_if_known.clone()
    } else {
        CalcTextSize(text, false, 0.0)
    };

    let clip_min: *const ImVec2 = if clip_rect { &clip_rect.Min } else { &pos_min };
    clip_max: *const ImVec2 = if clip_rect { &clip_rect.Max } else { &pos_max };
    let mut need_clipping: bool =
        (pos.x + text_size.x >= clip_max.x) || (pos.y + text_size.y >= clip_max.y);
    if clip_rect {
        // If we had no explicit clipping rectangle then pos==clip_min
        need_clipping |= (pos.x < clip_min.x) || (pos.y < clip_min.y);
    }

    // Align whole block. We should defer that to the better rendering function when we'll have support for individual line alignment.
    if align.x > 0.0 {
        pos.x = ImMax(pos.x, pos.x + (pos_max.x - pos.x - text_size.x) * align.x);
    }
    if align.y > 0.0 {
        pos.y = ImMax(pos.y, pos.y + (pos_max.y - pos.y - text_size.y) * align.y);
    }

    // Render
    if need_clipping {
        let mut fine_clip_rect =
            ImVec4::from_floats(clip_min.x, clip_min.y, clip_max.x, clip_max.y);
        draw_list.AddText2(
            null(),
            0.0,
            &pos,
            GetColorU32(ImGuiCol_Text, 0.0),
            text,
            0.0,
            &fine_clip_rect,
        );
    } else {
        draw_list.AddText2(
            null_mut(),
            0.0,
            &pos,
            GetColorU32(ImGuiCol_Text, 0.0),
            text,
            0.0,
            null_mut(),
        );
    }
}

// c_void RenderTextClipped(const pos_min: &mut ImVec2, const pos_max: &mut ImVec2, text: *const c_char, text_end: *const c_char, *const text_size_if_known: ImVec2, const align: &mut ImVec2, *const ImRect clip_rect)
pub unsafe fn RenderTextClipped(
    pos_min: &ImVec2,
    pos_max: &ImVec2,
    text: &str,
    text_size_if_known: Option<&ImVec2>,
    align: Option<&ImVec2>,
    clip_rect: Option<&ImRect>,
) {
    // Hide anything after a '##' string
    let mut text_display_end = FindRenderedTextEnd(text);
    let text_len: c_int = (text_display_end - text);
    if text_len == 0 {
        return;
    }

    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    RenderTextClippedEx(
        window.DrawList,
        pos_min,
        pos_max,
        text,
        text_size_if_known,
        align,
        clip_rect,
    );
    if g.LogEnabled {
        // LogRenderedText(pos_min, text);
    }
}

// Another overly complex function until we reorganize everything into a nice all-in-one helper.
// This is made more complex because we have dissociated the layout rectangle (pos_min..pos_max) which define _where_ the ellipsis is, from actual clipping of text and limit of the ellipsis display.
// This is because in the context of tabs we selectively hide part of the text when the Close Button appears, but we don't want the ellipsis to move.
// c_void RenderTextEllipsis(draw_list: *mut ImDrawList, const pos_min: &mut ImVec2, const pos_max: &mut ImVec2, c_float clip_max_x, c_float ellipsis_max_x, text: *const c_char, text_end_full: *const c_char, *const text_size_if_known: ImVec2)
pub unsafe fn RenderTextEllipsis(
    draw_list: *mut ImDrawList,
    pos_min: &ImVec2,
    pos_max: &ImVec2,
    clip_max_x: c_float,
    ellipsis_max_x: c_float,
    text: &str,
    text_size_if_known: *const ImVec2,
) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
                    // if text_end_full == null() {
                    //     text_end_full = FindRenderedTextEnd(text);
                    // }
    let text_end_full = FindRenderedTextEnd(text);
    let text_size: ImVec2 = if text_size_if_known {
        (*text_size_if_known).clone()
    } else {
        CalcTextSize(text, false, 0.0)
    };

    //draw_list.AddLine(ImVec2::new(pos_max.x, pos_min.y - 4), ImVec2::new(pos_max.x, pos_max.y + 4), IM_COL32(0, 0, 255, 255));
    //draw_list.AddLine(ImVec2::new(ellipsis_max_x, pos_min.y-2), ImVec2::new(ellipsis_max_x, pos_max.y+2), IM_COL32(0, 255, 0, 255));
    //draw_list.AddLine(ImVec2::new(clip_max_x, pos_min.y), ImVec2::new(clip_max_x, pos_max.y), IM_COL32(255, 0, 0, 255));
    // FIXME: We could technically remove (last_glyph->AdvanceX - last_glyph->X1) from text_size.x here and save a few pixels.
    if text_size.x > pos_max.x - pos_min.x {
        // Hello wo...
        // |       |   |
        // min   max   ellipsis_max
        //          <-> this is generally some padding value

        let mut font: *const ImFont = draw_list._Data.Font;
        let font_size: c_float = draw_list._Data.FontSize;
        let mut text_end_ellipsis = 0usize;

        let mut ellipsis_char: ImWchar = font.EllipsisChar;
        let mut ellipsis_char_count: c_int = 1;
        if ellipsis_char == -1 {
            ellipsis_char = font.DotChar;
            ellipsis_char_count = 3;
        }
        let glyph: *const ImFontGlyph = font.FindGlyph(ellipsis_char);

        let mut ellipsis_glyph_width: c_float = glyph.X1; // Width of the glyph with no padding on either side
        let mut ellipsis_total_width: c_float = ellipsis_glyph_width; // Full width of entire ellipsis

        if ellipsis_char_count > 1 {
            // Full ellipsis size without free spacing after it.
            let spacing_between_dots: c_float = 1.0 * (draw_list._Data.FontSize / font.FontSize);
            ellipsis_glyph_width = glyph.X1 - glyph.X0 + spacing_between_dots;
            ellipsis_total_width =
                ellipsis_glyph_width * ellipsis_char_count - spacing_between_dots;
        }

        // We can now claim the space between pos_max.x and ellipsis_max.x
        let text_avail_width: c_float = ImMax(
            (ImMax(pos_max.x, ellipsis_max_x) - ellipsis_total_width) - pos_min.x,
            1.0,
        );
        let mut text_size_clipped_x: c_float = font
            .CalcTextSizeA(
                font_size,
                text_avail_width,
                0.0,
                text,
                &mut text_end_ellipsis,
            )
            .x;
        if text == text_end_ellipsis && text_end_ellipsis < text_end_full {
            // Always display at least 1 character if there's no room for character + ellipsis
            text_end_ellipsis = text + ImTextCountUtf8BytesFromChar(text);
            text_size_clipped_x = font
                .CalcTextSizeA(font_size, f32::MAX, 0.0, text, &mut text_end_ellipsis)
                .x;
        }
        while text_end_ellipsis > text && ImCharIsBlankA(text_end_ellipsis[-1]) {
            // Trim trailing space before ellipsis (FIXME: Supporting non-ascii blanks would be nice, for this we need a function to backtrack in UTF-8 text)
            text_end_ellipsis -= 1;
            text_size_clipped_x -= font
                .CalcTextSizeA(
                    font_size,
                    f32::MAX,
                    0.0,
                    text_end_ellipsis,
                    text_end_ellipsis + 1,
                    null_mut(),
                )
                .x; // Ascii blanks are always 1 byte
        }

        // Render text, render ellipsis
        RenderTextClippedEx(
            draw_list,
            pos_min,
            &ImVec2::from_floats(clip_max_x, pos_max.y),
            text,
            text_end_ellipsis,
            &text_size,
            &ImVec2::from_floats(0.0, 0.0),
            null_mut(),
        );
        let mut ellipsis_x: c_float = pos_min.x + text_size_clipped_x;
        if ellipsis_x + ellipsis_total_width <= ellipsis_max_x {
            // for (let i: c_int = 0; i < ellipsis_char_count; i+ +)
            for i in 0..ellipsis_char_count {
                font.RenderChar(
                    draw_list,
                    font_size,
                    ImVec2::from_floats(ellipsis_x, pos_min.y),
                    GetColorU32(ImGuiCol_Text, 0.0),
                    ellipsis_char,
                );
                ellipsis_x += ellipsis_glyph_width;
            }
        }
    } else {
        RenderTextClippedEx(
            draw_list,
            pos_min,
            ImVec2::from_floats(clip_max_x, pos_max.y),
            text,
            text_end_full,
            &text_size,
            ImVec2::from_floats(0.0, 0.0),
            null(),
        );
    }

    if g.LogEnabled {
        LogRenderedText(pos_min, text, text_end_full);
    }
}

// Render a rectangle shaped with optional rounding and borders
// c_void RenderFrame(p_min: ImVec2, p_max: ImVec2, fill_col: u32, border: bool, c_float rounding)
pub unsafe fn RenderFrame(
    p_min: ImVec2,
    p_max: ImVec2,
    fill_col: u32,
    border: bool,
    rounding: c_float,
) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    window
        .DrawList
        .AddRectFilled(&p_min, &p_max, fill_col, rounding, ImDrawFlags_None);
    let border_size: c_float = g.Style.FrameBorderSize;
    if border && border_size > 0.0 {
        window.DrawList.AddRect(
            p_min + ImVec2::from_floats(1.0, 1.0),
            p_max + ImVec2::from_floats(1.0, 1.0),
            GetColorU32(ImGuiCol_BorderShadow, 0.0),
            rounding,
        );
        window
            .DrawList
            .AddRect(&p_min, &p_max, GetColorU32(ImGuiCol_Border, 0.0), rounding);
    }
}

// c_void RenderFrameBorder(p_min: ImVec2, p_max: ImVec2, c_float rounding)
pub unsafe fn RenderFrameBorder(p_min: ImVec2, p_max: ImVec2, rounding: c_float) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    let border_size: c_float = g.Style.FrameBorderSize;
    if border_size > 0.0 {
        window.DrawList.AddRect(
            p_min + ImVec2::from_floats(1, 1),
            p_max + ImVec2::from_floats(1, 1),
            GetColorU32(ImGuiCol_BorderShadow, 0.0),
            rounding,
        );
        window
            .DrawList
            .AddRect(&p_min, &p_max, GetColorU32(ImGuiCol_Border, 0.0), rounding);
    }
}

// c_void RenderNavHighlight(const ImRect& bb, ImGuiID id, ImGuiNavHighlightFlags flags)
pub unsafe fn RenderNavHighlight(bb: &ImRect, id: ImGuiID, flags: ImGuiNavHighlightFlags) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if id != g.NavId {
        return;
    }
    if g.NavDisableHighlight && flag_clear(flags, ImGuiNavHighlightFlags_AlwaysDraw) == 0 {
        return;
    }
    let mut window = g.CurrentWindow;
    if window.DC.NavHideHighlightOneFrame {
        return;
    }

    let rounding: c_float = if flag_set(flags, ImGuiNavHighlightFlags_NoRounding) {
        0.0
    } else {
        g.Style.FrameRounding
    };
    let mut display_rect: ImRect = bb.clone();
    display_rect.ClipWith(&window.ClipRect);
    if flags & ImGuiNavHighlightFlags_TypeDefault {
        let THICKNESS: c_float = 2.0;
        let DISTANCE: c_float = 3.0 + THICKNESS * 0.5;
        display_rect.Expand(ImVec2::from_floats(DISTANCE, DISTANCE));
        let mut fully_visible: bool = window.ClipRect.Contains2(&display_rect);
        if !fully_visible {
            window
                .DrawList
                .PushClipRect(&display_rect.Min, &display_rect.Max, false);
        }
        window.DrawList.AddRect(
            display_rect.Min + ImVec2::from_floats(THICKNESS * 0.5, THICKNESS * 0.5),
            display_rect.Max - ImVec2::from_floats(THICKNESS * 0.5, THICKNESS * 0.5),
            GetColorU32(ImGuiCol_NavHighlight, 0.0),
            rounding,
        );
        if !fully_visible {
            window.DrawList.PopClipRect();
        }
    }
    if flags & ImGuiNavHighlightFlags_TypeThin {
        window.DrawList.AddRect(
            &display_rect.Min,
            &display_rect.Max,
            GetColorU32(ImGuiCol_NavHighlight, 0.0),
            rounding,
        );
    }
}

// c_void RenderMouseCursor(base_pos: ImVec2, c_float base_scale, ImGuiMouseCursor mouse_cursor, col_fill: u32, col_border: u32, col_shadow: u32)
pub unsafe fn RenderMouseCursor(
    base_pos: ImVec2,
    base_scale: c_float,
    mouse_cursor: ImGuiMouseCursor,
    col_fill: u32,
    col_border: u32,
    col_shadow: u32,
) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
                    // IM_ASSERT(mouse_cursor > ImGuiMouseCursor_None && mouse_cursor < ImGuiMouseCursor_COUNT);
    let mut font_atlas: *mut ImFontAtlas = g.DrawListSharedData.Font.ContainerAtlas;
    // for (let n: c_int = 0; n < g.Viewports.Size; n++)
    for n in 0..g.Viewports.len() {
        // We scale cursor with current viewport/monitor, however Windows 10 for its own hardware cursor seems to be using a different scale factor.
        // offset: ImVec2, size, uv[4];
        let mut offset: ImVec2;
        let mut size: ImVec2 = ImVec2::from_floats(0.0, 0.0);
        let mut uv: [ImVec2; 4];
        let out_uv_border: [ImVec2; 2] = [uv[0].clone(), uv[1].clone()];
        let out_uv_fill: [ImVec2; 2] = [uv[2].clone(), uv[3].clone()];
        if !font_atlas.GetMouseCursorTexData(
            mouse_cursor,
            &mut offset,
            &mut size,
            out_uv_border,
            out_uv_fill,
        ) {
            continue;
        }
        let mut viewport: *mut ImGuiViewport = g.Viewports[n];
        let pos: ImVec2 = &base_pos - &offset;
        let scale: c_float = base_scale * viewport.DpiScale;
        if !viewport.GetMainRect().Overlaps(ImRect(
            pos.clone(),
            pos.clone() + ImVec2::from_floats(size.x + 2, size.y + 2) * scale,
        )) {
            continue;
        }
        let mut draw_list: *mut ImDrawList = GetForegroundDrawList(viewport);
        let mut tex_id: ImTextureID = font_atlas.TexID;
        draw_list.PushTextureID(tex_id);
        draw_list.AddImage(
            tex_id,
            pos + ImVec2::from_floats(1, 0) * scale,
            &pos + (ImVec2::from_floats(1.0, 0.0) + size) * scale,
            &uv[2],
            &uv[3],
            col_shadow,
        );
        draw_list.AddImage(
            tex_id,
            &pos + ImVec2::from_floats(2, 0) * scale,
            &pos + (ImVec2::from_floats(2, 0) + &size) * scale,
            &uv[2],
            &uv[3],
            col_shadow,
        );
        draw_list.AddImage(
            tex_id,
            &pos,
            &pos + &size * scale,
            &uv[2],
            &uv[3],
            col_border,
        );
        draw_list.AddImage(tex_id, &pos, &pos + &size * scale, &uv[0], &uv[1], col_fill);
        draw_list.PopTextureID();
    }
}

// Prepare the data for rendering so you can call GetDrawData()
// (As with anything within the  namspace this doesn't touch your GPU or graphics API at all:
// it is the role of the ImGui_ImplXXXX_RenderDrawData() function provided by the renderer backend)
// c_void Render()
pub unsafe fn Render() {
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
    for n in 0..g.Viewports.len() {
        let mut viewport: *mut ImGuiViewport = g.Viewports[n];
        viewport.DrawDataBuilder.Clear();
        if viewport.DrawLists[0] != null_mut() {
            AddDrawListToDrawData(
                &mut viewport.DrawDataBuilder.Layers[0],
                GetBackgroundDrawList(viewport),
            );
        }
    }

    // Add ImDrawList to render
    let mut windows_to_render_top_most: [*mut ImGuiWindow; 2] = [null_mut(), null_mut()];
    windows_to_render_top_most[0] = if g.NavWindowingTarget.is_null() == false
        && !flag_set(
            g.NavWindowingTarget.Flags,
            ImGuiWindowFlags_NoBringToFrontOnFocus,
        ) {
        g.NavWindowingTarget.RootWindowDockTree
    } else {
        null_mut()
    };
    windows_to_render_top_most[1] = (if g.NavWindowingTarget {
        g.NavWindowingListWindow
    } else {
        null_mut()
    });
    // for (let n: c_int = 0; n != g.Windows.Size; n++)
    for n in 0..g.Windows.len() {
        let mut window: *mut ImGuiWindow = g.Windows[n];
        IM_MSVC_WARNING_SUPPRESS(6011); // Static Analysis false positive "warning C6011: Dereferencing NULL pointer 'window'"
        if IsWindowActiveAndVisible(window)
            && flag_set(window.Flags, ImGuiWindowFlags_ChildWindow) == 0
            && window != windows_to_render_top_most[0]
            && window != windows_to_render_top_most[1]
        {
            AddRootWindowToDrawData(window);
        }
    }
    // for (let n: c_int = 0; n < IM_ARRAYSIZE(windows_to_render_top_most); n++)
    for n in 0..windows_to_render_top_most.len() {
        if !(windows_to_render_top_most[n].is_null())
            && IsWindowActiveAndVisible(windows_to_render_top_most[n])
        {
            // NavWindowingTarget is always temporarily displayed as the top-most window
            AddRootWindowToDrawData(windows_to_render_top_most[n]);
        }
    }

    // Draw modal/window whitening backgrounds
    if first_render_of_frame {
        RenderDimmedBackgrounds();
    }

    // Draw software mouse cursor if requested by io.MouseDrawCursor flag
    if g.IO.MouseDrawCursor && first_render_of_frame && g.MouseCursor != ImGuiMouseCursor_None {
        RenderMouseCursor(
            g.IO.MousePos.clone(),
            g.Style.MouseCursorScale,
            g.MouseCursor,
            IM_COL32_WHITE,
            IM_COL32_BLACK,
            IM_COL32(0, 0, 0, 48),
        );
    }

    // Setup ImDrawData structures for end-user
    g.IO.MetricsRenderVertices = 0;
    g.IO.MetricsRenderIndices = 0;
    // for (let n: c_int = 0; n < g.Viewports.Size; n++)
    for n in 0..g.Viewports.len() {
        let mut viewport: *mut ImGuiViewport = g.Viewports[n];
        viewport.DrawDataBuilder.FlattenIntoSingleLayer();

        // Add foreground ImDrawList (for each active viewport)
        if viewport.DrawLists[1] != null_mut() {
            AddDrawListToDrawData(
                &mut viewport.DrawDataBuilder.Layers[0],
                GetForegroundDrawList(viewport),
            );
        }

        SetupViewportDrawData(viewport, &mut viewport.DrawDataBuilder.Layers[0]);
        let draw_data = viewport.DrawData;
        g.IO.MetricsRenderVertices += draw_Data.TotalVtxCount;
        g.IO.MetricsRenderIndices += draw_Data.TotalIdxCount;
    }

    CallContextHooks(g, ImGuiContextHookType_RenderPost);
}

// Render an arrow aimed to be aligned with text (p_min is a position in the same space text would be positioned). To e.g. denote expanded/collapsed state
pub unsafe fn RenderArrow(
    mut draw_list: &mut ImDrawList,
    pos: &ImVec2,
    col: u32,
    dir: ImGuiDir,
    scale: c_float,
) {
    let h: c_float = draw_list._Data.FontSize * 1;
    let mut r: c_float = h * 0.40 * scale;
    let center: ImVec2 = pos + ImVec2::from_floats(h * 0.50, h * 0.50 * scale);

    // a: ImVec2, b, c;
    let mut a = ImVec2::default();
    let mut b = ImVec2::default();
    let mut c = ImVec2::default();

    match dir {
        ImGuiDir_Up | ImGuiDir_Down => {
            if dir == ImGuiDir_Up {
                r = -r
            };
            a = ImVec2::from_floats(0.0, 0.7500) * r;
            b = ImVec2::from_floats(-0.866, -0.7500) * r;
            c = ImVec2::from_floats(0.866, -0.7500) * r;
        }
        ImGuiDir_Left | ImGuiDir_Right => {
            if dir == ImGuiDir_Left {
                r = -r;
            }
            a = ImVec2::from_floats(0.750, 0.00) * r;
            b = ImVec2::from_floats(-0.750, 0.8660) * r;
            c = ImVec2::from_floats(-0.750, -0.8660) * r;
        }
        ImGuiDir_None | ImGuiDir_COUNT => {}
        _ => {}
    }

    draw_list.AddTriangleFilled(center + a, center + b, center + c, col);
}

pub unsafe fn RenderBullet(mut draw_list: *mut ImDrawList, pos: ImVec2, col: u32) {
    draw_list.AddCircleFilled(&pos, draw_list._Data.FontSize * 0.20, col, 8);
}

pub unsafe fn RenderCheckMark(
    mut draw_list: &mut ImDrawList,
    mut pos: &ImVec2,
    col: u32,
    mut sz: c_float,
) {
    let thickness: c_float = ImMax(sz / 5, 1.0);
    sz -= thickness * 0.5;
    pos += ImVec2::from_floats(thickness * 0.25, thickness * 0.250);

    let third: c_float = sz / 3.0;
    let bx: c_float = pos.x + third;
    let by: c_float = pos.y + sz - third * 0.5;
    draw_list.PathLineTo(&ImVec2::from_floats(bx - third, by - third));
    draw_list.PathLineTo(&ImVec2::from_floats(bx, by));
    draw_list.PathLineTo(&ImVec2::from_floats(bx + third * 2.0, by - third * 2.00));
    draw_list.PathStroke(col, 0, thickness);
}

// Render an arrow. 'pos' is position of the arrow tip. half_sz.x is length from base to tip. half_sz.y is length on each side.
pub unsafe fn RenderArrowPointingAt(
    mut draw_list: *mut ImDrawList,
    pos: ImVec2,
    half_sz: ImVec2,
    direction: ImGuiDir,
    col: u32,
) {
    match direction {
        ImGuiDir_Left => draw_list.AddTriangleFilled(
            &ImVec2::from_floats(pos.x + half_sz.x, pos.y - half_sz.y),
            &ImVec2::from_floats(pos.x + half_sz.x, pos.y + half_sz.y),
            &pos,
            col,
        ),
        ImGuiDir_Right => draw_list.AddTriangleFilled(
            &ImVec2::from_floats(pos.x - half_sz.x, pos.y + half_sz.y),
            &ImVec2::from_floats(pos.x - half_sz.x, pos.y - half_sz.y),
            &pos,
            col,
        ),
        ImGuiDir_Up => draw_list.AddTriangleFilled(
            &ImVec2::from_floats(pos.x + half_sz.x, pos.y + half_sz.y),
            &ImVec2::from_floats(pos.x - half_sz.x, pos.y + half_sz.y),
            &pos,
            col,
        ),
        ImGuiDir_Down => draw_list.AddTriangleFilled(
            &ImVec2::from_floats(pos.x - half_sz.x, pos.y - half_sz.y),
            &ImVec2::from_floats(pos.x + half_sz.x, pos.y - half_sz.y),
            &pos,
            col,
        ),
        ImGuiDir_None | ImGuiDir_COUNT => {} // Fix warnings
        _ => {}
    }
}

// This is less wide than RenderArrow() and we use in dock nodes instead of the regular RenderArrow() to denote a change of functionality,
// and because the saved space means that the left-most tab label can stay at exactly the same position as the label of a loose window.
pub unsafe fn RenderArrowDockMenu(
    mut draw_list: *mut ImDrawList,
    p_min: ImVec2,
    sz: c_float,
    col: u32,
) {
    draw_list.AddRectFilled(
        p_min + ImVec2::from_floats(sz * 0.20, sz * 0.150),
        p_min + ImVec2::from_floats(sz * 0.80, sz * 0.3),
        col,
        0.0,
        0,
    );
    RenderArrowPointingAt(
        draw_list,
        p_min + ImVec2::from_floats(sz * 0.50, sz * 0.850),
        ImVec2::from_floats(sz * 0.3, sz * 0.4),
        ImGuiDir_Down,
        col,
    );
}

// FIXME: Cleanup and move code to ImDrawList.
pub unsafe fn RenderRectFilledRangeH(
    mut draw_list: *mut ImDrawList,
    rect: &ImRect,
    col: u32,
    mut x_start_norm: c_float,
    mut x_end_norm: c_float,
    mut rounding: c_float,
) {
    if x_end_norm == x_start_norm {
        return;
    }
    if x_start_norm > x_end_norm {
        // ImSwap(&mut x_start_norm, &mut x_end_norm);
        swap(&mut x_start_norm, &mut x_end_norm);
    }

    let p0: ImVec2 = ImVec2::from_floats(ImLerp(rect.Min.x, rect.Max.x, x_start_norm), rect.Min.y);
    let p1: ImVec2 = ImVec2::from_floats(ImLerp(rect.Min.x, rect.Max.x, x_end_norm), rect.Max.y);
    if rounding == 0.0 {
        draw_list.AddRectFilled(&p0, &p1, col, 0.0, 0);
        return;
    }

    rounding = ImClamp(
        ImMin(
            (rect.Max.x - rect.Min.x) * 0.5,
            (rect.Max.y - rect.Min.y) * 0.5,
        ) - 1,
        0.0,
        rounding,
    );
    let inv_rounding: c_float = 1 / rounding;
    let arc0_b: c_float = ImAcosX(1 - (p0.x - rect.Min.x) * inv_rounding);
    let arc0_e: c_float = ImAcosX(1 - (p1.x - rect.Min.x) * inv_rounding);
    let half_pi: c_float = IM_PI * 0.5; // We will == compare to this because we know this is the exact value ImAcos01 can return.
    let x0: c_float = ImMax(p0.x, rect.Min.x + rounding);
    if arc0_b == arc0_e {
        draw_list.PathLineTo(&ImVec2::from_floats(x0, p1.y));
        draw_list.PathLineTo(&ImVec2::from_floats(x0, p0.y));
    } else if arc0_b == 0.0 && arc0_e == half_pi {
        draw_list.PathArcToFast(&ImVec2::from_floats(x0, p1.y - rounding), rounding, 3, 6); // BL
        draw_list.PathArcToFast(&ImVec2::from_floats(x0, p0.y + rounding), rounding, 6, 9);
    // TR
    } else {
        draw_list.PathArcTo(
            &ImVec2::from_floats(x0, p1.y - rounding),
            rounding,
            IM_PI - arc0_e,
            IM_PI - arc0_b,
            3,
        ); // BL
        draw_list.PathArcTo(
            &ImVec2::from_floats(x0, p0.y + rounding),
            rounding,
            IM_PI + arc0_b,
            IM_PI + arc0_e,
            3,
        ); // TR
    }
    if p1.x > rect.Min.x + rounding {
        let arc1_b: c_float = ImAcos01(1 - (rect.Max.x - p1.x) * inv_rounding);
        let arc1_e: c_float = ImAcos01(1 - (rect.Max.x - p0.x) * inv_rounding);
        let x1: c_float = ImMin(p1.x, rect.Max.x - rounding);
        if arc1_b == arc1_e {
            draw_list.PathLineTo(&ImVec2::from_floats(x1, p0.y));
            draw_list.PathLineTo(&ImVec2::from_floats(x1, p1.y));
        } else if arc1_b == 0.0 && arc1_e == half_pi {
            draw_list.PathArcToFast(&ImVec2::from_floats(x1, p0.y + rounding), rounding, 9, 12); // TR
            draw_list.PathArcToFast(&ImVec2::from_floats(x1, p1.y - rounding), rounding, 0, 3);
        // BR
        } else {
            draw_list.PathArcTo(
                &ImVec2::from_floats(x1, p0.y + rounding),
                rounding,
                -arc1_e,
                -arc1_b,
                3,
            ); // TR
            draw_list.PathArcTo(
                &ImVec2::from_floats(x1, p1.y - rounding),
                rounding,
                arc1_b,
                arc1_e,
                3,
            ); // BR
        }
    }
    draw_list.PathFillConvex(col);
}

pub unsafe fn RenderRectFilledWithHole(
    mut draw_list: *mut ImDrawList,
    outer: &ImRect,
    inner: &ImRect,
    col: u32,
    rounding: c_float,
) {
    let fill_L: bool = (inner.Min.x > outer.Min.x);
    let fill_R: bool = (inner.Max.x < outer.Max.x);
    let fill_U: bool = (inner.Min.y > outer.Min.y);
    let fill_D: bool = (inner.Max.y < outer.Max.y);
    if fill_L {
        draw_list.AddRectFilled(
            &ImVec2::from_floats(outer.Min.x, inner.Min.y),
            &ImVec2::from_floats(inner.Min.x, inner.Max.y),
            col,
            rounding,
            ImDrawFlags_RoundCornersNone
                | (if fill_U {
                    0
                } else {
                    ImDrawFlags_RoundCornersTopLeft
                })
                | (if fill_D {
                    0
                } else {
                    ImDrawFlags_RoundCornersBottomLeft
                }),
        );
    }
    if fill_R {
        draw_list.AddRectFilled(
            &ImVec2::from_floats(inner.Max.x, inner.Min.y),
            &ImVec2::from_floats(outer.Max.x, inner.Max.y),
            col,
            rounding,
            ImDrawFlags_RoundCornersNone
                | (if fill_U? {
                    0
                } else {
                    ImDrawFlags_RoundCornersTopRight
                })
                | (if fill_D {
                    0
                } else {
                    ImDrawFlags_RoundCornersBottomRight
                }),
        );
    }
    if fill_U {
        draw_list.AddRectFilled(
            &ImVec2::from_floats(inner.Min.x, outer.Min.y),
            &ImVec2::from_floats(inner.Max.x, inner.Min.y),
            col,
            rounding,
            ImDrawFlags_RoundCornersNone
                | (if fill_L {
                    0
                } else {
                    ImDrawFlags_RoundCornersTopLeft
                })
                | (if fill_R {
                    0
                } else {
                    ImDrawFlags_RoundCornersTopRight
                }),
        );
    }
    if fill_D {
        draw_list.AddRectFilled(
            &ImVec2::from_floats(inner.Min.x, inner.Max.y),
            &ImVec2::from_floats(inner.Max.x, outer.Max.y),
            col,
            rounding,
            ImDrawFlags_RoundCornersNone
                | (if fill_L {
                    0
                } else {
                    ImDrawFlags_RoundCornersBottomLeft
                })
                | (if fill_R {
                    0
                } else {
                    ImDrawFlags_RoundCornersBottomRight
                }),
        );
    }
    if fill_L && fill_U {
        draw_list.AddRectFilled(
            &ImVec2::from_floats(outer.Min.x, outer.Min.y),
            &ImVec2::from_floats(inner.Min.x, inner.Min.y),
            col,
            rounding,
            ImDrawFlags_RoundCornersTopLeft,
        );
    }
    if fill_R && fill_U {
        draw_list.AddRectFilled(
            &ImVec2::from_floats(inner.Max.x, outer.Min.y),
            &ImVec2::from_floats(outer.Max.x, inner.Min.y),
            col,
            rounding,
            ImDrawFlags_RoundCornersTopRight,
        );
    }
    if fill_L && fill_D {
        draw_list.AddRectFilled(
            &ImVec2::from_floats(outer.Min.x, inner.Max.y),
            &ImVec2::from_floats(inner.Min.x, outer.Max.y),
            col,
            rounding,
            ImDrawFlags_RoundCornersBottomLeft,
        );
    }
    if fill_R && fill_D {
        draw_list.AddRectFilled(
            &ImVec2::from_floats(inner.Max.x, inner.Max.y),
            &ImVec2::from_floats(outer.Max.x, outer.Max.y),
            col,
            rounding,
            ImDrawFlags_RoundCornersBottomRight,
        );
    }
}

pub fn CalcRoundingFlagsForRectInRect(
    r_in: &ImRect,
    r_outer: &ImRect,
    threshold: c_float,
) -> ImDrawFlags {
    let mut round_l: bool = r_in.Min.x <= r_outer.Min.x + threshold;
    let mut round_r: bool = r_in.Max.x >= r_outer.Max.x - threshold;
    let mut round_t: bool = r_in.Min.y <= r_outer.Min.y + threshold;
    let mut round_b: bool = r_in.Max.y >= r_outer.Max.y - threshold;
    return ImDrawFlags_RoundCornersNone
        | (if round_t && round_l {
            ImDrawFlags_RoundCornersTopLeft
        } else {
            0
        })
        | (if round_t && round_r {
            ImDrawFlags_RoundCornersTopRight
        } else {
            0
        })
        | (if round_b && round_l {
            ImDrawFlags_RoundCornersBottomLeft
        } else {
            0
        })
        | (if round_b && round_r {
            ImDrawFlags_RoundCornersBottomRight
        } else {
            0
        });
}

// Helper for ColorPicker4()
// NB: This is rather brittle and will show artifact when rounding this enabled if rounded corners overlap multiple cells. Caller currently responsible for avoiding that.
// Spent a non reasonable amount of time trying to getting this right for ColorButton with rounding+anti-aliasing+ImGuiColorEditFlags_HalfAlphaPreview flag + various grid sizes and offsets, and eventually gave up... probably more reasonable to disable rounding altogether.
// FIXME: uses GetColorU32
pub unsafe fn RenderColorRectWithAlphaCheckerboard(
    mut draw_list: *mut ImDrawList,
    p_min: ImVec2,
    p_max: ImVec2,
    col: u32,
    grid_step: c_float,
    grid_off: ImVec2,
    rounding: c_float,
    mut flags: ImDrawFlags,
) {
    if (flag_clear(flags, ImDrawFlags_RoundCornersMask_)) {
        flags = ImDrawFlags_RoundCornersDefault_;
    }
    if (((col & IM_COL32_A_MASK) >> IM_COL32_A_SHIFT) < 0xF0) {
        col_bg1: u32 = GetColorU32(ImAlphaBlendColors(IM_COL32(204, 204, 204, 255), col), 0.0);
        col_bg2: u32 = GetColorU32(ImAlphaBlendColors(IM_COL32(128, 128, 128, 255), col), 0.0);
        draw_list.AddRectFilled(&p_min, &p_max, col_bg1, rounding, flags);

        let mut yi: c_int = 0;
        // for (let y: c_float =  p_min.y + grid_off.y; y < p_max.y; y += grid_step, yi++)
        for y in (p_min.y..p_max.y).step(grid_step) {
            let y1: c_float = ImClamp(y, p_min.y, p_max.y);
            let y2 = ImMin(y + grid_step, p_max.y);
            if y2 <= y1 {
                continue;
            }
            // for (let x: c_float =  p_min.x + grid_off.x + (yi & 1) * grid_step; x < p_max.x; x += grid_step * 2.00)
            for x in (p_min.x + grid_off.x + (yi & 1) * grid_step..p_max.x).step(grid_step * 2.0) {
                let x1: c_float = ImClamp(x, p_min.x, p_max.x);
                let x2 = ImMin(x + grid_step, p_max.x);
                if x2 <= x1 {
                    continue;
                }
                cell_flags: ImDrawFlags = ImDrawFlags_RoundCornersNone;
                if y1 <= p_min.y {
                    if x1 <= p_min.x {
                        cell_flags |= ImDrawFlags_RoundCornersTopLeft;
                    }
                    if x2 >= p_max.x {
                        cell_flags |= ImDrawFlags_RoundCornersTopRight;
                    }
                }
                if y2 >= p_max.y {
                    if x1 <= p_min.x {
                        cell_flags |= ImDrawFlags_RoundCornersBottomLeft;
                    }
                    if x2 >= p_max.x {
                        cell_flags |= ImDrawFlags_RoundCornersBottomRight;
                    }
                }

                // Combine flags
                cell_flags = if flags == ImDrawFlags_RoundCornersNone
                    || cell_flags == ImDrawFlags_RoundCornersNone
                {
                    ImDrawFlags_RoundCornersNone
                } else {
                    (cell_flags & flags)
                };
                draw_list.AddRectFilled(
                    &ImVec2::from_floats(x1, y1),
                    &ImVec2::from_floats(x2, y2),
                    col_bg2,
                    rounding,
                    cell_flags,
                );
            }
            yi += 1;
        }
    } else {
        draw_list.AddRectFilled(&p_min, &p_max, col, rounding, flags);
    }
}
