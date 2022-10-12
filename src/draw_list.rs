#![allow(non_snake_case)]

use std::mem;
use std::ptr::{null, null_mut};
use libc::{c_char, c_float, c_int, c_uint, c_void};
use crate::color::IM_COL32_A_MASK;
use crate::draw::ImDrawCallback;
use crate::draw_cmd::ImDrawCmd;
use crate::draw_cmd_header::ImDrawCmdHeader;
use crate::draw_flags::{ImDrawFlags, ImDrawFlags_Closed, ImDrawFlags_RoundCornersBottom, ImDrawFlags_RoundCornersBottomLeft, ImDrawFlags_RoundCornersBottomRight, ImDrawFlags_RoundCornersLeft, ImDrawFlags_RoundCornersMask_, ImDrawFlags_RoundCornersNone, ImDrawFlags_RoundCornersRight, ImDrawFlags_RoundCornersTop, ImDrawFlags_RoundCornersTopLeft, ImDrawFlags_RoundCornersTopRight};
use crate::draw_list_flags::{ImDrawListFlags, ImDrawListFlags_AntiAliasedFill, ImDrawListFlags_AntiAliasedLines, ImDrawListFlags_AntiAliasedLinesUseTex, ImDrawListFlags_None};
use crate::draw_list_shared_data::ImDrawListSharedData;
use crate::draw_list_splitter::ImDrawListSplitter;
use crate::draw_vert::ImDrawVert;
use crate::font::ImFont;
use crate::math::{ImAbs, ImAbs2, ImBezierCubicCalc, ImBezierQuadraticCalc, ImClamp, ImFloorSigned, ImMax, ImMin};
use crate::rect::ImRect;
use crate::vec2::ImVec2;
use crate::vec4::ImVec4;
use crate::type_defs::{ImDrawIdx, ImTextureID};

// Draw command list
// This is the low-level list of polygons that  functions are filling. At the end of the frame,
// all command lists are passed to your ImGuiIO::RenderDrawListFn function for rendering.
// Each dear imgui window contains its own ImDrawList. You can use GetWindowDrawList() to
// access the current window draw list and draw custom primitives.
// You can interleave normal  calls and adding primitives to the current draw list.
// In single viewport mode, top-left is == GetMainViewport().Pos (generally 0,0), bottom-right is == GetMainViewport().Pos+Size (generally io.DisplaySize).
// You are totally free to apply whatever transformation matrix to want to the data (depending on the use of the transformation you may want to apply it to ClipRect as well!)
// Important: Primitives are always added to the list and not culled (culling is done at higher-level by  functions), if you use this API a lot consider coarse culling your drawn objects.
#[derive(Default, Debug, Clone)]
pub struct ImDrawList {
    // This is what you have to render
    pub CmdBuffer: Vec<ImDrawCmd>,
    // Draw commands. Typically 1 command = 1 GPU draw call, unless the command is a callback.
    pub IdxBuffer: Vec<ImDrawIdx>,
    // Index buffer. Each command consume ImDrawCmd::ElemCount of those
    pub VtxBuffer: Vec<ImDrawVert>,
    // Vertex buffer.
    pub Flags: ImDrawListFlags,              // Flags, you may poke into these to adjust anti-aliasing settings per-primitive.

    // [Internal, used while building lists]
    pub _VtxCurrentIdx: c_uint,
    // [Internal] generally == VtxBuffer.Size unless we are past 64K vertices, in which case this gets reset to 0.
    pub _Data: *const ImDrawListSharedData,
    // Pointer to shared draw data (you can use GetDrawListSharedData() to get the one from current ImGui context)
    pub _OwnerName: *const c_char,
    // Pointer to owner window's name for debugging
    pub _VtxWritePtr: *mut ImDrawVert,
    // [Internal] point within VtxBuffer.Data after each add command (to avoid using the ImVector<> operators too much)
    pub _IdxWritePtr: *mut ImDrawIdx,
    // [Internal] point within IdxBuffer.Data after each add command (to avoid using the ImVector<> operators too much)
    pub _ClipRectStack: Vec<ImRect>,
    // [Internal]
    pub _TextureIdStack: Vec<ImTextureID>,
    // [Internal]
    pub _Path: Vec<ImVec2>,
    // [Internal] current path building
    pub _CmdHeader: ImDrawCmdHeader,
    // [Internal] template of active commands. Fields should match those of CmdBuffer.back().
    pub _Splitter: ImDrawListSplitter,
    // [Internal] for channels api (note: prefer using your own persistent instance of ImDrawListSplitter!)
    pub _FringeScale: c_float,       // [Internal] anti-alias fringe is scaled by this value, this helps to keep things sharp while zooming at vertex buffer content
}

impl ImDrawList {
    // If you want to create ImDrawList instances, pass them GetDrawListSharedData() or create and use your own ImDrawListSharedData (so you can use ImDrawList without ImGui)
    // ImDrawList(const ImDrawListSharedData* shared_data) { memset(this, 0, sizeof(*this)); _Data = shared_data; }
    pub fn new(shared_data: *const ImDrawListSharedData) -> Self {
        Self {
            _Data: shared_data,
            ..Default::default()
        }
    }

    // ~ImDrawList() { _ClearFreeMemory(); }

    // void  PushClipRect(const clip_rect_min: &ImVec2, const clip_rect_max: &ImVec2, bool intersect_with_current_clip_rect = false);
    // Render-level scissoring. This is passed down to your render function but not used for CPU-side coarse clipping. Prefer using higher-level PushClipRect() to affect logic (hit-testing and widget culling)
    pub unsafe fn PushClipRect(&mut self, clip_rect_min: &ImVec2, clip_rect_max: &ImVec2, intersect_with_current_clip_rect: bool) {
        let mut cr = ImVec4::new2(clip_rect_min.x, clip_rect_min.y, clip_rect_max.x, clip_rect_max.y);
        if intersect_with_current_clip_rect {
            let current = _CmdHeader.ClipRect;
            if cr.x < current.x { cr.x = current.x; }
            if cr.y < current.y { cr.y = current.y; }
            if cr.z > current.z { cr.z = current.z; }
            if cr.w > current.w { cr.w = current.w; }
        }
        cr.z = ImMax(cr.x, cr.z);
        cr.w = ImMax(cr.y, cr.w);

        self._ClipRectStack.push(cr);
        self._CmdHeader.ClipRect = cr;
        self._OnChangedClipRect();
    }

    // void  PushClipRectFullScreen();
    pub unsafe fn PushClipRectFullScreen(&mut self) {
        self.PushClipRect(&ImVec2::new2(self._Data.ClipRectFullscreen.x, self._Data.ClipRectFullscreen.y), &ImVec2::new2(self._Data.ClipRectFullscreen.z, self._Data.ClipRectFullscreen.w), false);

    }

    // void  PopClipRect();
    pub unsafe fn PopClipRect(&mut self) {
         self._ClipRectStack.pop_back();
    self._CmdHeader.ClipRect = if self._ClipRectStack.Size == 0 { self._Data.ClipRectFullscreen } else
        { self._ClipRectStack.Data[self._ClipRectStack.Size - 1] };
    self._OnChangedClipRect();

    }

    // void  PushTextureID(ImTextureID texture_id);
    pub fn PushTextureID(&mut self, texture_id: ImTextureID) {
        self._TextureIdStack.push(texture_id);
        self._CmdHeader.TextureId = texture_id;
        self._OnChangedTextureID();
    }

    // void  PopTextureID();
    pub fn PopTextureID(&mut self) {
        self._TextureIdStack.pop_back();
        self._CmdHeader.TextureId = if (self._TextureIdStack.len() == 0) {
            null_mut()
        }: self._TextureIdStack[self._TextureIdStack.len() - 1];
        self._OnChangedTextureID();

    }

    // inline ImVec2   GetClipRectMin() const { const ImVec4& cr = _ClipRectStack.back(); return ImVec2::new(cr.x, cr.y); }
    pub fn GetClipRectMin(&mut self) -> ImVec2 {
        let cr = self._ClipRectStack.last().unwrap();
        return ImVec2::new2(cr.x, cr.y);
    }


    // inline ImVec2   GetClipRectMax() const { const ImVec4& cr = _ClipRectStack.back(); return ImVec2::new(cr.z, cr.w); }
    pub fn GetClipRectMax(&mut self) -> ImVec2 {
        let cr = self._ClipRectStack.last();
        return ImVec2::new2(cr.z, cr.w);
    }

    // Primitives
// - Filled shapes must always use clockwise winding order. The anti-aliasing fringe depends on it. Counter-clockwise shapes will have "inward" anti-aliasing.
// - For rectangular primitives, "p_min" and "p_max" represent the upper-left and lower-right corners.
// - For circle primitives, use "num_segments == 0" to automatically calculate tessellation (preferred).
//   In older versions (until Dear ImGui 1.77) the AddCircle functions defaulted to num_segments == 12.
//   In future versions we will use textures to provide cheaper and higher-quality circles.
//   Use AddNgon() and AddNgonFilled() functions if you need to guaranteed a specific number of sides.
// void  AddLine(const p1: &ImVec2, const p2: &ImVec2, u32 col, c_float thickness = 1f32);
    pub fn AddLine(&mut self, p1: &ImVec2, p2: &ImVec2, col: u32, thicknetss: c_float) {
        if (col & IM_COL32_A_MASK) == 0 {
            return;
        }
        self.PathLineTo(p1 + ImVec2::new2(0.5f32, 0.5f32));
        self.PathLineTo(p2 + ImVec2::new2(0.5f32, 0.5f32));
        self.PathStroke(col, 0, thickness);
    }

    // void  AddRect(const p_min: &ImVec2, const p_max: &ImVec2, u32 col, c_float rounding = 0f32, ImDrawFlags flags = 0, c_float thickness = 1f32);   // a: upper-left, b: lower-right (== upper-left + size)
    pub fn AddRect(&mut self, p_min: &ImVec2, p_max: &ImVec2, col: u32, rounding: c_float, flags: ImDrawFlags, thickness: f32) {
        if (col & IM_COL32_A_MASK) == 0 {
            return;
        }
        if self.Flags & ImDrawListFlags_AntiAliasedLines {
            self.PathRect(p_min + ImVec2::new2(0.50f32, 0.500f32), p_max - ImVec2::new2(0.50f32, 0.500f32), rounding, flags);
        }
        // Better looking lower-right corner and rounded non-AA shapes.
        self.PathStroke(col, ImDrawFlags_Closed, thickness);
    }


