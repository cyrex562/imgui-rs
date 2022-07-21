use crate::draw_list::{DRAW_LIST_ARCFAST_SAMPLE_MAX, DRAW_LIST_ARCFAST_TABLE_SIZE, drawlist_circle_auto_segment_calc, drawlist_circle_auto_segment_calc_r, DrawListFlags};
use crate::font::Font;
use crate::vectors::Vector4D;
use crate::vectors::two_d::Vector2D;

#[derive(Default,Debug,Clone)]
pub struct DrawListSharedData
{
    // Vector2D          tex_uv_white_pixel;            // UV of white pixel in the atlas
    pub text_uv_white_pixel: Vector2D,
    // ImFont*         font;                       // current/default font (optional, for simplified add_text overload)
    pub font: *mut Font,
    // float           font_size;                   // current/default font size (optional, for simplified add_text overload)
    pub font_size: f32,
    // float           CurveTessellationTol;       // Tessellation tolerance when using PathBezierCurveTo()
    pub curve_tesselation_tol: f32,
    // float           circle_segment_max_error;      // Number of circle segments to use per pixel of radius for add_circle() etc
    pub circle_segment_max_error: f32,
    // Vector4D          ClipRectFullscreen;         // value for PushClipRectFullscreen()
    pub clip_rect_full_screen: Vector4D,
    // ImDrawListFlags initial_flags;               // Initial flags at the beginning of the frame (it is possible to alter flags on a per-drawlist basis afterwards)
    pub initial_flags: HashSet<DrawListFlags>,
    // [Internal] Lookup tables
    // Vector2D          arc_fast_vtx[IM_DRAWLIST_ARCFAST_TABLE_SIZE]; // Sample points on the quarter of the circle.
    pub arc_fast_vtx: [Vector2D;DRAW_LIST_ARCFAST_TABLE_SIZE],//Vec<Vector2D>,
    // float           arc_fast_radius_cutoff;                        // Cutoff radius after which arc drawing will fallback to slower PathArcTo()
    pub arc_fast_radius_cutoff: f32,
    // ImU8            circle_segment_counts[64];    // Precomputed segment count for given radius before we calculate it dynamically (to avoid calculation overhead)
    pub circle_segment_counts: [u8;64],
    // const Vector4D*   tex_uv_lines;                 // UV of anti-aliased lines in the atlas
    pub tex_uv_lines: Vector4D,

    // ImDrawListSharedData();

    // void SetCircleTessellationMaxError(float max_error);
}

impl DrawListSharedData {
    pub fn new() -> Self {
        let mut out = Self {

            ..Default::default()
        };
        for i in 0 .. out.arc_fast_vtx.len() {
            let a: f32 = (i * 2 * f32::PI) / out.arc_fast_vtx.len();
            out.arc_fast_vtx[i] = Vector2D::new(f32::cos(a), f32::sin(a));
        }
        out.arc_fast_radius_cutoff = drawlist_circle_auto_segment_calc_r(DRAW_LIST_ARCFAST_SAMPLE_MAX as f32, out.circle_segment_max_error);

        out
    }

    pub fn set_circle_tesselation_max_error(&mut self, max_error: f32) {

    //     if (CircleSegmentMaxError == max_error)
        //         return;
        if self.circle_segment_max_error == max_error {
            return;
        }
        //
        //     IM_ASSERT(max_error > 0.0);
        //     CircleSegmentMaxError = max_error;
        self.circle_segment_max_error = max_error;
        //     for (int i = 0; i < IM_ARRAYSIZE(CircleSegmentCounts); i += 1)
        //     {
        //         const float radius = (float)i;
        //         CircleSegmentCounts[i] = (ImU8)((i > 0) ? IM_DRAWLIST_CIRCLE_AUTO_SEGMENT_CALC(radius, CircleSegmentMaxError) : IM_DRAWLIST_ARCFAST_SAMPLE_MAX);
        //     }
        for i in 0..self.circle_segment_counts.len() {
            let radius: f32 = f32::from(i);
            self.circle_segment_counts[i] = if i > 0 {
                u8::from(drawlist_circle_auto_segment_calc(radius, self.circle_segment_max_error))
            } else {
                u8::from(DRAW_LIST_ARCFAST_SAMPLE_MAX)
            };
        }
    }
}
