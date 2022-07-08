use std::ffi::c_void;
use std::os::raw::c_char;
use crate::defines::{DimgDrawCallback, DimgDrawCmd, ImDrawCmdHeader, DimgDrawFlags, ImDrawIdx, ImDrawListFlags, ImDrawListSplitter, DimgDrawVert, DimgFont, DimgTextureId};
use crate::vec_nd::{DimgVec2D, DimgVec4};

pub struct DimgDrawListSharedData
{
    // ImVec2          TexUvWhitePixel;            // UV of white pixel in the atlas
    pub TextUvWhitePixel: DimgVec2D,
    // ImFont*         font;                       // Current/default font (optional, for simplified AddText overload)
    pub Font: *mut DimgFont,
    // float           font_size;                   // Current/default font size (optional, for simplified AddText overload)
    pub FontSize: f32,
    // float           CurveTessellationTol;       // Tessellation tolerance when using PathBezierCurveTo()
    pub CurveTesselationTol: f32,
    // float           CircleSegmentMaxError;      // Number of circle segments to use per pixel of radius for AddCircle() etc
    pub CircleSegmentMaxError: f32,
    // ImVec4          ClipRectFullscreen;         // Value for PushClipRectFullscreen()
    pub ClipRectFullScreen: DimgVec4,
    // ImDrawListFlags InitialFlags;               // Initial flags at the beginning of the frame (it is possible to alter flags on a per-drawlist basis afterwards)
    pub InitialFlags: ImDrawListFlags,
    // [Internal] Lookup tables
    // ImVec2          ArcFastVtx[IM_DRAWLIST_ARCFAST_TABLE_SIZE]; // Sample points on the quarter of the circle.
    pub ArcFastVtx: Vec<DimgVec2D>,
    // float           ArcFastRadiusCutoff;                        // Cutoff radius after which arc drawing will fallback to slower PathArcTo()
    pub ArcFastRadiusCutoff: f32,
    // ImU8            CircleSegmentCounts[64];    // Precomputed segment count for given radius before we calculate it dynamically (to avoid calculation overhead)
    pub CircleSegmentCounts: [u8;64],
    // const ImVec4*   TexUvLines;                 // UV of anti-aliased lines in the atlas
    pub TexUvLines: *const DimgVec4,

    // ImDrawListSharedData();

    // void SetCircleTessellationMaxError(float max_error);
}

impl DimgDrawListSharedData {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn SetCircleTesselationMaxError(&mut self, max_error: f32) {
        todo!()
    }
}

/// Draw command list
/// This is the low-level list of polygons that ImGui:: functions are filling. At the end of the frame,
/// all command lists are passed to your ImGuiIO::RenderDrawListFn function for rendering.
/// Each dear imgui window contains its own ImDrawList. You can use ImGui::GetWindowDrawList() to
/// access the current window draw list and draw custom primitives.
/// You can interleave normal ImGui:: calls and adding primitives to the current draw list.
/// In single viewport mode, top-left is == GetMainViewport()->pos (generally 0,0), bottom-right is == GetMainViewport()->pos+size (generally io.DisplaySize).
/// You are totally free to apply whatever transformation matrix to want to the data (depending on the use of the transformation you may want to apply it to clip_rect as well!)
/// Important: Primitives are always added to the list and not culled (culling is done at higher-level by ImGui:: functions), if you use this API a lot consider coarse culling your drawn objects.
#[derive(Default,Debug,Clone)]
pub struct DimgDrawList
{
    // This is what you have to render
    // ImVector<ImDrawCmd>     CmdBuffer;          // Draw commands. Typically 1 command = 1 GPU draw call, unless the command is a callback.
    pub CmdBuffer: Vec<DimgDrawCmd>,
    // ImVector<ImDrawIdx>     IdxBuffer;          // Index buffer. Each command consume ImDrawCmd::elem_count of those
    pub IdxBuffer: Vec<ImDrawIdx>,
    // ImVector<ImDrawVert>    VtxBuffer;          // Vertex buffer.
    pub VtxBuffer: Vec<DimgDrawVert>,
    // ImDrawListFlags         flags;              // flags, you may poke into these to adjust anti-aliasing settings per-primitive.
    pub Flags: ImDrawListFlags,
    // [Internal, used while building lists]
    // unsigned pub _VtxCurrentIdx: i32,   // [Internal] generally == VtxBuffer.size unless we are past 64K vertices, in which case this gets reset to 0.
    // pub _VtxCurrentIdx: u32,
    // const ImDrawListSharedData* _Data;          // Pointer to shared draw data (you can use ImGui::GetDrawListSharedData() to get the one from current ImGui context)
    pub data: Box<DimgDrawListSharedData>,
    // const char*             _OwnerName;         // Pointer to owner window's name for debugging
    pub owner_name: String,
    // ImDrawVert*             _VtxWritePtr;       // [Internal] point within VtxBuffer.data after each add command (to avoid using the ImVector<> operators too much)
    // pub _VxWritePtr: *mut ImDrawVert,
    // ImDrawIdx*              _IdxWritePtr;       // [Internal] point within IdxBuffer.data after each add command (to avoid using the ImVector<> operators too much)
    // pub _IdxWritePtr: *mut ImDrawIdx,
    // ImVector<ImVec4>        _ClipRectStack;     // [Internal]
    pub _ClipRectStack: Vec<DimgVec4>,
    // ImVector<ImTextureID>   _TextureIdStack;    // [Internal]
    pub _TextureIdStack: Vec<DimgTextureId>,
    // ImVector<ImVec2>        _Path;              // [Internal] current path building
    pub _Path: Vec<DimgVec2D>,
    // ImDrawCmdHeader         _CmdHeader;         // [Internal] template of active commands. Fields should match those of CmdBuffer.back().
    pub _CmdHeader: ImDrawCmdHeader,
    // ImDrawListSplitter      _Splitter;          // [Internal] for channels api (note: prefer using your own persistent instance of ImDrawListSplitter!)
    pub _Splitter: ImDrawListSplitter,
    // pub _FringeScale: f32,      // [Internal] anti-alias fringe is scaled by this value, this helps to keep things sharp while zooming at vertex buffer content
    pub _FringeScale: f32,
}

