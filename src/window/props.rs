use std::ptr::null_mut;
use libc::{c_char, c_float, c_void};
use crate::condition::{ImGuiCond, ImGuiCond_Always, ImGuiCond_Appearing, ImGuiCond_FirstUseEver, ImGuiCond_None, ImGuiCond_Once};
use crate::draw_list::ImDrawList;
use crate::focused_flags::ImGuiFocusedFlags;
use crate::{GImGui, ImGuiViewport};
use crate::font::ImFont;
use crate::hovered_flags::{ImGuiHoveredFlags, ImGuiHoveredFlags_AllowWhenBlockedByActiveItem, ImGuiHoveredFlags_AnyWindow, ImGuiHoveredFlags_ChildWindows, ImGuiHoveredFlags_DockHierarchy, ImGuiHoveredFlags_NoPopupHierarchy, ImGuiHoveredFlags_RootWindow};
use crate::next_window_data_flags::{ImGuiNextWindowDataFlags_HasBgAlpha, ImGuiNextWindowDataFlags_HasCollapsed, ImGuiNextWindowDataFlags_HasContentSize, ImGuiNextWindowDataFlags_HasDock, ImGuiNextWindowDataFlags_HasFocus, ImGuiNextWindowDataFlags_HasPos, ImGuiNextWindowDataFlags_HasScroll, ImGuiNextWindowDataFlags_HasSizeConstraint, ImGuiNextWindowDataFlags_HasViewport};
use crate::rect::ImRect;
use crate::type_defs::ImGuiID;
use crate::utils::{flag_clear, flag_set, is_not_null};
use crate::vec2::{ImVec2, ImVec2ih};
use crate::window::find::{FindWindowByName, GetCombinedRootWindow, IsWindowChildOf};
use crate::window::focus::FocusWindow;
use crate::window::ImGuiWindow;
use crate::window::ops::{GetCurrentWindow, IsWindowContentHoverable};
use crate::window::window_flags::ImGuiWindowFlags_NoNavFocus;

