// sizeof() ~ 12

use libc::c_float;
use crate::sort_direction::{ImGuiSortDirection, ImGuiSortDirection_None};
use crate::type_defs::{ImGuiID, ImGuiTableColumnIdx};

#[derive(Default,Debug,Clone)]
pub struct ImGuiTableColumnSettings
{
pub WidthOrWeight:  c_float,
pub UserID:  ImGuiID,
pub Index:  ImGuiTableColumnIdx,
pub DisplayOrder:  ImGuiTableColumnIdx,
pub SortOrder:  ImGuiTableColumnIdx,
pub SortDirection:  ImGuiSortDirection,
pub IsEnabled:  bool, // "Visible" in ini file
pub IsStretch:  bool,

    
}

impl ImGuiTableColumnSettings {
    pub fn new() -> Self
    {
        Self {
            WidthOrWeight : 0f32,
            UserID : 0,
            Index : -1,
            DisplayOrder : -1,
            SortOrder : -1,
            SortDirection : ImGuiSortDirection_None,
            IsEnabled : true,
            IsStretch : false,
        }
    }
}
