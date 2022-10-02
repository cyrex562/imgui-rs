#![allow(non_snake_case)]

use std::ptr::{null, null_mut};
use libc::{c_char, c_float, c_int, c_void};
use crate::color::{IM_COL32_A_MASK, IM_COL32_A_SHIFT, ImGuiCol, ImGuiCol_Border, ImGuiCol_Button, ImGuiCol_ButtonActive, ImGuiCol_ButtonHovered, ImGuiCol_ChildBg, ImGuiCol_MenuBarBg, ImGuiCol_ModalWindowDimBg, ImGuiCol_NavWindowingDimBg, ImGuiCol_NavWindowingHighlight, ImGuiCol_PopupBg, ImGuiCol_SeparatorActive, ImGuiCol_TitleBg, ImGuiCol_TitleBgActive, ImGuiCol_TitleBgCollapsed, ImGuiCol_WindowBg};
use crate::condition::{ImGuiCond, ImGuiCond_FirstUseEver};
use crate::draw_flags::{ImDrawFlags_None, ImDrawFlags_RoundCornersBottom, ImDrawFlags_RoundCornersTop};
use crate::draw_list::ImDrawList;
use crate::draw_list_ops::GetForegroundDrawList;
use crate::hovered_flags::{ImGuiHoveredFlags, ImGuiHoveredFlags_AllowWhenBlockedByPopup};
use crate::imgui::GImGui;
use crate::{ImGuiViewport, ImHashStr};
use crate::axis::{ImGuiAxis_X, ImGuiAxis_Y};
use crate::context::ImGuiContext;
use crate::id_ops::KeepAliveID;
use crate::input_ops::IsMouseDragging;
use crate::input_source::{ImGuiInputSource_Mouse, ImGuiInputSource_Nav};
use crate::mouse_ops::StartMouseMovingWindowOrNode;
use crate::rect::ImRect;
use crate::render_ops::RenderFrame;
use crate::resize_border_def::resize_border_def;
use crate::resize_grip_def::resize_grip_def;
use crate::resize_ops::GetResizeBorderRect;
use crate::size_callback_data::ImGuiSizeCallbackData;
use crate::string_ops::str_to_const_c_char_ptr;
use crate::style::ImGuiStyle;
use crate::style_ops::GetColorU32;
use crate::type_defs::{ImGuiID, ImGuisizeCallback};
use crate::utils::{flag_clear, flag_set};
use crate::vec2::ImVec2;
use crate::window::ImGuiWindow;
use crate::window_flags::{ImGuiWindowFlags, ImGuiWindowFlags_AlwaysAutoResize, ImGuiWindowFlags_AlwaysHorizontalScrollbar, ImGuiWindowFlags_AlwaysVerticalScrollbar, ImGuiWindowFlags_ChildMenu, ImGuiWindowFlags_ChildWindow, ImGuiWindowFlags_DockNodeHost, ImGuiWindowFlags_HorizontalScrollbar, ImGuiWindowFlags_MenuBar, ImGuiWindowFlags_Modal, ImGuiWindowFlags_NoBackground, ImGuiWindowFlags_NoBringToFrontOnFocus, ImGuiWindowFlags_NoMouseInputs, ImGuiWindowFlags_NoResize, ImGuiWindowFlags_NoSavedSettings, ImGuiWindowFlags_NoScrollbar, ImGuiWindowFlags_NoTitleBar, ImGuiWindowFlags_Popup, ImGuiWindowFlags_Tooltip};
use crate::window_settings::ImGuiWindowSettings;

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
        RenderDimmedBackgroundBehindWindow(dim_behind_window, GetColorU32(ImGuiCol_ModalWindowDimBg, g.DimBgRatio, ));
        viewports_already_dimmed[0] = modal_window.Viewport;
    } else if dim_bg_for_window_list {
        // Draw dimming behind CTRL+Tab target window and behind CTRL+Tab UI window
        RenderDimmedBackgroundBehindWindow(g.NavWindowingTargetAnim, GetColorU32(ImGuiCol_NavWindowingDimBg, g.DimBgRatio, ));
        if g.NavWindowingListWindow != null_mut() && g.NavWindowingListwindow.Viewport && g.NavWindowingListwindow.Viewport != g.NavWindowingTargetAnim.Viewport {
            RenderDimmedBackgroundBehindWindow(g.NavWindowingListWindow, GetColorU32(ImGuiCol_NavWindowingDimBg, g.DimBgRatio, ));
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
        window.DrawList.AddRect(&bb.Min, &bb.Max, GetColorU32(ImGuiCol_NavWindowingHighlight, g.NavWindowingHighlightAlpha, ), window.WindowRounding, 0, 3.00f32);
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
        let dim_bg_col = GetColorU32(if dim_bg_for_modal { ImGuiCol_ModalWindowDimBg } else { ImGuiCol_NavWindowingDimBg }, g.DimBgRatio, );
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


// static c_void SetWindowConditionAllowFlags(ImGuiWindow* window, ImGuiCond flags, bool enabled)
pub fn SsetWindowConditionAllowFlags(window: *mut ImGuiWindow, flags: ImGuiCond, enabled: bool) {
    window.SetWindowPosAllowFlags = if enabled { (window.SetWindowPosAllowFlags | flags) } else { window.SetWindowPosAllowFlags & !flags };
    window.SetWindowSizeAllowFlags = if enabled { (window.SetWindowSizeAllowFlags | flags) } else { window.SetWindowSizeAllowFlags & !flags };
    window.SetWindowCollapsedAllowFlags = if enabled { (window.SetWindowCollapsedAllowFlags | flags) } else { window.SetWindowCollapsedAllowFlags & !flags };
    window.SetWindowDockAllowFlags = if enabled { (window.SetWindowDockAllowFlags | flags) } else { window.SetWindowDockAllowFlags & !flags };
}

// ImGuiWindow* FindWindowByID(ImGuiID id)
pub unsafe fn FindWindowByID(id: ImGuiID) -> *mut ImGuiWindow
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.WindowsById.GetVoidPtr(id) as *mut ImGuiWindow;
}

// ImGuiWindow* FindWindowByName(*const char name)
pub unsafe fn FindWindowByName(name: *const c_char) -> *mut ImGuiWindow
{
    let mut id: ImGuiID =  ImHashStr2(name);
    return FindWindowByID(id);
}

// static c_void ApplyWindowSettings(ImGuiWindow* window, ImGuiWindowSettings* settings)
pub fn ApplyWindowSettings(window: *mut ImGuiWindow, settings: *mut ImGuiWindowSettings)
{
    let main_viewport: *const ImGuiViewport = GetMainViewport();
    window.ViewportPos = main_viewport.Pos.clone();
    if settings.ViewportId
    {
        window.ViewportId = settings.ViewportId;
        window.ViewportPos = ImVec2(settings.ViewportPos.x, settings.ViewportPos.y);
    }
    window.Pos = ImFloor(ImVec2(settings.Pos.x + window.ViewportPos.x, settings.Pos.y + window.ViewportPos.y));
    if settings.Size.x > 0 && settings.Size.y > 0 {
        window.SizeFull = ImFloor(ImVec2(settings.Size.x, settings.Size.y));

        window.Size = window.SizeFull.clone();
    }
    window.Collapsed = settings.Collapsed;
    window.DockId = settings.DockId;
    window.DockOrder = settings.DockOrder;
}

// static c_void UpdateWindowInFocusOrderList(ImGuiWindow* window, bool just_created, ImGuiWindowFlags new_flags)
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

// static ImGuiWindow* CreateNewWindow(*const char name, ImGuiWindowFlags flags)
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
    window.Pos = main_viewport.Pos.clone() + ImVec2(60, 60);
    window.ViewportPos = main_viewport.Pos.clone();

    // User can disable loading and saving of settings. Tooltip and child windows also don't store settings.
    if !flag_set(flags, ImGuiWindowFlags_NoSavedSettings) {
        if ImGuiWindowSettings * settings = FindWindowSettings(window.ID) {
            // Retrieve settings from .ini file
            window.SettingsOffset = g.SettingsWindows.offset_from_ptr(settings);
            SetWindowConditionAllowFlags(window, ImGuiCond_FirstUseEver, false);
            ApplyWindowSettings(window, settings);
        }
    }
    window.DC.CursorStartPos = window.Pos.clone();
    window.DC.CursorMaxPos = window.Pos.clone();
    window.DC.IdealMaxPos = window.Pos.clone(); // So first call to CalcWindowContentSizes() doesn't return crazy values

    if (flags & ImGuiWindowFlags_AlwaysAutoResize) != 0
    {
        window.AutoFitFramesX = 2;
        window.AutoFitFramesY = 2;
        window.AutoFitOnlyGrows = false;
    }
    else
    {
        if window.Size.x <= 0f32 {
            window.AutoFitFramesX = 2;
        }
        if window.Size.y <= 0f32 {
            window.AutoFitFramesY = 2;
        }
        window.AutoFitOnlyGrows = (window.AutoFitFramesX > 0) || (window.AutoFitFramesY > 0);
    }

    if flag_set(flags, ImGuiWindowFlags_NoBringToFrontOnFocus) {
        g.Windows.push_front(window);
    } // Quite slow but rare and only once
    else {
        g.Windows.push(window);
    }

    return window;
}

