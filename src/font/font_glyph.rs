// Hold rendering data for one glyph.
// (Note: some language parsers may fail to convert the 31+1 bitfield members, in this case maybe drop store a single u32 or we can rework this)
#[derive(Clone,Debug,Default)]
pub struct FontGlyph
{
    // unsigned int    colored : 1;        // Flag to indicate glyph is colored and should generally ignore tinting (make it usable with no shift on little-endian as this is used in loops)
    pub colored: bool,
    // unsigned int    visible : 1;        // Flag to indicate glyph has no visible pixels (e.g. space). Allow early out when rendering.
    pub visible: bool,
    // unsigned int    codepoint : 30;     // 0x0000..0x10FFFF
    pub codepoint: char,
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


void ImFontGlyphRangesBuilder::add_text(const char* text, const char* text_end)
{
    while (text_end ? (text < text_end) : *text)
    {
        unsigned int c = 0;
        int c_len = text_char_from_utf8(&c, text, text_end);
        text += c_len;
        if (c_len == 0)
            break;
        AddChar((ImWchar)c);
    }
}

void ImFontGlyphRangesBuilder::AddRanges(const ImWchar* ranges)
{
    for (; ranges[0]; ranges += 2)
        for (unsigned int c = ranges[0]; c <= ranges[1] && c <= IM_UNICODE_CODEPOINT_MAX; c += 1) //-V560
            AddChar((ImWchar)c);
}

void ImFontGlyphRangesBuilder::BuildRanges(ImVector<ImWchar>* out_ranges)
{
    let max_codepoint = IM_UNICODE_CODEPOINT_MAX;
    for (int n = 0; n <= max_codepoint; n += 1)
        if (GetBit(n))
        {
            out_ranges.push_back((ImWchar)n);
            while (n < max_codepoint && GetBit(n + 1))
                n += 1;
            out_ranges.push_back((ImWchar)n);
        }
    out_ranges.push_back(0);
}
