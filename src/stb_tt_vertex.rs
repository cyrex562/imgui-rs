#![allow(non_camel_case_types)]

use libc::c_uchar;
use crate::stb_tt_types::stbtt_vertex_type;

// typedef struct
#[derive(Default, Debug, Copy, Clone)]
pub struct stbtt_vertex {
    // stbtt_vertex_type x,y,cx,cy,cx1,cy1;
    pub x: stbtt_vertex_type,
    pub y: stbtt_vertex_type,
    pub cx: stbtt_vertex_type,
    pub cy: stbtt_vertex_type,
    pub cx1: stbtt_vertex_type,
    pub cy1: stbtt_vertex_type,
    // c_uchar type,padding;
    pub vertex_type: c_uchar,
    pub padding: c_uchar,
}