// static ImGuiWindow* GetWindowForTitleDisplay(ImGuiWindow* window)
pub fn GetWindowForTitleDisplay(window: *mut ImGuiWindow) -> *mut ImGuiWindow
{
    return if window.DockNodeAsHost { window.DockNodeAsHost.VisibleWindow } else { window };
}

// static ImGuiWindow* GetWindowForTitleAndMenuHeight(ImGuiWindow* window)
pub fn GetWindowForTitleAndMenuHeight(window: *mut ImGuiWindow) -> *mut ImGuiWindow
{
    return if window.DockNodeAsHost.is_null() == false && window.DockNodeAsHost.VisibleWindow.is_null == false { window.DockNodeAsHost.VisibleWindow } else { window };
}

// static ImVec2 CalcWindowSizeAfterConstraint(ImGuiWindow* window, const ImVec2& size_desired)
pub unsafe fn CalcWindowSizeAfterConstraint(window: *mut ImGuiWindow, size_desired: &ImVec2) -> ImVec2
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut new_size: ImVec2 = size_desired.clone();
    if flag_set(g.NextWindowData.Flags, ImGuiNextWindowDataFlags_HasSizeConstraint)
    {
        // Using -1,-1 on either X/Y axis to preserve the current size.
        let cr: ImRect =  g.NextWindowData.SizeConstraintRect.clone();
        new_size.x = if cr.Min.x >= 0f32 && cr.Max.x >= 0f32 { ImClamp(new_size.x, cr.Min.x, cr.Max.x) }else { window.SizeFull.x };
        new_size.y = if cr.Min.y >= 0f32 && cr.Max.y >= 0f32 { ImClamp(new_size.y, cr.Min.y, cr.Max.y) } else { window.SizeFull.y };
        if g.NextWindowData.SizeCallback
        {
            // ImGuiSizeCallbackData data;
            let mut data: ImGuisizeCallbackData = ImGuiSizeCallbackData::new();
            data.UserData = g.NextWindowData.SizeCallbackUserData;
            data.Pos = window.Pos.clone();
            data.CurrentSize = window.SizeFull.clone();
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
        let decoration_up_height: c_float =  () + ();
        new_size = ImMax(new_size, g.Style.WindowMinSize.clone());
        new_size.y = ImMax(new_size.y, decoration_up_height + ImMax(0f32, g.Style.WindowRounding - 1f32)); // Reduce artifacts with very small windows
    }
    return new_size;
}

// static c_void CalcWindowContentSizes(ImGuiWindow* window, ImVec2* content_size_current, ImVec2* content_size_ideal)
pub unsafe fn CalcWindowContextSizes(window: *mut ImGuiWindow, content_size_current: *mut ImVec2)
{
    let mut preserve_old_content_sizes: bool =  false;
    if window.Collapsed && window.AutoFitFramesX <= 0 && window.AutoFitFramesY <= 0 {
        preserve_old_content_sizes = true;
    }
    else if window.Hidden && window.HiddenFramesCannotSkipItems == 0 && window.HiddenFramesCanSkipItems > 0 {
        preserve_old_content_sizes = true;
    }
    if preserve_old_content_sizes
    {
        *content_size_current = window.ContentSize.clone();
        *content_size_ideal = window.ContentSizeIdeal.clone();
        return;
    }

     // = (window.ContentSizeExplicit.x != 0f32) ? window.ContentSizeExplicit.x : IM_FLOOR(window.DC.CursorMaxPos.x - window.DC.CursorStartPos.x);
     // = (window.ContentSizeExplicit.y != 0f32) ? window.ContentSizeExplicit.y : IM_FLOOR(window.DC.CursorMaxPos.y - window.DC.CursorStartPos.y);
     // = (window.ContentSizeExplicit.x != 0f32) ? window.ContentSizeExplicit.x : IM_FLOOR(ImMax(window.DC.CursorMaxPos.x, window.DC.IdealMaxPos.x) - window.DC.CursorStartPos.x);
     // = (window.ContentSizeExplicit.y != 0f32) ? window.ContentSizeExplicit.y : IM_FLOOR(ImMax(window.DC.CursorMaxPos.y, window.DC.IdealMaxPos.y) - window.DC.CursorStartPos.y);
}

