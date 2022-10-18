#![allow(non_camel_case_types)]

use libc::c_short;

pub type stbtt_vertex_type = c_short;

// typedef stbtt__test_oversample_pow2: c_int[(STBTT_MAX_OVERSAMPLE & (STBTT_MAX_OVERSAMPLE-1)) == 0 ? 1 : -1];
