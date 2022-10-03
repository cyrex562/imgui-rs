use core::ptr::null_mut;
use libc::{c_char, c_float, c_int, c_short, size_t, strcmp};
use imgui_rs::context_hook::ImGuiContextHookType_Shutdown;
use imgui_rs::context_ops::CallContextHooks;
use imgui_rs::file_ops::ImFileClose;
use imgui_rs::imgui::GImGui;
use imgui_rs::settings_handler::ImGuiSettingsHandler;
use imgui_rs::viewport::ImGuiViewport;
use crate::cursor_ops::ErrorCheckUsingSetCursorPosToExtendParentBoundaries;
use crate::{CallContextHooks, GImGui, ImFileClose, ImGuiContextHookType_Shutdown, ImGuiSettingsHandler, ImGuiViewport};
use crate::condition::{ImGuiCond_Appearing, ImGuiCond_FirstUseEver, ImGuiCond_Once};
use crate::config_flags::{ImGuiConfigFlags_DockingEnable, ImGuiConfigFlags_DpiEnableScaleFonts};
use crate::direction::ImGuiDir_None;
use crate::garbage_collection::GcAwakeTransientWindowBuffers;
use crate::input_ops::IsMouseHoveringRect;
use crate::item_ops::SetLastItemData;
use crate::item_status_flags::ImGuiItemStatusFlags_HoveredRect;
use crate::nav_layer::ImGuiNavLayer_Main;
use crate::next_window_data_flags::{ImGuiNextWindowDataFlags_HasCollapsed, ImGuiNextWindowDataFlags_HasContentSize, ImGuiNextWindowDataFlags_HasDock, ImGuiNextWindowDataFlags_HasFocus, ImGuiNextWindowDataFlags_HasPos, ImGuiNextWindowDataFlags_HasScroll, ImGuiNextWindowDataFlags_HasSize, ImGuiNextWindowDataFlags_HasSizeConstraint, ImGuiNextWindowDataFlags_HasWindowClass};
use crate::platform_monitor::ImGuiPlatformMonitor;
use crate::popup_data::ImGuiPopupData;
use crate::rect::ImRect;
use crate::resize_ops::UpdateWindowManualResize;
use crate::string_ops::ImStrdupcpy;
use crate::utils::{flag_clear, flag_set};
use crate::vec2::ImVec2;
use crate::vec4::ImVec4;
use crate::window::ImGuiWindow;
use crate::window_flags::{ImGuiWindowFlags, ImGuiWindowFlags_AlwaysAutoResize, ImGuiWindowFlags_AlwaysHorizontalScrollbar, ImGuiWindowFlags_AlwaysUseWindowPadding, ImGuiWindowFlags_AlwaysVerticalScrollbar, ImGuiWindowFlags_ChildMenu, ImGuiWindowFlags_ChildWindow, ImGuiWindowFlags_DockNodeHost, ImGuiWindowFlags_HorizontalScrollbar, ImGuiWindowFlags_MenuBar, ImGuiWindowFlags_Modal, ImGuiWindowFlags_NavFlattened, ImGuiWindowFlags_NoCollapse, ImGuiWindowFlags_NoDocking, ImGuiWindowFlags_NoFocusOnAppearing, ImGuiWindowFlags_NoInputs, ImGuiWindowFlags_NoMove, ImGuiWindowFlags_NoNavFocus, ImGuiWindowFlags_NoResize, ImGuiWindowFlags_NoScrollbar, ImGuiWindowFlags_NoTitleBar, ImGuiWindowFlags_Popup, ImGuiWindowFlags_Tooltip};
use crate::window_ops::{CalcWindowSizeAfterConstraint, ClampWindowRect, CreateNewWindow, FindWindowByName, PopClipRect, PushClipRect, RenderWindowDecorations, SetCurrentWindow, UpdateWindowInFocusOrderList};
use crate::window_stack_data::ImGuiWindowStackData;

// c_void End()
pub unsafe fn End()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;

    // Error checking: verify that user hasn't called End() too many times!
    if g.CurrentWindowStack.Size <= 1 && g.WithinFrameScopeWithImplicitWindow
    {
        // IM_ASSERT_USER_ERROR(g.CurrentWindowStack.Size > 1, "Calling End() too many times!");
        return;
    }
    // IM_ASSERT(g.CurrentWindowStack.Size > 0);

    // Error checking: verify that user doesn't directly call End() on a child window.
    if flag_set(window.Flags, ImGuiWindowFlags_ChildWindow) && flag_clear(window.Flags, ImGuiWindowFlags_DockNodeHost) && !window.DockIsActive {}
        // IM_ASSERT_USER_ERROR(g.WithinEndChild, "Must call EndChild() and not End()!");

    // Close anything that is open
    if window.DC.CurrentColumns {
        EndColumns();
    }
    if !(window.Flags & ImGuiWindowFlags_DockNodeHost) {  // Pop inner window clip rectangle
        PopClipRect();
    }

    // Stop logging
    if !(window.Flags & ImGuiWindowFlags_ChildWindow) { // FIXME: add more options for scope of logging
        LogFinish();
    }

    if window.DC.IsSetPos {
        ErrorCheckUsingSetCursorPosToExtendParentBoundaries();
    }

    // Docking: report contents sizes to parent to allow for auto-resize
    if window.DockNode.is_null() == false && window.DockTabIsVisible {
         let mut host_window: *mut ImGuiWindow = window.DockNode.HostWindow;
        if host_window.is_null() == false
               { // FIXME-DOCK
            host_window.DC.CursorMaxPos = window.DC.CursorMaxPos + window.WindowPadding - host_window.WindowPadding;
        }
    }
    // Pop from window stack
    g.LastItemData = g.CurrentWindowStack.last().unwrap().ParentLastItemDataBackup.clone();
    if window.Flags & ImGuiWindowFlags_ChildMenu {
        g.BeginMenuCount -= 1;
    }
    if window.Flags & ImGuiWindowFlags_Popup {
        g.BeginPopupStack.pop_back();
    }
    g.CurrentWindowStack.last_mut().unwrap().StackSizesOnBegin.CompareWithCurrentState();
    g.CurrentWindowStack.pop_back();
    SetCurrentWindow(if g.CurrentWindowStack.Size == 0 { null_mut() } else { g.CurrentWindowStack.last().unwrap().Window });
    if g.CurrentWindow {
        SetCurrentViewport(g.CurrentWindow, g.Currentwindow.Viewport);
    }
}

// c_void Initialize()
pub unsafe fn Initialize()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(!g.Initialized && !g.SettingsLoaded);

    // Add .ini handle for ImGuiWindow type
    {
        let mut ini_handler = ImGuiSettingsHandler::new();
        ini_handler.TypeName = "Window";
        ini_handler.TypeHash = ImHashStr2("Window");
        ini_handler.ClearAllFn = WindowSettingsHandler_ClearAll;
        ini_handler.ReadOpenFn = WindowSettingsHandler_ReadOpen;
        ini_handler.ReadLineFn = WindowSettingsHandler_ReadLine;
        ini_handler.ApplyAllFn = WindowSettingsHandler_ApplyAll;
        ini_handler.WriteAllFn = WindowSettingsHandler_WriteAll;
        AddSettingsHandler(&ini_handler);
    }

    // Add .ini handle for ImGuiTable type
    TableSettingsAddSettingsHandler();

    // Create default viewport
    let mut viewport: *mut ImGuiViewport =  IM_NEW(ImGuiViewportP)();
    viewport.ID = IMGUI_VIEWPORT_DEFAULT_ID;
    viewport.Idx = 0;
    viewport.PlatformWindowCreated = true;
    viewport.Flags = ImGuiViewportFlags_OwnedByApp;
    g.Viewports.push(viewport);
    g.TempBuffer.resize(1024 * 3 + 1, 0);
    g.PlatformIO.Viewports.push(g.Viewports[0]);

