#![allow(non_upper_case_globals)]

use libc::c_int;

pub type ImGuiLogType = c_int;

// enum ImGuiLogType
// {
pub const ImGuiLogType_None: ImGuiLogType = 0;
pub const ImGuiLogType_TTY: ImGuiLogType = 1;
pub const ImGuiLogType_File: ImGuiLogType = 2;
pub const ImGuiLogType_Buffer: ImGuiLogType = 3;
pub const ImGuiLogType_Clipboard: ImGuiLogType = 4;
// };
