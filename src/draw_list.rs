#![allow(non_snake_case)]

use libc::{c_char, c_float, c_int, c_uint, c_void};
use crate::draw::ImDrawCallback;
use crate::draw_cmd::ImDrawCmd;
use crate::draw_cmd_header::ImDrawCmdHeader;
use crate::draw_flags::ImDrawFlags;
use crate::draw_list_flags::ImDrawListFlags;
use crate::draw_list_shared_data::ImDrawListSharedData;
use crate::draw_list_splitter::ImDrawListSplitter;
use crate::draw_vert::ImDrawVert;
use crate::font::ImFont;
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

    // void  PushClipRect(const ImVec2& clip_rect_min, const ImVec2& clip_rect_max, bool intersect_with_current_clip_rect = false);  // Render-level scissoring. This is passed down to your render function but not used for CPU-side coarse clipping. Prefer using higher-level PushClipRect() to affect logic (hit-testing and widget culling)
    pub fn PushClipRect(&mut self, clip_rect_min: &ImVec2, clip_rect_max: &ImVec2, intersect_with_current_clip_rect: bool) {}

    // void  PushClipRectFullScreen();
    pub fn PushClipRectFullScreen(&mut self) {}

    // void  PopClipRect();
    pub fn PopClipRect(&mut self) {}

    // void  PushTextureID(ImTextureID texture_id);
    pub fn PushTextureID(&mut self, texture_id: ImTextureID) {}

    // void  PopTextureID();
    pub fn PopTextureID(&mut self) {}

    // inline ImVec2   GetClipRectMin() const { const ImVec4& cr = _ClipRectStack.back(); return ImVec2(cr.x, cr.y); }
    pub fn GetClipRectMin(&mut self) -> ImVec2 {
        let cr = self._ClipRectStack.last().unwrap();
        return ImVec2::new2(cr.x, cr.y);
    }


    // inline ImVec2   GetClipRectMax() const { const ImVec4& cr = _ClipRectStack.back(); return ImVec2(cr.z, cr.w); }
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
// void  AddLine(const ImVec2& p1, const ImVec2& p2, u32 col, c_float thickness = 1f32);
    pub fn AddLine(&mut self, p1: &ImVec2, p2: &ImVec2, col: u32, thicknetss: c_float) {}

    // void  AddRect(const ImVec2& p_min, const ImVec2& p_max, u32 col, c_float rounding = 0f32, ImDrawFlags flags = 0, c_float thickness = 1f32);   // a: upper-left, b: lower-right (== upper-left + size)
    pub fn AddRect(&mut self, p_min: &ImVec2, p_max: &ImVec2, col: u32, rounding: c_float, flags: ImDrawFlags, thickness: f32) {}


    // void  AddRectFilled(const ImVec2& p_min, const ImVec2& p_max, u32 col, c_float rounding = 0f32, ImDrawFlags flags = 0);                     // a: upper-left, b: lower-right (== upper-left + size)
    pub fn AddRectFilled(&mut self, p_min: &ImVec2, p_masx: &ImVec2, col: u32, rounding: c_float, flags: ImDrawFlags) {}

    // void  AddRectFilledMultiColor(const ImVec2& p_min, const ImVec2& p_max, u32 col_upr_left, u32 col_upr_right, u32 col_bot_right, u32 col_bot_left);
    pub fn AddRectFilledMultiColor(&mut self, p_min: &ImVec2, p_max: &ImVec2, col_upr_left: u32, col_upr_right: u32, col_bot_right: u32, col_bot_left: u32) {}

    // void  AddQuad(const ImVec2& p1, const ImVec2& p2, const ImVec2& p3, const ImVec2& p4, u32 col, c_float thickness = 1f32);
    pub fn AddQuad(&mut self, p1: &ImVec2, p2: &ImVec2, p3: &ImVec2, p4: &ImVec2, col: u32, thickness: c_float) {}

    // void  AddQuadFilled(const ImVec2& p1, const ImVec2& p2, const ImVec2& p3, const ImVec2& p4, u32 col);
    pub fn AddQuadFilled(&mut self, p1: &ImVec2, p2: &ImVec2, p3: &ImVec2, p4: &ImVec2, col: u32) {}


    // void  AddTriangle(const ImVec2& p1, const ImVec2& p2, const ImVec2& p3, u32 col, c_float thickness = 1f32);
    pub fn AddTriangle(&mut self, p1: &ImVec2, p2: &ImVec2, p3: &ImVec2, col: u32, thickness: c_float) {}


    // void  AddTriangleFilled(const ImVec2& p1, const ImVec2& p2, const ImVec2& p3, u32 col);
    pub fn AddTriangleFilled(&mut self, p1: &ImVec2, p2: &ImVec2, p3: &ImVec2, col: u32) {}

    // void  AddCircle(const ImVec2& center, c_float radius, u32 col, c_int num_segments = 0, c_float thickness = 1f32);
    pub fn AddCircle(&mut self, center: &ImVec2, radius: c_float, col: u32, num_segments: c_int, thickness: c_float) {}


    // void  AddCircleFilled(const ImVec2& center, c_float radius, u32 col, c_int num_segments = 0);
    pub fn AddCircleFilled(&mut self, center: &ImVec2, radius: c_float, col: u32, num_segments: c_int) {}

    // void  AddNgon(const ImVec2& center, c_float radius, u32 col, c_int num_segments, c_float thickness = 1f32);
    pub fn AddNgon(&mut self, center: &ImVec2, radius: c_float, col: u32, num_semgnets: c_int, thickness: c_float) {}


    // void  AddNgonFilled(const ImVec2& center, c_float radius, u32 col, c_int num_segments);
    pub fn AddNgonFilled(&mut self, center: &ImVec2, radius: c_float, col: u32, num_segments: c_int) {}


    // void  AddText(const ImVec2& pos, u32 col, const char* text_begin, const char* text_end = NULL);
    pub fn AddText(&mut self, pos: *const ImVec2, col: u32, text_begin: *const c_char, text_end: *const c_char) {}

    // void  AddText(const ImFont* font, c_float font_size, const ImVec2& pos, u32 col, const char* text_begin, const char*
