use std::collections::HashSet;
use crate::imgui_h::ImGuiColor;
use crate::imgui_math::{IM_F32_TO_INT8_SAT, ImFabs, ImFmod, ImLerpU32, ImSwapF32};
use crate::imgui_vec::ImVec4;

//  ImU32 ImAlphaBlendColors(ImU32 col_a, ImU32 col_b)
pub fn ImAlphaBlendColors(col_a: u32, col_b: u32) -> u32
{
    // float t = ((col_b >> IM_COL32_A_SHIFT) & 0xFF) / 255.f;
    let t = ((col_b >> IM_COL32_A_SHIFT) & 0xff) as f32 / 255.0;
    // int r = ImLerp((col_a >> IM_COL32_R_SHIFT) & 0xFF, (col_b >> IM_COL32_R_SHIFT) & 0xFF, t);
    let r = ImLerpU32((col_a >> IM_COL32_R_SHIFT) & 0xff, (col_b >> IM_COL32_R_SHIFT) & 0xff, t);
    // int g = ImLerp((col_a >> IM_COL32_G_SHIFT) & 0xFF, (col_b >> IM_COL32_G_SHIFT) & 0xFF, t);
    let g = ImLerpU32((col_a >> IM_COL32_G_SHIFT) & 0xff, (col_b >> IM_COL32_G_SHIFT) & 0xff, t);
    // int b = ImLerp((col_a >> IM_COL32_B_SHIFT) & 0xFF, (col_b >> IM_COL32_B_SHIFT) & 0xFF, t);
    let b = ImLerpU32((col_a >> IM_COL32_B_SHIFT) & 0xff, (col_b >> IM_COL32_B_SHIFT) & 0xff, t);
    IM_COL32(r, g, b, 0xFF)
}

// ImVec4 ImGui::ColorConvertU32ToFloat4(ImU32 in)
pub fn ColorCovertU32ToFloat(in_u32: u32) -> ImVec4
{
    // float s = 1.0 / 255.0;
    let mut s: f32 = 1.0 / 255.0;
    return ImVec4{
        x:((in_u32 >> IM_COL32_R_SHIFT) & 0xFF) as f32 * s,
        y:((in_u32 >> IM_COL32_G_SHIFT) & 0xFF) as f32 * s,
        z:((in_u32 >> IM_COL32_B_SHIFT) & 0xFF) as f32 * s,
        w:((in_u32 >> IM_COL32_A_SHIFT) & 0xFF) as f32 * s};
}

// ImU32 ImGui::ColorConvertFloat4ToU32(const ImVec4& in)
pub fn ColorConvertFloat4ToU32(in_vec: &ImVec4) -> u32
{
    // ImU32 out;

    let mut out: u32  = ((IM_F32_TO_INT8_SAT(in_vec.x)) << IM_COL32_R_SHIFT )as u32;
    out |= ((IM_F32_TO_INT8_SAT(in_vec.y)) << IM_COL32_G_SHIFT) as u32;
    out |= ((IM_F32_TO_INT8_SAT(in_vec.z)) << IM_COL32_B_SHIFT) as u32;
    out |= ((IM_F32_TO_INT8_SAT(in_vec.w)) << IM_COL32_A_SHIFT) as u32;
    out
}

// Convert rgb floats ([0-1],[0-1],[0-1]) to hsv floats ([0-1],[0-1],[0-1]), from Foley & van Dam p592
// Optimized http://lolengine.net/blog/2013/01/13/fast-rgb-to-hsv
// void ImGui::ColorConvertRGBtoHSV(float r, float g, float b, float& out_h, float& out_s, float& out_v)
pub fn ColorConvertRGBtoHSV(mut r: f32, mut g: f32, mut b: f32, out_h: &mut f32, out_s: &mut f32, out_v: &mut f32)
{
    // float K = 0.f;
    let mut K: f32 = 0.0;
    if (g < b)
    {
        ImSwapF32(&mut g, &mut b);
        K = -1.0;
    }
    if (r < g)
    {
        ImSwapF32(&mut r, &mut g);
        K = -2.0 / 6.0 - K;
    }

    let mut chroma: f32 = r - (if g < b { g } else { b });
    *out_h = ImFabs(K + (g - b) / (6.0 * chroma + 1e-20));
    *out_s = chroma / (r + 1e-20);
    *out_v = r;
}

