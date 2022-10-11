// dear imgui, v1.89 WIP
// (drawing and font code)

use std::ffi::CStr;
use std::ptr::null_mut;
use libc::{c_char, c_float, c_int, c_uchar, c_uint, c_ushort, c_void, size_t};
use crate::color::{IM_COL32, IM_COL32_A_MASK, IM_COL32_B_SHIFT, IM_COL32_BLACK_TRANS, IM_COL32_G_SHIFT, IM_COL32_R_SHIFT, IM_COL32_WHITE, ImGuiCol_Border, ImGuiCol_BorderShadow, ImGuiCol_Button, ImGuiCol_ButtonActive, ImGuiCol_ButtonHovered, ImGuiCol_CheckMark, ImGuiCol_ChildBg, ImGuiCol_DockingEmptyBg, ImGuiCol_DockingPreview, ImGuiCol_DragDropTarget, ImGuiCol_FrameBg, ImGuiCol_FrameBgActive, ImGuiCol_FrameBgHovered, ImGuiCol_Header, ImGuiCol_HeaderActive, ImGuiCol_HeaderHovered, ImGuiCol_MenuBarBg, ImGuiCol_ModalWindowDimBg, ImGuiCol_NavHighlight, ImGuiCol_NavWindowingDimBg, ImGuiCol_NavWindowingHighlight, ImGuiCol_PlotHistogram, ImGuiCol_PlotHistogramHovered, ImGuiCol_PlotLines, ImGuiCol_PlotLinesHovered, ImGuiCol_PopupBg, ImGuiCol_ResizeGrip, ImGuiCol_ResizeGripActive, ImGuiCol_ResizeGripHovered, ImGuiCol_ScrollbarBg, ImGuiCol_ScrollbarGrab, ImGuiCol_ScrollbarGrabActive, ImGuiCol_ScrollbarGrabHovered, ImGuiCol_Separator, ImGuiCol_SeparatorActive, ImGuiCol_SeparatorHovered, ImGuiCol_SliderGrab, ImGuiCol_SliderGrabActive, ImGuiCol_Tab, ImGuiCol_TabActive, ImGuiCol_TabHovered, ImGuiCol_TableBorderLight, ImGuiCol_TableBorderStrong, ImGuiCol_TableHeaderBg, ImGuiCol_TableRowBg, ImGuiCol_TableRowBgAlt, ImGuiCol_TabUnfocused, ImGuiCol_TabUnfocusedActive, ImGuiCol_Text, ImGuiCol_TextDisabled, ImGuiCol_TextSelectedBg, ImGuiCol_TitleBg, ImGuiCol_TitleBgActive, ImGuiCol_TitleBgCollapsed, ImGuiCol_WindowBg};
use crate::draw_cmd::ImDrawCmd;
use crate::draw_list::ImDrawList;
use crate::draw_vert::ImDrawVert;
use crate::font::ImFont;
use crate::font_atlas::ImFontAtlas;
use crate::font_atlas_custom_rect::ImFontAtlasCustomRect;
use crate::font_atlas_default_tex_data::{FONT_ATLAS_DEFAULT_TEX_DATA_H, FONT_ATLAS_DEFAULT_TEX_DATA_PIXELS, FONT_ATLAS_DEFAULT_TEX_DATA_PIXELS_as_const_char_ptr, FONT_ATLAS_DEFAULT_TEX_DATA_PIXELS_as_vec, FONT_ATLAS_DEFAULT_TEX_DATA_W};
use crate::font_atlas_flags::{ImFontAtlasFlags_NoBakedLines, ImFontAtlasFlags_NoMouseCursors, ImFontAtlasFlags_NoPowerOfTwoHeight};
use crate::font_build_dst_data::ImFontBuildDstData;
use crate::font_build_src_data::ImFontBuildSrcData;
use crate::font_config::ImFontConfig;
use crate::math_ops::{ImAcos, ImClamp, ImLerp, ImMax, ImMin, ImMul, ImSqrt};
use crate::mouse_cursor::ImGuiMouseCursor_COUNT;
use crate::style::ImGuiStyle;
use crate::style_ops::GetStyle;
use crate::type_defs::ImWchar;
use crate::utils::{flag_clear, is_not_null};
use crate::vec2::ImVec2;
use crate::vec4::ImVec4;

// ImDrawCallback: Draw callbacks for advanced uses [configurable type: override in imconfig.h]
// NB: You most likely do NOT need to use draw callbacks just to create your own widget or customized UI rendering,
// you can poke into the draw list for that! Draw callback may be useful for example to:
//  A) Change your GPU render state,
//  B) render a complex 3D scene inside a UI element without an intermediate texture/render target, etc.
// The expected behavior from your rendering function is 'if (cmd.UserCallback != NULL) { cmd.UserCallback(parent_list, cmd); } else { RenderTriangles() }'
// If you want to override the signature of ImDrawCallback, you can simply use e.g. '#define ImDrawCallback MyDrawCallback' (in imconfig.h) + update rendering backend accordingly.
// #ifndef ImDrawCallback
// typedef c_void (*ImDrawCallback)(*const ImDrawList parent_list, *const ImDrawCmd cmd);
pub type ImDrawCallback = fn(parent_list: *const ImDrawList, cmd: *const ImDrawCmd);


// Default font TTF is compressed with stb_compress then base85 encoded (see misc/fonts/binary_to_compressed_c.cpp for encoder)
// static stb_decompress_length: c_uint(const input: *mut c_uchar);
// static stb_decompress: c_uint(output: *mut c_uchar, const input: *mut c_uchar, length: c_uint);
// static *const char  GetDefaultCompressedFontDataTTFBase85();
pub fn Decode85Byte(c: c_char)     -> c_uint                                {
    return if c >= '\\' as c_char { c - 36 } else { c - 35 } as c_uint;
}


pub unsafe fn Decode85(mut src: *const c_char, mut dst: *mut c_uchar) {
    while *src {
        let mut tmp: c_uint = Decode85Byte(src[0]) + 85 * (Decode85Byte(src[1]) + 85 * (Decode85Byte(src[2]) + 85 * (Decode85Byte(src[3]) + 85 * Decode85Byte(src[4]))));
        dst[0] = ((tmp >> 0) & 0xF0);
        dst[1] = ((tmp.clone() >> 8) & 0xF0);
        dst[2] = ((tmp.clone() >> 16) & 0xF0);
        dst[3] = ((tmp.clone() >> 24) & 0xF0);   // We can't assume little-endianness.
        src += 5;
        dst += 4;
    }
}

// #endif // IMGUI_ENABLE_STB_TRUETYPE

pub fn UnpackAccumulativeOffsetsIntoRanges(mut base_codepoint: c_int, accumulative_offsets: *const c_ushort, accumulative_offsets_count: size_t, mut out_ranges: *mut ImWchar) {
    // for (let n: c_int = 0; n < accumulative_offsets_count; n++, out_ranges += 2)
    for n in 0..accumulative_offsets_count {
        out_ranges[0] = out_ranges[1] = (base_codepoint + accumulative_offsets[n]);
        base_codepoint += accumulative_offsets[n];
        out_ranges += 2;
    }
    out_ranges[0] = 0;
}


pub fn FindFirstExistingGlyph(mut font: *mut ImFont, candidate_chars: *const ImWchar, candidate_chars_count: usize) -> ImWchar {
    // for (let n: c_int = 0; n < candidate_chars_count; n++)
    for n in 0..candidate_chars_count {
        if font.FindGlyphNoFallback(candidate_chars[n]) != null_mut() {
            return candidate_chars[n];
        }
    }
    return -1;
}


// Note: as with every ImDrawList drawing function, this expects that the font atlas texture is bound.
c_void ImFont::RenderChar(draw_list: *mut ImDrawList,size: c_float, pos: &ImVec2, col: u32, ImWchar c) const
{
    let glyph: *const ImFontGlyph = FindGlyph(c);
    if (!glyph || !glyph->Visible)
        return;
    if (glyph->Colored)
        col |= ~IM_COL32_A_MASK;
    let scale: c_float =  (size >= 0) ? (size / FontSize) : 1;
    let x: c_float =  IM_FLOOR(pos.x);
    let y: c_float =  IM_FLOOR(pos.y);
    draw_list.PrimReserve(6, 4);
    draw_list.PrimRectUV(ImVec2::new(x + glyph->X0 * scale, y + glyph->Y0 * scale), ImVec2::new(x + glyph->X1 * scale, y + glyph->Y1 * scale), ImVec2::new(glyph->U0, glyph->V0), ImVec2::new(glyph->U1, glyph->V1), col);
}