// text_end = NULL, c_float wrap_width = 0f32, const ImVec4* cpu_fine_clip_rect = NULL);
    pub fn AddText2(&mut self, font: *const ImFont, font_size: c_float, pos: &ImVec2, col: u32, text_begin: *const c_char, text_end: *const c_char, wrap_width: c_float, cpu_fine_clip_rect: *const ImVec4) {}


    // void  AddPolyline(const points: *mut ImVec2, c_int num_points, u32 col, ImDrawFlags flags, c_float thickness);
    pub fn AddPolyline(&mut self, points: *const ImVec2, num_points: c_int, col: u32, flags: ImDrawFlags, thickness: c_float) {}


    // void  AddConvexPolyFilled(const points: *mut ImVec2, c_int num_points, u32 col);
    pub fn AddConvexPolyFilled(&mut self, points: *const ImVec2, num_points: c_int, col: u32) {}

    // void  AddBezierCubic(const ImVec2& p1, const ImVec2& p2, const ImVec2& p3, const ImVec2& p4, u32 col, c_float thickness, c_int num_segments = 0); // Cubic Bezier (4 control points)
    pub fn AddBezierCubic(&mut self, p1: &ImVec2, p2: &ImVec2, p3: &ImVec2, p4: &ImVec2, col: u32, thickness: c_float, num_segments: c_int) {}

    // void  AddBezierQuadratic(const ImVec2& p1, const ImVec2& p2, const ImVec2& p3, u32 col, c_float thickness, c_int num_segments = 0);               // Quadratic Bezier (3 control points)
    pub fn AddBezierQuadratic(&mut self, p1: &ImVec2, p2: &ImVec2, p3: &ImVec2, col: u32, thickness: c_float, num_segments: c_int) {}

    // Image primitives
