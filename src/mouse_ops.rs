#![allow(non_snake_case)]

use std::ptr::null_mut;
use libc::{c_float, c_int};
use crate::condition::ImGuiCond_Always;
use crate::config_flags::{ImGuiConfigFlags_NavEnableKeyboard, ImGuiConfigFlags_NavNoCaptureKeyboard, ImGuiConfigFlags_NoMouse, ImGuiConfigFlags_ViewportsEnable};
use crate::constants::{WINDOWS_HOVER_PADDING, WINDOWS_MOUSE_WHEEL_SCROLL_LOCK_TIMER};
use crate::dock_node::ImGuiDockNode;
use crate::drag_drop_flags::ImGuiDragDropFlags_SourceExtern;
use crate::dock_context_ops::DockContextQueueUndockNode;
use crate::drag_drop_ops::IsDragDropPayloadBeingAccepted;
use crate::id_ops::{ClearActiveID, KeepAliveID, SetActiveID};
use crate::imgui::GImGui;
use crate::input_ops::{IsMouseClicked, IsMouseDragging, IsMousePosValid};
use crate::key::{ImGuiKey_MouseWheelX, ImGuiKey_MouseWheelY};
use crate::math_ops::{ImClamp, ImMax, ImMin};
use crate::popup_flags::ImGuiPopupFlags_AnyPopupLevel;
use crate::popup_ops::{ClosePopupsOverWindow, GetTopMostPopupModal, IsPopupOpen};
use crate::scrolling_ops::{SetScrollX, SetScrollY};
use crate::vec2::ImVec2;
use crate::viewport_ops::UpdateTryMergeWindowIntoHostViewport;
use crate::window::find::{IsWindowAbove, IsWindowWithinBeginStackOf};
use crate::window::focus::FocusWindow;
use crate::window::ImGuiWindow;
use crate::window::props::SetWindowPos;
use crate::window::window_flags::{ImGuiWindowFlags, ImGuiWindowFlags_ChildWindow, ImGuiWindowFlags_NoMouseInputs, ImGuiWindowFlags_NoMove, ImGuiWindowFlags_NoScrollWithMouse, ImGuiWindowFlags_NoTitleBar, ImGuiWindowFlags_Popup};
use crate::window_flags::{ImGuiWindowFlags, ImGuiWindowFlags_ChildWindow, ImGuiWindowFlags_NoMouseInputs, ImGuiWindowFlags_NoMove, ImGuiWindowFlags_NoScrollWithMouse, ImGuiWindowFlags_NoTitleBar, ImGuiWindowFlags_Popup};

// c_void StartMouseMovingWindow(window: *mut ImGuiWindow)
pub unsafe fn StartMouseMovingWindow(window: *mut ImGuiWindow) {
    // Set ActiveId even if the _NoMove flag is set. Without it, dragging away from a window with _NoMove would activate hover on other windows.
    // We _also_ call this when clicking in a window empty space when io.ConfigWindowsMoveFromTitleBarOnly is set, but clear g.MovingWindow afterward.
    // This is because we want ActiveId to be set even when the window is not permitted to move.
    let g = GImGui; // ImGuiContext& g = *GImGui;
    FocusWindow(window);
    SetActiveID(window.MoveId, window);
    g.NavDisableHighlight = true;
    g.ActiveIdClickOffset = g.IO.MouseClickedPos[0].clone() - window.RootWindowDockTree.Pos.clone();
    g.ActiveIdNoClearOnFocusLoss = true;
    SetActiveIdUsingAllKeyboardKeys();

    let mut can_move_window: bool = true;
    if flag_set(window.Flags, ImGuiWindowFlags_NoMove) || (window.RootWindowDockTree.Flags & ImGuiWindowFlags_NoMove) {
        can_move_window = false;
    }
    let mut node = window.DockNodeAsHost;
    if node.is_null() == false {
        if node.VisibleWindow && (node.Visiblewindow.Flags & ImGuiWindowFlags_NoMove) {
            can_move_window = false;
        }
    }
    if can_move_window {
        g.MovingWindow = window;
    }
}