pub unsafe fn IsWindowHovered(flags: ImGuiHoveredFlags) -> bool {
    // IM_ASSERT((flags & (ImGuiHoveredFlags_AllowWhenOverlapped | ImGuiHoveredFlags_AllowWhenDisabled)) == 0);   // Flags not supported by this function
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut ref_window: &mut ImGuiWindow = g.HoveredWindow;
    let mut cur_window: &mut ImGuiWindow = g.CurrentWindow;
    if ref_window == None {
        return false;
    }

    if flag_clear(flags, ImGuiHoveredFlags_AnyWindow) {
        // IM_ASSERT(cur_window); // Not inside a Begin()/End()
        let popup_hierarchy: bool = flag_clear(flags, ImGuiHoveredFlags_NoPopupHierarchy);
        let dock_hierarchy: bool = flag_set(flags, ImGuiHoveredFlags_DockHierarchy);
        if flags & ImGuiHoveredFlags_RootWindow {
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
    let mut ref_window: &mut ImGuiWindow = g.NavWindow;
    let mut cur_window: &mut ImGuiWindow = g.CurrentWindow;

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


pub unsafe fn GetWindowDockID() -> ImGuiID {
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
pub fn IsWindowNavFocusable(window: &mut ImGuiWindow) -> bool {
    return window.WasActive && window == window.RootWindow && flag_clear(window.Flags, ImGuiWindowFlags_NoNavFocus);
}


pub unsafe fn GetWindowWidth() -> c_float {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window: &mut ImGuiWindow = g.CurrentWindow;
    return window.Size.x;
}


pub unsafe fn GetWindowHeight() -> c_float {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window: &mut ImGuiWindow = g.CurrentWindow;
    return window.Size.y;
}

pub unsafe fn GetWindowPos() -> ImVec2 {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    return window.Pos;
}

pub unsafe fn SetWindowPos(window: &mut ImGuiWindow, pos: &ImVec2, cond: ImGuiCond) {
    // Test condition (NB: bit 0 is always true) and clear flags for next time
    if cond != ImGuiCond_None && flag_clear(window.SetWindowPosAllowFlags, cond) {
        return;
    }

    // IM_ASSERT(cond == 0 || ImIsPowerOfTwo(cond)); // Make sure the user doesn't attempt to combine multiple condition flags.
    window.SetWindowPosAllowFlags &= !(ImGuiCond_Once | ImGuiCond_FirstUseEver | ImGuiCond_Appearing);
    window.SetWindowPosVal = ImVec2::from_floats(f32::MAX, f32::MAX);

    // Set
    let old_pos: ImVec2 = window.Pos;
    window.Pos = ImFloor(pos);
    let offset: ImVec2 = window.Pos - old_pos;
    if offset.x == 0.0 && offset.y == 0.0 {
        return;
    }
    MarkIniSettingsDirty(window);
    // FIXME: share code with TranslateWindow(), need to confirm whether the 3 rect modified by TranslateWindow() are desirable here.
    window.DC.CursorPos += offset;         // As we happen to move the window while it is being appended to (which is a bad idea - will smear) let's at least offset the cursor
    window.DC.CursorMaxPos += offset;      // And more importantly we need to offset CursorMaxPos/CursorStartPos this so ContentSize calculation doesn't get affected.
    window.DC.IdealMaxPos += offset;
    window.DC.CursorStartPos += offset;
}


pub unsafe fn SetWindowPos2(pos: &ImVec2, cond: ImGuiCond) {
    let mut window: &mut ImGuiWindow = GetCurrentWindowRead();
    SetWindowPos(window, pos, cond);
}

pub unsafe fn SetWindowPos3(name: *const c_char, pos: &ImVec2, cond: ImGuiCond) {
    let mut window: &mut ImGuiWindow = FindWindowByName(name);
    if is_not_null(window) {
        SetWindowPos(window, pos, cond);
    }
}

pub unsafe fn GetWindowSize() -> ImVec2 {
    let mut window: &mut ImGuiWindow = GetCurrentWindowRead();
    return window.Size;
}


pub unsafe fn SetWindowSize(window: &mut ImGuiWindow, size: &ImVec2, cond: ImGuiCond) {
    // Test condition (NB: bit 0 is always true) and clear flags for next time
    if cond != ImGuiCond_None && flag_clear(window.SetWindowSizeAllowFlags, cond) {
        return;
    }

    // IM_ASSERT(cond == 0 || ImIsPowerOfTwo(cond)); // Make sure the user doesn't attempt to combine multiple condition flags.
    window.SetWindowSizeAllowFlags &= !(ImGuiCond_Once | ImGuiCond_FirstUseEver | ImGuiCond_Appearing);

    // Set
    let old_size: ImVec2 = window.SizeFull;
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

pub unsafe fn SetWindowSize2(size: &ImVec2, cond: ImGuiCond) {
    SetWindowSize(GimGui.CurrentWindow, size, cond);
}

pub unsafe fn SetWindowSize3(name: *const c_char, size: &ImVec2, cond: ImGuiCond) {
    let mut window: &mut ImGuiWindow = FindWindowByName(name);
    if is_not_null(window) {
        SetWindowSize(window, size, cond);
    }
}

pub unsafe fn SetWindowCollapsed(window: &mut ImGuiWindow, collapsed: bool, cond: ImGuiCond) {
    // Test condition (NB: bit 0 is always true) and clear flags for next time
    if cond != ImGuiCond_None && flag_clear(window.SetWindowCollapsedAllowFlags, cond) {
        return;
    }
    window.SetWindowCollapsedAllowFlags &= !(ImGuiCond_Once | ImGuiCond_FirstUseEver | ImGuiCond_Appearing);

    // Set
    window.Collapsed = collapsed;
}

pub unsafe fn SetWindowHitTestHole(window: &mut ImGuiWindow, pos: &ImVec2, size: &ImVec2) {
    // IM_ASSERT(window.HitTestHoleSize.x == 0);     // We don't support multiple holes/hit test filters
    window.HitTestHoleSize = ImVec2ih(size);
    window.HitTestHoleOffset = ImVec2ih(pos - window.Pos);
}

pub unsafe fn SetWindowCollapsed2(collapsed: bool, cond: ImGuiCond) {
    SetWindowCollapsed(GimGui.CurrentWindow, collapsed, cond);
}

pub unsafe fn IsWindowCollapsed() -> bool {
    let mut window: &mut ImGuiWindow = GetCurrentWindowRead();
    return window.Collapsed;
}

pub unsafe fn IsWindowAppearing() -> bool {
    let mut window: &mut ImGuiWindow = GetCurrentWindowRead();
    return window.Appearing;
}

pub unsafe fn SetWindowCollapsed3(name: *const c_char, collapsed: bool, cond: ImGuiCond) {
    let mut window: &mut ImGuiWindow = FindWindowByName(name);
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
        let mut window: &mut ImGuiWindow = FindWindowByName(name);
        if is_not_null(window) {
            FocusWindow(window);
        }
    } else {
        FocusWindow(null_mut());
    }
}

pub unsafe fn SetNextWindowPos(pos: &ImVec2, cond: ImGuiCond, pivot: &ImVec2) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(cond == 0 || ImIsPowerOfTwo(cond)); // Make sure the user doesn't attempt to combine multiple condition flags.
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasPos;
    g.NextWindowData.PosVal = pos.clone();
    g.NextWindowData.PosPivotVal = pivot.clone();
    g.NextWindowData.PosCond = if cond != ImGuiCond_None { cond } else { ImGuiCond_Always };
    g.NextWindowData.PosUndock = true;
}

pub unsafe fn SetNextWindowSizeConstraints(size_min: &ImVec2, size_max: &ImVec2, custom_callback: ImGuiSizeCallback, custom_callback_user_data: *mut c_void) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasSizeConstraint;
    g.NextWindowData.SizeConstraintRect = ImRect::from_vec2(size_min, size_max);
    g.NextWindowData.SizeCallback = custom_callback;
    g.NextWindowData.SizeCallbackUserData = custom_callback_user_data;
}


// Content size = inner scrollable rectangle, padded with WindowPadding.
// SetNextWindowContentSize(ImVec2::new(100,100) + ImGuiWindowFlags_AlwaysAutoResize will always allow submitting a 100x100 item.
pub unsafe fn SetNextWindowContentSize(size: &ImVec2) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasContentSize;
    g.NextWindowData.ContentSizeVal = ImFloor(size);
}


pub unsafe fn SetNextWindowScroll(scroll: &ImVec2) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasScroll;
    g.NextWindowData.ScrollVal = scroll.clone();
}