// static ImVec2 CalcWindowAutoFitSize(ImGuiWindow* window, const ImVec2& size_contents)
pub unsafe fn CalcWIndowAutoFitSize(window: *mut ImGuiWindow, size_contents: &ImVec2) -> ImVec2
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let style = &mut g.Style;
    let decoration_up_height: c_float =  window.TitleBarHeight() + window.MenuBarHeight();
    let size_pad: ImVec2 = window.WindowPadding * 2.0f32;
    let size_desired: ImVec2 = size_contents + size_pad + ImVec2::new2(0f32, decoration_up_height);
    if flag_set(window.Flags, ImGuiWindowFlags_Tooltip)
    {
        // Tooltip always resize
        return size_desired;
    }
    else
    {
        // Maximum window size is determined by the viewport size or monitor size
        let is_popup: bool = flag_set(window.Flags, ImGuiWindowFlags_Popup);
        let is_menu: bool = flag_set(window.Flags, ImGuiWindowFlags_ChildMenu);
        let mut size_min: ImVec2 = style.WindowMinSize;
        if is_popup || is_menu { // Popups and menus bypass style.WindowMinSize by default, but we give then a non-zero minimum size to facilitate understanding problematic cases (e.g. empty popups)
            size_min = ImMin(size_min, ImVec2(4.0f32, 4.00f32));
        }

        // FIXME-VIEWPORT-WORKAREA: May want to use GetWorkSize() instead of Size depending on the type of windows?
        let mut avail_size: ImVec2 = window.Viewport.Size;
        if window.ViewportOwned {
            avail_size = ImVec2(f32::MAX, f32::MAX);
        }
        let monitor_idx: c_int = window.ViewportAllowPlatformMonitorExtend;
        if (monitor_idx >= 0 && monitor_idx < g.PlatformIO.Monitors.Size) {
            avail_size = g.PlatformIO.Monitors[monitor_idx].WorkSize;
        }
        let mut size_auto_fit: ImVec2 = ImClamp(size_desired, size_min, ImMax(size_min, avail_size - style.DisplaySafeAreaPadding * 2.00f32));

        // When the window cannot fit all contents (either because of constraints, either because screen is too small),
        // we are growing the size on the other axis to compensate for expected scrollbar. FIXME: Might turn bigger than ViewportSize-WindowPadding.
        let size_auto_fit_after_constraint: ImVec2 = CalcWindowSizeAfterConstraint(window, &size_auto_fit);
        let mut will_have_scrollbar_x: bool =  (size_auto_fit_after_constraint.x - size_pad.x - 0f32                 < size_contents.x && flag_clear(window.Flags, ImGuiWindowFlags_NoScrollbar) && flag_set(window.Flags, ImGuiWindowFlags_HorizontalScrollbar)) || flag_set(window.Flags, ImGuiWindowFlags_AlwaysHorizontalScrollbar);
        let mut will_have_scrollbar_y: bool =  (size_auto_fit_after_constraint.y - size_pad.y - decoration_up_height < size_contents.y && flag_clear(window.Flags, ImGuiWindowFlags_NoScrollbar)) || flag_set(window.Flags, ImGuiWindowFlags_AlwaysVerticalScrollbar);
        if will_have_scrollbar_x {
            size_auto_fit.y += style.ScrollbarSize;
        }
        if will_have_scrollbar_y {
            size_auto_fit.x += style.ScrollbarSize;
        }
        return size_auto_fit;
    }
}

// ImVec2 CalcWindowNextAutoFitSize(ImGuiWindow* window)
pub unsafe fn CalcWindowNextAutoFitSize(window: *mut ImGuiWindow) -> ImVec2
{
    let mut size_contents_current = ImVec2::new();
    let mut size_contents_ideal = ImVec2::new();
    CalcWindowContentSizes(window, &size_contents_current, &size_contents_ideal);
    let size_auto_fit: ImVec2 = CalcWindowAutoFitSize(window, size_contents_ideal);
    let size_final: ImVec2 = CalcWindowSizeAfterConstraint(window, &size_auto_fit);
    return size_final;
}

// static ImGuiCol GetWindowBgColorIdx(ImGuiWindow* window)
pub unsafe fn GetWindowBgColorIdx(window: *mut ImGuiWindow) -> ImGuiCol
{
    if (window.Flags & (ImGuiWindowFlags_Tooltip | ImGuiWindowFlags_Popup)) {
        return ImGuiCol_PopupBg;
    }
    if (flag_set(window.Flags, ImGuiWindowFlags_ChildWindow) && !window.DockIsActive) {
        return ImGuiCol_ChildBg;
    }
    return ImGuiCol_WindowBg;
}


// static inline c_void ClampWindowRect(ImGuiWindow* window, const ImRect& visibility_rect)
pub unsafe fn ClampWindowRect(window: *mut ImGuiWindow, visibility_rect: &ImRect)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut size_for_clamping: ImVec2 = window.Size;
    if g.IO.ConfigWindowsMoveFromTitleBarOnly && (flag_clear(window.Flags, ImGuiWindowFlags_NoTitleBar) || window.DockNodeAsHost.is_null() == false) {
        size_for_clamping.y = GetFrameHeight();
    } // Not using window.TitleBarHeight() as DockNodeAsHost will report 0f32 here.
    window.Pos = ImClamp(window.Pos, visibility_rect.Min - size_for_clamping, visibility_rect.Max);
}

// static c_void RenderWindowOuterBorders(ImGuiWindow* window)
pub unsafe fn RenderWindowOuterBorders(window: *mut ImGuiWindow)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let rounding: c_float =  window.WindowRounding;
    let border_size: c_float =  window.WindowBorderSize;
    if border_size > 0f32 && flag_clear(window.Flags, ImGuiWindowFlags_NoBackground) {
        window.DrawList.AddRect(&window.Pos, window.Pos + window.Size, GetColorU32(ImGuiCol_Border, 0f32, ), rounding, 0, border_size)
    };

    let border_held: c_int = window.ResizeBorderHeld.clone() as c_int;
    if border_held != -1
    {
       let def = resize_border_def[border_held];
        let border_r: ImRect =  GetResizeBorderRect(window, border_held, rounding, 0f32);
        window.DrawList.PathArcTo(ImLerp(border_r.Min, border_r.Max, def.SegmentN1) + ImVec2(0.5f32, 0.5f32) + def.InnerDir * rounding, rounding, def.OuterAngle - IM_PI * 0.25f32, def.OuterAngle, 0);
        window.DrawList.PathArcTo2(ImLerp(border_r.Min, border_r.Max, def.SegmentN2) + ImVec2(0.5f32, 0.5f32) + def.InnerDir * rounding, rounding, def.OuterAngle, def.OuterAngle + IM_PI * 0.250f32);
        window.DrawList.PathStroke(GetColorU32(ImGuiCol_SeparatorActive, 0f32, ), 0, ImMax(2.0f32, border_size)); // Thicker than usual
    }
    if g.Style.FrameBorderSize > 0f32 && flag_clear(window.Flags, ImGuiWindowFlags_NoTitleBar) && !window.DockIsActive
    {
        let y: c_float =  window.Pos.y + window.TitleBarHeight() - 1;
        window.DrawList.AddLine(ImVec2(window.Pos.x + border_size, y), ImVec2(window.Pos.x + window.Size.x - border_size, y), GetColorU32(ImGuiCol_Border, 0f32, ), g.Style.FrameBorderSize);
    }
}

