#![allow(non_snake_case)]

use libc::{c_float, c_int};
use crate::imgui_vec2::ImVec2;
use crate::type_defs::ImGuiStyleVar;

// Stacked style modifier, backup of modified data so we can restore it. Data type inferred from the variable.
#[derive(Default, Debug, Clone)]
pub struct ImGuiStyleMod {
    pub VarIdx: ImGuiStyleVar,
    pub BackupInt: [c_int; 2],
    pub BackupFloat: [c_float; 2],
}

impl ImGuiStyleMod {
    // ImGuiStyleMod(ImGuiStyleVar idx, c_int v)     { VarIdx = idx; BackupInt[0] = v; }
    pub fn new(idx: ImGuiStyleVar, v: c_int) -> Self {
        Self {
            VarIdx: idx,
            BackupInt: [v, 0],
            BackupFloat: [0f32; 2],
        }
    }

    // ImGuiStyleMod(ImGuiStyleVar idx, c_float v)   { VarIdx = idx; BackupFloat[0] = v; }
    pub fn new2(idx: ImGuiStyleVar, v: c_float) -> Self {
        Self {
            VarIdx: idx,
            BackupFloat: [v, 0f32],
            ..Default::default()
        }
    }


    // ImGuiStyleMod(ImGuiStyleVar idx, ImVec2 v)  { VarIdx = idx; BackupFloat[0] = v.x; BackupFloat[1] = v.y; }
    pub fn new3(idx: ImGuiStyleVar, v: ImVec2) -> Self {
        Self {
            VarIdx: idx,
            BackupFloat: [v.x, 0f32],
            ..Default::default()
        }
    }
}