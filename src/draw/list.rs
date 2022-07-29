use crate::context::Context;
use crate::draw::cmd::{CmdHeader, DrawCmd};
use crate::draw::draw_defines::DrawFlags;
use crate::draw::list_shared_data::DrawListSharedData;
use crate::draw::list_splitter::DrawListSplitter;
use crate::draw::vertex::DrawVertex;
use crate::font::Font;
use crate::rect::Rect;
use crate::texture::TextureId;
use crate::types::{DrawIndex, Id32, INVALID_ID};
use crate::utils::set_hash_set;
use crate::vectors::two_d::Vector2D;
use crate::vectors::Vector4D;
use crate::viewport::Viewport;
use std::collections::HashSet;
use std::f32::consts::PI;
use std::ffi::c_void;
use std::mem::size_of;
use std::os::raw::c_char;

/// Draw command list
/// This is the low-level list of polygons that ImGui:: functions are filling. At the end of the frame,
/// all command lists are passed to your ImGuiIO::RenderDrawListFn function for rendering.
/// Each dear imgui window contains its own ImDrawList. You can use ImGui::GetWindowDrawList() to
/// access the current window draw list and draw custom primitives.
/// You can interleave normal ImGui:: calls and adding primitives to the current draw list.
/// In single viewport mode, top-left is == get_main_viewport()->pos (generally 0,0), bottom-right is == get_main_viewport()->pos+size (generally io.display_size).
/// You are totally free to apply whatever transformation matrix to want to the data (depending on the use of the transformation you may want to apply it to clip_rect as well!)
/// Important: Primitives are always added to the list and not culled (culling is done at higher-level by ImGui:: functions), if you use this API a lot consider coarse culling your drawn objects.
#[derive(Default, Debug, Clone)]
pub struct DrawList {
    // This is what you have to render
    // ImVector<ImDrawCmd>     cmd_buffer;          // Draw commands. Typically 1 command = 1 GPU draw call, unless the command is a callback.
    pub cmd_buffer: Vec<DrawCmd>,
    // ImVector<ImDrawIdx>     idx_buffer;          // index buffer. Each command consume ImDrawCmd::elem_count of those
    pub idx_buffer: Vec<DrawIndex>,
    // ImVector<ImDrawVert>    vtx_buffer;          // Vertex buffer.
    pub vtx_buffer: Vec<DrawVertex>,
    // ImDrawListFlags         flags;              // flags, you may poke into these to adjust anti-aliasing settings per-primitive.
    pub flags: HashSet<DrawListFlags>,
    // [Internal, used while building lists]
    // unsigned pub _VtxCurrentIdx: i32,   // [Internal] generally == vtx_buffer.size unless we are past 64K vertices, in which case this gets reset to 0.
    // pub _VtxCurrentIdx: u32,
    // const ImDrawListSharedData* _Data;          // Pointer to shared draw data (you can use ImGui::GetDrawListSharedData() to get the one from current ImGui context)
    pub data: DrawListSharedData,
    // const char*             _OwnerName;         // Pointer to owner window's name for debugging
    pub owner_name: String,
    // ImDrawVert*             _VtxWritePtr;       // [Internal] point within vtx_buffer.data after each add command (to avoid using the ImVector<> operators too much)
    // pub _VxWritePtr: *mut ImDrawVert,
    // ImDrawIdx*              _IdxWritePtr;       // [Internal] point within idx_buffer.data after each add command (to avoid using the ImVector<> operators too much)
    // pub _IdxWritePtr: *mut ImDrawIdx,
    // ImVector<Vector4D>        _clip_rect_stack;     // [Internal]
    pub clip_rect_stack: Vec<Rect>,
    // ImVector<ImTextureID>   _texture_id_stack;    // [Internal]
    pub _texture_id_stack: Vec<TextureId>,
    // ImVector<Vector2D>        _path;              // [Internal] current path building
    pub path: Vec<Vector2D>,
    // ImDrawCmdHeader         _cmd_header;         // [Internal] template of active commands. Fields should match those of cmd_buffer.back().
    pub _cmd_header: CmdHeader,
    // ImDrawListSplitter      _splitter;          // [Internal] for channels api (note: prefer using your own persistent instance of ImDrawListSplitter!)
    pub splitter: DrawListSplitter,
    // pub _fringe_scale: f32,      // [Internal] anti-alias fringe is scaled by this value, this helps to keep things sharp while zooming at vertex buffer content
    pub _fringe_scale: f32,
}