// Draw background and borders
// Draw and handle scrollbars
// c_void RenderWindowDecorations(ImGuiWindow* window, const ImRect& title_bar_rect, bool title_bar_is_highlight, bool handle_borders_and_resize_grips, c_int resize_grip_count, const u32 resize_grip_col[4], c_float resize_grip_draw_size)
pub unsafe fn RenderWindowDecorations(window: *mut ImGuiWindow, title_bar_rec: &ImRect, title_bar_is_highlight: bool, handle_borders_and_resize_grips: bool, resize_grip_count: c_int, resize_grip_col: [u32;4], resize_grip_draw_size: c_float)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let style = &mut g.Style;
    let flags = window.Flags;

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
        let title_bar_col = GetColorU32(if title_bar_is_highlight && !g.NavDisableHighlight { ImGuiCol_TitleBgActive } else { ImGuiCol_TitleBgCollapsed }, 0f32, );
        RenderFrame(title_bar_rect.Min, title_bar_rect.Max, title_bar_col, true, window_rounding);
        g.Style.FrameBorderSize = backup_border_size;
    }
    else
    {
        // Window background
        if !(flags & ImGuiWindowFlags_NoBackground)
        {
            let mut is_docking_transparent_payload: bool =  false;
            if g.DragDropActive && (g.FrameCount - g.DragDropAcceptFrameCount) <= 1 && g.IO.ConfigDockingTransparentPayload {
                if g.DragDropPayload.IsDataType(IMGUI_PAYLOAD_TYPE_WINDOW) &&
                g.DragDropPayload.Data == window {
                    is_docking_transparent_payload = true;
                }
            }

            let mut bg_col = GetColorU32(GetWindowBgColorIdx(window), 0f32, );
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
                if flag_set(g.NextWindowData.Flags, ImGuiNextWindowDataFlags_HasBgAlpha)
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
            bg_draw_list.AddRectFilled(window.Pos + ImVec2(0, window.TitleBarHeight()), window.Pos + window.Size, bg_col, window_rounding, if flag_set(flags, ImGuiWindowFlags_NoTitleBar) { 0 } else { ImDrawFlags_RoundCornersBottom });
            if window.DockIsActive || flag_set(flags, ImGuiWindowFlags_DockNodeHost) {
                bg_draw_list.ChannelsSetCurrent(DOCKING_HOST_DRAW_CHANNEL_FG);
            }
        }
        if (window.DockIsActive) {
            window.DockNode.IsBgDrawnThisFrame = true;
        }

        // Title bar
        // (when docked, DockNode are drawing their own title bar. Individual windows however do NOT set the _NoTitleBar flag,
        // in order for their pos/size to be matching their undocking state.)
        if flag_clear(flags, ImGuiWindowFlags_NoTitleBar) && !window.DockIsActive
        {
            let title_bar_col = GetColorU32(if title_bar_is_highlight { ImGuiCol_TitleBgActive } else { ImGuiCol_TitleBg }, 0f32, );
            window.DrawList.AddRectFilled(title_bar_rect.Min, title_bar_rect.Max, title_bar_col, window_rounding, ImDrawFlags_RoundCornersTop);
        }

        // Menu bar
        if flag_set(flags, ImGuiWindowFlags_MenuBar)
        {
            let mut menu_bar_rect: ImRect =  window.MenuBarRect();
            menu_bar_rect.ClipWith(window.Rect());  // Soft clipping, in particular child window don't have minimum size covering the menu bar so this is useful for them.
            window.DrawList.AddRectFilled(menu_bar_rect.Min + ImVec2(window_border_size, 0), menu_bar_rect.Max - ImVec2(window_border_size, 0), GetColorU32(ImGuiCol_MenuBarBg, 0f32, ), if flag_set(flags, ImGuiWindowFlags_NoTitleBar) { window_rounding } else { 0f32 }, ImDrawFlags_RoundCornersTop);
            if style.FrameBorderSize > 0f32 && menu_bar_rect.Max.y < window.Pos.y + window.Size.y {
                window.DrawList.AddLine(&(menu_bar_rect.GetBL()), &(menu_bar_rect.GetBR()), GetColorU32(ImGuiCol_Border, 0f32, ), style.FrameBorderSize);
            }
        }

        // Docking: Unhide tab bar (small triangle in the corner), drag from small triangle to quickly undock
        let node = window.DockNode;
        if window.DockIsActive && node.IsHiddenTabBar() && !node.IsNoTabBar()
        {
            let unhide_sz_draw: c_float =  ImFloor(g.FontSize * 0.700f32);
            let unhide_sz_hit: c_float =  ImFloor(g.FontSize * 0.550f32);
            let p: ImVec2 = node.Pos;
            let mut r: ImRect = ImRect::new2(&p, p + ImVec2(unhide_sz_hit, unhide_sz_hit));
            let mut unhide_id: ImGuiID =  window.GetID(str_to_const_c_char_ptr("#UNHIDE"),null() );
            KeepAliveID(unhide_id);
            // bool hovered, held;
            let mut hovered = false;
            let mut held = false

            if ButtonBehavior(r, unhide_id, &hovered, &held, ImGuiButtonFlags_FlattenChildren) {
                node.WantHiddenTabBarToggle = true;
            }
            else if held && IsMouseDragging(0) {
                StartMouseMovingWindowOrNode(window, node, true);
            }

            // FIXME-DOCK: Ideally we'd use ImGuiCol_TitleBgActive/ImGuiCol_TitleBg here, but neither is guaranteed to be visible enough at this sort of size..
            let col = GetColorU32(if (held && hovered) || (node.IsFocused && !hovered) {
                ImGuiCol_ButtonActive} else { if hovered                 {ImGuiCol_ButtonHovered
            } else { ImGuiCol_Button }}, 0f32);
            window.DrawList.AddTriangleFilled(p, p + ImVec2(unhide_sz_draw, 0f32), p + ImVec2(0f32, unhide_sz_draw), col);
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
                window.DrawList.PathLineTo(corner + grip.InnerDir * (if resize_grip_n & 1 { ImVec2::new2(window_border_size, resize_grip_draw_size) } else { ImVec2::new2(resize_grip_draw_size, window_border_size) }));
                window.DrawList.PathLineTo(corner + grip.InnerDir * (if resize_grip_n & 1 { ImVec2::new2(resize_grip_draw_size, window_border_size) } else { ImVec2::new2(window_border_size, resize_grip_draw_size) }));
                window.DrawList.PathArcToFast(ImVec2(corner.x + grip.InnerDir.x * (window_rounding + window_border_size), corner.y + grip.InnerDir.y * (window_rounding + window_border_size)), window_rounding, grip.AngleMin12, grip.AngleMax12);
                window.DrawList.PathFillConvex(resize_grip_col[resize_grip_n]);
            }
        }

        // Borders (for dock node host they will be rendered over after the tab bar)
        if handle_borders_and_resize_grips && window.DockNodeAsHost.is_null() {
            RenderWindowOuterBorders(window);
        }
    }
}


// inline    * mut ImGuiWindow  GetCurrentWindowRead()
pub unsafe fn GetCurrentWindowRead() -> *mut ImGuiWindow {
    let g = GImGui; //
// ImGuiContext& g = *GImGui;
    return g.CurrentWindow;
}

// inline * mut ImGuiWindow  GetCurrentWindow()
pub unsafe fn GetCurrentWindow() -> *mut ImGuiWindow {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.Currentwindow.WriteAccessed = true;
    return g.CurrentWindow;
}

