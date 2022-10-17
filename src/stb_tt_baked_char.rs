// typedef struct
#![allow(non_camel_case_types)]
// {
//    unsigned c_short x0,y0,x1,y1; // coordinates of bbox in bitmapxoff: c_float,yoff,xadvance;
// } stbtt_bakedchar;

use libc::c_ushort;

#[derive(Default, Debug, Copy, Clone)]
pub struct stbtt_bakedchar {
    pub x0: c_ushort,
    pub y0: c_ushort,
    pub x1: c_ushort,
    pub y1: c_ushort,
}