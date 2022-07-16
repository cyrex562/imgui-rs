use std::collections::HashSet;
use crate::layout::LayoutType;
use crate::rect::Rect;
use crate::vectors::two_d::Vector2D;

/// Storage data for BeginComboPreview()/EndComboPreview()
#[derive(Default,Debug,Clone)]
pub struct ComboPreviewData
{
    // ImRect          PreviewRect;
    pub preview_rect: Rect,
    // Vector2D          BackupCursorPos;
    pub backup_cursor_pos: Vector2D,
    // Vector2D          BackupCursorMaxPos;
    pub backup_cursor_max_pos: Vector2D,
    // Vector2D          BackupCursorPosPrevLine;
    pub backup_cursor_pos_prev_line: Vector2D,
    // float           BackupPrevLineTextBaseOffset;
    pub backup_prev_line_text_base_offset: f32,
    // ImGuiLayoutType BackupLayout;
    pub backup_layout: LayoutType,
    // ImGuiComboPreviewData() { memset(this, 0, sizeof(*this)); }
}

// flags for ImGui::BeginCombo()
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum ComboFlags
{
    None                    = 0,
    CustomPreview           = 1 << 20 ,  // enable BeginComboPreview()
    PopupAlignLeft          = 1 << 0,   // Align the popup toward the left by default
    HeightSmall             = 1 << 1,   // max ~4 items visible. Tip: If you want your combo popup to be a specific size you can use SetNextWindowSizeConstraints() prior to calling BeginCombo()
    HeightRegular           = 1 << 2,   // max ~8 items visible (default)
    HeightLarge             = 1 << 3,   // max ~20 items visible
    HeightLargest           = 1 << 4,   // As many fitting items as possible
    NoArrowButton           = 1 << 5,   // Display on the preview box without the square arrow button
    NoPreview               = 1 << 6,   // Display only a square arrow button
    // ImGuiComboFlags_HeightMask_             = ImGuiComboFlags_HeightSmall | ImGuiComboFlags_HeightRegular | ImGuiComboFlags_HeightLarge | ImGuiComboFlags_HeightLargest
}


// pub const HeightMask: i32             = DimgComboFlags::HeightSmall | DimgComboFlags::HeightRegular | DimgComboFlags::HeightLarge | DimgComboFlags::HeightLargest;
pub const HEIGHT_MASK: HashSet<ComboFlags> = HashSet::from([
    ComboFlags::HeightSmall, ComboFlags::HeightRegular, ComboFlags::HeightLarge, ComboFlags::HeightLargest
]);
