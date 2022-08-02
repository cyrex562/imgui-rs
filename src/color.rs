use std::collections::HashSet;
use crate::imgui_h::Color;
use crate::imgui_math::{IM_F32_TO_INT8_SAT};
use crate::imgui_vec::Vector4D;
use crate::math::{f32_mod, im_f32_to_int8_sat, lerp_u32};
use crate::vectors::Vector4D;

pub fn im_alpha_blend_colors(col_a: u32, col_b: u32) -> u32 {
    // float t = ((col_b >> IM_COL32_A_SHIFT) & 0xFF) / 255.f;
    let t = ((col_b >> IM_COL32_A_SHIFT) & 0xff) as f32 / 255.0;
    // int r = ImLerp((col_a >> IM_COL32_R_SHIFT) & 0xFF, (col_b >> IM_COL32_R_SHIFT) & 0xFF, t);
    let r = lerp_u32((col_a >> IM_COL32_R_SHIFT) & 0xff, (col_b >> IM_COL32_R_SHIFT) & 0xff, t);
    // int g = ImLerp((col_a >> IM_COL32_G_SHIFT) & 0xFF, (col_b >> IM_COL32_G_SHIFT) & 0xFF, t);
    let g = lerp_u32((col_a >> IM_COL32_G_SHIFT) & 0xff, (col_b >> IM_COL32_G_SHIFT) & 0xff, t);
    // int b = ImLerp((col_a >> IM_COL32_B_SHIFT) & 0xFF, (col_b >> IM_COL32_B_SHIFT) & 0xFF, t);
    let b = lerp_u32((col_a >> IM_COL32_B_SHIFT) & 0xff, (col_b >> IM_COL32_B_SHIFT) & 0xff, t);
    make_color_32(r, g, b, 0xFF)
}

pub fn convert_u32_color_to_vector4d(in_u32: u32) -> Vector4D {
    let mut s: f32 = 1.0 / 255.0;
    return Vector4D {
        x: ((in_u32 >> IM_COL32_R_SHIFT) & 0xFF) as f32 * s,
        y: ((in_u32 >> IM_COL32_G_SHIFT) & 0xFF) as f32 * s,
        z: ((in_u32 >> IM_COL32_B_SHIFT) & 0xFF) as f32 * s,
        w: ((in_u32 >> IM_COL32_A_SHIFT) & 0xFF) as f32 * s,
    };
}

pub fn convert_vector4d_to_u32_color(in_vec: &Vector4D) -> u32 {
    let mut out: u32 = ((im_f32_to_int8_sat(in_vec.x)) << IM_COL32_R_SHIFT) as u32;
    out |= ((im_f32_to_int8_sat(in_vec.y)) << IM_COL32_G_SHIFT) as u32;
    out |= ((im_f32_to_int8_sat(in_vec.z)) << IM_COL32_B_SHIFT) as u32;
    out |= ((im_f32_to_int8_sat(in_vec.w)) << IM_COL32_A_SHIFT) as u32;
    out
}

/// Convert rgb floats ([0-1],[0-1],[0-1]) to hsv floats ([0-1],[0-1],[0-1]), from Foley & van Dam p592
/// Optimized http://lolengine.net/blog/2013/01/13/fast-rgb-to-hsv
pub fn convert_rgb_to_hsv(mut r: f32, mut g: f32, mut b: f32, out_h: &mut f32, out_s: &mut f32, out_v: &mut f32) {
    let mut k: f32 = 0.0;
    if g < b {
        f32::swap(&mut g, &mut b);
        k = -1.0;
    }
    if r < g {
        f32::swap(&mut r, &mut g);
        k = -2.0 / 6.0 - k;
    }

    let mut chroma: f32 = r - (if g < b { g } else { b });
    *out_h = f32::abs(k + (g - b) / (6.0 * chroma + 1e-20));
    *out_s = chroma / (r + 1e-20);
    *out_v = r;
}

