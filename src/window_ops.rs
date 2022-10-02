#![allow(non_snake_case)]

use std::ptr::null_mut;
use libc::{c_float, c_int, c_void};
use crate::color::{IM_COL32_A_MASK, ImGuiCol_ModalWindowDimBg, ImGuiCol_NavWindowingDimBg, ImGuiCol_NavWindowingHighlight};
use crate::draw_flags::ImDrawFlags_None;
use crate::draw_list::ImDrawList;
use crate::draw_list_ops::GetForegroundDrawList;
use crate::hovered_flags::{ImGuiHoveredFlags, ImGuiHoveredFlags_AllowWhenBlockedByPopup};
use crate::imgui::GImGui;
use crate::ImGuiViewport;
use crate::input_source::{ImGuiInputSource_Mouse, ImGuiInputSource_Nav};
use crate::rect::ImRect;
use crate::style_ops::GetColorU32;
use crate::type_defs::ImGuiID;
use crate::vec2::ImVec2;
use crate::window::ImGuiWindow;
use crate::window_flags::{ImGuiWindowFlags_AlwaysAutoResize, ImGuiWindowFlags_ChildWindow, ImGuiWindowFlags_Modal, ImGuiWindowFlags_NoMouseInputs, ImGuiWindowFlags_NoResize, ImGuiWindowFlags_Popup, ImGuiWindowFlags_Tooltip};

// [Internal] Small optimization to avoid calls to PopClipRect/SetCurrentChannel/PushClipRect in sequences,
// they would meddle many times with the underlying ImDrawCmd.
// Instead, we do a preemptive overwrite of clipping rectangle _without_ altering the command-buffer and let
// the subsequent single call to SetCurrentChannel() does it things once.
// c_void SetWindowClipRectBeforeSetChannel(*mut ImGuiWindow window, const ImRect& clip_rect)
pub fn SetWindowClipRectBeforeSetChannel(window: *mut ImGuiWindow, clip_rect: &ImRect) {
    let mut clip_rect_vec4 = clip_rect.ToVec4();
    window.ClipRect = clip_rect.clone();
    window.DrawList._CmdHeader.ClipRect = clip_rect_vec4;
    window.DrawList._ClipRectStack[window.DrawList._ClipRectStack.Size - 1] = clip_rect_vec4.clone();
}


// inline ImRect           WindowRectRelToAbs(*mut ImGuiWindow window, const ImRect& r) 
pub fn WindowRectRelToAbs(window: *mut ImGuiWindow, r: &ImRect) -> ImRect {
    let off = window.DC.CursorStartPos.clone();
    ImRect::new4(r.Min.x + off.x, r.Min.y + off.y, r.Max.x + off.x, r.Max.y + off.y)
}


// inline ImRect           WindowRectAbsToRel(*mut ImGuiWindow window, const ImRect& r)
pub fn WindowRectAbsToRel(window: *mut ImGuiWindow, r: &ImRect) -> ImRect {
    let mut off: ImVec2 = window.DC.CursorStartPos.clone();
    return ImRect::new4(r.Min.x - off.x, r.Min.y - off.y, r.Max.x - off.x, r.Max.y - off.y);
}

// static c_void SetCurrentWindow(ImGuiWindow* window)
pub unsafe fn SetCurrentWindow(window: *mut ImGuiWindow) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.CurrentWindow = window;
    g.CurrentTable = if window.is_null() == false && window.DC.CurrentTableIdx != -1 { g.Tables.GetByIndex(window.DC.CurrentTableIdx) } else { null_mut() };
    if window {
        g.FontSize = window.CalcFontSize();
        g.DrawListSharedData.FontSize = window.CalcFontSize();
    }
}

// static inline bool IsWindowContentHoverable(ImGuiWindow* window, ImGuiHoveredFlags flags)
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
                if (focused_root_window.Flags & ImGuiWindowFlags_Popup) && !(flags & ImGuiHoveredFlags_AllowWhenBlockedByPopup) {
                    return false;
                }
            }
        }
    }
    // Filter by viewport
    if window.Viewport != g.MouseViewport {
        if g.MovingWindow == null_mut() || window.RootWindowDockTree != g.MovingWindow.RootWindowDockTree {
            return false;
        }
    }

    return true;
}


