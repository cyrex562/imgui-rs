#![allow(non_snake_case)]

use crate::draw_list_splitter::ImDrawListSplitter;
use crate::old_column_data::ImGuiOldColumnData;
use crate::old_column_flags::ImGuiOldColumnFlags;
use crate::rect::ImRect;
use crate::type_defs::ImguiHandle;
use libc::{c_float, c_int};

#[derive(Default, Debug, Clone)]
pub struct ImGuiOldColumns {
    pub ID: ImguiHandle,
    pub Flags: ImGuiOldColumnFlags,
    pub IsFirstFrame: bool,
    pub IsBeingResized: bool,
    pub Current: c_int,
    pub Count: c_int,
    // c_float               OffMinX, OffMaxX;       // Offsets from HostWorkRect.Min.x
    pub OffMinX: c_float,
    pub OffMaxX: c_float,
    // c_float               LineMinY, LineMaxY;
    pub LineMinY: c_float,
    pub LineMaxY: c_float,
    pub HostCursorPosY: c_float,
    // Backup of CursorPos at the time of BeginColumns()
    pub HostCursorMaxPosX: c_float,
    // Backup of CursorMaxPos at the time of BeginColumns()
    pub HostInitialClipRect: ImRect,
    // Backup of ClipRect at the time of BeginColumns()
    pub HostBackupClipRect: ImRect,
    // Backup of ClipRect during PushColumnsBackground()/PopColumnsBackground()
    pub HostBackupParentWorkRect: ImRect,
    //Backup of WorkRect at the time of BeginColumns()
    pub Columns: Vec<ImGuiOldColumnData>,
    pub Splitter: ImDrawListSplitter,
    // ImGuiOldColumns()   { memset(this, 0, sizeof(*this)); }
}
