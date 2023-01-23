use crate::core::context::AppContext;
use crate::core::condition::{
    ImGuiCond, ImGuiCond_Always, ImGuiCond_Appearing, ImGuiCond_FirstUseEver, ImGuiCond_None,
    ImGuiCond_Once,
};
use crate::drawing::draw_list::ImDrawList;
use crate::core::focused_flags::ImGuiFocusedFlags;
use crate::font::ImFont;
use crate::widgets::hovered_flags::{
    ImGuiHoveredFlags, ImGuiHoveredFlags_AllowWhenBlockedByActiveItem, ImGuiHoveredFlags_AnyWindow,
    ImGuiHoveredFlags_ChildWindows, ImGuiHoveredFlags_DockHierarchy,
    ImGuiHoveredFlags_NoPopupHierarchy, ImGuiHoveredFlags_RootWindow,
};
use crate::window::next_window_data_flags::{
    ImGuiNextWindowDataFlags_HasBgAlpha, ImGuiNextWindowDataFlags_HasCollapsed,
    ImGuiNextWindowDataFlags_HasContentSize, ImGuiNextWindowDataFlags_HasDock,
    ImGuiNextWindowDataFlags_HasFocus, ImGuiNextWindowDataFlags_HasPos,
    ImGuiNextWindowDataFlags_HasScroll, ImGuiNextWindowDataFlags_HasSizeConstraint,
    ImGuiNextWindowDataFlags_HasViewport,
};
use crate::rect::ImRect;
use crate::core::type_defs::ImguiHandle;
use crate::core::utils::{flag_clear, flag_set, is_not_null};
use crate::core::vec2::{Vector2, ImVec2ih};
use crate::window::find::{FindWindowByName, GetCombinedRootWindow, IsWindowChildOf};
use crate::window::focus::FocusWindow;
use crate::window::ops::{GetCurrentWindow, IsWindowContentHoverable};
use crate::window::window_flags::ImGuiWindowFlags_NoNavFocus;
use crate::window::ImguiWindow;
use crate::{GImGui, ImguiViewport};
use libc::{c_char, c_float, c_void};
use std::ptr::null_mut;

pub fn IsWindowHovered(g: &mut AppContext, flags: ImGuiHoveredFlags) -> bool {
    // IM_ASSERT((flags & (ImGuiHoveredFlags_AllowWhenOverlapped | ImGuiHoveredFlags_AllowWhenDisabled)) == 0);   // Flags not supported by this function
    // let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut ref_window = &g.HoveredWindow;
    let mut cur_window = g.current_window_mut().unwrap();
    if ref_window.is_none() {
        return false;
    }

    if flag_clear(flags, ImGuiHoveredFlags_AnyWindow) {
        // IM_ASSERT(cur_window); // Not inside a Begin()/End()
        let popup_hierarchy: bool = flag_clear(flags, ImGuiHoveredFlags_NoPopupHierarchy);
        let dock_hierarchy: bool = flag_set(flags, ImGuiHoveredFlags_DockHierarchy);
        if flag_set(flags, ImGuiHoveredFlags_RootWindow) {
            cur_window = GetCombinedRootWindow(cur_window, popup_hierarchy, dock_hierarchy);
        }

        result: bool;
        if flags & ImGuiHoveredFlags_ChildWindows {
            result = IsWindowChildOf(ref_window, cur_window, popup_hierarchy, dock_hierarchy);
        } else {
            result = (ref_window == cur_window);
        }
        if !result {
            return false;
        }
    }

    if !IsWindowContentHoverable(ref_window, flags) {
        return false;
    }
    if flag_clear(flags, ImGuiHoveredFlags_AllowWhenBlockedByActiveItem) {
        if g.ActiveId != 0 && !g.ActiveIdAllowOverlap && g.ActiveId != ref_window.MoveId {
            return false;
        }
    }
    return true;
}

