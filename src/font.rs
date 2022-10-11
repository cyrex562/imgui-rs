#![allow(non_snake_case)]

use std::borrow::BorrowMut;
use std::ffi::CStr;
use std::os::raw::c_void;
use std::ptr::null_mut;
use std::str::pattern::Pattern;
use libc::{c_char, c_float, c_int, c_short, c_uint, size_t};
use crate::draw::FindFirstExistingGlyph;
use crate::draw_list::ImDrawList;
use crate::font_atlas::ImFontAtlas;
use crate::font_config::ImFontConfig;
use crate::font_glyph::ImFontGlyph;
use crate::math_ops::{ImClamp, ImMax};
use crate::string_ops::ImTextCharFromUtf8;
use crate::vec2::ImVec2;
use crate::vec4::ImVec4;
use crate::type_defs::ImWchar;

// Font runtime data and rendering
// ImFontAtlas automatically loads a default embedded font for you when you call GetTexDataAsAlpha8() or GetTexDataAsRGBA32().
#[derive(Default, Debug, Clone)]
pub struct ImFont {
    // Members: Hot ~20/24 bytes (for CalcTextSize)
    pub IndexAdvanceX: Vec<c_float>,
    // 12-16 // out //            // Sparse. Glyphs->AdvanceX in a directly indexable way (cache-friendly for CalcTextSize functions which only this this info, and are often bottleneck in large UI).
    pub FallbackAdvanceX: c_float,
    // 4     // out // = FallbackGlyph->AdvanceX
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


    // const char*                 GetDebugName() const                { return ConfigData ? ConfigData->Name : "<unknown>"; }
    pub unsafe fn GetDebugName(&self) -> *const c_char {
        return if self.ConfigData.is_null() == false {
            self.ConfigData.Name.as_ptr()
        } else {
            CStr::from_bytes_with_nul_unchecked(String::from("<unknown>").as_bytes()).as_ptr()
        };
    }

    // 'max_width' stops rendering after a certain width (could be turned into a 2d size). f32::MAX to disable.
    // 'wrap_width' enable automatic word-wrapping across multiple lines to fit into given width. 0f32 to disable.
    // ImVec2            CalcTextSizeA(c_float size, c_float max_width, c_float wrap_width, const char* text_begin, const char* text_end = NULL, const char** remaining = NULL) const; // utf8
    pub fn CalcTextSizeA(&mut self, size: c_float, max_width: c_float, wrap_width: c_float, text_begin: *const c_char, text_end: *const c_char, remaining: *mut *const c_char) -> ImVec2 {
        todo!()
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


    // void              RenderChar(draw_list: *mut ImDrawList, c_float size, const ImVec2& pos, col: u32, ImWchar c) const;
    pub fn RenderChar(&mut self, draw_list: *const ImDrawList, size: c_float, pos: &ImVec2, col: u32, c: ImWchar) {
        todo!()
    }


    // void              RenderText(draw_list: *mut ImDrawList, c_float size, const ImVec2& pos, col: u32, const ImVec4& clip_rect, const char* text_begin, const char* text_end, c_float wrap_width = 0f32, cpu_fine_clip: bool = false) const;
    pub fn RenderText(&mut self, draw_list: *mut ImDrawList, size: c_float, pos: &ImVec2, col: u32, clip_rect: &ImVec4, text_begin: *const c_char, text_end: *const c_char, wrap_width: c_float, cpu_fine_clip: bool) {
        todo!()
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
        self.GrowIndex(max_codepoint + 1);
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

        self.GrowIndex(dst.clone() + 1);
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
            if (page_n >> 3) < self.Used4kPagesMap.len() {
                if self.Used4kPagesMap[page_n.clone() >> 3] & (1 << (page_n.clone() & 7)) {
                    return false;
                }
            }
        }
        return true;
    }
}