impl DrawList {
    // If you want to create ImDrawList instances, pass them ImGui::GetDrawListSharedData() or create and use your own ImDrawListSharedData (so you can use ImDrawList without ImGui)
    // ImDrawList(const ImDrawListSharedData* shared_data) { memset(this, 0, sizeof(*this)); _Data = shared_data; }
    pub fn new(shared_data: &mut DrawListSharedData) -> Self {
        let mut out = Self {
            ..Default::default()
        };
        out.data = shared_data.clone();
        out
    }
    // ~ImDrawList() { _ClearFreeMemory(); }
    //  void  push_clip_rect(const Vector2D& clip_rect_min, const Vector2D& clip_rect_max, bool intersect_with_current_clip_rect = false);  // Render-level scissoring. This is passed down to your render function but not used for CPU-side coarse clipping. Prefer using higher-level ImGui::push_clip_rect() to affect logic (hit-testing and widget culling)
    pub fn push_clip_rect(
        &mut self,
        clip_rect_min: &Vector2D,
        clip_rect_max: &Vector2D,
        intersect_with_current_clip_rect: bool,
    ) {
        // Vector4D cr(cr_min.x, cr_min.y, cr_max.x, cr_max.y);
        //     if (intersect_with_current_clip_rect)
        //     {
        //         Vector4D current = _CmdHeader.clip_rect;
        //         if (cr.x < current.x) cr.x = current.x;
        //         if (cr.y < current.y) cr.y = current.y;
        //         if (cr.z > current.z) cr.z = current.z;
        //         if (cr.w > current.w) cr.w = current.w;
        //     }
        //     cr.z = ImMax(cr.x, cr.z);
        //     cr.w = ImMax(cr.y, cr.w);
        //
        //     _ClipRectStack.push_back(cr);
        //     _CmdHeader.clip_rect = cr;
        //     _OnChangedClipRect();
    }
    //  void  push_clip_rect_full_screen();
    pub fn push_clip_rect_full_screen(&mut self) {
        // PushClipRect(Vector2D::new(_Data.clip_rect_full_screen.x, _Data.clip_rect_full_screen.y), Vector2D::new(_Data.clip_rect_full_screen.z, _Data.clip_rect_full_screen.w));
    }
    //  void  pop_clip_rect();
    pub fn pop_clip_rect(&mut self) {
        // _ClipRectStack.pop_back();
        //     _CmdHeader.clip_rect = (_ClipRectStack.size == 0) ? _Data.clip_rect_full_screen : _ClipRectStack.data[_ClipRectStack.size - 1];
        //     _OnChangedClipRect();
    }
    //  void  push_texture_id(ImTextureID texture_id);
    pub fn push_texture_id(&mut self, texture_id: TextureId) {
        //     _TextureIdStack.push_back(texture_id);
        //     _CmdHeader.TextureId = texture_id;
        //     _OnChangedTextureID();
    }
    //  void  pop_texture_id();
    pub fn pop_texture_id(&mut self) {
        //     _TextureIdStack.pop_back();
        //     _CmdHeader.TextureId = (_TextureIdStack.size == 0) ? (ImTextureID)None : _TextureIdStack.data[_TextureIdStack.size - 1];
        //     _OnChangedTextureID();
    }
    // inline Vector2D   get_clip_rect_min() const { const Vector4D& cr = _clip_rect_stack.back(); return Vector2D(cr.x, cr.y); }
    pub fn get_clip_rect_min(&self) -> Vector2D {
        let cr = self.clip_rect_stack.last().unwrap();
        Vector2D::new(cr.x, cr.y)
    }
    // inline Vector2D   get_clip_rect_max() const { const Vector4D& cr = _clip_rect_stack.back(); return Vector2D(cr.z, cr.w); }
    pub fn get_clip_rect_max(&self) -> Result<Vector2D, &'static str> {
        let cr = self.clip_rect_stack.last().unwrap();
        Ok(Vector2D::new(cr.z, cr.w))
    }
    // Primitives
    // - Filled shapes must always use clockwise winding order. The anti-aliasing fringe depends on it. Counter-clockwise shapes will have "inward" anti-aliasing.
    // - For rectangular primitives, "p_min" and "p_max" represent the upper-left and lower-right corners.
    // - For circle primitives, use "num_segments == 0" to automatically calculate tessellation (preferred).
    //   In older versions (until Dear ImGui 1.77) the add_circle functions defaulted to num_segments == 12.
    //   In future versions we will use textures to provide cheaper and higher-quality circles.
    //   Use add_ngon() and AddNgonFilled() functions if you need to guaranteed a specific number of sides.
    //  void  add_line(const Vector2D& p1, const Vector2D& p2, ImU32 col, float thickness = 1.0);
    pub fn add_line(&mut self, p1: &Vector2D, p2: &Vector2D, col: u32, thickness: f32) {
        // if ((col & IM_COL32_A_MASK) == 0)
        //         return;
        //     PathLineTo(p1 + Vector2D::new(0.5, 0.5));
        //     PathLineTo(p2 + Vector2D::new(0.5, 0.5));
        //     PathStroke(col, 0, thickness);
    }
    //  void  add_rect(const Vector2D& p_min, const Vector2D& p_max, ImU32 col, float rounding = 0.0, ImDrawFlags flags = 0, float thickness = 1.0);   // a: upper-left, b: lower-right (== upper-left + size)
    pub fn add_rect(
        &mut self,
        p_min: &Vector2D,
        p_max: Vector2D,
        col: u32,
        rounding: f32,
        flags: Option<&HashSet<DrawFlags>>,
        thickness: f32,
    ) {
        //  if ((col & IM_COL32_A_MASK) == 0)
        //         return;
        //     if (Flags & DrawListFlags::AntiAliasedLines)
        //         PathRect(p_min + Vector2D::new(0.50, 0.50), p_max - Vector2D::new(0.50, 0.50), rounding, flags);
        //     else
        //         PathRect(p_min + Vector2D::new(0.50, 0.50), p_max - Vector2D::new(0.49, 0.49), rounding, flags); // Better looking lower-right corner and rounded non-AA shapes.
        //     PathStroke(col, DrawFlags::Closed, thickness);
    }
    //  void  add_rect_filled(const Vector2D& p_min, const Vector2D& p_max, ImU32 col, float rounding = 0.0, ImDrawFlags flags = 0);                     // a: upper-left, b: lower-right (== upper-left + size)
    pub fn add_rect_filled(
        &mut self,
        p_min: &Vector2D,
        p_max: &Vector2D,
        col: u32,
        rounding: f32,
        flags: Option<&HashSet<DrawFlags>>
    ) {
        //  if ((col & IM_COL32_A_MASK) == 0)
        //         return;
        //     if (rounding < 0.5 || (flags & DrawFlags::RoundCornersMask_) == DrawFlags::RoundCornersNone)
        //     {
        //         PrimReserve(6, 4);
        //         PrimRect(p_min, p_max, col);
        //     }
        //     else
        //     {
        //         PathRect(p_min, p_max, rounding, flags);
        //         path_fill_convex(col);
        //     }
    }
    //  void  add_rect_filled_multi_color(const Vector2D& p_min, const Vector2D& p_max, ImU32 col_upr_left, ImU32 col_upr_right, ImU32 col_bot_right, ImU32 col_bot_left);
    pub fn add_rect_filled_multi_color(
        &mut self,
        p_min: &Vector2D,
        p_max: &Vector2D,
        col_upr_left: u32,
        col_upr_right: u32,
        col_bot_right: u32,
        col_bot_left: u32,
    ) {
        //  if (((col_upr_left | col_upr_right | col_bot_right | col_bot_left) & IM_COL32_A_MASK) == 0)
        //         return;
        //
        //     const Vector2D uv = _Data.TexUvWhitePixel;
        //     PrimReserve(6, 4);
        //     PrimWriteIdx((ImDrawIdx)(_VtxCurrentIdx)); PrimWriteIdx((ImDrawIdx)(_VtxCurrentIdx + 1)); PrimWriteIdx((ImDrawIdx)(_VtxCurrentIdx + 2));
        //     PrimWriteIdx((ImDrawIdx)(_VtxCurrentIdx)); PrimWriteIdx((ImDrawIdx)(_VtxCurrentIdx + 2)); PrimWriteIdx((ImDrawIdx)(_VtxCurrentIdx + 3));
        //     PrimWriteVtx(p_min, uv, col_upr_left);
        //     PrimWriteVtx(Vector2D::new(p_max.x, p_min.y), uv, col_upr_right);
        //     PrimWriteVtx(p_max, uv, col_bot_right);
        //     PrimWriteVtx(Vector2D::new(p_min.x, p_max.y), uv, col_bot_left);
    }
    //  void  add_quad(const Vector2D& p1, const Vector2D& p2, const Vector2D& p3, const Vector2D& p4, ImU32 col, float thickness = 1.0);
    pub fn add_quad(
        &mut self,
        p1: &Vector2D,
        p2: &Vector2D,
        p3: &Vector2D,
        p4: &Vector2D,
        col: u32,
        thickness: f32,
    ) {
        //  if ((col & IM_COL32_A_MASK) == 0)
        //         return;
        //
        //     PathLineTo(p1);
        //     PathLineTo(p2);
        //     PathLineTo(p3);
        //     PathLineTo(p4);
        //     PathStroke(col, DrawFlags::Closed, thickness);
    }
    //  void  add_quad_filled(const Vector2D& p1, const Vector2D& p2, const Vector2D& p3, const Vector2D& p4, ImU32 col);
    pub fn add_quad_filled(
        &mut self,
        p1: &Vector2D,
        p2: &Vector2D,
        p3: &Vector2D,
        p4: &Vector2D,
        col: u32,
    ) {
        //  if ((col & IM_COL32_A_MASK) == 0)
        //         return;
        //
        //     PathLineTo(p1);
        //     PathLineTo(p2);
        //     PathLineTo(p3);
        //     PathLineTo(p4);
        //     path_fill_convex(col);
    }
    //  void  add_triangle(const Vector2D& p1, const Vector2D& p2, const Vector2D& p3, ImU32 col, float thickness = 1.0);
    pub fn add_triangle(
        &mut self,
        p1: &Vector2D,
        p2: &Vector2D,
        p3: &Vector2D,
        col: u32,
        thickness: f32,
    ) {
        // if ((col & IM_COL32_A_MASK) == 0)
        //         return;
        //
        //     PathLineTo(p1);
        //     PathLineTo(p2);
        //     PathLineTo(p3);
        //     PathStroke(col, DrawFlags::Closed, thickness);
    }
    //  void  add_triangle_filled(const Vector2D& p1, const Vector2D& p2, const Vector2D& p3, ImU32 col);
    pub fn add_triangle_filled(&mut self, p1: &Vector2D, p2: &Vector2D, p3: &Vector2D, col: u32) {
        //   if ((col & IM_COL32_A_MASK) == 0)
        //         return;
        //
        //     PathLineTo(p1);
        //     PathLineTo(p2);
        //     PathLineTo(p3);
        //     path_fill_convex(col);
    }
    //  void  add_circle(const Vector2D& center, float radius, ImU32 col, int num_segments = 0, float thickness = 1.0);
    pub fn add_circle(
        &mut self,
        center: &Vector2D,
        radius: f32,
        col: u32,
        num_segments: i32,
        thickness: f32,
    ) {
       //  if ((col & IM_COL32_A_MASK) == 0 || radius < 0.5)
        //         return;
        //
        //     if (num_segments <= 0)
        //     {
        //         // Use arc with automatic segment count
        //         _PathArcToFastEx(center, radius - 0.5, 0, IM_DRAWLIST_ARCFAST_SAMPLE_MAX, 0);
        //         _Path.size--;
        //     }
        //     else
        //     {
        //         // Explicit segment count (still clamp to avoid drawing insanely tessellated shapes)
        //         num_segments = ImClamp(num_segments, 3, IM_DRAWLIST_CIRCLE_AUTO_SEGMENT_MAX);
        //
        //         // Because we are filling a closed shape we remove 1 from the count of segments/points
        //         let a_max = (f32::PI * 2.0) * ((float)num_segments - 1.0) / num_segments;
        //         PathArcTo(center, radius - 0.5, 0.0, a_max, num_segments - 1);
        //     }
        //
        //     PathStroke(col, DrawFlags::Closed, thickness);
    }
    //  void  add_circle_filled(const Vector2D& center, float radius, ImU32 col, int num_segments = 0);
    pub fn add_circle_filled(
        &mut self,
        center: &Vector2D,
        radius: f32,
        col: u32,
        num_segments: i32,
    ) {
        // if ((col & IM_COL32_A_MASK) == 0 || radius < 0.5)
        //         return;
        //
        //     if (num_segments <= 0)
        //     {
        //         // Use arc with automatic segment count
        //         _PathArcToFastEx(center, radius, 0, IM_DRAWLIST_ARCFAST_SAMPLE_MAX, 0);
        //         _Path.size--;
        //     }
        //     else
        //     {
        //         // Explicit segment count (still clamp to avoid drawing insanely tessellated shapes)
        //         num_segments = ImClamp(num_segments, 3, IM_DRAWLIST_CIRCLE_AUTO_SEGMENT_MAX);
        //
        //         // Because we are filling a closed shape we remove 1 from the count of segments/points
        //         let a_max = (f32::PI * 2.0) * ((float)num_segments - 1.0) / num_segments;
        //         PathArcTo(center, radius, 0.0, a_max, num_segments - 1);
        //     }
        //
        //     path_fill_convex(col);
    }
    //  void  add_ngon(const Vector2D& center, float radius, ImU32 col, int num_segments, float thickness = 1.0);
    pub fn add_ngon(
        &mut self,
        center: &Vector2D,
        radius: f32,
        col: u32,
        num_segments: i32,
        thickness: f32,
    ) {
        //  if ((col & IM_COL32_A_MASK) == 0 || num_segments <= 2)
        //         return;
        //
        //     // Because we are filling a closed shape we remove 1 from the count of segments/points
        //     let a_max = (f32::PI * 2.0) * ((float)num_segments - 1.0) / num_segments;
        //     PathArcTo(center, radius - 0.5, 0.0, a_max, num_segments - 1);
        //     PathStroke(col, DrawFlags::Closed, thickness);
    }
    //  void  AddNgonFilled(const Vector2D& center, float radius, ImU32 col, int num_segments);
    pub fn AddNgonFilled(&mut self, center: &Vector2D, radius: f32, col: u32, num_segments: i32) {
       //     if ((col & IM_COL32_A_MASK) == 0 || num_segments <= 2)
        //         return;
        //
        //     // Because we are filling a closed shape we remove 1 from the count of segments/points
        //     let a_max = (f32::PI * 2.0) * ((float)num_segments - 1.0) / num_segments;
        //     PathArcTo(center, radius, 0.0, a_max, num_segments - 1);
        //     path_fill_convex(col);
    }
    //  void  add_text(const Vector2D& pos, ImU32 col, const char* text_begin, const char* text_end = None);
    pub fn AddText(&mut self, pos: &Vector2D, col: u32, text_begin: &String, text_end: &String) {
         if ((col & IM_COL32_A_MASK) == 0)
        return;

    if (text_end == None)
        text_end = text_begin + strlen(text_begin);
    if (text_begin == text_end)
        return;

    // Pull default font/size from the shared ImDrawListSharedData instance
    if (font == None)
        font = _Data.Font;
    if (font_size == 0.0)
        font_size = _Data.font_size;

    // IM_ASSERT(font.container_atlas.TexID == _CmdHeader.TextureId);  // Use high-level ImGui::PushFont() or low-level ImDrawList::PushTextureId() to change font.

    Vector4D clip_rect = _CmdHeader.clip_rect;
    if (cpu_fine_clip_rect)
    {
        clip_rect.x = ImMax(clip_rect.x, cpu_fine_clip_rect.x);
        clip_rect.y = ImMax(clip_rect.y, cpu_fine_clip_rect.y);
        clip_rect.z = ImMin(clip_rect.z, cpu_fine_clip_rect.z);
        clip_rect.w = ImMin(clip_rect.w, cpu_fine_clip_rect.w);
    }
    font.RenderText(this, font_size, pos, col, clip_rect, text_begin, text_end, wrap_width, cpu_fine_clip_rect != None);
    }
    //  void  add_text(const ImFont* font, float font_size, const Vector2D& pos, ImU32 col, const char* text_begin, const char* text_end = None, float wrap_width = 0.0, const Vector4D* cpu_fine_clip_rect = None);
    pub fn AddText2(
        &mut self,
        font: &Font,
        font_size: f32,
        pos: &Vector2D,
        col: u32,
        text_begin: &str,
        text_end: &str,
        wrap_width: f32,
        cpu_fine_clip_rect: Option<&Vector4D>,
    ) {
        AddText(None, 0.0, pos, col, text_begin, text_end);
    }
    //  void  AddPolyline(const Vector2D* points, int num_points, ImU32 col, ImDrawFlags flags, float thickness);
    pub fn AddPolyline(
        &mut self,
        points: &[Vector2D],
        num_points: usize,
        col: u32,
        flags: DrawFlags,
        thickness: f32,
    ) {
        // if (points_count < 2)
        //         return;
        //
        //     const bool closed = (flags & DrawFlags::Closed) != 0;
        //     const Vector2D opaque_uv = _Data.TexUvWhitePixel;
        //     let count = closed ? points_count : points_count - 1; // The number of line segments we need to draw
        //     const bool thick_line = (thickness > _FringeScale);
        //
        //     if (Flags & DrawListFlags::AntiAliasedLines)
        //     {
        //         // Anti-aliased stroke
        //         let AA_SIZE = _FringeScale;
        //         const ImU32 col_trans = col & ~IM_COL32_A_MASK;
        //
        //         // Thicknesses <1.0 should behave like thickness 1.0
        //         thickness = ImMax(thickness, 1.0);
        //         let integer_thickness = thickness;
        //         let fractional_thickness = thickness - integer_thickness;
        //
        //         // Do we want to draw this line using a texture?
        //         // - For now, only draw integer-width lines using textures to avoid issues with the way scaling occurs, could be improved.
        //         // - If AA_SIZE is not 1.0 we cannot use the texture path.
        //         const bool use_texture = (Flags & DrawListFlags::AntiAliasedLinesUseTex) && (integer_thickness < IM_DRAWLIST_TEX_LINES_WIDTH_MAX) && (fractional_thickness <= 0.00001) && (AA_SIZE == 1.0);
        //
        //         // We should never hit this, because NewFrame() doesn't set ImDrawListFlags_AntiAliasedLinesUseTex unless ImFontAtlasFlags_NoBakedLines is off
        //         // IM_ASSERT_PARANOID(!use_texture || !(_Data.Font.container_atlas.flags & FontAtlasFlags::NoBakedLines));
        //
        //         let idx_count = use_texture ? (count * 6) : (thick_line ? count * 18 : count * 12);
        //         let vtx_count = use_texture ? (points_count * 2) : (thick_line ? points_count * 4 : points_count * 3);
        //         PrimReserve(idx_count, vtx_count);
        //
        //         // Temporary buffer
        //         // The first <points_count> items are normals at each line point, then after that there are either 2 or 4 temp points for each line point
        //         Vector2D* temp_normals = (Vector2D*)alloca(points_count * ((use_texture || !thick_line) ? 3 : 5) * sizeof(Vector2D)); //-V630
        //         Vector2D* temp_points = temp_normals + points_count;
        //
        //         // Calculate normals (tangents) for each line segment
        //         for (int i1 = 0; i1 < count; i1 += 1)
        //         {
        //             let i2 = (i1 + 1) == points_count ? 0 : i1 + 1;
        //             float dx = points[i2].x - points[i1].x;
        //             float dy = points[i2].y - points[i1].y;
        //             IM_NORMALIZE2F_OVER_ZERO(dx, dy);
        //             temp_normals[i1].x = dy;
        //             temp_normals[i1].y = -dx;
        //         }
        //         if (!closed)
        //             temp_normals[points_count - 1] = temp_normals[points_count - 2];
        //
        //         // If we are drawing a one-pixel-wide line without a texture, or a textured line of any width, we only need 2 or 3 vertices per point
        //         if (use_texture || !thick_line)
        //         {
        //             // [PATH 1] Texture-based lines (thick or non-thick)
        //             // [PATH 2] Non texture-based lines (non-thick)
        //
        //             // The width of the geometry we need to draw - this is essentially <thickness> pixels for the line itself, plus "one pixel" for AA.
        //             // - In the texture-based path, we don't use AA_SIZE here because the +1 is tied to the generated texture
        //             //   (see ImFontAtlasBuildRenderLinesTexData() function), and so alternate values won't work without changes to that code.
        //             // - In the non texture-based paths, we would allow AA_SIZE to potentially be != 1.0 with a patch (e.g. fringe_scale patch to
        //             //   allow scaling geometry while preserving one-screen-pixel AA fringe).
        //             let half_draw_size = use_texture ? ((thickness * 0.5) + 1) : AA_SIZE;
        //
        //             // If line is not closed, the first and last points need to be generated differently as there are no normals to blend
        //             if (!closed)
        //             {
        //                 temp_points[0] = points[0] + temp_normals[0] * half_draw_size;
        //                 temp_points[1] = points[0] - temp_normals[0] * half_draw_size;
        //                 temp_points[(points_count-1)*2+0] = points[points_count-1] + temp_normals[points_count-1] * half_draw_size;
        //                 temp_points[(points_count-1)*2+1] = points[points_count-1] - temp_normals[points_count-1] * half_draw_size;
        //             }
        //
        //             // Generate the indices to form a number of triangles for each line segment, and the vertices for the line edges
        //             // This takes points n and n+1 and writes into n+1, with the first point in a closed line being generated from the final one (as n+1 wraps)
        //             // FIXME-OPT: merge the different loops, possibly remove the temporary buffer.
        //             unsigned int idx1 = _VtxCurrentIdx; // Vertex index for start of line segment
        //             for (int i1 = 0; i1 < count; i1 += 1) // i1 is the first point of the line segment
        //             {
        //                 let i2 = (i1 + 1) == points_count ? 0 : i1 + 1; // i2 is the second point of the line segment
        //                 const unsigned int idx2 = ((i1 + 1) == points_count) ? _VtxCurrentIdx : (idx1 + (use_texture ? 2 : 3)); // Vertex index for end of segment
        //
        //                 // Average normals
        //                 float dm_x = (temp_normals[i1].x + temp_normals[i2].x) * 0.5;
        //                 float dm_y = (temp_normals[i1].y + temp_normals[i2].y) * 0.5;
        //                 IM_FIXNORMAL2F(dm_x, dm_y);
        //                 dm_x *= half_draw_size; // dm_x, dm_y are offset to the outer edge of the AA area
        //                 dm_y *= half_draw_size;
        //
        //                 // Add temporary vertexes for the outer edges
        //                 Vector2D* out_vtx = &temp_points[i2 * 2];
        //                 out_vtx[0].x = points[i2].x + dm_x;
        //                 out_vtx[0].y = points[i2].y + dm_y;
        //                 out_vtx[1].x = points[i2].x - dm_x;
        //                 out_vtx[1].y = points[i2].y - dm_y;
        //
        //                 if (use_texture)
        //                 {
        //                     // Add indices for two triangles
        //                     _IdxWritePtr[0] = (ImDrawIdx)(idx2 + 0); _IdxWritePtr[1] = (ImDrawIdx)(idx1 + 0); _IdxWritePtr[2] = (ImDrawIdx)(idx1 + 1); // Right tri
        //                     _IdxWritePtr[3] = (ImDrawIdx)(idx2 + 1); _IdxWritePtr[4] = (ImDrawIdx)(idx1 + 1); _IdxWritePtr[5] = (ImDrawIdx)(idx2 + 0); // Left tri
        //                     _IdxWritePtr += 6;
        //                 }
        //                 else
        //                 {
        //                     // Add indexes for four triangles
        //                     _IdxWritePtr[0] = (ImDrawIdx)(idx2 + 0); _IdxWritePtr[1] = (ImDrawIdx)(idx1 + 0); _IdxWritePtr[2] = (ImDrawIdx)(idx1 + 2); // Right tri 1
        //                     _IdxWritePtr[3] = (ImDrawIdx)(idx1 + 2); _IdxWritePtr[4] = (ImDrawIdx)(idx2 + 2); _IdxWritePtr[5] = (ImDrawIdx)(idx2 + 0); // Right tri 2
        //                     _IdxWritePtr[6] = (ImDrawIdx)(idx2 + 1); _IdxWritePtr[7] = (ImDrawIdx)(idx1 + 1); _IdxWritePtr[8] = (ImDrawIdx)(idx1 + 0); // Left tri 1
        //                     _IdxWritePtr[9] = (ImDrawIdx)(idx1 + 0); _IdxWritePtr[10] = (ImDrawIdx)(idx2 + 0); _IdxWritePtr[11] = (ImDrawIdx)(idx2 + 1); // Left tri 2
        //                     _IdxWritePtr += 12;
        //                 }
        //
        //                 idx1 = idx2;
        //             }
        //
        //             // Add vertexes for each point on the line
        //             if (use_texture)
        //             {
        //                 // If we're using textures we only need to emit the left/right edge vertices
        //                 Vector4D tex_uvs = _Data.TexUvLines[integer_thickness];
        //                 /*if (fractional_thickness != 0.0) // Currently always zero when use_texture==false!
        //                 {
        //                     const Vector4D tex_uvs_1 = _Data->tex_uv_lines[integer_thickness + 1];
        //                     tex_uvs.x = tex_uvs.x + (tex_uvs_1.x - tex_uvs.x) * fractional_thickness; // inlined ImLerp()
        //                     tex_uvs.y = tex_uvs.y + (tex_uvs_1.y - tex_uvs.y) * fractional_thickness;
        //                     tex_uvs.z = tex_uvs.z + (tex_uvs_1.z - tex_uvs.z) * fractional_thickness;
        //                     tex_uvs.w = tex_uvs.w + (tex_uvs_1.w - tex_uvs.w) * fractional_thickness;
        //                 }*/
        //                 Vector2D tex_uv0(tex_uvs.x, tex_uvs.y);
        //                 Vector2D tex_uv1(tex_uvs.z, tex_uvs.w);
        //                 for (int i = 0; i < points_count; i += 1)
        //                 {
        //                     _VtxWritePtr[0].pos = temp_points[i * 2 + 0]; _VtxWritePtr[0].uv = tex_uv0; _VtxWritePtr[0].col = col; // Left-side outer edge
        //                     _VtxWritePtr[1].pos = temp_points[i * 2 + 1]; _VtxWritePtr[1].uv = tex_uv1; _VtxWritePtr[1].col = col; // Right-side outer edge
        //                     _VtxWritePtr += 2;
        //                 }
        //             }
        //             else
        //             {
        //                 // If we're not using a texture, we need the center vertex as well
        //                 for (int i = 0; i < points_count; i += 1)
        //                 {
        //                     _VtxWritePtr[0].pos = points[i];              _VtxWritePtr[0].uv = opaque_uv; _VtxWritePtr[0].col = col;       // Center of line
        //                     _VtxWritePtr[1].pos = temp_points[i * 2 + 0]; _VtxWritePtr[1].uv = opaque_uv; _VtxWritePtr[1].col = col_trans; // Left-side outer edge
        //                     _VtxWritePtr[2].pos = temp_points[i * 2 + 1]; _VtxWritePtr[2].uv = opaque_uv; _VtxWritePtr[2].col = col_trans; // Right-side outer edge
        //                     _VtxWritePtr += 3;
        //                 }
        //             }
        //         }
        //         else
        //         {
        //             // [PATH 2] Non texture-based lines (thick): we need to draw the solid line core and thus require four vertices per point
        //             let half_inner_thickness = (thickness - AA_SIZE) * 0.5;
        //
        //             // If line is not closed, the first and last points need to be generated differently as there are no normals to blend
        //             if (!closed)
        //             {
        //                 let points_last = points_count - 1;
        //                 temp_points[0] = points[0] + temp_normals[0] * (half_inner_thickness + AA_SIZE);
        //                 temp_points[1] = points[0] + temp_normals[0] * (half_inner_thickness);
        //                 temp_points[2] = points[0] - temp_normals[0] * (half_inner_thickness);
        //                 temp_points[3] = points[0] - temp_normals[0] * (half_inner_thickness + AA_SIZE);
        //                 temp_points[points_last * 4 + 0] = points[points_last] + temp_normals[points_last] * (half_inner_thickness + AA_SIZE);
        //                 temp_points[points_last * 4 + 1] = points[points_last] + temp_normals[points_last] * (half_inner_thickness);
        //                 temp_points[points_last * 4 + 2] = points[points_last] - temp_normals[points_last] * (half_inner_thickness);
        //                 temp_points[points_last * 4 + 3] = points[points_last] - temp_normals[points_last] * (half_inner_thickness + AA_SIZE);
        //             }
        //
        //             // Generate the indices to form a number of triangles for each line segment, and the vertices for the line edges
        //             // This takes points n and n+1 and writes into n+1, with the first point in a closed line being generated from the final one (as n+1 wraps)
        //             // FIXME-OPT: merge the different loops, possibly remove the temporary buffer.
        //             unsigned int idx1 = _VtxCurrentIdx; // Vertex index for start of line segment
        //             for (int i1 = 0; i1 < count; i1 += 1) // i1 is the first point of the line segment
        //             {
        //                 let i2 = (i1 + 1) == points_count ? 0 : (i1 + 1); // i2 is the second point of the line segment
        //                 const unsigned int idx2 = (i1 + 1) == points_count ? _VtxCurrentIdx : (idx1 + 4); // Vertex index for end of segment
        //
        //                 // Average normals
        //                 float dm_x = (temp_normals[i1].x + temp_normals[i2].x) * 0.5;
        //                 float dm_y = (temp_normals[i1].y + temp_normals[i2].y) * 0.5;
        //                 IM_FIXNORMAL2F(dm_x, dm_y);
        //                 float dm_out_x = dm_x * (half_inner_thickness + AA_SIZE);
        //                 float dm_out_y = dm_y * (half_inner_thickness + AA_SIZE);
        //                 float dm_in_x = dm_x * half_inner_thickness;
        //                 float dm_in_y = dm_y * half_inner_thickness;
        //
        //                 // Add temporary vertices
        //                 Vector2D* out_vtx = &temp_points[i2 * 4];
        //                 out_vtx[0].x = points[i2].x + dm_out_x;
        //                 out_vtx[0].y = points[i2].y + dm_out_y;
        //                 out_vtx[1].x = points[i2].x + dm_in_x;
        //                 out_vtx[1].y = points[i2].y + dm_in_y;
        //                 out_vtx[2].x = points[i2].x - dm_in_x;
        //                 out_vtx[2].y = points[i2].y - dm_in_y;
        //                 out_vtx[3].x = points[i2].x - dm_out_x;
        //                 out_vtx[3].y = points[i2].y - dm_out_y;
        //
        //                 // Add indexes
        //                 _IdxWritePtr[0]  = (ImDrawIdx)(idx2 + 1); _IdxWritePtr[1]  = (ImDrawIdx)(idx1 + 1); _IdxWritePtr[2]  = (ImDrawIdx)(idx1 + 2);
        //                 _IdxWritePtr[3]  = (ImDrawIdx)(idx1 + 2); _IdxWritePtr[4]  = (ImDrawIdx)(idx2 + 2); _IdxWritePtr[5]  = (ImDrawIdx)(idx2 + 1);
        //                 _IdxWritePtr[6]  = (ImDrawIdx)(idx2 + 1); _IdxWritePtr[7]  = (ImDrawIdx)(idx1 + 1); _IdxWritePtr[8]  = (ImDrawIdx)(idx1 + 0);
        //                 _IdxWritePtr[9]  = (ImDrawIdx)(idx1 + 0); _IdxWritePtr[10] = (ImDrawIdx)(idx2 + 0); _IdxWritePtr[11] = (ImDrawIdx)(idx2 + 1);
        //                 _IdxWritePtr[12] = (ImDrawIdx)(idx2 + 2); _IdxWritePtr[13] = (ImDrawIdx)(idx1 + 2); _IdxWritePtr[14] = (ImDrawIdx)(idx1 + 3);
        //                 _IdxWritePtr[15] = (ImDrawIdx)(idx1 + 3); _IdxWritePtr[16] = (ImDrawIdx)(idx2 + 3); _IdxWritePtr[17] = (ImDrawIdx)(idx2 + 2);
        //                 _IdxWritePtr += 18;
        //
        //                 idx1 = idx2;
        //             }
        //
        //             // Add vertices
        //             for (int i = 0; i < points_count; i += 1)
        //             {
        //                 _VtxWritePtr[0].pos = temp_points[i * 4 + 0]; _VtxWritePtr[0].uv = opaque_uv; _VtxWritePtr[0].col = col_trans;
        //                 _VtxWritePtr[1].pos = temp_points[i * 4 + 1]; _VtxWritePtr[1].uv = opaque_uv; _VtxWritePtr[1].col = col;
        //                 _VtxWritePtr[2].pos = temp_points[i * 4 + 2]; _VtxWritePtr[2].uv = opaque_uv; _VtxWritePtr[2].col = col;
        //                 _VtxWritePtr[3].pos = temp_points[i * 4 + 3]; _VtxWritePtr[3].uv = opaque_uv; _VtxWritePtr[3].col = col_trans;
        //                 _VtxWritePtr += 4;
        //             }
        //         }
        //         _VtxCurrentIdx += (ImDrawIdx)vtx_count;
        //     }
        //     else
        //     {
        //         // [PATH 4] Non texture-based, Non anti-aliased lines
        //         let idx_count = count * 6;
        //         let vtx_count = count * 4;    // FIXME-OPT: Not sharing edges
        //         PrimReserve(idx_count, vtx_count);
        //
        //         for (int i1 = 0; i1 < count; i1 += 1)
        //         {
        //             let i2 = (i1 + 1) == points_count ? 0 : i1 + 1;
        //             const Vector2D& p1 = points[i1];
        //             const Vector2D& p2 = points[i2];
        //
        //             float dx = p2.x - p1.x;
        //             float dy = p2.y - p1.y;
        //             IM_NORMALIZE2F_OVER_ZERO(dx, dy);
        //             dx *= (thickness * 0.5);
        //             dy *= (thickness * 0.5);
        //
        //             _VtxWritePtr[0].pos.x = p1.x + dy; _VtxWritePtr[0].pos.y = p1.y - dx; _VtxWritePtr[0].uv = opaque_uv; _VtxWritePtr[0].col = col;
        //             _VtxWritePtr[1].pos.x = p2.x + dy; _VtxWritePtr[1].pos.y = p2.y - dx; _VtxWritePtr[1].uv = opaque_uv; _VtxWritePtr[1].col = col;
        //             _VtxWritePtr[2].pos.x = p2.x - dy; _VtxWritePtr[2].pos.y = p2.y + dx; _VtxWritePtr[2].uv = opaque_uv; _VtxWritePtr[2].col = col;
        //             _VtxWritePtr[3].pos.x = p1.x - dy; _VtxWritePtr[3].pos.y = p1.y + dx; _VtxWritePtr[3].uv = opaque_uv; _VtxWritePtr[3].col = col;
        //             _VtxWritePtr += 4;
        //
        //             _IdxWritePtr[0] = (ImDrawIdx)(_VtxCurrentIdx); _IdxWritePtr[1] = (ImDrawIdx)(_VtxCurrentIdx + 1); _IdxWritePtr[2] = (ImDrawIdx)(_VtxCurrentIdx + 2);
        //             _IdxWritePtr[3] = (ImDrawIdx)(_VtxCurrentIdx); _IdxWritePtr[4] = (ImDrawIdx)(_VtxCurrentIdx + 2); _IdxWritePtr[5] = (ImDrawIdx)(_VtxCurrentIdx + 3);
        //             _IdxWritePtr += 6;
        //             _VtxCurrentIdx += 4;
        //         }
        //     }
    }
    //  void  AddConvexPolyFilled(const Vector2D* points, int num_points, ImU32 col);
    pub fn AddConvexPolyFilled(&mut self, points: &[Vector2D], num_points: usize, col: u32) {
        //   if (points_count < 3)
        //         return;
        //
        //     const Vector2D uv = _Data.TexUvWhitePixel;
        //
        //     if (Flags & DrawListFlags::AntiAliasedFill)
        //     {
        //         // Anti-aliased Fill
        //         let AA_SIZE = _FringeScale;
        //         const ImU32 col_trans = col & ~IM_COL32_A_MASK;
        //         let idx_count = (points_count - 2)*3 + points_count * 6;
        //         let vtx_count = (points_count * 2);
        //         PrimReserve(idx_count, vtx_count);
        //
        //         // Add indexes for fill
        //         unsigned int vtx_inner_idx = _VtxCurrentIdx;
        //         unsigned int vtx_outer_idx = _VtxCurrentIdx + 1;
        //         for (int i = 2; i < points_count; i += 1)
        //         {
        //             _IdxWritePtr[0] = (ImDrawIdx)(vtx_inner_idx); _IdxWritePtr[1] = (ImDrawIdx)(vtx_inner_idx + ((i - 1) << 1)); _IdxWritePtr[2] = (ImDrawIdx)(vtx_inner_idx + (i << 1));
        //             _IdxWritePtr += 3;
        //         }
        //
        //         // Compute normals
        //         Vector2D* temp_normals = (Vector2D*)alloca(points_count * sizeof(Vector2D)); //-V630
        //         for (int i0 = points_count - 1, i1 = 0; i1 < points_count; i0 = i1 += 1)
        //         {
        //             const Vector2D& p0 = points[i0];
        //             const Vector2D& p1 = points[i1];
        //             float dx = p1.x - p0.x;
        //             float dy = p1.y - p0.y;
        //             IM_NORMALIZE2F_OVER_ZERO(dx, dy);
        //             temp_normals[i0].x = dy;
        //             temp_normals[i0].y = -dx;
        //         }
        //
        //         for (int i0 = points_count - 1, i1 = 0; i1 < points_count; i0 = i1 += 1)
        //         {
        //             // Average normals
        //             const Vector2D& n0 = temp_normals[i0];
        //             const Vector2D& n1 = temp_normals[i1];
        //             float dm_x = (n0.x + n1.x) * 0.5;
        //             float dm_y = (n0.y + n1.y) * 0.5;
        //             IM_FIXNORMAL2F(dm_x, dm_y);
        //             dm_x *= AA_SIZE * 0.5;
        //             dm_y *= AA_SIZE * 0.5;
        //
        //             // Add vertices
        //             _VtxWritePtr[0].pos.x = (points[i1].x - dm_x); _VtxWritePtr[0].pos.y = (points[i1].y - dm_y); _VtxWritePtr[0].uv = uv; _VtxWritePtr[0].col = col;        // Inner
        //             _VtxWritePtr[1].pos.x = (points[i1].x + dm_x); _VtxWritePtr[1].pos.y = (points[i1].y + dm_y); _VtxWritePtr[1].uv = uv; _VtxWritePtr[1].col = col_trans;  // Outer
        //             _VtxWritePtr += 2;
        //
        //             // Add indexes for fringes
        //             _IdxWritePtr[0] = (ImDrawIdx)(vtx_inner_idx + (i1 << 1)); _IdxWritePtr[1] = (ImDrawIdx)(vtx_inner_idx + (i0 << 1)); _IdxWritePtr[2] = (ImDrawIdx)(vtx_outer_idx + (i0 << 1));
        //             _IdxWritePtr[3] = (ImDrawIdx)(vtx_outer_idx + (i0 << 1)); _IdxWritePtr[4] = (ImDrawIdx)(vtx_outer_idx + (i1 << 1)); _IdxWritePtr[5] = (ImDrawIdx)(vtx_inner_idx + (i1 << 1));
        //             _IdxWritePtr += 6;
        //         }
        //         _VtxCurrentIdx += (ImDrawIdx)vtx_count;
        //     }
        //     else
        //     {
        //         // Non Anti-aliased Fill
        //         let idx_count = (points_count - 2)*3;
        //         let vtx_count = points_count;
        //         PrimReserve(idx_count, vtx_count);
        //         for (int i = 0; i < vtx_count; i += 1)
        //         {
        //             _VtxWritePtr[0].pos = points[i]; _VtxWritePtr[0].uv = uv; _VtxWritePtr[0].col = col;
        //             _VtxWritePtr += 1;
        //         }
        //         for (int i = 2; i < points_count; i += 1)
        //         {
        //             _IdxWritePtr[0] = (ImDrawIdx)(_VtxCurrentIdx); _IdxWritePtr[1] = (ImDrawIdx)(_VtxCurrentIdx + i - 1); _IdxWritePtr[2] = (ImDrawIdx)(_VtxCurrentIdx + i);
        //             _IdxWritePtr += 3;
        //         }
        //         _VtxCurrentIdx += (ImDrawIdx)vtx_count;
        //     }
    }
    //  void  AddBezierCubic(const Vector2D& p1, const Vector2D& p2, const Vector2D& p3, const Vector2D& p4, ImU32 col, float thickness, int num_segments = 0); // Cubic Bezier (4 control points)
    pub fn AddBezierCubic(
        &mut self,
        p1: &Vector2D,
        p2: &Vector2D,
        p3: &Vector2D,
        p4: &Vector2D,
        col: u32,
        thickness: f32,
        num_segments: i32,
    ) {
       if ((col & IM_COL32_A_MASK) == 0)
        return;

    PathLineTo(p1);
    PathBezierCubicCurveTo(p2, p3, p4, num_segments);
    PathStroke(col, 0, thickness);
    }
    //  void  AddBezierQuadratic(const Vector2D& p1, const Vector2D& p2, const Vector2D& p3, ImU32 col, float thickness, int num_segments = 0);               // Quadratic Bezier (3 control points)
    pub fn AddBezierQuadratic(
        &mut self,
        p1: &Vector2D,
        p2: &Vector2D,
        p3: &Vector2D,
        col: u32,
        thickness: f32,
        num_segments: i32,
    ) {
         if ((col & IM_COL32_A_MASK) == 0)
        return;

    PathLineTo(p1);
    PathBezierQuadraticCurveTo(p2, p3, num_segments);
    PathStroke(col, 0, thickness);
    }

    // Image primitives
    // - Read FAQ to understand what ImTextureID is.
    // - "p_min" and "p_max" represent the upper-left and lower-right corners of the rectangle.
    // - "uv_min" and "uv_max" represent the normalized texture coordinates to use for those corners. Using (0,0)->(1,1) texture coordinates will generally display the entire texture.
    //  void  AddImage(ImTextureID user_texture_id, const Vector2D& p_min, const Vector2D& p_max, const Vector2D& uv_min = Vector2D(0, 0), const Vector2D& uv_max = Vector2D(1, 1), ImU32 col = IM_COL32_WHITE);
    pub fn AddImage(
        &mut self,
        user_texture_id: TextureId,
        p_min: &Vector2D,
        p_max: &Vector2D,
        uv_min: &Vector2D,
        uv_max: &Vector2D,
        col: u32,
    ) {
        //
        if ((col & IM_COL32_A_MASK) == 0)
        return;

    const bool push_texture_id = user_texture_id != _CmdHeader.TextureId;
    if (push_texture_id)
        PushTextureID(user_texture_id);

    PrimReserve(6, 4);
    PrimRectUV(p_min, p_max, uv_min, uv_max, col);

    if (push_texture_id)
        PopTextureID();
    }
    //  void  AddImageQuad(ImTextureID user_texture_id, const Vector2D& p1, const Vector2D& p2, const Vector2D& p3, const Vector2D& p4, const Vector2D& uv1 = Vector2D(0, 0), const Vector2D& uv2 = Vector2D(1, 0), const Vector2D& uv3 = Vector2D(1, 1), const Vector2D& uv4 = Vector2D(0, 1), ImU32 col = IM_COL32_WHITE);
    pub fn AddImageQuad(
        &mut self,
        user_texture_id: TextureId,
        p1: &Vector2D,
        p2: &Vector2D,
        p3: &Vector2D,
        p4: &Vector2D,
        uv1: &Vector2D,
        uv2: &Vector2D,
        uv3: &Vector2D,
        uv4: &Vector2D,
        col: u32,
    ) {
        if ((col & IM_COL32_A_MASK) == 0)
        return;

    const bool push_texture_id = user_texture_id != _CmdHeader.TextureId;
    if (push_texture_id)
        PushTextureID(user_texture_id);

    PrimReserve(6, 4);
    PrimQuadUV(p1, p2, p3, p4, uv1, uv2, uv3, uv4, col);

    if (push_texture_id)
        PopTextureID();
    }
    //  void  AddImageRounded(ImTextureID user_texture_id, const Vector2D& p_min, const Vector2D& p_max, const Vector2D& uv_min, const Vector2D& uv_max, ImU32 col, float rounding, ImDrawFlags flags = 0);
    pub fn AddImageRounded(
        &mut self,
        user_texture_id: TextureId,
        p_min: &Vector2D,
        p_max: &Vector2D,
        uv_min: &Vector2D,
        uv_max: &Vector2D,
        col: u32,
        rounding: f32,
        flags: DrawFlags,
    ) {
        if ((col & IM_COL32_A_MASK) == 0)
        return;

    flags = FixRectCornerFlags(flags);
    if (rounding < 0.5 || (flags & DrawFlags::RoundCornersMask_) == DrawFlags::RoundCornersNone)
    {
        AddImage(user_texture_id, p_min, p_max, uv_min, uv_max, col);
        return;
    }

    const bool push_texture_id = user_texture_id != _CmdHeader.TextureId;
    if (push_texture_id)
        PushTextureID(user_texture_id);

    int vert_start_idx = VtxBuffer.size;
    PathRect(p_min, p_max, rounding, flags);
    path_fill_convex(col);
    int vert_end_idx = VtxBuffer.size;
    ImGui::ShadeVertsLinearUV(this, vert_start_idx, vert_end_idx, p_min, p_max, uv_min, uv_max, true);

    if (push_texture_id)
        PopTextureID();
    }

    // Stateful path API, add points then finish with PathFillConvex() or PathStroke()
    // - Filled shapes must always use clockwise winding order. The anti-aliasing fringe depends on it. Counter-clockwise shapes will have "inward" anti-aliasing.
    // inline    void  PathClear()                                                 { _path.size = 0; }
    pub fn PathClear(&mut self) {
        self.path.clear();
    }
    // inline    void  PathLineTo(const Vector2D& pos)                               { _path.push_back(pos); }
    pub fn PathLineTo(&mut self, pos: &Vector2D) {
        self.path.push(pos.clone())
    }
    // inline    void  PathLineToMergeDuplicate(const Vector2D& pos)                 { if (_path.size == 0 || memcmp(&_path.data[_path.size - 1], &pos, 8) != 0) _path.push_back(pos); }
    pub fn PathLineToMergeDuplicate(&mut self, pos: &Vector2D) {
        if self.path.len() == 0 || (self.path[self.path.len() - 1] != pos) {
            self.path.push(pos.clone())
        }
    }
    // inline    void  PathFillConvex(ImU32 col)                                   { AddConvexPolyFilled(_path.data, _path.size, col); _path.size = 0; }
    pub fn path_fill_convex(&mut self, col: u32) {
        self.AddConvexPolyFilled(self.path.as_slice(), self.path.len(), col);
        self.path.clear()
    }
    // inline    void  PathStroke(ImU32 col, ImDrawFlags flags = 0, float thickness = 1.0) { AddPolyline(_path.data, _path.size, col, flags, thickness); _path.size = 0; }
    pub fn PathStroke(&mut self, col: u32, flags: DrawFlags, thickness: f32) {
        self.AddPolyline(self.path.as_slice(), self.path.len(), col, flags, thickness);
        self.path.clear()
    }
    //  void  PathArcTo(const Vector2D& center, float radius, float a_min, float a_max, int num_segments = 0);
    pub fn PathArcTo(
        &mut self,
        center: &Vector2D,
        radius: f32,
        a_min: f32,
        a_max: f32,
        num_segments: i32,
    ) {
        //  if (radius < 0.5)
        //     {
        //         _Path.push_back(center);
        //         return;
        //     }
        //
        //     if (num_segments > 0)
        //     {
        //         _PathArcToN(center, radius, a_min, a_max, num_segments);
        //         return;
        //     }
        //
        //     // Automatic segment count
        //     if (radius <= _Data.ArcFastRadiusCutoff)
        //     {
        //         const bool a_is_reverse = a_max < a_min;
        //
        //         // We are going to use precomputed values for mid samples.
        //         // Determine first and last sample in lookup table that belong to the arc.
        //         let a_min_sample_f = IM_DRAWLIST_ARCFAST_SAMPLE_MAX * a_min / (f32::PI * 2.0);
        //         let a_max_sample_f = IM_DRAWLIST_ARCFAST_SAMPLE_MAX * a_max / (f32::PI * 2.0);
        //
        //         let a_min_sample = a_is_reverse ? f32::floor(a_min_sample_f) : ImCeil(a_min_sample_f);
        //         let a_max_sample = a_is_reverse ? ImCeil(a_max_sample_f) : f32::floor(a_max_sample_f);
        //         let a_mid_samples = a_is_reverse ? ImMax(a_min_sample - a_max_sample, 0) : ImMax(a_max_sample - a_min_sample, 0);
        //
        //         let a_min_segment_angle = a_min_sample * f32::PI * 2.0 / IM_DRAWLIST_ARCFAST_SAMPLE_MAX;
        //         let a_max_segment_angle = a_max_sample * f32::PI * 2.0 / IM_DRAWLIST_ARCFAST_SAMPLE_MAX;
        //         const bool a_emit_start = ImAbs(a_min_segment_angle - a_min) >= 1e-5f;
        //         const bool a_emit_end = ImAbs(a_max - a_max_segment_angle) >= 1e-5f;
        //
        //         _Path.reserve(_Path.size + (a_mid_samples + 1 + (a_emit_start ? 1 : 0) + (a_emit_end ? 1 : 0)));
        //         if (a_emit_start)
        //             _Path.push_back(Vector2D::new(center.x + ImCos(a_min) * radius, center.y + ImSin(a_min) * radius));
        //         if (a_mid_samples > 0)
        //             _PathArcToFastEx(center, radius, a_min_sample, a_max_sample, 0);
        //         if (a_emit_end)
        //             _Path.push_back(Vector2D::new(center.x + ImCos(a_max) * radius, center.y + ImSin(a_max) * radius));
        //     }
        //     else
        //     {
        //         let arc_length = ImAbs(a_max - a_min);
        //         let circle_segment_count = _CalcCircleAutoSegmentCount(radius);
        //         let arc_segment_count = ImMax(ImCeil(circle_segment_count * arc_length / (f32::PI * 2.0)), (2.0 * f32::PI / arc_length));
        //         _PathArcToN(center, radius, a_min, a_max, arc_segment_count);
        //     }
    }
    //  void  PathArcToFast(const Vector2D& center, float radius, int a_min_of_12, int a_max_of_12);                // Use precomputed angles for a 12 steps circle
    pub fn path_arc_to_fast(
        &mut self,
        center: &Vector2D,
        radius: f32,
        a_min_of_12: i32,
        a_max_of_12: i32,
    ) {
        //  if (radius < 0.5)
        //     {
        //         _Path.push_back(center);
        //         return;
        //     }
        //     _PathArcToFastEx(center, radius, a_min_of_12 * IM_DRAWLIST_ARCFAST_SAMPLE_MAX / 12, a_max_of_12 * IM_DRAWLIST_ARCFAST_SAMPLE_MAX / 12, 0);
    }
    //  void  PathBezierCubicCurveTo(const Vector2D& p2, const Vector2D& p3, const Vector2D& p4, int num_segments = 0); // Cubic Bezier (4 control points)
    pub fn PathBezierCubicCurveTo(
        &mut self,
        p2: &Vector2D,
        p3: &Vector2D,
        p4: &Vector2D,
        num_segments: usize,
    ) {
        //Vector2D p1 = _Path.back();
        //     if (num_segments == 0)
        //     {
        //         PathBezierCubicCurveToCasteljau(&_Path, p1.x, p1.y, p2.x, p2.y, p3.x, p3.y, p4.x, p4.y, _Data.curve_tessellation_tol, 0); // Auto-tessellated
        //     }
        //     else
        //     {
        //         float t_step = 1.0 / num_segments;
        //         for (int i_step = 1; i_step <= num_segments; i_step += 1)
        //             _Path.push_back(ImBezierCubicCalc(p1, p2, p3, p4, t_step * i_step));
        //     }
    }
    //  void  PathBezierQuadraticCurveTo(const Vector2D& p2, const Vector2D& p3, int num_segments = 0);               // Quadratic Bezier (3 control points)
    pub fn PathBezierQuadraticCurveTo(
        &mut self,
        p2: &Vector2D,
        p3: &Vector2D,
        num_segments: usize,
    ) {
        //  Vector2D p1 = _Path.back();
        //     if (num_segments == 0)
        //     {
        //         PathBezierQuadraticCurveToCasteljau(&_Path, p1.x, p1.y, p2.x, p2.y, p3.x, p3.y, _Data.curve_tessellation_tol, 0);// Auto-tessellated
        //     }
        //     else
        //     {
        //         float t_step = 1.0 / num_segments;
        //         for (int i_step = 1; i_step <= num_segments; i_step += 1)
        //             _Path.push_back(ImBezierQuadraticCalc(p1, p2, p3, t_step * i_step));
        //     }
    }
    //  void  PathRect(const Vector2D& rect_min, const Vector2D& rect_max, float rounding = 0.0, ImDrawFlags flags = 0);
    pub fn PathRect(
        &mut self,
        rect_min: &Vector2D,
        rect_max: &Vector2D,
        rounding: f32,
        flags: DrawFlags,
    ) {
        // flags = FixRectCornerFlags(flags);
        //     rounding = ImMin(rounding, f32::abs(b.x - a.x) * ( ((flags & DrawFlags::RoundCornersTop)  == DrawFlags::RoundCornersTop)  || ((flags & DrawFlags::RoundCornersBottom) == DrawFlags::RoundCornersBottom) ? 0.5 : 1.0 ) - 1.0);
        //     rounding = ImMin(rounding, f32::abs(b.y - a.y) * ( ((flags & DrawFlags::RoundCornersLeft) == DrawFlags::RoundCornersLeft) || ((flags & DrawFlags::RoundCornersRight)  == DrawFlags::RoundCornersRight)  ? 0.5 : 1.0 ) - 1.0);
        //
        //     if (rounding < 0.5 || (flags & DrawFlags::RoundCornersMask_) == DrawFlags::RoundCornersNone)
        //     {
        //         PathLineTo(a);
        //         PathLineTo(Vector2D::new(b.x, a.y));
        //         PathLineTo(b);
        //         PathLineTo(Vector2D::new(a.x, b.y));
        //     }
        //     else
        //     {
        //         let rounding_tl = (flags & DrawFlags::RoundCornersTopLeft)     ? rounding : 0.0;
        //         let rounding_tr = (flags & DrawFlags::RoundCornersTopRight)    ? rounding : 0.0;
        //         let rounding_br = (flags & DrawFlags::RoundCornersBottomRight) ? rounding : 0.0;
        //         let rounding_bl = (flags & DrawFlags::RoundCornersBottomLeft)  ? rounding : 0.0;
        //         path_arc_to_fast(Vector2D::new(a.x + rounding_tl, a.y + rounding_tl), rounding_tl, 6, 9);
        //         path_arc_to_fast(Vector2D::new(b.x - rounding_tr, a.y + rounding_tr), rounding_tr, 9, 12);
        //         path_arc_to_fast(Vector2D::new(b.x - rounding_br, b.y - rounding_br), rounding_br, 0, 3);
        //         path_arc_to_fast(Vector2D::new(a.x + rounding_bl, b.y - rounding_bl), rounding_bl, 3, 6);
        //     }
    }

    // Advanced
    //  void  AddCallback(ImDrawCallback callback, void* callback_data);  // Your rendering function must check for 'user_callback' in ImDrawCmd and call the function instead of rendering triangles.
    pub fn AddCallback(&mut self, callback: DimgDrawCallback, callback_data: *mut c_void) {
        // // IM_ASSERT_PARANOID(CmdBuffer.size > 0);
        //     ImDrawCmd* curr_cmd = &CmdBuffer.data[CmdBuffer.size - 1];
        //     // IM_ASSERT(curr_cmd.UserCallback == None);
        //     if (curr_cmd.ElemCount != 0)
        //     {
        //         AddDrawCmd();
        //         curr_cmd = &CmdBuffer.data[CmdBuffer.size - 1];
        //     }
        //     curr_cmd.UserCallback = callback;
        //     curr_cmd.UserCallbackData = callback_data;
        //
        //     AddDrawCmd(); // Force a new command after us (see comment below)
    }
    //  void  AddDrawCmd();                                               // This is useful if you need to forcefully create a new draw call (to allow for dependent rendering / blending). Otherwise primitives are merged into the same draw-call as much as possible
    pub fn AddDrawCmd(&mut self) {
        // ImDrawCmd draw_cmd;
        //     draw_cmd.clip_rect = _CmdHeader.clip_rect;    // Same as calling ImDrawCmd_HeaderCopy()
        //     draw_cmd.TextureId = _CmdHeader.TextureId;
        //     draw_cmd.VtxOffset = _CmdHeader.VtxOffset;
        //     draw_cmd.IdxOffset = IdxBuffer.size;
        //
        //     // IM_ASSERT(draw_cmd.clip_rect.x <= draw_cmd.clip_rect.z && draw_cmd.clip_rect.y <= draw_cmd.clip_rect.w);
        //     CmdBuffer.push_back(draw_cmd);
    }
    //  ImDrawList* CloneOutput() const;                                  // Create a clone of the cmd_buffer/idx_buffer/vtx_buffer.
    pub fn CloneOutput(&mut self) -> Vec<DrawList> {
        // ImDrawList* dst = IM_NEW(ImDrawList(_Data));
        //     dst.cmd_buffer = CmdBuffer;
        //     dst.IdxBuffer = IdxBuffer;
        //     dst.VtxBuffer = VtxBuffer;
        //     dst.flags = Flags;
        //     return dst;
    }

    // Advanced: Channels
    // - Use to split render into layers. By switching channels to can render out-of-order (e.g. submit FG primitives before BG primitives)
    // - Use to minimize draw calls (e.g. if going back-and-forth between multiple clipping rectangles, prefer to append into separate channels then merge at the end)
    // - FIXME-OBSOLETE: This API shouldn't have been in ImDrawList in the first place!
    //   Prefer using your own persistent instance of ImDrawListSplitter as you can stack them.
    //   Using the ImDrawList::ChannelsXXXX you cannot stack a split over another.
    // inline void     ChannelsSplit(int count)    { _splitter.split(this, count); }
    // inline void     ChannelsMerge()             { _splitter.merge(this); }
    // inline void     ChannelsSetCurrent(int n)   { _splitter.SetCurrentChannel(this, n); }

    // Advanced: Primitives allocations
    // - We render triangles (three vertices)
    // - All primitives needs to be reserved via PrimReserve() beforehand.
    //  void  PrimReserve(int idx_count, int vtx_count);
    pub fn PrimReserve(&mut self, idx_count: usize, vtx_count: usize) {
        //  // Large mesh support (when enabled)
        //     // IM_ASSERT_PARANOID(idx_count >= 0 && vtx_count >= 0);
        //     if (sizeof(ImDrawIdx) == 2 && (_VtxCurrentIdx + vtx_count >= (1 << 16)) && (Flags & DrawListFlags::AllowVtxOffset))
        //     {
        //         // FIXME: In theory we should be testing that vtx_count <64k here.
        //         // In practice, render_text() relies on reserving ahead for a worst case scenario so it is currently useful for us
        //         // to not make that check until we rework the text functions to handle clipping and large horizontal lines better.
        //         _CmdHeader.VtxOffset = VtxBuffer.size;
        //         _OnChangedVtxOffset();
        //     }
        //
        //     ImDrawCmd* draw_cmd = &CmdBuffer.data[CmdBuffer.size - 1];
        //     draw_cmd.ElemCount += idx_count;
        //
        //     int vtx_buffer_old_size = VtxBuffer.size;
        //     VtxBuffer.resize(vtx_buffer_old_size + vtx_count);
        //     _VtxWritePtr = VtxBuffer.data + vtx_buffer_old_size;
        //
        //     int idx_buffer_old_size = IdxBuffer.size;
        //     IdxBuffer.resize(idx_buffer_old_size + idx_count);
        //     _IdxWritePtr = IdxBuffer.data + idx_buffer_old_size;
    }
    //  void  PrimUnreserve(int idx_count, int vtx_count);
    pub fn PrimUnreserve(&mut self, idx_count: usize, vtx_count: usize) {
        // ImDrawCmd* draw_cmd = &CmdBuffer.data[CmdBuffer.size - 1];
        //     draw_cmd.ElemCount -= idx_count;
        //     VtxBuffer.shrink(VtxBuffer.size - vtx_count);
        //     IdxBuffer.shrink(IdxBuffer.size - idx_count);
    }
    //  void  PrimRect(const Vector2D& a, const Vector2D& b, ImU32 col);      // Axis aligned rectangle (composed of two triangles)
    pub fn PrimRect(&mut self, a: &Vector2D, b: &Vector2D, col: u32) {
        //     Vector2D b(c.x, a.y), d(a.x, c.y), uv(_Data.TexUvWhitePixel);
        //     ImDrawIdx idx = (ImDrawIdx)_VtxCurrentIdx;
        //     _IdxWritePtr[0] = idx; _IdxWritePtr[1] = (ImDrawIdx)(idx+1); _IdxWritePtr[2] = (ImDrawIdx)(idx+2);
        //     _IdxWritePtr[3] = idx; _IdxWritePtr[4] = (ImDrawIdx)(idx+2); _IdxWritePtr[5] = (ImDrawIdx)(idx+3);
        //     _VtxWritePtr[0].pos = a; _VtxWritePtr[0].uv = uv; _VtxWritePtr[0].col = col;
        //     _VtxWritePtr[1].pos = b; _VtxWritePtr[1].uv = uv; _VtxWritePtr[1].col = col;
        //     _VtxWritePtr[2].pos = c; _VtxWritePtr[2].uv = uv; _VtxWritePtr[2].col = col;
        //     _VtxWritePtr[3].pos = d; _VtxWritePtr[3].uv = uv; _VtxWritePtr[3].col = col;
        //     _VtxWritePtr += 4;
        //     _VtxCurrentIdx += 4;
        //     _IdxWritePtr += 6;
    }
    //  void  PrimRectUV(const Vector2D& a, const Vector2D& b, const Vector2D& uv_a, const Vector2D& uv_b, ImU32 col);
    pub fn PrimRectUV(
        &mut self,
        a: &Vector2D,
        b: &Vector2D,
        uv_a: &Vector2D,
        uv_b: &Vector2D,
        col: u32,
    ) {
        //  Vector2D b(c.x, a.y), d(a.x, c.y), uv_b(uv_c.x, uv_a.y), uv_d(uv_a.x, uv_c.y);
        //     ImDrawIdx idx = (ImDrawIdx)_VtxCurrentIdx;
        //     _IdxWritePtr[0] = idx; _IdxWritePtr[1] = (ImDrawIdx)(idx+1); _IdxWritePtr[2] = (ImDrawIdx)(idx+2);
        //     _IdxWritePtr[3] = idx; _IdxWritePtr[4] = (ImDrawIdx)(idx+2); _IdxWritePtr[5] = (ImDrawIdx)(idx+3);
        //     _VtxWritePtr[0].pos = a; _VtxWritePtr[0].uv = uv_a; _VtxWritePtr[0].col = col;
        //     _VtxWritePtr[1].pos = b; _VtxWritePtr[1].uv = uv_b; _VtxWritePtr[1].col = col;
        //     _VtxWritePtr[2].pos = c; _VtxWritePtr[2].uv = uv_c; _VtxWritePtr[2].col = col;
        //     _VtxWritePtr[3].pos = d; _VtxWritePtr[3].uv = uv_d; _VtxWritePtr[3].col = col;
        //     _VtxWritePtr += 4;
        //     _VtxCurrentIdx += 4;
        //     _IdxWritePtr += 6;
    }
    //  void  PrimQuadUV(const Vector2D& a, const Vector2D& b, const Vector2D& c, const Vector2D& d, const Vector2D& uv_a, const Vector2D& uv_b, const Vector2D& uv_c, const Vector2D& uv_d, ImU32 col);
    pub fn PrimQuadUV(
        &mut self,
        a: &Vector2D,
        b: &Vector2D,
        c: &Vector2D,
        d: &Vector2D,
        uv_a: &Vector2D,
        uv_b: &Vector2D,
        uv_c: &Vector2D,
        uv_d: &Vector2D,
        col: u32,
    ) {
        //     ImDrawIdx idx = (ImDrawIdx)_VtxCurrentIdx;
        //     _IdxWritePtr[0] = idx; _IdxWritePtr[1] = (ImDrawIdx)(idx+1); _IdxWritePtr[2] = (ImDrawIdx)(idx+2);
        //     _IdxWritePtr[3] = idx; _IdxWritePtr[4] = (ImDrawIdx)(idx+2); _IdxWritePtr[5] = (ImDrawIdx)(idx+3);
        //     _VtxWritePtr[0].pos = a; _VtxWritePtr[0].uv = uv_a; _VtxWritePtr[0].col = col;
        //     _VtxWritePtr[1].pos = b; _VtxWritePtr[1].uv = uv_b; _VtxWritePtr[1].col = col;
        //     _VtxWritePtr[2].pos = c; _VtxWritePtr[2].uv = uv_c; _VtxWritePtr[2].col = col;
        //     _VtxWritePtr[3].pos = d; _VtxWritePtr[3].uv = uv_d; _VtxWritePtr[3].col = col;
        //     _VtxWritePtr += 4;
        //     _VtxCurrentIdx += 4;
        //     _IdxWritePtr += 6;
    }
    // inline    void  PrimWriteVtx(const Vector2D& pos, const Vector2D& uv, ImU32 col)    { _VtxWritePtr->pos = pos; _VtxWritePtr->uv = uv; _VtxWritePtr->col = col; _VtxWritePtr++; _VtxCurrentIdx++; }
    pub fn PrimWriteVtx(&mut self, pos: &Vector2D, uv: &Vector2D, col: u32) {
        // TODO: replace VtxWritePtr with a vector of vertices
        self.vtx_buffer.push(DrawVertex {
            pos: pos.clone(),
            uv: uv.clone(),
            col,
        });
    }
    // inline    void  PrimWriteIdx(ImDrawIdx idx)                                     { *_IdxWritePtr = idx; _IdxWritePtr++; }
    pub fn PrimWriteIdx(&mut self, idx: DrawIndex) {
        self.idx_buffer.push(idx)
    }
    // inline    void  PrimVtx(const Vector2D& pos, const Vector2D& uv, ImU32 col)         { PrimWriteIdx((ImDrawIdx)_VtxCurrentIdx); PrimWriteVtx(pos, uv, col); } // Write vertex with unique index
    pub fn PrimVtx(&mut self, pos: &Vector2D, uv: &Vector2D, col: u32) {
        self.PrimWriteVtx(pos, uv, col);
        self.PrimWriteIdx(0)
    }

    // #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
    //     inline    void  AddBezierCurve(const Vector2D& p1, const Vector2D& p2, const Vector2D& p3, const Vector2D& p4, ImU32 col, float thickness, int num_segments = 0) { AddBezierCubic(p1, p2, p3, p4, col, thickness, num_segments); } // OBSOLETED in 1.80 (Jan 2021)
    //     inline    void  PathBezierCurveTo(const Vector2D& p2, const Vector2D& p3, const Vector2D& p4, int num_segments = 0) { PathBezierCubicCurveTo(p2, p3, p4, num_segments); } // OBSOLETED in 1.80 (Jan 2021)
    // #endif

    /// Initialize before use in a new frame. We always have a command ready in the buffer.
    fn reset_for_new_frame(&mut self) {
        //     // Verify that the ImDrawCmd fields we want to memcmp() are contiguous in memory.
        //     IM_STATIC_ASSERT(IM_OFFSETOF(ImDrawCmd, ClipRect) == 0);
        //     IM_STATIC_ASSERT(IM_OFFSETOF(ImDrawCmd, TextureId) == sizeof(Vector4D));
        //     IM_STATIC_ASSERT(IM_OFFSETOF(ImDrawCmd, VtxOffset) == sizeof(Vector4D) + sizeof(ImTextureID));
        //     if (_splitter._Count > 1)
        //         _splitter.Merge(this);
        if self.splitter.count > 1 {
            self.splitter.merge(self)
        }
        //
        //     cmd_buffer.resize(0);
        self.cmd_buffer.resize(0, DrawCmd::new());
        //     idx_buffer.resize(0);
        self.idx_buffer.resize(0, 0);
        //     vtx_buffer.resize(0);
        self.vtx_buffer.resize(0, DrawVertex::default());
        //     flags = _Data->InitialFlags;
        set_hash_set(&mut self.flags, self.data.initial_flags);
        //     memset(&_cmd_header, 0, sizeof(_cmd_header));
        self._cmd_header.clear();
        //     _VtxCurrentIdx = 0;
        // self.vtx_current_idx = 0;
        //     _VtxWritePtr = None;
        //     _IdxWritePtr = None;
        //     _clip_rect_stack.resize(0);
        //     _texture_id_stack.resize(0);
        //     _path.resize(0);
        //     _splitter.clear();
        //     cmd_buffer.push_back(ImDrawCmd());
        //     _fringe_scale = 1.0;
    }
    //  void  _ClearFreeMemory();
    fn clear_free_memory(&mut self) {
    //      CmdBuffer.clear();
        self.cmd_buffer.clear();
        //     IdxBuffer.clear();
        self.idx_buffer.clear();
        //     VtxBuffer.clear();
        self.vtx_buffer.clear();
        //     Flags = DrawListFlags::None;
        self.flags.clear();
        //     _VtxCurrentIdx = 0;
        self.clip_rect_stack.clear();
        self._texture_id_stack.clear();
        self.path.clear();
        self.splitter.clear_free_memory();
        //     _VtxWritePtr = None;
        //     _IdxWritePtr = None;
        //     _ClipRectStack.clear();
        //     _TextureIdStack.clear();
        //     _Path.clear();
        //     _Splitter.ClearFreeMemory();
    }
    //  void  _PopUnusedDrawCmd();
    fn PopUnusedDrawCmd(&mut self) {
        // if (CmdBuffer.size == 0)
        //         return;
        //     ImDrawCmd* curr_cmd = &CmdBuffer.data[CmdBuffer.size - 1];
        //     if (curr_cmd.ElemCount == 0 && curr_cmd.UserCallback == None)
        //         CmdBuffer.pop_back();
    }
    //  void  _TryMergeDrawCmds();
    fn TryMergeDrawCmds(&mut self) {
        // // IM_ASSERT_PARANOID(CmdBuffer.size > 0);
        //     ImDrawCmd* curr_cmd = &CmdBuffer.data[CmdBuffer.size - 1];
        //     ImDrawCmd* prev_cmd = curr_cmd - 1;
        //     if (ImDrawCmd_HeaderCompare(curr_cmd, prev_cmd) == 0 && ImDrawCmd_AreSequentialIdxOffset(prev_cmd, curr_cmd) && curr_cmd.UserCallback == None && prev_cmd.UserCallback == None)
        //     {
        //         prev_cmd.ElemCount += curr_cmd.ElemCount;
        //         CmdBuffer.pop_back();
        //     }
    }
    //  void  _OnChangedClipRect();
    fn OnChangedClipRect(&mut self) {

        //  // If current command is used with different settings we need to add a new command
        //     // IM_ASSERT_PARANOID(CmdBuffer.size > 0);
        //     ImDrawCmd* curr_cmd = &CmdBuffer.data[CmdBuffer.size - 1];
        //     if (curr_cmd.ElemCount != 0 && memcmp(&curr_cmd.clip_rect, &_CmdHeader.clip_rect, sizeof(Vector4D)) != 0)
        //     {
        //         AddDrawCmd();
        //         return;
        //     }
        //     // IM_ASSERT(curr_cmd.UserCallback == None);
        //
        //     // Try to merge with previous command if it matches, else use current command
        //     ImDrawCmd* prev_cmd = curr_cmd - 1;
        //     if (curr_cmd.ElemCount == 0 && CmdBuffer.size > 1 && ImDrawCmd_HeaderCompare(&_CmdHeader, prev_cmd) == 0 && ImDrawCmd_AreSequentialIdxOffset(prev_cmd, curr_cmd) && prev_cmd.UserCallback == None)
        //     {
        //         CmdBuffer.pop_back();
        //         return;
        //     }
        //
        //     curr_cmd.clip_rect = _CmdHeader.clip_rect;
    }
    //  void  _OnChangedTextureID();
    fn OnChangedTextureID(&mut self) {
        // // If current command is used with different settings we need to add a new command
        //     // IM_ASSERT_PARANOID(CmdBuffer.size > 0);
        //     ImDrawCmd* curr_cmd = &CmdBuffer.data[CmdBuffer.size - 1];
        //     if (curr_cmd.ElemCount != 0 && curr_cmd.TextureId != _CmdHeader.TextureId)
        //     {
        //         AddDrawCmd();
        //         return;
        //     }
        //     // IM_ASSERT(curr_cmd.UserCallback == None);
        //
        //     // Try to merge with previous command if it matches, else use current command
        //     ImDrawCmd* prev_cmd = curr_cmd - 1;
        //     if (curr_cmd.ElemCount == 0 && CmdBuffer.size > 1 && ImDrawCmd_HeaderCompare(&_CmdHeader, prev_cmd) == 0 && ImDrawCmd_AreSequentialIdxOffset(prev_cmd, curr_cmd) && prev_cmd.UserCallback == None)
        //     {
        //         CmdBuffer.pop_back();
        //         return;
        //     }
        //
        //     curr_cmd.TextureId = _CmdHeader.TextureId;
    }
    //  void  _OnChangedVtxOffset();
    fn OnChangedVtxOffset(&mut self) {
        // // We don't need to compare curr_cmd->vtx_offset != _cmd_header.vtx_offset because we know it'll be different at the time we call this.
        //     _VtxCurrentIdx = 0;
        //     // IM_ASSERT_PARANOID(CmdBuffer.size > 0);
        //     ImDrawCmd* curr_cmd = &CmdBuffer.data[CmdBuffer.size - 1];
        //     //IM_ASSERT(curr_cmd->vtx_offset != _cmd_header.vtx_offset); // See #3349
        //     if (curr_cmd.ElemCount != 0)
        //     {
        //         AddDrawCmd();
        //         return;
        //     }
        //     // IM_ASSERT(curr_cmd.UserCallback == None);
        //     curr_cmd.VtxOffset = _CmdHeader.VtxOffset;
    }
    //  int   _CalcCircleAutoSegmentCount(float radius) const;
    fn CalCircleAUtoSegmentCount(&mut self, radius: f32) -> i32 {
        // // Automatic segment count
        //     let radius_idx = (radius + 0.999999); // ceil to never reduce accuracy
        //     if (radius_idx < IM_ARRAYSIZE(_Data.CircleSegmentCounts))
        //         return _Data.CircleSegmentCounts[radius_idx]; // Use cached value
        //     else
        //         return IM_DRAWLIST_CIRCLE_AUTO_SEGMENT_CALC(radius, _Data.CircleSegmentMaxError);
    }
    //  void  _PathArcToFastEx(const Vector2D& center, float radius, int a_min_sample, int a_max_sample, int a_step);
    fn PathArcToFastEx(
        &mut self,
        center: &Vector2D,
        radius: f32,
        a_min_simple: i32,
        a_max_sample: i32,
        a_step: i32,
    ) {
        //  if (radius < 0.5)
        //     {
        //         _Path.push_back(center);
        //         return;
        //     }
        //
        //     // Calculate arc auto segment step size
        //     if (a_step <= 0)
        //         a_step = IM_DRAWLIST_ARCFAST_SAMPLE_MAX / _CalcCircleAutoSegmentCount(radius);
        //
        //     // Make sure we never do steps larger than one quarter of the circle
        //     a_step = ImClamp(a_step, 1, IM_DRAWLIST_ARCFAST_TABLE_SIZE / 4);
        //
        //     let sample_range = ImAbs(a_max_sample - a_min_sample);
        //     let a_next_step = a_step;
        //
        //     int samples = sample_range + 1;
        //     bool extra_max_sample = false;
        //     if (a_step > 1)
        //     {
        //         samples            = sample_range / a_step + 1;
        //         let overstep = sample_range % a_step;
        //
        //         if (overstep > 0)
        //         {
        //             extra_max_sample = true;
        //             samples += 1;
        //
        //             // When we have overstep to avoid awkwardly looking one long line and one tiny one at the end,
        //             // distribute first step range evenly between them by reducing first step size.
        //             if (sample_range > 0)
        //                 a_step -= (a_step - overstep) / 2;
        //         }
        //     }
        //
        //     _Path.resize(_Path.size + samples);
        //     Vector2D* out_ptr = _Path.data + (_Path.size - samples);
        //
        //     int sample_index = a_min_sample;
        //     if (sample_index < 0 || sample_index >= IM_DRAWLIST_ARCFAST_SAMPLE_MAX)
        //     {
        //         sample_index = sample_index % IM_DRAWLIST_ARCFAST_SAMPLE_MAX;
        //         if (sample_index < 0)
        //             sample_index += IM_DRAWLIST_ARCFAST_SAMPLE_MAX;
        //     }
        //
        //     if (a_max_sample >= a_min_sample)
        //     {
        //         for (int a = a_min_sample; a <= a_max_sample; a += a_step, sample_index += a_step, a_step = a_next_step)
        //         {
        //             // a_step is clamped to IM_DRAWLIST_ARCFAST_SAMPLE_MAX, so we have guaranteed that it will not wrap over range twice or more
        //             if (sample_index >= IM_DRAWLIST_ARCFAST_SAMPLE_MAX)
        //                 sample_index -= IM_DRAWLIST_ARCFAST_SAMPLE_MAX;
        //
        //             const Vector2D s = _Data.ArcFastVtx[sample_index];
        //             out_ptr.x = center.x + s.x * radius;
        //             out_ptr.y = center.y + s.y * radius;
        //             out_ptr += 1;
        //         }
        //     }
        //     else
        //     {
        //         for (int a = a_min_sample; a >= a_max_sample; a -= a_step, sample_index -= a_step, a_step = a_next_step)
        //         {
        //             // a_step is clamped to IM_DRAWLIST_ARCFAST_SAMPLE_MAX, so we have guaranteed that it will not wrap over range twice or more
        //             if (sample_index < 0)
        //                 sample_index += IM_DRAWLIST_ARCFAST_SAMPLE_MAX;
        //
        //             const Vector2D s = _Data.ArcFastVtx[sample_index];
        //             out_ptr.x = center.x + s.x * radius;
        //             out_ptr.y = center.y + s.y * radius;
        //             out_ptr += 1;
        //         }
        //     }
        //
        //     if (extra_max_sample)
        //     {
        //         int normalized_max_sample = a_max_sample % IM_DRAWLIST_ARCFAST_SAMPLE_MAX;
        //         if (normalized_max_sample < 0)
        //             normalized_max_sample += IM_DRAWLIST_ARCFAST_SAMPLE_MAX;
        //
        //         const Vector2D s = _Data.ArcFastVtx[normalized_max_sample];
        //         out_ptr.x = center.x + s.x * radius;
        //         out_ptr.y = center.y + s.y * radius;
        //         out_ptr += 1;
        //     }
        //
        //     // IM_ASSERT_PARANOID(_Path.data + _Path.size == out_ptr);
    }
    //  void  _PathArcToN(const Vector2D& center, float radius, float a_min, float a_max, int num_segments);
    fn PathArcToN(
        &mut self,
        center: &Vector2D,
        radius: f32,
        a_min: f32,
        a_max: f32,
        num_segments: i32,
    ) {
        // if (radius < 0.5)
        //     {
        //         _Path.push_back(center);
        //         return;
        //     }
        //
        //     // Note that we are adding a point at both a_min and a_max.
        //     // If you are trying to draw a full closed circle you don't want the overlapping points!
        //     _Path.reserve(_Path.size + (num_segments + 1));
        //     for (int i = 0; i <= num_segments; i += 1)
        //     {
        //         let a = a_min + ((float)i / num_segments) * (a_max - a_min);
        //         _Path.push_back(Vector2D::new(center.x + ImCos(a) * radius, center.y + ImSin(a) * radius));
        //     }
    }
}