/// Convert hsv floats ([0-1],[0-1],[0-1]) to rgb floats ([0-1],[0-1],[0-1]), from Foley & van Dam p593
/// also http://en.wikipedia.org/wiki/HSL_and_HSV
pub fn convert_hsv_to_rgb(mut h: f32, s: f32, v: f32, out_r: &mut f32, out_g: &mut f32, out_b: &mut f32) {
    if s == 0.0 {
        // gray

        *out_r = v;
        *out_g = v;
        *out_b = v;
        return;
    }

    h = f32_mod(h, 1.0) / (60.0 / 360.0);
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
        0 => {
            *out_r = v;
            *out_g = t;
            *out_b = p
        }
        // case 1: out_r = q; out_g = v; out_b = p; break;
        1 => {
            *out_r = q;
            *out_g = v;
            *out_b = p
        }
        // case 2: out_r = p; out_g = v; out_b = t; break;
        2 => {
            *out_r = p;
            *out_g = v;
            *out_b = t
        }
        // case 3: out_r = p; out_g = q; out_b = v; break;
        3 => {
            *out_r = p;
            *out_g = q;
            *out_b = v
        }
        // case 4: out_r = t; out_g = p; out_b = v; break;
        4 => {
            *out_r = t;
            *out_g = p;
            *out_b = v
        }
        // case 5: default: out_r = v; out_g = p; out_b = q; break;
        _ => {
            *out_r = v;
            *out_g = p;
            *out_b = q
        }
        // }
    }
}


/// Helpers macros to generate 32-bit encoded colors
/// User can declare their own format by #defining the 5 _SHIFT/_MASK macros in their imconfig file.
pub const IM_COL32_R_SHIFT: u32 = 0;
pub const IM_COL32_G_SHIFT: u32 = 8;
pub const IM_COL32_B_SHIFT: u32 = 16;
pub const IM_COL32_A_SHIFT: u32 = 24;
pub const COLOR_32_A_MASK: u32 = 0xFF000000;

///#define IM_COL32(R,G,B,A)    (((A)<<IM_COL32_A_SHIFT) | ((B)<<IM_COL32_B_SHIFT) | ((G)<<IM_COL32_G_SHIFT) | ((R)<<IM_COL32_R_SHIFT))
pub fn make_color_32(red: u32, green: u32, blue: u32, alpha: u32) -> u32 {
    alpha << IM_COL32_A_SHIFT | blue << IM_COL32_B_SHIFT | green << IM_COL32_G_SHIFT | red << IM_COL32_R_SHIFT
}

/// #define IM_COL32_WHITE       IM_COL32(255,255,255,255)  // Opaque white = 0xFFFFFFFF
pub const IM_COL32_WHITE: u32 = make_color_32(255, 255, 255, 255);
/// #define IM_COL32_BLACK       IM_COL32(0,0,0,255)        // Opaque black
pub const IM_COL32_BLACK: u32 = make_color_32(0, 0, 0, 255);
/// #define IM_COL32_BLACK_TRANS IM_COL32(0,0,0,0)          // Transparent black = 0x00000000
pub const IM_COL32_BLACK_TRANS: u32 = make_color_32(0, 0, 0, 0);

/// Helper: ImColor() implicitly converts colors to either ImU32 (packed 4x1 byte) or Vector4D (4x1 float)
/// Prefer using IM_COL32() macros if you want a guaranteed compile-time ImU32 for usage with ImDrawList API.
/// **Avoid storing ImColor! Store either u32 of Vector4D. This is not a full-featured color class. MAY OBSOLETE.
/// **None of the ImGui API are using ImColor directly but you can use it as a convenience to pass colors in either ImU32 or Vector4D formats. Explicitly cast to ImU32 or Vector4D if needed.
#[derive(Default, Debug, Clone)]
pub struct Color {
    // Vector4D          value;
    pub value: Vector4D,
}

impl From<(f32, f32, f32, f32)> for Color {
    fn from(floats: (f32, f32, f32, f32)) -> Self {
        let (r, g, b, a) = floats;
        Self {
            value: Vector4D {
                x: r,
                y: g,
                z: b,
                w: a,
            }
        }
    }
}

