#![allow(non_snake_case)]

use crate::string_ops::str_to_const_c_char_ptr;
use crate::GImGui;
use libc::c_char;

// GetClipboardText: *const c_char()
pub unsafe fn GetClipboardText() -> String {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return if g.IO.GetClipboardTextFn {
        g.IO.GetClipboardTextFn(g.IO.ClipboardUserData)
    } else {
        String::from("")
    };
}

// c_void SetClipboardText(text: *const c_char)
pub unsafe fn SetClipboardText(text: Stringing) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if g.IO.SetClipboardTextFn {
        g.IO.SetClipboardTextFn(g.IO.ClipboardUserData, text);
    }
}
