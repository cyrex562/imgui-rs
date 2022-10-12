#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiMouseButton;       // -> enum ImGuiMouseButton_     // Enum: A mouse button identifier (0=left, 1=right, 2=middle)
pub type ImGuiMouseButton = c_int;

// Identify a mouse button.
// Those values are guaranteed to be stable and we frequently use 0/1 directly. Named enums provided for convenience.
// enum ImGuiMouseButton_
// {
pub const ImGuiMouseButton_Left: ImGuiMouseButton = 0;
pub const ImGuiMouseButton_Right: ImGuiMouseButton = 1;
pub const ImGuiMouseButton_Middle: ImGuiMouseButton = 2;
pub const ImGuiMouseButton_COUNT: ImGuiMouseButton = 5;
// };
