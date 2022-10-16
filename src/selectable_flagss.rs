#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiSelectableFlags;   // -> enum ImGuiSelectableFlags_ // Flags: for Selectable()
pub type ImGuiSelectableFlags = c_int;
// 
// // Flags for Selectable()
// enum ImGuiSelectableFlags_
// {
    pub const ImGuiSelectableFlags_None: ImGuiSelectableFlags = 0;
    pub const ImGuiSelectableFlags_DontClosePopups: ImGuiSelectableFlags = 1 << 0;   // Clicking this don't close parent popup window
    pub const ImGuiSelectableFlags_SpanAllColumns: ImGuiSelectableFlags = 1 << 1;   // Selectable frame can span all columns (text will still fit in current column)
    pub const ImGuiSelectableFlags_AllowDoubleClick: ImGuiSelectableFlags = 1 << 2;   // Generate press events on double clicks too
    pub const ImGuiSelectableFlags_Disabled: ImGuiSelectableFlags = 1 << 3;   // Cannot be selected; display grayed out text
    pub const ImGuiSelectableFlags_AllowItemOverlap: ImGuiSelectableFlags = 1 << 4;   // (WIP) Hit testing to allow subsequent widgets to overlap this one
// };
