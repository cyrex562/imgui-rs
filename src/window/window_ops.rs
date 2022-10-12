#![allow(non_snake_case)]

use std::mem;
use crate::color::{ImGuiCol_ModalWindowDimBg, ImGuiCol_NavWindowingDimBg, ImGuiCol_NavWindowingHighlight, IM_COL32_A_MASK, ImGuiCol, ImGuiCol_WindowBg, ImGuiCol_ChildBg, ImGuiCol_PopupBg, ImGuiCol_ResizeGripActive, ImGuiCol_ResizeGripHovered, ImGuiCol_ResizeGrip, ImGuiCol_Border, ImGuiCol_SeparatorActive, ImGuiCol_ButtonHovered, ImGuiCol_ButtonActive, ImGuiCol_MenuBarBg, ImGuiCol_TitleBgActive, IM_COL32_A_SHIFT, ImGuiCol_TitleBgCollapsed, ImGuiCol_TitleBg, ImGuiCol_Button};
use crate::condition::{ImGuiCond, ImGuiCond_Always, ImGuiCond_FirstUseEver, ImGuiCond_None};
use crate::draw_flags::{ImDrawFlags_None, ImDrawFlags_RoundCornersBottom, ImDrawFlags_RoundCornersTop};
use crate::draw_list::ImDrawList;
use crate::draw_list_ops::GetForegroundDrawList;
use crate::hovered_flags::{ImGuiHoveredFlags, ImGuiHoveredFlags_AllowWhenBlockedByPopup};
use crate::imgui::GImGui;
use crate::input_source::{ImGuiInputSource_Gamepad, ImGuiInputSource_Keyboard, ImGuiInputSource_Mouse, ImGuiInputSource_Nav};
use crate::next_window_data_flags::{ImGuiNextWindowDataFlags_HasBgAlpha, ImGuiNextWindowDataFlags_HasSize, ImGuiNextWindowDataFlags_HasSizeConstraint};
use crate::rect::ImRect;
use crate::render_ops::{RenderFrame, RenderRectFilledWithHole};
use crate::style_ops::GetColorU32;
use crate::type_defs::ImGuiID;
use crate::vec2::ImVec2;
use crate::window::ImGuiWindow;
use crate::window_flags::{ImGuiWindowFlags, ImGuiWindowFlags_AlwaysAutoResize, ImGuiWindowFlags_AlwaysHorizontalScrollbar, ImGuiWindowFlags_AlwaysVerticalScrollbar, ImGuiWindowFlags_ChildMenu, ImGuiWindowFlags_ChildWindow, ImGuiWindowFlags_DockNodeHost, ImGuiWindowFlags_HorizontalScrollbar, ImGuiWindowFlags_MenuBar, ImGuiWindowFlags_Modal, ImGuiWindowFlags_NoBackground, ImGuiWindowFlags_NoBringToFrontOnFocus, ImGuiWindowFlags_NoMouseInputs, ImGuiWindowFlags_NoResize, ImGuiWindowFlags_NoSavedSettings, ImGuiWindowFlags_NoScrollbar, ImGuiWindowFlags_NoTitleBar, ImGuiWindowFlags_Popup, ImGuiWindowFlags_Tooltip};
use crate::{ImGuiViewport, ImHashStr};
use libc::{c_char, c_float, c_int, c_void};
use std::ptr::{null, null_mut};
use crate::axis::{ImGuiAxis_X, ImGuiAxis_Y};
use crate::constants::{WINDOWS_HOVER_PADDING, WINDOWS_RESIZE_FROM_EDGES_FEEDBACK_TIMER};
use crate::direction::{ImGuiDir, ImGuiDir_Down, ImGuiDir_Left, ImGuiDir_Right, ImGuiDir_Up};
use crate::dock_node::ImGuiDockNode;
use crate::hash_ops::ImHashData;
use crate::id_ops::{ClearActiveID, KeepAliveID};
use crate::input_ops::IsMouseDragging;
use crate::key::{ImGuiKey_DownArrow, ImGuiKey_GamepadDpadDown, ImGuiKey_GamepadDpadLeft, ImGuiKey_GamepadDpadRight, ImGuiKey_GamepadDpadUp, ImGuiKey_LeftArrow, ImGuiKey_RightArrow, ImGuiKey_UpArrow};
use crate::math_ops::{ImClamp, ImLerp, ImLerpVec2, ImLerpVec22, ImMax, ImMin, ImSwap};
use crate::mouse_cursor::{ImGuiMouseCursor_ResizeEW, ImGuiMouseCursor_ResizeNESW, ImGuiMouseCursor_ResizeNS, ImGuiMouseCursor_ResizeNWSE};
use crate::mouse_ops::StartMouseMovingWindowOrNode;
use crate::nav_layer::{ImGuiNavLayer_Main, ImGuiNavLayer_Menu};
use crate::resize_border_def::resize_border_def;
use crate::resize_grip_def::resize_grip_def;
use crate::size_callback_data::ImGuiSizeCallbackData;
use crate::string_ops::str_to_const_c_char_ptr;
use crate::utils::{flag_clear, flag_set, is_not_null, is_null};
use crate::window_settings::ImGuiWindowSettings;

// [Internal] Small optimization to avoid calls to PopClipRect/SetCurrentChannel/PushClipRect in sequences,
// they would meddle many times with the underlying ImDrawCmd.
// Instead, we do a preemptive overwrite of clipping rectangle _without_ altering the command-buffer and let
// the subsequent single call to SetCurrentChannel() does it things once.
// c_void SetWindowClipRectBeforeSetChannel(*mut ImGuiWindow window, const ImRect& clip_rect)
pub fn SetWindowClipRectBeforeSetChannel(window: *mut ImGuiWindow, clip_rect: &ImRect) {
    let mut clip_rect_vec4 = clip_rect.ToVec4();
    window.ClipRect = clip_rect.ToVec4();
    window.DrawList._CmdHeader.ClipRect = clip_rect_vec4;
    window.DrawList._ClipRectStack[window.DrawList._ClipRectStack.len() - 1] =
        clip_rect_vec4.clone();
}

// inline ImRect           WindowRectRelToAbs(*mut ImGuiWindow window, const ImRect& r)
pub fn WindowRectRelToAbs(window: *mut ImGuiWindow, r: &ImRect) -> ImRect {
    let off = window.DC.CursorStartPos.clone();
    ImRect::from_floats(
        r.Min.x + off.x,
        r.Min.y + off.y,
        r.Max.x + off.x,
        r.Max.y + off.y,
    )
}

// inline ImRect           WindowRectAbsToRel(*mut ImGuiWindow window, const ImRect& r)
pub fn WindowRectAbsToRel(window: *mut ImGuiWindow, r: &ImRect) -> ImRect {
    let mut off: ImVec2 = window.DC.CursorStartPos.clone();
    return ImRect::from_floats(
        r.Min.x - off.x,
        r.Min.y - off.y,
        r.Max.x - off.x,
        r.Max.y - off.y,
    );
}

// static c_void SetCurrentWindow(window: *mut ImGuiWindow)
pub unsafe fn SetCurrentWindow(window: *mut ImGuiWindow) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.CurrentWindow = window;
    g.CurrentTable = if window.is_null() == false && window.DC.CurrentTableIdx != -1 {
        g.Tables.GetByIndex(window.DC.CurrentTableIdx)
    } else {
        null_mut()
    };
    if window {
        g.FontSize = window.CalcFontSize();
        g.DrawListSharedData.FontSize = window.CalcFontSize();
    }
}

