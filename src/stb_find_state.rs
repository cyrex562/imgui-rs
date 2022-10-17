use libc::{c_float, c_int};

// typedef struct
#[derive(Default, Debug, Copy, Clone)]
pub struct StbFindState {
    // x: c_float,y;    // position of n'th character
    pub x: c_float,
    pub y: c_float,
    // let mut height: c_float = 0.0; // height of line
    pub height: c_float,
    // first_char: c_int, length; // first char of row, and length
    pub first_char: c_int,
    pub length: c_int,
    // let mut prev_first: c_int = 0;  // first char of previous row
    pub prev_first: c_int,
}
// StbFindState;
