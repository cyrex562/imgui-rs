#![allow(non_camel_case_types)]

use libc::{c_char, c_int};

// private structure
#[derive(Default, Debug, Copy, Clone)]
pub struct stbtt__buf {
    pub c_udata: *mut c_char,
    pub cursor: c_int,
    pub size: c_int,
}
