#![allow(non_snake_case)]

use std::ffi::CStr;
use std::ptr::null_mut;
use std::str::pattern::Pattern;
use libc::{c_char, c_float, c_int, c_short, c_uint};
use crate::color::IM_COL32_A_MASK;
use crate::draw_list::ImDrawList;
use crate::font_atlas::ImFontAtlas;
use crate::font_config::ImFontConfig;
use crate::font_glyph::ImFontGlyph;
use crate::string_ops::{ImCharIsBlankA, ImTextCharFromUtf8};
use crate::vec2::ImVec2;
use crate::vec4::ImVec4;
use crate::type_defs::ImWchar;

// Font runtime data and rendering
// ImFontAtlas automatically loads a default embedded font for you when you call GetTexDataAsAlpha8() or GetTexDataAsRGBA32().
#[derive(Default, Debug, Clone)]
pub struct ImFont {
    // Members: Hot ~20/24 bytes (for CalcTextSize)
    pub IndexAdvanceX: Vec<c_float>,
    // 12-16 // out //            // Sparse. Glyphs.AdvanceX in a directly indexable way (cache-friendly for CalcTextSize functions which only this this info, and are often bottleneck in large UI).
    pub FallbackAdvanceX: c_float,
    // 4     // out // = FallbackGlyph.AdvanceX
    pub FontSize: c_float,           // 4     // in  //            // Height of characters/line, set during loading (don't change after loading)

    // Members: Hot ~28/40 bytes (for CalcTextSize + render loop)
    pub IndexLookup: Vec<ImWchar>,
    // 12-16 // out //            // Sparse. Index glyphs by Unicode code-point.
    pub Glyphs: Vec<ImFontGlyph>,
    // 12-16 // out //            // All glyphs.
    pub FallbackGlyph: *const ImFontGlyph,      // 4-8   // out // = FindGlyph(FontFallbackChar)

    // Members: Cold ~32/40 bytes
    pub ContainerAtlas: *mut ImFontAtlas,
    // 4-8   // out //            // What we has been loaded into
    pub ConfigData: *const ImFontConfig,
    // 4-8   // in  //            // Pointer within Containeratlas.ConfigData
    pub ConfigDataCount: c_short,
    // 2     // in  // ~ 1        // Number of ImFontConfig involved in creating this font. Bigger than 1 when merging multiple font sources into one ImFont.
    pub FallbackChar: ImWchar,
    // 2     // out // = FFFD/'?' // Character used if a glyph isn't found.
    pub EllipsisChar: ImWchar,
    // 2     // out // = '...'    // Character used for ellipsis rendering.
    pub DotChar: ImWchar,
    // 2     // out // = '.'      // Character used for ellipsis rendering (if a single '...' character isn't found)
    pub DirtyLookupTables: bool,
    // 1     // out //
    pub Scale: c_float,
    // 4     // in  // = 1.f      // Base font scale, multiplied by the per-window font scale which you can adjust with SetWindowFontScale()
// c_float                       Ascent, Descent;    // 4+4   // out //            // Ascent: distance from top to bottom of e.g. 'A' [0..FontSize]
    pub Ascent: c_float,
    pub Descent: c_float,

    pub MetricsTotalSurface: c_int,
    // 4     // out //            // Total surface in pixels to get an idea of the font rasterization/texture cost (not exact, we approximate the cost of padding between glyphs)
// ImU8                        Used4kPagesMap[(IM_UNICODE_CODEPOINT_MAX+1)/4096/8]; // 2 bytes if ImWchar=ImWchar16, 34 bytes if ImWchar==ImWchar32. Store 1-bit for each block of 4K codepoints that has one active glyph. This is mainly used to facilitate iterations across all used codepoints.
    pub Used4kPagesMap: [u8; (IM_UNICODE_CODEPOINT_MAX + 1) / 4096 / 8],

}

impl ImFont {

    // Methods
    // ImFont();


    // ~ImFont();