// #ifdef IMGUI_HAS_DOCK
    // Initialize Docking
    DockContextInitialize(&g);
// #endif

    g.Initialized = true;
}

// This function is merely here to free heap allocations.
// c_void Shutdown()
pub unsafe fn Shutdown()
{
    // The fonts atlas can be used prior to calling NewFrame(), so we clear it even if g.Initialized is FALSE (which would happen if we never called NewFrame)
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if g.IO.Fonts.is_null() == false && g.FontAtlasOwnedByContext
    {
        g.IO.Fonts.Locked = false;
        IM_DELETE(g.IO.Fonts);
    }
    g.IO.Fonts= null_mut();

    // Cleanup of other data are conditional on actually having initialized Dear ImGui.
    if !g.Initialized {
        return;
    }

    // Save settings (unless we haven't attempted to load them: CreateContext/DestroyContext without a call to NewFrame shouldn't save an empty file)
    if g.SettingsLoaded && g.IO.IniFilename != null_mut() {
        SaveIniSettingsToDisk(g.IO.IniFilename);
    }

    // Destroy platform windows
    DestroyPlatformWindows();

    // Shutdown extensions
    DockContextShutdown(&g);

    CallContextHooks(g, ImGuiContextHookType_Shutdown);

    // Clear everything else
    g.Windows.clear_delete();
    g.WindowsFocusOrder.clear();
    g.WindowsTempSortBuffer.clear();
    g.CurrentWindow= null_mut();
    g.CurrentWindowStack.clear();
    g.WindowsById.Clear();
    g.NavWindow= null_mut();
    g.HoveredWindow = null_Mut();
    g.HoveredWindowUnderMovingWindow= null_mut();
    g.ActiveIdWindow = null_mut();
    g.ActiveIdPreviousFrameWindow= null_mut();
    g.MovingWindow= null_mut();
    g.ColorStack.clear();
    g.StyleVarStack.clear();
    g.FontStack.clear();
    g.OpenPopupStack.clear();
    g.BeginPopupStack.clear();

    g.CurrentViewport = null_mut();
    g.MouseViewport = null_mut();
    g.MouseLastHoveredViewport= null_mut();
    g.Viewports.clear_delete();

    g.TabBars.Clear();
    g.CurrentTabBarStack.clear();
    g.ShrinkWidthBuffer.clear();

    g.ClipperTempData.clear_destruct();

    g.Tables.Clear();
    g.TablesTempData.clear_destruct();
    g.DrawChannelsTempMergeBuffer.clear();

    g.ClipboardHandlerData.clear();
    g.MenusIdSubmittedThisFrame.clear();
    g.InputTextState.ClearFreeMemory();

    g.SettingsWindows.clear();
    g.SettingsHandlers.clear();

    if g.LogFile
    {
// #ifndef IMGUI_DISABLE_TTY_FUNCTIONS
        if g.LogFile != libc::stdout {
// #endif
            ImFileClose(g.LogFile);
        }
        g.LogFile= null_mut();
    }
    g.LogBuffer.clear();
    g.DebugLogBuf.clear();

    g.Initialized = false;
}


