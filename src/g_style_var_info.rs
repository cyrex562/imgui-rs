#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use crate::data_type::ImGuiDataType_Float;
use crate::style::ImGuiStyle;
use crate::style_var::ImGuiStyleVar;
use crate::style_var_info::ImGuiStyleVarInfo;

pub const  GStyleVarInfo: [ImGuiStyleVarInfo;25] =
[
ImGuiStyleVarInfo::new( ImGuiDataType_Float, 1, IM_OFFSETOF(ImGuiStyle, Alpha) ),               // ImGuiStyleVar_Alpha
    ImGuiStyleVarInfo::new( ImGuiDataType_Float, 1, IM_OFFSETOF(ImGuiStyle, DisabledAlpha) ),       // ImGuiStyleVar_DisabledAlpha
    ImGuiStyleVarInfo::new( ImGuiDataType_Float, 2, IM_OFFSETOF(ImGuiStyle, WindowPadding) ),       // ImGuiStyleVar_WindowPadding
    ImGuiStyleVarInfo::new( ImGuiDataType_Float, 1, IM_OFFSETOF(ImGuiStyle, WindowRounding) ),      // ImGuiStyleVar_WindowRounding
    ImGuiStyleVarInfo::new( ImGuiDataType_Float, 1, IM_OFFSETOF(ImGuiStyle, WindowBorderSize) ),    // ImGuiStyleVar_WindowBorderSize
    ImGuiStyleVarInfo::new( ImGuiDataType_Float, 2, IM_OFFSETOF(ImGuiStyle, WindowMinSize) ),       // ImGuiStyleVar_WindowMinSize
    ImGuiStyleVarInfo::new( ImGuiDataType_Float, 2, IM_OFFSETOF(ImGuiStyle, WindowTitleAlign) ),    // ImGuiStyleVar_WindowTitleAlign
    ImGuiStyleVarInfo::new( ImGuiDataType_Float, 1, IM_OFFSETOF(ImGuiStyle, ChildRounding) ),       // ImGuiStyleVar_ChildRounding
    ImGuiStyleVarInfo::new( ImGuiDataType_Float, 1, IM_OFFSETOF(ImGuiStyle, ChildBorderSize) ),     // ImGuiStyleVar_ChildBorderSize
    ImGuiStyleVarInfo::new( ImGuiDataType_Float, 1, IM_OFFSETOF(ImGuiStyle, PopupRounding) ),       // ImGuiStyleVar_PopupRounding
    ImGuiStyleVarInfo::new( ImGuiDataType_Float, 1, IM_OFFSETOF(ImGuiStyle, PopupBorderSize) ),     // ImGuiStyleVar_PopupBorderSize
    ImGuiStyleVarInfo::new( ImGuiDataType_Float, 2, IM_OFFSETOF(ImGuiStyle, FramePadding) ),        // ImGuiStyleVar_FramePadding
    ImGuiStyleVarInfo::new( ImGuiDataType_Float, 1, IM_OFFSETOF(ImGuiStyle, FrameRounding) ),       // ImGuiStyleVar_FrameRounding
    ImGuiStyleVarInfo::new( ImGuiDataType_Float, 1, IM_OFFSETOF(ImGuiStyle, FrameBorderSize) ),     // ImGuiStyleVar_FrameBorderSize
    ImGuiStyleVarInfo::new( ImGuiDataType_Float, 2, IM_OFFSETOF(ImGuiStyle, ItemSpacing) ),         // ImGuiStyleVar_ItemSpacing
    ImGuiStyleVarInfo::new( ImGuiDataType_Float, 2, IM_OFFSETOF(ImGuiStyle, ItemInnerSpacing) ),    // ImGuiStyleVar_ItemInnerSpacing
    ImGuiStyleVarInfo::new( ImGuiDataType_Float, 1, IM_OFFSETOF(ImGuiStyle, IndentSpacing) ),       // ImGuiStyleVar_IndentSpacing
    ImGuiStyleVarInfo::new( ImGuiDataType_Float, 2, IM_OFFSETOF(ImGuiStyle, CellPadding) ),         // ImGuiStyleVar_CellPadding
    ImGuiStyleVarInfo::new( ImGuiDataType_Float, 1, IM_OFFSETOF(ImGuiStyle, ScrollbarSize) ),       // ImGuiStyleVar_ScrollbarSize
    ImGuiStyleVarInfo::new( ImGuiDataType_Float, 1, IM_OFFSETOF(ImGuiStyle, ScrollbarRounding) ),   // ImGuiStyleVar_ScrollbarRounding
    ImGuiStyleVarInfo::new( ImGuiDataType_Float, 1, IM_OFFSETOF(ImGuiStyle, GrabMinSize) ),         // ImGuiStyleVar_GrabMinSize
    ImGuiStyleVarInfo::new( ImGuiDataType_Float, 1, IM_OFFSETOF(ImGuiStyle, GrabRounding) ),        // ImGuiStyleVar_GrabRounding
    ImGuiStyleVarInfo::new( ImGuiDataType_Float, 1, IM_OFFSETOF(ImGuiStyle, TabRounding) ),         // ImGuiStyleVar_TabRounding
    ImGuiStyleVarInfo::new( ImGuiDataType_Float, 2, IM_OFFSETOF(ImGuiStyle, ButtonTextAlign) ),     // ImGuiStyleVar_ButtonTextAlign
    ImGuiStyleVarInfo::new( ImGuiDataType_Float, 2, IM_OFFSETOF(ImGuiStyle, SelectableTextAlign) ), // ImGuiStyleVar_SelectableTextAlign
];


// static *const ImGuiStyleVarInfo GetStyleVarInfo(ImGuiStyleVar idx)
pub fn GetStyleVarInfo(idx: ImGuiStyleVar) -> *const ImGuiStyleVarInfo {
// IM_ASSERT(idx >= 0 && idx < ImGuiStyleVar_COUNT);
// IM_ASSERT(IM_ARRAYSIZE(GStyleVarInfo) == ImGuiStyleVar_COUNT);
    return &GStyleVarInfo[idx];
}