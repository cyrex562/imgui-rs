#![allow(non_snake_case)]

use libc::{c_float, c_uint};

// Hold rendering data for one glyph.
// (Note: some language parsers may fail to convert the 31+1 bitfield members, in this case maybe drop store a single or: u32 we can rework this)
#[derive(Default, Debug, Clone, Copy)]
pub struct ImFontGlyph {
    pub Colored: bool,
    //unsigned c_int    Colored : 1;        // Flag to indicate glyph is colored and should generally ignore tinting (make it usable with no shift on little-endian as this is used in loops)
    pub Visible: bool,
    // unsigned c_int    Visible : 1;        // Flag to indicate glyph has no visible pixels (e.g. space). Allow early out when rendering.
    pub CodePoint: c_uint,
    // unsigned c_int    Codepoint : 30;     // 0x0000..0x10FFFF
    pub AdvanceX: c_float,
    // c_float           AdvanceX;           // Distance to next character (= data from font + ImFontConfig::GlyphExtraSpacing.x baked in)
    // c_float           X0, Y0, X1, Y1;     // Glyph corners
    pub X0: c_float,
    pub Y0: c_float,
    pub X1: c_float,
    pub Y1: c_float,
    // c_float           U0, V0, U1, V1;     // Texture coordinates
    pub U0: c_float,
    pub V0: c_float,
    pub U1: c_float,
    pub V1: c_float,
}
