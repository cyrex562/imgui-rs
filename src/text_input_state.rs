use std::os::raw::c_char;
use crate::imgui_h::{ImGuiID, ImGuiInputTextFlags, ImWchar};
use crate::imgui_math::ImMinI32;
use crate::imstb_text_edit_state::STB_TexteditState;

/// Internal state of the currently focused/edited text input box For a given item id, access with ImGui::GetInputTextState()
#[derive(Debug,Default,Clone)]
pub struct InputTextState
{
    // ImGuiID                 id;                     // widget id owning the text state
    pub ID: ImGuiID,
    // int                     CurLenW, CurLenA;       // we need to maintain our buffer length in both UTF-8 and wchar format. UTF-8 length is valid even if TextA is not.
    pub CurLenW: usize,
    pub CurLenA: usize,
    // ImVector<ImWchar>       TextW;                  // edit buffer, we need to persist but can't guarantee the persistence of the user-provided buffer. so we copy into own buffer.
    pub TextW: Vec<ImWchar>,
    // ImVector<char>          TextA;                  // temporary UTF8 buffer for callbacks and other operations. this is not updated in every code-path! size=capacity.
    pub TextA: Vec<c_char>,
    // ImVector<char>          InitialTextA;           // backup of end-user buffer at the time of focus (in UTF-8, unaltered)
    pub InitialText: Vec<c_char>,
    // bool                    TextAIsValid;           // temporary UTF8 buffer is not initially valid before we make the widget active (until then we pull the data from user argument)
    pub TextAIsValid: bool,
    // int                     BufCapacityA;           // end-user buffer capacity
    pub BufCapacityA: i32,
    // float                   ScrollX;                // horizontal scrolling/offset
    pub ScrollX: f32,
    // ImStb::STB_TexteditState Stb;                   // state for stb_textedit.h
    pub Stb: STB_TexteditState,
    // float                   CursorAnim;             // timer for cursor blink, reset on every user action so the cursor reappears immediately
    pub CursorAnim: f32,
    // bool                    CursorFollow;           // set when we want scrolling to follow the current cursor position (not always!)
    pub CursorFollow: bool,
    // bool                    SelectedAllMouseLock;   // after a double-click to select all, we ignore further mouse drags to update selection
    pub SelectedAllMouseLock: bool,
    // bool                    Edited;                 // edited this frame
    pub Edited: bool,
    // ImGuiInputTextFlags     flags;                  // copy of InputText() flags
    pub Flags: ImGuiInputTextFlags,
}

impl InputTextState {
    // ImGuiInputTextState()                   { memset(this, 0, sizeof(*this)); }
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    //     void        ClearText()                 { CurLenW = CurLenA = 0; TextW[0] = 0; TextA[0] = 0; CursorClamp(); }
    pub fn ClearText(&mut self) {
        self.CurLenW = 0;
        self.CurLenA = 0;
        self.TextW[0] = 0;
        self.TextA[0] = 0;
        self.CursorClamp();
    }
    //     void        clear_free_memory()           { TextW.clear(); TextA.clear(); InitialTextA.clear(); }
    pub fn ClearFreeMemory(&mut self) {
        self.TextW.clear();
        self.TextA.clear();
    }
    //     int         GetUndoAvailCount() const   { return Stb.undostate.undo_point; }
    pub fn GetUndoAvailCount(&mut self) -> i32 {
        self.Stb.undostate.undo_point
    }

    //     int         GetRedoAvailCount() const   { return STB_TEXTEDIT_UNDOSTATECOUNT - Stb.undostate.redo_point; }
    pub fn GetRedoAvailCount(&mut self) -> i32 {
        STB_TEXTEDIT_UNDOSTATECOUNT - self.Stb.undostate.redo_point
    }
    //     void        OnKeyPressed(int key);      // Cannot be inline because we call in code in stb_textedit.h implementation
    pub fn OnKeyPressed(&mut self, key: i32) {
        todo!()
    }
    //
    //     // Cursor & Selection
    //     void        CursorAnimReset()           { CursorAnim = -0.30; }
    pub fn CursorAnimReset(&mut self) {
        self.CursorAdnim = -0.30
    }
    // After a user-input the cursor stays on for a while without blinking
    //     void        CursorClamp()               { Stb.cursor = ImMin(Stb.cursor, CurLenW); Stb.select_start = ImMin(Stb.select_start, CurLenW); Stb.select_end = ImMin(Stb.select_end, CurLenW); }
    pub fn CursorClamp(&mut self) {
        self.Stb.cursor = ImMinI32(self.Stb.cursor, self.CurLenW);
        self.Stb.select_start = ImMinI32(self.Stb.select_start, self.CurLenW)
    }
    //     bool        HasSelection() const        { return Stb.select_start != Stb.select_end; }
    pub fn HasSelection(&self) -> bool {
        self.Stb.select_start != self.Stb.select_end
    }
    //     void        ClearSelection()            { Stb.select_start = Stb.select_end = Stb.cursor; }
    pub fn ClearSelection(&mut self) {
        self.Stb.select_start = self.Stb.cursor;
        self.Stb.select_end = self.Stb.cursor;
    }
    //     int         GetCursorPos() const        { return Stb.cursor; }
    pub fn GetCursorPos(&self) -> i32 {
        self.Stb.cursor
    }
    //     int         GetSelectionStart() const   { return Stb.select_start; }
    pub fn GetSelectionStart(&self) -> i32 {
        self.Stb.select_start
    }
    //     int         GetSelectionEnd() const     { return Stb.select_end; }
    pub fn GetSelectionEnd(&self) -> i32 {
        self.Stb.select_end
    }
    //     void        SelectAll()                 { Stb.select_start = 0; Stb.cursor = Stb.select_end = CurLenW; Stb.has_preferred_x = 0; }
    pub fn SelectAll(&mut self) {
        self.Stb.select_start = 0;
        self.Stb.cursor = 0;
        self.Stb.select_end = 0;
        self.CurLenW = 0;
        self.Stb.has_preferred_x = 0;
    }
}