pub unsafe fn IsWindowFocused(flags: ImGuiFocusedFlags) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut ref_window: &mut ImguiWindow = g.NavWindow;
    let mut cur_window: &mut ImguiWindow = g.CurrentWindow;

    if (ref_window == null_mut()) {
        return false;
    }
    if flag_set(flags, ImGuiFocusedFlags_AnyWindow) {
        return true;
    }

    // IM_ASSERT(cur_window); // Not inside a Begin()/End()
    let popup_hierarchy: bool = flag_clear(flags, ImGuiFocusedFlags_NoPopupHierarchy);
    let dock_hierarchy: bool = flag_set(flags, ImGuiFocusedFlags_DockHierarchy);
    if flag_set(flags, ImGuiHoveredFlags_RootWindow) {
        cur_window = GetCombinedRootWindow(cur_window, popup_hierarchy, dock_hierarchy);
    }

    if flag_set(flags, ImGuiHoveredFlags_ChildWindows) {
        return IsWindowChildOf(ref_window, cur_window, popup_hierarchy, dock_hierarchy);
    } else {
        return (ref_window == cur_window);
    }
}

pub unsafe fn GetWindowDockID() -> ImguiHandle {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.Currentwindow.DockId;
}

pub unsafe fn IsWindowDocked() -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.Currentwindow.DockIsActive;
}

// Can we focus this window with CTRL+TAB (or PadMenu + PadFocusPrev/PadFocusNext)
// Note that NoNavFocus makes the window not reachable with CTRL+TAB but it can still be focused with mouse or programmatically.
// If you want a window to never be focused, you may use the e.g. NoInputs flag.
pub fn IsWindowNavFocusable(window: &mut ImguiWindow) -> bool {
    return window.WasActive
        && window == window.RootWindow
        && flag_clear(window.Flags, ImGuiWindowFlags_NoNavFocus);
}

pub unsafe fn GetWindowWidth() -> f32 {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.current_window_mut().unwrap();
    return window.Size.x;
}

pub unsafe fn GetWindowHeight() -> f32 {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.current_window_mut().unwrap();
    return window.Size.y;
}

pub unsafe fn GetWindowPos() -> Vector2 {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.current_window_mut().unwrap();
    return window.position;
}

pub unsafe fn SetWindowPos(window: &mut ImguiWindow, pos: &Vector2, cond: ImGuiCond) {
    // Test condition (NB: bit 0 is always true) and clear flags for next time
    if cond != ImGuiCond_None && flag_clear(window.SetWindowPosAllowFlags, cond) {
        return;
    }

    // IM_ASSERT(cond == 0 || ImIsPowerOfTwo(cond)); // Make sure the user doesn't attempt to combine multiple condition flags.
    window.SetWindowPosAllowFlags &=
        !(ImGuiCond_Once | ImGuiCond_FirstUseEver | ImGuiCond_Appearing);
    window.SetWindowPosVal = Vector2::from_floats(f32::MAX, f32::MAX);

    // Set
    let old_pos: Vector2 = window.position;
    window.position = ImFloor(pos);
    let offset: Vector2 = window.position - old_pos;
    if offset.x == 0.0 && offset.y == 0.0 {
        return;
    }
    MarkIniSettingsDirty(window);
    // FIXME: share code with TranslateWindow(), need to confirm whether the 3 rect modified by TranslateWindow() are desirable here.
    window.dc.cursor_pos += offset; // As we happen to move the window while it is being appended to (which is a bad idea - will smear) let's at least offset the cursor
    window.dc.CursorMaxPos += offset; // And more importantly we need to offset CursorMaxPos/CursorStartPos this so ContentSize calculation doesn't get affected.
    window.dc.IdealMaxPos += offset;
    window.dc.cursor_start_pos += offset;
}

pub unsafe fn SetWindowPos2(pos: &Vector2, cond: ImGuiCond) {
    let mut window = g.current_window_mut().unwrap();
    SetWindowPos(window, pos, cond);
}

pub unsafe fn SetWindowPos3(name: *const c_char, pos: &Vector2, cond: ImGuiCond) {
    let mut window: &mut ImguiWindow = FindWindowByName(name);
    if is_not_null(window) {
        SetWindowPos(window, pos, cond);
    }
}

pub unsafe fn GetWindowSize() -> Vector2 {
    let mut window = g.current_window_mut().unwrap();
    return window.Size;
}

