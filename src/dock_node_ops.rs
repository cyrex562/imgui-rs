use std::ptr::{null, null_mut};
use libc::{c_char, c_float, c_int, size_t};
use crate::axis::{ImGuiAxis, ImGuiAxis_X, ImGuiAxis_Y};
use crate::color::{ImGuiCol_DockingEmptyBg, ImGuiCol_DockingPreview, ImGuiCol_NavWindowingHighlight, ImGuiCol_Separator, ImGuiCol_Text, ImGuiCol_TitleBg, ImGuiCol_TitleBgActive, ImGuiCol_TitleBgCollapsed, ImGuiCol_WindowBg};
use crate::condition::ImGuiCond_Always;
use crate::constants::{DOCKING_SPLITTER_SIZE, WINDOWS_HOVER_PADDING, WINDOWS_RESIZE_FROM_EDGES_FEEDBACK_TIMER};
use crate::dock_node::ImGuiDockNode;
use crate::dock_node_flags::{ImGuiDockNodeFlags, ImGuiDockNodeFlags_AutoHideTabBar, ImGuiDockNodeFlags_HiddenTabBar, ImGuiDockNodeFlags_KeepAliveOnly, ImGuiDockNodeFlags_LocalFlagsTransferMask_, ImGuiDockNodeFlags_NoCloseButton, ImGuiDockNodeFlags_NoDocking, ImGuiDockNodeFlags_NoDockingInCentralNode, ImGuiDockNodeFlags_NoDockingOverEmpty, ImGuiDockNodeFlags_NoDockingOverMe, ImGuiDockNodeFlags_NoDockingOverOther, ImGuiDockNodeFlags_NoDockingSplitMe, ImGuiDockNodeFlags_NoDockingSplitOther, ImGuiDockNodeFlags_None, ImGuiDockNodeFlags_NoResize, ImGuiDockNodeFlags_NoResizeX, ImGuiDockNodeFlags_NoResizeY, ImGuiDockNodeFlags_NoSplit, ImGuiDockNodeFlags_NoWindowMenuButton, ImGuiDockNodeFlags_PassthruCentralNode, ImGuiDockNodeFlags_SharedFlagsInheritMask_};
use crate::data_authority::{IM_GUI_DATA_AUTHORITY_AUTO, IM_GUI_DATA_AUTHORITY_DOCK_NODE, IM_GUI_DATA_AUTHORITY_WINDOW};

use crate::direction::{ImGuiDir, ImGuiDir_COUNT, ImGuiDir_Down, ImGuiDir_Left, ImGuiDir_None, ImGuiDir_Right, ImGuiDir_Up};
use crate::dock_context_ops::{DockContextAddNode, DockContextRemoveNode};
use crate::dock_node_state::{ImGuiDockNodeState_HostWindowHiddenBecauseSingleWindow, ImGuiDockNodeState_HostWindowHiddenBecauseWindowsAreResizing};
use crate::dock_node_tree_info::ImGuiDockNodeTreeInfo;
use crate::dock_preview_data::ImGuiDockPreviewData;
use crate::drag_drop_ops::GetDragDropPayload;
use crate::draw_flags::ImDrawFlags;
use crate::draw_list::ImDrawList;
use crate::draw_list_ops::GetForegroundDrawList;
use crate::frame_ops::GetFrameHeight;
use crate::GImGui;
use crate::id_ops::{id_from_str, pop_win_id_from_stack, push_str_id, push_int_id, PushOverrideID};
use crate::a_imgui_cpp::{BeginDockableDragDropTarget, DockSettingsRenameNodeReferences};
use crate::item_ops::{IsItemActive, PopItemFlag, PushItemFlag};
use crate::layout_ops::same_line;
use crate::popup_ops::{BeginPopup, EndPopup, IsPopupOpen, IsPopupOpenWithStrId, OpenPopup};
use crate::rect::ImRect;
use crate::render_ops::{CalcRoundingFlagsForRectInRect, RenderRectFilledWithHole};
use crate::string_ops::str_to_const_c_char_ptr;
use crate::style_ops::{GetColorU32, PopStyleColor, PushStyleColor};
use crate::tab_bar::ImGuiTabBar;
use crate::tab_item_flags::{ImGuiTabItemFlags, ImGuiTabItemFlags_Button, ImGuiTabItemFlags_NoCloseWithMiddleMouseButton, ImGuiTabItemFlags_None, ImGuiTabItemFlags_Preview, ImGuiTabItemFlags_UnsavedDocument, ImGuiTabItemFlags_Unsorted};
use crate::type_defs::{ImguiHandle, INVALID_IMGUI_HANDLE};
use crate::input_ops::IsMouseClicked;
use crate::math_ops::{ImMax, ImMin};
use crate::mouse_ops::{StartMouseMovingWindow, StartMouseMovingWindowOrNode};
use crate::nav_layer::{ImGuiNavLayer_Main, ImGuiNavLayer_Menu};
use crate::nav_ops::{ImGetDirQuadrantFromDelta, NavInitWindow};
use crate::settings_ops::MarkIniSettingsDirty;
use crate::tab_bar_flags::{ImGuiTabBarFlags_AutoSelectNewTabs, ImGuiTabBarFlags_DockNode, ImGuiTabBarFlags_IsFocused, ImGuiTabBarFlags_NoCloseWithMiddleMouseButton, ImGuiTabBarFlags_Reorderable, ImGuiTabBarFlags_SaveSettings};
use crate::tab_item::ImGuiTabItem;
use crate::utils::{flag_clear, flag_set, ImQsort, is_not_null};
use crate::vec2::ImVec2;
use crate::vec4::ImVec4;
use crate::window::find::IsWindowWithinBeginStackOf;
use crate::window::focus::FocusWindow;
use crate::window::ImguiWindow;
use crate::window::ops::{Begin, BringWindowToDisplayFront, End, SetNextWindowSize};
use crate::window::props::{SetNextWindowBgAlpha, SetNextWindowCollapsed, SetNextWindowPos, SetNextWindowViewport, SetWindowHitTestHole, SetWindowPos, SetWindowSize};
use crate::window::rect::{PopClipRect, PushClipRect};
use crate::window::render::{RenderWindowOuterBorders, UpdateWindowParentAndRootLinks};
use crate::window::window_dock_style_color::{ImGuiWindowDockStyleCol_COUNT, ImGuiWindowDockStyleCol_TabActive, ImGuiWindowDockStyleCol_Text};
use crate::window::window_dock_style_colors::GWindowDockStyleColors;
use crate::window::window_flags::{ImGuiWindowFlags, ImGuiWindowFlags_ChildWindow, ImGuiWindowFlags_DockNodeHost, ImGuiWindowFlags_NoCollapse, ImGuiWindowFlags_NoFocusOnAppearing, ImGuiWindowFlags_NoNavFocus, ImGuiWindowFlags_NoSavedSettings, ImGuiWindowFlags_NoScrollbar, ImGuiWindowFlags_NoScrollWithMouse, ImGuiWindowFlags_NoTitleBar, ImGuiWindowFlags_UnsavedDocument};

pub fn DockNodeGetTabOrder(window: &mut ImguiWindow) -> c_int {
    let tab_bar = window.DockNode.TabBar;
    if tab_bar == None {
        return -1;
    }
    let tab = TabBarFindTabByID(tab_bar, window.TabId);
    return if tab { tab_bar.GetTabOrder(tab) } else { -1 };
}

pub unsafe fn DockNodeHideWindowDuringHostWindowCreation(window: &mut ImguiWindow) {
    window.Hidden = true;
    window.HiddenFramesCanSkipItems = if window.Active { 1 } else { 2 };
}

pub unsafe fn DockNodeAddWindow(node: *mut ImGuiDockNode, window: &mut ImguiWindow, add_to_tab_bar: bool) {
    let g = GImGui; // ImGuiContext& g = *GImGui; (void)g;
    if window.DockNode {
        // Can overwrite an existing window.DockNode (e.g. pointing to a disabled DockSpace node)
        // IM_ASSERT(window.DockNode.ID != node.ID);
        DockNodeRemoveWindow(window.DockNode, window, 0);
    }
    // IM_ASSERT(window.DockNode == NULL || window.DockNodeAsHost == NULL);
    // IMGUI_DEBUG_LOG_DOCKING("[docking] DockNodeAddWindow node 0x{} window '{}'\n", node.ID, window.Name);

    // If more than 2 windows appeared on the same frame leading to the creation of a new hosting window,
    // we'll hide windows until the host window is ready. Hide the 1st window after its been output (so it is not visible for one frame).
    // We will call DockNodeHideWindowDuringHostWindowCreation() on ourselves in Begin()
    if node.HostWindow == None && node.Windows.len() == 1 && node.Windows[0].WasActive == false {
        DockNodeHideWindowDuringHostWindowCreation(node.Windows[0]);
    }

    node.Windows.push(window);
    node.WantHiddenTabBarUpdate = true;
    window.DockNode = node;
    window.DockId = node.ID;
    window.DockIsActive = (node.Windows.len() > 1);
    window.DockTabWantClose = false;

    // When reactivating a node with one or two loose window, the window pos/size/viewport are authoritative over the node storage.
    // In particular it is important we init the viewport from the first window so we don't create two viewports and drop one.
    if node.HostWindow == None && node.IsFloatingNode() {
        if node.AuthorityForPos == IM_GUI_DATA_AUTHORITY_AUTO {
            node.AuthorityForPos = IM_GUI_DATA_AUTHORITY_WINDOW;
        }
        if node.AuthorityForSize == IM_GUI_DATA_AUTHORITY_AUTO {
            node.AuthorityForSize = IM_GUI_DATA_AUTHORITY_WINDOW;
        }
        if node.AuthorityForViewport == IM_GUI_DATA_AUTHORITY_AUTO {
            node.AuthorityForViewport = IM_GUI_DATA_AUTHORITY_WINDOW;
        }
    }

    // Add to tab bar if requested
    if add_to_tab_bar {
        if node.TabBar == None {
            DockNodeAddTabBar(node);
            node.TabBar.SelectedTabId = node.SelectedTabId;
            node.TabBar.NextSelectedTabId = node.SelectedTabId;

            // Add existing windows
            // for (let n: c_int = 0; n < node.Windows.len() - 1; n++)
            for n in 0..node.Windows.len() {
                TabBarAddTab(node.TabBar, ImGuiTabItemFlags_None, node.Windows[n]);
            }
        }
        TabBarAddTab(node.TabBar, ImGuiTabItemFlags_Unsorted, window);
    }

    DockNodeUpdateVisibleFlag(node);

    // Update this without waiting for the next time we Begin() in the window, so our host window will have the proper title bar color on its first frame.
    if node.HostWindow {
        UpdateWindowParentAndRootLinks(window, window.Flags | ImGuiWindowFlags_ChildWindow, node.HostWindow);
    }
}

pub unsafe fn DockNodeRemoveWindow(node: *mut ImGuiDockNode, window: &mut ImguiWindow, save_dock_id: ImguiHandle) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(window.DockNode == node);
    //IM_ASSERT(window.RootWindowDockTree == node->HostWindow);
    //IM_ASSERT(window.LastFrameActive < g.FrameCount);    // We may call this from Begin()
    // IM_ASSERT(save_dock_id == 0 || save_dock_id == node.ID);
    // IMGUI_DEBUG_LOG_DOCKING("[docking] DockNodeRemoveWindow node 0x{} window '{}'\n", node.ID, window.Name);

    window.DockNode = None;
    window.DockIsActive = false;
    window.DockTabWantClose = false;
    window.DockId = save_dock_id;
    window.Flags &= !ImGuiWindowFlags_ChildWindow;
    if window.ParentWindow {
        window.Parentwindow.DC.ChildWindows.find_erase(window);
    }
    UpdateWindowParentAndRootLinks(window, window.Flags, null_mut()); // Update immediately

    // Remove window
    let mut erased: bool = false;
    // for (let n: c_int = 0; n < node.Windows.len(); n++)
    for n in 0..node.Windows.len() {
        if node.Windows[n] == window {
            node.Windows.erase(node.Windows.Data + n);
            erased = true;
            break;
        }
    }
    if !erased {}
    // IM_ASSERT(erased);
    if node.VisibleWindow == window {
        node.VisibleWindow = None;
    }

    // Remove tab and possibly tab bar
    node.WantHiddenTabBarUpdate = true;
    if (node.TabBar) {
        TabBarRemoveTab(node.TabBar, window.TabId);
        let tab_count_threshold_for_tab_bar: usize = if node.IsCentralNode() { 1 } else { 2 };
        if node.Windows.len() < tab_count_threshold_for_tab_bar {
            DockNodeRemoveTabBar(node);
        }
    }

    if node.Windows.len() == 0 && !node.IsCentralNode() && !node.IsDockSpace() && window.DockId != node.ID {
        // Automatic dock node delete themselves if they are not holding at least one tab
        DockContextRemoveNode(g, node, true);
        return;
    }

    if node.Windows.len() == 1 && !node.IsCentralNode() && is_not_null(node.HostWindow) {
        let mut remaining_window: &mut ImguiWindow = node.Windows[0];
        if node.Hostwindow.ViewportOwned && node.IsRootNode() {
            // Transfer viewport back to the remaining loose window
            IMGUI_DEBUG_LOG_VIEWPORT("[viewport] Node {} transfer Viewport {}=>{} for Window '{}'\n", node.ID, node.Hostwindow.Viewport.ID, remaining_window.ID, remaining_window.Name);
            // IM_ASSERT(node->Hostwindow.Viewport.Window == node->HostWindow);
            node.Hostwindow.Viewport.Window = remaining_window;
            node.Hostwindow.Viewport.ID = remaining_window.ID;
        }
        remaining_window.Collapsed = node.Hostwindow.Collapsed;
    }

    // Update visibility immediately is required so the DockNodeUpdateRemoveInactiveChilds() processing can reflect changes up the tree
    DockNodeUpdateVisibleFlag(node);
}

