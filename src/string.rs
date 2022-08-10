use std::os::raw::c_char;
use crate::imgui_h::InputTextFlags;
use crate::imgui_text_input_state::InputTextState;
use crate::text_input_state::InputTextState;

// static inline bool      ImCharIsBlankW(unsigned int c)
pub fn char_is_blank_w(c: u32) -> bool {
    return c == u32::from(' ') || c == u32::from('\t') || c == 0x3000;
}

pub fn is_separator(c: u32) -> bool {
    return char_is_blank_w(c) || c == u32::from(',') || c == u32::from(';') || c == u32::from('(') || c == u32::from(')') || c == u32::from('{') || c == u32::from('}') || c == u32::from('[') || c == u32::from(']') || c == u32::from('|') || c == u32::from('\n') || c == u32::from('\r');
}

// static int  is_word_boundary_from_right(ImGuiInputTextState* obj, int idx)
pub fn is_word_boundary_from_right(obj: &mut InputTextState, idx: usize) -> bool {
    if obj.flags.contains(&InputTextFlags::Password) { return false; }
    return if idx > 0 { (is_separator(obj.text_w[idx - 1]) && !is_separator(obj.text_w[idx])) } else { 1 };
}


// // Find end-of-line. Return pointer will point to either first \n, either str_end.
// const char* ImStreolRange(const char* str, const char* str_end)
// {
//     const char* p = (const char*)memchr(str, '\n', str_end - str);
//     return p ? p : str_end;
// }
pub fn str_eol_range(str_begin: &str, str_end: &str) -> &str {
    let test_str = String::from(str_begin);
    let eol_idx = test_str.find('\n');
    if eol_idx.is_some() {
        str_begin + eol_idx.some()
    }
    str_end
}

