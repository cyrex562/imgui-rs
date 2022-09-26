#![allow(non_snake_case)]

use libc::{c_char, c_float, c_int, c_uint, c_void};
use crate::font::ImFont;
use crate::imgui_vec2::ImVec2;
use crate::type_defs::ImWchar;

#[derive(Default,Debug,Clone)]
pub struct ImFontConfig
{
pub FontData: *mut c_void,               //          // TTF/OTF data
pub FontDataSize: c_int,           //          // TTF/OTF data size
pub FontDataOwnedByAtlas: bool,   // true     // TTF/OTF data ownership taken by the container ImFontAtlas (will delete memory itsel0f32).
pub FontNo: c_int,                 // 0        // Index of font within TTF/OTF file
pub SizePixels: c_float,             //          // Size in pixels for rasterizer (more or less maps to the resulting font height).
pub OversampleH: c_int,            // 3        // Rasterize at higher quality for sub-pixel positioning. Note the difference between 2 and 3 is minimal so you can reduce this to 2 to save memory. Read https://github.com/nothings/stb/blob/master/tests/oversample/README.md for details.
pub OversampleV: c_int,            // 1        // Rasterize at higher quality for sub-pixel positioning. This is not really useful as we don't use sub-pixel positions on the Y axis.
pub PixelSnapH: bool,             // false    // Align every glyph to pixel boundary. Useful e.g. if you are merging a non-pixel aligned font with the default font. If enabled, you can set OversampleH/V to 1.
pub GlyphExtraSpacing: ImVec2,      // 0, 0     // Extra spacing (in pixels) between glyphs. Only X axis is supported for now.
pub GlyphOffset: ImVec2,            // 0, 0     // Offset all glyphs from this font input.
    pub  GlyphRanges: *const ImWchar,            // NULL     // Pointer to a user-provided list of Unicode range (2 value per range, values are inclusive, zero-terminated list). THE ARRAY DATA NEEDS TO PERSIST AS LONG AS THE FONT IS ALIVE.
pub GlyphMinAdvanceX: c_float,       // 0        // Minimum AdvanceX for glyphs, set Min to align font icons, set both Min/Max to enforce mono-space font
pub GlyphMaxAdvanceX: c_float,       // f32::MAX  // Maximum AdvanceX for glyphs
pub MergeMode: bool,              // false    // Merge into previous ImFont, so you can combine multiple inputs font into one ImFont (e.g. ASCII font + icons + Japanese glyphs). You may want to use GlyphOffset.y when merge font of different heights.
pub FontBuilderFlags: c_uint,       // 0        // Settings for custom font builder. THIS IS BUILDER IMPLEMENTATION DEPENDENT. Leave as zero if unsure.
pub RasterizerMultiply: c_float,     // 1f32     // Brighten (>1f32) or darken (<1f32) font output. Brightening small fonts may be a good workaround to make them more readable.
pub EllipsisChar: ImWchar,           // -1       // Explicitly specify unicode codepoint of ellipsis character. When fonts are being merged first specified ellipsis will be used.

    // [Internal]
    pub Name: [c_char;40],               // Name (strictly to ease debugging)
pub DstFont: *mut ImFont,

    // ImFontConfig();
}
