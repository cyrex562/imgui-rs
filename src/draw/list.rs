use crate::context::Context;
use crate::draw::command::{DrawCommandHeader, DrawCmd};
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
use crate::color::COLOR32_A_MASK;
use crate::draw::bezier::{bezier_cubic_calc, bezier_quadratic_calc};
use crate::draw::flags::{DrawFlags, fix_rect_corner_flags};
use crate::draw::{DrawCallback, ROUND_CORNERS_MASK};
use crate::window::clip::push_clip_rect;

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
    pub self.vtx_current_idx: usize,
    // const ImDrawListSharedData* _Data;          // Pointer to shared draw data (you can use ImGui::GetDrawListSharedData() to get the one from current ImGui context)
    pub data: DrawListSharedData,
    // const char*             _OwnerName;         // Pointer to owner window's name for debugging
    pub owner_name: String,
    // ImDrawVert*             _VtxWritePtr;       // [Internal] point within vtx_buffer.data after each add command (to avoid using the ImVector<> operators too much)
    pub vtx_write_offset: usize,
    // pub _VxWritePtr: *mut ImDrawVert,
    // ImDrawIdx*              _IdxWritePtr;       // [Internal] point within idx_buffer.data after each add command (to avoid using the ImVector<> operators too much)
    pub idx_write_offset: usize,
    // pub _IdxWritePtr: *mut ImDrawIdx,
    // ImVector<Vector4D>        _clip_rect_stack;     // [Internal]
    pub clip_rect_stack: Vec<Rect>,
    // ImVector<ImTextureID>   _texture_id_stack;    // [Internal]
    pub texture_id_stack: Vec<TextureId>,
    // ImVector<Vector2D>        _path;              // [Internal] current path building
    pub path: Vec<Vector2D>,
    // ImDrawCmdHeader         _cmd_header;         // [Internal] template of active commands. Fields should match those of cmd_buffer.back().
    pub command_header: DrawCommandHeader,
    // ImDrawListSplitter      _splitter;          // [Internal] for channels api (note: prefer using your own persistent instance of ImDrawListSplitter!)
    pub splitter: DrawListSplitter,
    // pub _fringe_scale: f32,      // [Internal] anti-alias fringe is scaled by this value, this helps to keep things sharp while zooming at vertex buffer content
    pub fringe_scale: f32,
}

