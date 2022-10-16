#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiTableBgTarget;     // -> enum ImGuiTableBgTarget_   // Enum: A color target for TableSetBgColor()
pub type ImGuiTableBgTarget = c_int;

// enum ImGuiTableBgTarget_
// {
    pub const ImGuiTableBgTarget_None: ImGuiTableBgTarget = 0;
    pub const ImGuiTableBgTarget_RowBg0: ImGuiTableBgTarget = 1;        // Set row background color 0 (generally used for background; automatically set when ImGuiTableFlags_RowBg is used)
    pub const ImGuiTableBgTarget_RowBg1: ImGuiTableBgTarget = 2;        // Set row background color 1 (generally used for selection marking)
    pub const ImGuiTableBgTarget_CellBg: ImGuiTableBgTarget = 3;        // Set cell background color (top-most color)
// };