// static inline IsWindowContentHoverable: bool(window: *mut ImGuiWindow, ImGuiHoveredFlags flags)
pub unsafe fn IsWindowContentHoverable(window: *mut ImGuiWindow, flags: ImGuiHoveredFlags) -> bool {
    // An active popup disable hovering on other windows (apart from its own children)
    // FIXME-OPT: This could be cached/stored within the window.
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if g.NavWindow {
        if focused_root_window: *mut ImGuiWindow = g.NavWindow.RootWindowDockTree {
            if focused_root_window.WasActive && focused_root_window != window.RootWindowDockTree {
                // For the purpose of those flags we differentiate "standard popup" from "modal popup"
                // NB: The order of those two tests is important because Modal windows are also Popups.
                if focused_root_window.Flags & ImGuiWindowFlags_Modal {
                    return false;
                }
                if (focused_root_window.Flags & ImGuiWindowFlags_Popup)
                    && !(flags & ImGuiHoveredFlags_AllowWhenBlockedByPopup)
                {
                    return false;
                }
            }
        }
    }
    // Filter by viewport
    if window.Viewport != g.MouseViewport {
        if g.MovingWindow == null_mut()
            || window.RootWindowDockTree != g.MovingWindow.RootWindowDockTree
        {
            return false;
        }
    }

    return true;
}

// This is called during NewFrame()->UpdateViewportsNewFrame() only.
// Need to keep in sync with SetWindowPos()
// static c_void TranslateWindow(window: *mut ImGuiWindow, const ImVec2& delta)
pub fn TranslateWindow(window: *mut ImGuiWindow, delta: &ImVec2) {
    window.Pos += delta;
    window.ClipRect.Translate(delta);
    window.OuterRectClipped.Translate(delta);
    window.InnerRect.Translate(delta);
    window.DC.CursorPos += delta;
    window.DC.CursorStartPos += delta;
    window.DC.CursorMaxPos += delta;
    window.DC.IdealMaxPos += delta;
}

// static c_void ScaleWindow(window: *mut ImGuiWindow, c_float scale)
pub fn ScaleWindow(window: *mut ImGuiWindow, scale: c_float) {
    let origin: ImVec2 = window.Viewport.Pos.clone();
    window.Pos = ImFloor((window.Pos.clone() - origin) * scale + origin.clone());
    window.Size = ImFloor(window.Size.clone() * scale);
    window.SizeFull = ImFloor(window.SizeFull.clone() * scale);
    window.ContentSize = ImFloor(window.ContentSize.clone() * scale);
}

// static IsWindowActiveAndVisible: bool(window: *mut ImGuiWindow)
pub fn IsWindowActiveAndVisible(window: *mut ImGuiWindow) -> bool {
    return (window.Active) && (!window.Hidden);
}

// FIXME: Add a more explicit sort order in the window structure.
// static IMGUI_CDECL: c_int ChildWindowComparer(*const c_void lhs, *const c_void rhs)
pub fn ChildWindowComparer(lhs: *const c_void, rhs: *const c_void) -> c_int {
    let a: *const ImGuiWindow = lhs;
    let b: *const ImGuiWindow = rhs;
    let mut d = (a.Flags & ImGuiWindowFlags_Popup) - (b.Flags & ImGuiWindowFlags_Popup);
    if d {
        return d;
    }
    //     if (let d: c_int = (a->Flags & ImGuiWindowFlags_Popup) - (b->Flags & ImGuiWindowFlags_Popup))
    //     {
    // return d;
    //     }
    //     if (let d: c_int = (a->Flags & ImGuiWindowFlags_Tooltip) - (b->Flags & ImGuiWindowFlags_Tooltip))
    //         return d;
    d = (a.Flags & ImGuiWindowFlags_Tooltip) - (b.Flags & ImGuiWindowFlags_Tooltip);
    if d {
        return d;
    }
    return (a.BeginOrderWithinParent - b.BeginOrderWithinParent) as c_int;
}

// static c_void AddWindowToSortBuffer(Vec<ImGuiWindow*>* out_sorted_windows, window: *mut ImGuiWindow)
pub fn AddWindowToSortBuffer(
    mut out_sorted_windows: *mut Vec<*mut ImGuiWindow>,
    window: *mut ImGuiWindow,
) {
    out_sorted_windows.push(window);
    if window.Active {
        let count: c_int = window.DC.ChildWindows.Size;
        // ImQsort(window.DC.ChildWindows.Data, count, sizeof(ImGuiWindow*), ChildWindowComparer);
        // todo!()
        // for (let i: c_int = 0; i < count; i++)
        for i in 0..count {
            let mut child: *mut ImGuiWindow = window.DC.ChildWindows[i];
            if child.Active {
                AddWindowToSortBuffer(out_sorted_windows, child);
            }
        }
    }
}

// static inline GetWindowDisplayLayer: c_int(window: *mut ImGuiWindow)
pub fn GetWindowDisplayLayer(window: *mut ImGuiWindow) -> c_int {
    return if window.Flags & ImGuiWindowFlags_Tooltip {
        1
    } else {
        0
    };
}

// Push a clipping rectangle for both ImGui logic (hit-testing etc.) and low-level ImDrawList rendering.
// - When using this function it is sane to ensure that float are perfectly rounded to integer values,
//   so that e.g. (max.x-min.x) in user's render produce correct result.
// - If the code here changes, may need to update code of functions like NextColumn() and PushColumnClipRect():
//   some frequently called functions which to modify both channels and clipping simultaneously tend to use the
//   more specialized SetWindowClipRectBeforeSetChannel() to avoid extraneous updates of underlying ImDrawCmds.
// c_void PushClipRect(const ImVec2& clip_rect_min, const ImVec2& clip_rect_max, intersect_with_current_clip_rect: bool)
pub unsafe fn PushClipRect(
    clip_rect_min: &ImVec2,
    clip_rect_max: &ImVec2,
    intersect_with_current_clip_rect: bool,
) {
    let mut window: *mut ImGuiWindow = GetCurrentWindow();
    window.DrawList.PushClipRect(
        clip_rect_min,
        clip_rect_max,
        intersect_with_current_clip_rect,
    );
    window.ClipRect = window.DrawList._ClipRectStack.last().unwrap().clone();
}

// c_void PopClipRect()
pub unsafe fn PopClipRect() {
    let mut window: *mut ImGuiWindow = GetCurrentWindow();
    window.DrawList.PopClipRect();
    window.ClipRect = window.DrawList._ClipRectStack.last().unwrap().clone();
}

// static FindFrontMostVisibleChildWindow: *mut ImGuiWindow(window: *mut ImGuiWindow)
pub fn FindFrontMostVisibleChildWindow(window: *mut ImGuiWindow) -> *mut ImGuiWindow {
    // for (let n: c_int = window.DC.ChildWindows.Size - 1; n >= 0; n--)
    for n in window.DC.ChildWindows.len() - 1..0 {
        if IsWindowActiveAndVisible(window.DC.ChildWindows[n]) {
            return FindFrontMostVisibleChildWindow(window.DC.ChildWindows[n]);
        }
    }
    return window;
}

