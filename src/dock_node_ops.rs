// inline * mut ImGuiDockNode   DockNodeGetRootNode( * mut ImGuiDockNode node)
pub fn DockNodeGetRootNode(node: *mut ImGuiDockNode) -> *mut ImGuiDockNode {
    while node.ParentNode { node = node.ParentNode; }
    return node;
}


// inline bool             DockNodeIsInHierarchyOf( * mut ImGuiDockNode node, *mut ImGuiDockNode parent)
pub fn DockNodeIsInHierarchy(node: *mut ImGuiDockNode, parent: *mut ImGuiDockNode) -> bool {
    while node {
        if node == parent {
            return true;
        }
        node = node.ParentNode;
    }
    return false;
}


// inline c_int              DockNodeGetDepth( * const ImGuiDockNode node)
pub fn DockNodeGetDepth(node: *const ImGuiDockNode) -> c_int {
    let depth: c_int = 0;
    while node.ParentNode {
        node = node.ParentNode;
        depth += 1;
    }
    return depth;
}


// inline ImGuiID          DockNodeGetWindowMenuButtonId( * const ImGuiDockNode node)
pub fn DockNodeGetWindowMenuButtonId(node: *const ImGuiDockNode) -> ImGuiID {
    return ImHashStr("#COLLAPSE", 0, node.ID);
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

ImGuiDockNode::~ImGuiDockNode()
{
    IM_DELETE(TabBar);
    TabBar= null_mut();
    ChildNodes[0] = ChildNodes[1]= null_mut();
}

c_int DockNodeGetTabOrder(ImGuiWindow* window)
{
    ImGuiTabBar* tab_bar = window.DockNode.TabBar;
    if (tab_bar == null_mut())
        return -1;
    ImGuiTabItem* tab = TabBarFindTabByID(tab_bar, window.TabId);
    return tab ? tab_bar->GetTabOrder(tab) : -1;
}

static c_void DockNodeHideWindowDuringHostWindowCreation(ImGuiWindow* window)
{
    window.Hidden = true;
    window.HiddenFramesCanSkipItems = window.Active ? 1 : 2;
}

static c_void DockNodeAddWindow(ImGuiDockNode* node, ImGuiWindow* window, bool add_to_tab_bar)
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

static c_void DockNodeRemoveWindow(ImGuiDockNode* node, ImGuiWindow* window, ImGuiID save_dock_id)
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
    window.Flags &= ~ImGuiWindowFlags_ChildWindow;
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
    for (ImGuiWindow* window : src_node.Windows)
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

static ImGuiWindow* DockNodeFindWindowByID(ImGuiDockNode* node, ImGuiID id)
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
        node.SetLocalFlags(node.LocalFlags & ~ImGuiDockNodeFlags_HiddenTabBar);
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

static c_void DockNodeStartMouseMovingWindow(ImGuiDockNode* node, ImGuiWindow* window)
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

static c_void DockNodeSetupHostWindow(ImGuiDockNode* node, ImGuiWindow* host_window)
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
            DockNodeGetHostWindowTitle(node, window_label, IM_ARRAYSIZE(window_label));
            ImGuiWindowFlags window_flags = ImGuiWindowFlags_NoScrollbar | ImGuiWindowFlags_NoScrollWithMouse | ImGuiWindowFlags_DockNodeHost;
            window_flags |= ImGuiWindowFlags_NoFocusOnAppearing;
            window_flags |= ImGuiWindowFlags_NoSavedSettings | ImGuiWindowFlags_NoNavFocus | ImGuiWindowFlags_NoCollapse;
            window_flags |= ImGuiWindowFlags_NoTitleBar;

            SetNextWindowBgAlpha(0f32); // Don't set ImGuiWindowFlags_NoBackground because it disables borders
            PushStyleVar(ImGuiStyleVar_WindowPadding, ImVec2(0, 0));
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
            RenderRectFilledWithHole(host_window.DrawList, node.Rect(), central_node.Rect(), GetColorU32(ImGuiCol_WindowBg), 0f32);
        else
            host_window.DrawList.AddRectFilled(node.Pos, node.Pos + node.Size, GetColorU32(ImGuiCol_WindowBg), 0f32);
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
static c_int IMGUI_CDECL TabItemComparerByDockOrder(*const c_void lhs, *const c_void rhs)
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
        SetNextWindowPos(ImVec2(node.Pos.x, node.Pos.y + GetFrameHeight()), ImGuiCond_Always, ImVec2(0f32, 0f32));
    else
        SetNextWindowPos(ImVec2(node.Pos.x + node.Size.x, node.Pos.y + GetFrameHeight()), ImGuiCond_Always, ImVec2(1f32, 0f32));
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
bool DockNodeBeginAmendTabBar(ImGuiDockNode* node)
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

static bool IsDockNodeTitleBarHighlighted(ImGuiDockNode* node, ImGuiDockNode* root_node, ImGuiWindow* host_window)
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
static c_void DockNodeUpdateTabBar(ImGuiDockNode* node, ImGuiWindow* host_window)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiStyle& style = g.Style;

    let node_was_active: bool = (node.LastFrameActive + 1 == g.FrameCount);
    let closed_all: bool = node.WantCloseAll && node_was_active;
    const let mut closed_one: ImGuiID =  node.WantCloseTabId && node_was_active;
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
    ImVec2 window_menu_button_pos;
    ImVec2 close_button_pos;
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
    let tabs_unsorted_start: c_int = tab_bar->Tabs.Size;
    for (let tab_n: c_int = tab_bar->Tabs.Size - 1; tab_n >= 0 && (tab_bar->Tabs[tab_n].Flags & ImGuiTabItemFlags_Unsorted); tab_n--)
    {
        // FIXME-DOCK: Consider only clearing the flag after the tab has been alive for a few consecutive frames, allowing late comers to not break sorting?
        tab_bar->Tabs[tab_n].Flags &= ~ImGuiTabItemFlags_Unsorted;
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
    ImVec4 backup_style_cols[ImGuiWindowDockStyleCol_COUNT];
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
            PushStyleColor(ImGuiCol_Text, style.Colors[ImGuiCol_Text] * ImVec4(1f32,1f32,1f32,0.40f32));
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
        bool held;
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

static bool DockNodeIsDropAllowed(ImGuiWindow* host_window, ImGuiWindow* root_payload)
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
static c_void DockNodeCalcTabBarLayout(*const ImGuiDockNode node, ImRect* out_title_rect, ImRect* out_tab_bar_rect, ImVec2* out_window_menu_button_pos, ImVec2* out_close_button_pos)
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
        if (out_close_button_pos) *out_close_button_pos = ImVec2(r.Max.x - style.FramePadding.x, r.Min.y);
    }
    if (node.HasWindowMenuButton && style.WindowMenuButtonPosition == ImGuiDir_Left)
    {
        r.Min.x += button_sz + style.ItemInnerSpacing.x;
    }
    else if (node.HasWindowMenuButton && style.WindowMenuButtonPosition == ImGuiDir_Right)
    {
        r.Max.x -= button_sz + style.FramePadding.x;
        window_menu_button_pos = ImVec2(r.Max.x, r.Min.y);
    }
    if (out_tab_bar_rect) { *out_tab_bar_rect = r; }
    if (out_window_menu_button_pos) { *out_window_menu_button_pos = window_menu_button_pos; }
}

