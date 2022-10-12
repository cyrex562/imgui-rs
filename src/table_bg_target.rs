#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiTableBgTarget;     // -> enum ImGuiTableBgTarget_   // Enum: A color target for TableSetBgColor()
pub type ImGuiTableBgTarget = c_int;

// Enum for TableSetBgColor()
// Background colors are rendering in 3 layers:
//  - Layer 0: draw with RowBg0 color if set, otherwise draw with ColumnBg0 if set.
//  - Layer 1: draw with RowBg1 color if set, otherwise draw with ColumnBg1 if set.
//  - Layer 2: draw with CellBg color if set.
// The purpose of the two row/columns layers is to let you decide if a background color changes should override or blend with the existing color.
// When using ImGuiTableFlags_RowBg on the table, each row has the RowBg0 color automatically set for odd/even rows.
// If you set the color of RowBg0 target, your color will override the existing RowBg0 color.
// If you set the color of RowBg1 or ColumnBg1 target, your color will blend over the RowBg0 color.
// enum ImGuiTableBgTarget_
// {
    pub const ImGuiTableBgTarget_None: ImGuiTableBgTarget = 0;
    pub const ImGuiTableBgTarget_RowBg0: ImGuiTableBgTarget = 1;        // Set row background color 0 (generally used for background, automatically set when ImGuiTableFlags_RowBg is used)
    pub const ImGuiTableBgTarget_RowBg1: ImGuiTableBgTarget = 2;        // Set row background color 1 (generally used for selection marking)
    pub const ImGuiTableBgTarget_CellBg: ImGuiTableBgTarget = 3;        // Set cell background color (top-most color)
// };

