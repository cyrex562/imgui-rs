// Hold rendering data for one glyph.
// (Note: some language parsers may fail to convert the 31+1 bitfield members, in this case maybe drop store a single u32 or we can rework this)
#[derive(Clone,Debug,Default)]
pub struct DimgFontGlyph
{
    // unsigned int    colored : 1;        // Flag to indicate glyph is colored and should generally ignore tinting (make it usable with no shift on little-endian as this is used in loops)
    pub colored: bool,
    // unsigned int    visible : 1;        // Flag to indicate glyph has no visible pixels (e.g. space). Allow early out when rendering.
    pub visible: bool,
    // unsigned int    codepoint : 30;     // 0x0000..0x10FFFF
    pub codepoint: u32,
    // pub advance_x: f32,          // Distance to next character (= data from font + ImFontConfig::glyph_extra_spacing.x baked in)
    pub advance_x: f32,
    // float           x0, y0, x1, y1;     // Glyph corners
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,
    // float           u0, v0, u1, v1;     // Texture coordinates
    pub u0: f32,
    pub v0: f32,
    pub u1: f32,
    pub v1: f32,
}
