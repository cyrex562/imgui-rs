#![allow(non_snake_case)]

use std::borrow::BorrowMut;
use std::ptr::null_mut;
use libc::{c_void, memcmp};
use crate::{type_defs::ImguiHandle, viewport::ImguiViewport, imgui::GImGui, window::{ImguiWindow, window_flags::{ImGuiWindowFlags_NoMouseInputs, ImGuiWindowFlags_NoNavInputs, ImGuiWindowFlags_NoFocusOnAppearing, ImGuiWindowFlags_ChildMenu, ImGuiWindowFlags_Tooltip, ImGuiWindowFlags_Popup, ImGuiWindowFlags_ChildWindow, ImGuiWindowFlags_DockNodeHost, ImGuiWindowFlags_Modal, ImGuiWindowFlags_NoBackground}}, rect::ImRect, vec2::ImVec2, config_flags::{ImGuiConfigFlags_ViewportsEnable, ImGuiConfigFlags_DpiEnableScaleViewports}, viewport_flags::{ImGuiViewportFlags_NoInputs, ImGuiViewportFlags_Minimized, ImGuiViewportFlags_OwnedByApp, ImGuiViewportFlags_CanHostOtherWindows, ImGuiViewportFlags_NoFocusOnAppearing, ImGuiViewportFlags_IsPlatformWindow, ImGuiViewportFlags_TopMost, ImGuiViewportFlags_NoDecoration, ImGuiViewportFlags_NoTaskBarIcon, ImGuiViewportFlags_NoRendererClear, ImGuiViewportFlags_NoFocusOnClick}, hash_string};
use crate::backend_flags::IM_GUI_BACKEND_FLAGS_HAS_MOUSE_HOVERED_VIEWPORT;
use crate::context_ops::GetPlatformIO;
use crate::draw_list::ImDrawList;
use crate::input_ops::{IsAnyMouseDown, IsMousePosValid};
use crate::io_ops::GetIO;
use crate::math_ops::{ImMax, ImMin};
use crate::nav_ops::NavCalcPreferredRefPos;
use crate::next_window_data_flags::ImGuiNextWindowDataFlags_HasViewport;
use crate::platform_io::ImguiPlatformIo;
use crate::platform_monitor::ImGuiPlatformMonitor;
use crate::render_ops::FindRenderedTextEnd;
use crate::settings_ops::MarkIniSettingsDirty;
use crate::utils::{flag_clear, is_not_null};
use crate::viewport_flags::{ImGuiViewportFlags_NoAutoMerge, ImGuiViewportFlags_None};
use crate::window::find::GetWindowForTitleDisplay;
use crate::window::ops::{BringWindowToDisplayFront, IsWindowActiveAndVisible, ScaleWindow, TranslateWindow};
use crate::window::window_flags::ImGuiWindowFlags;

// static c_void SetupViewportDrawData(viewport: *mut ImGuiViewport, Vec<ImDrawList*>* draw_lists)
pub fn SetupViewportDrawData(viewport: *mut ImguiViewport, draw_lists: *mut Vec<*mut ImDrawList>) {
    // When minimized, we report draw_data.DisplaySize as zero to be consistent with non-viewport mode,
    // and to allow applications/backends to easily skip rendering.
    // FIXME: Note that we however do NOT attempt to report "zero drawlist / vertices" into the ImDrawData structure.
    // This is because the work has been done already, and its wasted! We should fix that and add optimizations for
    // it earlier in the pipeline, rather than pretend to hide the data at the end of the pipeline.
    let is_minimized: bool = flag_set(viewport.Flags, ImGuiViewportFlags_Minimized) != 0;

    let io = GetIO();
    let mut draw_data = &mut viewport.DrawDataP;
    viewport.DrawData = draw_data; // Make publicly accessible
    draw_data.Valid = true;
    draw_data.CmdLists = if draw_lists.Size > 0 { draw_lists.Data } else { None };
    draw_data.CmdListsCount = draw_lists.Size;
    draw_data.TotalVtxCount = draw_data.TotalIdxCount = 0;
    draw_data.DisplayPos = viewport.Pos;
    draw_data.DisplaySize = if is_minimized { ImVec2::new2(0.0, 0.0) } else { viewport.Size };
    draw_data.FramebufferScale = io.DisplayFramebufferScale; // FIXME-VIEWPORT: This may vary on a per-monitor/viewport basis?
    draw_data.OwnerViewport = viewport;
    // for (let n: c_int = 0; n < draw_lists.Size; n++)
    for n in 0..draw_lists.len() {
        let mut draw_list: *mut ImDrawList = draw_lists.Data[n];
        draw_list._PopUnusedDrawCmd();
        draw_data.TotalVtxCount += draw_list.VtxBuffer.len();
        draw_data.TotalIdxCount += draw_list.IdxBuffer.len();
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

pub unsafe fn GetMainViewport() -> &mut ImguiViewport
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.Viewports[0].borrow_mut();
}

// FIXME: This leaks access to viewports not listed in PlatformIO.Viewports[]. Problematic? (#4236)
pub unsafe fn FindViewportByID(id: ImguiHandle) -> *mut ImguiViewport
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // for (let n: c_int = 0; n < g.Viewports.len(); n++)
    for n in 0 .. g.Viewports.len()
    {
        if (g.Viewports[n].ID == id)
        {
            return g.Viewports[n];
        }}
    return None;
}

pub unsafe fn FindViewportByPlatformHandle(platform_handle: *mut c_void) -> *mut ImguiViewport
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // for (let i: c_int = 0; i != g.Viewports.len(); i++)
    for i in 0 .. g.Viewports.len()
    {
        if (g.Viewports[i].PlatformHandle == platform_handle){
            return g.Viewports[i];
        }
    }
    return None;
}

pub unsafe fn SetCurrentViewport(current_window: &mut ImguiWindow, viewport: *mut ImguiViewport)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // current_window;

    if (viewport){
        viewport.LastFrameActive = g.FrameCount;}
    if (g.CurrentViewport == viewport){
        return;}
    g.CurrentDpiScale = if viewport { viewport.DpiScale} else {1.0};
    g.CurrentViewport = viewport;
    //IMGUI_DEBUG_LOG_VIEWPORT("[viewport] SetCurrentViewport changed '{}' 0x{}\n", current_window ? current_window.Name : NULL, viewport ? viewport.ID : 0);

    // Notify platform layer of viewport changes
    // FIXME-DPI: This is only currently used for experimenting with handling of multiple DPI
    if (g.CurrentViewport && g.PlatformIO.Platform_OnChangedViewport)
    {
        g.PlatformIO.Platform_OnChangedViewport(g.CurrentViewport);
    }
}

