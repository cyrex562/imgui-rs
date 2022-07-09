use std::collections::HashSet;
use crate::rect::DimgRect;
use crate::vec_nd::DimgVec2D;

/// Storage data for BeginComboPreview()/EndComboPreview()
#[derve(Default,Debug,Clone)]
pub struct DimgComboPreviewData
{
    // ImRect          PreviewRect;
    pub preview_rect: DimgRect,
    // ImVec2          BackupCursorPos;
    pub backup_cursor_pos: DimgVec2D,
    // ImVec2          BackupCursorMaxPos;
    pub backup_cursor_max_pos: DimgVec2D,
    // ImVec2          BackupCursorPosPrevLine;
    pub backup_cursor_pos_prev_line: DimgVec2D,
    // float           BackupPrevLineTextBaseOffset;
    pub backup_prev_line_text_base_offset: f32,
    // ImGuiLayoutType BackupLayout;
    pub backup_layout: DimgLayoutType,
    // ImGuiComboPreviewData() { memset(this, 0, sizeof(*this)); }
}

// Extend
pub enum DimgComboFlags
{
    CustomPreview           = 1 << 20   // enable BeginComboPreview()
}


// flags for ImGui::BeginCombo()
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgComboFlags
{
    None                    = 0,
    PopupAlignLeft          = 1 << 0,   // Align the popup toward the left by default
    HeightSmall             = 1 << 1,   // Max ~4 items visible. Tip: If you want your combo popup to be a specific size you can use SetNextWindowSizeConstraints() prior to calling BeginCombo()
    HeightRegular           = 1 << 2,   // Max ~8 items visible (default)
    HeightLarge             = 1 << 3,   // Max ~20 items visible
    HeightLargest           = 1 << 4,   // As many fitting items as possible
    NoArrowButton           = 1 << 5,   // Display on the preview box without the square arrow button
    NoPreview               = 1 << 6,   // Display only a square arrow button
    // ImGuiComboFlags_HeightMask_             = ImGuiComboFlags_HeightSmall | ImGuiComboFlags_HeightRegular | ImGuiComboFlags_HeightLarge | ImGuiComboFlags_HeightLargest
}


// pub const HeightMask: i32             = DimgComboFlags::HeightSmall | DimgComboFlags::HeightRegular | DimgComboFlags::HeightLarge | DimgComboFlags::HeightLargest;
pub const HEIGHT_MASK: HashSet<DimgComboFlags> = HashSet::from([
    DimgComboFlags::HeightSmall, DimgComboFlags::HeightRegular, DimgComboFlags::HeightLarge, DimgComboFlags::HeightLargest
]);