// - Read FAQ to understand what ImTextureID is.
// - "p_min" and "p_max" represent the upper-left and lower-right corners of the rectangle.
// - "uv_min" and "uv_max" represent the normalized texture coordinates to use for those corners. Using (0,0)->(1,1) texture coordinates will generally display the entire texture.
// void  AddImage(ImTextureID user_texture_id, const ImVec2& p_min, const ImVec2& p_max, const ImVec2& uv_min = ImVec2::new2(0, 0), const ImVec2& uv_max = ImVec2::new2(1, 1), u32 col = IM_COL32_WHITE);
    pub fn AddImage(&mut self, user_texture_id: ImTextureID, p_min: &ImVec2, p_max: &ImVec2, uv_ming: &ImVec2, uv_max: &ImVec2, col: u32) {}


    // void  AddImageQuad(ImTextureID user_texture_id, const ImVec2& p1, const ImVec2& p2, const ImVec2& p3, const ImVec2& p4, const ImVec2& uv1 = ImVec2::new2(0, 0), const ImVec2& uv2 = ImVec2::new2(1, 0), const ImVec2& uv3 = ImVec2::new2(1, 1), const ImVec2& uv4 = ImVec2::new2(0, 1), u32 col = IM_COL32_WHITE);
    pub fn AddImageQuad(&mut self, user_texture_id: ImTextureID, p1: &ImVec2, p2: &ImVec2, p3: &ImVec2, p4: &ImVec2, uv1: &ImVec2, uv2: &ImVec2, uv3: &ImVec2, uv4: &ImVec2, col: u32) {
        todo!()
    }


    // void  AddImageRounded(ImTextureID user_texture_id, const ImVec2& p_min, const ImVec2& p_max, const ImVec2& uv_min, const ImVec2& uv_max, u32 col, c_float rounding, ImDrawFlags flags = 0);
    pub fn AddImageRounded(&mut self, user_texture_id: ImTextureID, p_min: &ImVec2, p_max: &ImVec2, uv_min: &ImVec2, uv_max: &ImVec2, col: u32, rounding: c_float, flags: ImDrawFlags) {}

    // Stateful path API, add points then finish with PathFillConvex() or PathStroke()
// - Filled shapes must always use clockwise winding order. The anti-aliasing fringe depends on it. Counter-clockwise shapes will have "inward" anti-aliasing.
// inline    void  PathClear()                                                 { _Path.Size = 0; }
    pub fn PathClear(&mut self) { self._Path.clear() }

    // inline    void  PathLineTo(const ImVec2& pos)                               { _Path.push(pos); }
    pub fn PathLineTo(&mut self, pos: &ImVec2) {
        self._Path.push(pos.clone())
    }

    // inline    void  PathLineToMergeDuplicate(const ImVec2& pos)
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


    // void  PathArcTo(const ImVec2& center, c_float radius, c_float a_min, c_float a_max, c_int num_segments = 0);
    pub fn PathArcTo(&mut self, center: &ImVec2, radius: c_float, a_min: c_float, a_max: c_float, num_segments: c_int) {
        todo!()
    }


    // void  PathArcToFast(const ImVec2& center, c_float radius, c_int a_min_of_12, c_int a_max_of_12);                // Use precomputed angles for a 12 steps circle
    pub fn PathArcToFast(&mut self, center: &ImVec2, radius: c_float, a_min_of_12: c_int, a_max_of_12: c_int) {
        todo!()
    }


    // void  PathBezierCubicCurveTo(const ImVec2& p2, const ImVec2& p3, const ImVec2& p4, c_int num_segments = 0); // Cubic Bezier (4 control points)
    pub fn PathBezierCubicCurveTo(&mut self, p2: &ImVec2, p3: &ImVec2, p4: &ImVec2, num_segments: c_int) {
        todo!()
    }

    // void  PathBezierQuadraticCurveTo(const ImVec2& p2, const ImVec2& p3, c_int num_segments = 0);               // Quadratic Bezier (3 control points)
    pub fn PathBezierQuadraticCurveTo(&mut self, p2: &ImVec2, p3: &ImVec2, num_segments: c_int) {
        todo!()
    }

    // void  PathRect(const ImVec2& rect_min, const ImVec2& rect_max, c_float rounding = 0f32, ImDrawFlags flags = 0);
    pub fn PathRect(&mut self, rect_min: &ImVec2, rect_max: &ImVec2, rounding: c_float, flags: ImDrawFlags) {
        todo!()
    }

    // Advanced
