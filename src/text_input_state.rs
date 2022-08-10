use std::os::raw::c_char;
use crate::imgui_h::{Id32, InputTextFlags, ImWchar};
use crate::imgui_math::f32::min;
use crate::imstb_text_edit_state::STB_TexteditState;
use crate::stb::stb_textedit_h::STB_TEXTEDIT_UNDOSTATECOUNT;
use crate::types::Id32;

/// Internal state of the currently focused/edited text input box For a given item id, access with ImGui::GetInputTextState()
#[derive(Debug,Default,Clone)]
pub struct InputTextState
{
    // Id32                 id;                     // widget id owning the text state
    pub id: Id32,
    // int                     cur_len_w, cur_len_a;       // we need to maintain our buffer length in both UTF-8 and wchar format. UTF-8 length is valid even if text_a is not.
    pub cur_len_w: usize,
    pub cur_len_a: usize,
    // ImVector<ImWchar>       text_w;                  // edit buffer, we need to persist but can't guarantee the persistence of the user-provided buffer. so we copy into own buffer.
    pub text_w: String,
    // ImVector<char>          text_a;                  // temporary UTF8 buffer for callbacks and other operations. this is not updated in every code-path! size=capacity.
    pub text_a: String,
    // ImVector<char>          InitialTextA;           // backup of end-user buffer at the time of focus (in UTF-8, unaltered)
    pub initial_text: String,
    // bool                    text_ais_valid;           // temporary UTF8 buffer is not initially valid before we make the widget active (until then we pull the data from user argument)
    pub text_ais_valid: bool,
    // int                     buf_capacity_a;           // end-user buffer capacity
    pub buf_capacity_a: i32,
    // float                   scroll_x;                // horizontal scrolling/offset
    pub scroll_x: f32,
    // ImStb::StbTexteditState stb;                   // state for stb_textedit.h
    pub stb: STB_TexteditState,
    // float                   cursor_anim;             // timer for cursor blink, reset on every user action so the cursor reappears immediately
    pub cursor_anim: f32,
    // bool                    cursor_follow;           // set when we want scrolling to follow the current cursor position (not always!)
    pub cursor_follow: bool,
    // bool                    selected_all_mouse_lock;   // after a double-click to select all, we ignore further mouse drags to update selection
    pub selected_all_mouse_lock: bool,
    // bool                    edited;                 // edited this frame
    pub edited: bool,
    // ImGuiInputTextFlags     flags;                  // copy of InputText() flags
    pub flags: InputTextFlags,
}

impl InputTextState {
    // ImGuiInputTextState()                   { memset(this, 0, sizeof(*this)); }
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    //     void        clear_text()                 { cur_len_w = cur_len_a = 0; text_w[0] = 0; text_a[0] = 0; cursor_clamp(); }
    pub fn clear_text(&mut self) {
        self.cur_len_w = 0;
        self.cur_len_a = 0;
        self.text_w[0] = 0;
        self.text_a[0] = 0;
        self.cursor_clamp();
    }
    //     void        clear_free_memory()           { text_w.clear(); text_a.clear(); InitialTextA.clear(); }
    pub fn clear_free_memory(&mut self) {
        self.text_w.clear();
        self.text_a.clear();
    }
    //     int         get_undo_avail_count() const   { return stb.undostate.undo_point; }
    pub fn get_undo_avail_count(&mut self) -> i32 {
        self.stb.undostate.undo_point
    }

    //     int         get_redo_avail_count() const   { return STB_TEXTEDIT_UNDOSTATECOUNT - stb.undostate.redo_point; }
    pub fn get_redo_avail_count(&mut self) -> i32 {
        STB_TEXTEDIT_UNDOSTATECOUNT - self.stb.undostate.redo_point
    }
    //     void        on_key_pressed(int key);      // Cannot be inline because we call in code in stb_textedit.h implementation
    pub fn on_key_pressed(&mut self, key: i32) {
        todo!()
    }
    //
    //     // Cursor & Selection
    //     void        cursor_anim_reset()           { cursor_anim = -0.30; }
    pub fn cursor_anim_reset(&mut self) {
        self.cursor_anim = -0.30
    }
    // After a user-input the cursor stays on for a while without blinking
    //     void        cursor_clamp()               { stb.cursor = ImMin(stb.cursor, cur_len_w); stb.select_start = ImMin(stb.select_start, cur_len_w); stb.select_end = ImMin(stb.select_end, cur_len_w); }
    pub fn cursor_clamp(&mut self) {
        self.stb.cursor = usize::min(self.stb.cursor, self.cur_len_w);
        self.stb.select_start = usize::min(self.stb.select_start, self.cur_len_w)
    }
    //     bool        has_selection() const        { return stb.select_start != stb.select_end; }
    pub fn has_selection(&self) -> bool {
        self.stb.select_start != self.stb.select_end
    }
    //     void        clear_selection()            { stb.select_start = stb.select_end = stb.cursor; }
    pub fn clear_selection(&mut self) {
        self.stb.select_start = self.stb.cursor;
        self.stb.select_end = self.stb.cursor;
    }
    //     int         get_cursor_pos() const        { return stb.cursor; }
    pub fn get_cursor_pos(&self) -> i32 {
        self.stb.cursor
    }
    //     int         get_selection_start() const   { return stb.select_start; }
    pub fn get_selection_start(&self) -> i32 {
        self.stb.select_start
    }
    //     int         get_selection_end() const     { return stb.select_end; }
    pub fn get_selection_end(&self) -> i32 {
        self.stb.select_end
    }
    //     void        select_all()                 { stb.select_start = 0; stb.cursor = stb.select_end = cur_len_w; stb.has_preferred_x = 0; }
    pub fn select_all(&mut self) {
        self.stb.select_start = 0;
        self.stb.cursor = 0;
        self.stb.select_end = 0;
        self.cur_len_w = 0;
        self.stb.has_preferred_x = 0;
    }
}
