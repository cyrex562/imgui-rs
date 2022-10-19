// typedef struct stbtt__edge {x0: c_float,y0, x1,y1;
//    let mut invert: c_int = 0;
// } stbtt__edge;

use libc::{c_float, c_int};

#[derive(Default,Debug,Copy,Clone)]
pub struct stbtt__edge {
    pub x0: c_float,
    pub y0: c_float,
    pub x1: c_float,
    pub y1: c_float,
    pub invert: c_int,
}