pub unsafe fn DockNodeMoveChildNodes(dst_node: *mut ImGuiDockNode, src_node: *mut ImGuiDockNode) {
    // IM_ASSERT(dst_node.Windows.Size == 0);
    dst_node.ChildNodes[0] = src_node.ChildNodes[0];
    dst_node.ChildNodes[1] = src_node.ChildNodes[1];
    if (dst_node.ChildNodes[0]) {
        dst_node.ChildNodes[0].ParentNode = dst_node;
    }
    if (dst_node.ChildNodes[1]) {
        dst_node.ChildNodes[1].ParentNode = dst_node;
    }
    dst_node.SplitAxis = src_node.SplitAxis;
    dst_node.SizeRef = src_node.SizeRef;
    src_node.ChildNodes[0] = None;
    src_node.ChildNodes[1] = None;
}

pub unsafe fn DockNodeMoveWindows(dst_node: *mut ImGuiDockNode, src_node: *mut ImGuiDockNode) {
    // Insert tabs in the same orders as currently ordered (node.Windows isn't ordered)
    // IM_ASSERT(src_node && dst_node && dst_node != src_node);
    crate::tab_bar::ImGuiTabBar * src_tab_bar = src_node.TabBar;
    if src_tab_bar != None {}
    // IM_ASSERT(src_node.Windows.Size <= src_node.TabBar.Tabs.Size);

    // If the dst_node is empty we can just move the entire tab bar (to preserve selection, scrolling, etc.)
    let mut move_tab_bar: bool = (src_tab_bar != null_mut()) && (dst_node.TabBar == null_mut());
    if move_tab_bar {
        dst_node.TabBar = src_node.TabBar;
        src_node.TabBar = None;
    }

    // Tab order is not important here, it is preserved by sorting in DockNodeUpdateTabBar().
    // for (window: &mut ImGuiWindow : src_node.Windows)
    for window in src_node.Windows {
        window.DockNode = None;
        window.DockIsActive = false;
        DockNodeAddWindow(dst_node, window, !move_tab_bar);
    }
    src_node.Windows.clear();

    if !move_tab_bar && is_not_null(src_node.TabBar) {
        if (dst_node.TabBar) {
            dst_node.TabBar.SelectedTabId = src_node.TabBar.SelectedTabId;
        }
        DockNodeRemoveTabBar(src_node);
    }
}

pub unsafe fn DockNodeApplyPosSizeToWindows(node: *mut ImGuiDockNode) {
    // for (let n: c_int = 0; n < node.Windows.len(); n++)
    for n in 0..node.Windows.len() {
        SetWindowPos(node.Windows[n], &node.Pos, ImGuiCond_Always); // We don't assign directly to Pos because it can break the calculation of SizeContents on next frame
        SetWindowSize(node.Windows[n], &node.Size, ImGuiCond_Always);
    }
}

pub unsafe fn DockNodeHideHostWindow(node: *mut ImGuiDockNode) {
    if (node.HostWindow) {
        if (node.Hostwindow.DockNodeAsHost == node) {
            node.Hostwindow.DockNodeAsHost = None;
        }
        node.HostWindow = None;
    }

    if (node.Windows.len() == 1) {
        node.VisibleWindow = node.Windows[0];
        node.Windows[0].DockIsActive = false;
    }

    if (node.TabBar) {
        DockNodeRemoveTabBar(node);
    }
}


pub unsafe fn DockNodeFindInfo(node: *mut ImGuiDockNode, info: *mut ImGuiDockNodeTreeInfo) {
    if (node.Windows.len() > 0) {
        if (info.FirstNodeWithWindows == null_mut()) {
            info.FirstNodeWithWindows = node;
        }
        info.CountNodesWithWindows += 1;
    }
    if (node.IsCentralNode()) {
        // IM_ASSERT(info.CentralNode == NULL); // Should be only one
        // IM_ASSERT(node->IsLeafNode() && "If you get this assert: please submit .ini file + repro of actions leading to this.");
        info.CentralNode = node;
    }
    if (info.CountNodesWithWindows > 1 && info.CentralNode != null_mut()) {
        return;
    }
    if (node.ChildNodes[0]) {
        DockNodeFindInfo(node.ChildNodes[0], info);
    }
    if (node.ChildNodes[1]) {
        DockNodeFindInfo(node.ChildNodes[1], info);
    }
}

pub fn DockNodeFindWindowByID(node: *mut ImGuiDockNode, id: ImguiHandle) -> *mut ImguiWindow {
    // IM_ASSERT(id != 0);
    // for (let n: c_int = 0; n < node.Windows.len(); n++)
    for n in 0..node.Windows.len() {
        if node.Windows[n].ID == id {
            return node.Windows[n];
        }
    }
    return None;
}

// - Remove inactive windows/nodes.
// - Update visibility flag.
pub unsafe fn DockNodeUpdateFlagsAndCollapse(node: *mut ImGuiDockNode) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(node.ParentNode == NULL || node.ParentNode->ChildNodes[0] == node || node.ParentNode->ChildNodes[1] == node);

    // Inherit most flags
    if node.ParentNode {
        node.SharedFlags = node.ParentNode.SharedFlags & ImGuiDockNodeFlags_SharedFlagsInheritMask_;
    }

    // Recurse into children
    // There is the possibility that one of our child becoming empty will delete itself and moving its sibling contents into 'node'.
    // If 'node->ChildNode[0]' delete itself, then 'node->ChildNode[1].Windows' will be moved into 'node'
    // If 'node->ChildNode[1]' delete itself, then 'node->ChildNode[0].Windows' will be moved into 'node' and the "remove inactive windows" loop will have run twice on those windows (harmless)
    node.HasCentralNodeChild = false;
    if (node.ChildNodes[0]) {
        DockNodeUpdateFlagsAndCollapse(node.ChildNodes[0]);
    }
    if (node.ChildNodes[1]) {
        DockNodeUpdateFlagsAndCollapse(node.ChildNodes[1]);
    }

    // Remove inactive windows, collapse nodes
    // Merge node flags overrides stored in windows
    node.LocalFlagsInWindows = ImGuiDockNodeFlags_None;
    // for (let window_n: c_int = 0; window_n < node.Windows.len(); window_n++)
    for mut window_n in 0..node.Windows.len() {
        let mut window: &mut ImguiWindow = node.Windows[window_n];
        // IM_ASSERT(window.DockNode == node);

        let mut node_was_active: bool = (node.LastFrameActive + 1 == g.FrameCount);
        let mut remove: bool = false;
        remove |= node_was_active && (window.LastFrameActive + 1 < g.FrameCount);
        remove |= node_was_active && (node.WantCloseAll || node.WantCloseTabId == window.TabId) && window.HasCloseButton && flag_clear(window.Flags, ImGuiWindowFlags_UnsavedDocument);  // Submit all _expected_ closure from last frame
        remove |= (window.DockTabWantClose);
        if remove {
            window.DockTabWantClose = false;
            if node.Windows.len() == 1 && !node.IsCentralNode() {
                DockNodeHideHostWindow(node);
                node.State = ImGuiDockNodeState_HostWindowHiddenBecauseSingleWindow;
                DockNodeRemoveWindow(node, window, node.ID); // Will delete the node so it'll be invalid on return
                return;
            }
            DockNodeRemoveWindow(node, window, node.ID);
            window_n -= 1;
            continue;
        }

        // FIXME-DOCKING: Missing policies for conflict resolution, hence the "Experimental" tag on this.
        //node.LocalFlagsInWindow &= ~window.WindowClass.DockNodeFlagsOverrideClear;
        node.LocalFlagsInWindows |= window.WindowClass.DockNodeFlagsOverrideSet;
    }
    node.UpdateMergedFlags();

    // Auto-hide tab bar option
    let node_flags = node.MergedFlags;
    if node.WantHiddenTabBarUpdate && node.Windows.len() == 1 && flag_set(node_flags, ImGuiDockNodeFlags_AutoHideTabBar) && !node.IsHiddenTabBar() {
        node.WantHiddenTabBarToggle = true;
    }
    node.WantHiddenTabBarUpdate = false;

    // Cancel toggling if we know our tab bar is enforced to be hidden at all times
    if node.WantHiddenTabBarToggle && is_not_null(node.VisibleWindow) && flag_set(node.VisibleWindow.WindowClass.DockNodeFlagsOverrideSet,
                                                                                  ImGuiDockNodeFlags_HiddenTabBar) {
        node.WantHiddenTabBarToggle = false;
    }

    // Apply toggles at a single point of the frame (here!)
    if node.Windows.len() > 1 {
        node.SetLocalFlags(node.LocalFlags & !ImGuiDockNodeFlags_HiddenTabBar);
    } else if node.WantHiddenTabBarToggle {
        node.SetLocalFlags(node.LocalFlags ^ ImGuiDockNodeFlags_HiddenTabBar);
    }
    node.WantHiddenTabBarToggle = false;

    DockNodeUpdateVisibleFlag(node);
}

// This is rarely called as DockNodeUpdateForRootNode() generally does it most frames.
pub unsafe fn DockNodeUpdateHasCentralNodeChild(node: *mut ImGuiDockNode) {
    node.HasCentralNodeChild = false;
    if (node.ChildNodes[0]) {
        DockNodeUpdateHasCentralNodeChild(node.ChildNodes[0]);
    }
    if (node.ChildNodes[1]) {
        DockNodeUpdateHasCentralNodeChild(node.ChildNodes[1]);
    }
    if (node.IsRootNode()) {
        mark_node: *mut ImGuiDockNode = node.CentralNode;
        while (mark_node) {
            mark_node.HasCentralNodeChild = true;
            mark_node = mark_node.ParentNode;
        }
    }
}

pub unsafe fn DockNodeUpdateVisibleFlag(node: *mut ImGuiDockNode) {
    // Update visibility flag
    let mut is_visible: bool = if node.ParentNode == None { node.IsDockSpace() } else { node.IsCentralNode() };
    is_visible |= (node.Windows.len() > 0);
    is_visible |= (is_not_null(node.ChildNodes[0]) && node.ChildNodes[0].IsVisible);
    is_visible |= (is_not_null(node.ChildNodes[1]) && node.ChildNodes[1].IsVisible);
    node.IsVisible = is_visible;
}

pub unsafe fn DockNodeStartMouseMovingWindow(node: *mut ImGuiDockNode, window: &mut ImguiWindow) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(node->WantMouseMove == true);
    StartMouseMovingWindow(window);
    g.ActiveIdClickOffset = g.IO.MouseClickedPos[0] - node.Pos;
    g.MovingWindow = window; // If we are docked into a non moveable root window, StartMouseMovingWindow() won't set g.MovingWindow. Override that decision.
    node.WantMouseMove = false;
}

// Update CentralNode, OnlyNodeWithWindows, LastFocusedNodeID. Copy window class.
pub unsafe fn DockNodeUpdateForRootNode(node: *mut ImGuiDockNode) {
    DockNodeUpdateFlagsAndCollapse(node);

    // - Setup central node pointers
    // - Find if there's only a single visible window in the hierarchy (in which case we need to display a regular title bar -> FIXME-DOCK: that last part is not done yet!)
    // Cannot merge this with DockNodeUpdateFlagsAndCollapse() because FirstNodeWithWindows is found after window removal and child collapsing
    let mut info = ImGuiDockNodeTreeInfo::default();
    DockNodeFindInfo(node, &mut info);
    node.CentralNode = info.CentralNode;
    node.OnlyNodeWithWindows = if info.CountNodesWithWindows == 1 { info.FirstNodeWithWindows } else { None };
    node.CountNodeWithWindows = info.CountNodesWithWindows;
    if (node.LastFocusedNodeId == 0 && info.FirstNodeWithWindows != null_mut()) {
        node.LastFocusedNodeId = info.FirstNodeWithWindows.ID;
    }

    // Copy the window class from of our first window so it can be used for proper dock filtering.
    // When node has mixed windows, prioritize the class with the most constraint (DockingAllowUnclassed = false) as the reference to copy.
    // FIXME-DOCK: We don't recurse properly, this code could be reworked to work from DockNodeUpdateScanRec.
    if (first_node_with_windows: *mut ImGuiDockNode = info.FirstNodeWithWindows) {
        // node.WindowClass = first_node_with_windows-> std::slice::Windows[0].WindowClass;
        // for (let n: c_int = 1; n < first_node_with_windows.Windows.len(); n++)
        for n in 1..first_node_with_windows.Windows.len() {
            if (first_node_with_windows.Windows[n].WindowClass.DockingAllowUnclassed == false) {
                node.WindowClass = first_node_with_windows.Windows[n].WindowClass;
                break;
            }
        }
    }

    let mut mark_node = node.CentralNode;
    while mark_node {
        mark_node.HasCentralNodeChild = true;
        mark_node = mark_node.ParentNode;
    }
}

pub unsafe fn DockNodeSetupHostWindow(node: *mut ImGuiDockNode, host_window: &mut ImguiWindow) {
    // Remove ourselves from any previous different host window
    // This can happen if a user mistakenly does (see #4295 for details):
    //  - N+0: DockBuilderAddNode(id, 0)    // missing ImGuiDockNodeFlags_DockSpace
    //  - N+1: NewFrame()                   // will create floating host window for that node
    //  - N+1: DockSpace(id)                // requalify node as dockspace, moving host window
    if is_not_null(node.HostWindow) && node.HostWindow != host_window && node.Hostwindow.DockNodeAsHost == node {
        node.Hostwindow.DockNodeAsHost = None;
    }

    host_window.DockNodeAsHost = node;
    node.HostWindow = host_window;
}