pub unsafe fn SetWindowSize(window: &mut ImguiWindow, size: &Vector2, cond: ImGuiCond) {
    // Test condition (NB: bit 0 is always true) and clear flags for next time
    if cond != ImGuiCond_None && flag_clear(window.SetWindowSizeAllowFlags, cond) {
        return;
    }

    // IM_ASSERT(cond == 0 || ImIsPowerOfTwo(cond)); // Make sure the user doesn't attempt to combine multiple condition flags.
    window.SetWindowSizeAllowFlags &=
        !(ImGuiCond_Once | ImGuiCond_FirstUseEver | ImGuiCond_Appearing);

    // Set
    let old_size: Vector2 = window.SizeFull;
    window.AutoFitFramesX = if size.x <= 0.0 { 2 } else { 0 };
    window.AutoFitFramesY = if size.y <= 0.0 { 2 } else { 0 };
    if size.x <= 0.0 {
        window.AutoFitOnlyGrows = false;
    } else {
        window.SizeFull.x = IM_FLOOR(size.x);
    }
    if size.y <= 0.0 {
        window.AutoFitOnlyGrows = false;
    } else {
        window.SizeFull.y = IM_FLOOR(size.y);
    }
    if old_size.x != window.SizeFull.x || old_size.y != window.SizeFull.y {
        MarkIniSettingsDirty(window);
    }
}

pub unsafe fn SetWindowSize2(size: &Vector2, cond: ImGuiCond) {
    SetWindowSize(GimGui.CurrentWindow, size, cond);
}

pub unsafe fn SetWindowSize3(name: *const c_char, size: &Vector2, cond: ImGuiCond) {
    let mut window: &mut ImguiWindow = FindWindowByName(name);
    if is_not_null(window) {
        SetWindowSize(window, size, cond);
    }
}

pub unsafe fn SetWindowCollapsed(window: &mut ImguiWindow, collapsed: bool, cond: ImGuiCond) {
    // Test condition (NB: bit 0 is always true) and clear flags for next time
    if cond != ImGuiCond_None && flag_clear(window.SetWindowCollapsedAllowFlags, cond) {
        return;
    }
    window.SetWindowCollapsedAllowFlags &=
        !(ImGuiCond_Once | ImGuiCond_FirstUseEver | ImGuiCond_Appearing);

    // Set
    window.Collapsed = collapsed;
}

pub unsafe fn SetWindowHitTestHole(window: &mut ImguiWindow, pos: &Vector2, size: &Vector2) {
    // IM_ASSERT(window.HitTestHoleSize.x == 0);     // We don't support multiple holes/hit test filters
    window.HitTestHoleSize = ImVec2ih(size);
    window.HitTestHoleOffset = ImVec2ih(pos - window.position);
}

pub unsafe fn SetWindowCollapsed2(collapsed: bool, cond: ImGuiCond) {
    SetWindowCollapsed(GimGui.CurrentWindow, collapsed, cond);
}

pub unsafe fn IsWindowCollapsed() -> bool {
    let mut window = g.current_window_mut().unwrap();
    return window.Collapsed;
}

pub unsafe fn IsWindowAppearing() -> bool {
    let mut window = g.current_window_mut().unwrap();
    return window.Appearing;
}

pub unsafe fn SetWindowCollapsed3(name: *const c_char, collapsed: bool, cond: ImGuiCond) {
    let mut window: &mut ImguiWindow = FindWindowByName(name);
    if is_not_null(window) {
        SetWindowCollapsed(window, collapsed, cond);
    }
}

pub unsafe fn SetWindowFocus() {
    let ctx = GImGui;
    FocusWindow(ctx.CurrentWindow);
}

pub unsafe fn SetWindowFocus2(name: *const c_char) {
    if name {
        let mut window: &mut ImguiWindow = FindWindowByName(name);
        if is_not_null(window) {
            FocusWindow(window);
        }
    } else {
        FocusWindow(null_mut());
    }
}