// This is called during NewFrame()->UpdateViewportsNewFrame() only.
// Need to keep in sync with SetWindowPos()
// static c_void TranslateWindow(ImGuiWindow* window, const ImVec2& delta)
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

// static c_void ScaleWindow(ImGuiWindow* window, c_float scale)
pub fn ScaleWindow(window: *mut ImGuiWindow, scale: c_float) {
    let origin: ImVec2 = window.Viewport.Pos.clone();
    window.Pos = ImFloor((window.Pos.clone() - origin) * scale + origin.clone());
    window.Size = ImFloor(window.Size.clone() * scale);
    window.SizeFull = ImFloor(window.SizeFull.clone() * scale);
    window.ContentSize = ImFloor(window.ContentSize.clone() * scale);
}

// static bool IsWindowActiveAndVisible(ImGuiWindow* window)
pub fn IsWindowActiveAndVisible(window: *mut ImGuiWindow) -> bool {
    return (window.Active) && (!window.Hidden);
}


// FIXME: Add a more explicit sort order in the window structure.
// static c_int IMGUI_CDECL ChildWindowComparer(*const c_void lhs, *const c_void rhs)
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

// static c_void AddWindowToSortBuffer(Vec<ImGuiWindow*>* out_sorted_windows, ImGuiWindow* window)
pub fn AddWindowToSortBuffer(mut out_sorted_windows: *mut Vec<*mut ImGuiWindow>, window: *mut ImGuiWindow) {
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


// static inline c_int GetWindowDisplayLayer(ImGuiWindow* window)
pub fn GetWindowDisplayLayer(window: *mut ImGuiWindow) -> c_int {
    return if window.Flags & ImGuiWindowFlags_Tooltip { 1 } else { 0 };
}


// Push a clipping rectangle for both ImGui logic (hit-testing etc.) and low-level ImDrawList rendering.
// - When using this function it is sane to ensure that float are perfectly rounded to integer values,
//   so that e.g. (max.x-min.x) in user's render produce correct result.
// - If the code here changes, may need to update code of functions like NextColumn() and PushColumnClipRect():
//   some frequently called functions which to modify both channels and clipping simultaneously tend to use the
//   more specialized SetWindowClipRectBeforeSetChannel() to avoid extraneous updates of underlying ImDrawCmds.
// c_void PushClipRect(const ImVec2& clip_rect_min, const ImVec2& clip_rect_max, bool intersect_with_current_clip_rect)
pub fn PushClipRect(clip_rect_min: &ImVec2, clip_rect_max: &ImVec2, intersect_with_current_clip_rect: bool) {
    let mut window: *mut ImGuiWindow = GetCurrentWindow();
    window.DrawList.PushClipRect(clip_rect_min, clip_rect_max, intersect_with_current_clip_rect);
    window.ClipRect = window.DrawList._ClipRectStack.last().unwrap().clone();
}

// c_void PopClipRect()
pub fn PopClipRect() {
    let mut window: *mut ImGuiWindow = GetCurrentWindow();
    window.DrawList.PopClipRect();
    window.ClipRect = window.DrawList._ClipRectStack.last().unwrap().clone();
}

// static ImGuiWindow* FindFrontMostVisibleChildWindow(ImGuiWindow* window)
pub fn FindFrontMostVisibleChildWindow(window: *mut ImGuiWindow) -> *mut ImGuiWindow {
    // for (let n: c_int = window.DC.ChildWindows.Size - 1; n >= 0; n--)
    for n in window.DC.ChildWindows.len() - 1..0 {
        if IsWindowActiveAndVisible(window.DC.ChildWindows[n]) {
            return FindFrontMostVisibleChildWindow(window.DC.ChildWindows[n]);
        }
    }
    return window;
}

// static c_void RenderDimmedBackgroundBehindWindow(ImGuiWindow* window, u32 col)
pub fn RenderDimmedBackgroundBehindWindow(window: *mut ImGuiWindow, col: u32) {
    if (col & IM_COL32_A_MASK) == 0 {
        return;
    }

    let mut viewport: *mut ImGuiViewport = window.Viewport;
    let viewport_rect: ImRect = viewport.GetMainRect();

    // Draw behind window by moving the draw command at the FRONT of the draw list
    {
        // We've already called AddWindowToDrawData() which called DrawList.ChannelsMerge() on DockNodeHost windows,
        // and draw list have been trimmed already, hence the explicit recreation of a draw command if missing.
        // FIXME: This is creating complication, might be simpler if we could inject a drawlist in drawdata at a given position and not attempt to manipulate ImDrawCmd order.
        let mut draw_list: *mut ImDrawList = window.RootWindowDockTree.DrawList;
        if draw_list.CmdBuffer.len() == 0 {
            draw_list.AddDrawCmd();
        }
        draw_list.PushClipRect(viewport_rect.Min - ImVec2(1, 1), viewport_rect.Max + ImVec2(1, 1), false); // Ensure ImDrawCmd are not merged
        draw_list.AddRectFilled(&viewport_rect.Min, &viewport_rect.Max, col, 0f32, ImDrawFlags_None);
        let cmd = draw_list.CmdBuffer.last().unwrap();
        // IM_ASSERT(cmd.ElemCount == 6);
        draw_list.CmdBuffer.pop_back();
        draw_list.CmdBuffer.push_front(cmd);
        draw_list.PopClipRect();
        draw_list.AddDrawCmd(); // We need to create a command as CmdBuffer.back().IdxOffset won't be correct if we append to same command.
    }

    // Draw over sibling docking nodes in a same docking tree
    if window.Rootwindow.DockIsActive {
        let mut draw_list: *mut ImDrawList = FindFrontMostVisibleChildWindow(window.RootWindowDockTree).DrawList;
        if draw_list.CmdBuffer.Size == 0 {
            draw_list.AddDrawCmd();
        }
        draw_list.PushClipRect(&viewport_rect.Min, &viewport_rect.Max, false);
        RenderRectFilledWithHole(draw_list, window.RootWindowDockTree.Rect(), window.Rootwindow.Rect(), col, 0f32);// window.RootWindowDockTree->WindowRounding);
        draw_list.PopClipRect();
    }
}

// ImGuiWindow* FindBottomMostVisibleWindowWithinBeginStack(ImGuiWindow* parent_window)
pub unsafe fn FindBottomMostVisibleWindowWithBeginStack(parent_window: *mut ImGuiWindow) -> *mut ImGuiWindow {
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
        if IsWindowActiveAndVisible(window) && GetWindowDisplayLayer(window) <= GetWindowDisplayLayer(parent_window) {
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
    let dim_bg_for_window_list: bool = (g.NavWindowingTargetAnim != null_mut() && g.NavWindowingTargetAnim.Active);
    if !dim_bg_for_modal && !dim_bg_for_window_list {
        return;
    }

    let mut viewports_already_dimmed: [*mut ImGuiViewport; 2] = [null_mut(), null_mut()];
    if dim_bg_for_modal {
        // Draw dimming behind modal or a begin stack child, whichever comes first in draw order.
        let mut dim_behind_window: *mut ImGuiWindow = FindBottomMostVisibleWindowWithinBeginStack(modal_window);
        RenderDimmedBackgroundBehindWindow(dim_behind_window, GetColorU32(ImGuiCol_ModalWindowDimBg, g.DimBgRatio));
        viewports_already_dimmed[0] = modal_window.Viewport;
    } else if dim_bg_for_window_list {
        // Draw dimming behind CTRL+Tab target window and behind CTRL+Tab UI window
        RenderDimmedBackgroundBehindWindow(g.NavWindowingTargetAnim, GetColorU32(ImGuiCol_NavWindowingDimBg, g.DimBgRatio));
        if g.NavWindowingListWindow != null_mut() && g.NavWindowingListwindow.Viewport && g.NavWindowingListwindow.Viewport != g.NavWindowingTargetAnim.Viewport {
            RenderDimmedBackgroundBehindWindow(g.NavWindowingListWindow, GetColorU32(ImGuiCol_NavWindowingDimBg, g.DimBgRatio));
        }
        viewports_already_dimmed[0] = g.NavWindowingTargetAnim.Viewport;
        viewports_already_dimmed[1] = if g.NavWindowingListWindow { g.NavWindowingListwindow.Viewport } else { null_mut() };

        // Draw border around CTRL+Tab target window
        let mut window: *mut ImGuiWindow = g.NavWindowingTargetAnim;
        ImGuiViewport * viewport = window.Viewport;
        let distance: c_float = g.FontSize;
        let mut bb: ImRect = window.Rect();
        bb.Expand(distance);
        if bb.GetWidth() >= viewport.Size.x && bb.GetHeight() >= viewport.Size.y {
            bb.Expand(-distance - 1f32);
        } // If a window fits the entire viewport, adjust its highlight inward
        if window.DrawList.CmdBuffer.Size == 0 {
            window.DrawList.AddDrawCmd();
        }
        window.DrawList.PushClipRect(viewport.Pos, viewport.Pos + viewport.Size, false);
        window.DrawList.AddRect(&bb.Min, &bb.Max, GetColorU32(ImGuiCol_NavWindowingHighlight, g.NavWindowingHighlightAlpha), window.WindowRounding, 0, 3.00f32);
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
        let dim_bg_col = GetColorU32(if dim_bg_for_modal { ImGuiCol_ModalWindowDimBg } else { ImGuiCol_NavWindowingDimBg }, g.DimBgRatio);
        draw_list.AddRectFilled(&viewport.Pos, viewport.Pos.clone() + viewport.Size.clone(), dim_bg_col, 0f32, ImDrawFlags_None);
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
    let mut moving_window_viewport: *mut ImGuiViewport = if !(g.MovingWindow.is_null()) { g.Movingwindow.Viewport } else { null_mut() };
    if g.MovingWindow {
        g.Movingwindow.Viewport = g.MouseViewport;
    }

    let mut hovered_window: *mut ImGuiWindow = null_mut();
    let mut hovered_window_ignoring_moving_window: *mut ImGuiWindow = null_mut();
    if g.MovingWindow && !(g.Movingwindow.Flags & ImGuiWindowFlags_NoMouseInputs) {
        hovered_window = g.MovingWindow;
    }

    let padding_regular: ImVec2 = g.Style.TouchExtraPadding.clone();
    let padding_for_resize: ImVec2 = if g.IO.ConfigWindowsResizeFromEdges { g.WindowsHoverPadding.clone() } else { padding_regular };
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
        let mut bb: ImRect = ImRect::new3(window.OuterRectClipped.into());
        if window.Flags & (ImGuiWindowFlags_ChildWindow | ImGuiWindowFlags_NoResize | ImGuiWindowFlags_AlwaysAutoResize) {
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
            let hole_pos = ImVec2::new2(window.Pos.x + window.HitTestHoleOffset.x, window.Pos.y + window.HitTestHoleOffset.y);
            let hole_size = ImVec2::new2(window.HitTestHoleSize.x as c_float, window.HitTestHoleSize.y as c_float);
            if ImRect(hole_pos.clone(), hole_pos.clone() + hole_size).Contains(g.IO.MousePos.clone()) {
                continue;
            }
        }

        if hovered_window == null_mut() {
            hovered_window = window;
        }
        IM_MSVC_WARNING_SUPPRESS(28182); // [Static Analyzer] Dereferencing NULL pointer.
        if hovered_window_ignoring_moving_window == null_mut() && (g.MovingWindow.is_null() || window.RootWindowDockTree != g.Movingwindow.RootWindowDockTree) {
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
