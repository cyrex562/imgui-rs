#![allow(non_camel_case_types)]

use libc::{c_char, c_int, c_uint, c_void};

// this is an opaque structure that you shouldn't mess with which holds
// all the context needed from PackBegin to PackEnd.
#[derive(Default, Debug, Clone, Copy)]
pub struct stbtt_pack_context {
    // c_void *user_allocator_context;
    pub user_allocator_context: *mut c_void,
    // c_void *pack_info;
    pub pack_info: *mut c_void,
    // c_int   width;
    pub width: c_int,
    // c_int   height;
    pub height: c_int,
    // c_int   stride_in_bytes;
    pub stride_in_bytes: c_int,
    // c_int   padding;
    pub padding: c_int,
    // c_int   skip_missing;
    pub skip_missing: c_int,
    // c_uint   h_oversample, v_oversample;
    pub h_oversample: c_uint,
    pub v_oversample: c_uint,
    // c_upixels: *mut c_char;
    pub c_upixels: *mut c_char,
    // c_void  *nodes;
    pub nodes: * c_void,
}
