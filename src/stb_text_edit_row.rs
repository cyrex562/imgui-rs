
////////////////////////////////////////////////////////////////////////
//
//     StbTexteditRow
//
// Result of layout query, used by stb_textedit to determine where
// the text in each row is.

// result of layout query
// typedef use libc::c_int;
//
// struct
// {x0: c_float,x1;             // starting x location, end x location (allows for align=right, etc)
//    let mut baseline_y_delta: c_float = 0.0;  // position of baseline relative to previous row's baselineymin: c_float,ymax;         // height of row above and below baseline
//    let mut num_chars: c_int = 0;
// } StbTexteditRow;
// // #endif //INCLUDE_STB_TEXTEDIT_H

use libc::{c_float, c_int};

#[derive(Default,Debug,Copy,Clone)]
pub struct StbTextEditRow {
    pub x0: c_float,
    pub x1: c_float,
    pub baseline_y_delta: c_float,
    pub num_chars: c_int
}
