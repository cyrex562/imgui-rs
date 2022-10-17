use libc::size_t;
use crate::bit_vector::ImBitVector;

// Temporary data for one destination ImFont* (multiple source fonts can be merged into one destination ImFont)
#[derive(Default, Debug, Copy, Clone)]
pub struct ImFontBuildDstData {
    // c_int                 SrcCount;           // Number of source fonts targeting this destination font.
    pub SrcCount: size_t,
    // c_int                 GlyphsHighest;
    pub GlyphsHighest: size_t,
    // c_int                 GlyphsCount;
    pub GlyphsCount: size_t,
    // ImBitVector         GlyphsSet;          // This is used to resolve collision when multiple sources are merged into a same destination font.
    pub GlyphsSet: ImBitVector,
}