// We use 'undock_floating_node == false' when dragging from title bar to allow moving groups of floating nodes without undocking them.
// - undock_floating_node == true: when dragging from a floating node within a hierarchy, always undock the node.
// - undock_floating_node == false: when dragging from a floating node within a hierarchy, move root window.
// c_void StartMouseMovingWindowOrNode(window: *mut ImGuiWindow, node:*mut ImGuiDockNode, undock_floating_node: bool)
pub unsafe fn StartMouseMovingWindowOrNode(window: *mut ImGuiWindow, node: *mut ImGuiDockNode, undock_floating_node: bool) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut can_undock_node: bool = false;
    if node.is_null() == false && node.VisibleWindow.is_null() == false && (node.Visiblewindow.Flags & ImGuiWindowFlags_NoMove) == 0 {
        // Can undock if:
        // - part of a floating node hierarchy with more than one visible node (if only one is visible, we'll just move the whole hierarchy)
        // - part of a dockspace node hierarchy (trivia: undocking from a fixed/central node will create a new node and copy windows)
        let mut root_node = DockNodeGetRootNode(node);
        if root_node.OnlyNodeWithWindows != node || root_node.CentralNode != null_mut() {  // -V1051 PVS-Studio thinks node should be root_node and is wrong about that.
            if undock_floating_node || root_node.IsDockSpace() {
                can_undock_node = true;
            }
        }
    }

    let clicked: bool = IsMouseClicked(0, false);
    let dragging: bool = IsMouseDragging(0, g.IO.MouseDragThreshold * 1.70);
    if can_undock_node && dragging {
        DockContextQueueUndockNode(&g, node);
    } // Will lead to DockNodeStartMouseMovingWindow() -> StartMouseMovingWindow() being called next frame
    else if !can_undock_node && (clicked || dragging) && g.MovingWindow != window {
        StartMouseMovingWindow(window);
    }
}

// Handle mouse moving window
// Note: moving window with the navigation keys (Square + d-pad / CTRL+TAB + Arrows) are processed in NavUpdateWindowing()
// FIXME: We don't have strong guarantee that g.MovingWindow stay synched with g.ActiveId == g.Movingwindow.MoveId.
// This is currently enforced by the fact that BeginDragDropSource() is setting all g.ActiveIdUsingXXXX flags to inhibit navigation inputs,
// but if we should more thoroughly test cases where g.ActiveId or g.MovingWindow gets changed and not the other.
pub unsafe fn UpdateMouseMovingWindowNewFrame() {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if g.MovingWindow != null_mut() {
        // We actually want to move the root window. g.MovingWindow == window we clicked on (could be a child window).
        // We track it to preserve Focus and so that generally ActiveIdWindow == MovingWindow and ActiveId == Movingwindow.MoveId for consistency.
        KeepAliveID(g.ActiveId);
        // IM_ASSERT(g.MovingWindow && g.Movingwindow.RootWindowDockTree);
        let mut moving_window: *mut ImGuiWindow = g.MovingWindow.RootWindowDockTree;

        // When a window stop being submitted while being dragged, it may will its viewport until next Begin()
        let window_disappared: bool = ((!moving_window.WasActive && !moving_window.Active) || moving_window.Viewport == null_mut());
        if g.IO.MouseDown[0] && IsMousePosValid(&g.IO.MousePos) && !window_disappared {
            let pos: ImVec2 = g.IO.MousePos.clone() - g.ActiveIdClickOffset.clone();
            if moving_window.Pos.x != pos.x || moving_window.Pos.y != pos.y {
                SetWindowPos(moving_window, pos, ImGuiCond_Always);
                if moving_window.ViewportOwned // Synchronize viewport immediately because some overlays may relies on clipping rectangle before we Begin() into the window.
                {
                    moving_window.Viewport.Pos = pos.clone();
                    moving_window.Viewport.UpdateWorkRect();
                }
            }
            FocusWindow(g.MovingWindow);
        } else {
            if !window_disappared {
                // Try to merge the window back into the main viewport.
                // This works because MouseViewport should be != Movingwindow.Viewport on release (as per code in UpdateViewports)
                if g.ConfigFlagsCurrFrame & ImGuiConfigFlags_ViewportsEnable {
                    UpdateTryMergeWindowIntoHostViewport(moving_window, g.MouseViewport);
                }

                // Restore the mouse viewport so that we don't hover the viewport _under_ the moved window during the frame we released the mouse button.
                if !IsDragDropPayloadBeingAccepted() {
                    g.MouseViewport = moving_window.Viewport;
                }

                // Clear the NoInput window flag set by the Viewport system
                moving_window.Viewport.Flags &= !mGuiViewportFlags_NoInputs; // FIXME-VIEWPORT: Test engine managed to crash here because Viewport was NULL.
            }

            g.MovingWindow = null_mut();
            ClearActiveID();
        }
    } else {
        // When clicking/dragging from a window that has the _NoMove flag, we still set the ActiveId in order to prevent hovering others.
        if g.ActiveIdWindow.is_null() == false && g.ActiveIdWindow.MoveId == g.ActiveId {
            KeepAliveID(g.ActiveId);
            if !g.IO.MouseDown[0] {
                ClearActiveID();
            }
        }
    }
}