    // const ImFontGlyph*FindGlyph(ImWchar c) const;
    pub fn FindGlyph(&mut self, c: ImWchar) -> *const ImFontGlyph {
        todo!()
    }


    // const ImFontGlyph*FindGlyphNoFallback(ImWchar c) const;
    pub fn FindGlyphNoFallback(&mut self, c: ImWchar) -> *const ImFontGlyph {
        todo!()
    }


    // c_float                       GetCharAdvance(ImWchar c) const     { return (c < IndexAdvanceX.Size) ? IndexAdvanceX[c] : FallbackAdvanceX; }
    pub fn GetCharAdvance(&self, c: ImWchar) -> c_float {
        return if (c as usize) < self.IndexAdvanceX.len() {
            self.IndexAdvanceX[c]
        } else {
            self.FallbackAdvanceX
        };
    }


    // bool                        IsLoaded() const                    { return ContainerAtlas != None; }
    pub fn IsLoaded(&self) -> bool {
        self.ContainerAtlas.is_null() == false
    }


    // const char*                 GetDebugName() const                { return ConfigData ? ConfigData.Name : "<unknown>"; }
    pub unsafe fn GetDebugName(&self) -> *const c_char {
        return if self.ConfigData.is_null() == false {
            self.ConfigData.Name.as_ptr()
        } else {
            CStr::from_bytes_with_nul_unchecked(String::from("<unknown>").as_bytes()).as_ptr()
        };
    }

    // 'max_width' stops rendering after a certain width (could be turned into a 2d size). f32::MAX to disable.
    // 'wrap_width' enable automatic word-wrapping across multiple lines to fit into given width. 0f32 to disable.
    // ImVec2            CalcTextSizeA(size: c_float, max_width: c_float, wrap_width: c_float, const char* text_begin, const char* text_end = NULL, const char** remaining = NULL) const; // utf8
    pub fn CalcTextSizeA(&mut self, size: c_float, max_width: c_float, wrap_width: c_float, text_begin: *const c_char, text_end: *const c_char, remaining: *mut *const c_char) -> ImVec2 {
        todo!()
    }


    // const char*       CalcWordWrapPositionA(scale: c_float, const char* text, const char* text_end, wrap_width: c_float) const;
    pub fn CalcWordWrapPositionA(&mut self, scale: c_float, text: *const c_char, text_end: *const c_char, wrap_width: c_float) {
        todo!()
    }


    // void              RenderChar(ImDrawList* draw_list, size: c_float, const pos: &ImVec2, u32 col, ImWchar c) const;
    pub fn RenderChar(&mut self, mut draw_list: *const ImDrawList, size: c_float, pos: &ImVec2, mut col: u32, c: ImWchar) {
        let glyph: *const ImFontGlyph = self.FindGlyph(c);
        // TODO: go back and look at C source to see what variable is missing
        // if !glyph || !
        //     {
        //         return;
        //     }
        // if
        // {
        //     col |= !IM_COL32_A_MASK;
        // }
        let scale: c_float =  if size >= 0f32 { (size / self.FontSize) } else { 1f32 };
        let x: c_float =  IM_FLOOR(pos.x);
        let y: c_float =  IM_FLOOR(pos.y);
        draw_list.PrimReserve(6, 4);
        // TODO: go back and look at C source to see what variable is missing
        // draw_list.PrimRectUV(ImVec2::new2(x +  * scale, y +  * scale), ImVec2::new2(x +  * scale, y +  * scale), ImVec2::new2(, ), ImVec2::new2(, ), col);
    }


