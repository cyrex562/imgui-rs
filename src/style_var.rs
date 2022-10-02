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
// - When changing this enum, you need to update the associated internal table GStyleVarInfo[] accordingly. This is where we link enum values to members offset/type.
// enum ImGuiStyleVar_
// {
// Enum name --------------------- // Member in ImGuiStyle structure (see ImGuiStyle for descriptions)
pub const ImGuiStyleVar_Alpha: ImGuiStylerVar = 0;
// float     Alpha
pub const ImGuiStyleVar_DisabledAlpha: ImGuiStylerVar = 1;
// float     DisabledAlpha
pub const ImGuiStyleVar_WindowPadding: ImGuiStylerVar = 2;
// ImVec2    WindowPadding
pub const ImGuiStyleVar_WindowRounding: ImGuiStylerVar = 3;
// float     WindowRounding
pub const ImGuiStyleVar_WindowBorderSize: ImGuiStylerVar = 4;
// float     WindowBorderSize
pub const ImGuiStyleVar_WindowMinSize: ImGuiStylerVar = 5;
// ImVec2    WindowMinSize
pub const ImGuiStyleVar_WindowTitleAlign: ImGuiStylerVar = 6;
// ImVec2    WindowTitleAlign
pub const ImGuiStyleVar_ChildRounding: ImGuiStylerVar = 7;
// float     ChildRounding
pub const ImGuiStyleVar_ChildBorderSize: ImGuiStylerVar = 8;
// float     ChildBorderSize
pub const ImGuiStyleVar_PopupRounding: ImGuiStylerVar = 9;
// float     PopupRounding
pub const ImGuiStyleVar_PopupBorderSize: ImGuiStylerVar = 10;
// float     PopupBorderSize
pub const ImGuiStyleVar_FramePadding: ImGuiStylerVar = 11;
// ImVec2    FramePadding
pub const ImGuiStyleVar_FrameRounding: ImGuiStylerVar = 12;
// float     FrameRounding
pub const ImGuiStyleVar_FrameBorderSize: ImGuiStylerVar = 13;
// float     FrameBorderSize
pub const ImGuiStyleVar_ItemSpacing: ImGuiStylerVar = 14;
// ImVec2    ItemSpacing
pub const ImGuiStyleVar_ItemInnerSpacing: ImGuiStylerVar = 15;
// ImVec2    ItemInnerSpaci
pub const ImGuiStyleVar_IndentSpacing: ImGuiStylerVar = 16;
// float     IndentSpacing
pub const ImGuiStyleVar_CellPadding: ImGuiStylerVar = 17;
// ImVec2    CellPadding
pub const ImGuiStyleVar_ScrollbarSize: ImGuiStylerVar = 18;
// float     ScrollbarSize
pub const ImGuiStyleVar_ScrollbarRounding: ImGuiStylerVar = 19;
// float     ScrollbarRounding
pub const ImGuiStyleVar_GrabMinSize: ImGuiStylerVar = 20;
// float     GrabMinSize
pub const ImGuiStyleVar_GrabRounding: ImGuiStylerVar = 21;
// float     GrabRounding
pub const ImGuiStyleVar_TabRounding: ImGuiStylerVar = 22;
// float     TabRounding
pub const ImGuiStyleVar_ButtonTextAlign: ImGuiStylerVar = 23;
// ImVec2    ButtonTextAlign
pub const ImGuiStyleVar_SelectableTextAlign: ImGuiStylerVar = 24;
// ImVec2    SelectableTextAlign
pub const ImGuiStyleVar_COUNT: ImGuiStylerVar = 25;
// };
