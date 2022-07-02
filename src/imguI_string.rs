use crate::imgui_h::ImGuiInputTextFlags;
use crate::imgui_text_input_state::ImGuiInputTextState;

// static inline bool      ImCharIsBlankW(unsigned int c)
pub fn ImCharIsBlankW(c: u32) -> bool { return c == u32::from(' ') || c == u32::from('\t') || c == 0x3000; }

pub fn is_separator(c: u32) -> bool {
    return ImCharIsBlankW(c) || c == u32::from(',') || c == u32::from(';') || c == u32::from('(') || c == u32::from(')') || c == u32::from('{') || c == u32::from('}') || c == u32::from('[') || c == u32::from(']') || c == u32::from('|') || c == u32::from('\n') || c == u32::from('\r');
}

// static int  is_word_boundary_from_right(ImGuiInputTextState* obj, int idx)
pub fn is_word_boundary_from_right(obj: *mut ImGuiInputTextState, idx: usize) -> bool {
    if &obj.Flags & ImGuiInputTextFlags::Password { return false; } else {};
    return if idx > 0 { (is_separator(obj.TextW[idx - 1]) && !is_separator(obj.TextW[idx])) } else { 1 };
}
