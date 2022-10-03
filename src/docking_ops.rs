
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
// Docking: Public Functions (SetWindowDock, DockSpace, DockSpaceOverViewport)
//-----------------------------------------------------------------------------
// - SetWindowDock() [Internal]
// - DockSpace()
// - DockSpaceOverViewport()
//-----------------------------------------------------------------------------

// [Internal] Called via SetNextWindowDockID()
c_void SetWindowDock(window: *mut ImGuiWindow, dock_id: ImGuiID, ImGuiCond cond)
{
    // Test condition (NB: bit 0 is always true) and clear flags for next time
    if (cond && (window.SetWindowDockAllowFlags & cond) == 0)
        return;
    window.SetWindowDockAllowFlags &= ~(ImGuiCond_Once | ImGuiCond_FirstUseEver | ImGuiCond_Appearing);

    if (window.DockId == dock_id)
        return;

    // If the user attempt to set a dock id that is a split node, we'll dig within to find a suitable docking spot
    ctx: *mut ImGuiContext = GImGui;
    if (let mut new_node: *mut ImGuiDockNode = DockContextFindNodeByID(ctx, dock_id))
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
ImGuiID DockSpace(id: ImGuiID, const ImVec2& size_arg, ImGuiDockNodeFlags flags, *const ImGuiWindowClass window_class)
{
    ctx: *mut ImGuiContext = GImGui;
    let g = ctx;
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
    node: *mut ImGuiDockNode = DockContextFindNodeByID(ctx, id);
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
    if (node.LastFrameActive == g.FrameCount && !(flags & ImGuiDockNodeFlags_KeepAliveOnly))
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
    if (size.x <= 0f32)
        size.x = ImMax(content_avail.x + size.x, 4.00f32); // Arbitrary minimum child size (0f32 causing too much issues)
    if (size.y <= 0f32)
        size.y = ImMax(content_avail.y + size.y, 4.00f32);
    // IM_ASSERT(size.x > 0f32 && size.y > 0f32);

    node.Pos = window.DC.CursorPos;
    node.Size = node.SizeRef = size;
    SetNextWindowPos(node.Pos);
    SetNextWindowSize(node.Size);
    g.NextWindowData.PosUndock = false;

    // FIXME-DOCK: Why do we need a child window to host a dockspace, could we host it in the existing window?
    // FIXME-DOCK: What is the reason for not simply calling BeginChild()? (OK to have a reason but should be commented)
    let mut window_flags: ImGuiWindowFlags = ImGuiWindowFlags_ChildWindow | ImGuiWindowFlags_DockNodeHost;
    window_flags |= ImGuiWindowFlags_NoSavedSettings | ImGuiWindowFlags_NoResize | ImGuiWindowFlags_NoCollapse | ImGuiWindowFlags_NoTitleBar;
    window_flags |= ImGuiWindowFlags_NoScrollbar | ImGuiWindowFlags_NoScrollWithMouse;
    window_flags |= ImGuiWindowFlags_NoBackground;

    title: [c_char;256];
    ImFormatString(title, IM_ARRAYSIZE(title), "%s/DockSpace_%08X", window.Name, id);

    PushStyleVar(ImGuiStyleVar_ChildBorderSize, 0f32);
    Begin(title, null_mut(), window_flags);
    PopStyleVar();

    let mut host_window: *mut ImGuiWindow =  g.CurrentWindow;
    DockNodeSetupHostWindow(node, host_window);
    host_window.ChildId = window.GetID(title);
    node.OnlyNodeWithWindows= null_mut();

    // IM_ASSERT(node.IsRootNode());

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

    let mut host_window_flags: ImGuiWindowFlags = 0;
    host_window_flags |= ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoCollapse | ImGuiWindowFlags_NoResize | ImGuiWindowFlags_NoMove | ImGuiWindowFlags_NoDocking;
    host_window_flags |= ImGuiWindowFlags_NoBringToFrontOnFocus | ImGuiWindowFlags_NoNavFocus;
    if (dockspace_flags & ImGuiDockNodeFlags_PassthruCentralNode)
        host_window_flags |= ImGuiWindowFlags_NoBackground;

    label: [c_char;32];
    ImFormatString(label, IM_ARRAYSIZE(label), "DockSpaceViewport_%08X", viewport.ID);

    PushStyleVar(ImGuiStyleVar_WindowRounding, 0f32);
    PushStyleVar(ImGuiStyleVar_WindowBorderSize, 0f32);
    PushStyleVar(ImGuiStyleVar_WindowPadding, ImVec2::new2(0f32, 0f32));
    Begin(label, null_mut(), host_window_flags);
    PopStyleVar(3);

    let mut dockspace_id: ImGuiID =  GetID("DockSpace");
    DockSpace(dockspace_id, ImVec2::new2(0f32, 0f32), dockspace_flags, window_class);
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

c_void DockBuilderDockWindow(*const char window_name, node_id: ImGuiID)
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
        ImGuiWindowSettings* settings = FindWindowSettings(window_id);
        if (settings == null_mut())
            settings = CreateNewWindowSettings(window_name);
        settings.DockId = node_id;
        settings.DockOrder = -1;
    }
}

