use crate::{a_imgui_cpp::{DockSettingsHandler_ApplyAll, DockSettingsHandler_ClearAll, DockSettingsHandler_ReadLine, DockSettingsHandler_WriteAll}, direction::{ImGuiDir_Left, ImGuiDir_None, ImGuiDir_Right, ImGuiDir_Up}, utils::is_not_null};
use crate::core::context::ImguiContext;
use crate::core::axis::{IM_GUI_AXIS_X, IM_GUI_AXIS_Y};
use crate::core::config_flags::ImGuiConfigFlags_DockingEnable;
use crate::docking::dock_node::ImGuiDockNode;
use crate::docking::dock_request_type::{ImGuiDockRequestType_Dock, ImGuiDockRequestType_None, ImGuiDockRequestType_Undock};
use crate::docking_ops::{DockBuilderRemoveNodeChildNodes, DockBuilderRemoveNodeDockedWindows, DockSettingsHandler_ApplyAll, DockSettingsHandler_ClearAll, DockSettingsHandler_ReadLine, DockSettingsHandler_WriteAll};
use crate::hash_ops::hash_string;
use crate::settings_handler::SettingsHandler;
use crate::type_defs::ImguiHandle;


pub fn init_dock_context(g: &mut ImguiContext)
{
    // Add .ini handle for persistent docking data
    let mut ini_handler: SettingsHandler = Default::default();
    ini_handler.TypeName = String::from("Docking");
    ini_handler.TypeHash = hash_string(&String::from("Docking"), 0);
    ini_handler.ClearAllFn = DockSettingsHandler_ClearAll;
    ini_handler.ReadInitFn = DockSettingsHandler_ClearAll; // Also clear on read
    ini_handler.ReadOpenFn = DockSettingsHandler_ReadOpen;
    ini_handler.ReadLineFn = DockSettingsHandler_ReadLine;
    ini_handler.ApplyAllFn = DockSettingsHandler_ApplyAll;
    ini_handler.WriteAllFn = DockSettingsHandler_WriteAll;
    g.settings_handlers.push(ini_handler);
}

pub fn shutdown_dock_context(g: &mut ImguiContext)
{
    let dc  = &g.dock_context;
    // for (let n: c_int = 0; n < dc.Nodes.Data.Size; n++)
    for n in 0 .. dc.dock_nodes.len()
    {
        let node = dc.dock_nodes[n].val_p;
        if is_not_null(node) {
            IM_DELETE(node);
        }
    }
}

pub fn clear_dock_context_nodes(g: &mut ImguiContext,
                                root_id: ImguiHandle,
                                clear_settings_refs: bool)
{
    // IM_ASSERT(ctx == GImGui);
    DockBuilderRemoveNodeDockedWindows(g, root_id, clear_settings_refs);
    DockBuilderRemoveNodeChildNodes(g, root_id);
}

// [DEBUG] This function also acts as a defacto test to make sure we can rebuild from scratch without a glitch
// (Different from DockSettingsHandler_ClearAll() + DockSettingsHandler_ApplyAll() because this reuses current settings!)
pub unsafe fn DockContextRebuildNodes(g: &mut ImguiContext)
{
    let g =  ctx;
    let dc: *mut ImGuiDockContext = &ctx.DockContext;
    IMGUI_DEBUG_LOG_DOCKING("[docking] DockContextRebuildNodes\n");
    SaveIniSettingsToMemory();
    let mut root_id: ImguiHandle =  0; // Rebuild all
    clear_dock_context_nodes(ctx, root_id, false);
    DockContextBuildNodesFromSettings(ctx, dc.NodesSettings.Data, dc.NodesSettings.Size);
    DockContextBuildAddWindowsToNodes(ctx, root_id);
}

