use std::collections::{HashMap, HashSet};
use preview::DockPreviewData;
use request::{DockRequest, DockRequestType};
use crate::axis::Axis;
use crate::color::StyleColor;
use crate::config::ConfigFlags;
use crate::context::Context;
use crate::types::DataAuthority::Window;
use crate::types::{DataAuthority, Id32};
use crate::types::Direction;
use crate::drag_drop::DragDropFlags;
use crate::draw::draw_list::get_foreground_draw_list;
use crate::globals::GImGui;
use crate::input::mouse::{start_mouse_moving_window, start_mouse_moving_window_or_node};
use crate::input::NavLayer;
use crate::item::ItemFlags;
use crate::orig_imgui_single_file::{int, Rect};
use crate::rect::Rect;
use crate::settings::{find_window_settings, mark_ini_settings_dirty, SettingsHandler};
use crate::style::{get_color_u32, pop_style_color, push_style_color};
use crate::types::INVALID_ID;
use crate::vectors::ImLengthSqr;
use crate::vectors::two_d::Vector2D;
use crate::{Viewport, window};
use crate::condition::Condition;
use crate::dock::context::{dock_context_add_node, dock_context_find_node_by_id, dock_context_gen_node_id, DockContext, DockContextPruneNodeData};
use crate::dock::dock_context::dock_context_remove_node;
use crate::dock::dock_node::DockNode;
use crate::dock::node::{dock_node_get_root_node, DockNode, DockNodeFlags, DockNodeSettings};
use crate::frame::get_frame_height;
use crate::platform::get_viewport_platform_monitor;
use crate::popup::PopupPositionPolicy::Default;
use crate::tab_bar::TabBar;
use crate::text_buffer::TextBuffer;
use crate::utils::{add_hash_set, get_or_add};
use crate::window::class::WindowClass;
use crate::window::next_window::NextWindowDataFlags;
use crate::window::{WindowFlags, WINDOWS_HOVER_PADDING, WINDOWS_RESIZE_FROM_EDGES_FEEDBACK_TIMER};
use crate::window::get::find_window_by_name;
use crate::window::lifecycle::update_window_parent_and_root_links;
use crate::window::Window;

pub mod node;
pub mod context;
mod dock_context;
mod request;
mod preview;


/// List of colors that are stored at the time of Begin() into Docked windows.
/// We currently store the packed colors in a simple array window->dock_style.colors[].
/// A better solution may involve appending into a log of colors in ImGuiContext + store offsets into those arrays in ImGuiWindow,
/// but it would be more complex as we'd need to double-buffer both as e.g. drop target may refer to window from last frame.
#[derive(Debug,Clone)]
pub enum WindowDockStyleColor
{
    None,
    Text,
    Tab,
    TabHovered,
    TabActive,
    TabUnfocused,
    TabUnfocusedActive,
    LastItem
}

impl Default for WindowDockStyleColor {
    fn default() -> Self {
        Self::None
    }
}


// Docking
// static const float DOCKING_TRANSPARENT_PAYLOAD_ALPHA        = 0.50;    // For use with io.config_docking_transparent_payload. Apply to viewport _or_ WindowBg in host viewport.
pub const DOCKING_TRANSPARENT_PAYLOAD_ALPHA: f32 = 0.50;

// static const float DOCKING_SPLITTER_SIZE                    = 2.0;
pub const DOCKING_SPLITTER_SIZE: f32 = 2.0;

// Problem:
//   Undocking a large (~full screen) window would leave it so large that the bottom right sizing corner would more
//   than likely be off the screen and the window would be hard to resize to fit on screen. This can be particularly problematic
//   with 'config_windows_move_from_title_bar_only=true' and/or with 'config_windows_resize_from_edges=false' as well (the later can be
//   due to missing ImGuiBackendFlags_HasMouseCursors backend flag).
// Solution:
//   When undocking a window we currently force its maximum size to 90% of the host viewport or monitor.
// Reevaluate this when we implement preserving docked/undocked size ("docking_wip/undocked_size" branch).
// static Vector2D FixLargeWindowsWhenUndocking(const Vector2D& size, ImGuiViewport* ref_viewport)
pub fn fix_large_windows_when_undocking(g: &mut Context, size: &Vector2D, ref_viewport: Option<&mut Viewport>) -> Vector2D
{
    // if (ref_viewport == NULL)
    //     return size;
    if ref_viewport.is_none() {
        return size.clone();
    }

    // ImGuiContext& g = *GImGui;
    // Vector2D max_size = f32::floor(ref_viewport.work_size * 0.90);
    let max_size = Vector2D::floor(ref_viewport.unwrap().work_size.clone() * 0.90);
    // if (g.config_flags_curr_frame & ConfigFlags::ViewportsEnable)
    if g.config_flags_curr_frame.contains(&ConfigFlags::ViewportsEnable)
    {
        // const ImGuiPlatformMonitor* monitor = GetViewportPlatformMonitor(ref_viewport);
        let monitor = get_viewport_platform_monitor(g, ref_viewport.unwrap());
        let max_size = Vector2D::floor(&monitor.work_size * 0.90);
    }
    return Vector2D::min(size, &max_size);
}

// This is mostly used for automation.
// bool DockContextCalcDropPosForDocking(ImGuiWindow* target, ImGuiDockNode* target_node, ImGuiWindow* payload, ImGuiDir split_dir, bool split_outer, Vector2D* out_pos)
pub fn dock_context_calc_drop_pos_for_docking(
    g: &mut Context,
    target: &mut Window,
    target_node: Option<&mut DockNode>,
    payload: &mut Window,
    split_dir: Direction,
    mut split_outer: bool,
    out_pos: &mut Vector2D) -> bool
{
    // In dock_node_preview_dock_setup() for a root central node instead of showing both "inner" and "outer" drop rects
    // (which would be functionally identical) we only show the outer one. Reflect this here.
    // if (target_node && target_node.parent_node == NULL && target_node.is_central_node() && split_dir != Direction::None)
    if target_node.is_some() && target_node.unwrap().parent_node_id != INVALID_ID && split_idr != Direction::None
    {
        split_outer = true;
    }
    // ImGuiDockPreviewData split_data;
    let mut split_data = DockPreviewData::default();
    dock_node_preview_dock_setup(g , target, target_node.unwrap(), payload, &mut split_data, false, split_outer);
    if split_data.drop_rects_draw[&split_dir+1].is_inverted() {
        return false;
    }   
    *out_pos = split_data.drop_rects_draw[&split_dir+1].get_center().clone();
    return true;
}

// int DockNodeGetTabOrder(ImGuiWindow* window)
pub fn dock_node_get_tab_order(g: &mut Context, window: &mut window::Window) -> i32
{
    ImGuiTabBar* tab_bar = window.dock_node_id.tab_bar;
    if (tab_bar == NULL)
        return -1;
    ImGuiTabItem* tab = TabBarFindTabByID(tab_bar, window.tab_id);
    return tab ? tab_bar.GetTabOrder(tab) : -1;
}

// static void DockNodeHideWindowDuringHostWindowCreation(ImGuiWindow* window)
pub fn dock_node_hide_window_during_host_window_creation(g: &mut Context, window: &mut window::Window)
{
    window.hidden = true;
    window..hidden_frames_can_skip_items = window.active ? 1 : 2;
}

// static void DockNodeAddWindow(ImGuiDockNode* node, ImGuiWindow* window, bool add_to_tab_bar)
pub fn dock_node_add_window(g: &mut Context, node: &mut DockNode, window: &mut window::Window, add_to_tab_bar: bool)
{
    // ImGuiContext& g = *GImGui; (void)g;
    if (window.dock_node_id)
    {
        // Can overwrite an existing window->dock_node (e.g. pointing to a disabled DockSpace node)
        // IM_ASSERT(window.dock_node.ID != node.ID);
        dock_node_remove_window(window.dock_node_id, window, 0);
    }
    // IM_ASSERT(window.dock_node == NULL || window.DockNodeAsHost == NULL);
    // IMGUI_DEBUG_LOG_DOCKING("[docking] DockNodeAddWindow node 0x%08X window '%s'\n", node.ID, window.Name);

    // If more than 2 windows appeared on the same frame leading to the creation of a new hosting window,
    // we'll hide windows until the host window is ready. Hide the 1st window after its been output (so it is not visible for one frame).
    // We will call DockNodeHideWindowDuringHostWindowCreation() on ourselves in Begin()
    if (node.host_window_id == NULL && node.windows.size == 1 && node.windows[0].WasActive == false)
        DockNodeHideWindowDuringHostWindowCreation(node.windows[0]);

    node.windows.push_back(window);
    node.want_hiddent_tab_bar_update = true;
    window.dock_node_id = node;
    window.dock_id = node.id;
    window.dock_is_active = (node.windows.size > 1);
    window.DockTabWantClose = false;

    // When reactivating a node with one or two loose window, the window pos/size/viewport are authoritative over the node storage.
    // In particular it is important we init the viewport from the first window so we don't create two viewports and drop one.
    if (node.host_window_id == NULL && node.IsFloatingNode())
    {
        if (node.authority_for_pos == DataAuthority::Auto)
            node.authority_for_pos = DataAuthority::Window;
        if (node.authority_for_size == DataAuthority::Auto)
            node.authority_for_size = DataAuthority::Window;
        if (node.authority_for_viewport == DataAuthority::Auto)
            node.authority_for_viewport = DataAuthority::Window;
    }

    // Add to tab bar if requested
    if (add_to_tab_bar)
    {
        if (node.tab_bar == NULL)
        {
            dock_node_add_tab_bar(node);
            node.tab_bar.selected_tab_id = node.tab_bar.next_selected_tab_id = node.selected_tab_id;

            // Add existing windows
            for (int n = 0; n < node.windows.size - 1; n += 1)
                tab_bar_add_tab(node.tab_bar, TabItemFlags::None, node.windows[n]);
        }
        tab_bar_add_tab(node.tab_bar, TabItemFlags::Unsorted, window);
    }

    DockNodeUpdateVisibleFlag(node);

    // Update this without waiting for the next time we Begin() in the window, so our host window will have the proper title bar color on its first frame.
    if (node.host_window_id)
        UpdateWindowParentAndRootLinks(window, window.flags | WindowFlags::ChildWindow, node.host_window_id);
}

// static void DockNodeRemoveWindow(ImGuiDockNode* node, ImGuiWindow* window, ImGuiID save_dock_id)
pub fn dock_node_remove_window(g: &mut Context, node: &mut DockNode, window: &mut window::Window, save_dock_id: Id32)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(window.dock_node == node);
    //IM_ASSERT(window->root_window_dock_tree == node->host_window);
    //IM_ASSERT(window->last_frame_active < g.frame_count);    // We may call this from Begin()
    // IM_ASSERT(save_dock_id == 0 || save_dock_id == node.ID);
    // IMGUI_DEBUG_LOG_DOCKING("[docking] DockNodeRemoveWindow node 0x%08X window '%s'\n", node.ID, window.Name);

    window.dock_node_id = NULL;
    window.dock_is_active = window.DockTabWantClose = false;
    window.dock_id = save_dock_id;
    window.flags &= ~WindowFlags::ChildWindow;
    if (window.parent_window)
        window.parent_window.DC.ChildWindows.find_erase(window);
    UpdateWindowParentAndRootLinks(window, window.flags, NULL); // Update immediately

    // Remove window
    bool erased = false;
    for (int n = 0; n < node.windows.size; n += 1)
        if (node.windows[n] == window)
        {
            node.windows.erase(node.windows.data + n);
            erased = true;
            break;
        }
    if (!erased)
        // IM_ASSERT(erased);
    if (node.visible_window == window)
        node.visible_window = NULL;

    // Remove tab and possibly tab bar
    node.want_hiddent_tab_bar_update = true;
    if (node.tab_bar)
    {
        TabBarRemoveTab(node.tab_bar, window.tab_id);
        const int tab_count_threshold_for_tab_bar = node.is_central_node() ? 1 : 2;
        if (node.windows.size < tab_count_threshold_for_tab_bar)
            DockNodeRemoveTabBar(node);
    }

    if (node.windows.size == 0 && !node.is_central_node() && !node.IsDockSpace() && window.dock_id != node.id)
    {
        // Automatic dock node delete themselves if they are not holding at least one tab
        dock_context_remove_node(&g, node, true);
        return;
    }

    if (node.windows.size == 1 && !node.is_central_node() && node.host_window_id)
    {
        ImGuiWindow* remaining_window = node.windows[0];
        if (node.host_window_id.ViewportOwned && node.is_root_node())
        {
            // Transfer viewport back to the remaining loose window
            IMGUI_DEBUG_LOG_VIEWPORT("[viewport] Node %08X transfer viewport %08X=>%08X for window '%s'\n", node.id, node.host_window_id.Viewport.id, remaining_window.id, remaining_window.Name);
            // IM_ASSERT(node.host_window.Viewport.Window == node.host_window);
            node.host_window_id.Viewport.Window = remaining_window;
            node.host_window_id.Viewport.id = remaining_window.id;
        }
        remaining_window.collapsed = node.host_window_id.collapsed;
    }

    // Update visibility immediately is required so the DockNodeUpdateRemoveInactiveChilds() processing can reflect changes up the tree
    DockNodeUpdateVisibleFlag(node);
}

// static void dock_node_move_child_nodes(ImGuiDockNode* dst_node, ImGuiDockNode* src_node)
pub fn dock_node_move_child_nodes(g: &mut Context, dst_node: &mut DockNode, src_node: &DockNode)
{
    // IM_ASSERT(dst_node.Windows.size == 0);
    dst_node.child_nodes[0] = src_node.child_nodes[0];
    dst_node.child_nodes[1] = src_node.child_nodes[1];
    if (dst_node.child_nodes[0])
        dst_node.child_nodes[0]parent_node = dst_node;
    if (dst_node.child_nodes[1])
        dst_node.child_nodes[1]parent_node = dst_node;
    dst_node.split_axis = src_node.split_axis;
    dst_node.size_ref = src_node.size_ref;
    src_node.child_nodes[0] = src_node.child_nodes[1] = NULL;
}

// static void DockNodeMoveWindows(ImGuiDockNode* dst_node, ImGuiDockNode* src_node)
pub fn dock_node_move_windows(g: &mut Context, dst_node: &mut DockNode, src_node: &mut DockNode)
{
    // Insert tabs in the same orders as currently ordered (node->windows isn't ordered)
    // IM_ASSERT(src_node && dst_node && dst_node != src_node);
    ImGuiTabBar* src_tab_bar = src_node.tab_bar;
    if (src_tab_bar != NULL)
        // IM_ASSERT(src_node.Windows.size <= src_node.TabBar.Tabs.size);

    // If the dst_node is empty we can just move the entire tab bar (to preserve selection, scrolling, etc.)
    bool move_tab_bar = (src_tab_bar != NULL) && (dst_node.tab_bar == NULL);
    if (move_tab_bar)
    {
        dst_node.tab_bar = src_node.tab_bar;
        src_node.tab_bar = NULL;
    }

    for (int n = 0; n < src_node.windows.size; n += 1)
    {
        // dock_node's tab_bar may have non-window Tabs manually appended by user
        if (ImGuiWindow* window = src_tab_bar ? src_tab_bar.tabs[n].Window : src_node.windows[n])
        {
            window.dock_node = NULL;
            window.dock_is_active = false;
            dock_node_add_window(dst_node, window, move_tab_bar ? false : true);
        }
    }
    src_node.windows.clear();

    if (!move_tab_bar && src_node.tab_bar)
    {
        if (dst_node.tab_bar)
            dst_node.tab_bar.selected_tab_id = src_node.tab_bar.selected_tab_id;
        DockNodeRemoveTabBar(src_node);
    }
}

// static void DockNodeApplyPosSizeToWindows(ImGuiDockNode* node)
pub fn dock_node_apply_pos_size_to_windows(g: &mut Context, node: &mut DockNode)
{
    for (int n = 0; n < node.windows.size; n += 1)
    {
        set_window_pos(node.windows[n], node.pos, Cond::Always); // We don't assign directly to pos because it can break the calculation of SizeContents on next frame
        SetWindowSize(node.windows[n], node.size, Cond::Always);
    }
}

// static void DockNodeHideHostWindow(ImGuiDockNode* node)
pub fn dock_node_hide_host_window(g: &mut Context, node: &mut DockNode)
{
    if (node.host_window_id)
    {
        if (node.host_window_id.dock_node_as_host == node)
            node.host_window_id.dock_node_as_host = NULL;
        node.host_window_id = NULL;
    }

    if (node.windows.size == 1)
    {
        node.visible_window = node.windows[0];
        node.windows[0].dock_is_active = false;
    }

    if (node.tab_bar)
        DockNodeRemoveTabBar(node);
}

// Search function called once by root node in DockNodeUpdate()
pub struct DockNodeTreeInfo
{
    ImGuiDockNode*      CentralNode;
    ImGuiDockNode*      FirstNodeWithWindows;
    int                 CountNodesWithWindows;
    //ImGuiWindowClass  WindowClassForMerges;

    ImGuiDockNodeTreeInfo() { memset(this, 0, sizeof(*this)); }
}

// static void DockNodeFindInfo(ImGuiDockNode* node, ImGuiDockNodeTreeInfo* info)
pub fn dock_node_find_info(g: &mut Context, node: &mut DockNode, info: &mut DockNodeTreeInfo)
{
    if (node.windows.size > 0)
    {
        if (info.FirstNodeWithWindows == NULL)
            info.FirstNodeWithWindows = node;
        info.CountNodesWithWindows += 1;
    }
    if (node.is_central_node())
    {
        // IM_ASSERT(info.CentralNode == NULL); // Should be only one
        // IM_ASSERT(node.IsLeafNode() && "If you get this assert: please submit .ini file + repro of actions leading to this.");
        info.CentralNode = node;
    }
    if (info.CountNodesWithWindows > 1 && info.CentralNode != NULL)
        return;
    if (node.child_nodes[0])
        DockNodeFindInfo(node.child_nodes[0], info);
    if (node.child_nodes[1])
        DockNodeFindInfo(node.child_nodes[1], info);
}

// static ImGuiWindow* DockNodeFindWindowByID(ImGuiDockNode* node, ImGuiID id)
pub fn dock_node_find_window_by_id(g: &mut Context, node: &mut DockNode, id: Id32) -> &mut window::Window
{
    // IM_ASSERT(id != 0);
    for (int n = 0; n < node.windows.size; n += 1)
        if (node.windows[n].id == id)
            return node.windows[n];
    return NULL;
}

