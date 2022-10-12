#![allow(non_snake_case)]

use crate::io::ImGuiIO;

// ImGuiIO& GetIO()
pub fn GetIO() -> &mut ImGuiIO {
    // IM_ASSERT(GImGui != NULL && "No current context. Did you call CreateContext() and SetCurrentContext() ?");
    return &mut GimGui.IO;
}