pub unsafe fn DockNodeUpdate(node: *mut ImGuiDockNode) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(node->LastFrameActive != g.FrameCount);
    node.LastFrameAlive = g.FrameCount;
    node.IsBgDrawnThisFrame = false;

    node.CentralNode = None;
    node.OnlyNodeWithWindows = None;
    if node.IsRootNode() {
        DockNodeUpdateForRootNode(node);
    }

    // Remove tab bar if not needed
    if is_not_null(node.TabBar) && node.IsNoTabBar() {
        DockNodeRemoveTabBar(node);
    }
    // Early out for hidden root dock nodes (when all DockId references are in inactive windows, or there is only 1 floating window holding on the DockId)
    let mut want_to_hide_host_window: bool = false;
    if node.IsFloatingNode() {
        if node.Windows.len() <= 1 && node.IsLeafNode() {
            if !g.IO.ConfigDockingAlwaysTabBar && (node.Windows.len() == 0 || !node.Windows[0].WindowClass.DockingAlwaysTabBar) {
                want_to_hide_host_window = true;
            }
        }
        if node.CountNodeWithWindows == 0 {
            want_to_hide_host_window = true;
        }
    }
    if want_to_hide_host_window {
        if node.Windows.len() == 1 {
            // Floating window pos/size is authoritative
            let mut single_window: &mut ImguiWindow = node.Windows[0];
            node.Pos = single_window.position;
            node.Size = single_window.SizeFull;
            node.AuthorityForPos = IM_GUI_DATA_AUTHORITY_WINDOW;
            node.AuthorityForSize = IM_GUI_DATA_AUTHORITY_WINDOW;
            node.AuthorityForViewport = IM_GUI_DATA_AUTHORITY_WINDOW;

            // Transfer focus immediately so when we revert to a regular window it is immediately selected
            if is_not_null(node.HostWindow) && g.NavWindow == node.HostWindow {
                FocusWindow(single_window);
            }
            if node.HostWindow {
                single_window.Viewport = node.Hostwindow.Viewport;
                single_window.ViewportId = node.Hostwindow.ViewportId;
                if node.Hostwindow.ViewportOwned {
                    single_window.Viewport.Window = single_window;
                    single_window.ViewportOwned = true;
                }
            }
        }

        DockNodeHideHostWindow(node);
        node.State = ImGuiDockNodeState_HostWindowHiddenBecauseSingleWindow;
        node.WantCloseAll = false;
        node.WantCloseTabId = 0;
        node.HasCloseButton = false;
        node.HasWindowMenuButton = false;
        node.LastFrameActive = g.FrameCount;

        if node.WantMouseMove && node.Windows.len() == 1 {
            DockNodeStartMouseMovingWindow(node, node.Windows[0]);
        }
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
    if node.IsVisible && node.HostWindow == None && node.IsFloatingNode() && node.IsLeafNode() {
        // IM_ASSERT(node.Windows.Size > 0);
        let mut ref_window: &mut ImguiWindow = None;
        if node.SelectedTabId != 0 {// Note that we prune single-window-node settings on .ini loading, so this is generally 0 for them!
            ref_window = DockNodeFindWindowByID(node, node.SelectedTabId);
        }
        if ref_window == None {
            ref_window = node.Windows[0];
        }
        if ref_window.AutoFitFramesX > 0 || ref_window.AutoFitFramesY > 0 {
            node.State = ImGuiDockNodeState_HostWindowHiddenBecauseWindowsAreResizing;
            return;
        }
    }

    let node_flags = node.MergedFlags;

    // Decide if the node will have a close button and a window menu button
    node.HasWindowMenuButton = (node.Windows.len() > 0) && flag_clear(node_flags, ImGuiDockNodeFlags_NoWindowMenuButton);
    node.HasCloseButton = false;
    // for (let window_n: c_int = 0; window_n < node.Windows.len(); window_n++)
    for window_n in 0..node.Windows.len() {
        // FIXME-DOCK: Setting DockIsActive here means that for single active window in a leaf node, DockIsActive will be cleared until the next Begin() call.
        let mut window: &mut ImguiWindow = node.Windows[window_n];
        node.HasCloseButton |= window.HasCloseButton;
        window.DockIsActive = (node.Windows.len() > 1);
    }
    if node_flags & ImGuiDockNodeFlags_NoCloseButton {
        node.HasCloseButton = false;
    }

    // Bind or create host window
    let mut host_window: &mut ImguiWindow = None;
    let mut beginned_into_host_window: bool = false;
    if node.IsDockSpace() {
        // [Explicit root dockspace node]
        // IM_ASSERT(node->HostWindow);
        host_window = node.HostWindow;
    } else {
        // [Automatic root or child nodes]
        if node.IsRootNode() && node.IsVisible {
            let mut ref_window: &mut ImguiWindow = if (node.Windows.len() > 0) { node.Windows[0] } else { None };

            // Sync Pos
            if node.AuthorityForPos == IM_GUI_DATA_AUTHORITY_WINDOW && is_not_null(ref_window) {
                SetNextWindowPos(, &ref_window.position, 0, &Default::default());
            } else if node.AuthorityForPos == IM_GUI_DATA_AUTHORITY_DOCK_NODE {
                SetNextWindowPos(, &node.Pos, 0, &Default::default());
            }

            // Sync Size
            if node.AuthorityForSize == IM_GUI_DATA_AUTHORITY_WINDOW && ref_window.is_null() == false {
                SetNextWindowSize(&ref_window.SizeFull, 0);
            } else if node.AuthorityForSize == IM_GUI_DATA_AUTHORITY_DOCK_NODE {
                SetNextWindowSize(&node.Size, 0);
            }

            // Sync Collapsed
            if node.AuthorityForSize == IM_GUI_DATA_AUTHORITY_WINDOW && ref_window.is_null() == false {
                SetNextWindowCollapsed(ref_window.Collapsed, 0);
            }

            // Sync Viewport
            if node.AuthorityForViewport == IM_GUI_DATA_AUTHORITY_WINDOW && ref_window.is_null() == false {
                SetNextWindowViewport(ref_window.ViewportId);
            }

            crate::window::window_class::SetNextWindowClass(&node.WindowClass);

            // Begin into the host window
            window_label: [c_char; 20];
            DockNodeGetHostWindowTitle(node, window_label, window_label.len());
            window_flags: ImGuiWindowFlags = ImGuiWindowFlags_NoScrollbar | ImGuiWindowFlags_NoScrollWithMouse | ImGuiWindowFlags_DockNodeHost;
            window_flags |= ImGuiWindowFlags_NoFocusOnAppearing;
            window_flags |= ImGuiWindowFlags_NoSavedSettings | ImGuiWindowFlags_NoNavFocus | ImGuiWindowFlags_NoCollapse;
            window_flags |= ImGuiWindowFlags_NoTitleBar;

            SetNextWindowBgAlpha(0.0); // Don't set ImGuiWindowFlags_NoBackground because it disables borders
            PushStyleVar(crate::style_var::ImGuiStyleVar_WindowPadding, ImVec2::from_floats(0.0, 0.0));
            Begin(g, window_label, null_mut());
            PopStyleVar();
            beginned_into_host_window = true;

            host_window = g.CurrentWindow;
            DockNodeSetupHostWindow(node, host_window);
            host_window.dc.cursor_pos = host_window.position;
            node.Pos = host_window.position;
            node.Size = host_window.Size;

            // We set ImGuiWindowFlags_NoFocusOnAppearing because we don't want the host window to take full focus (e.g. steal NavWindow)
            // But we still it bring it to the front of display. There's no way to choose this precise behavior via window flags.
            // One simple case to ponder if: window A has a toggle to create windows B/C/D. Dock B/C/D together, clear the toggle and enable it again.
            // When reappearing B/C/D will request focus and be moved to the top of the display pile, but they are not linked to the dock host window
            // during the frame they appear. The dock host window would keep its old display order, and the sorting in EndFrame would move B/C/D back
            // after the dock host window, losing their top-most status.
            if node.Hostwindow.Appearing {
                BringWindowToDisplayFront(node.HostWindow);
            }

            node.AuthorityForPos = IM_GUI_DATA_AUTHORITY_AUTO;
            node.AuthorityForSize = IM_GUI_DATA_AUTHORITY_AUTO;
            node.AuthorityForViewport = IM_GUI_DATA_AUTHORITY_AUTO;
        } else if node.ParentNode {
            node.HostWindow = node.ParentNode.HostWindow;
            host_window = node.ParentNode.HostWindow;
            node.AuthorityForPos = mGuiDataAuthority_Auto;
            node.AuthorityForSize = mGuiDataAuthority_Auto;
            node.AuthorityForViewport = IM_GUI_DATA_AUTHORITY_AUTO;
        }
        if node.WantMouseMove && node.HostWindow.is_null() == false {
            DockNodeStartMouseMovingWindow(node, node.HostWindow);
        }
    }

    // Update focused node (the one whose title bar is highlight) within a node tree
    if (node.IsSplitNode()) {}
    // IM_ASSERT(node.TabBar == NULL);
    if (node.IsRootNode()) {
        if g.NavWindow && g.NavWindow.Rootwindow.DockNode && g.NavWindow.Rootwindow.ParentWindow == host_window {
            node.LastFocusedNodeId = g.NavWindow.Rootwindow.DockNode.ID;
        }
    }

    // Register a hit-test hole in the window unless we are currently dragging a window that is compatible with our dockspace
    central_node: *mut ImGuiDockNode = node.CentralNode;
    let central_node_hole: bool = node.IsRootNode() && host_window.is_null() == false && (node_flags & ImGuiDockNodeFlags_PassthruCentralNode) != 0 && central_node != None && central_node.IsEmpty();
    let mut central_node_hole_register_hit_test_hole: bool = central_node_hole;
    if central_node_hole {
        let mut payload = GetDragDropPayload();
        if payload.is_null() == false {
            if payload.IsDataType(IMGUI_PAYLOAD_TYPE_WINDOW) && DockNodeIsDropAllowed(host_window, payload.Data) {
                central_node_hole_register_hit_test_hole = false;
            }
        }
    }
    if central_node_hole_register_hit_test_hole {
        // We add a little padding to match the "resize from edges" behavior and allow grabbing the splitter easily.
        // (But we only add it if there's something else on the other side of the hole, otherwise for e.g. fullscreen
        // covering passthru node we'd have a gap on the edge not covered by the hole)
        // IM_ASSERT(node.IsDockSpace()); // We cannot pass this flag without the DockSpace() api. Testing this because we also setup the hole in host_window.ParentNode
        let root_node = DockNodeGetRootNode(central_node);
        let mut root_rect = ImRect::new(root_node.Pos, root_node.Pos + root_node.Size);
        let mut hole_rect = ImRect::new(central_node.Pos, central_node.Pos + central_node.Size);
        if hole_rect.Min.x > root_rect.Min.x { hole_rect.Min.x += WINDOWS_HOVER_PADDING; }
        if hole_rect.Max.x < root_rect.Max.x { hole_rect.Max.x -= WINDOWS_HOVER_PADDING; }
        if hole_rect.Min.y > root_rect.Min.y { hole_rect.Min.y += WINDOWS_HOVER_PADDING; }
        if hole_rect.Max.y < root_rect.Max.y { hole_rect.Max.y -= WINDOWS_HOVER_PADDING; }
        //GetForegroundDrawList().AddRect(hole_rect.Min, hole_rect.Max, IM_COL32(255, 0, 0, 255));
        if central_node_hole && !hole_rect.IsInverted() {
            SetWindowHitTestHole(host_window, hole_rect.Min, hole_rect.Max - hole_rect.Min);
            if host_window.ParentWindow {
                SetWindowHitTestHole(host_window.ParentWindow, hole_rect.Min, hole_rect.Max - hole_rect.Min);
            }
        }
    }

    // Update position/size, process and draw resizing splitters
    if node.IsRootNode() && host_window.is_null() == false {
        DockNodeTreeUpdatePosSize(node, host_window.position, host_window.Size, null_mut());
        DockNodeTreeUpdateSplitter(node);
    }

    // Draw empty node background (currently can only be the Central Node)
    if host_window.is_null() == false && node.IsEmpty() && node.IsVisible {
        host_window.DrawList.ChannelsSetCurrent(DOCKING_HOST_DRAW_CHANNEL_BG);
        node.LastBgColor = if node_flags & ImGuiDockNodeFlags_PassthruCentralNode { 0 } else { GetColorU32(ImGuiCol_DockingEmptyBg, 0.0) };
        if node.LastBgColor != 0 {
            host_window.DrawList.AddRectFilled(&node.Pos, node.Pos + node.Size, node.LastBgColor, 0.0, 0);
        }
        node.IsBgDrawnThisFrame = true;
    }

    // Draw whole dockspace background if ImGuiDockNodeFlags_PassthruCentralNode if set.
    // We need to draw a background at the root level if requested by ImGuiDockNodeFlags_PassthruCentralNode, but we will only know the correct pos/size
    // _after_ processing the resizing splitters. So we are using the DrawList channel splitting facility to submit drawing primitives out of order!
    let render_dockspace_bg: bool = node.IsRootNode() && host_window.is_null() == false && (node_flags & ImGuiDockNodeFlags_PassthruCentralNode) != 0;
    if render_dockspace_bg && node.IsVisible {
        host_window.DrawList.ChannelsSetCurrent(DOCKING_HOST_DRAW_CHANNEL_BG);
        if central_node_hole {
            RenderRectFilledWithHole(host_window.DrawList, node.Rect(), central_node.Rect(), GetColorU32(ImGuiCol_WindowBg, 0.0), 0.0);
        } else {
            host_window.DrawList.AddRectFilled(&node.Pos, node.Pos + node.Size, GetColorU32(ImGuiCol_WindowBg, 0.0), 0.0, 0);
        }
    }

    // Draw and populate Tab Bar
    if host_window {
        host_window.DrawList.ChannelsSetCurrent(DOCKING_HOST_DRAW_CHANNEL_FG);
    }
    if host_window.is_null() == false && node.Windows.len() > 0 {
        DockNodeUpdateTabBar(node, host_window);
    } else {
        node.WantCloseAll = false;
        node.WantCloseTabId = 0;
        node.IsFocused = false;
    }
    if node.TabBar.is_null() == false && node.TabBar.SelectedTabId != 0 {
        node.SelectedTabId = node.TabBar.SelectedTabId;
    } else if node.Windows.len() > 0 {
        node.SelectedTabId = node.Windows[0].TabId;
    }

    // Draw payload drop target
    if host_window.is_null() == false && node.IsVisible {
        if node.IsRootNode() && (g.MovingWindow == None || g.MovingWindow.RootWindowDockTree != host_window) {
            BeginDockableDragDropTarget(host_window);
        }
    }

    // We update this after DockNodeUpdateTabBar()
    node.LastFrameActive = g.FrameCount;

    // Recurse into children
    // FIXME-DOCK FIXME-OPT: Should not need to recurse into children
    if host_window {
        if node.ChildNodes[0] {
            DockNodeUpdate(node.ChildNodes[0]);
        }
        if node.ChildNodes[1] {
            DockNodeUpdate(node.ChildNodes[1]);
        }

        // Render outer borders last (after the tab bar)
        if node.IsRootNode() {
            RenderWindowOuterBorders(host_window);
        }
    }

    // End host window
    if beginned_into_host_window {//-V1020
        End();
    }
}

