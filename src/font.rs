#![allow(non_snake_case)]

use std::borrow::BorrowMut;
use std::ffi::CStr;
use std::os::raw::c_void;
use std::ptr::null_mut;
use std::str::pattern::Pattern;
use libc::{c_char, c_float, c_int, c_short, c_uint, size_t};
use crate::color::IM_COL32_A_MASK;
use crate::font_ops::FindFirstExistingGlyph;
use crate::draw_list::ImDrawList;
use crate::draw_vert::ImDrawVert;
use crate::font_atlas::ImFontAtlas;
use crate::font_config::ImFontConfig;
use crate::font_glyph::ImFontGlyph;
use crate::math_ops::{ImCharIsBlankA, ImCharIsBlankW, ImClamp, ImMax};
use crate::string_ops::ImTextCharFromUtf8;
use crate::vec2::ImVec2;
use crate::vec4::ImVec4;
use crate::type_defs::{ImDrawIdx, ImWchar};

// Font runtime data and rendering
// ImFontAtlas automatically loads a default embedded font for you when you call GetTexDataAsAlpha8() or GetTexDataAsRGBA32().
#[derive(Default, Debug, Clone)]
pub struct ImFont {
    // Members: Hot ~20/24 bytes (for CalcTextSize)
    pub IndexAdvanceX: Vec<c_float>,
    // 12-16 // out //            // Sparse. Glyphs->AdvanceX in a directly indexable way (cache-friendly for CalcTextSize functions which only this this info, and are often bottleneck in large UI).
    pub FallbackAdvanceX: c_float,
    // 4     // out // = Fallbackglyph.AdvanceX
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
    // 4-8   // in  //            // Pointer within ContainerAtlas->ConfigData
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
    pub fn new() -> Self {
        let mut out = Self::default();
        out.FontSize = 0.0;
        out.FallbackAdvanceX = 0.0;
        out.FallbackChar = -1;
        out.EllipsisChar = -1;
        out.DotChar = -1;
        out.FallbackGlyph = null_mut();
        out.ContainerAtlas = null_mut();
        out.ConfigData = null_mut();
        out.ConfigDataCount = 0;
        out.DirtyLookupTables = false;
        out.Scale = 1.0;
        out.Ascent = 0.0;
        out.Descent = 0.0;
        out.MetricsTotalSurface = 0;
        // memset(Used4kPagesMap, 0, sizeof(Used4kPagesMap));
        out
    }


    // ~ImFont();


    // const ImFontGlyph*FindGlyph(ImWchar c) const;
    pub unsafe fn FindGlyph(&mut self, c: ImWchar) -> *mut ImFontGlyph {
        let mut out_glyph: ImFontGlyph = ImFontGlyph::default();
        if c >= self.IndexLookup.len() as ImWchar {
            out_glyph = *(self.FallbackGlyph.clone());
            return &mut out_glyph;
        }
        let i = self.IndexLookup[c];
        if i == -1 {
            out_glyph = *(self.FallbackGlyph.clone());
            return &mut out_glyph;
        }
        return &mut self.Glyphs[i];
    }

    // const ImFontGlyph*FindGlyphNoFallback(ImWchar c) const;
    pub fn FindGlyphNoFallback(&mut self, c: ImWchar) -> *mut ImFontGlyph {
        if c >= self.IndexLookup.len() as ImWchar {
            return null_mut();
        }
        let i: ImWchar = self.IndexLookup[c];
        if i == -1 {
            return null_mut();
        }
        return &mut self.Glyphs[i];
    }


