use std::os::raw::c_char;
use std::ptr;
use crate::imgui_h::{IM_UNICODE_CODEPOINT_INVALID, IM_UNICODE_CODEPOINT_MAX, ImGuiID, ImGuiInputTextFlags, ImWchar};
use crate::imgui_math::ImMinI32;
use crate::imstb_textedit_h::STB_TexteditState;

// Convert UTF-8 to 32-bit character, process single character input.
// A nearly-branchless UTF-8 decoder, based on work of Christopher Wellons (https://github.com/skeeto/branchless-utf8).
// We handle UTF-8 decoding error by skipping forward.
// int ImTextCharFromUtf8(unsigned int* out_char, const char* in_text, const char* in_text_end)

pub unsafe fn ImTextCharFromUtf8(out_char: *mut u32, in_text: *const c_char, mut in_text_end: *mut c_char) -> i32 {
    const lengths: [u8;32] = [ 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0,
    2, 2, 2, 2, 3, 3, 4, 0 ];
    const masks: [i32;5]  = [ 0x00, 0x7f, 0x1f, 0x0f, 0x07 ];
    const mins: [u32;5] = [ 0x400000, 0, 0x80, 0x800, 0x10000 ];
    const shiftc: [i32;5] = [ 0, 18, 12, 6, 0 ];
    const shifte: [i32;5] = [ 0, 6, 4, 2, 0 ];
    let len = lengths[*in_text >> 3];
    let mut wanted = len + !len;

    if in_text_end.is_null() {
        in_text_end = in_text + wanted;
    }// Max length, nulls will be taken into account.

    // Copy at most 'len' bytes, stop copying at 0 or past in_text_end. Branch predictor does a good job here,
    // so it is fast even with excessive branching.
    let mut s: [u8;4] = [0;4];
    s[0] = if in_text + 0 < in_text_end { in_text[0] } else { 0 };
    s[1] = if in_text + 1 < in_text_end { in_text[1] } else { 0 };
    s[2] = if in_text + 2 < in_text_end { in_text[2] } else { 0 };
    s[3] = if in_text + 3 < in_text_end { in_text[3] } else { 0 };

    // Assume a four-byte character and load four bytes. Unused bits are shifted out.
    *out_char  = (s[0] & masks[len]) << 18;
    *out_char |= (s[1] & 0x3f) << 12;
    *out_char |= (s[2] & 0x3f) <<  6;
    *out_char |= (s[3] & 0x3f) <<  0;
    *out_char >>= shiftc[len];

    // Accumulate the various error conditions.
    let mut e = 0;
    e  = (*out_char < mins[len]) << 6; // non-canonical encoding
    e |= ((*out_char >> 11) == 0x1b) << 7;  // surrogate half?
    e |= (*out_char > IM_UNICODE_CODEPOINT_MAX) << 8;  // out of range?
    e |= (s[1] & 0xc0) >> 2;
    e |= (s[2] & 0xc0) >> 4;
    e |= (s[3]       ) >> 6;
    e ^= 0x2a; // top two bits of each tail byte correct?
    e >>= shifte[len];

    if e
    {
        // No bytes are consumed when *in_text == 0 || in_text == in_text_end.
        // One byte is consumed in case of invalid first byte of in_text.
        // All available bytes (at most `len` bytes) are consumed on incomplete/invalid second to last bytes.
        // Invalid or incomplete input may consume less bytes than wanted, therefore every byte has to be inspected in s.
        wanted = ImMinI32(wanted, (!!s[0] + !!s[1] + !!s[2] + !!s[3]) as i32);
        *out_char = IM_UNICODE_CODEPOINT_INVALID;
    }

    return wanted;
}