// Compare TabItem nodes given the last known DockOrder (will persist in .ini file as hint), used to sort tabs when multiple tabs are added on the same frame.
pub unsafe fn TabItemComparerByDockOrder(lhs: *const ImGuiTabItem, rhs: *const ImGuiTabItem) -> i32 {
    let mut a: *mut ImguiWindow = lhs.Window;
    let mut b: *mut ImguiWindow = rhs.Window;
    let d: i32 = (if a.DockOrder == -1 { i32::MAX } else { a.DockOrder }) - (if b.DockOrder == -1 { i32::MAX } else { b.DockOrder });
    if d {
        return d;
    }
    return a.BeginOrderWithinContext - b.BeginOrderWithinContext;
}

// static DockNodeUpdateWindowMenu: ImguiHandle:*mut ImGuiDockNode, ImGuiTabBar* tab_bar)
pub unsafe fn DockNodeUpdateWindowMenu(node: *mut ImGuiDockNode, tab_bar: *mut ImGuiTabBar) -> ImguiHandle {
    // Try to position the menu so it is more likely to stays within the same viewport
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut ret_tab_id: ImguiHandle = 0;
    if g.style.WindowMenuButtonPosition == ImGuiDir_Left {
        SetNextWindowPos(, &ImVec2::from_floats(node.Pos.x, &node.Pos.y + GetFrameHeight()), ImGuiCond_Always, &ImVec2::from_floats(0.0, 0.0));
    } else {
        SetNextWindowPos(, &ImVec2::from_floats(node.Pos.x + node.Size.x, node.Pos.y + GetFrameHeight()), ImGuiCond_Always, &ImVec2::from_floats(1.0, 0.0));
    }
    if BeginPopup(str_to_const_c_char_ptr("#WindowMenu"), 0) {
        node.IsFocused = true;
        if tab_bar.Tabs.len() == 1 {
            if MenuItem("Hide tab bar", None, node.IsHiddenTabBar()) {
                node.WantHiddenTabBarToggle = true;
            }
        } else {
            // for (let tab_n: c_int = 0; tab_n < tab_bar.Tabs.Size; tab_n++)
            for tab_n in 0..tab_bar.Tabs.len() {
                ImGuiTabItem * tab = &tab_bar.Tabs[tab_n];
                if tab.Flags & ImGuiTabItemFlags_Button {
                    continue;
                }
                if Selectable(tab_bar.GetTabNametab, tab.ID == tab_bar.SelectedTabId) {
                    ret_tab_id = tab.ID;
                }
                same_line(g, 0.0, 0.0);
                Text("   ");
            }
        }
        EndPopup(g);
    }
    return ret_tab_id;
}

// User helper to append/amend into a dock node tab bar. Most commonly used to add e.g. a "+" button.
pub unsafe fn DockNodeBeginAmendTabBar(node: *mut ImGuiDockNode) -> bool {
    if node.TabBar == None || node.HostWindow == None {
        return false;
    }
    if node.MergedFlags & ImGuiDockNodeFlags_KeepAliveOnly {
        return false;
    }
    Begin(g, node.Hostwindow.Name, null_mut());
    PushOverrideID(g, node.ID);
    let mut ret: bool = BeginTabBarEx(node.TabBar, node.TabBar.BarRect, node.TabBar.Flags, node);
    IM_UNUSED(ret);
    // IM_ASSERT(ret);
    return true;
}

pub unsafe fn DockNodeEndAmendTabBar() {
    EndTabBar();
    pop_win_id_from_stack(g);
    End();
}

pub unsafe fn IsDockNodeTitleBarHighlighted(node: *mut ImGuiDockNode, root_node: *mut ImGuiDockNode, host_window: &mut ImguiWindow) -> bool {
    // CTRL+Tab highlight (only highlighting leaf node, not whole hierarchy)
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if g.NavWindowingTarget {
        return g.NavWindowingTarget.DockNode == node;
    }

    // FIXME-DOCKING: May want alternative to treat central node void differently? e.g. if (g.NavWindow == host_window)
    if g.NavWindow.is_null() == false && g.NavWindow.RootWindowForTitleBarHighlight == host_window.RootWindowDockTree && root_node.LastFocusedNodeId == node.ID {
        // for (parent_node: * mut ImGuiDockNode = g.NavWindow.Rootwindow.DockNode; parent_node != None; parent_node = parent_node.HostWindow ? parent_node.Hostwindow.Rootwindow.DockNode : null_mut())
        let mut parent_node = g.NavWindow.RootWindowForNav.DockNode;
        while parent_node != None {
            let parent_node = DockNodeGetRootNode(parent_node);
            if parent_node == root_node {
                return true;
            }
            parent_node = if parent_node.HostWindow.is_null() == false { parent_node.HostWindow.RootWindow.DockNode } else { None };
        }
    }
    return false;
}

// Submit the tab bar corresponding to a dock node and various housekeeping details.
pub unsafe fn DockNodeUpdateTabBar(node: *mut ImGuiDockNode, host_window: &mut ImguiWindow) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let style = &mut g.style;

    let node_was_active: bool = (node.LastFrameActive + 1 == g.FrameCount);
    let closed_all: bool = node.WantCloseAll && node_was_active;
    let mut closed_one = node.WantCloseTabId != 0 && node_was_active;
    node.WantCloseAll = false;
    node.WantCloseTabId = 0;

    // Decide if we should use a focused title bar color
    let mut is_focused: bool = false;
    root_node: *mut ImGuiDockNode = DockNodeGetRootNode(node);
    if IsDockNodeTitleBarHighlighted(node, root_node, host_window) {
        is_focused = true;
    }

    // Hidden tab bar will show a triangle on the upper-left (in Begin)
    if node.IsHiddenTabBar() || node.IsNoTabBar() {
        node.VisibleWindow = if node.Windows.len() > 0 { node.Windows[0] } else { None };
        node.IsFocused = is_focused;
        if is_focused {
            node.LastFrameFocused = g.FrameCount;
        }
        if node.VisibleWindow {
            // Notify root of visible window (used to display title in OS task bar)
            if is_focused || root_node.VisibleWindow == None {
                root_node.VisibleWindow = node.VisibleWindow;
            }
            if node.TabBar {
                node.TabBar.VisibleTabId = node.Visiblewindow.TabId;
            }
        }
        return;
    }

    // Move ourselves to the Menu layer (so we can be accessed by tapping Alt) + undo SkipItems flag in order to draw over the title bar even if the window is collapsed
    let mut backup_skip_item: bool = host_window.skip_items;
    if !node.IsDockSpace() {
        host_window.skip_items = false;
        host_window.dc.NavLayerCurrent = ImGuiNavLayer_Menu;
    }

    // Use PushOverrideID() instead of PushID() to use the node id _without_ the host window ID.
    // This is to facilitate computing those ID from the outside, and will affect more or less only the ID of the collapse button, popup and tabs,
    // as docked windows themselves will override the stack with their own root ID.
    PushOverrideID(g, node.ID);
    crate::tab_bar::ImGuiTabBar * tab_bar = node.TabBar;
    let mut tab_bar_is_recreated: bool = (tab_bar == null_mut()); // Tab bar are automatically destroyed when a node gets hidden
    if tab_bar == None {
        DockNodeAddTabBar(node);
        tab_bar = node.TabBar;
    }

    let mut focus_tab_id: ImguiHandle = 0;
    node.IsFocused = is_focused;

    let node_flags = node.MergedFlags;
    let has_window_menu_button: bool = (node_flags & ImGuiDockNodeFlags_NoWindowMenuButton) == 0 && (style.WindowMenuButtonPosition != ImGuiDir_None);

    // In a dock node, the Collapse Button turns into the Window Menu button.
    // FIXME-DOCK FIXME-OPT: Could we recycle popups id across multiple dock nodes?
    if has_window_menu_button && IsPopupOpenWithStrId(, str_to_const_c_char_ptr("#WindowMenu"), 0) {
        let mut tab_id: ImguiHandle = DockNodeUpdateWindowMenu(node, tab_bar);
        if tab_id != INVALID_IMGUI_HANDLE {
            focus_tab_id = tab_id;
            tab_bar.NextSelectedTabId = tab_id;
        }
        is_focused |= node.IsFocused;
    }

    // Layout
    // ImRect
    // title_bar_rect, tab_bar_rect;
    let mut title_bar_rect = ImRect::default();
    let mut tab_bar_rect = ImRect::default();
    let mut window_menu_button_pos = ImVec2::default();
    let mut close_button_pos = ImVec2::default();
    DockNodeCalcTabBarLayout(node, &mut title_bar_rect, &mut tab_bar_rect, &mut window_menu_button_pos, &mut close_button_pos);

    // Submit new tabs, they will be added as Unsorted and sorted below based on relative DockOrder value.
    let tabs_count_old = tab_bar.Tabs.len();
    // for (let window_n: c_int = 0; window_n < node.Windows.len(); window_n++)
    for n in 0..node.Windows.len() {
        let mut window: &mut ImguiWindow = node.Windows[window_n];
        if TabBarFindTabByID(tab_bar, window.TabId) == None {
            TabBarAddTab(tab_bar, ImGuiTabItemFlags_Unsorted, window);
        }
    }

    // Title bar
    if (is_focused) {
        node.LastFrameFocused = g.FrameCount;
    }
    let title_bar_col: u32 = GetColorU32(if host_window.Collapsed { ImGuiCol_TitleBgCollapsed } else {
        if is_focused {
            ImGuiCol_TitleBgActive
        } else { ImGuiCol_TitleBg }
    }, 0.0);
    rounding_flags: ImDrawFlags = CalcRoundingFlagsForRectInRect(&title_bar_rect, host_window.Rect(), DOCKING_SPLITTER_SIZE);
    host_window.DrawList.AddRectFilled(&title_bar_rect.min, &title_bar_rect.max, title_bar_col, host_window.WindowRounding, rounding_flags);

    // Docking/Collapse button
    if has_window_menu_button {
        if CollapseButton(host_window.id_by_string(null(), str_to_const_c_char_ptr("#COLLAPSE")), window_menu_button_pos, node) { // == DockNodeGetWindowMenuButtonId(node)
            OpenPopup(str_to_const_c_char_ptr("#WindowMenu"), 0);
        }
        if IsItemActive() {
            focus_tab_id = tab_bar.SelectedTabId;
        }
    }

    // If multiple tabs are appearing on the same frame, sort them based on their persistent DockOrder value
    let mut tabs_unsorted_start: c_int = tab_bar.Tabs.Size;
    // for (let tab_n: c_int = tab_bar.Tabs.Size - 1; tab_n >= 0 && (tab_bar.Tabs[tab_n].Flags & ImGuiTabItemFlags_Unsorted); tab_n--)
    let mut tab_n = tab_bar.Tabs.len();
    while tab_n >= 0 && tab_bar.Tabs[tab_n].Flags & ImGuiTabItemFlags_Unsorted {
        // FIXME-DOCK: Consider only clearing the flag after the tab has been alive for a few consecutive frames, allowing late comers to not break sorting?
        tab_bar.Tabs[tab_n].Flags &= !ImGuiTabItemFlags_Unsorted;
        tabs_unsorted_start = tab_n;
        tab_n -= 1;
    }
    if tab_bar.Tabs.Size > tabs_unsorted_start {
        // IMGUI_DEBUG_LOG_DOCKING("[docking] In node 0x{}: {} new appearing tabs:{}\n", node.ID, tab_bar.Tabs.Size - tabs_unsorted_start, (tab_bar.Tabs.Size > tabs_unsorted_start + 1) ? " (will sort)" : "");
        // for (let tab_n: c_int = tabs_unsorted_start; tab_n < tab_bar.Tabs.Size; tab_n++)
        for tab_n in tabs_unsorted_start..tab_bar.Tabs.len() {
            // IMGUI_DEBUG_LOG_DOCKING("[docking] - Tab '{}' Order {}\n", tab_bar.Tabs[tab_n].window.Name, tab_bar.Tabs[tab_n].window.DockOrder);
        }
        if tab_bar.Tabs.Size > tabs_unsorted_start + 1 {
            ImQsort(tab_bar.Tabs.Data + tabs_unsorted_start, tab_bar.Tabs.Size - tabs_unsorted_start, sizeof(ImGuiTabItem), TabItemComparerByDockOrder);
        }
    }

    // Apply NavWindow focus back to the tab bar
    if g.NavWindow.is_null() == false && g.NavWindow.Rootwindow.DockNode == node {
        tab_bar.SelectedTabId = g.NavWindow.Rootwindow.TabId;
    }

    // Selected newly added tabs, or persistent tab ID if the tab bar was just recreated
    if tab_bar_is_recreated && TabBarFindTabByID(tab_bar, node.SelectedTabId) != None {
        tab_bar.SelectedTabId = tab_bar.NextSelectedTabId = node.SelectedTabId;
    } else if tab_bar.Tabs.Size > tabs_count_old {
        tab_bar.SelectedTabId = tab_bar.NextSelectedTabId = tab_bar.Tabs.last().unwrap().window.TabId;
    }

    // Begin tab bar
    let mut tab_bar_flags = ImGuiTabBarFlags_Reorderable | ImGuiTabBarFlags_AutoSelectNewTabs; // | ImGuiTabBarFlags_NoTabListScrollingButtons);
    tab_bar_flags |= ImGuiTabBarFlags_SaveSettings | ImGuiTabBarFlags_DockNode;
    if !host_window.Collapsed && is_focused {
        tab_bar_flags |= ImGuiTabBarFlags_IsFocused;
    }
    BeginTabBarEx(tab_bar, tab_bar_rect, tab_bar_flags, node);
    //host_window.DrawList.AddRect(tab_bar_rect.Min, tab_bar_rect.Max, IM_COL32(255,0,255,255));

    // Backup style colors
    let mut backup_style_cols: [ImVec4; ImGuiWindowDockStyleCol_COUNT as usize] = [ImVec4::default(); ImGuiWindowDockStyleCol_COUNT];
    // for (let color_n: c_int = 0; color_n < ImGuiWindowDockStyleCol_COUNT; color_n++)
    for color_n in 0..ImGuiWindowDockStyleCol_COUNT {
        backup_style_cols[color_n] = g.style.Colors[GWindowDockStyleColors[color_n]];
    }

    // Submit actual tabs
    node.VisibleWindow = None;
    // for (let window_n: c_int = 0; window_n < node.Windows.len(); window_n++)
    for window_n in 0..node.Windows.len() {
        let mut window: &mut ImguiWindow = node.Windows[window_n];
        if ((closed_all || closed_one) == (window.TabId != INVALID_IMGUI_HANDLE)) && window.HasCloseButton && flag_clear(window.Flags, ImGuiWindowFlags_UnsavedDocument) {
            continue;
        }
        if window.LastFrameActive + 1 >= g.FrameCount || !node_was_active {
            // ImGuiTabItemFlags
            // tab_item_flags = 0;
            let mut tab_item_flags = ImGuiTabItemFlags_None;
            tab_item_flags |= window.WindowClass.TabItemFlagsOverrideSet;
            if window.Flags & ImGuiWindowFlags_UnsavedDocument {
                tab_item_flags |= ImGuiTabItemFlags_UnsavedDocument;
            }
            if tab_bar.Flags & ImGuiTabBarFlags_NoCloseWithMiddleMouseButton {
                tab_item_flags |= ImGuiTabItemFlags_NoCloseWithMiddleMouseButton;
            }

            // Apply stored style overrides for the window
            // for (let color_n: c_int = 0; color_n < ImGuiWindowDockStyleCol_COUNT; color_n++)
            for color_n in 0..ImGuiWindowDockStyleCol_COUNT {
                g.style.Colors[GWindowDockStyleColors[color_n]] = crate::color_ops::ColorConvertU32ToFloat4(window.DockStyle.Colors[color_n]);
            }

            // Note that TabItemEx() calls TabBarCalcTabID() so our tab item ID will ignore the current ID stack (rightly so)
            let mut tab_open: bool = true;
            TabItemEx(tab_bar, window.Name, if window.HasCloseButton { &tab_open } else { None }, tab_item_flags, window);
            if !tab_open {
                node.WantCloseTabId = window.TabId;
            }
            if tab_bar.VisibleTabId == window.TabId {
                node.VisibleWindow = window;
            }

            // Store last item data so it can be queried with IsItemXXX functions after the user Begin() call
            window.DockTabItemStatusFlags = g.last_item_data.StatusFlags;
            window.DockTabItemRect = g.last_item_data.Rect;

            // Update navigation ID on menu layer
            if g.NavWindow.is_null() == false && g.NavWindow.RootWindow == window && (window.dc.NavLayersActiveMask & (1 << ImGuiNavLayer_Menu)) == 0 {
                host_window.NavLastIds[1] = window.TabId;
            }
        }
    }

    // Restore style colors
    // for (let color_n: c_int = 0; color_n < ImGuiWindowDockStyleCol_COUNT; color_n++)
    for color_n in 0..ImGuiWindowDockStyleCol_COUNT {
        g.style.Colors[GWindowDockStyleColors[color_n]] = backup_style_cols[color_n];
    }

    // Notify root of visible window (used to display title in OS task bar)
    if node.VisibleWindow {
        if is_focused || root_node.VisibleWindow == None {
            root_node.VisibleWindow = node.VisibleWindow;
        }
    }

    // Close button (after VisibleWindow was updated)
    // Note that VisibleWindow may have been overrided by CTRL+Tabbing, so Visiblewindow.TabId may be != from tab_bar->SelectedTabId
    let close_button_is_enabled: bool = node.HasCloseButton && node.VisibleWindow.is_null() == false && node.Visiblewindow.HasCloseButton;
    let close_button_is_visible: bool = node.HasCloseButton;
    //let close_button_is_visible: bool = close_button_is_enabled; // Most people would expect this behavior of not even showing the button (leaving a hole since we can't claim that space as other windows in the tba bar have one)
    if close_button_is_visible {
        if !close_button_is_enabled {
            PushItemFlag(crate::item_flags::ImGuiItemFlags_Disabled, true);
            PushStyleColor(ImGuiCol_Text, style.Colors[ImGuiCol_Text] * ImVec4(1.0, 1.0, 1.0, 0.40f32));
        }
        if CloseButton(host_window.id_by_string(null(), str_to_const_c_char_ptr("#CLOSE")), close_button_pos) {
            node.WantCloseAll = true;
            // for (let n: c_int = 0; n < tab_bar.Tabs.Size; n++)
            for n in 0..tab_bar.Tabs.len() {
                TabBarCloseTab(tab_bar, &tab_bar.Tabs[n]);
            }
        }
        //if (IsItemActive())
        //    focus_tab_id = tab_bar->SelectedTabId;
        if !close_button_is_enabled {
            PopStyleColor(0);
            PopItemFlag();
        }
    }

    // When clicking on the title bar outside of tabs, we still focus the selected tab for that node
    // FIXME: TabItem use AllowItemOverlap so we manually perform a more specific test for now (hovered || held)
    let mut title_bar_id: ImguiHandle = host_window.id_by_string(null(), str_to_const_c_char_ptr("#TITLEBAR"));
    if g.HoveredId == 0 || g.HoveredId == title_bar_id || g.ActiveId == title_bar_id {
        held: bool;
        ButtonBehavior(title_bar_rect, title_bar_id, None, &held, crate::button_flags::ImGuiButtonFlags_AllowItemOverlap);
        if g.HoveredId == title_bar_id {
            // ImGuiButtonFlags_AllowItemOverlap + SetItemAllowOverlap() required for appending into dock node tab bar,
            // otherwise dragging window will steal HoveredId and amended tabs cannot get them.
            g.last_item_data.ID = title_bar_id;
            SetItemAllowOverlap();
        }
        if held {
            if IsMouseClicked(0, false) {
                focus_tab_id = tab_bar.SelectedTabId;
            }

            // Forward moving request to selected window
            let tab = TabBarFindTabByID(tab_bar, tab_bar.SelectedTabId);
            if tab.is_null() == false {
                StartMouseMovingWindowOrNode(if tab.Window { tab.Window } else { node.HostWindow }, node, false);
            }
        }
    }

    // Forward focus from host node to selected window
    //if (is_focused && g.NavWindow == host_window && !g.NavWindowingTarget)
    //    focus_tab_id = tab_bar->SelectedTabId;

    // When clicked on a tab we requested focus to the docked child
    // This overrides the value set by "forward focus from host node to selected window".
    if tab_bar.NextSelectedTabId {
        focus_tab_id = tab_bar.NextSelectedTabId;
    }

    // Apply navigation focus
    if focus_tab_id != 0 {
        let tab = TabBarFindTabByID(tab_bar, focus_tab_id);
        if tab.is_null() == false {
            if tab.Window {
                FocusWindow(tab.Window);
                NavInitWindow(tab.Window, false);
            }
        }
    }

    EndTabBar();
    pop_win_id_from_stack(g);

    // Restore SkipItems flag
    if !node.IsDockSpace() {
        host_window.dc.NavLayerCurrent = ImGuiNavLayer_Main;
        host_window.skip_items = backup_skip_item;
    }
}