pub unsafe fn SetNextWindowCollapsed(collapsed: bool, cond: ImGuiCond) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(cond == 0 || ImIsPowerOfTwo(cond)); // Make sure the user doesn't attempt to combine multiple condition flags.
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasCollapsed;
    g.NextWindowData.CollapsedVal = collapsed;
    g.NextWindowData.CollapsedCond = if cond != ImGuiCond_None { cond } else { ImGuiCond_Always };
}

pub unsafe fn SetNextWindowFocus()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasFocus;
}

pub unsafe fn SetNextWindowBgAlpha(alpha: c_float)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasBgAlpha;
    g.NextWindowData.BgAlphaVal = alpha;
}

pub unsafe fn SetNextWindowViewport(id: ImGuiID)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasViewport;
    g.NextWindowData.ViewportId = id;
}

pub unsafe fn SetNextWindowDockID(id: ImGuiID, cond: ImGuiCond)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasDock;
    g.NextWindowData.DockCond = if cond != ImGuiCond_None { cond } else { ImGuiCond_Always };
    g.NextWindowData.DockId = id;
}

// ImDrawList* GetWindowDrawList()
pub unsafe fn GetWindowDrawList() -> ImDrawList
{
    let mut window =  GetCurrentWindow();
    return window.DrawList;
}

// GetWindowDpiScale: c_float()
pub unsafe fn GetWindowDpiScale() -> c_float
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.CurrentDpiScale;
}

// GetWindowViewport: *mut ImGuiViewport()
pub unsafe fn GetWindowViewport() -> *mut ImGuiViewport
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.CurrentViewport != NULL && g.CurrentViewport == g.Currentwindow.Viewport);
    return g.CurrentViewport;
}

// GetFont: *mut ImFont()
pub unsafe fn GetFont() -> *mut ImFont
{
     let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.Font;
}

// GetFontSize: c_float()
pub unsafe fn GetFontSize() -> c_float
{
     let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.FontSize;
}


// GetFontTexUvWhitePixel: ImVec2()
pub unsafe fn GetFontTexUvWhitePixel() -> ImVec2
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.DrawListSharedData.TexUvWhitePixel;
}

pub unsafe fn SetWindowFontScale(scale: c_float)
{
    // IM_ASSERT(scale > 0.0);
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window: &mut ImGuiWindow =  GetCurrentWindow();
    window.FontWindowScale = scale;
    g.FontSize = window.CalcFontSize();
    g.DrawListSharedData.FontSize = window.CalcFontSize();
}
