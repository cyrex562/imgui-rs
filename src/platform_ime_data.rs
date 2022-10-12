#![allow(non_snake_case)]

use libc::c_float;
use crate::vec2::ImVec2;

// (Optional) Support for IME (Input Method Editor) via the io.SetPlatformImeDataFn() function.
#[derive(Default, Debug, Clone)]
pub struct ImGuiPlatformImeData {
    pub WantVisible: bool,
    // A widget wants the IME to be visible
    pub InputPos: ImVec2,
    // Position of the input cursor
    pub InputLineHeight: c_float,    // Line height

    // ImGuiPlatformImeData() { memset(this, 0, sizeof(*this)); }
}