    // void  AddRectFilled(const p_min: &ImVec2, const p_max: &ImVec2, u32 col, c_float rounding = 0f32, ImDrawFlags flags = 0);                     // a: upper-left, b: lower-right (== upper-left + size)
    pub fn AddRectFilled(&mut self, p_min: &ImVec2, p_masx: &ImVec2, col: u32, rounding: c_float, flags: ImDrawFlags) {
        if (col & IM_COL32_A_MASK) == 0
        {
            return;
        }
        if rounding < 0.5f32 || (flags & ImDrawFlags_RoundCornersMask_) == ImDrawFlags_RoundCornersNone
        {
            self.PrimReserve(6, 4);
            PrimRect::new(p_min, p_max, col);
        }
        else
        {
            self.PathRect(p_min, p_max, rounding, flags);
            self.PathFillConvex(col);
        }


    }

    // void  AddRectFilledMultiColor(const p_min: &ImVec2, const p_max: &ImVec2, u32 col_upr_left, u32 col_upr_right, u32 col_bot_right, u32 col_bot_left);
    pub unsafe fn AddRectFilledMultiColor(&mut self, p_min: &ImVec2, p_max: &ImVec2, col_upr_left: u32, col_upr_right: u32, col_bot_right: u32, col_bot_left: u32) {

        if ((col_upr_left | col_upr_right | col_bot_right | col_bot_left) & IM_COL32_A_MASK) == 0
        {
            return;
        }

        let uv: ImVec2 = self._Data.TexUvWhitePixel;
        self.PrimReserve(6, 4);
        self.PrimWriteIdx((self._VtxCurrentIdx) as ImDrawIdx);
        self.PrimWriteIdx((self._VtxCurrentIdx + 1) as ImDrawIdx);
        self.PrimWriteIdx((self._VtxCurrentIdx + 2) as ImDrawIdx);
        self.PrimWriteIdx((self._VtxCurrentIdx) as ImDrawIdx);
        self.PrimWriteIdx((self._VtxCurrentIdx + 2) as ImDrawIdx);
        self.PrimWriteIdx((self._VtxCurrentIdx + 3) as ImDrawIdx);
        self.PrimWriteVtx(p_min, &uv, col_upr_left);
        self.PrimWriteVtx(&ImVec2::new2(p_max.x, p_min.y), &uv, col_upr_right);
        self.PrimWriteVtx(p_max, &uv, col_bot_right);
        self.PrimWriteVtx(&ImVec2::new2(p_min.x, p_max.y), &uv, col_bot_left);

    }

    // void  AddQuad(const p1: &ImVec2, const p2: &ImVec2, const p3: &ImVec2, const p4: &ImVec2, u32 col, c_float thickness = 1f32);
    pub fn AddQuad(&mut self, p1: &ImVec2, p2: &ImVec2, p3: &ImVec2, p4: &ImVec2, col: u32, thickness: c_float) {
        if (col & IM_COL32_A_MASK) == 0
        {
            return;
        }

        self.PathLineTo(p1);
        self.PathLineTo(p2);
        self.PathLineTo(p3);
        self.PathLineTo(p4);
        self.PathStroke(col, ImDrawFlags_Closed, thickness);

    }

    // void  AddQuadFilled(const p1: &ImVec2, const p2: &ImVec2, const p3: &ImVec2, const p4: &ImVec2, u32 col);
    pub fn AddQuadFilled(&mut self, p1: &ImVec2, p2: &ImVec2, p3: &ImVec2, p4: &ImVec2, col: u32) {
        if (col & IM_COL32_A_MASK) == 0
        {
            return;
        }

        self.PathLineTo(p1);
        self.PathLineTo(p2);
        self.PathLineTo(p3);
        self.PathLineTo(p4);
        self.PathFillConvex(col);


    }


    // void  AddTriangle(const p1: &ImVec2, const p2: &ImVec2, const p3: &ImVec2, u32 col, c_float thickness = 1f32);
    pub fn AddTriangle(&mut self, p1: &ImVec2, p2: &ImVec2, p3: &ImVec2, col: u32, thickness: c_float) {
        if (col & IM_COL32_A_MASK) == 0
        {
            return;
        }

        self.PathLineTo(p1);
        self.PathLineTo(p2);
        self.PathLineTo(p3);
        self.PathStroke(col, ImDrawFlags_Closed, thickness);
    }


    // void  AddTriangleFilled(const p1: &ImVec2, const p2: &ImVec2, const p3: &ImVec2, u32 col);
    pub fn AddTriangleFilled(&mut self, p1: &ImVec2, p2: &ImVec2, p3: &ImVec2, col: u32) {
        if (col & IM_COL32_A_MASK) == 0
        {
            return;
        }

        self.PathLineTo(p1);
        self.PathLineTo(p2);
        self.PathLineTo(p3);
        self.PathFillConvex(col);

    }

    // void  AddCircle(const center: &ImVec2, radius: c_float, u32 col, c_int num_segments = 0, c_float thickness = 1f32);
    pub fn AddCircle(&mut self, center: &ImVec2, radius: c_float, col: u32, mut num_segments: c_int, thickness: c_float) {
        if (col & IM_COL32_A_MASK) == 0 || radius < 0.5f32
        {
            return;
        }

        if (num_segments <= 0)
        {
            // Use arc with automatic segment count
            self._PathArcToFastEx(center, radius - 0.5f32, 0, IM_DRAWLIST_ARCFAST_SAMPLE_MAX, 0);
            self._Path.Size-= 1;
        }
        else
        {
            // Explicit segment count (still clamp to avoid drawing insanely tessellated shapes)
            num_segments = ImClamp(num_segments, 3, IM_DRAWLIST_CIRCLE_AUTO_SEGMENT_MAX);

            // Because we are filling a closed shape we remove 1 from the count of segments/points
            let a_max: c_float =  (IM_PI * 2.00f32) * (num_segments - 1f32) / num_segments;
            self.PathArcTo(center, radius - 0.5f32, 0f32, a_max, num_segments - 1);
        }

        self.PathStroke(col, ImDrawFlags_Closed, thickness);


    }


    // void  AddCircleFilled(const center: &ImVec2, radius: c_float, u32 col, c_int num_segments = 0);
    pub fn AddCircleFilled(&mut self, center: &ImVec2, radius: c_float, col: u32, mut num_segments: c_int) {
        if (col & IM_COL32_A_MASK) == 0 || radius < 0.5f32
        {
            return;
        }

        if num_segments <= 0
        {
            // Use arc with automatic segment count
            self._PathArcToFastEx(center, radius, 0, IM_DRAWLIST_ARCFAST_SAMPLE_MAX, 0);
            self._Path.Size-= 1;
        }
        else
        {
            // Explicit segment count (still clamp to avoid drawing insanely tessellated shapes)
            num_segments = ImClamp(num_segments, 3, IM_DRAWLIST_CIRCLE_AUTO_SEGMENT_MAX);

            // Because we are filling a closed shape we remove 1 from the count of segments/points
            let a_max: c_float =  (IM_PI * 2.00f32) * (num_segments - 1f32) / num_segments;
            self.PathArcTo(center, radius, 0f32, a_max, num_segments - 1);
        }

        self.PathFillConvex(col);
    }

    // void  AddNgon(const center: &ImVec2, radius: c_float, u32 col, num_segments: c_int, c_float thickness = 1f32);
    pub fn AddNgon(&mut self, center: &ImVec2, radius: c_float, col: u32, num_semgnets: c_int, thickness: c_float) {
        if (col & IM_COL32_A_MASK) == 0 || num_segments <= 2 {
            return;
        }

        // Because we are filling a closed shape we remove 1 from the count of segments/points
        let a_max: c_float = (IM_PI * 2.00f32) * (num_segments - 1f32) / num_segments;
        self.PathArcTo(center, radius - 0.5f32, 0f32, a_max, num_segments - 1);
        self.PathStroke(col, ImDrawFlags_Closed, thickness);
    }


    // void  AddNgonFilled(const center: &ImVec2, radius: c_float, u32 col, num_segments: c_int);
    pub fn AddNgonFilled(&mut self, center: &ImVec2, radius: c_float, col: u32, num_segments: c_int) {
        if (col & IM_COL32_A_MASK) == 0 || num_segments <= 2
        {
            return;
        }

        // Because we are filling a closed shape we remove 1 from the count of segments/points
        let a_max: c_float =  (IM_PI * 2.00f32) * (num_segments - 1f32) / num_segments;
        self.PathArcTo(center, radius, 0f32, a_max, num_segments - 1);
        self.PathFillConvex(col);
    }


    // void  AddText(const pos: &ImVec2, u32 col, const char* text_begin, const char* text_end = NULL);
    pub unsafe fn AddText(&mut self, pos: *const ImVec2, col: u32, text_begin: *const c_char, mut text_end: *mut c_char) {
        self.AddText2(null_mut(), 0f32, &*pos, col, text_begin, text_end, 0.0, null());
    }

    // void  AddText(const font: *mut ImFont, font_size: c_float, const pos: &ImVec2, u32 col, const char* text_begin, const char*
// text_end = NULL, c_float wrap_width = 0f32, const ImVec4* cpu_fine_clip_rect = NULL);
    pub unsafe fn AddText2(&mut self, mut font: *mut ImFont, mut font_size: c_float, pos: &ImVec2, col: u32, text_begin: *const c_char, mut text_end: *const c_char, wrap_width: c_float, cpu_fine_clip_rect: *const ImVec4) {
        if (col & IM_COL32_A_MASK) == 0
        {
            return;
        }

        if text_end == null_mut()
        {
            text_end = text_begin + libc::strlen(text_begin);
        }
        if text_begin == text_end
        {
            return;
        }

        // Pull default font/size from the shared ImDrawListSharedData instance
        if font == null_mut()
        {
            font = self._Data.Font;
        }
        if font_size == 0f32
        {
            font_size = self._Data.FontSize;
        }

        // IM_ASSERT(font.Containeratlas.TexID == _CmdHeader.TextureId);  // Use high-level PushFont() or low-level ImDrawList::PushTextureId() to change font.

        let mut clip_rect = self._CmdHeader.ClipRect.clone();
        if cpu_fine_clip_rect
        {
            clip_rect.x = ImMax(clip_rect.x, cpu_fine_clip_rect.x);
            clip_rect.y = ImMax(clip_rect.y, cpu_fine_clip_rect.y);
            clip_rect.z = ImMin(clip_rect.z, cpu_fine_clip_rect.z);
            clip_rect.w = ImMin(clip_rect.w, cpu_fine_clip_rect.w);
        }
        (this, font_size, pos, col, clip_rect, text_begin, text_end, wrap_width, cpu_fine_clip_rect != null_mut());


    }


