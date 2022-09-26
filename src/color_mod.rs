#![allow(non_snake_case)]

use crate::vec4::ImVec4;
use crate::type_defs::ImGuiCol;

// Stacked color modifier, backup of modified data so we can restore it
#[derive(Default, Debug, Clone)]
pub struct ImGuiColorMod {
    pub Col: ImGuiCol,
    pub BackupValue: ImVec4,
}
