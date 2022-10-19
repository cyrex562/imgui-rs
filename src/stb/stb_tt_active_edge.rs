use libc::{c_float, c_int};

#[derive(Default, Debug, Copy, Clone)]
pub struct stbtt__active_edge {
    // struct stbtt__active_edge *next;
    pub next: *mut stbtt__active_edge,
    // #if STBTT_RASTERIZER_VERSION==1
    // x: c_int,dx;
    // let mut ey: c_float = 0.0;
    // let mut direction: c_int = 0;
    // #elif STBTT_RASTERIZER_VERSION==2f
    //  x: c_float,fdx,fdy;
    pub x: c_float,
    pub fdx: c_float,
    pub fdy: c_float,
    pub direction: c_float,
    pub sy: c_float,
    pub ey: c_float,
    // #else
    // #error "Unrecognized value of STBTT_RASTERIZER_VERSION"
    // #endif
}
