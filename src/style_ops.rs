#![allow(non_snake_case)]

use std::ffi::CString;
use libc::{c_char, c_float, c_int};
use crate::color::{IM_COL32_A_MASK, IM_COL32_A_SHIFT, ImGuiCol, ImGuiCol_Border, ImGuiCol_BorderShadow, ImGuiCol_Button, ImGuiCol_ButtonActive, ImGuiCol_ButtonHovered, ImGuiCol_CheckMark, ImGuiCol_ChildBg, ImGuiCol_DockingEmptyBg, ImGuiCol_DockingPreview, ImGuiCol_DragDropTarget, ImGuiCol_FrameBg, ImGuiCol_FrameBgActive, ImGuiCol_FrameBgHovered, ImGuiCol_Header, ImGuiCol_HeaderActive, ImGuiCol_HeaderHovered, ImGuiCol_MenuBarBg, ImGuiCol_ModalWindowDimBg, ImGuiCol_NavHighlight, ImGuiCol_NavWindowingDimBg, ImGuiCol_NavWindowingHighlight, ImGuiCol_PlotHistogram, ImGuiCol_PlotHistogramHovered, ImGuiCol_PlotLines, ImGuiCol_PlotLinesHovered, ImGuiCol_PopupBg, ImGuiCol_ResizeGrip, ImGuiCol_ResizeGripActive, ImGuiCol_ResizeGripHovered, ImGuiCol_ScrollbarBg, ImGuiCol_ScrollbarGrab, ImGuiCol_ScrollbarGrabActive, ImGuiCol_ScrollbarGrabHovered, ImGuiCol_Separator, ImGuiCol_SeparatorActive, ImGuiCol_SeparatorHovered, ImGuiCol_SliderGrab, ImGuiCol_SliderGrabActive, ImGuiCol_Tab, ImGuiCol_TabActive, ImGuiCol_TabHovered, ImGuiCol_TableBorderLight, ImGuiCol_TableBorderStrong, ImGuiCol_TableHeaderBg, ImGuiCol_TableRowBg, ImGuiCol_TableRowBgAlt, ImGuiCol_TabUnfocused, ImGuiCol_TabUnfocusedActive, ImGuiCol_Text, ImGuiCol_TextDisabled, ImGuiCol_TextSelectedBg, ImGuiCol_TitleBg, ImGuiCol_TitleBgActive, ImGuiCol_TitleBgCollapsed, ImGuiCol_WindowBg};
use crate::color_mod::ImGuiColorMod;
use crate::color_ops::{ColorConvertFloat4ToU32, ColorConvertU32ToFloat4};
use crate::style::ImGuiStyle;
use crate::imgui::GImGui;
use crate::math_ops::ImLerp;
use crate::utils::is_not_null;
use crate::vec4::ImVec4;

// ImGuiStyle& GetStyle()
pub fn GetStyle() -> &mut ImGuiStyle {
// IM_ASSERT(GImGui != NULL && "No current context. Did you call CreateContext() and SetCurrentContext() ?");
    return GimGui.Style;
}


// GetColorU32: u32(ImGuiCol idx, c_float alpha_mul)
pub fn GetColorU32(idx: ImGuiCol, alpha_mul: c_float) -> u32 {
    let style = GimGui.Style;
    let mut c = style.Colors[idx];
    c.w *= style.Alpha.clone() * alpha_mul;
    return ColorConvertFloat4ToU32(c);
}

// GetColorU32: u32(const ImVec4& col)
pub fn GetColorU32FromImVec4(col: &ImVec4) -> u32 {
    let style = GimGui.Style;
    let mut c = col.clone();
    c.w *= style.Alpha.clone();
    return ColorConvertFloat4ToU32(&c);
}

// const ImVec4& GetStyleColorVec4(ImGuiCol idx)
pub fn GetStyleColorVec4(idx: ImGuiCol) -> &ImVec4 {
    let style = GimGui.Style;
    return style.Colors[idx];
}

// GetColorU32: u32(col: u32)
pub fn GetColorU32FromU32(col: u32) -> u32 {
    let style = GimGui.Style;
    if style.Alpha >= 1.0 {
        return col;
    }
    let a = (col & IM_COL32_A_MASK) >> IM_COL32_A_SHIFT;
    a = (a * style.Alpha.clone()); // We don't need to clamp 0..255 because Style.Alpha is in 0..1 range.
    return (col.clone() & !IM_COL32_A_MASK) | (a << IM_COL32_A_SHIFT);
}

