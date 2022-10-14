#![allow(non_snake_case)]


use std::ffi::CStr;
use libc::{c_char, c_int, c_void, size_t};
use crate::data_type::ImGuiDataType;
use crate::draw_vert::ImDrawVert;
use crate::imgui::GImGui;
use crate::io::ImGuiIO;
use crate::stack_level_info::ImGuiStackLevelInfo;
use crate::stack_tool::ImGuiStackTool;
use crate::string_ops::ImFormatString;
use crate::style::ImGuiStyle;
use crate::type_defs::ImGuiID;
use crate::vec2::ImVec2;
use crate::vec4::ImVec4;

// [DEBUG] Stack tool: hooks called by GetID() family functions
// c_void DebugHookIdInfo(ImGuiID id, ImGuiDataType data_type, *const c_void data_id, *const c_void data_id_end)
pub unsafe fn DebugHookIdInfo(id: ImGuiID, data_type: ImGuiDataType, data_id: *const c_void, data_id_ned: *const c_void) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    let mut tool: *mut ImGuiStackTool = &mut g.DebugStackTool;

    // Step 0: stack query
    // This assume that the ID was computed with the current ID stack, which tends to be the case for our widget.
    if tool.StackLevel == -1 {
        tool.StackLevel += 1;
        tool.Results.resize(window.IDStack.len() + 1, ImGuiStackLevelInfo::default());
        // for (let n: c_int = 0; n < window.IDStack.Size + 1; n++)
        for n in 0..window.IDStack.len() + 1 {
            tool.Results[n].ID = if n < window.IDStack.len() {
                window.IDStack[n]
            } else { id };
        }
        return;
    }

    // Step 1+: query for individual level
    // IM_ASSERT(tool.StackLevel >= 0);
    if tool.StackLevel != window.IDStack.len() as c_int {
        return;
    }
    let mut info: *mut ImGuiStackLevelInfo = &mut tool.Results[tool.StackLevel];
    // IM_ASSERT(info.ID == id && info.QueryFrameCount > 0);

    match data_type {
        ImGuiDataType_S32 => {
            // let fmt_1 = format!("{}", data_id);
            // let cstr_fmt_1 = CStr::from_bytes_with_nul_unchecked(fmt_1.as_bytes());
            // ImFormatString(info.Desc.as_mut_ptr(), IM_ARRAYSIZE(info.Desc), cstr_fmt_1.as_ptr());
            todo!()
        },
        ImGuiDataType_String => {
            // let raw_str_1 = if data_id_end.is_null() == false { dat_id_end - data_id } else {
            //     libc::strlen(data_id);
            // };
            // let data_id_cstr: CStr = Cstr::from_ptr(data_id);
            // let data_id_str = data_id_cstr.to_str().unwrap();
            //
            // ImFormatString(info.Desc.as_mut_ptr(), IM_ARRAYSIZE(info.Desc), data_id);
            todo!()
        },
        ImGuiDataType_Pointer => {
            // ImFormatString(info.Desc.as_mut_ptr(), IM_ARRAYSIZE(info.Desc), "(void*)0x%p", data_id);
            todo!()
        },

        ImGuiDataType_ID => {
            if (info.Desc[0] != 0) { // PushOverrideID() is often used to avoid hashing twice, which would lead to 2 calls to DebugHookIdInfo(). We prioritize the first one.
                return;
            }
            // ImFormatString(info.Desc, IM_ARRAYSIZE(info.Desc), "0x%08X [override]", id);
            todo!()
        },

        _ => {
            todo!()
        }
    };
    info.QuerySuccess = true;
    info.DataType = data_type;
}



// Helper function to verify ABI compatibility between caller code and compiled version of Dear ImGui.
// Verify that the type sizes are matching between the calling file's compilation unit and imgui.cpp's compilation unit
// If this triggers you have an issue:
// - Most commonly: mismatched headers and compiled code version.
// - Or: mismatched configuration #define, compilation settings, packing pragma etc.
//   The configuration settings mentioned in imconfig.h must be set for all compilation units involved with Dear ImGui,
//   which is way it is required you put them in your imconfig file (and not just before including imgui.h).
//   Otherwise it is possible that different compilation units would see different structure layout
pub unsafe fn DebugCheckVersionAndDataLayout(version: *const c_char, sz_io: size_t, sz_style: size_t, sz_vec2: size_t, sz_vec4: size_t, sz_vert: size_t, sz_idx: size_t) -> bool
{
    // let mut error: bool =  false;
    // if (libc::strcmp(version, IMGUI_VERSION) != 0) { error = true; IM_ASSERT(libc::strcmp(version, IMGUI_VERSION) == 0 && "Mismatched version string!"); }
    // if (sz_io != sizeof(ImGuiIO)) { error = true; IM_ASSERT(sz_io == sizeof(ImGuiIO) && "Mismatched struct layout!"); }
    // if (sz_style != sizeof(ImGuiStyle)) { error = true; IM_ASSERT(sz_style == sizeof(ImGuiStyle) && "Mismatched struct layout!"); }
    // if (sz_vec2 != sizeof(ImVec2)) { error = true; IM_ASSERT(sz_vec2 == sizeof(ImVec2) && "Mismatched struct layout!"); }
    // if (sz_vec4 != sizeof(ImVec4)) { error = true; IM_ASSERT(sz_vec4 == sizeof(ImVec4) && "Mismatched struct layout!"); }
    // if (sz_vert != sizeof(ImDrawVert)) { error = true; IM_ASSERT(sz_vert == sizeof(ImDrawVert) && "Mismatched struct layout!"); }
    // if (sz_idx != sizeof) { error = true; IM_ASSERT(sz_idx == sizeof && "Mismatched struct layout!"); }
    // return !error;
    todo!()
}