// Docking context update function, called by NewFrame()
pub unsafe fn DockContextNewFrameUpdateUndocking(g: &mut ImguiContext)
{
    let g: *mut ImGuiContext =  ctx;
    let dc: *mut ImGuiDockContext = &mut ctx.DockContext;
    if (!(g.IO.ConfigFlags & ImGuiConfigFlags_DockingEnable))
    {
        if (dc.Nodes.Data.Size > 0 || dc.Requests.Size > 0){
            clear_dock_context_nodes(ctx, 0, true);}
        return;
    }

    // Setting NoSplit at runtime merges all nodes
    if (g.IO.ConfigDockingNoSplit){
        // for (let n: c_int = 0; n < dc.Nodes.Data.Size; n++)
        for n in 0 .. dc.Nodes.len()
        {
            let node = &mut dc.Nodes[n].val_p;
            if (is_not_null(node)){
                if (node.IsRootNode() && node.IsSplitNode())
                {
                    DockBuilderRemoveNodeChildNodes(g, node.ID);
                    //dc.WantFullRebuild = true;
                }}}}

    // Process full rebuild
// #if 0
    if (IsKeyPressed(GetKeyIndex(ImGuiKey_C))){
        dc.WantFullRebuild = true;}
// #endif
    if (dc.WantFullRebuild)
    {
        DockContextRebuildNodes(ctx);
        dc.WantFullRebuild = false;
    }

    // Process Undocking requests (we need to process them _before_ the UpdateMouseMovingWindowNewFrame call in NewFrame)
    // for (let n: c_int = 0; n < dc.Requests.Size; n++)
    for n in 0 .. dc.Requests.len()
    {
        let req: *mut ImGuiDockRequest = &dc.Requests[n];
        if (req.Type == ImGuiDockRequestType_Undock && req.UndockTargetWindow){
            DockContextProcessUndockWindow(ctx, req.UndockTargetWindow, false);}
        else if (req.Type == ImGuiDockRequestType_Undock && req.UndockTargetNode){
            DockContextProcessUndockNode(ctx, req.UndockTargetNode);}
    }
}

// Docking context update function, called by NewFrame()
pub unsafe fn DockContextNewFrameUpdateDocking(g: &mut ImguiContext)
{
    let g =  ctx;
    let dc: *mut ImGuiDockContext  = &mut ctx.DockContext;
    if (!(g.IO.ConfigFlags & ImGuiConfigFlags_DockingEnable)){
        return;}

    // [DEBUG] Store hovered dock node.
    // We could in theory use DockNodeTreeFindVisibleNodeByPos() on the root host dock node, but using .DockNode is a good shortcut.
    // Note this is mostly a debug thing and isn't actually used for docking target, because docking involve more detailed filtering.
    g.DebugHoveredDockNode= None;
    let mut hovered_window: &mut ImGuiWindow =  g.HoveredWindowUnderMovingWindow;
    if (is_not_null(hovered_window))
    {
        if (hovered_window.DockNodeAsHost){
            g.DebugHoveredDockNode = DockNodeTreeFindVisibleNodeByPos(hovered_window.DockNodeAsHost, g.IO.MousePos);}
        else if (hovered_window.Rootwindow.DockNode){
            g.DebugHoveredDockNode = hovered_window.Rootwindow.DockNode;}
    }

    // Process Docking requests
    // for (let n: c_int = 0; n < dc.Requests.Size; n++)
    for n in 0 .. dc.Requests.len()
    {
        if (dc.Requests[n].Type == ImGuiDockRequestType_Dock){
            DockContextProcessDock(ctx, &dc.Requests[n]);}}
    dc.Requests.clear();

    // Create windows for each automatic docking nodes
    // We can have NULL pointers when we delete nodes, but because ID are recycled this should amortize nicely (and our node count will never be very high)
    // for (let n: c_int = 0; n < dc.Nodes.Data.Size; n++)
    for n in 0 .. dc.Nodes.len()
    {
    let node = dc.Nodes.Data[n].val_p;
        if (is_not_null(node)){
            if (node.IsFloatingNode()){
                DockNodeUpdate(node);}}}
}

pub unsafe fn DockContextEndFrame(g: &mut ImguiContext)
{
    // Draw backgrounds of node missing their window
    let g =  ctx;
    let dc = &mut g.DockContext;
    // for (let n: c_int = 0; n < dc.Nodes.Data.Size; n++)
    for n in 0 .. dc.Nodes.len()
    {
        let node: *mut ImGuiDockNode = dc.Nodes.Data[n].val_p;
        if (is_not_null(node))
        {
            if (node.LastFrameActive == g.FrameCount && node.IsVisible && node.HostWindow && node.IsLeafNode() && !node.IsBgDrawnThisFrame)
            {
                let mut bg_rect: ImRect = ImRect::new(node.Pos + ImVec2::new(0.0, GetFrameHeight()), node.Pos + node.Size);
                let bg_rounding_flags: ImDrawFlags = CalcRoundingFlagsForRectInRect(bg_rect, node.Hostwindow.Rect(), DOCKING_SPLITTER_SIZE);
                node.Hostwindow.DrawList.ChannelsSetCurrent(DOCKING_HOST_DRAW_CHANNEL_BG);
                node.Hostwindow.DrawList.AddRectFilled(bg_rect.Min, bg_rect.Max, node.LastBgColor, node.Hostwindow.WindowRounding, bg_rounding_flags);
            }}}
}

