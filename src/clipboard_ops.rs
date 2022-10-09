#![allow(non_snake_case)]

use crate::string_ops::str_to_const_c_char_ptr;
use crate::GImGui;
use libc::c_char;

// *const char GetClipboardText()
pub unsafe fn GetClipboardText() -> *const c_char {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return if g.IO.GetClipboardTextFn {
        g.IO.GetClipboardTextFn(g.IO.ClipboardUserData)
    } else {
        str_to_const_c_char_ptr("")
    };
}

// c_void SetClipboardText(*const char text)
pub unsafe fn SetClipboardText(text: *const c_char) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if g.IO.SetClipboardTextFn {
        g.IO.SetClipboardTextFn(g.IO.ClipboardUserData, text);
    }
}
