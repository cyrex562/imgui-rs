#![allow(non_snake_case)]

use crate::imvec1::ImVec1;
use crate::type_defs::ImguiHandle;
use crate::vec2::ImVec2;
use libc::c_float;

// Stacked storage data for BeginGroup()/EndGroup()
#[derive(Default, Debug, Clone, Copy)]
pub struct ImGuiGroupData {
    pub WindowID: ImguiHandle,
    pub BackupCursorPos: ImVec2,
    pub BackupCursorMaxPos: ImVec2,
    pub BackupIndent: ImVec1,
    pub BackupGroupOffset: ImVec1,
    pub BackupCurrLineSize: ImVec2,
    pub BackupCurrLineTextBaseOffset: c_float,
    pub BackupActiveIdIsAlive: ImguiHandle,
    pub BackupActiveIdPreviousFrameIsAlive: bool,
    pub BackupHoveredIdIsAlive: bool,
    pub EmitItem: bool,
}