// static c_void RenderDimmedBackgroundBehindWindow(window: *mut ImGuiWindow, col: u32)
pub unsafe fn RenderDimmedBackgroundBehindWindow(window: *mut ImGuiWindow, col: u32) {
    if (col & IM_COL32_A_MASK) == 0 {
        return;
    }

    let mut viewport: *mut ImGuiViewport = window.Viewport;
    let viewport_rect: ImRect = viewport.GetMainRect();

    // Draw behind window by moving the draw command at the FRONT of the draw list
    unsafe {
        // We've already called AddWindowToDrawData() which called DrawList.ChannelsMerge() on DockNodeHost windows,
        // and draw list have been trimmed already, hence the explicit recreation of a draw command if missing.
        // FIXME: This is creating complication, might be simpler if we could inject a drawlist in drawdata at a given position and not attempt to manipulate ImDrawCmd order.
        let mut draw_list: *mut ImDrawList = window.RootWindowDockTree.DrawList;
        if draw_list.CmdBuffer.len() == 0 {
            draw_list.AddDrawCmd();
        }
        draw_list.PushClipRect(
            viewport_rect.Min - ImVec2::new(1.0, 1.0),
            viewport_rect.Max + ImVec2::new(1.0, 1.0),
            false,
        ); // Ensure ImDrawCmd are not merged
        draw_list.AddRectFilled(
            &viewport_rect.Min,
            &viewport_rect.Max,
            col,
            0f32,
            ImDrawFlags_None,
        );
        let cmd = draw_list.CmdBuffer.last().unwrap();
        // IM_ASSERT(cmd.ElemCount == 6);
        draw_list.CmdBuffer.pop_back();
        draw_list.CmdBuffer.push_front(cmd);
        draw_list.PopClipRect();
        draw_list.AddDrawCmd(); // We need to create a command as CmdBuffer.back().IdxOffset won't be correct if we append to same command.
    }

    // Draw over sibling docking nodes in a same docking tree
    if window.Rootwindow.DockIsActive {
        let mut draw_list: *mut ImDrawList =
            FindFrontMostVisibleChildWindow(window.RootWindowDockTree).DrawList;
        if draw_list.CmdBuffer.len() == 0 {
            draw_list.AddDrawCmd();
        }
        draw_list.PushClipRect(&viewport_rect.Min, &viewport_rect.Max, false);
        RenderRectFilledWithHole(
            draw_list,
            window.RootWindowDockTree.Rect(),
            window.Rootwindow.Rect(),
            col,
            0f32,
        ); // window.RootWindowDockTree->WindowRounding);
        draw_list.PopClipRect();
    }
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
        if IsWindowActiveAndVisible(window)
            && GetWindowDisplayLayer(window) <= GetWindowDisplayLayer(parent_window)
        {
            bottom_most_visible_window = window;
        }
    }
    return bottom_most_visible_window;
}