pub unsafe fn DockNodeAddTabBar(node: *mut ImGuiDockNode) {
    // IM_ASSERT(node.TabBar == NULL);
    node.TabBar = IM_NEW(crate::tab_bar::ImGuiTabBar);
}

pub unsafe fn DockNodeRemoveTabBar(node: *mut ImGuiDockNode) {
    if node.TabBar == None {
        return;
    }
    IM_DELETE(node.TabBar);
    node.TabBar = None;
}

pub unsafe fn DockNodeIsDropAllowedOne(payload: *mut ImguiWindow, host_window: &mut ImguiWindow) -> bool {
    if host_window.DockNodeAsHost.is_null() == false && host_window.DockNodeAsHost.IsDockSpace() && payload.BeginOrderWithinContext < host_window.BeginOrderWithinContext {
        return false;
    }

    crate::window::window_class::ImGuiWindowClass * host_class = if host_window.DockNodeAsHost { &host_window.DockNodeAsHost.WindowClass } else { &host_window.WindowClass };
    crate::window::window_class::ImGuiWindowClass * payload_class = &payload.WindowClass;
    if host_class.ClassId != payload_class.ClassId {
        if host_class.ClassId != 0 && host_class.DockingAllowUnclassed && payload_class.ClassId == 0 {
            return true;
        }
        if payload_class.ClassId != 0 && payload_class.DockingAllowUnclassed && host_class.ClassId == 0 {
            return true;
        }
        return false;
    }

    // Prevent docking any window created above a popup
    // Technically we should support it (e.g. in the case of a long-lived modal window that had fancy docking features),
    // by e.g. adding a 'if (!IsWindowWithinBeginStackOf(host_window, popup_window))' test.
    // But it would requires more work on our end because the dock host windows is technically created in NewFrame()
    // and our ->ParentXXX and ->RootXXX pointers inside windows are currently mislading or lacking.
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // for (let i: c_int = g.OpenPopupStack.len() - 1; i >= 0; i--)
    for i in g.OpenPopupStack.len() - 1..0 {
        let mut popup_window = g.OpenPopupStack[i].Window;
        if popup_window.is_null() == false {
            if IsWindowWithinBeginStackOf(payload, popup_window) {  // Payload is created from within a popup begin stack.
                return false;
            }
        }
    }

    return true;
}

pub unsafe fn DockNodeIsDropAllowed(host_window: &mut ImguiWindow, root_payload: *mut ImguiWindow) -> bool {
    if root_payload.DockNodeAsHost.is_null() == false && root_payload.DockNodeAsHost.IsSplitNode() { // FIXME-DOCK: Missing filtering
        return true;
    }

    let payload_count = if root_payload.DockNodeAsHost { root_payload.DockNodeAsHost.Windows.len() } else { 1 };
    // for (let payload_n: c_int = 0; payload_n < payload_count; payload_n++)
    for payload_n in 0..payload_count {
        let mut payload = if root_payload.DockNodeAsHost { root_payload.DockNodeAsHost.Windows[payload_n] } else { root_payload };
        if DockNodeIsDropAllowedOne(payload, host_window) {
            return true;
        }
    }
    return false;
}

// window menu button == collapse button when not in a dock node.
// FIXME: This is similar to RenderWindowTitleBarContents(), may want to share code.
pub unsafe fn DockNodeCalcTabBarLayout(
    node: *const ImGuiDockNode,
    out_title_rect: *mut ImRect,
    out_tab_bar_rect: *mut ImRect,
    out_window_menu_button_pos: *mut ImVec2,
    out_close_button_pos: *mut ImVec2) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let style = &mut g.style;

    let mut r: ImRect = ImRect::from_floats(node.Pos.x, node.Pos.y, node.Pos.x + node.Size.x, node.Pos.y + g.FontSize + g.style.FramePadding.y * 2.0);
    if out_title_rect { *out_title_rect = r; }

    r.min.x += style.WindowBorderSize;
    r.max.x -= style.WindowBorderSize;

    let button_sz: c_float = g.FontSize;

    let mut window_menu_button_pos: ImVec2 = r.min;
    r.min.x += style.FramePadding.x;
    r.max.x -= style.FramePadding.x;
    if node.HasCloseButton {
        r.max.x -= button_sz;
        if out_close_button_pos { *out_close_button_pos = ImVec2::from_floats(r.max.x - style.FramePadding.x, r.min.y); }
    }
    if node.HasWindowMenuButton && style.WindowMenuButtonPosition == ImGuiDir_Left {
        r.min.x += button_sz + style.ItemInnerSpacing.x;
    } else if node.HasWindowMenuButton && style.WindowMenuButtonPosition == ImGuiDir_Right {
        r.max.x -= button_sz + style.FramePadding.x;
        window_menu_button_pos = ImVec2::from_floats(r.max.x, r.min.y);
    }
    if out_tab_bar_rect { *out_tab_bar_rect = r; }
    if out_window_menu_button_pos { *out_window_menu_button_pos = window_menu_button_pos; }
}

pub unsafe fn DockNodeCalcSplitRects(pos_old: &mut ImVec2, size_old: &mut ImVec2, pos_new: &mut ImVec2, size_new: &mut ImVec2, dir: ImGuiDir, size_new_desired: ImVec2) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let dock_spacing: c_float = g.style.ItemInnerSpacing.x;
    const axis: ImGuiAxis = if dir == ImGuiDir_Left || dir == ImGuiDir_Right { ImGuiAxis_X } else { ImGuiAxis_Y };
    pos_new[axis ^ 1] = pos_old[axis ^ 1];
    size_new[axis ^ 1] = size_old[axis ^ 1];

    // Distribute size on given axis (with a desired size or equally)
    let w_avail: c_float = size_old[axis] - dock_spacing;
    if (size_new_desired[axis] > 0.0 && size_new_desired[axis] <= w_avail * 0.5) {
        size_new[axis] = size_new_desired[axis];
        size_old[axis] = IM_FLOOR(w_avail - size_new[axis]);
    } else {
        size_new[axis] = IM_FLOOR(w_avail * 0.5);
        size_old[axis] = IM_FLOOR(w_avail - size_new[axis]);
    }

    // Position each node
    if (dir == ImGuiDir_Right || dir == ImGuiDir_Down) {
        pos_new[axis] = pos_old[axis] + size_old[axis] + dock_spacing;
    } else if (dir == ImGuiDir_Left || dir == ImGuiDir_Up) {
        pos_new[axis] = pos_old[axis];
        pos_old[axis] = pos_new[axis] + size_new[axis] + dock_spacing;
    }
}

