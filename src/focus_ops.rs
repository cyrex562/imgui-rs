#![allow(non_snake_case)]

use std::ptr::null_mut;
use libc::c_int;
use crate::direction::{ImGuiDir_Down, ImGuiDir_None, ImGuiDir_Up};
use crate::GImGui;
use crate::key::{ImGuiKey, ImGuiKey_MouseLeft};
use crate::mouse_button::ImGuiMouseButton;
use crate::nav_move_flags::{ImGuiNavMoveFlags_FocusApi, ImGuiNavMoveFlags_Tabbing};
use crate::scroll_flags::{ImGuiScrollFlags, ImGuiScrollFlags_AlwaysCenterY, ImGuiScrollFlags_KeepVisibleEdgeX, ImGuiScrollFlags_KeepVisibleEdgeY, ImGuiScrollFlags_None};
use crate::type_defs::ImGuiID;
use crate::window_ops::WindowRectAbsToRel;

// inline ImGuiID          GetFocusedFocusScope()
pub unsafe fn GetFocusedFocusScope() -> ImGuiID {
    let g = GImGui; // ImGuiContext& g = *GImGui; 
    return g.NavFocusScopeId;
}                            // Focus scope which is actually active


// inline ImGuiID          GetFocusScope()                 {
pub unsafe fn GetFocusScope() -> ImGuiID {
    let g = GImGui; // ImGuiContext& g
    toto!()
}


// inline ImGuiKey         MouseButtonToKey(ImGuiMouseButton button)                   
pub fn MouseButtonToKey(button: ImGuiMouseButton) -> ImGuiKey {
// IM_ASSERT(button > = 0 & & button < ImGuiMouseButton_COUNT); 
    return ImGuiKey_MouseLeft + button;
}


// c_void PushFocusScope(id: ImGuiID)
pub unsafe fn PushFocusScope(id: ImGuiID) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    g.FocusScopeStack.push(window.DC.NavFocusScopeIdCurrent);
    window.DC.NavFocusScopeIdCurrent = id;
}

// c_void PopFocusScope()
pub unsafe fn PopFocusScope() {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    // IM_ASSERT(g.FocusScopeStack.Size > 0); // Too many PopFocusScope() ?
    window.DC.NavFocusScopeIdCurrent = g.FocusScopeStack.last().unwrap().clone();
    g.FocusScopeStack.pop_back();
}

// Note: this will likely be called ActivateItem() once we rework our Focus/Activation system!
// c_void SetKeyboardFocusHere(offset: c_int)
pub unsafe fn SetKeyboardFocusHere(offset: c_int) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    // IM_ASSERT(offset >= -1);    // -1 is allowed but not below
    IMGUI_DEBUG_LOG_ACTIVEID("SetKeyboardFocusHere(%d) in window \"%s\"\n", offset, window.Name);

    // It makes sense in the vast majority of cases to never interrupt a drag and drop.
    // When we refactor this function into ActivateItem() we may want to make this an option.
    // MovingWindow is protected from most user inputs using SetActiveIdUsingNavAndKeys(), but
    // is also automatically dropped in the event g.ActiveId is stolen.
    if (g.DragDropActive || g.MovingWindow != null_mut()) {
        IMGUI_DEBUG_LOG_ACTIVEID("SetKeyboardFocusHere() ignored while DragDropActive!\n");
        return;
    }

    SetNavWindow(window);

    let scroll_flags = if window.Appearing { ImGuiScrollFlags_KeepVisibleEdgeX | ImGuiScrollFlags_AlwaysCenterY } else { ImGuiScrollFlags_KeepVisibleEdgeX | ImGuiScrollFlags_KeepVisibleEdgeY };
    NavMoveRequestSubmit(ImGuiDir_None, if offset < 0 { ImGuiDir_Up } else { ImGuiDir_Down }, ImGuiNavMoveFlags_Tabbing | ImGuiNavMoveFlags_FocusApi, scroll_flags); // FIXME-NAV: Once we refactor tabbing, add LegacyApi flag to not activate non-inputable.
    if offset == -1 {
        NavMoveRequestResolveWithLastItem(&g.NavMoveResultLocal);
    } else {
        g.NavTabbingDir = 1;
        g.NavTabbingCounter = offset + 1;
    }
}

// c_void SetItemDefaultFocus()
pub unsafe fn SetItemDefaultFocus() {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if !window.Appearing {
        return;
    }
    if g.NavWindow != window.RootWindowForNav || (!g.NavInitRequest && g.NavInitResultId == 0) || g.NavLayer != window.DC.NavLayerCurrent {
        return;
    }

    g.NavInitRequest = false;
    g.NavInitResultId = g.LastItemData.ID;
    g.NavInitResultRectRel = WindowRectAbsToRel(window, &g.LastItemData.Rect);
    NavUpdateAnyRequestFlag();

    // Scroll could be done in NavInitRequestApplyResult() via a opt-in flag (we however don't want regular init requests to scroll)
    if !IsItemVisible() {
        ScrollToRectEx(window, &g.LastItemData.Rect, ImGuiScrollFlags_None);
    }
}
