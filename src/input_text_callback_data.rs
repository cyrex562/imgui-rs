use crate::input_text_flags::{ImGuiInputTextFlags, ImGuiInputTextFlags_CallbackResize};
use crate::key::ImGuiKey;
use crate::type_defs::ImWchar;
use libc::{c_char, c_int, c_void, size_t};
use crate::GImGui;
use crate::input_text_state::ImGuiInputTextState;
use crate::math_ops::{ImClamp, ImMax};
use crate::utils::flag_set;

// Shared state of InputText(), passed as an argument to your callback when a ImGuiInputTextFlags_Callback* flag is used.
// The callback function should return 0 by default.
// Callbacks (follow a flag name and see comments in ImGuiInputTextFlags_ declarations for more details)
// - ImGuiInputTextFlags_CallbackEdit:        Callback on buffer edit (note that InputText() already returns true on edit, the callback is useful mainly to manipulate the underlying buffer while focus is active)
// - ImGuiInputTextFlags_CallbackAlways:      Callback on each iteration
// - ImGuiInputTextFlags_CallbackCompletion:  Callback on pressing TAB
// - ImGuiInputTextFlags_CallbackHistory:     Callback on pressing Up/Down arrows
// - ImGuiInputTextFlags_CallbackCharFilter:  Callback on character inputs to replace or discard them. Modify 'EventChar' to replace or discard, or return 1 in callback to discard.
// - ImGuiInputTextFlags_CallbackResize:      Callback on buffer capacity changes request (beyond 'buf_size' parameter value), allowing the string to grow.
#[derive(Default, Debug, Clone, Copy)]
pub struct ImGuiInputTextCallbackData {
    // EventFlag: ImGuiInputTextFlags;      // One ImGuiInputTextFlags_Callback*    // Read-only
    pub EventFlag: ImGuiInputTextFlags,
    // Flags: ImGuiInputTextFlags;          // What user passed to InputText()      // Read-only
    pub Flags: ImGuiInputTextFlags,
    // *mut c_void               UserData;       // What user passed to InputText()      // Read-only
    pub UserData: Vec<u8>,
    // Arguments for the different callback events
    // - To modify the text buffer in a callback, prefer using the InsertChars() / DeleteChars() function. InsertChars() will take care of calling the resize callback if necessary.
    // - If you know your edits are not going to resize the underlying buffer allocation, you may modify the contents of 'Buf[]' directly. You need to update 'BufTextLen' accordingly (0 <= BufTextLen < BufSize) and set 'BufDirty'' to true so InputText can update its internal state.
    // ImWchar             EventChar;      // Character input                      // Read-write   // [CharFilter] Replace character with another one, or set to zero to drop. return 1 is equivalent to setting EventChar=0;
    pub EventChar: ImWchar,
    // ImGuiKey            EventKey;       // Key pressed (Up/Down/TAB)            // Read-only    // [Completion,History]
    pub EventKey: ImGuiKey,
    // char*               Buf;            // Text buffer                          // Read-write   // [Resize] Can replace pointer / [Completion,History,Always] Only write to pointed data, don't replace the actual pointer!
    pub Buf: String,
    // c_int                 BufTextLen;     // Text length (in bytes)               // Read-write   // [Resize,Completion,History,Always] Exclude zero-terminator storage. In C land: == strlen(some_text), in C++ land: string.length()
    pub BufTextLen: size_t,
    // c_int                 BufSize;        // Buffer size (in bytes) = capacity+1  // Read-only    // [Resize,Completion,History,Always] Include zero-terminator storage. In C land == ARRAYSIZE(my_char_array), in C++ land: string.capacity()+1
    pub BufSize: size_t,
    // bool                BufDirty;       // Set if you modify Buf/BufTextLen!    // Write        // [Completion,History,Always]
    pub BufDirty: bool,
    // c_int                 CursorPos;      //                                      // Read-write   // [Completion,History,Always]
    pub CursorPos: size_t,
    // c_int                 SelectionStart; //                                      // Read-write   // [Completion,History,Always] == to SelectionEnd when no selection)
    pub SelectionStart: size_t,
    // c_int                 SelectionEnd;   //                                      // Read-write   // [Completion,History,Always]
    pub SelectionEnd: size_t,
}

