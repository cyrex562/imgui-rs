use std::collections::HashMap;
use crate::config::ConfigFlags;
use crate::context::Context;
use crate::dock::{dock_builder_remove_node_child_nodes, dock_builder_remove_node_docked_windows, DOCKING_SPLITTER_SIZE, ImGuiDockNode};
use crate::dock::node::{dock_node_get_root_node, DockNode, DockNodeFlags, DockNodeSettings};
use crate::frame::get_frame_height;
use crate::{dock, INVALID_ID, window};
use crate::dock::request::{DockRequest, DockRequestType};
use crate::rect::Rect;
use crate::settings::{find_window_settings, SettingsHandler};
use crate::types::{DataAuthority, Direction, Id32};
use crate::utils::get_or_add;
use crate::vectors::two_d::Vector2D;
use crate::window::get::find_window_by_name;
use crate::window::Window;


#[derive(Default,Debug,Clone)]
pub struct DockContext {
    //ImGuiStorage                    Nodes;          // Map id -> ImGuiDockNode*: active nodes
    // pub nodes: HashMap<Id32, DockNode>,
    pub nodes: Vec<Id32>,
    // ImVector<ImGuiDockRequest>      Requests;
    pub requests: Vec<DockRequest>,
    // ImVector<ImGuiDockNodeSettings> NodesSettings;
    pub nodes_settings: Vec<DockNodeSettings>,
    // bool                            WantFullRebuild;
    pub want_full_rebuild: bool,
    // ImGuiDockContext()              { memset(this, 0, sizeof(*this)); }
}

pub fn dock_ctx_initialize(ctx: &mut Context)
    {
    // ImGuiContext& g = *ctx;

    // Add .ini handle for persistent docking data
    // ImGuiSettingsHandler ini_handler;
        let mut ini_handler = SettingsHandler {
            type_name: String::from("Docking"),
            type_hash: DimgHashStr(String::from("Docking")),
            clear_all_fn: DockSettingsHandler::ClearAll,
            read_init_fn: DockSettingsHandler::ClearAll, // Also clear on read
            read_open_fn: DockSettingsHandler::ReadOpen,
            read_line_fn: DockSettingsHandler::ReadLine,
            apply_all_fn: DockSettingsHandler::ApplyAll,
            write_all_fn: DockSettingsHandler::WriteAll,
            user_data: vec![]
        };
    ctx.settings_handlers.push_back(ini_handler);
}

//void ImGui::DockContextShutdown(ImGuiContext* ctx)
pub fn dock_context_shutdown(ctx: &mut Context)
{
    // ImGuiDockContext* dc  = &ctx->DockContext;
    //for (int n = 0; n < dc->Nodes.Data.Size; n += 1)
    for n in 0 .. ctx.dock_context.nodes.len()
    {
        let node_id = ctx.dock_context.nodes.get(n);
        if node_id.is_some() {
            ctx.dock_context.nodes.remove(n);
        }
        // if (ImGuiDockNode * node = (ImGuiDockNode *)
        // dc -> Nodes.Data[n].val_p){
        //     IM_DELETE(node);
        // }
    }
}

// void ImGui::DockContextClearNodes(ImGuiContext* ctx, ImGuiID root_id, bool clear_settings_refs)
pub fn dock_context_clear_nodes(g: &mut Context, root_id: Id32, clear_settings_refs: bool)
{
    // IM_UNUSED(ctx);
    // IM_ASSERT(ctx == GImGui);
    // DockBuilderRemoveNodeDockedWindows(root_id, clear_settings_refs);
    dock_builder_remove_node_docked_windows(g, root_id, clear_settings_refs);
    // DockBuilderRemoveNodeChildNodes(root_id);
    dock_builder_remove_node_child_nodes(g, root_id);
}

// [DEBUG] This function also acts as a defacto test to make sure we can rebuild from scratch without a glitch
// (Different from DockSettingsHandler_ClearAll() + DockSettingsHandler_ApplyAll() because this reuses current settings!)
// void ImGui::DockContextRebuildNodes(ImGuiContext* ctx)
pub fn dock_context_rebuild_nodes(ctx: &mut Context)
{
    // ImGuiContext& g = *ctx;
    // ImGuiDockContext* dc = &ctx->DockContext;
    // IMGUI_DEBUG_LOG_DOCKING("[docking] DockContextRebuildNodes\n");
    SaveIniSettingsToMemory();
    // ImGuiID root_id = 0; // Rebuild all
    let mut root_id: Id32 = 0;
    dock_context_clear_nodes(ctx, root_id, false);
    dock_context_build_nodes_from_settings(ctx, &mut ctx.dock_context.nodes_settings);
    dock_context_build_add_windows_to_nodes(ctx, root_id);
}