// Retrieve the drop rectangles for a given direction or for the center + perform hit testing.
pub unsafe fn DockNodeCalcDropRectsAndTestMousePos(parent: &mut ImRect, dir: ImGuiDir, out_r: &mut ImRect, outer_docking: bool, test_mouse_pos: *mut ImVec2) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;

    let parent_smaller_axis: c_float = ImMin(parent.GetWidth(), parent.GetHeight());
    let hs_for_central_nodes: c_float = ImMin(g.FontSize * 1.5, ImMax(g.FontSize * 0.5, parent_smaller_axis / 8.0));
    let mut hs_w: c_float = 0.0; // Half-size, longer axis
    let mut hs_h: c_float = 0.0; // Half-size, smaller axis
    off: ImVec2; // Distance from edge or center
    if outer_docking {
        //hs_w = ImFloor(ImClamp(parent_smaller_axis - hs_for_central_nodes * 4.0, g.FontSize * 0.5, g.FontSize * 8.0));
        //hs_h = ImFloor(hs_w * 0.150f32);
        //off = ImVec2::new(ImFloor(parent.GetWidth() * 0.5 - GetFrameHeightWithSpacing() * 1.4 - hs_h), ImFloor(parent.GetHeight() * 0.5 - GetFrameHeightWithSpacing() * 1.4 - hs_h));
        hs_w = ImFloor(hs_for_central_nodes * 1.5);
        hs_h = ImFloor(hs_for_central_nodes * 0.8);
        off = ImVec2::from_floats(ImFloor(parent.GetWidth() * 0.5 - hs_h), ImFloor(parent.GetHeight() * 0.5 - hs_h));
    } else {
        hs_w = ImFloor(hs_for_central_nodes);
        hs_h = ImFloor(hs_for_central_nodes * 0.9);
        off = ImVec2::from_floats(ImFloor(hs_w * 2.4), ImFloor(hs_w * 2.4));
    }

    let c: ImVec2 = ImFloor(parent.GetCenter());
    if dir == ImGuiDir_None { *out_r = ImRect(c.x - hs_w, c.y - hs_w, c.x + hs_w, c.y + hs_w); } else if dir == ImGuiDir_Up { *out_r = ImRect(c.x - hs_w, c.y - off.y - hs_h, c.x + hs_w, c.y - off.y + hs_h); } else if dir == ImGuiDir_Down { *out_r = ImRect(c.x - hs_w, c.y + off.y - hs_h, c.x + hs_w, c.y + off.y + hs_h); } else if dir == ImGuiDir_Left { *out_r = ImRect(c.x - off.x - hs_h, c.y - hs_w, c.x - off.x + hs_h, c.y + hs_w); } else if dir == ImGuiDir_Right { *out_r = ImRect(c.x + off.x - hs_h, c.y - hs_w, c.x + off.x + hs_h, c.y + hs_w); }

    if test_mouse_pos == None {
        return false;
    }

    let mut hit_r = out_r;
    if !outer_docking {
        // Custom hit testing for the 5-way selection, designed to reduce flickering when moving diagonally between sides
        hit_r.Expand(ImFloor(hs_w * 0.3));
        let mouse_delta: ImVec2 = (*test_mouse_pos - c);
        let mouse_delta_len2: c_float = ImLengthSqr(mouse_delta);
        let r_threshold_center: c_float = hs_w * 1.4;
        let r_threshold_sides: c_float = hs_w * (1.4 + 1.20);
        if mouse_delta_len2 < r_threshold_center * r_threshold_center {
            return dir == ImGuiDir_None;
        }
        if mouse_delta_len2 < r_threshold_sides * r_threshold_sides {
            return dir == ImGetDirQuadrantFromDelta(mouse_delta.x, mouse_delta.y);
        }
    }
    return hit_r.Contains(&*test_mouse_pos);
}

// host_node may be NULL if the window doesn't have a DockNode already.
// FIXME-DOCK: This is misnamed since it's also doing the filtering.
pub unsafe fn DockNodePreviewDockSetup(
    host_window: &mut ImguiWindow,
    host_node: *mut ImGuiDockNode,
    payload_window: &mut ImguiWindow,
    mut payload_node: *mut ImGuiDockNode,
    data: *mut ImGuiDockPreviewData,
    is_explicit_target: bool,
    is_outer_docking: bool) {
    let g = GImGui; // ImGuiContext& g = *GImGui;

    // There is an edge case when docking into a dockspace which only has inactive nodes.
    // In this case DockNodeTreeFindNodeByPos() will have selected a leaf node which is inactive.
    // Because the inactive leaf node doesn't have proper pos/size yet, we'll use the root node as reference.
    if payload_node == None {
        payload_node = payload_window.DockNodeAsHost;
    }
    let ref_node_for_rect = if host_node.is_null() == false && !host_node.IsVisible {
        DockNodeGetRootNode(host_node)
    } else {
        host_node
    };
    if (ref_node_for_rect) {}
    // IM_ASSERT(ref_node_for_rect.IsVisible == true);

    // Filter, figure out where we are allowed to dock
    let src_node_flags = if payload_node.is_null() == false { payload_node.MergedFlags } else { payload_window.WindowClass.DockNodeFlagsOverrideSet };
    let dst_node_flags = if host_node.is_null() == false { host_node.MergedFlags } else { host_window.WindowClass.DockNodeFlagsOverrideSet };
    data.IsCenterAvailable = true;
    if is_outer_docking {
        data.IsCenterAvailable = false;
    } else if dst_node_flags & ImGuiDockNodeFlags_NoDocking {
        data.IsCenterAvailable = false;
    } else if host_node.is_null() == false && flag_set(dst_node_flags, ImGuiDockNodeFlags_NoDockingInCentralNode) && host_node.IsCentralNode() {
        data.IsCenterAvailable = false;
    } else if (!host_node.is_null() == false || !host_node.IsEmpty()) && payload_node.is_null() == false && payload_node.IsSplitNode() && (payload_node.OnlyNodeWithWindows == null_mut()) { // Is _visibly_ split?
        data.IsCenterAvailable = false;
    } else if dst_node_flags & ImGuiDockNodeFlags_NoDockingOverMe {
        data.IsCenterAvailable = false;
    } else if flag_set(src_node_flags, ImGuiDockNodeFlags_NoDockingOverOther) && (host_node.is_null() || !host_node.IsEmpty()) {
        data.IsCenterAvailable = false;
    } else if flag_set(src_node_flags, ImGuiDockNodeFlags_NoDockingOverEmpty) && host_node.is_null() == false && host_node.IsEmpty() {
        data.IsCenterAvailable = false;
    }

    data.IsSidesAvailable = true;
    if flag_set(dst_node_flags, ImGuiDockNodeFlags_NoSplit) || g.IO.ConfigDockingNoSplit {
        data.IsSidesAvailable = false;
    } else if !is_outer_docking && host_node.is_null() == false && host_node.ParentNode == None && host_node.IsCentralNode() {
        data.IsSidesAvailable = false;
    } else if (dst_node_flags & ImGuiDockNodeFlags_NoDockingSplitMe) || (src_node_flags & ImGuiDockNodeFlags_NoDockingSplitOther) {
        data.IsSidesAvailable = false;
    }

    // Build a tentative future node (reuse same structure because it is practical. Shape will be readjusted when previewing a split)
    data.FutureNode.HasCloseButton = (if host_node { host_node.HasCloseButton } else { host_window.HasCloseButton }) || (payload_window.HasCloseButton);
    data.FutureNode.HasWindowMenuButton = if host_node { true } else { flag_set(host_window.Flags, ImGuiWindowFlags_NoCollapse) == 0 };
    data.FutureNode.Pos = if ref_node_for_rect { ref_node_for_rect.Pos } else { host_window.position };
    data.FutureNode.Size = if ref_node_for_rect { ref_node_for_rect.Size } else { host_window.Size };

    // Calculate drop shapes geometry for allowed splitting directions
    // IM_ASSERT(ImGuiDir_None == -1);
    data.SplitNode = host_node;
    data.SplitDir = ImGuiDir_None;
    data.IsSplitDirExplicit = false;
    if !host_window.Collapsed {
        // for (let dir: c_int = ImGuiDir_None; dir < ImGuiDir_COUNT; dir+ +)
        for dir in ImGuiDirNone..ImGuiDir_COUNT {
            if dir == ImGuiDir_None && !data.IsCenterAvailable {
                continue;
            }
            if dir != ImGuiDir_None && !data.IsSidesAvailable {
                continue;
            }
            if DockNodeCalcDropRectsAndTestMousePos(data.FutureNode.Rect(), dir, data.DropRectsDraw[dir1], is_outer_docking, &mut g.IO.MousePos) {
                data.SplitDir = dir;
                data.IsSplitDirExplicit = true;
            }
        }
    }

    // When docking without holding Shift, we only allow and preview docking when hovering over a drop rect or over the title bar
    data.IsDropAllowed = (data.SplitDir != ImGuiDir_None) || (data.IsCenterAvailable);
    if !is_explicit_target && !data.IsSplitDirExplicit && !g.IO.ConfigDockingWithShift {
        data.IsDropAllowed = false;
    }

    // Calculate split area
    data.SplitRatio = 0.0;
    if data.SplitDir != ImGuiDir_None {
        split_dir: ImGuiDir = data.SplitDir;
        split_axis: ImGuiAxis = if split_dir == ImGuiDir_Left || split_dir == ImGuiDir_Right { ImGuiAxis_X } else { ImGuiAxis_Y };
        // pos_new: ImVec2, pos_old = data.FutureNode.Pos;
        let mut pos_new = data.FutureNode.Pos.clone();
        let mut pos_old = data.FutureNode.Pos.clone();
        // size_new: ImVec2, size_old = data.FutureNode.Size;
        let mut size_new = data.FutureNode.Size.clone();
        let mut size_old = data.FutureNode.Size.clone();

        DockNodeCalcSplitRects(&mut pos_old, &mut size_old, &mut pos_new, &mut size_new, split_dir, payload_window.Size);

        // Calculate split ratio so we can pass it down the docking request
        let split_ratio = ImSaturate(size_new[split_axis] / data.FutureNode.Size[split_axis]);
        data.FutureNode.Pos = pos_new;
        data.FutureNode.Size = size_new;
        data.SplitRatio = if split_dir == ImGuiDir_Right || split_dir == ImGuiDir_Down { (1.0 - split_ratio) } else { (split_ratio) };
    }
}

