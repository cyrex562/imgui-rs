use libc::c_int;
use crate::bit_vector::ImBitVector;

// Temporary data for one destination ImFont* (multiple source fonts can be merged into one destination ImFont)
#[derive(Default,Debug,Clone,Copy)]
pub struct ImFontBuildDstData
{
    // c_int                 SrcCount;           // Number of source fonts targeting this destination font.
    pub SrcCount: c_int,
    // c_int                 GlyphsHighest;
    pub GlyphsHighest: c_int,
    // c_int                 GlyphsCount;
    pub GlyphsCount: c_int,
    // ImBitVector         GlyphsSet;          // This is used to resolve collision when multiple sources are merged into a same destination font.
    pub GlyphsSet: ImBitVector,
}
