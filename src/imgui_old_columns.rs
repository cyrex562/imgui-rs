#![allow(non_snake_case)]

use libc::{c_float, c_int};
use crate::imgui_rect::ImRect;
use crate::type_defs::{ImGuiID, ImGuiOldColumnFlags};

pub struct ImGuiOldColumns
{
pub ID: ImGuiID,
pub Flags: ImGuiOldColumnFlags,
pub IsFirstFrame: bool,
pub IsBeingResized: bool,
pub Current: c_int,
pub Count: c_int,
    // c_float               OffMinX, OffMaxX;       // Offsets from HostWorkRect.Min.x
// c_float               LineMinY, LineMaxY;
pub HostCursorPosY: c_float,         // Backup of CursorPos at the time of BeginColumns()
pub HostCursorMaxPosX: c_float,      // Backup of CursorMaxPos at the time of BeginColumns()
pub HostInitialClipRect: ImRect,    // Backup of ClipRect at the time of BeginColumns()
pub HostBackupClipRect: ImRect,     // Backup of ClipRect during PushColumnsBackground()/PopColumnsBackground()
pub HostBackupParentWorkRect: ImRect,//Backup of WorkRect at the time of BeginColumns()
pub Columns: Vec<ImGuiOldColumnData>,
pub Splitter: ImDrawListSplitter,

    // ImGuiOldColumns()   { memset(this, 0, sizeof(*this)); }
}