// ImDrawList: Helper function to calculate a circle's segment count given its radius and a "maximum error" value.
// Estimation of number of circle segment based on error is derived using method described in https://stackoverflow.com/a/2244088/15194693
// Number of segments (N) is calculated using equation:
//   N = ceil ( pi / acos(1 - error / r) )     where r > 0, error <= r
// Our equation is significantly simpler that one in the post thanks for choosing segment that is
// perpendicular to x axis. Follow steps in the article from this starting condition and you will
// will get this result.
//
// Rendering circles with an odd number of segments, while mathematically correct will produce
// asymmetrical results on the raster grid. Therefore we're rounding N to next even number (7->8, 8->8, 9->10 etc.)
// #define IM_ROUNDUP_TO_EVEN(_V)                                  ((((_V) + 1) / 2) * 2)
// #define IM_DRAWLIST_CIRCLE_AUTO_SEGMENT_MIN                     4
pub const DIMG_DRAW_LIST_CIRCLE_AUTO_SEGMENT_MIN: f32 = 4.0;
// #define IM_DRAWLIST_CIRCLE_AUTO_SEGMENT_MAX                     512
pub const DIMG_DRAW_LIST_CIRCLE_AUTO_SEGMENT_MAX: f32 = 512.0;

// #define IM_DRAWLIST_CIRCLE_AUTO_SEGMENT_CALC(_RAD,_MAXERROR)    ImClamp(IM_ROUNDUP_TO_EVEN(ImCeil(f32::PI / ImAcos(1 - ImMin((_MAXERROR), (_RAD)) / (_RAD)))), IM_DRAWLIST_CIRCLE_AUTO_SEGMENT_MIN, IM_DRAWLIST_CIRCLE_AUTO_SEGMENT_MAX)
pub fn drawlist_circle_auto_segment_calc(radius: f32, max_error: f32) -> f32 {
    f32::clamp(
        f32::round(f32::ceil(
            PI / f32::acos(1 - f32::min(max_error, (radius)) / (radius)),
        )),
        DIMG_DRAW_LIST_CIRCLE_AUTO_SEGMENT_MIN,
        DIMG_DRAW_LIST_CIRCLE_AUTO_SEGMENT_MAX,
    )
}

