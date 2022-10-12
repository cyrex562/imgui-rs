#![allow(non_snake_case)]


use std::ffi::CStr;
use libc::{c_int, c_void};
use crate::data_type::ImGuiDataType;
use crate::imgui::GImGui;
use crate::stack_level_info::ImGuiStackLevelInfo;
use crate::stack_tool::ImGuiStackTool;
use crate::string_ops::ImFormatString;
use crate::type_defs::ImGuiID;

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
