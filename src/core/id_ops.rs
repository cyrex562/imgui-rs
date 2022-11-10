#![allow(non_snake_case)]

use crate::data_type::{IM_GUI_DATA_TYPE_ID, IM_GUI_DATA_TYPE_STRING};
use crate::debug_ops::DebugHookIdInfo;
use crate::imgui::GImGui;
use crate::io::input_source::{ImGuiInputSource_Mouse, ImGuiInputSource_Nav};
use crate::core::type_defs::ImguiHandle;
use crate::core::utils::is_not_null;
use crate::window::ImguiWindow;
use crate::hash_string;
use core::ptr::null_mut;
use imgui_rs::imgui::GImGui;
use imgui_rs::input_source::{ImGuiInputSource_Mouse, ImGuiInputSource_Nav};
use imgui_rs::type_defs::ImguiHandle;
use imgui_rs::window::ImGuiWindow;
use libc::{c_char, c_int, c_void};
use std::ptr::null;
use crate::core::context::AppContext;

// c_void SetActiveID(ImguiHandle id, window: &mut ImGuiWindow)
pub fn SetActiveID(g: &mut AppContext, id: ImguiHandle, window: Option<&mut ImguiWindow>) {
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
pub fn ClearActiveID(g: &mut AppContext) {
    SetActiveID(g, 0, None);
}

// c_void SetHoveredID(ImguiHandle id)
pub unsafe fn SetHoveredID(id: ImguiHandle) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.HoveredId = id;
    g.HoveredIdAllowOverlap = false;
    g.HoveredIdUsingMouseWheel = false;
    if id != 0 && g.HoveredIdPreviousFrame != id {
        g.HoveredIdTimer = 0.0;
        g.HoveredIdNotActiveTimer = 0.0;
    }
}

// ImguiHandle GetHoveredID()
pub unsafe fn GetHoveredID() -> ImguiHandle {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return if g.HoveredId {
        g.HoveredId
    } else {
        g.HoveredIdPreviousFrame
    };
}

// Code not using ItemAdd() may need to call this manually otherwise ActiveId will be
// cleared. In IMGUI_VERSION_NUM < 18717 this was called by GetID().
pub fn KeepAliveID(g: &mut AppContext, id: ImguiHandle) {
    if g.ActiveId == id {
        g.ActiveIdIsAlive = id;
    }
    if g.ActiveIdPreviousFrame == id {
        g.ActiveIdPreviousFrameIsAlive = true;
    }
}

pub fn push_str_id(g: &mut AppContext, str_id: &String) {
    let mut window = g.current_window_mut().unwrap();
    let mut id = window.id_by_string(g, str_id);
    window.id_stack.push(id);
}

pub fn push_int_id(g: &mut AppContext, int_id: c_int) {
    let mut window = g.current_window_mut().unwrap();
    let mut id: ImguiHandle = window.id_by_int(g, int_id);
    window.id_stack.push(id);
}

// Push a given id value ignoring the ID stack as a seed.
pub fn PushOverrideID(g: &mut AppContext, id: ImguiHandle) {
    let mut window = g.current_window_mut().unwrap();
    if g.DebugHookIdInfo == id {
        DebugHookIdInfo(g, id, IM_GUI_DATA_TYPE_ID, None);
    }
    window.id_stack.push(id);
}

// Helper to avoid a common series of PushOverrideID -> GetID() -> PopID() call
// (note that when using this pattern, TestEngine's "Stack Tool" will tend to not display the intermediate stack level.
//  for that to work we would need to do PushOverrideID() -> ItemAdd() -> PopID() which would alter widget code a little more)
pub fn GetIDWithSeed(arg: &str, seed: ImguiHandle) -> ImguiHandle {
    let mut id: ImguiHandle = hash_string(arg, seed as u32);
    let g = GImGui; // ImGuiContext& g = *GImGui; if (g.DebugHookIdInfo == id)
    DebugHookIdInfo(, id, IM_GUI_DATA_TYPE_STRING, str);
    return id;
}

pub fn pop_win_id_from_stack(g: &mut AppContext) {
    let mut window = g.current_window_mut().unwrap();
    // IM_ASSERT(window.id_stack.Size > 1); // Too many PopID(), or could be popping in a wrong/different window?
    window.id_stack.pop_back();
}

pub fn id_from_str(str_id: &str) -> ImguiHandle {
    let g = GImGui; // ImGuiContext& g = *GImGui
    let mut window  = g.current_window_mut().unwrap();
    return window.id_from_str(str_id);
}

// pub unsafe fn GetID2(str_id_begin: &str) -> ImguiHandle {
//     let g = GImGui; // ImGuiContext& g = *GImGui
//     let mut window  = g.current_window_mut().unwrap();
//     return window.id_from_str(str_id_begin);
// }

pub fn id_from_void_ptr(ptr_id: *const c_void) -> ImguiHandle {
    let g = GImGui; // ImGuiContext& g = *GImGui
    let mut window  = g.current_window_mut().unwrap();
    return window.GetID2(ptr_id);
}
