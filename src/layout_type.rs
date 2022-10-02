#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiLayoutType;            // -> enum ImGuiLayoutType_         // Enum: Horizontal or vertical
pub type ImGuiLayoutType = c_int;

// FIXME: this is in development, not exposed/functional as a generic feature yet.
// Horizontal/Vertical enums are fixed to 0/1 so they may be used to index ImVec2
// enum ImGuiLayoutType_
// {
pub const ImGuiLayoutType_Horizontal: ImGuiLayoutType = 0;
pub const ImGuiLayoutType_Vertical: ImGuiLayoutType = 1;
// };