// - Remove inactive windows/nodes.
// - Update visibility flag.
// static void DockNodeUpdateFlagsAndCollapse(ImGuiDockNode* node)
pub fn dock_node_update_flags_and_collapse(g: &mut Context, node: &mut DockNode)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(node.ParentNode == NULL || node.ParentNode.ChildNodes[0] == node || node.ParentNode.ChildNodes[1] == node);

    // Inherit most flags
    if (node.parent_node)
        node.SharedFlags = node.parent_node.SharedFlags & DockNodeFlags::SharedFlagsInheritMask_;

    // Recurse into children
    // There is the possibility that one of our child becoming empty will delete itself and moving its sibling contents into 'node'.
    // If 'node->ChildNode[0]' delete itself, then 'node->ChildNode[1]->windows' will be moved into 'node'
    // If 'node->ChildNode[1]' delete itself, then 'node->ChildNode[0]->windows' will be moved into 'node' and the "remove inactive windows" loop will have run twice on those windows (harmless)
    node.HasCentralNodeChild = false;
    if (node.child_nodes[0])
        DockNodeUpdateFlagsAndCollapse(node.child_nodes[0]);
    if (node.child_nodes[1])
        DockNodeUpdateFlagsAndCollapse(node.child_nodes[1]);

    // Remove inactive windows, collapse nodes
    // merge node flags overrides stored in windows
    node.LocalFlagsInWindows = DockNodeFlags::None;
    for (int window_n = 0; window_n < node.windows.size; window_n += 1)
    {
        ImGuiWindow* window = node.windows[window_n];
        // IM_ASSERT(window.dock_node == node);

        bool node_was_active = (node.last_frame_active + 1 == g.frame_count);
        bool remove = false;
        remove |= node_was_active && (window.last_frame_active + 1 < g.frame_count);
        remove |= node_was_active && (node.WantCloseAll || node.WantCloseTabId == window.tab_id) && window.HasCloseButton && !(window.flags & WindowFlags::UnsavedDocument);  // Submit all _expected_ closure from last frame
        remove |= (window.DockTabWantClose);
        if (remove)
        {
            window.DockTabWantClose = false;
            if (node.windows.size == 1 && !node.is_central_node())
            {
                DockNodeHideHostWindow(node);
                node.State = ImGuiDockNodeState_HostWindowHiddenBecauseSingleWindow;
                dock_node_remove_window(node, window, node.id); // Will delete the node so it'll be invalid on return
                return;
            }
            dock_node_remove_window(node, window, node.id);
            window_n--;
            continue;
        }

        // FIXME-DOCKING: Missing policies for conflict resolution, hence the "Experimental" tag on this.
        //node->LocalFlagsInWindow &= ~window->window_class.DockNodeFlagsOverrideClear;
        node.LocalFlagsInWindows |= window.WindowClass.DockNodeFlagsOverrideSet;
    }
    node.UpdateMergedFlags();

    // Auto-hide tab bar option
    ImGuiDockNodeFlags node_flags = node.MergedFlags;
    if (node.want_hiddent_tab_bar_update && node.windows.size == 1 && (node_flags & DockNodeFlags::AutoHideTabBar) && !node.is_hidden_tab_bar())
        node.want_hidden_tab_bar_toggle = true;
    node.want_hiddent_tab_bar_update = false;

    // Cancel toggling if we know our tab bar is enforced to be hidden at all times
    if (node.want_hidden_tab_bar_toggle && node.visible_window && (node.visible_window.WindowClass.DockNodeFlagsOverrideSet & DockNodeFlags::HiddenTabBar))
        node.want_hidden_tab_bar_toggle = false;

    // Apply toggles at a single point of the frame (here!)
    if (node.windows.size > 1)
        node.set_local_flags(node.LocalFlags & ~DockNodeFlags::HiddenTabBar);
    else if (node.want_hidden_tab_bar_toggle)
        node.set_local_flags(node.LocalFlags ^ DockNodeFlags::HiddenTabBar);
    node.want_hidden_tab_bar_toggle = false;

    DockNodeUpdateVisibleFlag(node);
}

// This is rarely called as DockNodeUpdateForRootNode() generally does it most frames.
// static void DockNodeUpdateHasCentralNodeChild(ImGuiDockNode* node)
pub fn dock_node_update_has_central_node_child(g: &mut Context, node: &mut DockNode)
{
    node.HasCentralNodeChild = false;
    if (node.child_nodes[0])
        DockNodeUpdateHasCentralNodeChild(node.child_nodes[0]);
    if (node.child_nodes[1])
        DockNodeUpdateHasCentralNodeChild(node.child_nodes[1]);
    if (node.is_root_node())
    {
        ImGuiDockNode* mark_node = node.CentralNode;
        while (mark_node)
        {
            mark_node.HasCentralNodeChild = true;
            mark_node = mark_node.parent_node;
        }
    }
}

// static void DockNodeUpdateVisibleFlag(ImGuiDockNode* node)
pub fn dock_node_update_visible_flag(g: &mut Context, node: &mut DockNode)
{
    // Update visibility flag
    bool is_visible = (node.parent_node == NULL) ? node.IsDockSpace() : node.is_central_node();
    is_visible |= (node.windows.size > 0);
    is_visible |= (node.child_nodes[0] && node.child_nodes[0].IsVisible);
    is_visible |= (node.child_nodes[1] && node.child_nodes[1].IsVisible);
    node.IsVisible = is_visible;
}

// static void DockNodeStartMouseMovingWindow(ImGuiDockNode* node, ImGuiWindow* window)
pub fn dock_node_start_mouse_moving_window(g: &mut Context, node: &mut DockNode, window: &mut window::Window)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(node.WantMouseMove == true);
    start_mouse_moving_window(window);
    g.ActiveIdClickOffset = g.io.mouse_clicked_pos[0] - node.pos;
    g.moving_window = window; // If we are docked into a non moveable root window, start_mouse_moving_window() won't set g.moving_window. Override that decision.
    nodewant_mouse_move = false;
}

// Update central_node, OnlyNodeWithWindows, LastFocusedNodeID. Copy window class.
// static void DockNodeUpdateForRootNode(ImGuiDockNode* node)
pub fn dock_node_update_for_root_node(g: &mut Context, node: &mut DockNode)
{
    DockNodeUpdateFlagsAndCollapse(node);

    // - Setup central node pointers
    // - Find if there's only a single visible window in the hierarchy (in which case we need to display a regular title bar -> FIXME-DOCK: that last part is not done yet!)
    // Cannot merge this with DockNodeUpdateFlagsAndCollapse() because FirstNodeWithWindows is found after window removal and child collapsing
    ImGuiDockNodeTreeInfo info;
    DockNodeFindInfo(node, &info);
    node.CentralNode = info.CentralNode;
    node.OnlyNodeWithWindows = (info.CountNodesWithWindows == 1) ? info.FirstNodeWithWindows : NULL;
    node.CountNodeWithWindows = info.CountNodesWithWindows;
    if (node.LastFocusedNodeId == 0 && info.FirstNodeWithWindows != NULL)
        node.LastFocusedNodeId = info.FirstNodeWithWindows.id;

    // Copy the window class from of our first window so it can be used for proper dock filtering.
    // When node has mixed windows, prioritize the class with the most constraint (docking_allow_unclassed = false) as the reference to copy.
    // FIXME-DOCK: We don't recurse properly, this code could be reworked to work from DockNodeUpdateScanRec.
    if (ImGuiDockNode* first_node_with_windows = info.FirstNodeWithWindows)
    {
        node.WindowClass = first_node_with_windows.windows[0].WindowClass;
        for (int n = 1; n < first_node_with_windows.windows.size; n += 1)
            if (first_node_with_windows.windows[n].WindowClass.DockingAllowUnclassed == false)
            {
                node.WindowClass = first_node_with_windows.windows[n].WindowClass;
                break;
            }
    }

    ImGuiDockNode* mark_node = node.CentralNode;
    while (mark_node)
    {
        mark_node.HasCentralNodeChild = true;
        mark_node = mark_node.parent_node;
    }
}

// static void DockNodeSetupHostWindow(ImGuiDockNode* node, ImGuiWindow* host_window)
pub fn dock_node_setup_host_window(g: &mut Context, node: &mut DockNode, host_window: &mut window::Window)
{
    // Remove ourselves from any previous different host window
    // This can happen if a user mistakenly does (see #4295 for details):
    //  - N+0: DockBuilderAddNode(id, 0)    // missing ImGuiDockNodeFlags_DockSpace
    //  - N+1: NewFrame()                   // will create floating host window for that node
    //  - N+1: DockSpace(id)                // requalify node as dockspace, moving host window
    if (node.host_window_id && node.host_window_id != host_window && node.host_window_id.dock_node_as_host == node)
        node.host_window_id.dock_node_as_host = NULL;

    host_window.dock_node_as_host_id = node;
    node.host_window_id = host_window;
}

// static void DockNodeUpdate(ImGuiDockNode* node)
pub fn dock_node_update(g: &mut Context, node: &mut DockNode)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(node.LastFrameActive != g.frame_count);
    node.LastFrameAlive = g.frame_count;
    node.is_bg_drawn_this_frame = false;

    node.CentralNode = node.OnlyNodeWithWindows = NULL;
    if (node.is_root_node())
        DockNodeUpdateForRootNode(node);

    // Remove tab bar if not needed
    if (node.tab_bar && node.is_no_tab_bar())
        DockNodeRemoveTabBar(node);

    // Early out for hidden root dock nodes (when all dock_id references are in inactive windows, or there is only 1 floating window holding on the dock_id)
    bool want_to_hide_host_window = false;
    if (node.IsFloatingNode())
    {
        if (node.windows.size <= 1 && node.is_leaf_node())
            if (!g.io.ConfigDockingAlwaysTabBar && (node.windows.size == 0 || !node.windows[0].WindowClass.DockingAlwaysTabBar))
                want_to_hide_host_window = true;
        if (node.CountNodeWithWindows == 0)
            want_to_hide_host_window = true;
    }
    if (want_to_hide_host_window)
    {
        if (node.windows.size == 1)
        {
            // Floating window pos/size is authoritative
            ImGuiWindow* single_window = node.windows[0];
            node.pos = single_window.pos;
            node.size = single_window.sizeFull;
            node.authority_for_pos = node.authority_for_size = node.authority_for_viewport = DataAuthority::Window;

            // Transfer focus immediately so when we revert to a regular window it is immediately selected
            if (node.host_window_id && g.nav_window == node.host_window_id)
                focus_window(single_window);
            if (node.host_window_id)
            {
                single_window.viewport = node.host_window_id.Viewport;
                single_window.viewport_id = node.host_window_id.viewport_id;
                if (node.host_window_id.ViewportOwned)
                {
                    single_window.viewport.Window = single_window;
                    single_window.viewport_owned = true;
                }
            }
        }

        DockNodeHideHostWindow(node);
        node.State = ImGuiDockNodeState_HostWindowHiddenBecauseSingleWindow;
        node.WantCloseAll = false;
        node.WantCloseTabId = 0;
        node.HasCloseButton = node.HasWindowMenuButton = false;
        node.last_frame_active = g.frame_count;

        if (nodewant_mouse_move && node.windows.size == 1)
            DockNodeStartMouseMovingWindow(node, node.windows[0]);
        return;
    }

    // In some circumstance we will defer creating the host window (so everything will be kept hidden),
    // while the expected visible window is resizing itself.
    // This is important for first-time (no ini settings restored) single window when io.config_docking_always_tab_bar is enabled,
    // otherwise the node ends up using the minimum window size. Effectively those windows will take an extra frame to show up:
    //   N+0: Begin(): window created (with no known size), node is created
    //   N+1: DockNodeUpdate(): node skip creating host window / Begin(): window size applied, not visible
    //   N+2: DockNodeUpdate(): node can create host window / Begin(): window becomes visible
    // We could remove this frame if we could reliably calculate the expected window size during node update, before the Begin() code.
    // It would require a generalization of CalcWindowExpectedSize(), probably extracting code away from Begin().
    // In reality it isn't very important as user quickly ends up with size data in .ini file.
    if (node.IsVisible && node.host_window_id == NULL && node.IsFloatingNode() && node.is_leaf_node())
    {
        // IM_ASSERT(node.Windows.size > 0);
        ImGuiWindow* ref_window = NULL;
        if (node.selected_tab_id != 0) // Note that we prune single-window-node settings on .ini loading, so this is generally 0 for them!
            ref_window = DockNodeFindWindowByID(node, node.selected_tab_id);
        if (ref_window == NULL)
            ref_window = node.windows[0];
        if (ref_window.auto_fit_frames_x > 0 || ref_window.auto_fit_frames_y > 0)
        {
            node.State = ImGuiDockNodeState_HostWindowHiddenBecauseWindowsAreResizing;
            return;
        }
    }

    const ImGuiDockNodeFlags node_flags = node.MergedFlags;

    // Decide if the node will have a close button and a window menu button
    node.HasWindowMenuButton = (node.windows.size > 0) && (node_flags & DockNodeFlags::NoWindowMenuButton) == 0;
    node.HasCloseButton = false;
    for (int window_n = 0; window_n < node.windows.size; window_n += 1)
    {
        // FIXME-DOCK: Setting dock_is_active here means that for single active window in a leaf node, dock_is_active will be cleared until the next Begin() call.
        ImGuiWindow* window = node.windows[window_n];
        node.HasCloseButton |= window.HasCloseButton;
        window.dock_is_active = (node.windows.size > 1);
    }
    if (node_flags & DockNodeFlags::NoCloseButton)
        node.HasCloseButton = false;

    // Bind or create host window
    ImGuiWindow* host_window = NULL;
    bool beginned_into_host_window = false;
    if (node.IsDockSpace())
    {
        // [Explicit root dockspace node]
        // IM_ASSERT(node.host_window);
        host_window = node.host_window_id;
    }
    else
    {
        // [Automatic root or child nodes]
        if (node.is_root_node() && node.IsVisible)
        {
            ImGuiWindow* ref_window = (node.windows.size > 0) ? node.windows[0] : NULL;

            // Sync pos
            if (node.authority_for_pos == DataAuthority::Window && ref_window)
                SetNextWindowPos(ref_window.pos);
            else if (node.authority_for_pos == DataAuthority::DockNode)
                SetNextWindowPos(node.pos);

            // Sync size
            if (node.authority_for_size == DataAuthority::Window && ref_window)
                set_next_window_size(ref_window.sizeFull);
            else if (node.authority_for_size == DataAuthority::DockNode)
                set_next_window_size(node.size);

            // Sync collapsed
            if (node.authority_for_size == DataAuthority::Window && ref_window)
                SetNextWindowCollapsed(ref_window.collapsed);

            // Sync viewport
            if (node.authority_for_viewport == DataAuthority::Window && ref_window)
                SetNextWindowViewport(ref_window.viewport_id);

            SetNextWindowClass(&node.WindowClass);

            // Begin into the host window
            char window_label[20];
            dock_node_get_host_window_title(node, window_label, IM_ARRAYSIZE(window_label));
            ImGuiWindowFlags window_flags = WindowFlags::NoScrollbar | WindowFlags::NoScrollWithMouse | WindowFlags::DockNodeHost;
            window_flags |= WindowFlags::NoFocusOnAppearing;
            window_flags |= WindowFlags::NoSavedSettings | WindowFlags::NoNavFocus | WindowFlags::NoCollapse;
            window_flags |= WindowFlags::NoTitleBar;

            SetNextWindowBgAlpha(0.0); // Don't set ImGuiWindowFlags_NoBackground because it disables borders
            push_style_var(StyleVar::WindowPadding, Vector2D::new(0, 0));
            begin(window_label, NULL, window_flags);
            pop_style_var();
            beginned_into_host_window = true;

            host_window = g.current_window;
            DockNodeSetupHostWindow(node, host_window);
            host_window.dc.cursor_pos = host_window.pos;
            node.pos = host_window.pos;
            node.size = host_window.size;

            // We set ImGuiWindowFlags_NoFocusOnAppearing because we don't want the host window to take full focus (e.g. steal nav_window)
            // But we still it bring it to the front of display. There's no way to choose this precise behavior via window flags.
            // One simple case to ponder if: window A has a toggle to create windows B/C/D. Dock B/C/D together, clear the toggle and enable it again.
            // When reappearing B/C/D will request focus and be moved to the top of the display pile, but they are not linked to the dock host window
            // during the frame they appear. The dock host window would keep its old display order, and the sorting in EndFrame would move B/C/D back
            // after the dock host window, losing their top-most status.
            if (node.host_window_id.appearing)
                BringWindowToDisplayFront(node.host_window_id);

            node.authority_for_pos = node.authority_for_size = node.authority_for_viewport = DataAuthority::Auto;
        }
        else if (node.parent_node)
        {
            node.host_window_id = host_window = node.parent_node.host_window;
            node.authority_for_pos = node.authority_for_size = node.authority_for_viewport = DataAuthority::Auto;
        }
        if (nodewant_mouse_move && node.host_window_id)
            DockNodeStartMouseMovingWindow(node, node.host_window_id);
    }

    // Update focused node (the one whose title bar is highlight) within a node tree
    if (node.is_split_node())
        // IM_ASSERT(node.TabBar == NULL);
    if (node.is_root_node())
        if (g.nav_window && g.nav_window.root_window.dock_node && g.nav_window.root_window.parent_window == host_window)
            node.LastFocusedNodeId = g.nav_window.root_window.dock_node.id;

    // Register a hit-test hole in the window unless we are currently dragging a window that is compatible with our dockspace
    ImGuiDockNode* central_node = node.CentralNode;
    const bool central_node_hole = node.is_root_node() && host_window && (node_flags & DockNodeFlags::PassthruCentralNode) != 0 && central_node != NULL && central_node.IsEmpty();
    bool central_node_hole_register_hit_test_hole = central_node_hole;
    if (central_node_hole)
        if (const ImGuiPayload* payload = GetDragDropPayload())
            if (payload.IsDataType(IMGUI_PAYLOAD_TYPE_WINDOW) && DockNodeIsDropAllowed(host_window, *(ImGuiWindow**)payload.Data))
                central_node_hole_register_hit_test_hole = false;
    if (central_node_hole_register_hit_test_hole)
    {
        // We add a little padding to match the "resize from edges" behavior and allow grabbing the splitter easily.
        // (But we only add it if there's something else on the other side of the hole, otherwise for e.g. fullscreen
        // covering passthru node we'd have a gap on the edge not covered by the hole)
        // IM_ASSERT(node.IsDockSpace()); // We cannot pass this flag without the DockSpace() api. Testing this because we also setup the hole in host_window->parent_node
        ImGuiDockNode* root_node = dock_node_get_root_node(central_node);
        Rect root_rect(root_node.pos, root_node.pos + root_node.size);
        Rect hole_rect(central_node.pos, central_node.pos + central_node.size);
        if (hole_rect.min.x > root_rect.min.x) { hole_rect.min.x += WINDOWS_HOVER_PADDING; }
        if (hole_rect.max.x < root_rect.max.x) { hole_rect.max.x -= WINDOWS_HOVER_PADDING; }
        if (hole_rect.min.y > root_rect.min.y) { hole_rect.min.y += WINDOWS_HOVER_PADDING; }
        if (hole_rect.max.y < root_rect.max.y) { hole_rect.max.y -= WINDOWS_HOVER_PADDING; }
        //GetForegroundDrawList()->add_rect(hole_rect.min, hole_rect.max, IM_COL32(255, 0, 0, 255));
        if (central_node_hole && !hole_rect.is_inverted())
        {
            SetWindowHitTestHole(host_window, hole_rect.min, hole_rect.max - hole_rect.min);
            if (host_window.parent_window)
                SetWindowHitTestHole(host_window.parent_window, hole_rect.min, hole_rect.max - hole_rect.min);
        }
    }

    // Update position/size, process and draw resizing splitters
    if (node.is_root_node() && host_window)
    {
        host_window.draw_list.channels_set_current(1);
        DockNodeTreeUpdatePosSize(node, host_window.pos, host_window.size);
        DockNodeTreeUpdateSplitter(node);
    }

    // Draw empty node background (currently can only be the Central Node)
    if (host_window && node.IsEmpty() && node.IsVisible)
    {
        host_window.draw_list.channels_set_current(0);
        node.last_bg_color = (node_flags & DockNodeFlags::PassthruCentralNode) ? 0 : get_color_u32(StyleColor::DockingEmptyBg);
        if (node.last_bg_color != 0)
            host_window.draw_list.add_rect_filled(node.pos, node.pos + node.size, node.last_bg_color);
        node.is_bg_drawn_this_frame = true;
    }

    // Draw whole dockspace background if ImGuiDockNodeFlags_PassthruCentralNode if set.
    // We need to draw a background at the root level if requested by ImGuiDockNodeFlags_PassthruCentralNode, but we will only know the correct pos/size
    // _after_ processing the resizing splitters. So we are using the draw_list channel splitting facility to submit drawing primitives out of order!
    const bool render_dockspace_bg = node.is_root_node() && host_window && (node_flags & DockNodeFlags::PassthruCentralNode) != 0;
    if (render_dockspace_bg && node.IsVisible)
    {
        host_window.draw_list.channels_set_current(0);
        if (central_node_hole)
            render_rect_filled_with_hole(host_window.draw_list, node.rect(), central_node.rect(), get_color_u32(StyleColor::WindowBg), 0.0);
        else
            host_window.draw_list.add_rect_filled(node.pos, node.pos + node.size, get_color_u32(StyleColor::WindowBg), 0.0);
    }

    // Draw and populate Tab Bar
    if (host_window)
        host_window.draw_list.channels_set_current(1);
    if (host_window && node.windows.size > 0)
    {
        DockNodeUpdateTabBar(node, host_window);
    }
    else
    {
        node.WantCloseAll = false;
        node.WantCloseTabId = 0;
        node.IsFocused = false;
    }
    if (node.tab_bar && node.tab_bar.selected_tab_id)
        node.selected_tab_id = node.tab_bar.selected_tab_id;
    else if (node.windows.size > 0)
        node.selected_tab_id = node.windows[0].id;

    // Draw payload drop target
    if (host_window && node.IsVisible)
        if (node.is_root_node() && (g.moving_window == NULL || g.moving_window.root_window_dock_tree != host_window))
            BeginDockableDragDropTarget(host_window);

    // We update this after DockNodeUpdateTabBar()
    node.last_frame_active = g.frame_count;

    // Recurse into children
    // FIXME-DOCK FIXME-OPT: Should not need to recurse into children
    if (host_window)
    {
        if (node.child_nodes[0])
            DockNodeUpdate(node.child_nodes[0]);
        if (node.child_nodes[1])
            DockNodeUpdate(node.child_nodes[1]);

        // Render outer borders last (after the tab bar)
        if (node.is_root_node())
        {
            host_window.draw_list.channels_set_current(1);
            RenderWindowOuterBorders(host_window);
        }

        // Further rendering (= hosted windows background) will be drawn on layer 0
        host_window.draw_list.channels_set_current(0);
    }

    // End host window
    if (beginned_into_host_window) //-V1020
        end();
}