// Docking context update function, called by NewFrame()
// void ImGui::DockContextNewFrameUpdateUndocking(ImGuiContext* ctx)
pub fn dock_context_new_frame_update_undocking(ctx: &mut Context)
{
    // ImGuiContext& g = *ctx;
    // ImGuiDockContext* dc = &ctx->DockContext;
    let mut dc = &mut ctx.dock_context;
    if !(ctx.io.config_flags.contains(&ConfigFlags::DockingEnable))
    {
        if dc.nodes.len() > 0 || dc.requests.len() > 0 {
            dock_context_clear_nodes(ctx, 0, true);
        }
        return;
    }

    // Setting NoSplit at runtime merges all nodes
    if ctx.io.config_docking_no_split {
        // for (int n = 0; n < dc->Nodes.Data.Size; n += 1){
        for n in 0 .. dc.nodes.len() {
            // if (ImGuiDockNode * node = (ImGuiDockNode *)
            // dc -> Nodes.Data[n].val_p){
            let node = dc.nodes.get(n);
            if node.is_some() {
                let node_u = node.unwrap();
                if node_u.is_root_node() && node_u.is_split_node()
                {
                    dock_builder_remove_node_child_nodes(node.id);
                    //dc->WantFullRebuild = true;
                }
            }
        }
    }

    // Process full rebuild
// #if 0
//     if (ImGui::IsKeyPressed(ImGui::GetKeyIndex(ImGuiKey_C)))
//         dc->WantFullRebuild = true;
// #endif
    if dc.want_full_rebuild
    {
        dock_context_rebuild_nodes(ctx);
        dc.want_full_rebuild = false;
    }

    // Process Undocking requests (we need to process them _before_ the UpdateMouseMovingWindowNewFrame call in NewFrame)
    // for (int n = 0; n < dc->Requests.Size; n += 1)
    for n in 0 .. dc.requests.len()
    {
        // ImGuiDockRequest* req = &dc->Requests[n];
        let req = dc.requests.get(n).unwrap();
        if req.Type == DockRequestType::Undock && req.undock_target_window_id {
            dock_context_process_undock_window(ctx, req.undock_target_window_id);
        }
        else if req.requst_type == DockRequestType::Undock && req.undock_target_node_id {
            dock_context_process_undock_node(ctx, req.undock_target_node_id);
        }
    }
}

// void ImGui::DockContextEndFrame(ImGuiContext* ctx)
pub fn dock_context_end_frame(g: &mut Context)
{
    // Draw backgrounds of node missing their window
    // ImGuiContext& g = *ctx;
    // ImGuiDockContext* dc = &g.DockContext;
    let dc = &mut g.dock_context;
    // for (int n = 0; n < dc.Nodes.data.size; n += 1)
for node in dc.nodes.iter_mut()
{
    if ImGuiDockNode * node = dc.Nodes.data[n].val_p {
        if node.last_frame_active == g.frame_count && node.IsVisible && node.host_window && node.is_leaf_node() && ! node.is_bg_drawn_this_frame {
            let mut bg_rect = Rect::new2(
                &node.pos + &Vector2D::new(0.0, get_frame_height(g)),
                &node.pos + &node.size);
            let mut bg_rounding_flags = calc_rounding_flags_for_rect_in_rect(bg_rect, node.host_window.rect(), DOCKING_SPLITTER_SIZE); node.host_window.draw_list.channels_set_current(0);
            node.host_window.draw_list.add_rect_filled(&bg_rect.min, &bg_rect.max, node.last_bg_color, node.host_window.WindowRounding, bg_rounding_flags);
}}}
}

