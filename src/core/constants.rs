use crate::core::type_defs::ImguiHandle;
use libc::size_t;

// When using CTRL+TAB (or Gamepad Square+L/R) we delay the visual a little in order to reduce visual noise doing a fast switch.
// static const float NAV_WINDOWING_HIGHLIGHT_DELAY            = 0.20;    // Time before the highlight and screen dimming starts fading in
pub const NAV_WINDOWING_HIGHLIGHT_DELAY: f32 = 0.20;
// static const float NAV_WINDOWING_LIST_APPEAR_DELAY          = 0.15f32;    // Time before the window list starts to appear
pub const NAV_WINDOWING_LIST_APPEAR_DELAY: f32 = 0.15;
// Window resizing from edges (when io.ConfigWindowsResizeFromEdges = true and IM_GUI_BACKEND_FLAGS_HAS_MOUSE_CURSORS is set in io.BackendFlags by backend)
// static const float WINDOWS_HOVER_PADDING                    = 4.0;     // Extend outside window for hovering/resizing (maxxed with TouchPadding) and inside windows for borders. Affect FindHoveredWindow().
pub const WINDOWS_HOVER_PADDING: f32 = 4.0;
// static const float WINDOWS_RESIZE_FROM_EDGES_FEEDBACK_TIMER = 0.04f;    // Reduce visual noise by only highlighting the border after a certain time.
pub const WINDOWS_RESIZE_FROM_EDGES_FEEDBACK_TIMER: f32 = 0.04;
// static const float WINDOWS_MOUSE_WHEEL_SCROLL_LOCK_TIMER    = 2.0;    // Lock scrolled window (so it doesn't pick child windows that are scrolling through) for a certain time, unless mouse moved.
pub const WINDOWS_MOUSE_WHEEL_SCROLL_LOCK_TIMER: f32 = 2.00;
// Docking
// static const float DOCKING_TRANSPARENT_PAYLOAD_ALPHA        = 0.50f32;    // For use with io.ConfigDockingTransparentPayload. Apply to Viewport _or_ WindowBg in host viewport.
pub const DOCKING_TRANSPORT_PAYLOAD_ALPHA: f32 = 0.50;
// static const float DOCKING_SPLITTER_SIZE                    = 2.0;
pub const DOCKING_SPLITTER_SIZE: f32 = 2.0;

// const ImguiHandle           IMGUI_VIEWPORT_DEFAULT_ID = 0x11111111; // Using an arbitrary constant instead of e.g. ImHashStr("ViewportDefault", 0); so it's easier to spot in the debugger. The exact value doesn't matter.
pub const IMGUI_VIEWPORT_DEFUALT_ID: ImguiHandle = 0x11111111;

pub const IM_DRAWLIST_ARCFAST_TABLE_SIZE: size_t = 48;
pub const IM_DRAWLIST_ARCFAST_SAMPLE_MAX: size_t = IM_DRAWLIST_ARCFAST_TABLE_SIZE;

// #define IM_DRAWLIST_CIRCLE_AUTO_SEGMENT_MIN                     4
pub const IM_DRAWLIST_CIRCLE_AUTO_SEGMENT_MIN: size_t = 4;
// #define IM_DRAWLIST_CIRCLE_AUTO_SEGMENT_MAX                     512
pub const IM_DRAWLIST_CIRCLE_AUTO_SEGMENT_MAX: size_t = 512;
