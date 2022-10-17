use crate::type_defs::ImWchar;
use libc::{c_int, size_t};
use crate::bit_vector::ImBitVector;

// #ifdef IMGUI_ENABLE_STB_TRUETYPE
// Temporary data for one source font (multiple source fonts can be merged into one destination ImFont)
// (C++03 doesn't allow instancing ImVector<> with function-local types so we declare the type here.)
#[derive(Default, Debug, Copy, Clone)]
pub struct ImFontBuildSrcData {
    // stbtt_fontinfo      FontInfo;
    pub FontInfo: stbtt_fontinfo,
    // stbtt_pack_range    PackRange;          // Hold the list of codepoints to pack (essentially points to Codepoints.Data)
    pub PackRange: stbtt_pack_range,
    // stbrp_rect*         Rects;              // Rectangle to pack. We first fill in their size and the packer will give us their position.
    pub Rects: *mut stbrp_rect,
    // stbtt_packedchar*   PackedChars;        // Output glyphs
    pub PackedChars: *mut stbtt_packedchar,
    // *const ImWchar      SrcRanges;          // Ranges as requested by user (user is allowed to request too much, e.g. 0x0020..0xFFF0)
    pub SrcRanges: *const ImWchar,
    // c_int                 DstIndex;           // Index into atlas->Fonts[] and dst_tmp_array[]
    pub DstIndex: size_t,
    // c_int                 GlyphsHighest;      // Highest requested codepoint
    pub GlyphsHighest: size_t,
    // c_int                 GlyphsCount;        // Glyph count (excluding missing glyphs and glyphs already set by an earlier source font)
    pub GlyphsCount: size_t,
    // ImBitVector         GlyphsSet;          // Glyph bit map (random access, 1-bit per codepoint. This will be a maximum of 8KB)
    pub GlyphsSet: ImBitVector,
    // Vec<c_int>       GlyphsList;         // Glyph codepoints list (flattened version of GlyphsMap)
    pub GlyphsLIst: Vec<c_int>,
}