    // void  AddPolyline(const points: *mut ImVec2, num_points: c_int, u32 col, ImDrawFlags flags, thickness: c_float);
    pub fn AddPolyline(&mut self, points: *const ImVec2, num_points: c_int, col: u32, flags: ImDrawFlags, mut thickness: c_float) {
        if points_count < 2 {
            return;
        }

        let closed: bool = (flags & ImDrawFlags_Closed) != 0;
        let opaque_uv: ImVec2 = _Data.TexUvWhitePixel;
        let count: c_int = if closed {
            points_count
        } else { points_count - 1 }; // The number of line segments we need to draw
        let thick_line: bool = (thickness > _FringeScale);

        if Flags & ImDrawListFlags_AntiAliasedLines {
            // Anti-aliased stroke
            let AA_SIZE: c_float = _FringeScale;
            let col_trans: u32 = col & !IM_COL32_A_MASK;

            // Thicknesses <1.0 should behave like thickness 1.0
            thickness = ImMax(thickness, 1f32);
            let integer_thickness: c_int = thickness as c_int;
            let fractional_thickness: c_float = thickness.clone() - integer_thickness;

            // Do we want to draw this line using a texture?
            // - For now, only draw integer-width lines using textures to avoid issues with the way scaling occurs, could be improved.
            // - If AA_SIZE is not 1f32 we cannot use the texture path.
            let use_texture: bool = (Flags & ImDrawListFlags_AntiAliasedLinesUseTex) && (integer_thickness < IM_DRAWLIST_TEX_LINES_WIDTH_MAX) && (fractional_thickness <= 0.000010f32) && (AA_SIZE == 1f32);

            // We should never hit this, because NewFrame() doesn't set ImDrawListFlags_AntiAliasedLinesUseTex unless ImFontAtlasFlags_NoBakedLines is off
            // IM_ASSERT_PARANOID(!use_texture || !(_Data.Font.Containeratlas.Flags & ImFontAtlasFlags_NoBakedLines));

            let idx_count: c_int = if use_texture { count * 6 } else {
                if thick_line {
                    count * 18
                } else { count * 12 }
            };
            let vtx_count: c_int = if use_texture { points_count * 2 } else { if thick_line {
                points_count * 4
            } else { points_count * 3 }};
            self.PrimReserve(idx_count, vtx_count);

            // Temporary buffer
            // The first <points_count> items are normals at each line point, then after that there are either 2 or 4 temp points for each line point
            ImVec2 * temp_normals = alloca(points_count * (if (use_texture || !thick_line) { 3 }else { 5 }) * sizeof(ImVec2)); //-V630
            ImVec2 * temp_points = temp_normals + points_count;

            // Calculate normals (tangents) for each line segment
            // for (let i1: c_int = 0; i1 < count; i1+ +)
            for i1 in 0 .. count
            {
                let i2: c_int = if (i1 + 1) == points_count {
                    0
                } else { i1 + 1 };
                let dx: c_float = points[i2].x - points[i1].x;
                let dy: c_float = points[i2.clone()].y - points[i1].y;
                IM_NORMALIZE2F_OVER_ZERO(dx, dy);
                temp_normals[i1].x = dy.clone();
                temp_normals[i1].y = -dx.clone();
            }
            if !closed {
                temp_normals[points_count - 1] = temp_normals[points_count - 2];
            }

            // If we are drawing a one-pixel-wide line without a texture, or a textured line of any width, we only need 2 or 3 vertices per point
            if (use_texture || !thick_line) {
                // [PATH 1] Texture-based lines (thick or non-thick)
                // [PATH 2] Non texture-based lines (non-thick)

                // The width of the geometry we need to draw - this is essentially <thickness> pixels for the line itself, plus "one pixel" for AA.
                // - In the texture-based path, we don't use AA_SIZE here because the +1 is tied to the generated texture
                //   (see ImFontAtlasBuildRenderLinesTexData() function), and so alternate values won't work without changes to that code.
                // - In the non texture-based paths, we would allow AA_SIZE to potentially be != 1f32 with a patch (e.g. fringe_scale patch to
                //   allow scaling geometry while preserving one-screen-pixel AA fringe).
                let half_draw_size: c_float = use_texture?((thickness * 0.5f32) + 1): AA_SIZE;

                // If line is not closed, the first and last points need to be generated differently as there are no normals to blend
                if (!closed) {
                    temp_points[0] = points[0] + temp_normals[0] * half_draw_size;
                    temp_points[1] = points[0] - temp_normals[0] * half_draw_size;
                    temp_points[(points_count - 1) * 2 + 0] = points[points_count - 1] + temp_normals[points_count - 1] * half_draw_size;
                    temp_points[(points_count - 1) * 2 + 1] = points[points_count - 1] - temp_normals[points_count - 1] * half_draw_size;
                }

                // Generate the indices to form a number of triangles for each line segment, and the vertices for the line edges
                // This takes points n and n+1 and writes into n+1, with the first point in a closed line being generated from the final one (as n+1 wraps)
                // FIXME-OPT: Merge the different loops, possibly remove the temporary buffer.
                let mut idx1: c_uint =self._VtxCurrentIdx.clone(); // Vertex index for start of line segment
                // for (let i1: c_int = 0; i1 < count; i1+ +) // i1 is the first point of the line segment
                for i1 in 0 .. count
                {
                    let i2: c_int = if (i1 + 1) == points_count {
                        0
                    } else { i1 + 1 }; // i2 is the second point of the line segment
                    let mut idx2: c_uint = if (i1 + 1) == points_count {
                        self._VtxCurrentIdx.clone()
                    } else {
                        if idx1 + use_texture.clone() {
                            2
                        } else { 3 }
                }; // Vertex index for end of segment

                    // Average normals
                    let mut dm_x: c_float = (temp_normals[i1].x + temp_normals[i2].x) * 0.5f32;
                    let mut dm_y: c_float = (temp_normals[i1].y + temp_normals[i2.clone()].y) * 0.5f32;
                    IM_FIXNORMAL2F(dm_x, dm_y);
                    dm_x *= half_draw_size.clone(); // dm_x, dm_y are offset to the outer edge of the AA area
                    dm_y *= half_draw_size.clone();

                    // Add temporary vertexes for the outer edges
                    let mut out_vtx = &mut temp_points[i2.clone() * 2];
                    out_vtx[0].x = points[i2.clone()].x + dm_x.clone();
                    out_vtx[0].y = points[i2.clone()].y + dm_y.clone();
                    out_vtx[1].x = points[i2.clone()].x - dm_x.clone();
                    out_vtx[1].y = points[i2.clone()].y - dm_y.clone();

                    if use_texture {
                        // Add indices for two triangles
                        self._IdxWritePtr[0] = (idx2 + 0);
                        self._IdxWritePtr[1] = (idx1.clone() + 0);
                        self._IdxWritePtr[2] = (idx1.clone() + 1); // Right tri
                        self._IdxWritePtr[3] = (idx2.clone() + 1);
                        self._IdxWritePtr[4] = (idx1.clone() + 1);
                        self._IdxWritePtr[5] = (idx2.clone() + 0); // Left tri
                        self._IdxWritePtr += 6;
                    } else {
                        // Add indexes for four triangles
                        self._IdxWritePtr[0] = (idx2.clone() + 0);
                        self._IdxWritePtr[1] = (idx1.clone() + 0);
                        self._IdxWritePtr[2] = (idx1.clone() + 2); // Right tri 1
                        self._IdxWritePtr[3] = (idx1.clone() + 2);
                        self._IdxWritePtr[4] = (idx2.clone() + 2);
                        self._IdxWritePtr[5] = (idx2.clone() + 0); // Right tri 2
                        self._IdxWritePtr[6] = (idx2.clone() + 1);
                        self._IdxWritePtr[7] = (idx1.clone() + 1);
                        self._IdxWritePtr[8] = (idx1.clone() + 0); // Left tri 1
                        self._IdxWritePtr[9] = (idx1.clone() + 0);
                        self._IdxWritePtr[10] = (idx2.clone() + 0);
                        self._IdxWritePtr[11] = (idx2.clone() + 1); // Left tri 2
                        self._IdxWritePtr += 12;
                    }

                    idx1 = idx2;
                }

                // Add vertexes for each point on the line
                if use_texture {
                    // If we're using textures we only need to emit the left/right edge vertices
                    let tex_uvs = self._Data.TexUvLines[integer_thickness];
                    /*if (fractional_thickness != 0f32) // Currently always zero when use_texture==false!
                    {
                        const ImVec4 tex_uvs_1 = _Data.TexUvLines[integer_thickness + 1];
                        tex_uvs.x = tex_uvs.x + (tex_uvs_1.x - tex_uvs.x) * fractional_thickness; // inlined ImLerp()
                        tex_uvs.y = tex_uvs.y + (tex_uvs_1.y - tex_uvs.y) * fractional_thickness;
                        tex_uvs.z = tex_uvs.z + (tex_uvs_1.z - tex_uvs.z) * fractional_thickness;
                        tex_uvs.w = tex_uvs.w + (tex_uvs_1.w - tex_uvs.w) * fractional_thickness;
                    }*/
                    let tex_uv0 = ImVec2::new2(tex_uvs.x, tex_uvs.y);
                    let tex_uv1 = ImVec2::new2(tex_uvs.z, tex_uvs.w);
                    // for (let i: c_int = 0; i < points_count; i+ +)
                    for i in 0 .. points_count
                    {
                        self._VtxWritePtr[0].pos = temp_points[i * 2 + 0];
                        self._VtxWritePtr[0].uv = tex_uv0;
                        self._VtxWritePtr[0].col = col.clone(); // Left-side outer edge
                        self._VtxWritePtr[1].pos = temp_points[i * 2 + 1];
                        self._VtxWritePtr[1].uv = tex_uv1;
                        self._VtxWritePtr[1].col = col.clone(); // Right-side outer edge
                        self._VtxWritePtr += 2;
                    }
                } else {
                    // If we're not using a texture, we need the center vertex as well
                    // for (let i: c_int = 0; i < points_count; i+ +)
                    for i in 0 .. points_count
                    {
                        self._VtxWritePtr[0].pos = points[i];
                        self._VtxWritePtr[0].uv = opaque_uv;
                        self._VtxWritePtr[0].col = col;       // Center of line
                        self._VtxWritePtr[1].pos = temp_points[i * 2 + 0];
                        self._VtxWritePtr[1].uv = opaque_uv;
                        self._VtxWritePtr[1].col = col_trans; // Left-side outer edge
                        self._VtxWritePtr[2].pos = temp_points[i * 2 + 1];
                        self._VtxWritePtr[2].uv = opaque_uv;
                        self._VtxWritePtr[2].col = col_trans; // Right-side outer edge
                        self._VtxWritePtr += 3;
                    }
                }
            } else {
                // [PATH 2] Non texture-based lines (thick): we need to draw the solid line core and thus require four vertices per point
                let half_inner_thickness: c_float = (thickness - AA_SIZE) * 0.5f32;

                // If line is not closed, the first and last points need to be generated differently as there are no normals to blend
                if (!closed) {
                    let points_last: c_int = points_count - 1;
                    temp_points[0] = points[0] + temp_normals[0] * (half_inner_thickness + AA_SIZE);
                    temp_points[1] = points[0] + temp_normals[0] * (half_inner_thickness);
                    temp_points[2] = points[0] - temp_normals[0] * (half_inner_thickness);
                    temp_points[3] = points[0] - temp_normals[0] * (half_inner_thickness + AA_SIZE);
                    temp_points[points_last * 4 + 0] = points[points_last] + temp_normals[points_last] * (half_inner_thickness + AA_SIZE);
                    temp_points[points_last * 4 + 1] = points[points_last] + temp_normals[points_last] * (half_inner_thickness);
                    temp_points[points_last * 4 + 2] = points[points_last] - temp_normals[points_last] * (half_inner_thickness);
                    temp_points[points_last * 4 + 3] = points[points_last] - temp_normals[points_last] * (half_inner_thickness + AA_SIZE);
                }

                // Generate the indices to form a number of triangles for each line segment, and the vertices for the line edges
                // This takes points n and n+1 and writes into n+1, with the first point in a closed line being generated from the final one (as n+1 wraps)
                // FIXME-OPT: Merge the different loops, possibly remove the temporary buffer.
                let mut idx1: c_uint =self._VtxCurrentIdx; // Vertex index for start of line segment
                // for (let i1: c_int = 0; i1 < count; i1+ +) // i1 is the first point of the line segment
                for i1 in 0 .. count
                {
                    let i2: c_int = if (i1 + 1) ==  points_count {
                        0
                    } else {
                        i1 + 1
                    }; // i2 is the second point of the line segment
                    let mut idx2: c_uint = if (i1 + 1) == points_count {
                        self._VtxCurrentIdx.clone()
                    } else {
                        
                            idx1 + 4
                    }; // Vertex index for end of segment

                    // Average normals
                    let dm_x: c_float = (temp_normals[i1].x + temp_normals[i2].x) * 0.5f32;
                    let dm_y: c_float = (temp_normals[i1].y + temp_normals[i2].y) * 0.5f32;
                    IM_FIXNORMAL2F(dm_x, dm_y);
                    let dm_out_x: c_float = dm_x.clone() * (half_inner_thickness.clone() + AA_SIZE.clone());
                    let dm_out_y: c_float = dm_y.clone() * (half_inner_thickness.clone() + AA_SIZE.clone());
                    let dm_in_x: c_float = dm_x.clone() * half_inner_thickness.clone();
                    let dm_in_y: c_float = dm_y.clone() * half_inner_thickness.clone();

                    // Add temporary vertices
                    ImVec2 * out_vtx = &temp_points[i2.clone() * 4];
                    out_vtx[0].x = points[i2.clone()].x + dm_out_x;
                    out_vtx[0].y = points[i2.clone()].y + dm_out_y;
                    out_vtx[1].x = points[i2.clone()].x + dm_in_x;
                    out_vtx[1].y = points[i2.clone()].y + dm_in_y;
                    out_vtx[2].x = points[i2.clone()].x - dm_in_x.clone();
                    out_vtx[2].y = points[i2.clone()].y - dm_in_y.clone();
                    out_vtx[3].x = points[i2.clone()].x - dm_out_x.clone();
                    out_vtx[3].y = points[i2.clone()].y - dm_out_y.clone();

                    // Add indexes
                    self._IdxWritePtr[0] = (idx2 + 1);
                    self._IdxWritePtr[1] = (idx1.clone() + 1);
                    self._IdxWritePtr[2] = (idx1.clone() + 2);
                    self._IdxWritePtr[3] = (idx1.clone() + 2);
                    self._IdxWritePtr[4] = (idx2.clone() + 2);
                    self._IdxWritePtr[5] = (idx2.clone() + 1);
                    self._IdxWritePtr[6] = (idx2.clone() + 1);
                    self._IdxWritePtr[7] = (idx1.clone() + 1);
                    self._IdxWritePtr[8] = (idx1.clone() + 0);
                    self._IdxWritePtr[9] = (idx1.clone() + 0);
                    self._IdxWritePtr[10] = (idx2.clone() + 0);
                    self._IdxWritePtr[11] = (idx2.clone() + 1);
                    self._IdxWritePtr[12] = (idx2.clone() + 2);
                    self._IdxWritePtr[13] = (idx1.clone() + 2);
                    self._IdxWritePtr[14] = (idx1.clone() + 3);
                    self._IdxWritePtr[15] = (idx1.clone() + 3);
                    self._IdxWritePtr[16] = (idx2.clone() + 3);
                    self._IdxWritePtr[17] = (idx2.clone() + 2);
                    self._IdxWritePtr += 18;

                    idx1 = idx2.clone();
                }

                // Add vertices
                // for (let i: c_int = 0; i < points_count; i+ +)
                for i in 0 .. points_count
                {
                    self._VtxWritePtr[0].pos = temp_points[i * 4 + 0];
                    self._VtxWritePtr[0].uv = opaque_uv;
                    self._VtxWritePtr[0].col = col_trans;
                    self._VtxWritePtr[1].pos = temp_points[i * 4 + 1];
                    self._VtxWritePtr[1].uv = opaque_uv;
                    self._VtxWritePtr[1].col = col;
                    self._VtxWritePtr[2].pos = temp_points[i * 4 + 2];
                    self._VtxWritePtr[2].uv = opaque_uv;
                    self._VtxWritePtr[2].col = col;
                    self._VtxWritePtr[3].pos = temp_points[i * 4 + 3];
                    self._VtxWritePtr[3].uv = opaque_uv;
                    self._VtxWritePtr[3].col = col_trans;
                    self._VtxWritePtr += 4;
                }
            }
           self._VtxCurrentIdx += vtx_count;
        } else {
            // [PATH 4] Non texture-based, Non anti-aliased lines
            let idx_count: c_int = count * 6;
            let vtx_count: c_int = count * 4;    // FIXME-OPT: Not sharing edges
            PrimReserve(idx_count, vtx_count);

            // for (let i1: c_int = 0; i1 < count; i1+ +)
            for i1 in 0 .. count
            {
                let i2: c_int = if(i1 + 1) == points_count {
                    0
                } else { i1 + 1 };
                let p1 = points[i1];
                let p2 = points[i2];

                let mut dx: c_float = p2.x - p1.x;
                let mut dy: c_float = p2.y - p1.y;
                IM_NORMALIZE2F_OVER_ZERO(dx, dy);
                dx *= (thickness * 0.5f32);
                dy *= (thickness * 0.5f32);

                self._VtxWritePtr[0].pos.x = p1.x + dy;
                self._VtxWritePtr[0].pos.y = p1.y - dx;
                self._VtxWritePtr[0].uv = opaque_uv;
                self._VtxWritePtr[0].col = col;
                self._VtxWritePtr[1].pos.x = p2.x + dy;
                self._VtxWritePtr[1].pos.y = p2.y - dx;
                self._VtxWritePtr[1].uv = opaque_uv;
                self._VtxWritePtr[1].col = col;
                self._VtxWritePtr[2].pos.x = p2.x - dy;
                self._VtxWritePtr[2].pos.y = p2.y + dx;
                self._VtxWritePtr[2].uv = opaque_uv;
                self._VtxWritePtr[2].col = col;
                self._VtxWritePtr[3].pos.x = p1.x - dy;
                self._VtxWritePtr[3].pos.y = p1.y + dx;
                self._VtxWritePtr[3].uv = opaque_uv;
                self._VtxWritePtr[3].col = col;
                self._VtxWritePtr += 4;

                self._IdxWritePtr[0] = (self._VtxCurrentIdx);
                self._IdxWritePtr[1] = (self._VtxCurrentIdx + 1);
                self._IdxWritePtr[2] = (self._VtxCurrentIdx + 2);
                self._IdxWritePtr[3] = (self._VtxCurrentIdx);
                self._IdxWritePtr[4] = (self._VtxCurrentIdx + 2);
                self._IdxWritePtr[5] = (self._VtxCurrentIdx + 3);
                self._IdxWritePtr += 6;
               self._VtxCurrentIdx += 4;
            }
        }


    }


