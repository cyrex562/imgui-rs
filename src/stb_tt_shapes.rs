//////////////////////////////////////////////////////////////////////////////
#![allow(non_upper_case_globals)]
//
// GLYPH SHAPES (you probably don't need these, but they have to go before
// the bitmaps for C declaration-order reasons)
//

use libc::c_int;

// #ifndef STBTT_vmove // you can predefine these to use different values (but why?)
//    enum {
pub const STBTT_vmove: c_int = 1;
pub const STBTT_vline: c_int = 2;
pub const STBTT_vcurve: c_int = 3;
pub const STBTT_vcubic: c_int = 4;
// };
// #endif
