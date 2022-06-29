use std::ffi::c_void;
use crate::imgui_h::{ImGuiInputTextFlags, ImGuiKey};

// Shared state of InputText(), passed as an argument to your callback when a ImGuiInputTextFlags_Callback* flag is used.
// The callback function should return 0 by default.
// Callbacks (follow a flag name and see comments in ImGuiInputTextFlags_ declarations for more details)
// - ImGuiInputTextFlags_CallbackEdit:        Callback on buffer edit (note that InputText() already returns true on edit, the callback is useful mainly to manipulate the underlying buffer while focus is active)
// - ImGuiInputTextFlags_CallbackAlways:      Callback on each iteration
// - ImGuiInputTextFlags_CallbackCompletion:  Callback on pressing TAB
// - ImGuiInputTextFlags_CallbackHistory:     Callback on pressing Up/Down arrows
// - ImGuiInputTextFlags_CallbackCharFilter:  Callback on character inputs to replace or discard them. Modify 'EventChar' to replace or discard, or return 1 in callback to discard.
// - ImGuiInputTextFlags_CallbackResize:      Callback on buffer capacity changes request (beyond 'buf_size' parameter value), allowing the string to grow.
pub struct ImGuiInputTextCallbackData
{
    pub EventFlag: ImGuiInputTextFlags,      // One ImGuiInputTextFlags_Callback*    // Read-only
    pub Flags: ImGuiInputTextFlags,          // What user passed to InputText()      // Read-only
    pub UserData: *mut c_void,       // What user passed to InputText()      // Read-only

    // Arguments for the different callback events
    // - To modify the text buffer in a callback, prefer using the InsertChars() / DeleteChars() function. InsertChars() will take care of calling the resize callback if necessary.
    // - If you know your edits are not going to resize the underlying buffer allocation, you may modify the contents of 'Buf[]' directly. You need to update 'BufTextLen' accordingly (0 <= BufTextLen < BufSize) and set 'BufDirty'' to true so InputText can update its internal state.
    pub EventChar: u8,      // Character input                      // Read-write   // [CharFilter] Replace character with another one, or set to zero to drop. return 1 is equivalent to setting EventChar=0;
    pub            EventKey: ImGuiKey,       // Key pressed (Up/Down/TAB)            // Read-only    // [Completion,History]
    pub               Buf: String,            // Text buffer                          // Read-write   // [Resize] Can replace pointer / [Completion,History,Always] Only write to pointed data, don't replace the actual pointer!
    pub BufTextLen: i32,   // Text length (in bytes)               // Read-write   // [Resize,Completion,History,Always] Exclude zero-terminator storage. In C land: == strlen(some_text), in C++ land: string.length()
    pub BufSize: i32,      // Buffer size (in bytes) = capacity+1  // Read-only    // [Resize,Completion,History,Always] Include zero-terminator storage. In C land == ARRAYSIZE(my_char_array), in C++ land: string.capacity()+1
    pub BufDirty: bool,       // Set if you modify Buf/BufTextLen!    // Write        // [Completion,History,Always]
    pub CursorPos: i32,    //                                      // Read-write   // [Completion,History,Always]
    pub                 SelectionStart: i32, //                                      // Read-write   // [Completion,History,Always] == to SelectionEnd when no selection)
    pub SelectionEnd: i32, //                                      // Read-write   // [Completion,History,Always]

    // Helper functions for text manipulation.
    // Use those function to benefit from the CallbackResize behaviors. Calling those function reset the selection.

}

impl ImGuiInputTextCallbackData {
    //  ImGuiInputTextCallbackData();
    pub fn ImGuiInputTextCallbackData() {
        todo!();
    }
    //  void      DeleteChars(int pos, int bytes_count);
    pub fn DeleteChars(pos: i32, bytes_count: i32) {
        todo!();
    }
    //  void      InsertChars(int pos, const char* text, const char* text_end = NULL);
    pub fn InsertChars(pos: i32, text: &str, text_end: &str) {
        todo!();
    }
    // void                SelectAll()             { SelectionStart = 0; SelectionEnd = BufTextLen; }
    pub fn SelectAll(&mut self) {
        self.SelectionStart = 0;
        self.SelectionEnd = self.BufTextLen;
    }
    // void                ClearSelection()        { SelectionStart = SelectionEnd = BufTextLen; }
    pub fn ClearSelection(&mut self) {
        self.SelectionStart = self.BufTextLen;
        self.SelectionEnd = self.BufTextLen;
    }
    // bool                HasSelection() const    { return SelectionStart != SelectionEnd; }
    pub fn HasSelection(&self) -> bool {
        self.SelectionStart != self.SelectionEnd
    }
}
