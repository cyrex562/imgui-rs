#![allow(non_snake_case)]

use std::borrow::BorrowMut;
use std::mem;
use crate::color::{IM_COL32_A_MASK, IM_COL32_A_SHIFT, ImGuiCol, ImGuiCol_Border, ImGuiCol_Button, ImGuiCol_ButtonActive, ImGuiCol_ButtonHovered, ImGuiCol_ChildBg, ImGuiCol_MenuBarBg, ImGuiCol_ModalWindowDimBg, ImGuiCol_NavWindowingDimBg, ImGuiCol_NavWindowingHighlight, ImGuiCol_PopupBg, ImGuiCol_ResizeGrip, ImGuiCol_ResizeGripActive, ImGuiCol_ResizeGripHovered, ImGuiCol_SeparatorActive, ImGuiCol_TitleBg, ImGuiCol_TitleBgActive, ImGuiCol_TitleBgCollapsed, ImGuiCol_WindowBg};
use crate::condition::{ImGuiCond, ImGuiCond_Always, ImGuiCond_Appearing, ImGuiCond_FirstUseEver, ImGuiCond_None, ImGuiCond_Once};
use crate::draw_flags::{ImDrawFlags_None, ImDrawFlags_RoundCornersBottom, ImDrawFlags_RoundCornersTop};
use crate::draw_list::ImDrawList;
use crate::draw_list_ops::GetForegroundDrawList;
use crate::hovered_flags::{ImGuiHoveredFlags, ImGuiHoveredFlags_AllowWhenBlockedByPopup};
use crate::imgui::GImGui;
use crate::input_source::{ImGuiInputSource_Gamepad, ImGuiInputSource_Keyboard, ImGuiInputSource_Mouse, ImGuiInputSource_Nav};
use crate::next_window_data_flags::{ImGuiNextWindowDataFlags_HasBgAlpha, ImGuiNextWindowDataFlags_HasCollapsed, ImGuiNextWindowDataFlags_HasContentSize, ImGuiNextWindowDataFlags_HasDock, ImGuiNextWindowDataFlags_HasFocus, ImGuiNextWindowDataFlags_HasPos, ImGuiNextWindowDataFlags_HasScroll, ImGuiNextWindowDataFlags_HasSize, ImGuiNextWindowDataFlags_HasSizeConstraint, ImGuiNextWindowDataFlags_HasWindowClass};
use crate::rect::ImRect;
use crate::render_ops::{RenderFrame, RenderRectFilledWithHole};
use crate::style_ops::GetColorU32;
use crate::type_defs::ImguiHandle;
use crate::vec2::ImVec2;
use crate::window::{find, focus, ImguiWindow, render};
use crate::window_flags::{ImGuiWindowFlags, ImGuiWindowFlags_AlwaysAutoResize, ImGuiWindowFlags_AlwaysHorizontalScrollbar, ImGuiWindowFlags_AlwaysVerticalScrollbar, ImGuiWindowFlags_ChildMenu, ImGuiWindowFlags_ChildWindow, ImGuiWindowFlags_DockNodeHost, ImGuiWindowFlags_HorizontalScrollbar, ImGuiWindowFlags_MenuBar, ImGuiWindowFlags_Modal, ImGuiWindowFlags_NoBackground, ImGuiWindowFlags_NoBringToFrontOnFocus, ImGuiWindowFlags_NoMouseInputs, ImGuiWindowFlags_NoResize, ImGuiWindowFlags_NoSavedSettings, ImGuiWindowFlags_NoScrollbar, ImGuiWindowFlags_NoTitleBar, ImGuiWindowFlags_Popup, ImGuiWindowFlags_Tooltip};
use crate::{ImguiViewport, hash_string};
use libc::{c_char, c_float, c_int, c_short, c_void, size_t, strcmp};
use std::ptr::{null, null_mut};
use crate::axis::{ImGuiAxis_X, ImGuiAxis_Y};
use crate::config_flags::{ImGuiConfigFlags_DockingEnable, ImGuiConfigFlags_DpiEnableScaleFonts};
use crate::constants::{WINDOWS_HOVER_PADDING, WINDOWS_RESIZE_FROM_EDGES_FEEDBACK_TIMER};
use crate::cursor_ops::ErrorCheckUsingSetCursorPosToExtendParentBoundaries;
use crate::direction::{ImGuiDir, ImGuiDir_Down, ImGuiDir_Left, ImGuiDir_None, ImGuiDir_Right, ImGuiDir_Up};
use crate::dock_node::ImGuiDockNode;
use crate::garbage_collection::GcAwakeTransientWindowBuffers;
use crate::hash_ops::hash_data;
use {ClearActiveID, KeepAliveID};
use crate::context::ImguiContext;
use crate::input_ops::{IsMouseDragging, IsMouseHoveringRect};
use crate::item_flags::ImGuiItemFlags_Disabled;
use crate::item_ops::set_last_item_data;
use crate::item_status_flags::ImGuiItemStatusFlags_HoveredRect;
use crate::key::{ImGuiKey_DownArrow, ImGuiKey_GamepadDpadDown, ImGuiKey_GamepadDpadLeft, ImGuiKey_GamepadDpadRight, ImGuiKey_GamepadDpadUp, ImGuiKey_LeftArrow, ImGuiKey_RightArrow, ImGuiKey_UpArrow};
use crate::layout_type::ImGuiLayoutType_Vertical;
use crate::math_ops::{ImClamp, ImLerp, ImLerpVec2, ImLerpVec22, ImMax, ImMin, ImSwap};
use crate::mouse_cursor::{ImGuiMouseCursor_ResizeEW, ImGuiMouseCursor_ResizeNESW, ImGuiMouseCursor_ResizeNS, ImGuiMouseCursor_ResizeNWSE};
use crate::mouse_ops::StartMouseMovingWindowOrNode;
use crate::nav_layer::{ImGuiNavLayer_Main, ImGuiNavLayer_Menu};
use crate::platform_monitor::ImGuiPlatformMonitor;
use crate::popup_data::ImGuiPopupData;
use crate::resize_border_def::resize_border_def;
use crate::resize_grip_def::resize_grip_def;
use crate::size_callback_data::ImGuiSizeCallbackData;
use crate::string_ops::{ImStrdupcpy, str_to_const_c_char_ptr};
use crate::utils::{flag_clear, flag_set, is_not_null, is_null};
use crate::vec4::ImVec4;
use crate::window::find::{FindBlockingModal, FindWindowByName, FindWindowDisplayIndex};
use crate::window::rect::{ClampWindowRect, PopClipRect, PushClipRect};
use crate::window::render::{RenderWindowDecorations, RenderWindowTitleBarContents, UpdateWindowParentAndRootLinks};
use crate::window::window_flags::{ImGuiWindowFlags, ImGuiWindowFlags_AlwaysAutoResize, ImGuiWindowFlags_AlwaysHorizontalScrollbar, ImGuiWindowFlags_AlwaysUseWindowPadding, ImGuiWindowFlags_AlwaysVerticalScrollbar, ImGuiWindowFlags_ChildMenu, ImGuiWindowFlags_ChildWindow, ImGuiWindowFlags_DockNodeHost, ImGuiWindowFlags_HorizontalScrollbar, ImGuiWindowFlags_MenuBar, ImGuiWindowFlags_Modal, ImGuiWindowFlags_NavFlattened, ImGuiWindowFlags_NoBackground, ImGuiWindowFlags_NoBringToFrontOnFocus, ImGuiWindowFlags_NoCollapse, ImGuiWindowFlags_NoDocking, ImGuiWindowFlags_NoFocusOnAppearing, ImGuiWindowFlags_NoInputs, ImGuiWindowFlags_NoMouseInputs, ImGuiWindowFlags_NoMove, ImGuiWindowFlags_NoNavFocus, ImGuiWindowFlags_NoResize, ImGuiWindowFlags_NoSavedSettings, ImGuiWindowFlags_NoScrollbar, ImGuiWindowFlags_NoTitleBar, ImGuiWindowFlags_Popup, ImGuiWindowFlags_Tooltip};
use crate::window::window_settings::ImGuiWindowSettings;
use crate::window::window_stack_data::ImGuiWindowStackData;
use crate::window_settings::ImGuiWindowSettings;

// static c_void SetCurrentWindow(window: &mut ImGuiWindow)
pub fn SetCurrentWindow(g: &mut ImguiContext, window: ImguiHandle) {
    // let g = GImGui; // ImGuiContext& g = *GImGui;
    g.CurrentWindow = window;
    g.current_table = if is_not_null(window) && window.dc.CurrentTableIdx != -1 {
        g.Tables.GetByIndex(window.dc.CurrentTableIdx.clone())
    } else {
        None
    };
    if window {
        g.FontSize = window.CalcFontSize();
        g.DrawListSharedData.FontSize = window.CalcFontSize();
    }
}

// pub fn GetCurrentWindow(g: &mut ImGuiContext) -> &mut ImGuiWindow {
//     // let g = GImGui;
//     g.CurrentWindow.unwrap().borrow_mut()
// }

// static inline IsWindowContentHoverable: bool(window: &mut ImGuiWindow, flags: ImGuiHoveredFlags)
pub unsafe fn IsWindowContentHoverable(window: &mut ImguiWindow, flags: ImGuiHoveredFlags) -> bool {
    // An active popup disable hovering on other windows (apart from its own children)
    // FIXME-OPT: This could be cached/stored within the window.
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if g.NavWindow {
        if focused_root_window: &mut ImguiWindow = g.NavWindow.RootWindowDockTree {
            if focused_root_window.WasActive && focused_root_window != window.RootWindowDockTree {
                // For the purpose of those flags we differentiate "standard popup" from "modal popup"
                // NB: The order of those two tests is important because Modal windows are also Popups.
                if focused_root_window.Flags & ImGuiWindowFlags_Modal {
                    return false;
                }
                if flag_set(focused_root_window.Flags, ImGuiWindowFlags_Popup)
                    && flag_clear(flags, ImGuiHoveredFlags_AllowWhenBlockedByPopup)
                {
                    return false;
                }
            }
        }
    }
    // Filter by viewport
    if window.Viewport != g.MouseViewport {
        if g.MovingWindow == None
            || window.RootWindowDockTree != g.MovingWindow.RootWindowDockTree
        {
            return false;
        }
    }

    return true;
}

// This is called during NewFrame()->UpdateViewportsNewFrame() only.
// Need to keep in sync with SetWindowPos()
// static c_void TranslateWindow(window: &mut ImGuiWindow, const delta: &mut ImVec2)
pub fn TranslateWindow(window: &mut ImguiWindow, delta: &ImVec2) {
    window.position += delta;
    window.ClipRect.Translate(delta);
    window.OuterRectClipped.Translate(delta);
    window.InnerRect.Translate(delta);
    window.dc.cursor_pos += delta;
    window.dc.cursor_start_pos += delta;
    window.dc.CursorMaxPos += delta;
    window.dc.IdealMaxPos += delta;
}

// static c_void ScaleWindow(window: &mut ImGuiWindow, c_float scale)
pub fn ScaleWindow(window: &mut ImguiWindow, scale: c_float) {
    let origin: ImVec2 = window.Viewport.Pos.clone();
    window.position = ImFloor((window.position.clone() - origin) * scale + origin.clone());
    window.Size = ImFloor(window.Size.clone() * scale.clone());
    window.SizeFull = ImFloor(window.SizeFull.clone() * scale.clone());
    window.ContentSize = ImFloor(window.ContentSize.clone() * scale.clone());
}

// static IsWindowActiveAndVisible: bool(window: &mut ImGuiWindow)
pub fn IsWindowActiveAndVisible(window: &mut ImguiWindow) -> bool {
    return (window.Active.clone()) && (!window.Hidden.clone());
}

// FIXME: Add a more explicit sort order in the window structure.
// : c_int ChildWindowComparer(lhs: *const c_void, rhs: *const c_void)
pub fn ChildWindowComparer(lhs: *const c_void, rhs: *const c_void) -> c_int {
    let a: *const ImguiWindow = lhs;
    let b: *const ImguiWindow = rhs;
    let mut d = (a.Flags.clone(), ImGuiWindowFlags_Popup) - (b.Flags.clone() & ImGuiWindowFlags_Popup);
    if d {
        return d;
    }
    //     if (let d: c_int = (a->Flags & ImGuiWindowFlags_Popup) - (b->Flags & ImGuiWindowFlags_Popup))
    //     {
    // return d;
    //     }
    //     if (let d: c_int = (a->Flags & ImGuiWindowFlags_Tooltip) - (b->Flags & ImGuiWindowFlags_Tooltip))
    //         return d;
    d = (a.Flags.clone() & ImGuiWindowFlags_Tooltip) - (b.Flags.clone() & ImGuiWindowFlags_Tooltip);
    if d {
        return d;
    }
    return (a.BeginOrderWithinParent.clone() - b.BeginOrderWithinParent.clone()) as c_int;
}