ImGuiDockNode* DockBuilderGetNode(node_id: ImGuiID)
{
    ctx: *mut ImGuiContext = GImGui;
    return DockContextFindNodeByID(ctx, node_id);
}

c_void DockBuilderSetNodePos(node_id: ImGuiID, ImVec2 pos)
{
    ctx: *mut ImGuiContext = GImGui;
    node: *mut ImGuiDockNode = DockContextFindNodeByID(ctx, node_id);
    if (node == null_mut())
        return;
    node.Pos = pos;
    node.AuthorityForPos = ImGuiDataAuthority_DockNode;
}

c_void DockBuilderSetNodeSize(node_id: ImGuiID, ImVec2 size)
{
    ctx: *mut ImGuiContext = GImGui;
    node: *mut ImGuiDockNode = DockContextFindNodeByID(ctx, node_id);
    if (node == null_mut())
        return;
    // IM_ASSERT(size.x > 0f32 && size.y > 0f32);
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
    ctx: *mut ImGuiContext = GImGui;

    if (id != 0)
        DockBuilderRemoveNode(id);

    node: *mut ImGuiDockNode= null_mut();
    if (flags & ImGuiDockNodeFlags_DockSpace)
    {
        DockSpace(id, ImVec2::new2(0, 0), (flags & ~ImGuiDockNodeFlags_DockSpace) | ImGuiDockNodeFlags_KeepAliveOnly);
        node = DockContextFindNodeByID(ctx, id);
    }
    else
    {
        node = DockContextAddNode(ctx, id);
        node.SetLocalFlags(flags);
    }
    node.LastFrameAlive = ctx.FrameCount;   // Set this otherwise BeginDocked will undock during the same frame.
    return node.ID;
}

