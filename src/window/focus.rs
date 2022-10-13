use std::ptr::null_mut;
use libc::c_int;
use crate::dock_node::ImGuiDockNode;
use crate::GImGui;
use crate::id_ops::ClearActiveID;
use crate::nav_layer::ImGuiNavLayer_Main;
use crate::utils::{flag_set, is_not_null};
use crate::window::{ImGuiWindow, ops};
use crate::window::window_flags::{ImGuiWindowFlags_ChildWindow, ImGuiWindowFlags_NoBringToFrontOnFocus, ImGuiWindowFlags_NoMouseInputs, ImGuiWindowFlags_NoNavInputs};

// Moving window to front of display and set focus (which happens to be back of our sorted list)
pub unsafe fn FocusWindow(window: *mut ImGuiWindow)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    if g.NavWindow != window
    {
        SetNavWindow(window);
        if is_not_null(window) && g.NavDisableMouseHover {
            g.NavMousePosDirty = true;
        }
        g.NavId = if is_not_null(window) { window.NavLastIds[0] } else { 0 }; // Restore NavId
        g.NavLayer = ImGuiNavLayer_Main;
        g.NavFocusScopeId = 0;
        g.NavIdIsAlive = false;
    }

    // Close popups if any
    ClosePopupsOverWindow(window, false);

    // Move the root window to the top of the pile
    // IM_ASSERT(window == NULL || window.RootWindowDockTree != NULL);
    let mut focus_front_window: *mut ImGuiWindow =  if is_not_null(window) { window.RootWindow } else { null_mut() };
    let mut display_front_window: *mut ImGuiWindow =  if is_not_null(window) { window.RootWindowDockTree } else { null_mut() };
    let dock_node = if is_not_null(window) { window.DockNode } else { null_mut() };
    let mut active_id_window_is_dock_node_host: bool =  (is_not_null(g.ActiveIdWindow) && is_not_null(dock_node) && dock_node.HostWindow == g.ActiveIdWindow);

    // Steal active widgets. Some of the cases it triggers includes:
    // - Focus a window while an InputText in another window is active, if focus happens before the old InputText can run.
    // - When using Nav to activate menu items (due to timing of activating on press->new window appears->losing ActiveId)
    // - Using dock host items (tab, collapse button) can trigger this before we redirect the ActiveIdWindow toward the child window.
    if g.ActiveId != 0 && is_not_null(g.ActiveIdWindow) && g.ActiveIdwindow.RootWindow != focus_front_window {
        if !g.ActiveIdNoClearOnFocusLoss && !active_id_window_is_dock_node_host {
            ClearActiveID();
        }
    }

    // Passing NULL allow to disable keyboard focus
    if !window {
        return;
    }
    window.LastFrameJustFocused = g.FrameCount;

    // Select in dock node
    if is_not_null(dock_node) && is_not_null(dock_node.TabBar) {
        dock_node.TabBar.SelectedTabId = window.TabId;
        dock_node.TabBar.NextSelectedTabId = window.TabId;
    }

    // Bring to front
    ops::BringWindowToFocusFront(focus_front_window);
    if ((window.Flags | focus_front_window.Flags | display_front_window.Flags) & ImGuiWindowFlags_NoBringToFrontOnFocus) == 0 {
        ops::BringWindowToDisplayFront(display_front_window);
    }
}



pub unsafe fn FocusTopMostWindowUnderOne(mut under_this_window: *mut ImGuiWindow, ignore_window: *mut ImGuiWindow)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut start_idx: c_int = g.WindowsFocusOrder.Size - 1;
    if under_this_window != null_mut()
    {
        // Aim at root window behind us, if we are in a child window that's our own root (see #4640)
        let mut offset: c_int = -1;
        while flag_set(under_this_window.Flags & ImGuiWindowFlags_ChildWindow)
        {
            under_this_window = under_this_window.ParentWindow;
            offset = 0;
        }
        start_idx = FindWindowFocusIndex(under_this_window) + offset;
    }
    // for (let i: c_int = start_idx; i >= 0; i--)
    for i in start_idx .. 0
    {
        // We may later decide to test for different NoXXXInputs based on the active navigation input (mouse vs nav) but that may feel more confusing to the user.
        let mut window: *mut ImGuiWindow =  g.WindowsFocusOrder[i];
        // IM_ASSERT(window == window.RootWindow);
        if window != ignore_window && window.WasActive {
            if (window.Flags & (ImGuiWindowFlags_NoMouseInputs | ImGuiWindowFlags_NoNavInputs)) != (ImGuiWindowFlags_NoMouseInputs | ImGuiWindowFlags_NoNavInputs) {
                // FIXME-DOCK: This is failing (lagging by one frame) for docked windows.
                // If A and B are docked into window and B disappear, at the NewFrame() call site window.NavLastChildNavWindow will still point to B.
                // We might leverage the tab order implicitly stored in window.DockNodeAsHost->TabBar (essentially the 'most_recently_selected_tab' code in tab bar will do that but on next update)
                // to tell which is the "previous" window. Or we may leverage 'LastFrameFocused/LastFrameJustFocused' and have this function handle child window itself?
                let mut focus_window: *mut ImGuiWindow = NavRestoreLastChildNavWindow(window);
                FocusWindow(focus_window);
                return;
            }
        }
    }
    FocusWindow(null_mut());
}

