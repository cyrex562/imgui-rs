#![allow(non_snake_case)]

use libc::{c_float, c_int};
use crate::type_defs::{ImGuiID, ImGuiInputTextFlags, ImWchar};

// Internal state of the currently focused/edited text input box
// For a given item ID, access with ImGui::GetInputTextState()
#[derive(Default, Debug, Clone)]
pub struct ImGuiInputTextState {
    pub ID: ImGuiID,
    // widget id owning the text state
// c_int                     CurLenW, CurLenA;       // we need to maintain our buffer length in both UTF-8 and wchar format. UTF-8 length is valid even if TextA is not.
    pub CurLenW: c_int,
    pub CurLenA: c_int,
    pub TextW: Vec<ImWchar>,
    // edit buffer, we need to persist but can't guarantee the persistence of the user-provided buffer. so we copy into own buffer.
    pub TextA: Vec<char>,
    // temporary UTF8 buffer for callbacks and other operations. this is not updated in every code-path! size=capacity.
    pub InitialTextA: Vec<char>,
    // backup of end-user buffer at the time of focus (in UTF-8, unaltered)
    pub TextAIsValid: bool,
    // temporary UTF8 buffer is not initially valid before we make the widget active (until then we pull the data from user argument)
    pub BufCapacityA: c_int,
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
    pub Flags: ImGuiInputTextFlags,                  // copy of InputText() flags
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
        Stb.undostate.undo_point
    }


    // c_int         GetRedoAvailCount() const   { return STB_TEXTEDIT_UNDOSTATECOUNT - Stb.undostate.redo_point; }
    pub fn GetRedoAvailcount(&self) -> c_int {
        STB_TEXTEDIT_UNDOSTATECOUNT - self.Stb.undostate.redo_point
    }


    // c_void        OnKeyPressed(c_int key);      // Cannot be inline because we call in code in stb_textedit.h implementation
    pub fn OnKeyPressed(&mut self, key: c_int) {
       todo!()
    }

    // Cursor & Selection
    // c_void        CursorAnimReset()           { CursorAnim = -0.3f32; }                                   // After a user-input the cursor stays on for a while without blinking
    pub fn CursorAnimReset(&mut self) {
        self.CursorAnim = -0.3f32;
    }

    // c_void        CursorClamp()               { Stb.cursor = ImMin(Stb.cursor, CurLenW); Stb.select_start = ImMin(Stb.select_start, CurLenW); Stb.select_end = ImMin(Stb.select_end, CurLenW); }
    pub fn CursorClamp(&mut self) {
        self.Stb.cursor = ImMin(self.Stb.cursor, self.CurLenW);
        self.Stb.select_start = ImMin(self.Stb.select_start, self.CurLenW);
        self.Stb.select_end = ImMin(self.stb.select_end, self.CurLenW);
    }

    // bool        HasSelection() const        { return Stb.select_start != Stb.select_end; }


    // c_void        ClearSelection()            { Stb.select_start = Stb.select_end = Stb.cursor; }


    // c_int         GetCursorPos() const        { return Stb.cursor; }


    // c_int         GetSelectionStart() const   { return Stb.select_start; }


    // c_int         GetSelectionEnd() const     { return Stb.select_end; }


    // c_void        SelectAll()                 { Stb.select_start = 0; Stb.cursor = Stb.select_end = CurLenW; Stb.has_preferred_x = 0; }
}
