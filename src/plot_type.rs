#![allow(non_upper_case_globals)]

use libc::c_int;

pub type ImGuiPlotType = c_int;

// enum ImGuiPlotType
// {
pub const ImGuiPlotType_Lines: ImGuiPlotType = 0;
pub const ImGuiPlotType_Histogram: ImGuiPlotType = 1;
// };