c_void DockNodeCalcSplitRects(ImVec2& pos_old, ImVec2& size_old, ImVec2& pos_new, ImVec2& size_new, ImGuiDir dir, ImVec2 size_new_desired)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let dock_spacing: c_float =  g.Style.ItemInnerSpacing.x;
    const ImGuiAxis axis = (dir == ImGuiDir_Left || dir == ImGuiDir_Right) ? ImGuiAxis_X : ImGuiAxis_Y;
    pos_new[axis ^ 1] = pos_old[axis ^ 1];
    size_new[axis ^ 1] = size_old[axis ^ 1];

    // Distribute size on given axis (with a desired size or equally)
    let w_avail: c_float =  size_old[axis] - dock_spacing;
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
bool DockNodeCalcDropRectsAndTestMousePos(const ImRect& parent, ImGuiDir dir, ImRect& out_r, bool outer_docking, ImVec2* test_mouse_pos)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    let parent_smaller_axis: c_float =  ImMin(parent.GetWidth(), parent.GetHeight());
    let hs_for_central_nodes: c_float =  ImMin(g.FontSize * 1.5f32, ImMax(g.FontSize * 0.5f32, parent_smaller_axis / 8.00f32));
    let mut hs_w: c_float = 0f32; // Half-size, longer axis
    let mut hs_h: c_float = 0f32; // Half-size, smaller axis
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
static c_void DockNodePreviewDockSetup(ImGuiWindow* host_window, ImGuiDockNode* host_node, ImGuiWindow* payload_window, ImGuiDockNode* payload_node, ImGuiDockPreviewData* data, bool is_explicit_target, bool is_outer_docking)
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
        let split_ratio: c_float =  ImSaturate(size_new[split_axis] / data.FutureNode.Size[split_axis]);
        data.FutureNode.Pos = pos_new;
        data.FutureNode.Size = size_new;
        data.SplitRatio = (split_dir == ImGuiDir_Right || split_dir == ImGuiDir_Down) ? (1f32 - split_ratio) : (split_ratio);
    }
}