pub unsafe fn DockContextFindNodeByID(g: &mut ImguiContext, id: ImguiHandle) -> Option<&mut ImGuiDockNode>
{
    // return ctx.DockContext.Nodes.GetVoidPtr(id);
    g.dock_context.dock_nodes.get_mut(&id)
}

pub unsafe fn DockContextGenNodeID(g: &mut ImguiContext) -> ImguiHandle
{
    // Generate an ID for new node (the exact ID value doesn't matter as long as it is not already used)
    // FIXME-OPT FIXME-DOCK: This is suboptimal, even if the node count is small enough not to be a worry.0
    // We should poke in ctx.Nodes to find a suitable ID faster. Even more so trivial that ctx.Nodes lookup is already sorted.
    let mut id: ImguiHandle =  0x0001;
    while (DockContextFindNodeByID(ctx, id) != null_mut()){
        id+= 1;}
    return id;
}

pub unsafe fn DockContextAddNode(g: &mut ImguiContext, id: ImguiHandle) -> *mut ImGuiDockNode
{
    // Generate an ID for the new node (the exact ID value doesn't matter as long as it is not already used) and add the first window.
    let g =  ctx;
    if (id == 0){
        id = DockContextGenNodeID(ctx);}
    else {}
        // IM_ASSERT(DockContextFindNodeByID(ctx, id) == NULL);

    // We don't set node.LastFrameAlive on construction. Nodes are always created at all time to reflect .ini settings!
    IMGUI_DEBUG_LOG_DOCKING("[docking] DockContextAddNode 0x{}\n", id);
    // node:*mut ImGuiDockNode = IM_NEW(ImGuiDockNode)(id);
    let mut node: ImGuiDockNode = ImGuiDockNode::new(id);
    ctx.DockContext.Nodes.SetVoidPtr(node.ID, node);
    return node;
}

pub unsafe fn DockContextRemoveNode(
    g: &mut ImguiContext,
    node: *mut ImGuiDockNode,
    merge_sibling_into_parent_node: bool)
{
    let g =  ctx;
    let dc  = &mut ctx.DockContext;

    // IMGUI_DEBUG_LOG_DOCKING("[docking] DockContextRemoveNode 0x{}\n", node.ID);
    // IM_ASSERT(DockContextFindNodeByID(ctx, node.ID) == node);
    // IM_ASSERT(node.ChildNodes[0] == NULL && node.ChildNodes[1] == NULL);
    // IM_ASSERT(node.Windows.Size == 0);

    if (node.HostWindow){
        node.Hostwindow.DockNodeAsHost= None;}

    let parent_node:*mut ImGuiDockNode = node.ParentNode;
    let merge: bool = (merge_sibling_into_parent_node && parent_node != null_mut());
    if (merge)
    {
        // IM_ASSERT(parent_node.ChildNodes[0] == node || parent_node.ChildNodes[1] == node);
        let mut sibling_node: *mut ImGuiDockNode = (if parent_node.ChildNodes[0] == node { parent_node.ChildNodes[1]} else {parent_node.ChildNodes[0]});
        DockNodeTreeMerge(&g, parent_node, sibling_node);
    }
    else
    {
        // for (let n: c_int = 0; parent_node && n < IM_ARRAYSIZE(parent_node.ChildNodes); n++)
        for n in 0 .. parent_node.ChildNodes.len()
        {
            if (parent_node.ChildNodes[n] == node){
                node.ParentNode.ChildNodes[n]= None;}
            if is_null(parent_node) {
                break;
            }
            }
        dc.Nodes.SetVoidPtr(node.ID, null_mut());
        IM_DELETE(node);
    }
}

pub unsafe fn DockNodeComparerDepthMostFirst(lhs: *const ImGuiDockNode, rhs: *const ImGuiDockNode) -> c_int
{
    // let a: *const ImGuiDockNode = *(*const ImGuiDockNode const*)lhs;
    // let b: *const ImGuiDockNode = *(*const ImGuiDockNode const*)rhs;
    // return DockNodeGetDepth(b) - DockNodeGetDepth(a);
    DockNodeGetDepth(rhs) - DockNodeGetDepth(lhs)
}

// Pre C++0x doesn't allow us to use a function-local type (without linkage) as template parameter, so we moved this here.


