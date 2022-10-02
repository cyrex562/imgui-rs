#![allow(non_snake_case)]

use std::ffi::CStr;
use std::str::pattern::Pattern;
use libc::{c_char, c_float, c_int, c_short, c_uint};
use crate::draw_list::ImDrawList;
use crate::font_atlas::ImFontAtlas;
use crate::font_config::ImFontConfig;
use crate::font_glyph::ImFontGlyph;
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
    pub fn CalcWordWrapPositionA(&mut self, scale: c_float, text: *const c_char, text_end: *const c_char, wrap_width: c_float) {
        todo!()
    }


    // void              RenderChar(ImDrawList* draw_list, c_float size, const ImVec2& pos, u32 col, ImWchar c) const;
    pub fn RenderChar(&mut self, draw_list: *const ImDrawList, size: c_float, pos: &ImVec2, col: u32, c: ImWchar) {
        todo!()
    }


    // void              RenderText(ImDrawList* draw_list, c_float size, const ImVec2& pos, u32 col, const ImVec4& clip_rect, const char* text_begin, const char* text_end, c_float wrap_width = 0f32, bool cpu_fine_clip = false) const;
    pub fn RenderText(&mut self, draw_list: *mut ImDrawList, size: c_float, pos: &ImVec2, col: u32, clip_rect: &ImVec4, text_begin: *const c_char, text_end: *const c_char, wrap_width: c_float, cpu_fine_clip: bool) {
        todo!()
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


    // void              GrowIndex(c_int new_size);
    pub fn GrowIndex(&mut self, new_size: c_int) {
        todo!()
    }


    // void              AddGlyph(const ImFontConfig* src_cfg, ImWchar c, c_float x0, c_float y0, c_float x1, c_float y1, c_float u0, c_float v0, c_float u1, c_float v1, c_float advance_x);
    pub fn AddGlyph(&mut self, src_cfg: *const ImFontConfig, c: ImWchar, x0: c_float, y0: c_float, x1: c_float, y1: c_float, u0: c_float, v0: c_float, u1: c_float, v1: c_float, advance_x: c_float) {
        todo!()
    }

    // void              AddRemapChar(ImWchar dst, ImWchar src, bool overwrite_dst = true); // Makes 'dst' character/glyph points to 'src' character/glyph. Currently needs to be called AFTER fonts have been built.
    pub fn AddRemapChar(&mut self, dst: ImWchar, src: ImWchar, overwrite_dst: bool) {
        todo!()
    }


    // void              SetGlyphVisible(ImWchar c, bool visible);
    pub fn SetGlyphVisible(&mut self, c: ImWchar, visible: bool) {
        todo!()
    }

    // bool              IsGlyphRangeUnused(unsigned c_int c_begin, unsigned c_int c_last);
    pub fn IsGlyphRangeUnused(&mut self, c_being: c_uint, c_last: c_uint) {
        todo!()
    }
}
