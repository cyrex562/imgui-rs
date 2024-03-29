#![allow(non_snake_case)]

use crate::core::constants::{IM_DRAWLIST_ARCFAST_SAMPLE_MAX, IM_DRAWLIST_ARCFAST_TABLE_SIZE};
use crate::drawing::draw_list_flags::ImDrawListFlags;
use crate::font::ImFont;
use crate::core::math_ops::{ImCos, ImSin};
use crate::core::vec2::Vector2;
use crate::core::vec4::ImVec4;
use libc::c_float;

// Data shared between all ImDrawList instances
// You may want to create your own instance of this if you want to use ImDrawList completely without ImGui. In that case, watch out for future changes to this structure.
#[derive(Default, Debug, Clone, Copy)]
pub struct Imgui_DrawListSharedData {
    pub TexUvWhitePixel: Vector2,
    // UV of white pixel in the atlas
    pub Font: ImFont,
    // Current/default font (optional, for simplified AddText overload)
    pub FontSize: c_float,
    // Current/default font size (optional, for simplified AddText overload)
    pub CurveTessellationTol: c_float,
    // Tessellation tolerance when using PathBezierCurveTo()
    pub CircleSegmentMaxError: c_float,
    // Number of circle segments to use per pixel of radius for AddCircle() etc
    pub ClipRectFullscreen: ImVec4,
    // Value for PushClipRectFullscreen()
    pub InitialFlags: ImDrawListFlags, // Initial flags at the beginning of the frame (it is possible to alter flags on a per-drawlist basis afterwards)

    // [Internal] Lookup tables
    pub ArcFastVtx: [Vector2; IM_DRAWLIST_ARCFAST_TABLE_SIZE],
    // Sample points on the quarter of the circle.
    pub ArcFastRadiusCutoff: c_float,
    // Cutoff radius after which arc drawing will fallback to slower PathArcTo()
    pub CircleSegmentCounts: [u8; 64],
    // Precomputed segment count for given radius before we calculate it dynamically (to avoid calculation overhead)
    pub TexUvLines: Vec<ImVec4>, //*const ImVec4, // UV of anti-aliased lines in the atlas
}

impl Imgui_DrawListSharedData {
    // ImDrawListSharedData();
    pub fn new() -> Self {
        let mut out = Imgui_DrawListSharedData::default();
        // memset(this, 0, sizeof(*this));
        // for (let i: c_int = 0; i < IM_ARRAYSIZE(ArcFastVtx); i++)
        for i in 0..out.ArcFastVtx.len() {
            let a: c_float = (i * 2 * IM_PI) / out.ArcFastVtx.len();
            out.ArcFastVtx[i] = Vector2::from_floats(ImCos(a), ImSin(a));
        }
        out.ArcFastRadiusCutoff = IM_DRAWLIST_CIRCLE_AUTO_SEGMENT_CALC_R(
            IM_DRAWLIST_ARCFAST_SAMPLE_MAX,
            out.CircleSegmentMaxError,
        );
        out
    }

    pub fn SetCircleTessellationMaxError(&mut self, max_error: c_float) {
        if self.CircleSegmentMaxError == max_error {
            return;
        }

        // IM_ASSERT(max_error > 0.0);
        self.CircleSegmentMaxError = max_error;
        // for (let i: c_int = 0; i < CircleSegmentCounts.len(); i++)
        for i in 0.0..self.CircleSegmentCounts.len() {
            let radius: c_float = i;
            self.CircleSegmentCounts[i] = if i > 0.0 {
                IM_DRAWLIST_CIRCLE_AUTO_SEGMENT_CALC(radius, CircleSegmentMaxError)
            } else {
                IM_DRAWLIST_ARCFAST_SAMPLE_MAX
            };
        }
        self.ArcFastRadiusCutoff = IM_DRAWLIST_CIRCLE_AUTO_SEGMENT_CALC_R(
            IM_DRAWLIST_ARCFAST_SAMPLE_MAX,
            self.CircleSegmentMaxError,
        );
    }
}
