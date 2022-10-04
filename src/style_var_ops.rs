#![allow(non_snake_case)]

use libc::{c_float, c_int, c_void};
use crate::data_type::ImGuiDataType_Float;
use crate::g_style_var_info::GetStyleVarInfo;
use crate::imgui::GImGui;
use crate::style_mod::ImGuiStyleMod;
use crate::style_var::ImGuiStyleVar;
use crate::vec2::ImVec2;

// c_void PushStyleVar(ImGuiStyleVar idx, val: c_float)
pub unsafe fn PushStyleVar(idx: ImGuiStyleVar, val: c_float) {
    let mut var_info = GetStyleVarInfo(idx);
    if var_info.Type == ImGuiDataType_Float && var_info.Count == 1 {
        let g = GImGui; // ImGuiContext& g = *GImGui;
        let mut pvar = var_info.GetVarPtr(&mut g.Style).clone();
        g.StyleVarStack.push(ImGuiStyleMod(idx, *pvar));
        *pvar = val;
        return;
    }
// IM_ASSERT(0 && "Called PushStyleVar() float variant but variable is not a float!");
}


// c_void PushStyleVar(ImGuiStyleVar idx, const val: &ImVec2)
pub unsafe fn PushStyleVar2(idx: ImGuiStyleVar, val: &ImVec2) {
    let mut var_info = GetStyleVarInfo(idx);
    if var_info.Type == ImGuiDataType_Float && var_info.Count == 2 {
        let g = GImGui; // ImGuiContext& g = *GImGui;
        ImVec2 * pvar = var_info.GetVarPtr(&mut g.Style);
        g.StyleVarStack.push(ImGuiStyleMod(idx, *pvar));
        *pvar = val;
        return;
    }
    // IM_ASSERT(0 && "Called PushStyleVar() ImVec2 variant but variable is not a ImVec2!");
}


// c_void PopStyleVar(count: c_int)
pub fn PopStyleVar(mut count: c_int) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if g.StyleVarStack.Size < count {
        // IM_ASSERT_USER_ERROR(g.StyleVarStack.Size > count, "Calling PopStyleVar() too many times: stack underflow.");
        count = g.StyleVarStack.Size;
    }
    while count > 0 {
        // We avoid a generic memcpy(data, &backup.Backup.., GDataTypeSize[info.Type] * info.Count), the overhead in Debug is not worth it.
        let backup = g.StyleVarStack.last().unwrap();
        let mut info = GetStyleVarInfo(backup.VarIdx);
        let data: *mut c_void = info.GetVarPtr(&mut g.Style);
        if info.Type == ImGuiDataType_Float && info.Count == 1 { (data)[0] = backup.BackupFloat[0]; } else if info.Type == ImGuiDataType_Float && info.Count == 2 {
            (data)[0] = backup.BackupFloat[0];
            (data)[1] = backup.BackupFloat[1];
        }
        g.StyleVarStack.pop().unwrap();
        count -= 1;
    }
}