// Compare TabItem nodes given the last known dock_order (will persist in .ini file as hint), used to sort tabs when multiple tabs are added on the same frame.
// static int  TabItemComparerByDockOrder(const void* lhs, const void* rhs)
pub fn tab_item_comparer_by_dock_order(g: &mut Context, lhs: &Vec<u8>, rhs: &Vec<u8>) -> i32
{
    ImGuiWindow* a = ((const ImGuiTabItem*)lhs).Window;
    ImGuiWindow* b = ((const ImGuiTabItem*)rhs).Window;
    if (int d = ((a.dock_order == -1) ? INT_MAX : a.dock_order) - ((b.dock_order == -1) ? INT_MAX : b.dock_order))
        return d;
    return (a.BeginOrderWithinContext - b.BeginOrderWithinContext);
}

// static ImGuiID DockNodeUpdateWindowMenu(ImGuiDockNode* node, ImGuiTabBar* tab_bar)
pub fn dock_node_update_window_menu(g: &mut Context, node: &mut DockNode, tab_bar: &mut TabBar) -> Id32
{
    // Try to position the menu so it is more likely to stays within the same viewport
    // ImGuiContext& g = *GImGui;
    ImGuiID ret_tab_id = INVALID_ID;
    if (g.style.window_menu_button_position == Direction::Left)
        SetNextWindowPos(Vector2D::new(node.pos.x, node.pos.y + get_frame_height()), Cond::Always, Vector2D::new(0.0, 0.0));
    else
        SetNextWindowPos(Vector2D::new(node.pos.x + node.size.x, node.pos.y + get_frame_height()), Cond::Always, Vector2D::new(1.0, 0.0));
    if (BeginPopup("#WindowMenu"))
    {
        node.IsFocused = true;
        if (tab_bar.tabs.size == 1)
        {
            if (MenuItem("Hide tab bar", NULL, node.is_hidden_tab_bar()))
                node.want_hidden_tab_bar_toggle = true;
        }
        else
        {
            for (int tab_n = 0; tab_n < tab_bar.tabs.size; tab_n += 1)
            {
                ImGuiTabItem* tab = &tab_bar.tabs[tab_n];
                if (tab.flags & TabItemFlags::Button)
                    continue;
                if (Selectable(tab_bar.GetTabName(tab), tab.id == tab_bar.selected_tab_id))
                    ret_tab_id = tab.id;
                SameLine();
                Text("   ");
            }
        }
        EndPopup();
    }
    return ret_tab_id;
}

// User helper to append/amend into a dock node tab bar. Most commonly used to add e.g. a "+" button.
// bool DockNodeBeginAmendTabBar(ImGuiDockNode* node)
pub fn dock_node_begin_amend_tab_bar(g: &mut Context, node: &mut DockNode) -> bool
{
    if (node.tab_bar == NULL || node.host_window_id == NULL)
        return false;
    if (node.MergedFlags & DockNodeFlags::KeepAliveOnly)
        return false;
    begin(node.host_window_id.Name);
    PushOverrideID(node.id);
    bool ret = BeginTabBarEx(node.tab_bar, node.tab_bar.BarRect, node.tab_bar.flags, node);
    IM_UNUSED(ret);
    // IM_ASSERT(ret);
    return true;
}

// void DockNodeEndAmendTabBar()
pub fn dock_node_end_amend_tab_bar(g: &mut Context)
{
    EndTabBar();
    PopID();
    end();
}

// static bool IsDockNodeTitleBarHighlighted(ImGuiDockNode* node, ImGuiDockNode* root_node, ImGuiWindow* host_window)
pub fn is_dock_node_title_bar_hihglighted(g: &mut Context, node: &mut DockNode, root_node: &mut DockNode, host_window: &mut window::Window) -> bool
{
    // CTRL+Tab highlight (only highlighting leaf node, not whole hierarchy)
    // ImGuiContext& g = *GImGui;
    if (g.nav_windowing_target)
        return (g.nav_windowing_target.dock_node == node);

    // FIXME-DOCKING: May want alternative to treat central node void differently? e.g. if (g.nav_window == host_window)
    if (g.nav_window && g.nav_window.root_window_for_title_bar_highlight == host_window.root_window_dock_tree && root_node.LastFocusedNodeId == node.id)
        for (ImGuiDockNode* parent_node = g.nav_window.root_window.dock_node; parent_node != NULL; parent_node = parent_node.host_window ? parent_node.host_window.root_window.dock_node : NULL)
            if ((parent_node = dock_node_get_root_node(parent_node)) == root_node)
                return true;
    return false;
}

// Submit the tab bar corresponding to a dock node and various housekeeping details.
// static void DockNodeUpdateTabBar(ImGuiDockNode* node, ImGuiWindow* host_window)
pub fn dock_node_update_tab_bar(g: &mut Context, node: &mut DockNode, host_window: &mut window::Window)
{
    // ImGuiContext& g = *GImGui;
    ImGuiStyle& style = g.style;

    const bool node_was_active = (node.last_frame_active + 1 == g.frame_count);
    const bool closed_all = node.WantCloseAll && node_was_active;
    const ImGuiID closed_one = node.WantCloseTabId && node_was_active;
    node.WantCloseAll = false;
    node.WantCloseTabId = 0;

    // Decide if we should use a focused title bar color
    bool is_focused = false;
    ImGuiDockNode* root_node = dock_node_get_root_node(node);
    if (IsDockNodeTitleBarHighlighted(node, root_node, host_window))
        is_focused = true;

    // hidden tab bar will show a triangle on the upper-left (in Begin)
    if (node.is_hidden_tab_bar() || node.is_no_tab_bar())
    {
        node.visible_window = (node.windows.size > 0) ? node.windows[0] : NULL;
        node.IsFocused = is_focused;
        if (is_focused)
            node.LastFrameFocused = g.frame_count;
        if (node.visible_window)
        {
            // Notify root of visible window (used to display title in OS task bar)
            if (is_focused || root_node.visible_window == NULL)
                root_node.visible_window = node.visible_window;
            if (node.tab_bar)
                node.tab_bar.VisibleTabId = node.visible_window.tab_id;
        }
        return;
    }

    // Move ourselves to the Menu layer (so we can be accessed by tapping Alt) + undo skip_items flag in order to draw over the title bar even if the window is collapsed
    bool backup_skip_item = host_window.skip_items;
    if (!node.IsDockSpace())
    {
        host_window.skip_items = false;
        host_window.dcnav_layer_current = NavLayer::Menu;
    }

    // Use PushOverrideID() instead of PushID() to use the node id _without_ the host window id.
    // This is to facilitate computing those id from the outside, and will affect more or less only the id of the collapse button, popup and tabs,
    // as docked windows themselves will override the stack with their own root id.
    PushOverrideID(node.id);
    ImGuiTabBar* tab_bar = node.tab_bar;
    bool tab_bar_is_recreated = (tab_bar == NULL); // Tab bar are automatically destroyed when a node gets hidden
    if (tab_bar == NULL)
    {
        dock_node_add_tab_bar(node);
        tab_bar = node.tab_bar;
    }

    ImGuiID focus_tab_id = INVALID_ID;
    node.IsFocused = is_focused;

    const ImGuiDockNodeFlags node_flags = node.MergedFlags;
    const bool has_window_menu_button = (node_flags & DockNodeFlags::NoWindowMenuButton) == 0 && (style.window_menu_button_position != Direction::None);

    // In a dock node, the Collapse Button turns into the window Menu button.
    // FIXME-DOCK FIXME-OPT: Could we recycle popups id across multiple dock nodes?
    if (has_window_menu_button && IsPopupOpen("#WindowMenu"))
    {
        if (ImGuiID tab_id = DockNodeUpdateWindowMenu(node, tab_bar))
            focus_tab_id = tab_bar.next_selected_tab_id = tab_id;
        is_focused |= node.IsFocused;
    }

    // Layout
    Rect title_bar_rect, tab_bar_rect;
    Vector2D window_menu_button_pos;
    Vector2D close_button_pos;
    DockNodeCalcTabBarLayout(node, &title_bar_rect, &tab_bar_rect, &window_menu_button_pos, &close_button_pos);

    // Submit new tabs, they will be added as Unsorted and sorted below based on relative dock_order value.
    const int tabs_count_old = tab_bar.tabs.size;
    for (int window_n = 0; window_n < node.windows.size; window_n += 1)
    {
        ImGuiWindow* window = node.windows[window_n];
        if (TabBarFindTabByID(tab_bar, window.tab_id) == NULL)
            tab_bar_add_tab(tab_bar, TabItemFlags::Unsorted, window);
    }

    // Title bar
    if (is_focused)
        node.LastFrameFocused = g.frame_count;
    ImU32 title_bar_col = get_color_u32(host_window.collapsed ? StyleColor::TitleBgCollapsed : is_focused ? StyleColor::TitleBgActive : StyleColor::TitleBg);
    ImDrawFlags rounding_flags = calc_rounding_flags_for_rect_in_rect(title_bar_rect, host_window.Rect(), DOCKING_SPLITTER_SIZE);
    host_window.draw_list.add_rect_filled(title_bar_rect.min, title_bar_rect.max, title_bar_col, host_window.WindowRounding, rounding_flags);

    // Docking/Collapse button
    if (has_window_menu_button)
    {
        if (collapse_button(host_window.get_id("#COLLAPSE"), window_menu_button_pos, node)) // == DockNodeGetWindowMenuButtonId(node)
            OpenPopup("#WindowMenu");
        if (IsItemActive())
            focus_tab_id = tab_bar.selected_tab_id;
    }

    // If multiple tabs are appearing on the same frame, sort them based on their persistent dock_order value
    int tabs_unsorted_start = tab_bar.tabs.size;
    for (int tab_n = tab_bar.tabs.size - 1; tab_n >= 0 && (tab_bar.tabs[tab_n].flags & TabItemFlags::Unsorted); tab_n--)
    {
        // FIXME-DOCK: Consider only clearing the flag after the tab has been alive for a few consecutive frames, allowing late comers to not break sorting?
        tab_bar.tabs[tab_n].flags &= ~TabItemFlags::Unsorted;
        tabs_unsorted_start = tab_n;
    }
    if (tab_bar.tabs.size > tabs_unsorted_start)
    {
        // IMGUI_DEBUG_LOG_DOCKING("[docking] In node 0x%08X: %d new appearing tabs:%s\n", node.ID, tab_bar.Tabs.size - tabs_unsorted_start, (tab_bar.Tabs.size > tabs_unsorted_start + 1) ? " (will sort)" : "");
        for (int tab_n = tabs_unsorted_start; tab_n < tab_bar.tabs.size; tab_n += 1)
            // IMGUI_DEBUG_LOG_DOCKING("[docking] - Tab '%s' Order %d\n", tab_bar.Tabs[tab_n].Window.Name, tab_bar.Tabs[tab_n].Window.dock_order);
        if (tab_bar.tabs.size > tabs_unsorted_start + 1)
            ImQsort(tab_bar.tabs.data + tabs_unsorted_start, tab_bar.tabs.size - tabs_unsorted_start, sizeof(ImGuiTabItem), TabItemComparerByDockOrder);
    }

    // Apply nav_window focus back to the tab bar
    if (g.nav_window && g.nav_window.root_window.dock_node == node)
        tab_bar.selected_tab_id = g.nav_window.root_window.id;

    // Selected newly added tabs, or persistent tab id if the tab bar was just recreated
    if (tab_bar_is_recreated && TabBarFindTabByID(tab_bar, node.selected_tab_id) != NULL)
        tab_bar.selected_tab_id = tab_bar.next_selected_tab_id = node.selected_tab_id;
    else if (tab_bar.tabs.size > tabs_count_old)
        tab_bar.selected_tab_id = tab_bar.next_selected_tab_id = tab_bar.tabs.back().Window.tab_id;

    // Begin tab bar
    ImGuiTabBarFlags tab_bar_flags = ImGuiTabBarFlags_Reorderable | ImGuiTabBarFlags_AutoSelectNewTabs; // | ImGuiTabBarFlags_NoTabListScrollingButtons);
    tab_bar_flags |= ImGuiTabBarFlags_SaveSettings | ImGuiTabBarFlags_DockNode;
    if (!host_window.collapsed && is_focused)
        tab_bar_flags |= ImGuiTabBarFlags_IsFocused;
    BeginTabBarEx(tab_bar, tab_bar_rect, tab_bar_flags, node);
    //host_window->draw_list->add_rect(tab_bar_rect.min, tab_bar_rect.max, IM_COL32(255,0,255,255));

    // Backup style colors
    Vector4D backup_style_cols[ImGuiWindowDockStyleCol_COUNT];
    for (int color_n = 0; color_n < ImGuiWindowDockStyleCol_COUNT; color_n += 1)
        backup_style_cols[color_n] = g.style.colors[GWindowDockStyleColors[color_n]];

    // Submit actual tabs
    node.visible_window = NULL;
    for (int window_n = 0; window_n < node.windows.size; window_n += 1)
    {
        ImGuiWindow* window = node.windows[window_n];
        if ((closed_all || closed_one == window.tab_id) && window.HasCloseButton && !(window.flags & WindowFlags::UnsavedDocument))
            continue;
        if (window.last_frame_active + 1 >= g.frame_count || !node_was_active)
        {
            ImGuiTabItemFlags tab_item_flags = 0;
            tab_item_flags |= window.WindowClass.TabItemFlagsOverrideSet;
            if (window.flags & WindowFlags::UnsavedDocument)
                tab_item_flags |= TabItemFlags::UnsavedDocument;
            if (tab_bar.flags & ImGuiTabBarFlags_NoCloseWithMiddleMouseButton)
                tab_item_flags |= TabItemFlags::NoCloseWithMiddleMouseButton;

            // Apply stored style overrides for the window
            for (int color_n = 0; color_n < ImGuiWindowDockStyleCol_COUNT; color_n += 1)
                g.style.colors[GWindowDockStyleColors[color_n]] = ColorConvertU32ToFloat4(window.DockStyle.colors[color_n]);

            // Note that TabItemEx() calls TabBarCalcTabID() so our tab item id will ignore the current id stack (rightly so)
            bool tab_open = true;
            TabItemEx(tab_bar, window.Name, window.HasCloseButton ? &tab_open : NULL, tab_item_flags, window);
            if (!tab_open)
                node.WantCloseTabId = window.tab_id;
            if (tab_bar.VisibleTabId == window.tab_id)
                node.visible_window = window;

            // Store last item data so it can be queried with IsItemXXX functions after the user Begin() call
            window.DockTabItemStatusFlags = g.last_item_data.status_flags;
            window.DockTabItemRect = g.last_item_data.Rect;

            // Update navigation id on menu layer
            if (g.nav_window && g.nav_window.root_window == window && (window.dc.nav_layers_active_mask & (1 << NavLayer::Menu)) == 0)
                host_window.NavLastIds[1] = window.tab_id;
        }
    }

    // Restore style colors
    for (int color_n = 0; color_n < ImGuiWindowDockStyleCol_COUNT; color_n += 1)
        g.style.colors[GWindowDockStyleColors[color_n]] = backup_style_cols[color_n];

    // Notify root of visible window (used to display title in OS task bar)
    if (node.visible_window)
        if (is_focused || root_node.visible_window == NULL)
            root_node.visible_window = node.visible_window;

    // Close button (after visible_window was updated)
    // Note that visible_window may have been overrided by CTRL+Tabbing, so visible_window->tab_id may be != from tab_bar->selected_tab_id
    const bool close_button_is_enabled = node.HasCloseButton && node.visible_window && node.visible_window.HasCloseButton;
    const bool close_button_is_visible = node.HasCloseButton;
    //const bool close_button_is_visible = close_button_is_enabled; // Most people would expect this behavior of not even showing the button (leaving a hole since we can't claim that space as other windows in the tba bar have one)
    if (close_button_is_visible)
    {
        if (!close_button_is_enabled)
        {
            push_item_flag(ItemFlags::Disabled, true);
            push_style_color(, StyleColor::Text, style.colors[StyleColor::Text] * Vector4D(1.0, 1.0, 1.0, 0.4));
        }
        if (close_button(host_window.get_id("#CLOSE"), close_button_pos))
        {
            node.WantCloseAll = true;
            for (int n = 0; n < tab_bar.tabs.size; n += 1)
                TabBarCloseTab(tab_bar, &tab_bar.tabs[n]);
        }
        //if (IsItemActive())
        //    focus_tab_id = tab_bar->selected_tab_id;
        if (!close_button_is_enabled)
        {
            pop_style_color();
            pop_item_flag();
        }
    }

    // When clicking on the title bar outside of tabs, we still focus the selected tab for that node
    // FIXME: TabItem use AllowItemOverlap so we manually perform a more specific test for now (hovered || held)
    ImGuiID title_bar_id = host_window.get_id("#TITLEBAR");
    if (g.hovered_id == 0 || g.hovered_id == title_bar_id || g.active_id == title_bar_id)
    {
        bool held;
        button_behavior(title_bar_rect, title_bar_id, NULL, &held, ButtonFlags::AllowItemOverlap);
        if (g.hovered_id == title_bar_id)
        {
            // ImGuiButtonFlags_AllowItemOverlap + SetItemAllowOverlap() required for appending into dock node tab bar,
            // otherwise dragging window will steal hovered_id and amended tabs cannot get them.
            g.last_item_data.id = title_bar_id;
            SetItemAllowOverlap();
        }
        if (held)
        {
            if (IsMouseClicked(0))
                focus_tab_id = tab_bar.selected_tab_id;

            // Forward moving request to selected window
            if (ImGuiTabItem* tab = TabBarFindTabByID(tab_bar, tab_bar.selected_tab_id))
                start_mouse_moving_window_or_node(tab.Window ? tab.Window : node.host_window, node, false);
        }
    }

    // Forward focus from host node to selected window
    //if (is_focused && g.nav_window == host_window && !g.nav_windowing_target)
    //    focus_tab_id = tab_bar->selected_tab_id;

    // When clicked on a tab we requested focus to the docked child
    // This overrides the value set by "forward focus from host node to selected window".
    if (tab_bar.next_selected_tab_id)
        focus_tab_id = tab_bar.next_selected_tab_id;

    // Apply navigation focus
    if (focus_tab_id != 0)
        if (ImGuiTabItem* tab = TabBarFindTabByID(tab_bar, focus_tab_id))
            if (tab.Window)
            {
                focus_window(tab.Window);
                nav_init_window(tab.Window, false);
            }

    EndTabBar();
    PopID();

    // Restore skip_items flag
    if (!node.IsDockSpace())
    {
        host_window.dcnav_layer_current = NavLayer::Main;
        host_window.skip_items = backup_skip_item;
    }
}

// static void DockNodeAddTabBar(ImGuiDockNode* node)
pub fn dock_node_add_tab_bar(g: &mut Context, node: &mut DockNode)
{
    // IM_ASSERT(node.TabBar == NULL);
    node.tab_bar = IM_NEW(ImGuiTabBar);
}

// static void DockNodeRemoveTabBar(ImGuiDockNode* node)
pub fn dock_node_remove_tab_bar(g: &mut Context, node: &mut DockNode)
{
    if (node.tab_bar == NULL)
        return;
    IM_DELETE(node.tab_bar);
    node.tab_bar = NULL;
}

