#![allow(non_snake_case)]

use crate::data_type::IM_GUI_DATA_TYPE_FLOAT;
use crate::core::g_style_var_info::GetStyleVarInfo;
use crate::imgui::GImGui;
use crate::style_mod::ImGuiStyleMod;
use crate::style_var::ImGuiStyleVar;
use crate::core::vec2::ImVec2;
use libc::{c_float, c_int, c_void};

// c_void PushStyleVar(ImGuiStyleVar idx, c_float val)
pub unsafe fn PushStyleVarFloat(idx: ImGuiStyleVar, val: c_float) {
    let mut var_info = GetStyleVarInfo(idx);
    if var_info.Type == IM_GUI_DATA_TYPE_FLOAT && var_info.Count == 1 {
        let g = GImGui; // ImGuiContext& g = *GImGui;
        let mut pvar = var_info.GetVarPtr(&mut g.style).clone();
        g.styleVarStack.push(ImGuiStyleMod(idx, *pvar));
        *pvar = val;
        return;
    }
    // IM_ASSERT(0 && "Called PushStyleVar() float variant but variable is not a float!");
}

// c_void PushStyleVar(ImGuiStyleVar idx, const val: &mut ImVec2)
pub unsafe fn PushStyleVarVec2(idx: ImGuiStyleVar, val: &ImVec2) {
    let mut var_info = GetStyleVarInfo(idx);
    if var_info.Type == IM_GUI_DATA_TYPE_FLOAT && var_info.Count == 2 {
        let g = GImGui; // ImGuiContext& g = *GImGui;
        ImVec2 * pvar = var_info.GetVarPtr(&mut g.style);
        g.styleVarStack.push(ImGuiStyleMod(idx, *pvar));
        *pvar = val;
        return;
    }
    // IM_ASSERT(0 && "Called PushStyleVar() variant: ImVec2 but variable is not a ImVec2!");
}

// c_void PopStyleVar(count: c_int)
pub unsafe fn PopStyleVarInt(mut count: c_int) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if g.styleVarStack.Size < count {
        // IM_ASSERT_USER_ERROR(g.styleVarStack.Size > count, "Calling PopStyleVar() too many times: stack underflow.");
        count = g.styleVarStack.Size;
    }
    while count > 0 {
        // We avoid a generic memcpy(data, &backup.Backup.., GDataTypeSize[info.Type] * info.Count), the overhead in Debug is not worth it.
        let backup = g.styleVarStack.last().unwrap();
        let mut info = GetStyleVarInfo(backup.VarIdx);
        let data: *mut c_void = info.GetVarPtr(&mut g.style);
        if info.Type == IM_GUI_DATA_TYPE_FLOAT && info.Count == 1 {
            (data)[0] = backup.BackupFloat[0];
        } else if info.Type == IM_GUI_DATA_TYPE_FLOAT && info.Count == 2 {
            (data)[0] = backup.BackupFloat[0];
            (data)[1] = backup.BackupFloat[1];
        }
        g.styleVarStack.pop().unwrap();
        count -= 1;
    }
}