static c_void DockNodePreviewDockRender(ImGuiWindow* host_window, ImGuiDockNode* host_node, ImGuiWindow* root_payload, *const ImGuiDockPreviewData data)
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
    const u32 overlay_col_main = GetColorU32(ImGuiCol_DockingPreview, is_transparent_payload ? 0.60f32 : 0.400f32);
    const u32 overlay_col_drop = GetColorU32(ImGuiCol_DockingPreview, is_transparent_payload ? 0.90f32 : 0.700f32);
    const u32 overlay_col_drop_hovered = GetColorU32(ImGuiCol_DockingPreview, is_transparent_payload ? 1.20f32 : 1.000f32);
    const u32 overlay_col_lines = GetColorU32(ImGuiCol_NavWindowingHighlight, is_transparent_payload ? 0.80f32 : 0.600f32);

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
            const u32 overlay_col_text = GetColorU32(payload_window.DockStyle.Colors[ImGuiWindowDockStyleCol_Text]);
            const u32 overlay_col_tabs = GetColorU32(payload_window.DockStyle.Colors[ImGuiWindowDockStyleCol_TabActive]);
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
    let overlay_rounding: c_float =  ImMax(3.0f32, g.Style.FrameRounding);
    for (let dir: c_int = ImGuiDir_None; dir < ImGuiDir_COUNT; dir++)
    {
        if (!data.DropRectsDraw[dir + 1].IsInverted())
        {
            let draw_r: ImRect =  data.DropRectsDraw[dir + 1];
            let draw_r_in: ImRect =  draw_r;
            draw_r_in.Expand(-2.00f32);
            u32 overlay_col = (data.SplitDir == (ImGuiDir)dir && data.IsSplitDirExplicit) ? overlay_col_drop_hovered : overlay_col_drop;
            for (let overlay_n: c_int = 0; overlay_n < overlay_draw_lists_count; overlay_n++)
            {
                let center: ImVec2 = ImFloor(draw_r_in.GetCenter());
                overlay_draw_lists[overlay_n]->AddRectFilled(draw_r.Min, draw_r.Max, overlay_col, overlay_rounding);
                overlay_draw_lists[overlay_n]->AddRect(draw_r_in.Min, draw_r_in.Max, overlay_col_lines, overlay_rounding);
                if (dir == ImGuiDir_Left || dir == ImGuiDir_Right)
                    overlay_draw_lists[overlay_n]->AddLine(ImVec2(center.x, draw_r_in.Min.y), ImVec2(center.x, draw_r_in.Max.y), overlay_col_lines);
                if (dir == ImGuiDir_Up || dir == ImGuiDir_Down)
                    overlay_draw_lists[overlay_n]->AddLine(ImVec2(draw_r_in.Min.x, center.y), ImVec2(draw_r_in.Max.x, center.y), overlay_col_lines);
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

c_void DockNodeTreeSplit(ImGuiContext* ctx, ImGuiDockNode* parent_node, ImGuiAxis split_axis, c_int split_inheritor_child_idx, c_float split_ratio, ImGuiDockNode* new_node)
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
    // IM_ASSERT(size_avail > 0f32); // If you created a node manually with DockBuilderAddNode(), you need to also call DockBuilderSetNodeSize() before splitting.
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
    parent_node.LocalFlags &= ~ImGuiDockNodeFlags_LocalFlagsTransferMask_;
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
    parent_node.LocalFlags &= ~ImGuiDockNodeFlags_LocalFlagsTransferMask_; // Preserve Dockspace flag
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
c_void DockNodeTreeUpdatePosSize(ImGuiDockNode* node, ImVec2 pos, ImVec2 size, ImGuiDockNode* only_write_to_single_node)
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
        let size_avail: c_float =  ImMax(size[axis] - spacing, 0f32);

        // Size allocation policy
        // 1) The first 0..WindowMinSize[axis]*2 are allocated evenly to both windows.
        let size_min_each: c_float =  ImFloor(ImMin(size_avail, g.Style.WindowMinSize[axis] * 2.00f32) * 0.5f32);

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
            let split_ratio: c_float =  child_0_size[axis] / (child_0_size[axis] + child_1_size[axis]);
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

static c_void DockNodeTreeUpdateSplitterFindTouchingNode(ImGuiDockNode* node, ImGuiAxis axis, c_int side, Vec<ImGuiDockNode*>* touching_nodes)
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
            let min_size: c_float =  g.Style.WindowMinSize[axis];
            c_float resize_limits[2];
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
                ImDrawList* draw_list = node->HostWindow ? GetForegroundDrawList(node->HostWindow) : GetForegroundDrawList(GetMainViewport());
                for (int n = 0; n < 2; n++)
                {
                    for (int touching_node_n = 0; touching_node_n < touching_nodes[n].Size; touching_node_n++)
                        draw_list.AddRect(touching_nodes[n][touching_node_n]->Pos, touching_nodes[n][touching_node_n]->Pos + touching_nodes[n][touching_node_n]->Size, IM_COL32(0, 255, 0, 255));
                    if (axis == ImGuiAxis_X)
                        draw_list.AddLine(ImVec2(resize_limits[n], node->ChildNodes[n]->Pos.y), ImVec2(resize_limits[n], node->ChildNodes[n]->Pos.y + node->ChildNodes[n]->Size.y), IM_COL32(255, 0, 255, 255), 3.00f32);
                    else
                        draw_list.AddLine(ImVec2(node->ChildNodes[n]->Pos.x, resize_limits[n]), ImVec2(node->ChildNodes[n]->Pos.x + node->ChildNodes[n]->Size.x, resize_limits[n]), IM_COL32(255, 0, 255, 255), 3.00f32);
                }
                */
            }

            // Use a short delay before highlighting the splitter (and changing the mouse cursor) in order for regular mouse movement to not highlight many splitters
            let cur_size_0: c_float =  child_0->Size[axis];
            let cur_size_1: c_float =  child_1->Size[axis];
            let min_size_0: c_float =  resize_limits[0] - child_0->Pos[axis];
            let min_size_1: c_float =  child_1->Pos[axis] + child_1->Size[axis] - resize_limits[1];
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
                    for (let side_n: c_int = 0; side_n < 2; side_n++)
                        for (let touching_node_n: c_int = 0; touching_node_n < touching_nodes[side_n].Size; touching_node_n++)
                        {
                            ImGuiDockNode* touching_node = touching_nodes[side_n][touching_node_n];
                            //ImDrawList* draw_list = node->HostWindow ? GetForegroundDrawList(node->HostWindow) : GetForegroundDrawList(GetMainViewport());
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

ImGuiDockNode* DockNodeTreeFindVisibleNodeByPos(ImGuiDockNode* node, ImVec2 pos)
{
    if (!node.IsVisible)
        return null_mut();

    let dock_spacing: c_float =  0f32;// g.Style.ItemInnerSpacing.x; // FIXME: Relation to DOCKING_SPLITTER_SIZE?
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