// static c_void AddWindowToSortBuffer(Vec<ImGuiWindow*>* out_sorted_windows, window: &mut ImGuiWindow)
pub fn AddWindowToSortBuffer(
    mut out_sorted_windows: *mut Vec<*mut ImguiWindow>,
    window: &mut ImguiWindow,
) {
    out_sorted_windows.push(window);
    if window.Active {
        let count: c_int = window.dc.ChildWindows.Size;
        // ImQsort(window.dc.ChildWindows.Data, count, sizeof(ImGuiWindow*), ChildWindowComparer);
        // todo!()
        // for (let i: c_int = 0; i < count; i++)
        for i in 0..count {
            let mut child: *mut ImguiWindow = window.dc.ChildWindows[i];
            if child.Active {
                AddWindowToSortBuffer(out_sorted_windows, child);
            }
        }
    }
}

// static inline GetWindowDisplayLayer: c_int(window: &mut ImGuiWindow)
pub fn GetWindowDisplayLayer(window: &mut ImguiWindow) -> c_int {
    return if flag_set(window.Flags.clone(), ImGuiWindowFlags_Tooltip) {
        1
    } else {
        0
    };
}

pub unsafe fn SetNextWindowSize(size: ImVec2, cond: ImGuiCond) {
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

pub fn SetWindowConditionAllowFlags(window: &mut ImguiWindow, flags: ImGuiCond, enabled: bool) {
    window.SetWindowPosAllowFlags = if enabled {
        (window.SetWindowPosAllowFlags.clone() | flags)
    } else {
        window.SetWindowPosAllowFlags.clone() & !flags
    };
    window.SetWindowSizeAllowFlags = if enabled {
        (window.SetWindowSizeAllowFlags.clone() | flags.clone())
    } else {
        (window.SetWindowSizeAllowFlags.clone() & !flags.clone())
    };
    window.SetWindowCollapsedAllowFlags = if enabled {
        (window.SetWindowCollapsedAllowFlags.clone() | flags.clone())
    } else {
        window.SetWindowCollapsedAllowFlags.clone() & !flags.clone()
    };
    window.SetWindowDockAllowFlags = if enabled {
        (window.SetWindowDockAllowFlags.clone() | flags.clone())
    } else {
        window.SetWindowDockAllowFlags.clone() & !flags.clone()
    };
}


pub fn ApplyWindowSettings(window: &mut ImguiWindow, settings: *mut ImGuiWindowSettings)
{
    let main_viewport: *const ImguiViewport = GetMainViewport();
    window.ViewportPos = main_viewport.Pos;
    if settings.ViewportId
    {
        window.ViewportId = settings.ViewportId.clone();
        window.ViewportPos = ImVec2::from_floats(settings.ViewportPos.x.clone() as c_float, settings.ViewportPos.y.clone() as c_float);
    }
    window.position = ImFloor(ImVec2::from_floats(settings.Pos.x.clone() + window.ViewportPos.x.clone(), settings.Pos.y.clone() + window.ViewportPos.y.clone()));
    if settings.Size.x > 0 && settings.Size.y > 0 {
        window.SizeFull = ImFloor(ImVec2::from_floats(settings.Size.x.clone() as c_float, settings.Size.y.clone() as c_float));
        window.Size = window.SizeFull;
    }
    window.Collapsed = settings.Collapsed.clone();
    window.DockId = settings.DockId.clone();
    window.DockOrder = settings.DockOrder.clone();
}


pub unsafe fn UpdateWindowInFocusOrderList(window: &mut ImguiWindow, just_created: bool, new_flags: ImGuiWindowFlags)
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
    else if !just_created.clone() && child_flag_changed.clone() && new_is_explicit_child.clone()
    {
        // IM_ASSERT(g.WindowsFocusOrder[window.FocusOrder] == window);
        // for (let n: c_int = window.FocusOrder + 1; n < g.WindowsFocusOrder.Size; n++)
        for n in window.FocusOrder.clone() + 1 .. g.WindowsFocusOrder.len()
        {
            g.WindowsFocusOrder[n].FocusOrder -= 1;
        }
        g.WindowsFocusOrder.erase(g.WindowsFocusOrder.Data + window.FocusOrder.clone());
        window.FocusOrder = -1;
    }
    window.IsExplicitChild = new_is_explicit_child.clone();
}


pub fn CreateNewWindow(g: &mut ImguiContext, name: &String, flags: ImGuiWindowFlags) -> ImguiWindow {
    // let g = GImGui; // ImGuiContext& g = *GImGui;
    //IMGUI_DEBUG_LOG("CreateNewWindow '{}', flags = 0x{}\n", name, flags);

    // Create window the first time
    let mut window =  ImguiWindow::new(g, name);
    window.Flags = flags;


    // Default/arbitrary window position. Use SetNextWindowPos() with the appropriate condition flag to change the initial position of a window.
    let main_viewport: *const ImguiViewport = GetMainViewport();
    window.position = main_viewport.Pos + ImVec2::from_floats(60.0, 60.0);
    window.ViewportPos = main_viewport.Pos;

    // User can disable loading and saving of settings. Tooltip and child windows also don't store settings.
    if flag_clear(flags.clone(), ImGuiWindowFlags_NoSavedSettings) {
        if settings: *mut ImGuiWindowSettings = FindWindowSettings(window.ID.clone()) {
            // Retrieve settings from .ini file
            window.SettingsOffset = g.SettingsWindows.offset_from_ptr(settings);
            SetWindowConditionAllowFlags(window, ImGuiCond_FirstUseEver, false);
            ApplyWindowSettings(window, settings);
        }
    }
    window.dc.cursor_start_pos = window.position;
    window.dc.CursorMaxPos = window.position;
    window.dc.IdealMaxPos = window.position; // So first call to CalcWindowContentSizes() doesn't return crazy values

    if flag_set(flags.clone(), ImGuiWindowFlags_AlwaysAutoResize)
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

    if flag_set(flags.clone(), ImGuiWindowFlags_NoBringToFrontOnFocus) {
        g.Windows.push_front(window);
    }
    else {
        // Quite slow but rare and only once
        g.Windows.push(window);
    }

    // g.WindowsById.SetVoidPtr(window.ID.clone(), window);
    g.Windows.insert(window.ID, window);

    return window.ID.clone();
}


pub fn CalcWindowSizeAfterConstraint(g: &mut ImguiContext, window: &ImguiWindow, size_desired: &ImVec2) -> ImVec2
{
    // let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut new_size: ImVec2 = size_desired.clone();
    if flag_set(g.NextWindowData.Flags, ImGuiNextWindowDataFlags_HasSizeConstraint)
    {
        // Using -1,-1 on either X/Y axis to preserve the current size.
        let cr: ImRect =  g.NextWindowData.SizeConstraintRect;
        new_size.x = if cr.min.x >= 0.0 && cr.max.x >= 0.0 { ImClamp(new_size.x, cr.min.x, cr.max.x) } else { window.SizeFull.x.clone() };
        new_size.y = if cr.min.y >= 0.0 && cr.max.y >= 0.0 { ImClamp(new_size.y, cr.min.y, cr.max.y) } else { window.SizeFull.y.clone() };
        if g.NextWindowData.SizeCallback
        {
            let mut data = ImGuiSizeCallbackData::default();
            data.UserData = g.NextWindowData.SizeCallbackUserData.unwrap().clone();
            data.Pos = window.position;
            data.CurrentSize = window.SizeFull;
            data.DesiredSize = new_size;
            g.NextWindowData.SizeCallback(&data);
            new_size = data.DesiredSize;
        }
        new_size.x = IM_FLOOR(new_size.x);
        new_size.y = IM_FLOOR(new_size.y);
    }

    // Minimum size
    if flag_clear(window.Flags.clone() , (ImGuiWindowFlags_ChildWindow | ImGuiWindowFlags_AlwaysAutoResize))
    {
        let mut window_for_height: *mut ImguiWindow =  find::GetWindowForTitleAndMenuHeight(window);
        let decoration_up_height: c_float =  window_for_height.TitleBarHeight() + window_for_height.MenuBarHeight();
        new_size = ImMax(new_size, g.style.WindowMinSize);
        new_size.y = ImMax(new_size.y, decoration_up_height + ImMax(0.0, g.style.WindowRounding.clone() - 1.0)); // Reduce artifacts with very small windows
    }
    return new_size;
}


pub fn CalcWindowAutoFitSize(g: &mut ImguiContext, window: &ImguiWindow, size_contents: &ImVec2) -> ImVec2 {
    // let g = GImGui; // ImGuiContext& g = *GImGui;
    let style = g.style.borrow_mut();
    let decoration_up_height: c_float = window.TitleBarHeight() + window.MenuBarHeight();
    let size_pad: ImVec2 = window.WindowPadding * 2.0;
    let size_desired: ImVec2 = size_contents + size_pad + ImVec2::from_floats(0.0, decoration_up_height);
    if flag_set(window.Flags.clone(), ImGuiWindowFlags_Tooltip) {
        // Tooltip always resize
        return size_desired;
    } else {
        // Maximum window size is determined by the viewport size or monitor size
        let is_popup: bool = flag_set(window.Flags.clone(), ImGuiWindowFlags_Popup);
        let is_menu: bool = flag_set(window.Flags.clone(), ImGuiWindowFlags_ChildMenu);
        let mut size_min: ImVec2 = style.WindowMinSize;
        if is_popup || is_menu { // Popups and menus bypass style.WindowMinSize by default, but we give then a non-zero minimum size to facilitate understanding problematic cases (e.g. empty popups)
            size_min = ImMin(size_min, ImVec2::from_floats(4.0, 4.0));
        }

        // FIXME-VIEWPORT-WORKAREA: May want to use GetWorkSize() instead of Size depending on the type of windows?
        let mut avail_size: ImVec2 = window.Viewport.Size;
        if window.ViewportOwned {
            avail_size = ImVec2::from_floats(f32::MAX, f32::MAX);
        }
        let monitor_idx: c_int = window.ViewportAllowPlatformMonitorExtend.clone();
        if monitor_idx >= 0 && monitor_idx < g.PlatformIO.Monitors.Size {
            avail_size = g.PlatformIO.Monitors[monitor_idx].WorkSize;
        }
        let mut size_auto_fit: ImVec2 = ImClamp(size_desired, size_min, ImMax(size_min, avail_size - style.DisplaySafeAreaPadding * 2.0));

        // When the window cannot fit all contents (either because of constraints, either because screen is too small),
        // we are growing the size on the other axis to compensate for expected scrollbar. FIXME: Might turn bigger than ViewportSize-WindowPadding.
        let size_auto_fit_after_constraint: ImVec2 = CalcWindowSizeAfterConstraint(g, window, &size_auto_fit);
        let mut will_have_scrollbar_x: bool = (size_auto_fit_after_constraint.x - size_pad.x - 0.0 < size_contents.x && flag_clear(window.Flags.clone(), ImGuiWindowFlags_NoScrollbar) && flag_set(window.Flags.clone(), ImGuiWindowFlags_HorizontalScrollbar)) || flag_set(window.Flags.clone(), ImGuiWindowFlags_AlwaysHorizontalScrollbar);
        let mut will_have_scrollbar_y: bool = (size_auto_fit_after_constraint.y - size_pad.y - decoration_up_height.clone() < size_contents.y && flag_clear(window.Flags.clone(), ImGuiWindowFlags_NoScrollbar)) || flag_set(window.Flags.clone(), ImGuiWindowFlags_AlwaysVerticalScrollbar);
        if will_have_scrollbar_x {
            size_auto_fit.y += style.ScrollbarSize.clone();
        }
        if will_have_scrollbar_y {
            size_auto_fit.x += style.ScrollbarSize.clone();
        }
        return size_auto_fit;
    }
}




pub fn CalcWindowNextAutoFitSize(g: &mut ImguiContext, window: &ImguiWindow) -> ImVec2
{
    let mut size_contents_current: ImVec2 = ImVec2::default();
    let mut size_contents_ideal: ImVec2 = ImVec2::default();
    CalcWindowContentSizes(window, &size_contents_current, &size_contents_ideal);
    let size_auto_fit = CalcWindowAutoFitSize(g, window, &size_contents_ideal);
    let size_final = CalcWindowSizeAfterConstraint(g, window, &size_auto_fit);
    return size_final;
}