// Raw equation from IM_DRAWLIST_CIRCLE_AUTO_SEGMENT_CALC rewritten for 'r' and 'error'.
// #define IM_DRAWLIST_CIRCLE_AUTO_SEGMENT_CALC_R(_N,_MAXERROR)    ((_MAXERROR) / (1 - ImCos(f32::PI / ImMax((float)(_N), f32::PI))))
pub fn drawlist_circle_auto_segment_calc_r(n: f32, max_error: f32) -> f32 {
    ((max_error) / (1 - f32::cos(f32::PI / f32::max(n, f32::PI))))
}

// #define IM_DRAWLIST_CIRCLE_AUTO_SEGMENT_CALC_ERROR(_N,_RAD)     ((1 - ImCos(f32::PI / ImMax((float)(_N), f32::PI))) / (_RAD))
pub fn drawlist_circl_auto_segment_calc_error(n: f32, rad: f32) -> f32 {
    ((1 - f32::cos(f32::PI / f32::max(n, f32::PI))) / rad)
}

// ImDrawList: Lookup table size for adaptive arc drawing, cover full circle.
// #ifndef IM_DRAWLIST_ARCFAST_TABLE_SIZE
// #define IM_DRAWLIST_ARCFAST_TABLE_SIZE                          48 // Number of samples in lookup table.
pub const DRAW_LIST_ARCFAST_TABLE_SIZE: usize = 48usize;
// #endif
// #define IM_DRAWLIST_ARCFAST_SAMPLE_MAX                          IM_DRAWLIST_ARCFAST_TABLE_SIZE // Sample index _PathArcToFastEx() for 360 angle.
pub const DRAW_LIST_ARCFAST_SAMPLE_MAX: usize = DRAW_LIST_ARCFAST_TABLE_SIZE;