pub unsafe fn SetWindowViewport(window: &mut ImguiWindow, viewport: *mut ImguiViewport)
{
    // Abandon viewport
    if (window.ViewportOwned && window.Viewport.Window == window){
        window.Viewport.Size = ImVec2::from_floats(0.0, 0.0);}

    window.Viewport = viewport;
    window.ViewportId = viewport.ID;
    window.ViewportOwned = (viewport.Window == window);
}

pub unsafe fn GetWindowAlwaysWantOwnViewport(window: &mut ImguiWindow) -> bool
{
    // Tooltips and menus are not automatically forced into their own viewport when the NoMerge flag is set, however the multiplication of viewports makes them more likely to protrude and create their own.
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.IO.ConfigViewportsNoAutoMerge || (window.WindowClass.ViewportFlagsOverrideSet & ImGuiViewportFlags_NoAutoMerge)){
        if (g.ConfigFlagsCurrFrame & ImGuiConfigFlags_ViewportsEnable){
            if (!window.DockIsActive){
                if ((window.Flags & (ImGuiWindowFlags_ChildWindow | ImGuiWindowFlags_ChildMenu | ImGuiWindowFlags_Tooltip)) == 0){
                    if ((window.Flags & ImGuiWindowFlags_Popup) == 0 || flag_set(window.Flags, ImGuiWindowFlags_Modal) != 0){
                        return true;}}}}}
    return false;
}

pub unsafe fn UpdateTryMergeWindowIntoHostViewport(window: &mut ImguiWindow, viewport: *mut ImguiViewport) -> bool
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (window.Viewport == viewport){
        return false;}
    if ((viewport.Flags & ImGuiViewportFlags_CanHostOtherWindows) == 0){
        return false;}
    if ((viewport.Flags & ImGuiViewportFlags_Minimized) != 0){
        return false;}
    if (!viewport.GetMainRect().Contains(window.Rect())){
        return false;}
    if (GetWindowAlwaysWantOwnViewport(window)){
        return false;}

    // FIXME: Can't use g.WindowsFocusOrder[] for root windows only as we care about Z order. If we maintained a DisplayOrder along with FocusOrder we could..
    // for (let n: c_int = 0; n < g.Windows.len(); n++)
    for n in 0 .. g.Windows.len()
    {
        let mut window_behind: *mut ImguiWindow =  g.Windows[n];
        if (window_behind == window){
            break;}
        if (window_behind.WasActive && windoe_behind.ViewportOwned && flag_clear(window_behind.Flags, ImGuiWindowFlags_ChildWindow)){
            if (window_behind.Viewport.GetMainRect().Overlaps(window.Rect())){
                return false;}}
    }

    // Move to the existing viewport, Move child/hosted windows as well (FIXME-OPT: iterate child)
    let mut old_viewport: *mut ImguiViewport =  window.Viewport;
    if (window.ViewportOwned){
        // for (let n: c_int = 0; n < g.Windows.len(); n++)
        for n in 0 .. g.Windows.len()
        {
            if (g.Windows[n].Viewport == old_viewport){
                SetWindowViewport(g.Windows[n], viewport);}}}
    SetWindowViewport(window, viewport);
    BringWindowToDisplayFront(window);

    return true;
}

// FIXME: handle 0 to N host viewports
pub unsafe fn UpdateTryMergeWindowIntoHostViewports(window: &mut ImguiWindow) -> bool
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return UpdateTryMergeWindowIntoHostViewport(window, g.Viewports[0]);
}

// Translate Dear ImGui windows when a Host Viewport has been moved
// (This additionally keeps windows at the same place when ImGuiConfigFlags_ViewportsEnable is toggled!)
pub unsafe fn TranslateWindowsInViewport(viewport: *mut ImguiViewport, old_pos: &ImVec2, new_pos: &ImVec2)
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
    // for (let window_n: c_int = 0; window_n < g.Windows.len(); window_n++)
    for window_n in 0 .. g.Windows.len()
    { // FIXME-OPT
        if (translate_all_windows || (g.Windows[window_n].Viewport == viewport && test_still_fit_rect.Contains(g.Windows[window_n].Rect())))
        {
            TranslateWindow(g.Windows[window_n], delta_pos);
        }
    }
}

// Scale all windows (position, size). Use when e.g. changing DPI. (This is a lossy operation!)
pub unsafe fn ScaleWindowsInViewport(viewport: *mut ImguiViewport, scale: c_float)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (viewport.Window)
    {
        ScaleWindow(viewport.Window, scale);
    }
    else
    {
        // for (let i: c_int = 0; i != g.Windows.len(); i++)
        for i in 0 .. g.Windows.len()
        {
            if (g.Windows[i].Viewport == viewport){
                ScaleWindow(g.Windows[i], scale);}}
    }
}

// If the backend doesn't set MouseLastHoveredViewport or doesn't honor ImGuiViewportFlags_NoInputs, we do a search ourselves.
// A) It won't take account of the possibility that non-imgui windows may be in-between our dragged window and our target window.
// B) It requires Platform_GetWindowFocus to be implemented by backend.
pub unsafe fn FindHoveredViewportFromPlatformWindowStack(mouse_platform_pos: &ImVec2) -> *mut ImguiViewport
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut best_candidate: *mut ImguiViewport =  None;
    // for (let n: c_int = 0; n < g.Viewports.len(); n++)
    for n in 0 .. g.Viewports.len()
    {
        let mut viewport: *mut ImguiViewport =  g.Viewports[n];
        if (!(viewport.Flags & (ImGuiViewportFlags_NoInputs | ImGuiViewportFlags_Minimized)) && viewport.GetMainRect().Contains(mouse_platform_pos)){
            if (best_candidate == None || best_candidate.last_front_most_stamp_count < viewport.last_front_most_stamp_count){
                best_candidate = viewport;}}
    }
    return best_candidate;
}

