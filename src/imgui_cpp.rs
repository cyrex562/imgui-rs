// dear imgui, v1.89 WIP
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use std::env::args;
use std::io::stdout;
use libc;
use windows_sys::Win32;
use std::ptr::{null, null_mut};
use std::slice::Windows;
use libc::{c_char, c_float, c_int, c_uchar, c_uint, c_void, memcmp, memcpy, memset, open, size_t, strlen, uintptr_t};
use crate::axis::{ImGuiAxis, ImGuiAxis_X, ImGuiAxis_Y};
use crate::child_ops::{BeginChild, EndChild};
use crate::clipboard_ops::SetClipboardText;
use crate::color::{IM_COL32, ImGuiCol_Border, ImGuiCol_Header, ImGuiCol_Separator, ImGuiCol_Text, ImGuiCol_TextDisabled, ImGuiCol_TitleBgActive, ImGuiCol_WindowBg};
use crate::color_ops::ColorConvertFloat4ToU32;
use crate::condition::{ImGuiCond, ImGuiCond_Always, ImGuiCond_Appearing, ImGuiCond_FirstUseEver, ImGuiCond_Once};
use crate::context::ImGuiContext;
use crate::context_ops::{GetFrameCount, GetPlatformIO};
use crate::cursor_ops::{GetCursorScreenPos, Indent, Unindent};
use crate::data_type::ImGuiDataType;
use crate::debug_log_flags::{ImGuiDebugLogFlags_EventActiveId, ImGuiDebugLogFlags_EventClipper, ImGuiDebugLogFlags_EventDocking, ImGuiDebugLogFlags_EventFocus, ImGuiDebugLogFlags_EventIO, ImGuiDebugLogFlags_EventMask_, ImGuiDebugLogFlags_EventNav, ImGuiDebugLogFlags_EventPopup, ImGuiDebugLogFlags_EventViewport, ImGuiDebugLogFlags_OutputToTTY};
use crate::dock_node::ImGuiDockNode;
use crate::drag_drop_flags::{ImGuiDragDropFlags, ImGuiDragDropFlags_AcceptBeforeDelivery, ImGuiDragDropFlags_AcceptNoDrawDefaultRect, ImGuiDragDropFlags_AcceptNoPreviewTooltip, ImGuiDragDropFlags_None, ImGuiDragDropFlags_SourceAllowNullID, ImGuiDragDropFlags_SourceAutoExpirePayload, ImGuiDragDropFlags_SourceExtern, ImGuiDragDropFlags_SourceNoDisableHover, ImGuiDragDropFlags_SourceNoHoldToOpenOthers, ImGuiDragDropFlags_SourceNoPreviewTooltip};
use crate::draw_list::ImDrawList;
use crate::draw_list_ops::GetForegroundDrawList;
use crate::font::ImFont;
use crate::font_atlas::ImFontAtlas;
use crate::font_glyph::ImFontGlyph;
use crate::font_ops::{PopFont, PushFont};
use crate::frame_ops::GetFrameHeight;
use crate::{GImGui, ImFileClose, ImGuiSettingsHandler, ImGuiViewport, ImHashStr};
use crate::activate_flags::ImGuiActivateFlags_None;
use crate::config_flags::{ImGuiConfigFlags_DockingEnable, ImGuiConfigFlags_NavEnableGamepad, ImGuiConfigFlags_NavEnableKeyboard, ImGuiConfigFlags_ViewportsEnable};
use crate::constants::{DOCKING_SPLITTER_SIZE, NAV_WINDOWING_LIST_APPEAR_DELAY, WINDOWS_HOVER_PADDING, WINDOWS_RESIZE_FROM_EDGES_FEEDBACK_TIMER};
use crate::data_authority::ImGuiDataAuthority;
use crate::direction::{ImGuiDir, ImGuiDir_Down, ImGuiDir_Left, ImGuiDir_None, ImGuiDir_Right, ImGuiDir_Up};
use crate::dock_context::ImGuiDockContext;
use crate::dock_request::ImGuiDockRequest;
use crate::dock_request_type::{ImGuiDockRequestType_Dock, ImGuiDockRequestType_None, ImGuiDockRequestType_Undock};
use crate::draw_flags::ImDrawFlags;
use crate::file_ops::{ImFileLoadToMemory, ImFileOpen, ImFileWrite};
use {ClearActiveID, GetID, KeepAliveID, PopID, PushID, PushOverrideID, SetActiveID};
use crate::input_flags::{ImGuiInputFlags_Repeat, ImGuiInputFlags_RepeatRateNavMove};
use crate::input_ops::{GetKeyIndex, IsKeyDown, IsKeyPressed, IsKeyPressedEx, IsMouseClicked, IsMouseDragging, IsMouseHoveringRect, PopAllowKeyboardFocus, PushAllowKeyboardFocus, SetMouseCursor};
use crate::input_source::{ImGuiInputSource_Gamepad, ImGuiInputSource_Nav};
use crate::io::ImGuiIO;
use crate::io_ops::GetIO;
use crate::item_flags::ImGuiItemFlags_Inputable;
use crate::item_ops::{IsItemHovered, ItemHoverable, SetNextItemWidth};
use crate::item_status_flags::{ImGuiItemStatusFlags_HasDisplayRect, ImGuiItemStatusFlags_HoveredRect};
use crate::key::{ImGuiKey_C, ImGuiKey_DownArrow, ImGuiKey_Escape, ImGuiKey_GamepadDpadDown, ImGuiKey_GamepadDpadLeft, ImGuiKey_GamepadDpadRight, ImGuiKey_GamepadDpadUp, ImGuiKey_LeftArrow, ImGuiKey_ModCtrl, ImGuiKey_RightArrow, ImGuiKey_Tab, ImGuiKey_UpArrow};
use crate::layout_ops::SameLine;
use crate::log_type::{ImGuiLogType, ImGuiLogType_Buffer, ImGuiLogType_Clipboard, ImGuiLogType_File, ImGuiLogType_None, ImGuiLogType_TTY};
use crate::math_ops::{ImFmod, ImMax, ImMin, ImSqrt};
use crate::metrics_config::ImGuiMetricsConfig;
use crate::mod_flags::{ImGuiModFlags_Ctrl, ImGuiModFlags_Shift};
use crate::mouse_button::{ImGuiMouseButton, ImGuiMouseButton_Left};
use crate::mouse_cursor::ImGuiMouseCursor_Hand;
use crate::nav_item_data::ImGuiNavItemData;
use crate::nav_layer::ImGuiNavLayer_Main;
use crate::nav_move_flags::{ImGuiNavMoveFlags, ImGuiNavMoveFlags_Activate, ImGuiNavMoveFlags_AllowCurrentNavId, ImGuiNavMoveFlags_AlsoScoreVisibleSet, ImGuiNavMoveFlags_DebugNoResult, ImGuiNavMoveFlags_DontSetNavHighlight, ImGuiNavMoveFlags_Forwarded, ImGuiNavMoveFlags_LoopX, ImGuiNavMoveFlags_LoopY, ImGuiNavMoveFlags_None, ImGuiNavMoveFlags_ScrollToEdgeY, ImGuiNavMoveFlags_Tabbing, ImGuiNavMoveFlags_WrapX, ImGuiNavMoveFlags_WrapY};
use crate::next_window_data_flags::{ImGuiNextWindowDataFlags_HasPos, ImGuiNextWindowDataFlags_HasSize};
use crate::old_columns::ImGuiOldColumns;
use crate::payload::ImGuiPayload;
use crate::platform_io::ImGuiPlatformIO;
use crate::rect::ImRect;
use crate::render_ops::{CalcRoundingFlagsForRectInRect, FindRenderedTextEnd};
use crate::scroll_flags::{ImGuiScrollFlags, ImGuiScrollFlags_AlwaysCenterY, ImGuiScrollFlags_KeepVisibleEdgeX, ImGuiScrollFlags_KeepVisibleEdgeY, ImGuiScrollFlags_None};
use crate::scrolling_ops::{GetScrollMaxY, GetScrollY, ScrollToRectEx, SetScrollHereY, SetScrollY};
use crate::stack_level_info::ImGuiStackLevelInfo;
use crate::stack_tool::ImGuiStackTool;
use crate::storage::{ImGuiStorage, ImGuiStoragePair};
use crate::string_ops::{ImFormatString, ImStrchrRange, ImTextCharFromUtf8, ImTextCharToUtf8};
use crate::style_ops::{GetColorU32, GetStyle, GetStyleColorVec4, PopStyleColor, PushStyleColor};
use crate::tab_bar::ImGuiTabBar;
use crate::tab_item::ImGuiTabItem;
use crate::table_flags::{ImGuiTableFlags_Borders, ImGuiTableFlags_RowBg, ImGuiTableFlags_SizingFixedFit};
use crate::text_ops::CalcTextSize;
use crate::tooltip_ops::{BeginTooltip, EndTooltip};
use crate::tree_node_flags::ImGuiTreeNodeFlags;
use crate::type_defs::{ImFileHandle, ImGuiID};
use crate::utils::{flag_clear, GetVersion};
use crate::vec2::{ImVec2, ImVec2ih};
use crate::vec4::ImVec4;
use crate::window::find::{FindWindowByID, FindWindowByName, GetWindowForTitleDisplay};
use crate::window::focus::FocusWindow;
use crate::window::ImGuiWindow;
use crate::window::ops::{Begin, IsWindowActiveAndVisible, ScaleWindow, SetNextWindowSize, TranslateWindow};
use crate::window::props::{GetFont, GetFontSize, GetWindowDrawList, IsWindowNavFocusable, SetNextWindowBgAlpha, SetNextWindowPos, SetNextWindowSizeConstraints};
use crate::window::rect::{WindowRectAbsToRel, WindowRectRelToAbs};
use crate::window::render::UpdateWindowParentAndRootLinks;
use crate::window::window_dock_style_colors::GWindowDockStyleColors;
use crate::window::window_flags::{ImGuiWindowFlags, ImGuiWindowFlags_AlwaysAutoResize, ImGuiWindowFlags_AlwaysHorizontalScrollbar, ImGuiWindowFlags_AlwaysUseWindowPadding, ImGuiWindowFlags_AlwaysVerticalScrollbar, ImGuiWindowFlags_ChildMenu, ImGuiWindowFlags_ChildWindow, ImGuiWindowFlags_Modal, ImGuiWindowFlags_NoBackground, ImGuiWindowFlags_NoDocking, ImGuiWindowFlags_NoFocusOnAppearing, ImGuiWindowFlags_NoInputs, ImGuiWindowFlags_NoMouseInputs, ImGuiWindowFlags_NoMove, ImGuiWindowFlags_NoNavInputs, ImGuiWindowFlags_NoResize, ImGuiWindowFlags_NoSavedSettings, ImGuiWindowFlags_NoTitleBar, ImGuiWindowFlags_Popup, ImGuiWindowFlags_Tooltip};
use crate::window::window_settings::ImGuiWindowSettings;


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

