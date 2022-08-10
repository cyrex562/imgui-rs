use crate::imstb_textedit_h::StbUndoState;
use crate::stb::stb_textedit_h::StbUndoState;

#[derive(Debug,Default,Clone)]
pub struct StbTexteditState {
    // int cursor;
    pub cursor: usize,
    // position of the text cursor within the string
    // int select_start;          // selection start point
    pub select_start: usize,
    // int select_end;
    pub select_end: usize,
    // selection start and end point in characters; if equal, no selection.
    // note that start may be less than or greater than end (e.g. when
    // dragging the mouse, start is where the initial click was, and you
    // can drag in either direction)

    // unsigned char insert_mode;
    pub insert_mode: u8,
    // each textfield keeps its own insert mode state. to keep an app-wide
    // insert mode, copy this value in/out of the app state

    // int row_count_per_page;
    pub row_count_per_page: usize,
    // page size in number of row.
    // this value MUST be set to >0 for pageup or pagedown in multilines documents.
    // unsigned char cursor_at_end_of_line; // not implemented yet
    pub cursor_at_end_of_line: bool,
    // unsigned char initialized;
    pub initialized: bool,
    // unsigned char has_preferred_x;
    pub has_preferred_x: bool,
    // unsigned char single_line;
    pub single_line: bool,
    // unsigned char padding1, padding2, padding3;
    pub padding1: u8,
    pub padding2: u8,
    pub padding3: u8,
    // float preferred_x; // this determines where the cursor up/down tries to seek to along x
    pub preferred_x: f32,
    // StbUndoState undostate;
    pub undostate: StbUndoState,
}