// FIXME: This may incur a round-trip (if the end user got their data from a float4) but eventually we aim to store the in-flight colors as ImU32
// c_void PushStyleColor(ImGuiCol idx, col: u32)
pub fn PushStyleColor(idx: ImGuiCol, col: u32) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut backup: ImGuiColorMod = ImGuiColorMod::default();
    backup.Col = idx;
    backup.BackupValue = g.Style.Colors[idx.clone()];
    g.ColorStack.push(backup);
    g.Style.Colors[idx.clone()] = ColorConvertU32ToFloat4(col);
}

// c_void PushStyleColor(ImGuiCol idx, const ImVec4& col)
pub fn PushStyleColor2(idx: ImGuiCol, col: &ImVec4) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut backup: ImGuiColorMod = ImGuiColorMod::default();
    backup.Col = idx;
    backup.BackupValue = g.Style.Colors[idx.clone()];
    g.ColorStack.push(backup);
    g.Style.Colors[idx.clone()] = col;
}

// c_void PopStyleColor(count: c_int)
pub fn PopStyleColor(mut count: c_int) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if g.ColorStack.Size < count {
// IM_ASSERT_USER_ERROR(g.ColorStack.Size > count, "Calling PopStyleColor() too many times: stack underflow.");
        count = g.ColorStack.Size;
    }
    while count > 0 {
        let backup = g.ColorStack.last().unwrap();
        g.Style.Colors[backup.Col] = backup.BackupValue.clone();
        g.ColorStack.pop_back();
        count -= 1;
    }
}