// static bool DockNodeIsDropAllowedOne(ImGuiWindow* payload, ImGuiWindow* host_window)
pub fn dock_node_is_drop_allowed_one(g: &mut Context, payload: &mut window::Window, host_window: &mut window::Window) -> bool
{
    if (host_window.dock_node_as_host_id && host_window.dock_node_as_host_id.IsDockSpace() && payload.BeginOrderWithinContext < host_window.BeginOrderWithinContext)
        return false;

    ImGuiWindowClass* host_class = host_window.dock_node_as_host_id? &host_window.dock_node_as_host_id.WindowClass : &host_window.WindowClass;
    ImGuiWindowClass* payload_class = &payload.WindowClass;
    if (host_class.ClassId != payload_class.ClassId)
    {
        if (host_class.ClassId != 0 && host_class.DockingAllowUnclassed && payload_class.ClassId == 0)
            return true;
        if (payload_class.ClassId != 0 && payload_class.DockingAllowUnclassed && host_class.ClassId == 0)
            return true;
        return false;
    }

    // Prevent docking any window created above a popup
    // Technically we should support it (e.g. in the case of a long-lived modal window that had fancy docking features),
    // by e.g. adding a 'if (!IsWindowWithinBeginStackOf(host_window, popup_window))' test.
    // But it would requires more work on our end because the dock host windows is technically created in NewFrame()
    // and our ->ParentXXX and ->RootXXX pointers inside windows are currently mislading or lacking.
    // ImGuiContext& g = *GImGui;
    for (int i = g.open_popup_stack.size - 1; i >= 0; i--)
        if (ImGuiWindow* popup_window = g.open_popup_stack[i].Window)
            if (is_window_within_begin_stack_of(payload, popup_window))   // Payload is created from within a popup begin stack.
                return false;

    return true;
}

// static bool DockNodeIsDropAllowed(ImGuiWindow* host_window, ImGuiWindow* root_payload)
pub fn dock_node_is_drop_allowed(g: &mut Context, host_window: &mut window::Window, root_payload: &mut window::Window) -> bool
{
    if (root_payload.dock_node_as_host_id && root_payload.dock_node_as_host_id.is_split_node()) // FIXME-DOCK: Missing filtering
        return true;

    const int payload_count = root_payload.dock_node_as_host_id? root_payload.dock_node_as_host_id.windows.size : 1;
    for (int payload_n = 0; payload_n < payload_count; payload_n += 1)
    {
        ImGuiWindow* payload = root_payload.dock_node_as_host_id? root_payload.dock_node_as_host_id.windows[payload_n] : root_payload;
        if (DockNodeIsDropAllowedOne(payload, host_window))
            return true;
    }
    return false;
}

// window menu button == collapse button when not in a dock node.
// FIXME: This is similar to RenderWindowTitleBarContents(), may want to share code.
// static void DockNodeCalcTabBarLayout(const ImGuiDockNode* node, Rect* out_title_rect, Rect* out_tab_bar_rect, Vector2D* out_window_menu_button_pos, Vector2D* out_close_button_pos)
pub fn dock_node_calc_tab_bar_layout(g: &mut Context, node: &mut DockNode, out_title_rect: &mut Rect, out_tab_bar_rect: &mut Rect, out_window_menu_button_pos: &mut Vector2D, out_close_button_pos: &mut Vector2D)
{
    // ImGuiContext& g = *GImGui;
    ImGuiStyle& style = g.style;

    Rect r = Rect(node.pos.x, node.pos.y, node.pos.x + node.size.x, node.pos.y + g.font_size + g.style.frame_padding.y * 2.0);
    if (out_title_rect) { *out_title_rect = r; }

    r.min.x += style.WindowBorderSize;
    r.max.x -= style.WindowBorderSize;

    float button_sz = g.font_size;

    Vector2D window_menu_button_pos = r.min;
    r.min.x += style.frame_padding.x;
    r.max.x -= style.frame_padding.x;
    if (node.HasCloseButton)
    {
        r.max.x -= button_sz;
        if (out_close_button_pos) *out_close_button_pos = Vector2D::new(r.max.x - style.frame_padding.x, r.min.y);
    }
    if (node.HasWindowMenuButton && style.window_menu_button_position == Direction::Left)
    {
        r.min.x += button_sz + style.item_inner_spacing.x;
    }
    else if (node.HasWindowMenuButton && style.window_menu_button_position == Direction::Right)
    {
        r.max.x -= button_sz + style.frame_padding.x;
        window_menu_button_pos = Vector2D::new(r.max.x, r.min.y);
    }
    if (out_tab_bar_rect) { *out_tab_bar_rect = r; }
    if (out_window_menu_button_pos) { *out_window_menu_button_pos = window_menu_button_pos; }
}

// void DockNodeCalcSplitRects(Vector2D& pos_old, Vector2D& size_old, Vector2D& pos_new, Vector2D& size_new, ImGuiDir dir, Vector2D size_new_desired)
pub fn dock_node_calc_split_rects(g: &mut Context, pos_old: &mut Vector2D, size_old: &mut Vector2D, size_new: &mut Vector2D, dir: Direction, size_new_desired: Vector2D)
{
    // ImGuiContext& g = *GImGui;
    const float dock_spacing = g.style.item_inner_spacing.x;
    const ImGuiAxis axis = (dir == Direction::Left || dir == Direction::Right) ? Axis::X : Axis::Y;
    pos_new[axis ^ 1] = pos_old[axis ^ 1];
    size_new[axis ^ 1] = size_old[axis ^ 1];

    // Distribute size on given axis (with a desired size or equally)
    const float w_avail = size_old[axis] - dock_spacing;
    if (size_new_desired[axis] > 0.0 && size_new_desired[axis] <= w_avail * 0.5)
    {
        size_new[axis] = size_new_desired[axis];
        size_old[axis] = f32::floor(w_avail - size_new[axis]);
    }
    else
    {
        size_new[axis] = f32::floor(w_avail * 0.5);
        size_old[axis] = f32::floor(w_avail - size_new[axis]);
    }

    // Position each node
    if (dir == Direction::Right || dir == Direction::Down)
    {
        pos_new[axis] = pos_old[axis] + size_old[axis] + dock_spacing;
    }
    else if (dir == Direction::Left || dir == Direction::Up)
    {
        pos_new[axis] = pos_old[axis];
        pos_old[axis] = pos_new[axis] + size_new[axis] + dock_spacing;
    }
}

// Retrieve the drop rectangles for a given direction or for the center + perform hit testing.
// bool DockNodeCalcDropRectsAndTestMousePos(const Rect& parent, ImGuiDir dir, Rect& out_r, bool outer_docking, Vector2D* test_mouse_pos)
pub fn dock_node_calc_drop_rects_and_test_mouse_pos(g: &mut Context, parent: &Rect, dir: &mut Direction, out_r: &mut Rect, outer_docking: bool, test_mouse_pos: &mut Vector2D) -> bool
{
    // ImGuiContext& g = *GImGui;

    const float parent_smaller_axis = ImMin(parent.get_width(), parent.get_height());
    const float hs_for_central_nodes = ImMin(g.font_size * 1.5, ImMax(g.font_size * 0.5, parent_smaller_axis / 8.0));
    float hs_w; // Half-size, longer axis
    float hs_h; // Half-size, smaller axis
    Vector2D off; // Distance from edge or center
    if (outer_docking)
    {
        //hs_w = f32::floor(ImClamp(parent_smaller_axis - hs_for_central_nodes * 4.0, g.font_size * 0.5, g.font_size * 8.0));
        //hs_h = f32::floor(hs_w * 0.15);
        //off = Vector2D(f32::floor(parent.get_width() * 0.5 - GetFrameHeightWithSpacing() * 1.4 - hs_h), f32::floor(parent.get_height() * 0.5 - GetFrameHeightWithSpacing() * 1.4 - hs_h));
        hs_w = f32::floor(hs_for_central_nodes * 1.50);
        hs_h = f32::floor(hs_for_central_nodes * 0.80);
        off = Vector2D::new(f32::floor(parent.get_width() * 0.5 - hs_h), f32::floor(parent.get_height() * 0.5 - hs_h));
    }
    else
    {
        hs_w = f32::floor(hs_for_central_nodes);
        hs_h = f32::floor(hs_for_central_nodes * 0.90);
        off = Vector2D::new(f32::floor(hs_w * 2.40), f32::floor(hs_w * 2.40));
    }

    Vector2D c = f32::floor(parent.get_center());
    if      (dir == Direction::None)  { out_r = Rect(c.x - hs_w, c.y - hs_w,         c.x + hs_w, c.y + hs_w);         }
    else if (dir == Direction::Up)    { out_r = Rect(c.x - hs_w, c.y - off.y - hs_h, c.x + hs_w, c.y - off.y + hs_h); }
    else if (dir == Direction::Down)  { out_r = Rect(c.x - hs_w, c.y + off.y - hs_h, c.x + hs_w, c.y + off.y + hs_h); }
    else if (dir == Direction::Left)  { out_r = Rect(c.x - off.x - hs_h, c.y - hs_w, c.x - off.x + hs_h, c.y + hs_w); }
    else if (dir == Direction::Right) { out_r = Rect(c.x + off.x - hs_h, c.y - hs_w, c.x + off.x + hs_h, c.y + hs_w); }

    if (test_mouse_pos == NULL)
        return false;

    Rect hit_r = out_r;
    if (!outer_docking)
    {
        // Custom hit testing for the 5-way selection, designed to reduce flickering when moving diagonally between sides
        hit_r.Expand(f32::floor(hs_w * 0.30));
        Vector2D mouse_delta = (*test_mouse_pos - c);
        float mouse_delta_len2 = ImLengthSqr(mouse_delta);
        float r_threshold_center = hs_w * 1.4;
        float r_threshold_sides = hs_w * (1.4 + 1.2);
        if (mouse_delta_len2 < r_threshold_center * r_threshold_center)
            return (dir == Direction::None);
        if (mouse_delta_len2 < r_threshold_sides * r_threshold_sides)
            return (dir == ImGetDirQuadrantFromDelta(mouse_delta.x, mouse_delta.y));
    }
    return hit_r.Contains(*test_mouse_pos);
}

// host_node may be NULL if the window doesn't have a dock_node already.
// FIXME-DOCK: This is misnamed since it's also doing the filtering.
// static void dock_node_preview_dock_setup(ImGuiWindow* host_window, ImGuiDockNode* host_node, ImGuiWindow* root_payload, ImGuiDockPreviewData* data, bool is_explicit_target, bool is_outer_docking)
pub fn dock_node_preview_dock_setup(g: &mut Context, host_window: &mut window::Window, host_node: &mut DockNode, root_payload: &mut window::Window, data: &mut DockPreviewData, is_explicit_target: bool, is_outer_docking: bool)
{
    // ImGuiContext& g = *GImGui;

    // There is an edge case when docking into a dockspace which only has inactive nodes.
    // In this case DockNodeTreeFindNodeByPos() will have selected a leaf node which is inactive.
    // Because the inactive leaf node doesn't have proper pos/size yet, we'll use the root node as reference.
    ImGuiDockNode* root_payload_as_host = root_payload.dock_node_as_host_id;
    ImGuiDockNode* ref_node_for_rect = (host_node && !host_node.IsVisible) ? dock_node_get_root_node(host_node) : host_node;
    if (ref_node_for_rect)
        // IM_ASSERT(ref_node_for_rect.IsVisible == true);

    // Filter, figure out where we are allowed to dock
    ImGuiDockNodeFlags src_node_flags = root_payload_as_host ? root_payload_as_host.MergedFlags : root_payload.WindowClass.DockNodeFlagsOverrideSet;
    ImGuiDockNodeFlags dst_node_flags = host_node ? host_node.MergedFlags : host_window.WindowClass.DockNodeFlagsOverrideSet;
    data.IsCenterAvailable = true;
    if (is_outer_docking)
        data.IsCenterAvailable = false;
    else if (dst_node_flags & DockNodeFlags::NoDocking)
        data.IsCenterAvailable = false;
    else if (host_node && (dst_node_flags & DockNodeFlags::NoDockingInCentralNode) && host_node.is_central_node())
        data.IsCenterAvailable = false;
    else if ((!host_node || !host_node.IsEmpty()) && root_payload_as_host && root_payload_as_host.is_split_node() && (root_payload_as_host.OnlyNodeWithWindows == NULL)) // Is _visibly_ split?
        data.IsCenterAvailable = false;
    else if (dst_node_flags & DockNodeFlags::NoDockingOverMe)
        data.IsCenterAvailable = false;
    else if ((src_node_flags & DockNodeFlags::NoDockingOverOther) && (!host_node || !host_node.IsEmpty()))
        data.IsCenterAvailable = false;
    else if ((src_node_flags & DockNodeFlags::NoDockingOverEmpty) && host_node && host_node.IsEmpty())
        data.IsCenterAvailable = false;

    data.IsSidesAvailable = true;
    if ((dst_node_flags & DockNodeFlags::NoSplit) || g.io.ConfigDockingNoSplit)
        data.IsSidesAvailable = false;
    else if (!is_outer_docking && host_node && host_node.parent_node == NULL && host_node.is_central_node())
        data.IsSidesAvailable = false;
    else if ((dst_node_flags & DockNodeFlags::NoDockingSplitMe) || (src_node_flags & DockNodeFlags::NoDockingSplitOther))
        data.IsSidesAvailable = false;

    // build a tentative future node (reuse same structure because it is practical. Shape will be readjusted when previewing a split)
    data.FutureNode.HasCloseButton = (host_node ? host_node.HasCloseButton : host_window.HasCloseButton) || (root_payload.HasCloseButton);
    data.FutureNode.HasWindowMenuButton = host_node ? true : ((host_window.flags & WindowFlags::NoCollapse) == 0);
    data.FutureNode.pos = ref_node_for_rect ? ref_node_for_rect.pos : host_window.pos;
    data.FutureNode.size = ref_node_for_rect ? ref_node_for_rect.size : host_window.size;

    // Calculate drop shapes geometry for allowed splitting directions
    // IM_ASSERT(Dir::None == -1);
    data.SplitNode = host_node;
    data.SplitDir = Direction::None;
    data.IsSplitDirExplicit = false;
    if (!host_window.collapsed)
        for (int dir = Direction::None; dir < Direction::COUNT; dir += 1)
        {
            if (dir == Direction::None && !data.IsCenterAvailable)
                continue;
            if (dir != Direction::None && !data.IsSidesAvailable)
                continue;
            if (DockNodeCalcDropRectsAndTestMousePos(data.FutureNode.Rect(), (ImGuiDir)dir, data.drop_rects_draw[dir+1], is_outer_docking, &g.io.mouse_pos))
            {
                data.SplitDir = (ImGuiDir)dir;
                data.IsSplitDirExplicit = true;
            }
        }

    // When docking without holding Shift, we only allow and preview docking when hovering over a drop rect or over the title bar
    data.IsDropAllowed = (data.SplitDir != Direction::None) || (data.IsCenterAvailable);
    if (!is_explicit_target && !data.IsSplitDirExplicit && !g.io.ConfigDockingWithShift)
        data.IsDropAllowed = false;

    // Calculate split area
    data.SplitRatio = 0.0;
    if (data.SplitDir != Direction::None)
    {
        ImGuiDir split_dir = data.SplitDir;
        ImGuiAxis split_axis = (split_dir == Direction::Left || split_dir == Direction::Right) ? Axis::X : Axis::Y;
        Vector2D pos_new, pos_old = data.FutureNode.pos;
        Vector2D size_new, size_old = data.FutureNode.size;
        DockNodeCalcSplitRects(pos_old, size_old, pos_new, size_new, split_dir, root_payload.size);

        // Calculate split ratio so we can pass it down the docking request
        float split_ratio = ImSaturate(size_new[split_axis] / data.FutureNode.size[split_axis]);
        data.FutureNode.pos = pos_new;
        data.FutureNode.size = size_new;
        data.SplitRatio = (split_dir == Direction::Right || split_dir == Direction::Down) ? (1.0 - split_ratio) : (split_ratio);
    }
}

// static void DockNodePreviewDockRender(ImGuiWindow* host_window, ImGuiDockNode* host_node, ImGuiWindow* root_payload, const ImGuiDockPreviewData* data)
pub fn dock_node_preview_dock_render(g: &mut Context, host_window: &mut host_window, host_node: &mut DockNode, root_payload: &mut window::Window, data: &mut DockPreviewData)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.current_window == host_window);   // Because we rely on font size to calculate tab sizes

    // With this option, we only display the preview on the target viewport, and the payload viewport is made transparent.
    // To compensate for the single layer obstructed by the payload, we'll increase the alpha of the preview nodes.
    const bool is_transparent_payload = g.io.config_docking_transparent_payload;

    // In case the two windows involved are on different viewports, we will draw the overlay on each of them.
    int overlay_draw_lists_count = 0;
    ImDrawList* overlay_draw_lists[2];
    overlay_draw_lists[overlay_draw_lists_count += 1] = get_foreground_draw_list(host_window.viewport);
    if (host_window.viewport != root_payload.Viewport && !is_transparent_payload)
        overlay_draw_lists[overlay_draw_lists_count += 1] = get_foreground_draw_list(root_payload.Viewport);

    // Draw main preview rectangle
    const ImU32 overlay_col_main = get_color_u32(StyleColor::DockingPreview, is_transparent_payload ? 0.60 : 0.40);
    const ImU32 overlay_col_drop = get_color_u32(StyleColor::DockingPreview, is_transparent_payload ? 0.90 : 0.70);
    const ImU32 overlay_col_drop_hovered = get_color_u32(StyleColor::DockingPreview, is_transparent_payload ? 1.20 : 1.00);
    const ImU32 overlay_col_lines = get_color_u32(StyleColor::NavWindowingHighlight, is_transparent_payload ? 0.80 : 0.60);

    // Display area preview
    const bool can_preview_tabs = (root_payload.dock_node_as_host_id == NULL || root_payload.dock_node_as_host_id.windows.size > 0);
    if (data.IsDropAllowed)
    {
        Rect overlay_rect = data.FutureNode.Rect();
        if (data.SplitDir == Direction::None && can_preview_tabs)
            overlay_rect.min.y += get_frame_height();
        if (data.SplitDir != Direction::None || data.IsCenterAvailable)
            for (int overlay_n = 0; overlay_n < overlay_draw_lists_count; overlay_n += 1)
                overlay_draw_lists[overlay_n].add_rect_filled(overlay_rect.min, overlay_rect.max, overlay_col_main, host_window.WindowRounding, calc_rounding_flags_for_rect_in_rect(overlay_rect, host_window.Rect(), DOCKING_SPLITTER_SIZE));
    }

    // Display tab shape/label preview unless we are splitting node (it generally makes the situation harder to read)
    if (data.IsDropAllowed && can_preview_tabs && data.SplitDir == Direction::None && data.IsCenterAvailable)
    {
        // Compute target tab bar geometry so we can locate our preview tabs
        Rect tab_bar_rect;
        DockNodeCalcTabBarLayout(&data.FutureNode, NULL, &tab_bar_rect, NULL, NULL);
        Vector2D tab_pos = tab_bar_rect.min;
        if (host_node && host_node.tab_bar)
        {
            if (!host_node.is_hidden_tab_bar() && !host_node.is_no_tab_bar())
                tab_pos.x += host_node.tab_bar.WidthAllTabs + g.style.item_inner_spacing.x; // We don't use OffsetNewTab because when using non-persistent-order tab bar it is incremented with each Tab submission.
            else
                tab_pos.x += g.style.item_inner_spacing.x + TabItemCalcSize(host_node.windows[0].Name, host_node.windows[0].HasCloseButton).x;
        }
        else if (!(host_window.flags & WindowFlags::DockNodeHost))
        {
            tab_pos.x += g.style.item_inner_spacing.x + TabItemCalcSize(host_window.Name, host_window.HasCloseButton).x; // Account for slight offset which will be added when changing from title bar to tab bar
        }

        // Draw tab shape/label preview (payload may be a loose window or a host window carrying multiple tabbed windows)
        if (root_payload.dock_node_as_host_id)
            // IM_ASSERT(root_payload.DockNodeAsHost.Windows.size <= root_payload.DockNodeAsHost.TabBar.Tabs.size);
        ImGuiTabBar* tab_bar_with_payload = root_payload.dock_node_as_host_id? root_payload.dock_node_as_host_id.tab_bar : NULL;
        const int payload_count = tab_bar_with_payload ? tab_bar_with_payload.tabs.size : 1;
        for (int payload_n = 0; payload_n < payload_count; payload_n += 1)
        {
            // dock_node's tab_bar may have non-window Tabs manually appended by user
            ImGuiWindow* payload_window = tab_bar_with_payload ? tab_bar_with_payload.tabs[payload_n].Window : root_payload;
            if (tab_bar_with_payload && payload_window == NULL)
                continue;
            if (!DockNodeIsDropAllowedOne(payload_window, host_window))
                continue;

            // Calculate the tab bounding box for each payload window
            Vector2D tab_size = TabItemCalcSize(payload_window.Name, payload_window.HasCloseButton);
            Rect tab_bb(tab_pos.x, tab_pos.y, tab_pos.x + tab_size.x, tab_pos.y + tab_size.y);
            tab_pos.x += tab_size.x + g.style.item_inner_spacing.x;
            const ImU32 overlay_col_text = get_color_u32(payload_window.DockStyle.colors[ImGuiWindowDockStyleCol_Text]);
            const ImU32 overlay_col_tabs = get_color_u32(payload_window.DockStyle.colors[ImGuiWindowDockStyleCol_TabActive]);
            push_style_color(, StyleColor::Text, overlay_col_text);
            for (int overlay_n = 0; overlay_n < overlay_draw_lists_count; overlay_n += 1)
            {
                ImGuiTabItemFlags tab_flags = TabItemFlags::Preview | ((payload_window.flags & WindowFlags::UnsavedDocument) ? TabItemFlags::UnsavedDocument : 0);
                if (!tab_bar_rect.Contains(tab_bb))
                    overlay_draw_lists[overlay_n].push_clip_rect(tab_bar_rect.min, tab_bar_rect.max);
                TabItemBackground(overlay_draw_lists[overlay_n], tab_bb, tab_flags, overlay_col_tabs);
                TabItemLabelAndCloseButton(overlay_draw_lists[overlay_n], tab_bb, tab_flags, g.style.frame_padding, payload_window.Name, 0, 0, false, NULL, NULL);
                if (!tab_bar_rect.Contains(tab_bb))
                    overlay_draw_lists[overlay_n].pop_clip_rect();
            }
            pop_style_color();
        }
    }

    // Display drop boxes
    const float overlay_rounding = ImMax(3.0, g.style.FrameRounding);
    for (int dir = Direction::None; dir < Direction::COUNT; dir += 1)
    {
        if (!data.drop_rects_draw[dir + 1].is_inverted())
        {
            Rect draw_r = data.drop_rects_draw[dir + 1];
            Rect draw_r_in = draw_r;
            draw_r_in.Expand(-2.0);
            ImU32 overlay_col = (data.SplitDir == (ImGuiDir)dir && data.IsSplitDirExplicit) ? overlay_col_drop_hovered : overlay_col_drop;
            for (int overlay_n = 0; overlay_n < overlay_draw_lists_count; overlay_n += 1)
            {
                Vector2D center = f32::floor(draw_r_in.get_center());
                overlay_draw_lists[overlay_n].add_rect_filled(draw_r.min, draw_r.max, overlay_col, overlay_rounding);
                overlay_draw_lists[overlay_n].AddRect(draw_r_in.min, draw_r_in.max, overlay_col_lines, overlay_rounding);
                if (dir == Direction::Left || dir == Direction::Right)
                    overlay_draw_lists[overlay_n].add_line(Vector2D::new(center.x, draw_r_in.min.y), Vector2D::new(center.x, draw_r_in.max.y), overlay_col_lines);
                if (dir == Direction::Up || dir == Direction::Down)
                    overlay_draw_lists[overlay_n].add_line(Vector2D::new(draw_r_in.min.x, center.y), Vector2D::new(draw_r_in.max.x, center.y), overlay_col_lines);
            }
        }

        // Stop after ImGuiDir_None
        if ((host_node && (host_node.MergedFlags & DockNodeFlags::NoSplit)) || g.io.ConfigDockingNoSplit)
            return;
    }
}

