use crate::axis::Axis;
use crate::config::ConfigFlags;
use crate::context::Context;
use crate::dock::builder::{
    dock_builder_remove_node_child_nodes, dock_builder_remove_node_docked_windows,
};
use crate::dock::defines::DOCKING_SPLITTER_SIZE;
use crate::dock::node::tree::{dock_node_tree_merge, dock_node_tree_split, dock_node_tree_update_pos_size};
use crate::dock::node::window::dock_node_add_window;
use crate::dock::node::{
    dock_node_get_root_node, dock_node_update_has_central_node_child
    , DockNode, DockNodeFlags, DockNodeSettings,
};
use crate::dock::preview::DockPreviewData;
use crate::dock::request::{DockRequest, DockRequestType};
use crate::dock::{node, ops, settings, ImGuiDockNode};
use crate::frame::get_frame_height;
use crate::rect::Rect;
use crate::settings::{find_window_settings, mark_ini_settings_dirty, SettingsHandler};
use crate::tab_bar::TabItemFlags;
use crate::types::{DataAuthority, Direction, Id32};
use crate::utils::{add_hash_set, get_or_add};
use crate::vectors::vector_2d::Vector2D;
use crate::window::get::find_window_by_name;
use crate::window::lifecycle::update_window_parent_and_root_links;
use crate::window::{Window, WindowFlags};
use crate::{dock, hash_string, window, INVALID_ID};
use std::collections::{HashMap, HashSet};
use crate::dock::node::preview::dock_node_preview_dock_setup;
use crate::dock::node::tab_bar::dock_node_add_tab_bar;

#[derive(Default, Debug, Clone)]
pub struct DockContext {
    //ImGuiStorage                    Nodes;          // Map id -> ImGuiDockNode*: active nodes
    // pub nodes: HashMap<Id32, DockNode>,
    pub nodes: Vec<Id32>,
    // ImVector<ImGuiDockRequest>      Requests;
    pub requests: Vec<DockRequest>,
    // ImVector<ImGuiDockNodeSettings> nodes_settings;
    pub nodes_settings: Vec<DockNodeSettings>,
    // bool                            WantFullRebuild;
    pub want_full_rebuild: bool,
    // ImGuiDockContext()              { memset(this, 0, sizeof(*this)); }
}

pub fn dock_ctx_initialize(g: &mut Context) {
    // ImGuiContext& g = *ctx;

    // Add .ini handle for persistent docking data
    // ImGuiSettingsHandler ini_handler;
    let mut ini_handler = SettingsHandler {
        type_name: String::from("Docking"),
        type_hash: hash_string(String::from("Docking").as_str(), 0),
        clear_all_fn: DockSettingsHandler::ClearAll,
        read_init_fn: DockSettingsHandler::ClearAll, // Also clear on read
        read_open_fn: DockSettingsHandler::ReadOpen,
        read_line_fn: DockSettingsHandler::ReadLine,
        apply_all_fn: DockSettingsHandler::ApplyAll,
        write_all_fn: DockSettingsHandler::WriteAll,
        user_data: vec![],
    };
    g.settings_handlers.push_back(ini_handler);
}

//void ImGui::DockContextShutdown(ImGuiContext* ctx)
pub fn dock_context_shutdown(g: &mut Context) {
    // ImGuiDockContext* dc  = &ctx->DockContext;
    //for (int n = 0; n < dc->Nodes.Data.Size; n += 1)
    for n in 0..g.dock_context.nodes.len() {
        let node_id = g.dock_context.nodes.get(n);
        if node_id.is_some() {
            g.dock_context.nodes.remove(n);
        }
        // TODO:
        // if (ImGuiDockNode * node = (ImGuiDockNode *)
        // dc -> Nodes.Data[n].val_p){
        //     IM_DELETE(node);
        // }
    }
}

