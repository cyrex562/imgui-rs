#![allow(non_snake_case)]

use core::ptr::null_mut;
use imgui_rs::imgui::GImGui;
use imgui_rs::input_source::{ImGuiInputSource_Mouse, ImGuiInputSource_Nav};
use imgui_rs::type_defs::ImGuiID;
use imgui_rs::window::ImGuiWindow;
use crate::imgui::GImGui;
use crate::input_source::{ImGuiInputSource_Mouse, ImGuiInputSource_Nav};
use crate::type_defs::ImGuiID;
use crate::window::ImGuiWindow;

// c_void SetActiveID(ImGuiID id, ImGuiWindow* window)
pub fn SetActiveID(id: ImGuiID, window: *mut ImGuiWindow) {
    let g = GImGui; // ImGuiContext& g = *GImGui;

    // While most behaved code would make an effort to not steal active id during window move/drag operations,
    // we at least need to be resilient to it. Cancelling the move is rather aggressive and users of 'master' branch
    // may prefer the weird ill-defined half working situation ('docking' did assert), so may need to rework that.
    if g.MovingWindow != null_mut() && g.ActiveId == g.Movingwindow.MoveId {
        IMGUI_DEBUG_LOG_ACTIVEID("SetActiveID() cancel MovingWindow\n");
        g.MovingWindow = null_mut();
    }

    // Set active id
    g.ActiveIdIsJustActivated = (g.ActiveId != id);
    if g.ActiveIdIsJustActivated {
        // IMGUI_DEBUG_LOG_ACTIVEID("SetActiveID() old:0x%08X (window \"%s\") -> new:0x%08X (window \"%s\")\n", g.ActiveId, g.ActiveIdWindow ? g.ActiveIdwindow.Name : "", id, window ? window.Name : "");
        g.ActiveIdTimer = 0f32;
        g.ActiveIdHasBeenPressedBefore = false;
        g.ActiveIdHasBeenEditedBefore = false;
        g.ActiveIdMouseButton = -1;
        if id != 0 {
            g.LastActiveId = id;
            g.LastActiveIdTimer = 0f32;
        }
    }
    g.ActiveId = id;
    g.ActiveIdAllowOverlap = false;
    g.ActiveIdNoClearOnFocusLoss = false;
    g.ActiveIdWindow = window;
    g.ActiveIdHasBeenEditedThisFrame = false;
    if id {
        g.ActiveIdIsAlive = id;
        g.ActiveIdSource = if g.NavActivateId == id || g.NavActivateInputId == id || g.NavJustMovedToId == id { ImGuiInputSource_Nav } else { ImGuiInputSource_Mouse };
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
pub fn ClearActiveID() {
    SetActiveID(0, null_mut()); // g.ActiveId = 0;
}

// c_void SetHoveredID(ImGuiID id)
pub fn SetHoveredID(id: ImGuiID) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.HoveredId = id;
    g.HoveredIdAllowOverlap = false;
    g.HoveredIdUsingMouseWheel = false;
    if id != 0 && g.HoveredIdPreviousFrame != id {
        g.HoveredIdTimer = 0f32;
        g.HoveredIdNotActiveTimer = 0f32;
    }
}

// ImGuiID GetHoveredID()
pub fn GetHoveredID() -> ImGuiID
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return if g.HoveredId { g.HoveredId } else { g.HoveredIdPreviousFrame };
}

// This is called by ItemAdd().
// Code not using ItemAdd() may need to call this manually otherwise ActiveId will be cleared. In IMGUI_VERSION_NUM < 18717 this was called by GetID().
// c_void KeepAliveID(ImGuiID id)
pub fn KeepAliveID(id: ImGuiID)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if g.ActiveId == id {
        g.ActiveIdIsAlive = id;
    }
    if g.ActiveIdPreviousFrame == id {
        g.ActiveIdPreviousFrameIsAlive = true;
    }
}