// Render title text, collapse button, close button
// When inside a dock node, this is handled in DockNodeCalcTabBarLayout() instead.
c_void RenderWindowTitleBarContents(ImGuiWindow* window, const ImRect& title_bar_rect, *const char name, bool* p_open)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiStyle& style = g.Style;
    ImGuiWindowFlags flags = window.Flags;

    let has_close_button: bool = (p_open != null_mut());
    let has_collapse_button: bool = !(flags & ImGuiWindowFlags_NoCollapse) && (style.WindowMenuButtonPosition != ImGuiDir_None);

    // Close & Collapse button are on the Menu NavLayer and don't default focus (unless there's nothing else on that layer)
    // FIXME-NAV: Might want (or not?) to set the equivalent of ImGuiButtonFlags_NoNavFocus so that mouse clicks on standard title bar items don't necessarily set nav/keyboard ref?
    const let mut item_flags_backup: ImGuiItemFlags =  g.CurrentItemFlags;
    g.CurrentItemFlags |= ImGuiItemFlags_NoNavDefaultFocus;
    window.DC.NavLayerCurrent = ImGuiNavLayer_Menu;

    // Layout buttons
    // FIXME: Would be nice to generalize the subtleties expressed here into reusable code.
    let pad_l: c_float =  style.FramePadding.x;
    let pad_r: c_float =  style.FramePadding.x;
    let button_sz: c_float =  g.FontSize;
    ImVec2 close_button_pos;
    ImVec2 collapse_button_pos;
    if (has_close_button)
    {
        pad_r += button_sz;
        close_button_pos = ImVec2(title_bar_rect.Max.x - pad_r - style.FramePadding.x, title_bar_rect.Min.y);
    }
    if (has_collapse_button && style.WindowMenuButtonPosition == ImGuiDir_Right)
    {
        pad_r += button_sz;
        collapse_button_pos = ImVec2(title_bar_rect.Max.x - pad_r - style.FramePadding.x, title_bar_rect.Min.y);
    }
    if (has_collapse_button && style.WindowMenuButtonPosition == ImGuiDir_Left)
    {
        collapse_button_pos = ImVec2(title_bar_rect.Min.x + pad_l - style.FramePadding.x, title_bar_rect.Min.y);
        pad_l += button_sz;
    }

    // Collapse button (submitting first so it gets priority when choosing a navigation init fallback)
    if (has_collapse_button)
        if (CollapseButton(window.GetID("#COLLAPSE"), collapse_button_pos, null_mut()))
            window.WantCollapseToggle = true; // Defer actual collapsing to next frame as we are too far in the Begin() function

    // Close button
    if (has_close_button)
        if (CloseButton(window.GetID("#CLOSE"), close_button_pos))
            *p_open = false;

    window.DC.NavLayerCurrent = ImGuiNavLayer_Main;
    g.CurrentItemFlags = item_flags_backup;

    // Title bar text (with: horizontal alignment, avoiding collapse/close button, optional "unsaved document" marker)
    // FIXME: Refactor text alignment facilities along with RenderText helpers, this is WAY too much messy code..
    let marker_size_x: c_float =  (flags & ImGuiWindowFlags_UnsavedDocument) ? button_sz * 0.80f32 : 0f32;
    let text_size: ImVec2 = CalcTextSize(name, null_mut(), true) + ImVec2(marker_size_x, 0f32);

    // As a nice touch we try to ensure that centered title text doesn't get affected by visibility of Close/Collapse button,
    // while uncentered title text will still reach edges correctly.
    if (pad_l > style.FramePadding.x)
        pad_l += g.Style.ItemInnerSpacing.x;
    if (pad_r > style.FramePadding.x)
        pad_r += g.Style.ItemInnerSpacing.x;
    if (style.WindowTitleAlign.x > 0f32 && style.WindowTitleAlign.x < 1f32)
    {
        let centerness: c_float =  ImSaturate(1f32 - ImFabs(style.WindowTitleAlign.x - 0.5f32) * 2.00f32); // 0f32 on either edges, 1f32 on center
        let pad_extend: c_float =  ImMin(ImMax(pad_l, pad_r), title_bar_rect.GetWidth() - pad_l - pad_r - text_size.x);
        pad_l = ImMax(pad_l, pad_extend * centerness);
        pad_r = ImMax(pad_r, pad_extend * centerness);
    }

    let mut layout_r: ImRect = ImRect::new(title_bar_rect.Min.x + pad_l, title_bar_rect.Min.y, title_bar_rect.Max.x - pad_r, title_bar_rect.Max.y);
    let mut clip_r: ImRect = ImRect::new(layout_r.Min.x, layout_r.Min.y, ImMin(layout_r.Max.x + g.Style.ItemInnerSpacing.x, title_bar_rect.Max.x), layout_r.Max.y);
    if (flags & ImGuiWindowFlags_UnsavedDocument)
    {
        ImVec2 marker_pos;
        marker_pos.x = ImClamp(layout_r.Min.x + (layout_r.GetWidth() - text_size.x) * style.WindowTitleAlign.x + text_size.x, layout_r.Min.x, layout_r.Max.x);
        marker_pos.y = (layout_r.Min.y + layout_r.Max.y) * 0.5f32;
        if (marker_pos.x > layout_r.Min.x)
        {
            RenderBullet(window.DrawList, marker_pos, GetColorU32(ImGuiCol_Text));
            clip_r.Max.x = ImMin(clip_r.Max.x, marker_pos.x - (marker_size_x * 0.5f32));
        }
    }
    //if (g.IO.KeyShift) window.DrawList.AddRect(layout_r.Min, layout_r.Max, IM_COL32(255, 128, 0, 255)); // [DEBUG]
    //if (g.IO.KeyCtrl) window.DrawList.AddRect(clip_r.Min, clip_r.Max, IM_COL32(255, 128, 0, 255)); // [DEBUG]
    RenderTextClipped(layout_r.Min, layout_r.Max, name, null_mut(), &text_size, style.WindowTitleAlign, &clip_r);
}

c_void UpdateWindowParentAndRootLinks(ImGuiWindow* window, ImGuiWindowFlags flags, ImGuiWindow* parent_window)
{
    window.ParentWindow = parent_window;
    window.RootWindow = window.RootWindowPopupTree = window.RootWindowDockTree = window.RootWindowForTitleBarHighlight = window.RootWindowForNav = window;
    if (parent_window && (flags & ImGuiWindowFlags_ChildWindow) && !(flags & ImGuiWindowFlags_Tooltip))
    {
        window.RootWindowDockTree = parent_window.RootWindowDockTree;
        if (!window.DockIsActive && !(parent_window.Flags & ImGuiWindowFlags_DockNodeHost))
            window.RootWindow = parent_window.RootWindow;
    }
    if (parent_window && (flags & ImGuiWindowFlags_Popup))
        window.RootWindowPopupTree = parent_window.RootWindowPopupTree;
    if (parent_window && !(flags & ImGuiWindowFlags_Modal) && (flags & (ImGuiWindowFlags_ChildWindow | ImGuiWindowFlags_Popup))) // FIXME: simply use _NoTitleBar ?
        window.RootWindowForTitleBarHighlight = parent_window.RootWindowForTitleBarHighlight;
    while (window.RootWindowForNav->Flags & ImGuiWindowFlags_NavFlattened)
    {
        // IM_ASSERT(window.RootWindowForNav->ParentWindow != NULL);
        window.RootWindowForNav = window.RootWindowForNav->ParentWindow;
    }
}


c_void BringWindowToFocusFront(ImGuiWindow* window)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(window == window.RootWindow);

    let cur_order: c_int = window.FocusOrder;
    // IM_ASSERT(g.WindowsFocusOrder[cur_order] == window);
    if (g.WindowsFocusOrder.last().unwrap() == window)
        return;

    let new_order: c_int = g.WindowsFocusOrder.Size - 1;
    for (let n: c_int = cur_order; n < new_order; n++)
    {
        g.WindowsFocusOrder[n] = g.WindowsFocusOrder[n + 1];
        g.WindowsFocusOrder[n]->FocusOrder-= 1;
        // IM_ASSERT(g.WindowsFocusOrder[n]->FocusOrder == n);
    }
    g.WindowsFocusOrder[new_order] = window;
    window.FocusOrder = new_order;
}

