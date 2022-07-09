use std::fmt::{Debug, Formatter};
use crate::defines;
use crate::defines::DimgDrawCallback;
use crate::texture::DimgTextureId;
use crate::types::DimgId;
use crate::vec_nd::DimgVec4;

// Typically, 1 command = 1 GPU draw call (unless command is a callback)
// - vtx_offset: When 'io.backend_flags & ImGuiBackendFlags_RendererHasVtxOffset' is enabled,
//   this fields allow us to render meshes larger than 64K vertices while keeping 16-bit indices.
//   Backends made for <1.71. will typically ignore the vtx_offset fields.
// - The clip_rect/texture_id/vtx_offset fields must be contiguous as we memcmp() them together (this is asserted for).
#[derive(Default,Clone)]
pub struct DimgDrawCmd
{
    pub clip_rect: DimgVec4,           // 4*4  // Clipping rectangle (x1, y1, x2, y2). Subtract ImDrawData->display_pos to get clipping rectangle in "viewport" coordinates
    // ImTextureID     texture_id,          // 4-8  // User-provided texture id. Set by user in ImfontAtlas::set_tex_id() for fonts or passed to Image*() functions. Ignore if never using images or multiple fonts atlas.
    pub texture_id: DimgTextureId,
pub vtx_offset: i32,        // 4    // Start offset in vertex buffer. ImGuiBackendFlags_RendererHasVtxOffset: always 0, otherwise may be >0 to support meshes larger than 64K vertices with 16-bit indices.
    pub idx_offset: i32,        // 4    // Start offset in index buffer.
    pub elem_count: i32,        // 4    // Number of indices (multiple of 3) to be rendered as triangles. Vertices are stored in the callee ImDrawList's vtx_buffer[] array, indices in idx_buffer[].
    // ImDrawCallback  user_callback;       // 4-8  // If != NULL, call the function instead of rendering the vertices. clip_rect and texture_id will be set normally.
    pub user_callback: Option<DimgDrawCallback>,
    // void*           user_callback_data;   // 4-8  // The draw callback code can access this.
    pub user_callback_data: Vec<u8>,
}

impl Debug for DimgDrawCmd {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl DimgDrawCmd {
    // ImDrawCmd() { memset(this, 0, sizeof(*this)); } // Also ensure our padding fields are zeroed
    //
    pub fn new() -> Self {
        Self {
            clip_rect: Default::default(),
            texture_id: DimgId::MAX,
            vtx_offset: 0,
            idx_offset: 0,
            elem_count: 0,
            user_callback: Some(defines::im_draw_callback_nop),
            user_callback_data: vec![]
        }
    }
    //     // Since 1.83: returns ImTextureID associated with this draw call. Warning: DO NOT assume this is always same as 'texture_id' (we will change this function for an upcoming feature)
    //     inline ImTextureID get_tex_id() const { return texture_id; }
    pub fn get_tex_id(&self) -> DimgTextureId {
        self.texture_id
    }
}

// [Internal] For use by ImDrawList
#[derive(Debug,Clone,Default)]
pub struct ImDrawCmdHeader
{
    // ImVec4          clip_rect;
    pub clip_rect: DimgVec4,
    // ImTextureID     texture_id;
    pub texture_id: DimgTextureId,
    // unsigned int    vtx_offset;
    pub vtx_offset: u32,
}