// Convert hsv floats ([0-1],[0-1],[0-1]) to rgb floats ([0-1],[0-1],[0-1]), from Foley & van Dam p593
// also http://en.wikipedia.org/wiki/HSL_and_HSV
// void ImGui::ColorConvertHSVtoRGB(float h, float s, float v, float& out_r, float& out_g, float& out_b)
pub fn ColorConvertHSVtoRGB(mut h: f32, s: f32, v: f32, out_r: &mut f32, out_g: &mut f32, out_b: &mut f32)
{
    if s == 0.0
    {
        // gray

        *out_r = v;
        *out_g = v;
        *out_b = v;
        return;
    }

    h = ImFmod(h, 1.0) / (60.0 / 360.0);
    // int   i = h;
    let mut i: i32 = h as i32;
    // float f = h - (float)i;
    let mut f: f32 = h - i as f32;
    // float p = v * (1.0 - s);
    let mut p: f32 = v * (1.0 - s);
    // float q = v * (1.0 - s * f);
    let mut q: f32 = v * (1.0 - s * f);
    // float t = v * (1.0 - s * (1.0 - f));
    let mut t: f32 = v * (1.0 - s * (1.0 - f));

    // switch (i)
    // {
    match i {
        // case 0: out_r = v; out_g = t; out_b = p; break;
        0 => { *out_r = v; *out_g = t; *out_b = p}
        // case 1: out_r = q; out_g = v; out_b = p; break;
        1 => { *out_r = q; *out_g = v; *out_b = p}
        // case 2: out_r = p; out_g = v; out_b = t; break;
        2 => {*out_r = p; *out_g = v; *out_b = t}
        // case 3: out_r = p; out_g = q; out_b = v; break;
        3 => {*out_r = p; *out_g = q; *out_b = v}
        // case 4: out_r = t; out_g = p; out_b = v; break;
        4 => {*out_r = t; *out_g = p; *out_b = v}
        // case 5: default: out_r = v; out_g = p; out_b = q; break;
        _ => {*out_r = v; *out_g = p; *out_b = q}
        // }
    }
}


// Helpers macros to generate 32-bit encoded colors
// User can declare their own format by #defining the 5 _SHIFT/_MASK macros in their imconfig file.
// #ifndef IM_COL32_R_SHIFT
// #ifdef IMGUI_USE_BGRA_PACKED_COLOR
pub const IM_COL32_R_SHIFT: u32 =    0;
pub const IM_COL32_G_SHIFT: u32 =    8;
pub const IM_COL32_B_SHIFT: u32 =    16;
pub const IM_COL32_A_SHIFT: u32 =    24;
pub const IM_COL32_A_MASK: u32 =     0xFF000000;
// #endif
// #endif
//#define IM_COL32(R,G,B,A)    (((A)<<IM_COL32_A_SHIFT) | ((B)<<IM_COL32_B_SHIFT) | ((G)<<IM_COL32_G_SHIFT) | ((R)<<IM_COL32_R_SHIFT))
pub fn IM_COL32(R: u32, G: u32, B: u32, A: u32) -> u32 {
    A << IM_COL32_A_SHIFT | B << IM_COL32_B_SHIFT | G << IM_COL32_G_SHIFT | R << IM_COL32_R_SHIFT
}
// #define IM_COL32_WHITE       IM_COL32(255,255,255,255)  // Opaque white = 0xFFFFFFFF
pub const IM_COL32_WHITE: u32 = IM_COL32(255,255,255,255);
// #define IM_COL32_BLACK       IM_COL32(0,0,0,255)        // Opaque black
pub const IM_COL32_BLACK: u32 = IM_COL32(0,0,0,255);
// #define IM_COL32_BLACK_TRANS IM_COL32(0,0,0,0)          // Transparent black = 0x00000000
pub const IM_COL32_BLACK_TRANS: u32 = IM_COL32(0,0,0,0);

