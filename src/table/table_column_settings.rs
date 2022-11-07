// sizeof() ~ 12

use crate::layout::sort_direction::{ImGuiSortDirection, ImGuiSortDirection_None};
use crate::core::type_defs::{ImGuiTableColumnIdx, ImguiHandle};
use libc::c_float;

#[derive(Default, Debug, Clone)]
pub struct ImGuiTableColumnSettings {
    pub WidthOrWeight: c_float,
    pub UserID: ImguiHandle,
    pub Index: ImGuiTableColumnIdx,
    pub DisplayOrder: ImGuiTableColumnIdx,
    pub SortOrder: ImGuiTableColumnIdx,
    pub SortDirection: ImGuiSortDirection,
    pub IsEnabled: bool, // "Visible" in ini file
    pub IsStretch: bool,
}

impl ImGuiTableColumnSettings {
    pub fn new() -> Self {
        Self {
            WidthOrWeight: 0.0,
            UserID: 0,
            Index: -1,
            DisplayOrder: -1,
            SortOrder: -1,
            SortDirection: ImGuiSortDirection_None,
            IsEnabled: true,
            IsStretch: false,
        }
    }
}