    // void  AddConvexPolyFilled(const points: *mut ImVec2, num_points: c_int, u32 col);
    pub fn AddConvexPolyFilled(&mut self, points: *const ImVec2, num_points: c_int, col: u32) {
        if points_count < 3
        {
            return;
        }

        let uv: ImVec2 = _Data.TexUvWhitePixel;

        if (Flags & ImDrawListFlags_AntiAliasedFill)
        {
            // Anti-aliased Fill
            let AA_SIZE: c_float =  _FringeScale;
            let col_trans: u32 = col & !IM_COL32_A_MASK;
            let idx_count: c_int = (points_count - 2)*3 + points_count * 6;
            let vtx_count: c_int = (points_count * 2);
            PrimReserve(idx_count, vtx_count);

            // Add indexes for fill
            let mut vtx_inner_idx: c_uint =  self._VtxCurrentIdx;
            let mut vtx_outer_idx: c_uint =  self._VtxCurrentIdx + 1;
            // for (let i: c_int = 2; i < points_count; i++)
            for i in 2 .. points_count
            {
                self._IdxWritePtr[0] = (vtx_inner_idx); self._IdxWritePtr[1] = (vtx_inner_idx + ((i - 1) << 1)); self._IdxWritePtr[2] = (vtx_inner_idx + (i << 1));
                self._IdxWritePtr += 3;
            }

            // Compute normals
            ImVec2* temp_normals = alloca(points_count * sizeof(ImVec2)); //-V630
            // for (let i0: c_int = points_count - 1, i1 = 0; i1 < points_count; i0 = i1++)
            let mut i0 = points_count - 1;
            let mut i1 = 0;
            while i1 < points_count
            {
                let p0 = points[i0];
                let p1 = points[i1];
                let dx: c_float =  p1.x - p0.x;
                let dy: c_float =  p1.y - p0.y;
                IM_NORMALIZE2F_OVER_ZERO(dx, dy);
                temp_normals[i0].x = dy;
                temp_normals[i0].y = -dx;
                i0 = i1;
                i1 += 1;
            }

            // for (let i0: c_int = points_count - 1, i1 = 0; i1 < points_count; i0 = i1++)
            i0 = points_count - 1;
            i1 = 0;
            while i1 < points_count
            {
                // Average normals
                let n0 = temp_normals[i0];
                let n1 = temp_normals[i1];
                let mut dm_x: c_float =  (n0.x + n1.x) * 0.5f32;
                let mut dm_y: c_float =  (n0.y + n1.y) * 0.5f32;
                IM_FIXNORMAL2F(dm_x, dm_y);
                dm_x *= AA_SIZE * 0.5f32;
                dm_y *= AA_SIZE * 0.5f32;

                // Add vertices
                _VtxWritePtr[0].pos.x = (points[i1].x - dm_x); _VtxWritePtr[0].pos.y = (points[i1].y - dm_y); _VtxWritePtr[0].uv = uv; _VtxWritePtr[0].col = col;        // Inner
                _VtxWritePtr[1].pos.x = (points[i1].x + dm_x); _VtxWritePtr[1].pos.y = (points[i1].y + dm_y); _VtxWritePtr[1].uv = uv; _VtxWritePtr[1].col = col_trans;  // Outer
                _VtxWritePtr += 2;

                // Add indexes for fringes
                self._IdxWritePtr[0] = (vtx_inner_idx + (i1 << 1)); self._IdxWritePtr[1] = (vtx_inner_idx + (i0 << 1)); self._IdxWritePtr[2] = (vtx_outer_idx + (i0 << 1));
                self._IdxWritePtr[3] = (vtx_outer_idx + (i0 << 1)); self._IdxWritePtr[4] = (vtx_outer_idx + (i1 << 1)); self._IdxWritePtr[5] = (vtx_inner_idx + (i1 << 1));
                self._IdxWritePtr += 6;
            }
            self._VtxCurrentIdx += vtx_count;
        }
        else
        {
            // Non Anti-aliased Fill
            let idx_count: c_int = (points_count - 2)*3;
            let vtx_count: c_int = points_count;
            PrimReserve(idx_count, vtx_count);
            // for (let i: c_int = 0; i < vtx_count; i++)
            for i in 0 .. vtx_count
            {
                _VtxWritePtr[0].pos = points[i]; _VtxWritePtr[0].uv = uv; _VtxWritePtr[0].col = col;
                _VtxWritePtr+= 1;
            }
            // for (let i: c_int = 2; i < points_count; i++)
            for i in 2 .. points_count
            {

                self._IdxWritePtr[0] = (self._VtxCurrentIdx); self._IdxWritePtr[1] = (self._VtxCurrentIdx + i - 1); self._IdxWritePtr[2] = (self._VtxCurrentIdx + i);
                self._IdxWritePtr += 3;
            }
            self._VtxCurrentIdx += vtx_count;
        }


    }

