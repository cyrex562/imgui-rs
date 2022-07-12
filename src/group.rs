use crate::imgui_h::ImGuiID;
use crate::imgui_vec::{ImVec1, Vector2D};

// Stacked storage data for BeginGroup()/EndGroup()
#[derive(Default,Debug,Clone)]
pub struct GroupData
{
    // ImGuiID     WindowID;
    pub WindowID: ImGuiID,
    // Vector2D      BackupCursorPos;
    pub BackupCursorPos: Vector2D,
    // Vector2D      BackupCursorMaxPos;
    pub BackupCursorMaxPos: Vector2D,
    // ImVec1      BackupIndent;
    pub BackupIndent: ImVec1,
    // ImVec1      BackupGroupOffset;
    pub BackupGroupOffset: ImVec1,
    // Vector2D      BackupCurrLineSize;
    pub BackupCurrLineSize: Vector2D,
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