// static ImGuiDockNode* ImGui::DockContextFindNodeByID(ImGuiContext* ctx, ImGuiID id)
pub fn dock_context_find_node_by_id(g: &mut Context, id: Id32) -> Option<&mut DockNode>
{
    // return (ImGuiDockNode*)ctx.DockContext.Nodes.GetVoidPtr(id);
    // g.dock_context.nodes.get_mut(&id).expect(format!("failed to get node for id {}", id).as_str())
    g.dock_context.nodes.get_mut(&id)
}

/// Docking context update function, called by NewFrame()
pub fn dock_context_new_frame_update_docking(g: &mut Context)
{
    // ImGuiContext& g = *ctx;
    // ImGuiDockContext* dc  = &ctx->DockContext;
    let mut dc: &mut DockContext = &mut g.dock_context;
    if !(g.io.config_flags.contains(&ConfigFlags::DockingEnable)) {
        return;
    }

    // [DEBUG] Store hovered dock node.
    // We could in theory use DockNodeTreeFindVisibleNodeByPos() on the root host dock node, but using ->dock_node is a good shortcut.
    // Note this is mostly a debug thing and isn't actually used for docking target, because docking involve more detailed filtering.
    g.hovered_dock_node_id = INVALID_ID;
    // if (ImGuiWindow* hovered_window = g.hovered_window_under_moving_window)
    if g.hovered_window_under_moving_window_id != INVALID_ID
    {
        let hovered_window = g.get_window(g.hovered_window_under_moving_window_id);
        let hovered_window_root_window = g.get_window(hovered_window.root_window_id);
        if hovered_window.dock_node_as_host_id.is_some() {
            g.hovered_dock_node = dock_node_tree_find_visible_node_by_pos(&hovered_window.dock_node_as_host_id.unwrap(), &g.io.mouse_pos);
        }
        else if hovered_window_root_window.dock_node.is_some() {
            g.hovered_dock_node = hovered_window.root_window.dock_node;
        }
    }

    // Process Docking requests
    // for (int n = 0; n < dc.Requests.size; n += 1)
    for req in dc.requests.iter_mut()
    {
        if req.request_type == DockRequestType::Dock {
            dock::dock_context_process_dock(g, req);
        }
    }
    // dc.Requests.resize(0);

    // Create windows for each automatic docking nodes
    // We can have NULL pointers when we delete nodes, but because id are recycled this should amortize nicely (and our node count will never be very high)
    // for (int n = 0; n < dc.Nodes.data.size; n += 1){
    for (_, node) in dc.nodes.iter_mut() {
        // if (ImGuiDockNode * node = (ImGuiDockNode *)
        // dc.Nodes.data[n].val_p){
        //     if (node.IsFloatingNode()) {
        //         DockNodeUpdate(node);
        //     }
        // }
        if node.is_floating_node() {
            dock::dock_node_update(g, node)
        }
    }
}

// ImGuiID ImGui::dock_context_gen_node_id(ImGuiContext* ctx)
pub fn dock_context_gen_node_id(g: &mut Context) -> Id32
{
    // Generate an id for new node (the exact id value doesn't matter as long as it is not already used)
    // FIXME-OPT FIXME-DOCK: This is suboptimal, even if the node count is small enough not to be a worry.0
    // We should poke in ctx->Nodes to find a suitable id faster. Even more so trivial that ctx->Nodes lookup is already sorted.
    // ImGuiID id = 0x0001;
    let mut id = 0x0001;
    while dock_context_find_node_by_id(g, id).is_some() {
        id += 1;
    }
    return id;
}

// static ImGuiDockNode* ImGui::DockContextAddNode(ImGuiContext* ctx, ImGuiID id)
pub fn dock_context_add_node(g: &mut Context, mut id: Id32) -> &mut DockNode
{
    // Generate an id for the new node (the exact id value doesn't matter as long as it is not already used) and add the first window.
    // ImGuiContext& g = *ctx;
    if id == INVALID_ID {
        id = dock_context_gen_node_id(g);
    }
    // else
        // IM_ASSERT(DockContextFindNodeByID(ctx, id) == NULL);

    // We don't set node->last_frame_alive on construction. Nodes are always created at all time to reflect .ini settings!
    // IMGUI_DEBUG_LOG_DOCKING("[docking] DockContextAddNode 0x%08X\n", id);
    // ImGuiDockNode* node = IM_NEW(ImGuiDockNode)(id);
    let mut node = DockNode::new(id);
    // ctx.DockContext.Nodes.SetVoidPtr(node.ID, node);
    g.dock_context.nodes.insert(id, node);
    return &mut node;
}

