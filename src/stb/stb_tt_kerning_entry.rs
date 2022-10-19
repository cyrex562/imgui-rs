#![allow(non_camel_case_types)]

use libc::c_int;

// typedef struct stbtt_kerningentry
#[derive(Default,Debug,Clone,Copy)]
pub struct stbtt_kerningentry
{
   // let mut glyph1: c_int = 0; // use stbtt_FindGlyphIndex
    pub glyph1: c_int,
   // let mut glyph2: c_int = 0;
    pub glyph2: c_int,
   // let mut advance: c_int = 0;
    pub advance: c_int
}