pub unsafe fn RenderDimmedBackgrounds() {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut modal_window: *mut ImGuiWindow = GetTopMostAndVisiblePopupModal();
    if g.DimBgRatio <= 0f32 && g.NavWindowingHighlightAlpha <= 0f32 {
        return;
    }
    let dim_bg_for_modal: bool = (modal_window != null_mut());
    let dim_bg_for_window_list: bool =
        (g.NavWindowingTargetAnim != null_mut() && g.NavWindowingTargetAnim.Active);
    if !dim_bg_for_modal && !dim_bg_for_window_list {
        return;
    }

    let mut viewports_already_dimmed: [*mut ImGuiViewport; 2] = [null_mut(), null_mut()];
    if dim_bg_for_modal {
        // Draw dimming behind modal or a begin stack child, whichever comes first in draw order.
        let mut dim_behind_window: *mut ImGuiWindow =
            FindBottomMostVisibleWindowWithinBeginStack(modal_window);
        RenderDimmedBackgroundBehindWindow(
            dim_behind_window,
            GetColorU32(ImGuiCol_ModalWindowDimBg, g.DimBgRatio),
        );
        viewports_already_dimmed[0] = modal_window.Viewport;
    } else if dim_bg_for_window_list {
        // Draw dimming behind CTRL+Tab target window and behind CTRL+Tab UI window
        RenderDimmedBackgroundBehindWindow(
            g.NavWindowingTargetAnim,
            GetColorU32(ImGuiCol_NavWindowingDimBg, g.DimBgRatio),
        );
        if g.NavWindowingListWindow != null_mut()
            && g.NavWindowingListwindow.Viewport
            && g.NavWindowingListwindow.Viewport != g.NavWindowingTargetAnim.Viewport
        {
            RenderDimmedBackgroundBehindWindow(
                g.NavWindowingListWindow,
                GetColorU32(ImGuiCol_NavWindowingDimBg, g.DimBgRatio),
            );
        }
        viewports_already_dimmed[0] = g.NavWindowingTargetAnim.Viewport;
        viewports_already_dimmed[1] = if g.NavWindowingListWindow {
            g.NavWindowingListwindow.Viewport
        } else {
            null_mut()
        };

        // Draw border around CTRL+Tab target window
        let mut window: *mut ImGuiWindow = g.NavWindowingTargetAnim;
        ImGuiViewport * viewport = window.Viewport;
        let distance: c_float = g.FontSize;
        let mut bb: ImRect = window.Rect();
        bb.Expand(distance);
        if bb.GetWidth() >= viewport.Size.x && bb.GetHeight() >= viewport.Size.y {
            bb.Expand(-distance - 1f32);
        } // If a window fits the entire viewport, adjust its highlight inward
        if window.DrawList.CmdBuffer.len() == 0 {
            window.DrawList.AddDrawCmd();
        }
        window
            .DrawList
            .PushClipRect(viewport.Pos, viewport.Pos + viewport.Size, false);
        window.DrawList.AddRect(
            &bb.Min,
            &bb.Max,
            GetColorU32(ImGuiCol_NavWindowingHighlight, g.NavWindowingHighlightAlpha),
            window.WindowRounding,
            0,
            3.00f32,
        );
        window.DrawList.PopClipRect();
    }

    // Draw dimming background on _other_ viewports than the ones our windows are in
    // for (let viewport_n: c_int = 0; viewport_n < g.Viewports.Size; viewport_n++)
    for viewport_n in 0..g.Viewports.len() {
        let mut viewport: *mut ImGuiViewport = g.Viewports[viewport_n];
        if viewport == viewports_already_dimmed[0] || viewport == viewports_already_dimmed[1] {
            continue;
        }
        if modal_window && viewport.Window && IsWindowAbove(viewport.Window, modal_window) {
            continue;
        }
        let mut draw_list: *mut ImDrawList = GetForegroundDrawList(viewport);
        let dim_bg_col = GetColorU32(
            if dim_bg_for_modal {
                ImGuiCol_ModalWindowDimBg
            } else {
                ImGuiCol_NavWindowingDimBg
            },
            g.DimBgRatio,
        );
        draw_list.AddRectFilled(
            &viewport.Pos,
            viewport.Pos.clone() + viewport.Size.clone(),
            dim_bg_col,
            0f32,
            ImDrawFlags_None,
        );
    }
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
            bb.Expand2(&padding_regular.clone());
        } else {
            bb.Expand2(&padding_for_resize);
        }
        if !bb.Contains(&g.IO.MousePos) {
            continue;
        }

        // Support for one rectangular hole in any given window
        // FIXME: Consider generalizing hit-testing override (with more generic data, callback, etc.) (#1512)
        if window.HitTestHoleSize.x != 0 {
            let hole_pos = ImVec2::new(
                window.Pos.x + window.HitTestHoleOffset.x,
                window.Pos.y + window.HitTestHoleOffset.y,
            );
            let hole_size = ImVec2::new(
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

pub unsafe fn SetNextWindowSize(size: &ImVec2, cond: ImGuiCond) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
                    // IM_ASSERT(cond == 0 || ImIsPowerOfTwo(cond)); // Make sure the user doesn't attempt to combine multiple condition flags.
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasSize;
    g.NextWindowData.SizeVal = size.clone();
    g.NextWindowData.SizeCond = if cond != ImGuiCond_None {
        cond
    } else {
        ImGuiCond_Always
    };
}

pub fn SetWindowConditionAllowFlags(window: *mut ImGuiWindow, flags: ImGuiCond, enabled: bool) {
    window.SetWindowPosAllowFlags = if enabled {
        (window.SetWindowPosAllowFlags | flags)
    } else {
        window.SetWindowPosAllowFlags & !flags
    };
    window.SetWindowSizeAllowFlags = if enabled {
        (window.SetWindowSizeAllowFlags | flags)
    } else {
        (window.SetWindowSizeAllowFlags & !flags)
    };
    window.SetWindowCollapsedAllowFlags = if enabled {
        (window.SetWindowCollapsedAllowFlags | flags)
    } else {
        window.SetWindowCollapsedAllowFlags & !flags
    };
    window.SetWindowDockAllowFlags = if enabled {
        (window.SetWindowDockAllowFlags | flags)
    } else {
        window.SetWindowDockAllowFlags & !flags
    };
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


pub fn ApplyWindowSettings(window: *mut ImGuiWindow, settings: *mut ImGuiWindowSettings)
{
    let main_viewport: *const ImGuiViewport = GetMainViewport();
    window.ViewportPos = main_viewport.Pos;
    if settings.ViewportId
    {
        window.ViewportId = settings.ViewportId;
        window.ViewportPos = ImVec2::new(settings.ViewportPos.x as c_float, settings.ViewportPos.y as c_float);
    }
    window.Pos = ImFloor(ImVec2::new(settings.Pos.x + window.ViewportPos.x, settings.Pos.y + window.ViewportPos.y));
    if settings.Size.x > 0 && settings.Size.y > 0 {
        window.SizeFull = ImFloor(ImVec2::new(settings.Size.x as c_float, settings.Size.y as c_float));
        window.Size = window.SizeFull;
    }
    window.Collapsed = settings.Collapsed;
    window.DockId = settings.DockId;
    window.DockOrder = settings.DockOrder;
}


pub unsafe fn UpdateWindowInFocusOrderList(window: *mut ImGuiWindow, just_created: bool, new_flags: ImGuiWindowFlags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let new_is_explicit_child: bool = (new_flags & ImGuiWindowFlags_ChildWindow) != 0;
    let child_flag_changed: bool = new_is_explicit_child != window.IsExplicitChild;
    if (just_created || child_flag_changed) && !new_is_explicit_child
    {
        // IM_ASSERT(!g.WindowsFocusOrder.contains(window));
        g.WindowsFocusOrder.push(window);
        window.FocusOrder = (g.WindowsFocusOrder.Size - 1);
    }
    else if !just_created && child_flag_changed && new_is_explicit_child
    {
        // IM_ASSERT(g.WindowsFocusOrder[window.FocusOrder] == window);
        // for (let n: c_int = window.FocusOrder + 1; n < g.WindowsFocusOrder.Size; n++)
        for n in window.FocusOrder + 1 .. g.WindowsFocusOrder.len()
        {
            g.WindowsFocusOrder[n].FocusOrder -= 1;
        }
        g.WindowsFocusOrder.erase(g.WindowsFocusOrder.Data + window.FocusOrder);
        window.FocusOrder = -1;
    }
    window.IsExplicitChild = new_is_explicit_child;
}


pub unsafe fn CreateNewWindow(name: *const c_char, flags: ImGuiWindowFlags) -> *mut ImGuiWindow
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    //IMGUI_DEBUG_LOG("CreateNewWindow '%s', flags = 0x%08X\n", name, flags);

    // Create window the first time
    let mut window: *mut ImGuiWindow =  IM_NEW(ImGuiWindow)(&g, name);
    window.Flags = flags;
    g.WindowsById.SetVoidPtr(window.ID, window);

    // Default/arbitrary window position. Use SetNextWindowPos() with the appropriate condition flag to change the initial position of a window.
    let main_viewport: *const ImGuiViewport = GetMainViewport();
    window.Pos = main_viewport.Pos + ImVec2::new(60.0, 60.0);
    window.ViewportPos = main_viewport.Pos;

    // User can disable loading and saving of settings. Tooltip and child windows also don't store settings.
    if !(flags & ImGuiWindowFlags_NoSavedSettings) {
        if settings: *mut ImGuiWindowSettings = FindWindowSettings(window.ID) {
            // Retrieve settings from .ini file
            window.SettingsOffset = g.SettingsWindows.offset_from_ptr(settings);
            SetWindowConditionAllowFlags(window, ImGuiCond_FirstUseEver, false);
            ApplyWindowSettings(window, settings);
        }
    }
    window.DC.CursorStartPos = window.Pos;
    window.DC.CursorMaxPos = window.Pos;
    window.DC.IdealMaxPos = window.Pos; // So first call to CalcWindowContentSizes() doesn't return crazy values

    if (flags & ImGuiWindowFlags_AlwaysAutoResize) != 0
    {
        window.AutoFitFramesX = 2;
        window.AutoFitFramesY = 2;
        window.AutoFitOnlyGrows = false;
    }
    else
    {
        if window.Size.x <= 0.0 {
            window.AutoFitFramesX = 2;
        }
        if window.Size.y <= 0.0 {
            window.AutoFitFramesY = 2;
        }
        window.AutoFitOnlyGrows = (window.AutoFitFramesX > 0) || (window.AutoFitFramesY > 0);
    }

    if flags & ImGuiWindowFlags_NoBringToFrontOnFocus {
        g.Windows.push_front(window);
    } // Quite slow but rare and only once
    else {
        g.Windows.push(window);
    }

    return window;
}

pub fn GetWindowForTitleDisplay(window: *mut ImGuiWindow) -> *mut ImGuiWindow
{
    return if window.DockNodeAsHost { window.DockNodeAsHost.VisibleWindow } else { window };
}

pub fn GetWindowForTitleAndMenuHeight(window: *mut ImGuiWindow) -> *mut ImGuiWindow
{
    return if is_not_null(window.DockNodeAsHost) && is_not_null(window.DockNodeAsHost.VisibleWindow) { window.DockNodeAsHost.VisibleWindow } else { window };
}


pub unsafe fn CalcWindowSizeAfterConstraint(window: *mut ImGuiWindow, size_desired: &ImVec2) -> ImVec2
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut new_size: ImVec2 = size_desired.clone();
    if (g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasSizeConstraint)
    {
        // Using -1,-1 on either X/Y axis to preserve the current size.
        let cr: ImRect =  g.NextWindowData.SizeConstraintRect;
        new_size.x = if cr.Min.x >= 0.0 && cr.Max.x >= 0.0 { ImClamp(new_size.x, cr.Min.x, cr.Max.x) } else { window.SizeFull.x };
        new_size.y = if cr.Min.y >= 0.0 && cr.Max.y >= 0.0 { ImClamp(new_size.y, cr.Min.y, cr.Max.y) } else { window.SizeFull.y };
        if g.NextWindowData.SizeCallback
        {
            let mut data = ImGuiSizeCallbackData::default();
            data.UserData = g.NextWindowData.SizeCallbackUserData;
            data.Pos = window.Pos;
            data.CurrentSize = window.SizeFull;
            data.DesiredSize = new_size;
            g.NextWindowData.SizeCallback(&data);
            new_size = data.DesiredSize;
        }
        new_size.x = IM_FLOOR(new_size.x);
        new_size.y = IM_FLOOR(new_size.y);
    }

    // Minimum size
    if !(window.Flags & (ImGuiWindowFlags_ChildWindow | ImGuiWindowFlags_AlwaysAutoResize))
    {
        let mut window_for_height: *mut ImGuiWindow =  GetWindowForTitleAndMenuHeight(window);
        let decoration_up_height: c_float =  window_for_height.TitleBarHeight() + window_for_height.MenuBarHeight();
        new_size = ImMax(new_size, g.Style.WindowMinSize);
        new_size.y = ImMax(new_size.y, decoration_up_height + ImMax(0f32, g.Style.WindowRounding - 1f32)); // Reduce artifacts with very small windows
    }
    return new_size;
}