    // void  AddBezierCubic(const p1: &ImVec2, const p2: &ImVec2, const p3: &ImVec2, const p4: &ImVec2, u32 col, thickness: c_float, c_int num_segments = 0); // Cubic Bezier (4 control points)
    pub fn AddBezierCubic(&mut self, p1: &ImVec2, p2: &ImVec2, p3: &ImVec2, p4: &ImVec2, col: u32, thickness: c_float, num_segments: c_int) {
        if (col & IM_COL32_A_MASK) == 0
        {
            return;
        }

        self.PathLineTo(p1);
        self.PathBezierCubicCurveTo(p2, p3, p4, num_segments);
        self.PathStroke(col, 0, thickness);

    }

    // void  AddBezierQuadratic(const p1: &ImVec2, const p2: &ImVec2, const p3: &ImVec2, u32 col, thickness: c_float, c_int num_segments = 0);               // Quadratic Bezier (3 control points)
    pub fn AddBezierQuadratic(&mut self, p1: &ImVec2, p2: &ImVec2, p3: &ImVec2, col: u32, thickness: c_float, num_segments: c_int) {
        if (col & IM_COL32_A_MASK) == 0
        {
            return;
        }

        self.PathLineTo(p1);
        self.PathBezierQuadraticCurveTo(p2, p3, num_segments);
        self.PathStroke(col, 0, thickness);

    }

    // Image primitives
// - Read FAQ to understand what ImTextureID is.
// - "p_min" and "p_max" represent the upper-left and lower-right corners of the rectangle.
// - "uv_min" and "uv_max" represent the normalized texture coordinates to use for those corners. Using (0,0)->(1,1) texture coordinates will generally display the entire texture.
// void  AddImage(ImTextureID user_texture_id, const p_min: &ImVec2, const p_max: &ImVec2, const ImVec2& uv_min = ImVec2::new2(0, 0), const ImVec2& uv_max = ImVec2::new2(1, 1), let mut col: u32 = IM_COL32_WHITE);
    pub fn AddImage(&mut self, user_texture_id: ImTextureID, p_min: &ImVec2, p_max: &ImVec2, uv_ming: &ImVec2, uv_max: &ImVec2, col: u32) {
        if (col & IM_COL32_A_MASK) == 0
        {
            return;
        }

        let push_texture_id: bool = user_texture_id != self._CmdHeader.TextureId;
        if push_texture_id
        {
            self.PushTextureID(user_texture_id);
        }

        self.PrimReserve(6, 4);
        self.PrimRectUV2(p_min, p_max, uv_min, uv_max, col);

        if push_texture_id
        {
            self.PopTextureID();
        }
    }


    // void  AddImageQuad(ImTextureID user_texture_id, const p1: &ImVec2, const p2: &ImVec2, const p3: &ImVec2, const p4: &ImVec2, const ImVec2& uv1 = ImVec2::new2(0, 0), const ImVec2& uv2 = ImVec2::new2(1, 0), const ImVec2& uv3 = ImVec2::new2(1, 1), const ImVec2& uv4 = ImVec2::new2(0, 1), let mut col: u32 = IM_COL32_WHITE);
    pub fn AddImageQuad(&mut self, user_texture_id: ImTextureID, p1: &ImVec2, p2: &ImVec2, p3: &ImVec2, p4: &ImVec2, uv1: &ImVec2, uv2: &ImVec2, uv3: &ImVec2, uv4: &ImVec2, col: u32) {
        if (col & IM_COL32_A_MASK) == 0
        {
            return;
        }

        let push_texture_id: bool = user_texture_id != self._CmdHeader.TextureId;
        if push_texture_id
        {
            self.PushTextureID(user_texture_id);
        }

        self.PrimReserve(6, 4);
        self.PrimQuadUV(p1, p2, p3, p4, uv1, uv2, uv3, uv4, col);

        if push_texture_id
        {
            self.PopTextureID();
        }
    }


    // void  AddImageRounded(ImTextureID user_texture_id, const p_min: &ImVec2, const p_max: &ImVec2, const uv_min: &ImVec2, const uv_max: &ImVec2, u32 col, rounding: c_float, ImDrawFlags flags = 0);
    pub fn AddImageRounded(&mut self, user_texture_id: ImTextureID, p_min: &ImVec2, p_max: &ImVec2, uv_min: &ImVec2, uv_max: &ImVec2, col: u32, rounding: c_float, mut flags: ImDrawFlags) {
        if (col & IM_COL32_A_MASK) == 0 {
            return;
        }

        flags = self.FixRectCornerFlags(flags);
        if rounding < 0.5f32 || (flags & ImDrawFlags_RoundCornersMask_) == ImDrawFlags_RoundCornersNone {
            self.AddImage(user_texture_id, p_min, p_max, uv_min, uv_max, col);
            return;
        }

        let push_texture_id: bool = user_texture_id != self._CmdHeader.TextureId;
        if push_texture_id {
            self.PushTextureID(user_texture_id);
        }

        let vert_start_idx: c_int = self.VtxBuffer.len() as c_int;
        self.PathRect(p_min, p_max, rounding, flags);
        self.PathFillConvex(col);
        let vert_end_idx: c_int = self.VtxBuffer.len() as c_int;
        self.ShadeVertsLinearUV(this, vert_start_idx, vert_end_idx, p_min, p_max, uv_min, uv_max, true);

        if push_texture_id {
            self.PopTextureID();
        }


    }

    // Stateful path API, add points then finish with PathFillConvex() or PathStroke()
// - Filled shapes must always use clockwise winding order. The anti-aliasing fringe depends on it. Counter-clockwise shapes will have "inward" anti-aliasing.
// inline    void  PathClear()                                                 { _Path.Size = 0; }
    pub fn PathClear(&mut self) { self._Path.clear() }

    // inline    void  PathLineTo(const pos: &ImVec2)                               { _Path.push(pos); }
    pub fn PathLineTo(&mut self, pos: &ImVec2) {
        self._Path.push(pos.clone())
    }

    // inline    void  PathLineToMergeDuplicate(const pos: &ImVec2)
    pub unsafe fn PathLineToMergeDuplicate(&mut self, pos: &ImVec2) {
        if self._Path.len() == 0 || libc::memcmp(&self._Path[self._Path.Size - 1], &pos, 8) != 0 { self._Path.push(pos.clone()); }
    }


    // inline    void  PathFillConvex(u32 col)                                   { AddConvexPolyFilled(_Path.Data, _Path.Size, col); _Path.Size = 0; }
    pub fn PathFillConvex(&mut self, col: u32) {
        self.AddConvexPolyFilled(self._Path.as_ptr(), self._Path.len() as c_int, 0);
        self._Path.clear();
    }


    // inline    void  PathStroke(u32 col, ImDrawFlags flags = 0, c_float thickness = 1f32) { AddPolyline(_Path.Data, _Path.Size, col, flags, thickness); _Path.Size = 0; }
    pub fn PathStroke(&mut self, col: u32, flags: ImDrawFlags, thickness: c_float) {
        self.AddPolyline(self._Path.as_ptr(), self._Path.len() as c_int, col, flags, thickness);
    }


    // void  PathArcTo(const center: &ImVec2, radius: c_float, a_min: c_float, a_max: c_float, c_int num_segments = 0);
    pub fn PathArcTo(&mut self, center: &ImVec2, radius: c_float, a_min: c_float, a_max: c_float, num_segments: c_int) {
        if radius < 0.5f32
        {
            self._Path.push(center.clone());
            return;
        }

        if num_segments > 0
        {
            self._PathArcToN(center, radius, a_min, a_max, num_segments);
            return;
        }

        // Automatic segment count
        if radius <= self._Data.ArcFastRadiusCutof0f32
        {
            let a_is_reverse: bool = a_max < a_min;

            // We are going to use precomputed values for mid samples.
            // Determine first and last sample in lookup table that belong to the arc.
            let a_min_sample_f: c_float =  IM_DRAWLIST_ARCFAST_SAMPLE_MAX * a_min / (IM_PI * 2.00f32);
            let a_max_sample_f: c_float =  IM_DRAWLIST_ARCFAST_SAMPLE_MAX * a_max / (IM_PI * 2.00f32);

            let a_min_sample: c_int = if a_is_reverse { ImFloorSigned(a_min_sample_0f32) } else { ImCeil(a_min_sample_0f32) } as c_int;
            let a_max_sample: c_int = if a_is_reverse { ImCeil(a_max_sample_0f32) } else { ImFloorSigned(a_max_sample_0f32) } as c_int;
            let a_mid_samples: c_int = if a_is_reverse { ImMax(a_min_sample - a_max_sample, 0) } else { ImMax(a_max_sample - a_min_sample, 0) };

            let a_min_segment_angle: c_float =  a_min_sample * IM_PI * 2.0f32 / IM_DRAWLIST_ARCFAST_SAMPLE_MAX;
            let a_max_segment_angle: c_float =  a_max_sample * IM_PI * 2.0f32 / IM_DRAWLIST_ARCFAST_SAMPLE_MAX;
            let a_emit_start: bool = ImAbs2((a_min_segment_angle - a_min)) >= 1e-5f32;
            let a_emit_end: bool = ImAbs2((a_max - a_max_segment_angle)) >= 1e-5f32;

           self._Path.reserve(_Path.Size + (a_mid_samples + 1 + (if a_emit_start { 1 } else { 0 }) + (if a_emit_end { 1 } else { 0 })));
            if a_emit_start
            {
                self._Path.push(ImVec2::new2(center.x + ImCos(a_min) * radius, center.y + ImSin(a_min) * radius));
            }
            if a_mid_samples > 0
            {
                self._PathArcToFastEx(center, radius, a_min_sample, a_max_sample, 0);
            }
            if a_emit_end
            {
                _Path.push(ImVec2::new2(center.x + ImCos(a_max) * radius, center.y + ImSin(a_max) * radius));
            }
        }
        else
        {
            let arc_length: c_float =  ImAbs2(a_max - a_min);
            let circle_segment_count: c_int = self._CalcCircleAutoSegmentCount(radius);
            let arc_segment_count: c_int = ImMax(ImCeil(circle_segment_count * arc_length / (IM_PI * 2.00f32)), (2.0f32 * IM_PI / arc_length));
            self._PathArcToN(center, radius, a_min, a_max, arc_segment_count);
        }
    }