pub fn GetWindowBgColorIdx(window: &mut ImguiWindow) -> ImGuiCol {
    if flag_set(window.Flags.clone(), ImGuiWindowFlags_Tooltip | ImGuiWindowFlags_Popup) {
        return ImGuiCol_PopupBg;
    }
    if flag_set(window.Flags.clone(), ImGuiWindowFlags_ChildWindow) && !window.DockIsActive.clone() {
        return ImGuiCol_ChildBg;
    }
    return ImGuiCol_WindowBg;
}


pub unsafe fn CalcResizePosSizeFromAnyCorner(window: &mut ImguiWindow, corner_target: &ImVec2, corner_norm: &ImVec2, out_pos: *mut ImVec2, out_size: *mut ImVec2) {
    let pos_min: ImVec2 = ImLerpVec22(corner_target, &window.position, corner_norm);                // Expected window upper-left
    let pos_max: ImVec2 = ImLerpVec22(window.position + window.Size, corner_target, corner_norm); // Expected window lower-right
    let size_expected: ImVec2 = pos_max - pos_min;
    let size_constrained: ImVec2 = CalcWindowSizeAfterConstraint(g, window, &size_expected);
    *out_pos = pos_min;
    if corner_norm.x == 0.0 {
        out_pos.x -= (size_constrained.x - size_expected.x);
    }
    if corner_norm.y == 0.0 {
        out_pos.y -= (size_constrained.y - size_expected.y);
    }
    *out_size = size_constrained.clone();
}


pub fn GetResizeBorderRect(window: &mut ImguiWindow, border_n: c_int, perp_padding: c_float, thickness: c_float) -> ImRect {
    let mut rect: ImRect = window.Rect();
    if thickness == 0.0 {
        rect.max -= ImVec2::from_floats(1.0, 1.0);
    }
    if border_n == ImGuiDir_Left {
        return ImRect(rect.min.x - thickness, rect.min.y + perp_padding, rect.min.x.clone() + thickness.clone(), rect.max.y - perp_padding.clone()); }
    if border_n == ImGuiDir_Right {
        return ImRect(rect.max.x - thickness, rect.min.y + perp_padding, rect.max.x.clone() + thickness.clone(), rect.max.y - perp_padding.clone()); }
    if border_n == ImGuiDir_Up {
        return ImRect(rect.min.x + perp_padding, rect.min.y - thickness, rect.max.x - perp_padding.clone(), rect.min.y.clone() + thickness.clone()); }
    if border_n == ImGuiDir_Down {
        return ImRect(rect.min.x + perp_padding, rect.max.y - thickness, rect.max.x - perp_padding.clone(), rect.max.y.clone() + thickness.clone()); }
    // IM_ASSERT(0);
    return ImRect::default();
}

// 0..3: corners (Lower-right, Lower-left, Unused, Unused)
pub unsafe fn GetWindowResizeCornerID(window: &mut ImguiWindow, n: c_int) -> ImguiHandle {
    // IM_ASSERT(n >= 0 && n < 4);
    let mut id: ImguiHandle = if window.DockIsActive {
        window.DockNode.Hostwindow.ID } else {
        window.ID.clone() };
    id = hash_string(str_to_const_c_char_ptr("#RESIZE"), id as u32);
    id = hash_data(&n, sizeof, id as u32);
    return id;
}


// Borders (Left, Right, Up, Down)
pub unsafe fn GetWindowResizeBorderID(window: &mut ImguiWindow, dir: ImGuiDir) -> ImguiHandle {
    // IM_ASSERT(dir >= 0 && dir < 4);
    let n: c_int = dir + 4;
    let mut id: ImguiHandle = if window.DockIsActive { window.DockNode.Hostwindow.ID } else { window.ID.clone() };
    id = hash_string(str_to_const_c_char_ptr("#RESIZE"), id as u32);
    id = hash_data(&n, sizeof, id as u32);
    return id;
}



