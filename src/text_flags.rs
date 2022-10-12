#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiTextFlags;             // -> enum ImGuiTextFlags_          // Flags: for TextEx()
pub type ImGuiTextFlags = c_int;

// enum ImGuiTextFlags_
// {
pub const    ImGuiTextFlags_None: ImGuiTextFlags                         = 0;
    pub const ImGuiTextFlags_NoWidthForLargeClippedText: ImGuiTextFlags   = 1 << 0;
// };
