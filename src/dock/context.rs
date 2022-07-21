use std::collections::HashMap;
use crate::config::ConfigFlags;
use crate::context::Context;
use crate::dock::{DimgDockRequestType, dock_builder_remove_node_child_nodes, dock_builder_remove_node_docked_windows, DOCKING_SPLITTER_SIZE, DockRequest, ImGuiDockNode};
use crate::dock::node::{DockNode, DockNodeSettings};
use crate::frame::get_frame_height;
use crate::{dock, INVALID_ID};
use crate::rect::Rect;
use crate::settings::SettingsHandler;
use crate::types::Id32;
use crate::vectors::two_d::Vector2D;


#[derive(Default,Debug,Clone)]
pub struct DockContext {
    //ImGuiStorage                    Nodes;          // Map id -> ImGuiDockNode*: active nodes
    pub nodes: HashMap<Id32, DockNode>,
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
        if req.Type == DimgDockRequestType::Undock && req.undock_target_window {
            dock_context_process_undock_window(ctx, req.undock_target_window);
        }
        else if req.requst_type == DimgDockRequestType::Undock && req.undock_target_node {
            dock_context_process_undock_node(ctx, req.undock_target_node);
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
        if node.LastFrameActive == g.frame_count && node.IsVisible && node.host_window && node.IsLeafNode() && ! node.is_bg_drawn_this_frame {
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
        if hovered_window.dock_node_as_host.is_some() {
            g.hovered_dock_node = dock_node_tree_find_visible_node_by_pos(&hovered_window.dock_node_as_host.unwrap(), &g.io.mouse_pos);
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
