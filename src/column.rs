use std::collections::HashSet;
use crate::types::Id32;
use crate::draw::list_splitter::DrawListSplitter;
use crate::rect::Rect;

#[derive(Debug, Default, Clone)]
pub struct OldColumns {
    // DimgId             id;
    pub id: Id32,
    // ImGuiOldColumnFlags flags;
    pub flags: HashSet<OldColumnFlags>,
    // bool                is_first_frame;
    pub is_first_frame: bool,
    // bool                is_being_resized;
    pub is_being_resized: bool,
    // int                 current;
    pub current: i32,
    // int                 count;
    pub count: i32,
    // float               off_min_x, off_max_x;       // Offsets from HostWorkRect.min.x
    pub off_min_x: f32,
    pub off_max_x: f32,
    // float               line_min_y, line_max_y;
    pub line_min_y: f32,
    pub line_max_y: f32,
    // float               host_cursor_pos_y;         // Backup of CursorPos at the time of BeginColumns()
    pub host_cursor_pos_y: f32,
    // float               host_cursor_max_pos_x;      // Backup of CursorMaxPos at the time of BeginColumns()
    pub host_cursor_max_pos_x: f32,
    // DimgRect              host_initial_clip_rect;    // Backup of clip_rect at the time of BeginColumns()
    pub host_initial_clip_rect: Rect,
    // DimgRect              host_backup_clip_rect;     // Backup of clip_rect during PushColumnsBackground()/PopColumnsBackground()
    pub host_backup_clip_rect: Rect,
    // DimgRect              host_backup_parent_work_rect;//Backup of work_rect at the time of BeginColumns()
    pub host_backup_parent_work_rect: Rect,
    // ImVector<ImGuiOldColumnData> columns;
    pub columns: Vec<OldColumnData>,
    // ImDrawListSplitter  splitter;
    pub splitter: DrawListSplitter,
    // ImGuiOldColumns()   { memset(this, 0, sizeof(*this)); }
}

// flags for internal's BeginColumns(). Prefix using BeginTable() nowadays!
#[derive(Debug, Clone)]
pub enum OldColumnFlags {
    None,
    NoBorder,
    // Disable column dividers
    NoResize,
    // Disable resizing columns when clicking on the dividers
    NoPreserveWidths,
    // Disable column width preservation when adjusting columns
    NoForceWithinWindow,
    // Disable forcing columns to fit within window
    GrowParentContentsSize,      // (WIP) Restore pre-1.51 behavior of extending the parent window contents size but _without affecting the columns width at all_. Will eventually remove.
}

impl Default for OldColumnFlags {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Default, Debug, Clone)]
pub struct OldColumnData {
    // float               offset_norm;         // column start offset, normalized 0.0 (far left) -> 1.0 (far right)
    pub offset_norm: f32,
    // float               offset_norm_before_resize;
    pub offset_norm_before_resize: f32,
    // ImGuiOldColumnFlags flags;              // Not exposed
    pub flags: HashSet<OldColumnFlags>,
    // DimgRect              clip_rect;
    pub clip_rect: Rect,
    // ImGuiOldColumnData() { memset(this, 0, sizeof(*this)); }
}



