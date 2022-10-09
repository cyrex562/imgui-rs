#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiConfigFlags;       // -> enum ImGuiConfigFlags_     // Flags: for io.ConfigFlags
pub type ImGuiConfigFlags = c_int;

// Configuration flags stored in io.ConfigFlags. Set by user/application.
// enum ImGuiConfigFlags_
// {
pub const ImGuiConfigFlags_None: ImGuiConfigFlags = 0;
pub const ImGuiConfigFlags_NavEnableKeyboard: ImGuiConfigFlags = 1 << 0;
// Master keyboard navigation enable flag.
pub const ImGuiConfigFlags_NavEnableGamepad: ImGuiConfigFlags = 1 << 1;
// Master gamepad navigation enable flag. Backend also needs to set ImGuiBackendFlags_HasGamepad.
pub const ImGuiConfigFlags_NavEnableSetMousePos: ImGuiConfigFlags = 1 << 2;
// Instruct navigation to move the mouse cursor. May be useful on TV/console systems where moving a virtual mouse is awkward. Will update io.MousePos and set io.WantSetMousePos=true. If enabled you MUST honor io.WantSetMousePos requests in your backend; otherwise ImGui will react as if the mouse is jumping around back and forth.
pub const ImGuiConfigFlags_NavNoCaptureKeyboard: ImGuiConfigFlags = 1 << 3;
// Instruct navigation to not set the io.WantCaptureKeyboard flag when io.NavActive is set.
pub const ImGuiConfigFlags_NoMouse: ImGuiConfigFlags = 1 << 4;
// Instruct imgui to clear mouse position/buttons in NewFrame(). This allows ignoring the mouse information set by the backend.
pub const ImGuiConfigFlags_NoMouseCursorChange: ImGuiConfigFlags = 1 << 5; // Instruct backend to not alter mouse cursor shape and visibility. Use if the backend cursor changes are interfering with yours and you don't want to use SetMouseCursor() to change mouse cursor. You may want to honor requests from imgui by reading GetMouseCursor() yourself instead.

// [BETA] Docking
pub const ImGuiConfigFlags_DockingEnable: ImGuiConfigFlags = 1 << 6; // Docking enable flags.

// [BETA] Viewports
// When using viewports it is recommended that your default value for ImGuiCol_WindowBg is opaque (Alpha=1.0) so transition to a viewport won't be noticeable.
pub const ImGuiConfigFlags_ViewportsEnable: ImGuiConfigFlags = 1 << 10;
// Viewport enable flags (require both ImGuiBackendFlags_PlatformHasViewports + ImGuiBackendFlags_RendererHasViewports set by the respective backends)
pub const ImGuiConfigFlags_DpiEnableScaleViewports: ImGuiConfigFlags = 1 << 14;
// [BETA: Don't use] FIXME-DPI: Reposition and resize imgui windows when the DpiScale of a viewport changed (mostly useful for the main viewport hosting other window). Note that resizing the main window itself is up to your application.
pub const ImGuiConfigFlags_DpiEnableScaleFonts: ImGuiConfigFlags = 1 << 15; // [BETA: Don't use] FIXME-DPI: Request bitmap-scaled fonts to match DpiScale. This is a very low-quality workaround. The correct way to handle DPI is _currently_ to replace the atlas and/or fonts in the Platform_OnChangedViewport callback; but this is all early work in progress.

// User storage (to allow your backend/engine to communicate to code that may be shared between multiple projects. Those flags are NOT used by core Dear ImGui)
pub const ImGuiConfigFlags_IsSRGB: ImGuiConfigFlags = 1 << 20;
// Application is SRGB-aware.
pub const ImGuiConfigFlags_IsTouchScreen: ImGuiConfigFlags = 1 << 21; // Application is using a touch screen instead of a mouse.
                                                                      // };
