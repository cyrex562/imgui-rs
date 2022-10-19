#![allow(non_camel_case_types)]

use libc::{c_float, c_int, c_uchar};
use crate::stb_undo_state::StbUndoState;

#[derive(Debug, Default, Clone)]
pub struct STB_TexteditState {
    /////////////////////
    //
    // public data
    //
    pub cursor: c_int,
    // position of the text cursor within the string
    pub select_start: c_int,
    // selection start point
    pub select_end: c_int,
    // selection start and end point in characters; if equal, no selection.
    // note that start may be less than or greater than end (e.g. when
    // dragging the mouse, start is where the initial click was, and you
    // can drag in either direction)
    pub insert_mode: c_uchar,
    // each textfield keeps its own insert mode state. to keep an app-wide
    // insert mode, copy this value in/out of the app state
    pub row_count_per_page: c_int,
    // page size in number of row.
    // this value MUST be set to >0 for pageup or pagedown in multilines documents.

    /////////////////////
    //
    // private data
    //
    pub cursor_at_end_of_line: c_uchar,
    // not implemented yet
    pub initialized: c_uchar,
    pub has_preferred_x: c_uchar,
    pub single_line: c_uchar,
    // c_uchar padding1, padding2, padding3;
    pub padding1: c_uchar,
    pub padding2: c_uchar,
    pub padding3: c_uchar,
    pub preferred_x: c_float,
    // this determines where the cursor up/down tries to seek to along x
    pub undostate: StbUndoState,
}