// Helper: ImColor() implicitly converts colors to either ImU32 (packed 4x1 byte) or ImVec4 (4x1 float)
// Prefer using IM_COL32() macros if you want a guaranteed compile-time ImU32 for usage with ImDrawList API.
// **Avoid storing ImColor! Store either u32 of ImVec4. This is not a full-featured color class. MAY OBSOLETE.
// **None of the ImGui API are using ImColor directly but you can use it as a convenience to pass colors in either ImU32 or ImVec4 formats. Explicitly cast to ImU32 or ImVec4 if needed.
#[derive(Default,Debug,Clone)]
pub struct ImColor
{
    // ImVec4          Value;
    pub Value: ImVec4,
}

impl ImColor {
    // constexpr ImColor()                                             { }
    // constexpr ImColor(float r, float g, float b, float a = 1.0)    : Value(r, g, b, a) { }
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn new2(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            Value: ImVec4 {
                x: r,
                y: g,
                z: b,
                w: a
            }
        }
    }
    // constexpr ImColor(const ImVec4& col)                            : Value(col) {}
    pub fn new3(col: &ImVec4) -> Self {
        Self {
            Value: col.clone()
        }
    }
    // ImColor(int r, int g, int b, int a = 255)                       { float sc = 1.0 / 255.0; Value.x = (float)r * sc; Value.y = (float)g * sc; Value.z = (float)b * sc; Value.w = (float)a * sc; }
    pub fn new4(r: i32, g: i32, b: i32, a: i32) -> Self {
        let sc: f32 = 1.0/255.0;
        let Value = ImVec4 {
            x: r as f32 * sc,
            y: g as f32 * sc,
            z: b as f32 * sc,
            w: a as f32 * sc,
        };
        Self {
            Value
        }
    }
    // ImColor(ImU32 rgba)                                             { float sc = 1.0 / 255.0; Value.x = (float)((rgba >> IM_COL32_R_SHIFT) & 0xFF) * sc; Value.y = (float)((rgba >> IM_COL32_G_SHIFT) & 0xFF) * sc; Value.z = (float)((rgba >> IM_COL32_B_SHIFT) & 0xFF) * sc; Value.w = (float)((rgba >> IM_COL32_A_SHIFT) & 0xFF) * sc; }
    pub fn new5(rgba: u32) -> Self {
        let sc: f32 = 1.0/255.0;
        let Value = ImVec4 {
            x: (rgba >> IM_COL32_R_SHIFT & 0xff) as f32 * sc,
            y: (rgba >> IM_COL32_G_SHIFT & 0xff) as f32 * sc,
            z: (rgba >> IM_COL32_B_SHIFT & 0xff) as f32 * sc,
            w: (rgba >> IM_COL32_A_SHIFT & 0xff) as f32 * sc
        };
        Self {
            Value
        }
    }

    // inline operator ImU32() const                                   { return ImGui::ColorConvertFloat4ToU32(Value); }
    pub fn get_u32(&self) -> u32 {
        ColorConvertFloat4ToU32(&self.Value)
    }
    // inline operator ImVec4() const                                  { return Value; }
    pub fn get_vec4(&self) -> ImVec4 {
        self.Value.clone()
    }
    //
    // // FIXME-OBSOLETE: May need to obsolete/cleanup those helpers.
    // inline void    SetHSV(float h, float s, float v, float a = 1.0){ ImGui::ColorConvertHSVtoRGB(h, s, v, Value.x, Value.y, Value.z); Value.w = a; }
    // static ImColor HSV(float h, float s, float v, float a = 1.0)   { float r, g, b; ImGui::ColorConvertHSVtoRGB(h, s, v, r, g, b); return ImColor(r, g, b, a); }
}

/// Stacked color modifier, backup of modified data so we can restore it
#[derive(Default,Debug,Clone)]
pub struct DimgColorMod
{
    // ImGuiCol        Col;
    pub Col: ImGuiColor,
    // ImVec4          BackupValue;
    pub BackupValue: ImVec4,
}