// void DockNodeTreeSplit(ImGuiContext* ctx, ImGuiDockNode* parent_node, ImGuiAxis split_axis, int split_inheritor_child_idx, float split_ratio, ImGuiDockNode* new_node)
pub fn dock_node_tree_split(g: &mut Context, parent_node: &mut DockNode, split_axis: Axis, split_inheritor_child_idx: i32, split_ratio: f32, new_node: &mut DockNode)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(split_axis != ImGuiAxis_None);

    ImGuiDockNode* child_0 = (new_node && split_inheritor_child_idx != 0) ? new_node : dock_context_add_node(.g, 0);
    child_0parent_node = parent_node;

    ImGuiDockNode* child_1 = (new_node && split_inheritor_child_idx != 1) ? new_node : dock_context_add_node(.g, 0);
    child_1parent_node = parent_node;

    ImGuiDockNode* child_inheritor = (split_inheritor_child_idx == 0) ? child_0 : child_1;
    dock_node_move_child_nodes(child_inheritor, parent_node);
    parent_node.child_nodes[0] = child_0;
    parent_node.child_nodes[1] = child_1;
    parent_node.child_nodes[split_inheritor_child_idx].visible_window = parent_node.visible_window;
    parent_node.split_axis = split_axis;
    parent_node.visible_window = NULL;
    parent_node.authority_for_pos = parent_node.authority_for_size = DataAuthority::DockNode;

    float size_avail = (parent_node.size[split_axis] - DOCKING_SPLITTER_SIZE);
    size_avail = ImMax(size_avail, g.style.window_min_size[split_axis] * 2.0);
    // IM_ASSERT(size_avail > 0.0); // If you created a node manually with DockBuilderAddNode(), you need to also call DockBuilderSetNodeSize() before splitting.
    child_0.size_ref = child_1.size_ref = parent_node.size;
    child_0.size_ref[split_axis] = f32::floor(size_avail * split_ratio);
    child_1.size_ref[split_axis] = f32::floor(size_avail - child_0.size_ref[split_axis]);

    dock_node_move_windows(parent_node.child_nodes[split_inheritor_child_idx], parent_node);
    dock_settings_rename_node_references(parent_node.id, parent_node.child_nodes[split_inheritor_child_idx].id);
    DockNodeUpdateHasCentralNodeChild(dock_node_get_root_node(parent_node));
    DockNodeTreeUpdatePosSize(parent_node, parent_node.pos, parent_node.size);

    // flags transfer (e.g. this is where we transfer the ImGuiDockNodeFlags_CentralNode property)
    child_0.SharedFlags = parent_node.SharedFlags & DockNodeFlags::SharedFlagsInheritMask_;
    child_1.SharedFlags = parent_node.SharedFlags & DockNodeFlags::SharedFlagsInheritMask_;
    child_inheritor.LocalFlags = parent_node.LocalFlags & DockNodeFlags::LocalFlagsTransferMask_;
    parent_node.LocalFlags &= ~DockNodeFlags::LocalFlagsTransferMask_;
    child_0.UpdateMergedFlags();
    child_1.UpdateMergedFlags();
    parent_node.UpdateMergedFlags();
    if (child_inheritor.is_central_node())
        dock_node_get_root_node(parent_node).CentralNode = child_inheritor;
}

// void DockNodeTreeMerge(ImGuiContext* ctx, ImGuiDockNode* parent_node, ImGuiDockNode* merge_lead_child)
pub fn dock_node_tree_merge(g: &mut Context, parent_node: &mut DockNode, merge_lead_child: Option<&mut DockNode>)
{
    // When called from DockContextProcessUndockNode() it is possible that one of the child is NULL.
    // ImGuiContext& g = *GImGui;
    ImGuiDockNode* child_0 = parent_node.child_nodes[0];
    ImGuiDockNode* child_1 = parent_node.child_nodes[1];
    // IM_ASSERT(child_0 || child_1);
    // IM_ASSERT(merge_lead_child == child_0 || merge_lead_child == child_1);
    if ((child_0 && child_0.windows.size > 0) || (child_1 && child_1.windows.size > 0))
    {
        // IM_ASSERT(parent_node.TabBar == NULL);
        // IM_ASSERT(parent_node.Windows.size == 0);
    }
    // IMGUI_DEBUG_LOG_DOCKING("[docking] DockNodeTreeMerge: 0x%08X + 0x%08X back into parent 0x%08X\n", child_0 ? child_0.ID : 0, child_1 ? child_1.ID : 0, parent_node.ID);

    Vector2D backup_last_explicit_size = parent_node.size_ref;
    dock_node_move_child_nodes(parent_node, merge_lead_child);
    if (child_0)
    {
        dock_node_move_windows(parent_node, child_0); // Generally only 1 of the 2 child node will have windows
        dock_settings_rename_node_references(child_0.id, parent_node.id);
    }
    if (child_1)
    {
        dock_node_move_windows(parent_node, child_1);
        dock_settings_rename_node_references(child_1.id, parent_node.id);
    }
    DockNodeApplyPosSizeToWindows(parent_node);
    parent_node.authority_for_pos = parent_node.authority_for_size = parent_node.authority_for_viewport = DataAuthority::Auto;
    parent_node.visible_window = merge_lead_child.visible_window;
    parent_node.size_ref = backup_last_explicit_size;

    // flags transfer
    parent_node.LocalFlags &= ~DockNodeFlags::LocalFlagsTransferMask_; // Preserve Dockspace flag
    parent_node.LocalFlags |= (child_0 ? child_0.LocalFlags : 0) & DockNodeFlags::LocalFlagsTransferMask_;
    parent_node.LocalFlags |= (child_1 ? child_1.LocalFlags : 0) & DockNodeFlags::LocalFlagsTransferMask_;
    parent_node.LocalFlagsInWindows = (child_0 ? child_0.LocalFlagsInWindows : 0) | (child_1 ? child_1.LocalFlagsInWindows : 0); // FIXME: Would be more consistent to update from actual windows
    parent_node.UpdateMergedFlags();

    if (child_0)
    {
        .g.dock_context.Nodes.SetVoidPtr(child_0.id, NULL);
        IM_DELETE(child_0);
    }
    if (child_1)
    {
        .g.dock_context.Nodes.SetVoidPtr(child_1.id, NULL);
        IM_DELETE(child_1);
    }
}

// Update pos/size for a node hierarchy (don't affect child windows yet)
// (Depth-first, Pre-Order)
// void DockNodeTreeUpdatePosSize(ImGuiDockNode* node, Vector2D pos, Vector2D size, ImGuiDockNode* only_write_to_single_node)
pub fn dock_node_tree_update_pos_size(g: &mut Context, node: &mut DockNode, pos: Vector2D, size: Vector2D, only_write_to_single_node: &mut DockNode)
{
    // During the regular dock node update we write to all nodes.
    // 'only_write_to_single_node' is only set when turning a node visible mid-frame and we need its size right-away.
    const bool write_to_node = only_write_to_single_node == NULL || only_write_to_single_node == node;
    if (write_to_node)
    {
        node.pos = pos;
        node.size = size;
    }

    if (node.is_leaf_node())
        return;

    ImGuiDockNode* child_0 = node.child_nodes[0];
    ImGuiDockNode* child_1 = node.child_nodes[1];
    Vector2D child_0_pos = pos, child_1_pos = pos;
    Vector2D child_0_size = size, child_1_size = size;

    const bool child_0_is_toward_single_node = (only_write_to_single_node != NULL && DockNodeIsInHierarchyOf(only_write_to_single_node, child_0));
    const bool child_1_is_toward_single_node = (only_write_to_single_node != NULL && DockNodeIsInHierarchyOf(only_write_to_single_node, child_1));
    const bool child_0_is_or_will_be_visible = child_0.IsVisible || child_0_is_toward_single_node;
    const bool child_1_is_or_will_be_visible = child_1.IsVisible || child_1_is_toward_single_node;

    if (child_0_is_or_will_be_visible && child_1_is_or_will_be_visible)
    {
        // ImGuiContext& g = *GImGui;
        const float spacing = DOCKING_SPLITTER_SIZE;
        const ImGuiAxis axis = node.split_axis;
        const float size_avail = ImMax(size[axis] - spacing, 0.0);

        // size allocation policy
        // 1) The first 0..WindowMinSize[axis]*2 are allocated evenly to both windows.
        const float size_min_each = f32::floor(ImMin(size_avail, g.style.window_min_size[axis] * 2.0) * 0.5);

        // FIXME: Blocks 2) and 3) are essentially doing nearly the same thing.
        // Difference are: write-back to size_ref; application of a minimum size; rounding before f32::floor()
        // Clarify and rework differences between size & size_ref and purpose of WantLockSizeOnce

        // 2) Process locked absolute size (during a splitter resize we preserve the child of nodes not touching the splitter edge)
        if (child_0.WantLockSizeOnce && !child_1.WantLockSizeOnce)
        {
            child_0_size[axis] = child_0.size_ref[axis] = ImMin(size_avail - 1.0, child_0.size[axis]);
            child_1_size[axis] = child_1.size_ref[axis] = (size_avail - child_0_size[axis]);
            // IM_ASSERT(child_0.sizeRef[axis] > 0.0 && child_1.sizeRef[axis] > 0.0);
        }
        else if (child_1.WantLockSizeOnce && !child_0.WantLockSizeOnce)
        {
            child_1_size[axis] = child_1.size_ref[axis] = ImMin(size_avail - 1.0, child_1.size[axis]);
            child_0_size[axis] = child_0.size_ref[axis] = (size_avail - child_1_size[axis]);
            // IM_ASSERT(child_0.sizeRef[axis] > 0.0 && child_1.sizeRef[axis] > 0.0);
        }
        else if (child_0.WantLockSizeOnce && child_1.WantLockSizeOnce)
        {
            // FIXME-DOCK: We cannot honor the requested size, so apply ratio.
            // Currently this path will only be taken if code programmatically sets WantLockSizeOnce
            float split_ratio = child_0_size[axis] / (child_0_size[axis] + child_1_size[axis]);
            child_0_size[axis] = child_0.size_ref[axis] = f32::floor(size_avail * split_ratio);
            child_1_size[axis] = child_1.size_ref[axis] = (size_avail - child_0_size[axis]);
            // IM_ASSERT(child_0.sizeRef[axis] > 0.0 && child_1.sizeRef[axis] > 0.0);
        }

        // 3) If one window is the central node (~ use remaining space, should be made explicit!), use explicit size from the other, and remainder for the central node
        else if (child_0.size_ref[axis] != 0.0 && child_1.HasCentralNodeChild)
        {
            child_0_size[axis] = ImMin(size_avail - size_min_each, child_0.size_ref[axis]);
            child_1_size[axis] = (size_avail - child_0_size[axis]);
        }
        else if (child_1.size_ref[axis] != 0.0 && child_0.HasCentralNodeChild)
        {
            child_1_size[axis] = ImMin(size_avail - size_min_each, child_1.size_ref[axis]);
            child_0_size[axis] = (size_avail - child_1_size[axis]);
        }
        else
        {
            // 4) Otherwise distribute according to the relative ratio of each size_ref value
            float split_ratio = child_0.size_ref[axis] / (child_0.size_ref[axis] + child_1.size_ref[axis]);
            child_0_size[axis] = ImMax(size_min_each, f32::floor(size_avail * split_ratio + 0.5));
            child_1_size[axis] = (size_avail - child_0_size[axis]);
        }

        child_1_pos[axis] += spacing + child_0_size[axis];
    }

    if (only_write_to_single_node == NULL)
        child_0.WantLockSizeOnce = child_1.WantLockSizeOnce = false;

    const bool child_0_recurse = only_write_to_single_node ? child_0_is_toward_single_node : child_0.IsVisible;
    const bool child_1_recurse = only_write_to_single_node ? child_1_is_toward_single_node : child_1.IsVisible;
    if (child_0_recurse)
        DockNodeTreeUpdatePosSize(child_0, child_0_pos, child_0_size);
    if (child_1_recurse)
        DockNodeTreeUpdatePosSize(child_1, child_1_pos, child_1_size);
}

// static void DockNodeTreeUpdateSplitterFindTouchingNode(ImGuiDockNode* node, ImGuiAxis axis, int side, ImVector<ImGuiDockNode*>* touching_nodes)
pub fn dock_node_tree_update_splitter_find_touching_node(g: &mut Context, node: &mut DockNode, axis: Axis, side: i32, touching_nodes: &mut Vec<Id32>)
{
    if (node.is_leaf_node())
    {
        touching_nodes.push_back(node);
        return;
    }
    if (node.child_nodes[0].IsVisible)
        if (node.split_axis != axis || side == 0 || !node.child_nodes[1].IsVisible)
            DockNodeTreeUpdateSplitterFindTouchingNode(node.child_nodes[0], axis, side, touching_nodes);
    if (node.child_nodes[1].IsVisible)
        if (node.split_axis != axis || side == 1 || !node.child_nodes[0].IsVisible)
            DockNodeTreeUpdateSplitterFindTouchingNode(node.child_nodes[1], axis, side, touching_nodes);
}

