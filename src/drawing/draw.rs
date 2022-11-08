// dear imgui, v1.89 WIP
#![allow(non_upper_case_globals)]
// (drawing and font code)

use crate::color::{
    color_u32_from_rgba, ImGuiCol_Border, ImGuiCol_BorderShadow, ImGuiCol_Button,
    ImGuiCol_ButtonActive, ImGuiCol_ButtonHovered, ImGuiCol_CheckMark, ImGuiCol_ChildBg,
    ImGuiCol_DockingEmptyBg, ImGuiCol_DockingPreview, ImGuiCol_DragDropTarget, ImGuiCol_FrameBg,
    ImGuiCol_FrameBgActive, ImGuiCol_FrameBgHovered, ImGuiCol_Header, ImGuiCol_HeaderActive,
    ImGuiCol_HeaderHovered, ImGuiCol_MenuBarBg, ImGuiCol_ModalWindowDimBg, ImGuiCol_NavHighlight,
    ImGuiCol_NavWindowingDimBg, ImGuiCol_NavWindowingHighlight, ImGuiCol_PlotHistogram,
    ImGuiCol_PlotHistogramHovered, ImGuiCol_PlotLines, ImGuiCol_PlotLinesHovered, ImGuiCol_PopupBg,
    ImGuiCol_ResizeGrip, ImGuiCol_ResizeGripActive, ImGuiCol_ResizeGripHovered,
    ImGuiCol_ScrollbarBg, ImGuiCol_ScrollbarGrab, ImGuiCol_ScrollbarGrabActive,
    ImGuiCol_ScrollbarGrabHovered, ImGuiCol_Separator, ImGuiCol_SeparatorActive,
    ImGuiCol_SeparatorHovered, ImGuiCol_SliderGrab, ImGuiCol_SliderGrabActive, ImGuiCol_Tab,
    ImGuiCol_TabActive, ImGuiCol_TabHovered, ImGuiCol_TabUnfocused, ImGuiCol_TabUnfocusedActive,
    ImGuiCol_TableBorderLight, ImGuiCol_TableBorderStrong, ImGuiCol_TableHeaderBg,
    ImGuiCol_TableRowBg, ImGuiCol_TableRowBgAlt, ImGuiCol_Text, ImGuiCol_TextDisabled,
    ImGuiCol_TextSelectedBg, ImGuiCol_TitleBg, ImGuiCol_TitleBgActive, ImGuiCol_TitleBgCollapsed,
    ImGuiCol_WindowBg, IM_COL32_A_MASK, IM_COL32_A_SHIFT, IM_COL32_BLACK_TRANS, IM_COL32_B_SHIFT,
    IM_COL32_G_SHIFT, IM_COL32_R_SHIFT, IM_COL32_WHITE,
};
use crate::direction::{
    ImGuiDir, ImGuiDir_COUNT, ImGuiDir_Down, ImGuiDir_Left, ImGuiDir_None, ImGuiDir_Right,
    ImGuiDir_Up,
};
use crate::draw_flags::{
    ImDrawFlags, ImDrawFlags_RoundCornersBottomLeft, ImDrawFlags_RoundCornersBottomRight,
    ImDrawFlags_RoundCornersDefault_, ImDrawFlags_RoundCornersMask_, ImDrawFlags_RoundCornersNone,
    ImDrawFlags_RoundCornersTopLeft, ImDrawFlags_RoundCornersTopRight,
};
use crate::draw_list::ImDrawList;
use crate::draw_vert::ImDrawVert;
use crate::drawing::draw_cmd::ImDrawCmd;
use crate::font::ImFont;
use crate::font_atlas::ImFontAtlas;
use crate::font_atlas_custom_rect::ImFontAtlasCustomRect;
use crate::font_atlas_default_tex_data::{
    FONT_ATLAS_DEFAULT_TEX_DATA_PIXELS_as_const_char_ptr,
    FONT_ATLAS_DEFAULT_TEX_DATA_PIXELS_as_vec, FONT_ATLAS_DEFAULT_TEX_DATA_H,
    FONT_ATLAS_DEFAULT_TEX_DATA_PIXELS, FONT_ATLAS_DEFAULT_TEX_DATA_W,
};
use crate::font_atlas_flags::{
    ImFontAtlasFlags_NoBakedLines, ImFontAtlasFlags_NoMouseCursors,
    ImFontAtlasFlags_NoPowerOfTwoHeight,
};
use crate::font_build_dst_data::ImFontBuildDstData;
use crate::font_build_src_data::ImFontBuildSrcData;
use crate::font_config::ImFontConfig;
use crate::math_ops::{ImAcos, ImAcosX, ImClamp, ImLerp, ImMax, ImMin, ImMul, ImSqrt, ImSwap};
use crate::mouse_cursor::ImGuiMouseCursor_COUNT;
use crate::rect::ImRect;
use crate::style::ImguiStyle;
use crate::style_ops::{GetColorU32, GetStyle};
use crate::type_defs::ImWchar;
use crate::utils::{flag_clear, is_not_null};
use crate::vec2::ImVec2;
use crate::vec4::ImVec4;
use libc::{c_char, c_float, c_int, c_uchar, c_uint, c_ushort, c_void, size_t};
use std::ffi::CStr;
use std::mem::swap;
use std::ptr::null_mut;

// ImDrawCallback: Draw callbacks for advanced uses [configurable type: override in imconfig.h]
// NB: You most likely do NOT need to use draw callbacks just to create your own widget or customized UI rendering,
// you can poke into the draw list for that! Draw callback may be useful for example to:
//  A) Change your GPU render state,
//  B) render a complex 3D scene inside a UI element without an intermediate texture/render target, etc.
// The expected behavior from your rendering function is 'if (cmd.UserCallback != NULL) { cmd.UserCallback(parent_list, cmd); } else { RenderTriangles() }'
// If you want to override the signature of ImDrawCallback, you can simply use e.g. '#define ImDrawCallback MyDrawCallback' (in imconfig.h) + update rendering backend accordingly.
// #ifndef ImDrawCallback
// typedef c_void (*ImDrawCallback)(*const ImDrawList parent_list, *const ImDrawCmd cmd);
pub type ImDrawCallback = fn(parent_list: *const ImDrawList, cmd: *const ImDrawCmd);
