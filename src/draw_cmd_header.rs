use libc::c_uint;
use crate::type_defs::ImTextureID;
use crate::vec4::ImVec4;

// [Internal] For use by ImDrawList
#[derive(Default, Debug, Clone)]
pub struct ImDrawCmdHeader {
    pub ClipRect: ImVec4,
    pub TextureId: ImTextureID,
    pub VtxOffset: c_uint,
}
