// dear imgui, v1.89 WIP
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use libc;
use windows_sys::Win32;
use std::ptr::{null, null_mut};
use libc::{c_char, c_int, c_uchar, c_uint, c_void, size_t};
use crate::context::ImGuiContext;
use crate::storage::ImGuiStoragePair;

















//-----------------------------------------------------------------------------
// [SECTION] MAIN CODE (most of the code! lots of stuff, needs tidying up!)
//-----------------------------------------------------------------------------

// ImGuiWindow is mostly a dumb struct. It merely has a constructor and a few helper methods
ImGuiWindow::ImGuiWindow(ImGuiContext* context, *const char name) : DrawListInst(NULL)
{
    memset(this, 0, sizeof(*this));
    Name = ImStrdup(name);
    NameBufLen = strlen(name) + 1;
    ID = ImHashStr(name);
    IDStack.push(ID);
    ViewportAllowPlatformMonitorExtend = -1;
    ViewportPos = ImVec2(f32::MAX, f32::MAX);
    MoveId = GetID("#MOVE");
    TabId = GetID("#TAB");
    ScrollTarget = ImVec2(f32::MAX, f32::MAX);
    ScrollTargetCenterRatio = ImVec2(0.5f32, 0.5f32);
    AutoFitFramesX = AutoFitFramesY = -1;
    AutoPosLastDirection = ImGuiDir_None;
    SetWindowPosAllowFlags = SetWindowSizeAllowFlags = SetWindowCollapsedAllowFlags = SetWindowDockAllowFlags = ImGuiCond_Always | ImGuiCond_Once | ImGuiCond_FirstUseEver | ImGuiCond_Appearing;
    SetWindowPosVal = SetWindowPosPivot = ImVec2(f32::MAX, f32::MAX);
    LastFrameActive = -1;
    LastFrameJustFocused = -1;
    LastTimeActive = -1f32;
    FontWindowScale = FontDpiScale = 1f32;
    SettingsOffset = -1;
    DockOrder = -1;
    DrawList = &DrawListInst;
    DrawList._Data = &context->DrawListSharedData;
    DrawList._OwnerName = Name;
    IM_PLACEMENT_NEW(&WindowClass) ImGuiWindowClass();
}

ImGuiWindow::~ImGuiWindow()
{
    // IM_ASSERT(DrawList == &DrawListInst);
    IM_DELETE(Name);
    ColumnsStorage.clear_destruct();
}

ImGuiID ImGuiWindow::GetID(*const char str, *const char str_end)
{
    ImGuiID seed = IDStack.back();
    ImGuiID id = ImHashStr(str, str_end ? (str_end - str) : 0, seed);
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.DebugHookIdInfo == id)
        ImGui::DebugHookIdInfo(id, ImGuiDataType_String, str, str_end);
    return id;
}

ImGuiID ImGuiWindow::GetID(*const c_void ptr)
{
    ImGuiID seed = IDStack.back();
    ImGuiID id = ImHashData(&ptr, sizeof(*mut c_void), seed);
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.DebugHookIdInfo == id)
        ImGui::DebugHookIdInfo(id, ImGuiDataType_Pointer, ptr, NULL);
    return id;
}

ImGuiID ImGuiWindow::GetID(c_int n)
{
    ImGuiID seed = IDStack.back();
    ImGuiID id = ImHashData(&n, sizeof(n), seed);
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.DebugHookIdInfo == id)
        ImGui::DebugHookIdInfo(id, ImGuiDataType_S32, (*mut c_void)(intptr_t)n, NULL);
    return id;
}

// This is only used in rare/specific situations to manufacture an ID out of nowhere.
ImGuiID ImGuiWindow::GetIDFromRectangle(const ImRect& r_abs)
{
    ImGuiID seed = IDStack.back();
    ImRect r_rel = ImGui::WindowRectAbsToRel(this, r_abs);
    ImGuiID id = ImHashData(&r_rel, sizeof(r_rel), seed);
    return id;
}

static c_void SetCurrentWindow(ImGuiWindow* window)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.CurrentWindow = window;
    g.CurrentTable = window && window.DC.CurrentTableIdx != -1 ? g.Tables.GetByIndex(window.DC.CurrentTableIdx) : NULL;
    if (window)
        g.FontSize = g.DrawListSharedData.FontSize = window.CalcFontSize();
}

c_void ImGui::GcCompactTransientMiscBuffers()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.ItemFlagsStack.clear();
    g.GroupStack.clear();
    TableGcCompactSettings();
}

// Free up/compact internal window buffers, we can use this when a window becomes unused.
// Not freed:
// - ImGuiWindow, ImGuiWindowSettings, Name, StateStorage, ColumnsStorage (may hold useful data)
// This should have no noticeable visual effect. When the window reappear however, expect new allocation/buffer growth/copy cost.
c_void ImGui::GcCompactTransientWindowBuffers(ImGuiWindow* window)
{
    window.MemoryCompacted = true;
    window.MemoryDrawListIdxCapacity = window.DrawList.IdxBuffer.Capacity;
    window.MemoryDrawListVtxCapacity = window.DrawList.VtxBuffer.Capacity;
    window.IDStack.clear();
    window.DrawList._ClearFreeMemory();
    window.DC.ChildWindows.clear();
    window.DC.ItemWidthStack.clear();
    window.DC.TextWrapPosStack.clear();
}

c_void ImGui::GcAwakeTransientWindowBuffers(ImGuiWindow* window)
{
    // We stored capacity of the ImDrawList buffer to reduce growth-caused allocation/copy when awakening.
    // The other buffers tends to amortize much faster.
    window.MemoryCompacted = false;
    window.DrawList.IdxBuffer.reserve(window.MemoryDrawListIdxCapacity);
    window.DrawList.VtxBuffer.reserve(window.MemoryDrawListVtxCapacity);
    window.MemoryDrawListIdxCapacity = window.MemoryDrawListVtxCapacity = 0;
}

c_void ImGui::SetActiveID(ImGuiID id, ImGuiWindow* window)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    // While most behaved code would make an effort to not steal active id during window move/drag operations,
    // we at least need to be resilient to it. Cancelling the move is rather aggressive and users of 'master' branch
    // may prefer the weird ill-defined half working situation ('docking' did assert), so may need to rework that.
    if (g.MovingWindow != NULL && g.ActiveId == g.Movingwindow.MoveId)
    {
        IMGUI_DEBUG_LOG_ACTIVEID("SetActiveID() cancel MovingWindow\n");
        g.MovingWindow = None;
    }

    // Set active id
    g.ActiveIdIsJustActivated = (g.ActiveId != id);
    if (g.ActiveIdIsJustActivated)
    {
        IMGUI_DEBUG_LOG_ACTIVEID("SetActiveID() old:0x%08X (window \"%s\") -> new:0x%08X (window \"%s\")\n", g.ActiveId, g.ActiveIdWindow ? g.ActiveIdwindow.Name : "", id, window ? window.Name : "");
        g.ActiveIdTimer = 0f32;
        g.ActiveIdHasBeenPressedBefore = false;
        g.ActiveIdHasBeenEditedBefore = false;
        g.ActiveIdMouseButton = -1;
        if (id != 0)
        {
            g.LastActiveId = id;
            g.LastActiveIdTimer = 0f32;
        }
    }
    g.ActiveId = id;
    g.ActiveIdAllowOverlap = false;
    g.ActiveIdNoClearOnFocusLoss = false;
    g.ActiveIdWindow = window;
    g.ActiveIdHasBeenEditedThisFrame = false;
    if (id)
    {
        g.ActiveIdIsAlive = id;
        g.ActiveIdSource = (g.NavActivateId == id || g.NavActivateInputId == id || g.NavJustMovedToId == id) ? (ImGuiInputSource)ImGuiInputSource_Nav : ImGuiInputSource_Mouse;
    }

    // Clear declaration of inputs claimed by the widget
    // (Please note that this is WIP and not all keys/inputs are thoroughly declared by all widgets yet)
    g.ActiveIdUsingNavDirMask = 0x00;
    g.ActiveIdUsingKeyInputMask.ClearAllBits();
// #ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
    g.ActiveIdUsingNavInputMask = 0x00;
// #endif
}

c_void ImGui::ClearActiveID()
{
    SetActiveID(0, NULL); // g.ActiveId = 0;
}

c_void ImGui::SetHoveredID(ImGuiID id)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.HoveredId = id;
    g.HoveredIdAllowOverlap = false;
    g.HoveredIdUsingMouseWheel = false;
    if (id != 0 && g.HoveredIdPreviousFrame != id)
        g.HoveredIdTimer = g.HoveredIdNotActiveTimer = 0f32;
}

ImGuiID ImGui::GetHoveredID()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.HoveredId ? g.HoveredId : g.HoveredIdPreviousFrame;
}

// This is called by ItemAdd().
// Code not using ItemAdd() may need to call this manually otherwise ActiveId will be cleared. In IMGUI_VERSION_NUM < 18717 this was called by GetID().
c_void ImGui::KeepAliveID(ImGuiID id)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.ActiveId == id)
        g.ActiveIdIsAlive = id;
    if (g.ActiveIdPreviousFrame == id)
        g.ActiveIdPreviousFrameIsAlive = true;
}

c_void ImGui::MarkItemEdited(ImGuiID id)
{
    // This marking is solely to be able to provide info for IsItemDeactivatedAfterEdit().
    // ActiveId might have been released by the time we call this (as in the typical press/release button behavior) but still need need to fill the data.
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.ActiveId == id || g.ActiveId == 0 || g.DragDropActive);
    IM_UNUSED(id); // Avoid unused variable warnings when asserts are compiled out.
    //IM_ASSERT(g.Currentwindow.DC.LastItemId == id);
    g.ActiveIdHasBeenEditedThisFrame = true;
    g.ActiveIdHasBeenEditedBefore = true;
    g.LastItemData.StatusFlags |= ImGuiItemStatusFlags_Edited;
}

static inline bool IsWindowContentHoverable(ImGuiWindow* window, ImGuiHoveredFlags flags)
{
    // An active popup disable hovering on other windows (apart from its own children)
    // FIXME-OPT: This could be cached/stored within the window.
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.NavWindow)
        if (ImGuiWindow* focused_root_window = g.Navwindow.RootWindowDockTree)
            if (focused_root_window.WasActive && focused_root_window != window.RootWindowDockTree)
            {
                // For the purpose of those flags we differentiate "standard popup" from "modal popup"
                // NB: The order of those two tests is important because Modal windows are also Popups.
                if (focused_root_window.Flags & ImGuiWindowFlags_Modal)
                    return false;
                if ((focused_root_window.Flags & ImGuiWindowFlags_Popup) && !(flags & ImGuiHoveredFlags_AllowWhenBlockedByPopup))
                    return false;
            }

    // Filter by viewport
    if (window.Viewport != g.MouseViewport)
        if (g.MovingWindow == NULL || window.RootWindowDockTree != g.Movingwindow.RootWindowDockTree)
            return false;

    return true;
}

// This is roughly matching the behavior of internal-facing ItemHoverable()
// - we allow hovering to be true when ActiveId==window.MoveID, so that clicking on non-interactive items such as a Text() item still returns true with IsItemHovered()
// - this should work even for non-interactive items that have no ID, so we cannot use LastItemId
bool ImGui::IsItemHovered(ImGuiHoveredFlags flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if (g.NavDisableMouseHover && !g.NavDisableHighlight && !(flags & ImGuiHoveredFlags_NoNavOverride))
    {
        if ((g.LastItemData.InFlags & ImGuiItemFlags_Disabled) && !(flags & ImGuiHoveredFlags_AllowWhenDisabled))
            return false;
        if (!IsItemFocused())
            return false;
    }
    else
    {
        // Test for bounding box overlap, as updated as ItemAdd()
        ImGuiItemStatusFlags status_flags = g.LastItemData.StatusFlags;
        if (!(status_flags & ImGuiItemStatusFlags_HoveredRect))
            return false;
        // IM_ASSERT((flags & (ImGuiHoveredFlags_AnyWindow | ImGuiHoveredFlags_RootWindow | ImGuiHoveredFlags_ChildWindows | ImGuiHoveredFlags_NoPopupHierarchy | ImGuiHoveredFlags_DockHierarchy)) == 0);   // Flags not supported by this function

        // Test if we are hovering the right window (our window could be behind another window)
        // [2021/03/02] Reworked / reverted the revert, finally. Note we want e.g. BeginGroup/ItemAdd/EndGroup to work as well. (#3851)
        // [2017/10/16] Reverted commit 344d48be3 and testing RootWindow instead. I believe it is correct to NOT test for RootWindow but this leaves us unable
        // to use IsItemHovered() after EndChild() itself. Until a solution is found I believe reverting to the test from 2017/09/27 is safe since this was
        // the test that has been running for a long while.
        if (g.HoveredWindow != window && (status_flags & ImGuiItemStatusFlags_HoveredWindow) == 0)
            if ((flags & ImGuiHoveredFlags_AllowWhenOverlapped) == 0)
                return false;

        // Test if another item is active (e.g. being dragged)
        if ((flags & ImGuiHoveredFlags_AllowWhenBlockedByActiveItem) == 0)
            if (g.ActiveId != 0 && g.ActiveId != g.LastItemData.ID && !g.ActiveIdAllowOverlap)
                if (g.ActiveId != window.MoveId && g.ActiveId != window.TabId)
                    return false;

        // Test if interactions on this window are blocked by an active popup or modal.
        // The ImGuiHoveredFlags_AllowWhenBlockedByPopup flag will be tested here.
        if (!IsWindowContentHoverable(window, flags))
            return false;

        // Test if the item is disabled
        if ((g.LastItemData.InFlags & ImGuiItemFlags_Disabled) && !(flags & ImGuiHoveredFlags_AllowWhenDisabled))
            return false;

        // Special handling for calling after Begin() which represent the title bar or tab.
        // When the window is skipped/collapsed (SkipItems==true) that last item (always ->MoveId submitted by Begin)
        // will never be overwritten so we need to detect the case.
        if (g.LastItemData.ID == window.MoveId && window.WriteAccessed)
            return false;
    }

    // Handle hover delay
    // (some ideas: https://www.nngroup.com/articles/timing-exposing-content)
    c_float delay;
    if (flags & ImGuiHoveredFlags_DelayNormal)
        delay = g.IO.HoverDelayNormal;
    else if (flags & ImGuiHoveredFlags_DelayShort)
        delay = g.IO.HoverDelayShort;
    else
        delay = 0f32;
    if (delay > 0f32)
    {
        ImGuiID hover_delay_id = (g.LastItemData.ID != 0) ? g.LastItemData.ID : window.GetIDFromRectangle(g.LastItemData.Rect);
        if ((flags & ImGuiHoveredFlags_NoSharedDelay) && (g.HoverDelayIdPreviousFrame != hover_delay_id))
            g.HoverDelayTimer = 0f32;
        g.HoverDelayId = hover_delay_id;
        return g.HoverDelayTimer >= delay;
    }

    return true;
}

// Internal facing ItemHoverable() used when submitting widgets. Differs slightly from IsItemHovered().
bool ImGui::ItemHoverable(const ImRect& bb, ImGuiID id)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.HoveredId != 0 && g.HoveredId != id && !g.HoveredIdAllowOverlap)
        return false;

    let mut window = g.CurrentWindow;
    if (g.HoveredWindow != window)
        return false;
    if (g.ActiveId != 0 && g.ActiveId != id && !g.ActiveIdAllowOverlap)
        return false;
    if (!IsMouseHoveringRect(bb.Min, bb.Max))
        return false;
    if (!IsWindowContentHoverable(window, ImGuiHoveredFlags_None))
    {
        g.HoveredIdDisabled = true;
        return false;
    }

    // We exceptionally allow this function to be called with id==0 to allow using it for easy high-level
    // hover test in widgets code. We could also decide to split this function is two.
    if (id != 0)
        SetHoveredID(id);

    // When disabled we'll return false but still set HoveredId
    ImGuiItemFlags item_flags = (g.LastItemData.ID == id ? g.LastItemData.InFlags : g.CurrentItemFlags);
    if (item_flags & ImGuiItemFlags_Disabled)
    {
        // Release active id if turning disabled
        if (g.ActiveId == id)
            ClearActiveID();
        g.HoveredIdDisabled = true;
        return false;
    }

    if (id != 0)
    {
        // [DEBUG] Item Picker tool!
        // We perform the check here because SetHoveredID() is not frequently called (1~ time a frame), making
        // the cost of this tool near-zero. We can get slightly better call-stack and support picking non-hovered
        // items if we perform the test in ItemAdd(), but that would incur a small runtime cost.
        // #define IMGUI_DEBUG_TOOL_ITEM_PICKER_EX in imconfig.h if you want this check to also be performed in ItemAdd().
        if (g.DebugItemPickerActive && g.HoveredIdPreviousFrame == id)
            GetForegroundDrawList()->AddRect(bb.Min, bb.Max, IM_COL32(255, 255, 0, 255));
        if (g.DebugItemPickerBreakId == id)
            IM_DEBUG_BREAK();
    }

    if (g.NavDisableMouseHover)
        return false;

    return true;
}

bool ImGui::IsClippedEx(const ImRect& bb, ImGuiID id)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if (!bb.Overlaps(window.ClipRect))
        if (id == 0 || (id != g.ActiveId && id != g.NavId))
            if (!g.LogEnabled)
                return true;
    return false;
}

// This is also inlined in ItemAdd()
// Note: if ImGuiItemStatusFlags_HasDisplayRect is set, user needs to set window.DC.LastItemDisplayRect!
c_void ImGui::SetLastItemData(ImGuiID item_id, ImGuiItemFlags in_flags, ImGuiItemStatusFlags item_flags, const ImRect& item_rect)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.LastItemData.ID = item_id;
    g.LastItemData.InFlags = in_flags;
    g.LastItemData.StatusFlags = item_flags;
    g.LastItemData.Rect = item_rect;
}

c_float ImGui::CalcWrapWidthForPos(const ImVec2& pos, c_float wrap_pos_x)
{
    if (wrap_pos_x < 0f32)
        return 0f32;

    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if (wrap_pos_x == 0f32)
    {
        // We could decide to setup a default wrapping max point for auto-resizing windows,
        // or have auto-wrap (with unspecified wrapping pos) behave as a ContentSize extending function?
        //if (window.Hidden && (window.Flags & ImGuiWindowFlags_AlwaysAutoResize))
        //    wrap_pos_x = ImMax(window.WorkRect.Min.x + g.FontSize * 10f32, window.WorkRect.Max.x);
        //else
        wrap_pos_x = window.WorkRect.Max.x;
    }
    else if (wrap_pos_x > 0f32)
    {
        wrap_pos_x += window.Pos.x - window.Scroll.x; // wrap_pos_x is provided is window local space
    }

    return ImMax(wrap_pos_x - pos.x, 1f32);
}

// IM_ALLOC() == ImGui::MemAlloc()
*mut c_void ImGui::MemAlloc(size_t size)
{
    if (ImGuiContext* ctx = GImGui)
        ctx->IO.MetricsActiveAllocations+= 1;
    return (*GImAllocatorAllocFunc)(size, GImAllocatorUserData);
}

// IM_FREE() == ImGui::MemFree()
c_void ImGui::MemFree(ptr: *mut c_void)
{
    if (ptr)
        if (ImGuiContext* ctx = GImGui)
            ctx->IO.MetricsActiveAllocations-= 1;
    return (*GImAllocatorFreeFunc)(ptr, GImAllocatorUserData);
}

*const char ImGui::GetClipboardText()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.IO.GetClipboardTextFn ? g.IO.GetClipboardTextFn(g.IO.ClipboardUserData) : "";
}

c_void ImGui::SetClipboardText(*const char text)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.IO.SetClipboardTextFn)
        g.IO.SetClipboardTextFn(g.IO.ClipboardUserData, text);
}

*const char ImGui::GetVersion()
{
    return IMGUI_VERSION;
}

// Internal state access - if you want to share Dear ImGui state between modules (e.g. DLL) or allocate it yourself
// Note that we still point to some static data and members (such as GFontAtlas), so the state instance you end up using will point to the static data within its module
ImGuiContext* ImGui::GetCurrentContext()
{
    return GImGui;
}

c_void ImGui::SetCurrentContext(ImGuiContext* ctx)
{
// #ifdef IMGUI_SET_CURRENT_CONTEXT_FUNC
    IMGUI_SET_CURRENT_CONTEXT_FUNC(ctx); // For custom thread-based hackery you may want to have control over this.
// #else
    GImGui = ctx;
// #endif
}

c_void ImGui::SetAllocatorFunctions(ImGuiMemAllocFunc alloc_func, ImGuiMemFreeFunc free_func, user_data: *mut c_void)
{
    GImAllocatorAllocFunc = alloc_func;
    GImAllocatorFreeFunc = free_func;
    GImAllocatorUserData = user_data;
}

// This is provided to facilitate copying allocators from one static/DLL boundary to another (e.g. retrieve default allocator of your executable address space)
c_void ImGui::GetAllocatorFunctions(ImGuiMemAllocFunc* p_alloc_func, ImGuiMemFreeFunc* p_free_func, c_void** p_user_data)
{
    *p_alloc_func = GImAllocatorAllocFunc;
    *p_free_func = GImAllocatorFreeFunc;
    *p_user_data = GImAllocatorUserData;
}

ImGuiContext* ImGui::CreateContext(ImFontAtlas* shared_font_atlas)
{
    ImGuiContext* prev_ctx = GetCurrentContext();
    ImGuiContext* ctx = IM_NEW(ImGuiContext)(shared_font_atlas);
    SetCurrentContext(ctx);
    Initialize();
    if (prev_ctx != NULL)
        SetCurrentContext(prev_ctx); // Restore previous context if any, else keep new one.
    return ctx;
}

c_void ImGui::DestroyContext(ImGuiContext* ctx)
{
    ImGuiContext* prev_ctx = GetCurrentContext();
    if (ctx == NULL) //-V1051
        ctx = prev_ctx;
    SetCurrentContext(ctx);
    Shutdown();
    SetCurrentContext((prev_ctx != ctx) ? prev_ctx : NULL);
    IM_DELETE(ctx);
}

// No specific ordering/dependency support, will see as needed
ImGuiID ImGui::AddContextHook(ImGuiContext* ctx, *const ImGuiContextHook hook)
{
    ImGuiContext& g = *ctx;
    // IM_ASSERT(hook->Callback != NULL && hook->HookId == 0 && hook->Type != ImGuiContextHookType_PendingRemoval_);
    g.Hooks.push(*hook);
    g.Hooks.back().HookId = ++g.HookIdNext;
    return g.HookIdNext;
}

// Deferred removal, avoiding issue with changing vector while iterating it
c_void ImGui::RemoveContextHook(ImGuiContext* ctx, ImGuiID hook_id)
{
    ImGuiContext& g = *ctx;
    // IM_ASSERT(hook_id != 0);
    for (c_int n = 0; n < g.Hooks.Size; n++)
        if (g.Hooks[n].HookId == hook_id)
            g.Hooks[n].Type = ImGuiContextHookType_PendingRemoval_;
}

// Call context hooks (used by e.g. test engine)
// We assume a small number of hooks so all stored in same array
c_void ImGui::CallContextHooks(ImGuiContext* ctx, ImGuiContextHookType hook_type)
{
    ImGuiContext& g = *ctx;
    for (c_int n = 0; n < g.Hooks.Size; n++)
        if (g.Hooks[n].Type == hook_type)
            g.Hooks[n].Callback(&g, &g.Hooks[n]);
}

ImGuiIO& ImGui::GetIO()
{
    // IM_ASSERT(GImGui != NULL && "No current context. Did you call ImGui::CreateContext() and ImGui::SetCurrentContext() ?");
    return GimGui.IO;
}

ImGuiPlatformIO& ImGui::GetPlatformIO()
{
    // IM_ASSERT(GImGui != NULL && "No current context. Did you call ImGui::CreateContext() or ImGui::SetCurrentContext()?");
    return GimGui.PlatformIO;
}

// Pass this to your backend rendering function! Valid after Render() and until the next call to NewFrame()
ImDrawData* ImGui::GetDrawData()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    *mut ImGuiViewportP viewport = g.Viewports[0];
    return viewport->DrawDataP.Valid ? &viewport->DrawDataP : NULL;
}

double ImGui::GetTime()
{
    return GimGui.Time;
}

c_int ImGui::GetFrameCount()
{
    return GimGui.FrameCount;
}

static ImDrawList* GetViewportDrawList(*mut ImGuiViewportP viewport, size_t drawlist_no, *const char drawlist_name)
{
    // Create the draw list on demand, because they are not frequently used for all viewports
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(drawlist_no < IM_ARRAYSIZE(viewport->DrawLists));
    ImDrawList* draw_list = viewport->DrawLists[drawlist_no];
    if (draw_list == NULL)
    {
        draw_list = IM_NEW(ImDrawList)(&g.DrawListSharedData);
        draw_list->_OwnerName = drawlist_name;
        viewport->DrawLists[drawlist_no] = draw_list;
    }

    // Our ImDrawList system requires that there is always a command
    if (viewport->DrawListsLastFrame[drawlist_no] != g.FrameCount)
    {
        draw_list->_ResetForNewFrame();
        draw_list->PushTextureID(g.IO.Fonts->TexID);
        draw_list->PushClipRect(viewport->Pos, viewport->Pos + viewport->Size, false);
        viewport->DrawListsLastFrame[drawlist_no] = g.FrameCount;
    }
    return draw_list;
}

ImDrawList* ImGui::GetBackgroundDrawList(ImGuiViewport* viewport)
{
    return GetViewportDrawList((*mut ImGuiViewportP)viewport, 0, "##Background");
}

ImDrawList* ImGui::GetBackgroundDrawList()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return GetBackgroundDrawList(g.Currentwindow.Viewport);
}

ImDrawList* ImGui::GetForegroundDrawList(ImGuiViewport* viewport)
{
    return GetViewportDrawList((*mut ImGuiViewportP)viewport, 1, "##Foreground");
}

ImDrawList* ImGui::GetForegroundDrawList()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return GetForegroundDrawList(g.Currentwindow.Viewport);
}

ImDrawListSharedData* ImGui::GetDrawListSharedData()
{
    return &GimGui.DrawListSharedData;
}

c_void ImGui::StartMouseMovingWindow(ImGuiWindow* window)
{
    // Set ActiveId even if the _NoMove flag is set. Without it, dragging away from a window with _NoMove would activate hover on other windows.
    // We _also_ call this when clicking in a window empty space when io.ConfigWindowsMoveFromTitleBarOnly is set, but clear g.MovingWindow afterward.
    // This is because we want ActiveId to be set even when the window is not permitted to move.
    let g = GImGui; // ImGuiContext& g = *GImGui;
    FocusWindow(window);
    SetActiveID(window.MoveId, window);
    g.NavDisableHighlight = true;
    g.ActiveIdClickOffset = g.IO.MouseClickedPos[0] - window.RootWindowDockTree->Pos;
    g.ActiveIdNoClearOnFocusLoss = true;
    SetActiveIdUsingAllKeyboardKeys();

    let mut can_move_window: bool =  true;
    if ((window.Flags & ImGuiWindowFlags_NoMove) || (window.RootWindowDockTree->Flags & ImGuiWindowFlags_NoMove))
        can_move_window = false;
    if (ImGuiDockNode* node = window.DockNodeAsHost)
        if (node->VisibleWindow && (node->Visiblewindow.Flags & ImGuiWindowFlags_NoMove))
            can_move_window = false;
    if (can_move_window)
        g.MovingWindow = window;
}

// We use 'undock_floating_node == false' when dragging from title bar to allow moving groups of floating nodes without undocking them.
// - undock_floating_node == true: when dragging from a floating node within a hierarchy, always undock the node.
// - undock_floating_node == false: when dragging from a floating node within a hierarchy, move root window.
c_void ImGui::StartMouseMovingWindowOrNode(ImGuiWindow* window, ImGuiDockNode* node, bool undock_floating_node)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut can_undock_node: bool =  false;
    if (node != NULL && node->VisibleWindow && (node->Visiblewindow.Flags & ImGuiWindowFlags_NoMove) == 0)
    {
        // Can undock if:
        // - part of a floating node hierarchy with more than one visible node (if only one is visible, we'll just move the whole hierarchy)
        // - part of a dockspace node hierarchy (trivia: undocking from a fixed/central node will create a new node and copy windows)
        ImGuiDockNode* root_node = DockNodeGetRootNode(node);
        if (root_node->OnlyNodeWithWindows != node || root_node->CentralNode != NULL)   // -V1051 PVS-Studio thinks node should be root_node and is wrong about that.
            if (undock_floating_node || root_node->IsDockSpace())
                can_undock_node = true;
    }

    let clicked: bool = IsMouseClicked(0);
    let dragging: bool = IsMouseDragging(0, g.IO.MouseDragThreshold * 1.700f32);
    if (can_undock_node && dragging)
        DockContextQueueUndockNode(&g, node); // Will lead to DockNodeStartMouseMovingWindow() -> StartMouseMovingWindow() being called next frame
    else if (!can_undock_node && (clicked || dragging) && g.MovingWindow != window)
        StartMouseMovingWindow(window);
}

// Handle mouse moving window
// Note: moving window with the navigation keys (Square + d-pad / CTRL+TAB + Arrows) are processed in NavUpdateWindowing()
// FIXME: We don't have strong guarantee that g.MovingWindow stay synched with g.ActiveId == g.Movingwindow.MoveId.
// This is currently enforced by the fact that BeginDragDropSource() is setting all g.ActiveIdUsingXXXX flags to inhibit navigation inputs,
// but if we should more thoroughly test cases where g.ActiveId or g.MovingWindow gets changed and not the other.
c_void ImGui::UpdateMouseMovingWindowNewFrame()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.MovingWindow != NULL)
    {
        // We actually want to move the root window. g.MovingWindow == window we clicked on (could be a child window).
        // We track it to preserve Focus and so that generally ActiveIdWindow == MovingWindow and ActiveId == Movingwindow.MoveId for consistency.
        KeepAliveID(g.ActiveId);
        // IM_ASSERT(g.MovingWindow && g.Movingwindow.RootWindowDockTree);
        ImGuiWindow* moving_window = g.Movingwindow.RootWindowDockTree;

        // When a window stop being submitted while being dragged, it may will its viewport until next Begin()
        let window_disappared: bool = ((!moving_window.WasActive && !moving_window.Active) || moving_window.Viewport == NULL);
        if (g.IO.MouseDown[0] && IsMousePosValid(&g.IO.MousePos) && !window_disappared)
        {
            ImVec2 pos = g.IO.MousePos - g.ActiveIdClickOffset;
            if (moving_window.Pos.x != pos.x || moving_window.Pos.y != pos.y)
            {
                SetWindowPos(moving_window, pos, ImGuiCond_Always);
                if (moving_window.ViewportOwned) // Synchronize viewport immediately because some overlays may relies on clipping rectangle before we Begin() into the window.
                {
                    moving_window.Viewport->Pos = pos;
                    moving_window.Viewport->UpdateWorkRect();
                }
            }
            FocusWindow(g.MovingWindow);
        }
        else
        {
            if (!window_disappared)
            {
                // Try to merge the window back into the main viewport.
                // This works because MouseViewport should be != Movingwindow.Viewport on release (as per code in UpdateViewports)
                if (g.ConfigFlagsCurrFrame & ImGuiConfigFlags_ViewportsEnable)
                    UpdateTryMergeWindowIntoHostViewport(moving_window, g.MouseViewport);

                // Restore the mouse viewport so that we don't hover the viewport _under_ the moved window during the frame we released the mouse button.
                if (!IsDragDropPayloadBeingAccepted())
                    g.MouseViewport = moving_window.Viewport;

                // Clear the NoInput window flag set by the Viewport system
                moving_window.Viewport->Flags &= ~ImGuiViewportFlags_NoInputs; // FIXME-VIEWPORT: Test engine managed to crash here because Viewport was NULL.
            }

            g.MovingWindow = None;
            ClearActiveID();
        }
    }
    else
    {
        // When clicking/dragging from a window that has the _NoMove flag, we still set the ActiveId in order to prevent hovering others.
        if (g.ActiveIdWindow && g.ActiveIdwindow.MoveId == g.ActiveId)
        {
            KeepAliveID(g.ActiveId);
            if (!g.IO.MouseDown[0])
                ClearActiveID();
        }
    }
}

// Initiate moving window when clicking on empty space or title bar.
// Handle left-click and right-click focus.
c_void ImGui::UpdateMouseMovingWindowEndFrame()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.ActiveId != 0 || g.HoveredId != 0)
        return;

    // Unless we just made a window/popup appear
    if (g.NavWindow && g.Navwindow.Appearing)
        return;

    // Click on empty space to focus window and start moving
    // (after we're done with all our widgets, so e.g. clicking on docking tab-bar which have set HoveredId already and not get us here!)
    if (g.IO.MouseClicked[0])
    {
        // Handle the edge case of a popup being closed while clicking in its empty space.
        // If we try to focus it, FocusWindow() > ClosePopupsOverWindow() will accidentally close any parent popups because they are not linked together any more.
        ImGuiWindow* root_window = g.HoveredWindow ? g.Hoveredwindow.RootWindow : NULL;
        let is_closed_popup: bool = root_window && (root_window.Flags & ImGuiWindowFlags_Popup) && !IsPopupOpen(root_window.PopupId, ImGuiPopupFlags_AnyPopupLevel);

        if (root_window != NULL && !is_closed_popup)
        {
            StartMouseMovingWindow(g.HoveredWindow); //-V595

            // Cancel moving if clicked outside of title bar
            if (g.IO.ConfigWindowsMoveFromTitleBarOnly)
                if (!(root_window.Flags & ImGuiWindowFlags_NoTitleBar) || root_window.DockIsActive)
                    if (!root_window.TitleBarRect().Contains(g.IO.MouseClickedPos[0]))
                        g.MovingWindow = None;

            // Cancel moving if clicked over an item which was disabled or inhibited by popups (note that we know HoveredId == 0 already)
            if (g.HoveredIdDisabled)
                g.MovingWindow = None;
        }
        else if (root_window == NULL && g.NavWindow != NULL && GetTopMostPopupModal() == NULL)
        {
            // Clicking on void disable focus
            FocusWindow(NULL);
        }
    }

    // With right mouse button we close popups without changing focus based on where the mouse is aimed
    // Instead, focus will be restored to the window under the bottom-most closed popup.
    // (The left mouse button path calls FocusWindow on the hovered window, which will lead NewFrame->ClosePopupsOverWindow to trigger)
    if (g.IO.MouseClicked[1])
    {
        // Find the top-most window between HoveredWindow and the top-most Modal Window.
        // This is where we can trim the popup stack.
        ImGuiWindow* modal = GetTopMostPopupModal();
        let mut hovered_window_above_modal: bool =  g.HoveredWindow && (modal == NULL || IsWindowAbove(g.HoveredWindow, modal));
        ClosePopupsOverWindow(hovered_window_above_modal ? g.HoveredWindow : modal, true);
    }
}

// This is called during NewFrame()->UpdateViewportsNewFrame() only.
// Need to keep in sync with SetWindowPos()
static c_void TranslateWindow(ImGuiWindow* window, const ImVec2& delta)
{
    window.Pos += delta;
    window.ClipRect.Translate(delta);
    window.OuterRectClipped.Translate(delta);
    window.InnerRect.Translate(delta);
    window.DC.CursorPos += delta;
    window.DC.CursorStartPos += delta;
    window.DC.CursorMaxPos += delta;
    window.DC.IdealMaxPos += delta;
}

static c_void ScaleWindow(ImGuiWindow* window, c_float scale)
{
    ImVec2 origin = window.Viewport->Pos;
    window.Pos = ImFloor((window.Pos - origin) * scale + origin);
    window.Size = ImFloor(window.Size * scale);
    window.SizeFull = ImFloor(window.SizeFull * scale);
    window.ContentSize = ImFloor(window.ContentSize * scale);
}

static bool IsWindowActiveAndVisible(ImGuiWindow* window)
{
    return (window.Active) && (!window.Hidden);
}

static c_void UpdateAliasKey(ImGuiKey key, bool v, c_float analog_value)
{
    // IM_ASSERT(ImGui::IsAliasKey(key));
    ImGuiKeyData* key_data = ImGui::GetKeyData(key);
    key_data->Down = v;
    key_data->AnalogValue = analog_value;
}

static c_void ImGui::UpdateKeyboardInputs()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiIO& io = g.IO;

    // Import legacy keys or verify they are not used
// #ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
    if (io.BackendUsingLegacyKeyArrays == 0)
    {
        // Backend used new io.AddKeyEvent() API: Good! Verify that old arrays are never written to externally.
        for (c_int n = 0; n < ImGuiKey_LegacyNativeKey_END; n++)
            // IM_ASSERT((io.KeysDown[n] == false || IsKeyDown(n)) && "Backend needs to either only use io.AddKeyEvent(), either only fill legacy io.KeysDown[] + io.KeyMap[]. Not both!");
    }
    else
    {
        if (g.FrameCount == 0)
            for (c_int n = ImGuiKey_LegacyNativeKey_BEGIN; n < ImGuiKey_LegacyNativeKey_END; n++)
                // IM_ASSERT(g.IO.KeyMap[n] == -1 && "Backend is not allowed to write to io.KeyMap[0..511]!");

        // Build reverse KeyMap (Named -> Legacy)
        for (c_int n = ImGuiKey_NamedKey_BEGIN; n < ImGuiKey_NamedKey_END; n++)
            if (io.KeyMap[n] != -1)
            {
                // IM_ASSERT(IsLegacyKey((ImGuiKey)io.KeyMap[n]));
                io.KeyMap[io.KeyMap[n]] = n;
            }

        // Import legacy keys into new ones
        for (c_int n = ImGuiKey_LegacyNativeKey_BEGIN; n < ImGuiKey_LegacyNativeKey_END; n++)
            if (io.KeysDown[n] || io.BackendUsingLegacyKeyArrays == 1)
            {
                const ImGuiKey key = (ImGuiKey)(io.KeyMap[n] != -1 ? io.KeyMap[n] : n);
                // IM_ASSERT(io.KeyMap[n] == -1 || IsNamedKey(key));
                io.KeysData[key].Down = io.KeysDown[n];
                if (key != n)
                    io.KeysDown[key] = io.KeysDown[n]; // Allow legacy code using io.KeysDown[GetKeyIndex()] with old backends
                io.BackendUsingLegacyKeyArrays = 1;
            }
        if (io.BackendUsingLegacyKeyArrays == 1)
        {
            io.KeysData[ImGuiKey_ModCtrl].Down = io.KeyCtrl;
            io.KeysData[ImGuiKey_ModShift].Down = io.KeyShift;
            io.KeysData[ImGuiKey_ModAlt].Down = io.KeyAlt;
            io.KeysData[ImGuiKey_ModSuper].Down = io.KeySuper;
        }
    }

// #ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
    let nav_gamepad_active: bool = (io.ConfigFlags & ImGuiConfigFlags_NavEnableGamepad) != 0 && (io.BackendFlags & ImGuiBackendFlags_HasGamepad) != 0;
    if (io.BackendUsingLegacyNavInputArray && nav_gamepad_active)
    {
        #define MAP_LEGACY_NAV_INPUT_TO_KEY1(_KEY, _NAV1)           do { io.KeysData[_KEY].Down = (io.NavInputs[_NAV1] > 0f32); io.KeysData[_KEY].AnalogValue = io.NavInputs[_NAV1]; } while (0)
        #define MAP_LEGACY_NAV_INPUT_TO_KEY2(_KEY, _NAV1, _NAV2)    do { io.KeysData[_KEY].Down = (io.NavInputs[_NAV1] > 0f32) || (io.NavInputs[_NAV2] > 0f32); io.KeysData[_KEY].AnalogValue = ImMax(io.NavInputs[_NAV1], io.NavInputs[_NAV2]); } while (0)
        MAP_LEGACY_NAV_INPUT_TO_KEY1(ImGuiKey_GamepadFaceDown, ImGuiNavInput_Activate);
        MAP_LEGACY_NAV_INPUT_TO_KEY1(ImGuiKey_GamepadFaceRight, ImGuiNavInput_Cancel);
        MAP_LEGACY_NAV_INPUT_TO_KEY1(ImGuiKey_GamepadFaceLeft, ImGuiNavInput_Menu);
        MAP_LEGACY_NAV_INPUT_TO_KEY1(ImGuiKey_GamepadFaceUp, ImGuiNavInput_Input);
        MAP_LEGACY_NAV_INPUT_TO_KEY1(ImGuiKey_GamepadDpadLeft, ImGuiNavInput_DpadLeft);
        MAP_LEGACY_NAV_INPUT_TO_KEY1(ImGuiKey_GamepadDpadRight, ImGuiNavInput_DpadRight);
        MAP_LEGACY_NAV_INPUT_TO_KEY1(ImGuiKey_GamepadDpadUp, ImGuiNavInput_DpadUp);
        MAP_LEGACY_NAV_INPUT_TO_KEY1(ImGuiKey_GamepadDpadDown, ImGuiNavInput_DpadDown);
        MAP_LEGACY_NAV_INPUT_TO_KEY2(ImGuiKey_GamepadL1, ImGuiNavInput_FocusPrev, ImGuiNavInput_TweakSlow);
        MAP_LEGACY_NAV_INPUT_TO_KEY2(ImGuiKey_GamepadR1, ImGuiNavInput_FocusNext, ImGuiNavInput_TweakFast);
        MAP_LEGACY_NAV_INPUT_TO_KEY1(ImGuiKey_GamepadLStickLeft, ImGuiNavInput_LStickLeft);
        MAP_LEGACY_NAV_INPUT_TO_KEY1(ImGuiKey_GamepadLStickRight, ImGuiNavInput_LStickRight);
        MAP_LEGACY_NAV_INPUT_TO_KEY1(ImGuiKey_GamepadLStickUp, ImGuiNavInput_LStickUp);
        MAP_LEGACY_NAV_INPUT_TO_KEY1(ImGuiKey_GamepadLStickDown, ImGuiNavInput_LStickDown);
        #undef NAV_MAP_KEY
    }
// #endif

// #endif

    // Synchronize io.KeyMods with individual modifiers io.KeyXXX bools, update aliases
    io.KeyMods = GetMergedModFlags();
    for (c_int n = 0; n < ImGuiMouseButton_COUNT; n++)
        UpdateAliasKey(MouseButtonToKey(n), io.MouseDown[n], io.MouseDown[n] ? 1f32 : 0f32);
    UpdateAliasKey(ImGuiKey_MouseWheelX, io.MouseWheelH != 0f32, io.MouseWheelH);
    UpdateAliasKey(ImGuiKey_MouseWheelY, io.MouseWheel != 0f32, io.MouseWheel);

    // Clear gamepad data if disabled
    if ((io.BackendFlags & ImGuiBackendFlags_HasGamepad) == 0)
        for (c_int i = ImGuiKey_Gamepad_BEGIN; i < ImGuiKey_Gamepad_END; i++)
        {
            io.KeysData[i - ImGuiKey_KeysData_OFFSET].Down = false;
            io.KeysData[i - ImGuiKey_KeysData_OFFSET].AnalogValue = 0f32;
        }

    // Update keys
    for (c_int i = 0; i < IM_ARRAYSIZE(io.KeysData); i++)
    {
        ImGuiKeyData* key_data = &io.KeysData[i];
        key_data->DownDurationPrev = key_data->DownDuration;
        key_data->DownDuration = key_data->Down ? (key_data->DownDuration < 0f32 ? 0f32 : key_data->DownDuration + io.DeltaTime) : -1f32;
    }
}

static c_void ImGui::UpdateMouseInputs()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiIO& io = g.IO;

    // Round mouse position to avoid spreading non-rounded position (e.g. UpdateManualResize doesn't support them well)
    if (IsMousePosValid(&io.MousePos))
        io.MousePos = g.MouseLastValidPos = ImFloorSigned(io.MousePos);

    // If mouse just appeared or disappeared (usually denoted by -f32::MAX components) we cancel out movement in MouseDelta
    if (IsMousePosValid(&io.MousePos) && IsMousePosValid(&io.MousePosPrev))
        io.MouseDelta = io.MousePos - io.MousePosPrev;
    else
        io.MouseDelta = ImVec2(0f32, 0f32);

    // If mouse moved we re-enable mouse hovering in case it was disabled by gamepad/keyboard. In theory should use a >0f32 threshold but would need to reset in everywhere we set this to true.
    if (io.MouseDelta.x != 0f32 || io.MouseDelta.y != 0f32)
        g.NavDisableMouseHover = false;

    io.MousePosPrev = io.MousePos;
    for (c_int i = 0; i < IM_ARRAYSIZE(io.MouseDown); i++)
    {
        io.MouseClicked[i] = io.MouseDown[i] && io.MouseDownDuration[i] < 0f32;
        io.MouseClickedCount[i] = 0; // Will be filled below
        io.MouseReleased[i] = !io.MouseDown[i] && io.MouseDownDuration[i] >= 0f32;
        io.MouseDownDurationPrev[i] = io.MouseDownDuration[i];
        io.MouseDownDuration[i] = io.MouseDown[i] ? (io.MouseDownDuration[i] < 0f32 ? 0f32 : io.MouseDownDuration[i] + io.DeltaTime) : -1f32;
        if (io.MouseClicked[i])
        {
            let mut is_repeated_click: bool =  false;
            if ((g.Time - io.MouseClickedTime[i]) < io.MouseDoubleClickTime)
            {
                ImVec2 delta_from_click_pos = IsMousePosValid(&io.MousePos) ? (io.MousePos - io.MouseClickedPos[i]) : ImVec2(0f32, 0f32);
                if (ImLengthSqr(delta_from_click_pos) < io.MouseDoubleClickMaxDist * io.MouseDoubleClickMaxDist)
                    is_repeated_click = true;
            }
            if (is_repeated_click)
                io.MouseClickedLastCount[i]+= 1;
            else
                io.MouseClickedLastCount[i] = 1;
            io.MouseClickedTime[i] = g.Time;
            io.MouseClickedPos[i] = io.MousePos;
            io.MouseClickedCount[i] = io.MouseClickedLastCount[i];
            io.MouseDragMaxDistanceAbs[i] = ImVec2(0f32, 0f32);
            io.MouseDragMaxDistanceSqr[i] = 0f32;
        }
        else if (io.MouseDown[i])
        {
            // Maintain the maximum distance we reaching from the initial click position, which is used with dragging threshold
            ImVec2 delta_from_click_pos = IsMousePosValid(&io.MousePos) ? (io.MousePos - io.MouseClickedPos[i]) : ImVec2(0f32, 0f32);
            io.MouseDragMaxDistanceSqr[i] = ImMax(io.MouseDragMaxDistanceSqr[i], ImLengthSqr(delta_from_click_pos));
            io.MouseDragMaxDistanceAbs[i].x = ImMax(io.MouseDragMaxDistanceAbs[i].x, delta_from_click_pos.x < 0f32 ? -delta_from_click_pos.x : delta_from_click_pos.x);
            io.MouseDragMaxDistanceAbs[i].y = ImMax(io.MouseDragMaxDistanceAbs[i].y, delta_from_click_pos.y < 0f32 ? -delta_from_click_pos.y : delta_from_click_pos.y);
        }

        // We provide io.MouseDoubleClicked[] as a legacy service
        io.MouseDoubleClicked[i] = (io.MouseClickedCount[i] == 2);

        // Clicking any mouse button reactivate mouse hovering which may have been deactivated by gamepad/keyboard navigation
        if (io.MouseClicked[i])
            g.NavDisableMouseHover = false;
    }
}

static c_void StartLockWheelingWindow(ImGuiWindow* window)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.WheelingWindow == window)
        return;
    g.WheelingWindow = window;
    g.WheelingWindowRefMousePos = g.IO.MousePos;
    g.WheelingWindowTimer = WINDOWS_MOUSE_WHEEL_SCROLL_LOCK_TIMER;
}

c_void ImGui::UpdateMouseWheel()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    // Reset the locked window if we move the mouse or after the timer elapses
    if (g.WheelingWindow != NULL)
    {
        g.WheelingWindowTimer -= g.IO.DeltaTime;
        if (IsMousePosValid() && ImLengthSqr(g.IO.MousePos - g.WheelingWindowRefMousePos) > g.IO.MouseDragThreshold * g.IO.MouseDragThreshold)
            g.WheelingWindowTimer = 0f32;
        if (g.WheelingWindowTimer <= 0f32)
        {
            g.WheelingWindow = None;
            g.WheelingWindowTimer = 0f32;
        }
    }

    let hovered_id_using_mouse_wheel: bool = (g.HoveredIdPreviousFrame != 0 && g.HoveredIdPreviousFrameUsingMouseWheel);
    let active_id_using_mouse_wheel_x: bool = g.ActiveIdUsingKeyInputMask.TestBit(ImGuiKey_MouseWheelX);
    let active_id_using_mouse_wheel_y: bool = g.ActiveIdUsingKeyInputMask.TestBit(ImGuiKey_MouseWheelY);

    c_float wheel_x = (!hovered_id_using_mouse_wheel && !active_id_using_mouse_wheel_x) ? g.IO.MouseWheelH : 0f32;
    c_float wheel_y = (!hovered_id_using_mouse_wheel && !active_id_using_mouse_wheel_y) ? g.IO.MouseWheel : 0;
    if (wheel_x == 0f32 && wheel_y == 0f32)
        return;

    ImGuiWindow* window = g.WheelingWindow ? g.WheelingWindow : g.HoveredWindow;
    if (!window || window.Collapsed)
        return;

    // Zoom / Scale window
    // FIXME-OBSOLETE: This is an old feature, it still works but pretty much nobody is using it and may be best redesigned.
    if (wheel_y != 0f32 && g.IO.KeyCtrl && g.IO.FontAllowUserScaling)
    {
        StartLockWheelingWindow(window);
        const c_float new_font_scale = ImClamp(window.FontWindowScale + g.IO.MouseWheel * 0.1f32, 0.50f32, 2.500f32);
        const c_float scale = new_font_scale / window.FontWindowScale;
        window.FontWindowScale = new_font_scale;
        if (window == window.RootWindow)
        {
            const ImVec2 offset = window.Size * (1f32 - scale) * (g.IO.MousePos - window.Pos) / window.Size;
            SetWindowPos(window, window.Pos + offset, 0);
            window.Size = ImFloor(window.Size * scale);
            window.SizeFull = ImFloor(window.SizeFull * scale);
        }
        return;
    }

    // Mouse wheel scrolling
    // If a child window has the ImGuiWindowFlags_NoScrollWithMouse flag, we give a chance to scroll its parent
    if (g.IO.KeyCtrl)
        return;

    // As a standard behavior holding SHIFT while using Vertical Mouse Wheel triggers Horizontal scroll instead
    // (we avoid doing it on OSX as it the OS input layer handles this already)
    let swap_axis: bool = g.IO.KeyShift && !g.IO.ConfigMacOSXBehaviors;
    if (swap_axis)
    {
        wheel_x = wheel_y;
        wheel_y = 0f32;
    }

    // Vertical Mouse Wheel scrolling
    if (wheel_y != 0f32)
    {
        StartLockWheelingWindow(window);
        while ((window.Flags & ImGuiWindowFlags_ChildWindow) && ((window.ScrollMax.y == 0f32) || ((window.Flags & ImGuiWindowFlags_NoScrollWithMouse) && !(window.Flags & ImGuiWindowFlags_NoMouseInputs))))
            window = window.ParentWindow;
        if (!(window.Flags & ImGuiWindowFlags_NoScrollWithMouse) && !(window.Flags & ImGuiWindowFlags_NoMouseInputs))
        {
            c_float max_step = window.InnerRect.GetHeight() * 0.67f;
            c_float scroll_step = ImFloor(ImMin(5 * window.CalcFontSize(), max_step));
            SetScrollY(window, window.Scroll.y - wheel_y * scroll_step);
        }
    }

    // Horizontal Mouse Wheel scrolling, or Vertical Mouse Wheel w/ Shift held
    if (wheel_x != 0f32)
    {
        StartLockWheelingWindow(window);
        while ((window.Flags & ImGuiWindowFlags_ChildWindow) && ((window.ScrollMax.x == 0f32) || ((window.Flags & ImGuiWindowFlags_NoScrollWithMouse) && !(window.Flags & ImGuiWindowFlags_NoMouseInputs))))
            window = window.ParentWindow;
        if (!(window.Flags & ImGuiWindowFlags_NoScrollWithMouse) && !(window.Flags & ImGuiWindowFlags_NoMouseInputs))
        {
            c_float max_step = window.InnerRect.GetWidth() * 0.67f;
            c_float scroll_step = ImFloor(ImMin(2 * window.CalcFontSize(), max_step));
            SetScrollX(window, window.Scroll.x - wheel_x * scroll_step);
        }
    }
}

// The reason this is exposed in imgui_internal.h is: on touch-based system that don't have hovering, we want to dispatch inputs to the right target (imgui vs imgui+app)
c_void ImGui::UpdateHoveredWindowAndCaptureFlags()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiIO& io = g.IO;
    g.WindowsHoverPadding = ImMax(g.Style.TouchExtraPadding, ImVec2(WINDOWS_HOVER_PADDING, WINDOWS_HOVER_PADDING));

    // Find the window hovered by mouse:
    // - Child windows can extend beyond the limit of their parent so we need to derive HoveredRootWindow from HoveredWindow.
    // - When moving a window we can skip the search, which also conveniently bypasses the fact that window.WindowRectClipped is lagging as this point of the frame.
    // - We also support the moved window toggling the NoInputs flag after moving has started in order to be able to detect windows below it, which is useful for e.g. docking mechanisms.
    let mut clear_hovered_windows: bool =  false;
    FindHoveredWindow();
    // IM_ASSERT(g.HoveredWindow == NULL || g.HoveredWindow == g.MovingWindow || g.Hoveredwindow.Viewport == g.MouseViewport);

    // Modal windows prevents mouse from hovering behind them.
    ImGuiWindow* modal_window = GetTopMostPopupModal();
    if (modal_window && g.HoveredWindow && !IsWindowWithinBeginStackOf(g.Hoveredwindow.RootWindow, modal_window)) // FIXME-MERGE: RootWindowDockTree ?
        clear_hovered_windows = true;

    // Disabled mouse?
    if (io.ConfigFlags & ImGuiConfigFlags_NoMouse)
        clear_hovered_windows = true;

    // We track click ownership. When clicked outside of a window the click is owned by the application and
    // won't report hovering nor request capture even while dragging over our windows afterward.
    let has_open_popup: bool = (g.OpenPopupStack.Size > 0);
    let has_open_modal: bool = (modal_window != NULL);
    c_int mouse_earliest_down = -1;
    let mut mouse_any_down: bool =  false;
    for (c_int i = 0; i < IM_ARRAYSIZE(io.MouseDown); i++)
    {
        if (io.MouseClicked[i])
        {
            io.MouseDownOwned[i] = (g.HoveredWindow != NULL) || has_open_popup;
            io.MouseDownOwnedUnlessPopupClose[i] = (g.HoveredWindow != NULL) || has_open_modal;
        }
        mouse_any_down |= io.MouseDown[i];
        if (io.MouseDown[i])
            if (mouse_earliest_down == -1 || io.MouseClickedTime[i] < io.MouseClickedTime[mouse_earliest_down])
                mouse_earliest_down = i;
    }
    let mouse_avail: bool = (mouse_earliest_down == -1) || io.MouseDownOwned[mouse_earliest_down];
    let mouse_avail_unless_popup_close: bool = (mouse_earliest_down == -1) || io.MouseDownOwnedUnlessPopupClose[mouse_earliest_down];

    // If mouse was first clicked outside of ImGui bounds we also cancel out hovering.
    // FIXME: For patterns of drag and drop across OS windows, we may need to rework/remove this test (first committed 311c0ca9 on 2015/02)
    let mouse_dragging_extern_payload: bool = g.DragDropActive && (g.DragDropSourceFlags & ImGuiDragDropFlags_SourceExtern) != 0;
    if (!mouse_avail && !mouse_dragging_extern_payload)
        clear_hovered_windows = true;

    if (clear_hovered_windows)
        g.HoveredWindow = g.HoveredWindowUnderMovingWindow = None;

    // Update io.WantCaptureMouse for the user application (true = dispatch mouse info to Dear ImGui only, false = dispatch mouse to Dear ImGui + underlying app)
    // Update io.WantCaptureMouseAllowPopupClose (experimental) to give a chance for app to react to popup closure with a drag
    if (g.WantCaptureMouseNextFrame != -1)
    {
        io.WantCaptureMouse = io.WantCaptureMouseUnlessPopupClose = (g.WantCaptureMouseNextFrame != 0);
    }
    else
    {
        io.WantCaptureMouse = (mouse_avail && (g.HoveredWindow != NULL || mouse_any_down)) || has_open_popup;
        io.WantCaptureMouseUnlessPopupClose = (mouse_avail_unless_popup_close && (g.HoveredWindow != NULL || mouse_any_down)) || has_open_modal;
    }

    // Update io.WantCaptureKeyboard for the user application (true = dispatch keyboard info to Dear ImGui only, false = dispatch keyboard info to Dear ImGui + underlying app)
    if (g.WantCaptureKeyboardNextFrame != -1)
        io.WantCaptureKeyboard = (g.WantCaptureKeyboardNextFrame != 0);
    else
        io.WantCaptureKeyboard = (g.ActiveId != 0) || (modal_window != NULL);
    if (io.NavActive && (io.ConfigFlags & ImGuiConfigFlags_NavEnableKeyboard) && !(io.ConfigFlags & ImGuiConfigFlags_NavNoCaptureKeyboard))
        io.WantCaptureKeyboard = true;

    // Update io.WantTextInput flag, this is to allow systems without a keyboard (e.g. mobile, hand-held) to show a software keyboard if possible
    io.WantTextInput = (g.WantTextInputNextFrame != -1) ? (g.WantTextInputNextFrame != 0) : false;
}

// [Internal] Do not use directly (can read io.KeyMods instead)
ImGuiModFlags ImGui::GetMergedModFlags()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiModFlags key_mods = ImGuiModFlags_None;
    if (g.IO.KeyCtrl)   { key_mods |= ImGuiModFlags_Ctrl; }
    if (g.IO.KeyShift)  { key_mods |= ImGuiModFlags_Shift; }
    if (g.IO.KeyAlt)    { key_mods |= ImGuiModFlags_Alt; }
    if (g.IO.KeySuper)  { key_mods |= ImGuiModFlags_Super; }
    return key_mods;
}

c_void ImGui::NewFrame()
{
    // IM_ASSERT(GImGui != NULL && "No current context. Did you call ImGui::CreateContext() and ImGui::SetCurrentContext() ?");
    let g = GImGui; // ImGuiContext& g = *GImGui;

    // Remove pending delete hooks before frame start.
    // This deferred removal avoid issues of removal while iterating the hook vector
    for (c_int n = g.Hooks.Size - 1; n >= 0; n--)
        if (g.Hooks[n].Type == ImGuiContextHookType_PendingRemoval_)
            g.Hooks.erase(&g.Hooks[n]);

    CallContextHooks(&g, ImGuiContextHookType_NewFramePre);

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
    g.MenusIdSubmittedThisFrame.resize(0);

    // Calculate frame-rate for the user, as a purely luxurious feature
    g.FramerateSecPerFrameAccum += g.IO.DeltaTime - g.FramerateSecPerFrame[g.FramerateSecPerFrameIdx];
    g.FramerateSecPerFrame[g.FramerateSecPerFrameIdx] = g.IO.DeltaTime;
    g.FramerateSecPerFrameIdx = (g.FramerateSecPerFrameIdx + 1) % IM_ARRAYSIZE(g.FramerateSecPerFrame);
    g.FramerateSecPerFrameCount = ImMin(g.FramerateSecPerFrameCount + 1, IM_ARRAYSIZE(g.FramerateSecPerFrame));
    g.IO.Framerate = (g.FramerateSecPerFrameAccum > 0f32) ? (1f32 / (g.FramerateSecPerFrameAccum / g.FramerateSecPerFrameCount)) : f32::MAX;

    UpdateViewportsNewFrame();

    // Setup current font and draw list shared data
    // FIXME-VIEWPORT: the concept of a single ClipRectFullscreen is not ideal!
    g.IO.Fonts->Locked = true;
    SetCurrentFont(GetDefaultFont());
    // IM_ASSERT(g.Font->IsLoaded());
    ImRect virtual_space(f32::MAX, f32::MAX, -f32::MAX, -f32::MAX);
    for (c_int n = 0; n < g.Viewports.Size; n++)
        virtual_space.Add(g.Viewports[n]->GetMainRect());
    g.DrawListSharedData.ClipRectFullscreen = virtual_space.ToVec4();
    g.DrawListSharedData.CurveTessellationTol = g.Style.CurveTessellationTol;
    g.DrawListSharedData.SetCircleTessellationMaxError(g.Style.CircleTessellationMaxError);
    g.DrawListSharedData.InitialFlags = ImDrawListFlags_None;
    if (g.Style.AntiAliasedLines)
        g.DrawListSharedData.InitialFlags |= ImDrawListFlags_AntiAliasedLines;
    if (g.Style.AntiAliasedLinesUseTex && !(g.Font->ContainerAtlas->Flags & ImFontAtlasFlags_NoBakedLines))
        g.DrawListSharedData.InitialFlags |= ImDrawListFlags_AntiAliasedLinesUseTex;
    if (g.Style.AntiAliasedFill)
        g.DrawListSharedData.InitialFlags |= ImDrawListFlags_AntiAliasedFill;
    if (g.IO.BackendFlags & ImGuiBackendFlags_RendererHasVtxOffset)
        g.DrawListSharedData.InitialFlags |= ImDrawListFlags_AllowVtxOffset;

    // Mark rendering data as invalid to prevent user who may have a handle on it to use it.
    for (c_int n = 0; n < g.Viewports.Size; n++)
    {
        *mut ImGuiViewportP viewport = g.Viewports[n];
        viewport->DrawData = None;
        viewport->DrawDataP.Clear();
    }

    // Drag and drop keep the source ID alive so even if the source disappear our state is consistent
    if (g.DragDropActive && g.DragDropPayload.SourceId == g.ActiveId)
        KeepAliveID(g.DragDropPayload.SourceId);

    // Update HoveredId data
    if (!g.HoveredIdPreviousFrame)
        g.HoveredIdTimer = 0f32;
    if (!g.HoveredIdPreviousFrame || (g.HoveredId && g.ActiveId == g.HoveredId))
        g.HoveredIdNotActiveTimer = 0f32;
    if (g.HoveredId)
        g.HoveredIdTimer += g.IO.DeltaTime;
    if (g.HoveredId && g.ActiveId != g.HoveredId)
        g.HoveredIdNotActiveTimer += g.IO.DeltaTime;
    g.HoveredIdPreviousFrame = g.HoveredId;
    g.HoveredIdPreviousFrameUsingMouseWheel = g.HoveredIdUsingMouseWheel;
    g.HoveredId = 0;
    g.HoveredIdAllowOverlap = false;
    g.HoveredIdUsingMouseWheel = false;
    g.HoveredIdDisabled = false;

    // Clear ActiveID if the item is not alive anymore.
    // In 1.87, the common most call to KeepAliveID() was moved from GetID() to ItemAdd().
    // As a result, custom widget using ButtonBehavior() _without_ ItemAdd() need to call KeepAliveID() themselves.
    if (g.ActiveId != 0 && g.ActiveIdIsAlive != g.ActiveId && g.ActiveIdPreviousFrame == g.ActiveId)
    {
        IMGUI_DEBUG_LOG_ACTIVEID("NewFrame(): ClearActiveID() because it isn't marked alive anymore!\n");
        ClearActiveID();
    }

    // Update ActiveId data (clear reference to active widget if the widget isn't alive anymore)
    if (g.ActiveId)
        g.ActiveIdTimer += g.IO.DeltaTime;
    g.LastActiveIdTimer += g.IO.DeltaTime;
    g.ActiveIdPreviousFrame = g.ActiveId;
    g.ActiveIdPreviousFrameWindow = g.ActiveIdWindow;
    g.ActiveIdPreviousFrameHasBeenEditedBefore = g.ActiveIdHasBeenEditedBefore;
    g.ActiveIdIsAlive = 0;
    g.ActiveIdHasBeenEditedThisFrame = false;
    g.ActiveIdPreviousFrameIsAlive = false;
    g.ActiveIdIsJustActivated = false;
    if (g.TempInputId != 0 && g.ActiveId != g.TempInputId)
        g.TempInputId = 0;
    if (g.ActiveId == 0)
    {
        g.ActiveIdUsingNavDirMask = 0x00;
        g.ActiveIdUsingKeyInputMask.ClearAllBits();
    }

// #ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
    if (g.ActiveId == 0)
        g.ActiveIdUsingNavInputMask = 0;
    else if (g.ActiveIdUsingNavInputMask != 0)
    {
        // If your custom widget code used:                 { g.ActiveIdUsingNavInputMask |= (1 << ImGuiNavInput_Cancel); }
        // Since IMGUI_VERSION_NUM >= 18804 it should be:   { SetActiveIdUsingKey(ImGuiKey_Escape); SetActiveIdUsingKey(ImGuiKey_NavGamepadCancel); }
        if (g.ActiveIdUsingNavInputMask & (1 << ImGuiNavInput_Cancel))
            SetActiveIdUsingKey(ImGuiKey_Escape);
        if (g.ActiveIdUsingNavInputMask & ~(1 << ImGuiNavInput_Cancel))
            // IM_ASSERT(0); // Other values unsupported
    }
// #endif

    // Update hover delay for IsItemHovered() with delays and tooltips
    g.HoverDelayIdPreviousFrame = g.HoverDelayId;
    if (g.HoverDelayId != 0)
    {
        //if (g.IO.MouseDelta.x == 0f32 && g.IO.MouseDelta.y == 0f32) // Need design/flags
        g.HoverDelayTimer += g.IO.DeltaTime;
        g.HoverDelayClearTimer = 0f32;
        g.HoverDelayId = 0;
    }
    else if (g.HoverDelayTimer > 0f32)
    {
        // This gives a little bit of leeway before clearing the hover timer, allowing mouse to cross gaps
        g.HoverDelayClearTimer += g.IO.DeltaTime;
        if (g.HoverDelayClearTimer >= ImMax(0.20f32, g.IO.DeltaTime * 2.00f32)) // ~6 frames at 30 Hz + allow for low framerate
            g.HoverDelayTimer = g.HoverDelayClearTimer = 0f32; // May want a decaying timer, in which case need to clamp at max first, based on max of caller last requested timer.
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
    g.InputEventsTrail.resize(0);
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
    if (GetTopMostPopupModal() != NULL || (g.NavWindowingTarget != NULL && g.NavWindowingHighlightAlpha > 0f32))
        g.DimBgRatio = ImMin(g.DimBgRatio + g.IO.DeltaTime * 6f32, 1f32);
    else
        g.DimBgRatio = ImMax(g.DimBgRatio - g.IO.DeltaTime * 10f32, 0f32);

    g.MouseCursor = ImGuiMouseCursor_Arrow;
    g.WantCaptureMouseNextFrame = g.WantCaptureKeyboardNextFrame = g.WantTextInputNextFrame = -1;

    // Platform IME data: reset for the frame
    g.PlatformImeDataPrev = g.PlatformImeData;
    g.PlatformImeData.WantVisible = false;

    // Mouse wheel scrolling, scale
    UpdateMouseWheel();

    // Mark all windows as not visible and compact unused memory.
    // IM_ASSERT(g.WindowsFocusOrder.Size <= g.Windows.Size);
    const c_float memory_compact_start_time = (g.GcCompactAll || g.IO.ConfigMemoryCompactTimer < 0f32) ? f32::MAX : g.Time - g.IO.ConfigMemoryCompactTimer;
    for (c_int i = 0; i != g.Windows.Size; i++)
    {
        ImGuiWindow* window = g.Windows[i];
        window.WasActive = window.Active;
        window.BeginCount = 0;
        window.Active = false;
        window.WriteAccessed = false;

        // Garbage collect transient buffers of recently unused windows
        if (!window.WasActive && !window.MemoryCompacted && window.LastTimeActive < memory_compact_start_time)
            GcCompactTransientWindowBuffers(window);
    }

    // Garbage collect transient buffers of recently unused tables
    for (c_int i = 0; i < g.TablesLastTimeActive.Size; i++)
        if (g.TablesLastTimeActive[i] >= 0f32 && g.TablesLastTimeActive[i] < memory_compact_start_time)
            TableGcCompactTransientBuffers(g.Tables.GetByIndex(i));
    for (c_int i = 0; i < g.TablesTempData.Size; i++)
        if (g.TablesTempData[i].LastTimeActive >= 0f32 && g.TablesTempData[i].LastTimeActive < memory_compact_start_time)
            TableGcCompactTransientBuffers(&g.TablesTempData[i]);
    if (g.GcCompactAll)
        GcCompactTransientMiscBuffers();
    g.GcCompactAll = false;

    // Closing the focused window restore focus to the first active root window in descending z-order
    if (g.NavWindow && !g.Navwindow.WasActive)
        FocusTopMostWindowUnderOne(NULL, NULL);

    // No window should be open at the beginning of the frame.
    // But in order to allow the user to call NewFrame() multiple times without calling Render(), we are doing an explicit clear.
    g.CurrentWindowStack.resize(0);
    g.BeginPopupStack.resize(0);
    g.ItemFlagsStack.resize(0);
    g.ItemFlagsStack.push(ImGuiItemFlags_None);
    g.GroupStack.resize(0);

    // Docking
    DockContextNewFrameUpdateDocking(&g);

    // [DEBUG] Update debug features
    UpdateDebugToolItemPicker();
    UpdateDebugToolStackQueries();

    // Create implicit/fallback window - which we will only render it if the user has added something to it.
    // We don't use "Debug" to avoid colliding with user trying to create a "Debug" window with custom flags.
    // This fallback is particularly important as it avoid ImGui:: calls from crashing.
    g.WithinFrameScopeWithImplicitWindow = true;
    SetNextWindowSize(ImVec2(400, 400), ImGuiCond_FirstUseEver);
    Begin("Debug##Default");
    // IM_ASSERT(g.Currentwindow.IsFallbackWindow == true);

    CallContextHooks(&g, ImGuiContextHookType_NewFramePost);
}

c_void ImGui::Initialize()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(!g.Initialized && !g.SettingsLoaded);

    // Add .ini handle for ImGuiWindow type
    {
        ImGuiSettingsHandler ini_handler;
        ini_handler.TypeName = "Window";
        ini_handler.TypeHash = ImHashStr("Window");
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
    *mut ImGuiViewportP viewport = IM_NEW(ImGuiViewportP)();
    viewport->ID = IMGUI_VIEWPORT_DEFAULT_ID;
    viewport->Idx = 0;
    viewport->PlatformWindowCreated = true;
    viewport->Flags = ImGuiViewportFlags_OwnedByApp;
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
c_void ImGui::Shutdown()
{
    // The fonts atlas can be used prior to calling NewFrame(), so we clear it even if g.Initialized is FALSE (which would happen if we never called NewFrame)
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.IO.Fonts && g.FontAtlasOwnedByContext)
    {
        g.IO.Fonts->Locked = false;
        IM_DELETE(g.IO.Fonts);
    }
    g.IO.Fonts = None;

    // Cleanup of other data are conditional on actually having initialized Dear ImGui.
    if (!g.Initialized)
        return;

    // Save settings (unless we haven't attempted to load them: CreateContext/DestroyContext without a call to NewFrame shouldn't save an empty file)
    if (g.SettingsLoaded && g.IO.IniFilename != NULL)
        SaveIniSettingsToDisk(g.IO.IniFilename);

    // Destroy platform windows
    DestroyPlatformWindows();

    // Shutdown extensions
    DockContextShutdown(&g);

    CallContextHooks(&g, ImGuiContextHookType_Shutdown);

    // Clear everything else
    g.Windows.clear_delete();
    g.WindowsFocusOrder.clear();
    g.WindowsTempSortBuffer.clear();
    g.CurrentWindow = None;
    g.CurrentWindowStack.clear();
    g.WindowsById.Clear();
    g.NavWindow = None;
    g.HoveredWindow = g.HoveredWindowUnderMovingWindow = None;
    g.ActiveIdWindow = g.ActiveIdPreviousFrameWindow = None;
    g.MovingWindow = None;
    g.ColorStack.clear();
    g.StyleVarStack.clear();
    g.FontStack.clear();
    g.OpenPopupStack.clear();
    g.BeginPopupStack.clear();

    g.CurrentViewport = g.MouseViewport = g.MouseLastHoveredViewport = None;
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

    if (g.LogFile)
    {
// #ifndef IMGUI_DISABLE_TTY_FUNCTIONS
        if (g.LogFile != stdout)
// #endif
            ImFileClose(g.LogFile);
        g.LogFile = None;
    }
    g.LogBuffer.clear();
    g.DebugLogBuf.clear();

    g.Initialized = false;
}

// FIXME: Add a more explicit sort order in the window structure.
static c_int IMGUI_CDECL ChildWindowComparer(*const c_void lhs, *const c_void rhs)
{
    *const ImGuiWindow const a = *(*const ImGuiWindow const *)lhs;
    *const ImGuiWindow const b = *(*const ImGuiWindow const *)rhs;
    if (c_int d = (a->Flags & ImGuiWindowFlags_Popup) - (b->Flags & ImGuiWindowFlags_Popup))
        return d;
    if (c_int d = (a->Flags & ImGuiWindowFlags_Tooltip) - (b->Flags & ImGuiWindowFlags_Tooltip))
        return d;
    return (a->BeginOrderWithinParent - b->BeginOrderWithinParent);
}

static c_void AddWindowToSortBuffer(Vec<ImGuiWindow*>* out_sorted_windows, ImGuiWindow* window)
{
    out_sorted_windows.push(window);
    if (window.Active)
    {
        c_int count = window.DC.ChildWindows.Size;
        ImQsort(window.DC.ChildWindows.Data, count, sizeof(ImGuiWindow*), ChildWindowComparer);
        for (c_int i = 0; i < count; i++)
        {
            ImGuiWindow* child = window.DC.ChildWindows[i];
            if (child->Active)
                AddWindowToSortBuffer(out_sorted_windows, child);
        }
    }
}

static c_void AddDrawListToDrawData(Vec<ImDrawList*>* out_list, ImDrawList* draw_list)
{
    if (draw_list->CmdBuffer.Size == 0)
        return;
    if (draw_list->CmdBuffer.Size == 1 && draw_list->CmdBuffer[0].ElemCount == 0 && draw_list->CmdBuffer[0].UserCallback == NULL)
        return;

    // Draw list sanity check. Detect mismatch between PrimReserve() calls and incrementing _VtxCurrentIdx, _VtxWritePtr etc.
    // May trigger for you if you are using PrimXXX functions incorrectly.
    // IM_ASSERT(draw_list->VtxBuffer.Size == 0 || draw_list->_VtxWritePtr == draw_list->VtxBuffer.Data + draw_list->VtxBuffer.Size);
    // IM_ASSERT(draw_list->IdxBuffer.Size == 0 || draw_list->_IdxWritePtr == draw_list->IdxBuffer.Data + draw_list->IdxBuffer.Size);
    if (!(draw_list->Flags & ImDrawListFlags_AllowVtxOffset))
        // IM_ASSERT(draw_list->_VtxCurrentIdx == draw_list->VtxBuffer.Size);

    // Check that draw_list doesn't use more vertices than indexable (default ImDrawIdx = unsigned short = 2 bytes = 64K vertices per ImDrawList = per window)
    // If this assert triggers because you are drawing lots of stuff manually:
    // - First, make sure you are coarse clipping yourself and not trying to draw many things outside visible bounds.
    //   Be mindful that the ImDrawList API doesn't filter vertices. Use the Metrics/Debugger window to inspect draw list contents.
    // - If you want large meshes with more than 64K vertices, you can either:
    //   (A) Handle the ImDrawCmd::VtxOffset value in your renderer backend, and set 'io.BackendFlags |= ImGuiBackendFlags_RendererHasVtxOffset'.
    //       Most example backends already support this from 1.71. Pre-1.71 backends won't.
    //       Some graphics API such as GL ES 1/2 don't have a way to offset the starting vertex so it is not supported for them.
    //   (B) Or handle 32-bit indices in your renderer backend, and uncomment '#define ImDrawIdx unsigned int' line in imconfig.h.
    //       Most example backends already support this. For example, the OpenGL example code detect index size at compile-time:
    //         glDrawElements(GL_TRIANGLES, (GLsizei)pcmd->ElemCount, sizeof(ImDrawIdx) == 2 ? GL_UNSIGNED_SHORT : GL_UNSIGNED_INT, idx_buffer_offset);
    //       Your own engine or render API may use different parameters or function calls to specify index sizes.
    //       2 and 4 bytes indices are generally supported by most graphics API.
    // - If for some reason neither of those solutions works for you, a workaround is to call BeginChild()/EndChild() before reaching
    //   the 64K limit to split your draw commands in multiple draw lists.
    if (sizeof(ImDrawIdx) == 2)
        // IM_ASSERT(draw_list->_VtxCurrentIdx < (1 << 16) && "Too many vertices in ImDrawList using 16-bit indices. Read comment above");

    out_list.push(draw_list);
}

static c_void AddWindowToDrawData(ImGuiWindow* window, c_int layer)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    *mut ImGuiViewportP viewport = window.Viewport;
    g.IO.MetricsRenderWindows+= 1;
    if (window.Flags & ImGuiWindowFlags_DockNodeHost)
        window.DrawList.ChannelsMerge();
    AddDrawListToDrawData(&viewport->DrawDataBuilder.Layers[layer], window.DrawList);
    for (c_int i = 0; i < window.DC.ChildWindows.Size; i++)
    {
        ImGuiWindow* child = window.DC.ChildWindows[i];
        if (IsWindowActiveAndVisible(child)) // Clipped children may have been marked not active
            AddWindowToDrawData(child, layer);
    }
}

static inline c_int GetWindowDisplayLayer(ImGuiWindow* window)
{
    return (window.Flags & ImGuiWindowFlags_Tooltip) ? 1 : 0;
}

// Layer is locked for the root window, however child windows may use a different viewport (e.g. extruding menu)
static inline c_void AddRootWindowToDrawData(ImGuiWindow* window)
{
    AddWindowToDrawData(window, GetWindowDisplayLayer(window));
}

c_void ImDrawDataBuilder::FlattenIntoSingleLayer()
{
    c_int n = Layers[0].Size;
    c_int size = n;
    for (c_int i = 1; i < IM_ARRAYSIZE(Layers); i++)
        size += Layers[i].Size;
    Layers[0].resize(size);
    for (c_int layer_n = 1; layer_n < IM_ARRAYSIZE(Layers); layer_n++)
    {
        Vec<ImDrawList*>& layer = Layers[layer_n];
        if (layer.empty())
            continue;
        memcpy(&Layers[0][n], &layer[0], layer.Size * sizeof(ImDrawList*));
        n += layer.Size;
        layer.resize(0);
    }
}

static c_void SetupViewportDrawData(*mut ImGuiViewportP viewport, Vec<ImDrawList*>* draw_lists)
{
    // When minimized, we report draw_data->DisplaySize as zero to be consistent with non-viewport mode,
    // and to allow applications/backends to easily skip rendering.
    // FIXME: Note that we however do NOT attempt to report "zero drawlist / vertices" into the ImDrawData structure.
    // This is because the work has been done already, and its wasted! We should fix that and add optimizations for
    // it earlier in the pipeline, rather than pretend to hide the data at the end of the pipeline.
    let is_minimized: bool = (viewport->Flags & ImGuiViewportFlags_Minimized) != 0;

    ImGuiIO& io = ImGui::GetIO();
    ImDrawData* draw_data = &viewport->DrawDataP;
    viewport->DrawData = draw_data; // Make publicly accessible
    draw_data->Valid = true;
    draw_data->CmdLists = (draw_lists->Size > 0) ? draw_lists->Data : NULL;
    draw_data->CmdListsCount = draw_lists->Size;
    draw_data->TotalVtxCount = draw_data->TotalIdxCount = 0;
    draw_data->DisplayPos = viewport->Pos;
    draw_data->DisplaySize = is_minimized ? ImVec2(0f32, 0f32) : viewport->Size;
    draw_data->FramebufferScale = io.DisplayFramebufferScale; // FIXME-VIEWPORT: This may vary on a per-monitor/viewport basis?
    draw_data->OwnerViewport = viewport;
    for (c_int n = 0; n < draw_lists->Size; n++)
    {
        ImDrawList* draw_list = draw_lists->Data[n];
        draw_list->_PopUnusedDrawCmd();
        draw_data->TotalVtxCount += draw_list->VtxBuffer.Size;
        draw_data->TotalIdxCount += draw_list->IdxBuffer.Size;
    }
}

// Push a clipping rectangle for both ImGui logic (hit-testing etc.) and low-level ImDrawList rendering.
// - When using this function it is sane to ensure that float are perfectly rounded to integer values,
//   so that e.g. (max.x-min.x) in user's render produce correct result.
// - If the code here changes, may need to update code of functions like NextColumn() and PushColumnClipRect():
//   some frequently called functions which to modify both channels and clipping simultaneously tend to use the
//   more specialized SetWindowClipRectBeforeSetChannel() to avoid extraneous updates of underlying ImDrawCmds.
c_void ImGui::PushClipRect(const ImVec2& clip_rect_min, const ImVec2& clip_rect_max, bool intersect_with_current_clip_rect)
{
    ImGuiWindow* window = GetCurrentWindow();
    window.DrawList.PushClipRect(clip_rect_min, clip_rect_max, intersect_with_current_clip_rect);
    window.ClipRect = window.DrawList._ClipRectStack.back();
}

c_void ImGui::PopClipRect()
{
    ImGuiWindow* window = GetCurrentWindow();
    window.DrawList.PopClipRect();
    window.ClipRect = window.DrawList._ClipRectStack.back();
}

static ImGuiWindow* FindFrontMostVisibleChildWindow(ImGuiWindow* window)
{
    for (c_int n = window.DC.ChildWindows.Size - 1; n >= 0; n--)
        if (IsWindowActiveAndVisible(window.DC.ChildWindows[n]))
            return FindFrontMostVisibleChildWindow(window.DC.ChildWindows[n]);
    return window;
}

static c_void ImGui::RenderDimmedBackgroundBehindWindow(ImGuiWindow* window, u32 col)
{
    if ((col & IM_COL32_A_MASK) == 0)
        return;

    *mut ImGuiViewportP viewport = window.Viewport;
    ImRect viewport_rect = viewport->GetMainRect();

    // Draw behind window by moving the draw command at the FRONT of the draw list
    {
        // We've already called AddWindowToDrawData() which called DrawList.ChannelsMerge() on DockNodeHost windows,
        // and draw list have been trimmed already, hence the explicit recreation of a draw command if missing.
        // FIXME: This is creating complication, might be simpler if we could inject a drawlist in drawdata at a given position and not attempt to manipulate ImDrawCmd order.
        ImDrawList* draw_list = window.RootWindowDockTree->DrawList;
        if (draw_list->CmdBuffer.Size == 0)
            draw_list->AddDrawCmd();
        draw_list->PushClipRect(viewport_rect.Min - ImVec2(1, 1), viewport_rect.Max + ImVec2(1, 1), false); // Ensure ImDrawCmd are not merged
        draw_list->AddRectFilled(viewport_rect.Min, viewport_rect.Max, col);
        ImDrawCmd cmd = draw_list->CmdBuffer.back();
        // IM_ASSERT(cmd.ElemCount == 6);
        draw_list->CmdBuffer.pop_back();
        draw_list->CmdBuffer.push_front(cmd);
        draw_list->PopClipRect();
        draw_list->AddDrawCmd(); // We need to create a command as CmdBuffer.back().IdxOffset won't be correct if we append to same command.
    }

    // Draw over sibling docking nodes in a same docking tree
    if (window.Rootwindow.DockIsActive)
    {
        ImDrawList* draw_list = FindFrontMostVisibleChildWindow(window.RootWindowDockTree)->DrawList;
        if (draw_list->CmdBuffer.Size == 0)
            draw_list->AddDrawCmd();
        draw_list->PushClipRect(viewport_rect.Min, viewport_rect.Max, false);
        RenderRectFilledWithHole(draw_list, window.RootWindowDockTree->Rect(), window.Rootwindow.Rect(), col, 0f32);// window.RootWindowDockTree->WindowRounding);
        draw_list->PopClipRect();
    }
}

ImGuiWindow* ImGui::FindBottomMostVisibleWindowWithinBeginStack(ImGuiWindow* parent_window)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiWindow* bottom_most_visible_window = parent_window;
    for (c_int i = FindWindowDisplayIndex(parent_window); i >= 0; i--)
    {
        ImGuiWindow* window = g.Windows[i];
        if (window.Flags & ImGuiWindowFlags_ChildWindow)
            continue;
        if (!IsWindowWithinBeginStackOf(window, parent_window))
            break;
        if (IsWindowActiveAndVisible(window) && GetWindowDisplayLayer(window) <= GetWindowDisplayLayer(parent_window))
            bottom_most_visible_window = window;
    }
    return bottom_most_visible_window;
}

static c_void ImGui::RenderDimmedBackgrounds()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiWindow* modal_window = GetTopMostAndVisiblePopupModal();
    if (g.DimBgRatio <= 0f32 && g.NavWindowingHighlightAlpha <= 0f32)
        return;
    let dim_bg_for_modal: bool = (modal_window != NULL);
    let dim_bg_for_window_list: bool = (g.NavWindowingTargetAnim != NULL && g.NavWindowingTargetAnim->Active);
    if (!dim_bg_for_modal && !dim_bg_for_window_list)
        return;

    ImGuiViewport* viewports_already_dimmed[2] = { NULL, NULL };
    if (dim_bg_for_modal)
    {
        // Draw dimming behind modal or a begin stack child, whichever comes first in draw order.
        ImGuiWindow* dim_behind_window = FindBottomMostVisibleWindowWithinBeginStack(modal_window);
        RenderDimmedBackgroundBehindWindow(dim_behind_window, GetColorU32(ImGuiCol_ModalWindowDimBg, g.DimBgRatio));
        viewports_already_dimmed[0] = modal_window.Viewport;
    }
    else if (dim_bg_for_window_list)
    {
        // Draw dimming behind CTRL+Tab target window and behind CTRL+Tab UI window
        RenderDimmedBackgroundBehindWindow(g.NavWindowingTargetAnim, GetColorU32(ImGuiCol_NavWindowingDimBg, g.DimBgRatio));
        if (g.NavWindowingListWindow != NULL && g.NavWindowingListwindow.Viewport && g.NavWindowingListwindow.Viewport != g.NavWindowingTargetAnim->Viewport)
            RenderDimmedBackgroundBehindWindow(g.NavWindowingListWindow, GetColorU32(ImGuiCol_NavWindowingDimBg, g.DimBgRatio));
        viewports_already_dimmed[0] = g.NavWindowingTargetAnim->Viewport;
        viewports_already_dimmed[1] = g.NavWindowingListWindow ? g.NavWindowingListwindow.Viewport : NULL;

        // Draw border around CTRL+Tab target window
        ImGuiWindow* window = g.NavWindowingTargetAnim;
        ImGuiViewport* viewport = window.Viewport;
        c_float distance = g.FontSize;
        ImRect bb = window.Rect();
        bb.Expand(distance);
        if (bb.GetWidth() >= viewport->Size.x && bb.GetHeight() >= viewport->Size.y)
            bb.Expand(-distance - 1f32); // If a window fits the entire viewport, adjust its highlight inward
        if (window.DrawList.CmdBuffer.Size == 0)
            window.DrawList.AddDrawCmd();
        window.DrawList.PushClipRect(viewport->Pos, viewport->Pos + viewport->Size);
        window.DrawList.AddRect(bb.Min, bb.Max, GetColorU32(ImGuiCol_NavWindowingHighlight, g.NavWindowingHighlightAlpha), window.WindowRounding, 0, 3.00f32);
        window.DrawList.PopClipRect();
    }

    // Draw dimming background on _other_ viewports than the ones our windows are in
    for (c_int viewport_n = 0; viewport_n < g.Viewports.Size; viewport_n++)
    {
        *mut ImGuiViewportP viewport = g.Viewports[viewport_n];
        if (viewport == viewports_already_dimmed[0] || viewport == viewports_already_dimmed[1])
            continue;
        if (modal_window && viewport->Window && IsWindowAbove(viewport->Window, modal_window))
            continue;
        ImDrawList* draw_list = GetForegroundDrawList(viewport);
        const u32 dim_bg_col = GetColorU32(dim_bg_for_modal ? ImGuiCol_ModalWindowDimBg : ImGuiCol_NavWindowingDimBg, g.DimBgRatio);
        draw_list->AddRectFilled(viewport->Pos, viewport->Pos + viewport->Size, dim_bg_col);
    }
}

// This is normally called by Render(). You may want to call it directly if you want to avoid calling Render() but the gain will be very minimal.
c_void ImGui::EndFrame()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.Initialized);

    // Don't process EndFrame() multiple times.
    if (g.FrameCountEnded == g.FrameCount)
        return;
    // IM_ASSERT(g.WithinFrameScope && "Forgot to call ImGui::NewFrame()?");

    CallContextHooks(&g, ImGuiContextHookType_EndFramePre);

    ErrorCheckEndFrameSanityChecks();

    // Notify Platform/OS when our Input Method Editor cursor has moved (e.g. CJK inputs using Microsoft IME)
    if (g.IO.SetPlatformImeDataFn && memcmp(&g.PlatformImeData, &g.PlatformImeDataPrev, sizeof(ImGuiPlatformImeData)) != 0)
    {
        ImGuiViewport* viewport = FindViewportByID(g.PlatformImeViewport);
        g.IO.SetPlatformImeDataFn(viewport ? viewport : GetMainViewport(), &g.PlatformImeData);
    }

    // Hide implicit/fallback "Debug" window if it hasn't been used
    g.WithinFrameScopeWithImplicitWindow = false;
    if (g.CurrentWindow && !g.Currentwindow.WriteAccessed)
        g.Currentwindow.Active = false;
    End();

    // Update navigation: CTRL+Tab, wrap-around requests
    NavEndFrame();

    // Update docking
    DockContextEndFrame(&g);

    SetCurrentViewport(NULL, NULL);

    // Drag and Drop: Elapse payload (if delivered, or if source stops being submitted)
    if (g.DragDropActive)
    {
        let mut is_delivered: bool =  g.DragDropPayload.Delivery;
        let mut is_elapsed: bool =  (g.DragDropPayload.DataFrameCount + 1 < g.FrameCount) && ((g.DragDropSourceFlags & ImGuiDragDropFlags_SourceAutoExpirePayload) || !IsMouseDown(g.DragDropMouseButton));
        if (is_delivered || is_elapsed)
            ClearDragDrop();
    }

    // Drag and Drop: Fallback for source tooltip. This is not ideal but better than nothing.
    if (g.DragDropActive && g.DragDropSourceFrameCount < g.FrameCount && !(g.DragDropSourceFlags & ImGuiDragDropFlags_SourceNoPreviewTooltip))
    {
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
    g.WindowsTempSortBuffer.resize(0);
    g.WindowsTempSortBuffer.reserve(g.Windows.Size);
    for (c_int i = 0; i != g.Windows.Size; i++)
    {
        ImGuiWindow* window = g.Windows[i];
        if (window.Active && (window.Flags & ImGuiWindowFlags_ChildWindow))       // if a child is active its parent will add it
            continue;
        AddWindowToSortBuffer(&g.WindowsTempSortBuffer, window);
    }

    // This usually assert if there is a mismatch between the ImGuiWindowFlags_ChildWindow / ParentWindow values and DC.ChildWindows[] in parents, aka we've done something wrong.
    // IM_ASSERT(g.Windows.Size == g.WindowsTempSortBuffer.Size);
    g.Windows.swap(g.WindowsTempSortBuffer);
    g.IO.MetricsActiveWindows = g.WindowsActiveCount;

    // Unlock font atlas
    g.IO.Fonts->Locked = false;

    // Clear Input data for next frame
    g.IO.MouseWheel = g.IO.MouseWheelH = 0f32;
    g.IO.InputQueueCharacters.resize(0);

    CallContextHooks(&g, ImGuiContextHookType_EndFramePost);
}

// Prepare the data for rendering so you can call GetDrawData()
// (As with anything within the ImGui:: namspace this doesn't touch your GPU or graphics API at all:
// it is the role of the ImGui_ImplXXXX_RenderDrawData() function provided by the renderer backend)
c_void ImGui::Render()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.Initialized);

    if (g.FrameCountEnded != g.FrameCount)
        EndFrame();
    let first_render_of_frame: bool = (g.FrameCountRendered != g.FrameCount);
    g.FrameCountRendered = g.FrameCount;
    g.IO.MetricsRenderWindows = 0;

    CallContextHooks(&g, ImGuiContextHookType_RenderPre);

    // Add background ImDrawList (for each active viewport)
    for (c_int n = 0; n != g.Viewports.Size; n++)
    {
        *mut ImGuiViewportP viewport = g.Viewports[n];
        viewport->DrawDataBuilder.Clear();
        if (viewport->DrawLists[0] != NULL)
            AddDrawListToDrawData(&viewport->DrawDataBuilder.Layers[0], GetBackgroundDrawList(viewport));
    }

    // Add ImDrawList to render
    ImGuiWindow* windows_to_render_top_most[2];
    windows_to_render_top_most[0] = (g.NavWindowingTarget && !(g.NavWindowingTarget->Flags & ImGuiWindowFlags_NoBringToFrontOnFocus)) ? g.NavWindowingTarget->RootWindowDockTree : NULL;
    windows_to_render_top_most[1] = (g.NavWindowingTarget ? g.NavWindowingListWindow : NULL);
    for (c_int n = 0; n != g.Windows.Size; n++)
    {
        ImGuiWindow* window = g.Windows[n];
        IM_MSVC_WARNING_SUPPRESS(6011); // Static Analysis false positive "warning C6011: Dereferencing NULL pointer 'window'"
        if (IsWindowActiveAndVisible(window) && (window.Flags & ImGuiWindowFlags_ChildWindow) == 0 && window != windows_to_render_top_most[0] && window != windows_to_render_top_most[1])
            AddRootWindowToDrawData(window);
    }
    for (c_int n = 0; n < IM_ARRAYSIZE(windows_to_render_top_most); n++)
        if (windows_to_render_top_most[n] && IsWindowActiveAndVisible(windows_to_render_top_most[n])) // NavWindowingTarget is always temporarily displayed as the top-most window
            AddRootWindowToDrawData(windows_to_render_top_most[n]);

    // Draw modal/window whitening backgrounds
    if (first_render_of_frame)
        RenderDimmedBackgrounds();

    // Draw software mouse cursor if requested by io.MouseDrawCursor flag
    if (g.IO.MouseDrawCursor && first_render_of_frame && g.MouseCursor != ImGuiMouseCursor_None)
        RenderMouseCursor(g.IO.MousePos, g.Style.MouseCursorScale, g.MouseCursor, IM_COL32_WHITE, IM_COL32_BLACK, IM_COL32(0, 0, 0, 48));

    // Setup ImDrawData structures for end-user
    g.IO.MetricsRenderVertices = g.IO.MetricsRenderIndices = 0;
    for (c_int n = 0; n < g.Viewports.Size; n++)
    {
        *mut ImGuiViewportP viewport = g.Viewports[n];
        viewport->DrawDataBuilder.FlattenIntoSingleLayer();

        // Add foreground ImDrawList (for each active viewport)
        if (viewport->DrawLists[1] != NULL)
            AddDrawListToDrawData(&viewport->DrawDataBuilder.Layers[0], GetForegroundDrawList(viewport));

        SetupViewportDrawData(viewport, &viewport->DrawDataBuilder.Layers[0]);
        ImDrawData* draw_data = viewport->DrawData;
        g.IO.MetricsRenderVertices += draw_data->TotalVtxCount;
        g.IO.MetricsRenderIndices += draw_data->TotalIdxCount;
    }

    CallContextHooks(&g, ImGuiContextHookType_RenderPost);
}

// Calculate text size. Text can be multi-line. Optionally ignore text after a ## marker.
// CalcTextSize("") should return ImVec2(0f32, g.FontSize)
ImVec2 ImGui::CalcTextSize(*const char text, *const char text_end, bool hide_text_after_double_hash, c_float wrap_width)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    *const char text_display_end;
    if (hide_text_after_double_hash)
        text_display_end = FindRenderedTextEnd(text, text_end);      // Hide anything after a '##' string
    else
        text_display_end = text_end;

    ImFont* font = g.Font;
    const c_float font_size = g.FontSize;
    if (text == text_display_end)
        return ImVec2(0f32, font_size);
    ImVec2 text_size = font->CalcTextSizeA(font_size, f32::MAX, wrap_width, text, text_display_end, NULL);

    // Round
    // FIXME: This has been here since Dec 2015 (7b0bf230) but down the line we want this out.
    // FIXME: Investigate using ceilf or e.g.
    // - https://git.musl-libc.org/cgit/musl/tree/src/math/ceilf.c
    // - https://embarkstudios.github.io/rust-gpu/api/src/libm/math/ceilf.rs.html
    text_size.x = IM_FLOOR(text_size.x + 0.999990f32);

    return text_size;
}

// Find window given position, search front-to-back
// FIXME: Note that we have an inconsequential lag here: OuterRectClipped is updated in Begin(), so windows moved programmatically
// with SetWindowPos() and not SetNextWindowPos() will have that rectangle lagging by a frame at the time FindHoveredWindow() is
// called, aka before the next Begin(). Moving window isn't affected.
static c_void FindHoveredWindow()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    // Special handling for the window being moved: Ignore the mouse viewport check (because it may reset/lose its viewport during the undocking frame)
    *mut ImGuiViewportP moving_window_viewport = g.MovingWindow ? g.Movingwindow.Viewport : NULL;
    if (g.MovingWindow)
        g.Movingwindow.Viewport = g.MouseViewport;

    ImGuiWindow* hovered_window = None;
    ImGuiWindow* hovered_window_ignoring_moving_window = None;
    if (g.MovingWindow && !(g.Movingwindow.Flags & ImGuiWindowFlags_NoMouseInputs))
        hovered_window = g.MovingWindow;

    ImVec2 padding_regular = g.Style.TouchExtraPadding;
    ImVec2 padding_for_resize = g.IO.ConfigWindowsResizeFromEdges ? g.WindowsHoverPadding : padding_regular;
    for (c_int i = g.Windows.Size - 1; i >= 0; i--)
    {
        ImGuiWindow* window = g.Windows[i];
        IM_MSVC_WARNING_SUPPRESS(28182); // [Static Analyzer] Dereferencing NULL pointer.
        if (!window.Active || window.Hidden)
            continue;
        if (window.Flags & ImGuiWindowFlags_NoMouseInputs)
            continue;
        // IM_ASSERT(window.Viewport);
        if (window.Viewport != g.MouseViewport)
            continue;

        // Using the clipped AABB, a child window will typically be clipped by its parent (not always)
        ImRect bb(window.OuterRectClipped);
        if (window.Flags & (ImGuiWindowFlags_ChildWindow | ImGuiWindowFlags_NoResize | ImGuiWindowFlags_AlwaysAutoResize))
            bb.Expand(padding_regular);
        else
            bb.Expand(padding_for_resize);
        if (!bb.Contains(g.IO.MousePos))
            continue;

        // Support for one rectangular hole in any given window
        // FIXME: Consider generalizing hit-testing override (with more generic data, callback, etc.) (#1512)
        if (window.HitTestHoleSize.x != 0)
        {
            ImVec2 hole_pos(window.Pos.x + window.HitTestHoleOffset.x, window.Pos.y + window.HitTestHoleOffset.y);
            ImVec2 hole_size(window.HitTestHoleSize.x, window.HitTestHoleSize.y);
            if (ImRect(hole_pos, hole_pos + hole_size).Contains(g.IO.MousePos))
                continue;
        }

        if (hovered_window == NULL)
            hovered_window = window;
        IM_MSVC_WARNING_SUPPRESS(28182); // [Static Analyzer] Dereferencing NULL pointer.
        if (hovered_window_ignoring_moving_window == NULL && (!g.MovingWindow || window.RootWindowDockTree != g.Movingwindow.RootWindowDockTree))
            hovered_window_ignoring_moving_window = window;
        if (hovered_window && hovered_window_ignoring_moving_window)
            break;
    }

    g.HoveredWindow = hovered_window;
    g.HoveredWindowUnderMovingWindow = hovered_window_ignoring_moving_window;

    if (g.MovingWindow)
        g.Movingwindow.Viewport = moving_window_viewport;
}

bool ImGui::IsItemActive()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.ActiveId)
        return g.ActiveId == g.LastItemData.ID;
    return false;
}

bool ImGui::IsItemActivated()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.ActiveId)
        if (g.ActiveId == g.LastItemData.ID && g.ActiveIdPreviousFrame != g.LastItemData.ID)
            return true;
    return false;
}

bool ImGui::IsItemDeactivated()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.LastItemData.StatusFlags & ImGuiItemStatusFlags_HasDeactivated)
        return (g.LastItemData.StatusFlags & ImGuiItemStatusFlags_Deactivated) != 0;
    return (g.ActiveIdPreviousFrame == g.LastItemData.ID && g.ActiveIdPreviousFrame != 0 && g.ActiveId != g.LastItemData.ID);
}

bool ImGui::IsItemDeactivatedAfterEdit()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return IsItemDeactivated() && (g.ActiveIdPreviousFrameHasBeenEditedBefore || (g.ActiveId == 0 && g.ActiveIdHasBeenEditedBefore));
}

// == GetItemID() == GetFocusID()
bool ImGui::IsItemFocused()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.NavId != g.LastItemData.ID || g.NavId == 0)
        return false;

    // Special handling for the dummy item after Begin() which represent the title bar or tab.
    // When the window is collapsed (SkipItems==true) that last item will never be overwritten so we need to detect the case.
    let mut window = g.CurrentWindow;
    if (g.LastItemData.ID == window.ID && window.WriteAccessed)
        return false;

    return true;
}

// Important: this can be useful but it is NOT equivalent to the behavior of e.g.Button()!
// Most widgets have specific reactions based on mouse-up/down state, mouse position etc.
bool ImGui::IsItemClicked(ImGuiMouseButton mouse_button)
{
    return IsMouseClicked(mouse_button) && IsItemHovered(ImGuiHoveredFlags_None);
}

bool ImGui::IsItemToggledOpen()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return (g.LastItemData.StatusFlags & ImGuiItemStatusFlags_ToggledOpen) ? true : false;
}

bool ImGui::IsItemToggledSelection()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return (g.LastItemData.StatusFlags & ImGuiItemStatusFlags_ToggledSelection) ? true : false;
}

bool ImGui::IsAnyItemHovered()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.HoveredId != 0 || g.HoveredIdPreviousFrame != 0;
}

bool ImGui::IsAnyItemActive()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.ActiveId != 0;
}

bool ImGui::IsAnyItemFocused()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.NavId != 0 && !g.NavDisableHighlight;
}

bool ImGui::IsItemVisible()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.Currentwindow.ClipRect.Overlaps(g.LastItemData.Rect);
}

bool ImGui::IsItemEdited()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return (g.LastItemData.StatusFlags & ImGuiItemStatusFlags_Edited) != 0;
}

// Allow last item to be overlapped by a subsequent item. Both may be activated during the same frame before the later one takes priority.
// FIXME: Although this is exposed, its interaction and ideal idiom with using ImGuiButtonFlags_AllowItemOverlap flag are extremely confusing, need rework.
c_void ImGui::SetItemAllowOverlap()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiID id = g.LastItemData.ID;
    if (g.HoveredId == id)
        g.HoveredIdAllowOverlap = true;
    if (g.ActiveId == id)
        g.ActiveIdAllowOverlap = true;
}

c_void ImGui::SetItemUsingMouseWheel()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiID id = g.LastItemData.ID;
    if (g.HoveredId == id)
        g.HoveredIdUsingMouseWheel = true;
    if (g.ActiveId == id)
    {
        g.ActiveIdUsingKeyInputMask.SetBit(ImGuiKey_MouseWheelX);
        g.ActiveIdUsingKeyInputMask.SetBit(ImGuiKey_MouseWheelY);
    }
}

// FIXME: Technically this also prevents use of Gamepad D-Pad, may not be an issue.
c_void ImGui::SetActiveIdUsingAllKeyboardKeys()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.ActiveId != 0);
    g.ActiveIdUsingNavDirMask = ~0;
    g.ActiveIdUsingKeyInputMask.SetBitRange(ImGuiKey_Keyboard_BEGIN, ImGuiKey_Keyboard_END);
    g.ActiveIdUsingKeyInputMask.SetBit(ImGuiKey_ModCtrl);
    g.ActiveIdUsingKeyInputMask.SetBit(ImGuiKey_ModShift);
    g.ActiveIdUsingKeyInputMask.SetBit(ImGuiKey_ModAlt);
    g.ActiveIdUsingKeyInputMask.SetBit(ImGuiKey_ModSuper);
    NavMoveRequestCancel();
}

ImVec2 ImGui::GetItemRectMin()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.LastItemData.Rect.Min;
}

ImVec2 ImGui::GetItemRectMax()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.LastItemData.Rect.Max;
}

ImVec2 ImGui::GetItemRectSize()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.LastItemData.Rect.GetSize();
}

bool ImGui::BeginChildEx(*const char name, ImGuiID id, const ImVec2& size_arg, bool border, ImGuiWindowFlags flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiWindow* parent_window = g.CurrentWindow;

    flags |= ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoResize | ImGuiWindowFlags_NoSavedSettings | ImGuiWindowFlags_ChildWindow | ImGuiWindowFlags_NoDocking;
    flags |= (parent_window.Flags & ImGuiWindowFlags_NoMove);  // Inherit the NoMove flag

    // Size
    const ImVec2 content_avail = GetContentRegionAvail();
    ImVec2 size = ImFloor(size_arg);
    let auto_fit_axises: c_int = ((size.x == 0f32) ? (1 << ImGuiAxis_X) : 0x00) | ((size.y == 0f32) ? (1 << ImGuiAxis_Y) : 0x00);
    if (size.x <= 0f32)
        size.x = ImMax(content_avail.x + size.x, 4.00f32); // Arbitrary minimum child size (0f32 causing too much issues)
    if (size.y <= 0f32)
        size.y = ImMax(content_avail.y + size.y, 4.00f32);
    SetNextWindowSize(size);

    // Build up name. If you need to append to a same child from multiple location in the ID stack, use BeginChild(ImGuiID id) with a stable value.
    *const char temp_window_name;
    if (name)
        ImFormatStringToTempBuffer(&temp_window_name, NULL, "%s/%s_%08X", parent_window.Name, name, id);
    else
        ImFormatStringToTempBuffer(&temp_window_name, NULL, "%s/%08X", parent_window.Name, id);

    const c_float backup_border_size = g.Style.ChildBorderSize;
    if (!border)
        g.Style.ChildBorderSize = 0f32;
    let mut ret: bool =  Begin(temp_window_name, NULL, flags);
    g.Style.ChildBorderSize = backup_border_size;

    ImGuiWindow* child_window = g.CurrentWindow;
    child_window.ChildId = id;
    child_window.AutoFitChildAxises = auto_fit_axises;

    // Set the cursor to handle case where the user called SetNextWindowPos()+BeginChild() manually.
    // While this is not really documented/defined, it seems that the expected thing to do.
    if (child_window.BeginCount == 1)
        parent_window.DC.CursorPos = child_window.Pos;

    // Process navigation-in immediately so NavInit can run on first frame
    if (g.NavActivateId == id && !(flags & ImGuiWindowFlags_NavFlattened) && (child_window.DC.NavLayersActiveMask != 0 || child_window.DC.NavHasScroll))
    {
        FocusWindow(child_window);
        NavInitWindow(child_window, false);
        SetActiveID(id + 1, child_window); // Steal ActiveId with another arbitrary id so that key-press won't activate child item
        g.ActiveIdSource = ImGuiInputSource_Nav;
    }
    return ret;
}

bool ImGui::BeginChild(*const char str_id, const ImVec2& size_arg, bool border, ImGuiWindowFlags extra_flags)
{
    ImGuiWindow* window = GetCurrentWindow();
    return BeginChildEx(str_id, window.GetID(str_id), size_arg, border, extra_flags);
}

bool ImGui::BeginChild(ImGuiID id, const ImVec2& size_arg, bool border, ImGuiWindowFlags extra_flags)
{
    // IM_ASSERT(id != 0);
    return BeginChildEx(NULL, id, size_arg, border, extra_flags);
}

c_void ImGui::EndChild()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;

    // IM_ASSERT(g.WithinEndChild == false);
    // IM_ASSERT(window.Flags & ImGuiWindowFlags_ChildWindow);   // Mismatched BeginChild()/EndChild() calls

    g.WithinEndChild = true;
    if (window.BeginCount > 1)
    {
        End();
    }
    else
    {
        ImVec2 sz = window.Size;
        if (window.AutoFitChildAxises & (1 << ImGuiAxis_X)) // Arbitrary minimum zero-ish child size of 4.0f32 causes less trouble than a 0f32
            sz.x = ImMax(4.0f32, sz.x);
        if (window.AutoFitChildAxises & (1 << ImGuiAxis_Y))
            sz.y = ImMax(4.0f32, sz.y);
        End();

        ImGuiWindow* parent_window = g.CurrentWindow;
        ImRect bb(parent_window.DC.CursorPos, parent_window.DC.CursorPos + sz);
        ItemSize(sz);
        if ((window.DC.NavLayersActiveMask != 0 || window.DC.NavHasScroll) && !(window.Flags & ImGuiWindowFlags_NavFlattened))
        {
            ItemAdd(bb, window.ChildId);
            RenderNavHighlight(bb, window.ChildId);

            // When browsing a window that has no activable items (scroll only) we keep a highlight on the child (pass g.NavId to trick into always displaying)
            if (window.DC.NavLayersActiveMask == 0 && window == g.NavWindow)
                RenderNavHighlight(ImRect(bb.Min - ImVec2(2, 2), bb.Max + ImVec2(2, 2)), g.NavId, ImGuiNavHighlightFlags_TypeThin);
        }
        else
        {
            // Not navigable into
            ItemAdd(bb, 0);
        }
        if (g.HoveredWindow == window)
            g.LastItemData.StatusFlags |= ImGuiItemStatusFlags_HoveredWindow;
    }
    g.WithinEndChild = false;
    g.LogLinePosY = -f32::MAX; // To enforce a carriage return
}

// Helper to create a child window / scrolling region that looks like a normal widget frame.
bool ImGui::BeginChildFrame(ImGuiID id, const ImVec2& size, ImGuiWindowFlags extra_flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    const ImGuiStyle& style = g.Style;
    PushStyleColor(ImGuiCol_ChildBg, style.Colors[ImGuiCol_FrameBg]);
    PushStyleVar(ImGuiStyleVar_ChildRounding, style.FrameRounding);
    PushStyleVar(ImGuiStyleVar_ChildBorderSize, style.FrameBorderSize);
    PushStyleVar(ImGuiStyleVar_WindowPadding, style.FramePadding);
    let mut ret: bool =  BeginChild(id, size, true, ImGuiWindowFlags_NoMove | ImGuiWindowFlags_AlwaysUseWindowPadding | extra_flags);
    PopStyleVar(3);
    PopStyleColor();
    return ret;
}

c_void ImGui::EndChildFrame()
{
    EndChild();
}

static c_void SetWindowConditionAllowFlags(ImGuiWindow* window, ImGuiCond flags, bool enabled)
{
    window.SetWindowPosAllowFlags       = enabled ? (window.SetWindowPosAllowFlags       | flags) : (window.SetWindowPosAllowFlags       & ~flags);
    window.SetWindowSizeAllowFlags      = enabled ? (window.SetWindowSizeAllowFlags      | flags) : (window.SetWindowSizeAllowFlags      & ~flags);
    window.SetWindowCollapsedAllowFlags = enabled ? (window.SetWindowCollapsedAllowFlags | flags) : (window.SetWindowCollapsedAllowFlags & ~flags);
    window.SetWindowDockAllowFlags      = enabled ? (window.SetWindowDockAllowFlags      | flags) : (window.SetWindowDockAllowFlags      & ~flags);
}

ImGuiWindow* ImGui::FindWindowByID(ImGuiID id)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return (ImGuiWindow*)g.WindowsById.GetVoidPtr(id);
}

ImGuiWindow* ImGui::FindWindowByName(*const char name)
{
    ImGuiID id = ImHashStr(name);
    return FindWindowByID(id);
}

static c_void ApplyWindowSettings(ImGuiWindow* window, ImGuiWindowSettings* settings)
{
    *const ImGuiViewport main_viewport = ImGui::GetMainViewport();
    window.ViewportPos = main_viewport->Pos;
    if (settings->ViewportId)
    {
        window.ViewportId = settings->ViewportId;
        window.ViewportPos = ImVec2(settings->ViewportPos.x, settings->ViewportPos.y);
    }
    window.Pos = ImFloor(ImVec2(settings->Pos.x + window.ViewportPos.x, settings->Pos.y + window.ViewportPos.y));
    if (settings->Size.x > 0 && settings->Size.y > 0)
        window.Size = window.SizeFull = ImFloor(ImVec2(settings->Size.x, settings->Size.y));
    window.Collapsed = settings->Collapsed;
    window.DockId = settings->DockId;
    window.DockOrder = settings->DockOrder;
}

static c_void UpdateWindowInFocusOrderList(ImGuiWindow* window, bool just_created, ImGuiWindowFlags new_flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let new_is_explicit_child: bool = (new_flags & ImGuiWindowFlags_ChildWindow) != 0;
    let child_flag_changed: bool = new_is_explicit_child != window.IsExplicitChild;
    if ((just_created || child_flag_changed) && !new_is_explicit_child)
    {
        // IM_ASSERT(!g.WindowsFocusOrder.contains(window));
        g.WindowsFocusOrder.push(window);
        window.FocusOrder = (c_short)(g.WindowsFocusOrder.Size - 1);
    }
    else if (!just_created && child_flag_changed && new_is_explicit_child)
    {
        // IM_ASSERT(g.WindowsFocusOrder[window.FocusOrder] == window);
        for (c_int n = window.FocusOrder + 1; n < g.WindowsFocusOrder.Size; n++)
            g.WindowsFocusOrder[n]->FocusOrder-= 1;
        g.WindowsFocusOrder.erase(g.WindowsFocusOrder.Data + window.FocusOrder);
        window.FocusOrder = -1;
    }
    window.IsExplicitChild = new_is_explicit_child;
}

static ImGuiWindow* CreateNewWindow(*const char name, ImGuiWindowFlags flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    //IMGUI_DEBUG_LOG("CreateNewWindow '%s', flags = 0x%08X\n", name, flags);

    // Create window the first time
    ImGuiWindow* window = IM_NEW(ImGuiWindow)(&g, name);
    window.Flags = flags;
    g.WindowsById.SetVoidPtr(window.ID, window);

    // Default/arbitrary window position. Use SetNextWindowPos() with the appropriate condition flag to change the initial position of a window.
    *const ImGuiViewport main_viewport = ImGui::GetMainViewport();
    window.Pos = main_viewport->Pos + ImVec2(60, 60);
    window.ViewportPos = main_viewport->Pos;

    // User can disable loading and saving of settings. Tooltip and child windows also don't store settings.
    if (!(flags & ImGuiWindowFlags_NoSavedSettings))
        if (ImGuiWindowSettings* settings = ImGui::FindWindowSettings(window.ID))
        {
            // Retrieve settings from .ini file
            window.SettingsOffset = g.SettingsWindows.offset_from_ptr(settings);
            SetWindowConditionAllowFlags(window, ImGuiCond_FirstUseEver, false);
            ApplyWindowSettings(window, settings);
        }
    window.DC.CursorStartPos = window.DC.CursorMaxPos = window.DC.IdealMaxPos = window.Pos; // So first call to CalcWindowContentSizes() doesn't return crazy values

    if ((flags & ImGuiWindowFlags_AlwaysAutoResize) != 0)
    {
        window.AutoFitFramesX = window.AutoFitFramesY = 2;
        window.AutoFitOnlyGrows = false;
    }
    else
    {
        if (window.Size.x <= 0f32)
            window.AutoFitFramesX = 2;
        if (window.Size.y <= 0f32)
            window.AutoFitFramesY = 2;
        window.AutoFitOnlyGrows = (window.AutoFitFramesX > 0) || (window.AutoFitFramesY > 0);
    }

    if (flags & ImGuiWindowFlags_NoBringToFrontOnFocus)
        g.Windows.push_front(window); // Quite slow but rare and only once
    else
        g.Windows.push(window);

    return window;
}

static ImGuiWindow* GetWindowForTitleDisplay(ImGuiWindow* window)
{
    return window.DockNodeAsHost ? window.DockNodeAsHost->VisibleWindow : window;
}

static ImGuiWindow* GetWindowForTitleAndMenuHeight(ImGuiWindow* window)
{
    return (window.DockNodeAsHost && window.DockNodeAsHost->VisibleWindow) ? window.DockNodeAsHost->VisibleWindow : window;
}

static ImVec2 CalcWindowSizeAfterConstraint(ImGuiWindow* window, const ImVec2& size_desired)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImVec2 new_size = size_desired;
    if (g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasSizeConstraint)
    {
        // Using -1,-1 on either X/Y axis to preserve the current size.
        ImRect cr = g.NextWindowData.SizeConstraintRect;
        new_size.x = (cr.Min.x >= 0 && cr.Max.x >= 0) ? ImClamp(new_size.x, cr.Min.x, cr.Max.x) : window.SizeFull.x;
        new_size.y = (cr.Min.y >= 0 && cr.Max.y >= 0) ? ImClamp(new_size.y, cr.Min.y, cr.Max.y) : window.SizeFull.y;
        if (g.NextWindowData.SizeCallback)
        {
            ImGuiSizeCallbackData data;
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
    if (!(window.Flags & (ImGuiWindowFlags_ChildWindow | ImGuiWindowFlags_AlwaysAutoResize)))
    {
        ImGuiWindow* window_for_height = GetWindowForTitleAndMenuHeight(window);
        const c_float decoration_up_height = window_for_height->TitleBarHeight() + window_for_height->MenuBarHeight();
        new_size = ImMax(new_size, g.Style.WindowMinSize);
        new_size.y = ImMax(new_size.y, decoration_up_height + ImMax(0f32, g.Style.WindowRounding - 1f32)); // Reduce artifacts with very small windows
    }
    return new_size;
}

static c_void CalcWindowContentSizes(ImGuiWindow* window, ImVec2* content_size_current, ImVec2* content_size_ideal)
{
    let mut preserve_old_content_sizes: bool =  false;
    if (window.Collapsed && window.AutoFitFramesX <= 0 && window.AutoFitFramesY <= 0)
        preserve_old_content_sizes = true;
    else if (window.Hidden && window.HiddenFramesCannotSkipItems == 0 && window.HiddenFramesCanSkipItems > 0)
        preserve_old_content_sizes = true;
    if (preserve_old_content_sizes)
    {
        *content_size_current = window.ContentSize;
        *content_size_ideal = window.ContentSizeIdeal;
        return;
    }

    content_size_current->x = (window.ContentSizeExplicit.x != 0f32) ? window.ContentSizeExplicit.x : IM_FLOOR(window.DC.CursorMaxPos.x - window.DC.CursorStartPos.x);
    content_size_current->y = (window.ContentSizeExplicit.y != 0f32) ? window.ContentSizeExplicit.y : IM_FLOOR(window.DC.CursorMaxPos.y - window.DC.CursorStartPos.y);
    content_size_ideal->x = (window.ContentSizeExplicit.x != 0f32) ? window.ContentSizeExplicit.x : IM_FLOOR(ImMax(window.DC.CursorMaxPos.x, window.DC.IdealMaxPos.x) - window.DC.CursorStartPos.x);
    content_size_ideal->y = (window.ContentSizeExplicit.y != 0f32) ? window.ContentSizeExplicit.y : IM_FLOOR(ImMax(window.DC.CursorMaxPos.y, window.DC.IdealMaxPos.y) - window.DC.CursorStartPos.y);
}

static ImVec2 CalcWindowAutoFitSize(ImGuiWindow* window, const ImVec2& size_contents)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiStyle& style = g.Style;
    const c_float decoration_up_height = window.TitleBarHeight() + window.MenuBarHeight();
    ImVec2 size_pad = window.WindowPadding * 2.0f32;
    ImVec2 size_desired = size_contents + size_pad + ImVec2(0f32, decoration_up_height);
    if (window.Flags & ImGuiWindowFlags_Tooltip)
    {
        // Tooltip always resize
        return size_desired;
    }
    else
    {
        // Maximum window size is determined by the viewport size or monitor size
        let is_popup: bool = (window.Flags & ImGuiWindowFlags_Popup) != 0;
        let is_menu: bool = (window.Flags & ImGuiWindowFlags_ChildMenu) != 0;
        ImVec2 size_min = style.WindowMinSize;
        if (is_popup || is_menu) // Popups and menus bypass style.WindowMinSize by default, but we give then a non-zero minimum size to facilitate understanding problematic cases (e.g. empty popups)
            size_min = ImMin(size_min, ImVec2(4.0f32, 4.00f32));

        // FIXME-VIEWPORT-WORKAREA: May want to use GetWorkSize() instead of Size depending on the type of windows?
        ImVec2 avail_size = window.Viewport->Size;
        if (window.ViewportOwned)
            avail_size = ImVec2(f32::MAX, f32::MAX);
        let monitor_idx: c_int = window.ViewportAllowPlatformMonitorExtend;
        if (monitor_idx >= 0 && monitor_idx < g.PlatformIO.Monitors.Size)
            avail_size = g.PlatformIO.Monitors[monitor_idx].WorkSize;
        ImVec2 size_auto_fit = ImClamp(size_desired, size_min, ImMax(size_min, avail_size - style.DisplaySafeAreaPadding * 2.00f32));

        // When the window cannot fit all contents (either because of constraints, either because screen is too small),
        // we are growing the size on the other axis to compensate for expected scrollbar. FIXME: Might turn bigger than ViewportSize-WindowPadding.
        ImVec2 size_auto_fit_after_constraint = CalcWindowSizeAfterConstraint(window, size_auto_fit);
        let mut will_have_scrollbar_x: bool =  (size_auto_fit_after_constraint.x - size_pad.x - 0f32                 < size_contents.x && !(window.Flags & ImGuiWindowFlags_NoScrollbar) && (window.Flags & ImGuiWindowFlags_HorizontalScrollbar)) || (window.Flags & ImGuiWindowFlags_AlwaysHorizontalScrollbar);
        let mut will_have_scrollbar_y: bool =  (size_auto_fit_after_constraint.y - size_pad.y - decoration_up_height < size_contents.y && !(window.Flags & ImGuiWindowFlags_NoScrollbar)) || (window.Flags & ImGuiWindowFlags_AlwaysVerticalScrollbar);
        if (will_have_scrollbar_x)
            size_auto_fit.y += style.ScrollbarSize;
        if (will_have_scrollbar_y)
            size_auto_fit.x += style.ScrollbarSize;
        return size_auto_fit;
    }
}

ImVec2 ImGui::CalcWindowNextAutoFitSize(ImGuiWindow* window)
{
    ImVec2 size_contents_current;
    ImVec2 size_contents_ideal;
    CalcWindowContentSizes(window, &size_contents_current, &size_contents_ideal);
    ImVec2 size_auto_fit = CalcWindowAutoFitSize(window, size_contents_ideal);
    ImVec2 size_final = CalcWindowSizeAfterConstraint(window, size_auto_fit);
    return size_final;
}

static ImGuiCol GetWindowBgColorIdx(ImGuiWindow* window)
{
    if (window.Flags & (ImGuiWindowFlags_Tooltip | ImGuiWindowFlags_Popup))
        return ImGuiCol_PopupBg;
    if ((window.Flags & ImGuiWindowFlags_ChildWindow) && !window.DockIsActive)
        return ImGuiCol_ChildBg;
    return ImGuiCol_WindowBg;
}

static c_void CalcResizePosSizeFromAnyCorner(ImGuiWindow* window, const ImVec2& corner_target, const ImVec2& corner_norm, ImVec2* out_pos, ImVec2* out_size)
{
    ImVec2 pos_min = ImLerp(corner_target, window.Pos, corner_norm);                // Expected window upper-left
    ImVec2 pos_max = ImLerp(window.Pos + window.Size, corner_target, corner_norm); // Expected window lower-right
    ImVec2 size_expected = pos_max - pos_min;
    ImVec2 size_constrained = CalcWindowSizeAfterConstraint(window, size_expected);
    *out_pos = pos_min;
    if (corner_norm.x == 0f32)
        out_pos->x -= (size_constrained.x - size_expected.x);
    if (corner_norm.y == 0f32)
        out_pos->y -= (size_constrained.y - size_expected.y);
    *out_size = size_constrained;
}

// Data for resizing from corner
struct ImGuiResizeGripDef
{
    ImVec2  CornerPosN;
    ImVec2  InnerDir;
    c_int     AngleMin12, AngleMax12;
};
static const ImGuiResizeGripDef resize_grip_def[4] =
{
    { ImVec2(1, 1), ImVec2(-1, -1), 0, 3 },  // Lower-right
    { ImVec2(0, 1), ImVec2(+1, -1), 3, 6 },  // Lower-left
    { ImVec2(0, 0), ImVec2(+1, +1), 6, 9 },  // Upper-left (Unused)
    { ImVec2(1, 0), ImVec2(-1, +1), 9, 12 }  // Upper-right (Unused)
};

// Data for resizing from borders
struct ImGuiResizeBorderDef
{
    ImVec2 InnerDir;
    ImVec2 SegmentN1, SegmentN2;
    c_float  OuterAngle;
};
static const ImGuiResizeBorderDef resize_border_def[4] =
{
    { ImVec2(+1, 0), ImVec2(0, 1), ImVec2(0, 0), IM_PI * 1f32 }, // Left
    { ImVec2(-1, 0), ImVec2(1, 0), ImVec2(1, 1), IM_PI * 0.00f32 }, // Right
    { ImVec2(0, +1), ImVec2(0, 0), ImVec2(1, 0), IM_PI * 1.50f32 }, // Up
    { ImVec2(0, -1), ImVec2(1, 1), ImVec2(0, 1), IM_PI * 0.50f32 }  // Down
};

static ImRect GetResizeBorderRect(ImGuiWindow* window, c_int border_n, c_float perp_padding, c_float thickness)
{
    ImRect rect = window.Rect();
    if (thickness == 0f32)
        rect.Max -= ImVec2(1, 1);
    if (border_n == ImGuiDir_Left)  { return ImRect(rect.Min.x - thickness,    rect.Min.y + perp_padding, rect.Min.x + thickness,    rect.Max.y - perp_padding); }
    if (border_n == ImGuiDir_Right) { return ImRect(rect.Max.x - thickness,    rect.Min.y + perp_padding, rect.Max.x + thickness,    rect.Max.y - perp_padding); }
    if (border_n == ImGuiDir_Up)    { return ImRect(rect.Min.x + perp_padding, rect.Min.y - thickness,    rect.Max.x - perp_padding, rect.Min.y + thickness);    }
    if (border_n == ImGuiDir_Down)  { return ImRect(rect.Min.x + perp_padding, rect.Max.y - thickness,    rect.Max.x - perp_padding, rect.Max.y + thickness);    }
    // IM_ASSERT(0);
    return ImRect();
}

// 0..3: corners (Lower-right, Lower-left, Unused, Unused)
ImGuiID ImGui::GetWindowResizeCornerID(ImGuiWindow* window, c_int n)
{
    // IM_ASSERT(n >= 0 && n < 4);
    ImGuiID id = window.DockIsActive ? window.DockNode->Hostwindow.ID : window.ID;
    id = ImHashStr("#RESIZE", 0, id);
    id = ImHashData(&n, sizeof, id);
    return id;
}

// Borders (Left, Right, Up, Down)
ImGuiID ImGui::GetWindowResizeBorderID(ImGuiWindow* window, ImGuiDir dir)
{
    // IM_ASSERT(dir >= 0 && dir < 4);
    c_int n = dir + 4;
    ImGuiID id = window.DockIsActive ? window.DockNode->Hostwindow.ID : window.ID;
    id = ImHashStr("#RESIZE", 0, id);
    id = ImHashData(&n, sizeof, id);
    return id;
}

// Handle resize for: Resize Grips, Borders, Gamepad
// Return true when using auto-fit (double click on resize grip)
static bool ImGui::UpdateWindowManualResize(ImGuiWindow* window, const ImVec2& size_auto_fit, c_int* border_held, c_int resize_grip_count, u32 resize_grip_col[4], const ImRect& visibility_rect)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiWindowFlags flags = window.Flags;

    if ((flags & ImGuiWindowFlags_NoResize) || (flags & ImGuiWindowFlags_AlwaysAutoResize) || window.AutoFitFramesX > 0 || window.AutoFitFramesY > 0)
        return false;
    if (window.WasActive == false) // Early out to avoid running this code for e.g. an hidden implicit/fallback Debug window.
        return false;

    let mut ret_auto_fit: bool =  false;
    let resize_border_count: c_int = g.IO.ConfigWindowsResizeFromEdges ? 4 : 0;
    const c_float grip_draw_size = IM_FLOOR(ImMax(g.FontSize * 1.35f, window.WindowRounding + 1f32 + g.FontSize * 0.20f32));
    const c_float grip_hover_inner_size = IM_FLOOR(grip_draw_size * 0.750f32);
    const c_float grip_hover_outer_size = g.IO.ConfigWindowsResizeFromEdges ? WINDOWS_HOVER_PADDING : 0f32;

    ImVec2 pos_target(f32::MAX, f32::MAX);
    ImVec2 size_target(f32::MAX, f32::MAX);

    // Clip mouse interaction rectangles within the viewport rectangle (in practice the narrowing is going to happen most of the time).
    // - Not narrowing would mostly benefit the situation where OS windows _without_ decoration have a threshold for hovering when outside their limits.
    //   This is however not the case with current backends under Win32, but a custom borderless window implementation would benefit from it.
    // - When decoration are enabled we typically benefit from that distance, but then our resize elements would be conflicting with OS resize elements, so we also narrow.
    // - Note that we are unable to tell if the platform setup allows hovering with a distance threshold (on Win32, decorated window have such threshold).
    // We only clip interaction so we overwrite window.ClipRect, cannot call PushClipRect() yet as DrawList is not yet setup.
    let clip_with_viewport_rect: bool = !(g.IO.BackendFlags & ImGuiBackendFlags_HasMouseHoveredViewport) || (g.IO.MouseHoveredViewport != window.ViewportId) || !(window.Viewport->Flags & ImGuiViewportFlags_NoDecoration);
    if (clip_with_viewport_rect)
        window.ClipRect = window.Viewport->GetMainRect();

    // Resize grips and borders are on layer 1
    window.DC.NavLayerCurrent = ImGuiNavLayer_Menu;

    // Manual resize grips
    PushID("#RESIZE");
    for (c_int resize_grip_n = 0; resize_grip_n < resize_grip_count; resize_grip_n++)
    {
        const ImGuiResizeGripDef& def = resize_grip_def[resize_grip_n];
        const ImVec2 corner = ImLerp(window.Pos, window.Pos + window.Size, def.CornerPosN);

        // Using the FlattenChilds button flag we make the resize button accessible even if we are hovering over a child window
        bool hovered, held;
        ImRect resize_rect(corner - def.InnerDir * grip_hover_outer_size, corner + def.InnerDir * grip_hover_inner_size);
        if (resize_rect.Min.x > resize_rect.Max.x) ImSwap(resize_rect.Min.x, resize_rect.Max.x);
        if (resize_rect.Min.y > resize_rect.Max.y) ImSwap(resize_rect.Min.y, resize_rect.Max.y);
        ImGuiID resize_grip_id = window.GetID(resize_grip_n); // == GetWindowResizeCornerID()
        KeepAliveID(resize_grip_id);
        ButtonBehavior(resize_rect, resize_grip_id, &hovered, &held, ImGuiButtonFlags_FlattenChildren | ImGuiButtonFlags_NoNavFocus);
        //GetForegroundDrawList(window)->AddRect(resize_rect.Min, resize_rect.Max, IM_COL32(255, 255, 0, 255));
        if (hovered || held)
            g.MouseCursor = (resize_grip_n & 1) ? ImGuiMouseCursor_ResizeNESW : ImGuiMouseCursor_ResizeNWSE;

        if (held && g.IO.MouseClickedCount[0] == 2 && resize_grip_n == 0)
        {
            // Manual auto-fit when double-clicking
            size_target = CalcWindowSizeAfterConstraint(window, size_auto_fit);
            ret_auto_fit = true;
            ClearActiveID();
        }
        else if (held)
        {
            // Resize from any of the four corners
            // We don't use an incremental MouseDelta but rather compute an absolute target size based on mouse position
            ImVec2 clamp_min = ImVec2(def.CornerPosN.x == 1f32 ? visibility_rect.Min.x : -f32::MAX, def.CornerPosN.y == 1f32 ? visibility_rect.Min.y : -f32::MAX);
            ImVec2 clamp_max = ImVec2(def.CornerPosN.x == 0f32 ? visibility_rect.Max.x : +f32::MAX, def.CornerPosN.y == 0f32 ? visibility_rect.Max.y : +f32::MAX);
            ImVec2 corner_target = g.IO.MousePos - g.ActiveIdClickOffset + ImLerp(def.InnerDir * grip_hover_outer_size, def.InnerDir * -grip_hover_inner_size, def.CornerPosN); // Corner of the window corresponding to our corner grip
            corner_target = ImClamp(corner_target, clamp_min, clamp_max);
            CalcResizePosSizeFromAnyCorner(window, corner_target, def.CornerPosN, &pos_target, &size_target);
        }

        // Only lower-left grip is visible before hovering/activating
        if (resize_grip_n == 0 || held || hovered)
            resize_grip_col[resize_grip_n] = GetColorU32(held ? ImGuiCol_ResizeGripActive : hovered ? ImGuiCol_ResizeGripHovered : ImGuiCol_ResizeGrip);
    }
    for (c_int border_n = 0; border_n < resize_border_count; border_n++)
    {
        const ImGuiResizeBorderDef& def = resize_border_def[border_n];
        const ImGuiAxis axis = (border_n == ImGuiDir_Left || border_n == ImGuiDir_Right) ? ImGuiAxis_X : ImGuiAxis_Y;

        bool hovered, held;
        ImRect border_rect = GetResizeBorderRect(window, border_n, grip_hover_inner_size, WINDOWS_HOVER_PADDING);
        ImGuiID border_id = window.GetID(border_n + 4); // == GetWindowResizeBorderID()
        KeepAliveID(border_id);
        ButtonBehavior(border_rect, border_id, &hovered, &held, ImGuiButtonFlags_FlattenChildren | ImGuiButtonFlags_NoNavFocus);
        //GetForegroundDrawLists(window)->AddRect(border_rect.Min, border_rect.Max, IM_COL32(255, 255, 0, 255));
        if ((hovered && g.HoveredIdTimer > WINDOWS_RESIZE_FROM_EDGES_FEEDBACK_TIMER) || held)
        {
            g.MouseCursor = (axis == ImGuiAxis_X) ? ImGuiMouseCursor_ResizeEW : ImGuiMouseCursor_ResizeNS;
            if (held)
                *border_held = border_n;
        }
        if (held)
        {
            ImVec2 clamp_min(border_n == ImGuiDir_Right ? visibility_rect.Min.x : -f32::MAX, border_n == ImGuiDir_Down ? visibility_rect.Min.y : -f32::MAX);
            ImVec2 clamp_max(border_n == ImGuiDir_Left  ? visibility_rect.Max.x : +f32::MAX, border_n == ImGuiDir_Up   ? visibility_rect.Max.y : +f32::MAX);
            ImVec2 border_target = window.Pos;
            border_target[axis] = g.IO.MousePos[axis] - g.ActiveIdClickOffset[axis] + WINDOWS_HOVER_PADDING;
            border_target = ImClamp(border_target, clamp_min, clamp_max);
            CalcResizePosSizeFromAnyCorner(window, border_target, ImMin(def.SegmentN1, def.SegmentN2), &pos_target, &size_target);
        }
    }
    PopID();

    // Restore nav layer
    window.DC.NavLayerCurrent = ImGuiNavLayer_Main;

    // Navigation resize (keyboard/gamepad)
    // FIXME: This cannot be moved to NavUpdateWindowing() because CalcWindowSizeAfterConstraint() need to callback into user.
    // Not even sure the callback works here.
    if (g.NavWindowingTarget && g.NavWindowingTarget->RootWindowDockTree == window)
    {
        ImVec2 nav_resize_dir;
        if (g.NavInputSource == ImGuiInputSource_Keyboard && g.IO.KeyShift)
            nav_resize_dir = GetKeyVector2d(ImGuiKey_LeftArrow, ImGuiKey_RightArrow, ImGuiKey_UpArrow, ImGuiKey_DownArrow);
        if (g.NavInputSource == ImGuiInputSource_Gamepad)
            nav_resize_dir = GetKeyVector2d(ImGuiKey_GamepadDpadLeft, ImGuiKey_GamepadDpadRight, ImGuiKey_GamepadDpadUp, ImGuiKey_GamepadDpadDown);
        if (nav_resize_dir.x != 0f32 || nav_resize_dir.y != 0f32)
        {
            const c_float NAV_RESIZE_SPEED = 600f32;
            const c_float resize_step = NAV_RESIZE_SPEED * g.IO.DeltaTime * ImMin(g.IO.DisplayFramebufferScale.x, g.IO.DisplayFramebufferScale.y);
            g.NavWindowingAccumDeltaSize += nav_resize_dir * resize_step;
            g.NavWindowingAccumDeltaSize = ImMax(g.NavWindowingAccumDeltaSize, visibility_rect.Min - window.Pos - window.Size); // We need Pos+Size >= visibility_rect.Min, so Size >= visibility_rect.Min - Pos, so size_delta >= visibility_rect.Min - window.Pos - window.Size
            g.NavWindowingToggleLayer = false;
            g.NavDisableMouseHover = true;
            resize_grip_col[0] = GetColorU32(ImGuiCol_ResizeGripActive);
            ImVec2 accum_floored = ImFloor(g.NavWindowingAccumDeltaSize);
            if (accum_floored.x != 0f32 || accum_floored.y != 0f32)
            {
                // FIXME-NAV: Should store and accumulate into a separate size buffer to handle sizing constraints properly, right now a constraint will make us stuck.
                size_target = CalcWindowSizeAfterConstraint(window, window.SizeFull + accum_floored);
                g.NavWindowingAccumDeltaSize -= accum_floored;
            }
        }
    }

    // Apply back modified position/size to window
    if (size_target.x != f32::MAX)
    {
        window.SizeFull = size_target;
        MarkIniSettingsDirty(window);
    }
    if (pos_target.x != f32::MAX)
    {
        window.Pos = ImFloor(pos_target);
        MarkIniSettingsDirty(window);
    }

    window.Size = window.SizeFull;
    return ret_auto_fit;
}

static inline c_void ClampWindowRect(ImGuiWindow* window, const ImRect& visibility_rect)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImVec2 size_for_clamping = window.Size;
    if (g.IO.ConfigWindowsMoveFromTitleBarOnly && (!(window.Flags & ImGuiWindowFlags_NoTitleBar) || window.DockNodeAsHost))
        size_for_clamping.y = ImGui::GetFrameHeight(); // Not using window.TitleBarHeight() as DockNodeAsHost will report 0f32 here.
    window.Pos = ImClamp(window.Pos, visibility_rect.Min - size_for_clamping, visibility_rect.Max);
}

static c_void ImGui::RenderWindowOuterBorders(ImGuiWindow* window)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    c_float rounding = window.WindowRounding;
    c_float border_size = window.WindowBorderSize;
    if (border_size > 0f32 && !(window.Flags & ImGuiWindowFlags_NoBackground))
        window.DrawList.AddRect(window.Pos, window.Pos + window.Size, GetColorU32(ImGuiCol_Border), rounding, 0, border_size);

    c_int border_held = window.ResizeBorderHeld;
    if (border_held != -1)
    {
        const ImGuiResizeBorderDef& def = resize_border_def[border_held];
        ImRect border_r = GetResizeBorderRect(window, border_held, rounding, 0f32);
        window.DrawList.PathArcTo(ImLerp(border_r.Min, border_r.Max, def.SegmentN1) + ImVec2(0.5f32, 0.5f32) + def.InnerDir * rounding, rounding, def.OuterAngle - IM_PI * 0.25f, def.OuterAngle);
        window.DrawList.PathArcTo(ImLerp(border_r.Min, border_r.Max, def.SegmentN2) + ImVec2(0.5f32, 0.5f32) + def.InnerDir * rounding, rounding, def.OuterAngle, def.OuterAngle + IM_PI * 0.250f32);
        window.DrawList.PathStroke(GetColorU32(ImGuiCol_SeparatorActive), 0, ImMax(2.0f32, border_size)); // Thicker than usual
    }
    if (g.Style.FrameBorderSize > 0 && !(window.Flags & ImGuiWindowFlags_NoTitleBar) && !window.DockIsActive)
    {
        c_float y = window.Pos.y + window.TitleBarHeight() - 1;
        window.DrawList.AddLine(ImVec2(window.Pos.x + border_size, y), ImVec2(window.Pos.x + window.Size.x - border_size, y), GetColorU32(ImGuiCol_Border), g.Style.FrameBorderSize);
    }
}

// Draw background and borders
// Draw and handle scrollbars
c_void ImGui::RenderWindowDecorations(ImGuiWindow* window, const ImRect& title_bar_rect, bool title_bar_is_highlight, bool handle_borders_and_resize_grips, c_int resize_grip_count, const u32 resize_grip_col[4], c_float resize_grip_draw_size)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiStyle& style = g.Style;
    ImGuiWindowFlags flags = window.Flags;

    // Ensure that ScrollBar doesn't read last frame's SkipItems
    // IM_ASSERT(window.BeginCount == 0);
    window.SkipItems = false;

    // Draw window + handle manual resize
    // As we highlight the title bar when want_focus is set, multiple reappearing windows will have have their title bar highlighted on their reappearing frame.
    const c_float window_rounding = window.WindowRounding;
    const c_float window_border_size = window.WindowBorderSize;
    if (window.Collapsed)
    {
        // Title bar only
        c_float backup_border_size = style.FrameBorderSize;
        g.Style.FrameBorderSize = window.WindowBorderSize;
        u32 title_bar_col = GetColorU32((title_bar_is_highlight && !g.NavDisableHighlight) ? ImGuiCol_TitleBgActive : ImGuiCol_TitleBgCollapsed);
        RenderFrame(title_bar_rect.Min, title_bar_rect.Max, title_bar_col, true, window_rounding);
        g.Style.FrameBorderSize = backup_border_size;
    }
    else
    {
        // Window background
        if (!(flags & ImGuiWindowFlags_NoBackground))
        {
            let mut is_docking_transparent_payload: bool =  false;
            if (g.DragDropActive && (g.FrameCount - g.DragDropAcceptFrameCount) <= 1 && g.IO.ConfigDockingTransparentPayload)
                if (g.DragDropPayload.IsDataType(IMGUI_PAYLOAD_TYPE_WINDOW) && *(ImGuiWindow**)g.DragDropPayload.Data == window)
                    is_docking_transparent_payload = true;

            u32 bg_col = GetColorU32(GetWindowBgColorIdx(window));
            if (window.ViewportOwned)
            {
                // No alpha
                bg_col = (bg_col | IM_COL32_A_MASK);
                if (is_docking_transparent_payload)
                    window.Viewport->Alpha *= DOCKING_TRANSPARENT_PAYLOAD_ALPHA;
            }
            else
            {
                // Adjust alpha. For docking
                let mut override_alpha: bool =  false;
                c_float alpha = 1f32;
                if (g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasBgAlpha)
                {
                    alpha = g.NextWindowData.BgAlphaVal;
                    override_alpha = true;
                }
                if (is_docking_transparent_payload)
                {
                    alpha *= DOCKING_TRANSPARENT_PAYLOAD_ALPHA; // FIXME-DOCK: Should that be an override?
                    override_alpha = true;
                }
                if (override_alpha)
                    bg_col = (bg_col & ~IM_COL32_A_MASK) | (IM_F32_TO_INT8_SAT(alpha) << IM_COL32_A_SHIFT);
            }

            // Render, for docked windows and host windows we ensure bg goes before decorations
            if (window.DockIsActive)
                window.DockNode->LastBgColor = bg_col;
            ImDrawList* bg_draw_list = window.DockIsActive ? window.DockNode->Hostwindow.DrawList : window.DrawList;
            if (window.DockIsActive || (flags & ImGuiWindowFlags_DockNodeHost))
                bg_draw_list->ChannelsSetCurrent(DOCKING_HOST_DRAW_CHANNEL_BG);
            bg_draw_list->AddRectFilled(window.Pos + ImVec2(0, window.TitleBarHeight()), window.Pos + window.Size, bg_col, window_rounding, (flags & ImGuiWindowFlags_NoTitleBar) ? 0 : ImDrawFlags_RoundCornersBottom);
            if (window.DockIsActive || (flags & ImGuiWindowFlags_DockNodeHost))
                bg_draw_list->ChannelsSetCurrent(DOCKING_HOST_DRAW_CHANNEL_FG);
        }
        if (window.DockIsActive)
            window.DockNode->IsBgDrawnThisFrame = true;

        // Title bar
        // (when docked, DockNode are drawing their own title bar. Individual windows however do NOT set the _NoTitleBar flag,
        // in order for their pos/size to be matching their undocking state.)
        if (!(flags & ImGuiWindowFlags_NoTitleBar) && !window.DockIsActive)
        {
            u32 title_bar_col = GetColorU32(title_bar_is_highlight ? ImGuiCol_TitleBgActive : ImGuiCol_TitleBg);
            window.DrawList.AddRectFilled(title_bar_rect.Min, title_bar_rect.Max, title_bar_col, window_rounding, ImDrawFlags_RoundCornersTop);
        }

        // Menu bar
        if (flags & ImGuiWindowFlags_MenuBar)
        {
            ImRect menu_bar_rect = window.MenuBarRect();
            menu_bar_rect.ClipWith(window.Rect());  // Soft clipping, in particular child window don't have minimum size covering the menu bar so this is useful for them.
            window.DrawList.AddRectFilled(menu_bar_rect.Min + ImVec2(window_border_size, 0), menu_bar_rect.Max - ImVec2(window_border_size, 0), GetColorU32(ImGuiCol_MenuBarBg), (flags & ImGuiWindowFlags_NoTitleBar) ? window_rounding : 0f32, ImDrawFlags_RoundCornersTop);
            if (style.FrameBorderSize > 0f32 && menu_bar_rect.Max.y < window.Pos.y + window.Size.y)
                window.DrawList.AddLine(menu_bar_rect.GetBL(), menu_bar_rect.GetBR(), GetColorU32(ImGuiCol_Border), style.FrameBorderSize);
        }

        // Docking: Unhide tab bar (small triangle in the corner), drag from small triangle to quickly undock
        ImGuiDockNode* node = window.DockNode;
        if (window.DockIsActive && node->IsHiddenTabBar() && !node->IsNoTabBar())
        {
            c_float unhide_sz_draw = ImFloor(g.FontSize * 0.700f32);
            c_float unhide_sz_hit = ImFloor(g.FontSize * 0.550f32);
            ImVec2 p = node->Pos;
            ImRect r(p, p + ImVec2(unhide_sz_hit, unhide_sz_hit));
            ImGuiID unhide_id = window.GetID("#UNHIDE");
            KeepAliveID(unhide_id);
            bool hovered, held;
            if (ButtonBehavior(r, unhide_id, &hovered, &held, ImGuiButtonFlags_FlattenChildren))
                node->WantHiddenTabBarToggle = true;
            else if (held && IsMouseDragging(0))
                StartMouseMovingWindowOrNode(window, node, true);

            // FIXME-DOCK: Ideally we'd use ImGuiCol_TitleBgActive/ImGuiCol_TitleBg here, but neither is guaranteed to be visible enough at this sort of size..
            u32 col = GetColorU32(((held && hovered) || (node->IsFocused && !hovered)) ? ImGuiCol_ButtonActive : hovered ? ImGuiCol_ButtonHovered : ImGuiCol_Button);
            window.DrawList.AddTriangleFilled(p, p + ImVec2(unhide_sz_draw, 0f32), p + ImVec2(0f32, unhide_sz_draw), col);
        }

        // Scrollbars
        if (window.ScrollbarX)
            Scrollbar(ImGuiAxis_X);
        if (window.ScrollbarY)
            Scrollbar(ImGuiAxis_Y);

        // Render resize grips (after their input handling so we don't have a frame of latency)
        if (handle_borders_and_resize_grips && !(flags & ImGuiWindowFlags_NoResize))
        {
            for (c_int resize_grip_n = 0; resize_grip_n < resize_grip_count; resize_grip_n++)
            {
                const ImGuiResizeGripDef& grip = resize_grip_def[resize_grip_n];
                const ImVec2 corner = ImLerp(window.Pos, window.Pos + window.Size, grip.CornerPosN);
                window.DrawList.PathLineTo(corner + grip.InnerDir * ((resize_grip_n & 1) ? ImVec2(window_border_size, resize_grip_draw_size) : ImVec2(resize_grip_draw_size, window_border_size)));
                window.DrawList.PathLineTo(corner + grip.InnerDir * ((resize_grip_n & 1) ? ImVec2(resize_grip_draw_size, window_border_size) : ImVec2(window_border_size, resize_grip_draw_size)));
                window.DrawList.PathArcToFast(ImVec2(corner.x + grip.InnerDir.x * (window_rounding + window_border_size), corner.y + grip.InnerDir.y * (window_rounding + window_border_size)), window_rounding, grip.AngleMin12, grip.AngleMax12);
                window.DrawList.PathFillConvex(resize_grip_col[resize_grip_n]);
            }
        }

        // Borders (for dock node host they will be rendered over after the tab bar)
        if (handle_borders_and_resize_grips && !window.DockNodeAsHost)
            RenderWindowOuterBorders(window);
    }
}

// Render title text, collapse button, close button
// When inside a dock node, this is handled in DockNodeCalcTabBarLayout() instead.
c_void ImGui::RenderWindowTitleBarContents(ImGuiWindow* window, const ImRect& title_bar_rect, *const char name, bool* p_open)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiStyle& style = g.Style;
    ImGuiWindowFlags flags = window.Flags;

    let has_close_button: bool = (p_open != NULL);
    let has_collapse_button: bool = !(flags & ImGuiWindowFlags_NoCollapse) && (style.WindowMenuButtonPosition != ImGuiDir_None);

    // Close & Collapse button are on the Menu NavLayer and don't default focus (unless there's nothing else on that layer)
    // FIXME-NAV: Might want (or not?) to set the equivalent of ImGuiButtonFlags_NoNavFocus so that mouse clicks on standard title bar items don't necessarily set nav/keyboard ref?
    const ImGuiItemFlags item_flags_backup = g.CurrentItemFlags;
    g.CurrentItemFlags |= ImGuiItemFlags_NoNavDefaultFocus;
    window.DC.NavLayerCurrent = ImGuiNavLayer_Menu;

    // Layout buttons
    // FIXME: Would be nice to generalize the subtleties expressed here into reusable code.
    c_float pad_l = style.FramePadding.x;
    c_float pad_r = style.FramePadding.x;
    c_float button_sz = g.FontSize;
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
        if (CollapseButton(window.GetID("#COLLAPSE"), collapse_button_pos, NULL))
            window.WantCollapseToggle = true; // Defer actual collapsing to next frame as we are too far in the Begin() function

    // Close button
    if (has_close_button)
        if (CloseButton(window.GetID("#CLOSE"), close_button_pos))
            *p_open = false;

    window.DC.NavLayerCurrent = ImGuiNavLayer_Main;
    g.CurrentItemFlags = item_flags_backup;

    // Title bar text (with: horizontal alignment, avoiding collapse/close button, optional "unsaved document" marker)
    // FIXME: Refactor text alignment facilities along with RenderText helpers, this is WAY too much messy code..
    const c_float marker_size_x = (flags & ImGuiWindowFlags_UnsavedDocument) ? button_sz * 0.80f32 : 0f32;
    const ImVec2 text_size = CalcTextSize(name, NULL, true) + ImVec2(marker_size_x, 0f32);

    // As a nice touch we try to ensure that centered title text doesn't get affected by visibility of Close/Collapse button,
    // while uncentered title text will still reach edges correctly.
    if (pad_l > style.FramePadding.x)
        pad_l += g.Style.ItemInnerSpacing.x;
    if (pad_r > style.FramePadding.x)
        pad_r += g.Style.ItemInnerSpacing.x;
    if (style.WindowTitleAlign.x > 0f32 && style.WindowTitleAlign.x < 1f32)
    {
        c_float centerness = ImSaturate(1f32 - ImFabs(style.WindowTitleAlign.x - 0.5f32) * 2.00f32); // 0f32 on either edges, 1f32 on center
        c_float pad_extend = ImMin(ImMax(pad_l, pad_r), title_bar_rect.GetWidth() - pad_l - pad_r - text_size.x);
        pad_l = ImMax(pad_l, pad_extend * centerness);
        pad_r = ImMax(pad_r, pad_extend * centerness);
    }

    ImRect layout_r(title_bar_rect.Min.x + pad_l, title_bar_rect.Min.y, title_bar_rect.Max.x - pad_r, title_bar_rect.Max.y);
    ImRect clip_r(layout_r.Min.x, layout_r.Min.y, ImMin(layout_r.Max.x + g.Style.ItemInnerSpacing.x, title_bar_rect.Max.x), layout_r.Max.y);
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
    RenderTextClipped(layout_r.Min, layout_r.Max, name, NULL, &text_size, style.WindowTitleAlign, &clip_r);
}

c_void ImGui::UpdateWindowParentAndRootLinks(ImGuiWindow* window, ImGuiWindowFlags flags, ImGuiWindow* parent_window)
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

// When a modal popup is open, newly created windows that want focus (i.e. are not popups and do not specify ImGuiWindowFlags_NoFocusOnAppearing)
// should be positioned behind that modal window, unless the window was created inside the modal begin-stack.
// In case of multiple stacked modals newly created window honors begin stack order and does not go below its own modal parent.
// - Window             // FindBlockingModal() returns Modal1
//   - Window           //                  .. returns Modal1
//   - Modal1           //                  .. returns Modal2
//      - Window        //                  .. returns Modal2
//          - Window    //                  .. returns Modal2
//          - Modal2    //                  .. returns Modal2
static ImGuiWindow* ImGui::FindBlockingModal(ImGuiWindow* window)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.OpenPopupStack.Size <= 0)
        return NULL;

    // Find a modal that has common parent with specified window. Specified window should be positioned behind that modal.
    for (c_int i = g.OpenPopupStack.Size - 1; i >= 0; i--)
    {
        ImGuiWindow* popup_window = g.OpenPopupStack.Data[i].Window;
        if (popup_window == NULL || !(popup_window.Flags & ImGuiWindowFlags_Modal))
            continue;
        if (!popup_window.Active && !popup_window.WasActive)      // Check WasActive, because this code may run before popup renders on current frame, also check Active to handle newly created windows.
            continue;
        if (IsWindowWithinBeginStackOf(window, popup_window))       // Window is rendered over last modal, no render order change needed.
            break;
        for (ImGuiWindow* parent = popup_window.ParentWindowInBeginStack->RootWindow; parent != None; parent = parent->ParentWindowInBeginStack->RootWindow)
            if (IsWindowWithinBeginStackOf(window, parent))
                return popup_window;                                // Place window above its begin stack parent.
    }
    return NULL;
}

// Push a new Dear ImGui window to add widgets to.
// - A default window called "Debug" is automatically stacked at the beginning of every frame so you can use widgets without explicitly calling a Begin/End pair.
// - Begin/End can be called multiple times during the frame with the same window name to append content.
// - The window name is used as a unique identifier to preserve window information across frames (and save rudimentary information to the .ini file).
//   You can use the "##" or "###" markers to use the same label with different id, or same id with different label. See documentation at the top of this file.
// - Return false when window is collapsed, so you can early out in your code. You always need to call ImGui::End() even if false is returned.
// - Passing 'bool* p_open' displays a Close button on the upper-right corner of the window, the pointed value will be set to false when the button is pressed.
bool ImGui::Begin(*const char name, bool* p_open, ImGuiWindowFlags flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    const ImGuiStyle& style = g.Style;
    // IM_ASSERT(name != NULL && name[0] != '\0');     // Window name required
    // IM_ASSERT(g.WithinFrameScope);                  // Forgot to call ImGui::NewFrame()
    // IM_ASSERT(g.FrameCountEnded != g.FrameCount);   // Called ImGui::Render() or ImGui::EndFrame() and haven't called ImGui::NewFrame() again yet

    // Find or create
    ImGuiWindow* window = FindWindowByName(name);
    let window_just_created: bool = (window == NULL);
    if (window_just_created)
        window = CreateNewWindow(name, flags);

    // Automatically disable manual moving/resizing when NoInputs is set
    if ((flags & ImGuiWindowFlags_NoInputs) == ImGuiWindowFlags_NoInputs)
        flags |= ImGuiWindowFlags_NoMove | ImGuiWindowFlags_NoResize;

    if (flags & ImGuiWindowFlags_NavFlattened)
        // IM_ASSERT(flags & ImGuiWindowFlags_ChildWindow);

    let current_frame: c_int = g.FrameCount;
    let first_begin_of_the_frame: bool = (window.LastFrameActive != current_frame);
    window.IsFallbackWindow = (g.CurrentWindowStack.Size == 0 && g.WithinFrameScopeWithImplicitWindow);

    // Update the Appearing flag (note: the BeginDocked() path may also set this to true later)
    let mut window_just_activated_by_user: bool =  (window.LastFrameActive < current_frame - 1); // Not using !WasActive because the implicit "Debug" window would always toggle off->on
    if (flags & ImGuiWindowFlags_Popup)
    {
        ImGuiPopupData& popup_ref = g.OpenPopupStack[g.BeginPopupStack.Size];
        window_just_activated_by_user |= (window.PopupId != popup_ref.PopupId); // We recycle popups so treat window as activated if popup id changed
        window_just_activated_by_user |= (window != popup_ref.Window);
    }

    // Update Flags, LastFrameActive, BeginOrderXXX fields
    let window_was_appearing: bool = window.Appearing;
    if (first_begin_of_the_frame)
    {
        UpdateWindowInFocusOrderList(window, window_just_created, flags);
        window.Appearing = window_just_activated_by_user;
        if (window.Appearing)
            SetWindowConditionAllowFlags(window, ImGuiCond_Appearing, true);
        window.FlagsPreviousFrame = window.Flags;
        window.Flags = (ImGuiWindowFlags)flags;
        window.LastFrameActive = current_frame;
        window.LastTimeActive = g.Time;
        window.BeginOrderWithinParent = 0;
        window.BeginOrderWithinContext = (c_short)(g.WindowsActiveCount++);
    }
    else
    {
        flags = window.Flags;
    }

    // Docking
    // (NB: during the frame dock nodes are created, it is possible that (window.DockIsActive == false) even though (window.DockNode->Windows.Size > 1)
    // IM_ASSERT(window.DockNode == NULL || window.DockNodeAsHost == NULL); // Cannot be both
    if (g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasDock)
        SetWindowDock(window, g.NextWindowData.DockId, g.NextWindowData.DockCond);
    if (first_begin_of_the_frame)
    {
        let mut has_dock_node: bool =  (window.DockId != 0 || window.DockNode != NULL);
        let mut new_auto_dock_node: bool =  !has_dock_node && GetWindowAlwaysWantOwnTabBar(window);
        let mut dock_node_was_visible: bool =  window.DockNodeIsVisible;
        let mut dock_tab_was_visible: bool =  window.DockTabIsVisible;
        if (has_dock_node || new_auto_dock_node)
        {
            BeginDocked(window, p_open);
            flags = window.Flags;
            if (window.DockIsActive)
            {
                // IM_ASSERT(window.DockNode != NULL);
                g.NextWindowData.Flags &= ~ImGuiNextWindowDataFlags_HasSizeConstraint; // Docking currently override constraints
            }

            // Amend the Appearing flag
            if (window.DockTabIsVisible && !dock_tab_was_visible && dock_node_was_visible && !window.Appearing && !window_was_appearing)
            {
                window.Appearing = true;
                SetWindowConditionAllowFlags(window, ImGuiCond_Appearing, true);
            }
        }
        else
        {
            window.DockIsActive = window.DockNodeIsVisible = window.DockTabIsVisible = false;
        }
    }

    // Parent window is latched only on the first call to Begin() of the frame, so further append-calls can be done from a different window stack
    ImGuiWindow* parent_window_in_stack = (window.DockIsActive && window.DockNode->HostWindow) ? window.DockNode->HostWindow : g.CurrentWindowStack.empty() ? NULL : g.CurrentWindowStack.back().Window;
    ImGuiWindow* parent_window = first_begin_of_the_frame ? ((flags & (ImGuiWindowFlags_ChildWindow | ImGuiWindowFlags_Popup)) ? parent_window_in_stack : NULL) : window.ParentWindow;
    // IM_ASSERT(parent_window != NULL || !(flags & ImGuiWindowFlags_ChildWindow));

    // We allow window memory to be compacted so recreate the base stack when needed.
    if (window.IDStack.Size == 0)
        window.IDStack.push(window.ID);

    // Add to stack
    // We intentionally set g.CurrentWindow to NULL to prevent usage until when the viewport is set, then will call SetCurrentWindow()
    g.CurrentWindow = window;
    ImGuiWindowStackData window_stack_data;
    window_stack_data.Window = window;
    window_stack_data.ParentLastItemDataBackup = g.LastItemData;
    window_stack_data.StackSizesOnBegin.SetToCurrentState();
    g.CurrentWindowStack.push(window_stack_data);
    g.CurrentWindow = None;
    if (flags & ImGuiWindowFlags_ChildMenu)
        g.BeginMenuCount+= 1;

    if (flags & ImGuiWindowFlags_Popup)
    {
        ImGuiPopupData& popup_ref = g.OpenPopupStack[g.BeginPopupStack.Size];
        popup_ref.Window = window;
        popup_ref.ParentNavLayer = parent_window_in_stack->DC.NavLayerCurrent;
        g.BeginPopupStack.push(popup_re0f32);
        window.PopupId = popup_ref.PopupId;
    }

    // Update ->RootWindow and others pointers (before any possible call to FocusWindow)
    if (first_begin_of_the_frame)
    {
        UpdateWindowParentAndRootLinks(window, flags, parent_window);
        window.ParentWindowInBeginStack = parent_window_in_stack;
    }

    // Process SetNextWindow***() calls
    // (FIXME: Consider splitting the HasXXX flags into X/Y components
    let mut window_pos_set_by_api: bool =  false;
    let mut window_size_x_set_by_api: bool =  false, window_size_y_set_by_api = false;
    if (g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasPos)
    {
        window_pos_set_by_api = (window.SetWindowPosAllowFlags & g.NextWindowData.PosCond) != 0;
        if (window_pos_set_by_api && ImLengthSqr(g.NextWindowData.PosPivotVal) > 0.000010f32)
        {
            // May be processed on the next frame if this is our first frame and we are measuring size
            // FIXME: Look into removing the branch so everything can go through this same code path for consistency.
            window.SetWindowPosVal = g.NextWindowData.PosVal;
            window.SetWindowPosPivot = g.NextWindowData.PosPivotVal;
            window.SetWindowPosAllowFlags &= ~(ImGuiCond_Once | ImGuiCond_FirstUseEver | ImGuiCond_Appearing);
        }
        else
        {
            SetWindowPos(window, g.NextWindowData.PosVal, g.NextWindowData.PosCond);
        }
    }
    if (g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasSize)
    {
        window_size_x_set_by_api = (window.SetWindowSizeAllowFlags & g.NextWindowData.SizeCond) != 0 && (g.NextWindowData.SizeVal.x > 0f32);
        window_size_y_set_by_api = (window.SetWindowSizeAllowFlags & g.NextWindowData.SizeCond) != 0 && (g.NextWindowData.SizeVal.y > 0f32);
        SetWindowSize(window, g.NextWindowData.SizeVal, g.NextWindowData.SizeCond);
    }
    if (g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasScroll)
    {
        if (g.NextWindowData.ScrollVal.x >= 0f32)
        {
            window.ScrollTarget.x = g.NextWindowData.ScrollVal.x;
            window.ScrollTargetCenterRatio.x = 0f32;
        }
        if (g.NextWindowData.ScrollVal.y >= 0f32)
        {
            window.ScrollTarget.y = g.NextWindowData.ScrollVal.y;
            window.ScrollTargetCenterRatio.y = 0f32;
        }
    }
    if (g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasContentSize)
        window.ContentSizeExplicit = g.NextWindowData.ContentSizeVal;
    else if (first_begin_of_the_frame)
        window.ContentSizeExplicit = ImVec2(0f32, 0f32);
    if (g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasWindowClass)
        window.WindowClass = g.NextWindowData.WindowClass;
    if (g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasCollapsed)
        SetWindowCollapsed(window, g.NextWindowData.CollapsedVal, g.NextWindowData.CollapsedCond);
    if (g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasFocus)
        FocusWindow(window);
    if (window.Appearing)
        SetWindowConditionAllowFlags(window, ImGuiCond_Appearing, false);

    // When reusing window again multiple times a frame, just append content (don't need to setup again)
    if (first_begin_of_the_frame)
    {
        // Initialize
        let window_is_child_tooltip: bool = (flags & ImGuiWindowFlags_ChildWindow) && (flags & ImGuiWindowFlags_Tooltip); // FIXME-WIP: Undocumented behavior of Child+Tooltip for pinned tooltip (#1345)
        let window_just_appearing_after_hidden_for_resize: bool = (window.HiddenFramesCannotSkipItems > 0);
        window.Active = true;
        window.HasCloseButton = (p_open != NULL);
        window.ClipRect = ImVec4(-f32::MAX, -f32::MAX, +f32::MAX, +f32::MAX);
        window.IDStack.resize(1);
        window.DrawList._ResetForNewFrame();
        window.DC.CurrentTableIdx = -1;
        if (flags & ImGuiWindowFlags_DockNodeHost)
        {
            window.DrawList.ChannelsSplit(2);
            window.DrawList.ChannelsSetCurrent(DOCKING_HOST_DRAW_CHANNEL_FG); // Render decorations on channel 1 as we will render the backgrounds manually later
        }

        // Restore buffer capacity when woken from a compacted state, to avoid
        if (window.MemoryCompacted)
            GcAwakeTransientWindowBuffers(window);

        // Update stored window name when it changes (which can _only_ happen with the "###" operator, so the ID would stay unchanged).
        // The title bar always display the 'name' parameter, so we only update the string storage if it needs to be visible to the end-user elsewhere.
        let mut window_title_visible_elsewhere: bool =  false;
        if ((window.Viewport && window.Viewport->Window == window) || (window.DockIsActive))
            window_title_visible_elsewhere = true;
        else if (g.NavWindowingListWindow != NULL && (window.Flags & ImGuiWindowFlags_NoNavFocus) == 0)   // Window titles visible when using CTRL+TAB
            window_title_visible_elsewhere = true;
        if (window_title_visible_elsewhere && !window_just_created && strcmp(name, window.Name) != 0)
        {
            size_t buf_len = window.NameBufLen;
            window.Name = ImStrdupcpy(window.Name, &buf_len, name);
            window.NameBufLen = buf_len;
        }

        // UPDATE CONTENTS SIZE, UPDATE HIDDEN STATUS

        // Update contents size from last frame for auto-fitting (or use explicit size)
        CalcWindowContentSizes(window, &window.ContentSize, &window.ContentSizeIdeal);

        // FIXME: These flags are decremented before they are used. This means that in order to have these fields produce their intended behaviors
        // for one frame we must set them to at least 2, which is counter-intuitive. HiddenFramesCannotSkipItems is a more complicated case because
        // it has a single usage before this code block and may be set below before it is finally checked.
        if (window.HiddenFramesCanSkipItems > 0)
            window.HiddenFramesCanSkipItems-= 1;
        if (window.HiddenFramesCannotSkipItems > 0)
            window.HiddenFramesCannotSkipItems-= 1;
        if (window.HiddenFramesForRenderOnly > 0)
            window.HiddenFramesForRenderOnly-= 1;

        // Hide new windows for one frame until they calculate their size
        if (window_just_created && (!window_size_x_set_by_api || !window_size_y_set_by_api))
            window.HiddenFramesCannotSkipItems = 1;

        // Hide popup/tooltip window when re-opening while we measure size (because we recycle the windows)
        // We reset Size/ContentSize for reappearing popups/tooltips early in this function, so further code won't be tempted to use the old size.
        if (window_just_activated_by_user && (flags & (ImGuiWindowFlags_Popup | ImGuiWindowFlags_Tooltip)) != 0)
        {
            window.HiddenFramesCannotSkipItems = 1;
            if (flags & ImGuiWindowFlags_AlwaysAutoResize)
            {
                if (!window_size_x_set_by_api)
                    window.Size.x = window.SizeFull.x = 0.f;
                if (!window_size_y_set_by_api)
                    window.Size.y = window.SizeFull.y = 0.f;
                window.ContentSize = window.ContentSizeIdeal = ImVec2(0.f, 0f32);
            }
        }

        // SELECT VIEWPORT
        // We need to do this before using any style/font sizes, as viewport with a different DPI may affect font sizes.

        WindowSelectViewport(window);
        SetCurrentViewport(window, window.Viewport);
        window.FontDpiScale = (g.IO.ConfigFlags & ImGuiConfigFlags_DpiEnableScaleFonts) ? window.Viewport->DpiScale : 1f32;
        SetCurrentWindow(window);
        flags = window.Flags;

        // LOCK BORDER SIZE AND PADDING FOR THE FRAME (so that altering them doesn't cause inconsistencies)
        // We read Style data after the call to UpdateSelectWindowViewport() which might be swapping the style.

        if (flags & ImGuiWindowFlags_ChildWindow)
            window.WindowBorderSize = style.ChildBorderSize;
        else
            window.WindowBorderSize = ((flags & (ImGuiWindowFlags_Popup | ImGuiWindowFlags_Tooltip)) && !(flags & ImGuiWindowFlags_Modal)) ? style.PopupBorderSize : style.WindowBorderSize;
        if (!window.DockIsActive && (flags & ImGuiWindowFlags_ChildWindow) && !(flags & (ImGuiWindowFlags_AlwaysUseWindowPadding | ImGuiWindowFlags_Popup)) && window.WindowBorderSize == 0f32)
            window.WindowPadding = ImVec2(0f32, (flags & ImGuiWindowFlags_MenuBar) ? style.WindowPadding.y : 0f32);
        else
            window.WindowPadding = style.WindowPadding;

        // Lock menu offset so size calculation can use it as menu-bar windows need a minimum size.
        window.DC.MenuBarOffset.x = ImMax(ImMax(window.WindowPadding.x, style.ItemSpacing.x), g.NextWindowData.MenuBarOffsetMinVal.x);
        window.DC.MenuBarOffset.y = g.NextWindowData.MenuBarOffsetMinVal.y;

        // Collapse window by double-clicking on title bar
        // At this point we don't have a clipping rectangle setup yet, so we can use the title bar area for hit detection and drawing
        if (!(flags & ImGuiWindowFlags_NoTitleBar) && !(flags & ImGuiWindowFlags_NoCollapse) && !window.DockIsActive)
        {
            // We don't use a regular button+id to test for double-click on title bar (mostly due to legacy reason, could be fixed), so verify that we don't have items over the title bar.
            ImRect title_bar_rect = window.TitleBarRect();
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
        const ImVec2 size_auto_fit = CalcWindowAutoFitSize(window, window.ContentSizeIdeal);
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
        const c_float decoration_up_height = window.TitleBarHeight() + window.MenuBarHeight();

        // POSITION

        // Popup latch its initial position, will position itself when it appears next frame
        if (window_just_activated_by_user)
        {
            window.AutoPosLastDirection = ImGuiDir_None;
            if ((flags & ImGuiWindowFlags_Popup) != 0 && !(flags & ImGuiWindowFlags_Modal) && !window_pos_set_by_api) // FIXME: BeginPopup() could use SetNextWindowPos()
                window.Pos = g.BeginPopupStack.back().OpenPopupPos;
        }

        // Position child window
        if (flags & ImGuiWindowFlags_ChildWindow)
        {
            // IM_ASSERT(parent_window && parent_window.Active);
            window.BeginOrderWithinParent = (c_short)parent_window.DC.ChildWindows.Size;
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
        if (window.ViewportAllowPlatformMonitorExtend >= 0 && !window.ViewportOwned && !(window.Viewport->Flags & ImGuiViewportFlags_Minimized))
            if (!window.Viewport->GetMainRect().Contains(window.Rect()))
            {
                // This is based on the assumption that the DPI will be known ahead (same as the DPI of the selection done in UpdateSelectWindowViewport)
                //ImGuiViewport* old_viewport = window.Viewport;
                window.Viewport = AddUpdateViewport(window, window.ID, window.Pos, window.Size, ImGuiViewportFlags_NoFocusOnAppearing);

                // FIXME-DPI
                //IM_ASSERT(old_viewport->DpiScale == window.Viewport->DpiScale); // FIXME-DPI: Something went wrong
                SetCurrentViewport(window, window.Viewport);
                window.FontDpiScale = (g.IO.ConfigFlags & ImGuiConfigFlags_DpiEnableScaleFonts) ? window.Viewport->DpiScale : 1f32;
                SetCurrentWindow(window);
            }

        if (window.ViewportOwned)
            WindowSyncOwnedViewport(window, parent_window_in_stack);

        // Calculate the range of allowed position for that window (to be movable and visible past safe area padding)
        // When clamping to stay visible, we will enforce that window.Pos stays inside of visibility_rect.
        ImRect viewport_rect(window.Viewport->GetMainRect());
        ImRect viewport_work_rect(window.Viewport->GetWorkRect());
        ImVec2 visibility_padding = ImMax(style.DisplayWindowPadding, style.DisplaySafeAreaPadding);
        ImRect visibility_rect(viewport_work_rect.Min + visibility_padding, viewport_work_rect.Max - visibility_padding);

        // Clamp position/size so window stays visible within its viewport or monitor
        // Ignore zero-sized display explicitly to avoid losing positions if a window manager reports zero-sized window when initializing or minimizing.
        // FIXME: Similar to code in GetWindowAllowedExtentRect()
        if (!window_pos_set_by_api && !(flags & ImGuiWindowFlags_ChildWindow) && window.AutoFitFramesX <= 0 && window.AutoFitFramesY <= 0)
        {
            if (!window.ViewportOwned && viewport_rect.GetWidth() > 0 && viewport_rect.GetHeight() > 0f32)
            {
                ClampWindowRect(window, visibility_rect);
            }
            else if (window.ViewportOwned && g.PlatformIO.Monitors.Size > 0)
            {
                // Lost windows (e.g. a monitor disconnected) will naturally moved to the fallback/dummy monitor aka the main viewport.
                *const ImGuiPlatformMonitor monitor = GetViewportPlatformMonitor(window.Viewport);
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

            ImGuiWindow* modal = GetTopMostPopupModal();
            if (modal != NULL && !IsWindowWithinBeginStackOf(window, modal))
            {
                // Avoid focusing a window that is created outside of active modal. This will prevent active modal from being closed.
                // Since window is not focused it would reappear at the same display position like the last time it was visible.
                // In case of completely new windows it would go to the top (over current modal), but input to such window would still be blocked by modal.
                // Position window behind a modal that is not a begin-parent of this window.
                want_focus = false;
                if (window == window.RootWindow)
                {
                    ImGuiWindow* blocking_modal = FindBlockingModal(window);
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
        c_int border_held = -1;
        u32 resize_grip_col[4] = {};
        let resize_grip_count: c_int = g.IO.ConfigWindowsResizeFromEdges ? 2 : 1; // Allow resize from lower-left if we have the mouse cursor feedback for it.
        const c_float resize_grip_draw_size = IM_FLOOR(ImMax(g.FontSize * 1.10f32, window.WindowRounding + 1f32 + g.FontSize * 0.20f32));
        if (handle_borders_and_resize_grips && !window.Collapsed)
            if (UpdateWindowManualResize(window, size_auto_fit, &border_held, resize_grip_count, &resize_grip_col[0], visibility_rect))
                use_current_size_for_scrollbar_x = use_current_size_for_scrollbar_y = true;
        window.ResizeBorderHeld = border_held;

        // Synchronize window --> viewport again and one last time (clamping and manual resize may have affected either)
        if (window.ViewportOwned)
        {
            if (!window.Viewport->PlatformRequestMove)
                window.Viewport->Pos = window.Pos;
            if (!window.Viewport->PlatformRequestResize)
                window.Viewport->Size = window.Size;
            window.Viewport->UpdateWorkRect();
            viewport_rect = window.Viewport->GetMainRect();
        }

        // Save last known viewport position within the window itself (so it can be saved in .ini file and restored)
        window.ViewportPos = window.Viewport->Pos;

        // SCROLLBAR VISIBILITY

        // Update scrollbar visibility (based on the Size that was effective during last frame or the auto-resized Size).
        if (!window.Collapsed)
        {
            // When reading the current size we need to read it after size constraints have been applied.
            // When we use InnerRect here we are intentionally reading last frame size, same for ScrollbarSizes values before we set them again.
            ImVec2 avail_size_from_current_frame = ImVec2(window.SizeFull.x, window.SizeFull.y - decoration_up_height);
            ImVec2 avail_size_from_last_frame = window.InnerRect.GetSize() + window.ScrollbarSizes;
            ImVec2 needed_size_from_last_frame = window_just_created ? ImVec2(0, 0) : window.ContentSize + window.WindowPadding * 2.0f32;
            c_float size_x_for_scrollbars = use_current_size_for_scrollbar_x ? avail_size_from_current_frame.x : avail_size_from_last_frame.x;
            c_float size_y_for_scrollbars = use_current_size_for_scrollbar_y ? avail_size_from_current_frame.y : avail_size_from_last_frame.y;
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
        const ImRect host_rect = ((flags & ImGuiWindowFlags_ChildWindow) && !(flags & ImGuiWindowFlags_Popup) && !window_is_child_tooltip) ? parent_window.ClipRect : viewport_rect;
        const ImRect outer_rect = window.Rect();
        const ImRect title_bar_rect = window.TitleBarRect();
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
        c_float top_border_size = (((flags & ImGuiWindowFlags_MenuBar) || !(flags & ImGuiWindowFlags_NoTitleBar)) ? style.FrameBorderSize : window.WindowBorderSize);
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
        window.DrawList.PushTextureID(g.Font->ContainerAtlas->TexID);
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
                ImGuiWindow* previous_child = parent_window.DC.ChildWindows.Size >= 2 ? parent_window.DC.ChildWindows[parent_window.DC.ChildWindows.Size - 2] : NULL;
                let mut previous_child_overlapping: bool =  previous_child ? previous_child->Rect().Overlaps(window.Rect()) : false;
                let mut parent_is_empty: bool =  parent_window.DrawList.VtxBuffer.Size > 0;
                if (window.DrawList.CmdBuffer.back().ElemCount == 0 && parent_is_empty && !previous_child_overlapping)
                    render_decorations_in_parent = true;
            }
            if (render_decorations_in_parent)
                window.DrawList = parent_window.DrawList;

            // Handle title bar, scrollbar, resize grips and resize borders
            *const ImGuiWindow window_to_highlight = g.NavWindowingTarget ? g.NavWindowingTarget : g.NavWindow;
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
        const c_float work_rect_size_x = (window.ContentSizeExplicit.x != 0f32 ? window.ContentSizeExplicit.x : ImMax(allow_scrollbar_x ? window.ContentSize.x : 0f32, window.Size.x - window.WindowPadding.x * 2.0f32 - window.ScrollbarSizes.x));
        const c_float work_rect_size_y = (window.ContentSizeExplicit.y != 0f32 ? window.ContentSizeExplicit.y : ImMax(allow_scrollbar_y ? window.ContentSize.y : 0f32, window.Size.y - window.WindowPadding.y * 2.0f32 - decoration_up_height - window.ScrollbarSizes.y));
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
        window.DC.ChildWindows.resize(0);
        window.DC.StateStorage = &window.StateStorage;
        window.DC.CurrentColumns = None;
        window.DC.LayoutType = ImGuiLayoutType_Vertical;
        window.DC.ParentLayoutType = parent_window ? parent_window.DC.LayoutType : ImGuiLayoutType_Vertical;

        window.DC.ItemWidth = window.ItemWidthDefault;
        window.DC.TextWrapPos = -1f32; // disabled
        window.DC.ItemWidthStack.resize(0);
        window.DC.TextWrapPosStack.resize(0);

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
        if (p_open != NULL && window.Viewport->PlatformRequestClose && window.Viewport != GetMainViewport())
        {
            if (!window.DockIsActive || window.DockTabIsVisible)
            {
                window.Viewport->PlatformRequestClose = false;
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
                if ((window.RootWindowDockTree->Flags & ImGuiWindowFlags_NoDocking) == 0)
                    BeginDockableDragDropSource(window);

            // Docking: Any dockable window can act as a target. For dock node hosts we call BeginDockableDragDropTarget() in DockNodeUpdate() instead.
            if (g.DragDropActive && !(flags & ImGuiWindowFlags_NoDocking))
                if (g.MovingWindow == NULL || g.Movingwindow.RootWindowDockTree != window)
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
                let nav_request: bool = (flags & ImGuiWindowFlags_NavFlattened) && (g.NavAnyRequest && g.NavWindow && g.Navwindow.RootWindowForNav == window.RootWindowForNav);
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

c_void ImGui::End()
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
        if (ImGuiWindow* host_window = window.DockNode->HostWindow)         // FIXME-DOCK
            host_window.DC.CursorMaxPos = window.DC.CursorMaxPos + window.WindowPadding - host_window.WindowPadding;

    // Pop from window stack
    g.LastItemData = g.CurrentWindowStack.back().ParentLastItemDataBackup;
    if (window.Flags & ImGuiWindowFlags_ChildMenu)
        g.BeginMenuCount-= 1;
    if (window.Flags & ImGuiWindowFlags_Popup)
        g.BeginPopupStack.pop_back();
    g.CurrentWindowStack.back().StackSizesOnBegin.CompareWithCurrentState();
    g.CurrentWindowStack.pop_back();
    SetCurrentWindow(g.CurrentWindowStack.Size == 0 ? NULL : g.CurrentWindowStack.back().Window);
    if (g.CurrentWindow)
        SetCurrentViewport(g.CurrentWindow, g.Currentwindow.Viewport);
}

c_void ImGui::BringWindowToFocusFront(ImGuiWindow* window)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(window == window.RootWindow);

    let cur_order: c_int = window.FocusOrder;
    // IM_ASSERT(g.WindowsFocusOrder[cur_order] == window);
    if (g.WindowsFocusOrder.back() == window)
        return;

    let new_order: c_int = g.WindowsFocusOrder.Size - 1;
    for (c_int n = cur_order; n < new_order; n++)
    {
        g.WindowsFocusOrder[n] = g.WindowsFocusOrder[n + 1];
        g.WindowsFocusOrder[n]->FocusOrder-= 1;
        // IM_ASSERT(g.WindowsFocusOrder[n]->FocusOrder == n);
    }
    g.WindowsFocusOrder[new_order] = window;
    window.FocusOrder = (c_short)new_order;
}

c_void ImGui::BringWindowToDisplayFront(ImGuiWindow* window)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiWindow* current_front_window = g.Windows.back();
    if (current_front_window == window || current_front_window.RootWindowDockTree == window) // Cheap early out (could be better)
        return;
    for (c_int i = g.Windows.Size - 2; i >= 0; i--) // We can ignore the top-most window
        if (g.Windows[i] == window)
        {
            memmove(&g.Windows[i], &g.Windows[i + 1], (g.Windows.Size - i - 1) * sizeof(ImGuiWindow*));
            g.Windows[g.Windows.Size - 1] = window;
            break;
        }
}

c_void ImGui::BringWindowToDisplayBack(ImGuiWindow* window)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.Windows[0] == window)
        return;
    for (c_int i = 0; i < g.Windows.Size; i++)
        if (g.Windows[i] == window)
        {
            memmove(&g.Windows[1], &g.Windows[0], i * sizeof(ImGuiWindow*));
            g.Windows[0] = window;
            break;
        }
}

c_void ImGui::BringWindowToDisplayBehind(ImGuiWindow* window, ImGuiWindow* behind_window)
{
    // IM_ASSERT(window != NULL && behind_window != NULL);
    let g = GImGui; // ImGuiContext& g = *GImGui;
    window = window.RootWindow;
    behind_window = behind_window.RootWindow;
    c_int pos_wnd = FindWindowDisplayIndex(window);
    c_int pos_beh = FindWindowDisplayIndex(behind_window);
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

c_int ImGui::FindWindowDisplayIndex(ImGuiWindow* window)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.Windows.index_from_ptr(g.Windows.find(window));
}

// Moving window to front of display and set focus (which happens to be back of our sorted list)
c_void ImGui::FocusWindow(ImGuiWindow* window)
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
    ImGuiWindow* focus_front_window = window ? window.RootWindow : NULL;
    ImGuiWindow* display_front_window = window ? window.RootWindowDockTree : NULL;
    ImGuiDockNode* dock_node = window ? window.DockNode : NULL;
    let mut active_id_window_is_dock_node_host: bool =  (g.ActiveIdWindow && dock_node && dock_node->HostWindow == g.ActiveIdWindow);

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
    if (dock_node && dock_node->TabBar)
        dock_node->TabBar->SelectedTabId = dock_node->TabBar->NextSelectedTabId = window.TabId;

    // Bring to front
    BringWindowToFocusFront(focus_front_window);
    if (((window.Flags | focus_front_window.Flags | display_front_window.Flags) & ImGuiWindowFlags_NoBringToFrontOnFocus) == 0)
        BringWindowToDisplayFront(display_front_window);
}

c_void ImGui::FocusTopMostWindowUnderOne(ImGuiWindow* under_this_window, ImGuiWindow* ignore_window)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    c_int start_idx = g.WindowsFocusOrder.Size - 1;
    if (under_this_window != NULL)
    {
        // Aim at root window behind us, if we are in a child window that's our own root (see #4640)
        c_int offset = -1;
        while (under_this_window.Flags & ImGuiWindowFlags_ChildWindow)
        {
            under_this_window = under_this_window.ParentWindow;
            offset = 0;
        }
        start_idx = FindWindowFocusIndex(under_this_window) + offset;
    }
    for (c_int i = start_idx; i >= 0; i--)
    {
        // We may later decide to test for different NoXXXInputs based on the active navigation input (mouse vs nav) but that may feel more confusing to the user.
        ImGuiWindow* window = g.WindowsFocusOrder[i];
        // IM_ASSERT(window == window.RootWindow);
        if (window != ignore_window && window.WasActive)
            if ((window.Flags & (ImGuiWindowFlags_NoMouseInputs | ImGuiWindowFlags_NoNavInputs)) != (ImGuiWindowFlags_NoMouseInputs | ImGuiWindowFlags_NoNavInputs))
            {
                // FIXME-DOCK: This is failing (lagging by one frame) for docked windows.
                // If A and B are docked into window and B disappear, at the NewFrame() call site window.NavLastChildNavWindow will still point to B.
                // We might leverage the tab order implicitly stored in window.DockNodeAsHost->TabBar (essentially the 'most_recently_selected_tab' code in tab bar will do that but on next update)
                // to tell which is the "previous" window. Or we may leverage 'LastFrameFocused/LastFrameJustFocused' and have this function handle child window itself?
                ImGuiWindow* focus_window = NavRestoreLastChildNavWindow(window);
                FocusWindow(focus_window);
                return;
            }
    }
    FocusWindow(NULL);
}

// Important: this alone doesn't alter current ImDrawList state. This is called by PushFont/PopFont only.
c_void ImGui::SetCurrentFont(ImFont* font)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(font && font->IsLoaded());    // Font Atlas not created. Did you call io.Fonts->GetTexDataAsRGBA32 / GetTexDataAsAlpha8 ?
    // IM_ASSERT(font->Scale > 0f32);
    g.Font = font;
    g.FontBaseSize = ImMax(1f32, g.IO.FontGlobalScale * g.Font->FontSize * g.Font->Scale);
    g.FontSize = g.CurrentWindow ? g.Currentwindow.CalcFontSize() : 0f32;

    ImFontAtlas* atlas = g.Font->ContainerAtlas;
    g.DrawListSharedData.TexUvWhitePixel = atlas->TexUvWhitePixel;
    g.DrawListSharedData.TexUvLines = atlas->TexUvLines;
    g.DrawListSharedData.Font = g.Font;
    g.DrawListSharedData.FontSize = g.FontSize;
}

c_void ImGui::PushFont(ImFont* font)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (!font)
        font = GetDefaultFont();
    SetCurrentFont(font);
    g.FontStack.push(font);
    g.Currentwindow.DrawList.PushTextureID(font->ContainerAtlas->TexID);
}

c_void  ImGui::PopFont()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.Currentwindow.DrawList.PopTextureID();
    g.FontStack.pop_back();
    SetCurrentFont(g.FontStack.empty() ? GetDefaultFont() : g.FontStack.back());
}

c_void ImGui::PushItemFlag(ImGuiItemFlags option, bool enabled)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiItemFlags item_flags = g.CurrentItemFlags;
    // IM_ASSERT(item_flags == g.ItemFlagsStack.back());
    if (enabled)
        item_flags |= option;
    else
        item_flags &= ~option;
    g.CurrentItemFlags = item_flags;
    g.ItemFlagsStack.push(item_flags);
}

c_void ImGui::PopItemFlag()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.ItemFlagsStack.Size > 1); // Too many calls to PopItemFlag() - we always leave a 0 at the bottom of the stack.
    g.ItemFlagsStack.pop_back();
    g.CurrentItemFlags = g.ItemFlagsStack.back();
}

// BeginDisabled()/EndDisabled()
// - Those can be nested but it cannot be used to enable an already disabled section (a single BeginDisabled(true) in the stack is enough to keep everything disabled)
// - Visually this is currently altering alpha, but it is expected that in a future styling system this would work differently.
// - Feedback welcome at https://github.com/ocornut/imgui/issues/211
// - BeginDisabled(false) essentially does nothing useful but is provided to facilitate use of boolean expressions. If you can avoid calling BeginDisabled(False)/EndDisabled() best to avoid it.
// - Optimized shortcuts instead of PushStyleVar() + PushItemFlag()
c_void ImGui::BeginDisabled(bool disabled)
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

c_void ImGui::EndDisabled()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.DisabledStackSize > 0);
    g.DisabledStackSize-= 1;
    let mut was_disabled: bool =  (g.CurrentItemFlags & ImGuiItemFlags_Disabled) != 0;
    //PopItemFlag();
    g.ItemFlagsStack.pop_back();
    g.CurrentItemFlags = g.ItemFlagsStack.back();
    if (was_disabled && (g.CurrentItemFlags & ImGuiItemFlags_Disabled) == 0)
        g.Style.Alpha = g.DisabledAlphaBackup; //PopStyleVar();
}

// FIXME: Look into renaming this once we have settled the new Focus/Activation/TabStop system.
c_void ImGui::PushAllowKeyboardFocus(bool allow_keyboard_focus)
{
    PushItemFlag(ImGuiItemFlags_NoTabStop, !allow_keyboard_focus);
}

c_void ImGui::PopAllowKeyboardFocus()
{
    PopItemFlag();
}

c_void ImGui::PushButtonRepeat(bool repeat)
{
    PushItemFlag(ImGuiItemFlags_ButtonRepeat, repeat);
}

c_void ImGui::PopButtonRepeat()
{
    PopItemFlag();
}

c_void ImGui::PushTextWrapPos(c_float wrap_pos_x)
{
    ImGuiWindow* window = GetCurrentWindow();
    window.DC.TextWrapPosStack.push(window.DC.TextWrapPos);
    window.DC.TextWrapPos = wrap_pos_x;
}

c_void ImGui::PopTextWrapPos()
{
    ImGuiWindow* window = GetCurrentWindow();
    window.DC.TextWrapPos = window.DC.TextWrapPosStack.back();
    window.DC.TextWrapPosStack.pop_back();
}

static ImGuiWindow* GetCombinedRootWindow(ImGuiWindow* window, bool popup_hierarchy, bool dock_hierarchy)
{
    ImGuiWindow* last_window = None;
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

bool ImGui::IsWindowChildOf(ImGuiWindow* window, ImGuiWindow* potential_parent, bool popup_hierarchy, bool dock_hierarchy)
{
    ImGuiWindow* window_root = GetCombinedRootWindow(window, popup_hierarchy, dock_hierarchy);
    if (window_root == potential_parent)
        return true;
    while (window != NULL)
    {
        if (window == potential_parent)
            return true;
        if (window == window_root) // end of chain
            return false;
        window = window.ParentWindow;
    }
    return false;
}

bool ImGui::IsWindowWithinBeginStackOf(ImGuiWindow* window, ImGuiWindow* potential_parent)
{
    if (window.RootWindow == potential_parent)
        return true;
    while (window != NULL)
    {
        if (window == potential_parent)
            return true;
        window = window.ParentWindowInBeginStack;
    }
    return false;
}

bool ImGui::IsWindowAbove(ImGuiWindow* potential_above, ImGuiWindow* potential_below)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    // It would be saner to ensure that display layer is always reflected in the g.Windows[] order, which would likely requires altering all manipulations of that array
    let display_layer_delta: c_int = GetWindowDisplayLayer(potential_above) - GetWindowDisplayLayer(potential_below);
    if (display_layer_delta != 0)
        return display_layer_delta > 0;

    for (c_int i = g.Windows.Size - 1; i >= 0; i--)
    {
        ImGuiWindow* candidate_window = g.Windows[i];
        if (candidate_window == potential_above)
            return true;
        if (candidate_window == potential_below)
            return false;
    }
    return false;
}

bool ImGui::IsWindowHovered(ImGuiHoveredFlags flags)
{
    // IM_ASSERT((flags & (ImGuiHoveredFlags_AllowWhenOverlapped | ImGuiHoveredFlags_AllowWhenDisabled)) == 0);   // Flags not supported by this function
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiWindow* ref_window = g.HoveredWindow;
    ImGuiWindow* cur_window = g.CurrentWindow;
    if (ref_window == NULL)
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

bool ImGui::IsWindowFocused(ImGuiFocusedFlags flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiWindow* ref_window = g.NavWindow;
    ImGuiWindow* cur_window = g.CurrentWindow;

    if (ref_window == NULL)
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

ImGuiID ImGui::GetWindowDockID()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.Currentwindow.DockId;
}

bool ImGui::IsWindowDocked()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.Currentwindow.DockIsActive;
}

// Can we focus this window with CTRL+TAB (or PadMenu + PadFocusPrev/PadFocusNext)
// Note that NoNavFocus makes the window not reachable with CTRL+TAB but it can still be focused with mouse or programmatically.
// If you want a window to never be focused, you may use the e.g. NoInputs flag.
bool ImGui::IsWindowNavFocusable(ImGuiWindow* window)
{
    return window.WasActive && window == window.RootWindow && !(window.Flags & ImGuiWindowFlags_NoNavFocus);
}

c_float ImGui::GetWindowWidth()
{
    ImGuiWindow* window = GimGui.CurrentWindow;
    return window.Size.x;
}

c_float ImGui::GetWindowHeight()
{
    ImGuiWindow* window = GimGui.CurrentWindow;
    return window.Size.y;
}

ImVec2 ImGui::GetWindowPos()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    return window.Pos;
}

c_void ImGui::SetWindowPos(ImGuiWindow* window, const ImVec2& pos, ImGuiCond cond)
{
    // Test condition (NB: bit 0 is always true) and clear flags for next time
    if (cond && (window.SetWindowPosAllowFlags & cond) == 0)
        return;

    // IM_ASSERT(cond == 0 || ImIsPowerOfTwo(cond)); // Make sure the user doesn't attempt to combine multiple condition flags.
    window.SetWindowPosAllowFlags &= ~(ImGuiCond_Once | ImGuiCond_FirstUseEver | ImGuiCond_Appearing);
    window.SetWindowPosVal = ImVec2(f32::MAX, f32::MAX);

    // Set
    const ImVec2 old_pos = window.Pos;
    window.Pos = ImFloor(pos);
    ImVec2 offset = window.Pos - old_pos;
    if (offset.x == 0f32 && offset.y == 0f32)
        return;
    MarkIniSettingsDirty(window);
    // FIXME: share code with TranslateWindow(), need to confirm whether the 3 rect modified by TranslateWindow() are desirable here.
    window.DC.CursorPos += offset;         // As we happen to move the window while it is being appended to (which is a bad idea - will smear) let's at least offset the cursor
    window.DC.CursorMaxPos += offset;      // And more importantly we need to offset CursorMaxPos/CursorStartPos this so ContentSize calculation doesn't get affected.
    window.DC.IdealMaxPos += offset;
    window.DC.CursorStartPos += offset;
}

c_void ImGui::SetWindowPos(const ImVec2& pos, ImGuiCond cond)
{
    ImGuiWindow* window = GetCurrentWindowRead();
    SetWindowPos(window, pos, cond);
}

c_void ImGui::SetWindowPos(*const char name, const ImVec2& pos, ImGuiCond cond)
{
    if (ImGuiWindow* window = FindWindowByName(name))
        SetWindowPos(window, pos, cond);
}

ImVec2 ImGui::GetWindowSize()
{
    ImGuiWindow* window = GetCurrentWindowRead();
    return window.Size;
}

c_void ImGui::SetWindowSize(ImGuiWindow* window, const ImVec2& size, ImGuiCond cond)
{
    // Test condition (NB: bit 0 is always true) and clear flags for next time
    if (cond && (window.SetWindowSizeAllowFlags & cond) == 0)
        return;

    // IM_ASSERT(cond == 0 || ImIsPowerOfTwo(cond)); // Make sure the user doesn't attempt to combine multiple condition flags.
    window.SetWindowSizeAllowFlags &= ~(ImGuiCond_Once | ImGuiCond_FirstUseEver | ImGuiCond_Appearing);

    // Set
    ImVec2 old_size = window.SizeFull;
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

c_void ImGui::SetWindowSize(const ImVec2& size, ImGuiCond cond)
{
    SetWindowSize(GimGui.CurrentWindow, size, cond);
}

c_void ImGui::SetWindowSize(*const char name, const ImVec2& size, ImGuiCond cond)
{
    if (ImGuiWindow* window = FindWindowByName(name))
        SetWindowSize(window, size, cond);
}

c_void ImGui::SetWindowCollapsed(ImGuiWindow* window, bool collapsed, ImGuiCond cond)
{
    // Test condition (NB: bit 0 is always true) and clear flags for next time
    if (cond && (window.SetWindowCollapsedAllowFlags & cond) == 0)
        return;
    window.SetWindowCollapsedAllowFlags &= ~(ImGuiCond_Once | ImGuiCond_FirstUseEver | ImGuiCond_Appearing);

    // Set
    window.Collapsed = collapsed;
}

c_void ImGui::SetWindowHitTestHole(ImGuiWindow* window, const ImVec2& pos, const ImVec2& size)
{
    // IM_ASSERT(window.HitTestHoleSize.x == 0);     // We don't support multiple holes/hit test filters
    window.HitTestHoleSize = ImVec2ih(size);
    window.HitTestHoleOffset = ImVec2ih(pos - window.Pos);
}

c_void ImGui::SetWindowCollapsed(bool collapsed, ImGuiCond cond)
{
    SetWindowCollapsed(GimGui.CurrentWindow, collapsed, cond);
}

bool ImGui::IsWindowCollapsed()
{
    ImGuiWindow* window = GetCurrentWindowRead();
    return window.Collapsed;
}

bool ImGui::IsWindowAppearing()
{
    ImGuiWindow* window = GetCurrentWindowRead();
    return window.Appearing;
}

c_void ImGui::SetWindowCollapsed(*const char name, bool collapsed, ImGuiCond cond)
{
    if (ImGuiWindow* window = FindWindowByName(name))
        SetWindowCollapsed(window, collapsed, cond);
}

c_void ImGui::SetWindowFocus()
{
    FocusWindow(GimGui.CurrentWindow);
}

c_void ImGui::SetWindowFocus(*const char name)
{
    if (name)
    {
        if (ImGuiWindow* window = FindWindowByName(name))
            FocusWindow(window);
    }
    else
    {
        FocusWindow(NULL);
    }
}

c_void ImGui::SetNextWindowPos(const ImVec2& pos, ImGuiCond cond, const ImVec2& pivot)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(cond == 0 || ImIsPowerOfTwo(cond)); // Make sure the user doesn't attempt to combine multiple condition flags.
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasPos;
    g.NextWindowData.PosVal = pos;
    g.NextWindowData.PosPivotVal = pivot;
    g.NextWindowData.PosCond = cond ? cond : ImGuiCond_Always;
    g.NextWindowData.PosUndock = true;
}

c_void ImGui::SetNextWindowSize(const ImVec2& size, ImGuiCond cond)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(cond == 0 || ImIsPowerOfTwo(cond)); // Make sure the user doesn't attempt to combine multiple condition flags.
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasSize;
    g.NextWindowData.SizeVal = size;
    g.NextWindowData.SizeCond = cond ? cond : ImGuiCond_Always;
}

c_void ImGui::SetNextWindowSizeConstraints(const ImVec2& size_min, const ImVec2& size_max, ImGuiSizeCallback custom_callback, custom_callback_user_data: *mut c_void)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasSizeConstraint;
    g.NextWindowData.SizeConstraintRect = ImRect(size_min, size_max);
    g.NextWindowData.SizeCallback = custom_callback;
    g.NextWindowData.SizeCallbackUserData = custom_callback_user_data;
}

// Content size = inner scrollable rectangle, padded with WindowPadding.
// SetNextWindowContentSize(ImVec2(100,100) + ImGuiWindowFlags_AlwaysAutoResize will always allow submitting a 100x100 item.
c_void ImGui::SetNextWindowContentSize(const ImVec2& size)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasContentSize;
    g.NextWindowData.ContentSizeVal = ImFloor(size);
}

c_void ImGui::SetNextWindowScroll(const ImVec2& scroll)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasScroll;
    g.NextWindowData.ScrollVal = scroll;
}

c_void ImGui::SetNextWindowCollapsed(bool collapsed, ImGuiCond cond)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(cond == 0 || ImIsPowerOfTwo(cond)); // Make sure the user doesn't attempt to combine multiple condition flags.
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasCollapsed;
    g.NextWindowData.CollapsedVal = collapsed;
    g.NextWindowData.CollapsedCond = cond ? cond : ImGuiCond_Always;
}

c_void ImGui::SetNextWindowFocus()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasFocus;
}

c_void ImGui::SetNextWindowBgAlpha(c_float alpha)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasBgAlpha;
    g.NextWindowData.BgAlphaVal = alpha;
}

c_void ImGui::SetNextWindowViewport(ImGuiID id)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasViewport;
    g.NextWindowData.ViewportId = id;
}

c_void ImGui::SetNextWindowDockID(ImGuiID id, ImGuiCond cond)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasDock;
    g.NextWindowData.DockCond = cond ? cond : ImGuiCond_Always;
    g.NextWindowData.DockId = id;
}

c_void ImGui::SetNextWindowClass(*const ImGuiWindowClass window_class)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT((window_class->ViewportFlagsOverrideSet & window_class->ViewportFlagsOverrideClear) == 0); // Cannot set both set and clear for the same bit
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasWindowClass;
    g.NextWindowData.WindowClass = *window_class;
}

ImDrawList* ImGui::GetWindowDrawList()
{
    ImGuiWindow* window = GetCurrentWindow();
    return window.DrawList;
}

c_float ImGui::GetWindowDpiScale()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.CurrentDpiScale;
}

ImGuiViewport* ImGui::GetWindowViewport()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.CurrentViewport != NULL && g.CurrentViewport == g.Currentwindow.Viewport);
    return g.CurrentViewport;
}

ImFont* ImGui::GetFont()
{
    return GimGui.Font;
}

c_float ImGui::GetFontSize()
{
    return GimGui.FontSize;
}

ImVec2 ImGui::GetFontTexUvWhitePixel()
{
    return GimGui.DrawListSharedData.TexUvWhitePixel;
}

c_void ImGui::SetWindowFontScale(c_float scale)
{
    // IM_ASSERT(scale > 0f32);
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiWindow* window = GetCurrentWindow();
    window.FontWindowScale = scale;
    g.FontSize = g.DrawListSharedData.FontSize = window.CalcFontSize();
}

c_void ImGui::ActivateItem(ImGuiID id)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NavNextActivateId = id;
    g.NavNextActivateFlags = ImGuiActivateFlags_None;
}

c_void ImGui::PushFocusScope(ImGuiID id)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    g.FocusScopeStack.push(window.DC.NavFocusScopeIdCurrent);
    window.DC.NavFocusScopeIdCurrent = id;
}

c_void ImGui::PopFocusScope()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    // IM_ASSERT(g.FocusScopeStack.Size > 0); // Too many PopFocusScope() ?
    window.DC.NavFocusScopeIdCurrent = g.FocusScopeStack.back();
    g.FocusScopeStack.pop_back();
}

// Note: this will likely be called ActivateItem() once we rework our Focus/Activation system!
c_void ImGui::SetKeyboardFocusHere(c_int offset)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    // IM_ASSERT(offset >= -1);    // -1 is allowed but not below
    IMGUI_DEBUG_LOG_ACTIVEID("SetKeyboardFocusHere(%d) in window \"%s\"\n", offset, window.Name);

    // It makes sense in the vast majority of cases to never interrupt a drag and drop.
    // When we refactor this function into ActivateItem() we may want to make this an option.
    // MovingWindow is protected from most user inputs using SetActiveIdUsingNavAndKeys(), but
    // is also automatically dropped in the event g.ActiveId is stolen.
    if (g.DragDropActive || g.MovingWindow != NULL)
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

c_void ImGui::SetItemDefaultFocus()
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

c_void ImGui::SetStateStorage(ImGuiStorage* tree)
{
    ImGuiWindow* window = GimGui.CurrentWindow;
    window.DC.StateStorage = tree ? tree : &window.StateStorage;
}

ImGuiStorage* ImGui::GetStateStorage()
{
    ImGuiWindow* window = GimGui.CurrentWindow;
    return window.DC.StateStorage;
}

c_void ImGui::PushID(*const char str_id)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    ImGuiID id = window.GetID(str_id);
    window.IDStack.push(id);
}

c_void ImGui::PushID(*const char str_id_begin, *const char str_id_end)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    ImGuiID id = window.GetID(str_id_begin, str_id_end);
    window.IDStack.push(id);
}

c_void ImGui::PushID(*const c_void ptr_id)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    ImGuiID id = window.GetID(ptr_id);
    window.IDStack.push(id);
}

c_void ImGui::PushID(c_int int_id)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    ImGuiID id = window.GetID(int_id);
    window.IDStack.push(id);
}

// Push a given id value ignoring the ID stack as a seed.
c_void ImGui::PushOverrideID(ImGuiID id)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if (g.DebugHookIdInfo == id)
        DebugHookIdInfo(id, ImGuiDataType_ID, NULL, NULL);
    window.IDStack.push(id);
}

// Helper to avoid a common series of PushOverrideID -> GetID() -> PopID() call
// (note that when using this pattern, TestEngine's "Stack Tool" will tend to not display the intermediate stack level.
//  for that to work we would need to do PushOverrideID() -> ItemAdd() -> PopID() which would alter widget code a little more)
ImGuiID ImGui::GetIDWithSeed(*const char str, *const char str_end, ImGuiID seed)
{
    ImGuiID id = ImHashStr(str, str_end ? (str_end - str) : 0, seed);
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.DebugHookIdInfo == id)
        DebugHookIdInfo(id, ImGuiDataType_String, str, str_end);
    return id;
}

c_void ImGui::PopID()
{
    ImGuiWindow* window = GimGui.CurrentWindow;
    // IM_ASSERT(window.IDStack.Size > 1); // Too many PopID(), or could be popping in a wrong/different window?
    window.IDStack.pop_back();
}

ImGuiID ImGui::GetID(*const char str_id)
{
    ImGuiWindow* window = GimGui.CurrentWindow;
    return window.GetID(str_id);
}

ImGuiID ImGui::GetID(*const char str_id_begin, *const char str_id_end)
{
    ImGuiWindow* window = GimGui.CurrentWindow;
    return window.GetID(str_id_begin, str_id_end);
}

ImGuiID ImGui::GetID(*const c_void ptr_id)
{
    ImGuiWindow* window = GimGui.CurrentWindow;
    return window.GetID(ptr_id);
}

bool ImGui::IsRectVisible(const ImVec2& size)
{
    ImGuiWindow* window = GimGui.CurrentWindow;
    return window.ClipRect.Overlaps(ImRect(window.DC.CursorPos, window.DC.CursorPos + size));
}

bool ImGui::IsRectVisible(const ImVec2& rect_min, const ImVec2& rect_max)
{
    ImGuiWindow* window = GimGui.CurrentWindow;
    return window.ClipRect.Overlaps(ImRect(rect_min, rect_max));
}


//-----------------------------------------------------------------------------
// [SECTION] INPUTS
//-----------------------------------------------------------------------------

// Test if mouse cursor is hovering given rectangle
// NB- Rectangle is clipped by our current clip setting
// NB- Expand the rectangle to be generous on imprecise inputs systems (g.Style.TouchExtraPadding)
bool ImGui::IsMouseHoveringRect(const ImVec2& r_min, const ImVec2& r_max, bool clip)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    // Clip
    ImRect rect_clipped(r_min, r_max);
    if (clip)
        rect_clipped.ClipWith(g.Currentwindow.ClipRect);

    // Expand for touch input
    const ImRect rect_for_touch(rect_clipped.Min - g.Style.TouchExtraPadding, rect_clipped.Max + g.Style.TouchExtraPadding);
    if (!rect_for_touch.Contains(g.IO.MousePos))
        return false;
    if (!g.MouseViewport->GetMainRect().Overlaps(rect_clipped))
        return false;
    return true;
}

ImGuiKeyData* ImGui::GetKeyData(ImGuiKey key)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    c_int index;
// #ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
    // IM_ASSERT(key >= ImGuiKey_LegacyNativeKey_BEGIN && key < ImGuiKey_NamedKey_END);
    if (IsLegacyKey(key))
        index = (g.IO.KeyMap[key] != -1) ? g.IO.KeyMap[key] : key; // Remap native->imgui or imgui->native
    else
        index = key;
// #else
    // IM_ASSERT(IsNamedKey(key) && "Support for user key indices was dropped in favor of ImGuiKey. Please update backend & user code.");
    index = key - ImGuiKey_NamedKey_BEGIN;
// #endif
    return &g.IO.KeysData[index];
}

// #ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
c_int ImGui::GetKeyIndex(ImGuiKey key)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(IsNamedKey(key));
    *const ImGuiKeyData key_data = GetKeyData(key);
    return (key_data - g.IO.KeysData);
}
// #endif

// Those names a provided for debugging purpose and are not meant to be saved persistently not compared.
static *const char const GKeyNames[] =
{
    "Tab", "LeftArrow", "RightArrow", "UpArrow", "DownArrow", "PageUp", "PageDown",
    "Home", "End", "Insert", "Delete", "Backspace", "Space", "Enter", "Escape",
    "LeftCtrl", "LeftShift", "LeftAlt", "LeftSuper", "RightCtrl", "RightShift", "RightAlt", "RightSuper", "Menu",
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F", "G", "H",
    "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
    "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10", "F11", "F12",
    "Apostrophe", "Comma", "Minus", "Period", "Slash", "Semicolon", "Equal", "LeftBracket",
    "Backslash", "RightBracket", "GraveAccent", "CapsLock", "ScrollLock", "NumLock", "PrintScreen",
    "Pause", "Keypad0", "Keypad1", "Keypad2", "Keypad3", "Keypad4", "Keypad5", "Keypad6",
    "Keypad7", "Keypad8", "Keypad9", "KeypadDecimal", "KeypadDivide", "KeypadMultiply",
    "KeypadSubtract", "KeypadAdd", "KeypadEnter", "KeypadEqual",
    "GamepadStart", "GamepadBack",
    "GamepadFaceLeft", "GamepadFaceRight", "GamepadFaceUp", "GamepadFaceDown",
    "GamepadDpadLeft", "GamepadDpadRight", "GamepadDpadUp", "GamepadDpadDown",
    "GamepadL1", "GamepadR1", "GamepadL2", "GamepadR2", "GamepadL3", "GamepadR3",
    "GamepadLStickLeft", "GamepadLStickRight", "GamepadLStickUp", "GamepadLStickDown",
    "GamepadRStickLeft", "GamepadRStickRight", "GamepadRStickUp", "GamepadRStickDown",
    "ModCtrl", "ModShift", "ModAlt", "ModSuper",
    "MouseLeft", "MouseRight", "MouseMiddle", "MouseX1", "MouseX2", "MouseWheelX", "MouseWheelY",
};
IM_STATIC_ASSERT(ImGuiKey_NamedKey_COUNT == IM_ARRAYSIZE(GKeyNames));

*const char ImGui::GetKeyName(ImGuiKey key)
{
// #ifdef IMGUI_DISABLE_OBSOLETE_KEYIO
    // IM_ASSERT((IsNamedKey(key) || key == ImGuiKey_None) && "Support for user key indices was dropped in favor of ImGuiKey. Please update backend and user code.");
// #else
    if (IsLegacyKey(key))
    {
        ImGuiIO& io = GetIO();
        if (io.KeyMap[key] == -1)
            return "N/A";
        // IM_ASSERT(IsNamedKey((ImGuiKey)io.KeyMap[key]));
        key = (ImGuiKey)io.KeyMap[key];
    }
// #endif
    if (key == ImGuiKey_None)
        return "None";
    if (!IsNamedKey(key))
        return "Unknown";

    return GKeyNames[key - ImGuiKey_NamedKey_BEGIN];
}

c_void ImGui::GetKeyChordName(ImGuiModFlags mods, ImGuiKey key, char* out_buf, c_int out_buf_size)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT((mods & ~ImGuiModFlags_All) == 0 && "Passing invalid ImGuiModFlags value!"); // A frequent mistake is to pass ImGuiKey_ModXXX instead of ImGuiModFlags_XXX
    ImFormatString(out_buf, out_buf_size, "%s%s%s%s%s",
        (mods & ImGuiModFlags_Ctrl) ? "Ctrl+" : "",
        (mods & ImGuiModFlags_Shift) ? "Shift+" : "",
        (mods & ImGuiModFlags_Alt) ? "Alt+" : "",
        (mods & ImGuiModFlags_Super) ? (g.IO.ConfigMacOSXBehaviors ? "Cmd+" : "Super+") : "",
        GetKeyName(key));
}

// t0 = previous time (e.g.: g.Time - g.IO.DeltaTime)
// t1 = current time (e.g.: g.Time)
// An event is triggered at:
//  t = 0f32     t = repeat_delay,    t = repeat_delay + repeat_rate*N
c_int ImGui::CalcTypematicRepeatAmount(c_float t0, c_float t1, c_float repeat_delay, c_float repeat_rate)
{
    if (t1 == 0f32)
        return 1;
    if (t0 >= t1)
        return 0;
    if (repeat_rate <= 0f32)
        return (t0 < repeat_delay) && (t1 >= repeat_delay);
    let count_t0: c_int = (t0 < repeat_delay) ? -1 : ((t0 - repeat_delay) / repeat_rate);
    let count_t1: c_int = (t1 < repeat_delay) ? -1 : ((t1 - repeat_delay) / repeat_rate);
    let count: c_int = count_t1 - count_t0;
    return count;
}

c_void ImGui::GetTypematicRepeatRate(ImGuiInputFlags flags, c_float* repeat_delay, c_float* repeat_rate)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    switch (flags & ImGuiInputFlags_RepeatRateMask_)
    {
    case ImGuiInputFlags_RepeatRateNavMove:             *repeat_delay = g.IO.KeyRepeatDelay * 0.72f; *repeat_rate = g.IO.KeyRepeatRate * 0.80f32; return;
    case ImGuiInputFlags_RepeatRateNavTweak:            *repeat_delay = g.IO.KeyRepeatDelay * 0.72f; *repeat_rate = g.IO.KeyRepeatRate * 0.3f32; return;
    case ImGuiInputFlags_RepeatRateDefault: default:    *repeat_delay = g.IO.KeyRepeatDelay * 1f32; *repeat_rate = g.IO.KeyRepeatRate * 1f32; return;
    }
}

// Return value representing the number of presses in the last time period, for the given repeat rate
// (most often returns 0 or 1. The result is generally only >1 when RepeatRate is smaller than DeltaTime, aka large DeltaTime or fast RepeatRate)
c_int ImGui::GetKeyPressedAmount(ImGuiKey key, c_float repeat_delay, c_float repeat_rate)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    *const ImGuiKeyData key_data = GetKeyData(key);
    if (!key_data->Down) // In theory this should already be encoded as (DownDuration < 0f32), but testing this facilitate eating mechanism (until we finish work on input ownership)
        return 0;
    const c_float t = key_data->DownDuration;
    return CalcTypematicRepeatAmount(t - g.IO.DeltaTime, t, repeat_delay, repeat_rate);
}

// Return 2D vector representing the combination of four cardinal direction, with analog value support (for e.g. ImGuiKey_GamepadLStick* values).
ImVec2 ImGui::GetKeyVector2d(ImGuiKey key_left, ImGuiKey key_right, ImGuiKey key_up, ImGuiKey key_down)
{
    return ImVec2(
        GetKeyData(key_right)->AnalogValue - GetKeyData(key_left)->AnalogValue,
        GetKeyData(key_down)->AnalogValue - GetKeyData(key_up)->AnalogValue);
}

// Note that Dear ImGui doesn't know the meaning/semantic of ImGuiKey from 0..511: they are legacy native keycodes.
// Consider transitioning from 'IsKeyDown(MY_ENGINE_KEY_A)' (<1.87) to IsKeyDown(ImGuiKey_A) (>= 1.87)
bool ImGui::IsKeyDown(ImGuiKey key)
{
    *const ImGuiKeyData key_data = GetKeyData(key);
    if (!key_data->Down)
        return false;
    return true;
}

bool ImGui::IsKeyPressed(ImGuiKey key, bool repeat)
{
    return IsKeyPressedEx(key, repeat ? ImGuiInputFlags_Repeat : ImGuiInputFlags_None);
}

// Important: unlike legacy IsKeyPressed(ImGuiKey, bool repeat=true) which DEFAULT to repeat, this requires EXPLICIT repeat.
// [Internal] 2022/07: Do not call this directly! It is a temporary entry point which we will soon replace with an overload for IsKeyPressed() when we introduce key ownership.
bool ImGui::IsKeyPressedEx(ImGuiKey key, ImGuiInputFlags flags)
{
    *const ImGuiKeyData key_data = GetKeyData(key);
    if (!key_data->Down) // In theory this should already be encoded as (DownDuration < 0f32), but testing this facilitate eating mechanism (until we finish work on input ownership)
        return false;
    const c_float t = key_data->DownDuration;
    if (t < 0f32)
        return false;

    let mut pressed: bool =  (t == 0f32);
    if (!pressed && ((flags & ImGuiInputFlags_Repeat) != 0))
    {
        c_float repeat_delay, repeat_rate;
        GetTypematicRepeatRate(flags, &repeat_delay, &repeat_rate);
        pressed = (t > repeat_delay) && GetKeyPressedAmount(key, repeat_delay, repeat_rate) > 0;
    }

    if (!pressed)
        return false;
    return true;
}

bool ImGui::IsKeyReleased(ImGuiKey key)
{
    *const ImGuiKeyData key_data = GetKeyData(key);
    if (key_data->DownDurationPrev < 0f32 || key_data->Down)
        return false;
    return true;
}

bool ImGui::IsMouseDown(ImGuiMouseButton button)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(button >= 0 && button < IM_ARRAYSIZE(g.IO.MouseDown));
    return g.IO.MouseDown[button];
}

bool ImGui::IsMouseClicked(ImGuiMouseButton button, bool repeat)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(button >= 0 && button < IM_ARRAYSIZE(g.IO.MouseDown));
    if (!g.IO.MouseDown[button]) // In theory this should already be encoded as (DownDuration < 0f32), but testing this facilitate eating mechanism (until we finish work on input ownership)
        return false;
    const c_float t = g.IO.MouseDownDuration[button];
    if (t == 0f32)
        return true;
    if (repeat && t > g.IO.KeyRepeatDelay)
        return CalcTypematicRepeatAmount(t - g.IO.DeltaTime, t, g.IO.KeyRepeatDelay, g.IO.KeyRepeatRate) > 0;
    return false;
}

bool ImGui::IsMouseReleased(ImGuiMouseButton button)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(button >= 0 && button < IM_ARRAYSIZE(g.IO.MouseDown));
    return g.IO.MouseReleased[button];
}

bool ImGui::IsMouseDoubleClicked(ImGuiMouseButton button)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(button >= 0 && button < IM_ARRAYSIZE(g.IO.MouseDown));
    return g.IO.MouseClickedCount[button] == 2;
}

c_int ImGui::GetMouseClickedCount(ImGuiMouseButton button)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(button >= 0 && button < IM_ARRAYSIZE(g.IO.MouseDown));
    return g.IO.MouseClickedCount[button];
}

// Return if a mouse click/drag went past the given threshold. Valid to call during the MouseReleased frame.
// [Internal] This doesn't test if the button is pressed
bool ImGui::IsMouseDragPastThreshold(ImGuiMouseButton button, c_float lock_threshold)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(button >= 0 && button < IM_ARRAYSIZE(g.IO.MouseDown));
    if (lock_threshold < 0f32)
        lock_threshold = g.IO.MouseDragThreshold;
    return g.IO.MouseDragMaxDistanceSqr[button] >= lock_threshold * lock_threshold;
}

bool ImGui::IsMouseDragging(ImGuiMouseButton button, c_float lock_threshold)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(button >= 0 && button < IM_ARRAYSIZE(g.IO.MouseDown));
    if (!g.IO.MouseDown[button])
        return false;
    return IsMouseDragPastThreshold(button, lock_threshold);
}

ImVec2 ImGui::GetMousePos()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.IO.MousePos;
}

// NB: prefer to call right after BeginPopup(). At the time Selectable/MenuItem is activated, the popup is already closed!
ImVec2 ImGui::GetMousePosOnOpeningCurrentPopup()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.BeginPopupStack.Size > 0)
        return g.OpenPopupStack[g.BeginPopupStack.Size - 1].OpenMousePos;
    return g.IO.MousePos;
}

// We typically use ImVec2(-f32::MAX,-f32::MAX) to denote an invalid mouse position.
bool ImGui::IsMousePosValid(*const ImVec2 mouse_pos)
{
    // The assert is only to silence a false-positive in XCode Static Analysis.
    // Because GImGui is not dereferenced in every code path, the static analyzer assume that it may be NULL (which it doesn't for other functions).
    // IM_ASSERT(GImGui != NULL);
    const c_float MOUSE_INVALID = -256000f32;
    ImVec2 p = mouse_pos ? *mouse_pos : GimGui.IO.MousePos;
    return p.x >= MOUSE_INVALID && p.y >= MOUSE_INVALID;
}

// [WILL OBSOLETE] This was designed for backends, but prefer having backend maintain a mask of held mouse buttons, because upcoming input queue system will make this invalid.
bool ImGui::IsAnyMouseDown()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    for (c_int n = 0; n < IM_ARRAYSIZE(g.IO.MouseDown); n++)
        if (g.IO.MouseDown[n])
            return true;
    return false;
}

// Return the delta from the initial clicking position while the mouse button is clicked or was just released.
// This is locked and return 0f32 until the mouse moves past a distance threshold at least once.
// NB: This is only valid if IsMousePosValid(). backends in theory should always keep mouse position valid when dragging even outside the client window.
ImVec2 ImGui::GetMouseDragDelta(ImGuiMouseButton button, c_float lock_threshold)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(button >= 0 && button < IM_ARRAYSIZE(g.IO.MouseDown));
    if (lock_threshold < 0f32)
        lock_threshold = g.IO.MouseDragThreshold;
    if (g.IO.MouseDown[button] || g.IO.MouseReleased[button])
        if (g.IO.MouseDragMaxDistanceSqr[button] >= lock_threshold * lock_threshold)
            if (IsMousePosValid(&g.IO.MousePos) && IsMousePosValid(&g.IO.MouseClickedPos[button]))
                return g.IO.MousePos - g.IO.MouseClickedPos[button];
    return ImVec2(0f32, 0f32);
}

c_void ImGui::ResetMouseDragDelta(ImGuiMouseButton button)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(button >= 0 && button < IM_ARRAYSIZE(g.IO.MouseDown));
    // NB: We don't need to reset g.IO.MouseDragMaxDistanceSqr
    g.IO.MouseClickedPos[button] = g.IO.MousePos;
}

ImGuiMouseCursor ImGui::GetMouseCursor()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.MouseCursor;
}

c_void ImGui::SetMouseCursor(ImGuiMouseCursor cursor_type)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.MouseCursor = cursor_type;
}

c_void ImGui::SetNextFrameWantCaptureKeyboard(bool want_capture_keyboard)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.WantCaptureKeyboardNextFrame = want_capture_keyboard ? 1 : 0;
}

c_void ImGui::SetNextFrameWantCaptureMouse(bool want_capture_mouse)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.WantCaptureMouseNextFrame = want_capture_mouse ? 1 : 0;
}

// #ifndef IMGUI_DISABLE_DEBUG_TOOLS
static *const char GetInputSourceName(ImGuiInputSource source)
{
    *const char input_source_names[] = { "None", "Mouse", "Keyboard", "Gamepad", "Nav", "Clipboard" };
    // IM_ASSERT(IM_ARRAYSIZE(input_source_names) == ImGuiInputSource_COUNT && source >= 0 && source < ImGuiInputSource_COUNT);
    return input_source_names[source];
}
static c_void DebugPrintInputEvent(*const char prefix, *const ImGuiInputEvent e)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (e->Type == ImGuiInputEventType_MousePos)    { IMGUI_DEBUG_LOG_IO("%s: MousePos (%.1f, %.10f32)\n", prefix, e->MousePos.PosX, e->MousePos.PosY); return; }
    if (e->Type == ImGuiInputEventType_MouseButton) { IMGUI_DEBUG_LOG_IO("%s: MouseButton %d %s\n", prefix, e->MouseButton.Button, e->MouseButton.Down ? "Down" : "Up"); return; }
    if (e->Type == ImGuiInputEventType_MouseWheel)  { IMGUI_DEBUG_LOG_IO("%s: MouseWheel (%.1f, %.10f32)\n", prefix, e->MouseWheel.WheelX, e->MouseWheel.WheelY); return; }
    if (e->Type == ImGuiInputEventType_Key)         { IMGUI_DEBUG_LOG_IO("%s: Key \"%s\" %s\n", prefix, ImGui::GetKeyName(e->Key.Key), e->Key.Down ? "Down" : "Up"); return; }
    if (e->Type == ImGuiInputEventType_Text)        { IMGUI_DEBUG_LOG_IO("%s: Text: %c (U+%08X)\n", prefix, e->Text.Char, e->Text.Char); return; }
    if (e->Type == ImGuiInputEventType_Focus)       { IMGUI_DEBUG_LOG_IO("%s: AppFocused %d\n", prefix, e->AppFocused.Focused); return; }
}
// #endif

// Process input queue
// We always call this with the value of 'bool g.IO.ConfigInputTrickleEventQueue'.
// - trickle_fast_inputs = false : process all events, turn into flattened input state (e.g. successive down/up/down/up will be lost)
// - trickle_fast_inputs = true  : process as many events as possible (successive down/up/down/up will be trickled over several frames so nothing is lost) (new feature in 1.87)
c_void ImGui::UpdateInputEvents(bool trickle_fast_inputs)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiIO& io = g.IO;

    // Only trickle chars<>key when working with InputText()
    // FIXME: InputText() could parse event trail?
    // FIXME: Could specialize chars<>keys trickling rules for control keys (those not typically associated to characters)
    let trickle_interleaved_keys_and_text: bool = (trickle_fast_inputs && g.WantTextInputNextFrame == 1);

    let mut mouse_moved: bool =  false, mouse_wheeled = false, key_changed = false, text_inputted = false;
    c_int  mouse_button_changed = 0x00;
    ImBitArray<ImGuiKey_KeysData_SIZE> key_changed_mask;

    c_int event_n = 0;
    for (; event_n < g.InputEventsQueue.Size; event_n++)
    {
        ImGuiInputEvent* e = &g.InputEventsQueue[event_n];
        if (e->Type == ImGuiInputEventType_MousePos)
        {
            ImVec2 event_pos(e->MousePos.PosX, e->MousePos.PosY);
            if (IsMousePosValid(&event_pos))
                event_pos = ImVec2(ImFloorSigned(event_pos.x), ImFloorSigned(event_pos.y)); // Apply same flooring as UpdateMouseInputs()
            e->IgnoredAsSame = (io.MousePos.x == event_pos.x && io.MousePos.y == event_pos.y);
            if (!e->IgnoredAsSame)
            {
                // Trickling Rule: Stop processing queued events if we already handled a mouse button change
                if (trickle_fast_inputs && (mouse_button_changed != 0 || mouse_wheeled || key_changed || text_inputted))
                    break;
                io.MousePos = event_pos;
                mouse_moved = true;
            }
        }
        else if (e->Type == ImGuiInputEventType_MouseButton)
        {
            const ImGuiMouseButton button = e->MouseButton.Button;
            // IM_ASSERT(button >= 0 && button < ImGuiMouseButton_COUNT);
            e->IgnoredAsSame = (io.MouseDown[button] == e->MouseButton.Down);
            if (!e->IgnoredAsSame)
            {
                // Trickling Rule: Stop processing queued events if we got multiple action on the same button
                if (trickle_fast_inputs && ((mouse_button_changed & (1 << button)) || mouse_wheeled))
                    break;
                io.MouseDown[button] = e->MouseButton.Down;
                mouse_button_changed |= (1 << button);
            }
        }
        else if (e->Type == ImGuiInputEventType_MouseWheel)
        {
            e->IgnoredAsSame = (e->MouseWheel.WheelX == 0f32 && e->MouseWheel.WheelY == 0f32);
            if (!e->IgnoredAsSame)
            {
                // Trickling Rule: Stop processing queued events if we got multiple action on the event
                if (trickle_fast_inputs && (mouse_moved || mouse_button_changed != 0))
                    break;
                io.MouseWheelH += e->MouseWheel.WheelX;
                io.MouseWheel += e->MouseWheel.WheelY;
                mouse_wheeled = true;
            }
        }
        else if (e->Type == ImGuiInputEventType_MouseViewport)
        {
            io.MouseHoveredViewport = e->MouseViewport.HoveredViewportID;
        }
        else if (e->Type == ImGuiInputEventType_Key)
        {
            ImGuiKey key = e->Key.Key;
            // IM_ASSERT(key != ImGuiKey_None);
            let keydata_index: c_int = (key - ImGuiKey_KeysData_OFFSET);
            ImGuiKeyData* keydata = &io.KeysData[keydata_index];
            e->IgnoredAsSame = (keydata->Down == e->Key.Down && keydata->AnalogValue == e->Key.AnalogValue);
            if (!e->IgnoredAsSame)
            {
                // Trickling Rule: Stop processing queued events if we got multiple action on the same button
                if (trickle_fast_inputs && keydata->Down != e->Key.Down && (key_changed_mask.TestBit(keydata_index) || text_inputted || mouse_button_changed != 0))
                    break;
                keydata->Down = e->Key.Down;
                keydata->AnalogValue = e->Key.AnalogValue;
                key_changed = true;
                key_changed_mask.SetBit(keydata_index);

                if (key == ImGuiKey_ModCtrl || key == ImGuiKey_ModShift || key == ImGuiKey_ModAlt || key == ImGuiKey_ModSuper)
                {
                    if (key == ImGuiKey_ModCtrl) { io.KeyCtrl = keydata->Down; }
                    if (key == ImGuiKey_ModShift) { io.KeyShift = keydata->Down; }
                    if (key == ImGuiKey_ModAlt) { io.KeyAlt = keydata->Down; }
                    if (key == ImGuiKey_ModSuper) { io.KeySuper = keydata->Down; }
                    io.KeyMods = GetMergedModFlags();
                }

                // Allow legacy code using io.KeysDown[GetKeyIndex()] with new backends
// #ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
                io.KeysDown[key] = keydata->Down;
                if (io.KeyMap[key] != -1)
                    io.KeysDown[io.KeyMap[key]] = keydata->Down;
// #endif
            }
        }
        else if (e->Type == ImGuiInputEventType_Text)
        {
            // Trickling Rule: Stop processing queued events if keys/mouse have been interacted with
            if (trickle_fast_inputs && ((key_changed && trickle_interleaved_keys_and_text) || mouse_button_changed != 0 || mouse_moved || mouse_wheeled))
                break;
            c_uint c = e->Text.Char;
            io.InputQueueCharacters.push(c <= IM_UNICODE_CODEPOINT_MAX ? c : IM_UNICODE_CODEPOINT_INVALID);
            if (trickle_interleaved_keys_and_text)
                text_inputted = true;
        }
        else if (e->Type == ImGuiInputEventType_Focus)
        {
            // We intentionally overwrite this and process lower, in order to give a chance
            // to multi-viewports backends to queue AddFocusEvent(false) + AddFocusEvent(true) in same frame.
            let focus_lost: bool = !e->AppFocused.Focused;
            e->IgnoredAsSame = (io.AppFocusLost == focus_lost);
            if (!e->IgnoredAsSame)
                io.AppFocusLost = focus_lost;
        }
        else
        {
            // IM_ASSERT(0 && "Unknown event!");
        }
    }

    // Record trail (for domain-specific applications wanting to access a precise trail)
    //if (event_n != 0) IMGUI_DEBUG_LOG_IO("Processed: %d / Remaining: %d\n", event_n, g.InputEventsQueue.Size - event_n);
    for (c_int n = 0; n < event_n; n++)
        g.InputEventsTrail.push(g.InputEventsQueue[n]);

    // [DEBUG]
// #ifndef IMGUI_DISABLE_DEBUG_TOOLS
    if (event_n != 0 && (g.DebugLogFlags & ImGuiDebugLogFlags_EventIO))
        for (c_int n = 0; n < g.InputEventsQueue.Size; n++)
            DebugPrintInputEvent(n < event_n ? (g.InputEventsQueue[n].IgnoredAsSame ? "Processed (Same)" : "Processed") : "Remaining", &g.InputEventsQueue[n]);
// #endif

    // Remaining events will be processed on the next frame
    if (event_n == g.InputEventsQueue.Size)
        g.InputEventsQueue.resize(0);
    else
        g.InputEventsQueue.erase(g.InputEventsQueue.Data, g.InputEventsQueue.Data + event_n);

    // Clear buttons state when focus is lost
    // (this is useful so e.g. releasing Alt after focus loss on Alt-Tab doesn't trigger the Alt menu toggle)
    if (g.IO.AppFocusLost)
    {
        g.IO.ClearInputKeys();
        g.IO.AppFocusLost = false;
    }
}


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
bool ImGui::DebugCheckVersionAndDataLayout(*const char version, size_t sz_io, size_t sz_style, size_t sz_vec2, size_t sz_vec4, size_t sz_vert, size_t sz_idx)
{
    let mut error: bool =  false;
    if (strcmp(version, IMGUI_VERSION) != 0) { error = true; IM_ASSERT(strcmp(version, IMGUI_VERSION) == 0 && "Mismatched version string!"); }
    if (sz_io != sizeof(ImGuiIO)) { error = true; IM_ASSERT(sz_io == sizeof(ImGuiIO) && "Mismatched struct layout!"); }
    if (sz_style != sizeof(ImGuiStyle)) { error = true; IM_ASSERT(sz_style == sizeof(ImGuiStyle) && "Mismatched struct layout!"); }
    if (sz_vec2 != sizeof(ImVec2)) { error = true; IM_ASSERT(sz_vec2 == sizeof(ImVec2) && "Mismatched struct layout!"); }
    if (sz_vec4 != sizeof(ImVec4)) { error = true; IM_ASSERT(sz_vec4 == sizeof(ImVec4) && "Mismatched struct layout!"); }
    if (sz_vert != sizeof(ImDrawVert)) { error = true; IM_ASSERT(sz_vert == sizeof(ImDrawVert) && "Mismatched struct layout!"); }
    if (sz_idx != sizeof(ImDrawIdx)) { error = true; IM_ASSERT(sz_idx == sizeof(ImDrawIdx) && "Mismatched struct layout!"); }
    return !error;
}



static c_void ImGui::ErrorCheckNewFrameSanityChecks()
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
    // IM_ASSERT((g.IO.DeltaTime > 0f32 || g.FrameCount == 0)              && "Need a positive DeltaTime!");
    // IM_ASSERT((g.FrameCount == 0 || g.FrameCountEnded == g.FrameCount)  && "Forgot to call Render() or EndFrame() at the end of the previous frame?");
    // IM_ASSERT(g.IO.DisplaySize.x >= 0f32 && g.IO.DisplaySize.y >= 0f32  && "Invalid DisplaySize value!");
    // IM_ASSERT(g.IO.Fonts->IsBuilt()                                     && "Font Atlas not built! Make sure you called ImGui_ImplXXXX_NewFrame() function for renderer backend, which should call io.Fonts->GetTexDataAsRGBA32() / GetTexDataAsAlpha8()");
    // IM_ASSERT(g.Style.CurveTessellationTol > 0f32                       && "Invalid style setting!");
    // IM_ASSERT(g.Style.CircleTessellationMaxError > 0f32                 && "Invalid style setting!");
    // IM_ASSERT(g.Style.Alpha >= 0f32 && g.Style.Alpha <= 1f32            && "Invalid style setting!"); // Allows us to avoid a few clamps in color computations
    // IM_ASSERT(g.Style.WindowMinSize.x >= 1f32 && g.Style.WindowMinSize.y >= 1f32 && "Invalid style setting.");
    // IM_ASSERT(g.Style.WindowMenuButtonPosition == ImGuiDir_None || g.Style.WindowMenuButtonPosition == ImGuiDir_Left || g.Style.WindowMenuButtonPosition == ImGuiDir_Right);
    // IM_ASSERT(g.Style.ColorButtonPosition == ImGuiDir_Left || g.Style.ColorButtonPosition == ImGuiDir_Right);
// #ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
    for (c_int n = ImGuiKey_NamedKey_BEGIN; n < ImGuiKey_COUNT; n++)
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
            g.IO.ConfigFlags &= ~ImGuiConfigFlags_ViewportsEnable;
        }

        // Perform simple checks on platform monitor data + compute a total bounding box for quick early outs
        for (c_int monitor_n = 0; monitor_n < g.PlatformIO.Monitors.Size; monitor_n++)
        {
            ImGuiPlatformMonitor& mon = g.PlatformIO.Monitors[monitor_n];
            IM_UNUSED(mon);
            // IM_ASSERT(mon.MainSize.x > 0f32 && mon.MainSize.y > 0f32 && "Monitor main bounds not setup properly.");
            // IM_ASSERT(ImRect(mon.MainPos, mon.MainPos + mon.MainSize).Contains(ImRect(mon.WorkPos, mon.WorkPos + mon.WorkSize)) && "Monitor work bounds not setup properly. If you don't have work area information, just copy MainPos/MainSize into them.");
            // IM_ASSERT(mon.DpiScale != 0f32);
        }
    }
}

static c_void ImGui::ErrorCheckEndFrameSanityChecks()
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
c_void    ImGui::ErrorCheckEndFrameRecover(ImGuiErrorLogCallback log_callback, user_data: *mut c_void)
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
c_void    ImGui::ErrorCheckEndWindowRecover(ImGuiErrorLogCallback log_callback, user_data: *mut c_void)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    while (g.CurrentTable && (g.Currenttable.OuterWindow == g.CurrentWindow || g.Currenttable.InnerWindow == g.CurrentWindow))
    {
        if (log_callback) log_callback(user_data, "Recovered from missing EndTable() in '%s'", g.Currenttable.Outerwindow.Name);
        EndTable();
    }

    let mut window = g.CurrentWindow;
    ImGuiStackSizes* stack_sizes = &g.CurrentWindowStack.back().StackSizesOnBegin;
    // IM_ASSERT(window != NULL);
    while (g.CurrentTabBar != NULL) //-V1044
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
        if (log_callback) log_callback(user_data, "Recovered from missing PopStyleColor() in '%s' for ImGuiCol_%s", window.Name, GetStyleColorName(g.ColorStack.back().Col));
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
    SizeOfIDStack = (c_short)window.IDStack.Size;
    SizeOfColorStack = (c_short)g.ColorStack.Size;
    SizeOfStyleVarStack = (c_short)g.StyleVarStack.Size;
    SizeOfFontStack = (c_short)g.FontStack.Size;
    SizeOfFocusScopeStack = (c_short)g.FocusScopeStack.Size;
    SizeOfGroupStack = (c_short)g.GroupStack.Size;
    SizeOfItemFlagsStack = (c_short)g.ItemFlagsStack.Size;
    SizeOfBeginPopupStack = (c_short)g.BeginPopupStack.Size;
    SizeOfDisabledStack = (c_short)g.DisabledStackSize;
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

// Advance cursor given item size for layout.
// Register minimum needed size so it can extend the bounding box used for auto-fit calculation.
// See comments in ItemAdd() about how/why the size provided to ItemSize() vs ItemAdd() may often different.
c_void ImGui::ItemSize(const ImVec2& size, c_float text_baseline_y)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if (window.SkipItems)
        return;

    // We increase the height in this function to accommodate for baseline offset.
    // In theory we should be offsetting the starting position (window.DC.CursorPos), that will be the topic of a larger refactor,
    // but since ItemSize() is not yet an API that moves the cursor (to handle e.g. wrapping) enlarging the height has the same effect.
    const c_float offset_to_match_baseline_y = (text_baseline_y >= 0) ? ImMax(0f32, window.DC.CurrLineTextBaseOffset - text_baseline_y) : 0f32;

    const c_float line_y1 = window.DC.IsSameLine ? window.DC.CursorPosPrevLine.y : window.DC.CursorPos.y;
    const c_float line_height = ImMax(window.DC.CurrLineSize.y, /*ImMax(*/window.DC.CursorPos.y - line_y1/*, 0f32)*/ + size.y + offset_to_match_baseline_y);

    // Always align ourselves on pixel boundaries
    //if (g.IO.KeyAlt) window.DrawList.AddRect(window.DC.CursorPos, window.DC.CursorPos + ImVec2(size.x, line_height), IM_COL32(255,0,0,200)); // [DEBUG]
    window.DC.CursorPosPrevLine.x = window.DC.CursorPos.x + size.x;
    window.DC.CursorPosPrevLine.y = line_y1;
    window.DC.CursorPos.x = IM_FLOOR(window.Pos.x + window.DC.Indent.x + window.DC.ColumnsOffset.x);    // Next line
    window.DC.CursorPos.y = IM_FLOOR(line_y1 + line_height + g.Style.ItemSpacing.y);                    // Next line
    window.DC.CursorMaxPos.x = ImMax(window.DC.CursorMaxPos.x, window.DC.CursorPosPrevLine.x);
    window.DC.CursorMaxPos.y = ImMax(window.DC.CursorMaxPos.y, window.DC.CursorPos.y - g.Style.ItemSpacing.y);
    //if (g.IO.KeyAlt) window.DrawList.AddCircle(window.DC.CursorMaxPos, 3.0f32, IM_COL32(255,0,0,255), 4); // [DEBUG]

    window.DC.PrevLineSize.y = line_height;
    window.DC.CurrLineSize.y = 0f32;
    window.DC.PrevLineTextBaseOffset = ImMax(window.DC.CurrLineTextBaseOffset, text_baseline_y);
    window.DC.CurrLineTextBaseOffset = 0f32;
    window.DC.IsSameLine = window.DC.IsSetPos = false;

    // Horizontal layout mode
    if (window.DC.LayoutType == ImGuiLayoutType_Horizontal)
        SameLine();
}

// Declare item bounding box for clipping and interaction.
// Note that the size can be different than the one provided to ItemSize(). Typically, widgets that spread over available surface
// declare their minimum size requirement to ItemSize() and provide a larger region to ItemAdd() which is used drawing/interaction.
bool ImGui::ItemAdd(const ImRect& bb, ImGuiID id, *const ImRect nav_bb_arg, ImGuiItemFlags extra_flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;

    // Set item data
    // (DisplayRect is left untouched, made valid when ImGuiItemStatusFlags_HasDisplayRect is set)
    g.LastItemData.ID = id;
    g.LastItemData.Rect = bb;
    g.LastItemData.NavRect = nav_bb_arg ? *nav_bb_arg : bb;
    g.LastItemData.InFlags = g.CurrentItemFlags | extra_flags;
    g.LastItemData.StatusFlags = ImGuiItemStatusFlags_None;

    // Directional navigation processing
    if (id != 0)
    {
        KeepAliveID(id);

        // Runs prior to clipping early-out
        //  (a) So that NavInitRequest can be honored, for newly opened windows to select a default widget
        //  (b) So that we can scroll up/down past clipped items. This adds a small O(N) cost to regular navigation requests
        //      unfortunately, but it is still limited to one window. It may not scale very well for windows with ten of
        //      thousands of item, but at least NavMoveRequest is only set on user interaction, aka maximum once a frame.
        //      We could early out with "if (is_clipped && !g.NavInitRequest) return false;" but when we wouldn't be able
        //      to reach unclipped widgets. This would work if user had explicit scrolling control (e.g. mapped on a stick).
        // We intentionally don't check if g.NavWindow != NULL because g.NavAnyRequest should only be set when it is non null.
        // If we crash on a NULL g.NavWindow we need to fix the bug elsewhere.
        window.DC.NavLayersActiveMaskNext |= (1 << window.DC.NavLayerCurrent);
        if (g.NavId == id || g.NavAnyRequest)
            if (g.Navwindow.RootWindowForNav == window.RootWindowForNav)
                if (window == g.NavWindow || ((window.Flags | g.Navwindow.Flags) & ImGuiWindowFlags_NavFlattened))
                    NavProcessItem();

        // [DEBUG] People keep stumbling on this problem and using "" as identifier in the root of a window instead of "##something".
        // Empty identifier are valid and useful in a small amount of cases, but 99.9% of the time you want to use "##something".
        // READ THE FAQ: https://dearimgui.org/faq
        // IM_ASSERT(id != window.ID && "Cannot have an empty ID at the root of a window. If you need an empty label, use ## and read the FAQ about how the ID Stack works!");

        // [DEBUG] Item Picker tool, when enabling the "extended" version we perform the check in ItemAdd()
// #ifdef IMGUI_DEBUG_TOOL_ITEM_PICKER_EX
        if (id == g.DebugItemPickerBreakId)
        {
            IM_DEBUG_BREAK();
            g.DebugItemPickerBreakId = 0;
        }
// #endif
    }
    g.NextItemData.Flags = ImGuiNextItemDataFlags_None;

// #ifdef IMGUI_ENABLE_TEST_ENGINE
    if (id != 0)
        IMGUI_TEST_ENGINE_ITEM_ADD(nav_bb_arg ? *nav_bb_arg : bb, id);
// #endif

    // Clipping test
    let is_clipped: bool = IsClippedEx(bb, id);
    if (is_clipped)
        return false;
    //if (g.IO.KeyAlt) window.DrawList.AddRect(bb.Min, bb.Max, IM_COL32(255,255,0,120)); // [DEBUG]

    // We need to calculate this now to take account of the current clipping rectangle (as items like Selectable may change them)
    if (IsMouseHoveringRect(bb.Min, bb.Max))
        g.LastItemData.StatusFlags |= ImGuiItemStatusFlags_HoveredRect;
    return true;
}

// Gets back to previous line and continue with horizontal layout
//      offset_from_start_x == 0 : follow right after previous item
//      offset_from_start_x != 0 : align to specified x position (relative to window/group left)
//      spacing_w < 0            : use default spacing if pos_x == 0, no spacing if pos_x != 0
//      spacing_w >= 0           : enforce spacing amount
c_void ImGui::SameLine(c_float offset_from_start_x, c_float spacing_w)
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

ImVec2 ImGui::GetCursorScreenPos()
{
    ImGuiWindow* window = GetCurrentWindowRead();
    return window.DC.CursorPos;
}

// 2022/08/05: Setting cursor position also extend boundaries (via modifying CursorMaxPos) used to compute window size, group size etc.
// I believe this was is a judicious choice but it's probably being relied upon (it has been the case since 1.31 and 1.50)
// It would be sane if we requested user to use SetCursorPos() + Dummy(ImVec2(0,0)) to extend CursorMaxPos...
c_void ImGui::SetCursorScreenPos(const ImVec2& pos)
{
    ImGuiWindow* window = GetCurrentWindow();
    window.DC.CursorPos = pos;
    //window.DC.CursorMaxPos = ImMax(window.DC.CursorMaxPos, window.DC.CursorPos);
    window.DC.IsSetPos = true;
}

// User generally sees positions in window coordinates. Internally we store CursorPos in absolute screen coordinates because it is more convenient.
// Conversion happens as we pass the value to user, but it makes our naming convention confusing because GetCursorPos() == (DC.CursorPos - window.Pos). May want to rename 'DC.CursorPos'.
ImVec2 ImGui::GetCursorPos()
{
    ImGuiWindow* window = GetCurrentWindowRead();
    return window.DC.CursorPos - window.Pos + window.Scroll;
}

c_float ImGui::GetCursorPosX()
{
    ImGuiWindow* window = GetCurrentWindowRead();
    return window.DC.CursorPos.x - window.Pos.x + window.Scroll.x;
}

c_float ImGui::GetCursorPosY()
{
    ImGuiWindow* window = GetCurrentWindowRead();
    return window.DC.CursorPos.y - window.Pos.y + window.Scroll.y;
}

c_void ImGui::SetCursorPos(const ImVec2& local_pos)
{
    ImGuiWindow* window = GetCurrentWindow();
    window.DC.CursorPos = window.Pos - window.Scroll + local_pos;
    //window.DC.CursorMaxPos = ImMax(window.DC.CursorMaxPos, window.DC.CursorPos);
    window.DC.IsSetPos = true;
}

c_void ImGui::SetCursorPosX(c_float x)
{
    ImGuiWindow* window = GetCurrentWindow();
    window.DC.CursorPos.x = window.Pos.x - window.Scroll.x + x;
    //window.DC.CursorMaxPos.x = ImMax(window.DC.CursorMaxPos.x, window.DC.CursorPos.x);
    window.DC.IsSetPos = true;
}

c_void ImGui::SetCursorPosY(c_float y)
{
    ImGuiWindow* window = GetCurrentWindow();
    window.DC.CursorPos.y = window.Pos.y - window.Scroll.y + y;
    //window.DC.CursorMaxPos.y = ImMax(window.DC.CursorMaxPos.y, window.DC.CursorPos.y);
    window.DC.IsSetPos = true;
}

ImVec2 ImGui::GetCursorStartPos()
{
    ImGuiWindow* window = GetCurrentWindowRead();
    return window.DC.CursorStartPos - window.Pos;
}

c_void ImGui::Indent(c_float indent_w)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiWindow* window = GetCurrentWindow();
    window.DC.Indent.x += (indent_w != 0f32) ? indent_w : g.Style.IndentSpacing;
    window.DC.CursorPos.x = window.Pos.x + window.DC.Indent.x + window.DC.ColumnsOffset.x;
}

c_void ImGui::Unindent(c_float indent_w)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiWindow* window = GetCurrentWindow();
    window.DC.Indent.x -= (indent_w != 0f32) ? indent_w : g.Style.IndentSpacing;
    window.DC.CursorPos.x = window.Pos.x + window.DC.Indent.x + window.DC.ColumnsOffset.x;
}

// Affect large frame+labels widgets only.
c_void ImGui::SetNextItemWidth(c_float item_width)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NextItemData.Flags |= ImGuiNextItemDataFlags_HasWidth;
    g.NextItemData.Width = item_width;
}

// FIXME: Remove the == 0f32 behavior?
c_void ImGui::PushItemWidth(c_float item_width)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    window.DC.ItemWidthStack.push(window.DC.ItemWidth); // Backup current width
    window.DC.ItemWidth = (item_width == 0f32 ? window.ItemWidthDefault : item_width);
    g.NextItemData.Flags &= ~ImGuiNextItemDataFlags_HasWidth;
}

c_void ImGui::PushMultiItemsWidths(c_int components, c_float w_full)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    const ImGuiStyle& style = g.Style;
    const c_float w_item_one  = ImMax(1f32, IM_FLOOR((w_full - (style.ItemInnerSpacing.x) * (components - 1)) / components));
    const c_float w_item_last = ImMax(1f32, IM_FLOOR(w_full - (w_item_one + style.ItemInnerSpacing.x) * (components - 1)));
    window.DC.ItemWidthStack.push(window.DC.ItemWidth); // Backup current width
    window.DC.ItemWidthStack.push(w_item_last);
    for (c_int i = 0; i < components - 2; i++)
        window.DC.ItemWidthStack.push(w_item_one);
    window.DC.ItemWidth = (components == 1) ? w_item_last : w_item_one;
    g.NextItemData.Flags &= ~ImGuiNextItemDataFlags_HasWidth;
}

c_void ImGui::PopItemWidth()
{
    ImGuiWindow* window = GetCurrentWindow();
    window.DC.ItemWidth = window.DC.ItemWidthStack.back();
    window.DC.ItemWidthStack.pop_back();
}

// Calculate default item width given value passed to PushItemWidth() or SetNextItemWidth().
// The SetNextItemWidth() data is generally cleared/consumed by ItemAdd() or NextItemData.ClearFlags()
c_float ImGui::CalcItemWidth()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    c_float w;
    if (g.NextItemData.Flags & ImGuiNextItemDataFlags_HasWidth)
        w = g.NextItemData.Width;
    else
        w = window.DC.ItemWidth;
    if (w < 0f32)
    {
        c_float region_max_x = GetContentRegionMaxAbs().x;
        w = ImMax(1f32, region_max_x - window.DC.CursorPos.x + w);
    }
    w = IM_FLOOR(w);
    return w;
}

// [Internal] Calculate full item size given user provided 'size' parameter and default width/height. Default width is often == CalcItemWidth().
// Those two functions CalcItemWidth vs CalcItemSize are awkwardly named because they are not fully symmetrical.
// Note that only CalcItemWidth() is publicly exposed.
// The 4.0f32 here may be changed to match CalcItemWidth() and/or BeginChild() (right now we have a mismatch which is harmless but undesirable)
ImVec2 ImGui::CalcItemSize(ImVec2 size, c_float default_w, c_float default_h)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;

    ImVec2 region_max;
    if (size.x < 0f32 || size.y < 0f32)
        region_max = GetContentRegionMaxAbs();

    if (size.x == 0f32)
        size.x = default_w;
    else if (size.x < 0f32)
        size.x = ImMax(4.0f32, region_max.x - window.DC.CursorPos.x + size.x);

    if (size.y == 0f32)
        size.y = default_h;
    else if (size.y < 0f32)
        size.y = ImMax(4.0f32, region_max.y - window.DC.CursorPos.y + size.y);

    return size;
}

c_float ImGui::GetTextLineHeight()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.FontSize;
}

c_float ImGui::GetTextLineHeightWithSpacing()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.FontSize + g.Style.ItemSpacing.y;
}

c_float ImGui::GetFrameHeight()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.FontSize + g.Style.FramePadding.y * 2.0f32;
}

c_float ImGui::GetFrameHeightWithSpacing()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.FontSize + g.Style.FramePadding.y * 2.0f32 + g.Style.ItemSpacing.y;
}

// FIXME: All the Contents Region function are messy or misleading. WE WILL AIM TO OBSOLETE ALL OF THEM WITH A NEW "WORK RECT" API. Thanks for your patience!

// FIXME: This is in window space (not screen space!).
ImVec2 ImGui::GetContentRegionMax()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    ImVec2 mx = window.ContentRegionRect.Max - window.Pos;
    if (window.DC.CurrentColumns || g.CurrentTable)
        mx.x = window.WorkRect.Max.x - window.Pos.x;
    return mx;
}

// [Internal] Absolute coordinate. Saner. This is not exposed until we finishing refactoring work rect features.
ImVec2 ImGui::GetContentRegionMaxAbs()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    ImVec2 mx = window.ContentRegionRect.Max;
    if (window.DC.CurrentColumns || g.CurrentTable)
        mx.x = window.WorkRect.Max.x;
    return mx;
}

ImVec2 ImGui::GetContentRegionAvail()
{
    ImGuiWindow* window = GimGui.CurrentWindow;
    return GetContentRegionMaxAbs() - window.DC.CursorPos;
}

// In window space (not screen space!)
ImVec2 ImGui::GetWindowContentRegionMin()
{
    ImGuiWindow* window = GimGui.CurrentWindow;
    return window.ContentRegionRect.Min - window.Pos;
}

ImVec2 ImGui::GetWindowContentRegionMax()
{
    ImGuiWindow* window = GimGui.CurrentWindow;
    return window.ContentRegionRect.Max - window.Pos;
}

// Lock horizontal starting position + capture group bounding box into one "item" (so you can use IsItemHovered() or layout primitives such as SameLine() on whole group, etc.)
// Groups are currently a mishmash of functionalities which should perhaps be clarified and separated.
// FIXME-OPT: Could we safely early out on ->SkipItems?
c_void ImGui::BeginGroup()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;

    g.GroupStack.resize(g.GroupStack.Size + 1);
    ImGuiGroupData& group_data = g.GroupStack.back();
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
    window.DC.CurrLineSize = ImVec2(0f32, 0f32);
    if (g.LogEnabled)
        g.LogLinePosY = -f32::MAX; // To enforce a carriage return
}

c_void ImGui::EndGroup()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    // IM_ASSERT(g.GroupStack.Size > 0); // Mismatched BeginGroup()/EndGroup() calls

    ImGuiGroupData& group_data = g.GroupStack.back();
    // IM_ASSERT(group_data.WindowID == window.ID); // EndGroup() in wrong window?

    if (window.DC.IsSetPos)
        ErrorCheckUsingSetCursorPosToExtendParentBoundaries();

    ImRect group_bb(group_data.BackupCursorPos, ImMax(window.DC.CursorMaxPos, group_data.BackupCursorPos));

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
    ItemAdd(group_bb, 0, NULL, ImGuiItemFlags_NoTabStop);

    // If the current ActiveId was declared within the boundary of our group, we copy it to LastItemId so IsItemActive(), IsItemDeactivated() etc. will be functional on the entire group.
    // It would be be neater if we replaced window.DC.LastItemId by e.g. 'bool LastItemIsActive', but would put a little more burden on individual widgets.
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
static c_float CalcScrollEdgeSnap(c_float target, c_float snap_min, c_float snap_max, c_float snap_threshold, c_float center_ratio)
{
    if (target <= snap_min + snap_threshold)
        return ImLerp(snap_min, target, center_ratio);
    if (target >= snap_max - snap_threshold)
        return ImLerp(target, snap_max, center_ratio);
    return target;
}

static ImVec2 CalcNextScrollFromScrollTargetAndClamp(ImGuiWindow* window)
{
    ImVec2 scroll = window.Scroll;
    if (window.ScrollTarget.x < f32::MAX)
    {
        c_float decoration_total_width = window.ScrollbarSizes.x;
        c_float center_x_ratio = window.ScrollTargetCenterRatio.x;
        c_float scroll_target_x = window.ScrollTarget.x;
        if (window.ScrollTargetEdgeSnapDist.x > 0f32)
        {
            c_float snap_x_min = 0f32;
            c_float snap_x_max = window.ScrollMax.x + window.SizeFull.x - decoration_total_width;
            scroll_target_x = CalcScrollEdgeSnap(scroll_target_x, snap_x_min, snap_x_max, window.ScrollTargetEdgeSnapDist.x, center_x_ratio);
        }
        scroll.x = scroll_target_x - center_x_ratio * (window.SizeFull.x - decoration_total_width);
    }
    if (window.ScrollTarget.y < f32::MAX)
    {
        c_float decoration_total_height = window.TitleBarHeight() + window.MenuBarHeight() + window.ScrollbarSizes.y;
        c_float center_y_ratio = window.ScrollTargetCenterRatio.y;
        c_float scroll_target_y = window.ScrollTarget.y;
        if (window.ScrollTargetEdgeSnapDist.y > 0f32)
        {
            c_float snap_y_min = 0f32;
            c_float snap_y_max = window.ScrollMax.y + window.SizeFull.y - decoration_total_height;
            scroll_target_y = CalcScrollEdgeSnap(scroll_target_y, snap_y_min, snap_y_max, window.ScrollTargetEdgeSnapDist.y, center_y_ratio);
        }
        scroll.y = scroll_target_y - center_y_ratio * (window.SizeFull.y - decoration_total_height);
    }
    scroll.x = IM_FLOOR(ImMax(scroll.x, 0f32));
    scroll.y = IM_FLOOR(ImMax(scroll.y, 0f32));
    if (!window.Collapsed && !window.SkipItems)
    {
        scroll.x = ImMin(scroll.x, window.ScrollMax.x);
        scroll.y = ImMin(scroll.y, window.ScrollMax.y);
    }
    return scroll;
}

c_void ImGui::ScrollToItem(ImGuiScrollFlags flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    ScrollToRectEx(window, g.LastItemData.NavRect, flags);
}

c_void ImGui::ScrollToRect(ImGuiWindow* window, const ImRect& item_rect, ImGuiScrollFlags flags)
{
    ScrollToRectEx(window, item_rect, flags);
}

// Scroll to keep newly navigated item fully into view
ImVec2 ImGui::ScrollToRectEx(ImGuiWindow* window, const ImRect& item_rect, ImGuiScrollFlags flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImRect window_rect(window.InnerRect.Min - ImVec2(1, 1), window.InnerRect.Max + ImVec2(1, 1));
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
            SetScrollFromPosX(window, item_rect.Min.x - g.Style.ItemSpacing.x - window.Pos.x, 0f32);
        else if (item_rect.Max.x >= window_rect.Max.x)
            SetScrollFromPosX(window, item_rect.Max.x + g.Style.ItemSpacing.x - window.Pos.x, 1f32);
    }
    else if (((flags & ImGuiScrollFlags_KeepVisibleCenterX) && !fully_visible_x) || (flags & ImGuiScrollFlags_AlwaysCenterX))
    {
        c_float target_x = can_be_fully_visible_x ? ImFloor((item_rect.Min.x + item_rect.Max.x - window.InnerRect.GetWidth()) * 0.5f32) : item_rect.Min.x;
        SetScrollFromPosX(window, target_x - window.Pos.x, 0f32);
    }

    if ((flags & ImGuiScrollFlags_KeepVisibleEdgeY) && !fully_visible_y)
    {
        if (item_rect.Min.y < window_rect.Min.y || !can_be_fully_visible_y)
            SetScrollFromPosY(window, item_rect.Min.y - g.Style.ItemSpacing.y - window.Pos.y, 0f32);
        else if (item_rect.Max.y >= window_rect.Max.y)
            SetScrollFromPosY(window, item_rect.Max.y + g.Style.ItemSpacing.y - window.Pos.y, 1f32);
    }
    else if (((flags & ImGuiScrollFlags_KeepVisibleCenterY) && !fully_visible_y) || (flags & ImGuiScrollFlags_AlwaysCenterY))
    {
        c_float target_y = can_be_fully_visible_y ? ImFloor((item_rect.Min.y + item_rect.Max.y - window.InnerRect.GetHeight()) * 0.5f32) : item_rect.Min.y;
        SetScrollFromPosY(window, target_y - window.Pos.y, 0f32);
    }

    ImVec2 next_scroll = CalcNextScrollFromScrollTargetAndClamp(window);
    ImVec2 delta_scroll = next_scroll - window.Scroll;

    // Also scroll parent window to keep us into view if necessary
    if (!(flags & ImGuiScrollFlags_NoScrollParent) && (window.Flags & ImGuiWindowFlags_ChildWindow))
    {
        // FIXME-SCROLL: May be an option?
        if ((in_flags & (ImGuiScrollFlags_AlwaysCenterX | ImGuiScrollFlags_KeepVisibleCenterX)) != 0)
            in_flags = (in_flags & ~ImGuiScrollFlags_MaskX_) | ImGuiScrollFlags_KeepVisibleEdgeX;
        if ((in_flags & (ImGuiScrollFlags_AlwaysCenterY | ImGuiScrollFlags_KeepVisibleCenterY)) != 0)
            in_flags = (in_flags & ~ImGuiScrollFlags_MaskY_) | ImGuiScrollFlags_KeepVisibleEdgeY;
        delta_scroll += ScrollToRectEx(window.ParentWindow, ImRect(item_rect.Min - delta_scroll, item_rect.Max - delta_scroll), in_flags);
    }

    return delta_scroll;
}

c_float ImGui::GetScrollX()
{
    ImGuiWindow* window = GimGui.CurrentWindow;
    return window.Scroll.x;
}

c_float ImGui::GetScrollY()
{
    ImGuiWindow* window = GimGui.CurrentWindow;
    return window.Scroll.y;
}

c_float ImGui::GetScrollMaxX()
{
    ImGuiWindow* window = GimGui.CurrentWindow;
    return window.ScrollMax.x;
}

c_float ImGui::GetScrollMaxY()
{
    ImGuiWindow* window = GimGui.CurrentWindow;
    return window.ScrollMax.y;
}

c_void ImGui::SetScrollX(ImGuiWindow* window, c_float scroll_x)
{
    window.ScrollTarget.x = scroll_x;
    window.ScrollTargetCenterRatio.x = 0f32;
    window.ScrollTargetEdgeSnapDist.x = 0f32;
}

c_void ImGui::SetScrollY(ImGuiWindow* window, c_float scroll_y)
{
    window.ScrollTarget.y = scroll_y;
    window.ScrollTargetCenterRatio.y = 0f32;
    window.ScrollTargetEdgeSnapDist.y = 0f32;
}

c_void ImGui::SetScrollX(c_float scroll_x)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    SetScrollX(g.CurrentWindow, scroll_x);
}

c_void ImGui::SetScrollY(c_float scroll_y)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    SetScrollY(g.CurrentWindow, scroll_y);
}

// Note that a local position will vary depending on initial scroll value,
// This is a little bit confusing so bear with us:
//  - local_pos = (absolution_pos - window.Pos)
//  - So local_x/local_y are 0f32 for a position at the upper-left corner of a window,
//    and generally local_x/local_y are >(padding+decoration) && <(size-padding-decoration) when in the visible area.
//  - They mostly exists because of legacy API.
// Following the rules above, when trying to work with scrolling code, consider that:
//  - SetScrollFromPosY(0f32) == SetScrollY(0f32 + scroll.y) == has no effect!
//  - SetScrollFromPosY(-scroll.y) == SetScrollY(-scroll.y + scroll.y) == SetScrollY(0f32) == reset scroll. Of course writing SetScrollY(0f32) directly then makes more sense
// We store a target position so centering and clamping can occur on the next frame when we are guaranteed to have a known window size
c_void ImGui::SetScrollFromPosX(ImGuiWindow* window, c_float local_x, c_float center_x_ratio)
{
    // IM_ASSERT(center_x_ratio >= 0f32 && center_x_ratio <= 1f32);
    window.ScrollTarget.x = IM_FLOOR(local_x + window.Scroll.x); // Convert local position to scroll offset
    window.ScrollTargetCenterRatio.x = center_x_ratio;
    window.ScrollTargetEdgeSnapDist.x = 0f32;
}

c_void ImGui::SetScrollFromPosY(ImGuiWindow* window, c_float local_y, c_float center_y_ratio)
{
    // IM_ASSERT(center_y_ratio >= 0f32 && center_y_ratio <= 1f32);
    const c_float decoration_up_height = window.TitleBarHeight() + window.MenuBarHeight(); // FIXME: Would be nice to have a more standardized access to our scrollable/client rect;
    local_y -= decoration_up_height;
    window.ScrollTarget.y = IM_FLOOR(local_y + window.Scroll.y); // Convert local position to scroll offset
    window.ScrollTargetCenterRatio.y = center_y_ratio;
    window.ScrollTargetEdgeSnapDist.y = 0f32;
}

c_void ImGui::SetScrollFromPosX(c_float local_x, c_float center_x_ratio)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    SetScrollFromPosX(g.CurrentWindow, local_x, center_x_ratio);
}

c_void ImGui::SetScrollFromPosY(c_float local_y, c_float center_y_ratio)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    SetScrollFromPosY(g.CurrentWindow, local_y, center_y_ratio);
}

// center_x_ratio: 0f32 left of last item, 0.5f32 horizontal center of last item, 1f32 right of last item.
c_void ImGui::SetScrollHereX(c_float center_x_ratio)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    c_float spacing_x = ImMax(window.WindowPadding.x, g.Style.ItemSpacing.x);
    c_float target_pos_x = ImLerp(g.LastItemData.Rect.Min.x - spacing_x, g.LastItemData.Rect.Max.x + spacing_x, center_x_ratio);
    SetScrollFromPosX(window, target_pos_x - window.Pos.x, center_x_ratio); // Convert from absolute to local pos

    // Tweak: snap on edges when aiming at an item very close to the edge
    window.ScrollTargetEdgeSnapDist.x = ImMax(0f32, window.WindowPadding.x - spacing_x);
}

// center_y_ratio: 0f32 top of last item, 0.5f32 vertical center of last item, 1f32 bottom of last item.
c_void ImGui::SetScrollHereY(c_float center_y_ratio)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    c_float spacing_y = ImMax(window.WindowPadding.y, g.Style.ItemSpacing.y);
    c_float target_pos_y = ImLerp(window.DC.CursorPosPrevLine.y - spacing_y, window.DC.CursorPosPrevLine.y + window.DC.PrevLineSize.y + spacing_y, center_y_ratio);
    SetScrollFromPosY(window, target_pos_y - window.Pos.y, center_y_ratio); // Convert from absolute to local pos

    // Tweak: snap on edges when aiming at an item very close to the edge
    window.ScrollTargetEdgeSnapDist.y = ImMax(0f32, window.WindowPadding.y - spacing_y);
}

//-----------------------------------------------------------------------------
// [SECTION] TOOLTIPS
//-----------------------------------------------------------------------------

c_void ImGui::BeginTooltip()
{
    BeginTooltipEx(ImGuiTooltipFlags_None, ImGuiWindowFlags_None);
}

c_void ImGui::BeginTooltipEx(ImGuiTooltipFlags tooltip_flags, ImGuiWindowFlags extra_window_flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    if (g.DragDropWithinSource || g.DragDropWithinTarget)
    {
        // The default tooltip position is a little offset to give space to see the context menu (it's also clamped within the current viewport/monitor)
        // In the context of a dragging tooltip we try to reduce that offset and we enforce following the cursor.
        // Whatever we do we want to call SetNextWindowPos() to enforce a tooltip position and disable clipping the tooltip without our display area, like regular tooltip do.
        //ImVec2 tooltip_pos = g.IO.MousePos - g.ActiveIdClickOffset - g.Style.WindowPadding;
        ImVec2 tooltip_pos = g.IO.MousePos + ImVec2(16 * g.Style.MouseCursorScale, 8 * g.Style.MouseCursorScale);
        SetNextWindowPos(tooltip_pos);
        SetNextWindowBgAlpha(g.Style.Colors[ImGuiCol_PopupBg].w * 0.600f32);
        //PushStyleVar(ImGuiStyleVar_Alpha, g.Style.Alpha * 0.600f32); // This would be nice but e.g ColorButton with checkboard has issue with transparent colors :(
        tooltip_flags |= ImGuiTooltipFlags_OverridePreviousTooltip;
    }

    window_name: [c_char;16];
    ImFormatString(window_name, IM_ARRAYSIZE(window_name), "##Tooltip_%02d", g.TooltipOverrideCount);
    if (tooltip_flags & ImGuiTooltipFlags_OverridePreviousTooltip)
        if (ImGuiWindow* window = FindWindowByName(window_name))
            if (window.Active)
            {
                // Hide previous tooltip from being displayed. We can't easily "reset" the content of a window so we create a new one.
                window.Hidden = true;
                window.HiddenFramesCanSkipItems = 1; // FIXME: This may not be necessary?
                ImFormatString(window_name, IM_ARRAYSIZE(window_name), "##Tooltip_%02d", ++g.TooltipOverrideCount);
            }
    ImGuiWindowFlags flags = ImGuiWindowFlags_Tooltip | ImGuiWindowFlags_NoInputs | ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoMove | ImGuiWindowFlags_NoResize | ImGuiWindowFlags_NoSavedSettings | ImGuiWindowFlags_AlwaysAutoResize | ImGuiWindowFlags_NoDocking;
    Begin(window_name, NULL, flags | extra_window_flags);
}

c_void ImGui::EndTooltip()
{
    // IM_ASSERT(GetCurrentWindowRead()->Flags & ImGuiWindowFlags_Tooltip);   // Mismatched BeginTooltip()/EndTooltip() calls
    End();
}

c_void ImGui::SetTooltipV(*const char fmt, va_list args)
{
    BeginTooltipEx(ImGuiTooltipFlags_OverridePreviousTooltip, ImGuiWindowFlags_None);
    TextV(fmt, args);
    EndTooltip();
}

c_void ImGui::SetTooltip(*const char fmt, ...)
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
bool ImGui::IsPopupOpen(ImGuiID id, ImGuiPopupFlags popup_flags)
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
            for (c_int n = 0; n < g.OpenPopupStack.Size; n++)
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

bool ImGui::IsPopupOpen(*const char str_id, ImGuiPopupFlags popup_flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiID id = (popup_flags & ImGuiPopupFlags_AnyPopupId) ? 0 : g.Currentwindow.GetID(str_id);
    if ((popup_flags & ImGuiPopupFlags_AnyPopupLevel) && id != 0)
        // IM_ASSERT(0 && "Cannot use IsPopupOpen() with a string id and ImGuiPopupFlags_AnyPopupLevel."); // But non-string version is legal and used internally
    return IsPopupOpen(id, popup_flags);
}

ImGuiWindow* ImGui::GetTopMostPopupModal()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    for (c_int n = g.OpenPopupStack.Size - 1; n >= 0; n--)
        if (ImGuiWindow* popup = g.OpenPopupStack.Data[n].Window)
            if (popup->Flags & ImGuiWindowFlags_Modal)
                return popup;
    return NULL;
}

ImGuiWindow* ImGui::GetTopMostAndVisiblePopupModal()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    for (c_int n = g.OpenPopupStack.Size - 1; n >= 0; n--)
        if (ImGuiWindow* popup = g.OpenPopupStack.Data[n].Window)
            if ((popup->Flags & ImGuiWindowFlags_Modal) && IsWindowActiveAndVisible(popup))
                return popup;
    return NULL;
}

c_void ImGui::OpenPopup(*const char str_id, ImGuiPopupFlags popup_flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiID id = g.Currentwindow.GetID(str_id);
    IMGUI_DEBUG_LOG_POPUP("[popup] OpenPopup(\"%s\" -> 0x%08X\n", str_id, id);
    OpenPopupEx(id, popup_flags);
}

c_void ImGui::OpenPopup(ImGuiID id, ImGuiPopupFlags popup_flags)
{
    OpenPopupEx(id, popup_flags);
}

// Mark popup as open (toggle toward open state).
// Popups are closed when user click outside, or activate a pressable item, or CloseCurrentPopup() is called within a BeginPopup()/EndPopup() block.
// Popup identifiers are relative to the current ID-stack (so OpenPopup and BeginPopup needs to be at the same level).
// One open popup per level of the popup hierarchy (NB: when assigning we reset the Window member of ImGuiPopupRef to NULL)
c_void ImGui::OpenPopupEx(ImGuiID id, ImGuiPopupFlags popup_flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiWindow* parent_window = g.CurrentWindow;
    let current_stack_size: c_int = g.BeginPopupStack.Size;

    if (popup_flags & ImGuiPopupFlags_NoOpenOverExistingPopup)
        if (IsPopupOpen(0u, ImGuiPopupFlags_AnyPopupId))
            return;

    ImGuiPopupData popup_ref; // Tagged as new ref as Window will be set back to NULL if we write this into OpenPopupStack.
    popup_ref.PopupId = id;
    popup_ref.Window = None;
    popup_ref.BackupNavWindow = g.NavWindow;            // When popup closes focus may be restored to NavWindow (depend on window type).
    popup_ref.OpenFrameCount = g.FrameCount;
    popup_ref.OpenParentId = parent_window.IDStack.back();
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
c_void ImGui::ClosePopupsOverWindow(ImGuiWindow* ref_window, bool restore_focus_to_window_under_popup)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.OpenPopupStack.Size == 0)
        return;

    // Don't close our own child popup windows.
    c_int popup_count_to_keep = 0;
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
            for (c_int n = popup_count_to_keep; n < g.OpenPopupStack.Size; n++)
                if (ImGuiWindow* popup_window = g.OpenPopupStack[n].Window)
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

c_void ImGui::ClosePopupsExceptModals()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    c_int popup_count_to_keep;
    for (popup_count_to_keep = g.OpenPopupStack.Size; popup_count_to_keep > 0; popup_count_to_keep--)
    {
        ImGuiWindow* window = g.OpenPopupStack[popup_count_to_keep - 1].Window;
        if (!window || window.Flags & ImGuiWindowFlags_Modal)
            break;
    }
    if (popup_count_to_keep < g.OpenPopupStack.Size) // This test is not required but it allows to set a convenient breakpoint on the statement below
        ClosePopupToLevel(popup_count_to_keep, true);
}

c_void ImGui::ClosePopupToLevel(c_int remaining, bool restore_focus_to_window_under_popup)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    IMGUI_DEBUG_LOG_POPUP("[popup] ClosePopupToLevel(%d), restore_focus_to_window_under_popup=%d\n", remaining, restore_focus_to_window_under_popup);
    // IM_ASSERT(remaining >= 0 && remaining < g.OpenPopupStack.Size);

    // Trim open popup stack
    ImGuiWindow* popup_window = g.OpenPopupStack[remaining].Window;
    ImGuiWindow* popup_backup_nav_window = g.OpenPopupStack[remaining].BackupNavWindow;
    g.OpenPopupStack.resize(remaining);

    if (restore_focus_to_window_under_popup)
    {
        ImGuiWindow* focus_window = (popup_window && popup_window.Flags & ImGuiWindowFlags_ChildMenu) ? popup_window.ParentWindow : popup_backup_nav_window;
        if (focus_window && !focus_window.WasActive && popup_window)
        {
            // Fallback
            FocusTopMostWindowUnderOne(popup_window, NULL);
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
c_void ImGui::CloseCurrentPopup()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    c_int popup_idx = g.BeginPopupStack.Size - 1;
    if (popup_idx < 0 || popup_idx >= g.OpenPopupStack.Size || g.BeginPopupStack[popup_idx].PopupId != g.OpenPopupStack[popup_idx].PopupId)
        return;

    // Closing a menu closes its top-most parent popup (unless a modal)
    while (popup_idx > 0)
    {
        ImGuiWindow* popup_window = g.OpenPopupStack[popup_idx].Window;
        ImGuiWindow* parent_popup_window = g.OpenPopupStack[popup_idx - 1].Window;
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
    if (ImGuiWindow* window = g.NavWindow)
        window.DC.NavHideHighlightOneFrame = true;
}

// Attention! BeginPopup() adds default flags which BeginPopupEx()!
bool ImGui::BeginPopupEx(ImGuiID id, ImGuiWindowFlags flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (!IsPopupOpen(id, ImGuiPopupFlags_None))
    {
        g.NextWindowData.ClearFlags(); // We behave like Begin() and need to consume those values
        return false;
    }

    name: [c_char;20];
    if (flags & ImGuiWindowFlags_ChildMenu)
        ImFormatString(name, IM_ARRAYSIZE(name), "##Menu_%02d", g.BeginMenuCount); // Recycle windows based on depth
    else
        ImFormatString(name, IM_ARRAYSIZE(name), "##Popup_%08x", id); // Not recycling, so we can close/open during the same frame

    flags |= ImGuiWindowFlags_Popup | ImGuiWindowFlags_NoDocking;
    let mut is_open: bool =  Begin(name, NULL, flags);
    if (!is_open) // NB: Begin can return false when the popup is completely clipped (e.g. zero size display)
        EndPopup();

    return is_open;
}

bool ImGui::BeginPopup(*const char str_id, ImGuiWindowFlags flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.OpenPopupStack.Size <= g.BeginPopupStack.Size) // Early out for performance
    {
        g.NextWindowData.ClearFlags(); // We behave like Begin() and need to consume those values
        return false;
    }
    flags |= ImGuiWindowFlags_AlwaysAutoResize | ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoSavedSettings;
    ImGuiID id = g.Currentwindow.GetID(str_id);
    return BeginPopupEx(id, flags);
}

// If 'p_open' is specified for a modal popup window, the popup will have a regular close button which will close the popup.
// Note that popup visibility status is owned by Dear ImGui (and manipulated with e.g. OpenPopup) so the actual value of *p_open is meaningless here.
bool ImGui::BeginPopupModal(*const char name, bool* p_open, ImGuiWindowFlags flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    const ImGuiID id = window.GetID(name);
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
        *const ImGuiViewport viewport = window.WasActive ? window.Viewport : GetMainViewport(); // FIXME-VIEWPORT: What may be our reference viewport?
        SetNextWindowPos(viewport->GetCenter(), ImGuiCond_FirstUseEver, ImVec2(0.5f32, 0.5f32));
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

c_void ImGui::EndPopup()
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
c_void ImGui::OpenPopupOnItemClick(*const char str_id, ImGuiPopupFlags popup_flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    c_int mouse_button = (popup_flags & ImGuiPopupFlags_MouseButtonMask_);
    if (IsMouseReleased(mouse_button) && IsItemHovered(ImGuiHoveredFlags_AllowWhenBlockedByPopup))
    {
        ImGuiID id = str_id ? window.GetID(str_id) : g.LastItemData.ID;    // If user hasn't passed an ID, we can use the LastItemID. Using LastItemID as a Popup ID won't conflict!
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
bool ImGui::BeginPopupContextItem(*const char str_id, ImGuiPopupFlags popup_flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if (window.SkipItems)
        return false;
    ImGuiID id = str_id ? window.GetID(str_id) : g.LastItemData.ID;    // If user hasn't passed an ID, we can use the LastItemID. Using LastItemID as a Popup ID won't conflict!
    // IM_ASSERT(id != 0);                                             // You cannot pass a NULL str_id if the last item has no identifier (e.g. a Text() item)
    c_int mouse_button = (popup_flags & ImGuiPopupFlags_MouseButtonMask_);
    if (IsMouseReleased(mouse_button) && IsItemHovered(ImGuiHoveredFlags_AllowWhenBlockedByPopup))
        OpenPopupEx(id, popup_flags);
    return BeginPopupEx(id, ImGuiWindowFlags_AlwaysAutoResize | ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoSavedSettings);
}

bool ImGui::BeginPopupContextWindow(*const char str_id, ImGuiPopupFlags popup_flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if (!str_id)
        str_id = "window_context";
    ImGuiID id = window.GetID(str_id);
    c_int mouse_button = (popup_flags & ImGuiPopupFlags_MouseButtonMask_);
    if (IsMouseReleased(mouse_button) && IsWindowHovered(ImGuiHoveredFlags_AllowWhenBlockedByPopup))
        if (!(popup_flags & ImGuiPopupFlags_NoOpenOverItems) || !IsAnyItemHovered())
            OpenPopupEx(id, popup_flags);
    return BeginPopupEx(id, ImGuiWindowFlags_AlwaysAutoResize | ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoSavedSettings);
}

bool ImGui::BeginPopupContextVoid(*const char str_id, ImGuiPopupFlags popup_flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if (!str_id)
        str_id = "void_context";
    ImGuiID id = window.GetID(str_id);
    c_int mouse_button = (popup_flags & ImGuiPopupFlags_MouseButtonMask_);
    if (IsMouseReleased(mouse_button) && !IsWindowHovered(ImGuiHoveredFlags_AnyWindow))
        if (GetTopMostPopupModal() == NULL)
            OpenPopupEx(id, popup_flags);
    return BeginPopupEx(id, ImGuiWindowFlags_AlwaysAutoResize | ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoSavedSettings);
}

// r_avoid = the rectangle to avoid (e.g. for tooltip it is a rectangle around the mouse cursor which we want to avoid. for popups it's a small point around the cursor.)
// r_outer = the visible area rectangle, minus safe area padding. If our popup size won't fit because of safe area padding we ignore it.
// (r_outer is usually equivalent to the viewport rectangle minus padding, but when multi-viewports are enabled and monitor
//  information are available, it may represent the entire platform monitor from the frame of reference of the current viewport.
//  this allows us to have tooltips/popups displayed out of the parent viewport.)
ImVec2 ImGui::FindBestWindowPosForPopupEx(const ImVec2& ref_pos, const ImVec2& size, ImGuiDir* last_dir, const ImRect& r_outer, const ImRect& r_avoid, ImGuiPopupPositionPolicy policy)
{
    ImVec2 base_pos_clamped = ImClamp(ref_pos, r_outer.Min, r_outer.Max - size);
    //GetForegroundDrawList()->AddRect(r_avoid.Min, r_avoid.Max, IM_COL32(255,0,0,255));
    //GetForegroundDrawList()->AddRect(r_outer.Min, r_outer.Max, IM_COL32(0,255,0,255));

    // Combo Box policy (we want a connecting edge)
    if (policy == ImGuiPopupPositionPolicy_ComboBox)
    {
        const ImGuiDir dir_prefered_order[ImGuiDir_COUNT] = { ImGuiDir_Down, ImGuiDir_Right, ImGuiDir_Left, ImGuiDir_Up };
        for (c_int n = (*last_dir != ImGuiDir_None) ? -1 : 0; n < ImGuiDir_COUNT; n++)
        {
            const ImGuiDir dir = (n == -1) ? *last_dir : dir_prefered_order[n];
            if (n != -1 && dir == *last_dir) // Already tried this direction?
                continue;
            ImVec2 pos;
            if (dir == ImGuiDir_Down)  pos = ImVec2(r_avoid.Min.x, r_avoid.Max.y);          // Below, Toward Right (default)
            if (dir == ImGuiDir_Right) pos = ImVec2(r_avoid.Min.x, r_avoid.Min.y - size.y); // Above, Toward Right
            if (dir == ImGuiDir_Left)  pos = ImVec2(r_avoid.Max.x - size.x, r_avoid.Max.y); // Below, Toward Left
            if (dir == ImGuiDir_Up)    pos = ImVec2(r_avoid.Max.x - size.x, r_avoid.Min.y - size.y); // Above, Toward Left
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
        const ImGuiDir dir_prefered_order[ImGuiDir_COUNT] = { ImGuiDir_Right, ImGuiDir_Down, ImGuiDir_Up, ImGuiDir_Left };
        for (c_int n = (*last_dir != ImGuiDir_None) ? -1 : 0; n < ImGuiDir_COUNT; n++)
        {
            const ImGuiDir dir = (n == -1) ? *last_dir : dir_prefered_order[n];
            if (n != -1 && dir == *last_dir) // Already tried this direction?
                continue;

            const c_float avail_w = (dir == ImGuiDir_Left ? r_avoid.Min.x : r_outer.Max.x) - (dir == ImGuiDir_Right ? r_avoid.Max.x : r_outer.Min.x);
            const c_float avail_h = (dir == ImGuiDir_Up ? r_avoid.Min.y : r_outer.Max.y) - (dir == ImGuiDir_Down ? r_avoid.Max.y : r_outer.Min.y);

            // If there not enough room on one axis, there's no point in positioning on a side on this axis (e.g. when not enough width, use a top/bottom position to maximize available width)
            if (avail_w < size.x && (dir == ImGuiDir_Left || dir == ImGuiDir_Right))
                continue;
            if (avail_h < size.y && (dir == ImGuiDir_Up || dir == ImGuiDir_Down))
                continue;

            ImVec2 pos;
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
        return ref_pos + ImVec2(2, 2);

    // Otherwise try to keep within display
    ImVec2 pos = ref_pos;
    pos.x = ImMax(ImMin(pos.x + size.x, r_outer.Max.x) - size.x, r_outer.Min.x);
    pos.y = ImMax(ImMin(pos.y + size.y, r_outer.Max.y) - size.y, r_outer.Min.y);
    return pos;
}

// Note that this is used for popups, which can overlap the non work-area of individual viewports.
ImRect ImGui::GetPopupAllowedExtentRect(ImGuiWindow* window)
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
        r_screen = window.Viewport->GetMainRect();
    }
    ImVec2 padding = g.Style.DisplaySafeAreaPadding;
    r_screen.Expand(ImVec2((r_screen.GetWidth() > padding.x * 2) ? -padding.x : 0f32, (r_screen.GetHeight() > padding.y * 2) ? -padding.y : 0f32));
    return r_screen;
}

ImVec2 ImGui::FindBestWindowPosForPopup(ImGuiWindow* window)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    ImRect r_outer = GetPopupAllowedExtentRect(window);
    if (window.Flags & ImGuiWindowFlags_ChildMenu)
    {
        // Child menus typically request _any_ position within the parent menu item, and then we move the new menu outside the parent bounds.
        // This is how we end up with child menus appearing (most-commonly) on the right of the parent menu.
        ImGuiWindow* parent_window = window.ParentWindow;
        c_float horizontal_overlap = g.Style.ItemInnerSpacing.x; // We want some overlap to convey the relative depth of each menu (currently the amount of overlap is hard-coded to style.ItemSpacing.x).
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
        c_float sc = g.Style.MouseCursorScale;
        ImVec2 ref_pos = NavCalcPreferredRefPos();
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

c_void ImGui::SetNavWindow(ImGuiWindow* window)
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

c_void ImGui::SetNavID(ImGuiID id, ImGuiNavLayer nav_layer, ImGuiID focus_scope_id, const ImRect& rect_rel)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.NavWindow != NULL);
    // IM_ASSERT(nav_layer == ImGuiNavLayer_Main || nav_layer == ImGuiNavLayer_Menu);
    g.NavId = id;
    g.NavLayer = nav_layer;
    g.NavFocusScopeId = focus_scope_id;
    g.Navwindow.NavLastIds[nav_layer] = id;
    g.Navwindow.NavRectRel[nav_layer] = rect_rel;
}

c_void ImGui::SetFocusID(ImGuiID id, ImGuiWindow* window)
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

ImGuiDir ImGetDirQuadrantFromDelta(c_float dx, c_float dy)
{
    if (ImFabs(dx) > ImFabs(dy))
        return (dx > 0f32) ? ImGuiDir_Right : ImGuiDir_Left;
    return (dy > 0f32) ? ImGuiDir_Down : ImGuiDir_Up;
}

static c_float inline NavScoreItemDistInterval(c_float a0, c_float a1, c_float b0, c_float b1)
{
    if (a1 < b0)
        return a1 - b0;
    if (b1 < a0)
        return a0 - b1;
    return 0f32;
}

static c_void inline NavClampRectToVisibleAreaForMoveDir(ImGuiDir move_dir, ImRect& r, const ImRect& clip_rect)
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
static bool ImGui::NavScoreItem(ImGuiNavItemData* result)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if (g.NavLayer != window.DC.NavLayerCurrent)
        return false;

    // FIXME: Those are not good variables names
    ImRect cand = g.LastItemData.NavRect;   // Current item nav rectangle
    const ImRect curr = g.NavScoringRect;   // Current modified source rect (NB: we've applied Max.x = Min.x in NavUpdate() to inhibit the effect of having varied item width)
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
    c_float dbx = NavScoreItemDistInterval(cand.Min.x, cand.Max.x, curr.Min.x, curr.Max.x);
    c_float dby = NavScoreItemDistInterval(ImLerp(cand.Min.y, cand.Max.y, 0.20f32), ImLerp(cand.Min.y, cand.Max.y, 0.80f32), ImLerp(curr.Min.y, curr.Max.y, 0.20f32), ImLerp(curr.Min.y, curr.Max.y, 0.80f32)); // Scale down on Y to keep using box-distance for vertically touching items
    if (dby != 0f32 && dbx != 0f32)
        dbx = (dbx / 1000f32) + ((dbx > 0f32) ? +1f32 : -1f32);
    c_float dist_box = ImFabs(dbx) + ImFabs(dby);

    // Compute distance between centers (this is off by a factor of 2, but we only compare center distances with each other so it doesn't matter)
    c_float dcx = (cand.Min.x + cand.Max.x) - (curr.Min.x + curr.Max.x);
    c_float dcy = (cand.Min.y + cand.Max.y) - (curr.Min.y + curr.Max.y);
    c_float dist_center = ImFabs(dcx) + ImFabs(dcy); // L1 metric (need this for our connectedness guarantee)

    // Determine which quadrant of 'curr' our candidate item 'cand' lies in based on distance
    ImGuiDir quadrant;
    c_float dax = 0f32, day = 0f32, dist_axial = 0f32;
    if (dbx != 0f32 || dby != 0f32)
    {
        // For non-overlapping boxes, use distance between boxes
        dax = dbx;
        day = dby;
        dist_axial = dist_box;
        quadrant = ImGetDirQuadrantFromDelta(dbx, dby);
    }
    else if (dcx != 0f32 || dcy != 0f32)
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
        ImFormatString(buf, IM_ARRAYSIZE(bu0f32), "dbox (%.2f,%.2f->%.40f32)\ndcen (%.2f,%.2f->%.40f32)\nd (%.2f,%.2f->%.40f32)\nnav %c, quadrant %c", dbx, dby, dist_box, dcx, dcy, dist_center, dax, day, dist_axial, "WENS"[g.NavMoveDir], "WENS"[quadrant]);
        ImDrawList* draw_list = GetForegroundDrawList(window);
        draw_list->AddRect(curr.Min, curr.Max, IM_COL32(255,200,0,100));
        draw_list->AddRect(cand.Min, cand.Max, IM_COL32(255,255,0,200));
        draw_list->AddRectFilled(cand.Max - ImVec2(4, 4), cand.Max + CalcTextSize(bu0f32) + ImVec2(4, 4), IM_COL32(40,0,0,150));
        draw_list->AddText(cand.Max, ~0U, bu0f32);
    }
    else if (g.IO.KeyCtrl) // Hold to preview score in matching quadrant. Press C to rotate.
    {
        if (quadrant == g.NavMoveDir)
        {
            ImFormatString(buf, IM_ARRAYSIZE(bu0f32), "%.0f/%.0f", dist_box, dist_center);
            ImDrawList* draw_list = GetForegroundDrawList(window);
            draw_list->AddRectFilled(cand.Min, cand.Max, IM_COL32(255, 0, 0, 200));
            draw_list->AddText(cand.Min, IM_COL32(255, 255, 255, 255), bu0f32);
        }
    }
// #endif

    // Is it in the quadrant we're interesting in moving to?
    let mut new_best: bool =  false;
    const ImGuiDir move_dir = g.NavMoveDir;
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
                if (((move_dir == ImGuiDir_Up || move_dir == ImGuiDir_Down) ? dby : dbx) < 0f32) // moving bj to the right/down decreases distance
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
        if (g.NavLayer == ImGuiNavLayer_Menu && !(g.Navwindow.Flags & ImGuiWindowFlags_ChildMenu))
            if ((move_dir == ImGuiDir_Left && dax < 0f32) || (move_dir == ImGuiDir_Right && dax > 0f32) || (move_dir == ImGuiDir_Up && day < 0f32) || (move_dir == ImGuiDir_Down && day > 0f32))
            {
                result->DistAxial = dist_axial;
                new_best = true;
            }

    return new_best;
}

static c_void ImGui::NavApplyItemToResult(ImGuiNavItemData* result)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    result->Window = window;
    result->ID = g.LastItemData.ID;
    result->FocusScopeId = window.DC.NavFocusScopeIdCurrent;
    result->InFlags = g.LastItemData.InFlags;
    result->RectRel = WindowRectAbsToRel(window, g.LastItemData.NavRect);
}

// We get there when either NavId == id, or when g.NavAnyRequest is set (which is updated by NavUpdateAnyRequestFlag above)
// This is called after LastItemData is set.
static c_void ImGui::NavProcessItem()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    const ImGuiID id = g.LastItemData.ID;
    const ImRect nav_bb = g.LastItemData.NavRect;
    const ImGuiItemFlags item_flags = g.LastItemData.InFlags;

    // Process Init Request
    if (g.NavInitRequest && g.NavLayer == window.DC.NavLayerCurrent && (item_flags & ImGuiItemFlags_Disabled) == 0)
    {
        // Even if 'ImGuiItemFlags_NoNavDefaultFocus' is on (typically collapse/close button) we record the first ResultId so they can be used as a fallback
        let candidate_for_nav_default_focus: bool = (item_flags & ImGuiItemFlags_NoNavDefaultFocus) == 0;
        if (candidate_for_nav_default_focus || g.NavInitResultId == 0)
        {
            g.NavInitResultId = id;
            g.NavInitResultRectRel = WindowRectAbsToRel(window, nav_bb);
        }
        if (candidate_for_nav_default_focus)
        {
            g.NavInitRequest = false; // Found a match, clear request
            NavUpdateAnyRequestFlag();
        }
    }

    // Process Move Request (scoring for navigation)
    // FIXME-NAV: Consider policy for double scoring (scoring from NavScoringRect + scoring from a rect wrapped according to current wrapping policy)
    if (g.NavMoveScoringItems)
    {
        let is_tab_stop: bool = (item_flags & ImGuiItemFlags_Inputable) && (item_flags & (ImGuiItemFlags_NoTabStop | ImGuiItemFlags_Disabled)) == 0;
        let is_tabbing: bool = (g.NavMoveFlags & ImGuiNavMoveFlags_Tabbing) != 0;
        if (is_tabbing)
        {
            if (is_tab_stop || (g.NavMoveFlags & ImGuiNavMoveFlags_FocusApi))
                NavProcessItemForTabbingRequest(id);
        }
        else if ((g.NavId != id || (g.NavMoveFlags & ImGuiNavMoveFlags_AllowCurrentNavId)) && !(item_flags & (ImGuiItemFlags_Disabled | ImGuiItemFlags_NoNav)))
        {
            ImGuiNavItemData* result = (window == g.NavWindow) ? &g.NavMoveResultLocal : &g.NavMoveResultOther;
            if (!is_tabbing)
            {
                if (NavScoreItem(result))
                    NavApplyItemToResult(result);

                // Features like PageUp/PageDown need to maintain a separate score for the visible set of items.
                const c_float VISIBLE_RATIO = 0.70f32;
                if ((g.NavMoveFlags & ImGuiNavMoveFlags_AlsoScoreVisibleSet) && window.ClipRect.Overlaps(nav_bb))
                    if (ImClamp(nav_bb.Max.y, window.ClipRect.Min.y, window.ClipRect.Max.y) - ImClamp(nav_bb.Min.y, window.ClipRect.Min.y, window.ClipRect.Max.y) >= (nav_bb.Max.y - nav_bb.Min.y) * VISIBLE_RATIO)
                        if (NavScoreItem(&g.NavMoveResultLocalVisible))
                            NavApplyItemToResult(&g.NavMoveResultLocalVisible);
            }
        }
    }

    // Update window-relative bounding box of navigated item
    if (g.NavId == id)
    {
        if (g.NavWindow != window)
            SetNavWindow(window); // Always refresh g.NavWindow, because some operations such as FocusItem() may not have a window.
        g.NavLayer = window.DC.NavLayerCurrent;
        g.NavFocusScopeId = window.DC.NavFocusScopeIdCurrent;
        g.NavIdIsAlive = true;
        window.NavRectRel[window.DC.NavLayerCurrent] = WindowRectAbsToRel(window, nav_bb);    // Store item bounding box (relative to window position)
    }
}

// Handle "scoring" of an item for a tabbing/focusing request initiated by NavUpdateCreateTabbingRequest().
// Note that SetKeyboardFocusHere() API calls are considered tabbing requests!
// - Case 1: no nav/active id:    set result to first eligible item, stop storing.
// - Case 2: tab forward:         on ref id set counter, on counter elapse store result
// - Case 3: tab forward wrap:    set result to first eligible item (preemptively), on ref id set counter, on next frame if counter hasn't elapsed store result. // FIXME-TABBING: Could be done as a next-frame forwarded request
// - Case 4: tab backward:        store all results, on ref id pick prev, stop storing
// - Case 5: tab backward wrap:   store all results, on ref id if no result keep storing until last // FIXME-TABBING: Could be done as next-frame forwarded requested
c_void ImGui::NavProcessItemForTabbingRequest(ImGuiID id)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    // Always store in NavMoveResultLocal (unlike directional request which uses NavMoveResultOther on sibling/flattened windows)
    ImGuiNavItemData* result = &g.NavMoveResultLocal;
    if (g.NavTabbingDir == +1)
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

bool ImGui::NavMoveRequestButNoResultYet()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.NavMoveScoringItems && g.NavMoveResultLocal.ID == 0 && g.NavMoveResultOther.ID == 0;
}

// FIXME: ScoringRect is not set
c_void ImGui::NavMoveRequestSubmit(ImGuiDir move_dir, ImGuiDir clip_dir, ImGuiNavMoveFlags move_flags, ImGuiScrollFlags scroll_flags)
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

c_void ImGui::NavMoveRequestResolveWithLastItem(ImGuiNavItemData* result)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NavMoveScoringItems = false; // Ensure request doesn't need more processing
    NavApplyItemToResult(result);
    NavUpdateAnyRequestFlag();
}

c_void ImGui::NavMoveRequestCancel()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NavMoveSubmitted = g.NavMoveScoringItems = false;
    NavUpdateAnyRequestFlag();
}

// Forward will reuse the move request again on the next frame (generally with modifications done to it)
c_void ImGui::NavMoveRequestForward(ImGuiDir move_dir, ImGuiDir clip_dir, ImGuiNavMoveFlags move_flags, ImGuiScrollFlags scroll_flags)
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
c_void ImGui::NavMoveRequestTryWrapping(ImGuiWindow* window, ImGuiNavMoveFlags wrap_flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(wrap_flags != 0); // Call with _WrapX, _WrapY, _LoopX, _LoopY
    // In theory we should test for NavMoveRequestButNoResultYet() but there's no point doing it, NavEndFrame() will do the same test
    if (g.NavWindow == window && g.NavMoveScoringItems && g.NavLayer == ImGuiNavLayer_Main)
        g.NavMoveFlags |= wrap_flags;
}

// FIXME: This could be replaced by updating a frame number in each window when (window == NavWindow) and (NavLayer == 0).
// This way we could find the last focused window among our children. It would be much less confusing this way?
static c_void ImGui::NavSaveLastChildNavWindowIntoParent(ImGuiWindow* nav_window)
{
    ImGuiWindow* parent = nav_window;
    while (parent && parent->RootWindow != parent && (parent->Flags & (ImGuiWindowFlags_Popup | ImGuiWindowFlags_ChildMenu)) == 0)
        parent = parent->ParentWindow;
    if (parent && parent != nav_window)
        parent->NavLastChildNavWindow = nav_window;
}

// Restore the last focused child.
// Call when we are expected to land on the Main Layer (0) after FocusWindow()
static ImGuiWindow* ImGui::NavRestoreLastChildNavWindow(ImGuiWindow* window)
{
    if (window.NavLastChildNavWindow && window.NavLastChildNavwindow.WasActive)
        return window.NavLastChildNavWindow;
    if (window.DockNodeAsHost && window.DockNodeAsHost->TabBar)
        if (ImGuiTabItem* tab = TabBarFindMostRecentlySelectedTabForActiveWindow(window.DockNodeAsHost->TabBar))
            return tab->Window;
    return window;
}

c_void ImGui::NavRestoreLayer(ImGuiNavLayer layer)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (layer == ImGuiNavLayer_Main)
    {
        ImGuiWindow* prev_nav_window = g.NavWindow;
        g.NavWindow = NavRestoreLastChildNavWindow(g.NavWindow);    // FIXME-NAV: Should clear ongoing nav requests?
        if (prev_nav_window)
            IMGUI_DEBUG_LOG_FOCUS("[focus] NavRestoreLayer: from \"%s\" to SetNavWindow(\"%s\")\n", prev_nav_window.Name, g.Navwindow.Name);
    }
    ImGuiWindow* window = g.NavWindow;
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

c_void ImGui::NavRestoreHighlightAfterMove()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NavDisableHighlight = false;
    g.NavDisableMouseHover = g.NavMousePosDirty = true;
}

static inline c_void ImGui::NavUpdateAnyRequestFlag()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.NavAnyRequest = g.NavMoveScoringItems || g.NavInitRequest || (IMGUI_DEBUG_NAV_SCORING && g.NavWindow != NULL);
    if (g.NavAnyRequest)
        // IM_ASSERT(g.NavWindow != NULL);
}

// This needs to be called before we submit any widget (aka in or before Begin)
c_void ImGui::NavInitWindow(ImGuiWindow* window, bool force_reinit)
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

static ImVec2 ImGui::NavCalcPreferredRefPos()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.NavWindow;
    if (g.NavDisableHighlight || !g.NavDisableMouseHover || !window)
    {
        // Mouse (we need a fallback in case the mouse becomes invalid after being used)
        // The +1f32 offset when stored by OpenPopupEx() allows reopening this or another popup (same or another mouse button) while not moving the mouse, it is pretty standard.
        // In theory we could move that +1f32 offset in OpenPopupEx()
        ImVec2 p = IsMousePosValid(&g.IO.MousePos) ? g.IO.MousePos : g.MouseLastValidPos;
        return ImVec2(p.x + 1f32, p.y);
    }
    else
    {
        // When navigation is active and mouse is disabled, pick a position around the bottom left of the currently navigated item
        // Take account of upcoming scrolling (maybe set mouse pos should be done in EndFrame?)
        ImRect rect_rel = WindowRectRelToAbs(window, window.NavRectRel[g.NavLayer]);
        if (window.LastFrameActive != g.FrameCount && (window.ScrollTarget.x != f32::MAX || window.ScrollTarget.y != f32::MAX))
        {
            ImVec2 next_scroll = CalcNextScrollFromScrollTargetAndClamp(window);
            rect_rel.Translate(window.Scroll - next_scroll);
        }
        ImVec2 pos = ImVec2(rect_rel.Min.x + ImMin(g.Style.FramePadding.x * 4, rect_rel.GetWidth()), rect_rel.Max.y - ImMin(g.Style.FramePadding.y, rect_rel.GetHeight()));
        ImGuiViewport* viewport = window.Viewport;
        return ImFloor(ImClamp(pos, viewport->Pos, viewport->Pos + viewport->Size)); // ImFloor() is important because non-integer mouse position application in backend might be lossy and result in undesirable non-zero delta.
    }
}

c_float ImGui::GetNavTweakPressedAmount(ImGuiAxis axis)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    c_float repeat_delay, repeat_rate;
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
    c_float amount = GetKeyPressedAmount(key_more, repeat_delay, repeat_rate) - GetKeyPressedAmount(key_less, repeat_delay, repeat_rate);
    if (amount != 0f32 && IsKeyDown(key_less) && IsKeyDown(key_more)) // Cancel when opposite directions are held, regardless of repeat phase
        amount = 0f32;
    return amount;
}

static c_void ImGui::NavUpdate()
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
    if (g.NavWindow && g.Navwindow.NavLastChildNavWindow != NULL && g.NavLayer == ImGuiNavLayer_Main)
        g.Navwindow.NavLastChildNavWindow = None;

    // Update CTRL+TAB and Windowing features (hold Square to move/resize/etc.)
    NavUpdateWindowing();

    // Set output flags for user application
    io.NavActive = (nav_keyboard_active || nav_gamepad_active) && g.NavWindow && !(g.Navwindow.Flags & ImGuiWindowFlags_NoNavInputs);
    io.NavVisible = (io.NavActive && g.NavId != 0 && !g.NavDisableHighlight) || (g.NavWindowingTarget != NULL);

    // Process NavCancel input (to close a popup, get back to parent, clear focus)
    NavUpdateCancelRequest();

    // Process manual activation request
    g.NavActivateId = g.NavActivateDownId = g.NavActivatePressedId = g.NavActivateInputId = 0;
    g.NavActivateFlags = ImGuiActivateFlags_None;
    if (g.NavId != 0 && !g.NavDisableHighlight && !g.NavWindowingTarget && g.NavWindow && !(g.Navwindow.Flags & ImGuiWindowFlags_NoNavInputs))
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
    if (g.NavWindow && (g.Navwindow.Flags & ImGuiWindowFlags_NoNavInputs))
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
    if (g.NavWindow && !(g.Navwindow.Flags & ImGuiWindowFlags_NoNavInputs) && !g.NavWindowingTarget)
    {
        // *Fallback* manual-scroll with Nav directional keys when window has no navigable item
        ImGuiWindow* window = g.NavWindow;
        const c_float scroll_speed = IM_ROUND(window.CalcFontSize() * 100 * io.DeltaTime); // We need round the scrolling speed because sub-pixel scroll isn't reliably supported.
        const ImGuiDir move_dir = g.NavMoveDir;
        if (window.DC.NavLayersActiveMask == 0x00 && window.DC.NavHasScroll && move_dir != ImGuiDir_None)
        {
            if (move_dir == ImGuiDir_Left || move_dir == ImGuiDir_Right)
                SetScrollX(window, ImFloor(window.Scroll.x + ((move_dir == ImGuiDir_Left) ? -1f32 : +1f32) * scroll_speed));
            if (move_dir == ImGuiDir_Up || move_dir == ImGuiDir_Down)
                SetScrollY(window, ImFloor(window.Scroll.y + ((move_dir == ImGuiDir_Up) ? -1f32 : +1f32) * scroll_speed));
        }

        // *Normal* Manual scroll with LStick
        // Next movement request will clamp the NavId reference rectangle to the visible area, so navigation will resume within those bounds.
        if (nav_gamepad_active)
        {
            const ImVec2 scroll_dir = GetKeyVector2d(ImGuiKey_GamepadLStickLeft, ImGuiKey_GamepadLStickRight, ImGuiKey_GamepadLStickUp, ImGuiKey_GamepadLStickDown);
            const c_float tweak_factor = IsKeyDown(ImGuiKey_NavGamepadTweakSlow) ? 1f32 / 10f32 : IsKeyDown(ImGuiKey_NavGamepadTweakFast) ? 10f32 : 1f32;
            if (scroll_dir.x != 0f32 && window.ScrollbarX)
                SetScrollX(window, ImFloor(window.Scroll.x + scroll_dir.x * scroll_speed * tweak_factor));
            if (scroll_dir.y != 0f32)
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
        ImDrawList* draw_list = GetForegroundDrawList(g.NavWindow);
        if (1) { for (c_int layer = 0; layer < 2; layer++) { ImRect r = WindowRectRelToAbs(g.NavWindow, g.Navwindow.NavRectRel[layer]); draw_list->AddRect(r.Min, r.Max, IM_COL32(255,200,0,255)); } } // [DEBUG]
        if (1) { u32 col = (!g.Navwindow.Hidden) ? IM_COL32(255,0,255,255) : IM_COL32(255,0,0,255); ImVec2 p = NavCalcPreferredRefPos(); buf: [c_char;32]; ImFormatString(buf, 32, "%d", g.NavLayer); draw_list->AddCircleFilled(p, 3.0f32, col); draw_list->AddText(NULL, 13.0f32, p + ImVec2(8,-4), col, bu0f32); }
    }
// #endif
}

c_void ImGui::NavInitRequestApplyResult()
{
    // In very rare cases g.NavWindow may be null (e.g. clearing focus after requesting an init request, which does happen when releasing Alt while clicking on void)
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (!g.NavWindow)
        return;

    // Apply result from previous navigation init request (will typically select the first item, unless SetItemDefaultFocus() has been called)
    // FIXME-NAV: On _NavFlattened windows, g.NavWindow will only be updated during subsequent frame. Not a problem currently.
    IMGUI_DEBUG_LOG_NAV("[nav] NavInitRequest: ApplyResult: NavID 0x%08X in Layer %d Window \"%s\"\n", g.NavInitResultId, g.NavLayer, g.Navwindow.Name);
    SetNavID(g.NavInitResultId, g.NavLayer, 0, g.NavInitResultRectRel);
    g.NavIdIsAlive = true; // Mark as alive from previous frame as we got a result
    if (g.NavInitRequestFromMove)
        NavRestoreHighlightAfterMove();
}

c_void ImGui::NavUpdateCreateMoveRequest()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiIO& io = g.IO;
    ImGuiWindow* window = g.NavWindow;
    let nav_gamepad_active: bool = (io.ConfigFlags & ImGuiConfigFlags_NavEnableGamepad) != 0 && (io.BackendFlags & ImGuiBackendFlags_HasGamepad) != 0;
    let nav_keyboard_active: bool = (io.ConfigFlags & ImGuiConfigFlags_NavEnableKeyboard) != 0;

    if (g.NavMoveForwardToNextFrame && window != NULL)
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
        g.NavScoringNoClipRect = ImRect(+f32::MAX, +f32::MAX, -f32::MAX, -f32::MAX);
    }

    // Update PageUp/PageDown/Home/End scroll
    // FIXME-NAV: Consider enabling those keys even without the master ImGuiConfigFlags_NavEnableKeyboard flag?
    c_float scoring_rect_offset_y = 0f32;
    if (window && g.NavMoveDir == ImGuiDir_None && nav_keyboard_active)
        scoring_rect_offset_y = NavUpdatePageUpPageDown();
    if (scoring_rect_offset_y != 0f32)
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
    if (g.NavMoveSubmitted && g.NavInputSource == ImGuiInputSource_Gamepad && g.NavLayer == ImGuiNavLayer_Main && window != NULL)// && (g.NavMoveFlags & ImGuiNavMoveFlags_Forwarded))
    {
        let mut clamp_x: bool =  (g.NavMoveFlags & (ImGuiNavMoveFlags_LoopX | ImGuiNavMoveFlags_WrapX)) == 0;
        let mut clamp_y: bool =  (g.NavMoveFlags & (ImGuiNavMoveFlags_LoopY | ImGuiNavMoveFlags_WrapY)) == 0;
        ImRect inner_rect_rel = WindowRectAbsToRel(window, ImRect(window.InnerRect.Min - ImVec2(1, 1), window.InnerRect.Max + ImVec2(1, 1)));
        if ((clamp_x || clamp_y) && !inner_rect_rel.Contains(window.NavRectRel[g.NavLayer]))
        {
            //IMGUI_DEBUG_LOG_NAV("[nav] NavMoveRequest: clamp NavRectRel for gamepad move\n");
            c_float pad_x = ImMin(inner_rect_rel.GetWidth(), window.CalcFontSize() * 0.5f32);
            c_float pad_y = ImMin(inner_rect_rel.GetHeight(), window.CalcFontSize() * 0.5f32); // Terrible approximation for the intent of starting navigation from first fully visible item
            inner_rect_rel.Min.x = clamp_x ? (inner_rect_rel.Min.x + pad_x) : -f32::MAX;
            inner_rect_rel.Max.x = clamp_x ? (inner_rect_rel.Max.x - pad_x) : +f32::MAX;
            inner_rect_rel.Min.y = clamp_y ? (inner_rect_rel.Min.y + pad_y) : -f32::MAX;
            inner_rect_rel.Max.y = clamp_y ? (inner_rect_rel.Max.y - pad_y) : +f32::MAX;
            window.NavRectRel[g.NavLayer].ClipWithFull(inner_rect_rel);
            g.NavId = g.NavFocusScopeId = 0;
        }
    }

    // For scoring we use a single segment on the left side our current item bounding box (not touching the edge to avoid box overlap with zero-spaced items)
    ImRect scoring_rect;
    if (window != NULL)
    {
        ImRect nav_rect_rel = !window.NavRectRel[g.NavLayer].IsInverted() ? window.NavRectRel[g.NavLayer] : ImRect(0, 0, 0, 0);
        scoring_rect = WindowRectRelToAbs(window, nav_rect_rel);
        scoring_rect.TranslateY(scoring_rect_offset_y);
        scoring_rect.Min.x = ImMin(scoring_rect.Min.x + 1f32, scoring_rect.Max.x);
        scoring_rect.Max.x = scoring_rect.Min.x;
        // IM_ASSERT(!scoring_rect.IsInverted()); // Ensure if we have a finite, non-inverted bounding box here will allows us to remove extraneous ImFabs() calls in NavScoreItem().
        //GetForegroundDrawList()->AddRect(scoring_rect.Min, scoring_rect.Max, IM_COL32(255,200,0,255)); // [DEBUG]
        //if (!g.NavScoringNoClipRect.IsInverted()) { GetForegroundDrawList()->AddRect(g.NavScoringNoClipRect.Min, g.NavScoringNoClipRect.Max, IM_COL32(255, 200, 0, 255)); } // [DEBUG]
    }
    g.NavScoringRect = scoring_rect;
    g.NavScoringNoClipRect.Add(scoring_rect);
}

c_void ImGui::NavUpdateCreateTabbingRequest()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.NavWindow;
    // IM_ASSERT(g.NavMoveDir == ImGuiDir_None);
    if (window == NULL || g.NavWindowingTarget != NULL || (window.Flags & ImGuiWindowFlags_NoNavInputs))
        return;

    let tab_pressed: bool = IsKeyPressed(ImGuiKey_Tab, true) && !IsActiveIdUsingKey(ImGuiKey_Tab) && !g.IO.KeyCtrl && !g.IO.KeyAlt;
    if (!tab_pressed)
        return;

    // Initiate tabbing request
    // (this is ALWAYS ENABLED, regardless of ImGuiConfigFlags_NavEnableKeyboard flag!)
    // Initially this was designed to use counters and modulo arithmetic, but that could not work with unsubmitted items (list clipper). Instead we use a strategy close to other move requests.
    // See NavProcessItemForTabbingRequest() for a description of the various forward/backward tabbing cases with and without wrapping.
    //// FIXME: We use (g.ActiveId == 0) but (g.NavDisableHighlight == false) might be righter once we can tab through anything
    g.NavTabbingDir = g.IO.KeyShift ? -1 : (g.ActiveId == 0) ? 0 : +1;
    ImGuiScrollFlags scroll_flags = window.Appearing ? ImGuiScrollFlags_KeepVisibleEdgeX | ImGuiScrollFlags_AlwaysCenterY : ImGuiScrollFlags_KeepVisibleEdgeX | ImGuiScrollFlags_KeepVisibleEdgeY;
    ImGuiDir clip_dir = (g.NavTabbingDir < 0) ? ImGuiDir_Up : ImGuiDir_Down;
    NavMoveRequestSubmit(ImGuiDir_None, clip_dir, ImGuiNavMoveFlags_Tabbing, scroll_flags); // FIXME-NAV: Once we refactor tabbing, add LegacyApi flag to not activate non-inputable.
    g.NavTabbingCounter = -1;
}

// Apply result from previous frame navigation directional move request. Always called from NavUpdate()
c_void ImGui::NavMoveRequestApplyResult()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
// #if IMGUI_DEBUG_NAV_SCORING
    if (g.NavMoveFlags & ImGuiNavMoveFlags_DebugNoResult) // [DEBUG] Scoring all items in NavWindow at all times
        return;
// #endif

    // Select which result to use
    ImGuiNavItemData* result = (g.NavMoveResultLocal.ID != 0) ? &g.NavMoveResultLocal : (g.NavMoveResultOther.ID != 0) ? &g.NavMoveResultOther : NULL;

    // Tabbing forward wrap
    if (g.NavMoveFlags & ImGuiNavMoveFlags_Tabbing)
        if ((g.NavTabbingCounter == 1 || g.NavTabbingDir == 0) && g.NavTabbingResultFirst.ID)
            result = &g.NavTabbingResultFirst;

    // In a situation when there is no results but NavId != 0, re-enable the Navigation highlight (because g.NavId is not considered as a possible result)
    if (result == NULL)
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
            c_float scroll_target = (g.NavMoveDir == ImGuiDir_Up) ? result->window.ScrollMax.y : 0f32;
            SetScrollY(result->Window, scroll_target);
        }
        else
        {
            ImRect rect_abs = WindowRectRelToAbs(result->Window, result->RectRel);
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
    IMGUI_DEBUG_LOG_NAV("[nav] NavMoveRequest: result NavID 0x%08X in Layer %d Window \"%s\"\n", result->ID, g.NavLayer, g.Navwindow.Name);
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
static c_void ImGui::NavUpdateCancelRequest()
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
    else if (g.NavWindow && g.NavWindow != g.Navwindow.RootWindow && !(g.Navwindow.Flags & ImGuiWindowFlags_Popup) && g.Navwindow.ParentWindow)
    {
        // Exit child window
        ImGuiWindow* child_window = g.NavWindow;
        ImGuiWindow* parent_window = g.Navwindow.ParentWindow;
        // IM_ASSERT(child_window.ChildId != 0);
        ImRect child_rect = child_window.Rect();
        FocusWindow(parent_window);
        SetNavID(child_window.ChildId, ImGuiNavLayer_Main, 0, WindowRectAbsToRel(parent_window, child_rect));
        NavRestoreHighlightAfterMove();
    }
    else if (g.OpenPopupStack.Size > 0 && !(g.OpenPopupStack.back().window.Flags & ImGuiWindowFlags_Modal))
    {
        // Close open popup/menu
        ClosePopupToLevel(g.OpenPopupStack.Size - 1, true);
    }
    else
    {
        // Clear NavLastId for popups but keep it for regular child window so we can leave one and come back where we were
        if (g.NavWindow && ((g.Navwindow.Flags & ImGuiWindowFlags_Popup) || !(g.Navwindow.Flags & ImGuiWindowFlags_ChildWindow)))
            g.Navwindow.NavLastIds[0] = 0;
        g.NavId = g.NavFocusScopeId = 0;
    }
}

// Handle PageUp/PageDown/Home/End keys
// Called from NavUpdateCreateMoveRequest() which will use our output to create a move request
// FIXME-NAV: This doesn't work properly with NavFlattened siblings as we use NavWindow rectangle for reference
// FIXME-NAV: how to get Home/End to aim at the beginning/end of a 2D grid?
static c_float ImGui::NavUpdatePageUpPageDown()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.NavWindow;
    if ((window.Flags & ImGuiWindowFlags_NoNavInputs) || g.NavWindowingTarget != NULL)
        return 0f32;

    let page_up_held: bool = IsKeyDown(ImGuiKey_PageUp) && !IsActiveIdUsingKey(ImGuiKey_PageUp);
    let page_down_held: bool = IsKeyDown(ImGuiKey_PageDown) && !IsActiveIdUsingKey(ImGuiKey_PageDown);
    let home_pressed: bool = IsKeyPressed(ImGuiKey_Home) && !IsActiveIdUsingKey(ImGuiKey_Home);
    let end_pressed: bool = IsKeyPressed(ImGuiKey_End) && !IsActiveIdUsingKey(ImGuiKey_End);
    if (page_up_held == page_down_held && home_pressed == end_pressed) // Proceed if either (not both) are pressed, otherwise early out
        return 0f32;

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
            SetScrollY(window, 0f32);
        else if (end_pressed)
            SetScrollY(window, window.ScrollMax.y);
    }
    else
    {
        ImRect& nav_rect_rel = window.NavRectRel[g.NavLayer];
        const c_float page_offset_y = ImMax(0f32, window.InnerRect.GetHeight() - window.CalcFontSize() * 1f32 + nav_rect_rel.GetHeight());
        c_float nav_scoring_rect_offset_y = 0f32;
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
            nav_rect_rel.Min.y = nav_rect_rel.Max.y = 0f32;
            if (nav_rect_rel.IsInverted())
                nav_rect_rel.Min.x = nav_rect_rel.Max.x = 0f32;
            g.NavMoveDir = ImGuiDir_Down;
            g.NavMoveFlags = ImGuiNavMoveFlags_AllowCurrentNavId | ImGuiNavMoveFlags_ScrollToEdgeY;
            // FIXME-NAV: MoveClipDir left to _None, intentional?
        }
        else if (end_pressed)
        {
            nav_rect_rel.Min.y = nav_rect_rel.Max.y = window.ContentSize.y;
            if (nav_rect_rel.IsInverted())
                nav_rect_rel.Min.x = nav_rect_rel.Max.x = 0f32;
            g.NavMoveDir = ImGuiDir_Up;
            g.NavMoveFlags = ImGuiNavMoveFlags_AllowCurrentNavId | ImGuiNavMoveFlags_ScrollToEdgeY;
            // FIXME-NAV: MoveClipDir left to _None, intentional?
        }
        return nav_scoring_rect_offset_y;
    }
    return 0f32;
}

static c_void ImGui::NavEndFrame()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    // Show CTRL+TAB list window
    if (g.NavWindowingTarget != NULL)
        NavUpdateWindowingOverlay();

    // Perform wrap-around in menus
    // FIXME-NAV: Wrap may need to apply a weight bias on the other axis. e.g. 4x4 grid with 2 last items missing on last item won't handle LoopY/WrapY correctly.
    // FIXME-NAV: Wrap (not Loop) support could be handled by the scoring function and then WrapX would function without an extra frame.
    const ImGuiNavMoveFlags wanted_flags = ImGuiNavMoveFlags_WrapX | ImGuiNavMoveFlags_LoopX | ImGuiNavMoveFlags_WrapY | ImGuiNavMoveFlags_LoopY;
    if (g.NavWindow && NavMoveRequestButNoResultYet() && (g.NavMoveFlags & wanted_flags) && (g.NavMoveFlags & ImGuiNavMoveFlags_Forwarded) == 0)
        NavUpdateCreateWrappingRequest();
}

static c_void ImGui::NavUpdateCreateWrappingRequest()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.NavWindow;

    let mut do_forward: bool =  false;
    ImRect bb_rel = window.NavRectRel[g.NavLayer];
    ImGuiDir clip_dir = g.NavMoveDir;
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

static c_int ImGui::FindWindowFocusIndex(ImGuiWindow* window)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    IM_UNUSED(g);
    c_int order = window.FocusOrder;
    // IM_ASSERT(window.RootWindow == window); // No child window (not testing _ChildWindow because of docking)
    // IM_ASSERT(g.WindowsFocusOrder[order] == window);
    return order;
}

static ImGuiWindow* FindWindowNavFocusable(c_int i_start, c_int i_stop, c_int dir) // FIXME-OPT O(N)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    for (c_int i = i_start; i >= 0 && i < g.WindowsFocusOrder.Size && i != i_stop; i += dir)
        if (ImGui::IsWindowNavFocusable(g.WindowsFocusOrder[i]))
            return g.WindowsFocusOrder[i];
    return NULL;
}

static c_void NavUpdateWindowingHighlightWindow(c_int focus_change_dir)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.NavWindowingTarget);
    if (g.NavWindowingTarget->Flags & ImGuiWindowFlags_Modal)
        return;

    let i_current: c_int = ImGui::FindWindowFocusIndex(g.NavWindowingTarget);
    ImGuiWindow* window_target = FindWindowNavFocusable(i_current + focus_change_dir, -INT_MAX, focus_change_dir);
    if (!window_target)
        window_target = FindWindowNavFocusable((focus_change_dir < 0) ? (g.WindowsFocusOrder.Size - 1) : 0, i_current, focus_change_dir);
    if (window_target) // Don't reset windowing target if there's a single window in the list
    {
        g.NavWindowingTarget = g.NavWindowingTargetAnim = window_target;
        g.NavWindowingAccumDeltaPos = g.NavWindowingAccumDeltaSize = ImVec2(0f32, 0f32);
    }
    g.NavWindowingToggleLayer = false;
}

// Windowing management mode
// Keyboard: CTRL+Tab (change focus/move/resize), Alt (toggle menu layer)
// Gamepad:  Hold Menu/Square (change focus/move/resize), Tap Menu/Square (toggle menu layer)
static c_void ImGui::NavUpdateWindowing()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiIO& io = g.IO;

    ImGuiWindow* apply_focus_window = None;
    let mut apply_toggle_layer: bool =  false;

    ImGuiWindow* modal_window = GetTopMostPopupModal();
    let mut allow_windowing: bool =  (modal_window == NULL);
    if (!allow_windowing)
        g.NavWindowingTarget = None;

    // Fade out
    if (g.NavWindowingTargetAnim && g.NavWindowingTarget == NULL)
    {
        g.NavWindowingHighlightAlpha = ImMax(g.NavWindowingHighlightAlpha - io.DeltaTime * 10f32, 0f32);
        if (g.DimBgRatio <= 0f32 && g.NavWindowingHighlightAlpha <= 0f32)
            g.NavWindowingTargetAnim = None;
    }

    // Start CTRL+Tab or Square+L/R window selection
    let nav_gamepad_active: bool = (io.ConfigFlags & ImGuiConfigFlags_NavEnableGamepad) != 0 && (io.BackendFlags & ImGuiBackendFlags_HasGamepad) != 0;
    let nav_keyboard_active: bool = (io.ConfigFlags & ImGuiConfigFlags_NavEnableKeyboard) != 0;
    let start_windowing_with_gamepad: bool = allow_windowing && nav_gamepad_active && !g.NavWindowingTarget && IsKeyPressed(ImGuiKey_NavGamepadMenu, false);
    let start_windowing_with_keyboard: bool = allow_windowing && !g.NavWindowingTarget && io.KeyCtrl && IsKeyPressed(ImGuiKey_Tab, false); // Note: enabled even without NavEnableKeyboard!
    if (start_windowing_with_gamepad || start_windowing_with_keyboard)
        if (ImGuiWindow* window = g.NavWindow ? g.NavWindow : FindWindowNavFocusable(g.WindowsFocusOrder.Size - 1, -INT_MAX, -1))
        {
            g.NavWindowingTarget = g.NavWindowingTargetAnim = window.RootWindow;
            g.NavWindowingTimer = g.NavWindowingHighlightAlpha = 0f32;
            g.NavWindowingAccumDeltaPos = g.NavWindowingAccumDeltaSize = ImVec2(0f32, 0f32);
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
            g.NavWindowingHighlightAlpha = 1f32;
        }

        // Single press toggles NavLayer, long press with L/R apply actual focus on release (until then the window was merely rendered top-most)
        if (!IsKeyDown(ImGuiKey_NavGamepadMenu))
        {
            g.NavWindowingToggleLayer &= (g.NavWindowingHighlightAlpha < 1f32); // Once button was held long enough we don't consider it a tap-to-toggle-layer press anymore.
            if (g.NavWindowingToggleLayer && g.NavWindow)
                apply_toggle_layer = true;
            else if (!g.NavWindowingToggleLayer)
                apply_focus_window = g.NavWindowingTarget;
            g.NavWindowingTarget = None;
        }
    }

    // Keyboard: Focus
    if (g.NavWindowingTarget && g.NavInputSource == ImGuiInputSource_Keyboard)
    {
        // Visuals only appears after a brief time after pressing TAB the first time, so that a fast CTRL+TAB doesn't add visual noise
        g.NavWindowingHighlightAlpha = ImMax(g.NavWindowingHighlightAlpha, ImSaturate((g.NavWindowingTimer - NAV_WINDOWING_HIGHLIGHT_DELAY) / 0.05f32)); // 1.0f
        if (IsKeyPressed(ImGuiKey_Tab, true))
            NavUpdateWindowingHighlightWindow(io.KeyShift ? +1 : -1);
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
        ImVec2 nav_move_dir;
        if (g.NavInputSource == ImGuiInputSource_Keyboard && !io.KeyShift)
            nav_move_dir = GetKeyVector2d(ImGuiKey_LeftArrow, ImGuiKey_RightArrow, ImGuiKey_UpArrow, ImGuiKey_DownArrow);
        if (g.NavInputSource == ImGuiInputSource_Gamepad)
            nav_move_dir = GetKeyVector2d(ImGuiKey_GamepadLStickLeft, ImGuiKey_GamepadLStickRight, ImGuiKey_GamepadLStickUp, ImGuiKey_GamepadLStickDown);
        if (nav_move_dir.x != 0f32 || nav_move_dir.y != 0f32)
        {
            const c_float NAV_MOVE_SPEED = 800f32;
            const c_float move_step = NAV_MOVE_SPEED * io.DeltaTime * ImMin(io.DisplayFramebufferScale.x, io.DisplayFramebufferScale.y);
            g.NavWindowingAccumDeltaPos += nav_move_dir * move_step;
            g.NavDisableMouseHover = true;
            ImVec2 accum_floored = ImFloor(g.NavWindowingAccumDeltaPos);
            if (accum_floored.x != 0f32 || accum_floored.y != 0f32)
            {
                ImGuiWindow* moving_window = g.NavWindowingTarget->RootWindowDockTree;
                SetWindowPos(moving_window, moving_window.Pos + accum_floored, ImGuiCond_Always);
                g.NavWindowingAccumDeltaPos -= accum_floored;
            }
        }
    }

    // Apply final focus
    if (apply_focus_window && (g.NavWindow == NULL || apply_focus_window != g.Navwindow.RootWindow))
    {
        ImGuiViewport* previous_viewport = g.NavWindow ? g.Navwindow.Viewport : NULL;
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
        g.NavWindowingTarget = None;

    // Apply menu/layer toggle
    if (apply_toggle_layer && g.NavWindow)
    {
        ClearActiveID();

        // Move to parent menu if necessary
        ImGuiWindow* new_nav_window = g.NavWindow;
        while (new_nav_window.ParentWindow
            && (new_nav_window.DC.NavLayersActiveMask & (1 << ImGuiNavLayer_Menu)) == 0
            && (new_nav_window.Flags & ImGuiWindowFlags_ChildWindow) != 0
            && (new_nav_window.Flags & (ImGuiWindowFlags_Popup | ImGuiWindowFlags_ChildMenu)) == 0)
            new_nav_window = new_nav_window.ParentWindow;
        if (new_nav_window != g.NavWindow)
        {
            ImGuiWindow* old_nav_window = g.NavWindow;
            FocusWindow(new_nav_window);
            new_nav_window.NavLastChildNavWindow = old_nav_window;
        }

        // Toggle layer
        const ImGuiNavLayer new_nav_layer = (g.Navwindow.DC.NavLayersActiveMask & (1 << ImGuiNavLayer_Menu)) ? (ImGuiNavLayer)(g.NavLayer ^ 1) : ImGuiNavLayer_Main;
        if (new_nav_layer != g.NavLayer)
        {
            // Reinitialize navigation when entering menu bar with the Alt key (FIXME: could be a properly of the layer?)
            let preserve_layer_1_nav_id: bool = (new_nav_window.DockNodeAsHost != NULL);
            if (new_nav_layer == ImGuiNavLayer_Menu && !preserve_layer_1_nav_id)
                g.Navwindow.NavLastIds[new_nav_layer] = 0;
            NavRestoreLayer(new_nav_layer);
            NavRestoreHighlightAfterMove();
        }
    }
}

// Window has already passed the IsWindowNavFocusable()
static *const char GetFallbackWindowNameForWindowingList(ImGuiWindow* window)
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
c_void ImGui::NavUpdateWindowingOverlay()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.NavWindowingTarget != NULL);

    if (g.NavWindowingTimer < NAV_WINDOWING_LIST_APPEAR_DELAY)
        return;

    if (g.NavWindowingListWindow == NULL)
        g.NavWindowingListWindow = FindWindowByName("###NavWindowingList");
    *const ImGuiViewport viewport = /*g.NavWindow ? g.Navwindow.Viewport :*/ GetMainViewport();
    SetNextWindowSizeConstraints(ImVec2(viewport->Size.x * 0.20f32, viewport->Size.y * 0.200f32), ImVec2(f32::MAX, f32::MAX));
    SetNextWindowPos(viewport->GetCenter(), ImGuiCond_Always, ImVec2(0.5f32, 0.5f32));
    PushStyleVar(ImGuiStyleVar_WindowPadding, g.Style.WindowPadding * 2.00f32);
    Begin("###NavWindowingList", NULL, ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoFocusOnAppearing | ImGuiWindowFlags_NoResize | ImGuiWindowFlags_NoMove | ImGuiWindowFlags_NoInputs | ImGuiWindowFlags_AlwaysAutoResize | ImGuiWindowFlags_NoSavedSettings);
    for (c_int n = g.WindowsFocusOrder.Size - 1; n >= 0; n--)
    {
        ImGuiWindow* window = g.WindowsFocusOrder[n];
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

bool ImGui::IsDragDropActive()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.DragDropActive;
}

c_void ImGui::ClearDragDrop()
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
bool ImGui::BeginDragDropSource(ImGuiDragDropFlags flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;

    // FIXME-DRAGDROP: While in the common-most "drag from non-zero active id" case we can tell the mouse button,
    // in both SourceExtern and id==0 cases we may requires something else (explicit flags or some heuristic).
    ImGuiMouseButton mouse_button = ImGuiMouseButton_Left;

    let mut source_drag_active: bool =  false;
    ImGuiID source_id = 0;
    ImGuiID source_parent_id = 0;
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
        source_parent_id = window.IDStack.back();
        source_drag_active = IsMouseDragging(mouse_button);

        // Disable navigation and key inputs while dragging + cancel existing request if any
        SetActiveIdUsingAllKeyboardKeys();
    }
    else
    {
        window = None;
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
                ImGuiWindow* tooltip_window = g.CurrentWindow;
                tooltip_window.Hidden = tooltip_window.SkipItems = true;
                tooltip_window.HiddenFramesCanSkipItems = 1;
            }
        }

        if (!(flags & ImGuiDragDropFlags_SourceNoDisableHover) && !(flags & ImGuiDragDropFlags_SourceExtern))
            g.LastItemData.StatusFlags &= ~ImGuiItemStatusFlags_HoveredRect;

        return true;
    }
    return false;
}

c_void ImGui::EndDragDropSource()
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
bool ImGui::SetDragDropPayload(*const char type, *const c_void data, size_t data_size, ImGuiCond cond)
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
        g.DragDropPayloadBufHeap.resize(0);
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
            payload.Data = None;
        }
        payload.DataSize = data_size;
    }
    payload.DataFrameCount = g.FrameCount;

    // Return whether the payload has been accepted
    return (g.DragDropAcceptFrameCount == g.FrameCount) || (g.DragDropAcceptFrameCount == g.FrameCount - 1);
}

bool ImGui::BeginDragDropTargetCustom(const ImRect& bb, ImGuiID id)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (!g.DragDropActive)
        return false;

    let mut window = g.CurrentWindow;
    ImGuiWindow* hovered_window = g.HoveredWindowUnderMovingWindow;
    if (hovered_window == NULL || window.RootWindowDockTree != hovered_window.RootWindowDockTree)
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
bool ImGui::BeginDragDropTarget()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (!g.DragDropActive)
        return false;

    let mut window = g.CurrentWindow;
    if (!(g.LastItemData.StatusFlags & ImGuiItemStatusFlags_HoveredRect))
        return false;
    ImGuiWindow* hovered_window = g.HoveredWindowUnderMovingWindow;
    if (hovered_window == NULL || window.RootWindowDockTree != hovered_window.RootWindowDockTree || window.SkipItems)
        return false;

    const ImRect& display_rect = (g.LastItemData.StatusFlags & ImGuiItemStatusFlags_HasDisplayRect) ? g.LastItemData.DisplayRect : g.LastItemData.Rect;
    ImGuiID id = g.LastItemData.ID;
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

bool ImGui::IsDragDropPayloadBeingAccepted()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.DragDropActive && g.DragDropAcceptIdPrev != 0;
}

*const ImGuiPayload ImGui::AcceptDragDropPayload(*const char type, ImGuiDragDropFlags flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    ImGuiPayload& payload = g.DragDropPayload;
    // IM_ASSERT(g.DragDropActive);                        // Not called between BeginDragDropTarget() and EndDragDropTarget() ?
    // IM_ASSERT(payload.DataFrameCount != -1);            // Forgot to call EndDragDropTarget() ?
    if (type != NULL && !payload.IsDataType(type))
        return NULL;

    // Accept smallest drag target bounding box, this allows us to nest drag targets conveniently without ordering constraints.
    // NB: We currently accept NULL id as target. However, overlapping targets requires a unique ID to function!
    let was_accepted_previously: bool = (g.DragDropAcceptIdPrev == g.DragDropTargetId);
    ImRect r = g.DragDropTargetRect;
    c_float r_surface = r.GetWidth() * r.GetHeight();
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
        window.DrawList.AddRect(r.Min - ImVec2(3.5f32,3.5f32), r.Max + ImVec2(3.5f32, 3.5f32), GetColorU32(ImGuiCol_DragDropTarget), 0f32, 0, 2.00f32);

    g.DragDropAcceptFrameCount = g.FrameCount;
    payload.Delivery = was_accepted_previously && !IsMouseDown(g.DragDropMouseButton); // For extern drag sources affecting os window focus, it's easier to just test !IsMouseDown() instead of IsMouseReleased()
    if (!payload.Delivery && !(flags & ImGuiDragDropFlags_AcceptBeforeDelivery))
        return NULL;

    return &payload;
}

*const ImGuiPayload ImGui::GetDragDropPayload()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.DragDropActive ? &g.DragDropPayload : NULL;
}

// We don't really use/need this now, but added it for the sake of consistency and because we might need it later.
c_void ImGui::EndDragDropTarget()
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
static inline c_void LogTextV(ImGuiContext& g, *const char fmt, va_list args)
{
    if (g.LogFile)
    {
        g.LogBuffer.Buf.resize(0);
        g.LogBuffer.appendfv(fmt, args);
        ImFileWrite(g.LogBuffer.c_str(), sizeof, (u64)g.LogBuffer.size(), g.LogFile);
    }
    else
    {
        g.LogBuffer.appendfv(fmt, args);
    }
}

c_void ImGui::LogText(*const char fmt, ...)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (!g.LogEnabled)
        return;

    va_list args;
    va_start(args, fmt);
    LogTextV(g, fmt, args);
    va_end(args);
}

c_void ImGui::LogTextV(*const char fmt, va_list args)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (!g.LogEnabled)
        return;

    LogTextV(g, fmt, args);
}

// Start logging/capturing text output
c_void ImGui::LogBegin(ImGuiLogType type, c_int auto_open_depth)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    // IM_ASSERT(g.LogEnabled == false);
    // IM_ASSERT(g.LogFile == NULL);
    // IM_ASSERT(g.LogBuffer.empty());
    g.LogEnabled = true;
    g.LogType = type;
    g.LogNextPrefix = g.LogNextSuffix = None;
    g.LogDepthRef = window.DC.TreeDepth;
    g.LogDepthToExpand = ((auto_open_depth >= 0) ? auto_open_depth : g.LogDepthToExpandDefault);
    g.LogLinePosY = f32::MAX;
    g.LogLineFirstItem = true;
}

// Important: doesn't copy underlying data, use carefully (prefix/suffix must be in scope at the time of the next LogRenderedText)
c_void ImGui::LogSetNextTextDecoration(*const char prefix, *const char suffix)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.LogNextPrefix = prefix;
    g.LogNextSuffix = suffix;
}

c_void ImGui::LogToTTY(c_int auto_open_depth)
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
c_void ImGui::LogToFile(c_int auto_open_depth, *const char filename)
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
    if (!0f32)
    {
        // IM_ASSERT(0);
        return;
    }

    LogBegin(ImGuiLogType_File, auto_open_depth);
    g.LogFile = f;
}

// Start logging/capturing text output to clipboard
c_void ImGui::LogToClipboard(c_int auto_open_depth)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.LogEnabled)
        return;
    LogBegin(ImGuiLogType_Clipboard, auto_open_depth);
}

c_void ImGui::LogToBuffer(c_int auto_open_depth)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.LogEnabled)
        return;
    LogBegin(ImGuiLogType_Buffer, auto_open_depth);
}

c_void ImGui::LogFinish()
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
    g.LogFile = None;
    g.LogBuffer.clear();
}

// Helper to display logging buttons
// FIXME-OBSOLETE: We should probably obsolete this and let the user have their own helper (this is one of the oldest function alive!)
c_void ImGui::LogButtons()
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
    SliderInt("Default Depth", &g.LogDepthToExpandDefault, 0, 9, NULL);
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
c_void ImGui::UpdateSettings()
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
    if (g.SettingsDirtyTimer > 0f32)
    {
        g.SettingsDirtyTimer -= g.IO.DeltaTime;
        if (g.SettingsDirtyTimer <= 0f32)
        {
            if (g.IO.IniFilename != NULL)
                SaveIniSettingsToDisk(g.IO.IniFilename);
            else
                g.IO.WantSaveIniSettings = true;  // Let user know they can call SaveIniSettingsToMemory(). user will need to clear io.WantSaveIniSettings themselves.
            g.SettingsDirtyTimer = 0f32;
        }
    }
}

c_void ImGui::MarkIniSettingsDirty()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.SettingsDirtyTimer <= 0f32)
        g.SettingsDirtyTimer = g.IO.IniSavingRate;
}

c_void ImGui::MarkIniSettingsDirty(ImGuiWindow* window)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (!(window.Flags & ImGuiWindowFlags_NoSavedSettings))
        if (g.SettingsDirtyTimer <= 0f32)
            g.SettingsDirtyTimer = g.IO.IniSavingRate;
}

ImGuiWindowSettings* ImGui::CreateNewWindowSettings(*const char name)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

// #if !IMGUI_DEBUG_INI_SETTINGS
    // Skip to the "###" marker if any. We don't skip past to match the behavior of GetID()
    // Preserve the full string when IMGUI_DEBUG_INI_SETTINGS is set to make .ini inspection easier.
    if (*const char p = strstr(name, "###"))
        name = p;
// #endif
    const size_t name_len = strlen(name);

    // Allocate chunk
    const size_t chunk_size = sizeof(ImGuiWindowSettings) + name_len + 1;
    ImGuiWindowSettings* settings = g.SettingsWindows.alloc_chunk(chunk_size);
    IM_PLACEMENT_NEW(settings) ImGuiWindowSettings();
    settings->ID = ImHashStr(name, name_len);
    memcpy(settings->GetName(), name, name_len + 1);   // Store with zero terminator

    return settings;
}

ImGuiWindowSettings* ImGui::FindWindowSettings(ImGuiID id)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    for (ImGuiWindowSettings* settings = g.SettingsWindows.begin(); settings != None; settings = g.SettingsWindows.next_chunk(settings))
        if (settings->ID == id)
            return settings;
    return NULL;
}

ImGuiWindowSettings* ImGui::FindOrCreateWindowSettings(*const char name)
{
    if (ImGuiWindowSettings* settings = FindWindowSettings(ImHashStr(name)))
        return settings;
    return CreateNewWindowSettings(name);
}

c_void ImGui::AddSettingsHandler(*const ImGuiSettingsHandler handler)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(FindSettingsHandler(handler->TypeName) == NULL);
    g.SettingsHandlers.push(*handler);
}

c_void ImGui::RemoveSettingsHandler(*const char type_name)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (ImGuiSettingsHandler* handler = FindSettingsHandler(type_name))
        g.SettingsHandlers.erase(handler);
}

ImGuiSettingsHandler* ImGui::FindSettingsHandler(*const char type_name)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    const ImGuiID type_hash = ImHashStr(type_name);
    for (c_int handler_n = 0; handler_n < g.SettingsHandlers.Size; handler_n++)
        if (g.SettingsHandlers[handler_n].TypeHash == type_hash)
            return &g.SettingsHandlers[handler_n];
    return NULL;
}

c_void ImGui::ClearIniSettings()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.SettingsIniData.clear();
    for (c_int handler_n = 0; handler_n < g.SettingsHandlers.Size; handler_n++)
        if (g.SettingsHandlers[handler_n].ClearAllFn)
            g.SettingsHandlers[handler_n].ClearAllFn(&g, &g.SettingsHandlers[handler_n]);
}

c_void ImGui::LoadIniSettingsFromDisk(*const char ini_filename)
{
    size_t file_data_size = 0;
    char* file_data = (char*)ImFileLoadToMemory(ini_filename, "rb", &file_data_size);
    if (!file_data)
        return;
    if (file_data_size > 0)
        LoadIniSettingsFromMemory(file_data, file_data_size);
    IM_FREE(file_data);
}

// Zero-tolerance, no error reporting, cheap .ini parsing
c_void ImGui::LoadIniSettingsFromMemory(*const char ini_data, size_t ini_size)
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
    for (c_int handler_n = 0; handler_n < g.SettingsHandlers.Size; handler_n++)
        if (g.SettingsHandlers[handler_n].ReadInitFn)
            g.SettingsHandlers[handler_n].ReadInitFn(&g, &g.SettingsHandlers[handler_n]);

    entry_data: *mut c_void = None;
    ImGuiSettingsHandler* entry_handler = None;

    char* line_end = None;
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
            let mut  name_start: *const c_char = type_end ? ImStrchrRange(type_end + 1, name_end, '[') : NULL;
            if (!type_end || !name_start)
                continue;
            *type_end = 0; // Overwrite first ']'
            name_start+= 1;  // Skip second '['
            entry_handler = FindSettingsHandler(type_start);
            entry_data = entry_handler ? entry_handler->ReadOpenFn(&g, entry_handler, name_start) : NULL;
        }
        else if (entry_handler != NULL && entry_data != NULL)
        {
            // Let type handler parse the line
            entry_handler->ReadLineFn(&g, entry_handler, entry_data, line);
        }
    }
    g.SettingsLoaded = true;

    // [DEBUG] Restore untouched copy so it can be browsed in Metrics (not strictly necessary)
    memcpy(buf, ini_data, ini_size);

    // Call post-read handlers
    for (c_int handler_n = 0; handler_n < g.SettingsHandlers.Size; handler_n++)
        if (g.SettingsHandlers[handler_n].ApplyAllFn)
            g.SettingsHandlers[handler_n].ApplyAllFn(&g, &g.SettingsHandlers[handler_n]);
}

c_void ImGui::SaveIniSettingsToDisk(*const char ini_filename)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.SettingsDirtyTimer = 0f32;
    if (!ini_filename)
        return;

    size_t ini_data_size = 0;
    let mut  ini_data: *const c_char = SaveIniSettingsToMemory(&ini_data_size);
    ImFileHandle f = ImFileOpen(ini_filename, "wt");
    if (!0f32)
        return;
    ImFileWrite(ini_data, sizeof, ini_data_size, 0f32);
    ImFileClose(0f32);
}

// Call registered handlers (e.g. SettingsHandlerWindow_WriteAll() + custom handlers) to write their stuff into a text buffer
*const char ImGui::SaveIniSettingsToMemory(size_t* out_size)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.SettingsDirtyTimer = 0f32;
    g.SettingsIniData.Buf.resize(0);
    g.SettingsIniData.Buf.push(0);
    for (c_int handler_n = 0; handler_n < g.SettingsHandlers.Size; handler_n++)
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
    for (c_int i = 0; i != g.Windows.Size; i++)
        g.Windows[i]->SettingsOffset = -1;
    g.SettingsWindows.clear();
}

static *mut c_void WindowSettingsHandler_ReadOpen(ImGuiContext*, ImGuiSettingsHandler*, *const char name)
{
    ImGuiWindowSettings* settings = ImGui::FindOrCreateWindowSettings(name);
    ImGuiID id = settings->ID;
    *settings = ImGuiWindowSettings(); // Clear existing if recycling previous entry
    settings->ID = id;
    settings->WantApply = true;
    return (*mut c_void)settings;
}

static c_void WindowSettingsHandler_ReadLine(ImGuiContext*, ImGuiSettingsHandler*, entry: *mut c_void, *const char line)
{
    ImGuiWindowSettings* settings = (ImGuiWindowSettings*)entry;
    c_int x, y;
    c_int i;
    u32 u1;
    if (sscanf(line, "Pos=%i,%i", &x, &y) == 2)             { settings->Pos = ImVec2ih((c_short)x, (c_short)y); }
    else if (sscanf(line, "Size=%i,%i", &x, &y) == 2)       { settings->Size = ImVec2ih((c_short)x, (c_short)y); }
    else if (sscanf(line, "ViewportId=0x%08X", &u1) == 1)   { settings->ViewportId = u1; }
    else if (sscanf(line, "ViewportPos=%i,%i", &x, &y) == 2){ settings->ViewportPos = ImVec2ih((c_short)x, (c_short)y); }
    else if (sscanf(line, "Collapsed=%d", &i) == 1)         { settings->Collapsed = (i != 0); }
    else if (sscanf(line, "DockId=0x%X,%d", &u1, &i) == 2)  { settings->DockId = u1; settings->DockOrder = (c_short)i; }
    else if (sscanf(line, "DockId=0x%X", &u1) == 1)         { settings->DockId = u1; settings->DockOrder = -1; }
    else if (sscanf(line, "ClassId=0x%X", &u1) == 1)        { settings->ClassId = u1; }
}

// Apply to existing windows (if any)
static c_void WindowSettingsHandler_ApplyAll(ImGuiContext* ctx, ImGuiSettingsHandler*)
{
    ImGuiContext& g = *ctx;
    for (ImGuiWindowSettings* settings = g.SettingsWindows.begin(); settings != None; settings = g.SettingsWindows.next_chunk(settings))
        if (settings->WantApply)
        {
            if (ImGuiWindow* window = ImGui::FindWindowByID(settings->ID))
                ApplyWindowSettings(window, settings);
            settings->WantApply = false;
        }
}

static c_void WindowSettingsHandler_WriteAll(ImGuiContext* ctx, ImGuiSettingsHandler* handler, ImGuiTextBuffer* bu0f32)
{
    // Gather data from windows that were active during this session
    // (if a window wasn't opened in this session we preserve its settings)
    ImGuiContext& g = *ctx;
    for (c_int i = 0; i != g.Windows.Size; i++)
    {
        ImGuiWindow* window = g.Windows[i];
        if (window.Flags & ImGuiWindowFlags_NoSavedSettings)
            continue;

        ImGuiWindowSettings* settings = (window.SettingsOffset != -1) ? g.SettingsWindows.ptr_from_offset(window.SettingsOffset) : ImGui::FindWindowSettings(window.ID);
        if (!settings)
        {
            settings = ImGui::CreateNewWindowSettings(window.Name);
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
    for (ImGuiWindowSettings* settings = g.SettingsWindows.begin(); settings != None; settings = g.SettingsWindows.next_chunk(settings))
    {
        let mut  settings_name: *const c_char = settings->GetName();
        buf->appendf("[%s][%s]\n", handler->TypeName, settings_name);
        if (settings->ViewportId != 0 && settings->ViewportId != ImGui::IMGUI_VIEWPORT_DEFAULT_ID)
        {
            buf->appendf("ViewportPos=%d,%d\n", settings->ViewportPos.x, settings->ViewportPos.y);
            buf->appendf("ViewportId=0x%08X\n", settings->ViewportId);
        }
        if (settings->Pos.x != 0 || settings->Pos.y != 0 || settings->ViewportId == ImGui::IMGUI_VIEWPORT_DEFAULT_ID)
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

ImGuiViewport* ImGui::GetMainViewport()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.Viewports[0];
}

// FIXME: This leaks access to viewports not listed in PlatformIO.Viewports[]. Problematic? (#4236)
ImGuiViewport* ImGui::FindViewportByID(ImGuiID id)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    for (c_int n = 0; n < g.Viewports.Size; n++)
        if (g.Viewports[n]->ID == id)
            return g.Viewports[n];
    return NULL;
}

ImGuiViewport* ImGui::FindViewportByPlatformHandle(platform_handle: *mut c_void)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    for (c_int i = 0; i != g.Viewports.Size; i++)
        if (g.Viewports[i]->PlatformHandle == platform_handle)
            return g.Viewports[i];
    return NULL;
}

c_void ImGui::SetCurrentViewport(ImGuiWindow* current_window, *mut ImGuiViewportP viewport)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    (c_void)current_window;

    if (viewport)
        viewport->LastFrameActive = g.FrameCount;
    if (g.CurrentViewport == viewport)
        return;
    g.CurrentDpiScale = viewport ? viewport->DpiScale : 1f32;
    g.CurrentViewport = viewport;
    //IMGUI_DEBUG_LOG_VIEWPORT("[viewport] SetCurrentViewport changed '%s' 0x%08X\n", current_window ? current_window.Name : NULL, viewport ? viewport->ID : 0);

    // Notify platform layer of viewport changes
    // FIXME-DPI: This is only currently used for experimenting with handling of multiple DPI
    if (g.CurrentViewport && g.PlatformIO.Platform_OnChangedViewport)
        g.PlatformIO.Platform_OnChangedViewport(g.CurrentViewport);
}

c_void ImGui::SetWindowViewport(ImGuiWindow* window, *mut ImGuiViewportP viewport)
{
    // Abandon viewport
    if (window.ViewportOwned && window.Viewport->Window == window)
        window.Viewport->Size = ImVec2(0f32, 0f32);

    window.Viewport = viewport;
    window.ViewportId = viewport->ID;
    window.ViewportOwned = (viewport->Window == window);
}

static bool ImGui::GetWindowAlwaysWantOwnViewport(ImGuiWindow* window)
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

static bool ImGui::UpdateTryMergeWindowIntoHostViewport(ImGuiWindow* window, *mut ImGuiViewportP viewport)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (window.Viewport == viewport)
        return false;
    if ((viewport->Flags & ImGuiViewportFlags_CanHostOtherWindows) == 0)
        return false;
    if ((viewport->Flags & ImGuiViewportFlags_Minimized) != 0)
        return false;
    if (!viewport->GetMainRect().Contains(window.Rect()))
        return false;
    if (GetWindowAlwaysWantOwnViewport(window))
        return false;

    // FIXME: Can't use g.WindowsFocusOrder[] for root windows only as we care about Z order. If we maintained a DisplayOrder along with FocusOrder we could..
    for (c_int n = 0; n < g.Windows.Size; n++)
    {
        ImGuiWindow* window_behind = g.Windows[n];
        if (window_behind == window)
            break;
        if (window_behind->WasActive && window_behind->ViewportOwned && !(window_behind->Flags & ImGuiWindowFlags_ChildWindow))
            if (window_behind->Viewport->GetMainRect().Overlaps(window.Rect()))
                return false;
    }

    // Move to the existing viewport, Move child/hosted windows as well (FIXME-OPT: iterate child)
    *mut ImGuiViewportP old_viewport = window.Viewport;
    if (window.ViewportOwned)
        for (c_int n = 0; n < g.Windows.Size; n++)
            if (g.Windows[n]->Viewport == old_viewport)
                SetWindowViewport(g.Windows[n], viewport);
    SetWindowViewport(window, viewport);
    BringWindowToDisplayFront(window);

    return true;
}

// FIXME: handle 0 to N host viewports
static bool ImGui::UpdateTryMergeWindowIntoHostViewports(ImGuiWindow* window)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return UpdateTryMergeWindowIntoHostViewport(window, g.Viewports[0]);
}

// Translate Dear ImGui windows when a Host Viewport has been moved
// (This additionally keeps windows at the same place when ImGuiConfigFlags_ViewportsEnable is toggled!)
c_void ImGui::TranslateWindowsInViewport(*mut ImGuiViewportP viewport, const ImVec2& old_pos, const ImVec2& new_pos)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(viewport->Window == NULL && (viewport->Flags & ImGuiViewportFlags_CanHostOtherWindows));

    // 1) We test if ImGuiConfigFlags_ViewportsEnable was just toggled, which allows us to conveniently
    // translate imgui windows from OS-window-local to absolute coordinates or vice-versa.
    // 2) If it's not going to fit into the new size, keep it at same absolute position.
    // One problem with this is that most Win32 applications doesn't update their render while dragging,
    // and so the window will appear to teleport when releasing the mouse.
    let translate_all_windows: bool = (g.ConfigFlagsCurrFrame & ImGuiConfigFlags_ViewportsEnable) != (g.ConfigFlagsLastFrame & ImGuiConfigFlags_ViewportsEnable);
    ImRect test_still_fit_rect(old_pos, old_pos + viewport->Size);
    ImVec2 delta_pos = new_pos - old_pos;
    for (c_int window_n = 0; window_n < g.Windows.Size; window_n++) // FIXME-OPT
        if (translate_all_windows || (g.Windows[window_n]->Viewport == viewport && test_still_fit_rect.Contains(g.Windows[window_n]->Rect())))
            TranslateWindow(g.Windows[window_n], delta_pos);
}

// Scale all windows (position, size). Use when e.g. changing DPI. (This is a lossy operation!)
c_void ImGui::ScaleWindowsInViewport(*mut ImGuiViewportP viewport, c_float scale)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (viewport->Window)
    {
        ScaleWindow(viewport->Window, scale);
    }
    else
    {
        for (c_int i = 0; i != g.Windows.Size; i++)
            if (g.Windows[i]->Viewport == viewport)
                ScaleWindow(g.Windows[i], scale);
    }
}

// If the backend doesn't set MouseLastHoveredViewport or doesn't honor ImGuiViewportFlags_NoInputs, we do a search ourselves.
// A) It won't take account of the possibility that non-imgui windows may be in-between our dragged window and our target window.
// B) It requires Platform_GetWindowFocus to be implemented by backend.
*mut ImGuiViewportP ImGui::FindHoveredViewportFromPlatformWindowStack(const ImVec2& mouse_platform_pos)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    *mut ImGuiViewportP best_candidate = None;
    for (c_int n = 0; n < g.Viewports.Size; n++)
    {
        *mut ImGuiViewportP viewport = g.Viewports[n];
        if (!(viewport->Flags & (ImGuiViewportFlags_NoInputs | ImGuiViewportFlags_Minimized)) && viewport->GetMainRect().Contains(mouse_platform_pos))
            if (best_candidate == NULL || best_candidate->LastFrontMostStampCount < viewport->LastFrontMostStampCount)
                best_candidate = viewport;
    }
    return best_candidate;
}

// Update viewports and monitor infos
// Note that this is running even if 'ImGuiConfigFlags_ViewportsEnable' is not set, in order to clear unused viewports (if any) and update monitor info.
static c_void ImGui::UpdateViewportsNewFrame()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.PlatformIO.Viewports.Size <= g.Viewports.Size);

    // Update Minimized status (we need it first in order to decide if we'll apply Pos/Size of the main viewport)
    let viewports_enabled: bool = (g.ConfigFlagsCurrFrame & ImGuiConfigFlags_ViewportsEnable) != 0;
    if (viewports_enabled)
    {
        for (c_int n = 0; n < g.Viewports.Size; n++)
        {
            *mut ImGuiViewportP viewport = g.Viewports[n];
            let platform_funcs_available: bool = viewport->PlatformWindowCreated;
            if (g.PlatformIO.Platform_GetWindowMinimized && platform_funcs_available)
            {
                let mut minimized: bool =  g.PlatformIO.Platform_GetWindowMinimized(viewport);
                if (minimized)
                    viewport->Flags |= ImGuiViewportFlags_Minimized;
                else
                    viewport->Flags &= ~ImGuiViewportFlags_Minimized;
            }
        }
    }

    // Create/update main viewport with current platform position.
    // FIXME-VIEWPORT: Size is driven by backend/user code for backward-compatibility but we should aim to make this more consistent.
    *mut ImGuiViewportP main_viewport = g.Viewports[0];
    // IM_ASSERT(main_viewport->ID == IMGUI_VIEWPORT_DEFAULT_ID);
    // IM_ASSERT(main_viewport->Window == NULL);
    ImVec2 main_viewport_pos = viewports_enabled ? g.PlatformIO.Platform_GetWindowPos(main_viewport) : ImVec2(0f32, 0f32);
    ImVec2 main_viewport_size = g.IO.DisplaySize;
    if (viewports_enabled && (main_viewport->Flags & ImGuiViewportFlags_Minimized))
    {
        main_viewport_pos = main_viewport->Pos;    // Preserve last pos/size when minimized (FIXME: We don't do the same for Size outside of the viewport path)
        main_viewport_size = main_viewport->Size;
    }
    AddUpdateViewport(NULL, IMGUI_VIEWPORT_DEFAULT_ID, main_viewport_pos, main_viewport_size, ImGuiViewportFlags_OwnedByApp | ImGuiViewportFlags_CanHostOtherWindows);

    g.CurrentDpiScale = 0f32;
    g.CurrentViewport = None;
    g.MouseViewport = None;
    for (c_int n = 0; n < g.Viewports.Size; n++)
    {
        *mut ImGuiViewportP viewport = g.Viewports[n];
        viewport->Idx = n;

        // Erase unused viewports
        if (n > 0 && viewport->LastFrameActive < g.FrameCount - 2)
        {
            DestroyViewport(viewport);
            n-= 1;
            continue;
        }

        let platform_funcs_available: bool = viewport->PlatformWindowCreated;
        if (viewports_enabled)
        {
            // Update Position and Size (from Platform Window to ImGui) if requested.
            // We do it early in the frame instead of waiting for UpdatePlatformWindows() to avoid a frame of lag when moving/resizing using OS facilities.
            if (!(viewport->Flags & ImGuiViewportFlags_Minimized) && platform_funcs_available)
            {
                // Viewport->WorkPos and WorkSize will be updated below
                if (viewport->PlatformRequestMove)
                    viewport->Pos = viewport->LastPlatformPos = g.PlatformIO.Platform_GetWindowPos(viewport);
                if (viewport->PlatformRequestResize)
                    viewport->Size = viewport->LastPlatformSize = g.PlatformIO.Platform_GetWindowSize(viewport);
            }
        }

        // Update/copy monitor info
        UpdateViewportPlatformMonitor(viewport);

        // Lock down space taken by menu bars and status bars, reset the offset for functions like BeginMainMenuBar() to alter them again.
        viewport->WorkOffsetMin = viewport->BuildWorkOffsetMin;
        viewport->WorkOffsetMax = viewport->BuildWorkOffsetMax;
        viewport->BuildWorkOffsetMin = viewport->BuildWorkOffsetMax = ImVec2(0f32, 0f32);
        viewport->UpdateWorkRect();

        // Reset alpha every frame. Users of transparency (docking) needs to request a lower alpha back.
        viewport->Alpha = 1f32;

        // Translate Dear ImGui windows when a Host Viewport has been moved
        // (This additionally keeps windows at the same place when ImGuiConfigFlags_ViewportsEnable is toggled!)
        const ImVec2 viewport_delta_pos = viewport->Pos - viewport->LastPos;
        if ((viewport->Flags & ImGuiViewportFlags_CanHostOtherWindows) && (viewport_delta_pos.x != 0f32 || viewport_delta_pos.y != 0f32))
            TranslateWindowsInViewport(viewport, viewport->LastPos, viewport->Pos);

        // Update DPI scale
        c_float new_dpi_scale;
        if (g.PlatformIO.Platform_GetWindowDpiScale && platform_funcs_available)
            new_dpi_scale = g.PlatformIO.Platform_GetWindowDpiScale(viewport);
        else if (viewport->PlatformMonitor != -1)
            new_dpi_scale = g.PlatformIO.Monitors[viewport->PlatformMonitor].DpiScale;
        else
            new_dpi_scale = (viewport->DpiScale != 0f32) ? viewport->DpiScale : 1f32;
        if (viewport->DpiScale != 0f32 && new_dpi_scale != viewport->DpiScale)
        {
            c_float scale_factor = new_dpi_scale / viewport->DpiScale;
            if (g.IO.ConfigFlags & ImGuiConfigFlags_DpiEnableScaleViewports)
                ScaleWindowsInViewport(viewport, scale_factor);
            //if (viewport == GetMainViewport())
            //    g.PlatformInterface.SetWindowSize(viewport, viewport->Size * scale_factor);

            // Scale our window moving pivot so that the window will rescale roughly around the mouse position.
            // FIXME-VIEWPORT: This currently creates a resizing feedback loop when a window is straddling a DPI transition border.
            // (Minor: since our sizes do not perfectly linearly scale, deferring the click offset scale until we know the actual window scale ratio may get us slightly more precise mouse positioning.)
            //if (g.MovingWindow != NULL && g.Movingwindow.Viewport == viewport)
            //    g.ActiveIdClickOffset = ImFloor(g.ActiveIdClickOffset * scale_factor);
        }
        viewport->DpiScale = new_dpi_scale;
    }

    // Update fallback monitor
    if (g.PlatformIO.Monitors.Size == 0)
    {
        ImGuiPlatformMonitor* monitor = &g.FallbackMonitor;
        monitor->MainPos = main_viewport->Pos;
        monitor->MainSize = main_viewport->Size;
        monitor->WorkPos = main_viewport->WorkPos;
        monitor->WorkSize = main_viewport->WorkSize;
        monitor->DpiScale = main_viewport->DpiScale;
    }

    if (!viewports_enabled)
    {
        g.MouseViewport = main_viewport;
        return;
    }

    // Mouse handling: decide on the actual mouse viewport for this frame between the active/focused viewport and the hovered viewport.
    // Note that 'viewport_hovered' should skip over any viewport that has the ImGuiViewportFlags_NoInputs flags set.
    *mut ImGuiViewportP viewport_hovered = None;
    if (g.IO.BackendFlags & ImGuiBackendFlags_HasMouseHoveredViewport)
    {
        viewport_hovered = g.IO.MouseHoveredViewport ? (*mut ImGuiViewportP)FindViewportByID(g.IO.MouseHoveredViewport) : NULL;
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
    if (viewport_hovered != NULL)
        g.MouseLastHoveredViewport = viewport_hovered;
    else if (g.MouseLastHoveredViewport == NULL)
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
    if (is_mouse_dragging_with_an_expected_destination && viewport_hovered == NULL)
        viewport_hovered = g.MouseLastHoveredViewport;
    if (is_mouse_dragging_with_an_expected_destination || g.ActiveId == 0 || !IsAnyMouseDown())
        if (viewport_hovered != NULL && viewport_hovered != g.MouseViewport && !(viewport_hovered->Flags & ImGuiViewportFlags_NoInputs))
            g.MouseViewport = viewport_hovered;

    // IM_ASSERT(g.MouseViewport != NULL);
}

// Update user-facing viewport list (g.Viewports -> g.PlatformIO.Viewports after filtering out some)
static c_void ImGui::UpdateViewportsEndFrame()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.PlatformIO.Viewports.resize(0);
    for (c_int i = 0; i < g.Viewports.Size; i++)
    {
        *mut ImGuiViewportP viewport = g.Viewports[i];
        viewport->LastPos = viewport->Pos;
        if (viewport->LastFrameActive < g.FrameCount || viewport->Size.x <= 0f32 || viewport->Size.y <= 0f32)
            if (i > 0) // Always include main viewport in the list
                continue;
        if (viewport->Window && !IsWindowActiveAndVisible(viewport->Window))
            continue;
        if (i > 0)
            // IM_ASSERT(viewport->Window != NULL);
        g.PlatformIO.Viewports.push(viewport);
    }
    g.Viewports[0]->ClearRequestFlags(); // Clear main viewport flags because UpdatePlatformWindows() won't do it and may not even be called
}

// FIXME: We should ideally refactor the system to call this every frame (we currently don't)
*mut ImGuiViewportP ImGui::AddUpdateViewport(ImGuiWindow* window, ImGuiID id, const ImVec2& pos, const ImVec2& size, ImGuiViewportFlags flags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(id != 0);

    flags |= ImGuiViewportFlags_IsPlatformWindow;
    if (window != NULL)
    {
        if (g.MovingWindow && g.Movingwindow.RootWindowDockTree == window)
            flags |= ImGuiViewportFlags_NoInputs | ImGuiViewportFlags_NoFocusOnAppearing;
        if ((window.Flags & ImGuiWindowFlags_NoMouseInputs) && (window.Flags & ImGuiWindowFlags_NoNavInputs))
            flags |= ImGuiViewportFlags_NoInputs;
        if (window.Flags & ImGuiWindowFlags_NoFocusOnAppearing)
            flags |= ImGuiViewportFlags_NoFocusOnAppearing;
    }

    *mut ImGuiViewportP viewport = (*mut ImGuiViewportP)FindViewportByID(id);
    if (viewport)
    {
        // Always update for main viewport as we are already pulling correct platform pos/size (see #4900)
        if (!viewport->PlatformRequestMove || viewport->ID == IMGUI_VIEWPORT_DEFAULT_ID)
            viewport->Pos = pos;
        if (!viewport->PlatformRequestResize || viewport->ID == IMGUI_VIEWPORT_DEFAULT_ID)
            viewport->Size = size;
        viewport->Flags = flags | (viewport->Flags & ImGuiViewportFlags_Minimized); // Preserve existing flags
    }
    else
    {
        // New viewport
        viewport = IM_NEW(ImGuiViewportP)();
        viewport->ID = id;
        viewport->Idx = g.Viewports.Size;
        viewport->Pos = viewport->LastPos = pos;
        viewport->Size = size;
        viewport->Flags = flags;
        UpdateViewportPlatformMonitor(viewport);
        g.Viewports.push(viewport);
        IMGUI_DEBUG_LOG_VIEWPORT("[viewport] Add Viewport %08X '%s'\n", id, window ? window.Name : "<NULL>");

        // We normally setup for all viewports in NewFrame() but here need to handle the mid-frame creation of a new viewport.
        // We need to extend the fullscreen clip rect so the OverlayDrawList clip is correct for that the first frame
        g.DrawListSharedData.ClipRectFullscreen.x = ImMin(g.DrawListSharedData.ClipRectFullscreen.x, viewport->Pos.x);
        g.DrawListSharedData.ClipRectFullscreen.y = ImMin(g.DrawListSharedData.ClipRectFullscreen.y, viewport->Pos.y);
        g.DrawListSharedData.ClipRectFullscreen.z = ImMax(g.DrawListSharedData.ClipRectFullscreen.z, viewport->Pos.x + viewport->Size.x);
        g.DrawListSharedData.ClipRectFullscreen.w = ImMax(g.DrawListSharedData.ClipRectFullscreen.w, viewport->Pos.y + viewport->Size.y);

        // Store initial DpiScale before the OS platform window creation, based on expected monitor data.
        // This is so we can select an appropriate font size on the first frame of our window lifetime
        if (viewport->PlatformMonitor != -1)
            viewport->DpiScale = g.PlatformIO.Monitors[viewport->PlatformMonitor].DpiScale;
    }

    viewport->Window = window;
    viewport->LastFrameActive = g.FrameCount;
    viewport->UpdateWorkRect();
    // IM_ASSERT(window == NULL || viewport->ID == window.ID);

    if (window != NULL)
        window.ViewportOwned = true;

    return viewport;
}

static c_void ImGui::DestroyViewport(*mut ImGuiViewportP viewport)
{
    // Clear references to this viewport in windows (window.ViewportId becomes the master data)
    let g = GImGui; // ImGuiContext& g = *GImGui;
    for (c_int window_n = 0; window_n < g.Windows.Size; window_n++)
    {
        ImGuiWindow* window = g.Windows[window_n];
        if (window.Viewport != viewport)
            continue;
        window.Viewport = None;
        window.ViewportOwned = false;
    }
    if (viewport == g.MouseLastHoveredViewport)
        g.MouseLastHoveredViewport = None;

    // Destroy
    IMGUI_DEBUG_LOG_VIEWPORT("[viewport] Delete Viewport %08X '%s'\n", viewport->ID, viewport->Window ? viewport->window.Name : "n/a");
    DestroyPlatformWindow(viewport); // In most circumstances the platform window will already be destroyed here.
    // IM_ASSERT(g.PlatformIO.Viewports.contains(viewport) == false);
    // IM_ASSERT(g.Viewports[viewport->Idx] == viewport);
    g.Viewports.erase(g.Viewports.Data + viewport->Idx);
    IM_DELETE(viewport);
}

// FIXME-VIEWPORT: This is all super messy and ought to be clarified or rewritten.
static c_void ImGui::WindowSelectViewport(ImGuiWindow* window)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiWindowFlags flags = window.Flags;
    window.ViewportAllowPlatformMonitorExtend = -1;

    // Restore main viewport if multi-viewport is not supported by the backend
    *mut ImGuiViewportP main_viewport = (*mut ImGuiViewportP)(*mut c_void)GetMainViewport();
    if (!(g.ConfigFlagsCurrFrame & ImGuiConfigFlags_ViewportsEnable))
    {
        SetWindowViewport(window, main_viewport);
        return;
    }
    window.ViewportOwned = false;

    // Appearing popups reset their viewport so they can inherit again
    if ((flags & (ImGuiWindowFlags_Popup | ImGuiWindowFlags_Tooltip)) && window.Appearing)
    {
        window.Viewport = None;
        window.ViewportId = 0;
    }

    if ((g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasViewport) == 0)
    {
        // By default inherit from parent window
        if (window.Viewport == NULL && window.ParentWindow && (!window.Parentwindow.IsFallbackWindow || window.Parentwindow.WasActive))
            window.Viewport = window.Parentwindow.Viewport;

        // Attempt to restore saved viewport id (= window that hasn't been activated yet), try to restore the viewport based on saved 'window.ViewportPos' restored from .ini file
        if (window.Viewport == NULL && window.ViewportId != 0)
        {
            window.Viewport = (*mut ImGuiViewportP)FindViewportByID(window.ViewportId);
            if (window.Viewport == NULL && window.ViewportPos.x != f32::MAX && window.ViewportPos.y != f32::MAX)
                window.Viewport = AddUpdateViewport(window, window.ID, window.ViewportPos, window.Size, ImGuiViewportFlags_None);
        }
    }

    let mut lock_viewport: bool =  false;
    if (g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasViewport)
    {
        // Code explicitly request a viewport
        window.Viewport = (*mut ImGuiViewportP)FindViewportByID(g.NextWindowData.ViewportId);
        window.ViewportId = g.NextWindowData.ViewportId; // Store ID even if Viewport isn't resolved yet.
        lock_viewport = true;
    }
    else if ((flags & ImGuiWindowFlags_ChildWindow) || (flags & ImGuiWindowFlags_ChildMenu))
    {
        // Always inherit viewport from parent window
        if (window.DockNode && window.DockNode->HostWindow)
            // IM_ASSERT(window.DockNode->Hostwindow.Viewport == window.Parentwindow.Viewport);
        window.Viewport = window.Parentwindow.Viewport;
    }
    else if (window.DockNode && window.DockNode->HostWindow)
    {
        // This covers the "always inherit viewport from parent window" case for when a window reattach to a node that was just created mid-frame
        window.Viewport = window.DockNode->Hostwindow.Viewport;
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
        if (window.Viewport != NULL && window.Viewport->Window == window)
            window.Viewport = AddUpdateViewport(window, window.ID, window.Pos, window.Size, ImGuiViewportFlags_None);
    }
    else
    {
        // Merge into host viewport?
        // We cannot test window.ViewportOwned as it set lower in the function.
        // Testing (g.ActiveId == 0 || g.ActiveIdAllowOverlap) to avoid merging during a short-term widget interaction. Main intent was to avoid during resize (see #4212)
        let mut try_to_merge_into_host_viewport: bool =  (window.Viewport && window == window.Viewport->Window && (g.ActiveId == 0 || g.ActiveIdAllowOverlap));
        if (try_to_merge_into_host_viewport)
            UpdateTryMergeWindowIntoHostViewports(window);
    }

    // Fallback: merge in default viewport if z-order matches, otherwise create a new viewport
    if (window.Viewport == NULL)
        if (!UpdateTryMergeWindowIntoHostViewport(window, main_viewport))
            window.Viewport = AddUpdateViewport(window, window.ID, window.Pos, window.Size, ImGuiViewportFlags_None);

    // Mark window as allowed to protrude outside of its viewport and into the current monitor
    if (!lock_viewport)
    {
        if (flags & (ImGuiWindowFlags_Tooltip | ImGuiWindowFlags_Popup))
        {
            // We need to take account of the possibility that mouse may become invalid.
            // Popups/Tooltip always set ViewportAllowPlatformMonitorExtend so GetWindowAllowedExtentRect() will return full monitor bounds.
            ImVec2 mouse_ref = (flags & ImGuiWindowFlags_Tooltip) ? g.IO.MousePos : g.BeginPopupStack.back().OpenMousePos;
            let mut use_mouse_ref: bool =  (g.NavDisableHighlight || !g.NavDisableMouseHover || !g.NavWindow);
            let mut mouse_valid: bool =  IsMousePosValid(&mouse_re0f32);
            if ((window.Appearing || (flags & (ImGuiWindowFlags_Tooltip | ImGuiWindowFlags_ChildMenu))) && (!use_mouse_ref || mouse_valid))
                window.ViewportAllowPlatformMonitorExtend = FindPlatformMonitorForPos((use_mouse_ref && mouse_valid) ? mouse_ref : NavCalcPreferredRefPos());
            else
                window.ViewportAllowPlatformMonitorExtend = window.Viewport->PlatformMonitor;
        }
        else if (window.Viewport && window != window.Viewport->Window && window.Viewport->Window && !(flags & ImGuiWindowFlags_ChildWindow) && window.DockNode == NULL)
        {
            // When called from Begin() we don't have access to a proper version of the Hidden flag yet, so we replicate this code.
            let will_be_visible: bool = (window.DockIsActive && !window.DockTabIsVisible) ? false : true;
            if ((window.Flags & ImGuiWindowFlags_DockNodeHost) && window.Viewport->LastFrameActive < g.FrameCount && will_be_visible)
            {
                // Steal/transfer ownership
                IMGUI_DEBUG_LOG_VIEWPORT("[viewport] Window '%s' steal Viewport %08X from Window '%s'\n", window.Name, window.Viewport->ID, window.Viewport->window.Name);
                window.Viewport->Window = window;
                window.Viewport->ID = window.ID;
                window.Viewport->LastNameHash = 0;
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
            window.ViewportAllowPlatformMonitorExtend = window.Viewport->PlatformMonitor;
        }
    }

    // Update flags
    window.ViewportOwned = (window == window.Viewport->Window);
    window.ViewportId = window.Viewport->ID;

    // If the OS window has a title bar, hide our imgui title bar
    //if (window.ViewportOwned && !(window.Viewport->Flags & ImGuiViewportFlags_NoDecoration))
    //    window.Flags |= ImGuiWindowFlags_NoTitleBar;
}

c_void ImGui::WindowSyncOwnedViewport(ImGuiWindow* window, ImGuiWindow* parent_window_in_stack)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    let mut viewport_rect_changed: bool =  false;

    // Synchronize window --> viewport in most situations
    // Synchronize viewport -> window in case the platform window has been moved or resized from the OS/WM
    if (window.Viewport->PlatformRequestMove)
    {
        window.Pos = window.Viewport->Pos;
        MarkIniSettingsDirty(window);
    }
    else if (memcmp(&window.Viewport->Pos, &window.Pos, sizeof(window.Pos)) != 0)
    {
        viewport_rect_changed = true;
        window.Viewport->Pos = window.Pos;
    }

    if (window.Viewport->PlatformRequestResize)
    {
        window.Size = window.SizeFull = window.Viewport->Size;
        MarkIniSettingsDirty(window);
    }
    else if (memcmp(&window.Viewport->Size, &window.Size, sizeof(window.Size)) != 0)
    {
        viewport_rect_changed = true;
        window.Viewport->Size = window.Size;
    }
    window.Viewport->UpdateWorkRect();

    // The viewport may have changed monitor since the global update in UpdateViewportsNewFrame()
    // Either a SetNextWindowPos() call in the current frame or a SetWindowPos() call in the previous frame may have this effect.
    if (viewport_rect_changed)
        UpdateViewportPlatformMonitor(window.Viewport);

    // Update common viewport flags
    const ImGuiViewportFlags viewport_flags_to_clear = ImGuiViewportFlags_TopMost | ImGuiViewportFlags_NoTaskBarIcon | ImGuiViewportFlags_NoDecoration | ImGuiViewportFlags_NoRendererClear;
    ImGuiViewportFlags viewport_flags = window.Viewport->Flags & ~viewport_flags_to_clear;
    ImGuiWindowFlags window_flags = window.Flags;
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
        viewport_flags &= ~window.WindowClass.ViewportFlagsOverrideClear;

    // We can also tell the backend that clearing the platform window won't be necessary,
    // as our window background is filling the viewport and we have disabled BgAlpha.
    // FIXME: Work on support for per-viewport transparency (#2766)
    if (!(window_flags & ImGuiWindowFlags_NoBackground))
        viewport_flags |= ImGuiViewportFlags_NoRendererClear;

    window.Viewport->Flags = viewport_flags;

    // Update parent viewport ID
    // (the !IsFallbackWindow test mimic the one done in WindowSelectViewport())
    if (window.WindowClass.ParentViewportId != (ImGuiID)-1)
        window.Viewport->ParentViewportId = window.WindowClass.ParentViewportId;
    else if ((window_flags & (ImGuiWindowFlags_Popup | ImGuiWindowFlags_Tooltip)) && parent_window_in_stack && (!parent_window_in_stack->IsFallbackWindow || parent_window_in_stack->WasActive))
        window.Viewport->ParentViewportId = parent_window_in_stack->Viewport->ID;
    else
        window.Viewport->ParentViewportId = g.IO.ConfigViewportsNoDefaultParent ? 0 : IMGUI_VIEWPORT_DEFAULT_ID;
}

// Called by user at the end of the main loop, after EndFrame()
// This will handle the creation/update of all OS windows via function defined in the ImGuiPlatformIO api.
c_void ImGui::UpdatePlatformWindows()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.FrameCountEnded == g.FrameCount && "Forgot to call Render() or EndFrame() before UpdatePlatformWindows()?");
    // IM_ASSERT(g.FrameCountPlatformEnded < g.FrameCount);
    g.FrameCountPlatformEnded = g.FrameCount;
    if (!(g.ConfigFlagsCurrFrame & ImGuiConfigFlags_ViewportsEnable))
        return;

    // Create/resize/destroy platform windows to match each active viewport.
    // Skip the main viewport (index 0), which is always fully handled by the application!
    for (c_int i = 1; i < g.Viewports.Size; i++)
    {
        *mut ImGuiViewportP viewport = g.Viewports[i];

        // Destroy platform window if the viewport hasn't been submitted or if it is hosting a hidden window
        // (the implicit/fallback Debug##Default window will be registering its viewport then be disabled, causing a dummy DestroyPlatformWindow to be made each frame)
        let mut destroy_platform_window: bool =  false;
        destroy_platform_window |= (viewport->LastFrameActive < g.FrameCount - 1);
        destroy_platform_window |= (viewport->Window && !IsWindowActiveAndVisible(viewport->Window));
        if (destroy_platform_window)
        {
            DestroyPlatformWindow(viewport);
            continue;
        }

        // New windows that appears directly in a new viewport won't always have a size on their first frame
        if (viewport->LastFrameActive < g.FrameCount || viewport->Size.x <= 0 || viewport->Size.y <= 0)
            continue;

        // Create window
        let mut is_new_platform_window: bool =  (viewport->PlatformWindowCreated == false);
        if (is_new_platform_window)
        {
            IMGUI_DEBUG_LOG_VIEWPORT("[viewport] Create Platform Window %08X '%s'\n", viewport->ID, viewport->Window ? viewport->window.Name : "n/a");
            g.PlatformIO.Platform_CreateWindow(viewport);
            if (g.PlatformIO.Renderer_CreateWindow != NULL)
                g.PlatformIO.Renderer_CreateWindow(viewport);
            viewport->LastNameHash = 0;
            viewport->LastPlatformPos = viewport->LastPlatformSize = ImVec2(f32::MAX, f32::MAX); // By clearing those we'll enforce a call to Platform_SetWindowPos/Size below, before Platform_ShowWindow (FIXME: Is that necessary?)
            viewport->LastRendererSize = viewport->Size;                                       // We don't need to call Renderer_SetWindowSize() as it is expected Renderer_CreateWindow() already did it.
            viewport->PlatformWindowCreated = true;
        }

        // Apply Position and Size (from ImGui to Platform/Renderer backends)
        if ((viewport->LastPlatformPos.x != viewport->Pos.x || viewport->LastPlatformPos.y != viewport->Pos.y) && !viewport->PlatformRequestMove)
            g.PlatformIO.Platform_SetWindowPos(viewport, viewport->Pos);
        if ((viewport->LastPlatformSize.x != viewport->Size.x || viewport->LastPlatformSize.y != viewport->Size.y) && !viewport->PlatformRequestResize)
            g.PlatformIO.Platform_SetWindowSize(viewport, viewport->Size);
        if ((viewport->LastRendererSize.x != viewport->Size.x || viewport->LastRendererSize.y != viewport->Size.y) && g.PlatformIO.Renderer_SetWindowSize)
            g.PlatformIO.Renderer_SetWindowSize(viewport, viewport->Size);
        viewport->LastPlatformPos = viewport->Pos;
        viewport->LastPlatformSize = viewport->LastRendererSize = viewport->Size;

        // Update title bar (if it changed)
        if (ImGuiWindow* window_for_title = GetWindowForTitleDisplay(viewport->Window))
        {
            let mut  title_begin: *const c_char = window_for_title->Name;
            char* title_end = (char*)(intptr_t)FindRenderedTextEnd(title_begin);
            const ImGuiID title_hash = ImHashStr(title_begin, title_end - title_begin);
            if (viewport->LastNameHash != title_hash)
            {
                char title_end_backup_c = *title_end;
                *title_end = 0; // Cut existing buffer short instead of doing an alloc/free, no small gain.
                g.PlatformIO.Platform_SetWindowTitle(viewport, title_begin);
                *title_end = title_end_backup_c;
                viewport->LastNameHash = title_hash;
            }
        }

        // Update alpha (if it changed)
        if (viewport->LastAlpha != viewport->Alpha && g.PlatformIO.Platform_SetWindowAlpha)
            g.PlatformIO.Platform_SetWindowAlpha(viewport, viewport->Alpha);
        viewport->LastAlpha = viewport->Alpha;

        // Optional, general purpose call to allow the backend to perform general book-keeping even if things haven't changed.
        if (g.PlatformIO.Platform_UpdateWindow)
            g.PlatformIO.Platform_UpdateWindow(viewport);

        if (is_new_platform_window)
        {
            // On startup ensure new platform window don't steal focus (give it a few frames, as nested contents may lead to viewport being created a few frames late)
            if (g.FrameCount < 3)
                viewport->Flags |= ImGuiViewportFlags_NoFocusOnAppearing;

            // Show window
            g.PlatformIO.Platform_ShowWindow(viewport);

            // Even without focus, we assume the window becomes front-most.
            // This is useful for our platform z-order heuristic when io.MouseHoveredViewport is not available.
            if (viewport->LastFrontMostStampCount != g.ViewportFrontMostStampCount)
                viewport->LastFrontMostStampCount = ++g.ViewportFrontMostStampCount;
            }

        // Clear request flags
        viewport->ClearRequestFlags();
    }

    // Update our implicit z-order knowledge of platform windows, which is used when the backend cannot provide io.MouseHoveredViewport.
    // When setting Platform_GetWindowFocus, it is expected that the platform backend can handle calls without crashing if it doesn't have data stored.
    // FIXME-VIEWPORT: We should use this information to also set dear imgui-side focus, allowing us to handle os-level alt+tab.
    if (g.PlatformIO.Platform_GetWindowFocus != NULL)
    {
        *mut ImGuiViewportP focused_viewport = None;
        for (c_int n = 0; n < g.Viewports.Size && focused_viewport == None; n++)
        {
            *mut ImGuiViewportP viewport = g.Viewports[n];
            if (viewport->PlatformWindowCreated)
                if (g.PlatformIO.Platform_GetWindowFocus(viewport))
                    focused_viewport = viewport;
        }

        // Store a tag so we can infer z-order easily from all our windows
        // We compare PlatformLastFocusedViewportId so newly created viewports with _NoFocusOnAppearing flag
        // will keep the front most stamp instead of losing it back to their parent viewport.
        if (focused_viewport && g.PlatformLastFocusedViewportId != focused_viewport->ID)
        {
            if (focused_viewport->LastFrontMostStampCount != g.ViewportFrontMostStampCount)
                focused_viewport->LastFrontMostStampCount = ++g.ViewportFrontMostStampCount;
            g.PlatformLastFocusedViewportId = focused_viewport->ID;
        }
    }
}

// This is a default/basic function for performing the rendering/swap of multiple Platform Windows.
// Custom renderers may prefer to not call this function at all, and instead iterate the publicly exposed platform data and handle rendering/sync themselves.
// The Render/Swap functions stored in ImGuiPlatformIO are merely here to allow for this helper to exist, but you can do it yourself:
//
//    ImGuiPlatformIO& platform_io = ImGui::GetPlatformIO();
//    for (int i = 1; i < platform_io.Viewports.Size; i++)
//        if ((platform_io.Viewports[i]->Flags & ImGuiViewportFlags_Minimized) == 0)
//            MyRenderFunction(platform_io.Viewports[i], my_args);
//    for (int i = 1; i < platform_io.Viewports.Size; i++)
//        if ((platform_io.Viewports[i]->Flags & ImGuiViewportFlags_Minimized) == 0)
//            MySwapBufferFunction(platform_io.Viewports[i], my_args);
//
c_void ImGui::RenderPlatformWindowsDefault(platform_render_arg: *mut c_void, renderer_render_arg: *mut c_void)
{
    // Skip the main viewport (index 0), which is always fully handled by the application!
    ImGuiPlatformIO& platform_io = ImGui::GetPlatformIO();
    for (c_int i = 1; i < platform_io.Viewports.Size; i++)
    {
        ImGuiViewport* viewport = platform_io.Viewports[i];
        if (viewport->Flags & ImGuiViewportFlags_Minimized)
            continue;
        if (platform_io.Platform_RenderWindow) platform_io.Platform_RenderWindow(viewport, platform_render_arg);
        if (platform_io.Renderer_RenderWindow) platform_io.Renderer_RenderWindow(viewport, renderer_render_arg);
    }
    for (c_int i = 1; i < platform_io.Viewports.Size; i++)
    {
        ImGuiViewport* viewport = platform_io.Viewports[i];
        if (viewport->Flags & ImGuiViewportFlags_Minimized)
            continue;
        if (platform_io.Platform_SwapBuffers) platform_io.Platform_SwapBuffers(viewport, platform_render_arg);
        if (platform_io.Renderer_SwapBuffers) platform_io.Renderer_SwapBuffers(viewport, renderer_render_arg);
    }
}

static c_int ImGui::FindPlatformMonitorForPos(const ImVec2& pos)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    for (c_int monitor_n = 0; monitor_n < g.PlatformIO.Monitors.Size; monitor_n++)
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
static c_int ImGui::FindPlatformMonitorForRect(const ImRect& rect)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    let monitor_count: c_int = g.PlatformIO.Monitors.Size;
    if (monitor_count <= 1)
        return monitor_count - 1;

    // Use a minimum threshold of 1f32 so a zero-sized rect won't false positive, and will still find the correct monitor given its position.
    // This is necessary for tooltips which always resize down to zero at first.
    const c_float surface_threshold = ImMax(rect.GetWidth() * rect.GetHeight() * 0.5f32, 1f32);
    c_int best_monitor_n = -1;
    c_float best_monitor_surface = 0.001f;

    for (c_int monitor_n = 0; monitor_n < g.PlatformIO.Monitors.Size && best_monitor_surface < surface_threshold; monitor_n++)
    {
        const ImGuiPlatformMonitor& monitor = g.PlatformIO.Monitors[monitor_n];
        const ImRect monitor_rect = ImRect(monitor.MainPos, monitor.MainPos + monitor.MainSize);
        if (monitor_rect.Contains(rect))
            return monitor_n;
        ImRect overlapping_rect = rect;
        overlapping_rect.ClipWithFull(monitor_rect);
        c_float overlapping_surface = overlapping_rect.GetWidth() * overlapping_rect.GetHeight();
        if (overlapping_surface < best_monitor_surface)
            continue;
        best_monitor_surface = overlapping_surface;
        best_monitor_n = monitor_n;
    }
    return best_monitor_n;
}

// Update monitor from viewport rectangle (we'll use this info to clamp windows and save windows lost in a removed monitor)
static c_void ImGui::UpdateViewportPlatformMonitor(*mut ImGuiViewportP viewport)
{
    viewport->PlatformMonitor = (c_short)FindPlatformMonitorForRect(viewport->GetMainRect());
}

// Return value is always != NULL, but don't hold on it across frames.
*const ImGuiPlatformMonitor ImGui::GetViewportPlatformMonitor(ImGuiViewport* viewport_p)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    *mut ImGuiViewportP viewport = (*mut ImGuiViewportP)(*mut c_void)viewport_p;
    c_int monitor_idx = viewport->PlatformMonitor;
    if (monitor_idx >= 0 && monitor_idx < g.PlatformIO.Monitors.Size)
        return &g.PlatformIO.Monitors[monitor_idx];
    return &g.FallbackMonitor;
}

c_void ImGui::DestroyPlatformWindow(*mut ImGuiViewportP viewport)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (viewport->PlatformWindowCreated)
    {
        if (g.PlatformIO.Renderer_DestroyWindow)
            g.PlatformIO.Renderer_DestroyWindow(viewport);
        if (g.PlatformIO.Platform_DestroyWindow)
            g.PlatformIO.Platform_DestroyWindow(viewport);
        // IM_ASSERT(viewport->RendererUserData == NULL && viewport->PlatformUserData == NULL);

        // Don't clear PlatformWindowCreated for the main viewport, as we initially set that up to true in Initialize()
        // The righter way may be to leave it to the backend to set this flag all-together, and made the flag public.
        if (viewport->ID != IMGUI_VIEWPORT_DEFAULT_ID)
            viewport->PlatformWindowCreated = false;
    }
    else
    {
        // IM_ASSERT(viewport->RendererUserData == NULL && viewport->PlatformUserData == NULL && viewport->PlatformHandle == NULL);
    }
    viewport->RendererUserData = viewport->PlatformUserData = viewport->PlatformHandle = None;
    viewport->ClearRequestFlags();
}

c_void ImGui::DestroyPlatformWindows()
{
    // We call the destroy window on every viewport (including the main viewport, index 0) to give a chance to the backend
    // to clear any data they may have stored in e.g. PlatformUserData, RendererUserData.
    // It is convenient for the platform backend code to store something in the main viewport, in order for e.g. the mouse handling
    // code to operator a consistent manner.
    // It is expected that the backend can handle calls to Renderer_DestroyWindow/Platform_DestroyWindow without
    // crashing if it doesn't have data stored.
    let g = GImGui; // ImGuiContext& g = *GImGui;
    for (c_int i = 0; i < g.Viewports.Size; i++)
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
    ImGuiDir        SplitDir;
    c_float           SplitRatio;
    ImRect          DropRectsDraw[ImGuiDir_COUNT + 1];  // May be slightly different from hit-testing drop rects used in DockNodeCalcDropRects()

    ImGuiDockPreviewData() : FutureNode(0) { IsDropAllowed = IsCenterAvailable = IsSidesAvailable = IsSplitDirExplicit = false; SplitNode = None; SplitDir = ImGuiDir_None; SplitRatio = 0.f; for (c_int n = 0; n < IM_ARRAYSIZE(DropRectsDraw); n++) DropRectsDraw[n] = ImRect(+f32::MAX, +f32::MAX, -f32::MAX, -f32::MAX); }
};



//-----------------------------------------------------------------------------
// Docking: Forward Declarations
//-----------------------------------------------------------------------------

namespace ImGui
{
    // ImGuiDockContext
    static ImGuiDockNode*   DockContextAddNode(ImGuiContext* ctx, ImGuiID id);
    static c_void             DockContextRemoveNode(ImGuiContext* ctx, ImGuiDockNode* node, bool merge_sibling_into_parent_node);
    static c_void             DockContextQueueNotifyRemovedNode(ImGuiContext* ctx, ImGuiDockNode* node);
    static c_void             DockContextProcessDock(ImGuiContext* ctx, ImGuiDockRequest* req);
    static c_void             DockContextProcessUndockWindow(ImGuiContext* ctx, ImGuiWindow* window, let mut clear_persistent_docking_ref: bool =  true);
    static c_void             DockContextProcessUndockNode(ImGuiContext* ctx, ImGuiDockNode* node);
    static c_void             DockContextPruneUnusedSettingsNodes(ImGuiContext* ctx);
    static ImGuiDockNode*   DockContextBindNodeToWindow(ImGuiContext* ctx, ImGuiWindow* window);
    static c_void             DockContextBuildNodesFromSettings(ImGuiContext* ctx, ImGuiDockNodeSettings* node_settings_array, c_int node_settings_count);
    static c_void             DockContextBuildAddWindowsToNodes(ImGuiContext* ctx, ImGuiID root_id);                            // Use root_id==0 to add all

    // ImGuiDockNode
    static c_void             DockNodeAddWindow(ImGuiDockNode* node, ImGuiWindow* window, bool add_to_tab_bar);
    static c_void             DockNodeMoveWindows(ImGuiDockNode* dst_node, ImGuiDockNode* src_node);
    static c_void             DockNodeMoveChildNodes(ImGuiDockNode* dst_node, ImGuiDockNode* src_node);
    static ImGuiWindow*     DockNodeFindWindowByID(ImGuiDockNode* node, ImGuiID id);
    static c_void             DockNodeApplyPosSizeToWindows(ImGuiDockNode* node);
    static c_void             DockNodeRemoveWindow(ImGuiDockNode* node, ImGuiWindow* window, ImGuiID save_dock_id);
    static c_void             DockNodeHideHostWindow(ImGuiDockNode* node);
    static c_void             DockNodeUpdate(ImGuiDockNode* node);
    static c_void             DockNodeUpdateForRootNode(ImGuiDockNode* node);
    static c_void             DockNodeUpdateFlagsAndCollapse(ImGuiDockNode* node);
    static c_void             DockNodeUpdateHasCentralNodeChild(ImGuiDockNode* node);
    static c_void             DockNodeUpdateTabBar(ImGuiDockNode* node, ImGuiWindow* host_window);
    static c_void             DockNodeAddTabBar(ImGuiDockNode* node);
    static c_void             DockNodeRemoveTabBar(ImGuiDockNode* node);
    static ImGuiID          DockNodeUpdateWindowMenu(ImGuiDockNode* node, ImGuiTabBar* tab_bar);
    static c_void             DockNodeUpdateVisibleFlag(ImGuiDockNode* node);
    static c_void             DockNodeStartMouseMovingWindow(ImGuiDockNode* node, ImGuiWindow* window);
    static bool             DockNodeIsDropAllowed(ImGuiWindow* host_window, ImGuiWindow* payload_window);
    static c_void             DockNodePreviewDockSetup(ImGuiWindow* host_window, ImGuiDockNode* host_node, ImGuiWindow* payload_window, ImGuiDockNode* payload_node, ImGuiDockPreviewData* preview_data, bool is_explicit_target, bool is_outer_docking);
    static c_void             DockNodePreviewDockRender(ImGuiWindow* host_window, ImGuiDockNode* host_node, ImGuiWindow* payload_window, *const ImGuiDockPreviewData preview_data);
    static c_void             DockNodeCalcTabBarLayout(*const ImGuiDockNode node, ImRect* out_title_rect, ImRect* out_tab_bar_rect, ImVec2* out_window_menu_button_pos, ImVec2* out_close_button_pos);
    static c_void             DockNodeCalcSplitRects(ImVec2& pos_old, ImVec2& size_old, ImVec2& pos_new, ImVec2& size_new, ImGuiDir dir, ImVec2 size_new_desired);
    static bool             DockNodeCalcDropRectsAndTestMousePos(const ImRect& parent, ImGuiDir dir, ImRect& out_draw, bool outer_docking, ImVec2* test_mouse_pos);
    static *const char      DockNodeGetHostWindowTitle(ImGuiDockNode* node, char* buf, c_int buf_size) { ImFormatString(buf, buf_size, "##DockNode_%02X", node->ID); return buf; }
    static c_int              DockNodeGetTabOrder(ImGuiWindow* window);

    // ImGuiDockNode tree manipulations
    static c_void             DockNodeTreeSplit(ImGuiContext* ctx, ImGuiDockNode* parent_node, ImGuiAxis split_axis, c_int split_first_child, c_float split_ratio, ImGuiDockNode* new_node);
    static c_void             DockNodeTreeMerge(ImGuiContext* ctx, ImGuiDockNode* parent_node, ImGuiDockNode* merge_lead_child);
    static c_void             DockNodeTreeUpdatePosSize(ImGuiDockNode* node, ImVec2 pos, ImVec2 size, ImGuiDockNode* only_write_to_single_node = NULL);
    static c_void             DockNodeTreeUpdateSplitter(ImGuiDockNode* node);
    static ImGuiDockNode*   DockNodeTreeFindVisibleNodeByPos(ImGuiDockNode* node, ImVec2 pos);
    static ImGuiDockNode*   DockNodeTreeFindFallbackLeafNode(ImGuiDockNode* node);

    // Settings
    static c_void             DockSettingsRenameNodeReferences(ImGuiID old_node_id, ImGuiID new_node_id);
    static c_void             DockSettingsRemoveNodeReferences(ImGuiID* node_ids, c_int node_ids_count);
    static ImGuiDockNodeSettings*   DockSettingsFindNodeSettings(ImGuiContext* ctx, ImGuiID node_id);
    static c_void             DockSettingsHandler_ClearAll(ImGuiContext*, ImGuiSettingsHandler*);
    static c_void             DockSettingsHandler_ApplyAll(ImGuiContext*, ImGuiSettingsHandler*);
    static *mut c_void            DockSettingsHandler_ReadOpen(ImGuiContext*, ImGuiSettingsHandler*, *const char name);
    static c_void             DockSettingsHandler_ReadLine(ImGuiContext*, ImGuiSettingsHandler*, entry: *mut c_void, *const char line);
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

c_void ImGui::DockContextInitialize(ImGuiContext* ctx)
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

c_void ImGui::DockContextShutdown(ImGuiContext* ctx)
{
    ImGuiDockContext* dc  = &ctx->DockContext;
    for (c_int n = 0; n < dc->Nodes.Data.Size; n++)
        if (ImGuiDockNode* node = (ImGuiDockNode*)dc->Nodes.Data[n].val_p)
            IM_DELETE(node);
}

c_void ImGui::DockContextClearNodes(ImGuiContext* ctx, ImGuiID root_id, bool clear_settings_refs)
{
    IM_UNUSED(ctx);
    // IM_ASSERT(ctx == GImGui);
    DockBuilderRemoveNodeDockedWindows(root_id, clear_settings_refs);
    DockBuilderRemoveNodeChildNodes(root_id);
}

// [DEBUG] This function also acts as a defacto test to make sure we can rebuild from scratch without a glitch
// (Different from DockSettingsHandler_ClearAll() + DockSettingsHandler_ApplyAll() because this reuses current settings!)
c_void ImGui::DockContextRebuildNodes(ImGuiContext* ctx)
{
    ImGuiContext& g = *ctx;
    ImGuiDockContext* dc = &ctx->DockContext;
    IMGUI_DEBUG_LOG_DOCKING("[docking] DockContextRebuildNodes\n");
    SaveIniSettingsToMemory();
    ImGuiID root_id = 0; // Rebuild all
    DockContextClearNodes(ctx, root_id, false);
    DockContextBuildNodesFromSettings(ctx, dc->NodesSettings.Data, dc->NodesSettings.Size);
    DockContextBuildAddWindowsToNodes(ctx, root_id);
}

// Docking context update function, called by NewFrame()
c_void ImGui::DockContextNewFrameUpdateUndocking(ImGuiContext* ctx)
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
        for (c_int n = 0; n < dc->Nodes.Data.Size; n++)
            if (ImGuiDockNode* node = (ImGuiDockNode*)dc->Nodes.Data[n].val_p)
                if (node->IsRootNode() && node->IsSplitNode())
                {
                    DockBuilderRemoveNodeChildNodes(node->ID);
                    //dc->WantFullRebuild = true;
                }

    // Process full rebuild
// #if 0
    if (ImGui::IsKeyPressed(ImGui::GetKeyIndex(ImGuiKey_C)))
        dc->WantFullRebuild = true;
// #endif
    if (dc->WantFullRebuild)
    {
        DockContextRebuildNodes(ctx);
        dc->WantFullRebuild = false;
    }

    // Process Undocking requests (we need to process them _before_ the UpdateMouseMovingWindowNewFrame call in NewFrame)
    for (c_int n = 0; n < dc->Requests.Size; n++)
    {
        ImGuiDockRequest* req = &dc->Requests[n];
        if (req->Type == ImGuiDockRequestType_Undock && req->UndockTargetWindow)
            DockContextProcessUndockWindow(ctx, req->UndockTargetWindow);
        else if (req->Type == ImGuiDockRequestType_Undock && req->UndockTargetNode)
            DockContextProcessUndockNode(ctx, req->UndockTargetNode);
    }
}

// Docking context update function, called by NewFrame()
c_void ImGui::DockContextNewFrameUpdateDocking(ImGuiContext* ctx)
{
    ImGuiContext& g = *ctx;
    ImGuiDockContext* dc  = &ctx->DockContext;
    if (!(g.IO.ConfigFlags & ImGuiConfigFlags_DockingEnable))
        return;

    // [DEBUG] Store hovered dock node.
    // We could in theory use DockNodeTreeFindVisibleNodeByPos() on the root host dock node, but using ->DockNode is a good shortcut.
    // Note this is mostly a debug thing and isn't actually used for docking target, because docking involve more detailed filtering.
    g.DebugHoveredDockNode = None;
    if (ImGuiWindow* hovered_window = g.HoveredWindowUnderMovingWindow)
    {
        if (hovered_window.DockNodeAsHost)
            g.DebugHoveredDockNode = DockNodeTreeFindVisibleNodeByPos(hovered_window.DockNodeAsHost, g.IO.MousePos);
        else if (hovered_window.Rootwindow.DockNode)
            g.DebugHoveredDockNode = hovered_window.Rootwindow.DockNode;
    }

    // Process Docking requests
    for (c_int n = 0; n < dc->Requests.Size; n++)
        if (dc->Requests[n].Type == ImGuiDockRequestType_Dock)
            DockContextProcessDock(ctx, &dc->Requests[n]);
    dc->Requests.resize(0);

    // Create windows for each automatic docking nodes
    // We can have NULL pointers when we delete nodes, but because ID are recycled this should amortize nicely (and our node count will never be very high)
    for (c_int n = 0; n < dc->Nodes.Data.Size; n++)
        if (ImGuiDockNode* node = (ImGuiDockNode*)dc->Nodes.Data[n].val_p)
            if (node->IsFloatingNode())
                DockNodeUpdate(node);
}

c_void ImGui::DockContextEndFrame(ImGuiContext* ctx)
{
    // Draw backgrounds of node missing their window
    ImGuiContext& g = *ctx;
    ImGuiDockContext* dc = &g.DockContext;
    for (c_int n = 0; n < dc->Nodes.Data.Size; n++)
        if (ImGuiDockNode* node = (ImGuiDockNode*)dc->Nodes.Data[n].val_p)
            if (node->LastFrameActive == g.FrameCount && node->IsVisible && node->HostWindow && node->IsLeafNode() && !node->IsBgDrawnThisFrame)
            {
                ImRect bg_rect(node->Pos + ImVec2(0f32, GetFrameHeight()), node->Pos + node->Size);
                ImDrawFlags bg_rounding_flags = CalcRoundingFlagsForRectInRect(bg_rect, node->Hostwindow.Rect(), DOCKING_SPLITTER_SIZE);
                node->Hostwindow.DrawList.ChannelsSetCurrent(DOCKING_HOST_DRAW_CHANNEL_BG);
                node->Hostwindow.DrawList.AddRectFilled(bg_rect.Min, bg_rect.Max, node->LastBgColor, node->Hostwindow.WindowRounding, bg_rounding_flags);
            }
}

ImGuiDockNode* ImGui::DockContextFindNodeByID(ImGuiContext* ctx, ImGuiID id)
{
    return (ImGuiDockNode*)ctx->DockContext.Nodes.GetVoidPtr(id);
}

ImGuiID ImGui::DockContextGenNodeID(ImGuiContext* ctx)
{
    // Generate an ID for new node (the exact ID value doesn't matter as long as it is not already used)
    // FIXME-OPT FIXME-DOCK: This is suboptimal, even if the node count is small enough not to be a worry.0
    // We should poke in ctx->Nodes to find a suitable ID faster. Even more so trivial that ctx->Nodes lookup is already sorted.
    ImGuiID id = 0x0001;
    while (DockContextFindNodeByID(ctx, id) != NULL)
        id+= 1;
    return id;
}

static ImGuiDockNode* ImGui::DockContextAddNode(ImGuiContext* ctx, ImGuiID id)
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
    ctx->DockContext.Nodes.SetVoidPtr(node->ID, node);
    return node;
}

static c_void ImGui::DockContextRemoveNode(ImGuiContext* ctx, ImGuiDockNode* node, bool merge_sibling_into_parent_node)
{
    ImGuiContext& g = *ctx;
    ImGuiDockContext* dc  = &ctx->DockContext;

    IMGUI_DEBUG_LOG_DOCKING("[docking] DockContextRemoveNode 0x%08X\n", node->ID);
    // IM_ASSERT(DockContextFindNodeByID(ctx, node->ID) == node);
    // IM_ASSERT(node->ChildNodes[0] == NULL && node->ChildNodes[1] == NULL);
    // IM_ASSERT(node->Windows.Size == 0);

    if (node->HostWindow)
        node->Hostwindow.DockNodeAsHost = None;

    ImGuiDockNode* parent_node = node->ParentNode;
    let merge: bool = (merge_sibling_into_parent_node && parent_node != NULL);
    if (merge)
    {
        // IM_ASSERT(parent_node->ChildNodes[0] == node || parent_node->ChildNodes[1] == node);
        ImGuiDockNode* sibling_node = (parent_node->ChildNodes[0] == node ? parent_node->ChildNodes[1] : parent_node->ChildNodes[0]);
        DockNodeTreeMerge(&g, parent_node, sibling_node);
    }
    else
    {
        for (c_int n = 0; parent_node && n < IM_ARRAYSIZE(parent_node->ChildNodes); n++)
            if (parent_node->ChildNodes[n] == node)
                node->ParentNode->ChildNodes[n] = None;
        dc->Nodes.SetVoidPtr(node->ID, NULL);
        IM_DELETE(node);
    }
}

static c_int IMGUI_CDECL DockNodeComparerDepthMostFirst(*const c_void lhs, *const c_void rhs)
{
    *const ImGuiDockNode a = *(*const ImGuiDockNode const*)lhs;
    *const ImGuiDockNode b = *(*const ImGuiDockNode const*)rhs;
    return ImGui::DockNodeGetDepth(b) - ImGui::DockNodeGetDepth(a);
}

// Pre C++0x doesn't allow us to use a function-local type (without linkage) as template parameter, so we moved this here.
struct ImGuiDockContextPruneNodeData
{
    c_int         CountWindows, CountChildWindows, CountChildNodes;
    ImGuiID     RootId;
    ImGuiDockContextPruneNodeData() { CountWindows = CountChildWindows = CountChildNodes = 0; RootId = 0; }
};

// Garbage collect unused nodes (run once at init time)
static c_void ImGui::DockContextPruneUnusedSettingsNodes(ImGuiContext* ctx)
{
    ImGuiContext& g = *ctx;
    ImGuiDockContext* dc  = &ctx->DockContext;
    // IM_ASSERT(g.Windows.Size == 0);

    ImPool<ImGuiDockContextPruneNodeData> pool;
    pool.Reserve(dc->NodesSettings.Size);

    // Count child nodes and compute RootID
    for (c_int settings_n = 0; settings_n < dc->NodesSettings.Size; settings_n++)
    {
        ImGuiDockNodeSettings* settings = &dc->NodesSettings[settings_n];
        ImGuiDockContextPruneNodeData* parent_data = settings->ParentNodeId ? pool.GetByKey(settings->ParentNodeId) : 0;
        pool.GetOrAddByKey(settings->ID)->RootId = parent_data ? parent_data->RootId : settings->ID;
        if (settings->ParentNodeId)
            pool.GetOrAddByKey(settings->ParentNodeId)->CountChildNodes+= 1;
    }

    // Count reference to dock ids from dockspaces
    // We track the 'auto-DockNode <- manual-Window <- manual-DockSpace' in order to avoid 'auto-DockNode' being ditched by DockContextPruneUnusedSettingsNodes()
    for (c_int settings_n = 0; settings_n < dc->NodesSettings.Size; settings_n++)
    {
        ImGuiDockNodeSettings* settings = &dc->NodesSettings[settings_n];
        if (settings->ParentWindowId != 0)
            if (ImGuiWindowSettings* window_settings = FindWindowSettings(settings->ParentWindowId))
                if (window_settings->DockId)
                    if (ImGuiDockContextPruneNodeData* data = pool.GetByKey(window_settings->DockId))
                        data.CountChildNodes+= 1;
    }

    // Count reference to dock ids from window settings
    // We guard against the possibility of an invalid .ini file (RootID may point to a missing node)
    for (ImGuiWindowSettings* settings = g.SettingsWindows.begin(); settings != None; settings = g.SettingsWindows.next_chunk(settings))
        if (ImGuiID dock_id = settings->DockId)
            if (ImGuiDockContextPruneNodeData* data = pool.GetByKey(dock_id))
            {
                data.CountWindows+= 1;
                if (ImGuiDockContextPruneNodeData* data_root = (data.RootId == dock_id) ? data : pool.GetByKey(data.RootId))
                    data_root->CountChildWindows+= 1;
            }

    // Prune
    for (c_int settings_n = 0; settings_n < dc->NodesSettings.Size; settings_n++)
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

static c_void ImGui::DockContextBuildNodesFromSettings(ImGuiContext* ctx, ImGuiDockNodeSettings* node_settings_array, c_int node_settings_count)
{
    // Build nodes
    for (c_int node_n = 0; node_n < node_settings_count; node_n++)
    {
        ImGuiDockNodeSettings* settings = &node_settings_array[node_n];
        if (settings->ID == 0)
            continue;
        ImGuiDockNode* node = DockContextAddNode(ctx, settings->ID);
        node->ParentNode = settings->ParentNodeId ? DockContextFindNodeByID(ctx, settings->ParentNodeId) : NULL;
        node->Pos = ImVec2(settings->Pos.x, settings->Pos.y);
        node->Size = ImVec2(settings->Size.x, settings->Size.y);
        node->SizeRef = ImVec2(settings->SizeRef.x, settings->SizeRef.y);
        node->AuthorityForPos = node->AuthorityForSize = node->AuthorityForViewport = ImGuiDataAuthority_DockNode;
        if (node->ParentNode && node->ParentNode->ChildNodes[0] == NULL)
            node->ParentNode->ChildNodes[0] = node;
        else if (node->ParentNode && node->ParentNode->ChildNodes[1] == NULL)
            node->ParentNode->ChildNodes[1] = node;
        node->SelectedTabId = settings->SelectedTabId;
        node->SplitAxis = (ImGuiAxis)settings->SplitAxis;
        node->SetLocalFlags(settings->Flags & ImGuiDockNodeFlags_SavedFlagsMask_);

        // Bind host window immediately if it already exist (in case of a rebuild)
        // This is useful as the RootWindowForTitleBarHighlight links necessary to highlight the currently focused node requires node->HostWindow to be set.
        host_window_title: [c_char;20];
        ImGuiDockNode* root_node = DockNodeGetRootNode(node);
        node->HostWindow = FindWindowByName(DockNodeGetHostWindowTitle(root_node, host_window_title, IM_ARRAYSIZE(host_window_title)));
    }
}

c_void ImGui::DockContextBuildAddWindowsToNodes(ImGuiContext* ctx, ImGuiID root_id)
{
    // Rebind all windows to nodes (they can also lazily rebind but we'll have a visible glitch during the first frame)
    ImGuiContext& g = *ctx;
    for (c_int n = 0; n < g.Windows.Size; n++)
    {
        ImGuiWindow* window = g.Windows[n];
        if (window.DockId == 0 || window.LastFrameActive < g.FrameCount - 1)
            continue;
        if (window.DockNode != NULL)
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

c_void ImGui::DockContextQueueDock(ImGuiContext* ctx, ImGuiWindow* target, ImGuiDockNode* target_node, ImGuiWindow* payload, ImGuiDir split_dir, c_float split_ratio, bool split_outer)
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

c_void ImGui::DockContextQueueUndockWindow(ImGuiContext* ctx, ImGuiWindow* window)
{
    ImGuiDockRequest req;
    req.Type = ImGuiDockRequestType_Undock;
    req.UndockTargetWindow = window;
    ctx->DockContext.Requests.push(req);
}

c_void ImGui::DockContextQueueUndockNode(ImGuiContext* ctx, ImGuiDockNode* node)
{
    ImGuiDockRequest req;
    req.Type = ImGuiDockRequestType_Undock;
    req.UndockTargetNode = node;
    ctx->DockContext.Requests.push(req);
}

c_void ImGui::DockContextQueueNotifyRemovedNode(ImGuiContext* ctx, ImGuiDockNode* node)
{
    ImGuiDockContext* dc  = &ctx->DockContext;
    for (c_int n = 0; n < dc->Requests.Size; n++)
        if (dc->Requests[n].DockTargetNode == node)
            dc->Requests[n].Type = ImGuiDockRequestType_None;
}

c_void ImGui::DockContextProcessDock(ImGuiContext* ctx, ImGuiDockRequest* req)
{
    // IM_ASSERT((req->Type == ImGuiDockRequestType_Dock && req->DockPayload != NULL) || (req->Type == ImGuiDockRequestType_Split && req->DockPayload == NULL));
    // IM_ASSERT(req->DockTargetWindow != NULL || req->DockTargetNode != NULL);

    ImGuiContext& g = *ctx;
    IM_UNUSED(g);

    ImGuiWindow* payload_window = req->DockPayload;     // Optional
    ImGuiWindow* target_window = req->DockTargetWindow;
    ImGuiDockNode* node = req->DockTargetNode;
    if (payload_window)
        IMGUI_DEBUG_LOG_DOCKING("[docking] DockContextProcessDock node 0x%08X target '%s' dock window '%s', split_dir %d\n", node ? node->ID : 0, target_window ? target_window.Name : "NULL", payload_window.Name, req->DockSplitDir);
    else
        IMGUI_DEBUG_LOG_DOCKING("[docking] DockContextProcessDock node 0x%08X, split_dir %d\n", node ? node->ID : 0, req->DockSplitDir);

    // Decide which Tab will be selected at the end of the operation
    ImGuiID next_selected_id = 0;
    ImGuiDockNode* payload_node = None;
    if (payload_window)
    {
        payload_node = payload_window.DockNodeAsHost;
        payload_window.DockNodeAsHost = None; // Important to clear this as the node will have its life as a child which might be merged/deleted later.
        if (payload_node && payload_node->IsLeafNode())
            next_selected_id = payload_node->TabBar->NextSelectedTabId ? payload_node->TabBar->NextSelectedTabId : payload_node->TabBar->SelectedTabId;
        if (payload_node == NULL)
            next_selected_id = payload_window.TabId;
    }

    // FIXME-DOCK: When we are trying to dock an existing single-window node into a loose window, transfer Node ID as well
    // When processing an interactive split, usually LastFrameAlive will be < g.FrameCount. But DockBuilder operations can make it ==.
    if (node)
        // IM_ASSERT(node->LastFrameAlive <= g.FrameCount);
    if (node && target_window && node == target_window.DockNodeAsHost)
        // IM_ASSERT(node->Windows.Size > 0 || node->IsSplitNode() || node->IsCentralNode());

    // Create new node and add existing window to it
    if (node == NULL)
    {
        node = DockContextAddNode(ctx, 0);
        node->Pos = target_window.Pos;
        node->Size = target_window.Size;
        if (target_window.DockNodeAsHost == NULL)
        {
            DockNodeAddWindow(node, target_window, true);
            node->TabBar->Tabs[0].Flags &= ~ImGuiTabItemFlags_Unsorted;
            target_window.DockIsActive = true;
        }
    }

    ImGuiDir split_dir = req->DockSplitDir;
    if (split_dir != ImGuiDir_None)
    {
        // Split into two, one side will be our payload node unless we are dropping a loose window
        const ImGuiAxis split_axis = (split_dir == ImGuiDir_Left || split_dir == ImGuiDir_Right) ? ImGuiAxis_X : ImGuiAxis_Y;
        let split_inheritor_child_idx: c_int = (split_dir == ImGuiDir_Left || split_dir == ImGuiDir_Up) ? 1 : 0; // Current contents will be moved to the opposite side
        const c_float split_ratio = req->DockSplitRatio;
        DockNodeTreeSplit(ctx, node, split_axis, split_inheritor_child_idx, split_ratio, payload_node);  // payload_node may be NULL here!
        ImGuiDockNode* new_node = node->ChildNodes[split_inheritor_child_idx ^ 1];
        new_node->HostWindow = node->HostWindow;
        node = new_node;
    }
    node->SetLocalFlags(node->LocalFlags & ~ImGuiDockNodeFlags_HiddenTabBar);

    if (node != payload_node)
    {
        // Create tab bar before we call DockNodeMoveWindows (which would attempt to move the old tab-bar, which would lead us to payload tabs wrongly appearing before target tabs!)
        if (node->Windows.Size > 0 && node->TabBar == NULL)
        {
            DockNodeAddTabBar(node);
            for (c_int n = 0; n < node->Windows.Size; n++)
                TabBarAddTab(node->TabBar, ImGuiTabItemFlags_None, node->Windows[n]);
        }

        if (payload_node != NULL)
        {
            // Transfer full payload node (with 1+ child windows or child nodes)
            if (payload_node->IsSplitNode())
            {
                if (node->Windows.Size > 0)
                {
                    // We can dock a split payload into a node that already has windows _only_ if our payload is a node tree with a single visible node.
                    // In this situation, we move the windows of the target node into the currently visible node of the payload.
                    // This allows us to preserve some of the underlying dock tree settings nicely.
                    // IM_ASSERT(payload_node->OnlyNodeWithWindows != NULL); // The docking should have been blocked by DockNodePreviewDockSetup() early on and never submitted.
                    ImGuiDockNode* visible_node = payload_node->OnlyNodeWithWindows;
                    if (visible_node->TabBar)
                        // IM_ASSERT(visible_node->TabBar->Tabs.Size > 0);
                    DockNodeMoveWindows(node, visible_node);
                    DockNodeMoveWindows(visible_node, node);
                    DockSettingsRenameNodeReferences(node->ID, visible_node->ID);
                }
                if (node->IsCentralNode())
                {
                    // Central node property needs to be moved to a leaf node, pick the last focused one.
                    // FIXME-DOCK: If we had to transfer other flags here, what would the policy be?
                    ImGuiDockNode* last_focused_node = DockContextFindNodeByID(ctx, payload_node->LastFocusedNodeId);
                    // IM_ASSERT(last_focused_node != NULL);
                    ImGuiDockNode* last_focused_root_node = DockNodeGetRootNode(last_focused_node);
                    // IM_ASSERT(last_focused_root_node == DockNodeGetRootNode(payload_node));
                    last_focused_node->SetLocalFlags(last_focused_node->LocalFlags | ImGuiDockNodeFlags_CentralNode);
                    node->SetLocalFlags(node->LocalFlags & ~ImGuiDockNodeFlags_CentralNode);
                    last_focused_root_node->CentralNode = last_focused_node;
                }

                // IM_ASSERT(node->Windows.Size == 0);
                DockNodeMoveChildNodes(node, payload_node);
            }
            else
            {
                const ImGuiID payload_dock_id = payload_node->ID;
                DockNodeMoveWindows(node, payload_node);
                DockSettingsRenameNodeReferences(payload_dock_id, node->ID);
            }
            DockContextRemoveNode(ctx, payload_node, true);
        }
        else if (payload_window)
        {
            // Transfer single window
            const ImGuiID payload_dock_id = payload_window.DockId;
            node->VisibleWindow = payload_window;
            DockNodeAddWindow(node, payload_window, true);
            if (payload_dock_id != 0)
                DockSettingsRenameNodeReferences(payload_dock_id, node->ID);
        }
    }
    else
    {
        // When docking a floating single window node we want to reevaluate auto-hiding of the tab bar
        node->WantHiddenTabBarUpdate = true;
    }

    // Update selection immediately
    if (ImGuiTabBar* tab_bar = node->TabBar)
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
static ImVec2 FixLargeWindowsWhenUndocking(const ImVec2& size, ImGuiViewport* ref_viewport)
{
    if (ref_viewport == NULL)
        return size;

    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImVec2 max_size = ImFloor(ref_viewport->WorkSize * 0.900f32);
    if (g.ConfigFlagsCurrFrame & ImGuiConfigFlags_ViewportsEnable)
    {
        *const ImGuiPlatformMonitor monitor = ImGui::GetViewportPlatformMonitor(ref_viewport);
        max_size = ImFloor(monitor->WorkSize * 0.900f32);
    }
    return ImMin(size, max_size);
}

c_void ImGui::DockContextProcessUndockWindow(ImGuiContext* ctx, ImGuiWindow* window, bool clear_persistent_docking_re0f32)
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

c_void ImGui::DockContextProcessUndockNode(ImGuiContext* ctx, ImGuiDockNode* node)
{
    ImGuiContext& g = *ctx;
    IMGUI_DEBUG_LOG_DOCKING("[docking] DockContextProcessUndockNode node %08X\n", node->ID);
    // IM_ASSERT(node->IsLeafNode());
    // IM_ASSERT(node->Windows.Size >= 1);

    if (node->IsRootNode() || node->IsCentralNode())
    {
        // In the case of a root node or central node, the node will have to stay in place. Create a new node to receive the payload.
        ImGuiDockNode* new_node = DockContextAddNode(ctx, 0);
        new_node->Pos = node->Pos;
        new_node->Size = node->Size;
        new_node->SizeRef = node->SizeRef;
        DockNodeMoveWindows(new_node, node);
        DockSettingsRenameNodeReferences(node->ID, new_node->ID);
        node = new_node;
    }
    else
    {
        // Otherwise extract our node and merge our sibling back into the parent node.
        // IM_ASSERT(node->ParentNode->ChildNodes[0] == node || node->ParentNode->ChildNodes[1] == node);
        c_int index_in_parent = (node->ParentNode->ChildNodes[0] == node) ? 0 : 1;
        node->ParentNode->ChildNodes[index_in_parent] = None;
        DockNodeTreeMerge(ctx, node->ParentNode, node->ParentNode->ChildNodes[index_in_parent ^ 1]);
        node->ParentNode->AuthorityForViewport = ImGuiDataAuthority_Window; // The node that stays in place keeps the viewport, so our newly dragged out node will create a new viewport
        node->ParentNode = None;
    }
    for (c_int n = 0; n < node->Windows.Size; n++)
    {
        ImGuiWindow* window = node->Windows[n];
        window.Flags &= ~ImGuiWindowFlags_ChildWindow;
        if (window.ParentWindow)
            window.Parentwindow.DC.ChildWindows.find_erase(window);
        UpdateWindowParentAndRootLinks(window, window.Flags, NULL);
    }
    node->AuthorityForPos = node->AuthorityForSize = ImGuiDataAuthority_DockNode;
    node->Size = FixLargeWindowsWhenUndocking(node->Size, node->Windows[0]->Viewport);
    node->WantMouseMove = true;
    MarkIniSettingsDirty();
}

// This is mostly used for automation.
bool ImGui::DockContextCalcDropPosForDocking(ImGuiWindow* target, ImGuiDockNode* target_node, ImGuiWindow* payload_window, ImGuiDockNode* payload_node, ImGuiDir split_dir, bool split_outer, ImVec2* out_pos)
{
    // In DockNodePreviewDockSetup() for a root central node instead of showing both "inner" and "outer" drop rects
    // (which would be functionally identical) we only show the outer one. Reflect this here.
    if (target_node && target_node->ParentNode == NULL && target_node->IsCentralNode() && split_dir != ImGuiDir_None)
        split_outer = true;
    ImGuiDockPreviewData split_data;
    DockNodePreviewDockSetup(target, target_node, payload_window, payload_node, &split_data, false, split_outer);
    if (split_data.DropRectsDraw[split_dir+1].IsInverted())
        return false;
    *out_pos = split_data.DropRectsDraw[split_dir+1].GetCenter();
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

ImGuiDockNode::ImGuiDockNode(ImGuiID id)
{
    ID = id;
    SharedFlags = LocalFlags = LocalFlagsInWindows = MergedFlags = ImGuiDockNodeFlags_None;
    ParentNode = ChildNodes[0] = ChildNodes[1] = None;
    TabBar = None;
    SplitAxis = ImGuiAxis_None;

    State = ImGuiDockNodeState_Unknown;
    LastBgColor = IM_COL32_WHITE;
    HostWindow = VisibleWindow = None;
    CentralNode = OnlyNodeWithWindows = None;
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

ImGuiDockNode::~ImGuiDockNode()
{
    IM_DELETE(TabBar);
    TabBar = None;
    ChildNodes[0] = ChildNodes[1] = None;
}

c_int ImGui::DockNodeGetTabOrder(ImGuiWindow* window)
{
    ImGuiTabBar* tab_bar = window.DockNode->TabBar;
    if (tab_bar == NULL)
        return -1;
    ImGuiTabItem* tab = TabBarFindTabByID(tab_bar, window.TabId);
    return tab ? tab_bar->GetTabOrder(tab) : -1;
}

static c_void DockNodeHideWindowDuringHostWindowCreation(ImGuiWindow* window)
{
    window.Hidden = true;
    window.HiddenFramesCanSkipItems = window.Active ? 1 : 2;
}

static c_void ImGui::DockNodeAddWindow(ImGuiDockNode* node, ImGuiWindow* window, bool add_to_tab_bar)
{
    let g = GImGui; // ImGuiContext& g = *GImGui; (void)g;
    if (window.DockNode)
    {
        // Can overwrite an existing window.DockNode (e.g. pointing to a disabled DockSpace node)
        // IM_ASSERT(window.DockNode->ID != node->ID);
        DockNodeRemoveWindow(window.DockNode, window, 0);
    }
    // IM_ASSERT(window.DockNode == NULL || window.DockNodeAsHost == NULL);
    IMGUI_DEBUG_LOG_DOCKING("[docking] DockNodeAddWindow node 0x%08X window '%s'\n", node->ID, window.Name);

    // If more than 2 windows appeared on the same frame leading to the creation of a new hosting window,
    // we'll hide windows until the host window is ready. Hide the 1st window after its been output (so it is not visible for one frame).
    // We will call DockNodeHideWindowDuringHostWindowCreation() on ourselves in Begin()
    if (node->HostWindow == NULL && node->Windows.Size == 1 && node->Windows[0]->WasActive == false)
        DockNodeHideWindowDuringHostWindowCreation(node->Windows[0]);

    node->Windows.push(window);
    node->WantHiddenTabBarUpdate = true;
    window.DockNode = node;
    window.DockId = node->ID;
    window.DockIsActive = (node->Windows.Size > 1);
    window.DockTabWantClose = false;

    // When reactivating a node with one or two loose window, the window pos/size/viewport are authoritative over the node storage.
    // In particular it is important we init the viewport from the first window so we don't create two viewports and drop one.
    if (node->HostWindow == NULL && node->IsFloatingNode())
    {
        if (node->AuthorityForPos == ImGuiDataAuthority_Auto)
            node->AuthorityForPos = ImGuiDataAuthority_Window;
        if (node->AuthorityForSize == ImGuiDataAuthority_Auto)
            node->AuthorityForSize = ImGuiDataAuthority_Window;
        if (node->AuthorityForViewport == ImGuiDataAuthority_Auto)
            node->AuthorityForViewport = ImGuiDataAuthority_Window;
    }

    // Add to tab bar if requested
    if (add_to_tab_bar)
    {
        if (node->TabBar == NULL)
        {
            DockNodeAddTabBar(node);
            node->TabBar->SelectedTabId = node->TabBar->NextSelectedTabId = node->SelectedTabId;

            // Add existing windows
            for (c_int n = 0; n < node->Windows.Size - 1; n++)
                TabBarAddTab(node->TabBar, ImGuiTabItemFlags_None, node->Windows[n]);
        }
        TabBarAddTab(node->TabBar, ImGuiTabItemFlags_Unsorted, window);
    }

    DockNodeUpdateVisibleFlag(node);

    // Update this without waiting for the next time we Begin() in the window, so our host window will have the proper title bar color on its first frame.
    if (node->HostWindow)
        UpdateWindowParentAndRootLinks(window, window.Flags | ImGuiWindowFlags_ChildWindow, node->HostWindow);
}

static c_void ImGui::DockNodeRemoveWindow(ImGuiDockNode* node, ImGuiWindow* window, ImGuiID save_dock_id)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(window.DockNode == node);
    //IM_ASSERT(window.RootWindowDockTree == node->HostWindow);
    //IM_ASSERT(window.LastFrameActive < g.FrameCount);    // We may call this from Begin()
    // IM_ASSERT(save_dock_id == 0 || save_dock_id == node->ID);
    IMGUI_DEBUG_LOG_DOCKING("[docking] DockNodeRemoveWindow node 0x%08X window '%s'\n", node->ID, window.Name);

    window.DockNode = None;
    window.DockIsActive = window.DockTabWantClose = false;
    window.DockId = save_dock_id;
    window.Flags &= ~ImGuiWindowFlags_ChildWindow;
    if (window.ParentWindow)
        window.Parentwindow.DC.ChildWindows.find_erase(window);
    UpdateWindowParentAndRootLinks(window, window.Flags, NULL); // Update immediately

    // Remove window
    let mut erased: bool =  false;
    for (c_int n = 0; n < node->Windows.Size; n++)
        if (node->Windows[n] == window)
        {
            node->Windows.erase(node->Windows.Data + n);
            erased = true;
            break;
        }
    if (!erased)
        // IM_ASSERT(erased);
    if (node->VisibleWindow == window)
        node->VisibleWindow = None;

    // Remove tab and possibly tab bar
    node->WantHiddenTabBarUpdate = true;
    if (node->TabBar)
    {
        TabBarRemoveTab(node->TabBar, window.TabId);
        let tab_count_threshold_for_tab_bar: c_int = node->IsCentralNode() ? 1 : 2;
        if (node->Windows.Size < tab_count_threshold_for_tab_bar)
            DockNodeRemoveTabBar(node);
    }

    if (node->Windows.Size == 0 && !node->IsCentralNode() && !node->IsDockSpace() && window.DockId != node->ID)
    {
        // Automatic dock node delete themselves if they are not holding at least one tab
        DockContextRemoveNode(&g, node, true);
        return;
    }

    if (node->Windows.Size == 1 && !node->IsCentralNode() && node->HostWindow)
    {
        ImGuiWindow* remaining_window = node->Windows[0];
        if (node->Hostwindow.ViewportOwned && node->IsRootNode())
        {
            // Transfer viewport back to the remaining loose window
            IMGUI_DEBUG_LOG_VIEWPORT("[viewport] Node %08X transfer Viewport %08X=>%08X for Window '%s'\n", node->ID, node->Hostwindow.Viewport->ID, remaining_window.ID, remaining_window.Name);
            // IM_ASSERT(node->Hostwindow.Viewport->Window == node->HostWindow);
            node->Hostwindow.Viewport->Window = remaining_window;
            node->Hostwindow.Viewport->ID = remaining_window.ID;
        }
        remaining_window.Collapsed = node->Hostwindow.Collapsed;
    }

    // Update visibility immediately is required so the DockNodeUpdateRemoveInactiveChilds() processing can reflect changes up the tree
    DockNodeUpdateVisibleFlag(node);
}

static c_void ImGui::DockNodeMoveChildNodes(ImGuiDockNode* dst_node, ImGuiDockNode* src_node)
{
    // IM_ASSERT(dst_node->Windows.Size == 0);
    dst_node->ChildNodes[0] = src_node->ChildNodes[0];
    dst_node->ChildNodes[1] = src_node->ChildNodes[1];
    if (dst_node->ChildNodes[0])
        dst_node->ChildNodes[0]->ParentNode = dst_node;
    if (dst_node->ChildNodes[1])
        dst_node->ChildNodes[1]->ParentNode = dst_node;
    dst_node->SplitAxis = src_node->SplitAxis;
    dst_node->SizeRef = src_node->SizeRef;
    src_node->ChildNodes[0] = src_node->ChildNodes[1] = None;
}

static c_void ImGui::DockNodeMoveWindows(ImGuiDockNode* dst_node, ImGuiDockNode* src_node)
{
    // Insert tabs in the same orders as currently ordered (node->Windows isn't ordered)
    // IM_ASSERT(src_node && dst_node && dst_node != src_node);
    ImGuiTabBar* src_tab_bar = src_node->TabBar;
    if (src_tab_bar != NULL)
        // IM_ASSERT(src_node->Windows.Size <= src_node->TabBar->Tabs.Size);

    // If the dst_node is empty we can just move the entire tab bar (to preserve selection, scrolling, etc.)
    let mut move_tab_bar: bool =  (src_tab_bar != NULL) && (dst_node->TabBar == NULL);
    if (move_tab_bar)
    {
        dst_node->TabBar = src_node->TabBar;
        src_node->TabBar = None;
    }

    // Tab order is not important here, it is preserved by sorting in DockNodeUpdateTabBar().
    for (ImGuiWindow* window : src_node->Windows)
    {
        window.DockNode = None;
        window.DockIsActive = false;
        DockNodeAddWindow(dst_node, window, !move_tab_bar);
    }
    src_node->Windows.clear();

    if (!move_tab_bar && src_node->TabBar)
    {
        if (dst_node->TabBar)
            dst_node->TabBar->SelectedTabId = src_node->TabBar->SelectedTabId;
        DockNodeRemoveTabBar(src_node);
    }
}

static c_void ImGui::DockNodeApplyPosSizeToWindows(ImGuiDockNode* node)
{
    for (c_int n = 0; n < node->Windows.Size; n++)
    {
        SetWindowPos(node->Windows[n], node->Pos, ImGuiCond_Always); // We don't assign directly to Pos because it can break the calculation of SizeContents on next frame
        SetWindowSize(node->Windows[n], node->Size, ImGuiCond_Always);
    }
}

static c_void ImGui::DockNodeHideHostWindow(ImGuiDockNode* node)
{
    if (node->HostWindow)
    {
        if (node->Hostwindow.DockNodeAsHost == node)
            node->Hostwindow.DockNodeAsHost = None;
        node->HostWindow = None;
    }

    if (node->Windows.Size == 1)
    {
        node->VisibleWindow = node->Windows[0];
        node->Windows[0]->DockIsActive = false;
    }

    if (node->TabBar)
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
    if (node->Windows.Size > 0)
    {
        if (info.FirstNodeWithWindows == NULL)
            info.FirstNodeWithWindows = node;
        info.CountNodesWithWindows+= 1;
    }
    if (node->IsCentralNode())
    {
        // IM_ASSERT(info.CentralNode == NULL); // Should be only one
        // IM_ASSERT(node->IsLeafNode() && "If you get this assert: please submit .ini file + repro of actions leading to this.");
        info.CentralNode = node;
    }
    if (info.CountNodesWithWindows > 1 && info.CentralNode != NULL)
        return;
    if (node->ChildNodes[0])
        DockNodeFindInfo(node->ChildNodes[0], info);
    if (node->ChildNodes[1])
        DockNodeFindInfo(node->ChildNodes[1], info);
}

static ImGuiWindow* ImGui::DockNodeFindWindowByID(ImGuiDockNode* node, ImGuiID id)
{
    // IM_ASSERT(id != 0);
    for (c_int n = 0; n < node->Windows.Size; n++)
        if (node->Windows[n]->ID == id)
            return node->Windows[n];
    return NULL;
}

// - Remove inactive windows/nodes.
// - Update visibility flag.
static c_void ImGui::DockNodeUpdateFlagsAndCollapse(ImGuiDockNode* node)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(node->ParentNode == NULL || node->ParentNode->ChildNodes[0] == node || node->ParentNode->ChildNodes[1] == node);

    // Inherit most flags
    if (node->ParentNode)
        node->SharedFlags = node->ParentNode->SharedFlags & ImGuiDockNodeFlags_SharedFlagsInheritMask_;

    // Recurse into children
    // There is the possibility that one of our child becoming empty will delete itself and moving its sibling contents into 'node'.
    // If 'node->ChildNode[0]' delete itself, then 'node->ChildNode[1]->Windows' will be moved into 'node'
    // If 'node->ChildNode[1]' delete itself, then 'node->ChildNode[0]->Windows' will be moved into 'node' and the "remove inactive windows" loop will have run twice on those windows (harmless)
    node->HasCentralNodeChild = false;
    if (node->ChildNodes[0])
        DockNodeUpdateFlagsAndCollapse(node->ChildNodes[0]);
    if (node->ChildNodes[1])
        DockNodeUpdateFlagsAndCollapse(node->ChildNodes[1]);

    // Remove inactive windows, collapse nodes
    // Merge node flags overrides stored in windows
    node->LocalFlagsInWindows = ImGuiDockNodeFlags_None;
    for (c_int window_n = 0; window_n < node->Windows.Size; window_n++)
    {
        ImGuiWindow* window = node->Windows[window_n];
        // IM_ASSERT(window.DockNode == node);

        let mut node_was_active: bool =  (node->LastFrameActive + 1 == g.FrameCount);
        let mut remove: bool =  false;
        remove |= node_was_active && (window.LastFrameActive + 1 < g.FrameCount);
        remove |= node_was_active && (node->WantCloseAll || node->WantCloseTabId == window.TabId) && window.HasCloseButton && !(window.Flags & ImGuiWindowFlags_UnsavedDocument);  // Submit all _expected_ closure from last frame
        remove |= (window.DockTabWantClose);
        if (remove)
        {
            window.DockTabWantClose = false;
            if (node->Windows.Size == 1 && !node->IsCentralNode())
            {
                DockNodeHideHostWindow(node);
                node->State = ImGuiDockNodeState_HostWindowHiddenBecauseSingleWindow;
                DockNodeRemoveWindow(node, window, node->ID); // Will delete the node so it'll be invalid on return
                return;
            }
            DockNodeRemoveWindow(node, window, node->ID);
            window_n-= 1;
            continue;
        }

        // FIXME-DOCKING: Missing policies for conflict resolution, hence the "Experimental" tag on this.
        //node->LocalFlagsInWindow &= ~window.WindowClass.DockNodeFlagsOverrideClear;
        node->LocalFlagsInWindows |= window.WindowClass.DockNodeFlagsOverrideSet;
    }
    node->UpdateMergedFlags();

    // Auto-hide tab bar option
    ImGuiDockNodeFlags node_flags = node->MergedFlags;
    if (node->WantHiddenTabBarUpdate && node->Windows.Size == 1 && (node_flags & ImGuiDockNodeFlags_AutoHideTabBar) && !node->IsHiddenTabBar())
        node->WantHiddenTabBarToggle = true;
    node->WantHiddenTabBarUpdate = false;

    // Cancel toggling if we know our tab bar is enforced to be hidden at all times
    if (node->WantHiddenTabBarToggle && node->VisibleWindow && (node->Visiblewindow.WindowClass.DockNodeFlagsOverrideSet & ImGuiDockNodeFlags_HiddenTabBar))
        node->WantHiddenTabBarToggle = false;

    // Apply toggles at a single point of the frame (here!)
    if (node->Windows.Size > 1)
        node->SetLocalFlags(node->LocalFlags & ~ImGuiDockNodeFlags_HiddenTabBar);
    else if (node->WantHiddenTabBarToggle)
        node->SetLocalFlags(node->LocalFlags ^ ImGuiDockNodeFlags_HiddenTabBar);
    node->WantHiddenTabBarToggle = false;

    DockNodeUpdateVisibleFlag(node);
}

// This is rarely called as DockNodeUpdateForRootNode() generally does it most frames.
static c_void ImGui::DockNodeUpdateHasCentralNodeChild(ImGuiDockNode* node)
{
    node->HasCentralNodeChild = false;
    if (node->ChildNodes[0])
        DockNodeUpdateHasCentralNodeChild(node->ChildNodes[0]);
    if (node->ChildNodes[1])
        DockNodeUpdateHasCentralNodeChild(node->ChildNodes[1]);
    if (node->IsRootNode())
    {
        ImGuiDockNode* mark_node = node->CentralNode;
        while (mark_node)
        {
            mark_node->HasCentralNodeChild = true;
            mark_node = mark_node->ParentNode;
        }
    }
}

static c_void ImGui::DockNodeUpdateVisibleFlag(ImGuiDockNode* node)
{
    // Update visibility flag
    let mut is_visible: bool =  (node->ParentNode == NULL) ? node->IsDockSpace() : node->IsCentralNode();
    is_visible |= (node->Windows.Size > 0);
    is_visible |= (node->ChildNodes[0] && node->ChildNodes[0]->IsVisible);
    is_visible |= (node->ChildNodes[1] && node->ChildNodes[1]->IsVisible);
    node->IsVisible = is_visible;
}

static c_void ImGui::DockNodeStartMouseMovingWindow(ImGuiDockNode* node, ImGuiWindow* window)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(node->WantMouseMove == true);
    StartMouseMovingWindow(window);
    g.ActiveIdClickOffset = g.IO.MouseClickedPos[0] - node->Pos;
    g.MovingWindow = window; // If we are docked into a non moveable root window, StartMouseMovingWindow() won't set g.MovingWindow. Override that decision.
    node->WantMouseMove = false;
}

// Update CentralNode, OnlyNodeWithWindows, LastFocusedNodeID. Copy window class.
static c_void ImGui::DockNodeUpdateForRootNode(ImGuiDockNode* node)
{
    DockNodeUpdateFlagsAndCollapse(node);

    // - Setup central node pointers
    // - Find if there's only a single visible window in the hierarchy (in which case we need to display a regular title bar -> FIXME-DOCK: that last part is not done yet!)
    // Cannot merge this with DockNodeUpdateFlagsAndCollapse() because FirstNodeWithWindows is found after window removal and child collapsing
    ImGuiDockNodeTreeInfo info;
    DockNodeFindInfo(node, &info);
    node->CentralNode = info.CentralNode;
    node->OnlyNodeWithWindows = (info.CountNodesWithWindows == 1) ? info.FirstNodeWithWindows : NULL;
    node->CountNodeWithWindows = info.CountNodesWithWindows;
    if (node->LastFocusedNodeId == 0 && info.FirstNodeWithWindows != NULL)
        node->LastFocusedNodeId = info.FirstNodeWithWindows->ID;

    // Copy the window class from of our first window so it can be used for proper dock filtering.
    // When node has mixed windows, prioritize the class with the most constraint (DockingAllowUnclassed = false) as the reference to copy.
    // FIXME-DOCK: We don't recurse properly, this code could be reworked to work from DockNodeUpdateScanRec.
    if (ImGuiDockNode* first_node_with_windows = info.FirstNodeWithWindows)
    {
        node->WindowClass = first_node_with_windows->Windows[0]->WindowClass;
        for (c_int n = 1; n < first_node_with_windows->Windows.Size; n++)
            if (first_node_with_windows->Windows[n]->WindowClass.DockingAllowUnclassed == false)
            {
                node->WindowClass = first_node_with_windows->Windows[n]->WindowClass;
                break;
            }
    }

    ImGuiDockNode* mark_node = node->CentralNode;
    while (mark_node)
    {
        mark_node->HasCentralNodeChild = true;
        mark_node = mark_node->ParentNode;
    }
}

static c_void DockNodeSetupHostWindow(ImGuiDockNode* node, ImGuiWindow* host_window)
{
    // Remove ourselves from any previous different host window
    // This can happen if a user mistakenly does (see #4295 for details):
    //  - N+0: DockBuilderAddNode(id, 0)    // missing ImGuiDockNodeFlags_DockSpace
    //  - N+1: NewFrame()                   // will create floating host window for that node
    //  - N+1: DockSpace(id)                // requalify node as dockspace, moving host window
    if (node->HostWindow && node->HostWindow != host_window && node->Hostwindow.DockNodeAsHost == node)
        node->Hostwindow.DockNodeAsHost = None;

    host_window.DockNodeAsHost = node;
    node->HostWindow = host_window;
}

static c_void ImGui::DockNodeUpdate(ImGuiDockNode* node)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(node->LastFrameActive != g.FrameCount);
    node->LastFrameAlive = g.FrameCount;
    node->IsBgDrawnThisFrame = false;

    node->CentralNode = node->OnlyNodeWithWindows = None;
    if (node->IsRootNode())
        DockNodeUpdateForRootNode(node);

    // Remove tab bar if not needed
    if (node->TabBar && node->IsNoTabBar())
        DockNodeRemoveTabBar(node);

    // Early out for hidden root dock nodes (when all DockId references are in inactive windows, or there is only 1 floating window holding on the DockId)
    let mut want_to_hide_host_window: bool =  false;
    if (node->IsFloatingNode())
    {
        if (node->Windows.Size <= 1 && node->IsLeafNode())
            if (!g.IO.ConfigDockingAlwaysTabBar && (node->Windows.Size == 0 || !node->Windows[0]->WindowClass.DockingAlwaysTabBar))
                want_to_hide_host_window = true;
        if (node->CountNodeWithWindows == 0)
            want_to_hide_host_window = true;
    }
    if (want_to_hide_host_window)
    {
        if (node->Windows.Size == 1)
        {
            // Floating window pos/size is authoritative
            ImGuiWindow* single_window = node->Windows[0];
            node->Pos = single_window.Pos;
            node->Size = single_window.SizeFull;
            node->AuthorityForPos = node->AuthorityForSize = node->AuthorityForViewport = ImGuiDataAuthority_Window;

            // Transfer focus immediately so when we revert to a regular window it is immediately selected
            if (node->HostWindow && g.NavWindow == node->HostWindow)
                FocusWindow(single_window);
            if (node->HostWindow)
            {
                single_window.Viewport = node->Hostwindow.Viewport;
                single_window.ViewportId = node->Hostwindow.ViewportId;
                if (node->Hostwindow.ViewportOwned)
                {
                    single_window.Viewport->Window = single_window;
                    single_window.ViewportOwned = true;
                }
            }
        }

        DockNodeHideHostWindow(node);
        node->State = ImGuiDockNodeState_HostWindowHiddenBecauseSingleWindow;
        node->WantCloseAll = false;
        node->WantCloseTabId = 0;
        node->HasCloseButton = node->HasWindowMenuButton = false;
        node->LastFrameActive = g.FrameCount;

        if (node->WantMouseMove && node->Windows.Size == 1)
            DockNodeStartMouseMovingWindow(node, node->Windows[0]);
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
    if (node->IsVisible && node->HostWindow == NULL && node->IsFloatingNode() && node->IsLeafNode())
    {
        // IM_ASSERT(node->Windows.Size > 0);
        ImGuiWindow* ref_window = None;
        if (node->SelectedTabId != 0) // Note that we prune single-window-node settings on .ini loading, so this is generally 0 for them!
            ref_window = DockNodeFindWindowByID(node, node->SelectedTabId);
        if (ref_window == NULL)
            ref_window = node->Windows[0];
        if (ref_window.AutoFitFramesX > 0 || ref_window.AutoFitFramesY > 0)
        {
            node->State = ImGuiDockNodeState_HostWindowHiddenBecauseWindowsAreResizing;
            return;
        }
    }

    const ImGuiDockNodeFlags node_flags = node->MergedFlags;

    // Decide if the node will have a close button and a window menu button
    node->HasWindowMenuButton = (node->Windows.Size > 0) && (node_flags & ImGuiDockNodeFlags_NoWindowMenuButton) == 0;
    node->HasCloseButton = false;
    for (c_int window_n = 0; window_n < node->Windows.Size; window_n++)
    {
        // FIXME-DOCK: Setting DockIsActive here means that for single active window in a leaf node, DockIsActive will be cleared until the next Begin() call.
        ImGuiWindow* window = node->Windows[window_n];
        node->HasCloseButton |= window.HasCloseButton;
        window.DockIsActive = (node->Windows.Size > 1);
    }
    if (node_flags & ImGuiDockNodeFlags_NoCloseButton)
        node->HasCloseButton = false;

    // Bind or create host window
    ImGuiWindow* host_window = None;
    let mut beginned_into_host_window: bool =  false;
    if (node->IsDockSpace())
    {
        // [Explicit root dockspace node]
        // IM_ASSERT(node->HostWindow);
        host_window = node->HostWindow;
    }
    else
    {
        // [Automatic root or child nodes]
        if (node->IsRootNode() && node->IsVisible)
        {
            ImGuiWindow* ref_window = (node->Windows.Size > 0) ? node->Windows[0] : NULL;

            // Sync Pos
            if (node->AuthorityForPos == ImGuiDataAuthority_Window && ref_window)
                SetNextWindowPos(ref_window.Pos);
            else if (node->AuthorityForPos == ImGuiDataAuthority_DockNode)
                SetNextWindowPos(node->Pos);

            // Sync Size
            if (node->AuthorityForSize == ImGuiDataAuthority_Window && ref_window)
                SetNextWindowSize(ref_window.SizeFull);
            else if (node->AuthorityForSize == ImGuiDataAuthority_DockNode)
                SetNextWindowSize(node->Size);

            // Sync Collapsed
            if (node->AuthorityForSize == ImGuiDataAuthority_Window && ref_window)
                SetNextWindowCollapsed(ref_window.Collapsed);

            // Sync Viewport
            if (node->AuthorityForViewport == ImGuiDataAuthority_Window && ref_window)
                SetNextWindowViewport(ref_window.ViewportId);

            SetNextWindowClass(&node->WindowClass);

            // Begin into the host window
            window_label: [c_char;20];
            DockNodeGetHostWindowTitle(node, window_label, IM_ARRAYSIZE(window_label));
            ImGuiWindowFlags window_flags = ImGuiWindowFlags_NoScrollbar | ImGuiWindowFlags_NoScrollWithMouse | ImGuiWindowFlags_DockNodeHost;
            window_flags |= ImGuiWindowFlags_NoFocusOnAppearing;
            window_flags |= ImGuiWindowFlags_NoSavedSettings | ImGuiWindowFlags_NoNavFocus | ImGuiWindowFlags_NoCollapse;
            window_flags |= ImGuiWindowFlags_NoTitleBar;

            SetNextWindowBgAlpha(0f32); // Don't set ImGuiWindowFlags_NoBackground because it disables borders
            PushStyleVar(ImGuiStyleVar_WindowPadding, ImVec2(0, 0));
            Begin(window_label, NULL, window_flags);
            PopStyleVar();
            beginned_into_host_window = true;

            host_window = g.CurrentWindow;
            DockNodeSetupHostWindow(node, host_window);
            host_window.DC.CursorPos = host_window.Pos;
            node->Pos = host_window.Pos;
            node->Size = host_window.Size;

            // We set ImGuiWindowFlags_NoFocusOnAppearing because we don't want the host window to take full focus (e.g. steal NavWindow)
            // But we still it bring it to the front of display. There's no way to choose this precise behavior via window flags.
            // One simple case to ponder if: window A has a toggle to create windows B/C/D. Dock B/C/D together, clear the toggle and enable it again.
            // When reappearing B/C/D will request focus and be moved to the top of the display pile, but they are not linked to the dock host window
            // during the frame they appear. The dock host window would keep its old display order, and the sorting in EndFrame would move B/C/D back
            // after the dock host window, losing their top-most status.
            if (node->Hostwindow.Appearing)
                BringWindowToDisplayFront(node->HostWindow);

            node->AuthorityForPos = node->AuthorityForSize = node->AuthorityForViewport = ImGuiDataAuthority_Auto;
        }
        else if (node->ParentNode)
        {
            node->HostWindow = host_window = node->ParentNode->HostWindow;
            node->AuthorityForPos = node->AuthorityForSize = node->AuthorityForViewport = ImGuiDataAuthority_Auto;
        }
        if (node->WantMouseMove && node->HostWindow)
            DockNodeStartMouseMovingWindow(node, node->HostWindow);
    }

    // Update focused node (the one whose title bar is highlight) within a node tree
    if (node->IsSplitNode())
        // IM_ASSERT(node->TabBar == NULL);
    if (node->IsRootNode())
        if (g.NavWindow && g.Navwindow.Rootwindow.DockNode && g.Navwindow.Rootwindow.ParentWindow == host_window)
            node->LastFocusedNodeId = g.Navwindow.Rootwindow.DockNode->ID;

    // Register a hit-test hole in the window unless we are currently dragging a window that is compatible with our dockspace
    ImGuiDockNode* central_node = node->CentralNode;
    let central_node_hole: bool = node->IsRootNode() && host_window && (node_flags & ImGuiDockNodeFlags_PassthruCentralNode) != 0 && central_node != NULL && central_node->IsEmpty();
    let mut central_node_hole_register_hit_test_hole: bool =  central_node_hole;
    if (central_node_hole)
        if (*const ImGuiPayload payload = ImGui::GetDragDropPayload())
            if (payload->IsDataType(IMGUI_PAYLOAD_TYPE_WINDOW) && DockNodeIsDropAllowed(host_window, *(ImGuiWindow**)payload->Data))
                central_node_hole_register_hit_test_hole = false;
    if (central_node_hole_register_hit_test_hole)
    {
        // We add a little padding to match the "resize from edges" behavior and allow grabbing the splitter easily.
        // (But we only add it if there's something else on the other side of the hole, otherwise for e.g. fullscreen
        // covering passthru node we'd have a gap on the edge not covered by the hole)
        // IM_ASSERT(node->IsDockSpace()); // We cannot pass this flag without the DockSpace() api. Testing this because we also setup the hole in host_window.ParentNode
        ImGuiDockNode* root_node = DockNodeGetRootNode(central_node);
        ImRect root_rect(root_node->Pos, root_node->Pos + root_node->Size);
        ImRect hole_rect(central_node->Pos, central_node->Pos + central_node->Size);
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
    if (node->IsRootNode() && host_window)
    {
        DockNodeTreeUpdatePosSize(node, host_window.Pos, host_window.Size);
        DockNodeTreeUpdateSplitter(node);
    }

    // Draw empty node background (currently can only be the Central Node)
    if (host_window && node->IsEmpty() && node->IsVisible)
    {
        host_window.DrawList.ChannelsSetCurrent(DOCKING_HOST_DRAW_CHANNEL_BG);
        node->LastBgColor = (node_flags & ImGuiDockNodeFlags_PassthruCentralNode) ? 0 : GetColorU32(ImGuiCol_DockingEmptyBg);
        if (node->LastBgColor != 0)
            host_window.DrawList.AddRectFilled(node->Pos, node->Pos + node->Size, node->LastBgColor);
        node->IsBgDrawnThisFrame = true;
    }

    // Draw whole dockspace background if ImGuiDockNodeFlags_PassthruCentralNode if set.
    // We need to draw a background at the root level if requested by ImGuiDockNodeFlags_PassthruCentralNode, but we will only know the correct pos/size
    // _after_ processing the resizing splitters. So we are using the DrawList channel splitting facility to submit drawing primitives out of order!
    let render_dockspace_bg: bool = node->IsRootNode() && host_window && (node_flags & ImGuiDockNodeFlags_PassthruCentralNode) != 0;
    if (render_dockspace_bg && node->IsVisible)
    {
        host_window.DrawList.ChannelsSetCurrent(DOCKING_HOST_DRAW_CHANNEL_BG);
        if (central_node_hole)
            RenderRectFilledWithHole(host_window.DrawList, node->Rect(), central_node->Rect(), GetColorU32(ImGuiCol_WindowBg), 0f32);
        else
            host_window.DrawList.AddRectFilled(node->Pos, node->Pos + node->Size, GetColorU32(ImGuiCol_WindowBg), 0f32);
    }

    // Draw and populate Tab Bar
    if (host_window)
        host_window.DrawList.ChannelsSetCurrent(DOCKING_HOST_DRAW_CHANNEL_FG);
    if (host_window && node->Windows.Size > 0)
    {
        DockNodeUpdateTabBar(node, host_window);
    }
    else
    {
        node->WantCloseAll = false;
        node->WantCloseTabId = 0;
        node->IsFocused = false;
    }
    if (node->TabBar && node->TabBar->SelectedTabId)
        node->SelectedTabId = node->TabBar->SelectedTabId;
    else if (node->Windows.Size > 0)
        node->SelectedTabId = node->Windows[0]->TabId;

    // Draw payload drop target
    if (host_window && node->IsVisible)
        if (node->IsRootNode() && (g.MovingWindow == NULL || g.Movingwindow.RootWindowDockTree != host_window))
            BeginDockableDragDropTarget(host_window);

    // We update this after DockNodeUpdateTabBar()
    node->LastFrameActive = g.FrameCount;

    // Recurse into children
    // FIXME-DOCK FIXME-OPT: Should not need to recurse into children
    if (host_window)
    {
        if (node->ChildNodes[0])
            DockNodeUpdate(node->ChildNodes[0]);
        if (node->ChildNodes[1])
            DockNodeUpdate(node->ChildNodes[1]);

        // Render outer borders last (after the tab bar)
        if (node->IsRootNode())
            RenderWindowOuterBorders(host_window);
    }

    // End host window
    if (beginned_into_host_window) //-V1020
        End();
}

// Compare TabItem nodes given the last known DockOrder (will persist in .ini file as hint), used to sort tabs when multiple tabs are added on the same frame.
static c_int IMGUI_CDECL TabItemComparerByDockOrder(*const c_void lhs, *const c_void rhs)
{
    ImGuiWindow* a = ((*const ImGuiTabItem)lhs)->Window;
    ImGuiWindow* b = ((*const ImGuiTabItem)rhs)->Window;
    if (c_int d = ((a->DockOrder == -1) ? INT_MAX : a->DockOrder) - ((b->DockOrder == -1) ? INT_MAX : b->DockOrder))
        return d;
    return (a->BeginOrderWithinContext - b->BeginOrderWithinContext);
}

static ImGuiID ImGui::DockNodeUpdateWindowMenu(ImGuiDockNode* node, ImGuiTabBar* tab_bar)
{
    // Try to position the menu so it is more likely to stays within the same viewport
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiID ret_tab_id = 0;
    if (g.Style.WindowMenuButtonPosition == ImGuiDir_Left)
        SetNextWindowPos(ImVec2(node->Pos.x, node->Pos.y + GetFrameHeight()), ImGuiCond_Always, ImVec2(0f32, 0f32));
    else
        SetNextWindowPos(ImVec2(node->Pos.x + node->Size.x, node->Pos.y + GetFrameHeight()), ImGuiCond_Always, ImVec2(1f32, 0f32));
    if (BeginPopup("#WindowMenu"))
    {
        node->IsFocused = true;
        if (tab_bar->Tabs.Size == 1)
        {
            if (MenuItem("Hide tab bar", NULL, node->IsHiddenTabBar()))
                node->WantHiddenTabBarToggle = true;
        }
        else
        {
            for (c_int tab_n = 0; tab_n < tab_bar->Tabs.Size; tab_n++)
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
bool ImGui::DockNodeBeginAmendTabBar(ImGuiDockNode* node)
{
    if (node->TabBar == NULL || node->HostWindow == NULL)
        return false;
    if (node->MergedFlags & ImGuiDockNodeFlags_KeepAliveOnly)
        return false;
    Begin(node->Hostwindow.Name);
    PushOverrideID(node->ID);
    let mut ret: bool =  BeginTabBarEx(node->TabBar, node->TabBar->BarRect, node->TabBar->Flags, node);
    IM_UNUSED(ret);
    // IM_ASSERT(ret);
    return true;
}

c_void ImGui::DockNodeEndAmendTabBar()
{
    EndTabBar();
    PopID();
    End();
}

static bool IsDockNodeTitleBarHighlighted(ImGuiDockNode* node, ImGuiDockNode* root_node, ImGuiWindow* host_window)
{
    // CTRL+Tab highlight (only highlighting leaf node, not whole hierarchy)
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.NavWindowingTarget)
        return (g.NavWindowingTarget->DockNode == node);

    // FIXME-DOCKING: May want alternative to treat central node void differently? e.g. if (g.NavWindow == host_window)
    if (g.NavWindow && g.Navwindow.RootWindowForTitleBarHighlight == host_window.RootWindowDockTree && root_node->LastFocusedNodeId == node->ID)
        for (ImGuiDockNode* parent_node = g.Navwindow.Rootwindow.DockNode; parent_node != None; parent_node = parent_node->HostWindow ? parent_node->Hostwindow.Rootwindow.DockNode : NULL)
            if ((parent_node = ImGui::DockNodeGetRootNode(parent_node)) == root_node)
                return true;
    return false;
}

// Submit the tab bar corresponding to a dock node and various housekeeping details.
static c_void ImGui::DockNodeUpdateTabBar(ImGuiDockNode* node, ImGuiWindow* host_window)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiStyle& style = g.Style;

    let node_was_active: bool = (node->LastFrameActive + 1 == g.FrameCount);
    let closed_all: bool = node->WantCloseAll && node_was_active;
    const ImGuiID closed_one = node->WantCloseTabId && node_was_active;
    node->WantCloseAll = false;
    node->WantCloseTabId = 0;

    // Decide if we should use a focused title bar color
    let mut is_focused: bool =  false;
    ImGuiDockNode* root_node = DockNodeGetRootNode(node);
    if (IsDockNodeTitleBarHighlighted(node, root_node, host_window))
        is_focused = true;

    // Hidden tab bar will show a triangle on the upper-left (in Begin)
    if (node->IsHiddenTabBar() || node->IsNoTabBar())
    {
        node->VisibleWindow = (node->Windows.Size > 0) ? node->Windows[0] : NULL;
        node->IsFocused = is_focused;
        if (is_focused)
            node->LastFrameFocused = g.FrameCount;
        if (node->VisibleWindow)
        {
            // Notify root of visible window (used to display title in OS task bar)
            if (is_focused || root_node->VisibleWindow == NULL)
                root_node->VisibleWindow = node->VisibleWindow;
            if (node->TabBar)
                node->TabBar->VisibleTabId = node->Visiblewindow.TabId;
        }
        return;
    }

    // Move ourselves to the Menu layer (so we can be accessed by tapping Alt) + undo SkipItems flag in order to draw over the title bar even if the window is collapsed
    let mut backup_skip_item: bool =  host_window.SkipItems;
    if (!node->IsDockSpace())
    {
        host_window.SkipItems = false;
        host_window.DC.NavLayerCurrent = ImGuiNavLayer_Menu;
    }

    // Use PushOverrideID() instead of PushID() to use the node id _without_ the host window ID.
    // This is to facilitate computing those ID from the outside, and will affect more or less only the ID of the collapse button, popup and tabs,
    // as docked windows themselves will override the stack with their own root ID.
    PushOverrideID(node->ID);
    ImGuiTabBar* tab_bar = node->TabBar;
    let mut tab_bar_is_recreated: bool =  (tab_bar == NULL); // Tab bar are automatically destroyed when a node gets hidden
    if (tab_bar == NULL)
    {
        DockNodeAddTabBar(node);
        tab_bar = node->TabBar;
    }

    ImGuiID focus_tab_id = 0;
    node->IsFocused = is_focused;

    const ImGuiDockNodeFlags node_flags = node->MergedFlags;
    let has_window_menu_button: bool = (node_flags & ImGuiDockNodeFlags_NoWindowMenuButton) == 0 && (style.WindowMenuButtonPosition != ImGuiDir_None);

    // In a dock node, the Collapse Button turns into the Window Menu button.
    // FIXME-DOCK FIXME-OPT: Could we recycle popups id across multiple dock nodes?
    if (has_window_menu_button && IsPopupOpen("#WindowMenu"))
    {
        if (ImGuiID tab_id = DockNodeUpdateWindowMenu(node, tab_bar))
            focus_tab_id = tab_bar->NextSelectedTabId = tab_id;
        is_focused |= node->IsFocused;
    }

    // Layout
    ImRect title_bar_rect, tab_bar_rect;
    ImVec2 window_menu_button_pos;
    ImVec2 close_button_pos;
    DockNodeCalcTabBarLayout(node, &title_bar_rect, &tab_bar_rect, &window_menu_button_pos, &close_button_pos);

    // Submit new tabs, they will be added as Unsorted and sorted below based on relative DockOrder value.
    let tabs_count_old: c_int = tab_bar->Tabs.Size;
    for (c_int window_n = 0; window_n < node->Windows.Size; window_n++)
    {
        ImGuiWindow* window = node->Windows[window_n];
        if (TabBarFindTabByID(tab_bar, window.TabId) == NULL)
            TabBarAddTab(tab_bar, ImGuiTabItemFlags_Unsorted, window);
    }

    // Title bar
    if (is_focused)
        node->LastFrameFocused = g.FrameCount;
    u32 title_bar_col = GetColorU32(host_window.Collapsed ? ImGuiCol_TitleBgCollapsed : is_focused ? ImGuiCol_TitleBgActive : ImGuiCol_TitleBg);
    ImDrawFlags rounding_flags = CalcRoundingFlagsForRectInRect(title_bar_rect, host_window.Rect(), DOCKING_SPLITTER_SIZE);
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
    c_int tabs_unsorted_start = tab_bar->Tabs.Size;
    for (c_int tab_n = tab_bar->Tabs.Size - 1; tab_n >= 0 && (tab_bar->Tabs[tab_n].Flags & ImGuiTabItemFlags_Unsorted); tab_n--)
    {
        // FIXME-DOCK: Consider only clearing the flag after the tab has been alive for a few consecutive frames, allowing late comers to not break sorting?
        tab_bar->Tabs[tab_n].Flags &= ~ImGuiTabItemFlags_Unsorted;
        tabs_unsorted_start = tab_n;
    }
    if (tab_bar->Tabs.Size > tabs_unsorted_start)
    {
        IMGUI_DEBUG_LOG_DOCKING("[docking] In node 0x%08X: %d new appearing tabs:%s\n", node->ID, tab_bar->Tabs.Size - tabs_unsorted_start, (tab_bar->Tabs.Size > tabs_unsorted_start + 1) ? " (will sort)" : "");
        for (c_int tab_n = tabs_unsorted_start; tab_n < tab_bar->Tabs.Size; tab_n++)
            IMGUI_DEBUG_LOG_DOCKING("[docking] - Tab '%s' Order %d\n", tab_bar->Tabs[tab_n].window.Name, tab_bar->Tabs[tab_n].window.DockOrder);
        if (tab_bar->Tabs.Size > tabs_unsorted_start + 1)
            ImQsort(tab_bar->Tabs.Data + tabs_unsorted_start, tab_bar->Tabs.Size - tabs_unsorted_start, sizeof(ImGuiTabItem), TabItemComparerByDockOrder);
    }

    // Apply NavWindow focus back to the tab bar
    if (g.NavWindow && g.Navwindow.Rootwindow.DockNode == node)
        tab_bar->SelectedTabId = g.Navwindow.Rootwindow.TabId;

    // Selected newly added tabs, or persistent tab ID if the tab bar was just recreated
    if (tab_bar_is_recreated && TabBarFindTabByID(tab_bar, node->SelectedTabId) != NULL)
        tab_bar->SelectedTabId = tab_bar->NextSelectedTabId = node->SelectedTabId;
    else if (tab_bar->Tabs.Size > tabs_count_old)
        tab_bar->SelectedTabId = tab_bar->NextSelectedTabId = tab_bar->Tabs.back().window.TabId;

    // Begin tab bar
    ImGuiTabBarFlags tab_bar_flags = ImGuiTabBarFlags_Reorderable | ImGuiTabBarFlags_AutoSelectNewTabs; // | ImGuiTabBarFlags_NoTabListScrollingButtons);
    tab_bar_flags |= ImGuiTabBarFlags_SaveSettings | ImGuiTabBarFlags_DockNode;
    if (!host_window.Collapsed && is_focused)
        tab_bar_flags |= ImGuiTabBarFlags_IsFocused;
    BeginTabBarEx(tab_bar, tab_bar_rect, tab_bar_flags, node);
    //host_window.DrawList.AddRect(tab_bar_rect.Min, tab_bar_rect.Max, IM_COL32(255,0,255,255));

    // Backup style colors
    ImVec4 backup_style_cols[ImGuiWindowDockStyleCol_COUNT];
    for (c_int color_n = 0; color_n < ImGuiWindowDockStyleCol_COUNT; color_n++)
        backup_style_cols[color_n] = g.Style.Colors[GWindowDockStyleColors[color_n]];

    // Submit actual tabs
    node->VisibleWindow = None;
    for (c_int window_n = 0; window_n < node->Windows.Size; window_n++)
    {
        ImGuiWindow* window = node->Windows[window_n];
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
            for (c_int color_n = 0; color_n < ImGuiWindowDockStyleCol_COUNT; color_n++)
                g.Style.Colors[GWindowDockStyleColors[color_n]] = ColorConvertU32ToFloat4(window.DockStyle.Colors[color_n]);

            // Note that TabItemEx() calls TabBarCalcTabID() so our tab item ID will ignore the current ID stack (rightly so)
            let mut tab_open: bool =  true;
            TabItemEx(tab_bar, window.Name, window.HasCloseButton ? &tab_open : NULL, tab_item_flags, window);
            if (!tab_open)
                node->WantCloseTabId = window.TabId;
            if (tab_bar->VisibleTabId == window.TabId)
                node->VisibleWindow = window;

            // Store last item data so it can be queried with IsItemXXX functions after the user Begin() call
            window.DockTabItemStatusFlags = g.LastItemData.StatusFlags;
            window.DockTabItemRect = g.LastItemData.Rect;

            // Update navigation ID on menu layer
            if (g.NavWindow && g.Navwindow.RootWindow == window && (window.DC.NavLayersActiveMask & (1 << ImGuiNavLayer_Menu)) == 0)
                host_window.NavLastIds[1] = window.TabId;
        }
    }

    // Restore style colors
    for (c_int color_n = 0; color_n < ImGuiWindowDockStyleCol_COUNT; color_n++)
        g.Style.Colors[GWindowDockStyleColors[color_n]] = backup_style_cols[color_n];

    // Notify root of visible window (used to display title in OS task bar)
    if (node->VisibleWindow)
        if (is_focused || root_node->VisibleWindow == NULL)
            root_node->VisibleWindow = node->VisibleWindow;

    // Close button (after VisibleWindow was updated)
    // Note that VisibleWindow may have been overrided by CTRL+Tabbing, so Visiblewindow.TabId may be != from tab_bar->SelectedTabId
    let close_button_is_enabled: bool = node->HasCloseButton && node->VisibleWindow && node->Visiblewindow.HasCloseButton;
    let close_button_is_visible: bool = node->HasCloseButton;
    //let close_button_is_visible: bool = close_button_is_enabled; // Most people would expect this behavior of not even showing the button (leaving a hole since we can't claim that space as other windows in the tba bar have one)
    if (close_button_is_visible)
    {
        if (!close_button_is_enabled)
        {
            PushItemFlag(ImGuiItemFlags_Disabled, true);
            PushStyleColor(ImGuiCol_Text, style.Colors[ImGuiCol_Text] * ImVec4(1f32,1f32,1f32,0.40f32));
        }
        if (CloseButton(host_window.GetID("#CLOSE"), close_button_pos))
        {
            node->WantCloseAll = true;
            for (c_int n = 0; n < tab_bar->Tabs.Size; n++)
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
    ImGuiID title_bar_id = host_window.GetID("#TITLEBAR");
    if (g.HoveredId == 0 || g.HoveredId == title_bar_id || g.ActiveId == title_bar_id)
    {
        bool held;
        ButtonBehavior(title_bar_rect, title_bar_id, NULL, &held, ImGuiButtonFlags_AllowItemOverlap);
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
                StartMouseMovingWindowOrNode(tab->Window ? tab->Window : node->HostWindow, node, false);
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
    if (!node->IsDockSpace())
    {
        host_window.DC.NavLayerCurrent = ImGuiNavLayer_Main;
        host_window.SkipItems = backup_skip_item;
    }
}

static c_void ImGui::DockNodeAddTabBar(ImGuiDockNode* node)
{
    // IM_ASSERT(node->TabBar == NULL);
    node->TabBar = IM_NEW(ImGuiTabBar);
}

static c_void ImGui::DockNodeRemoveTabBar(ImGuiDockNode* node)
{
    if (node->TabBar == NULL)
        return;
    IM_DELETE(node->TabBar);
    node->TabBar = None;
}

static bool DockNodeIsDropAllowedOne(ImGuiWindow* payload, ImGuiWindow* host_window)
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
    // by e.g. adding a 'if (!ImGui::IsWindowWithinBeginStackOf(host_window, popup_window))' test.
    // But it would requires more work on our end because the dock host windows is technically created in NewFrame()
    // and our ->ParentXXX and ->RootXXX pointers inside windows are currently mislading or lacking.
    let g = GImGui; // ImGuiContext& g = *GImGui;
    for (c_int i = g.OpenPopupStack.Size - 1; i >= 0; i--)
        if (ImGuiWindow* popup_window = g.OpenPopupStack[i].Window)
            if (ImGui::IsWindowWithinBeginStackOf(payload, popup_window))   // Payload is created from within a popup begin stack.
                return false;

    return true;
}

static bool ImGui::DockNodeIsDropAllowed(ImGuiWindow* host_window, ImGuiWindow* root_payload)
{
    if (root_payload->DockNodeAsHost && root_payload->DockNodeAsHost->IsSplitNode()) // FIXME-DOCK: Missing filtering
        return true;

    let payload_count: c_int = root_payload->DockNodeAsHost ? root_payload->DockNodeAsHost->Windows.Size : 1;
    for (c_int payload_n = 0; payload_n < payload_count; payload_n++)
    {
        ImGuiWindow* payload = root_payload->DockNodeAsHost ? root_payload->DockNodeAsHost->Windows[payload_n] : root_payload;
        if (DockNodeIsDropAllowedOne(payload, host_window))
            return true;
    }
    return false;
}

// window menu button == collapse button when not in a dock node.
// FIXME: This is similar to RenderWindowTitleBarContents(), may want to share code.
static c_void ImGui::DockNodeCalcTabBarLayout(*const ImGuiDockNode node, ImRect* out_title_rect, ImRect* out_tab_bar_rect, ImVec2* out_window_menu_button_pos, ImVec2* out_close_button_pos)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiStyle& style = g.Style;

    ImRect r = ImRect(node->Pos.x, node->Pos.y, node->Pos.x + node->Size.x, node->Pos.y + g.FontSize + g.Style.FramePadding.y * 2.00f32);
    if (out_title_rect) { *out_title_rect = r; }

    r.Min.x += style.WindowBorderSize;
    r.Max.x -= style.WindowBorderSize;

    c_float button_sz = g.FontSize;

    ImVec2 window_menu_button_pos = r.Min;
    r.Min.x += style.FramePadding.x;
    r.Max.x -= style.FramePadding.x;
    if (node->HasCloseButton)
    {
        r.Max.x -= button_sz;
        if (out_close_button_pos) *out_close_button_pos = ImVec2(r.Max.x - style.FramePadding.x, r.Min.y);
    }
    if (node->HasWindowMenuButton && style.WindowMenuButtonPosition == ImGuiDir_Left)
    {
        r.Min.x += button_sz + style.ItemInnerSpacing.x;
    }
    else if (node->HasWindowMenuButton && style.WindowMenuButtonPosition == ImGuiDir_Right)
    {
        r.Max.x -= button_sz + style.FramePadding.x;
        window_menu_button_pos = ImVec2(r.Max.x, r.Min.y);
    }
    if (out_tab_bar_rect) { *out_tab_bar_rect = r; }
    if (out_window_menu_button_pos) { *out_window_menu_button_pos = window_menu_button_pos; }
}

c_void ImGui::DockNodeCalcSplitRects(ImVec2& pos_old, ImVec2& size_old, ImVec2& pos_new, ImVec2& size_new, ImGuiDir dir, ImVec2 size_new_desired)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    const c_float dock_spacing = g.Style.ItemInnerSpacing.x;
    const ImGuiAxis axis = (dir == ImGuiDir_Left || dir == ImGuiDir_Right) ? ImGuiAxis_X : ImGuiAxis_Y;
    pos_new[axis ^ 1] = pos_old[axis ^ 1];
    size_new[axis ^ 1] = size_old[axis ^ 1];

    // Distribute size on given axis (with a desired size or equally)
    const c_float w_avail = size_old[axis] - dock_spacing;
    if (size_new_desired[axis] > 0f32 && size_new_desired[axis] <= w_avail * 0.5f32)
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
bool ImGui::DockNodeCalcDropRectsAndTestMousePos(const ImRect& parent, ImGuiDir dir, ImRect& out_r, bool outer_docking, ImVec2* test_mouse_pos)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    const c_float parent_smaller_axis = ImMin(parent.GetWidth(), parent.GetHeight());
    const c_float hs_for_central_nodes = ImMin(g.FontSize * 1.5f32, ImMax(g.FontSize * 0.5f32, parent_smaller_axis / 8.00f32));
    c_float hs_w; // Half-size, longer axis
    c_float hs_h; // Half-size, smaller axis
    ImVec2 off; // Distance from edge or center
    if (outer_docking)
    {
        //hs_w = ImFloor(ImClamp(parent_smaller_axis - hs_for_central_nodes * 4.0f32, g.FontSize * 0.5f32, g.FontSize * 8.00f32));
        //hs_h = ImFloor(hs_w * 0.150f32);
        //off = ImVec2(ImFloor(parent.GetWidth() * 0.5f32 - GetFrameHeightWithSpacing() * 1.4f - hs_h), ImFloor(parent.GetHeight() * 0.5f32 - GetFrameHeightWithSpacing() * 1.4f - hs_h));
        hs_w = ImFloor(hs_for_central_nodes * 1.500f32);
        hs_h = ImFloor(hs_for_central_nodes * 0.800f32);
        off = ImVec2(ImFloor(parent.GetWidth() * 0.5f32 - hs_h), ImFloor(parent.GetHeight() * 0.5f32 - hs_h));
    }
    else
    {
        hs_w = ImFloor(hs_for_central_nodes);
        hs_h = ImFloor(hs_for_central_nodes * 0.900f32);
        off = ImVec2(ImFloor(hs_w * 2.400f32), ImFloor(hs_w * 2.400f32));
    }

    ImVec2 c = ImFloor(parent.GetCenter());
    if      (dir == ImGuiDir_None)  { out_r = ImRect(c.x - hs_w, c.y - hs_w,         c.x + hs_w, c.y + hs_w);         }
    else if (dir == ImGuiDir_Up)    { out_r = ImRect(c.x - hs_w, c.y - off.y - hs_h, c.x + hs_w, c.y - off.y + hs_h); }
    else if (dir == ImGuiDir_Down)  { out_r = ImRect(c.x - hs_w, c.y + off.y - hs_h, c.x + hs_w, c.y + off.y + hs_h); }
    else if (dir == ImGuiDir_Left)  { out_r = ImRect(c.x - off.x - hs_h, c.y - hs_w, c.x - off.x + hs_h, c.y + hs_w); }
    else if (dir == ImGuiDir_Right) { out_r = ImRect(c.x + off.x - hs_h, c.y - hs_w, c.x + off.x + hs_h, c.y + hs_w); }

    if (test_mouse_pos == NULL)
        return false;

    ImRect hit_r = out_r;
    if (!outer_docking)
    {
        // Custom hit testing for the 5-way selection, designed to reduce flickering when moving diagonally between sides
        hit_r.Expand(ImFloor(hs_w * 0.300f32));
        ImVec2 mouse_delta = (*test_mouse_pos - c);
        c_float mouse_delta_len2 = ImLengthSqr(mouse_delta);
        c_float r_threshold_center = hs_w * 1.4f;
        c_float r_threshold_sides = hs_w * (1.4f + 1.20f32);
        if (mouse_delta_len2 < r_threshold_center * r_threshold_center)
            return (dir == ImGuiDir_None);
        if (mouse_delta_len2 < r_threshold_sides * r_threshold_sides)
            return (dir == ImGetDirQuadrantFromDelta(mouse_delta.x, mouse_delta.y));
    }
    return hit_r.Contains(*test_mouse_pos);
}

// host_node may be NULL if the window doesn't have a DockNode already.
// FIXME-DOCK: This is misnamed since it's also doing the filtering.
static c_void ImGui::DockNodePreviewDockSetup(ImGuiWindow* host_window, ImGuiDockNode* host_node, ImGuiWindow* payload_window, ImGuiDockNode* payload_node, ImGuiDockPreviewData* data, bool is_explicit_target, bool is_outer_docking)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    // There is an edge case when docking into a dockspace which only has inactive nodes.
    // In this case DockNodeTreeFindNodeByPos() will have selected a leaf node which is inactive.
    // Because the inactive leaf node doesn't have proper pos/size yet, we'll use the root node as reference.
    if (payload_node == NULL)
        payload_node = payload_window.DockNodeAsHost;
    ImGuiDockNode* ref_node_for_rect = (host_node && !host_node->IsVisible) ? DockNodeGetRootNode(host_node) : host_node;
    if (ref_node_for_rect)
        // IM_ASSERT(ref_node_for_rect->IsVisible == true);

    // Filter, figure out where we are allowed to dock
    ImGuiDockNodeFlags src_node_flags = payload_node ? payload_node->MergedFlags : payload_window.WindowClass.DockNodeFlagsOverrideSet;
    ImGuiDockNodeFlags dst_node_flags = host_node ? host_node->MergedFlags : host_window.WindowClass.DockNodeFlagsOverrideSet;
    data.IsCenterAvailable = true;
    if (is_outer_docking)
        data.IsCenterAvailable = false;
    else if (dst_node_flags & ImGuiDockNodeFlags_NoDocking)
        data.IsCenterAvailable = false;
    else if (host_node && (dst_node_flags & ImGuiDockNodeFlags_NoDockingInCentralNode) && host_node->IsCentralNode())
        data.IsCenterAvailable = false;
    else if ((!host_node || !host_node->IsEmpty()) && payload_node && payload_node->IsSplitNode() && (payload_node->OnlyNodeWithWindows == NULL)) // Is _visibly_ split?
        data.IsCenterAvailable = false;
    else if (dst_node_flags & ImGuiDockNodeFlags_NoDockingOverMe)
        data.IsCenterAvailable = false;
    else if ((src_node_flags & ImGuiDockNodeFlags_NoDockingOverOther) && (!host_node || !host_node->IsEmpty()))
        data.IsCenterAvailable = false;
    else if ((src_node_flags & ImGuiDockNodeFlags_NoDockingOverEmpty) && host_node && host_node->IsEmpty())
        data.IsCenterAvailable = false;

    data.IsSidesAvailable = true;
    if ((dst_node_flags & ImGuiDockNodeFlags_NoSplit) || g.IO.ConfigDockingNoSplit)
        data.IsSidesAvailable = false;
    else if (!is_outer_docking && host_node && host_node->ParentNode == NULL && host_node->IsCentralNode())
        data.IsSidesAvailable = false;
    else if ((dst_node_flags & ImGuiDockNodeFlags_NoDockingSplitMe) || (src_node_flags & ImGuiDockNodeFlags_NoDockingSplitOther))
        data.IsSidesAvailable = false;

    // Build a tentative future node (reuse same structure because it is practical. Shape will be readjusted when previewing a split)
    data.FutureNode.HasCloseButton = (host_node ? host_node->HasCloseButton : host_window.HasCloseButton) || (payload_window.HasCloseButton);
    data.FutureNode.HasWindowMenuButton = host_node ? true : ((host_window.Flags & ImGuiWindowFlags_NoCollapse) == 0);
    data.FutureNode.Pos = ref_node_for_rect ? ref_node_for_rect->Pos : host_window.Pos;
    data.FutureNode.Size = ref_node_for_rect ? ref_node_for_rect->Size : host_window.Size;

    // Calculate drop shapes geometry for allowed splitting directions
    // IM_ASSERT(ImGuiDir_None == -1);
    data.SplitNode = host_node;
    data.SplitDir = ImGuiDir_None;
    data.IsSplitDirExplicit = false;
    if (!host_window.Collapsed)
        for (c_int dir = ImGuiDir_None; dir < ImGuiDir_COUNT; dir++)
        {
            if (dir == ImGuiDir_None && !data.IsCenterAvailable)
                continue;
            if (dir != ImGuiDir_None && !data.IsSidesAvailable)
                continue;
            if (DockNodeCalcDropRectsAndTestMousePos(data.FutureNode.Rect(), (ImGuiDir)dir, data.DropRectsDraw[dir+1], is_outer_docking, &g.IO.MousePos))
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
    data.SplitRatio = 0f32;
    if (data.SplitDir != ImGuiDir_None)
    {
        ImGuiDir split_dir = data.SplitDir;
        ImGuiAxis split_axis = (split_dir == ImGuiDir_Left || split_dir == ImGuiDir_Right) ? ImGuiAxis_X : ImGuiAxis_Y;
        ImVec2 pos_new, pos_old = data.FutureNode.Pos;
        ImVec2 size_new, size_old = data.FutureNode.Size;
        DockNodeCalcSplitRects(pos_old, size_old, pos_new, size_new, split_dir, payload_window.Size);

        // Calculate split ratio so we can pass it down the docking request
        c_float split_ratio = ImSaturate(size_new[split_axis] / data.FutureNode.Size[split_axis]);
        data.FutureNode.Pos = pos_new;
        data.FutureNode.Size = size_new;
        data.SplitRatio = (split_dir == ImGuiDir_Right || split_dir == ImGuiDir_Down) ? (1f32 - split_ratio) : (split_ratio);
    }
}

static c_void ImGui::DockNodePreviewDockRender(ImGuiWindow* host_window, ImGuiDockNode* host_node, ImGuiWindow* root_payload, *const ImGuiDockPreviewData data)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.CurrentWindow == host_window);   // Because we rely on font size to calculate tab sizes

    // With this option, we only display the preview on the target viewport, and the payload viewport is made transparent.
    // To compensate for the single layer obstructed by the payload, we'll increase the alpha of the preview nodes.
    let is_transparent_payload: bool = g.IO.ConfigDockingTransparentPayload;

    // In case the two windows involved are on different viewports, we will draw the overlay on each of them.
    c_int overlay_draw_lists_count = 0;
    ImDrawList* overlay_draw_lists[2];
    overlay_draw_lists[overlay_draw_lists_count++] = GetForegroundDrawList(host_window.Viewport);
    if (host_window.Viewport != root_payload->Viewport && !is_transparent_payload)
        overlay_draw_lists[overlay_draw_lists_count++] = GetForegroundDrawList(root_payload->Viewport);

    // Draw main preview rectangle
    const u32 overlay_col_main = GetColorU32(ImGuiCol_DockingPreview, is_transparent_payload ? 0.60f32 : 0.400f32);
    const u32 overlay_col_drop = GetColorU32(ImGuiCol_DockingPreview, is_transparent_payload ? 0.90f32 : 0.700f32);
    const u32 overlay_col_drop_hovered = GetColorU32(ImGuiCol_DockingPreview, is_transparent_payload ? 1.20f32 : 1.000f32);
    const u32 overlay_col_lines = GetColorU32(ImGuiCol_NavWindowingHighlight, is_transparent_payload ? 0.80f32 : 0.600f32);

    // Display area preview
    let can_preview_tabs: bool = (root_payload->DockNodeAsHost == NULL || root_payload->DockNodeAsHost->Windows.Size > 0);
    if (data.IsDropAllowed)
    {
        ImRect overlay_rect = data.FutureNode.Rect();
        if (data.SplitDir == ImGuiDir_None && can_preview_tabs)
            overlay_rect.Min.y += GetFrameHeight();
        if (data.SplitDir != ImGuiDir_None || data.IsCenterAvailable)
            for (c_int overlay_n = 0; overlay_n < overlay_draw_lists_count; overlay_n++)
                overlay_draw_lists[overlay_n]->AddRectFilled(overlay_rect.Min, overlay_rect.Max, overlay_col_main, host_window.WindowRounding, CalcRoundingFlagsForRectInRect(overlay_rect, host_window.Rect(), DOCKING_SPLITTER_SIZE));
    }

    // Display tab shape/label preview unless we are splitting node (it generally makes the situation harder to read)
    if (data.IsDropAllowed && can_preview_tabs && data.SplitDir == ImGuiDir_None && data.IsCenterAvailable)
    {
        // Compute target tab bar geometry so we can locate our preview tabs
        ImRect tab_bar_rect;
        DockNodeCalcTabBarLayout(&data.FutureNode, NULL, &tab_bar_rect, NULL, NULL);
        ImVec2 tab_pos = tab_bar_rect.Min;
        if (host_node && host_node->TabBar)
        {
            if (!host_node->IsHiddenTabBar() && !host_node->IsNoTabBar())
                tab_pos.x += host_node->TabBar->WidthAllTabs + g.Style.ItemInnerSpacing.x; // We don't use OffsetNewTab because when using non-persistent-order tab bar it is incremented with each Tab submission.
            else
                tab_pos.x += g.Style.ItemInnerSpacing.x + TabItemCalcSize(host_node->Windows[0]->Name, host_node->Windows[0]->HasCloseButton).x;
        }
        else if (!(host_window.Flags & ImGuiWindowFlags_DockNodeHost))
        {
            tab_pos.x += g.Style.ItemInnerSpacing.x + TabItemCalcSize(host_window.Name, host_window.HasCloseButton).x; // Account for slight offset which will be added when changing from title bar to tab bar
        }

        // Draw tab shape/label preview (payload may be a loose window or a host window carrying multiple tabbed windows)
        if (root_payload->DockNodeAsHost)
            // IM_ASSERT(root_payload->DockNodeAsHost->Windows.Size <= root_payload->DockNodeAsHost->TabBar->Tabs.Size);
        ImGuiTabBar* tab_bar_with_payload = root_payload->DockNodeAsHost ? root_payload->DockNodeAsHost->TabBar : NULL;
        let payload_count: c_int = tab_bar_with_payload ? tab_bar_with_payload->Tabs.Size : 1;
        for (c_int payload_n = 0; payload_n < payload_count; payload_n++)
        {
            // DockNode's TabBar may have non-window Tabs manually appended by user
            ImGuiWindow* payload_window = tab_bar_with_payload ? tab_bar_with_payload->Tabs[payload_n].Window : root_payload;
            if (tab_bar_with_payload && payload_window == NULL)
                continue;
            if (!DockNodeIsDropAllowedOne(payload_window, host_window))
                continue;

            // Calculate the tab bounding box for each payload window
            ImVec2 tab_size = TabItemCalcSize(payload_window.Name, payload_window.HasCloseButton);
            ImRect tab_bb(tab_pos.x, tab_pos.y, tab_pos.x + tab_size.x, tab_pos.y + tab_size.y);
            tab_pos.x += tab_size.x + g.Style.ItemInnerSpacing.x;
            const u32 overlay_col_text = GetColorU32(payload_window.DockStyle.Colors[ImGuiWindowDockStyleCol_Text]);
            const u32 overlay_col_tabs = GetColorU32(payload_window.DockStyle.Colors[ImGuiWindowDockStyleCol_TabActive]);
            PushStyleColor(ImGuiCol_Text, overlay_col_text);
            for (c_int overlay_n = 0; overlay_n < overlay_draw_lists_count; overlay_n++)
            {
                ImGuiTabItemFlags tab_flags = ImGuiTabItemFlags_Preview | ((payload_window.Flags & ImGuiWindowFlags_UnsavedDocument) ? ImGuiTabItemFlags_UnsavedDocument : 0);
                if (!tab_bar_rect.Contains(tab_bb))
                    overlay_draw_lists[overlay_n]->PushClipRect(tab_bar_rect.Min, tab_bar_rect.Max);
                TabItemBackground(overlay_draw_lists[overlay_n], tab_bb, tab_flags, overlay_col_tabs);
                TabItemLabelAndCloseButton(overlay_draw_lists[overlay_n], tab_bb, tab_flags, g.Style.FramePadding, payload_window.Name, 0, 0, false, NULL, NULL);
                if (!tab_bar_rect.Contains(tab_bb))
                    overlay_draw_lists[overlay_n]->PopClipRect();
            }
            PopStyleColor();
        }
    }

    // Display drop boxes
    const c_float overlay_rounding = ImMax(3.0f32, g.Style.FrameRounding);
    for (c_int dir = ImGuiDir_None; dir < ImGuiDir_COUNT; dir++)
    {
        if (!data.DropRectsDraw[dir + 1].IsInverted())
        {
            ImRect draw_r = data.DropRectsDraw[dir + 1];
            ImRect draw_r_in = draw_r;
            draw_r_in.Expand(-2.00f32);
            u32 overlay_col = (data.SplitDir == (ImGuiDir)dir && data.IsSplitDirExplicit) ? overlay_col_drop_hovered : overlay_col_drop;
            for (c_int overlay_n = 0; overlay_n < overlay_draw_lists_count; overlay_n++)
            {
                ImVec2 center = ImFloor(draw_r_in.GetCenter());
                overlay_draw_lists[overlay_n]->AddRectFilled(draw_r.Min, draw_r.Max, overlay_col, overlay_rounding);
                overlay_draw_lists[overlay_n]->AddRect(draw_r_in.Min, draw_r_in.Max, overlay_col_lines, overlay_rounding);
                if (dir == ImGuiDir_Left || dir == ImGuiDir_Right)
                    overlay_draw_lists[overlay_n]->AddLine(ImVec2(center.x, draw_r_in.Min.y), ImVec2(center.x, draw_r_in.Max.y), overlay_col_lines);
                if (dir == ImGuiDir_Up || dir == ImGuiDir_Down)
                    overlay_draw_lists[overlay_n]->AddLine(ImVec2(draw_r_in.Min.x, center.y), ImVec2(draw_r_in.Max.x, center.y), overlay_col_lines);
            }
        }

        // Stop after ImGuiDir_None
        if ((host_node && (host_node->MergedFlags & ImGuiDockNodeFlags_NoSplit)) || g.IO.ConfigDockingNoSplit)
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

c_void ImGui::DockNodeTreeSplit(ImGuiContext* ctx, ImGuiDockNode* parent_node, ImGuiAxis split_axis, c_int split_inheritor_child_idx, c_float split_ratio, ImGuiDockNode* new_node)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(split_axis != ImGuiAxis_None);

    ImGuiDockNode* child_0 = (new_node && split_inheritor_child_idx != 0) ? new_node : DockContextAddNode(ctx, 0);
    child_0->ParentNode = parent_node;

    ImGuiDockNode* child_1 = (new_node && split_inheritor_child_idx != 1) ? new_node : DockContextAddNode(ctx, 0);
    child_1->ParentNode = parent_node;

    ImGuiDockNode* child_inheritor = (split_inheritor_child_idx == 0) ? child_0 : child_1;
    DockNodeMoveChildNodes(child_inheritor, parent_node);
    parent_node->ChildNodes[0] = child_0;
    parent_node->ChildNodes[1] = child_1;
    parent_node->ChildNodes[split_inheritor_child_idx]->VisibleWindow = parent_node->VisibleWindow;
    parent_node->SplitAxis = split_axis;
    parent_node->VisibleWindow = None;
    parent_node->AuthorityForPos = parent_node->AuthorityForSize = ImGuiDataAuthority_DockNode;

    c_float size_avail = (parent_node->Size[split_axis] - DOCKING_SPLITTER_SIZE);
    size_avail = ImMax(size_avail, g.Style.WindowMinSize[split_axis] * 2.00f32);
    // IM_ASSERT(size_avail > 0f32); // If you created a node manually with DockBuilderAddNode(), you need to also call DockBuilderSetNodeSize() before splitting.
    child_0->SizeRef = child_1->SizeRef = parent_node->Size;
    child_0->SizeRef[split_axis] = ImFloor(size_avail * split_ratio);
    child_1->SizeRef[split_axis] = ImFloor(size_avail - child_0->SizeRef[split_axis]);

    DockNodeMoveWindows(parent_node->ChildNodes[split_inheritor_child_idx], parent_node);
    DockSettingsRenameNodeReferences(parent_node->ID, parent_node->ChildNodes[split_inheritor_child_idx]->ID);
    DockNodeUpdateHasCentralNodeChild(DockNodeGetRootNode(parent_node));
    DockNodeTreeUpdatePosSize(parent_node, parent_node->Pos, parent_node->Size);

    // Flags transfer (e.g. this is where we transfer the ImGuiDockNodeFlags_CentralNode property)
    child_0->SharedFlags = parent_node->SharedFlags & ImGuiDockNodeFlags_SharedFlagsInheritMask_;
    child_1->SharedFlags = parent_node->SharedFlags & ImGuiDockNodeFlags_SharedFlagsInheritMask_;
    child_inheritor->LocalFlags = parent_node->LocalFlags & ImGuiDockNodeFlags_LocalFlagsTransferMask_;
    parent_node->LocalFlags &= ~ImGuiDockNodeFlags_LocalFlagsTransferMask_;
    child_0->UpdateMergedFlags();
    child_1->UpdateMergedFlags();
    parent_node->UpdateMergedFlags();
    if (child_inheritor->IsCentralNode())
        DockNodeGetRootNode(parent_node)->CentralNode = child_inheritor;
}

c_void ImGui::DockNodeTreeMerge(ImGuiContext* ctx, ImGuiDockNode* parent_node, ImGuiDockNode* merge_lead_child)
{
    // When called from DockContextProcessUndockNode() it is possible that one of the child is NULL.
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiDockNode* child_0 = parent_node->ChildNodes[0];
    ImGuiDockNode* child_1 = parent_node->ChildNodes[1];
    // IM_ASSERT(child_0 || child_1);
    // IM_ASSERT(merge_lead_child == child_0 || merge_lead_child == child_1);
    if ((child_0 && child_0->Windows.Size > 0) || (child_1 && child_1->Windows.Size > 0))
    {
        // IM_ASSERT(parent_node->TabBar == NULL);
        // IM_ASSERT(parent_node->Windows.Size == 0);
    }
    IMGUI_DEBUG_LOG_DOCKING("[docking] DockNodeTreeMerge: 0x%08X + 0x%08X back into parent 0x%08X\n", child_0 ? child_0->ID : 0, child_1 ? child_1->ID : 0, parent_node->ID);

    ImVec2 backup_last_explicit_size = parent_node->SizeRef;
    DockNodeMoveChildNodes(parent_node, merge_lead_child);
    if (child_0)
    {
        DockNodeMoveWindows(parent_node, child_0); // Generally only 1 of the 2 child node will have windows
        DockSettingsRenameNodeReferences(child_0->ID, parent_node->ID);
    }
    if (child_1)
    {
        DockNodeMoveWindows(parent_node, child_1);
        DockSettingsRenameNodeReferences(child_1->ID, parent_node->ID);
    }
    DockNodeApplyPosSizeToWindows(parent_node);
    parent_node->AuthorityForPos = parent_node->AuthorityForSize = parent_node->AuthorityForViewport = ImGuiDataAuthority_Auto;
    parent_node->VisibleWindow = merge_lead_child->VisibleWindow;
    parent_node->SizeRef = backup_last_explicit_size;

    // Flags transfer
    parent_node->LocalFlags &= ~ImGuiDockNodeFlags_LocalFlagsTransferMask_; // Preserve Dockspace flag
    parent_node->LocalFlags |= (child_0 ? child_0->LocalFlags : 0) & ImGuiDockNodeFlags_LocalFlagsTransferMask_;
    parent_node->LocalFlags |= (child_1 ? child_1->LocalFlags : 0) & ImGuiDockNodeFlags_LocalFlagsTransferMask_;
    parent_node->LocalFlagsInWindows = (child_0 ? child_0->LocalFlagsInWindows : 0) | (child_1 ? child_1->LocalFlagsInWindows : 0); // FIXME: Would be more consistent to update from actual windows
    parent_node->UpdateMergedFlags();

    if (child_0)
    {
        ctx->DockContext.Nodes.SetVoidPtr(child_0->ID, NULL);
        IM_DELETE(child_0);
    }
    if (child_1)
    {
        ctx->DockContext.Nodes.SetVoidPtr(child_1->ID, NULL);
        IM_DELETE(child_1);
    }
}

// Update Pos/Size for a node hierarchy (don't affect child Windows yet)
// (Depth-first, Pre-Order)
c_void ImGui::DockNodeTreeUpdatePosSize(ImGuiDockNode* node, ImVec2 pos, ImVec2 size, ImGuiDockNode* only_write_to_single_node)
{
    // During the regular dock node update we write to all nodes.
    // 'only_write_to_single_node' is only set when turning a node visible mid-frame and we need its size right-away.
    let write_to_node: bool = only_write_to_single_node == NULL || only_write_to_single_node == node;
    if (write_to_node)
    {
        node->Pos = pos;
        node->Size = size;
    }

    if (node->IsLeafNode())
        return;

    ImGuiDockNode* child_0 = node->ChildNodes[0];
    ImGuiDockNode* child_1 = node->ChildNodes[1];
    ImVec2 child_0_pos = pos, child_1_pos = pos;
    ImVec2 child_0_size = size, child_1_size = size;

    let child_0_is_toward_single_node: bool = (only_write_to_single_node != NULL && DockNodeIsInHierarchyOf(only_write_to_single_node, child_0));
    let child_1_is_toward_single_node: bool = (only_write_to_single_node != NULL && DockNodeIsInHierarchyOf(only_write_to_single_node, child_1));
    let child_0_is_or_will_be_visible: bool = child_0->IsVisible || child_0_is_toward_single_node;
    let child_1_is_or_will_be_visible: bool = child_1->IsVisible || child_1_is_toward_single_node;

    if (child_0_is_or_will_be_visible && child_1_is_or_will_be_visible)
    {
        let g = GImGui; // ImGuiContext& g = *GImGui;
        const c_float spacing = DOCKING_SPLITTER_SIZE;
        const ImGuiAxis axis = (ImGuiAxis)node->SplitAxis;
        const c_float size_avail = ImMax(size[axis] - spacing, 0f32);

        // Size allocation policy
        // 1) The first 0..WindowMinSize[axis]*2 are allocated evenly to both windows.
        const c_float size_min_each = ImFloor(ImMin(size_avail, g.Style.WindowMinSize[axis] * 2.00f32) * 0.5f32);

        // FIXME: Blocks 2) and 3) are essentially doing nearly the same thing.
        // Difference are: write-back to SizeRef; application of a minimum size; rounding before ImFloor()
        // Clarify and rework differences between Size & SizeRef and purpose of WantLockSizeOnce

        // 2) Process locked absolute size (during a splitter resize we preserve the child of nodes not touching the splitter edge)
        if (child_0->WantLockSizeOnce && !child_1->WantLockSizeOnce)
        {
            child_0_size[axis] = child_0->SizeRef[axis] = ImMin(size_avail - 1f32, child_0->Size[axis]);
            child_1_size[axis] = child_1->SizeRef[axis] = (size_avail - child_0_size[axis]);
            // IM_ASSERT(child_0->SizeRef[axis] > 0f32 && child_1->SizeRef[axis] > 0f32);
        }
        else if (child_1->WantLockSizeOnce && !child_0->WantLockSizeOnce)
        {
            child_1_size[axis] = child_1->SizeRef[axis] = ImMin(size_avail - 1f32, child_1->Size[axis]);
            child_0_size[axis] = child_0->SizeRef[axis] = (size_avail - child_1_size[axis]);
            // IM_ASSERT(child_0->SizeRef[axis] > 0f32 && child_1->SizeRef[axis] > 0f32);
        }
        else if (child_0->WantLockSizeOnce && child_1->WantLockSizeOnce)
        {
            // FIXME-DOCK: We cannot honor the requested size, so apply ratio.
            // Currently this path will only be taken if code programmatically sets WantLockSizeOnce
            c_float split_ratio = child_0_size[axis] / (child_0_size[axis] + child_1_size[axis]);
            child_0_size[axis] = child_0->SizeRef[axis] = ImFloor(size_avail * split_ratio);
            child_1_size[axis] = child_1->SizeRef[axis] = (size_avail - child_0_size[axis]);
            // IM_ASSERT(child_0->SizeRef[axis] > 0f32 && child_1->SizeRef[axis] > 0f32);
        }

        // 3) If one window is the central node (~ use remaining space, should be made explicit!), use explicit size from the other, and remainder for the central node
        else if (child_0->SizeRef[axis] != 0f32 && child_1->HasCentralNodeChild)
        {
            child_0_size[axis] = ImMin(size_avail - size_min_each, child_0->SizeRef[axis]);
            child_1_size[axis] = (size_avail - child_0_size[axis]);
        }
        else if (child_1->SizeRef[axis] != 0f32 && child_0->HasCentralNodeChild)
        {
            child_1_size[axis] = ImMin(size_avail - size_min_each, child_1->SizeRef[axis]);
            child_0_size[axis] = (size_avail - child_1_size[axis]);
        }
        else
        {
            // 4) Otherwise distribute according to the relative ratio of each SizeRef value
            c_float split_ratio = child_0->SizeRef[axis] / (child_0->SizeRef[axis] + child_1->SizeRef[axis]);
            child_0_size[axis] = ImMax(size_min_each, ImFloor(size_avail * split_ratio + 0.5f32));
            child_1_size[axis] = (size_avail - child_0_size[axis]);
        }

        child_1_pos[axis] += spacing + child_0_size[axis];
    }

    if (only_write_to_single_node == NULL)
        child_0->WantLockSizeOnce = child_1->WantLockSizeOnce = false;

    let child_0_recurse: bool = only_write_to_single_node ? child_0_is_toward_single_node : child_0->IsVisible;
    let child_1_recurse: bool = only_write_to_single_node ? child_1_is_toward_single_node : child_1->IsVisible;
    if (child_0_recurse)
        DockNodeTreeUpdatePosSize(child_0, child_0_pos, child_0_size);
    if (child_1_recurse)
        DockNodeTreeUpdatePosSize(child_1, child_1_pos, child_1_size);
}

static c_void DockNodeTreeUpdateSplitterFindTouchingNode(ImGuiDockNode* node, ImGuiAxis axis, c_int side, Vec<ImGuiDockNode*>* touching_nodes)
{
    if (node->IsLeafNode())
    {
        touching_nodes.push(node);
        return;
    }
    if (node->ChildNodes[0]->IsVisible)
        if (node->SplitAxis != axis || side == 0 || !node->ChildNodes[1]->IsVisible)
            DockNodeTreeUpdateSplitterFindTouchingNode(node->ChildNodes[0], axis, side, touching_nodes);
    if (node->ChildNodes[1]->IsVisible)
        if (node->SplitAxis != axis || side == 1 || !node->ChildNodes[0]->IsVisible)
            DockNodeTreeUpdateSplitterFindTouchingNode(node->ChildNodes[1], axis, side, touching_nodes);
}

// (Depth-First, Pre-Order)
c_void ImGui::DockNodeTreeUpdateSplitter(ImGuiDockNode* node)
{
    if (node->IsLeafNode())
        return;

    let g = GImGui; // ImGuiContext& g = *GImGui;

    ImGuiDockNode* child_0 = node->ChildNodes[0];
    ImGuiDockNode* child_1 = node->ChildNodes[1];
    if (child_0->IsVisible && child_1->IsVisible)
    {
        // Bounding box of the splitter cover the space between both nodes (w = Spacing, h = Size[xy^1] for when splitting horizontally)
        const ImGuiAxis axis = (ImGuiAxis)node->SplitAxis;
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
            PushID(node->ID);

            // Find resizing limits by gathering list of nodes that are touching the splitter line.
            Vec<ImGuiDockNode*> touching_nodes[2];
            c_float min_size = g.Style.WindowMinSize[axis];
            c_float resize_limits[2];
            resize_limits[0] = node->ChildNodes[0]->Pos[axis] + min_size;
            resize_limits[1] = node->ChildNodes[1]->Pos[axis] + node->ChildNodes[1]->Size[axis] - min_size;

            ImGuiID splitter_id = GetID("##Splitter");
            if (g.ActiveId == splitter_id) // Only process when splitter is active
            {
                DockNodeTreeUpdateSplitterFindTouchingNode(child_0, axis, 1, &touching_nodes[0]);
                DockNodeTreeUpdateSplitterFindTouchingNode(child_1, axis, 0, &touching_nodes[1]);
                for (c_int touching_node_n = 0; touching_node_n < touching_nodes[0].Size; touching_node_n++)
                    resize_limits[0] = ImMax(resize_limits[0], touching_nodes[0][touching_node_n]->Rect().Min[axis] + min_size);
                for (c_int touching_node_n = 0; touching_node_n < touching_nodes[1].Size; touching_node_n++)
                    resize_limits[1] = ImMin(resize_limits[1], touching_nodes[1][touching_node_n]->Rect().Max[axis] - min_size);

                // [DEBUG] Render touching nodes & limits
                /*
                ImDrawList* draw_list = node->HostWindow ? GetForegroundDrawList(node->HostWindow) : GetForegroundDrawList(GetMainViewport());
                for (int n = 0; n < 2; n++)
                {
                    for (int touching_node_n = 0; touching_node_n < touching_nodes[n].Size; touching_node_n++)
                        draw_list->AddRect(touching_nodes[n][touching_node_n]->Pos, touching_nodes[n][touching_node_n]->Pos + touching_nodes[n][touching_node_n]->Size, IM_COL32(0, 255, 0, 255));
                    if (axis == ImGuiAxis_X)
                        draw_list->AddLine(ImVec2(resize_limits[n], node->ChildNodes[n]->Pos.y), ImVec2(resize_limits[n], node->ChildNodes[n]->Pos.y + node->ChildNodes[n]->Size.y), IM_COL32(255, 0, 255, 255), 3.00f32);
                    else
                        draw_list->AddLine(ImVec2(node->ChildNodes[n]->Pos.x, resize_limits[n]), ImVec2(node->ChildNodes[n]->Pos.x + node->ChildNodes[n]->Size.x, resize_limits[n]), IM_COL32(255, 0, 255, 255), 3.00f32);
                }
                */
            }

            // Use a short delay before highlighting the splitter (and changing the mouse cursor) in order for regular mouse movement to not highlight many splitters
            c_float cur_size_0 = child_0->Size[axis];
            c_float cur_size_1 = child_1->Size[axis];
            c_float min_size_0 = resize_limits[0] - child_0->Pos[axis];
            c_float min_size_1 = child_1->Pos[axis] + child_1->Size[axis] - resize_limits[1];
            u32 bg_col = GetColorU32(ImGuiCol_WindowBg);
            if (SplitterBehavior(bb, GetID("##Splitter"), axis, &cur_size_0, &cur_size_1, min_size_0, min_size_1, WINDOWS_HOVER_PADDING, WINDOWS_RESIZE_FROM_EDGES_FEEDBACK_TIMER, bg_col))
            {
                if (touching_nodes[0].Size > 0 && touching_nodes[1].Size > 0)
                {
                    child_0->Size[axis] = child_0->SizeRef[axis] = cur_size_0;
                    child_1->Pos[axis] -= cur_size_1 - child_1->Size[axis];
                    child_1->Size[axis] = child_1->SizeRef[axis] = cur_size_1;

                    // Lock the size of every node that is a sibling of the node we are touching
                    // This might be less desirable if we can merge sibling of a same axis into the same parental level.
                    for (c_int side_n = 0; side_n < 2; side_n++)
                        for (c_int touching_node_n = 0; touching_node_n < touching_nodes[side_n].Size; touching_node_n++)
                        {
                            ImGuiDockNode* touching_node = touching_nodes[side_n][touching_node_n];
                            //ImDrawList* draw_list = node->HostWindow ? GetForegroundDrawList(node->HostWindow) : GetForegroundDrawList(GetMainViewport());
                            //draw_list->AddRect(touching_node->Pos, touching_node->Pos + touching_node->Size, IM_COL32(255, 128, 0, 255));
                            while (touching_node->ParentNode != node)
                            {
                                if (touching_node->ParentNode->SplitAxis == axis)
                                {
                                    // Mark other node so its size will be preserved during the upcoming call to DockNodeTreeUpdatePosSize().
                                    ImGuiDockNode* node_to_preserve = touching_node->ParentNode->ChildNodes[side_n];
                                    node_to_preserve->WantLockSizeOnce = true;
                                    //draw_list->AddRect(touching_node->Pos, touching_node->Rect().Max, IM_COL32(255, 0, 0, 255));
                                    //draw_list->AddRectFilled(node_to_preserve->Pos, node_to_preserve->Rect().Max, IM_COL32(0, 255, 0, 100));
                                }
                                touching_node = touching_node->ParentNode;
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

ImGuiDockNode* ImGui::DockNodeTreeFindFallbackLeafNode(ImGuiDockNode* node)
{
    if (node->IsLeafNode())
        return node;
    if (ImGuiDockNode* leaf_node = DockNodeTreeFindFallbackLeafNode(node->ChildNodes[0]))
        return leaf_node;
    if (ImGuiDockNode* leaf_node = DockNodeTreeFindFallbackLeafNode(node->ChildNodes[1]))
        return leaf_node;
    return NULL;
}

ImGuiDockNode* ImGui::DockNodeTreeFindVisibleNodeByPos(ImGuiDockNode* node, ImVec2 pos)
{
    if (!node->IsVisible)
        return NULL;

    const c_float dock_spacing = 0f32;// g.Style.ItemInnerSpacing.x; // FIXME: Relation to DOCKING_SPLITTER_SIZE?
    ImRect r(node->Pos, node->Pos + node->Size);
    r.Expand(dock_spacing * 0.5f32);
    let mut inside: bool =  r.Contains(pos);
    if (!inside)
        return NULL;

    if (node->IsLeafNode())
        return node;
    if (ImGuiDockNode* hovered_node = DockNodeTreeFindVisibleNodeByPos(node->ChildNodes[0], pos))
        return hovered_node;
    if (ImGuiDockNode* hovered_node = DockNodeTreeFindVisibleNodeByPos(node->ChildNodes[1], pos))
        return hovered_node;

    return NULL;
}

//-----------------------------------------------------------------------------
// Docking: Public Functions (SetWindowDock, DockSpace, DockSpaceOverViewport)
//-----------------------------------------------------------------------------
// - SetWindowDock() [Internal]
// - DockSpace()
// - DockSpaceOverViewport()
//-----------------------------------------------------------------------------

// [Internal] Called via SetNextWindowDockID()
c_void ImGui::SetWindowDock(ImGuiWindow* window, ImGuiID dock_id, ImGuiCond cond)
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
        if (new_node->IsSplitNode())
        {
            // Policy: Find central node or latest focused node. We first move back to our root node.
            new_node = DockNodeGetRootNode(new_node);
            if (new_node->CentralNode)
            {
                // IM_ASSERT(new_node->CentralNode->IsCentralNode());
                dock_id = new_node->CentralNode->ID;
            }
            else
            {
                dock_id = new_node->LastFocusedNodeId;
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
ImGuiID ImGui::DockSpace(ImGuiID id, const ImVec2& size_arg, ImGuiDockNodeFlags flags, *const ImGuiWindowClass window_class)
{
    ImGuiContext* ctx = GImGui;
    ImGuiContext& g = *ctx;
    ImGuiWindow* window = GetCurrentWindow();
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
        node->SetLocalFlags(ImGuiDockNodeFlags_CentralNode);
    }
    if (window_class && window_class->ClassId != node->WindowClass.ClassId)
        IMGUI_DEBUG_LOG_DOCKING("[docking] DockSpace: dockspace node 0x%08X: setup WindowClass 0x%08X -> 0x%08X\n", id, node->WindowClass.ClassId, window_class->ClassId);
    node->SharedFlags = flags;
    node->WindowClass = window_class ? *window_class : ImGuiWindowClass();

    // When a DockSpace transitioned form implicit to explicit this may be called a second time
    // It is possible that the node has already been claimed by a docked window which appeared before the DockSpace() node, so we overwrite IsDockSpace again.
    if (node->LastFrameActive == g.FrameCount && !(flags & ImGuiDockNodeFlags_KeepAliveOnly))
    {
        // IM_ASSERT(node->IsDockSpace() == false && "Cannot call DockSpace() twice a frame with the same ID");
        node->SetLocalFlags(node->LocalFlags | ImGuiDockNodeFlags_DockSpace);
        return id;
    }
    node->SetLocalFlags(node->LocalFlags | ImGuiDockNodeFlags_DockSpace);

    // Keep alive mode, this is allow windows docked into this node so stay docked even if they are not visible
    if (flags & ImGuiDockNodeFlags_KeepAliveOnly)
    {
        node->LastFrameAlive = g.FrameCount;
        return id;
    }

    const ImVec2 content_avail = GetContentRegionAvail();
    ImVec2 size = ImFloor(size_arg);
    if (size.x <= 0f32)
        size.x = ImMax(content_avail.x + size.x, 4.00f32); // Arbitrary minimum child size (0f32 causing too much issues)
    if (size.y <= 0f32)
        size.y = ImMax(content_avail.y + size.y, 4.00f32);
    // IM_ASSERT(size.x > 0f32 && size.y > 0f32);

    node->Pos = window.DC.CursorPos;
    node->Size = node->SizeRef = size;
    SetNextWindowPos(node->Pos);
    SetNextWindowSize(node->Size);
    g.NextWindowData.PosUndock = false;

    // FIXME-DOCK: Why do we need a child window to host a dockspace, could we host it in the existing window?
    // FIXME-DOCK: What is the reason for not simply calling BeginChild()? (OK to have a reason but should be commented)
    ImGuiWindowFlags window_flags = ImGuiWindowFlags_ChildWindow | ImGuiWindowFlags_DockNodeHost;
    window_flags |= ImGuiWindowFlags_NoSavedSettings | ImGuiWindowFlags_NoResize | ImGuiWindowFlags_NoCollapse | ImGuiWindowFlags_NoTitleBar;
    window_flags |= ImGuiWindowFlags_NoScrollbar | ImGuiWindowFlags_NoScrollWithMouse;
    window_flags |= ImGuiWindowFlags_NoBackground;

    title: [c_char;256];
    ImFormatString(title, IM_ARRAYSIZE(title), "%s/DockSpace_%08X", window.Name, id);

    PushStyleVar(ImGuiStyleVar_ChildBorderSize, 0f32);
    Begin(title, NULL, window_flags);
    PopStyleVar();

    ImGuiWindow* host_window = g.CurrentWindow;
    DockNodeSetupHostWindow(node, host_window);
    host_window.ChildId = window.GetID(title);
    node->OnlyNodeWithWindows = None;

    // IM_ASSERT(node->IsRootNode());

    // We need to handle the rare case were a central node is missing.
    // This can happen if the node was first created manually with DockBuilderAddNode() but _without_ the ImGuiDockNodeFlags_Dockspace.
    // Doing it correctly would set the _CentralNode flags, which would then propagate according to subsequent split.
    // It would also be ambiguous to attempt to assign a central node while there are split nodes, so we wait until there's a single node remaining.
    // The specific sub-property of _CentralNode we are interested in recovering here is the "Don't delete when empty" property,
    // as it doesn't make sense for an empty dockspace to not have this property.
    if (node->IsLeafNode() && !node->IsCentralNode())
        node->SetLocalFlags(node->LocalFlags | ImGuiDockNodeFlags_CentralNode);

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
ImGuiID ImGui::DockSpaceOverViewport(*const ImGuiViewport viewport, ImGuiDockNodeFlags dockspace_flags, *const ImGuiWindowClass window_class)
{
    if (viewport == NULL)
        viewport = GetMainViewport();

    SetNextWindowPos(viewport->WorkPos);
    SetNextWindowSize(viewport->WorkSize);
    SetNextWindowViewport(viewport->ID);

    ImGuiWindowFlags host_window_flags = 0;
    host_window_flags |= ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoCollapse | ImGuiWindowFlags_NoResize | ImGuiWindowFlags_NoMove | ImGuiWindowFlags_NoDocking;
    host_window_flags |= ImGuiWindowFlags_NoBringToFrontOnFocus | ImGuiWindowFlags_NoNavFocus;
    if (dockspace_flags & ImGuiDockNodeFlags_PassthruCentralNode)
        host_window_flags |= ImGuiWindowFlags_NoBackground;

    label: [c_char;32];
    ImFormatString(label, IM_ARRAYSIZE(label), "DockSpaceViewport_%08X", viewport->ID);

    PushStyleVar(ImGuiStyleVar_WindowRounding, 0f32);
    PushStyleVar(ImGuiStyleVar_WindowBorderSize, 0f32);
    PushStyleVar(ImGuiStyleVar_WindowPadding, ImVec2(0f32, 0f32));
    Begin(label, NULL, host_window_flags);
    PopStyleVar(3);

    ImGuiID dockspace_id = GetID("DockSpace");
    DockSpace(dockspace_id, ImVec2(0f32, 0f32), dockspace_flags, window_class);
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

c_void ImGui::DockBuilderDockWindow(*const char window_name, ImGuiID node_id)
{
    // We don't preserve relative order of multiple docked windows (by clearing DockOrder back to -1)
    ImGuiID window_id = ImHashStr(window_name);
    if (ImGuiWindow* window = FindWindowByID(window_id))
    {
        // Apply to created window
        SetWindowDock(window, node_id, ImGuiCond_Always);
        window.DockOrder = -1;
    }
    else
    {
        // Apply to settings
        ImGuiWindowSettings* settings = FindWindowSettings(window_id);
        if (settings == NULL)
            settings = CreateNewWindowSettings(window_name);
        settings->DockId = node_id;
        settings->DockOrder = -1;
    }
}

ImGuiDockNode* ImGui::DockBuilderGetNode(ImGuiID node_id)
{
    ImGuiContext* ctx = GImGui;
    return DockContextFindNodeByID(ctx, node_id);
}

c_void ImGui::DockBuilderSetNodePos(ImGuiID node_id, ImVec2 pos)
{
    ImGuiContext* ctx = GImGui;
    ImGuiDockNode* node = DockContextFindNodeByID(ctx, node_id);
    if (node == NULL)
        return;
    node->Pos = pos;
    node->AuthorityForPos = ImGuiDataAuthority_DockNode;
}

c_void ImGui::DockBuilderSetNodeSize(ImGuiID node_id, ImVec2 size)
{
    ImGuiContext* ctx = GImGui;
    ImGuiDockNode* node = DockContextFindNodeByID(ctx, node_id);
    if (node == NULL)
        return;
    // IM_ASSERT(size.x > 0f32 && size.y > 0f32);
    node->Size = node->SizeRef = size;
    node->AuthorityForSize = ImGuiDataAuthority_DockNode;
}

// Make sure to use the ImGuiDockNodeFlags_DockSpace flag to create a dockspace node! Otherwise this will create a floating node!
// - Floating node: you can then call DockBuilderSetNodePos()/DockBuilderSetNodeSize() to position and size the floating node.
// - Dockspace node: calling DockBuilderSetNodePos() is unnecessary.
// - If you intend to split a node immediately after creation using DockBuilderSplitNode(), make sure to call DockBuilderSetNodeSize() beforehand!
//   For various reason, the splitting code currently needs a base size otherwise space may not be allocated as precisely as you would expect.
// - Use (id == 0) to let the system allocate a node identifier.
// - Existing node with a same id will be removed.
ImGuiID ImGui::DockBuilderAddNode(ImGuiID id, ImGuiDockNodeFlags flags)
{
    ImGuiContext* ctx = GImGui;

    if (id != 0)
        DockBuilderRemoveNode(id);

    ImGuiDockNode* node = None;
    if (flags & ImGuiDockNodeFlags_DockSpace)
    {
        DockSpace(id, ImVec2(0, 0), (flags & ~ImGuiDockNodeFlags_DockSpace) | ImGuiDockNodeFlags_KeepAliveOnly);
        node = DockContextFindNodeByID(ctx, id);
    }
    else
    {
        node = DockContextAddNode(ctx, id);
        node->SetLocalFlags(flags);
    }
    node->LastFrameAlive = ctx->FrameCount;   // Set this otherwise BeginDocked will undock during the same frame.
    return node->ID;
}

c_void ImGui::DockBuilderRemoveNode(ImGuiID node_id)
{
    ImGuiContext* ctx = GImGui;
    ImGuiDockNode* node = DockContextFindNodeByID(ctx, node_id);
    if (node == NULL)
        return;
    DockBuilderRemoveNodeDockedWindows(node_id, true);
    DockBuilderRemoveNodeChildNodes(node_id);
    // Node may have moved or deleted if e.g. any merge happened
    node = DockContextFindNodeByID(ctx, node_id);
    if (node == NULL)
        return;
    if (node->IsCentralNode() && node->ParentNode)
        node->ParentNode->SetLocalFlags(node->ParentNode->LocalFlags | ImGuiDockNodeFlags_CentralNode);
    DockContextRemoveNode(ctx, node, true);
}

// root_id = 0 to remove all, root_id != 0 to remove child of given node.
c_void ImGui::DockBuilderRemoveNodeChildNodes(ImGuiID root_id)
{
    ImGuiContext* ctx = GImGui;
    ImGuiDockContext* dc  = &ctx->DockContext;

    ImGuiDockNode* root_node = root_id ? DockContextFindNodeByID(ctx, root_id) : NULL;
    if (root_id && root_node == NULL)
        return;
    let mut has_central_node: bool =  false;

    ImGuiDataAuthority backup_root_node_authority_for_pos = root_node ? root_node->AuthorityForPos : ImGuiDataAuthority_Auto;
    ImGuiDataAuthority backup_root_node_authority_for_size = root_node ? root_node->AuthorityForSize : ImGuiDataAuthority_Auto;

    // Process active windows
    Vec<ImGuiDockNode*> nodes_to_remove;
    for (c_int n = 0; n < dc->Nodes.Data.Size; n++)
        if (ImGuiDockNode* node = (ImGuiDockNode*)dc->Nodes.Data[n].val_p)
        {
            let mut want_removal: bool =  (root_id == 0) || (node->ID != root_id && DockNodeGetRootNode(node)->ID == root_id);
            if (want_removal)
            {
                if (node->IsCentralNode())
                    has_central_node = true;
                if (root_id != 0)
                    DockContextQueueNotifyRemovedNode(ctx, node);
                if (root_node)
                {
                    DockNodeMoveWindows(root_node, node);
                    DockSettingsRenameNodeReferences(node->ID, root_node->ID);
                }
                nodes_to_remove.push(node);
            }
        }

    // DockNodeMoveWindows->DockNodeAddWindow will normally set those when reaching two windows (which is only adequate during interactive merge)
    // Make sure we don't lose our current pos/size. (FIXME-DOCK: Consider tidying up that code in DockNodeAddWindow instead)
    if (root_node)
    {
        root_node->AuthorityForPos = backup_root_node_authority_for_pos;
        root_node->AuthorityForSize = backup_root_node_authority_for_size;
    }

    // Apply to settings
    for (ImGuiWindowSettings* settings = ctx->SettingsWindows.begin(); settings != None; settings = ctx->SettingsWindows.next_chunk(settings))
        if (ImGuiID window_settings_dock_id = settings->DockId)
            for (c_int n = 0; n < nodes_to_remove.Size; n++)
                if (nodes_to_remove[n]->ID == window_settings_dock_id)
                {
                    settings->DockId = root_id;
                    break;
                }

    // Not really efficient, but easier to destroy a whole hierarchy considering DockContextRemoveNode is attempting to merge nodes
    if (nodes_to_remove.Size > 1)
        ImQsort(nodes_to_remove.Data, nodes_to_remove.Size, sizeof(ImGuiDockNode*), DockNodeComparerDepthMostFirst);
    for (c_int n = 0; n < nodes_to_remove.Size; n++)
        DockContextRemoveNode(ctx, nodes_to_remove[n], false);

    if (root_id == 0)
    {
        dc->Nodes.Clear();
        dc->Requests.clear();
    }
    else if (has_central_node)
    {
        root_node->CentralNode = root_node;
        root_node->SetLocalFlags(root_node->LocalFlags | ImGuiDockNodeFlags_CentralNode);
    }
}

c_void ImGui::DockBuilderRemoveNodeDockedWindows(ImGuiID root_id, bool clear_settings_refs)
{
    // Clear references in settings
    ImGuiContext* ctx = GImGui;
    ImGuiContext& g = *ctx;
    if (clear_settings_refs)
    {
        for (ImGuiWindowSettings* settings = g.SettingsWindows.begin(); settings != None; settings = g.SettingsWindows.next_chunk(settings))
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
    for (c_int n = 0; n < g.Windows.Size; n++)
    {
        ImGuiWindow* window = g.Windows[n];
        let mut want_removal: bool =  (root_id == 0) || (window.DockNode && DockNodeGetRootNode(window.DockNode)->ID == root_id) || (window.DockNodeAsHost && window.DockNodeAsHost->ID == root_id);
        if (want_removal)
        {
            const ImGuiID backup_dock_id = window.DockId;
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
ImGuiID ImGui::DockBuilderSplitNode(ImGuiID id, ImGuiDir split_dir, c_float size_ratio_for_node_at_dir, ImGuiID* out_id_at_dir, ImGuiID* out_id_at_opposite_dir)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(split_dir != ImGuiDir_None);
    IMGUI_DEBUG_LOG_DOCKING("[docking] DockBuilderSplitNode: node 0x%08X, split_dir %d\n", id, split_dir);

    ImGuiDockNode* node = DockContextFindNodeByID(&g, id);
    if (node == NULL)
    {
        // IM_ASSERT(node != NULL);
        return 0;
    }

    // IM_ASSERT(!node->IsSplitNode()); // Assert if already Split

    ImGuiDockRequest req;
    req.Type = ImGuiDockRequestType_Split;
    req.DockTargetWindow = None;
    req.DockTargetNode = node;
    req.DockPayload = None;
    req.DockSplitDir = split_dir;
    req.DockSplitRatio = ImSaturate((split_dir == ImGuiDir_Left || split_dir == ImGuiDir_Up) ? size_ratio_for_node_at_dir : 1f32 - size_ratio_for_node_at_dir);
    req.DockSplitOuter = false;
    DockContextProcessDock(&g, &req);

    ImGuiID id_at_dir = node->ChildNodes[(split_dir == ImGuiDir_Left || split_dir == ImGuiDir_Up) ? 0 : 1]->ID;
    ImGuiID id_at_opposite_dir = node->ChildNodes[(split_dir == ImGuiDir_Left || split_dir == ImGuiDir_Up) ? 1 : 0]->ID;
    if (out_id_at_dir)
        *out_id_at_dir = id_at_dir;
    if (out_id_at_opposite_dir)
        *out_id_at_opposite_dir = id_at_opposite_dir;
    return id_at_dir;
}

static ImGuiDockNode* DockBuilderCopyNodeRec(ImGuiDockNode* src_node, ImGuiID dst_node_id_if_known, Vec<ImGuiID>* out_node_remap_pairs)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiDockNode* dst_node = ImGui::DockContextAddNode(&g, dst_node_id_if_known);
    dst_node->SharedFlags = src_node->SharedFlags;
    dst_node->LocalFlags = src_node->LocalFlags;
    dst_node->LocalFlagsInWindows = ImGuiDockNodeFlags_None;
    dst_node->Pos = src_node->Pos;
    dst_node->Size = src_node->Size;
    dst_node->SizeRef = src_node->SizeRef;
    dst_node->SplitAxis = src_node->SplitAxis;
    dst_node->UpdateMergedFlags();

    out_node_remap_pairs.push(src_node->ID);
    out_node_remap_pairs.push(dst_node->ID);

    for (c_int child_n = 0; child_n < IM_ARRAYSIZE(src_node->ChildNodes); child_n++)
        if (src_node->ChildNodes[child_n])
        {
            dst_node->ChildNodes[child_n] = DockBuilderCopyNodeRec(src_node->ChildNodes[child_n], 0, out_node_remap_pairs);
            dst_node->ChildNodes[child_n]->ParentNode = dst_node;
        }

    IMGUI_DEBUG_LOG_DOCKING("[docking] Fork node %08X -> %08X (%d childs)\n", src_node->ID, dst_node->ID, dst_node->IsSplitNode() ? 2 : 0);
    return dst_node;
}

c_void ImGui::DockBuilderCopyNode(ImGuiID src_node_id, ImGuiID dst_node_id, Vec<ImGuiID>* out_node_remap_pairs)
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

c_void ImGui::DockBuilderCopyWindowSettings(*const char src_name, *const char dst_name)
{
    ImGuiWindow* src_window = FindWindowByName(src_name);
    if (src_window == NULL)
        return;
    if (ImGuiWindow* dst_window = FindWindowByName(dst_name))
    {
        dst_window.Pos = src_window.Pos;
        dst_window.Size = src_window.Size;
        dst_window.SizeFull = src_window.SizeFull;
        dst_window.Collapsed = src_window.Collapsed;
    }
    else if (ImGuiWindowSettings* dst_settings = FindOrCreateWindowSettings(dst_name))
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
c_void ImGui::DockBuilderCopyDockSpace(ImGuiID src_dockspace_id, ImGuiID dst_dockspace_id, Vec<*const char>* in_window_remap_pairs)
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
    for (c_int remap_window_n = 0; remap_window_n < in_window_remap_pairs->Size; remap_window_n += 2)
    {
        let mut  src_window_name: *const c_char = (*in_window_remap_pairs)[remap_window_n];
        let mut  dst_window_name: *const c_char = (*in_window_remap_pairs)[remap_window_n + 1];
        ImGuiID src_window_id = ImHashStr(src_window_name);
        src_windows.push(src_window_id);

        // Search in the remapping tables
        ImGuiID src_dock_id = 0;
        if (ImGuiWindow* src_window = FindWindowByID(src_window_id))
            src_dock_id = src_window.DockId;
        else if (ImGuiWindowSettings* src_window_settings = FindWindowSettings(src_window_id))
            src_dock_id = src_window_settings->DockId;
        ImGuiID dst_dock_id = 0;
        for (c_int dock_remap_n = 0; dock_remap_n < node_remap_pairs.Size; dock_remap_n += 2)
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
    for (c_int dock_remap_n = 0; dock_remap_n < node_remap_pairs.Size; dock_remap_n += 2)
        if (ImGuiID src_dock_id = node_remap_pairs[dock_remap_n])
        {
            ImGuiID dst_dock_id = node_remap_pairs[dock_remap_n + 1];
            ImGuiDockNode* node = DockBuilderGetNode(src_dock_id);
            for (c_int window_n = 0; window_n < node->Windows.Size; window_n++)
            {
                ImGuiWindow* window = node->Windows[window_n];
                if (src_windows.contains(window.ID))
                    continue;

                // Docked windows gets redocked into the new node hierarchy.
                IMGUI_DEBUG_LOG_DOCKING("[docking] Remap window '%s' %08X -> %08X\n", window.Name, src_dock_id, dst_dock_id);
                DockBuilderDockWindow(window.Name, dst_dock_id);
            }
        }
}

// FIXME-DOCK: This is awkward because in series of split user is likely to loose access to its root node.
c_void ImGui::DockBuilderFinish(ImGuiID root_id)
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

bool ImGui::GetWindowAlwaysWantOwnTabBar(ImGuiWindow* window)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.IO.ConfigDockingAlwaysTabBar || window.WindowClass.DockingAlwaysTabBar)
        if ((window.Flags & (ImGuiWindowFlags_ChildWindow | ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoDocking)) == 0)
            if (!window.IsFallbackWindow)    // We don't support AlwaysTabBar on the fallback/implicit window to avoid unused dock-node overhead/noise
                return true;
    return false;
}

static ImGuiDockNode* ImGui::DockContextBindNodeToWindow(ImGuiContext* ctx, ImGuiWindow* window)
{
    ImGuiContext& g = *ctx;
    ImGuiDockNode* node = DockContextFindNodeByID(ctx, window.DockId);
    // IM_ASSERT(window.DockNode == NULL);

    // We should not be docking into a split node (SetWindowDock should avoid this)
    if (node && node->IsSplitNode())
    {
        DockContextProcessUndockWindow(ctx, window);
        return NULL;
    }

    // Create node
    if (node == NULL)
    {
        node = DockContextAddNode(ctx, window.DockId);
        node->AuthorityForPos = node->AuthorityForSize = node->AuthorityForViewport = ImGuiDataAuthority_Window;
        node->LastFrameAlive = g.FrameCount;
    }

    // If the node just turned visible and is part of a hierarchy, it doesn't have a Size assigned by DockNodeTreeUpdatePosSize() yet,
    // so we're forcing a Pos/Size update from the first ancestor that is already visible (often it will be the root node).
    // If we don't do this, the window will be assigned a zero-size on its first frame, which won't ideally warm up the layout.
    // This is a little wonky because we don't normally update the Pos/Size of visible node mid-frame.
    if (!node->IsVisible)
    {
        ImGuiDockNode* ancestor_node = node;
        while (!ancestor_node->IsVisible && ancestor_node->ParentNode)
            ancestor_node = ancestor_node->ParentNode;
        // IM_ASSERT(ancestor_node->Size.x > 0f32 && ancestor_node->Size.y > 0f32);
        DockNodeUpdateHasCentralNodeChild(DockNodeGetRootNode(ancestor_node));
        DockNodeTreeUpdatePosSize(ancestor_node, ancestor_node->Pos, ancestor_node->Size, node);
    }

    // Add window to node
    let mut node_was_visible: bool =  node->IsVisible;
    DockNodeAddWindow(node, window, true);
    node->IsVisible = node_was_visible; // Don't mark visible right away (so DockContextEndFrame() doesn't render it, maybe other side effects? will see)
    // IM_ASSERT(node == window.DockNode);
    return node;
}

c_void ImGui::BeginDocked(ImGuiWindow* window, bool* p_open)
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
    if (node != NULL)
        // IM_ASSERT(window.DockId == node->ID);
    if (window.DockId != 0 && node == NULL)
    {
        node = DockContextBindNodeToWindow(ctx, window);
        if (node == NULL)
            return;
    }

// #if 0
    // Undock if the ImGuiDockNodeFlags_NoDockingInCentralNode got set
    if (node->IsCentralNode && (node->Flags & ImGuiDockNodeFlags_NoDockingInCentralNode))
    {
        DockContextProcessUndockWindow(ctx, window);
        return;
    }
// #endif

    // Undock if our dockspace node disappeared
    // Note how we are testing for LastFrameAlive and NOT LastFrameActive. A DockSpace node can be maintained alive while being inactive with ImGuiDockNodeFlags_KeepAliveOnly.
    if (node->LastFrameAlive < g.FrameCount)
    {
        // If the window has been orphaned, transition the docknode to an implicit node processed in DockContextNewFrameUpdateDocking()
        ImGuiDockNode* root_node = DockNodeGetRootNode(node);
        if (root_node->LastFrameAlive < g.FrameCount)
            DockContextProcessUndockWindow(ctx, window);
        else
            window.DockIsActive = true;
        return;
    }

    // Store style overrides
    for (c_int color_n = 0; color_n < ImGuiWindowDockStyleCol_COUNT; color_n++)
        window.DockStyle.Colors[color_n] = ColorConvertFloat4ToU32(g.Style.Colors[GWindowDockStyleColors[color_n]]);

    // Fast path return. It is common for windows to hold on a persistent DockId but be the only visible window,
    // and never create neither a host window neither a tab bar.
    // FIXME-DOCK: replace ->HostWindow NULL compare with something more explicit (~was initially intended as a first frame test)
    if (node->HostWindow == NULL)
    {
        if (node->State == ImGuiDockNodeState_HostWindowHiddenBecauseWindowsAreResizing)
            window.DockIsActive = true;
        if (node->Windows.Size > 1)
            DockNodeHideWindowDuringHostWindowCreation(window);
        return;
    }

    // We can have zero-sized nodes (e.g. children of a small-size dockspace)
    // IM_ASSERT(node->HostWindow);
    // IM_ASSERT(node->IsLeafNode());
    // IM_ASSERT(node->Size.x >= 0f32 && node->Size.y >= 0f32);
    node->State = ImGuiDockNodeState_HostWindowVisible;

    // Undock if we are submitted earlier than the host window
    if (!(node->MergedFlags & ImGuiDockNodeFlags_KeepAliveOnly) && window.BeginOrderWithinContext < node->Hostwindow.BeginOrderWithinContext)
    {
        DockContextProcessUndockWindow(ctx, window);
        return;
    }

    // Position/Size window
    SetNextWindowPos(node->Pos);
    SetNextWindowSize(node->Size);
    g.NextWindowData.PosUndock = false; // Cancel implicit undocking of SetNextWindowPos()
    window.DockIsActive = true;
    window.DockNodeIsVisible = true;
    window.DockTabIsVisible = false;
    if (node->MergedFlags & ImGuiDockNodeFlags_KeepAliveOnly)
        return;

    // When the window is selected we mark it as visible.
    if (node->VisibleWindow == window)
        window.DockTabIsVisible = true;

    // Update window flag
    // IM_ASSERT((window.Flags & ImGuiWindowFlags_ChildWindow) == 0);
    window.Flags |= ImGuiWindowFlags_ChildWindow | ImGuiWindowFlags_AlwaysUseWindowPadding | ImGuiWindowFlags_NoResize;
    if (node->IsHiddenTabBar() || node->IsNoTabBar())
        window.Flags |= ImGuiWindowFlags_NoTitleBar;
    else
        window.Flags &= ~ImGuiWindowFlags_NoTitleBar;      // Clear the NoTitleBar flag in case the user set it: confusingly enough we need a title bar height so we are correctly offset, but it won't be displayed!

    // Save new dock order only if the window has been visible once already
    // This allows multiple windows to be created in the same frame and have their respective dock orders preserved.
    if (node->TabBar && window.WasActive)
        window.DockOrder = (c_short)DockNodeGetTabOrder(window);

    if ((node->WantCloseAll || node->WantCloseTabId == window.TabId) && p_open != NULL)
        *p_open = false;

    // Update ChildId to allow returning from Child to Parent with Escape
    ImGuiWindow* parent_window = window.DockNode->HostWindow;
    window.ChildId = parent_window.GetID(window.Name);
}

c_void ImGui::BeginDockableDragDropSource(ImGuiWindow* window)
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
        for (c_int color_n = 0; color_n < ImGuiWindowDockStyleCol_COUNT; color_n++)
            window.DockStyle.Colors[color_n] = ColorConvertFloat4ToU32(g.Style.Colors[GWindowDockStyleColors[color_n]]);
    }
}

c_void ImGui::BeginDockableDragDropTarget(ImGuiWindow* window)
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
    *const ImGuiPayload payload = &g.DragDropPayload;
    if (!payload->IsDataType(IMGUI_PAYLOAD_TYPE_WINDOW) || !DockNodeIsDropAllowed(window, *(ImGuiWindow**)payload->Data))
    {
        EndDragDropTarget();
        return;
    }

    ImGuiWindow* payload_window = *(ImGuiWindow**)payload->Data;
    if (AcceptDragDropPayload(IMGUI_PAYLOAD_TYPE_WINDOW, ImGuiDragDropFlags_AcceptBeforeDelivery | ImGuiDragDropFlags_AcceptNoDrawDefaultRect))
    {
        // Select target node
        // (Important: we cannot use g.HoveredDockNode here! Because each of our target node have filters based on payload, each candidate drop target will do its own evaluation)
        let mut dock_into_floating_window: bool =  false;
        ImGuiDockNode* node = None;
        if (window.DockNodeAsHost)
        {
            // Cannot assume that node will != NULL even though we passed the rectangle test: it depends on padding/spacing handled by DockNodeTreeFindVisibleNodeByPos().
            node = DockNodeTreeFindVisibleNodeByPos(window.DockNodeAsHost, g.IO.MousePos);

            // There is an edge case when docking into a dockspace which only has _inactive_ nodes (because none of the windows are active)
            // In this case we need to fallback into any leaf mode, possibly the central node.
            // FIXME-20181220: We should not have to test for IsLeafNode() here but we have another bug to fix first.
            if (node && node->IsDockSpace() && node->IsRootNode())
                node = (node->CentralNode && node->IsLeafNode()) ? node->CentralNode : DockNodeTreeFindFallbackLeafNode(node);
        }
        else
        {
            if (window.DockNode)
                node = window.DockNode;
            else
                dock_into_floating_window = true; // Dock into a regular window
        }

        const ImRect explicit_target_rect = (node && node->TabBar && !node->IsHiddenTabBar() && !node->IsNoTabBar()) ? node->TabBar->BarRect : ImRect(window.Pos, window.Pos + ImVec2(window.Size.x, GetFrameHeight()));
        let is_explicit_target: bool = g.IO.ConfigDockingWithShift || IsMouseHoveringRect(explicit_target_rect.Min, explicit_target_rect.Max);

        // Preview docking request and find out split direction/ratio
        //let do_preview: bool = true;     // Ignore testing for payload->IsPreview() which removes one frame of delay, but breaks overlapping drop targets within the same window.
        let do_preview: bool = payload->IsPreview() || payload->IsDelivery();
        if (do_preview && (node != NULL || dock_into_floating_window))
        {
            ImGuiDockPreviewData split_inner;
            ImGuiDockPreviewData split_outer;
            ImGuiDockPreviewData* split_data = &split_inner;
            if (node && (node->ParentNode || node->IsCentralNode()))
                if (ImGuiDockNode* root_node = DockNodeGetRootNode(node))
                {
                    DockNodePreviewDockSetup(window, root_node, payload_window, NULL, &split_outer, is_explicit_target, true);
                    if (split_outer.IsSplitDirExplicit)
                        split_data = &split_outer;
                }
            DockNodePreviewDockSetup(window, node, payload_window, NULL, &split_inner, is_explicit_target, false);
            if (split_data == &split_outer)
                split_inner.IsDropAllowed = false;

            // Draw inner then outer, so that previewed tab (in inner data) will be behind the outer drop boxes
            DockNodePreviewDockRender(window, node, payload_window, &split_inner);
            DockNodePreviewDockRender(window, node, payload_window, &split_outer);

            // Queue docking request
            if (split_data->IsDropAllowed && payload->IsDelivery())
                DockContextQueueDock(ctx, window, split_data->SplitNode, payload_window, split_data->SplitDir, split_data->SplitRatio, split_data == &split_outer);
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

static c_void ImGui::DockSettingsRenameNodeReferences(ImGuiID old_node_id, ImGuiID new_node_id)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    IMGUI_DEBUG_LOG_DOCKING("[docking] DockSettingsRenameNodeReferences: from 0x%08X -> to 0x%08X\n", old_node_id, new_node_id);
    for (c_int window_n = 0; window_n < g.Windows.Size; window_n++)
    {
        ImGuiWindow* window = g.Windows[window_n];
        if (window.DockId == old_node_id && window.DockNode == NULL)
            window.DockId = new_node_id;
    }
    //// FIXME-OPT: We could remove this loop by storing the index in the map
    for (ImGuiWindowSettings* settings = g.SettingsWindows.begin(); settings != None; settings = g.SettingsWindows.next_chunk(settings))
        if (settings->DockId == old_node_id)
            settings->DockId = new_node_id;
}

// Remove references stored in ImGuiWindowSettings to the given ImGuiDockNodeSettings
static c_void ImGui::DockSettingsRemoveNodeReferences(ImGuiID* node_ids, c_int node_ids_count)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    c_int found = 0;
    //// FIXME-OPT: We could remove this loop by storing the index in the map
    for (ImGuiWindowSettings* settings = g.SettingsWindows.begin(); settings != None; settings = g.SettingsWindows.next_chunk(settings))
        for (c_int node_n = 0; node_n < node_ids_count; node_n++)
            if (settings->DockId == node_ids[node_n])
            {
                settings->DockId = 0;
                settings->DockOrder = -1;
                if (++found < node_ids_count)
                    break;
                return;
            }
}

static ImGuiDockNodeSettings* ImGui::DockSettingsFindNodeSettings(ImGuiContext* ctx, ImGuiID id)
{
    // FIXME-OPT
    ImGuiDockContext* dc  = &ctx->DockContext;
    for (c_int n = 0; n < dc->NodesSettings.Size; n++)
        if (dc->NodesSettings[n].ID == id)
            return &dc->NodesSettings[n];
    return NULL;
}

// Clear settings data
static c_void ImGui::DockSettingsHandler_ClearAll(ImGuiContext* ctx, ImGuiSettingsHandler*)
{
    ImGuiDockContext* dc  = &ctx->DockContext;
    dc->NodesSettings.clear();
    DockContextClearNodes(ctx, 0, true);
}

// Recreate nodes based on settings data
static c_void ImGui::DockSettingsHandler_ApplyAll(ImGuiContext* ctx, ImGuiSettingsHandler*)
{
    // Prune settings at boot time only
    ImGuiDockContext* dc  = &ctx->DockContext;
    if (ctx->Windows.Size == 0)
        DockContextPruneUnusedSettingsNodes(ctx);
    DockContextBuildNodesFromSettings(ctx, dc->NodesSettings.Data, dc->NodesSettings.Size);
    DockContextBuildAddWindowsToNodes(ctx, 0);
}

static *mut c_void ImGui::DockSettingsHandler_ReadOpen(ImGuiContext*, ImGuiSettingsHandler*, *const char name)
{
    if (strcmp(name, "Data") != 0)
        return NULL;
    return (*mut c_void)1;
}

static c_void ImGui::DockSettingsHandler_ReadLine(ImGuiContext* ctx, ImGuiSettingsHandler*, *mut c_void, *const char line)
{
    char c = 0;
    c_int x = 0, y = 0;
    c_int r = 0;

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
        if (sscanf(line, " Pos=%i,%i%n",  &x, &y, &r) == 2)         { line += r; node.Pos = ImVec2ih((c_short)x, (c_short)y); } else return;
        if (sscanf(line, " Size=%i,%i%n", &x, &y, &r) == 2)         { line += r; node.Size = ImVec2ih((c_short)x, (c_short)y); } else return;
    }
    else
    {
        if (sscanf(line, " SizeRef=%i,%i%n", &x, &y, &r) == 2)      { line += r; node.SizeRef = ImVec2ih((c_short)x, (c_short)y); }
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

static c_void DockSettingsHandler_DockNodeToSettings(ImGuiDockContext* dc, ImGuiDockNode* node, c_int depth)
{
    ImGuiDockNodeSettings node_settings;
    // IM_ASSERT(depth < (1 << (sizeof(node_settings.Depth) << 3)));
    node_settings.ID = node->ID;
    node_settings.ParentNodeId = node->ParentNode ? node->ParentNode->ID : 0;
    node_settings.ParentWindowId = (node->IsDockSpace() && node->HostWindow && node->Hostwindow.ParentWindow) ? node->Hostwindow.Parentwindow.ID : 0;
    node_settings.SelectedTabId = node->SelectedTabId;
    node_settings.SplitAxis = (node->IsSplitNode() ? node->SplitAxis : ImGuiAxis_None);
    node_settings.Depth = depth;
    node_settings.Flags = (node->LocalFlags & ImGuiDockNodeFlags_SavedFlagsMask_);
    node_settings.Pos = ImVec2ih(node->Pos);
    node_settings.Size = ImVec2ih(node->Size);
    node_settings.SizeRef = ImVec2ih(node->SizeRe0f32);
    dc->NodesSettings.push(node_settings);
    if (node->ChildNodes[0])
        DockSettingsHandler_DockNodeToSettings(dc, node->ChildNodes[0], depth + 1);
    if (node->ChildNodes[1])
        DockSettingsHandler_DockNodeToSettings(dc, node->ChildNodes[1], depth + 1);
}

static c_void ImGui::DockSettingsHandler_WriteAll(ImGuiContext* ctx, ImGuiSettingsHandler* handler, ImGuiTextBuffer* bu0f32)
{
    ImGuiContext& g = *ctx;
    ImGuiDockContext* dc = &ctx->DockContext;
    if (!(g.IO.ConfigFlags & ImGuiConfigFlags_DockingEnable))
        return;

    // Gather settings data
    // (unlike our windows settings, because nodes are always built we can do a full rewrite of the SettingsNode buffer)
    dc->NodesSettings.resize(0);
    dc->NodesSettings.reserve(dc->Nodes.Data.Size);
    for (c_int n = 0; n < dc->Nodes.Data.Size; n++)
        if (ImGuiDockNode* node = (ImGuiDockNode*)dc->Nodes.Data[n].val_p)
            if (node->IsRootNode())
                DockSettingsHandler_DockNodeToSettings(dc, node, 0);

    c_int max_depth = 0;
    for (c_int node_n = 0; node_n < dc->NodesSettings.Size; node_n++)
        max_depth = ImMax(dc->NodesSettings[node_n].Depth, max_depth);

    // Write to text buffer
    buf->appendf("[%s][Data]\n", handler->TypeName);
    for (c_int node_n = 0; node_n < dc->NodesSettings.Size; node_n++)
    {
        let line_start_pos: c_int = buf->size(); (c_void)line_start_pos;
        *const ImGuiDockNodeSettings node_settings = &dc->NodesSettings[node_n];
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
            if (node->IsDockSpace() && node->HostWindow && node->Hostwindow.ParentWindow)
                buf->appendf(" ; in '%s'", node->Hostwindow.Parentwindow.Name);
            // Iterate settings so we can give info about windows that didn't exist during the session.
            c_int contains_window = 0;
            for (ImGuiWindowSettings* settings = g.SettingsWindows.begin(); settings != None; settings = g.SettingsWindows.next_chunk(settings))
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
static *const char GetClipboardTextFn_DefaultImpl(*mut c_void)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.ClipboardHandlerData.clear();
    if (!::OpenClipboard(NULL))
        return NULL;
    HANDLE wbuf_handle = ::GetClipboardData(CF_UNICODETEXT);
    if (wbuf_handle == NULL)
    {
        ::CloseClipboard();
        return NULL;
    }
    if (*const WCHAR wbuf_global = (*const WCHAR)::GlobalLock(wbuf_handle))
    {
        c_int buf_len = ::WideCharToMultiByte(CP_UTF8, 0, wbuf_global, -1, NULL, 0, NULL, NULL);
        g.ClipboardHandlerData.resize(buf_len);
        ::WideCharToMultiByte(CP_UTF8, 0, wbuf_global, -1, g.ClipboardHandlerData.Data, buf_len, NULL, NULL);
    }
    ::GlobalUnlock(wbuf_handle);
    ::CloseClipboard();
    return g.ClipboardHandlerData.Data;
}

static c_void SetClipboardTextFn_DefaultImpl(*mut c_void, *const char text)
{
    if (!::OpenClipboard(NULL))
        return;
    let wbuf_length: c_int = ::MultiByteToWideChar(CP_UTF8, 0, text, -1, NULL, 0);
    HGLOBAL wbuf_handle = ::GlobalAlloc(GMEM_MOVEABLE, wbuf_length * sizeof(WCHAR));
    if (wbuf_handle == NULL)
    {
        ::CloseClipboard();
        return;
    }
    WCHAR* wbuf_global = (WCHAR*)::GlobalLock(wbuf_handle);
    ::MultiByteToWideChar(CP_UTF8, 0, text, -1, wbuf_global, wbuf_length);
    ::GlobalUnlock(wbuf_handle);
    ::EmptyClipboard();
    if (::SetClipboardData(CF_UNICODETEXT, wbuf_handle) == NULL)
        ::GlobalFree(wbuf_handle);
    ::CloseClipboard();
}

// #elif defined(__APPLE__) && TARGET_OS_OSX && defined(IMGUI_ENABLE_OSX_DEFAULT_CLIPBOARD_FUNCTIONS)

// #include <Carbon/Carbon.h>  // Use old API to avoid need for separate .mm file
static PasteboardRef main_clipboard = 0;

// OSX clipboard implementation
// If you enable this you will need to add '-framework ApplicationServices' to your linker command-line!
static c_void SetClipboardTextFn_DefaultImpl(*mut c_void, *const char text)
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

static *const char GetClipboardTextFn_DefaultImpl(*mut c_void)
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
                c_int length = CFDataGetLength(cf_data);
                g.ClipboardHandlerData.resize(length + 1);
                CFDataGetBytes(cf_data, CFRangeMake(0, length), (UInt8*)g.ClipboardHandlerData.Data);
                g.ClipboardHandlerData[length] = 0;
                CFRelease(cf_data);
                return g.ClipboardHandlerData.Data;
            }
        }
    }
    return NULL;
}

// #else

// Local Dear ImGui-only clipboard implementation, if user hasn't defined better clipboard handlers.
static *const char GetClipboardTextFn_DefaultImpl(*mut c_void)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.ClipboardHandlerData.empty() ? NULL : g.ClipboardHandlerData.begin();
}

static c_void SetClipboardTextFn_DefaultImpl(*mut c_void, *const char text)
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
    HWND hwnd = (HWND)viewport->PlatformHandleRaw;
// #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
    if (hwnd == 0)
        hwnd = (HWND)ImGui::GetIO().ImeWindowHandle;
// #endif
    if (hwnd == 0)
        return;

    //::ImmAssociateContextEx(hwnd, NULL, data->WantVisible ? IACE_DEFAULT : 0);
    if (HIMC himc = ::ImmGetContext(hwnd))
    {
        COMPOSITIONFORM composition_form = {};
        composition_form.ptCurrentPos.x = (LONG)(data.InputPos.x - viewport->Pos.x);
        composition_form.ptCurrentPos.y = (LONG)(data.InputPos.y - viewport->Pos.y);
        composition_form.dwStyle = CFS_FORCE_POSITION;
        ::ImmSetCompositionWindow(himc, &composition_form);
        CANDIDATEFORM candidate_form = {};
        candidate_form.dwStyle = CFS_CANDIDATEPOS;
        candidate_form.ptCurrentPos.x = (LONG)(data.InputPos.x - viewport->Pos.x);
        candidate_form.ptCurrentPos.y = (LONG)(data.InputPos.y - viewport->Pos.y);
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

c_void ImGui::DebugRenderViewportThumbnail(ImDrawList* draw_list, *mut ImGuiViewportP viewport, const ImRect& bb)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;

    ImVec2 scale = bb.GetSize() / viewport->Size;
    ImVec2 off = bb.Min - viewport->Pos * scale;
    c_float alpha_mul = (viewport->Flags & ImGuiViewportFlags_Minimized) ? 0.3f32 : 1f32;
    window.DrawList.AddRectFilled(bb.Min, bb.Max, ImGui::GetColorU32(ImGuiCol_Border, alpha_mul * 0.400f32));
    for (c_int i = 0; i != g.Windows.Size; i++)
    {
        ImGuiWindow* thumb_window = g.Windows[i];
        if (!thumb_window.WasActive || (thumb_window.Flags & ImGuiWindowFlags_ChildWindow))
            continue;
        if (thumb_window.Viewport != viewport)
            continue;

        ImRect thumb_r = thumb_window.Rect();
        ImRect title_r = thumb_window.TitleBarRect();
        thumb_r = ImRect(ImFloor(off + thumb_r.Min * scale), ImFloor(off +  thumb_r.Max * scale));
        title_r = ImRect(ImFloor(off + title_r.Min * scale), ImFloor(off +  ImVec2(title_r.Max.x, title_r.Min.y) * scale) + ImVec2(0,5)); // Exaggerate title bar height
        thumb_r.ClipWithFull(bb);
        title_r.ClipWithFull(bb);
        let window_is_focused: bool = (g.NavWindow && thumb_window.RootWindowForTitleBarHighlight == g.Navwindow.RootWindowForTitleBarHighlight);
        window.DrawList.AddRectFilled(thumb_r.Min, thumb_r.Max, GetColorU32(ImGuiCol_WindowBg, alpha_mul));
        window.DrawList.AddRectFilled(title_r.Min, title_r.Max, GetColorU32(window_is_focused ? ImGuiCol_TitleBgActive : ImGuiCol_TitleBg, alpha_mul));
        window.DrawList.AddRect(thumb_r.Min, thumb_r.Max, GetColorU32(ImGuiCol_Border, alpha_mul));
        window.DrawList.AddText(g.Font, g.FontSize * 1f32, title_r.Min, GetColorU32(ImGuiCol_Text, alpha_mul), thumb_window.Name, FindRenderedTextEnd(thumb_window.Name));
    }
    draw_list->AddRect(bb.Min, bb.Max, GetColorU32(ImGuiCol_Border, alpha_mul));
}

static c_void RenderViewportsThumbnails()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;

    // We don't display full monitor bounds (we could, but it often looks awkward), instead we display just enough to cover all of our viewports.
    c_float SCALE = 1f32 / 8.0f32;
    ImRect bb_full(f32::MAX, f32::MAX, -f32::MAX, -f32::MAX);
    for (c_int n = 0; n < g.Viewports.Size; n++)
        bb_full.Add(g.Viewports[n]->GetMainRect());
    ImVec2 p = window.DC.CursorPos;
    ImVec2 off = p - bb_full.Min * SCALE;
    for (c_int n = 0; n < g.Viewports.Size; n++)
    {
        *mut ImGuiViewportP viewport = g.Viewports[n];
        ImRect viewport_draw_bb(off + (viewport->Pos) * SCALE, off + (viewport->Pos + viewport->Size) * SCALE);
        ImGui::DebugRenderViewportThumbnail(window.DrawList, viewport, viewport_draw_bb);
    }
    ImGui::Dummy(bb_full.GetSize() * SCALE);
}

static c_int IMGUI_CDECL ViewportComparerByFrontMostStampCount(*const c_void lhs, *const c_void rhs)
{
    const *mut ImGuiViewportP a = *(const *mut ImGuiViewportP const*)lhs;
    const *mut ImGuiViewportP b = *(const *mut ImGuiViewportP const*)rhs;
    return b->LastFrontMostStampCount - a->LastFrontMostStampCount;
}

// Helper tool to diagnose between text encoding issues and font loading issues. Pass your UTF-8 string and verify that there are correct.
c_void ImGui::DebugTextEncoding(*const char str)
{
    Text("Text: \"%s\"", str);
    if (!BeginTable("list", 4, ImGuiTableFlags_Borders | ImGuiTableFlags_RowBg | ImGuiTableFlags_SizingFixedFit))
        return;
    TableSetupColumn("Offset");
    TableSetupColumn("UTF-8");
    TableSetupColumn("Glyph");
    TableSetupColumn("Codepoint");
    TableHeadersRow();
    for (*const char p = str; *p != 0; )
    {
        c_uint c;
        let c_utf8_len: c_int = ImTextCharFromUtf8(&c, p, NULL);
        TableNextColumn();
        Text("%d", (p - str));
        TableNextColumn();
        for (c_int byte_index = 0; byte_index < c_utf8_len; byte_index++)
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
static c_void MetricsHelpMarker(*const char desc)
{
    ImGui::TextDisabled("(?)");
    if (ImGui::IsItemHovered(ImGuiHoveredFlags_DelayShort))
    {
        ImGui::BeginTooltip();
        ImGui::PushTextWrapPos(ImGui::GetFontSize() * 35.00f32);
        ImGui::TextUnformatted(desc);
        ImGui::PopTextWrapPos();
        ImGui::EndTooltip();
    }
}

// [DEBUG] List fonts in a font atlas and display its texture
c_void ImGui::ShowFontAtlas(ImFontAtlas* atlas)
{
    for (c_int i = 0; i < atlas->Fonts.Size; i++)
    {
        ImFont* font = atlas->Fonts[i];
        PushID(font);
        DebugNodeFont(font);
        PopID();
    }
    if (TreeNode("Atlas texture", "Atlas texture (%dx%d pixels)", atlas->TexWidth, atlas->TexHeight))
    {
        ImVec4 tint_col = ImVec4(1f32, 1f32, 1f32, 1f32);
        ImVec4 border_col = ImVec4(1f32, 1f32, 1f32, 0.5f32);
        Image(atlas->TexID, ImVec2(atlas->TexWidth, atlas->TexHeight), ImVec2(0f32, 0f32), ImVec2(1f32, 1f32), tint_col, border_col);
        TreePop();
    }
}

c_void ImGui::ShowMetricsWindow(bool* p_open)
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
    *const char wrt_rects_names[WRT_Count] = { "OuterRect", "OuterRectClipped", "InnerRect", "InnerClipRect", "WorkRect", "Content", "ContentIdeal", "ContentRegionRect" };
    enum { TRT_OuterRect, TRT_InnerRect, TRT_WorkRect, TRT_HostClipRect, TRT_InnerClipRect, TRT_BackgroundClipRect, TRT_ColumnsRect, TRT_ColumnsWorkRect, TRT_ColumnsClipRect, TRT_ColumnsContentHeadersUsed, TRT_ColumnsContentHeadersIdeal, TRT_ColumnsContentFrozen, TRT_ColumnsContentUnfrozen, TRT_Count }; // Tables Rect Type
    *const char trt_rects_names[TRT_Count] = { "OuterRect", "InnerRect", "WorkRect", "HostClipRect", "InnerClipRect", "BackgroundClipRect", "ColumnsRect", "ColumnsWorkRect", "ColumnsClipRect", "ColumnsContentHeadersUsed", "ColumnsContentHeadersIdeal", "ColumnsContentFrozen", "ColumnsContentUnfrozen" };
    if (cfg->ShowWindowsRectsType < 0)
        cfg->ShowWindowsRectsType = WRT_WorkRect;
    if (cfg->ShowTablesRectsType < 0)
        cfg->ShowTablesRectsType = TRT_WorkRect;

    struct Funcs
    {
        static ImRect GetTableRect(ImGuiTable* table, c_int rect_type, c_int n)
        {
            ImGuiTableInstanceData* table_instance = TableGetInstanceData(table, table.InstanceCurrent); // Always using last submitted instance
            if (rect_type == TRT_OuterRect)                     { return table.OuterRect; }
            else if (rect_type == TRT_InnerRect)                { return table.InnerRect; }
            else if (rect_type == TRT_WorkRect)                 { return table.WorkRect; }
            else if (rect_type == TRT_HostClipRect)             { return table.HostClipRect; }
            else if (rect_type == TRT_InnerClipRect)            { return table.InnerClipRect; }
            else if (rect_type == TRT_BackgroundClipRect)       { return table.BgClipRect; }
            else if (rect_type == TRT_ColumnsRect)              { ImGuiTableColumn* c = &table.Columns[n]; return ImRect(c->MinX, table.InnerClipRect.Min.y, c->MaxX, table.InnerClipRect.Min.y + table_instance->LastOuterHeight); }
            else if (rect_type == TRT_ColumnsWorkRect)          { ImGuiTableColumn* c = &table.Columns[n]; return ImRect(c->WorkMinX, table.WorkRect.Min.y, c->WorkMaxX, table.WorkRect.Max.y); }
            else if (rect_type == TRT_ColumnsClipRect)          { ImGuiTableColumn* c = &table.Columns[n]; return c->ClipRect; }
            else if (rect_type == TRT_ColumnsContentHeadersUsed){ ImGuiTableColumn* c = &table.Columns[n]; return ImRect(c->WorkMinX, table.InnerClipRect.Min.y, c->ContentMaxXHeadersUsed, table.InnerClipRect.Min.y + table_instance->LastFirstRowHeight); } // Note: y1/y2 not always accurate
            else if (rect_type == TRT_ColumnsContentHeadersIdeal){ImGuiTableColumn* c = &table.Columns[n]; return ImRect(c->WorkMinX, table.InnerClipRect.Min.y, c->ContentMaxXHeadersIdeal, table.InnerClipRect.Min.y + table_instance->LastFirstRowHeight); }
            else if (rect_type == TRT_ColumnsContentFrozen)     { ImGuiTableColumn* c = &table.Columns[n]; return ImRect(c->WorkMinX, table.InnerClipRect.Min.y, c->ContentMaxXFrozen, table.InnerClipRect.Min.y + table_instance->LastFirstRowHeight); }
            else if (rect_type == TRT_ColumnsContentUnfrozen)   { ImGuiTableColumn* c = &table.Columns[n]; return ImRect(c->WorkMinX, table.InnerClipRect.Min.y + table_instance->LastFirstRowHeight, c->ContentMaxXUnfrozen, table.InnerClipRect.Max.y); }
            // IM_ASSERT(0);
            return ImRect();
        }

        static ImRect GetWindowRect(ImGuiWindow* window, c_int rect_type)
        {
            if (rect_type == WRT_OuterRect)                 { return window.Rect(); }
            else if (rect_type == WRT_OuterRectClipped)     { return window.OuterRectClipped; }
            else if (rect_type == WRT_InnerRect)            { return window.InnerRect; }
            else if (rect_type == WRT_InnerClipRect)        { return window.InnerClipRect; }
            else if (rect_type == WRT_WorkRect)             { return window.WorkRect; }
            else if (rect_type == WRT_Content)       { ImVec2 min = window.InnerRect.Min - window.Scroll + window.WindowPadding; return ImRect(min, min + window.ContentSize); }
            else if (rect_type == WRT_ContentIdeal)         { ImVec2 min = window.InnerRect.Min - window.Scroll + window.WindowPadding; return ImRect(min, min + window.ContentSizeIdeal); }
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
        MetricsHelpMarker("You can also call ImGui::DebugTextEncoding() from your code with a given string to test that your UTF-8 encoding settings are correct.");
        if (show_encoding_viewer)
        {
            static buf: [c_char;100] = "";
            SetNextItemWidth(-FLT_MIN);
            InputText("##Text", buf, IM_ARRAYSIZE(bu0f32));
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
        MetricsHelpMarker("You can also call ImGui::ShowDebugLogWindow() from your code.");

        // Stack Tool is your best friend!
        Checkbox("Show Stack Tool", &cfg->ShowStackTool);
        SameLine();
        MetricsHelpMarker("You can also call ImGui::ShowStackToolWindow() from your code.");

        Checkbox("Show windows begin order", &cfg->ShowWindowsBeginOrder);
        Checkbox("Show windows rectangles", &cfg->ShowWindowsRects);
        SameLine();
        SetNextItemWidth(GetFontSize() * 12);
        cfg->ShowWindowsRects |= Combo("##show_windows_rect_type", &cfg->ShowWindowsRectsType, wrt_rects_names, WRT_Count, WRT_Count);
        if (cfg->ShowWindowsRects && g.NavWindow != NULL)
        {
            BulletText("'%s':", g.Navwindow.Name);
            Indent();
            for (c_int rect_n = 0; rect_n < WRT_Count; rect_n++)
            {
                ImRect r = Funcs::GetWindowRect(g.NavWindow, rect_n);
                Text("(%6.1f,%6.10f32) (%6.1f,%6.10f32) Size (%6.1f,%6.10f32) %s", r.Min.x, r.Min.y, r.Max.x, r.Max.y, r.GetWidth(), r.GetHeight(), wrt_rects_names[rect_n]);
            }
            Unindent();
        }

        Checkbox("Show tables rectangles", &cfg->ShowTablesRects);
        SameLine();
        SetNextItemWidth(GetFontSize() * 12);
        cfg->ShowTablesRects |= Combo("##show_table_rects_type", &cfg->ShowTablesRectsType, trt_rects_names, TRT_Count, TRT_Count);
        if (cfg->ShowTablesRects && g.NavWindow != NULL)
        {
            for (c_int table_n = 0; table_n < g.Tables.GetMapSize(); table_n++)
            {
                ImGuiTable* table = g.Tables.TryGetMapData(table_n);
                if (table == NULL || table.LastFrameActive < g.FrameCount - 1 || (table.OuterWindow != g.NavWindow && table.InnerWindow != g.NavWindow))
                    continue;

                BulletText("Table 0x%08X (%d columns, in '%s')", table.ID, table.ColumnsCount, table.Outerwindow.Name);
                if (IsItemHovered())
                    GetForegroundDrawList()->AddRect(table.OuterRect.Min - ImVec2(1, 1), table.OuterRect.Max + ImVec2(1, 1), IM_COL32(255, 255, 0, 255), 0f32, 0, 2.00f32);
                Indent();
                buf: [c_char;128];
                for (c_int rect_n = 0; rect_n < TRT_Count; rect_n++)
                {
                    if (rect_n >= TRT_ColumnsRect)
                    {
                        if (rect_n != TRT_ColumnsRect && rect_n != TRT_ColumnsClipRect)
                            continue;
                        for (c_int column_n = 0; column_n < table.ColumnsCount; column_n++)
                        {
                            ImRect r = Funcs::GetTableRect(table, rect_n, column_n);
                            ImFormatString(buf, IM_ARRAYSIZE(bu0f32), "(%6.1f,%6.10f32) (%6.1f,%6.10f32) Size (%6.1f,%6.10f32) Col %d %s", r.Min.x, r.Min.y, r.Max.x, r.Max.y, r.GetWidth(), r.GetHeight(), column_n, trt_rects_names[rect_n]);
                            Selectable(bu0f32);
                            if (IsItemHovered())
                                GetForegroundDrawList()->AddRect(r.Min - ImVec2(1, 1), r.Max + ImVec2(1, 1), IM_COL32(255, 255, 0, 255), 0f32, 0, 2.00f32);
                        }
                    }
                    else
                    {
                        ImRect r = Funcs::GetTableRect(table, rect_n, -1);
                        ImFormatString(buf, IM_ARRAYSIZE(bu0f32), "(%6.1f,%6.10f32) (%6.1f,%6.10f32) Size (%6.1f,%6.10f32) %s", r.Min.x, r.Min.y, r.Max.x, r.Max.y, r.GetWidth(), r.GetHeight(), trt_rects_names[rect_n]);
                        Selectable(bu0f32);
                        if (IsItemHovered())
                            GetForegroundDrawList()->AddRect(r.Min - ImVec2(1, 1), r.Max + ImVec2(1, 1), IM_COL32(255, 255, 0, 255), 0f32, 0, 2.00f32);
                    }
                }
                Unindent();
            }
        }

        TreePop();
    }

    // Windows
    if (TreeNode("Windows", "Windows (%d)", g.Windows.Size))
    {
        //SetNextItemOpen(true, ImGuiCond_Once);
        DebugNodeWindowsList(&g.Windows, "By display order");
        DebugNodeWindowsList(&g.WindowsFocusOrder, "By focus order (root windows)");
        if (TreeNode("By submission order (begin stack)"))
        {
            // Here we display windows in their submitted order/hierarchy, however note that the Begin stack doesn't constitute a Parent<>Child relationship!
            Vec<ImGuiWindow*>& temp_buffer = g.WindowsTempSortBuffer;
            temp_buffer.resize(0);
            for (c_int i = 0; i < g.Windows.Size; i++)
                if (g.Windows[i]->LastFrameActive + 1 >= g.FrameCount)
                    temp_buffer.push(g.Windows[i]);
            struct Func { static c_int IMGUI_CDECL WindowComparerByBeginOrder(*const c_void lhs, *const c_void rhs) { return ((*(*const ImGuiWindow const *)lhs)->BeginOrderWithinContext - (*(*const ImGuiWindow const*)rhs)->BeginOrderWithinContext); } };
            ImQsort(temp_buffer.Data, temp_buffer.Size, sizeof(ImGuiWindow*), Func::WindowComparerByBeginOrder);
            DebugNodeWindowsListByBeginStackParent(temp_buffer.Data, temp_buffer.Size, NULL);
            TreePop();
        }

        TreePop();
    }

    // DrawLists
    c_int drawlist_count = 0;
    for (c_int viewport_i = 0; viewport_i < g.Viewports.Size; viewport_i++)
        drawlist_count += g.Viewports[viewport_i]->DrawDataBuilder.GetDrawListCount();
    if (TreeNode("DrawLists", "DrawLists (%d)", drawlist_count))
    {
        Checkbox("Show ImDrawCmd mesh when hovering", &cfg->ShowDrawCmdMesh);
        Checkbox("Show ImDrawCmd bounding boxes when hovering", &cfg->ShowDrawCmdBoundingBoxes);
        for (c_int viewport_i = 0; viewport_i < g.Viewports.Size; viewport_i++)
        {
            *mut ImGuiViewportP viewport = g.Viewports[viewport_i];
            let mut viewport_has_drawlist: bool =  false;
            for (c_int layer_i = 0; layer_i < IM_ARRAYSIZE(viewport->DrawDataBuilder.Layers); layer_i++)
                for (c_int draw_list_i = 0; draw_list_i < viewport->DrawDataBuilder.Layers[layer_i].Size; draw_list_i++)
                {
                    if (!viewport_has_drawlist)
                        Text("Active DrawLists in Viewport #%d, ID: 0x%08X", viewport->Idx, viewport->ID);
                    viewport_has_drawlist = true;
                    DebugNodeDrawList(NULL, viewport, viewport->DrawDataBuilder.Layers[layer_i][draw_list_i], "DrawList");
                }
        }
        TreePop();
    }

    // Viewports
    if (TreeNode("Viewports", "Viewports (%d)", g.Viewports.Size))
    {
        Indent(GetTreeNodeToLabelSpacing());
        RenderViewportsThumbnails();
        Unindent(GetTreeNodeToLabelSpacing());

        let mut open: bool =  TreeNode("Monitors", "Monitors (%d)", g.PlatformIO.Monitors.Size);
        SameLine();
        MetricsHelpMarker("Dear ImGui uses monitor data:\n- to query DPI settings on a per monitor basis\n- to position popup/tooltips so they don't straddle monitors.");
        if (open)
        {
            for (c_int i = 0; i < g.PlatformIO.Monitors.Size; i++)
            {
                const ImGuiPlatformMonitor& mon = g.PlatformIO.Monitors[i];
                BulletText("Monitor #%d: DPI %.0f%%\n MainMin (%.0f32,%.00f32), MainMax (%.0f32,%.00f32), MainSize (%.0f32,%.00f32)\n WorkMin (%.0f32,%.00f32), WorkMax (%.0f32,%.00f32), WorkSize (%.0f32,%.00f32)",
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
            viewports.resize(g.Viewports.Size);
            memcpy(viewports.Data, g.Viewports.Data, g.Viewports.size_in_bytes());
            if (viewports.Size > 1)
                ImQsort(viewports.Data, viewports.Size, sizeof(ImGuiViewport*), ViewportComparerByFrontMostStampCount);
            for (c_int i = 0; i < viewports.Size; i++)
                BulletText("Viewport #%d, ID: 0x%08X, FrontMostStampCount = %08d, Window: \"%s\"", viewports[i]->Idx, viewports[i]->ID, viewports[i]->LastFrontMostStampCount, viewports[i]->Window ? viewports[i]->window.Name : "N/A");
            TreePop();
        }

        for (c_int i = 0; i < g.Viewports.Size; i++)
            DebugNodeViewport(g.Viewports[i]);
        TreePop();
    }

    // Details for Popups
    if (TreeNode("Popups", "Popups (%d)", g.OpenPopupStack.Size))
    {
        for (c_int i = 0; i < g.OpenPopupStack.Size; i++)
        {
            // As it's difficult to interact with tree nodes while popups are open, we display everything inline.
            *const ImGuiPopupData popup_data = &g.OpenPopupStack[i];
            ImGuiWindow* window = popup_data->Window;
            BulletText("PopupID: %08x, Window: '%s' (%s%s), BackupNavWindow '%s', ParentWindow '%s'",
                popup_data->PopupId, window ? window.Name : "NULL", window && (window.Flags & ImGuiWindowFlags_ChildWindow) ? "Child;" : "", window && (window.Flags & ImGuiWindowFlags_ChildMenu) ? "Menu;" : "",
                popup_data->BackupNavWindow ? popup_data->BackupNavwindow.Name : "NULL", window && window.ParentWindow ? window.Parentwindow.Name : "NULL");
        }
        TreePop();
    }

    // Details for TabBars
    if (TreeNode("TabBars", "Tab Bars (%d)", g.TabBars.GetAliveCount()))
    {
        for (c_int n = 0; n < g.TabBars.GetMapSize(); n++)
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
        for (c_int n = 0; n < g.Tables.GetMapSize(); n++)
            if (ImGuiTable* table = g.Tables.TryGetMapData(n))
                DebugNodeTable(table);
        TreePop();
    }

    // Details for Fonts
    ImFontAtlas* atlas = g.IO.Fonts;
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
        for (c_int n = 0; n < dc->Nodes.Data.Size; n++)
            if (ImGuiDockNode* node = (ImGuiDockNode*)dc->Nodes.Data[n].val_p)
                if (!root_nodes_only || node->IsRootNode())
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
            for (c_int n = 0; n < g.SettingsHandlers.Size; n++)
                BulletText("%s", g.SettingsHandlers[n].TypeName);
            TreePop();
        }
        if (TreeNode("SettingsWindows", "Settings packed data: Windows: %d bytes", g.SettingsWindows.size()))
        {
            for (ImGuiWindowSettings* settings = g.SettingsWindows.begin(); settings != None; settings = g.SettingsWindows.next_chunk(settings))
                DebugNodeWindowSettings(settings);
            TreePop();
        }

        if (TreeNode("SettingsTables", "Settings packed data: Tables: %d bytes", g.SettingsTables.size()))
        {
            for (ImGuiTableSettings* settings = g.SettingsTables.begin(); settings != None; settings = g.SettingsTables.next_chunk(settings))
                DebugNodeTableSettings(settings);
            TreePop();
        }

// #ifdef IMGUI_HAS_DOCK
        if (TreeNode("SettingsDocking", "Settings packed data: Docking"))
        {
            ImGuiDockContext* dc = &g.DockContext;
            Text("In SettingsWindows:");
            for (ImGuiWindowSettings* settings = g.SettingsWindows.begin(); settings != None; settings = g.SettingsWindows.next_chunk(settings))
                if (settings->DockId != 0)
                    BulletText("Window '%s' -> DockId %08X", settings->GetName(), settings->DockId);
            Text("In SettingsNodes:");
            for (c_int n = 0; n < dc->NodesSettings.Size; n++)
            {
                ImGuiDockNodeSettings* settings = &dc->NodesSettings[n];
                let mut  selected_tab_name: *const c_char = None;
                if (settings->SelectedTabId)
                {
                    if (ImGuiWindow* window = FindWindowByID(settings->SelectedTabId))
                        selected_tab_name = window.Name;
                    else if (ImGuiWindowSettings* window_settings = FindWindowSettings(settings->SelectedTabId))
                        selected_tab_name = window_settings->GetName();
                }
                BulletText("Node %08X, Parent %08X, SelectedTab %08X ('%s')", settings->ID, settings->ParentNodeId, settings->SelectedTabId, selected_tab_name ? selected_tab_name : settings->SelectedTabId ? "N/A" : "");
            }
            TreePop();
        }
// #endif // #ifdef IMGUI_HAS_DOCK

        if (TreeNode("SettingsIniData", "Settings unpacked data (.ini): %d bytes", g.SettingsIniData.size()))
        {
            InputTextMultiline("##Ini", (char*)(*mut c_void)g.SettingsIniData.c_str(), g.SettingsIniData.Buf.Size, ImVec2(-FLT_MIN, GetTextLineHeight() * 20), ImGuiInputTextFlags_ReadOnly);
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
        Text("Hoveredwindow.Root: '%s'", g.HoveredWindow ? g.Hoveredwindow.RootWindowDockTree->Name : "NULL");
        Text("HoveredWindowUnderMovingWindow: '%s'", g.HoveredWindowUnderMovingWindow ? g.HoveredWindowUnderMovingwindow.Name : "NULL");
        Text("HoveredDockNode: 0x%08X", g.DebugHoveredDockNode ? g.DebugHoveredDockNode->ID : 0);
        Text("MovingWindow: '%s'", g.MovingWindow ? g.Movingwindow.Name : "NULL");
        Text("MouseViewport: 0x%08X (UserHovered 0x%08X, LastHovered 0x%08X)", g.MouseViewport->ID, g.IO.MouseHoveredViewport, g.MouseLastHoveredViewport ? g.MouseLastHoveredViewport->ID : 0);
        Unindent();

        Text("ITEMS");
        Indent();
        Text("ActiveId: 0x%08X/0x%08X (%.2f sec), AllowOverlap: %d, Source: %s", g.ActiveId, g.ActiveIdPreviousFrame, g.ActiveIdTimer, g.ActiveIdAllowOverlap, GetInputSourceName(g.ActiveIdSource));
        Text("ActiveIdWindow: '%s'", g.ActiveIdWindow ? g.ActiveIdwindow.Name : "NULL");

        c_int active_id_using_key_input_count = 0;
        for (c_int n = ImGuiKey_NamedKey_BEGIN; n < ImGuiKey_NamedKey_END; n++)
            active_id_using_key_input_count += g.ActiveIdUsingKeyInputMask[n] ? 1 : 0;
        Text("ActiveIdUsing: NavDirMask: %X, KeyInputMask: %d key(s)", g.ActiveIdUsingNavDirMask, active_id_using_key_input_count);
        Text("HoveredId: 0x%08X (%.2f sec), AllowOverlap: %d", g.HoveredIdPreviousFrame, g.HoveredIdTimer, g.HoveredIdAllowOverlap); // Not displaying g.HoveredId as it is update mid-frame
        Text("HoverDelayId: 0x%08X, Timer: %.2f, ClearTimer: %.2f", g.HoverDelayId, g.HoverDelayTimer, g.HoverDelayClearTimer);
        Text("DragDrop: %d, SourceId = 0x%08X, Payload \"%s\" (%d bytes)", g.DragDropActive, g.DragDropPayload.SourceId, g.DragDropPayload.DataType, g.DragDropPayload.DataSize);
        Unindent();

        Text("NAV,FOCUS");
        Indent();
        Text("NavWindow: '%s'", g.NavWindow ? g.Navwindow.Name : "NULL");
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
        for (c_int n = 0; n < g.Windows.Size; n++)
        {
            ImGuiWindow* window = g.Windows[n];
            if (!window.WasActive)
                continue;
            ImDrawList* draw_list = GetForegroundDrawList(window);
            if (cfg->ShowWindowsRects)
            {
                ImRect r = Funcs::GetWindowRect(window, cfg->ShowWindowsRectsType);
                draw_list->AddRect(r.Min, r.Max, IM_COL32(255, 0, 128, 255));
            }
            if (cfg->ShowWindowsBeginOrder && !(window.Flags & ImGuiWindowFlags_ChildWindow))
            {
                buf: [c_char;32];
                ImFormatString(buf, IM_ARRAYSIZE(bu0f32), "%d", window.BeginOrderWithinContext);
                c_float font_size = GetFontSize();
                draw_list->AddRectFilled(window.Pos, window.Pos + ImVec2(font_size, font_size), IM_COL32(200, 100, 100, 255));
                draw_list->AddText(window.Pos, IM_COL32(255, 255, 255, 255), bu0f32);
            }
        }
    }

    // Overlay: Display Tables Rectangles
    if (cfg->ShowTablesRects)
    {
        for (c_int table_n = 0; table_n < g.Tables.GetMapSize(); table_n++)
        {
            ImGuiTable* table = g.Tables.TryGetMapData(table_n);
            if (table == NULL || table.LastFrameActive < g.FrameCount - 1)
                continue;
            ImDrawList* draw_list = GetForegroundDrawList(table.OuterWindow);
            if (cfg->ShowTablesRectsType >= TRT_ColumnsRect)
            {
                for (c_int column_n = 0; column_n < table.ColumnsCount; column_n++)
                {
                    ImRect r = Funcs::GetTableRect(table, cfg->ShowTablesRectsType, column_n);
                    u32 col = (table.HoveredColumnBody == column_n) ? IM_COL32(255, 255, 128, 255) : IM_COL32(255, 0, 128, 255);
                    c_float thickness = (table.HoveredColumnBody == column_n) ? 3.0f32 : 1f32;
                    draw_list->AddRect(r.Min, r.Max, col, 0f32, 0, thickness);
                }
            }
            else
            {
                ImRect r = Funcs::GetTableRect(table, cfg->ShowTablesRectsType, -1);
                draw_list->AddRect(r.Min, r.Max, IM_COL32(255, 0, 128, 255));
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
        ImDrawList* overlay_draw_list = node->HostWindow ? GetForegroundDrawList(node->HostWindow) : GetForegroundDrawList(GetMainViewport());
        p += ImFormatString(p, buf + IM_ARRAYSIZE(bu0f32) - p, "DockId: %X%s\n", node->ID, node->IsCentralNode() ? " *CentralNode*" : "");
        p += ImFormatString(p, buf + IM_ARRAYSIZE(bu0f32) - p, "WindowClass: %08X\n", node->WindowClass.ClassId);
        p += ImFormatString(p, buf + IM_ARRAYSIZE(bu0f32) - p, "Size: (%.0f32, %.00f32)\n", node->Size.x, node->Size.y);
        p += ImFormatString(p, buf + IM_ARRAYSIZE(bu0f32) - p, "SizeRef: (%.0f32, %.00f32)\n", node->SizeRef.x, node->SizeRef.y);
        c_int depth = DockNodeGetDepth(node);
        overlay_draw_list->AddRect(node->Pos + ImVec2(3, 3) * depth, node->Pos + node->Size - ImVec2(3, 3) * depth, IM_COL32(200, 100, 100, 255));
        ImVec2 pos = node->Pos + ImVec2(3, 3) * depth;
        overlay_draw_list->AddRectFilled(pos - ImVec2(1, 1), pos + CalcTextSize(bu0f32) + ImVec2(1, 1), IM_COL32(200, 100, 100, 255));
        overlay_draw_list->AddText(NULL, 0f32, pos, IM_COL32(255, 255, 255, 255), bu0f32);
    }
// #endif // #ifdef IMGUI_HAS_DOCK

    End();
}

// [DEBUG] Display contents of Columns
c_void ImGui::DebugNodeColumns(ImGuiOldColumns* columns)
{
    if (!TreeNode((*mut c_void)(uintptr_t)columns->ID, "Columns Id: 0x%08X, Count: %d, Flags: 0x%04X", columns->ID, columns->Count, columns->Flags))
        return;
    BulletText("Width: %.1f (MinX: %.1f, MaxX: %.10f32)", columns->OffMaxX - columns->OffMinX, columns->OffMinX, columns->OffMaxX);
    for (c_int column_n = 0; column_n < columns->Columns.Size; column_n++)
        BulletText("Column %02d: OffsetNorm %.3f (= %.1f px)", column_n, columns->Columns[column_n].OffsetNorm, GetColumnOffsetFromNorm(columns, columns->Columns[column_n].OffsetNorm));
    TreePop();
}

static c_void DebugNodeDockNodeFlags(ImGuiDockNodeFlags* p_flags, *const char label, bool enabled)
{
    using namespace ImGui;
    PushID(label);
    PushStyleVar(ImGuiStyleVar_FramePadding, ImVec2(0f32, 0f32));
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
c_void ImGui::DebugNodeDockNode(ImGuiDockNode* node, *const char label)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let is_alive: bool = (g.FrameCount - node->LastFrameAlive < 2);    // Submitted with ImGuiDockNodeFlags_KeepAliveOnly
    let is_active: bool = (g.FrameCount - node->LastFrameActive < 2);  // Submitted
    if (!is_alive) { PushStyleColor(ImGuiCol_Text, GetStyleColorVec4(ImGuiCol_TextDisabled)); }
    bool open;
    ImGuiTreeNodeFlags tree_node_flags = node->IsFocused ? ImGuiTreeNodeFlags_Selected : ImGuiTreeNodeFlags_None;
    if (node->Windows.Size > 0)
        open = TreeNodeEx((*mut c_void)(intptr_t)node->ID, tree_node_flags, "%s 0x%04X%s: %d windows (vis: '%s')", label, node->ID, node->IsVisible ? "" : " (hidden)", node->Windows.Size, node->VisibleWindow ? node->Visiblewindow.Name : "NULL");
    else
        open = TreeNodeEx((*mut c_void)(intptr_t)node->ID, tree_node_flags, "%s 0x%04X%s: %s split (vis: '%s')", label, node->ID, node->IsVisible ? "" : " (hidden)", (node->SplitAxis == ImGuiAxis_X) ? "horizontal" : (node->SplitAxis == ImGuiAxis_Y) ? "vertical" : "n/a", node->VisibleWindow ? node->Visiblewindow.Name : "NULL");
    if (!is_alive) { PopStyleColor(); }
    if (is_active && IsItemHovered())
        if (ImGuiWindow* window = node->HostWindow ? node->HostWindow : node->VisibleWindow)
            GetForegroundDrawList(window)->AddRect(node->Pos, node->Pos + node->Size, IM_COL32(255, 255, 0, 255));
    if (open)
    {
        // IM_ASSERT(node->ChildNodes[0] == NULL || node->ChildNodes[0]->ParentNode == node);
        // IM_ASSERT(node->ChildNodes[1] == NULL || node->ChildNodes[1]->ParentNode == node);
        BulletText("Pos (%.0f32,%.00f32), Size (%.0f32, %.00f32) Ref (%.0f32, %.00f32)",
            node->Pos.x, node->Pos.y, node->Size.x, node->Size.y, node->SizeRef.x, node->SizeRef.y);
        DebugNodeWindow(node->HostWindow, "HostWindow");
        DebugNodeWindow(node->VisibleWindow, "VisibleWindow");
        BulletText("SelectedTabID: 0x%08X, LastFocusedNodeID: 0x%08X", node->SelectedTabId, node->LastFocusedNodeId);
        BulletText("Misc:%s%s%s%s%s%s%s",
            node->IsDockSpace() ? " IsDockSpace" : "",
            node->IsCentralNode() ? " IsCentralNode" : "",
            is_alive ? " IsAlive" : "", is_active ? " IsActive" : "", node->IsFocused ? " IsFocused" : "",
            node->WantLockSizeOnce ? " WantLockSizeOnce" : "",
            node->HasCentralNodeChild ? " HasCentralNodeChild" : "");
        if (TreeNode("flags", "Flags Merged: 0x%04X, Local: 0x%04X, InWindows: 0x%04X, Shared: 0x%04X", node->MergedFlags, node->LocalFlags, node->LocalFlagsInWindows, node->SharedFlags))
        {
            if (BeginTable("flags", 4))
            {
                TableNextColumn(); DebugNodeDockNodeFlags(&node->MergedFlags, "MergedFlags", false);
                TableNextColumn(); DebugNodeDockNodeFlags(&node->LocalFlags, "LocalFlags", true);
                TableNextColumn(); DebugNodeDockNodeFlags(&node->LocalFlagsInWindows, "LocalFlagsInWindows", false);
                TableNextColumn(); DebugNodeDockNodeFlags(&node->SharedFlags, "SharedFlags", true);
                EndTable();
            }
            TreePop();
        }
        if (node->ParentNode)
            DebugNodeDockNode(node->ParentNode, "ParentNode");
        if (node->ChildNodes[0])
            DebugNodeDockNode(node->ChildNodes[0], "Child[0]");
        if (node->ChildNodes[1])
            DebugNodeDockNode(node->ChildNodes[1], "Child[1]");
        if (node->TabBar)
            DebugNodeTabBar(node->TabBar, "TabBar");
        DebugNodeWindowsList(&node->Windows, "Windows");

        TreePop();
    }
}

// [DEBUG] Display contents of ImDrawList
// Note that both 'window' and 'viewport' may be NULL here. Viewport is generally null of destroyed popups which previously owned a viewport.
c_void ImGui::DebugNodeDrawList(ImGuiWindow* window, *mut ImGuiViewportP viewport, *const ImDrawList draw_list, *const char label)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiMetricsConfig* cfg = &g.DebugMetricsConfig;
    c_int cmd_count = draw_list->CmdBuffer.Size;
    if (cmd_count > 0 && draw_list->CmdBuffer.back().ElemCount == 0 && draw_list->CmdBuffer.back().UserCallback == NULL)
        cmd_count-= 1;
    let mut node_open: bool =  TreeNode(draw_list, "%s: '%s' %d vtx, %d indices, %d cmds", label, draw_list->_OwnerName ? draw_list->_OwnerName : "", draw_list->VtxBuffer.Size, draw_list->IdxBuffer.Size, cmd_count);
    if (draw_list == GetWindowDrawList())
    {
        SameLine();
        TextColored(ImVec4(1f32, 0.4f, 0.4f, 1f32), "CURRENTLY APPENDING"); // Can't display stats for active draw list! (we don't have the data double-buffered)
        if (node_open)
            TreePop();
        return;
    }

    ImDrawList* fg_draw_list = viewport ? GetForegroundDrawList(viewport) : NULL; // Render additional visuals into the top-most draw list
    if (window && IsItemHovered() && fg_draw_list)
        fg_draw_list->AddRect(window.Pos, window.Pos + window.Size, IM_COL32(255, 255, 0, 255));
    if (!node_open)
        return;

    if (window && !window.WasActive)
        TextDisabled("Warning: owning Window is inactive. This DrawList is not being rendered!");

    for (*const ImDrawCmd pcmd = draw_list->CmdBuffer.Data; pcmd < draw_list->CmdBuffer.Data + cmd_count; pcmd++)
    {
        if (pcmd->UserCallback)
        {
            BulletText("Callback %p, user_data %p", pcmd->UserCallback, pcmd->UserCallbackData);
            continue;
        }

        buf: [c_char;300];
        ImFormatString(buf, IM_ARRAYSIZE(bu0f32), "DrawCmd:%5d tris, Tex 0x%p, ClipRect (%4.0f32,%4.00f32)-(%4.0f32,%4.00f32)",
            pcmd->ElemCount / 3, (*mut c_void)(intptr_t)pcmd->TextureId,
            pcmd->ClipRect.x, pcmd->ClipRect.y, pcmd->ClipRect.z, pcmd->ClipRect.w);
        let mut pcmd_node_open: bool =  TreeNode((*mut c_void)(pcmd - draw_list->CmdBuffer.begin()), "%s", bu0f32);
        if (IsItemHovered() && (cfg->ShowDrawCmdMesh || cfg->ShowDrawCmdBoundingBoxes) && fg_draw_list)
            DebugNodeDrawCmdShowMeshAndBoundingBox(fg_draw_list, draw_list, pcmd, cfg->ShowDrawCmdMesh, cfg->ShowDrawCmdBoundingBoxes);
        if (!pcmd_node_open)
            continue;

        // Calculate approximate coverage area (touched pixel count)
        // This will be in pixels squared as long there's no post-scaling happening to the renderer output.
        *const ImDrawIdx idx_buffer = (draw_list->IdxBuffer.Size > 0) ? draw_list->IdxBuffer.Data : NULL;
        *const ImDrawVert vtx_buffer = draw_list->VtxBuffer.Data + pcmd->VtxOffset;
        c_float total_area = 0f32;
        for (c_uint idx_n = pcmd->IdxOffset; idx_n < pcmd->IdxOffset + pcmd->ElemCount; )
        {
            ImVec2 triangle[3];
            for (c_int n = 0; n < 3; n++, idx_n++)
                triangle[n] = vtx_buffer[idx_buffer ? idx_buffer[idx_n] : idx_n].pos;
            total_area += ImTriangleArea(triangle[0], triangle[1], triangle[2]);
        }

        // Display vertex information summary. Hover to get all triangles drawn in wire-frame
        ImFormatString(buf, IM_ARRAYSIZE(bu0f32), "Mesh: ElemCount: %d, VtxOffset: +%d, IdxOffset: +%d, Area: ~%0.f px", pcmd->ElemCount, pcmd->VtxOffset, pcmd->IdxOffset, total_area);
        Selectable(bu0f32);
        if (IsItemHovered() && fg_draw_list)
            DebugNodeDrawCmdShowMeshAndBoundingBox(fg_draw_list, draw_list, pcmd, true, false);

        // Display individual triangles/vertices. Hover on to get the corresponding triangle highlighted.
        ImGuiListClipper clipper;
        clipper.Begin(pcmd->ElemCount / 3); // Manually coarse clip our print out of individual vertices to save CPU, only items that may be visible.
        while (clipper.Step())
            for (c_int prim = clipper.DisplayStart, idx_i = pcmd->IdxOffset + clipper.DisplayStart * 3; prim < clipper.DisplayEnd; prim++)
            {
                char* buf_p = buf, * buf_end = buf + IM_ARRAYSIZE(bu0f32);
                ImVec2 triangle[3];
                for (c_int n = 0; n < 3; n++, idx_i++)
                {
                    const ImDrawVert& v = vtx_buffer[idx_buffer ? idx_buffer[idx_i] : idx_i];
                    triangle[n] = v.pos;
                    buf_p += ImFormatString(buf_p, buf_end - buf_p, "%s %04d: pos (%8.2f,%8.20f32), uv (%.6f,%.60f32), col %08X\n",
                        (n == 0) ? "Vert:" : "     ", idx_i, v.pos.x, v.pos.y, v.uv.x, v.uv.y, v.col);
                }

                Selectable(buf, false);
                if (fg_draw_list && IsItemHovered())
                {
                    ImDrawListFlags backup_flags = fg_draw_list->Flags;
                    fg_draw_list->Flags &= ~ImDrawListFlags_AntiAliasedLines; // Disable AA on triangle outlines is more readable for very large and thin triangles.
                    fg_draw_list->AddPolyline(triangle, 3, IM_COL32(255, 255, 0, 255), ImDrawFlags_Closed, 1f32);
                    fg_draw_list->Flags = backup_flags;
                }
            }
        TreePop();
    }
    TreePop();
}

// [DEBUG] Display mesh/aabb of a ImDrawCmd
c_void ImGui::DebugNodeDrawCmdShowMeshAndBoundingBox(ImDrawList* out_draw_list, *const ImDrawList draw_list, *const ImDrawCmd draw_cmd, bool show_mesh, bool show_aabb)
{
    // IM_ASSERT(show_mesh || show_aabb);

    // Draw wire-frame version of all triangles
    ImRect clip_rect = draw_cmd->ClipRect;
    ImRect vtxs_rect(f32::MAX, f32::MAX, -f32::MAX, -f32::MAX);
    ImDrawListFlags backup_flags = out_draw_list->Flags;
    out_draw_list->Flags &= ~ImDrawListFlags_AntiAliasedLines; // Disable AA on triangle outlines is more readable for very large and thin triangles.
    for (c_uint idx_n = draw_cmd->IdxOffset, idx_end = draw_cmd->IdxOffset + draw_cmd->ElemCount; idx_n < idx_end; )
    {
        ImDrawIdx* idx_buffer = (draw_list->IdxBuffer.Size > 0) ? draw_list->IdxBuffer.Data : NULL; // We don't hold on those pointers past iterations as ->AddPolyline() may invalidate them if out_draw_list==draw_list
        ImDrawVert* vtx_buffer = draw_list->VtxBuffer.Data + draw_cmd->VtxOffset;

        ImVec2 triangle[3];
        for (c_int n = 0; n < 3; n++, idx_n++)
            vtxs_rect.Add((triangle[n] = vtx_buffer[idx_buffer ? idx_buffer[idx_n] : idx_n].pos));
        if (show_mesh)
            out_draw_list->AddPolyline(triangle, 3, IM_COL32(255, 255, 0, 255), ImDrawFlags_Closed, 1f32); // In yellow: mesh triangles
    }
    // Draw bounding boxes
    if (show_aabb)
    {
        out_draw_list->AddRect(ImFloor(clip_rect.Min), ImFloor(clip_rect.Max), IM_COL32(255, 0, 255, 255)); // In pink: clipping rectangle submitted to GPU
        out_draw_list->AddRect(ImFloor(vtxs_rect.Min), ImFloor(vtxs_rect.Max), IM_COL32(0, 255, 255, 255)); // In cyan: bounding box of triangles
    }
    out_draw_list->Flags = backup_flags;
}

// [DEBUG] Display details for a single font, called by ShowStyleEditor().
c_void ImGui::DebugNodeFont(ImFont* font)
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
    DragFloat("Font scale", &font->Scale, 0.005f, 0.3f, 2.0f32, "%.1f");
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
    for (c_int config_i = 0; config_i < font->ConfigDataCount; config_i++)
        if (font->ConfigData)
            if (*const ImFontConfig cfg = &font->ConfigData[config_i])
                BulletText("Input %d: \'%s\', Oversample: (%d,%d), PixelSnapH: %d, Offset: (%.1f,%.10f32)",
                    config_i, cfg->Name, cfg->OversampleH, cfg->OversampleV, cfg->PixelSnapH, cfg->GlyphOffset.x, cfg->GlyphOffset.y);

    // Display all glyphs of the fonts in separate pages of 256 characters
    if (TreeNode("Glyphs", "Glyphs (%d)", font->Glyphs.Size))
    {
        ImDrawList* draw_list = GetWindowDrawList();
        const u32 glyph_col = GetColorU32(ImGuiCol_Text);
        const c_float cell_size = font->FontSize * 1;
        const c_float cell_spacing = GetStyle().ItemSpacing.y;
        for (c_uint base = 0; base <= IM_UNICODE_CODEPOINT_MAX; base += 256)
        {
            // Skip ahead if a large bunch of glyphs are not present in the font (test in chunks of 4k)
            // This is only a small optimization to reduce the number of iterations when IM_UNICODE_MAX_CODEPOINT
            // is large // (if ImWchar==ImWchar32 we will do at least about 272 queries here)
            if (!(base & 4095) && font->IsGlyphRangeUnused(base, base + 4095))
            {
                base += 4096 - 256;
                continue;
            }

            c_int count = 0;
            for (c_uint n = 0; n < 256; n++)
                if (font->FindGlyphNoFallback((base + n)))
                    count+= 1;
            if (count <= 0)
                continue;
            if (!TreeNode((*mut c_void)(intptr_t)base, "U+%04X..U+%04X (%d %s)", base, base + 255, count, count > 1 ? "glyphs" : "glyph"))
                continue;

            // Draw a 16x16 grid of glyphs
            ImVec2 base_pos = GetCursorScreenPos();
            for (c_uint n = 0; n < 256; n++)
            {
                // We use ImFont::RenderChar as a shortcut because we don't have UTF-8 conversion functions
                // available here and thus cannot easily generate a zero-terminated UTF-8 encoded string.
                ImVec2 cell_p1(base_pos.x + (n % 16) * (cell_size + cell_spacing), base_pos.y + (n / 16) * (cell_size + cell_spacing));
                ImVec2 cell_p2(cell_p1.x + cell_size, cell_p1.y + cell_size);
                *const ImFontGlyph glyph = font->FindGlyphNoFallback((base + n));
                draw_list->AddRect(cell_p1, cell_p2, glyph ? IM_COL32(255, 255, 255, 100) : IM_COL32(255, 255, 255, 50));
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
            Dummy(ImVec2((cell_size + cell_spacing) * 16, (cell_size + cell_spacing) * 16));
            TreePop();
        }
        TreePop();
    }
    TreePop();
}

c_void ImGui::DebugNodeFontGlyph(ImFont*, *const ImFontGlyph glyph)
{
    Text("Codepoint: U+%04X", glyph->Codepoint);
    Separator();
    Text("Visible: %d", glyph->Visible);
    Text("AdvanceX: %.1f", glyph->AdvanceX);
    Text("Pos: (%.2f,%.20f32)->(%.2f,%.20f32)", glyph->X0, glyph->Y0, glyph->X1, glyph->Y1);
    Text("UV: (%.3f,%.30f32)->(%.3f,%.30f32)", glyph->U0, glyph->V0, glyph->U1, glyph->V1);
}

// [DEBUG] Display contents of ImGuiStorage
c_void ImGui::DebugNodeStorage(ImGuiStorage* storage, *const char label)
{
    if (!TreeNode(label, "%s: %d entries, %d bytes", label, storage->Data.Size, storage->Data.size_in_bytes()))
        return;
    for (c_int n = 0; n < storage->Data.Size; n++)
    {
        const ImGuiStorage::ImGuiStoragePair& p = storage->Data[n];
        BulletText("Key 0x%08X Value { i: %d }", p.key, p.val_i); // Important: we currently don't store a type, real value may not be integer.
    }
    TreePop();
}

// [DEBUG] Display contents of ImGuiTabBar
c_void ImGui::DebugNodeTabBar(ImGuiTabBar* tab_bar, *const char label)
{
    // Standalone tab bars (not associated to docking/windows functionality) currently hold no discernible strings.
    buf: [c_char;256];
    char* p = buf;
    let mut  buf_end: *const c_char = buf + IM_ARRAYSIZE(bu0f32);
    let is_active: bool = (tab_bar->PrevFrameVisible >= GetFrameCount() - 2);
    p += ImFormatString(p, buf_end - p, "%s 0x%08X (%d tabs)%s", label, tab_bar->ID, tab_bar->Tabs.Size, is_active ? "" : " *Inactive*");
    p += ImFormatString(p, buf_end - p, "  { ");
    for (c_int tab_n = 0; tab_n < ImMin(tab_bar->Tabs.Size, 3); tab_n++)
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
        ImDrawList* draw_list = GetForegroundDrawList();
        draw_list->AddRect(tab_bar->BarRect.Min, tab_bar->BarRect.Max, IM_COL32(255, 255, 0, 255));
        draw_list->AddLine(ImVec2(tab_bar->ScrollingRectMinX, tab_bar->BarRect.Min.y), ImVec2(tab_bar->ScrollingRectMinX, tab_bar->BarRect.Max.y), IM_COL32(0, 255, 0, 255));
        draw_list->AddLine(ImVec2(tab_bar->ScrollingRectMaxX, tab_bar->BarRect.Min.y), ImVec2(tab_bar->ScrollingRectMaxX, tab_bar->BarRect.Max.y), IM_COL32(0, 255, 0, 255));
    }
    if (open)
    {
        for (c_int tab_n = 0; tab_n < tab_bar->Tabs.Size; tab_n++)
        {
            *const ImGuiTabItem tab = &tab_bar->Tabs[tab_n];
            PushID(tab);
            if (SmallButton("<")) { TabBarQueueReorder(tab_bar, tab, -1); } SameLine(0, 2);
            if (SmallButton(">")) { TabBarQueueReorder(tab_bar, tab, +1); } SameLine();
            Text("%02d%c Tab 0x%08X '%s' Offset: %.2f, Width: %.2f/%.2f",
                tab_n, (tab->ID == tab_bar->SelectedTabId) ? '*' : ' ', tab->ID, (tab->Window || tab->NameOffset != -1) ? tab_bar->GetTabName(tab) : "???", tab->Offset, tab->Width, tab->ContentWidth);
            PopID();
        }
        TreePop();
    }
}

c_void ImGui::DebugNodeViewport(*mut ImGuiViewportP viewport)
{
    SetNextItemOpen(true, ImGuiCond_Once);
    if (TreeNode((*mut c_void)(intptr_t)viewport->ID, "Viewport #%d, ID: 0x%08X, Parent: 0x%08X, Window: \"%s\"", viewport->Idx, viewport->ID, viewport->ParentViewportId, viewport->Window ? viewport->window.Name : "N/A"))
    {
        ImGuiWindowFlags flags = viewport->Flags;
        BulletText("Main Pos: (%.0f32,%.00f32), Size: (%.0f32,%.00f32)\nWorkArea Offset Left: %.0f32 Top: %.0f32, Right: %.0f32, Bottom: %.0f\nMonitor: %d, DpiScale: %.0f%%",
            viewport->Pos.x, viewport->Pos.y, viewport->Size.x, viewport->Size.y,
            viewport->WorkOffsetMin.x, viewport->WorkOffsetMin.y, viewport->WorkOffsetMax.x, viewport->WorkOffsetMax.y,
            viewport->PlatformMonitor, viewport->DpiScale * 100f32);
        if (viewport->Idx > 0) { SameLine(); if (SmallButton("Reset Pos")) { viewport->Pos = ImVec2(200, 200); viewport->UpdateWorkRect(); if (viewport->Window) viewport->window.Pos = viewport->Pos; } }
        BulletText("Flags: 0x%04X =%s%s%s%s%s%s%s%s%s%s%s%s", viewport->Flags,
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
        for (c_int layer_i = 0; layer_i < IM_ARRAYSIZE(viewport->DrawDataBuilder.Layers); layer_i++)
            for (c_int draw_list_i = 0; draw_list_i < viewport->DrawDataBuilder.Layers[layer_i].Size; draw_list_i++)
                DebugNodeDrawList(NULL, viewport, viewport->DrawDataBuilder.Layers[layer_i][draw_list_i], "DrawList");
        TreePop();
    }
}

c_void ImGui::DebugNodeWindow(ImGuiWindow* window, *const char label)
{
    if (window == NULL)
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

    ImGuiWindowFlags flags = window.Flags;
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
    for (c_int layer = 0; layer < ImGuiNavLayer_COUNT; layer++)
    {
        ImRect r = window.NavRectRel[layer];
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

    BulletText("Viewport: %d%s, ViewportId: 0x%08X, ViewportPos: (%.1f,%.10f32)", window.Viewport ? window.Viewport->Idx : -1, window.ViewportOwned ? " (Owned)" : "", window.ViewportId, window.ViewportPos.x, window.ViewportPos.y);
    BulletText("ViewportMonitor: %d", window.Viewport ? window.Viewport->PlatformMonitor : -1);
    BulletText("DockId: 0x%04X, DockOrder: %d, Act: %d, Vis: %d", window.DockId, window.DockOrder, window.DockIsActive, window.DockTabIsVisible);
    if (window.DockNode || window.DockNodeAsHost)
        DebugNodeDockNode(window.DockNodeAsHost ? window.DockNodeAsHost : window.DockNode, window.DockNodeAsHost ? "DockNodeAsHost" : "DockNode");

    if (window.RootWindow != window)       { DebugNodeWindow(window.RootWindow, "RootWindow"); }
    if (window.RootWindowDockTree != window.RootWindow) { DebugNodeWindow(window.RootWindowDockTree, "RootWindowDockTree"); }
    if (window.ParentWindow != NULL)       { DebugNodeWindow(window.ParentWindow, "ParentWindow"); }
    if (window.DC.ChildWindows.Size > 0)   { DebugNodeWindowsList(&window.DC.ChildWindows, "ChildWindows"); }
    if (window.ColumnsStorage.Size > 0 && TreeNode("Columns", "Columns sets (%d)", window.ColumnsStorage.Size))
    {
        for (c_int n = 0; n < window.ColumnsStorage.Size; n++)
            DebugNodeColumns(&window.ColumnsStorage[n]);
        TreePop();
    }
    DebugNodeStorage(&window.StateStorage, "Storage");
    TreePop();
}

c_void ImGui::DebugNodeWindowSettings(ImGuiWindowSettings* settings)
{
    Text("0x%08X \"%s\" Pos (%d,%d) Size (%d,%d) Collapsed=%d",
        settings->ID, settings->GetName(), settings->Pos.x, settings->Pos.y, settings->Size.x, settings->Size.y, settings->Collapsed);
}

c_void ImGui::DebugNodeWindowsList(Vec<ImGuiWindow*>* windows, *const char label)
{
    if (!TreeNode(label, "%s (%d)", label, windows->Size))
        return;
    for (c_int i = windows->Size - 1; i >= 0; i--) // Iterate front to back
    {
        PushID((*windows)[i]);
        DebugNodeWindow((*windows)[i], "Window");
        PopID();
    }
    TreePop();
}

// FIXME-OPT: This is technically suboptimal, but it is simpler this way.
c_void ImGui::DebugNodeWindowsListByBeginStackParent(ImGuiWindow** windows, c_int windows_size, ImGuiWindow* parent_in_begin_stack)
{
    for (c_int i = 0; i < windows_size; i++)
    {
        ImGuiWindow* window = windows[i];
        if (window.ParentWindowInBeginStack != parent_in_begin_stack)
            continue;
        buf: [c_char;20];
        ImFormatString(buf, IM_ARRAYSIZE(bu0f32), "[%04d] Window", window.BeginOrderWithinContext);
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

c_void ImGui::DebugLog(*const char fmt, ...)
{
    va_list args;
    va_start(args, fmt);
    DebugLogV(fmt, args);
    va_end(args);
}

c_void ImGui::DebugLogV(*const char fmt, va_list args)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let old_size: c_int = g.DebugLogBuf.size();
    g.DebugLogBuf.appendf("[%05d] ", g.FrameCount);
    g.DebugLogBuf.appendfv(fmt, args);
    if (g.DebugLogFlags & ImGuiDebugLogFlags_OutputToTTY)
        IMGUI_DEBUG_PRINTF("%s", g.DebugLogBuf.begin() + old_size);
}

c_void ImGui::ShowDebugLogWindow(bool* p_open)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (!(g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasSize))
        SetNextWindowSize(ImVec2(0f32, GetFontSize() * 12.00f32), ImGuiCond_FirstUseEver);
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
    BeginChild("##log", ImVec2(0f32, 0f32), true, ImGuiWindowFlags_AlwaysVerticalScrollbar | ImGuiWindowFlags_AlwaysHorizontalScrollbar);
    TextUnformatted(g.DebugLogBuf.begin(), g.DebugLogBuf.end()); // FIXME-OPT: Could use a line index, but TextUnformatted() has a semi-decent fast path for large text.
    if (GetScrollY() >= GetScrollMaxY())
        SetScrollHereY(1f32);
    EndChild();

    End();
}

//-----------------------------------------------------------------------------
// [SECTION] OTHER DEBUG TOOLS (ITEM PICKER, STACK TOOL)
//-----------------------------------------------------------------------------

// [DEBUG] Item picker tool - start with DebugStartItemPicker() - useful to visually select an item and break into its call-stack.
c_void ImGui::UpdateDebugToolItemPicker()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.DebugItemPickerBreakId = 0;
    if (!g.DebugItemPickerActive)
        return;

    const ImGuiID hovered_id = g.HoveredIdPreviousFrame;
    SetMouseCursor(ImGuiMouseCursor_Hand);
    if (IsKeyPressed(ImGuiKey_Escape))
        g.DebugItemPickerActive = false;
    let change_mapping: bool = g.IO.KeyMods == (ImGuiModFlags_Ctrl | ImGuiModFlags_Shift);
    if (!change_mapping && IsMouseClicked(g.DebugItemPickerMouseButton) && hovered_id)
    {
        g.DebugItemPickerBreakId = hovered_id;
        g.DebugItemPickerActive = false;
    }
    for (c_int mouse_button = 0; mouse_button < 3; mouse_button++)
        if (change_mapping && IsMouseClicked(mouse_button))
            g.DebugItemPickerMouseButton = mouse_button;
    SetNextWindowBgAlpha(0.700f32);
    BeginTooltip();
    Text("HoveredId: 0x%08X", hovered_id);
    Text("Press ESC to abort picking.");
    *const char mouse_button_names[] = { "Left", "Right", "Middle" };
    if (change_mapping)
        Text("Remap w/ Ctrl+Shift: click anywhere to select new mouse button.");
    else
        TextColored(GetStyleColorVec4(hovered_id ? ImGuiCol_Text : ImGuiCol_TextDisabled), "Click %s Button to break in debugger! (remap w/ Ctrl+Shift)", mouse_button_names[g.DebugItemPickerMouseButton]);
    EndTooltip();
}

// [DEBUG] Stack Tool: update queries. Called by NewFrame()
c_void ImGui::UpdateDebugToolStackQueries()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiStackTool* tool = &g.DebugStackTool;

    // Clear hook when stack tool is not visible
    g.DebugHookIdInfo = 0;
    if (g.FrameCount != tool->LastActiveFrame + 1)
        return;

    // Update queries. The steps are: -1: query Stack, >= 0: query each stack item
    // We can only perform 1 ID Info query every frame. This is designed so the GetID() tests are cheap and constant-time
    const ImGuiID query_id = g.HoveredIdPreviousFrame ? g.HoveredIdPreviousFrame : g.ActiveId;
    if (tool->QueryId != query_id)
    {
        tool->QueryId = query_id;
        tool->StackLevel = -1;
        tool->Results.resize(0);
    }
    if (query_id == 0)
        return;

    // Advance to next stack level when we got our result, or after 2 frames (in case we never get a result)
    c_int stack_level = tool->StackLevel;
    if (stack_level >= 0 && stack_level < tool->Results.Size)
        if (tool->Results[stack_level].QuerySuccess || tool->Results[stack_level].QueryFrameCount > 2)
            tool->StackLevel+= 1;

    // Update hook
    stack_level = tool->StackLevel;
    if (stack_level == -1)
        g.DebugHookIdInfo = query_id;
    if (stack_level >= 0 && stack_level < tool->Results.Size)
    {
        g.DebugHookIdInfo = tool->Results[stack_level].ID;
        tool->Results[stack_level].QueryFrameCount+= 1;
    }
}

// [DEBUG] Stack tool: hooks called by GetID() family functions
c_void ImGui::DebugHookIdInfo(ImGuiID id, ImGuiDataType data_type, *const c_void data_id, *const c_void data_id_end)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    ImGuiStackTool* tool = &g.DebugStackTool;

    // Step 0: stack query
    // This assume that the ID was computed with the current ID stack, which tends to be the case for our widget.
    if (tool->StackLevel == -1)
    {
        tool->StackLevel+= 1;
        tool->Results.resize(window.IDStack.Size + 1, ImGuiStackLevelInfo());
        for (c_int n = 0; n < window.IDStack.Size + 1; n++)
            tool->Results[n].ID = (n < window.IDStack.Size) ? window.IDStack[n] : id;
        return;
    }

    // Step 1+: query for individual level
    // IM_ASSERT(tool->StackLevel >= 0);
    if (tool->StackLevel != window.IDStack.Size)
        return;
    ImGuiStackLevelInfo* info = &tool->Results[tool->StackLevel];
    // IM_ASSERT(info.ID == id && info.QueryFrameCount > 0);

    switch (data_type)
    {
    case ImGuiDataType_S32:
        ImFormatString(info.Desc, IM_ARRAYSIZE(info.Desc), "%d", (intptr_t)data_id);
        break;
    case ImGuiDataType_String:
        ImFormatString(info.Desc, IM_ARRAYSIZE(info.Desc), "%.*s", data_id_end ? ((*const char)data_id_end - (*const char)data_id) : strlen((*const char)data_id), (*const char)data_id);
        break;
    case ImGuiDataType_Pointer:
        ImFormatString(info.Desc, IM_ARRAYSIZE(info.Desc), "(void*)0x%p", data_id);
        break;
    case ImGuiDataType_ID:
        if (info.Desc[0] != 0) // PushOverrideID() is often used to avoid hashing twice, which would lead to 2 calls to DebugHookIdInfo(). We prioritize the first one.
            return;
        ImFormatString(info.Desc, IM_ARRAYSIZE(info.Desc), "0x%08X [override]", id);
        break;
    default:
        // IM_ASSERT(0);
    }
    info.QuerySuccess = true;
    info.DataType = data_type;
}

static c_int StackToolFormatLevelInfo(ImGuiStackTool* tool, c_int n, bool format_for_ui, char* buf, size_t buf_size)
{
    ImGuiStackLevelInfo* info = &tool->Results[n];
    ImGuiWindow* window = (info.Desc[0] == 0 && n == 0) ? ImGui::FindWindowByID(info.ID) : NULL;
    if (window)                                                                 // Source: window name (because the root ID don't call GetID() and so doesn't get hooked)
        return ImFormatString(buf, buf_size, format_for_ui ? "\"%s\" [window]" : "%s", window.Name);
    if (info.QuerySuccess)                                                     // Source: GetID() hooks (prioritize over ItemInfo() because we frequently use patterns like: PushID(str), Button("") where they both have same id)
        return ImFormatString(buf, buf_size, (format_for_ui && info.DataType == ImGuiDataType_String) ? "\"%s\"" : "%s", info.Desc);
    if (tool->StackLevel < tool->Results.Size)                                  // Only start using fallback below when all queries are done, so during queries we don't flickering ??? markers.
        return (*buf = 0);
// #ifdef IMGUI_ENABLE_TEST_ENGINE
    if (*const char label = ImGuiTestEngine_FindItemDebugLabel(GImGui, info.ID))   // Source: ImGuiTestEngine's ItemInfo()
        return ImFormatString(buf, buf_size, format_for_ui ? "??? \"%s\"" : "%s", label);
// #endif
    return ImFormatString(buf, buf_size, "???");
}

// Stack Tool: Display UI
c_void ImGui::ShowStackToolWindow(bool* p_open)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (!(g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasSize))
        SetNextWindowSize(ImVec2(0f32, GetFontSize() * 8.00f32), ImGuiCond_FirstUseEver);
    if (!Begin("Dear ImGui Stack Tool", p_open) || GetCurrentWindow()->BeginCount > 1)
    {
        End();
        return;
    }

    // Display hovered/active status
    ImGuiStackTool* tool = &g.DebugStackTool;
    const ImGuiID hovered_id = g.HoveredIdPreviousFrame;
    const ImGuiID active_id = g.ActiveId;
// #ifdef IMGUI_ENABLE_TEST_ENGINE
    Text("HoveredId: 0x%08X (\"%s\"), ActiveId:  0x%08X (\"%s\")", hovered_id, hovered_id ? ImGuiTestEngine_FindItemDebugLabel(&g, hovered_id) : "", active_id, active_id ? ImGuiTestEngine_FindItemDebugLabel(&g, active_id) : "");
// #else
    Text("HoveredId: 0x%08X, ActiveId:  0x%08X", hovered_id, active_id);
// #endif
    SameLine();
    MetricsHelpMarker("Hover an item with the mouse to display elements of the ID Stack leading to the item's final ID.\nEach level of the stack correspond to a PushID() call.\nAll levels of the stack are hashed together to make the final ID of a widget (ID displayed at the bottom level of the stack).\nRead FAQ entry about the ID stack for details.");

    // CTRL+C to copy path
    const c_float time_since_copy = g.Time - tool->CopyToClipboardLastTime;
    Checkbox("Ctrl+C: copy path to clipboard", &tool->CopyToClipboardOnCtrlC);
    SameLine();
    TextColored((time_since_copy >= 0f32 && time_since_copy < 0.75f && ImFmod(time_since_copy, 0.250f32) < 0.25f * 0.5f32) ? ImVec4(1.f, 1.f, 0.3f, 1f32) : ImVec4(), "*COPIED*");
    if (tool->CopyToClipboardOnCtrlC && IsKeyDown(ImGuiKey_ModCtrl) && IsKeyPressed(ImGuiKey_C))
    {
        tool->CopyToClipboardLastTime = g.Time;
        char* p = g.TempBuffer.Data;
        char* p_end = p + g.TempBuffer.Size;
        for (c_int stack_n = 0; stack_n < tool->Results.Size && p + 3 < p_end; stack_n++)
        {
            *p++ = '/';
            level_desc: [c_char;256];
            StackToolFormatLevelInfo(tool, stack_n, false, level_desc, IM_ARRAYSIZE(level_desc));
            for (c_int n = 0; level_desc[n] && p + 2 < p_end; n++)
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
    tool->LastActiveFrame = g.FrameCount;
    if (tool->Results.Size > 0 && BeginTable("##table", 3, ImGuiTableFlags_Borders))
    {
        const c_float id_width = CalcTextSize("0xDDDDDDDD").x;
        TableSetupColumn("Seed", ImGuiTableColumnFlags_WidthFixed, id_width);
        TableSetupColumn("PushID", ImGuiTableColumnFlags_WidthStretch);
        TableSetupColumn("Result", ImGuiTableColumnFlags_WidthFixed, id_width);
        TableHeadersRow();
        for (c_int n = 0; n < tool->Results.Size; n++)
        {
            ImGuiStackLevelInfo* info = &tool->Results[n];
            TableNextColumn();
            Text("0x%08X", (n > 0) ? tool->Results[n - 1].ID : 0);
            TableNextColumn();
            StackToolFormatLevelInfo(tool, n, true, g.TempBuffer.Data, g.TempBuffer.Size);
            TextUnformatted(g.TempBuffer.Data);
            TableNextColumn();
            Text("0x%08X", info.ID);
            if (n == tool->Results.Size - 1)
                TableSetBgColor(ImGuiTableBgTarget_CellBg, GetColorU32(ImGuiCol_Header));
        }
        EndTable();
    }
    End();
}

// #else

c_void ImGui::ShowMetricsWindow(bool*) {}
c_void ImGui::ShowFontAtlas(ImFontAtlas*) {}
c_void ImGui::DebugNodeColumns(ImGuiOldColumns*) {}
c_void ImGui::DebugNodeDrawList(ImGuiWindow*, *mut ImGuiViewportP, *const ImDrawList, *const char) {}
c_void ImGui::DebugNodeDrawCmdShowMeshAndBoundingBox(ImDrawList*, *const ImDrawList, *const ImDrawCmd, bool, bool) {}
c_void ImGui::DebugNodeFont(ImFont*) {}
c_void ImGui::DebugNodeStorage(ImGuiStorage*, *const char) {}
c_void ImGui::DebugNodeTabBar(ImGuiTabBar*, *const char) {}
c_void ImGui::DebugNodeWindow(ImGuiWindow*, *const char) {}
c_void ImGui::DebugNodeWindowSettings(ImGuiWindowSettings*) {}
c_void ImGui::DebugNodeWindowsList(Vec<ImGuiWindow*>*, *const char) {}
c_void ImGui::DebugNodeViewport(*mut ImGuiViewportP) {}

c_void ImGui::DebugLog(*const char, ...) {}
c_void ImGui::DebugLogV(*const char, va_list) {}
c_void ImGui::ShowDebugLogWindow(bool*) {}
c_void ImGui::ShowStackToolWindow(bool*) {}
c_void ImGui::DebugHookIdInfo(ImGuiID, ImGuiDataType, *const c_void, *const c_void) {}
c_void ImGui::UpdateDebugToolItemPicker() {}
c_void ImGui::UpdateDebugToolStackQueries() {}

// #endif // #ifndef IMGUI_DISABLE_DEBUG_TOOLS

//-----------------------------------------------------------------------------

// Include imgui_user.inl at the end of imgui.cpp to access private data/functions that aren't exposed.
// Prefer just including imgui_internal.h from your code rather than using this define. If a declaration is missing from imgui_internal.h add it or request it on the github.
// #ifdef IMGUI_INCLUDE_IMGUI_USER_INL
// #include "imgui_user.inl"
// #endif

//-----------------------------------------------------------------------------

// #endif // #ifndef IMGUI_DISABLE