// Garbage collect unused nodes (run once at init time)
pub unsafe fn DockContextPruneUnusedSettingsNodes(g: &mut ImguiContext)
{
    let g =  ctx;
    let dc  = &mut ctx.DockContext;
    // IM_ASSERT(g.Windows.Size == 0);

    ImPool<ImGuiDockContextPruneNodeData> pool;
    pool.Reserve(dc.NodesSettings.Size);

    // Count child nodes and compute RootID
    // for (let settings_n: c_int = 0; settings_n < dc.NodesSettings.Size; settings_n++)
    for settings_n in 0 .. dc.NodesSettings.len()
    {
        let settings: *mut ImGuiDockNodeSettings = &mut dc.NodesSettings[settings_n];
        let parent_data: *mut ImGuiDockContextPruneNodeData = if settings.ParentNodeId { pool.GetByKey(settings.ParentNodeId)} else {0};
        pool.GetOrAddByKey(settings.ID).RootId = if parent_data { parent_Data.RootId} else {settings.ID};
        if (settings.ParentNodeId){
            pool.GetOrAddByKey(settings.ParentNodeId).CountChildNodes+= 1;}
    }

    // Count reference to dock ids from dockspaces
    // We track the 'auto-DockNode <- manual-Window <- manual-DockSpace' in order to avoid 'auto-DockNode' being ditched by DockContextPruneUnusedSettingsNodes()
    // for (let settings_n: c_int = 0; settings_n < dc.NodesSettings.Size; settings_n++)
    for settings_n in 0 .. dc.NodesSettings.len()
    {
        let settings: *mut ImGuiDockNodeSettings = &mut dc.NodesSettings[settings_n];
        if (settings.ParentWindowId != 0){
            let window_settings: *mut ImGuiWindowSettings = FindWindowSettings(settings.ParentWindowId);
            if (is_not_null(window_settings))
            {
                if (window_settings.DockId){
                    let data: *mut ImGuiDockContextPruneNodeData = pool.GetByKey(window_settings.DockId);
                    if is_not_null(data){
                        data.CountChildNodes+= 1;}}}}
    }

    // Count reference to dock ids from window settings
    // We guard against the possibility of an invalid .ini file (RootID may point to a missing node)
    // for (settings: *mut ImGuiWindowSettings = g.SettingsWindows.begin(); settings != None; settings = g.SettingsWindows.next_chunk(settings))
    for settings in g.SettingsWindows.iter_mut()
    {
        let mut dock_id: ImguiHandle =  settings.DockId;
        if (dock_id){
            let data: *mut ImGuiDockContextPruneNodeData = pool.GetByKey(dock_id);
            if is_not_null(data)
            {
                data.CountWindows+= 1;
                let data_root: *mut ImGuiDockContextPruneNodeData = if (data.RootId == dock_id) { data} else {pool.GetByKey(data.RootId)};
                if is_not_null(data_root){
                    data_root.CountChildWindows+= 1;}
            }}}

    // Prune
    // for (let settings_n: c_int = 0; settings_n < dc.NodesSettings.Size; settings_n++)
    for settings_n in 0 .. dc.NodesSettings.len()
    {
        let settings: *mut ImGuiDockNodeSettings = &mut dc.NodesSettings[settings_n];
        let data: *mut ImGuiDockContextPruneNodeData = pool.GetByKey(settings.ID);
        if (data.CountWindows > 1){
            continue;}
        let data_root: *mut ImGuiDockContextPruneNodeData = if data.RootId == settings.ID { data} else { pool.GetByKey(data.RootId)};

        let mut remove: bool =  false;
        remove |= (data.CountWindows == 1 && settings.ParentNodeId == 0 && data.CountChildNodes == 0 && flag_clear(settings.Flags, ImGuiDockNodeFlags_CentralNode));  // Floating root node with only 1 window
        remove |= (data.CountWindows == 0 && settings.ParentNodeId == 0 && data.CountChildNodes == 0); // Leaf nodes with 0 window
        remove |= (data_root.CountChildWindows == 0);
        if (remove)
        {
            IMGUI_DEBUG_LOG_DOCKING("[docking] DockContextPruneUnusedSettingsNodes: Prune 0x{}\n", settings.ID);
            DockSettingsRemoveNodeReferences(&settings.ID, 1);
            settings.ID = 0;
        }
    }
}

