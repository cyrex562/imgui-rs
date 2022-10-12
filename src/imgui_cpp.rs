// dear imgui, v1.89 WIP
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use libc;
use windows_sys::Win32;
use std::ptr::{null, null_mut};
use libc::{c_char, c_int, c_uchar, c_uint, c_void, size_t};
use crate::context::ImGuiContext;
use crate::storage::ImGuiStoragePair;





c_void End()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;

    // Error checking: verify that user hasn't called End() too many times!
    if (g.CurrentWindowStack.Size <= 1 && g.WithinFrameScopeWithImplicitWindow)
    {
        // IM_ASSERT_USER_ERROR(g.CurrentWindowStack.Size > 1, "Calling End() too many times!");
        return;
    }
    // IM_ASSERT(g.CurrentWindowStack.Size > 0);

    // Error checking: verify that user doesn't directly call End() on a child window.
    if ((window.Flags & ImGuiWindowFlags_ChildWindow) && !(window.Flags & ImGuiWindowFlags_DockNodeHost) && !window.DockIsActive)
        // IM_ASSERT_USER_ERROR(g.WithinEndChild, "Must call EndChild() and not End()!");

    // Close anything that is open
    if (window.DC.CurrentColumns)
        EndColumns();
    if (!(window.Flags & ImGuiWindowFlags_DockNodeHost))   // Pop inner window clip rectangle
        PopClipRect();

    // Stop logging
    if (!(window.Flags & ImGuiWindowFlags_ChildWindow))    // FIXME: add more options for scope of logging
        LogFinish();

    if (window.DC.IsSetPos)
        ErrorCheckUsingSetCursorPosToExtendParentBoundaries();

    // Docking: report contents sizes to parent to allow for auto-resize
    if (window.DockNode && window.DockTabIsVisible)
        if (let mut host_window: *mut ImGuiWindow =  window.DockNode.HostWindow)         // FIXME-DOCK
            host_window.DC.CursorMaxPos = window.DC.CursorMaxPos + window.WindowPadding - host_window.WindowPadding;

    // Pop from window stack
    g.LastItemData = g.CurrentWindowStack.last().unwrap().ParentLastItemDataBackup;
    if (window.Flags & ImGuiWindowFlags_ChildMenu)
        g.BeginMenuCount-= 1;
    if (window.Flags & ImGuiWindowFlags_Popup)
        g.BeginPopupStack.pop_back();
    g.CurrentWindowStack.last().unwrap().StackSizesOnBegin.CompareWithCurrentState();
    g.CurrentWindowStack.pop_back();
    SetCurrentWindow(g.CurrentWindowStack.Size == 0 ? null_mut() : g.CurrentWindowStack.last().unwrap().Window);
    if (g.CurrentWindow)
        SetCurrentViewport(g.CurrentWindow, g.Currentwindow.Viewport);
}

c_void BringWindowToFocusFront(window: *mut ImGuiWindow)
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

c_void BringWindowToDisplayFront(window: *mut ImGuiWindow)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut current_front_window: *mut ImGuiWindow =  g.Windows.last().unwrap();
    if (current_front_window == window || current_front_window.RootWindowDockTree == window) // Cheap early out (could be better)
        return;
    for (let i: c_int = g.Windows.len() - 2; i >= 0; i--) // We can ignore the top-most window
        if (g.Windows[i] == window)
        {
            memmove(&g.Windows[i], &g.Windows[i + 1], (g.Windows.len() - i - 1) * sizeof);
            g.Windows[g.Windows.len() - 1] = window;
            break;
        }
}

c_void BringWindowToDisplayBack(window: *mut ImGuiWindow)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.Windows[0] == window)
        return;
    for (let i: c_int = 0; i < g.Windows.len(); i++)
        if (g.Windows[i] == window)
        {
            memmove(&g.Windows[1], &g.Windows[0], i * sizeof);
            g.Windows[0] = window;
            break;
        }
}

c_void BringWindowToDisplayBehind(window: *mut ImGuiWindow, behind_window: *mut ImGuiWindow)
{
    // IM_ASSERT(window != NULL && behind_window != NULL);
    let g = GImGui; // ImGuiContext& g = *GImGui;
    window = window.RootWindow;
    behind_window = behind_window.RootWindow;
    let pos_wnd: c_int = FindWindowDisplayIndex(window);
    let pos_beh: c_int = FindWindowDisplayIndex(behind_window);
    if (pos_wnd < pos_beh)
    {
        copy_bytes: size_t = (pos_beh - pos_wnd - 1) * sizeof;
        memmove(&g.Windows.Data[pos_wnd], &g.Windows.Data[pos_wnd + 1], copy_bytes);
        g.Windows[pos_beh - 1] = window;
    }
    else
    {
        copy_bytes: size_t = (pos_wnd - pos_beh) * sizeof;
        memmove(&g.Windows.Data[pos_beh + 1], &g.Windows.Data[pos_beh], copy_bytes);
        g.Windows[pos_beh] = window;
    }
}

FindWindowDisplayIndex: c_int(window: *mut ImGuiWindow)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.Windows.index_from_ptr(g.Windows.find(window));
}

// Moving window to front of display and set focus (which happens to be back of our sorted list)
c_void FocusWindow(window: *mut ImGuiWindow)
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

c_void FocusTopMostWindowUnderOne(under_this_window: *mut ImGuiWindow, ignore_window: *mut ImGuiWindow)
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

// Important: this alone doesn't alter current ImDrawList state. This is called by PushFont/PopFont only.
c_void SetCurrentFont(font: *mut ImFont)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(font && font->IsLoaded());    // Font Atlas not created. Did you call io.Fonts.GetTexDataAsRGBA32 / GetTexDataAsAlpha8 ?
    // IM_ASSERT(font->Scale > 0.0);
    g.Font = font;
    g.FontBaseSize = ImMax(1.0, g.IO.FontGlobalScale * g.Font.FontSize * g.Font.Scale);
    g.FontSize = g.CurrentWindow ? g.Currentwindow.CalcFontSize() : 0.0;

    atlas: *mut ImFontAtlas = g.Font.ContainerAtlas;
    g.DrawListSharedData.TexUvWhitePixel = atlas->TexUvWhitePixel;
    g.DrawListSharedData.TexUvLines = atlas->TexUvLines;
    g.DrawListSharedData.Font = g.Font;
    g.DrawListSharedData.FontSize = g.FontSize;
}

c_void PushFont(font: *mut ImFont)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (!font)
        font = GetDefaultFont();
    SetCurrentFont(font);
    g.FontStack.push(font);
    g.Currentwindow.DrawList.PushTextureID(font->ContainerAtlas.TexID);
}

c_void  PopFont()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.Currentwindow.DrawList.PopTextureID();
    g.FontStack.pop_back();
    SetCurrentFont(g.FontStack.empty() ? GetDefaultFont() : g.FontStack.last().unwrap());
}

c_void PushItemFlag(option: ImGuiItemFlags, enabled: bool)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut item_flags: ImGuiItemFlags =  g.CurrentItemFlags;
    // IM_ASSERT(item_flags == g.ItemFlagsStack.back());
    if (enabled)
        item_flags |= option;
    else
        item_flags &= !option;
    g.CurrentItemFlags = item_flags;
    g.ItemFlagsStack.push(item_flags);
}

c_void PopItemFlag()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.ItemFlagsStack.Size > 1); // Too many calls to PopItemFlag() - we always leave a 0 at the bottom of the stack.
    g.ItemFlagsStack.pop_back();
    g.CurrentItemFlags = g.ItemFlagsStack.last().unwrap();
}

// BeginDisabled()/EndDisabled()
// - Those can be nested but it cannot be used to enable an already disabled section (a single BeginDisabled(true) in the stack is enough to keep everything disabled)
// - Visually this is currently altering alpha, but it is expected that in a future styling system this would work differently.
// - Feedback welcome at https://github.com/ocornut/imgui/issues/211
// - BeginDisabled(false) essentially does nothing useful but is provided to facilitate use of boolean expressions. If you can avoid calling BeginDisabled(False)/EndDisabled() best to avoid it.
// - Optimized shortcuts instead of PushStyleVar() + PushItemFlag()
c_void BeginDisabled(disabled: bool)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut was_disabled: bool =  (g.CurrentItemFlags & ImGuiItemFlags_Disabled) != 0;
    if (!was_disabled && disabled)
    {
        g.DisabledAlphaBackup = g.Style.Alpha;
        g.Style.Alpha *= g.Style.DisabledAlpha; // PushStyleVar(ImGuiStyleVar_Alpha, g.Style.Alpha * g.Style.DisabledAlpha);
    }
    if (was_disabled || disabled)
        g.CurrentItemFlags |= ImGuiItemFlags_Disabled;
    g.ItemFlagsStack.push(g.CurrentItemFlags);
    g.DisabledStackSize+= 1;
}

c_void EndDisabled()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.DisabledStackSize > 0);
    g.DisabledStackSize-= 1;
    let mut was_disabled: bool =  (g.CurrentItemFlags & ImGuiItemFlags_Disabled) != 0;
    //PopItemFlag();
    g.ItemFlagsStack.pop_back();
    g.CurrentItemFlags = g.ItemFlagsStack.last().unwrap();
    if (was_disabled && (g.CurrentItemFlags & ImGuiItemFlags_Disabled) == 0)
        g.Style.Alpha = g.DisabledAlphaBackup; //PopStyleVar();
}

// FIXME: Look into renaming this once we have settled the new Focus/Activation/TabStop system.
c_void PushAllowKeyboardFocus(allow_keyboard_focus: bool)
{
    PushItemFlag(ImGuiItemFlags_NoTabStop, !allow_keyboard_focus);
}

c_void PopAllowKeyboardFocus()
{
    PopItemFlag();
}

c_void PushButtonRepeat(repeat: bool)
{
    PushItemFlag(ImGuiItemFlags_ButtonRepeat, repeat);
}

c_void PopButtonRepeat()
{
    PopItemFlag();
}

c_void PushTextWrapPos(wrap_pos_x: c_float)
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    window.DC.TextWrapPosStack.push(window.DC.TextWrapPos);
    window.DC.TextWrapPos = wrap_pos_x;
}

c_void PopTextWrapPos()
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    window.DC.TextWrapPos = window.DC.TextWrapPosStack.last().unwrap();
    window.DC.TextWrapPosStack.pop_back();
}

static GetCombinedRootWindow: *mut ImGuiWindow(window: *mut ImGuiWindow, popup_hierarchy: bool, dock_hierarchy: bool)
{
    let mut last_window: *mut ImGuiWindow =  null_mut();
    while (last_window != window)
    {
        last_window = window;
        window = window.RootWindow;
        if (popup_hierarchy)
            window = window.RootWindowPopupTree;
		if (dock_hierarchy)
			window = window.RootWindowDockTree;
	}
    return window;
}

IsWindowChildOf: bool(window: *mut ImGuiWindow, potential_parent: *mut ImGuiWindow, popup_hierarchy: bool, dock_hierarchy: bool)
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

IsWindowWithinBeginStackOf: bool(window: *mut ImGuiWindow, potential_parent: *mut ImGuiWindow)
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

IsWindowAbove: bool(potential_above: *mut ImGuiWindow, potential_below: *mut ImGuiWindow)
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

IsWindowHovered: bool(ImGuiHoveredFlags flags)
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

        result: bool;
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

IsWindowFocused: bool(ImGuiFocusedFlags flags)
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

IsWindowDocked: bool()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.Currentwindow.DockIsActive;
}

// Can we focus this window with CTRL+TAB (or PadMenu + PadFocusPrev/PadFocusNext)
// Note that NoNavFocus makes the window not reachable with CTRL+TAB but it can still be focused with mouse or programmatically.
// If you want a window to never be focused, you may use the e.g. NoInputs flag.
IsWindowNavFocusable: bool(window: *mut ImGuiWindow)
{
    return window.WasActive && window == window.RootWindow && !(window.Flags & ImGuiWindowFlags_NoNavFocus);
}GetWindowWidth: c_float()
{
    let mut window: *mut ImGuiWindow =  GimGui.CurrentWindow;
    return window.Size.x;
}GetWindowHeight: c_float()
{
    let mut window: *mut ImGuiWindow =  GimGui.CurrentWindow;
    return window.Size.y;
}

GetWindowPos: ImVec2()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    return window.Pos;
}

c_void SetWindowPos(window: *mut ImGuiWindow, pos: &ImVec2, cond: ImGuiCond)
{
    // Test condition (NB: bit 0 is always true) and clear flags for next time
    if (cond && (window.SetWindowPosAllowFlags & cond) == 0)
        return;

    // IM_ASSERT(cond == 0 || ImIsPowerOfTwo(cond)); // Make sure the user doesn't attempt to combine multiple condition flags.
    window.SetWindowPosAllowFlags &= ~(ImGuiCond_Once | ImGuiCond_FirstUseEver | ImGuiCond_Appearing);
    window.SetWindowPosVal = ImVec2::new(f32::MAX, f32::MAX);

    // Set
    let old_pos: ImVec2 = window.Pos;
    window.Pos = ImFloor(pos);
    let offset: ImVec2 = window.Pos - old_pos;
    if (offset.x == 0.0 && offset.y == 0.0)
        return;
    MarkIniSettingsDirty(window);
    // FIXME: share code with TranslateWindow(), need to confirm whether the 3 rect modified by TranslateWindow() are desirable here.
    window.DC.CursorPos += offset;         // As we happen to move the window while it is being appended to (which is a bad idea - will smear) let's at least offset the cursor
    window.DC.CursorMaxPos += offset;      // And more importantly we need to offset CursorMaxPos/CursorStartPos this so ContentSize calculation doesn't get affected.
    window.DC.IdealMaxPos += offset;
    window.DC.CursorStartPos += offset;
}

c_void SetWindowPos(pos: &ImVec2, cond: ImGuiCond)
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindowRead();
    SetWindowPos(window, pos, cond);
}

c_void SetWindowPos(name: *const c_char, pos: &ImVec2, cond: ImGuiCond)
{
    if (let mut window: *mut ImGuiWindow =  FindWindowByName(name))
        SetWindowPos(window, pos, cond);
}

GetWindowSize: ImVec2()
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindowRead();
    return window.Size;
}

c_void SetWindowSize(window: *mut ImGuiWindow, size: &ImVec2, cond: ImGuiCond)
{
    // Test condition (NB: bit 0 is always true) and clear flags for next time
    if (cond && (window.SetWindowSizeAllowFlags & cond) == 0)
        return;

    // IM_ASSERT(cond == 0 || ImIsPowerOfTwo(cond)); // Make sure the user doesn't attempt to combine multiple condition flags.
    window.SetWindowSizeAllowFlags &= ~(ImGuiCond_Once | ImGuiCond_FirstUseEver | ImGuiCond_Appearing);

    // Set
    let old_size: ImVec2 = window.SizeFull;
    window.AutoFitFramesX = (size.x <= 0.0) ? 2 : 0;
    window.AutoFitFramesY = (size.y <= 0.0) ? 2 : 0;
    if (size.x <= 0.0)
        window.AutoFitOnlyGrows = false;
    else
        window.SizeFull.x = IM_FLOOR(size.x);
    if (size.y <= 0.0)
        window.AutoFitOnlyGrows = false;
    else
        window.SizeFull.y = IM_FLOOR(size.y);
    if (old_size.x != window.SizeFull.x || old_size.y != window.SizeFull.y)
        MarkIniSettingsDirty(window);
}

c_void SetWindowSize(size: &ImVec2, cond: ImGuiCond)
{
    SetWindowSize(GimGui.CurrentWindow, size, cond);
}

c_void SetWindowSize(name: *const c_char, size: &ImVec2, cond: ImGuiCond)
{
    if (let mut window: *mut ImGuiWindow =  FindWindowByName(name))
        SetWindowSize(window, size, cond);
}

c_void SetWindowCollapsed(window: *mut ImGuiWindow, collapsed: bool, cond: ImGuiCond)
{
    // Test condition (NB: bit 0 is always true) and clear flags for next time
    if (cond && (window.SetWindowCollapsedAllowFlags & cond) == 0)
        return;
    window.SetWindowCollapsedAllowFlags &= ~(ImGuiCond_Once | ImGuiCond_FirstUseEver | ImGuiCond_Appearing);

    // Set
    window.Collapsed = collapsed;
}

c_void SetWindowHitTestHole(window: *mut ImGuiWindow, pos: &ImVec2, size: &ImVec2)
{
    // IM_ASSERT(window.HitTestHoleSize.x == 0);     // We don't support multiple holes/hit test filters
    window.HitTestHoleSize = ImVec2ih(size);
    window.HitTestHoleOffset = ImVec2ih(pos - window.Pos);
}

c_void SetWindowCollapsed(collapsed: bool, cond: ImGuiCond)
{
    SetWindowCollapsed(GimGui.CurrentWindow, collapsed, cond);
}

IsWindowCollapsed: bool()
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindowRead();
    return window.Collapsed;
}

IsWindowAppearing: bool()
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindowRead();
    return window.Appearing;
}

c_void SetWindowCollapsed(name: *const c_char, collapsed: bool, cond: ImGuiCond)
{
    if (let mut window: *mut ImGuiWindow =  FindWindowByName(name))
        SetWindowCollapsed(window, collapsed, cond);
}

c_void SetWindowFocus()
{
    FocusWindow(GimGui.CurrentWindow);
}

c_void SetWindowFocus(name: *const c_char)
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

c_void SetNextWindowPos(pos: &ImVec2, cond: ImGuiCond, pivot: &ImVec2)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(cond == 0 || ImIsPowerOfTwo(cond)); // Make sure the user doesn't attempt to combine multiple condition flags.
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasPos;
    g.NextWindowData.PosVal = pos;
    g.NextWindowData.PosPivotVal = pivot;
    g.NextWindowData.PosCond = cond ? cond : ImGuiCond_Always;
    g.NextWindowData.PosUndock = true;
}



c_void SetNextWindowSizeConstraints(size_min: &ImVec2, size_max: &ImVec2, ImGuiSizeCallback custom_callback, custom_callback_user_data: *mut c_void)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasSizeConstraint;
    g.NextWindowData.SizeConstraintRect = ImRect(size_min, size_max);
    g.NextWindowData.SizeCallback = custom_callback;
    g.NextWindowData.SizeCallbackUserData = custom_callback_user_data;
}

// Content size = inner scrollable rectangle, padded with WindowPadding.
// SetNextWindowContentSize(ImVec2::new(100,100) + ImGuiWindowFlags_AlwaysAutoResize will always allow submitting a 100x100 item.
c_void SetNextWindowContentSize(size: &ImVec2)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasContentSize;
    g.NextWindowData.ContentSizeVal = ImFloor(size);
}

c_void SetNextWindowScroll(scroll: &ImVec2)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasScroll;
    g.NextWindowData.ScrollVal = scroll;
}

c_void SetNextWindowCollapsed(collapsed: bool, cond: ImGuiCond)
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

c_void SetNextWindowBgAlpha(alpha: c_float)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasBgAlpha;
    g.NextWindowData.BgAlphaVal = alpha;
}

c_void SetNextWindowViewport(id: ImGuiID)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasViewport;
    g.NextWindowData.ViewportId = id;
}

c_void SetNextWindowDockID(id: ImGuiID, cond: ImGuiCond)
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
}GetWindowDpiScale: c_float()
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

GetFont: *mut ImFont()
{
    return GimGui.Font;
}GetFontSize: c_float()
{
    return GimGui.FontSize;
}

GetFontTexUvWhitePixel: ImVec2()
{
    return GimGui.DrawListSharedData.TexUvWhitePixel;
}

c_void SetWindowFontScale(scale: c_float)
{
    // IM_ASSERT(scale > 0.0);
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    window.FontWindowScale = scale;
    g.FontSize = g.DrawListSharedData.FontSize = window.CalcFontSize();
}

c_void ActivateItem(id: ImGuiID)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NavNextActivateId = id;
    g.NavNextActivateFlags = ImGuiActivateFlags_None;
}

c_void PushFocusScope(id: ImGuiID)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    g.FocusScopeStack.push(window.DC.NavFocusScopeIdCurrent);
    window.DC.NavFocusScopeIdCurrent = id;
}

c_void PopFocusScope()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    // IM_ASSERT(g.FocusScopeStack.Size > 0); // Too many PopFocusScope() ?
    window.DC.NavFocusScopeIdCurrent = g.FocusScopeStack.last().unwrap();
    g.FocusScopeStack.pop_back();
}

// Note: this will likely be called ActivateItem() once we rework our Focus/Activation system!
c_void SetKeyboardFocusHere(offset: c_int)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    // IM_ASSERT(offset >= -1);    // -1 is allowed but not below
    IMGUI_DEBUG_LOG_ACTIVEID("SetKeyboardFocusHere(%d) in window \"%s\"\n", offset, window.Name);

    // It makes sense in the vast majority of cases to never interrupt a drag and drop.
    // When we refactor this function into ActivateItem() we may want to make this an option.
    // MovingWindow is protected from most user inputs using SetActiveIdUsingNavAndKeys(), but
    // is also automatically dropped in the event g.ActiveId is stolen.
    if (g.DragDropActive || g.MovingWindow != null_mut())
    {
        IMGUI_DEBUG_LOG_ACTIVEID("SetKeyboardFocusHere() ignored while DragDropActive!\n");
        return;
    }

    SetNavWindow(window);

    ImGuiScrollFlags scroll_flags = window.Appearing ? ImGuiScrollFlags_KeepVisibleEdgeX | ImGuiScrollFlags_AlwaysCenterY : ImGuiScrollFlags_KeepVisibleEdgeX | ImGuiScrollFlags_KeepVisibleEdgeY;
    NavMoveRequestSubmit(ImGuiDir_None, offset < 0 ? ImGuiDir_Up : ImGuiDir_Down, ImGuiNavMoveFlags_Tabbing | ImGuiNavMoveFlags_FocusApi, scroll_flags); // FIXME-NAV: Once we refactor tabbing, add LegacyApi flag to not activate non-inputable.
    if (offset == -1)
    {
        NavMoveRequestResolveWithLastItem(&g.NavMoveResultLocal);
    }
    else
    {
        g.NavTabbingDir = 1;
        g.NavTabbingCounter = offset + 1;
    }
}

c_void SetItemDefaultFocus()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if (!window.Appearing)
        return;
    if (g.NavWindow != window.RootWindowForNav || (!g.NavInitRequest && g.NavInitResultId == 0) || g.NavLayer != window.DC.NavLayerCurrent)
        return;

    g.NavInitRequest = false;
    g.NavInitResultId = g.LastItemData.ID;
    g.NavInitResultRectRel = WindowRectAbsToRel(window, g.LastItemData.Rect);
    NavUpdateAnyRequestFlag();

    // Scroll could be done in NavInitRequestApplyResult() via a opt-in flag (we however don't want regular init requests to scroll)
    if (!IsItemVisible())
        ScrollToRectEx(window, g.LastItemData.Rect, ImGuiScrollFlags_None);
}

c_void SetStateStorage(ImGuiStorage* tree)
{
    let mut window: *mut ImGuiWindow =  GimGui.CurrentWindow;
    window.DC.StateStorage = tree ? tree : &window.StateStorage;
}

ImGuiStorage* GetStateStorage()
{
    let mut window: *mut ImGuiWindow =  GimGui.CurrentWindow;
    return window.DC.StateStorage;
}

c_void PushID(str_id: *const c_char)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    let mut id: ImGuiID =  window.GetID(str_id);
    window.IDStack.push(id);
}

c_void PushID(str_id_begin: *const c_char, str_id_end: *const c_char)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    let mut id: ImGuiID =  window.GetID(str_id_begin, str_id_end);
    window.IDStack.push(id);
}

c_void PushID(*const c_void ptr_id)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    let mut id: ImGuiID =  window.GetID(ptr_id);
    window.IDStack.push(id);
}

c_void PushID(int_id: c_int)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    let mut id: ImGuiID =  window.GetID(int_id);
    window.IDStack.push(id);
}

// Push a given id value ignoring the ID stack as a seed.
c_void PushOverrideID(id: ImGuiID)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if (g.DebugHookIdInfo == id)
        DebugHookIdInfo(id, ImGuiDataType_ID, null_mut(), null_mut());
    window.IDStack.push(id);
}

// Helper to avoid a common series of PushOverrideID -> GetID() -> PopID() call
// (note that when using this pattern, TestEngine's "Stack Tool" will tend to not display the intermediate stack level.
//  for that to work we would need to do PushOverrideID() -> ItemAdd() -> PopID() which would alter widget code a little more)
ImGuiID GetIDWithSeed(str: *const c_char, str_end: *const c_char, ImGuiID seed)
{
    let mut id: ImGuiID =  ImHashStr(str, str_end ? (str_end - str) : 0, seed);
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.DebugHookIdInfo == id)
        DebugHookIdInfo(id, ImGuiDataType_String, str, str_end);
    return id;
}

c_void PopID()
{
    let mut window: *mut ImGuiWindow =  GimGui.CurrentWindow;
    // IM_ASSERT(window.IDStack.Size > 1); // Too many PopID(), or could be popping in a wrong/different window?
    window.IDStack.pop_back();
}

ImGuiID GetID(str_id: *const c_char)
{
    let mut window: *mut ImGuiWindow =  GimGui.CurrentWindow;
    return window.GetID(str_id);
}

ImGuiID GetID(str_id_begin: *const c_char, str_id_end: *const c_char)
{
    let mut window: *mut ImGuiWindow =  GimGui.CurrentWindow;
    return window.GetID(str_id_begin, str_id_end);
}

ImGuiID GetID(*const c_void ptr_id)
{
    let mut window: *mut ImGuiWindow =  GimGui.CurrentWindow;
    return window.GetID(ptr_id);
}

IsRectVisible: bool(size: &ImVec2)
{
    let mut window: *mut ImGuiWindow =  GimGui.CurrentWindow;
    return window.ClipRect.Overlaps(ImRect(window.DC.CursorPos, window.DC.CursorPos + size));
}

IsRectVisible: bool(rect_min: &ImVec2, rect_max: &ImVec2)
{
    let mut window: *mut ImGuiWindow =  GimGui.CurrentWindow;
    return window.ClipRect.Overlaps(ImRect(rect_min, rect_max));
}


//-----------------------------------------------------------------------------
// [SECTION] INPUTS
//-----------------------------------------------------------------------------




//-----------------------------------------------------------------------------
// [SECTION] ERROR CHECKING
//-----------------------------------------------------------------------------

// Helper function to verify ABI compatibility between caller code and compiled version of Dear ImGui.
// Verify that the type sizes are matching between the calling file's compilation unit and imgui.cpp's compilation unit
// If this triggers you have an issue:
// - Most commonly: mismatched headers and compiled code version.
// - Or: mismatched configuration #define, compilation settings, packing pragma etc.
//   The configuration settings mentioned in imconfig.h must be set for all compilation units involved with Dear ImGui,
//   which is way it is required you put them in your imconfig file (and not just before including imgui.h).
//   Otherwise it is possible that different compilation units would see different structure layout
DebugCheckVersionAndDataLayout: bool(version: *const c_char, sz_io: size_t, sz_style: size_t, sz_vec2: size_t, sz_vec4: size_t, sz_vert: size_t, sz_idx: size_t)
{
    let mut error: bool =  false;
    if (strcmp(version, IMGUI_VERSION) != 0) { error = true; IM_ASSERT(strcmp(version, IMGUI_VERSION) == 0 && "Mismatched version string!"); }
    if (sz_io != sizeof(ImGuiIO)) { error = true; IM_ASSERT(sz_io == sizeof(ImGuiIO) && "Mismatched struct layout!"); }
    if (sz_style != sizeof(ImGuiStyle)) { error = true; IM_ASSERT(sz_style == sizeof(ImGuiStyle) && "Mismatched struct layout!"); }
    if (sz_vec2 != sizeof(ImVec2)) { error = true; IM_ASSERT(sz_vec2 == sizeof(ImVec2) && "Mismatched struct layout!"); }
    if (sz_vec4 != sizeof(ImVec4)) { error = true; IM_ASSERT(sz_vec4 == sizeof(ImVec4) && "Mismatched struct layout!"); }
    if (sz_vert != sizeof(ImDrawVert)) { error = true; IM_ASSERT(sz_vert == sizeof(ImDrawVert) && "Mismatched struct layout!"); }
    if (sz_idx != sizeof) { error = true; IM_ASSERT(sz_idx == sizeof && "Mismatched struct layout!"); }
    return !error;
}



static c_void ErrorCheckNewFrameSanityChecks()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    // Check user IM_ASSERT macro
    // (IF YOU GET A WARNING OR COMPILE ERROR HERE: it means your assert macro is incorrectly defined!
    //  If your macro uses multiple statements, it NEEDS to be surrounded by a 'do { ... } while (0)' block.
    //  This is a common C/C++ idiom to allow multiple statements macros to be used in control flow blocks.)
    // #define IM_ASSERT(EXPR)   if (SomeCode(EXPR)) SomeMoreCode();                    // Wrong!
    // #define IM_ASSERT(EXPR)   do { if (SomeCode(EXPR)) SomeMoreCode(); } while (0)   // Correct!
    if (true) IM_ASSERT(1); else IM_ASSERT(0);

    // Check user data
    // (We pass an error message in the assert expression to make it visible to programmers who are not using a debugger, as most assert handlers display their argument)
    // IM_ASSERT(g.Initialized);
    // IM_ASSERT((g.IO.DeltaTime > 0.0 || g.FrameCount == 0)              && "Need a positive DeltaTime!");
    // IM_ASSERT((g.FrameCount == 0 || g.FrameCountEnded == g.FrameCount)  && "Forgot to call Render() or EndFrame() at the end of the previous frame?");
    // IM_ASSERT(g.IO.DisplaySize.x >= 0.0 && g.IO.DisplaySize.y >= 0.0  && "Invalid DisplaySize value!");
    // IM_ASSERT(g.IO.Fonts.IsBuilt()                                     && "Font Atlas not built! Make sure you called ImGui_ImplXXXX_NewFrame() function for renderer backend, which should call io.Fonts.GetTexDataAsRGBA32() / GetTexDataAsAlpha8()");
    // IM_ASSERT(g.Style.CurveTessellationTol > 0.0                       && "Invalid style setting!");
    // IM_ASSERT(g.Style.CircleTessellationMaxError > 0.0                 && "Invalid style setting!");
    // IM_ASSERT(g.Style.Alpha >= 0.0 && g.Style.Alpha <= 1.0            && "Invalid style setting!"); // Allows us to avoid a few clamps in color computations
    // IM_ASSERT(g.Style.WindowMinSize.x >= 1.0 && g.Style.WindowMinSize.y >= 1.0 && "Invalid style setting.");
    // IM_ASSERT(g.Style.WindowMenuButtonPosition == ImGuiDir_None || g.Style.WindowMenuButtonPosition == ImGuiDir_Left || g.Style.WindowMenuButtonPosition == ImGuiDir_Right);
    // IM_ASSERT(g.Style.ColorButtonPosition == ImGuiDir_Left || g.Style.ColorButtonPosition == ImGuiDir_Right);
// #ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
    for (let n: c_int = ImGuiKey_NamedKey_BEGIN; n < ImGuiKey_COUNT; n++)
        // IM_ASSERT(g.IO.KeyMap[n] >= -1 && g.IO.KeyMap[n] < ImGuiKey_LegacyNativeKey_END && "io.KeyMap[] contains an out of bound value (need to be 0..511, or -1 for unmapped key)");

    // Check: required key mapping (we intentionally do NOT check all keys to not pressure user into setting up everything, but Space is required and was only added in 1.60 WIP)
    if ((g.IO.ConfigFlags & ImGuiConfigFlags_NavEnableKeyboard) && g.IO.BackendUsingLegacyKeyArrays == 1)
        // IM_ASSERT(g.IO.KeyMap[ImGuiKey_Space] != -1 && "ImGuiKey_Space is not mapped, required for keyboard navigation.");
// #endif

    // Check: the io.ConfigWindowsResizeFromEdges option requires backend to honor mouse cursor changes and set the ImGuiBackendFlags_HasMouseCursors flag accordingly.
    if (g.IO.ConfigWindowsResizeFromEdges && !(g.IO.BackendFlags & ImGuiBackendFlags_HasMouseCursors))
        g.IO.ConfigWindowsResizeFromEdges = false;

    // Perform simple check: error if Docking or Viewport are enabled _exactly_ on frame 1 (instead of frame 0 or later), which is a common error leading to loss of .ini data.
    if (g.FrameCount == 1 && (g.IO.ConfigFlags & ImGuiConfigFlags_DockingEnable) && (g.ConfigFlagsLastFrame & ImGuiConfigFlags_DockingEnable) == 0)
        // IM_ASSERT(0 && "Please set DockingEnable before the first call to NewFrame()! Otherwise you will lose your .ini settings!");
    if (g.FrameCount == 1 && (g.IO.ConfigFlags & ImGuiConfigFlags_ViewportsEnable) && (g.ConfigFlagsLastFrame & ImGuiConfigFlags_ViewportsEnable) == 0)
        // IM_ASSERT(0 && "Please set ViewportsEnable before the first call to NewFrame()! Otherwise you will lose your .ini settings!");

    // Perform simple checks: multi-viewport and platform windows support
    if (g.IO.ConfigFlags & ImGuiConfigFlags_ViewportsEnable)
    {
        if ((g.IO.BackendFlags & ImGuiBackendFlags_PlatformHasViewports) && (g.IO.BackendFlags & ImGuiBackendFlags_RendererHasViewports))
        {
            // IM_ASSERT((g.FrameCount == 0 || g.FrameCount == g.FrameCountPlatformEnded) && "Forgot to call UpdatePlatformWindows() in main loop after EndFrame()? Check examples/ applications for reference.");
            // IM_ASSERT(g.PlatformIO.Platform_CreateWindow  != NULL && "Platform init didn't install handlers?");
            // IM_ASSERT(g.PlatformIO.Platform_DestroyWindow != NULL && "Platform init didn't install handlers?");
            // IM_ASSERT(g.PlatformIO.Platform_GetWindowPos  != NULL && "Platform init didn't install handlers?");
            // IM_ASSERT(g.PlatformIO.Platform_SetWindowPos  != NULL && "Platform init didn't install handlers?");
            // IM_ASSERT(g.PlatformIO.Platform_GetWindowSize != NULL && "Platform init didn't install handlers?");
            // IM_ASSERT(g.PlatformIO.Platform_SetWindowSize != NULL && "Platform init didn't install handlers?");
            // IM_ASSERT(g.PlatformIO.Monitors.Size > 0 && "Platform init didn't setup Monitors list?");
            // IM_ASSERT((g.Viewports[0]->PlatformUserData != NULL || g.Viewports[0]->PlatformHandle != NULL) && "Platform init didn't setup main viewport.");
            if (g.IO.ConfigDockingTransparentPayload && (g.IO.ConfigFlags & ImGuiConfigFlags_DockingEnable))
                // IM_ASSERT(g.PlatformIO.Platform_SetWindowAlpha != NULL && "Platform_SetWindowAlpha handler is required to use io.ConfigDockingTransparent!");
        }
        else
        {
            // Disable feature, our backends do not support it
            g.IO.ConfigFlags &= !ImGuiConfigFlags_ViewportsEnable;
        }

        // Perform simple checks on platform monitor data + compute a total bounding box for quick early outs
        for (let monitor_n: c_int = 0; monitor_n < g.PlatformIO.Monitors.Size; monitor_n++)
        {
            ImGuiPlatformMonitor& mon = g.PlatformIO.Monitors[monitor_n];
            IM_UNUSED(mon);
            // IM_ASSERT(mon.MainSize.x > 0.0 && mon.MainSize.y > 0.0 && "Monitor main bounds not setup properly.");
            // IM_ASSERT(ImRect(mon.MainPos, mon.MainPos + mon.MainSize).Contains(ImRect(mon.WorkPos, mon.WorkPos + mon.WorkSize)) && "Monitor work bounds not setup properly. If you don't have work area information, just copy MainPos/MainSize into them.");
            // IM_ASSERT(mon.DpiScale != 0.0);
        }
    }
}

static c_void ErrorCheckEndFrameSanityChecks()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    // Verify that io.KeyXXX fields haven't been tampered with. Key mods should not be modified between NewFrame() and EndFrame()
    // One possible reason leading to this assert is that your backends update inputs _AFTER_ NewFrame().
    // It is known that when some modal native windows called mid-frame takes focus away, some backends such as GLFW will
    // send key release events mid-frame. This would normally trigger this assertion and lead to sheared inputs.
    // We silently accommodate for this case by ignoring/ the case where all io.KeyXXX modifiers were released (aka key_mod_flags == 0),
    // while still correctly asserting on mid-frame key press events.
    const ImGuiModFlags key_mods = GetMergedModFlags();
    // IM_ASSERT((key_mods == 0 || g.IO.KeyMods == key_mods) && "Mismatching io.KeyCtrl/io.KeyShift/io.KeyAlt/io.KeySuper vs io.KeyMods");
    IM_UNUSED(key_mods);

    // [EXPERIMENTAL] Recover from errors: You may call this yourself before EndFrame().
    //ErrorCheckEndFrameRecover();

    // Report when there is a mismatch of Begin/BeginChild vs End/EndChild calls. Important: Remember that the Begin/BeginChild API requires you
    // to always call End/EndChild even if Begin/BeginChild returns false! (this is unfortunately inconsistent with most other Begin* API).
    if (g.CurrentWindowStack.Size != 1)
    {
        if (g.CurrentWindowStack.Size > 1)
        {
            // IM_ASSERT_USER_ERROR(g.CurrentWindowStack.Size == 1, "Mismatched Begin/BeginChild vs End/EndChild calls: did you forget to call End/EndChild?");
            while (g.CurrentWindowStack.Size > 1)
                End();
        }
        else
        {
            // IM_ASSERT_USER_ERROR(g.CurrentWindowStack.Size == 1, "Mismatched Begin/BeginChild vs End/EndChild calls: did you call End/EndChild too much?");
        }
    }

    // IM_ASSERT_USER_ERROR(g.GroupStack.Size == 0, "Missing EndGroup call!");
}

// Experimental recovery from incorrect usage of BeginXXX/EndXXX/PushXXX/PopXXX calls.
// Must be called during or before EndFrame().
// This is generally flawed as we are not necessarily End/Popping things in the right order.
// FIXME: Can't recover from inside BeginTabItem/EndTabItem yet.
// FIXME: Can't recover from interleaved BeginTabBar/Begin
c_void    ErrorCheckEndFrameRecover(ImGuiErrorLogCallback log_callback, user_data: *mut c_void)
{
    // PVS-Studio V1044 is "Loop break conditions do not depend on the number of iterations"
    let g = GImGui; // ImGuiContext& g = *GImGui;
    while (g.CurrentWindowStack.Size > 0) //-V1044
    {
        ErrorCheckEndWindowRecover(log_callback, user_data);
        let mut window = g.CurrentWindow;
        if (g.CurrentWindowStack.Size == 1)
        {
            // IM_ASSERT(window.IsFallbackWindow);
            break;
        }
        if (window.Flags & ImGuiWindowFlags_ChildWindow)
        {
            if (log_callback) log_callback(user_data, "Recovered from missing EndChild() for '%s'", window.Name);
            EndChild();
        }
        else
        {
            if (log_callback) log_callback(user_data, "Recovered from missing End() for '%s'", window.Name);
            End();
        }
    }
}

// Must be called before End()/EndChild()
c_void    ErrorCheckEndWindowRecover(ImGuiErrorLogCallback log_callback, user_data: *mut c_void)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    while (g.CurrentTable && (g.Currenttable.OuterWindow == g.CurrentWindow || g.Currenttable.InnerWindow == g.CurrentWindow))
    {
        if (log_callback) log_callback(user_data, "Recovered from missing EndTable() in '%s'", g.Currenttable.Outerwindow.Name);
        EndTable();
    }

    let mut window = g.CurrentWindow;
    ImGuiStackSizes* stack_sizes = &g.CurrentWindowStack.last().unwrap().StackSizesOnBegin;
    // IM_ASSERT(window != NULL);
    while (g.CurrentTabBar != null_mut()) //-V1044
    {
        if (log_callback) log_callback(user_data, "Recovered from missing EndTabBar() in '%s'", window.Name);
        EndTabBar();
    }
    while (window.DC.TreeDepth > 0)
    {
        if (log_callback) log_callback(user_data, "Recovered from missing TreePop() in '%s'", window.Name);
        TreePop();
    }
    while (g.GroupStack.Size > stack_sizes->SizeOfGroupStack) //-V1044
    {
        if (log_callback) log_callback(user_data, "Recovered from missing EndGroup() in '%s'", window.Name);
        EndGroup();
    }
    while (window.IDStack.Size > 1)
    {
        if (log_callback) log_callback(user_data, "Recovered from missing PopID() in '%s'", window.Name);
        PopID();
    }
    while (g.DisabledStackSize > stack_sizes->SizeOfDisabledStack) //-V1044
    {
        if (log_callback) log_callback(user_data, "Recovered from missing EndDisabled() in '%s'", window.Name);
        EndDisabled();
    }
    while (g.ColorStack.Size > stack_sizes->SizeOfColorStack)
    {
        if (log_callback) log_callback(user_data, "Recovered from missing PopStyleColor() in '%s' for ImGuiCol_%s", window.Name, GetStyleColorName(g.ColorStack.last().unwrap().Col));
        PopStyleColor();
    }
    while (g.ItemFlagsStack.Size > stack_sizes->SizeOfItemFlagsStack) //-V1044
    {
        if (log_callback) log_callback(user_data, "Recovered from missing PopItemFlag() in '%s'", window.Name);
        PopItemFlag();
    }
    while (g.StyleVarStack.Size > stack_sizes->SizeOfStyleVarStack) //-V1044
    {
        if (log_callback) log_callback(user_data, "Recovered from missing PopStyleVar() in '%s'", window.Name);
        PopStyleVar();
    }
    while (g.FocusScopeStack.Size > stack_sizes->SizeOfFocusScopeStack) //-V1044
    {
        if (log_callback) log_callback(user_data, "Recovered from missing PopFocusScope() in '%s'", window.Name);
        PopFocusScope();
    }
}

// Save current stack sizes for later compare
c_void ImGuiStackSizes::SetToCurrentState()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    SizeOfIDStack = window.IDStack.Size;
    SizeOfColorStack = g.ColorStack.Size;
    SizeOfStyleVarStack = g.StyleVarStack.Size;
    SizeOfFontStack = g.FontStack.Size;
    SizeOfFocusScopeStack = g.FocusScopeStack.Size;
    SizeOfGroupStack = g.GroupStack.Size;
    SizeOfItemFlagsStack = g.ItemFlagsStack.Size;
    SizeOfBeginPopupStack = g.BeginPopupStack.Size;
    SizeOfDisabledStack = g.DisabledStackSize;
}

// Compare to detect usage errors
c_void ImGuiStackSizes::CompareWithCurrentState()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    IM_UNUSED(window);

    // Window stacks
    // NOT checking: DC.ItemWidth, DC.TextWrapPos (per window) to allow user to conveniently push once and not pop (they are cleared on Begin)
    // IM_ASSERT(SizeOfIDStack         == window.IDStack.Size     && "PushID/PopID or TreeNode/TreePop Mismatch!");

    // Global stacks
    // For color, style and font stacks there is an incentive to use Push/Begin/Pop/.../End patterns, so we relax our checks a little to allow them.
    // IM_ASSERT(SizeOfGroupStack      == g.GroupStack.Size        && "BeginGroup/EndGroup Mismatch!");
    // IM_ASSERT(SizeOfBeginPopupStack == g.BeginPopupStack.Size   && "BeginPopup/EndPopup or BeginMenu/EndMenu Mismatch!");
    // IM_ASSERT(SizeOfDisabledStack   == g.DisabledStackSize      && "BeginDisabled/EndDisabled Mismatch!");
    // IM_ASSERT(SizeOfItemFlagsStack  >= g.ItemFlagsStack.Size    && "PushItemFlag/PopItemFlag Mismatch!");
    // IM_ASSERT(SizeOfColorStack      >= g.ColorStack.Size        && "PushStyleColor/PopStyleColor Mismatch!");
    // IM_ASSERT(SizeOfStyleVarStack   >= g.StyleVarStack.Size     && "PushStyleVar/PopStyleVar Mismatch!");
    // IM_ASSERT(SizeOfFontStack       >= g.FontStack.Size         && "PushFont/PopFont Mismatch!");
    // IM_ASSERT(SizeOfFocusScopeStack == g.FocusScopeStack.Size   && "PushFocusScope/PopFocusScope Mismatch!");
}


//-----------------------------------------------------------------------------
// [SECTION] LAYOUT
//-----------------------------------------------------------------------------
// - ItemSize()
// - ItemAdd()
// - SameLine()
// - GetCursorScreenPos()
// - SetCursorScreenPos()
// - GetCursorPos(), GetCursorPosX(), GetCursorPosY()
// - SetCursorPos(), SetCursorPosX(), SetCursorPosY()
// - GetCursorStartPos()
// - Indent()
// - Unindent()
// - SetNextItemWidth()
// - PushItemWidth()
// - PushMultiItemsWidths()
// - PopItemWidth()
// - CalcItemWidth()
// - CalcItemSize()
// - GetTextLineHeight()
// - GetTextLineHeightWithSpacing()
// - GetFrameHeight()
// - GetFrameHeightWithSpacing()
// - GetContentRegionMax()
// - GetContentRegionMaxAbs() [Internal]
// - GetContentRegionAvail(),
// - GetWindowContentRegionMin(), GetWindowContentRegionMax()
// - BeginGroup()
// - EndGroup()
// Also see in imgui_widgets: tab bars, and in imgui_tables: tables, columns.
//-----------------------------------------------------------------------------

GetCursorScreenPos: ImVec2()
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindowRead();
    return window.DC.CursorPos;
}

// 2022/08/05: Setting cursor position also extend boundaries (via modifying CursorMaxPos) used to compute window size, group size etc.
// I believe this was is a judicious choice but it's probably being relied upon (it has been the case since 1.31 and 1.50)
// It would be sane if we requested user to use SetCursorPos() + Dummy(ImVec2::new(0,0)) to extend CursorMaxPos...
c_void SetCursorScreenPos(pos: &ImVec2)
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    window.DC.CursorPos = pos;
    //window.DC.CursorMaxPos = ImMax(window.DC.CursorMaxPos, window.DC.CursorPos);
    window.DC.IsSetPos = true;
}

// User generally sees positions in window coordinates. Internally we store CursorPos in absolute screen coordinates because it is more convenient.
// Conversion happens as we pass the value to user, but it makes our naming convention confusing because GetCursorPos() == (DC.CursorPos - window.Pos). May want to rename 'DC.CursorPos'.
GetCursorPos: ImVec2()
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindowRead();
    return window.DC.CursorPos - window.Pos + window.Scroll;
}GetCursorPosX: c_float()
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindowRead();
    return window.DC.CursorPos.x - window.Pos.x + window.Scroll.x;
}GetCursorPosY: c_float()
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindowRead();
    return window.DC.CursorPos.y - window.Pos.y + window.Scroll.y;
}

c_void SetCursorPos(local_pos: &ImVec2)
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    window.DC.CursorPos = window.Pos - window.Scroll + local_pos;
    //window.DC.CursorMaxPos = ImMax(window.DC.CursorMaxPos, window.DC.CursorPos);
    window.DC.IsSetPos = true;
}

c_void SetCursorPosX(x: c_float)
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    window.DC.CursorPos.x = window.Pos.x - window.Scroll.x + x;
    //window.DC.CursorMaxPos.x = ImMax(window.DC.CursorMaxPos.x, window.DC.CursorPos.x);
    window.DC.IsSetPos = true;
}

c_void SetCursorPosY(y: c_float)
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    window.DC.CursorPos.y = window.Pos.y - window.Scroll.y + y;
    //window.DC.CursorMaxPos.y = ImMax(window.DC.CursorMaxPos.y, window.DC.CursorPos.y);
    window.DC.IsSetPos = true;
}

GetCursorStartPos: ImVec2()
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindowRead();
    return window.DC.CursorStartPos - window.Pos;
}

c_void Indent(indent_w: c_float)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    window.DC.Indent.x += (indent_w != 0.0) ? indent_w : g.Style.IndentSpacing;
    window.DC.CursorPos.x = window.Pos.x + window.DC.Indent.x + window.DC.ColumnsOffset.x;
}

c_void Unindent(indent_w: c_float)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    window.DC.Indent.x -= (indent_w != 0.0) ? indent_w : g.Style.IndentSpacing;
    window.DC.CursorPos.x = window.Pos.x + window.DC.Indent.x + window.DC.ColumnsOffset.x;
}

// Affect large frame+labels widgets only.
c_void SetNextItemWidth(item_width: c_float)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NextItemData.Flags |= ImGuiNextItemDataFlags_HasWidth;
    g.NextItemData.Width = item_width;
}

// FIXME: Remove the == 0.0 behavior?
c_void PushItemWidth(item_width: c_float)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    window.DC.ItemWidthStack.push(window.DC.ItemWidth); // Backup current width
    window.DC.ItemWidth = (item_width == 0.0 ? window.ItemWidthDefault : item_width);
    g.NextItemData.Flags &= !ImGuiNextItemDataFlags_HasWidth;
}

c_void PushMultiItemsWidths(components: c_int,w_full: c_float)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    let setyle = &mut g.Style;
    w_item_one: c_float  = ImMax(1.0, IM_FLOOR((w_full - (style.ItemInnerSpacing.x) * (components - 1)) / components));
    let w_item_last: c_float =  ImMax(1.0, IM_FLOOR(w_full - (w_item_one + style.ItemInnerSpacing.x) * (components - 1)));
    window.DC.ItemWidthStack.push(window.DC.ItemWidth); // Backup current width
    window.DC.ItemWidthStack.push(w_item_last);
    for (let i: c_int = 0; i < components - 2; i++)
        window.DC.ItemWidthStack.push(w_item_one);
    window.DC.ItemWidth = (components == 1) ? w_item_last : w_item_one;
    g.NextItemData.Flags &= !ImGuiNextItemDataFlags_HasWidth;
}

c_void PopItemWidth()
{
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    window.DC.ItemWidth = window.DC.ItemWidthStack.last().unwrap();
    window.DC.ItemWidthStack.pop_back();
}

// Calculate default item width given value passed to PushItemWidth() or SetNextItemWidth().
// The SetNextItemWidth() data is generally cleared/consumed by ItemAdd() or NextItemData.ClearFlags()CalcItemWidth: c_float()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    let mut w: c_float = 0.0;
    if (g.NextItemData.Flags & ImGuiNextItemDataFlags_HasWidth)
        w = g.NextItemData.Width;
    else
        w = window.DC.ItemWidth;
    if (w < 0.0)
    {
        let region_max_x: c_float =  GetContentRegionMaxAbs().x;
        w = ImMax(1.0, region_max_x - window.DC.CursorPos.x + w);
    }
    w = IM_FLOOR(w);
    return w;
}

// [Internal] Calculate full item size given user provided 'size' parameter and default width/height. Default width is often == CalcItemWidth().
// Those two functions CalcItemWidth vs CalcItemSize are awkwardly named because they are not fully symmetrical.
// Note that only CalcItemWidth() is publicly exposed.
// The 4.0.0 here may be changed to match CalcItemWidth() and/or BeginChild() (right now we have a mismatch which is harmless but undesirable)
CalcItemSize: ImVec2(size: ImVec2,default_w: c_float,default_h: c_float)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;

    region_max: ImVec2;
    if (size.x < 0.0 || size.y < 0.0)
        region_max = GetContentRegionMaxAbs();

    if (size.x == 0.0)
        size.x = default_w;
    else if (size.x < 0.0)
        size.x = ImMax(4.0.0, region_max.x - window.DC.CursorPos.x + size.x);

    if (size.y == 0.0)
        size.y = default_h;
    else if (size.y < 0.0)
        size.y = ImMax(4.0.0, region_max.y - window.DC.CursorPos.y + size.y);

    return size;
}GetTextLineHeight: c_float()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.FontSize;
}GetTextLineHeightWithSpacing: c_float()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.FontSize + g.Style.ItemSpacing.y;
}GetFrameHeight: c_float()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.FontSize + g.Style.FramePadding.y * 2.0.0;
}GetFrameHeightWithSpacing: c_float()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.FontSize + g.Style.FramePadding.y * 2.0.0 + g.Style.ItemSpacing.y;
}

// FIXME: All the Contents Region function are messy or misleading. WE WILL AIM TO OBSOLETE ALL OF THEM WITH A NEW "WORK RECT" API. Thanks for your patience!

// FIXME: This is in window space (not screen space!).
GetContentRegionMax: ImVec2()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    let mx: ImVec2 = window.ContentRegionRect.Max - window.Pos;
    if (window.DC.CurrentColumns || g.CurrentTable)
        mx.x = window.WorkRect.Max.x - window.Pos.x;
    return mx;
}





// In window space (not screen space!)
GetWindowContentRegionMin: ImVec2()
{
    let mut window: *mut ImGuiWindow =  GimGui.CurrentWindow;
    return window.ContentRegionRect.Min - window.Pos;
}

GetWindowContentRegionMax: ImVec2()
{
    let mut window: *mut ImGuiWindow =  GimGui.CurrentWindow;
    return window.ContentRegionRect.Max - window.Pos;
}

// Lock horizontal starting position + capture group bounding box into one "item" (so you can use IsItemHovered() or layout primitives such as SameLine() on whole group, etc.)
// Groups are currently a mishmash of functionalities which should perhaps be clarified and separated.
// FIXME-OPT: Could we safely early out on ->SkipItems?
c_void BeginGroup()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;

    g.GroupStack.resize(g.GroupStack.Size + 1);
    ImGuiGroupData& group_data = g.GroupStack.last().unwrap();
    group_data.WindowID = window.ID;
    group_data.BackupCursorPos = window.DC.CursorPos;
    group_data.BackupCursorMaxPos = window.DC.CursorMaxPos;
    group_data.BackupIndent = window.DC.Indent;
    group_data.BackupGroupOffset = window.DC.GroupOffset;
    group_data.BackupCurrLineSize = window.DC.CurrLineSize;
    group_data.BackupCurrLineTextBaseOffset = window.DC.CurrLineTextBaseOffset;
    group_data.BackupActiveIdIsAlive = g.ActiveIdIsAlive;
    group_data.BackupHoveredIdIsAlive = g.HoveredId != 0;
    group_data.BackupActiveIdPreviousFrameIsAlive = g.ActiveIdPreviousFrameIsAlive;
    group_data.EmitItem = true;

    window.DC.GroupOffset.x = window.DC.CursorPos.x - window.Pos.x - window.DC.ColumnsOffset.x;
    window.DC.Indent = window.DC.GroupOffset;
    window.DC.CursorMaxPos = window.DC.CursorPos;
    window.DC.CurrLineSize = ImVec2::new(0.0, 0.0);
    if (g.LogEnabled)
        g.LogLinePosY = -f32::MAX; // To enforce a carriage return
}

c_void EndGroup()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    // IM_ASSERT(g.GroupStack.Size > 0); // Mismatched BeginGroup()/EndGroup() calls

    ImGuiGroupData& group_data = g.GroupStack.last().unwrap();
    // IM_ASSERT(group_data.WindowID == window.ID); // EndGroup() in wrong window?

    if (window.DC.IsSetPos)
        ErrorCheckUsingSetCursorPosToExtendParentBoundaries();

    let mut group_bb: ImRect = ImRect::new(group_data.BackupCursorPos, ImMax(window.DC.CursorMaxPos, group_data.BackupCursorPos));

    window.DC.CursorPos = group_data.BackupCursorPos;
    window.DC.CursorMaxPos = ImMax(group_data.BackupCursorMaxPos, window.DC.CursorMaxPos);
    window.DC.Indent = group_data.BackupIndent;
    window.DC.GroupOffset = group_data.BackupGroupOffset;
    window.DC.CurrLineSize = group_data.BackupCurrLineSize;
    window.DC.CurrLineTextBaseOffset = group_data.BackupCurrLineTextBaseOffset;
    if (g.LogEnabled)
        g.LogLinePosY = -f32::MAX; // To enforce a carriage return

    if (!group_data.EmitItem)
    {
        g.GroupStack.pop_back();
        return;
    }

    window.DC.CurrLineTextBaseOffset = ImMax(window.DC.PrevLineTextBaseOffset, group_data.BackupCurrLineTextBaseOffset);      // FIXME: Incorrect, we should grab the base offset from the *first line* of the group but it is hard to obtain now.
    ItemSize(group_bb.GetSize());
    ItemAdd(group_bb, 0, null_mut(), ImGuiItemFlags_NoTabStop);

    // If the current ActiveId was declared within the boundary of our group, we copy it to LastItemId so IsItemActive(), IsItemDeactivated() etc. will be functional on the entire group.
    // It would be be neater if we replaced window.DC.LastItemId by e.g. 'LastItemIsActive: bool', but would put a little more burden on individual widgets.
    // Also if you grep for LastItemId you'll notice it is only used in that context.
    // (The two tests not the same because ActiveIdIsAlive is an ID itself, in order to be able to handle ActiveId being overwritten during the frame.)
    let group_contains_curr_active_id: bool = (group_data.BackupActiveIdIsAlive != g.ActiveId) && (g.ActiveIdIsAlive == g.ActiveId) && g.ActiveId;
    let group_contains_prev_active_id: bool = (group_data.BackupActiveIdPreviousFrameIsAlive == false) && (g.ActiveIdPreviousFrameIsAlive == true);
    if (group_contains_curr_active_id)
        g.LastItemData.ID = g.ActiveId;
    else if (group_contains_prev_active_id)
        g.LastItemData.ID = g.ActiveIdPreviousFrame;
    g.LastItemData.Rect = group_bb;

    // Forward Hovered flag
    let group_contains_curr_hovered_id: bool = (group_data.BackupHoveredIdIsAlive == false) && g.HoveredId != 0;
    if (group_contains_curr_hovered_id)
        g.LastItemData.StatusFlags |= ImGuiItemStatusFlags_HoveredWindow;

    // Forward Edited flag
    if (group_contains_curr_active_id && g.ActiveIdHasBeenEditedThisFrame)
        g.LastItemData.StatusFlags |= ImGuiItemStatusFlags_Edited;

    // Forward Deactivated flag
    g.LastItemData.StatusFlags |= ImGuiItemStatusFlags_HasDeactivated;
    if (group_contains_prev_active_id && g.ActiveId != g.ActiveIdPreviousFrame)
        g.LastItemData.StatusFlags |= ImGuiItemStatusFlags_Deactivated;

    g.GroupStack.pop_back();
    //window.DrawList.AddRect(group_bb.Min, group_bb.Max, IM_COL32(255,0,255,255));   // [Debug]
}


//-----------------------------------------------------------------------------
// [SECTION] SCROLLING
//-----------------------------------------------------------------------------

// Helper to snap on edges when aiming at an item very close to the edge,
// So the difference between WindowPadding and ItemSpacing will be in the visible area after scrolling.
// When we refactor the scrolling API this may be configurable with a flag?
// Note that the effect for this won't be visible on X axis with default Style settings as WindowPadding.x == ItemSpacing.x by default.
staticCalcScrollEdgeSnap: c_float(target: c_float,snap_min: c_float,snap_max: c_float,snap_threshold: c_float,center_ratio: c_float)
{
    if (target <= snap_min + snap_threshold)
        return ImLerp(snap_min, target, center_ratio);
    if (target >= snap_max - snap_threshold)
        return ImLerp(target, snap_max, center_ratio);
    return target;
}

static CalcNextScrollFromScrollTargetAndClamp: ImVec2(window: *mut ImGuiWindow)
{
    let scroll: ImVec2 = window.Scroll;
    if (window.ScrollTarget.x < f32::MAX)
    {
        let decoration_total_width: c_float =  window.ScrollbarSizes.x;
        let center_x_ratio: c_float =  window.ScrollTargetCenterRatio.x;
        let scroll_target_x: c_float =  window.ScrollTarget.x;
        if (window.ScrollTargetEdgeSnapDist.x > 0.0)
        {
            let snap_x_min: c_float =  0.0;
            let snap_x_max: c_float =  window.ScrollMax.x + window.SizeFull.x - decoration_total_width;
            scroll_target_x = CalcScrollEdgeSnap(scroll_target_x, snap_x_min, snap_x_max, window.ScrollTargetEdgeSnapDist.x, center_x_ratio);
        }
        scroll.x = scroll_target_x - center_x_ratio * (window.SizeFull.x - decoration_total_width);
    }
    if (window.ScrollTarget.y < f32::MAX)
    {
        let decoration_total_height: c_float =  window.TitleBarHeight() + window.MenuBarHeight() + window.ScrollbarSizes.y;
        let center_y_ratio: c_float =  window.ScrollTargetCenterRatio.y;
        let scroll_target_y: c_float =  window.ScrollTarget.y;
        if (window.ScrollTargetEdgeSnapDist.y > 0.0)
        {
            let snap_y_min: c_float =  0.0;
            let snap_y_max: c_float =  window.ScrollMax.y + window.SizeFull.y - decoration_total_height;
            scroll_target_y = CalcScrollEdgeSnap(scroll_target_y, snap_y_min, snap_y_max, window.ScrollTargetEdgeSnapDist.y, center_y_ratio);
        }
        scroll.y = scroll_target_y - center_y_ratio * (window.SizeFull.y - decoration_total_height);
    }
    scroll.x = IM_FLOOR(ImMax(scroll.x, 0.0));
    scroll.y = IM_FLOOR(ImMax(scroll.y, 0.0));
    if (!window.Collapsed && !window.SkipItems)
    {
        scroll.x = ImMin(scroll.x, window.ScrollMax.x);
        scroll.y = ImMin(scroll.y, window.ScrollMax.y);
    }
    return scroll;
}

c_void ScrollToItem(ImGuiScrollFlags flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    ScrollToRectEx(window, g.LastItemData.NavRect, flags);
}

c_void ScrollToRect(window: *mut ImGuiWindow, item_rect: &ImRect, ImGuiScrollFlags flags)
{
    ScrollToRectEx(window, item_rect, flags);
}

// Scroll to keep newly navigated item fully into view
ScrollToRectEx: ImVec2(window: *mut ImGuiWindow, item_rect: &ImRect, ImGuiScrollFlags flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window_rect: ImRect = ImRect::new(window.InnerRect.Min - ImVec2::new(1, 1), window.InnerRect.Max + ImVec2::new(1, 1));
    //GetForegroundDrawList(window)->AddRect(window_rect.Min, window_rect.Max, IM_COL32_WHITE); // [DEBUG]

    // Check that only one behavior is selected per axis
    // IM_ASSERT((flags & ImGuiScrollFlags_MaskX_) == 0 || ImIsPowerOfTwo(flags & ImGuiScrollFlags_MaskX_));
    // IM_ASSERT((flags & ImGuiScrollFlags_MaskY_) == 0 || ImIsPowerOfTwo(flags & ImGuiScrollFlags_MaskY_));

    // Defaults
    ImGuiScrollFlags in_flags = flags;
    if ((flags & ImGuiScrollFlags_MaskX_) == 0 && window.ScrollbarX)
        flags |= ImGuiScrollFlags_KeepVisibleEdgeX;
    if ((flags & ImGuiScrollFlags_MaskY_) == 0)
        flags |= window.Appearing ? ImGuiScrollFlags_AlwaysCenterY : ImGuiScrollFlags_KeepVisibleEdgeY;

    let fully_visible_x: bool = item_rect.Min.x >= window_rect.Min.x && item_rect.Max.x <= window_rect.Max.x;
    let fully_visible_y: bool = item_rect.Min.y >= window_rect.Min.y && item_rect.Max.y <= window_rect.Max.y;
    let can_be_fully_visible_x: bool = (item_rect.GetWidth() + g.Style.ItemSpacing.x * 2.00f32) <= window_rect.GetWidth();
    let can_be_fully_visible_y: bool = (item_rect.GetHeight() + g.Style.ItemSpacing.y * 2.00f32) <= window_rect.GetHeight();

    if ((flags & ImGuiScrollFlags_KeepVisibleEdgeX) && !fully_visible_x)
    {
        if (item_rect.Min.x < window_rect.Min.x || !can_be_fully_visible_x)
            SetScrollFromPosX(window, item_rect.Min.x - g.Style.ItemSpacing.x - window.Pos.x, 0.0);
        else if (item_rect.Max.x >= window_rect.Max.x)
            SetScrollFromPosX(window, item_rect.Max.x + g.Style.ItemSpacing.x - window.Pos.x, 1.0);
    }
    else if (((flags & ImGuiScrollFlags_KeepVisibleCenterX) && !fully_visible_x) || (flags & ImGuiScrollFlags_AlwaysCenterX))
    {
        let target_x: c_float =  can_be_fully_visible_x ? ImFloor((item_rect.Min.x + item_rect.Max.x - window.InnerRect.GetWidth()) * 0.5f32) : item_rect.Min.x;
        SetScrollFromPosX(window, target_x - window.Pos.x, 0.0);
    }

    if ((flags & ImGuiScrollFlags_KeepVisibleEdgeY) && !fully_visible_y)
    {
        if (item_rect.Min.y < window_rect.Min.y || !can_be_fully_visible_y)
            SetScrollFromPosY(window, item_rect.Min.y - g.Style.ItemSpacing.y - window.Pos.y, 0.0);
        else if (item_rect.Max.y >= window_rect.Max.y)
            SetScrollFromPosY(window, item_rect.Max.y + g.Style.ItemSpacing.y - window.Pos.y, 1.0);
    }
    else if (((flags & ImGuiScrollFlags_KeepVisibleCenterY) && !fully_visible_y) || (flags & ImGuiScrollFlags_AlwaysCenterY))
    {
        let target_y: c_float =  can_be_fully_visible_y ? ImFloor((item_rect.Min.y + item_rect.Max.y - window.InnerRect.GetHeight()) * 0.5f32) : item_rect.Min.y;
        SetScrollFromPosY(window, target_y - window.Pos.y, 0.0);
    }

    let next_scroll: ImVec2 = CalcNextScrollFromScrollTargetAndClamp(window);
    let delta_scroll: ImVec2 = next_scroll - window.Scroll;

    // Also scroll parent window to keep us into view if necessary
    if (!(flags & ImGuiScrollFlags_NoScrollParent) && (window.Flags & ImGuiWindowFlags_ChildWindow))
    {
        // FIXME-SCROLL: May be an option?
        if ((in_flags & (ImGuiScrollFlags_AlwaysCenterX | ImGuiScrollFlags_KeepVisibleCenterX)) != 0)
            in_flags = (in_flags & !ImGuiScrollFlags_MaskX_) | ImGuiScrollFlags_KeepVisibleEdgeX;
        if ((in_flags & (ImGuiScrollFlags_AlwaysCenterY | ImGuiScrollFlags_KeepVisibleCenterY)) != 0)
            in_flags = (in_flags & !ImGuiScrollFlags_MaskY_) | ImGuiScrollFlags_KeepVisibleEdgeY;
        delta_scroll += ScrollToRectEx(window.ParentWindow, ImRect(item_rect.Min - delta_scroll, item_rect.Max - delta_scroll), in_flags);
    }

    return delta_scroll;
}GetScrollX: c_float()
{
    let mut window: *mut ImGuiWindow =  GimGui.CurrentWindow;
    return window.Scroll.x;
}GetScrollY: c_float()
{
    let mut window: *mut ImGuiWindow =  GimGui.CurrentWindow;
    return window.Scroll.y;
}GetScrollMaxX: c_float()
{
    let mut window: *mut ImGuiWindow =  GimGui.CurrentWindow;
    return window.ScrollMax.x;
}GetScrollMaxY: c_float()
{
    let mut window: *mut ImGuiWindow =  GimGui.CurrentWindow;
    return window.ScrollMax.y;
}

c_void SetScrollX(window: *mut ImGuiWindow,scroll_x: c_float)
{
    window.ScrollTarget.x = scroll_x;
    window.ScrollTargetCenterRatio.x = 0.0;
    window.ScrollTargetEdgeSnapDist.x = 0.0;
}

c_void SetScrollY(window: *mut ImGuiWindow,scroll_y: c_float)
{
    window.ScrollTarget.y = scroll_y;
    window.ScrollTargetCenterRatio.y = 0.0;
    window.ScrollTargetEdgeSnapDist.y = 0.0;
}

c_void SetScrollX(scroll_x: c_float)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    SetScrollX(g.CurrentWindow, scroll_x);
}

c_void SetScrollY(scroll_y: c_float)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    SetScrollY(g.CurrentWindow, scroll_y);
}

// Note that a local position will vary depending on initial scroll value,
// This is a little bit confusing so bear with us:
//  - local_pos = (absolution_pos - window.Pos)
//  - So local_x/local_y are 0.0 for a position at the upper-left corner of a window,
//    and generally local_x/local_y are >(padding+decoration) && <(size-padding-decoration) when in the visible area.
//  - They mostly exists because of legacy API.
// Following the rules above, when trying to work with scrolling code, consider that:
//  - SetScrollFromPosY(0.0) == SetScrollY(0.0 + scroll.y) == has no effect!
//  - SetScrollFromPosY(-scroll.y) == SetScrollY(-scroll.y + scroll.y) == SetScrollY(0.0) == reset scroll. Of course writing SetScrollY(0.0) directly then makes more sense
// We store a target position so centering and clamping can occur on the next frame when we are guaranteed to have a known window size
c_void SetScrollFromPosX(window: *mut ImGuiWindow,local_x: c_float,center_x_ratio: c_float)
{
    // IM_ASSERT(center_x_ratio >= 0.0 && center_x_ratio <= 1.0);
    window.ScrollTarget.x = IM_FLOOR(local_x + window.Scroll.x); // Convert local position to scroll offset
    window.ScrollTargetCenterRatio.x = center_x_ratio;
    window.ScrollTargetEdgeSnapDist.x = 0.0;
}

c_void SetScrollFromPosY(window: *mut ImGuiWindow,local_y: c_float,center_y_ratio: c_float)
{
    // IM_ASSERT(center_y_ratio >= 0.0 && center_y_ratio <= 1.0);
    let decoration_up_height: c_float =  window.TitleBarHeight() + window.MenuBarHeight(); // FIXME: Would be nice to have a more standardized access to our scrollable/client rect;
    local_y -= decoration_up_height;
    window.ScrollTarget.y = IM_FLOOR(local_y + window.Scroll.y); // Convert local position to scroll offset
    window.ScrollTargetCenterRatio.y = center_y_ratio;
    window.ScrollTargetEdgeSnapDist.y = 0.0;
}

c_void SetScrollFromPosX(local_x: c_float,center_x_ratio: c_float)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    SetScrollFromPosX(g.CurrentWindow, local_x, center_x_ratio);
}

c_void SetScrollFromPosY(local_y: c_float,center_y_ratio: c_float)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    SetScrollFromPosY(g.CurrentWindow, local_y, center_y_ratio);
}

// center_x_ratio: 0.0 left of last item, 0.5f32 horizontal center of last item, 1.0 right of last item.
c_void SetScrollHereX(center_x_ratio: c_float)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    let spacing_x: c_float =  ImMax(window.WindowPadding.x, g.Style.ItemSpacing.x);
    let target_pos_x: c_float =  ImLerp(g.LastItemData.Rect.Min.x - spacing_x, g.LastItemData.Rect.Max.x + spacing_x, center_x_ratio);
    SetScrollFromPosX(window, target_pos_x - window.Pos.x, center_x_ratio); // Convert from absolute to local pos

    // Tweak: snap on edges when aiming at an item very close to the edge
    window.ScrollTargetEdgeSnapDist.x = ImMax(0.0, window.WindowPadding.x - spacing_x);
}

// center_y_ratio: 0.0 top of last item, 0.5f32 vertical center of last item, 1.0 bottom of last item.
c_void SetScrollHereY(center_y_ratio: c_float)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    let spacing_y: c_float =  ImMax(window.WindowPadding.y, g.Style.ItemSpacing.y);
    let target_pos_y: c_float =  ImLerp(window.DC.CursorPosPrevLine.y - spacing_y, window.DC.CursorPosPrevLine.y + window.DC.PrevLineSize.y + spacing_y, center_y_ratio);
    SetScrollFromPosY(window, target_pos_y - window.Pos.y, center_y_ratio); // Convert from absolute to local pos

    // Tweak: snap on edges when aiming at an item very close to the edge
    window.ScrollTargetEdgeSnapDist.y = ImMax(0.0, window.WindowPadding.y - spacing_y);
}

//-----------------------------------------------------------------------------
// [SECTION] TOOLTIPS
//-----------------------------------------------------------------------------

c_void BeginTooltip()
{
    BeginTooltipEx(ImGuiTooltipFlags_None, ImGuiWindowFlags_None);
}

c_void BeginTooltipEx(ImGuiTooltipFlags tooltip_flags, extra_window_flags: ImGuiWindowFlags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    if (g.DragDropWithinSource || g.DragDropWithinTarget)
    {
        // The default tooltip position is a little offset to give space to see the context menu (it's also clamped within the current viewport/monitor)
        // In the context of a dragging tooltip we try to reduce that offset and we enforce following the cursor.
        // Whatever we do we want to call SetNextWindowPos() to enforce a tooltip position and disable clipping the tooltip without our display area, like regular tooltip do.
        //let mut tooltip_pos: ImVec2 =  g.IO.MousePos - g.ActiveIdClickOffset - g.Style.WindowPadding;
        let tooltip_pos: ImVec2 = g.IO.MousePos + ImVec2::new(16 * g.Style.MouseCursorScale, 8 * g.Style.MouseCursorScale);
        SetNextWindowPos(tooltip_pos);
        SetNextWindowBgAlpha(g.Style.Colors[ImGuiCol_PopupBg].w * 0.600f32);
        //PushStyleVar(ImGuiStyleVar_Alpha, g.Style.Alpha * 0.600f32); // This would be nice but e.g ColorButton with checkboard has issue with transparent colors :(
        tooltip_flags |= ImGuiTooltipFlags_OverridePreviousTooltip;
    }

    window_name: [c_char;16];
    ImFormatString(window_name, window_name.len(), "##Tooltip_%02d", g.TooltipOverrideCount);
    if (tooltip_flags & ImGuiTooltipFlags_OverridePreviousTooltip)
        if (let mut window: *mut ImGuiWindow =  FindWindowByName(window_name))
            if (window.Active)
            {
                // Hide previous tooltip from being displayed. We can't easily "reset" the content of a window so we create a new one.
                window.Hidden = true;
                window.HiddenFramesCanSkipItems = 1; // FIXME: This may not be necessary?
                ImFormatString(window_name, window_name.len(), "##Tooltip_%02d", ++g.TooltipOverrideCount);
            }
    flags: ImGuiWindowFlags = ImGuiWindowFlags_Tooltip | ImGuiWindowFlags_NoInputs | ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoMove | ImGuiWindowFlags_NoResize | ImGuiWindowFlags_NoSavedSettings | ImGuiWindowFlags_AlwaysAutoResize | ImGuiWindowFlags_NoDocking;
    Begin(window_name, null_mut(), flags | extra_window_flags);
}

c_void EndTooltip()
{
    // IM_ASSERT(GetCurrentWindowRead()->Flags & ImGuiWindowFlags_Tooltip);   // Mismatched BeginTooltip()/EndTooltip() calls
    End();
}

c_void SetTooltipV(fmt: *const c_char, va_list args)
{
    BeginTooltipEx(ImGuiTooltipFlags_OverridePreviousTooltip, ImGuiWindowFlags_None);
    TextV(fmt, args);
    EndTooltip();
}

c_void SetTooltip(fmt: *const c_char, ...)
{
    va_list args;
    va_start(args, fmt);
    SetTooltipV(fmt, args);
    va_end(args);
}

//-----------------------------------------------------------------------------
// [SECTION] POPUPS
//-----------------------------------------------------------------------------

// Supported flags: ImGuiPopupFlags_AnyPopupId, ImGuiPopupFlags_AnyPopupLevel
IsPopupOpen: bool(id: ImGuiID, ImGuiPopupFlags popup_flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (popup_flags & ImGuiPopupFlags_AnyPopupId)
    {
        // Return true if any popup is open at the current BeginPopup() level of the popup stack
        // This may be used to e.g. test for another popups already opened to handle popups priorities at the same level.
        // IM_ASSERT(id == 0);
        if (popup_flags & ImGuiPopupFlags_AnyPopupLevel)
            return g.OpenPopupStack.Size > 0;
        else
            return g.OpenPopupStack.Size > g.BeginPopupStack.Size;
    }
    else
    {
        if (popup_flags & ImGuiPopupFlags_AnyPopupLevel)
        {
            // Return true if the popup is open anywhere in the popup stack
            for (let n: c_int = 0; n < g.OpenPopupStack.Size; n++)
                if (g.OpenPopupStack[n].PopupId == id)
                    return true;
            return false;
        }
        else
        {
            // Return true if the popup is open at the current BeginPopup() level of the popup stack (this is the most-common query)
            return g.OpenPopupStack.Size > g.BeginPopupStack.Size && g.OpenPopupStack[g.BeginPopupStack.Size].PopupId == id;
        }
    }
}

IsPopupOpen: bool(str_id: *const c_char, ImGuiPopupFlags popup_flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut id: ImGuiID =  (popup_flags & ImGuiPopupFlags_AnyPopupId) ? 0 : g.Currentwindow.GetID(str_id);
    if ((popup_flags & ImGuiPopupFlags_AnyPopupLevel) && id != 0)
        // IM_ASSERT(0 && "Cannot use IsPopupOpen() with a string id and ImGuiPopupFlags_AnyPopupLevel."); // But non-string version is legal and used internally
    return IsPopupOpen(id, popup_flags);
}

GetTopMostPopupModal: *mut ImGuiWindow()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    for (let n: c_int = g.OpenPopupStack.Size - 1; n >= 0; n--)
        if (let mut popup: *mut ImGuiWindow =  g.OpenPopupStack.Data[n].Window)
            if (popup->Flags & ImGuiWindowFlags_Modal)
                return popup;
    return null_mut();
}

GetTopMostAndVisiblePopupModal: *mut ImGuiWindow()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    for (let n: c_int = g.OpenPopupStack.Size - 1; n >= 0; n--)
        if (let mut popup: *mut ImGuiWindow =  g.OpenPopupStack.Data[n].Window)
            if ((popup->Flags & ImGuiWindowFlags_Modal) && IsWindowActiveAndVisible(popup))
                return popup;
    return null_mut();
}

c_void OpenPopup(str_id: *const c_char, ImGuiPopupFlags popup_flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut id: ImGuiID =  g.Currentwindow.GetID(str_id);
    IMGUI_DEBUG_LOG_POPUP("[popup] OpenPopup(\"%s\" -> 0x%08X\n", str_id, id);
    OpenPopupEx(id, popup_flags);
}

c_void OpenPopup(id: ImGuiID, ImGuiPopupFlags popup_flags)
{
    OpenPopupEx(id, popup_flags);
}

// Mark popup as open (toggle toward open state).
// Popups are closed when user click outside, or activate a pressable item, or CloseCurrentPopup() is called within a BeginPopup()/EndPopup() block.
// Popup identifiers are relative to the current ID-stack (so OpenPopup and BeginPopup needs to be at the same level).
// One open popup per level of the popup hierarchy (NB: when assigning we reset the Window member of ImGuiPopupRef to NULL)
c_void OpenPopupEx(id: ImGuiID, ImGuiPopupFlags popup_flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut parent_window: *mut ImGuiWindow =  g.CurrentWindow;
    let current_stack_size: c_int = g.BeginPopupStack.Size;

    if (popup_flags & ImGuiPopupFlags_NoOpenOverExistingPopup)
        if (IsPopupOpen(0u, ImGuiPopupFlags_AnyPopupId))
            return;

    ImGuiPopupData popup_ref; // Tagged as new ref as Window will be set back to NULL if we write this into OpenPopupStack.
    popup_ref.PopupId = id;
    popup_ref.Window= null_mut();
    popup_ref.BackupNavWindow = g.NavWindow;            // When popup closes focus may be restored to NavWindow (depend on window type).
    popup_ref.OpenFrameCount = g.FrameCount;
    popup_ref.OpenParentId = parent_window.IDStack.last().unwrap();
    popup_ref.OpenPopupPos = NavCalcPreferredRefPos();
    popup_ref.OpenMousePos = IsMousePosValid(&g.IO.MousePos) ? g.IO.MousePos : popup_ref.OpenPopupPos;

    IMGUI_DEBUG_LOG_POPUP("[popup] OpenPopupEx(0x%08X)\n", id);
    if (g.OpenPopupStack.Size < current_stack_size + 1)
    {
        g.OpenPopupStack.push(popup_re0f32);
    }
    else
    {
        // Gently handle the user mistakenly calling OpenPopup() every frame. It is a programming mistake! However, if we were to run the regular code path, the ui
        // would become completely unusable because the popup will always be in hidden-while-calculating-size state _while_ claiming focus. Which would be a very confusing
        // situation for the programmer. Instead, we silently allow the popup to proceed, it will keep reappearing and the programming error will be more obvious to understand.
        if (g.OpenPopupStack[current_stack_size].PopupId == id && g.OpenPopupStack[current_stack_size].OpenFrameCount == g.FrameCount - 1)
        {
            g.OpenPopupStack[current_stack_size].OpenFrameCount = popup_ref.OpenFrameCount;
        }
        else
        {
            // Close child popups if any, then flag popup for open/reopen
            ClosePopupToLevel(current_stack_size, false);
            g.OpenPopupStack.push(popup_re0f32);
        }

        // When reopening a popup we first refocus its parent, otherwise if its parent is itself a popup it would get closed by ClosePopupsOverWindow().
        // This is equivalent to what ClosePopupToLevel() does.
        //if (g.OpenPopupStack[current_stack_size].PopupId == id)
        //    FocusWindow(parent_window);
    }
}

// When popups are stacked, clicking on a lower level popups puts focus back to it and close popups above it.
// This function closes any popups that are over 'ref_window'.
c_void ClosePopupsOverWindow(ref_window: *mut ImGuiWindow, restore_focus_to_window_under_popup: bool)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.OpenPopupStack.Size == 0)
        return;

    // Don't close our own child popup windows.
    let popup_count_to_keep: c_int = 0;
    if (ref_window)
    {
        // Find the highest popup which is a descendant of the reference window (generally reference window = NavWindow)
        for (; popup_count_to_keep < g.OpenPopupStack.Size; popup_count_to_keep++)
        {
            ImGuiPopupData& popup = g.OpenPopupStack[popup_count_to_keep];
            if (!popup.Window)
                continue;
            // IM_ASSERT((popup.window.Flags & ImGuiWindowFlags_Popup) != 0);
            if (popup.window.Flags & ImGuiWindowFlags_ChildWindow)
                continue;

            // Trim the stack unless the popup is a direct parent of the reference window (the reference window is often the NavWindow)
            // - With this stack of window, clicking/focusing Popup1 will close Popup2 and Popup3:
            //     Window -> Popup1 -> Popup2 -> Popup3
            // - Each popups may contain child windows, which is why we compare ->RootWindowDockTree!
            //     Window -> Popup1 -> Popup1_Child -> Popup2 -> Popup2_Child
            let mut ref_window_is_descendent_of_popup: bool =  false;
            for (let n: c_int = popup_count_to_keep; n < g.OpenPopupStack.Size; n++)
                if (let mut popup_window: *mut ImGuiWindow =  g.OpenPopupStack[n].Window)
                    //if (popup_window.RootWindowDockTree == ref_window.RootWindowDockTree) // FIXME-MERGE
                    if (IsWindowWithinBeginStackOf(ref_window, popup_window))
                    {
                        ref_window_is_descendent_of_popup = true;
                        break;
                    }
            if (!ref_window_is_descendent_of_popup)
                break;
        }
    }
    if (popup_count_to_keep < g.OpenPopupStack.Size) // This test is not required but it allows to set a convenient breakpoint on the statement below
    {
        IMGUI_DEBUG_LOG_POPUP("[popup] ClosePopupsOverWindow(\"%s\")\n", ref_window ? ref_window.Name : "<NULL>");
        ClosePopupToLevel(popup_count_to_keep, restore_focus_to_window_under_popup);
    }
}

c_void ClosePopupsExceptModals()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    let mut popup_count_to_keep: c_int = 0;
    for (popup_count_to_keep = g.OpenPopupStack.Size; popup_count_to_keep > 0; popup_count_to_keep--)
    {
        let mut window: *mut ImGuiWindow =  g.OpenPopupStack[popup_count_to_keep - 1].Window;
        if (!window || window.Flags & ImGuiWindowFlags_Modal)
            break;
    }
    if (popup_count_to_keep < g.OpenPopupStack.Size) // This test is not required but it allows to set a convenient breakpoint on the statement below
        ClosePopupToLevel(popup_count_to_keep, true);
}

c_void ClosePopupToLevel(remaining: c_int, restore_focus_to_window_under_popup: bool)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    IMGUI_DEBUG_LOG_POPUP("[popup] ClosePopupToLevel(%d), restore_focus_to_window_under_popup=%d\n", remaining, restore_focus_to_window_under_popup);
    // IM_ASSERT(remaining >= 0 && remaining < g.OpenPopupStack.Size);

    // Trim open popup stack
    let mut popup_window: *mut ImGuiWindow =  g.OpenPopupStack[remaining].Window;
    let mut popup_backup_nav_window: *mut ImGuiWindow =  g.OpenPopupStack[remaining].BackupNavWindow;
    g.OpenPopupStack.resize(remaining);

    if (restore_focus_to_window_under_popup)
    {
        let mut focus_window: *mut ImGuiWindow =  (popup_window && popup_window.Flags & ImGuiWindowFlags_ChildMenu) ? popup_window.ParentWindow : popup_backup_nav_window;
        if (focus_window && !focus_window.WasActive && popup_window)
        {
            // Fallback
            FocusTopMostWindowUnderOne(popup_window, null_mut());
        }
        else
        {
            if (g.NavLayer == ImGuiNavLayer_Main && focus_window)
                focus_window = NavRestoreLastChildNavWindow(focus_window);
            FocusWindow(focus_window);
        }
    }
}

// Close the popup we have begin-ed into.
c_void CloseCurrentPopup()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let popup_idx: c_int = g.BeginPopupStack.Size - 1;
    if (popup_idx < 0 || popup_idx >= g.OpenPopupStack.Size || g.BeginPopupStack[popup_idx].PopupId != g.OpenPopupStack[popup_idx].PopupId)
        return;

    // Closing a menu closes its top-most parent popup (unless a modal)
    while (popup_idx > 0)
    {
        let mut popup_window: *mut ImGuiWindow =  g.OpenPopupStack[popup_idx].Window;
        let mut parent_popup_window: *mut ImGuiWindow =  g.OpenPopupStack[popup_idx - 1].Window;
        let mut close_parent: bool =  false;
        if (popup_window && (popup_window.Flags & ImGuiWindowFlags_ChildMenu))
            if (parent_popup_window && !(parent_popup_window.Flags & ImGuiWindowFlags_MenuBar))
                close_parent = true;
        if (!close_parent)
            break;
        popup_idx-= 1;
    }
    IMGUI_DEBUG_LOG_POPUP("[popup] CloseCurrentPopup %d -> %d\n", g.BeginPopupStack.Size - 1, popup_idx);
    ClosePopupToLevel(popup_idx, true);

    // A common pattern is to close a popup when selecting a menu item/selectable that will open another window.
    // To improve this usage pattern, we avoid nav highlight for a single frame in the parent window.
    // Similarly, we could avoid mouse hover highlight in this window but it is less visually problematic.
    if (let mut window: *mut ImGuiWindow =  g.NavWindow)
        window.DC.NavHideHighlightOneFrame = true;
}

// Attention! BeginPopup() adds default flags which BeginPopupEx()!
BeginPopupEx: bool(id: ImGuiID, flags: ImGuiWindowFlags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (!IsPopupOpen(id, ImGuiPopupFlags_None))
    {
        g.NextWindowData.ClearFlags(); // We behave like Begin() and need to consume those values
        return false;
    }

    name: [c_char;20];
    if (flags & ImGuiWindowFlags_ChildMenu)
        ImFormatString(name, name.len(), "##Menu_%02d", g.BeginMenuCount); // Recycle windows based on depth
    else
        ImFormatString(name, name.len(), "##Popup_%08x", id); // Not recycling, so we can close/open during the same frame

    flags |= ImGuiWindowFlags_Popup | ImGuiWindowFlags_NoDocking;
    let mut is_open: bool =  Begin(name, null_mut(), flags);
    if (!is_open) // NB: Begin can return false when the popup is completely clipped (e.g. zero size display)
        EndPopup();

    return is_open;
}

BeginPopup: bool(str_id: *const c_char, flags: ImGuiWindowFlags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.OpenPopupStack.Size <= g.BeginPopupStack.Size) // Early out for performance
    {
        g.NextWindowData.ClearFlags(); // We behave like Begin() and need to consume those values
        return false;
    }
    flags |= ImGuiWindowFlags_AlwaysAutoResize | ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoSavedSettings;
    let mut id: ImGuiID =  g.Currentwindow.GetID(str_id);
    return BeginPopupEx(id, flags);
}

// If 'p_open' is specified for a modal popup window, the popup will have a regular close button which will close the popup.
// Note that popup visibility status is owned by Dear ImGui (and manipulated with e.g. OpenPopup) so the actual value of *p_open is meaningless here.
BeginPopupModal: bool(name: *const c_char,p_open: *mut bool, flags: ImGuiWindowFlags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    let mut id: ImGuiID =  window.GetID(name);
    if (!IsPopupOpen(id, ImGuiPopupFlags_None))
    {
        g.NextWindowData.ClearFlags(); // We behave like Begin() and need to consume those values
        return false;
    }

    // Center modal windows by default for increased visibility
    // (this won't really last as settings will kick in, and is mostly for backward compatibility. user may do the same themselves)
    // FIXME: Should test for (PosCond & window.SetWindowPosAllowFlags) with the upcoming window.
    if ((g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasPos) == 0)
    {
        let viewport: *const ImGuiViewport = window.WasActive ? window.Viewport : GetMainViewport(); // FIXME-VIEWPORT: What may be our reference viewport?
        SetNextWindowPos(viewport.GetCenter(), ImGuiCond_FirstUseEver, ImVec2::new(0.5f32, 0.5f32));
    }

    flags |= ImGuiWindowFlags_Popup | ImGuiWindowFlags_Modal | ImGuiWindowFlags_NoCollapse | ImGuiWindowFlags_NoDocking;
    let is_open: bool = Begin(name, p_open, flags);
    if (!is_open || (p_open && !*p_open)) // NB: is_open can be 'false' when the popup is completely clipped (e.g. zero size display)
    {
        EndPopup();
        if (is_open)
            ClosePopupToLevel(g.BeginPopupStack.Size, true);
        return false;
    }
    return is_open;
}

c_void EndPopup()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    // IM_ASSERT(window.Flags & ImGuiWindowFlags_Popup);  // Mismatched BeginPopup()/EndPopup() calls
    // IM_ASSERT(g.BeginPopupStack.Size > 0);

    // Make all menus and popups wrap around for now, may need to expose that policy (e.g. focus scope could include wrap/loop policy flags used by new move requests)
    if (g.NavWindow == window)
        NavMoveRequestTryWrapping(window, ImGuiNavMoveFlags_LoopY);

    // Child-popups don't need to be laid out
    // IM_ASSERT(g.WithinEndChild == false);
    if (window.Flags & ImGuiWindowFlags_ChildWindow)
        g.WithinEndChild = true;
    End();
    g.WithinEndChild = false;
}

// Helper to open a popup if mouse button is released over the item
// - This is essentially the same as BeginPopupContextItem() but without the trailing BeginPopup()
c_void OpenPopupOnItemClick(str_id: *const c_char, ImGuiPopupFlags popup_flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    let mouse_button: c_int = (popup_flags & ImGuiPopupFlags_MouseButtonMask_);
    if (IsMouseReleased(mouse_button) && IsItemHovered(ImGuiHoveredFlags_AllowWhenBlockedByPopup))
    {
        let mut id: ImGuiID =  str_id ? window.GetID(str_id) : g.LastItemData.ID;    // If user hasn't passed an ID, we can use the LastItemID. Using LastItemID as a Popup ID won't conflict!
        // IM_ASSERT(id != 0);                                             // You cannot pass a NULL str_id if the last item has no identifier (e.g. a Text() item)
        OpenPopupEx(id, popup_flags);
    }
}

// This is a helper to handle the simplest case of associating one named popup to one given widget.
// - To create a popup associated to the last item, you generally want to pass a NULL value to str_id.
// - To create a popup with a specific identifier, pass it in str_id.
//    - This is useful when using using BeginPopupContextItem() on an item which doesn't have an identifier, e.g. a Text() call.
//    - This is useful when multiple code locations may want to manipulate/open the same popup, given an explicit id.
// - You may want to handle the whole on user side if you have specific needs (e.g. tweaking IsItemHovered() parameters).
//   This is essentially the same as:
//       id = str_id ? GetID(str_id) : GetItemID();
//       OpenPopupOnItemClick(str_id, ImGuiPopupFlags_MouseButtonRight);
//       return BeginPopup(id);
//   Which is essentially the same as:
//       id = str_id ? GetID(str_id) : GetItemID();
//       if (IsItemHovered() && IsMouseReleased(ImGuiMouseButton_Right))
//           OpenPopup(id);
//       return BeginPopup(id);
//   The main difference being that this is tweaked to avoid computing the ID twice.
BeginPopupContextItem: bool(str_id: *const c_char, ImGuiPopupFlags popup_flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if (window.SkipItems)
        return false;
    let mut id: ImGuiID =  str_id ? window.GetID(str_id) : g.LastItemData.ID;    // If user hasn't passed an ID, we can use the LastItemID. Using LastItemID as a Popup ID won't conflict!
    // IM_ASSERT(id != 0);                                             // You cannot pass a NULL str_id if the last item has no identifier (e.g. a Text() item)
    let mouse_button: c_int = (popup_flags & ImGuiPopupFlags_MouseButtonMask_);
    if (IsMouseReleased(mouse_button) && IsItemHovered(ImGuiHoveredFlags_AllowWhenBlockedByPopup))
        OpenPopupEx(id, popup_flags);
    return BeginPopupEx(id, ImGuiWindowFlags_AlwaysAutoResize | ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoSavedSettings);
}

BeginPopupContextWindow: bool(str_id: *const c_char, ImGuiPopupFlags popup_flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if (!str_id)
        str_id = "window_context";
    let mut id: ImGuiID =  window.GetID(str_id);
    let mouse_button: c_int = (popup_flags & ImGuiPopupFlags_MouseButtonMask_);
    if (IsMouseReleased(mouse_button) && IsWindowHovered(ImGuiHoveredFlags_AllowWhenBlockedByPopup))
        if (!(popup_flags & ImGuiPopupFlags_NoOpenOverItems) || !IsAnyItemHovered())
            OpenPopupEx(id, popup_flags);
    return BeginPopupEx(id, ImGuiWindowFlags_AlwaysAutoResize | ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoSavedSettings);
}

BeginPopupContextVoid: bool(str_id: *const c_char, ImGuiPopupFlags popup_flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if (!str_id)
        str_id = "void_context";
    let mut id: ImGuiID =  window.GetID(str_id);
    let mouse_button: c_int = (popup_flags & ImGuiPopupFlags_MouseButtonMask_);
    if (IsMouseReleased(mouse_button) && !IsWindowHovered(ImGuiHoveredFlags_AnyWindow))
        if (GetTopMostPopupModal() == null_mut())
            OpenPopupEx(id, popup_flags);
    return BeginPopupEx(id, ImGuiWindowFlags_AlwaysAutoResize | ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoSavedSettings);
}

// r_avoid = the rectangle to avoid (e.g. for tooltip it is a rectangle around the mouse cursor which we want to avoid. for popups it's a small point around the cursor.)
// r_outer = the visible area rectangle, minus safe area padding. If our popup size won't fit because of safe area padding we ignore it.
// (r_outer is usually equivalent to the viewport rectangle minus padding, but when multi-viewports are enabled and monitor
//  information are available, it may represent the entire platform monitor from the frame of reference of the current viewport.
//  this allows us to have tooltips/popups displayed out of the parent viewport.)
FindBestWindowPosForPopupEx: ImVec2(ref_pos: &ImVec2, size: &ImVec2, ImGuiDir* last_dir, r_outer: &ImRect, r_avoid: &ImRect, ImGuiPopupPositionPolicy policy)
{
    let base_pos_clamped: ImVec2 = ImClamp(ref_pos, r_outer.Min, r_outer.Max - size);
    //GetForegroundDrawList()->AddRect(r_avoid.Min, r_avoid.Max, IM_COL32(255,0,0,255));
    //GetForegroundDrawList()->AddRect(r_outer.Min, r_outer.Max, IM_COL32(0,255,0,255));

    // Combo Box policy (we want a connecting edge)
    if (policy == ImGuiPopupPositionPolicy_ComboBox)
    {
        const dir_prefered_order: ImGuiDir[ImGuiDir_COUNT] = { ImGuiDir_Down, ImGuiDir_Right, ImGuiDir_Left, ImGuiDir_Up };
        for (let n: c_int = (*last_dir != ImGuiDir_None) ? -1 : 0; n < ImGuiDir_COUNT; n++)
        {
            const dir: ImGuiDir = (n == -1) ? *last_dir : dir_prefered_order[n];
            if (n != -1 && dir == *last_dir) // Already tried this direction?
                continue;
            pos: ImVec2;
            if (dir == ImGuiDir_Down)  pos = ImVec2::new(r_avoid.Min.x, r_avoid.Max.y);          // Below, Toward Right (default)
            if (dir == ImGuiDir_Right) pos = ImVec2::new(r_avoid.Min.x, r_avoid.Min.y - size.y); // Above, Toward Right
            if (dir == ImGuiDir_Left)  pos = ImVec2::new(r_avoid.Max.x - size.x, r_avoid.Max.y); // Below, Toward Left
            if (dir == ImGuiDir_Up)    pos = ImVec2::new(r_avoid.Max.x - size.x, r_avoid.Min.y - size.y); // Above, Toward Left
            if (!r_outer.Contains(ImRect(pos, pos + size)))
                continue;
            *last_dir = dir;
            return pos;
        }
    }

    // Tooltip and Default popup policy
    // (Always first try the direction we used on the last frame, if any)
    if (policy == ImGuiPopupPositionPolicy_Tooltip || policy == ImGuiPopupPositionPolicy_Default)
    {
        const dir_prefered_order: ImGuiDir[ImGuiDir_COUNT] = { ImGuiDir_Right, ImGuiDir_Down, ImGuiDir_Up, ImGuiDir_Left };
        for (let n: c_int = (*last_dir != ImGuiDir_None) ? -1 : 0; n < ImGuiDir_COUNT; n++)
        {
            const dir: ImGuiDir = (n == -1) ? *last_dir : dir_prefered_order[n];
            if (n != -1 && dir == *last_dir) // Already tried this direction?
                continue;

            let avail_w: c_float =  (dir == ImGuiDir_Left ? r_avoid.Min.x : r_outer.Max.x) - (dir == ImGuiDir_Right ? r_avoid.Max.x : r_outer.Min.x);
            let avail_h: c_float =  (dir == ImGuiDir_Up ? r_avoid.Min.y : r_outer.Max.y) - (dir == ImGuiDir_Down ? r_avoid.Max.y : r_outer.Min.y);

            // If there not enough room on one axis, there's no point in positioning on a side on this axis (e.g. when not enough width, use a top/bottom position to maximize available width)
            if (avail_w < size.x && (dir == ImGuiDir_Left || dir == ImGuiDir_Right))
                continue;
            if (avail_h < size.y && (dir == ImGuiDir_Up || dir == ImGuiDir_Down))
                continue;

            pos: ImVec2;
            pos.x = (dir == ImGuiDir_Left) ? r_avoid.Min.x - size.x : (dir == ImGuiDir_Right) ? r_avoid.Max.x : base_pos_clamped.x;
            pos.y = (dir == ImGuiDir_Up) ? r_avoid.Min.y - size.y : (dir == ImGuiDir_Down) ? r_avoid.Max.y : base_pos_clamped.y;

            // Clamp top-left corner of popup
            pos.x = ImMax(pos.x, r_outer.Min.x);
            pos.y = ImMax(pos.y, r_outer.Min.y);

            *last_dir = dir;
            return pos;
        }
    }

    // Fallback when not enough room:
    *last_dir = ImGuiDir_None;

    // For tooltip we prefer avoiding the cursor at all cost even if it means that part of the tooltip won't be visible.
    if (policy == ImGuiPopupPositionPolicy_Tooltip)
        return ref_pos + ImVec2::new(2, 2);

    // Otherwise try to keep within display
    let pos: ImVec2 = ref_pos;
    pos.x = ImMax(ImMin(pos.x + size.x, r_outer.Max.x) - size.x, r_outer.Min.x);
    pos.y = ImMax(ImMin(pos.y + size.y, r_outer.Max.y) - size.y, r_outer.Min.y);
    return pos;
}

// Note that this is used for popups, which can overlap the non work-area of individual viewports.
ImRect GetPopupAllowedExtentRect(window: *mut ImGuiWindow)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImRect r_screen;
    if (window.ViewportAllowPlatformMonitorExtend >= 0)
    {
        // Extent with be in the frame of reference of the given viewport (so Min is likely to be negative here)
        const ImGuiPlatformMonitor& monitor = g.PlatformIO.Monitors[window.ViewportAllowPlatformMonitorExtend];
        r_screen.Min = monitor.WorkPos;
        r_screen.Max = monitor.WorkPos + monitor.WorkSize;
    }
    else
    {
        // Use the full viewport area (not work area) for popups
        r_screen = window.Viewport.GetMainRect();
    }
    let padding: ImVec2 = g.Style.DisplaySafeAreaPadding;
    r_screen.Expand(ImVec2::new((r_screen.GetWidth() > padding.x * 2) ? -padding.x : 0.0, (r_screen.GetHeight() > padding.y * 2) ? -padding.y : 0.0));
    return r_screen;
}

FindBestWindowPosForPopup: ImVec2(window: *mut ImGuiWindow)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    let r_outer: ImRect =  GetPopupAllowedExtentRect(window);
    if (window.Flags & ImGuiWindowFlags_ChildMenu)
    {
        // Child menus typically request _any_ position within the parent menu item, and then we move the new menu outside the parent bounds.
        // This is how we end up with child menus appearing (most-commonly) on the right of the parent menu.
        let mut parent_window: *mut ImGuiWindow =  window.ParentWindow;
        let horizontal_overlap: c_float =  g.Style.ItemInnerSpacing.x; // We want some overlap to convey the relative depth of each menu (currently the amount of overlap is hard-coded to style.ItemSpacing.x).
        ImRect r_avoid;
        if (parent_window.DC.MenuBarAppending)
            r_avoid = ImRect(-f32::MAX, parent_window.ClipRect.Min.y, f32::MAX, parent_window.ClipRect.Max.y); // Avoid parent menu-bar. If we wanted multi-line menu-bar, we may instead want to have the calling window setup e.g. a NextWindowData.PosConstraintAvoidRect field
        else
            r_avoid = ImRect(parent_window.Pos.x + horizontal_overlap, -f32::MAX, parent_window.Pos.x + parent_window.Size.x - horizontal_overlap - parent_window.ScrollbarSizes.x, f32::MAX);
        return FindBestWindowPosForPopupEx(window.Pos, window.Size, &window.AutoPosLastDirection, r_outer, r_avoid, ImGuiPopupPositionPolicy_Default);
    }
    if (window.Flags & ImGuiWindowFlags_Popup)
    {
        return FindBestWindowPosForPopupEx(window.Pos, window.Size, &window.AutoPosLastDirection, r_outer, ImRect(window.Pos, window.Pos), ImGuiPopupPositionPolicy_Default); // Ideally we'd disable r_avoid here
    }
    if (window.Flags & ImGuiWindowFlags_Tooltip)
    {
        // Position tooltip (always follows mouse)
        let sc: c_float =  g.Style.MouseCursorScale;
        let ref_pos: ImVec2 = NavCalcPreferredRefPos();
        ImRect r_avoid;
        if (!g.NavDisableHighlight && g.NavDisableMouseHover && !(g.IO.ConfigFlags & ImGuiConfigFlags_NavEnableSetMousePos))
            r_avoid = ImRect(ref_pos.x - 16, ref_pos.y - 8, ref_pos.x + 16, ref_pos.y + 8);
        else
            r_avoid = ImRect(ref_pos.x - 16, ref_pos.y - 8, ref_pos.x + 24 * sc, ref_pos.y + 24 * sc); // FIXME: Hard-coded based on mouse cursor shape expectation. Exact dimension not very important.
        return FindBestWindowPosForPopupEx(ref_pos, window.Size, &window.AutoPosLastDirection, r_outer, r_avoid, ImGuiPopupPositionPolicy_Tooltip);
    }
    // IM_ASSERT(0);
    return window.Pos;
}

//-----------------------------------------------------------------------------
// [SECTION] KEYBOARD/GAMEPAD NAVIGATION
//-----------------------------------------------------------------------------

// FIXME-NAV: The existence of SetNavID vs SetFocusID vs FocusWindow() needs to be clarified/reworked.
// In our terminology those should be interchangeable, yet right now this is super confusing.
// Those two functions are merely a legacy artifact, so at minimum naming should be clarified.

c_void SetNavWindow(window: *mut ImGuiWindow)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.NavWindow != window)
    {
        IMGUI_DEBUG_LOG_FOCUS("[focus] SetNavWindow(\"%s\")\n", window ? window.Name : "<NULL>");
        g.NavWindow = window;
    }
    g.NavInitRequest = g.NavMoveSubmitted = g.NavMoveScoringItems = false;
    NavUpdateAnyRequestFlag();
}

c_void SetNavID(id: ImGuiID, ImGuiNavLayer nav_layer, ImGuiID focus_scope_id, rect_rel: &ImRect)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.NavWindow != NULL);
    // IM_ASSERT(nav_layer == ImGuiNavLayer_Main || nav_layer == ImGuiNavLayer_Menu);
    g.NavId = id;
    g.NavLayer = nav_layer;
    g.NavFocusScopeId = focus_scope_id;
    g.NavWindow.NavLastIds[nav_layer] = id;
    g.NavWindow.NavRectRel[nav_layer] = rect_rel;
}

c_void SetFocusID(id: ImGuiID, window: *mut ImGuiWindow)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(id != 0);

    if (g.NavWindow != window)
       SetNavWindow(window);

    // Assume that SetFocusID() is called in the context where its window.DC.NavLayerCurrent and window.DC.NavFocusScopeIdCurrent are valid.
    // Note that window may be != g.CurrentWindow (e.g. SetFocusID call in InputTextEx for multi-line text)
    const ImGuiNavLayer nav_layer = window.DC.NavLayerCurrent;
    g.NavId = id;
    g.NavLayer = nav_layer;
    g.NavFocusScopeId = window.DC.NavFocusScopeIdCurrent;
    window.NavLastIds[nav_layer] = id;
    if (g.LastItemData.ID == id)
        window.NavRectRel[nav_layer] = WindowRectAbsToRel(window, g.LastItemData.NavRect);

    if (g.ActiveIdSource == ImGuiInputSource_Nav)
        g.NavDisableMouseHover = true;
    else
        g.NavDisableHighlight = true;
}

ImGetDirQuadrantFromDelta: ImGuiDir(dx: c_float,dy: c_float)
{
    if (ImFabs(dx) > ImFabs(dy))
        return (dx > 0.0) ? ImGuiDir_Right : ImGuiDir_Left;
    return (dy > 0.0) ? ImGuiDir_Down : ImGuiDir_Up;
}

staticinline: c_float NavScoreItemDistInterval(a0: c_float,a1: c_float,b0: c_float,b1: c_float)
{
    if (a1 < b0)
        return a1 - b0;
    if (b1 < a0)
        return a0 - b1;
    return 0.0;
}

static c_void inline NavClampRectToVisibleAreaForMoveDir(move_dir: ImGuiDir, ImRect& r, clip_rect: &ImRect)
{
    if (move_dir == ImGuiDir_Left || move_dir == ImGuiDir_Right)
    {
        r.Min.y = ImClamp(r.Min.y, clip_rect.Min.y, clip_rect.Max.y);
        r.Max.y = ImClamp(r.Max.y, clip_rect.Min.y, clip_rect.Max.y);
    }
    else // FIXME: PageUp/PageDown are leaving move_dir == None
    {
        r.Min.x = ImClamp(r.Min.x, clip_rect.Min.x, clip_rect.Max.x);
        r.Max.x = ImClamp(r.Max.x, clip_rect.Min.x, clip_rect.Max.x);
    }
}

// Scoring function for gamepad/keyboard directional navigation. Based on https://gist.github.com/rygorous/6981057
static NavScoreItem: bool(ImGuiNavItemData* result)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if (g.NavLayer != window.DC.NavLayerCurrent)
        return false;

    // FIXME: Those are not good variables names
    let cand: ImRect =  g.LastItemData.NavRect;   // Current item nav rectangle
    let curr: ImRect =  g.NavScoringRect;   // Current modified source rect (NB: we've applied Max.x = Min.x in NavUpdate() to inhibit the effect of having varied item width)
    g.NavScoringDebugCount+= 1;

    // When entering through a NavFlattened border, we consider child window items as fully clipped for scoring
    if (window.ParentWindow == g.NavWindow)
    {
        // IM_ASSERT((window.Flags | g.Navwindow.Flags) & ImGuiWindowFlags_NavFlattened);
        if (!window.ClipRect.Overlaps(cand))
            return false;
        cand.ClipWithFull(window.ClipRect); // This allows the scored item to not overlap other candidates in the parent window
    }

    // We perform scoring on items bounding box clipped by the current clipping rectangle on the other axis (clipping on our movement axis would give us equal scores for all clipped items)
    // For example, this ensure that items in one column are not reached when moving vertically from items in another column.
    NavClampRectToVisibleAreaForMoveDir(g.NavMoveClipDir, cand, window.ClipRect);

    // Compute distance between boxes
    // FIXME-NAV: Introducing biases for vertical navigation, needs to be removed.
    let dbx: c_float =  NavScoreItemDistInterval(cand.Min.x, cand.Max.x, curr.Min.x, curr.Max.x);
    let dby: c_float =  NavScoreItemDistInterval(ImLerp(cand.Min.y, cand.Max.y, 0.20f32), ImLerp(cand.Min.y, cand.Max.y, 0.80f32), ImLerp(curr.Min.y, curr.Max.y, 0.20f32), ImLerp(curr.Min.y, curr.Max.y, 0.80f32)); // Scale down on Y to keep using box-distance for vertically touching items
    if (dby != 0.0 && dbx != 0.0)
        dbx = (dbx / 1000f32) + ((dbx > 0.0) ? 1.0 : -1.0);
    let dist_box: c_float =  ImFabs(dbx) + ImFabs(dby);

    // Compute distance between centers (this is off by a factor of 2, but we only compare center distances with each other so it doesn't matter)
    let dcx: c_float =  (cand.Min.x + cand.Max.x) - (curr.Min.x + curr.Max.x);
    let dcy: c_float =  (cand.Min.y + cand.Max.y) - (curr.Min.y + curr.Max.y);
    let dist_center: c_float =  ImFabs(dcx) + ImFabs(dcy); // L1 metric (need this for our connectedness guarantee)

    // Determine which quadrant of 'curr' our candidate item 'cand' lies in based on distance
    quadrant: ImGuiDir;
    let dax: c_float =  0.0, day = 0.0, dist_axial = 0.0;
    if (dbx != 0.0 || dby != 0.0)
    {
        // For non-overlapping boxes, use distance between boxes
        dax = dbx;
        day = dby;
        dist_axial = dist_box;
        quadrant = ImGetDirQuadrantFromDelta(dbx, dby);
    }
    else if (dcx != 0.0 || dcy != 0.0)
    {
        // For overlapping boxes with different centers, use distance between centers
        dax = dcx;
        day = dcy;
        dist_axial = dist_center;
        quadrant = ImGetDirQuadrantFromDelta(dcx, dcy);
    }
    else
    {
        // Degenerate case: two overlapping buttons with same center, break ties arbitrarily (note that LastItemId here is really the _previous_ item order, but it doesn't matter)
        quadrant = (g.LastItemData.ID < g.NavId) ? ImGuiDir_Left : ImGuiDir_Right;
    }

// #if IMGUI_DEBUG_NAV_SCORING
    buf: [c_char;128];
    if (IsMouseHoveringRect(cand.Min, cand.Max))
    {
        ImFormatString(buf, bu0f32.len(), "dbox (%.2f,%.2f->%.40f32)\ndcen (%.2f,%.2f->%.40f32)\nd (%.2f,%.2f->%.40f32)\nnav %c, quadrant %c", dbx, dby, dist_box, dcx, dcy, dist_center, dax, day, dist_axial, "WENS"[g.NavMoveDir], "WENS"[quadrant]);
        let mut  draw_list: *mut ImDrawList =  GetForegroundDrawList(window);
        draw_list.AddRect(curr.Min, curr.Max, IM_COL32(255,200,0,100));
        draw_list.AddRect(cand.Min, cand.Max, IM_COL32(255,255,0,200));
        draw_list.AddRectFilled(cand.Max - ImVec2::new(4, 4), cand.Max + CalcTextSize(bu0f32) + ImVec2::new(4, 4), IM_COL32(40,0,0,150));
        draw_list.AddText(cand.Max, !0U, bu0f32);
    }
    else if (g.IO.KeyCtrl) // Hold to preview score in matching quadrant. Press C to rotate.
    {
        if (quadrant == g.NavMoveDir)
        {
            ImFormatString(buf, bu0f32.len(), "%.0f/%.0f", dist_box, dist_center);
            let mut  draw_list: *mut ImDrawList =  GetForegroundDrawList(window);
            draw_list.AddRectFilled(cand.Min, cand.Max, IM_COL32(255, 0, 0, 200));
            draw_list.AddText(cand.Min, IM_COL32(255, 255, 255, 255), bu0f32);
        }
    }
// #endif

    // Is it in the quadrant we're interesting in moving to?
    let mut new_best: bool =  false;
    const move_dir: ImGuiDir = g.NavMoveDir;
    if (quadrant == move_dir)
    {
        // Does it beat the current best candidate?
        if (dist_box < result->DistBox)
        {
            result->DistBox = dist_box;
            result->DistCenter = dist_center;
            return true;
        }
        if (dist_box == result->DistBox)
        {
            // Try using distance between center points to break ties
            if (dist_center < result->DistCenter)
            {
                result->DistCenter = dist_center;
                new_best = true;
            }
            else if (dist_center == result->DistCenter)
            {
                // Still tied! we need to be extra-careful to make sure everything gets linked properly. We consistently break ties by symbolically moving "later" items
                // (with higher index) to the right/downwards by an infinitesimal amount since we the current "best" button already (so it must have a lower index),
                // this is fairly easy. This rule ensures that all buttons with dx==dy==0 will end up being linked in order of appearance along the x axis.
                if (((move_dir == ImGuiDir_Up || move_dir == ImGuiDir_Down) ? dby : dbx) < 0.0) // moving bj to the right/down decreases distance
                    new_best = true;
            }
        }
    }

    // Axial check: if 'curr' has no link at all in some direction and 'cand' lies roughly in that direction, add a tentative link. This will only be kept if no "real" matches
    // are found, so it only augments the graph produced by the above method using extra links. (important, since it doesn't guarantee strong connectedness)
    // This is just to avoid buttons having no links in a particular direction when there's a suitable neighbor. you get good graphs without this too.
    // 2017/09/29: FIXME: This now currently only enabled inside menu bars, ideally we'd disable it everywhere. Menus in particular need to catch failure. For general navigation it feels awkward.
    // Disabling it may lead to disconnected graphs when nodes are very spaced out on different axis. Perhaps consider offering this as an option?
    if (result->DistBox == f32::MAX && dist_axial < result->DistAxial)  // Check axial match
        if (g.NavLayer == ImGuiNavLayer_Menu && !(g.NavWindow.Flags & ImGuiWindowFlags_ChildMenu))
            if ((move_dir == ImGuiDir_Left && dax < 0.0) || (move_dir == ImGuiDir_Right && dax > 0.0) || (move_dir == ImGuiDir_Up && day < 0.0) || (move_dir == ImGuiDir_Down && day > 0.0))
            {
                result->DistAxial = dist_axial;
                new_best = true;
            }

    return new_best;
}

static c_void NavApplyItemToResult(ImGuiNavItemData* result)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    result->Window = window;
    result->ID = g.LastItemData.ID;
    result->FocusScopeId = window.DC.NavFocusScopeIdCurrent;
    result->InFlags = g.LastItemData.InFlags;
    result->RectRel = WindowRectAbsToRel(window, g.LastItemData.NavRect);
}

// Handle "scoring" of an item for a tabbing/focusing request initiated by NavUpdateCreateTabbingRequest().
// Note that SetKeyboardFocusHere() API calls are considered tabbing requests!
// - Case 1: no nav/active id:    set result to first eligible item, stop storing.
// - Case 2: tab forward:         on ref id set counter, on counter elapse store result
// - Case 3: tab forward wrap:    set result to first eligible item (preemptively), on ref id set counter, on next frame if counter hasn't elapsed store result. // FIXME-TABBING: Could be done as a next-frame forwarded request
// - Case 4: tab backward:        store all results, on ref id pick prev, stop storing
// - Case 5: tab backward wrap:   store all results, on ref id if no result keep storing until last // FIXME-TABBING: Could be done as next-frame forwarded requested
c_void NavProcessItemForTabbingRequest(id: ImGuiID)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    // Always store in NavMoveResultLocal (unlike directional request which uses NavMoveResultOther on sibling/flattened windows)
    ImGuiNavItemData* result = &g.NavMoveResultLocal;
    if (g.NavTabbingDir == 1)
    {
        // Tab Forward or SetKeyboardFocusHere() with >= 0
        if (g.NavTabbingResultFirst.ID == 0)
            NavApplyItemToResult(&g.NavTabbingResultFirst);
        if (--g.NavTabbingCounter == 0)
            NavMoveRequestResolveWithLastItem(result);
        else if (g.NavId == id)
            g.NavTabbingCounter = 1;
    }
    else if (g.NavTabbingDir == -1)
    {
        // Tab Backward
        if (g.NavId == id)
        {
            if (result->ID)
            {
                g.NavMoveScoringItems = false;
                NavUpdateAnyRequestFlag();
            }
        }
        else
        {
            NavApplyItemToResult(result);
        }
    }
    else if (g.NavTabbingDir == 0)
    {
        // Tab Init
        if (g.NavTabbingResultFirst.ID == 0)
            NavMoveRequestResolveWithLastItem(&g.NavTabbingResultFirst);
    }
}

NavMoveRequestButNoResultYet: bool()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.NavMoveScoringItems && g.NavMoveResultLocal.ID == 0 && g.NavMoveResultOther.ID == 0;
}

// FIXME: ScoringRect is not set
c_void NavMoveRequestSubmit(move_dir: ImGuiDir, clip_dir: ImGuiDir, ImGuiNavMoveFlags move_flags, ImGuiScrollFlags scroll_flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.NavWindow != NULL);

    if (move_flags & ImGuiNavMoveFlags_Tabbing)
        move_flags |= ImGuiNavMoveFlags_AllowCurrentNavId;

    g.NavMoveSubmitted = g.NavMoveScoringItems = true;
    g.NavMoveDir = move_dir;
    g.NavMoveDirForDebug = move_dir;
    g.NavMoveClipDir = clip_dir;
    g.NavMoveFlags = move_flags;
    g.NavMoveScrollFlags = scroll_flags;
    g.NavMoveForwardToNextFrame = false;
    g.NavMoveKeyMods = g.IO.KeyMods;
    g.NavMoveResultLocal.Clear();
    g.NavMoveResultLocalVisible.Clear();
    g.NavMoveResultOther.Clear();
    g.NavTabbingCounter = 0;
    g.NavTabbingResultFirst.Clear();
    NavUpdateAnyRequestFlag();
}

c_void NavMoveRequestResolveWithLastItem(ImGuiNavItemData* result)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NavMoveScoringItems = false; // Ensure request doesn't need more processing
    NavApplyItemToResult(result);
    NavUpdateAnyRequestFlag();
}

c_void NavMoveRequestCancel()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NavMoveSubmitted = g.NavMoveScoringItems = false;
    NavUpdateAnyRequestFlag();
}

// Forward will reuse the move request again on the next frame (generally with modifications done to it)
c_void NavMoveRequestForward(move_dir: ImGuiDir, clip_dir: ImGuiDir, ImGuiNavMoveFlags move_flags, ImGuiScrollFlags scroll_flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.NavMoveForwardToNextFrame == false);
    NavMoveRequestCancel();
    g.NavMoveForwardToNextFrame = true;
    g.NavMoveDir = move_dir;
    g.NavMoveClipDir = clip_dir;
    g.NavMoveFlags = move_flags | ImGuiNavMoveFlags_Forwarded;
    g.NavMoveScrollFlags = scroll_flags;
}

// Navigation wrap-around logic is delayed to the end of the frame because this operation is only valid after entire
// popup is assembled and in case of appended popups it is not clear which EndPopup() call is final.
c_void NavMoveRequestTryWrapping(window: *mut ImGuiWindow, ImGuiNavMoveFlags wrap_flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(wrap_flags != 0); // Call with _WrapX, _WrapY, _LoopX, _LoopY
    // In theory we should test for NavMoveRequestButNoResultYet() but there's no point doing it, NavEndFrame() will do the same test
    if (g.NavWindow == window && g.NavMoveScoringItems && g.NavLayer == ImGuiNavLayer_Main)
        g.NavMoveFlags |= wrap_flags;
}

// FIXME: This could be replaced by updating a frame number in each window when (window == NavWindow) and (NavLayer == 0).
// This way we could find the last focused window among our children. It would be much less confusing this way?
static c_void NavSaveLastChildNavWindowIntoParent(nav_window: *mut ImGuiWindow)
{
    let mut parent: *mut ImGuiWindow =  nav_window;
    while (parent && parent->RootWindow != parent && (parent->Flags & (ImGuiWindowFlags_Popup | ImGuiWindowFlags_ChildMenu)) == 0)
        parent = parent->ParentWindow;
    if (parent && parent != nav_window)
        parent->NavLastChildNavWindow = nav_window;
}

// Restore the last focused child.
// Call when we are expected to land on the Main Layer (0) after FocusWindow()
static NavRestoreLastChildNavWindow: *mut ImGuiWindow(window: *mut ImGuiWindow)
{
    if (window.NavLastChildNavWindow && window.NavLastChildNavwindow.WasActive)
        return window.NavLastChildNavWindow;
    if (window.DockNodeAsHost && window.DockNodeAsHost->TabBar)
        if (ImGuiTabItem* tab = TabBarFindMostRecentlySelectedTabForActiveWindow(window.DockNodeAsHost->TabBar))
            return tab->Window;
    return window;
}

c_void NavRestoreLayer(ImGuiNavLayer layer)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (layer == ImGuiNavLayer_Main)
    {
        let mut prev_nav_window: *mut ImGuiWindow =  g.NavWindow;
        g.NavWindow = NavRestoreLastChildNavWindow(g.NavWindow);    // FIXME-NAV: Should clear ongoing nav requests?
        if (prev_nav_window)
            IMGUI_DEBUG_LOG_FOCUS("[focus] NavRestoreLayer: from \"%s\" to SetNavWindow(\"%s\")\n", prev_nav_window.Name, g.NavWindow.Name);
    }
    let mut window: *mut ImGuiWindow =  g.NavWindow;
    if (window.NavLastIds[layer] != 0)
    {
        SetNavID(window.NavLastIds[layer], layer, 0, window.NavRectRel[layer]);
    }
    else
    {
        g.NavLayer = layer;
        NavInitWindow(window, true);
    }
}

c_void NavRestoreHighlightAfterMove()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NavDisableHighlight = false;
    g.NavDisableMouseHover = g.NavMousePosDirty = true;
}

static inline c_void NavUpdateAnyRequestFlag()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NavAnyRequest = g.NavMoveScoringItems || g.NavInitRequest || (IMGUI_DEBUG_NAV_SCORING && g.NavWindow != null_mut());
    if (g.NavAnyRequest)
        // IM_ASSERT(g.NavWindow != NULL);
}

// This needs to be called before we submit any widget (aka in or before Begin)
c_void NavInitWindow(window: *mut ImGuiWindow, force_reinit: bool)
{
    // FIXME: ChildWindow test here is wrong for docking
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(window == g.NavWindow);

    if (window.Flags & ImGuiWindowFlags_NoNavInputs)
    {
        g.NavId = g.NavFocusScopeId = 0;
        return;
    }

    let mut init_for_nav: bool =  false;
    if (window == window.RootWindow || (window.Flags & ImGuiWindowFlags_Popup) || (window.NavLastIds[0] == 0) || force_reinit)
        init_for_nav = true;
    IMGUI_DEBUG_LOG_NAV("[nav] NavInitRequest: from NavInitWindow(), init_for_nav=%d, window=\"%s\", layer=%d\n", init_for_nav, window.Name, g.NavLayer);
    if (init_for_nav)
    {
        SetNavID(0, g.NavLayer, 0, ImRect());
        g.NavInitRequest = true;
        g.NavInitRequestFromMove = false;
        g.NavInitResultId = 0;
        g.NavInitResultRectRel = ImRect();
        NavUpdateAnyRequestFlag();
    }
    else
    {
        g.NavId = window.NavLastIds[0];
        g.NavFocusScopeId = 0;
    }
}

static NavCalcPreferredRefPos: ImVec2()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window: *mut ImGuiWindow =  g.NavWindow;
    if (g.NavDisableHighlight || !g.NavDisableMouseHover || !window)
    {
        // Mouse (we need a fallback in case the mouse becomes invalid after being used)
        // The +1.0 offset when stored by OpenPopupEx() allows reopening this or another popup (same or another mouse button) while not moving the mouse, it is pretty standard.
        // In theory we could move that +1.0 offset in OpenPopupEx()
        let p: ImVec2 = IsMousePosValid(&g.IO.MousePos) ? g.IO.MousePos : g.MouseLastValidPos;
        return ImVec2::new(p.x + 1.0, p.y);
    }
    else
    {
        // When navigation is active and mouse is disabled, pick a position around the bottom left of the currently navigated item
        // Take account of upcoming scrolling (maybe set mouse pos should be done in EndFrame?)
        let rect_rel: ImRect =  WindowRectRelToAbs(window, window.NavRectRel[g.NavLayer]);
        if (window.LastFrameActive != g.FrameCount && (window.ScrollTarget.x != f32::MAX || window.ScrollTarget.y != f32::MAX))
        {
            let next_scroll: ImVec2 = CalcNextScrollFromScrollTargetAndClamp(window);
            rect_rel.Translate(window.Scroll - next_scroll);
        }
        let pos: ImVec2 = ImVec2::new(rect_rel.Min.x + ImMin(g.Style.FramePadding.x * 4, rect_rel.GetWidth()), rect_rel.Max.y - ImMin(g.Style.FramePadding.y, rect_rel.GetHeight()));
        ImGuiViewport* viewport = window.Viewport;
        return ImFloor(ImClamp(pos, viewport.Pos, viewport.Pos + viewport.Size)); // ImFloor() is important because non-integer mouse position application in backend might be lossy and result in undesirable non-zero delta.
    }
}GetNavTweakPressedAmount: c_float(ImGuiAxis axis)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;repeat_delay: c_float, repeat_rate;
    GetTypematicRepeatRate(ImGuiInputFlags_RepeatRateNavTweak, &repeat_delay, &repeat_rate);

    ImGuiKey key_less, key_more;
    if (g.NavInputSource == ImGuiInputSource_Gamepad)
    {
        key_less = (axis == ImGuiAxis_X) ? ImGuiKey_GamepadDpadLeft : ImGuiKey_GamepadDpadUp;
        key_more = (axis == ImGuiAxis_X) ? ImGuiKey_GamepadDpadRight : ImGuiKey_GamepadDpadDown;
    }
    else
    {
        key_less = (axis == ImGuiAxis_X) ? ImGuiKey_LeftArrow : ImGuiKey_UpArrow;
        key_more = (axis == ImGuiAxis_X) ? ImGuiKey_RightArrow : ImGuiKey_DownArrow;
    }
    let amount: c_float =  GetKeyPressedAmount(key_more, repeat_delay, repeat_rate) - GetKeyPressedAmount(key_less, repeat_delay, repeat_rate);
    if (amount != 0.0 && IsKeyDown(key_less) && IsKeyDown(key_more)) // Cancel when opposite directions are held, regardless of repeat phase
        amount = 0.0;
    return amount;
}

static c_void NavUpdate()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiIO& io = g.IO;

    io.WantSetMousePos = false;
    //if (g.NavScoringDebugCount > 0) IMGUI_DEBUG_LOG_NAV("[nav] NavScoringDebugCount %d for '%s' layer %d (Init:%d, Move:%d)\n", g.NavScoringDebugCount, g.NavWindow ? g.Navwindow.Name : "NULL", g.NavLayer, g.NavInitRequest || g.NavInitResultId != 0, g.NavMoveRequest);

    // Set input source based on which keys are last pressed (as some features differs when used with Gamepad vs Keyboard)
    // FIXME-NAV: Now that keys are separated maybe we can get rid of NavInputSource?
    let nav_gamepad_active: bool = (io.ConfigFlags & ImGuiConfigFlags_NavEnableGamepad) != 0 && (io.BackendFlags & ImGuiBackendFlags_HasGamepad) != 0;
    const ImGuiKey nav_gamepad_keys_to_change_source[] = { ImGuiKey_GamepadFaceRight, ImGuiKey_GamepadFaceLeft, ImGuiKey_GamepadFaceUp, ImGuiKey_GamepadFaceDown, ImGuiKey_GamepadDpadRight, ImGuiKey_GamepadDpadLeft, ImGuiKey_GamepadDpadUp, ImGuiKey_GamepadDpadDown };
    if (nav_gamepad_active)
        for (ImGuiKey key : nav_gamepad_keys_to_change_source)
            if (IsKeyDown(key))
                g.NavInputSource = ImGuiInputSource_Gamepad;
    let nav_keyboard_active: bool = (io.ConfigFlags & ImGuiConfigFlags_NavEnableKeyboard) != 0;
    const ImGuiKey nav_keyboard_keys_to_change_source[] = { ImGuiKey_Space, ImGuiKey_Enter, ImGuiKey_Escape, ImGuiKey_RightArrow, ImGuiKey_LeftArrow, ImGuiKey_UpArrow, ImGuiKey_DownArrow };
    if (nav_keyboard_active)
        for (ImGuiKey key : nav_keyboard_keys_to_change_source)
            if (IsKeyDown(key))
                g.NavInputSource = ImGuiInputSource_Keyboard;

    // Process navigation init request (select first/default focus)
    if (g.NavInitResultId != 0)
        NavInitRequestApplyResult();
    g.NavInitRequest = false;
    g.NavInitRequestFromMove = false;
    g.NavInitResultId = 0;
    g.NavJustMovedToId = 0;

    // Process navigation move request
    if (g.NavMoveSubmitted)
        NavMoveRequestApplyResult();
    g.NavTabbingCounter = 0;
    g.NavMoveSubmitted = g.NavMoveScoringItems = false;

    // Schedule mouse position update (will be done at the bottom of this function, after 1) processing all move requests and 2) updating scrolling)
    let mut set_mouse_pos: bool =  false;
    if (g.NavMousePosDirty && g.NavIdIsAlive)
        if (!g.NavDisableHighlight && g.NavDisableMouseHover && g.NavWindow)
            set_mouse_pos = true;
    g.NavMousePosDirty = false;
    // IM_ASSERT(g.NavLayer == ImGuiNavLayer_Main || g.NavLayer == ImGuiNavLayer_Menu);

    // Store our return window (for returning from Menu Layer to Main Layer) and clear it as soon as we step back in our own Layer 0
    if (g.NavWindow)
        NavSaveLastChildNavWindowIntoParent(g.NavWindow);
    if (g.NavWindow && g.NavWindow.NavLastChildNavWindow != null_mut() && g.NavLayer == ImGuiNavLayer_Main)
        g.NavWindow.NavLastChildNavWindow= null_mut();

    // Update CTRL+TAB and Windowing features (hold Square to move/resize/etc.)
    NavUpdateWindowing();

    // Set output flags for user application
    io.NavActive = (nav_keyboard_active || nav_gamepad_active) && g.NavWindow && !(g.NavWindow.Flags & ImGuiWindowFlags_NoNavInputs);
    io.NavVisible = (io.NavActive && g.NavId != 0 && !g.NavDisableHighlight) || (g.NavWindowingTarget != null_mut());

    // Process NavCancel input (to close a popup, get back to parent, clear focus)
    NavUpdateCancelRequest();

    // Process manual activation request
    g.NavActivateId = g.NavActivateDownId = g.NavActivatePressedId = g.NavActivateInputId = 0;
    g.NavActivateFlags = ImGuiActivateFlags_None;
    if (g.NavId != 0 && !g.NavDisableHighlight && !g.NavWindowingTarget && g.NavWindow && !(g.NavWindow.Flags & ImGuiWindowFlags_NoNavInputs))
    {
        let activate_down: bool = (nav_keyboard_active && IsKeyDown(ImGuiKey_Space)) || (nav_gamepad_active && IsKeyDown(ImGuiKey_NavGamepadActivate));
        let activate_pressed: bool = activate_down && ((nav_keyboard_active && IsKeyPressed(ImGuiKey_Space, false)) || (nav_gamepad_active && IsKeyPressed(ImGuiKey_NavGamepadActivate, false)));
        let input_down: bool = (nav_keyboard_active && IsKeyDown(ImGuiKey_Enter)) || (nav_gamepad_active && IsKeyDown(ImGuiKey_NavGamepadInput));
        let input_pressed: bool = input_down && ((nav_keyboard_active && IsKeyPressed(ImGuiKey_Enter, false)) || (nav_gamepad_active && IsKeyPressed(ImGuiKey_NavGamepadInput, false)));
        if (g.ActiveId == 0 && activate_pressed)
        {
            g.NavActivateId = g.NavId;
            g.NavActivateFlags = ImGuiActivateFlags_PreferTweak;
        }
        if ((g.ActiveId == 0 || g.ActiveId == g.NavId) && input_pressed)
        {
            g.NavActivateInputId = g.NavId;
            g.NavActivateFlags = ImGuiActivateFlags_PreferInput;
        }
        if ((g.ActiveId == 0 || g.ActiveId == g.NavId) && activate_down)
            g.NavActivateDownId = g.NavId;
        if ((g.ActiveId == 0 || g.ActiveId == g.NavId) && activate_pressed)
            g.NavActivatePressedId = g.NavId;
    }
    if (g.NavWindow && (g.NavWindow.Flags & ImGuiWindowFlags_NoNavInputs))
        g.NavDisableHighlight = true;
    if (g.NavActivateId != 0)
        // IM_ASSERT(g.NavActivateDownId == g.NavActivateId);

    // Process programmatic activation request
    // FIXME-NAV: Those should eventually be queued (unlike focus they don't cancel each others)
    if (g.NavNextActivateId != 0)
    {
        if (g.NavNextActivateFlags & ImGuiActivateFlags_PreferInput)
            g.NavActivateInputId = g.NavNextActivateId;
        else
            g.NavActivateId = g.NavActivateDownId = g.NavActivatePressedId = g.NavNextActivateId;
        g.NavActivateFlags = g.NavNextActivateFlags;
    }
    g.NavNextActivateId = 0;

    // Process move requests
    NavUpdateCreateMoveRequest();
    if (g.NavMoveDir == ImGuiDir_None)
        NavUpdateCreateTabbingRequest();
    NavUpdateAnyRequestFlag();
    g.NavIdIsAlive = false;

    // Scrolling
    if (g.NavWindow && !(g.NavWindow.Flags & ImGuiWindowFlags_NoNavInputs) && !g.NavWindowingTarget)
    {
        // *Fallback* manual-scroll with Nav directional keys when window has no navigable item
        let mut window: *mut ImGuiWindow =  g.NavWindow;
        let scroll_speed: c_float =  IM_ROUND(window.CalcFontSize() * 100 * io.DeltaTime); // We need round the scrolling speed because sub-pixel scroll isn't reliably supported.
        const move_dir: ImGuiDir = g.NavMoveDir;
        if (window.DC.NavLayersActiveMask == 0x00 && window.DC.NavHasScroll && move_dir != ImGuiDir_None)
        {
            if (move_dir == ImGuiDir_Left || move_dir == ImGuiDir_Right)
                SetScrollX(window, ImFloor(window.Scroll.x + ((move_dir == ImGuiDir_Left) ? -1.0 : 1.0) * scroll_speed));
            if (move_dir == ImGuiDir_Up || move_dir == ImGuiDir_Down)
                SetScrollY(window, ImFloor(window.Scroll.y + ((move_dir == ImGuiDir_Up) ? -1.0 : 1.0) * scroll_speed));
        }

        // *Normal* Manual scroll with LStick
        // Next movement request will clamp the NavId reference rectangle to the visible area, so navigation will resume within those bounds.
        if (nav_gamepad_active)
        {
            let scroll_dir: ImVec2 = GetKeyVector2d(ImGuiKey_GamepadLStickLeft, ImGuiKey_GamepadLStickRight, ImGuiKey_GamepadLStickUp, ImGuiKey_GamepadLStickDown);
            let tweak_factor: c_float =  IsKeyDown(ImGuiKey_NavGamepadTweakSlow) ? 1.0 / 10f32 : IsKeyDown(ImGuiKey_NavGamepadTweakFast) ? 10f32 : 1.0;
            if (scroll_dir.x != 0.0 && window.ScrollbarX)
                SetScrollX(window, ImFloor(window.Scroll.x + scroll_dir.x * scroll_speed * tweak_factor));
            if (scroll_dir.y != 0.0)
                SetScrollY(window, ImFloor(window.Scroll.y + scroll_dir.y * scroll_speed * tweak_factor));
        }
    }

    // Always prioritize mouse highlight if navigation is disabled
    if (!nav_keyboard_active && !nav_gamepad_active)
    {
        g.NavDisableHighlight = true;
        g.NavDisableMouseHover = set_mouse_pos = false;
    }

    // Update mouse position if requested
    // (This will take into account the possibility that a Scroll was queued in the window to offset our absolute mouse position before scroll has been applied)
    if (set_mouse_pos && (io.ConfigFlags & ImGuiConfigFlags_NavEnableSetMousePos) && (io.BackendFlags & ImGuiBackendFlags_HasSetMousePos))
    {
        io.MousePos = io.MousePosPrev = NavCalcPreferredRefPos();
        io.WantSetMousePos = true;
        //IMGUI_DEBUG_LOG_IO("SetMousePos: (%.1f,%.10f32)\n", io.MousePos.x, io.MousePos.y);
    }

    // [DEBUG]
    g.NavScoringDebugCount = 0;
// #if IMGUI_DEBUG_NAV_RECTS
    if (g.NavWindow)
    {
        let mut  draw_list: *mut ImDrawList =  GetForegroundDrawList(g.NavWindow);
        if (1) { for (let layer: c_int = 0; layer < 2; layer++) { let r: ImRect =  WindowRectRelToAbs(g.NavWindow, g.NavWindow.NavRectRel[layer]); draw_list.AddRect(r.Min, r.Max, IM_COL32(255,200,0,255)); } } // [DEBUG]
        if (1) { col: u32 = (!g.NavWindow.Hidden) ? IM_COL32(255,0,255,255) : IM_COL32(255,0,0,255); let mut p: ImVec2 =  NavCalcPreferredRefPos(); buf: [c_char;32]; ImFormatString(buf, 32, "%d", g.NavLayer); draw_list.AddCircleFilled(p, 3.0.0, col); draw_list.AddText(null_mut(), 13.0.0, p + ImVec2::new(8,-4), col, bu0f32); }
    }
// #endif
}

c_void NavInitRequestApplyResult()
{
    // In very rare cases g.NavWindow may be null (e.g. clearing focus after requesting an init request, which does happen when releasing Alt while clicking on void)
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (!g.NavWindow)
        return;

    // Apply result from previous navigation init request (will typically select the first item, unless SetItemDefaultFocus() has been called)
    // FIXME-NAV: On _NavFlattened windows, g.NavWindow will only be updated during subsequent frame. Not a problem currently.
    IMGUI_DEBUG_LOG_NAV("[nav] NavInitRequest: ApplyResult: NavID 0x%08X in Layer %d Window \"%s\"\n", g.NavInitResultId, g.NavLayer, g.NavWindow.Name);
    SetNavID(g.NavInitResultId, g.NavLayer, 0, g.NavInitResultRectRel);
    g.NavIdIsAlive = true; // Mark as alive from previous frame as we got a result
    if (g.NavInitRequestFromMove)
        NavRestoreHighlightAfterMove();
}

c_void NavUpdateCreateMoveRequest()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiIO& io = g.IO;
    let mut window: *mut ImGuiWindow =  g.NavWindow;
    let nav_gamepad_active: bool = (io.ConfigFlags & ImGuiConfigFlags_NavEnableGamepad) != 0 && (io.BackendFlags & ImGuiBackendFlags_HasGamepad) != 0;
    let nav_keyboard_active: bool = (io.ConfigFlags & ImGuiConfigFlags_NavEnableKeyboard) != 0;

    if (g.NavMoveForwardToNextFrame && window != null_mut())
    {
        // Forwarding previous request (which has been modified, e.g. wrap around menus rewrite the requests with a starting rectangle at the other side of the window)
        // (preserve most state, which were already set by the NavMoveRequestForward() function)
        // IM_ASSERT(g.NavMoveDir != ImGuiDir_None && g.NavMoveClipDir != ImGuiDir_None);
        // IM_ASSERT(g.NavMoveFlags & ImGuiNavMoveFlags_Forwarded);
        IMGUI_DEBUG_LOG_NAV("[nav] NavMoveRequestForward %d\n", g.NavMoveDir);
    }
    else
    {
        // Initiate directional inputs request
        g.NavMoveDir = ImGuiDir_None;
        g.NavMoveFlags = ImGuiNavMoveFlags_None;
        g.NavMoveScrollFlags = ImGuiScrollFlags_None;
        if (window && !g.NavWindowingTarget && !(window.Flags & ImGuiWindowFlags_NoNavInputs))
        {
            const ImGuiInputFlags repeat_mode = ImGuiInputFlags_Repeat | ImGuiInputFlags_RepeatRateNavMove;
            if (!IsActiveIdUsingNavDir(ImGuiDir_Left)  && ((nav_gamepad_active && IsKeyPressedEx(ImGuiKey_GamepadDpadLeft,  repeat_mode)) || (nav_keyboard_active && IsKeyPressedEx(ImGuiKey_LeftArrow,  repeat_mode)))) { g.NavMoveDir = ImGuiDir_Left; }
            if (!IsActiveIdUsingNavDir(ImGuiDir_Right) && ((nav_gamepad_active && IsKeyPressedEx(ImGuiKey_GamepadDpadRight, repeat_mode)) || (nav_keyboard_active && IsKeyPressedEx(ImGuiKey_RightArrow, repeat_mode)))) { g.NavMoveDir = ImGuiDir_Right; }
            if (!IsActiveIdUsingNavDir(ImGuiDir_Up)    && ((nav_gamepad_active && IsKeyPressedEx(ImGuiKey_GamepadDpadUp,    repeat_mode)) || (nav_keyboard_active && IsKeyPressedEx(ImGuiKey_UpArrow,    repeat_mode)))) { g.NavMoveDir = ImGuiDir_Up; }
            if (!IsActiveIdUsingNavDir(ImGuiDir_Down)  && ((nav_gamepad_active && IsKeyPressedEx(ImGuiKey_GamepadDpadDown,  repeat_mode)) || (nav_keyboard_active && IsKeyPressedEx(ImGuiKey_DownArrow,  repeat_mode)))) { g.NavMoveDir = ImGuiDir_Down; }
        }
        g.NavMoveClipDir = g.NavMoveDir;
        g.NavScoringNoClipRect = ImRect(f32::MAX, f32::MAX, -f32::MAX, -f32::MAX);
    }

    // Update PageUp/PageDown/Home/End scroll
    // FIXME-NAV: Consider enabling those keys even without the master ImGuiConfigFlags_NavEnableKeyboard flag?
    let scoring_rect_offset_y: c_float =  0.0;
    if (window && g.NavMoveDir == ImGuiDir_None && nav_keyboard_active)
        scoring_rect_offset_y = NavUpdatePageUpPageDown();
    if (scoring_rect_offset_y != 0.0)
    {
        g.NavScoringNoClipRect = window.InnerRect;
        g.NavScoringNoClipRect.TranslateY(scoring_rect_offset_y);
    }

    // [DEBUG] Always send a request
// #if IMGUI_DEBUG_NAV_SCORING
    if (io.KeyCtrl && IsKeyPressed(ImGuiKey_C))
        g.NavMoveDirForDebug = (ImGuiDir)((g.NavMoveDirForDebug + 1) & 3);
    if (io.KeyCtrl && g.NavMoveDir == ImGuiDir_None)
    {
        g.NavMoveDir = g.NavMoveDirForDebug;
        g.NavMoveFlags |= ImGuiNavMoveFlags_DebugNoResult;
    }
// #endif

    // Submit
    g.NavMoveForwardToNextFrame = false;
    if (g.NavMoveDir != ImGuiDir_None)
        NavMoveRequestSubmit(g.NavMoveDir, g.NavMoveClipDir, g.NavMoveFlags, g.NavMoveScrollFlags);

    // Moving with no reference triggers a init request (will be used as a fallback if the direction fails to find a match)
    if (g.NavMoveSubmitted && g.NavId == 0)
    {
        IMGUI_DEBUG_LOG_NAV("[nav] NavInitRequest: from move, window \"%s\", layer=%d\n", window ? window.Name : "<NULL>", g.NavLayer);
        g.NavInitRequest = g.NavInitRequestFromMove = true;
        g.NavInitResultId = 0;
        g.NavDisableHighlight = false;
    }

    // When using gamepad, we project the reference nav bounding box into window visible area.
    // This is to allow resuming navigation inside the visible area after doing a large amount of scrolling, since with gamepad every movements are relative
    // (can't focus a visible object like we can with the mouse).
    if (g.NavMoveSubmitted && g.NavInputSource == ImGuiInputSource_Gamepad && g.NavLayer == ImGuiNavLayer_Main && window != null_mut())// && (g.NavMoveFlags & ImGuiNavMoveFlags_Forwarded))
    {
        let mut clamp_x: bool =  (g.NavMoveFlags & (ImGuiNavMoveFlags_LoopX | ImGuiNavMoveFlags_WrapX)) == 0;
        let mut clamp_y: bool =  (g.NavMoveFlags & (ImGuiNavMoveFlags_LoopY | ImGuiNavMoveFlags_WrapY)) == 0;
        let inner_rect_rel: ImRect =  WindowRectAbsToRel(window, ImRect(window.InnerRect.Min - ImVec2::new(1, 1), window.InnerRect.Max + ImVec2::new(1, 1)));
        if ((clamp_x || clamp_y) && !inner_rect_rel.Contains(window.NavRectRel[g.NavLayer]))
        {
            //IMGUI_DEBUG_LOG_NAV("[nav] NavMoveRequest: clamp NavRectRel for gamepad move\n");
            let pad_x: c_float =  ImMin(inner_rect_rel.GetWidth(), window.CalcFontSize() * 0.5f32);
            let pad_y: c_float =  ImMin(inner_rect_rel.GetHeight(), window.CalcFontSize() * 0.5f32); // Terrible approximation for the intent of starting navigation from first fully visible item
            inner_rect_rel.Min.x = clamp_x ? (inner_rect_rel.Min.x + pad_x) : -f32::MAX;
            inner_rect_rel.Max.x = clamp_x ? (inner_rect_rel.Max.x - pad_x) : f32::MAX;
            inner_rect_rel.Min.y = clamp_y ? (inner_rect_rel.Min.y + pad_y) : -f32::MAX;
            inner_rect_rel.Max.y = clamp_y ? (inner_rect_rel.Max.y - pad_y) : f32::MAX;
            window.NavRectRel[g.NavLayer].ClipWithFull(inner_rect_rel);
            g.NavId = g.NavFocusScopeId = 0;
        }
    }

    // For scoring we use a single segment on the left side our current item bounding box (not touching the edge to avoid box overlap with zero-spaced items)
    ImRect scoring_rect;
    if (window != null_mut())
    {
        let nav_rect_rel: ImRect =  !window.NavRectRel[g.NavLayer].IsInverted() ? window.NavRectRel[g.NavLayer] : ImRect(0, 0, 0, 0);
        scoring_rect = WindowRectRelToAbs(window, nav_rect_rel);
        scoring_rect.TranslateY(scoring_rect_offset_y);
        scoring_rect.Min.x = ImMin(scoring_rect.Min.x + 1.0, scoring_rect.Max.x);
        scoring_rect.Max.x = scoring_rect.Min.x;
        // IM_ASSERT(!scoring_rect.IsInverted()); // Ensure if we have a finite, non-inverted bounding box here will allows us to remove extraneous ImFabs() calls in NavScoreItem().
        //GetForegroundDrawList()->AddRect(scoring_rect.Min, scoring_rect.Max, IM_COL32(255,200,0,255)); // [DEBUG]
        //if (!g.NavScoringNoClipRect.IsInverted()) { GetForegroundDrawList()->AddRect(g.NavScoringNoClipRect.Min, g.NavScoringNoClipRect.Max, IM_COL32(255, 200, 0, 255)); } // [DEBUG]
    }
    g.NavScoringRect = scoring_rect;
    g.NavScoringNoClipRect.Add(scoring_rect);
}

c_void NavUpdateCreateTabbingRequest()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window: *mut ImGuiWindow =  g.NavWindow;
    // IM_ASSERT(g.NavMoveDir == ImGuiDir_None);
    if (window == null_mut() || g.NavWindowingTarget != null_mut() || (window.Flags & ImGuiWindowFlags_NoNavInputs))
        return;

    let tab_pressed: bool = IsKeyPressed(ImGuiKey_Tab, true) && !IsActiveIdUsingKey(ImGuiKey_Tab) && !g.IO.KeyCtrl && !g.IO.KeyAlt;
    if (!tab_pressed)
        return;

    // Initiate tabbing request
    // (this is ALWAYS ENABLED, regardless of ImGuiConfigFlags_NavEnableKeyboard flag!)
    // Initially this was designed to use counters and modulo arithmetic, but that could not work with unsubmitted items (list clipper). Instead we use a strategy close to other move requests.
    // See NavProcessItemForTabbingRequest() for a description of the various forward/backward tabbing cases with and without wrapping.
    //// FIXME: We use (g.ActiveId == 0) but (g.NavDisableHighlight == false) might be righter once we can tab through anything
    g.NavTabbingDir = g.IO.KeyShift ? -1 : (g.ActiveId == 0) ? 0 : 1;
    ImGuiScrollFlags scroll_flags = window.Appearing ? ImGuiScrollFlags_KeepVisibleEdgeX | ImGuiScrollFlags_AlwaysCenterY : ImGuiScrollFlags_KeepVisibleEdgeX | ImGuiScrollFlags_KeepVisibleEdgeY;
    clip_dir: ImGuiDir = (g.NavTabbingDir < 0) ? ImGuiDir_Up : ImGuiDir_Down;
    NavMoveRequestSubmit(ImGuiDir_None, clip_dir, ImGuiNavMoveFlags_Tabbing, scroll_flags); // FIXME-NAV: Once we refactor tabbing, add LegacyApi flag to not activate non-inputable.
    g.NavTabbingCounter = -1;
}

// Apply result from previous frame navigation directional move request. Always called from NavUpdate()
c_void NavMoveRequestApplyResult()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
// #if IMGUI_DEBUG_NAV_SCORING
    if (g.NavMoveFlags & ImGuiNavMoveFlags_DebugNoResult) // [DEBUG] Scoring all items in NavWindow at all times
        return;
// #endif

    // Select which result to use
    ImGuiNavItemData* result = (g.NavMoveResultLocal.ID != 0) ? &g.NavMoveResultLocal : (g.NavMoveResultOther.ID != 0) ? &g.NavMoveResultOther : null_mut();

    // Tabbing forward wrap
    if (g.NavMoveFlags & ImGuiNavMoveFlags_Tabbing)
        if ((g.NavTabbingCounter == 1 || g.NavTabbingDir == 0) && g.NavTabbingResultFirst.ID)
            result = &g.NavTabbingResultFirst;

    // In a situation when there is no results but NavId != 0, re-enable the Navigation highlight (because g.NavId is not considered as a possible result)
    if (result == null_mut())
    {
        if (g.NavMoveFlags & ImGuiNavMoveFlags_Tabbing)
            g.NavMoveFlags |= ImGuiNavMoveFlags_DontSetNavHighlight;
        if (g.NavId != 0 && (g.NavMoveFlags & ImGuiNavMoveFlags_DontSetNavHighlight) == 0)
            NavRestoreHighlightAfterMove();
        return;
    }

    // PageUp/PageDown behavior first jumps to the bottom/top mostly visible item, _otherwise_ use the result from the previous/next page.
    if (g.NavMoveFlags & ImGuiNavMoveFlags_AlsoScoreVisibleSet)
        if (g.NavMoveResultLocalVisible.ID != 0 && g.NavMoveResultLocalVisible.ID != g.NavId)
            result = &g.NavMoveResultLocalVisible;

    // Maybe entering a flattened child from the outside? In this case solve the tie using the regular scoring rules.
    if (result != &g.NavMoveResultOther && g.NavMoveResultOther.ID != 0 && g.NavMoveResultOther.window.ParentWindow == g.NavWindow)
        if ((g.NavMoveResultOther.DistBox < result->DistBox) || (g.NavMoveResultOther.DistBox == result->DistBox && g.NavMoveResultOther.DistCenter < result->DistCenter))
            result = &g.NavMoveResultOther;
    // IM_ASSERT(g.NavWindow && result->Window);

    // Scroll to keep newly navigated item fully into view.
    if (g.NavLayer == ImGuiNavLayer_Main)
    {
        if (g.NavMoveFlags & ImGuiNavMoveFlags_ScrollToEdgeY)
        {
            // FIXME: Should remove this
            let scroll_target: c_float =  (g.NavMoveDir == ImGuiDir_Up) ? result->window.ScrollMax.y : 0.0;
            SetScrollY(result->Window, scroll_target);
        }
        else
        {
            let rect_abs: ImRect =  WindowRectRelToAbs(result->Window, result->RectRel);
            ScrollToRectEx(result->Window, rect_abs, g.NavMoveScrollFlags);
        }
    }

    if (g.NavWindow != result->Window)
    {
        IMGUI_DEBUG_LOG_FOCUS("[focus] NavMoveRequest: SetNavWindow(\"%s\")\n", result->window.Name);
        g.NavWindow = result->Window;
    }
    if (g.ActiveId != result->ID)
        ClearActiveID();
    if (g.NavId != result->ID)
    {
        // Don't set NavJustMovedToId if just landed on the same spot (which may happen with ImGuiNavMoveFlags_AllowCurrentNavId)
        g.NavJustMovedToId = result->ID;
        g.NavJustMovedToFocusScopeId = result->FocusScopeId;
        g.NavJustMovedToKeyMods = g.NavMoveKeyMods;
    }

    // Focus
    IMGUI_DEBUG_LOG_NAV("[nav] NavMoveRequest: result NavID 0x%08X in Layer %d Window \"%s\"\n", result->ID, g.NavLayer, g.NavWindow.Name);
    SetNavID(result->ID, g.NavLayer, result->FocusScopeId, result->RectRel);

    // Tabbing: Activates Inputable or Focus non-Inputable
    if ((g.NavMoveFlags & ImGuiNavMoveFlags_Tabbing) && (result->InFlags & ImGuiItemFlags_Inputable))
    {
        g.NavNextActivateId = result->ID;
        g.NavNextActivateFlags = ImGuiActivateFlags_PreferInput | ImGuiActivateFlags_TryToPreserveState;
        g.NavMoveFlags |= ImGuiNavMoveFlags_DontSetNavHighlight;
    }

    // Activate
    if (g.NavMoveFlags & ImGuiNavMoveFlags_Activate)
    {
        g.NavNextActivateId = result->ID;
        g.NavNextActivateFlags = ImGuiActivateFlags_None;
    }

    // Enable nav highlight
    if ((g.NavMoveFlags & ImGuiNavMoveFlags_DontSetNavHighlight) == 0)
        NavRestoreHighlightAfterMove();
}

// Process NavCancel input (to close a popup, get back to parent, clear focus)
// FIXME: In order to support e.g. Escape to clear a selection we'll need:
// - either to store the equivalent of ActiveIdUsingKeyInputMask for a FocusScope and test for it.
// - either to move most/all of those tests to the epilogue/end functions of the scope they are dealing with (e.g. exit child window in EndChild()) or in EndFrame(), to allow an earlier intercept
static c_void NavUpdateCancelRequest()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let nav_gamepad_active: bool = (g.IO.ConfigFlags & ImGuiConfigFlags_NavEnableGamepad) != 0 && (g.IO.BackendFlags & ImGuiBackendFlags_HasGamepad) != 0;
    let nav_keyboard_active: bool = (g.IO.ConfigFlags & ImGuiConfigFlags_NavEnableKeyboard) != 0;
    if (!(nav_keyboard_active && IsKeyPressed(ImGuiKey_Escape, false)) && !(nav_gamepad_active && IsKeyPressed(ImGuiKey_NavGamepadCancel, false)))
        return;

    IMGUI_DEBUG_LOG_NAV("[nav] NavUpdateCancelRequest()\n");
    if (g.ActiveId != 0)
    {
        if (!IsActiveIdUsingKey(ImGuiKey_Escape) && !IsActiveIdUsingKey(ImGuiKey_NavGamepadCancel))
            ClearActiveID();
    }
    else if (g.NavLayer != ImGuiNavLayer_Main)
    {
        // Leave the "menu" layer
        NavRestoreLayer(ImGuiNavLayer_Main);
        NavRestoreHighlightAfterMove();
    }
    else if (g.NavWindow && g.NavWindow != g.NavWindow.RootWindow && !(g.NavWindow.Flags & ImGuiWindowFlags_Popup) && g.NavWindow.ParentWindow)
    {
        // Exit child window
        let mut child_window: *mut ImGuiWindow =  g.NavWindow;
        let mut parent_window: *mut ImGuiWindow =  g.NavWindow.ParentWindow;
        // IM_ASSERT(child_window.ChildId != 0);
        let child_rect: ImRect =  child_window.Rect();
        FocusWindow(parent_window);
        SetNavID(child_window.ChildId, ImGuiNavLayer_Main, 0, WindowRectAbsToRel(parent_window, child_rect));
        NavRestoreHighlightAfterMove();
    }
    else if (g.OpenPopupStack.Size > 0 && !(g.OpenPopupStack.last().unwrap().window.Flags & ImGuiWindowFlags_Modal))
    {
        // Close open popup/menu
        ClosePopupToLevel(g.OpenPopupStack.Size - 1, true);
    }
    else
    {
        // Clear NavLastId for popups but keep it for regular child window so we can leave one and come back where we were
        if (g.NavWindow && ((g.NavWindow.Flags & ImGuiWindowFlags_Popup) || !(g.NavWindow.Flags & ImGuiWindowFlags_ChildWindow)))
            g.NavWindow.NavLastIds[0] = 0;
        g.NavId = g.NavFocusScopeId = 0;
    }
}

// Handle PageUp/PageDown/Home/End keys
// Called from NavUpdateCreateMoveRequest() which will use our output to create a move request
// FIXME-NAV: This doesn't work properly with NavFlattened siblings as we use NavWindow rectangle for reference
// FIXME-NAV: how to get Home/End to aim at the beginning/end of a 2D grid?
staticNavUpdatePageUpPageDown: c_float()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window: *mut ImGuiWindow =  g.NavWindow;
    if ((window.Flags & ImGuiWindowFlags_NoNavInputs) || g.NavWindowingTarget != null_mut())
        return 0.0;

    let page_up_held: bool = IsKeyDown(ImGuiKey_PageUp) && !IsActiveIdUsingKey(ImGuiKey_PageUp);
    let page_down_held: bool = IsKeyDown(ImGuiKey_PageDown) && !IsActiveIdUsingKey(ImGuiKey_PageDown);
    let home_pressed: bool = IsKeyPressed(ImGuiKey_Home) && !IsActiveIdUsingKey(ImGuiKey_Home);
    let end_pressed: bool = IsKeyPressed(ImGuiKey_End) && !IsActiveIdUsingKey(ImGuiKey_End);
    if (page_up_held == page_down_held && home_pressed == end_pressed) // Proceed if either (not both) are pressed, otherwise early out
        return 0.0;

    if (g.NavLayer != ImGuiNavLayer_Main)
        NavRestoreLayer(ImGuiNavLayer_Main);

    if (window.DC.NavLayersActiveMask == 0x00 && window.DC.NavHasScroll)
    {
        // Fallback manual-scroll when window has no navigable item
        if (IsKeyPressed(ImGuiKey_PageUp, true))
            SetScrollY(window, window.Scroll.y - window.InnerRect.GetHeight());
        else if (IsKeyPressed(ImGuiKey_PageDown, true))
            SetScrollY(window, window.Scroll.y + window.InnerRect.GetHeight());
        else if (home_pressed)
            SetScrollY(window, 0.0);
        else if (end_pressed)
            SetScrollY(window, window.ScrollMax.y);
    }
    else
    {
        ImRect& nav_rect_rel = window.NavRectRel[g.NavLayer];
        let page_offset_y: c_float =  ImMax(0.0, window.InnerRect.GetHeight() - window.CalcFontSize() * 1.0 + nav_rect_rel.GetHeight());
        let nav_scoring_rect_offset_y: c_float =  0.0;
        if (IsKeyPressed(ImGuiKey_PageUp, true))
        {
            nav_scoring_rect_offset_y = -page_offset_y;
            g.NavMoveDir = ImGuiDir_Down; // Because our scoring rect is offset up, we request the down direction (so we can always land on the last item)
            g.NavMoveClipDir = ImGuiDir_Up;
            g.NavMoveFlags = ImGuiNavMoveFlags_AllowCurrentNavId | ImGuiNavMoveFlags_AlsoScoreVisibleSet;
        }
        else if (IsKeyPressed(ImGuiKey_PageDown, true))
        {
            nav_scoring_rect_offset_y = +page_offset_y;
            g.NavMoveDir = ImGuiDir_Up; // Because our scoring rect is offset down, we request the up direction (so we can always land on the last item)
            g.NavMoveClipDir = ImGuiDir_Down;
            g.NavMoveFlags = ImGuiNavMoveFlags_AllowCurrentNavId | ImGuiNavMoveFlags_AlsoScoreVisibleSet;
        }
        else if (home_pressed)
        {
            // FIXME-NAV: handling of Home/End is assuming that the top/bottom most item will be visible with Scroll.y == 0/ScrollMax.y
            // Scrolling will be handled via the ImGuiNavMoveFlags_ScrollToEdgeY flag, we don't scroll immediately to avoid scrolling happening before nav result.
            // Preserve current horizontal position if we have any.
            nav_rect_rel.Min.y = nav_rect_rel.Max.y = 0.0;
            if (nav_rect_rel.IsInverted())
                nav_rect_rel.Min.x = nav_rect_rel.Max.x = 0.0;
            g.NavMoveDir = ImGuiDir_Down;
            g.NavMoveFlags = ImGuiNavMoveFlags_AllowCurrentNavId | ImGuiNavMoveFlags_ScrollToEdgeY;
            // FIXME-NAV: MoveClipDir left to _None, intentional?
        }
        else if (end_pressed)
        {
            nav_rect_rel.Min.y = nav_rect_rel.Max.y = window.ContentSize.y;
            if (nav_rect_rel.IsInverted())
                nav_rect_rel.Min.x = nav_rect_rel.Max.x = 0.0;
            g.NavMoveDir = ImGuiDir_Up;
            g.NavMoveFlags = ImGuiNavMoveFlags_AllowCurrentNavId | ImGuiNavMoveFlags_ScrollToEdgeY;
            // FIXME-NAV: MoveClipDir left to _None, intentional?
        }
        return nav_scoring_rect_offset_y;
    }
    return 0.0;
}

static c_void NavEndFrame()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    // Show CTRL+TAB list window
    if (g.NavWindowingTarget != null_mut())
        NavUpdateWindowingOverlay();

    // Perform wrap-around in menus
    // FIXME-NAV: Wrap may need to apply a weight bias on the other axis. e.g. 4x4 grid with 2 last items missing on last item won't handle LoopY/WrapY correctly.
    // FIXME-NAV: Wrap (not Loop) support could be handled by the scoring function and then WrapX would function without an extra frame.
    const ImGuiNavMoveFlags wanted_flags = ImGuiNavMoveFlags_WrapX | ImGuiNavMoveFlags_LoopX | ImGuiNavMoveFlags_WrapY | ImGuiNavMoveFlags_LoopY;
    if (g.NavWindow && NavMoveRequestButNoResultYet() && (g.NavMoveFlags & wanted_flags) && (g.NavMoveFlags & ImGuiNavMoveFlags_Forwarded) == 0)
        NavUpdateCreateWrappingRequest();
}

static c_void NavUpdateCreateWrappingRequest()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window: *mut ImGuiWindow =  g.NavWindow;

    let mut do_forward: bool =  false;
    let bb_rel: ImRect =  window.NavRectRel[g.NavLayer];
    clip_dir: ImGuiDir = g.NavMoveDir;
    const ImGuiNavMoveFlags move_flags = g.NavMoveFlags;
    if (g.NavMoveDir == ImGuiDir_Left && (move_flags & (ImGuiNavMoveFlags_WrapX | ImGuiNavMoveFlags_LoopX)))
    {
        bb_rel.Min.x = bb_rel.Max.x = window.ContentSize.x + window.WindowPadding.x;
        if (move_flags & ImGuiNavMoveFlags_WrapX)
        {
            bb_rel.TranslateY(-bb_rel.GetHeight()); // Previous row
            clip_dir = ImGuiDir_Up;
        }
        do_forward = true;
    }
    if (g.NavMoveDir == ImGuiDir_Right && (move_flags & (ImGuiNavMoveFlags_WrapX | ImGuiNavMoveFlags_LoopX)))
    {
        bb_rel.Min.x = bb_rel.Max.x = -window.WindowPadding.x;
        if (move_flags & ImGuiNavMoveFlags_WrapX)
        {
            bb_rel.TranslateY(+bb_rel.GetHeight()); // Next row
            clip_dir = ImGuiDir_Down;
        }
        do_forward = true;
    }
    if (g.NavMoveDir == ImGuiDir_Up && (move_flags & (ImGuiNavMoveFlags_WrapY | ImGuiNavMoveFlags_LoopY)))
    {
        bb_rel.Min.y = bb_rel.Max.y = window.ContentSize.y + window.WindowPadding.y;
        if (move_flags & ImGuiNavMoveFlags_WrapY)
        {
            bb_rel.TranslateX(-bb_rel.GetWidth()); // Previous column
            clip_dir = ImGuiDir_Left;
        }
        do_forward = true;
    }
    if (g.NavMoveDir == ImGuiDir_Down && (move_flags & (ImGuiNavMoveFlags_WrapY | ImGuiNavMoveFlags_LoopY)))
    {
        bb_rel.Min.y = bb_rel.Max.y = -window.WindowPadding.y;
        if (move_flags & ImGuiNavMoveFlags_WrapY)
        {
            bb_rel.TranslateX(+bb_rel.GetWidth()); // Next column
            clip_dir = ImGuiDir_Right;
        }
        do_forward = true;
    }
    if (!do_forward)
        return;
    window.NavRectRel[g.NavLayer] = bb_rel;
    NavMoveRequestForward(g.NavMoveDir, clip_dir, move_flags, g.NavMoveScrollFlags);
}

static FindWindowFocusIndex: c_int(window: *mut ImGuiWindow)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    IM_UNUSED(g);
    let order: c_int = window.FocusOrder;
    // IM_ASSERT(window.RootWindow == window); // No child window (not testing _ChildWindow because of docking)
    // IM_ASSERT(g.WindowsFocusOrder[order] == window);
    return order;
}

static FindWindowNavFocusable: *mut ImGuiWindow(i_start: c_int, i_stop: c_int, dir: c_int) // FIXME-OPT O(N)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    for (let i: c_int = i_start; i >= 0 && i < g.WindowsFocusOrder.Size && i != i_stop; i += dir)
        if (IsWindowNavFocusable(g.WindowsFocusOrder[i]))
            return g.WindowsFocusOrder[i];
    return null_mut();
}

static c_void NavUpdateWindowingHighlightWindow(focus_change_dir: c_int)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.NavWindowingTarget);
    if (g.NavWindowingTarget->Flags & ImGuiWindowFlags_Modal)
        return;

    let i_current: c_int = FindWindowFocusIndex(g.NavWindowingTarget);
    let mut window_target: *mut ImGuiWindow =  FindWindowNavFocusable(i_current + focus_change_dir, -INT_MAX, focus_change_dir);
    if (!window_target)
        window_target = FindWindowNavFocusable((focus_change_dir < 0) ? (g.WindowsFocusOrder.Size - 1) : 0, i_current, focus_change_dir);
    if (window_target) // Don't reset windowing target if there's a single window in the list
    {
        g.NavWindowingTarget = g.NavWindowingTargetAnim = window_target;
        g.NavWindowingAccumDeltaPos = g.NavWindowingAccumDeltaSize = ImVec2::new(0.0, 0.0);
    }
    g.NavWindowingToggleLayer = false;
}

// Windowing management mode
// Keyboard: CTRL+Tab (change focus/move/resize), Alt (toggle menu layer)
// Gamepad:  Hold Menu/Square (change focus/move/resize), Tap Menu/Square (toggle menu layer)
static c_void NavUpdateWindowing()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiIO& io = g.IO;

    let mut apply_focus_window: *mut ImGuiWindow =  null_mut();
    let mut apply_toggle_layer: bool =  false;

    let mut modal_window: *mut ImGuiWindow =  GetTopMostPopupModal();
    let mut allow_windowing: bool =  (modal_window == null_mut());
    if (!allow_windowing)
        g.NavWindowingTarget= null_mut();

    // Fade out
    if (g.NavWindowingTargetAnim && g.NavWindowingTarget == null_mut())
    {
        g.NavWindowingHighlightAlpha = ImMax(g.NavWindowingHighlightAlpha - io.DeltaTime * 10f32, 0.0);
        if (g.DimBgRatio <= 0.0 && g.NavWindowingHighlightAlpha <= 0.0)
            g.NavWindowingTargetAnim= null_mut();
    }

    // Start CTRL+Tab or Square+L/R window selection
    let nav_gamepad_active: bool = (io.ConfigFlags & ImGuiConfigFlags_NavEnableGamepad) != 0 && (io.BackendFlags & ImGuiBackendFlags_HasGamepad) != 0;
    let nav_keyboard_active: bool = (io.ConfigFlags & ImGuiConfigFlags_NavEnableKeyboard) != 0;
    let start_windowing_with_gamepad: bool = allow_windowing && nav_gamepad_active && !g.NavWindowingTarget && IsKeyPressed(ImGuiKey_NavGamepadMenu, false);
    let start_windowing_with_keyboard: bool = allow_windowing && !g.NavWindowingTarget && io.KeyCtrl && IsKeyPressed(ImGuiKey_Tab, false); // Note: enabled even without NavEnableKeyboard!
    if (start_windowing_with_gamepad || start_windowing_with_keyboard)
        if (let mut window: *mut ImGuiWindow =  g.NavWindow ? g.NavWindow : FindWindowNavFocusable(g.WindowsFocusOrder.Size - 1, -INT_MAX, -1))
        {
            g.NavWindowingTarget = g.NavWindowingTargetAnim = window.RootWindow;
            g.NavWindowingTimer = g.NavWindowingHighlightAlpha = 0.0;
            g.NavWindowingAccumDeltaPos = g.NavWindowingAccumDeltaSize = ImVec2::new(0.0, 0.0);
            g.NavWindowingToggleLayer = start_windowing_with_gamepad ? true : false; // Gamepad starts toggling layer
            g.NavInputSource = start_windowing_with_keyboard ? ImGuiInputSource_Keyboard : ImGuiInputSource_Gamepad;
        }

    // Gamepad update
    g.NavWindowingTimer += io.DeltaTime;
    if (g.NavWindowingTarget && g.NavInputSource == ImGuiInputSource_Gamepad)
    {
        // Highlight only appears after a brief time holding the button, so that a fast tap on PadMenu (to toggle NavLayer) doesn't add visual noise
        g.NavWindowingHighlightAlpha = ImMax(g.NavWindowingHighlightAlpha, ImSaturate((g.NavWindowingTimer - NAV_WINDOWING_HIGHLIGHT_DELAY) / 0.05f32));

        // Select window to focus
        let focus_change_dir: c_int = IsKeyPressed(ImGuiKey_GamepadL1) - IsKeyPressed(ImGuiKey_GamepadR1);
        if (focus_change_dir != 0)
        {
            NavUpdateWindowingHighlightWindow(focus_change_dir);
            g.NavWindowingHighlightAlpha = 1.0;
        }

        // Single press toggles NavLayer, long press with L/R apply actual focus on release (until then the window was merely rendered top-most)
        if (!IsKeyDown(ImGuiKey_NavGamepadMenu))
        {
            g.NavWindowingToggleLayer &= (g.NavWindowingHighlightAlpha < 1.0); // Once button was held long enough we don't consider it a tap-to-toggle-layer press anymore.
            if (g.NavWindowingToggleLayer && g.NavWindow)
                apply_toggle_layer = true;
            else if (!g.NavWindowingToggleLayer)
                apply_focus_window = g.NavWindowingTarget;
            g.NavWindowingTarget= null_mut();
        }
    }

    // Keyboard: Focus
    if (g.NavWindowingTarget && g.NavInputSource == ImGuiInputSource_Keyboard)
    {
        // Visuals only appears after a brief time after pressing TAB the first time, so that a fast CTRL+TAB doesn't add visual noise
        g.NavWindowingHighlightAlpha = ImMax(g.NavWindowingHighlightAlpha, ImSaturate((g.NavWindowingTimer - NAV_WINDOWING_HIGHLIGHT_DELAY) / 0.05f32)); // 1.0f
        if (IsKeyPressed(ImGuiKey_Tab, true))
            NavUpdateWindowingHighlightWindow(io.KeyShift ? 1 : -1);
        if (!io.KeyCtrl)
            apply_focus_window = g.NavWindowingTarget;
    }

    // Keyboard: Press and Release ALT to toggle menu layer
    // - Testing that only Alt is tested prevents Alt+Shift or AltGR from toggling menu layer.
    // - AltGR is normally Alt+Ctrl but we can't reliably detect it (not all backends/systems/layout emit it as Alt+Ctrl). But even on keyboards without AltGR we don't want Alt+Ctrl to open menu anyway.
    if (nav_keyboard_active && IsKeyPressed(ImGuiKey_ModAlt))
    {
        g.NavWindowingToggleLayer = true;
        g.NavInputSource = ImGuiInputSource_Keyboard;
    }
    if (g.NavWindowingToggleLayer && g.NavInputSource == ImGuiInputSource_Keyboard)
    {
        // We cancel toggling nav layer when any text has been typed (generally while holding Alt). (See #370)
        // We cancel toggling nav layer when other modifiers are pressed. (See #4439)
        if (io.InputQueueCharacters.Size > 0 || io.KeyCtrl || io.KeyShift || io.KeySuper)
            g.NavWindowingToggleLayer = false;

        // Apply layer toggle on release
        // Important: as before version <18314 we lacked an explicit IO event for focus gain/loss, we also compare mouse validity to detect old backends clearing mouse pos on focus loss.
        if (IsKeyReleased(ImGuiKey_ModAlt) && g.NavWindowingToggleLayer)
            if (g.ActiveId == 0 || g.ActiveIdAllowOverlap)
                if (IsMousePosValid(&io.MousePos) == IsMousePosValid(&io.MousePosPrev))
                    apply_toggle_layer = true;
        if (!IsKeyDown(ImGuiKey_ModAlt))
            g.NavWindowingToggleLayer = false;
    }

    // Move window
    if (g.NavWindowingTarget && !(g.NavWindowingTarget->Flags & ImGuiWindowFlags_NoMove))
    {
        nav_move_dir: ImVec2;
        if (g.NavInputSource == ImGuiInputSource_Keyboard && !io.KeyShift)
            nav_move_dir = GetKeyVector2d(ImGuiKey_LeftArrow, ImGuiKey_RightArrow, ImGuiKey_UpArrow, ImGuiKey_DownArrow);
        if (g.NavInputSource == ImGuiInputSource_Gamepad)
            nav_move_dir = GetKeyVector2d(ImGuiKey_GamepadLStickLeft, ImGuiKey_GamepadLStickRight, ImGuiKey_GamepadLStickUp, ImGuiKey_GamepadLStickDown);
        if (nav_move_dir.x != 0.0 || nav_move_dir.y != 0.0)
        {
            let NAV_MOVE_SPEED: c_float =  800f32;
            let move_step: c_float =  NAV_MOVE_SPEED * io.DeltaTime * ImMin(io.DisplayFramebufferScale.x, io.DisplayFramebufferScale.y);
            g.NavWindowingAccumDeltaPos += nav_move_dir * move_step;
            g.NavDisableMouseHover = true;
            let accum_floored: ImVec2 = ImFloor(g.NavWindowingAccumDeltaPos);
            if (accum_floored.x != 0.0 || accum_floored.y != 0.0)
            {
                let mut moving_window: *mut ImGuiWindow =  g.NavWindowingTarget->RootWindowDockTree;
                SetWindowPos(moving_window, moving_window.Pos + accum_floored, ImGuiCond_Always);
                g.NavWindowingAccumDeltaPos -= accum_floored;
            }
        }
    }

    // Apply final focus
    if (apply_focus_window && (g.NavWindow == null_mut() || apply_focus_window != g.NavWindow.RootWindow))
    {
        ImGuiViewport* previous_viewport = g.NavWindow ? g.NavWindow.Viewport : null_mut();
        ClearActiveID();
        NavRestoreHighlightAfterMove();
        apply_focus_window = NavRestoreLastChildNavWindow(apply_focus_window);
        ClosePopupsOverWindow(apply_focus_window, false);
        FocusWindow(apply_focus_window);
        if (apply_focus_window.NavLastIds[0] == 0)
            NavInitWindow(apply_focus_window, false);

        // If the window has ONLY a menu layer (no main layer), select it directly
        // Use NavLayersActiveMaskNext since windows didn't have a chance to be Begin()-ed on this frame,
        // so CTRL+Tab where the keys are only held for 1 frame will be able to use correct layers mask since
        // the target window as already been previewed once.
        // FIXME-NAV: This should be done in NavInit.. or in FocusWindow... However in both of those cases,
        // we won't have a guarantee that windows has been visible before and therefore NavLayersActiveMask*
        // won't be valid.
        if (apply_focus_window.DC.NavLayersActiveMaskNext == (1 << ImGuiNavLayer_Menu))
            g.NavLayer = ImGuiNavLayer_Menu;

        // Request OS level focus
        if (apply_focus_window.Viewport != previous_viewport && g.PlatformIO.Platform_SetWindowFocus)
            g.PlatformIO.Platform_SetWindowFocus(apply_focus_window.Viewport);
    }
    if (apply_focus_window)
        g.NavWindowingTarget= null_mut();

    // Apply menu/layer toggle
    if (apply_toggle_layer && g.NavWindow)
    {
        ClearActiveID();

        // Move to parent menu if necessary
        let mut new_nav_window: *mut ImGuiWindow =  g.NavWindow;
        while (new_nav_window.ParentWindow
            && (new_nav_window.DC.NavLayersActiveMask & (1 << ImGuiNavLayer_Menu)) == 0
            && (new_nav_window.Flags & ImGuiWindowFlags_ChildWindow) != 0
            && (new_nav_window.Flags & (ImGuiWindowFlags_Popup | ImGuiWindowFlags_ChildMenu)) == 0)
            new_nav_window = new_nav_window.ParentWindow;
        if (new_nav_window != g.NavWindow)
        {
            let mut old_nav_window: *mut ImGuiWindow =  g.NavWindow;
            FocusWindow(new_nav_window);
            new_nav_window.NavLastChildNavWindow = old_nav_window;
        }

        // Toggle layer
        const ImGuiNavLayer new_nav_layer = (g.NavWindow.DC.NavLayersActiveMask & (1 << ImGuiNavLayer_Menu)) ? (ImGuiNavLayer)(g.NavLayer ^ 1) : ImGuiNavLayer_Main;
        if (new_nav_layer != g.NavLayer)
        {
            // Reinitialize navigation when entering menu bar with the Alt key (FIXME: could be a properly of the layer?)
            let preserve_layer_1_nav_id: bool = (new_nav_window.DockNodeAsHost != null_mut());
            if (new_nav_layer == ImGuiNavLayer_Menu && !preserve_layer_1_nav_id)
                g.NavWindow.NavLastIds[new_nav_layer] = 0;
            NavRestoreLayer(new_nav_layer);
            NavRestoreHighlightAfterMove();
        }
    }
}

// Window has already passed the IsWindowNavFocusable()
static GetFallbackWindowNameForWindowingList: *const c_char(window: *mut ImGuiWindow)
{
    if (window.Flags & ImGuiWindowFlags_Popup)
        return "(Popup)";
    if ((window.Flags & ImGuiWindowFlags_MenuBar) && strcmp(window.Name, "##MainMenuBar") == 0)
        return "(Main menu bar)";
    if (window.DockNodeAsHost)
        return "(Dock node)";
    return "(Untitled)";
}

// Overlay displayed when using CTRL+TAB. Called by EndFrame().
c_void NavUpdateWindowingOverlay()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.NavWindowingTarget != NULL);

    if (g.NavWindowingTimer < NAV_WINDOWING_LIST_APPEAR_DELAY)
        return;

    if (g.NavWindowingListWindow == null_mut())
        g.NavWindowingListWindow = FindWindowByName("###NavWindowingList");
    let viewport: *const ImGuiViewport = /*g.NavWindow ? g.Navwindow.Viewport :*/ GetMainViewport();
    SetNextWindowSizeConstraints(ImVec2::new(viewport.Size.x * 0.20f32, viewport.Size.y * 0.200f32), ImVec2::new(f32::MAX, f32::MAX));
    SetNextWindowPos(viewport.GetCenter(), ImGuiCond_Always, ImVec2::new(0.5f32, 0.5f32));
    PushStyleVar(ImGuiStyleVar_WindowPadding, g.Style.WindowPadding * 2.00f32);
    Begin("###NavWindowingList", null_mut(), ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoFocusOnAppearing | ImGuiWindowFlags_NoResize | ImGuiWindowFlags_NoMove | ImGuiWindowFlags_NoInputs | ImGuiWindowFlags_AlwaysAutoResize | ImGuiWindowFlags_NoSavedSettings);
    for (let n: c_int = g.WindowsFocusOrder.Size - 1; n >= 0; n--)
    {
        let mut window: *mut ImGuiWindow =  g.WindowsFocusOrder[n];
        // IM_ASSERT(window != NULL); // Fix static analyzers
        if (!IsWindowNavFocusable(window))
            continue;
        let mut  label: *const c_char = window.Name;
        if (label == FindRenderedTextEnd(label))
            label = GetFallbackWindowNameForWindowingList(window);
        Selectable(label, g.NavWindowingTarget == window);
    }
    End();
    PopStyleVar();
}


//-----------------------------------------------------------------------------
// [SECTION] DRAG AND DROP
//-----------------------------------------------------------------------------

IsDragDropActive: bool()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.DragDropActive;
}

c_void ClearDragDrop()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.DragDropActive = false;
    g.DragDropPayload.Clear();
    g.DragDropAcceptFlags = ImGuiDragDropFlags_None;
    g.DragDropAcceptIdCurr = g.DragDropAcceptIdPrev = 0;
    g.DragDropAcceptIdCurrRectSurface = f32::MAX;
    g.DragDropAcceptFrameCount = -1;

    g.DragDropPayloadBufHeap.clear();
    memset(&g.DragDropPayloadBufLocal, 0, sizeof(g.DragDropPayloadBufLocal));
}

// When this returns true you need to: a) call SetDragDropPayload() exactly once, b) you may render the payload visual/description, c) call EndDragDropSource()
// If the item has an identifier:
// - This assume/require the item to be activated (typically via ButtonBehavior).
// - Therefore if you want to use this with a mouse button other than left mouse button, it is up to the item itself to activate with another button.
// - We then pull and use the mouse button that was used to activate the item and use it to carry on the drag.
// If the item has no identifier:
// - Currently always assume left mouse button.
BeginDragDropSource: bool(ImGuiDragDropFlags flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;

    // FIXME-DRAGDROP: While in the common-most "drag from non-zero active id" case we can tell the mouse button,
    // in both SourceExtern and id==0 cases we may requires something else (explicit flags or some heuristic).
    let mut mouse_button: ImGuiMouseButton =  ImGuiMouseButton_Left;

    let mut source_drag_active: bool =  false;
    let mut source_id: ImGuiID =  0;
    let mut source_parent_id: ImGuiID =  0;
    if (!(flags & ImGuiDragDropFlags_SourceExtern))
    {
        source_id = g.LastItemData.ID;
        if (source_id != 0)
        {
            // Common path: items with ID
            if (g.ActiveId != source_id)
                return false;
            if (g.ActiveIdMouseButton != -1)
                mouse_button = g.ActiveIdMouseButton;
            if (g.IO.MouseDown[mouse_button] == false || window.SkipItems)
                return false;
            g.ActiveIdAllowOverlap = false;
        }
        else
        {
            // Uncommon path: items without ID
            if (g.IO.MouseDown[mouse_button] == false || window.SkipItems)
                return false;
            if ((g.LastItemData.StatusFlags & ImGuiItemStatusFlags_HoveredRect) == 0 && (g.ActiveId == 0 || g.ActiveIdWindow != window))
                return false;

            // If you want to use BeginDragDropSource() on an item with no unique identifier for interaction, such as Text() or Image(), you need to:
            // A) Read the explanation below, B) Use the ImGuiDragDropFlags_SourceAllowNullID flag.
            if (!(flags & ImGuiDragDropFlags_SourceAllowNullID))
            {
                // IM_ASSERT(0);
                return false;
            }

            // Magic fallback to handle items with no assigned ID, e.g. Text(), Image()
            // We build a throwaway ID based on current ID stack + relative AABB of items in window.
            // THE IDENTIFIER WON'T SURVIVE ANY REPOSITIONING/RESIZINGG OF THE WIDGET, so if your widget moves your dragging operation will be canceled.
            // We don't need to maintain/call ClearActiveID() as releasing the button will early out this function and trigger !ActiveIdIsAlive.
            // Rely on keeping other window.LastItemXXX fields intact.
            source_id = g.LastItemData.ID = window.GetIDFromRectangle(g.LastItemData.Rect);
            KeepAliveID(source_id);
            let mut is_hovered: bool =  ItemHoverable(g.LastItemData.Rect, source_id);
            if (is_hovered && g.IO.MouseClicked[mouse_button])
            {
                SetActiveID(source_id, window);
                FocusWindow(window);
            }
            if (g.ActiveId == source_id) // Allow the underlying widget to display/return hovered during the mouse release frame, else we would get a flicker.
                g.ActiveIdAllowOverlap = is_hovered;
        }
        if (g.ActiveId != source_id)
            return false;
        source_parent_id = window.IDStack.last().unwrap();
        source_drag_active = IsMouseDragging(mouse_button);

        // Disable navigation and key inputs while dragging + cancel existing request if any
        SetActiveIdUsingAllKeyboardKeys();
    }
    else
    {
        window= null_mut();
        source_id = ImHashStr("#SourceExtern");
        source_drag_active = true;
    }

    if (source_drag_active)
    {
        if (!g.DragDropActive)
        {
            // IM_ASSERT(source_id != 0);
            ClearDragDrop();
            ImGuiPayload& payload = g.DragDropPayload;
            payload.SourceId = source_id;
            payload.SourceParentId = source_parent_id;
            g.DragDropActive = true;
            g.DragDropSourceFlags = flags;
            g.DragDropMouseButton = mouse_button;
            if (payload.SourceId == g.ActiveId)
                g.ActiveIdNoClearOnFocusLoss = true;
        }
        g.DragDropSourceFrameCount = g.FrameCount;
        g.DragDropWithinSource = true;

        if (!(flags & ImGuiDragDropFlags_SourceNoPreviewTooltip))
        {
            // Target can request the Source to not display its tooltip (we use a dedicated flag to make this request explicit)
            // We unfortunately can't just modify the source flags and skip the call to BeginTooltip, as caller may be emitting contents.
            BeginTooltip();
            if (g.DragDropAcceptIdPrev && (g.DragDropAcceptFlags & ImGuiDragDropFlags_AcceptNoPreviewTooltip))
            {
                let mut tooltip_window: *mut ImGuiWindow =  g.CurrentWindow;
                tooltip_window.Hidden = tooltip_window.SkipItems = true;
                tooltip_window.HiddenFramesCanSkipItems = 1;
            }
        }

        if (!(flags & ImGuiDragDropFlags_SourceNoDisableHover) && !(flags & ImGuiDragDropFlags_SourceExtern))
            g.LastItemData.StatusFlags &= !ImGuiItemStatusFlags_HoveredRect;

        return true;
    }
    return false;
}

c_void EndDragDropSource()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.DragDropActive);
    // IM_ASSERT(g.DragDropWithinSource && "Not after a BeginDragDropSource()?");

    if (!(g.DragDropSourceFlags & ImGuiDragDropFlags_SourceNoPreviewTooltip))
        EndTooltip();

    // Discard the drag if have not called SetDragDropPayload()
    if (g.DragDropPayload.DataFrameCount == -1)
        ClearDragDrop();
    g.DragDropWithinSource = false;
}

// Use 'cond' to choose to submit payload on drag start or every frame
SetDragDropPayload: bool(type: *const c_char, *const c_void data, data_size: size_t, cond: ImGuiCond)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiPayload& payload = g.DragDropPayload;
    if (cond == 0)
        cond = ImGuiCond_Always;

    // IM_ASSERT(type != NULL);
    // IM_ASSERT(strlen(type) < IM_ARRAYSIZE(payload.DataType) && "Payload type can be at most 32 characters long");
    // IM_ASSERT((data != NULL && data_size > 0) || (data == NULL && data_size == 0));
    // IM_ASSERT(cond == ImGuiCond_Always || cond == ImGuiCond_Once);
    // IM_ASSERT(payload.SourceId != 0);                               // Not called between BeginDragDropSource() and EndDragDropSource()

    if (cond == ImGuiCond_Always || payload.DataFrameCount == -1)
    {
        // Copy payload
        ImStrncpy(payload.DataType, type, IM_ARRAYSIZE(payload.DataType));
        g.DragDropPayloadBufHeap.clear();
        if (data_size > sizeof(g.DragDropPayloadBufLocal))
        {
            // Store in heap
            g.DragDropPayloadBufHeap.resize(data_size);
            payload.Data = g.DragDropPayloadBufHeap.Data;
            memcpy(payload.Data, data, data_size);
        }
        else if (data_size > 0)
        {
            // Store locally
            memset(&g.DragDropPayloadBufLocal, 0, sizeof(g.DragDropPayloadBufLocal));
            payload.Data = g.DragDropPayloadBufLocal;
            memcpy(payload.Data, data, data_size);
        }
        else
        {
            payload.Data= null_mut();
        }
        payload.DataSize = data_size;
    }
    payload.DataFrameCount = g.FrameCount;

    // Return whether the payload has been accepted
    return (g.DragDropAcceptFrameCount == g.FrameCount) || (g.DragDropAcceptFrameCount == g.FrameCount - 1);
}

BeginDragDropTargetCustom: bool(bb: &ImRect, id: ImGuiID)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (!g.DragDropActive)
        return false;

    let mut window = g.CurrentWindow;
    let mut hovered_window: *mut ImGuiWindow =  g.HoveredWindowUnderMovingWindow;
    if (hovered_window == null_mut() || window.RootWindowDockTree != hovered_window.RootWindowDockTree)
        return false;
    // IM_ASSERT(id != 0);
    if (!IsMouseHoveringRect(bb.Min, bb.Max) || (id == g.DragDropPayload.SourceId))
        return false;
    if (window.SkipItems)
        return false;

    // IM_ASSERT(g.DragDropWithinTarget == false);
    g.DragDropTargetRect = bb;
    g.DragDropTargetId = id;
    g.DragDropWithinTarget = true;
    return true;
}

// We don't use BeginDragDropTargetCustom() and duplicate its code because:
// 1) we use LastItemRectHoveredRect which handles items that pushes a temporarily clip rectangle in their code. Calling BeginDragDropTargetCustom(LastItemRect) would not handle them.
// 2) and it's faster. as this code may be very frequently called, we want to early out as fast as we can.
// Also note how the HoveredWindow test is positioned differently in both functions (in both functions we optimize for the cheapest early out case)
BeginDragDropTarget: bool()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (!g.DragDropActive)
        return false;

    let mut window = g.CurrentWindow;
    if (!(g.LastItemData.StatusFlags & ImGuiItemStatusFlags_HoveredRect))
        return false;
    let mut hovered_window: *mut ImGuiWindow =  g.HoveredWindowUnderMovingWindow;
    if (hovered_window == null_mut() || window.RootWindowDockTree != hovered_window.RootWindowDockTree || window.SkipItems)
        return false;

    display_rect: &ImRect = (g.LastItemData.StatusFlags & ImGuiItemStatusFlags_HasDisplayRect) ? g.LastItemData.DisplayRect : g.LastItemData.Rect;
    let mut id: ImGuiID =  g.LastItemData.ID;
    if (id == 0)
    {
        id = window.GetIDFromRectangle(display_rect);
        KeepAliveID(id);
    }
    if (g.DragDropPayload.SourceId == id)
        return false;

    // IM_ASSERT(g.DragDropWithinTarget == false);
    g.DragDropTargetRect = display_rect;
    g.DragDropTargetId = id;
    g.DragDropWithinTarget = true;
    return true;
}

IsDragDropPayloadBeingAccepted: bool()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.DragDropActive && g.DragDropAcceptIdPrev != 0;
}

*const ImGuiPayload AcceptDragDropPayload(type: *const c_char, ImGuiDragDropFlags flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    ImGuiPayload& payload = g.DragDropPayload;
    // IM_ASSERT(g.DragDropActive);                        // Not called between BeginDragDropTarget() and EndDragDropTarget() ?
    // IM_ASSERT(payload.DataFrameCount != -1);            // Forgot to call EndDragDropTarget() ?
    if (type != null_mut() && !payload.IsDataType(type))
        return null_mut();

    // Accept smallest drag target bounding box, this allows us to nest drag targets conveniently without ordering constraints.
    // NB: We currently accept NULL id as target. However, overlapping targets requires a unique ID to function!
    let was_accepted_previously: bool = (g.DragDropAcceptIdPrev == g.DragDropTargetId);
    let r: ImRect =  g.DragDropTargetRect;
    let r_surface: c_float =  r.GetWidth() * r.GetHeight();
    if (r_surface <= g.DragDropAcceptIdCurrRectSurface)
    {
        g.DragDropAcceptFlags = flags;
        g.DragDropAcceptIdCurr = g.DragDropTargetId;
        g.DragDropAcceptIdCurrRectSurface = r_surface;
    }

    // Render default drop visuals
    // FIXME-DRAGDROP: Settle on a proper default visuals for drop target.
    payload.Preview = was_accepted_previously;
    flags |= (g.DragDropSourceFlags & ImGuiDragDropFlags_AcceptNoDrawDefaultRect); // Source can also inhibit the preview (useful for external sources that lives for 1 frame)
    if (!(flags & ImGuiDragDropFlags_AcceptNoDrawDefaultRect) && payload.Preview)
        window.DrawList.AddRect(r.Min - ImVec2::new(3.5f32,3.5f32), r.Max + ImVec2::new(3.5f32, 3.5f32), GetColorU32(ImGuiCol_DragDropTarget), 0.0, 0, 2.00f32);

    g.DragDropAcceptFrameCount = g.FrameCount;
    payload.Delivery = was_accepted_previously && !IsMouseDown(g.DragDropMouseButton); // For extern drag sources affecting os window focus, it's easier to just test !IsMouseDown() instead of IsMouseReleased()
    if (!payload.Delivery && !(flags & ImGuiDragDropFlags_AcceptBeforeDelivery))
        return null_mut();

    return &payload;
}

*const ImGuiPayload GetDragDropPayload()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.DragDropActive ? &g.DragDropPayload : null_mut();
}

// We don't really use/need this now, but added it for the sake of consistency and because we might need it later.
c_void EndDragDropTarget()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.DragDropActive);
    // IM_ASSERT(g.DragDropWithinTarget);
    g.DragDropWithinTarget = false;
}

//-----------------------------------------------------------------------------
// [SECTION] LOGGING/CAPTURING
//-----------------------------------------------------------------------------
// All text output from the interface can be captured into tty/file/clipboard.
// By default, tree nodes are automatically opened during logging.
//-----------------------------------------------------------------------------

// Pass text data straight to log (without being displayed)
static inline c_void LogTextV(ImGuiContext& g, fmt: *const c_char, va_list args)
{
    if (g.LogFile)
    {
        g.LogBuffer.Buf.clear();
        g.LogBuffer.appendfv(fmt, args);
        ImFileWrite(g.LogBuffer.c_str(), sizeof, (u64)g.LogBuffer.size(), g.LogFile);
    }
    else
    {
        g.LogBuffer.appendfv(fmt, args);
    }
}

c_void LogText(fmt: *const c_char, ...)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (!g.LogEnabled)
        return;

    va_list args;
    va_start(args, fmt);
    LogTextV(g, fmt, args);
    va_end(args);
}

c_void LogTextV(fmt: *const c_char, va_list args)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (!g.LogEnabled)
        return;

    LogTextV(g, fmt, args);
}

// Start logging/capturing text output
c_void LogBegin(ImGuiLogType type, auto_open_depth: c_int)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    // IM_ASSERT(g.LogEnabled == false);
    // IM_ASSERT(g.LogFile == NULL);
    // IM_ASSERT(g.LogBuffer.empty());
    g.LogEnabled = true;
    g.LogType = type;
    g.LogNextPrefix = g.LogNextSuffix= null_mut();
    g.LogDepthRef = window.DC.TreeDepth;
    g.LogDepthToExpand = ((auto_open_depth >= 0) ? auto_open_depth : g.LogDepthToExpandDefault);
    g.LogLinePosY = f32::MAX;
    g.LogLineFirstItem = true;
}

// Important: doesn't copy underlying data, use carefully (prefix/suffix must be in scope at the time of the next LogRenderedText)
c_void LogSetNextTextDecoration(prefix: *const c_char, suffix: *const c_char)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.LogNextPrefix = prefix;
    g.LogNextSuffix = suffix;
}

c_void LogToTTY(auto_open_depth: c_int)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.LogEnabled)
        return;
    IM_UNUSED(auto_open_depth);
// #ifndef IMGUI_DISABLE_TTY_FUNCTIONS
    LogBegin(ImGuiLogType_TTY, auto_open_depth);
    g.LogFile = stdout;
// #endif
}

// Start logging/capturing text output to given file
c_void LogToFile(auto_open_depth: c_int, filename: *const c_char)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.LogEnabled)
        return;

    // FIXME: We could probably open the file in text mode "at", however note that clipboard/buffer logging will still
    // be subject to outputting OS-incompatible carriage return if within strings the user doesn't use IM_NEWLINE.
    // By opening the file in binary mode "ab" we have consistent output everywhere.
    if (!filename)
        filename = g.IO.LogFilename;
    if (!filename || !filename[0])
        return;
    ImFileHandle f = ImFileOpen(filename, "ab");
    if (!0.0)
    {
        // IM_ASSERT(0);
        return;
    }

    LogBegin(ImGuiLogType_File, auto_open_depth);
    g.LogFile = f;
}

// Start logging/capturing text output to clipboard
c_void LogToClipboard(auto_open_depth: c_int)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.LogEnabled)
        return;
    LogBegin(ImGuiLogType_Clipboard, auto_open_depth);
}

c_void LogToBuffer(auto_open_depth: c_int)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.LogEnabled)
        return;
    LogBegin(ImGuiLogType_Buffer, auto_open_depth);
}

c_void LogFinish()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (!g.LogEnabled)
        return;

    LogText(IM_NEWLINE);
    switch (g.LogType)
    {
    case ImGuiLogType_TTY:
// #ifndef IMGUI_DISABLE_TTY_FUNCTIONS
        fflush(g.LogFile);
// #endif
        break;
    case ImGuiLogType_File:
        ImFileClose(g.LogFile);
        break;
    case ImGuiLogType_Buffer:
        break;
    case ImGuiLogType_Clipboard:
        if (!g.LogBuffer.empty())
            SetClipboardText(g.LogBuffer.begin());
        break;
    case ImGuiLogType_None:
        // IM_ASSERT(0);
        break;
    }

    g.LogEnabled = false;
    g.LogType = ImGuiLogType_None;
    g.LogFile= null_mut();
    g.LogBuffer.clear();
}

// Helper to display logging buttons
// FIXME-OBSOLETE: We should probably obsolete this and let the user have their own helper (this is one of the oldest function alive!)
c_void LogButtons()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    PushID("LogButtons");
// #ifndef IMGUI_DISABLE_TTY_FUNCTIONS
    let log_to_tty: bool = Button("Log To TTY"); SameLine();
// #else
    let log_to_tty: bool = false;
// #endif
    let log_to_file: bool = Button("Log To File"); SameLine();
    let log_to_clipboard: bool = Button("Log To Clipboard"); SameLine();
    PushAllowKeyboardFocus(false);
    SetNextItemWidth(80f32);
    SliderInt("Default Depth", &g.LogDepthToExpandDefault, 0, 9, null_mut());
    PopAllowKeyboardFocus();
    PopID();

    // Start logging at the end of the function so that the buttons don't appear in the log
    if (log_to_tty)
        LogToTTY();
    if (log_to_file)
        LogToFile();
    if (log_to_clipboard)
        LogToClipboard();
}


//-----------------------------------------------------------------------------
// [SECTION] SETTINGS
//-----------------------------------------------------------------------------
// - UpdateSettings() [Internal]
// - MarkIniSettingsDirty() [Internal]
// - CreateNewWindowSettings() [Internal]
// - FindWindowSettings() [Internal]
// - FindOrCreateWindowSettings() [Internal]
// - FindSettingsHandler() [Internal]
// - ClearIniSettings() [Internal]
// - LoadIniSettingsFromDisk()
// - LoadIniSettingsFromMemory()
// - SaveIniSettingsToDisk()
// - SaveIniSettingsToMemory()
// - WindowSettingsHandler_***() [Internal]
//-----------------------------------------------------------------------------

// Called by NewFrame()
c_void UpdateSettings()
{
    // Load settings on first frame (if not explicitly loaded manually before)
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (!g.SettingsLoaded)
    {
        // IM_ASSERT(g.SettingsWindows.empty());
        if (g.IO.IniFilename)
            LoadIniSettingsFromDisk(g.IO.IniFilename);
        g.SettingsLoaded = true;
    }

    // Save settings (with a delay after the last modification, so we don't spam disk too much)
    if (g.SettingsDirtyTimer > 0.0)
    {
        g.SettingsDirtyTimer -= g.IO.DeltaTime;
        if (g.SettingsDirtyTimer <= 0.0)
        {
            if (g.IO.IniFilename != null_mut())
                SaveIniSettingsToDisk(g.IO.IniFilename);
            else
                g.IO.WantSaveIniSettings = true;  // Let user know they can call SaveIniSettingsToMemory(). user will need to clear io.WantSaveIniSettings themselves.
            g.SettingsDirtyTimer = 0.0;
        }
    }
}

c_void MarkIniSettingsDirty()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.SettingsDirtyTimer <= 0.0)
        g.SettingsDirtyTimer = g.IO.IniSavingRate;
}

c_void MarkIniSettingsDirty(window: *mut ImGuiWindow)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (!(window.Flags & ImGuiWindowFlags_NoSavedSettings))
        if (g.SettingsDirtyTimer <= 0.0)
            g.SettingsDirtyTimer = g.IO.IniSavingRate;
}

CreateNewWindowSettings: *mut ImGuiWindowSettings(name: *const c_char)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

// #if !IMGUI_DEBUG_INI_SETTINGS
    // Skip to the "###" marker if any. We don't skip past to match the behavior of GetID()
    // Preserve the full string when IMGUI_DEBUG_INI_SETTINGS is set to make .ini inspection easier.
    if (p: *const c_char = strstr(name, "###"))
        name = p;
// #endif
    const name_len: size_t = strlen(name);

    // Allocate chunk
    const chunk_size: size_t = sizeof(ImGuiWindowSettings) + name_len + 1;
    settings: *mut ImGuiWindowSettings = g.SettingsWindows.alloc_chunk(chunk_size);
    IM_PLACEMENT_NEW(settings) ImGuiWindowSettings();
    settings->ID = ImHashStr(name, name_len);
    memcpy(settings->GetName(), name, name_len + 1);   // Store with zero terminator

    return settings;
}

FindWindowSettings: *mut ImGuiWindowSettings(id: ImGuiID)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    for (settings: *mut ImGuiWindowSettings = g.SettingsWindows.begin(); settings != null_mut(); settings = g.SettingsWindows.next_chunk(settings))
        if (settings->ID == id)
            return settings;
    return null_mut();
}

FindOrCreateWindowSettings: *mut ImGuiWindowSettings(name: *const c_char)
{
    if (settings: *mut ImGuiWindowSettings = FindWindowSettings(ImHashStr(name)))
        return settings;
    return CreateNewWindowSettings(name);
}

c_void AddSettingsHandler(*const ImGuiSettingsHandler handler)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(FindSettingsHandler(handler->TypeName) == NULL);
    g.SettingsHandlers.push(*handler);
}

c_void RemoveSettingsHandler(type_name: *const c_char)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (ImGuiSettingsHandler* handler = FindSettingsHandler(type_name))
        g.SettingsHandlers.erase(handler);
}

ImGuiSettingsHandler* FindSettingsHandler(type_name: *const c_char)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut type_hash: ImGuiID =  ImHashStr(type_name);
    for (let handler_n: c_int = 0; handler_n < g.SettingsHandlers.Size; handler_n++)
        if (g.SettingsHandlers[handler_n].TypeHash == type_hash)
            return &g.SettingsHandlers[handler_n];
    return null_mut();
}

c_void ClearIniSettings()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.SettingsIniData.clear();
    for (let handler_n: c_int = 0; handler_n < g.SettingsHandlers.Size; handler_n++)
        if (g.SettingsHandlers[handler_n].ClearAllFn)
            g.SettingsHandlers[handler_n].ClearAllFn(&g, &g.SettingsHandlers[handler_n]);
}

c_void LoadIniSettingsFromDisk(ini_filename: *const c_char)
{
    file_data_size: size_t = 0;
    char* file_data = (char*)ImFileLoadToMemory(ini_filename, "rb", &file_data_size);
    if (!file_data)
        return;
    if (file_data_size > 0)
        LoadIniSettingsFromMemory(file_data, file_data_size);
    IM_FREE(file_data);
}

// Zero-tolerance, no error reporting, cheap .ini parsing
c_void LoadIniSettingsFromMemory(ini_data: *const c_char, ini_size: size_t)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.Initialized);
    //IM_ASSERT(!g.WithinFrameScope && "Cannot be called between NewFrame() and EndFrame()");
    //IM_ASSERT(g.SettingsLoaded == false && g.FrameCount == 0);

    // For user convenience, we allow passing a non zero-terminated string (hence the ini_size parameter).
    // For our convenience and to make the code simpler, we'll also write zero-terminators within the buffer. So let's create a writable copy..
    if (ini_size == 0)
        ini_size = strlen(ini_data);
    g.SettingsIniData.Buf.resize(ini_size + 1);
    char* const buf = g.SettingsIniData.Buf.Data;
    char* const buf_end = buf + ini_size;
    memcpy(buf, ini_data, ini_size);
    buf_end[0] = 0;

    // Call pre-read handlers
    // Some types will clear their data (e.g. dock information) some types will allow merge/override (window)
    for (let handler_n: c_int = 0; handler_n < g.SettingsHandlers.Size; handler_n++)
        if (g.SettingsHandlers[handler_n].ReadInitFn)
            g.SettingsHandlers[handler_n].ReadInitFn(&g, &g.SettingsHandlers[handler_n]);

    entry_data: *mut c_void= null_mut();
    ImGuiSettingsHandler* entry_handler= null_mut();

    char* line_end= null_mut();
    for (char* line = buf; line < buf_end; line = line_end + 1)
    {
        // Skip new lines markers, then find end of the line
        while (*line == '\n' || *line == '\r')
            line+= 1;
        line_end = line;
        while (line_end < buf_end && *line_end != '\n' && *line_end != '\r')
            line_end+= 1;
        line_end[0] = 0;
        if (line[0] == ';')
            continue;
        if (line[0] == '[' && line_end > line && line_end[-1] == ']')
        {
            // Parse "[Type][Name]". Note that 'Name' can itself contains [] characters, which is acceptable with the current format and parsing code.
            line_end[-1] = 0;
            let mut  name_end: *const c_char = line_end - 1;
            let mut  type_start: *const c_char = line + 1;
            char* type_end = (char*)(*mut c_void)ImStrchrRange(type_start, name_end, ']');
            let mut  name_start: *const c_char = type_end ? ImStrchrRange(type_end + 1, name_end, '[') : null_mut();
            if (!type_end || !name_start)
                continue;
            *type_end = 0; // Overwrite first ']'
            name_start+= 1;  // Skip second '['
            entry_handler = FindSettingsHandler(type_start);
            entry_data = entry_handler ? entry_handler->ReadOpenFn(&g, entry_handler, name_start) : null_mut();
        }
        else if (entry_handler != null_mut() && entry_data != null_mut())
        {
            // Let type handler parse the line
            entry_handler->ReadLineFn(&g, entry_handler, entry_data, line);
        }
    }
    g.SettingsLoaded = true;

    // [DEBUG] Restore untouched copy so it can be browsed in Metrics (not strictly necessary)
    memcpy(buf, ini_data, ini_size);

    // Call post-read handlers
    for (let handler_n: c_int = 0; handler_n < g.SettingsHandlers.Size; handler_n++)
        if (g.SettingsHandlers[handler_n].ApplyAllFn)
            g.SettingsHandlers[handler_n].ApplyAllFn(&g, &g.SettingsHandlers[handler_n]);
}

c_void SaveIniSettingsToDisk(ini_filename: *const c_char)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.SettingsDirtyTimer = 0.0;
    if (!ini_filename)
        return;

    ini_data_size: size_t = 0;
    let mut  ini_data: *const c_char = SaveIniSettingsToMemory(&ini_data_size);
    ImFileHandle f = ImFileOpen(ini_filename, "wt");
    if (!0.0)
        return;
    ImFileWrite(ini_data, sizeof, ini_data_size, 0.0);
    ImFileClose(0.0);
}

// Call registered handlers (e.g. SettingsHandlerWindow_WriteAll() + custom handlers) to write their stuff into a text buffer
SaveIniSettingsToMemory: *const c_char(size_t* out_size)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.SettingsDirtyTimer = 0.0;
    g.SettingsIniData.Buf.clear();
    g.SettingsIniData.Buf.push(0);
    for (let handler_n: c_int = 0; handler_n < g.SettingsHandlers.Size; handler_n++)
    {
        ImGuiSettingsHandler* handler = &g.SettingsHandlers[handler_n];
        handler->WriteAllFn(&g, handler, &g.SettingsIniData);
    }
    if (out_size)
        *out_size = g.SettingsIniData.size();
    return g.SettingsIniData.c_str();
}

static c_void WindowSettingsHandler_ClearAll(ImGuiContext* ctx, ImGuiSettingsHandler*)
{
    ImGuiContext& g = *ctx;
    for (let i: c_int = 0; i != g.Windows.len(); i++)
        g.Windows[i]->SettingsOffset = -1;
    g.SettingsWindows.clear();
}

static *mut c_void WindowSettingsHandler_ReadOpen(ImGuiContext*, ImGuiSettingsHandler*, name: *const c_char)
{
    settings: *mut ImGuiWindowSettings = FindOrCreateWindowSettings(name);
    let mut id: ImGuiID =  settings->ID;
    *settings = ImGuiWindowSettings(); // Clear existing if recycling previous entry
    settings->ID = id;
    settings->WantApply = true;
    return (*mut c_void)settings;
}

static c_void WindowSettingsHandler_ReadLine(ImGuiContext*, ImGuiSettingsHandler*, entry: *mut c_void, line: *const c_char)
{
    settings: *mut ImGuiWindowSettings = (ImGuiWindowSettings*)entry;
    x: c_int, y;
    let mut i: c_int = 0;
    u1: u32;
    if (sscanf(line, "Pos=%i,%i", &x, &y) == 2)             { settings->Pos = ImVec2ih(x, y); }
    else if (sscanf(line, "Size=%i,%i", &x, &y) == 2)       { settings->Size = ImVec2ih(x, y); }
    else if (sscanf(line, "ViewportId=0x%08X", &u1) == 1)   { settings->ViewportId = u1; }
    else if (sscanf(line, "ViewportPos=%i,%i", &x, &y) == 2){ settings->ViewportPos = ImVec2ih(x, y); }
    else if (sscanf(line, "Collapsed=%d", &i) == 1)         { settings->Collapsed = (i != 0); }
    else if (sscanf(line, "DockId=0x%X,%d", &u1, &i) == 2)  { settings->DockId = u1; settings->DockOrder = i; }
    else if (sscanf(line, "DockId=0x%X", &u1) == 1)         { settings->DockId = u1; settings->DockOrder = -1; }
    else if (sscanf(line, "ClassId=0x%X", &u1) == 1)        { settings->ClassId = u1; }
}

// Apply to existing windows (if any)
static c_void WindowSettingsHandler_ApplyAll(ImGuiContext* ctx, ImGuiSettingsHandler*)
{
    ImGuiContext& g = *ctx;
    for (settings: *mut ImGuiWindowSettings = g.SettingsWindows.begin(); settings != null_mut(); settings = g.SettingsWindows.next_chunk(settings))
        if (settings->WantApply)
        {
            if (let mut window: *mut ImGuiWindow =  FindWindowByID(settings->ID))
                ApplyWindowSettings(window, settings);
            settings->WantApply = false;
        }
}

static c_void WindowSettingsHandler_WriteAll(ImGuiContext* ctx, ImGuiSettingsHandler* handler, ImGuiTextBuffer* bu0f32)
{
    // Gather data from windows that were active during this session
    // (if a window wasn't opened in this session we preserve its settings)
    ImGuiContext& g = *ctx;
    for (let i: c_int = 0; i != g.Windows.len(); i++)
    {
        let mut window: *mut ImGuiWindow =  g.Windows[i];
        if (window.Flags & ImGuiWindowFlags_NoSavedSettings)
            continue;

        settings: *mut ImGuiWindowSettings = (window.SettingsOffset != -1) ? g.SettingsWindows.ptr_from_offset(window.SettingsOffset) : FindWindowSettings(window.ID);
        if (!settings)
        {
            settings = CreateNewWindowSettings(window.Name);
            window.SettingsOffset = g.SettingsWindows.offset_from_ptr(settings);
        }
        // IM_ASSERT(settings->ID == window.ID);
        settings->Pos = ImVec2ih(window.Pos - window.ViewportPos);
        settings->Size = ImVec2ih(window.SizeFull);
        settings->ViewportId = window.ViewportId;
        settings->ViewportPos = ImVec2ih(window.ViewportPos);
        // IM_ASSERT(window.DockNode == NULL || window.DockNode->ID == window.DockId);
        settings->DockId = window.DockId;
        settings->ClassId = window.WindowClass.ClassId;
        settings->DockOrder = window.DockOrder;
        settings->Collapsed = window.Collapsed;
    }

    // Write to text buffer
    buf->reserve(buf->size() + g.SettingsWindows.size() * 6); // ballpark reserve
    for (settings: *mut ImGuiWindowSettings = g.SettingsWindows.begin(); settings != null_mut(); settings = g.SettingsWindows.next_chunk(settings))
    {
        let mut  settings_name: *const c_char = settings->GetName();
        buf->appendf("[%s][%s]\n", handler.TypeName, settings_name);
        if (settings->ViewportId != 0 && settings->ViewportId != IMGUI_VIEWPORT_DEFAULT_ID)
        {
            buf->appendf("ViewportPos=%d,%d\n", settings->ViewportPos.x, settings->ViewportPos.y);
            buf->appendf("ViewportId=0x%08X\n", settings->ViewportId);
        }
        if (settings->Pos.x != 0 || settings->Pos.y != 0 || settings->ViewportId == IMGUI_VIEWPORT_DEFAULT_ID)
            buf->appendf("Pos=%d,%d\n", settings->Pos.x, settings->Pos.y);
        if (settings->Size.x != 0 || settings->Size.y != 0)
            buf->appendf("Size=%d,%d\n", settings->Size.x, settings->Size.y);
        buf->appendf("Collapsed=%d\n", settings->Collapsed);
        if (settings->DockId != 0)
        {
            //buf->appendf("TabId=0x%08X\n", ImHashStr("#TAB", 4, settings->ID)); // window.TabId: this is not read back but writing it makes "debugging" the .ini data easier.
            if (settings->DockOrder == -1)
                buf->appendf("DockId=0x%08X\n", settings->DockId);
            else
                buf->appendf("DockId=0x%08X,%d\n", settings->DockId, settings->DockOrder);
            if (settings->ClassId != 0)
                buf->appendf("ClassId=0x%08X\n", settings->ClassId);
        }
        buf->append("\n");
    }
}


//-----------------------------------------------------------------------------
// [SECTION] VIEWPORTS, PLATFORM WINDOWS
//-----------------------------------------------------------------------------
// - GetMainViewport()
// - FindViewportByID()
// - FindViewportByPlatformHandle()
// - SetCurrentViewport() [Internal]
// - SetWindowViewport() [Internal]
// - GetWindowAlwaysWantOwnViewport() [Internal]
// - UpdateTryMergeWindowIntoHostViewport() [Internal]
// - UpdateTryMergeWindowIntoHostViewports() [Internal]
// - TranslateWindowsInViewport() [Internal]
// - ScaleWindowsInViewport() [Internal]
// - FindHoveredViewportFromPlatformWindowStack() [Internal]
// - UpdateViewportsNewFrame() [Internal]
// - UpdateViewportsEndFrame() [Internal]
// - AddUpdateViewport() [Internal]
// - WindowSelectViewport() [Internal]
// - WindowSyncOwnedViewport() [Internal]
// - UpdatePlatformWindows()
// - RenderPlatformWindowsDefault()
// - FindPlatformMonitorForPos() [Internal]
// - FindPlatformMonitorForRect() [Internal]
// - UpdateViewportPlatformMonitor() [Internal]
// - DestroyPlatformWindow() [Internal]
// - DestroyPlatformWindows()
//-----------------------------------------------------------------------------

ImGuiViewport* GetMainViewport()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.Viewports[0];
}

// FIXME: This leaks access to viewports not listed in PlatformIO.Viewports[]. Problematic? (#4236)
ImGuiViewport* FindViewportByID(id: ImGuiID)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    for (let n: c_int = 0; n < g.Viewports.len(); n++)
        if (g.Viewports[n]->ID == id)
            return g.Viewports[n];
    return null_mut();
}

ImGuiViewport* FindViewportByPlatformHandle(platform_handle: *mut c_void)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    for (let i: c_int = 0; i != g.Viewports.len(); i++)
        if (g.Viewports[i]->PlatformHandle == platform_handle)
            return g.Viewports[i];
    return null_mut();
}

c_void SetCurrentViewport(current_window: *mut ImGuiWindow, *mut ImGuiViewportP viewport)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    (c_void)current_window;

    if (viewport)
        viewport.LastFrameActive = g.FrameCount;
    if (g.CurrentViewport == viewport)
        return;
    g.CurrentDpiScale = viewport ? viewport.DpiScale : 1.0;
    g.CurrentViewport = viewport;
    //IMGUI_DEBUG_LOG_VIEWPORT("[viewport] SetCurrentViewport changed '%s' 0x%08X\n", current_window ? current_window.Name : NULL, viewport ? viewport.ID : 0);

    // Notify platform layer of viewport changes
    // FIXME-DPI: This is only currently used for experimenting with handling of multiple DPI
    if (g.CurrentViewport && g.PlatformIO.Platform_OnChangedViewport)
        g.PlatformIO.Platform_OnChangedViewport(g.CurrentViewport);
}

c_void SetWindowViewport(window: *mut ImGuiWindow, *mut ImGuiViewportP viewport)
{
    // Abandon viewport
    if (window.ViewportOwned && window.Viewport.Window == window)
        window.Viewport.Size = ImVec2::new(0.0, 0.0);

    window.Viewport = viewport;
    window.ViewportId = viewport.ID;
    window.ViewportOwned = (viewport.Window == window);
}

static GetWindowAlwaysWantOwnViewport: bool(window: *mut ImGuiWindow)
{
    // Tooltips and menus are not automatically forced into their own viewport when the NoMerge flag is set, however the multiplication of viewports makes them more likely to protrude and create their own.
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.IO.ConfigViewportsNoAutoMerge || (window.WindowClass.ViewportFlagsOverrideSet & ImGuiViewportFlags_NoAutoMerge))
        if (g.ConfigFlagsCurrFrame & ImGuiConfigFlags_ViewportsEnable)
            if (!window.DockIsActive)
                if ((window.Flags & (ImGuiWindowFlags_ChildWindow | ImGuiWindowFlags_ChildMenu | ImGuiWindowFlags_Tooltip)) == 0)
                    if ((window.Flags & ImGuiWindowFlags_Popup) == 0 || (window.Flags & ImGuiWindowFlags_Modal) != 0)
                        return true;
    return false;
}

static UpdateTryMergeWindowIntoHostViewport: bool(window: *mut ImGuiWindow, *mut ImGuiViewportP viewport)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (window.Viewport == viewport)
        return false;
    if ((viewport.Flags & ImGuiViewportFlags_CanHostOtherWindows) == 0)
        return false;
    if ((viewport.Flags & ImGuiViewportFlags_Minimized) != 0)
        return false;
    if (!viewport.GetMainRect().Contains(window.Rect()))
        return false;
    if (GetWindowAlwaysWantOwnViewport(window))
        return false;

    // FIXME: Can't use g.WindowsFocusOrder[] for root windows only as we care about Z order. If we maintained a DisplayOrder along with FocusOrder we could..
    for (let n: c_int = 0; n < g.Windows.len(); n++)
    {
        let mut window_behind: *mut ImGuiWindow =  g.Windows[n];
        if (window_behind == window)
            break;
        if (window_behind->WasActive && window_behind->ViewportOwned && !(window_behind->Flags & ImGuiWindowFlags_ChildWindow))
            if (window_behind->Viewport.GetMainRect().Overlaps(window.Rect()))
                return false;
    }

    // Move to the existing viewport, Move child/hosted windows as well (FIXME-OPT: iterate child)
    let mut old_viewport: *mut ImGuiViewport =  window.Viewport;
    if (window.ViewportOwned)
        for (let n: c_int = 0; n < g.Windows.len(); n++)
            if (g.Windows[n]->Viewport == old_viewport)
                SetWindowViewport(g.Windows[n], viewport);
    SetWindowViewport(window, viewport);
    BringWindowToDisplayFront(window);

    return true;
}

// FIXME: handle 0 to N host viewports
static UpdateTryMergeWindowIntoHostViewports: bool(window: *mut ImGuiWindow)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return UpdateTryMergeWindowIntoHostViewport(window, g.Viewports[0]);
}

// Translate Dear ImGui windows when a Host Viewport has been moved
// (This additionally keeps windows at the same place when ImGuiConfigFlags_ViewportsEnable is toggled!)
c_void TranslateWindowsInViewport(*mut ImGuiViewportP viewport, old_pos: &ImVec2, new_pos: &ImVec2)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(viewport.Window == NULL && (viewport.Flags & ImGuiViewportFlags_CanHostOtherWindows));

    // 1) We test if ImGuiConfigFlags_ViewportsEnable was just toggled, which allows us to conveniently
    // translate imgui windows from OS-window-local to absolute coordinates or vice-versa.
    // 2) If it's not going to fit into the new size, keep it at same absolute position.
    // One problem with this is that most Win32 applications doesn't update their render while dragging,
    // and so the window will appear to teleport when releasing the mouse.
    let translate_all_windows: bool = (g.ConfigFlagsCurrFrame & ImGuiConfigFlags_ViewportsEnable) != (g.ConfigFlagsLastFrame & ImGuiConfigFlags_ViewportsEnable);
    let mut test_still_fit_rect: ImRect = ImRect::new(old_pos, old_pos + viewport.Size);
    let delta_pos: ImVec2 = new_pos - old_pos;
    for (let window_n: c_int = 0; window_n < g.Windows.len(); window_n++) // FIXME-OPT
        if (translate_all_windows || (g.Windows[window_n]->Viewport == viewport && test_still_fit_rect.Contains(g.Windows[window_n]->Rect())))
            TranslateWindow(g.Windows[window_n], delta_pos);
}

// Scale all windows (position, size). Use when e.g. changing DPI. (This is a lossy operation!)
c_void ScaleWindowsInViewport(*mut ImGuiViewportP viewport,scale: c_float)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (viewport.Window)
    {
        ScaleWindow(viewport.Window, scale);
    }
    else
    {
        for (let i: c_int = 0; i != g.Windows.len(); i++)
            if (g.Windows[i]->Viewport == viewport)
                ScaleWindow(g.Windows[i], scale);
    }
}

// If the backend doesn't set MouseLastHoveredViewport or doesn't honor ImGuiViewportFlags_NoInputs, we do a search ourselves.
// A) It won't take account of the possibility that non-imgui windows may be in-between our dragged window and our target window.
// B) It requires Platform_GetWindowFocus to be implemented by backend.
*mut ImGuiViewportP FindHoveredViewportFromPlatformWindowStack(mouse_platform_pos: &ImVec2)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut best_candidate: *mut ImGuiViewport =  null_mut();
    for (let n: c_int = 0; n < g.Viewports.len(); n++)
    {
        let mut viewport: *mut ImGuiViewport =  g.Viewports[n];
        if (!(viewport.Flags & (ImGuiViewportFlags_NoInputs | ImGuiViewportFlags_Minimized)) && viewport.GetMainRect().Contains(mouse_platform_pos))
            if (best_candidate == null_mut() || best_candidate.LastFrontMostStampCount < viewport.LastFrontMostStampCount)
                best_candidate = viewport;
    }
    return best_candidate;
}

// Update viewports and monitor infos
// Note that this is running even if 'ImGuiConfigFlags_ViewportsEnable' is not set, in order to clear unused viewports (if any) and update monitor info.
static c_void UpdateViewportsNewFrame()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.PlatformIO.Viewports.Size <= g.Viewports.Size);

    // Update Minimized status (we need it first in order to decide if we'll apply Pos/Size of the main viewport)
    let viewports_enabled: bool = (g.ConfigFlagsCurrFrame & ImGuiConfigFlags_ViewportsEnable) != 0;
    if (viewports_enabled)
    {
        for (let n: c_int = 0; n < g.Viewports.len(); n++)
        {
            let mut viewport: *mut ImGuiViewport =  g.Viewports[n];
            let platform_funcs_available: bool = viewport.PlatformWindowCreated;
            if (g.PlatformIO.Platform_GetWindowMinimized && platform_funcs_available)
            {
                let mut minimized: bool =  g.PlatformIO.Platform_GetWindowMinimized(viewport);
                if (minimized)
                    viewport.Flags |= ImGuiViewportFlags_Minimized;
                else
                    viewport.Flags &= !ImGuiViewportFlags_Minimized;
            }
        }
    }

    // Create/update main viewport with current platform position.
    // FIXME-VIEWPORT: Size is driven by backend/user code for backward-compatibility but we should aim to make this more consistent.
    let mut main_viewport: *mut ImGuiViewport =  g.Viewports[0];
    // IM_ASSERT(main_viewport.ID == IMGUI_VIEWPORT_DEFAULT_ID);
    // IM_ASSERT(main_viewport.Window == NULL);
    let main_viewport_pos: ImVec2 = viewports_enabled ? g.PlatformIO.Platform_GetWindowPos(main_viewport) : ImVec2::new(0.0, 0.0);
    let main_viewport_size: ImVec2 = g.IO.DisplaySize;
    if (viewports_enabled && (main_viewport.Flags & ImGuiViewportFlags_Minimized))
    {
        main_viewport_pos = main_viewport.Pos;    // Preserve last pos/size when minimized (FIXME: We don't do the same for Size outside of the viewport path)
        main_viewport_size = main_viewport.Size;
    }
    AddUpdateViewport(null_mut(), IMGUI_VIEWPORT_DEFAULT_ID, main_viewport_pos, main_viewport_size, ImGuiViewportFlags_OwnedByApp | ImGuiViewportFlags_CanHostOtherWindows);

    g.CurrentDpiScale = 0.0;
    g.CurrentViewport= null_mut();
    g.MouseViewport= null_mut();
    for (let n: c_int = 0; n < g.Viewports.len(); n++)
    {
        let mut viewport: *mut ImGuiViewport =  g.Viewports[n];
        viewport.Idx = n;

        // Erase unused viewports
        if (n > 0 && viewport.LastFrameActive < g.FrameCount - 2)
        {
            DestroyViewport(viewport);
            n-= 1;
            continue;
        }

        let platform_funcs_available: bool = viewport.PlatformWindowCreated;
        if (viewports_enabled)
        {
            // Update Position and Size (from Platform Window to ImGui) if requested.
            // We do it early in the frame instead of waiting for UpdatePlatformWindows() to avoid a frame of lag when moving/resizing using OS facilities.
            if (!(viewport.Flags & ImGuiViewportFlags_Minimized) && platform_funcs_available)
            {
                // Viewport->WorkPos and WorkSize will be updated below
                if (viewport.PlatformRequestMove)
                    viewport.Pos = viewport.LastPlatformPos = g.PlatformIO.Platform_GetWindowPos(viewport);
                if (viewport.PlatformRequestResize)
                    viewport.Size = viewport.LastPlatformSize = g.PlatformIO.Platform_GetWindowSize(viewport);
            }
        }

        // Update/copy monitor info
        UpdateViewportPlatformMonitor(viewport);

        // Lock down space taken by menu bars and status bars, reset the offset for functions like BeginMainMenuBar() to alter them again.
        viewport.WorkOffsetMin = viewport.BuildWorkOffsetMin;
        viewport.WorkOffsetMax = viewport.BuildWorkOffsetMax;
        viewport.BuildWorkOffsetMin = viewport.BuildWorkOffsetMax = ImVec2::new(0.0, 0.0);
        viewport.UpdateWorkRect();

        // Reset alpha every frame. Users of transparency (docking) needs to request a lower alpha back.
        viewport.Alpha = 1.0;

        // Translate Dear ImGui windows when a Host Viewport has been moved
        // (This additionally keeps windows at the same place when ImGuiConfigFlags_ViewportsEnable is toggled!)
        let viewport_delta_pos: ImVec2 = viewport.Pos - viewport.LastPos;
        if ((viewport.Flags & ImGuiViewportFlags_CanHostOtherWindows) && (viewport_delta_pos.x != 0.0 || viewport_delta_pos.y != 0.0))
            TranslateWindowsInViewport(viewport, viewport.LastPos, viewport.Pos);

        // Update DPI scale
        let mut new_dpi_scale: c_float = 0.0;
        if (g.PlatformIO.Platform_GetWindowDpiScale && platform_funcs_available)
            new_dpi_scale = g.PlatformIO.Platform_GetWindowDpiScale(viewport);
        else if (viewport.PlatformMonitor != -1)
            new_dpi_scale = g.PlatformIO.Monitors[viewport.PlatformMonitor].DpiScale;
        else
            new_dpi_scale = (viewport.DpiScale != 0.0) ? viewport.DpiScale : 1.0;
        if (viewport.DpiScale != 0.0 && new_dpi_scale != viewport.DpiScale)
        {
            let scale_factor: c_float =  new_dpi_scale / viewport.DpiScale;
            if (g.IO.ConfigFlags & ImGuiConfigFlags_DpiEnableScaleViewports)
                ScaleWindowsInViewport(viewport, scale_factor);
            //if (viewport == GetMainViewport())
            //    g.PlatformInterface.SetWindowSize(viewport, viewport.Size * scale_factor);

            // Scale our window moving pivot so that the window will rescale roughly around the mouse position.
            // FIXME-VIEWPORT: This currently creates a resizing feedback loop when a window is straddling a DPI transition border.
            // (Minor: since our sizes do not perfectly linearly scale, deferring the click offset scale until we know the actual window scale ratio may get us slightly more precise mouse positioning.)
            //if (g.MovingWindow != NULL && g.Movingwindow.Viewport == viewport)
            //    g.ActiveIdClickOffset = ImFloor(g.ActiveIdClickOffset * scale_factor);
        }
        viewport.DpiScale = new_dpi_scale;
    }

    // Update fallback monitor
    if (g.PlatformIO.Monitors.Size == 0)
    {
        ImGuiPlatformMonitor* monitor = &g.FallbackMonitor;
        monitor->MainPos = main_viewport.Pos;
        monitor->MainSize = main_viewport.Size;
        monitor->WorkPos = main_viewport.WorkPos;
        monitor->WorkSize = main_viewport.WorkSize;
        monitor->DpiScale = main_viewport.DpiScale;
    }

    if (!viewports_enabled)
    {
        g.MouseViewport = main_viewport;
        return;
    }

    // Mouse handling: decide on the actual mouse viewport for this frame between the active/focused viewport and the hovered viewport.
    // Note that 'viewport_hovered' should skip over any viewport that has the ImGuiViewportFlags_NoInputs flags set.
    let mut viewport_hovered: *mut ImGuiViewport =  null_mut();
    if (g.IO.BackendFlags & ImGuiBackendFlags_HasMouseHoveredViewport)
    {
        viewport_hovered = g.IO.MouseHoveredViewport ? FindViewportByID(g.IO.MouseHoveredViewport) : null_mut();
        if (viewport_hovered && (viewport_hovered->Flags & ImGuiViewportFlags_NoInputs))
            viewport_hovered = FindHoveredViewportFromPlatformWindowStack(g.IO.MousePos); // Backend failed to handle _NoInputs viewport: revert to our fallback.
    }
    else
    {
        // If the backend doesn't know how to honor ImGuiViewportFlags_NoInputs, we do a search ourselves. Note that this search:
        // A) won't take account of the possibility that non-imgui windows may be in-between our dragged window and our target window.
        // B) won't take account of how the backend apply parent<>child relationship to secondary viewports, which affects their Z order.
        // C) uses LastFrameAsRefViewport as a flawed replacement for the last time a window was focused (we could/should fix that by introducing Focus functions in PlatformIO)
        viewport_hovered = FindHoveredViewportFromPlatformWindowStack(g.IO.MousePos);
    }
    if (viewport_hovered != null_mut())
        g.MouseLastHoveredViewport = viewport_hovered;
    else if (g.MouseLastHoveredViewport == null_mut())
        g.MouseLastHoveredViewport = g.Viewports[0];

    // Update mouse reference viewport
    // (when moving a window we aim at its viewport, but this will be overwritten below if we go in drag and drop mode)
    // (MovingViewport->Viewport will be NULL in the rare situation where the window disappared while moving, set UpdateMouseMovingWindowNewFrame() for details)
    if (g.MovingWindow && g.Movingwindow.Viewport)
        g.MouseViewport = g.Movingwindow.Viewport;
    else
        g.MouseViewport = g.MouseLastHoveredViewport;

    // When dragging something, always refer to the last hovered viewport.
    // - when releasing a moving window we will revert to aiming behind (at viewport_hovered)
    // - when we are between viewports, our dragged preview will tend to show in the last viewport _even_ if we don't have tooltips in their viewports (when lacking monitor info)
    // - consider the case of holding on a menu item to browse child menus: even thou a mouse button is held, there's no active id because menu items only react on mouse release.
    // FIXME-VIEWPORT: This is essentially broken, when ImGuiBackendFlags_HasMouseHoveredViewport is set we want to trust when viewport_hovered==NULL and use that.
    let is_mouse_dragging_with_an_expected_destination: bool = g.DragDropActive;
    if (is_mouse_dragging_with_an_expected_destination && viewport_hovered == null_mut())
        viewport_hovered = g.MouseLastHoveredViewport;
    if (is_mouse_dragging_with_an_expected_destination || g.ActiveId == 0 || !IsAnyMouseDown())
        if (viewport_hovered != null_mut() && viewport_hovered != g.MouseViewport && !(viewport_hovered->Flags & ImGuiViewportFlags_NoInputs))
            g.MouseViewport = viewport_hovered;

    // IM_ASSERT(g.MouseViewport != NULL);
}

// Update user-facing viewport list (g.Viewports -> g.PlatformIO.Viewports after filtering out some)
static c_void UpdateViewportsEndFrame()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.PlatformIO.Viewports.clear();
    for (let i: c_int = 0; i < g.Viewports.len(); i++)
    {
        let mut viewport: *mut ImGuiViewport =  g.Viewports[i];
        viewport.LastPos = viewport.Pos;
        if (viewport.LastFrameActive < g.FrameCount || viewport.Size.x <= 0.0 || viewport.Size.y <= 0.0)
            if (i > 0) // Always include main viewport in the list
                continue;
        if (viewport.Window && !IsWindowActiveAndVisible(viewport.Window))
            continue;
        if (i > 0)
            // IM_ASSERT(viewport.Window != NULL);
        g.PlatformIO.Viewports.push(viewport);
    }
    g.Viewports[0]->ClearRequestFlags(); // Clear main viewport flags because UpdatePlatformWindows() won't do it and may not even be called
}

// FIXME: We should ideally refactor the system to call this every frame (we currently don't)
*mut ImGuiViewportP AddUpdateViewport(window: *mut ImGuiWindow, id: ImGuiID, pos: &ImVec2, size: &ImVec2, ImGuiViewportFlags flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(id != 0);

    flags |= ImGuiViewportFlags_IsPlatformWindow;
    if (window != null_mut())
    {
        if (g.MovingWindow && g.Movingwindow.RootWindowDockTree == window)
            flags |= ImGuiViewportFlags_NoInputs | ImGuiViewportFlags_NoFocusOnAppearing;
        if ((window.Flags & ImGuiWindowFlags_NoMouseInputs) && (window.Flags & ImGuiWindowFlags_NoNavInputs))
            flags |= ImGuiViewportFlags_NoInputs;
        if (window.Flags & ImGuiWindowFlags_NoFocusOnAppearing)
            flags |= ImGuiViewportFlags_NoFocusOnAppearing;
    }

    let mut viewport: *mut ImGuiViewport =  FindViewportByID(id);
    if (viewport)
    {
        // Always update for main viewport as we are already pulling correct platform pos/size (see #4900)
        if (!viewport.PlatformRequestMove || viewport.ID == IMGUI_VIEWPORT_DEFAULT_ID)
            viewport.Pos = pos;
        if (!viewport.PlatformRequestResize || viewport.ID == IMGUI_VIEWPORT_DEFAULT_ID)
            viewport.Size = size;
        viewport.Flags = flags | (viewport.Flags & ImGuiViewportFlags_Minimized); // Preserve existing flags
    }
    else
    {
        // New viewport
        viewport = IM_NEW(ImGuiViewportP)();
        viewport.ID = id;
        viewport.Idx = g.Viewports.len();
        viewport.Pos = viewport.LastPos = pos;
        viewport.Size = size;
        viewport.Flags = flags;
        UpdateViewportPlatformMonitor(viewport);
        g.Viewports.push(viewport);
        IMGUI_DEBUG_LOG_VIEWPORT("[viewport] Add Viewport %08X '%s'\n", id, window ? window.Name : "<NULL>");

        // We normally setup for all viewports in NewFrame() but here need to handle the mid-frame creation of a new viewport.
        // We need to extend the fullscreen clip rect so the OverlayDrawList clip is correct for that the first frame
        g.DrawListSharedData.ClipRectFullscreen.x = ImMin(g.DrawListSharedData.ClipRectFullscreen.x, viewport.Pos.x);
        g.DrawListSharedData.ClipRectFullscreen.y = ImMin(g.DrawListSharedData.ClipRectFullscreen.y, viewport.Pos.y);
        g.DrawListSharedData.ClipRectFullscreen.z = ImMax(g.DrawListSharedData.ClipRectFullscreen.z, viewport.Pos.x + viewport.Size.x);
        g.DrawListSharedData.ClipRectFullscreen.w = ImMax(g.DrawListSharedData.ClipRectFullscreen.w, viewport.Pos.y + viewport.Size.y);

        // Store initial DpiScale before the OS platform window creation, based on expected monitor data.
        // This is so we can select an appropriate font size on the first frame of our window lifetime
        if (viewport.PlatformMonitor != -1)
            viewport.DpiScale = g.PlatformIO.Monitors[viewport.PlatformMonitor].DpiScale;
    }

    viewport.Window = window;
    viewport.LastFrameActive = g.FrameCount;
    viewport.UpdateWorkRect();
    // IM_ASSERT(window == NULL || viewport.ID == window.ID);

    if (window != null_mut())
        window.ViewportOwned = true;

    return viewport;
}

static c_void DestroyViewport(*mut ImGuiViewportP viewport)
{
    // Clear references to this viewport in windows (window.ViewportId becomes the master data)
    let g = GImGui; // ImGuiContext& g = *GImGui;
    for (let window_n: c_int = 0; window_n < g.Windows.len(); window_n++)
    {
        let mut window: *mut ImGuiWindow =  g.Windows[window_n];
        if (window.Viewport != viewport)
            continue;
        window.Viewport= null_mut();
        window.ViewportOwned = false;
    }
    if (viewport == g.MouseLastHoveredViewport)
        g.MouseLastHoveredViewport= null_mut();

    // Destroy
    IMGUI_DEBUG_LOG_VIEWPORT("[viewport] Delete Viewport %08X '%s'\n", viewport.ID, viewport.Window ? viewport.window.Name : "n/a");
    DestroyPlatformWindow(viewport); // In most circumstances the platform window will already be destroyed here.
    // IM_ASSERT(g.PlatformIO.Viewports.contains(viewport) == false);
    // IM_ASSERT(g.Viewports[viewport.Idx] == viewport);
    g.Viewports.erase(g.Viewports.Data + viewport.Idx);
    IM_DELETE(viewport);
}

// FIXME-VIEWPORT: This is all super messy and ought to be clarified or rewritten.
static c_void WindowSelectViewport(window: *mut ImGuiWindow)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    flags: ImGuiWindowFlags = window.Flags;
    window.ViewportAllowPlatformMonitorExtend = -1;

    // Restore main viewport if multi-viewport is not supported by the backend
    let mut main_viewport: *mut ImGuiViewport =  (*mut c_void)GetMainViewport();
    if (!(g.ConfigFlagsCurrFrame & ImGuiConfigFlags_ViewportsEnable))
    {
        SetWindowViewport(window, main_viewport);
        return;
    }
    window.ViewportOwned = false;

    // Appearing popups reset their viewport so they can inherit again
    if ((flags & (ImGuiWindowFlags_Popup | ImGuiWindowFlags_Tooltip)) && window.Appearing)
    {
        window.Viewport= null_mut();
        window.ViewportId = 0;
    }

    if ((g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasViewport) == 0)
    {
        // By default inherit from parent window
        if (window.Viewport == null_mut() && window.ParentWindow && (!window.Parentwindow.IsFallbackWindow || window.Parentwindow.WasActive))
            window.Viewport = window.Parentwindow.Viewport;

        // Attempt to restore saved viewport id (= window that hasn't been activated yet), try to restore the viewport based on saved 'window.ViewportPos' restored from .ini file
        if (window.Viewport == null_mut() && window.ViewportId != 0)
        {
            window.Viewport = FindViewportByID(window.ViewportId);
            if (window.Viewport == null_mut() && window.ViewportPos.x != f32::MAX && window.ViewportPos.y != f32::MAX)
                window.Viewport = AddUpdateViewport(window, window.ID, window.ViewportPos, window.Size, ImGuiViewportFlags_None);
        }
    }

    let mut lock_viewport: bool =  false;
    if (g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasViewport)
    {
        // Code explicitly request a viewport
        window.Viewport = FindViewportByID(g.NextWindowData.ViewportId);
        window.ViewportId = g.NextWindowData.ViewportId; // Store ID even if Viewport isn't resolved yet.
        lock_viewport = true;
    }
    else if ((flags & ImGuiWindowFlags_ChildWindow) || (flags & ImGuiWindowFlags_ChildMenu))
    {
        // Always inherit viewport from parent window
        if (window.DockNode && window.DockNode.HostWindow)
            // IM_ASSERT(window.DockNode->Hostwindow.Viewport == window.Parentwindow.Viewport);
        window.Viewport = window.Parentwindow.Viewport;
    }
    else if (window.DockNode && window.DockNode.HostWindow)
    {
        // This covers the "always inherit viewport from parent window" case for when a window reattach to a node that was just created mid-frame
        window.Viewport = window.DockNode.Hostwindow.Viewport;
    }
    else if (flags & ImGuiWindowFlags_Tooltip)
    {
        window.Viewport = g.MouseViewport;
    }
    else if (GetWindowAlwaysWantOwnViewport(window))
    {
        window.Viewport = AddUpdateViewport(window, window.ID, window.Pos, window.Size, ImGuiViewportFlags_None);
    }
    else if (g.MovingWindow && g.Movingwindow.RootWindowDockTree == window && IsMousePosValid())
    {
        if (window.Viewport != null_mut() && window.Viewport.Window == window)
            window.Viewport = AddUpdateViewport(window, window.ID, window.Pos, window.Size, ImGuiViewportFlags_None);
    }
    else
    {
        // Merge into host viewport?
        // We cannot test window.ViewportOwned as it set lower in the function.
        // Testing (g.ActiveId == 0 || g.ActiveIdAllowOverlap) to avoid merging during a short-term widget interaction. Main intent was to avoid during resize (see #4212)
        let mut try_to_merge_into_host_viewport: bool =  (window.Viewport && window == window.Viewport.Window && (g.ActiveId == 0 || g.ActiveIdAllowOverlap));
        if (try_to_merge_into_host_viewport)
            UpdateTryMergeWindowIntoHostViewports(window);
    }

    // Fallback: merge in default viewport if z-order matches, otherwise create a new viewport
    if (window.Viewport == null_mut())
        if (!UpdateTryMergeWindowIntoHostViewport(window, main_viewport))
            window.Viewport = AddUpdateViewport(window, window.ID, window.Pos, window.Size, ImGuiViewportFlags_None);

    // Mark window as allowed to protrude outside of its viewport and into the current monitor
    if (!lock_viewport)
    {
        if (flags & (ImGuiWindowFlags_Tooltip | ImGuiWindowFlags_Popup))
        {
            // We need to take account of the possibility that mouse may become invalid.
            // Popups/Tooltip always set ViewportAllowPlatformMonitorExtend so GetWindowAllowedExtentRect() will return full monitor bounds.
            let mouse_ref: ImVec2 = (flags & ImGuiWindowFlags_Tooltip) ? g.IO.MousePos : g.BeginPopupStack.last().unwrap().OpenMousePos;
            let mut use_mouse_ref: bool =  (g.NavDisableHighlight || !g.NavDisableMouseHover || !g.NavWindow);
            let mut mouse_valid: bool =  IsMousePosValid(&mouse_re0f32);
            if ((window.Appearing || (flags & (ImGuiWindowFlags_Tooltip | ImGuiWindowFlags_ChildMenu))) && (!use_mouse_ref || mouse_valid))
                window.ViewportAllowPlatformMonitorExtend = FindPlatformMonitorForPos((use_mouse_ref && mouse_valid) ? mouse_ref : NavCalcPreferredRefPos());
            else
                window.ViewportAllowPlatformMonitorExtend = window.Viewport.PlatformMonitor;
        }
        else if (window.Viewport && window != window.Viewport.Window && window.Viewport.Window && !(flags & ImGuiWindowFlags_ChildWindow) && window.DockNode == null_mut())
        {
            // When called from Begin() we don't have access to a proper version of the Hidden flag yet, so we replicate this code.
            let will_be_visible: bool = (window.DockIsActive && !window.DockTabIsVisible) ? false : true;
            if ((window.Flags & ImGuiWindowFlags_DockNodeHost) && window.Viewport.LastFrameActive < g.FrameCount && will_be_visible)
            {
                // Steal/transfer ownership
                IMGUI_DEBUG_LOG_VIEWPORT("[viewport] Window '%s' steal Viewport %08X from Window '%s'\n", window.Name, window.Viewport.ID, window.Viewport.window.Name);
                window.Viewport.Window = window;
                window.Viewport.ID = window.ID;
                window.Viewport.LastNameHash = 0;
            }
            else if (!UpdateTryMergeWindowIntoHostViewports(window)) // Merge?
            {
                // New viewport
                window.Viewport = AddUpdateViewport(window, window.ID, window.Pos, window.Size, ImGuiViewportFlags_NoFocusOnAppearing);
            }
        }
        else if (window.ViewportAllowPlatformMonitorExtend < 0 && (flags & ImGuiWindowFlags_ChildWindow) == 0)
        {
            // Regular (non-child, non-popup) windows by default are also allowed to protrude
            // Child windows are kept contained within their parent.
            window.ViewportAllowPlatformMonitorExtend = window.Viewport.PlatformMonitor;
        }
    }

    // Update flags
    window.ViewportOwned = (window == window.Viewport.Window);
    window.ViewportId = window.Viewport.ID;

    // If the OS window has a title bar, hide our imgui title bar
    //if (window.ViewportOwned && !(window.Viewport->Flags & ImGuiViewportFlags_NoDecoration))
    //    window.Flags |= ImGuiWindowFlags_NoTitleBar;
}

c_void WindowSyncOwnedViewport(window: *mut ImGuiWindow, parent_window_in_stack: *mut ImGuiWindow)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    let mut viewport_rect_changed: bool =  false;

    // Synchronize window --> viewport in most situations
    // Synchronize viewport -> window in case the platform window has been moved or resized from the OS/WM
    if (window.Viewport.PlatformRequestMove)
    {
        window.Pos = window.Viewport.Pos;
        MarkIniSettingsDirty(window);
    }
    else if (memcmp(&window.Viewport.Pos, &window.Pos, sizeof(window.Pos)) != 0)
    {
        viewport_rect_changed = true;
        window.Viewport.Pos = window.Pos;
    }

    if (window.Viewport.PlatformRequestResize)
    {
        window.Size = window.SizeFull = window.Viewport.Size;
        MarkIniSettingsDirty(window);
    }
    else if (memcmp(&window.Viewport.Size, &window.Size, sizeof(window.Size)) != 0)
    {
        viewport_rect_changed = true;
        window.Viewport.Size = window.Size;
    }
    window.Viewport.UpdateWorkRect();

    // The viewport may have changed monitor since the global update in UpdateViewportsNewFrame()
    // Either a SetNextWindowPos() call in the current frame or a SetWindowPos() call in the previous frame may have this effect.
    if (viewport_rect_changed)
        UpdateViewportPlatformMonitor(window.Viewport);

    // Update common viewport flags
    const ImGuiViewportFlags viewport_flags_to_clear = ImGuiViewportFlags_TopMost | ImGuiViewportFlags_NoTaskBarIcon | ImGuiViewportFlags_NoDecoration | ImGuiViewportFlags_NoRendererClear;
    ImGuiViewportFlags viewport_flags = window.Viewport.Flags & !viewport_flags_to_clear;
    window_flags: ImGuiWindowFlags = window.Flags;
    let is_modal: bool = (window_flags & ImGuiWindowFlags_Modal) != 0;
    let is_short_lived_floating_window: bool = (window_flags & (ImGuiWindowFlags_ChildMenu | ImGuiWindowFlags_Tooltip | ImGuiWindowFlags_Popup)) != 0;
    if (window_flags & ImGuiWindowFlags_Tooltip)
        viewport_flags |= ImGuiViewportFlags_TopMost;
    if ((g.IO.ConfigViewportsNoTaskBarIcon || is_short_lived_floating_window) && !is_modal)
        viewport_flags |= ImGuiViewportFlags_NoTaskBarIcon;
    if (g.IO.ConfigViewportsNoDecoration || is_short_lived_floating_window)
        viewport_flags |= ImGuiViewportFlags_NoDecoration;

    // Not correct to set modal as topmost because:
    // - Because other popups can be stacked above a modal (e.g. combo box in a modal)
    // - ImGuiViewportFlags_TopMost is currently handled different in backends: in Win32 it is "appear top most" whereas in GLFW and SDL it is "stay topmost"
    //if (flags & ImGuiWindowFlags_Modal)
    //    viewport_flags |= ImGuiViewportFlags_TopMost;

    // For popups and menus that may be protruding out of their parent viewport, we enable _NoFocusOnClick so that clicking on them
    // won't steal the OS focus away from their parent window (which may be reflected in OS the title bar decoration).
    // Setting _NoFocusOnClick would technically prevent us from bringing back to front in case they are being covered by an OS window from a different app,
    // but it shouldn't be much of a problem considering those are already popups that are closed when clicking elsewhere.
    if (is_short_lived_floating_window && !is_modal)
        viewport_flags |= ImGuiViewportFlags_NoFocusOnAppearing | ImGuiViewportFlags_NoFocusOnClick;

    // We can overwrite viewport flags using ImGuiWindowClass (advanced users)
    if (window.WindowClass.ViewportFlagsOverrideSet)
        viewport_flags |= window.WindowClass.ViewportFlagsOverrideSet;
    if (window.WindowClass.ViewportFlagsOverrideClear)
        viewport_flags &= !window.WindowClass.ViewportFlagsOverrideClear;

    // We can also tell the backend that clearing the platform window won't be necessary,
    // as our window background is filling the viewport and we have disabled BgAlpha.
    // FIXME: Work on support for per-viewport transparency (#2766)
    if (!(window_flags & ImGuiWindowFlags_NoBackground))
        viewport_flags |= ImGuiViewportFlags_NoRendererClear;

    window.Viewport.Flags = viewport_flags;

    // Update parent viewport ID
    // (the !IsFallbackWindow test mimic the one done in WindowSelectViewport())
    if (window.WindowClass.ParentViewportId != (ImGuiID)-1)
        window.Viewport.ParentViewportId = window.WindowClass.ParentViewportId;
    else if ((window_flags & (ImGuiWindowFlags_Popup | ImGuiWindowFlags_Tooltip)) && parent_window_in_stack && (!parent_window_in_stack->IsFallbackWindow || parent_window_in_stack->WasActive))
        window.Viewport.ParentViewportId = parent_window_in_stack->Viewport.ID;
    else
        window.Viewport.ParentViewportId = g.IO.ConfigViewportsNoDefaultParent ? 0 : IMGUI_VIEWPORT_DEFAULT_ID;
}

// Called by user at the end of the main loop, after EndFrame()
// This will handle the creation/update of all OS windows via function defined in the ImGuiPlatformIO api.
c_void UpdatePlatformWindows()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.FrameCountEnded == g.FrameCount && "Forgot to call Render() or EndFrame() before UpdatePlatformWindows()?");
    // IM_ASSERT(g.FrameCountPlatformEnded < g.FrameCount);
    g.FrameCountPlatformEnded = g.FrameCount;
    if (!(g.ConfigFlagsCurrFrame & ImGuiConfigFlags_ViewportsEnable))
        return;

    // Create/resize/destroy platform windows to match each active viewport.
    // Skip the main viewport (index 0), which is always fully handled by the application!
    for (let i: c_int = 1; i < g.Viewports.len(); i++)
    {
        let mut viewport: *mut ImGuiViewport =  g.Viewports[i];

        // Destroy platform window if the viewport hasn't been submitted or if it is hosting a hidden window
        // (the implicit/fallback Debug##Default window will be registering its viewport then be disabled, causing a dummy DestroyPlatformWindow to be made each frame)
        let mut destroy_platform_window: bool =  false;
        destroy_platform_window |= (viewport.LastFrameActive < g.FrameCount - 1);
        destroy_platform_window |= (viewport.Window && !IsWindowActiveAndVisible(viewport.Window));
        if (destroy_platform_window)
        {
            DestroyPlatformWindow(viewport);
            continue;
        }

        // New windows that appears directly in a new viewport won't always have a size on their first frame
        if (viewport.LastFrameActive < g.FrameCount || viewport.Size.x <= 0 || viewport.Size.y <= 0)
            continue;

        // Create window
        let mut is_new_platform_window: bool =  (viewport.PlatformWindowCreated == false);
        if (is_new_platform_window)
        {
            IMGUI_DEBUG_LOG_VIEWPORT("[viewport] Create Platform Window %08X '%s'\n", viewport.ID, viewport.Window ? viewport.window.Name : "n/a");
            g.PlatformIO.Platform_CreateWindow(viewport);
            if (g.PlatformIO.Renderer_CreateWindow != null_mut())
                g.PlatformIO.Renderer_CreateWindow(viewport);
            viewport.LastNameHash = 0;
            viewport.LastPlatformPos = viewport.LastPlatformSize = ImVec2::new(f32::MAX, f32::MAX); // By clearing those we'll enforce a call to Platform_SetWindowPos/Size below, before Platform_ShowWindow (FIXME: Is that necessary?)
            viewport.LastRendererSize = viewport.Size;                                       // We don't need to call Renderer_SetWindowSize() as it is expected Renderer_CreateWindow() already did it.
            viewport.PlatformWindowCreated = true;
        }

        // Apply Position and Size (from ImGui to Platform/Renderer backends)
        if ((viewport.LastPlatformPos.x != viewport.Pos.x || viewport.LastPlatformPos.y != viewport.Pos.y) && !viewport.PlatformRequestMove)
            g.PlatformIO.Platform_SetWindowPos(viewport, viewport.Pos);
        if ((viewport.LastPlatformSize.x != viewport.Size.x || viewport.LastPlatformSize.y != viewport.Size.y) && !viewport.PlatformRequestResize)
            g.PlatformIO.Platform_SetWindowSize(viewport, viewport.Size);
        if ((viewport.LastRendererSize.x != viewport.Size.x || viewport.LastRendererSize.y != viewport.Size.y) && g.PlatformIO.Renderer_SetWindowSize)
            g.PlatformIO.Renderer_SetWindowSize(viewport, viewport.Size);
        viewport.LastPlatformPos = viewport.Pos;
        viewport.LastPlatformSize = viewport.LastRendererSize = viewport.Size;

        // Update title bar (if it changed)
        if (let mut window_for_title: *mut ImGuiWindow =  GetWindowForTitleDisplay(viewport.Window))
        {
            let mut  title_begin: *const c_char = window_for_title.Name;
            char* title_end = (char*)FindRenderedTextEnd(title_begin);
            let mut title_hash: ImGuiID =  ImHashStr(title_begin, title_end - title_begin);
            if (viewport.LastNameHash != title_hash)
            {
                 title_end_backup_c: c_char = *title_end;
                *title_end = 0; // Cut existing buffer short instead of doing an alloc/free, no small gain.
                g.PlatformIO.Platform_SetWindowTitle(viewport, title_begin);
                *title_end = title_end_backup_c;
                viewport.LastNameHash = title_hash;
            }
        }

        // Update alpha (if it changed)
        if (viewport.LastAlpha != viewport.Alpha && g.PlatformIO.Platform_SetWindowAlpha)
            g.PlatformIO.Platform_SetWindowAlpha(viewport, viewport.Alpha);
        viewport.LastAlpha = viewport.Alpha;

        // Optional, general purpose call to allow the backend to perform general book-keeping even if things haven't changed.
        if (g.PlatformIO.Platform_UpdateWindow)
            g.PlatformIO.Platform_UpdateWindow(viewport);

        if (is_new_platform_window)
        {
            // On startup ensure new platform window don't steal focus (give it a few frames, as nested contents may lead to viewport being created a few frames late)
            if (g.FrameCount < 3)
                viewport.Flags |= ImGuiViewportFlags_NoFocusOnAppearing;

            // Show window
            g.PlatformIO.Platform_ShowWindow(viewport);

            // Even without focus, we assume the window becomes front-most.
            // This is useful for our platform z-order heuristic when io.MouseHoveredViewport is not available.
            if (viewport.LastFrontMostStampCount != g.ViewportFrontMostStampCount)
                viewport.LastFrontMostStampCount = ++g.ViewportFrontMostStampCount;
            }

        // Clear request flags
        viewport.ClearRequestFlags();
    }

    // Update our implicit z-order knowledge of platform windows, which is used when the backend cannot provide io.MouseHoveredViewport.
    // When setting Platform_GetWindowFocus, it is expected that the platform backend can handle calls without crashing if it doesn't have data stored.
    // FIXME-VIEWPORT: We should use this information to also set dear imgui-side focus, allowing us to handle os-level alt+tab.
    if (g.PlatformIO.Platform_GetWindowFocus != null_mut())
    {
        let mut focused_viewport: *mut ImGuiViewport =  null_mut();
        for (let n: c_int = 0; n < g.Viewports.len() && focused_viewport == null_mut(); n++)
        {
            let mut viewport: *mut ImGuiViewport =  g.Viewports[n];
            if (viewport.PlatformWindowCreated)
                if (g.PlatformIO.Platform_GetWindowFocus(viewport))
                    focused_viewport = viewport;
        }

        // Store a tag so we can infer z-order easily from all our windows
        // We compare PlatformLastFocusedViewportId so newly created viewports with _NoFocusOnAppearing flag
        // will keep the front most stamp instead of losing it back to their parent viewport.
        if (focused_viewport && g.PlatformLastFocusedViewportId != focused_viewport.ID)
        {
            if (focused_viewport.LastFrontMostStampCount != g.ViewportFrontMostStampCount)
                focused_viewport.LastFrontMostStampCount = ++g.ViewportFrontMostStampCount;
            g.PlatformLastFocusedViewportId = focused_viewport.ID;
        }
    }
}

// This is a default/basic function for performing the rendering/swap of multiple Platform Windows.
// Custom renderers may prefer to not call this function at all, and instead iterate the publicly exposed platform data and handle rendering/sync themselves.
// The Render/Swap functions stored in ImGuiPlatformIO are merely here to allow for this helper to exist, but you can do it yourself:
//
//    ImGuiPlatformIO& platform_io = GetPlatformIO();
//    for (int i = 1; i < platform_io.Viewports.Size; i++)
//        if ((platform_io.Viewports[i]->Flags & ImGuiViewportFlags_Minimized) == 0)
//            MyRenderFunction(platform_io.Viewports[i], my_args);
//    for (int i = 1; i < platform_io.Viewports.Size; i++)
//        if ((platform_io.Viewports[i]->Flags & ImGuiViewportFlags_Minimized) == 0)
//            MySwapBufferFunction(platform_io.Viewports[i], my_args);
//
c_void RenderPlatformWindowsDefault(platform_render_arg: *mut c_void, renderer_render_arg: *mut c_void)
{
    // Skip the main viewport (index 0), which is always fully handled by the application!
    ImGuiPlatformIO& platform_io = GetPlatformIO();
    for (let i: c_int = 1; i < platform_io.Viewports.len(); i++)
    {
        ImGuiViewport* viewport = platform_io.Viewports[i];
        if (viewport.Flags & ImGuiViewportFlags_Minimized)
            continue;
        if (platform_io.Platform_RenderWindow) platform_io.Platform_RenderWindow(viewport, platform_render_arg);
        if (platform_io.Renderer_RenderWindow) platform_io.Renderer_RenderWindow(viewport, renderer_render_arg);
    }
    for (let i: c_int = 1; i < platform_io.Viewports.len(); i++)
    {
        ImGuiViewport* viewport = platform_io.Viewports[i];
        if (viewport.Flags & ImGuiViewportFlags_Minimized)
            continue;
        if (platform_io.Platform_SwapBuffers) platform_io.Platform_SwapBuffers(viewport, platform_render_arg);
        if (platform_io.Renderer_SwapBuffers) platform_io.Renderer_SwapBuffers(viewport, renderer_render_arg);
    }
}

static FindPlatformMonitorForPos: c_int(pos: &ImVec2)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    for (let monitor_n: c_int = 0; monitor_n < g.PlatformIO.Monitors.Size; monitor_n++)
    {
        const ImGuiPlatformMonitor& monitor = g.PlatformIO.Monitors[monitor_n];
        if (ImRect(monitor.MainPos, monitor.MainPos + monitor.MainSize).Contains(pos))
            return monitor_n;
    }
    return -1;
}

// Search for the monitor with the largest intersection area with the given rectangle
// We generally try to avoid searching loops but the monitor count should be very small here
// FIXME-OPT: We could test the last monitor used for that viewport first, and early
static FindPlatformMonitorForRect: c_int(rect: &ImRect)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    let monitor_count: c_int = g.PlatformIO.Monitors.Size;
    if (monitor_count <= 1)
        return monitor_count - 1;

    // Use a minimum threshold of 1.0 so a zero-sized rect won't false positive, and will still find the correct monitor given its position.
    // This is necessary for tooltips which always resize down to zero at first.
    let surface_threshold: c_float =  ImMax(rect.GetWidth() * rect.GetHeight() * 0.5f32, 1.0);
    let best_monitor_n: c_int = -1;
    let best_monitor_surface: c_float =  0.001f;

    for (let monitor_n: c_int = 0; monitor_n < g.PlatformIO.Monitors.Size && best_monitor_surface < surface_threshold; monitor_n++)
    {
        const ImGuiPlatformMonitor& monitor = g.PlatformIO.Monitors[monitor_n];
        let monitor_rect: ImRect =  ImRect(monitor.MainPos, monitor.MainPos + monitor.MainSize);
        if (monitor_rect.Contains(rect))
            return monitor_n;
        let overlapping_rect: ImRect =  rect;
        overlapping_rect.ClipWithFull(monitor_rect);
        let overlapping_surface: c_float =  overlapping_rect.GetWidth() * overlapping_rect.GetHeight();
        if (overlapping_surface < best_monitor_surface)
            continue;
        best_monitor_surface = overlapping_surface;
        best_monitor_n = monitor_n;
    }
    return best_monitor_n;
}

// Update monitor from viewport rectangle (we'll use this info to clamp windows and save windows lost in a removed monitor)
static c_void UpdateViewportPlatformMonitor(*mut ImGuiViewportP viewport)
{
    viewport.PlatformMonitor = FindPlatformMonitorForRect(viewport.GetMainRect());
}

// Return value is always != NULL, but don't hold on it across frames.
*const ImGuiPlatformMonitor GetViewportPlatformMonitor(ImGuiViewport* viewport_p)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut viewport: *mut ImGuiViewport =  (*mut c_void)viewport_p;
    let monitor_idx: c_int = viewport.PlatformMonitor;
    if (monitor_idx >= 0 && monitor_idx < g.PlatformIO.Monitors.Size)
        return &g.PlatformIO.Monitors[monitor_idx];
    return &g.FallbackMonitor;
}

c_void DestroyPlatformWindow(*mut ImGuiViewportP viewport)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (viewport.PlatformWindowCreated)
    {
        if (g.PlatformIO.Renderer_DestroyWindow)
            g.PlatformIO.Renderer_DestroyWindow(viewport);
        if (g.PlatformIO.Platform_DestroyWindow)
            g.PlatformIO.Platform_DestroyWindow(viewport);
        // IM_ASSERT(viewport.RendererUserData == NULL && viewport.PlatformUserData == NULL);

        // Don't clear PlatformWindowCreated for the main viewport, as we initially set that up to true in Initialize()
        // The righter way may be to leave it to the backend to set this flag all-together, and made the flag public.
        if (viewport.ID != IMGUI_VIEWPORT_DEFAULT_ID)
            viewport.PlatformWindowCreated = false;
    }
    else
    {
        // IM_ASSERT(viewport.RendererUserData == NULL && viewport.PlatformUserData == NULL && viewport.PlatformHandle == NULL);
    }
    viewport.RendererUserData = viewport.PlatformUserData = viewport.PlatformHandle= null_mut();
    viewport.ClearRequestFlags();
}

c_void DestroyPlatformWindows()
{
    // We call the destroy window on every viewport (including the main viewport, index 0) to give a chance to the backend
    // to clear any data they may have stored in e.g. PlatformUserData, RendererUserData.
    // It is convenient for the platform backend code to store something in the main viewport, in order for e.g. the mouse handling
    // code to operator a consistent manner.
    // It is expected that the backend can handle calls to Renderer_DestroyWindow/Platform_DestroyWindow without
    // crashing if it doesn't have data stored.
    let g = GImGui; // ImGuiContext& g = *GImGui;
    for (let i: c_int = 0; i < g.Viewports.len(); i++)
        DestroyPlatformWindow(g.Viewports[i]);
}


//-----------------------------------------------------------------------------
// [SECTION] DOCKING
//-----------------------------------------------------------------------------
// Docking: Internal Types
// Docking: Forward Declarations
// Docking: ImGuiDockContext
// Docking: ImGuiDockContext Docking/Undocking functions
// Docking: ImGuiDockNode
// Docking: ImGuiDockNode Tree manipulation functions
// Docking: Public Functions (SetWindowDock, DockSpace, DockSpaceOverViewport)
// Docking: Builder Functions
// Docking: Begin/End Support Functions (called from Begin/End)
// Docking: Settings
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// Typical Docking call flow: (root level is generally public API):
//-----------------------------------------------------------------------------
// - NewFrame()                               new dear imgui frame
//    | DockContextNewFrameUpdateUndocking()  - process queued undocking requests
//    | - DockContextProcessUndockWindow()    - process one window undocking request
//    | - DockContextProcessUndockNode()      - process one whole node undocking request
//    | DockContextNewFrameUpdateUndocking()  - process queue docking requests, create floating dock nodes
//    | - update g.HoveredDockNode            - [debug] update node hovered by mouse
//    | - DockContextProcessDock()            - process one docking request
//    | - DockNodeUpdate()
//    |   - DockNodeUpdateForRootNode()
//    |     - DockNodeUpdateFlagsAndCollapse()
//    |     - DockNodeFindInfo()
//    |   - destroy unused node or tab bar
//    |   - create dock node host window
//    |      - Begin() etc.
//    |   - DockNodeStartMouseMovingWindow()
//    |   - DockNodeTreeUpdatePosSize()
//    |   - DockNodeTreeUpdateSplitter()
//    |   - draw node background
//    |   - DockNodeUpdateTabBar()            - create/update tab bar for a docking node
//    |     - DockNodeAddTabBar()
//    |     - DockNodeUpdateWindowMenu()
//    |     - DockNodeCalcTabBarLayout()
//    |     - BeginTabBarEx()
//    |     - TabItemEx() calls
//    |     - EndTabBar()
//    |   - BeginDockableDragDropTarget()
//    |      - DockNodeUpdate()               - recurse into child nodes...
//-----------------------------------------------------------------------------
// - DockSpace()                              user submit a dockspace into a window
//    | Begin(Child)                          - create a child window
//    | DockNodeUpdate()                      - call main dock node update function
//    | End(Child)
//    | ItemSize()
//-----------------------------------------------------------------------------
// - Begin()
//    | BeginDocked()
//    | BeginDockableDragDropSource()
//    | BeginDockableDragDropTarget()
//    | - DockNodePreviewDockRender()
//-----------------------------------------------------------------------------
// - EndFrame()
//    | DockContextEndFrame()
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// Docking: Internal Types
//-----------------------------------------------------------------------------
// - ImGuiDockRequestType
// - ImGuiDockRequest
// - ImGuiDockPreviewData
// - ImGuiDockNodeSettings
// - ImGuiDockContext
//-----------------------------------------------------------------------------





struct ImGuiDockPreviewData
{
    ImGuiDockNode   FutureNode;
    bool            IsDropAllowed;
    bool            IsCenterAvailable;
    bool            IsSidesAvailable;           // Hold your breath, grammar freaks..
    bool            IsSplitDirExplicit;         // Set when hovered the drop rect (vs. implicit SplitDir==None when hovered the window)
    ImGuiDockNode*  SplitNode;
    ImGuiDir        SplitDir;SplitRatio: c_float;
    ImRect          DropRectsDraw[ImGuiDir_COUNT + 1];  // May be slightly different from hit-testing drop rects used in DockNodeCalcDropRects()

    ImGuiDockPreviewData() : FutureNode(0) { IsDropAllowed = IsCenterAvailable = IsSidesAvailable = IsSplitDirExplicit = false; SplitNode= null_mut(); SplitDir = ImGuiDir_None; SplitRatio = 0.f; for (let n: c_int = 0; n < DropRectsDraw.len(); n++) DropRectsDraw[n] = ImRect(f32::MAX, f32::MAX, -f32::MAX, -f32::MAX); }
};



//-----------------------------------------------------------------------------
// Docking: Forward Declarations
//-----------------------------------------------------------------------------

namespace ImGui
{
    // ImGuiDockContext
    static ImGuiDockNode*   DockContextAddNode(ImGuiContext* ctx, id: ImGuiID);
    static c_void             DockContextRemoveNode(ImGuiContext* ctx, ImGuiDockNode* node, merge_sibling_into_parent_node: bool);
    static c_void             DockContextQueueNotifyRemovedNode(ImGuiContext* ctx, ImGuiDockNode* node);
    static c_void             DockContextProcessDock(ImGuiContext* ctx, ImGuiDockRequest* req);
    static c_void             DockContextProcessUndockWindow(ImGuiContext* ctx, window: *mut ImGuiWindow, let mut clear_persistent_docking_ref: bool =  true);
    static c_void             DockContextProcessUndockNode(ImGuiContext* ctx, ImGuiDockNode* node);
    static c_void             DockContextPruneUnusedSettingsNodes(ImGuiContext* ctx);
    static ImGuiDockNode*   DockContextBindNodeToWindow(ImGuiContext* ctx, window: *mut ImGuiWindow);
    static c_void             DockContextBuildNodesFromSettings(ImGuiContext* ctx, ImGuiDockNodeSettings* node_settings_array, node_settings_count: c_int);
    static c_void             DockContextBuildAddWindowsToNodes(ImGuiContext* ctx, ImGuiID root_id);                            // Use root_id==0 to add all

    // ImGuiDockNode
    static c_void             DockNodeAddWindow(ImGuiDockNode* node, window: *mut ImGuiWindow, add_to_tab_bar: bool);
    static c_void             DockNodeMoveWindows(ImGuiDockNode* dst_node, ImGuiDockNode* src_node);
    static c_void             DockNodeMoveChildNodes(ImGuiDockNode* dst_node, ImGuiDockNode* src_node);
    static ImGuiWindow*     DockNodeFindWindowByID(ImGuiDockNode* node, id: ImGuiID);
    static c_void             DockNodeApplyPosSizeToWindows(ImGuiDockNode* node);
    static c_void             DockNodeRemoveWindow(ImGuiDockNode* node, window: *mut ImGuiWindow, ImGuiID save_dock_id);
    static c_void             DockNodeHideHostWindow(ImGuiDockNode* node);
    static c_void             DockNodeUpdate(ImGuiDockNode* node);
    static c_void             DockNodeUpdateForRootNode(ImGuiDockNode* node);
    static c_void             DockNodeUpdateFlagsAndCollapse(ImGuiDockNode* node);
    static c_void             DockNodeUpdateHasCentralNodeChild(ImGuiDockNode* node);
    static c_void             DockNodeUpdateTabBar(ImGuiDockNode* node, host_window: *mut ImGuiWindow);
    static c_void             DockNodeAddTabBar(ImGuiDockNode* node);
    static c_void             DockNodeRemoveTabBar(ImGuiDockNode* node);
    static ImGuiID          DockNodeUpdateWindowMenu(ImGuiDockNode* node, ImGuiTabBar* tab_bar);
    static c_void             DockNodeUpdateVisibleFlag(ImGuiDockNode* node);
    static c_void             DockNodeStartMouseMovingWindow(ImGuiDockNode* node, window: *mut ImGuiWindow);
    static bool             DockNodeIsDropAllowed(host_window: *mut ImGuiWindow, payload_window: *mut ImGuiWindow);
    static c_void             DockNodePreviewDockSetup(host_window: *mut ImGuiWindow, ImGuiDockNode* host_node, payload_window: *mut ImGuiWindow, ImGuiDockNode* payload_node, ImGuiDockPreviewData* preview_data, is_explicit_target: bool, is_outer_docking: bool);
    static c_void             DockNodePreviewDockRender(host_window: *mut ImGuiWindow, ImGuiDockNode* host_node, payload_window: *mut ImGuiWindow, *const ImGuiDockPreviewData preview_data);
    static c_void             DockNodeCalcTabBarLayout(*const ImGuiDockNode node, ImRect* out_title_rect, ImRect* out_tab_bar_rect, out_window_menu_button_pos: *mut ImVec2, out_close_button_pos: *mut ImVec2);
    static c_void             DockNodeCalcSplitRects(ImVec2& pos_old, ImVec2& size_old, ImVec2& pos_new, ImVec2& size_new, dir: ImGuiDir, size_new_desired: ImVec2);
    static bool             DockNodeCalcDropRectsAndTestMousePos(parent: &ImRect, dir: ImGuiDir, ImRect& out_draw, outer_docking: bool, test_mouse_pos: *mut ImVec2);
    static *const char      DockNodeGetHostWindowTitle(ImGuiDockNode* node, char* buf, buf_size: c_int) { ImFormatString(buf, buf_size, "##DockNode_%02X", node.ID); return buf; }
    static c_int              DockNodeGetTabOrder(window: *mut ImGuiWindow);

    // ImGuiDockNode tree manipulations
    static c_void             DockNodeTreeSplit(ImGuiContext* ctx, ImGuiDockNode* parent_node, ImGuiAxis split_axis, split_first_child: c_int,split_ratio: c_float, ImGuiDockNode* new_node);
    static c_void             DockNodeTreeMerge(ImGuiContext* ctx, ImGuiDockNode* parent_node, ImGuiDockNode* merge_lead_child);
    static c_void             DockNodeTreeUpdatePosSize(ImGuiDockNode* node, pos: ImVec2, size: ImVec2, ImGuiDockNode* only_write_to_single_node = null_mut());
    static c_void             DockNodeTreeUpdateSplitter(ImGuiDockNode* node);
    static ImGuiDockNode*   DockNodeTreeFindVisibleNodeByPos(ImGuiDockNode* node, pos: ImVec2);
    static ImGuiDockNode*   DockNodeTreeFindFallbackLeafNode(ImGuiDockNode* node);

    // Settings
    static c_void             DockSettingsRenameNodeReferences(ImGuiID old_node_id, ImGuiID new_node_id);
    static c_void             DockSettingsRemoveNodeReferences(ImGuiID* node_ids, node_ids_count: c_int);
    static ImGuiDockNodeSettings*   DockSettingsFindNodeSettings(ImGuiContext* ctx, ImGuiID node_id);
    static c_void             DockSettingsHandler_ClearAll(ImGuiContext*, ImGuiSettingsHandler*);
    static c_void             DockSettingsHandler_ApplyAll(ImGuiContext*, ImGuiSettingsHandler*);
    static *mut c_void            DockSettingsHandler_ReadOpen(ImGuiContext*, ImGuiSettingsHandler*, name: *const c_char);
    static c_void             DockSettingsHandler_ReadLine(ImGuiContext*, ImGuiSettingsHandler*, entry: *mut c_void, line: *const c_char);
    static c_void             DockSettingsHandler_WriteAll(ImGuiContext* imgui_ctx, ImGuiSettingsHandler* handler, ImGuiTextBuffer* bu0f32);
}

//-----------------------------------------------------------------------------
// Docking: ImGuiDockContext
//-----------------------------------------------------------------------------
// The lifetime model is different from the one of regular windows: we always create a ImGuiDockNode for each ImGuiDockNodeSettings,
// or we always hold the entire docking node tree. Nodes are frequently hidden, e.g. if the window(s) or child nodes they host are not active.
// At boot time only, we run a simple GC to remove nodes that have no references.
// Because dock node settings (which are small, contiguous structures) are always mirrored by their corresponding dock nodes (more complete structures),
// we can also very easily recreate the nodes from scratch given the settings data (this is what DockContextRebuild() does).
// This is convenient as docking reconfiguration can be implemented by mostly poking at the simpler settings data.
//-----------------------------------------------------------------------------
// - DockContextInitialize()
// - DockContextShutdown()
// - DockContextClearNodes()
// - DockContextRebuildNodes()
// - DockContextNewFrameUpdateUndocking()
// - DockContextNewFrameUpdateDocking()
// - DockContextEndFrame()
// - DockContextFindNodeByID()
// - DockContextBindNodeToWindow()
// - DockContextGenNodeID()
// - DockContextAddNode()
// - DockContextRemoveNode()
// - ImGuiDockContextPruneNodeData
// - DockContextPruneUnusedSettingsNodes()
// - DockContextBuildNodesFromSettings()
// - DockContextBuildAddWindowsToNodes()
//-----------------------------------------------------------------------------

c_void DockContextInitialize(ImGuiContext* ctx)
{
    ImGuiContext& g = *ctx;

    // Add .ini handle for persistent docking data
    ImGuiSettingsHandler ini_handler;
    ini_handler.TypeName = "Docking";
    ini_handler.TypeHash = ImHashStr("Docking");
    ini_handler.ClearAllFn = DockSettingsHandler_ClearAll;
    ini_handler.ReadInitFn = DockSettingsHandler_ClearAll; // Also clear on read
    ini_handler.ReadOpenFn = DockSettingsHandler_ReadOpen;
    ini_handler.ReadLineFn = DockSettingsHandler_ReadLine;
    ini_handler.ApplyAllFn = DockSettingsHandler_ApplyAll;
    ini_handler.WriteAllFn = DockSettingsHandler_WriteAll;
    g.SettingsHandlers.push(ini_handler);
}

c_void DockContextShutdown(ImGuiContext* ctx)
{
    ImGuiDockContext* dc  = &ctx->DockContext;
    for (let n: c_int = 0; n < dc->Nodes.Data.Size; n++)
        if (ImGuiDockNode* node = (ImGuiDockNode*)dc->Nodes.Data[n].val_p)
            IM_DELETE(node);
}

c_void DockContextClearNodes(ImGuiContext* ctx, ImGuiID root_id, clear_settings_refs: bool)
{
    IM_UNUSED(ctx);
    // IM_ASSERT(ctx == GImGui);
    DockBuilderRemoveNodeDockedWindows(root_id, clear_settings_refs);
    DockBuilderRemoveNodeChildNodes(root_id);
}

// [DEBUG] This function also acts as a defacto test to make sure we can rebuild from scratch without a glitch
// (Different from DockSettingsHandler_ClearAll() + DockSettingsHandler_ApplyAll() because this reuses current settings!)
c_void DockContextRebuildNodes(ImGuiContext* ctx)
{
    ImGuiContext& g = *ctx;
    ImGuiDockContext* dc = &ctx->DockContext;
    IMGUI_DEBUG_LOG_DOCKING("[docking] DockContextRebuildNodes\n");
    SaveIniSettingsToMemory();
    let mut root_id: ImGuiID =  0; // Rebuild all
    DockContextClearNodes(ctx, root_id, false);
    DockContextBuildNodesFromSettings(ctx, dc->NodesSettings.Data, dc->NodesSettings.Size);
    DockContextBuildAddWindowsToNodes(ctx, root_id);
}

// Docking context update function, called by NewFrame()
c_void DockContextNewFrameUpdateUndocking(ImGuiContext* ctx)
{
    ImGuiContext& g = *ctx;
    ImGuiDockContext* dc = &ctx->DockContext;
    if (!(g.IO.ConfigFlags & ImGuiConfigFlags_DockingEnable))
    {
        if (dc->Nodes.Data.Size > 0 || dc->Requests.Size > 0)
            DockContextClearNodes(ctx, 0, true);
        return;
    }

    // Setting NoSplit at runtime merges all nodes
    if (g.IO.ConfigDockingNoSplit)
        for (let n: c_int = 0; n < dc->Nodes.Data.Size; n++)
            if (ImGuiDockNode* node = (ImGuiDockNode*)dc->Nodes.Data[n].val_p)
                if (node.IsRootNode() && node.IsSplitNode())
                {
                    DockBuilderRemoveNodeChildNodes(node.ID);
                    //dc->WantFullRebuild = true;
                }

    // Process full rebuild
// #if 0
    if (IsKeyPressed(GetKeyIndex(ImGuiKey_C)))
        dc->WantFullRebuild = true;
// #endif
    if (dc->WantFullRebuild)
    {
        DockContextRebuildNodes(ctx);
        dc->WantFullRebuild = false;
    }

    // Process Undocking requests (we need to process them _before_ the UpdateMouseMovingWindowNewFrame call in NewFrame)
    for (let n: c_int = 0; n < dc->Requests.Size; n++)
    {
        ImGuiDockRequest* req = &dc->Requests[n];
        if (req.Type == ImGuiDockRequestType_Undock && req->UndockTargetWindow)
            DockContextProcessUndockWindow(ctx, req->UndockTargetWindow);
        else if (req.Type == ImGuiDockRequestType_Undock && req->UndockTargetNode)
            DockContextProcessUndockNode(ctx, req->UndockTargetNode);
    }
}

// Docking context update function, called by NewFrame()
c_void DockContextNewFrameUpdateDocking(ImGuiContext* ctx)
{
    ImGuiContext& g = *ctx;
    ImGuiDockContext* dc  = &ctx->DockContext;
    if (!(g.IO.ConfigFlags & ImGuiConfigFlags_DockingEnable))
        return;

    // [DEBUG] Store hovered dock node.
    // We could in theory use DockNodeTreeFindVisibleNodeByPos() on the root host dock node, but using ->DockNode is a good shortcut.
    // Note this is mostly a debug thing and isn't actually used for docking target, because docking involve more detailed filtering.
    g.DebugHoveredDockNode= null_mut();
    if (let mut hovered_window: *mut ImGuiWindow =  g.HoveredWindowUnderMovingWindow)
    {
        if (hovered_window.DockNodeAsHost)
            g.DebugHoveredDockNode = DockNodeTreeFindVisibleNodeByPos(hovered_window.DockNodeAsHost, g.IO.MousePos);
        else if (hovered_window.Rootwindow.DockNode)
            g.DebugHoveredDockNode = hovered_window.Rootwindow.DockNode;
    }

    // Process Docking requests
    for (let n: c_int = 0; n < dc->Requests.Size; n++)
        if (dc->Requests[n].Type == ImGuiDockRequestType_Dock)
            DockContextProcessDock(ctx, &dc->Requests[n]);
    dc->Requests.clear();

    // Create windows for each automatic docking nodes
    // We can have NULL pointers when we delete nodes, but because ID are recycled this should amortize nicely (and our node count will never be very high)
    for (let n: c_int = 0; n < dc->Nodes.Data.Size; n++)
        if (ImGuiDockNode* node = (ImGuiDockNode*)dc->Nodes.Data[n].val_p)
            if (node.IsFloatingNode())
                DockNodeUpdate(node);
}

c_void DockContextEndFrame(ImGuiContext* ctx)
{
    // Draw backgrounds of node missing their window
    ImGuiContext& g = *ctx;
    ImGuiDockContext* dc = &g.DockContext;
    for (let n: c_int = 0; n < dc->Nodes.Data.Size; n++)
        if (ImGuiDockNode* node = (ImGuiDockNode*)dc->Nodes.Data[n].val_p)
            if (node.LastFrameActive == g.FrameCount && node.IsVisible && node.HostWindow && node.IsLeafNode() && !node.IsBgDrawnThisFrame)
            {
                let mut bg_rect: ImRect = ImRect::new(node.Pos + ImVec2::new(0.0, GetFrameHeight()), node.Pos + node.Size);
                bg_rounding_flags: ImDrawFlags = CalcRoundingFlagsForRectInRect(bg_rect, node.Hostwindow.Rect(), DOCKING_SPLITTER_SIZE);
                node.Hostwindow.DrawList.ChannelsSetCurrent(DOCKING_HOST_DRAW_CHANNEL_BG);
                node.Hostwindow.DrawList.AddRectFilled(bg_rect.Min, bg_rect.Max, node.LastBgColor, node.Hostwindow.WindowRounding, bg_rounding_flags);
            }
}

ImGuiDockNode* DockContextFindNodeByID(ImGuiContext* ctx, id: ImGuiID)
{
    return (ImGuiDockNode*)ctx->DockContext.Nodes.GetVoidPtr(id);
}

ImGuiID DockContextGenNodeID(ImGuiContext* ctx)
{
    // Generate an ID for new node (the exact ID value doesn't matter as long as it is not already used)
    // FIXME-OPT FIXME-DOCK: This is suboptimal, even if the node count is small enough not to be a worry.0
    // We should poke in ctx->Nodes to find a suitable ID faster. Even more so trivial that ctx->Nodes lookup is already sorted.
    let mut id: ImGuiID =  0x0001;
    while (DockContextFindNodeByID(ctx, id) != null_mut())
        id+= 1;
    return id;
}

static ImGuiDockNode* DockContextAddNode(ImGuiContext* ctx, id: ImGuiID)
{
    // Generate an ID for the new node (the exact ID value doesn't matter as long as it is not already used) and add the first window.
    ImGuiContext& g = *ctx;
    if (id == 0)
        id = DockContextGenNodeID(ctx);
    else
        // IM_ASSERT(DockContextFindNodeByID(ctx, id) == NULL);

    // We don't set node->LastFrameAlive on construction. Nodes are always created at all time to reflect .ini settings!
    IMGUI_DEBUG_LOG_DOCKING("[docking] DockContextAddNode 0x%08X\n", id);
    ImGuiDockNode* node = IM_NEW(ImGuiDockNode)(id);
    ctx->DockContext.Nodes.SetVoidPtr(node.ID, node);
    return node;
}

static c_void DockContextRemoveNode(ImGuiContext* ctx, ImGuiDockNode* node, merge_sibling_into_parent_node: bool)
{
    ImGuiContext& g = *ctx;
    ImGuiDockContext* dc  = &ctx->DockContext;

    IMGUI_DEBUG_LOG_DOCKING("[docking] DockContextRemoveNode 0x%08X\n", node.ID);
    // IM_ASSERT(DockContextFindNodeByID(ctx, node->ID) == node);
    // IM_ASSERT(node->ChildNodes[0] == NULL && node->ChildNodes[1] == NULL);
    // IM_ASSERT(node->Windows.Size == 0);

    if (node.HostWindow)
        node.Hostwindow.DockNodeAsHost= null_mut();

    ImGuiDockNode* parent_node = node.ParentNode;
    let merge: bool = (merge_sibling_into_parent_node && parent_node != null_mut());
    if (merge)
    {
        // IM_ASSERT(parent_node->ChildNodes[0] == node || parent_node->ChildNodes[1] == node);
        ImGuiDockNode* sibling_node = (parent_node.ChildNodes[0] == node ? parent_node.ChildNodes[1] : parent_node.ChildNodes[0]);
        DockNodeTreeMerge(&g, parent_node, sibling_node);
    }
    else
    {
        for (let n: c_int = 0; parent_node && n < IM_ARRAYSIZE(parent_node.ChildNodes); n++)
            if (parent_node.ChildNodes[n] == node)
                node.ParentNode.ChildNodes[n]= null_mut();
        dc->Nodes.SetVoidPtr(node.ID, null_mut());
        IM_DELETE(node);
    }
}

static IMGUI_CDECL: c_int DockNodeComparerDepthMostFirst(*const c_void lhs, *const c_void rhs)
{
    let a: *const ImGuiDockNode = *(*const ImGuiDockNode const*)lhs;
    let b: *const ImGuiDockNode = *(*const ImGuiDockNode const*)rhs;
    return DockNodeGetDepth(b) - DockNodeGetDepth(a);
}

// Pre C++0x doesn't allow us to use a function-local type (without linkage) as template parameter, so we moved this here.
struct ImGuiDockContextPruneNodeData
{
    c_int         CountWindows, CountChildWindows, CountChildNodes;
    ImGuiID     RootId;
    ImGuiDockContextPruneNodeData() { CountWindows = CountChildWindows = CountChildNodes = 0; RootId = 0; }
};

// Garbage collect unused nodes (run once at init time)
static c_void DockContextPruneUnusedSettingsNodes(ImGuiContext* ctx)
{
    ImGuiContext& g = *ctx;
    ImGuiDockContext* dc  = &ctx->DockContext;
    // IM_ASSERT(g.Windows.Size == 0);

    ImPool<ImGuiDockContextPruneNodeData> pool;
    pool.Reserve(dc->NodesSettings.Size);

    // Count child nodes and compute RootID
    for (let settings_n: c_int = 0; settings_n < dc->NodesSettings.Size; settings_n++)
    {
        ImGuiDockNodeSettings* settings = &dc->NodesSettings[settings_n];
        ImGuiDockContextPruneNodeData* parent_data = settings->ParentNodeId ? pool.GetByKey(settings->ParentNodeId) : 0;
        pool.GetOrAddByKey(settings->ID)->RootId = parent_data ? parent_Data.RootId : settings->ID;
        if (settings->ParentNodeId)
            pool.GetOrAddByKey(settings->ParentNodeId)->CountChildNodes+= 1;
    }

    // Count reference to dock ids from dockspaces
    // We track the 'auto-DockNode <- manual-Window <- manual-DockSpace' in order to avoid 'auto-DockNode' being ditched by DockContextPruneUnusedSettingsNodes()
    for (let settings_n: c_int = 0; settings_n < dc->NodesSettings.Size; settings_n++)
    {
        ImGuiDockNodeSettings* settings = &dc->NodesSettings[settings_n];
        if (settings->ParentWindowId != 0)
            if (window_settings: *mut ImGuiWindowSettings = FindWindowSettings(settings->ParentWindowId))
                if (window_settings->DockId)
                    if (ImGuiDockContextPruneNodeData* data = pool.GetByKey(window_settings->DockId))
                        data.CountChildNodes+= 1;
    }

    // Count reference to dock ids from window settings
    // We guard against the possibility of an invalid .ini file (RootID may point to a missing node)
    for (settings: *mut ImGuiWindowSettings = g.SettingsWindows.begin(); settings != null_mut(); settings = g.SettingsWindows.next_chunk(settings))
        if (let mut dock_id: ImGuiID =  settings->DockId)
            if (ImGuiDockContextPruneNodeData* data = pool.GetByKey(dock_id))
            {
                data.CountWindows+= 1;
                if (ImGuiDockContextPruneNodeData* data_root = (data.RootId == dock_id) ? data : pool.GetByKey(data.RootId))
                    data_root->CountChildWindows+= 1;
            }

    // Prune
    for (let settings_n: c_int = 0; settings_n < dc->NodesSettings.Size; settings_n++)
    {
        ImGuiDockNodeSettings* settings = &dc->NodesSettings[settings_n];
        ImGuiDockContextPruneNodeData* data = pool.GetByKey(settings->ID);
        if (data.CountWindows > 1)
            continue;
        ImGuiDockContextPruneNodeData* data_root = (data.RootId == settings->ID) ? data : pool.GetByKey(data.RootId);

        let mut remove: bool =  false;
        remove |= (data.CountWindows == 1 && settings->ParentNodeId == 0 && data.CountChildNodes == 0 && !(settings->Flags & ImGuiDockNodeFlags_CentralNode));  // Floating root node with only 1 window
        remove |= (data.CountWindows == 0 && settings->ParentNodeId == 0 && data.CountChildNodes == 0); // Leaf nodes with 0 window
        remove |= (data_root->CountChildWindows == 0);
        if (remove)
        {
            IMGUI_DEBUG_LOG_DOCKING("[docking] DockContextPruneUnusedSettingsNodes: Prune 0x%08X\n", settings->ID);
            DockSettingsRemoveNodeReferences(&settings->ID, 1);
            settings->ID = 0;
        }
    }
}

static c_void DockContextBuildNodesFromSettings(ImGuiContext* ctx, ImGuiDockNodeSettings* node_settings_array, node_settings_count: c_int)
{
    // Build nodes
    for (let node_n: c_int = 0; node_n < node_settings_count; node_n++)
    {
        ImGuiDockNodeSettings* settings = &node_settings_array[node_n];
        if (settings->ID == 0)
            continue;
        ImGuiDockNode* node = DockContextAddNode(ctx, settings->ID);
        node.ParentNode = settings->ParentNodeId ? DockContextFindNodeByID(ctx, settings->ParentNodeId) : null_mut();
        node.Pos = ImVec2::new(settings->Pos.x, settings->Pos.y);
        node.Size = ImVec2::new(settings->Size.x, settings->Size.y);
        node.SizeRef = ImVec2::new(settings->SizeRef.x, settings->SizeRef.y);
        node.AuthorityForPos = node.AuthorityForSize = node.AuthorityForViewport = ImGuiDataAuthority_DockNode;
        if (node.ParentNode && node.ParentNode.ChildNodes[0] == null_mut())
            node.ParentNode.ChildNodes[0] = node;
        else if (node.ParentNode && node.ParentNode.ChildNodes[1] == null_mut())
            node.ParentNode.ChildNodes[1] = node;
        node.SelectedTabId = settings->SelectedTabId;
        node.SplitAxis = (ImGuiAxis)settings->SplitAxis;
        node.SetLocalFlags(settings->Flags & ImGuiDockNodeFlags_SavedFlagsMask_);

        // Bind host window immediately if it already exist (in case of a rebuild)
        // This is useful as the RootWindowForTitleBarHighlight links necessary to highlight the currently focused node requires node->HostWindow to be set.
        host_window_title: [c_char;20];
        ImGuiDockNode* root_node = DockNodeGetRootNode(node);
        node.HostWindow = FindWindowByName(DockNodeGetHostWindowTitle(root_node, host_window_title, host_window_title.len()));
    }
}

c_void DockContextBuildAddWindowsToNodes(ImGuiContext* ctx, ImGuiID root_id)
{
    // Rebind all windows to nodes (they can also lazily rebind but we'll have a visible glitch during the first frame)
    ImGuiContext& g = *ctx;
    for (let n: c_int = 0; n < g.Windows.len(); n++)
    {
        let mut window: *mut ImGuiWindow =  g.Windows[n];
        if (window.DockId == 0 || window.LastFrameActive < g.FrameCount - 1)
            continue;
        if (window.DockNode != null_mut())
            continue;

        ImGuiDockNode* node = DockContextFindNodeByID(ctx, window.DockId);
        // IM_ASSERT(node != NULL);   // This should have been called after DockContextBuildNodesFromSettings()
        if (root_id == 0 || DockNodeGetRootNode(node)->ID == root_id)
            DockNodeAddWindow(node, window, true);
    }
}

//-----------------------------------------------------------------------------
// Docking: ImGuiDockContext Docking/Undocking functions
//-----------------------------------------------------------------------------
// - DockContextQueueDock()
// - DockContextQueueUndockWindow()
// - DockContextQueueUndockNode()
// - DockContextQueueNotifyRemovedNode()
// - DockContextProcessDock()
// - DockContextProcessUndockWindow()
// - DockContextProcessUndockNode()
// - DockContextCalcDropPosForDocking()
//-----------------------------------------------------------------------------

c_void DockContextQueueDock(ImGuiContext* ctx, target: *mut ImGuiWindow, ImGuiDockNode* target_node, payload: *mut ImGuiWindow, split_dir: ImGuiDir,split_ratio: c_float, split_outer: bool)
{
    // IM_ASSERT(target != payload);
    ImGuiDockRequest req;
    req.Type = ImGuiDockRequestType_Dock;
    req.DockTargetWindow = target;
    req.DockTargetNode = target_node;
    req.DockPayload = payload;
    req.DockSplitDir = split_dir;
    req.DockSplitRatio = split_ratio;
    req.DockSplitOuter = split_outer;
    ctx->DockContext.Requests.push(req);
}

c_void DockContextQueueUndockWindow(ImGuiContext* ctx, window: *mut ImGuiWindow)
{
    ImGuiDockRequest req;
    req.Type = ImGuiDockRequestType_Undock;
    req.UndockTargetWindow = window;
    ctx->DockContext.Requests.push(req);
}

c_void DockContextQueueUndockNode(ImGuiContext* ctx, ImGuiDockNode* node)
{
    ImGuiDockRequest req;
    req.Type = ImGuiDockRequestType_Undock;
    req.UndockTargetNode = node;
    ctx->DockContext.Requests.push(req);
}

c_void DockContextQueueNotifyRemovedNode(ImGuiContext* ctx, ImGuiDockNode* node)
{
    ImGuiDockContext* dc  = &ctx->DockContext;
    for (let n: c_int = 0; n < dc->Requests.Size; n++)
        if (dc->Requests[n].DockTargetNode == node)
            dc->Requests[n].Type = ImGuiDockRequestType_None;
}

c_void DockContextProcessDock(ImGuiContext* ctx, ImGuiDockRequest* req)
{
    // IM_ASSERT((req->Type == ImGuiDockRequestType_Dock && req->DockPayload != NULL) || (req->Type == ImGuiDockRequestType_Split && req->DockPayload == NULL));
    // IM_ASSERT(req->DockTargetWindow != NULL || req->DockTargetNode != NULL);

    ImGuiContext& g = *ctx;
    IM_UNUSED(g);

    let mut payload_window: *mut ImGuiWindow =  req->DockPayload;     // Optional
    let mut target_window: *mut ImGuiWindow =  req->DockTargetWindow;
    ImGuiDockNode* node = req->DockTargetNode;
    if (payload_window)
        IMGUI_DEBUG_LOG_DOCKING("[docking] DockContextProcessDock node 0x%08X target '%s' dock window '%s', split_dir %d\n", node ? node.ID : 0, target_window ? target_window.Name : "NULL", payload_window.Name, req->DockSplitDir);
    else
        IMGUI_DEBUG_LOG_DOCKING("[docking] DockContextProcessDock node 0x%08X, split_dir %d\n", node ? node.ID : 0, req->DockSplitDir);

    // Decide which Tab will be selected at the end of the operation
    let mut next_selected_id: ImGuiID =  0;
    ImGuiDockNode* payload_node= null_mut();
    if (payload_window)
    {
        payload_node = payload_window.DockNodeAsHost;
        payload_window.DockNodeAsHost= null_mut(); // Important to clear this as the node will have its life as a child which might be merged/deleted later.
        if (payload_node && payload_node.IsLeafNode())
            next_selected_id = payload_node.TabBar->NextSelectedTabId ? payload_node.TabBar->NextSelectedTabId : payload_node.TabBar->SelectedTabId;
        if (payload_node == null_mut())
            next_selected_id = payload_window.TabId;
    }

    // FIXME-DOCK: When we are trying to dock an existing single-window node into a loose window, transfer Node ID as well
    // When processing an interactive split, usually LastFrameAlive will be < g.FrameCount. But DockBuilder operations can make it ==.
    if (node)
        // IM_ASSERT(node->LastFrameAlive <= g.FrameCount);
    if (node && target_window && node == target_window.DockNodeAsHost)
        // IM_ASSERT(node->Windows.Size > 0 || node->IsSplitNode() || node->IsCentralNode());

    // Create new node and add existing window to it
    if (node == null_mut())
    {
        node = DockContextAddNode(ctx, 0);
        node.Pos = target_window.Pos;
        node.Size = target_window.Size;
        if (target_window.DockNodeAsHost == null_mut())
        {
            DockNodeAddWindow(node, target_window, true);
            node.TabBar->Tabs[0].Flags &= !ImGuiTabItemFlags_Unsorted;
            target_window.DockIsActive = true;
        }
    }

    split_dir: ImGuiDir = req->DockSplitDir;
    if (split_dir != ImGuiDir_None)
    {
        // Split into two, one side will be our payload node unless we are dropping a loose window
        const ImGuiAxis split_axis = (split_dir == ImGuiDir_Left || split_dir == ImGuiDir_Right) ? ImGuiAxis_X : ImGuiAxis_Y;
        let split_inheritor_child_idx: c_int = (split_dir == ImGuiDir_Left || split_dir == ImGuiDir_Up) ? 1 : 0; // Current contents will be moved to the opposite side
        let split_ratio: c_float =  req->DockSplitRatio;
        DockNodeTreeSplit(ctx, node, split_axis, split_inheritor_child_idx, split_ratio, payload_node);  // payload_node may be NULL here!
        ImGuiDockNode* new_node = node.ChildNodes[split_inheritor_child_idx ^ 1];
        new_node.HostWindow = node.HostWindow;
        node = new_node;
    }
    node.SetLocalFlags(node.LocalFlags & !ImGuiDockNodeFlags_HiddenTabBar);

    if (node != payload_node)
    {
        // Create tab bar before we call DockNodeMoveWindows (which would attempt to move the old tab-bar, which would lead us to payload tabs wrongly appearing before target tabs!)
        if (node.Windows.len() > 0 && node.TabBar == null_mut())
        {
            DockNodeAddTabBar(node);
            for (let n: c_int = 0; n < node.Windows.len(); n++)
                TabBarAddTab(node.TabBar, ImGuiTabItemFlags_None, node.Windows[n]);
        }

        if (payload_node != null_mut())
        {
            // Transfer full payload node (with 1+ child windows or child nodes)
            if (payload_node.IsSplitNode())
            {
                if (node.Windows.len() > 0)
                {
                    // We can dock a split payload into a node that already has windows _only_ if our payload is a node tree with a single visible node.
                    // In this situation, we move the windows of the target node into the currently visible node of the payload.
                    // This allows us to preserve some of the underlying dock tree settings nicely.
                    // IM_ASSERT(payload_node->OnlyNodeWithWindows != NULL); // The docking should have been blocked by DockNodePreviewDockSetup() early on and never submitted.
                    ImGuiDockNode* visible_node = payload_node.OnlyNodeWithWindows;
                    if (visible_node.TabBar)
                        // IM_ASSERT(visible_node->TabBar->Tabs.Size > 0);
                    DockNodeMoveWindows(node, visible_node);
                    DockNodeMoveWindows(visible_node, node);
                    DockSettingsRenameNodeReferences(node.ID, visible_node.ID);
                }
                if (node.IsCentralNode())
                {
                    // Central node property needs to be moved to a leaf node, pick the last focused one.
                    // FIXME-DOCK: If we had to transfer other flags here, what would the policy be?
                    ImGuiDockNode* last_focused_node = DockContextFindNodeByID(ctx, payload_node.LastFocusedNodeId);
                    // IM_ASSERT(last_focused_node != NULL);
                    ImGuiDockNode* last_focused_root_node = DockNodeGetRootNode(last_focused_node);
                    // IM_ASSERT(last_focused_root_node == DockNodeGetRootNode(payload_node));
                    last_focused_node.SetLocalFlags(last_focused_node.LocalFlags | ImGuiDockNodeFlags_CentralNode);
                    node.SetLocalFlags(node.LocalFlags & !ImGuiDockNodeFlags_CentralNode);
                    last_focused_root_node.CentralNode = last_focused_node;
                }

                // IM_ASSERT(node->Windows.Size == 0);
                DockNodeMoveChildNodes(node, payload_node);
            }
            else
            {
                let mut payload_dock_id: ImGuiID =  payload_node.ID;
                DockNodeMoveWindows(node, payload_node);
                DockSettingsRenameNodeReferences(payload_dock_id, node.ID);
            }
            DockContextRemoveNode(ctx, payload_node, true);
        }
        else if (payload_window)
        {
            // Transfer single window
            let mut payload_dock_id: ImGuiID =  payload_window.DockId;
            node.VisibleWindow = payload_window;
            DockNodeAddWindow(node, payload_window, true);
            if (payload_dock_id != 0)
                DockSettingsRenameNodeReferences(payload_dock_id, node.ID);
        }
    }
    else
    {
        // When docking a floating single window node we want to reevaluate auto-hiding of the tab bar
        node.WantHiddenTabBarUpdate = true;
    }

    // Update selection immediately
    if (ImGuiTabBar* tab_bar = node.TabBar)
        tab_bar->NextSelectedTabId = next_selected_id;
    MarkIniSettingsDirty();
}

// Problem:
//   Undocking a large (~full screen) window would leave it so large that the bottom right sizing corner would more
//   than likely be off the screen and the window would be hard to resize to fit on screen. This can be particularly problematic
//   with 'ConfigWindowsMoveFromTitleBarOnly=true' and/or with 'ConfigWindowsResizeFromEdges=false' as well (the later can be
//   due to missing ImGuiBackendFlags_HasMouseCursors backend flag).
// Solution:
//   When undocking a window we currently force its maximum size to 90% of the host viewport or monitor.
// Reevaluate this when we implement preserving docked/undocked size ("docking_wip/undocked_size" branch).
static FixLargeWindowsWhenUndocking: ImVec2(size: &ImVec2, ImGuiViewport* ref_viewport)
{
    if (ref_viewport == null_mut())
        return size;

    let g = GImGui; // ImGuiContext& g = *GImGui;
    let max_size: ImVec2 = ImFloor(ref_viewport.WorkSize * 0.900f32);
    if (g.ConfigFlagsCurrFrame & ImGuiConfigFlags_ViewportsEnable)
    {
        let monitor: *const ImGuiPlatformMonitor = GetViewportPlatformMonitor(ref_viewport);
        max_size = ImFloor(monitor->WorkSize * 0.900f32);
    }
    return ImMin(size, max_size);
}

c_void DockContextProcessUndockWindow(ImGuiContext* ctx, window: *mut ImGuiWindow, clear_persistent_docking_re0f32: bool)
{
    ImGuiContext& g = *ctx;
    IMGUI_DEBUG_LOG_DOCKING("[docking] DockContextProcessUndockWindow window '%s', clear_persistent_docking_ref = %d\n", window.Name, clear_persistent_docking_re0f32);
    if (window.DockNode)
        DockNodeRemoveWindow(window.DockNode, window, clear_persistent_docking_ref ? 0 : window.DockId);
    else
        window.DockId = 0;
    window.Collapsed = false;
    window.DockIsActive = false;
    window.DockNodeIsVisible = window.DockTabIsVisible = false;
    window.Size = window.SizeFull = FixLargeWindowsWhenUndocking(window.SizeFull, window.Viewport);

    MarkIniSettingsDirty();
}

c_void DockContextProcessUndockNode(ImGuiContext* ctx, ImGuiDockNode* node)
{
    ImGuiContext& g = *ctx;
    IMGUI_DEBUG_LOG_DOCKING("[docking] DockContextProcessUndockNode node %08X\n", node.ID);
    // IM_ASSERT(node->IsLeafNode());
    // IM_ASSERT(node->Windows.Size >= 1);

    if (node.IsRootNode() || node.IsCentralNode())
    {
        // In the case of a root node or central node, the node will have to stay in place. Create a new node to receive the payload.
        ImGuiDockNode* new_node = DockContextAddNode(ctx, 0);
        new_node.Pos = node.Pos;
        new_node.Size = node.Size;
        new_node.SizeRef = node.SizeRef;
        DockNodeMoveWindows(new_node, node);
        DockSettingsRenameNodeReferences(node.ID, new_node.ID);
        node = new_node;
    }
    else
    {
        // Otherwise extract our node and merge our sibling back into the parent node.
        // IM_ASSERT(node->ParentNode->ChildNodes[0] == node || node->ParentNode->ChildNodes[1] == node);
        let index_in_parent: c_int = (node.ParentNode.ChildNodes[0] == node) ? 0 : 1;
        node.ParentNode.ChildNodes[index_in_parent]= null_mut();
        DockNodeTreeMerge(ctx, node.ParentNode, node.ParentNode.ChildNodes[index_in_parent ^ 1]);
        node.ParentNode.AuthorityForViewport = ImGuiDataAuthority_Window; // The node that stays in place keeps the viewport, so our newly dragged out node will create a new viewport
        node.ParentNode= null_mut();
    }
    for (let n: c_int = 0; n < node.Windows.len(); n++)
    {
        let mut window: *mut ImGuiWindow =  node.Windows[n];
        window.Flags &= !ImGuiWindowFlags_ChildWindow;
        if (window.ParentWindow)
            window.Parentwindow.DC.ChildWindows.find_erase(window);
        UpdateWindowParentAndRootLinks(window, window.Flags, null_mut());
    }
    node.AuthorityForPos = node.AuthorityForSize = ImGuiDataAuthority_DockNode;
    node.Size = FixLargeWindowsWhenUndocking(node.Size, node.Windows[0]->Viewport);
    node.WantMouseMove = true;
    MarkIniSettingsDirty();
}

// This is mostly used for automation.
DockContextCalcDropPosForDocking: bool(target: *mut ImGuiWindow, ImGuiDockNode* target_node, payload_window: *mut ImGuiWindow, ImGuiDockNode* payload_node, split_dir: ImGuiDir, split_outer: bool, out_pos: *mut ImVec2)
{
    // In DockNodePreviewDockSetup() for a root central node instead of showing both "inner" and "outer" drop rects
    // (which would be functionally identical) we only show the outer one. Reflect this here.
    if (target_node && target_node.ParentNode == null_mut() && target_node.IsCentralNode() && split_dir != ImGuiDir_None)
        split_outer = true;
    ImGuiDockPreviewData split_data;
    DockNodePreviewDockSetup(target, target_node, payload_window, payload_node, &split_data, false, split_outer);
    if (split_data.DropRectsDraw[split_dir1].IsInverted())
        return false;
    *out_pos = split_data.DropRectsDraw[split_dir1].GetCenter();
    return true;
}

//-----------------------------------------------------------------------------
// Docking: ImGuiDockNode
//-----------------------------------------------------------------------------
// - DockNodeGetTabOrder()
// - DockNodeAddWindow()
// - DockNodeRemoveWindow()
// - DockNodeMoveChildNodes()
// - DockNodeMoveWindows()
// - DockNodeApplyPosSizeToWindows()
// - DockNodeHideHostWindow()
// - ImGuiDockNodeFindInfoResults
// - DockNodeFindInfo()
// - DockNodeFindWindowByID()
// - DockNodeUpdateFlagsAndCollapse()
// - DockNodeUpdateHasCentralNodeFlag()
// - DockNodeUpdateVisibleFlag()
// - DockNodeStartMouseMovingWindow()
// - DockNodeUpdate()
// - DockNodeUpdateWindowMenu()
// - DockNodeBeginAmendTabBar()
// - DockNodeEndAmendTabBar()
// - DockNodeUpdateTabBar()
// - DockNodeAddTabBar()
// - DockNodeRemoveTabBar()
// - DockNodeIsDropAllowedOne()
// - DockNodeIsDropAllowed()
// - DockNodeCalcTabBarLayout()
// - DockNodeCalcSplitRects()
// - DockNodeCalcDropRectsAndTestMousePos()
// - DockNodePreviewDockSetup()
// - DockNodePreviewDockRender()
//-----------------------------------------------------------------------------

ImGuiDockNode::ImGuiDockNode(id: ImGuiID)
{
    ID = id;
    SharedFlags = LocalFlags = LocalFlagsInWindows = MergedFlags = ImGuiDockNodeFlags_None;
    ParentNode = ChildNodes[0] = ChildNodes[1]= null_mut();
    TabBar= null_mut();
    SplitAxis = ImGuiAxis_None;

    State = ImGuiDockNodeState_Unknown;
    LastBgColor = IM_COL32_WHITE;
    HostWindow = VisibleWindow= null_mut();
    CentralNode = OnlyNodeWithWindows= null_mut();
    CountNodeWithWindows = 0;
    LastFrameAlive = LastFrameActive = LastFrameFocused = -1;
    LastFocusedNodeId = 0;
    SelectedTabId = 0;
    WantCloseTabId = 0;
    AuthorityForPos = AuthorityForSize = ImGuiDataAuthority_DockNode;
    AuthorityForViewport = ImGuiDataAuthority_Auto;
    IsVisible = true;
    IsFocused = HasCloseButton = HasWindowMenuButton = HasCentralNodeChild = false;
    IsBgDrawnThisFrame = false;
    WantCloseAll = WantLockSizeOnce = WantMouseMove = WantHiddenTabBarUpdate = WantHiddenTabBarToggle = false;
}

ImGuiDockNode::!ImGuiDockNode()
{
    IM_DELETE(TabBar);
    TabBar= null_mut();
    ChildNodes[0] = ChildNodes[1]= null_mut();
}

DockNodeGetTabOrder: c_int(window: *mut ImGuiWindow)
{
    ImGuiTabBar* tab_bar = window.DockNode.TabBar;
    if (tab_bar == null_mut())
        return -1;
    ImGuiTabItem* tab = TabBarFindTabByID(tab_bar, window.TabId);
    return tab ? tab_bar->GetTabOrder(tab) : -1;
}

static c_void DockNodeHideWindowDuringHostWindowCreation(window: *mut ImGuiWindow)
{
    window.Hidden = true;
    window.HiddenFramesCanSkipItems = window.Active ? 1 : 2;
}

static c_void DockNodeAddWindow(ImGuiDockNode* node, window: *mut ImGuiWindow, add_to_tab_bar: bool)
{
    let g = GImGui; // ImGuiContext& g = *GImGui; (void)g;
    if (window.DockNode)
    {
        // Can overwrite an existing window.DockNode (e.g. pointing to a disabled DockSpace node)
        // IM_ASSERT(window.DockNode->ID != node->ID);
        DockNodeRemoveWindow(window.DockNode, window, 0);
    }
    // IM_ASSERT(window.DockNode == NULL || window.DockNodeAsHost == NULL);
    IMGUI_DEBUG_LOG_DOCKING("[docking] DockNodeAddWindow node 0x%08X window '%s'\n", node.ID, window.Name);

    // If more than 2 windows appeared on the same frame leading to the creation of a new hosting window,
    // we'll hide windows until the host window is ready. Hide the 1st window after its been output (so it is not visible for one frame).
    // We will call DockNodeHideWindowDuringHostWindowCreation() on ourselves in Begin()
    if (node.HostWindow == null_mut() && node.Windows.len() == 1 && node.Windows[0]->WasActive == false)
        DockNodeHideWindowDuringHostWindowCreation(node.Windows[0]);

    node.Windows.push(window);
    node.WantHiddenTabBarUpdate = true;
    window.DockNode = node;
    window.DockId = node.ID;
    window.DockIsActive = (node.Windows.len() > 1);
    window.DockTabWantClose = false;

    // When reactivating a node with one or two loose window, the window pos/size/viewport are authoritative over the node storage.
    // In particular it is important we init the viewport from the first window so we don't create two viewports and drop one.
    if (node.HostWindow == null_mut() && node.IsFloatingNode())
    {
        if (node.AuthorityForPos == ImGuiDataAuthority_Auto)
            node.AuthorityForPos = ImGuiDataAuthority_Window;
        if (node.AuthorityForSize == ImGuiDataAuthority_Auto)
            node.AuthorityForSize = ImGuiDataAuthority_Window;
        if (node.AuthorityForViewport == ImGuiDataAuthority_Auto)
            node.AuthorityForViewport = ImGuiDataAuthority_Window;
    }

    // Add to tab bar if requested
    if (add_to_tab_bar)
    {
        if (node.TabBar == null_mut())
        {
            DockNodeAddTabBar(node);
            node.TabBar->SelectedTabId = node.TabBar->NextSelectedTabId = node.SelectedTabId;

            // Add existing windows
            for (let n: c_int = 0; n < node.Windows.len() - 1; n++)
                TabBarAddTab(node.TabBar, ImGuiTabItemFlags_None, node.Windows[n]);
        }
        TabBarAddTab(node.TabBar, ImGuiTabItemFlags_Unsorted, window);
    }

    DockNodeUpdateVisibleFlag(node);

    // Update this without waiting for the next time we Begin() in the window, so our host window will have the proper title bar color on its first frame.
    if (node.HostWindow)
        UpdateWindowParentAndRootLinks(window, window.Flags | ImGuiWindowFlags_ChildWindow, node.HostWindow);
}

static c_void DockNodeRemoveWindow(ImGuiDockNode* node, window: *mut ImGuiWindow, ImGuiID save_dock_id)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(window.DockNode == node);
    //IM_ASSERT(window.RootWindowDockTree == node->HostWindow);
    //IM_ASSERT(window.LastFrameActive < g.FrameCount);    // We may call this from Begin()
    // IM_ASSERT(save_dock_id == 0 || save_dock_id == node->ID);
    IMGUI_DEBUG_LOG_DOCKING("[docking] DockNodeRemoveWindow node 0x%08X window '%s'\n", node.ID, window.Name);

    window.DockNode= null_mut();
    window.DockIsActive = window.DockTabWantClose = false;
    window.DockId = save_dock_id;
    window.Flags &= !ImGuiWindowFlags_ChildWindow;
    if (window.ParentWindow)
        window.Parentwindow.DC.ChildWindows.find_erase(window);
    UpdateWindowParentAndRootLinks(window, window.Flags, null_mut()); // Update immediately

    // Remove window
    let mut erased: bool =  false;
    for (let n: c_int = 0; n < node.Windows.len(); n++)
        if (node.Windows[n] == window)
        {
            node.Windows.erase(node.Windows.Data + n);
            erased = true;
            break;
        }
    if (!erased)
        // IM_ASSERT(erased);
    if (node.VisibleWindow == window)
        node.VisibleWindow= null_mut();

    // Remove tab and possibly tab bar
    node.WantHiddenTabBarUpdate = true;
    if (node.TabBar)
    {
        TabBarRemoveTab(node.TabBar, window.TabId);
        let tab_count_threshold_for_tab_bar: c_int = node.IsCentralNode() ? 1 : 2;
        if (node.Windows.len() < tab_count_threshold_for_tab_bar)
            DockNodeRemoveTabBar(node);
    }

    if (node.Windows.len() == 0 && !node.IsCentralNode() && !node.IsDockSpace() && window.DockId != node.ID)
    {
        // Automatic dock node delete themselves if they are not holding at least one tab
        DockContextRemoveNode(&g, node, true);
        return;
    }

    if (node.Windows.len() == 1 && !node.IsCentralNode() && node.HostWindow)
    {
        let mut remaining_window: *mut ImGuiWindow =  node.Windows[0];
        if (node.Hostwindow.ViewportOwned && node.IsRootNode())
        {
            // Transfer viewport back to the remaining loose window
            IMGUI_DEBUG_LOG_VIEWPORT("[viewport] Node %08X transfer Viewport %08X=>%08X for Window '%s'\n", node.ID, node.Hostwindow.Viewport.ID, remaining_window.ID, remaining_window.Name);
            // IM_ASSERT(node->Hostwindow.Viewport->Window == node->HostWindow);
            node.Hostwindow.Viewport.Window = remaining_window;
            node.Hostwindow.Viewport.ID = remaining_window.ID;
        }
        remaining_window.Collapsed = node.Hostwindow.Collapsed;
    }

    // Update visibility immediately is required so the DockNodeUpdateRemoveInactiveChilds() processing can reflect changes up the tree
    DockNodeUpdateVisibleFlag(node);
}

static c_void DockNodeMoveChildNodes(ImGuiDockNode* dst_node, ImGuiDockNode* src_node)
{
    // IM_ASSERT(dst_node->Windows.Size == 0);
    dst_node.ChildNodes[0] = src_node.ChildNodes[0];
    dst_node.ChildNodes[1] = src_node.ChildNodes[1];
    if (dst_node.ChildNodes[0])
        dst_node.ChildNodes[0]->ParentNode = dst_node;
    if (dst_node.ChildNodes[1])
        dst_node.ChildNodes[1]->ParentNode = dst_node;
    dst_node.SplitAxis = src_node.SplitAxis;
    dst_node.SizeRef = src_node.SizeRef;
    src_node.ChildNodes[0] = src_node.ChildNodes[1]= null_mut();
}

static c_void DockNodeMoveWindows(ImGuiDockNode* dst_node, ImGuiDockNode* src_node)
{
    // Insert tabs in the same orders as currently ordered (node->Windows isn't ordered)
    // IM_ASSERT(src_node && dst_node && dst_node != src_node);
    ImGuiTabBar* src_tab_bar = src_node.TabBar;
    if (src_tab_bar != null_mut())
        // IM_ASSERT(src_node->Windows.Size <= src_node->TabBar->Tabs.Size);

    // If the dst_node is empty we can just move the entire tab bar (to preserve selection, scrolling, etc.)
    let mut move_tab_bar: bool =  (src_tab_bar != null_mut()) && (dst_node.TabBar == null_mut());
    if (move_tab_bar)
    {
        dst_node.TabBar = src_node.TabBar;
        src_node.TabBar= null_mut();
    }

    // Tab order is not important here, it is preserved by sorting in DockNodeUpdateTabBar().
    for (window: *mut ImGuiWindow : src_node.Windows)
    {
        window.DockNode= null_mut();
        window.DockIsActive = false;
        DockNodeAddWindow(dst_node, window, !move_tab_bar);
    }
    src_node.Windows.clear();

    if (!move_tab_bar && src_node.TabBar)
    {
        if (dst_node.TabBar)
            dst_node.TabBar->SelectedTabId = src_node.TabBar->SelectedTabId;
        DockNodeRemoveTabBar(src_node);
    }
}

static c_void DockNodeApplyPosSizeToWindows(ImGuiDockNode* node)
{
    for (let n: c_int = 0; n < node.Windows.len(); n++)
    {
        SetWindowPos(node.Windows[n], node.Pos, ImGuiCond_Always); // We don't assign directly to Pos because it can break the calculation of SizeContents on next frame
        SetWindowSize(node.Windows[n], node.Size, ImGuiCond_Always);
    }
}

static c_void DockNodeHideHostWindow(ImGuiDockNode* node)
{
    if (node.HostWindow)
    {
        if (node.Hostwindow.DockNodeAsHost == node)
            node.Hostwindow.DockNodeAsHost= null_mut();
        node.HostWindow= null_mut();
    }

    if (node.Windows.len() == 1)
    {
        node.VisibleWindow = node.Windows[0];
        node.Windows[0]->DockIsActive = false;
    }

    if (node.TabBar)
        DockNodeRemoveTabBar(node);
}

// Search function called once by root node in DockNodeUpdate()
struct ImGuiDockNodeTreeInfo
{
    ImGuiDockNode*      CentralNode;
    ImGuiDockNode*      FirstNodeWithWindows;
    c_int                 CountNodesWithWindows;
    //ImGuiWindowClass  WindowClassForMerges;

    ImGuiDockNodeTreeInfo() { memset(this, 0, sizeof(*this)); }
};

static c_void DockNodeFindInfo(ImGuiDockNode* node, ImGuiDockNodeTreeInfo* info)
{
    if (node.Windows.len() > 0)
    {
        if (info.FirstNodeWithWindows == null_mut())
            info.FirstNodeWithWindows = node;
        info.CountNodesWithWindows+= 1;
    }
    if (node.IsCentralNode())
    {
        // IM_ASSERT(info.CentralNode == NULL); // Should be only one
        // IM_ASSERT(node->IsLeafNode() && "If you get this assert: please submit .ini file + repro of actions leading to this.");
        info.CentralNode = node;
    }
    if (info.CountNodesWithWindows > 1 && info.CentralNode != null_mut())
        return;
    if (node.ChildNodes[0])
        DockNodeFindInfo(node.ChildNodes[0], info);
    if (node.ChildNodes[1])
        DockNodeFindInfo(node.ChildNodes[1], info);
}

static DockNodeFindWindowByID: *mut ImGuiWindow(ImGuiDockNode* node, id: ImGuiID)
{
    // IM_ASSERT(id != 0);
    for (let n: c_int = 0; n < node.Windows.len(); n++)
        if (node.Windows[n]->ID == id)
            return node.Windows[n];
    return null_mut();
}

// - Remove inactive windows/nodes.
// - Update visibility flag.
static c_void DockNodeUpdateFlagsAndCollapse(ImGuiDockNode* node)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(node->ParentNode == NULL || node->ParentNode->ChildNodes[0] == node || node->ParentNode->ChildNodes[1] == node);

    // Inherit most flags
    if (node.ParentNode)
        node.SharedFlags = node.ParentNode.SharedFlags & ImGuiDockNodeFlags_SharedFlagsInheritMask_;

    // Recurse into children
    // There is the possibility that one of our child becoming empty will delete itself and moving its sibling contents into 'node'.
    // If 'node->ChildNode[0]' delete itself, then 'node->ChildNode[1]->Windows' will be moved into 'node'
    // If 'node->ChildNode[1]' delete itself, then 'node->ChildNode[0]->Windows' will be moved into 'node' and the "remove inactive windows" loop will have run twice on those windows (harmless)
    node.HasCentralNodeChild = false;
    if (node.ChildNodes[0])
        DockNodeUpdateFlagsAndCollapse(node.ChildNodes[0]);
    if (node.ChildNodes[1])
        DockNodeUpdateFlagsAndCollapse(node.ChildNodes[1]);

    // Remove inactive windows, collapse nodes
    // Merge node flags overrides stored in windows
    node.LocalFlagsInWindows = ImGuiDockNodeFlags_None;
    for (let window_n: c_int = 0; window_n < node.Windows.len(); window_n++)
    {
        let mut window: *mut ImGuiWindow =  node.Windows[window_n];
        // IM_ASSERT(window.DockNode == node);

        let mut node_was_active: bool =  (node.LastFrameActive + 1 == g.FrameCount);
        let mut remove: bool =  false;
        remove |= node_was_active && (window.LastFrameActive + 1 < g.FrameCount);
        remove |= node_was_active && (node.WantCloseAll || node.WantCloseTabId == window.TabId) && window.HasCloseButton && !(window.Flags & ImGuiWindowFlags_UnsavedDocument);  // Submit all _expected_ closure from last frame
        remove |= (window.DockTabWantClose);
        if (remove)
        {
            window.DockTabWantClose = false;
            if (node.Windows.len() == 1 && !node.IsCentralNode())
            {
                DockNodeHideHostWindow(node);
                node.State = ImGuiDockNodeState_HostWindowHiddenBecauseSingleWindow;
                DockNodeRemoveWindow(node, window, node.ID); // Will delete the node so it'll be invalid on return
                return;
            }
            DockNodeRemoveWindow(node, window, node.ID);
            window_n-= 1;
            continue;
        }

        // FIXME-DOCKING: Missing policies for conflict resolution, hence the "Experimental" tag on this.
        //node->LocalFlagsInWindow &= ~window.WindowClass.DockNodeFlagsOverrideClear;
        node.LocalFlagsInWindows |= window.WindowClass.DockNodeFlagsOverrideSet;
    }
    node.UpdateMergedFlags();

    // Auto-hide tab bar option
    ImGuiDockNodeFlags node_flags = node.MergedFlags;
    if (node.WantHiddenTabBarUpdate && node.Windows.len() == 1 && (node_flags & ImGuiDockNodeFlags_AutoHideTabBar) && !node.IsHiddenTabBar())
        node.WantHiddenTabBarToggle = true;
    node.WantHiddenTabBarUpdate = false;

    // Cancel toggling if we know our tab bar is enforced to be hidden at all times
    if (node.WantHiddenTabBarToggle && node.VisibleWindow && (node.Visiblewindow.WindowClass.DockNodeFlagsOverrideSet & ImGuiDockNodeFlags_HiddenTabBar))
        node.WantHiddenTabBarToggle = false;

    // Apply toggles at a single point of the frame (here!)
    if (node.Windows.len() > 1)
        node.SetLocalFlags(node.LocalFlags & !ImGuiDockNodeFlags_HiddenTabBar);
    else if (node.WantHiddenTabBarToggle)
        node.SetLocalFlags(node.LocalFlags ^ ImGuiDockNodeFlags_HiddenTabBar);
    node.WantHiddenTabBarToggle = false;

    DockNodeUpdateVisibleFlag(node);
}

// This is rarely called as DockNodeUpdateForRootNode() generally does it most frames.
static c_void DockNodeUpdateHasCentralNodeChild(ImGuiDockNode* node)
{
    node.HasCentralNodeChild = false;
    if (node.ChildNodes[0])
        DockNodeUpdateHasCentralNodeChild(node.ChildNodes[0]);
    if (node.ChildNodes[1])
        DockNodeUpdateHasCentralNodeChild(node.ChildNodes[1]);
    if (node.IsRootNode())
    {
        ImGuiDockNode* mark_node = node.CentralNode;
        while (mark_node)
        {
            mark_node.HasCentralNodeChild = true;
            mark_node = mark_node.ParentNode;
        }
    }
}

static c_void DockNodeUpdateVisibleFlag(ImGuiDockNode* node)
{
    // Update visibility flag
    let mut is_visible: bool =  (node.ParentNode == null_mut()) ? node.IsDockSpace() : node.IsCentralNode();
    is_visible |= (node.Windows.len() > 0);
    is_visible |= (node.ChildNodes[0] && node.ChildNodes[0]->IsVisible);
    is_visible |= (node.ChildNodes[1] && node.ChildNodes[1]->IsVisible);
    node.IsVisible = is_visible;
}

static c_void DockNodeStartMouseMovingWindow(ImGuiDockNode* node, window: *mut ImGuiWindow)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(node->WantMouseMove == true);
    StartMouseMovingWindow(window);
    g.ActiveIdClickOffset = g.IO.MouseClickedPos[0] - node.Pos;
    g.MovingWindow = window; // If we are docked into a non moveable root window, StartMouseMovingWindow() won't set g.MovingWindow. Override that decision.
    node.WantMouseMove = false;
}

// Update CentralNode, OnlyNodeWithWindows, LastFocusedNodeID. Copy window class.
static c_void DockNodeUpdateForRootNode(ImGuiDockNode* node)
{
    DockNodeUpdateFlagsAndCollapse(node);

    // - Setup central node pointers
    // - Find if there's only a single visible window in the hierarchy (in which case we need to display a regular title bar -> FIXME-DOCK: that last part is not done yet!)
    // Cannot merge this with DockNodeUpdateFlagsAndCollapse() because FirstNodeWithWindows is found after window removal and child collapsing
    ImGuiDockNodeTreeInfo info;
    DockNodeFindInfo(node, &info);
    node.CentralNode = info.CentralNode;
    node.OnlyNodeWithWindows = (info.CountNodesWithWindows == 1) ? info.FirstNodeWithWindows : null_mut();
    node.CountNodeWithWindows = info.CountNodesWithWindows;
    if (node.LastFocusedNodeId == 0 && info.FirstNodeWithWindows != null_mut())
        node.LastFocusedNodeId = info.FirstNodeWithWindows->ID;

    // Copy the window class from of our first window so it can be used for proper dock filtering.
    // When node has mixed windows, prioritize the class with the most constraint (DockingAllowUnclassed = false) as the reference to copy.
    // FIXME-DOCK: We don't recurse properly, this code could be reworked to work from DockNodeUpdateScanRec.
    if (ImGuiDockNode* first_node_with_windows = info.FirstNodeWithWindows)
    {
        node.WindowClass = first_node_with_windows->Windows[0]->WindowClass;
        for (let n: c_int = 1; n < first_node_with_windows->Windows.len(); n++)
            if (first_node_with_windows->Windows[n]->WindowClass.DockingAllowUnclassed == false)
            {
                node.WindowClass = first_node_with_windows->Windows[n]->WindowClass;
                break;
            }
    }

    ImGuiDockNode* mark_node = node.CentralNode;
    while (mark_node)
    {
        mark_node.HasCentralNodeChild = true;
        mark_node = mark_node.ParentNode;
    }
}

static c_void DockNodeSetupHostWindow(ImGuiDockNode* node, host_window: *mut ImGuiWindow)
{
    // Remove ourselves from any previous different host window
    // This can happen if a user mistakenly does (see #4295 for details):
    //  - N+0: DockBuilderAddNode(id, 0)    // missing ImGuiDockNodeFlags_DockSpace
    //  - N+1: NewFrame()                   // will create floating host window for that node
    //  - N+1: DockSpace(id)                // requalify node as dockspace, moving host window
    if (node.HostWindow && node.HostWindow != host_window && node.Hostwindow.DockNodeAsHost == node)
        node.Hostwindow.DockNodeAsHost= null_mut();

    host_window.DockNodeAsHost = node;
    node.HostWindow = host_window;
}

static c_void DockNodeUpdate(ImGuiDockNode* node)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(node->LastFrameActive != g.FrameCount);
    node.LastFrameAlive = g.FrameCount;
    node.IsBgDrawnThisFrame = false;

    node.CentralNode = node.OnlyNodeWithWindows= null_mut();
    if (node.IsRootNode())
        DockNodeUpdateForRootNode(node);

    // Remove tab bar if not needed
    if (node.TabBar && node.IsNoTabBar())
        DockNodeRemoveTabBar(node);

    // Early out for hidden root dock nodes (when all DockId references are in inactive windows, or there is only 1 floating window holding on the DockId)
    let mut want_to_hide_host_window: bool =  false;
    if (node.IsFloatingNode())
    {
        if (node.Windows.len() <= 1 && node.IsLeafNode())
            if (!g.IO.ConfigDockingAlwaysTabBar && (node.Windows.len() == 0 || !node.Windows[0]->WindowClass.DockingAlwaysTabBar))
                want_to_hide_host_window = true;
        if (node.CountNodeWithWindows == 0)
            want_to_hide_host_window = true;
    }
    if (want_to_hide_host_window)
    {
        if (node.Windows.len() == 1)
        {
            // Floating window pos/size is authoritative
            let mut single_window: *mut ImGuiWindow =  node.Windows[0];
            node.Pos = single_window.Pos;
            node.Size = single_window.SizeFull;
            node.AuthorityForPos = node.AuthorityForSize = node.AuthorityForViewport = ImGuiDataAuthority_Window;

            // Transfer focus immediately so when we revert to a regular window it is immediately selected
            if (node.HostWindow && g.NavWindow == node.HostWindow)
                FocusWindow(single_window);
            if (node.HostWindow)
            {
                single_window.Viewport = node.Hostwindow.Viewport;
                single_window.ViewportId = node.Hostwindow.ViewportId;
                if (node.Hostwindow.ViewportOwned)
                {
                    single_window.Viewport.Window = single_window;
                    single_window.ViewportOwned = true;
                }
            }
        }

        DockNodeHideHostWindow(node);
        node.State = ImGuiDockNodeState_HostWindowHiddenBecauseSingleWindow;
        node.WantCloseAll = false;
        node.WantCloseTabId = 0;
        node.HasCloseButton = node.HasWindowMenuButton = false;
        node.LastFrameActive = g.FrameCount;

        if (node.WantMouseMove && node.Windows.len() == 1)
            DockNodeStartMouseMovingWindow(node, node.Windows[0]);
        return;
    }

    // In some circumstance we will defer creating the host window (so everything will be kept hidden),
    // while the expected visible window is resizing itself.
    // This is important for first-time (no ini settings restored) single window when io.ConfigDockingAlwaysTabBar is enabled,
    // otherwise the node ends up using the minimum window size. Effectively those windows will take an extra frame to show up:
    //   N+0: Begin(): window created (with no known size), node is created
    //   N+1: DockNodeUpdate(): node skip creating host window / Begin(): window size applied, not visible
    //   N+2: DockNodeUpdate(): node can create host window / Begin(): window becomes visible
    // We could remove this frame if we could reliably calculate the expected window size during node update, before the Begin() code.
    // It would require a generalization of CalcWindowExpectedSize(), probably extracting code away from Begin().
    // In reality it isn't very important as user quickly ends up with size data in .ini file.
    if (node.IsVisible && node.HostWindow == null_mut() && node.IsFloatingNode() && node.IsLeafNode())
    {
        // IM_ASSERT(node->Windows.Size > 0);
        let mut ref_window: *mut ImGuiWindow =  null_mut();
        if (node.SelectedTabId != 0) // Note that we prune single-window-node settings on .ini loading, so this is generally 0 for them!
            ref_window = DockNodeFindWindowByID(node, node.SelectedTabId);
        if (ref_window == null_mut())
            ref_window = node.Windows[0];
        if (ref_window.AutoFitFramesX > 0 || ref_window.AutoFitFramesY > 0)
        {
            node.State = ImGuiDockNodeState_HostWindowHiddenBecauseWindowsAreResizing;
            return;
        }
    }

    const ImGuiDockNodeFlags node_flags = node.MergedFlags;

    // Decide if the node will have a close button and a window menu button
    node.HasWindowMenuButton = (node.Windows.len() > 0) && (node_flags & ImGuiDockNodeFlags_NoWindowMenuButton) == 0;
    node.HasCloseButton = false;
    for (let window_n: c_int = 0; window_n < node.Windows.len(); window_n++)
    {
        // FIXME-DOCK: Setting DockIsActive here means that for single active window in a leaf node, DockIsActive will be cleared until the next Begin() call.
        let mut window: *mut ImGuiWindow =  node.Windows[window_n];
        node.HasCloseButton |= window.HasCloseButton;
        window.DockIsActive = (node.Windows.len() > 1);
    }
    if (node_flags & ImGuiDockNodeFlags_NoCloseButton)
        node.HasCloseButton = false;

    // Bind or create host window
    let mut host_window: *mut ImGuiWindow =  null_mut();
    let mut beginned_into_host_window: bool =  false;
    if (node.IsDockSpace())
    {
        // [Explicit root dockspace node]
        // IM_ASSERT(node->HostWindow);
        host_window = node.HostWindow;
    }
    else
    {
        // [Automatic root or child nodes]
        if (node.IsRootNode() && node.IsVisible)
        {
            let mut ref_window: *mut ImGuiWindow =  (node.Windows.len() > 0) ? node.Windows[0] : null_mut();

            // Sync Pos
            if (node.AuthorityForPos == ImGuiDataAuthority_Window && ref_window)
                SetNextWindowPos(ref_window.Pos);
            else if (node.AuthorityForPos == ImGuiDataAuthority_DockNode)
                SetNextWindowPos(node.Pos);

            // Sync Size
            if (node.AuthorityForSize == ImGuiDataAuthority_Window && ref_window)
                SetNextWindowSize(ref_window.SizeFull);
            else if (node.AuthorityForSize == ImGuiDataAuthority_DockNode)
                SetNextWindowSize(node.Size);

            // Sync Collapsed
            if (node.AuthorityForSize == ImGuiDataAuthority_Window && ref_window)
                SetNextWindowCollapsed(ref_window.Collapsed);

            // Sync Viewport
            if (node.AuthorityForViewport == ImGuiDataAuthority_Window && ref_window)
                SetNextWindowViewport(ref_window.ViewportId);

            SetNextWindowClass(&node.WindowClass);

            // Begin into the host window
            window_label: [c_char;20];
            DockNodeGetHostWindowTitle(node, window_label, window_label.len());
            window_flags: ImGuiWindowFlags = ImGuiWindowFlags_NoScrollbar | ImGuiWindowFlags_NoScrollWithMouse | ImGuiWindowFlags_DockNodeHost;
            window_flags |= ImGuiWindowFlags_NoFocusOnAppearing;
            window_flags |= ImGuiWindowFlags_NoSavedSettings | ImGuiWindowFlags_NoNavFocus | ImGuiWindowFlags_NoCollapse;
            window_flags |= ImGuiWindowFlags_NoTitleBar;

            SetNextWindowBgAlpha(0.0); // Don't set ImGuiWindowFlags_NoBackground because it disables borders
            PushStyleVar(ImGuiStyleVar_WindowPadding, ImVec2::new(0, 0));
            Begin(window_label, null_mut(), window_flags);
            PopStyleVar();
            beginned_into_host_window = true;

            host_window = g.CurrentWindow;
            DockNodeSetupHostWindow(node, host_window);
            host_window.DC.CursorPos = host_window.Pos;
            node.Pos = host_window.Pos;
            node.Size = host_window.Size;

            // We set ImGuiWindowFlags_NoFocusOnAppearing because we don't want the host window to take full focus (e.g. steal NavWindow)
            // But we still it bring it to the front of display. There's no way to choose this precise behavior via window flags.
            // One simple case to ponder if: window A has a toggle to create windows B/C/D. Dock B/C/D together, clear the toggle and enable it again.
            // When reappearing B/C/D will request focus and be moved to the top of the display pile, but they are not linked to the dock host window
            // during the frame they appear. The dock host window would keep its old display order, and the sorting in EndFrame would move B/C/D back
            // after the dock host window, losing their top-most status.
            if (node.Hostwindow.Appearing)
                BringWindowToDisplayFront(node.HostWindow);

            node.AuthorityForPos = node.AuthorityForSize = node.AuthorityForViewport = ImGuiDataAuthority_Auto;
        }
        else if (node.ParentNode)
        {
            node.HostWindow = host_window = node.ParentNode.HostWindow;
            node.AuthorityForPos = node.AuthorityForSize = node.AuthorityForViewport = ImGuiDataAuthority_Auto;
        }
        if (node.WantMouseMove && node.HostWindow)
            DockNodeStartMouseMovingWindow(node, node.HostWindow);
    }

    // Update focused node (the one whose title bar is highlight) within a node tree
    if (node.IsSplitNode())
        // IM_ASSERT(node->TabBar == NULL);
    if (node.IsRootNode())
        if (g.NavWindow && g.NavWindow.Rootwindow.DockNode && g.NavWindow.Rootwindow.ParentWindow == host_window)
            node.LastFocusedNodeId = g.NavWindow.Rootwindow.DockNode.ID;

    // Register a hit-test hole in the window unless we are currently dragging a window that is compatible with our dockspace
    ImGuiDockNode* central_node = node.CentralNode;
    let central_node_hole: bool = node.IsRootNode() && host_window && (node_flags & ImGuiDockNodeFlags_PassthruCentralNode) != 0 && central_node != null_mut() && central_node.IsEmpty();
    let mut central_node_hole_register_hit_test_hole: bool =  central_node_hole;
    if (central_node_hole)
        if (*const ImGuiPayload payload = GetDragDropPayload())
            if (payload->IsDataType(IMGUI_PAYLOAD_TYPE_WINDOW) && DockNodeIsDropAllowed(host_window, *(ImGuiWindow**)payload->Data))
                central_node_hole_register_hit_test_hole = false;
    if (central_node_hole_register_hit_test_hole)
    {
        // We add a little padding to match the "resize from edges" behavior and allow grabbing the splitter easily.
        // (But we only add it if there's something else on the other side of the hole, otherwise for e.g. fullscreen
        // covering passthru node we'd have a gap on the edge not covered by the hole)
        // IM_ASSERT(node->IsDockSpace()); // We cannot pass this flag without the DockSpace() api. Testing this because we also setup the hole in host_window.ParentNode
        ImGuiDockNode* root_node = DockNodeGetRootNode(central_node);
        let mut root_rect: ImRect = ImRect::new(root_node.Pos, root_node.Pos + root_node.Size);
        let mut hole_rect: ImRect = ImRect::new(central_node.Pos, central_node.Pos + central_node.Size);
        if (hole_rect.Min.x > root_rect.Min.x) { hole_rect.Min.x += WINDOWS_HOVER_PADDING; }
        if (hole_rect.Max.x < root_rect.Max.x) { hole_rect.Max.x -= WINDOWS_HOVER_PADDING; }
        if (hole_rect.Min.y > root_rect.Min.y) { hole_rect.Min.y += WINDOWS_HOVER_PADDING; }
        if (hole_rect.Max.y < root_rect.Max.y) { hole_rect.Max.y -= WINDOWS_HOVER_PADDING; }
        //GetForegroundDrawList()->AddRect(hole_rect.Min, hole_rect.Max, IM_COL32(255, 0, 0, 255));
        if (central_node_hole && !hole_rect.IsInverted())
        {
            SetWindowHitTestHole(host_window, hole_rect.Min, hole_rect.Max - hole_rect.Min);
            if (host_window.ParentWindow)
                SetWindowHitTestHole(host_window.ParentWindow, hole_rect.Min, hole_rect.Max - hole_rect.Min);
        }
    }

    // Update position/size, process and draw resizing splitters
    if (node.IsRootNode() && host_window)
    {
        DockNodeTreeUpdatePosSize(node, host_window.Pos, host_window.Size);
        DockNodeTreeUpdateSplitter(node);
    }

    // Draw empty node background (currently can only be the Central Node)
    if (host_window && node.IsEmpty() && node.IsVisible)
    {
        host_window.DrawList.ChannelsSetCurrent(DOCKING_HOST_DRAW_CHANNEL_BG);
        node.LastBgColor = (node_flags & ImGuiDockNodeFlags_PassthruCentralNode) ? 0 : GetColorU32(ImGuiCol_DockingEmptyBg);
        if (node.LastBgColor != 0)
            host_window.DrawList.AddRectFilled(node.Pos, node.Pos + node.Size, node.LastBgColor);
        node.IsBgDrawnThisFrame = true;
    }

    // Draw whole dockspace background if ImGuiDockNodeFlags_PassthruCentralNode if set.
    // We need to draw a background at the root level if requested by ImGuiDockNodeFlags_PassthruCentralNode, but we will only know the correct pos/size
    // _after_ processing the resizing splitters. So we are using the DrawList channel splitting facility to submit drawing primitives out of order!
    let render_dockspace_bg: bool = node.IsRootNode() && host_window && (node_flags & ImGuiDockNodeFlags_PassthruCentralNode) != 0;
    if (render_dockspace_bg && node.IsVisible)
    {
        host_window.DrawList.ChannelsSetCurrent(DOCKING_HOST_DRAW_CHANNEL_BG);
        if (central_node_hole)
            RenderRectFilledWithHole(host_window.DrawList, node.Rect(), central_node.Rect(), GetColorU32(ImGuiCol_WindowBg), 0.0);
        else
            host_window.DrawList.AddRectFilled(node.Pos, node.Pos + node.Size, GetColorU32(ImGuiCol_WindowBg), 0.0);
    }

    // Draw and populate Tab Bar
    if (host_window)
        host_window.DrawList.ChannelsSetCurrent(DOCKING_HOST_DRAW_CHANNEL_FG);
    if (host_window && node.Windows.len() > 0)
    {
        DockNodeUpdateTabBar(node, host_window);
    }
    else
    {
        node.WantCloseAll = false;
        node.WantCloseTabId = 0;
        node.IsFocused = false;
    }
    if (node.TabBar && node.TabBar->SelectedTabId)
        node.SelectedTabId = node.TabBar->SelectedTabId;
    else if (node.Windows.len() > 0)
        node.SelectedTabId = node.Windows[0]->TabId;

    // Draw payload drop target
    if (host_window && node.IsVisible)
        if (node.IsRootNode() && (g.MovingWindow == null_mut() || g.Movingwindow.RootWindowDockTree != host_window))
            BeginDockableDragDropTarget(host_window);

    // We update this after DockNodeUpdateTabBar()
    node.LastFrameActive = g.FrameCount;

    // Recurse into children
    // FIXME-DOCK FIXME-OPT: Should not need to recurse into children
    if (host_window)
    {
        if (node.ChildNodes[0])
            DockNodeUpdate(node.ChildNodes[0]);
        if (node.ChildNodes[1])
            DockNodeUpdate(node.ChildNodes[1]);

        // Render outer borders last (after the tab bar)
        if (node.IsRootNode())
            RenderWindowOuterBorders(host_window);
    }

    // End host window
    if (beginned_into_host_window) //-V1020
        End();
}

// Compare TabItem nodes given the last known DockOrder (will persist in .ini file as hint), used to sort tabs when multiple tabs are added on the same frame.
static IMGUI_CDECL: c_int TabItemComparerByDockOrder(*const c_void lhs, *const c_void rhs)
{
    let mut a: *mut ImGuiWindow =  ((*const ImGuiTabItem)lhs)->Window;
    let mut b: *mut ImGuiWindow =  ((*const ImGuiTabItem)rhs)->Window;
    if (let d: c_int = ((a->DockOrder == -1) ? INT_MAX : a->DockOrder) - ((b->DockOrder == -1) ? INT_MAX : b->DockOrder))
        return d;
    return (a->BeginOrderWithinContext - b->BeginOrderWithinContext);
}

static ImGuiID DockNodeUpdateWindowMenu(ImGuiDockNode* node, ImGuiTabBar* tab_bar)
{
    // Try to position the menu so it is more likely to stays within the same viewport
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut ret_tab_id: ImGuiID =  0;
    if (g.Style.WindowMenuButtonPosition == ImGuiDir_Left)
        SetNextWindowPos(ImVec2::new(node.Pos.x, node.Pos.y + GetFrameHeight()), ImGuiCond_Always, ImVec2::new(0.0, 0.0));
    else
        SetNextWindowPos(ImVec2::new(node.Pos.x + node.Size.x, node.Pos.y + GetFrameHeight()), ImGuiCond_Always, ImVec2::new(1.0, 0.0));
    if (BeginPopup("#WindowMenu"))
    {
        node.IsFocused = true;
        if (tab_bar->Tabs.Size == 1)
        {
            if (MenuItem("Hide tab bar", null_mut(), node.IsHiddenTabBar()))
                node.WantHiddenTabBarToggle = true;
        }
        else
        {
            for (let tab_n: c_int = 0; tab_n < tab_bar->Tabs.Size; tab_n++)
            {
                ImGuiTabItem* tab = &tab_bar->Tabs[tab_n];
                if (tab->Flags & ImGuiTabItemFlags_Button)
                    continue;
                if (Selectable(tab_bar->GetTabName(tab), tab->ID == tab_bar->SelectedTabId))
                    ret_tab_id = tab->ID;
                SameLine();
                Text("   ");
            }
        }
        EndPopup();
    }
    return ret_tab_id;
}

// User helper to append/amend into a dock node tab bar. Most commonly used to add e.g. a "+" button.
DockNodeBeginAmendTabBar: bool(ImGuiDockNode* node)
{
    if (node.TabBar == null_mut() || node.HostWindow == null_mut())
        return false;
    if (node.MergedFlags & ImGuiDockNodeFlags_KeepAliveOnly)
        return false;
    Begin(node.Hostwindow.Name);
    PushOverrideID(node.ID);
    let mut ret: bool =  BeginTabBarEx(node.TabBar, node.TabBar->BarRect, node.TabBar->Flags, node);
    IM_UNUSED(ret);
    // IM_ASSERT(ret);
    return true;
}

c_void DockNodeEndAmendTabBar()
{
    EndTabBar();
    PopID();
    End();
}

static IsDockNodeTitleBarHighlighted: bool(ImGuiDockNode* node, ImGuiDockNode* root_node, host_window: *mut ImGuiWindow)
{
    // CTRL+Tab highlight (only highlighting leaf node, not whole hierarchy)
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.NavWindowingTarget)
        return (g.NavWindowingTarget->DockNode == node);

    // FIXME-DOCKING: May want alternative to treat central node void differently? e.g. if (g.NavWindow == host_window)
    if (g.NavWindow && g.NavWindow.RootWindowForTitleBarHighlight == host_window.RootWindowDockTree && root_node.LastFocusedNodeId == node.ID)
        for (ImGuiDockNode* parent_node = g.NavWindow.Rootwindow.DockNode; parent_node != null_mut(); parent_node = parent_node.HostWindow ? parent_node.Hostwindow.Rootwindow.DockNode : null_mut())
            if ((parent_node = DockNodeGetRootNode(parent_node)) == root_node)
                return true;
    return false;
}

// Submit the tab bar corresponding to a dock node and various housekeeping details.
static c_void DockNodeUpdateTabBar(ImGuiDockNode* node, host_window: *mut ImGuiWindow)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiStyle& style = g.Style;

    let node_was_active: bool = (node.LastFrameActive + 1 == g.FrameCount);
    let closed_all: bool = node.WantCloseAll && node_was_active;
    let mut closed_one: ImGuiID =  node.WantCloseTabId && node_was_active;
    node.WantCloseAll = false;
    node.WantCloseTabId = 0;

    // Decide if we should use a focused title bar color
    let mut is_focused: bool =  false;
    ImGuiDockNode* root_node = DockNodeGetRootNode(node);
    if (IsDockNodeTitleBarHighlighted(node, root_node, host_window))
        is_focused = true;

    // Hidden tab bar will show a triangle on the upper-left (in Begin)
    if (node.IsHiddenTabBar() || node.IsNoTabBar())
    {
        node.VisibleWindow = (node.Windows.len() > 0) ? node.Windows[0] : null_mut();
        node.IsFocused = is_focused;
        if (is_focused)
            node.LastFrameFocused = g.FrameCount;
        if (node.VisibleWindow)
        {
            // Notify root of visible window (used to display title in OS task bar)
            if (is_focused || root_node.VisibleWindow == null_mut())
                root_node.VisibleWindow = node.VisibleWindow;
            if (node.TabBar)
                node.TabBar->VisibleTabId = node.Visiblewindow.TabId;
        }
        return;
    }

    // Move ourselves to the Menu layer (so we can be accessed by tapping Alt) + undo SkipItems flag in order to draw over the title bar even if the window is collapsed
    let mut backup_skip_item: bool =  host_window.SkipItems;
    if (!node.IsDockSpace())
    {
        host_window.SkipItems = false;
        host_window.DC.NavLayerCurrent = ImGuiNavLayer_Menu;
    }

    // Use PushOverrideID() instead of PushID() to use the node id _without_ the host window ID.
    // This is to facilitate computing those ID from the outside, and will affect more or less only the ID of the collapse button, popup and tabs,
    // as docked windows themselves will override the stack with their own root ID.
    PushOverrideID(node.ID);
    ImGuiTabBar* tab_bar = node.TabBar;
    let mut tab_bar_is_recreated: bool =  (tab_bar == null_mut()); // Tab bar are automatically destroyed when a node gets hidden
    if (tab_bar == null_mut())
    {
        DockNodeAddTabBar(node);
        tab_bar = node.TabBar;
    }

    let mut focus_tab_id: ImGuiID =  0;
    node.IsFocused = is_focused;

    const ImGuiDockNodeFlags node_flags = node.MergedFlags;
    let has_window_menu_button: bool = (node_flags & ImGuiDockNodeFlags_NoWindowMenuButton) == 0 && (style.WindowMenuButtonPosition != ImGuiDir_None);

    // In a dock node, the Collapse Button turns into the Window Menu button.
    // FIXME-DOCK FIXME-OPT: Could we recycle popups id across multiple dock nodes?
    if (has_window_menu_button && IsPopupOpen("#WindowMenu"))
    {
        if (let mut tab_id: ImGuiID =  DockNodeUpdateWindowMenu(node, tab_bar))
            focus_tab_id = tab_bar->NextSelectedTabId = tab_id;
        is_focused |= node.IsFocused;
    }

    // Layout
    ImRect title_bar_rect, tab_bar_rect;
    window_menu_button_pos: ImVec2;
    close_button_pos: ImVec2;
    DockNodeCalcTabBarLayout(node, &title_bar_rect, &tab_bar_rect, &window_menu_button_pos, &close_button_pos);

    // Submit new tabs, they will be added as Unsorted and sorted below based on relative DockOrder value.
    let tabs_count_old: c_int = tab_bar->Tabs.Size;
    for (let window_n: c_int = 0; window_n < node.Windows.len(); window_n++)
    {
        let mut window: *mut ImGuiWindow =  node.Windows[window_n];
        if (TabBarFindTabByID(tab_bar, window.TabId) == null_mut())
            TabBarAddTab(tab_bar, ImGuiTabItemFlags_Unsorted, window);
    }

    // Title bar
    if (is_focused)
        node.LastFrameFocused = g.FrameCount;
    title_bar_col: u32 = GetColorU32(host_window.Collapsed ? ImGuiCol_TitleBgCollapsed : is_focused ? ImGuiCol_TitleBgActive : ImGuiCol_TitleBg);
    rounding_flags: ImDrawFlags = CalcRoundingFlagsForRectInRect(title_bar_rect, host_window.Rect(), DOCKING_SPLITTER_SIZE);
    host_window.DrawList.AddRectFilled(title_bar_rect.Min, title_bar_rect.Max, title_bar_col, host_window.WindowRounding, rounding_flags);

    // Docking/Collapse button
    if (has_window_menu_button)
    {
        if (CollapseButton(host_window.GetID("#COLLAPSE"), window_menu_button_pos, node)) // == DockNodeGetWindowMenuButtonId(node)
            OpenPopup("#WindowMenu");
        if (IsItemActive())
            focus_tab_id = tab_bar->SelectedTabId;
    }

    // If multiple tabs are appearing on the same frame, sort them based on their persistent DockOrder value
    let tabs_unsorted_start: c_int = tab_bar->Tabs.Size;
    for (let tab_n: c_int = tab_bar->Tabs.Size - 1; tab_n >= 0 && (tab_bar->Tabs[tab_n].Flags & ImGuiTabItemFlags_Unsorted); tab_n--)
    {
        // FIXME-DOCK: Consider only clearing the flag after the tab has been alive for a few consecutive frames, allowing late comers to not break sorting?
        tab_bar->Tabs[tab_n].Flags &= !ImGuiTabItemFlags_Unsorted;
        tabs_unsorted_start = tab_n;
    }
    if (tab_bar->Tabs.Size > tabs_unsorted_start)
    {
        IMGUI_DEBUG_LOG_DOCKING("[docking] In node 0x%08X: %d new appearing tabs:%s\n", node.ID, tab_bar->Tabs.Size - tabs_unsorted_start, (tab_bar->Tabs.Size > tabs_unsorted_start + 1) ? " (will sort)" : "");
        for (let tab_n: c_int = tabs_unsorted_start; tab_n < tab_bar->Tabs.Size; tab_n++)
            IMGUI_DEBUG_LOG_DOCKING("[docking] - Tab '%s' Order %d\n", tab_bar->Tabs[tab_n].window.Name, tab_bar->Tabs[tab_n].window.DockOrder);
        if (tab_bar->Tabs.Size > tabs_unsorted_start + 1)
            ImQsort(tab_bar->Tabs.Data + tabs_unsorted_start, tab_bar->Tabs.Size - tabs_unsorted_start, sizeof(ImGuiTabItem), TabItemComparerByDockOrder);
    }

    // Apply NavWindow focus back to the tab bar
    if (g.NavWindow && g.NavWindow.Rootwindow.DockNode == node)
        tab_bar->SelectedTabId = g.NavWindow.Rootwindow.TabId;

    // Selected newly added tabs, or persistent tab ID if the tab bar was just recreated
    if (tab_bar_is_recreated && TabBarFindTabByID(tab_bar, node.SelectedTabId) != null_mut())
        tab_bar->SelectedTabId = tab_bar->NextSelectedTabId = node.SelectedTabId;
    else if (tab_bar->Tabs.Size > tabs_count_old)
        tab_bar->SelectedTabId = tab_bar->NextSelectedTabId = tab_bar->Tabs.last().unwrap().window.TabId;

    // Begin tab bar
    ImGuiTabBarFlags tab_bar_flags = ImGuiTabBarFlags_Reorderable | ImGuiTabBarFlags_AutoSelectNewTabs; // | ImGuiTabBarFlags_NoTabListScrollingButtons);
    tab_bar_flags |= ImGuiTabBarFlags_SaveSettings | ImGuiTabBarFlags_DockNode;
    if (!host_window.Collapsed && is_focused)
        tab_bar_flags |= ImGuiTabBarFlags_IsFocused;
    BeginTabBarEx(tab_bar, tab_bar_rect, tab_bar_flags, node);
    //host_window.DrawList.AddRect(tab_bar_rect.Min, tab_bar_rect.Max, IM_COL32(255,0,255,255));

    // Backup style colors
    backup_style_cols: ImVec4[ImGuiWindowDockStyleCol_COUNT];
    for (let color_n: c_int = 0; color_n < ImGuiWindowDockStyleCol_COUNT; color_n++)
        backup_style_cols[color_n] = g.Style.Colors[GWindowDockStyleColors[color_n]];

    // Submit actual tabs
    node.VisibleWindow= null_mut();
    for (let window_n: c_int = 0; window_n < node.Windows.len(); window_n++)
    {
        let mut window: *mut ImGuiWindow =  node.Windows[window_n];
        if ((closed_all || closed_one == window.TabId) && window.HasCloseButton && !(window.Flags & ImGuiWindowFlags_UnsavedDocument))
            continue;
        if (window.LastFrameActive + 1 >= g.FrameCount || !node_was_active)
        {
            ImGuiTabItemFlags tab_item_flags = 0;
            tab_item_flags |= window.WindowClass.TabItemFlagsOverrideSet;
            if (window.Flags & ImGuiWindowFlags_UnsavedDocument)
                tab_item_flags |= ImGuiTabItemFlags_UnsavedDocument;
            if (tab_bar->Flags & ImGuiTabBarFlags_NoCloseWithMiddleMouseButton)
                tab_item_flags |= ImGuiTabItemFlags_NoCloseWithMiddleMouseButton;

            // Apply stored style overrides for the window
            for (let color_n: c_int = 0; color_n < ImGuiWindowDockStyleCol_COUNT; color_n++)
                g.Style.Colors[GWindowDockStyleColors[color_n]] = ColorConvertU32ToFloat4(window.DockStyle.Colors[color_n]);

            // Note that TabItemEx() calls TabBarCalcTabID() so our tab item ID will ignore the current ID stack (rightly so)
            let mut tab_open: bool =  true;
            TabItemEx(tab_bar, window.Name, window.HasCloseButton ? &tab_open : null_mut(), tab_item_flags, window);
            if (!tab_open)
                node.WantCloseTabId = window.TabId;
            if (tab_bar->VisibleTabId == window.TabId)
                node.VisibleWindow = window;

            // Store last item data so it can be queried with IsItemXXX functions after the user Begin() call
            window.DockTabItemStatusFlags = g.LastItemData.StatusFlags;
            window.DockTabItemRect = g.LastItemData.Rect;

            // Update navigation ID on menu layer
            if (g.NavWindow && g.NavWindow.RootWindow == window && (window.DC.NavLayersActiveMask & (1 << ImGuiNavLayer_Menu)) == 0)
                host_window.NavLastIds[1] = window.TabId;
        }
    }

    // Restore style colors
    for (let color_n: c_int = 0; color_n < ImGuiWindowDockStyleCol_COUNT; color_n++)
        g.Style.Colors[GWindowDockStyleColors[color_n]] = backup_style_cols[color_n];

    // Notify root of visible window (used to display title in OS task bar)
    if (node.VisibleWindow)
        if (is_focused || root_node.VisibleWindow == null_mut())
            root_node.VisibleWindow = node.VisibleWindow;

    // Close button (after VisibleWindow was updated)
    // Note that VisibleWindow may have been overrided by CTRL+Tabbing, so Visiblewindow.TabId may be != from tab_bar->SelectedTabId
    let close_button_is_enabled: bool = node.HasCloseButton && node.VisibleWindow && node.Visiblewindow.HasCloseButton;
    let close_button_is_visible: bool = node.HasCloseButton;
    //let close_button_is_visible: bool = close_button_is_enabled; // Most people would expect this behavior of not even showing the button (leaving a hole since we can't claim that space as other windows in the tba bar have one)
    if (close_button_is_visible)
    {
        if (!close_button_is_enabled)
        {
            PushItemFlag(ImGuiItemFlags_Disabled, true);
            PushStyleColor(ImGuiCol_Text, style.Colors[ImGuiCol_Text] * ImVec4(1.0,1.0,1.0,0.40f32));
        }
        if (CloseButton(host_window.GetID("#CLOSE"), close_button_pos))
        {
            node.WantCloseAll = true;
            for (let n: c_int = 0; n < tab_bar->Tabs.Size; n++)
                TabBarCloseTab(tab_bar, &tab_bar->Tabs[n]);
        }
        //if (IsItemActive())
        //    focus_tab_id = tab_bar->SelectedTabId;
        if (!close_button_is_enabled)
        {
            PopStyleColor();
            PopItemFlag();
        }
    }

    // When clicking on the title bar outside of tabs, we still focus the selected tab for that node
    // FIXME: TabItem use AllowItemOverlap so we manually perform a more specific test for now (hovered || held)
    let mut title_bar_id: ImGuiID =  host_window.GetID("#TITLEBAR");
    if (g.HoveredId == 0 || g.HoveredId == title_bar_id || g.ActiveId == title_bar_id)
    {
        held: bool;
        ButtonBehavior(title_bar_rect, title_bar_id, null_mut(), &held, ImGuiButtonFlags_AllowItemOverlap);
        if (g.HoveredId == title_bar_id)
        {
            // ImGuiButtonFlags_AllowItemOverlap + SetItemAllowOverlap() required for appending into dock node tab bar,
            // otherwise dragging window will steal HoveredId and amended tabs cannot get them.
            g.LastItemData.ID = title_bar_id;
            SetItemAllowOverlap();
        }
        if (held)
        {
            if (IsMouseClicked(0))
                focus_tab_id = tab_bar->SelectedTabId;

            // Forward moving request to selected window
            if (ImGuiTabItem* tab = TabBarFindTabByID(tab_bar, tab_bar->SelectedTabId))
                StartMouseMovingWindowOrNode(tab->Window ? tab->Window : node.HostWindow, node, false);
        }
    }

    // Forward focus from host node to selected window
    //if (is_focused && g.NavWindow == host_window && !g.NavWindowingTarget)
    //    focus_tab_id = tab_bar->SelectedTabId;

    // When clicked on a tab we requested focus to the docked child
    // This overrides the value set by "forward focus from host node to selected window".
    if (tab_bar->NextSelectedTabId)
        focus_tab_id = tab_bar->NextSelectedTabId;

    // Apply navigation focus
    if (focus_tab_id != 0)
        if (ImGuiTabItem* tab = TabBarFindTabByID(tab_bar, focus_tab_id))
            if (tab->Window)
            {
                FocusWindow(tab->Window);
                NavInitWindow(tab->Window, false);
            }

    EndTabBar();
    PopID();

    // Restore SkipItems flag
    if (!node.IsDockSpace())
    {
        host_window.DC.NavLayerCurrent = ImGuiNavLayer_Main;
        host_window.SkipItems = backup_skip_item;
    }
}

static c_void DockNodeAddTabBar(ImGuiDockNode* node)
{
    // IM_ASSERT(node->TabBar == NULL);
    node.TabBar = IM_NEW(ImGuiTabBar);
}

static c_void DockNodeRemoveTabBar(ImGuiDockNode* node)
{
    if (node.TabBar == null_mut())
        return;
    IM_DELETE(node.TabBar);
    node.TabBar= null_mut();
}

static DockNodeIsDropAllowedOne: bool(payload: *mut ImGuiWindow, host_window: *mut ImGuiWindow)
{
    if (host_window.DockNodeAsHost && host_window.DockNodeAsHost->IsDockSpace() && payload->BeginOrderWithinContext < host_window.BeginOrderWithinContext)
        return false;

    ImGuiWindowClass* host_class = host_window.DockNodeAsHost ? &host_window.DockNodeAsHost->WindowClass : &host_window.WindowClass;
    ImGuiWindowClass* payload_class = &payload->WindowClass;
    if (host_class->ClassId != payload_class->ClassId)
    {
        if (host_class->ClassId != 0 && host_class->DockingAllowUnclassed && payload_class->ClassId == 0)
            return true;
        if (payload_class->ClassId != 0 && payload_class->DockingAllowUnclassed && host_class->ClassId == 0)
            return true;
        return false;
    }

    // Prevent docking any window created above a popup
    // Technically we should support it (e.g. in the case of a long-lived modal window that had fancy docking features),
    // by e.g. adding a 'if (!IsWindowWithinBeginStackOf(host_window, popup_window))' test.
    // But it would requires more work on our end because the dock host windows is technically created in NewFrame()
    // and our ->ParentXXX and ->RootXXX pointers inside windows are currently mislading or lacking.
    let g = GImGui; // ImGuiContext& g = *GImGui;
    for (let i: c_int = g.OpenPopupStack.Size - 1; i >= 0; i--)
        if (let mut popup_window: *mut ImGuiWindow =  g.OpenPopupStack[i].Window)
            if (IsWindowWithinBeginStackOf(payload, popup_window))   // Payload is created from within a popup begin stack.
                return false;

    return true;
}

static DockNodeIsDropAllowed: bool(host_window: *mut ImGuiWindow, root_payload: *mut ImGuiWindow)
{
    if (root_payload->DockNodeAsHost && root_payload->DockNodeAsHost->IsSplitNode()) // FIXME-DOCK: Missing filtering
        return true;

    let payload_count: c_int = root_payload->DockNodeAsHost ? root_payload->DockNodeAsHost->Windows.len() : 1;
    for (let payload_n: c_int = 0; payload_n < payload_count; payload_n++)
    {
        let mut payload: *mut ImGuiWindow =  root_payload->DockNodeAsHost ? root_payload->DockNodeAsHost->Windows[payload_n] : root_payload;
        if (DockNodeIsDropAllowedOne(payload, host_window))
            return true;
    }
    return false;
}

// window menu button == collapse button when not in a dock node.
// FIXME: This is similar to RenderWindowTitleBarContents(), may want to share code.
static c_void DockNodeCalcTabBarLayout(*const ImGuiDockNode node, ImRect* out_title_rect, ImRect* out_tab_bar_rect, out_window_menu_button_pos: *mut ImVec2, out_close_button_pos: *mut ImVec2)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiStyle& style = g.Style;

    let r: ImRect =  ImRect(node.Pos.x, node.Pos.y, node.Pos.x + node.Size.x, node.Pos.y + g.FontSize + g.Style.FramePadding.y * 2.00f32);
    if (out_title_rect) { *out_title_rect = r; }

    r.Min.x += style.WindowBorderSize;
    r.Max.x -= style.WindowBorderSize;

    let button_sz: c_float =  g.FontSize;

    let window_menu_button_pos: ImVec2 = r.Min;
    r.Min.x += style.FramePadding.x;
    r.Max.x -= style.FramePadding.x;
    if (node.HasCloseButton)
    {
        r.Max.x -= button_sz;
        if (out_close_button_pos) *out_close_button_pos = ImVec2::new(r.Max.x - style.FramePadding.x, r.Min.y);
    }
    if (node.HasWindowMenuButton && style.WindowMenuButtonPosition == ImGuiDir_Left)
    {
        r.Min.x += button_sz + style.ItemInnerSpacing.x;
    }
    else if (node.HasWindowMenuButton && style.WindowMenuButtonPosition == ImGuiDir_Right)
    {
        r.Max.x -= button_sz + style.FramePadding.x;
        window_menu_button_pos = ImVec2::new(r.Max.x, r.Min.y);
    }
    if (out_tab_bar_rect) { *out_tab_bar_rect = r; }
    if (out_window_menu_button_pos) { *out_window_menu_button_pos = window_menu_button_pos; }
}

c_void DockNodeCalcSplitRects(ImVec2& pos_old, ImVec2& size_old, ImVec2& pos_new, ImVec2& size_new, dir: ImGuiDir, size_new_desired: ImVec2)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let dock_spacing: c_float =  g.Style.ItemInnerSpacing.x;
    const ImGuiAxis axis = (dir == ImGuiDir_Left || dir == ImGuiDir_Right) ? ImGuiAxis_X : ImGuiAxis_Y;
    pos_new[axis ^ 1] = pos_old[axis ^ 1];
    size_new[axis ^ 1] = size_old[axis ^ 1];

    // Distribute size on given axis (with a desired size or equally)
    let w_avail: c_float =  size_old[axis] - dock_spacing;
    if (size_new_desired[axis] > 0.0 && size_new_desired[axis] <= w_avail * 0.5f32)
    {
        size_new[axis] = size_new_desired[axis];
        size_old[axis] = IM_FLOOR(w_avail - size_new[axis]);
    }
    else
    {
        size_new[axis] = IM_FLOOR(w_avail * 0.5f32);
        size_old[axis] = IM_FLOOR(w_avail - size_new[axis]);
    }

    // Position each node
    if (dir == ImGuiDir_Right || dir == ImGuiDir_Down)
    {
        pos_new[axis] = pos_old[axis] + size_old[axis] + dock_spacing;
    }
    else if (dir == ImGuiDir_Left || dir == ImGuiDir_Up)
    {
        pos_new[axis] = pos_old[axis];
        pos_old[axis] = pos_new[axis] + size_new[axis] + dock_spacing;
    }
}

// Retrieve the drop rectangles for a given direction or for the center + perform hit testing.
DockNodeCalcDropRectsAndTestMousePos: bool(parent: &ImRect, dir: ImGuiDir, ImRect& out_r, outer_docking: bool, test_mouse_pos: *mut ImVec2)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    let parent_smaller_axis: c_float =  ImMin(parent.GetWidth(), parent.GetHeight());
    let hs_for_central_nodes: c_float =  ImMin(g.FontSize * 1.5f32, ImMax(g.FontSize * 0.5f32, parent_smaller_axis / 8.00f32));
    let mut hs_w: c_float = 0.0; // Half-size, longer axis
    let mut hs_h: c_float = 0.0; // Half-size, smaller axis
    off: ImVec2; // Distance from edge or center
    if (outer_docking)
    {
        //hs_w = ImFloor(ImClamp(parent_smaller_axis - hs_for_central_nodes * 4.0.0, g.FontSize * 0.5f32, g.FontSize * 8.00f32));
        //hs_h = ImFloor(hs_w * 0.150f32);
        //off = ImVec2::new(ImFloor(parent.GetWidth() * 0.5f32 - GetFrameHeightWithSpacing() * 1.4f - hs_h), ImFloor(parent.GetHeight() * 0.5f32 - GetFrameHeightWithSpacing() * 1.4f - hs_h));
        hs_w = ImFloor(hs_for_central_nodes * 1.500f32);
        hs_h = ImFloor(hs_for_central_nodes * 0.800f32);
        off = ImVec2::new(ImFloor(parent.GetWidth() * 0.5f32 - hs_h), ImFloor(parent.GetHeight() * 0.5f32 - hs_h));
    }
    else
    {
        hs_w = ImFloor(hs_for_central_nodes);
        hs_h = ImFloor(hs_for_central_nodes * 0.900f32);
        off = ImVec2::new(ImFloor(hs_w * 2.400f32), ImFloor(hs_w * 2.400f32));
    }

    let c: ImVec2 = ImFloor(parent.GetCenter());
    if      (dir == ImGuiDir_None)  { out_r = ImRect(c.x - hs_w, c.y - hs_w,         c.x + hs_w, c.y + hs_w);         }
    else if (dir == ImGuiDir_Up)    { out_r = ImRect(c.x - hs_w, c.y - off.y - hs_h, c.x + hs_w, c.y - off.y + hs_h); }
    else if (dir == ImGuiDir_Down)  { out_r = ImRect(c.x - hs_w, c.y + off.y - hs_h, c.x + hs_w, c.y + off.y + hs_h); }
    else if (dir == ImGuiDir_Left)  { out_r = ImRect(c.x - off.x - hs_h, c.y - hs_w, c.x - off.x + hs_h, c.y + hs_w); }
    else if (dir == ImGuiDir_Right) { out_r = ImRect(c.x + off.x - hs_h, c.y - hs_w, c.x + off.x + hs_h, c.y + hs_w); }

    if (test_mouse_pos == null_mut())
        return false;

    let hit_r: ImRect =  out_r;
    if (!outer_docking)
    {
        // Custom hit testing for the 5-way selection, designed to reduce flickering when moving diagonally between sides
        hit_r.Expand(ImFloor(hs_w * 0.300f32));
        let mouse_delta: ImVec2 = (*test_mouse_pos - c);
        let mouse_delta_len2: c_float =  ImLengthSqr(mouse_delta);
        let r_threshold_center: c_float =  hs_w * 1.4f;
        let r_threshold_sides: c_float =  hs_w * (1.4f + 1.20f32);
        if (mouse_delta_len2 < r_threshold_center * r_threshold_center)
            return (dir == ImGuiDir_None);
        if (mouse_delta_len2 < r_threshold_sides * r_threshold_sides)
            return (dir == ImGetDirQuadrantFromDelta(mouse_delta.x, mouse_delta.y));
    }
    return hit_r.Contains(*test_mouse_pos);
}

// host_node may be NULL if the window doesn't have a DockNode already.
// FIXME-DOCK: This is misnamed since it's also doing the filtering.
static c_void DockNodePreviewDockSetup(host_window: *mut ImGuiWindow, ImGuiDockNode* host_node, payload_window: *mut ImGuiWindow, ImGuiDockNode* payload_node, ImGuiDockPreviewData* data, is_explicit_target: bool, is_outer_docking: bool)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    // There is an edge case when docking into a dockspace which only has inactive nodes.
    // In this case DockNodeTreeFindNodeByPos() will have selected a leaf node which is inactive.
    // Because the inactive leaf node doesn't have proper pos/size yet, we'll use the root node as reference.
    if (payload_node == null_mut())
        payload_node = payload_window.DockNodeAsHost;
    ImGuiDockNode* ref_node_for_rect = (host_node && !host_node.IsVisible) ? DockNodeGetRootNode(host_node) : host_node;
    if (ref_node_for_rect)
        // IM_ASSERT(ref_node_for_rect->IsVisible == true);

    // Filter, figure out where we are allowed to dock
    ImGuiDockNodeFlags src_node_flags = payload_node ? payload_node.MergedFlags : payload_window.WindowClass.DockNodeFlagsOverrideSet;
    ImGuiDockNodeFlags dst_node_flags = host_node ? host_node.MergedFlags : host_window.WindowClass.DockNodeFlagsOverrideSet;
    data.IsCenterAvailable = true;
    if (is_outer_docking)
        data.IsCenterAvailable = false;
    else if (dst_node_flags & ImGuiDockNodeFlags_NoDocking)
        data.IsCenterAvailable = false;
    else if (host_node && (dst_node_flags & ImGuiDockNodeFlags_NoDockingInCentralNode) && host_node.IsCentralNode())
        data.IsCenterAvailable = false;
    else if ((!host_node || !host_node.IsEmpty()) && payload_node && payload_node.IsSplitNode() && (payload_node.OnlyNodeWithWindows == null_mut())) // Is _visibly_ split?
        data.IsCenterAvailable = false;
    else if (dst_node_flags & ImGuiDockNodeFlags_NoDockingOverMe)
        data.IsCenterAvailable = false;
    else if ((src_node_flags & ImGuiDockNodeFlags_NoDockingOverOther) && (!host_node || !host_node.IsEmpty()))
        data.IsCenterAvailable = false;
    else if ((src_node_flags & ImGuiDockNodeFlags_NoDockingOverEmpty) && host_node && host_node.IsEmpty())
        data.IsCenterAvailable = false;

    data.IsSidesAvailable = true;
    if ((dst_node_flags & ImGuiDockNodeFlags_NoSplit) || g.IO.ConfigDockingNoSplit)
        data.IsSidesAvailable = false;
    else if (!is_outer_docking && host_node && host_node.ParentNode == null_mut() && host_node.IsCentralNode())
        data.IsSidesAvailable = false;
    else if ((dst_node_flags & ImGuiDockNodeFlags_NoDockingSplitMe) || (src_node_flags & ImGuiDockNodeFlags_NoDockingSplitOther))
        data.IsSidesAvailable = false;

    // Build a tentative future node (reuse same structure because it is practical. Shape will be readjusted when previewing a split)
    data.FutureNode.HasCloseButton = (host_node ? host_node.HasCloseButton : host_window.HasCloseButton) || (payload_window.HasCloseButton);
    data.FutureNode.HasWindowMenuButton = host_node ? true : ((host_window.Flags & ImGuiWindowFlags_NoCollapse) == 0);
    data.FutureNode.Pos = ref_node_for_rect ? ref_node_for_rect->Pos : host_window.Pos;
    data.FutureNode.Size = ref_node_for_rect ? ref_node_for_rect->Size : host_window.Size;

    // Calculate drop shapes geometry for allowed splitting directions
    // IM_ASSERT(ImGuiDir_None == -1);
    data.SplitNode = host_node;
    data.SplitDir = ImGuiDir_None;
    data.IsSplitDirExplicit = false;
    if (!host_window.Collapsed)
        for (let dir: c_int = ImGuiDir_None; dir < ImGuiDir_COUNT; dir++)
        {
            if (dir == ImGuiDir_None && !data.IsCenterAvailable)
                continue;
            if (dir != ImGuiDir_None && !data.IsSidesAvailable)
                continue;
            if (DockNodeCalcDropRectsAndTestMousePos(data.FutureNode.Rect(), (ImGuiDir)dir, data.DropRectsDraw[dir1], is_outer_docking, &g.IO.MousePos))
            {
                data.SplitDir = (ImGuiDir)dir;
                data.IsSplitDirExplicit = true;
            }
        }

    // When docking without holding Shift, we only allow and preview docking when hovering over a drop rect or over the title bar
    data.IsDropAllowed = (data.SplitDir != ImGuiDir_None) || (data.IsCenterAvailable);
    if (!is_explicit_target && !data.IsSplitDirExplicit && !g.IO.ConfigDockingWithShift)
        data.IsDropAllowed = false;

    // Calculate split area
    data.SplitRatio = 0.0;
    if (data.SplitDir != ImGuiDir_None)
    {
        split_dir: ImGuiDir = data.SplitDir;
        ImGuiAxis split_axis = (split_dir == ImGuiDir_Left || split_dir == ImGuiDir_Right) ? ImGuiAxis_X : ImGuiAxis_Y;
        pos_new: ImVec2, pos_old = data.FutureNode.Pos;
        size_new: ImVec2, size_old = data.FutureNode.Size;
        DockNodeCalcSplitRects(pos_old, size_old, pos_new, size_new, split_dir, payload_window.Size);

        // Calculate split ratio so we can pass it down the docking request
        let split_ratio: c_float =  ImSaturate(size_new[split_axis] / data.FutureNode.Size[split_axis]);
        data.FutureNode.Pos = pos_new;
        data.FutureNode.Size = size_new;
        data.SplitRatio = (split_dir == ImGuiDir_Right || split_dir == ImGuiDir_Down) ? (1.0 - split_ratio) : (split_ratio);
    }
}

static c_void DockNodePreviewDockRender(host_window: *mut ImGuiWindow, ImGuiDockNode* host_node, root_payload: *mut ImGuiWindow, *const ImGuiDockPreviewData data)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.CurrentWindow == host_window);   // Because we rely on font size to calculate tab sizes

    // With this option, we only display the preview on the target viewport, and the payload viewport is made transparent.
    // To compensate for the single layer obstructed by the payload, we'll increase the alpha of the preview nodes.
    let is_transparent_payload: bool = g.IO.ConfigDockingTransparentPayload;

    // In case the two windows involved are on different viewports, we will draw the overlay on each of them.
    let overlay_draw_lists_count: c_int = 0;
    ImDrawList* overlay_draw_lists[2];
    overlay_draw_lists[overlay_draw_lists_count++] = GetForegroundDrawList(host_window.Viewport);
    if (host_window.Viewport != root_payload->Viewport && !is_transparent_payload)
        overlay_draw_lists[overlay_draw_lists_count++] = GetForegroundDrawList(root_payload->Viewport);

    // Draw main preview rectangle
    overlay_col_main: u32 = GetColorU32(ImGuiCol_DockingPreview, is_transparent_payload ? 0.60f32 : 0.400f32);
    overlay_col_drop: u32 = GetColorU32(ImGuiCol_DockingPreview, is_transparent_payload ? 0.90f32 : 0.700f32);
    overlay_col_drop_hovered: u32 = GetColorU32(ImGuiCol_DockingPreview, is_transparent_payload ? 1.20f32 : 1.000f32);
    overlay_col_lines: u32 = GetColorU32(ImGuiCol_NavWindowingHighlight, is_transparent_payload ? 0.80f32 : 0.600f32);

    // Display area preview
    let can_preview_tabs: bool = (root_payload->DockNodeAsHost == null_mut() || root_payload->DockNodeAsHost->Windows.len() > 0);
    if (data.IsDropAllowed)
    {
        let overlay_rect: ImRect =  data.FutureNode.Rect();
        if (data.SplitDir == ImGuiDir_None && can_preview_tabs)
            overlay_rect.Min.y += GetFrameHeight();
        if (data.SplitDir != ImGuiDir_None || data.IsCenterAvailable)
            for (let overlay_n: c_int = 0; overlay_n < overlay_draw_lists_count; overlay_n++)
                overlay_draw_lists[overlay_n]->AddRectFilled(overlay_rect.Min, overlay_rect.Max, overlay_col_main, host_window.WindowRounding, CalcRoundingFlagsForRectInRect(overlay_rect, host_window.Rect(), DOCKING_SPLITTER_SIZE));
    }

    // Display tab shape/label preview unless we are splitting node (it generally makes the situation harder to read)
    if (data.IsDropAllowed && can_preview_tabs && data.SplitDir == ImGuiDir_None && data.IsCenterAvailable)
    {
        // Compute target tab bar geometry so we can locate our preview tabs
        ImRect tab_bar_rect;
        DockNodeCalcTabBarLayout(&data.FutureNode, null_mut(), &tab_bar_rect, null_mut(), null_mut());
        let tab_pos: ImVec2 = tab_bar_rect.Min;
        if (host_node && host_node.TabBar)
        {
            if (!host_node.IsHiddenTabBar() && !host_node.IsNoTabBar())
                tab_pos.x += host_node.TabBar->WidthAllTabs + g.Style.ItemInnerSpacing.x; // We don't use OffsetNewTab because when using non-persistent-order tab bar it is incremented with each Tab submission.
            else
                tab_pos.x += g.Style.ItemInnerSpacing.x + TabItemCalcSize(host_node.Windows[0]->Name, host_node.Windows[0]->HasCloseButton).x;
        }
        else if (!(host_window.Flags & ImGuiWindowFlags_DockNodeHost))
        {
            tab_pos.x += g.Style.ItemInnerSpacing.x + TabItemCalcSize(host_window.Name, host_window.HasCloseButton).x; // Account for slight offset which will be added when changing from title bar to tab bar
        }

        // Draw tab shape/label preview (payload may be a loose window or a host window carrying multiple tabbed windows)
        if (root_payload->DockNodeAsHost)
            // IM_ASSERT(root_payload->DockNodeAsHost->Windows.Size <= root_payload->DockNodeAsHost->TabBar->Tabs.Size);
        ImGuiTabBar* tab_bar_with_payload = root_payload->DockNodeAsHost ? root_payload->DockNodeAsHost->TabBar : null_mut();
        let payload_count: c_int = tab_bar_with_payload ? tab_bar_with_payload->Tabs.Size : 1;
        for (let payload_n: c_int = 0; payload_n < payload_count; payload_n++)
        {
            // DockNode's TabBar may have non-window Tabs manually appended by user
            let mut payload_window: *mut ImGuiWindow =  tab_bar_with_payload ? tab_bar_with_payload->Tabs[payload_n].Window : root_payload;
            if (tab_bar_with_payload && payload_window == null_mut())
                continue;
            if (!DockNodeIsDropAllowedOne(payload_window, host_window))
                continue;

            // Calculate the tab bounding box for each payload window
            let tab_size: ImVec2 = TabItemCalcSize(payload_window.Name, payload_window.HasCloseButton);
            let mut tab_bb: ImRect = ImRect::new(tab_pos.x, tab_pos.y, tab_pos.x + tab_size.x, tab_pos.y + tab_size.y);
            tab_pos.x += tab_size.x + g.Style.ItemInnerSpacing.x;
            overlay_col_text: u32 = GetColorU32(payload_window.DockStyle.Colors[ImGuiWindowDockStyleCol_Text]);
            overlay_col_tabs: u32 = GetColorU32(payload_window.DockStyle.Colors[ImGuiWindowDockStyleCol_TabActive]);
            PushStyleColor(ImGuiCol_Text, overlay_col_text);
            for (let overlay_n: c_int = 0; overlay_n < overlay_draw_lists_count; overlay_n++)
            {
                ImGuiTabItemFlags tab_flags = ImGuiTabItemFlags_Preview | ((payload_window.Flags & ImGuiWindowFlags_UnsavedDocument) ? ImGuiTabItemFlags_UnsavedDocument : 0);
                if (!tab_bar_rect.Contains(tab_bb))
                    overlay_draw_lists[overlay_n]->PushClipRect(tab_bar_rect.Min, tab_bar_rect.Max);
                TabItemBackground(overlay_draw_lists[overlay_n], tab_bb, tab_flags, overlay_col_tabs);
                TabItemLabelAndCloseButton(overlay_draw_lists[overlay_n], tab_bb, tab_flags, g.Style.FramePadding, payload_window.Name, 0, 0, false, null_mut(), null_mut());
                if (!tab_bar_rect.Contains(tab_bb))
                    overlay_draw_lists[overlay_n]->PopClipRect();
            }
            PopStyleColor();
        }
    }

    // Display drop boxes
    let overlay_rounding: c_float =  ImMax(3.0.0, g.Style.FrameRounding);
    for (let dir: c_int = ImGuiDir_None; dir < ImGuiDir_COUNT; dir++)
    {
        if (!data.DropRectsDraw[dir + 1].IsInverted())
        {
            let draw_r: ImRect =  data.DropRectsDraw[dir + 1];
            let draw_r_in: ImRect =  draw_r;
            draw_r_in.Expand(-2.00f32);
            overlay_col: u32 = (data.SplitDir == (ImGuiDir)dir && data.IsSplitDirExplicit) ? overlay_col_drop_hovered : overlay_col_drop;
            for (let overlay_n: c_int = 0; overlay_n < overlay_draw_lists_count; overlay_n++)
            {
                let center: ImVec2 = ImFloor(draw_r_in.GetCenter());
                overlay_draw_lists[overlay_n]->AddRectFilled(draw_r.Min, draw_r.Max, overlay_col, overlay_rounding);
                overlay_draw_lists[overlay_n]->AddRect(draw_r_in.Min, draw_r_in.Max, overlay_col_lines, overlay_rounding);
                if (dir == ImGuiDir_Left || dir == ImGuiDir_Right)
                    overlay_draw_lists[overlay_n]->AddLine(ImVec2::new(center.x, draw_r_in.Min.y), ImVec2::new(center.x, draw_r_in.Max.y), overlay_col_lines);
                if (dir == ImGuiDir_Up || dir == ImGuiDir_Down)
                    overlay_draw_lists[overlay_n]->AddLine(ImVec2::new(draw_r_in.Min.x, center.y), ImVec2::new(draw_r_in.Max.x, center.y), overlay_col_lines);
            }
        }

        // Stop after ImGuiDir_None
        if ((host_node && (host_node.MergedFlags & ImGuiDockNodeFlags_NoSplit)) || g.IO.ConfigDockingNoSplit)
            return;
    }
}

//-----------------------------------------------------------------------------
// Docking: ImGuiDockNode Tree manipulation functions
//-----------------------------------------------------------------------------
// - DockNodeTreeSplit()
// - DockNodeTreeMerge()
// - DockNodeTreeUpdatePosSize()
// - DockNodeTreeUpdateSplitterFindTouchingNode()
// - DockNodeTreeUpdateSplitter()
// - DockNodeTreeFindFallbackLeafNode()
// - DockNodeTreeFindNodeByPos()
//-----------------------------------------------------------------------------

c_void DockNodeTreeSplit(ImGuiContext* ctx, ImGuiDockNode* parent_node, ImGuiAxis split_axis, split_inheritor_child_idx: c_int,split_ratio: c_float, ImGuiDockNode* new_node)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(split_axis != ImGuiAxis_None);

    ImGuiDockNode* child_0 = (new_node && split_inheritor_child_idx != 0) ? new_node : DockContextAddNode(ctx, 0);
    child_0->ParentNode = parent_node;

    ImGuiDockNode* child_1 = (new_node && split_inheritor_child_idx != 1) ? new_node : DockContextAddNode(ctx, 0);
    child_1->ParentNode = parent_node;

    ImGuiDockNode* child_inheritor = (split_inheritor_child_idx == 0) ? child_0 : child_1;
    DockNodeMoveChildNodes(child_inheritor, parent_node);
    parent_node.ChildNodes[0] = child_0;
    parent_node.ChildNodes[1] = child_1;
    parent_node.ChildNodes[split_inheritor_child_idx]->VisibleWindow = parent_node.VisibleWindow;
    parent_node.SplitAxis = split_axis;
    parent_node.VisibleWindow= null_mut();
    parent_node.AuthorityForPos = parent_node.AuthorityForSize = ImGuiDataAuthority_DockNode;

    let size_avail: c_float =  (parent_node.Size[split_axis] - DOCKING_SPLITTER_SIZE);
    size_avail = ImMax(size_avail, g.Style.WindowMinSize[split_axis] * 2.00f32);
    // IM_ASSERT(size_avail > 0.0); // If you created a node manually with DockBuilderAddNode(), you need to also call DockBuilderSetNodeSize() before splitting.
    child_0->SizeRef = child_1->SizeRef = parent_node.Size;
    child_0->SizeRef[split_axis] = ImFloor(size_avail * split_ratio);
    child_1->SizeRef[split_axis] = ImFloor(size_avail - child_0->SizeRef[split_axis]);

    DockNodeMoveWindows(parent_node.ChildNodes[split_inheritor_child_idx], parent_node);
    DockSettingsRenameNodeReferences(parent_node.ID, parent_node.ChildNodes[split_inheritor_child_idx]->ID);
    DockNodeUpdateHasCentralNodeChild(DockNodeGetRootNode(parent_node));
    DockNodeTreeUpdatePosSize(parent_node, parent_node.Pos, parent_node.Size);

    // Flags transfer (e.g. this is where we transfer the ImGuiDockNodeFlags_CentralNode property)
    child_0->SharedFlags = parent_node.SharedFlags & ImGuiDockNodeFlags_SharedFlagsInheritMask_;
    child_1->SharedFlags = parent_node.SharedFlags & ImGuiDockNodeFlags_SharedFlagsInheritMask_;
    child_inheritor->LocalFlags = parent_node.LocalFlags & ImGuiDockNodeFlags_LocalFlagsTransferMask_;
    parent_node.LocalFlags &= !ImGuiDockNodeFlags_LocalFlagsTransferMask_;
    child_0->UpdateMergedFlags();
    child_1->UpdateMergedFlags();
    parent_node.UpdateMergedFlags();
    if (child_inheritor->IsCentralNode())
        DockNodeGetRootNode(parent_node)->CentralNode = child_inheritor;
}

c_void DockNodeTreeMerge(ImGuiContext* ctx, ImGuiDockNode* parent_node, ImGuiDockNode* merge_lead_child)
{
    // When called from DockContextProcessUndockNode() it is possible that one of the child is NULL.
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiDockNode* child_0 = parent_node.ChildNodes[0];
    ImGuiDockNode* child_1 = parent_node.ChildNodes[1];
    // IM_ASSERT(child_0 || child_1);
    // IM_ASSERT(merge_lead_child == child_0 || merge_lead_child == child_1);
    if ((child_0 && child_0->Windows.len() > 0) || (child_1 && child_1->Windows.len() > 0))
    {
        // IM_ASSERT(parent_node->TabBar == NULL);
        // IM_ASSERT(parent_node->Windows.Size == 0);
    }
    IMGUI_DEBUG_LOG_DOCKING("[docking] DockNodeTreeMerge: 0x%08X + 0x%08X back into parent 0x%08X\n", child_0 ? child_0->ID : 0, child_1 ? child_1->ID : 0, parent_node.ID);

    let backup_last_explicit_size: ImVec2 = parent_node.SizeRef;
    DockNodeMoveChildNodes(parent_node, merge_lead_child);
    if (child_0)
    {
        DockNodeMoveWindows(parent_node, child_0); // Generally only 1 of the 2 child node will have windows
        DockSettingsRenameNodeReferences(child_0->ID, parent_node.ID);
    }
    if (child_1)
    {
        DockNodeMoveWindows(parent_node, child_1);
        DockSettingsRenameNodeReferences(child_1->ID, parent_node.ID);
    }
    DockNodeApplyPosSizeToWindows(parent_node);
    parent_node.AuthorityForPos = parent_node.AuthorityForSize = parent_node.AuthorityForViewport = ImGuiDataAuthority_Auto;
    parent_node.VisibleWindow = merge_lead_child->VisibleWindow;
    parent_node.SizeRef = backup_last_explicit_size;

    // Flags transfer
    parent_node.LocalFlags &= !ImGuiDockNodeFlags_LocalFlagsTransferMask_; // Preserve Dockspace flag
    parent_node.LocalFlags |= (child_0 ? child_0->LocalFlags : 0) & ImGuiDockNodeFlags_LocalFlagsTransferMask_;
    parent_node.LocalFlags |= (child_1 ? child_1->LocalFlags : 0) & ImGuiDockNodeFlags_LocalFlagsTransferMask_;
    parent_node.LocalFlagsInWindows = (child_0 ? child_0->LocalFlagsInWindows : 0) | (child_1 ? child_1->LocalFlagsInWindows : 0); // FIXME: Would be more consistent to update from actual windows
    parent_node.UpdateMergedFlags();

    if (child_0)
    {
        ctx->DockContext.Nodes.SetVoidPtr(child_0->ID, null_mut());
        IM_DELETE(child_0);
    }
    if (child_1)
    {
        ctx->DockContext.Nodes.SetVoidPtr(child_1->ID, null_mut());
        IM_DELETE(child_1);
    }
}

// Update Pos/Size for a node hierarchy (don't affect child Windows yet)
// (Depth-first, Pre-Order)
c_void DockNodeTreeUpdatePosSize(ImGuiDockNode* node, pos: ImVec2, size: ImVec2, ImGuiDockNode* only_write_to_single_node)
{
    // During the regular dock node update we write to all nodes.
    // 'only_write_to_single_node' is only set when turning a node visible mid-frame and we need its size right-away.
    let write_to_node: bool = only_write_to_single_node == null_mut() || only_write_to_single_node == node;
    if (write_to_node)
    {
        node.Pos = pos;
        node.Size = size;
    }

    if (node.IsLeafNode())
        return;

    ImGuiDockNode* child_0 = node.ChildNodes[0];
    ImGuiDockNode* child_1 = node.ChildNodes[1];
    let child_0_pos: ImVec2 = pos, child_1_pos = pos;
    let child_0_size: ImVec2 = size, child_1_size = size;

    let child_0_is_toward_single_node: bool = (only_write_to_single_node != null_mut() && DockNodeIsInHierarchyOf(only_write_to_single_node, child_0));
    let child_1_is_toward_single_node: bool = (only_write_to_single_node != null_mut() && DockNodeIsInHierarchyOf(only_write_to_single_node, child_1));
    let child_0_is_or_will_be_visible: bool = child_0->IsVisible || child_0_is_toward_single_node;
    let child_1_is_or_will_be_visible: bool = child_1->IsVisible || child_1_is_toward_single_node;

    if (child_0_is_or_will_be_visible && child_1_is_or_will_be_visible)
    {
        let g = GImGui; // ImGuiContext& g = *GImGui;
        let spacing: c_float =  DOCKING_SPLITTER_SIZE;
        const ImGuiAxis axis = (ImGuiAxis)node.SplitAxis;
        let size_avail: c_float =  ImMax(size[axis] - spacing, 0.0);

        // Size allocation policy
        // 1) The first 0..WindowMinSize[axis]*2 are allocated evenly to both windows.
        let size_min_each: c_float =  ImFloor(ImMin(size_avail, g.Style.WindowMinSize[axis] * 2.00f32) * 0.5f32);

        // FIXME: Blocks 2) and 3) are essentially doing nearly the same thing.
        // Difference are: write-back to SizeRef; application of a minimum size; rounding before ImFloor()
        // Clarify and rework differences between Size & SizeRef and purpose of WantLockSizeOnce

        // 2) Process locked absolute size (during a splitter resize we preserve the child of nodes not touching the splitter edge)
        if (child_0->WantLockSizeOnce && !child_1->WantLockSizeOnce)
        {
            child_0_size[axis] = child_0->SizeRef[axis] = ImMin(size_avail - 1.0, child_0->Size[axis]);
            child_1_size[axis] = child_1->SizeRef[axis] = (size_avail - child_0_size[axis]);
            // IM_ASSERT(child_0->SizeRef[axis] > 0.0 && child_1->SizeRef[axis] > 0.0);
        }
        else if (child_1->WantLockSizeOnce && !child_0->WantLockSizeOnce)
        {
            child_1_size[axis] = child_1->SizeRef[axis] = ImMin(size_avail - 1.0, child_1->Size[axis]);
            child_0_size[axis] = child_0->SizeRef[axis] = (size_avail - child_1_size[axis]);
            // IM_ASSERT(child_0->SizeRef[axis] > 0.0 && child_1->SizeRef[axis] > 0.0);
        }
        else if (child_0->WantLockSizeOnce && child_1->WantLockSizeOnce)
        {
            // FIXME-DOCK: We cannot honor the requested size, so apply ratio.
            // Currently this path will only be taken if code programmatically sets WantLockSizeOnce
            let split_ratio: c_float =  child_0_size[axis] / (child_0_size[axis] + child_1_size[axis]);
            child_0_size[axis] = child_0->SizeRef[axis] = ImFloor(size_avail * split_ratio);
            child_1_size[axis] = child_1->SizeRef[axis] = (size_avail - child_0_size[axis]);
            // IM_ASSERT(child_0->SizeRef[axis] > 0.0 && child_1->SizeRef[axis] > 0.0);
        }

        // 3) If one window is the central node (~ use remaining space, should be made explicit!), use explicit size from the other, and remainder for the central node
        else if (child_0->SizeRef[axis] != 0.0 && child_1->HasCentralNodeChild)
        {
            child_0_size[axis] = ImMin(size_avail - size_min_each, child_0->SizeRef[axis]);
            child_1_size[axis] = (size_avail - child_0_size[axis]);
        }
        else if (child_1->SizeRef[axis] != 0.0 && child_0->HasCentralNodeChild)
        {
            child_1_size[axis] = ImMin(size_avail - size_min_each, child_1->SizeRef[axis]);
            child_0_size[axis] = (size_avail - child_1_size[axis]);
        }
        else
        {
            // 4) Otherwise distribute according to the relative ratio of each SizeRef value
            let split_ratio: c_float =  child_0->SizeRef[axis] / (child_0->SizeRef[axis] + child_1->SizeRef[axis]);
            child_0_size[axis] = ImMax(size_min_each, ImFloor(size_avail * split_ratio + 0.5f32));
            child_1_size[axis] = (size_avail - child_0_size[axis]);
        }

        child_1_pos[axis] += spacing + child_0_size[axis];
    }

    if (only_write_to_single_node == null_mut())
        child_0->WantLockSizeOnce = child_1->WantLockSizeOnce = false;

    let child_0_recurse: bool = only_write_to_single_node ? child_0_is_toward_single_node : child_0->IsVisible;
    let child_1_recurse: bool = only_write_to_single_node ? child_1_is_toward_single_node : child_1->IsVisible;
    if (child_0_recurse)
        DockNodeTreeUpdatePosSize(child_0, child_0_pos, child_0_size);
    if (child_1_recurse)
        DockNodeTreeUpdatePosSize(child_1, child_1_pos, child_1_size);
}

static c_void DockNodeTreeUpdateSplitterFindTouchingNode(ImGuiDockNode* node, ImGuiAxis axis, side: c_int, Vec<ImGuiDockNode*>* touching_nodes)
{
    if (node.IsLeafNode())
    {
        touching_nodes.push(node);
        return;
    }
    if (node.ChildNodes[0]->IsVisible)
        if (node.SplitAxis != axis || side == 0 || !node.ChildNodes[1]->IsVisible)
            DockNodeTreeUpdateSplitterFindTouchingNode(node.ChildNodes[0], axis, side, touching_nodes);
    if (node.ChildNodes[1]->IsVisible)
        if (node.SplitAxis != axis || side == 1 || !node.ChildNodes[0]->IsVisible)
            DockNodeTreeUpdateSplitterFindTouchingNode(node.ChildNodes[1], axis, side, touching_nodes);
}

// (Depth-First, Pre-Order)
c_void DockNodeTreeUpdateSplitter(ImGuiDockNode* node)
{
    if (node.IsLeafNode())
        return;

    let g = GImGui; // ImGuiContext& g = *GImGui;

    ImGuiDockNode* child_0 = node.ChildNodes[0];
    ImGuiDockNode* child_1 = node.ChildNodes[1];
    if (child_0->IsVisible && child_1->IsVisible)
    {
        // Bounding box of the splitter cover the space between both nodes (w = Spacing, h = Size[xy^1] for when splitting horizontally)
        const ImGuiAxis axis = (ImGuiAxis)node.SplitAxis;
        // IM_ASSERT(axis != ImGuiAxis_None);
        ImRect bb;
        bb.Min = child_0->Pos;
        bb.Max = child_1->Pos;
        bb.Min[axis] += child_0->Size[axis];
        bb.Max[axis ^ 1] += child_1->Size[axis ^ 1];
        //if (g.IO.KeyCtrl) GetForegroundDrawList(g.Currentwindow.Viewport)->AddRect(bb.Min, bb.Max, IM_COL32(255,0,255,255));

        const ImGuiDockNodeFlags merged_flags = child_0->MergedFlags | child_1->MergedFlags; // Merged flags for BOTH childs
        const ImGuiDockNodeFlags no_resize_axis_flag = (axis == ImGuiAxis_X) ? ImGuiDockNodeFlags_NoResizeX : ImGuiDockNodeFlags_NoResizeY;
        if ((merged_flags & ImGuiDockNodeFlags_NoResize) || (merged_flags & no_resize_axis_flag))
        {
            let mut window = g.CurrentWindow;
            window.DrawList.AddRectFilled(bb.Min, bb.Max, GetColorU32(ImGuiCol_Separator), g.Style.FrameRounding);
        }
        else
        {
            //bb.Min[axis] += 1; // Display a little inward so highlight doesn't connect with nearby tabs on the neighbor node.
            //bb.Max[axis] -= 1;
            PushID(node.ID);

            // Find resizing limits by gathering list of nodes that are touching the splitter line.
            Vec<ImGuiDockNode*> touching_nodes[2];
            let min_size: c_float =  g.Style.WindowMinSize[axis];resize_limits: c_float[2];
            resize_limits[0] = node.ChildNodes[0]->Pos[axis] + min_size;
            resize_limits[1] = node.ChildNodes[1]->Pos[axis] + node.ChildNodes[1]->Size[axis] - min_size;

            let mut splitter_id: ImGuiID =  GetID("##Splitter");
            if (g.ActiveId == splitter_id) // Only process when splitter is active
            {
                DockNodeTreeUpdateSplitterFindTouchingNode(child_0, axis, 1, &touching_nodes[0]);
                DockNodeTreeUpdateSplitterFindTouchingNode(child_1, axis, 0, &touching_nodes[1]);
                for (let touching_node_n: c_int = 0; touching_node_n < touching_nodes[0].Size; touching_node_n++)
                    resize_limits[0] = ImMax(resize_limits[0], touching_nodes[0][touching_node_n]->Rect().Min[axis] + min_size);
                for (let touching_node_n: c_int = 0; touching_node_n < touching_nodes[1].Size; touching_node_n++)
                    resize_limits[1] = ImMin(resize_limits[1], touching_nodes[1][touching_node_n]->Rect().Max[axis] - min_size);

                // [DEBUG] Render touching nodes & limits
                /*
                draw_list: *mut ImDrawList = node->HostWindow ? GetForegroundDrawList(node->HostWindow) : GetForegroundDrawList(GetMainViewport());
                for (int n = 0; n < 2; n++)
                {
                    for (int touching_node_n = 0; touching_node_n < touching_nodes[n].Size; touching_node_n++)
                        draw_list.AddRect(touching_nodes[n][touching_node_n]->Pos, touching_nodes[n][touching_node_n]->Pos + touching_nodes[n][touching_node_n]->Size, IM_COL32(0, 255, 0, 255));
                    if (axis == ImGuiAxis_X)
                        draw_list.AddLine(ImVec2::new(resize_limits[n], node->ChildNodes[n]->Pos.y), ImVec2::new(resize_limits[n], node->ChildNodes[n]->Pos.y + node->ChildNodes[n]->Size.y), IM_COL32(255, 0, 255, 255), 3.00f32);
                    else
                        draw_list.AddLine(ImVec2::new(node->ChildNodes[n]->Pos.x, resize_limits[n]), ImVec2::new(node->ChildNodes[n]->Pos.x + node->ChildNodes[n]->Size.x, resize_limits[n]), IM_COL32(255, 0, 255, 255), 3.00f32);
                }
                */
            }

            // Use a short delay before highlighting the splitter (and changing the mouse cursor) in order for regular mouse movement to not highlight many splitters
            let cur_size_0: c_float =  child_0->Size[axis];
            let cur_size_1: c_float =  child_1->Size[axis];
            let min_size_0: c_float =  resize_limits[0] - child_0->Pos[axis];
            let min_size_1: c_float =  child_1->Pos[axis] + child_1->Size[axis] - resize_limits[1];
            bg_col: u32 = GetColorU32(ImGuiCol_WindowBg);
            if (SplitterBehavior(bb, GetID("##Splitter"), axis, &cur_size_0, &cur_size_1, min_size_0, min_size_1, WINDOWS_HOVER_PADDING, WINDOWS_RESIZE_FROM_EDGES_FEEDBACK_TIMER, bg_col))
            {
                if (touching_nodes[0].Size > 0 && touching_nodes[1].Size > 0)
                {
                    child_0->Size[axis] = child_0->SizeRef[axis] = cur_size_0;
                    child_1->Pos[axis] -= cur_size_1 - child_1->Size[axis];
                    child_1->Size[axis] = child_1->SizeRef[axis] = cur_size_1;

                    // Lock the size of every node that is a sibling of the node we are touching
                    // This might be less desirable if we can merge sibling of a same axis into the same parental level.
                    for (let side_n: c_int = 0; side_n < 2; side_n++)
                        for (let touching_node_n: c_int = 0; touching_node_n < touching_nodes[side_n].Size; touching_node_n++)
                        {
                            ImGuiDockNode* touching_node = touching_nodes[side_n][touching_node_n];
                            //draw_list: *mut ImDrawList = node->HostWindow ? GetForegroundDrawList(node->HostWindow) : GetForegroundDrawList(GetMainViewport());
                            //draw_list.AddRect(touching_node->Pos, touching_node->Pos + touching_node->Size, IM_COL32(255, 128, 0, 255));
                            while (touching_node.ParentNode != node)
                            {
                                if (touching_node.ParentNode.SplitAxis == axis)
                                {
                                    // Mark other node so its size will be preserved during the upcoming call to DockNodeTreeUpdatePosSize().
                                    ImGuiDockNode* node_to_preserve = touching_node.ParentNode.ChildNodes[side_n];
                                    node_to_preserve.WantLockSizeOnce = true;
                                    //draw_list.AddRect(touching_node->Pos, touching_node->Rect().Max, IM_COL32(255, 0, 0, 255));
                                    //draw_list.AddRectFilled(node_to_preserve->Pos, node_to_preserve->Rect().Max, IM_COL32(0, 255, 0, 100));
                                }
                                touching_node = touching_node.ParentNode;
                            }
                        }

                    DockNodeTreeUpdatePosSize(child_0, child_0->Pos, child_0->Size);
                    DockNodeTreeUpdatePosSize(child_1, child_1->Pos, child_1->Size);
                    MarkIniSettingsDirty();
                }
            }
            PopID();
        }
    }

    if (child_0->IsVisible)
        DockNodeTreeUpdateSplitter(child_0);
    if (child_1->IsVisible)
        DockNodeTreeUpdateSplitter(child_1);
}

ImGuiDockNode* DockNodeTreeFindFallbackLeafNode(ImGuiDockNode* node)
{
    if (node.IsLeafNode())
        return node;
    if (ImGuiDockNode* leaf_node = DockNodeTreeFindFallbackLeafNode(node.ChildNodes[0]))
        return leaf_node;
    if (ImGuiDockNode* leaf_node = DockNodeTreeFindFallbackLeafNode(node.ChildNodes[1]))
        return leaf_node;
    return null_mut();
}

ImGuiDockNode* DockNodeTreeFindVisibleNodeByPos(ImGuiDockNode* node, pos: ImVec2)
{
    if (!node.IsVisible)
        return null_mut();

    let dock_spacing: c_float =  0.0;// g.Style.ItemInnerSpacing.x; // FIXME: Relation to DOCKING_SPLITTER_SIZE?
    let mut r: ImRect = ImRect::new(node.Pos, node.Pos + node.Size);
    r.Expand(dock_spacing * 0.5f32);
    let mut inside: bool =  r.Contains(pos);
    if (!inside)
        return null_mut();

    if (node.IsLeafNode())
        return node;
    if (ImGuiDockNode* hovered_node = DockNodeTreeFindVisibleNodeByPos(node.ChildNodes[0], pos))
        return hovered_node;
    if (ImGuiDockNode* hovered_node = DockNodeTreeFindVisibleNodeByPos(node.ChildNodes[1], pos))
        return hovered_node;

    return null_mut();
}

//-----------------------------------------------------------------------------
// Docking: Public Functions (SetWindowDock, DockSpace, DockSpaceOverViewport)
//-----------------------------------------------------------------------------
// - SetWindowDock() [Internal]
// - DockSpace()
// - DockSpaceOverViewport()
//-----------------------------------------------------------------------------

// [Internal] Called via SetNextWindowDockID()
c_void SetWindowDock(window: *mut ImGuiWindow, ImGuiID dock_id, cond: ImGuiCond)
{
    // Test condition (NB: bit 0 is always true) and clear flags for next time
    if (cond && (window.SetWindowDockAllowFlags & cond) == 0)
        return;
    window.SetWindowDockAllowFlags &= ~(ImGuiCond_Once | ImGuiCond_FirstUseEver | ImGuiCond_Appearing);

    if (window.DockId == dock_id)
        return;

    // If the user attempt to set a dock id that is a split node, we'll dig within to find a suitable docking spot
    ImGuiContext* ctx = GImGui;
    if (ImGuiDockNode* new_node = DockContextFindNodeByID(ctx, dock_id))
        if (new_node.IsSplitNode())
        {
            // Policy: Find central node or latest focused node. We first move back to our root node.
            new_node = DockNodeGetRootNode(new_node);
            if (new_node.CentralNode)
            {
                // IM_ASSERT(new_node->CentralNode->IsCentralNode());
                dock_id = new_node.CentralNode.ID;
            }
            else
            {
                dock_id = new_node.LastFocusedNodeId;
            }
        }

    if (window.DockId == dock_id)
        return;

    if (window.DockNode)
        DockNodeRemoveWindow(window.DockNode, window, 0);
    window.DockId = dock_id;
}

// Create an explicit dockspace node within an existing window. Also expose dock node flags and creates a CentralNode by default.
// The Central Node is always displayed even when empty and shrink/extend according to the requested size of its neighbors.
// DockSpace() needs to be submitted _before_ any window they can host. If you use a dockspace, submit it early in your app.
ImGuiID DockSpace(id: ImGuiID, size_arg: &ImVec2, ImGuiDockNodeFlags flags, *const ImGuiWindowClass window_class)
{
    ImGuiContext* ctx = GImGui;
    ImGuiContext& g = *ctx;
    let mut window: *mut ImGuiWindow =  GetCurrentWindow();
    if (!(g.IO.ConfigFlags & ImGuiConfigFlags_DockingEnable))
        return 0;

    // Early out if parent window is hidden/collapsed
    // This is faster but also DockNodeUpdateTabBar() relies on TabBarLayout() running (which won't if SkipItems=true) to set NextSelectedTabId = 0). See #2960.
    // If for whichever reason this is causing problem we would need to ensure that DockNodeUpdateTabBar() ends up clearing NextSelectedTabId even if SkipItems=true.
    if (window.SkipItems)
        flags |= ImGuiDockNodeFlags_KeepAliveOnly;

    // IM_ASSERT((flags & ImGuiDockNodeFlags_DockSpace) == 0);
    // IM_ASSERT(id != 0);
    ImGuiDockNode* node = DockContextFindNodeByID(ctx, id);
    if (!node)
    {
        IMGUI_DEBUG_LOG_DOCKING("[docking] DockSpace: dockspace node 0x%08X created\n", id);
        node = DockContextAddNode(ctx, id);
        node.SetLocalFlags(ImGuiDockNodeFlags_CentralNode);
    }
    if (window_class && window_class->ClassId != node.WindowClass.ClassId)
        IMGUI_DEBUG_LOG_DOCKING("[docking] DockSpace: dockspace node 0x%08X: setup WindowClass 0x%08X -> 0x%08X\n", id, node.WindowClass.ClassId, window_class->ClassId);
    node.SharedFlags = flags;
    node.WindowClass = window_class ? *window_class : ImGuiWindowClass();

    // When a DockSpace transitioned form implicit to explicit this may be called a second time
    // It is possible that the node has already been claimed by a docked window which appeared before the DockSpace() node, so we overwrite IsDockSpace again.
    if (node.LastFrameActive == g.FrameCount && !(flags & ImGuiDockNodeFlags_KeepAliveOnly))
    {
        // IM_ASSERT(node->IsDockSpace() == false && "Cannot call DockSpace() twice a frame with the same ID");
        node.SetLocalFlags(node.LocalFlags | ImGuiDockNodeFlags_DockSpace);
        return id;
    }
    node.SetLocalFlags(node.LocalFlags | ImGuiDockNodeFlags_DockSpace);

    // Keep alive mode, this is allow windows docked into this node so stay docked even if they are not visible
    if (flags & ImGuiDockNodeFlags_KeepAliveOnly)
    {
        node.LastFrameAlive = g.FrameCount;
        return id;
    }

    let content_avail: ImVec2 = GetContentRegionAvail();
    let size: ImVec2 = ImFloor(size_arg);
    if (size.x <= 0.0)
        size.x = ImMax(content_avail.x + size.x, 4.00f32); // Arbitrary minimum child size (0.0 causing too much issues)
    if (size.y <= 0.0)
        size.y = ImMax(content_avail.y + size.y, 4.00f32);
    // IM_ASSERT(size.x > 0.0 && size.y > 0.0);

    node.Pos = window.DC.CursorPos;
    node.Size = node.SizeRef = size;
    SetNextWindowPos(node.Pos);
    SetNextWindowSize(node.Size);
    g.NextWindowData.PosUndock = false;

    // FIXME-DOCK: Why do we need a child window to host a dockspace, could we host it in the existing window?
    // FIXME-DOCK: What is the reason for not simply calling BeginChild()? (OK to have a reason but should be commented)
    window_flags: ImGuiWindowFlags = ImGuiWindowFlags_ChildWindow | ImGuiWindowFlags_DockNodeHost;
    window_flags |= ImGuiWindowFlags_NoSavedSettings | ImGuiWindowFlags_NoResize | ImGuiWindowFlags_NoCollapse | ImGuiWindowFlags_NoTitleBar;
    window_flags |= ImGuiWindowFlags_NoScrollbar | ImGuiWindowFlags_NoScrollWithMouse;
    window_flags |= ImGuiWindowFlags_NoBackground;

    title: [c_char;256];
    ImFormatString(title, title.len(), "%s/DockSpace_%08X", window.Name, id);

    PushStyleVar(ImGuiStyleVar_ChildBorderSize, 0.0);
    Begin(title, null_mut(), window_flags);
    PopStyleVar();

    let mut host_window: *mut ImGuiWindow =  g.CurrentWindow;
    DockNodeSetupHostWindow(node, host_window);
    host_window.ChildId = window.GetID(title);
    node.OnlyNodeWithWindows= null_mut();

    // IM_ASSERT(node->IsRootNode());

    // We need to handle the rare case were a central node is missing.
    // This can happen if the node was first created manually with DockBuilderAddNode() but _without_ the ImGuiDockNodeFlags_Dockspace.
    // Doing it correctly would set the _CentralNode flags, which would then propagate according to subsequent split.
    // It would also be ambiguous to attempt to assign a central node while there are split nodes, so we wait until there's a single node remaining.
    // The specific sub-property of _CentralNode we are interested in recovering here is the "Don't delete when empty" property,
    // as it doesn't make sense for an empty dockspace to not have this property.
    if (node.IsLeafNode() && !node.IsCentralNode())
        node.SetLocalFlags(node.LocalFlags | ImGuiDockNodeFlags_CentralNode);

    // Update the node
    DockNodeUpdate(node);

    End();
    ItemSize(size);
    return id;
}

// Tips: Use with ImGuiDockNodeFlags_PassthruCentralNode!
// The limitation with this call is that your window won't have a menu bar.
// Even though we could pass window flags, it would also require the user to be able to call BeginMenuBar() somehow meaning we can't Begin/End in a single function.
// But you can also use BeginMainMenuBar(). If you really want a menu bar inside the same window as the one hosting the dockspace, you will need to copy this code somewhere and tweak it.
ImGuiID DockSpaceOverViewport(*const ImGuiViewport viewport, ImGuiDockNodeFlags dockspace_flags, *const ImGuiWindowClass window_class)
{
    if (viewport == null_mut())
        viewport = GetMainViewport();

    SetNextWindowPos(viewport.WorkPos);
    SetNextWindowSize(viewport.WorkSize);
    SetNextWindowViewport(viewport.ID);

    host_window_flags: ImGuiWindowFlags = 0;
    host_window_flags |= ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoCollapse | ImGuiWindowFlags_NoResize | ImGuiWindowFlags_NoMove | ImGuiWindowFlags_NoDocking;
    host_window_flags |= ImGuiWindowFlags_NoBringToFrontOnFocus | ImGuiWindowFlags_NoNavFocus;
    if (dockspace_flags & ImGuiDockNodeFlags_PassthruCentralNode)
        host_window_flags |= ImGuiWindowFlags_NoBackground;

    label: [c_char;32];
    ImFormatString(label, label.len(), "DockSpaceViewport_%08X", viewport.ID);

    PushStyleVar(ImGuiStyleVar_WindowRounding, 0.0);
    PushStyleVar(ImGuiStyleVar_WindowBorderSize, 0.0);
    PushStyleVar(ImGuiStyleVar_WindowPadding, ImVec2::new(0.0, 0.0));
    Begin(label, null_mut(), host_window_flags);
    PopStyleVar(3);

    let mut dockspace_id: ImGuiID =  GetID("DockSpace");
    DockSpace(dockspace_id, ImVec2::new(0.0, 0.0), dockspace_flags, window_class);
    End();

    return dockspace_id;
}

//-----------------------------------------------------------------------------
// Docking: Builder Functions
//-----------------------------------------------------------------------------
// Very early end-user API to manipulate dock nodes.
// Only available in imgui_internal.h. Expect this API to change/break!
// It is expected that those functions are all called _before_ the dockspace node submission.
//-----------------------------------------------------------------------------
// - DockBuilderDockWindow()
// - DockBuilderGetNode()
// - DockBuilderSetNodePos()
// - DockBuilderSetNodeSize()
// - DockBuilderAddNode()
// - DockBuilderRemoveNode()
// - DockBuilderRemoveNodeChildNodes()
// - DockBuilderRemoveNodeDockedWindows()
// - DockBuilderSplitNode()
// - DockBuilderCopyNodeRec()
// - DockBuilderCopyNode()
// - DockBuilderCopyWindowSettings()
// - DockBuilderCopyDockSpace()
// - DockBuilderFinish()
//-----------------------------------------------------------------------------

c_void DockBuilderDockWindow(window_name: *const c_char, ImGuiID node_id)
{
    // We don't preserve relative order of multiple docked windows (by clearing DockOrder back to -1)
    let mut window_id: ImGuiID =  ImHashStr(window_name);
    if (let mut window: *mut ImGuiWindow =  FindWindowByID(window_id))
    {
        // Apply to created window
        SetWindowDock(window, node_id, ImGuiCond_Always);
        window.DockOrder = -1;
    }
    else
    {
        // Apply to settings
        settings: *mut ImGuiWindowSettings = FindWindowSettings(window_id);
        if (settings == null_mut())
            settings = CreateNewWindowSettings(window_name);
        settings->DockId = node_id;
        settings->DockOrder = -1;
    }
}

ImGuiDockNode* DockBuilderGetNode(ImGuiID node_id)
{
    ImGuiContext* ctx = GImGui;
    return DockContextFindNodeByID(ctx, node_id);
}

c_void DockBuilderSetNodePos(ImGuiID node_id, pos: ImVec2)
{
    ImGuiContext* ctx = GImGui;
    ImGuiDockNode* node = DockContextFindNodeByID(ctx, node_id);
    if (node == null_mut())
        return;
    node.Pos = pos;
    node.AuthorityForPos = ImGuiDataAuthority_DockNode;
}

c_void DockBuilderSetNodeSize(ImGuiID node_id, size: ImVec2)
{
    ImGuiContext* ctx = GImGui;
    ImGuiDockNode* node = DockContextFindNodeByID(ctx, node_id);
    if (node == null_mut())
        return;
    // IM_ASSERT(size.x > 0.0 && size.y > 0.0);
    node.Size = node.SizeRef = size;
    node.AuthorityForSize = ImGuiDataAuthority_DockNode;
}

// Make sure to use the ImGuiDockNodeFlags_DockSpace flag to create a dockspace node! Otherwise this will create a floating node!
// - Floating node: you can then call DockBuilderSetNodePos()/DockBuilderSetNodeSize() to position and size the floating node.
// - Dockspace node: calling DockBuilderSetNodePos() is unnecessary.
// - If you intend to split a node immediately after creation using DockBuilderSplitNode(), make sure to call DockBuilderSetNodeSize() beforehand!
//   For various reason, the splitting code currently needs a base size otherwise space may not be allocated as precisely as you would expect.
// - Use (id == 0) to let the system allocate a node identifier.
// - Existing node with a same id will be removed.
ImGuiID DockBuilderAddNode(id: ImGuiID, ImGuiDockNodeFlags flags)
{
    ImGuiContext* ctx = GImGui;

    if (id != 0)
        DockBuilderRemoveNode(id);

    ImGuiDockNode* node= null_mut();
    if (flags & ImGuiDockNodeFlags_DockSpace)
    {
        DockSpace(id, ImVec2::new(0, 0), (flags & !ImGuiDockNodeFlags_DockSpace) | ImGuiDockNodeFlags_KeepAliveOnly);
        node = DockContextFindNodeByID(ctx, id);
    }
    else
    {
        node = DockContextAddNode(ctx, id);
        node.SetLocalFlags(flags);
    }
    node.LastFrameAlive = ctx->FrameCount;   // Set this otherwise BeginDocked will undock during the same frame.
    return node.ID;
}

c_void DockBuilderRemoveNode(ImGuiID node_id)
{
    ImGuiContext* ctx = GImGui;
    ImGuiDockNode* node = DockContextFindNodeByID(ctx, node_id);
    if (node == null_mut())
        return;
    DockBuilderRemoveNodeDockedWindows(node_id, true);
    DockBuilderRemoveNodeChildNodes(node_id);
    // Node may have moved or deleted if e.g. any merge happened
    node = DockContextFindNodeByID(ctx, node_id);
    if (node == null_mut())
        return;
    if (node.IsCentralNode() && node.ParentNode)
        node.ParentNode.SetLocalFlags(node.ParentNode.LocalFlags | ImGuiDockNodeFlags_CentralNode);
    DockContextRemoveNode(ctx, node, true);
}

// root_id = 0 to remove all, root_id != 0 to remove child of given node.
c_void DockBuilderRemoveNodeChildNodes(ImGuiID root_id)
{
    ImGuiContext* ctx = GImGui;
    ImGuiDockContext* dc  = &ctx->DockContext;

    ImGuiDockNode* root_node = root_id ? DockContextFindNodeByID(ctx, root_id) : null_mut();
    if (root_id && root_node == null_mut())
        return;
    let mut has_central_node: bool =  false;

    ImGuiDataAuthority backup_root_node_authority_for_pos = root_node ? root_node.AuthorityForPos : ImGuiDataAuthority_Auto;
    ImGuiDataAuthority backup_root_node_authority_for_size = root_node ? root_node.AuthorityForSize : ImGuiDataAuthority_Auto;

    // Process active windows
    Vec<ImGuiDockNode*> nodes_to_remove;
    for (let n: c_int = 0; n < dc->Nodes.Data.Size; n++)
        if (ImGuiDockNode* node = (ImGuiDockNode*)dc->Nodes.Data[n].val_p)
        {
            let mut want_removal: bool =  (root_id == 0) || (node.ID != root_id && DockNodeGetRootNode(node)->ID == root_id);
            if (want_removal)
            {
                if (node.IsCentralNode())
                    has_central_node = true;
                if (root_id != 0)
                    DockContextQueueNotifyRemovedNode(ctx, node);
                if (root_node)
                {
                    DockNodeMoveWindows(root_node, node);
                    DockSettingsRenameNodeReferences(node.ID, root_node.ID);
                }
                nodes_to_remove.push(node);
            }
        }

    // DockNodeMoveWindows->DockNodeAddWindow will normally set those when reaching two windows (which is only adequate during interactive merge)
    // Make sure we don't lose our current pos/size. (FIXME-DOCK: Consider tidying up that code in DockNodeAddWindow instead)
    if (root_node)
    {
        root_node.AuthorityForPos = backup_root_node_authority_for_pos;
        root_node.AuthorityForSize = backup_root_node_authority_for_size;
    }

    // Apply to settings
    for (settings: *mut ImGuiWindowSettings = ctx->SettingsWindows.begin(); settings != null_mut(); settings = ctx->SettingsWindows.next_chunk(settings))
        if (let mut window_settings_dock_id: ImGuiID =  settings->DockId)
            for (let n: c_int = 0; n < nodes_to_remove.Size; n++)
                if (nodes_to_remove[n]->ID == window_settings_dock_id)
                {
                    settings->DockId = root_id;
                    break;
                }

    // Not really efficient, but easier to destroy a whole hierarchy considering DockContextRemoveNode is attempting to merge nodes
    if (nodes_to_remove.Size > 1)
        ImQsort(nodes_to_remove.Data, nodes_to_remove.Size, sizeof(ImGuiDockNode*), DockNodeComparerDepthMostFirst);
    for (let n: c_int = 0; n < nodes_to_remove.Size; n++)
        DockContextRemoveNode(ctx, nodes_to_remove[n], false);

    if (root_id == 0)
    {
        dc->Nodes.Clear();
        dc->Requests.clear();
    }
    else if (has_central_node)
    {
        root_node.CentralNode = root_node;
        root_node.SetLocalFlags(root_node.LocalFlags | ImGuiDockNodeFlags_CentralNode);
    }
}

c_void DockBuilderRemoveNodeDockedWindows(ImGuiID root_id, clear_settings_refs: bool)
{
    // Clear references in settings
    ImGuiContext* ctx = GImGui;
    ImGuiContext& g = *ctx;
    if (clear_settings_refs)
    {
        for (settings: *mut ImGuiWindowSettings = g.SettingsWindows.begin(); settings != null_mut(); settings = g.SettingsWindows.next_chunk(settings))
        {
            let mut want_removal: bool =  (root_id == 0) || (settings->DockId == root_id);
            if (!want_removal && settings->DockId != 0)
                if (ImGuiDockNode* node = DockContextFindNodeByID(ctx, settings->DockId))
                    if (DockNodeGetRootNode(node)->ID == root_id)
                        want_removal = true;
            if (want_removal)
                settings->DockId = 0;
        }
    }

    // Clear references in windows
    for (let n: c_int = 0; n < g.Windows.len(); n++)
    {
        let mut window: *mut ImGuiWindow =  g.Windows[n];
        let mut want_removal: bool =  (root_id == 0) || (window.DockNode && DockNodeGetRootNode(window.DockNode)->ID == root_id) || (window.DockNodeAsHost && window.DockNodeAsHost->ID == root_id);
        if (want_removal)
        {
            let mut backup_dock_id: ImGuiID =  window.DockId;
            IM_UNUSED(backup_dock_id);
            DockContextProcessUndockWindow(ctx, window, clear_settings_refs);
            if (!clear_settings_refs)
                // IM_ASSERT(window.DockId == backup_dock_id);
        }
    }
}

// If 'out_id_at_dir' or 'out_id_at_opposite_dir' are non NULL, the function will write out the ID of the two new nodes created.
// Return value is ID of the node at the specified direction, so same as (*out_id_at_dir) if that pointer is set.
// FIXME-DOCK: We are not exposing nor using split_outer.
ImGuiID DockBuilderSplitNode(id: ImGuiID, split_dir: ImGuiDir,size_ratio_for_node_at_dir: c_float, ImGuiID* out_id_at_dir, ImGuiID* out_id_at_opposite_dir)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(split_dir != ImGuiDir_None);
    IMGUI_DEBUG_LOG_DOCKING("[docking] DockBuilderSplitNode: node 0x%08X, split_dir %d\n", id, split_dir);

    ImGuiDockNode* node = DockContextFindNodeByID(&g, id);
    if (node == null_mut())
    {
        // IM_ASSERT(node != NULL);
        return 0;
    }

    // IM_ASSERT(!node->IsSplitNode()); // Assert if already Split

    ImGuiDockRequest req;
    req.Type = ImGuiDockRequestType_Split;
    req.DockTargetWindow= null_mut();
    req.DockTargetNode = node;
    req.DockPayload= null_mut();
    req.DockSplitDir = split_dir;
    req.DockSplitRatio = ImSaturate((split_dir == ImGuiDir_Left || split_dir == ImGuiDir_Up) ? size_ratio_for_node_at_dir : 1.0 - size_ratio_for_node_at_dir);
    req.DockSplitOuter = false;
    DockContextProcessDock(&g, &req);

    let mut id_at_dir: ImGuiID =  node.ChildNodes[(split_dir == ImGuiDir_Left || split_dir == ImGuiDir_Up) ? 0 : 1]->ID;
    let mut id_at_opposite_dir: ImGuiID =  node.ChildNodes[(split_dir == ImGuiDir_Left || split_dir == ImGuiDir_Up) ? 1 : 0]->ID;
    if (out_id_at_dir)
        *out_id_at_dir = id_at_dir;
    if (out_id_at_opposite_dir)
        *out_id_at_opposite_dir = id_at_opposite_dir;
    return id_at_dir;
}

static ImGuiDockNode* DockBuilderCopyNodeRec(ImGuiDockNode* src_node, ImGuiID dst_node_id_if_known, Vec<ImGuiID>* out_node_remap_pairs)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiDockNode* dst_node = DockContextAddNode(&g, dst_node_id_if_known);
    dst_node.SharedFlags = src_node.SharedFlags;
    dst_node.LocalFlags = src_node.LocalFlags;
    dst_node.LocalFlagsInWindows = ImGuiDockNodeFlags_None;
    dst_node.Pos = src_node.Pos;
    dst_node.Size = src_node.Size;
    dst_node.SizeRef = src_node.SizeRef;
    dst_node.SplitAxis = src_node.SplitAxis;
    dst_node.UpdateMergedFlags();

    out_node_remap_pairs.push(src_node.ID);
    out_node_remap_pairs.push(dst_node.ID);

    for (let child_n: c_int = 0; child_n < IM_ARRAYSIZE(src_node.ChildNodes); child_n++)
        if (src_node.ChildNodes[child_n])
        {
            dst_node.ChildNodes[child_n] = DockBuilderCopyNodeRec(src_node.ChildNodes[child_n], 0, out_node_remap_pairs);
            dst_node.ChildNodes[child_n]->ParentNode = dst_node;
        }

    IMGUI_DEBUG_LOG_DOCKING("[docking] Fork node %08X -> %08X (%d childs)\n", src_node.ID, dst_node.ID, dst_node.IsSplitNode() ? 2 : 0);
    return dst_node;
}

c_void DockBuilderCopyNode(ImGuiID src_node_id, ImGuiID dst_node_id, Vec<ImGuiID>* out_node_remap_pairs)
{
    ImGuiContext* ctx = GImGui;
    // IM_ASSERT(src_node_id != 0);
    // IM_ASSERT(dst_node_id != 0);
    // IM_ASSERT(out_node_remap_pairs != NULL);

    DockBuilderRemoveNode(dst_node_id);

    ImGuiDockNode* src_node = DockContextFindNodeByID(ctx, src_node_id);
    // IM_ASSERT(src_node != NULL);

    out_node_remap_pairs->clear();
    DockBuilderCopyNodeRec(src_node, dst_node_id, out_node_remap_pairs);

    // IM_ASSERT((out_node_remap_pairs->Size % 2) == 0);
}

c_void DockBuilderCopyWindowSettings(src_name: *const c_char, dst_name: *const c_char)
{
    let mut src_window: *mut ImGuiWindow =  FindWindowByName(src_name);
    if (src_window == null_mut())
        return;
    if (let mut dst_window: *mut ImGuiWindow =  FindWindowByName(dst_name))
    {
        dst_window.Pos = src_window.Pos;
        dst_window.Size = src_window.Size;
        dst_window.SizeFull = src_window.SizeFull;
        dst_window.Collapsed = src_window.Collapsed;
    }
    else if (dst_settings: *mut ImGuiWindowSettings = FindOrCreateWindowSettings(dst_name))
    {
        ImVec2ih window_pos_2ih = ImVec2ih(src_window.Pos);
        if (src_window.ViewportId != 0 && src_window.ViewportId != IMGUI_VIEWPORT_DEFAULT_ID)
        {
            dst_settings->ViewportPos = window_pos_2ih;
            dst_settings->ViewportId = src_window.ViewportId;
            dst_settings->Pos = ImVec2ih(0, 0);
        }
        else
        {
            dst_settings->Pos = window_pos_2ih;
        }
        dst_settings->Size = ImVec2ih(src_window.SizeFull);
        dst_settings->Collapsed = src_window.Collapsed;
    }
}

// FIXME: Will probably want to change this signature, in particular how the window remapping pairs are passed.
c_void DockBuilderCopyDockSpace(ImGuiID src_dockspace_id, ImGuiID dst_dockspace_id, Vec<*const char>* in_window_remap_pairs)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(src_dockspace_id != 0);
    // IM_ASSERT(dst_dockspace_id != 0);
    // IM_ASSERT(in_window_remap_pairs != NULL);
    // IM_ASSERT((in_window_remap_pairs->Size % 2) == 0);

    // Duplicate entire dock
    // FIXME: When overwriting dst_dockspace_id, windows that aren't part of our dockspace window class but that are docked in a same node will be split apart,
    // whereas we could attempt to at least keep them together in a new, same floating node.
    Vec<ImGuiID> node_remap_pairs;
    DockBuilderCopyNode(src_dockspace_id, dst_dockspace_id, &node_remap_pairs);

    // Attempt to transition all the upcoming windows associated to dst_dockspace_id into the newly created hierarchy of dock nodes
    // (The windows associated to src_dockspace_id are staying in place)
    Vec<ImGuiID> src_windows;
    for (let remap_window_n: c_int = 0; remap_window_n < in_window_remap_pairs->Size; remap_window_n += 2)
    {
        let mut  src_window_name: *const c_char = (*in_window_remap_pairs)[remap_window_n];
        let mut  dst_window_name: *const c_char = (*in_window_remap_pairs)[remap_window_n + 1];
        let mut src_window_id: ImGuiID =  ImHashStr(src_window_name);
        src_windows.push(src_window_id);

        // Search in the remapping tables
        let mut src_dock_id: ImGuiID =  0;
        if (let mut src_window: *mut ImGuiWindow =  FindWindowByID(src_window_id))
            src_dock_id = src_window.DockId;
        else if (src_window_settings: *mut ImGuiWindowSettings = FindWindowSettings(src_window_id))
            src_dock_id = src_window_settings->DockId;
        let mut dst_dock_id: ImGuiID =  0;
        for (let dock_remap_n: c_int = 0; dock_remap_n < node_remap_pairs.Size; dock_remap_n += 2)
            if (node_remap_pairs[dock_remap_n] == src_dock_id)
            {
                dst_dock_id = node_remap_pairs[dock_remap_n + 1];
                //node_remap_pairs[dock_remap_n] = node_remap_pairs[dock_remap_n + 1] = 0; // Clear
                break;
            }

        if (dst_dock_id != 0)
        {
            // Docked windows gets redocked into the new node hierarchy.
            IMGUI_DEBUG_LOG_DOCKING("[docking] Remap live window '%s' 0x%08X -> '%s' 0x%08X\n", src_window_name, src_dock_id, dst_window_name, dst_dock_id);
            DockBuilderDockWindow(dst_window_name, dst_dock_id);
        }
        else
        {
            // Floating windows gets their settings transferred (regardless of whether the new window already exist or not)
            // When this is leading to a Copy and not a Move, we would get two overlapping floating windows. Could we possibly dock them together?
            IMGUI_DEBUG_LOG_DOCKING("[docking] Remap window settings '%s' -> '%s'\n", src_window_name, dst_window_name);
            DockBuilderCopyWindowSettings(src_window_name, dst_window_name);
        }
    }

    // Anything else in the source nodes of 'node_remap_pairs' are windows that were docked in src_dockspace_id but are not owned by it (unaffiliated windows, e.g. "ImGui Demo")
    // Find those windows and move to them to the cloned dock node. This may be optional?
    for (let dock_remap_n: c_int = 0; dock_remap_n < node_remap_pairs.Size; dock_remap_n += 2)
        if (let mut src_dock_id: ImGuiID =  node_remap_pairs[dock_remap_n])
        {
            let mut dst_dock_id: ImGuiID =  node_remap_pairs[dock_remap_n + 1];
            ImGuiDockNode* node = DockBuilderGetNode(src_dock_id);
            for (let window_n: c_int = 0; window_n < node.Windows.len(); window_n++)
            {
                let mut window: *mut ImGuiWindow =  node.Windows[window_n];
                if (src_windows.contains(window.ID))
                    continue;

                // Docked windows gets redocked into the new node hierarchy.
                IMGUI_DEBUG_LOG_DOCKING("[docking] Remap window '%s' %08X -> %08X\n", window.Name, src_dock_id, dst_dock_id);
                DockBuilderDockWindow(window.Name, dst_dock_id);
            }
        }
}

// FIXME-DOCK: This is awkward because in series of split user is likely to loose access to its root node.
c_void DockBuilderFinish(ImGuiID root_id)
{
    ImGuiContext* ctx = GImGui;
    //DockContextRebuild(ctx);
    DockContextBuildAddWindowsToNodes(ctx, root_id);
}

//-----------------------------------------------------------------------------
// Docking: Begin/End Support Functions (called from Begin/End)
//-----------------------------------------------------------------------------
// - GetWindowAlwaysWantOwnTabBar()
// - DockContextBindNodeToWindow()
// - BeginDocked()
// - BeginDockableDragDropSource()
// - BeginDockableDragDropTarget()
//-----------------------------------------------------------------------------

GetWindowAlwaysWantOwnTabBar: bool(window: *mut ImGuiWindow)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.IO.ConfigDockingAlwaysTabBar || window.WindowClass.DockingAlwaysTabBar)
        if ((window.Flags & (ImGuiWindowFlags_ChildWindow | ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoDocking)) == 0)
            if (!window.IsFallbackWindow)    // We don't support AlwaysTabBar on the fallback/implicit window to avoid unused dock-node overhead/noise
                return true;
    return false;
}

static ImGuiDockNode* DockContextBindNodeToWindow(ImGuiContext* ctx, window: *mut ImGuiWindow)
{
    ImGuiContext& g = *ctx;
    ImGuiDockNode* node = DockContextFindNodeByID(ctx, window.DockId);
    // IM_ASSERT(window.DockNode == NULL);

    // We should not be docking into a split node (SetWindowDock should avoid this)
    if (node && node.IsSplitNode())
    {
        DockContextProcessUndockWindow(ctx, window);
        return null_mut();
    }

    // Create node
    if (node == null_mut())
    {
        node = DockContextAddNode(ctx, window.DockId);
        node.AuthorityForPos = node.AuthorityForSize = node.AuthorityForViewport = ImGuiDataAuthority_Window;
        node.LastFrameAlive = g.FrameCount;
    }

    // If the node just turned visible and is part of a hierarchy, it doesn't have a Size assigned by DockNodeTreeUpdatePosSize() yet,
    // so we're forcing a Pos/Size update from the first ancestor that is already visible (often it will be the root node).
    // If we don't do this, the window will be assigned a zero-size on its first frame, which won't ideally warm up the layout.
    // This is a little wonky because we don't normally update the Pos/Size of visible node mid-frame.
    if (!node.IsVisible)
    {
        ImGuiDockNode* ancestor_node = node;
        while (!ancestor_node.IsVisible && ancestor_node.ParentNode)
            ancestor_node = ancestor_node.ParentNode;
        // IM_ASSERT(ancestor_node->Size.x > 0.0 && ancestor_node->Size.y > 0.0);
        DockNodeUpdateHasCentralNodeChild(DockNodeGetRootNode(ancestor_node));
        DockNodeTreeUpdatePosSize(ancestor_node, ancestor_node.Pos, ancestor_node.Size, node);
    }

    // Add window to node
    let mut node_was_visible: bool =  node.IsVisible;
    DockNodeAddWindow(node, window, true);
    node.IsVisible = node_was_visible; // Don't mark visible right away (so DockContextEndFrame() doesn't render it, maybe other side effects? will see)
    // IM_ASSERT(node == window.DockNode);
    return node;
}

c_void BeginDocked(window: *mut ImGuiWindow,p_open: *mut bool)
{
    ImGuiContext* ctx = GImGui;
    ImGuiContext& g = *ctx;

    // Clear fields ahead so most early-out paths don't have to do it
    window.DockIsActive = window.DockNodeIsVisible = window.DockTabIsVisible = false;

    let auto_dock_node: bool = GetWindowAlwaysWantOwnTabBar(window);
    if (auto_dock_node)
    {
        if (window.DockId == 0)
        {
            // IM_ASSERT(window.DockNode == NULL);
            window.DockId = DockContextGenNodeID(ctx);
        }
    }
    else
    {
        // Calling SetNextWindowPos() undock windows by default (by setting PosUndock)
        let mut want_undock: bool =  false;
        want_undock |= (window.Flags & ImGuiWindowFlags_NoDocking) != 0;
        want_undock |= (g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasPos) && (window.SetWindowPosAllowFlags & g.NextWindowData.PosCond) && g.NextWindowData.PosUndock;
        if (want_undock)
        {
            DockContextProcessUndockWindow(ctx, window);
            return;
        }
    }

    // Bind to our dock node
    ImGuiDockNode* node = window.DockNode;
    if (node != null_mut())
        // IM_ASSERT(window.DockId == node->ID);
    if (window.DockId != 0 && node == null_mut())
    {
        node = DockContextBindNodeToWindow(ctx, window);
        if (node == null_mut())
            return;
    }

// #if 0
    // Undock if the ImGuiDockNodeFlags_NoDockingInCentralNode got set
    if (node.IsCentralNode && (node.Flags & ImGuiDockNodeFlags_NoDockingInCentralNode))
    {
        DockContextProcessUndockWindow(ctx, window);
        return;
    }
// #endif

    // Undock if our dockspace node disappeared
    // Note how we are testing for LastFrameAlive and NOT LastFrameActive. A DockSpace node can be maintained alive while being inactive with ImGuiDockNodeFlags_KeepAliveOnly.
    if (node.LastFrameAlive < g.FrameCount)
    {
        // If the window has been orphaned, transition the docknode to an implicit node processed in DockContextNewFrameUpdateDocking()
        ImGuiDockNode* root_node = DockNodeGetRootNode(node);
        if (root_node.LastFrameAlive < g.FrameCount)
            DockContextProcessUndockWindow(ctx, window);
        else
            window.DockIsActive = true;
        return;
    }

    // Store style overrides
    for (let color_n: c_int = 0; color_n < ImGuiWindowDockStyleCol_COUNT; color_n++)
        window.DockStyle.Colors[color_n] = ColorConvertFloat4ToU32(g.Style.Colors[GWindowDockStyleColors[color_n]]);

    // Fast path return. It is common for windows to hold on a persistent DockId but be the only visible window,
    // and never create neither a host window neither a tab bar.
    // FIXME-DOCK: replace ->HostWindow NULL compare with something more explicit (~was initially intended as a first frame test)
    if (node.HostWindow == null_mut())
    {
        if (node.State == ImGuiDockNodeState_HostWindowHiddenBecauseWindowsAreResizing)
            window.DockIsActive = true;
        if (node.Windows.len() > 1)
            DockNodeHideWindowDuringHostWindowCreation(window);
        return;
    }

    // We can have zero-sized nodes (e.g. children of a small-size dockspace)
    // IM_ASSERT(node->HostWindow);
    // IM_ASSERT(node->IsLeafNode());
    // IM_ASSERT(node->Size.x >= 0.0 && node->Size.y >= 0.0);
    node.State = ImGuiDockNodeState_HostWindowVisible;

    // Undock if we are submitted earlier than the host window
    if (!(node.MergedFlags & ImGuiDockNodeFlags_KeepAliveOnly) && window.BeginOrderWithinContext < node.Hostwindow.BeginOrderWithinContext)
    {
        DockContextProcessUndockWindow(ctx, window);
        return;
    }

    // Position/Size window
    SetNextWindowPos(node.Pos);
    SetNextWindowSize(node.Size);
    g.NextWindowData.PosUndock = false; // Cancel implicit undocking of SetNextWindowPos()
    window.DockIsActive = true;
    window.DockNodeIsVisible = true;
    window.DockTabIsVisible = false;
    if (node.MergedFlags & ImGuiDockNodeFlags_KeepAliveOnly)
        return;

    // When the window is selected we mark it as visible.
    if (node.VisibleWindow == window)
        window.DockTabIsVisible = true;

    // Update window flag
    // IM_ASSERT((window.Flags & ImGuiWindowFlags_ChildWindow) == 0);
    window.Flags |= ImGuiWindowFlags_ChildWindow | ImGuiWindowFlags_AlwaysUseWindowPadding | ImGuiWindowFlags_NoResize;
    if (node.IsHiddenTabBar() || node.IsNoTabBar())
        window.Flags |= ImGuiWindowFlags_NoTitleBar;
    else
        window.Flags &= !ImGuiWindowFlags_NoTitleBar;      // Clear the NoTitleBar flag in case the user set it: confusingly enough we need a title bar height so we are correctly offset, but it won't be displayed!

    // Save new dock order only if the window has been visible once already
    // This allows multiple windows to be created in the same frame and have their respective dock orders preserved.
    if (node.TabBar && window.WasActive)
        window.DockOrder = DockNodeGetTabOrder(window);

    if ((node.WantCloseAll || node.WantCloseTabId == window.TabId) && p_open != null_mut())
        *p_open = false;

    // Update ChildId to allow returning from Child to Parent with Escape
    let mut parent_window: *mut ImGuiWindow =  window.DockNode.HostWindow;
    window.ChildId = parent_window.GetID(window.Name);
}

c_void BeginDockableDragDropSource(window: *mut ImGuiWindow)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.ActiveId == window.MoveId);
    // IM_ASSERT(g.MovingWindow == window);
    // IM_ASSERT(g.CurrentWindow == window);

    g.LastItemData.ID = window.MoveId;
    window = window.RootWindowDockTree;
    // IM_ASSERT((window.Flags & ImGuiWindowFlags_NoDocking) == 0);
    let mut is_drag_docking: bool =  (g.IO.ConfigDockingWithShift) || ImRect(0, 0, window.SizeFull.x, GetFrameHeight()).Contains(g.ActiveIdClickOffset); // FIXME-DOCKING: Need to make this stateful and explicit
    if (is_drag_docking && BeginDragDropSource(ImGuiDragDropFlags_SourceNoPreviewTooltip | ImGuiDragDropFlags_SourceNoHoldToOpenOthers | ImGuiDragDropFlags_SourceAutoExpirePayload))
    {
        SetDragDropPayload(IMGUI_PAYLOAD_TYPE_WINDOW, &window, sizeof(window));
        EndDragDropSource();

        // Store style overrides
        for (let color_n: c_int = 0; color_n < ImGuiWindowDockStyleCol_COUNT; color_n++)
            window.DockStyle.Colors[color_n] = ColorConvertFloat4ToU32(g.Style.Colors[GWindowDockStyleColors[color_n]]);
    }
}

c_void BeginDockableDragDropTarget(window: *mut ImGuiWindow)
{
    ImGuiContext* ctx = GImGui;
    ImGuiContext& g = *ctx;

    //IM_ASSERT(window.RootWindowDockTree == window); // May also be a DockSpace
    // IM_ASSERT((window.Flags & ImGuiWindowFlags_NoDocking) == 0);
    if (!g.DragDropActive)
        return;
    //GetForegroundDrawList(window)->AddRect(window.Pos, window.Pos + window.Size, IM_COL32(255, 255, 0, 255));
    if (!BeginDragDropTargetCustom(window.Rect(), window.ID))
        return;

    // Peek into the payload before calling AcceptDragDropPayload() so we can handle overlapping dock nodes with filtering
    // (this is a little unusual pattern, normally most code would call AcceptDragDropPayload directly)
    let payload: *const ImGuiPayload = &g.DragDropPayload;
    if (!payload->IsDataType(IMGUI_PAYLOAD_TYPE_WINDOW) || !DockNodeIsDropAllowed(window, *(ImGuiWindow**)payload->Data))
    {
        EndDragDropTarget();
        return;
    }

    let mut payload_window: *mut ImGuiWindow =  *(ImGuiWindow**)payload->Data;
    if (AcceptDragDropPayload(IMGUI_PAYLOAD_TYPE_WINDOW, ImGuiDragDropFlags_AcceptBeforeDelivery | ImGuiDragDropFlags_AcceptNoDrawDefaultRect))
    {
        // Select target node
        // (Important: we cannot use g.HoveredDockNode here! Because each of our target node have filters based on payload, each candidate drop target will do its own evaluation)
        let mut dock_into_floating_window: bool =  false;
        ImGuiDockNode* node= null_mut();
        if (window.DockNodeAsHost)
        {
            // Cannot assume that node will != NULL even though we passed the rectangle test: it depends on padding/spacing handled by DockNodeTreeFindVisibleNodeByPos().
            node = DockNodeTreeFindVisibleNodeByPos(window.DockNodeAsHost, g.IO.MousePos);

            // There is an edge case when docking into a dockspace which only has _inactive_ nodes (because none of the windows are active)
            // In this case we need to fallback into any leaf mode, possibly the central node.
            // FIXME-20181220: We should not have to test for IsLeafNode() here but we have another bug to fix first.
            if (node && node.IsDockSpace() && node.IsRootNode())
                node = (node.CentralNode && node.IsLeafNode()) ? node.CentralNode : DockNodeTreeFindFallbackLeafNode(node);
        }
        else
        {
            if (window.DockNode)
                node = window.DockNode;
            else
                dock_into_floating_window = true; // Dock into a regular window
        }

        let explicit_target_rect: ImRect =  (node && node.TabBar && !node.IsHiddenTabBar() && !node.IsNoTabBar()) ? node.TabBar->BarRect : ImRect(window.Pos, window.Pos + ImVec2::new(window.Size.x, GetFrameHeight()));
        let is_explicit_target: bool = g.IO.ConfigDockingWithShift || IsMouseHoveringRect(explicit_target_rect.Min, explicit_target_rect.Max);

        // Preview docking request and find out split direction/ratio
        //let do_preview: bool = true;     // Ignore testing for payload->IsPreview() which removes one frame of delay, but breaks overlapping drop targets within the same window.
        let do_preview: bool = payload->IsPreview() || payload->IsDelivery();
        if (do_preview && (node != null_mut() || dock_into_floating_window))
        {
            ImGuiDockPreviewData split_inner;
            ImGuiDockPreviewData split_outer;
            ImGuiDockPreviewData* split_data = &split_inner;
            if (node && (node.ParentNode || node.IsCentralNode()))
                if (ImGuiDockNode* root_node = DockNodeGetRootNode(node))
                {
                    DockNodePreviewDockSetup(window, root_node, payload_window, null_mut(), &split_outer, is_explicit_target, true);
                    if (split_outer.IsSplitDirExplicit)
                        split_data = &split_outer;
                }
            DockNodePreviewDockSetup(window, node, payload_window, null_mut(), &split_inner, is_explicit_target, false);
            if (split_data == &split_outer)
                split_inner.IsDropAllowed = false;

            // Draw inner then outer, so that previewed tab (in inner data) will be behind the outer drop boxes
            DockNodePreviewDockRender(window, node, payload_window, &split_inner);
            DockNodePreviewDockRender(window, node, payload_window, &split_outer);

            // Queue docking request
            if (split_Data.IsDropAllowed && payload->IsDelivery())
                DockContextQueueDock(ctx, window, split_Data.SplitNode, payload_window, split_Data.SplitDir, split_Data.SplitRatio, split_data == &split_outer);
        }
    }
    EndDragDropTarget();
}

//-----------------------------------------------------------------------------
// Docking: Settings
//-----------------------------------------------------------------------------
// - DockSettingsRenameNodeReferences()
// - DockSettingsRemoveNodeReferences()
// - DockSettingsFindNodeSettings()
// - DockSettingsHandler_ApplyAll()
// - DockSettingsHandler_ReadOpen()
// - DockSettingsHandler_ReadLine()
// - DockSettingsHandler_DockNodeToSettings()
// - DockSettingsHandler_WriteAll()
//-----------------------------------------------------------------------------

static c_void DockSettingsRenameNodeReferences(ImGuiID old_node_id, ImGuiID new_node_id)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    IMGUI_DEBUG_LOG_DOCKING("[docking] DockSettingsRenameNodeReferences: from 0x%08X -> to 0x%08X\n", old_node_id, new_node_id);
    for (let window_n: c_int = 0; window_n < g.Windows.len(); window_n++)
    {
        let mut window: *mut ImGuiWindow =  g.Windows[window_n];
        if (window.DockId == old_node_id && window.DockNode == null_mut())
            window.DockId = new_node_id;
    }
    //// FIXME-OPT: We could remove this loop by storing the index in the map
    for (settings: *mut ImGuiWindowSettings = g.SettingsWindows.begin(); settings != null_mut(); settings = g.SettingsWindows.next_chunk(settings))
        if (settings->DockId == old_node_id)
            settings->DockId = new_node_id;
}

// Remove references stored in ImGuiWindowSettings to the given ImGuiDockNodeSettings
static c_void DockSettingsRemoveNodeReferences(ImGuiID* node_ids, node_ids_count: c_int)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let found: c_int = 0;
    //// FIXME-OPT: We could remove this loop by storing the index in the map
    for (settings: *mut ImGuiWindowSettings = g.SettingsWindows.begin(); settings != null_mut(); settings = g.SettingsWindows.next_chunk(settings))
        for (let node_n: c_int = 0; node_n < node_ids_count; node_n++)
            if (settings->DockId == node_ids[node_n])
            {
                settings->DockId = 0;
                settings->DockOrder = -1;
                if (++found < node_ids_count)
                    break;
                return;
            }
}

static ImGuiDockNodeSettings* DockSettingsFindNodeSettings(ImGuiContext* ctx, id: ImGuiID)
{
    // FIXME-OPT
    ImGuiDockContext* dc  = &ctx->DockContext;
    for (let n: c_int = 0; n < dc->NodesSettings.Size; n++)
        if (dc->NodesSettings[n].ID == id)
            return &dc->NodesSettings[n];
    return null_mut();
}

// Clear settings data
static c_void DockSettingsHandler_ClearAll(ImGuiContext* ctx, ImGuiSettingsHandler*)
{
    ImGuiDockContext* dc  = &ctx->DockContext;
    dc->NodesSettings.clear();
    DockContextClearNodes(ctx, 0, true);
}

// Recreate nodes based on settings data
static c_void DockSettingsHandler_ApplyAll(ImGuiContext* ctx, ImGuiSettingsHandler*)
{
    // Prune settings at boot time only
    ImGuiDockContext* dc  = &ctx->DockContext;
    if (ctx->Windows.len() == 0)
        DockContextPruneUnusedSettingsNodes(ctx);
    DockContextBuildNodesFromSettings(ctx, dc->NodesSettings.Data, dc->NodesSettings.Size);
    DockContextBuildAddWindowsToNodes(ctx, 0);
}

static *mut c_void DockSettingsHandler_ReadOpen(ImGuiContext*, ImGuiSettingsHandler*, name: *const c_char)
{
    if (strcmp(name, "Data") != 0)
        return null_mut();
    return (*mut c_void)1;
}

static c_void DockSettingsHandler_ReadLine(ImGuiContext* ctx, ImGuiSettingsHandler*, *mut c_void, line: *const c_char)
{
     c: c_char = 0;
    let x: c_int = 0, y = 0;
    let r: c_int = 0;

    // Parsing, e.g.
    // " DockNode   ID=0x00000001 Pos=383,193 Size=201,322 Split=Y,0.506 "
    // "   DockNode ID=0x00000002 Parent=0x00000001 "
    // Important: this code expect currently fields in a fixed order.
    ImGuiDockNodeSettings node;
    line = ImStrSkipBlank(line);
    if      (strncmp(line, "DockNode", 8) == 0)  { line = ImStrSkipBlank(line + strlen("DockNode")); }
    else if (strncmp(line, "DockSpace", 9) == 0) { line = ImStrSkipBlank(line + strlen("DockSpace")); node.Flags |= ImGuiDockNodeFlags_DockSpace; }
    else return;
    if (sscanf(line, "ID=0x%08X%n",      &node.ID, &r) == 1)            { line += r; } else return;
    if (sscanf(line, " Parent=0x%08X%n", &node.ParentNodeId, &r) == 1)  { line += r; if (node.ParentNodeId == 0) return; }
    if (sscanf(line, " Window=0x%08X%n", &node.ParentWindowId, &r) ==1) { line += r; if (node.ParentWindowId == 0) return; }
    if (node.ParentNodeId == 0)
    {
        if (sscanf(line, " Pos=%i,%i%n",  &x, &y, &r) == 2)         { line += r; node.Pos = ImVec2ih(x, y); } else return;
        if (sscanf(line, " Size=%i,%i%n", &x, &y, &r) == 2)         { line += r; node.Size = ImVec2ih(x, y); } else return;
    }
    else
    {
        if (sscanf(line, " SizeRef=%i,%i%n", &x, &y, &r) == 2)      { line += r; node.SizeRef = ImVec2ih(x, y); }
    }
    if (sscanf(line, " Split=%c%n", &c, &r) == 1)                   { line += r; if (c == 'X') node.SplitAxis = ImGuiAxis_X; else if (c == 'Y') node.SplitAxis = ImGuiAxis_Y; }
    if (sscanf(line, " NoResize=%d%n", &x, &r) == 1)                { line += r; if (x != 0) node.Flags |= ImGuiDockNodeFlags_NoResize; }
    if (sscanf(line, " CentralNode=%d%n", &x, &r) == 1)             { line += r; if (x != 0) node.Flags |= ImGuiDockNodeFlags_CentralNode; }
    if (sscanf(line, " NoTabBar=%d%n", &x, &r) == 1)                { line += r; if (x != 0) node.Flags |= ImGuiDockNodeFlags_NoTabBar; }
    if (sscanf(line, " HiddenTabBar=%d%n", &x, &r) == 1)            { line += r; if (x != 0) node.Flags |= ImGuiDockNodeFlags_HiddenTabBar; }
    if (sscanf(line, " NoWindowMenuButton=%d%n", &x, &r) == 1)      { line += r; if (x != 0) node.Flags |= ImGuiDockNodeFlags_NoWindowMenuButton; }
    if (sscanf(line, " NoCloseButton=%d%n", &x, &r) == 1)           { line += r; if (x != 0) node.Flags |= ImGuiDockNodeFlags_NoCloseButton; }
    if (sscanf(line, " Selected=0x%08X%n", &node.SelectedTabId,&r) == 1) { line += r; }
    if (node.ParentNodeId != 0)
        if (ImGuiDockNodeSettings* parent_settings = DockSettingsFindNodeSettings(ctx, node.ParentNodeId))
            node.Depth = parent_settings->Depth + 1;
    ctx->DockContext.NodesSettings.push(node);
}

static c_void DockSettingsHandler_DockNodeToSettings(ImGuiDockContext* dc, ImGuiDockNode* node, depth: c_int)
{
    ImGuiDockNodeSettings node_settings;
    // IM_ASSERT(depth < (1 << (sizeof(node_settings.Depth) << 3)));
    node_settings.ID = node.ID;
    node_settings.ParentNodeId = node.ParentNode ? node.ParentNode.ID : 0;
    node_settings.ParentWindowId = (node.IsDockSpace() && node.HostWindow && node.Hostwindow.ParentWindow) ? node.Hostwindow.Parentwindow.ID : 0;
    node_settings.SelectedTabId = node.SelectedTabId;
    node_settings.SplitAxis = (node.IsSplitNode() ? node.SplitAxis : ImGuiAxis_None);
    node_settings.Depth = depth;
    node_settings.Flags = (node.LocalFlags & ImGuiDockNodeFlags_SavedFlagsMask_);
    node_settings.Pos = ImVec2ih(node.Pos);
    node_settings.Size = ImVec2ih(node.Size);
    node_settings.SizeRef = ImVec2ih(node.SizeRe0f32);
    dc->NodesSettings.push(node_settings);
    if (node.ChildNodes[0])
        DockSettingsHandler_DockNodeToSettings(dc, node.ChildNodes[0], depth + 1);
    if (node.ChildNodes[1])
        DockSettingsHandler_DockNodeToSettings(dc, node.ChildNodes[1], depth + 1);
}

static c_void DockSettingsHandler_WriteAll(ImGuiContext* ctx, ImGuiSettingsHandler* handler, ImGuiTextBuffer* bu0f32)
{
    ImGuiContext& g = *ctx;
    ImGuiDockContext* dc = &ctx->DockContext;
    if (!(g.IO.ConfigFlags & ImGuiConfigFlags_DockingEnable))
        return;

    // Gather settings data
    // (unlike our windows settings, because nodes are always built we can do a full rewrite of the SettingsNode buffer)
    dc->NodesSettings.clear();
    dc->NodesSettings.reserve(dc->Nodes.Data.Size);
    for (let n: c_int = 0; n < dc->Nodes.Data.Size; n++)
        if (ImGuiDockNode* node = (ImGuiDockNode*)dc->Nodes.Data[n].val_p)
            if (node.IsRootNode())
                DockSettingsHandler_DockNodeToSettings(dc, node, 0);

    let max_depth: c_int = 0;
    for (let node_n: c_int = 0; node_n < dc->NodesSettings.Size; node_n++)
        max_depth = ImMax(dc->NodesSettings[node_n].Depth, max_depth);

    // Write to text buffer
    buf->appendf("[%s][Data]\n", handler.TypeName);
    for (let node_n: c_int = 0; node_n < dc->NodesSettings.Size; node_n++)
    {
        let line_start_pos: c_int = buf->size(); (c_void)line_start_pos;
        let node_settings: *const ImGuiDockNodeSettings = &dc->NodesSettings[node_n];
        buf->appendf("%*s%s%*s", node_settings->Depth * 2, "", (node_settings->Flags & ImGuiDockNodeFlags_DockSpace) ? "DockSpace" : "DockNode ", (max_depth - node_settings->Depth) * 2, "");  // Text align nodes to facilitate looking at .ini file
        buf->appendf(" ID=0x%08X", node_settings->ID);
        if (node_settings->ParentNodeId)
        {
            buf->appendf(" Parent=0x%08X SizeRef=%d,%d", node_settings->ParentNodeId, node_settings->SizeRef.x, node_settings->SizeRef.y);
        }
        else
        {
            if (node_settings->ParentWindowId)
                buf->appendf(" Window=0x%08X", node_settings->ParentWindowId);
            buf->appendf(" Pos=%d,%d Size=%d,%d", node_settings->Pos.x, node_settings->Pos.y, node_settings->Size.x, node_settings->Size.y);
        }
        if (node_settings->SplitAxis != ImGuiAxis_None)
            buf->appendf(" Split=%c", (node_settings->SplitAxis == ImGuiAxis_X) ? 'X' : 'Y');
        if (node_settings->Flags & ImGuiDockNodeFlags_NoResize)
            buf->appendf(" NoResize=1");
        if (node_settings->Flags & ImGuiDockNodeFlags_CentralNode)
            buf->appendf(" CentralNode=1");
        if (node_settings->Flags & ImGuiDockNodeFlags_NoTabBar)
            buf->appendf(" NoTabBar=1");
        if (node_settings->Flags & ImGuiDockNodeFlags_HiddenTabBar)
            buf->appendf(" HiddenTabBar=1");
        if (node_settings->Flags & ImGuiDockNodeFlags_NoWindowMenuButton)
            buf->appendf(" NoWindowMenuButton=1");
        if (node_settings->Flags & ImGuiDockNodeFlags_NoCloseButton)
            buf->appendf(" NoCloseButton=1");
        if (node_settings->SelectedTabId)
            buf->appendf(" Selected=0x%08X", node_settings->SelectedTabId);

// #if IMGUI_DEBUG_INI_SETTINGS
        // [DEBUG] Include comments in the .ini file to ease debugging
        if (ImGuiDockNode* node = DockContextFindNodeByID(ctx, node_settings->ID))
        {
            buf->appendf("%*s", ImMax(2, (line_start_pos + 92) - buf->size()), "");     // Align everything
            if (node.IsDockSpace() && node.HostWindow && node.Hostwindow.ParentWindow)
                buf->appendf(" ; in '%s'", node.Hostwindow.Parentwindow.Name);
            // Iterate settings so we can give info about windows that didn't exist during the session.
            let contains_window: c_int = 0;
            for (settings: *mut ImGuiWindowSettings = g.SettingsWindows.begin(); settings != null_mut(); settings = g.SettingsWindows.next_chunk(settings))
                if (settings->DockId == node_settings->ID)
                {
                    if (contains_window++ == 0)
                        buf->appendf(" ; contains ");
                    buf->appendf("'%s' ", settings->GetName());
                }
        }
// #endif
        buf->appendf("\n");
    }
    buf->appendf("\n");
}


//-----------------------------------------------------------------------------
// [SECTION] PLATFORM DEPENDENT HELPERS
//-----------------------------------------------------------------------------

// #if defined(_WIN32) && !defined(IMGUI_DISABLE_WIN32_FUNCTIONS) && !defined(IMGUI_DISABLE_WIN32_DEFAULT_CLIPBOARD_FUNCTIONS)

// #ifdef _MSC_VER
// #pragma comment(lib, "user32")
// #pragma comment(lib, "kernel32")
// #endif

// Win32 clipboard implementation
// We use g.ClipboardHandlerData for temporary storage to ensure it is freed on Shutdown()
static GetClipboardTextFn_DefaultImpl: *const c_char(*mut c_void)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.ClipboardHandlerData.clear();
    if (!::OpenClipboard(null_mut()))
        return null_mut();
    HANDLE wbuf_handle = ::GetClipboardData(CF_UNICODETEXT);
    if (wbuf_handle == null_mut())
    {
        ::CloseClipboard();
        return null_mut();
    }
    if (*const WCHAR wbuf_global = (*const WCHAR)::GlobalLock(wbuf_handle))
    {
        let buf_len: c_int = ::WideCharToMultiByte(CP_UTF8, 0, wbuf_global, -1, null_mut(), 0, null_mut(), null_mut());
        g.ClipboardHandlerData.resize(buf_len);
        ::WideCharToMultiByte(CP_UTF8, 0, wbuf_global, -1, g.ClipboardHandlerData.Data, buf_len, null_mut(), null_mut());
    }
    ::GlobalUnlock(wbuf_handle);
    ::CloseClipboard();
    return g.ClipboardHandlerData.Data;
}

static c_void SetClipboardTextFn_DefaultImpl(*mut c_void, text: *const c_char)
{
    if (!::OpenClipboard(null_mut()))
        return;
    let wbuf_length: c_int = ::MultiByteToWideChar(CP_UTF8, 0, text, -1, null_mut(), 0);
    HGLOBAL wbuf_handle = ::GlobalAlloc(GMEM_MOVEABLE, wbuf_length * sizeof(WCHAR));
    if (wbuf_handle == null_mut())
    {
        ::CloseClipboard();
        return;
    }
    WCHAR* wbuf_global = (WCHAR*)::GlobalLock(wbuf_handle);
    ::MultiByteToWideChar(CP_UTF8, 0, text, -1, wbuf_global, wbuf_length);
    ::GlobalUnlock(wbuf_handle);
    ::EmptyClipboard();
    if (::SetClipboardData(CF_UNICODETEXT, wbuf_handle) == null_mut())
        ::GlobalFree(wbuf_handle);
    ::CloseClipboard();
}

// #elif defined(__APPLE__) && TARGET_OS_OSX && defined(IMGUI_ENABLE_OSX_DEFAULT_CLIPBOARD_FUNCTIONS)

// #include <Carbon/Carbon.h>  // Use old API to avoid need for separate .mm file
static PasteboardRef main_clipboard = 0;

// OSX clipboard implementation
// If you enable this you will need to add '-framework ApplicationServices' to your linker command-line!
static c_void SetClipboardTextFn_DefaultImpl(*mut c_void, text: *const c_char)
{
    if (!main_clipboard)
        PasteboardCreate(kPasteboardClipboard, &main_clipboard);
    PasteboardClear(main_clipboard);
    CFDataRef cf_data = CFDataCreate(kCFAllocatorDefault, (*const UInt8)text, strlen(text));
    if (cf_data)
    {
        PasteboardPutItemFlavor(main_clipboard, (PasteboardItemID)1, CFSTR("public.utf8-plain-text"), cf_data, 0);
        CFRelease(cf_data);
    }
}

static GetClipboardTextFn_DefaultImpl: *const c_char(*mut c_void)
{
    if (!main_clipboard)
        PasteboardCreate(kPasteboardClipboard, &main_clipboard);
    PasteboardSynchronize(main_clipboard);

    ItemCount item_count = 0;
    PasteboardGetItemCount(main_clipboard, &item_count);
    for (ItemCount i = 0; i < item_count; i++)
    {
        PasteboardItemID item_id = 0;
        PasteboardGetItemIdentifier(main_clipboard, i + 1, &item_id);
        CFArrayRef flavor_type_array = 0;
        PasteboardCopyItemFlavors(main_clipboard, item_id, &flavor_type_array);
        for (CFIndex j = 0, nj = CFArrayGetCount(flavor_type_array); j < nj; j++)
        {
            CFDataRef cf_data;
            if (PasteboardCopyItemFlavorData(main_clipboard, item_id, CFSTR("public.utf8-plain-text"), &cf_data) == noErr)
            {
                let g = GImGui; // ImGuiContext& g = *GImGui;
                g.ClipboardHandlerData.clear();
                let length: c_int = CFDataGetLength(cf_data);
                g.ClipboardHandlerData.resize(length + 1);
                CFDataGetBytes(cf_data, CFRangeMake(0, length), (UInt8*)g.ClipboardHandlerData.Data);
                g.ClipboardHandlerData[length] = 0;
                CFRelease(cf_data);
                return g.ClipboardHandlerData.Data;
            }
        }
    }
    return null_mut();
}

// #else

// Local Dear ImGui-only clipboard implementation, if user hasn't defined better clipboard handlers.
static GetClipboardTextFn_DefaultImpl: *const c_char(*mut c_void)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.ClipboardHandlerData.empty() ? null_mut() : g.ClipboardHandlerData.begin();
}

static c_void SetClipboardTextFn_DefaultImpl(*mut c_void, text: *const c_char)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.ClipboardHandlerData.clear();
    let mut  text_end: *const c_char = text + strlen(text);
    g.ClipboardHandlerData.resize((text_end - text) + 1);
    memcpy(&g.ClipboardHandlerData[0], text, (text_end - text));
    g.ClipboardHandlerData[(text_end - text)] = 0;
}

// #endif

// Win32 API IME support (for Asian languages, etc.)
// #if defined(_WIN32) && !defined(IMGUI_DISABLE_WIN32_FUNCTIONS) && !defined(IMGUI_DISABLE_WIN32_DEFAULT_IME_FUNCTIONS)

// #include <imm.h>
// #ifdef _MSC_VER
// #pragma comment(lib, "imm32")
// #endif

static c_void SetPlatformImeDataFn_DefaultImpl(ImGuiViewport* viewport, ImGuiPlatformImeData* data)
{
    // Notify OS Input Method Editor of text input position
    HWND hwnd = (HWND)viewport.PlatformHandleRaw;
// #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
    if (hwnd == 0)
        hwnd = (HWND)GetIO().ImeWindowHandle;
// #endif
    if (hwnd == 0)
        return;

    //::ImmAssociateContextEx(hwnd, NULL, data->WantVisible ? IACE_DEFAULT : 0);
    if (HIMC himc = ::ImmGetContext(hwnd))
    {
        COMPOSITIONFORM composition_form = {};
        composition_form.ptCurrentPos.x = (LONG)(data.InputPos.x - viewport.Pos.x);
        composition_form.ptCurrentPos.y = (LONG)(data.InputPos.y - viewport.Pos.y);
        composition_form.dwStyle = CFS_FORCE_POSITION;
        ::ImmSetCompositionWindow(himc, &composition_form);
        CANDIDATEFORM candidate_form = {};
        candidate_form.dwStyle = CFS_CANDIDATEPOS;
        candidate_form.ptCurrentPos.x = (LONG)(data.InputPos.x - viewport.Pos.x);
        candidate_form.ptCurrentPos.y = (LONG)(data.InputPos.y - viewport.Pos.y);
        ::ImmSetCandidateWindow(himc, &candidate_form);
        ::ImmReleaseContext(hwnd, himc);
    }
}

// #else

static c_void SetPlatformImeDataFn_DefaultImpl(ImGuiViewport*, ImGuiPlatformImeData*) {}

// #endif

//-----------------------------------------------------------------------------
// [SECTION] METRICS/DEBUGGER WINDOW
//-----------------------------------------------------------------------------
// - RenderViewportThumbnail() [Internal]
// - RenderViewportsThumbnails() [Internal]
// - DebugTextEncoding()
// - MetricsHelpMarker() [Internal]
// - ShowFontAtlas() [Internal]
// - ShowMetricsWindow()
// - DebugNodeColumns() [Internal]
// - DebugNodeDockNode() [Internal]
// - DebugNodeDrawList() [Internal]
// - DebugNodeDrawCmdShowMeshAndBoundingBox() [Internal]
// - DebugNodeFont() [Internal]
// - DebugNodeFontGlyph() [Internal]
// - DebugNodeStorage() [Internal]
// - DebugNodeTabBar() [Internal]
// - DebugNodeViewport() [Internal]
// - DebugNodeWindow() [Internal]
// - DebugNodeWindowSettings() [Internal]
// - DebugNodeWindowsList() [Internal]
// - DebugNodeWindowsListByBeginStackParent() [Internal]
//-----------------------------------------------------------------------------

// #ifndef IMGUI_DISABLE_DEBUG_TOOLS

c_void DebugRenderViewportThumbnail(draw_list: *mut ImDrawList, *mut ImGuiViewportP viewport, bb: &ImRect)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;

    let scale: ImVec2 = bb.GetSize() / viewport.Size;
    let off: ImVec2 = bb.Min - viewport.Pos * scale;
    let alpha_mul: c_float =  (viewport.Flags & ImGuiViewportFlags_Minimized) ? 0.3f32 : 1.0;
    window.DrawList.AddRectFilled(bb.Min, bb.Max, GetColorU32(ImGuiCol_Border, alpha_mul * 0.400f32));
    for (let i: c_int = 0; i != g.Windows.len(); i++)
    {
        let mut thumb_window: *mut ImGuiWindow =  g.Windows[i];
        if (!thumb_window.WasActive || (thumb_window.Flags & ImGuiWindowFlags_ChildWindow))
            continue;
        if (thumb_window.Viewport != viewport)
            continue;

        let thumb_r: ImRect =  thumb_window.Rect();
        let title_r: ImRect =  thumb_window.TitleBarRect();
        thumb_r = ImRect(ImFloor(off + thumb_r.Min * scale), ImFloor(off +  thumb_r.Max * scale));
        title_r = ImRect(ImFloor(off + title_r.Min * scale), ImFloor(off +  ImVec2::new(title_r.Max.x, title_r.Min.y) * scale) + ImVec2::new(0,5)); // Exaggerate title bar height
        thumb_r.ClipWithFull(bb);
        title_r.ClipWithFull(bb);
        let window_is_focused: bool = (g.NavWindow && thumb_window.RootWindowForTitleBarHighlight == g.NavWindow.RootWindowForTitleBarHighlight);
        window.DrawList.AddRectFilled(thumb_r.Min, thumb_r.Max, GetColorU32(ImGuiCol_WindowBg, alpha_mul));
        window.DrawList.AddRectFilled(title_r.Min, title_r.Max, GetColorU32(window_is_focused ? ImGuiCol_TitleBgActive : ImGuiCol_TitleBg, alpha_mul));
        window.DrawList.AddRect(thumb_r.Min, thumb_r.Max, GetColorU32(ImGuiCol_Border, alpha_mul));
        window.DrawList.AddText(g.Font, g.FontSize * 1.0, title_r.Min, GetColorU32(ImGuiCol_Text, alpha_mul), thumb_window.Name, FindRenderedTextEnd(thumb_window.Name));
    }
    draw_list.AddRect(bb.Min, bb.Max, GetColorU32(ImGuiCol_Border, alpha_mul));
}

static c_void RenderViewportsThumbnails()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;

    // We don't display full monitor bounds (we could, but it often looks awkward), instead we display just enough to cover all of our viewports.
    let SCALE: c_float =  1.0 / 8.0.0;
    let mut bb_full: ImRect = ImRect::new(f32::MAX, f32::MAX, -f32::MAX, -f32::MAX);
    for (let n: c_int = 0; n < g.Viewports.len(); n++)
        bb_full.Add(g.Viewports[n].GetMainRect());
    let p: ImVec2 = window.DC.CursorPos;
    let off: ImVec2 = p - bb_full.Min * SCALE;
    for (let n: c_int = 0; n < g.Viewports.len(); n++)
    {
        let mut viewport: *mut ImGuiViewport =  g.Viewports[n];
        let mut viewport_draw_bb: ImRect = ImRect::new(off + (viewport.Pos) * SCALE, off + (viewport.Pos + viewport.Size) * SCALE);
        DebugRenderViewportThumbnail(window.DrawList, viewport, viewport_draw_bb);
    }
    Dummy(bb_full.GetSize() * SCALE);
}

static IMGUI_CDECL: c_int ViewportComparerByFrontMostStampCount(*const c_void lhs, *const c_void rhs)
{
    let mut a: *mut ImGuiViewport =  *(const *mut ImGuiViewportP const*)lhs;
    let mut b: *mut ImGuiViewport =  *(const *mut ImGuiViewportP const*)rhs;
    return b->LastFrontMostStampCount - a->LastFrontMostStampCount;
}

// Helper tool to diagnose between text encoding issues and font loading issues. Pass your UTF-8 string and verify that there are correct.
c_void DebugTextEncoding(str: *const c_char)
{
    Text("Text: \"%s\"", str);
    if (!BeginTable("list", 4, ImGuiTableFlags_Borders | ImGuiTableFlags_RowBg | ImGuiTableFlags_SizingFixedFit))
        return;
    TableSetupColumn("Offset");
    TableSetupColumn("UTF-8");
    TableSetupColumn("Glyph");
    TableSetupColumn("Codepoint");
    TableHeadersRow();
    for (p: *const c_char = str; *p != 0; )
    {
        c: c_uint;
        let c_utf8_len: c_int = ImTextCharFromUtf8(&c, p, null_mut());
        TableNextColumn();
        Text("%d", (p - str));
        TableNextColumn();
        for (let byte_index: c_int = 0; byte_index < c_utf8_len; byte_index++)
        {
            if (byte_index > 0)
                SameLine();
            Text("0x%02X", (c_uchar)p[byte_index]);
        }
        TableNextColumn();
        if (GetFont()->FindGlyphNoFallback(c))
            TextUnformatted(p, p + c_utf8_len);
        else
            TextUnformatted((c == IM_UNICODE_CODEPOINT_INVALID) ? "[invalid]" : "[missing]");
        TableNextColumn();
        Text("U+%04X", c);
        p += c_utf8_len;
    }
    EndTable();
}

// Avoid naming collision with imgui_demo.cpp's HelpMarker() for unity builds.
static c_void MetricsHelpMarker(desc: *const c_char)
{
    TextDisabled("(?)");
    if (IsItemHovered(ImGuiHoveredFlags_DelayShort))
    {
        BeginTooltip();
        PushTextWrapPos(GetFontSize() * 35.00f32);
        TextUnformatted(desc);
        PopTextWrapPos();
        EndTooltip();
    }
}

// [DEBUG] List fonts in a font atlas and display its texture
c_void ShowFontAtlas(atlas: *mut ImFontAtlas)
{
    for (let i: c_int = 0; i < atlas->Fonts.Size; i++)
    {
        font: *mut ImFont = atlas->Fonts[i];
        PushID(font);
        DebugNodeFont(font);
        PopID();
    }
    if (TreeNode("Atlas texture", "Atlas texture (%dx%d pixels)", atlas->TexWidth, atlas->TexHeight))
    {
        tint_col: ImVec4 = ImVec4(1.0, 1.0, 1.0, 1.0);
        border_col: ImVec4 = ImVec4(1.0, 1.0, 1.0, 0.5f32);
        Image(atlas->TexID, ImVec2::new(atlas->TexWidth, atlas->TexHeight), ImVec2::new(0.0, 0.0), ImVec2::new(1.0, 1.0), tint_col, border_col);
        TreePop();
    }
}

c_void ShowMetricsWindow(bool* p_open)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiIO& io = g.IO;
    ImGuiMetricsConfig* cfg = &g.DebugMetricsConfig;
    if (cfg->ShowDebugLog)
        ShowDebugLogWindow(&cfg->ShowDebugLog);
    if (cfg->ShowStackTool)
        ShowStackToolWindow(&cfg->ShowStackTool);

    if (!Begin("Dear ImGui Metrics/Debugger", p_open) || GetCurrentWindow()->BeginCount > 1)
    {
        End();
        return;
    }

    // Basic info
    Text("Dear ImGui %s", GetVersion());
    Text("Application average %.3f ms/frame (%.1f FPS)", 1000f32 / io.Framerate, io.Framerate);
    Text("%d vertices, %d indices (%d triangles)", io.MetricsRenderVertices, io.MetricsRenderIndices, io.MetricsRenderIndices / 3);
    Text("%d visible windows, %d active allocations", io.MetricsRenderWindows, io.MetricsActiveAllocations);
    //SameLine(); if (SmallButton("GC")) { g.GcCompactAll = true; }

    Separator();

    // Debugging enums
    enum { WRT_OuterRect, WRT_OuterRectClipped, WRT_InnerRect, WRT_InnerClipRect, WRT_WorkRect, WRT_Content, WRT_ContentIdeal, WRT_ContentRegionRect, WRT_Count }; // Windows Rect Type
    wrt_rects_names: *const c_char[WRT_Count] = { "OuterRect", "OuterRectClipped", "InnerRect", "InnerClipRect", "WorkRect", "Content", "ContentIdeal", "ContentRegionRect" };
    enum { TRT_OuterRect, TRT_InnerRect, TRT_WorkRect, TRT_HostClipRect, TRT_InnerClipRect, TRT_BackgroundClipRect, TRT_ColumnsRect, TRT_ColumnsWorkRect, TRT_ColumnsClipRect, TRT_ColumnsContentHeadersUsed, TRT_ColumnsContentHeadersIdeal, TRT_ColumnsContentFrozen, TRT_ColumnsContentUnfrozen, TRT_Count }; // Tables Rect Type
    trt_rects_names: *const c_char[TRT_Count] = { "OuterRect", "InnerRect", "WorkRect", "HostClipRect", "InnerClipRect", "BackgroundClipRect", "ColumnsRect", "ColumnsWorkRect", "ColumnsClipRect", "ColumnsContentHeadersUsed", "ColumnsContentHeadersIdeal", "ColumnsContentFrozen", "ColumnsContentUnfrozen" };
    if (cfg->ShowWindowsRectsType < 0)
        cfg->ShowWindowsRectsType = WRT_WorkRect;
    if (cfg->ShowTablesRectsType < 0)
        cfg->ShowTablesRectsType = TRT_WorkRect;

    struct Funcs
    {
        static ImRect GetTableRect(ImGuiTable* table, rect_type: c_int, n: c_int)
        {
            ImGuiTableInstanceData* table_instance = TableGetInstanceData(table, table.InstanceCurrent); // Always using last submitted instance
            if (rect_type == TRT_OuterRect)                     { return table.OuterRect; }
            else if (rect_type == TRT_InnerRect)                { return table.InnerRect; }
            else if (rect_type == TRT_WorkRect)                 { return table.WorkRect; }
            else if (rect_type == TRT_HostClipRect)             { return table.HostClipRect; }
            else if (rect_type == TRT_InnerClipRect)            { return table.InnerClipRect; }
            else if (rect_type == TRT_BackgroundClipRect)       { return table.BgClipRect; }
            else if (rect_type == TRT_ColumnsRect)              { ImGuiTableColumn* c = &table.Columns[n]; return ImRect(c->MinX, table.InnerClipRect.Min.y, c->MaxX, table.InnerClipRect.Min.y + table_instance.LastOuterHeight); }
            else if (rect_type == TRT_ColumnsWorkRect)          { ImGuiTableColumn* c = &table.Columns[n]; return ImRect(c->WorkMinX, table.WorkRect.Min.y, c->WorkMaxX, table.WorkRect.Max.y); }
            else if (rect_type == TRT_ColumnsClipRect)          { ImGuiTableColumn* c = &table.Columns[n]; return c->ClipRect; }
            else if (rect_type == TRT_ColumnsContentHeadersUsed){ ImGuiTableColumn* c = &table.Columns[n]; return ImRect(c->WorkMinX, table.InnerClipRect.Min.y, c->ContentMaxXHeadersUsed, table.InnerClipRect.Min.y + table_instance.LastFirstRowHeight); } // Note: y1/y2 not always accurate
            else if (rect_type == TRT_ColumnsContentHeadersIdeal){ImGuiTableColumn* c = &table.Columns[n]; return ImRect(c->WorkMinX, table.InnerClipRect.Min.y, c->ContentMaxXHeadersIdeal, table.InnerClipRect.Min.y + table_instance.LastFirstRowHeight); }
            else if (rect_type == TRT_ColumnsContentFrozen)     { ImGuiTableColumn* c = &table.Columns[n]; return ImRect(c->WorkMinX, table.InnerClipRect.Min.y, c->ContentMaxXFrozen, table.InnerClipRect.Min.y + table_instance.LastFirstRowHeight); }
            else if (rect_type == TRT_ColumnsContentUnfrozen)   { ImGuiTableColumn* c = &table.Columns[n]; return ImRect(c->WorkMinX, table.InnerClipRect.Min.y + table_instance.LastFirstRowHeight, c->ContentMaxXUnfrozen, table.InnerClipRect.Max.y); }
            // IM_ASSERT(0);
            return ImRect();
        }

        static ImRect GetWindowRect(window: *mut ImGuiWindow, rect_type: c_int)
        {
            if (rect_type == WRT_OuterRect)                 { return window.Rect(); }
            else if (rect_type == WRT_OuterRectClipped)     { return window.OuterRectClipped; }
            else if (rect_type == WRT_InnerRect)            { return window.InnerRect; }
            else if (rect_type == WRT_InnerClipRect)        { return window.InnerClipRect; }
            else if (rect_type == WRT_WorkRect)             { return window.WorkRect; }
            else if (rect_type == WRT_Content)       { let mut min: ImVec2 =  window.InnerRect.Min - window.Scroll + window.WindowPadding; return ImRect(min, min + window.ContentSize); }
            else if (rect_type == WRT_ContentIdeal)         { let mut min: ImVec2 =  window.InnerRect.Min - window.Scroll + window.WindowPadding; return ImRect(min, min + window.ContentSizeIdeal); }
            else if (rect_type == WRT_ContentRegionRect)    { return window.ContentRegionRect; }
            // IM_ASSERT(0);
            return ImRect();
        }
    };

    // Tools
    if (TreeNode("Tools"))
    {
        let mut show_encoding_viewer: bool =  TreeNode("UTF-8 Encoding viewer");
        SameLine();
        MetricsHelpMarker("You can also call DebugTextEncoding() from your code with a given string to test that your UTF-8 encoding settings are correct.");
        if (show_encoding_viewer)
        {
            static buf: [c_char;100] = "";
            SetNextItemWidth(-FLT_MIN);
            InputText("##Text", buf, bu0f32.len());
            if (buf[0] != 0)
                DebugTextEncoding(bu0f32);
            TreePop();
        }

        // The Item Picker tool is super useful to visually select an item and break into the call-stack of where it was submitted.
        if (Checkbox("Show Item Picker", &g.DebugItemPickerActive) && g.DebugItemPickerActive)
            DebugStartItemPicker();
        SameLine();
        MetricsHelpMarker("Will call the IM_DEBUG_BREAK() macro to break in debugger.\nWarning: If you don't have a debugger attached, this will probably crash.");

        // Stack Tool is your best friend!
        Checkbox("Show Debug Log", &cfg->ShowDebugLog);
        SameLine();
        MetricsHelpMarker("You can also call ShowDebugLogWindow() from your code.");

        // Stack Tool is your best friend!
        Checkbox("Show Stack Tool", &cfg->ShowStackTool);
        SameLine();
        MetricsHelpMarker("You can also call ShowStackToolWindow() from your code.");

        Checkbox("Show windows begin order", &cfg->ShowWindowsBeginOrder);
        Checkbox("Show windows rectangles", &cfg->ShowWindowsRects);
        SameLine();
        SetNextItemWidth(GetFontSize() * 12);
        cfg->ShowWindowsRects |= Combo("##show_windows_rect_type", &cfg->ShowWindowsRectsType, wrt_rects_names, WRT_Count, WRT_Count);
        if (cfg->ShowWindowsRects && g.NavWindow != null_mut())
        {
            BulletText("'%s':", g.NavWindow.Name);
            Indent();
            for (let rect_n: c_int = 0; rect_n < WRT_Count; rect_n++)
            {
                let r: ImRect =  Funcs::GetWindowRect(g.NavWindow, rect_n);
                Text("(%6.1f,%6.10f32) (%6.1f,%6.10f32) Size (%6.1f,%6.10f32) %s", r.Min.x, r.Min.y, r.Max.x, r.Max.y, r.GetWidth(), r.GetHeight(), wrt_rects_names[rect_n]);
            }
            Unindent();
        }

        Checkbox("Show tables rectangles", &cfg->ShowTablesRects);
        SameLine();
        SetNextItemWidth(GetFontSize() * 12);
        cfg->ShowTablesRects |= Combo("##show_table_rects_type", &cfg->ShowTablesRectsType, trt_rects_names, TRT_Count, TRT_Count);
        if (cfg->ShowTablesRects && g.NavWindow != null_mut())
        {
            for (let table_n: c_int = 0; table_n < g.Tables.GetMapSize(); table_n++)
            {
                ImGuiTable* table = g.Tables.TryGetMapData(table_n);
                if (table == null_mut() || table.LastFrameActive < g.FrameCount - 1 || (table.OuterWindow != g.NavWindow && table.InnerWindow != g.NavWindow))
                    continue;

                BulletText("Table 0x%08X (%d columns, in '%s')", table.ID, table.ColumnsCount, table.Outerwindow.Name);
                if (IsItemHovered())
                    GetForegroundDrawList()->AddRect(table.OuterRect.Min - ImVec2::new(1, 1), table.OuterRect.Max + ImVec2::new(1, 1), IM_COL32(255, 255, 0, 255), 0.0, 0, 2.00f32);
                Indent();
                buf: [c_char;128];
                for (let rect_n: c_int = 0; rect_n < TRT_Count; rect_n++)
                {
                    if (rect_n >= TRT_ColumnsRect)
                    {
                        if (rect_n != TRT_ColumnsRect && rect_n != TRT_ColumnsClipRect)
                            continue;
                        for (let column_n: c_int = 0; column_n < table.ColumnsCount; column_n++)
                        {
                            let r: ImRect =  Funcs::GetTableRect(table, rect_n, column_n);
                            ImFormatString(buf, bu0f32.len(), "(%6.1f,%6.10f32) (%6.1f,%6.10f32) Size (%6.1f,%6.10f32) Col %d %s", r.Min.x, r.Min.y, r.Max.x, r.Max.y, r.GetWidth(), r.GetHeight(), column_n, trt_rects_names[rect_n]);
                            Selectable(bu0f32);
                            if (IsItemHovered())
                                GetForegroundDrawList()->AddRect(r.Min - ImVec2::new(1, 1), r.Max + ImVec2::new(1, 1), IM_COL32(255, 255, 0, 255), 0.0, 0, 2.00f32);
                        }
                    }
                    else
                    {
                        let r: ImRect =  Funcs::GetTableRect(table, rect_n, -1);
                        ImFormatString(buf, bu0f32.len(), "(%6.1f,%6.10f32) (%6.1f,%6.10f32) Size (%6.1f,%6.10f32) %s", r.Min.x, r.Min.y, r.Max.x, r.Max.y, r.GetWidth(), r.GetHeight(), trt_rects_names[rect_n]);
                        Selectable(bu0f32);
                        if (IsItemHovered())
                            GetForegroundDrawList()->AddRect(r.Min - ImVec2::new(1, 1), r.Max + ImVec2::new(1, 1), IM_COL32(255, 255, 0, 255), 0.0, 0, 2.00f32);
                    }
                }
                Unindent();
            }
        }

        TreePop();
    }

    // Windows
    if (TreeNode("Windows", "Windows (%d)", g.Windows.len()))
    {
        //SetNextItemOpen(true, ImGuiCond_Once);
        DebugNodeWindowsList(&g.Windows, "By display order");
        DebugNodeWindowsList(&g.WindowsFocusOrder, "By focus order (root windows)");
        if (TreeNode("By submission order (begin stack)"))
        {
            // Here we display windows in their submitted order/hierarchy, however note that the Begin stack doesn't constitute a Parent<>Child relationship!
            Vec<ImGuiWindow*>& temp_buffer = g.WindowsTempSortBuffer;
            temp_buffer.clear();
            for (let i: c_int = 0; i < g.Windows.len(); i++)
                if (g.Windows[i]->LastFrameActive + 1 >= g.FrameCount)
                    temp_buffer.push(g.Windows[i]);
            struct Func { static IMGUI_CDECL: c_int WindowComparerByBeginOrder(*const c_void lhs, *const c_void rhs) { return ((*(*const ImGuiWindow const *)lhs)->BeginOrderWithinContext - (*(*const ImGuiWindow const*)rhs)->BeginOrderWithinContext); } };
            ImQsort(temp_buffer.Data, temp_buffer.Size, sizeof, Func::WindowComparerByBeginOrder);
            DebugNodeWindowsListByBeginStackParent(temp_buffer.Data, temp_buffer.Size, null_mut());
            TreePop();
        }

        TreePop();
    }

    // DrawLists
    let drawlist_count: c_int = 0;
    for (let viewport_i: c_int = 0; viewport_i < g.Viewports.len(); viewport_i++)
        drawlist_count += g.Viewports[viewport_i]->DrawDataBuilder.GetDrawListCount();
    if (TreeNode("DrawLists", "DrawLists (%d)", drawlist_count))
    {
        Checkbox("Show ImDrawCmd mesh when hovering", &cfg->ShowDrawCmdMesh);
        Checkbox("Show ImDrawCmd bounding boxes when hovering", &cfg->ShowDrawCmdBoundingBoxes);
        for (let viewport_i: c_int = 0; viewport_i < g.Viewports.len(); viewport_i++)
        {
            let mut viewport: *mut ImGuiViewport =  g.Viewports[viewport_i];
            let mut viewport_has_drawlist: bool =  false;
            for (let layer_i: c_int = 0; layer_i < IM_ARRAYSIZE(viewport.DrawDataBuilder.Layers); layer_i++)
                for (let draw_list_i: c_int = 0; draw_list_i < viewport.DrawDataBuilder.Layers[layer_i].Size; draw_list_i++)
                {
                    if (!viewport_has_drawlist)
                        Text("Active DrawLists in Viewport #%d, ID: 0x%08X", viewport.Idx, viewport.ID);
                    viewport_has_drawlist = true;
                    DebugNodeDrawList(null_mut(), viewport, viewport.DrawDataBuilder.Layers[layer_i][draw_list_i], "DrawList");
                }
        }
        TreePop();
    }

    // Viewports
    if (TreeNode("Viewports", "Viewports (%d)", g.Viewports.len()))
    {
        Indent(GetTreeNodeToLabelSpacing());
        RenderViewportsThumbnails();
        Unindent(GetTreeNodeToLabelSpacing());

        let mut open: bool =  TreeNode("Monitors", "Monitors (%d)", g.PlatformIO.Monitors.Size);
        SameLine();
        MetricsHelpMarker("Dear ImGui uses monitor data:\n- to query DPI settings on a per monitor basis\n- to position popup/tooltips so they don't straddle monitors.");
        if (open)
        {
            for (let i: c_int = 0; i < g.PlatformIO.Monitors.Size; i++)
            {
                const ImGuiPlatformMonitor& mon = g.PlatformIO.Monitors[i];
                BulletText("Monitor #%d: DPI %.0f%%\n MainMin (%.0.0,%.00f32), MainMax (%.0.0,%.00f32), MainSize (%.0.0,%.00f32)\n WorkMin (%.0.0,%.00f32), WorkMax (%.0.0,%.00f32), WorkSize (%.0.0,%.00f32)",
                    i, mon.DpiScale * 100f32,
                    mon.MainPos.x, mon.MainPos.y, mon.MainPos.x + mon.MainSize.x, mon.MainPos.y + mon.MainSize.y, mon.MainSize.x, mon.MainSize.y,
                    mon.WorkPos.x, mon.WorkPos.y, mon.WorkPos.x + mon.WorkSize.x, mon.WorkPos.y + mon.WorkSize.y, mon.WorkSize.x, mon.WorkSize.y);
            }
            TreePop();
        }

        BulletText("MouseViewport: 0x%08X (UserHovered 0x%08X, LastHovered 0x%08X)", g.MouseViewport ? g.MouseViewport->ID : 0, g.IO.MouseHoveredViewport, g.MouseLastHoveredViewport ? g.MouseLastHoveredViewport->ID : 0);
        if (TreeNode("Inferred Z order (front-to-back)"))
        {
            static Vec<*mut ImGuiViewportP> viewports;
            viewports.resize(g.Viewports.len());
            memcpy(viewports.Data, g.Viewports.Data, g.Viewports.size_in_bytes());
            if (viewports.Size > 1)
                ImQsort(viewports.Data, viewports.Size, sizeof(ImGuiViewport*), ViewportComparerByFrontMostStampCount);
            for (let i: c_int = 0; i < viewports.Size; i++)
                BulletText("Viewport #%d, ID: 0x%08X, FrontMostStampCount = %08d, Window: \"%s\"", viewports[i]->Idx, viewports[i]->ID, viewports[i]->LastFrontMostStampCount, viewports[i]->Window ? viewports[i]->window.Name : "N/A");
            TreePop();
        }

        for (let i: c_int = 0; i < g.Viewports.len(); i++)
            DebugNodeViewport(g.Viewports[i]);
        TreePop();
    }

    // Details for Popups
    if (TreeNode("Popups", "Popups (%d)", g.OpenPopupStack.Size))
    {
        for (let i: c_int = 0; i < g.OpenPopupStack.Size; i++)
        {
            // As it's difficult to interact with tree nodes while popups are open, we display everything inline.
            let popup_data: *const ImGuiPopupData = &g.OpenPopupStack[i];
            let mut window: *mut ImGuiWindow =  popup_Data.Window;
            BulletText("PopupID: %08x, Window: '%s' (%s%s), BackupNavWindow '%s', ParentWindow '%s'",
                popup_Data.PopupId, window ? window.Name : "NULL", window && (window.Flags & ImGuiWindowFlags_ChildWindow) ? "Child;" : "", window && (window.Flags & ImGuiWindowFlags_ChildMenu) ? "Menu;" : "",
                popup_Data.BackupNavWindow ? popup_Data.BackupNavwindow.Name : "NULL", window && window.ParentWindow ? window.Parentwindow.Name : "NULL");
        }
        TreePop();
    }

    // Details for TabBars
    if (TreeNode("TabBars", "Tab Bars (%d)", g.TabBars.GetAliveCount()))
    {
        for (let n: c_int = 0; n < g.TabBars.GetMapSize(); n++)
            if (ImGuiTabBar* tab_bar = g.TabBars.TryGetMapData(n))
            {
                PushID(tab_bar);
                DebugNodeTabBar(tab_bar, "TabBar");
                PopID();
            }
        TreePop();
    }

    // Details for Tables
    if (TreeNode("Tables", "Tables (%d)", g.Tables.GetAliveCount()))
    {
        for (let n: c_int = 0; n < g.Tables.GetMapSize(); n++)
            if (ImGuiTable* table = g.Tables.TryGetMapData(n))
                DebugNodeTable(table);
        TreePop();
    }

    // Details for Fonts
    atlas: *mut ImFontAtlas = g.IO.Fonts;
    if (TreeNode("Fonts", "Fonts (%d)", atlas->Fonts.Size))
    {
        ShowFontAtlas(atlas);
        TreePop();
    }

    // Details for InputText
    if (TreeNode("InputText"))
    {
        DebugNodeInputTextState(&g.InputTextState);
        TreePop();
    }

    // Details for Docking
// #ifdef IMGUI_HAS_DOCK
    if (TreeNode("Docking"))
    {
        static let mut root_nodes_only: bool =  true;
        ImGuiDockContext* dc = &g.DockContext;
        Checkbox("List root nodes", &root_nodes_only);
        Checkbox("Ctrl shows window dock info", &cfg->ShowDockingNodes);
        if (SmallButton("Clear nodes")) { DockContextClearNodes(&g, 0, true); }
        SameLine();
        if (SmallButton("Rebuild all")) { dc->WantFullRebuild = true; }
        for (let n: c_int = 0; n < dc->Nodes.Data.Size; n++)
            if (ImGuiDockNode* node = (ImGuiDockNode*)dc->Nodes.Data[n].val_p)
                if (!root_nodes_only || node.IsRootNode())
                    DebugNodeDockNode(node, "Node");
        TreePop();
    }
// #endif // #ifdef IMGUI_HAS_DOCK

    // Settings
    if (TreeNode("Settings"))
    {
        if (SmallButton("Clear"))
            ClearIniSettings();
        SameLine();
        if (SmallButton("Save to memory"))
            SaveIniSettingsToMemory();
        SameLine();
        if (SmallButton("Save to disk"))
            SaveIniSettingsToDisk(g.IO.IniFilename);
        SameLine();
        if (g.IO.IniFilename)
            Text("\"%s\"", g.IO.IniFilename);
        else
            TextUnformatted("<NULL>");
        Text("SettingsDirtyTimer %.2f", g.SettingsDirtyTimer);
        if (TreeNode("SettingsHandlers", "Settings handlers: (%d)", g.SettingsHandlers.Size))
        {
            for (let n: c_int = 0; n < g.SettingsHandlers.Size; n++)
                BulletText("%s", g.SettingsHandlers[n].TypeName);
            TreePop();
        }
        if (TreeNode("SettingsWindows", "Settings packed data: Windows: %d bytes", g.SettingsWindows.size()))
        {
            for (settings: *mut ImGuiWindowSettings = g.SettingsWindows.begin(); settings != null_mut(); settings = g.SettingsWindows.next_chunk(settings))
                DebugNodeWindowSettings(settings);
            TreePop();
        }

        if (TreeNode("SettingsTables", "Settings packed data: Tables: %d bytes", g.SettingsTables.size()))
        {
            for (ImGuiTableSettings* settings = g.SettingsTables.begin(); settings != null_mut(); settings = g.SettingsTables.next_chunk(settings))
                DebugNodeTableSettings(settings);
            TreePop();
        }

// #ifdef IMGUI_HAS_DOCK
        if (TreeNode("SettingsDocking", "Settings packed data: Docking"))
        {
            ImGuiDockContext* dc = &g.DockContext;
            Text("In SettingsWindows:");
            for (settings: *mut ImGuiWindowSettings = g.SettingsWindows.begin(); settings != null_mut(); settings = g.SettingsWindows.next_chunk(settings))
                if (settings->DockId != 0)
                    BulletText("Window '%s' -> DockId %08X", settings->GetName(), settings->DockId);
            Text("In SettingsNodes:");
            for (let n: c_int = 0; n < dc->NodesSettings.Size; n++)
            {
                ImGuiDockNodeSettings* settings = &dc->NodesSettings[n];
                let mut  selected_tab_name: *const c_char= null_mut();
                if (settings->SelectedTabId)
                {
                    if (let mut window: *mut ImGuiWindow =  FindWindowByID(settings->SelectedTabId))
                        selected_tab_name = window.Name;
                    else if (window_settings: *mut ImGuiWindowSettings = FindWindowSettings(settings->SelectedTabId))
                        selected_tab_name = window_settings->GetName();
                }
                BulletText("Node %08X, Parent %08X, SelectedTab %08X ('%s')", settings->ID, settings->ParentNodeId, settings->SelectedTabId, selected_tab_name ? selected_tab_name : settings->SelectedTabId ? "N/A" : "");
            }
            TreePop();
        }
// #endif // #ifdef IMGUI_HAS_DOCK

        if (TreeNode("SettingsIniData", "Settings unpacked data (.ini): %d bytes", g.SettingsIniData.size()))
        {
            InputTextMultiline("##Ini", (char*)(*mut c_void)g.SettingsIniData.c_str(), g.SettingsIniData.Buf.Size, ImVec2::new(-FLT_MIN, GetTextLineHeight() * 20), ImGuiInputTextFlags_ReadOnly);
            TreePop();
        }
        TreePop();
    }

    // Misc Details
    if (TreeNode("Internal state"))
    {
        Text("WINDOWING");
        Indent();
        Text("HoveredWindow: '%s'", g.HoveredWindow ? g.Hoveredwindow.Name : "NULL");
        Text("Hoveredwindow.Root: '%s'", g.HoveredWindow ? g.Hoveredwindow.RootWindowDockTree.Name : "NULL");
        Text("HoveredWindowUnderMovingWindow: '%s'", g.HoveredWindowUnderMovingWindow ? g.HoveredWindowUnderMovingwindow.Name : "NULL");
        Text("HoveredDockNode: 0x%08X", g.DebugHoveredDockNode ? g.DebugHoveredDockNode.ID : 0);
        Text("MovingWindow: '%s'", g.MovingWindow ? g.Movingwindow.Name : "NULL");
        Text("MouseViewport: 0x%08X (UserHovered 0x%08X, LastHovered 0x%08X)", g.MouseViewport->ID, g.IO.MouseHoveredViewport, g.MouseLastHoveredViewport ? g.MouseLastHoveredViewport->ID : 0);
        Unindent();

        Text("ITEMS");
        Indent();
        Text("ActiveId: 0x%08X/0x%08X (%.2f sec), AllowOverlap: %d, Source: %s", g.ActiveId, g.ActiveIdPreviousFrame, g.ActiveIdTimer, g.ActiveIdAllowOverlap, GetInputSourceName(g.ActiveIdSource));
        Text("ActiveIdWindow: '%s'", g.ActiveIdWindow ? g.ActiveIdwindow.Name : "NULL");

        let active_id_using_key_input_count: c_int = 0;
        for (let n: c_int = ImGuiKey_NamedKey_BEGIN; n < ImGuiKey_NamedKey_END; n++)
            active_id_using_key_input_count += g.ActiveIdUsingKeyInputMask[n] ? 1 : 0;
        Text("ActiveIdUsing: NavDirMask: %X, KeyInputMask: %d key(s)", g.ActiveIdUsingNavDirMask, active_id_using_key_input_count);
        Text("HoveredId: 0x%08X (%.2f sec), AllowOverlap: %d", g.HoveredIdPreviousFrame, g.HoveredIdTimer, g.HoveredIdAllowOverlap); // Not displaying g.HoveredId as it is update mid-frame
        Text("HoverDelayId: 0x%08X, Timer: %.2f, ClearTimer: %.2f", g.HoverDelayId, g.HoverDelayTimer, g.HoverDelayClearTimer);
        Text("DragDrop: %d, SourceId = 0x%08X, Payload \"%s\" (%d bytes)", g.DragDropActive, g.DragDropPayload.SourceId, g.DragDropPayload.DataType, g.DragDropPayload.DataSize);
        Unindent();

        Text("NAV,FOCUS");
        Indent();
        Text("NavWindow: '%s'", g.NavWindow ? g.NavWindow.Name : "NULL");
        Text("NavId: 0x%08X, NavLayer: %d", g.NavId, g.NavLayer);
        Text("NavInputSource: %s", GetInputSourceName(g.NavInputSource));
        Text("NavActive: %d, NavVisible: %d", g.IO.NavActive, g.IO.NavVisible);
        Text("NavActivateId/DownId/PressedId/InputId: %08X/%08X/%08X/%08X", g.NavActivateId, g.NavActivateDownId, g.NavActivatePressedId, g.NavActivateInputId);
        Text("NavActivateFlags: %04X", g.NavActivateFlags);
        Text("NavDisableHighlight: %d, NavDisableMouseHover: %d", g.NavDisableHighlight, g.NavDisableMouseHover);
        Text("NavFocusScopeId = 0x%08X", g.NavFocusScopeId);
        Text("NavWindowingTarget: '%s'", g.NavWindowingTarget ? g.NavWindowingTarget->Name : "NULL");
        Unindent();

        TreePop();
    }

    // Overlay: Display windows Rectangles and Begin Order
    if (cfg->ShowWindowsRects || cfg->ShowWindowsBeginOrder)
    {
        for (let n: c_int = 0; n < g.Windows.len(); n++)
        {
            let mut window: *mut ImGuiWindow =  g.Windows[n];
            if (!window.WasActive)
                continue;
            let mut  draw_list: *mut ImDrawList =  GetForegroundDrawList(window);
            if (cfg->ShowWindowsRects)
            {
                let r: ImRect =  Funcs::GetWindowRect(window, cfg->ShowWindowsRectsType);
                draw_list.AddRect(r.Min, r.Max, IM_COL32(255, 0, 128, 255));
            }
            if (cfg->ShowWindowsBeginOrder && !(window.Flags & ImGuiWindowFlags_ChildWindow))
            {
                buf: [c_char;32];
                ImFormatString(buf, bu0f32.len(), "%d", window.BeginOrderWithinContext);
                let font_size: c_float =  GetFontSize();
                draw_list.AddRectFilled(window.Pos, window.Pos + ImVec2::new(font_size, font_size), IM_COL32(200, 100, 100, 255));
                draw_list.AddText(window.Pos, IM_COL32(255, 255, 255, 255), bu0f32);
            }
        }
    }

    // Overlay: Display Tables Rectangles
    if (cfg->ShowTablesRects)
    {
        for (let table_n: c_int = 0; table_n < g.Tables.GetMapSize(); table_n++)
        {
            ImGuiTable* table = g.Tables.TryGetMapData(table_n);
            if (table == null_mut() || table.LastFrameActive < g.FrameCount - 1)
                continue;
            let mut  draw_list: *mut ImDrawList =  GetForegroundDrawList(table.OuterWindow);
            if (cfg->ShowTablesRectsType >= TRT_ColumnsRect)
            {
                for (let column_n: c_int = 0; column_n < table.ColumnsCount; column_n++)
                {
                    let r: ImRect =  Funcs::GetTableRect(table, cfg->ShowTablesRectsType, column_n);
                    col: u32 = (table.HoveredColumnBody == column_n) ? IM_COL32(255, 255, 128, 255) : IM_COL32(255, 0, 128, 255);
                    let thickness: c_float =  (table.HoveredColumnBody == column_n) ? 3.0.0 : 1.0;
                    draw_list.AddRect(r.Min, r.Max, col, 0.0, 0, thickness);
                }
            }
            else
            {
                let r: ImRect =  Funcs::GetTableRect(table, cfg->ShowTablesRectsType, -1);
                draw_list.AddRect(r.Min, r.Max, IM_COL32(255, 0, 128, 255));
            }
        }
    }

// #ifdef IMGUI_HAS_DOCK
    // Overlay: Display Docking info
    if (cfg->ShowDockingNodes && g.IO.KeyCtrl && g.DebugHoveredDockNode)
    {
        buf: [c_char;64] = "";
        char* p = buf;
        ImGuiDockNode* node = g.DebugHoveredDockNode;
        let mut  overlay_draw_list: *mut ImDrawList =  node.HostWindow ? GetForegroundDrawList(node.HostWindow) : GetForegroundDrawList(GetMainViewport());
        p += ImFormatString(p, buf + bu0f32.len() - p, "DockId: %X%s\n", node.ID, node.IsCentralNode() ? " *CentralNode*" : "");
        p += ImFormatString(p, buf + bu0f32.len() - p, "WindowClass: %08X\n", node.WindowClass.ClassId);
        p += ImFormatString(p, buf + bu0f32.len() - p, "Size: (%.0.0, %.00f32)\n", node.Size.x, node.Size.y);
        p += ImFormatString(p, buf + bu0f32.len() - p, "SizeRef: (%.0.0, %.00f32)\n", node.SizeRef.x, node.SizeRef.y);
        let depth: c_int = DockNodeGetDepth(node);
        overlay_draw_list.AddRect(node.Pos + ImVec2::new(3, 3) * depth, node.Pos + node.Size - ImVec2::new(3, 3) * depth, IM_COL32(200, 100, 100, 255));
        let pos: ImVec2 = node.Pos + ImVec2::new(3, 3) * depth;
        overlay_draw_list.AddRectFilled(pos - ImVec2::new(1, 1), pos + CalcTextSize(bu0f32) + ImVec2::new(1, 1), IM_COL32(200, 100, 100, 255));
        overlay_draw_list.AddText(null_mut(), 0.0, pos, IM_COL32(255, 255, 255, 255), bu0f32);
    }
// #endif // #ifdef IMGUI_HAS_DOCK

    End();
}

// [DEBUG] Display contents of Columns
c_void DebugNodeColumns(ImGuiOldColumns* columns)
{
    if (!TreeNode((*mut c_void)(uintptr_t)columns->ID, "Columns Id: 0x%08X, Count: %d, Flags: 0x%04X", columns->ID, columns->Count, columns->Flags))
        return;
    BulletText("Width: %.1f (MinX: %.1f, MaxX: %.10f32)", columns->OffMaxX - columns->OffMinX, columns->OffMinX, columns->OffMaxX);
    for (let column_n: c_int = 0; column_n < columns->Columns.Size; column_n++)
        BulletText("Column %02d: OffsetNorm %.3f (= %.1f px)", column_n, columns->Columns[column_n].OffsetNorm, GetColumnOffsetFromNorm(columns, columns->Columns[column_n].OffsetNorm));
    TreePop();
}

static c_void DebugNodeDockNodeFlags(ImGuiDockNodeFlags* p_flags, label: *const c_char, enabled: bool)
{
    using namespace ImGui;
    PushID(label);
    PushStyleVar(ImGuiStyleVar_FramePadding, ImVec2::new(0.0, 0.0));
    Text("%s:", label);
    if (!enabled)
        BeginDisabled();
    CheckboxFlags("NoSplit", p_flags, ImGuiDockNodeFlags_NoSplit);
    CheckboxFlags("NoResize", p_flags, ImGuiDockNodeFlags_NoResize);
    CheckboxFlags("NoResizeX", p_flags, ImGuiDockNodeFlags_NoResizeX);
    CheckboxFlags("NoResizeY",p_flags, ImGuiDockNodeFlags_NoResizeY);
    CheckboxFlags("NoTabBar", p_flags, ImGuiDockNodeFlags_NoTabBar);
    CheckboxFlags("HiddenTabBar", p_flags, ImGuiDockNodeFlags_HiddenTabBar);
    CheckboxFlags("NoWindowMenuButton", p_flags, ImGuiDockNodeFlags_NoWindowMenuButton);
    CheckboxFlags("NoCloseButton", p_flags, ImGuiDockNodeFlags_NoCloseButton);
    CheckboxFlags("NoDocking", p_flags, ImGuiDockNodeFlags_NoDocking);
    CheckboxFlags("NoDockingSplitMe", p_flags, ImGuiDockNodeFlags_NoDockingSplitMe);
    CheckboxFlags("NoDockingSplitOther", p_flags, ImGuiDockNodeFlags_NoDockingSplitOther);
    CheckboxFlags("NoDockingOverMe", p_flags, ImGuiDockNodeFlags_NoDockingOverMe);
    CheckboxFlags("NoDockingOverOther", p_flags, ImGuiDockNodeFlags_NoDockingOverOther);
    CheckboxFlags("NoDockingOverEmpty", p_flags, ImGuiDockNodeFlags_NoDockingOverEmpty);
    if (!enabled)
        EndDisabled();
    PopStyleVar();
    PopID();
}

// [DEBUG] Display contents of ImDockNode
c_void DebugNodeDockNode(ImGuiDockNode* node, label: *const c_char)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let is_alive: bool = (g.FrameCount - node.LastFrameAlive < 2);    // Submitted with ImGuiDockNodeFlags_KeepAliveOnly
    let is_active: bool = (g.FrameCount - node.LastFrameActive < 2);  // Submitted
    if (!is_alive) { PushStyleColor(ImGuiCol_Text, GetStyleColorVec4(ImGuiCol_TextDisabled)); }
    open: bool;
    ImGuiTreeNodeFlags tree_node_flags = node.IsFocused ? ImGuiTreeNodeFlags_Selected : ImGuiTreeNodeFlags_None;
    if (node.Windows.len() > 0)
        open = TreeNodeEx((*mut c_void)node.ID, tree_node_flags, "%s 0x%04X%s: %d windows (vis: '%s')", label, node.ID, node.IsVisible ? "" : " (hidden)", node.Windows.len(), node.VisibleWindow ? node.Visiblewindow.Name : "NULL");
    else
        open = TreeNodeEx((*mut c_void)node.ID, tree_node_flags, "%s 0x%04X%s: %s split (vis: '%s')", label, node.ID, node.IsVisible ? "" : " (hidden)", (node.SplitAxis == ImGuiAxis_X) ? "horizontal" : (node.SplitAxis == ImGuiAxis_Y) ? "vertical" : "n/a", node.VisibleWindow ? node.Visiblewindow.Name : "NULL");
    if (!is_alive) { PopStyleColor(); }
    if (is_active && IsItemHovered())
        if (let mut window: *mut ImGuiWindow =  node.HostWindow ? node.HostWindow : node.VisibleWindow)
            GetForegroundDrawList(window)->AddRect(node.Pos, node.Pos + node.Size, IM_COL32(255, 255, 0, 255));
    if (open)
    {
        // IM_ASSERT(node->ChildNodes[0] == NULL || node->ChildNodes[0]->ParentNode == node);
        // IM_ASSERT(node->ChildNodes[1] == NULL || node->ChildNodes[1]->ParentNode == node);
        BulletText("Pos (%.0.0,%.00f32), Size (%.0.0, %.00f32) Ref (%.0.0, %.00f32)",
            node.Pos.x, node.Pos.y, node.Size.x, node.Size.y, node.SizeRef.x, node.SizeRef.y);
        DebugNodeWindow(node.HostWindow, "HostWindow");
        DebugNodeWindow(node.VisibleWindow, "VisibleWindow");
        BulletText("SelectedTabID: 0x%08X, LastFocusedNodeID: 0x%08X", node.SelectedTabId, node.LastFocusedNodeId);
        BulletText("Misc:%s%s%s%s%s%s%s",
            node.IsDockSpace() ? " IsDockSpace" : "",
            node.IsCentralNode() ? " IsCentralNode" : "",
            is_alive ? " IsAlive" : "", is_active ? " IsActive" : "", node.IsFocused ? " IsFocused" : "",
            node.WantLockSizeOnce ? " WantLockSizeOnce" : "",
            node.HasCentralNodeChild ? " HasCentralNodeChild" : "");
        if (TreeNode("flags", "Flags Merged: 0x%04X, Local: 0x%04X, InWindows: 0x%04X, Shared: 0x%04X", node.MergedFlags, node.LocalFlags, node.LocalFlagsInWindows, node.SharedFlags))
        {
            if (BeginTable("flags", 4))
            {
                TableNextColumn(); DebugNodeDockNodeFlags(&node.MergedFlags, "MergedFlags", false);
                TableNextColumn(); DebugNodeDockNodeFlags(&node.LocalFlags, "LocalFlags", true);
                TableNextColumn(); DebugNodeDockNodeFlags(&node.LocalFlagsInWindows, "LocalFlagsInWindows", false);
                TableNextColumn(); DebugNodeDockNodeFlags(&node.SharedFlags, "SharedFlags", true);
                EndTable();
            }
            TreePop();
        }
        if (node.ParentNode)
            DebugNodeDockNode(node.ParentNode, "ParentNode");
        if (node.ChildNodes[0])
            DebugNodeDockNode(node.ChildNodes[0], "Child[0]");
        if (node.ChildNodes[1])
            DebugNodeDockNode(node.ChildNodes[1], "Child[1]");
        if (node.TabBar)
            DebugNodeTabBar(node.TabBar, "TabBar");
        DebugNodeWindowsList(&node.Windows, "Windows");

        TreePop();
    }
}

// [DEBUG] Display contents of ImDrawList
// Note that both 'window' and 'viewport' may be NULL here. Viewport is generally null of destroyed popups which previously owned a viewport.
c_void DebugNodeDrawList(window: *mut ImGuiWindow, *mut ImGuiViewportP viewport, *const ImDrawList draw_list, label: *const c_char)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiMetricsConfig* cfg = &g.DebugMetricsConfig;
    let cmd_count: c_int = draw_list.CmdBuffer.len();
    if (cmd_count > 0 && draw_list.CmdBuffer.last().unwrap().ElemCount == 0 && draw_list.CmdBuffer.last().unwrap().UserCallback == null_mut())
        cmd_count-= 1;
    let mut node_open: bool =  TreeNode(draw_list, "%s: '%s' %d vtx, %d indices, %d cmds", label, draw_list._OwnerName ? draw_list._OwnerName : "", draw_list.VtxBuffer.len(), draw_list.IdxBuffer.len(), cmd_count);
    if (draw_list == GetWindowDrawList())
    {
        SameLine();
        TextColored(ImVec4(1.0, 0.4f, 0.4f, 1.0), "CURRENTLY APPENDING"); // Can't display stats for active draw list! (we don't have the data double-buffered)
        if (node_open)
            TreePop();
        return;
    }

    let mut  fg_draw_list: *mut ImDrawList =  viewport ? GetForegroundDrawList(viewport) : null_mut(); // Render additional visuals into the top-most draw list
    if (window && IsItemHovered() && fg_draw_list)
        fg_draw_list.AddRect(window.Pos, window.Pos + window.Size, IM_COL32(255, 255, 0, 255));
    if (!node_open)
        return;

    if (window && !window.WasActive)
        TextDisabled("Warning: owning Window is inactive. This DrawList is not being rendered!");

    for (*const ImDrawCmd pcmd = draw_list.CmdBuffer; pcmd < draw_list.CmdBuffer + cmd_count; pcmd++)
    {
        if (pcmd->UserCallback)
        {
            BulletText("Callback %p, user_data %p", pcmd->UserCallback, pcmd->UserCallbackData);
            continue;
        }

        buf: [c_char;300];
        ImFormatString(buf, bu0f32.len(), "DrawCmd:%5d tris, Tex 0x%p, ClipRect (%4.0.0,%4.00f32)-(%4.0.0,%4.00f32)",
            pcmd->ElemCount / 3, (*mut c_void)pcmd.TextureId,
            pcmd->ClipRect.x, pcmd->ClipRect.y, pcmd->ClipRect.z, pcmd->ClipRect.w);
        let mut pcmd_node_open: bool =  TreeNode((*mut c_void)(pcmd - draw_list.CmdBuffer.begin()), "%s", bu0f32);
        if (IsItemHovered() && (cfg->ShowDrawCmdMesh || cfg->ShowDrawCmdBoundingBoxes) && fg_draw_list)
            DebugNodeDrawCmdShowMeshAndBoundingBox(fg_draw_list, draw_list, pcmd, cfg->ShowDrawCmdMesh, cfg->ShowDrawCmdBoundingBoxes);
        if (!pcmd_node_open)
            continue;

        // Calculate approximate coverage area (touched pixel count)
        // This will be in pixels squared as long there's no post-scaling happening to the renderer output.
        let idx_buffer: *const ImDrawIdx = (draw_list.IdxBuffer.len() > 0) ? draw_list.IdxBuffer.Data : null_mut();
        let vtx_buffer: *const ImDrawVert = draw_list.VtxBuffer.Data + pcmd->VtxOffset;
        let total_area: c_float =  0.0;
        for (let mut idx_n: c_uint =  pcmd->IdxOffset; idx_n < pcmd->IdxOffset + pcmd->ElemCount; )
        {
            triangle: ImVec2[3];
            for (let n: c_int = 0; n < 3; n++, idx_n++)
                triangle[n] = vtx_buffer[idx_buffer ? idx_buffer[idx_n] : idx_n].pos;
            total_area += ImTriangleArea(triangle[0], triangle[1], triangle[2]);
        }

        // Display vertex information summary. Hover to get all triangles drawn in wire-frame
        ImFormatString(buf, bu0f32.len(), "Mesh: ElemCount: %d, VtxOffset: +%d, IdxOffset: +%d, Area: ~%0.f px", pcmd->ElemCount, pcmd->VtxOffset, pcmd->IdxOffset, total_area);
        Selectable(bu0f32);
        if (IsItemHovered() && fg_draw_list)
            DebugNodeDrawCmdShowMeshAndBoundingBox(fg_draw_list, draw_list, pcmd, true, false);

        // Display individual triangles/vertices. Hover on to get the corresponding triangle highlighted.
        ImGuiListClipper clipper;
        clipper.Begin(pcmd->ElemCount / 3); // Manually coarse clip our print out of individual vertices to save CPU, only items that may be visible.
        while (clipper.Step())
            for (let prim: c_int = clipper.DisplayStart, idx_i = pcmd->IdxOffset + clipper.DisplayStart * 3; prim < clipper.DisplayEnd; prim++)
            {
                char* buf_p = buf, * buf_end = buf + bu0f32.len();
                triangle: ImVec2[3];
                for (let n: c_int = 0; n < 3; n++, idx_i++)
                {
                    const ImDrawVert& v = vtx_buffer[idx_buffer ? idx_buffer[idx_i] : idx_i];
                    triangle[n] = v.pos;
                    buf_p += ImFormatString(buf_p, buf_end - buf_p, "%s %04d: pos (%8.2f,%8.20f32), uv (%.6f,%.60f32), col %08X\n",
                        (n == 0) ? "Vert:" : "     ", idx_i, v.pos.x, v.pos.y, v.uv.x, v.uv.y, v.col);
                }

                Selectable(buf, false);
                if (fg_draw_list && IsItemHovered())
                {
                    ImDrawListFlags backup_flags = fg_draw_list.Flags;
                    fg_draw_list.Flags &= !ImDrawListFlags_AntiAliasedLines; // Disable AA on triangle outlines is more readable for very large and thin triangles.
                    fg_draw_list.AddPolyline(triangle, 3, IM_COL32(255, 255, 0, 255), ImDrawFlags_Closed, 1.0);
                    fg_draw_list.Flags = backup_flags;
                }
            }
        TreePop();
    }
    TreePop();
}

// [DEBUG] Display mesh/aabb of a ImDrawCmd
c_void DebugNodeDrawCmdShowMeshAndBoundingBox(ImDrawList* out_draw_list, *const ImDrawList draw_list, *const ImDrawCmd draw_cmd, show_mesh: bool, show_aabb: bool)
{
    // IM_ASSERT(show_mesh || show_aabb);

    // Draw wire-frame version of all triangles
    let clip_rect: ImRect =  draw_cmd->ClipRect;
    let mut vtxs_rect: ImRect = ImRect::new(f32::MAX, f32::MAX, -f32::MAX, -f32::MAX);
    ImDrawListFlags backup_flags = out_draw_list.Flags;
    out_draw_list.Flags &= !ImDrawListFlags_AntiAliasedLines; // Disable AA on triangle outlines is more readable for very large and thin triangles.
    for (let mut idx_n: c_uint =  draw_cmd->IdxOffset, idx_end = draw_cmd->IdxOffset + draw_cmd->ElemCount; idx_n < idx_end; )
    {
        ImDrawIdx* idx_buffer = (draw_list.IdxBuffer.len() > 0) ? draw_list.IdxBuffer.Data : null_mut(); // We don't hold on those pointers past iterations as ->AddPolyline() may invalidate them if out_draw_list==draw_list
        vtx_buffer: *mut ImDrawVert = draw_list.VtxBuffer.Data + draw_cmd->VtxOffset;

        triangle: ImVec2[3];
        for (let n: c_int = 0; n < 3; n++, idx_n++)
            vtxs_rect.Add((triangle[n] = vtx_buffer[idx_buffer ? idx_buffer[idx_n] : idx_n].pos));
        if (show_mesh)
            out_draw_list.AddPolyline(triangle, 3, IM_COL32(255, 255, 0, 255), ImDrawFlags_Closed, 1.0); // In yellow: mesh triangles
    }
    // Draw bounding boxes
    if (show_aabb)
    {
        out_draw_list.AddRect(ImFloor(clip_rect.Min), ImFloor(clip_rect.Max), IM_COL32(255, 0, 255, 255)); // In pink: clipping rectangle submitted to GPU
        out_draw_list.AddRect(ImFloor(vtxs_rect.Min), ImFloor(vtxs_rect.Max), IM_COL32(0, 255, 255, 255)); // In cyan: bounding box of triangles
    }
    out_draw_list.Flags = backup_flags;
}

// [DEBUG] Display details for a single font, called by ShowStyleEditor().
c_void DebugNodeFont(font: *mut ImFont)
{
    let mut opened: bool =  TreeNode(font, "Font: \"%s\"\n%.2f px, %d glyphs, %d file(s)",
        font->ConfigData ? font->ConfigData[0].Name : "", font->FontSize, font->Glyphs.Size, font->ConfigDataCount);
    SameLine();
    if (SmallButton("Set as default"))
        GetIO().FontDefault = font;
    if (!opened)
        return;

    // Display preview text
    PushFont(font);
    Text("The quick brown fox jumps over the lazy dog");
    PopFont();

    // Display details
    SetNextItemWidth(GetFontSize() * 8);
    DragFloat("Font scale", &font->Scale, 0.005f, 0.3f, 2.0.0, "%.1f");
    SameLine(); MetricsHelpMarker(
        "Note than the default embedded font is NOT meant to be scaled.\n\n"
        "Font are currently rendered into bitmaps at a given size at the time of building the atlas. "
        "You may oversample them to get some flexibility with scaling. "
        "You can also render at multiple sizes and select which one to use at runtime.\n\n"
        "(Glimmer of hope: the atlas system will be rewritten in the future to make scaling more flexible.)");
    Text("Ascent: %f, Descent: %f, Height: %f", font->Ascent, font->Descent, font->Ascent - font->Descent);
    c_str: [c_char;5];
    Text("Fallback character: '%s' (U+%04X)", ImTextCharToUtf8(c_str, font->FallbackChar), font->FallbackChar);
    Text("Ellipsis character: '%s' (U+%04X)", ImTextCharToUtf8(c_str, font->EllipsisChar), font->EllipsisChar);
    let surface_sqrt: c_int = ImSqrt(font->MetricsTotalSurface);
    Text("Texture Area: about %d px ~%dx%d px", font->MetricsTotalSurface, surface_sqrt, surface_sqrt);
    for (let config_i: c_int = 0; config_i < font->ConfigDataCount; config_i++)
        if (font->ConfigData)
            if (*const ImFontConfig cfg = &font->ConfigData[config_i])
                BulletText("Input %d: \'%s\', Oversample: (%d,%d), PixelSnapH: %d, Offset: (%.1f,%.10f32)",
                    config_i, cfg->Name, cfg->OversampleH, cfg->OversampleV, cfg->PixelSnapH, cfg->GlyphOffset.x, cfg->GlyphOffset.y);

    // Display all glyphs of the fonts in separate pages of 256 characters
    if (TreeNode("Glyphs", "Glyphs (%d)", font->Glyphs.Size))
    {
        let mut  draw_list: *mut ImDrawList =  GetWindowDrawList();
        glyph_col: u32 = GetColorU32(ImGuiCol_Text);
        let cell_size: c_float =  font->FontSize * 1;
        let cell_spacing: c_float =  GetStyle().ItemSpacing.y;
        for (let mut base: c_uint =  0; base <= IM_UNICODE_CODEPOINT_MAX; base += 256)
        {
            // Skip ahead if a large bunch of glyphs are not present in the font (test in chunks of 4k)
            // This is only a small optimization to reduce the number of iterations when IM_UNICODE_MAX_CODEPOINT
            // is large // (if ImWchar==ImWchar32 we will do at least about 272 queries here)
            if (!(base & 4095) && font->IsGlyphRangeUnused(base, base + 4095))
            {
                base += 4096 - 256;
                continue;
            }

            let count: c_int = 0;
            for (let mut n: c_uint =  0; n < 256; n++)
                if (font->FindGlyphNoFallback((base + n)))
                    count+= 1;
            if (count <= 0)
                continue;
            if (!TreeNode((*mut c_void)base, "U+%04X..U+%04X (%d %s)", base, base + 255, count, count > 1 ? "glyphs" : "glyph"))
                continue;

            // Draw a 16x16 grid of glyphs
            let base_pos: ImVec2 = GetCursorScreenPos();
            for (let mut n: c_uint =  0; n < 256; n++)
            {
                // We use ImFont::RenderChar as a shortcut because we don't have UTF-8 conversion functions
                // available here and thus cannot easily generate a zero-terminated UTF-8 encoded string.
                cell_p1: ImVec2(base_pos.x + (n % 16) * (cell_size + cell_spacing), base_pos.y + (n / 16) * (cell_size + cell_spacing));
                cell_p2: ImVec2(cell_p1.x + cell_size, cell_p1.y + cell_size);
                let glyph: *const ImFontGlyph = font->FindGlyphNoFallback((base + n));
                draw_list.AddRect(cell_p1, cell_p2, glyph ? IM_COL32(255, 255, 255, 100) : IM_COL32(255, 255, 255, 50));
                if (!glyph)
                    continue;
                font->RenderChar(draw_list, cell_size, cell_p1, glyph_col, (base + n));
                if (IsMouseHoveringRect(cell_p1, cell_p2))
                {
                    BeginTooltip();
                    DebugNodeFontGlyph(font, glyph);
                    EndTooltip();
                }
            }
            Dummy(ImVec2::new((cell_size + cell_spacing) * 16, (cell_size + cell_spacing) * 16));
            TreePop();
        }
        TreePop();
    }
    TreePop();
}

c_void DebugNodeFontGlyph(ImFont*, *const ImFontGlyph glyph)
{
    Text("Codepoint: U+%04X", glyph->Codepoint);
    Separator();
    Text("Visible: %d", glyph->Visible);
    Text("AdvanceX: %.1f", glyph->AdvanceX);
    Text("Pos: (%.2f,%.20f32)->(%.2f,%.20f32)", glyph->X0, glyph->Y0, glyph->X1, glyph->Y1);
    Text("UV: (%.3f,%.30f32)->(%.3f,%.30f32)", glyph->U0, glyph->V0, glyph->U1, glyph->V1);
}

// [DEBUG] Display contents of ImGuiStorage
c_void DebugNodeStorage(ImGuiStorage* storage, label: *const c_char)
{
    if (!TreeNode(label, "%s: %d entries, %d bytes", label, storage.Data.Size, storage.Data.size_in_bytes()))
        return;
    for (let n: c_int = 0; n < storage.Data.Size; n++)
    {
        const ImGuiStorage::ImGuiStoragePair& p = storage.Data[n];
        BulletText("Key 0x%08X Value { i: %d }", p.key, p.val_i); // Important: we currently don't store a type, real value may not be integer.
    }
    TreePop();
}

// [DEBUG] Display contents of ImGuiTabBar
c_void DebugNodeTabBar(ImGuiTabBar* tab_bar, label: *const c_char)
{
    // Standalone tab bars (not associated to docking/windows functionality) currently hold no discernible strings.
    buf: [c_char;256];
    char* p = buf;
    let mut  buf_end: *const c_char = buf + bu0f32.len();
    let is_active: bool = (tab_bar->PrevFrameVisible >= GetFrameCount() - 2);
    p += ImFormatString(p, buf_end - p, "%s 0x%08X (%d tabs)%s", label, tab_bar->ID, tab_bar->Tabs.Size, is_active ? "" : " *Inactive*");
    p += ImFormatString(p, buf_end - p, "  { ");
    for (let tab_n: c_int = 0; tab_n < ImMin(tab_bar->Tabs.Size, 3); tab_n++)
    {
        ImGuiTabItem* tab = &tab_bar->Tabs[tab_n];
        p += ImFormatString(p, buf_end - p, "%s'%s'",
            tab_n > 0 ? ", " : "", (tab->Window || tab->NameOffset != -1) ? tab_bar->GetTabName(tab) : "???");
    }
    p += ImFormatString(p, buf_end - p, (tab_bar->Tabs.Size > 3) ? " ... }" : " } ");
    if (!is_active) { PushStyleColor(ImGuiCol_Text, GetStyleColorVec4(ImGuiCol_TextDisabled)); }
    let mut open: bool =  TreeNode(label, "%s", bu0f32);
    if (!is_active) { PopStyleColor(); }
    if (is_active && IsItemHovered())
    {
        let mut  draw_list: *mut ImDrawList =  GetForegroundDrawList();
        draw_list.AddRect(tab_bar->BarRect.Min, tab_bar->BarRect.Max, IM_COL32(255, 255, 0, 255));
        draw_list.AddLine(ImVec2::new(tab_bar->ScrollingRectMinX, tab_bar->BarRect.Min.y), ImVec2::new(tab_bar->ScrollingRectMinX, tab_bar->BarRect.Max.y), IM_COL32(0, 255, 0, 255));
        draw_list.AddLine(ImVec2::new(tab_bar->ScrollingRectMaxX, tab_bar->BarRect.Min.y), ImVec2::new(tab_bar->ScrollingRectMaxX, tab_bar->BarRect.Max.y), IM_COL32(0, 255, 0, 255));
    }
    if (open)
    {
        for (let tab_n: c_int = 0; tab_n < tab_bar->Tabs.Size; tab_n++)
        {
            let tab: *const ImGuiTabItem = &tab_bar->Tabs[tab_n];
            PushID(tab);
            if (SmallButton("<")) { TabBarQueueReorder(tab_bar, tab, -1); } SameLine(0, 2);
            if (SmallButton(">")) { TabBarQueueReorder(tab_bar, tab, 1); } SameLine();
            Text("%02d%c Tab 0x%08X '%s' Offset: %.2f, Width: %.2f/%.2f",
                tab_n, (tab->ID == tab_bar->SelectedTabId) ? '*' : ' ', tab->ID, (tab->Window || tab->NameOffset != -1) ? tab_bar->GetTabName(tab) : "???", tab->Offset, tab->Width, tab->ContentWidth);
            PopID();
        }
        TreePop();
    }
}

c_void DebugNodeViewport(*mut ImGuiViewportP viewport)
{
    SetNextItemOpen(true, ImGuiCond_Once);
    if (TreeNode((*mut c_void)viewport.ID, "Viewport #%d, ID: 0x%08X, Parent: 0x%08X, Window: \"%s\"", viewport.Idx, viewport.ID, viewport.ParentViewportId, viewport.Window ? viewport.window.Name : "N/A"))
    {
        flags: ImGuiWindowFlags = viewport.Flags;
        BulletText("Main Pos: (%.0.0,%.00f32), Size: (%.0.0,%.00f32)\nWorkArea Offset Left: %.0.0 Top: %.0.0, Right: %.0.0, Bottom: %.0f\nMonitor: %d, DpiScale: %.0f%%",
            viewport.Pos.x, viewport.Pos.y, viewport.Size.x, viewport.Size.y,
            viewport.WorkOffsetMin.x, viewport.WorkOffsetMin.y, viewport.WorkOffsetMax.x, viewport.WorkOffsetMax.y,
            viewport.PlatformMonitor, viewport.DpiScale * 100f32);
        if (viewport.Idx > 0) { SameLine(); if (SmallButton("Reset Pos")) { viewport.Pos = ImVec2::new(200, 200); viewport.UpdateWorkRect(); if (viewport.Window) viewport.window.Pos = viewport.Pos; } }
        BulletText("Flags: 0x%04X =%s%s%s%s%s%s%s%s%s%s%s%s", viewport.Flags,
            //(flags & ImGuiViewportFlags_IsPlatformWindow) ? " IsPlatformWindow" : "", // Omitting because it is the standard
            (flags & ImGuiViewportFlags_IsPlatformMonitor) ? " IsPlatformMonitor" : "",
            (flags & ImGuiViewportFlags_OwnedByApp) ? " OwnedByApp" : "",
            (flags & ImGuiViewportFlags_NoDecoration) ? " NoDecoration" : "",
            (flags & ImGuiViewportFlags_NoTaskBarIcon) ? " NoTaskBarIcon" : "",
            (flags & ImGuiViewportFlags_NoFocusOnAppearing) ? " NoFocusOnAppearing" : "",
            (flags & ImGuiViewportFlags_NoFocusOnClick) ? " NoFocusOnClick" : "",
            (flags & ImGuiViewportFlags_NoInputs) ? " NoInputs" : "",
            (flags & ImGuiViewportFlags_NoRendererClear) ? " NoRendererClear" : "",
            (flags & ImGuiViewportFlags_TopMost) ? " TopMost" : "",
            (flags & ImGuiViewportFlags_Minimized) ? " Minimized" : "",
            (flags & ImGuiViewportFlags_NoAutoMerge) ? " NoAutoMerge" : "",
            (flags & ImGuiViewportFlags_CanHostOtherWindows) ? " CanHostOtherWindows" : "");
        for (let layer_i: c_int = 0; layer_i < IM_ARRAYSIZE(viewport.DrawDataBuilder.Layers); layer_i++)
            for (let draw_list_i: c_int = 0; draw_list_i < viewport.DrawDataBuilder.Layers[layer_i].Size; draw_list_i++)
                DebugNodeDrawList(null_mut(), viewport, viewport.DrawDataBuilder.Layers[layer_i][draw_list_i], "DrawList");
        TreePop();
    }
}

c_void DebugNodeWindow(window: *mut ImGuiWindow, label: *const c_char)
{
    if (window == null_mut())
    {
        BulletText("%s: NULL", label);
        return;
    }

    let g = GImGui; // ImGuiContext& g = *GImGui;
    let is_active: bool = window.WasActive;
    ImGuiTreeNodeFlags tree_node_flags = (window == g.NavWindow) ? ImGuiTreeNodeFlags_Selected : ImGuiTreeNodeFlags_None;
    if (!is_active) { PushStyleColor(ImGuiCol_Text, GetStyleColorVec4(ImGuiCol_TextDisabled)); }
    let open: bool = TreeNodeEx(label, tree_node_flags, "%s '%s'%s", label, window.Name, is_active ? "" : " *Inactive*");
    if (!is_active) { PopStyleColor(); }
    if (IsItemHovered() && is_active)
        GetForegroundDrawList(window)->AddRect(window.Pos, window.Pos + window.Size, IM_COL32(255, 255, 0, 255));
    if (!open)
        return;

    if (window.MemoryCompacted)
        TextDisabled("Note: some memory buffers have been compacted/freed.");

    flags: ImGuiWindowFlags = window.Flags;
    DebugNodeDrawList(window, window.Viewport, window.DrawList, "DrawList");
    BulletText("Pos: (%.1f,%.10f32), Size: (%.1f,%.10f32), ContentSize (%.1f,%.10f32) Ideal (%.1f,%.10f32)", window.Pos.x, window.Pos.y, window.Size.x, window.Size.y, window.ContentSize.x, window.ContentSize.y, window.ContentSizeIdeal.x, window.ContentSizeIdeal.y);
    BulletText("Flags: 0x%08X (%s%s%s%s%s%s%s%s%s..)", flags,
        (flags & ImGuiWindowFlags_ChildWindow)  ? "Child " : "",      (flags & ImGuiWindowFlags_Tooltip)     ? "Tooltip "   : "",  (flags & ImGuiWindowFlags_Popup) ? "Popup " : "",
        (flags & ImGuiWindowFlags_Modal)        ? "Modal " : "",      (flags & ImGuiWindowFlags_ChildMenu)   ? "ChildMenu " : "",  (flags & ImGuiWindowFlags_NoSavedSettings) ? "NoSavedSettings " : "",
        (flags & ImGuiWindowFlags_NoMouseInputs)? "NoMouseInputs":"", (flags & ImGuiWindowFlags_NoNavInputs) ? "NoNavInputs" : "", (flags & ImGuiWindowFlags_AlwaysAutoResize) ? "AlwaysAutoResize" : "");
    BulletText("WindowClassId: 0x%08X", window.WindowClass.ClassId);
    BulletText("Scroll: (%.2f/%.2f,%.2f/%.20f32) Scrollbar:%s%s", window.Scroll.x, window.ScrollMax.x, window.Scroll.y, window.ScrollMax.y, window.ScrollbarX ? "X" : "", window.ScrollbarY ? "Y" : "");
    BulletText("Active: %d/%d, WriteAccessed: %d, BeginOrderWithinContext: %d", window.Active, window.WasActive, window.WriteAccessed, (window.Active || window.WasActive) ? window.BeginOrderWithinContext : -1);
    BulletText("Appearing: %d, Hidden: %d (CanSkip %d Cannot %d), SkipItems: %d", window.Appearing, window.Hidden, window.HiddenFramesCanSkipItems, window.HiddenFramesCannotSkipItems, window.SkipItems);
    for (let layer: c_int = 0; layer < ImGuiNavLayer_COUNT; layer++)
    {
        let r: ImRect =  window.NavRectRel[layer];
        if (r.Min.x >= r.Max.y && r.Min.y >= r.Max.y)
        {
            BulletText("NavLastIds[%d]: 0x%08X", layer, window.NavLastIds[layer]);
            continue;
        }
        BulletText("NavLastIds[%d]: 0x%08X at +(%.1f,%.10f32)(%.1f,%.10f32)", layer, window.NavLastIds[layer], r.Min.x, r.Min.y, r.Max.x, r.Max.y);
        if (IsItemHovered())
            GetForegroundDrawList(window)->AddRect(r.Min + window.Pos, r.Max + window.Pos, IM_COL32(255, 255, 0, 255));
    }
    BulletText("NavLayersActiveMask: %X, NavLastChildNavWindow: %s", window.DC.NavLayersActiveMask, window.NavLastChildNavWindow ? window.NavLastChildNavwindow.Name : "NULL");

    BulletText("Viewport: %d%s, ViewportId: 0x%08X, ViewportPos: (%.1f,%.10f32)", window.Viewport ? window.Viewport.Idx : -1, window.ViewportOwned ? " (Owned)" : "", window.ViewportId, window.ViewportPos.x, window.ViewportPos.y);
    BulletText("ViewportMonitor: %d", window.Viewport ? window.Viewport.PlatformMonitor : -1);
    BulletText("DockId: 0x%04X, DockOrder: %d, Act: %d, Vis: %d", window.DockId, window.DockOrder, window.DockIsActive, window.DockTabIsVisible);
    if (window.DockNode || window.DockNodeAsHost)
        DebugNodeDockNode(window.DockNodeAsHost ? window.DockNodeAsHost : window.DockNode, window.DockNodeAsHost ? "DockNodeAsHost" : "DockNode");

    if (window.RootWindow != window)       { DebugNodeWindow(window.RootWindow, "RootWindow"); }
    if (window.RootWindowDockTree != window.RootWindow) { DebugNodeWindow(window.RootWindowDockTree, "RootWindowDockTree"); }
    if (window.ParentWindow != null_mut())       { DebugNodeWindow(window.ParentWindow, "ParentWindow"); }
    if (window.DC.ChildWindows.Size > 0)   { DebugNodeWindowsList(&window.DC.ChildWindows, "ChildWindows"); }
    if (window.ColumnsStorage.Size > 0 && TreeNode("Columns", "Columns sets (%d)", window.ColumnsStorage.Size))
    {
        for (let n: c_int = 0; n < window.ColumnsStorage.Size; n++)
            DebugNodeColumns(&window.ColumnsStorage[n]);
        TreePop();
    }
    DebugNodeStorage(&window.StateStorage, "Storage");
    TreePop();
}

c_void DebugNodeWindowSettings(settings: *mut ImGuiWindowSettings)
{
    Text("0x%08X \"%s\" Pos (%d,%d) Size (%d,%d) Collapsed=%d",
        settings->ID, settings->GetName(), settings->Pos.x, settings->Pos.y, settings->Size.x, settings->Size.y, settings->Collapsed);
}

c_void DebugNodeWindowsList(Vec<ImGuiWindow*>* windows, label: *const c_char)
{
    if (!TreeNode(label, "%s (%d)", label, windows->Size))
        return;
    for (let i: c_int = windows->Size - 1; i >= 0; i--) // Iterate front to back
    {
        PushID((*windows)[i]);
        DebugNodeWindow((*windows)[i], "Window");
        PopID();
    }
    TreePop();
}

// FIXME-OPT: This is technically suboptimal, but it is simpler this way.
c_void DebugNodeWindowsListByBeginStackParent(ImGuiWindow** windows, windows_size: c_int, parent_in_begin_stack: *mut ImGuiWindow)
{
    for (let i: c_int = 0; i < windows_size; i++)
    {
        let mut window: *mut ImGuiWindow =  windows[i];
        if (window.ParentWindowInBeginStack != parent_in_begin_stack)
            continue;
        buf: [c_char;20];
        ImFormatString(buf, bu0f32.len(), "[%04d] Window", window.BeginOrderWithinContext);
        //BulletText("[%04d] Window '%s'", window.BeginOrderWithinContext, window.Name);
        DebugNodeWindow(window, bu0f32);
        Indent();
        DebugNodeWindowsListByBeginStackParent(windows + i + 1, windows_size - i - 1, window);
        Unindent();
    }
}

//-----------------------------------------------------------------------------
// [SECTION] DEBUG LOG
//-----------------------------------------------------------------------------

c_void DebugLog(fmt: *const c_char, ...)
{
    va_list args;
    va_start(args, fmt);
    DebugLogV(fmt, args);
    va_end(args);
}

c_void DebugLogV(fmt: *const c_char, va_list args)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let old_size: c_int = g.DebugLogBuf.size();
    g.DebugLogBuf.appendf("[%05d] ", g.FrameCount);
    g.DebugLogBuf.appendfv(fmt, args);
    if (g.DebugLogFlags & ImGuiDebugLogFlags_OutputToTTY)
        IMGUI_DEBUG_PRINTF("%s", g.DebugLogBuf.begin() + old_size);
}

c_void ShowDebugLogWindow(bool* p_open)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (!(g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasSize))
        SetNextWindowSize(ImVec2::new(0.0, GetFontSize() * 12.00f32), ImGuiCond_FirstUseEver);
    if (!Begin("Dear ImGui Debug Log", p_open) || GetCurrentWindow()->BeginCount > 1)
    {
        End();
        return;
    }

    AlignTextToFramePadding();
    Text("Log events:");
    SameLine(); CheckboxFlags("All", &g.DebugLogFlags, ImGuiDebugLogFlags_EventMask_);
    SameLine(); CheckboxFlags("ActiveId", &g.DebugLogFlags, ImGuiDebugLogFlags_EventActiveId);
    SameLine(); CheckboxFlags("Focus", &g.DebugLogFlags, ImGuiDebugLogFlags_EventFocus);
    SameLine(); CheckboxFlags("Popup", &g.DebugLogFlags, ImGuiDebugLogFlags_EventPopup);
    SameLine(); CheckboxFlags("Nav", &g.DebugLogFlags, ImGuiDebugLogFlags_EventNav);
    SameLine(); CheckboxFlags("Clipper", &g.DebugLogFlags, ImGuiDebugLogFlags_EventClipper);
    SameLine(); CheckboxFlags("IO", &g.DebugLogFlags, ImGuiDebugLogFlags_EventIO);
    SameLine(); CheckboxFlags("Docking", &g.DebugLogFlags, ImGuiDebugLogFlags_EventDocking);
    SameLine(); CheckboxFlags("Viewport", &g.DebugLogFlags, ImGuiDebugLogFlags_EventViewport);

    if (SmallButton("Clear"))
        g.DebugLogBuf.clear();
    SameLine();
    if (SmallButton("Copy"))
        SetClipboardText(g.DebugLogBuf.c_str());
    BeginChild("##log", ImVec2::new(0.0, 0.0), true, ImGuiWindowFlags_AlwaysVerticalScrollbar | ImGuiWindowFlags_AlwaysHorizontalScrollbar);
    TextUnformatted(g.DebugLogBuf.begin(), g.DebugLogBuf.end()); // FIXME-OPT: Could use a line index, but TextUnformatted() has a semi-decent fast path for large text.
    if (GetScrollY() >= GetScrollMaxY())
        SetScrollHereY(1.0);
    EndChild();

    End();
}

//-----------------------------------------------------------------------------
// [SECTION] OTHER DEBUG TOOLS (ITEM PICKER, STACK TOOL)
//-----------------------------------------------------------------------------

// [DEBUG] Item picker tool - start with DebugStartItemPicker() - useful to visually select an item and break into its call-stack.
c_void UpdateDebugToolItemPicker()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.DebugItemPickerBreakId = 0;
    if (!g.DebugItemPickerActive)
        return;

    let mut hovered_id: ImGuiID =  g.HoveredIdPreviousFrame;
    SetMouseCursor(ImGuiMouseCursor_Hand);
    if (IsKeyPressed(ImGuiKey_Escape))
        g.DebugItemPickerActive = false;
    let change_mapping: bool = g.IO.KeyMods == (ImGuiModFlags_Ctrl | ImGuiModFlags_Shift);
    if (!change_mapping && IsMouseClicked(g.DebugItemPickerMouseButton) && hovered_id)
    {
        g.DebugItemPickerBreakId = hovered_id;
        g.DebugItemPickerActive = false;
    }
    for (let mouse_button: c_int = 0; mouse_button < 3; mouse_button++)
        if (change_mapping && IsMouseClicked(mouse_button))
            g.DebugItemPickerMouseButton = mouse_button;
    SetNextWindowBgAlpha(0.700f32);
    BeginTooltip();
    Text("HoveredId: 0x%08X", hovered_id);
    Text("Press ESC to abort picking.");
    mouse_button_names: *const c_char[] = { "Left", "Right", "Middle" };
    if (change_mapping)
        Text("Remap w/ Ctrl+Shift: click anywhere to select new mouse button.");
    else
        TextColored(GetStyleColorVec4(hovered_id ? ImGuiCol_Text : ImGuiCol_TextDisabled), "Click %s Button to break in debugger! (remap w/ Ctrl+Shift)", mouse_button_names[g.DebugItemPickerMouseButton]);
    EndTooltip();
}

// [DEBUG] Stack Tool: update queries. Called by NewFrame()
c_void UpdateDebugToolStackQueries()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut tool: *mut ImGuiStackTool =  &g.DebugStackTool;

    // Clear hook when stack tool is not visible
    g.DebugHookIdInfo = 0;
    if (g.FrameCount != tool.LastActiveFrame + 1)
        return;

    // Update queries. The steps are: -1: query Stack, >= 0: query each stack item
    // We can only perform 1 ID Info query every frame. This is designed so the GetID() tests are cheap and constant-time
    let mut query_id: ImGuiID =  g.HoveredIdPreviousFrame ? g.HoveredIdPreviousFrame : g.ActiveId;
    if (tool.QueryId != query_id)
    {
        tool.QueryId = query_id;
        tool.StackLevel = -1;
        tool.Results.clear();
    }
    if (query_id == 0)
        return;

    // Advance to next stack level when we got our result, or after 2 frames (in case we never get a result)
    let stack_level: c_int = tool.StackLevel;
    if (stack_level >= 0 && stack_level < tool.Results.Size)
        if (tool.Results[stack_level].QuerySuccess || tool.Results[stack_level].QueryFrameCount > 2)
            tool.StackLevel+= 1;

    // Update hook
    stack_level = tool.StackLevel;
    if (stack_level == -1)
        g.DebugHookIdInfo = query_id;
    if (stack_level >= 0 && stack_level < tool.Results.Size)
    {
        g.DebugHookIdInfo = tool.Results[stack_level].ID;
        tool.Results[stack_level].QueryFrameCount+= 1;
    }
}

static StackToolFormatLevelInfo: c_int(ImGuiStackTool* tool, n: c_int, format_for_ui: bool, char* buf, buf_size: size_t)
{
    let mut info: *mut ImGuiStackLevelInfo =  &tool.Results[n];
    let mut window: *mut ImGuiWindow =  (info.Desc[0] == 0 && n == 0) ? FindWindowByID(info.ID) : null_mut();
    if (window)                                                                 // Source: window name (because the root ID don't call GetID() and so doesn't get hooked)
        return ImFormatString(buf, buf_size, format_for_ui ? "\"%s\" [window]" : "%s", window.Name);
    if (info.QuerySuccess)                                                     // Source: GetID() hooks (prioritize over ItemInfo() because we frequently use patterns like: PushID(str), Button("") where they both have same id)
        return ImFormatString(buf, buf_size, (format_for_ui && info.DataType == ImGuiDataType_String) ? "\"%s\"" : "%s", info.Desc);
    if (tool.StackLevel < tool.Results.Size)                                  // Only start using fallback below when all queries are done, so during queries we don't flickering ??? markers.
        return (*buf = 0);
// #ifdef IMGUI_ENABLE_TEST_ENGINE
    if (label: *const c_char = ImGuiTestEngine_FindItemDebugLabel(GImGui, info.ID))   // Source: ImGuiTestEngine's ItemInfo()
        return ImFormatString(buf, buf_size, format_for_ui ? "??? \"%s\"" : "%s", label);
// #endif
    return ImFormatString(buf, buf_size, "???");
}

// Stack Tool: Display UI
c_void ShowStackToolWindow(bool* p_open)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (!(g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasSize))
        SetNextWindowSize(ImVec2::new(0.0, GetFontSize() * 8.00f32), ImGuiCond_FirstUseEver);
    if (!Begin("Dear ImGui Stack Tool", p_open) || GetCurrentWindow()->BeginCount > 1)
    {
        End();
        return;
    }

    // Display hovered/active status
    let mut tool: *mut ImGuiStackTool =  &g.DebugStackTool;
    let mut hovered_id: ImGuiID =  g.HoveredIdPreviousFrame;
    let mut active_id: ImGuiID =  g.ActiveId;
// #ifdef IMGUI_ENABLE_TEST_ENGINE
    Text("HoveredId: 0x%08X (\"%s\"), ActiveId:  0x%08X (\"%s\")", hovered_id, hovered_id ? ImGuiTestEngine_FindItemDebugLabel(&g, hovered_id) : "", active_id, active_id ? ImGuiTestEngine_FindItemDebugLabel(&g, active_id) : "");
// #else
    Text("HoveredId: 0x%08X, ActiveId:  0x%08X", hovered_id, active_id);
// #endif
    SameLine();
    MetricsHelpMarker("Hover an item with the mouse to display elements of the ID Stack leading to the item's final ID.\nEach level of the stack correspond to a PushID() call.\nAll levels of the stack are hashed together to make the final ID of a widget (ID displayed at the bottom level of the stack).\nRead FAQ entry about the ID stack for details.");

    // CTRL+C to copy path
    let time_since_copy: c_float =  g.Time - tool.CopyToClipboardLastTime;
    Checkbox("Ctrl+C: copy path to clipboard", &tool.CopyToClipboardOnCtrlC);
    SameLine();
    TextColored((time_since_copy >= 0.0 && time_since_copy < 0.75f && ImFmod(time_since_copy, 0.250f32) < 0.25f * 0.5f32) ? ImVec4(1.f, 1.f, 0.3f, 1.0) : ImVec4(), "*COPIED*");
    if (tool.CopyToClipboardOnCtrlC && IsKeyDown(ImGuiKey_ModCtrl) && IsKeyPressed(ImGuiKey_C))
    {
        tool.CopyToClipboardLastTime = g.Time;
        char* p = g.TempBuffer.Data;
        char* p_end = p + g.TempBuffer.Size;
        for (let stack_n: c_int = 0; stack_n < tool.Results.Size && p + 3 < p_end; stack_n++)
        {
            *p++ = '/';
            level_desc: [c_char;256];
            StackToolFormatLevelInfo(tool, stack_n, false, level_desc, level_desc.len());
            for (let n: c_int = 0; level_desc[n] && p + 2 < p_end; n++)
            {
                if (level_desc[n] == '/')
                    *p++ = '\\';
                *p++ = level_desc[n];
            }
        }
        *p = '\0';
        SetClipboardText(g.TempBuffer.Data);
    }

    // Display decorated stack
    tool.LastActiveFrame = g.FrameCount;
    if (tool.Results.Size > 0 && BeginTable("##table", 3, ImGuiTableFlags_Borders))
    {
        let id_width: c_float =  CalcTextSize("0xDDDDDDDD").x;
        TableSetupColumn("Seed", ImGuiTableColumnFlags_WidthFixed, id_width);
        TableSetupColumn("PushID", ImGuiTableColumnFlags_WidthStretch);
        TableSetupColumn("Result", ImGuiTableColumnFlags_WidthFixed, id_width);
        TableHeadersRow();
        for (let n: c_int = 0; n < tool.Results.Size; n++)
        {
            let mut info: *mut ImGuiStackLevelInfo =  &tool.Results[n];
            TableNextColumn();
            Text("0x%08X", (n > 0) ? tool.Results[n - 1].ID : 0);
            TableNextColumn();
            StackToolFormatLevelInfo(tool, n, true, g.TempBuffer.Data, g.TempBuffer.Size);
            TextUnformatted(g.TempBuffer.Data);
            TableNextColumn();
            Text("0x%08X", info.ID);
            if (n == tool.Results.Size - 1)
                TableSetBgColor(ImGuiTableBgTarget_CellBg, GetColorU32(ImGuiCol_Header));
        }
        EndTable();
    }
    End();
}

// #else

c_void ShowMetricsWindow(bool*) {}
c_void ShowFontAtlas(ImFontAtlas*) {}
c_void DebugNodeColumns(ImGuiOldColumns*) {}
c_void DebugNodeDrawList(ImGuiWindow*, *mut ImGuiViewportP, *const ImDrawList, *const char) {}
c_void DebugNodeDrawCmdShowMeshAndBoundingBox(ImDrawList*, *const ImDrawList, *const ImDrawCmd, bool, bool) {}
c_void DebugNodeFont(ImFont*) {}
c_void DebugNodeStorage(ImGuiStorage*, *const char) {}
c_void DebugNodeTabBar(ImGuiTabBar*, *const char) {}
c_void DebugNodeWindow(ImGuiWindow*, *const char) {}
c_void DebugNodeWindowSettings(ImGuiWindowSettings*) {}
c_void DebugNodeWindowsList(Vec<ImGuiWindow*>*, *const char) {}
c_void DebugNodeViewport {}

c_void DebugLog(*const char, ...) {}
c_void DebugLogV(*const char, va_list) {}
c_void ShowDebugLogWindow(bool*) {}
c_void ShowStackToolWindow(bool*) {}
c_void DebugHookIdInfo(ImGuiID, ImGuiDataType, *const c_void, *const c_void) {}
c_void UpdateDebugToolItemPicker() {}
c_void UpdateDebugToolStackQueries() {}

// #endif // #ifndef IMGUI_DISABLE_DEBUG_TOOLS

//-----------------------------------------------------------------------------

// Include imgui_user.inl at the end of imgui.cpp to access private data/functions that aren't exposed.
// Prefer just including imgui_internal.h from your code rather than using this define. If a declaration is missing from imgui_internal.h add it or request it on the github.
// #ifdef IMGUI_INCLUDE_IMGUI_USER_INL
// #include "imgui_user.inl"
// #endif

//-----------------------------------------------------------------------------

// #endif // #ifndef IMGUI_DISABLE