// Update viewports and monitor infos
// Note that this is running even if 'ImGuiConfigFlags_ViewportsEnable' is not set, in order to clear unused viewports (if any) and update monitor info.
pub unsafe fn UpdateViewportsNewFrame()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.PlatformIO.Viewports.Size <= g.Viewports.Size);

    // Update Minimized status (we need it first in order to decide if we'll apply Pos/Size of the main viewport)
    let viewports_enabled: bool = (g.ConfigFlagsCurrFrame & ImGuiConfigFlags_ViewportsEnable) != 0;
    if (viewports_enabled)
    {
        // for (let n: c_int = 0; n < g.Viewports.len(); n++)
        for n in 0 .. g.Viewports.len()
        {
            let mut viewport: *mut ImguiViewport =  g.Viewports[n];
            let platform_funcs_available: bool = viewport.PlatformWindowCreated;
            if (g.PlatformIO.Platform_GetWindowMinimized && platform_funcs_available)
            {
                let mut minimized: bool =  g.PlatformIO.Platform_GetWindowMinimized(viewport);
                if (minimized){
                    viewport.Flags |= ImGuiViewportFlags_Minimized;}
                else{
                    viewport.Flags &= !ImGuiViewportFlags_Minimized;}
            }
        }
    }

    // Create/update main viewport with current platform position.
    // FIXME-VIEWPORT: Size is driven by backend/user code for backward-compatibility but we should aim to make this more consistent.
    let mut main_viewport: *mut ImguiViewport =  g.Viewports[0];
    // IM_ASSERT(main_viewport.ID == IMGUI_VIEWPORT_DEFAULT_ID);
    // IM_ASSERT(main_viewport.Window == NULL);
    let main_viewport_pos: ImVec2 = if viewports_enabled { g.PlatformIO.Platform_GetWindowPos(main_viewport)} else {ImVec2::from_floats(0.0, 0.0)};
    let main_viewport_size: ImVec2 = g.IO.DisplaySize;
    if (viewports_enabled && (main_viewport.Flags & ImGuiViewportFlags_Minimized))
    {
        main_viewport_pos = main_viewport.Pos;    // Preserve last pos/size when minimized (FIXME: We don't do the same for Size outside of the viewport path)
        main_viewport_size = main_viewport.Size;
    }
    AddUpdateViewport(None, IMGUI_VIEWPORT_DEFAULT_ID, &main_viewport_pos, &main_viewport_size, ImGuiViewportFlags_OwnedByApp | ImGuiViewportFlags_CanHostOtherWindows);

    g.CurrentDpiScale = 0.0;
    g.CurrentViewport= None;
    g.MouseViewport= None;
    // for (let n: c_int = 0; n < g.Viewports.len(); n++)
    for n in 0 .. g.Viewports.len()
    {
        let mut viewport: *mut ImguiViewport =  g.Viewports[n];
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
            if (flag_clear(viewport.Flags, ImGuiViewportFlags_Minimized) && platform_funcs_available)
            {
                // Viewport->WorkPos and WorkSize will be updated below
                if (viewport.PlatformRequestMove){
                    viewport.Pos = viewport.LastPlatformPos = g.PlatformIO.Platform_GetWindowPos(viewport);}
                if (viewport.PlatformRequestResize){
                    viewport.Size = viewport.LastPlatformSize = g.PlatformIO.Platform_GetWindowSize(viewport);}
            }
        }

        // Update/copy monitor info
        UpdateViewportPlatformMonitor(viewport);

        // Lock down space taken by menu bars and status bars, reset the offset for functions like BeginMainMenuBar() to alter them again.
        viewport.WorkOffsetMin = viewport.BuildWorkOffsetMin;
        viewport.WorkOffsetMax = viewport.BuildWorkOffsetMax;
        viewport.BuildWorkOffsetMin = viewport.BuildWorkOffsetMax = ImVec2::from_floats(0.0, 0.0);
        viewport.UpdateWorkRect();

        // Reset alpha every frame. Users of transparency (docking) needs to request a lower alpha back.
        viewport.Alpha = 1.0;

        // Translate Dear ImGui windows when a Host Viewport has been moved
        // (This additionally keeps windows at the same place when ImGuiConfigFlags_ViewportsEnable is toggled!)
        let viewport_delta_pos: ImVec2 = viewport.Pos - viewport.LastPos;
        if ((viewport.Flags & ImGuiViewportFlags_CanHostOtherWindows) && (viewport_delta_pos.x != 0.0 || viewport_delta_pos.y != 0.0)){
            TranslateWindowsInViewport(viewport, &viewport.LastPos, &viewport.Pos);}

        // Update DPI scale
        let mut new_dpi_scale: c_float = 0.0;
        if (g.PlatformIO.Platform_GetWindowDpiScale && platform_funcs_available){
            new_dpi_scale = g.PlatformIO.Platform_GetWindowDpiScale(viewport);}
        else if (viewport.PlatformMonitor != -1){
            new_dpi_scale = g.PlatformIO.Monitors[viewport.PlatformMonitor].DpiScale;}
        else{
            new_dpi_scale = if viewport.DpiScale != 0.0 { viewport.DpiScale} else { 1.0};}
        if (viewport.DpiScale != 0.0 && new_dpi_scale != viewport.DpiScale)
        {
            let scale_factor: c_float =  new_dpi_scale / viewport.DpiScale;
            if (g.IO.ConfigFlags & ImGuiConfigFlags_DpiEnableScaleViewports){
                ScaleWindowsInViewport(viewport, scale_factor);}
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
        monitor.MainPos = main_viewport.Pos;
        monitor.MainSize = main_viewport.Size;
        monitor.WorkPos = main_viewport.WorkPos;
        monitor.WorkSize = main_viewport.WorkSize;
        monitor.DpiScale = main_viewport.DpiScale;
    }

    if (!viewports_enabled)
    {
        g.MouseViewport = main_viewport;
        return;
    }

    // Mouse handling: decide on the actual mouse viewport for this frame between the active/focused viewport and the hovered viewport.
    // Note that 'viewport_hovered' should skip over any viewport that has the ImGuiViewportFlags_NoInputs flags set.
    let mut viewport_hovered: *mut ImguiViewport =  None;
    if (g.IO.BackendFlags & IM_GUI_BACKEND_FLAGS_HAS_MOUSE_HOVERED_VIEWPORT)
    {
        viewport_hovered = if g.IO.MouseHoveredViewport { FindViewportByID(g.IO.MouseHoveredViewport)} else {None};
        if (viewport_hovered && (viewport_hovered.Flags & ImGuiViewportFlags_NoInputs)){
            viewport_hovered = FindHoveredViewportFromPlatformWindowStack(&g.IO.MousePos);} // Backend failed to handle _NoInputs viewport: revert to our fallback.
    }
    else
    {
        // If the backend doesn't know how to honor ImGuiViewportFlags_NoInputs, we do a search ourselves. Note that this search:
        // A) won't take account of the possibility that non-imgui windows may be in-between our dragged window and our target window.
        // B) won't take account of how the backend apply parent<>child relationship to secondary viewports, which affects their Z order.
        // C) uses LastFrameAsRefViewport as a flawed replacement for the last time a window was focused (we could/should fix that by introducing Focus functions in PlatformIO)
        viewport_hovered = FindHoveredViewportFromPlatformWindowStack(&g.IO.MousePos);
    }
    if (viewport_hovered != null_mut()){
        g.MouseLastHoveredViewport = viewport_hovered;}
    else if (g.MouseLastHoveredViewport == null_mut()){
        g.MouseLastHoveredViewport = g.Viewports[0];}

    // Update mouse reference viewport
    // (when moving a window we aim at its viewport, but this will be overwritten below if we go in drag and drop mode)
    // (MovingViewport.Viewport will be NULL in the rare situation where the window disappared while moving, set UpdateMouseMovingWindowNewFrame() for details)
    if (g.MovingWindow && g.Movingwindow.Viewport){
        g.MouseViewport = g.Movingwindow.Viewport;}
    else{
        g.MouseViewport = g.MouseLastHoveredViewport;}

    // When dragging something, always refer to the last hovered viewport.
    // - when releasing a moving window we will revert to aiming behind (at viewport_hovered)
    // - when we are between viewports, our dragged preview will tend to show in the last viewport _even_ if we don't have tooltips in their viewports (when lacking monitor info)
    // - consider the case of holding on a menu item to browse child menus: even thou a mouse button is held, there's no active id because menu items only react on mouse release.
    // FIXME-VIEWPORT: This is essentially broken, when IM_GUI_BACKEND_FLAGS_HAS_MOUSE_HOVERED_VIEWPORT is set we want to trust when viewport_hovered==NULL and use that.
    let is_mouse_dragging_with_an_expected_destination: bool = g.DragDropActive;
    if (is_mouse_dragging_with_an_expected_destination && viewport_hovered == null_mut()){
        viewport_hovered = g.MouseLastHoveredViewport;}
    if (is_mouse_dragging_with_an_expected_destination || g.ActiveId == 0 || !IsAnyMouseDown()){
        if (viewport_hovered != None && viewport_hovered != g.MouseViewport && flag_clear(viewport_hovered.Flags, ImGuiViewportFlags_NoInputs)){
            g.MouseViewport = viewport_hovered;}}

    // IM_ASSERT(g.MouseViewport != NULL);
}