/// Enumeration for PushStyleColor() / PopStyleColor()
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgColor
{
    Text,
    TextDisabled,
    WindowBg,              // Background of normal windows
    ChildBg,               // Background of child windows
    PopupBg,               // Background of popups, menus, tooltips windows
    Border,
    BorderShadow,
    FrameBg,               // Background of checkbox, radio button, plot, slider, text input
    FrameBgHovered,
    FrameBgActive,
    TitleBg,
    TitleBgActive,
    TitleBgCollapsed,
    MenuBarBg,
    ScrollbarBg,
    ScrollbarGrab,
    ScrollbarGrabHovered,
    ScrollbarGrabActive,
    CheckMark,
    SliderGrab,
    SliderGrabActive,
    Button,
    ButtonHovered,
    ButtonActive,
    Header,                // Header* colors are used for CollapsingHeader, TreeNode, Selectable, MenuItem
    HeaderHovered,
    HeaderActive,
    Separator,
    SeparatorHovered,
    SeparatorActive,
    ResizeGrip,            // Resize grip in lower-right and lower-left corners of windows.
    ResizeGripHovered,
    ResizeGripActive,
    Tab,                   // TabItem in a tab_bar
    TabHovered,
    TabActive,
    TabUnfocused,
    TabUnfocusedActive,
    DockingPreview,        // preview overlay color when about to docking something
    DockingEmptyBg,        // Background color for empty node (e.g. central_node with no window docked into it)
    PlotLines,
    PlotLinesHovered,
    PlotHistogram,
    PlotHistogramHovered,
    TableHeaderBg,         // Table header background
    TableBorderStrong,     // Table outer and header borders (prefer using Alpha=1.0 here)
    TableBorderLight,      // Table inner borders (prefer using Alpha=1.0 here)
    TableRowBg,            // Table row background (even rows)
    TableRowBgAlt,         // Table row background (odd rows)
    TextSelectedBg,
    DragDropTarget,        // Rectangle highlighting a drop target
    NavHighlight,          // Gamepad/keyboard: current highlighted item
    NavWindowingHighlight, // Highlight window when using CTRL+TAB
    NavWindowingDimBg,     // Darken/colorize entire screen behind the CTRL+TAB window list, when active
    ModalWindowDimBg,      // Darken/colorize entire screen behind a modal window, when one is active
}

// pub const DefaultOptions_: i32 = DimgColorEditFlags::Uint8 | DimgColorEditFlags::DisplayRGB | DimgColorEditFlags::InputRGB | DimgColorEditFlags::PickerHueBar;
pub const DFLT_OPTS: HashSet<DimgColorEditFlags> = HashSet::from([
    DimgColorEditFlags::Uint8, DimgColorEditFlags::DisplayRGB, DimgColorEditFlags::InputRGB, DimgColorEditFlags::PickerHueBar
]);

// [Internal] Masks
    // pub const DisplayMask_: i32    = DimgColorEditFlags::DisplayRGB | DimgColorEditFlags::DisplayHSV | DimgColorEditFlags::DisplayHex;
   pub const DISPLAY_MASK: HashSet<DimgColorEditFlags> = HashSet::from([
        DimgColorEditFlags::DisplayRGB, DimgColorEditFlags::DisplayHSV, DimgColorEditFlags::DisplayHex
    ]);

// pub const PickerMask_: i32     = DimgColorEditFlags::PickerHueWheel | DimgColorEditFlags::PickerHueBar;
pub const PICKER_MASK: HashSet<DimgColorEditFlags> = HashSet::from([
    DimgColorEditFlags::PickerHueWheel, DimgColorEditFlags::PickerHueBar
]);

// pub const InputMask_: i32      = DimgColorEditFlags::InputRGB | DimgColorEditFlags::InputHSV;
    pub const INPUT_MASK: HashSet<DimgColorEditFlags> = HashSet::from([
        DimgColorEditFlags::InputRGB, DimgColorEditFlags::InputHSV
    ]);