//noinspection ALL
// GetStyleColorName: *const c_char(ImGuiCol idx)
pub unsafe fn GetStyleColorName(idx: ImGuiCol) -> *const c_char
{
    // Create switch-case from enum with regexp: ImGuiCol_{.*}, --> case ImGuiCol_\1: return "\1";
    return match idx {
        ImGuiCol_Text => CString::new("Text").unwrap().as_ptr(),
        ImGuiCol_TextDisabled => CString::new("TextDisabled").unwrap().as_ptr(),
        ImGuiCol_WindowBg => CString::new("WindowBg").unwrap().as_ptr(),
        ImGuiCol_ChildBg => CString::new("ChildBg").unwrap().as_ptr(),
        ImGuiCol_PopupBg => CString::new("PopupBg").unwrap().as_ptr(),
        ImGuiCol_Border => CString::new("Border").unwrap().as_ptr(),
        ImGuiCol_BorderShadow => CString::new("BorderShadow").unwrap().as_ptr(),
        ImGuiCol_FrameBg => CString::new("FrameBg").unwrap().as_ptr(),
        ImGuiCol_FrameBgHovered => CString::new("FrameBgHovered").unwrap().as_ptr(),
        ImGuiCol_FrameBgActive => CString::new("FrameBgActive").unwrap().as_ptr(),
        ImGuiCol_TitleBg => CString::new("TitleBg").unwrap().as_ptr(),
        ImGuiCol_TitleBgActive => CString::new("TitleBgActive").unwrap().as_ptr(),
        ImGuiCol_TitleBgCollapsed => CString::new("TitleBgCollapsed").unwrap().as_ptr(),
        ImGuiCol_MenuBarBg => CString::new("MenuBarBg").unwrap().as_ptr(),
        ImGuiCol_ScrollbarBg => CString::new("ScrollbarBg").unwrap().as_ptr(),
        ImGuiCol_ScrollbarGrab => CString::new("ScrollbarGrab").unwrap().as_ptr(),
        ImGuiCol_ScrollbarGrabHovered => CString::new("ScrollbarGrabHovered").unwrap().as_ptr(),
        ImGuiCol_ScrollbarGrabActive => CString::new("ScrollbarGrabActive").unwrap().as_ptr(),
        ImGuiCol_CheckMark => CString::new("CheckMark").unwrap().as_ptr(),
        ImGuiCol_SliderGrab => CString::new("SliderGrab").unwrap().as_ptr(),
        ImGuiCol_SliderGrabActive => CString::new("SliderGrabActive").unwrap().as_ptr(),
        ImGuiCol_Button => CString::new("Button").unwrap().as_ptr(),
        ImGuiCol_ButtonHovered => CString::new("ButtonHovered").unwrap().as_ptr(),
        ImGuiCol_ButtonActive => CString::new("ButtonActive").unwrap().as_ptr(),
        ImGuiCol_Header => CString::new("Header").unwrap().as_ptr(),
        ImGuiCol_HeaderHovered => CString::new("HeaderHovered").unwrap().as_ptr(),
        ImGuiCol_HeaderActive => CString::new("HeaderActive").unwrap().as_ptr(),
        ImGuiCol_Separator => CString::new("Separator").unwrap().as_ptr(),
        ImGuiCol_SeparatorHovered => CString::new("SeparatorHovered").unwrap().as_ptr(),
        ImGuiCol_SeparatorActive => CString::new("SeparatorActive").unwrap().as_ptr(),
        ImGuiCol_ResizeGrip => CString::new("ResizeGrip").unwrap().as_ptr(),
        ImGuiCol_ResizeGripHovered => CString::new("ResizeGripHovered").unwrap().as_ptr(),
        ImGuiCol_ResizeGripActive => CString::new("ResizeGripActive").unwrap().as_ptr(),
        ImGuiCol_Tab => CString::new("Tab").unwrap().as_ptr(),
        ImGuiCol_TabHovered => CString::new("TabHovered").unwrap().as_ptr(),
        ImGuiCol_TabActive => CString::new("TabActive").unwrap().as_ptr(),
        ImGuiCol_TabUnfocused => CString::new("TabUnfocused").unwrap().as_ptr(),
        ImGuiCol_TabUnfocusedActive => CString::new("TabUnfocusedActive").unwrap().as_ptr(),
        ImGuiCol_DockingPreview => CString::new("DockingPreview").unwrap().as_ptr(),
        ImGuiCol_DockingEmptyBg => CString::new("DockingEmptyBg").unwrap().as_ptr(),
        ImGuiCol_PlotLines => CString::new("PlotLines").unwrap().as_ptr(),
        ImGuiCol_PlotLinesHovered => CString::new("PlotLinesHovered").unwrap().as_ptr(),
        ImGuiCol_PlotHistogram => CString::new("PlotHistogram").unwrap().as_ptr(),
        ImGuiCol_PlotHistogramHovered => CString::new("PlotHistogramHovered").unwrap().as_ptr(),
        ImGuiCol_TableHeaderBg => CString::new("TableHeaderBg").unwrap().as_ptr(),
        ImGuiCol_TableBorderStrong => CString::new("TableBorderStrong").unwrap().as_ptr(),
        ImGuiCol_TableBorderLight => CString::new("TableBorderLight").unwrap().as_ptr(),
        ImGuiCol_TableRowBg => CString::new("TableRowBg").unwrap().as_ptr(),
        ImGuiCol_TableRowBgAlt => CString::new("TableRowBgAlt").unwrap().as_ptr(),
        ImGuiCol_TextSelectedBg => CString::new("TextSelectedBg").unwrap().as_ptr(),
        ImGuiCol_DragDropTarget => CString::new("DragDropTarget").unwrap().as_ptr(),
        ImGuiCol_NavHighlight => CString::new("NavHighlight").unwrap().as_ptr(),
        ImGuiCol_NavWindowingHighlight => CString::new("NavWindowingHighlight").unwrap().as_ptr(),
        ImGuiCol_NavWindowingDimBg => CString::new("NavWindowingDimBg").unwrap().as_ptr(),
        ImGuiCol_ModalWindowDimBg => CString::new("ModalWindowDimBg").unwrap().as_ptr(),
        _ => CString::new("Unknown").unwrap().as_ptr()
    };
    // IM_ASSERT(0);
    // return "Unknown";
}