    // void  PathArcToFast(const center: &ImVec2, radius: c_float, a_min_of_12: c_int, a_max_of_12: c_int);                // Use precomputed angles for a 12 steps circle
    pub fn PathArcToFast(&mut self, center: &ImVec2, radius: c_float, a_min_of_12: c_int, a_max_of_12: c_int) {
        if radius < 0.5f32
        {
            self._Path.push(center.clone());
            return;
        }
        self._PathArcToFastEx(center, radius, a_min_of_12 * IM_DRAWLIST_ARCFAST_SAMPLE_MAX / 12, a_max_of_12 * IM_DRAWLIST_ARCFAST_SAMPLE_MAX / 12, 0);
    }


    // void  PathBezierCubicCurveTo(const p2: &ImVec2, const p3: &ImVec2, const p4: &ImVec2, c_int num_segments = 0); // Cubic Bezier (4 control points)
    pub fn PathBezierCubicCurveTo(&mut self, p2: &ImVec2, p3: &ImVec2, p4: &ImVec2, num_segments: c_int) {
        let p1: ImVec2 = _Path.last().unwrap();
        if num_segments == 0
        {
            PathBezierCubicCurveToCasteljau(&_Path, p1.x, p1.y, p2.x, p2.y, p3.x, p3.y, p4.x, p4.y, _Data.CurveTessellationTol, 0); // Auto-tessellated
        }
        else
        {
            let t_step: c_float =  1f32 / num_segments;
            // for (let i_step: c_int = 1; i_step <= num_segments; i_step++)
            for i_step in 1 .. num_segments
            {
                self._Path.push(ImBezierCubicCalc(&p1, p2, p3, p4, t_step * i_step));
            }
        }
    }

    // void  PathBezierQuadraticCurveTo(const p2: &ImVec2, const p3: &ImVec2, c_int num_segments = 0);               // Quadratic Bezier (3 control points)
    pub fn PathBezierQuadraticCurveTo(&mut self, p2: &ImVec2, p3: &ImVec2, num_segments: c_int) {
        let p1: ImVec2 = _Path.last().unwrap();
        if (num_segments == 0)
        {
            PathBezierQuadraticCurveToCasteljau(&_Path, p1.x, p1.y, p2.x, p2.y, p3.x, p3.y, self._Data.CurveTessellationTol, 0);// Auto-tessellated
        }
        else
        {
            let t_step: c_float =  1f32 / num_segments;
            // for (let i_step: c_int = 1; i_step <= num_segments; i_step++)
            for i_step in 1 .. num_segments
            {
                self._Path.push(ImBezierQuadraticCalc(&p1, p2, p3, t_step * i_step));
            }
        }
    }

    // void  PathRect(const rect_min: &ImVec2, const rect_max: &ImVec2, c_float rounding = 0f32, ImDrawFlags flags = 0);
    pub fn PathRect(&mut self, rect_min: &ImVec2, rect_max: &ImVec2, mut rounding: c_float, mut flags: ImDrawFlags) {
        flags = self.FixRectCornerFlags(flags);
        rounding = ImMin(rounding, ImFabs(b.x - a.x) * ( if ((flags & ImDrawFlags_RoundCornersTop)  == ImDrawFlags_RoundCornersTop)  || ((flags & ImDrawFlags_RoundCornersBottom) == ImDrawFlags_RoundCornersBottom) { 0.5f32 } else { 1f32 } ) - 1f32);
        rounding = ImMin(rounding, ImFabs(b.y - a.y) * ( if ((flags & ImDrawFlags_RoundCornersLeft) == ImDrawFlags_RoundCornersLeft) || ((flags & ImDrawFlags_RoundCornersRight)  == ImDrawFlags_RoundCornersRight) { 0.5f32 } else { 1f32 } ) - 1f32);

        if rounding < 0.5f32 || (flags & ImDrawFlags_RoundCornersMask_) == ImDrawFlags_RoundCornersNone
        {
            self.PathLineTo(a);
            self.PathLineTo(&ImVec2::new2(b.x, a.y));
            self.PathLineTo(b);
            self.PathLineTo(&ImVec2::new2(a.x, b.y));
        }
        else
        {
            let rounding_tl: c_float = if flags & ImDrawFlags_RoundCornersTopLeft { rounding } else { 0f32 };
            let rounding_tr: c_float =  if flags & ImDrawFlags_RoundCornersTopRight { rounding } else { 0f32 };
            let rounding_br: c_float =  if flags & ImDrawFlags_RoundCornersBottomRight { rounding } else { 0f32 };
            let rounding_bl: c_float =  if flags & ImDrawFlags_RoundCornersBottomLeft { rounding } else { 0f32 };
            PathArcToFast(ImVec2::new2(a.x + rounding_tl, a.y + rounding_tl), rounding_tl, 6, 9);
            PathArcToFast(ImVec2::new2(b.x - rounding_tr, a.y + rounding_tr), rounding_tr, 9, 12);
            PathArcToFast(ImVec2::new2(b.x - rounding_br, b.y - rounding_br), rounding_br, 0, 3);
            PathArcToFast(ImVec2::new2(a.x + rounding_bl, b.y - rounding_bl), rounding_bl, 3, 6);
        }
    }

    // Advanced
// void  AddCallback(ImDrawCallback callback, void* callback_data);  // Your rendering function must check for 'UserCallback' in ImDrawCmd and call the function instead of rendering triangles.
    pub fn AddCallback(&mut self, callback: ImDrawCallback, callback_data: *mut c_void) {
        // IM_ASSERT_PARANOID(CmdBuffer.Size > 0);
        let mut curr_cmd = &mut self.CmdBuffer[self.CmdBuffer.len() - 1];
        // IM_ASSERT(curr_cmd.UserCallback == NULL);
        if curr_cmd.ElemCount != 0
        {
            self.AddDrawCmd();
            curr_cmd = &mut self.CmdBuffer[self.CmdBuffer.len() - 1];
        }
        curr_cmd.UserCallback = callback;
        curr_cmd.UserCallbackData = callback_data;

        self.AddDrawCmd(); // Force a new command after us (see comment below)
    }

    // void  AddDrawCmd();                                               // This is useful if you need to forcefully create a new draw call (to allow for dependent rendering / blending). Otherwise primitives are merged into the same draw-call as much as possible
    pub fn AddDrawCmd(&mut self) {
        let mut draw_cmd = ImDrawCmd::new();
        draw_cmd.ClipRect = self._CmdHeader.ClipRect;    // Same as calling ImDrawCmd_HeaderCopy()
        draw_cmd.TextureId = self._CmdHeader.TextureId;
        draw_cmd.VtxOffset = self._CmdHeader.VtxOffset;
        draw_cmd.IdxOffset = self.IdxBuffer.len();

        // IM_ASSERT(draw_cmd.ClipRect.x <= draw_cmd.ClipRect.z && draw_cmd.ClipRect.y <= draw_cmd.ClipRect.w);
        self.CmdBuffer.push(draw_cmd);
    }

    // ImDrawList* CloneOutput() const;                                  // Create a clone of the CmdBuffer/IdxBuffer/VtxBuffer.
    pub fn CloneOutpost(&mut self) -> *mut ImDrawList {
        let mut dst = ImDrawList::new(self._Data);
        dst.CmdBuffer = CmdBuffer;
        dst.IdxBuffer = IdxBuffer;
        dst.VtxBuffer = VtxBuffer;
        dst.Flags = Flags;
        return &mut dst;
    }

    // Advanced: Channels
// - Use to split render into layers. By switching channels to can render out-of-order (e.g. submit FG primitives before BG primitives)
// - Use to minimize draw calls (e.g. if going back-and-forth between multiple clipping rectangles, prefer to append into separate channels then merge at the end)
// - FIXME-OBSOLETE: This API shouldn't have been in ImDrawList in the first place!
//   Prefer using your own persistent instance of ImDrawListSplitter as you can stack them.
//   Using the ImDrawList::ChannelsXXXX you cannot stack a split over another.
// inline void     ChannelsSplit(count: c_int)    { _Splitter.Split(this, count); }
    pub fn ChannelsSplit(&mut self, count: c_int) {
        self._Splitter.Split(self, count)
    }


    // inline void     ChannelsMerge()             { _Splitter.Merge(this); }
    pub fn ChannelsMerge(&mut self) {
        self._Splitter.Merge(self)
    }

    // inline void     ChannelsSetCurrent(n: c_int)   { _Splitter.SetCurrentChannel(this, n); }
    pub fn ChannelsSetCurrent(&mut self, n: c_int) {
        self._Splitter.SetCurrentChannel(self, n)
    }