// Initiate moving window when clicking on empty space or title bar.
// Handle left-click and right-click focus.
// c_void UpdateMouseMovingWindowEndFrame()
pub unsafe fn UpdateMouseMovingWindowEndFrame() {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if g.ActiveId != 0 || g.HoveredId != 0 {
        return;
    }

    // Unless we just made a window/popup appear
    if g.NavWindow.is_null() == false && g.NavWindow.Appearing {
        return;
    }

    // Click on empty space to focus window and start moving
    // (after we're done with all our widgets, so e.g. clicking on docking tab-bar which have set HoveredId already and not get us here!)
    if g.IO.MouseClicked[0] {
        // Handle the edge case of a popup being closed while clicking in its empty space.
        // If we try to focus it, FocusWindow() > ClosePopupsOverWindow() will accidentally close any parent popups because they are not linked together any more.
        let mut root_window: *mut ImGuiWindow = if g.HoveredWindow.is_null() == false { g.Hoveredwindow.RootWindow } else { null_mut() };
        let is_closed_popup: bool = root_window.is_null() == false && flag_set(root_window.Flags, ImGuiWindowFlags_Popup) != 0 && !IsPopupOpen(root_window.PopupId, ImGuiPopupFlags_AnyPopupLevel);

        if root_window != null_mut() && !is_closed_popup {
            StartMouseMovingWindow(g.HoveredWindow); //-V595

            // Cancel moving if clicked outside of title bar
            if g.IO.ConfigWindowsMoveFromTitleBarOnly {
                if flag_clear(root_window.Flags, ImGuiWindowFlags_NoTitleBar) != 0 || root_window.DockIsActive {
                    if !root_window.TitleBarRect().Contains(g.IO.MouseClickedPos[0].clone()) {
                        g.MovingWindow = null_mut();
                    }
                }
            }

            // Cancel moving if clicked over an item which was disabled or inhibited by popups (note that we know HoveredId == 0 already)
            if g.HoveredIdDisabled {
                g.MovingWindow = null_mut();
            }
        } else if root_window == null_mut() && g.NavWindow != null_mut() && GetTopMostPopupModal() == null_mut() {
            // Clicking on void disable focus
            FocusWindow(null_mut());
        }
    }

    // With right mouse button we close popups without changing focus based on where the mouse is aimed
    // Instead, focus will be restored to the window under the bottom-most closed popup.
    // (The left mouse button path calls FocusWindow on the hovered window, which will lead NewFrame->ClosePopupsOverWindow to trigger)
    if g.IO.MouseClicked[1] {
        // Find the top-most window between HoveredWindow and the top-most Modal Window.
        // This is where we can trim the popup stack.
        let mut modal: *mut ImGuiWindow = GetTopMostPopupModal();
        let mut hovered_window_above_modal: bool = g.HoveredWindow.is_null() == false && (modal == null_mut() || IsWindowAbove(g.HoveredWindow, modal));
        ClosePopupsOverWindow(if hovered_window_above_modal { g.HoveredWindow } else { modal }, true);
    }
}


