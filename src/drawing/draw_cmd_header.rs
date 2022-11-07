use crate::core::type_defs::ImTextureID;
use crate::core::vec4::ImVec4;
use libc::{c_uint, size_t};

// [Internal] For use by ImDrawList
#[derive(Default, Debug, Clone, Copy)]
pub struct ImDrawCmdHeader {
    pub ClipRect: ImVec4,
    pub TextureId: ImTextureID,
    pub VtxOffset: size_t,
}
