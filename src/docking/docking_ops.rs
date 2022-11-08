
//-----------------------------------------------------------------------------
// Docking: Public Functions (SetWindowDock, DockSpace, DockSpaceOverViewport)
//-----------------------------------------------------------------------------
// - SetWindowDock() [Internal]
// - DockSpace()
// - DockSpaceOverViewport()
//-----------------------------------------------------------------------------

use std::ptr::null_mut;
use libc::{c_char, c_int, strlen};
use crate::core::condition::{ImGuiCond, ImGuiCond_Always, ImGuiCond_Appearing, ImGuiCond_FirstUseEver, ImGuiCond_Once};
use crate::core::context::ImguiContext;
use crate::docking::dock_node::ImGuiDockNode;
use crate::{GImGui, SettingsHandler, hash_string};
use crate::core::axis::{IM_GUI_AXIS_X, IM_GUI_AXIS_Y};
use crate::color::color_ops::ColorConvertFloat4ToU32;
use crate::core::config_flags::ImGuiConfigFlags_DockingEnable;
use crate::docking::dock_context::ImGuiDockContext;
use crate::dock_context_ops::DockContextFindNodeByID;
use crate::docking::dock_node_flags::ImGuiDockNodeFlags_CentralNode;
use crate::drag_drop::drag_drop_flags::{ImGuiDragDropFlags_AcceptBeforeDelivery, ImGuiDragDropFlags_AcceptNoDrawDefaultRect, ImGuiDragDropFlags_SourceAutoExpirePayload, ImGuiDragDropFlags_SourceNoHoldToOpenOthers, ImGuiDragDropFlags_SourceNoPreviewTooltip};
use crate::frame_ops::GetFrameHeight;
use crate::input_ops::IsMouseHoveringRect;
use crate::math_ops::ImMax;
use crate::next_window_data_flags::ImGuiNextWindowDataFlags_HasPos;
use crate::payload::ImGuiPayload;
use crate::rect::ImRect;
use crate::type_defs::{ImguiHandle, INVALID_IMGUI_HANDLE};
use crate::vec2::ImVec2;
use crate::window::find::{FindWindowByID, FindWindowByName};
use crate::window::ImguiWindow;
use crate::window::ops::SetNextWindowSize;
use crate::window::props::SetNextWindowPos;
use crate::window::window_dock_style_colors::GWindowDockStyleColors;
use crate::window::window_flags::{ImGuiWindowFlags_AlwaysUseWindowPadding, ImGuiWindowFlags_ChildWindow, ImGuiWindowFlags_NoDocking, ImGuiWindowFlags_NoResize, ImGuiWindowFlags_NoTitleBar};
use crate::window::window_settings::ImGuiWindowSettings;

// [Internal] Called via SetNextWindowDockID()
pub unsafe fn SetWindowDock(window: &mut ImguiWindow, dock_id: ImguiHandle, cond: ImGuiCond)
{
    // Test condition (NB: bit 0 is always true) and clear flags for next time
    if cond && (window.SetWindowDockAllowFlags & cond) == 0 { return ; }
    window.SetWindowDockAllowFlags &= ~(ImGuiCond_Once | ImGuiCond_FirstUseEver | ImGuiCond_Appearing);

    if window.DockId == dock_id { return ; }

    // If the user attempt to set a dock id that is a split node, we'll dig within to find a suitable docking spot
    g: &mut ImguiContext = GImGui;
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

    if window.DockId == dock_id { return ; }

    if (window.DockNode)
        DockNodeRemoveWindow(window.DockNode, window, 0);
    window.DockId = dock_id;
}

