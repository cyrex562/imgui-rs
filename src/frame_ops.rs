#![allow(non_snake_case)]

use std::ptr::null_mut;
use libc::{c_float, c_int};
use crate::{CallContextHooks, GImGui, ImGuiViewport};
use crate::condition::ImGuiCond_FirstUseEver;
use crate::context_hook::{ImGuiContextHookType_EndFramePost, ImGuiContextHookType_EndFramePre, ImGuiContextHookType_NewFramePost, ImGuiContextHookType_NewFramePre, ImGuiContextHookType_PendingRemoval_};
use crate::drag_drop_flags::{ImGuiDragDropFlags_SourceAutoExpirePayload, ImGuiDragDropFlags_SourceNoPreviewTooltip};
use crate::draw_list_flags::{ImDrawListFlags_AllowVtxOffset, ImDrawListFlags_AntiAliasedFill, ImDrawListFlags_AntiAliasedLines, ImDrawListFlags_AntiAliasedLinesUseTex, ImDrawListFlags_None};
use crate::error_ops::{ErrorCheckEndFrameSanityChecks, ErrorCheckNewFrameSanityChecks};
use crate::font_atlas_flags::ImFontAtlasFlags_NoBakedLines;
use crate::font_ops::SetCurrentFont;
use crate::garbage_collection::GcCompactTransientWindowBuffers;
use {ClearActiveID, KeepAliveID};
use crate::input_ops::{IsMouseDown, UpdateInputEvents};
use crate::item_flags::ImGuiItemFlags_None;
use crate::key::ImGuiKey_Escape;
use crate::keyboard_ops::UpdateKeyboardInputs;
use crate::math_ops::{ImMax, ImMin};
use crate::mouse_cursor::ImGuiMouseCursor_Arrow;
use crate::mouse_ops::{UpdateHoveredWindowAndCaptureFlags, UpdateMouseInputs, UpdateMouseMovingWindowEndFrame, UpdateMouseMovingWindowNewFrame, UpdateMouseWheel};
use crate::platform_ime_data::ImGuiPlatformImeData;
use crate::rect::ImRect;
use crate::string_ops::str_to_const_c_char_ptr;
use crate::utils::{flag_clear, flag_set};
use crate::vec2::ImVec2;
use crate::window::focus::FocusTopMostWindowUnderOne;
use crate::window::ImGuiWindow;
use crate::window::ops::{AddWindowToSortBuffer, Begin, End, SetNextWindowSize};
use crate::window::window_flags::ImGuiWindowFlags_ChildWindow;
use crate::window_flags::ImGuiWindowFlags_ChildWindow;
use crate::window_ops::{AddWindowToSortBuffer, SetNextWindowSize};

