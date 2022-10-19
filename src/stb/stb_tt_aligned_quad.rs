#![allow(non_camel_case_types)]typedef use libc::c_float;

#[derive(Default, Debug, Copy, Clone)]
pub struct stbtt_aligned_quad {
    // x0: c_float,y0,s0,t0; // top-leftx1: c_float,y1,s1,t1; // bottom-right
    pub x0: c_float,
    pub y0: c_float,
    pub s0: c_float,
    pub t0: c_float,
}