// (Depth-First, Pre-Order)
// void DockNodeTreeUpdateSplitter(ImGuiDockNode* node)
pub fn dock_node_tree_update_splitter(g: &mut Context, node: &mut DockNode)
{
    if (node.is_leaf_node())
        return;

    // ImGuiContext& g = *GImGui;

    ImGuiDockNode* child_0 = node.child_nodes[0];
    ImGuiDockNode* child_1 = node.child_nodes[1];
    if (child_0.IsVisible && child_1.IsVisible)
    {
        // Bounding box of the splitter cover the space between both nodes (w = Spacing, h = size[xy^1] for when splitting horizontally)
        const ImGuiAxis axis = node.split_axis;
        // IM_ASSERT(axis != ImGuiAxis_None);
        Rect bb;
        bb.min = child_0.pos;
        bb.max = child_1.pos;
        bb.min[axis] += child_0.size[axis];
        bb.max[axis ^ 1] += child_1.size[axis ^ 1];
        //if (g.io.key_ctrl) GetForegroundDrawList(g.current_window->viewport)->add_rect(bb.min, bb.max, IM_COL32(255,0,255,255));

        const ImGuiDockNodeFlags merged_flags = child_0.MergedFlags | child_1.MergedFlags; // Merged flags for BOTH childs
        const ImGuiDockNodeFlags no_resize_axis_flag = (axis == Axis::X) ? DockNodeFlags::NoResizeX : DockNodeFlags::NoResizeY;
        if ((merged_flags & DockNodeFlags::NoResize) || (merged_flags & no_resize_axis_flag))
        {
            ImGuiWindow* window = g.current_window;
            window.draw_list.add_rect_filled(bb.min, bb.max, get_color_u32(StyleColor::Separator), g.style.FrameRounding);
        }
        else
        {
            //bb.min[axis] += 1; // Display a little inward so highlight doesn't connect with nearby tabs on the neighbor node.
            //bb.max[axis] -= 1;
            PushID(node.id);

            // Find resizing limits by gathering list of nodes that are touching the splitter line.
            ImVector<ImGuiDockNode*> touching_nodes[2];
            float min_size = g.style.window_min_size[axis];
            float resize_limits[2];
            resize_limits[0] = node.child_nodes[0].pos[axis] + min_size;
            resize_limits[1] = node.child_nodes[1].pos[axis] + node.child_nodes[1].size[axis] - min_size;

            ImGuiID splitter_id = GetID("##splitter");
            if (g.active_id == splitter_id) // Only process when splitter is active
            {
                DockNodeTreeUpdateSplitterFindTouchingNode(child_0, axis, 1, &touching_nodes[0]);
                DockNodeTreeUpdateSplitterFindTouchingNode(child_1, axis, 0, &touching_nodes[1]);
                for (int touching_node_n = 0; touching_node_n < touching_nodes[0].size; touching_node_n += 1)
                    resize_limits[0] = ImMax(resize_limits[0], touching_nodes[0][touching_node_n].rect().min[axis] + min_size);
                for (int touching_node_n = 0; touching_node_n < touching_nodes[1].size; touching_node_n += 1)
                    resize_limits[1] = ImMin(resize_limits[1], touching_nodes[1][touching_node_n].rect().max[axis] - min_size);

                // [DEBUG] Render touching nodes & limits
                /*
                ImDrawList* draw_list = node->host_window ? GetForegroundDrawList(node->host_window) : GetForegroundDrawList(GetMainViewport());
                for (int n = 0; n < 2; n++)
                {
                    for (int touching_node_n = 0; touching_node_n < touching_nodes[n].size; touching_node_n++)
                        draw_list->add_rect(touching_nodes[n][touching_node_n]->pos, touching_nodes[n][touching_node_n]->pos + touching_nodes[n][touching_node_n]->size, IM_COL32(0, 255, 0, 255));
                    if (axis == ImGuiAxis_X)
                        draw_list->add_line(Vector2D(resize_limits[n], node->child_nodes[n]->pos.y), Vector2D(resize_limits[n], node->child_nodes[n]->pos.y + node->child_nodes[n]->size.y), IM_COL32(255, 0, 255, 255), 3.0);
                    else
                        draw_list->add_line(Vector2D(node->child_nodes[n]->pos.x, resize_limits[n]), Vector2D(node->child_nodes[n]->pos.x + node->child_nodes[n]->size.x, resize_limits[n]), IM_COL32(255, 0, 255, 255), 3.0);
                }
                */
            }

            // Use a short delay before highlighting the splitter (and changing the mouse cursor) in order for regular mouse movement to not highlight many splitters
            float cur_size_0 = child_0.size[axis];
            float cur_size_1 = child_1.size[axis];
            float min_size_0 = resize_limits[0] - child_0.pos[axis];
            float min_size_1 = child_1.pos[axis] + child_1.size[axis] - resize_limits[1];
            ImU32 bg_col = get_color_u32(StyleColor::WindowBg);
            if (SplitterBehavior(bb, GetID("##splitter"), axis, &cur_size_0, &cur_size_1, min_size_0, min_size_1, WINDOWS_HOVER_PADDING, WINDOWS_RESIZE_FROM_EDGES_FEEDBACK_TIMER, bg_col))
            {
                if (touching_nodes[0].size > 0 && touching_nodes[1].size > 0)
                {
                    child_0.size[axis] = child_0.size_ref[axis] = cur_size_0;
                    child_1.pos[axis] -= cur_size_1 - child_1.size[axis];
                    child_1.size[axis] = child_1.size_ref[axis] = cur_size_1;

                    // Lock the size of every node that is a sibling of the node we are touching
                    // This might be less desirable if we can merge sibling of a same axis into the same parental level.
                    for (int side_n = 0; side_n < 2; side_n += 1)
                        for (int touching_node_n = 0; touching_node_n < touching_nodes[side_n].size; touching_node_n += 1)
                        {
                            ImGuiDockNode* touching_node = touching_nodes[side_n][touching_node_n];
                            //ImDrawList* draw_list = node->host_window ? GetForegroundDrawList(node->host_window) : GetForegroundDrawList(GetMainViewport());
                            //draw_list->add_rect(touching_node->pos, touching_node->pos + touching_node->size, IM_COL32(255, 128, 0, 255));
                            while (touching_node.parent_node != node)
                            {
                                if (touching_node.parent_node.split_axis == axis)
                                {
                                    // Mark other node so its size will be preserved during the upcoming call to DockNodeTreeUpdatePosSize().
                                    ImGuiDockNode* node_to_preserve = touching_node.parent_node.child_nodes[side_n];
                                    node_to_preserve.WantLockSizeOnce = true;
                                    //draw_list->add_rect(touching_node->pos, touching_node->rect().max, IM_COL32(255, 0, 0, 255));
                                    //draw_list->add_rect_filled(node_to_preserve->pos, node_to_preserve->rect().max, IM_COL32(0, 255, 0, 100));
                                }
                                touching_node = touching_node.parent_node;
                            }
                        }

                    DockNodeTreeUpdatePosSize(child_0, child_0.pos, child_0.size);
                    DockNodeTreeUpdatePosSize(child_1, child_1.pos, child_1.size);
                    mark_ini_settings_dirty();
                }
            }
            PopID();
        }
    }

    if (child_0.IsVisible)
        DockNodeTreeUpdateSplitter(child_0);
    if (child_1.IsVisible)
        DockNodeTreeUpdateSplitter(child_1);
}

// [Internal] Called via SetNextWindowDockID()
// void SetWindowDock(ImGuiWindow* window, ImGuiID dock_id, ImGuiCond cond)
pub fn set_window_dock(g: &mut Context, window: &mut window::Window, dock_id: Id32, cond: Condition)
{
    // Test condition (NB: bit 0 is always true) and clear flags for next time
    if (cond && (window.set_window_dock_allow_flags & cond) == 0)
        return;
    window.set_window_dock_allow_flags &= ~(ImGuiCond_Once | Cond::FirstUseEver | ImGuiCond_Appearing);

    if (window.dock_id == dock_id)
        return;

    // If the user attempt to set a dock id that is a split node, we'll dig within to find a suitable docking spot
    ImGuiContext* .g = GImGui;
    if (ImGuiDockNode* new_node = dock_context_find_node_by_id(.g, dock_id))
        if (new_node.is_split_node())
        {
            // Policy: Find central node or latest focused node. We first move back to our root node.
            new_node = dock_node_get_root_node(new_node);
            if (new_node.CentralNode)
            {
                // IM_ASSERT(new_node.CentralNode.IsCentralNode());
                dock_id = new_node.CentralNode.id;
            }
            else
            {
                dock_id = new_node.LastFocusedNodeId;
            }
        }

    if (window.dock_id == dock_id)
        return;

    if (window.dock_node_id)
        dock_node_remove_window(window.dock_node_id, window, 0);
    window.dock_id = dock_id;
}

// Create an explicit dockspace node within an existing window. Also expose dock node flags and creates a central_node by default.
// The Central Node is always displayed even when empty and shrink/extend according to the requested size of its neighbors.
// DockSpace() needs to be submitted _before_ any window they can host. If you use a dockspace, submit it early in your app.
// ImGuiID DockSpace(ImGuiID id, const Vector2D& size_arg, ImGuiDockNodeFlags flags, const ImGuiWindowClass* window_class)
pub fn dock_space(g: &mut Context, id: Id32, size_arg: &Vector2D, flags: &mut HashSet<DockNodeFlags>, window_class: &WindowClass) -> Id32
{
    ImGuiContext* .g = GImGui;
    // ImGuiContext& g = *.g;
    ImGuiWindow* window = GetCurrentWindow();
    if (!(g.io.config_flags & ImGuiConfigFlags_DockingEnable))
        return 0;

    // Early out if parent window is hidden/collapsed
    // This is faster but also DockNodeUpdateTabBar() relies on TabBarLayout() running (which won't if skip_items=true) to set NextSelectedTabId = 0). See #2960.
    // If for whichever reason this is causing problem we would need to ensure that DockNodeUpdateTabBar() ends up clearing NextSelectedTabId even if skip_items=true.
    if (window.skip_items)
        flags |= DockNodeFlags::KeepAliveOnly;

    // IM_ASSERT((flags & ImGuiDockNodeFlags_DockSpace) == 0);
    // IM_ASSERT(id != 0);
    ImGuiDockNode* node = dock_context_find_node_by_id(.g, id);
    if (!node)
    {
        // IMGUI_DEBUG_LOG_DOCKING("[docking] DockSpace: dockspace node 0x%08X created\n", id);
        node = dock_context_add_node(.g, id);
        node.set_local_flags(DockNodeFlags::CentralNode);
    }
    if (window_class && window_class.ClassId != node.WindowClass.ClassId)
        // IMGUI_DEBUG_LOG_DOCKING("[docking] DockSpace: dockspace node 0x%08X: setup window_class 0x%08X -> 0x%08X\n", id, node.WindowClass.ClassId, window_class.ClassId);
    node.SharedFlags = flags;
    node.WindowClass = window_class ? *window_class : ImGuiWindowClass();

    // When a DockSpace transitioned form implicit to explicit this may be called a second time
    // It is possible that the node has already been claimed by a docked window which appeared before the DockSpace() node, so we overwrite is_dock_space again.
    if (node.last_frame_active == g.frame_count && !(flags & DockNodeFlags::KeepAliveOnly))
    {
        // IM_ASSERT(node.IsDockSpace() == false && "Cannot call DockSpace() twice a frame with the same id");
        node.set_local_flags(node.LocalFlags | DockNodeFlags::DockSpace);
        return id;
    }
    node.set_local_flags(node.LocalFlags | DockNodeFlags::DockSpace);

    // Keep alive mode, this is allow windows docked into this node so stay docked even if they are not visible
    if (flags & DockNodeFlags::KeepAliveOnly)
    {
        node.LastFrameAlive = g.frame_count;
        return id;
    }

    const Vector2D content_avail = GetContentRegionAvail();
    Vector2D size = f32::floor(size_arg);
    if (size.x <= 0.0)
        size.x = ImMax(content_avail.x + size.x, 4.0); // Arbitrary minimum child size (0.0 causing too much issues)
    if (size.y <= 0.0)
        size.y = ImMax(content_avail.y + size.y, 4.0);
    // IM_ASSERT(size.x > 0.0 && size.y > 0.0);

    node.pos = window.dc.cursor_pos;
    node.size = node.size_ref = size;
    SetNextWindowPos(node.pos);
    set_next_window_size(node.size);
    g.next_window_data.PosUndock = false;

    // FIXME-DOCK: Why do we need a child window to host a dockspace, could we host it in the existing window?
    // FIXME-DOCK: What is the reason for not simply calling BeginChild()? (OK to have a reason but should be commented)
    ImGuiWindowFlags window_flags = WindowFlags::ChildWindow | WindowFlags::DockNodeHost;
    window_flags |= WindowFlags::NoSavedSettings | WindowFlags::NoResize | WindowFlags::NoCollapse | WindowFlags::NoTitleBar;
    window_flags |= WindowFlags::NoScrollbar | WindowFlags::NoScrollWithMouse;
    window_flags |= WindowFlags::NoBackground;

    char title[256];
    ImFormatString(title, IM_ARRAYSIZE(title), "%s/DockSpace_%08X", window.Name, id);

    push_style_var(StyleVar::ChildBorderSize, 0.0);
    begin(title, NULL, window_flags);
    pop_style_var();

    ImGuiWindow* host_window = g.current_window;
    DockNodeSetupHostWindow(node, host_window);
    host_windowchild_id = window.get_id(title);
    node.OnlyNodeWithWindows = NULL;

    // IM_ASSERT(node.IsRootNode());

    // We need to handle the rare case were a central node is missing.
    // This can happen if the node was first created manually with DockBuilderAddNode() but _without_ the ImGuiDockNodeFlags_Dockspace.
    // Doing it correctly would set the _CentralNode flags, which would then propagate according to subsequent split.
    // It would also be ambiguous to attempt to assign a central node while there are split nodes, so we wait until there's a single node remaining.
    // The specific sub-property of _CentralNode we are interested in recovering here is the "Don't delete when empty" property,
    // as it doesn't make sense for an empty dockspace to not have this property.
    if (node.is_leaf_node() && !node.is_central_node())
        node.set_local_flags(node.LocalFlags | DockNodeFlags::CentralNode);

    // Update the node
    DockNodeUpdate(node);

    end();
    item_size(size);
    return id;
}

// Tips: Use with ImGuiDockNodeFlags_PassthruCentralNode!
// The limitation with this call is that your window won't have a menu bar.
// Even though we could pass window flags, it would also require the user to be able to call BeginMenuBar() somehow meaning we can't Begin/End in a single function.
// But you can also use BeginMainMenuBar(). If you really want a menu bar inside the same window as the one hosting the dockspace, you will need to copy this code somewhere and tweak it.
// ImGuiID DockSpaceOverViewport(const ImGuiViewport* viewport, ImGuiDockNodeFlags dockspace_flags, const ImGuiWindowClass* window_class)
pub fn dock_space_over_viewport(g: &mut Context, viewport: &mut Viewport, dockspace_flags: &HashSet<DockNodeFlags>, window_class: &WindowClass) -> Id32
{
    if (viewport == NULL)
        viewport = GetMainViewport();

    SetNextWindowPos(viewport.WorkPos);
    set_next_window_size(viewport.work_size);
    SetNextWindowViewport(viewport.id);

    ImGuiWindowFlags host_window_flags = 0;
    host_window_flags |= WindowFlags::NoTitleBar | WindowFlags::NoCollapse | WindowFlags::NoResize | WindowFlags::NoMove | WindowFlags::NoDocking;
    host_window_flags |= WindowFlags::NoBringToFrontOnFocus | WindowFlags::NoNavFocus;
    if (dockspace_flags & DockNodeFlags::PassthruCentralNode)
        host_window_flags |= WindowFlags::NoBackground;

    char label[32];
    ImFormatString(label, IM_ARRAYSIZE(label), "DockSpaceViewport_%08X", viewport.id);

    push_style_var(StyleVar::WindowRounding, 0.0);
    push_style_var(StyleVar::WindowBorderSize, 0.0);
    push_style_var(StyleVar::WindowPadding, Vector2D::new(0.0, 0.0));
    begin(label, NULL, host_window_flags);
    pop_style_var(3);

    ImGuiID dockspace_id = GetID("DockSpace");
    DockSpace(dockspace_id, Vector2D::new(0.0, 0.0), dockspace_flags, window_class);
    end();

    return dockspace_id;
}

// void DockBuilderDockWindow(const char* window_name, ImGuiID node_id)
pub fn dock_builder_dock_window(g: &mut Context, window_name: &str, node_id: Id32)
{
    // We don't preserve relative order of multiple docked windows (by clearing dock_order back to -1)
    ImGuiID window_id = ImHashStr(window_name);
    if (ImGuiWindow* window = FindWindowByID(window_id))
    {
        // Apply to created window
        SetWindowDock(window, node_id, Cond::Always);
        window.DockOrder = -1;
    }
    else
    {
        // Apply to settings
        ImGuiWindowSettings* settings = FindWindowSettings(window_id);
        if (settings == NULL)
            settings = CreateNewWindowSettings(window_name);
        settings.dock_id = node_id;
        settings.dock_order = -1;
    }
}

// ImGuiDockNode* DockBuilderGetNode(ImGuiID node_id)
pub fn dock_builder_get_node(g: &mut Context, node_id: Id32) -> &mut DockNode
{
    ImGuiContext* .g = GImGui;
    return dock_context_find_node_by_id(.g, node_id);
}

// void DockBuilderSetNodePos(ImGuiID node_id, Vector2D pos)
pub fn dock_builder_set_node_pos(g: &mut Context, node_id: Id32, pos: Vector2D)
{
    ImGuiContext* .g = GImGui;
    ImGuiDockNode* node = dock_context_find_node_by_id(.g, node_id);
    if (node == NULL)
        return;
    node.pos = pos;
    node.authority_for_pos = DataAuthority::DockNode;
}

// void DockBuilderSetNodeSize(ImGuiID node_id, Vector2D size)
pub fn dock_builder_set_node_size(g: &mut Context, node_id: Id32, size: Vector2D)
{
    ImGuiContext* .g = GImGui;
    ImGuiDockNode* node = dock_context_find_node_by_id(.g, node_id);
    if (node == NULL)
        return;
    // IM_ASSERT(size.x > 0.0 && size.y > 0.0);
    node.size = node.size_ref = size;
    node.authority_for_size = DataAuthority::DockNode;
}

// Make sure to use the ImGuiDockNodeFlags_DockSpace flag to create a dockspace node! Otherwise this will create a floating node!
// - Floating node: you can then call DockBuilderSetNodePos()/DockBuilderSetNodeSize() to position and size the floating node.
// - Dockspace node: calling DockBuilderSetNodePos() is unnecessary.
// - If you intend to split a node immediately after creation using DockBuilderSplitNode(), make sure to call DockBuilderSetNodeSize() beforehand!
//   For various reason, the splitting code currently needs a base size otherwise space may not be allocated as precisely as you would expect.
// - Use (id == 0) to let the system allocate a node identifier.
// - Existing node with a same id will be removed.
// ImGuiID DockBuilderAddNode(ImGuiID id, ImGuiDockNodeFlags flags)
pub fn dock_builder_add_node(g: &mut Context, id: Id32, flags: &HashSet<DockNodeFlags>) -> Id32
{
    ImGuiContext* .g = GImGui;

    if (id != 0)
        DockBuilderRemoveNode(id);

    ImGuiDockNode* node = NULL;
    if (flags & DockNodeFlags::DockSpace)
    {
        DockSpace(id, Vector2D::new(0, 0), (flags & ~DockNodeFlags::DockSpace) | DockNodeFlags::KeepAliveOnly);
        node = dock_context_find_node_by_id(.g, id);
    }
    else
    {
        node = dock_context_add_node(.g, id);
        node.set_local_flags(flags);
    }
    node.LastFrameAlive = .g.frame_count;   // Set this otherwise BeginDocked will undock during the same frame.
    return node.id;
}

// void DockBuilderRemoveNode(ImGuiID node_id)
pub fn dock_builder_remove_node(g: &mut Context, node_id: Id32)
{
    ImGuiContext* .g = GImGui;
    ImGuiDockNode* node = dock_context_find_node_by_id(.g, node_id);
    if (node == NULL)
        return;
    DockBuilderRemoveNodeDockedWindows(node_id, true);
    DockBuilderRemoveNodeChildNodes(node_id);
    // Node may have moved or deleted if e.g. any merge happened
    node = dock_context_find_node_by_id(.g, node_id);
    if (node == NULL)
        return;
    if (node.is_central_node() && node.parent_node)
        node.parent_node.set_local_flags(node.parent_node.LocalFlags | DockNodeFlags::CentralNode);
    dock_context_remove_node(.g, node, true);
}

// root_id = 0 to remove all, root_id != 0 to remove child of given node.
// void DockBuilderRemoveNodeChildNodes(ImGuiID root_id)
pub fn dock_builder_remove_node_child_nodes(g: &mut Context, root_id: Id32)
{
    ImGuiContext* .g = GImGui;
    ImGuiDockContext* dc  = &.g.dock_context;

    ImGuiDockNode* root_node = root_id ? dock_context_find_node_by_id(.g, root_id) : NULL;
    if (root_id && root_node == NULL)
        return;
    bool has_central_node = false;

    ImGuiDataAuthority backup_root_node_authority_for_pos = root_node ? root_node.authority_for_pos : DataAuthority::Auto;
    ImGuiDataAuthority backup_root_node_authority_for_size = root_node ? root_node.authority_for_size : DataAuthority::Auto;

    // Process active windows
    ImVector<ImGuiDockNode*> nodes_to_remove;
    for (int n = 0; n < dc.Nodes.data.size; n += 1)
        if (ImGuiDockNode* node = (ImGuiDockNode*)dc.Nodes.data[n].val_p)
        {
            bool want_removal = (root_id == 0) || (node.id != root_id && dock_node_get_root_node(node).id == root_id);
            if (want_removal)
            {
                if (node.is_central_node())
                    has_central_node = true;
                if (root_id != 0)
                    DockContextQueueNotifyRemovedNode(.g, node);
                if (root_node)
                {
                    dock_node_move_windows(root_node, node);
                    dock_settings_rename_node_references(node.id, root_node.id);
                }
                nodes_to_remove.push_back(node);
            }
        }

    // DockNodeMoveWindows->DockNodeAddWindow will normally set those when reaching two windows (which is only adequate during interactive merge)
    // Make sure we don't lose our current pos/size. (FIXME-DOCK: Consider tidying up that code in DockNodeAddWindow instead)
    if (root_node)
    {
        root_node.authority_for_pos = backup_root_node_authority_for_pos;
        root_node.authority_for_size = backup_root_node_authority_for_size;
    }

    // Apply to settings
    for (ImGuiWindowSettings* settings = .g.SettingsWindows.begin(); settings != NULL; settings = .g.SettingsWindows.next_chunk(settings))
        if (ImGuiID window_settings_dock_id = settings.dock_id)
            for (int n = 0; n < nodes_to_remove.size; n += 1)
                if (nodes_to_remove[n].id == window_settings_dock_id)
                {
                    settings.dock_id = root_id;
                    break;
                }

    // Not really efficient, but easier to destroy a whole hierarchy considering dock_context_remove_node is attempting to merge nodes
    if (nodes_to_remove.size > 1)
        ImQsort(nodes_to_remove.data, nodes_to_remove.size, sizeof(ImGuiDockNode*), DockNodeComparerDepthMostFirst);
    for (int n = 0; n < nodes_to_remove.size; n += 1)
        dock_context_remove_node(.g, nodes_to_remove[n], false);

    if (root_id == 0)
    {
        dc.Nodes.Clear();
        dc.requests.clear();
    }
    else if (has_central_node)
    {
        root_node.CentralNode = root_node;
        root_node.set_local_flags(root_node.LocalFlags | DockNodeFlags::CentralNode);
    }
}