// Push a new Dear ImGui window to add widgets to.
// - A default window called "Debug" is automatically stacked at the beginning of every frame so you can use widgets without explicitly calling a Begin/End pair.
// - Begin/End can be called multiple times during the frame with the same window name to append content.
// - The window name is used as a unique identifier to preserve window information across frames (and save rudimentary information to the .ini file).
//   You can use the "##" or "###" markers to use the same label with different id, or same id with different label. See documentation at the top of this file.
// - Return false when window is collapsed, so you can early out in your code. You always need to call End() even if false is returned.
// - Passing 'bool* p_open' displays a Close button on the upper-right corner of the window, the pointed value will be set to false when the button is pressed.
// bool Begin(*const char name, bool* p_open, ImGuiWindowFlags flags)
pub unsafe fn Begin(name: *const c_char, p_open: *mut bool, mut flags: ImGuiWindowFlags) -> bool
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let style = &mut g.Style;
    // IM_ASSERT(name != NULL && name[0] != '\0');     // Window name required
    // IM_ASSERT(g.WithinFrameScope);                  // Forgot to call NewFrame()
    // IM_ASSERT(g.FrameCountEnded != g.FrameCount);   // Called Render() or EndFrame() and haven't called NewFrame() again yet

    // Find or create
    let mut window: *mut ImGuiWindow =  FindWindowByName(name);
    let window_just_created: bool = (window == null_mut());
    if window_just_created {
        window = CreateNewWindow(name, flags);
    }

    // Automatically disable manual moving/resizing when NoInputs is set
    if (flags & ImGuiWindowFlags_NoInputs) == ImGuiWindowFlags_NoInputs {
        flags |= ImGuiWindowFlags_NoMove | ImGuiWindowFlags_NoResize;
    }

    if flag_set(flags, ImGuiWindowFlags_NavFlattened) {}
        // IM_ASSERT(flags & ImGuiWindowFlags_ChildWindow);

    let current_frame: c_int = g.FrameCount;
    let first_begin_of_the_frame: bool = (window.LastFrameActive != current_frame);
    window.IsFallbackWindow = (g.CurrentWindowStack.Size == 0 && g.WithinFrameScopeWithImplicitWindow);

    // Update the Appearing flag (note: the BeginDocked() path may also set this to true later)
    let mut window_just_activated_by_user: bool =  (window.LastFrameActive < current_frame - 1); // Not using !WasActive because the implicit "Debug" window would always toggle off->on
    if flag_set(flags, ImGuiWindowFlags_Popup)
    {
        let popup_ref = g.OpenPopupStack[g.BeginPopupStack.Size];
        window_just_activated_by_user |= (window.PopupId != popup_ref.PopupId); // We recycle popups so treat window as activated if popup id changed
        window_just_activated_by_user |= (window != popup_ref.Window);
    }

    // Update Flags, LastFrameActive, BeginOrderXXX fields
    let window_was_appearing: bool = window.Appearing;
    if first_begin_of_the_frame
    {
        UpdateWindowInFocusOrderList(window, window_just_created, flags);
        window.Appearing = window_just_activated_by_user;
        if window.Appearing {
            SetWindowConditionAllowFlags(window, ImGuiCond_Appearing, true);
        }
        window.FlagsPreviousFrame = window.Flags;
        window.Flags = flags;
        window.LastFrameActive = current_frame;
        window.LastTimeActive = g.Time.clone() as c_float;
        window.BeginOrderWithinParent = 0;
        window.BeginOrderWithinContext = (g.WindowsActiveCount) as c_short;
        g.WindowsActiveCount += 1;
    }
    else
    {
        flags = window.Flags;
    }

    // Docking
    // (NB: during the frame dock nodes are created, it is possible that (window.DockIsActive == false) even though (window.DockNode->Windows.Size > 1)
    // IM_ASSERT(window.DockNode == NULL || window.DockNodeAsHost == NULL); // Cannot be both
    if flag_set(g.NextWindowData.Flags, ImGuiNextWindowDataFlags_HasDock) {
        SetWindowDock(window, g.NextWindowData.DockId, g.NextWindowData.DockCond);
    }
    if first_begin_of_the_frame
    {
        let mut has_dock_node: bool =  (window.DockId != 0 || window.DockNode != null_mut());
        let mut new_auto_dock_node: bool =  !has_dock_node && GetWindowAlwaysWantOwnTabBar(window);
        let mut dock_node_was_visible: bool =  window.DockNodeIsVisible;
        let mut dock_tab_was_visible: bool =  window.DockTabIsVisible;
        if has_dock_node || new_auto_dock_node
        {
            BeginDocked(window, p_open);
            flags = window.Flags;
            if window.DockIsActive
            {
                // IM_ASSERT(window.DockNode != NULL);
                g.NextWindowData.Flags &= !ImGuiNextWindowDataFlags_HasSizeConstraint; // Docking currently override constraints
            }

            // Amend the Appearing flag
            if window.DockTabIsVisible && !dock_tab_was_visible && dock_node_was_visible && !window.Appearing && !window_was_appearing
            {
                window.Appearing = true;
                SetWindowConditionAllowFlags(window, ImGuiCond_Appearing, true);
            }
        }
        else
        {
            window.DockIsActive = false; window.DockNodeIsVisible = false; window.DockTabIsVisible = false;
        }
    }

    // Parent window is latched only on the first call to Begin() of the frame, so further append-calls can be done from a different window stack
    let mut parent_window_in_stack: *mut ImGuiWindow =  if window.DockIsActive && window.DockNode.HostWindow.is_null() == false { window.DockNode.HostWindow} else { if g.CurrentWindowStack.empty() { null_mut()} else {g.CurrentWindowStack.last().unwrap().Window}};
    let mut parent_window: *mut ImGuiWindow =  if first_begin_of_the_frame { if flags & (ImGuiWindowFlags_ChildWindow | ImGuiWindowFlags_Popup) { parent_window_in_stack} else { null_mut()}} else { window.ParentWindow };
    // IM_ASSERT(parent_window != NULL || !(flags & ImGuiWindowFlags_ChildWindow));

    // We allow window memory to be compacted so recreate the base stack when needed.
    if window.IDStack.Size == 0 {
        window.IDStack.push(window.ID);
    }

    // Add to stack
    // We intentionally set g.CurrentWindow to NULL to prevent usage until when the viewport is set, then will call SetCurrentWindow()
    g.CurrentWindow = window;
    // ImGuiWindowStackData window_stack_data;
    let mut window_stack_data = ImGuiWindowStackData::default();
    window_stack_data.Window = window;
    window_stack_data.ParentLastItemDataBackup = g.LastItemData.clone();
    window_stack_data.StackSizesOnBegin.SetToCurrentState();
    g.CurrentWindowStack.push(window_stack_data);
    g.CurrentWindow= null_mut();
    if flag_set(flags, ImGuiWindowFlags_ChildMenu) {
        g.BeginMenuCount += 1;
    }

    if flag_set(flags, ImGuiWindowFlags_Popup)
    {
        let popup_ref = g.OpenPopupStack[g.BeginPopupStack.Size];
        popup_ref.Window = window;
        popup_ref.ParentNavLayer = parent_window_in_stack.DC.NavLayerCurrent;
        g.BeginPopupStack.push(popup_re0f32);
        window.PopupId = popup_ref.PopupId;
    }

    // Update ->RootWindow and others pointers (before any possible call to FocusWindow)
    if first_begin_of_the_frame
    {
        UpdateWindowParentAndRootLinks(window, flags, parent_window);
        window.ParentWindowInBeginStack = parent_window_in_stack;
    }

    // Process SetNextWindow***() calls
    // (FIXME: Consider splitting the HasXXX flags into X/Y components
    let mut window_pos_set_by_api: bool =  false;
    let mut window_size_x_set_by_api: bool =  false;
    let mut window_size_y_set_by_api = false;
    if flag_set(g.NextWindowData.Flags, ImGuiNextWindowDataFlags_HasPos)
    {
        window_pos_set_by_api = (window.SetWindowPosAllowFlags & g.NextWindowData.PosCond) != 0;
        if window_pos_set_by_api && ImLengthSqr(g.NextWindowData.PosPivotVal) > 0.000010f32
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
    if flag_set(g.NextWindowData.Flags , ImGuiNextWindowDataFlags_HasSize)
    {
        window_size_x_set_by_api = (window.SetWindowSizeAllowFlags & g.NextWindowData.SizeCond) != 0 && (g.NextWindowData.SizeVal.x > 0f32);
        window_size_y_set_by_api = (window.SetWindowSizeAllowFlags & g.NextWindowData.SizeCond) != 0 && (g.NextWindowData.SizeVal.y > 0f32);
        SetWindowSize(window, g.NextWindowData.SizeVal, g.NextWindowData.SizeCond);
    }
    if flag_set(g.NextWindowData.Flags , ImGuiNextWindowDataFlags_HasScroll)
    {
        if g.NextWindowData.ScrollVal.x >= 0f32
        {
            window.ScrollTarget.x = g.NextWindowData.ScrollVal.x;
            window.ScrollTargetCenterRatio.x = 0f32;
        }
        if g.NextWindowData.ScrollVal.y >= 0f32
        {
            window.ScrollTarget.y = g.NextWindowData.ScrollVal.y;
            window.ScrollTargetCenterRatio.y = 0f32;
        }
    }
    if g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasContentSize {
        window.ContentSizeExplicit = g.NextWindowData.ContentSizeVal;
    }
    else if first_begin_of_the_frame {
        window.ContentSizeExplicit = ImVec2::new2(0f32, 0f32);
    }
    if g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasWindowClass {
        window.WindowClass = g.NextWindowData.WindowClass.clone();
    }
    if g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasCollapsed {
        SetWindowCollapsed(window, g.NextWindowData.CollapsedVal, g.NextWindowData.CollapsedCond);
    }
    if g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasFocus {
        FocusWindow(window);
    }
    if window.Appearing {
        SetWindowConditionAllowFlags(window, ImGuiCond_Appearing, false);
    }
    // When reusing window again multiple times a frame, just append content (don't need to setup again)
    if first_begin_of_the_frame
    {
        // Initialize
        let window_is_child_tooltip: bool = (flags & ImGuiWindowFlags_ChildWindow) && (flags & ImGuiWindowFlags_Tooltip); // FIXME-WIP: Undocumented behavior of Child+Tooltip for pinned tooltip (#1345)
        let window_just_appearing_after_hidden_for_resize: bool = (window.HiddenFramesCannotSkipItems > 0);
        window.Active = true;
        window.HasCloseButton = (p_open != null_mut());
        window.ClipRect = ImVec4::new2(f32::MIN, f32::MIN, f32::MAX, f32::MAX);
        // window.IDStack.resize(1);
        window.DrawList._ResetForNewFrame();
        window.DC.CurrentTableIdx = -1;
        if flag_set(flags, ImGuiWindowFlags_DockNodeHost)
        {
            window.DrawList.ChannelsSplit(2);
            window.DrawList.ChannelsSetCurrent(DOCKING_HOST_DRAW_CHANNEL_FG); // Render decorations on channel 1 as we will render the backgrounds manually later
        }

        // Restore buffer capacity when woken from a compacted state, to avoid
        if (window.MemoryCompacted) {
            GcAwakeTransientWindowBuffers(window);
        }

        // Update stored window name when it changes (which can _only_ happen with the "###" operator, so the ID would stay unchanged).
        // The title bar always display the 'name' parameter, so we only update the string storage if it needs to be visible to the end-user elsewhere.
        let mut window_title_visible_elsewhere: bool =  false;
        if (window.Viewport.is_null() == false && window.Viewport.Window == window) || (window.DockIsActive) {
            window_title_visible_elsewhere = true;
        }
        else if g.NavWindowingListWindow != null_mut() && (window.Flags & ImGuiWindowFlags_NoNavFocus) == 0 {   // Window titles visible when using CTRL+TAB
            window_title_visible_elsewhere = true;
        }
        if window_title_visible_elsewhere && !window_just_created && strcmp(name, window.Name) != 0
        {
            let mut buf_len = window.NameBufLen;
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
        if window_just_created && (!window_size_x_set_by_api || !window_size_y_set_by_api) {
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
                    window.Size.x = 0f32;
                    window.SizeFull.x = 0f32;
                }
                if !window_size_y_set_by_api {
                    window.Size.y = 0f32;
                    window.SizeFull.y = 0f32;
                }
                window.ContentSize = ImVec2::default();
                window.ContentSizeIdeal = ImVec2::default();
            }
        }

        // SELECT VIEWPORT
        // We need to do this before using any style/font sizes, as viewport with a different DPI may affect font sizes.

        WindowSelectViewport(window);
        SetCurrentViewport(window, window.Viewport);
        window.FontDpiScale = if g.IO.ConfigFlags & ImGuiConfigFlags_DpiEnableScaleFonts { window.Viewport.DpiScale } else { 1f32 };
        SetCurrentWindow(window);
        flags = window.Flags;

        // LOCK BORDER SIZE AND PADDING FOR THE FRAME (so that altering them doesn't cause inconsistencies)
        // We read Style data after the call to UpdateSelectWindowViewport() which might be swapping the style.

        if flags & ImGuiWindowFlags_ChildWindow {
            window.WindowBorderSize = style.ChildBorderSize;
        }
        else {
            window.WindowBorderSize = if (flags & (ImGuiWindowFlags_Popup | ImGuiWindowFlags_Tooltip)) && (flags & ImGuiWindowFlags_Modal) {
                style.PopupBorderSize
            } else { style.WindowBorderSize };
        }
        if !window.DockIsActive && flag_set(flags, ImGuiWindowFlags_ChildWindow) && !(flags & (ImGuiWindowFlags_AlwaysUseWindowPadding | ImGuiWindowFlags_Popup)) != 0 && window.WindowBorderSize == 0f32 {
            window.WindowPadding = ImVec2::new2(0f32, if flags & ImGuiWindowFlags_MenuBar { style.WindowPadding.y } else { 0f32 });
        }
        else {
            window.WindowPadding = style.WindowPadding;
        }

        // Lock menu offset so size calculation can use it as menu-bar windows need a minimum size.
        window.DC.MenuBarOffset.x = ImMax(ImMax(window.WindowPadding.x, style.ItemSpacing.x), g.NextWindowData.MenuBarOffsetMinVal.x);
        window.DC.MenuBarOffset.y = g.NextWindowData.MenuBarOffsetMinVal.y;

        // Collapse window by double-clicking on title bar
        // At this point we don't have a clipping rectangle setup yet, so we can use the title bar area for hit detection and drawing
        if (!(flags & ImGuiWindowFlags_NoTitleBar) && !(flags & ImGuiWindowFlags_NoCollapse) && !window.DockIsActive)
        {
            // We don't use a regular button+id to test for double-click on title bar (mostly due to legacy reason, could be fixed), so verify that we don't have items over the title bar.
            let title_bar_rect: ImRect =  window.TitleBarRect();
            if (g.HoveredWindow == window && g.HoveredId == 0 && g.HoveredIdPreviousFrame == 0 && IsMouseHoveringRect(title_bar_rect.Min, title_bar_rect.Max) && g.IO.MouseClickedCount[0] == 2)
                window.WantCollapseToggle = true;
            if (window.WantCollapseToggle)
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
        let size_auto_fit: ImVec2 = CalcWindowAutoFitSize(window, window.ContentSizeIdeal);
        let mut use_current_size_for_scrollbar_x: bool =  window_just_created;
        let mut use_current_size_for_scrollbar_y: bool =  window_just_created;
        if ((flags & ImGuiWindowFlags_AlwaysAutoResize) && !window.Collapsed)
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
                window.SizeFull.x = window.AutoFitOnlyGrows ? ImMax(window.SizeFull.x, size_auto_fit.x) : size_auto_fit.x;
                use_current_size_for_scrollbar_x = true;
            }
            if (!window_size_y_set_by_api && window.AutoFitFramesY > 0)
            {
                window.SizeFull.y = window.AutoFitOnlyGrows ? ImMax(window.SizeFull.y, size_auto_fit.y) : size_auto_fit.y;
                use_current_size_for_scrollbar_y = true;
            }
            if (!window.Collapsed)
                MarkIniSettingsDirty(window);
        }

        // Apply minimum/maximum window size constraints and final size
        window.SizeFull = CalcWindowSizeAfterConstraint(window, window.SizeFull);
        window.Size = window.Collapsed && !(flags & ImGuiWindowFlags_ChildWindow) ? window.TitleBarRect().GetSize() : window.SizeFull;

        // Decoration size
        let decoration_up_height: c_float =  window.TitleBarHeight() + window.MenuBarHeight();

        // POSITION

        // Popup latch its initial position, will position itself when it appears next frame
        if (window_just_activated_by_user)
        {
            window.AutoPosLastDirection = ImGuiDir_None;
            if ((flags & ImGuiWindowFlags_Popup) != 0 && !(flags & ImGuiWindowFlags_Modal) && !window_pos_set_by_api) // FIXME: BeginPopup() could use SetNextWindowPos()
                window.Pos = g.BeginPopupStack.last().unwrap().OpenPopupPos;
        }

        // Position child window
        if (flags & ImGuiWindowFlags_ChildWindow)
        {
            // IM_ASSERT(parent_window && parent_window.Active);
            window.BeginOrderWithinParent = parent_window.DC.ChildWindows.Size;
            parent_window.DC.ChildWindows.push(window);
            if (!(flags & ImGuiWindowFlags_Popup) && !window_pos_set_by_api && !window_is_child_tooltip)
                window.Pos = parent_window.DC.CursorPos;
        }

        let window_pos_with_pivot: bool = (window.SetWindowPosVal.x != f32::MAX && window.HiddenFramesCannotSkipItems == 0);
        if (window_pos_with_pivot)
            SetWindowPos(window, window.SetWindowPosVal - window.Size * window.SetWindowPosPivot, 0); // Position given a pivot (e.g. for centering)
        else if ((flags & ImGuiWindowFlags_ChildMenu) != 0)
            window.Pos = FindBestWindowPosForPopup(window);
        else if ((flags & ImGuiWindowFlags_Popup) != 0 && !window_pos_set_by_api && window_just_appearing_after_hidden_for_resize)
            window.Pos = FindBestWindowPosForPopup(window);
        else if ((flags & ImGuiWindowFlags_Tooltip) != 0 && !window_pos_set_by_api && !window_is_child_tooltip)
            window.Pos = FindBestWindowPosForPopup(window);

        // Late create viewport if we don't fit within our current host viewport.
        if (window.ViewportAllowPlatformMonitorExtend >= 0 && !window.ViewportOwned && !(window.Viewport.Flags & ImGuiViewportFlags_Minimized))
            if (!window.Viewport.GetMainRect().Contains(window.Rect()))
            {
                // This is based on the assumption that the DPI will be known ahead (same as the DPI of the selection done in UpdateSelectWindowViewport)
                //ImGuiViewport* old_viewport = window.Viewport;
                window.Viewport = AddUpdateViewport(window, window.ID, window.Pos, window.Size, ImGuiViewportFlags_NoFocusOnAppearing);

                // FIXME-DPI
                //IM_ASSERT(old_viewport.DpiScale == window.Viewport->DpiScale); // FIXME-DPI: Something went wrong
                SetCurrentViewport(window, window.Viewport);
                window.FontDpiScale = (g.IO.ConfigFlags & ImGuiConfigFlags_DpiEnableScaleFonts) ? window.Viewport.DpiScale : 1f32;
                SetCurrentWindow(window);
            }

        if (window.ViewportOwned)
            WindowSyncOwnedViewport(window, parent_window_in_stack);

        // Calculate the range of allowed position for that window (to be movable and visible past safe area padding)
        // When clamping to stay visible, we will enforce that window.Pos stays inside of visibility_rect.
        let mut viewport_rect: ImRect = ImRect::new(window.Viewport.GetMainRect());
        let mut viewport_work_rect: ImRect = ImRect::new(window.Viewport.GetWorkRect());
        let visibility_padding: ImVec2 = ImMax(style.DisplayWindowPadding, style.DisplaySafeAreaPadding);
        let mut visibility_rect: ImRect = ImRect::new(viewport_work_rect.Min + visibility_padding, viewport_work_rect.Max - visibility_padding);

        // Clamp position/size so window stays visible within its viewport or monitor
        // Ignore zero-sized display explicitly to avoid losing positions if a window manager reports zero-sized window when initializing or minimizing.
        // FIXME: Similar to code in GetWindowAllowedExtentRect()
        if (!window_pos_set_by_api && !(flags & ImGuiWindowFlags_ChildWindow) && window.AutoFitFramesX <= 0 && window.AutoFitFramesY <= 0)
        {
            if (!window.ViewportOwned && viewport_rect.GetWidth() > 0 && viewport_rect.GetHeight() > 0f32)
            {
                ClampWindowRect(window, visibility_rect);
            }
            else if (window.ViewportOwned && g.PlatformIO.Monitors.len() > 0)
            {
                // Lost windows (e.g. a monitor disconnected) will naturally moved to the fallback/dummy monitor aka the main viewport.
                let monitor: *const ImGuiPlatformMonitor = GetViewportPlatformMonitor(window.Viewport);
                visibility_rect.Min = monitor->WorkPos + visibility_padding;
                visibility_rect.Max = monitor->WorkPos + monitor->WorkSize - visibility_padding;
                ClampWindowRect(window, visibility_rect);
            }
        }
        window.Pos = ImFloor(window.Pos);

        // Lock window rounding for the frame (so that altering them doesn't cause inconsistencies)
        // Large values tend to lead to variety of artifacts and are not recommended.
        if (window.ViewportOwned || window.DockIsActive)
            window.WindowRounding = 0f32;
        else
            window.WindowRounding = (flags & ImGuiWindowFlags_ChildWindow) ? style.ChildRounding : ((flags & ImGuiWindowFlags_Popup) && !(flags & ImGuiWindowFlags_Modal)) ? style.PopupRounding : style.WindowRounding;

        // For windows with title bar or menu bar, we clamp to FrameHeight(FontSize + FramePadding.y * 2.00f32) to completely hide artifacts.
        //if ((window.Flags & ImGuiWindowFlags_MenuBar) || !(window.Flags & ImGuiWindowFlags_NoTitleBar))
        //    window.WindowRounding = ImMin(window.WindowRounding, g.FontSize + style.FramePadding.y * 2.00f32);

        // Apply window focus (new and reactivated windows are moved to front)
        let mut want_focus: bool =  false;
        if (window_just_activated_by_user && !(flags & ImGuiWindowFlags_NoFocusOnAppearing))
        {
            if (flags & ImGuiWindowFlags_Popup)
                want_focus = true;
            else if ((window.DockIsActive || (flags & ImGuiWindowFlags_ChildWindow) == 0) && !(flags & ImGuiWindowFlags_Tooltip))
                want_focus = true;

            let mut modal: *mut ImGuiWindow =  GetTopMostPopupModal();
            if (modal != null_mut() && !IsWindowWithinBeginStackOf(window, modal))
            {
                // Avoid focusing a window that is created outside of active modal. This will prevent active modal from being closed.
                // Since window is not focused it would reappear at the same display position like the last time it was visible.
                // In case of completely new windows it would go to the top (over current modal), but input to such window would still be blocked by modal.
                // Position window behind a modal that is not a begin-parent of this window.
                want_focus = false;
                if (window == window.RootWindow)
                {
                    let mut blocking_modal: *mut ImGuiWindow =  FindBlockingModal(window);
                    // IM_ASSERT(blocking_modal != NULL);
                    BringWindowToDisplayBehind(window, blocking_modal);
                }
            }
        }

        // [Test Engine] Register whole window in the item system
