#![allow(non_snake_case)]

use crate::type_defs::ImGuiTableColumnIdx;

// Transient cell data stored per row.
// sizeof() ~ 6
#[derive(Default, Debug, Clone)]
pub struct ImGuiTableCellData {
    pub BgColor: u32,
    // Actual color
    pub Column: ImGuiTableColumnIdx,     // Column number
}
