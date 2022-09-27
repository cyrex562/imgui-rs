#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiTableRowFlags;     // -> enum ImGuiTableRowFlags_   // Flags: For TableNextRow()
pub type ImGuiTableRowFlags = c_int;


// Flags for ImGui::TableNextRow()
// enum ImGuiTableRowFlags_
// {
pub const ImGuiTableRowFlags_None: ImGuiTableRowFlags = 0;
pub const ImGuiTableRowFlags_Headers: ImGuiTableRowFlags = 1 << 0;   // Identify header row (set default background color + width of its contents accounted differently for auto column width)
// };