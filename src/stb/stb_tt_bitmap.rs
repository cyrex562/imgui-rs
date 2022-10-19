// @TODO: don't expose this structure
#![allow(non_camel_case_types)]
typedef use libc::{c_char, c_int};

pub struct stbtt_bitmap
{
   // w: c_int,h,stride;
  pub w: c_int,
   pub h: c_int,
   pub stride: c_int,
   // c_upixels: *mut c_char;
   pub c_upixels: *mut c_char
} 
