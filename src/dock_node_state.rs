#![allow(non_upper_case_globals)]

use libc::c_int;

pub type ImGuiDockNodeState = c_int;

// enum ImGuiDockNodeState
// {
pub const ImGuiDockNodeState_Unknown: ImGuiDockNodeState = 0;
pub const ImGuiDockNodeState_HostWindowHiddenBecauseSingleWindow: ImGuiDockNodeState = 1;
pub const ImGuiDockNodeState_HostWindowHiddenBecauseWindowsAreResizing: ImGuiDockNodeState = 2;
pub const ImGuiDockNodeState_HostWindowVisible: ImGuiDockNodeState = 3;
// };