impl DrawList {
    // If you want to create ImDrawList instances, pass them ImGui::GetDrawListSharedData() or create and use your own ImDrawListSharedData (so you can use ImDrawList without ImGui)
    // ImDrawList(const ImDrawListSharedData* shared_data) { memset(this, 0, sizeof(*this)); _Data = shared_data; }
    pub fn new(shared_data: &mut DrawListSharedData) -> Self {
        Self {
            data: shared_data.clone(),
            ..Default::default()
        }
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
        let mut cr = Vector4D::new(cr_min.x, cr_min.y, cr_max.y, 0.0);
            if intersect_with_current_clip_rect
            {
                // Vector4D current = _CmdHeader.clip_rect;
                let current = &self.command_header.clip_rect;
                if cr.x < current.x { cr.x = current.x; }
                if cr.y < current.y { cr.y = current.y; }
                if cr.z > current.z { cr.z = current.z; }
                if cr.w > current.w { cr.w = current.w; }
            }
            cr.z = f32::max(cr.x, cr.z);
            cr.w = f32::max(cr.y, cr.w);

            self.clip_rect_stack.push_back(cr);
            self.command_header.clip_rect = cr.to_owned();
            on_changed_clip_rect();
    }
    //  void  push_clip_rect_full_screen();
    pub fn push_clip_rect_full_screen(&mut self, g: &mut Context) {
        push_clip_rect(g,
                       &Vector2D::new(self.data.clip_rect_full_screen.x,
                                      self.data.clip_rect_full_screen.y),
                       &Vector2D::new(self.data.clip_rect_full_screen.z,
                                     self.data.clip_rect_full_screen.w),
        false);
    }
    //  void  pop_clip_rect();
    pub fn pop_clip_rect(&mut self) {
        self.clip_rect_stack.pop_back();
            self.command_header.clip_rect = if self.clip_rect_stack.is_empty() {
                self.data.clip_rect_full_screen.clone() } else {
                self.clip_rect_stack.data[self.clip_rect_stack.len() - 1] };
            on_changed_clip_rect();
    }
    //  void  push_texture_id(ImTextureID texture_id);
    pub fn push_texture_id(&mut self, texture_id: TextureId) {
            self.texture_id_stack.push_back(texture_id);
            self.command_header.texture_id = texture_id;
            on_changed_texture_id();
    }
    //  void  pop_texture_id();
    pub fn pop_texture_id(&mut self) {
            self.texture_id_stack.pop_back();
            self.command_header.texture_id = if (self.texture_id_stack.is_empty) {
                INVALID_ID } else {
                self.texture_id_stack.data[self.texture_id_stack.size - 1] };
            on_changed_texture_id();
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
    pub fn add_line(&mut self, p1: &Vector2D, p2: &Vector2D, color: u32, thickness: f32) {
        if (color & COLOR32_A_MASK) == 0 {
            return;
        }
            path_line_to(p1 + Vector2D::new(0.5, 0.5));
            path_line_to(p2 + Vector2D::new(0.5, 0.5));
            path_stroke(color, 0, thickness);
    }
    //  void  add_rect(const Vector2D& p_min, const Vector2D& p_max, ImU32 col, float rounding = 0.0, ImDrawFlags flags = 0, float thickness = 1.0);   // a: upper-left, b: lower-right (== upper-left + size)
    pub fn add_rect(
        &mut self,
        p_min: &Vector2D,
        p_max: Vector2D,
        color: u32,
        rounding: f32,
        flags: Option<&HashSet<DrawFlags>>,
        thickness: f32,
    ) {
         if (color & COLOR32_A_MASK) == 0 {
             return;
         }
            if self.flags.contains(&DrawListFlags::AntiAliasedLines) {
                self.path_rect(p_min + Vector2D::new(0.50, 0.50), &(p_max - Vector2D::new(0.50, 0.50)), rounding, flags);
            }
            else {
                self.path_rect(p_min + Vector2D::new(0.50, 0.50), &(p_max - Vector2D::new(0.49, 0.49)), rounding, flags);
            } // Better looking lower-right corner and rounded non-AA shapes.
            self.path_stroke(color, DrawFlags::Closed, thickness);
    }
    //  void  add_rect_filled(const Vector2D& p_min, const Vector2D& p_max, ImU32 col, float rounding = 0.0, ImDrawFlags flags = 0);                     // a: upper-left, b: lower-right (== upper-left + size)
    pub fn add_rect_filled(
        &mut self,
        p_min: &Vector2D,
        p_max: &Vector2D,
        color: u32,
        rounding: f32,
        flags: Option<&HashSet<DrawFlags>>,
    ) {
         if (color & COLOR32_A_MASK) == 0 {
             return;
         }
            if rounding < 0.5 || flags.unwrap().contains(&DrawFlags::RoundCornersNone)
            {
                self.prim_reserve(6, 4);
                self.prim_rect(p_min, p_max, color);
            }
            else
            {
                self.path_rect(p_min, p_max, rounding, flags);
                self.path_fill_convex(color);
            }
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
         if ((col_upr_left | col_upr_right | col_bot_right | col_bot_left) & COLOR32_A_MASK) == 0 {
             return;
         }

            let uv = &self.data.TexUvWhitePixel;
            self.prim_reserve(6, 4);
            self.prim_write_idx((self.vtx_current_idx));
        self.prim_write_idx((self.vtx_current_idx + 1));
        self.prim_write_idx((self.vtx_current_idx + 2));
            self.prim_write_idx((self.vtx_current_idx));
        self.prim_write_idx((self.vtx_current_idx + 2));
        self.prim_write_idx((self.vtx_current_idx + 3));
            self.prime_write_vtx(p_min, uv, col_upr_left);
            self.prime_write_vtx(&Vector2D::new(p_max.x, p_min.y), uv, col_upr_right);
            self.prime_write_vtx(p_max, uv, col_bot_right);
            self.prime_write_vtx(&Vector2D::new(p_min.x, p_max.y), uv, col_bot_left);
    }
    //  void  add_quad(const Vector2D& p1, const Vector2D& p2, const Vector2D& p3, const Vector2D& p4, ImU32 col, float thickness = 1.0);
    pub fn add_quad(
        &mut self,
        p1: &Vector2D,
        p2: &Vector2D,
        p3: &Vector2D,
        p4: &Vector2D,
        color: u32,
        thickness: f32,
    ) {
         if (color & COLOR32_A_MASK) == 0 {
             return;
         }

            self.path_line_to(p1);
            self.path_line_to(p2);
            self.path_line_to(p3);
            self.path_line_to(p4);
            self.path_stroke(color, DrawFlags::Closed, thickness);
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
         if (col & COLOR32_A_MASK) == 0 {
             return;
         }

            self.path_line_to(p1);
            self.path_line_to(p2);
            self.path_line_to(p3);
            self.path_line_to(p4);
            self.path_fill_convex(col);
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
        if (col & COLOR32_A_MASK) == 0 {
            return;
        }

            self.path_line_to(p1);
            self.path_line_to(p2);
            self.path_line_to(p3);
            self.path_stroke(col, DrawFlags::Closed, thickness);
    }
    //  void  add_triangle_filled(const Vector2D& p1, const Vector2D& p2, const Vector2D& p3, ImU32 col);
    pub fn add_triangle_filled(&mut self, p1: &Vector2D, p2: &Vector2D, p3: &Vector2D, col: u32) {
          if (col & COLOR32_A_MASK) == 0 {
              return;
          }

            self.path_line_to(p1);
            self.path_line_to(p2);
            self.path_line_to(p3);
            self.path_fill_convex(col);
    }
    //  void  add_circle(const Vector2D& center, float radius, ImU32 col, int num_segments = 0, float thickness = 1.0);
    pub fn add_circle(
        &mut self,
        center: &Vector2D,
        radius: f32,
        col: u32,
        mut num_segments: i32,
        thickness: f32,
    ) {
         if (col & COLOR32_A_MASK) == 0 || radius < 0.5 {
             return;
         }

            if num_segments <= 0
            {
                // Use arc with automatic segment count
                self.path_arc_to_fast_ex(center, radius - 0.5, 0, DRAW_LIST_ARCFAST_SAMPLE_MAX, 0);
                self.path.size -= 1 ;
            }
            else
            {
                // Explicit segment count (still clamp to avoid drawing insanely tessellated shapes)
                num_segments = i32::clamp(num_segments, 3, DRAW_LIST_CIRCLE_AUTO_SEGMENT_MAX);

                // Because we are filling a closed shape we remove 1 from the count of segments/points
                let a_max = (f32::PI * 2.0) * (num_segments - 1.0) / num_segments;
                self.path_arc_to(center, radius - 0.5, 0.0, a_max, num_segments - 1);
            }

            self.path_stroke(col, DrawFlags::Closed, thickness);
    }
    //  void  add_circle_filled(const Vector2D& center, float radius, ImU32 col, int num_segments = 0);
    pub fn add_circle_filled(
        &mut self,
        center: &Vector2D,
        radius: f32,
        col: u32,
        mut num_segments: i32,
    ) {
        if (col & COLOR32_A_MASK) == 0 || radius < 0.5 {
            return;
        }

            if num_segments <= 0
            {
                // Use arc with automatic segment count
                self.path_arc_to_fast_ex(center, radius, 0, DRAW_LIST_ARCFAST_SAMPLE_MAX, 0);
                self.path.size -= 1 ;
            }
            else
            {
                // Explicit segment count (still clamp to avoid drawing insanely tessellated shapes)
                num_segments = i32::clamp(num_segments, 3, DRAW_LIST_CIRCLE_AUTO_SEGMENT_MAX);

                // Because we are filling a closed shape we remove 1 from the count of segments/points
                let a_max = (f32::PI * 2.0) * (num_segments - 1.0) / num_segments;
                self.path_arc_to(center, radius, 0.0, a_max, num_segments - 1);
            }

            self.path_fill_convex(col);
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
         if (col & COLOR32_A_MASK) == 0 || num_segments <= 2 {
             return;
         }

            // Because we are filling a closed shape we remove 1 from the count of segments/points
            let a_max = (f32::PI * 2.0) * (num_segments - 1.0) / num_segments;
            self.path_arc_to(center, radius - 0.5, 0.0, a_max, num_segments - 1);
            self.path_stroke(col, DrawFlags::Closed, thickness);
    }
    //  void  AddNgonFilled(const Vector2D& center, float radius, ImU32 col, int num_segments);
    pub fn add_ngon_filled(&mut self, center: &Vector2D, radius: f32, col: u32, num_segments: i32) {
            if (col & COLOR32_A_MASK) == 0 || num_segments <= 2 {
                return;
            }

            // Because we are filling a closed shape we remove 1 from the count of segments/points
            let a_max = (f32::PI * 2.0) * (num_segments - 1.0) / num_segments;
            self.path_arc_to(center, radius, 0.0, a_max, num_segments - 1);
            self.path_fill_convex(col);
    }
    //  void  add_text(const Vector2D& pos, ImU32 col, const char* text_begin, const char* text_end = None);
    pub fn add_text(&mut self, pos: &Vector2D, color: u32, text: &String) {
        if (color & COLOR32_A_MASK) == 0 {
            return;
        }

        // if (text_end == None) {
        //     text_end = text_begin + strlen(text_begin);
        // }
        // if (text_begin == text_end) {
        //     return;
        // }

        // Pull default font/size from the shared ImDrawListSharedData instance
        if self.font == None {
            self.font = self.data.font;
        }
        if self.font_size == 0.0 {
            self.font_size = self.data.font_size;
        }

        // IM_ASSERT(font.container_atlas.TexID == _CmdHeader.TextureId);  // Use high-level ImGui::PushFont() or low-level ImDrawList::PushTextureId() to change font.

        // Vector4D
        let mut clip_rect = self.command_header.clip_rect.clone();
        if self.cpu_fine_clip_rect {
            clip_rect.x = f32::max(clip_rect.x, cpu_fine_clip_rect.x);
            clip_rect.y = f32::max(clip_rect.y, cpu_fine_clip_rect.y);
            clip_rect.z = f32::min(clip_rect.z, cpu_fine_clip_rect.z);
            clip_rect.w = f32::min(clip_rect.w, cpu_fine_clip_rect.w);
        }
        self.font.render_text(self, self.font_size, pos, color, clip_rect, text, self.wrap_width, self.cpu_fine_clip_rect != None);
    }
    //  void  add_text(const ImFont* font, float font_size, const Vector2D& pos, ImU32 col, const char* text_begin, const char* text_end = None, float wrap_width = 0.0, const Vector4D* cpu_fine_clip_rect = None);
    pub fn add_text_2(
        &mut self,
        font: &Font,
        font_size: f32,
        pos: &Vector2D,
        color: u32,
        text: &String,
        wrap_width: f32,
        cpu_fine_clip_rect: Option<&Vector4D>,
    ) {
        // self.add_text(None, 0.0, pos, col, text_begin, text_end);
        self.add_text(pos, color, text);
    }
    //  void  add_polyline(const Vector2D* points, int num_points, ImU32 col, ImDrawFlags flags, float thickness);
    pub fn add_polyline(
        &mut self,
        points: &Vec<Vector2D>,
        points_count: usize,
        color: u32,
        flags: &HashSet<DrawFlags>,
        mut thickness: f32
    ) {
        if points_count < 2 {
            return;
        }

            // const bool closed = (flags & DrawFlags::Closed) != 0;
            let closed = flags.contains(&DrawFlags::Closed);
            // const Vector2D opaque_uv = data.TexUvWhitePixel;
        let opaque_uv = &self.data.text_uv_white_pixel;
        // let count = closed ? points_count : points_count - 1; // The number of line segments we need to draw
        let count = if closed { points_count } else { points_count - 1};
            // const bool thick_line = (thickness > _FringeScale);
        let thick_line = thickness > self.fringe_scale;

            // if (Flags & DrawListFlags::AntiAliasedLines)
        if self.flags.contains(&DrawListFlags::AntiAliasedLines)
        {
                // Anti-aliased stroke
                let aa_size = self.fringe_scale;
                // const ImU32 col_trans = color & ~COLOR32_A_MASK;
                let col_trans = color & !COLOR32_A_MASK;

                // Thicknesses <1.0 should behave like thickness 1.0
                thickness = f32::max(thickness, 1.0);
                let integer_thickness = thickness;
                let fractional_thickness = thickness - integer_thickness;

                // Do we want to draw this line using a texture?
                // - For now, only draw integer-width lines using textures to avoid issues with the way scaling occurs, could be improved.
                // - If aa_size is not 1.0 we cannot use the texture path.
                // const bool use_texture = (Flags & DrawListFlags::AntiAliasedLinesUseTex) && (integer_thickness < IM_DRAWLIST_TEX_LINES_WIDTH_MAX) && (fractional_thickness <= 0.00001) && (aa_size == 1.0);
                let use_texture = self.flags.contains(&DrawListFlags::AntiAliasedLinesUseTex) && (integer_thickness < DRAW_LIST_TEX_LINES_WIDTH_MAX) && (fractional_thickness <= 0.00001) && (aa_size == 1.0);

                // We should never hit this, because NewFrame() doesn't set ImDrawListFlags_AntiAliasedLinesUseTex unless ImFontAtlasFlags_NoBakedLines is off
                // IM_ASSERT_PARANOID(!use_texture || !(_Data.Font.container_atlas.flags & FontAtlasFlags::NoBakedLines));

                // let idx_count = use_texture ? (count * 6) : (thick_line ? count * 18 : count * 12);
            let idx_count = if use_texture { count * 6 } else { if thick_line { count * 18} else { count * 12}};
            // let vtx_count = use_texture ? (points_count * 2) : (thick_line ? points_count * 4 : points_count * 3);
            let vtx_count = if use_texzture { points_count * 2 } else { if thick_line { points_count * 4} else { points_count * 3}};

                self.prim_reserve(idx_count, vtx_count);

                // Temporary buffer
                // The first <points_count> items are normals at each line point, then after that there are either 2 or 4 temp points for each line point
                // Vector2D* temp_normals = alloca(points_count * ((use_texture || !thick_line) ? 3 : 5) * sizeof(Vector2D)); //-V630
                let mut temp_normals: Vec<Vector2D> = Vec::new();
                temp_normals.reserve(points_count * (if use_texture || !thick_line { 3} else {5}));
            // Vector2D* temp_points = temp_normals + points_count;
            let temp_points_offset = points_count;

                // Calculate normals (tangents) for each line segment
                // for (int i1 = 0; i1 < count; i1 += 1)
            for i1 in 0 .. count
            {
                    let i2 = if (i1 + 1) == points_count { 0 } else { i1 + 1 };
                    let dx = points[i2].x - points[i1].x;
                    let dy = points[i2].y - points[i1].y;
                    normalize_2f_over_zero(dx, dy);
                    temp_normals[i1].x = dy;
                    temp_normals[i1].y = -dx;
                }
                if !closed {
                    temp_normals[points_count - 1] = temp_normals[points_count - 2];
                }

                // If we are drawing a one-pixel-wide line without a texture, or a textured line of any width, we only need 2 or 3 vertices per point
                if use_texture || !thick_line
                {
                    // [PATH 1] Texture-based lines (thick or non-thick)
                    // [PATH 2] Non texture-based lines (non-thick)

                    // The width of the geometry we need to draw - this is essentially <thickness> pixels for the line itself, plus "one pixel" for AA.
                    // - In the texture-based path, we don't use aa_size here because the +1 is tied to the generated texture
                    //   (see ImFontAtlasBuildRenderLinesTexData() function), and so alternate values won't work without changes to that code.
                    // - In the non texture-based paths, we would allow aa_size to potentially be != 1.0 with a patch (e.g. fringe_scale patch to
                    //   allow scaling geometry while preserving one-screen-pixel AA fringe).
                    let half_draw_size = if use_texture { ((thickness * 0.5) + 1) } else { aa_size };

                    // If line is not closed, the first and last points need to be generated differently as there are no normals to blend
                    if !closed
                    {
                        temp_points[0] = &points[0] + &temp_normals[0] * half_draw_size;
                        temp_points[1] = &points[0] - &temp_normals[0] * half_draw_size;
                        temp_points[(points_count-1)*2+0] = points[points_count-1] + temp_normals[points_count-1] * half_draw_size;
                        temp_points[(points_count-1)*2+1] = points[points_count-1] - temp_normals[points_count-1] * half_draw_size;
                    }

                    // Generate the indices to form a number of triangles for each line segment, and the vertices for the line edges
                    // This takes points n and n+1 and writes into n+1, with the first point in a closed line being generated from the final one (as n+1 wraps)
                    // FIXME-OPT: merge the different loops, possibly remove the temporary buffer.
                    let mut idx1 = self.vtx_current_idx; // Vertex index for start of line segment
                    // for (int i1 = 0; i1 < count; i1 += 1) // i1 is the first point of the line segment
                    for i1 in 0 .. count
                    {
                        let i2 = if (i1 + 1) == points_count { 0 } else { i1 + 1 }; // i2 is the second point of the line segment
                        let idx2 = if (i1 + 1) == points_count { self.vtx_current_idx } else {
                            (idx1 + (if use_texture {
                                2
                            } else { 3 }))
                        }; // Vertex index for end of segment

                        // Average normals
                        let mut dm_x = (temp_normals[i1].x + temp_normals[i2].x) * 0.5;
                        let mut dm_y = (temp_normals[i1].y + temp_normals[i2].y) * 0.5;
                        fix_normal_2f(dm_x, dm_y);
                        dm_x *= half_draw_size; // dm_x, dm_y are offset to the outer edge of the AA area
                        dm_y *= half_draw_size;

                        // Add temporary vertexes for the outer edges
                        // Vector2D* out_vtx = &temp_points[i2 * 2];
                        let out_vtx = &temp_points[i2*2];
                        out_vtx[0].x = points[i2].x + dm_x;
                        out_vtx[0].y = points[i2].y + dm_y;
                        out_vtx[1].x = points[i2].x - dm_x;
                        out_vtx[1].y = points[i2].y - dm_y;

                        if use_texture
                        {
                            // Add indices for two triangles
                            self.idx_write_ptr[0] = (idx2 + 0);
                            self.idx_write_ptr[1] = (idx1 + 0);
                            self.idx_write_ptr[2] = (idx1 + 1); // Right tri
                            self.idx_write_ptr[3] = (idx2 + 1);
                            self.idx_write_ptr[4] = (idx1 + 1);
                            self.idx_write_ptr[5] = (idx2 + 0); // Left tri
                            self.idx_write_ptr += 6;
                        }
                        else
                        {
                            // Add indexes for four triangles
                            self.idx_write_ptr[0] = (idx2 + 0);
                            self.idx_write_ptr[1] = (idx1 + 0);
                            self.idx_write_ptr[2] = (idx1 + 2); // Right tri 1
                            self.idx_write_ptr[3] = (idx1 + 2);
                            self.idx_write_ptr[4] = (idx2 + 2);
                            self.idx_write_ptr[5] = (idx2 + 0); // Right tri 2
                            self.idx_write_ptr[6] = (idx2 + 1);
                            self.idx_write_ptr[7] = (idx1 + 1);
                            self.idx_write_ptr[8] = (idx1 + 0); // Left tri 1
                            self.idx_write_ptr[9] = (idx1 + 0);
                            self.idx_write_ptr[10] = (idx2 + 0);
                            self.idx_write_ptr[11] = (idx2 + 1); // Left tri 2
                            self.idx_write_ptr += 12;
                        }

                        idx1 = idx2;
                    }

                    // Add vertexes for each point on the line
                    if use_texture
                    {
                        // If we're using textures we only need to emit the left/right edge vertices
                        let tex_uvs = self.data.tex_uv_lines[integer_thickness];
                        /*if (fractional_thickness != 0.0) // Currently always zero when use_texture==false!
                        {
                            const Vector4D tex_uvs_1 = _Data->tex_uv_lines[integer_thickness + 1];
                            tex_uvs.x = tex_uvs.x + (tex_uvs_1.x - tex_uvs.x) * fractional_thickness; // inlined ImLerp()
                            tex_uvs.y = tex_uvs.y + (tex_uvs_1.y - tex_uvs.y) * fractional_thickness;
                            tex_uvs.z = tex_uvs.z + (tex_uvs_1.z - tex_uvs.z) * fractional_thickness;
                            tex_uvs.w = tex_uvs.w + (tex_uvs_1.w - tex_uvs.w) * fractional_thickness;
                        }*/
                        let  tex_uv0 = Vector2D::new(tex_uvs.x, tex_uvs.y);
                        let tex_uv1 = Vector2D::new(tex_uvs.z, tex_uvs.w);
                        // for (int i = 0; i < points_count; i += 1)
                        for i in 0 .. points_count
                        {
                            self.vtx_write_ptr[0].pos = temp_points[i * 2 + 0];
                            self.vtx_write_ptr[0].uv = tex_uv0.clone();
                            self.vtx_write_ptr[0].col = color; // Left-side outer edge
                            self.vtx_write_ptr[1].pos = temp_points[i * 2 + 1];
                            self.vtx_write_ptr[1].uv = tex_uv1.clone();
                            self.vtx_write_ptr[1].col = color; // Right-side outer edge
                            self.vtx_write_ptr += 2;
                        }
                    }
                    else
                    {
                        // If we're not using a texture, we need the center vertex as well
                        // for (int i = 0; i < points_count; i += 1)
                        for i in 0 .. points_count
                        {
                            self.vtx_write_ptr[0].pos = points[i].clone();
                            self.vtx_write_ptr[0].uv = opaque_uv;
                            self.vtx_write_ptr[0].col = color;       // Center of line
                            self.vtx_write_ptr[1].pos = temp_points[i * 2 + 0];
                            self.vtx_write_ptr[1].uv = opaque_uv;
                            self.vtx_write_ptr[1].col = col_trans; // Left-side outer edge
                            self.vtx_write_ptr[2].pos = temp_points[i * 2 + 1];
                            self.vtx_write_ptr[2].uv = opaque_uv;
                            self.vtx_write_ptr[2].col = col_trans; // Right-side outer edge
                            self.vtx_write_ptr += 3;
                        }
                    }
                }
                else
                {
                    // [PATH 2] Non texture-based lines (thick): we need to draw the solid line core and thus require four vertices per point
                    let half_inner_thickness = (thickness - aa_size) * 0.5;

                    // If line is not closed, the first and last points need to be generated differently as there are no normals to blend
                    if !closed
                    {
                        let points_last = points_count - 1;
                        temp_points[0] = &points[0] + &temp_normals[0] * (half_inner_thickness + aa_size);
                        temp_points[1] = &points[0] + &temp_normals[0] * (half_inner_thickness);
                        temp_points[2] = &points[0] - &temp_normals[0] * (half_inner_thickness);
                        temp_points[3] = &points[0] - &temp_normals[0] * (half_inner_thickness + aa_size);
                        temp_points[points_last * 4 + 0] = &points[points_last] + &temp_normals[points_last] * (half_inner_thickness + aa_size);
                        temp_points[points_last * 4 + 1] = &points[points_last] + &temp_normals[points_last] * (half_inner_thickness);
                        temp_points[points_last * 4 + 2] = &points[points_last] - &temp_normals[points_last] * (half_inner_thickness);
                        temp_points[points_last * 4 + 3] = &points[points_last] - &temp_normals[points_last] * (half_inner_thickness + aa_size);
                    }

                    // Generate the indices to form a number of triangles for each line segment, and the vertices for the line edges
                    // This takes points n and n+1 and writes into n+1, with the first point in a closed line being generated from the final one (as n+1 wraps)
                    // FIXME-OPT: merge the different loops, possibly remove the temporary buffer.
                    let idx1 = self.vtx_current_idx; // Vertex index for start of line segment
                    // for (int i1 = 0; i1 < count; i1 += 1) // i1 is the first point of the line segment
                    for i1 in 0 .. count
                    {
                        let i2 = if (i1 + 1) == points_count { 0 } else { i1 + 1 }; // i2 is the second point of the line segment
                        let idx2 = if (i1 + 1) == points_count { self.vtx_current_idx }else { idx1 + 4 }; // Vertex index for end of segment

                        // Average normals
                        let dm_x =  (temp_normals[i1].x + temp_normals[i2].x) * 0.5;
                        let dm_y =  (temp_normals[i1].y + temp_normals[i2].y) * 0.5;
                        fix_normal_2f(dm_x, dm_y);
                        let dm_out_x =  dm_x * (half_inner_thickness + aa_size);
                        let dm_out_y =  dm_y * (half_inner_thickness + aa_size);
                        let dm_in_x =  dm_x * half_inner_thickness;
                        let dm_in_y =  dm_y * half_inner_thickness;

                        // Add temporary vertices
                        let out_vtx = &temp_points[i2 * 4];
                        out_vtx[0].x = points[i2].x + dm_out_x;
                        out_vtx[0].y = points[i2].y + dm_out_y;
                        out_vtx[1].x = points[i2].x + dm_in_x;
                        out_vtx[1].y = points[i2].y + dm_in_y;
                        out_vtx[2].x = points[i2].x - dm_in_x;
                        out_vtx[2].y = points[i2].y - dm_in_y;
                        out_vtx[3].x = points[i2].x - dm_out_x;
                        out_vtx[3].y = points[i2].y - dm_out_y;

                        // Add indexes
                        self.idx_write_ptr[0] = (idx2 + 1);
                        self.idx_write_ptr[1] = (idx1 + 1);
                        self.idx_write_ptr[2] = (idx1 + 2);
                        self.idx_write_ptr[3] = (idx1 + 2);
                        self.idx_write_ptr[4] = (idx2 + 2);
                        self.idx_write_ptr[5] = (idx2 + 1);
                        self.idx_write_ptr[6] = (idx2 + 1);
                        self.idx_write_ptr[7] = (idx1 + 1);
                        self.idx_write_ptr[8] = (idx1 + 0);
                        self.idx_write_ptr[9] = (idx1 + 0);
                        self.idx_write_ptr[10] = (idx2 + 0);
                        self.idx_write_ptr[11] = (idx2 + 1);
                        self.idx_write_ptr[12] = (idx2 + 2);
                        self.idx_write_ptr[13] = (idx1 + 2);
                        self.idx_write_ptr[14] = (idx1 + 3);
                        self.idx_write_ptr[15] = (idx1 + 3);
                        self.idx_write_ptr[16] = (idx2 + 3);
                        self.idx_write_ptr[17] = (idx2 + 2);
                        self.idx_write_ptr += 18;

                        idx1 = idx2;
                    }

                    // Add vertices
                    // for (int i = 0; i < points_count; i += 1)
                    for i in 0 .. points_count
                    {
                        self.vtx_write_ptr[0].pos = temp_points[i * 4 + 0];
                        self.vtx_write_ptr[0].uv = opaque_uv;
                        self.vtx_write_ptr[0].col = col_trans;
                        self.vtx_write_ptr[1].pos = temp_points[i * 4 + 1];
                        self.vtx_write_ptr[1].uv = opaque_uv;
                        self.vtx_write_ptr[1].col = color;
                        self.vtx_write_ptr[2].pos = temp_points[i * 4 + 2];
                        self.vtx_write_ptr[2].uv = opaque_uv;
                        self.vtx_write_ptr[2].col = color;
                        self.vtx_write_ptr[3].pos = temp_points[i * 4 + 3];
                        self.vtx_write_ptr[3].uv = opaque_uv;
                        self.vtx_write_ptr[3].col = col_trans;
                        self.vtx_write_ptr += 4;
                    }
                }
                self.vtx_current_idx += vtx_count;
            }
            else
            {
                // [PATH 4] Non texture-based, Non anti-aliased lines
                let idx_count = count * 6;
                let vtx_count = count * 4;    // FIXME-OPT: Not sharing edges
                self.prim_reserve(idx_count, vtx_count);

                // for (int i1 = 0; i1 < count; i1 += 1)
                for i1 in 0 .. count
                {
                    let i2 = if (i1 + 1) == points_count { 0 } else { i1 + 1 };
                    let p1 = &points[i1];
                    let p2 = &points[i2];

                    let mut dx =  p2.x - p1.x;
                    let mut dy =  p2.y - p1.y;
                    normalize_2f_over_zero(dx, dy);
                    dx *= (thickness * 0.5);
                    dy *= (thickness * 0.5);

                    self.vtx_write_ptr[0].pos.x = p1.x + dy;
                    self.vtx_write_ptr[0].pos.y = p1.y - dx;
                    self.vtx_write_ptr[0].uv = opaque_uv;
                    self.vtx_write_ptr[0].col = color;
                    self.vtx_write_ptr[1].pos.x = p2.x + dy;
                    self.vtx_write_ptr[1].pos.y = p2.y - dx;
                    self.vtx_write_ptr[1].uv = opaque_uv;
                    self.vtx_write_ptr[1].col = color;
                    self.vtx_write_ptr[2].pos.x = p2.x - dy;
                    self.vtx_write_ptr[2].pos.y = p2.y + dx;
                    self.vtx_write_ptr[2].uv = opaque_uv;
                    self.vtx_write_ptr[2].col = color;
                    self.vtx_write_ptr[3].pos.x = p1.x - dy;
                    self.vtx_write_ptr[3].pos.y = p1.y + dx;
                    self.vtx_write_ptr[3].uv = opaque_uv;
                    self.vtx_write_ptr[3].col = color;
                    self.vtx_write_ptr += 4;

                    self.idx_write_ptr[0] = (self.vtx_current_idx);
                    self.idx_write_ptr[1] = (self.vtx_current_idx + 1);
                    self.idx_write_ptr[2] = (self.vtx_current_idx + 2);
                    self.idx_write_ptr[3] = (self.vtx_current_idx);
                    self.idx_write_ptr[4] = (self.vtx_current_idx + 2);
                    self.idx_write_ptr[5] = (self.vtx_current_idx + 3);
                    self.idx_write_ptr += 6;
                    self.vtx_current_idx += 4;
                }
            }
    }
    //  void  add_convex_poly_filled(const Vector2D* points, int num_points, ImU32 col);
    pub fn add_convex_poly_filled(&mut self, points: &Vec<Vector2D>, points_count: usize, col: u32) {
          if points_count < 3 {
              return;
          }


            let uv = &self.data.text_uv_white_pixel;

            if self.flags.contains(&DrawListFlags::AntiAliasedFill)
            {
                // Anti-aliased Fill
                let aa_size = fringe_scale;
                let col_trans = col & !COLOR32_A_MASK;
                let idx_count = (points_count - 2)*3 + points_count * 6;
                let vtx_count = (points_count * 2);
                self.prim_reserve(idx_count, vtx_count);

                // Add indexes for fill
                let vtx_inner_idx = self.vtx_current_idx;
                let vtx_outer_idx = self.vtx_current_idx + 1;
                // for (int i = 2; i < points_count; i += 1)
                for i in 2 .. points_count
                {
                    self.idx_write_ptr[0] = (vtx_inner_idx);
                    self.idx_write_ptr[1] = (vtx_inner_idx + ((i - 1) << 1));
                    self.idx_write_ptr[2] = (vtx_inner_idx + (i << 1));
                    self.idx_write_ptr += 3;
                }

                // Compute normals
                // let temp_normals = alloca(points_count * sizeof(Vector2D)); //-V630
                let mut temp_normals: Vec<Vector2D> = Vec::new();
                temp_normals.reserve(points_count);

                // for (int i0 = points_count - 1, i1 = 0; i1 < points_count; i0 = i1 += 1)
                let indexes_a = 0 .. points_count - 1;
                let indexes_b = 0 .. points_count;
                for (i0,i1) in indexes_a.zip(indexes_b)
                {
                   let p0 = &points[i0];
                    let p1 = &points[i1];
                    let dx =  p1.x - p0.x;
                    let dy =  p1.y - p0.y;
                    normalize_2f_over_zero(dx, dy);
                    temp_normals[i0].x = dy;
                    temp_normals[i0].y = -dx;
                }

                // for (int i0 = points_count - 1, i1 = 0; i1 < points_count; i0 = i1 += 1)
                for (i0, i1) in indexes_a.zip(indexes_b)
                {
                    // Average normals
                    let n0 = &temp_normals[i0];
                    let n1 = &temp_normals[i1];
                    let mut dm_x =  (n0.x + n1.x) * 0.5;
                    let dm_y =  (n0.y + n1.y) * 0.5;
                    fix_normal_2f(dm_x, dm_y);
                    dm_x *= aa_size * 0.5;
                    dm_y *= aa_size * 0.5;

                    // Add vertices
                    self.vtx_write_ptr[0].pos.x = (points[i1].x - dm_x);
                    self.vtx_write_ptr[0].pos.y = (points[i1].y - dm_y);
                    self.vtx_write_ptr[0].uv = uv;
                    self.vtx_write_ptr[0].col = col;        // Inner
                    self.vtx_write_ptr[1].pos.x = (points[i1].x + dm_x);
                    self.vtx_write_ptr[1].pos.y = (points[i1].y + dm_y);
                    self.vtx_write_ptr[1].uv = uv;
                    self.vtx_write_ptr[1].col = col_trans;  // Outer
                    self.vtx_write_ptr += 2;

                    // Add indexes for fringes
                    self.idx_write_ptr[0] = (vtx_inner_idx + (i1 << 1));
                    self.idx_write_ptr[1] = (vtx_inner_idx + (i0 << 1));
                    self.idx_write_ptr[2] = (vtx_outer_idx + (i0 << 1));
                    self.idx_write_ptr[3] = (vtx_outer_idx + (i0 << 1));
                    self.idx_write_ptr[4] = (vtx_outer_idx + (i1 << 1));
                    self.idx_write_ptr[5] = (vtx_inner_idx + (i1 << 1));
                    self.idx_write_ptr += 6;
                }
                self.vtx_current_idx += vtx_count;
            }
            else
            {
                // Non Anti-aliased Fill
                let idx_count = (points_count - 2)*3;
                let vtx_count = points_count;
                self.prim_reserve(idx_count, vtx_count);
                // for (int i = 0; i < vtx_count; i += 1)
                for i in 0 .. vtx_count
                {
                    self.vtx_write_ptr[0].pos = &points[i];
                    self.vtx_write_ptr[0].uv = uv;
                    self.vtx_write_ptr[0].col = col;
                    self.vtx_write_ptr += 1;
                }
                // for (int i = 2; i < points_count; i += 1)
                for i in 2 .. points_count
                {
                    self.idx_write_ptr[0] = (self.vtx_current_idx);
                    self.idx_write_ptr[1] = (self.vtx_current_idx + i - 1);
                    self.idx_write_ptr[2] = (self.vtx_current_idx + i);
                    self.idx_write_ptr += 3;
                }
                self.vtx_current_idx += vtx_count;
            }
    }
    //  void  add_bezier_cubic(const Vector2D& p1, const Vector2D& p2, const Vector2D& p3, const Vector2D& p4, ImU32 col, float thickness, int num_segments = 0); // Cubic Bezier (4 control points)
    pub fn add_bezier_cubic(
        &mut self,
        p1: &Vector2D,
        p2: &Vector2D,
        p3: &Vector2D,
        p4: &Vector2D,
        col: u32,
        thickness: f32,
        num_segments: usize,
    ) {
        if (col & COLOR32_A_MASK) == 0 {
            return;
        }

        self.path_line_to(p1);
        self.path_bezier_cubic_curve_to(p2, p3, p4, num_segments);
        self.path_stroke(col, None, thickness);
    }
    //  void  add_bezier_quadratic(const Vector2D& p1, const Vector2D& p2, const Vector2D& p3, ImU32 col, float thickness, int num_segments = 0);               // Quadratic Bezier (3 control points)
    pub fn add_bezier_quadratic(
        &mut self,
        p1: &Vector2D,
        p2: &Vector2D,
        p3: &Vector2D,
        col: u32,
        thickness: f32,
        num_segments: usize,
    ) {
        if (col & COLOR32_A_MASK) == 0 {
            return;
        }

        self.path_line_to(p1);
        self.path_bezier_quadratic_curve_to(p2, p3, num_segments);
        self.path_stroke(col, None, thickness);
    }

    // Image primitives
    // - Read FAQ to understand what ImTextureID is.
    // - "p_min" and "p_max" represent the upper-left and lower-right corners of the rectangle.
    // - "uv_min" and "uv_max" represent the normalized texture coordinates to use for those corners. Using (0,0)->(1,1) texture coordinates will generally display the entire texture.
    //  void  AddImage(ImTextureID user_texture_id, const Vector2D& p_min, const Vector2D& p_max, const Vector2D& uv_min = Vector2D(0, 0), const Vector2D& uv_max = Vector2D(1, 1), ImU32 col = IM_COL32_WHITE);
    pub fn add_image(
        &mut self,
        user_texture_id: TextureId,
        p_min: &Vector2D,
        p_max: &Vector2D,
        uv_min: &Vector2D,
        uv_max: &Vector2D,
        col: u32,
    ) {
        //
        if (col & COLOR32_A_MASK) == 0 {
            return;
        }

        // const bool
        let push_texture_id = user_texture_id != self.command_header.texture_id;
        if push_texture_id {
            self.push_texture_id(user_texture_id);
        }

        self.prim_reserve(6, 4);
        self.prim_rect_uv(p_min, p_max, uv_min, uv_max, col);

        if push_texture_id {
            self.pop_texture_id();
        }
    }
    //  void  add_image_quad(ImTextureID user_texture_id, const Vector2D& p1, const Vector2D& p2, const Vector2D& p3, const Vector2D& p4, const Vector2D& uv1 = Vector2D(0, 0), const Vector2D& uv2 = Vector2D(1, 0), const Vector2D& uv3 = Vector2D(1, 1), const Vector2D& uv4 = Vector2D(0, 1), ImU32 col = IM_COL32_WHITE);
    pub fn add_image_quad(
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
        if (col & COLOR32_A_MASK) == 0 {
            return;
        }

        // const bool
        let push_texture_id = user_texture_id != self.command_header.texture_id;
        if push_texture_id {
            self.push_texture_id(user_texture_id);
        }

        self.prim_reserve(6, 4);
        self.prim_quad_uv(p1, p2, p3, p4, uv1, uv2, uv3, uv4, col);

        if push_texture_id {
            self.pop_texture_id();
        }
    }
    //  void  add_image_rounded(ImTextureID user_texture_id, const Vector2D& p_min, const Vector2D& p_max, const Vector2D& uv_min, const Vector2D& uv_max, ImU32 col, float rounding, ImDrawFlags flags = 0);
    pub fn add_image_rounded(
        &mut self,
        user_texture_id: TextureId,
        p_min: &Vector2D,
        p_max: &Vector2D,
        uv_min: &Vector2D,
        uv_max: &Vector2D,
        col: u32,
        rounding: f32,
        flags: &mut HashSet<DrawFlags>,
    ) {
        if (col & COLOR32_A_MASK) == 0 {
            return;
        }

        // flags = fix_rect_corner_flags(flags);
        set_hash_set(flags, &fix_rect_corner_flags(flags));
        // let flags_to_check = flags.insert(ROUND_CORNERS_MASK);
        if rounding < 0.5 || flags.contains(&DrawFlags::RoundCornersNone) {
            self.add_image(user_texture_id, p_min, p_max, uv_min, uv_max, col);
            return;
        }

        // const bool
        let push_texture_id = user_texture_id != self.command_header.texture_id;
        if push_texture_id {
            self.push_texture_id(user_texture_id);
        }

        // int
        let vert_start_idx = self.vtx_buffer.len();
        self.path_rect(p_min, p_max, rounding, flags);
        self.path_fill_convex(col);
        // int
        let vert_end_idx = vtx_buffer.size;
        self.shade_verts_linear_uv(self, vert_start_idx, vert_end_idx, p_min, p_max, uv_min, uv_max, true);

        if push_texture_id {
            self.pop_texture_id();
        }
    }

    // Stateful path API, add points then finish with PathFillConvex() or PathStroke()
    // - Filled shapes must always use clockwise winding order. The anti-aliasing fringe depends on it. Counter-clockwise shapes will have "inward" anti-aliasing.
    // inline    void  path_clear()                                                 { _path.size = 0; }
    pub fn path_clear(&mut self) {
        self.path.clear();
    }
    // inline    void  PathLineTo(const Vector2D& pos)                               { _path.push_back(pos); }
    pub fn path_line_to(&mut self, pos: &Vector2D) {
        self.path.push(pos.clone())
    }
    // inline    void  PathLineToMergeDuplicate(const Vector2D& pos)                 { if (_path.size == 0 || memcmp(&_path.data[_path.size - 1], &pos, 8) != 0) _path.push_back(pos); }
    pub fn path_line_to_merge_duplicate(&mut self, pos: &Vector2D) {
        if self.path.len() == 0 || (self.path[self.path.len() - 1] != pos) {
            self.path.push(pos.clone())
        }
    }
    // inline    void  PathFillConvex(ImU32 col)                                   { add_convex_poly_filled(_path.data, _path.size, col); _path.size = 0; }
    pub fn path_fill_convex(&mut self, col: u32) {
        self.add_convex_poly_filled(&self.path, self.path.len(), col);
        self.path.clear()
    }
    // inline    void  PathStroke(ImU32 col, ImDrawFlags flags = 0, float thickness = 1.0) { add_polyline(_path.data, _path.size, col, flags, thickness); _path.size = 0; }
    pub fn path_stroke(&mut self, col: u32, flags: &HashSet<DrawFlags>, thickness: f32) {
        self.add_polyline(&self.path, self.path.len(), col, flags, thickness);
        self.path.clear()
    }
    //  void  PathArcTo(const Vector2D& center, float radius, float a_min, float a_max, int num_segments = 0);
    pub fn path_arc_to(
        &mut self,
        center: &Vector2D,
        radius: f32,
        a_min: f32,
        a_max: f32,
        num_segments: i32,
    ) {
         if radius < 0.5
            {
                self.path.push_back(center);
                return;
            }

            if num_segments > 0
            {
                self.path_arc_to_n(center, radius, a_min, a_max, num_segments);
                return;
            }

            // Automatic segment count
            if radius <= self.data.arc_fast_radius_cutoff
            {
                let a_is_reverse = a_max < a_min;

                // We are going to use precomputed values for mid samples.
                // Determine first and last sample in lookup table that belong to the arc.
                let a_min_sample_f = DRAW_LIST_ARCFAST_SAMPLE_MAX * a_min / (f32::PI * 2.0);
                let a_max_sample_f = DRAW_LIST_ARCFAST_SAMPLE_MAX * a_max / (f32::PI * 2.0);

                let a_min_sample = if a_is_reverse { f32::floor(a_min_sample_f) } else
                { ImCeil(a_min_sample_f) };
                let a_max_sample = a_is_reverse ? ImCeil(a_max_sample_f) : f32::floor(a_max_sample_f);
                let a_mid_samples = a_is_reverse ? ImMax(a_min_sample - a_max_sample, 0) : ImMax(a_max_sample - a_min_sample, 0);

                let a_min_segment_angle = a_min_sample * f32::PI * 2.0 / DRAW_LIST_ARCFAST_SAMPLE_MAX;
                let a_max_segment_angle = a_max_sample * f32::PI * 2.0 / DRAW_LIST_ARCFAST_SAMPLE_MAX;
                let a_emit_start = f32::abs(a_min_segment_angle - a_min) >= 1e-5f32;
                let a_emit_end = f32::abs(a_max - a_max_segment_angle) >= 1e-5f32;

                self.path.reserve(path.size + (a_mid_samples + 1 + (if a_emit_start { 1 } else { 0 }) + (if a_emit_end { 1 } else { 0 })));
                if a_emit_start {
                    path.push_back(Vector2D::new(center.x + ImCos(a_min) * radius, center.y + ImSin(a_min) * radius));
                }
                if a_mid_samples > 0 {
                    path_arc_to_fast_ex(center, radius, a_min_sample, a_max_sample, 0);
                }
                if a_emit_end {
                    path.push_back(Vector2D::new(center.x + ImCos(a_max) * radius, center.y + ImSin(a_max) * radius));
                }
            }
            else
            {
                let arc_length = f32::abs(a_max - a_min);
                let circle_segment_count = calc_circle_auto_segment_count(radius);
                let arc_segment_count = ImMax(ImCeil(circle_segment_count * arc_length / (f32::PI * 2.0)), (2.0 * f32::PI / arc_length));
                path_arc_to_n(center, radius, a_min, a_max, arc_segment_count);
            }
    }
    //  void  PathArcToFast(const Vector2D& center, float radius, int a_min_of_12, int a_max_of_12);                // Use precomputed angles for a 12 steps circle
    pub fn path_arc_to_fast(
        &mut self,
        center: &Vector2D,
        radius: f32,
        a_min_of_12: i32,
        a_max_of_12: i32,
    ) {
         if radius < 0.5
            {
                path.push_back(center);
                return;
            }
            self.path_arc_to_fast_ex(center, radius, a_min_of_12 * DRAW_LIST_ARCFAST_SAMPLE_MAX / 12, a_max_of_12 * DRAW_LIST_ARCFAST_SAMPLE_MAX / 12, 0);
    }
    //  void  path_bezier_cubic_curve_to(const Vector2D& p2, const Vector2D& p3, const Vector2D& p4, int num_segments = 0); // Cubic Bezier (4 control points)
    pub fn path_bezier_cubic_curve_to(
        &mut self,
        p2: &Vector2D,
        p3: &Vector2D,
        p4: &Vector2D,
        num_segments: usize,
    ) {
        let p1 = path.back();
            if num_segments == 0
            {
                self.path_bezier_cubic_curve_toCasteljau(&_Path, p1.x, p1.y, p2.x, p2.y, p3.x, p3.y, p4.x, p4.y, self.data.curve_tessellation_tol, 0); // Auto-tessellated
            }
            else
            {
                let t_step =  1.0 / num_segments;
                // for (int i_step = 1; i_step <= num_segments; i_step += 1)
                for i_step in 1 .. num_segments
                {
                    self.path.push_back(bezier_cubic_calc(p1, p2, p3, p4, t_step * i_step));
                }
            }
    }
    //  void  path_bezier_quadratic_curve_to(const Vector2D& p2, const Vector2D& p3, int num_segments = 0);               // Quadratic Bezier (3 control points)
    pub fn path_bezier_quadratic_curve_to(
        &mut self,
        p2: &Vector2D,
        p3: &Vector2D,
        num_segments: usize,
    ) {
         Vector2D p1 = path.back();
            if num_segments == 0
            {
                self.path_bezier_quadratic_curve_to_casteljau(&_Path, p1.x, p1.y, p2.x, p2.y, p3.x, p3.y, self.data.curve_tessellation_tol, 0);// Auto-tessellated
            }
            else
            {
                let t_step =  1.0 / num_segments;
                // for (int i_step = 1; i_step <= num_segments; i_step += 1)
                for i_step in 1.. num_segments
                {
                    self.path.push_back(bezier_quadratic_calc(p1, p2, p3, t_step * i_step));
                }
            }
    }
    //  void  PathRect(const Vector2D& rect_min, const Vector2D& rect_max, float rounding = 0.0, ImDrawFlags flags = 0);
    pub fn path_rect(
        &mut self,
        rect_min: &Vector2D,
        rect_max: &Vector2D,
        mut rounding: f32,
        flags: &mut HashSet<DrawFlags>,
    ) {

        // flags.extend(fix_rect_corner_flags(flags).iter()); //= fix_rect_corner_flags(flags);
        set_hash_set(flags, &fix_rect_corner_flags(flags));
            rounding = f32::min(rounding, f32::abs(b.x - a.x) * (if ((flags & DrawFlags::RoundCornersTop)  == DrawFlags::RoundCornersTop)  || ((flags & DrawFlags::RoundCornersBottom) == DrawFlags::RoundCornersBottom) { 0.5 } else { 1.0 } ) - 1.0);
            rounding = f32::min(rounding, f32::abs(b.y - a.y) * ( if ((flags & DrawFlags::RoundCornersLeft) == DrawFlags::RoundCornersLeft) || ((flags & DrawFlags::RoundCornersRight)  == DrawFlags::RoundCornersRight) { 0.5 } else { 1.0 } ) - 1.0);

            // if rounding < 0.5 || (flags & DrawFlags::RoundCornersMask_) == DrawFlags::RoundCornersNone
        if rounding < 0.5 || flags.contains(&DrawFlags::RoundCornersNone)
        {
                path_line_to(a);
                path_line_to(Vector2D::new(b.x, a.y));
                path_line_to(b);
                path_line_to(Vector2D::new(a.x, b.y));
            }
            else
            {
                let rounding_tl = if flags.contains(&DrawFlags::RoundCornersTopLeft) { rounding } else { 0.0 };
                let rounding_tr = if flags.contains(&DrawFlags::RoundCornersTopRight) { rounding } else { 0.0 };
                let rounding_br = if flags.contains(&DrawFlags::RoundCornersBottomRight) { rounding } else { 0.0 };
                let rounding_bl = if flags.contains(&DrawFlags::RoundCornersBottomLeft) { rounding } else { 0.0 };
                self.path_arc_to_fast(&Vector2D::new(a.x + rounding_tl, a.y + rounding_tl), rounding_tl, 6, 9);
                self.path_arc_to_fast(&Vector2D::new(b.x - rounding_tr, a.y + rounding_tr), rounding_tr, 9, 12);
                self.path_arc_to_fast(&Vector2D::new(b.x - rounding_br, b.y - rounding_br), rounding_br, 0, 3);
                self.path_arc_to_fast(&Vector2D::new(a.x + rounding_bl, b.y - rounding_bl), rounding_bl, 3, 6);
            }
    }

    // Advanced
    //  void  add_callback(ImDrawCallback callback, void* callback_data);  // Your rendering function must check for 'user_callback' in ImDrawCmd and call the function instead of rendering triangles.
    pub fn add_callback(&mut self, callback: DrawCallback, callback_data: &mut Vec<u8>) {
        // IM_ASSERT_PARANOID(CmdBuffer.size > 0);
        //     ImDrawCmd* curr_cmd = &CmdBuffer.data[CmdBuffer.size - 1];
            // IM_ASSERT(curr_cmd.UserCallback == None);
        let mut curr_cmd= &mut self.cmd_buffer[self.cmd_buffer.len() -1];
        if curr_cmd.element_count != 0
            {
                self.add_draw_cmd();
               curr_cmd = &mut self.cmd_buffer[self.cmd_buffer.len() - 1];
            }
            curr_cmd.user_callback = Some(callback);
            curr_cmd.user_callback_data = callback_data.to_owned();

            self.add_draw_cmd(); // Force a new command after us (see comment below)
    }
    //  void  AddDrawCmd();                                               // This is useful if you need to forcefully create a new draw call (to allow for dependent rendering / blending). Otherwise primitives are merged into the same draw-call as much as possible
    pub fn add_draw_cmd(&mut self) {
        // ImDrawCmd draw_cmd;
        let mut draw_cmd = DrawCmd::default();
            draw_cmd.clip_rect = command_header.clip_rect;    // Same as calling ImDrawCmd_HeaderCopy()
            draw_cmd.texture_id = command_header.texture_id;
            draw_cmd.vtx_offset = command_header.vtx_offset;
            draw_cmd.idx_offset = self.idx_buffer.size;

            // IM_ASSERT(draw_cmd.clip_rect.x <= draw_cmd.clip_rect.z && draw_cmd.clip_rect.y <= draw_cmd.clip_rect.w);
            self.cmd_buffer.push_back(draw_cmd);
    }
    //  ImDrawList* clone_output() const;                                  // Create a clone of the cmd_buffer/idx_buffer/vtx_buffer.
    pub fn clone_output(&mut self) -> DrawList {
        // ImDrawList* dst = IM_NEW(ImDrawList(data));
        let mut dst = DrawList::new(&mut self.data);
            dst.cmd_buffer = self.cmd_buffer.clone();
            dst.idx_buffer = self.idx_buffer.clone();
            dst.vtx_buffer = self.vtx_buffer.clone();
            dst.flags = self.flags.clone();
            return dst;
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
    pub fn prim_reserve(&mut self, idx_count: usize, vtx_count: usize) {
         // Large mesh support (when enabled)
            // IM_ASSERT_PARANOID(idx_count >= 0 && vtx_count >= 0);
            if sizeof == 2 && (self.vtx_current_idx + vtx_count >= (1 << 16)) && (self.flags.contains(&DrawListFlags::AllowVtxOffset))
            {
                // FIXME: In theory we should be testing that vtx_count <64k here.
                // In practice, render_text() relies on reserving ahead for a worst case scenario so it is currently useful for us
                // to not make that check until we rework the text functions to handle clipping and large horizontal lines better.
                self.command_header.vtx_offset = self.vtx_buffer.size;
                self.on_changed_vtx_offset();
            }

            // ImDrawCmd* draw_cmd = &CmdBuffer.data[CmdBuffer.size - 1];
            let draw_cmd = &mut self.cmd_buffer[self.cmd_buffer.len()-1];
        draw_cmd.elem_count += idx_count;

            let vtx_buffer_old_size = self.vtx_buffer.len();
            self.vtx_buffer.resize(vtx_buffer_old_size + vtx_count);
            self.vtx_write_ptr = self.vtx_buffer.data + vtx_buffer_old_size;

            let idx_buffer_old_size = self.idx_buffer.len();
            self.idx_buffer.resize(idx_buffer_old_size + idx_count, INVALID_DRAW_INDEX);
            self.idx_write_ptr = idx_buffer.data + idx_buffer_old_size;
    }
    //  void  PrimUnreserve(int idx_count, int vtx_count);
    pub fn prim_unreserve(&mut self, idx_count: usize, vtx_count: usize) {
        // ImDrawCmd* draw_cmd = &CmdBuffer.data[CmdBuffer.size - 1];
        let draw_cmd = &mut self.cmd_buffer.data[self.cmd_buffer.len() - 1];
            draw_cmd.elem_count -= idx_count;
            self.vtx_buffer.shrink(self.vtx_buffer.size - vtx_count);
            self.idx_buffer.shrink(self.idx_buffer.size - idx_count);
    }
    //  void  PrimRect(const Vector2D& a, const Vector2D& b, ImU32 col);      // Axis aligned rectangle (composed of two triangles)
    pub fn prim_rect(&mut self, a: &Vector2D, b: &Vector2D, col: u32) {
            // Vector2D b(c.x, a.y), d(a.x, c.y), uv(data.TexUvWhitePixel);
        let b = Vector2D::new(c.x,a.y);
        let d = Vector2D::new(a.x,c.y);
        let uv = self.data.text_uv_white_pixel.clone();

        // ImDrawIdx idx = self.vtx_current_idx;
        let idx = &self.vtx_current_idx;
        self.idx_write_ptr[0] = idx;
        self.idx_write_ptr[1] = (idx + 1);
        self.idx_write_ptr[2] = (idx + 2);
        self.idx_write_ptr[3] = idx;
        self.idx_write_ptr[4] = (idx + 2);
        self.idx_write_ptr[5] = (idx + 3);
        self.vtx_write_ptr[0].pos = a;
        self.vtx_write_ptr[0].uv = uv;
        self.vtx_write_ptr[0].col = col;
        self.vtx_write_ptr[1].pos = b;
        self.vtx_write_ptr[1].uv = uv.clone();
        self.vtx_write_ptr[1].col = col;
        self.vtx_write_ptr[2].pos = c;
        self.vtx_write_ptr[2].uv = uv.clone();
        self.vtx_write_ptr[2].col = col;
        self.vtx_write_ptr[3].pos = d;
        self.vtx_write_ptr[3].uv = uv.clone();
        self.vtx_write_ptr[3].col = col;
            self.vtx_write_ptr += 4;
            self.vtx_current_idx += 4;
            self.idx_write_ptr += 6;
    }
    //  void  prim_rect_uv(const Vector2D& a, const Vector2D& b, const Vector2D& uv_a, const Vector2D& uv_b, ImU32 col);
    pub fn prim_rect_uv(
        &mut self,
        a: &Vector2D,
        b: &Vector2D,
        uv_a: &Vector2D,
        uv_b: &Vector2D,
        col: u32,
    ) {
         // Vector2D b(c.x, a.y), d(a.x, c.y), uv_b(uv_c.x, uv_a.y), uv_d(uv_a.x, uv_c.y);
         let b = Vector2D::new(c.x, a.y);
        let d = Vector2D::new(a.x,c.y);
        let uv_b = Vector2D::new(uv_c.x, uv_a.y);
        let uv_d = Vector2D::new(uv_a.x, uv_c.y);
            let idx = self.vtx_current_idx;
        self.idx_write_ptr[0] = idx;
        self.idx_write_ptr[1] = (idx + 1);
        self.idx_write_ptr[2] = (idx + 2);
        self.idx_write_ptr[3] = idx;
        self.idx_write_ptr[4] = (idx + 2);
        self.idx_write_ptr[5] = (idx + 3);
        self.vtx_write_ptr[0].pos = a;
        self.vtx_write_ptr[0].uv = uv_a;
        self.vtx_write_ptr[0].col = col;
        self.vtx_write_ptr[1].pos = b;
        self.vtx_write_ptr[1].uv = uv_b;
        self.vtx_write_ptr[1].col = col;
        self.vtx_write_ptr[2].pos = c;
        self.vtx_write_ptr[2].uv = uv_c;
        self.vtx_write_ptr[2].col = col;
        self.vtx_write_ptr[3].pos = d;
        self.vtx_write_ptr[3].uv = uv_d;
        self.vtx_write_ptr[3].col = col;
            self.vtx_write_ptr += 4;
            self.vtx_current_idx += 4;
            self.idx_write_ptr += 6;
    }
    //  void  prim_quad_uv(const Vector2D& a, const Vector2D& b, const Vector2D& c, const Vector2D& d, const Vector2D& uv_a, const Vector2D& uv_b, const Vector2D& uv_c, const Vector2D& uv_d, ImU32 col);
    pub fn prim_quad_uv(
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
            let idx = self.vtx_current_idx;
        self.idx_write_ptr[0] = idx;
        self.idx_write_ptr[1] = (idx + 1);
        self.idx_write_ptr[2] = (idx + 2);
        self.idx_write_ptr[3] = idx;
        self.idx_write_ptr[4] = (idx + 2);
        self.idx_write_ptr[5] = (idx + 3);
        self.vtx_write_ptr[0].pos = a;
        self.vtx_write_ptr[0].uv = uv_a;
        self.vtx_write_ptr[0].col = col;
        self.vtx_write_ptr[1].pos = b;
        self.vtx_write_ptr[1].uv = uv_b;
        self.vtx_write_ptr[1].col = col;
        self.vtx_write_ptr[2].pos = c;
        self.vtx_write_ptr[2].uv = uv_c;
        self.vtx_write_ptr[2].col = col;
        self.vtx_write_ptr[3].pos = d;
        self.vtx_write_ptr[3].uv = uv_d;
        self.vtx_write_ptr[3].col = col;
            self.vtx_write_ptr += 4;
            self.vtx_current_idx += 4;
            self.idx_write_ptr += 6;
    }
    // inline    void  PrimWriteVtx(const Vector2D& pos, const Vector2D& uv, ImU32 col)    { _VtxWritePtr->pos = pos; _VtxWritePtr->uv = uv; _VtxWritePtr->col = col; _VtxWritePtr++; _VtxCurrentIdx++; }
    pub fn prime_write_vtx(&mut self, pos: &Vector2D, uv: &Vector2D, col: u32) {
        // TODO: replace VtxWritePtr with a vector of vertices
        self.vtx_buffer.push(DrawVertex {
            pos: pos.clone(),
            uv: uv.clone(),
            col,
        });
    }
    // inline    void  PrimWriteIdx(ImDrawIdx idx)                                     { *_IdxWritePtr = idx; _IdxWritePtr++; }
    pub fn prim_write_idx(&mut self, idx: DrawIndex) {
        self.idx_buffer.push(idx)
    }
    // inline    void  PrimVtx(const Vector2D& pos, const Vector2D& uv, ImU32 col)         { PrimWriteIdx((ImDrawIdx)_VtxCurrentIdx); PrimWriteVtx(pos, uv, col); } // Write vertex with unique index
    pub fn prim_vtx(&mut self, pos: &Vector2D, uv: &Vector2D, col: u32) {
        self.prime_write_vtx(pos, uv, col);
        self.prim_write_idx(0)
    }

    // #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
    //     inline    void  AddBezierCurve(const Vector2D& p1, const Vector2D& p2, const Vector2D& p3, const Vector2D& p4, ImU32 col, float thickness, int num_segments = 0) { add_bezier_cubic(p1, p2, p3, p4, col, thickness, num_segments); } // OBSOLETED in 1.80 (Jan 2021)
    //     inline    void  PathBezierCurveTo(const Vector2D& p2, const Vector2D& p3, const Vector2D& p4, int num_segments = 0) { path_bezier_cubic_curve_to(p2, p3, p4, num_segments); } // OBSOLETED in 1.80 (Jan 2021)
    // #endif

    /// Initialize before use in a new frame. We always have a command ready in the buffer.
    pub fn reset_for_new_frame(&mut self) {
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
        set_hash_set(&mut self.flags, &self.data.initial_flags);
        //     memset(&_cmd_header, 0, sizeof(_cmd_header));
        self.command_header.clear();
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
    pub fn clear_free_memory(&mut self) {
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
        self.texture_id_stack.clear();
        self.path.clear();
        self.splitter.clear_free_memory();
        //     _VtxWritePtr = None;
        //     _IdxWritePtr = None;
        //     _ClipRectStack.clear();
        //     _TextureIdStack.clear();
        //     _Path.clear();
        //     _Splitter.ClearFreeMemory();
    }
     // void  _PopUnusedDrawCmd();
pub fn pop_unused_draw_cmd(&mut self) {
        if self.cmd_buffer.is_empty() {
            return;
        }
            // ImDrawCmd* curr_cmd = &cmd_buffer.data[cmd_buffer.size - 1];
         let curr_md = &mut self.cmd_buffer[self.cmd_buffer.len() -1];
         if curr_cmd.elem_count == 0 && curr_cmd.user_callback == None {
             self.cmd_buffer.pop_back();
         }
    }
    //  void  _TryMergeDrawCmds();
pub fn try_merge_draw_commands(&mut self) {
        // IM_ASSERT_PARANOID(CmdBuffer.size > 0);
        //     ImDrawCmd* curr_cmd = &CmdBuffer.data[CmdBuffer.size - 1];
        let curr_cmd = &mut self.cmd_buffer[self.cmd_buffer.len() -1];
        // ImDrawCmd* prev_cmd = curr_cmd - 1;
        let prev_cmd = curr_cmd - 1;
            if draw_cmd_header_compare(curr_cmd, prev_cmd) == 0 && are_sequential_idx_offset(prev_cmd, curr_cmd) && curr_cmd.user_callback == None && prev_cmd.user_callback == None
            {
                prev_cmd.elem_count += curr_cmd.elem_count;
                self.cmd_buffer.pop_back();
            }
    }
    //  void  _OnChangedClipRect();
pub fn OnChangedClipRect(&mut self) {

         // If current command is used with different settings we need to add a new command
            // IM_ASSERT_PARANOID(CmdBuffer.size > 0);
            ImDrawCmd* curr_cmd = &cmd_buffer.data[cmd_buffer.size - 1];
            if (curr_cmd.elem_count != 0 && memcmp(&curr_cmd.clip_rect, &command_header.clip_rect, sizeof(Vector4D)) != 0)
            {
                AddDrawCmd();
                return;
            }
            // IM_ASSERT(curr_cmd.UserCallback == None);

            // Try to merge with previous command if it matches, else use current command
            ImDrawCmd* prev_cmd = curr_cmd - 1;
            if (curr_cmd.elem_count == 0 && cmd_buffer.size > 1 && ImDrawCmd_HeaderCompare(&command_header, prev_cmd) == 0 && ImDrawCmd_AreSequentialIdxOffset(prev_cmd, curr_cmd) && prev_cmd.user_callback == None)
            {
                cmd_buffer.pop_back();
                return;
            }

            curr_cmd.clip_rect = command_header.clip_rect;
    }
    //  void  _OnChangedTextureID();
pub fn OnChangedTextureID(&mut self) {
        // If current command is used with different settings we need to add a new command
            // IM_ASSERT_PARANOID(CmdBuffer.size > 0);
            ImDrawCmd* curr_cmd = &cmd_buffer.data[cmd_buffer.size - 1];
            if (curr_cmd.elem_count != 0 && curr_cmd.texture_id != command_header.texture_id)
            {
                AddDrawCmd();
                return;
            }
            // IM_ASSERT(curr_cmd.UserCallback == None);

            // Try to merge with previous command if it matches, else use current command
            ImDrawCmd* prev_cmd = curr_cmd - 1;
            if (curr_cmd.elem_count == 0 && cmd_buffer.size > 1 && ImDrawCmd_HeaderCompare(&command_header, prev_cmd) == 0 && ImDrawCmd_AreSequentialIdxOffset(prev_cmd, curr_cmd) && prev_cmd.user_callback == None)
            {
                cmd_buffer.pop_back();
                return;
            }

            curr_cmd.texture_id = command_header.texture_id;
    }
    //  void  _OnChangedVtxOffset();
pub fn OnChangedvtx_offset(&mut self) {
        // // We don't need to compare curr_cmd->vtx_offset != _cmd_header.vtx_offset because we know it'll be different at the time we call this.
            self.vtx_current_idx = 0;
            // IM_ASSERT_PARANOID(CmdBuffer.size > 0);
            ImDrawCmd* curr_cmd = &cmd_buffer.data[cmd_buffer.size - 1];
            //IM_ASSERT(curr_cmd->vtx_offset != _cmd_header.vtx_offset); // See #3349
            if (curr_cmd.elem_count != 0)
            {
                AddDrawCmd();
                return;
            }
            // IM_ASSERT(curr_cmd.UserCallback == None);
            curr_cmd.vtx_offset = command_header.vtx_offset;
    }
    //  int   _CalcCircleAutoSegmentCount(float radius) const;
pub fn CalCircleAUtoSegmentCount(&mut self, radius: f32) -> i32 {
        // // Automatic segment count
            let radius_idx = (radius + 0.999999); // ceil to never reduce accuracy
            if (radius_idx < IM_ARRAYSIZE(data.CircleSegmentCounts))
                return data.CircleSegmentCounts[radius_idx]; // Use cached value
            else
                return IM_DRAWLIST_CIRCLE_AUTO_SEGMENT_CALC(radius, data.CircleSegmentMaxError);
    }
    //  void  _PathArcToFastEx(const Vector2D& center, float radius, int a_min_sample, int a_max_sample, int a_step);
pub fn path_arc_toFastEx(
        &mut self,
        center: &Vector2D,
        radius: f32,
        a_min_simple: i32,
        a_max_sample: i32,
        a_step: i32,
    ) {
         if (radius < 0.5)
            {
                path.push_back(center);
                return;
            }

            // Calculate arc auto segment step size
            if (a_step <= 0)
                a_step = DRAW_LIST_ARCFAST_SAMPLE_MAX / calc_circle_auto_segment_count(radius);

            // Make sure we never do steps larger than one quarter of the circle
            a_step = ImClamp(a_step, 1, IM_DRAWLIST_ARCFAST_TABLE_SIZE / 4);

            let sample_range = ImAbs(a_max_sample - a_min_sample);
            let a_next_step = a_step;

            int samples = sample_range + 1;
            bool extra_max_sample = false;
            if (a_step > 1)
            {
                samples            = sample_range / a_step + 1;
                let overstep = sample_range % a_step;

                if (overstep > 0)
                {
                    extra_max_sample = true;
                    samples += 1;

                    // When we have overstep to avoid awkwardly looking one long line and one tiny one at the end,
                    // distribute first step range evenly between them by reducing first step size.
                    if (sample_range > 0)
                        a_step -= (a_step - overstep) / 2;
                }
            }

            path.resize(path.size + samples);
            Vector2D* out_ptr = path.data + (path.size - samples);

            int sample_index = a_min_sample;
            if (sample_index < 0 || sample_index >= DRAW_LIST_ARCFAST_SAMPLE_MAX)
            {
                sample_index = sample_index % DRAW_LIST_ARCFAST_SAMPLE_MAX;
                if (sample_index < 0)
                    sample_index += DRAW_LIST_ARCFAST_SAMPLE_MAX;
            }

            if (a_max_sample >= a_min_sample)
            {
                for (int a = a_min_sample; a <= a_max_sample; a += a_step, sample_index += a_step, a_step = a_next_step)
                {
                    // a_step is clamped to IM_DRAWLIST_ARCFAST_SAMPLE_MAX, so we have guaranteed that it will not wrap over range twice or more
                    if (sample_index >= DRAW_LIST_ARCFAST_SAMPLE_MAX)
                        sample_index -= DRAW_LIST_ARCFAST_SAMPLE_MAX;

                    const Vector2D s = data.ArcFastVtx[sample_index];
                    out_ptr.x = center.x + s.x * radius;
                    out_ptr.y = center.y + s.y * radius;
                    out_ptr += 1;
                }
            }
            else
            {
                for (int a = a_min_sample; a >= a_max_sample; a -= a_step, sample_index -= a_step, a_step = a_next_step)
                {
                    // a_step is clamped to IM_DRAWLIST_ARCFAST_SAMPLE_MAX, so we have guaranteed that it will not wrap over range twice or more
                    if (sample_index < 0)
                        sample_index += DRAW_LIST_ARCFAST_SAMPLE_MAX;

                    const Vector2D s = data.ArcFastVtx[sample_index];
                    out_ptr.x = center.x + s.x * radius;
                    out_ptr.y = center.y + s.y * radius;
                    out_ptr += 1;
                }
            }

            if (extra_max_sample)
            {
                int normalized_max_sample = a_max_sample % DRAW_LIST_ARCFAST_SAMPLE_MAX;
                if (normalized_max_sample < 0)
                    normalized_max_sample += DRAW_LIST_ARCFAST_SAMPLE_MAX;

                const Vector2D s = data.ArcFastVtx[normalized_max_sample];
                out_ptr.x = center.x + s.x * radius;
                out_ptr.y = center.y + s.y * radius;
                out_ptr += 1;
            }

            // IM_ASSERT_PARANOID(_Path.data + _Path.size == out_ptr);
    }
    //  void  _PathArcToN(const Vector2D& center, float radius, float a_min, float a_max, int num_segments);
pub fn path_arc_toN(
        &mut self,
        center: &Vector2D,
        radius: f32,
        a_min: f32,
        a_max: f32,
        num_segments: i32,
    ) {
        if (radius < 0.5)
            {
                path.push_back(center);
                return;
            }

            // Note that we are adding a point at both a_min and a_max.
            // If you are trying to draw a full closed circle you don't want the overlapping points!
            path.reserve(path.size + (num_segments + 1));
            for (int i = 0; i <= num_segments; i += 1)
            {
                let a = a_min + (i / num_segments) * (a_max - a_min);
                path.push_back(Vector2D::new(center.x + ImCos(a) * radius, center.y + ImSin(a) * radius));
            }
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
    anti_aliased_lines,
    // Enable anti-aliased lines/borders (*2 the number of triangles for 1.0 wide line or lines thin enough to be drawn using textures, otherwise *3 the number of triangles)
    anti_aliased_lines_use_tex,
    // Enable anti-aliased lines/borders using textures when possible. Require backend to render with bilinear filtering (NOT point/nearest filtering).
    anti_aliased_fill,
    // Enable anti-aliased edge around filled shapes (rounded rectangles, circles).
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
        draw_list.push_clip_rect(viewport.pos, viewport.pos + viewport.size, false);
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

    if draw_list.cmd_buffer.size == 1 && draw_list.cmd_buffer[0].elem_count == 0 && draw_list.cmd_buffer[0].user_callback.is_none() {
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
