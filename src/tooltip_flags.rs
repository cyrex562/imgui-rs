#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiTooltipFlags;          // -> enum ImGuiTooltipFlags_       // Flags: for BeginTooltipEx()
pub type ImGuiTooltipFlags = c_int;

// enum ImGuiTooltipFlags_
// {
pub const ImGuiTooltipFlags_None: ImGuiTooltipFlags = 0;
pub const ImGuiTooltipFlags_OverridePreviousTooltip: ImGuiTooltipFlags = 1 << 0;   // Override will clear/ignore previously submitted tooltip (defaults to append)
// };