pub unsafe fn CalcWindowAutoFitSize(window: *mut ImGuiWindow, size_contents: &ImVec2) -> ImVec2 {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let style = &mut g.Style;
    let decoration_up_height: c_float = window.TitleBarHeight() + window.MenuBarHeight();
    let size_pad: ImVec2 = window.WindowPadding * 2.0f32;
    let size_desired: ImVec2 = size_contents + size_pad + ImVec2::new(0f32, decoration_up_height);
    if window.Flags & ImGuiWindowFlags_Tooltip {
        // Tooltip always resize
        return size_desired;
    } else {
        // Maximum window size is determined by the viewport size or monitor size
        let is_popup: bool = (window.Flags & ImGuiWindowFlags_Popup) != 0;
        let is_menu: bool = (window.Flags & ImGuiWindowFlags_ChildMenu) != 0;
        let mut size_min: ImVec2 = style.WindowMinSize;
        if is_popup || is_menu { // Popups and menus bypass style.WindowMinSize by default, but we give then a non-zero minimum size to facilitate understanding problematic cases (e.g. empty popups)
            size_min = ImMin(size_min, ImVec2::new(4.0, 4.0));
        }

        // FIXME-VIEWPORT-WORKAREA: May want to use GetWorkSize() instead of Size depending on the type of windows?
        let mut avail_size: ImVec2 = window.Viewport.Size;
        if window.ViewportOwned {
            avail_size = ImVec2::new(f32::MAX, f32::MAX);
        }
        let monitor_idx: c_int = window.ViewportAllowPlatformMonitorExtend;
        if monitor_idx >= 0 && monitor_idx < g.PlatformIO.Monitors.Size {
            avail_size = g.PlatformIO.Monitors[monitor_idx].WorkSize;
        }
        let mut size_auto_fit: ImVec2 = ImClamp(size_desired, size_min, ImMax(size_min, avail_size - style.DisplaySafeAreaPadding * 2.00f32));

        // When the window cannot fit all contents (either because of constraints, either because screen is too small),
        // we are growing the size on the other axis to compensate for expected scrollbar. FIXME: Might turn bigger than ViewportSize-WindowPadding.
        let size_auto_fit_after_constraint: ImVec2 = CalcWindowSizeAfterConstraint(window, &size_auto_fit);
        let mut will_have_scrollbar_x: bool = (size_auto_fit_after_constraint.x - size_pad.x - 0.0 < size_contents.x && flag_clear(window.Flags, ImGuiWindowFlags_NoScrollbar) && flag_set(window.Flags, ImGuiWindowFlags_HorizontalScrollbar)) || flag_set(window.Flags, ImGuiWindowFlags_AlwaysHorizontalScrollbar);
        let mut will_have_scrollbar_y: bool = (size_auto_fit_after_constraint.y - size_pad.y - decoration_up_height < size_contents.y && flag_clear(window.Flags, ImGuiWindowFlags_NoScrollbar)) || flag_set(window.Flags, ImGuiWindowFlags_AlwaysVerticalScrollbar);
        if will_have_scrollbar_x {
            size_auto_fit.y += style.ScrollbarSize;
        }
        if will_have_scrollbar_y {
            size_auto_fit.x += style.ScrollbarSize;
        }
        return size_auto_fit;
    }
}




pub unsafe fn CalcWindowNextAutoFitSize(window: *mut ImGuiWindow) -> ImVec2
{
    let mut size_contents_current: ImVec2 = ImVec2::default();
    let mut size_contents_ideal: ImVec2 = ImVec2::default();
    CalcWindowContentSizes(window, &size_contents_current, &size_contents_ideal);
    let size_auto_fit: ImVec2 = CalcWindowAutoFitSize(window, &size_contents_ideal);
    let size_final: ImVec2 = CalcWindowSizeAfterConstraint(window, &size_auto_fit);
    return size_final;
}


pub fn GetWindowBgColorIdx(window: *mut ImGuiWindow) -> ImGuiCol
{
    if window.Flags & (ImGuiWindowFlags_Tooltip | ImGuiWindowFlags_Popup) {
        return ImGuiCol_PopupBg;
    }
    if flag_set(window.Flags, ImGuiWindowFlags_ChildWindow) && !window.DockIsActive {
        return ImGuiCol_ChildBg;
    }
    return ImGuiCol_WindowBg;
}


pub unsafe fn CalcResizePosSizeFromAnyCorner(window: *mut ImGuiWindow, corner_target: &ImVec2, corner_norm: &ImVec2, out_pos: *mut ImVec2, out_size: *mut ImVec2) {
    let pos_min: ImVec2 = ImLerpVec22(corner_target, &window.Pos, corner_norm);                // Expected window upper-left
    let pos_max: ImVec2 = ImLerpVec22(window.Pos + window.Size, corner_target, corner_norm); // Expected window lower-right
    let size_expected: ImVec2 = pos_max - pos_min;
    let size_constrained: ImVec2 = CalcWindowSizeAfterConstraint(window, &size_expected);
    *out_pos = pos_min;
    if corner_norm.x == 0.0 {
        out_pos.x -= (size_constrained.x - size_expected.x);
    }
    if corner_norm.y == 0.0 {
        out_pos.y -= (size_constrained.y - size_expected.y);
    }
    *out_size = size_constrained;
}


pub fn GetResizeBorderRect(window: *mut ImGuiWindow, border_n: c_int, perp_padding: c_float, thickness: c_float) -> ImRect {
    let mut rect: ImRect = window.Rect();
    if thickness == 0.0 {
        rect.Max -= ImVec2::new(1.0, 1.0);
    }
    if border_n == ImGuiDir_Left { return ImRect(rect.Min.x - thickness, rect.Min.y + perp_padding, rect.Min.x + thickness, rect.Max.y - perp_padding); }
    if border_n == ImGuiDir_Right { return ImRect(rect.Max.x - thickness, rect.Min.y + perp_padding, rect.Max.x + thickness, rect.Max.y - perp_padding); }
    if border_n == ImGuiDir_Up { return ImRect(rect.Min.x + perp_padding, rect.Min.y - thickness, rect.Max.x - perp_padding, rect.Min.y + thickness); }
    if border_n == ImGuiDir_Down { return ImRect(rect.Min.x + perp_padding, rect.Max.y - thickness, rect.Max.x - perp_padding, rect.Max.y + thickness); }
    // IM_ASSERT(0);
    return ImRect::default();
}

// 0..3: corners (Lower-right, Lower-left, Unused, Unused)
pub unsafe fn GetWindowResizeCornerID(window: *mut ImGuiWindow, n: c_int) -> ImGuiID {
    // IM_ASSERT(n >= 0 && n < 4);
    let mut id: ImGuiID = if window.DockIsActive { window.DockNode.Hostwindow.ID } else { window.ID };
    id = ImHashStr(str_to_const_c_char_ptr("#RESIZE"), 0, id as u32);
    id = ImHashData(&n, sizeof, id as u32);
    return id;
}


// Borders (Left, Right, Up, Down)
pub unsafe fn GetWindowResizeBorderID(window: *mut ImGuiWindow, dir: ImGuiDir) -> ImGuiID {
    // IM_ASSERT(dir >= 0 && dir < 4);
    let n: c_int = dir + 4;
    let mut id: ImGuiID = if window.DockIsActive { window.DockNode.Hostwindow.ID } else { window.ID };
    id = ImHashStr(str_to_const_c_char_ptr("#RESIZE"), 0, id as u32);
    id = ImHashData(&n, sizeof, id as u32);
    return id;
}