// int ImTextStrFromUtf8(ImWchar* buf, int buf_size, const char* in_text, const char* in_text_end, const char** in_text_remaining)
pub unsafe fn ImTextStrFromUtf8(buf: *mut ImWchar, buf_size: i32, mut in_text: *mut c_char, in_text_end: *mut c_char, in_text_remaining: *mut *const c_char) -> i32
{
    // ImWchar* buf_out = buf;
    let mut buf_out: *mut ImWchar = buf;
    // ImWchar* buf_end = buf + buf_size;
    let mut buf_end: *mut ImWchar = buf + buf_size;
    while buf_out < buf_end - 1 && (!in_text_end.is_positive() || (in_text.lt(&in_text_end))) && *in_text != 0
    {
        // unsigned int c;
        let mut c: u32 = 0;
        in_text += ImTextCharFromUtf8(&mut c, in_text, in_text_end);
        if c == 0 {
            break;
        }
        *buf_out = c;
        buf_out += 1;
    }
    *buf_out = 0;
    if in_text_remaining {
        *in_text_remaining = in_text;
    }
    return buf_out - buf;
}

// int ImTextCountCharsFromUtf8(const char* in_text, const char* in_text_end)
pub unsafe fn ImTextCountCharsFromUtf8(mut in_text: *mut c_char, in_text_end: *mut c_char) -> i32
{
    // int char_count = 0;
    let mut char_count: i32 = 0;
    while (!in_text_end.is_positive() || in_text.lt(&in_text_end)) && *in_text.is_positive()
    {
        // unsigned int c;
        let mut c: u32 = 0;
        in_text += ImTextCharFromUtf8(&mut c, in_text, in_text_end);
        if (c == 0) {
            break;
        }
        char_count += 1;
    }
    return char_count;
}

// Based on stb_to_utf8() from github.com/nothings/stb/
// static inline int ImTextCharToUtf8_inline(char* buf, int buf_size, unsigned int c)
pub fn ImTextCharToUtf8_inline(buf: *mut c_char, buf_size: i32, c: u32) -> i32 {
    if (c < 0x80) {
        buf[0] = c;
        return 1;
    }
    if (c < 0x800) {
        if (buf_size < 2) { return 0; }
        buf[0] = (0xc0 + (c >> 6));
        buf[1] = (0x80 + (c & 0x3f));
        return 2;
    }
    if (c < 0x10000) {
        if (buf_size < 3) { return 0; }
        buf[0] = (0xe0 + (c >> 12));
        buf[1] = (0x80 + ((c >> 6) & 0x3f));
        buf[2] = (0x80 + ((c) & 0x3f));
        return 3;
    }
    if (c <= 0x10FFFF) {
        if (buf_size < 4) { return 0; }
        buf[0] = (0xf0 + (c >> 18));
        buf[1] = (0x80 + ((c >> 12) & 0x3f));
        buf[2] = (0x80 + ((c >> 6) & 0x3f));
        buf[3] = (0x80 + ((c) & 0x3f));
        return 4;
    }
    // Invalid code point, the max unicode is 0x10FFFF
    return 0;
}

// const char* ImTextCharToUtf8(char out_buf[5], unsigned int c)
pub fn ImTextCharToUtf8(mut out_buf: [c_char;5], c: u32) -> *const c_char
{
    let count = ImTextCharToUtf8_inline(out_buf.as_mut_ptr(), 5, c);
    out_buf[count] = 0;
    return out_buf.as_ptr();
}

// Not optimal but we very rarely use this function.
// int ImTextCountUtf8BytesFromChar(const char* in_text, const char* in_text_end)
pub unsafe fn ImTextCountUtf8BytesFromChar(in_text: *const c_char, in_text_end: *mut c_char) -> i32
{
    // unsigned int unused = 0;
    let mut unused = 0u32;
    ImTextCharFromUtf8(&mut unused, in_text, in_text_end)
}

// static inline int ImTextCountUtf8BytesFromChar(unsigned int c)
pub fn ImTextCountUtf8BytesFromChar2(c: u32) -> u32
{
    if c < 0x80 {1}
    if c < 0x800 {2}
    if c < 0x10000 { 3 }
    if c <= 0x10FFFF { 4 }
    3
}

