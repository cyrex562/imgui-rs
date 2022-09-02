use crate::font::font::Font;
use crate::vectors::Vector2D;

// See ImFontAtlas::AddCustomRectXXX functions.
#[derive(Default, Debug, Clone)]
pub struct FontAtlasCustomRect {
    // unsigned short  width, height;  // Input    // Desired rectangle dimension
    pub width: u32,
    pub height: u32,
    // unsigned short  x, Y;           // Output   // Packed position in Atlas
    pub x: u16,
    pub Y: u16,
    // unsigned pub glyph_id: i32,      // Input    // For custom font glyphs only (id < 0x110000)
    pub glyph_id: u32,
    pub glyph_advance_x: f32,
    // Input    // For custom font glyphs only: glyph xadvance
    pub glyph_offset: Vector2D,
    // Input    // For custom font glyphs only: glyph display offset
    pub font: Font, // ImFont*         font;           // Input    // For custom font glyphs only: target font
}

impl FontAtlasCustomRect {
    // ImFontAtlasCustomRect()         { width = height = 0; x = Y = 0xFFFF; glyph_id = 0; glyph_advance_x = 0.0; glyph_offset = Vector2D(0, 0); font = None;
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            x: 0xFFFF,
            Y: 0xFFFF,
            glyph_id: 0,
            glyph_advance_x: 0.0,
            glyph_offset: Default::default(),
            font: Default::default(),
        }
    }
    //     bool is_packed() const           { return x != 0xFFFF; }
    pub fn is_packed(&self) -> bool {
        self.x != 0xFFFF
    }
}