pub unsafe fn DockNodePreviewDockRender(
    host_window: &mut ImguiWindow,
    host_node: *mut ImGuiDockNode,
    root_payload: *mut ImguiWindow,
    data: *const ImGuiDockPreviewData) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.CurrentWindow == host_window);   // Because we rely on font size to calculate tab sizes

    // With this option, we only display the preview on the target viewport, and the payload viewport is made transparent.
    // To compensate for the single layer obstructed by the payload, we'll increase the alpha of the preview nodes.
    let is_transparent_payload: bool = g.IO.ConfigDockingTransparentPayload;

    // In case the two windows involved are on different viewports, we will draw the overlay on each of them.
    let mut overlay_draw_lists_count: size_t = 0;
    // ImDrawList* overlay_draw_lists[2];
    let mut overlay_draw_lists: [*mut ImDrawList; 2] = [None; 2];
    overlay_draw_lists[overlay_draw_lists_count] = GetForegroundDrawList(host_window.Viewport);
    overlay_draw_lists_count += 1;
    if host_window.Viewport != root_payload.Viewport && !is_transparent_payload {
        overlay_draw_lists[overlay_draw_lists_count] = GetForegroundDrawList(root_payload.Viewport);
        overlay_draw_lists_count += 1;
    }

    // Draw main preview rectangle
    let mut overlay_col_main: u32 = GetColorU32(ImGuiCol_DockingPreview, if is_transparent_payload { 0.60 } else { 0.4 });
    let mut overlay_col_drop: u32 = GetColorU32(ImGuiCol_DockingPreview, if is_transparent_payload { 0.90 } else { 0.70 });
    let mut overlay_col_drop_hovered: u32 = GetColorU32(ImGuiCol_DockingPreview, if is_transparent_payload { 1.20 } else { 1.0 });
    let mut overlay_col_lines: u32 = GetColorU32(ImGuiCol_NavWindowingHighlight, if is_transparent_payload { 0.80 } else { 0.60 });

    // Display area preview
    let can_preview_tabs: bool = (root_payload.DockNodeAsHost == None || root_payload.DockNodeAsHost.Windows.len() > 0);
    if data.IsDropAllowed {
        let mut overlay_rect = data.FutureNode.Rect();
        if data.SplitDir == ImGuiDir_None && can_preview_tabs {
            overlay_rect.Min.y += GetFrameHeight();
        }
        if data.SplitDir != ImGuiDir_None || data.IsCenterAvailable {
            // for (let overlay_n: c_int = 0; overlay_n < overlay_draw_lists_count; overlay_n+ +)
            for overlay_n in 0..overlay_draw_lists_count {
                overlay_draw_lists[overlay_n].AddRectFilled(overlay_rect.Min, overlay_rect.Max, overlay_col_main, host_window.WindowRounding, CalcRoundingFlagsForRectInRect(overlay_rect, host_window.Rect(), DOCKING_SPLITTER_SIZE));
            }
        }
    }

    // Display tab shape/label preview unless we are splitting node (it generally makes the situation harder to read)
    if data.IsDropAllowed && can_preview_tabs && data.SplitDir == ImGuiDir_None && data.IsCenterAvailable {
        // Compute target tab bar geometry so we can locate our preview tabs
        let mut tab_bar_rect: ImRect = ImRect::default();
        DockNodeCalcTabBarLayout(&data.FutureNode, None, &mut tab_bar_rect, None, null_mut());
        let mut tab_pos: ImVec2 = tab_bar_rect.min;
        if host_node.is_null() == false && host_node.TabBar.is_null() == false {
            if !host_node.IsHiddenTabBar() && !host_node.IsNoTabBar() {
                tab_pos.x += host_node.TabBar.WidthAllTabs + g.style.ItemInnerSpacing.x;
            } // We don't use OffsetNewTab because when using non-persistent-order tab bar it is incremented with each Tab submission.
            else {
                tab_pos.x += g.style.ItemInnerSpacing.x + TabItemCalcSize(host_node.Windows[0].Name, host_node.Windows[0].HasCloseButton).x;
            }
        } else if flag_clear(host_window.Flags, ImGuiWindowFlags_DockNodeHost) {
            tab_pos.x += g.style.ItemInnerSpacing.x + TabItemCalcSize(host_window.Name, host_window.HasCloseButton).x; // Account for slight offset which will be added when changing from title bar to tab bar
        }

        // Draw tab shape/label preview (payload may be a loose window or a host window carrying multiple tabbed windows)
        if root_payload.DockNodeAsHost {}
        // IM_ASSERT(root_payload.DockNodeAsHost.Windows.Size <= root_payload.DockNodeAsHost.TabBar.Tabs.Size);
        ImGuiTabBar * tab_bar_with_payload = if root_payload.DockNodeAsHost { root_payload.DockNodeAsHost.TabBar } else { None };
        let payload_count: c_int = if tab_bar_with_payload { tab_bar_with_payload.Tabs.Size } else { 1 };
        // for (let payload_n: c_int = 0; payload_n < payload_count; payload_n++)
        for payload_n in 0..payload_count {
            // DockNode's TabBar may have non-window Tabs manually appended by user
            let mut payload_window: &mut ImguiWindow = if tab_bar_with_payload { tab_bar_with_payload.Tabs[payload_n].Window } else { root_payload };
            if tab_bar_with_payload && payload_window == None {
                continue;
            }
            if !DockNodeIsDropAllowedOne(payload_window, host_window) {
                continue;
            }

            // Calculate the tab bounding box for each payload window
            let tab_size: ImVec2 = TabItemCalcSize(payload_window.Name, payload_window.HasCloseButton);
            let mut tab_bb: ImRect = ImRect::new(tab_pos.x, tab_pos.y, tab_pos.x + tab_size.x, tab_pos.y + tab_size.y);
            tab_pos.x += tab_size.x + g.style.ItemInnerSpacing.x;
            overlay_col_text: u32 = GetColorU32(payload_window.DockStyle.Colors[ImGuiWindowDockStyleCol_Text], 0.0);
            overlay_col_tabs: u32 = GetColorU32(payload_window.DockStyle.Colors[ImGuiWindowDockStyleCol_TabActive], 0.0);
            PushStyleColor(ImGuiCol_Text, overlay_col_text);
            // for (let overlay_n: c_int = 0; overlay_n < overlay_draw_lists_count; overlay_n++)
            for overlay_n in 0..overlay_draw_lists_count {
                let tab_flags = ImGuiTabItemFlags_Preview | (if payload_window.Flags & ImGuiWindowFlags_UnsavedDocument { ImGuiTabItemFlags_UnsavedDocument } else { 0 });
                if !tab_bar_rect.Contains(tab_bb.into()) {
                    overlay_draw_lists[overlay_n].PushClipRect(&tab_bar_rect.min, &tab_bar_rect.max, false);
                }
                TabItemBackground(overlay_draw_lists[overlay_n], tab_bb, tab_flags, overlay_col_tabs);
                TabItemLabelAndCloseButton(overlay_draw_lists[overlay_n], tab_bb, tab_flags, g.style.FramePadding, payload_window.Name, 0, 0, false, None, null_mut());
                if !tab_bar_rect.Contains(tab_bb.into()) {
                    overlay_draw_lists[overlay_n].PopClipRect();
                    overlay_draw_lists_count
                }
            }
            PopStyleColor(0);
        }
    }

    // Display drop boxes
    let overlay_rounding: c_float = ImMax(3.0, g.style.FrameRounding);
    // for (let dir: c_int = ImGuiDir_None; dir < ImGuiDir_COUNT; dir++)
    for dir in ImGuiDir_None..ImGuiDir_COUNT {
        if !data.DropRectsDraw[dir + 1].IsInverted() {
            let mut draw_r: ImRect = data.DropRectsDraw[dir + 1];
            let mut draw_r_in: ImRect = draw_r;
            draw_r_in.Expand(-2.0);
            let overlay_col = if data.SplitDir == dir && data.IsSplitDirExplicit { overlay_col_drop_hovered } else { overlay_col_drop };
            // for (let overlay_n: c_int = 0; overlay_n < overlay_draw_lists_count; overlay_n++)
            {
                let center: ImVec2 = ImFloor(draw_r_in.GetCenter());
                overlay_draw_lists[overlay_n].AddRectFilled(draw_r.min, draw_r.max, overlay_col, overlay_rounding);
                overlay_draw_lists[overlay_n].AddRect(draw_r_in.min, draw_r_in.max, overlay_col_lines, overlay_rounding);
                if dir == ImGuiDir_Left || dir == ImGuiDir_Right {
                    overlay_draw_lists[overlay_n].AddLine(ImVec2::from_floats(center.x, draw_r_in.min.y), ImVec2::from_floats(center.x, draw_r_in.max.y), overlay_col_lines);
                }
                if dir == ImGuiDir_Up || dir == ImGuiDir_Down {
                    overlay_draw_lists[overlay_n].AddLine(ImVec2::from_floats(draw_r_in.min.x, center.y), ImVec2::from_floats(draw_r_in.max.x, center.y), overlay_col_lines);
                }
            }
        }

        // Stop after ImGuiDir_None
        if (host_node.is_null() == false && flag_set(host_node.MergedFlags, ImGuiDockNodeFlags_NoSplit)) || g.IO.ConfigDockingNoSplit {
            return;
        }
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

pub unsafe fn DockNodeTreeSplit(ctx: *mut crate::context::ImguiContext, parent_node: *mut ImGuiDockNode, split_axis: ImGuiAxis, split_inheritor_child_idx: c_int, split_ratio: c_float, new_node: *mut ImGuiDockNode) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(split_axis != ImGuiAxis_None);

    let child_0: *mut ImGuiDockNode = if new_node.is_null() == false && split_inheritor_child_idx != 0 { new_node } else { DockContextAddNode(ctx, 0) };
    child_0.ParentNode = parent_node;

    let child_1: *mut ImGuiDockNode = if new_node.is_null() == false && split_inheritor_child_idx != 1 { new_node } else { DockContextAddNode(ctx, 0) };
    child_1.ParentNode = parent_node;

    let child_inheritor: *mut ImGuiDockNode = if split_inheritor_child_idx == 0 { child_0 } else { child_1 };
    DockNodeMoveChildNodes(child_inheritor, parent_node);
    parent_node.ChildNodes[0] = child_0;
    parent_node.ChildNodes[1] = child_1;
    parent_node.ChildNodes[split_inheritor_child_idx].VisibleWindow = parent_node.VisibleWindow;
    parent_node.SplitAxis = split_axis;
    parent_node.VisibleWindow = None;
    parent_node.AuthorityForPos = IM_GUI_DATA_AUTHORITY_DOCK_NODE;
    parent_node.AuthorityForSize = IM_GUI_DATA_AUTHORITY_DOCK_NODE;

    let mut size_avail = (parent_node.Size[split_axis] - DOCKING_SPLITTER_SIZE);
    size_avail = ImMax(size_avail, g.style.WindowMinSize[split_axis] * 2.0);
    // IM_ASSERT(size_avail > 0.0); // If you created a node manually with DockBuilderAddNode(), you need to also call DockBuilderSetNodeSize() before splitting.
    child_0.SizeRef = parent_node.Size;
    child_1.SizeRef = parent_node.Size;
    child_0.SizeRef[split_axis] = ImFloor(size_avail * split_ratio);
    child_1.SizeRef[split_axis] = ImFloor(size_avail - child_0.SizeRef[split_axis]);

    DockNodeMoveWindows(parent_node.ChildNodes[split_inheritor_child_idx], parent_node);
    DockSettingsRenameNodeReferences(parent_node.ID, parent_node.ChildNodes[split_inheritor_child_idx].ID);
    DockNodeUpdateHasCentralNodeChild(DockNodeGetRootNode(parent_node));
    DockNodeTreeUpdatePosSize(parent_node, parent_node.Pos, parent_node.Size, null_mut());

    // Flags transfer (e.g. this is where we transfer the ImGuiDockNodeFlags_CentralNode property)
    child_0.SharedFlags = parent_node.SharedFlags & ImGuiDockNodeFlags_SharedFlagsInheritMask_;
    child_1.SharedFlags = parent_node.SharedFlags & ImGuiDockNodeFlags_SharedFlagsInheritMask_;
    child_inheritor.LocalFlags = parent_node.LocalFlags & ImGuiDockNodeFlags_LocalFlagsTransferMask_;
    parent_node.LocalFlags &= !ImGuiDockNodeFlags_LocalFlagsTransferMask_;
    child_0.UpdateMergedFlags();
    child_1.UpdateMergedFlags();
    parent_node.UpdateMergedFlags();
    if child_inheritor.IsCentralNode() {
        DockNodeGetRootNode(parent_node).CentralNode = child_inheritor;
    }
}

pub unsafe fn DockNodeTreeMerge(ctx: *mut crate::context::ImguiContext, parent_node: *mut ImGuiDockNode, merge_lead_child: *mut ImGuiDockNode) {
    // When called from DockContextProcessUndockNode() it is possible that one of the child is NULL.
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let child_0: *mut ImGuiDockNode = parent_node.ChildNodes[0];
    let child_1: *mut ImGuiDockNode = parent_node.ChildNodes[1];
    // IM_ASSERT(child_0 || child_1);
    // IM_ASSERT(merge_lead_child == child_0 || merge_lead_child == child_1);
    if (child_0.is_null() == false && child_0.Windows.len() > 0) || (child_1.is_null() == false && child_1.Windows.len() > 0) {
        // IM_ASSERT(parent_node.TabBar == NULL);
        // IM_ASSERT(parent_node.Windows.Size == 0);
    }
    // IMGUI_DEBUG_LOG_DOCKING("[docking] DockNodeTreeMerge: 0x{} + 0x{} back into parent 0x{}\n", child_0 ? child_0.ID : 0, child_1 ? child_1.ID : 0, parent_node.ID);

    let backup_last_explicit_size: ImVec2 = parent_node.SizeRef;
    DockNodeMoveChildNodes(parent_node, merge_lead_child);
    if (child_0) {
        DockNodeMoveWindows(parent_node, child_0); // Generally only 1 of the 2 child node will have windows
        DockSettingsRenameNodeReferences(child_0.ID, parent_node.ID);
    }
    if (child_1) {
        DockNodeMoveWindows(parent_node, child_1);
        DockSettingsRenameNodeReferences(child_1.ID, parent_node.ID);
    }
    DockNodeApplyPosSizeToWindows(parent_node);
    parent_node.AuthorityForPos = IM_GUI_DATA_AUTHORITY_AUTO;
    parent_node.AuthorityForSize = IM_GUI_DATA_AUTHORITY_AUTO;
    parent_node.AuthorityForViewport = IM_GUI_DATA_AUTHORITY_AUTO;
    parent_node.VisibleWindow = merge_lead_child.VisibleWindow;
    parent_node.SizeRef = backup_last_explicit_size;

    // Flags transfer
    parent_node.LocalFlags &= !ImGuiDockNodeFlags_LocalFlagsTransferMask_; // Preserve Dockspace flag
    parent_node.LocalFlags |= (if child_0 { child_0.LocalFlags } else { 0 }) & ImGuiDockNodeFlags_LocalFlagsTransferMask_;
    parent_node.LocalFlags |= (if child_1 { child_1.LocalFlags } else { 0 }) & ImGuiDockNodeFlags_LocalFlagsTransferMask_;
    parent_node.LocalFlagsInWindows = (if child_0 { child_0.LocalFlagsInWindows } else { 0 }) | (if child_1 { child_1.LocalFlagsInWindows } else { 0 }); // FIXME: Would be more consistent to update from actual windows
    parent_node.UpdateMergedFlags();

    if child_0 {
        ctx.dock_context.dock_nodes.SetVoidPtr(child_0.ID, null_mut());
        IM_DELETE(child_0);
    }
    if child_1 {
        ctx.dock_context.dock_nodes.SetVoidPtr(child_1.ID, null_mut());
        IM_DELETE(child_1);
    }
}