// c_void NewFrame()
pub unsafe fn NewFrame() {
    // IM_ASSERT(GImGui != NULL && "No current context. Did you call CreateContext() and SetCurrentContext() ?");
    let g = GImGui; // ImGuiContext& g = *GImGui;

    // Remove pending delete hooks before frame start.
    // This deferred removal avoid issues of removal while iterating the hook vector
    // for (let n: c_int = g.Hooks.Size - 1; n >= 0; n--)
    for n in g.Hooks.len() - 1..0 {
        if g.Hooks[n].Type == ImGuiContextHookType_PendingRemoval_ {
            g.Hooks.erase(&g.Hooks[n]);
        }
    }

    CallContextHooks(g, ImGuiContextHookType_NewFramePre);

    // Check and assert for various common IO and Configuration mistakes
    g.ConfigFlagsLastFrame = g.ConfigFlagsCurrFrame;
    ErrorCheckNewFrameSanityChecks();
    g.ConfigFlagsCurrFrame = g.IO.ConfigFlags;

    // Load settings on first frame, save settings when modified (after a delay)
    UpdateSettings();

    g.Time += g.IO.DeltaTime;
    g.WithinFrameScope = true;
    g.FrameCount += 1;
    g.TooltipOverrideCount = 0;
    g.WindowsActiveCount = 0;
    // g.MenusIdSubmittedThisFrame.resize(0);

    // Calculate frame-rate for the user, as a purely luxurious feature
    g.FramerateSecPerFrameAccum += g.IO.DeltaTime - g.FramerateSecPerFrame[g.FramerateSecPerFrameIdx];
    g.FramerateSecPerFrame[g.FramerateSecPerFrameIdx] = g.IO.DeltaTime;
    g.FramerateSecPerFrameIdx = (g.FramerateSecPerFrameIdx + 1) % g.FramerateSecPerFrame.len();
    g.FramerateSecPerFrameCount = ImMin(g.FramerateSecPerFrameCount + 1, g.FramerateSecPerFrame.len() as c_int);
    g.IO.Framerate = (g.FramerateSecPerFrameAccum > 0.0)?(1.0 / (g.FramerateSecPerFrameAccum / g.FramerateSecPerFrameCount)): f32::MAX;

    UpdateViewportsNewFrame();

    // Setup current font and draw list shared data
    // FIXME-VIEWPORT: the concept of a single ClipRectFullscreen is not ideal!
    g.IO.Fonts.Locked = true;
    SetCurrentFont(GetDefaultFont());
    // IM_ASSERT(g.Font->IsLoaded());
    let mut virtual_space: ImRect = ImRect::from_floats(f32::MAX, f32::MAX, -f32::MAX, -f32::MAX);
    // for (let n: c_int = 0; n < g.Viewports.Size; n++)
    for n in 0..g.Viewports.len() {
        virtual_space.Add(&g.Viewports[n].GetMainRect().Min);
    }
    g.DrawListSharedData.ClipRectFullscreen = virtual_space.ToVec4();
    g.DrawListSharedData.CurveTessellationTol = g.Style.CurveTessellationTol;
    g.DrawListSharedData.SetCircleTessellationMaxError(g.Style.CircleTessellationMaxError);
    g.DrawListSharedData.InitialFlags = ImDrawListFlags_None;
    if g.Style.AntiAliasedLines {
        g.DrawListSharedData.InitialFlags |= ImDrawListFlags_AntiAliasedLines;
    }
    if g.Style.AntiAliasedLinesUseTex && flag_clear(g.Font.ContainerAtlas.Flags, ImFontAtlasFlags_NoBakedLines) {
        g.DrawListSharedData.InitialFlags |= ImDrawListFlags_AntiAliasedLinesUseTex;
    }
    if g.Style.AntiAliasedFill {
        g.DrawListSharedData.InitialFlags |= ImDrawListFlags_AntiAliasedFill;
    }
    if g.IO.BackendFlags & ImGuiBackendFlags_RendererHasVtxOffset {
        g.DrawListSharedData.InitialFlags |= ImDrawListFlags_AllowVtxOffset;
    }

    // Mark rendering data as invalid to prevent user who may have a handle on it to use it.
    // for (let n: c_int = 0; n < g.Viewports.Size; n++)
    for n in 0..g.Viewports.len() {
        let mut viewport: *mut ImGuiViewport = g.Viewports[n];
        viewport.DrawData = null_mut();
        viewport.DrawDataP.Clear();
    }

    // Drag and drop keep the source ID alive so even if the source disappear our state is consistent
    if g.DragDropActive && g.DragDropPayload.SourceId == g.ActiveId {
        KeepAliveID(g.DragDropPayload.SourceId);
    }

    // Update HoveredId data
    if !g.HoveredIdPreviousFrame {
        g.HoveredIdTimer = 0.0;
    }
    if !g.HoveredIdPreviousFrame >= 0 || (g.HoveredId != -1 && g.ActiveId == g.HoveredId) {
        g.HoveredIdNotActiveTimer = 0.0;
    }
    if g.HoveredId {
        g.HoveredIdTimer += g.IO.DeltaTime;
    }
    if g.HoveredId != -1 && g.ActiveId != g.HoveredId {
        g.HoveredIdNotActiveTimer += g.IO.DeltaTime;
    }
    g.HoveredIdPreviousFrame = g.HoveredId;
    g.HoveredIdPreviousFrameUsingMouseWheel = g.HoveredIdUsingMouseWheel;
    g.HoveredId = 0;
    g.HoveredIdAllowOverlap = false;
    g.HoveredIdUsingMouseWheel = false;
    g.HoveredIdDisabled = false;

    // Clear ActiveID if the item is not alive anymore.
    // In 1.87, the common most call to KeepAliveID() was moved from GetID() to ItemAdd().
    // As a result, custom widget using ButtonBehavior() _without_ ItemAdd() need to call KeepAliveID() themselves.
    if g.ActiveId != 0 && g.ActiveIdIsAlive != g.ActiveId && g.ActiveIdPreviousFrame == g.ActiveId {
        // IMGUI_DEBUG_LOG_ACTIVEID("NewFrame(): ClearActiveID() because it isn't marked alive anymore!\n");
        ClearActiveID();
    }

    // Update ActiveId data (clear reference to active widget if the widget isn't alive anymore)
    if g.ActiveId {
        g.ActiveIdTimer += g.IO.DeltaTime;
    }
    g.LastActiveIdTimer += g.IO.DeltaTime;
    g.ActiveIdPreviousFrame = g.ActiveId;
    g.ActiveIdPreviousFrameWindow = g.ActiveIdWindow;
    g.ActiveIdPreviousFrameHasBeenEditedBefore = g.ActiveIdHasBeenEditedBefore;
    g.ActiveIdIsAlive = 0;
    g.ActiveIdHasBeenEditedThisFrame = false;
    g.ActiveIdPreviousFrameIsAlive = false;
    g.ActiveIdIsJustActivated = false;
    if g.TempInputId != 0 && g.ActiveId != g.TempInputId {
        g.TempInputId = 0;
    }
    if g.ActiveId == 0 {
        g.ActiveIdUsingNavDirMask = 0x00;
        g.ActiveIdUsingKeyInputMask.ClearAllBits();
    }

// #ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
    if g.ActiveId == 0 {
        g.ActiveIdUsingNavInputMask = 0;
    } else if g.ActiveIdUsingNavInputMask != 0 {
        // If your custom widget code used:                 { g.ActiveIdUsingNavInputMask |= (1 << ImGuiNavInput_Cancel); }
        // Since IMGUI_VERSION_NUM >= 18804 it should be:   { SetActiveIdUsingKey(ImGuiKey_Escape); SetActiveIdUsingKey(ImGuiKey_NavGamepadCancel); }
        if g.ActiveIdUsingNavInputMask & (1 << ImGuiNavInput_Cancel) {
            SetActiveIdUsingKey(ImGuiKey_Escape);
        }
        if g.ActiveIdUsingNavInputMask & !(1 << ImGuiNavInput_Cancel) {}
        // IM_ASSERT(0); // Other values unsupported
    }
// #endif

    // Update hover delay for IsItemHovered() with delays and tooltips
    g.HoverDelayIdPreviousFrame = g.HoverDelayId;
    if g.HoverDelayId != 0 {
        //if (g.IO.MouseDelta.x == 0.0 && g.IO.MouseDelta.y == 0.0) // Need design/flags
        g.HoverDelayTimer += g.IO.DeltaTime;
        g.HoverDelayClearTimer = 0.0;
        g.HoverDelayId = 0;
    } else if g.HoverDelayTimer > 0.0 {
        // This gives a little bit of leeway before clearing the hover timer, allowing mouse to cross gaps
        g.HoverDelayClearTimer += g.IO.DeltaTime;
        if g.HoverDelayClearTimer >= ImMax(0.20, g.IO.DeltaTime * 2.0) {
            // ~6 frames at 30 Hz + allow for low framerate
            g.HoverDelayTimer = 0.0;
            g.HoverDelayClearTimer = 0.0;
        }
        // May want a decaying timer, in which case need to clamp at max first, based on max of caller last requested timer.
    }

    // Drag and drop
    g.DragDropAcceptIdPrev = g.DragDropAcceptIdCurr;
    g.DragDropAcceptIdCurr = 0;
    g.DragDropAcceptIdCurrRectSurface = f32::MAX;
    g.DragDropWithinSource = false;
    g.DragDropWithinTarget = false;
    g.DragDropHoldJustPressedId = 0;

    // Close popups on focus lost (currently wip/opt-in)
    //if (g.IO.AppFocusLost)
    //    ClosePopupsExceptModals();

    // Process input queue (trickle as many events as possible)
    // g.InputEventsTrail.resize(0);
    UpdateInputEvents(g.IO.ConfigInputTrickleEventQueue);

    // Update keyboard input state
    UpdateKeyboardInputs();

    //IM_ASSERT(g.IO.KeyCtrl == IsKeyDown(ImGuiKey_LeftCtrl) || IsKeyDown(ImGuiKey_RightCtrl));
    //IM_ASSERT(g.IO.KeyShift == IsKeyDown(ImGuiKey_LeftShift) || IsKeyDown(ImGuiKey_RightShift));
    //IM_ASSERT(g.IO.KeyAlt == IsKeyDown(ImGuiKey_LeftAlt) || IsKeyDown(ImGuiKey_RightAlt));
    //IM_ASSERT(g.IO.KeySuper == IsKeyDown(ImGuiKey_LeftSuper) || IsKeyDown(ImGuiKey_RightSuper));

    // Update gamepad/keyboard navigation
    NavUpdate();

    // Update mouse input state
    UpdateMouseInputs();

    // Undocking
    // (needs to be before UpdateMouseMovingWindowNewFrame so the window is already offset and following the mouse on the detaching frame)
    DockContextNewFrameUpdateUndocking(&g);

    // Find hovered window
    // (needs to be before UpdateMouseMovingWindowNewFrame so we fill g.HoveredWindowUnderMovingWindow on the mouse release frame)
    UpdateHoveredWindowAndCaptureFlags();

    // Handle user moving window with mouse (at the beginning of the frame to avoid input lag or sheering)
    UpdateMouseMovingWindowNewFrame();

    // Background darkening/whitening
    if GetTopMostPopupModal() != null_mut() || (g.NavWindowingTarget != null_mut() && g.NavWindowingHighlightAlpha > 0.0) {
        g.DimBgRatio = ImMin(g.DimBgRatio + g.IO.DeltaTime * 6f32, 1.0);
    } else {
        g.DimBgRatio = ImMax(g.DimBgRatio - g.IO.DeltaTime * 10f32, 0.0);
    }

    g.MouseCursor = ImGuiMouseCursor_Arrow;
    g.WantCaptureMouseNextFrame = -1;
    g.WantCaptureKeyboardNextFrame = -1;
    g.WantTextInputNextFrame = -1;

    // Platform IME data: reset for the frame
    g.PlatformImeDataPrev = g.PlatformImeData.clone();
    g.PlatformImeData.WantVisible = false;

    // Mouse wheel scrolling, scale
    UpdateMouseWheel();

    // Mark all windows as not visible and compact unused memory.
    // IM_ASSERT(g.WindowsFocusOrder.Size <= g.Windows.Size);
    let memory_compact_start_time: c_float = if g.GcCompactAll || g.IO.ConfigMemoryCompactTimer < 0.0 { f32::MAX } else { g.Time - g.IO.ConfigMemoryCompactTimer };
    // for (let i: c_int = 0; i != g.Windows.Size; i++)
    for i in 0..g.Windows.len() {
        let mut window: *mut ImGuiWindow = g.Windows[i];
        window.WasActive = window.Active;
        window.BeginCount = 0;
        window.Active = false;
        window.WriteAccessed = false;

        // Garbage collect transient buffers of recently unused windows
        if !window.WasActive && !window.MemoryCompacted && window.LastTimeActive < memory_compact_start_time {
            GcCompactTransientWindowBuffers(window);
        }
    }

    // Garbage collect transient buffers of recently unused tables
    // for (let i: c_int = 0; i < g.TablesLastTimeActive.Size; i++)
    for i in 0..g.TablesLastTimeActive.len() {
        if g.TablesLastTimeActive[i] >= 0.0 && g.TablesLastTimeActive[i] < memory_compact_start_time {
            TableGcCompactTransientBuffers(g.Tables.GetByIndex(i));
        }
    }
    // for (let i: c_int = 0; i < g.TablesTempData.Size; i++)
    for i in 0..g.TablesTempData.len() {
        if g.TablesTempData[i].LastTimeActive >= 0.0 && g.TablesTempData[i].LastTimeActive < memory_compact_start_time {
            TableGcCompactTransientBuffers(&g.TablesTempData[i]);
        }
    }
    if g.GcCompactAll {
        GcCompactTransientMiscBuffers();
    }
    g.GcCompactAll = false;

    // Closing the focused window restore focus to the first active root window in descending z-order
    if g.NavWindow.is_null() == false && !g.NavWindow.WasActive {
        FocusTopMostWindowUnderOne(null_mut(), null_mut());
    }

    // No window should be open at the beginning of the frame.
    // But in order to allow the user to call NewFrame() multiple times without calling Render(), we are doing an explicit clear.
    // g.CurrentWindowStack.resize(0);
    // g.BeginPopupStack.resize(0);
    // g.ItemFlagsStack.resize(0);
    // g.ItemFlagsStack.push(ImGuiItemFlags_None);
    // g.GroupStack.resize(0);

    // Docking
    DockContextNewFrameUpdateDocking(&g);

    // [DEBUG] Update debug features
    UpdateDebugToolItemPicker();
    UpdateDebugToolStackQueries();

    // Create implicit/fallback window - which we will only render it if the user has added something to it.
    // We don't use "Debug" to avoid colliding with user trying to create a "Debug" window with custom flags.
    // This fallback is particularly important as it avoid  calls from crashing.
    g.WithinFrameScopeWithImplicitWindow = true;
    SetNextWindowSize(&ImVec2::new(400.0, 400.0), ImGuiCond_FirstUseEver);
    Begin(str_to_const_c_char_ptr("Debug##Default"), null_mut(), 0);
    // IM_ASSERT(g.Currentwindow.IsFallbackWindow == true);

    CallContextHooks(g, ImGuiContextHookType_NewFramePost);
}


