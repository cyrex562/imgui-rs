#![allow(non_snake_case)]

use libc::{c_float, c_int};
use crate::core::vec2::Vector2;
use crate::style_var::ImGuiStyleVar;

// Stacked style modifier, backup of modified data so we can restore it. Data type inferred from the variable.
#[derive(Default, Debug, Clone)]
pub struct ImGuiStyleMod {
    pub VarIdx: ImGuiStyleVar,
    pub BackupInt: [c_int; 2],
    pub BackupFloat: [c_float; 2],
}

impl ImGuiStyleMod {
    // ImGuiStyleMod(ImGuiStyleVar idx, v: c_int)     { VarIdx = idx; BackupInt[0] = v; }
    pub fn new(idx: ImGuiStyleVar, v: c_int) -> Self {
        Self {
            VarIdx: idx,
            BackupInt: [v, 0],
            BackupFloat: [0.0; 2],
        }
    }

    // ImGuiStyleMod(ImGuiStyleVar idx, c_float v)   { VarIdx = idx; BackupFloat[0] = v; }
    pub fn new2(idx: ImGuiStyleVar, v: c_float) -> Self {
        Self {
            VarIdx: idx,
            BackupFloat: [v, 0.0],
            ..Default::default()
        }
    }


    // ImGuiStyleMod(ImGuiStyleVar idx, v: ImVec2)  { VarIdx = idx; BackupFloat[0] = v.x; BackupFloat[1] = v.y; }
    pub fn new3(idx: ImGuiStyleVar, v: Vector2) -> Self {
        Self {
            VarIdx: idx,
            BackupFloat: [v.x, 0.0],
            ..Default::default()
        }
    }
}
