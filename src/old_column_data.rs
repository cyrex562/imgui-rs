#![allow(non_snake_case)]

use libc::c_float;
use crate::old_column_flags::ImGuiOldColumnFlags;
use crate::rect::ImRect;

pub struct ImGuiOldColumnData {
    pub OffsetNorm: c_float,
    // Column start offset, normalized 0.0 (far left) -> 1.0 (far right)
    pub OffsetNormBeforeResize: c_float,
    pub Flags: ImGuiOldColumnFlags,
    // Not exposed
    pub ClipRect: ImRect,

    // ImGuiOldColumnData() { memset(this, 0, sizeof(*this)); }
}