// Update Pos/Size for a node hierarchy (don't affect child Windows yet)
// (Depth-first, Pre-Order)
pub unsafe fn DockNodeTreeUpdatePosSize(node: *mut ImGuiDockNode, pos: ImVec2, size: ImVec2, only_write_to_single_node: *mut ImGuiDockNode) {
    // During the regular dock node update we write to all nodes.
    // 'only_write_to_single_node' is only set when turning a node visible mid-frame and we need its size right-away.
    let write_to_node: bool = only_write_to_single_node == None || only_write_to_single_node == node;
    if write_to_node {
        node.Pos = pos;
        node.Size = size;
    }

    if node.IsLeafNode() {
        return;
    }

    let child_0: *mut ImGuiDockNode = node.ChildNodes[0];
    let child_1: *mut ImGuiDockNode = node.ChildNodes[1];
    let child_0_pos: ImVec2 = pos;
    let child_1_pos = pos;
    let child_0_size: ImVec2 = size;
    let child_1_size = size;

    let child_0_is_toward_single_node: bool = (only_write_to_single_node != None && DockNodeIsInHierarchyOf(only_write_to_single_node, child_0));
    let child_1_is_toward_single_node: bool = (only_write_to_single_node != None && DockNodeIsInHierarchyOf(only_write_to_single_node, child_1));
    let child_0_is_or_will_be_visible: bool = child_0.IsVisible || child_0_is_toward_single_node;
    let child_1_is_or_will_be_visible: bool = child_1.IsVisible || child_1_is_toward_single_node;

    if child_0_is_or_will_be_visible && child_1_is_or_will_be_visible {
        let g = GImGui; // ImGuiContext& g = *GImGui;
        let spacing: c_float = DOCKING_SPLITTER_SIZE;
        const axis: ImGuiAxis = node.SplitAxis;
        let size_avail: c_float = ImMax(size[axis] - spacing, 0.0);

        // Size allocation policy
        // 1) The first 0..WindowMinSize[axis]*2 are allocated evenly to both windows.
        let size_min_each: c_float = ImFloor(ImMin(size_avail, g.style.WindowMinSize[axis] * 2.0) * 0.5);

        // FIXME: Blocks 2) and 3) are essentially doing nearly the same thing.
        // Difference are: write-back to SizeRef; application of a minimum size; rounding before ImFloor()
        // Clarify and rework differences between Size & SizeRef and purpose of WantLockSizeOnce

        // 2) Process locked absolute size (during a splitter resize we preserve the child of nodes not touching the splitter edge)
        if child_0.WantLockSizeOnce && !child_1.WantLockSizeOnce {
            child_0_size[axis] = child_0.SizeRef[axis] = ImMin(size_avail - 1.0, child_0.Size[axis]);
            child_1_size[axis] = child_1.SizeRef[axis] = (size_avail - child_0_size[axis]);
            // IM_ASSERT(child_0.SizeRef[axis] > 0.0 && child_1.SizeRef[axis] > 0.0);
        } else if child_1.WantLockSizeOnce && !child_0.WantLockSizeOnce {
            child_1_size[axis] = child_1.SizeRef[axis] = ImMin(size_avail - 1.0, child_1.Size[axis]);
            child_0_size[axis] = child_0.SizeRef[axis] = (size_avail - child_1_size[axis]);
            // IM_ASSERT(child_0.SizeRef[axis] > 0.0 && child_1.SizeRef[axis] > 0.0);
        } else if child_0.WantLockSizeOnce && child_1.WantLockSizeOnce {
            // FIXME-DOCK: We cannot honor the requested size, so apply ratio.
            // Currently this path will only be taken if code programmatically sets WantLockSizeOnce
            let split_ratio: c_float = child_0_size[axis] / (child_0_size[axis] + child_1_size[axis]);
            child_0_size[axis] = child_0.SizeRef[axis] = ImFloor(size_avail * split_ratio);
            child_1_size[axis] = child_1.SizeRef[axis] = (size_avail - child_0_size[axis]);
            // IM_ASSERT(child_0.SizeRef[axis] > 0.0 && child_1.SizeRef[axis] > 0.0);
        }

        // 3) If one window is the central node (~ use remaining space, should be made explicit!), use explicit size from the other, and remainder for the central node
        else if child_0.SizeRef[axis] != 0.0 && child_1.HasCentralNodeChild {
            child_0_size[axis] = ImMin(size_avail - size_min_each, child_0.SizeRef[axis]);
            child_1_size[axis] = (size_avail - child_0_size[axis]);
        } else if child_1.SizeRef[axis] != 0.0 && child_0.HasCentralNodeChild {
            child_1_size[axis] = ImMin(size_avail - size_min_each, child_1.SizeRef[axis]);
            child_0_size[axis] = (size_avail - child_1_size[axis]);
        } else {
            // 4) Otherwise distribute according to the relative ratio of each SizeRef value
            let split_ratio: c_float = child_0.SizeRef[axis] / (child_0.SizeRef[axis] + child_1.SizeRef[axis]);
            child_0_size[axis] = ImMax(size_min_each, ImFloor(size_avail * split_ratio + 0.5));
            child_1_size[axis] = (size_avail - child_0_size[axis]);
        }

        child_1_pos[axis] += spacing + child_0_size[axis];
    }

    if only_write_to_single_node == None {
        child_0.WantLockSizeOnce = false;
        child_1.WantLockSizeOnce = false;
    }

    let child_0_recurse: bool = if only_write_to_single_node { child_0_is_toward_single_node } else { child_0.IsVisible };
    let child_1_recurse: bool = if only_write_to_single_node { child_1_is_toward_single_node } else { child_1.IsVisible };
    if child_0_recurse {
        DockNodeTreeUpdatePosSize(child_0, child_0_pos, child_0_size, null_mut());
    }
    if child_1_recurse {
        DockNodeTreeUpdatePosSize(child_1, child_1_pos, child_1_size, null_mut());
    }
}

pub unsafe fn DockNodeTreeUpdateSplitterFindTouchingNode(
    node: *mut ImGuiDockNode,
    axis: ImGuiAxis,
    side: c_int,
    touching_nodes: &mut Vec<*mut ImGuiDockNode>) {
    if node.IsLeafNode() {
        touching_nodes.push(node.clone());
        return;
    }
    if node.ChildNodes[0].IsVisible {
        if node.SplitAxis != axis || side == 0 || !node.ChildNodes[1].IsVisible {
            DockNodeTreeUpdateSplitterFindTouchingNode(node.ChildNodes[0], axis, side, touching_nodes);
        }
    }
    if node.ChildNodes[1].IsVisible {
        if node.SplitAxis != axis || side == 1 || !node.ChildNodes[0].IsVisible {
            DockNodeTreeUpdateSplitterFindTouchingNode(node.ChildNodes[1], axis, side, touching_nodes);
        }
    }
}

// (Depth-First, Pre-Order)
pub unsafe fn DockNodeTreeUpdateSplitter(node: *mut ImGuiDockNode) {
    if (node.IsLeafNode()) {
        return;
    }

    let g = GImGui; // ImGuiContext& g = *GImGui;

    child_0: *mut ImGuiDockNode = node.ChildNodes[0];
    child_1: *mut ImGuiDockNode = node.ChildNodes[1];
    if child_0.IsVisible && child_1.IsVisible {
        // Bounding box of the splitter cover the space between both nodes (w = Spacing, h = Size[xy^1] for when splitting horizontally)
        let axis = node.SplitAxis;
        // IM_ASSERT(axis != ImGuiAxis_None);
        let mut bb: ImRect = ImRect::default();
        bb.min = child_0.Pos;
        bb.max = child_1.Pos;
        bb.min[axis] += child_0.Size[axis];
        bb.max[axis ^ 1] += child_1.Size[axis ^ 1];
        //if (g.IO.KeyCtrl) GetForegroundDrawList(g.Currentwindow.Viewport).AddRect(bb.Min, bb.Max, IM_COL32(255,0,255,255));

        let merged_flags = child_0.MergedFlags | child_1.MergedFlags; // Merged flags for BOTH childs
        let no_resize_axis_flag = if axis == ImGuiAxis_X { ImGuiDockNodeFlags_NoResizeX } else { ImGuiDockNodeFlags_NoResizeY };
        if (merged_flags & ImGuiDockNodeFlags_NoResize) || (merged_flags & no_resize_axis_flag) {
            let mut window  = g.current_window_mut().unwrap();
            window.DrawList.AddRectFilled(&bb.min, &bb.max, GetColorU32(ImGuiCol_Separator, 0.0), g.style.FrameRounding, 0);
        } else {
            //bb.Min[axis] += 1; // Display a little inward so highlight doesn't connect with nearby tabs on the neighbor node.
            //bb.Max[axis] -= 1;
            push_int_id(g, node.ID);

            // Find resizing limits by gathering list of nodes that are touching the splitter line.
            // Vec<ImGuiDockNode*> touching_nodes[2];
            let mut touching_nodes: [Vec<*mut ImGuiDockNode>; 2] = [vec![]; 2];
            let min_size: c_float = g.style.WindowMinSize[axis];
            resize_limits: [c_float;2];
            resize_limits[0] = node.ChildNodes[0].Pos[axis] + min_size;
            resize_limits[1] = node.ChildNodes[1].Pos[axis] + node.ChildNodes[1].Size[axis] - min_size;

            let mut splitter_id: ImguiHandle = id_from_str(str_to_const_c_char_ptr("##Splitter"));
            // Only process when splitter is active {
            if g.ActiveId == splitter_id {
                DockNodeTreeUpdateSplitterFindTouchingNode(child_0, axis, 1, &mut touching_nodes[0]);
                DockNodeTreeUpdateSplitterFindTouchingNode(child_1, axis, 0, &mut touching_nodes[1]);
                // for (let touching_node_n: c_int = 0; touching_node_n < touching_nodes[0].Size; touching_node_n++)
                for touching_node_n in 0..touching_nodes[0].len() {
                    resize_limits[0] = ImMax(resize_limits[0], touching_nodes[0][touching_node_n].Rect().Min[axis] + min_size);
                }
                // for (let touching_node_n: c_int = 0; touching_node_n < touching_nodes[1].Size; touching_node_n++)
                for touching_node_n in 0..touching_nodes[1].len() {
                    resize_limits[1] = ImMin(resize_limits[1], touching_nodes[1][touching_node_n].Rect().Max[axis] - min_size);
                }

                // [DEBUG] Render touching nodes & limits
                /*
                draw_list: *mut ImDrawList = node->HostWindow ? GetForegroundDrawList(node->HostWindow) : GetForegroundDrawList(GetMainViewport());
                for (int n = 0; n < 2; n++)
                {
                    for (int touching_node_n = 0; touching_node_n < touching_nodes[n].Size; touching_node_n++)
                        draw_list.AddRect(touching_nodes[n][touching_node_n].Pos, touching_nodes[n][touching_node_n].Pos + touching_nodes[n][touching_node_n].Size, IM_COL32(0, 255, 0, 255));
                    if (axis == ImGuiAxis_X)
                        draw_list.AddLine(ImVec2::new(resize_limits[n], node->ChildNodes[n].Pos.y), ImVec2::new(resize_limits[n], node->ChildNodes[n].Pos.y + node->ChildNodes[n].Size.y), IM_COL32(255, 0, 255, 255), 3.0);
                    else
                        draw_list.AddLine(ImVec2::new(node->ChildNodes[n].Pos.x, resize_limits[n]), ImVec2::new(node->ChildNodes[n].Pos.x + node->ChildNodes[n].Size.x, resize_limits[n]), IM_COL32(255, 0, 255, 255), 3.0);
                }
                */
            }

            // Use a short delay before highlighting the splitter (and changing the mouse cursor) in order for regular mouse movement to not highlight many splitters
            let cur_size_0: c_float = child_0.Size[axis];
            let cur_size_1: c_float = child_1.Size[axis];
            let min_size_0: c_float = resize_limits[0] - child_0.Pos[axis];
            let min_size_1: c_float = child_1.Pos[axis] + child_1.Size[axis] - resize_limits[1];
            bg_col: u32 = GetColorU32(ImGuiCol_WindowBg, 0.0);
            if SplitterBehavior(bb, id_from_str(str_to_const_c_char_ptr("##Splitter")), axis, &cur_size_0, &cur_size_1, min_size_0, min_size_1, WINDOWS_HOVER_PADDING, WINDOWS_RESIZE_FROM_EDGES_FEEDBACK_TIMER, bg_col) {
                if touching_nodes[0].Size > 0 && touching_nodes[1].Size > 0 {
                    child_0.Size[axis] = child_0.SizeRef[axis] = cur_size_0;
                    child_1.Pos[axis] -= cur_size_1 - child_1.Size[axis];
                    child_1.Size[axis] = child_1.SizeRef[axis] = cur_size_1;

                    // Lock the size of every node that is a sibling of the node we are touching
                    // This might be less desirable if we can merge sibling of a same axis into the same parental level.
                    // for (let side_n: c_int = 0; side_n < 2; side_n++)
                    for side_n in 0..2 {
                        // for (let touching_node_n: c_int = 0; touching_node_n < touching_nodes[side_n].Size; touching_node_n+ +)
                        for touching_node_n in 0..touching_nodes[side_n].len() {
                            let mut touching_node = touching_nodes[side_n][touching_node_n];
                            //draw_list: *mut ImDrawList = node->HostWindow ? GetForegroundDrawList(node->HostWindow) : GetForegroundDrawList(GetMainViewport());
                            //draw_list.AddRect(touching_node.Pos, touching_node.Pos + touching_node.Size, IM_COL32(255, 128, 0, 255));
                            while touching_node.ParentNode != node {
                                if touching_node.ParentNode.SplitAxis == axis {
                                    // Mark other node so its size will be preserved during the upcoming call to DockNodeTreeUpdatePosSize().
                                    let mut node_to_preserve = touching_node.ParentNode.ChildNodes[side_n];
                                    node_to_preserve.WantLockSizeOnce = true;
                                    //draw_list.AddRect(touching_node.Pos, touching_node.Rect().Max, IM_COL32(255, 0, 0, 255));
                                    //draw_list.AddRectFilled(node_to_preserve.Pos, node_to_preserve.Rect().Max, IM_COL32(0, 255, 0, 100));
                                }
                                touching_node = touching_node.ParentNode;
                            }
                        }
                    }

                    DockNodeTreeUpdatePosSize(child_0, child_0.Pos, child_0.Size, null_mut());
                    DockNodeTreeUpdatePosSize(child_1, child_1.Pos, child_1.Size, null_mut());
                    MarkIniSettingsDirty();
                }
            }
            pop_win_id_from_stack(g);
        }
    }

    if child_0.IsVisible {
        DockNodeTreeUpdateSplitter(child_0);
    }
    if child_1.IsVisible {
        DockNodeTreeUpdateSplitter(child_1);
    }
}

// DockNodeTreeFindFallbackLeafNode:*mut ImGuiDockNode(node:*mut ImGuiDockNode)
pub unsafe fn DockNodeTreeFindFallbackLeafNode(node: *mut ImGuiDockNode) -> *mut ImGuiDockNode {
    if node.IsLeafNode() {
        return node;
    }
    let mut leaf_node = DockNodeTreeFindFallbackLeafNode(node.ChildNodes[0]);
    if leaf_node.is_null() == false {
        return leaf_node;
    }
    leaf_node = DockNodeTreeFindFallbackLeafNode(node.ChildNodes[1]);
    if leaf_node.is_null() {
        return leaf_node;
    }
    return None;
}

// DockNodeTreeFindVisibleNodeByPos:*mut ImGuiDockNode(node:*mut ImGuiDockNode, pos: ImVec2)
pub unsafe fn DockNodeTreeFindVisibleNodeByPos(node: *mut ImGuiDockNode, pos: ImVec2) -> *mut ImGuiDockNode {
    if !node.IsVisible {
        return None;
    }

    let dock_spacing: c_float = 0.0;// g.style.ItemInnerSpacing.x; // FIXME: Relation to DOCKING_SPLITTER_SIZE?
    let mut r: ImRect = ImRect::new(node.Pos, node.Pos + node.Size);
    r.Expand(dock_spacing * 0.5);
    let mut inside: bool = r.Contains(&pos);
    if !inside {
        return None;
    }

    if node.IsLeafNode() {
        return node;
    }
    let mut hovered_node = DockNodeTreeFindVisibleNodeByPos(node.ChildNodes[0], pos);
    if hovered_node.is_null() == false {
        return hovered_node;
    }

    hovered_node = DockNodeTreeFindVisibleNodeByPos(node.ChildNodes[1], pos);
    if hovered_node.is_null() == false {
        return hovered_node;
    }

    return None;
}