// static c_void UpdateMouseInputs()
pub unsafe fn UpdateMouseInputs() {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let io = &mut g.IO;

    // Round mouse position to avoid spreading non-rounded position (e.g. UpdateManualResize doesn't support them well)
    if IsMousePosValid(&io.MousePos) {
        io.MousePos = ImFloorSigned(io.MousePos.clone());
        g.MouseLastValidPos = ImFloorSigned(io.MousePos.clone());
    }

    // If mouse just appeared or disappeared (usually denoted by -f32::MAX components) we cancel out movement in MouseDelta
    if IsMousePosValid(&io.MousePos) && IsMousePosValid(&io.MousePosPrev) {
        io.MouseDelta = io.MousePos.clone() - io.MousePosPrev.clone();
    } else {
        io.MouseDelta = ImVec2::new(0.0, 0.0);
    }

    // If mouse moved we re-enable mouse hovering in case it was disabled by gamepad/keyboard. In theory should use a >0.0 threshold but would need to reset in everywhere we set this to true.
    if io.MouseDelta.x != 0.0 || io.MouseDelta.y != 0.0 {
        g.NavDisableMouseHover = false;
    }

    io.MousePosPrev = io.MousePos.clone();
    // for (let i: c_int = 0; i < IM_ARRAYSIZE(io.MouseDown); i++)
    for i in 0..io.MouseDown.len() {
        io.MouseClicked[i] = io.MouseDown[i] && io.MouseDownDuration[i] < 0.0;
        io.MouseClickedCount[i] = 0; // Will be filled below
        io.MouseReleased[i] = !io.MouseDown[i] && io.MouseDownDuration[i] >= 0.0;
        io.MouseDownDurationPrev[i] = io.MouseDownDuration[i];
        io.MouseDownDuration[i] = if io.MouseDown[i] { (if io.MouseDownDuration[i] < 0.0 { 0.0 } else { io.MouseDownDuration[i] + io.DeltaTime }) } else { -1.0 };
        if io.MouseClicked[i] {
            let mut is_repeated_click: bool = false;
            if (g.Time - io.MouseClickedTime[i]) < io.MouseDoubleClickTime {
                let delta_from_click_pos: ImVec2 = if IsMousePosValid(&io.MousePos) { (io.MousePos.clone() - io.MouseClickedPos[i].clone()) } else { ImVec2::new(0.0, 0.0) };
                if ImLengthSqr(delta_from_click_pos) < io.MouseDoubleClickMaxDist * io.MouseDoubleClickMaxDist {
                    is_repeated_click = true;
                }
            }
            if (is_repeated_click) {
                io.MouseClickedLastCount[i] += 1;
            } else {
                io.MouseClickedLastCount[i] = 1;
            }
            io.MouseClickedTime[i] = g.Time.clone();
            io.MouseClickedPos[i] = io.MousePos.clone();
            io.MouseClickedCount[i] = io.MouseClickedLastCount[i];
            io.MouseDragMaxDistanceAbs[i] = ImVec2::new(0.0, 0.0);
            io.MouseDragMaxDistanceSqr[i] = 0.0;
        } else if io.MouseDown[i] {
            // Maintain the maximum distance we reaching from the initial click position, which is used with dragging threshold
            let delta_from_click_pos: ImVec2 = if IsMousePosValid(&io.MousePos) { (io.MousePos.clone() - io.MouseClickedPos[i].clone()) } else { ImVec2::new(0.0, 0.0) };
            io.MouseDragMaxDistanceSqr[i] = ImMax(io.MouseDragMaxDistanceSqr[i], ImLengthSqr(delta_from_click_pos));
            io.MouseDragMaxDistanceAbs[i].x = ImMax(io.MouseDragMaxDistanceAbs[i].x, if delta_from_click_pos.x < 0.0 { delta_from_click_pos.x.clone() * -1 } else { delta_from_click_pos.x.clone() });
            io.MouseDragMaxDistanceAbs[i].y = ImMax(io.MouseDragMaxDistanceAbs[i].y, if delta_from_click_pos.y < 0.0 { delta_from_click_pos.y.clone() * -1 } else { delta_from_click_pos.y.clone() });
        }

        // We provide io.MouseDoubleClicked[] as a legacy service
        io.MouseDoubleClicked[i] = (io.MouseClickedCount[i] == 2);

        // Clicking any mouse button reactivate mouse hovering which may have been deactivated by gamepad/keyboard navigation
        if (io.MouseClicked[i]) {
            g.NavDisableMouseHover = false;
        }
    }
}

