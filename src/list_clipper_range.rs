//-----------------------------------------------------------------------------
#![allow(non_snake_case)]
// [SECTION] Clipper support
//-----------------------------------------------------------------------------

use libc::{c_float, c_int};

#[derive(Default, Debug, Clone)]
pub struct ImGuiListClipperRange {
    pub Min: c_float,
    pub Max: c_float,
    pub PosToIndexConvert: bool,
    // Begin/End are absolute position (will be converted to indices later)
    pub PosToIndexOffsetMin: c_int,
    // Add to Min after converting to indices
    pub PosToIndexOffsetMax: c_int,    // Add to Min after converting to indices
}

impl ImGuiListClipperRange {
    // static ImGuiListClipperRange    FromIndices(c_int min, c_int max)                               { ImGuiListClipperRange r = { min, max, false, 0, 0 }; return r; }
    pub fn FromIndices(min: c_int, max: c_int) -> Self {
        Self {
            Min: min as c_float,
            Max: max as c_float,
            PosToIndexConvert: false,
            PosToIndexOffsetMin: 0,
            PosToIndexOffsetMax: 0
        }
    }

    // static ImGuiListClipperRange    FromPositions(c_float y1, c_float y2, c_int off_min, c_int off_max) { ImGuiListClipperRange r = { y1, y2, true, off_min, off_max }; return r; }
    pub fn FromPositions(y1: c_float, y2: c_float, off_min: c_int, off_max: c_int) -> Self {
        Self {
            Min: y1,
            Max: y2,
            PosToIndexConvert: true,
            PosToIndexOffsetMin: off_min,
            PosToIndexOffsetMax: off_max
        }
    }

}