impl ImGuiInputTextCallbackData {
    // Helper functions for text manipulation.
    // Use those function to benefit from the CallbackResize behaviors. Calling those function reset the selection.
    //  ImGuiInputTextCallbackData();

    // c_void      DeleteChars(pos: c_int, bytes_count: c_int);
    // FIXME: The existence of this rarely exercised code path is a bit of a nuisance.
pub unsafe fn DeleteChars(&mut self, pos: usize, bytes_count: usize)
{
    // IM_ASSERT(pos + bytes_count <= BufTextLen);
    // TODO
    // dst: *mut c_char = self.Buf + pos;
    // let mut  src: &str = self.Buf + pos + bytes_count;
    // while ( c: c_char = *src++) {
    //     *dst + + = c;
    // }
    // *dst = '\0';

    if self.CursorPos >= pos + bytes_count {
        self.CursorPos -= bytes_count;
    }
    else if self.CursorPos >= pos {
        self.CursorPos = pos;}
    self.SelectionEnd = self.CursorPos;
    self.SelectionStart = self.SelectionEnd;
    self.BufDirty = true;
    self.BufTextLen -= bytes_count;
}

    // c_void      InsertChars(pos: c_int, text: &String, text_end: *const c_char = null_mut());
    pub unsafe fn InsertChars(&mut self, pos: usize, new_text: String, new_text_end: &str)
{
    let is_resizable: bool = flag_set(self.Flags, ImGuiInputTextFlags_CallbackResize);
    let new_text_len: usize = if new_text_end { (new_text_end - new_text)} else {new_text.len()};
    if new_text_len + self.BufTextLen >= BufSize
    {
        if !is_resizable { return ; }

        // Contrary to STB_TEXTEDIT_INSERTCHARS() this is working in the UTF8 buffer, hence the mildly similar code (until we remove the U16 buffer altogether!)
        let g = GImGui; // ImGuiContext& g = *GImGui;
        edit_state: &mut ImGuiInputTextState = &mut g.InputTextState;
        // IM_ASSERT(edit_state.ID != 0 && g.ActiveId == edit_state.ID);
        // IM_ASSERT(Buf == edit_state->TextA.Data);
        let new_buf_size: usize = self.BufTextLen + ImClamp(new_text_len * 4, 32, ImMax(256, new_text_len)) + 1;
        edit_state.TextA.reserve(new_buf_size + 1);
        Buf = edit_state.TextA.Data;
        BufSize = edit_state.BufCapacityA = new_buf_size;
    }

    if (self.BufTextLen != pos) {
        // TODO
        // memmove(self.Buf + pos + new_text_len, self.Buf + pos, (self.BufTextLen - pos));
    }
    // TODO
    // memcpy(Buf + pos, new_text, new_text_len * sizeof);

    self.Buf[self.BufTextLen + new_text_len] = '\0';

    if self.CursorPos >= pos {
        self.CursorPos += new_text_len;
    }
    self.SelectionEnd = self.CursorPos;
    self.SelectionStart = self.SelectionEnd;
    self.BufDirty = true;
    self.BufTextLen += new_text_len;
}


    // c_void                SelectAll()             { SelectionStart = 0; SelectionEnd = BufTextLen; }
    pub fn SelectAll(&mut self) {
        self.SelectionStart = 0;
        self.SelectionEnd = self.BufTextLen;
    }

    // c_void                ClearSelection()        { SelectionStart = SelectionEnd = BufTextLen; }
    pub fn ClearSelection(&mut self) {
        self.SelectionStart = self.BufTextLen;
        self.SelectionEnd = self.BufTextLen;
    }

    // bool                HasSelection() const    { return SelectionStart != SelectionEnd; }
    pub fn HasSelection(&self) -> bool {
        self.SelectionStart != self.SelectionEnd
    }
}