impl DimgDrawList {
     // If you want to create ImDrawList instances, pass them ImGui::GetDrawListSharedData() or create and use your own ImDrawListSharedData (so you can use ImDrawList without ImGui)
    // ImDrawList(const ImDrawListSharedData* shared_data) { memset(this, 0, sizeof(*this)); _Data = shared_data; }
    pub fn new(shared_data: &mut DimgDrawListSharedData) -> Self {
         Self {
             data: shared_data,
             ..Default::default()
         }
     }
    // ~ImDrawList() { _ClearFreeMemory(); }
    //  void  PushClipRect(const ImVec2& clip_rect_min, const ImVec2& clip_rect_max, bool intersect_with_current_clip_rect = false);  // Render-level scissoring. This is passed down to your render function but not used for CPU-side coarse clipping. Prefer using higher-level ImGui::PushClipRect() to affect logic (hit-testing and widget culling)
    pub fn PushClipRect(&mut self, clip_rect_min: &DimgVec2D, clip_rect_max: &DimgVec2D, intersect_with_current_clip_rect: bool) {
        todo!()
    }
    //  void  PushClipRectFullScreen();
    pub fn PushClipRectFullScreen(&mut self) {
        todo!()
    }
    //  void  PopClipRect();
    pub fn PopClipRect(&mut self) {
        todo!()
    }
    //  void  PushTextureID(ImTextureID texture_id);
    pub fn PushTextureID(&mut self, texture_id: DimgTextureId) {todo!()}
    //  void  PopTextureID();
    pub fn PopTextureID(&mut self) {todo!()}
    // inline ImVec2   GetClipRectMin() const { const ImVec4& cr = _ClipRectStack.back(); return ImVec2(cr.x, cr.y); }
    pub fn GetClipRectMin(&self) -> DimgVec2D {
        let cr = self._ClipRectStack.back();
        DimgVec2D::new(cr.x, cr.y)
    }
    // inline ImVec2   GetClipRectMax() const { const ImVec4& cr = _ClipRectStack.back(); return ImVec2(cr.z, cr.w); }
    pub fn GetClipRectMax(&self) -> DimgVec2D {
        let cr = self._ClipRectStack.back();
        DimgVec2D::new(cr.z, cr.w)
    }
    // Primitives
    // - Filled shapes must always use clockwise winding order. The anti-aliasing fringe depends on it. Counter-clockwise shapes will have "inward" anti-aliasing.
    // - For rectangular primitives, "p_min" and "p_max" represent the upper-left and lower-right corners.
    // - For circle primitives, use "num_segments == 0" to automatically calculate tessellation (preferred).
    //   In older versions (until Dear ImGui 1.77) the AddCircle functions defaulted to num_segments == 12.
    //   In future versions we will use textures to provide cheaper and higher-quality circles.
    //   Use AddNgon() and AddNgonFilled() functions if you need to guaranteed a specific number of sides.
    //  void  AddLine(const ImVec2& p1, const ImVec2& p2, ImU32 col, float thickness = 1.0);
    pub fn AddLine(&mut self, p1: &DimgVec2D, p2: &DimgVec2D, col: u32, thickness: f32) {
        todo!()
    }
    //  void  AddRect(const ImVec2& p_min, const ImVec2& p_max, ImU32 col, float rounding = 0.0, ImDrawFlags flags = 0, float thickness = 1.0);   // a: upper-left, b: lower-right (== upper-left + size)
    pub fn AddRect(&mut self, p_min: &DimgVec2D, p_max: DimgVec2D, col: u32, rounding: f32, flags: DimgDrawFlags, thickness: f32) {
        todo!()
    }
    //  void  AddRectFilled(const ImVec2& p_min, const ImVec2& p_max, ImU32 col, float rounding = 0.0, ImDrawFlags flags = 0);                     // a: upper-left, b: lower-right (== upper-left + size)
    pub fn AddRectFilled(&mut self, p_min: &DimgVec2D, p_max: &DimgVec2D, col: u32, rounding: f32, flags: f32) {
        todo!()
    }
    //  void  AddRectFilledMultiColor(const ImVec2& p_min, const ImVec2& p_max, ImU32 col_upr_left, ImU32 col_upr_right, ImU32 col_bot_right, ImU32 col_bot_left);
    pub fn AddRectFilledMultiColor(&mut self, p_min: &DimgVec2D, p_max: &DimgVec2D, col_upr_left: u32, col_upr_right: u32, col_bot_right: u32, col_bot_left: u32) {
        todo!()
    }
    //  void  AddQuad(const ImVec2& p1, const ImVec2& p2, const ImVec2& p3, const ImVec2& p4, ImU32 col, float thickness = 1.0);
    pub fn AddQuad(&mut self, p1: &DimgVec2D, p2: &DimgVec2D, p3: &DimgVec2D, p4: &DimgVec2D, col: u32, thickness: f32) {
        todo!()
    }
    //  void  AddQuadFilled(const ImVec2& p1, const ImVec2& p2, const ImVec2& p3, const ImVec2& p4, ImU32 col);
    pub fn AddQuadFilled(&mut self, p1: &DimgVec2D, p2: &DimgVec2D, p3: &DimgVec2D, p4: &DimgVec2D, col: u32) {
        todo!()
    }
    //  void  AddTriangle(const ImVec2& p1, const ImVec2& p2, const ImVec2& p3, ImU32 col, float thickness = 1.0);
    pub fn AddTriangle(&mut self, p1: &DimgVec2D, p2: &DimgVec2D, p3: &DimgVec2D, col: u32, thickness: f32) {
        todo!()
    }
    //  void  AddTriangleFilled(const ImVec2& p1, const ImVec2& p2, const ImVec2& p3, ImU32 col);
    pub fn AddTriangleFilled(&mut self, p1: &DimgVec2D, p2: &DimgVec2D, p3: &DimgVec2D, col: u32) {
        todo!()
    }
    //  void  AddCircle(const ImVec2& center, float radius, ImU32 col, int num_segments = 0, float thickness = 1.0);
    pub fn AddCircle(&mut self, center: &DimgVec2D, radius: f32, col: u32, num_segments: i32, thickness: f32) {
        todo!()
    }
    //  void  AddCircleFilled(const ImVec2& center, float radius, ImU32 col, int num_segments = 0);
    pub fn AddCircleFilled(&mut self, center: &DimgVec2D, radius: f32, col: u32, num_segments: i32) {
        todo!()
    }
    //  void  AddNgon(const ImVec2& center, float radius, ImU32 col, int num_segments, float thickness = 1.0);
    pub fn AddNgon(&mut self, center: &DimgVec2D, radius: f32, col: u32, num_segments: i32, thickness: f32) {
        todo!()
    }
    //  void  AddNgonFilled(const ImVec2& center, float radius, ImU32 col, int num_segments);
    pub fn AddNgonFilled(&mut self, center: &DimgVec2D, radius: f32, col: u32, num_segments: i32) {
        todo!()
    }
    //  void  AddText(const ImVec2& pos, ImU32 col, const char* text_begin, const char* text_end = NULL);
    pub fn AddText(&mut self, pos: &DimgVec2D, col: u32, text_begin: &String, text_end: &String) {
        todo!()
    }
    //  void  AddText(const ImFont* font, float font_size, const ImVec2& pos, ImU32 col, const char* text_begin, const char* text_end = NULL, float wrap_width = 0.0, const ImVec4* cpu_fine_clip_rect = NULL);
    pub fn AddText2(&mut self, font: &DimgFont, font_size: f32, pos: &DimgVec2D, col: u32, text_begin: *const c_char, text_end: *const c_char, wrap_width: f32, cpu_fine_clip_rect: Option<&DimgVec4>) {
        todo!()
    }
    //  void  AddPolyline(const ImVec2* points, int num_points, ImU32 col, ImDrawFlags flags, float thickness);
    pub fn AddPolyline(&mut self, points: &[DimgVec2D], num_points: usize, col: u32, flags: DimgDrawFlags, thickness: f32) {
        todo!()
    }
    //  void  AddConvexPolyFilled(const ImVec2* points, int num_points, ImU32 col);
    pub fn AddConvexPolyFilled(&mut self, points: &[DimgVec2D], num_points: usize, col: u32) {
        todo!()
    }
    //  void  AddBezierCubic(const ImVec2& p1, const ImVec2& p2, const ImVec2& p3, const ImVec2& p4, ImU32 col, float thickness, int num_segments = 0); // Cubic Bezier (4 control points)
    pub fn AddBezierCubic(&mut self, p1: &DimgVec2D, p2: &DimgVec2D, p3: &DimgVec2D, p4: &DimgVec2D, col: u32, thickness: f32, num_segments: i32) {
        todo!()
    }
    //  void  AddBezierQuadratic(const ImVec2& p1, const ImVec2& p2, const ImVec2& p3, ImU32 col, float thickness, int num_segments = 0);               // Quadratic Bezier (3 control points)
    pub fn AddBezierQuadratic(&mut self, p1: &DimgVec2D, p2: &DimgVec2D, p3: &DimgVec2D, col: u32, thickness: f32, num_segments: i32) {
        todo!()
    }