c_void BringWindowToDisplayFront(ImGuiWindow* window)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut current_front_window: *mut ImGuiWindow =  g.Windows.last().unwrap();
    if (current_front_window == window || current_front_window.RootWindowDockTree == window) // Cheap early out (could be better)
        return;
    for (let i: c_int = g.Windows.len() - 2; i >= 0; i--) // We can ignore the top-most window
        if (g.Windows[i] == window)
        {
            memmove(&g.Windows[i], &g.Windows[i + 1], (g.Windows.len() - i - 1) * sizeof(ImGuiWindow*));
            g.Windows[g.Windows.len() - 1] = window;
            break;
        }
}

c_void BringWindowToDisplayBack(ImGuiWindow* window)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.Windows[0] == window)
        return;
    for (let i: c_int = 0; i < g.Windows.len(); i++)
        if (g.Windows[i] == window)
        {
            memmove(&g.Windows[1], &g.Windows[0], i * sizeof(ImGuiWindow*));
            g.Windows[0] = window;
            break;
        }
}

c_void BringWindowToDisplayBehind(ImGuiWindow* window, ImGuiWindow* behind_window)
{
    // IM_ASSERT(window != NULL && behind_window != NULL);
    let g = GImGui; // ImGuiContext& g = *GImGui;
    window = window.RootWindow;
    behind_window = behind_window.RootWindow;
    let pos_wnd: c_int = FindWindowDisplayIndex(window);
    let pos_beh: c_int = FindWindowDisplayIndex(behind_window);
    if (pos_wnd < pos_beh)
    {
        size_t copy_bytes = (pos_beh - pos_wnd - 1) * sizeof(ImGuiWindow*);
        memmove(&g.Windows.Data[pos_wnd], &g.Windows.Data[pos_wnd + 1], copy_bytes);
        g.Windows[pos_beh - 1] = window;
    }
    else
    {
        size_t copy_bytes = (pos_wnd - pos_beh) * sizeof(ImGuiWindow*);
        memmove(&g.Windows.Data[pos_beh + 1], &g.Windows.Data[pos_beh], copy_bytes);
        g.Windows[pos_beh] = window;
    }
}

c_int FindWindowDisplayIndex(ImGuiWindow* window)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.Windows.index_from_ptr(g.Windows.find(window));
}

// Moving window to front of display and set focus (which happens to be back of our sorted list)
c_void FocusWindow(ImGuiWindow* window)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    if (g.NavWindow != window)
    {
        SetNavWindow(window);
        if (window && g.NavDisableMouseHover)
            g.NavMousePosDirty = true;
        g.NavId = window ? window.NavLastIds[0] : 0; // Restore NavId
        g.NavLayer = ImGuiNavLayer_Main;
        g.NavFocusScopeId = 0;
        g.NavIdIsAlive = false;
    }

    // Close popups if any
    ClosePopupsOverWindow(window, false);

    // Move the root window to the top of the pile
    // IM_ASSERT(window == NULL || window.RootWindowDockTree != NULL);
    let mut focus_front_window: *mut ImGuiWindow =  window ? window.RootWindow : null_mut();
    let mut display_front_window: *mut ImGuiWindow =  window ? window.RootWindowDockTree : null_mut();
    ImGuiDockNode* dock_node = window ? window.DockNode : null_mut();
    let mut active_id_window_is_dock_node_host: bool =  (g.ActiveIdWindow && dock_node && dock_node.HostWindow == g.ActiveIdWindow);

    // Steal active widgets. Some of the cases it triggers includes:
    // - Focus a window while an InputText in another window is active, if focus happens before the old InputText can run.
    // - When using Nav to activate menu items (due to timing of activating on press->new window appears->losing ActiveId)
    // - Using dock host items (tab, collapse button) can trigger this before we redirect the ActiveIdWindow toward the child window.
    if (g.ActiveId != 0 && g.ActiveIdWindow && g.ActiveIdwindow.RootWindow != focus_front_window)
        if (!g.ActiveIdNoClearOnFocusLoss && !active_id_window_is_dock_node_host)
            ClearActiveID();

    // Passing NULL allow to disable keyboard focus
    if (!window)
        return;
    window.LastFrameJustFocused = g.FrameCount;

    // Select in dock node
    if (dock_node && dock_node.TabBar)
        dock_node.TabBar->SelectedTabId = dock_node.TabBar->NextSelectedTabId = window.TabId;

    // Bring to front
    BringWindowToFocusFront(focus_front_window);
    if (((window.Flags | focus_front_window.Flags | display_front_window.Flags) & ImGuiWindowFlags_NoBringToFrontOnFocus) == 0)
        BringWindowToDisplayFront(display_front_window);
}

c_void FocusTopMostWindowUnderOne(ImGuiWindow* under_this_window, ImGuiWindow* ignore_window)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let start_idx: c_int = g.WindowsFocusOrder.Size - 1;
    if (under_this_window != null_mut())
    {
        // Aim at root window behind us, if we are in a child window that's our own root (see #4640)
        let offset: c_int = -1;
        while (under_this_window.Flags & ImGuiWindowFlags_ChildWindow)
        {
            under_this_window = under_this_window.ParentWindow;
            offset = 0;
        }
        start_idx = FindWindowFocusIndex(under_this_window) + offset;
    }
    for (let i: c_int = start_idx; i >= 0; i--)
    {
        // We may later decide to test for different NoXXXInputs based on the active navigation input (mouse vs nav) but that may feel more confusing to the user.
        let mut window: *mut ImGuiWindow =  g.WindowsFocusOrder[i];
        // IM_ASSERT(window == window.RootWindow);
        if (window != ignore_window && window.WasActive)
            if ((window.Flags & (ImGuiWindowFlags_NoMouseInputs | ImGuiWindowFlags_NoNavInputs)) != (ImGuiWindowFlags_NoMouseInputs | ImGuiWindowFlags_NoNavInputs))
            {
                // FIXME-DOCK: This is failing (lagging by one frame) for docked windows.
                // If A and B are docked into window and B disappear, at the NewFrame() call site window.NavLastChildNavWindow will still point to B.
                // We might leverage the tab order implicitly stored in window.DockNodeAsHost->TabBar (essentially the 'most_recently_selected_tab' code in tab bar will do that but on next update)
                // to tell which is the "previous" window. Or we may leverage 'LastFrameFocused/LastFrameJustFocused' and have this function handle child window itself?
                let mut focus_window: *mut ImGuiWindow =  NavRestoreLastChildNavWindow(window);
                FocusWindow(focus_window);
                return;
            }
    }
    FocusWindow(null_mut());
}


bool IsWindowChildOf(ImGuiWindow* window, ImGuiWindow* potential_parent, bool popup_hierarchy, bool dock_hierarchy)
{
    let mut window_root: *mut ImGuiWindow =  GetCombinedRootWindow(window, popup_hierarchy, dock_hierarchy);
    if (window_root == potential_parent)
        return true;
    while (window != null_mut())
    {
        if (window == potential_parent)
            return true;
        if (window == window_root) // end of chain
            return false;
        window = window.ParentWindow;
    }
    return false;
}