// void  AddCallback(ImDrawCallback callback, void* callback_data);  // Your rendering function must check for 'UserCallback' in ImDrawCmd and call the function instead of rendering triangles.
    pub fn AddCallback(&mut self, callback: ImDrawCallback, callback_data: *mut c_void) {
        todo!()
    }

    // void  AddDrawCmd();                                               // This is useful if you need to forcefully create a new draw call (to allow for dependent rendering / blending). Otherwise primitives are merged into the same draw-call as much as possible
    pub fn AddDrawCmd(&mut self) {
        todo!()
    }

    // ImDrawList* CloneOutput() const;                                  // Create a clone of the CmdBuffer/IdxBuffer/VtxBuffer.
    pub fn CloneOutpost(&mut self) -> *mut ImDrawList {
        todo!()
    }

    // Advanced: Channels
// - Use to split render into layers. By switching channels to can render out-of-order (e.g. submit FG primitives before BG primitives)
// - Use to minimize draw calls (e.g. if going back-and-forth between multiple clipping rectangles, prefer to append into separate channels then merge at the end)
// - FIXME-OBSOLETE: This API shouldn't have been in ImDrawList in the first place!
//   Prefer using your own persistent instance of ImDrawListSplitter as you can stack them.
//   Using the ImDrawList::ChannelsXXXX you cannot stack a split over another.
// inline void     ChannelsSplit(c_int count)    { _Splitter.Split(this, count); }
    pub fn ChannelsSplit(&mut self, count: c_int) {
        self._Splitter.Split(self, count)
    }


    // inline void     ChannelsMerge()             { _Splitter.Merge(this); }
    pub fn ChannelsMerge(&mut self) {
        self._Splitter.Merge(self)
    }

    // inline void     ChannelsSetCurrent(c_int n)   { _Splitter.SetCurrentChannel(this, n); }
    pub fn ChannelsSetCurrent(&mut self, n: c_int) {
        self._Splitter.SetCurrentChannel(self, n)
    }

    // Advanced: Primitives allocations
// - We render triangles (three vertices)
// - All primitives needs to be reserved via PrimReserve() beforehand.
// void  PrimReserve(c_int idx_count, c_int vtx_count);
    pub fn PrimReserve(&mut self, idx_count: c_int, vtx_count: c_int) {
        todo!()
    }

    // void  PrimUnreserve(c_int idx_count, c_int vtx_count);
    pub fn PrimUnreserve(&mut self, idx_count: c_int, vtx_count: c_int) {
        todo!()
    }

    // void  PrimRect(const ImVec2& a, const ImVec2& b, u32 col);      // Axis aligned rectangle (composed of two triangles)
    pub fn PrimRect(&mut self, a: &ImVec2, b: &ImVec2, col: u32) {
        todo!()
    }


    // void  PrimRectUV(const ImVec2& a, const ImVec2& b, const ImVec2& uv_a, const ImVec2& uv_b, u32 col);
    pub fn PrimRectUV(&mut self, a: &ImVec2, b: &ImVec2, uv_a: &imVec2, uv_b: &ImVec2, col: u32) {
        todo!()
    }


    // void  PrimQuadUV(const ImVec2& a, const ImVec2& b, const ImVec2& c, const ImVec2& d, const ImVec2& uv_a, const ImVec2& uv_b, const ImVec2& uv_c, const ImVec2& uv_d, u32 col);
    pub fn PrimQuadUV(&mut self, a: &ImVec2, b: &ImVec2, c: &ImVec2, d: &ImVec2, uv_a: &ImVec2, uv_b: &ImVec2, uv_c: &ImVec2, uv_d: &ImVec2, col: u32) {
        todo!()
    }

    // inline    void  PrimWriteVtx(const ImVec2& pos, const ImVec2& uv, u32 col)    { _VtxWritePtr.pos = pos; _VtxWritePtr.uv = uv; _VtxWritePtr.col = col; _VtxWritePtr+= 1; _VtxCurrentIdx+= 1; }
    pub fn PrimWriteVtx(&mut self, pos: &ImVec2, uv: &ImVec2, col: u32) {
        self._VtxWritePtr.pos = pos.clone();
        self._VtxWritePtr.uv = uv.clone();
        self._VtxWritePtr.col = col;
        self._VtxWritePtr += 1;
        self._VtxCurrentIdx += 1;
    }


    // inline    void  PrimWriteIdx(ImDrawIdx idx)                                     { *_IdxWritePtr = idx; _IdxWritePtr+= 1; }
    pub unsafe fn PrimWriteIdx(&mut self, idx: ImDrawIdx) {
        *self._IdxWritePtr = idx;
        self._IdxWritePtr += 1;
    }


    // inline    void  PrimVtx(const ImVec2& pos, const ImVec2& uv, u32 col)         { PrimWriteIdx((ImDrawIdx)_VtxCurrentIdx); PrimWriteVtx(pos, uv, col); } // Write vertex with unique index
    pub unsafe fn PrimVtx(&mut self, pos: &ImVec2, uv: &ImVec2, col: u32) {
        self.PrimWriteIdx(self._VtxCurrentIdx as ImDrawIdx);
        self.PrimWriteVtx(pos, uv, col);
    }