// void DockBuilderRemoveNodeDockedWindows(ImGuiID root_id, bool clear_settings_refs)
pub fn dock_builder_remove_node_docked_windows(g: &mut Context, root_id: Id32, clear_settings_refs: bool)
{
    // clear references in settings
    ImGuiContext* .g = GImGui;
    // ImGuiContext& g = *.g;
    if (clear_settings_refs)
    {
        for (ImGuiWindowSettings* settings = g.settings_windows.begin(); settings != NULL; settings = g.settings_windows.next_chunk(settings))
        {
            bool want_removal = (root_id == 0) || (settings.dock_id == root_id);
            if (!want_removal && settings.dock_id != 0)
                if (ImGuiDockNode* node = dock_context_find_node_by_id(.g, settings.dock_id))
                    if (dock_node_get_root_node(node).id == root_id)
                        want_removal = true;
            if (want_removal)
                settings.dock_id = INVALID_ID;
        }
    }

    // clear references in windows
    for (int n = 0; n < g.windows.size; n += 1)
    {
        ImGuiWindow* window = g.windows[n];
        bool want_removal = (root_id == 0) || (window.dock_node && dock_node_get_root_node(window.dock_node).id == root_id) || (window.dock_node_as_host && window.dock_node_as_host.id == root_id);
        if (want_removal)
        {
            const ImGuiID backup_dock_id = window.dock_id;
            IM_UNUSED(backup_dock_id);
            DockContextProcessUndockWindow(.g, window, clear_settings_refs);
            if (!clear_settings_refs)
                // IM_ASSERT(window.DockId == backup_dock_id);
        }
    }
}

// If 'out_id_at_dir' or 'out_id_at_opposite_dir' are non NULL, the function will write out the id of the two new nodes created.
// Return value is id of the node at the specified direction, so same as (*out_id_at_dir) if that pointer is set.
// FIXME-DOCK: We are not exposing nor using split_outer.
// ImGuiID DockBuilderSplitNode(ImGuiID id, ImGuiDir split_dir, float size_ratio_for_node_at_dir, ImGuiID* out_id_at_dir, ImGuiID* out_id_at_opposite_dir)
pub fn dock_builder_split_node(g: &mut Context, id: Id32, split_dir: Direction, size_ratio_for_node_at_dir: f32, out_id_at_dir: &mut Id32, out_id_at_opposite_dir: &mut Id32) -> Id32
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(split_dir != Dir::None);
    // IMGUI_DEBUG_LOG_DOCKING("[docking] DockBuilderSplitNode: node 0x%08X, split_dir %d\n", id, split_dir);

    ImGuiDockNode* node = dock_context_find_node_by_id(&g, id);
    if (node == NULL)
    {
        // IM_ASSERT(node != NULL);
        return 0;
    }

    // IM_ASSERT(!node.IsSplitNode()); // Assert if already split

    ImGuiDockRequest req;
    req.Type = DockRequestType::Split;
    req.dock_target_window = NULL;
    req.dock_target_node = node;
    req.dock_payload = NULL;
    req.dock_split_dir = split_dir;
    req.dock_split_ratio = ImSaturate((split_dir == Direction::Left || split_dir == Direction::Up) ? size_ratio_for_node_at_dir : 1.0 - size_ratio_for_node_at_dir);
    req.dock_split_outer = false;
    context::dock_context_process_dock(&g, &req);

    ImGuiID id_at_dir = node.child_nodes[(split_dir == Direction::Left || split_dir == Direction::Up) ? 0 : 1].id;
    ImGuiID id_at_opposite_dir = node.child_nodes[(split_dir == Direction::Left || split_dir == Direction::Up) ? 1 : 0].id;
    if (out_id_at_dir)
        *out_id_at_dir = id_at_dir;
    if (out_id_at_opposite_dir)
        *out_id_at_opposite_dir = id_at_opposite_dir;
    return id_at_dir;
}

// static ImGuiDockNode* DockBuilderCopyNodeRec(ImGuiDockNode* src_node, ImGuiID dst_node_id_if_known, ImVector<ImGuiID>* out_node_remap_pairs)
pub fn dock_builder_copy_node_rec(g: &mut Context, src_node: &mut DockNode, dst_node_id_if_known: Id32, out_node_remap_pairs: &mut Vec<Id32>) -> &mut DockNode
{
    // ImGuiContext& g = *GImGui;
    ImGuiDockNode* dst_node = dock_context_add_node(&g, dst_node_id_if_known);
    dst_node.SharedFlags = src_node.SharedFlags;
    dst_node.LocalFlags = src_node.LocalFlags;
    dst_node.LocalFlagsInWindows = DockNodeFlags::None;
    dst_node.pos = src_node.pos;
    dst_node.size = src_node.size;
    dst_node.size_ref = src_node.size_ref;
    dst_node.split_axis = src_node.split_axis;
    dst_node.UpdateMergedFlags();

    out_node_remap_pairs.push_back(src_node.id);
    out_node_remap_pairs.push_back(dst_node.id);

    for (int child_n = 0; child_n < IM_ARRAYSIZE(src_node.child_nodes); child_n += 1)
        if (src_node.child_nodes[child_n])
        {
            dst_node.child_nodes[child_n] = DockBuilderCopyNodeRec(src_node.child_nodes[child_n], 0, out_node_remap_pairs);
            dst_node.child_nodes[child_n]parent_node = dst_node;
        }

    // IMGUI_DEBUG_LOG_DOCKING("[docking] Fork node %08X -> %08X (%d childs)\n", src_node.ID, dst_node.ID, dst_node.IsSplitNode() ? 2 : 0);
    return dst_node;
}

// void DockBuilderCopyNode(ImGuiID src_node_id, ImGuiID dst_node_id, ImVector<ImGuiID>* out_node_remap_pairs)
pub fn dock_builder_copy_node(g: &mut Context, src_node_id: Id32, dst_node_id: Id32, out_node_remap_pairs: &mut Vec<Id32>)
{
    ImGuiContext* .g = GImGui;
    // IM_ASSERT(src_node_id != 0);
    // IM_ASSERT(dst_node_id != 0);
    // IM_ASSERT(out_node_remap_pairs != NULL);

    DockBuilderRemoveNode(dst_node_id);

    ImGuiDockNode* src_node = dock_context_find_node_by_id(.g, src_node_id);
    // IM_ASSERT(src_node != NULL);

    out_node_remap_pairs.clear();
    DockBuilderCopyNodeRec(src_node, dst_node_id, out_node_remap_pairs);

    // IM_ASSERT((out_node_remap_pairs.size % 2) == 0);
}

// void DockBuilderCopyWindowSettings(const char* src_name, const char* dst_name)
pub fn dock_builder_copy_window_settings(g: &mut Context, src_name: &str, dst_name: &str)
{
    ImGuiWindow* src_window = find_window_by_name(src_name);
    if (src_window == NULL)
        return;
    if (ImGuiWindow* dst_window = find_window_by_name(dst_name))
    {
        dst_window.pos = src_window.pos;
        dst_window.size = src_window.size;
        dst_window.sizeFull = src_window.sizeFull;
        dst_window.collapsed = src_window.collapsed;
    }
    else if (ImGuiWindowSettings* dst_settings = FindOrCreateWindowSettings(dst_name))
    {
        Vector2Dih window_pos_2ih = Vector2Dih(src_window.pos);
        if (src_window.viewport_id != 0 && src_window.viewport_id != IMGUI_VIEWPORT_DEFAULT_ID)
        {
            dst_settings.viewport_pos = window_pos_2ih;
            dst_settings.viewport_id = src_window.viewport_id;
            dst_settings.pos = Vector2Dih(0, 0);
        }
        else
        {
            dst_settings.pos = window_pos_2ih;
        }
        dst_settings.size = Vector2Dih(src_window.sizeFull);
        dst_settings.collapsed = src_window.collapsed;
    }
}

// FIXME: Will probably want to change this signature, in particular how the window remapping pairs are passed.
// void DockBuilderCopyDockSpace(ImGuiID src_dockspace_id, ImGuiID dst_dockspace_id, ImVector<const char*>* in_window_remap_pairs)
pub fn dock_builder_copy_dock_space(g: &mut Context, src_dockspace_id: Id32, dst_dockspace_id: Id32, in_window_remap_pairs: &mut Vec<String>)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(src_dockspace_id != 0);
    // IM_ASSERT(dst_dockspace_id != 0);
    // IM_ASSERT(in_window_remap_pairs != NULL);
    // IM_ASSERT((in_window_remap_pairs.size % 2) == 0);

    // Duplicate entire dock
    // FIXME: When overwriting dst_dockspace_id, windows that aren't part of our dockspace window class but that are docked in a same node will be split apart,
    // whereas we could attempt to at least keep them together in a new, same floating node.
    ImVector<ImGuiID> node_remap_pairs;
    DockBuilderCopyNode(src_dockspace_id, dst_dockspace_id, &node_remap_pairs);

    // Attempt to transition all the upcoming windows associated to dst_dockspace_id into the newly created hierarchy of dock nodes
    // (The windows associated to src_dockspace_id are staying in place)
    ImVector<ImGuiID> src_windows;
    for (int remap_window_n = 0; remap_window_n < in_window_remap_pairs.size; remap_window_n += 2)
    {
        const char* src_window_name = (*in_window_remap_pairs)[remap_window_n];
        const char* dst_window_name = (*in_window_remap_pairs)[remap_window_n + 1];
        ImGuiID src_window_id = ImHashStr(src_window_name);
        src_windows.push_back(src_window_id);

        // Search in the remapping tables
        ImGuiID src_dock_id = INVALID_ID;
        if (ImGuiWindow* src_window = FindWindowByID(src_window_id))
            src_dock_id = src_window.dock_id;
        else if (ImGuiWindowSettings* src_window_settings = FindWindowSettings(src_window_id))
            src_dock_id = src_window_settings.dock_id;
        ImGuiID dst_dock_id = INVALID_ID;
        for (int dock_remap_n = 0; dock_remap_n < node_remap_pairs.size; dock_remap_n += 2)
            if (node_remap_pairs[dock_remap_n] == src_dock_id)
            {
                dst_dock_id = node_remap_pairs[dock_remap_n + 1];
                //node_remap_pairs[dock_remap_n] = node_remap_pairs[dock_remap_n + 1] = 0; // clear
                break;
            }

        if (dst_dock_id != 0)
        {
            // Docked windows gets redocked into the new node hierarchy.
            // IMGUI_DEBUG_LOG_DOCKING("[docking] Remap live window '%s' 0x%08X -> '%s' 0x%08X\n", src_window_name, src_dock_id, dst_window_name, dst_dock_id);
            DockBuilderDockWindow(dst_window_name, dst_dock_id);
        }
        else
        {
            // Floating windows gets their settings transferred (regardless of whether the new window already exist or not)
            // When this is leading to a Copy and not a Move, we would get two overlapping floating windows. Could we possibly dock them together?
            // IMGUI_DEBUG_LOG_DOCKING("[docking] Remap window settings '%s' -> '%s'\n", src_window_name, dst_window_name);
            DockBuilderCopyWindowSettings(src_window_name, dst_window_name);
        }
    }

    // Anything else in the source nodes of 'node_remap_pairs' are windows that were docked in src_dockspace_id but are not owned by it (unaffiliated windows, e.g. "ImGui Demo")
    // Find those windows and move to them to the cloned dock node. This may be optional?
    for (int dock_remap_n = 0; dock_remap_n < node_remap_pairs.size; dock_remap_n += 2)
        if (ImGuiID src_dock_id = node_remap_pairs[dock_remap_n])
        {
            ImGuiID dst_dock_id = node_remap_pairs[dock_remap_n + 1];
            ImGuiDockNode* node = DockBuilderGetNode(src_dock_id);
            for (int window_n = 0; window_n < node.windows.size; window_n += 1)
            {
                ImGuiWindow* window = node.windows[window_n];
                if (src_windows.contains(window.id))
                    continue;

                // Docked windows gets redocked into the new node hierarchy.
                // IMGUI_DEBUG_LOG_DOCKING("[docking] Remap window '%s' %08X -> %08X\n", window.Name, src_dock_id, dst_dock_id);
                DockBuilderDockWindow(window.Name, dst_dock_id);
            }
        }
}

// FIXME-DOCK: This is awkward because in series of split user is likely to loose access to its root node.
// void DockBuilderFinish(ImGuiID root_id)
pub fn dock_builder_finish(g: &mut Context, root_id: Id32)
{
    ImGuiContext* .g = GImGui;
    //DockContextRebuild(ctx);
    DockContextBuildAddWindowsToNodes(.g, root_id);
}

// bool GetWindowAlwaysWantOwnTabBar(ImGuiWindow* window)
pub fn get_window_always_want_own_tab_bar(g: &mut Context, window: &mut window::Window) -> bool
{
    // ImGuiContext& g = *GImGui;
    if (g.io.ConfigDockingAlwaysTabBar || window.WindowClass.DockingAlwaysTabBar)
        if ((window.flags & (WindowFlags::ChildWindow | WindowFlags::NoTitleBar | WindowFlags::NoDocking)) == 0)
            if (!window.IsFallbackWindow)    // We don't support AlwaysTabBar on the fallback/implicit window to avoid unused dock-node overhead/noise
                return true;
    return false;
}

// static ImGuiDockNode* DockContextBindNodeToWindow(ImGuiContext* ctx, ImGuiWindow* window)
pub fn dock_context_bind_node_to_window(g: &mut Context, window: &mut window::Window) -> &mut DockNode
{
    // ImGuiContext& g = *.g;
    ImGuiDockNode* node = dock_context_find_node_by_id(.g, window.dock_id);
    // IM_ASSERT(window.dock_node == NULL);

    // We should not be docking into a split node (SetWindowDock should avoid this)
    if (node && node.is_split_node())
    {
        DockContextProcessUndockWindow(.g, window);
        return NULL;
    }

    // Create node
    if (node == NULL)
    {
        node = dock_context_add_node(.g, window.dock_id);
        node.authority_for_pos = node.authority_for_size = node.authority_for_viewport = DataAuthority::Window;
        node.LastFrameAlive = g.frame_count;
    }

    // If the node just turned visible and is part of a hierarchy, it doesn't have a size assigned by DockNodeTreeUpdatePosSize() yet,
    // so we're forcing a pos/size update from the first ancestor that is already visible (often it will be the root node).
    // If we don't do this, the window will be assigned a zero-size on its first frame, which won't ideally warm up the layout.
    // This is a little wonky because we don't normally update the pos/size of visible node mid-frame.
    if (!node.IsVisible)
    {
        ImGuiDockNode* ancestor_node = node;
        while (!ancestor_node.IsVisible && ancestor_node.parent_node)
            ancestor_node = ancestor_node.parent_node;
        // IM_ASSERT(ancestor_node.size.x > 0.0 && ancestor_node.size.y > 0.0);
        DockNodeUpdateHasCentralNodeChild(dock_node_get_root_node(ancestor_node));
        DockNodeTreeUpdatePosSize(ancestor_node, ancestor_node.pos, ancestor_node.size, node);
    }

    // Add window to node
    bool node_was_visible = node.IsVisible;
    dock_node_add_window(node, window, true);
    node.IsVisible = node_was_visible; // Don't mark visible right away (so DockContextEndFrame() doesn't render it, maybe other side effects? will see)
    // IM_ASSERT(node == window.dock_node);
    return node;
}

// void BeginDocked(ImGuiWindow* window, bool* p_open)
pub fn begin_docked(g: &mut Context, window: &mut window::Window, p_open: &mut bool)
{
    ImGuiContext* .g = GImGui;
    // ImGuiContext& g = *.g;

    // clear fields ahead so most early-out paths don't have to do it
    window.dock_is_active = window.dock_node_is_visible = window.dock_tab_is_visible = false;

    const bool auto_dock_node = GetWindowAlwaysWantOwnTabBar(window);
    if (auto_dock_node)
    {
        if (window.dock_id == 0)
        {
            // IM_ASSERT(window.dock_node == NULL);
            window.dock_id = dock_context_gen_node_id(.g);
        }
    }
    else
    {
        // Calling SetNextWindowPos() undock windows by default (by setting PosUndock)
        bool want_undock = false;
        want_undock |= (window.flags & WindowFlags::NoDocking) != 0;
        want_undock |= (g.next_window_data.flags & NextWindowDataFlags::HasPos) && (window.set_window_pos_allow_flags & g.next_window_data.PosCond) && g.next_window_data.PosUndock;
        if (want_undock)
        {
            DockContextProcessUndockWindow(.g, window);
            return;
        }
    }

    // Bind to our dock node
    ImGuiDockNode* node = window.dock_node_id;
    if (node != NULL)
        // IM_ASSERT(window.DockId == node.ID);
    if (window.dock_id != 0 && node == NULL)
    {
        node = DockContextBindNodeToWindow(.g, window);
        if (node == NULL)
            return;
    }

// #if0
    // Undock if the ImGuiDockNodeFlags_NoDockingInCentralNode got set
    if (node.is_central_node && (node.flags & DockNodeFlags::NoDockingInCentralNode))
    {
        DockContextProcessUndockWindow(.g, window);
        return;
    }


    // Undock if our dockspace node disappeared
    // Note how we are testing for last_frame_alive and NOT last_frame_active. A DockSpace node can be maintained alive while being inactive with ImGuiDockNodeFlags_KeepAliveOnly.
    if (node.LastFrameAlive < g.frame_count)
    {
        // If the window has been orphaned, transition the docknode to an implicit node processed in DockContextNewFrameUpdateDocking()
        ImGuiDockNode* root_node = dock_node_get_root_node(node);
        if (root_node.LastFrameAlive < g.frame_count)
            DockContextProcessUndockWindow(.g, window);
        else
            window.dock_is_active = true;
        return;
    }

    // Store style overrides
    for (int color_n = 0; color_n < ImGuiWindowDockStyleCol_COUNT; color_n += 1)
        window.DockStyle.colors[color_n] = ColorConvertFloat4ToU32(g.style.colors[GWindowDockStyleColors[color_n]]);

    // Fast path return. It is common for windows to hold on a persistent dock_id but be the only visible window,
    // and never create neither a host window neither a tab bar.
    // FIXME-DOCK: replace ->host_window NULL compare with something more explicit (~was initially intended as a first frame test)
    if (node.host_window == NULL)
    {
        if (node.State == ImGuiDockNodeState_HostWindowHiddenBecauseWindowsAreResizing)
            window.dock_is_active = true;
        if (node.windows.size > 1)
            DockNodeHideWindowDuringHostWindowCreation(window);
        return;
    }

    // We can have zero-sized nodes (e.g. children of a small-size dockspace)
    // IM_ASSERT(node.host_window);
    // IM_ASSERT(node.IsLeafNode());
    // IM_ASSERT(node.size.x >= 0.0 && node.size.y >= 0.0);
    node.State = ImGuiDockNodeState_HostWindowVisible;

    // Undock if we are submitted earlier than the host window
    if (!(node.MergedFlags & DockNodeFlags::KeepAliveOnly) && window.BeginOrderWithinContext < node.host_window.BeginOrderWithinContext)
    {
        DockContextProcessUndockWindow(.g, window);
        return;
    }

    // Position/size window
    SetNextWindowPos(node.pos);
    set_next_window_size(node.size);
    g.next_window_data.PosUndock = false; // Cancel implicit undocking of SetNextWindowPos()
    window.dock_is_active = true;
    window.dock_node_is_visible = true;
    window.dock_tab_is_visible = false;
    if (node.MergedFlags & DockNodeFlags::KeepAliveOnly)
        return;

    // When the window is selected we mark it as visible.
    if (node.visible_window == window)
        window.dock_tab_is_visible = true;

    // Update window flag
    // IM_ASSERT((window.flags & WindowFlags::ChildWindow) == 0);
    window.flags |= WindowFlags::ChildWindow | WindowFlags::AlwaysUseWindowPadding | WindowFlags::NoResize;
    if (node.is_hidden_tab_bar() || node.is_no_tab_bar())
        window.flags |= WindowFlags::NoTitleBar;
    else
        window.flags &= ~WindowFlags::NoTitleBar;      // clear the NoTitleBar flag in case the user set it: confusingly enough we need a title bar height so we are correctly offset, but it won't be displayed!

    // Save new dock order only if the window has been visible once already
    // This allows multiple windows to be created in the same frame and have their respective dock orders preserved.
    if (node.tab_bar && window.was_active)
        window.DockOrder = DockNodeGetTabOrder(window);

    if ((node.WantCloseAll || node.WantCloseTabId == window.tab_id) && p_open != NULL)
        *p_open = false;

    // Update child_id to allow returning from Child to Parent with Escape
    ImGuiWindow* parent_window = window.dock_node_id.host_window_id;
    windowchild_id = parent_window.get_id(window.Name);
}

