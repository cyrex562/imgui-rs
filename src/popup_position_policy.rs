#![allow(non_upper_case_globals)]

use libc::c_int;

pub type ImGuiPopupPositionPolicy = c_int;
// enum ImGuiPopupPositionPolicy

// {
pub const ImGuiPopupPositionPolicy_Default: ImGuiPopupPositionPolicy = 0;
pub const ImGuiPopupPositionPolicy_ComboBox: ImGuiPopupPositionPolicy = 1;
pub const ImGuiPopupPositionPolicy_Tooltip: ImGuiPopupPositionPolicy = 2;
// };
