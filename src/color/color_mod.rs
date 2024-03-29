#![allow(non_snake_case)]

use crate::color::ImGuiCol;
use crate::core::vec4::ImVec4;
use crate::core::type_defs::ImGuiCol;

// Stacked color modifier, backup of modified data so we can restore it
#[derive(Default, Debug, Clone, Copy)]
pub struct ImGuiColorMod {
    pub Col: ImGuiCol,
    pub BackupValue: ImVec4,
}
