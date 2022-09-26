#![allow(non_snake_case)]

use libc::c_float;
use crate::vec2::ImVec2;
use crate::type_defs::ImGuiID;

// Stacked storage data for BeginGroup()/EndGroup()
#[derive(Default, Debug, Clone)]
pub struct ImGuiGroupData {
    pub WindowID: ImGuiID,
    pub BackupCursorPos: ImVec2,
    pub BackupCursorMaxPos: ImVec2,
    pub BackupIndent: ImVec1,
    pub BackupGroupOffset: ImVec1,
    pub BackupCurrLineSize: ImVec2,
    pub BackupCurrLineTextBaseOffset: c_float,
    pub BackupActiveIdIsAlive: ImGuiID,
    pub BackupActiveIdPreviousFrameIsAlive: bool,
    pub BackupHoveredIdIsAlive: bool,
    pub EmitItem: bool,
}