// Handle resize for: Resize Grips, Borders, Gamepad
// Return true when using auto-fit (double click on resize grip)
pub unsafe fn UpdateWindowManualResize(window: *mut ImGuiWindow, size_auto_fit: &ImVec2, border_held:  *mut c_int, resize_grip_count: c_int, mut resize_grip_col: [u32;4], visibility_rect: &ImRect) -> bool
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    flags: ImGuiWindowFlags = window.Flags;

    if (flags & ImGuiWindowFlags_NoResize) || (flags & ImGuiWindowFlags_AlwaysAutoResize) || window.AutoFitFramesX > 0 || window.AutoFitFramesY > 0 {
        return false;
    }
    if window.WasActive == false { // Early out to avoid running this code for e.g. an hidden implicit/fallback Debug window.
        return false;
    }

    let mut ret_auto_fit: bool =  false;
    let resize_border_count: c_int = if g.IO.ConfigWindowsResizeFromEdges { 4 } else { 0 };
    let grip_draw_size: c_float =  IM_FLOOR(ImMax(g.FontSize * 1.35, window.WindowRounding + 1.0 + g.FontSize * 0.20));
    let grip_hover_inner_size: c_float =  IM_FLOOR(grip_draw_size * 0.75);
    let grip_hover_outer_size: c_float =  if g.IO.ConfigWindowsResizeFromEdges { WINDOWS_HOVER_PADDING } else { 0.0 };

    pos_target: ImVec2(f32::MAX, f32::MAX);
    size_target: ImVec2(f32::MAX, f32::MAX);

    // Clip mouse interaction rectangles within the viewport rectangle (in practice the narrowing is going to happen most of the time).
    // - Not narrowing would mostly benefit the situation where OS windows _without_ decoration have a threshold for hovering when outside their limits.
    //   This is however not the case with current backends under Win32, but a custom borderless window implementation would benefit from it.
    // - When decoration are enabled we typically benefit from that distance, but then our resize elements would be conflicting with OS resize elements, so we also narrow.
    // - Note that we are unable to tell if the platform setup allows hovering with a distance threshold (on Win32, decorated window have such threshold).
    // We only clip interaction so we overwrite window.ClipRect, cannot call PushClipRect() yet as DrawList is not yet setup.
    let clip_with_viewport_rect: bool = !(g.IO.BackendFlags & ImGuiBackendFlags_HasMouseHoveredViewport) || (g.IO.MouseHoveredViewport != window.ViewportId) || !(window.Viewport.Flags & ImGuiViewportFlags_NoDecoration);
    if clip_with_viewport_rect {
        window.ClipRect = window.Viewport.GetMainRect().ToVec4();
    }

    // Resize grips and borders are on layer 1
    window.DC.NavLayerCurrent = ImGuiNavLayer_Menu;

    // Manual resize grips
    PushID("#RESIZE");
    // for (let resize_grip_n: c_int = 0; resize_grip_n < resize_grip_count; resize_grip_n++)
    for resize_grip_n in 0 .. resize_grip_count
    {
        let def = resize_grip_def[resize_grip_n];
        let corner: ImVec2 = ImLerp(window.Pos, window.Pos + window.Size, def.CornerPosN);

        // Using the FlattenChilds button flag we make the resize button accessible even if we are hovering over a child window
        let mut hovered = false;
        let mut held = false;
        let mut resize_rect: ImRect = ImRect::new(corner - def.InnerDir * grip_hover_outer_size, corner + def.InnerDir * grip_hover_inner_size);
        if resize_rect.Min.x > resize_rect.Max.x { mem::swap(&mut resize_rect.Min.x, &mut resize_rect.Max.x); }
        if resize_rect.Min.y > resize_rect.Max.y { mem::swap(&mut resize_rect.Min.y, &mut resize_rect.Max.y); }
        let mut resize_grip_id: ImGuiID =  window.GetID3(resize_grip_n); // == GetWindowResizeCornerID()
        KeepAliveID(resize_grip_id);
        ButtonBehavior(resize_rect, resize_grip_id, &hovered, &held, ImGuiButtonFlags_FlattenChildren | ImGuiButtonFlags_NoNavFocus);
        //GetForegroundDrawList(window)->AddRect(resize_rect.Min, resize_rect.Max, IM_COL32(255, 255, 0, 255));
        if hovered || held {
            g.MouseCursor = if resize_grip_n & 1 { ImGuiMouseCursor_ResizeNESW } else { ImGuiMouseCursor_ResizeNWSE };
        }

        if held && g.IO.MouseClickedCount[0] == 2 && resize_grip_n == 0
        {
            // Manual auto-fit when double-clicking
            size_target = CalcWindowSizeAfterConstraint(window, size_auto_fit);
            ret_auto_fit = true;
            ClearActiveID();
        }
        else if held
        {
            // Resize from any of the four corners
            // We don't use an incremental MouseDelta but rather compute an absolute target size based on mouse position
            let clamp_min: ImVec2 = ImVec2::new(if def.CornerPosN.x == 1f32 { visibility_rect.Min.x } else { -f32::MAX }, if def.CornerPosN.y == 1f32 { visibility_rect.Min.y } else { -f32::MAX });
            let clamp_max: ImVec2 = ImVec2::new(if def.CornerPosN.x == 0f32 { visibility_rect.Max.x } else { f32::MAX }, if def.CornerPosN.y == 0f32 { visibility_rect.Max.y } else { f32::MAX });
            let mut corner_target: ImVec2 = g.IO.MousePos - g.ActiveIdClickOffset + ImLerp(def.InnerDir * grip_hover_outer_size, def.InnerDir * -grip_hover_inner_size, def.CornerPosN); // Corner of the window corresponding to our corner grip
            corner_target = ImClamp(corner_target, clamp_min, clamp_max);
            CalcResizePosSizeFromAnyCorner(window, &corner_target, def.CornerPosN, &mut pos_target, &mut size_target);
        }

        // Only lower-left grip is visible before hovering/activating
        if resize_grip_n == 0 || held || hovered {
            resize_grip_col[resize_grip_n] = GetColorU32(if held { ImGuiCol_ResizeGripActive } else {
                if hovered {
                    ImGuiCol_ResizeGripHovered
                } else { ImGuiCol_ResizeGrip }
            }, 0.0);
        }
    }
    // for (let border_n: c_int = 0; border_n < resize_border_count; border_n++)
    for border_n in 0 .. resize_border_count
    {
        let def = resize_border_def[border_n];
        let axis = if (border_n == ImGuiDir_Left || border_n == ImGuiDir_Right) { ImGuiAxis_X } else { ImGuiAxis_Y };

        // hovered: bool, held;
        let mut hovered = false;
        let mut held = false;
        let border_rect: ImRect =  GetResizeBorderRect(window, border_n, grip_hover_inner_size, WINDOWS_HOVER_PADDING);
        let mut border_id: ImGuiID =  window.GetID3(border_n + 4); // == GetWindowResizeBorderID()
        KeepAliveID(border_id);
        ButtonBehavior(border_rect, border_id, &hovered, &held, ImGuiButtonFlags_FlattenChildren | ImGuiButtonFlags_NoNavFocus);
        //GetForegroundDrawLists(window)->AddRect(border_rect.Min, border_rect.Max, IM_COL32(255, 255, 0, 255));
        if ((hovered && g.HoveredIdTimer > WINDOWS_RESIZE_FROM_EDGES_FEEDBACK_TIMER) || held)
        {
            g.MouseCursor = if (axis == ImGuiAxis_X) { ImGuiMouseCursor_ResizeEW } else { ImGuiMouseCursor_ResizeNS };
            if held {
                *border_held = border_n;
            }
        }
        if held
        {
            let clamp_min = ImVec2::new(if border_n == ImGuiDir_Right { visibility_rect.Min.x } else { -f32::MAX }, if border_n == ImGuiDir_Down { visibility_rect.Min.y } else { -f32::MAX });
            let clamp_max = ImVec2::New(if border_n == ImGuiDir_Left { visibility_rect.Max.x } else { f32::MAX }, if border_n == ImGuiDir_Up { visibility_rect.Max.y } else { f32::MAX });
            let mut border_target: ImVec2 = window.Pos;
            border_target[axis] = g.IO.MousePos[axis] - g.ActiveIdClickOffset[axis] + WINDOWS_HOVER_PADDING;
            border_target = ImClamp(border_target, clamp_min, clamp_max);
            CalcResizePosSizeFromAnyCorner(window, &border_target, ImMin(def.SegmentN1, def.SegmentN2), &mut pos_target, &mut size_target);
        }
    }
    PopID();

    // Restore nav layer
    window.DC.NavLayerCurrent = ImGuiNavLayer_Main;

    // Navigation resize (keyboard/gamepad)
    // FIXME: This cannot be moved to NavUpdateWindowing() because CalcWindowSizeAfterConstraint() need to callback into user.
    // Not even sure the callback works here.
    if is_not_null(g.NavWindowingTarget) && g.NavWindowingTarget.RootWindowDockTree == window
    {
        let mut nav_resize_dir = ImVec2::default();
        if g.NavInputSource == ImGuiInputSource_Keyboard && g.IO.KeyShift {
            nav_resize_dir = GetKeyVector2d(ImGuiKey_LeftArrow, ImGuiKey_RightArrow, ImGuiKey_UpArrow, ImGuiKey_DownArrow);
        }
        if g.NavInputSource == ImGuiInputSource_Gamepad {
            nav_resize_dir = GetKeyVector2d(ImGuiKey_GamepadDpadLeft, ImGuiKey_GamepadDpadRight, ImGuiKey_GamepadDpadUp, ImGuiKey_GamepadDpadDown);
        }
        if nav_resize_dir.x != 0f32 || nav_resize_dir.y != 0f32
        {
            let NAV_RESIZE_SPEED: c_float =  600f32;
            let resize_step: c_float =  NAV_RESIZE_SPEED * g.IO.DeltaTime * ImMin(g.IO.DisplayFramebufferScale.x, g.IO.DisplayFramebufferScale.y);
            g.NavWindowingAccumDeltaSize += nav_resize_dir * resize_step;
            g.NavWindowingAccumDeltaSize = ImMax(g.NavWindowingAccumDeltaSize, visibility_rect.Min - window.Pos - window.Size); // We need Pos+Size >= visibility_rect.Min, so Size >= visibility_rect.Min - Pos, so size_delta >= visibility_rect.Min - window.Pos - window.Size
            g.NavWindowingToggleLayer = false;
            g.NavDisableMouseHover = true;
            resize_grip_col[0] = GetColorU32(ImGuiCol_ResizeGripActive, 0.0);
            let accum_floored: ImVec2 = ImFloor(g.NavWindowingAccumDeltaSize);
            if accum_floored.x != 0f32 || accum_floored.y != 0f32
            {
                // FIXME-NAV: Should store and accumulate into a separate size buffer to handle sizing constraints properly, right now a constraint will make us stuck.
                size_target = CalcWindowSizeAfterConstraint(window, window.SizeFull + accum_floored);
                g.NavWindowingAccumDeltaSize -= accum_floored;
            }
        }
    }

    // Apply back modified position/size to window
    if size_target.x != f32::MAX
    {
        window.SizeFull = size_target;
        MarkIniSettingsDirty(window);
    }
    if pos_target.x != f32::MAX
    {
        window.Pos = ImFloor(pos_target);
        MarkIniSettingsDirty(window);
    }

    window.Size = window.SizeFull;
    return ret_auto_fit;
}



