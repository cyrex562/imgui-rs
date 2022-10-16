//-----------------------------------------------------------------------------
// [SECTION] Helpers (ImGuiOnceUponAFrame, ImGuiTextFilter, ImGuiTextBuffer, ImGuiStorage, ImGuiListClipper, ImColor)
//-----------------------------------------------------------------------------

// Helper: Unicode defines
// #define IM_UNICODE_CODEPOINT_INVALID 0xFFFD     // Invalid Unicode code point (standard value).
// #ifdef IMGUI_USE_WCHAR32
// #define IM_UNICODE_CODEPOINT_MAX     0x10FFFF   // Maximum Unicode code point supported by this build.
// #else
// #define IM_UNICODE_CODEPOINT_MAX     0xFFFF     // Maximum Unicode code point supported by this build.
// #endif

// Helper: Execute a block of code at maximum once a frame. Convenient if you want to quickly create an UI within deep-nested code that runs multiple times every frame.
// Usage: static ImGuiOnceUponAFrame oaf; if (oa0f32) Text("This will be called only once per frame");
#[derive(Default, Debug, Clone, Copy, PartialOrd, PartialEq)]
pub struct ImGuiOnceUponAFrame {
    pub RefFrame: i32,
}

impl ImGuiOnceUponAFrame {
    pub fn new() -> Self {
        let mut out = Self::default();
        out.RefFrame = -1;
        out
    }
}