bool IsWindowWithinBeginStackOf(ImGuiWindow* window, ImGuiWindow* potential_parent)
{
    if (window.RootWindow == potential_parent)
        return true;
    while (window != null_mut())
    {
        if (window == potential_parent)
            return true;
        window = window.ParentWindowInBeginStack;
    }
    return false;
}

bool IsWindowAbove(ImGuiWindow* potential_above, ImGuiWindow* potential_below)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    // It would be saner to ensure that display layer is always reflected in the g.Windows[] order, which would likely requires altering all manipulations of that array
    let display_layer_delta: c_int = GetWindowDisplayLayer(potential_above) - GetWindowDisplayLayer(potential_below);
    if (display_layer_delta != 0)
        return display_layer_delta > 0;

    for (let i: c_int = g.Windows.len() - 1; i >= 0; i--)
    {
        let mut candidate_window: *mut ImGuiWindow =  g.Windows[i];
        if (candidate_window == potential_above)
            return true;
        if (candidate_window == potential_below)
            return false;
    }
    return false;
}

bool IsWindowHovered(ImGuiHoveredFlags flags)
{
    // IM_ASSERT((flags & (ImGuiHoveredFlags_AllowWhenOverlapped | ImGuiHoveredFlags_AllowWhenDisabled)) == 0);   // Flags not supported by this function
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut ref_window: *mut ImGuiWindow =  g.HoveredWindow;
    let mut cur_window: *mut ImGuiWindow =  g.CurrentWindow;
    if (ref_window == null_mut())
        return false;

    if ((flags & ImGuiHoveredFlags_AnyWindow) == 0)
    {
        // IM_ASSERT(cur_window); // Not inside a Begin()/End()
        let popup_hierarchy: bool = (flags & ImGuiHoveredFlags_NoPopupHierarchy) == 0;
        let dock_hierarchy: bool = (flags & ImGuiHoveredFlags_DockHierarchy) != 0;
        if (flags & ImGuiHoveredFlags_RootWindow)
            cur_window = GetCombinedRootWindow(cur_window, popup_hierarchy, dock_hierarchy);

        bool result;
        if (flags & ImGuiHoveredFlags_ChildWindows)
            result = IsWindowChildOf(ref_window, cur_window, popup_hierarchy, dock_hierarchy);
        else
            result = (ref_window == cur_window);
        if (!result)
            return false;
    }

    if (!IsWindowContentHoverable(ref_window, flags))
        return false;
    if (!(flags & ImGuiHoveredFlags_AllowWhenBlockedByActiveItem))
        if (g.ActiveId != 0 && !g.ActiveIdAllowOverlap && g.ActiveId != ref_window.MoveId)
            return false;
    return true;
}

bool IsWindowFocused(ImGuiFocusedFlags flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut ref_window: *mut ImGuiWindow =  g.NavWindow;
    let mut cur_window: *mut ImGuiWindow =  g.CurrentWindow;

    if (ref_window == null_mut())
        return false;
    if (flags & ImGuiFocusedFlags_AnyWindow)
        return true;

    // IM_ASSERT(cur_window); // Not inside a Begin()/End()
    let popup_hierarchy: bool = (flags & ImGuiFocusedFlags_NoPopupHierarchy) == 0;
    let dock_hierarchy: bool = (flags & ImGuiFocusedFlags_DockHierarchy) != 0;
    if (flags & ImGuiHoveredFlags_RootWindow)
        cur_window = GetCombinedRootWindow(cur_window, popup_hierarchy, dock_hierarchy);

    if (flags & ImGuiHoveredFlags_ChildWindows)
        return IsWindowChildOf(ref_window, cur_window, popup_hierarchy, dock_hierarchy);
    else
        return (ref_window == cur_window);
}

ImGuiID GetWindowDockID()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.Currentwindow.DockId;
}

bool IsWindowDocked()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.Currentwindow.DockIsActive;
}

// Can we focus this window with CTRL+TAB (or PadMenu + PadFocusPrev/PadFocusNext)
// Note that NoNavFocus makes the window not reachable with CTRL+TAB but it can still be focused with mouse or programmatically.
// If you want a window to never be focused, you may use the e.g. NoInputs flag.
bool IsWindowNavFocusable(ImGuiWindow* window)
{
    return window.WasActive && window == window.RootWindow && !(window.Flags & ImGuiWindowFlags_NoNavFocus);
}

c_float GetWindowWidth()
{
    let mut window: *mut ImGuiWindow =  GimGui.CurrentWindow;
    return window.Size.x;
}

c_float GetWindowHeight()
{
    let mut window: *mut ImGuiWindow =  GimGui.CurrentWindow;
    return window.Size.y;
}

ImVec2 GetWindowPos()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    return window.Pos;
}

c_void SetWindowPos(ImGuiWindow* window, const ImVec2& pos, ImGuiCond cond)
{
    // Test condition (NB: bit 0 is always true) and clear flags for next time
    if (cond && (window.SetWindowPosAllowFlags & cond) == 0)
        return;

    // IM_ASSERT(cond == 0 || ImIsPowerOfTwo(cond)); // Make sure the user doesn't attempt to combine multiple condition flags.
    window.SetWindowPosAllowFlags &= ~(ImGuiCond_Once | ImGuiCond_FirstUseEver | ImGuiCond_Appearing);
    window.SetWindowPosVal = ImVec2(f32::MAX, f32::MAX);

    // Set
    let old_pos: ImVec2 = window.Pos;
    window.Pos = ImFloor(pos);
    let offset: ImVec2 = window.Pos - old_pos;
    if (offset.x == 0f32 && offset.y == 0f32)
        return;
    MarkIniSettingsDirty(window);
    // FIXME: share code with TranslateWindow(), need to confirm whether the 3 rect modified by TranslateWindow() are desirable here.
    window.DC.CursorPos += offset;         // As we happen to move the window while it is being appended to (which is a bad idea - will smear) let's at least offset the cursor
    window.DC.CursorMaxPos += offset;      // And more importantly we need to offset CursorMaxPos/CursorStartPos this so ContentSize calculation doesn't get affected.
    window.DC.IdealMaxPos += offset;
    window.DC.CursorStartPos += offset;
}

c_void SetWindowPos(const ImVec2& pos, ImGuiCond cond)
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindowRead();
    SetWindowPos(window, pos, cond);
}

c_void SetWindowPos(*const char name, const ImVec2& pos, ImGuiCond cond)
{
    if (let mut window: *mut ImGuiWindow =  FindWindowByName(name))
        SetWindowPos(window, pos, cond);
}

ImVec2 GetWindowSize()
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindowRead();
    return window.Size;
}

c_void SetWindowSize(ImGuiWindow* window, const ImVec2& size, ImGuiCond cond)
{
    // Test condition (NB: bit 0 is always true) and clear flags for next time
    if (cond && (window.SetWindowSizeAllowFlags & cond) == 0)
        return;

    // IM_ASSERT(cond == 0 || ImIsPowerOfTwo(cond)); // Make sure the user doesn't attempt to combine multiple condition flags.
    window.SetWindowSizeAllowFlags &= ~(ImGuiCond_Once | ImGuiCond_FirstUseEver | ImGuiCond_Appearing);

    // Set
    let old_size: ImVec2 = window.SizeFull;
    window.AutoFitFramesX = (size.x <= 0f32) ? 2 : 0;
    window.AutoFitFramesY = (size.y <= 0f32) ? 2 : 0;
    if (size.x <= 0f32)
        window.AutoFitOnlyGrows = false;
    else
        window.SizeFull.x = IM_FLOOR(size.x);
    if (size.y <= 0f32)
        window.AutoFitOnlyGrows = false;
    else
        window.SizeFull.y = IM_FLOOR(size.y);
    if (old_size.x != window.SizeFull.x || old_size.y != window.SizeFull.y)
        MarkIniSettingsDirty(window);
}

