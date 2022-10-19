#![allow(non_camel_case_types)]typedef use libc::c_int;

// struct
// {
//    width: c_int,height;
//    x: c_int,y,bottom_y;
// } stbrp_context;

#[derive(Default,Debug,Copy, Clone)]
pub struct stbrp_context {
    pub width: c_int,
    pub height: c_int,
    pub x: c_int,
    pub y: c_int,
    pub bottom_y: c_int
}