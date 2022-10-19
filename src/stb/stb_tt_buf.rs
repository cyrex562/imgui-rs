#![allow(non_camel_case_types)]

use libc::{c_char, c_int, size_t};

// private structure
#[derive(Default, Debug, Copy, Clone)]
pub struct stbtt__buf {
    pub c_udata: *mut c_char,
    pub cursor: size_t,
    pub size: size_t,
}