pub unsafe fn ClampWindowRect(window: *mut ImGuiWindow, visibility_rect: &ImRect)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut size_for_clamping: ImVec2 = window.Size;
    if g.IO.ConfigWindowsMoveFromTitleBarOnly && (flag_clear(window.Flags, ImGuiWindowFlags_NoTitleBar) || is_not_null(window.DockNodeAsHost)) {
        size_for_clamping.y = GetFrameHeight();
    } // Not using window.TitleBarHeight() as DockNodeAsHost will report 0f32 here.
    window.Pos = ImClamp(window.Pos, visibility_rect.Min - size_for_clamping, visibility_rect.Max);
}



pub unsafe fn RenderWindowOuterBorders(window: *mut ImGuiWindow)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let rounding: c_float =  window.WindowRounding;
    let border_size: c_float =  window.WindowBorderSize;
    if border_size > 0f32 && flag_clear(window.Flags, ImGuiWindowFlags_NoBackground) {
        window.DrawList.AddRect(&window.Pos, window.Pos + window.Size, GetColorU32(ImGuiCol_Border, 0.0), rounding, 0, border_size);
    }

    let border_held = window.ResizeBorderHeld;
    if border_held != -1
    {
        let def = resize_border_def[border_held];
        let border_r: ImRect =  GetResizeBorderRect(window, border_held as c_int, rounding, 0f32);
        window.DrawList.PathArcTo(ImLerp(border_r.Min, border_r.Max, def.SegmentN1) + ImVec2::new(0.5f32, 0.5f32) + def.InnerDir * rounding, rounding, def.OuterAngle - IM_PI * 0.25, def.OuterAngle, 0);
        window.DrawList.PathArcTo(ImLerp(border_r.Min, border_r.Max, def.SegmentN2) + ImVec2::new(0.5f32, 0.5f32) + def.InnerDir * rounding, rounding, def.OuterAngle, def.OuterAngle + IM_PI * 0.25, 0);
        window.DrawList.PathStroke(GetColorU32(ImGuiCol_SeparatorActive, 0.0), 0, ImMax(2.0f32, border_size)); // Thicker than usual
    }
    if g.Style.FrameBorderSize > 0.0 && flag_clear(window.Flags, ImGuiWindowFlags_NoTitleBar) && !window.DockIsActive
    {
        let y: c_float =  window.Pos.y + window.TitleBarHeight() - 1;
        window.DrawList.AddLine(&ImVec2::new(window.Pos.x + border_size, y), &ImVec2::new(window.Pos.x + window.Size.x - border_size, y), GetColorU32(ImGuiCol_Border, 0.0), g.Style.FrameBorderSize);
    }
}



