#![allow(non_snake_case)]

use crate::drawing::draw::ImDrawCallback;
use crate::core::type_defs::ImTextureID;
use crate::core::vec4::ImVec4;
use libc::{c_uint, c_void, size_t};

// Typically, 1 command = 1 GPU draw call (unless command is a callback)
// - VtxOffset: When 'io.BackendFlags & IM_GUI_BACKEND_FLAGS_RENDERER_HAS_VTX_OFFSET' is enabled,
//   this fields allow us to render meshes larger than 64K vertices while keeping 16-bit indices.
//   Backends made for <1.71. will typically ignore the VtxOffset fields.
// - The ClipRect/TextureId/VtxOffset fields must be contiguous as we memcmp() them together (this is asserted for).
#[derive(Default, Debug, Clone, Copy)]
pub struct ImDrawCmd {
    pub ClipRect: ImVec4, // 4*4  // Clipping rectangle (x1, y1, x2, y2). Subtract ImDrawData->DisplayPos to get clipping rectangle in "viewport" coordinates
    pub TextureId: ImTextureID, // 4-8  // User-provided texture ID. Set by user in ImfontAtlas::SetTexID() for fonts or passed to Image*() functions. Ignore if never using images or multiple fonts atlas.
    pub VtxOffset: size_t, // 4    // Start offset in vertex buffer. IM_GUI_BACKEND_FLAGS_RENDERER_HAS_VTX_OFFSET: always 0, otherwise may be >0 to support meshes larger than 64K vertices with 16-bit indices.
    pub IdxOffset: size_t, // 4    // Start offset in index buffer.
    pub ElemCount: size_t, // 4    // Number of indices (multiple of 3) to be rendered as triangles. Vertices are stored in the callee ImDrawList's vtx_buffer[] array, indices in idx_buffer[].
    pub UserCallback: ImDrawCallback, // 4-8  // If != NULL, call the function instead of rendering the vertices. clip_rect and texture_id will be set normally.
    pub UserCallbackData: *mut c_void, // 4-8  // The draw callback code can access this.
}

impl ImDrawCmd {
    // ImDrawCmd() { memset(this, 0, sizeof(*this)); } // Also ensure our padding fields are zeroed

    // Since 1.83: returns associated: ImTextureID with this draw call. Warning: DO NOT assume this is always same as 'TextureId' (we will change this function for an upcoming feature)
    // inline GetTexID: ImTextureID() const { return TextureId; }
    pub fn GetTexID(&self) -> ImTextureID {
        self.TextureId
    }
}