// static c_void StartLockWheelingWindow(window: *mut ImGuiWindow)
pub unsafe fn StartLockWheelingWindow(window: *mut ImGuiWindow) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if g.WheelingWindow == window {
        return;
    }
    g.WheelingWindow = window;
    g.WheelingWindowRefMousePos = g.IO.MousePos.clone();
    g.WheelingWindowTimer = WINDOWS_MOUSE_WHEEL_SCROLL_LOCK_TIMER;
}

// c_void UpdateMouseWheel()
pub unsafe fn UpdateMouseWheel() {
    let g = GImGui; // ImGuiContext& g = *GImGui;

    // Reset the locked window if we move the mouse or after the timer elapses
    if g.WheelingWindow != null_mut() {
        g.WheelingWindowTimer -= g.IO.DeltaTime;
        if IsMousePosValid(null_mut()) && ImLengthSqr(g.IO.MousePos.clone() - g.WheelingWindowRefMousePos.clone()) > g.IO.MouseDragThreshold * g.IO.MouseDragThreshold {
            g.WheelingWindowTimer = 0.0;
        }
        if g.WheelingWindowTimer <= 0.0 {
            g.WheelingWindow = null_mut();
            g.WheelingWindowTimer = 0.0;
        }
    }

    let hovered_id_using_mouse_wheel: bool = (g.HoveredIdPreviousFrame != 0 && g.HoveredIdPreviousFrameUsingMouseWheel);
    let active_id_using_mouse_wheel_x: bool = g.ActiveIdUsingKeyInputMask.TestBit(ImGuiKey_MouseWheelX);
    let active_id_using_mouse_wheel_y: bool = g.ActiveIdUsingKeyInputMask.TestBit(ImGuiKey_MouseWheelY);

    let mut wheel_x: c_float = if !hovered_id_using_mouse_wheel && !active_id_using_mouse_wheel_x { g.IO.MouseWheelH } else { 0.0 };
    let mut wheel_y: c_float = if !hovered_id_using_mouse_wheel && !active_id_using_mouse_wheel_y { g.IO.MouseWheel } else { 0 };
    if wheel_x == 0.0 && wheel_y == 0.0 {
        return;
    }

    let mut window: *mut ImGuiWindow = if g.WheelingWindow { g.WheelingWindow } else { g.HoveredWindow };
    if !window.is_null() || window.Collapsed {
        return;
    }

    // Zoom / Scale window
    // FIXME-OBSOLETE: This is an old feature, it still works but pretty much nobody is using it and may be best redesigned.
    if wheel_y != 0.0 && g.IO.KeyCtrl && g.IO.FontAllowUserScaling {
        StartLockWheelingWindow(window);
        let new_font_scale: c_float = ImClamp(window.FontWindowScale + g.IO.MouseWheel * 0.1.0, 0.50f32, 2.5);
        let scale: c_float = new_font_scale / window.FontWindowScale;
        window.FontWindowScale = new_font_scale;
        if window == window.RootWindow {
            let offset: ImVec2 = window.Size.clone() * (1.0 - scale) * (g.IO.MousePos.clone() - window.Pos.clone()) / window.Size.clone();
            SetWindowPos(window, window.Pos.clone() + offset, 0);
            window.Size = ImFloor(window.Size.clone() * scale);
            window.SizeFull = ImFloor(window.SizeFull.clone() * scale);
        }
        return;
    }

    // Mouse wheel scrolling
    // If a child window has the ImGuiWindowFlags_NoScrollWithMouse flag, we give a chance to scroll its parent
    if (g.IO.KeyCtrl) {
        return;
    }

    // As a standard behavior holding SHIFT while using Vertical Mouse Wheel triggers Horizontal scroll instead
    // (we avoid doing it on OSX as it the OS input layer handles this already)
    let swap_axis: bool = g.IO.KeyShift && !g.IO.ConfigMacOSXBehaviors;
    if swap_axis {
        wheel_x = wheel_y;
        wheel_y = 0.0;
    }

    // Vertical Mouse Wheel scrolling
    if wheel_y != 0.0 {
        StartLockWheelingWindow(window);
        while flag_set(window.Flags, ImGuiWindowFlags_ChildWindow) && ImGuiWindowFlags::from(((window.ScrollMax.y == 0.0) || ((window.Flags & ImGuiWindowFlags_NoScrollWithMouse) && flag_clear(window.Flags, ImGuiWindowFlags_NoMouseInputs)))) {
            window = window.ParentWindow;
        }
        if flag_clear(window.Flags, ImGuiWindowFlags_NoScrollWithMouse) && flag_clear(window.Flags, ImGuiWindowFlags_NoMouseInputs) {
            let max_step: c_float = window.InnerRect.GetHeight() * 0.67f32;
            let scroll_step: c_float = ImFloor(ImMin(5 * window.CalcFontSize(), max_step));
            SetScrollY(window, window.Scroll.y - wheel_y * scroll_step);
        }
    }

    // Horizontal Mouse Wheel scrolling, or Vertical Mouse Wheel w/ Shift held
    if wheel_x != 0.0 {
        StartLockWheelingWindow(window);
        while flag_set(window.Flags, ImGuiWindowFlags_ChildWindow) != 0 && ((window.ScrollMax.x == 0.0) || ((window.Flags & ImGuiWindowFlags_NoScrollWithMouse) != 0 && flag_clear(window.Flags, ImGuiWindowFlags_NoMouseInputs) != 0)) {
            window = window.ParentWindow;
        }
        if flag_clear(window.Flags, ImGuiWindowFlags_NoScrollWithMouse) && flag_clear(window.Flags, ImGuiWindowFlags_NoMouseInputs) {
            let max_step: c_float = window.InnerRect.GetWidth() * 0.67f32;
            let scroll_step: c_float = ImFloor(ImMin(2 * window.CalcFontSize(), max_step));
            SetScrollX(window, window.Scroll.x - wheel_x * scroll_step);
        }
    }
}