    // c_float                       GetCharAdvance(ImWchar c) const     { return (c < IndexAdvanceX.len()) ? IndexAdvanceX[c] : FallbackAdvanceX; }
    pub fn GetCharAdvance(&self, c: ImWchar) -> c_float {
        return if (c as usize) < self.IndexAdvanceX.len() {
            self.IndexAdvanceX[c.clone()]
        } else {
            self.FallbackAdvanceX.clone()
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
    // 'wrap_width' enable automatic word-wrapping across multiple lines to fit into given width. 0.0 to disable.
    // ImVec2            CalcTextSizeA(c_float size, c_float max_width, c_float wrap_width, const char* text_begin, const char* text_end = NULL, const char** remaining = NULL) const; // utf8
    pub unsafe fn CalcTextSizeA(&mut self, size: c_float, max_width: c_float, wrap_width: c_float, text_begin: *const c_char, mut text_end: *const c_char, remaining: *mut *const c_char) -> ImVec2 {
        if !text_end {
            text_end = text_begin + libc::strlen(text_begin);
        } // FIXME-OPT: Need to avoid this.

        let line_height: c_float = size;
        let scale: c_float = size / FontSize;

        let mut text_size: ImVec2 = ImVec2::new(0.0, 0.0);
        let mut line_width: c_float = 0.0;

        let word_wrap_enabled: bool = (wrap_width > 0.0);
        let mut word_wrap_eol: *const c_char = null_mut();

        let mut s: *const c_char = text_begin;
        while s < text_end {
            if word_wrap_enabled {
                // Calculate how far we can render. Requires two passes on the string data but keeps the code simple and not intrusive for what's essentially an uncommon feature.
                if !word_wrap_eol {
                    word_wrap_eol = CalcWordWrapPositionA(scale, s, text_end, wrap_width - line_width);
                    if word_wrap_eol == s {// Wrap_width is too small to fit anything. Force displaying 1 character to minimize the height discontinuity.
                        word_wrap_eol += 1;
                    }    // +1 may not be a character start point in UTF-8 but it's ok because we use s >= word_wrap_eol below
                }

                if s >= word_wrap_eol {
                    if text_size.x < line_width {
                        text_size.x = line_width;
                    }
                    text_size.y += line_height;
                    line_width = 0.0;
                    word_wrap_eol = null_mut();

                    // Wrapping skips upcoming blanks
                    while s < text_end {
                        const c: c_char = *s;
                        if ImCharIsBlankA(c) { s += 1; } else if c == '\n' as c_char {
                            s += 1;
                            break;
                        } else { break; }
                    }
                    continue;
                }
            }

            // Decode and advance source
            let mut prev_s: *const c_char = s;
            let mut c: c_uint = *s as c_uint;
            if c < 0x80 {
                s += 1;
            } else {
                s += ImTextCharFromUtf8(&mut c, s, text_end);
                if c == 0 { // Malformed UTF-8?
                    break;
                }
            }

            if c < 32 {
                if c == c_uint::from('\n') {
                    text_size.x = ImMax(text_size.x, line_width);
                    text_size.y += line_height;
                    line_width = 0.0;
                    continue;
                }
                if c == c_uint::from('\r') {
                    continue;
                }
            }

            let char_width: c_float = (if c < self.IndexAdvanceX.Size { self.IndexAdvanceX[c] } else { self.FallbackAdvanceX }) * scale;
            if line_width + char_width >= max_width {
                s = prev_s;
                break;
            }

            line_width += char_width;
        }

        if text_size.x < line_width {
            text_size.x = line_width;
        }

        if line_width > 0.0 || text_size.y == 0.0 {
            text_size.y += line_height;
        }

        if remaining {
            *remaining = s;
        }

        return text_size;
    }


    // const char*       CalcWordWrapPositionA(c_float scale, const char* text, const char* text_end, c_float wrap_width) const;
    pub unsafe fn CalcWordWrapPositionA(&mut self, scale: c_float, text: *const c_char, text_end: *const c_char, mut wrap_width: c_float) -> *const c_char{
        // Simple word-wrapping for English, not full-featured. Please submit failing cases!
        // FIXME: Much possible improvements (don't cut things like "word !", "word!!!" but cut within "word,,,,", more sensible support for punctuations, support for Unicode punctuations, etc.)

        // For references, possible wrap point marked with ^
        //  "aaa bbb, ccc,ddd. eee   fff. ggg!"
        //      ^    ^    ^   ^   ^__    ^    ^

        // List of hardcoded separators: .,;!?'"

        // Skip extra blanks after a line returns (that includes not counting them in width computation)
        // e.g. "Hello    world" --> "Hello" "World"

        // Cut words that cannot possibly fit within one line.
        // e.g.: "The tropical fish" with ~5 characters worth of width --> "The tr" "opical" "fish"

        let mut line_width: c_float =  0.0;
        let mut word_width: c_float =  0.0;
        let mut blank_width: c_float =  0.0;
        wrap_width /= scale; // We work with unscaled widths to avoid scaling every characters

        let mut  word_end: *const c_char = text;
        let mut  prev_word_end: *const c_char= null_mut();
        let mut inside_word: bool =  true;

        let mut  s: *const c_char = text;
        while s < text_end
        {
            let mut c: c_uint = (*s).clone() as c_uint;
            let next_s: *const c_char;
            if c < 0x80 {
                next_s = s + 1;
            }
            else {
                next_s = s + ImTextCharFromUtf8(&mut c, s, text_end);
            }
            if c == 0 {
                break;
            }

            if c < 32
            {
                if c == c_uint::from('\n')
                {
                    line_width = 0.0;
                    word_width = 0.0;
                    blank_width = 0.0;
                    inside_word = true;
                    s = next_s;
                    continue;
                }
                if c == c_uint::from('\r')
                {
                    s = next_s;
                    continue;
                }
            }

            let char_width: c_float =  (if c < self.IndexAdvanceX.len() as c_uint { self.IndexAdvanceX[c] } else { self.FallbackAdvanceX.clone() });
            if ImCharIsBlankW(c.clone())
            {
                if inside_word
                {
                    line_width += blank_width;
                    blank_width = 0.0;
                    word_end = s;
                }
                blank_width += char_width;
                inside_word = false;
            }
            else
            {
                word_width += char_width;
                if inside_word
                {
                    word_end = next_s;
                }
                else
                {
                    prev_word_end = word_end;
                    line_width += word_width.clone() + blank_width;
                    word_width = 0.0;
                    blank_width = 0.0;
                }

                // Allow wrapping after punctuation.
                inside_word = (c != c_uint::from('.') && c != c_uint::from(',') && c != c_uint::from(';') && c != c_uint::from('!') && c != c_uint::from('?') && c != c_uint::from('\"'));
            }

            // We ignore blank width at the end of the line (they can be skipped)
            if line_width.clone() + word_width.clone() > wrap_width
            {
                // Words that cannot possibly fit within an entire line will be cut anywhere.
                if word_width < wrap_width {
                    s = if prev_word_end {
                        prev_word_end
                    } else { word_end };
                }
                break;
            }

            s = next_s;
        }

        return s;
    }


    // void              RenderChar(draw_list: *mut ImDrawList, c_float size, const pos: &mut ImVec2, col: u32, ImWchar c) const;
    pub unsafe fn RenderChar(&mut self, draw_list: &mut ImDrawList, size: c_float, pos: &ImVec2, mut col: u32, c: ImWchar) {
        let glyph: *const ImFontGlyph = self.FindGlyph(c);
        if glyph.is_null() || !glyph.Visible {
            return;
        }
        if glyph.Colored {

            col |= !IM_COL32_A_MASK;
        }
        let scale: c_float =  if size >= 0.0 { (size / self.FontSize) } else { 1 };
        let x: c_float =  IM_FLOOR(pos.x);
        let y: c_float =  IM_FLOOR(pos.y);
        draw_list.PrimReserve(6, 4);
        draw_list.PrimRectUV(&ImVec2::new(x + glyph.X0 * scale, y + glyph.Y0 * scale), &ImVec2::new(x + glyph.X1 * scale, y + glyph.Y1 * scale), &ImVec2::new(glyph.U0, glyph.V0), &ImVec2::new(glyph.U1, glyph.V1), col);
    }


    // void              RenderText(draw_list: *mut ImDrawList, c_float size, const pos: &mut ImVec2, col: u32, const ImVec4& clip_rect, const char* text_begin, const char* text_end, c_float wrap_width = 0.0, cpu_fine_clip: bool = false) const;
    pub unsafe fn RenderText(&mut self, draw_list: &mut ImDrawList, size: c_float, pos: &ImVec2, mut col: u32, clip_rect: &ImVec4, text_begin: *const c_char, mut text_end: *const c_char, wrap_width: c_float, cpu_fine_clip: bool) {
        if !text_end {
            text_end = text_begin + libc::strlen(text_begin);
        } //  functions generally already provides a valid text_end, so this is merely to handle direct calls.

        // Align to be pixel perfect
        let mut x: c_float = IM_FLOOR(pos.x);
        let mut y: c_float = IM_FLOOR(pos.y);
        if y > clip_rect.w {
            return;
        }

        let start_x: c_float = x;
        let scale: c_float = size / FontSize;
        let line_height: c_float = FontSize * scale;
        let word_wrap_enabled: bool = (wrap_width > 0.0);
        let mut word_wrap_eol: *const c_char = null_mut();

        // Fast-forward to first visible line
        let mut s: *const c_char = text_begin;
        if y + line_height < clip_rect.y && !word_wrap_enabled {
            while y + line_height < clip_rect.y && s < text_end {
                s = libc::memchr(s, '\n' as c_int, text_end - s);
                s = if s { s + 1 } else { text_end };
                y += line_height;
            }
        }

        // For large text, scan for the last visible line in order to avoid over-reserving in the call to PrimReserve()
        // Note that very large horizontal line will still be affected by the issue (e.g. a one megabyte string buffer without a newline will likely crash atm)
        if text_end - s > 10000 && !word_wrap_enabled {
            let mut s_end: *const c_char = s;
            let mut y_end: c_float = y;
            while y_end < clip_rect.w && s_end < text_end {
                s_end = libc::memchr(s_end, '\n' as c_int, text_end - s_end);
                s_end = if s_end { s_end + 1 } else { text_end };
                y_end += line_height;
            }
            text_end = s_end;
        }
        if s == text_end {
            return;
        }

        // Reserve vertices for remaining worse case (over-reserving is useful and easily amortized)
        let vtx_count_max: size_t = (text_end - s) * 4;
        let idx_count_max: size_t = (text_end - s) * 6;
        let idx_expected_size: size_t = (draw_list.IdxBuffer.len() + idx_count_max);
        draw_list.PrimReserve(idx_count_max, vtx_count_max);

        vtx_write: *mut ImDrawVert = draw_list._VtxWritePtr;
        ImDrawIdx * idx_write = draw_list._IdxWritePtr;
        let mut vtx_current_idx: size_t = draw_list._VtxCurrentIdx;

        col_untinted: u32 = col | !IM_COL32_A_MASK;

        while s < text_end {
            if word_wrap_enabled {
                // Calculate how far we can render. Requires two passes on the string data but keeps the code simple and not intrusive for what's essentially an uncommon feature.
                if !word_wrap_eol {
                    word_wrap_eol = self.CalcWordWrapPositionA(scale, s, text_end, wrap_width - (x - start_x));
                    if word_wrap_eol == s { // Wrap_width is too small to fit anything. Force displaying 1 character to minimize the height discontinuity.
                        word_wrap_eol += 1;
                    } // +1 may not be a character start point in UTF-8 but it's ok because we use s >= word_wrap_eol below
                }

                if s >= word_wrap_eol {
                    x = start_x;
                    y += line_height;
                    word_wrap_eol = null_mut();

                    // Wrapping skips upcoming blanks
                    while s < text_end {
                        const c: c_char = *s;
                        if ImCharIsBlankA(c) { s += 1; } else if c == '\n' as c_char {
                            s += 1;
                            break;
                        } else { break; }
                    }
                    continue;
                }
            }

            // Decode and advance source
            let mut c: c_uint = *s as c_uint;
            if c < 0x80 {
                s += 1;
            } else {
                s += ImTextCharFromUtf8(&mut c, s, text_end);
                if c == 0 { // Malformed UTF-8?
                    break;
                }
            }

            if c < 32 {
                if c == c_uint::from('\n') {
                    x = start_x;
                    y += line_height;
                    if y > clip_rect.w {
                        break;
                    } // break out of main loop
                    continue;
                }
                if c == c_uint::from('\r') {
                    continue;
                }
            }

            let glyph: *const ImFontGlyph = FindGlyph(c);
            if glyph == null_mut() {
                continue;
            }

            let char_width: c_float = glyph.AdvanceX * scale;
            if glyph.Visible {
                // We don't do a second finer clipping test on the Y axis as we've already skipped anything before clip_rect.y and exit once we pass clip_rect.w
                let mut x1: c_float = x + glyph.X0 * scale;
                let mut x2: c_float = x + glyph.X1 * scale;
                let mut y1: c_float = y + glyph.Y0 * scale;
                let mut y2: c_float = y + glyph.Y1 * scale;
                if x1 <= clip_rect.z && x2 >= clip_rect.x {
                    // Render a character
                    let mut u1: c_float = glyph.U0;
                    let mut v1: c_float = glyph.V0;
                    let mut u2: c_float = glyph.U1;
                    let mut v2: c_float = glyph.V1;

                    // CPU side clipping used to fit text in their frame when the frame is too small. Only does clipping for axis aligned quads.
                    if cpu_fine_clip {
                        if x1 < clip_rect.x {
                            u1 = u1 + (1 - (x2 - clip_rect.x) / (x2 - x1)) * (u2 - u1);
                            x1 = clip_rect.x;
                        }
                        if y1 < clip_rect.y {
                            v1 = v1 + (1 - (y2 - clip_rect.y) / (y2 - y1)) * (v2 - v1);
                            y1 = clip_rect.y;
                        }
                        if x2 > clip_rect.z {
                            u2 = u1 + ((clip_rect.z - x1) / (x2 - x1)) * (u2 - u1);
                            x2 = clip_rect.z;
                        }
                        if y2 > clip_rect.w {
                            v2 = v1 + ((clip_rect.w - y1) / (y2 - y1)) * (v2 - v1);
                            y2 = clip_rect.w;
                        }
                        if y1 >= y2 {
                            x += char_width;
                            continue;
                        }
                    }

                    // Support for untinted glyphs
                    glyph_col: u32 = if glyph.Colored { col_untinted } else { col };

                    // We are NOT calling PrimRectUV() here because non-inlined causes too much overhead in a debug builds. Inlined here:
                    {
                        idx_write[0] = (vtx_current_idx);
                        idx_write[1] = (vtx_current_idx + 1);
                        idx_write[2] = (vtx_current_idx + 2);
                        idx_write[3] = (vtx_current_idx);
                        idx_write[4] = (vtx_current_idx + 2);
                        idx_write[5] = (vtx_current_idx + 3);
                        vtx_write[0].pos.x = x1;
                        vtx_write[0].pos.y = y1;
                        vtx_write[0].col = glyph_col;
                        vtx_write[0].uv.x = u1;
                        vtx_write[0].uv.y = v1;
                        vtx_write[1].pos.x = x2;
                        vtx_write[1].pos.y = y1;
                        vtx_write[1].col = glyph_col;
                        vtx_write[1].uv.x = u2;
                        vtx_write[1].uv.y = v1;
                        vtx_write[2].pos.x = x2;
                        vtx_write[2].pos.y = y2;
                        vtx_write[2].col = glyph_col;
                        vtx_write[2].uv.x = u2;
                        vtx_write[2].uv.y = v2;
                        vtx_write[3].pos.x = x1;
                        vtx_write[3].pos.y = y2;
                        vtx_write[3].col = glyph_col;
                        vtx_write[3].uv.x = u1;
                        vtx_write[3].uv.y = v2;
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


    // [Internal] Don't use!
    // void              BuildLookupTable();
    pub unsafe fn BuildLookupTable(&mut self) {
        let mut max_codepoint: c_int = 0;
        // for (let i: c_int = 0; i != Glyphs.len(); i++)
        for i in 0..self.Glyphs.len() {
            max_codepoint = ImMax(max_codepoint, self.Glyphs[i].Codepoint);
        }

        // Build lookup table
        // IM_ASSERT(Glyphs.len() < 0xFFF0); // -1 is reserved
        self.IndexAdvanceX.clear();
        self.IndexLookup.clear();
        self.DirtyLookupTables = false;
        libc::memset(self.Used4kPagesMap.as_mut_ptr() as *mut c_void, 0, self.Used4kPagesMap.len());
        self.GrowIndex((max_codepoint + 1) as size_t);
        // for (let i: c_int = 0; i < Glyphs.len(); i++)
        for i in 0..self.Glyphs.len() {
            let codepoint: c_int = self.Glyphs[i].Codepoint;
            let glyph = (&self.Glyphs).get(i.clone()).unwrap().clone();
            self.IndexAdvanceX[codepoint] = glyph.AdvanceX;
            self.IndexLookup[codepoint.clone()] = i.clone();

            // Mark 4K page as used
            let page_n: c_int = codepoint.clone() / 4096;
            self.Used4kPagesMap[page_n >> 3] |= 1 << (page_n.clone() & 7);
        }

        // Create a glyph to handle TAB
        // FIXME: Needs proper TAB handling but it needs to be contextualized (or we could arbitrary say that each string starts at "column 0" ?)
        if self.FindGlyph(ImWchar::from(' ')) {
            if self.Glyphs.last().unwrap().Codepoint != '\t' {   // So we can call this function multiple times (FIXME: Flaky)
                self.Glyphs.resize_with(self.Glyphs.len() + 1, ImFontGlyph::default());
            }
            let mut tab_glyph: &mut ImFontGlyph = self.Glyphs.last_mut().unwrap();
            tab_glyph = self.FindGlyph(ImWchar::from(' ')).borrow_mut();
            tab_glyph.Codepoint = '\t';
            tab_glyph.AdvanceX *= IM_TABSIZE;
            self.IndexAdvanceX[tab_glyph.Codepoint] = tab_glyph.AdvanceX.clone();
            self.IndexLookup[tab_glyph.Codepoint] = (self.Glyphs.len() - 1);
        }

        // Mark special glyphs as not visible (note that AddGlyph already mark as non-visible glyphs with zero-size polygons)
        self.SetGlyphVisible(ImWchar::from(' '), false);
        self.SetGlyphVisible(ImWchar::from('\t'), false);

        // Ellipsis character is required for rendering elided text. We prefer using U+2026 (horizontal ellipsis).
        // However some old fonts may contain ellipsis at U+0085. Here we auto-detect most suitable ellipsis character.
        // FIXME: Note that 0x2026 is rarely included in our font ranges. Because of this we are more likely to use three individual dots.
        let ellipsis_chars: [ImWchar; 2] = [0x2026, 0x0085];
        let dots_chars: [ImWchar; 2] = [ImWchar::from('.'), 0xFF0E];
        if self.EllipsisChar == -1 {
            self.EllipsisChar = FindFirstExistingGlyph(this, ellipsis_chars.as_ptr(), ellipsis_chars.len());
        }
        if self.DotChar == -1 {
            self.DotChar = FindFirstExistingGlyph(this, dots_chars.as_ptr(), dots_chars.len());
        }

        // Setup fallback character
        let fallback_chars: [ImWchar; 3] = [ImWchar::from(IM_UNICODE_CODEPOINT_INVALID), ImWchar::from('?'), ImWchar::from(' ')];
        self.FallbackGlyph = self.FindGlyphNoFallback(self.FallbackChar.clone());
        if self.FallbackGlyph == null_mut() {
            self.FallbackChar = FindFirstExistingGlyph(this, fallback_chars.as_ptr(), fallback_chars.len());
            self.FallbackGlyph = self.FindGlyphNoFallback(self.FallbackChar.clone());
            if self.FallbackGlyph == null_mut() {
                self.FallbackGlyph = self.Glyphs.last().unwrap();
                self.FallbackChar = self.FallbackGlyph.Codepoint;
            }
        }

        self.FallbackAdvanceX = self.FallbackGlyph.AdvanceX.clone();
        // for (let i: c_int = 0; i < max_codepoint + 1; i++)
        for i in 0..max_codepoint {
            if self.IndexAdvanceX[i] < 0 {
                self.IndexAdvanceX[i.clone()] = self.FallbackAdvanceX.clone();
            }
        }
    }


    // void              ClearOutputData();
    pub fn ClearOutputData(&mut self) {
        self.FontSize = 0.0;
        self.FallbackAdvanceX = 0.0;
        self.Glyphs.clear();
        self.IndexAdvanceX.clear();
        self.IndexLookup.clear();
        self.FallbackGlyph = null_mut();
        self.ContainerAtlas = null_mut();
        self.DirtyLookupTables = true;
        self.Ascent = 0.0;
        self.Descent = 0.0;
        self.MetricsTotalSurface = 0;
    }


    // void              GrowIndex(new_size: c_int);
    pub fn GrowIndex(&mut self, new_size: size_t) {
        // IM_ASSERT(IndexAdvanceX.len() == IndexLookup.len());
        if new_size <= self.IndexLookup.len() {
            return;
        }
        self.IndexAdvanceX.resize(new_size, -1.0);
        self.IndexLookup.resize(new_size.clone(), -1);
    }


    // void              AddGlyph(const ImFontConfig* src_cfg, ImWchar c, c_float x0, c_float y0, c_float x1, c_float y1, c_float u0, c_float v0, c_float u1, c_float v1, c_float advance_x);
    pub fn AddGlyph(&mut self, src_cfg: *const ImFontConfig, c: ImWchar, mut x0: c_float, y0: c_float, mut x1: c_float, y1: c_float, u0: c_float, v0: c_float, u1: c_float, v1: c_float, mut advance_x: c_float) {
        if cfg != null_mut()
        {
            // Clamp & recenter if needed
            let advance_x_original: c_float =  advance_x;
            advance_x = ImClamp(advance_x.clone(), cfg.GlyphMinAdvanceX, cfg.GlyphMaxAdvanceX);
            if advance_x != advance_x_original
            {
                let char_off_x: c_float =  if cfg.PixelSnapH { ImFloor((advance_x - advance_x_original) * 0.5) } else { (advance_x - advance_x_original) * 0.5 };
                x0 += char_off_x;
                x1 += char_off_x.clone();
            }

            // Snap to pixel
            if cfg.PixelSnapH {
                advance_x = IM_ROUND(advance_x.clone());
            }

            // Bake spacing
            advance_x += cfg.GlyphExtraSpacing.x;
        }

        self.Glyphs.resize_with(Glyphs.Size + 1, ImFontGlyph::default());
        let glyph = self.Glyphs.last_mut().unwrap();
        glyph.Codepoint = codepoint;
        glyph.Visible = (x0 != x1) && (y0 != y1);
        glyph.Colored = false;
        glyph.X0 = x0;
        glyph.Y0 = y0;
        glyph.X1 = x1;
        glyph.Y1 = y1;
        glyph.U0 = u0;
        glyph.V0 = v0;
        glyph.U1 = u1;
        glyph.V1 = v1;
        glyph.AdvanceX = advance_x.clone();

        // Compute rough surface usage metrics (+1 to account for average padding, +0.99 to round)
        // We use (U1-U0)*TexWidth instead of X1-X0 to account for oversampling.
        let pad: c_float =  self.ContainerAtlas.TexGlyphPadding.clone() + 0.99;
        self.DirtyLookupTables = true;
        self.MetricsTotalSurface += ((glyph.U1.clone() - glyph.U0.clone()) * self.ContainerAtlas.TexWidth.clone() + pad) * ((glyph.V1.clone() - glyph.V0.clone()) * self.ContainerAtlas.TexHeight.clone() + pad.clone());
    }

    // void              AddRemapChar(ImWchar dst, ImWchar src, overwrite_dst: bool = true); // Makes 'dst' character/glyph points to 'src' character/glyph. Currently needs to be called AFTER fonts have been built.
    pub fn AddRemapChar(&mut self, dst: ImWchar, src: ImWchar, overwrite_dst: bool) {

        // IM_ASSERT(IndexLookup.len() > 0);    // Currently this can only be called AFTER the font has been built, aka after calling ImFontAtlas::GetTexDataAs*() function.
        let mut index_size: size_t = self.IndexLookup.len();

        if dst < index_size as ImWchar && self.IndexLookup[dst] == -1 && !overwrite_dst { // 'dst' already exists
            return;
        }
        if src >= index_size.clone() as ImWchar && dst >= index_size.clone() as ImWchar {// both 'dst' and 'src' don't exist -> no-op
            return;
        }

        self.GrowIndex((dst.clone() + 1) as size_t);
        self.IndexLookup[dst.clone()] = if src < index_size.clone() as ImWchar { self.IndexLookup[src] } else { -1 };
        self.IndexAdvanceX[dst.clone()] = if src < index_size.clone() as ImWchar { self.IndexAdvanceX[src.clone()] } else { 1 };
    }


    // void              SetGlyphVisible(ImWchar c, visible: bool);
    pub unsafe fn SetGlyphVisible(&mut self, c: ImWchar, visible: bool) {

        let glyph = self.FindGlyph(c);
        if glyph.is_null() == false {
            glyph.Visible = visible;
        }
    }

    // bool              IsGlyphRangeUnused(unsigned c_begin: c_int, unsigned c_last: c_int);
    pub fn IsGlyphRangeUnused(&mut self, c_being: c_uint, c_last: c_uint) -> bool {
        let mut page_begin: c_uint = (c_begin / 4096);
        let mut page_last: c_uint = (c_last / 4096);
        // for (let mut page_n: c_uint =  page_begin; page_n <= page_last; page_n++)
        for page_n in page_begin..page_last {
            if (page_n >> 3) < self.Used4kPagesMap.len() as c_uint {
                if self.Used4kPagesMap[page_n.clone() >> 3] & (1 << (page_n.clone() & 7)) {
                    return false;
                }
            }
        }
        return true;
    }
}
