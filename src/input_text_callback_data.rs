use crate::input_text_flags::ImGuiInputTextFlags;
use crate::key::ImGuiKey;
use crate::type_defs::ImWchar;
use libc::{c_char, c_void, size_t};

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
    pub UserData: *mut c_void,
    // Arguments for the different callback events
    // - To modify the text buffer in a callback, prefer using the InsertChars() / DeleteChars() function. InsertChars() will take care of calling the resize callback if necessary.
    // - If you know your edits are not going to resize the underlying buffer allocation, you may modify the contents of 'Buf[]' directly. You need to update 'BufTextLen' accordingly (0 <= BufTextLen < BufSize) and set 'BufDirty'' to true so InputText can update its internal state.
    // ImWchar             EventChar;      // Character input                      // Read-write   // [CharFilter] Replace character with another one, or set to zero to drop. return 1 is equivalent to setting EventChar=0;
    pub EventChar: ImWchar,
    // ImGuiKey            EventKey;       // Key pressed (Up/Down/TAB)            // Read-only    // [Completion,History]
    pub EventKey: ImGuiKey,
    // char*               Buf;            // Text buffer                          // Read-write   // [Resize] Can replace pointer / [Completion,History,Always] Only write to pointed data, don't replace the actual pointer!
    pub Buf: *mut c_char,
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

    // c_void      InsertChars(pos: c_int, text: *const c_char, text_end: *const c_char = null_mut());

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