    // Image primitives
    // - Read FAQ to understand what ImTextureID is.
    // - "p_min" and "p_max" represent the upper-left and lower-right corners of the rectangle.
    // - "uv_min" and "uv_max" represent the normalized texture coordinates to use for those corners. Using (0,0)->(1,1) texture coordinates will generally display the entire texture.
    //  void  AddImage(ImTextureID user_texture_id, const ImVec2& p_min, const ImVec2& p_max, const ImVec2& uv_min = ImVec2(0, 0), const ImVec2& uv_max = ImVec2(1, 1), ImU32 col = IM_COL32_WHITE);
    pub fn AddImage(&mut self, user_texture_id: DimgTextureId, p_min: &DimgVec2D, p_max: &DimgVec2D, uv_min: &DimgVec2D, uv_max: &DimgVec2D, col: u32) {
        todo!()
    }
    //  void  AddImageQuad(ImTextureID user_texture_id, const ImVec2& p1, const ImVec2& p2, const ImVec2& p3, const ImVec2& p4, const ImVec2& uv1 = ImVec2(0, 0), const ImVec2& uv2 = ImVec2(1, 0), const ImVec2& uv3 = ImVec2(1, 1), const ImVec2& uv4 = ImVec2(0, 1), ImU32 col = IM_COL32_WHITE);
    pub fn AddImageQuad(&mut self, user_texture_id: DimgTextureId, p1: &DimgVec2D, p2: &DimgVec2D, p3: &DimgVec2D, p4: &DimgVec2D, uv1: &DimgVec2D, uv2: &DimgVec2D, uv3: &DimgVec2D, uv4: &DimgVec2D, col: u32) {
        todo!()
    }
    //  void  AddImageRounded(ImTextureID user_texture_id, const ImVec2& p_min, const ImVec2& p_max, const ImVec2& uv_min, const ImVec2& uv_max, ImU32 col, float rounding, ImDrawFlags flags = 0);
    pub fn AddImageRounded(&mut self, user_texture_id: DimgTextureId, p_min: &DimgVec2D, p_max: &DimgVec2D, uv_min: &DimgVec2D, uv_max: &DimgVec2D, col: u32, rounding: f32, flags: DimgDrawFlags) {
        todo!()
    }