//-----------------------------------------------------------------------------
// Docking: Public Functions (SetWindowDock, DockSpace, DockSpaceOverViewport)
//-----------------------------------------------------------------------------
// - SetWindowDock() [Internal]
// - DockSpace()
// - DockSpaceOverViewport()
//-----------------------------------------------------------------------------

// [Internal] Called via SetNextWindowDockID()
pub unsafe fn SetWindowDock(window: *mut ImGuiWindow, dock_id: ImGuiID, cond: ImGuiCond)
{
    // Test condition (NB: bit 0 is always true) and clear flags for next time
    if (cond && (window.SetWindowDockAllowFlags & cond) == 0)
        return;
    window.SetWindowDockAllowFlags &= ~(ImGuiCond_Once | ImGuiCond_FirstUseEver | ImGuiCond_Appearing);

    if (window.DockId == dock_id)
        return;

    // If the user attempt to set a dock id that is a split node, we'll dig within to find a suitable docking spot
    ctx: *mut ImGuiContext = GImGui;
    if (new_node:*mut ImGuiDockNode = DockContextFindNodeByID(ctx, dock_id))
        if (new_node.IsSplitNode())
        {
            // Policy: Find central node or latest focused node. We first move back to our root node.
            new_node = DockNodeGetRootNode(new_node);
            if (new_node.CentralNode)
            {
                // IM_ASSERT(new_node.CentralNode.IsCentralNode());
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
DockSpace: ImGuiID(id: ImGuiID, size_arg: &ImVec2, ImGuiDockNodeFlags flags, *const ImGuiWindowClass window_class)
{
    ctx: *mut ImGuiContext = GImGui;
    let g =  ctx;
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
    node:*mut ImGuiDockNode = DockContextFindNodeByID(ctx, id);
    if (!node)
    {
        IMGUI_DEBUG_LOG_DOCKING("[docking] DockSpace: dockspace node 0x%08X created\n", id);
        node = DockContextAddNode(ctx, id);
        node.SetLocalFlags(ImGuiDockNodeFlags_CentralNode);
    }
    if (window_class && window_class.ClassId != node.WindowClass.ClassId)
        IMGUI_DEBUG_LOG_DOCKING("[docking] DockSpace: dockspace node 0x%08X: setup WindowClass 0x%08X -> 0x%08X\n", id, node.WindowClass.ClassId, window_class.ClassId);
    node.SharedFlags = flags;
    node.WindowClass = window_class ? *window_class : ImGuiWindowClass();

    // When a DockSpace transitioned form implicit to explicit this may be called a second time
    // It is possible that the node has already been claimed by a docked window which appeared before the DockSpace() node, so we overwrite IsDockSpace again.
    if (node.LastFrameActive == g.FrameCount && flag_clear(flags, ImGuiDockNodeFlags_KeepAliveOnly))
    {
        // IM_ASSERT(node.IsDockSpace() == false && "Cannot call DockSpace() twice a frame with the same ID");
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
        size.x = ImMax(content_avail.x + size.x, 4.0); // Arbitrary minimum child size (0.0 causing too much issues)
    if (size.y <= 0.0)
        size.y = ImMax(content_avail.y + size.y, 4.0);
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
DockSpaceOverViewport: ImGuiID(*const ImGuiViewport viewport, ImGuiDockNodeFlags dockspace_flags, *const ImGuiWindowClass window_class)
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

pub unsafe fn DockBuilderDockWindow(window_name: *const c_char, node_id: ImGuiID)
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
        settings.DockId = node_id;
        settings.DockOrder = -1;
    }
}

DockBuilderGetNode:*mut ImGuiDockNode(node_id: ImGuiID)
{
    ctx: *mut ImGuiContext = GImGui;
    return DockContextFindNodeByID(ctx, node_id);
}

pub unsafe fn DockBuilderSetNodePos(node_id: ImGuiID, pos: ImVec2)
{
    ctx: *mut ImGuiContext = GImGui;
    node:*mut ImGuiDockNode = DockContextFindNodeByID(ctx, node_id);
    if (node == null_mut())
        return;
    node.Pos = pos;
    node.AuthorityForPos = ImGuiDataAuthority_DockNode;
}

pub unsafe fn DockBuilderSetNodeSize(node_id: ImGuiID, size: ImVec2)
{
    ctx: *mut ImGuiContext = GImGui;
    node:*mut ImGuiDockNode = DockContextFindNodeByID(ctx, node_id);
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
DockBuilderAddNode: ImGuiID(id: ImGuiID, ImGuiDockNodeFlags flags)
{
    ctx: *mut ImGuiContext = GImGui;

    if (id != 0)
        DockBuilderRemoveNode(id);

    node:*mut ImGuiDockNode= null_mut();
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

pub unsafe fn DockBuilderRemoveNode(node_id: ImGuiID)
{
    ctx: *mut ImGuiContext = GImGui;
    node:*mut ImGuiDockNode = DockContextFindNodeByID(ctx, node_id);
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
pub unsafe fn DockBuilderRemoveNodeChildNodes(root_id: ImGuiID)
{
    ctx: *mut ImGuiContext = GImGui;
    dc: *mut ImGuiDockContext  = &ctx.DockContext;

    root_node:*mut ImGuiDockNode = root_id ? DockContextFindNodeByID(ctx, root_id) : null_mut();
    if (root_id && root_node == null_mut())
        return;
    let mut has_central_node: bool =  false;

    ImGuiDataAuthority backup_root_node_authority_for_pos = root_node ? root_node.AuthorityForPos : ImGuiDataAuthority_Auto;
    ImGuiDataAuthority backup_root_node_authority_for_size = root_node ? root_node.AuthorityForSize : ImGuiDataAuthority_Auto;

    // Process active windows
    Vec<ImGuiDockNode*> nodes_to_remove;
    for (let n: c_int = 0; n < dc->Nodes.Data.Size; n++)
        if (node:*mut ImGuiDockNode = dc->Nodes.Data[n].val_p)
        {
            let mut want_removal: bool =  (root_id == 0) || (node.ID != root_id && DockNodeGetRootNode(node).ID == root_id);
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

    // DockNodeMoveWindows.DockNodeAddWindow will normally set those when reaching two windows (which is only adequate during interactive merge)
    // Make sure we don't lose our current pos/size. (FIXME-DOCK: Consider tidying up that code in DockNodeAddWindow instead)
    if (root_node)
    {
        root_node.AuthorityForPos = backup_root_node_authority_for_pos;
        root_node.AuthorityForSize = backup_root_node_authority_for_size;
    }

    // Apply to settings
    for (settings: *mut ImGuiWindowSettings = ctx->SettingsWindows.begin(); settings != null_mut(); settings = ctx->SettingsWindows.next_chunk(settings))
        if (let mut window_settings_dock_id: ImGuiID =  settings.DockId)
            for (let n: c_int = 0; n < nodes_to_remove.Size; n++)
                if (nodes_to_remove[n].ID == window_settings_dock_id)
                {
                    settings.DockId = root_id;
                    break;
                }

    // Not really efficient, but easier to destroy a whole hierarchy considering DockContextRemoveNode is attempting to merge nodes
    if (nodes_to_remove.Size > 1)
        ImQsort(nodes_to_remove.Data, nodes_to_remove.Size, sizeof, DockNodeComparerDepthMostFirst);
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

pub unsafe fn DockBuilderRemoveNodeDockedWindows(root_id: ImGuiID, clear_settings_refs: bool)
{
    // Clear references in settings
    ctx: *mut ImGuiContext = GImGui;
    let g =  ctx;
    if (clear_settings_refs)
    {
        for (settings: *mut ImGuiWindowSettings = g.SettingsWindows.begin(); settings != null_mut(); settings = g.SettingsWindows.next_chunk(settings))
        {
            let mut want_removal: bool =  (root_id == 0) || (settings.DockId == root_id);
            if (!want_removal && settings.DockId != 0)
                if (node:*mut ImGuiDockNode = DockContextFindNodeByID(ctx, settings.DockId))
                    if (DockNodeGetRootNode(node).ID == root_id)
                        want_removal = true;
            if (want_removal)
                settings.DockId = 0;
        }
    }

    // Clear references in windows
    for (let n: c_int = 0; n < g.Windows.len(); n++)
    {
        let mut window: *mut ImGuiWindow =  g.Windows[n];
        let mut want_removal: bool =  (root_id == 0) || (window.DockNode && DockNodeGetRootNode(window.DockNode).ID == root_id) || (window.DockNodeAsHost && window.DockNodeAsHost.ID == root_id);
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
DockBuilderSplitNode: ImGuiID(id: ImGuiID, split_dir: ImGuiDir,size_ratio_for_node_at_dir: c_float, ImGuiID* out_id_at_dir, ImGuiID* out_id_at_opposite_dir)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(split_dir != ImGuiDir_None);
    IMGUI_DEBUG_LOG_DOCKING("[docking] DockBuilderSplitNode: node 0x%08X, split_dir %d\n", id, split_dir);

    node:*mut ImGuiDockNode = DockContextFindNodeByID(&g, id);
    if (node == null_mut())
    {
        // IM_ASSERT(node != NULL);
        return 0;
    }

    // IM_ASSERT(!node.IsSplitNode()); // Assert if already Split

    req: ImGuiDockRequest;
    req.Type = ImGuiDockRequestType_Split;
    req.DockTargetWindow= null_mut();
    req.DockTargetNode = node;
    req.DockPayload= null_mut();
    req.DockSplitDir = split_dir;
    req.DockSplitRatio = ImSaturate((split_dir == ImGuiDir_Left || split_dir == ImGuiDir_Up) ? size_ratio_for_node_at_dir : 1.0 - size_ratio_for_node_at_dir);
    req.DockSplitOuter = false;
    DockContextProcessDock(&g, &req);

    let mut id_at_dir: ImGuiID =  node.ChildNodes[(split_dir == ImGuiDir_Left || split_dir == ImGuiDir_Up) ? 0 : 1].ID;
    let mut id_at_opposite_dir: ImGuiID =  node.ChildNodes[(split_dir == ImGuiDir_Left || split_dir == ImGuiDir_Up) ? 1 : 0].ID;
    if (out_id_at_dir)
        *out_id_at_dir = id_at_dir;
    if (out_id_at_opposite_dir)
        *out_id_at_opposite_dir = id_at_opposite_dir;
    return id_at_dir;
}

static DockBuilderCopyNodeRec:*mut ImGuiDockNode(src_node:*mut ImGuiDockNode, dst_node_id_if_known: ImGuiID, Vec<ImGuiID>* out_node_remap_pairs)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    dst_node:*mut ImGuiDockNode = DockContextAddNode(&g, dst_node_id_if_known);
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
            dst_node.ChildNodes[child_n].ParentNode = dst_node;
        }

    IMGUI_DEBUG_LOG_DOCKING("[docking] Fork node %08X -> %08X (%d childs)\n", src_node.ID, dst_node.ID, dst_node.IsSplitNode() ? 2 : 0);
    return dst_node;
}

pub unsafe fn DockBuilderCopyNode(src_node_id: ImGuiID, dst_node_id: ImGuiID, Vec<ImGuiID>* out_node_remap_pairs)
{
    ctx: *mut ImGuiContext = GImGui;
    // IM_ASSERT(src_node_id != 0);
    // IM_ASSERT(dst_node_id != 0);
    // IM_ASSERT(out_node_remap_pairs != NULL);

    DockBuilderRemoveNode(dst_node_id);

    src_node:*mut ImGuiDockNode = DockContextFindNodeByID(ctx, src_node_id);
    // IM_ASSERT(src_node != NULL);

    out_node_remap_pairs->clear();
    DockBuilderCopyNodeRec(src_node, dst_node_id, out_node_remap_pairs);

    // IM_ASSERT((out_node_remap_pairs.Size % 2) == 0);
}

pub unsafe fn DockBuilderCopyWindowSettings(src_name: *const c_char, dst_name: *const c_char)
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
            dst_settings.ViewportPos = window_pos_2ih;
            dst_settings.ViewportId = src_window.ViewportId;
            dst_settings.Pos = ImVec2ih(0, 0);
        }
        else
        {
            dst_settings.Pos = window_pos_2ih;
        }
        dst_settings.Size = ImVec2ih(src_window.SizeFull);
        dst_settings->Collapsed = src_window.Collapsed;
    }
}

// FIXME: Will probably want to change this signature, in particular how the window remapping pairs are passed.
pub unsafe fn DockBuilderCopyDockSpace(src_dockspace_id: ImGuiID, dst_dockspace_id: ImGuiID, Vec<*const char>* in_window_remap_pairs)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(src_dockspace_id != 0);
    // IM_ASSERT(dst_dockspace_id != 0);
    // IM_ASSERT(in_window_remap_pairs != NULL);
    // IM_ASSERT((in_window_remap_pairs.Size % 2) == 0);

    // Duplicate entire dock
    // FIXME: When overwriting dst_dockspace_id, windows that aren't part of our dockspace window class but that are docked in a same node will be split apart,
    // whereas we could attempt to at least keep them together in a new, same floating node.
    Vec<ImGuiID> node_remap_pairs;
    DockBuilderCopyNode(src_dockspace_id, dst_dockspace_id, &node_remap_pairs);

    // Attempt to transition all the upcoming windows associated to dst_dockspace_id into the newly created hierarchy of dock nodes
    // (The windows associated to src_dockspace_id are staying in place)
    Vec<ImGuiID> src_windows;
    for (let remap_window_n: c_int = 0; remap_window_n < in_window_remap_pairs.Size; remap_window_n += 2)
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
            node:*mut ImGuiDockNode = DockBuilderGetNode(src_dock_id);
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
pub unsafe fn DockBuilderFinish(root_id: ImGuiID)
{
    ctx: *mut ImGuiContext = GImGui;
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

pub unsafe fn GetWindowAlwaysWantOwnTabBar(window: *mut ImGuiWindow) -> bool
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.IO.ConfigDockingAlwaysTabBar || window.WindowClass.DockingAlwaysTabBar)
        if ((window.Flags & (ImGuiWindowFlags_ChildWindow | ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoDocking)) == 0)
            if (!window.IsFallbackWindow)    // We don't support AlwaysTabBar on the fallback/implicit window to avoid unused dock-node overhead/noise
                return true;
    return false;
}

static DockContextBindNodeToWindow:*mut ImGuiDockNode(ctx: *mut ImGuiContext, window: *mut ImGuiWindow)
{
    let g =  ctx;
    node:*mut ImGuiDockNode = DockContextFindNodeByID(ctx, window.DockId);
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
        ancestor_node:*mut ImGuiDockNode = node;
        while (!ancestor_node.IsVisible && ancestor_node.ParentNode)
            ancestor_node = ancestor_node.ParentNode;
        // IM_ASSERT(ancestor_node.Size.x > 0.0 && ancestor_node.Size.y > 0.0);
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

pub unsafe fn BeginDocked(window: *mut ImGuiWindow,p_open: *mut bool)
{
    ctx: *mut ImGuiContext = GImGui;
    let g =  ctx;

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
    node:*mut ImGuiDockNode = window.DockNode;
    if (node != null_mut())
        // IM_ASSERT(window.DockId == node.ID);
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
        root_node:*mut ImGuiDockNode = DockNodeGetRootNode(node);
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
    // IM_ASSERT(node.Size.x >= 0.0 && node.Size.y >= 0.0);
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

pub unsafe fn BeginDockableDragDropSource(window: *mut ImGuiWindow)
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

pub unsafe fn BeginDockableDragDropTarget(window: *mut ImGuiWindow)
{
    ctx: *mut ImGuiContext = GImGui;
    let g =  ctx;

    //IM_ASSERT(window.RootWindowDockTree == window); // May also be a DockSpace
    // IM_ASSERT((window.Flags & ImGuiWindowFlags_NoDocking) == 0);
    if (!g.DragDropActive)
        return;
    //GetForegroundDrawList(window).AddRect(window.Pos, window.Pos + window.Size, IM_COL32(255, 255, 0, 255));
    if (!BeginDragDropTargetCustom(window.Rect(), window.ID))
        return;

    // Peek into the payload before calling AcceptDragDropPayload() so we can handle overlapping dock nodes with filtering
    // (this is a little unusual pattern, normally most code would call AcceptDragDropPayload directly)
    let payload: *const ImGuiPayload = &g.DragDropPayload;
    if (!payload->IsDataType(IMGUI_PAYLOAD_TYPE_WINDOW) || !DockNodeIsDropAllowed(window, *(ImGuiWindow**)payload.Data))
    {
        EndDragDropTarget();
        return;
    }

    let mut payload_window: *mut ImGuiWindow =  *(ImGuiWindow**)payload.Data;
    if (AcceptDragDropPayload(IMGUI_PAYLOAD_TYPE_WINDOW, ImGuiDragDropFlags_AcceptBeforeDelivery | ImGuiDragDropFlags_AcceptNoDrawDefaultRect))
    {
        // Select target node
        // (Important: we cannot use g.HoveredDockNode here! Because each of our target node have filters based on payload, each candidate drop target will do its own evaluation)
        let mut dock_into_floating_window: bool =  false;
        node:*mut ImGuiDockNode= null_mut();
        if (window.DockNodeAsHost)
        {
            // Cannot assume that node will != NULL even though we passed the rectangle test: it depends on padding/spacing handled by DockNodeTreeFindVisibleNodeByPos().
            node = DockNodeTreeFindVisibleNodeByPos(window.DockNodeAsHost, g.IO.MousePos);

            // There is an edge case when docking into a dockspace which only has _inactive_ nodes (because none of the windows are active)
            // In this case we need to fallback into any leaf mode, possibly the central node.
            // FIXME-20181220: We should not have to test for IsLeafNode() here but we have another bug to fix first.
            if (node && node.IsDockSpace() && node.IsRootNode())
                node = if node.CentralNode && node.IsLeafNode() { node.CentralNode} else { DockNodeTreeFindFallbackLeafNode(node)};
        }
        else
        {
            if (window.DockNode)
                node = window.DockNode;
            else
                dock_into_floating_window = true; // Dock into a regular window
        }

        let explicit_target_rect: ImRect =  (node && node.TabBar && !node.IsHiddenTabBar() && !node.IsNoTabBar()) ? node.TabBar.BarRect : ImRect(window.Pos, window.Pos + ImVec2::new(window.Size.x, GetFrameHeight()));
        let is_explicit_target: bool = g.IO.ConfigDockingWithShift || IsMouseHoveringRect(explicit_target_rect.Min, explicit_target_rect.Max);

        // Preview docking request and find out split direction/ratio
        //let do_preview: bool = true;     // Ignore testing for payload->IsPreview() which removes one frame of delay, but breaks overlapping drop targets within the same window.
        let do_preview: bool = payload->IsPreview() || payload->IsDelivery();
        if (do_preview && (node != null_mut() || dock_into_floating_window))
        {
            let mut split_inner = ImGuiDockPreviewData::default();
            let mut split_outer = ImGuiDockPreviewData::default();
            split_data: *mut ImGuiDockPreviewData = &split_inner;
            if (node && (node.ParentNode || node.IsCentralNode()))
                if (root_node:*mut ImGuiDockNode = DockNodeGetRootNode(node))
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

pub unsafe fn DockSettingsRenameNodeReferences(old_node_id: ImGuiID, new_node_id: ImGuiID)
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
        if (settings.DockId == old_node_id)
            settings.DockId = new_node_id;
}

// Remove references stored in ImGuiWindowSettings to the given ImGuiDockNodeSettings
pub unsafe fn DockSettingsRemoveNodeReferences(ImGuiID* node_ids, node_ids_count: c_int)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let found: c_int = 0;
    //// FIXME-OPT: We could remove this loop by storing the index in the map
    for (settings: *mut ImGuiWindowSettings = g.SettingsWindows.begin(); settings != null_mut(); settings = g.SettingsWindows.next_chunk(settings))
        for (let node_n: c_int = 0; node_n < node_ids_count; node_n++)
            if (settings.DockId == node_ids[node_n])
            {
                settings.DockId = 0;
                settings.DockOrder = -1;
                if (++found < node_ids_count)
                    break;
                return;
            }
}

static DockSettingsFindNodeSettings: *mut ImGuiDockNodeSettings(ctx: *mut ImGuiContext, id: ImGuiID)
{
    // FIXME-OPT
    dc: *mut ImGuiDockContext  = &ctx.DockContext;
    for (let n: c_int = 0; n < dc->NodesSettings.Size; n++)
        if (dc->NodesSettings[n].ID == id)
            return &dc->NodesSettings[n];
    return null_mut();
}

// Clear settings data
pub unsafe fn DockSettingsHandler_ClearAll(ctx: *mut ImGuiContext, ImGuiSettingsHandler*)
{
    dc: *mut ImGuiDockContext  = &ctx.DockContext;
    dc->NodesSettings.clear();
    DockContextClearNodes(ctx, 0, true);
}

// Recreate nodes based on settings data
pub unsafe fn DockSettingsHandler_ApplyAll(ctx: *mut ImGuiContext, ImGuiSettingsHandler*)
{
    // Prune settings at boot time only
    dc: *mut ImGuiDockContext  = &ctx.DockContext;
    if (ctx.Windows.len() == 0)
        DockContextPruneUnusedSettingsNodes(ctx);
    DockContextBuildNodesFromSettings(ctx, dc->NodesSettings.Data, dc->NodesSettings.Size);
    DockContextBuildAddWindowsToNodes(ctx, 0);
}

static DockSettingsHandler_ReadOpen: *mut c_void(ImGuiContext*, ImGuiSettingsHandler*, name: *const c_char)
{
    if (strcmp(name, "Data") != 0)
        return null_mut();
    return 1;
}

pub unsafe fn DockSettingsHandler_ReadLine(ctx: *mut ImGuiContext, ImGuiSettingsHandler*, *mut c_void, line: *const c_char)
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
        if (parent_settings: *mut ImGuiDockNodeSettings = DockSettingsFindNodeSettings(ctx, node.ParentNodeId))
            node.Depth = parent_settings->Depth + 1;
    ctx.DockContext.NodesSettings.push(node);
}

pub unsafe fn DockSettingsHandler_DockNodeToSettings(dc: *mut ImGuiDockContext, node:*mut ImGuiDockNode, depth: c_int)
{
    ImGuiDockNodeSettings node_settings;
    // IM_ASSERT(depth < (1 << (sizeof(node_settings.Depth) << 3)));
    node_settings.ID = node.ID;
    node_settings.ParentNodeId = node.ParentNode ? node.ParentNode.ID : 0;
    node_settings.ParentWindowId = if node.IsDockSpace() && node.HostWindow && node.Hostwindow.ParentWindow { node.Hostwindow.Parentwindow.ID} else { 0};
    node_settings.SelectedTabId = node.SelectedTabId;
    node_settings.SplitAxis = if node.IsSplitNode( { node.SplitAxis} else { ImGuiAxis_None)};
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

pub unsafe fn DockSettingsHandler_WriteAll(ctx: *mut ImGuiContext, ImGuiSettingsHandler* handler, ImGuiTextBuffer* buf)
{
    let g =  ctx;
    dc: *mut ImGuiDockContext = &ctx.DockContext;
    if (!(g.IO.ConfigFlags & ImGuiConfigFlags_DockingEnable))
        return;

    // Gather settings data
    // (unlike our windows settings, because nodes are always built we can do a full rewrite of the SettingsNode buffer)
    dc->NodesSettings.clear();
    dc->NodesSettings.reserve(dc->Nodes.Data.Size);
    for (let n: c_int = 0; n < dc->Nodes.Data.Size; n++)
        if (node:*mut ImGuiDockNode = dc->Nodes.Data[n].val_p)
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
        buf->appendf("%*s%s%*s", node_settings->Depth * 2, "", (node_settings.Flags & ImGuiDockNodeFlags_DockSpace) ? "DockSpace" : "DockNode ", (max_depth - node_settings->Depth) * 2, "");  // Text align nodes to facilitate looking at .ini file
        buf->appendf(" ID=0x%08X", node_settings.ID);
        if (node_settings.ParentNodeId)
        {
            buf->appendf(" Parent=0x%08X SizeRef=%d,%d", node_settings.ParentNodeId, node_settings.SizeRef.x, node_settings.SizeRef.y);
        }
        else
        {
            if (node_settings->ParentWindowId)
                buf->appendf(" Window=0x%08X", node_settings->ParentWindowId);
            buf->appendf(" Pos=%d,%d Size=%d,%d", node_settings.Pos.x, node_settings.Pos.y, node_settings.Size.x, node_settings.Size.y);
        }
        if (node_settings->SplitAxis != ImGuiAxis_None)
            buf->appendf(" Split=%c", (node_settings->SplitAxis == ImGuiAxis_X) ? 'X' : 'Y');
        if (node_settings.Flags & ImGuiDockNodeFlags_NoResize)
            buf->appendf(" NoResize=1");
        if (node_settings.Flags & ImGuiDockNodeFlags_CentralNode)
            buf->appendf(" CentralNode=1");
        if (node_settings.Flags & ImGuiDockNodeFlags_NoTabBar)
            buf->appendf(" NoTabBar=1");
        if (node_settings.Flags & ImGuiDockNodeFlags_HiddenTabBar)
            buf->appendf(" HiddenTabBar=1");
        if (node_settings.Flags & ImGuiDockNodeFlags_NoWindowMenuButton)
            buf->appendf(" NoWindowMenuButton=1");
        if (node_settings.Flags & ImGuiDockNodeFlags_NoCloseButton)
            buf->appendf(" NoCloseButton=1");
        if (node_settings.SelectedTabId)
            buf->appendf(" Selected=0x%08X", node_settings.SelectedTabId);

// #if IMGUI_DEBUG_INI_SETTINGS
        // [DEBUG] Include comments in the .ini file to ease debugging
        if (node:*mut ImGuiDockNode = DockContextFindNodeByID(ctx, node_settings.ID))
        {
            buf->appendf("%*s", ImMax(2, (line_start_pos + 92) - buf->size()), "");     // Align everything
            if (node.IsDockSpace() && node.HostWindow && node.Hostwindow.ParentWindow)
                buf->appendf(" ; in '%s'", node.Hostwindow.Parentwindow.Name);
            // Iterate settings so we can give info about windows that didn't exist during the session.
            let contains_window: c_int = 0;
            for (settings: *mut ImGuiWindowSettings = g.SettingsWindows.begin(); settings != null_mut(); settings = g.SettingsWindows.next_chunk(settings))
                if (settings.DockId == node_settings.ID)
                {
                    if (contains_window++ == 0)
                        buf->appendf(" ; contains ");
                    buf->appendf("'%s' ", settings.GetName());
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
static GetClipboardTextFn_DefaultImpl: *const c_char
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

pub unsafe fn SetClipboardTextFn_DefaultImpl(*mut c_void, text: *const c_char)
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
pub unsafe fn SetClipboardTextFn_DefaultImpl(*mut c_void, text: *const c_char)
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

static GetClipboardTextFn_DefaultImpl: *const c_char
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
static GetClipboardTextFn_DefaultImpl: *const c_char
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.ClipboardHandlerData.empty() ? null_mut() : g.ClipboardHandlerData.begin();
}

pub unsafe fn SetClipboardTextFn_DefaultImpl(*mut c_void, text: *const c_char)
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

pub unsafe fn SetPlatformImeDataFn_DefaultImpl(viewport: *mut ImGuiViewport, ImGuiPlatformImeData* data)
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

pub unsafe fn SetPlatformImeDataFn_DefaultImpl(ImGuiViewport*, ImGuiPlatformImeData*) {}

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

pub unsafe fn DebugRenderViewportThumbnail(draw_list: *mut ImDrawList, viewport: *mut ImGuiViewport, bb: &ImRect)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;

    let scale: ImVec2 = bb.GetSize() / viewport.Size;
    let off: ImVec2 = bb.Min - viewport.Pos * scale;
    let alpha_mul: c_float =  (viewport.Flags & ImGuiViewportFlags_Minimized) ? 0.3f32 : 1.0;
    window.DrawList.AddRectFilled(bb.Min, bb.Max, GetColorU32(ImGuiCol_Border, alpha_mul * 0.4));
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

pub unsafe fn RenderViewportsThumbnails()
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

: c_int ViewportComparerByFrontMostStampCount(lhs: *const c_void, rhs: *const c_void)
{
    let mut a: *mut ImGuiViewport =  *(const const: *mut ImGuiViewport*)lhs;
    let mut b: *mut ImGuiViewport =  *(const const: *mut ImGuiViewport*)rhs;
    return b->LastFrontMostStampCount - a->LastFrontMostStampCount;
}

// Helper tool to diagnose between text encoding issues and font loading issues. Pass your UTF-8 string and verify that there are correct.
pub unsafe fn DebugTextEncoding(str: *const c_char)
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
pub unsafe fn MetricsHelpMarker(desc: *const c_char)
{
    TextDisabled("(?)");
    if (IsItemHovered(ImGuiHoveredFlags_DelayShort))
    {
        BeginTooltip();
        PushTextWrapPos(GetFontSize() * 35.0);
        TextUnformatted(desc);
        PopTextWrapPos();
        EndTooltip();
    }
}

// [DEBUG] List fonts in a font atlas and display its texture
pub unsafe fn ShowFontAtlas(atlas: *mut ImFontAtlas)
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
        border_col: ImVec4 = ImVec4(1.0, 1.0, 1.0, 0.5);
        Image(atlas->TexID, ImVec2::new(atlas->TexWidth, atlas->TexHeight), ImVec2::new(0.0, 0.0), ImVec2::new(1.0, 1.0), tint_col, border_col);
        TreePop();
    }
}

pub unsafe fn ShowMetricsWindow(bool* p_open)
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
    Text("Application average %.3f ms/frame (%.1f FPS)", 1000 / io.Framerate, io.Framerate);
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
        static pub unsafe fn GetTableRect(ImGuiTable* table, rect_type: c_int, n: c_int) -> ImRect
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

        static pub unsafe fn GetWindowRect(window: *mut ImGuiWindow, rect_type: c_int) -> ImRect
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
            InputText("##Text", buf, buf.len());
            if (buf[0] != 0)
                DebugTextEncoding(buf);
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
                    GetForegroundDrawList().AddRect(table.OuterRect.Min - ImVec2::new(1, 1), table.OuterRect.Max + ImVec2::new(1, 1), IM_COL32(255, 255, 0, 255), 0.0, 0, 2.0);
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
                            ImFormatString(buf, buf.len(), "(%6.1f,%6.10f32) (%6.1f,%6.10f32) Size (%6.1f,%6.10f32) Col %d %s", r.Min.x, r.Min.y, r.Max.x, r.Max.y, r.GetWidth(), r.GetHeight(), column_n, trt_rects_names[rect_n]);
                            Selectable(buf);
                            if (IsItemHovered())
                                GetForegroundDrawList().AddRect(r.Min - ImVec2::new(1, 1), r.Max + ImVec2::new(1, 1), IM_COL32(255, 255, 0, 255), 0.0, 0, 2.0);
                        }
                    }
                    else
                    {
                        let r: ImRect =  Funcs::GetTableRect(table, rect_n, -1);
                        ImFormatString(buf, buf.len(), "(%6.1f,%6.10f32) (%6.1f,%6.10f32) Size (%6.1f,%6.10f32) %s", r.Min.x, r.Min.y, r.Max.x, r.Max.y, r.GetWidth(), r.GetHeight(), trt_rects_names[rect_n]);
                        Selectable(buf);
                        if (IsItemHovered())
                            GetForegroundDrawList().AddRect(r.Min - ImVec2::new(1, 1), r.Max + ImVec2::new(1, 1), IM_COL32(255, 255, 0, 255), 0.0, 0, 2.0);
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
            struct Func { : c_int WindowComparerByBeginOrder(lhs: *const c_void, rhs: *const c_void) { return ((*(*const ImGuiWindow const *)lhs).BeginOrderWithinContext - (*(*const ImGuiWindow const*)rhs).BeginOrderWithinContext); } };
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
                BulletText("Monitor #%d: DPI %.0f%%\n MainMin (%.0.0,%.0), MainMax (%.0.0,%.0), MainSize (%.0.0,%.0)\n WorkMin (%.0.0,%.0), WorkMax (%.0.0,%.0), WorkSize (%.0.0,%.0)",
                    i, mon.DpiScale * 100,
                    mon.MainPos.x, mon.MainPos.y, mon.MainPos.x + mon.MainSize.x, mon.MainPos.y + mon.MainSize.y, mon.MainSize.x, mon.MainSize.y,
                    mon.WorkPos.x, mon.WorkPos.y, mon.WorkPos.x + mon.WorkSize.x, mon.WorkPos.y + mon.WorkSize.y, mon.WorkSize.x, mon.WorkSize.y);
            }
            TreePop();
        }

        BulletText("MouseViewport: 0x%08X (UserHovered 0x%08X, LastHovered 0x%08X)", g.MouseViewport ? g.MouseViewport.ID : 0, g.IO.MouseHoveredViewport, g.MouseLastHoveredViewport ? g.MouseLastHoveredViewport.ID : 0);
        if (TreeNode("Inferred Z order (front-to-back)"))
        {
            static Vec<*mut ImGuiViewportP> viewports;
            viewports.resize(g.Viewports.len());
            memcpy(viewports.Data, g.Viewports.Data, g.Viewports.size_in_bytes());
            if (viewports.Size > 1)
                ImQsort(viewports.Data, viewports.Size, sizeof(ImGuiViewport*), ViewportComparerByFrontMostStampCount);
            for (let i: c_int = 0; i < viewports.Size; i++)
                BulletText("Viewport #%d, ID: 0x%08X, FrontMostStampCount = %08d, Window: \"%s\"", viewports[i]->Idx, viewports[i].ID, viewports[i]->LastFrontMostStampCount, viewports[i].Window ? viewports[i]->window.Name : "N/A");
            TreePop();
        }

        for (let i: c_int = 0; i < g.Viewports.len(); i++)
            DebugNodeViewport(g.Viewports[i]);
        TreePop();
    }

    // Details for Popups
    if (TreeNode("Popups", "Popups (%d)", g.OpenPopupStack.len()))
    {
        for (let i: c_int = 0; i < g.OpenPopupStack.len(); i++)
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
        dc: *mut ImGuiDockContext = &g.DockContext;
        Checkbox("List root nodes", &root_nodes_only);
        Checkbox("Ctrl shows window dock info", &cfg->ShowDockingNodes);
        if (SmallButton("Clear nodes")) { DockContextClearNodes(&g, 0, true); }
        SameLine();
        if (SmallButton("Rebuild all")) { dc->WantFullRebuild = true; }
        for (let n: c_int = 0; n < dc->Nodes.Data.Size; n++)
            if (node:*mut ImGuiDockNode = dc->Nodes.Data[n].val_p)
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
            dc: *mut ImGuiDockContext = &g.DockContext;
            Text("In SettingsWindows:");
            for (settings: *mut ImGuiWindowSettings = g.SettingsWindows.begin(); settings != null_mut(); settings = g.SettingsWindows.next_chunk(settings))
                if (settings.DockId != 0)
                    BulletText("Window '%s' -> DockId %08X", settings.GetName(), settings.DockId);
            Text("In SettingsNodes:");
            for (let n: c_int = 0; n < dc->NodesSettings.Size; n++)
            {
                settings: *mut ImGuiDockNodeSettings = &dc->NodesSettings[n];
                let mut  selected_tab_name: *const c_char= null_mut();
                if (settings.SelectedTabId)
                {
                    if (let mut window: *mut ImGuiWindow =  FindWindowByID(settings.SelectedTabId))
                        selected_tab_name = window.Name;
                    else if (window_settings: *mut ImGuiWindowSettings = FindWindowSettings(settings.SelectedTabId))
                        selected_tab_name = window_settings->GetName();
                }
                BulletText("Node %08X, Parent %08X, SelectedTab %08X ('%s')", settings.ID, settings.ParentNodeId, settings.SelectedTabId, selected_tab_name ? selected_tab_name : settings.SelectedTabId ? "N/A" : "");
            }
            TreePop();
        }
// #endif // #ifdef IMGUI_HAS_DOCK

        if (TreeNode("SettingsIniData", "Settings unpacked data (.ini): %d bytes", g.SettingsIniData.size()))
        {
            InputTextMultiline("##Ini", g.SettingsIniData.c_str(), g.SettingsIniData.Buf.Size, ImVec2::new(-FLT_MIN, GetTextLineHeight() * 20), ImGuiInputTextFlags_ReadOnly);
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
        Text("MouseViewport: 0x%08X (UserHovered 0x%08X, LastHovered 0x%08X)", g.MouseViewport.ID, g.IO.MouseHoveredViewport, g.MouseLastHoveredViewport ? g.MouseLastHoveredViewport.ID : 0);
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
        Text("NavWindowingTarget: '%s'", g.NavWindowingTarget ? g.NavWindowingTarget.Name : "NULL");
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
                ImFormatString(buf, buf.len(), "%d", window.BeginOrderWithinContext);
                let font_size: c_float =  GetFontSize();
                draw_list.AddRectFilled(window.Pos, window.Pos + ImVec2::new(font_size, font_size), IM_COL32(200, 100, 100, 255));
                draw_list.AddText(window.Pos, IM_COL32(255, 255, 255, 255), buf);
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
                    col: u32 = if table.HoveredColumnBody == column_n { IM_COL32(255, 255, 128, 255)} else { IM_COL32(255, 0, 128, 255)};
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
        node:*mut ImGuiDockNode = g.DebugHoveredDockNode;
        let mut  overlay_draw_list: *mut ImDrawList =  node.HostWindow ? GetForegroundDrawList(node.HostWindow) : GetForegroundDrawList(GetMainViewport());
        p += ImFormatString(p, buf + buf.len() - p, "DockId: %X%s\n", node.ID, node.IsCentralNode() ? " *CentralNode*" : "");
        p += ImFormatString(p, buf + buf.len() - p, "WindowClass: %08X\n", node.WindowClass.ClassId);
        p += ImFormatString(p, buf + buf.len() - p, "Size: (%.0.0, %.0)\n", node.Size.x, node.Size.y);
        p += ImFormatString(p, buf + buf.len() - p, "SizeRef: (%.0.0, %.0)\n", node.SizeRef.x, node.SizeRef.y);
        let depth: c_int = DockNodeGetDepth(node);
        overlay_draw_list.AddRect(node.Pos + ImVec2::new(3, 3) * depth, node.Pos + node.Size - ImVec2::new(3, 3) * depth, IM_COL32(200, 100, 100, 255));
        let pos: ImVec2 = node.Pos + ImVec2::new(3, 3) * depth;
        overlay_draw_list.AddRectFilled(pos - ImVec2::new(1, 1), pos + CalcTextSize(buf) + ImVec2::new(1, 1), IM_COL32(200, 100, 100, 255));
        overlay_draw_list.AddText(null_mut(), 0.0, pos, IM_COL32(255, 255, 255, 255), buf);
    }
// #endif // #ifdef IMGUI_HAS_DOCK

    End();
}

// [DEBUG] Display contents of Columns
pub unsafe fn DebugNodeColumns(ImGuiOldColumns* columns)
{
    if (!TreeNode((uintptr_t)columns.ID, "Columns Id: 0x%08X, Count: %d, Flags: 0x%04X", columns.ID, columns->Count, columns.Flags))
        return;
    BulletText("Width: %.1f (MinX: %.1f, MaxX: %.10f32)", columns->OffMaxX - columns->OffMinX, columns->OffMinX, columns->OffMaxX);
    for (let column_n: c_int = 0; column_n < columns->Columns.Size; column_n++)
        BulletText("Column %02d: OffsetNorm %.3f (= %.1f px)", column_n, columns->Columns[column_n].OffsetNorm, GetColumnOffsetFromNorm(columns, columns->Columns[column_n].OffsetNorm));
    TreePop();
}

pub unsafe fn DebugNodeDockNodeFlags(ImGuiDockNodeFlags* p_flags, label: *const c_char, enabled: bool)
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
pub unsafe fn DebugNodeDockNode(node:*mut ImGuiDockNode, label: *const c_char)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let is_alive: bool = (g.FrameCount - node.LastFrameAlive < 2);    // Submitted with ImGuiDockNodeFlags_KeepAliveOnly
    let is_active: bool = (g.FrameCount - node.LastFrameActive < 2);  // Submitted
    if (!is_alive) { PushStyleColor(ImGuiCol_Text, GetStyleColorVec4(ImGuiCol_TextDisabled)); }
    open: bool;
    ImGuiTreeNodeFlags tree_node_flags = node.IsFocused ? ImGuiTreeNodeFlags_Selected : ImGuiTreeNodeFlags_None;
    if (node.Windows.len() > 0)
        open = TreeNodeEx(node.ID, tree_node_flags, "%s 0x%04X%s: %d windows (vis: '%s')", label, node.ID, node.IsVisible ? "" : " (hidden)", node.Windows.len(), node.VisibleWindow ? node.Visiblewindow.Name : "NULL");
    else
        open = TreeNodeEx(node.ID, tree_node_flags, "%s 0x%04X%s: %s split (vis: '%s')", label, node.ID, node.IsVisible ? "" : " (hidden)", (node.SplitAxis == ImGuiAxis_X) ? "horizontal" : (node.SplitAxis == ImGuiAxis_Y) ? "vertical" : "n/a", node.VisibleWindow ? node.Visiblewindow.Name : "NULL");
    if (!is_alive) { PopStyleColor(); }
    if (is_active && IsItemHovered())
        if (let mut window: *mut ImGuiWindow =  node.HostWindow ? node.HostWindow : node.VisibleWindow)
            GetForegroundDrawList(window).AddRect(node.Pos, node.Pos + node.Size, IM_COL32(255, 255, 0, 255));
    if (open)
    {
        // IM_ASSERT(node->ChildNodes[0] == NULL || node->ChildNodes[0].ParentNode == node);
        // IM_ASSERT(node->ChildNodes[1] == NULL || node->ChildNodes[1].ParentNode == node);
        BulletText("Pos (%.0.0,%.0), Size (%.0.0, %.0) Ref (%.0.0, %.0)",
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
pub unsafe fn DebugNodeDrawList(window: *mut ImGuiWindow, viewport: *mut ImGuiViewport, *const ImDrawList draw_list, label: *const c_char)
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
    if (is_not_null(window) && IsItemHovered() && fg_draw_list)
        fg_draw_list.AddRect(window.Pos, window.Pos + window.Size, IM_COL32(255, 255, 0, 255));
    if (!node_open)
        return;

    if (is_not_null(window) && !window.WasActive)
        TextDisabled("Warning: owning Window is inactive. This DrawList is not being rendered!");

    for (*const ImDrawCmd pcmd = draw_list.CmdBuffer; pcmd < draw_list.CmdBuffer + cmd_count; pcmd++)
    {
        if (pcmd->UserCallback)
        {
            BulletText("Callback %p, user_data %p", pcmd->UserCallback, pcmd->UserCallbackData);
            continue;
        }

        buf: [c_char;300];
        ImFormatString(buf, buf.len(), "DrawCmd:%5d tris, Tex 0x%p, ClipRect (%4.0.0,%4.0)-(%4.0.0,%4.0)",
            pcmd->ElemCount / 3, pcmd.TextureId,
            pcmd->ClipRect.x, pcmd->ClipRect.y, pcmd->ClipRect.z, pcmd->ClipRect.w);
        let mut pcmd_node_open: bool =  TreeNode((pcmd - draw_list.CmdBuffer.begin()), "%s", buf);
        if (IsItemHovered() && (cfg->ShowDrawCmdMesh || cfg->ShowDrawCmdBoundingBoxes) && fg_draw_list)
            DebugNodeDrawCmdShowMeshAndBoundingBox(fg_draw_list, draw_list, pcmd, cfg->ShowDrawCmdMesh, cfg->ShowDrawCmdBoundingBoxes);
        if (!pcmd_node_open)
            continue;

        // Calculate approximate coverage area (touched pixel count)
        // This will be in pixels squared as long there's no post-scaling happening to the renderer output.
        let idx_buffer: *const ImDrawIdx = if draw_list.IdxBuffer.len() > 0 { draw_list.IdxBuffer.Data} else { null_mut()};
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
        ImFormatString(buf, buf.len(), "Mesh: ElemCount: %d, VtxOffset: +%d, IdxOffset: +%d, Area: ~%0.f px", pcmd->ElemCount, pcmd->VtxOffset, pcmd->IdxOffset, total_area);
        Selectable(buf);
        if (IsItemHovered() && fg_draw_list)
            DebugNodeDrawCmdShowMeshAndBoundingBox(fg_draw_list, draw_list, pcmd, true, false);

        // Display individual triangles/vertices. Hover on to get the corresponding triangle highlighted.
        ImGuiListClipper clipper;
        clipper.Begin(pcmd->ElemCount / 3); // Manually coarse clip our print out of individual vertices to save CPU, only items that may be visible.
        while (clipper.Step())
            for (let prim: c_int = clipper.DisplayStart, idx_i = pcmd->IdxOffset + clipper.DisplayStart * 3; prim < clipper.DisplayEnd; prim++)
            {
                char* buf_p = buf, * buf_end = buf + buf.len();
                triangle: ImVec2[3];
                for (let n: c_int = 0; n < 3; n++, idx_i++)
                {
                    const ImDrawVert& v = vtx_buffer[idx_buffer ? idx_buffer[idx_i] : idx_i];
                    triangle[n] = v.pos;
                    buf_p += ImFormatString(buf_p, buf_end - buf_p, "%s %04d: pos (%8.2f,%8.20), uv (%.6f,%.60), col %08X\n",
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
pub unsafe fn DebugNodeDrawCmdShowMeshAndBoundingBox(ImDrawList* out_draw_list, *const ImDrawList draw_list, *const ImDrawCmd draw_cmd, show_mesh: bool, show_aabb: bool)
{
    // IM_ASSERT(show_mesh || show_aabb);

    // Draw wire-frame version of all triangles
    let clip_rect: ImRect =  draw_cmd->ClipRect;
    let mut vtxs_rect: ImRect = ImRect::new(f32::MAX, f32::MAX, -f32::MAX, -f32::MAX);
    ImDrawListFlags backup_flags = out_draw_list.Flags;
    out_draw_list.Flags &= !ImDrawListFlags_AntiAliasedLines; // Disable AA on triangle outlines is more readable for very large and thin triangles.
    for (let mut idx_n: c_uint =  draw_cmd->IdxOffset, idx_end = draw_cmd->IdxOffset + draw_cmd->ElemCount; idx_n < idx_end; )
    {
        ImDrawIdx* idx_buffer = if draw_list.IdxBuffer.len() > 0 { draw_list.IdxBuffer.Data} else { null_mut()}; // We don't hold on those pointers past iterations as ->AddPolyline() may invalidate them if out_draw_list==draw_list
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
pub unsafe fn DebugNodeFont(font: *mut ImFont)
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
                    config_i, cfg.Name, cfg->OversampleH, cfg->OversampleV, cfg->PixelSnapH, cfg->GlyphOffset.x, cfg->GlyphOffset.y);

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
            if (!TreeNode(base, "U+%04X..U+%04X (%d %s)", base, base + 255, count, count > 1 ? "glyphs" : "glyph"))
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

pub unsafe fn DebugNodeFontGlyph(ImFont*, *const ImFontGlyph glyph)
{
    Text("Codepoint: U+%04X", glyph->Codepoint);
    Separator();
    Text("Visible: %d", glyph->Visible);
    Text("AdvanceX: %.1f", glyph->AdvanceX);
    Text("Pos: (%.2f,%.20)->(%.2f,%.20)", glyph->X0, glyph->Y0, glyph->X1, glyph->Y1);
    Text("UV: (%.3f,%.30f32)->(%.3f,%.30f32)", glyph->U0, glyph->V0, glyph->U1, glyph->V1);
}

// [DEBUG] Display contents of ImGuiStorage
pub unsafe fn DebugNodeStorage(ImGuiStorage* storage, label: *const c_char)
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
pub unsafe fn DebugNodeTabBar(ImGuiTabBar* tab_bar, label: *const c_char)
{
    // Standalone tab bars (not associated to docking/windows functionality) currently hold no discernible strings.
    buf: [c_char;256];
    char* p = buf;
    let mut  buf_end: *const c_char = buf + buf.len();
    let is_active: bool = (tab_bar->PrevFrameVisible >= GetFrameCount() - 2);
    p += ImFormatString(p, buf_end - p, "%s 0x%08X (%d tabs)%s", label, tab_bar.ID, tab_bar.Tabs.Size, is_active ? "" : " *Inactive*");
    p += ImFormatString(p, buf_end - p, "  { ");
    for (let tab_n: c_int = 0; tab_n < ImMin(tab_bar.Tabs.Size, 3); tab_n++)
    {
        ImGuiTabItem* tab = &tab_bar.Tabs[tab_n];
        p += ImFormatString(p, buf_end - p, "%s'%s'",
            tab_n > 0 ? ", " : "", (tab.Window || tab.NameOffset != -1) ? tab_bar.GetTabNametab) : "???");
    }
    p += ImFormatString(p, buf_end - p, (tab_bar.Tabs.Size > 3) ? " ... }" : " } ");
    if (!is_active) { PushStyleColor(ImGuiCol_Text, GetStyleColorVec4(ImGuiCol_TextDisabled)); }
    let mut open: bool =  TreeNode(label, "%s", buf);
    if (!is_active) { PopStyleColor(); }
    if (is_active && IsItemHovered())
    {
        let mut  draw_list: *mut ImDrawList =  GetForegroundDrawList();
        draw_list.AddRect(tab_bar.BarRect.Min, tab_bar.BarRect.Max, IM_COL32(255, 255, 0, 255));
        draw_list.AddLine(ImVec2::new(tab_bar->ScrollingRectMinX, tab_bar.BarRect.Min.y), ImVec2::new(tab_bar->ScrollingRectMinX, tab_bar.BarRect.Max.y), IM_COL32(0, 255, 0, 255));
        draw_list.AddLine(ImVec2::new(tab_bar->ScrollingRectMaxX, tab_bar.BarRect.Min.y), ImVec2::new(tab_bar->ScrollingRectMaxX, tab_bar.BarRect.Max.y), IM_COL32(0, 255, 0, 255));
    }
    if (open)
    {
        for (let tab_n: c_int = 0; tab_n < tab_bar.Tabs.Size; tab_n++)
        {
            let tab: *const ImGuiTabItem = &tab_bar.Tabs[tab_n];
            PushID(tab);
            if (SmallButton("<")) { TabBarQueueReorder(tab_bar, tab, -1); } SameLine(0, 2);
            if (SmallButton(">")) { TabBarQueueReorder(tab_bar, tab, 1); } SameLine();
            Text("%02d%c Tab 0x%08X '%s' Offset: %.2f, Width: %.2f/%.2f",
                tab_n, (tab.ID == tab_bar.SelectedTabId) ? '*' : ' ', tab.ID, (tab.Window || tab.NameOffset != -1) ? tab_bar.GetTabNametab) : "???", tab->Offset, tab->Width, tab->ContentWidth);
            PopID();
        }
        TreePop();
    }
}

pub unsafe fn DebugNodeViewport(viewport: *mut ImGuiViewport)
{
    SetNextItemOpen(true, ImGuiCond_Once);
    if (TreeNode(viewport.ID, "Viewport #%d, ID: 0x%08X, Parent: 0x%08X, Window: \"%s\"", viewport.Idx, viewport.ID, viewport.ParentViewportId, viewport.Window ? viewport.window.Name : "N/A"))
    {
        flags: ImGuiWindowFlags = viewport.Flags;
        BulletText("Main Pos: (%.0.0,%.0), Size: (%.0.0,%.0)\nWorkArea Offset Left: %.0.0 Top: %.0.0, Right: %.0.0, Bottom: %.0f\nMonitor: %d, DpiScale: %.0f%%",
            viewport.Pos.x, viewport.Pos.y, viewport.Size.x, viewport.Size.y,
            viewport.WorkOffsetMin.x, viewport.WorkOffsetMin.y, viewport.WorkOffsetMax.x, viewport.WorkOffsetMax.y,
            viewport.PlatformMonitor, viewport.DpiScale * 100);
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

pub unsafe fn DebugNodeWindow(window: *mut ImGuiWindow, label: *const c_char)
{
    if (window == null_mut())
    {
        BulletText("%s: NULL", label);
        return;
    }

    let g = GImGui; // ImGuiContext& g = *GImGui;
    let is_active: bool = window.WasActive;
    ImGuiTreeNodeFlags tree_node_flags = if window == g.NavWindow { ImGuiTreeNodeFlags_Selected} else { ImGuiTreeNodeFlags_None};
    if (!is_active) { PushStyleColor(ImGuiCol_Text, GetStyleColorVec4(ImGuiCol_TextDisabled)); }
    let open: bool = TreeNodeEx(label, tree_node_flags, "%s '%s'%s", label, window.Name, is_active ? "" : " *Inactive*");
    if (!is_active) { PopStyleColor(); }
    if (IsItemHovered() && is_active)
        GetForegroundDrawList(window).AddRect(window.Pos, window.Pos + window.Size, IM_COL32(255, 255, 0, 255));
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
    BulletText("Scroll: (%.2f/%.2f,%.2f/%.20) Scrollbar:%s%s", window.Scroll.x, window.ScrollMax.x, window.Scroll.y, window.ScrollMax.y, window.ScrollbarX ? "X" : "", window.ScrollbarY ? "Y" : "");
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
            GetForegroundDrawList(window).AddRect(r.Min + window.Pos, r.Max + window.Pos, IM_COL32(255, 255, 0, 255));
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

pub unsafe fn DebugNodeWindowSettings(settings: *mut ImGuiWindowSettings)
{
    Text("0x%08X \"%s\" Pos (%d,%d) Size (%d,%d) Collapsed=%d",
        settings.ID, settings.GetName(), settings.Pos.x, settings.Pos.y, settings.Size.x, settings.Size.y, settings.Collapsed);
}

pub unsafe fn DebugNodeWindowsList(Vec<ImGuiWindow*>* windows, label: *const c_char)
{
    if (!TreeNode(label, "%s (%d)", label, windows.Size))
        return;
    for (let i: c_int = windows.Size - 1; i >= 0; i--) // Iterate front to back
    {
        PushID((*windows)[i]);
        DebugNodeWindow((*windows)[i], "Window");
        PopID();
    }
    TreePop();
}

// FIXME-OPT: This is technically suboptimal, but it is simpler this way.
pub unsafe fn DebugNodeWindowsListByBeginStackParent(ImGuiWindow** windows, windows_size: c_int, parent_in_begin_stack: *mut ImGuiWindow)
{
    for (let i: c_int = 0; i < windows_size; i++)
    {
        let mut window: *mut ImGuiWindow =  windows[i];
        if (window.ParentWindowInBeginStack != parent_in_begin_stack)
            continue;
        buf: [c_char;20];
        ImFormatString(buf, buf.len(), "[%04d] Window", window.BeginOrderWithinContext);
        //BulletText("[%04d] Window '%s'", window.BeginOrderWithinContext, window.Name);
        DebugNodeWindow(window, buf);
        Indent();
        DebugNodeWindowsListByBeginStackParent(windows + i + 1, windows_size - i - 1, window);
        Unindent();
    }
}

//-----------------------------------------------------------------------------
// [SECTION] DEBUG LOG
//-----------------------------------------------------------------------------

pub unsafe fn DebugLog(fmt: *const c_char, ...)
{
    va_list args;
    va_start(args, fmt);
    DebugLogV(fmt, args);
    va_end(args);
}

pub unsafe fn DebugLogV(fmt: *const c_char, va_list args)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let old_size: c_int = g.DebugLogBuf.size();
    g.DebugLogBuf.appendf("[%05d] ", g.FrameCount);
    g.DebugLogBuf.appendfv(fmt, args);
    if (g.DebugLogFlags & ImGuiDebugLogFlags_OutputToTTY)
        IMGUI_DEBUG_PRINTF("%s", g.DebugLogBuf.begin() + old_size);
}

pub unsafe fn ShowDebugLogWindow(bool* p_open)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (!(g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasSize))
        SetNextWindowSize(ImVec2::new(0.0, GetFontSize() * 12.0), ImGuiCond_FirstUseEver);
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
pub unsafe fn UpdateDebugToolItemPicker()
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
    SetNextWindowBgAlpha(0.70);
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
pub unsafe fn UpdateDebugToolStackQueries()
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
pub unsafe fn ShowStackToolWindow(bool* p_open)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (!(g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasSize))
        SetNextWindowSize(ImVec2::new(0.0, GetFontSize() * 8.0), ImGuiCond_FirstUseEver);
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
    TextColored((time_since_copy >= 0.0 && time_since_copy < 0.75f && ImFmod(time_since_copy, 0.250f32) < 0.25f * 0.5) ? ImVec4(1.f, 1.f, 0.3f, 1.0) : ImVec4(), "*COPIED*");
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

pub unsafe fn ShowMetricsWindow(bool*) {}
pub unsafe fn ShowFontAtlas(ImFontAtlas*) {}
pub unsafe fn DebugNodeColumns(ImGuiOldColumns*) {}
pub unsafe fn DebugNodeDrawList(ImGuiWindow*, *mut ImGuiViewportP, *const ImDrawList, *const char) {}
pub unsafe fn DebugNodeDrawCmdShowMeshAndBoundingBox(ImDrawList*, *const ImDrawList, *const ImDrawCmd, bool, bool) {}
pub unsafe fn DebugNodeFont(ImFont*) {}
pub unsafe fn DebugNodeStorage(ImGuiStorage*, *const char) {}
pub unsafe fn DebugNodeTabBar(ImGuiTabBar*, *const char) {}
pub unsafe fn DebugNodeWindow(ImGuiWindow*, *const char) {}
pub unsafe fn DebugNodeWindowSettings(ImGuiWindowSettings*) {}
pub unsafe fn DebugNodeWindowsList(Vec<ImGuiWindow*>*, *const char) {}
c_void DebugNodeViewport {}

pub unsafe fn DebugLog(*const char, ...) {}
pub unsafe fn DebugLogV(*const char, va_list) {}
pub unsafe fn ShowDebugLogWindow(bool*) {}
pub unsafe fn ShowStackToolWindow(bool*) {}
pub unsafe fn DebugHookIdInfo(ImGuiID, ImGuiDataType, *const c_void, *const c_void) {}
pub unsafe fn UpdateDebugToolItemPicker() {}
pub unsafe fn UpdateDebugToolStackQueries() {}

// #endif // #ifndef IMGUI_DISABLE_DEBUG_TOOLS

//-----------------------------------------------------------------------------

// Include imgui_user.inl at the end of imgui.cpp to access private data/functions that aren't exposed.
// Prefer just including imgui_internal.h from your code rather than using this define. If a declaration is missing from imgui_internal.h add it or request it on the github.
// #ifdef IMGUI_INCLUDE_IMGUI_USER_INL
// #include "imgui_user.inl"
// #endif

//-----------------------------------------------------------------------------

// #endif // #ifndef IMGUI_DISABLE