c_void DockBuilderRemoveNode(node_id: ImGuiID)
{
    ctx: *mut ImGuiContext = GImGui;
    node: *mut ImGuiDockNode = DockContextFindNodeByID(ctx, node_id);
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
c_void DockBuilderRemoveNodeChildNodes(root_id: ImGuiID)
{
    ctx: *mut ImGuiContext = GImGui;
    let dc = &mut ctx.DockContext;

    let mut root_node: *mut ImGuiDockNode = root_id ? DockContextFindNodeByID(ctx, root_id) : null_mut();
    if (root_id && root_node == null_mut())
        return;
    let mut has_central_node: bool =  false;

    ImGuiDataAuthority backup_root_node_authority_for_pos = root_node ? root_node.AuthorityForPos : ImGuiDataAuthority_Auto;
    ImGuiDataAuthority backup_root_node_authority_for_size = root_node ? root_node.AuthorityForSize : ImGuiDataAuthority_Auto;

    // Process active windows
    Vec<ImGuiDockNode*> nodes_to_remove;
    for (let n: c_int = 0; n < dc.Nodes.Data.Size; n++)
        if (node: *mut ImGuiDockNode = (ImGuiDockNode*)dc.Nodes.Data[n].val_p)
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
    for (ImGuiWindowSettings* settings = ctx.SettingsWindows.begin(); settings != null_mut(); settings = ctx.SettingsWindows.next_chunk(settings))
        if (let mut window_settings_dock_id: ImGuiID =  settings.DockId)
            for (let n: c_int = 0; n < nodes_to_remove.Size; n++)
                if (nodes_to_remove[n].ID == window_settings_dock_id)
                {
                    settings.DockId = root_id;
                    break;
                }

    // Not really efficient, but easier to destroy a whole hierarchy considering DockContextRemoveNode is attempting to merge nodes
    if (nodes_to_remove.Size > 1)
        ImQsort(nodes_to_remove.Data, nodes_to_remove.Size, sizeof(ImGuiDockNode*), DockNodeComparerDepthMostFirst);
    for (let n: c_int = 0; n < nodes_to_remove.Size; n++)
        DockContextRemoveNode(ctx, nodes_to_remove[n], false);

    if (root_id == 0)
    {
        dc.Nodes.Clear();
        dc.Requests.clear();
    }
    else if (has_central_node)
    {
        root_node.CentralNode = root_node;
        root_node.SetLocalFlags(root_node.LocalFlags | ImGuiDockNodeFlags_CentralNode);
    }
}

c_void DockBuilderRemoveNodeDockedWindows(root_id: ImGuiID, clear_settings_refs: bool)
{
    // Clear references in settings
    ctx: *mut ImGuiContext = GImGui;
    let g = ctx;
    if (clear_settings_refs)
    {
        for (ImGuiWindowSettings* settings = g.SettingsWindows.begin(); settings != null_mut(); settings = g.SettingsWindows.next_chunk(settings))
        {
            let mut want_removal: bool =  (root_id == 0) || (settings.DockId == root_id);
            if (!want_removal && settings.DockId != 0)
                if (node: *mut ImGuiDockNode = DockContextFindNodeByID(ctx, settings.DockId))
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
ImGuiID DockBuilderSplitNode(id: ImGuiID, split_dir: ImGuiDir, c_float size_ratio_for_node_at_dir, ImGuiID* out_id_at_dir, ImGuiID* out_id_at_opposite_dir)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(split_dir != ImGuiDir_None);
    IMGUI_DEBUG_LOG_DOCKING("[docking] DockBuilderSplitNode: node 0x%08X, split_dir %d\n", id, split_dir);

    node: *mut ImGuiDockNode = DockContextFindNodeByID(&g, id);
    if (node == null_mut())
    {
        // IM_ASSERT(node != NULL);
        return 0;
    }

    // IM_ASSERT(!node.IsSplitNode()); // Assert if already Split

    ImGuiDockRequest req;
    req.Type = ImGuiDockRequestType_Split;
    req.DockTargetWindow= null_mut();
    req.DockTargetNode = node;
    req.DockPayload= null_mut();
    req.DockSplitDir = split_dir;
    req.DockSplitRatio = ImSaturate((split_dir == ImGuiDir_Left || split_dir == ImGuiDir_Up) ? size_ratio_for_node_at_dir : 1f32 - size_ratio_for_node_at_dir);
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

static ImGuiDockNode* DockBuilderCopyNodeRec(src_node: *mut ImGuiDockNode, dst_node_id_if_known: ImGuiID, Vec<ImGuiID>* out_node_remap_pairs)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut dst_node: *mut ImGuiDockNode = DockContextAddNode(&g, dst_node_id_if_known);
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

c_void DockBuilderCopyNode(src_node_id: ImGuiID, dst_node_id: ImGuiID, Vec<ImGuiID>* out_node_remap_pairs)
{
    ctx: *mut ImGuiContext = GImGui;
    // IM_ASSERT(src_node_id != 0);
    // IM_ASSERT(dst_node_id != 0);
    // IM_ASSERT(out_node_remap_pairs != NULL);

    DockBuilderRemoveNode(dst_node_id);

    let mut src_node: *mut ImGuiDockNode = DockContextFindNodeByID(ctx, src_node_id);
    // IM_ASSERT(src_node != NULL);

    out_node_remap_pairs.clear();
    DockBuilderCopyNodeRec(src_node, dst_node_id, out_node_remap_pairs);

    // IM_ASSERT((out_node_remap_pairs.Size % 2) == 0);
}

c_void DockBuilderCopyWindowSettings(*const char src_name, *const char dst_name)
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
    else if (ImGuiWindowSettings* dst_settings = FindOrCreateWindowSettings(dst_name))
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
        dst_settings.Collapsed = src_window.Collapsed;
    }
}

// FIXME: Will probably want to change this signature, in particular how the window remapping pairs are passed.
c_void DockBuilderCopyDockSpace(src_dockspace_id: ImGuiID, dst_dockspace_id: ImGuiID, Vec<*const char>* in_window_remap_pairs)
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
        else if (ImGuiWindowSettings* src_window_settings = FindWindowSettings(src_window_id))
            src_dock_id = src_window_settings.DockId;
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
            node: *mut ImGuiDockNode = DockBuilderGetNode(src_dock_id);
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
c_void DockBuilderFinish(root_id: ImGuiID)
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

bool GetWindowAlwaysWantOwnTabBar(window: *mut ImGuiWindow)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.IO.ConfigDockingAlwaysTabBar || window.WindowClass.DockingAlwaysTabBar)
        if ((window.Flags & (ImGuiWindowFlags_ChildWindow | ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoDocking)) == 0)
            if (!window.IsFallbackWindow)    // We don't support AlwaysTabBar on the fallback/implicit window to avoid unused dock-node overhead/noise
                return true;
    return false;
}

