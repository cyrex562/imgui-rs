#![allow(non_snake_case)]

use crate::drawing::draw_list_splitter::ImDrawListSplitter;
use crate::core::imvec1::ImVec1;
use crate::rect::ImRect;
use crate::core::vec2::Vector2;
use libc::{c_float, c_int};

// Transient data that are only needed between BeginTable() and EndTable(), those buffers are shared (1 per level of stacked table).
// - Accessing those requires chasing an extra pointer so for very frequently used data we leave them in the main table structure.
// - We also leave out of this structure data that tend to be particularly useful for debugging/metrics.
#[derive(Debug, Default, Clone)]
pub struct ImGuiTableTempData {
    pub TableIndex: c_int,       // Index in g.Tables.Buf[] pool
    pub LastTimeActive: c_float, // Last timestamp this structure was used
    pub UserOuterSize: Vector2,   // outer_size.x passed to BeginTable()
    pub DrawSplitter: ImDrawListSplitter,
    pub HostBackupWorkRect: ImRect, // Backup of Innerwindow.work_rect at the end of BeginTable()
    pub HostBackupParentWorkRect: ImRect, // Backup of Innerwindow.ParentWorkRect at the end of BeginTable()
    pub HostBackupPrevLineSize: Vector2, // Backup of Innerwindow.DC.PrevLineSize at the end of BeginTable()
    pub HostBackupCurrLineSize: Vector2, // Backup of Innerwindow.DC.CurrLineSize at the end of BeginTable()
    pub HostBackupCursorMaxPos: Vector2, // Backup of Innerwindow.DC.CursorMaxPos at the end of BeginTable()
    pub HostBackupColumnsOffset: ImVec1, // Backup of Outerwindow.DC.ColumnsOffset at the end of BeginTable()
    pub HostBackupItemWidth: c_float, // Backup of Outerwindow.DC.ItemWidth at the end of BeginTable()
    pub HostBackupItemWidthStackSize: c_int, //Backup of Outerwindow.DC.ItemWidthStack.Size at the end of BeginTable()
}

impl ImGuiTableTempData {
    // ImGuiTableTempData()        { memset(this, 0, sizeof(*this)); LastTimeActive = -1.0; }
}
