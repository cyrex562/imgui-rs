
pub type ImGuiBackendFlags = i32;

// Backend capabilities flags stored in io.BackendFlags. Set by imgui_impl_xxx or custom backend.
// enum ImGuiBackendFlags_
// {
    pub const IM_GUI_BACKEND_FLAGS_NONE: ImGuiBackendFlags =  0;
    pub const IM_GUI_BACKEND_FLAGS_HAS_GAMEPAD: ImGuiBackendFlags =  1 << 0;   // Backend Platform supports gamepad and currently has one connected.
    pub const IM_GUI_BACKEND_FLAGS_HAS_MOUSE_CURSORS: ImGuiBackendFlags =  1 << 1;   // Backend Platform supports honoring GetMouseCursor() value to change the OS cursor shape.
    pub const IM_GUI_BACKEND_FLAGS_HAS_SET_MOUSE_POS: ImGuiBackendFlags =  1 << 2;   // Backend Platform supports io.WantSetMousePos requests to reposition the OS mouse position (only used if ImGuiConfigFlags_NavEnableSetMousePos is set).
    pub const IM_GUI_BACKEND_FLAGS_RENDERER_HAS_VTX_OFFSET: ImGuiBackendFlags =  1 << 3;   // Backend Renderer supports ImDrawCmd::VtxOffset. This enables output of large meshes (64K+ vertices) while still using 16-bit indices.

    // [BETA] Viewports
    pub const IM_GUI_BACKEND_FLAGS_PLATFORM_HAS_VIEWPORTS: ImGuiBackendFlags =  1 << 10;  // Backend Platform supports multiple viewports.
    pub const IM_GUI_BACKEND_FLAGS_HAS_MOUSE_HOVERED_VIEWPORT: ImGuiBackendFlags = 1 << 11;  // Backend Platform supports calling io.AddMouseViewportEvent() with the viewport under the mouse. IF POSSIBLE; ignore viewports with the ImguiViewportFlags_NoInputs flag (Win32 backend; GLFW 3.30+ backend can do this; SDL backend cannot). If this cannot be done; Dear ImGui needs to use a flawed heuristic to find the viewport under.
    pub const IM_GUI_BACKEND_FLAGS_RENDERER_HAS_VIEWPORTS: ImGuiBackendFlags =  1 << 12;  // Backend Renderer supports multiple viewports.
// };