c_void SetWindowSize(const ImVec2& size, ImGuiCond cond)
{
    SetWindowSize(GimGui.CurrentWindow, size, cond);
}

c_void SetWindowSize(*const char name, const ImVec2& size, ImGuiCond cond)
{
    if (let mut window: *mut ImGuiWindow =  FindWindowByName(name))
        SetWindowSize(window, size, cond);
}

c_void SetWindowCollapsed(ImGuiWindow* window, bool collapsed, ImGuiCond cond)
{
    // Test condition (NB: bit 0 is always true) and clear flags for next time
    if (cond && (window.SetWindowCollapsedAllowFlags & cond) == 0)
        return;
    window.SetWindowCollapsedAllowFlags &= ~(ImGuiCond_Once | ImGuiCond_FirstUseEver | ImGuiCond_Appearing);

    // Set
    window.Collapsed = collapsed;
}

c_void SetWindowHitTestHole(ImGuiWindow* window, const ImVec2& pos, const ImVec2& size)
{
    // IM_ASSERT(window.HitTestHoleSize.x == 0);     // We don't support multiple holes/hit test filters
    window.HitTestHoleSize = ImVec2ih(size);
    window.HitTestHoleOffset = ImVec2ih(pos - window.Pos);
}

c_void SetWindowCollapsed(bool collapsed, ImGuiCond cond)
{
    SetWindowCollapsed(GimGui.CurrentWindow, collapsed, cond);
}

bool IsWindowCollapsed()
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindowRead();
    return window.Collapsed;
}

bool IsWindowAppearing()
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindowRead();
    return window.Appearing;
}

c_void SetWindowCollapsed(*const char name, bool collapsed, ImGuiCond cond)
{
    if (let mut window: *mut ImGuiWindow =  FindWindowByName(name))
        SetWindowCollapsed(window, collapsed, cond);
}

c_void SetWindowFocus()
{
    FocusWindow(GimGui.CurrentWindow);
}

c_void SetWindowFocus(*const char name)
{
    if (name)
    {
        if (let mut window: *mut ImGuiWindow =  FindWindowByName(name))
            FocusWindow(window);
    }
    else
    {
        FocusWindow(null_mut());
    }
}

c_void SetNextWindowPos(const ImVec2& pos, ImGuiCond cond, const ImVec2& pivot)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(cond == 0 || ImIsPowerOfTwo(cond)); // Make sure the user doesn't attempt to combine multiple condition flags.
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasPos;
    g.NextWindowData.PosVal = pos;
    g.NextWindowData.PosPivotVal = pivot;
    g.NextWindowData.PosCond = cond ? cond : ImGuiCond_Always;
    g.NextWindowData.PosUndock = true;
}

c_void SetNextWindowSize(const ImVec2& size, ImGuiCond cond)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(cond == 0 || ImIsPowerOfTwo(cond)); // Make sure the user doesn't attempt to combine multiple condition flags.
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasSize;
    g.NextWindowData.SizeVal = size;
    g.NextWindowData.SizeCond = cond ? cond : ImGuiCond_Always;
}

c_void SetNextWindowSizeConstraints(const ImVec2& size_min, const ImVec2& size_max, ImGuiSizeCallback custom_callback, custom_callback_user_data: *mut c_void)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasSizeConstraint;
    g.NextWindowData.SizeConstraintRect = ImRect(size_min, size_max);
    g.NextWindowData.SizeCallback = custom_callback;
    g.NextWindowData.SizeCallbackUserData = custom_callback_user_data;
}

// Content size = inner scrollable rectangle, padded with WindowPadding.
// SetNextWindowContentSize(ImVec2(100,100) + ImGuiWindowFlags_AlwaysAutoResize will always allow submitting a 100x100 item.
c_void SetNextWindowContentSize(const ImVec2& size)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasContentSize;
    g.NextWindowData.ContentSizeVal = ImFloor(size);
}

c_void SetNextWindowScroll(const ImVec2& scroll)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasScroll;
    g.NextWindowData.ScrollVal = scroll;
}

c_void SetNextWindowCollapsed(bool collapsed, ImGuiCond cond)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(cond == 0 || ImIsPowerOfTwo(cond)); // Make sure the user doesn't attempt to combine multiple condition flags.
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasCollapsed;
    g.NextWindowData.CollapsedVal = collapsed;
    g.NextWindowData.CollapsedCond = cond ? cond : ImGuiCond_Always;
}

c_void SetNextWindowFocus()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasFocus;
}

c_void SetNextWindowBgAlpha(c_float alpha)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasBgAlpha;
    g.NextWindowData.BgAlphaVal = alpha;
}

c_void SetNextWindowViewport(ImGuiID id)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasViewport;
    g.NextWindowData.ViewportId = id;
}

c_void SetNextWindowDockID(ImGuiID id, ImGuiCond cond)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasDock;
    g.NextWindowData.DockCond = cond ? cond : ImGuiCond_Always;
    g.NextWindowData.DockId = id;
}

c_void SetNextWindowClass(*const ImGuiWindowClass window_class)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT((window_class->ViewportFlagsOverrideSet & window_class->ViewportFlagsOverrideClear) == 0); // Cannot set both set and clear for the same bit
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasWindowClass;
    g.NextWindowData.WindowClass = *window_class;
}

ImDrawList* GetWindowDrawList()
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    return window.DrawList;
}

c_float GetWindowDpiScale()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.CurrentDpiScale;
}

ImGuiViewport* GetWindowViewport()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.CurrentViewport != NULL && g.CurrentViewport == g.Currentwindow.Viewport);
    return g.CurrentViewport;
}



// Gets back to previous line and continue with horizontal layout
//      offset_from_start_x == 0 : follow right after previous item
//      offset_from_start_x != 0 : align to specified x position (relative to window/group left)
//      spacing_w < 0            : use default spacing if pos_x == 0, no spacing if pos_x != 0
//      spacing_w >= 0           : enforce spacing amount
c_void SameLine(c_float offset_from_start_x, c_float spacing_w)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if (window.SkipItems)
        return;

    if (offset_from_start_x != 0f32)
    {
        if (spacing_w < 0f32)
            spacing_w = 0f32;
        window.DC.CursorPos.x = window.Pos.x - window.Scroll.x + offset_from_start_x + spacing_w + window.DC.GroupOffset.x + window.DC.ColumnsOffset.x;
        window.DC.CursorPos.y = window.DC.CursorPosPrevLine.y;
    }
    else
    {
        if (spacing_w < 0f32)
            spacing_w = g.Style.ItemSpacing.x;
        window.DC.CursorPos.x = window.DC.CursorPosPrevLine.x + spacing_w;
        window.DC.CursorPos.y = window.DC.CursorPosPrevLine.y;
    }
    window.DC.CurrLineSize = window.DC.PrevLineSize;
    window.DC.CurrLineTextBaseOffset = window.DC.PrevLineTextBaseOffset;
    window.DC.IsSameLine = true;
}