static ImGuiDockNode* DockContextBindNodeToWindow(ctx: *mut ImGuiContext, window: *mut ImGuiWindow)
{
    let g = ctx;
    node: *mut ImGuiDockNode = DockContextFindNodeByID(ctx, window.DockId);
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
        let mut ancestor_node: *mut ImGuiDockNode = node;
        while (!ancestor_node.IsVisible && ancestor_node.ParentNode)
            ancestor_node = ancestor_node.ParentNode;
        // IM_ASSERT(ancestor_node.Size.x > 0f32 && ancestor_node.Size.y > 0f32);
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

c_void BeginDocked(window: *mut ImGuiWindow, bool* p_open)
{
    ctx: *mut ImGuiContext = GImGui;
    let g = ctx;

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
    node: *mut ImGuiDockNode = window.DockNode;
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
        let mut root_node: *mut ImGuiDockNode = DockNodeGetRootNode(node);
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
    // FIXME-DOCK: replace .HostWindow NULL compare with something more explicit (~was initially intended as a first frame test)
    if (node.HostWindow == null_mut())
    {
        if (node.State == ImGuiDockNodeState_HostWindowHiddenBecauseWindowsAreResizing)
            window.DockIsActive = true;
        if (node.Windows.len() > 1)
            DockNodeHideWindowDuringHostWindowCreation(window);
        return;
    }

    // We can have zero-sized nodes (e.g. children of a small-size dockspace)
    // IM_ASSERT(node.HostWindow);
    // IM_ASSERT(node.IsLeafNode());
    // IM_ASSERT(node.Size.x >= 0f32 && node.Size.y >= 0f32);
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
        window.Flags &= ~ImGuiWindowFlags_NoTitleBar;      // Clear the NoTitleBar flag in case the user set it: confusingly enough we need a title bar height so we are correctly offset, but it won't be displayed!

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
    ctx: *mut ImGuiContext = GImGui;
    let g = ctx;

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
    if (!payload.IsDataType(IMGUI_PAYLOAD_TYPE_WINDOW) || !DockNodeIsDropAllowed(window, *(ImGuiWindow**)payload.Data))
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
        node: *mut ImGuiDockNode= null_mut();
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

        const let explicit_target_rect: ImRect =  (node && node.TabBar && !node.IsHiddenTabBar() && !node.IsNoTabBar()) ? node.TabBar.BarRect : ImRect(window.Pos, window.Pos + ImVec2(window.Size.x, GetFrameHeight()));
        let is_explicit_target: bool = g.IO.ConfigDockingWithShift || IsMouseHoveringRect(explicit_target_rect.Min, explicit_target_rect.Max);

        // Preview docking request and find out split direction/ratio
        //let do_preview: bool = true;     // Ignore testing for payload.IsPreview() which removes one frame of delay, but breaks overlapping drop targets within the same window.
        let do_preview: bool = payload.IsPreview() || payload.IsDelivery();
        if (do_preview && (node != null_mut() || dock_into_floating_window))
        {
            let mut split_inner = ImGuiDockPrevewData::default();
            let mut split_outer = ImGuiDockPrevewData::default();
            ImGuiDockPreviewData* split_data = &split_inner;
            if (node && (node.ParentNode || node.IsCentralNode()))
                if (let mut root_node: *mut ImGuiDockNode = DockNodeGetRootNode(node))
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
            if (split_Data.IsDropAllowed && payload.IsDelivery())
                DockContextQueueDock(ctx, window, split_Data.SplitNode, payload_window, split_Data.SplitDir, split_Data.SplitRatio, split_data == &split_outer);
        }
    }
    EndDragDropTarget();
}