// void BeginDockableDragDropSource(ImGuiWindow* window)
pub fn begin_dockable_drag_drop_source(g: &mut Context, window: &mut window::Window)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.active_id == window.move_id);
    // IM_ASSERT(g.moving_window == window);
    // IM_ASSERT(g.current_window == window);

    g.last_item_data.id = window.move_id;
    window = window.root_window_dock_tree;
    // IM_ASSERT((window.flags & WindowFlags::NoDocking) == 0);
    bool is_drag_docking = (g.io.ConfigDockingWithShift) || Rect(0, 0, window.size_full.x, get_frame_height()).Contains(g.ActiveIdClickOffset); // FIXME-DOCKING: Need to make this stateful and explicit
    if (is_drag_docking && BeginDragDropSource(DragDropFlags::SourceNoPreviewTooltip | ImGuiDragDropFlags_SourceNoHoldToOpenOthers | ImGuiDragDropFlags_SourceAutoExpirePayload))
    {
        SetDragDropPayload(IMGUI_PAYLOAD_TYPE_WINDOW, &window, sizeof(window));
        EndDragDropSource();

        // Store style overrides
        for (int color_n = 0; color_n < ImGuiWindowDockStyleCol_COUNT; color_n += 1)
            window.DockStyle.colors[color_n] = ColorConvertFloat4ToU32(g.style.colors[GWindowDockStyleColors[color_n]]);
    }
}

// void BeginDockableDragDropTarget(ImGuiWindow* window)
pub fn begin_dockable_drag_drop_target(g: &mut Context, window: &mut window::Window)
{
    ImGuiContext* .g = GImGui;
    // ImGuiContext& g = *.g;

    //IM_ASSERT(window->root_window_dock_tree == window); // May also be a DockSpace
    // IM_ASSERT((window.flags & WindowFlags::NoDocking) == 0);
    if (!g.drag_drop_active)
        return;
    //GetForegroundDrawList(window)->add_rect(window->pos, window->pos + window->size, IM_COL32(255, 255, 0, 255));
    if (!BeginDragDropTargetCustom(window.Rect(), window.id))
        return;

    // Peek into the payload before calling AcceptDragDropPayload() so we can handle overlapping dock nodes with filtering
    // (this is a little unusual pattern, normally most code would call AcceptDragDropPayload directly)
    const ImGuiPayload* payload = &g.drag_drop_payload;
    if (!payload.IsDataType(IMGUI_PAYLOAD_TYPE_WINDOW) || !DockNodeIsDropAllowed(window, *(ImGuiWindow**)payload.Data))
    {
        EndDragDropTarget();
        return;
    }

    ImGuiWindow* payload_window = *(ImGuiWindow**)payload.Data;
    if (AcceptDragDropPayload(IMGUI_PAYLOAD_TYPE_WINDOW, ImGuiDragDropFlags_AcceptBeforeDelivery | ImGuiDragDropFlags_AcceptNoDrawDefaultRect))
    {
        // Select target node
        // (Important: we cannot use g.hovered_dock_node here! Because each of our target node have filters based on payload, each candidate drop target will do its own evaluation)
        bool dock_into_floating_window = false;
        ImGuiDockNode* node = NULL;
        if (window.dock_node_as_host_id)
        {
            // Cannot assume that node will != NULL even though we passed the rectangle test: it depends on padding/spacing handled by DockNodeTreeFindVisibleNodeByPos().
            node = dock_node_tree_find_visible_node_by_pos(window.dock_node_as_host_id, g.io.mouse_pos);

            // There is an edge case when docking into a dockspace which only has _inactive_ nodes (because none of the windows are active)
            // In this case we need to fallback into any leaf mode, possibly the central node.
            // FIXME-20181220: We should not have to test for is_leaf_node() here but we have another bug to fix first.
            if (node && node.IsDockSpace() && node.is_root_node())
                node = (node.CentralNode && node.is_leaf_node()) ? node.CentralNode : DockNodeTreeFindFallbackLeafNode(node);
        }
        else
        {
            if (window.dock_node_id)
                node = window.dock_node_id;
            else
                dock_into_floating_window = true; // Dock into a regular window
        }

        const Rect explicit_target_rect = (node && node.tab_bar && !node.is_hidden_tab_bar() && !node.is_no_tab_bar()) ? node.tab_bar.BarRect : Rect(window.pos, window.pos + Vector2D::new(window.size.x, get_frame_height()));
        const bool is_explicit_target = g.io.ConfigDockingWithShift || IsMouseHoveringRect(explicit_target_rect.min, explicit_target_rect.max);

        // preview docking request and find out split direction/ratio
        //const bool do_preview = true;     // Ignore testing for payload->is_preview() which removes one frame of delay, but breaks overlapping drop targets within the same window.
        const bool do_preview = payload.IsPreview() || payload.IsDelivery();
        if (do_preview && (node != NULL || dock_into_floating_window))
        {
            ImGuiDockPreviewData split_inner;
            ImGuiDockPreviewData split_outer;
            ImGuiDockPreviewData* split_data = &split_inner;
            if (node && (node.parent_node || node.is_central_node()))
                if (ImGuiDockNode* root_node = dock_node_get_root_node(node))
                {
                    dock_node_preview_dock_setup(window, root_node, payload_window, &split_outer, is_explicit_target, true);
                    if (split_outer.IsSplitDirExplicit)
                        split_data = &split_outer;
                }
            dock_node_preview_dock_setup(window, node, payload_window, &split_inner, is_explicit_target, false);
            if (split_data == &split_outer)
                split_inner.IsDropAllowed = false;

            // Draw inner then outer, so that previewed tab (in inner data) will be behind the outer drop boxes
            DockNodePreviewDockRender(window, node, payload_window, &split_inner);
            DockNodePreviewDockRender(window, node, payload_window, &split_outer);

            // Queue docking request
            if (split_data.IsDropAllowed && payload.IsDelivery())
                DockContextQueueDock(.g, window, split_data.SplitNode, payload_window, split_data.SplitDir, split_data.SplitRatio, split_data == &split_outer);
        }
    }
    EndDragDropTarget();
}

// static void DockSettingsRenameNodeReferences(ImGuiID old_node_id, ImGuiID new_node_id)
pub fn dock_settings_rename_node_references(g: &mut Context, old_node_id: Id32, new_node_id: Id32)
{
    // ImGuiContext& g = *GImGui;
    // IMGUI_DEBUG_LOG_DOCKING("[docking] DockSettingsRenameNodeReferences: from 0x%08X -> to 0x%08X\n", old_node_id, new_node_id);
    for (int window_n = 0; window_n < g.windows.size; window_n += 1)
    {
        ImGuiWindow* window = g.windows[window_n];
        if (window.dock_id == old_node_id && window.dock_node == NULL)
            window.dock_id = new_node_id;
    }
    //// FIXME-OPT: We could remove this loop by storing the index in the map
    for (ImGuiWindowSettings* settings = g.settings_windows.begin(); settings != NULL; settings = g.settings_windows.next_chunk(settings))
        if (settings.dock_id == old_node_id)
            settings.dock_id = new_node_id;
}

// Remove references stored in ImGuiWindowSettings to the given ImGuiDockNodeSettings
// static void DockSettingsRemoveNodeReferences(ImGuiID* node_ids, int node_ids_count)
pub fn dock_settings_remove_node_references(g: &mut Context, node_ids: &mut Id32, node_ids_count: i32)
{
    // ImGuiContext& g = *GImGui;
    int found = 0;
    //// FIXME-OPT: We could remove this loop by storing the index in the map
    for (ImGuiWindowSettings* settings = g.settings_windows.begin(); settings != NULL; settings = g.settings_windows.next_chunk(settings))
        for (int node_n = 0; node_n < node_ids_count; node_n += 1)
            if (settings.dock_id == node_ids[node_n])
            {
                settings.dock_id = INVALID_ID;
                settings.dock_order = -1;
                if (found += 1 < node_ids_count)
                    break;
                return;
            }
}

// static ImGuiDockNodeSettings* DockSettingsFindNodeSettings(ImGuiContext* ctx, ImGuiID id)
pub fn dock_settings_find_node_settings(g: &mut Context, id: Id32) -> &mut DockNodeSettings
{
    // FIXME-OPT
    ImGuiDockContext* dc  = &.g.dock_context;
    for (int n = 0; n < dc.NodesSettings.size; n += 1)
        if (dc.NodesSettings[n].id == id)
            return &dc.NodesSettings[n];
    return NULL;
}

// clear settings data
// static void DockSettingsHandler_ClearAll(ImGuiContext* ctx, ImGuiSettingsHandler*)
pub fn dock_settings_handler_clear_all(g: &mut Context, handler: &mut SettingsHandler)
{
    ImGuiDockContext* dc  = &.g.dock_context;
    dc.NodesSettings.clear();
    DockContextClearNodes(.g, 0, true);
}

// Recreate nodes based on settings data
// static void DockSettingsHandler_ApplyAll(ImGuiContext* ctx, ImGuiSettingsHandler*)
pub fn dock_settings_handler_apply_all(g: &mut Context, handler: &mut SettingsHandler)
{
    // Prune settings at boot time only
    ImGuiDockContext* dc  = &.g.dock_context;
    if (.g.windows.size == 0)
        DockContextPruneUnusedSettingsNodes(.g);
    DockContextBuildNodesFromSettings(.g, dc.NodesSettings.data, dc.NodesSettings.size);
    DockContextBuildAddWindowsToNodes(.g, 0);
}

// static void* DockSettingsHandler_ReadOpen(ImGuiContext*, ImGuiSettingsHandler*, const char* name)
pub fn dock_settings_handler_read_open(g: &mut Context, handler: &mut SettingsHandler, name: &str) -> bool
{
    if (strcmp(name, "data") != 0)
        return NULL;
    return (void*)1;
}

// static void DockSettingsHandler_ReadLine(ImGuiContext* ctx, ImGuiSettingsHandler*, void*, const char* line)
pub fn dock_settings_handler_read_line(g: &mut Context, handler: &mut SettingsHandler, data: &Vec<u8>, line: &str)
{
    char c = 0;
    int x = 0, y = 0;
    int r = 0;

    // Parsing, e.g.
    // " dock_node   id=0x00000001 pos=383,193 size=201,322 split=Y,0.506 "
    // "   dock_node id=0x00000002 Parent=0x00000001 "
    // Important: this code expect currently fields in a fixed order.
    ImGuiDockNodeSettings node;
    line = ImStrSkipBlank(line);
    if      (strncmp(line, "dock_node", 8) == 0)  { line = ImStrSkipBlank(line + strlen("dock_node")); }
    else if (strncmp(line, "DockSpace", 9) == 0) { line = ImStrSkipBlank(line + strlen("DockSpace")); node.flags |= DockNodeFlags::DockSpace; }
    else return;
    if (sscanf(line, "id=0x%08X%n",      &node.id, &r) == 1)            { line += r; } else return;
    if (sscanf(line, " Parent=0x%08X%n", &node.parent_node_id, &r) == 1)  { line += r; if (node.parent_node_id == 0) return; }
    if (sscanf(line, " window=0x%08X%n", &node.ParentWindowId, &r) ==1) { line += r; if (node.ParentWindowId == 0) return; }
    if (node.parent_node_id == 0)
    {
        if (sscanf(line, " pos=%i,%i%n",  &x, &y, &r) == 2)         { line += r; node.pos = Vector2Dih(x, y); } else return;
        if (sscanf(line, " size=%i,%i%n", &x, &y, &r) == 2)         { line += r; node.size = Vector2Dih(x, y); } else return;
    }
    else
    {
        if (sscanf(line, " size_ref=%i,%i%n", &x, &y, &r) == 2)      { line += r; node.size_ref = Vector2Dih(x, y); }
    }
    if (sscanf(line, " split=%c%n", &c, &r) == 1)                   { line += r; if (c == 'X') node.split_axis = Axis::X; else if (c == 'Y') node.split_axis = Axis::Y; }
    if (sscanf(line, " NoResize=%d%n", &x, &r) == 1)                { line += r; if (x != 0) node.flags |= DockNodeFlags::NoResize; }
    if (sscanf(line, " central_node=%d%n", &x, &r) == 1)             { line += r; if (x != 0) node.flags |= DockNodeFlags::CentralNode; }
    if (sscanf(line, " NoTabBar=%d%n", &x, &r) == 1)                { line += r; if (x != 0) node.flags |= DockNodeFlags::NoTabBar; }
    if (sscanf(line, " HiddenTabBar=%d%n", &x, &r) == 1)            { line += r; if (x != 0) node.flags |= DockNodeFlags::HiddenTabBar; }
    if (sscanf(line, " NoWindowMenuButton=%d%n", &x, &r) == 1)      { line += r; if (x != 0) node.flags |= DockNodeFlags::NoWindowMenuButton; }
    if (sscanf(line, " NoCloseButton=%d%n", &x, &r) == 1)           { line += r; if (x != 0) node.flags |= DockNodeFlags::NoCloseButton; }
    if (sscanf(line, " Selected=0x%08X%n", &node.selected_tab_id,&r) == 1) { line += r; }
    if (node.parent_node_id != 0)
        if (ImGuiDockNodeSettings* parent_settings = DockSettingsFindNodeSettings(.g, node.parent_node_id))
            node.Depth = parent_settings.Depth + 1;
    .g.dock_context.NodesSettings.push_back(node);
}

// static void DockSettingsHandler_DockNodeToSettings(ImGuiDockContext* dc, ImGuiDockNode* node, int depth)
pub fn dock_settings_handler_dock_node_to_settings(g: &mut Context, dc: &mut DockContext, node: &mut DockNode, depth: i32)
{
    ImGuiDockNodeSettings node_settings;
    // IM_ASSERT(depth < (1 << (sizeof(node_settings.Depth) << 3)));
    node_settings.id = node.id;
    node_settings.parent_node_id = node.parent_node ? node.parent_node.id : 0;
    node_settings.ParentWindowId = (node.IsDockSpace() && node.host_window_id && node.host_window_id.parent_window) ? node.host_window_id.parent_window.id : 0;
    node_settings.selected_tab_id = node.selected_tab_id;
    node_settings.split_axis = (signed char)(node.is_split_node() ? node.split_axis : ImGuiAxis_None);
    node_settings.Depth = (char)depth;
    node_settings.flags = (node.LocalFlags & DockNodeFlags::SavedFlagsMask_);
    node_settings.pos = Vector2Dih(node.pos);
    node_settings.size = Vector2Dih(node.size);
    node_settings.size_ref = Vector2Dih(node.size_ref);
    dc.NodesSettings.push_back(node_settings);
    if (node.child_nodes[0])
        DockSettingsHandler::DockNodeToSettings(dc, node.child_nodes[0], depth + 1);
    if (node.child_nodes[1])
        DockSettingsHandler::DockNodeToSettings(dc, node.child_nodes[1], depth + 1);
}

// static void DockSettingsHandler_WriteAll(ImGuiContext* ctx, ImGuiSettingsHandler* handler, ImGuiTextBuffer* buf)
pub fn dock_settings_handler_write_all(g: &mut Context, handler: &mut SettingsHandler, bug: &mut TextBuffer)
{
    // ImGuiContext& g = *.g;
    ImGuiDockContext* dc = &.g.dock_context;
    if (!(g.io.config_flags & ImGuiConfigFlags_DockingEnable))
        return;

    // Gather settings data
    // (unlike our windows settings, because nodes are always built we can do a full rewrite of the SettingsNode buffer)
    dc.NodesSettings.resize(0);
    dc.NodesSettings.reserve(dc.Nodes.data.size);
    for (int n = 0; n < dc.Nodes.data.size; n += 1)
        if (ImGuiDockNode* node = (ImGuiDockNode*)dc.Nodes.data[n].val_p)
            if (node.is_root_node())
                DockSettingsHandler::DockNodeToSettings(dc, node, 0);

    int max_depth = 0;
    for (int node_n = 0; node_n < dc.NodesSettings.size; node_n += 1)
        max_depth = ImMax(dc.NodesSettings[node_n].Depth, max_depth);

    // Write to text buffer
    buf.appendf("[%s][data]\n", handler.TypeName);
    for (int node_n = 0; node_n < dc.NodesSettings.size; node_n += 1)
    {
        const int line_start_pos = buf->size(); (void)line_start_pos;
        const ImGuiDockNodeSettings* node_settings = &dc.NodesSettings[node_n];
        buf.appendf("%*s%s%*s", node_settings.Depth * 2, "", (node_settings.flags & DockNodeFlags::DockSpace) ? "DockSpace" : "dock_node ", (max_depth - node_settings.Depth) * 2, "");  // Text align nodes to facilitate looking at .ini file
        buf.appendf(" id=0x%08X", node_settings->ID);
        if (node_settings->parent_node_id)
        {
            buf->appendf(" Parent=0x%08X size_ref=%d,%d", node_settings->parent_node_id, node_settings.size_ref.x, node_settings.size_ref.y);
        }
        else
        {
            if (node_settings->ParentWindowId)
                buf->appendf(" window=0x%08X", node_settings->ParentWindowId);
            buf->appendf(" pos=%d,%d size=%d,%d", node_settings.pos.x, node_settings.pos.y, node_settings.size.x, node_settings.size.y);
        }
        if (node_settings->SplitAxis != ImGuiAxis_None)
            buf->appendf(" split=%c", (node_settings->SplitAxis == Axis::X) ? 'X' : 'Y');
        if (node_settings.flags & DockNodeFlags::NoResize)
            buf->appendf(" NoResize=1");
        if (node_settings.flags & DockNodeFlags::CentralNode)
            buf->appendf(" central_node=1");
        if (node_settings.flags & DockNodeFlags::NoTabBar)
            buf->appendf(" NoTabBar=1");
        if (node_settings.flags & DockNodeFlags::HiddenTabBar)
            buf->appendf(" HiddenTabBar=1");
        if (node_settings.flags & DockNodeFlags::NoWindowMenuButton)
            buf->appendf(" NoWindowMenuButton=1");
        if (node_settings.flags & DockNodeFlags::NoCloseButton)
            buf->appendf(" NoCloseButton=1");
        if (node_settings->SelectedTabId)
            buf->appendf(" Selected=0x%08X", node_settings->SelectedTabId);

// #ifIMGUI_DEBUG_INI_SETTINGS
        // [DEBUG] Include comments in the .ini file to ease debugging
        if (ImGuiDockNode* node = dock_context_find_node_by_id(.g, node_settings->ID))
        {
            buf->appendf("%*s", ImMax(2, (line_start_pos + 92) - buf->size()), "");     // Align everything
            if (node->IsDockSpace() && node->HostWindow && node->HostWindow->parent_window)
                buf->appendf(" ; in '%s'", node->HostWindow->parent_window->Name);
            // Iterate settings so we can give info about windows that didn't exist during the session.
            int contains_window = 0;
            for (ImGuiWindowSettings* settings = g.settings_windows.begin(); settings != NULL; settings = g.settings_windows.next_chunk(settings))
                if (settings.dock_id == node_settings->ID)
                {
                    if (contains_window += 1 == 0)
                        buf->appendf(" ; contains ");
                    buf->appendf("'%s' ", settings->GetName());
                }
        }

        buf->appendf("\n");
    }
    buf->appendf("\n");
}
