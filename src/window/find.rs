use std::ptr::null_mut;
use libc::{c_char, c_float, c_int};
use crate::{GImGui, ImGuiViewport, ImHashStr};
use crate::rect::ImRect;
use crate::type_defs::ImGuiID;
use crate::utils::{flag_clear, is_not_null};
use crate::vec2::ImVec2;
use crate::window::{ImGuiWindow, ops};
use crate::window::ops::GetWindowDisplayLayer;
use crate::window::window_flags::{ImGuiWindowFlags_AlwaysAutoResize, ImGuiWindowFlags_ChildWindow, ImGuiWindowFlags_Modal, ImGuiWindowFlags_NoMouseInputs, ImGuiWindowFlags_NoResize};

// static FindFrontMostVisibleChildWindow: *mut ImGuiWindow(window: *mut ImGuiWindow)
pub fn FindFrontMostVisibleChildWindow(window: *mut ImGuiWindow) -> *mut ImGuiWindow {
    // for (let n: c_int = window.DC.ChildWindows.Size - 1; n >= 0; n--)
    for n in window.DC.ChildWindows.len() - 1..0 {
        if ops::IsWindowActiveAndVisible(window.DC.ChildWindows[n]) {
            return FindFrontMostVisibleChildWindow(window.DC.ChildWindows[n]);
        }
    }
    return window;
}

// FindBottomMostVisibleWindowWithinBeginStack: *mut ImGuiWindow(parent_window: *mut ImGuiWindow)
pub unsafe fn FindBottomMostVisibleWindowWithBeginStack(
    parent_window: *mut ImGuiWindow,
) -> *mut ImGuiWindow {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut bottom_most_visible_window: *mut ImGuiWindow = parent_window;
    // for (let i: c_int = FindWindowDisplayIndex(parent_window); i >= 0; i--)
    for i in FindWindowDisplayIndex(parent_window)..0 {
        let mut window: *mut ImGuiWindow = g.Windows[i];
        if window.Flags & ImGuiWindowFlags_ChildWindow {
            continue;
        }
        if !IsWindowWithinBeginStackOf(window, parent_window) {
            break;
        }
        if ops::IsWindowActiveAndVisible(window)
            && ops::GetWindowDisplayLayer(window) <= ops::GetWindowDisplayLayer(parent_window)
        {
            bottom_most_visible_window = window;
        }
    }
    return bottom_most_visible_window;
}

