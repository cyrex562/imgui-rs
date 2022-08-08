//-----------------------------------------------------------------------------
// COMPILE-TIME OPTIONS FOR DEAR IMGUI
// Runtime options (clipboard callbacks, enabling various features, etc.) can generally be set via the ImGuiIO structure.
// You can use ImGui::SetAllocatorFunctions() before calling ImGui::CreateContext() to rewire memory allocation functions.
//-----------------------------------------------------------------------------
// A) You may edit imconfig.h (and not overwrite it when updating Dear ImGui, or maintain a patch/rebased branch with your modifications to it)
// B) or '#define IMGUI_USER_CONFIG "my_imgui_config.h"' in your project and then add directives in your own file without touching this template.
//-----------------------------------------------------------------------------
// You need to make sure that configuration settings are defined consistently _everywhere_ Dear ImGui is used, which include the imgui*.cpp
// files but also _any_ of your code that uses Dear ImGui. This is because some compile-time options have an affect on data structures.
// Defining those options in imconfig.h will ensure every compilation unit gets to see the same data structure layouts.
// Call IMGUI_CHECKVERSION() from your .cpp files to verify that the data structures your files are using are matching the ones imgui.cpp is using.
//-----------------------------------------------------------------------------

//---- Override ImDrawCallback signature (will need to modify renderer backends accordingly)
//struct ImDrawList;
//struct ImDrawCmd;
//typedef void (*MyImDrawCallback)(const ImDrawList* draw_list, const ImDrawCmd* cmd, void* my_renderer_user_data);
//#define ImDrawCallback MyImDrawCallback


/// Configuration flags stored in io.config_flags. Set by user/application.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ConfigFlags {
    None,
    NavEnableKeyboard,
    // Master keyboard navigation enable flag. NewFrame() will automatically fill io.nav_inputs[] based on io.add_key_event() calls
    NavEnableGamepad,
    // Master gamepad navigation enable flag. This is mostly to instruct your imgui backend to fill io.nav_inputs[]. Backend also needs to set BackendFlags::HasGamepad.
    NavEnableSetMousePos,
    // Instruct navigation to move the mouse cursor. May be useful on TV/console systems where moving a virtual mouse is awkward. Will update io.mouse_pos and set io.want_set_mouse_pos=true. If enabled you MUST honor io.want_set_mouse_pos requests in your backend, otherwise ImGui will react as if the mouse is jumping around back and forth.
    NavNoCaptureKeyboard,
    // Instruct navigation to not set the io.want_capture_keyboard flag when io.nav_active is set.
    NoMouse,
    // Instruct imgui to clear mouse position/buttons in NewFrame(). This allows ignoring the mouse information set by the backend.
    NoMouseCursorChange,   // Instruct backend to not alter mouse cursor shape and visibility. Use if the backend cursor changes are interfering with yours and you don't want to use SetMouseCursor() to change mouse cursor. You may want to honor requests from imgui by reading GetMouseCursor() yourself instead.

    // [BETA] Docking
    DockingEnable,   // Docking enable flags.

    // [BETA] viewports
    // When using viewports it is recommended that your default value for ImGuiCol_WindowBg is opaque (Alpha=1.0) so transition to a viewport won't be noticeable.
    ViewportsEnable,
    // viewport enable flags (require both ImGuiBackendFlags_PlatformHasViewports + ImGuiBackendFlags_RendererHasViewports set by the respective backends)
    DpiEnableScaleViewports,
    // [BETA: Don't use] FIXME-DPI: Reposition and resize imgui windows when the dpi_scale of a viewport changed (mostly useful for the main viewport hosting other window). Note that resizing the main window itself is up to your application.
    DpiEnableScaleFonts,  // [BETA: Don't use] FIXME-DPI: Request bitmap-scaled fonts to match dpi_scale. This is a very low-quality workaround. The correct way to handle DPI is _currently_ to replace the atlas and/or fonts in the platform_on_changed_viewport callback, but this is all early work in progress.

    // User storage (to allow your backend/engine to communicate to code that may be shared between multiple projects. Those flags are NOT used by core Dear ImGui)
    IsSRGB,
    // Application is SRGB-aware.
    IsTouchScreen,   // Application is using a touch screen instead of a mouse.
}

impl Default for ConfigFlags {
    fn default() -> Self {
        Self::None
    }
}

/// Backend capabilities flags stored in io.backend_flags. Set by imgui_impl_xxx or custom backend.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum BackendFlags {
    None,
    HasGamepad,
    // Backend Platform supports gamepad and currently has one connected.
    HasMouseCursors,
    // Backend Platform supports honoring GetMouseCursor() value to change the OS cursor shape.
    HasSetMousePos,
    // Backend Platform supports io.want_set_mouse_pos requests to reposition the OS mouse position (only used if ImGuiConfigFlags_NavEnableSetMousePos is set).
    RendererHasvtx_offset,   // Backend Renderer supports ImDrawCmd::vtx_offset. This enables output of large meshes (64K+ vertices) while still using 16-bit indices.

    // [BETA] viewports
    PlatformHasViewports,
    // Backend Platform supports multiple viewports.
    HasMouseHoveredViewport,
    // Backend Platform supports calling io.add_mouse_viewport_event() with the viewport under the mouse. IF POSSIBLE, ignore viewports with the NoInputs flag (Win32 backend, GLFW 3.30+ backend can do this, SDL backend cannot). If this cannot be done, Dear ImGui needs to use a flawed heuristic to find the viewport under.
    RendererHasViewports,   // Backend Renderer supports multiple viewports.
}

/// Save additional comments in .ini file (particularly helps for Docking, but makes saving slower)
pub const IMGUI_DEBUG_INI_SETINGS: bool = false;
