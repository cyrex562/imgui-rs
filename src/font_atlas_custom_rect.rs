#![allow(non_snake_case)]

use std::ptr::null_mut;
use libc::{c_float, c_uint, c_ushort};
use crate::font::ImFont;
use crate::vec2::ImVec2;

// See ImFontAtlas::AddCustomRectXXX functions.
#[derive(Default, Debug, Copy, Clone)]
pub struct ImFontAtlasCustomRect {
    // unsigned c_short  Width, Height;  // Input    // Desired rectangle dimension
    pub Width: c_ushort,
    pub Height: c_ushort,
    // unsigned c_short  X, Y;           // Output   // Packed position in Atlas
    pub X: c_ushort,
    pub Y: c_ushort,
    // c_uint    GlyphID;        // Input    // For custom font glyphs only (ID < 0x110000)
    pub GlyphID: c_uint,
    // c_float           GlyphAdvanceX;  // Input    // For custom font glyphs only: glyph xadvance
    pub GlyphAdvanceX: c_float,
    // ImVec2          GlyphOffset;    // Input    // For custom font glyphs only: glyph display offset
    pub GlyphOffset: ImVec2,
    // ImFont*         Font;           // Input    // For custom font glyphs only: target font
    pub Font: *mut ImFont,
}

impl ImFontAtlasCustomRect {
    // ImFontAtlasCustomRect()
    pub fn new() -> Self {
        Self {
            Width: 0,
            Height: 0,
            X: 0xFFFF,
            Y: 0xFFFF,
            GlyphID: 0,
            GlyphAdvanceX: 0f32,
            GlyphOffset: ImVec2::new2(0f32, 0f32),
            Font: null_mut(),
        }
    }

    // bool IsPacked() const
    pub fn IsPacked(&self) -> bool {
        return self.X != 0xFFFF;
    }
}