// Pre C++0x doesn't allow us to use a function-local type (without linkage) as template parameter, so we moved this here.
#[derive(Default,Debug,Clone)]
pub struct DockContextPruneNodeData
{
    // int         CountWindows, CountChildWindows, CountChildNodes;
    pub count_windows: i32,
    pub count_child_windows: i32,
    pub count_child_nodes: i32,
    // ImGuiID     RootId;
    pub root_id: Id32
    // ImGuiDockContextPruneNodeData() { CountWindows = CountChildWindows = CountChildNodes = 0; RootId = 0; }
}

// Garbage collect unused nodes (run once at init time)
// static void DockContextPruneUnusedSettingsNodes(ImGuiContext* ctx)
pub fn dock_context_prune_unused_settings_nodes(g: &mut Context)
{
    // ImGuiContext& g = *ctx;
    // ImGuiDockContext* dc  = &ctx.DockContext;
    let mut dc = &mut g.dock_context;
    // IM_ASSERT(g.windows.size == 0);

    // ImPool<ImGuiDockContextPruneNodeData> pool;
    let mut pool: HashMap<Id32, DockContextPruneNodeData> = HashMap::new();

    // Count child nodes and compute RootID
    // for (int settings_n = 0; settings_n < dc.NodesSettings.size; settings_n += 1)
    for settings_n in 0 .. dc.nodes_settings.len()
    {
        // ImGuiDockNodeSettings* settings = &dc.NodesSettings[settings_n];
        let settings = &dc.nodes_settings[settings_n];
        // ImGuiDockContextPruneNodeData* parent_data = settings.parent_node_id ? pool.GetByKey(settings.parent_node_id) : 0;
        let mut parent_data = pool.get_mut(&settings.parent_node_id);

        // pool.GetOrAddByKey(settings.ID).RootId = parent_data ? parent_data.RootId : settings.ID;
        let mut pool_val = get_or_add(&mut pool, &settings.id);
        pool_val.root_id = if parent_data.is_some() {
            parent_data.unwrap().root_id
        } else {
            settings.id
        };

        if settings.parent_node_id != INVALID_ID {
            // pool.GetOrAddByKey(settings.parent_node_id).CountChildNodes += 1;
            pool_val.count_child_nodes += 1;
        }
    }

    // Count reference to dock ids from dockspaces
    // We track the 'auto-dock_node <- manual-window <- manual-DockSpace' in order to avoid 'auto-dock_node' being ditched by DockContextPruneUnusedSettingsNodes()
    // for (int settings_n = 0; settings_n < dc.NodesSettings.size; settings_n += 1)
    for settings_n in 0 .. dc.nodes_settings.len()
    {
        // ImGuiDockNodeSettings* settings = &dc.NodesSettings[settings_n];
        let settings = &mut dc.nodes_settings[settings_n];
        // if (settings.ParentWindowId != 0)
        if settings.parent_window_id != INVALID_ID
        {
            let window_settings = find_window_settings(g, settings.parent_window_id);
            // if (ImGuiWindowSettings * window_settings = FindWindowSettings(settings.ParentWindowId))
            if window_settings.id != INVALID_ID
            {
                // if (window_settings.dock_id)
                if window_settings.dock_id != INVALID_ID
                {
                    // if (ImGuiDockContextPruneNodeData * data = pool.GetByKey(window_settings.dock_id))
                    let data = pool.get_mut(&window_settings.dock_id)
                    if data.is_some()
                    {
                        // data.CountChildNodes += 1;
                        data.unwrap().count_child_nodes += 1;
                    }
                }
            }
        }
    }

    // Count reference to dock ids from window settings
    // We guard against the possibility of an invalid .ini file (RootID may point to a missing node)
    // for (ImGuiWindowSettings* settings = g.settings_windows.begin(); settings != NULL; settings = g.settings_windows.next_chunk(settings))
    for settings in g.settings_windows.iter_mut() {
        let dock_id = settings.dock_id;
        // if (ImGuiID dock_id = settings.dock_id){
        if dock_id != INVALID_ID {
            // if (ImGuiDockContextPruneNodeData * data = pool.GetByKey(dock_id)) {
            let data = pool.get_mut(&dock_id);
            if data.is_some() {
                data.unwrap().count_windows += 1;
                // if (ImGuiDockContextPruneNodeData * data_root = (data.RootId == dock_id)? data: pool.GetByKey(data.RootId)){
                let data_root = if data.unwrap().root_id == dock_id {
                   data }
                        else {
                         pool.get_mut(&data.unwrap().root_id)
                    };
                    if data_root.is_some() {
                    data_root.unwrap().child_count_windows += 1;
                }
            }
        }
    }

    // Prune
    // for (int settings_n = 0; settings_n < dc.NodesSettings.size; settings_n += 1)
    for settings_n in 0 .. dc.nodes_settings.len()
    {
        // ImGuiDockNodeSettings* settings = &dc.NodesSettings[settings_n];
        let settings = &mut dc.nodes_settings[settings_n];
        // ImGuiDockContextPruneNodeData* data = pool.GetByKey(settings.ID);
        let data = pool.get_mut(&settings.id);
        if data.unwrap().count_windows > 1 {
            continue;
        }
        // ImGuiDockContextPruneNodeData* data_root = (data.RootId == settings.ID) ? data : pool.GetByKey(data.RootId);
        let data_root = if data.unwrap().root_id == settings.id { data} else {
            pool.get_mut(&data.unwrap().root_id)
        };

        let mut remove = false;
        remove |= (data.count_windows == 1 && settings.parent_node_id == 0 && data.count_child_nodes == 0 && !(settings.flags.contains(&DockNodeFlags::CentralNode)));  // Floating root node with only 1 window
        remove |= (data.count_windows == 0 && settings.parent_node_id == 0 && data.count_child_nodes == 0); // Leaf nodes with 0 window
        remove |= (data_root.count_child_windows == 0);
        if remove
        {
            // IMGUI_DEBUG_LOG_DOCKING("[docking] DockContextPruneUnusedSettingsNodes: Prune 0x%08X\n", settings.ID);
            dock::dock_settings_remove_node_references(g, &settings.id, 1);
            settings.id = 0;
        }
    }
}

