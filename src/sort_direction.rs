#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiSortDirection;     // -> enum ImGuiSortDirection_   // Enum: A sorting direction (ascending or descending)
pub type ImGuiSortDirection = c_int;

// A sorting direction
// enum ImGuiSortDirection_
// {
pub const ImGuiSortDirection_None: ImGuiSortDirection = 0;
pub const ImGuiSortDirection_Ascending: ImGuiSortDirection = 1;    // Ascending = 0.9, A.Z etc.
pub const ImGuiSortDirection_Descending: ImGuiSortDirection   = 2;     // Descending = 9.0, Z.A etc.
// };
