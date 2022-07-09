use crate::config::DimgConfigFlags;
use crate::context::DimgContext;
use crate::dock::{DimgDockRequest, DimgDockRequestType};
use crate::dock_node::{DimgDockNode, DimgDockNodeSettings};
use crate::settings::DimgSettingsHandler;
use crate::types::DimgId;


#[derive(Default,Debug,Clone)]
pub struct DimgDockContext {
    //ImGuiStorage                    Nodes;          // Map id -> ImGuiDockNode*: active nodes
    pub nodes: Vec<DimgDockNode>,
    // ImVector<ImGuiDockRequest>      Requests;
    pub requests: Vec<DimgDockRequest>,
    // ImVector<ImGuiDockNodeSettings> NodesSettings;
    pub nodes_settings: Vec<DimgDockNodeSettings>,
    // bool                            WantFullRebuild;
    pub want_full_rebuild: bool,
    // ImGuiDockContext()              { memset(this, 0, sizeof(*this)); }
}


impl DimgDockContext {


}

pub fn dock_ctx_initialize(ctx: &mut DimgContext)
    {
    // ImGuiContext& g = *ctx;

    // Add .ini handle for persistent docking data
    // ImGuiSettingsHandler ini_handler;
        let mut ini_handler = DimgSettingsHandler {
            type_name: String::from("Docking"),
            type_hash: DimgHashStr(String::from("Docking")),
            clear_all_fn: DockSettingsHandler_ClearAll,
            read_init_fn: DockSettingsHandler_ClearAll, // Also clear on read
            read_open_fn: DockSettingsHandler_ReadOpen,
            read_line_fn: DockSettingsHandler_ReadLine,
            apply_all_fn: DockSettingsHandler_ApplyAll,
            write_all_fn: DockSettingsHandler_WriteAll,
            user_data: vec![]
        };
    ctx.SettingsHandlers.push_back(ini_handler);
}

//void ImGui::DockContextShutdown(ImGuiContext* ctx)
pub fn dock_context_shutdown(ctx: &mut DimgContext)
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
pub fn dock_context_clear_nodes(ctx:&mut DimgContext, root_id: DimgId, clear_settings_refs: bool)
{
    // IM_UNUSED(ctx);
    // IM_ASSERT(ctx == GImGui);
    // DockBuilderRemoveNodeDockedWindows(root_id, clear_settings_refs);
    dock_builder_remove_node_docked_windows(root_id, clear_settings_refs);
    // DockBuilderRemoveNodeChildNodes(root_id);
    dock_build_remove_node_child_nodes(root_id);
}

// [DEBUG] This function also acts as a defacto test to make sure we can rebuild from scratch without a glitch
// (Different from DockSettingsHandler_ClearAll() + DockSettingsHandler_ApplyAll() because this reuses current settings!)
// void ImGui::DockContextRebuildNodes(ImGuiContext* ctx)
pub fn dock_context_rebuild_nodes(ctx: &mut DimgContext)
{
    // ImGuiContext& g = *ctx;
    // ImGuiDockContext* dc = &ctx->DockContext;
    // IMGUI_DEBUG_LOG_DOCKING("[docking] DockContextRebuildNodes\n");
    SaveIniSettingsToMemory();
    // ImGuiID root_id = 0; // Rebuild all
    let mut root_id: DimgId = 0;
    dock_context_clear_nodes(ctx, root_id, false);
    dock_context_build_nodes_from_settings(ctx, &mut ctx.dock_context.nodes_settings);
    dock_context_build_add_windows_to_nodes(ctx, root_id);
}

// Docking context update function, called by NewFrame()
// void ImGui::DockContextNewFrameUpdateUndocking(ImGuiContext* ctx)
pub fn dock_context_new_frame_update_undocking(ctx: &mut DimgContext)
{
    // ImGuiContext& g = *ctx;
    // ImGuiDockContext* dc = &ctx->DockContext;
    let mut dc = &mut ctx.dock_context;
    if !(ctx.io.config_flags.contains(&DimgConfigFlags::DockingEnable))
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