impl From<Vector4D> for Color {
    fn from(x: Vector4D) -> Self {
        Self {
            value: x
        }
    }
}

impl From<(i32, i32, i32, i32)> for Color {
    fn from(ints: (i32, i32, i32, i32)) -> Self {
        let (r, g, b, a) = ints;
        let sc: f32 = 1.0 / 255.0;
        let value = Vector4D::new(r as f32 * sc, g as f32 * sc, b as f32 * sc, a as f32 * sc);
        Self {
            value
        }
    }
}

impl From<u32> for Color {
    fn from(rgba: u32) -> Self {
        let sc: f32 = 1.0 / 255.0;
        let value = Vector4D {
            x: (rgba >> IM_COL32_R_SHIFT & 0xff) as f32 * sc,
            y: (rgba >> IM_COL32_G_SHIFT & 0xff) as f32 * sc,
            z: (rgba >> IM_COL32_B_SHIFT & 0xff) as f32 * sc,
            w: (rgba >> IM_COL32_A_SHIFT & 0xff) as f32 * sc,
        };
        Self {
            value
        }
    }
}

impl Into<u32> for Color {
    fn into(self) -> u32 {
        convert_vector4d_to_u32_color(&self.value)
    }
}

impl Into<Vector4D> for Color {
    fn into(self) -> Vector4D {
        self.value.clone()
    }
}

/// Stacked color modifier, backup of modified data so we can restore it
#[derive(Default, Debug, Clone)]
pub struct StackedColorModifier {
    // ImGuiCol        col;
    pub col: Color,
    // Vector4D          backup_value;
    pub backup_value: Vector4D,
}

/// Enumeration for PushStyleColor() / PopStyleColor()
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum StyleColor {
    Text,
    TextDisabled,
    WindowBg,
    // Background of normal windows
    ChildBg,
    // Background of child windows
    PopupBg,
    // Background of popups, menus, tooltips windows
    Border,
    BorderShadow,
    FrameBg,
    // Background of checkbox, radio button, plot, slider, text input
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
    Header,
    // Header* colors are used for CollapsingHeader, TreeNode, selectable, menu_item
    HeaderHovered,
    HeaderActive,
    Separator,
    SeparatorHovered,
    SeparatorActive,
    ResizeGrip,
    // Resize grip in lower-right and lower-left corners of windows.
    ResizeGripHovered,
    ResizeGripActive,
    Tab,
    // TabItem in a tab_bar
    TabHovered,
    TabActive,
    TabUnfocused,
    TabUnfocusedActive,
    DockingPreview,
    // preview overlay color when about to docking something
    DockingEmptyBg,
    // Background color for empty node (e.g. central_node with no window docked into it)
    PlotLines,
    PlotLinesHovered,
    PlotHistogram,
    PlotHistogramHovered,
    TableHeaderBg,
    // Table header background
    TableBorderStrong,
    // Table outer and header borders (prefer using Alpha=1.0 here)
    TableBorderLight,
    // Table inner borders (prefer using Alpha=1.0 here)
    TableRowBg,
    // Table row background (even rows)
    TableRowBgAlt,
    // Table row background (odd rows)
    TextSelectedBg,
    DragDropTarget,
    // Rectangle highlighting a drop target
    NavHighlight,
    // Gamepad/keyboard: current highlighted item
    NavWindowingHighlight,
    // Highlight window when using CTRL+TAB
    NavWindowingDimBg,
    // Darken/colorize entire screen behind the CTRL+TAB window list, when active
    ModalWindowDimBg,      // Darken/colorize entire screen behind a modal window, when one is active
}

