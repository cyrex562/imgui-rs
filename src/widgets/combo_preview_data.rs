#![allow(non_snake_case)]

use libc::c_float;
use crate::rect::ImRect;
use crate::layout::layout_type::ImGuiLayoutType;
use crate::core::vec2::ImVec2;

// Storage data for BeginComboPreview()/EndComboPreview()
#[derive(Default, Debug, Clone, Copy)]
pub struct ImGuiComboPreviewData {
    pub PreviewRect: ImRect,
    pub BackupCursorPos: ImVec2,
    pub BackupCursorMaxPos: ImVec2,
    pub BackupCursorPosPrevLine: ImVec2,
    pub BackupPrevLineTextBaseOffset: c_float,
    pub BackupLayout: ImGuiLayoutType,

    // ImGuiComboPreviewData() { memset(this, 0, sizeof(*this)); }
}