// flags for ColorEdit3() / ColorEdit4() / ColorPicker3() / ColorPicker4() / ColorButton()
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgColorEditFlags
{
    None            = 0,
    NoAlpha         = 1 << 1,   //              // ColorEdit, ColorPicker, ColorButton: ignore Alpha component (will only read 3 components from the input pointer).
    NoPicker        = 1 << 2,   //              // ColorEdit: disable picker when clicking on color square.
    NoOptions       = 1 << 3,   //              // ColorEdit: disable toggling options menu when right-clicking on inputs/small preview.
    NoSmallPreview  = 1 << 4,   //              // ColorEdit, ColorPicker: disable color square preview next to the inputs. (e.g. to show only the inputs)
    NoInputs        = 1 << 5,   //              // ColorEdit, ColorPicker: disable inputs sliders/text widgets (e.g. to show only the small preview color square).
    NoTooltip       = 1 << 6,   //              // ColorEdit, ColorPicker, ColorButton: disable tooltip when hovering the preview.
    NoLabel         = 1 << 7,   //              // ColorEdit, ColorPicker: disable display of inline text label (the label is still forwarded to the tooltip and picker).
    NoSidePreview   = 1 << 8,   //              // ColorPicker: disable bigger color preview on right side of the picker, use small color square preview instead.
    NoDragDrop      = 1 << 9,   //              // ColorEdit: disable drag and drop target. ColorButton: disable drag and drop source.
    NoBorder        = 1 << 10,  //              // ColorButton: disable border (which is enforced by default)

    // User Options (right-click on widget to change some of them).
    AlphaBar        = 1 << 16,  //              // ColorEdit, ColorPicker: show vertical alpha bar/gradient in picker.
    AlphaPreview    = 1 << 17,  //              // ColorEdit, ColorPicker, ColorButton: display preview as a transparent color over a checkerboard, instead of opaque.
    AlphaPreviewHalf= 1 << 18,  //              // ColorEdit, ColorPicker, ColorButton: display half opaque / half checkerboard, instead of opaque.
    HDR             = 1 << 19,  //              // (WIP) ColorEdit: Currently only disable 0.0..1.0 limits in RGBA edition (note: you probably want to use ImGuiColorEditFlags_Float flag as well).
    DisplayRGB      = 1 << 20,  // [Display]    // ColorEdit: override _display_ type among RGB/HSV/Hex. ColorPicker: select any combination using one or more of RGB/HSV/Hex.
    DisplayHSV      = 1 << 21,  // [Display]    // "
    DisplayHex      = 1 << 22,  // [Display]    // "
    Uint8           = 1 << 23,  // [data_type]   // ColorEdit, ColorPicker, ColorButton: _display_ values formatted as 0..255.
    Float           = 1 << 24,  // [data_type]   // ColorEdit, ColorPicker, ColorButton: _display_ values formatted as 0.0..1.0 floats instead of 0..255 integers. No round-trip of value via integers.
    PickerHueBar    = 1 << 25,  // [Picker]     // ColorPicker: bar for Hue, rectangle for Sat/Value.
    PickerHueWheel  = 1 << 26,  // [Picker]     // ColorPicker: wheel for Hue, triangle for Sat/Value.
    InputRGB        = 1 << 27,  // [Input]      // ColorEdit, ColorPicker: input and output data in RGB format.
    InputHSV        = 1 << 28,  // [Input]      // ColorEdit, ColorPicker: input and output data in HSV format.

    // Defaults Options. You can set application defaults using SetColorEditOptions(). The intent is that you probably don't want to
    // override them in most of your calls. Let the user choose via the option menu and/or call SetColorEditOptions() once during startup.
    // ImGuiColorEditFlags_DefaultOptions_ = ImGuiColorEditFlags_Uint8 | ImGuiColorEditFlags_DisplayRGB | ImGuiColorEditFlags_InputRGB | ImGuiColorEditFlags_PickerHueBar,
    //
    // // [Internal] Masks
    // ImGuiColorEditFlags_DisplayMask_    = ImGuiColorEditFlags_DisplayRGB | ImGuiColorEditFlags_DisplayHSV | ImGuiColorEditFlags_DisplayHex,
    // ImGuiColorEditFlags_DataTypeMask_   = ImGuiColorEditFlags_Uint8 | ImGuiColorEditFlags_Float,
    // ImGuiColorEditFlags_PickerMask_     = ImGuiColorEditFlags_PickerHueWheel | ImGuiColorEditFlags_PickerHueBar,
    // ImGuiColorEditFlags_InputMask_      = ImGuiColorEditFlags_InputRGB | ImGuiColorEditFlags_InputHSV

    // Obsolete names (will be removed)
    // ImGuiColorEditFlags_RGB = ImGuiColorEditFlags_DisplayRGB, ImGuiColorEditFlags_HSV = ImGuiColorEditFlags_DisplayHSV, ImGuiColorEditFlags_HEX = ImGuiColorEditFlags_DisplayHex  // [renamed in 1.69]
}
