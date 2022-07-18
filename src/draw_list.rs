use std::collections::HashSet;
use std::ffi::c_void;
use std::os::raw::c_char;
use std::f32::consts::PI;
use std::mem::size_of;
use crate::context::Context;
use crate::draw_defines::DrawFlags;
use crate::types::{DrawIndex, Id32, INVALID_ID};
use crate::draw_cmd::{CmdHeader, DrawCmd};
use crate::draw_list_shared_data::DrawListSharedData;
use crate::draw_list_splitter::DrawListSplitter;
use crate::draw_vert::DrawVertex;
use crate::font::Font;
use crate::rect::Rect;
use crate::texture::TextureId;
use crate::utils::set_hash_set;
use crate::vectors::Vector4D;
use crate::vectors::two_d::Vector2D;
use crate::viewport::Viewport;

/// Draw command list
/// This is the low-level list of polygons that ImGui:: functions are filling. At the end of the frame,
/// all command lists are passed to your ImGuiIO::RenderDrawListFn function for rendering.
/// Each dear imgui window contains its own ImDrawList. You can use ImGui::GetWindowDrawList() to
/// access the current window draw list and draw custom primitives.
/// You can interleave normal ImGui:: calls and adding primitives to the current draw list.
/// In single viewport mode, top-left is == GetMainViewport()->pos (generally 0,0), bottom-right is == GetMainViewport()->pos+size (generally io.display_size).
/// You are totally free to apply whatever transformation matrix to want to the data (depending on the use of the transformation you may want to apply it to clip_rect as well!)
/// Important: Primitives are always added to the list and not culled (culling is done at higher-level by ImGui:: functions), if you use this API a lot consider coarse culling your drawn objects.
#[derive(Default,Debug,Clone)]
pub struct DrawList
{
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
    pub _splitter: DrawListSplitter,
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
    pub fn push_clip_rect(&mut self, clip_rect_min: &Vector2D, clip_rect_max: &Vector2D, intersect_with_current_clip_rect: bool) {
        todo!()
    }
    //  void  push_clip_rect_full_screen();
    pub fn push_clip_rect_full_screen(&mut self) {
        todo!()
    }
    //  void  pop_clip_rect();
    pub fn pop_clip_rect(&mut self) {
        todo!()
    }
    //  void  push_texture_id(ImTextureID texture_id);
    pub fn push_texture_id(&mut self, texture_id: TextureId) {todo!()}
    //  void  pop_texture_id();
    pub fn pop_texture_id(&mut self) {todo!()}
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
        todo!()
    }
    //  void  add_rect(const Vector2D& p_min, const Vector2D& p_max, ImU32 col, float rounding = 0.0, ImDrawFlags flags = 0, float thickness = 1.0);   // a: upper-left, b: lower-right (== upper-left + size)
    pub fn add_rect(&mut self, p_min: &Vector2D, p_max: Vector2D, col: u32, rounding: f32, flags: DrawFlags, thickness: f32) {
        todo!()
    }
    //  void  add_rect_filled(const Vector2D& p_min, const Vector2D& p_max, ImU32 col, float rounding = 0.0, ImDrawFlags flags = 0);                     // a: upper-left, b: lower-right (== upper-left + size)
    pub fn add_rect_filled(&mut self, p_min: &Vector2D, p_max: &Vector2D, col: u32, rounding: f32, flags: &HashSet<DrawFlags>) {
        todo!()
    }
    //  void  add_rect_filled_multi_color(const Vector2D& p_min, const Vector2D& p_max, ImU32 col_upr_left, ImU32 col_upr_right, ImU32 col_bot_right, ImU32 col_bot_left);
    pub fn add_rect_filled_multi_color(&mut self, p_min: &Vector2D, p_max: &Vector2D, col_upr_left: u32, col_upr_right: u32, col_bot_right: u32, col_bot_left: u32) {
        todo!()
    }
    //  void  add_quad(const Vector2D& p1, const Vector2D& p2, const Vector2D& p3, const Vector2D& p4, ImU32 col, float thickness = 1.0);
    pub fn add_quad(&mut self, p1: &Vector2D, p2: &Vector2D, p3: &Vector2D, p4: &Vector2D, col: u32, thickness: f32) {
        todo!()
    }
    //  void  add_quad_filled(const Vector2D& p1, const Vector2D& p2, const Vector2D& p3, const Vector2D& p4, ImU32 col);
    pub fn add_quad_filled(&mut self, p1: &Vector2D, p2: &Vector2D, p3: &Vector2D, p4: &Vector2D, col: u32) {
        todo!()
    }
    //  void  add_triangle(const Vector2D& p1, const Vector2D& p2, const Vector2D& p3, ImU32 col, float thickness = 1.0);
    pub fn add_triangle(&mut self, p1: &Vector2D, p2: &Vector2D, p3: &Vector2D, col: u32, thickness: f32) {
        todo!()
    }
    //  void  add_triangle_filled(const Vector2D& p1, const Vector2D& p2, const Vector2D& p3, ImU32 col);
    pub fn add_triangle_filled(&mut self, p1: &Vector2D, p2: &Vector2D, p3: &Vector2D, col: u32) {
        todo!()
    }
    //  void  add_circle(const Vector2D& center, float radius, ImU32 col, int num_segments = 0, float thickness = 1.0);
    pub fn add_circle(&mut self, center: &Vector2D, radius: f32, col: u32, num_segments: i32, thickness: f32) {
        todo!()
    }
    //  void  add_circle_filled(const Vector2D& center, float radius, ImU32 col, int num_segments = 0);
    pub fn add_circle_filled(&mut self, center: &Vector2D, radius: f32, col: u32, num_segments: i32) {
        todo!()
    }
    //  void  add_ngon(const Vector2D& center, float radius, ImU32 col, int num_segments, float thickness = 1.0);
    pub fn add_ngon(&mut self, center: &Vector2D, radius: f32, col: u32, num_segments: i32, thickness: f32) {
        todo!()
    }
    //  void  AddNgonFilled(const Vector2D& center, float radius, ImU32 col, int num_segments);
    pub fn AddNgonFilled(&mut self, center: &Vector2D, radius: f32, col: u32, num_segments: i32) {
        todo!()
    }
    //  void  add_text(const Vector2D& pos, ImU32 col, const char* text_begin, const char* text_end = NULL);
    pub fn AddText(&mut self, pos: &Vector2D, col: u32, text_begin: &String, text_end: &String) {
        todo!()
    }
    //  void  add_text(const ImFont* font, float font_size, const Vector2D& pos, ImU32 col, const char* text_begin, const char* text_end = NULL, float wrap_width = 0.0, const Vector4D* cpu_fine_clip_rect = NULL);
    pub fn AddText2(&mut self, font: &Font, font_size: f32, pos: &Vector2D, col: u32, text_begin: *const c_char, text_end: *const c_char, wrap_width: f32, cpu_fine_clip_rect: Option<&Vector4D>) {
        todo!()
    }
    //  void  AddPolyline(const Vector2D* points, int num_points, ImU32 col, ImDrawFlags flags, float thickness);
    pub fn AddPolyline(&mut self, points: &[Vector2D], num_points: usize, col: u32, flags: DrawFlags, thickness: f32) {
        todo!()
    }
    //  void  AddConvexPolyFilled(const Vector2D* points, int num_points, ImU32 col);
    pub fn AddConvexPolyFilled(&mut self, points: &[Vector2D], num_points: usize, col: u32) {
        todo!()
    }
    //  void  AddBezierCubic(const Vector2D& p1, const Vector2D& p2, const Vector2D& p3, const Vector2D& p4, ImU32 col, float thickness, int num_segments = 0); // Cubic Bezier (4 control points)
    pub fn AddBezierCubic(&mut self, p1: &Vector2D, p2: &Vector2D, p3: &Vector2D, p4: &Vector2D, col: u32, thickness: f32, num_segments: i32) {
        todo!()
    }
    //  void  AddBezierQuadratic(const Vector2D& p1, const Vector2D& p2, const Vector2D& p3, ImU32 col, float thickness, int num_segments = 0);               // Quadratic Bezier (3 control points)
    pub fn AddBezierQuadratic(&mut self, p1: &Vector2D, p2: &Vector2D, p3: &Vector2D, col: u32, thickness: f32, num_segments: i32) {
        todo!()
    }

    // Image primitives
    // - Read FAQ to understand what ImTextureID is.
    // - "p_min" and "p_max" represent the upper-left and lower-right corners of the rectangle.
    // - "uv_min" and "uv_max" represent the normalized texture coordinates to use for those corners. Using (0,0)->(1,1) texture coordinates will generally display the entire texture.
    //  void  AddImage(ImTextureID user_texture_id, const Vector2D& p_min, const Vector2D& p_max, const Vector2D& uv_min = Vector2D(0, 0), const Vector2D& uv_max = Vector2D(1, 1), ImU32 col = IM_COL32_WHITE);
    pub fn AddImage(&mut self, user_texture_id: TextureId, p_min: &Vector2D, p_max: &Vector2D, uv_min: &Vector2D, uv_max: &Vector2D, col: u32) {
        todo!()
    }
    //  void  AddImageQuad(ImTextureID user_texture_id, const Vector2D& p1, const Vector2D& p2, const Vector2D& p3, const Vector2D& p4, const Vector2D& uv1 = Vector2D(0, 0), const Vector2D& uv2 = Vector2D(1, 0), const Vector2D& uv3 = Vector2D(1, 1), const Vector2D& uv4 = Vector2D(0, 1), ImU32 col = IM_COL32_WHITE);
    pub fn AddImageQuad(&mut self, user_texture_id: TextureId, p1: &Vector2D, p2: &Vector2D, p3: &Vector2D, p4: &Vector2D, uv1: &Vector2D, uv2: &Vector2D, uv3: &Vector2D, uv4: &Vector2D, col: u32) {
        todo!()
    }
    //  void  AddImageRounded(ImTextureID user_texture_id, const Vector2D& p_min, const Vector2D& p_max, const Vector2D& uv_min, const Vector2D& uv_max, ImU32 col, float rounding, ImDrawFlags flags = 0);
    pub fn AddImageRounded(&mut self, user_texture_id: TextureId, p_min: &Vector2D, p_max: &Vector2D, uv_min: &Vector2D, uv_max: &Vector2D, col: u32, rounding: f32, flags: DrawFlags) {
        todo!()
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
    pub fn PathArcTo(&mut self, center: &Vector2D, radius: f32, a_min: f32, a_max: f32, num_segments: i32) {
        todo!()
    }
    //  void  PathArcToFast(const Vector2D& center, float radius, int a_min_of_12, int a_max_of_12);                // Use precomputed angles for a 12 steps circle
    pub fn path_arc_to_fast(&mut self, center: &Vector2D, radius: f32, a_min_of_12: i32, a_max_of_12: i32) {
        todo!()
    }
    //  void  PathBezierCubicCurveTo(const Vector2D& p2, const Vector2D& p3, const Vector2D& p4, int num_segments = 0); // Cubic Bezier (4 control points)
    pub fn PathBezierCubicCurveTo(&mut self, p2: &Vector2D, p3: &Vector2D, p4: &Vector2D, num_segments: usize) {
        todo!()
    }
    //  void  PathBezierQuadraticCurveTo(const Vector2D& p2, const Vector2D& p3, int num_segments = 0);               // Quadratic Bezier (3 control points)
    pub fn PathBezierQuadraticCurveTo(&mut self, p2: &Vector2D, p3: &Vector2D, num_segments: usize) {
        todo!()
    }
    //  void  PathRect(const Vector2D& rect_min, const Vector2D& rect_max, float rounding = 0.0, ImDrawFlags flags = 0);
    pub fn PathRect(&mut self, rect_min: &Vector2D, rect_max: &Vector2D, rounding: f32, flags: DrawFlags) {
        todo!()
    }

    // Advanced
    //  void  AddCallback(ImDrawCallback callback, void* callback_data);  // Your rendering function must check for 'user_callback' in ImDrawCmd and call the function instead of rendering triangles.
    pub fn AddCallback(&mut self, callback: DimgDrawCallback, callback_data: *mut c_void) {
        todo!()
    }
    //  void  AddDrawCmd();                                               // This is useful if you need to forcefully create a new draw call (to allow for dependent rendering / blending). Otherwise primitives are merged into the same draw-call as much as possible
    pub fn AddDrawCmd(&mut self) {
        todo!()
    }
    //  ImDrawList* CloneOutput() const;                                  // Create a clone of the cmd_buffer/idx_buffer/vtx_buffer.
    pub fn CloneOutput(&mut self) -> Vec<DrawList> {
        todo!()
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
        todo!()
    }
    //  void  PrimUnreserve(int idx_count, int vtx_count);
    pub fn PrimUnreserve(&mut self, idx_count: usize, vtx_count: usize) {
        todo!()
    }
    //  void  PrimRect(const Vector2D& a, const Vector2D& b, ImU32 col);      // Axis aligned rectangle (composed of two triangles)
    pub fn PrimRect(&mut self, a: &Vector2D, b: &Vector2D, col: u32) {
        todo!()
    }
    //  void  PrimRectUV(const Vector2D& a, const Vector2D& b, const Vector2D& uv_a, const Vector2D& uv_b, ImU32 col);
    pub fn PrimRectUV(&mut self, a: &Vector2D, b: &Vector2D, uv_a: &Vector2D, uv_b: &Vector2D, col: u32) {
        todo!()
    }
    //  void  PrimQuadUV(const Vector2D& a, const Vector2D& b, const Vector2D& c, const Vector2D& d, const Vector2D& uv_a, const Vector2D& uv_b, const Vector2D& uv_c, const Vector2D& uv_d, ImU32 col);
    pub fn PrimQuadUV(&mut self, a: &Vector2D, b: &Vector2D, c: &Vector2D, d: &Vector2D, uv_a: &Vector2D, uv_b: &Vector2D, uv_c: &Vector2D, uv_d: &Vector2D, col: u32) {
        todo!()
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

    // [Internal helpers]
    //  void  _ResetForNewFrame();
    fn reset_for_new_frame(&mut self) {

    //     // Verify that the ImDrawCmd fields we want to memcmp() are contiguous in memory.
        //     IM_STATIC_ASSERT(IM_OFFSETOF(ImDrawCmd, ClipRect) == 0);
        //     IM_STATIC_ASSERT(IM_OFFSETOF(ImDrawCmd, TextureId) == sizeof(Vector4D));
        //     IM_STATIC_ASSERT(IM_OFFSETOF(ImDrawCmd, VtxOffset) == sizeof(Vector4D) + sizeof(ImTextureID));
        //     if (_splitter._Count > 1)
        //         _splitter.Merge(this);
        if self._splitter.count > 1 {
            self._splitter.merge(self)
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
        //     _VtxWritePtr = NULL;
        //     _IdxWritePtr = NULL;
        //     _clip_rect_stack.resize(0);
        //     _texture_id_stack.resize(0);
        //     _path.resize(0);
        //     _splitter.clear();
        //     cmd_buffer.push_back(ImDrawCmd());
        //     _fringe_scale = 1.0;

    }
    //  void  _ClearFreeMemory();
    fn ClearFreeMemory(&mut self){todo!()}
    //  void  _PopUnusedDrawCmd();
    fn PopUnusedDrawCmd(&mut self) {todo!()}
    //  void  _TryMergeDrawCmds();
    fn TryMergeDrawCmds(&mut self) { todo!()}
    //  void  _OnChangedClipRect();
    fn OnChangedClipRect(&mut self) {todo!()}
    //  void  _OnChangedTextureID();
    fn OnChangedTextureID(&mut self) {todo!()}
    //  void  _OnChangedVtxOffset();
    fn OnChangedVtxOffset(&mut self) {todo!()}
    //  int   _CalcCircleAutoSegmentCount(float radius) const;
    fn CalCircleAUtoSegmentCount(&mut self, radius: f32) -> i32 {
        todo!()
    }
    //  void  _PathArcToFastEx(const Vector2D& center, float radius, int a_min_sample, int a_max_sample, int a_step);
    fn PathArcToFastEx(&mut self, center: &Vector2D, radius: f32, a_min_simple: i32, a_max_sample: i32, a_step: i32) {
        todo!()
    }
    //  void  _PathArcToN(const Vector2D& center, float radius, float a_min, float a_max, int num_segments);
    fn PathArcToN(&mut self, center: &Vector2D, radius: f32, a_min: f32, a_max: f32, num_segments: i32) {
        todo!()
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
    f32::clamp(f32::round(f32::ceil(PI / f32::acos(1 - f32::min(max_error, (radius)) / (radius)))), DIMG_DRAW_LIST_CIRCLE_AUTO_SEGMENT_MIN, DIMG_DRAW_LIST_CIRCLE_AUTO_SEGMENT_MAX)
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
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DrawListFlags
{
    None                    = 0,
    anti_aliased_lines        = 1 << 0,  // Enable anti-aliased lines/borders (*2 the number of triangles for 1.0 wide line or lines thin enough to be drawn using textures, otherwise *3 the number of triangles)
    anti_aliased_lines_use_tex  = 1 << 1,  // Enable anti-aliased lines/borders using textures when possible. Require backend to render with bilinear filtering (NOT point/nearest filtering).
    anti_aliased_fill         = 1 << 2,  // Enable anti-aliased edge around filled shapes (rounded rectangles, circles).
    AllowVtxOffset          = 1 << 3   // Can emit 'vtx_offset > 0' to allow large meshes. Set when 'ImGuiBackendFlags_RendererHasVtxOffset' is enabled.
}

// The maximum line width to bake anti-aliased textures for. build atlas with NoBakedLines to disable baking.
// #ifndef IM_DRAWLIST_TEX_LINES_WIDTH_MAX
// #define IM_DRAWLIST_TEX_LINES_WIDTH_MAX     (63)
// #endif
pub const IM_DRAWLIST_TEX_LINES_WIDTH_MAX: usize = 63;

/// static ImDrawList* get_viewport_draw_list(ImGuiViewportP* viewport, size_t drawlist_no, const char* drawlist_name)
pub fn get_viewport_draw_list(g: &mut Context, viewport: &mut Viewport, drawlist_no: usize, drawlist_name: &String) -> &mut DrawList
{
    // Create the draw list on demand, because they are not frequently used for all viewports
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(drawlist_no < IM_ARRAYSIZE(viewport->DrawLists));
    // ImDrawList* draw_list = viewport->DrawLists[drawlist_no];
    let draw_list = &mut viewport.draw_lists[drawlist_no];
    if draw_list.id == INVALID_ID
    {
        // draw_list = IM_NEW(ImDrawList)(&g.DrawListSharedData);
        viewport.draw_lists[drawlist_no] = DrawList::new(&mut g.draw_list_shared_data);
        // draw_list->_OwnerName = drawlist_name;
        viewport.draw_lists[drawlist_no].owner_name = drawlist_name.clone();
        // viewport.draw_lists[drawlist_no] = draw_list;
    }

    // Our ImDrawList system requires that there is always a command
    if viewport.draw_lists_last_frame[drawlist_no] != g.frame_count
    {
        draw_list.ResetForNewFrame();
        draw_list.PushTextureID(g.io.fonts.TexID);
        draw_list.PushClipRect(viewport.Pos, viewport.Pos + viewport.size, false);
        viewport.draw_lists_last_frame[drawlist_no] = g.frame_count;
    }
    return draw_list;
}

/// ImDrawList* ImGui::GetBackgroundDrawList(ImGuiViewport* viewport)
pub fn get_background_draw_list(g: &mut Context, viewport: &mut Viewport) -> &mut DrawList
{
    return get_viewport_draw_list(g, viewport, 0, &String::from("##Background"));
}

/// ImDrawList* ImGui::GetBackgroundDrawList()
pub fn get_background_draw_list2(g: &mut Context) -> &mut DrawList
{
    // ImGuiContext& g = *GImGui;
    //return GetBackgroundDrawList(g.CurrentWindow->Viewport);
    let curr_win = g.get_current_window()?;
    let vp = g.get_viewport(curr_win.viewport_id).unwrap();
    get_background_draw_list(g, vp)
}

/// ImDrawList* ImGui::GetForegroundDrawList(ImGuiViewport* viewport)
pub fn get_foreground_draw_list(g: &mut Context, viewport: &mut Viewport) -> &mut DrawList
{
    // return GetViewportDrawList((ImGuiViewportP*)viewport, 1, "##Foreground");
    get_viewport_draw_list(g, viewport, 1, &String::from("##Foreground"))
}

/// ImDrawList* ImGui::GetForegroundDrawList()
pub fn get_foreground_draw_list2(g: &mut Context) -> &mut DrawList
{
    // ImGuiContext& g = *GImGui;
    // return GetForegroundDrawList(g.CurrentWindow->Viewport);
    let curr_win = g.get_current_window()?;
    let vp= g.get_viewport(curr_win.viewport_id).unwrap();
    get_foreground_draw_list(g, vp)
}

// static void add_draw_list_to_draw_data(ImVector<ImDrawList*>* out_list, ImDrawList* draw_list)
pub fn add_draw_list_to_draw_data(ctx: &mut Context, out_list: &mut Vec<Id32>, draw_list_id: Id32)
{
    let draw_list = ctx.get_draw_list(draw_list_id).unwrap();
    if draw_list.cmd_buffer.is_empty() {return;}

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
