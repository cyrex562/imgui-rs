#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiSeparatorFlags;        // -> enum ImGuiSeparatorFlags_     // Flags: for SeparatorEx()
pub type ImGuiSeparatorFlags = c_int;

// enum ImGuiSeparatorFlags_
// {
    pub const ImGuiSeparatorFlags_None: ImGuiSeparatorFlags =  0;
    pub const ImGuiSeparatorFlags_Horizontal: ImGuiSeparatorFlags =  1 << 0;   // Axis default to current layout type, so generally Horizontal unless e.g. in a menu bar
    pub const ImGuiSeparatorFlags_Vertical: ImGuiSeparatorFlags =  1 << 1;
    pub const ImGuiSeparatorFlags_SpanAllColumns: ImGuiSeparatorFlags =  1 << 2;
// };