// Note: as with every ImDrawList drawing function, this expects that the font atlas texture is bound.
c_void ImFont::RenderText(draw_list: *mut ImDrawList,size: c_float, pos: &ImVec2, col: u32, const ImVec4& clip_rect, text_begin: *const c_char, text_end: *const c_char,wrap_width: c_float, cpu_fine_clip: bool) const
{
    if (!text_end)
        text_end = text_begin + strlen(text_begin); //  functions generally already provides a valid text_end, so this is merely to handle direct calls.

    // Align to be pixel perfect
    let x: c_float =  IM_FLOOR(pos.x);
    let y: c_float =  IM_FLOOR(pos.y);
    if (y > clip_rect.w)
        return;

    let start_x: c_float =  x;
    let scale: c_float =  size / FontSize;
    let line_height: c_float =  FontSize * scale;
    let word_wrap_enabled: bool = (wrap_width > 0);
    let mut  word_wrap_eol: *const c_char= null_mut();

    // Fast-forward to first visible line
    let mut  s: *const c_char = text_begin;
    if (y + line_height < clip_rect.y && !word_wrap_enabled)
        while (y + line_height < clip_rect.y && s < text_end)
        {
            s =memchr(s, '\n', text_end - s);
            s = s ? s + 1 : text_end;
            y += line_height;
        }

    // For large text, scan for the last visible line in order to avoid over-reserving in the call to PrimReserve()
    // Note that very large horizontal line will still be affected by the issue (e.g. a one megabyte string buffer without a newline will likely crash atm)
    if (text_end - s > 10000 && !word_wrap_enabled)
    {
        let mut  s_end: *const c_char = s;
        let y_end: c_float =  y;
        while (y_end < clip_rect.w && s_end < text_end)
        {
            s_end =memchr(s_end, '\n', text_end - s_end);
            s_end = s_end ? s_end + 1 : text_end;
            y_end += line_height;
        }
        text_end = s_end;
    }
    if (s == text_end)
        return;

    // Reserve vertices for remaining worse case (over-reserving is useful and easily amortized)
    let vtx_count_max: c_int = (text_end - s) * 4;
    let idx_count_max: c_int = (text_end - s) * 6;
    let idx_expected_size: c_int = draw_list.IdxBuffer.len() + idx_count_max;
    draw_list.PrimReserve(idx_count_max, vtx_count_max);

    vtx_write: *mut ImDrawVert = draw_list._VtxWritePtr;
    ImDrawIdx* idx_write = draw_list._IdxWritePtr;
    let mut vtx_current_idx: c_uint =  draw_list._VtxCurrentIdx;

    col_untinted: u32 = col | ~IM_COL32_A_MASK;

    while (s < text_end)
    {
        if (word_wrap_enabled)
        {
            // Calculate how far we can render. Requires two passes on the string data but keeps the code simple and not intrusive for what's essentially an uncommon feature.
            if (!word_wrap_eol)
            {
                word_wrap_eol = CalcWordWrapPositionA(scale, s, text_end, wrap_width - (x - start_x));
                if (word_wrap_eol == s) // Wrap_width is too small to fit anything. Force displaying 1 character to minimize the height discontinuity.
                    word_wrap_eol+= 1;    // +1 may not be a character start point in UTF-8 but it's ok because we use s >= word_wrap_eol below
            }

            if (s >= word_wrap_eol)
            {
                x = start_x;
                y += line_height;
                word_wrap_eol= null_mut();

                // Wrapping skips upcoming blanks
                while (s < text_end)
                {
                    const  c: c_char = *s;
                    if (ImCharIsBlankA(c)) { s+= 1; } else if (c == '\n') { s+= 1; break; } else { break; }
                }
                continue;
            }
        }

        // Decode and advance source
        let mut c: c_uint =  *s;
        if (c < 0x80)
        {
            s += 1;
        }
        else
        {
            s += ImTextCharFromUtf8(&c, s, text_end);
            if (c == 0) // Malformed UTF-8?
                break;
        }

        if (c < 32)
        {
            if (c == '\n')
            {
                x = start_x;
                y += line_height;
                if (y > clip_rect.w)
                    break; // break out of main loop
                continue;
            }
            if (c == '\r')
                continue;
        }

        let glyph: *const ImFontGlyph = FindGlyph(c);
        if (glyph == null_mut())
            continue;

        let char_width: c_float =  glyph->AdvanceX * scale;
        if (glyph->Visible)
        {
            // We don't do a second finer clipping test on the Y axis as we've already skipped anything before clip_rect.y and exit once we pass clip_rect.w
            let x1: c_float =  x + glyph->X0 * scale;
            let x2: c_float =  x + glyph->X1 * scale;
            let y1: c_float =  y + glyph->Y0 * scale;
            let y2: c_float =  y + glyph->Y1 * scale;
            if (x1 <= clip_rect.z && x2 >= clip_rect.x)
            {
                // Render a character
                let u1: c_float =  glyph->U0;
                let v1: c_float =  glyph->V0;
                let u2: c_float =  glyph->U1;
                let v2: c_float =  glyph->V1;

                // CPU side clipping used to fit text in their frame when the frame is too small. Only does clipping for axis aligned quads.
                if (cpu_fine_clip)
                {
                    if (x1 < clip_rect.x)
                    {
                        u1 = u1 + (1 - (x2 - clip_rect.x) / (x2 - x1)) * (u2 - u1);
                        x1 = clip_rect.x;
                    }
                    if (y1 < clip_rect.y)
                    {
                        v1 = v1 + (1 - (y2 - clip_rect.y) / (y2 - y1)) * (v2 - v1);
                        y1 = clip_rect.y;
                    }
                    if (x2 > clip_rect.z)
                    {
                        u2 = u1 + ((clip_rect.z - x1) / (x2 - x1)) * (u2 - u1);
                        x2 = clip_rect.z;
                    }
                    if (y2 > clip_rect.w)
                    {
                        v2 = v1 + ((clip_rect.w - y1) / (y2 - y1)) * (v2 - v1);
                        y2 = clip_rect.w;
                    }
                    if (y1 >= y2)
                    {
                        x += char_width;
                        continue;
                    }
                }

                // Support for untinted glyphs
                glyph_col: u32 = glyph->Colored ? col_untinted : col;

                // We are NOT calling PrimRectUV() here because non-inlined causes too much overhead in a debug builds. Inlined here:
                {
                    idx_write[0] = (vtx_current_idx); idx_write[1] = (vtx_current_idx+1); idx_write[2] = (vtx_current_idx+2);
                    idx_write[3] = (vtx_current_idx); idx_write[4] = (vtx_current_idx+2); idx_write[5] = (vtx_current_idx+3);
                    vtx_write[0].pos.x = x1; vtx_write[0].pos.y = y1; vtx_write[0].col = glyph_col; vtx_write[0].uv.x = u1; vtx_write[0].uv.y = v1;
                    vtx_write[1].pos.x = x2; vtx_write[1].pos.y = y1; vtx_write[1].col = glyph_col; vtx_write[1].uv.x = u2; vtx_write[1].uv.y = v1;
                    vtx_write[2].pos.x = x2; vtx_write[2].pos.y = y2; vtx_write[2].col = glyph_col; vtx_write[2].uv.x = u2; vtx_write[2].uv.y = v2;
                    vtx_write[3].pos.x = x1; vtx_write[3].pos.y = y2; vtx_write[3].col = glyph_col; vtx_write[3].uv.x = u1; vtx_write[3].uv.y = v2;
                    vtx_write += 4;
                    vtx_current_idx += 4;
                    idx_write += 6;
                }
            }
        }
        x += char_width;
    }

    // Give back unused vertices (clipped ones, blanks) ~ this is essentially a PrimUnreserve() action.
    draw_list.VtxBuffer.len() = (vtx_write - draw_list.VtxBuffer.Data); // Same as calling shrink()
    draw_list.IdxBuffer.len() = (idx_write - draw_list.IdxBuffer.Data);
    draw_list.CmdBuffer[draw_list.CmdBuffer.len() - 1].ElemCount -= (idx_expected_size - draw_list.IdxBuffer.len());
    draw_list._VtxWritePtr = vtx_write;
    draw_list._IdxWritePtr = idx_write;
    draw_list._VtxCurrentIdx = vtx_current_idx;
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
c_void RenderArrow(draw_list: *mut ImDrawList, pos: ImVec2, col: u32, ImGuiDir dir,scale: c_float)
{
    let h: c_float =  draw_list._Data.FontSize * 1;
    let r: c_float =  h * 0.40 * scale;
    let center: ImVec2 = pos + ImVec2::new(h * 0.50, h * 0.50 * scale);

    a: ImVec2, b, c;
    switch (dir)
    {
    case ImGuiDir_Up:
    case ImGuiDir_Down:
        if (dir == ImGuiDir_Up) r = -r;
        a = ImVec2::new(+0.000, +0.7500) * r;
        b = ImVec2::new(-0.866, -0.7500) * r;
        c = ImVec2::new(+0.866, -0.7500) * r;
        break;
    case ImGuiDir_Left:
    case ImGuiDir_Right:
        if (dir == ImGuiDir_Left) r = -r;
        a = ImVec2::new(+0.750, +0.0000) * r;
        b = ImVec2::new(-0.750, +0.8660) * r;
        c = ImVec2::new(-0.750, -0.8660) * r;
        break;
    case ImGuiDir_None:
    case ImGuiDir_COUNT:
        // IM_ASSERT(0);
        break;
    }
    draw_list.AddTriangleFilled(center + a, center + b, center + c, col);
}

c_void RenderBullet(draw_list: *mut ImDrawList, pos: ImVec2, col: u32)
{
    draw_list.AddCircleFilled(pos, draw_list._Data.FontSize * 0.20, col, 8);
}

c_void RenderCheckMark(draw_list: *mut ImDrawList, pos: ImVec2, col: u32,sz: c_float)
{
    let thickness: c_float =  ImMax(sz / 5, 1);
    sz -= thickness * 0.5;
    pos += ImVec2::new(thickness * 0.25, thickness * 0.250);

    let third: c_float =  sz / 3.0;
    let bx: c_float =  pos.x + third;
    let by: c_float =  pos.y + sz - third * 0.5;
    draw_list.PathLineTo(ImVec2::new(bx - third, by - third));
    draw_list.PathLineTo(ImVec2::new(bx, by));
    draw_list.PathLineTo(ImVec2::new(bx + third * 2.0, by - third * 2.00));
    draw_list.PathStroke(col, 0, thickness);
}

// Render an arrow. 'pos' is position of the arrow tip. half_sz.x is length from base to tip. half_sz.y is length on each side.
c_void RenderArrowPointingAt(draw_list: *mut ImDrawList, pos: ImVec2, half_sz: ImVec2, ImGuiDir direction, col: u32)
{
    switch (direction)
    {
    case ImGuiDir_Left:  draw_list.AddTriangleFilled(ImVec2::new(pos.x + half_sz.x, pos.y - half_sz.y), ImVec2::new(pos.x + half_sz.x, pos.y + half_sz.y), pos, col); return;
    case ImGuiDir_Right: draw_list.AddTriangleFilled(ImVec2::new(pos.x - half_sz.x, pos.y + half_sz.y), ImVec2::new(pos.x - half_sz.x, pos.y - half_sz.y), pos, col); return;
    case ImGuiDir_Up:    draw_list.AddTriangleFilled(ImVec2::new(pos.x + half_sz.x, pos.y + half_sz.y), ImVec2::new(pos.x - half_sz.x, pos.y + half_sz.y), pos, col); return;
    case ImGuiDir_Down:  draw_list.AddTriangleFilled(ImVec2::new(pos.x - half_sz.x, pos.y - half_sz.y), ImVec2::new(pos.x + half_sz.x, pos.y - half_sz.y), pos, col); return;
    case ImGuiDir_None: case ImGuiDir_COUNT: break; // Fix warnings
    }
}

// This is less wide than RenderArrow() and we use in dock nodes instead of the regular RenderArrow() to denote a change of functionality,
// and because the saved space means that the left-most tab label can stay at exactly the same position as the label of a loose window.
c_void RenderArrowDockMenu(draw_list: *mut ImDrawList, p_min: ImVec2,sz: c_float, col: u32)
{
    draw_list.AddRectFilled(p_min + ImVec2::new(sz * 0.20, sz * 0.150), p_min + ImVec2::new(sz * 0.80, sz * 0.300), col);
    RenderArrowPointingAt(draw_list, p_min + ImVec2::new(sz * 0.50, sz * 0.850), ImVec2::new(sz * 0.3, sz * 0.400), ImGuiDir_Down, col);
}

pub fn ImAcosX(x: c_float) -> c_float {
    if x <= 0 { return IM_PI * 0.5; };
    if x >= 1 { return 0.0; };
    return ImAcos(x);
    //return (-0.69813170079773212 * x * x - 0.872664625997164770) * x + 1.5707963267948966; // Cheap approximation, may be enough for what we do.
}

// FIXME: Cleanup and move code to ImDrawList.
c_void RenderRectFilledRangeH(draw_list: *mut ImDrawList, rect: &ImRect, col: u32,x_start_norm: c_float,x_end_norm: c_float,rounding: c_float)
{
    if (x_end_norm == x_start_norm)
        return;
    if (x_start_norm > x_end_norm)
        ImSwap(x_start_norm, x_end_norm);

    let p0: ImVec2 = ImVec2::new(ImLerp(rect.Min.x, rect.Max.x, x_start_norm), rect.Min.y);
    let p1: ImVec2 = ImVec2::new(ImLerp(rect.Min.x, rect.Max.x, x_end_norm), rect.Max.y);
    if (rounding == 0)
    {
        draw_list.AddRectFilled(p0, p1, col, 0);
        return;
    }

    rounding = ImClamp(ImMin((rect.Max.x - rect.Min.x) * 0.5, (rect.Max.y - rect.Min.y) * 0.5) - 1, 0, rounding);
    let inv_rounding: c_float =  1 / rounding;
    let arc0_b: c_float =  ImAcos01(1 - (p0.x - rect.Min.x) * inv_rounding);
    let arc0_e: c_float =  ImAcos01(1 - (p1.x - rect.Min.x) * inv_rounding);
    let half_pi: c_float =  IM_PI * 0.5; // We will == compare to this because we know this is the exact value ImAcos01 can return.
    let x0: c_float =  ImMax(p0.x, rect.Min.x + rounding);
    if (arc0_b == arc0_e)
    {
        draw_list.PathLineTo(ImVec2::new(x0, p1.y));
        draw_list.PathLineTo(ImVec2::new(x0, p0.y));
    }
    else if (arc0_b == 0 && arc0_e == half_pi)
    {
        draw_list.PathArcToFast(ImVec2::new(x0, p1.y - rounding), rounding, 3, 6); // BL
        draw_list.PathArcToFast(ImVec2::new(x0, p0.y + rounding), rounding, 6, 9); // TR
    }
    else
    {
        draw_list.PathArcTo(ImVec2::new(x0, p1.y - rounding), rounding, IM_PI - arc0_e, IM_PI - arc0_b, 3); // BL
        draw_list.PathArcTo(ImVec2::new(x0, p0.y + rounding), rounding, IM_PI + arc0_b, IM_PI + arc0_e, 3); // TR
    }
    if (p1.x > rect.Min.x + rounding)
    {
        let arc1_b: c_float =  ImAcos01(1 - (rect.Max.x - p1.x) * inv_rounding);
        let arc1_e: c_float =  ImAcos01(1 - (rect.Max.x - p0.x) * inv_rounding);
        let x1: c_float =  ImMin(p1.x, rect.Max.x - rounding);
        if (arc1_b == arc1_e)
        {
            draw_list.PathLineTo(ImVec2::new(x1, p0.y));
            draw_list.PathLineTo(ImVec2::new(x1, p1.y));
        }
        else if (arc1_b == 0 && arc1_e == half_pi)
        {
            draw_list.PathArcToFast(ImVec2::new(x1, p0.y + rounding), rounding, 9, 12); // TR
            draw_list.PathArcToFast(ImVec2::new(x1, p1.y - rounding), rounding, 0, 3);  // BR
        }
        else
        {
            draw_list.PathArcTo(ImVec2::new(x1, p0.y + rounding), rounding, -arc1_e, -arc1_b, 3); // TR
            draw_list.PathArcTo(ImVec2::new(x1, p1.y - rounding), rounding, +arc1_b, +arc1_e, 3); // BR
        }
    }
    draw_list.PathFillConvex(col);
}

c_void RenderRectFilledWithHole(draw_list: *mut ImDrawList, outer: &ImRect, inner: &ImRect, col: u32,rounding: c_float)
{
    let fill_L: bool = (inner.Min.x > outer.Min.x);
    let fill_R: bool = (inner.Max.x < outer.Max.x);
    let fill_U: bool = (inner.Min.y > outer.Min.y);
    let fill_D: bool = (inner.Max.y < outer.Max.y);
    if (fill_L) draw_list.AddRectFilled(ImVec2::new(outer.Min.x, inner.Min.y), ImVec2::new(inner.Min.x, inner.Max.y), col, rounding, ImDrawFlags_RoundCornersNone | (fill_U ? 0 : ImDrawFlags_RoundCornersTopLeft)    | (fill_D ? 0 : ImDrawFlags_RoundCornersBottomLeft));
    if (fill_R) draw_list.AddRectFilled(ImVec2::new(inner.Max.x, inner.Min.y), ImVec2::new(outer.Max.x, inner.Max.y), col, rounding, ImDrawFlags_RoundCornersNone | (fill_U ? 0 : ImDrawFlags_RoundCornersTopRight)   | (fill_D ? 0 : ImDrawFlags_RoundCornersBottomRight));
    if (fill_U) draw_list.AddRectFilled(ImVec2::new(inner.Min.x, outer.Min.y), ImVec2::new(inner.Max.x, inner.Min.y), col, rounding, ImDrawFlags_RoundCornersNone | (fill_L ? 0 : ImDrawFlags_RoundCornersTopLeft)    | (fill_R ? 0 : ImDrawFlags_RoundCornersTopRight));
    if (fill_D) draw_list.AddRectFilled(ImVec2::new(inner.Min.x, inner.Max.y), ImVec2::new(inner.Max.x, outer.Max.y), col, rounding, ImDrawFlags_RoundCornersNone | (fill_L ? 0 : ImDrawFlags_RoundCornersBottomLeft) | (fill_R ? 0 : ImDrawFlags_RoundCornersBottomRight));
    if (fill_L && fill_U) draw_list.AddRectFilled(ImVec2::new(outer.Min.x, outer.Min.y), ImVec2::new(inner.Min.x, inner.Min.y), col, rounding, ImDrawFlags_RoundCornersTopLeft);
    if (fill_R && fill_U) draw_list.AddRectFilled(ImVec2::new(inner.Max.x, outer.Min.y), ImVec2::new(outer.Max.x, inner.Min.y), col, rounding, ImDrawFlags_RoundCornersTopRight);
    if (fill_L && fill_D) draw_list.AddRectFilled(ImVec2::new(outer.Min.x, inner.Max.y), ImVec2::new(inner.Min.x, outer.Max.y), col, rounding, ImDrawFlags_RoundCornersBottomLeft);
    if (fill_R && fill_D) draw_list.AddRectFilled(ImVec2::new(inner.Max.x, inner.Max.y), ImVec2::new(outer.Max.x, outer.Max.y), col, rounding, ImDrawFlags_RoundCornersBottomRight);
}

ImDrawFlags CalcRoundingFlagsForRectInRect(r_in: &ImRect, r_outer: &ImRect,threshold: c_float)
{
    let mut round_l: bool =  r_in.Min.x <= r_outer.Min.x + threshold;
    let mut round_r: bool =  r_in.Max.x >= r_outer.Max.x - threshold;
    let mut round_t: bool =  r_in.Min.y <= r_outer.Min.y + threshold;
    let mut round_b: bool =  r_in.Max.y >= r_outer.Max.y - threshold;
    return ImDrawFlags_RoundCornersNone
        | ((round_t && round_l) ? ImDrawFlags_RoundCornersTopLeft : 0) | ((round_t && round_r) ? ImDrawFlags_RoundCornersTopRight : 0)
        | ((round_b && round_l) ? ImDrawFlags_RoundCornersBottomLeft : 0) | ((round_b && round_r) ? ImDrawFlags_RoundCornersBottomRight : 0);
}

// Helper for ColorPicker4()
// NB: This is rather brittle and will show artifact when rounding this enabled if rounded corners overlap multiple cells. Caller currently responsible for avoiding that.
// Spent a non reasonable amount of time trying to getting this right for ColorButton with rounding+anti-aliasing+ImGuiColorEditFlags_HalfAlphaPreview flag + various grid sizes and offsets, and eventually gave up... probably more reasonable to disable rounding altogether.
// FIXME: uses GetColorU32
c_void RenderColorRectWithAlphaCheckerboard(draw_list: *mut ImDrawList, p_min: ImVec2, p_max: ImVec2, col: u32,grid_step: c_float, grid_off: ImVec2,rounding: c_float, ImDrawFlags flags)
{
    if ((flags & ImDrawFlags_RoundCornersMask_) == 0)
        flags = ImDrawFlags_RoundCornersDefault_;
    if (((col & IM_COL32_A_MASK) >> IM_COL32_A_SHIFT) < 0xF0)
    {
        col_bg1: u32 = GetColorU32(ImAlphaBlendColors(IM_COL32(204, 204, 204, 255), col));
        col_bg2: u32 = GetColorU32(ImAlphaBlendColors(IM_COL32(128, 128, 128, 255), col));
        draw_list.AddRectFilled(p_min, p_max, col_bg1, rounding, flags);

        let yi: c_int = 0;
        for (let y: c_float =  p_min.y + grid_off.y; y < p_max.y; y += grid_step, yi++)
        {
            let y1: c_float =  ImClamp(y, p_min.y, p_max.y), y2 = ImMin(y + grid_step, p_max.y);
            if (y2 <= y1)
                continue;
            for (let x: c_float =  p_min.x + grid_off.x + (yi & 1) * grid_step; x < p_max.x; x += grid_step * 2.00)
            {
                let x1: c_float =  ImClamp(x, p_min.x, p_max.x), x2 = ImMin(x + grid_step, p_max.x);
                if (x2 <= x1)
                    continue;
                ImDrawFlags cell_flags = ImDrawFlags_RoundCornersNone;
                if (y1 <= p_min.y) { if (x1 <= p_min.x) cell_flags |= ImDrawFlags_RoundCornersTopLeft; if (x2 >= p_max.x) cell_flags |= ImDrawFlags_RoundCornersTopRight; }
                if (y2 >= p_max.y) { if (x1 <= p_min.x) cell_flags |= ImDrawFlags_RoundCornersBottomLeft; if (x2 >= p_max.x) cell_flags |= ImDrawFlags_RoundCornersBottomRight; }

                // Combine flags
                cell_flags = (flags == ImDrawFlags_RoundCornersNone || cell_flags == ImDrawFlags_RoundCornersNone) ? ImDrawFlags_RoundCornersNone : (cell_flags & flags);
                draw_list.AddRectFilled(ImVec2::new(x1, y1), ImVec2::new(x2, y2), col_bg2, rounding, cell_flags);
            }
        }
    }
    else
    {
        draw_list.AddRectFilled(p_min, p_max, col, rounding, flags);
    }
}

//-----------------------------------------------------------------------------
// [SECTION] Decompression code
//-----------------------------------------------------------------------------
// Compressed with stb_compress() then converted to a C array and encoded as base85.
// Use the program in misc/fonts/binary_to_compressed_c.cpp to create the array from a TTF file.
// The purpose of encoding as base85 instead of "0x00,0x01,..." style is only save on _source code_ size.
// Decompression from stb.h (public domain) by Sean Barrett https://github.com/nothings/stb/blob/master/stb.h
//-----------------------------------------------------------------------------

static stb_decompress_length: c_uint(const c_uchar *input)
{
    return (input[8] << 24) + (input[9] << 16) + (input[10] << 8) + input[11];
}

static c_uchar *stb__barrier_out_e, *stb__barrier_out_b;
static const c_uchar *stb__barrier_in_b;
static c_uchar *stb__dout;
static c_void stb__match(const c_uchar *data, length: c_uint)
{
    // INVERSE of memmove... write each byte before copying the next...
    // IM_ASSERT(stb__dout + length <= stb__barrier_out_e);
    if (stb__dout + length > stb__barrier_out_e) { stb__dout += length; return; }
    if (data < stb__barrier_out_b) { stb__dout = stb__barrier_out_e+1; return; }
    while (length--) *stb__dout++ = *data+= 1;
}

static c_void stb__lit(const c_uchar *data, length: c_uint)
{
    // IM_ASSERT(stb__dout + length <= stb__barrier_out_e);
    if (stb__dout + length > stb__barrier_out_e) { stb__dout += length; return; }
    if (data < stb__barrier_in_b) { stb__dout = stb__barrier_out_e+1; return; }
    memcpy(stb__dout, data, length);
    stb__dout += length;
}

// #define stb__in2(x)   ((i[x] << 8) + i[(x)+1])
// #define stb__in3(x)   ((i[x] << 16) + stb__in2((x)+1))
// #define stb__in4(x)   ((i[x] << 24) + stb__in3((x)+1))

static const c_uchar *stb_decompress_token(const c_uchar *i)
{
    if (*i >= 0x20) { // use fewer if's for cases that expand small
        if (*i >= 0x80)       stb__match(stb__dout-i[1]-1, i[0] - 0x80 + 1), i += 2;
        else if (*i >= 0x40)  stb__match(stb__dout-(stb__in2(0) - 0x4000 + 1), i[2]+1), i += 3;
        else /* *i >= 0x20 */ stb__lit(i+1, i[0] - 0x20 + 1), i += 1 + (i[0] - 0x20 + 1);
    } else { // more ifs for cases that expand large, since overhead is amortized
        if (*i >= 0x18)       stb__match(stb__dout-(stb__in3(0) - 0x180000 + 1), i[3]+1), i += 4;
        else if (*i >= 0x10)  stb__match(stb__dout-(stb__in3(0) - 0x100000 + 1), stb__in2(3)+1), i += 5;
        else if (*i >= 0x08)  stb__lit(i+2, stb__in2(0) - 0x0800 + 1), i += 2 + (stb__in2(0) - 0x0800 + 1);
        else if (*i == 0x07)  stb__lit(i+3, stb__in2(1) + 1), i += 3 + (stb__in2(1) + 1);
        else if (*i == 0x06)  stb__match(stb__dout-(stb__in3(1)+1), i[4]+1), i += 5;
        else if (*i == 0x04)  stb__match(stb__dout-(stb__in3(1)+1), stb__in2(4)+1), i += 6;
    }
    return i;
}

static stb_adler32: c_uint(adler32: c_uint, c_uchar *buffer, buflen: c_uint)
{
    const unsigned long ADLER_MOD = 65521;
    unsigned long s1 = adler32 & 0xffff, s2 = adler32 >> 16;
    unsigned long blocklen = buflen % 5552;

    unsigned long i;
    while (buflen) {
        for (i=0; i + 7 < blocklen; i += 8) {
            s1 += buffer[0], s2 += s1;
            s1 += buffer[1], s2 += s1;
            s1 += buffer[2], s2 += s1;
            s1 += buffer[3], s2 += s1;
            s1 += buffer[4], s2 += s1;
            s1 += buffer[5], s2 += s1;
            s1 += buffer[6], s2 += s1;
            s1 += buffer[7], s2 += s1;

            buffer += 8;
        }

        for (; i < blocklen; ++i)
            s1 += *buffer++, s2 += s1;

        s1 %= ADLER_MOD, s2 %= ADLER_MOD;
        buflen -= blocklen;
        blocklen = 5552;
    }
    return (s2 << 16) + s1;
}

static stb_decompress: c_uint(c_uchar *output, const c_uchar *i, c_uint /*length*/)
{
    if (stb__in4(0) != 0x57bC0000) return 0;
    if (stb__in4(4) != 0)          return 0; // error! stream is > 4GB
    const let mut olen: c_uint =  stb_decompress_length(i);
    stb__barrier_in_b = i;
    stb__barrier_out_e = output + olen;
    stb__barrier_out_b = output;
    i += 16;

    stb__dout = output;
    for (;;) {
        const c_uchar *old_i = i;
        i = stb_decompress_token(i);
        if (i == old_i) {
            if (*i == 0x05 && i[1] == 0xfa) {
                // IM_ASSERT(stb__dout == output + olen);
                if (stb__dout != output + olen) return 0;
                if (stb_adler32(1, output, olen) !=  stb__in4(2))
                    return 0;
                return olen;
            } else {
                // IM_ASSERT(0); /* NOTREACHED */
                return 0;
            }
        }
        // IM_ASSERT(stb__dout <= output + olen);
        if (stb__dout > output + olen)
            return 0;
    }
}

//-----------------------------------------------------------------------------
// [SECTION] Default font data (ProggyClean.tt0)
//-----------------------------------------------------------------------------
// ProggyClean.ttf
// Copyright (c) 2004, 2005 Tristan Grimmer
// MIT license (see License.txt in http://www.upperbounds.net/download/ProggyClean.ttf.zip)
// Download and more information at http://upperbounds.net
//-----------------------------------------------------------------------------
// File: 'ProggyClean.ttf' (41208 bytes)
// Exported using misc/fonts/binary_to_compressed_c.cpp (with compression + base85 string encoding).
// The purpose of encoding as base85 instead of "0x00,0x01,..." style is only save on _source code_ size.
//-----------------------------------------------------------------------------
static const  proggy_clean_ttf_compressed_data_base85: c_char[11980 + 1] =
    "7])#######hV0qs'/###[),##/l:$#Q6>##5[n42>c-TH`->>#/e>11NNV=Bv(*:.F?uu#(gRU.o0XGH`$vhLG1hxt9?W`#,5LsCp#-i>.r$<$6pD>Lb';9Crc6tgXmKVeU2cD4Eo3R/"
    "2*>]b(MC;$jPfY.;h^`IWM9<Lh2TlS+f-s$o6Q<BWH`YiU.xfLq$N;$0iR/GX:U(jcW2p/W*q?-qmnUCI;jHSAiFWM.R*kU@C=GH?a9wp8$e.-4^Qg1)Q-GL(lf(r/7GrRgwV%MS=C#"
    "`8ND>Qo#t'X#(v#Y9w0#1D$CIf;W'#pWUPXOuxXuU(H9M(1<q-UE31#^-V'8IRUo7Qf./L>=Ke$$'5F%)]0^#0X@U.a<r:QLtFsLcL6##lOj)#.Y5<-R&KgLwqJfLgN&;Q?gI^#DY2uL"
    "i@^rMl9t=cWq6##weg>$FBjVQTSDgEKnIS7EM9>ZY9w0#L;>>#Mx&4Mvt//L[MkA#W@lK.N'[0#7RL_&#w+F%HtG9M#XL`N&.,GM4Pg;-<nLENhvx>-VsM.M0rJfLH2eTM`*oJMHRC`N"
    "kfimM2J,W-jXS:)r0wK#@Fge$U>`w'N7G#$#fB#$E^$#:9:hk+eOe--6x)F7*E%?76%^GMHePW-Z5l'&GiF#$956:rS?dA#fiK:)Yr+`&#0j@'DbG&#^$PG.Ll+DNa<XCMKEV*N)LN/N"
    "*b=%Q6pia-Xg8I$<MR&,VdJe$<(7G;Ckl'&hF;;$<_=X(b.RS%%)###MPBuuE1V:v&cX&#2m#(&cV]`k9OhLMbn%s$G2,B$BfD3X*sp5#l,$R#]x_X1xKX%b5U*[r5iMfUo9U`N99hG)"
    "tm+/Us9pG)XPu`<0s-)WTt(gCRxIg(%6sfh=ktMKn3j)<6<b5Sk_/0(^]AaN#(p/L>&VZ>1i%h1S9u5o@YaaW$e+b<TWFn/Z:Oh(Cx2$lNEoN^e)#CFY@@I;BOQ*sRwZtZxRcU7uW6CX"
    "ow0i(?$Q[cjOd[P4d)]>ROPOpxTO7Stwi1::iB1q)C_=dV26J;2,]7op$]uQr@_V7$q^%lQwtuHY]=DX,n3L#0PHDO49>dC@O>HBuKPpP*E,N+b3L#lpR/MrTEH.IAQk.a>D[.e;mc."
    "x]Ip.PH^'/aqUO/$1WxLoW0[iLA<QT;5HKD+@qQ'NQ(3_PLhE48R.qAPSwQ0/WK?Z,[x?-J;jQTWA0X@KJ(_Y8N-:/M74:/-ZpKrUss?d#dZq]DAbkU*JqkL+nwX@@47`5>w=4h(9.`G"
    "CRUxHPeR`5Mjol(dUWxZa(>STrPkrJiWx`5U7F#.g*jrohGg`cg:lSTvEY/EV_7H4Q9[Z%cnv;JQYZ5q.l7Zeas:HOIZOB?G<Nald$qs]@]L<J7bR*>gv:[7MI2k).'2($5FNP&EQ(,)"
    "U]W]+fh18.vsai00);D3@4ku5P?DP8aJt+;qUM]=+b'8@;mViBKx0DE[-auGl8:PJ&Dj+M6OC]O^((##]`0i)drT;-7X`=-H3[igUnPG-NZlo.#k@h#=Ork$m>a>$-?Tm$UV(?#P6YY#"
    "'/###xe7q.73rI3*pP/$1>s9)W,JrM7SN]'/4C#v$U`0#V.[0>xQsH$fEmPMgY2u7Kh(G%siIfLSoS+MK2eTM$=5,M8p`A.;_R%#u[K#$x4AG8.kK/HSB==-'Ie/QTtG?-.*^N-4B/ZM"
    "_3YlQC7(p7q)&](`6_c)$/*JL(L-^(]$wIM`dPtOdGA,U3:w2M-0<q-]L_?^)1vw'.,MRsqVr.L;aN&#/EgJ)PBc[-f>+WomX2u7lqM2iEumMTcsF?-aT=Z-97UEnXglEn1K-bnEO`gu"
    "Ft(c%=;Am_Qs@jLooI&NX;]0#j4#F14;gl8-GQpgwhrq8'=l_f-b49'UOqkLu7-##oDY2L(te+Mch&gLYtJ,MEtJfLh'x'M=$CS-ZZ%P]8bZ>#S?YY#%Q&q'3^Fw&?D)UDNrocM3A76/"
    "/oL?#h7gl85[qW/NDOk%16ij;+:1a'iNIdb-ou8.P*w,v5#EI$TWS>Pot-R*H'-SEpA:g)f+O$%%`kA#G=8RMmG1&O`>to8bC]T&$,n.LoO>29sp3dt-52U%VM#q7'DHpg+#Z9%H[K<L"
    "%a2E-grWVM3@2=-k22tL]4$##6We'8UJCKE[d_=%wI;'6X-GsLX4j^SgJ$##R*w,vP3wK#iiW&#*h^D&R?jp7+/u&#(AP##XU8c$fSYW-J95_-Dp[g9wcO&#M-h1OcJlc-*vpw0xUX&#"
    "OQFKNX@QI'IoPp7nb,QU//MQ&ZDkKP)X<WSVL(68uVl&#c'[0#(s1X&xm$Y%B7*K:eDA323j998GXbA#pwMs-jgD$9QISB-A_(aN4xoFM^@C58D0+Q+q3n0#3U1InDjF682-SjMXJK)("
    "h$hxua_K]ul92%'BOU&#BRRh-slg8KDlr:%L71Ka:.A;%YULjDPmL<LYs8i#XwJOYaKPKc1h:'9Ke,g)b),78=I39B;xiY$bgGw-&.Zi9InXDuYa%G*f2Bq7mn9^#p1vv%#(Wi-;/Z5h"
    "o;#2:;%d&#x9v68C5g?ntX0X)pT`;%pB3q7mgGN)3%(P8nTd5L7GeA-GL@+%J3u2:(Yf>et`e;)f#Km8&+DC$I46>#Kr]]u-[=99tts1.qb#q72g1WJO81q+eN'03'eM>&1XxY-caEnO"
    "j%2n8)),?ILR5^.Ibn<-X-Mq7[a82Lq:F&#ce+S9wsCK*x`569E8ew'He]h:sI[2LM$[guka3ZRd6:t%IG:;$%YiJ:Nq=?eAw;/:nnDq0(CYcMpG)qLN4$##&J<j$UpK<Q4a1]MupW^-"
    "sj_$%[HK%'F####QRZJ::Y3EGl4'@%FkiAOg#p[##O`gukTfBHagL<LHw%q&OV0##F=6/:chIm0@eCP8X]:kFI%hl8hgO@RcBhS-@Qb$%+m=hPDLg*%K8ln(wcf3/'DW-$.lR?n[nCH-"
    "eXOONTJlh:.RYF%3'p6sq:UIMA945&^HFS87@$EP2iG<-lCO$%c`uKGD3rC$x0BL8aFn--`ke%#HMP'vh1/R&O_J9'um,.<tx[@%wsJk&bUT2`0uMv7gg#qp/ij.L56'hl;.s5CUrxjO"
    "M7-##.l+Au'A&O:-T72L]P`&=;ctp'XScX*rU.>-XTt,%OVU4)S1+R-#dg0/Nn?Ku1^0$B*P:Rowwm-`0PKjYDDM'3]d39VZHEl4,.j']Pk-M.h^&:0FACm$maq-&sgw0t7/6(^xtk%"
    "LuH88Fj-ekm>GA#_>568x6(OFRl-IZp`&b,_P'$M<Jnq79VsJW/mWS*PUiq76;]/NM_>hLbxfc$mj`,O;&%W2m`Zh:/)Uetw:aJ%]K9h:TcF]u_-Sj9,VK3M.*'&0D[Ca]J9gp8,kAW]"
    "%(?A%R$f<->Zts'^kn=-^@c4%-pY6qI%J%1IGxfLU9CP8cbPlXv);C=b),<2mOvP8up,UVf3839acAWAW-W?#ao/^#%KYo8RULNd2.>%m]UK:n%r$'sw]J;5pAoO_#2mO3n,'=H5(et"
    "Hg*`+RLgv>=4U8guD$I%D:W>-r5V*%j*W:Kvej.Lp$<M-SGZ':+Q_k+uvOSLiEo(<aD/K<CCc`'Lx>'?;++O'>()jLR-^u68PHm8ZFWe+ej8h:9r6L*0//c&iH&R8pRbA#Kjm%upV1g:"
    "a_#Ur7FuA#(tRh#.Y5K+@?3<-8m0$PEn;J:rh6?I6uG<-`wMU'ircp0LaE_OtlMb&1#6T.#FDKu#1Lw%u%+GM+X'e?YLfjM[VO0MbuFp7;>Q&#WIo)0@F%q7c#4XAXN-U&VB<HFF*qL("
    "$/V,;(kXZejWO`<[5?\?ewY(*9=%wDc;,u<'9t3W-(H1th3+G]ucQ]kLs7df($/*JL]@*t7Bu_G3_7mp7<iaQjO@.kLg;x3B0lqp7Hf,^Ze7-##@/c58Mo(3;knp0%)A7?-W+eI'o8)b<"
    "nKnw'Ho8C=Y>pqB>0ie&jhZ[?iLR@@_AvA-iQC(=ksRZRVp7`.=+NpBC%rh&3]R:8XDmE5^V8O(x<<aG/1N$#FX$0V5Y6x'aErI3I$7x%E`v<-BY,)%-?Psf*l?%C3.mM(=/M0:JxG'?"
    "7WhH%o'a<-80g0NBxoO(GH<dM]n.+%q@jH?f.UsJ2Ggs&4<-e47&Kl+f//9@`b+?.TeN_&B8Ss?v;^Trk;f#YvJkl&w$]>-+k?'(<S:68tq*WoDfZu';mM?8X[ma8W%*`-=;D.(nc7/;"
    ")g:T1=^J$&BRV(-lTmNB6xqB[@0*o.erM*<SWF]u2=st-*(6v>^](H.aREZSi,#1:[IXaZFOm<-ui#qUq2$##Ri;u75OK#(RtaW-K-F`S+cF]uN`-KMQ%rP/Xri.LRcB##=YL3BgM/3M"
    "D?@f&1'BW-)Ju<L25gl8uhVm1hL$##*8###'A3/LkKW+(^rWX?5W_8g)a(m&K8P>#bmmWCMkk&#TR`C,5d>g)F;t,4:@_l8G/5h4vUd%&%950:VXD'QdWoY-F$BtUwmfe$YqL'8(PWX("
    "P?^@Po3$##`MSs?DWBZ/S>+4%>fX,VWv/w'KD`LP5IbH;rTV>n3cEK8U#bX]l-/V+^lj3;vlMb&[5YQ8#pekX9JP3XUC72L,,?+Ni&co7ApnO*5NK,((W-i:$,kp'UDAO(G0Sq7MVjJs"
    "bIu)'Z,*[>br5X^:FPAWr-m2KgL<LUN098kTF&#lvo58=/vjDo;.;)Ka*hLR#/k=rKbxuV`>Q_nN6'8uTG&#1T5g)uLv:873UpTLgH+#FgpH'_o1780Ph8KmxQJ8#H72L4@768@Tm&Q"
    "h4CB/5OvmA&,Q&QbUoi$a_%3M01H)4x7I^&KQVgtFnV+;[Pc>[m4k//,]1?#`VY[Jr*3&&slRfLiVZJ:]?=K3Sw=[$=uRB?3xk48@aeg<Z'<$#4H)6,>e0jT6'N#(q%.O=?2S]u*(m<-"
    "V8J'(1)G][68hW$5'q[GC&5j`TE?m'esFGNRM)j,ffZ?-qx8;->g4t*:CIP/[Qap7/9'#(1sao7w-.qNUdkJ)tCF&#B^;xGvn2r9FEPFFFcL@.iFNkTve$m%#QvQS8U@)2Z+3K:AKM5i"
    "sZ88+dKQ)W6>J%CL<KE>`.d*(B`-n8D9oK<Up]c$X$(,)M8Zt7/[rdkqTgl-0cuGMv'?>-XV1q['-5k'cAZ69e;D_?$ZPP&s^+7])$*$#@QYi9,5P&#9r+$%CE=68>K8r0=dSC%%(@p7"
    ".m7jilQ02'0-VWAg<a/''3u.=4L$Y)6k/K:_[3=&jvL<L0C/2'v:^;-DIBW,B4E68:kZ;%?8(Q8BH=kO65BW?xSG&#@uU,DS*,?.+(o(#1vCS8#CHF>TlGW'b)Tq7VT9q^*^$$.:&N@@"
    "$&)WHtPm*5_rO0&e%K&#-30j(E4#'Zb.o/(Tpm$>K'f@[PvFl,hfINTNU6u'0pao7%XUp9]5.>%h`8_=VYbxuel.NTSsJfLacFu3B'lQSu/m6-Oqem8T+oE--$0a/k]uj9EwsG>%veR*"
    "hv^BFpQj:K'#SJ,sB-'#](j.Lg92rTw-*n%@/;39rrJF,l#qV%OrtBeC6/,;qB3ebNW[?,Hqj2L.1NP&GjUR=1D8QaS3Up&@*9wP?+lo7b?@%'k4`p0Z$22%K3+iCZj?XJN4Nm&+YF]u"
    "@-W$U%VEQ/,,>>#)D<h#`)h0:<Q6909ua+&VU%n2:cG3FJ-%@Bj-DgLr`Hw&HAKjKjseK</xKT*)B,N9X3]krc12t'pgTV(Lv-tL[xg_%=M_q7a^x?7Ubd>#%8cY#YZ?=,`Wdxu/ae&#"
    "w6)R89tI#6@s'(6Bf7a&?S=^ZI_kS&ai`&=tE72L_D,;^R)7[$s<Eh#c&)q.MXI%#v9ROa5FZO%sF7q7Nwb&#ptUJ:aqJe$Sl68%.D###EC><?-aF&#RNQv>o8lKN%5/$(vdfq7+ebA#"
    "u1p]ovUKW&Y%q]'>$1@-[xfn$7ZTp7mM,G,Ko7a&Gu%G[RMxJs[0MM%wci.LFDK)(<c`Q8N)jEIF*+?P2a8g%)$q]o2aH8C&<SibC/q,(e:v;-b#6[$NtDZ84Je2KNvB#$P5?tQ3nt(0"
    "d=j.LQf./Ll33+(;q3L-w=8dX$#WF&uIJ@-bfI>%:_i2B5CsR8&9Z&#=mPEnm0`<&c)QL5uJ#%u%lJj+D-r;BoF&#4DoS97h5g)E#o:&S4weDF,9^Hoe`h*L+_a*NrLW-1pG_&2UdB8"
    "6e%B/:=>)N4xeW.*wft-;$'58-ESqr<b?UI(_%@[P46>#U`'6AQ]m&6/`Z>#S?YY#Vc;r7U2&326d=w&H####?TZ`*4?&.MK?LP8Vxg>$[QXc%QJv92.(Db*B)gb*BM9dM*hJMAo*c&#"
    "b0v=Pjer]$gG&JXDf->'StvU7505l9$AFvgYRI^&<^b68?j#q9QX4SM'RO#&sL1IM.rJfLUAj221]d##DW=m83u5;'bYx,*Sl0hL(W;;$doB&O/TQ:(Z^xBdLjL<Lni;''X.`$#8+1GD"
    ":k$YUWsbn8ogh6rxZ2Z9]%nd+>V#*8U_72Lh+2Q8Cj0i:6hp&$C/:p(HK>T8Y[gHQ4`4)'$Ab(Nof%V'8hL&#<NEdtg(n'=S1A(Q1/I&4([%dM`,Iu'1:_hL>SfD07&6D<fp8dHM7/g+"
    "tlPN9J*rKaPct&?'uBCem^jn%9_K)<,C5K3s=5g&GmJb*[SYq7K;TRLGCsM-$$;S%:Y@r7AK0pprpL<Lrh,q7e/%KWK:50I^+m'vi`3?%Zp+<-d+$L-Sv:@.o19n$s0&39;kn;S%BSq*"
    "$3WoJSCLweV[aZ'MQIjO<7;X-X;&+dMLvu#^UsGEC9WEc[X(wI7#2.(F0jV*eZf<-Qv3J-c+J5AlrB#$p(H68LvEA'q3n0#m,[`*8Ft)FcYgEud]CWfm68,(aLA$@EFTgLXoBq/UPlp7"
    ":d[/;r_ix=:TF`S5H-b<LI&HY(K=h#)]Lk$K14lVfm:x$H<3^Ql<M`$OhapBnkup'D#L$Pb_`N*g]2e;X/Dtg,bsj&K#2[-:iYr'_wgH)NUIR8a1n#S?Yej'h8^58UbZd+^FKD*T@;6A"
    "7aQC[K8d-(v6GI$x:T<&'Gp5Uf>@M.*J:;$-rv29'M]8qMv-tLp,'886iaC=Hb*YJoKJ,(j%K=H`K.v9HggqBIiZu'QvBT.#=)0ukruV&.)3=(^1`o*Pj4<-<aN((^7('#Z0wK#5GX@7"
    "u][`*S^43933A4rl][`*O4CgLEl]v$1Q3AeF37dbXk,.)vj#x'd`;qgbQR%FW,2(?LO=s%Sc68%NP'##Aotl8x=BE#j1UD([3$M(]UI2LX3RpKN@;/#f'f/&_mt&0)XdF<9t4)Qa.*kT"
    "LwQ'(TTB9.xH'>#MJ+gLq9-##@HuZPN0]u:h7.T..G:;$/Usj(T7`Q8tT72LnYl<-qx8;-HV7Q-&Xdx%1a,hC=0u+HlsV>nuIQL-5<N?)NBS)QN*_I,?&)2'IM%L3I)X((e/dl2&8'<M"
    ":^#M*Q+[T.Xri.LYS3v%fF`68h;b-X[/En'CR.q7E)p'/kle2HM,u;^%OKC-N+Ll%F9CF<Nf'^#t2L,;27W:0O@6##U6W7:$rJfLWHj$#)woqBefIZ.PK<b*t7ed;p*_m;4ExK#h@&]>"
    "_>@kXQtMacfD.m-VAb8;IReM3$wf0''hra*so568'Ip&vRs849'MRYSp%:t:h5qSgwpEr$B>Q,;s(C#$)`svQuF$##-D,##,g68@2[T;.XSdN9Qe)rpt._K-#5w0)sP'##p#C0c%-Gb%"
    "hd+<-j'Ai*x&&HMkT]C'OSl##5RG[JXaHN;d'uA#x._U;.`PU@(Z3dt4r152@:v,'R.Sj'w#0<-;kPI)FfJ&#AYJ&#//)>-k=m=*XnK$>=)72L]0I%>.G690a:$##<,);?;72#?x9+d;"
    "^V'9;jY@;)br#q^YQpx:X#Te$Z^'=-=bGhLf:D6&bNwZ9-ZD#n^9HhLMr5G;']d&6'wYmTFmL<LD)F^%[tC'8;+9E#C$g%#5Y>q9wI>P(9mI[>kC-ekLC/R&CH+s'B;K-M6$EB%is00:"
    "+A4[7xks.LrNk0&E)wILYF@2L'0Nb$+pv<(2.768/FrY&h$^3i&@+G%JT'<-,v`3;_)I9M^AE]CN?Cl2AZg+%4iTpT3<n-&%H%b<FDj2M<hH=&Eh<2Len$b*aTX=-8QxN)k11IM1c^j%"
    "9s<L<NFSo)B?+<-(GxsF,^-Eh@$4dXhN$+#rxK8'je'D7k`e;)2pYwPA'_p9&@^18ml1^[@g4t*[JOa*[=Qp7(qJ_oOL^('7B&Hq-:sf,sNj8xq^>$U4O]GKx'm9)b@p7YsvK3w^YR-"
    "CdQ*:Ir<($u&)#(&?L9Rg3H)4iEp^iI9O8KnTj,]H?D*r7'M;PwZ9K0E^k&-cpI;.p/6_vwoFMV<->#%Xi.LxVnrU(4&8/P+:hLSKj$#U%]49t'I:rgMi'FL@a:0Y-uA[39',(vbma*"
    "hU%<-SRF`Tt:542R_VV$p@[p8DV[A,?1839FWdF<TddF<9Ah-6&9tWoDlh]&1SpGMq>Ti1O*H&#(AL8[_P%.M>v^-))qOT*F5Cq0`Ye%+$B6i:7@0IX<N+T+0MlMBPQ*Vj>SsD<U4JHY"
    "8kD2)2U/M#$e.)T4,_=8hLim[&);?UkK'-x?'(:siIfL<$pFM`i<?%W(mGDHM%>iWP,##P`%/L<eXi:@Z9C.7o=@(pXdAO/NLQ8lPl+HPOQa8wD8=^GlPa8TKI1CjhsCTSLJM'/Wl>-"
    "S(qw%sf/@%#B6;/U7K]uZbi^Oc^2n<bhPmUkMw>%t<)'mEVE''n`WnJra$^TKvX5B>;_aSEK',(hwa0:i4G?.Bci.(X[?b*($,=-n<.Q%`(X=?+@Am*Js0&=3bh8K]mL<LoNs'6,'85`"
    "0?t/'_U59@]ddF<#LdF<eWdF<OuN/45rY<-L@&#+fm>69=Lb,OcZV/);TTm8VI;?%OtJ<(b4mq7M6:u?KRdF<gR@2L=FNU-<b[(9c/ML3m;Z[$oF3g)GAWqpARc=<ROu7cL5l;-[A]%/"
    "+fsd;l#SafT/f*W]0=O'$(Tb<[)*@e775R-:Yob%g*>l*:xP?Yb.5)%w_I?7uk5JC+FS(m#i'k.'a0i)9<7b'fs'59hq$*5Uhv##pi^8+hIEBF`nvo`;'l0.^S1<-wUK2/Coh58KKhLj"
    "M=SO*rfO`+qC`W-On.=AJ56>>i2@2LH6A:&5q`?9I3@@'04&p2/LVa*T-4<-i3;M9UvZd+N7>b*eIwg:CC)c<>nO&#<IGe;__.thjZl<%w(Wk2xmp4Q@I#I9,DF]u7-P=.-_:YJ]aS@V"
    "?6*C()dOp7:WL,b&3Rg/.cmM9&r^>$(>.Z-I&J(Q0Hd5Q%7Co-b`-c<N(6r@ip+AurK<m86QIth*#v;-OBqi+L7wDE-Ir8K['m+DDSLwK&/.?-V%U_%3:qKNu$_b*B-kp7NaD'QdWQPK"
    "Yq[@>P)hI;*_F]u`Rb[.j8_Q/<&>uu+VsH$sM9TA%?)(vmJ80),P7E>)tjD%2L=-t#fK[%`v=Q8<FfNkgg^oIbah*#8/Qt$F&:K*-(N/'+1vMB,u()-a.VUU*#[e%gAAO(S>WlA2);Sa"
    ">gXm8YB`1d@K#n]76-a$U,mF<fX]idqd)<3,]J7JmW4`6]uks=4-72L(jEk+:bJ0M^q-8Dm_Z?0olP1C9Sa&H[d&c$ooQUj]Exd*3ZM@-WGW2%s',B-_M%>%Ul:#/'xoFM9QX-$.QN'>"
    "[%$Z$uF6pA6Ki2O5:8w*vP1<-1`[G,)-m#>0`P&#eb#.3i)rtB61(o'$?X3B</R90;eZ]%Ncq;-Tl]#F>2Qft^ae_5tKL9MUe9b*sLEQ95C&`=G?@Mj=wh*'3E>=-<)Gt*Iw)'QG:`@I"
    "wOf7&]1i'S01B+Ev/Nac#9S;=;YQpg_6U`*kVY39xK,[/6Aj7:'1Bm-_1EYfa1+o&o4hp7KN_Q(OlIo@S%;jVdn0'1<Vc52=u`3^o-n1'g4v58Hj&6_t7$##?M)c<$bgQ_'SY((-xkA#"
    "Y(,p'H9rIVY-b,'%bCPF7.J<Up^,(dU1VY*5#WkTU>h19w,WQhLI)3S#f$2(eb,jr*b;3Vw]*7NH%$c4Vs,eD9>XW8?N]o+(*pgC%/72LV-u<Hp,3@e^9UB1J+ak9-TN/mhKPg+AJYd$"
    "MlvAF_jCK*.O-^(63adMT->W%iewS8W6m2rtCpo'RS1R84=@paTKt)>=%&1[)*vp'u+x,VrwN;&]kuO9JDbg=pO$J*.jVe;u'm0dr9l,<*wMK*Oe=g8lV_KEBFkO'oU]^=[-792#ok,)"
    "i]lR8qQ2oA8wcRCZ^7w/Njh;?.stX?Q1>S1q4Bn$)K1<-rGdO'$Wr.Lc.CG)$/*JL4tNR/,SVO3,aUw'DJN:)Ss;wGn9A32ijw%FL+Z0Fn.U9;reSq)bmI32U==5ALuG&#Vf1398/pVo"
    "1*c-(aY168o<`JsSbk-,1N;$>0:OUas(3:8Z972LSfF8eb=c-;>SPw7.6hn3m`9^Xkn(r.qS[0;T%&Qc=+STRxX'q1BNk3&*eu2;&8q$&x>Q#Q7^Tf+6<(d%ZVmj2bDi%.3L2n+4W'$P"
    "iDDG)g,r%+?,$@?uou5tSe2aN_AQU*<h`e-GI7)?OK2A.d7_c)?wQ5AS@DL3r#7Skgl6-++D:'A,uq7SvlB$pcpH'q3n0#_%dY#xCpr-l<F0NR@-##FEV6NTF6##$l84N1w?AO>'IAO"
    "URQ##V^Fv-XFbGM7Fl(N<3DhLGF%q.1rC$#:T__&Pi68%0xi_&[qFJ(77j_&JWoF.V735&T,[R*:xFR*K5>>#`bW-?4Ne_&6Ne_&6Ne_&n`kr-#GJcM6X;uM6X;uM(.a..^2TkL%oR(#"
    ";u.T%fAr%4tJ8&><1=GHZ_+m9/#H1F^R#SC#*N=BA9(D?v[UiFY>>^8p,KKF.W]L29uLkLlu/+4T<XoIB&hx=T1PcDaB&;HH+-AFr?(m9HZV)FKS8JCw;SD=6[^/DZUL`EUDf]GGlG&>"
    "w$)F./^n3+rlo+DB;5sIYGNk+i1t-69Jg--0pao7Sm#K)pdHW&;LuDNH@H>#/X-TI(;P>#,Gc>#0Su>#4`1?#8lC?#<xU?#@.i?#D:%@#HF7@#LRI@#P_[@#Tkn@#Xw*A#]-=A#a9OA#"
    "d<F&#*;G##.GY##2Sl##6`($#:l:$#>xL$#B.`$#F:r$#JF.%#NR@%#R_R%#Vke%#Zww%#_-4&#3^Rh%Sflr-k'MS.o?.5/sWel/wpEM0%3'/1)K^f1-d>G21&v(35>V`39V7A4=onx4"
    "A1OY5EI0;6Ibgr6M$HS7Q<)58C5w,;WoA*#[%T*#`1g*#d=#+#hI5+#lUG+#pbY+#tnl+#x$),#&1;,#*=M,#.I`,#2Ur,#6b.-#;w[H#iQtA#m^0B#qjBB#uvTB##-hB#'9$C#+E6C#"
    "/QHC#3^ZC#7jmC#;v)D#?,<D#C8ND#GDaD#KPsD#O]/E#g1A5#KA*1#gC17#MGd;#8(02#L-d3#rWM4#Hga1#,<w0#T.j<#O#'2#CYN1#qa^:#_4m3#o@/=#eG8=#t8J5#`+78#4uI-#"
    "m3B2#SB[8#Q0@8#i[*9#iOn8#1Nm;#^sN9#qh<9#:=x-#P;K2#$%X9#bC+.#Rg;<#mN=.#MTF.#RZO.#2?)4#Y#(/#[)1/#b;L/#dAU/#0Sv;#lY$0#n`-0#sf60#(F24#wrH0#%/e0#"
    "TmD<#%JSMFove:CTBEXI:<eh2g)B,3h2^G3i;#d3jD>)4kMYD4lVu`4m`:&5niUA5@(A5BA1]PBB:xlBCC=2CDLXMCEUtiCf&0g2'tN?PGT4CPGT4CPGT4CPGT4CPGT4CPGT4CPGT4CP"
    "GT4CPGT4CPGT4CPGT4CPGT4CPGT4CP-qekC`.9kEg^+F$kwViFJTB&5KTB&5KTB&5KTB&5KTB&5KTB&5KTB&5KTB&5KTB&5KTB&5KTB&5KTB&5KTB&5KTB&5KTB&5o,^<-28ZI'O?;xp"
    "O?;xpO?;xpO?;xpO?;xpO?;xpO?;xpO?;xpO?;xpO?;xpO?;xpO?;xpO?;xpO?;xp;7q-#lLYI:xvD=#";

static GetDefaultCompressedFontDataTTFBase85: *const c_char()
{
    return proggy_clean_ttf_compressed_data_base85;
}

// #endif // #ifndef IMGUI_DISABLE