// Draw background and borders
// Draw and handle scrollbars
pub unsafe fn RenderWindowDecorations(window: *mut ImGuiWindow, title_bar_rect: &ImRect, title_bar_is_highlight: bool, handle_borders_and_resize_grips: bool, resize_grip_count: c_int, resize_grip_col: [u32;4],resize_grip_draw_size: c_float)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let style = &mut g.Style;
    let flags: ImGuiWindowFlags = window.Flags;

    // Ensure that ScrollBar doesn't read last frame's SkipItems
    // IM_ASSERT(window.BeginCount == 0);
    window.SkipItems = false;

    // Draw window + handle manual resize
    // As we highlight the title bar when want_focus is set, multiple reappearing windows will have have their title bar highlighted on their reappearing frame.
    let window_rounding: c_float =  window.WindowRounding;
    let window_border_size: c_float =  window.WindowBorderSize;
    if window.Collapsed
    {
        // Title bar only
        let backup_border_size: c_float =  style.FrameBorderSize;
        g.Style.FrameBorderSize = window.WindowBorderSize;
        title_bar_col: u32 = GetColorU32(if title_bar_is_highlight && !g.NavDisableHighlight { ImGuiCol_TitleBgActive } else { ImGuiCol_TitleBgCollapsed }, 0.0);
        RenderFrame(title_bar_rect.Min, title_bar_rect.Max, title_bar_col, true, window_rounding);
        g.Style.FrameBorderSize = backup_border_size;
    }
    else
    {
        // Window background
        if flag_clear(flags, ImGuiWindowFlags_NoBackground)
        {
            let mut is_docking_transparent_payload: bool =  false;
            if g.DragDropActive && (g.FrameCount - g.DragDropAcceptFrameCount) <= 1 && g.IO.ConfigDockingTransparentPayload {
                if g.DragDropPayload.IsDataType(IMGUI_PAYLOAD_TYPE_WINDOW) &&
                g.DragDropPayload.Data == window{
                    is_docking_transparent_payload = true;
                }
            }

            bg_col: u32 = GetColorU32(GetWindowBgColorIdx(window), 0.0);
            if window.ViewportOwned
            {
                // No alpha
                bg_col = (bg_col | IM_COL32_A_MASK);
                if is_docking_transparent_payload {
                    window.Viewport.Alpha *= DOCKING_TRANSPARENT_PAYLOAD_ALPHA;
                }
            }
            else
            {
                // Adjust alpha. For docking
                let mut override_alpha: bool =  false;
                let mut alpha: c_float =  1f32;
                if g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasBgAlpha
                {
                    alpha = g.NextWindowData.BgAlphaVal;
                    override_alpha = true;
                }
                if is_docking_transparent_payload
                {
                    alpha *= DOCKING_TRANSPARENT_PAYLOAD_ALPHA; // FIXME-DOCK: Should that be an override?
                    override_alpha = true;
                }
                if override_alpha {
                    bg_col = (bg_col & !IM_COL32_A_MASK) | (IM_F32_TO_INT8_SAT(alpha) << IM_COL32_A_SHIFT);
                }
            }

            // Render, for docked windows and host windows we ensure bg goes before decorations
            if window.DockIsActive {
                window.DockNode.LastBgColor = bg_col;
            }
            let mut  bg_draw_list: *mut ImDrawList =  if window.DockIsActive { window.DockNode.Hostwindow.DrawList } else { window.DrawList };
            if window.DockIsActive || flag_set(flags, ImGuiWindowFlags_DockNodeHost) {
                bg_draw_list.ChannelsSetCurrent(DOCKING_HOST_DRAW_CHANNEL_BG);
            }
            bg_draw_list.AddRectFilled(window.Pos + ImVec2::new(0.0, window.TitleBarHeight()), window.Pos + window.Size, bg_col, window_rounding, if flag_set(flags, ImGuiWindowFlags_NoTitleBar) { 0 } else { ImDrawFlags_RoundCornersBottom });
            if window.DockIsActive || flag_set(flags, ImGuiWindowFlags_DockNodeHost) {
                bg_draw_list.ChannelsSetCurrent(DOCKING_HOST_DRAW_CHANNEL_FG);
            }
        }
        if window.DockIsActive {
            window.DockNode.IsBgDrawnThisFrame = true;
        }

        // Title bar
        // (when docked, DockNode are drawing their own title bar. Individual windows however do NOT set the _NoTitleBar flag,
        // in order for their pos/size to be matching their undocking state.)
        if flag_clear(flags, ImGuiWindowFlags_NoTitleBar) && !window.DockIsActive
        {
            title_bar_col: u32 = GetColorU32(if title_bar_is_highlight { ImGuiCol_TitleBgActive } else { ImGuiCol_TitleBg }, 0.0);
            window.DrawList.AddRectFilled(&title_bar_rect.Min, &title_bar_rect.Max, title_bar_col, window_rounding, ImDrawFlags_RoundCornersTop);
        }

        // Menu bar
        if flag_set(flags, ImGuiWindowFlags_MenuBar)
        {
            let mut menu_bar_rect: ImRect =  window.MenuBarRect();
            menu_bar_rect.ClipWith(window.Rect());  // Soft clipping, in particular child window don't have minimum size covering the menu bar so this is useful for them.
            window.DrawList.AddRectFilled(menu_bar_rect.Min + ImVec2::new(window_border_size, 0.0), menu_bar_rect.Max - ImVec2::new(window_border_size, 0.0), GetColorU32(ImGuiCol_MenuBarBg, 0.0), if flag_set(flags, ImGuiWindowFlags_NoTitleBar) { window_rounding }else { 0.0 }, ImDrawFlags_RoundCornersTop);
            if style.FrameBorderSize > 0f32 && menu_bar_rect.Max.y < window.Pos.y + window.Size.y {
                window.DrawList.AddLine(&menu_bar_rect.GetBL(), &menu_bar_rect.GetBR(), GetColorU32(ImGuiCol_Border, 0.0), style.FrameBorderSize);
            }
        }

        // Docking: Unhide tab bar (small triangle in the corner), drag from small triangle to quickly undock
        ImGuiDockNode* node = window.DockNode;
        if window.DockIsActive && node.IsHiddenTabBar() && !node.IsNoTabBar()
        {
            let unhide_sz_draw: c_float =  ImFloor(g.FontSize * 0.700f32);
            let unhide_sz_hit: c_float =  ImFloor(g.FontSize * 0.550f32);
            let p: ImVec2 = node.Pos;
            let mut r: ImRect = ImRect::new(p, p + ImVec2::new(unhide_sz_hit, unhide_sz_hit));
            let mut unhide_id: ImGuiID =  window.GetID(str_to_const_c_char_ptr("#UNHIDE"), null());
            KeepAliveID(unhide_id);
            // hovered: bool, held;
            let mut hovered = false;
            let mut held = false;
            if ButtonBehavior(r, unhide_id, &hovered, &held, ImGuiButtonFlags_FlattenChildren) {
                node.WantHiddenTabBarToggle = true;
            }
            else if held && IsMouseDragging(0, 0.0) {
                StartMouseMovingWindowOrNode(window, node, true);
            }

            // FIXME-DOCK: Ideally we'd use ImGuiCol_TitleBgActive/ImGuiCol_TitleBg here, but neither is guaranteed to be visible enough at this sort of size..
            col: u32 = GetColorU32(if (held && hovered) || (node.IsFocused && !hovered) { ImGuiCol_ButtonActive } else {
                if hovered {
                    ImGuiCol_ButtonHovered
                } else { ImGuiCol_Button }
            }, 0.0);
            window.DrawList.AddTriangleFilled(&p, p + ImVec2::new(unhide_sz_draw, 0f32), p + ImVec2::new(0f32, unhide_sz_draw), col);
        }

        // Scrollbars
        if window.ScrollbarX {
            Scrollbar(ImGuiAxis_X);
        }
        if window.ScrollbarY {
            Scrollbar(ImGuiAxis_Y);
        }

        // Render resize grips (after their input handling so we don't have a frame of latency)
        if handle_borders_and_resize_grips && flag_clear(flags, ImGuiWindowFlags_NoResize)
        {
            // for (let resize_grip_n: c_int = 0; resize_grip_n < resize_grip_count; resize_grip_n++)
            for resize_grip_n in 0 .. resize_grip_count
            {
                let grip = resize_grip_def[resize_grip_n];
                let corner: ImVec2 = ImLerp(window.Pos, window.Pos + window.Size, grip.CornerPosN);
                window.DrawList.PathLineTo(corner + grip.InnerDir * (if resize_grip_n & 1 { ImVec2::new(window_border_size, resize_grip_draw_size) } else { ImVec2::new(resize_grip_draw_size, window_border_size) }));
                window.DrawList.PathLineTo(corner + grip.InnerDir * (if resize_grip_n & 1 { ImVec2::new(resize_grip_draw_size, window_border_size) } else { ImVec2::new(window_border_size, resize_grip_draw_size) }));
                window.DrawList.PathArcToFast(&ImVec2::new(corner.x + grip.InnerDir.x * (window_rounding + window_border_size), corner.y + grip.InnerDir.y * (window_rounding + window_border_size)), window_rounding, grip.AngleMin12, grip.AngleMax12);
                window.DrawList.PathFillConvex(resize_grip_col[resize_grip_n]);
            }
        }

        // Borders (for dock node host they will be rendered over after the tab bar)
        if handle_borders_and_resize_grips && is_null(window.DockNodeAsHost) {
            RenderWindowOuterBorders(window);
        }
    }
}