// The reason this is exposed in imgui_internal.h is: on touch-based system that don't have hovering, we want to dispatch inputs to the right target (imgui vs imgui+app)
// c_void UpdateHoveredWindowAndCaptureFlags()
pub unsafe fn UpdateHoveredWindowAndCaptureFlags() {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let io = &mut g.IO;
    g.WindowsHoverPadding = ImMax(g.Style.TouchExtraPadding.clone(), ImVec2::new(WINDOWS_HOVER_PADDING, WINDOWS_HOVER_PADDING));

    // Find the window hovered by mouse:
    // - Child windows can extend beyond the limit of their parent so we need to derive HoveredRootWindow from HoveredWindow.
    // - When moving a window we can skip the search, which also conveniently bypasses the fact that window.WindowRectClipped is lagging as this point of the frame.
    // - We also support the moved window toggling the NoInputs flag after moving has started in order to be able to detect windows below it, which is useful for e.g. docking mechanisms.
    let mut clear_hovered_windows: bool = false;
    FindHoveredWindow();
    // IM_ASSERT(g.HoveredWindow == NULL || g.HoveredWindow == g.MovingWindow || g.Hoveredwindow.Viewport == g.MouseViewport);

    // Modal windows prevents mouse from hovering behind them.
    let mut modal_window: *mut ImGuiWindow = GetTopMostPopupModal();
    if modal_window && g.HoveredWindow && !IsWindowWithinBeginStackOf(g.Hoveredwindow.RootWindow, modal_window) {// FIXME-MERGE: RootWindowDockTree ?
        clear_hovered_windows = true;
    }

    // Disabled mouse?
    if (io.ConfigFlags & ImGuiConfigFlags_NoMouse) {
        clear_hovered_windows = true;
    }

    // We track click ownership. When clicked outside of a window the click is owned by the application and
    // won't report hovering nor request capture even while dragging over our windows afterward.
    let has_open_popup: bool = (g.OpenPopupStack.len() > 0);
    let has_open_modal: bool = (modal_window != null_mut());
    let mut mouse_earliest_down: c_int = -1;
    let mut mouse_any_down: bool = false;
    // for (let i: c_int = 0; i < IM_ARRAYSIZE(io.MouseDown); i++)
    for i in 0..io.MouseDown {
        if io.MouseClicked[i] {
            io.MouseDownOwned[i] = (g.HoveredWindow != null_mut()) || has_open_popup;
            io.MouseDownOwnedUnlessPopupClose[i] = (g.HoveredWindow != null_mut()) || has_open_modal;
        }
        mouse_any_down |= io.MouseDown[i];
        if io.MouseDown[i] {
            if mouse_earliest_down == -1 || io.MouseClickedTime[i] < io.MouseClickedTime[mouse_earliest_down] {
                mouse_earliest_down = i as c_int;
            }
        }
    }
    let mouse_avail: bool = (mouse_earliest_down == -1) || io.MouseDownOwned[mouse_earliest_down];
    let mouse_avail_unless_popup_close: bool = (mouse_earliest_down == -1) || io.MouseDownOwnedUnlessPopupClose[mouse_earliest_down];

    // If mouse was first clicked outside of ImGui bounds we also cancel out hovering.
    // FIXME: For patterns of drag and drop across OS windows, we may need to rework/remove this test (first committed 311c0ca9 on 2015/02)
    let mouse_dragging_extern_payload: bool = g.DragDropActive && (g.DragDropSourceFlags & ImGuiDragDropFlags_SourceExtern) != 0;
    if !mouse_avail && !mouse_dragging_extern_payload {
        clear_hovered_windows = true;
    }

    if clear_hovered_windows {
        g.HoveredWindow = null_mut();
        g.HoveredWindowUnderMovingWindow = null_mut();
    }

    // Update io.WantCaptureMouse for the user application (true = dispatch mouse info to Dear ImGui only, false = dispatch mouse to Dear ImGui + underlying app)
    // Update io.WantCaptureMouseAllowPopupClose (experimental) to give a chance for app to react to popup closure with a drag
    if g.WantCaptureMouseNextFrame != -1 {
        io.WantCaptureMouse = (g.WantCaptureMouseNextFrame != 0);
        io.WantCaptureMouseUnlessPopupClose = (g.WantCaptureMouseNextFrame != 0);
    } else {
        io.WantCaptureMouse = (mouse_avail && (g.HoveredWindow != null_mut() || mouse_any_down)) || has_open_popup;
        io.WantCaptureMouseUnlessPopupClose = (mouse_avail_unless_popup_close && (g.HoveredWindow != null_mut() || mouse_any_down)) || has_open_modal;
    }

    // Update io.WantCaptureKeyboard for the user application (true = dispatch keyboard info to Dear ImGui only, false = dispatch keyboard info to Dear ImGui + underlying app)
    if g.WantCaptureKeyboardNextFrame != -1 {
        io.WantCaptureKeyboard = (g.WantCaptureKeyboardNextFrame != 0);
    } else {
        io.WantCaptureKeyboard = (g.ActiveId != 0) || (modal_window != null_mut());
    }
    if io.NavActive && (io.ConfigFlags & ImGuiConfigFlags_NavEnableKeyboard) != 0 && !(io.ConfigFlags & ImGuiConfigFlags_NavNoCaptureKeyboard) != 0 {
        io.WantCaptureKeyboard = true;
    }

    // Update io.WantTextInput flag, this is to allow systems without a keyboard (e.g. mobile, hand-held) to show a software keyboard if possible
    io.WantTextInput = if g.WantTextInputNextFrame != -1 { (g.WantTextInputNextFrame != 0) } else { false };
}