// #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
// inline    void  AddBezierCurve(const ImVec2& p1, const ImVec2& p2, const ImVec2& p3, const ImVec2& p4, u32 col, c_float thickness, c_int num_segments = 0) { AddBezierCubic(p1, p2, p3, p4, col, thickness, num_segments); } // OBSOLETED in 1.80 (Jan 2021)
// pub fn AddBezierCurve(&mut self, p1: &ImVec2, p2: &ImVec2, p3: &ImVec2, p4: &ImVec2, col: u32, thickness: c_float, num_segments: c_int) {
//     self.AddBezierCubic(p1,p2,p3,p4,col,thickness,num_segments)
// }

// inline    void  PathBezierCurveTo(const ImVec2& p2, const ImVec2& p3, const ImVec2& p4, c_int num_segments = 0) { PathBezierCubicCurveTo(p2, p3, p4, num_segments); } // OBSOLETED in 1.80 (Jan 2021)

// #endif

    // [Internal helpers]
// void  _ResetForNewFrame(); 
    pub(crate) fn _ResetForNewFrame(&mut self) {
        todo!()
    }

    // void  _ClearFreeMemory();
    pub fn _ClearFreeMemory(&mut self) {
        todo!()
    }

    // void  _PopUnusedDrawCmd();
    pub fn _PopUnusedDrawCmd(&mut self) {
        todo!()
    }

    // void  _TryMergeDrawCmds();
    pub fn _TryMergeDrawCmds(&mut self) {
        todo!()
    }

    // void  _OnChangedClipRect();
    pub fn _OnChangedClipRect(&mut self) {
        todo!()
    }

    // void  _OnChangedTextureID();
    pub fn _OnChangedTextureID(&mut self) {
        todo!()
    }

    // void  _OnChangedVtxOffset();
    pub fn _OnChangedVtxOffset(&mut self) {
        todo!()
    }

    // c_int   _CalcCircleAutoSegmentCount(c_float radius) const;
    pub fn _CalcCircleAutoSegment(&mut self, radius: f32) {
        todo!()
    }

    // void  _PathArcToFastEx(const ImVec2& center, c_float radius, c_int a_min_sample, c_int a_max_sample, c_int a_step);
    pub fn _PathArcToFastEx(&mut self, center: &ImVec2, radius: c_float, a_min_sample: c_int, a_max_sample: c_int, a_step: c_int) {
        todo!()
    }

    // void  _PathArcToN(const ImVec2& center, c_float radius, c_float a_min, c_float a_max, c_int num_segments);
    pub fn _PathArcToN(&mut self, center: &ImVec2, radius: c_float, a_min: c_float, a_max: c_float, num_segments: c_int) {
        todo!()
    }
}
