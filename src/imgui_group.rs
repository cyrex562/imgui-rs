use crate::imgui_h::ImGuiID;
use crate::imgui_vec::{ImVec1, ImVec2};

// Stacked storage data for BeginGroup()/EndGroup()
#[derive(Default,Debug,Clone)]
pub struct  ImGuiGroupData
{
    // ImGuiID     WindowID;
    pub WindowID: ImGuiID,
    // ImVec2      BackupCursorPos;
    pub BackupCursorPos: ImVec2,
    // ImVec2      BackupCursorMaxPos;
    pub BackupCursorMaxPos: ImVec2,
    // ImVec1      BackupIndent;
    pub BackupIndent: ImVec1,
    // ImVec1      BackupGroupOffset;
    pub BackupGroupOffset: ImVec1,
    // ImVec2      BackupCurrLineSize;
    pub BackupCurrLineSize: ImVec2,
    // float       BackupCurrLineTextBaseOffset;
    pub BackupCurrLineTextBaseOffset: f32,
    // ImGuiID     BackupActiveIdIsAlive;
    pub BackupActiveIdIsAlive: ImGuiID,
    // bool        BackupActiveIdPreviousFrameIsAlive;
    pub BackupActiveIdPreviousFrameIsAlive: bool,
    // bool        BackupHoveredIdIsAlive;
    pub BackupHoveredIdIsAlive: bool,
    // bool        EmitItem;
    pub EmitItem: bool,
}