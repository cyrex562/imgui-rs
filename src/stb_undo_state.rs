use libc::{c_int, c_short};
use crate::a_stb_textedit::{STB_TEXTEDIT_CHARTYPE, STB_TEXTEDIT_UNDOCHARCOUNT, STB_TEXTEDIT_UNDOSTATECOUNT};
use crate::stb_undo_record::StbUndoRecord;

#[derive(Default,Debug,Clone)]
pub struct StbUndoState
{
    // private data
    // StbUndoRecord          undo_rec [STB_TEXTEDIT_UNDOSTATECOUNT];
    pub undo_rec: [StbUndoRecord;STB_TEXTEDIT_UNDOSTATECOUNT],

    // STB_TEXTEDIT_CHARTYPE  undo_char[STB_TEXTEDIT_UNDOCHARCOUNT];
    pub undo_char: [STB_TEXTEDIT_CHARTYPE; STB_TEXTEDIT_UNDOCHARCOUNT],

    // c_short undo_point, redo_point;
    pub undo_point: c_short,
    pub redo_point: c_short,

    // undo_char_point: c_int, redo_char_point;
    pub undo_char_point: c_int,
    pub redo_char_point: c_int
}
