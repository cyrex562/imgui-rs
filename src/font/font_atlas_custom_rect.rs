use crate::font::ImFont;
use crate::core::vec2::Vector2;
use libc::{c_uint, c_ushort, size_t};

// See ImFontAtlas::AddCustomRectXXX functions.
#[derive(Default, Debug, Clone, Copy)]
pub struct ImFontAtlasCustomRect {
    // unsigned c_short  Width, Height;  // Input    // Desired rectangle dimension
    pub Width: size_t,
    pub Height: size_t,
    // unsigned c_short  X, Y;           // Output   // Packed position in Atlas
    pub X: c_ushort,
    pub Y: c_ushort,
    // c_uint    GlyphID;        // Input    // For custom font glyphs only (ID < 0x110000)GlyphAdvanceX: c_float;  // Input    // For custom font glyphs only: glyph xadvance
    pub GlyphID: c_uint,
    // ImVec2          GlyphOffset;    // Input    // For custom font glyphs only: glyph display offset
    pub GlyphOffset: Vector2,
    // ImFont*         Font;           // Input    // For custom font glyphs only: target font
    pub Font: *mut ImFont,
}

impl ImFontAtlasCustomRect {}
