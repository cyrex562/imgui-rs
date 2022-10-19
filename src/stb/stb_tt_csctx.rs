#![allow(non_camel_case_types)]typedef use libc::{c_float, c_int};
use crate::stb_tt_vertex::stbtt_vertex;

#[derive(Default, Debug, Clone, Copy)]
pub struct stbtt__csctx {
    // let mut bounds: c_int = 0;
    pub bounds: c_int,
    // let mut started: c_int = 0;first_x: c_float, first_y;x: c_float, y;
    pub started: c_int,
    pub first_x: c_float,
    pub first_y: c_float,
    pub x: c_float,
    pub y: c_float,
    // min_x: i32, max_x, min_y, max_y;
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
    // pvertices: *mut stbtt_vertex;
    pub pvertices: *mut stbtt_vertex,
    // let mut num_vertices: c_int = 0;
    pub num_vertices: c_int,
}
