#![allow(non_upper_case_globals)]

use libc::c_int;

pub type ImGuiBackendFlags = c_int;

// Backend capabilities flags stored in io.BackendFlags. Set by imgui_impl_xxx or custom backend.
// enum ImGuiBackendFlags_
// {
    pub const ImGuiBackendFlags_None: ImGuiBackendFlags =  0;
    pub const ImGuiBackendFlags_HasGamepad: ImGuiBackendFlags =  1 << 0;   // Backend Platform supports gamepad and currently has one connected.
    pub const ImGuiBackendFlags_HasMouseCursors: ImGuiBackendFlags =  1 << 1;   // Backend Platform supports honoring GetMouseCursor() value to change the OS cursor shape.
    pub const ImGuiBackendFlags_HasSetMousePos: ImGuiBackendFlags =  1 << 2;   // Backend Platform supports io.WantSetMousePos requests to reposition the OS mouse position (only used if ImGuiConfigFlags_NavEnableSetMousePos is set).
    pub const ImGuiBackendFlags_RendererHasVtxOffset: ImGuiBackendFlags =  1 << 3;   // Backend Renderer supports ImDrawCmd::VtxOffset. This enables output of large meshes (64K+ vertices) while still using 16-bit indices.

    // [BETA] Viewports
    pub const ImGuiBackendFlags_PlatformHasViewports: ImGuiBackendFlags =  1 << 10;  // Backend Platform supports multiple viewports.
    pub const ImGuiBackendFlags_HasMouseHoveredViewport: ImGuiBackendFlags = 1 << 11;  // Backend Platform supports calling io.AddMouseViewportEvent() with the viewport under the mouse. IF POSSIBLE; ignore viewports with the ImGuiViewportFlags_NoInputs flag (Win32 backend; GLFW 3.30+ backend can do this; SDL backend cannot). If this cannot be done; Dear ImGui needs to use a flawed heuristic to find the viewport under.
    pub const ImGuiBackendFlags_RendererHasViewports: ImGuiBackendFlags =  1 << 12;  // Backend Renderer supports multiple viewports.
// };
