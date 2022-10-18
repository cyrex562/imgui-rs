use libc::{c_uchar, c_void};

// The following structure is defined publicly so you can declare one on
// the stack or as a global or etc, but you should treat it as opaque.
#[derive(Default,Debug,Copy,Clone)]
pub struct stbtt_fontinfo
{
   // c_void           * userdata;
   pub userdata: *mut c_void,
   // c_uchar  * data;              // pointer to .ttf file
   pub data: *mut c_uchar,
   // c_int              fontstart;         // offset of start of font
    p
   // let mut numGlyphs: c_int = 0;                     // number of glyphs, needed for range checking

   // loca: c_int,head,glyf,hhea,hmtx,kern,gpos,svg; // table locations as offset from start of .ttf

    // let mut index_map: c_int = 0;                     // a cmap mapping for our chosen character encoding

    // let mut indexToLocFormat: c_int = 0;              // format needed to map from glyph index to glyph

   // let mut cff = stbtt__buf::default();                    // cff font data

   // let mut charstrings = stbtt__buf::default();            // the charstring index

   // let mut gsubrs = stbtt__buf::default();                 // global charstring subroutines index

   // let mut subrs = stbtt__buf::default();                  // private charstring subroutines index

   // let mut fontdicts = stbtt__buf::default();              // array of font dicts

   // let mut fdselect = stbtt__buf::default();               // map from glyph to fontdict

}