// Handle resize for: Resize Grips, Borders, Gamepad
// Return true when using auto-fit (double click on resize grip)
pub unsafe fn UpdateWindowManualResize(window: &mut ImguiWindow, size_auto_fit: &ImVec2, border_held:  *mut c_int, resize_grip_count: c_int, mut resize_grip_col: [u32;4], visibility_rect: &ImRect) -> bool
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    flags: ImGuiWindowFlags = window.Flags.clone();

    if flag_set(flags, ImGuiWindowFlags_NoResize) || flag_set(flags, ImGuiWindowFlags_AlwaysAutoResize) || window.AutoFitFramesX > 0 || window.AutoFitFramesY > 0 {
        return false;
    }
    if window.WasActive == false { // Early out to avoid running this code for e.g. an hidden implicit/fallback Debug window.
        return false;
    }

    let mut ret_auto_fit: bool =  false;
    let resize_border_count: c_int = if g.IO.ConfigWindowsResizeFromEdges { 4 } else { 0 };
    let grip_draw_size: c_float =  IM_FLOOR(ImMax(g.FontSize.clone() * 1.35, window.WindowRounding.clone() + 1.0 + g.FontSize.clone() * 0.20));
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
    let clip_with_viewport_rect: bool = flag_clear(g.IO.BackendFlags.clone() , ImGuiBackendFlags_HasMouseHoveredViewport) || (g.IO.MouseHoveredViewport != window.ViewportId) || flag_clear(window.Viewport.Flags.clone() , ImGuiViewportFlags_NoDecoration);
    if clip_with_viewport_rect {
        window.ClipRect = window.Viewport.GetMainRect().ToVec4();
    }

    // Resize grips and borders are on layer 1
    window.dc.NavLayerCurrent = ImGuiNavLayer_Menu;

    // Manual resize grips
    PushID("#RESIZE");
    // for (let resize_grip_n: c_int = 0; resize_grip_n < resize_grip_count; resize_grip_n++)
    for resize_grip_n in 0 .. resize_grip_count
    {
        let def = resize_grip_def[resize_grip_n];
        let corner: ImVec2 = ImLerp(window.position, window.position + window.Size, def.CornerPosN);

        // Using the FlattenChilds button flag we make the resize button accessible even if we are hovering over a child window
        let mut hovered = false;
        let mut held = false;
        let mut resize_rect: ImRect = ImRect::new(corner - def.InnerDir * grip_hover_outer_size.clone() + corner + def.InnerDir * grip_hover_inner_size.clone());
        if resize_rect.min.x > resize_rect.max.x { mem::swap(&mut resize_rect.min.x, &mut resize_rect.max.x); }
        if resize_rect.min.y > resize_rect.max.y { mem::swap(&mut resize_rect.min.y, &mut resize_rect.max.y); }
        let mut resize_grip_id: ImguiHandle =  window.id_by_int(g, resize_grip_n.clone()); // == GetWindowResizeCornerID()
        KeepAliveID(resize_grip_id);
        ButtonBehavior(resize_rect, resize_grip_id.clone(), &hovered, &held, ImGuiButtonFlags_FlattenChildren | ImGuiButtonFlags_NoNavFocus);
        //GetForegroundDrawList(window).AddRect(resize_rect.Min, resize_rect.Max, IM_COL32(255, 255, 0, 255));
        if hovered || held {
            g.MouseCursor = if resize_grip_n.clone() & 1 { ImGuiMouseCursor_ResizeNESW } else { ImGuiMouseCursor_ResizeNWSE };
        }

        if held.clone() && g.IO.MouseClickedCount[0] == 2 && resize_grip_n == 0
        {
            // Manual auto-fit when double-clicking
            size_target = CalcWindowSizeAfterConstraint(g, window, size_auto_fit);
            ret_auto_fit = true;
            ClearActiveID();
        }
        else if held
        {
            // Resize from any of the four corners
            // We don't use an incremental MouseDelta but rather compute an absolute target size based on mouse position
            let clamp_min: ImVec2 = ImVec2::from_floats(if def.CornerPosN.x == 1.0 { visibility_rect.min.x.clone() } else { -f32::MAX }, if def.CornerPosN.y == 1.0 { visibility_rect.min.y.clone() } else { -f32::MAX });
            let clamp_max: ImVec2 = ImVec2::from_floats(if def.CornerPosN.x == 0.0 { visibility_rect.max.x.clone() } else { f32::MAX }, if def.CornerPosN.y == 0.0 { visibility_rect.max.y.clone() } else { f32::MAX });
            let mut corner_target: ImVec2 = g.IO.MousePos - g.ActiveIdClickOffset + ImLerp(def.InnerDir * grip_hover_outer_size.clone(), def.InnerDir * -grip_hover_inner_size.clone(), def.CornerPosN); // Corner of the window corresponding to our corner grip
            corner_target = ImClamp(corner_target, clamp_min, clamp_max);
            CalcResizePosSizeFromAnyCorner(window, &corner_target, def.CornerPosN, &mut pos_target, &mut size_target);
        }

        // Only lower-left grip is visible before hovering/activating
        if resize_grip_n == 0 || held.clone() || hovered.clone() {
            resize_grip_col[resize_grip_n.clone()] = GetColorU32(if held { ImGuiCol_ResizeGripActive } else {
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
        let axis = if border_n == ImGuiDir_Left || border_n == ImGuiDir_Right { ImGuiAxis_X } else { ImGuiAxis_Y };

        // hovered: bool, held;
        let mut hovered = false;
        let mut held = false;
        let border_rect: ImRect =  GetResizeBorderRect(window, border_n.clone(), grip_hover_inner_size.clone(), WINDOWS_HOVER_PADDING);
        let mut border_id: ImguiHandle =  window.id_by_int(g, border_n.clone() + 4); // == GetWindowResizeBorderID()
        KeepAliveID(border_id);
        ButtonBehavior(border_rect, border_id.clone(), &hovered, &held, ImGuiButtonFlags_FlattenChildren | ImGuiButtonFlags_NoNavFocus);
        //GetForegroundDrawLists(window).AddRect(border_rect.Min, border_rect.Max, IM_COL32(255, 255, 0, 255));
        if (hovered && g.HoveredIdTimer > WINDOWS_RESIZE_FROM_EDGES_FEEDBACK_TIMER) || held
        {
            g.MouseCursor = if axis == ImGuiAxis_X { ImGuiMouseCursor_ResizeEW } else { ImGuiMouseCursor_ResizeNS };
            if held {
                *border_held = border_n.clone();
            }
        }
        if held
        {
            let clamp_min = ImVec2::from_floats(if border_n == ImGuiDir_Right { visibility_rect.min.x.clone() } else { -f32::MAX }, if border_n == ImGuiDir_Down { visibility_rect.min.y.clone() } else { -f32::MAX });
            let clamp_max = ImVec2::New(if border_n == ImGuiDir_Left { visibility_rect.max.x.clone() } else { f32::MAX }, if border_n == ImGuiDir_Up { visibility_rect.max.y.clone() } else { f32::MAX });
            let mut border_target: ImVec2 = window.position;
            border_target[axis] = g.IO.MousePos[axis.clone()] - g.ActiveIdClickOffset[axis.clone()] + WINDOWS_HOVER_PADDING;
            border_target = ImClamp(border_target, clamp_min, clamp_max);
            CalcResizePosSizeFromAnyCorner(window, &border_target, ImMin(def.SegmentN1, def.SegmentN2), &mut pos_target, &mut size_target);
        }
    }
    PopID();

    // Restore nav layer
    window.dc.NavLayerCurrent = ImGuiNavLayer_Main;

    // Navigation resize (keyboard/gamepad)
    // FIXME: This cannot be moved to NavUpdateWindowing() because CalcWindowSizeAfterConstraint() need to callback into user.
    // Not even sure the callback works here.
    if is_not_null(g.NavWindowingTarget) && g.NavWindowingTarget.RootWindowDockTree == window
    {
        let mut nav_resize_dir = ImVec2::default();
        if g.NavInputSource == ImGuiInputSource_Keyboard && g.IO.KeyShift.clone() {
            nav_resize_dir = GetKeyVector2d(ImGuiKey_LeftArrow, ImGuiKey_RightArrow, ImGuiKey_UpArrow, ImGuiKey_DownArrow);
        }
        if g.NavInputSource == ImGuiInputSource_Gamepad {
            nav_resize_dir = GetKeyVector2d(ImGuiKey_GamepadDpadLeft, ImGuiKey_GamepadDpadRight, ImGuiKey_GamepadDpadUp, ImGuiKey_GamepadDpadDown);
        }
        if nav_resize_dir.x != 0.0 || nav_resize_dir.y != 0.0
        {
            let NAV_RESIZE_SPEED: c_float =  600;
            let resize_step: c_float =  NAV_RESIZE_SPEED * g.IO.DeltaTime.clone() * ImMin(g.IO.DisplayFramebufferScale.x.clone(), g.IO.DisplayFramebufferScale.y.clone());
            g.NavWindowingAccumDeltaSize += nav_resize_dir * resize_step;
            g.NavWindowingAccumDeltaSize = ImMax(g.NavWindowingAccumDeltaSize, visibility_rect.min - window.position - window.Size); // We need Pos+Size >= visibility_rect.Min, so Size >= visibility_rect.Min - Pos, so size_delta >= visibility_rect.Min - window.position - window.Size
            g.NavWindowingToggleLayer = false;
            g.NavDisableMouseHover = true;
            resize_grip_col[0] = GetColorU32(ImGuiCol_ResizeGripActive, 0.0);
            let accum_floored: ImVec2 = ImFloor(g.NavWindowingAccumDeltaSize);
            if accum_floored.x != 0.0 || accum_floored.y != 0.0
            {
                // FIXME-NAV: Should store and accumulate into a separate size buffer to handle sizing constraints properly, right now a constraint will make us stuck.
                size_target = CalcWindowSizeAfterConstraint(g, window, window.SizeFull + accum_floored);
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
        window.position = ImFloor(pos_target);
        MarkIniSettingsDirty(window);
    }

    window.Size = window.SizeFull;
    return ret_auto_fit;
}



// Push a new Dear ImGui window to add widgets to.
// - A default window called "Debug" is automatically stacked at the beginning of every frame so you can use widgets without explicitly calling a Begin/End pair.
// - Begin/End can be called multiple times during the frame with the same window name to append content.
// - The window name is used as a unique identifier to preserve window information across frames (and save rudimentary information to the .ini file).
//   You can use the "##" or "###" markers to use the same label with different id, or same id with different label. See documentation at the top of this file.
// - Return false when window is collapsed, so you can early out in your code. You always need to call End() even if false is returned.
// - Passing 'bool* p_open' displays a Close button on the upper-right corner of the window, the pointed value will be set to false when the button is pressed.
pub fn Begin(g: &mut ImguiContext, name: &String, p_open: Option<&mut bool>) -> bool
{
    // let g = GImGui; // ImGuiContext& g = *GImGui;
    let style = &mut g.style;
    // IM_ASSERT(name != NULL && name[0] != '\0');     // Window name required
    // IM_ASSERT(g.WithinFrameScope);                  // Forgot to call NewFrame()
    // IM_ASSERT(g.FrameCountEnded != g.FrameCount);   // Called Render() or EndFrame() and haven't called NewFrame() again yet

    // Find or create
    let mut window =  FindWindowByName(g,name);
    let window_just_created = window.is_none();
    if window_just_created {
        window = CreateNewWindow(, name, flags);
    }

    // Automatically disable manual moving/resizing when NoInputs is set
    if (flags.clone() & ImGuiWindowFlags_NoInputs) == ImGuiWindowFlags_NoInputs {
        flags |= ImGuiWindowFlags_NoMove | ImGuiWindowFlags_NoResize;
    }

    if flags.clone() & ImGuiWindowFlags_NavFlattened {}
        // IM_ASSERT(flags & ImGuiWindowFlags_ChildWindow);

    let current_frame: c_int = g.FrameCount.clone();
    let first_begin_of_the_frame: bool = (window.LastFrameActive != current_frame);
    window.IsFallbackWindow = (g.CurrentWindowStack.Size == 0 && g.WithinFrameScopeWithImplicitWindow.clone());

    // Update the Appearing flag (note: the BeginDocked() path may also set this to true later)
    let mut window_just_activated_by_user: bool =  (window.LastFrameActive < current_frame - 1); // Not using !WasActive because the implicit "Debug" window would always toggle off->on
    if flags.clone() & ImGuiWindowFlags_Popup
    {
        ImGuiPopupData& popup_ref = g.OpenPopupStack[g.BeginPopupStack.len()];
        window_just_activated_by_user |= (window.PopupId != popup_ref.PopupId); // We recycle popups so treat window as activated if popup id changed
        window_just_activated_by_user |= (window != popup_ref.Window);
    }

    // Update Flags, LastFrameActive, BeginOrderXXX fields
    let window_was_appearing: bool = window.Appearing.clone();
    if first_begin_of_the_frame
    {
        UpdateWindowInFocusOrderList(window, window_just_created, flags.clone());
        window.Appearing = window_just_activated_by_user;
        if window.Appearing {
            SetWindowConditionAllowFlags(window, ImGuiCond_Appearing, true);
        }
        window.FlagsPreviousFrame = window.Flags.clone();
        window.Flags = flags.clone();
        window.LastFrameActive = current_frame.clone();
        window.LastTimeActive = g.Time.clone() as c_float;
        window.BeginOrderWithinParent = 0;
        window.BeginOrderWithinContext = (g.WindowsActiveCount.clone()) as c_short;
        g.WindowsActiveCount += 1;
    }
    else
    {
        flags = window.Flags.clone();
    }

    // Docking
    // (NB: during the frame dock nodes are created, it is possible that (window.DockIsActive == false) even though (window.DockNode.Windows.Size > 1)
    // IM_ASSERT(window.DockNode == NULL || window.DockNodeAsHost == NULL); // Cannot be both
    if g.NextWindowData.Flags.clone() & ImGuiNextWindowDataFlags_HasDock {
        SetWindowDock(window, g.NextWindowData.DockId.clone(), g.NextWindowData.DockCond.clone());
    }
    if first_begin_of_the_frame
    {
        let mut has_dock_node: bool =  (window.DockId != 0 || window.DockNode != null_mut());
        let mut new_auto_dock_node: bool =  !has_dock_node && GetWindowAlwaysWantOwnTabBar(window);
        let mut dock_node_was_visible: bool =  window.DockNodeIsVisible.clone();
        let mut dock_tab_was_visible: bool =  window.DockTabIsVisible.clone();
        if has_dock_node.clone() || new_auto_dock_node
        {
            BeginDocked(window, p_open);
            flags = window.Flags.clone();
            if window.DockIsActive
            {
                // IM_ASSERT(window.DockNode != NULL);
                g.NextWindowData.Flags &= !ImGuiNextWindowDataFlags_HasSizeConstraint; // Docking currently override constraints
            }

            // Amend the Appearing flag
            if window.DockTabIsVisible.clone() && !dock_tab_was_visible && dock_node_was_visible && !window.Appearing.clone() && !window_was_appearing
            {
                window.Appearing = true;
                SetWindowConditionAllowFlags(window, ImGuiCond_Appearing, true);
            }
        }
        else
        {
            window.DockIsActive = false;
            window.DockNodeIsVisible = false;
            window.DockTabIsVisible = false;
        }
    }

    // Parent window is latched only on the first call to Begin() of the frame, so further append-calls can be done from a different window stack
    let mut parent_window_in_stack: *mut ImguiWindow =  if window.DockIsActive.clone() && is_not_null(window.DockNode.HostWindow) { window.DockNode.HostWindow } else {
        if g.CurrentWindowStack.empty() {
            None
        } else { g.CurrentWindowStack.last().unwrap().Window }
    };
    let mut parent_window: &mut ImguiWindow =  if first_begin_of_the_frame {
        if flags.clone() & (ImGuiWindowFlags_ChildWindow | ImGuiWindowFlags_Popup) {
            parent_window_in_stack
        } else { None }
    } else { window.ParentWindow };
    // IM_ASSERT(parent_window != NULL || flag_clear(flags, ImGuiWindowFlags_ChildWindow));

    // We allow window memory to be compacted so recreate the base stack when needed.
    if window.id_stack.Size == 0 {
        window.id_stack.push(window.ID.clone());
    }

    // Add to stack
    // We intentionally set g.CurrentWindow to NULL to prevent usage until when the viewport is set, then will call SetCurrentWindow()
    g.CurrentWindow = window;
    let mut window_stack_data = ImGuiWindowStackData::default();
    window_stack_data.Window = window;
    window_stack_data.ParentLastItemDataBackup = g.last_item_data.clone();
    window_stack_data.StackSizesOnBegin.SetToCurrentState();
    g.CurrentWindowStack.push(window_stack_data);
    g.CurrentWindow= None;
    if flags.clone() & ImGuiWindowFlags_ChildMenu {
        g.BeginMenuCount += 1;
    }

    if flags.clone() & ImGuiWindowFlags_Popup
    {
        ImGuiPopupData& popup_ref = g.OpenPopupStack[g.BeginPopupStack.len()];
        popup_ref.Window = window;
        popup_ref.ParentNavLayer = parent_window_in_stack.dc.NavLayerCurrent;
        g.BeginPopupStack.push(popup_re0f32);
        window.PopupId = popup_ref.PopupId;
    }

    // Update ->RootWindow and others pointers (before any possible call to FocusWindow)
    if first_begin_of_the_frame
    {
        UpdateWindowParentAndRootLinks(window, flags.clone(), parent_window);
        window.ParentWindowInBeginStack = parent_window_in_stack;
    }

    // Process SetNextWindow***() calls
    // (FIXME: Consider splitting the HasXXX flags into X/Y components
    let mut window_pos_set_by_api: bool =  false;
    let mut window_size_x_set_by_api: bool =  false;
    let mut window_size_y_set_by_api = false;
    if g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasPos
    {
        window_pos_set_by_api = flag_set(window.SetWindowPosAllowFlags.clone(), g.NextWindowData.PosCond.clone());
        if window_pos_set_by_api && ImLengthSqr(g.NextWindowData.PosPivotVal) > 0.001
        {
            // May be processed on the next frame if this is our first frame and we are measuring size
            // FIXME: Look into removing the branch so everything can go through this same code path for consistency.
            window.SetWindowPosVal = g.NextWindowData.PosVal;
            window.SetWindowPosPivot = g.NextWindowData.PosPivotVal;
            window.SetWindowPosAllowFlags &= !(ImGuiCond_Once | ImGuiCond_FirstUseEver | ImGuiCond_Appearing);
        }
        else
        {
            SetWindowPos(window, g.NextWindowData.PosVal, g.NextWindowData.PosCond);
        }
    }
    if g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasSize
    {
        window_size_x_set_by_api = flag_set(window.SetWindowSizeAllowFlags.clone(), g.NextWindowData.SizeCond.clone()) && (g.NextWindowData.SizeVal.x > 0.0);
        window_size_y_set_by_api = flag_set(window.SetWindowSizeAllowFlags.clone(), g.NextWindowData.SizeCond.clone()) && (g.NextWindowData.SizeVal.y > 0.0);
        SetWindowSize(window, g.NextWindowData.SizeVal, g.NextWindowData.SizeCond.clone());
    }
    if flag_set(g.NextWindowData.Flags.clone(), ImGuiNextWindowDataFlags_HasScroll)
    {
        if g.NextWindowData.ScrollVal.x >= 0.0
        {
            window.scrollTarget.x = g.NextWindowData.ScrollVal.x.clone();
            window.scrollTargetCenterRatio.x = 0.0;
        }
        if g.NextWindowData.ScrollVal.y >= 0.0
        {
            window.scrollTarget.y = g.NextWindowData.ScrollVal.y.clone();
            window.scrollTargetCenterRatio.y = 0.0;
        }
    }
    if flag_set(g.NextWindowData.Flags.clone(), ImGuiNextWindowDataFlags_HasContentSize) {
        window.ContentSizeExplicit = g.NextWindowData.ContentSizeVal;
    }
    else if first_begin_of_the_frame {
    window.ContentSizeExplicit = ImVec2::from_floats(0.0, 0.0);
}
    if flag_set(g.NextWindowData.Flags.clone(), ImGuiNextWindowDataFlags_HasWindowClass) {
        window.WindowClass = g.NextWindowData.WindowClass;
    }
    if flag_set(g.NextWindowData.Flags.clone(), ImGuiNextWindowDataFlags_HasCollapsed) {
        SetWindowCollapsed(window, g.NextWindowData.CollapsedVal.clone(), g.NextWindowData.CollapsedCond.clone());
    }
    if flag_set(g.NextWindowData.Flags.clone(), ImGuiNextWindowDataFlags_HasFocus) {
        focus::FocusWindow(window);
    }
    if window.Appearing {
        SetWindowConditionAllowFlags(window, ImGuiCond_Appearing, false);
    }

    // When reusing window again multiple times a frame, just append content (don't need to setup again)
    if first_begin_of_the_frame
    {
        // Initialize
        let window_is_child_tooltip: bool = flag_set(flags.clone(), ImGuiWindowFlags_ChildWindow) && flag_set(flags.clone(), ImGuiWindowFlags_Tooltip); // FIXME-WIP: Undocumented behavior of Child+Tooltip for pinned tooltip (#1345)
        let window_just_appearing_after_hidden_for_resize: bool = (window.HiddenFramesCannotSkipItems > 0);
        window.Active = true;
        window.HasCloseButton = (p_open != null_mut());
        window.ClipRect = ImVec4(-f32::MAX, -f32::MAX, f32::MAX, f32::MAX);
        window.id_stack.resize(1,0);
        window.DrawList._ResetForNewFrame();
        window.dc.CurrentTableIdx = -1;
        if flag_set(flags.clone(), ImGuiWindowFlags_DockNodeHost)
        {
            window.DrawList.ChannelsSplit(2);
            window.DrawList.ChannelsSetCurrent(DOCKING_HOST_DRAW_CHANNEL_FG); // Render decorations on channel 1 as we will render the backgrounds manually later
        }

        // Restore buffer capacity when woken from a compacted state, to avoid
        if window.MemoryCompacted {
            GcAwakeTransientWindowBuffers(window);
        }

        // Update stored window name when it changes (which can _only_ happen with the "###" operator, so the ID would stay unchanged).
        // The title bar always display the 'name' parameter, so we only update the string storage if it needs to be visible to the end-user elsewhere.
        let mut window_title_visible_elsewhere: bool =  false;
        if (is_not_null(window.Viewport) && window.Viewport.Window == window) || (window.DockIsActive.clone()) {
            window_title_visible_elsewhere = true;
        }
        else if g.NavWindowingListWindow != None && flag_clear(window.Flags.clone(), ImGuiWindowFlags_NoNavFocus) {  // Window titles visible when using CTRL+TAB
            window_title_visible_elsewhere = true;
        }
        if window_title_visible_elsewhere && !window_just_created && strcmp(name, window.Name) != 0
        {
            buf_len: size_t = window.NameBufLen;
            window.Name = ImStrdupcpy(window.Name, &mut buf_len, name);
            window.NameBufLen = buf_len;
        }

        // UPDATE CONTENTS SIZE, UPDATE HIDDEN STATUS

        // Update contents size from last frame for auto-fitting (or use explicit size)
        CalcWindowContentSizes(window, &window.ContentSize, &window.ContentSizeIdeal);

        // FIXME: These flags are decremented before they are used. This means that in order to have these fields produce their intended behaviors
        // for one frame we must set them to at least 2, which is counter-intuitive. HiddenFramesCannotSkipItems is a more complicated case because
        // it has a single usage before this code block and may be set below before it is finally checked.
        if window.HiddenFramesCanSkipItems > 0 {
            window.HiddenFramesCanSkipItems -= 1;
        }
        if window.HiddenFramesCannotSkipItems > 0 {
            window.HiddenFramesCannotSkipItems -= 1;
        }
        if window.HiddenFramesForRenderOnly > 0 {
            window.HiddenFramesForRenderOnly -= 1;
        }

        // Hide new windows for one frame until they calculate their size
        if (window_just_created && (!window_size_x_set_by_api || !window_size_y_set_by_api)) {
            window.HiddenFramesCannotSkipItems = 1;
        }

        // Hide popup/tooltip window when re-opening while we measure size (because we recycle the windows)
        // We reset Size/ContentSize for reappearing popups/tooltips early in this function, so further code won't be tempted to use the old size.
        if window_just_activated_by_user && (flags & (ImGuiWindowFlags_Popup | ImGuiWindowFlags_Tooltip)) != 0
        {
            window.HiddenFramesCannotSkipItems = 1;
            if flags & ImGuiWindowFlags_AlwaysAutoResize
            {
                if !window_size_x_set_by_api {
                    window.Size.x = 0.0; window.SizeFull.x = 0.0;
                }
                if !window_size_y_set_by_api {
                    window.Size.y = 0.0; window.SizeFull.y = 0.0;
                }
                window.ContentSize = ImVec2::default();
                window.ContentSizeIdeal = ImVec2::default();
            }
        }

        // SELECT VIEWPORT
        // We need to do this before using any style/font sizes, as viewport with a different DPI may affect font sizes.

        WindowSelectViewport(window);
        SetCurrentViewport(window, window.Viewport);
        window.FontDpiScale = if flag_set(g.IO.ConfigFlags, ImGuiConfigFlags_DpiEnableScaleFonts) { window.Viewport.DpiScale } else { 1.0 };
        SetCurrentWindow(g, window);
        flags = window.Flags;

        // LOCK BORDER SIZE AND PADDING FOR THE FRAME (so that altering them doesn't cause inconsistencies)
        // We read Style data after the call to UpdateSelectWindowViewport() which might be swapping the style.

        if flag_set(flags, ImGuiWindowFlags_ChildWindow) {
            window.WindowBorderSize = style.ChildBorderSize;
        }
        else {
            window.WindowBorderSize = if flag_set(flags , (ImGuiWindowFlags_Popup | ImGuiWindowFlags_Tooltip)) && flag_clear(flags, ImGuiWindowFlags_Modal) {
                style.PopupBorderSize
            } else { style.WindowBorderSize };
        }
        if !window.DockIsActive && flag_set(flags, ImGuiWindowFlags_ChildWindow) && flag_clear(flags, (ImGuiWindowFlags_AlwaysUseWindowPadding | ImGuiWindowFlags_Popup)) && window.WindowBorderSize == 0.0 {
            window.WindowPadding = ImVec2::from_floats(0.0, if flag_set(flags, ImGuiWindowFlags_MenuBar) { style.WindowPadding.y }else { 0.0 });
        }
        else {
            window.WindowPadding = style.WindowPadding;
        }

        // Lock menu offset so size calculation can use it as menu-bar windows need a minimum size.
        window.dc.MenuBarOffset.x = ImMax(ImMax(window.WindowPadding.x, style.item_spacing.x), g.NextWindowData.MenuBarOffsetMinVal.x);
        window.dc.MenuBarOffset.y = g.NextWindowData.MenuBarOffsetMinVal.y;

        // Collapse window by double-clicking on title bar
        // At this point we don't have a clipping rectangle setup yet, so we can use the title bar area for hit detection and drawing
        if flag_clear(flags, ImGuiWindowFlags_NoTitleBar) && flag_clear(flags, ImGuiWindowFlags_NoCollapse) && !window.DockIsActive
        {
            // We don't use a regular button+id to test for double-click on title bar (mostly due to legacy reason, could be fixed), so verify that we don't have items over the title bar.
            let title_bar_rect: ImRect =  window.TitleBarRect();
            if g.HoveredWindow == window && g.HoveredId == 0 && g.HoveredIdPreviousFrame == 0 && IsMouseHoveringRect(&title_bar_rect.min, &title_bar_rect.max, false) && g.IO.MouseClickedCount[0] == 2 {
                window.WantCollapseToggle = true;
            }
            if window.WantCollapseToggle
            {
                window.Collapsed = !window.Collapsed;
                MarkIniSettingsDirty(window);
            }
        }
        else
        {
            window.Collapsed = false;
        }
        window.WantCollapseToggle = false;

        // SIZE

        // Calculate auto-fit size, handle automatic resize
        let size_auto_fit: ImVec2 = CalcWindowAutoFitSize(, window, &window.ContentSizeIdeal);
        let mut use_current_size_for_scrollbar_x: bool =  window_just_created;
        let mut use_current_size_for_scrollbar_y: bool =  window_just_created;
        if flag_set(flags, ImGuiWindowFlags_AlwaysAutoResize) && !window.Collapsed
        {
            // Using SetNextWindowSize() overrides ImGuiWindowFlags_AlwaysAutoResize, so it can be used on tooltips/popups, etc.
            if (!window_size_x_set_by_api)
            {
                window.SizeFull.x = size_auto_fit.x;
                use_current_size_for_scrollbar_x = true;
            }
            if (!window_size_y_set_by_api)
            {
                window.SizeFull.y = size_auto_fit.y;
                use_current_size_for_scrollbar_y = true;
            }
        }
        else if (window.AutoFitFramesX > 0 || window.AutoFitFramesY > 0)
        {
            // Auto-fit may only grow window during the first few frames
            // We still process initial auto-fit on collapsed windows to get a window width, but otherwise don't honor ImGuiWindowFlags_AlwaysAutoResize when collapsed.
            if (!window_size_x_set_by_api && window.AutoFitFramesX > 0)
            {
                window.SizeFull.x = if window.AutoFitOnlyGrows { ImMax(window.SizeFull.x, size_auto_fit.x) } else { size_auto_fit.x };
                use_current_size_for_scrollbar_x = true;
            }
            if (!window_size_y_set_by_api && window.AutoFitFramesY > 0)
            {
                window.SizeFull.y = if window.AutoFitOnlyGrows { ImMax(window.SizeFull.y, size_auto_fit.y) } else { size_auto_fit.y };
                use_current_size_for_scrollbar_y = true;
            }
            if (!window.Collapsed) {
                MarkIniSettingsDirty(window);
            }
        }

        // Apply minimum/maximum window size constraints and final size
        window.SizeFull = CalcWindowSizeAfterConstraint(g, window, &window.SizeFull);
        window.Size = if window.Collapsed && flag_clear(flags, ImGuiWindowFlags_ChildWindow) { window.TitleBarRect().GetSize() } else { window.SizeFull };

        // Decoration size
        let decoration_up_height: c_float =  window.TitleBarHeight() + window.MenuBarHeight();

        // POSITION

        // Popup latch its initial position, will position itself when it appears next frame
        if window_just_activated_by_user
        {
            window.AutoPosLastDirection = ImGuiDir_None;
            if flag_set(flags, ImGuiWindowFlags_Popup) && flag_clear(flags, ImGuiWindowFlags_Modal) && !window_pos_set_by_api {// FIXME: BeginPopup() could use SetNextWindowPos()
                window.position = g.BeginPopupStack.last().unwrap().OpenPopupPos;
            }
        }

        // Position child window
        if flags & ImGuiWindowFlags_ChildWindow
        {
            // IM_ASSERT(parent_window && parent_window.Active);
            window.BeginOrderWithinParent = parent_window.dc.ChildWindows.Size;
            parent_window.dc.ChildWindows.push(window);
            if flag_clear(flags, ImGuiWindowFlags_Popup) && !window_pos_set_by_api && !window_is_child_tooltip {
                window.position = parent_window.dc.cursor_pos;
            }
        }

        let window_pos_with_pivot: bool = (window.SetWindowPosVal.x != f32::MAX && window.HiddenFramesCannotSkipItems == 0);
        if (window_pos_with_pivot) {
            SetWindowPos(window, window.SetWindowPosVal - window.Size * window.SetWindowPosPivot, 0);
        }// Position given a pivot (e.g. for centering)
        else if (flag_set(flags, ImGuiWindowFlags_ChildMenu)) {
            window.position = FindBestWindowPosForPopup(window);
        }
        else if (flag_set(flags, ImGuiWindowFlags_Popup) && !window_pos_set_by_api && window_just_appearing_after_hidden_for_resize) {
            window.position = FindBestWindowPosForPopup(window);
        }
        else if (flag_set(flags, ImGuiWindowFlags_Tooltip) && !window_pos_set_by_api && !window_is_child_tooltip) {
            window.position = FindBestWindowPosForPopup(window);
        }

        // Late create viewport if we don't fit within our current host viewport.
        if (window.ViewportAllowPlatformMonitorExtend >= 0 && !window.ViewportOwned && !(window.Viewport.Flags & ImGuiViewportFlags_Minimized)) {
            if (!window.Viewport.GetMainRect().Contains(window.Rect())) {
                // This is based on the assumption that the DPI will be known ahead (same as the DPI of the selection done in UpdateSelectWindowViewport)
                //old_viewport: *mut ImGuiViewport = window.Viewport;
                window.Viewport = AddUpdateViewport(window, window.ID, window.position, window.Size, ImGuiViewportFlags_NoFocusOnAppearing);

                // FIXME-DPI
                //IM_ASSERT(old_viewport.DpiScale == window.Viewport->DpiScale); // FIXME-DPI: Something went wrong
                SetCurrentViewport(window, window.Viewport);
                window.FontDpiScale = if (g.IO.ConfigFlags & ImGuiConfigFlags_DpiEnableScaleFonts) {
                    window.Viewport.DpiScale
                } else { 1.0 };
                SetCurrentWindow(g, window);
            }
        }

        if (window.ViewportOwned) {
            WindowSyncOwnedViewport(window, parent_window_in_stack);
        }

        // Calculate the range of allowed position for that window (to be movable and visible past safe area padding)
        // When clamping to stay visible, we will enforce that window.position stays inside of visibility_rect.
        let mut viewport_rect: ImRect = ImRect::new(window.Viewport.GetMainRect());
        let mut viewport_work_rect: ImRect = ImRect::new(window.Viewport.GetWorkRect());
        let visibility_padding: ImVec2 = ImMax(style.DisplayWindowPadding, style.DisplaySafeAreaPadding);
        let mut visibility_rect: ImRect = ImRect::new(viewport_work_rect.min + visibility_padding, viewport_work_rect.max - visibility_padding);

        // Clamp position/size so window stays visible within its viewport or monitor
        // Ignore zero-sized display explicitly to avoid losing positions if a window manager reports zero-sized window when initializing or minimizing.
        // FIXME: Similar to code in GetWindowAllowedExtentRect()
        if (!window_pos_set_by_api && flag_clear(flags, ImGuiWindowFlags_ChildWindow) && window.AutoFitFramesX <= 0 && window.AutoFitFramesY <= 0)
        {
            if (!window.ViewportOwned && viewport_rect.GetWidth() > 0.0 && viewport_rect.GetHeight() > 0.0)
            {
                ClampWindowRect(window, &visibility_rect);
            }
            else if (window.ViewportOwned && g.PlatformIO.Monitors.len() > 0)
            {
                // Lost windows (e.g. a monitor disconnected) will naturally moved to the fallback/dummy monitor aka the main viewport.
                let monitor: *const ImGuiPlatformMonitor = GetViewportPlatformMonitor(window.Viewport);
                visibility_rect.min = monitor.WorkPos + visibility_padding;
                visibility_rect.max = monitor.WorkPos + monitor.WorkSize - visibility_padding;
                ClampWindowRect(window, &visibility_rect);
            }
        }
        window.position = ImFloor(window.position);

        // Lock window rounding for the frame (so that altering them doesn't cause inconsistencies)
        // Large values tend to lead to variety of artifacts and are not recommended.
        if window.ViewportOwned || window.DockIsActive {
            window.WindowRounding = 0.0;
        }
        else {
            window.WindowRounding = if flag_set(flags, ImGuiWindowFlags_ChildWindow) {
                style.ChildRounding
            } else {
                if flag_set(flags, ImGuiWindowFlags_Popup) && flag_clear(flags, ImGuiWindowFlags_Modal) {
                    style.PopupRounding
                } else { style.WindowRounding }
            };
        }
        // For windows with title bar or menu bar, we clamp to FrameHeight(FontSize + FramePadding.y * 2.0) to completely hide artifacts.
        //if ((window.Flags & ImGuiWindowFlags_MenuBar) || flag_clear(window.Flags, ImGuiWindowFlags_NoTitleBar))
        //    window.WindowRounding = ImMin(window.WindowRounding, g.FontSize + style.FramePadding.y * 2.0);

        // Apply window focus (new and reactivated windows are moved to front)
        let mut want_focus: bool =  false;
        if window_just_activated_by_user && flag_clear(flags, ImGuiWindowFlags_NoFocusOnAppearing)
        {
            if flags & ImGuiWindowFlags_Popup {
                want_focus = true;
            }
            else if (window.DockIsActive || flag_clear(flags, ImGuiWindowFlags_ChildWindow)) && flag_clear(flags, ImGuiWindowFlags_Tooltip) {
                want_focus = true;
            }

            let mut modal: *mut ImguiWindow =  GetTopMostPopupModal();
            if modal != None && !IsWindowWithinBeginStackOf(window, modal)
            {
                // Avoid focusing a window that is created outside of active modal. This will prevent active modal from being closed.
                // Since window is not focused it would reappear at the same display position like the last time it was visible.
                // In case of completely new windows it would go to the top (over current modal), but input to such window would still be blocked by modal.
                // Position window behind a modal that is not a begin-parent of this window.
                want_focus = false;
                if window == window.RootWindow
                {
                    let mut blocking_modal: *mut ImguiWindow =  FindBlockingModal(window);
                    // IM_ASSERT(blocking_modal != NULL);
                    BringWindowToDisplayBehind(window, blocking_modal);
                }
            }
        }

        // [Test Engine] Register whole window in the item system
// #ifdef IMGUI_ENABLE_TEST_ENGINE
        if g.TestEngineHookItems
        {
            // IM_ASSERT(window.id_stack.Size == 1);
            window.id_stack.Size = 0; // As window.id_stack[0] == window.ID here, make sure TestEngine doesn't erroneously see window as parent of itself.
            IMGUI_TEST_ENGINE_ITEM_ADD(window.Rect(), window.ID);
            IMGUI_TEST_ENGINE_ITEM_INFO(window.ID, window.Name, if g.HoveredWindow == window { ImGuiItemStatusFlags_HoveredRect }else { 0 });
            window.id_stack.Size = 1;
        }
// #endif

        // Decide if we are going to handle borders and resize grips
        let handle_borders_and_resize_grips: bool = (is_not_null(window.DockNodeAsHost) || !window.DockIsActive);

        // Handle manual resize: Resize Grips, Borders, Gamepad
        let mut border_held: c_int = -1;
        resize_grip_col: u32[4] = {};
        let resize_grip_count: c_int = if g.IO.ConfigWindowsResizeFromEdges { 2 } else { 1 }; // Allow resize from lower-left if we have the mouse cursor feedback for it.
        let resize_grip_draw_size: c_float =  IM_FLOOR(ImMax(g.FontSize * 1.10.0, window.WindowRounding + 1.0 + g.FontSize * 0.20));
        if handle_borders_and_resize_grips && !window.Collapsed {
            if UpdateWindowManualResize(window, &size_auto_fit, &mut border_held, resize_grip_count, resize_grip_col[0], &visibility_rect) {
                use_current_size_for_scrollbar_x = true;
                use_current_size_for_scrollbar_y = true;
            }
        }
        window.ResizeBorderHeld = border_held as i8;

        // Synchronize window --> viewport again and one last time (clamping and manual resize may have affected either)
        if (window.ViewportOwned)
        {
            if (!window.Viewport.PlatformRequestMove) {
                window.Viewport.Pos = window.position;
            }
            if (!window.Viewport.PlatformRequestResize) {
                window.Viewport.Size = window.Size;
            }
            window.Viewport.UpdateWorkRect();
            viewport_rect = window.Viewport.GetMainRect();
        }

        // Save last known viewport position within the window itself (so it can be saved in .ini file and restored)
        window.ViewportPos = window.Viewport.Pos;

        // SCROLLBAR VISIBILITY

        // Update scrollbar visibility (based on the Size that was effective during last frame or the auto-resized Size).
        if !window.Collapsed
        {
            // When reading the current size we need to read it after size constraints have been applied.
            // When we use InnerRect here we are intentionally reading last frame size, same for ScrollbarSizes values before we set them again.
            let avail_size_from_current_frame: ImVec2 = ImVec2::from_floats(window.SizeFull.x, window.SizeFull.y - decoration_up_height);
            let avail_size_from_last_frame: ImVec2 = window.InnerRect.GetSize() + window.scrollbarSizes;
            let needed_size_from_last_frame: ImVec2 = if window_just_created { ImVec2::from_floats(0.0, 0.0) } else { window.ContentSize + window.WindowPadding * 2.0 };
            let size_x_for_scrollbars: c_float =  if use_current_size_for_scrollbar_x { avail_size_from_current_frame.x } else { avail_size_from_last_frame.x };
            let size_y_for_scrollbars: c_float =  if use_current_size_for_scrollbar_y { avail_size_from_current_frame.y } else { avail_size_from_last_frame.y };
            //scrollbar_y_from_last_frame: bool = window.scrollbarY; // FIXME: May want to use that in the ScrollbarX expression? How many pros vs cons?
            window.scrollbarY = flag_set(flags, ImGuiWindowFlags_AlwaysVerticalScrollbar) || ((needed_size_from_last_frame.y > size_y_for_scrollbars) && flag_clear(flags, ImGuiWindowFlags_NoScrollbar));
            // window.scrollbarX = flag_set(flags, ImGuiWindowFlags_AlwaysHorizontalScrollbar) || (needed_size_from_last_frame.x > size_x_for_scrollbars - window.scrollbarY { style.ScrollbarSize } else { 0.0 }) && flag_clear(flags, ImGuiWindowFlags_NoScrollbar) && flag_set(flags, ImGuiWindowFlags_HorizontalScrollbar);
            if window.scrollbarX && !window.scrollbarY {
                window.scrollbarY = (needed_size_from_last_frame.y > size_y_for_scrollbars) && flag_clear(flags, ImGuiWindowFlags_NoScrollbar);
            }
            window.scrollbarSizes = ImVec2::from_floats(if window.scrollbarY { style.ScrollbarSize }else { 0.0 }, if window.scrollbarX { style.ScrollbarSize }else { 0.0 });
        }

        // UPDATE RECTANGLES (1- THOSE NOT AFFECTED BY SCROLLING)
        // Update various regions. Variables they depends on should be set above in this function.
        // We set this up after processing the resize grip so that our rectangles doesn't lag by a frame.

        // Outer rectangle
        // Not affected by window border size. Used by:
        // - FindHoveredWindow() (w/ extra padding when border resize is enabled)
        // - Begin() initial clipping rect for drawing window background and borders.
        // - Begin() clipping whole child
        let host_rect: ImRect = if flag_set(flags, ImGuiWindowFlags_ChildWindow) && flag_clear(flags, ImGuiWindowFlags_Popup) && !window_is_child_tooltip { ImRect::from_vec4(&parent_window.ClipRect) } else { viewport_rect };
        let outer_rect: ImRect =  window.Rect();
        let title_bar_rect: ImRect =  window.TitleBarRect();
        window.OuterRectClipped = outer_rect;
        if window.DockIsActive {
            window.OuterRectClipped.Min.y += window.TitleBarHeight();
        }
        window.OuterRectClipped.ClipWith(host_rect);

        // Inner rectangle
        // Not affected by window border size. Used by:
        // - InnerClipRect
        // - ScrollToRectEx()
        // - NavUpdatePageUpPageDown()
        // - Scrollbar()
        window.InnerRect.Min.x = window.position.x;
        window.InnerRect.Min.y = window.position.y + decoration_up_height;
        window.InnerRect.Max.x = window.position.x + window.Size.x - window.scrollbarSizes.x;
        window.InnerRect.Max.y = window.position.y + window.Size.y - window.scrollbarSizes.y;

        // Inner clipping rectangle.
        // Will extend a little bit outside the normal work region.
        // This is to allow e.g. Selectable or CollapsingHeader or some separators to cover that space.
        // Force round operator last to ensure that e.g. (max.x-min.x) in user's render code produce correct result.
        // Note that if our window is collapsed we will end up with an inverted (~null) clipping rectangle which is the correct behavior.
        // Affected by window/frame border size. Used by:
        // - Begin() initial clip rect
        let top_border_size: c_float =  if flag_set(flags, ImGuiWindowFlags_MenuBar) || flag_clear(flags, ImGuiWindowFlags_NoTitleBar) { style.FrameBorderSize }else { window.WindowBorderSize };
        window.InnerClipRect.Min.x = ImFloor(0.5 + window.InnerRect.Min.x + ImMax(ImFloor(window.WindowPadding.x * 0.5), window.WindowBorderSize));
        window.InnerClipRect.Min.y = ImFloor(0.5 + window.InnerRect.Min.y + top_border_size);
        window.InnerClipRect.Max.x = ImFloor(0.5 + window.InnerRect.Max.x - ImMax(ImFloor(window.WindowPadding.x * 0.5), window.WindowBorderSize));
        window.InnerClipRect.Max.y = ImFloor(0.5 + window.InnerRect.Max.y - window.WindowBorderSize);
        window.InnerClipRect.ClipWithFull(&host_rect);

        // Default item width. Make it proportional to window size if window manually resizes
        if window.Size.x > 0.0 && flag_clear(flags, ImGuiWindowFlags_Tooltip) && flag_clear(flags, ImGuiWindowFlags_AlwaysAutoResize) {
            window.ItemWidthDefault = ImFloor(window.Size.x * 0.650f32);
        }
        else {
            window.ItemWidthDefault = ImFloor(g.FontSize * 16.0);
        }

        // SCROLLING

        // Lock down maximum scrolling
        // The value of ScrollMax are ahead from ScrollbarX/ScrollbarY which is intentionally using InnerRect from previous rect in order to accommodate
        // for right/bottom aligned items without creating a scrollbar.
        window.scrollMax.x = ImMax(0.0, window.ContentSize.x + window.WindowPadding.x * 2.0 - window.InnerRect.GetWidth());
        window.scrollMax.y = ImMax(0.0, window.ContentSize.y + window.WindowPadding.y * 2.0 - window.InnerRect.GetHeight());

        // Apply scrolling
        window.scroll = CalcNextScrollFromScrollTargetAndClamp(window);
        window.scrollTarget = ImVec2::from_floats(f32::MAX, f32::MAX);

        // DRAWING

        // Setup draw list and outer clipping rectangle
        // IM_ASSERT(window.DrawList.CmdBuffer.Size == 1 && window.DrawList.CmdBuffer[0].ElemCount == 0);
        window.DrawList.PushTextureID(g.Font.ContainerAtlas.TexID);
        PushClipRect(g, &host_rect.min, &host_rect.max, false);

        // Child windows can render their decoration (bg color, border, scrollbars, etc.) within their parent to save a draw call (since 1.71)
        // When using overlapping child windows, this will break the assumption that child z-order is mapped to submission order.
        // FIXME: User code may rely on explicit sorting of overlapping child window and would need to disable this somehow. Please get in contact if you are affected (github #4493)
        let is_undocked_or_docked_visible: bool = !window.DockIsActive || window.DockTabIsVisible;
        if is_undocked_or_docked_visible
        {
            let mut render_decorations_in_parent: bool =  false;
            if flag_set(flags, ImGuiWindowFlags_ChildWindow) && flag_clear(flags, ImGuiWindowFlags_Popup) && !window_is_child_tooltip
            {
                // - We test overlap with the previous child window only (testing all would end up being O(log N) not a good investment here)
                // - We disable this when the parent window has zero vertices, which is a common pattern leading to laying out multiple overlapping childs
                let mut previous_child: *mut ImguiWindow =  if parent_window.dc.ChildWindows.Size >= 2 { parent_window.dc.ChildWindows[parent_window.dc.ChildWindows.Size - 2] } else { None };
                let mut previous_child_overlapping: bool = if previous_child { previous_child.Rect().Overlaps(window.Rect()) } else { false };
                let mut parent_is_empty: bool =  parent_window.DrawList.VtxBuffer.len() > 0;
                if window.DrawList.CmdBuffer.last().unwrap().ElemCount == 0 && parent_is_empty && !previous_child_overlapping {
                    render_decorations_in_parent = true;
                }
            }
            if render_decorations_in_parent {
                window.DrawList = parent_window.DrawList;
            }

            // Handle title bar, scrollbar, resize grips and resize borders
            let window_to_highlight: *const ImguiWindow = if g.NavWindowingTarget { g.NavWindowingTarget } else { g.NavWindow };
            let title_bar_is_highlight: bool = want_focus || (is_not_null(window_to_highlight) && (window.RootWindowForTitleBarHighlight == window_to_highlight.RootWindowForTitleBarHighlight || (is_not_null(window.DockNode) && window.DockNode == window_to_highlight.DockNode)));
            RenderWindowDecorations(window, &title_bar_rect, title_bar_is_highlight, handle_borders_and_resize_grips, resize_grip_count, resize_grip_col, resize_grip_draw_size);

            if render_decorations_in_parent {
                window.DrawList = window.DrawListInst.clone();
            }
        }

        // UPDATE RECTANGLES (2- THOSE AFFECTED BY SCROLLING)

        // Work rectangle.
        // Affected by window padding and border size. Used by:
        // - Columns() for right-most edge
        // - TreeNode(), CollapsingHeader() for right-most edge
        // - BeginTabBar() for right-most edge
        let allow_scrollbar_x: bool = flag_clear(flags, ImGuiWindowFlags_NoScrollbar) && flag_clear(flags, ImGuiWindowFlags_HorizontalScrollbar);
        let allow_scrollbar_y: bool = flag_clear(flags, ImGuiWindowFlags_NoScrollbar);
        let work_rect_size_x: c_float =  if window.ContentSizeExplicit.x != 0.0 { window.ContentSizeExplicit.x } else { ImMax(if allow_scrollbar_x { window.ContentSize.x } else { 0.0 }, window.Size.x - window.WindowPadding.x * 2.0 - window.scrollbarSizes.x) };
        let work_rect_size_y: c_float =  if window.ContentSizeExplicit.y != 0.0 { window.ContentSizeExplicit.y } else { ImMax(if allow_scrollbar_y { window.ContentSize.y } else { 0.0 }, window.Size.y - window.WindowPadding.y * 2.0 - decoration_up_height - window.scrollbarSizes.y) };
        window.work_rect.Min.x = ImFloor(window.InnerRect.Min.x - window.scroll.x + ImMax(window.WindowPadding.x, window.WindowBorderSize));
        window.work_rect.Min.y = ImFloor(window.InnerRect.Min.y - window.scroll.y + ImMax(window.WindowPadding.y, window.WindowBorderSize));
        window.work_rect.Max.x = window.work_rect.Min.x + work_rect_size_x;
        window.work_rect.Max.y = window.work_rect.Min.y + work_rect_size_y;
        window.ParentWorkRect = window.work_rect;

        // [LEGACY] Content Region
        // FIXME-OBSOLETE: window.content_region_rect.Max is currently very misleading / partly faulty, but some BeginChild() patterns relies on it.
        // Used by:
        // - Mouse wheel scrolling + many other things
        window.content_region_rect.Min.x = window.position.x - window.scroll.x + window.WindowPadding.x;
        window.content_region_rect.Min.y = window.position.y - window.scroll.y + window.WindowPadding.y + decoration_up_height;
        window.content_region_rect.Max.x = window.content_region_rect.Min.x + (if window.ContentSizeExplicit.x != 0.0 { window.ContentSizeExplicit.x } else { window.Size.x - window.WindowPadding.x * 2.0 - window.scrollbarSizes.x });
        window.content_region_rect.Max.y = window.content_region_rect.Min.y + (if window.ContentSizeExplicit.y != 0.0 { window.ContentSizeExplicit.y } else { (window.Size.y - window.WindowPadding.y * 2.0 - decoration_up_height - window.scrollbarSizes.y) });

        // Setup drawing context
        // (NB: That term "drawing context / DC" lost its meaning a long time ago. Initially was meant to hold transient data only. Nowadays difference between window. and window.dc-> is dubious.)
        window.dc.indent.x = 0.0 + window.WindowPadding.x - window.scroll.x;
        window.dc.group_offset.x = 0.0;
        window.dc.ColumnsOffset.x = 0.0;

        // Record the loss of precision of CursorStartPos which can happen due to really large scrolling amount.
        // This is used by clipper to compensate and fix the most common use case of large scroll area. Easy and cheap, next best thing compared to switching everything to double or u64.
        let start_pos_highp_x = window.position.x + window.WindowPadding.x - window.scroll.x + window.dc.ColumnsOffset.x;
        let start_pos_highp_y = window.position.y + window.WindowPadding.y - window.scroll.y + decoration_up_height;
        window.dc.CursorStartPos  = ImVec2::from_floats(start_pos_highp_x, start_pos_highp_y);
        window.dc.CursorStartPosLossyness = ImVec2::from_floats((start_pos_highp_x - window.dc.CursorStartPos.x), (start_pos_highp_y - window.dc.CursorStartPos.y));
        window.dc.cursor_pos = window.dc.CursorStartPos;
        window.dc.cursor_pos_prev_line = window.dc.cursor_pos;
        window.dc.CursorMaxPos = window.dc.CursorStartPos;
        window.dc.IdealMaxPos = window.dc.CursorStartPos;
        window.dc.CurrLineSize = ImVec2::from_floats(0.0, 0.0);
        window.dc.PrevLineSize = ImVec2::from_floats(0.0, 0.0);
        window.dc.CurrLineTextBaseOffset = 0.0;
        window.dc.PrevLineTextBaseOffset = 0.0;
        window.dc.IsSameLine = false;
        window.dc.is_set_pos = false;

        window.dc.NavLayerCurrent = ImGuiNavLayer_Main;
        window.dc.NavLayersActiveMask = window.dc.NavLayersActiveMaskNext;
        window.dc.NavLayersActiveMaskNext = 0x00;
        window.dc.NavHideHighlightOneFrame = false;
        window.dc.NavHasScroll = (window.scrollMax.y > 0.0);

        window.dc.MenuBarAppending = false;
        window.dc.MenuColumns.Update(style.item_spacing.x, window_just_activated_by_user);
        window.dc.TreeDepth = 0;
        window.dc.TreeJumpToParentOnPopMask = 0x00;
        window.dc.ChildWindows.clear();
        window.dc.StateStorage = &mut window.StateStorage;
        window.dc.CurrentColumns= None;
        window.dc.LayoutType = ImGuiLayoutType_Vertical;
        window.dc.ParentLayoutType = if parent_window { parent_window.dc.LayoutType } else { ImGuiLayoutType_Vertical };

        window.dc.ItemWidth = window.ItemWidthDefault;
        window.dc.TextWrapPos = -1.0; // disabled
        window.dc.ItemWidthStack.clear();
        window.dc.TextWrapPosStack.clear();

        if (window.AutoFitFramesX > 0) {
            window.AutoFitFramesX -= 1;
        }
        if (window.AutoFitFramesY > 0) {
            window.AutoFitFramesY -= 1;
        }

        // Apply focus (we need to call FocusWindow() AFTER setting DC.CursorStartPos so our initial navigation reference rectangle can start around there)
        if (want_focus)
        {
            focus::FocusWindow(window);
            NavInitWindow(window, false); // <-- this is in the way for us to be able to defer and sort reappearing FocusWindow() calls
        }

        // Close requested by platform window
        if (p_open != None && window.Viewport.PlatformRequestClose && window.Viewport != GetMainViewport())
        {
            if (!window.DockIsActive || window.DockTabIsVisible)
            {
                window.Viewport.PlatformRequestClose = false;
                g.NavWindowingToggleLayer = false; // Assume user mapped PlatformRequestClose on ALT-F4 so we disable ALT for menu toggle. False positive not an issue.
                IMGUI_DEBUG_LOG_VIEWPORT("[viewport] Window '{}' PlatformRequestClose\n", window.Name);
                *p_open = false;
            }
        }

        // Title bar
        if (flag_clear(flags, ImGuiWindowFlags_NoTitleBar) && !window.DockIsActive) {
            RenderWindowTitleBarContents(window, ImRect(title_bar_rect.min.x + window.WindowBorderSize, title_bar_rect.min.y, title_bar_rect.max.x - window.WindowBorderSize, title_bar_rect.max.y), name, p_open);
        }

        // Clear hit test shape every frame
        window.HitTestHoleSize.x = 0;
        window.HitTestHoleSize.y = 0;

        // Pressing CTRL+C while holding on a window copy its content to the clipboard
        // This works but 1. doesn't handle multiple Begin/End pairs, 2. recursing into another Begin/End pair - so we need to work that out and add better logging scope.
        // Maybe we can support CTRL+C on every element?
        /*
        //if (g.NavWindow == window && g.ActiveId == 0)
        if (g.ActiveId == window.MoveId)
            if (g.IO.KeyCtrl && IsKeyPressed(ImGuiKey_C))
                LogToClipboard();
        */

        if (g.IO.ConfigFlags & ImGuiConfigFlags_DockingEnable)
        {
            // Docking: Dragging a dockable window (or any of its child) turns it into a drag and drop source.
            // We need to do this _before_ we overwrite window.dc.LastItemId below because BeginDockableDragDropSource() also overwrites it.
            if ((g.MovingWindow == window) && (g.IO.ConfigDockingWithShift == g.IO.KeyShift)) {
                if ((window.RootWindowDockTree.Flags & ImGuiWindowFlags_NoDocking) == 0) {
                    BeginDockableDragDropSource(window);
                }
            }

            // Docking: Any dockable window can act as a target. For dock node hosts we call BeginDockableDragDropTarget() in DockNodeUpdate() instead.
            if (g.DragDropActive && flag_clear(flags, ImGuiWindowFlags_NoDocking)) {
                if (g.MovingWindow == None || g.Movingwindow.RootWindowDockTree != window) {
                    if ((window == window.RootWindowDockTree) && flag_clear(window.Flags, ImGuiWindowFlags_DockNodeHost)) {
                        BeginDockableDragDropTarget(window);
                    }
                }
            }
        }

        // We fill last item data based on Title Bar/Tab, in order for IsItemHovered() and IsItemActive() to be usable after Begin().
        // This is useful to allow creating context menus on title bar only, etc.
        if (window.DockIsActive) {
            set_last_item_data(g, window.MoveId, g.CurrentItemFlags, window.DockTabItemStatusFlags, &window.DockTabItemRect);
        }
        else {
            set_last_item_data(g, window.MoveId, g.CurrentItemFlags, if IsMouseHoveringRect(&title_bar_rect.min, &title_bar_rect.max, false) { ImGuiItemStatusFlags_HoveredRect }else { 0 }, &title_bar_rect);
        }

        // [Test Engine] Register title bar / tab
        if (flag_clear(window.Flags, ImGuiWindowFlags_NoTitleBar)) {
            IMGUI_TEST_ENGINE_ITEM_ADD(g.last_item_data.rect, g.last_item_data.id);
        }
    }
    else
    {
        // Append
        SetCurrentViewport(window, window.Viewport);
        SetCurrentWindow(g, window);
    }

    // Pull/inherit current state
    window.dc.NavFocusScopeIdCurrent = if flag_set(flags, ImGuiWindowFlags_ChildWindow)
    { parent_window.dc.NavFocusScopeIdCurrent } else { window.id_from_str(null(), str_to_const_c_char_ptr("#FOCUSSCOPE")) }; // Inherit from parent only // -V595

    if (flag_clear(flags, ImGuiWindowFlags_DockNodeHost)) {
        PushClipRect(g, &window.InnerClipRect.Min, &window.InnerClipRect.Max, true);
    }

    // Clear 'accessed' flag last thing (After PushClipRect which will set the flag. We want the flag to stay false when the default "Debug" window is unused)
    window.WriteAccessed = false;
    window.BeginCount+= 1;
    g.NextWindowData.ClearFlags();

    // Update visibility
    if (first_begin_of_the_frame)
    {
        // When we are about to select this tab (which will only be visible on the _next frame_), flag it with a non-zero HiddenFramesCannotSkipItems.
        // This will have the important effect of actually returning true in Begin() and not setting SkipItems, allowing an earlier submission of the window contents.
        // This is analogous to regular windows being hidden from one frame.
        // It is especially important as e.g. nested TabBars would otherwise generate flicker in the form of one empty frame, or focus requests won't be processed.
        if (window.DockIsActive && !window.DockTabIsVisible)
        {
            if (window.LastFrameJustFocused == g.FrameCount) {
                window.HiddenFramesCannotSkipItems = 1;
            }
            else {
                window.HiddenFramesCanSkipItems = 1;
            }
        }

        if (flags & ImGuiWindowFlags_ChildWindow)
        {
            // Child window can be out of sight and have "negative" clip windows.
            // Mark them as collapsed so commands are skipped earlier (we can't manually collapse them because they have no title bar).
            // IM_ASSERT((flags& ImGuiWindowFlags_NoTitleBar) != 0 || (window.DockIsActive));
            if (flag_clear(flags, ImGuiWindowFlags_AlwaysAutoResize) && window.AutoFitFramesX <= 0 && window.AutoFitFramesY <= 0) // FIXME: Doesn't make sense for ChildWindow??
            {
                let nav_request: bool = flag_set(flags, ImGuiWindowFlags_NavFlattened) && (g.NavAnyRequest && is_not_null(g.NavWindow) && g.NavWindow.RootWindowForNav == window.RootWindowForNav);
                if (!g.LogEnabled && !nav_request) {
                    if (window.OuterRectClipped.Min.x >= window.OuterRectClipped.Max.x || window.OuterRectClipped.Min.y >= window.OuterRectClipped.Max.y) {
                        window.HiddenFramesCanSkipItems = 1;
                    }
                }
            }

            // Hide along with parent or if parent is collapsed
            if (is_not_null(parent_window) && (parent_window.Collapsed || parent_window.HiddenFramesCanSkipItems > 0)) {
                window.HiddenFramesCanSkipItems = 1;
            }
            if (is_not_null(parent_window) && (parent_window.Collapsed || parent_window.HiddenFramesCannotSkipItems > 0)) {
                window.HiddenFramesCannotSkipItems = 1;
            }
        }

        // Don't render if style alpha is 0.0 at the time of Begin(). This is arbitrary and inconsistent but has been there for a long while (may remove at some point)
        if (style.Alpha <= 0.0) {
            window.HiddenFramesCanSkipItems = 1;
        }

        // Update the Hidden flag
        let mut hidden_regular: bool =  (window.HiddenFramesCanSkipItems > 0) || (window.HiddenFramesCannotSkipItems > 0);
        window.Hidden = hidden_regular || (window.HiddenFramesForRenderOnly > 0);

        // Disable inputs for requested number of frames
        if (window.DisableInputsFrames > 0)
        {
            window.DisableInputsFrames-= 1;
            window.Flags |= ImGuiWindowFlags_NoInputs;
        }

        // Update the SkipItems flag, used to early out of all items functions (no layout required)
        let mut skip_items: bool =  false;
        if (window.Collapsed || !window.Active || hidden_regular) {
            if (window.AutoFitFramesX <= 0 && window.AutoFitFramesY <= 0 && window.HiddenFramesCannotSkipItems <= 0) {
                skip_items = true;
            }
        }
        window.skip_items = skip_items;

        // Restore NavLayersActiveMaskNext to previous value when not visible, so a CTRL+Tab back can use a safe value.
        if (window.skip_items) {
            window.dc.NavLayersActiveMaskNext = window.dc.NavLayersActiveMask;
        }

        // Sanity check: there are two spots which can set Appearing = true
        // - when 'window_just_activated_by_user' is set -> HiddenFramesCannotSkipItems is set -> SkipItems always false
        // - in BeginDocked() path when DockNodeIsVisible == DockTabIsVisible == true -> hidden _should_ be all zero // FIXME: Not formally proven, hence the assert.
        if (window.skip_items && !window.Appearing) {}
            // IM_ASSERT(window.Appearing == false); // Please report on GitHub if this triggers: https://github.com/ocornut/imgui/issues/4177
    }

    return !window.skip_items;
}



pub unsafe fn End()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window  = g.current_window_mut().unwrap();

    // Error checking: verify that user hasn't called End() too many times!
    if (g.CurrentWindowStack.Size <= 1 && g.WithinFrameScopeWithImplicitWindow)
    {
        // IM_ASSERT_USER_ERROR(g.CurrentWindowStack.Size > 1, "Calling End() too many times!");
        return;
    }
    // IM_ASSERT(g.CurrentWindowStack.Size > 0);

    // Error checking: verify that user doesn't directly call End() on a child window.
    if ((window.Flags & ImGuiWindowFlags_ChildWindow) && flag_clear(window.Flags, ImGuiWindowFlags_DockNodeHost) && !window.DockIsActive) {}
        // IM_ASSERT_USER_ERROR(g.WithinEndChild, "Must call EndChild() and not End()!");

    // Close anything that is open
    if (window.dc.CurrentColumns) {
        EndColumns();
    }
    if (flag_clear(window.Flags, ImGuiWindowFlags_DockNodeHost)) {   // Pop inner window clip rectangle
        PopClipRect(g);
    }

    // Stop logging
    if (flag_clear(window.Flags, ImGuiWindowFlags_ChildWindow)) {   // FIXME: add more options for scope of logging
        LogFinish();
    }

    if (window.dc.is_set_pos) {
        ErrorCheckUsingSetCursorPosToExtendParentBoundaries(g);
    }

    // Docking: report contents sizes to parent to allow for auto-resize
    if is_not_null(window.DockNode) && window.DockTabIsVisible {
        let mut host_window: &mut ImguiWindow = window.DockNode.HostWindow;
        if is_not_null(host_window)
        {         // FIXME-DOCK
            host_window.dc.CursorMaxPos = window.dc.CursorMaxPos + window.WindowPadding - host_window.WindowPadding;
        }
    }

    // Pop from window stack
    g.LastItemData = g.CurrentWindowStack.last().unwrap().ParentLastItemDataBackup;
    if (window.Flags & ImGuiWindowFlags_ChildMenu) {
        g.BeginMenuCount -= 1;
    }
    if (window.Flags & ImGuiWindowFlags_Popup) {
        g.BeginPopupStack.pop_back();
    }
    g.CurrentWindowStack.last().unwrap().StackSizesOnBegin.CompareWithCurrentState();
    g.CurrentWindowStack.pop_back();
    SetCurrentWindow(g, if g.CurrentWindowStack.Size == 0 { None } else { g.CurrentWindowStack.last().unwrap().Window });
    if (g.CurrentWindow) {
        SetCurrentViewport(g.CurrentWindow, g.Currentwindow.Viewport);
    }
}


pub unsafe fn BringWindowToFocusFront(window: &mut ImguiWindow)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(window == window.RootWindow);

    let cur_order: c_int = window.FocusOrder as c_int;
    // IM_ASSERT(g.WindowsFocusOrder[cur_order] == window);
    if g.WindowsFocusOrder.last().unwrap() == window {
        return;
    }

    let new_order: c_int = g.WindowsFocusOrder.Size - 1;
    // for (let n: c_int = cur_order; n < new_order; n++)
    for n in cur_order ..new_order
    {
        g.WindowsFocusOrder[n] = g.WindowsFocusOrder[n + 1];
        g.WindowsFocusOrder[n].FocusOrder-= 1;
        // IM_ASSERT(g.WindowsFocusOrder[n]->FocusOrder == n);
    }
    g.WindowsFocusOrder[new_order] = window;
    window.FocusOrder = new_order as c_short;
}


pub unsafe fn BringWindowToDisplayFront(window: &mut ImguiWindow) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut current_front_window: &mut ImguiWindow = *g.Windows.last_mut().unwrap();
    if current_front_window == window || current_front_window.RootWindowDockTree == window { // Cheap early out (could be better)
        return;
    }
    // for (let i: c_int = g.Windows.len() - 2; i >= 0; i--)
    for i in g.Windows.len() - 2..0 { // We can ignore the top-most window
        if g.Windows[i] == window {
            libc::memmove(g.Windows[i], &g.Windows[i + 1], (g.Windows.len() - i - 1) * sizeof);
            g.Windows[g.Windows.len() - 1] = window;
            break;
        }
    }
}

pub unsafe fn BringWindowToDisplayBack(window: &mut ImguiWindow) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if g.Windows[0] == window {
        return;
    }
    // for (let i: c_int = 0; i < g.Windows.len(); i++)
    for i in 0..g.Windows.len() {
        if g.Windows[i] == window {
            libc::memmove(g.Windows[1], &g.Windows[0], i * sizeof);
            g.Windows[0] = window;
            break;
        }
    }
}

