#![allow(non_snake_case)]

use crate::layout::sort_direction::ImGuiSortDirection;
use crate::core::type_defs::ImguiHandle;

// Sorting specification for one column of a table (sizeof == 12 bytes)
#[derive(Default, Debug, Clone)]
pub struct ImGuiTableColumnSortSpecs {
    pub ColumnUserID: ImguiHandle, // User id of the column (if specified by a TableSetupColumn() call)
    pub ColumnIndex: i16,          // Index of the column
    pub SortOrder: i16, // Index within parent ImGuiTableSortSpecs (always stored in order starting from 0, tables sorted on a single criteria will always have a 0 here)
    pub SortDirection: ImGuiSortDirection, // ImGuiSortDirection_Ascending or ImGuiSortDirection_Descending (you can use this or SortSign, whichever is more convenient for your sort function)

                                           // ImGuiTableColumnSortSpecs() { memset(this, 0, sizeof(*this)); }
}