pub fn StyleColorsDark(dst: *mut ImGuiStyle)
{
    let style: *mut ImGuiStyle = if is_not_null(dst) { dst } else { &GetStyle() };
    colors: *mut ImVec4 = style.Colors.as_mut_ptr();

    colors[ImGuiCol_Text]                   = ImVec4::from_floats(1.0, 1.0, 1.0, 1.0);
    colors[ImGuiCol_TextDisabled]           = ImVec4::from_floats(0.50, 0.50, 0.50, 1.0);
    colors[ImGuiCol_WindowBg]               = ImVec4::from_floats(0.06, 0.06, 0.06, 0.940);
    colors[ImGuiCol_ChildBg]                = ImVec4::from_floats(0.00, 0.00, 0.00, 0.0);
    colors[ImGuiCol_PopupBg]                = ImVec4::from_floats(0.08, 0.08, 0.08, 0.940);
    colors[ImGuiCol_Border]                 = ImVec4::from_floats(0.43, 0.43, 0.50, 0.5);
    colors[ImGuiCol_BorderShadow]           = ImVec4::from_floats(0.00, 0.00, 0.00, 0.0);
    colors[ImGuiCol_FrameBg]                = ImVec4::from_floats(0.16, 0.29, 0.48, 0.540);
    colors[ImGuiCol_FrameBgHovered]         = ImVec4::from_floats(0.26, 0.59, 0.98, 0.4);
    colors[ImGuiCol_FrameBgActive]          = ImVec4::from_floats(0.26, 0.59, 0.98, 0.670);
    colors[ImGuiCol_TitleBg]                = ImVec4::from_floats(0.04, 0.04, 0.04, 1.0);
    colors[ImGuiCol_TitleBgActive]          = ImVec4::from_floats(0.16, 0.29, 0.48, 1.0);
    colors[ImGuiCol_TitleBgCollapsed]       = ImVec4::from_floats(0.00, 0.00, 0.00, 0.510);
    colors[ImGuiCol_MenuBarBg]              = ImVec4::from_floats(0.14, 0.14, 0.14, 1.0);
    colors[ImGuiCol_ScrollbarBg]            = ImVec4::from_floats(0.02, 0.02, 0.02, 0.530);
    colors[ImGuiCol_ScrollbarGrab]          = ImVec4::from_floats(0.31, 0.31, 0.31, 1.0);
    colors[ImGuiCol_ScrollbarGrabHovered]   = ImVec4::from_floats(0.41, 0.41, 0.41, 1.0);
    colors[ImGuiCol_ScrollbarGrabActive]    = ImVec4::from_floats(0.51, 0.51, 0.51, 1.0);
    colors[ImGuiCol_CheckMark]              = ImVec4::from_floats(0.26, 0.59, 0.98, 1.0);
    colors[ImGuiCol_SliderGrab]             = ImVec4::from_floats(0.24, 0.52, 0.88, 1.0);
    colors[ImGuiCol_SliderGrabActive]       = ImVec4::from_floats(0.26, 0.59, 0.98, 1.0);
    colors[ImGuiCol_Button]                 = ImVec4::from_floats(0.26, 0.59, 0.98, 0.4);
    colors[ImGuiCol_ButtonHovered]          = ImVec4::from_floats(0.26, 0.59, 0.98, 1.0);
    colors[ImGuiCol_ButtonActive]           = ImVec4::from_floats(0.06, 0.53, 0.98, 1.0);
    colors[ImGuiCol_Header]                 = ImVec4::from_floats(0.26, 0.59, 0.98, 0.310);
    colors[ImGuiCol_HeaderHovered]          = ImVec4::from_floats(0.26, 0.59, 0.98, 0.8);
    colors[ImGuiCol_HeaderActive]           = ImVec4::from_floats(0.26, 0.59, 0.98, 1.0);
    colors[ImGuiCol_Separator]              = colors[ImGuiCol_Border];
    colors[ImGuiCol_SeparatorHovered]       = ImVec4::from_floats(0.1, 0.40, 0.75, 0.780);
    colors[ImGuiCol_SeparatorActive]        = ImVec4::from_floats(0.1, 0.40, 0.75, 1.0);
    colors[ImGuiCol_ResizeGrip]             = ImVec4::from_floats(0.26, 0.59, 0.98, 0.200);
    colors[ImGuiCol_ResizeGripHovered]      = ImVec4::from_floats(0.26, 0.59, 0.98, 0.670);
    colors[ImGuiCol_ResizeGripActive]       = ImVec4::from_floats(0.26, 0.59, 0.98, 0.950);
    colors[ImGuiCol_Tab]                    = ImLerp(colors[ImGuiCol_Header],       colors[ImGuiCol_TitleBgActive], 0.8);
    colors[ImGuiCol_TabHovered]             = colors[ImGuiCol_HeaderHovered];
    colors[ImGuiCol_TabActive]              = ImLerp(colors[ImGuiCol_HeaderActive], colors[ImGuiCol_TitleBgActive], 0.60);
    colors[ImGuiCol_TabUnfocused]           = ImLerp(colors[ImGuiCol_Tab],          colors[ImGuiCol_TitleBg], 0.8);
    colors[ImGuiCol_TabUnfocusedActive]     = ImLerp(colors[ImGuiCol_TabActive],    colors[ImGuiCol_TitleBg], 0.4);
    colors[ImGuiCol_DockingPreview]         = colors[ImGuiCol_HeaderActive] * ImVec4::from_floats(1.0, 1.0, 1.0, 0.70);
    colors[ImGuiCol_DockingEmptyBg]         = ImVec4::from_floats(0.20, 0.20, 0.20, 1.0);
    colors[ImGuiCol_PlotLines]              = ImVec4::from_floats(0.61, 0.61, 0.61, 1.0);
    colors[ImGuiCol_PlotLinesHovered]       = ImVec4::from_floats(1.0, 0.43, 0.35, 1.0);
    colors[ImGuiCol_PlotHistogram]          = ImVec4::from_floats(0.90, 0.70, 0.00, 1.0);
    colors[ImGuiCol_PlotHistogramHovered]   = ImVec4::from_floats(1.0, 0.60, 0.00, 1.0);
    colors[ImGuiCol_TableHeaderBg]          = ImVec4::from_floats(0.19, 0.19, 0.20, 1.0);
    colors[ImGuiCol_TableBorderStrong]      = ImVec4::from_floats(0.31, 0.31, 0.35, 1.0);   // Prefer using Alpha=1.0 here
    colors[ImGuiCol_TableBorderLight]       = ImVec4::from_floats(0.23, 0.23, 0.25, 1.0);   // Prefer using Alpha=1.0 here
    colors[ImGuiCol_TableRowBg]             = ImVec4::from_floats(0.00, 0.00, 0.00, 0.0);
    colors[ImGuiCol_TableRowBgAlt]          = ImVec4::from_floats(1.0, 1.0, 1.0, 0.060);
    colors[ImGuiCol_TextSelectedBg]         = ImVec4::from_floats(0.26, 0.59, 0.98, 0.350);
    colors[ImGuiCol_DragDropTarget]         = ImVec4::from_floats(1.0, 1.0, 0.00, 0.9);
    colors[ImGuiCol_NavHighlight]           = ImVec4::from_floats(0.26, 0.59, 0.98, 1.0);
    colors[ImGuiCol_NavWindowingHighlight]  = ImVec4::from_floats(1.0, 1.0, 1.0, 0.70);
    colors[ImGuiCol_NavWindowingDimBg]      = ImVec4::from_floats(0.80, 0.80, 0.80, 0.200);
    colors[ImGuiCol_ModalWindowDimBg]       = ImVec4::from_floats(0.80, 0.80, 0.80, 0.350);
}