pub unsafe fn DockContextBuildNodesFromSettings(g: &mut ImguiContext, node_settings_array: *mut ImGuiDockNodeSettings, node_settings_count: c_int)
{
    // Build nodes
    // for (let node_n: c_int = 0; node_n < node_settings_count; node_n++)
    for node_n in 0 .. node_settings_count
    {
        let settings: *mut ImGuiDockNodeSettings = &mut node_settings_array[node_n];
        if (settings.ID == 0){
            continue;}
        let node:*mut ImGuiDockNode = DockContextAddNode(ctx, settings.ID);
        node.ParentNode = if settings.ParentNodeId { DockContextFindNodeByID(ctx, settings.ParentNodeId)} else {None};
        node.Pos = ImVec2::new(settings.Pos.x, settings.Pos.y);
        node.Size = ImVec2::new(settings.Size.x, settings.Size.y);
        node.SizeRef = ImVec2::new(settings.SizeRef.x, settings.SizeRef.y);
        node.AuthorityForPos = node.AuthorityForSize = node.AuthorityForViewport = ImGuiDataAuthority_DockNode;
        if (node.ParentNode && node.ParentNode.ChildNodes[0] == null_mut()){
            node.ParentNode.ChildNodes[0] = node;}
        else if (node.ParentNode && node.ParentNode.ChildNodes[1] == null_mut()){
            node.ParentNode.ChildNodes[1] = node;}
        node.SelectedTabId = settings.SelectedTabId;
        node.SplitAxis = settings.SplitAxis;
        node.SetLocalFlags(settings.Flags & ImGuiDockNodeFlags_SavedFlagsMask_);

        // Bind host window immediately if it already exist (in case of a rebuild)
        // This is useful as the RootWindowForTitleBarHighlight links necessary to highlight the currently focused node requires node.HostWindow to be set.
        let mut host_window_title: [c_char;20] = [0;20];
        let root_node:*mut ImGuiDockNode = DockNodeGetRootNode(node);
        node.HostWindow = FindWindowByName(DockNodeGetHostWindowTitle(root_node, host_window_title, host_window_title.len()));
    }
}

