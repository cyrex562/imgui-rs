#![allow(non_upper_case_globals)]

use libc::c_int;

// List of colors that are stored at the time of Begin() into Docked Windows.
// We currently store the packed colors in a simple array window.DockStyle.Colors[].
// A better solution may involve appending into a log of colors in ImGuiContext + store offsets into those arrays in ImGuiWindow,
// but it would be more complex as we'd need to double-buffer both as e.g. drop target may refer to window from last frame.
pub type ImGuiDockStyleCol = c_int;

// enum ImGuiWindowDockStyleCol
// {
    pub const ImGuiWindowDockStyleCol_Text: ImGuiDockStyleCol = 0;
pub const ImGuiWindowDockStyleCol_Tab: ImGuiDockStyleCol = 1;
pub const ImGuiWindowDockStyleCol_TabHovered: ImGuiDockStyleCol = 2;
pub const ImGuiWindowDockStyleCol_TabActive: ImGuiDockStyleCol = 3;
pub const ImGuiWindowDockStyleCol_TabUnfocused: ImGuiDockStyleCol = 4;
pub const ImGuiWindowDockStyleCol_TabUnfocusedActive: ImGuiDockStyleCol = 5;
pub const ImGuiWindowDockStyleCol_COUNT: ImGuiDockStyleCol = 6;
// };