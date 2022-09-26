#![allow(non_snake_case)]

use libc::c_float;
use crate::font::ImFont;
use crate::imgui_vec2::ImVec2;
use crate::imgui_vec4::ImVec4;
use crate::type_defs::ImDrawListFlags;

// Data shared between all ImDrawList instances
// You may want to create your own instance of this if you want to use ImDrawList completely without ImGui. In that case, watch out for future changes to this structure.
#[derive(Default, Debug, Clone)]
pub struct ImDrawListSharedData {
    pub TexUvWhitePixel: ImVec2,
    // UV of white pixel in the atlas
    pub Font: *mut ImFont,
    // Current/default font (optional, for simplified AddText overload)
    pub FontSize: c_float,
    // Current/default font size (optional, for simplified AddText overload)
    pub CurveTessellationTol: c_float,
    // Tessellation tolerance when using PathBezierCurveTo()
    pub CircleSegmentMaxError: c_float,
    // Number of circle segments to use per pixel of radius for AddCircle() etc
    pub ClipRectFullscreen: ImVec4,
    // Value for PushClipRectFullscreen()
    pub InitialFlags: ImDrawListFlags,               // Initial flags at the beginning of the frame (it is possible to alter flags on a per-drawlist basis afterwards)

    // [Internal] Lookup tables
    pub ArcFastVtx: [ImVec2; IM_DRAWLIST_ARCFAST_TABLE_SIZE],
    // Sample points on the quarter of the circle.
    pub ArcFastRadiusCutoff: c_float,
    // Cutoff radius after which arc drawing will fallback to slower PathArcTo()
    pub CircleSegmentCounts: [u8; 64],
    // Precomputed segment count for given radius before we calculate it dynamically (to avoid calculation overhead)
    pub TexUvLines: *const ImVec4,                 // UV of anti-aliased lines in the atlas
}

impl ImDrawListSharedData {
    // ImDrawListSharedData();

    // void SetCircleTessellationMaxError(c_float max_error);
    pub fn SetCircleTesselationMaxError(&mut self, max_error: c_float) {
        todo!()
    }
}
