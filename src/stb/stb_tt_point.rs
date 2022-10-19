#![allow(non_camel_case_types)]

use libc::c_float;

#[derive(Default, Debug, Copy, Clone)]
pub struct stbtt__point {
    // x: c_float,y;
    pub x: c_float,
    pub y: c_float,
}