pub fn StyleColorsClassic(dst: *mut ImGuiStyle)
{
    style: *mut ImGuiStyle = if is_not_null(dst) { dst } else { &mut  GetStyle() };
    colors: *mut ImVec4 = style.Colors;

    colors[ImGuiCol_Text]                   = ImVec4::from_floats(0.90, 0.90, 0.90, 1.0);
    colors[ImGuiCol_TextDisabled]           = ImVec4::from_floats(0.60, 0.60, 0.60, 1.0);
    colors[ImGuiCol_WindowBg]               = ImVec4::from_floats(0.00, 0.00, 0.00, 0.850);
    colors[ImGuiCol_ChildBg]                = ImVec4::from_floats(0.00, 0.00, 0.00, 0.0);
    colors[ImGuiCol_PopupBg]                = ImVec4::from_floats(0.11, 0.11, 0.14, 0.920);
    colors[ImGuiCol_Border]                 = ImVec4::from_floats(0.50, 0.50, 0.50, 0.5);
    colors[ImGuiCol_BorderShadow]           = ImVec4::from_floats(0.00, 0.00, 0.00, 0.0);
    colors[ImGuiCol_FrameBg]                = ImVec4::from_floats(0.43, 0.43, 0.43, 0.390);
    colors[ImGuiCol_FrameBgHovered]         = ImVec4::from_floats(0.47, 0.47, 0.69, 0.4);
    colors[ImGuiCol_FrameBgActive]          = ImVec4::from_floats(0.42, 0.41, 0.64, 0.690);
    colors[ImGuiCol_TitleBg]                = ImVec4::from_floats(0.27, 0.27, 0.54, 0.830);
    colors[ImGuiCol_TitleBgActive]          = ImVec4::from_floats(0.32, 0.32, 0.63, 0.870);
    colors[ImGuiCol_TitleBgCollapsed]       = ImVec4::from_floats(0.40, 0.40, 0.80, 0.200);
    colors[ImGuiCol_MenuBarBg]              = ImVec4::from_floats(0.40, 0.40, 0.55, 0.8);
    colors[ImGuiCol_ScrollbarBg]            = ImVec4::from_floats(0.20, 0.25, 0.3, 0.60);
    colors[ImGuiCol_ScrollbarGrab]          = ImVec4::from_floats(0.40, 0.40, 0.80, 0.3);
    colors[ImGuiCol_ScrollbarGrabHovered]   = ImVec4::from_floats(0.40, 0.40, 0.80, 0.4);
    colors[ImGuiCol_ScrollbarGrabActive]    = ImVec4::from_floats(0.41, 0.39, 0.80, 0.60);
    colors[ImGuiCol_CheckMark]              = ImVec4::from_floats(0.90, 0.90, 0.90, 0.5);
    colors[ImGuiCol_SliderGrab]             = ImVec4::from_floats(1.0, 1.0, 1.0, 0.3);
    colors[ImGuiCol_SliderGrabActive]       = ImVec4::from_floats(0.41, 0.39, 0.80, 0.60);
    colors[ImGuiCol_Button]                 = ImVec4::from_floats(0.35, 0.40, 0.61, 0.620);
    colors[ImGuiCol_ButtonHovered]          = ImVec4::from_floats(0.40, 0.48, 0.71, 0.790);
    colors[ImGuiCol_ButtonActive]           = ImVec4::from_floats(0.46, 0.54, 0.80, 1.0);
    colors[ImGuiCol_Header]                 = ImVec4::from_floats(0.40, 0.40, 0.90, 0.450);
    colors[ImGuiCol_HeaderHovered]          = ImVec4::from_floats(0.45, 0.45, 0.90, 0.8);
    colors[ImGuiCol_HeaderActive]           = ImVec4::from_floats(0.53, 0.53, 0.87, 0.8);
    colors[ImGuiCol_Separator]              = ImVec4::from_floats(0.50, 0.50, 0.50, 0.60);
    colors[ImGuiCol_SeparatorHovered]       = ImVec4::from_floats(0.60, 0.60, 0.70, 1.0);
    colors[ImGuiCol_SeparatorActive]        = ImVec4::from_floats(0.70, 0.70, 0.90, 1.0);
    colors[ImGuiCol_ResizeGrip]             = ImVec4::from_floats(1.0, 1.0, 1.0, 0.100);
    colors[ImGuiCol_ResizeGripHovered]      = ImVec4::from_floats(0.78, 0.82, 1.0, 0.60);
    colors[ImGuiCol_ResizeGripActive]       = ImVec4::from_floats(0.78, 0.82, 1.0, 0.9);
    colors[ImGuiCol_Tab]                    = ImLerp(colors[ImGuiCol_Header],       colors[ImGuiCol_TitleBgActive], 0.8);
    colors[ImGuiCol_TabHovered]             = colors[ImGuiCol_HeaderHovered];
    colors[ImGuiCol_TabActive]              = ImLerp(colors[ImGuiCol_HeaderActive], colors[ImGuiCol_TitleBgActive], 0.60);
    colors[ImGuiCol_TabUnfocused]           = ImLerp(colors[ImGuiCol_Tab],          colors[ImGuiCol_TitleBg], 0.8);
    colors[ImGuiCol_TabUnfocusedActive]     = ImLerp(colors[ImGuiCol_TabActive],    colors[ImGuiCol_TitleBg], 0.4);
    colors[ImGuiCol_DockingPreview]         = colors[ImGuiCol_Header] * ImVec4::from_floats(1.0, 1.0, 1.0, 0.70);
    colors[ImGuiCol_DockingEmptyBg]         = ImVec4::from_floats(0.20, 0.20, 0.20, 1.0);
    colors[ImGuiCol_PlotLines]              = ImVec4::from_floats(1.0, 1.0, 1.0, 1.0);
    colors[ImGuiCol_PlotLinesHovered]       = ImVec4::from_floats(0.90, 0.70, 0.00, 1.0);
    colors[ImGuiCol_PlotHistogram]          = ImVec4::from_floats(0.90, 0.70, 0.00, 1.0);
    colors[ImGuiCol_PlotHistogramHovered]   = ImVec4::from_floats(1.0, 0.60, 0.00, 1.0);
    colors[ImGuiCol_TableHeaderBg]          = ImVec4::from_floats(0.27, 0.27, 0.38, 1.0);
    colors[ImGuiCol_TableBorderStrong]      = ImVec4::from_floats(0.31, 0.31, 0.45, 1.0);   // Prefer using Alpha=1.0 here
    colors[ImGuiCol_TableBorderLight]       = ImVec4::from_floats(0.26, 0.26, 0.28, 1.0);   // Prefer using Alpha=1.0 here
    colors[ImGuiCol_TableRowBg]             = ImVec4::from_floats(0.00, 0.00, 0.00, 0.0);
    colors[ImGuiCol_TableRowBgAlt]          = ImVec4::from_floats(1.0, 1.0, 1.0, 0.070);
    colors[ImGuiCol_TextSelectedBg]         = ImVec4::from_floats(0.00, 0.00, 1.0, 0.350);
    colors[ImGuiCol_DragDropTarget]         = ImVec4::from_floats(1.0, 1.0, 0.00, 0.9);
    colors[ImGuiCol_NavHighlight]           = colors[ImGuiCol_HeaderHovered];
    colors[ImGuiCol_NavWindowingHighlight]  = ImVec4::from_floats(1.0, 1.0, 1.0, 0.70);
    colors[ImGuiCol_NavWindowingDimBg]      = ImVec4::from_floats(0.80, 0.80, 0.80, 0.200);
    colors[ImGuiCol_ModalWindowDimBg]       = ImVec4::from_floats(0.20, 0.20, 0.20, 0.350);
}