// Find window given position, search front-to-back
// FIXME: Note that we have an inconsequential lag here: OuterRectClipped is updated in Begin(), so windows moved programmatically
// with SetWindowPos() and not SetNextWindowPos() will have that rectangle lagging by a frame at the time FindHoveredWindow() is
// called, aka before the next Begin(). Moving window isn't affected.
// static c_void FindHoveredWindow()
pub unsafe fn FindHoveredWindows() {
    let g = GImGui; // ImGuiContext& g = *GImGui;

    // Special handling for the window being moved: Ignore the mouse viewport check (because it may reset/lose its viewport during the undocking frame)
    let mut moving_window_viewport: *mut ImGuiViewport = if !(g.MovingWindow.is_null()) {
        g.Movingwindow.Viewport
    } else {
        null_mut()
    };
    if g.MovingWindow {
        g.Movingwindow.Viewport = g.MouseViewport;
    }

    let mut hovered_window: *mut ImGuiWindow = null_mut();
    let mut hovered_window_ignoring_moving_window: *mut ImGuiWindow = null_mut();
    if g.MovingWindow && !(g.Movingwindow.Flags & ImGuiWindowFlags_NoMouseInputs) {
        hovered_window = g.MovingWindow;
    }

    let padding_regular: ImVec2 = g.Style.TouchExtraPadding.clone();
    let padding_for_resize: ImVec2 = if g.IO.ConfigWindowsResizeFromEdges {
        g.WindowsHoverPadding.clone()
    } else {
        padding_regular
    };
    // for (let i: c_int = g.Windows.Size - 1; i >= 0; i--)
    for i in g.Windows.len() - 1..0 {
        let mut window: *mut ImGuiWindow = g.Windows[i];
        IM_MSVC_WARNING_SUPPRESS(28182); // [Static Analyzer] Dereferencing NULL pointer.
        if !window.Active || window.Hidden {
            continue;
        }
        if window.Flags & ImGuiWindowFlags_NoMouseInputs {
            continue;
        }
        // IM_ASSERT(window.Viewport);
        if window.Viewport != g.MouseViewport {
            continue;
        }

        // Using the clipped AABB, a child window will typically be clipped by its parent (not always)
        let mut bb: ImRect = ImRect::from_vec4(window.OuterRectClipped.into());
        if window.Flags
            & (ImGuiWindowFlags_ChildWindow
                | ImGuiWindowFlags_NoResize
                | ImGuiWindowFlags_AlwaysAutoResize)
        {
            bb.expand_from_vec(&padding_regular.clone());
        } else {
            bb.expand_from_vec(&padding_for_resize);
        }
        if !bb.Contains(&g.IO.MousePos) {
            continue;
        }

        // Support for one rectangular hole in any given window
        // FIXME: Consider generalizing hit-testing override (with more generic data, callback, etc.) (#1512)
        if window.HitTestHoleSize.x != 0 {
            let hole_pos = ImVec2::from_floats(
                window.Pos.x + window.HitTestHoleOffset.x,
                window.Pos.y + window.HitTestHoleOffset.y,
            );
            let hole_size = ImVec2::from_floats(
                window.HitTestHoleSize.x as c_float,
                window.HitTestHoleSize.y as c_float,
            );
            if ImRect(hole_pos.clone(), hole_pos.clone() + hole_size)
                .Contains(g.IO.MousePos.clone())
            {
                continue;
            }
        }

        if hovered_window == null_mut() {
            hovered_window = window;
        }
        IM_MSVC_WARNING_SUPPRESS(28182); // [Static Analyzer] Dereferencing NULL pointer.
        if hovered_window_ignoring_moving_window == null_mut()
            && (g.MovingWindow.is_null()
                || window.RootWindowDockTree != g.Movingwindow.RootWindowDockTree)
        {
            hovered_window_ignoring_moving_window = window;
        }
        if hovered_window && hovered_window_ignoring_moving_window {
            break;
        }
    }

    g.HoveredWindow = hovered_window;
    g.HoveredWindowUnderMovingWindow = hovered_window_ignoring_moving_window;

    if g.MovingWindow {
        g.Movingwindow.Viewport = moving_window_viewport;
    }
}

pub unsafe fn FindWindowByID(id: ImGuiID) -> *mut ImGuiWindow {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.WindowsById.GetVoidPtr(id) as *mut ImGuiWindow;
}


pub unsafe fn FindWindowByName(name: *const c_char) ->  *mut ImGuiWindow
{
    let mut id: ImGuiID =  ImHashStr(name, 0, 0);
    return FindWindowByID(id);
}

pub fn GetWindowForTitleDisplay(window: *mut ImGuiWindow) -> *mut ImGuiWindow
{
    return if window.DockNodeAsHost { window.DockNodeAsHost.VisibleWindow } else { window };
}

pub fn GetWindowForTitleAndMenuHeight(window: *mut ImGuiWindow) -> *mut ImGuiWindow
{
    return if is_not_null(window.DockNodeAsHost) && is_not_null(window.DockNodeAsHost.VisibleWindow) { window.DockNodeAsHost.VisibleWindow } else { window };
}