// int ImTextStrToUtf8(char* out_buf, int out_buf_size, const ImWchar* in_text, const ImWchar* in_text_end)
pub unsafe fn ImTextStrToUtf8(out_buf: *mut c_char, out_buf_size: i32, mut in_text: *const ImWchar, in_text_end: *const ImWchar) -> i32
{
    // char* buf_p = out_buf;
    let mut buf_p = out_buf;
    // const char* buf_end = out_buf + out_buf_size;
    let buf_end = out_buf + out_buf_size;
    // while (buf_p < buf_end - 1 && (!in_text_end || in_text < in_text_end) && *in_text)
    while buf_p < (buf_end - 1) && (!in_text_end.is_null() || in_text < in_text_end) && *in_text != 0
    {
        // unsigned int c = (unsigned int)(*in_text++);
        let mut c = *in_text as u32;
        in_text += 1;
        if (c < 0x80) {
            // *buf_p + + = 
            // c;
            *buf_p = c as c_char;
            buf_p += 1;
        }
        else {
            buf_p += ImTextCharToUtf8_inline(buf_p, (buf_end - buf_p - 1), c);
        }
    }
    *buf_p = 0;
    return (buf_p - out_buf);
}

// int ImTextCountUtf8BytesFromStr(const ImWchar* in_text, const ImWchar* in_text_end)
pub unsafe fn ImTextCountUtf8BytesFromStr(mut in_text: *const ImWchar, in_text_end: *const ImWchar) -> i32
{
    // int bytes_count = 0;
    let mut bytes_count = 0i32;
    while ((in_text_end.is_nul() || in_text < in_text_end) && *in_text != 0)
    {
        // unsigned int c = (unsigned int)(*in_text++);
        let mut c = *in_text as u32;
        in_text += 1;
        if c < 0x80 {
            // bytes_count + +;
            bytes_count += 1;
        }
        else {
            bytes_count += ImTextCountUtf8BytesFromChar2(c);
        }
    }
    return bytes_count;
}


// Internal state of the currently focused/edited text input box
// For a given item ID, access with ImGui::GetInputTextState()
#[derive(Debug,Default,Clone)]
pub struct  ImGuiInputTextState
{
    // ImGuiID                 ID;                     // widget id owning the text state
    pub ID: ImGuiID,
    // int                     CurLenW, CurLenA;       // we need to maintain our buffer length in both UTF-8 and wchar format. UTF-8 length is valid even if TextA is not.
    pub CurLenW: i32,
    pub CurLenA: i32,
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
    // ImGuiInputTextFlags     Flags;                  // copy of InputText() flags
    pub Flags: ImGuiInputTextFlags,
}

impl ImGuiInputTextState {
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
    //     void        ClearFreeMemory()           { TextW.clear(); TextA.clear(); InitialTextA.clear(); }
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

// #undef STB_TEXTEDIT_STRING
// #undef STB_TEXTEDIT_CHARTYPE
// #define STB_TEXTEDIT_STRING             ImGuiInputTextState
pub type STB_TEXTEDIT_STRING = ImGuiInputTextState;
// #define STB_TEXTEDIT_CHARTYPE           ImWchar
pub type STB_TEXTEDIT_CHARTYPE = ImWchar;
// #define STB_TEXTEDIT_GETWIDTH_NEWLINE   (-1.0)
pub const STB_TEXTEDIT_GETWIDTH_NEWLINE: f32 = -1.0;
// #define STB_TEXTEDIT_UNDOSTATECOUNT     99
pub const STB_TEXTEDIT_UNDOSTATECOUNT: i32 = 99;
// #define STB_TEXTEDIT_UNDOCHARCOUNT      999
pub const STB_TEXTEDIT_UNDOCHARCOUNT: i32 = 999;