pub unsafe fn DockContextBuildAddWindowsToNodes(g: &mut ImguiContext, root_id: ImguiHandle)
{
    // Rebind all windows to nodes (they can also lazily rebind but we'll have a visible glitch during the first frame)
    // for (let n: c_int = 0; n < g.Windows.len(); n++)
    // for n in 0 .. g.Windows.len()
    for _, window in g.
    {
        let mut window: &mut ImGuiWindow =  g.Windows[n];
        if (window.DockId == 0 || window.LastFrameActive < g.FrameCount - 1){
            continue;}
        if (window.DockNode != null_mut()){
            continue;}

        // let node:*mut ImGuiDockNode = DockContextFindNodeByID(ctx, window.DockId);

        let node = g.dock_context.find_node_by_id_mut(&window.DockId);

        // IM_ASSERT(node != NULL);   // This should have been called after DockContextBuildNodesFromSettings()
        if (root_id == 0 || DockNodeGetRootNode(node).ID == root_id){
            DockNodeAddWindow(node, window, true);}
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

pub unsafe fn DockContextQueueDock(g: &mut ImguiContext, target: *mut ImGuiWindow, target_node:*mut ImGuiDockNode, payload: *mut ImGuiWindow, split_dir: ImGuiDir, split_ratio: c_float, split_outer: bool)
{
    // IM_ASSERT(target != payload);
    let mut req: ImGuiDockRequest = Default::default();
    req.Type = ImGuiDockRequestType_Dock;
    req.DockTargetWindow = target;
    req.DockTargetNode = target_node;
    req.DockPayload = payload;
    req.DockSplitDir = split_dir;
    req.DockSplitRatio = split_ratio;
    req.DockSplitOuter = split_outer;
    ctx.DockContext.Requests.push(req);
}

pub unsafe fn DockContextQueueUndockWindow(g: &mut ImguiContext, window: &mut ImGuiWindow)
{
    let mut req: ImGuiDockRequest = Default::default();
    req.Type = ImGuiDockRequestType_Undock;
    req.UndockTargetWindow = window;
    ctx.DockContext.Requests.push(req);
}

pub unsafe fn DockContextQueueUndockNode(g: &mut ImguiContext, node:*mut ImGuiDockNode)
{
    let mut req: ImGuiDockRequest = Default::default();
    req.Type = ImGuiDockRequestType_Undock;
    req.UndockTargetNode = node;
    ctx.DockContext.Requests.push(req);
}

pub unsafe fn DockContextQueueNotifyRemovedNode(g: &mut ImguiContext, node:*mut ImGuiDockNode)
{
    let dc  = &mut ctx.DockContext;
    // for (let n: c_int = 0; n < dc.Requests.Size; n++)
    for n in 0 .. dc.Requests.len()
    {
        if (dc.Requests[n].DockTargetNode == node){
            dc.Requests[n].Type = ImGuiDockRequestType_None;}}
}

pub unsafe fn DockContextProcessDock(g: &mut ImguiContext, req: *mut ImGuiDockRequest)
{
    // IM_ASSERT((req.Type == ImGuiDockRequestType_Dock && req.DockPayload != NULL) || (req.Type == ImGuiDockRequestType_Split && req.DockPayload == NULL));
    // IM_ASSERT(req.DockTargetWindow != NULL || req.DockTargetNode != NULL);

    let g =  ctx;
    IM_UNUSED(g);

    let mut payload_window: &mut ImGuiWindow =  req.DockPayload;     // Optional
    let mut target_window: &mut ImGuiWindow =  req.DockTargetWindow;
    let node:*mut ImGuiDockNode = req.DockTargetNode;
    if (payload_window){
        // IMGUI_DEBUG_LOG_DOCKING("[docking] DockContextProcessDock node 0x{} target '{}' dock window '{}', split_dir {}\n", node ? node.ID : 0, target_window ? target_window.Name : "NULL", payload_window.Name, req.DockSplitDir);
    }
    else{ 
        // IMGUI_DEBUG_LOG_DOCKING("[docking] DockContextProcessDock node 0x{}, split_dir {}\n", node ? node.ID : 0, req.DockSplitDir);}
    }
    // Decide which Tab will be selected at the end of the operation
    let mut next_selected_id: ImguiHandle =  0;
    let payload_node:*mut ImGuiDockNode= None;
    if (payload_window)
    {
        payload_node = payload_window.DockNodeAsHost;
        payload_window.DockNodeAsHost= None; // Important to clear this as the node will have its life as a child which might be merged/deleted later.
        if (payload_node && payload_node.IsLeafNode()){
            next_selected_id = if payload_node.TabBar.NextSelectedTabId { payload_node.TabBar.NextSelectedTabId} else {payload_node.TabBar.SelectedTabId};}
        if (payload_node == null_mut()){
            next_selected_id = payload_window.TabId;}
    }

    // FIXME-DOCK: When we are trying to dock an existing single-window node into a loose window, transfer Node ID as well
    // When processing an interactive split, usually LastFrameAlive will be < g.FrameCount. But DockBuilder operations can make it ==.
    if (node) {}
        // IM_ASSERT(node.LastFrameAlive <= g.FrameCount);
    if (node && target_window && node == target_window.DockNodeAsHost) {}
        // IM_ASSERT(node.Windows.Size > 0 || node.IsSplitNode() || node.IsCentralNode());

    // Create new node and add existing window to it
    if (node == null_mut())
    {
        node = DockContextAddNode(ctx, 0);
        node.Pos = target_window.Pos;
        node.Size = target_window.Size;
        if (target_window.DockNodeAsHost == null_mut())
        {
            DockNodeAddWindow(node, target_window, true);
            node.TabBar.Tabs[0].Flags &= !ImGuiTabItemFlags_Unsorted;
            target_window.DockIsActive = true;
        }
    }

    let split_dir: ImGuiDir = req.DockSplitDir;
    if (split_dir != ImGuiDir_None)
    {
        // Split into two, one side will be our payload node unless we are dropping a loose window
        let split_axis: ImGuiAxis = if split_dir == ImGuiDir_Left || split_dir == ImGuiDir_Right { IM_GUI_AXIS_X } else { IM_GUI_AXIS_Y };
        let split_inheritor_child_idx: c_int = if split_dir == ImGuiDir_Left || split_dir == ImGuiDir_Up { 1} else { 0}; // Current contents will be moved to the opposite side
        let split_ratio: c_float =  req.DockSplitRatio;
        DockNodeTreeSplit(ctx, node, split_axis, split_inheritor_child_idx, split_ratio, payload_node);  // payload_node may be NULL here!
        let new_node:*mut ImGuiDockNode = node.ChildNodes[split_inheritor_child_idx ^ 1];
        new_node.HostWindow = node.HostWindow;
        node = new_node;
    }
    node.SetLocalFlags(node.LocalFlags & !ImGuiDockNodeFlags_HiddenTabBar);

    if (node != payload_node)
    {
        // Create tab bar before we call DockNodeMoveWindows (which would attempt to move the old tab-bar, which would lead us to payload tabs wrongly appearing before target tabs!)
        if (node.Windows.len() > 0 && node.TabBar == null_mut())
        {
            DockNodeAddTabBar(node);
            // for (let n: c_int = 0; n < node.Windows.len(); n++)
            for n in 0 .. node.Windows.len()
            {
                TabBarAddTab(node.TabBar, ImGuiTabItemFlags_None, node.Windows[n]);}
        }

        if (payload_node != null_mut())
        {
            // Transfer full payload node (with 1+ child windows or child nodes)
            if (payload_node.IsSplitNode())
            {
                if (node.Windows.len() > 0)
                {
                    // We can dock a split payload into a node that already has windows _only_ if our payload is a node tree with a single visible node.
                    // In this situation, we move the windows of the target node into the currently visible node of the payload.
                    // This allows us to preserve some of the underlying dock tree settings nicely.
                    // IM_ASSERT(payload_node.OnlyNodeWithWindows != NULL); // The docking should have been blocked by DockNodePreviewDockSetup() early on and never submitted.
                    let visible_node:*mut ImGuiDockNode = payload_node.OnlyNodeWithWindows;
                    if (visible_node.TabBar) {}
                        // IM_ASSERT(visible_node.TabBar.Tabs.Size > 0);
                    DockNodeMoveWindows(node, visible_node);
                    DockNodeMoveWindows(visible_node, node);
                    DockSettingsRenameNodeReferences(node.ID, visible_node.ID);
                }
                if (node.IsCentralNode())
                {
                    // Central node property needs to be moved to a leaf node, pick the last focused one.
                    // FIXME-DOCK: If we had to transfer other flags here, what would the policy be?
                    let last_focused_node:*mut ImGuiDockNode = DockContextFindNodeByID(ctx, payload_node.LastFocusedNodeId);
                    // IM_ASSERT(last_focused_node != NULL);
                    let last_focused_root_node:*mut ImGuiDockNode = DockNodeGetRootNode(last_focused_node);
                    // IM_ASSERT(last_focused_root_node == DockNodeGetRootNode(payload_node));
                    last_focused_node.SetLocalFlags(last_focused_node.LocalFlags | ImGuiDockNodeFlags_CentralNode);
                    node.SetLocalFlags(node.LocalFlags & !ImGuiDockNodeFlags_CentralNode);
                    last_focused_root_node.CentralNode = last_focused_node;
                }

                // IM_ASSERT(node.Windows.Size == 0);
                DockNodeMoveChildNodes(node, payload_node);
            }
            else
            {
                let mut payload_dock_id: ImguiHandle =  payload_node.ID;
                DockNodeMoveWindows(node, payload_node);
                DockSettingsRenameNodeReferences(payload_dock_id, node.ID);
            }
            DockContextRemoveNode(ctx, payload_node, true);
        }
        else if (payload_window)
        {
            // Transfer single window
            let mut payload_dock_id: ImguiHandle =  payload_window.DockId;
            node.VisibleWindow = payload_window;
            DockNodeAddWindow(node, payload_window, true);
            if (payload_dock_id != 0){
                DockSettingsRenameNodeReferences(payload_dock_id, node.ID);}
        }
    }
    else
    {
        // When docking a floating single window node we want to reevaluate auto-hiding of the tab bar
        node.WantHiddenTabBarUpdate = true;
    }

    // Update selection immediately
    if (tab_bar: &mut ImGuiTabBar = node.TabBar){
        tab_bar.NextSelectedTabId = next_selected_id;}
    MarkIniSettingsDirty();
}

// Problem:
//   Undocking a large (~full screen) window would leave it so large that the bottom right sizing corner would more
//   than likely be off the screen and the window would be hard to resize to fit on screen. This can be particularly problematic
//   with 'ConfigWindowsMoveFromTitleBarOnly=true' and/or with 'ConfigWindowsResizeFromEdges=false' as well (the later can be
//   due to missing IM_GUI_BACKEND_FLAGS_HAS_MOUSE_CURSORS backend flag).
// Solution:
//   When undocking a window we currently force its maximum size to 90% of the host viewport or monitor.
// Reevaluate this when we implement preserving docked/undocked size ("docking_wip/undocked_size" branch).
pub unsafe fn FixLargeWindowsWhenUndocking(size: &ImVec2, ref_viewport: *mut ImGuiViewport) -> ImVec2
{
    if (ref_viewport == null_mut()){
        return size;}

    let g = GImGui; // ImGuiContext& g = *GImGui;
    let max_size: ImVec2 = ImFloor(ref_viewport.WorkSize * 0.9);
    if (g.ConfigFlagsCurrFrame & ImGuiConfigFlags_ViewportsEnable)
    {
        let monitor: *const ImGuiPlatformMonitor = GetViewportPlatformMonitor(ref_viewport);
        max_size = ImFloor(monitor.WorkSize * 0.9);
    }
    return ImMin(size, max_size);
}

pub unsafe fn DockContextProcessUndockWindow(
    g: &mut ImguiContext,
    window: &mut ImGuiWindow,
    clear_persistent_docking_re0f32: bool)
{
    let g =  ctx;
    // IMGUI_DEBUG_LOG_DOCKING("[docking] DockContextProcessUndockWindow window '{}', clear_persistent_docking_ref = {}\n", window.Name, clear_persistent_docking_re0f32);
    if (window.DockNode){
        DockNodeRemoveWindow(window.DockNode, window, if clear_persistent_docking_ref { 0 }else {window.DockId});}
    else{
        window.DockId = 0;}
    window.Collapsed = false;
    window.DockIsActive = false;
    window.DockNodeIsVisible = window.DockTabIsVisible = false;
    window.Size = window.SizeFull = FixLargeWindowsWhenUndocking(window.SizeFull, window.Viewport);

    MarkIniSettingsDirty();
}

pub unsafe fn DockContextProcessUndockNode(g: &mut ImguiContext, node:*mut ImGuiDockNode)
{
    let g =  ctx;
    // IMGUI_DEBUG_LOG_DOCKING("[docking] DockContextProcessUndockNode node {}\n", node.ID);
    // IM_ASSERT(node.IsLeafNode());
    // IM_ASSERT(node.Windows.Size >= 1);

    if (node.IsRootNode() || node.IsCentralNode())
    {
        // In the case of a root node or central node, the node will have to stay in place. Create a new node to receive the payload.
        let new_node:*mut ImGuiDockNode = DockContextAddNode(ctx, 0);
        new_node.Pos = node.Pos;
        new_node.Size = node.Size;
        new_node.SizeRef = node.SizeRef;
        DockNodeMoveWindows(new_node, node);
        DockSettingsRenameNodeReferences(node.ID, new_node.ID);
        node = new_node;
    }
    else
    {
        // Otherwise extract our node and merge our sibling back into the parent node.
        // IM_ASSERT(node.ParentNode.ChildNodes[0] == node || node.ParentNode.ChildNodes[1] == node);
        let index_in_parent: c_int = if node.ParentNode.ChildNodes[0] == node { 0} else { 1};
        node.ParentNode.ChildNodes[index_in_parent]= None;
        DockNodeTreeMerge(ctx, node.ParentNode, node.ParentNode.ChildNodes[index_in_parent ^ 1]);
        node.ParentNode.AuthorityForViewport = ImGuiDataAuthority_Window; // The node that stays in place keeps the viewport, so our newly dragged out node will create a new viewport
        node.ParentNode= None;
    }
    // for (let n: c_int = 0; n < node.Windows.len(); n++)
    for n in 0 .. node.Windows.len()
    {
        let mut window: &mut ImGuiWindow =  node.Windows[n];
        window.Flags &= !ImGuiWindowFlags_ChildWindow;
        if (window.ParentWindow){
            window.Parentwindow.DC.ChildWindows.find_erase(window);}
        UpdateWindowParentAndRootLinks(window, window.Flags, null_mut());
    }
    node.AuthorityForPos = node.AuthorityForSize = ImGuiDataAuthority_DockNode;
    node.Size = FixLargeWindowsWhenUndocking(node.Size, node.Windows[0].Viewport);
    node.WantMouseMove = true;
    MarkIniSettingsDirty();
}

// This is mostly used for automation.
pub unsafe fn DockContextCalcDropPosForDocking(target: *mut ImGuiWindow, target_node:*mut ImGuiDockNode, payload_window: &mut ImGuiWindow, payload_node:*mut ImGuiDockNode, split_dir: ImGuiDir, split_outer: bool, out_pos: *mut ImVec2) -> bool
{
    // In DockNodePreviewDockSetup() for a root central node instead of showing both "inner" and "outer" drop rects
    // (which would be functionally identical) we only show the outer one. Reflect this here.
    if (target_node && target_node.ParentNode == None && target_node.IsCentralNode() && split_dir != ImGuiDir_None){
        split_outer = true;}
    let mut split_data = ImGuiDockPreviewData::default();
    DockNodePreviewDockSetup(target, target_node, payload_window, payload_node, &split_data, false, split_outer);
    if (split_data.DropRectsDraw[split_dir1].IsInverted()){
        return false;}
    *out_pos = split_data.DropRectsDraw[split_dir1].GetCenter();
    return true;
}