// Those light colors are better suited with a thicker font than the default one + FrameBorder
pub fn StyleColorsLight(dst: *mut ImGuiStyle)
{
    let style: *mut ImGuiStyle = if is_not_null(dst) { dst } else { &mut GetStyle() };
    let colors: *mut ImVec4 = style.Colors.as_mut_ptr();

    colors[ImGuiCol_Text]                   = ImVec4::from_floats(0.00, 0.00, 0.00, 1.0);
    colors[ImGuiCol_TextDisabled]           = ImVec4::from_floats(0.60, 0.60, 0.60, 1.0);
    colors[ImGuiCol_WindowBg]               = ImVec4::from_floats(0.94, 0.94, 0.94, 1.0);
    colors[ImGuiCol_ChildBg]                = ImVec4::from_floats(0.00, 0.00, 0.00, 0.0);
    colors[ImGuiCol_PopupBg]                = ImVec4::from_floats(1.0, 1.0, 1.0, 0.980);
    colors[ImGuiCol_Border]                 = ImVec4::from_floats(0.00, 0.00, 0.00, 0.3);
    colors[ImGuiCol_BorderShadow]           = ImVec4::from_floats(0.00, 0.00, 0.00, 0.0);
    colors[ImGuiCol_FrameBg]                = ImVec4::from_floats(1.0, 1.0, 1.0, 1.0);
    colors[ImGuiCol_FrameBgHovered]         = ImVec4::from_floats(0.26, 0.59, 0.98, 0.4);
    colors[ImGuiCol_FrameBgActive]          = ImVec4::from_floats(0.26, 0.59, 0.98, 0.670);
    colors[ImGuiCol_TitleBg]                = ImVec4::from_floats(0.96, 0.96, 0.96, 1.0);
    colors[ImGuiCol_TitleBgActive]          = ImVec4::from_floats(0.82, 0.82, 0.82, 1.0);
    colors[ImGuiCol_TitleBgCollapsed]       = ImVec4::from_floats(1.0, 1.0, 1.0, 0.510);
    colors[ImGuiCol_MenuBarBg]              = ImVec4::from_floats(0.86, 0.86, 0.86, 1.0);
    colors[ImGuiCol_ScrollbarBg]            = ImVec4::from_floats(0.98, 0.98, 0.98, 0.530);
    colors[ImGuiCol_ScrollbarGrab]          = ImVec4::from_floats(0.69, 0.69, 0.69, 0.8);
    colors[ImGuiCol_ScrollbarGrabHovered]   = ImVec4::from_floats(0.49, 0.49, 0.49, 0.8);
    colors[ImGuiCol_ScrollbarGrabActive]    = ImVec4::from_floats(0.49, 0.49, 0.49, 1.0);
    colors[ImGuiCol_CheckMark]              = ImVec4::from_floats(0.26, 0.59, 0.98, 1.0);
    colors[ImGuiCol_SliderGrab]             = ImVec4::from_floats(0.26, 0.59, 0.98, 0.780);
    colors[ImGuiCol_SliderGrabActive]       = ImVec4::from_floats(0.46, 0.54, 0.80, 0.60);
    colors[ImGuiCol_Button]                 = ImVec4::from_floats(0.26, 0.59, 0.98, 0.4);
    colors[ImGuiCol_ButtonHovered]          = ImVec4::from_floats(0.26, 0.59, 0.98, 1.0);
    colors[ImGuiCol_ButtonActive]           = ImVec4::from_floats(0.06, 0.53, 0.98, 1.0);
    colors[ImGuiCol_Header]                 = ImVec4::from_floats(0.26, 0.59, 0.98, 0.310);
    colors[ImGuiCol_HeaderHovered]          = ImVec4::from_floats(0.26, 0.59, 0.98, 0.8);
    colors[ImGuiCol_HeaderActive]           = ImVec4::from_floats(0.26, 0.59, 0.98, 1.0);
    colors[ImGuiCol_Separator]              = ImVec4::from_floats(0.39, 0.39, 0.39, 0.620);
    colors[ImGuiCol_SeparatorHovered]       = ImVec4::from_floats(0.14, 0.44, 0.80, 0.780);
    colors[ImGuiCol_SeparatorActive]        = ImVec4::from_floats(0.14, 0.44, 0.80, 1.0);
    colors[ImGuiCol_ResizeGrip]             = ImVec4::from_floats(0.35, 0.35, 0.35, 0.170);
    colors[ImGuiCol_ResizeGripHovered]      = ImVec4::from_floats(0.26, 0.59, 0.98, 0.670);
    colors[ImGuiCol_ResizeGripActive]       = ImVec4::from_floats(0.26, 0.59, 0.98, 0.950);
    colors[ImGuiCol_Tab]                    = ImLerp(colors[ImGuiCol_Header],       colors[ImGuiCol_TitleBgActive], 0.9);
    colors[ImGuiCol_TabHovered]             = colors[ImGuiCol_HeaderHovered];
    colors[ImGuiCol_TabActive]              = ImLerp(colors[ImGuiCol_HeaderActive], colors[ImGuiCol_TitleBgActive], 0.60);
    colors[ImGuiCol_TabUnfocused]           = ImLerp(colors[ImGuiCol_Tab],          colors[ImGuiCol_TitleBg], 0.8);
    colors[ImGuiCol_TabUnfocusedActive]     = ImLerp(colors[ImGuiCol_TabActive],    colors[ImGuiCol_TitleBg], 0.4);
    colors[ImGuiCol_DockingPreview]         = colors[ImGuiCol_Header] * ImVec4::from_floats(1.0, 1.0, 1.0, 0.70);
    colors[ImGuiCol_DockingEmptyBg]         = ImVec4::from_floats(0.20, 0.20, 0.20, 1.0);
    colors[ImGuiCol_PlotLines]              = ImVec4::from_floats(0.39, 0.39, 0.39, 1.0);
    colors[ImGuiCol_PlotLinesHovered]       = ImVec4::from_floats(1.0, 0.43, 0.35, 1.0);
    colors[ImGuiCol_PlotHistogram]          = ImVec4::from_floats(0.90, 0.70, 0.00, 1.0);
    colors[ImGuiCol_PlotHistogramHovered]   = ImVec4::from_floats(1.0, 0.45, 0.00, 1.0);
    colors[ImGuiCol_TableHeaderBg]          = ImVec4::from_floats(0.78, 0.87, 0.98, 1.0);
    colors[ImGuiCol_TableBorderStrong]      = ImVec4::from_floats(0.57, 0.57, 0.64, 1.0);   // Prefer using Alpha=1.0 here
    colors[ImGuiCol_TableBorderLight]       = ImVec4::from_floats(0.68, 0.68, 0.74, 1.0);   // Prefer using Alpha=1.0 here
    colors[ImGuiCol_TableRowBg]             = ImVec4::from_floats(0.00, 0.00, 0.00, 0.0);
    colors[ImGuiCol_TableRowBgAlt]          = ImVec4::from_floats(0.3, 0.3, 0.3, 0.090);
    colors[ImGuiCol_TextSelectedBg]         = ImVec4::from_floats(0.26, 0.59, 0.98, 0.350);
    colors[ImGuiCol_DragDropTarget]         = ImVec4::from_floats(0.26, 0.59, 0.98, 0.950);
    colors[ImGuiCol_NavHighlight]           = colors[ImGuiCol_HeaderHovered];
    colors[ImGuiCol_NavWindowingHighlight]  = ImVec4::from_floats(0.70, 0.70, 0.70, 0.70);
    colors[ImGuiCol_NavWindowingDimBg]      = ImVec4::from_floats(0.20, 0.20, 0.20, 0.200);
    colors[ImGuiCol_ModalWindowDimBg]       = ImVec4::from_floats(0.20, 0.20, 0.20, 0.350);
}