// static void DockContextBuildNodesFromSettings(ImGuiContext* ctx, ImGuiDockNodeSettings* node_settings_array, int node_settings_count)
pub fn dock_context_buld_nodes_from_settings(g: &mut Context, node_settings_array: &mut Vec<DockNodeSettings>, node_settings_count: i32)
{
    // build nodes
    // for (int node_n = 0; node_n < node_settings_count; node_n += 1)
    for node_n in 0 .. node_settings_count
    {
        // ImGuiDockNodeSettings* settings = &node_settings_array[node_n];
        let settings = node_settings_array.get_mut(node_n).unwrap();

        if settings.id == INVALID_ID {
            continue;
        }
        // ImGuiDockNode* node = DockContextAddNode(ctx, settings.id);
        let mut node = dock_context_add_node(g, settings.id);
        // node.ParentNode = settings.parent_node_id ? dock_context_find_node_by_id(ctx, settings.parent_node_id) : NULL;
        node.parent_node_id = settings.parent_node_id;
        node.pos = Vector2D::new(settings.pos.x, settings.pos.y);
        node.size = Vector2D::new(settings.size.x, settings.size.y);
        node.size_ref = Vector2D::new(settings.size_ref.x, settings.size_ref.y);
        node.authority_for_pos = DataAuthority::DockNode;
        node.authority_for_size = DataAuthority::DockNode;
            node.authority_for_viewport = DataAuthority::DockNode;
        if node.parent_node && node.parent_node.child_nodes[0] == NULL {
            node.parent_node.child_nodes[0] = node;
        }
        else if node.parent_node && node.parent_node.child_nodes[1] == NULL {
            node.parent_node.child_nodes[1] = node;
        }
        node.selected_tab_id = settings.selected_tab_id;
        node.split_axis = settings.split_axis;
        node.set_local_flags(settings.flags & DockNodeFlags::SavedFlagsMask);

        // Bind host window immediately if it already exist (in case of a rebuild)
        // This is useful as the root_window_for_title_bar_highlight links necessary to highlight the currently focused node requires node->host_window to be set.
        // char host_window_title[20];
        let mut host_window_tile = String::from("");
        // ImGuiDockNode* root_node = DockNodeGetRootNode(node);
        let root_node = dock_node_get_root_node(g, node);
        node.host_window_id = find_window_by_name(g, dock_node_get_host_window_title(root_node, host_window_title, (host_window_title.len()))).unwrap().id;
    }
}

