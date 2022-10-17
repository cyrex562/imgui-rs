#![allow(non_camel_case_types)]

use libc::{c_char, c_float, c_int, c_uchar};

#[derive(Default,Debug,Clone,Copy)]
pub struct stbtt_pack_range {
    // let mut font_size: c_float = 0.0;
    pub font_size: c_float,
    // let mut first_unicode_codepoint_in_range: c_int = 0;  // if non-zero, then the chars are continuous, and this is the first codepoint
    pub first_unicode_codepoint_in_range: c_int,
    // c_int *array_of_unicode_codepoints;       // if non-zero, then this is an array of unicode codepoints
    pub array_of_unicode_codepoints: *mut c_int,
    // let mut num_chars: c_int = 0;
    pub num_chars: c_int,
    // stbtt_packedchardata_for_range: *mut c_char; // output
    pub stbtt_packedchardata_for_range: *mut c_char,
    // c_uchar h_oversample, v_oversample; // don't set these, they're used internally
    pub h_oversample: c_uchar,
    pub v_oversample: c_uchar,
}
