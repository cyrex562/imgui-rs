#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiFocusedFlags;      // -> enum ImGuiFocusedFlags_    // Flags: for IsWindowFocused()
pub type ImGuiFocusedFlags = c_int;

// Flags for IsWindowFocused()
// enum ImGuiFocusedFlags_
// {
pub const ImGuiFocusedFlags_None: ImGuiFocusedFlags = 0;
pub const ImGuiFocusedFlags_ChildWindows: ImGuiFocusedFlags = 1 << 0;
// Return true if any children of the window is focused
pub const ImGuiFocusedFlags_RootWindow: ImGuiFocusedFlags = 1 << 1;
// Test from root window (top most parent of the current hierarchy)
pub const ImGuiFocusedFlags_AnyWindow: ImGuiFocusedFlags = 1 << 2;
// Return true if any window is focused. Important: If you are trying to tell how to dispatch your low-level inputs, do NOT use this. Use 'io.WantCaptureMouse' instead! Please read the FAQ!
pub const ImGuiFocusedFlags_NoPopupHierarchy: ImGuiFocusedFlags = 1 << 3;
// Do not consider popup hierarchy (do not treat popup emitter as parent of popup) (when used with _ChildWindows or _RootWindow)
pub const ImGuiFocusedFlags_DockHierarchy: ImGuiFocusedFlags = 1 << 4;
// Consider docking hierarchy (treat dockspace host as parent of docked window) (when used with _ChildWindows or _RootWindow)
pub const ImGuiFocusedFlags_RootAndChildWindows: ImGuiFocusedFlags = ImGuiFocusedFlags_RootWindow | ImGuiFocusedFlags_ChildWindows;
// };