// Update user-facing viewport list (g.Viewports -> g.PlatformIO.Viewports after filtering out some)
pub unsafe fn UpdateViewportsEndFrame()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.PlatformIO.Viewports.clear();
    // for (let i: c_int = 0; i < g.Viewports.len(); i++)
    for i in 0 .. g.Viewports.len()
    {
        let mut viewport: *mut ImguiViewport =  g.Viewports[i];
        viewport.LastPos = viewport.Pos;
        if (viewport.LastFrameActive < g.FrameCount || viewport.Size.x <= 0.0 || viewport.Size.y <= 0.0){
            if (i > 0){ // Always include main viewport in the list
                continue;}}
        if (viewport.Window && !IsWindowActiveAndVisible(viewport.Window)){
            continue;}
        if (i > 0) {}
            // IM_ASSERT(viewport.Window != NULL);
        g.PlatformIO.Viewports.push(viewport);
    }
    g.Viewports[0].ClearRequestFlags(); // Clear main viewport flags because UpdatePlatformWindows() won't do it and may not even be called
}

// FIXME: We should ideally refactor the system to call this every frame (we currently don't)
pub unsafe fn AddUpdateViewport(window: &mut ImguiWindow, id: ImguiHandle, pos: &ImVec2, size: &ImVec2, flags: ImGuiVIewportFlags) -> *mut ImguiViewport
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(id != 0);

    flags |= ImGuiViewportFlags_IsPlatformWindow;
    if (window != null_mut())
    {
        if (g.MovingWindow && g.Movingwindow.RootWindowDockTree == window){
            flags |= ImGuiViewportFlags_NoInputs | ImGuiViewportFlags_NoFocusOnAppearing;}
        if ((window.Flags & ImGuiWindowFlags_NoMouseInputs) && (window.Flags & ImGuiWindowFlags_NoNavInputs)){
            flags |= ImGuiViewportFlags_NoInputs;}
        if (window.Flags & ImGuiWindowFlags_NoFocusOnAppearing){
            flags |= ImGuiViewportFlags_NoFocusOnAppearing;}
    }

    let mut viewport: *mut ImguiViewport =  FindViewportByID(id);
    if (viewport)
    {
        // Always update for main viewport as we are already pulling correct platform pos/size (see #4900)
        if (!viewport.PlatformRequestMove || viewport.ID == IMGUI_VIEWPORT_DEFAULT_ID){
            viewport.Pos = pos;}
        if (!viewport.PlatformRequestResize || viewport.ID == IMGUI_VIEWPORT_DEFAULT_ID){
            viewport.Size = size;}
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
        // IMGUI_DEBUG_LOG_VIEWPORT("[viewport] Add Viewport {} '{}'\n", id, window ? window.Name : "<NULL>");

        // We normally setup for all viewports in NewFrame() but here need to handle the mid-frame creation of a new viewport.
        // We need to extend the fullscreen clip rect so the OverlayDrawList clip is correct for that the first frame
        g.DrawListSharedData.ClipRectFullscreen.x = ImMin(g.DrawListSharedData.ClipRectFullscreen.x, viewport.Pos.x);
        g.DrawListSharedData.ClipRectFullscreen.y = ImMin(g.DrawListSharedData.ClipRectFullscreen.y, viewport.Pos.y);
        g.DrawListSharedData.ClipRectFullscreen.z = ImMax(g.DrawListSharedData.ClipRectFullscreen.z, viewport.Pos.x + viewport.Size.x);
        g.DrawListSharedData.ClipRectFullscreen.w = ImMax(g.DrawListSharedData.ClipRectFullscreen.w, viewport.Pos.y + viewport.Size.y);

        // Store initial DpiScale before the OS platform window creation, based on expected monitor data.
        // This is so we can select an appropriate font size on the first frame of our window lifetime
        if (viewport.PlatformMonitor != -1){
            viewport.DpiScale = g.PlatformIO.Monitors[viewport.PlatformMonitor].DpiScale;}
    }

    viewport.Window = window;
    viewport.LastFrameActive = g.FrameCount;
    viewport.UpdateWorkRect();
    // IM_ASSERT(window == NULL || viewport.ID == window.ID);

    if (window != null_mut()){
        window.ViewportOwned = true;}

    return viewport;
}