// #ifdef IMGUI_ENABLE_TEST_ENGINE
        if (g.TestEngineHookItems)
        {
            // IM_ASSERT(window.IDStack.Size == 1);
            window.IDStack.Size = 0; // As window.IDStack[0] == window.ID here, make sure TestEngine doesn't erroneously see window as parent of itself.
            IMGUI_TEST_ENGINE_ITEM_ADD(window.Rect(), window.ID);
            IMGUI_TEST_ENGINE_ITEM_INFO(window.ID, window.Name, (g.HoveredWindow == window) ? ImGuiItemStatusFlags_HoveredRect : 0);
            window.IDStack.Size = 1;
        }
// #endif

        // Decide if we are going to handle borders and resize grips
        let handle_borders_and_resize_grips: bool = (window.DockNodeAsHost || !window.DockIsActive);

        // Handle manual resize: Resize Grips, Borders, Gamepad
        let border_held: c_int = -1;
        u32 resize_grip_col[4] = {};
        let resize_grip_count: c_int = g.IO.ConfigWindowsResizeFromEdges ? 2 : 1; // Allow resize from lower-left if we have the mouse cursor feedback for it.
        let resize_grip_draw_size: c_float =  IM_FLOOR(ImMax(g.FontSize * 1.10f32, window.WindowRounding + 1f32 + g.FontSize * 0.20f32));
        if (handle_borders_and_resize_grips && !window.Collapsed)
            if (UpdateWindowManualResize(window, size_auto_fit, &border_held, resize_grip_count, &resize_grip_col[0], visibility_rect))
                use_current_size_for_scrollbar_x = use_current_size_for_scrollbar_y = true;
        window.ResizeBorderHeld = border_held;

        // Synchronize window --> viewport again and one last time (clamping and manual resize may have affected either)
        if (window.ViewportOwned)
        {
            if (!window.Viewport.PlatformRequestMove)
                window.Viewport.Pos = window.Pos;
            if (!window.Viewport.PlatformRequestResize)
                window.Viewport.Size = window.Size;
            window.Viewport.UpdateWorkRect();
            viewport_rect = window.Viewport.GetMainRect();
        }

        // Save last known viewport position within the window itself (so it can be saved in .ini file and restored)
        window.ViewportPos = window.Viewport.Pos;

        // SCROLLBAR VISIBILITY

        // Update scrollbar visibility (based on the Size that was effective during last frame or the auto-resized Size).
        if (!window.Collapsed)
        {
            // When reading the current size we need to read it after size constraints have been applied.
            // When we use InnerRect here we are intentionally reading last frame size, same for ScrollbarSizes values before we set them again.
            let avail_size_from_current_frame: ImVec2 = ImVec2(window.SizeFull.x, window.SizeFull.y - decoration_up_height);
            let avail_size_from_last_frame: ImVec2 = window.InnerRect.GetSize() + window.ScrollbarSizes;
            let needed_size_from_last_frame: ImVec2 = window_just_created ? ImVec2(0, 0) : window.ContentSize + window.WindowPadding * 2.0f32;
            let size_x_for_scrollbars: c_float =  use_current_size_for_scrollbar_x ? avail_size_from_current_frame.x : avail_size_from_last_frame.x;
            let size_y_for_scrollbars: c_float =  use_current_size_for_scrollbar_y ? avail_size_from_current_frame.y : avail_size_from_last_frame.y;
            //bool scrollbar_y_from_last_frame = window.ScrollbarY; // FIXME: May want to use that in the ScrollbarX expression? How many pros vs cons?
            window.ScrollbarY = (flags & ImGuiWindowFlags_AlwaysVerticalScrollbar) || ((needed_size_from_last_frame.y > size_y_for_scrollbars) && !(flags & ImGuiWindowFlags_NoScrollbar));
            window.ScrollbarX = (flags & ImGuiWindowFlags_AlwaysHorizontalScrollbar) || ((needed_size_from_last_frame.x > size_x_for_scrollbars - (window.ScrollbarY ? style.ScrollbarSize : 0f32)) && !(flags & ImGuiWindowFlags_NoScrollbar) && (flags & ImGuiWindowFlags_HorizontalScrollbar));
            if (window.ScrollbarX && !window.ScrollbarY)
                window.ScrollbarY = (needed_size_from_last_frame.y > size_y_for_scrollbars) && !(flags & ImGuiWindowFlags_NoScrollbar);
            window.ScrollbarSizes = ImVec2(window.ScrollbarY ? style.ScrollbarSize : 0f32, window.ScrollbarX ? style.ScrollbarSize : 0f32);
        }

        // UPDATE RECTANGLES (1- THOSE NOT AFFECTED BY SCROLLING)
        // Update various regions. Variables they depends on should be set above in this function.
        // We set this up after processing the resize grip so that our rectangles doesn't lag by a frame.

        // Outer rectangle
        // Not affected by window border size. Used by:
        // - FindHoveredWindow() (w/ extra padding when border resize is enabled)
        // - Begin() initial clipping rect for drawing window background and borders.
        // - Begin() clipping whole child
        const let host_rect: ImRect =  ((flags & ImGuiWindowFlags_ChildWindow) && !(flags & ImGuiWindowFlags_Popup) && !window_is_child_tooltip) ? parent_window.ClipRect : viewport_rect;
        const let outer_rect: ImRect =  window.Rect();
        const let title_bar_rect: ImRect =  window.TitleBarRect();
        window.OuterRectClipped = outer_rect;
        if (window.DockIsActive)
            window.OuterRectClipped.Min.y += window.TitleBarHeight();
        window.OuterRectClipped.ClipWith(host_rect);

        // Inner rectangle
        // Not affected by window border size. Used by:
        // - InnerClipRect
        // - ScrollToRectEx()
        // - NavUpdatePageUpPageDown()
        // - Scrollbar()
        window.InnerRect.Min.x = window.Pos.x;
        window.InnerRect.Min.y = window.Pos.y + decoration_up_height;
        window.InnerRect.Max.x = window.Pos.x + window.Size.x - window.ScrollbarSizes.x;
        window.InnerRect.Max.y = window.Pos.y + window.Size.y - window.ScrollbarSizes.y;

        // Inner clipping rectangle.
        // Will extend a little bit outside the normal work region.
        // This is to allow e.g. Selectable or CollapsingHeader or some separators to cover that space.
        // Force round operator last to ensure that e.g. (max.x-min.x) in user's render code produce correct result.
        // Note that if our window is collapsed we will end up with an inverted (~null) clipping rectangle which is the correct behavior.
        // Affected by window/frame border size. Used by:
        // - Begin() initial clip rect
        let top_border_size: c_float =  (((flags & ImGuiWindowFlags_MenuBar) || !(flags & ImGuiWindowFlags_NoTitleBar)) ? style.FrameBorderSize : window.WindowBorderSize);
        window.InnerClipRect.Min.x = ImFloor(0.5f32 + window.InnerRect.Min.x + ImMax(ImFloor(window.WindowPadding.x * 0.5f32), window.WindowBorderSize));
        window.InnerClipRect.Min.y = ImFloor(0.5f32 + window.InnerRect.Min.y + top_border_size);
        window.InnerClipRect.Max.x = ImFloor(0.5f32 + window.InnerRect.Max.x - ImMax(ImFloor(window.WindowPadding.x * 0.5f32), window.WindowBorderSize));
        window.InnerClipRect.Max.y = ImFloor(0.5f32 + window.InnerRect.Max.y - window.WindowBorderSize);
        window.InnerClipRect.ClipWithFull(host_rect);

        // Default item width. Make it proportional to window size if window manually resizes
        if (window.Size.x > 0f32 && !(flags & ImGuiWindowFlags_Tooltip) && !(flags & ImGuiWindowFlags_AlwaysAutoResize))
            window.ItemWidthDefault = ImFloor(window.Size.x * 0.650f32);
        else
            window.ItemWidthDefault = ImFloor(g.FontSize * 16.00f32);

        // SCROLLING

        // Lock down maximum scrolling
        // The value of ScrollMax are ahead from ScrollbarX/ScrollbarY which is intentionally using InnerRect from previous rect in order to accommodate
        // for right/bottom aligned items without creating a scrollbar.
        window.ScrollMax.x = ImMax(0f32, window.ContentSize.x + window.WindowPadding.x * 2.0f32 - window.InnerRect.GetWidth());
        window.ScrollMax.y = ImMax(0f32, window.ContentSize.y + window.WindowPadding.y * 2.0f32 - window.InnerRect.GetHeight());

        // Apply scrolling
        window.Scroll = CalcNextScrollFromScrollTargetAndClamp(window);
        window.ScrollTarget = ImVec2(f32::MAX, f32::MAX);

        // DRAWING

        // Setup draw list and outer clipping rectangle
        // IM_ASSERT(window.DrawList.CmdBuffer.Size == 1 && window.DrawList.CmdBuffer[0].ElemCount == 0);
        window.DrawList.PushTextureID(g.Font.ContainerAtlas.TexID);
        PushClipRect(host_rect.Min, host_rect.Max, false);

        // Child windows can render their decoration (bg color, border, scrollbars, etc.) within their parent to save a draw call (since 1.71)
        // When using overlapping child windows, this will break the assumption that child z-order is mapped to submission order.
        // FIXME: User code may rely on explicit sorting of overlapping child window and would need to disable this somehow. Please get in contact if you are affected (github #4493)
        let is_undocked_or_docked_visible: bool = !window.DockIsActive || window.DockTabIsVisible;
        if (is_undocked_or_docked_visible)
        {
            let mut render_decorations_in_parent: bool =  false;
            if ((flags & ImGuiWindowFlags_ChildWindow) && !(flags & ImGuiWindowFlags_Popup) && !window_is_child_tooltip)
            {
                // - We test overlap with the previous child window only (testing all would end up being O(log N) not a good investment here)
                // - We disable this when the parent window has zero vertices, which is a common pattern leading to laying out multiple overlapping childs
                let mut previous_child: *mut ImGuiWindow =  parent_window.DC.ChildWindows.Size >= 2 ? parent_window.DC.ChildWindows[parent_window.DC.ChildWindows.Size - 2] : null_mut();
                let mut previous_child_overlapping: bool =  previous_child ? previous_child->Rect().Overlaps(window.Rect()) : false;
                let mut parent_is_empty: bool =  parent_window.DrawList.VtxBuffer.Size > 0;
                if (window.DrawList.CmdBuffer.last().unwrap().ElemCount == 0 && parent_is_empty && !previous_child_overlapping)
                    render_decorations_in_parent = true;
            }
            if (render_decorations_in_parent)
                window.DrawList = parent_window.DrawList;

            // Handle title bar, scrollbar, resize grips and resize borders
            let window_to_highlight: *const ImGuiWindow = g.NavWindowingTarget ? g.NavWindowingTarget : g.NavWindow;
            let title_bar_is_highlight: bool = want_focus || (window_to_highlight && (window.RootWindowForTitleBarHighlight == window_to_highlight->RootWindowForTitleBarHighlight || (window.DockNode && window.DockNode == window_to_highlight->DockNode)));
            RenderWindowDecorations(window, title_bar_rect, title_bar_is_highlight, handle_borders_and_resize_grips, resize_grip_count, resize_grip_col, resize_grip_draw_size);

            if (render_decorations_in_parent)
                window.DrawList = &window.DrawListInst;
        }

        // UPDATE RECTANGLES (2- THOSE AFFECTED BY SCROLLING)

        // Work rectangle.
        // Affected by window padding and border size. Used by:
        // - Columns() for right-most edge
        // - TreeNode(), CollapsingHeader() for right-most edge
        // - BeginTabBar() for right-most edge
        let allow_scrollbar_x: bool = !(flags & ImGuiWindowFlags_NoScrollbar) && (flags & ImGuiWindowFlags_HorizontalScrollbar);
        let allow_scrollbar_y: bool = !(flags & ImGuiWindowFlags_NoScrollbar);
        let work_rect_size_x: c_float =  (window.ContentSizeExplicit.x != 0f32 ? window.ContentSizeExplicit.x : ImMax(allow_scrollbar_x ? window.ContentSize.x : 0f32, window.Size.x - window.WindowPadding.x * 2.0f32 - window.ScrollbarSizes.x));
        let work_rect_size_y: c_float =  (window.ContentSizeExplicit.y != 0f32 ? window.ContentSizeExplicit.y : ImMax(allow_scrollbar_y ? window.ContentSize.y : 0f32, window.Size.y - window.WindowPadding.y * 2.0f32 - decoration_up_height - window.ScrollbarSizes.y));
        window.WorkRect.Min.x = ImFloor(window.InnerRect.Min.x - window.Scroll.x + ImMax(window.WindowPadding.x, window.WindowBorderSize));
        window.WorkRect.Min.y = ImFloor(window.InnerRect.Min.y - window.Scroll.y + ImMax(window.WindowPadding.y, window.WindowBorderSize));
        window.WorkRect.Max.x = window.WorkRect.Min.x + work_rect_size_x;
        window.WorkRect.Max.y = window.WorkRect.Min.y + work_rect_size_y;
        window.ParentWorkRect = window.WorkRect;

        // [LEGACY] Content Region
        // FIXME-OBSOLETE: window.ContentRegionRect.Max is currently very misleading / partly faulty, but some BeginChild() patterns relies on it.
        // Used by:
        // - Mouse wheel scrolling + many other things
        window.ContentRegionRect.Min.x = window.Pos.x - window.Scroll.x + window.WindowPadding.x;
        window.ContentRegionRect.Min.y = window.Pos.y - window.Scroll.y + window.WindowPadding.y + decoration_up_height;
        window.ContentRegionRect.Max.x = window.ContentRegionRect.Min.x + (window.ContentSizeExplicit.x != 0f32 ? window.ContentSizeExplicit.x : (window.Size.x - window.WindowPadding.x * 2.0f32 - window.ScrollbarSizes.x));
        window.ContentRegionRect.Max.y = window.ContentRegionRect.Min.y + (window.ContentSizeExplicit.y != 0f32 ? window.ContentSizeExplicit.y : (window.Size.y - window.WindowPadding.y * 2.0f32 - decoration_up_height - window.ScrollbarSizes.y));

        // Setup drawing context
        // (NB: That term "drawing context / DC" lost its meaning a long time ago. Initially was meant to hold transient data only. Nowadays difference between window. and window.DC-> is dubious.)
        window.DC.Indent.x = 0f32 + window.WindowPadding.x - window.Scroll.x;
        window.DC.GroupOffset.x = 0f32;
        window.DC.ColumnsOffset.x = 0f32;

        // Record the loss of precision of CursorStartPos which can happen due to really large scrolling amount.
        // This is used by clipper to compensate and fix the most common use case of large scroll area. Easy and cheap, next best thing compared to switching everything to double or u64.
        double start_pos_highp_x = window.Pos.x + window.WindowPadding.x - window.Scroll.x + window.DC.ColumnsOffset.x;
        double start_pos_highp_y = window.Pos.y + window.WindowPadding.y - window.Scroll.y + decoration_up_height;
        window.DC.CursorStartPos  = ImVec2(start_pos_highp_x, start_pos_highp_y);
        window.DC.CursorStartPosLossyness = ImVec2((start_pos_highp_x - window.DC.CursorStartPos.x), (start_pos_highp_y - window.DC.CursorStartPos.y));
        window.DC.CursorPos = window.DC.CursorStartPos;
        window.DC.CursorPosPrevLine = window.DC.CursorPos;
        window.DC.CursorMaxPos = window.DC.CursorStartPos;
        window.DC.IdealMaxPos = window.DC.CursorStartPos;
        window.DC.CurrLineSize = window.DC.PrevLineSize = ImVec2(0f32, 0f32);
        window.DC.CurrLineTextBaseOffset = window.DC.PrevLineTextBaseOffset = 0f32;
        window.DC.IsSameLine = window.DC.IsSetPos = false;

        window.DC.NavLayerCurrent = ImGuiNavLayer_Main;
        window.DC.NavLayersActiveMask = window.DC.NavLayersActiveMaskNext;
        window.DC.NavLayersActiveMaskNext = 0x00;
        window.DC.NavHideHighlightOneFrame = false;
        window.DC.NavHasScroll = (window.ScrollMax.y > 0f32);

        window.DC.MenuBarAppending = false;
        window.DC.MenuColumns.Update(style.ItemSpacing.x, window_just_activated_by_user);
        window.DC.TreeDepth = 0;
        window.DC.TreeJumpToParentOnPopMask = 0x00;
        window.DC.ChildWindows.clear();
        window.DC.StateStorage = &window.StateStorage;
        window.DC.CurrentColumns= null_mut();
        window.DC.LayoutType = ImGuiLayoutType_Vertical;
        window.DC.ParentLayoutType = parent_window ? parent_window.DC.LayoutType : ImGuiLayoutType_Vertical;

        window.DC.ItemWidth = window.ItemWidthDefault;
        window.DC.TextWrapPos = -1f32; // disabled
        window.DC.ItemWidthStack.clear();
        window.DC.TextWrapPosStack.clear();

        if (window.AutoFitFramesX > 0)
            window.AutoFitFramesX-= 1;
        if (window.AutoFitFramesY > 0)
            window.AutoFitFramesY-= 1;

        // Apply focus (we need to call FocusWindow() AFTER setting DC.CursorStartPos so our initial navigation reference rectangle can start around there)
        if (want_focus)
        {
            FocusWindow(window);
            NavInitWindow(window, false); // <-- this is in the way for us to be able to defer and sort reappearing FocusWindow() calls
        }

        // Close requested by platform window
        if (p_open != null_mut() && window.Viewport.PlatformRequestClose && window.Viewport != GetMainViewport())
        {
            if (!window.DockIsActive || window.DockTabIsVisible)
            {
                window.Viewport.PlatformRequestClose = false;
                g.NavWindowingToggleLayer = false; // Assume user mapped PlatformRequestClose on ALT-F4 so we disable ALT for menu toggle. False positive not an issue.
                IMGUI_DEBUG_LOG_VIEWPORT("[viewport] Window '%s' PlatformRequestClose\n", window.Name);
                *p_open = false;
            }
        }

        // Title bar
        if (!(flags & ImGuiWindowFlags_NoTitleBar) && !window.DockIsActive)
            RenderWindowTitleBarContents(window, ImRect(title_bar_rect.Min.x + window.WindowBorderSize, title_bar_rect.Min.y, title_bar_rect.Max.x - window.WindowBorderSize, title_bar_rect.Max.y), name, p_open);

        // Clear hit test shape every frame
        window.HitTestHoleSize.x = window.HitTestHoleSize.y = 0;

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
            // We need to do this _before_ we overwrite window.DC.LastItemId below because BeginDockableDragDropSource() also overwrites it.
            if ((g.MovingWindow == window) && (g.IO.ConfigDockingWithShift == g.IO.KeyShift))
                if ((window.RootWindowDockTree.Flags & ImGuiWindowFlags_NoDocking) == 0)
                    BeginDockableDragDropSource(window);

            // Docking: Any dockable window can act as a target. For dock node hosts we call BeginDockableDragDropTarget() in DockNodeUpdate() instead.
            if (g.DragDropActive && !(flags & ImGuiWindowFlags_NoDocking))
                if (g.MovingWindow == null_mut() || g.Movingwindow.RootWindowDockTree != window)
                    if ((window == window.RootWindowDockTree) && !(window.Flags & ImGuiWindowFlags_DockNodeHost))
                        BeginDockableDragDropTarget(window);
        }

        // We fill last item data based on Title Bar/Tab, in order for IsItemHovered() and IsItemActive() to be usable after Begin().
        // This is useful to allow creating context menus on title bar only, etc.
        if (window.DockIsActive)
            SetLastItemData(window.MoveId, g.CurrentItemFlags, window.DockTabItemStatusFlags, window.DockTabItemRect);
        else
            SetLastItemData(window.MoveId, g.CurrentItemFlags, IsMouseHoveringRect(title_bar_rect.Min, title_bar_rect.Max, false) ? ImGuiItemStatusFlags_HoveredRect : 0, title_bar_rect);

        // [Test Engine] Register title bar / tab
        if (!(window.Flags & ImGuiWindowFlags_NoTitleBar))
            IMGUI_TEST_ENGINE_ITEM_ADD(g.LastItemData.Rect, g.LastItemData.ID);
    }
    else
    {
        // Append
        SetCurrentViewport(window, window.Viewport);
        SetCurrentWindow(window);
    }

    // Pull/inherit current state
    window.DC.NavFocusScopeIdCurrent = (flags & ImGuiWindowFlags_ChildWindow) ? parent_window.DC.NavFocusScopeIdCurrent : window.GetID("#FOCUSSCOPE"); // Inherit from parent only // -V595

    if (!(flags & ImGuiWindowFlags_DockNodeHost))
        PushClipRect(window.InnerClipRect.Min, window.InnerClipRect.Max, true);

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
            if (window.LastFrameJustFocused == g.FrameCount)
                window.HiddenFramesCannotSkipItems = 1;
            else
                window.HiddenFramesCanSkipItems = 1;
        }

        if (flags & ImGuiWindowFlags_ChildWindow)
        {
            // Child window can be out of sight and have "negative" clip windows.
            // Mark them as collapsed so commands are skipped earlier (we can't manually collapse them because they have no title bar).
            // IM_ASSERT((flags& ImGuiWindowFlags_NoTitleBar) != 0 || (window.DockIsActive));
            if (!(flags & ImGuiWindowFlags_AlwaysAutoResize) && window.AutoFitFramesX <= 0 && window.AutoFitFramesY <= 0) // FIXME: Doesn't make sense for ChildWindow??
            {
                let nav_request: bool = (flags & ImGuiWindowFlags_NavFlattened) && (g.NavAnyRequest && g.NavWindow && g.NavWindow.RootWindowForNav == window.RootWindowForNav);
                if (!g.LogEnabled && !nav_request)
                    if (window.OuterRectClipped.Min.x >= window.OuterRectClipped.Max.x || window.OuterRectClipped.Min.y >= window.OuterRectClipped.Max.y)
                        window.HiddenFramesCanSkipItems = 1;
            }

            // Hide along with parent or if parent is collapsed
            if (parent_window && (parent_window.Collapsed || parent_window.HiddenFramesCanSkipItems > 0))
                window.HiddenFramesCanSkipItems = 1;
            if (parent_window && (parent_window.Collapsed || parent_window.HiddenFramesCannotSkipItems > 0))
                window.HiddenFramesCannotSkipItems = 1;
        }

        // Don't render if style alpha is 0.0 at the time of Begin(). This is arbitrary and inconsistent but has been there for a long while (may remove at some point)
        if (style.Alpha <= 0f32)
            window.HiddenFramesCanSkipItems = 1;

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
        if (window.Collapsed || !window.Active || hidden_regular)
            if (window.AutoFitFramesX <= 0 && window.AutoFitFramesY <= 0 && window.HiddenFramesCannotSkipItems <= 0)
                skip_items = true;
        window.SkipItems = skip_items;

        // Restore NavLayersActiveMaskNext to previous value when not visible, so a CTRL+Tab back can use a safe value.
        if (window.SkipItems)
            window.DC.NavLayersActiveMaskNext = window.DC.NavLayersActiveMask;

        // Sanity check: there are two spots which can set Appearing = true
        // - when 'window_just_activated_by_user' is set -> HiddenFramesCannotSkipItems is set -> SkipItems always false
        // - in BeginDocked() path when DockNodeIsVisible == DockTabIsVisible == true -> hidden _should_ be all zero // FIXME: Not formally proven, hence the assert.
        if (window.SkipItems && !window.Appearing)
            // IM_ASSERT(window.Appearing == false); // Please report on GitHub if this triggers: https://github.com/ocornut/imgui/issues/4177
    }

    return !window.SkipItems;
}


// BeginDisabled()/EndDisabled()
// - Those can be nested but it cannot be used to enable an already disabled section (a single BeginDisabled(true) in the stack is enough to keep everything disabled)
// - Visually this is currently altering alpha, but it is expected that in a future styling system this would work differently.
// - Feedback welcome at https://github.com/ocornut/imgui/issues/211
// - BeginDisabled(false) essentially does nothing useful but is provided to facilitate use of boolean expressions. If you can avoid calling BeginDisabled(False)/EndDisabled() best to avoid it.
// - Optimized shortcuts instead of PushStyleVar() + PushItemFlag()
c_void BeginDisabled(bool disabled)
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