    // Advanced: Primitives allocations
// - We render triangles (three vertices)
// - All primitives needs to be reserved via PrimReserve() beforehand.
// void  PrimReserve(idx_count: c_int, vtx_count: c_int);
    pub fn PrimReserve(&mut self, idx_count: c_int, vtx_count: c_int) {
        // Large mesh support (when enabled)
        // IM_ASSERT_PARANOID(idx_count >= 0 && vtx_count >= 0);
        // todo: find the missing (x) for sizeof
        // if (sizeof == 2 && (_VtxCurrentIdx + vtx_count >= (1 << 16)) && (Flags & ImDrawListFlags_AllowVtxOffset))
        // {
        //     // FIXME: In theory we should be testing that vtx_count <64k here.
        //     // In practice, RenderText() relies on reserving ahead for a worst case scenario so it is currently useful for us
        //     // to not make that check until we rework the text functions to handle clipping and large horizontal lines better.
        //     _CmdHeader.VtxOffset = VtxBuffer.Size;
        //     _OnChangedVtxOffset();
        // }

        let mut draw_cmd: *mut ImDrawCmd = &mut self.CmdBuffer[self.CmdBuffer.len() - 1];
        // TODO: find missing value
        // += idx_count;

        let vtx_buffer_old_size: c_int = self.VtxBuffer.len() as c_int;
        self.VtxBuffer.reserve((vtx_buffer_old_size + vtx_count) as usize);
        self._VtxWritePtr = &mut self.VtxBuffer + vtx_buffer_old_size;

        let idx_buffer_old_size: c_int = self.IdxBuffer.len() as c_int;
        self.IdxBuffer.reserve((idx_buffer_old_size + idx_count) as usize);
        self._IdxWritePtr = &mut self.IdxBuffer + idx_buffer_old_size;
    }

    // void  PrimUnreserve(idx_count: c_int, vtx_count: c_int);
    pub fn PrimUnreserve(&mut self, idx_count: c_int, vtx_count: c_int) {
        // IM_ASSERT_PARANOID(idx_count >= 0 && vtx_count >= 0);

        let mut draw_cmd: *mut ImDrawCmd = &mut self.CmdBuffer[self.CmdBuffer.len() - 1];
        // todo: figure out what's missing
        //-= idx_count;
        self.VtxBuffer.shrink(self.VtxBuffer.len() - vtx_count);
        self.IdxBuffer.shrink(self.IdxBuffer.len() - idx_count);
    }

    // void  PrImRect::new(const a: &ImVec2, const b: &ImVec2, u32 col);      // Axis aligned rectangle (composed of two triangles)
    // pub fn PrImRect::new( & mut self , a: & ImVec2, b: & ImVec2, col: u32) {
    // todo ! ()
    // }


    // void  PrimRectUV(const a: &ImVec2, const b: &ImVec2, const uv_a: &ImVec2, const uv_b: &ImVec2, u32 col);
    pub fn PrimRectUV(&mut self, a: &ImVec2, b: &ImVec2, uv_a: &imVec2, uv_b: &ImVec2, uv_c: &ImVec2, col: u32) {
        // ImVec2 b(c.x, a.y), d(a.x, c.y), uv_b(uv_c.x, uv_a.y), uv_d(uv_a.x, uv_c.y);
        let b = ImVec2::new2(c.x, a.y);
        let d = ImVec2::new2(a.x, c.y);
        let uv_b = ImVec2::new2(uv_c.x, uv_a.y);
        let uv_d = ImVec2::new2(uv_a.x, uv_c.y);

        let idx = self._VtxCurrentIdx;
        self._IdxWritePtr[0] = idx;
        self._IdxWritePtr[1] = (idx + 1);
        self._IdxWritePtr[2] = (idx + 2);
        self._IdxWritePtr[3] = idx;
        self._IdxWritePtr[4] = (idx + 2);
        self._IdxWritePtr[5] = (idx + 3);
        self._VtxWritePtr[0].pos = a;
        self._VtxWritePtr[0].uv = uv_a;
        self._VtxWritePtr[0].col = col;
        self._VtxWritePtr[1].pos = b;
        self._VtxWritePtr[1].uv = uv_b;
        self._VtxWritePtr[1].col = col;
        self._VtxWritePtr[2].pos = c;
        self._VtxWritePtr[2].uv = uv_c;
        self._VtxWritePtr[2].col = col;
        self._VtxWritePtr[3].pos = d;
        self._VtxWritePtr[3].uv = uv_d;
        self._VtxWritePtr[3].col = col;
        self._VtxWritePtr += 4;
        self._VtxCurrentIdx += 4;
        self._IdxWritePtr += 6;
    }


    // void  PrimQuadUV(const a: &ImVec2, const b: &ImVec2, const c: &ImVec2, const d: &ImVec2, const uv_a: &ImVec2, const uv_b: &ImVec2, const uv_c: &ImVec2, const uv_d: &ImVec2, u32 col);
    pub fn PrimQuadUV(&mut self, a: &ImVec2, b: &ImVec2, c: &ImVec2, d: &ImVec2, uv_a: &ImVec2, uv_b: &ImVec2, uv_c: &ImVec2, uv_d: &ImVec2, col: u32) {
        let idx = self._VtxCurrentIdx;
        self._IdxWritePtr[0] = idx;
        self._IdxWritePtr[1] = (idx + 1);
        self._IdxWritePtr[2] = (idx + 2);
        self._IdxWritePtr[3] = idx;
        self._IdxWritePtr[4] = (idx + 2);
        self._IdxWritePtr[5] = (idx + 3);
        self._VtxWritePtr[0].pos = a;
        self._VtxWritePtr[0].uv = uv_a;
        self._VtxWritePtr[0].col = col;
        self._VtxWritePtr[1].pos = b;
        self._VtxWritePtr[1].uv = uv_b;
        self._VtxWritePtr[1].col = col;
        self._VtxWritePtr[2].pos = c;
        self._VtxWritePtr[2].uv = uv_c;
        self._VtxWritePtr[2].col = col;
        self._VtxWritePtr[3].pos = d;
        self._VtxWritePtr[3].uv = uv_d;
        self._VtxWritePtr[3].col = col;
        self._VtxWritePtr += 4;
        self._VtxCurrentIdx += 4;
        self._IdxWritePtr += 6;
    }

    // inline    void  PrimWriteVtx(const pos: &ImVec2, const uv: &ImVec2, u32 col)    { self._VtxWritePtr.pos = pos; self._VtxWritePtr.uv = uv; self._VtxWritePtr.col = col; self._VtxWritePtr+= 1;self._VtxCurrentIdx+= 1; }
    pub fn PrimWriteVtx(&mut self, pos: &ImVec2, uv: &ImVec2, col: u32) {
        self._VtxWritePtr.pos = pos.clone();
        self._VtxWritePtr.uv = uv.clone();
        self._VtxWritePtr.col = col;
        self._VtxWritePtr += 1;
        self._VtxCurrentIdx += 1;
    }


    // inline    void  PrimWriteIdx(ImDrawIdx idx)                                     { *self._IdxWritePtr = idx; self._IdxWritePtr+= 1; }
    pub unsafe fn PrimWriteIdx(&mut self, idx: ImDrawIdx) {
        *self._IdxWritePtr = idx;
        self._IdxWritePtr += 1;
    }


    // inline    void  PrimVtx(const pos: &ImVec2, const uv: &ImVec2, u32 col)         { PrimWriteIdx(self._VtxCurrentIdx); PrimWriteVtx(pos, uv, col); } // Write vertex with unique index
    pub unsafe fn PrimVtx(&mut self, pos: &ImVec2, uv: &ImVec2, col: u32) {
        self.PrimWriteIdx(self._VtxCurrentIdx as ImDrawIdx);
        self.PrimWriteVtx(pos, uv, col);
    }

// #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
// inline    void  AddBezierCurve(const p1: &ImVec2, const p2: &ImVec2, const p3: &ImVec2, const p4: &ImVec2, u32 col, thickness: c_float, c_int num_segments = 0) { AddBezierCubic(p1, p2, p3, p4, col, thickness, num_segments); } // OBSOLETED in 1.80 (Jan 2021)
// pub fn AddBezierCurve(&mut self, p1: &ImVec2, p2: &ImVec2, p3: &ImVec2, p4: &ImVec2, col: u32, thickness: c_float, num_segments: c_int) {
//     self.AddBezierCubic(p1,p2,p3,p4,col,thickness,num_segments)
// }

// inline    void  PathBezierCurveTo(const p2: &ImVec2, const p3: &ImVec2, const p4: &ImVec2, c_int num_segments = 0) { PathBezierCubicCurveTo(p2, p3, p4, num_segments); } // OBSOLETED in 1.80 (Jan 2021)

// #endif

    // [Internal helpers]
// void  _ResetForNewFrame(); 
    pub unsafe fn _ResetForNewFrame(&mut self) {
          // Verify that the ImDrawCmd fields we want to memcmp() are contiguous in memory.
        IM_STATIC_ASSERT(IM_OFFSETOF(ImDrawCmd, ClipRect) == 0);
        IM_STATIC_ASSERT(IM_OFFSETOF(ImDrawCmd, TextureId) == sizeof(ImVec4));
        IM_STATIC_ASSERT(IM_OFFSETOF(ImDrawCmd, VtxOffset) == sizeof(ImVec4) + sizeof(ImTextureID));
        if self._Splitter._Count > 1
        {
            self._Splitter.Merge(this);
        }

        self.CmdBuffer.clear();
        self.IdxBuffer.clear();
        self.VtxBuffer.clear();
        self.Flags = self._Data.InitialFlags;
        // libc::memset(&self._CmdHeader, 0, sizeof(self._CmdHeader));
        self._CmdHeader = ImDrawCmdHeader::default();
        self._VtxCurrentIdx = 0;
        self._VtxWritePtr= null_mut();
        self._IdxWritePtr= null_mut();
        self._ClipRectStack.clear();
        self._TextureIdStack.clear();
        self._Path.clear();
        self._Splitter.Clear();
        self.CmdBuffer.push(ImDrawCmd::default());
        self._FringeScale = 1f32;
    }

    // void  _ClearFreeMemory();
    pub fn _ClearFreeMemory(&mut self) {
        self.CmdBuffer.clear();
        self.IdxBuffer.clear();
        self.VtxBuffer.clear();
        self.Flags = ImDrawListFlags_None;
        self._VtxCurrentIdx = 0;
        self._VtxWritePtr = null_mut();
        self._IdxWritePtr = null_mut();
        self._ClipRectStack.clear();
        self._TextureIdStack.clear();
        self._Path.clear();
        self._Splitter.ClearFreeMemory();
    }

    // void  _PopUnusedDrawCmd();
    pub fn _PopUnusedDrawCmd(&mut self) {
        if self.CmdBuffer.len() == 0 {
            return;
        }
        let mut curr_cmd = &mut self.CmdBuffer[self.CmdBuffer.len() - 1];
        if curr_cmd.ElemCount == 0 && curr_cmd.UserCallback == null_mut() {
            self.CmdBuffer.pop_back();
        }
    }