pub unsafe fn DestroyViewport(viewport: *mut ImguiViewport)
{
    // Clear references to this viewport in windows (window.ViewportId becomes the master data)
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // for (let window_n: c_int = 0; window_n < g.Windows.len(); window_n++)
    for window_n in 0 .. g.Windows.len()
    {
        let mut window: &mut ImguiWindow =  g.Windows[window_n];
        if (window.Viewport != viewport){
            continue;}
        window.Viewport= None;
        window.ViewportOwned = false;
    }
    if (viewport == g.MouseLastHoveredViewport){
        g.MouseLastHoveredViewport= None;}

    // Destroy
    // IMGUI_DEBUG_LOG_VIEWPORT("[viewport] Delete Viewport {} '{}'\n", viewport.ID, viewport.Window ? viewport.window.Name : "n/a");
    DestroyPlatformWindow(viewport); // In most circumstances the platform window will already be destroyed here.
    // IM_ASSERT(g.PlatformIO.Viewports.contains(viewport) == false);
    // IM_ASSERT(g.Viewports[viewport.Idx] == viewport);
    g.Viewports.erase(g.Viewports.Data + viewport.Idx);
    IM_DELETE(viewport);
}

// FIXME-VIEWPORT: This is all super messy and ought to be clarified or rewritten.
pub unsafe fn WindowSelectViewport(window: &mut ImguiWindow)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let flags: ImGuiWindowFlags = window.Flags;
    window.ViewportAllowPlatformMonitorExtend = -1;

    // Restore main viewport if multi-viewport is not supported by the backend
    let mut main_viewport: *mut ImguiViewport =  GetMainViewport();
    if (!(g.ConfigFlagsCurrFrame & ImGuiConfigFlags_ViewportsEnable))
    {
        SetWindowViewport(window, main_viewport);
        return;
    }
    window.ViewportOwned = false;

    // Appearing popups reset their viewport so they can inherit again
    if ((flags & (ImGuiWindowFlags_Popup | ImGuiWindowFlags_Tooltip)) && window.Appearing)
    {
        window.Viewport= None;
        window.ViewportId = 0;
    }

    if ((g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasViewport) == 0)
    {
        // By default inherit from parent window
        if (window.Viewport == None && window.ParentWindow && (!window.Parentwindow.IsFallbackWindow || window.Parentwindow.WasActive)){
            window.Viewport = window.Parentwindow.Viewport;}

        // Attempt to restore saved viewport id (= window that hasn't been activated yet), try to restore the viewport based on saved 'window.ViewportPos' restored from .ini file
        if (window.Viewport == None && window.ViewportId != 0)
        {
            window.Viewport = FindViewportByID(window.ViewportId);
            if (window.Viewport == None && window.ViewportPos.x != f32::MAX && window.ViewportPos.y != f32::MAX){
                window.Viewport = AddUpdateViewport(window, window.ID, &window.ViewportPos, &window.Size, ImGuiViewportFlags_None);}
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
    else if (flag_set(flags, ImGuiWindowFlags_ChildWindow) || (flags & ImGuiWindowFlags_ChildMenu))
    {
        // Always inherit viewport from parent window
        if (window.DockNode && window.DockNode.HostWindow) {}
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
        window.Viewport = AddUpdateViewport(window, window.ID, &window.position, &window.Size, ImGuiViewportFlags_None);
    }
    else if (g.MovingWindow && g.Movingwindow.RootWindowDockTree == window && IsMousePosValid())
    {
        if (window.Viewport != None && window.Viewport.Window == window){
            window.Viewport = AddUpdateViewport(window, window.ID, &window.position, &window.Size, ImGuiViewportFlags_None);}
    }
    else
    {
        // Merge into host viewport?
        // We cannot test window.ViewportOwned as it set lower in the function.
        // Testing (g.ActiveId == 0 || g.ActiveIdAllowOverlap) to avoid merging during a short-term widget interaction. Main intent was to avoid during resize (see #4212)
        let mut try_to_merge_into_host_viewport: bool =  (window.Viewport && window == window.Viewport.Window && (g.ActiveId == 0 || g.ActiveIdAllowOverlap));
        if (try_to_merge_into_host_viewport){
            UpdateTryMergeWindowIntoHostViewports(window);}
    }

    // Fallback: merge in default viewport if z-order matches, otherwise create a new viewport
    if (window.Viewport == null_mut()){
        if (!UpdateTryMergeWindowIntoHostViewport(window, main_viewport)){
            window.Viewport = AddUpdateViewport(window, window.ID, &window.position, &window.Size, ImGuiViewportFlags_None);}}

    // Mark window as allowed to protrude outside of its viewport and into the current monitor
    if (!lock_viewport)
    {
        if (flags & (ImGuiWindowFlags_Tooltip | ImGuiWindowFlags_Popup))
        {
            // We need to take account of the possibility that mouse may become invalid.
            // Popups/Tooltip always set ViewportAllowPlatformMonitorExtend so GetWindowAllowedExtentRect() will return full monitor bounds.
            let mouse_ref: ImVec2 = if flags & ImGuiWindowFlags_Tooltip { g.IO.MousePos} else { g.BeginPopupStack.last().unwrap().OpenMousePos};
            let mut use_mouse_ref: bool =  (g.NavDisableHighlight || !g.NavDisableMouseHover || !g.NavWindow);
            let mut mouse_valid: bool =  IsMousePosValid(&mouse_re0f32);
            if ((window.Appearing || (flags & (ImGuiWindowFlags_Tooltip | ImGuiWindowFlags_ChildMenu))) && (!use_mouse_ref || mouse_valid)){
                window.ViewportAllowPlatformMonitorExtend = FindPlatformMonitorForPos(if(use_mouse_ref && mouse_valid) { &mouse_ref} else {NavCalcPreferredRefPos(g)});}
            else{
                window.ViewportAllowPlatformMonitorExtend = window.Viewport.PlatformMonitor;}
        }
        else if (window.Viewport && window != window.Viewport.Window && window.Viewport.Window && flag_clear(flags, ImGuiWindowFlags_ChildWindow) && window.DockNode == null_mut())
        {
            // When called from Begin() we don't have access to a proper version of the Hidden flag yet, so we replicate this code.
            let will_be_visible: bool = if window.DockIsActive && !window.DockTabIsVisible { false} else { true};
            if ((window.Flags & ImGuiWindowFlags_DockNodeHost) && window.Viewport.LastFrameActive < g.FrameCount && will_be_visible)
            {
                // Steal/transfer ownership
                // IMGUI_DEBUG_LOG_VIEWPORT("[viewport] Window '{}' steal Viewport {} from Window '{}'\n", window.Name, window.Viewport.ID, window.Viewport.window.Name);
                window.Viewport.Window = window;
                window.Viewport.ID = window.ID;
                window.Viewport.LastNameHash = 0;
            }
            else if (!UpdateTryMergeWindowIntoHostViewports(window)) // Merge?
            {
                // New viewport
                window.Viewport = AddUpdateViewport(window, window.ID, &window.position, &window.Size, ImGuiViewportFlags_NoFocusOnAppearing);
            }
        }
        else if (window.ViewportAllowPlatformMonitorExtend < 0 && flag_clear(flags, ImGuiWindowFlags_ChildWindow))
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

pub unsafe fn WindowSyncOwnedViewport(window: &mut ImguiWindow, parent_window_in_stack: *mut ImguiWindow)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    let mut viewport_rect_changed: bool =  false;

    // Synchronize window --> viewport in most situations
    // Synchronize viewport -> window in case the platform window has been moved or resized from the OS/WM
    if (window.Viewport.PlatformRequestMove)
    {
        window.position = window.Viewport.Pos;
        MarkIniSettingsDirty(window);
    }
    else if (memcmp(&window.Viewport.Pos, &window.position, sizeof(window.position)) != 0)
    {
        viewport_rect_changed = true;
        window.Viewport.Pos = window.position;
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
    if (viewport_rect_changed){
        UpdateViewportPlatformMonitor(window.Viewport);}

    // Update common viewport flags
    const viewport_flags_to_clear: ImGuiVIewportFlags = ImGuiViewportFlags_TopMost | ImGuiViewportFlags_NoTaskBarIcon | ImGuiViewportFlags_NoDecoration | ImGuiViewportFlags_NoRendererClear;
    let viewport_flags: ImGuiVIewportFlags = window.Viewport.Flags & !viewport_flags_to_clear;
    let window_flags: ImGuiWindowFlags = window.Flags;
    let is_modal: bool = (window_flags & ImGuiWindowFlags_Modal) != 0;
    let is_short_lived_floating_window: bool = (window_flags & (ImGuiWindowFlags_ChildMenu | ImGuiWindowFlags_Tooltip | ImGuiWindowFlags_Popup)) != 0;
    if (window_flags & ImGuiWindowFlags_Tooltip){
        viewport_flags |= ImGuiViewportFlags_TopMost;}
    if ((g.IO.ConfigViewportsNoTaskBarIcon || is_short_lived_floating_window) && !is_modal){
        viewport_flags |= ImGuiViewportFlags_NoTaskBarIcon;}
    if (g.IO.ConfigViewportsNoDecoration || is_short_lived_floating_window){
        viewport_flags |= ImGuiViewportFlags_NoDecoration;}

    // Not correct to set modal as topmost because:
    // - Because other popups can be stacked above a modal (e.g. combo box in a modal)
    // - ImGuiViewportFlags_TopMost is currently handled different in backends: in Win32 it is "appear top most" whereas in GLFW and SDL it is "stay topmost"
    //if (flags & ImGuiWindowFlags_Modal)
    //    viewport_flags |= ImGuiViewportFlags_TopMost;

    // For popups and menus that may be protruding out of their parent viewport, we enable _NoFocusOnClick so that clicking on them
    // won't steal the OS focus away from their parent window (which may be reflected in OS the title bar decoration).
    // Setting _NoFocusOnClick would technically prevent us from bringing back to front in case they are being covered by an OS window from a different app,
    // but it shouldn't be much of a problem considering those are already popups that are closed when clicking elsewhere.
    if (is_short_lived_floating_window && !is_modal){
        viewport_flags |= ImGuiViewportFlags_NoFocusOnAppearing | ImGuiViewportFlags_NoFocusOnClick;}

    // We can overwrite viewport flags using ImGuiWindowClass (advanced users)
    if (window.WindowClass.ViewportFlagsOverrideSet){
        viewport_flags |= window.WindowClass.ViewportFlagsOverrideSet;}
    if (window.WindowClass.ViewportFlagsOverrideClear){
        viewport_flags &= !window.WindowClass.ViewportFlagsOverrideClear;}

    // We can also tell the backend that clearing the platform window won't be necessary,
    // as our window background is filling the viewport and we have disabled BgAlpha.
    // FIXME: Work on support for per-viewport transparency (#2766)
    if (!(window_flags & ImGuiWindowFlags_NoBackground)){
        viewport_flags |= ImGuiViewportFlags_NoRendererClear;}

    window.Viewport.Flags = viewport_flags;

    // Update parent viewport ID
    // (the !IsFallbackWindow test mimic the one done in WindowSelectViewport())
    if (window.WindowClass.ParentViewportId != -1){
        window.Viewport.ParentViewportId = window.WindowClass.ParentViewportId;}
    else if ((window_flags & (ImGuiWindowFlags_Popup | ImGuiWindowFlags_Tooltip)) && parent_window_in_stack && (!parent_window_in_stack.IsFallbackWindow || parent_window_in_stack.WasActive)){
        window.Viewport.ParentViewportId = parent_window_in_stack.Viewport.ID;}
    else{
        window.Viewport.ParentViewportId = if g.IO.ConfigViewportsNoDefaultParent { 0} else {IMGUI_VIEWPORT_DEFAULT_ID};}
}

// Called by user at the end of the main loop, after EndFrame()
// This will handle the creation/update of all OS windows via function defined in the ImGuiPlatformIO api.
pub unsafe fn UpdatePlatformWindows()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.FrameCountEnded == g.FrameCount && "Forgot to call Render() or EndFrame() before UpdatePlatformWindows()?");
    // IM_ASSERT(g.FrameCountPlatformEnded < g.FrameCount);
    g.FrameCountPlatformEnded = g.FrameCount;
    if (!(g.ConfigFlagsCurrFrame & ImGuiConfigFlags_ViewportsEnable)){
        return;}

    // Create/resize/destroy platform windows to match each active viewport.
    // Skip the main viewport (index 0), which is always fully handled by the application!
    // for (let i: c_int = 1; i < g.Viewports.len(); i++)
    for i in 1 .. g.Viewports.len()
    {
        let mut viewport: *mut ImguiViewport =  g.Viewports[i];

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
        if (viewport.LastFrameActive < g.FrameCount || viewport.Size.x <= 0 || viewport.Size.y <= 0){
            continue;}

        // Create window
        let mut is_new_platform_window: bool =  (viewport.PlatformWindowCreated == false);
        if (is_new_platform_window)
        {
            // IMGUI_DEBUG_LOG_VIEWPORT("[viewport] Create Platform Window {} '{}'\n", viewport.ID, viewport.Window ? viewport.window.Name : "n/a");
            g.PlatformIO.Platform_CreateWindow(viewport);
            if (g.PlatformIO.Renderer_CreateWindow != null_mut()){
                g.PlatformIO.Renderer_CreateWindow(viewport);}
            viewport.LastNameHash = 0;
            viewport.LastPlatformPos = viewport.LastPlatformSize = ImVec2::from_floats(f32::MAX, f32::MAX); // By clearing those we'll enforce a call to Platform_SetWindowPos/Size below, before Platform_ShowWindow (FIXME: Is that necessary?)
            viewport.LastRendererSize = viewport.Size;                                       // We don't need to call Renderer_SetWindowSize() as it is expected Renderer_CreateWindow() already did it.
            viewport.PlatformWindowCreated = true;
        }

        // Apply Position and Size (from ImGui to Platform/Renderer backends)
        if ((viewport.LastPlatformPos.x != viewport.Pos.x || viewport.LastPlatformPos.y != viewport.Pos.y) && !viewport.PlatformRequestMove){
            g.PlatformIO.Platform_SetWindowPos(viewport, viewport.Pos);}
        if ((viewport.LastPlatformSize.x != viewport.Size.x || viewport.LastPlatformSize.y != viewport.Size.y) && !viewport.PlatformRequestResize){
            g.PlatformIO.Platform_SetWindowSize(viewport, viewport.Size);}
        if ((viewport.LastRendererSize.x != viewport.Size.x || viewport.LastRendererSize.y != viewport.Size.y) && g.PlatformIO.Renderer_SetWindowSize){
            g.PlatformIO.Renderer_SetWindowSize(viewport, viewport.Size);}
        viewport.LastPlatformPos = viewport.Pos;
        viewport.LastPlatformSize = viewport.LastRendererSize = viewport.Size;

        // Update title bar (if it changed)
        let mut window_for_title: *mut ImguiWindow =  GetWindowForTitleDisplay(viewport.Window);
        if (is_not_null(window_for_title))
        {
            let mut  title_begin: *const c_char = window_for_title.Name;
            char* title_end = FindRenderedTextEnd(title_begin);
            let mut title_hash: ImguiHandle =  hash_string(title_begin, title_end - title_begin);
            if (viewport.LastNameHash != title_hash)
            {
                 let mut title_end_backup_c: c_char = *title_end;
                *title_end = 0; // Cut existing buffer short instead of doing an alloc/free, no small gain.
                g.PlatformIO.Platform_SetWindowTitle(viewport, title_begin);
                *title_end = title_end_backup_c;
                viewport.LastNameHash = title_hash;
            }
        }

        // Update alpha (if it changed)
        if (viewport.LastAlpha != viewport.Alpha && g.PlatformIO.Platform_SetWindowAlpha){
            g.PlatformIO.Platform_SetWindowAlpha(viewport, viewport.Alpha);}
        viewport.LastAlpha = viewport.Alpha;

        // Optional, general purpose call to allow the backend to perform general book-keeping even if things haven't changed.
        if (g.PlatformIO.Platform_UpdateWindow){
            g.PlatformIO.Platform_UpdateWindow(viewport);}

        if (is_new_platform_window)
        {
            // On startup ensure new platform window don't steal focus (give it a few frames, as nested contents may lead to viewport being created a few frames late)
            if (g.FrameCount < 3){
                viewport.Flags |= ImGuiViewportFlags_NoFocusOnAppearing;}

            // Show window
            g.PlatformIO.Platform_ShowWindow(viewport);

            // Even without focus, we assume the window becomes front-most.
            // This is useful for our platform z-order heuristic when io.MouseHoveredViewport is not available.
            if (viewport.last_front_most_stamp_count != g.ViewportFrontMostStampCount){
                viewport.last_front_most_stamp_count = g.ViewportFrontMostStampCount;}
            }

        // Clear request flags
        viewport.ClearRequestFlags();
    }

    // Update our implicit z-order knowledge of platform windows, which is used when the backend cannot provide io.MouseHoveredViewport.
    // When setting Platform_GetWindowFocus, it is expected that the platform backend can handle calls without crashing if it doesn't have data stored.
    // FIXME-VIEWPORT: We should use this information to also set dear imgui-side focus, allowing us to handle os-level alt+tab.
    if (g.PlatformIO.Platform_GetWindowFocus != null_mut())
    {
        let mut focused_viewport: *mut ImguiViewport =  None;
        // for (let n: c_int = 0; n < g.Viewports.len() && focused_viewport == None; n++)
        for n in 0 .. g.Viewports.len()
        {
            let mut viewport: *mut ImguiViewport =  g.Viewports[n];
            if (viewport.PlatformWindowCreated){
                if (g.PlatformIO.Platform_GetWindowFocus(viewport)){
                    focused_viewport = viewport;}}
        }

        // Store a tag so we can infer z-order easily from all our windows
        // We compare PlatformLastFocusedViewportId so newly created viewports with _NoFocusOnAppearing flag
        // will keep the front most stamp instead of losing it back to their parent viewport.
        if (focused_viewport && g.PlatformLastFocusedViewportId != focused_viewport.ID)
        {
            if (focused_viewport.last_front_most_stamp_count != g.ViewportFrontMostStampCount){
                g.ViewportFrontMostStampCount += 1;
                focused_viewport.last_front_most_stamp_count = g.ViewportFrontMostStampCount;}
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
pub unsafe fn RenderPlatformWindowsDefault(platform_render_arg: *mut c_void, renderer_render_arg: *mut c_void)
{
    // Skip the main viewport (index 0), which is always fully handled by the application!
    ImGuiPlatformIO& platform_io = GetPlatformIO();
    // for (let i: c_int = 1; i < platform_io.Viewports.len(); i++)
    for i in 1 .. platform_io.Viewports.len()
    {
        let viewport = platform_io.Viewports[i];
        if (viewport.Flags & ImGuiViewportFlags_Minimized){
            continue;}
        if (platform_io.Platform_RenderWindow) {platform_io.Platform_RenderWindow(viewport, platform_render_arg);}
        if (platform_io.Renderer_RenderWindow) {platform_io.Renderer_RenderWindow(viewport, renderer_render_arg);}
    }
    // for (let i: c_int = 1; i < platform_io.Viewports.len(); i++)
    for i in 1 .. platform_io.Voewports.len()
    {
        let viewport = platform_io.Viewports[i];
        if (viewport.Flags & ImGuiViewportFlags_Minimized){
            continue;}
        if (platform_io.Platform_SwapBuffers) {platform_io.Platform_SwapBuffers(viewport, platform_render_arg);}
        if (platform_io.Renderer_SwapBuffers){ platform_io.Renderer_SwapBuffers(viewport, renderer_render_arg);}
    }
}

pub unsafe fn FindPlatformMonitorForPos(pos: &ImVec2) -> c_int
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // for (let monitor_n: c_int = 0; monitor_n < g.PlatformIO.Monitors.Size; monitor_n++)
    for monitor_n in 0 .. g.PlatformIO.Monitors.len()
    {
        let monitor = g.PlatformIO.Monitors[monitor_n];
        if (ImRect(monitor.MainPos, monitor.MainPos + monitor.MainSize).Contains(pos)){
            return monitor_n;}
    }
    return -1;
}

// Search for the monitor with the largest intersection area with the given rectangle
// We generally try to avoid searching loops but the monitor count should be very small here
// FIXME-OPT: We could test the last monitor used for that viewport first, and early
pub unsafe fn FindPlatformMonitorForRect(rect: &ImRect) -> c_int
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    let monitor_count: c_int = g.PlatformIO.Monitors.Size;
    if (monitor_count <= 1){
        return monitor_count - 1;}

    // Use a minimum threshold of 1.0 so a zero-sized rect won't false positive, and will still find the correct monitor given its position.
    // This is necessary for tooltips which always resize down to zero at first.
    let surface_threshold: c_float =  ImMax(rect.GetWidth() * rect.GetHeight() * 0.5, 1.0);
    let best_monitor_n: c_int = -1;
    let best_monitor_surface: c_float =  0.001f;

    // for (let monitor_n: c_int = 0; monitor_n < g.PlatformIO.Monitors.Size && best_monitor_surface < surface_threshold; monitor_n++)
    for moniotor_n in 0 .. g.PlatformIO.Monitors.len()
    {
        let monitor = g.PlatformIO.Monitors[monitor_n];
        let monitor_rect: ImRect =  ImRect(monitor.MainPos, monitor.MainPos + monitor.MainSize);
        if (monitor_rect.Contains(rect)){
            return monitor_n;}
        let overlapping_rect: ImRect =  rect;
        overlapping_rect.ClipWithFull(&monitor_rect);
        let overlapping_surface: c_float =  overlapping_rect.GetWidth() * overlapping_rect.GetHeight();
        if (overlapping_surface < best_monitor_surface){
            continue;}
        best_monitor_surface = overlapping_surface;
        best_monitor_n = monitor_n;
        if best_monitor_surface >= surface_threshold {
            break;
        }
    }
    return best_monitor_n;
}

// Update monitor from viewport rectangle (we'll use this info to clamp windows and save windows lost in a removed monitor)
pub unsafe fn UpdateViewportPlatformMonitor(viewport: *mut ImguiViewport)
{
    viewport.PlatformMonitor = FindPlatformMonitorForRect(&viewport.GetMainRect());
}

// Return value is always != NULL, but don't hold on it across frames.
pub unsafe fn GetViewportPlatformMonitor(viewport_p: *mut ImguiViewport) -> *const ImGuiPlatformMonitor
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut viewport: *mut ImguiViewport =  viewport_p;
    let monitor_idx: c_int = viewport.PlatformMonitor;
    if (monitor_idx >= 0 && monitor_idx < g.PlatformIO.Monitors.Size){
        return &g.PlatformIO.Monitors[monitor_idx];}
    return &g.FallbackMonitor;
}

pub unsafe fn DestroyPlatformWindow(viewport: *mut ImguiViewport)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (viewport.PlatformWindowCreated)
    {
        if (g.PlatformIO.Renderer_DestroyWindow){
            g.PlatformIO.Renderer_DestroyWindow(viewport);}
        if (g.PlatformIO.Platform_DestroyWindow){
            g.PlatformIO.Platform_DestroyWindow(viewport);}
        // IM_ASSERT(viewport.RendererUserData == NULL && viewport.PlatformUserData == NULL);

        // Don't clear PlatformWindowCreated for the main viewport, as we initially set that up to true in Initialize()
        // The righter way may be to leave it to the backend to set this flag all-together, and made the flag public.
        if (viewport.ID != IMGUI_VIEWPORT_DEFAULT_ID){
            viewport.PlatformWindowCreated = false;}
    }
    else
    {
        // IM_ASSERT(viewport.RendererUserData == NULL && viewport.PlatformUserData == NULL && viewport.PlatformHandle == NULL);
    }
    viewport.RendererUserData = viewport.PlatformUserData = viewport.PlatformHandle= None;
    viewport.ClearRequestFlags();
}

pub unsafe fn DestroyPlatformWindows()
{
    // We call the destroy window on every viewport (including the main viewport, index 0) to give a chance to the backend
    // to clear any data they may have stored in e.g. PlatformUserData, RendererUserData.
    // It is convenient for the platform backend code to store something in the main viewport, in order for e.g. the mouse handling
    // code to operator a consistent manner.
    // It is expected that the backend can handle calls to Renderer_DestroyWindow/Platform_DestroyWindow without
    // crashing if it doesn't have data stored.
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // for (let i: c_int = 0; i < g.Viewports.len(); i++)
    for i in 0 .. g.Viewports.len()
    {
        DestroyPlatformWindow(g.Viewports[i]);
    }
}