// flags for ImDrawList instance. Those are set automatically by ImGui:: functions from ImGuiIO settings, and generally not manipulated directly.
// It is however possible to temporarily alter flags between calls to ImDrawList:: functions.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum DrawListFlags {
    None = 0,
    anti_aliased_lines, // Enable anti-aliased lines/borders (*2 the number of triangles for 1.0 wide line or lines thin enough to be drawn using textures, otherwise *3 the number of triangles)
    anti_aliased_lines_use_tex, // Enable anti-aliased lines/borders using textures when possible. Require backend to render with bilinear filtering (NOT point/nearest filtering).
    anti_aliased_fill, // Enable anti-aliased edge around filled shapes (rounded rectangles, circles).
    AllowVtxOffset = 1 << 3, // Can emit 'vtx_offset > 0' to allow large meshes. Set when 'ImGuiBackendFlags_RendererHasVtxOffset' is enabled.
}

// The maximum line width to bake anti-aliased textures for. build atlas with NoBakedLines to disable baking.
// #ifndef IM_DRAWLIST_TEX_LINES_WIDTH_MAX
// #define IM_DRAWLIST_TEX_LINES_WIDTH_MAX     (63)
// #endif
pub const IM_DRAWLIST_TEX_LINES_WIDTH_MAX: usize = 63;

