#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiMouseCursor;       // -> enum ImGuiMouseCursor_     // Enum: A mouse cursor identifier
pub type ImGuiMouseCursor = c_int;

// Enumeration for GetMouseCursor()
// User code may request backend to display given cursor by calling SetMouseCursor(), which is why we have some cursors that are marked unused here
// enum ImGuiMouseCursor_
// {
pub const ImGuiMouseCursor_None: ImGuiMouseCursor = -1;
pub const ImGuiMouseCursor_Arrow: ImGuiMouseCursor = 0;
pub const ImGuiMouseCursor_TextInput: ImGuiMouseCursor = 1;
// When hovering over InputText, etc.
pub const ImGuiMouseCursor_ResizeAll: ImGuiMouseCursor = 2;
// (Unused by Dear ImGui functions)
pub const ImGuiMouseCursor_ResizeNS: ImGuiMouseCursor = 3;
// When hovering over an horizontal border
pub const ImGuiMouseCursor_ResizeEW: ImGuiMouseCursor = 4;
// When hovering over a vertical border or a column
pub const ImGuiMouseCursor_ResizeNESW: ImGuiMouseCursor = 5;
// When hovering over the bottom-left corner of a window
pub const ImGuiMouseCursor_ResizeNWSE: ImGuiMouseCursor = 6;
// When hovering over the bottom-right corner of a window
pub const ImGuiMouseCursor_Hand: ImGuiMouseCursor = 7;
// (Unused by Dear ImGui functions. Use for e.g. hyperlinks)
pub const ImGuiMouseCursor_NotAllowed: ImGuiMouseCursor = 8;
// When hovering something with disallowed interaction. Usually a crossed circle.
pub const ImGuiMouseCursor_COUNT: ImGuiMouseCursor = 10;
// };
