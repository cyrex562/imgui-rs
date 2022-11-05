#![allow(non_snake_case)]

use crate::data_type::{ImGuiDataType_ID, ImGuiDataType_String};
use crate::debug_ops::DebugHookIdInfo;
use crate::imgui::GImGui;
use crate::input_source::{ImGuiInputSource_Mouse, ImGuiInputSource_Nav};
use crate::type_defs::ImGuiID;
use crate::utils::is_not_null;
use crate::window::ImGuiWindow;
use crate::ImHashStr;
use core::ptr::null_mut;
use imgui_rs::imgui::GImGui;
use imgui_rs::input_source::{ImGuiInputSource_Mouse, ImGuiInputSource_Nav};
use imgui_rs::type_defs::ImGuiID;
use imgui_rs::window::ImGuiWindow;
use libc::{c_char, c_int, c_void};
use std::ptr::null;

// c_void SetActiveID(ImGuiID id, window: &mut ImGuiWindow)
pub unsafe fn SetActiveID(id: ImGuiID, window: &mut ImGuiWindow) {
    let g = GImGui; // ImGuiContext& g = *GImGui;

    // While most behaved code would make an effort to not steal active id during window move/drag operations,
    // we at least need to be resilient to it. Cancelling the move is rather aggressive and users of 'master' branch
    // may prefer the weird ill-defined half working situation ('docking' did assert), so may need to rework that.
    if g.MovingWindow.is_some() && g.ActiveId == g.Movingwindow.unwrap().MoveId {
        // IMGUI_DEBUG_LOG_ACTIVEID("SetActiveID() cancel MovingWindow\n");
        g.MovingWindow = None;
    }

    // Set active id
    g.ActiveIdIsJustActivated = (g.ActiveId != id);
    if g.ActiveIdIsJustActivated {
        // IMGUI_DEBUG_LOG_ACTIVEID("SetActiveID() old:0x{} (window \"{}\") -> new:0x{} (window \"{}\")\n", g.ActiveId, g.ActiveIdWindow ? g.ActiveIdwindow.Name : "", id, window ? window.Name : "");
        g.ActiveIdTimer = 0.0;
        g.ActiveIdHasBeenPressedBefore = false;
        g.ActiveIdHasBeenEditedBefore = false;
        g.ActiveIdMouseButton = -1;
        if id != 0 {
            g.LastActiveId = id;
            g.LastActiveIdTimer = 0.0;
        }
    }
    g.ActiveId = id;
    g.ActiveIdAllowOverlap = false;
    g.ActiveIdNoClearOnFocusLoss = false;
    g.ActiveIdWindow = window;
    g.ActiveIdHasBeenEditedThisFrame = false;
    if id {
        g.ActiveIdIsAlive = id;
        g.ActiveIdSource =
            if g.NavActivateId == id || g.NavActivateInputId == id || g.NavJustMovedToId == id {
                ImGuiInputSource_Nav
            } else {
                ImGuiInputSource_Mouse
            };
    }

    // Clear declaration of inputs claimed by the widget
    // (Please note that this is WIP and not all keys/inputs are thoroughly declared by all widgets yet)
    g.ActiveIdUsingNavDirMask = 0x00;
    g.ActiveIdUsingKeyInputMask.ClearAllBits();
    // #ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
    //     g.ActiveIdUsingNavInputMask = 0x00;
    // #endif
}

// c_void ClearActiveID()
pub unsafe fn ClearActiveID() {
    SetActiveID(0, null_mut()); // g.ActiveId = 0;
}

// c_void SetHoveredID(ImGuiID id)
pub unsafe fn SetHoveredID(id: ImGuiID) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.HoveredId = id;
    g.HoveredIdAllowOverlap = false;
    g.HoveredIdUsingMouseWheel = false;
    if id != 0 && g.HoveredIdPreviousFrame != id {
        g.HoveredIdTimer = 0.0;
        g.HoveredIdNotActiveTimer = 0.0;
    }
}

// ImGuiID GetHoveredID()
pub unsafe fn GetHoveredID() -> ImGuiID {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return if g.HoveredId {
        g.HoveredId
    } else {
        g.HoveredIdPreviousFrame
    };
}

// This is called by ItemAdd().
// Code not using ItemAdd() may need to call this manually otherwise ActiveId will be cleared. In IMGUI_VERSION_NUM < 18717 this was called by GetID().
// c_void KeepAliveID(ImGuiID id)
pub unsafe fn KeepAliveID(id: ImGuiID) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if g.ActiveId == id {
        g.ActiveIdIsAlive = id;
    }
    if g.ActiveIdPreviousFrame == id {
        g.ActiveIdPreviousFrameIsAlive = true;
    }
}

// pub unsafe fn PushID(str_id: &str) {
//     let g = GImGui; // ImGuiContext& g = *GImGui;
//     let mut window = g.CurrentWindow;
//     let mut id: ImGuiID = window.id_from_str(str_id);
//     window.IDStack.push(id);
// }

pub unsafe fn push_str_id(str_id_begin: &str) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = &g.CurrentWindow;
    let mut id: ImGuiID = window.id_from_str(str_id_begin);
    window.IDStack.push(id);
}

pub unsafe fn push_void_ptr_id(ptr_id: *const c_void) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = &g.CurrentWindow;
    let mut id: ImGuiID = window.GetID2(ptr_id);
    window.IDStack.push(id);
}

pub unsafe fn push_int_id(int_id: c_int) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = &g.CurrentWindow;
    let mut id: ImGuiID = window.GetID3(int_id);
    window.IDStack.push(id);
}

// Push a given id value ignoring the ID stack as a seed.
pub unsafe fn PushOverrideID(id: ImGuiID) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = &g.CurrentWindow;
    if g.DebugHookIdInfo == id {
        DebugHookIdInfo(id, ImGuiDataType_ID, None);
    }
    window.IDStack.push(id);
}

// Helper to avoid a common series of PushOverrideID -> GetID() -> PopID() call
// (note that when using this pattern, TestEngine's "Stack Tool" will tend to not display the intermediate stack level.
//  for that to work we would need to do PushOverrideID() -> ItemAdd() -> PopID() which would alter widget code a little more)
pub unsafe fn GetIDWithSeed(arg: &str, seed: ImGuiID) -> ImGuiID {
    let mut id: ImGuiID = ImHashStr(arg, arg.len(), seed as u32);
    let g = GImGui; // ImGuiContext& g = *GImGui; if (g.DebugHookIdInfo == id)
    DebugHookIdInfo(id, ImGuiDataType_String, str);
    return id;
}

pub unsafe fn PopID() {
    let mut window: &mut ImGuiWindow = GimGui.CurrentWindow;
    // IM_ASSERT(window.IDStack.Size > 1); // Too many PopID(), or could be popping in a wrong/different window?
    window.IDStack.pop_back();
}

pub unsafe fn id_from_str(str_id: &str) -> ImGuiID {
    let g = GImGui; // ImGuiContext& g = *GImGui
    let mut window: &mut ImGuiWindow = g.CurrentWindow;
    return window.id_from_str(str_id);
}

// pub unsafe fn GetID2(str_id_begin: &str) -> ImGuiID {
//     let g = GImGui; // ImGuiContext& g = *GImGui
//     let mut window: &mut ImGuiWindow = g.CurrentWindow;
//     return window.id_from_str(str_id_begin);
// }

pub unsafe fn id_from_void_ptr(ptr_id: *const c_void) -> ImGuiID {
    let g = GImGui; // ImGuiContext& g = *GImGui
    let mut window: &mut ImGuiWindow = g.CurrentWindow;
    return window.GetID2(ptr_id);
}