    // Stateful path API, add points then finish with PathFillConvex() or PathStroke()
    // - Filled shapes must always use clockwise winding order. The anti-aliasing fringe depends on it. Counter-clockwise shapes will have "inward" anti-aliasing.
    // inline    void  PathClear()                                                 { _Path.size = 0; }
    pub fn PathClear(&mut self) {
        self._Path.Size = 0
    }
    // inline    void  PathLineTo(const ImVec2& pos)                               { _Path.push_back(pos); }
    pub fn PathLineTo(&mut self, pos: &DimgVec2D) {
        self._Path.push(pos.clone())
    }
    // inline    void  PathLineToMergeDuplicate(const ImVec2& pos)                 { if (_Path.size == 0 || memcmp(&_Path.data[_Path.size - 1], &pos, 8) != 0) _Path.push_back(pos); }
    pub fn PathLineToMergeDuplicate(&mut self, pos: &DimgVec2D) {
        if self._Path.len() == 0 || (self._Path[self._Path.len() - 1] != pos) {
            self._Path.push(pos.clone())
        }
    }
    // inline    void  PathFillConvex(ImU32 col)                                   { AddConvexPolyFilled(_Path.data, _Path.size, col); _Path.size = 0; }
    pub fn PathFillConvex(&mut self, col: u32) {
        self.AddConvexPolyFilled(self._Path.as_slice(), self._Path.len(), col);
        self._Path.clear()
    }
    // inline    void  PathStroke(ImU32 col, ImDrawFlags flags = 0, float thickness = 1.0) { AddPolyline(_Path.data, _Path.size, col, flags, thickness); _Path.size = 0; }
    pub fn PathStroke(&mut self, col: u32, flags: DimgDrawFlags, thickness: f32) {
        self.AddPolyline(self._Path.as_slice(), self._Path.len(), col, flags, thickness);
        self._Path.clear()
    }
    //  void  PathArcTo(const ImVec2& center, float radius, float a_min, float a_max, int num_segments = 0);
    pub fn PathArcTo(&mut self, center: &DimgVec2D, radius: f32, a_min: f32, a_max: f32, num_segments: i32) {
        todo!()
    }
    //  void  PathArcToFast(const ImVec2& center, float radius, int a_min_of_12, int a_max_of_12);                // Use precomputed angles for a 12 steps circle
    pub fn PathArcToFast(&mut self, center: &DimgVec2D, radius: f32, a_min_of_12: i32, a_max_of_12: i32) {
        todo!()
    }
    //  void  PathBezierCubicCurveTo(const ImVec2& p2, const ImVec2& p3, const ImVec2& p4, int num_segments = 0); // Cubic Bezier (4 control points)
    pub fn PathBezierCubicCurveTo(&mut self, p2: &DimgVec2D, p3: &DimgVec2D, p4: &DimgVec2D, num_segments: usize) {
        todo!()
    }
    //  void  PathBezierQuadraticCurveTo(const ImVec2& p2, const ImVec2& p3, int num_segments = 0);               // Quadratic Bezier (3 control points)
    pub fn PathBezierQuadraticCurveTo(&mut self, p2: &DimgVec2D, p3: &DimgVec2D, num_segments: usize) {
        todo!()
    }
    //  void  PathRect(const ImVec2& rect_min, const ImVec2& rect_max, float rounding = 0.0, ImDrawFlags flags = 0);
    pub fn PathRect(&mut self, rect_min: &DimgVec2D, rect_max: &DimgVec2D, rounding: f32, flags: DimgDrawFlags) {
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
    //  ImDrawList* CloneOutput() const;                                  // Create a clone of the CmdBuffer/IdxBuffer/VtxBuffer.
    pub fn CloneOutput(&mut self) -> Vec<DimgDrawList> {
        todo!()
    }

    // Advanced: Channels
    // - Use to split render into layers. By switching channels to can render out-of-order (e.g. submit FG primitives before BG primitives)
    // - Use to minimize draw calls (e.g. if going back-and-forth between multiple clipping rectangles, prefer to append into separate channels then merge at the end)
    // - FIXME-OBSOLETE: This API shouldn't have been in ImDrawList in the first place!
    //   Prefer using your own persistent instance of ImDrawListSplitter as you can stack them.
    //   Using the ImDrawList::ChannelsXXXX you cannot stack a split over another.
    // inline void     ChannelsSplit(int count)    { _Splitter.split(this, count); }
    // inline void     ChannelsMerge()             { _Splitter.merge(this); }
    // inline void     ChannelsSetCurrent(int n)   { _Splitter.SetCurrentChannel(this, n); }

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
    //  void  PrimRect(const ImVec2& a, const ImVec2& b, ImU32 col);      // Axis aligned rectangle (composed of two triangles)
    pub fn PrimRect(&mut self, a: &DimgVec2D, b: &DimgVec2D, col: u32) {
        todo!()
    }
    //  void  PrimRectUV(const ImVec2& a, const ImVec2& b, const ImVec2& uv_a, const ImVec2& uv_b, ImU32 col);
    pub fn PrimRectUV(&mut self, a: &DimgVec2D, b: &DimgVec2D, uv_a: &DimgVec2D, uv_b: &DimgVec2D, col: u32) {
        todo!()
    }
    //  void  PrimQuadUV(const ImVec2& a, const ImVec2& b, const ImVec2& c, const ImVec2& d, const ImVec2& uv_a, const ImVec2& uv_b, const ImVec2& uv_c, const ImVec2& uv_d, ImU32 col);
    pub fn PrimQuadUV(&mut self, a: &DimgVec2D, b: &DimgVec2D, c: &DimgVec2D, d: &DimgVec2D, uv_a: &DimgVec2D, uv_b: &DimgVec2D, uv_c: &DimgVec2D, uv_d: &DimgVec2D, col: u32) {
        todo!()
    }
    // inline    void  PrimWriteVtx(const ImVec2& pos, const ImVec2& uv, ImU32 col)    { _VtxWritePtr->pos = pos; _VtxWritePtr->uv = uv; _VtxWritePtr->col = col; _VtxWritePtr++; _VtxCurrentIdx++; }
    pub fn PrimWriteVtx(&mut self, pos: &DimgVec2D, uv: &DimgVec2D, col: u32) {
        // TODO: replace VtxWritePtr with a vector of vertices
        self.VtxBuffer.push(DimgDrawVert {
            pos: pos.clone(),
            uv: uv.clone(),
            col,
        });
    }
    // inline    void  PrimWriteIdx(ImDrawIdx idx)                                     { *_IdxWritePtr = idx; _IdxWritePtr++; }
    pub fn PrimWriteIdx(&mut self, idx: ImDrawIdx) {
        self.IdxBuffer.push(idx)
    }
    // inline    void  PrimVtx(const ImVec2& pos, const ImVec2& uv, ImU32 col)         { PrimWriteIdx((ImDrawIdx)_VtxCurrentIdx); PrimWriteVtx(pos, uv, col); } // Write vertex with unique index
    pub fn PrimVtx(&mut self, pos: &DimgVec2D, uv: &DimgVec2D, col: u32) {
        self.PrimWriteVtx(pos, uv, col);
        self.PrimWriteIdx(0)
    }

// #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
//     inline    void  AddBezierCurve(const ImVec2& p1, const ImVec2& p2, const ImVec2& p3, const ImVec2& p4, ImU32 col, float thickness, int num_segments = 0) { AddBezierCubic(p1, p2, p3, p4, col, thickness, num_segments); } // OBSOLETED in 1.80 (Jan 2021)
//     inline    void  PathBezierCurveTo(const ImVec2& p2, const ImVec2& p3, const ImVec2& p4, int num_segments = 0) { PathBezierCubicCurveTo(p2, p3, p4, num_segments); } // OBSOLETED in 1.80 (Jan 2021)
// #endif

    // [Internal helpers]
    //  void  _ResetForNewFrame();
    fn ResetForNewFrame(&mut self) { todo!()}
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
    //  void  _PathArcToFastEx(const ImVec2& center, float radius, int a_min_sample, int a_max_sample, int a_step);
    fn PathArcToFastEx(&mut self, center: &DimgVec2D, radius: f32, a_min_simple: i32, a_max_sample: i32, a_step: i32) {
        todo!()
    }
    //  void  _PathArcToN(const ImVec2& center, float radius, float a_min, float a_max, int num_segments);
    fn PathArcToN(&mut self, center: &DimgVec2D, radius: f32, a_min: f32, a_max: f32, num_segments: i32) {
        todo!()
    }
}
