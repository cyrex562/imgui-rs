#![allow(non_camel_case_types)]

use libc::{c_int, c_uchar, c_void};
use crate::stb_tt_buf::stbtt__buf;

// The following structure is defined publicly so you can declare one on
// the stack or as a global or etc, but you should treat it as opaque.
#[derive(Default, Debug, Copy, Clone)]
pub struct stbtt_fontinfo {
    // c_void           * userdata;
    pub userdata: *mut c_void,
    // c_uchar  * data;              // pointer to .ttf file
    pub data: *mut c_uchar,
    // c_int              fontstart;         // offset of start of font
    pub fontstart: c_int,
    // let mut numGlyphs: c_int = 0;                     // number of glyphs, needed for range checking
    pub numGlyphs: c_int,
    // loca: c_int,head,glyf,hhea,hmtx,kern,gpos,svg; // table locations as offset from start of .ttf
    pub loca: c_int,
    pub head: c_int,
    pub glyf: c_int,
    pub hhea: c_int,
    pub hmtx: c_int,
    pub kern: c_int,
    pub gpos: c_int,
    pub svg: c_int,
    // let mut index_map: c_int = 0;                     // a cmap mapping for our chosen character encoding
    pub index_map: c_int,
    // let mut indexToLocFormat: c_int = 0;              // format needed to map from glyph index to glyph
    pub indexToLocFormat: c_int,
    // let mut cff = stbtt__buf::default();                    // cff font data
    pub cff: stbtt__buf,
    // let mut charstrings = stbtt__buf::default();            // the charstring index
    pub charstrings: stbtt__buf,
    // let mut gsubrs = stbtt__buf::default();                 // global charstring subroutines index
    pub gsubrs: stbtt__buf,
    // let mut subrs = stbtt__buf::default();                  // private charstring subroutines index
    pub subrs: stbtt__buf,
    // let mut fontdicts = stbtt__buf::default();              // array of font dicts
    pub fontdicts: stbtt__buf,
    // let mut fdselect = stbtt__buf::default();               // map from glyph to fontdict
    pub fdselect: stbtt__buf,
}
