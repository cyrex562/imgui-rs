#![allow(non_snake_case)]

use libc::c_float;
use crate::rect::ImRect;
use crate::layout::layout_type::ImGuiLayoutType;
use crate::core::vec2::Vector2;

// Storage data for BeginComboPreview()/EndComboPreview()
#[derive(Default, Debug, Clone, Copy)]
pub struct ImGuiComboPreviewData {
    pub PreviewRect: ImRect,
    pub BackupCursorPos: Vector2,
    pub BackupCursorMaxPos: Vector2,
    pub BackupCursorPosPrevLine: Vector2,
    pub BackupPrevLineTextBaseOffset: c_float,
    pub BackupLayout: ImGuiLayoutType,

    // ImGuiComboPreviewData() { memset(this, 0, sizeof(*this)); }
}
