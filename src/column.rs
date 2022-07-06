use crate::imgui_h::{ImDrawListSplitter, ImGuiID};
use crate::imgui_rect::ImRect;

pub struct ImGuiOldColumns
{
    // ImGuiID             ID;
    pub ID: ImGuiID,
    // ImGuiOldColumnFlags Flags;
    pub Flags: ImGuiOldColumnFlags,
    // bool                IsFirstFrame;
    pub IsFirstFrame: bool,
    // bool                IsBeingResized;
    pub IsBeingResized: bool,
    // int                 Current;
    pub Current: i32,
    // int                 Count;
    pub Count: i32,
    // float               OffMinX, OffMaxX;       // Offsets from HostWorkRect.Min.x
    pub OffMinX: f32,
    pub OffMaxX: f32,
    // float               LineMinY, LineMaxY;
    pub LineMinY: f32,
    pub LineMaxY: f32,
    // float               HostCursorPosY;         // Backup of CursorPos at the time of BeginColumns()
    pub HostCursorPosY: f32,
    // float               HostCursorMaxPosX;      // Backup of CursorMaxPos at the time of BeginColumns()
    pub HostCursorMaxPosX: f32,
    // ImRect              HostInitialClipRect;    // Backup of ClipRect at the time of BeginColumns()
    pub HostInitialClipRect: ImRect,
    // ImRect              HostBackupClipRect;     // Backup of ClipRect during PushColumnsBackground()/PopColumnsBackground()
    pub HostBackupClipRect: ImRect,
    // ImRect              HostBackupParentWorkRect;//Backup of WorkRect at the time of BeginColumns()
    pub HostBackupParentWorkRect: ImRect,
    // ImVector<ImGuiOldColumnData> Columns;
    pub Columns: Vec<ImGuiOldColumnData>,
    // ImDrawListSplitter  Splitter;
    pub Splitter: ImDrawListSplitter,
    // ImGuiOldColumns()   { memset(this, 0, sizeof(*this)); }
}

impl ImGuiOldColumns {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}


// Flags for internal's BeginColumns(). Prefix using BeginTable() nowadays!
enum ImGuiOldColumnFlags
{
    None                    = 0,
    NoBorder                = 1 << 0,   // Disable column dividers
    NoResize                = 1 << 1,   // Disable resizing columns when clicking on the dividers
    NoPreserveWidths        = 1 << 2,   // Disable column width preservation when adjusting columns
    NoForceWithinWindow     = 1 << 3,   // Disable forcing columns to fit within window
    GrowParentContentsSize  = 1 << 4    // (WIP) Restore pre-1.51 behavior of extending the parent window contents size but _without affecting the columns width at all_. Will eventually remove.

    // Obsolete names (will be removed)
// #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
//     , ImGuiColumnsFlags_None                    = None,
//     ImGuiColumnsFlags_NoBorder                  = NoBorder,
//     ImGuiColumnsFlags_NoResize                  = NoResize,
//     ImGuiColumnsFlags_NoPreserveWidths          = NoPreserveWidths,
//     ImGuiColumnsFlags_NoForceWithinWindow       = NoForceWithinWindow,
//     ImGuiColumnsFlags_GrowParentContentsSize    = GrowParentContentsSize
// #endif
}

#[derive(Default,Debug,Clone)]
pub struct ImGuiOldColumnData
{
    // float               OffsetNorm;         // Column start offset, normalized 0.0 (far left) -> 1.0 (far right)
    pub OffsetNorm: f32,
    // float               OffsetNormBeforeResize;
    pub OffsetNormBeforeResize: f32,
    // ImGuiOldColumnFlags Flags;              // Not exposed
    pub Flags: ImGuiOldColumnFlags,
    // ImRect              ClipRect;
    pub ClipRect: ImRect,
    // ImGuiOldColumnData() { memset(this, 0, sizeof(*this)); }
}

impl ImGuiOldColumnData {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }

    }
}


