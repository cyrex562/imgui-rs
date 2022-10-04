#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use libc::{c_float, c_int};
use crate::vec4::ImVec4;

//   With Visual Assist installed: ALT+G ("VAssistX.GoToImplementation") can also follow symbols in comments.
// typedef int ImGuiCol;               // -> enum ImGuiCol_             // Enum: A color identifier for styling
pub type ImGuiCol = u32;


// Helpers macros to generate 32-bit encoded colors
// User can declare their own format by #defining the 5 _SHIFT/_MASK macros in their imconfig file.
// #ifndef IM_COL32_R_SHIFT
// #ifdef IMGUI_USE_BGRA_PACKED_COLOR
// #define IM_COL32_R_SHIFT    16
// #define IM_COL32_G_SHIFT    8
// #define IM_COL32_B_SHIFT    0
// #define IM_COL32_A_SHIFT    24
// #define IM_COL32_A_MASK     0xFF000000
// #else
// #define IM_COL32_R_SHIFT    0
pub const IM_COL32_R_SHIFT: ImGUiCol = 0;
// #define IM_COL32_G_SHIFT    8
pub const IM_COL32_G_SHIFT: ImGUiCol = 8;
// #define IM_COL32_B_SHIFT    16
pub const IM_COL32_B_SHIFT: ImGuiCol = 16;
// #define IM_COL32_A_SHIFT    24
pub const IM_COL32_A_SHIFT: IMGuiCol = 24;
// #define IM_COL32_A_MASK     0xFF000000
pub const IM_COL32_A_MASK: ImGuiCol = 0xFF000000;

// #endif
// #endif
// #define IM_COL32(R,G,B,A)    (((ImU32)(A)<<IM_COL32_A_SHIFT) | ((ImU32)(B)<<IM_COL32_B_SHIFT) | ((ImU32)(G)<<IM_COL32_G_SHIFT) | ((ImU32)(R)<<IM_COL32_R_SHIFT))
pub fn IM_COL32(r: u32, g: u32, b: u32, a: u32) -> u32 {
    a << IM_COL32_A_SHIFT | b << IM_COL32_B_SHIFT | g << IM_COL32_G_SHIFT | r << IM_COL32_R_SHIFT
}


// #define IM_COL32_DISABLE                IM_COL32(0,0,0,1)   // Special sentinel code which cannot be used as a regular color.
pub const IM_COL32_DISABLE: ImGuiCol = IM_COL32(0, 0, 0, 1);
// #define IM_COL32_WHITE       IM_COL32(255,255,255,255)  // Opaque white = 0xFFFFFFFF
pub const IM_COL32_WHITE: ImGuiCol = IM_COL32(255, 255, 255, 255);
// #define IM_COL32_BLACK       IM_COL32(0,0,0,255)        // Opaque black
pub const IM_COL32_BLACK: ImGuiCol = IM_COL32(0, 0, 0, 255);
// #define IM_COL32_BLACK_TRANS IM_COL32(0,0,0,0)          // Transparent black = 0x00000000
pub const IM_COL32_BLACK_TRANS: ImGuiCol = IM_COL32(0, 0, 0, 0);