// flags for ColorEdit3() / ColorEdit4() / ColorPicker3() / ColorPicker4() / ColorButton()
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ColorEditFlags {
    None,
    NoAlpha,
    //              // ColorEdit, ColorPicker, ColorButton: ignore Alpha component (will only read 3 components from the input pointer).
    NoPicker,
    //              // ColorEdit: disable picker when clicking on color square.
    NoOptions,
    //              // ColorEdit: disable toggling options menu when right-clicking on inputs/small preview.
    NoSmallPreview,
    //              // ColorEdit, ColorPicker: disable color square preview next to the inputs. (e.g. to show only the inputs)
    NoInputs,
    //              // ColorEdit, ColorPicker: disable inputs sliders/text widgets (e.g. to show only the small preview color square).
    NoTooltip,
    //              // ColorEdit, ColorPicker, ColorButton: disable tooltip when hovering the preview.
    NoLabel,
    //              // ColorEdit, ColorPicker: disable display of inline text label (the label is still forwarded to the tooltip and picker).
    NoSidePreview,
    //              // ColorPicker: disable bigger color preview on right side of the picker, use small color square preview instead.
    NoDragDrop,
    //              // ColorEdit: disable drag and drop target. ColorButton: disable drag and drop source.
    NoBorder,  //              // ColorButton: disable border (which is enforced by default)

    // User Options (right-click on widget to change some of them).
    AlphaBar,
    //              // ColorEdit, ColorPicker: show vertical alpha bar/gradient in picker.
    AlphaPreview,
    //              // ColorEdit, ColorPicker, ColorButton: display preview as a transparent color over a checkerboard, instead of opaque.
    AlphaPreviewHalf,
    //              // ColorEdit, ColorPicker, ColorButton: display half opaque / half checkerboard, instead of opaque.
    HDR,
    //              // (WIP) ColorEdit: Currently only disable 0.0..1.0 limits in RGBA edition (note: you probably want to use ImGuiColorEditFlags_Float flag as well).
    DisplayRGB,
    // [Display]    // ColorEdit: override _display_ type among RGB/HSV/Hex. ColorPicker: select any combination using one or more of RGB/HSV/Hex.
    DisplayHSV,
    // [Display]    // "
    DisplayHex,
    // [Display]    // "
    Uint8,
    // [data_type]   // ColorEdit, ColorPicker, ColorButton: _display_ values formatted as 0..255.
    Float,
    // [data_type]   // ColorEdit, ColorPicker, ColorButton: _display_ values formatted as 0.0..1.0 floats instead of 0..255 integers. No round-trip of value via integers.
    PickerHueBar,
    // [Picker]     // ColorPicker: bar for Hue, rectangle for Sat/value.
    PickerHueWheel,
    // [Picker]     // ColorPicker: wheel for Hue, triangle for Sat/value.
    InputRGB,
    // [Input]      // ColorEdit, ColorPicker: input and output data in RGB format.
    InputHSV,  // [Input]      // ColorEdit, ColorPicker: input and output data in HSV format.
}

/// Defaults Options. You can set application defaults using SetColorEditOptions(). The intent is that you probably don't want to
/// override them in most of your calls. Let the user choose via the option menu and/or call SetColorEditOptions() once during startup.
pub const COLOR_EDIT_FLAGS_DFLT_OPTS: HashSet<ColorEditFlags> = HashSet::from([
    ColorEditFlags::Uint8,
    ColorEditFlags::DisplayRGB,
    ColorEditFlags::InputRGB,
    ColorEditFlags::PickerHueBar
]);

pub const COLOR_EDIT_FLAGS_DISPLAY_MASK: HashSet<ColorEditFlags> = HashSet::from([
    ColorEditFlags::DisplayRGB, ColorEditFlags::DisplayHSV, ColorEditFlags::DisplayHex
]);

pub const COLOR_EDIT_FLAGS_DATA_TYPE_MASK: HashSet<ColorEditFlags> = HashSet::from([
    ColorEditFlags::Uint8, ColorEditFlags::Float
]);

pub const COLOR_EDIT_FLAGS_PICKER_MASK: HashSet<ColorEditFlags> = HashSet::from([
    ColorEditFlags::PickerHueBar, ColorEditFlags::PickerHueBar
]);

pub const COLOR_EDIT_FLAGS_INPUT_MASK: HashSet<ColorEditFlags> = HashSet::from([
    ColorEditFlags::InputRGB, ColorEditFlags::InputHSV
]);

