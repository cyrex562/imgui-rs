#![allow(non_snake_case)]

use crate::io::IoContext;

// ImGuiIO& GetIO()
pub fn GetIO() -> &mut IoContext {
    // IM_ASSERT(GImGui != NULL && "No current context. Did you call CreateContext() and SetCurrentContext() ?");
    return &mut GimGui.IO;
}
