#![allow(non_snake_case)]

use std::ffi::CString;
use libc::{c_char, c_float, c_int};
use crate::color::{IM_COL32_A_MASK, IM_COL32_A_SHIFT, ImGuiCol};
use crate::color_mod::ImGuiColorMod;
use crate::color_ops::{ColorConvertFloat4ToU32, ColorConvertU32ToFloat4};
use crate::style::ImGuiStyle;
use crate::imgui::GImGui;
use crate::vec4::ImVec4;

// ImGuiStyle& GetStyle()
pub fn GetStyle() -> &mut ImGuiStyle {
// IM_ASSERT(GImGui != NULL && "No current context. Did you call CreateContext() and SetCurrentContext() ?");
    return GimGui.Style;
}


// u32 GetColorU32(ImGuiCol idx, alpha_mul: c_float)
pub unsafe fn GetColorU32(idx: ImGuiCol, alpha_mul: f32) -> u32 {
    let g = GImGui;
    let style = &mut g.Style;
    let mut c = style.Colors[idx];
    c.w *= style.Alpha.clone() * alpha_mul;
    return ColorConvertFloat4ToU32(c);
}

// u32 GetColorU32(const ImVec4& col)
pub unsafe fn GetColorU32FromImVec4(col: &ImVec4) -> u32 {
    let g = GImGui;
    let style = &mut g.Style;
    let mut c = col.clone();
    c.w *= style.Alpha.clone();
    return ColorConvertFloat4ToU32(&c);
}

// const ImVec4& GetStyleColorVec4(ImGuiCol idx)
pub unsafe fn GetStyleColorVec4(idx: ImGuiCol) -> &ImVec4 {
    let g = GImGui;
    let style = &mut g.Style;
    return style.Colors[idx];
}

pub unsafe fn GetStyleColorU32(idx: ImGuiCol) -> u32 {
    let g = GImGui;
    let style = &mut g.Style;
    let col = style.Colors[idx];
    GetColorU32FromImVec4(col)
}


// u32 GetColorU32(u32 col)
pub unsafe fn GetColorU32FromU32(col: u32) -> u32 {
    let g = GImGui;
    let style = &mut g.Style;
    if style.Alpha >= 1f32 {
        return col;
    }
    let a = (col & IM_COL32_A_MASK) >> IM_COL32_A_SHIFT;
    a = (a * style.Alpha.clone()); // We don't need to clamp 0..255 because Style.Alpha is in 0..1 range.
    return (col.clone() & !IM_COL32_A_MASK) | (a << IM_COL32_A_SHIFT);
}

// FIXME: This may incur a round-trip (if the end user got their data from a float4) but eventually we aim to store the in-flight colors as ImU32
// c_void PushStyleColor(ImGuiCol idx, u32 col)
pub unsafe fn PushStyleColor(idx: ImGuiCol, col: u32) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut backup: ImGuiColorMod = ImGuiColorMod::default();
    backup.Col = idx;
    backup.BackupValue = g.Style.Colors[idx.clone()];
    g.ColorStack.push(backup);
    g.Style.Colors[idx.clone()] = ColorConvertU32ToFloat4(col);
}

// c_void PushStyleColor(ImGuiCol idx, const ImVec4& col)
pub unsafe fn PushStyleColor2(idx: ImGuiCol, col: &ImVec4) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut backup: ImGuiColorMod = ImGuiColorMod::default();
    backup.Col = idx;
    backup.BackupValue = g.Style.Colors[idx.clone()];
    g.ColorStack.push(backup);
    g.Style.Colors[idx.clone()] = col;
}

// c_void PopStyleColor(count: c_int)
pub unsafe fn PopStyleColor(mut count: c_int) {
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
// *const char GetStyleColorName(ImGuiCol idx)
pub unsafe fn GetStyleColorName(idx: ImGuiCol) -> *const c_char {
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