pub fn SetNextWindowPos(
    g: &mut AppContext,
    pos: &Vector2,
    cond: ImGuiCond,
    pivot: Option<Vector2>,
) {
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasPos;
    g.NextWindowData.PosVal = pos.clone();
    g.NextWindowData.PosPivotVal = pivot.clone();
    g.NextWindowData.PosCond = if cond != ImGuiCond_None {
        cond
    } else {
        ImGuiCond_Always
    };
    g.NextWindowData.PosUndock = true;
}

pub fn SetNextWindowSizeConstraints(
    g: &mut AppContext,
    size_min: &Vector2,
    size_max: &Vector2,
    custom_callback: ImGuiSizeCallback,
    custom_callback_user_data: Option<&Vec<u8>>,
) {
    // let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasSizeConstraint;
    g.NextWindowData.SizeConstraintRect = ImRect::from_vec2(size_min, size_max);
    g.NextWindowData.SizeCallback = custom_callback;
    g.NextWindowData.SizeCallbackUserData = Some(custom_callback_user_data.unwrap().clone());
}

// Content size = inner scrollable rectangle, padded with WindowPadding.
// SetNextWindowContentSize(ImVec2::new(100,100) + ImGuiWindowFlags_AlwaysAutoResize will always allow submitting a 100x100 item.
pub unsafe fn SetNextWindowContentSize(size: &Vector2) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasContentSize;
    g.NextWindowData.ContentSizeVal = ImFloor(size);
}

pub unsafe fn SetNextWindowScroll(scroll: &Vector2) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasScroll;
    g.NextWindowData.ScrollVal = scroll.clone();
}

pub unsafe fn SetNextWindowCollapsed(collapsed: bool, cond: ImGuiCond) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
                    // IM_ASSERT(cond == 0 || ImIsPowerOfTwo(cond)); // Make sure the user doesn't attempt to combine multiple condition flags.
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasCollapsed;
    g.NextWindowData.CollapsedVal = collapsed;
    g.NextWindowData.CollapsedCond = if cond != ImGuiCond_None {
        cond
    } else {
        ImGuiCond_Always
    };
}

pub unsafe fn SetNextWindowFocus() {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasFocus;
}

pub unsafe fn SetNextWindowBgAlpha(alpha: c_float) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasBgAlpha;
    g.NextWindowData.BgAlphaVal = alpha;
}

pub unsafe fn SetNextWindowViewport(id: ImguiHandle) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasViewport;
    g.NextWindowData.ViewportId = id;
}

pub unsafe fn SetNextWindowDockID(id: ImguiHandle, cond: ImGuiCond) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasDock;
    g.NextWindowData.DockCond = if cond != ImGuiCond_None {
        cond
    } else {
        ImGuiCond_Always
    };
    g.NextWindowData.DockId = id;
}

// ImDrawList* GetWindowDrawList()
pub unsafe fn GetWindowDrawList() -> ImDrawList {
    let mut window = g.current_window_mut().unwrap();
    return window.DrawList;
}

// GetWindowDpiScale: c_float()
pub unsafe fn GetWindowDpiScale() -> f32 {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.CurrentDpiScale;
}

// GetWindowViewport: *mut ImguiViewport()
pub unsafe fn GetWindowViewport() -> *mut ImguiViewport {
    let g = GImGui; // ImGuiContext& g = *GImGui;
                    // IM_ASSERT(g.CurrentViewport != NULL && g.CurrentViewport == g.Currentwindow.Viewport);
    return g.CurrentViewport;
}

// GetFont: *mut ImFont()
pub unsafe fn GetFont() -> *mut ImFont {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.Font;
}

// GetFontSize: c_float()
pub unsafe fn GetFontSize() -> f32 {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.FontSize;
}

// GetFontTexUvWhitePixel: ImVec2()
pub unsafe fn GetFontTexUvWhitePixel() -> Vector2 {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.DrawListSharedData.TexUvWhitePixel;
}

pub unsafe fn SetWindowFontScale(scale: c_float) {
    // IM_ASSERT(scale > 0.0);
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window: &mut ImguiWindow = GetCurrentWindow();
    window.FontWindowScale = scale;
    g.FontSize = window.CalcFontSize();
    g.DrawListSharedData.FontSize = window.CalcFontSize();
}
