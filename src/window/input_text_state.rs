#![allow(non_snake_case)]

use crate::input_text_flags::ImGuiInputTextFlags;
use crate::core::math_ops::ImMin;
use crate::stb::stb_text_edit_state::STB_TexteditState;
use crate::stb::stb_textedit::{stb_textedit_key, STB_TEXTEDIT_UNDOSTATECOUNT};
use crate::stb_text_edit_state::STB_TexteditState;
use crate::stb_textedit::STB_TEXTEDIT_UNDOSTATECOUNT;
use crate::core::type_defs::{ImWchar, ImguiHandle};
use libc::{c_float, c_int};

// Internal state of the currently focused/edited text input box
// For a given item ID, access with GetInputTextState()
#[derive(Default, Debug, Clone)]
pub struct ImGuiInputTextState {
    pub ID: ImguiHandle,
    // widget id owning the text state
    // c_int                     CurLenW, CurLenA;       // we need to maintain our buffer length in both UTF-8 and wchar format. UTF-8 length is valid even if TextA is not.
    pub CurLenW: usize,
    pub CurLenA: usize,
    pub TextW: Vec<char>,
    // edit buffer, we need to persist but can't guarantee the persistence of the user-provided buffer. so we copy into own buffer.
    pub TextA: Vec<char>,
    // temporary UTF8 buffer for callbacks and other operations. this is not updated in every code-path! size=capacity.
    pub InitialTextA: Vec<char>,
    // backup of end-user buffer at the time of focus (in UTF-8, unaltered)
    pub TextAIsValid: bool,
    // temporary UTF8 buffer is not initially valid before we make the widget active (until then we pull the data from user argument)
    pub BufCapacityA: usize,
    // end-user buffer capacity
    pub ScrollX: c_float,
    // horizontal scrolling/offset
    pub Stb: STB_TexteditState,
    // state for stb_textedit.h
    pub CursorAnim: c_float,
    // timer for cursor blink, reset on every user action so the cursor reappears immediately
    pub CursorFollow: bool,
    // set when we want scrolling to follow the current cursor position (not always!)
    pub SelectedAllMouseLock: bool,
    // after a double-click to select all, we ignore further mouse drags to update selection
    pub Edited: bool,
    // edited this frame
    pub Flags: ImGuiInputTextFlags, // copy of InputText() flags
}

impl ImGuiInputTextState {
    // ImGuiInputTextState()                   { memset(this, 0, sizeof(*this)); }

    // c_void        ClearText()                 { CurLenW = CurLenA = 0; TextW[0] = 0; TextA[0] = 0; CursorClamp(); }
    pub fn ClearText(&mut self) {
        self.CurLenA = 0;
        self.CurLenW = 0;
        self.TextW.clear();
        self.TextA.clear();
        self.CursorClamp();
    }

    // c_void        ClearFreeMemory()           { TextW.clear(); TextA.clear(); InitialTextA.clear(); }
    pub fn ClearFreeMemory(&mut self) {
        self.TextW.clear();
        self.TextA.clear();
        self.InitialTextA.clear();
    }

    // c_int         GetUndoAvailCount() const   { return Stb.undostate.undo_point; }
    pub fn GetUndoAvailCount(&self) -> c_int {
        self.Stb.undostate.undo_point as c_int
    }

    // c_int         GetRedoAvailCount() const   { return STB_TEXTEDIT_UNDOSTATECOUNT - Stb.undostate.redo_point; }
    pub fn GetRedoAvailcount(&self) -> c_int {
        (STB_TEXTEDIT_UNDOSTATECOUNT - self.Stb.undostate.redo_point) as c_int
    }

    // c_void        OnKeyPressed(key: c_int);      // Cannot be inline because we call in code in stb_textedit.h implementation
    pub unsafe fn OnKeyPressed(&mut self, key: c_int) {
        stb_textedit_key(&mut String::from(self.TextW.clone()), &mut self.Stb, key);
        self.CursorFollow = true;
        self.CursorAnimReset();
    }

    // Cursor & Selection
    // c_void        CursorAnimReset()           { CursorAnim = -0.3f32; }                                   // After a user-input the cursor stays on for a while without blinking
    pub fn CursorAnimReset(&mut self) {
        self.CursorAnim = -0.3f32;
    }

    // c_void        CursorClamp()               { Stb.cursor = ImMin(Stb.cursor, CurLenW); Stb.select_start = ImMin(Stb.select_start, CurLenW); Stb.select_end = ImMin(Stb.select_end, CurLenW); }
    pub fn CursorClamp(&mut self) {
        self.Stb.cursor = self.Stb.cursor.min(self.CurLenW);
        self.Stb.select_start = self.Stb.select_start.min(self.CurLenW);
        self.Stb.select_end = self.Stb.select_end.min(self.CurLenW);
    }

    // bool        HasSelection() const        { return Stb.select_start != Stb.select_end; }
    pub fn HasSelection(&self) -> bool {
        self.Stb.select_start != self.Stb.select_end
    }

    // c_void        ClearSelection()            { Stb.select_start = Stb.select_end = Stb.cursor; }
    pub fn ClearSelection(&mut self) {
        self.Stb.select_start = self.Stb.cursor;
        self.Stb.select_end = self.Stb.cursor;
    }

    // c_int         GetCursorPos() const        { return Stb.cursor; }
    pub fn GetCursorPos(&self) -> usize {
        self.Stb.cursor
    }

    // c_int         GetSelectionStart() const   { return Stb.select_start; }
    pub fn GetSelectionStart(&self) -> usize {
        self.Stb.select_start
    }

    // c_int         GetSelectionEnd() const     { return Stb.select_end; }
    pub fn GetSelectionEnd(&self) -> usize {
        self.Stb.select_end
    }

    // c_void        SelectAll()                 { Stb.select_start = 0; Stb.cursor = Stb.select_end = CurLenW; Stb.has_preferred_x = 0; }
    pub fn SelectAll(&mut self) {
        self.Stb.select_start = 0;
        self.Stb.cursor = self.CurLenW;
        self.Stb.select_end = self.CurLenW;
    }
}
