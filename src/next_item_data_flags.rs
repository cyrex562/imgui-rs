#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiNextItemDataFlags;     // -> enum ImGuiNextItemDataFlags_  // Flags: for SetNextItemXXX() functions
pub type ImGuiNextItemDataFlags = c_int;

// enum ImGuiNextItemDataFlags_
// {
pub const ImGuiNextItemDataFlags_None: ImGuiNextItemDataFlags = 0;
pub const ImGuiNextItemDataFlags_HasWidth: ImGuiNextItemDataFlags = 1 << 0;
pub const ImGuiNextItemDataFlags_HasOpen: ImGuiNextItemDataFlags = 1 << 1;
// };