use crate::imgui_h::{ImDrawListFlags, ImFont, ImVec4};
use crate::imgui_vec2::ImVec2;

pub struct ImDrawListSharedData
{
    // ImVec2          TexUvWhitePixel;            // UV of white pixel in the atlas
    pub TextUvWhitePixel: ImVec2,
    // ImFont*         Font;                       // Current/default font (optional, for simplified AddText overload)
    pub Font: *mut ImFont,
    // float           FontSize;                   // Current/default font size (optional, for simplified AddText overload)
    pub FontSize: f32,
    // float           CurveTessellationTol;       // Tessellation tolerance when using PathBezierCurveTo()
    pub CurveTesselationTol: f32,
    // float           CircleSegmentMaxError;      // Number of circle segments to use per pixel of radius for AddCircle() etc
    pub CircleSegmentMaxError: f32,
    // ImVec4          ClipRectFullscreen;         // Value for PushClipRectFullscreen()
    pub ClipRectFullScreen: ImVec4,
    // ImDrawListFlags InitialFlags;               // Initial flags at the beginning of the frame (it is possible to alter flags on a per-drawlist basis afterwards)
    pub InitialFlags: ImDrawListFlags,
    // [Internal] Lookup tables
    // ImVec2          ArcFastVtx[IM_DRAWLIST_ARCFAST_TABLE_SIZE]; // Sample points on the quarter of the circle.
    pub ArcFastVtx: Vec<ImVec2>,
    // float           ArcFastRadiusCutoff;                        // Cutoff radius after which arc drawing will fallback to slower PathArcTo()
    pub ArcFastRadiusCutoff: f32,
    // ImU8            CircleSegmentCounts[64];    // Precomputed segment count for given radius before we calculate it dynamically (to avoid calculation overhead)
    pub CircleSegmentCounts: [u8;64],
    // const ImVec4*   TexUvLines;                 // UV of anti-aliased lines in the atlas
    pub TexUvLines: *const ImVec4,
    
    // ImDrawListSharedData();
    
    // void SetCircleTessellationMaxError(float max_error);
}

impl ImDrawListSharedData {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    
    pub fn SetCircleTesselationMaxError(&mut self, max_error: f32) {
        todo!()
    }
}