// This is normally called by Render(). You may want to call it directly if you want to avoid calling Render() but the gain will be very minimal.
// c_void EndFrame()
pub unsafe fn EndFrame() {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.Initialized);

    // Don't process EndFrame() multiple times.
    if g.FrameCountEnded == g.FrameCount {
        return;
    }
    // IM_ASSERT(g.WithinFrameScope && "Forgot to call NewFrame()?");

    CallContextHooks(g, ImGuiContextHookType_EndFramePre);

    ErrorCheckEndFrameSanityChecks();

    // Notify Platform/OS when our Input Method Editor cursor has moved (e.g. CJK inputs using Microsoft IME)
    if g.IO.SetPlatformImeDataFn != null_mut() && libc::memcmp(&g.PlatformImeData, &g.PlatformImeDataPrev, libc::sizeof(ImGuiPlatformImeData)) != 0 {
        let viewport = FindViewportByID(g.PlatformImeViewport);
        g.IO.SetPlatformImeDataFn(if viewport.is_null() == false { viewport } else { GetMainViewport() }, &g.PlatformImeData);
    }

    // Hide implicit/fallback "Debug" window if it hasn't been used
    g.WithinFrameScopeWithImplicitWindow = false;
    if g.CurrentWindow && !g.Currentwindow.WriteAccessed {
        g.Currentwindow.Active = false;
    }
    End();

    // Update navigation: CTRL+Tab, wrap-around requests
    NavEndFrame();

    // Update docking
    DockContextEndFrame(&g);

    SetCurrentViewport(null_mut(), null_mut());

    // Drag and Drop: Elapse payload (if delivered, or if source stops being submitted)
    if g.DragDropActive {
        let mut is_delivered: bool = g.DragDropPayload.Delivery;
        let mut is_elapsed: bool = (g.DragDropPayload.DataFrameCount + 1 < g.FrameCount) && ((g.DragDropSourceFlags & ImGuiDragDropFlags_SourceAutoExpirePayload) != 0 || !IsMouseDown(g.DragDropMouseButton));
        if is_delivered || is_elapsed {
            ClearDragDrop();
        }
    }

    // Drag and Drop: Fallback for source tooltip. This is not ideal but better than nothing.
    if g.DragDropActive && g.DragDropSourceFrameCount < g.FrameCount && !(g.DragDropSourceFlags & ImGuiDragDropFlags_SourceNoPreviewTooltip) != 0 {
        g.DragDropWithinSource = true;
        SetTooltip("...");
        g.DragDropWithinSource = false;
    }

    // End frame
    g.WithinFrameScope = false;
    g.FrameCountEnded = g.FrameCount;

    // Initiate moving window + handle left-click and right-click focus
    UpdateMouseMovingWindowEndFrame();

    // Update user-facing viewport list (g.Viewports -> g.PlatformIO.Viewports after filtering out some)
    UpdateViewportsEndFrame();

    // Sort the window list so that all child windows are after their parent
    // We cannot do that on FocusWindow() because children may not exist yet
    g.WindowsTempSortBuffer.clear();
    g.WindowsTempSortBuffer.reserve(g.Windows.len());
    // for (let i: c_int = 0; i != g.Windows.Size; i++)
    for i in 0..g.Windows.len() {
        let mut window: *mut ImGuiWindow = g.Windows[i];
        if window.Active && flag_set(window.Flags, ImGuiWindowFlags_ChildWindow) {     // if a child is active its parent will add it
            continue;
        }
        AddWindowToSortBuffer(&mut g.WindowsTempSortBuffer, window);
    }

    // This usually assert if there is a mismatch between the ImGuiWindowFlags_ChildWindow / ParentWindow values and DC.ChildWindows[] in parents, aka we've done something wrong.
    // IM_ASSERT(g.Windows.Size == g.WindowsTempSortBuffer.Size);
    // g.Windows.swap(g.WindowsTempSortBuffer);
    g.IO.MetricsActiveWindows = g.WindowsActiveCount;

    // Unlock font atlas
    g.IO.Fonts.Locked = false;

    // Clear Input data for next frame
    g.IO.MouseWheel = 0.0;
    g.IO.MouseWheelH = 0.0;
    g.IO.InputQueueCharacters.clear();

    CallContextHooks(g, ImGuiContextHookType_EndFramePost);
}


// GetFrameHeight: c_float()
pub unsafe fn GetFrameHeight() -> c_float {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.FontSize + g.Style.FramePadding.y * 2.0.0;
}

// GetFrameHeightWithSpacing: c_float()
pub unsafe fn GetFrameHeightWithSpacing() -> c_float {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.FontSize + g.Style.FramePadding.y * 2.0.0 + g.Style.ItemSpacing.y;
}