/// static ImDrawList* get_viewport_draw_list(ImGuiViewportP* viewport, size_t drawlist_no, const char* drawlist_name)
pub fn get_viewport_draw_list(
    g: &mut Context,
    viewport: &mut Viewport,
    drawlist_no: usize,
    drawlist_name: &String,
) -> &mut DrawList {
    // Create the draw list on demand, because they are not frequently used for all viewports
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(drawlist_no < IM_ARRAYSIZE(viewport->DrawLists));
    // ImDrawList* draw_list = viewport->DrawLists[drawlist_no];
    let draw_list = &mut viewport.draw_lists[drawlist_no];
    if draw_list.id == INVALID_ID {
        // draw_list = IM_NEW(ImDrawList)(&g.DrawListSharedData);
        viewport.draw_lists[drawlist_no] = DrawList::new(&mut g.draw_list_shared_data);
        // draw_list->_OwnerName = drawlist_name;
        viewport.draw_lists[drawlist_no].owner_name = drawlist_name.clone();
        // viewport.draw_lists[drawlist_no] = draw_list;
    }

    // Our ImDrawList system requires that there is always a command
    if viewport.draw_lists_last_frame[drawlist_no] != g.frame_count {
        draw_list.ResetForNewFrame();
        draw_list.PushTextureID(g.io.fonts.TexID);
        draw_list.PushClipRect(viewport.pos, viewport.pos + viewport.size, false);
        viewport.draw_lists_last_frame[drawlist_no] = g.frame_count;
    }
    return draw_list;
}