pub unsafe fn BringWindowToDisplayBehind(mut window: &mut ImguiWindow, mut behind_window: &mut ImguiWindow) {
    // IM_ASSERT(window != NULL && behind_window != NULL);
    let g = GImGui; // ImGuiContext& g = *GImGui;
    window = window.RootWindow;
    behind_window = behind_window.RootWindow;
    let pos_wnd: c_int = FindWindowDisplayIndex(window);
    let pos_beh: c_int = FindWindowDisplayIndex(behind_window);
    if pos_wnd < pos_beh {
        let copy_bytes: size_t = (pos_beh - pos_wnd - 1) * sizeof;
        libc::memmove(g.Windows.Data[pos_wnd], &g.Windows.Data[pos_wnd + 1], copy_bytes);
        g.Windows[pos_beh - 1] = window;
    } else {
        copy_bytes: size_t = (pos_wnd - pos_beh) * sizeof;
        libc::memmove(g.Windows.Data[pos_beh + 1], &g.Windows.Data[pos_beh], copy_bytes);
        g.Windows[pos_beh] = window;
    }
}


// BeginDisabled()/EndDisabled()
// - Those can be nested but it cannot be used to enable an already disabled section (a single BeginDisabled(true) in the stack is enough to keep everything disabled)
// - Visually this is currently altering alpha, but it is expected that in a future styling system this would work differently.
// - Feedback welcome at https://github.com/ocornut/imgui/issues/211
// - BeginDisabled(false) essentially does nothing useful but is provided to facilitate use of boolean expressions. If you can avoid calling BeginDisabled(False)/EndDisabled() best to avoid it.
// - Optimized shortcuts instead of PushStyleVar() + PushItemFlag()
pub unsafe fn BeginDisabled(disabled: bool)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut was_disabled: bool =  (g.CurrentItemFlags & ImGuiItemFlags_Disabled) != 0;
    if !was_disabled && disabled
    {
        g.DisabledAlphaBackup = g.style.Alpha;
        g.style.Alpha *= g.style.DisabledAlpha; // PushStyleVar(ImGuiStyleVar_Alpha, g.style.Alpha * g.style.DisabledAlpha);
    }
    if was_disabled || disabled {
        g.CurrentItemFlags |= ImGuiItemFlags_Disabled;
    }
    g.ItemFlagsStack.push(g.CurrentItemFlags);
    g.DisabledStackSize+= 1;
}


pub unsafe fn EndDisabled() {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.DisabledStackSize > 0);
    g.DisabledStackSize -= 1;
    let mut was_disabled: bool = (g.CurrentItemFlags & ImGuiItemFlags_Disabled) != 0;
    //PopItemFlag();
    g.ItemFlagsStack.pop_back();
    g.CurrentItemFlags = g.ItemFlagsStack.last().unwrap().clone();
    if was_disabled && (g.CurrentItemFlags & ImGuiItemFlags_Disabled) == 0 {
        g.style.Alpha = g.DisabledAlphaBackup; //PopStyleVar();}
    }
}
