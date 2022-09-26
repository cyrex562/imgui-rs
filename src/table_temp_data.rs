#![allow(non_snake_case)]

use libc::{c_float, c_int};
use crate::imgui_rect::ImRect;
use crate::imgui_vec2::ImVec2;

// Transient data that are only needed between BeginTable() and EndTable(), those buffers are shared (1 per level of stacked table).
// - Accessing those requires chasing an extra pointer so for very frequently used data we leave them in the main table structure.
// - We also leave out of this structure data that tend to be particularly useful for debugging/metrics.
#[derive(Debug,Default,Clone)]
pub struct  ImGuiTableTempData
{
pub TableIndex: c_int,                 // Index in g.Tables.Buf[] pool
pub LastTimeActive: c_float,             // Last timestamp this structure was used
pub UserOuterSize: ImVec2,              // outer_size.x passed to BeginTable()
pub DrawSplitter: ImDrawListSplitter,
pub HostBackupWorkRect: ImRect,         // Backup of Innerwindow.WorkRect at the end of BeginTable()
pub HostBackupParentWorkRect: ImRect,   // Backup of Innerwindow.ParentWorkRect at the end of BeginTable()
pub HostBackupPrevLineSize: ImVec2,     // Backup of Innerwindow.DC.PrevLineSize at the end of BeginTable()
pub HostBackupCurrLineSize: ImVec2,     // Backup of Innerwindow.DC.CurrLineSize at the end of BeginTable()
pub HostBackupCursorMaxPos: ImVec2,     // Backup of Innerwindow.DC.CursorMaxPos at the end of BeginTable()
pub HostBackupColumnsOffset: ImVec1,    // Backup of Outerwindow.DC.ColumnsOffset at the end of BeginTable()
pub HostBackupItemWidth: c_float,        // Backup of Outerwindow.DC.ItemWidth at the end of BeginTable()
pub HostBackupItemWidthStackSize: c_int,//Backup of Outerwindow.DC.ItemWidthStack.Size at the end of BeginTable()

    
}

impl ImGuiTableTempData {
    // ImGuiTableTempData()        { memset(this, 0, sizeof(*this)); LastTimeActive = -1f32; }
}