/// ImDrawList* ImGui::GetBackgroundDrawList(ImGuiViewport* viewport)
pub fn get_background_draw_list(g: &mut Context, viewport: &mut Viewport) -> &mut DrawList {
    return get_viewport_draw_list(g, viewport, 0, &String::from("##Background"));
}

/// ImDrawList* ImGui::GetBackgroundDrawList()
pub fn get_background_draw_list2(g: &mut Context) -> &mut DrawList {
    // ImGuiContext& g = *GImGui;
    //return GetBackgroundDrawList(g.CurrentWindow->Viewport);
    let curr_win = g.get_current_window()?;
    let vp = g.get_viewport(curr_win.viewport_id).unwrap();
    get_background_draw_list(g, vp)
}

/// ImDrawList* ImGui::GetForegroundDrawList(ImGuiViewport* viewport)
pub fn get_foreground_draw_list(g: &mut Context, viewport: &mut Viewport) -> &mut DrawList {
    // return GetViewportDrawList((ImGuiViewportP*)viewport, 1, "##Foreground");
    get_viewport_draw_list(g, viewport, 1, &String::from("##Foreground"))
}

/// ImDrawList* ImGui::GetForegroundDrawList()
pub fn get_foreground_draw_list2(g: &mut Context) -> &mut DrawList {
    // ImGuiContext& g = *GImGui;
    // return GetForegroundDrawList(g.CurrentWindow->Viewport);
    let curr_win = g.get_current_window()?;
    let vp = g.get_viewport(curr_win.viewport_id).unwrap();
    get_foreground_draw_list(g, vp)
}

