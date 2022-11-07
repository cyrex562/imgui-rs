#![allow(non_snake_case)]

use crate::io::ImguiIo;

// ImGuiIO& GetIO()
pub fn GetIO() -> &mut ImguiIo {
    // IM_ASSERT(GImGui != NULL && "No current context. Did you call CreateContext() and SetCurrentContext() ?");
    return &mut GimGui.IO;
}
