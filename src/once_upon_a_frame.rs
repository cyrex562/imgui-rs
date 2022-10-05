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

use libc::c_int;

// Helper: Execute a block of code at maximum once a frame. Convenient if you want to quickly create an UI within deep-nested code that runs multiple times every frame.
// Usage: static ImGuiOnceUponAFrame oaf; if (oa0f32) Text("This will be called only once per frame");
#[derive(Default,Debug,Copy, Clone)]
pub struct ImGuiOnceUponAFrame
{
    pub RefFrame: c_int,
}

impl ImGuiOnceUponAFrame {
        // ImGuiOnceUponAFrame() { RefFrame = -1; }
    // mutable let mut RefFrame: c_int = 0;
    // operator bool(&self) const {
    //     let current_frame: c_int = GetFrameCount();
    //     if (RefFrame == current_frame) {return false; }
    // RefFrame = current_frame;
    // return true; }
}
