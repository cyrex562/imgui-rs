use std::collections::HashSet;
use crate::types::Id32;
use crate::draw_list_splitter::DrawListSplitter;
use crate::rect::Rect;

#[derive(Debug,Default,Clone)]
pub struct DimgOldColumns
{
    // DimgId             id;
    pub ID: Id32,
    // ImGuiOldColumnFlags flags;
    pub Flags: DimgOldColumnFlags,
    // bool                IsFirstFrame;
    pub IsFirstFrame: bool,
    // bool                IsBeingResized;
    pub IsBeingResized: bool,
    // int                 Current;
    pub Current: i32,
    // int                 Count;
    pub Count: i32,
    // float               OffMinX, OffMaxX;       // Offsets from HostWorkRect.min.x
    pub OffMinX: f32,
    pub OffMaxX: f32,
    // float               LineMinY, LineMaxY;
    pub LineMinY: f32,
    pub LineMaxY: f32,
    // float               HostCursorPosY;         // Backup of CursorPos at the time of BeginColumns()
    pub HostCursorPosY: f32,
    // float               HostCursorMaxPosX;      // Backup of CursorMaxPos at the time of BeginColumns()
    pub HostCursorMaxPosX: f32,
    // DimgRect              HostInitialClipRect;    // Backup of clip_rect at the time of BeginColumns()
    pub HostInitialClipRect: Rect,
    // DimgRect              HostBackupClipRect;     // Backup of clip_rect during PushColumnsBackground()/PopColumnsBackground()
    pub HostBackupClipRect: Rect,
    // DimgRect              HostBackupParentWorkRect;//Backup of work_rect at the time of BeginColumns()
    pub HostBackupParentWorkRect: Rect,
    // ImVector<ImGuiOldColumnData> Columns;
    pub Columns: Vec<ImGuiOldColumnData>,
    // ImDrawListSplitter  Splitter;
    pub Splitter: DrawListSplitter,
    // ImGuiOldColumns()   { memset(this, 0, sizeof(*this)); }
}

impl DimgOldColumns {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}


// flags for internal's BeginColumns(). Prefix using BeginTable() nowadays!
#[derive(Debug,Clone)]
enum DimgOldColumnFlags
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

impl Default for DimgOldColumnFlags {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Default,Debug,Clone)]
pub struct ImGuiOldColumnData
{
    // float               offset_norm;         // column start offset, normalized 0.0 (far left) -> 1.0 (far right)
    pub offset_norm: f32,
    // float               offset_norm_before_resize;
    pub offset_norm_before_resize: f32,
    // ImGuiOldColumnFlags flags;              // Not exposed
    pub flags: HashSet<DimgOldColumnFlags>,
    // DimgRect              clip_rect;
    pub clip_rect: Rect,
    // ImGuiOldColumnData() { memset(this, 0, sizeof(*this)); }
}

impl ImGuiOldColumnData {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }

    }
}