// When a modal popup is open, newly created windows that want focus (i.e. are not popups and do not specify ImGuiWindowFlags_NoFocusOnAppearing)
// should be positioned behind that modal window, unless the window was created inside the modal begin-stack.
// In case of multiple stacked modals newly created window honors begin stack order and does not go below its own modal parent.
// - Window             // FindBlockingModal() returns Modal1
//   - Window           //                  .. returns Modal1
//   - Modal1           //                  .. returns Modal2
//      - Window        //                  .. returns Modal2
//          - Window    //                  .. returns Modal2
//          - Modal2    //                  .. returns Modal2
pub unsafe fn FindBlockingModal(window: *mut ImGuiWindow) -> *mut ImGuiWindow
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if g.OpenPopupStack.len() <= 0 {
        return null_mut();
    }

    // Find a modal that has common parent with specified window. Specified window should be positioned behind that modal.
    // for (let i: c_int = g.OpenPopupStack.Size - 1; i >= 0; i--)
    for i in g.OpenPopupStack.len() - 1 ..0
    {
        let mut popup_window: *mut ImGuiWindow =  g.OpenPopupStack.Data[i].Window;
        if popup_window == null_mut() || flag_clear(popup_window.Flags, ImGuiWindowFlags_Modal) {
            continue;
        }
        if !popup_window.Active && !popup_window.WasActive {    // Check WasActive, because this code may run before popup renders on current frame, also check Active to handle newly created windows.
            continue;
        }
        if IsWindowWithinBeginStackOf(window, popup_window) {     // Window is rendered over last modal, no render order change needed.
            break;
        }
        // for (let mut parent: *mut ImGuiWindow =  popup_window.ParentWindowInBeginStack->RootWindow; parent != null_mut(); parent = parent->ParentWindowInBeginStack->RootWindow)
        let mut parent: *mut ImGuiWindow = popup_window.ParentWindowInBeginStack.RootWindow;
        while parent != null_mut()
        {
            if IsWindowWithinBeginStackOf(window, parent)
            {
                return popup_window;
            }
            parent = parent.ParentWindowInBeginStack.RootWindow
        }      // Place window above its begin stack parent.
    }
    return null_mut();
}



pub unsafe fn FindWindowDisplayIndex(window: *mut ImGuiWindow) -> c_int
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.Windows.index_from_ptr(g.Windows.find(window));
}


pub unsafe fn GetCombinedRootWindow(mut window: *mut ImGuiWindow, popup_hierarchy: bool, dock_hierarchy: bool) -> *mut ImGuiWindow
{
    let mut last_window: *mut ImGuiWindow =  null_mut();
    while last_window != window
    {
        last_window = window;
        window = window.RootWindow;
        if popup_hierarchy {
            window = window.RootWindowPopupTree;
        }
		if dock_hierarchy {
            window = window.RootWindowDockTree;
        }
	}
    return window;
}


pub unsafe fn IsWindowChildOf(mut window: *mut ImGuiWindow, potential_parent: *mut ImGuiWindow, popup_hierarchy: bool, dock_hierarchy: bool) -> bool
{
    let mut window_root: *mut ImGuiWindow =  GetCombinedRootWindow(window, popup_hierarchy, dock_hierarchy);
    if window_root == potential_parent {
        return true;
    }
    while window != null_mut()
    {
        if window == potential_parent {
            return true;
        }
        if window == window_root {// end of chain
            return false;
        }
        window = window.ParentWindow;
    }
    return false;
}


pub fn IsWindowWithinBeginStackOf(mut window: *mut ImGuiWindow, potential_parent: *mut ImGuiWindow) -> bool
{
    if window.RootWindow == potential_parent {
        return true;
    }
    while window != null_mut()
    {
        if window == potential_parent {
            return true;
        }
        window = window.ParentWindowInBeginStack;
    }
    return false;
}

pub unsafe fn IsWindowAbove(potential_above: *mut ImGuiWindow, potential_below: *mut ImGuiWindow) -> bool
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    // It would be saner to ensure that display layer is always reflected in the g.Windows[] order, which would likely requires altering all manipulations of that array
    let display_layer_delta: c_int = GetWindowDisplayLayer(potential_above) - GetWindowDisplayLayer(potential_below);
    if display_layer_delta != 0 {
        return display_layer_delta > 0;
    }

    // for (let i: c_int = g.Windows.len() - 1; i >= 0; i--)
    for i in g.Windows.len() - 1 ..0
    {
        let mut candidate_window: *mut ImGuiWindow =  g.Windows[i];
        if candidate_window == potential_above {
            return true;
        }
        if candidate_window == potential_below {
            return false;
        }
    }
    return false;
}
