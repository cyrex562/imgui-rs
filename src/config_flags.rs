#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiConfigFlags;       // -> enum ImGuiConfigFlags_     // Flags: for io.ConfigFlags
pub type ImguiConfigFlags = c_int;

// Configuration flags stored in io.ConfigFlags. Set by user/application.
// enum ImGuiConfigFlags_
// {
pub const ImGuiConfigFlags_None: ImguiConfigFlags = 0;
pub const ImGuiConfigFlags_NavEnableKeyboard: ImguiConfigFlags = 1 << 0;
// Master keyboard navigation enable flag.
pub const ImGuiConfigFlags_NavEnableGamepad: ImguiConfigFlags = 1 << 1;
// Master gamepad navigation enable flag. Backend also needs to set ImGuiBackendFlags_HasGamepad.
pub const ImGuiConfigFlags_NavEnableSetMousePos: ImguiConfigFlags = 1 << 2;
// Instruct navigation to move the mouse cursor. May be useful on TV/console systems where moving a virtual mouse is awkward. Will update io.MousePos and set io.WantSetMousePos=true. If enabled you MUST honor io.WantSetMousePos requests in your backend; otherwise ImGui will react as if the mouse is jumping around back and forth.
pub const ImGuiConfigFlags_NavNoCaptureKeyboard: ImguiConfigFlags = 1 << 3;
// Instruct navigation to not set the io.WantCaptureKeyboard flag when io.NavActive is set.
pub const ImGuiConfigFlags_NoMouse: ImguiConfigFlags = 1 << 4;
// Instruct imgui to clear mouse position/buttons in NewFrame(). This allows ignoring the mouse information set by the backend.
pub const ImGuiConfigFlags_NoMouseCursorChange: ImguiConfigFlags = 1 << 5; // Instruct backend to not alter mouse cursor shape and visibility. Use if the backend cursor changes are interfering with yours and you don't want to use SetMouseCursor() to change mouse cursor. You may want to honor requests from imgui by reading GetMouseCursor() yourself instead.

// [BETA] Docking
pub const ImGuiConfigFlags_DockingEnable: ImguiConfigFlags = 1 << 6; // Docking enable flags.

// [BETA] Viewports
// When using viewports it is recommended that your default value for ImGuiCol_WindowBg is opaque (Alpha=1.0) so transition to a viewport won't be noticeable.
pub const ImGuiConfigFlags_ViewportsEnable: ImguiConfigFlags = 1 << 10;
// Viewport enable flags (require both ImGuiBackendFlags_PlatformHasViewports + ImGuiBackendFlags_RendererHasViewports set by the respective backends)
pub const ImGuiConfigFlags_DpiEnableScaleViewports: ImguiConfigFlags = 1 << 14;
// [BETA: Don't use] FIXME-DPI: Reposition and resize imgui windows when the DpiScale of a viewport changed (mostly useful for the main viewport hosting other window). Note that resizing the main window itself is up to your application.
pub const ImGuiConfigFlags_DpiEnableScaleFonts: ImguiConfigFlags = 1 << 15; // [BETA: Don't use] FIXME-DPI: Request bitmap-scaled fonts to match DpiScale. This is a very low-quality workaround. The correct way to handle DPI is _currently_ to replace the atlas and/or fonts in the Platform_OnChangedViewport callback; but this is all early work in progress.

// User storage (to allow your backend/engine to communicate to code that may be shared between multiple projects. Those flags are NOT used by core Dear ImGui)
pub const ImGuiConfigFlags_IsSRGB: ImguiConfigFlags = 1 << 20;
// Application is SRGB-aware.
pub const ImGuiConfigFlags_IsTouchScreen: ImguiConfigFlags = 1 << 21; // Application is using a touch screen instead of a mouse.
                                                                      // };
