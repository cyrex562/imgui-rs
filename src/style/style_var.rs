#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiStyleVar;          // -> enum ImGuiStyleVar_        // Enum: A variable identifier for styling
pub type ImGuiStyleVar = c_int;

// Enumeration for PushStyleVar() / PopStyleVar() to temporarily modify the ImGuiStyle structure.
// - The enum only refers to fields of ImGuiStyle which makes sense to be pushed/popped inside UI code.
//   During initialization or between frames, feel free to just poke into ImGuiStyle directly.
// - Tip: Use your programming IDE navigation facilities on the names in the _second column_ below to find the actual members and their description.
//   In Visual Studio IDE: CTRL+comma ("Edit.GoToAll") can follow symbols in comments, whereas CTRL+F12 ("Edit.GoToImplementation") cannot.
//   With Visual Assist installed: ALT+G ("VAssistX.GoToImplementation") can also follow symbols in comments.
// - When changing this enum, you need to update the associated internal table GSTYLE_VAR_INFO[] accordingly. This is where we link enum values to members offset/type.
// enum ImGuiStyleVar_
// {
// Enum name --------------------- // Member in ImGuiStyle structure (see ImGuiStyle for descriptions)
pub const ImGuiStyleVar_Alpha: ImGuiStyleVar = 0; // float     Alpha
pub const ImGuiStyleVar_DisabledAlpha: ImGuiStyleVar = 1; // float     DisabledAlpha
pub const ImGuiStyleVar_WindowPadding: ImGuiStyleVar = 2; // ImVec2    WindowPadding
pub const ImGuiStyleVar_WindowRounding: ImGuiStyleVar = 3; // float     WindowRounding
pub const ImGuiStyleVar_WindowBorderSize: ImGuiStyleVar = 4; // float     WindowBorderSize
pub const ImGuiStyleVar_WindowMinSize: ImGuiStyleVar = 5; // ImVec2    WindowMinSize
pub const ImGuiStyleVar_WindowTitleAlign: ImGuiStyleVar = 6; // ImVec2    WindowTitleAlign
pub const ImGuiStyleVar_ChildRounding: ImGuiStyleVar = 7; // float     ChildRounding
pub const ImGuiStyleVar_ChildBorderSize: ImGuiStyleVar = 8; // float     ChildBorderSize
pub const ImGuiStyleVar_PopupRounding: ImGuiStyleVar = 9; // float     PopupRounding
pub const ImGuiStyleVar_PopupBorderSize: ImGuiStyleVar = 10; // float     PopupBorderSize
pub const ImGuiStyleVar_FramePadding: ImGuiStyleVar = 11; // ImVec2    FramePadding
pub const ImGuiStyleVar_FrameRounding: ImGuiStyleVar = 12; // float     FrameRounding
pub const ImGuiStyleVar_FrameBorderSize: ImGuiStyleVar = 13; // float     FrameBorderSize
pub const ImGuiStyleVar_ItemSpacing: ImGuiStyleVar = 14; // ImVec2    ItemSpacing
pub const ImGuiStyleVar_ItemInnerSpacing: ImGuiStyleVar = 15; // ImVec2    ItemInnerSpacing
pub const ImGuiStyleVar_IndentSpacing: ImGuiStyleVar = 16; // float     IndentSpacing
pub const ImGuiStyleVar_CellPadding: ImGuiStyleVar = 17; // ImVec2    CellPadding
pub const ImGuiStyleVar_ScrollbarSize: ImGuiStyleVar = 18; // float     ScrollbarSize
pub const ImGuiStyleVar_ScrollbarRounding: ImGuiStyleVar = 19; // float     ScrollbarRounding
pub const ImGuiStyleVar_GrabMinSize: ImGuiStyleVar = 20; // float     GrabMinSize
pub const ImGuiStyleVar_GrabRounding: ImGuiStyleVar = 21; // float     GrabRounding
pub const ImGuiStyleVar_TabRounding: ImGuiStyleVar = 22; // float     TabRounding
pub const ImGuiStyleVar_ButtonTextAlign: ImGuiStyleVar = 23; // ImVec2    ButtonTextAlign
pub const ImGuiStyleVar_SelectableTextAlign: ImGuiStyleVar = 24; // ImVec2    SelectableTextAlign
pub const ImGuiStyleVar_COUNT: ImGuiStyleVar = 25;