// void ImGui::DockContextClearNodes(ImGuiContext* ctx, Id32 root_id, bool clear_settings_refs)
pub fn dock_context_clear_nodes(g: &mut Context, root_id: Id32, clear_settings_refs: bool) {
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
pub fn dock_context_rebuild_nodes(g: &mut Context) {
    // ImGuiContext& g = *ctx;
    // ImGuiDockContext* dc = &ctx->DockContext;
    // IMGUI_DEBUG_LOG_DOCKING("[docking] DockContextRebuildNodes\n");
    SaveIniSettingsToMemory();
    // Id32 root_id = 0; // Rebuild all
    let mut root_id: Id32 = 0;
    dock_context_clear_nodes(g, root_id, false);
    dock_context_build_nodes_from_settings(g, &mut g.dock_context.nodes_settings);
    dock_context_build_add_windows_to_nodes(g, root_id);
}

// Docking context update function, called by NewFrame()
// void ImGui::DockContextNewFrameUpdateUndocking(ImGuiContext* ctx)
pub fn dock_context_new_frame_update_undocking(g: &mut Context) {
    // ImGuiContext& g = *ctx;
    // ImGuiDockContext* dc = &ctx->DockContext;
    let mut dc = &mut g.dock_context;
    if !(g.io.config_flags.contains(&ConfigFlags::DockingEnable)) {
        if dc.nodes.len() > 0 || dc.requests.len() > 0 {
            dock_context_clear_nodes(g, 0, true);
        }
        return;
    }

    // Setting NoSplit at runtime merges all nodes
    if g.io.config_docking_no_split {
        // for (int n = 0; n < dc->Nodes.Data.Size; n += 1){
        for n in 0..dc.nodes.len() {
            // if (ImGuiDockNode * node = (ImGuiDockNode *)
            // dc -> Nodes.Data[n].val_p){
            let node = dc.nodes.get(n);
            if node.is_some() {
                let node_u = node.unwrap();
                if node_u.is_root_node() && node_u.is_split_node() {
                    dock_builder_remove_node_child_nodes(g, node.id);
                    dc.want_full_rebuild = true;
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
    if dc.want_full_rebuild {
        dock_context_rebuild_nodes(g);
        dc.want_full_rebuild = false;
    }

    // Process Undocking requests (we need to process them _before_ the UpdateMouseMovingWindowNewFrame call in NewFrame)
    // for (int n = 0; n < dc->Requests.Size; n += 1)
    for n in 0..dc.requests.len() {
        // ImGuiDockRequest* req = &dc->Requests[n];
        let req = dc.requests.get(n).unwrap();
        if req.request_type == DockRequestType::Undock && req.undock_target_window_id != INVALID_ID
        {
            let tgt_win = g.window_mut(req.undock_target_window_id);
            dock_context_process_undock_window(g, tgt_win, false);
        } else if req.requst_type == DockRequestType::Undock
            && req.undock_target_node_id != INVALID_ID
        {
            let tgt_node = g.dock_node_mut(req.undock_target_node_id).unwrap();
            dock_context_process_undock_node(g, tgt_node);
        }
    }
}

// void ImGui::DockContextEndFrame(ImGuiContext* ctx)
pub fn dock_context_end_frame(g: &mut Context) {
    // Draw backgrounds of node missing their window
    // ImGuiContext& g = *ctx;
    // ImGuiDockContext* dc = &g.DockContext;
    let dc = &mut g.dock_context;
    // for (int n = 0; n < dc.Nodes.data.size; n += 1)
    for node in dc.nodes.iter_mut() {
        if ImGuiDockNode * node = dc.Nodes.data[n].val_p {
            if node.last_frame_active == g.frame_count
                && node.is_visible
                && node.host_window
                && node.is_leaf_node()
                && !node.is_bg_drawn_this_frame
            {
                let mut bg_rect = Rect::new2(
                    &node.pos + &Vector2D::new(0.0, get_frame_height(g)),
                    &node.pos + &node.size,
                );
                let mut bg_rounding_flags = calc_rounding_flags_for_rect_in_rect(
                    bg_rect,
                    node.host_window.rect(),
                    DOCKING_SPLITTER_SIZE,
                );
                node.host_window.draw_list.channels_set_current(0);
                node.host_window.draw_list.add_rect_filled(
                    &bg_rect.min,
                    &bg_rect.max,
                    node.last_bg_color,
                    node.host_window.WindowRounding,
                    bg_rounding_flags,
                );
            }
        }
    }
}

// static ImGuiDockNode* ImGui::DockContextFindNodeByID(ImGuiContext* ctx, Id32 id)
pub fn dock_context_find_node_by_id(g: &mut Context, id: Id32) -> Option<&mut DockNode> {
    // return (ImGuiDockNode*)ctx.DockContext.Nodes.GetVoidPtr(id);
    // g.dock_context.nodes.get_mut(&id).expect(format!("failed to get node for id {}", id).as_str())
    g.dock_context.nodes.get_mut(&id)
}

/// Docking context update function, called by NewFrame()
pub fn dock_context_new_frame_update_docking(g: &mut Context) {
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
    // if (Window* hovered_window = g.hovered_window_under_moving_window)
    if g.hovered_window_under_moving_window_id != INVALID_ID {
        let hovered_window = g.window_mut(g.hovered_window_under_moving_window_id);
        let hovered_window_root_window = g.window_mut(hovered_window.root_window_id);
        if hovered_window.dock_node_as_host_id.is_some() {
            g.hovered_dock_node = dock_node_tree_find_visible_node_by_pos(
                &hovered_window.dock_node_as_host_id.unwrap(),
                &g.io.mouse_pos,
            );
        } else if hovered_window_root_window.dock_node_id.is_some() {
            g.hovered_dock_node = hovered_window.root_window.dock_node;
        }
    }

    // Process Docking requests
    // for (int n = 0; n < dc.Requests.size; n += 1)
    for req in dc.requests.iter_mut() {
        if req.request_type == DockRequestType::Dock {
            dock_context_process_dock(g, req);
        }
    }
    // dc.Requests.resize(0);

    // Create windows for each automatic docking nodes
    // We can have None pointers when we delete nodes, but because id are recycled this should amortize nicely (and our node count will never be very high)
    // for (int n = 0; n < dc.Nodes.data.size; n += 1){
    for (_, node) in dc.nodes.iter_mut() {
        // if (ImGuiDockNode * node = (ImGuiDockNode *)
        // dc.Nodes.data[n].val_p){
        //     if (node.is_floating_node()) {
        //         dock_node_update(node);
        //     }
        // }
        if node.is_floating_node() {
            node::dock_node_update(g, node)
        }
    }
}

// Id32 ImGui::dock_context_gen_node_id(ImGuiContext* ctx)
pub fn dock_context_gen_node_id(g: &mut Context) -> Id32 {
    // Generate an id for new node (the exact id value doesn't matter as long as it is not already used)
    // FIXME-OPT FIXME-DOCK: This is suboptimal, even if the node count is small enough not to be a worry.0
    // We should poke in ctx->Nodes to find a suitable id faster. Even more so trivial that ctx->Nodes lookup is already sorted.
    // Id32 id = 0x0001;
    let mut id = INVALID_IDx0001;
    while dock_context_find_node_by_id(g, id).is_some() {
        id += 1;
    }
    return id;
}

// static ImGuiDockNode* ImGui::DockContextAddNode(ImGuiContext* ctx, Id32 id)
pub fn dock_context_add_node(g: &mut Context, mut id: Id32) -> &mut DockNode {
    // Generate an id for the new node (the exact id value doesn't matter as long as it is not already used) and add the first window.
    // ImGuiContext& g = *ctx;
    if id == INVALID_ID {
        id = dock_context_gen_node_id(g);
    }
    // else
    // IM_ASSERT(DockContextFindNodeByID(ctx, id) == None);

    // We don't set node->last_frame_alive on construction. Nodes are always created at all time to reflect .ini settings!
    // IMGUI_DEBUG_LOG_DOCKING("[docking] DockContextAddNode 0x%08X\n", id);
    // ImGuiDockNode* node = IM_NEW(ImGuiDockNode)(id);
    let mut node = DockNode::new(id);
    // ctx.DockContext.Nodes.SetVoidPtr(node.ID, node);
    g.dock_context.nodes.push(node.id);
    return &mut node;
}

// Pre C++0x doesn't allow us to use a function-local type (without linkage) as template parameter, so we moved this here.
#[derive(Default, Debug, Clone)]
pub struct DockContextPruneNodeData {
    // int         CountWindows, CountChildWindows, CountChildNodes;
    pub count_windows: i32,
    pub count_child_windows: i32,
    pub count_child_nodes: i32,
    // Id32     RootId;
    pub root_id: Id32, // ImGuiDockContextPruneNodeData() { CountWindows = CountChildWindows = CountChildNodes = 0; RootId = 0; }
}

// Garbage collect unused nodes (run once at init time)
// static void dock_context_prune_unused_settings_nodes(ImGuiContext* ctx)
pub fn dock_context_prune_unused_settings_nodes(g: &mut Context) {
    // ImGuiContext& g = *ctx;
    // ImGuiDockContext* dc  = &ctx.DockContext;
    let mut dc = &mut g.dock_context;
    // IM_ASSERT(g.windows.len() == 0);

    // ImPool<ImGuiDockContextPruneNodeData> pool;
    let mut pool: HashMap<Id32, DockContextPruneNodeData> = HashMap::new();

    // Count child nodes and compute RootID
    // for (int settings_n = 0; settings_n < dc.nodes_settings.size; settings_n += 1)
    for settings_n in 0..dc.nodes_settings.len() {
        // ImGuiDockNodeSettings* settings = &dc.nodes_settings[settings_n];
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
            // pool.GetOrAddByKey(settings.parent_node_id).countChildNodes += 1;
            pool_val.count_child_nodes += 1;
        }
    }

    // Count reference to dock ids from dockspaces
    // We track the 'auto-dock_node <- manual-window <- manual-DockSpace' in order to avoid 'auto-dock_node' being ditched by dock_context_prune_unused_settings_nodes()
    // for (int settings_n = 0; settings_n < dc.nodes_settings.size; settings_n += 1)
    for settings_n in 0..dc.nodes_settings.len() {
        // ImGuiDockNodeSettings* settings = &dc.nodes_settings[settings_n];
        let settings = &mut dc.nodes_settings[settings_n];
        // if (settings.ParentWindowId != 0)
        if settings.parent_window_id != INVALID_ID {
            let window_settings = find_window_settings(g, settings.parent_window_id);
            // if (WindowSettings * window_settings = FindWindowSettings(settings.ParentWindowId))
            if window_settings.id != INVALID_ID {
                // if (window_settings.dock_id)
                if window_settings.dock_id != INVALID_ID {
                    // if (ImGuiDockContextPruneNodeData * data = pool.GetByKey(window_settings.dock_id))
                    let data = pool.get_mut(&window_settings.dock_id);
                    if data.is_some() {
                        // data.countChildNodes += 1;
                        data.unwrap().count_child_nodes += 1;
                    }
                }
            }
        }
    }

    // Count reference to dock ids from window settings
    // We guard against the possibility of an invalid .ini file (RootID may point to a missing node)
    // for (WindowSettings* settings = g.settings_windows.begin(); settings != None; settings = g.settings_windows.next_chunk(settings))
    for settings in g.settings_windows.iter_mut() {
        let dock_id = settings.dock_id;
        // if (Id32 dock_id = settings.dock_id){
        if dock_id != INVALID_ID {
            // if (ImGuiDockContextPruneNodeData * data = pool.GetByKey(dock_id)) {
            let data = pool.get_mut(&dock_id);
            if data.is_some() {
                data.unwrap().count_windows += 1;
                // if (ImGuiDockContextPruneNodeData * data_root = (data.RootId == dock_id)? data: pool.GetByKey(data.RootId)){
                let data_root = if data.unwrap().root_id == dock_id {
                    data
                } else {
                    pool.get_mut(&data.unwrap().root_id)
                };
                if data_root.is_some() {
                    data_root.unwrap().child_count_windows += 1;
                }
            }
        }
    }

    // Prune
    // for (int settings_n = 0; settings_n < dc.nodes_settings.size; settings_n += 1)
    for settings_n in 0..dc.nodes_settings.len() {
        // ImGuiDockNodeSettings* settings = &dc.nodes_settings[settings_n];
        let settings = &mut dc.nodes_settings[settings_n];
        // ImGuiDockContextPruneNodeData* data = pool.GetByKey(settings.ID);
        let data = pool.get_mut(&settings.id);
        if data.unwrap().count_windows > 1 {
            continue;
        }
        // ImGuiDockContextPruneNodeData* data_root = (data.RootId == settings.ID) ? data : pool.GetByKey(data.RootId);
        let data_root = if data.unwrap().root_id == settings.id {
            data
        } else {
            pool.get_mut(&data.unwrap().root_id)
        };

        let mut remove = false;
        remove |= (data.count_windows == 1
            && settings.parent_node_id == 0
            && data.count_child_nodes == 0
            && !(settings.flags.contains(&DockNodeFlags::CentralNode))); // Floating root node with only 1 window
        remove |= (data.count_windows == 0
            && settings.parent_node_id == 0
            && data.count_child_nodes == 0); // Leaf nodes with 0 window
        remove |= (data_root.count_child_windows == 0);
        if remove {
            // IMGUI_DEBUG_LOG_DOCKING("[docking] dock_context_prune_unused_settings_nodes: Prune 0x%08X\n", settings.ID);
            settings::dock_settings_remove_node_references(g, &mut settings.id, 1);
            settings.id = INVALID_ID;
        }
    }
}

// static void dock_context_build_nodes_from_settings(ImGuiContext* ctx, ImGuiDockNodeSettings* node_settings_array, int node_settings_count)
pub fn dock_context_buld_nodes_from_settings(
    g: &mut Context,
    node_settings_array: &mut Vec<DockNodeSettings>,
    node_settings_count: i32,
) {
    // build nodes
    // for (int node_n = 0; node_n < node_settings_count; node_n += 1)
    for node_n in 0..node_settings_count {
        // ImGuiDockNodeSettings* settings = &node_settings_array[node_n];
        let settings = node_settings_array.get_mut(node_n).unwrap();

        if settings.id == INVALID_ID {
            continue;
        }
        // ImGuiDockNode* node = DockContextAddNode(ctx, settings.id);
        let mut node = dock_context_add_node(g, settings.id);
        // node.ParentNode = settings.parent_node_id ? dock_context_find_node_by_id(ctx, settings.parent_node_id) : None;
        node.parent_node_id = settings.parent_node_id;
        node.pos = Vector2D::new(settings.pos.x, settings.pos.y);
        node.size = Vector2D::new(settings.size.x, settings.size.y);
        node.size_ref = Vector2D::new(settings.size_ref.x, settings.size_ref.y);
        node.authority_for_pos = DataAuthority::DockNode;
        node.authority_for_size = DataAuthority::DockNode;
        node.authority_for_viewport = DataAuthority::DockNode;
        if node.parent_node && node.parent_node.child_nodes[0] == None {
            node.parent_node.child_nodes[0] = node;
        } else if node.parent_node && node.parent_node.child_nodes[1] == None {
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
        node.host_window_id = find_window_by_name(
            g,
            dock_node_get_host_window_title(
                root_node,
                host_window_title,
                (host_window_title.len()),
            ),
        )
        .unwrap()
        .id;
    }
}

// void dock_context_build_add_windows_to_nodes(ImGuiContext* ctx, Id32 root_id)
pub fn dock_context_build_add_windows_to_nodes(g: &mut Context, root_id: Id32) {
    // Rebind all windows to nodes (they can also lazily rebind but we'll have a visible glitch during the first frame)
    // ImGuiContext& g = *ctx;
    // for (int n = 0; n < g.windows.len(); n += 1)
    // for n in 0 .. g.windows.len()
    for (_, window) in g.windows.iter_mut() {
        // Window* window = g.windows[n];
        // let window = g.windows.get_mut(n).unwrap();
        if window.dock_id == INVALID_ID || window.last_frame_active < g.frame_count - 1 {
            continue;
        }
        if window.dock_node_id != INVALID_ID {
            continue;
        }

        // ImGuiDockNode* node = dock_context_find_node_by_id(ctx, window.dock_id);
        let node = dock_context_find_node_by_id(g, window.dock_id);
        // IM_ASSERT(node != None);   // This should have been called after dock_context_build_nodes_from_settings()
        if root_id == INVALID_ID || dock_node_get_root_node(g, node.unwrap()).id == root_id {
            window::dock_node_add_window(g, node.unwrap(), window, true);
        }
    }
}

// void DockContextQueueDock(ImGuiContext* ctx, Window* target, ImGuiDockNode* target_node, Window* payload, ImGuiDir split_dir, float split_ratio, bool split_outer)
pub fn dock_context_queue_dock(
    g: &mut Context,
    target: &mut Window,
    target_node: &mut DockNode,
    payload: &mut window::Window,
    split_dir: Direction,
    split_ratio: f32,
    split_outer: bool,
) {
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

// void DockContextQueueUndockWindow(ImGuiContext* ctx, Window* window)
pub fn dock_context_queue_undock_window(g: &mut Context, window: &mut Window) {
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
pub fn dock_context_queue_undock_node(g: &mut Context, node: &mut DockNode) {
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
pub fn dock_context_queue_notify_remove_node(g: &mut Context, node: &mut DockNode) {
    // ImGuiDockContext* dc  = &ctx.dock_context;
    let dc = &mut g.dock_context;
    // for (int n = 0; n < dc.requests.size; n += 1
    for req in dc.requests.iter_mut() {
        // if (dc.requests[n].dock_target_node == node)
        if req.dock_target_node_id == node.id {
            // dc.requests[n].Type = DockRequestType::None;
            req.request_type = DockRequestType::None;
        }
    }
}

// void dock_context_process_dock(ImGuiContext* ctx, ImGuiDockRequest* req)
pub fn dock_context_process_dock(g: &mut Context, req: &mut DockRequest) {
    // IM_ASSERT((req.Type == DockRequestType::Dock && req.DockPayload != None) || (req.Type == DockRequestType::Split && req.DockPayload == None));
    // IM_ASSERT(req.DockTargetWindow != None || req.DockTargetNode != None);

    // ImGuiContext& g = *ctx;
    // IM_UNUSED(g);

    // Window* payload_window = req.dock_payload_id;     // Optional
    let payload_window_id = req.dock_payload_id;
    let payload_window = g.window_mut(payload_window_id);
    // Window* target_window = req.dock_target_window_id;
    let target_window_id = req.dock_target_window_id;
    let target_window = g.window_mut(target_window_id);
    // ImGuiDockNode* node = req.dock_target_node_id;
    let node_id = req.dock_target_node_id;
    let mut node = g.dock_node_mut(node_id);

    if payload_window.id != INVALID_ID {
    }
    // IMGUI_DEBUG_LOG_DOCKING("[docking] dock_context_process_dock node 0x%08X target '%s' dock window '%s', split_dir %d\n", node ? node.ID : 0, target_window ? target_window.Name : "None", payload_window ? payload_window.Name : "None", req.DockSplitDir);
    else {
    }
    // IMGUI_DEBUG_LOG_DOCKING("[docking] dock_context_process_dock node 0x%08X, split_dir %d\n", node ? node.ID : 0, req.DockSplitDir);

    // Decide which Tab will be selected at the end of the operation
    // Id32 next_selected_id = 0;
    let mut next_selected_id: Id32 = 0;
    // ImGuiDockNode* payload_node = None;
    let mut payload_node: &mut DockNode = &mut Default::default();
    if payload_window.id != INVALID_ID {
        payload_node = g
            .dock_node_mut(payload_window.dock_node_as_host_id)
            .unwrap();
        payload_window.dock_node_as_host_id = INVALID_ID; // Important to clear this as the node will have its life as a child which might be merged/deleted later.
        if payload_node.is_some() && payload_node.unwrap().is_leaf_node() {
            next_selected_id = if payload_node.tab_bar.next_selected_tab_id != INVALID_ID {
                payload_node.tab_bar.next_selected_tab_id
            } else {
                payload_node.tab_bar.selected_tab_id
            };
        }
        if payload_node.is_none() {
            next_selected_id = payload_window.tab_id;
        }
    }

    // FIXME-DOCK: When we are trying to dock an existing single-window node into a loose window, transfer Node id as well
    // When processing an interactive split, usually last_frame_alive will be < g.frame_count. But DockBuilder operations can make it ==.
    if node.is_some() {}
    // IM_ASSERT(node.LastFrameAlive <= g.frame_count);
    if node.is_some()
        && target_window.id != INVALID_ID
        && node.unwrap().id == target_window.dock_node_as_host_id
    {}
    // IM_ASSERT(node.Windows.size > 0 || node.IsSplitNode() || node.IsCentralNode());

    // Create new node and add existing window to it
    if node.is_none() {
        node = Some(dock_context_add_node(g, 0));
        node.unwrap().pos = target_window.pos.clone();
        node.unwrap().size = target_window.size.clone();
        if target_window.dock_node_as_host_id == INVALID_ID {
            window::dock_node_add_window(g, node.unwrap(), target_window, true);
            node.unwrap().tab_bar.tabs[0]
                .flags
                .remove(TabItemFlags::Unsorted);
            target_window.dock_is_active = true;
        }
    }

    let split_dir = req.dock_split_dir.clone();
    if split_dir != Direction::None {
        // split into two, one side will be our payload node unless we are dropping a loose window
        // const ImGuiAxis split_axis = (split_dir == Direction::Left || split_dir == Direction::Right) ? Axis::X : Axis::Y;
        let split_axis = if split_dir == Direction::Left || split_dir == Direction::Right {
            Axis::X
        } else {
            Axis::Y
        };

        // let split_inheritor_child_idx = (split_dir == Direction::Left || split_dir == Direction::Up) ? 1 : 0; // Current contents will be moved to the opposite side
        let split_inheritor_child_idx =
            if split_dir == Direction::Left || split_dir == Direction::Up {
                1
            } else {
                0
            };

        // let split_ratio = req.dock_split_ratio;
        let split_ratio = req.dock_split_ratio;
        // let mut payload_node: &mut DockNode;
        dock_node_tree_split(
            g,
            node.unwrap(),
            split_axis,
            split_inheritor_child_idx,
            split_ratio,
            payload_node,
        ); // payload_node may be None here!
           // ImGuiDockNode* new_node = node.child_nodes[split_inheritor_child_idx ^ 1];
        let new_node = node.unwrap().child_nodes[split_inheritor_child_idx ^ 1];
        new_node.host_window = node.unwrap().host_window;
        node = Some(new_node);
    }
    // node.set_local_flags(node.LocalFlags & ~DockNodeFlags::HiddenTabBar);
    node.unwrap()
        .local_flags
        .remove(&DockNodeFlags::HiddenTabBar);

    if node.unwrap().id != payload_node.id {
        // Create tab bar before we call DockNodeMoveWindows (which would attempt to move the old tab-bar, which would lead us to payload tabs wrongly appearing before target tabs!)
        if node.unwrap().windows.len() > 0 && node.unwrap().tab_bar.is_none() {
            dock_node_add_tab_bar(g, node.unwrap());
            // for (int n = 0; n < node.windows.len(); n += 1)
            for win_id in node.unwrap().windows.iter_mut() {
                let win = g.window_mut(*win_id).unwrap();
                tab_bar_add_tab(
                    &mut node.unwrap().tab_bar.unwrwap(),
                    TabItemFlags::None,
                    win,
                );
            }
        }

        if payload_node.id != INVALID_ID {
            // Transfer full payload node (with 1+ child windows or child nodes)
            if payload_node.is_split_node() {
                if node.unwrap().windows.len() > 0 {
                    // We can dock a split payload into a node that already has windows _only_ if our payload is a node tree with a single visible node.
                    // In this situation, we move the windows of the target node into the currently visible node of the payload.
                    // This allows us to preserve some of the underlying dock tree settings nicely.
                    // IM_ASSERT(payload_node.only_node_with_windows != None); // The docking should have been blocked by dock_node_preview_dock_setup() early on and never submitted.
                    // ImGuiDockNode* visible_node = payload_node.only_node_with_windows;
                    let visible_node = g
                        .dock_node_mut(payload_node.only_node_with_window_id)
                        .unwrap();
                    if visible_node.tab_bar.is_some() {}
                    // IM_ASSERT(visible_node.TabBar.Tabs.size > 0);
                    window::dock_node_move_windows(g, node.unwrap(), visible_node);
                    window::dock_node_move_windows(g, visible_node, node.unwrap());
                    settings::dock_settings_rename_node_references(
                        g,
                        node.unwrap().id,
                        visible_node.id,
                    );
                }
                if node.is_central_node() {
                    // Central node property needs to be moved to a leaf node, pick the last focused one.
                    // FIXME-DOCK: If we had to transfer other flags here, what would the policy be?
                    // ImGuiDockNode* last_focused_node = dock_context_find_node_by_id(ctx, payload_node.last_focused_node_id);
                    let last_focused_node =
                        dock_context_find_node_by_id(g, payload_node.last_focused_node_id);
                    // IM_ASSERT(last_focused_node != None);
                    // ImGuiDockNode* last_focused_root_node = dock_node_get_root_node(g, last_focused_node);
                    let last_focused_root_node = dock_node_get_root_node(g, g.last_focused_node);
                    // IM_ASSERT(last_focused_root_node == DockNodeGetRootNode(payload_node));
                    // last_focused_node.set_local_flags(last_focused_node.LocalFlags | DockNodeFlags::CentralNode);
                    let mut flags_to_add = HashSet::from([DockNodeFlags::CentralNode]);
                    flags_to_add =
                        add_hash_set(&flags_to_add, &last_focused_node.unwrap().local_flags);
                    last_focused_node.unwrap().set_local_flags(&flags_to_add);
                    flags_to_add.clone_from(&node.local_flags);
                    flags_to_add.remove(&DockNodeFlags::CentralNode);
                    node.set_local_flags(flags_to_add);
                    last_focused_root_node.central_node_id = last_focused_node.id;
                }

                // IM_ASSERT(node.Windows.size == 0);
                node::dock_node_move_child_nodes(g, node.unwrap(), payload_node);
            } else {
                // const Id32 payload_dock_id = payload_node.id;
                let payload_dock_id = payload_node.id;
                window::dock_node_move_windows(g, node.unwrap(), payload_node);
                settings::dock_settings_rename_node_references(g, payload_dock_id, node.id);
            }
            dock_context_remove_node(g, payload_node, true);
        } else if payload_window {
            // Transfer single window
            // const Id32 payload_dock_id = payload_window.dock_id;
            let payload_dock_id = payload_window.dock_id;
            node.unwrap().visible_window_id = payload_window.id;
            window::dock_node_add_window(g, node.unwrap(), payload_window, true);
            if payload_dock_id != 0 {
                settings::dock_settings_rename_node_references(
                    g,
                    payload_dock_id,
                    node.unwrap().id,
                );
            }
        }
    } else {
        // When docking a floating single window node we want to reevaluate auto-hiding of the tab bar
        node.unwrap().want_hiddent_tab_bar_update = true;
    }

    // Update selection immediately
    // if ImGuiTabBar* tab_bar = node.tab_bar {
    let mut tab_bar = &mut node.unwrap().tab_bar;
    if tab_bar.is_some() {
        tab_bar.unwrap().next_selected_tab_id = next_selected_id;
    }
    mark_ini_settings_dirty(g);
}

// void DockContextProcessUndockWindow(ImGuiContext* ctx, Window* window, bool clear_persistent_docking_ref)
pub fn dock_context_process_undock_window(
    g: &mut Context,
    window: &mut Window,
    clear_persistent_docking_ref: bool,
) {
    // ImGuiContext& g = *.g;
    // IMGUI_DEBUG_LOG_DOCKING("[docking] DockContextProcessUndockWindow window '%s', clear_persistent_docking_ref = %d\n", window.Name, clear_persistent_docking_ref);
    // if (window.dock_node)
    if window.dock_node_id != INVALID_ID {
        let win_dock_node = g.dock_node_mut(window.dock_node_id);
        window::dock_node_remove_window(
            g,
            win_dock_node.unwrap(),
            window,
            if clear_persistent_docking_ref {
                0
            } else {
                window.dock_id
            },
        );
    } else {
        window.dock_id = INVALID_ID;
    }
    window.collapsed = false;
    window.dock_is_active = false;
    window.dock_node_is_visible = false;
    window.dock_tab_is_visible = false;
    let ref_vp = g.viewport_mut(window.viewport_id);
    window.size_full = ops::fix_large_windows_when_undocking(g, &window.size_full, ref_vp);
    window.size = window.size_full.clone();

    mark_ini_settings_dirty(g);
}

// void DockContextProcessUndockNode(ImGuiContext* ctx, ImGuiDockNode* node)
pub fn dock_context_process_undock_node(g: &mut Context, node: &mut DockNode) {
    // ImGuiContext& g = *.g;
    // IMGUI_DEBUG_LOG_DOCKING("[docking] DockContextProcessUndockNode node %08X\n", node.ID);
    // IM_ASSERT(node.IsLeafNode());
    // IM_ASSERT(node.Windows.size >= 1);

    if node.is_root_node() || node.is_central_node() {
        // In the case of a root node or central node, the node will have to stay in place. Create a new node to receive the payload.
        // ImGuiDockNode* new_node = dock_context_add_node(.g, 0);
        let new_node = dock_context_add_node(g, INVALID_ID);
        new_node.pos = node.pos.clone();
        new_node.size = node.size.clone();
        new_node.size_ref = node.size_ref.clone();
        window::dock_node_move_windows(g, new_node, node);
        settings::dock_settings_rename_node_references(g, node.id, new_node.id);
        // for (int n = 0; n < new_node.windows.len(); n += 1)
        for win_id in new_node.windows.iter() {
            // Window* window = new_node.windows[n];
            let mut win = g.window_mut(*win_id);
            window.flags.remove(WindowFlags::ChildWindow);
            if window.parent_window_id != INVALID_ID {
                let parent_win = g.window_mut(window.parent_window_id);
                // window.parent_window.DC.ChildWindows.find_erase(window);
                parent_win.dc.child_windows.retain(|x| x != window.id);
            }
            // update_window_parent_and_root_links(window, window.flags, None);
            update_window_parent_and_root_links(g, window, window.flags, None);
        }
        node.clone_from(&new_node);
    } else {
        // Otherwise extract our node and merge our sibling back into the parent node.
        // IM_ASSERT(node.ParentNode.ChildNodes[0] == node || node.ParentNode.ChildNodes[1] == node);
        // int index_in_parent = (node.parent_node.child_nodes[0] == node) ? 0 : 1;
        let parent_node = g.dock_node_mut(node.parent_node_id);
        let index_in_parent = if parent_node.unwrap().child_nodes[0] == node.id {
            0
        } else {
            1
        };
        // node.parent_node.child_nodes[index_in_parent] = None;
        parent_node.unwrap().child_nodes[index_in_parent] = INVALID_ID;
        dock_node_tree_merge(
            g,
            parent_node.unwrap(),
            g.dock_node_mut(parent_node.unwrap().child_nodes[index_in_parent ^ 1]),
        );
        // node.parent_node.authority_for_viewport = DataAuthority::Window; // The node that stays in place keeps the viewport, so our newly dragged out node will create a new viewport
        parent_node.unwrap().authority_for_viewport = DataAuthority::Window;
        // node.parent_node = None;
        node.parent_node_id = INVALID_ID;
    }

    node.authority_for_pos = DataAuthority::DockNode;
    node.authority_for_size = DataAuthority::DockNode;
    let node_win_0 = g.window_mut(node.windows[0]);
    let node_win_0_vp = g.viewport_mut(node_win_0.viewport_id);
    node.size = ops::fix_large_windows_when_undocking(g, &node.size, node_win_0_vp);
    nodewant_mouse_move = true;
    mark_ini_settings_dirty(g);
}

// This is mostly used for automation.
// bool DockContextCalcDropPosForDocking(Window* target, ImGuiDockNode* target_node, Window* payload, ImGuiDir split_dir, bool split_outer, Vector2D* out_pos)
pub fn dock_context_calc_drop_pos_for_docking(
    g: &mut Context,
    target: &mut Window,
    target_node: Option<&mut DockNode>,
    payload: &mut Window,
    split_dir: Direction,
    mut split_outer: bool,
    out_pos: &mut Vector2D,
) -> bool {
    // In dock_node_preview_dock_setup() for a root central node instead of showing both "inner" and "outer" drop rects
    // (which would be functionally identical) we only show the outer one. Reflect this here.
    // if (target_node && target_node.parent_node == None && target_node.is_central_node() && split_dir != Direction::None)
    if target_node.is_some()
        && target_node.unwrap().parent_node_id != INVALID_ID
        && split_idr != Direction::None
    {
        split_outer = true;
    }
    // DockPreviewData split_data;
    let mut split_data = DockPreviewData::default();
    dock_node_preview_dock_setup(
        g,
        target,
        target_node,
        payload,
        &mut split_data,
        false,
        split_outer,
    );
    if split_data.drop_rects_draw[&split_dir + 1].is_inverted() {
        return false;
    }
    *out_pos = split_data.drop_rects_draw[&split_dir + 1]
        .get_center()
        .clone();
    return true;
}

// static void ImGui::dock_context_remove_node(ImGuiContext* ctx, ImGuiDockNode* node, bool merge_sibling_into_parent_node)
pub fn dock_context_remove_node(
    g: &mut Context,
    node: &mut DockNode,
    merge_sibling_into_parent_node: bool,
) {
    // ImGuiContext& g = *ctx;
    // ImGuiDockContext* dc  = &ctx.DockContext;
    let dc = &mut g.dock_context;

    // IMGUI_DEBUG_LOG_DOCKING("[docking] dock_context_remove_node 0x%08X\n", node.ID);
    // IM_ASSERT(DockContextFindNodeByID(ctx, node.id) == node);
    // IM_ASSERT(node.ChildNodes[0] == None && node.ChildNodes[1] == None);
    // IM_ASSERT(node.Windows.size == 0);

    if node.host_window_id != INVALID_ID {
        let win = g.window_mut(node.host_window_id);
        // node.host_window.dock_node_as_host = None;
        win.dock_node_as_host_id = INVALID_ID;
    }

    // ImGuiDockNode* parent_node = node.ParentNode;
    let parent_node = g.dock_node_mut(node.parent_node_id);

    // const bool merge = (merge_sibling_into_parent_node && parent_node != None);
    let merge = merge_sibling_into_parent_node && parent_node.is_some();
    let parent_node_obj = parent_node.unwrap();
    if parent_node.is_some() {
        if merge {
            // IM_ASSERT(parent_node.ChildNodes[0] == node || parent_node.ChildNodes[1] == node);
            // ImGuiDockNode* sibling_node = (parent_node.ChildNodes[0] == node ? parent_node.ChildNodes[1] : parent_node.ChildNodes[0]);
            let sibling_node_id = if parent_node_obj.child_nodes[0] == node.id {
                parent_node_obj.child_nodes[1]
            } else {
                parent_node.child_nodes[0]
            };
            let sibling_node = g.dock_node_mut(sibling_node_id);
            dock_node_tree_merge(g, parent_node_obj, sibling_node);
        } else {
            // for (int n = 0; parent_node && n < IM_ARRAYSIZE(parent_node.ChildNodes); n += 1)
            // if (parent_node.ChildNodes[n] == node) {
            //    parent_node_obj.child_nodes.remove()

            // }

            parent_node_obj
                .child_nodes
                .retain(|child_node| child_node != node.id);

            // dc.Nodes.SetVoidPtr(node.ID, None);
            dc.nodes.retain(|x| x != node.id);
            // IM_DELETE(node);
            g.dock_nodes.retain(|dn| dn != node.id);
        }
    }
}

// static ImGuiDockNode* dock_context_bind_node_to_window(ImGuiContext* ctx, Window* window)
pub fn dock_context_bind_node_to_window(
    g: &mut Context,
    window: &mut window::Window,
) -> Option<&mut DockNode> {
    // ImGuiContext& g = *.g;
    // ImGuiDockNode* node = dock_context_find_node_by_id(g, window.dock_id);
    let mut node = dock_context_find_node_by_id(g, window.dock_id);
    // IM_ASSERT(window.dock_node == None);

    // We should not be docking into a split node (SetWindowDock should avoid this)
    if node.is_some() && node.unwrap().is_split_node() {
        dock_context_process_undock_window(g, window, false);
        return None;
    }

    // Create node
    if node.is_none() {
        node = Some(dock_context_add_node(g, window.dock_id));
        node.unwrap().authority_for_pos = DataAuthority::Window;
        node.unwrap().authority_for_size = DataAuthority::Window;
        node.unwrap().authority_for_viewport = DataAuthority::Window;
        node.unwrap().last_frame_alive = g.frame_count;
    }

    // If the node just turned visible and is part of a hierarchy, it doesn't have a size assigned by dock_node_tree_update_pos_size() yet,
    // so we're forcing a pos/size update from the first ancestor that is already visible (often it will be the root node).
    // If we don't do this, the window will be assigned a zero-size on its first frame, which won't ideally warm up the layout.
    // This is a little wonky because we don't normally update the pos/size of visible node mid-frame.
    if !node.unwrap().is_visible {
        let mut ancestor_node = node.unwrap();
        while !ancestor_node.is_visible && ancestor_node.parent_node_id != INVALID_ID {
            let parent_node = g.dock_node_mut(ancestor_node.parent_node_id);
            ancestor_node = parent_node.unwrap();
        }
        // IM_ASSERT(ancestor_node.size.x > 0.0 && ancestor_node.size.y > 0.0);
        dock_node_update_has_central_node_child(g, dock_node_get_root_node(g, ancestor_node).unwrap());
        dock_node_tree_update_pos_size(
            g,
            ancestor_node,
            &ancestor_node.pos,
            &ancestor_node.size,
            node.clone(),
        );
    }

    // Add window to node
    let node_was_visible = node.unwrap().is_visible;
    dock_node_add_window(g, node.unwrap(), window, true);
    node.unwrap().is_visible = node_was_visible; // Don't mark visible right away (so DockContextEndFrame() doesn't render it, maybe other side effects? will see)
                                        // IM_ASSERT(node == window.dock_node);
    return node;
}
