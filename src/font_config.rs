#![allow(non_snake_case)]

use crate::font::ImFont;
use crate::type_defs::ImWchar;
use crate::vec2::ImVec2;
use libc::{c_char, c_float, c_int, c_uint, c_void, size_t};

#[derive(Default, Debug, Clone, Copy)]
pub struct ImFontConfig {
    pub FontData: *mut c_void,
    //          // TTF/OTF data
    pub FontDataSize: usize,
    //          // TTF/OTF data size
    pub FontDataOwnedByAtlas: bool,
    // true     // TTF/OTF data ownership taken by the container ImFontAtlas (will delete memory itsel0f32).
    pub FontNo: i32,
    // 0        // Index of font within TTF/OTF file
    pub SizePixels: usize,
    //          // Size in pixels for rasterizer (more or less maps to the resulting font height).
    pub OversampleH: i32,
    // 3        // Rasterize at higher quality for sub-pixel positioning. Note the difference between 2 and 3 is minimal so you can reduce this to 2 to save memory. Read https://github.com/nothings/stb/blob/master/tests/oversample/README.md for details.
    pub OversampleV: i32,
    // 1        // Rasterize at higher quality for sub-pixel positioning. This is not really useful as we don't use sub-pixel positions on the Y axis.
    pub PixelSnapH: bool,
    // false    // Align every glyph to pixel boundary. Useful e.g. if you are merging a non-pixel aligned font with the default font. If enabled, you can set OversampleH/V to 1.
    pub GlyphExtraSpacing: ImVec2,
    // 0, 0     // Extra spacing (in pixels) between glyphs. Only X axis is supported for now.
    pub GlyphOffset: usize,
    // 0, 0     // Offset all glyphs from this font input.
    pub GlyphRanges: *const ImWchar,
    // NULL     // Pointer to a user-provided list of Unicode range (2 value per range, values are inclusive, zero-terminated list). THE ARRAY DATA NEEDS TO PERSIST AS LONG AS THE FONT IS ALIVE.
    pub GlyphMinAdvanceX: f32,
    // 0        // Minimum AdvanceX for glyphs, set Min to align font icons, set both Min/Max to enforce mono-space font
    pub GlyphMaxAdvanceX: f32,
    // f32::MAX  // Maximum AdvanceX for glyphs
    pub MergeMode: bool,
    // false    // Merge into previous ImFont, so you can combine multiple inputs font into one ImFont (e.g. ASCII font + icons + Japanese glyphs). You may want to use GlyphOffset.y when merge font of different heights.
    pub FontBuilderFlags: u32,
    // 0        // Settings for custom font builder. THIS IS BUILDER IMPLEMENTATION DEPENDENT. Leave as zero if unsure.
    pub RasterizerMultiply: f32,
    // 1.0     // Brighten (>1.0) or darken (<1.0) font output. Brightening small fonts may be a good workaround to make them more readable.
    pub EllipsisChar: char, // -1       // Explicitly specify unicode codepoint of ellipsis character. When fonts are being merged first specified ellipsis will be used.

    // [Internal]
    pub Name: String,
    // Name (strictly to ease debugging)
    pub DstFont: ImFont,
    // ImFontConfig();
}

impl ImFontConfig {
    pub fn new() -> Self {
        // memset(this, 0, sizeof(*this));
        let mut out = Self::default();
        out.FontDataOwnedByAtlas = true;
        out.OversampleH = 3; // FIXME: 2 may be a better default?
        out.OversampleV = 1;
        out.GlyphMaxAdvanceX = f32::MAX;
        out.RasterizerMultiply = 1.0;
        out.EllipsisChar = '\0';
        out
    }
}