    // void  _TryMergeDrawCmds();
    pub fn _TryMergeDrawCmds(&mut self) {
        // IM_ASSERT_PARANOID(CmdBuffer.Size > 0);
        let mut curr_cmd = self.CmdBuffer[CmdBuffer.len() - 1];
        let mut prev_cmd = curr_cmd - 1;
        if ImDrawCmd_HeaderCompare(curr_cmd, prev_cmd) == 0 &&
            ImDrawCmd_AreSequentialIdxOffset(prev_cmd, curr_cmd) &&
            curr_cmd.UserCallback == null_mut() &&
            prev_cmd.UserCallback == null_mut()
        {
            prev_cmd.ElemCount += curr_cmd.ElemCount;
            self.CmdBuffer.pop_back();
    }
    }

    // void  _OnChangedClipRect();
    pub unsafe fn _OnChangedClipRect(&mut self) {
        // If current command is used with different settings we need to add a new command
        // IM_ASSERT_PARANOID(CmdBuffer.Size > 0);
        let curr_cmd = &mut self.CmdBuffer[self.CmdBuffer.len() - 1];
        if curr_cmd.ElemCount != 0 && libc::memcmp(&curr_cmd.ClipRect, &self._CmdHeader.ClipRect, mem::size_of::<ImVec4>()) != 0 {
            self.AddDrawCmd();
            return;
        }
        // IM_ASSERT(curr_cmd.UserCallback == NULL);

        // Try to merge with previous command if it matches, else use current command
        let prev_cmd = curr_cmd - 1;
        if curr_cmd.ElemCount == 0 && self.CmdBuffer.len() > 1 && ImDrawCmd_HeaderCompare(&self._CmdHeader, prev_cmd) == 0 && ImDrawCmd_AreSequentialIdxOffset(prev_cmd, curr_cmd) && prev_cmd.UserCallback == null_mut() {
            self.CmdBuffer.pop_back();
            return;
        }

        curr_cmd.ClipRect = self._CmdHeader.ClipRect;
    }

    // void  _OnChangedTextureID();
    pub fn _OnChangedTextureID(&mut self) {
        // If current command is used with different settings we need to add a new command
        // IM_ASSERT_PARANOID(CmdBuffer.Size > 0);
        let mut curr_cmd: *mut ImDrawCmd = &mut self.CmdBuffer[self.CmdBuffer.len() - 1];
        if curr_cmd.ElemCount != 0 && curr_cmd.TextureId != self._CmdHeader.TextureId
        {
            self.AddDrawCmd();
            return;
        }
        // IM_ASSERT(curr_cmd.UserCallback == NULL);

        // Try to merge with previous command if it matches, else use current command
        let prev_cmd = curr_cmd - 1;
        // if (curr_cmd.ElemCount == 0 && self.CmdBuffer.len() > 1 && ImDrawCmd_HeaderCompare(&self._CmdHeader, prev_cmd) == 0 && ImDrawCmd_AreSequentialIdxOffset(prev_cmd, curr_cmd) &&  == null_mut())
        // {
        //     self.CmdBuffer.pop_back();
        //     return;
        // }

        curr_cmd.TextureId = self._CmdHeader.TextureId;
    }

    // void  _OnChangedVtxOffset();
    pub fn _OnChangedVtxOffset(&mut self) {
        // We don't need to compare curr_cmd.VtxOffset != _CmdHeader.VtxOffset because we know it'll be different at the time we call this.
        self._VtxCurrentIdx = 0;
        // IM_ASSERT_PARANOID(CmdBuffer.Size > 0);
        let curr_cmd = &mut self.CmdBuffer[self.CmdBuffer.len() - 1];
        //IM_ASSERT(curr_cmd.VtxOffset != _CmdHeader.VtxOffset); // See #3349
        // if ( != 0)
        // {
        //     AddDrawCmd();
        //     return;
        // }
        //     // IM_ASSERT(curr_cmd.UserCallback == NULL);
        //     = _CmdHeader.VtxOffset;
    }

    // c_int   _CalcCircleAutoSegmentCount(radius: c_float) const;
    pub fn _CalcCircleAutoSegmentCount(&mut self, radius: f32) -> c_int {
        // Automatic segment count
        let radius_idx: c_int = (radius + 0.9999990f32) as c_int; // ceil to never reduce accuracy
        if radius_idx < (self._Data.CircleSegmentCounts.len()) as c_int
        {
            return self._Data.CircleSegmentCounts[radius_idx];
        } // Use cached value
        // TODO: check what the original function's return value was supposed to be
        return 0
    }

    // void  _PathArcToFastEx(const center: &ImVec2, radius: c_float, a_min_sample: c_int, a_max_sample: c_int, a_step: c_int);
    pub fn _PathArcToFastEx(&mut self, center: &ImVec2, radius: c_float, a_min_sample: c_int, a_max_sample: c_int, mut a_step: c_int) {
        if (radius < 0.5f32)
        {
            _Path.push(center);
            return;
        }

        // Calculate arc auto segment step size
        if a_step <= 0
        {
            a_step = IM_DRAWLIST_ARCFAST_SAMPLE_MAX / _CalcCircleAutoSegmentCount(radius);
        }

        // Make sure we never do steps larger than one quarter of the circle
        a_step = ImClamp(a_step, 1, IM_DRAWLIST_ARCFAST_TABLE_SIZE / 4);

        let sample_range: c_int = ImAbs(a_max_sample - a_min_sample);
        let a_next_step: c_int = a_step;

        let mut samples: c_int = sample_range + 1;
        let mut extra_max_sample: bool =  false;
        if (a_step > 1)
        {
            samples            = sample_range / a_step + 1;
            let overstep: c_int = sample_range % a_step;

            if (overstep > 0)
            {
                extra_max_sample = true;
                samples+= 1;

                // When we have overstep to avoid awkwardly looking one long line and one tiny one at the end,
                // distribute first step range evenly between them by reducing first step size.
                if sample_range > 0
                {
                    a_step -= (a_step - overstep) / 2;
                }
            }
        }

        _Path.resize(_Path.Size + samples);
        ImVec2* out_ptr = _Path.Data + (_Path.Size - samples);

        let mut sample_index: c_int = a_min_sample;
        if (sample_index < 0 || sample_index >= IM_DRAWLIST_ARCFAST_SAMPLE_MAX)
        {
            sample_index = sample_index % IM_DRAWLIST_ARCFAST_SAMPLE_MAX;
            if sample_index < 0
            {
                sample_index += IM_DRAWLIST_ARCFAST_SAMPLE_MAX;
            }
        }

        if (a_max_sample >= a_min_sample)
        {
            // for (let a: c_int = a_min_sample; a <= a_max_sample; a += a_step, sample_index += a_step, a_step = a_next_step)
            let mut a = a_min_sample;
            while a <= max_sample
            {
                // a_step is clamped to IM_DRAWLIST_ARCFAST_SAMPLE_MAX, so we have guaranteed that it will not wrap over range twice or more
                if sample_index >= IM_DRAWLIST_ARCFAST_SAMPLE_MAX
                {
                    sample_index -= IM_DRAWLIST_ARCFAST_SAMPLE_MAX;
                }

                let s: ImVec2 = self._Data.ArcFastVtx[sample_index];
                // = center.x + s.x * radius;
                // = center.y + s.y * radius;
                out_ptr+= 1;
                a += a_step;
                sample_index += a_step;
                a_step = a_next_step;
            }
        }
        else
        {
            // for (let a: c_int = a_min_sample; a >= a_max_sample; a -= a_step, sample_index -= a_step, a_step = a_next_step)
           let mut a = a_min_sample;
            while a >= a_max_sample
            {
                // a_step is clamped to IM_DRAWLIST_ARCFAST_SAMPLE_MAX, so we have guaranteed that it will not wrap over range twice or more
                if sample_index < 0
                {
                    sample_index += IM_DRAWLIST_ARCFAST_SAMPLE_MAX;
                }

                let s: ImVec2 = _Data.ArcFastVtx[sample_index];
                // = center.x + s.x * radius;
                // = center.y + s.y * radius;
                out_ptr+= 1;
                a -= a_step;
                sample_index -= a_step;
                a_step = a_next_step;
            }
        }

        if extra_max_sample
        {
            let mut normalized_max_sample: c_int = a_max_sample % IM_DRAWLIST_ARCFAST_SAMPLE_MAX;
            if normalized_max_sample < 0
            {
                normalized_max_sample += IM_DRAWLIST_ARCFAST_SAMPLE_MAX;
            }

            let s: ImVec2 = _Data.ArcFastVtx[normalized_max_sample];
            // = center.x + s.x * radius;
            // = center.y + s.y * radius;
            out_ptr+= 1;
        }

    // IM_ASSERT_PARANOID(_Path.Data + _Path.Size == out_ptr);
    }

    // void  _PathArcToN(const center: &ImVec2, radius: c_float, a_min: c_float, a_max: c_float, num_segments: c_int);
    pub fn _PathArcToN(&mut self, center: &ImVec2, radius: c_float, a_min: c_float, a_max: c_float, num_segments: c_int) {
        if radius < 0.5f32
        {
            self._Path.push(center.clone());
            return;
        }

        // Note that we are adding a point at both a_min and a_max.
        // If you are trying to draw a full closed circle you don't want the overlapping points!
        self._Path.reserve(self._Path.Size + (num_segments + 1));
        // for (let i: c_int = 0; i <= num_segments; i++)
        for i in 0 .. num_segments
        {
            let a: c_float =  a_min + (i / num_segments) * (a_max - a_min);
            self._Path.push(ImVec2::new2(center.x + ImCos(a) * radius, center.y + ImSin(a) * radius));
        }
    }




    // // Fully unrolled with inline call to keep our debug builds decently fast.
    // c_void ImDrawList::PrImRect::new(const a: &ImVec2, const c: &ImVec2, u32 col)
    // {
    //     ImVec2 b(c.x, a.y), d(a.x, c.y), uv(_Data.TexUvWhitePixel);
    //     ImDrawIdx idx =self._VtxCurrentIdx;
    //     self._IdxWritePtr[0] = idx; self._IdxWritePtr[1] = (idx+1); self._IdxWritePtr[2] = (idx+2);
    //     self._IdxWritePtr[3] = idx; self._IdxWritePtr[4] = (idx+2); self._IdxWritePtr[5] = (idx+3);
    //     self._VtxWritePtr[0].pos = a; self._VtxWritePtr[0].uv = uv; self._VtxWritePtr[0].col = col;
    //     self._VtxWritePtr[1].pos = b; self._VtxWritePtr[1].uv = uv; self._VtxWritePtr[1].col = col;
    //     self._VtxWritePtr[2].pos = c; self._VtxWritePtr[2].uv = uv; self._VtxWritePtr[2].col = col;
    //     self._VtxWritePtr[3].pos = d; self._VtxWritePtr[3].uv = uv; self._VtxWritePtr[3].col = col;
    //     self._VtxWritePtr += 4;
    //    self._VtxCurrentIdx += 4;
    //     self._IdxWritePtr += 6;
    // }
}