// void DockContextBuildAddWindowsToNodes(ImGuiContext* ctx, ImGuiID root_id)
pub fn dock_context_build_add_windows_to_nodes(g: &mut Context, root_id: Id32)
{
    // Rebind all windows to nodes (they can also lazily rebind but we'll have a visible glitch during the first frame)
    // ImGuiContext& g = *ctx;
    // for (int n = 0; n < g.windows.size; n += 1)
    // for n in 0 .. g.windows.len()
    for (_, window) in g.windows.iter_mut()
    {
        // ImGuiWindow* window = g.windows[n];
        // let window = g.windows.get_mut(n).unwrap();
        if window.dock_id == INVALID_ID || window.last_frame_active < g.frame_count - 1 {
            continue;
        }
        if window.dock_node_id != INVALID_ID {
            continue;
        }

        // ImGuiDockNode* node = dock_context_find_node_by_id(ctx, window.dock_id);
        let node = dock_context_find_node_by_id(g, window.dock_id);
        // IM_ASSERT(node != NULL);   // This should have been called after DockContextBuildNodesFromSettings()
        if root_id == INVALID_ID || dock_node_get_root_node(g, node.unwrap()).id == root_id {
            dock::dock_node_add_window(g, node.unwrap(), window, true);
        }
    }
}

// void DockContextQueueDock(ImGuiContext* ctx, ImGuiWindow* target, ImGuiDockNode* target_node, ImGuiWindow* payload, ImGuiDir split_dir, float split_ratio, bool split_outer)
pub fn dock_context_queue_dock(g: &mut Context, target: &mut Window, target_node: &mut DockNode, payload: &mut window::Window, split_dir: Direction, split_ratio: f32, split_outer: bool)
{
    // IM_ASSERT(target != payload);
    // ImGuiDockRequest req;
    let mut req = DockRequest::default();
    req.request_type = DockRequestType::Dock;
    req.dock_target_window_id = target.id;
    req.dock_target_node_id = target_node.id;
    req.dock_payload_id = payload.id;
    req.dock_split_dir = split_dir;
    req.dock_split_ratio = split_ratio;
    req.dock_split_outer = split_outer;
    g.dock_context.requests.push_back(req);
}

// void DockContextQueueUndockWindow(ImGuiContext* ctx, ImGuiWindow* window)
pub fn dock_context_queue_undock_window(g: &mut Context, window: &mut Window)
{
    // ImGuiDockRequest req;
    let mut req = DockRequest::default();
    // req.Type = DockRequestType::Undock;
    req.request_type = DockRequestType::Undock;
    // req.UndockTargetWindow = window;
    req.undock_target_window_id = window.id;
    // ctx.dock_context.requests.push_back(req);
    g.dock_context.requests.push(req);
}

// void DockContextQueueUndockNode(ImGuiContext* ctx, ImGuiDockNode* node)
pub fn dock_context_queue_undock_node(g: &mut Context, node: &mut DockNode)
{
    // ImGuiDockRequest req;
    // req.Type = DockRequestType::Undock;
    // req.UndockTargetNode = node;
    // ctx.dock_context.requests.push_back(req);
    let req = DockRequest {
        request_type: DockRequestType::Undock,
        undock_target_node_id: node.id,
        ..Default::default()
    };
    g.dock_context.requests.push(req);
}

// void DockContextQueueNotifyRemovedNode(ImGuiContext* ctx, ImGuiDockNode* node)
pub fn dock_context_queue_notify_remove_node(g: &mut Context, node: &mut DockNode)
{
    // ImGuiDockContext* dc  = &ctx.dock_context;
    let dc = &mut g.dock_context;
    // for (int n = 0; n < dc.requests.size; n += 1
    for req in dc.requests.iter_mut()
    {

        // if (dc.requests[n].dock_target_node == node)
        if req.dock_target_node_id == node.id
        {
            // dc.requests[n].Type = DockRequestType::None;
            req.request_type = DockRequestType::None;
        }
    }
}