// Enumeration for PushStyleColor() / PopStyleColor()
// enum ImGuiCol_
// {
pub const ImGuiCol_Text: ImGuiCol = 0;
pub const ImGuiCol_TextDisabled: ImGuiCol = 1;
pub const ImGuiCol_WindowBg: ImGuiCol = 2;
// Background of normal windows
pub const ImGuiCol_ChildBg: ImGuiCol = 3;
// Background of child windows
pub const ImGuiCol_PopupBg: ImGuiCol = 4;
// Background of popups; menus; tooltips windows
pub const ImGuiCol_Border: ImGuiCol = 5;
pub const ImGuiCol_BorderShadow: ImGuiCol = 6;
pub const ImGuiCol_FrameBg: ImGuiCol = 7;
// Background of checkbox; radio button; plot; slider; text input
pub const ImGuiCol_FrameBgHovered: ImGuiCol = 8;
pub const ImGuiCol_FrameBgActive: ImGuiCol = 9;
pub const ImGuiCol_TitleBg: ImGuiCol = 10;
pub const ImGuiCol_TitleBgActive: ImGuiCol = 11;
pub const ImGuiCol_TitleBgCollapsed: ImGuiCol = 12;
pub const ImGuiCol_MenuBarBg: ImGuiCol = 13;
pub const ImGuiCol_ScrollbarBg: ImGuiCol = 14;
pub const ImGuiCol_ScrollbarGrab: ImGuiCol = 15;
pub const ImGuiCol_ScrollbarGrabHovered: ImGuiCol = 16;
pub const ImGuiCol_ScrollbarGrabActive: ImGuiCol = 17;
pub const ImGuiCol_CheckMark: ImGuiCol = 18;
pub const ImGuiCol_SliderGrab: ImGuiCol = 19;
pub const ImGuiCol_SliderGrabActive: ImGuiCol = 20;
pub const ImGuiCol_Button: ImGuiCol = 21;
pub const ImGuiCol_ButtonHovered: ImGuiCol = 22;
pub const ImGuiCol_ButtonActive: ImGuiCol = 23;
pub const ImGuiCol_Header: ImGuiCol = 24;
// Header* colors are used for CollapsingHeader; TreeNode; Selectable; MenuItem
pub const ImGuiCol_HeaderHovered: ImGuiCol = 25;
pub const ImGuiCol_HeaderActive: ImGuiCol = 26;
pub const ImGuiCol_Separator: ImGuiCol = 27;
pub const ImGuiCol_SeparatorHovered: ImGuiCol = 28;
pub const ImGuiCol_SeparatorActive: ImGuiCol = 29;
pub const ImGuiCol_ResizeGrip: ImGuiCol = 30;
// Resize grip in lower-right and lower-left corners of windows.
pub const ImGuiCol_ResizeGripHovered: ImGuiCol = 31;
pub const ImGuiCol_ResizeGripActive: ImGuiCol = 32;
pub const ImGuiCol_Tab: ImGuiCol = 33;
// TabItem in a TabBar
pub const ImGuiCol_TabHovered: ImGuiCol = 34;
pub const ImGuiCol_TabActive: ImGuiCol = 35;
pub const ImGuiCol_TabUnfocused: ImGuiCol = 36;
pub const ImGuiCol_TabUnfocusedActive: ImGuiCol = 37;
pub const ImGuiCol_DockingPreview: ImGuiCol = 38;
// Preview overlay color when about to docking something
pub const ImGuiCol_DockingEmptyBg: ImGuiCol = 39;
// Background color for empty node (e.g. CentralNode with no window docked into it)
pub const ImGuiCol_PlotLines: ImGuiCol = 40;
pub const ImGuiCol_PlotLinesHovered: ImGuiCol = 41;
pub const ImGuiCol_PlotHistogram: ImGuiCol = 42;
pub const ImGuiCol_PlotHistogramHovered: ImGuiCol = 43;
pub const ImGuiCol_TableHeaderBg: ImGuiCol = 44;
// Table header background
pub const ImGuiCol_TableBorderStrong: ImGuiCol = 45;
// Table outer and header borders (prefer using Alpha=1.0 here)
pub const ImGuiCol_TableBorderLight: ImGuiCol = 46;
// Table inner borders (prefer using Alpha=1.0 here)
pub const ImGuiCol_TableRowBg: ImGuiCol = 47;
// Table row background (even rows)
pub const ImGuiCol_TableRowBgAlt: ImGuiCol = 48;
// Table row background (odd rows)
pub const ImGuiCol_TextSelectedBg: ImGuiCol = 49;
pub const ImGuiCol_DragDropTarget: ImGuiCol = 50;
// Rectangle highlighting a drop target
pub const ImGuiCol_NavHighlight: ImGuiCol = 51;
// Gamepad/keyboard: current highlighted item
pub const ImGuiCol_NavWindowingHighlight: ImGuiCol = 52;
// Highlight window when using CTRL+TAB
pub const ImGuiCol_NavWindowingDimBg: ImGuiCol = 53;
// Darken/colorize entire screen behind the CTRL+TAB window list; when active
pub const ImGuiCol_ModalWindowDimBg: ImGuiCol = 54;
// Darken/colorize entire screen behind a modal window, when one is active
pub const ImGuiCol_COUNT: ImGuiCol = 55;
// };


// Helper: ImColor() implicitly converts colors to either ImU32 (packed 4x1 byte) or ImVec4 (4x1 float)
// Prefer using IM_COL32() macros if you want a guaranteed compile-time ImU32 for usage with ImDrawList API.
// **Avoid storing ImColor! Store either u32 of ImVec4. This is not a full-featured color class. MAY OBSOLETE.
// **None of the ImGui API are using ImColor directly but you can use it as a convenience to pass colors in either ImU32 or ImVec4 formats. Explicitly cast to ImU32 or ImVec4 if needed.
#[derive(Default, Debug, Copy, Clone)]
pub struct ImColor {
    pub Value: ImVec4,

}

impl ImColor {
    // constexpr ImColor()                                             { }
    // constexpr ImColor(r: c_float, g: c_float, b: c_float, let a: c_float =  1f32)    : Value(r, g, b, a) { }
    pub fn new(r: c_float, g: c_float, b: c_float, a: c_float) -> Self {
        Self {
            Value: ImVec4::new4(r, g, b, a)
        }
    }


    // constexpr ImColor(const ImVec4& col)                            : Value(col) {}
    pub fn new2(col: &ImVec4) -> Self {
        Self {
            Value: col.clone(),
        }
    }


    // ImColor(r: c_int, g: c_int, b: c_int, let a: c_int = 255)
    pub fn new3(r: c_int, g: c_int, b: c_int, a: c_int) -> Self {
        let sc: c_float = 1f32 / 255f32;
        let mut out = Self {
            Value: ImVec4::default(),
        };
        out.Value.x = r * sc;
        out.Value.y = g * sc;
        out.Value.z = b * sc;
        out.Value.w = a * sc;
        out
    }


    // ImColor(u32 rgba)                                             
    pub fn new4(rgba: u32) -> Self {
        let sc: c_float = 1f32 / 255f32;
        let mut out = Self {
            Value: ImVec4::default(),
        };
        out.Value.x = ((rgba >> IM_COL32_R_SHIFT) & 0xF0f32) * sc;
        out.Value.y = ((rgba >> IM_COL32_G_SHIFT) & 0xF0f32) * sc;
        out.Value.z = ((rgba >> IM_COL32_B_SHIFT) & 0xF0f32) * sc;
        out.Value.w = ((rgba >> IM_COL32_A_SHIFT) & 0xF0f32) * sc;
        out
    }


    // inline operator u32() const                                   { return ColorConvertFloat4ToU32(Value); }


    // inline operator ImVec4() const                                  { return Value; }

    // FIXME-OBSOLETE: May need to obsolete/cleanup those helpers.
    // inline c_void    SetHSV(h: c_float, s: c_float, v: c_float, let a: c_float =  1f32){ ColorConvertHSVtoRGB(h, s, v, Value.x, Value.y, Value.z); Value.w = a; }


    // static ImColor HSV(h: c_float, s: c_float, v: c_float, let a: c_float =  1f32)   { r: c_float, g, b; ColorConvertHSVtoRGB(h, s, v, r, g, b); return ImColor(r, g, b, a); }
}
