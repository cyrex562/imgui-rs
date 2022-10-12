#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiDebugLogFlags;         // -> enum ImGuiDebugLogFlags_      // Flags: for ShowDebugLogWindow(), g.DebugLogFlags
pub type ImGuiDebugLogFlags = c_int;


// enum ImGuiDebugLogFlags_
// {
// Event types
pub const ImGuiDebugLogFlags_None: ImGuiDebugLogFlags = 0;
pub const ImGuiDebugLogFlags_EventActiveId: ImGuiDebugLogFlags = 1 << 0;
pub const ImGuiDebugLogFlags_EventFocus: ImGuiDebugLogFlags = 1 << 1;
pub const ImGuiDebugLogFlags_EventPopup: ImGuiDebugLogFlags = 1 << 2;
pub const ImGuiDebugLogFlags_EventNav: ImGuiDebugLogFlags = 1 << 3;
pub const ImGuiDebugLogFlags_EventClipper: ImGuiDebugLogFlags = 1 << 4;
pub const ImGuiDebugLogFlags_EventIO: ImGuiDebugLogFlags = 1 << 5;
pub const ImGuiDebugLogFlags_EventDocking: ImGuiDebugLogFlags = 1 << 6;
pub const ImGuiDebugLogFlags_EventViewport: ImGuiDebugLogFlags = 1 << 7;
pub const ImGuiDebugLogFlags_EventMask_: ImGuiDebugLogFlags = ImGuiDebugLogFlags_EventActiveId | ImGuiDebugLogFlags_EventFocus | ImGuiDebugLogFlags_EventPopup | ImGuiDebugLogFlags_EventNav | ImGuiDebugLogFlags_EventClipper | ImGuiDebugLogFlags_EventIO | ImGuiDebugLogFlags_EventDocking | ImGuiDebugLogFlags_EventViewport;
pub const ImGuiDebugLogFlags_OutputToTTY: ImGuiDebugLogFlags = 1 << 10;  // Also send output to TTY
// };