// Create an explicit dockspace node within an existing window. Also expose dock node flags and creates a CentralNode by default.
// The Central Node is always displayed even when empty and shrink/extend according to the requested size of its neighbors.
// DockSpace() needs to be submitted _before_ any window they can host. If you use a dockspace, submit it early in your app.
DockSpace: ImguiHandle(id: ImguiHandle, size_arg: &ImVec2, ImGuiDockNodeFlags flags, *const ImGuiWindowClass window_class)
{
    g: &mut ImguiContext = GImGui;
    let g =  ctx;
    let mut window: &mut ImGuiWindow =  GetCurrentWindow();
    if !(g.IO.ConfigFlags & ImGuiConfigFlags_DockingEnable) { return  0; }

    // Early out if parent window is hidden/collapsed
    // This is faster but also DockNodeUpdateTabBar() relies on TabBarLayout() running (which won't if SkipItems=true) to set NextSelectedTabId = 0). See #2960.
    // If for whichever reason this is causing problem we would need to ensure that DockNodeUpdateTabBar() ends up clearing NextSelectedTabId even if SkipItems=true.
    if (window.skip_items)
        flags |= ImGuiDockNodeFlags_KeepAliveOnly;

    // IM_ASSERT(flag_set(flags, ImGuiDockNodeFlags_DockSpace) == 0);
    // IM_ASSERT(id != 0);
    node:*mut ImGuiDockNode = DockContextFindNodeByID(ctx, id);
    if (!node)
    {
        IMGUI_DEBUG_LOG_DOCKING("[docking] DockSpace: dockspace node 0x{} created\n", id);
        node = DockContextAddNode(ctx, id);
        node.SetLocalFlags(ImGuiDockNodeFlags_CentralNode);
    }
    if (window_class && window_class.ClassId != node.WindowClass.ClassId)
        IMGUI_DEBUG_LOG_DOCKING("[docking] DockSpace: dockspace node 0x{}: setup WindowClass 0x{} -> 0x{}\n", id, node.WindowClass.ClassId, window_class.ClassId);
    node.SharedFlags = flags;
    node.WindowClass = if window_class { * window_class} else {ImGuiWindowClass()};

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

    node.Pos = window.dc.cursor_pos;
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
    ImFormatString(title, title.len(), "{}/DockSpace_{}", window.Name, id);

    PushStyleVar(ImGuiStyleVar_ChildBorderSize, 0.0);
    Begin(title, None, window_flags);
    PopStyleVar();

    let mut host_window: &mut ImGuiWindow =  g.CurrentWindow;
    DockNodeSetupHostWindow(node, host_window);
    host_window.ChildId = window.GetID(title);
    node.OnlyNodeWithWindows= None;

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
DockSpaceOverViewport: ImguiHandle(*const ImGuiViewport viewport, ImGuiDockNodeFlags dockspace_flags, *const ImGuiWindowClass window_class)
{
    if viewport == None {
        viewport = GetMainViewport()(); }

    SetNextWindowPos(viewport.WorkPos);
    SetNextWindowSize(viewport.WorkSize);
    SetNextWindowViewport(viewport.ID);

    host_window_flags: ImGuiWindowFlags = 0;
    host_window_flags |= ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoCollapse | ImGuiWindowFlags_NoResize | ImGuiWindowFlags_NoMove | ImGuiWindowFlags_NoDocking;
    host_window_flags |= ImGuiWindowFlags_NoBringToFrontOnFocus | ImGuiWindowFlags_NoNavFocus;
    if (dockspace_flags & ImGuiDockNodeFlags_PassthruCentralNode)
        host_window_flags |= ImGuiWindowFlags_NoBackground;

    label: [c_char;32];
    ImFormatString(label, label.len(), "DockSpaceViewport_{}", viewport.ID);

    PushStyleVar(ImGuiStyleVar_WindowRounding, 0.0);
    PushStyleVar(ImGuiStyleVar_WindowBorderSize, 0.0);
    PushStyleVar(ImGuiStyleVar_WindowPadding, ImVec2::new(0.0, 0.0));
    Begin(label, None, host_window_flags);
    PopStyleVar(3);

    let mut dockspace_id: ImguiHandle =  GetID("DockSpace");
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

pub unsafe fn DockBuilderDockWindow(window_name: *const c_char, node_id: ImguiHandle)
{
    // We don't preserve relative order of multiple docked windows (by clearing DockOrder back to -1)
    let mut window_id: ImguiHandle =  hash_string(window_name);
    if (let mut window: &mut ImguiWindow =  FindWindowByID(, window_id))
    {
        // Apply to created window
        SetWindowDock(window, node_id, ImGuiCond_Always);
        window.DockOrder = -1;
    }
    else
    {
        // Apply to settings
        settings: *mut ImGuiWindowSettings = FindWindowSettings(window_id);
        if settings == None {
            settings = CreateNewWindowSettings(window_name)(); }
        settings.DockId = node_id;
        settings.DockOrder = -1;
    }
}

DockBuilderGetNode:*mut ImGuiDockNode(node_id: ImguiHandle)
{
    g: &mut ImguiContext = GImGui;
    return DockContextFindNodeByID(ctx, node_id);
}

pub unsafe fn DockBuilderSetNodePos(node_id: ImguiHandle, pos: ImVec2)
{
    g: &mut ImguiContext = GImGui;
    node:*mut ImGuiDockNode = DockContextFindNodeByID(ctx, node_id);
    if node == None { return ; }
    node.Pos = pos;
    node.AuthorityForPos = ImGuiDataAuthority_DockNode;
}

pub unsafe fn DockBuilderSetNodeSize(node_id: ImguiHandle, size: ImVec2)
{
    g: &mut ImguiContext = GImGui;
    node:*mut ImGuiDockNode = DockContextFindNodeByID(ctx, node_id);
    if node == None { return ; }
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
DockBuilderAddNode: ImguiHandle(id: ImguiHandle, ImGuiDockNodeFlags flags)
{
    g: &mut ImguiContext = GImGui;

    if (id != 0)
        DockBuilderRemoveNode(id);

    node:*mut ImGuiDockNode= None;
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

pub unsafe fn DockBuilderRemoveNode(node_id: ImguiHandle)
{
    g: &mut ImguiContext = GImGui;
    node:*mut ImGuiDockNode = DockContextFindNodeByID(ctx, node_id);
    if node == None { return ; }
    DockBuilderRemoveNodeDockedWindows(g, node_id, true);
    DockBuilderRemoveNodeChildNodes(g, node_id);
    // Node may have moved or deleted if e.g. any merge happened
    node = DockContextFindNodeByID(ctx, node_id);
    if node == None { return ; }
    if (node.IsCentralNode() && node.ParentNode)
        node.ParentNode.SetLocalFlags(node.ParentNode.LocalFlags | ImGuiDockNodeFlags_CentralNode);
    DockContextRemoveNode(ctx, node, true);
}

// root_id = 0 to remove all, root_id != 0 to remove child of given node.
pub fn DockBuilderRemoveNodeChildNodes(g: &mut ImguiContext, root_id: ImguiHandle)
{
    let dc = &g.dock_context;
    let root_node = if root_id { DockContextFindNodeByID(g, root_id) } else { None };
    if root_id != INVALID_IMGUI_HANDLE && root_node.is_none() { return; }
    let mut has_central_node: bool =  false;

    ImGuiDataAuthority backup_root_node_authority_for_pos = if root_node { root_node.AuthorityForPos } else { ImGuiDataAuthority_Auto };
    ImGuiDataAuthority backup_root_node_authority_for_size = if root_node { root_node.AuthorityForSize } else { ImGuiDataAuthority_Auto };

    // Process active windows
    Vec<ImGuiDockNode*> nodes_to_remove;
    for (let n: c_int = 0; n < dc.Nodes.Data.Size; n++)
        if (node:*mut ImGuiDockNode = dc.Nodes.Data[n].val_p)
        {
            let mut want_removal: bool =  (root_id == 0) || (node.ID != root_id && DockNodeGetRootNode(node).ID == root_id);
            if (want_removal)
            {
                if node.IsCentralNode() {
                    has_central_node = true;}
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
    for (settings: *mut ImGuiWindowSettings = ctx->SettingsWindows.begin(); settings != None; settings = ctx->SettingsWindows.next_chunk(settings))
        if (let mut window_settings_dock_id: ImguiHandle =  settings.DockId)
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
        dc.Nodes.Clear();
        dc.Requests.clear();
    }
    else if has_central_node
    {
        root_node.CentralNode = root_node;
        root_node.SetLocalFlags(root_node.LocalFlags | ImGuiDockNodeFlags_CentralNode);
    }
}

pub fn DockBuilderRemoveNodeDockedWindows(
    g: &mut ImguiContext, root_id: ImguiHandle,
    clear_settings_refs: bool)
{
    if clear_settings_refs
    {
        for (settings: *mut ImGuiWindowSettings = g.SettingsWindows.begin(); settings != None; settings = g.SettingsWindows.next_chunk(settings))
        {
            let mut want_removal: bool =  (root_id == 0) || (settings.DockId == root_id);
            if (!want_removal && settings.DockId != 0)
                if (node:*mut ImGuiDockNode = DockContextFindNodeByID(ctx, settings.DockId))
                    if DockNodeGetRootNode(node).ID == root_id {
                        want_removal = true;}
            if want_removal{
                settings.DockId = 0;}
        }
    }

    // Clear references in windows
    for (let n: c_int = 0; n < g.Windows.len(); n++)
    {
        let mut window: &mut ImguiWindow =  g.Windows[n];
        let mut want_removal: bool =  (root_id == 0) || (window.DockNode && DockNodeGetRootNode(window.DockNode).ID == root_id) || (window.DockNodeAsHost && window.DockNodeAsHost.ID == root_id);
        if (want_removal)
        {
            let mut backup_dock_id: ImguiHandle =  window.DockId;
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
DockBuilderSplitNode: ImguiHandle(id: ImguiHandle, split_dir: ImGuiDir,size_ratio_for_node_at_dir: c_float, ImguiHandle* out_id_at_dir, ImguiHandle* out_id_at_opposite_dir)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(split_dir != ImGuiDir_None);
    IMGUI_DEBUG_LOG_DOCKING("[docking] DockBuilderSplitNode: node 0x{}, split_dir {}\n", id, split_dir);

    node:*mut ImGuiDockNode = DockContextFindNodeByID(&g, id);
    if (node == null_mut())
    {
        // IM_ASSERT(node != NULL);
        return 0;
    }

    // IM_ASSERT(!node.IsSplitNode()); // Assert if already Split

    req: ImGuiDockRequest;
    req.Type = ImGuiDockRequestType_Split;
    req.DockTargetWindow= None;
    req.DockTargetNode = node;
    req.DockPayload= None;
    req.DockSplitDir = split_dir;
    req.DockSplitRatio = ImSaturate(if (split_dir == ImGuiDir_Left || split_dir == ImGuiDir_Up) { size_ratio_for_node_at_dir }else {1.0 - size_ratio_for_node_at_dir});
    req.DockSplitOuter = false;
    DockContextProcessDock(&g, &req);

    let mut id_at_dir: ImguiHandle =  node.ChildNodes[if (split_dir == ImGuiDir_Left || split_dir == ImGuiDir_Up) { 0} else {1}].ID;
    let mut id_at_opposite_dir: ImguiHandle =  node.ChildNodes[if (split_dir == ImGuiDir_Left || split_dir == ImGuiDir_Up) { 1} else {0}].ID;
    if (out_id_at_dir)
        *out_id_at_dir = id_at_dir;
    if (out_id_at_opposite_dir)
        *out_id_at_opposite_dir = id_at_opposite_dir;
    return id_at_dir;
}

static DockBuilderCopyNodeRec:*mut ImGuiDockNode(src_node:*mut ImGuiDockNode, dst_node_id_if_known: ImguiHandle, Vec<ImguiHandle>* out_node_remap_pairs)
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

    IMGUI_DEBUG_LOG_DOCKING("[docking] Fork node {} -> {} ({} childs)\n", src_node.ID, dst_node.ID, if dst_node.IsSplitNode() { 2 }else {0});
    return dst_node;
}

pub unsafe fn DockBuilderCopyNode(src_node_id: ImguiHandle, dst_node_id: ImguiHandle, Vec<ImguiHandle>* out_node_remap_pairs)
{
    g: &mut ImguiContext = GImGui;
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
    let mut src_window: &mut ImguiWindow =  FindWindowByName(src_name, );
    if src_window == None { return ; }
    if (let mut dst_window: &mut ImguiWindow =  FindWindowByName(dst_name, ))
    {
        dst_window.position = src_window.position;
        dst_window.Size = src_window.Size;
        dst_window.SizeFull = src_window.SizeFull;
        dst_window.Collapsed = src_window.Collapsed;
    }
    else if (dst_settings: *mut ImGuiWindowSettings = FindOrCreateWindowSettings(dst_name))
    {
        ImVec2ih window_pos_2ih = ImVec2ih(src_window.position);
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
pub unsafe fn DockBuilderCopyDockSpace(src_dockspace_id: ImguiHandle, dst_dockspace_id: ImguiHandle, Vec<*const char>* in_window_remap_pairs)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(src_dockspace_id != 0);
    // IM_ASSERT(dst_dockspace_id != 0);
    // IM_ASSERT(in_window_remap_pairs != NULL);
    // IM_ASSERT((in_window_remap_pairs.Size % 2) == 0);

    // Duplicate entire dock
    // FIXME: When overwriting dst_dockspace_id, windows that aren't part of our dockspace window class but that are docked in a same node will be split apart,
    // whereas we could attempt to at least keep them together in a new, same floating node.
    Vec<ImguiHandle> node_remap_pairs;
    DockBuilderCopyNode(src_dockspace_id, dst_dockspace_id, &node_remap_pairs);

    // Attempt to transition all the upcoming windows associated to dst_dockspace_id into the newly created hierarchy of dock nodes
    // (The windows associated to src_dockspace_id are staying in place)
    Vec<ImguiHandle> src_windows;
    for (let remap_window_n: c_int = 0; remap_window_n < in_window_remap_pairs.Size; remap_window_n += 2)
    {
        let mut  src_window_name: *const c_char = (*in_window_remap_pairs)[remap_window_n];
        let mut  dst_window_name: *const c_char = (*in_window_remap_pairs)[remap_window_n + 1];
        let mut src_window_id: ImguiHandle =  hash_string(src_window_name);
        src_windows.push(src_window_id);

        // Search in the remapping tables
        let mut src_dock_id: ImguiHandle =  0;
        if (let mut src_window: &mut ImguiWindow =  FindWindowByID(, src_window_id))
            src_dock_id = src_window.DockId;
        else if (src_window_settings: *mut ImGuiWindowSettings = FindWindowSettings(src_window_id))
            src_dock_id = src_window_settings->DockId;
        let mut dst_dock_id: ImguiHandle =  0;
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
            IMGUI_DEBUG_LOG_DOCKING("[docking] Remap live window '{}' 0x{} -> '{}' 0x{}\n", src_window_name, src_dock_id, dst_window_name, dst_dock_id);
            DockBuilderDockWindow(dst_window_name, dst_dock_id);
        }
        else
        {
            // Floating windows gets their settings transferred (regardless of whether the new window already exist or not)
            // When this is leading to a Copy and not a Move, we would get two overlapping floating windows. Could we possibly dock them together?
            IMGUI_DEBUG_LOG_DOCKING("[docking] Remap window settings '{}' -> '{}'\n", src_window_name, dst_window_name);
            DockBuilderCopyWindowSettings(src_window_name, dst_window_name);
        }
    }

    // Anything else in the source nodes of 'node_remap_pairs' are windows that were docked in src_dockspace_id but are not owned by it (unaffiliated windows, e.g. "ImGui Demo")
    // Find those windows and move to them to the cloned dock node. This may be optional?
    for (let dock_remap_n: c_int = 0; dock_remap_n < node_remap_pairs.Size; dock_remap_n += 2)
        if (let mut src_dock_id: ImguiHandle =  node_remap_pairs[dock_remap_n])
        {
            let mut dst_dock_id: ImguiHandle =  node_remap_pairs[dock_remap_n + 1];
            node:*mut ImGuiDockNode = DockBuilderGetNode(src_dock_id);
            for (let window_n: c_int = 0; window_n < node.Windows.len(); window_n++)
            {
                let mut window: &mut ImguiWindow =  node.Windows[window_n];
                if src_windows.contains(window.ID){
                    continue;}

                // Docked windows gets redocked into the new node hierarchy.
                IMGUI_DEBUG_LOG_DOCKING("[docking] Remap window '{}' {} -> {}\n", window.Name, src_dock_id, dst_dock_id);
                DockBuilderDockWindow(window.Name, dst_dock_id);
            }
        }
}

// FIXME-DOCK: This is awkward because in series of split user is likely to loose access to its root node.
pub unsafe fn DockBuilderFinish(root_id: ImguiHandle)
{
    g: &mut ImguiContext = GImGui;
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

pub unsafe fn GetWindowAlwaysWantOwnTabBar(window: &mut ImguiWindow) -> bool
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.IO.ConfigDockingAlwaysTabBar || window.WindowClass.DockingAlwaysTabBar)
        if ((window.Flags & (ImGuiWindowFlags_ChildWindow | ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoDocking)) == 0)
            if (!window.IsFallbackWindow)    // We don't support AlwaysTabBar on the fallback/implicit window to avoid unused dock-node overhead/noise
                return true;
    return false;
}

static DockContextBindNodeToWindow:*mut ImGuiDockNode(g: &mut ImguiContext, window: &mut ImGuiWindow)
{
    let g =  ctx;
    node:*mut ImGuiDockNode = DockContextFindNodeByID(ctx, window.DockId);
    // IM_ASSERT(window.DockNode == NULL);

    // We should not be docking into a split node (SetWindowDock should avoid this)
    if (node && node.IsSplitNode())
    {
        DockContextProcessUndockWindow(ctx, window);
        return None;
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

pub unsafe fn BeginDocked(window: &mut ImguiWindow, p_open: *mut bool)
{
    g: &mut ImguiContext = GImGui;
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
        want_undock |= flag_set(window.Flags, ImGuiWindowFlags_NoDocking) != 0;
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
        if node == None { return ; }
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
        window.DockStyle.Colors[color_n] = ColorConvertFloat4ToU32(g.style.Colors[GWindowDockStyleColors[color_n]]);

    // Fast path return. It is common for windows to hold on a persistent DockId but be the only visible window,
    // and never create neither a host window neither a tab bar.
    // FIXME-DOCK: replace ->HostWindow NULL compare with something more explicit (~was initially intended as a first frame test)
    if (node.HostWindow == null_mut())
    {
        if node.State == ImGuiDockNodeState_HostWindowHiddenBecauseWindowsAreResizing{
            window.DockIsActive = true;}
        if node.Windows.len() > 1{
            DockNodeHideWindowDuringHostWindowCreation(window);}
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
    if node.MergedFlags & ImGuiDockNodeFlags_KeepAliveOnly { return ; }

    // When the window is selected we mark it as visible.
    if node.VisibleWindow == window{
        window.DockTabIsVisible = true;}

    // Update window flag
    // IM_ASSERT((window.Flags & ImGuiWindowFlags_ChildWindow) == 0);
    window.Flags |= ImGuiWindowFlags_ChildWindow | ImGuiWindowFlags_AlwaysUseWindowPadding | ImGuiWindowFlags_NoResize;
    if (node.IsHiddenTabBar() || node.IsNoTabBar())
        window.Flags |= ImGuiWindowFlags_NoTitleBar;
    else
        window.Flags &= !ImGuiWindowFlags_NoTitleBar;      // Clear the NoTitleBar flag in case the user set it: confusingly enough we need a title bar height so we are correctly offset, but it won't be displayed!

    // Save new dock order only if the window has been visible once already
    // This allows multiple windows to be created in the same frame and have their respective dock orders preserved.
    if node.TabBar && window.WasActive{
        window.DockOrder = DockNodeGetTabOrder(window);}

    if ((node.WantCloseAll || node.WantCloseTabId == window.TabId) && p_open != null_mut())
        *p_open = false;

    // Update ChildId to allow returning from Child to Parent with Escape
    let mut parent_window: &mut ImguiWindow =  window.DockNode.HostWindow;
    window.ChildId = parent_window.id_by_string(window.Name, );
}

pub unsafe fn BeginDockableDragDropSource(window: &mut ImguiWindow)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.ActiveId == window.MoveId);
    // IM_ASSERT(g.MovingWindow == window);
    // IM_ASSERT(g.CurrentWindow == window);

    g.last_item_data.ID = window.MoveId;
    window = window.RootWindowDockTree;
    // IM_ASSERT((window.Flags & ImGuiWindowFlags_NoDocking) == 0);
    let mut is_drag_docking: bool =  (g.IO.ConfigDockingWithShift) || ImRect(0, 0, window.SizeFull.x, GetFrameHeight()).Contains(g.ActiveIdClickOffset); // FIXME-DOCKING: Need to make this stateful and explicit
    if (is_drag_docking && BeginDragDropSource(ImGuiDragDropFlags_SourceNoPreviewTooltip | ImGuiDragDropFlags_SourceNoHoldToOpenOthers | ImGuiDragDropFlags_SourceAutoExpirePayload))
    {
        SetDragDropPayload(IMGUI_PAYLOAD_TYPE_WINDOW, &window, sizeof(window));
        EndDragDropSource();

        // Store style overrides
        for (let color_n: c_int = 0; color_n < ImGuiWindowDockStyleCol_COUNT; color_n++)
            window.DockStyle.Colors[color_n] = ColorConvertFloat4ToU32(g.style.Colors[GWindowDockStyleColors[color_n]]);
    }
}

pub unsafe fn BeginDockableDragDropTarget(window: &mut ImguiWindow)
{
    g: &mut ImguiContext = GImGui;
    let g =  ctx;

    //IM_ASSERT(window.RootWindowDockTree == window); // May also be a DockSpace
    // IM_ASSERT((window.Flags & ImGuiWindowFlags_NoDocking) == 0);
    if !g.DragDropActive { return ; }
    //GetForegroundDrawList(window).AddRect(window.position, window.position + window.Size, IM_COL32(255, 255, 0, 255));
    if !BeginDragDropTargetCustom(window.Rect(), window.ID) { return ; }

    // Peek into the payload before calling AcceptDragDropPayload() so we can handle overlapping dock nodes with filtering
    // (this is a little unusual pattern, normally most code would call AcceptDragDropPayload directly)
    let payload: *const ImGuiPayload = &g.DragDropPayload;
    if (!payload->IsDataType(IMGUI_PAYLOAD_TYPE_WINDOW) || !DockNodeIsDropAllowed(window, *(ImGuiWindow**)payload.Data))
    {
        EndDragDropTarget();
        return;
    }

    let mut payload_window: &mut ImguiWindow =  *(ImGuiWindow**)payload.Data;
    if (AcceptDragDropPayload(IMGUI_PAYLOAD_TYPE_WINDOW, ImGuiDragDropFlags_AcceptBeforeDelivery | ImGuiDragDropFlags_AcceptNoDrawDefaultRect))
    {
        // Select target node
        // (Important: we cannot use g.HoveredDockNode here! Because each of our target node have filters based on payload, each candidate drop target will do its own evaluation)
        let mut dock_into_floating_window: bool =  false;
        node:*mut ImGuiDockNode= None;
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
            if window.DockNode{
                node = window.DockNode;}
            else
                dock_into_floating_window = true; // Dock into a regular window
        }

        let explicit_target_rect: ImRect =  if (node && node.TabBar && !node.IsHiddenTabBar() && !node.IsNoTabBar()) { node.TabBar.BarRect } else { ImRect(window.position, window.position + ImVec2::from_floats(window.Size.x, GetFrameHeight())) };
        let is_explicit_target: bool = g.IO.ConfigDockingWithShift || IsMouseHoveringRect(explicit_target_rect.min, explicit_target_rect.max);

        // Preview docking request and find out split direction/ratio
        //let do_preview: bool = true;     // Ignore testing for payload->IsPreview() which removes one frame of delay, but breaks overlapping drop targets within the same window.
        let do_preview: bool = payload->IsPreview() || payload->IsDelivery();
        if (do_preview && (node != None || dock_into_floating_window))
        {
            let mut split_inner = ImGuiDockPreviewData::default();
            let mut split_outer = ImGuiDockPreviewData::default();
            split_data: *mut ImGuiDockPreviewData = &split_inner;
            if (node && (node.ParentNode || node.IsCentralNode()))
                if (root_node:*mut ImGuiDockNode = DockNodeGetRootNode(node))
                {
                    DockNodePreviewDockSetup(window, root_node, payload_window, None, &split_outer, is_explicit_target, true);
                    if (split_outer.IsSplitDirExplicit)
                        split_data = &split_outer;
                }
            DockNodePreviewDockSetup(window, node, payload_window, None, &split_inner, is_explicit_target, false);
            if split_data == &split_outer{
                split_inner.IsDropAllowed = false;}

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

pub unsafe fn DockSettingsRenameNodeReferences(old_node_id: ImguiHandle, new_node_id: ImguiHandle)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    IMGUI_DEBUG_LOG_DOCKING("[docking] DockSettingsRenameNodeReferences: from 0x{} -> to 0x{}\n", old_node_id, new_node_id);
    for (let window_n: c_int = 0; window_n < g.Windows.len(); window_n++)
    {
        let mut window: &mut ImguiWindow =  g.Windows[window_n];
        if window.DockId == old_node_id && window.DockNode == None{
            window.DockId = new_node_id;}
    }
    //// FIXME-OPT: We could remove this loop by storing the index in the map
    for (settings: *mut ImGuiWindowSettings = g.SettingsWindows.begin(); settings != None; settings = g.SettingsWindows.next_chunk(settings))
        if settings.DockId == old_node_id{
            settings.DockId = new_node_id;}
}

// Remove references stored in ImGuiWindowSettings to the given ImGuiDockNodeSettings
pub unsafe fn DockSettingsRemoveNodeReferences(ImguiHandle* node_ids, node_ids_count: c_int)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let found: c_int = 0;
    //// FIXME-OPT: We could remove this loop by storing the index in the map
    for (settings: *mut ImGuiWindowSettings = g.SettingsWindows.begin(); settings != None; settings = g.SettingsWindows.next_chunk(settings))
        for (let node_n: c_int = 0; node_n < node_ids_count; node_n++)
            if (settings.DockId == node_ids[node_n])
            {
                settings.DockId = 0;
                settings.DockOrder = -1;
                if ++found < node_ids_count {
                    break(); }
                return;
            }
}

static DockSettingsFindNodeSettings: *mut ImGuiDockNodeSettings(g: &mut ImguiContext, id: ImguiHandle)
{
    // FIXME-OPT
    dc: *mut ImGuiDockContext  = &ctx.DockContext;
    for (let n: c_int = 0; n < dc.NodesSettings.Size; n++)
        if (dc.NodesSettings[n].ID == id)
            return &dc.NodesSettings[n];
    return None;
}

// Clear settings data
pub unsafe fn DockSettingsHandler_ClearAll(g: &mut ImguiContext, SettingsHandler*)
{
    dc: *mut ImGuiDockContext  = &ctx.DockContext;
    dc.NodesSettings.clear();
    DockContextClearNodes(ctx, 0, true);
}

// Recreate nodes based on settings data
pub unsafe fn DockSettingsHandler_ApplyAll(g: &mut ImguiContext, SettingsHandler*)
{
    // Prune settings at boot time only
    dc: *mut ImGuiDockContext  = &ctx.DockContext;
    if ctx.Windows.len() == 0{
        DockContextPruneUnusedSettingsNodes(ctx);}
    DockContextBuildNodesFromSettings(ctx, dc.NodesSettings.Data, dc.NodesSettings.Size);
    DockContextBuildAddWindowsToNodes(ctx, 0);
}

static DockSettingsHandler_ReadOpen: *mut c_void(ImguiContext *, ImGuiSettingsHandler*, name: *const c_char)
{
    if (strcmp(name, "Data") != 0)
        return None;
    return 1;
}

pub unsafe fn DockSettingsHandler_ReadLine(g: &mut ImguiContext, SettingsHandler*, * mut c_void, line: *const c_char)
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
    if (sscanf(line, "ID=0x{}%n",      &node.ID, &r) == 1)            { line += r; } else return;
    if sscanf(line, " Parent=0x{}%n", &node.ParentNodeId, &r) == 1)  { line += r; if (node.ParentNodeId == 0 { return ; } }
    if sscanf(line, " Window=0x{}%n", &node.ParentWindowId, &r) ==1) { line += r; if (node.ParentWindowId == 0 { return ; } }
    if (node.ParentNodeId == 0)
    {
        if (sscanf(line, " Pos=%i,%i%n",  &x, &y, &r) == 2)         { line += r; node.Pos = ImVec2ih(x, y); } else return;
        if (sscanf(line, " Size=%i,%i%n", &x, &y, &r) == 2)         { line += r; node.Size = ImVec2ih(x, y); } else return;
    }
    else
    {
        if (sscanf(line, " SizeRef=%i,%i%n", &x, &y, &r) == 2)      { line += r; node.SizeRef = ImVec2ih(x, y); }
    }
    if (sscanf(line, " Split={}%n", &c, &r) == 1)                   { line += r; if (c == 'X') node.SplitAxis = IM_GUI_AXIS_X; else if (c == 'Y') node.SplitAxis = IM_GUI_AXIS_Y; }
    if (sscanf(line, " NoResize={}%n", &x, &r) == 1)                { line += r; if (x != 0) node.Flags |= ImGuiDockNodeFlags_NoResize; }
    if (sscanf(line, " CentralNode={}%n", &x, &r) == 1)             { line += r; if (x != 0) node.Flags |= ImGuiDockNodeFlags_CentralNode; }
    if (sscanf(line, " NoTabBar={}%n", &x, &r) == 1)                { line += r; if (x != 0) node.Flags |= ImGuiDockNodeFlags_NoTabBar; }
    if (sscanf(line, " HiddenTabBar={}%n", &x, &r) == 1)            { line += r; if (x != 0) node.Flags |= ImGuiDockNodeFlags_HiddenTabBar; }
    if (sscanf(line, " NoWindowMenuButton={}%n", &x, &r) == 1)      { line += r; if (x != 0) node.Flags |= ImGuiDockNodeFlags_NoWindowMenuButton; }
    if (sscanf(line, " NoCloseButton={}%n", &x, &r) == 1)           { line += r; if (x != 0) node.Flags |= ImGuiDockNodeFlags_NoCloseButton; }
    if (sscanf(line, " Selected=0x{}%n", &node.SelectedTabId,&r) == 1) { line += r; }
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
    node_settings.ParentNodeId = if node.ParentNode { node.ParentNode.ID } else { 0 };
    node_settings.ParentWindowId = if node.IsDockSpace() && node.HostWindow && node.Hostwindow.ParentWindow { node.Hostwindow.Parentwindow.ID} else { 0};
    node_settings.SelectedTabId = node.SelectedTabId;
    node_settings.SplitAxis = if node.IsSplitNode( { node.SplitAxis} else { ImGuiAxis_None)};
    node_settings.Depth = depth;
    node_settings.Flags = (node.LocalFlags & ImGuiDockNodeFlags_SavedFlagsMask_);
    node_settings.Pos = ImVec2ih(node.Pos);
    node_settings.Size = ImVec2ih(node.Size);
    node_settings.SizeRef = ImVec2ih(node.SizeRe0f32);
    dc.NodesSettings.push(node_settings);
    if (node.ChildNodes[0])
        DockSettingsHandler_DockNodeToSettings(dc, node.ChildNodes[0], depth + 1);
    if (node.ChildNodes[1])
        DockSettingsHandler_DockNodeToSettings(dc, node.ChildNodes[1], depth + 1);
}

pub unsafe fn DockSettingsHandler_WriteAll(g: &mut ImguiContext, SettingsHandler* handler, ImGuiTextBuffer* buf)
{
    let g =  ctx;
    dc: *mut ImGuiDockContext = &ctx.DockContext;
    if !(g.IO.ConfigFlags & ImGuiConfigFlags_DockingEnable) { return ; }

    // Gather settings data
    // (unlike our windows settings, because nodes are always built we can do a full rewrite of the SettingsNode buffer)
    dc.NodesSettings.clear();
    dc.NodesSettings.reserve(dc.Nodes.Data.Size);
    for (let n: c_int = 0; n < dc.Nodes.Data.Size; n++)
        if (node:*mut ImGuiDockNode = dc.Nodes.Data[n].val_p)
            if (node.IsRootNode())
                DockSettingsHandler_DockNodeToSettings(dc, node, 0);

    let max_depth: c_int = 0;
    for (let node_n: c_int = 0; node_n < dc.NodesSettings.Size; node_n++)
        max_depth = ImMax(dc.NodesSettings[node_n].Depth, max_depth);

    // Write to text buffer
    buf->appendf("[{}][Data]\n", handler.TypeName);
    for (let node_n: c_int = 0; node_n < dc.NodesSettings.Size; node_n++)
    {
        let line_start_pos: c_int = buf->size(); line_start_pos;
        let node_settings: *const ImGuiDockNodeSettings = &dc.NodesSettings[node_n];
        buf->appendf("%*s{}%*s", node_settings->Depth * 2, "", flag_set(node_settings.Flags, ImGuiDockNodeFlags_DockSpace) ? "DockSpace" : "DockNode ", (max_depth - node_settings->Depth) * 2, "");  // Text align nodes to facilitate looking at .ini file
        buf->appendf(" ID=0x{}", node_settings.ID);
        if (node_settings.ParentNodeId)
        {
            buf->appendf(" Parent=0x{} SizeRef={},{}", node_settings.ParentNodeId, node_settings.SizeRef.x, node_settings.SizeRef.y);
        }
        else
        {
            if (node_settings->ParentWindowId)
                buf->appendf(" Window=0x{}", node_settings->ParentWindowId);
            buf->appendf(" Pos={},{} Size={},{}", node_settings.Pos.x, node_settings.Pos.y, node_settings.Size.x, node_settings.Size.y);
        }
        if (node_settings->SplitAxis != ImGuiAxis_None)
            buf->appendf(" Split={}", (node_settings->SplitAxis == IM_GUI_AXIS_X) ? 'X' : 'Y');
        if flag_set(node_settings.Flags, ImGuiDockNodeFlags_NoResize)
            buf->appendf(" NoResize=1");
        if flag_set(node_settings.Flags, ImGuiDockNodeFlags_CentralNode)
            buf->appendf(" CentralNode=1");
        if flag_set(node_settings.Flags, ImGuiDockNodeFlags_NoTabBar)
            buf->appendf(" NoTabBar=1");
        if flag_set(node_settings.Flags, ImGuiDockNodeFlags_HiddenTabBar)
            buf->appendf(" HiddenTabBar=1");
        if flag_set(node_settings.Flags, ImGuiDockNodeFlags_NoWindowMenuButton)
            buf->appendf(" NoWindowMenuButton=1");
        if flag_set(node_settings.Flags, ImGuiDockNodeFlags_NoCloseButton)
            buf->appendf(" NoCloseButton=1");
        if (node_settings.SelectedTabId)
            buf->appendf(" Selected=0x{}", node_settings.SelectedTabId);

// #if IMGUI_DEBUG_INI_SETTINGS
        // [DEBUG] Include comments in the .ini file to ease debugging
        if (node:*mut ImGuiDockNode = DockContextFindNodeByID(ctx, node_settings.ID))
        {
            buf->appendf("%*s", ImMax(2, (line_start_pos + 92) - buf->size()), "");     // Align everything
            if (node.IsDockSpace() && node.HostWindow && node.Hostwindow.ParentWindow)
                buf->appendf(" ; in '{}'", node.Hostwindow.Parentwindow.Name);
            // Iterate settings so we can give info about windows that didn't exist during the session.
            let contains_window: c_int = 0;
            for (settings: *mut ImGuiWindowSettings = g.SettingsWindows.begin(); settings != None; settings = g.SettingsWindows.next_chunk(settings))
                if (settings.DockId == node_settings.ID)
                {
                    if (contains_window++ == 0)
                        buf->appendf(" ; contains ");
                    buf->appendf("'{}' ", settings.GetName());
                }
        }
// #endif
        buf->appendf("\n");
    }
    buf->appendf("\n");
}