// static void add_draw_list_to_draw_data(ImVector<ImDrawList*>* out_list, ImDrawList* draw_list)
pub fn add_draw_list_to_draw_data(g: &mut Context, out_list: &mut Vec<Id32>, draw_list_id: Id32) {
    let draw_list = g.get_draw_list(draw_list_id).unwrap();
    if draw_list.cmd_buffer.is_empty() {
        return;
    }

    if draw_list.cmd_buffer.size == 1
        && draw_list.cmd_buffer[0].elem_count == 0
        && draw_list.cmd_buffer[0].user_callback.is_none()
    {
        return;
    }

    // Draw list sanity check. Detect mismatch between PrimReserve() calls and incrementing _VtxCurrentIdx, _VtxWritePtr etc.
    // May trigger for you if you are using PrimXXX functions incorrectly.
    // IM_ASSERT(draw_list->VtxBuffer.Size == 0 || draw_list->_VtxWritePtr == draw_list->VtxBuffer.Data + draw_list->VtxBuffer.Size);
    // IM_ASSERT(draw_list->IdxBuffer.Size == 0 || draw_list->_IdxWritePtr == draw_list->IdxBuffer.Data + draw_list->IdxBuffer.Size);
    if !(draw_list.flags.contains(&DrawListFlags::AllowVtxOffset)) {
        // IM_ASSERT(draw_list->_VtxCurrentIdx == draw_list->VtxBuffer.Size);
    }

    // Check that draw_list doesn't use more vertices than indexable (default ImDrawIdx = unsigned short = 2 bytes = 64K vertices per ImDrawList = per window)
    // If this assert triggers because you are drawing lots of stuff manually:
    // - First, make sure you are coarse clipping yourself and not trying to draw many things outside visible bounds.
    //   Be mindful that the ImDrawList API doesn't filter vertices. Use the Metrics/Debugger window to inspect draw list contents.
    // - If you want large meshes with more than 64K vertices, you can either:
    //   (A) Handle the ImDrawCmd::vtx_offset value in your renderer backend, and set 'io.backend_flags |= ImGuiBackendFlags_RendererHasVtxOffset'.
    //       Most example backends already support this from 1.71. Pre-1.71 backends won't.
    //       Some graphics API such as GL ES 1/2 don't have a way to offset the starting vertex so it is not supported for them.
    //   (B) Or handle 32-bit indices in your renderer backend, and uncomment '#define ImDrawIdx unsigned int' line in imconfig.h.
    //       Most example backends already support this. For example, the OpenGL example code detect index size at compile-time:
    //         glDrawElements(GL_TRIANGLES, (GLsizei)pcmd->elem_count, sizeof(ImDrawIdx) == 2 ? GL_UNSIGNED_SHORT : GL_UNSIGNED_INT, idx_buffer_offset);
    //       Your own engine or render API may use different parameters or function calls to specify index sizes.
    //       2 and 4 bytes indices are generally supported by most graphics API.
    // - If for some reason neither of those solutions works for you, a workaround is to call BeginChild()/EndChild() before reaching
    //   the 64K limit to split your draw commands in multiple draw lists.
    if size_of::<DrawIdx>() == 2 {
        // IM_ASSERT(draw_list->_VtxCurrentIdx < (1 << 16) && "Too many vertices in ImDrawList using 16-bit indices. Read comment above");
    }

    out_list.push_back(draw_list);
}