    // void              RenderText(ImDrawList* draw_list, size: c_float, const pos: &ImVec2, u32 col, const ImVec4& clip_rect, const char* text_begin, const char* text_end, c_float wrap_width = 0f32, bool cpu_fine_clip = false) const;
    pub unsafe fn RenderText(&mut self, mut draw_list: *mut ImDrawList, size: c_float, pos: &ImVec2, col: u32, clip_rect: &ImVec4, text_begin: *const c_char, mut text_end: *const c_char, wrap_width: c_float, cpu_fine_clip: bool) {
        if !text_end
        {
            text_end = text_begin + libc::strlen(text_begin);
        } //  functions generally already provides a valid text_end, so this is merely to handle direct calls.

        // Align to be pixel perfect
        let mut x: c_float =  IM_FLOOR(pos.x);
        let mut y: c_float =  IM_FLOOR(pos.y);
        if y > clip_rect.w
        {
            return;
        }

        let start_x: c_float =  x;
        let scale: c_float =  size / FontSize;
        let line_height: c_float =  FontSize * scale;
        let word_wrap_enabled: bool = (wrap_width > 0f32);
        let mut  word_wrap_eol: *const c_char= null_mut();

        // Fast-forward to first visible line
        let mut  s: *const c_char = text_begin;
        if y + line_height < clip_rect.y && !word_wrap_enabled {
            while y + line_height < clip_rect.y && s < text_end {
                s = libc::memchr(s, '\n' as c_int, text_end - s);
                s = if s {
                    s + 1
                } else { text_end };
                y += line_height;
            }
        }
        // For large text, scan for the last visible line in order to avoid over-reserving in the call to PrimReserve()
        // Note that very large horizontal line will still be affected by the issue (e.g. a one megabyte string buffer without a newline will likely crash atm)
        if text_end - s > 10000 && !word_wrap_enabled
        {
            let mut  s_end: *const c_char = s;
            let mut y_end: c_float =  y;
            while (y_end < clip_rect.w && s_end < text_end)
            {
                s_end =libc::memchr(s_end, '\n' as c_int, text_end - s_end);
                s_end = if s_end { s_end + 1 } else { text_end };
                y_end += line_height;
            }
            text_end = s_end;
        }
        if s == text_end
        {
            return;
        }

        // Reserve vertices for remaining worse case (over-reserving is useful and easily amortized)
        let vtx_count_max: c_int = (text_end - s) * 4;
        let idx_count_max: c_int = (text_end - s) * 6;
        let idx_expected_size: c_int = draw_list.IdxBuffer.len() + idx_count_max;
        draw_list.PrimReserve(idx_count_max, vtx_count_max);

        let mut vtx_write = draw_list._VtxWritePtr;
        let mut idx_write = draw_list._IdxWritePtr;
        let mut vtx_current_idx: c_uint =  draw_list._VtxCurrentIdx;

        let col_untinted: u32 = col | !IM_COL32_A_MASK;

        while s < text_end
        {
            if word_wrap_enabled
            {
                // Calculate how far we can render. Requires two passes on the string data but keeps the code simple and not intrusive for what's essentially an uncommon feature.
                if !word_wrap_eol
                {
                    word_wrap_eol = CalcWordWrapPositionA(scale, s, text_end, wrap_width - (x - start_x));
                    if word_wrap_eol == s {// Wrap_width is too small to fit anything. Force displaying 1 character to minimize the height discontinuity.
                        word_wrap_eol += 1;
                    }  // +1 may not be a character start point in UTF-8 but it's ok because we use s >= word_wrap_eol below
                }

                if s >= word_wrap_eol
                {
                    x = start_x;
                    y += line_height;
                    word_wrap_eol= null_mut();

                    // Wrapping skips upcoming blanks
                    while s < text_end
                    {
                        let c = *s;
                        if ImCharIsBlankA(c) { s+= 1; } else if c == '\n' as c_char { s+= 1; break; } else { break; }
                    }
                    continue;
                }
            }

            // Decode and advance source
            let mut c: c_uint =  *s as c_uint;
            if c < 0x80
            {
                s += 1;
            }
            else
            {
                s += ImTextCharFromUtf8(&mut c, s, text_end);
                if (c == 0) { // Malformed UTF-8?
                    break;
                }
            }

            if c < 32
            {
                if c == '\n' as c_uint
                {
                    x = start_x;
                    y += line_height;
                    if y > clip_rect.w
                    {
                        break;
                    } // break out of main loop
                    continue;
                }
                if c == '\r' as c_uint
                {
                    continue;
                }
            }

            let glyph: *const ImFontGlyph = self.FindGlyph(c);
            if glyph == null_mut()
            {
                continue;
            }

            // let char_width: c_float =   *scale;
            if ()
            {
                // We don't do a second finer clipping test on the Y axis as we've already skipped anything before clip_rect.y and exit once we pass clip_rect.w
                // let x1: c_float =  x +  * scale;
                // let x2: c_float =  x +  * scale;
                // let y1: c_float =  y +  * scale;
                // let y2: c_float =  y +  * scale;
                if (x1 <= clip_rect.z && x2 >= clip_rect.x)
                {
                    // Render a character
                    // let u1: c_float =  ;
                    // let v1: c_float =  ;
                    // let u2: c_float =  ;
                    // let v2: c_float =  ;

                    // CPU side clipping used to fit text in their frame when the frame is too small. Only does clipping for axis aligned quads.
                    if (cpu_fine_clip)
                    {
                        if (x1 < clip_rect.x)
                        {
                            u1 = u1 + (1f32 - (x2 - clip_rect.x) / (x2 - x1)) * (u2 - u1);
                            x1 = clip_rect.x;
                        }
                        if (y1 < clip_rect.y)
                        {
                            v1 = v1 + (1f32 - (y2 - clip_rect.y) / (y2 - y1)) * (v2 - v1);
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
                    // let mut glyph_col: u32 =  ? col_untinted : col;

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
        draw_list.VtxBuffer.len() = (vtx_write - draw_list.VtxBuffer); // Same as calling shrink()
        draw_list.IdxBuffer.len() = (idx_write - draw_list.IdxBuffer);
        draw_list.CmdBuffer[draw_list.CmdBuffer.len() - 1].ElemCount -= (idx_expected_size - draw_list.IdxBuffer.len());
        draw_list._VtxWritePtr = vtx_write;
        draw_list._IdxWritePtr = idx_write;
        draw_list._VtxCurrentIdx = vtx_current_idx;
    }


    // [Internal] Don't use!
    // void              BuildLookupTable();
    pub fn BuildLookupTable(&mut self) {
        todo!()
    }


    // void              ClearOutputData();
    pub fn ClearOutputData(&mut self) {
        todo!()
    }


    // void              GrowIndex(new_size: c_int);
    pub fn GrowIndex(&mut self, new_size: c_int) {
        todo!()
    }


    // void              AddGlyph(const ImFontConfig* src_cfg, ImWchar c, x0: c_float, y0: c_float, x1: c_float, y1: c_float, u0: c_float, v0: c_float, u1: c_float, v1: c_float, advance_x: c_float);
    pub fn AddGlyph(&mut self, src_cfg: *const ImFontConfig, c: ImWchar, x0: c_float, y0: c_float, x1: c_float, y1: c_float, u0: c_float, v0: c_float, u1: c_float, v1: c_float, advance_x: c_float) {
        todo!()
    }

    // void              AddRemapChar(ImWchar dst, ImWchar src, bool overwrite_dst = true); // Makes 'dst' character/glyph points to 'src' character/glyph. Currently needs to be called AFTER fonts have been built.
    pub fn AddRemapChar(&mut self, dst: ImWchar, src: ImWchar, overwrite_dst: bool) {
        todo!()
    }


    // void              SetGlyphVisible(ImWchar c, visible: bool);
    pub fn SetGlyphVisible(&mut self, c: ImWchar, visible: bool) {
        todo!()
    }

    // bool              IsGlyphRangeUnused(unsigned c_begin: c_int, unsigned c_last: c_int);
    pub fn IsGlyphRangeUnused(&mut self, c_being: c_uint, c_last: c_uint) -> bool {
        todo!()
    }
}
