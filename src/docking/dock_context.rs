use crate::docking::dock_node::ImGuiDockNode;
use crate::docking::dock_node_settings::ImGuiDockNodeSettings;
use crate::docking::dock_request::ImGuiDockRequest;
use crate::core::storage::ImGuiStorage;
use crate::core::type_defs::ImguiHandle;
use std::collections::HashMap;

#[derive(Default, Debug, Clone, Copy)]
pub struct ImGuiDockContext {
    pub dock_nodes: HashMap<ImguiHandle, ImGuiDockNode>, //ImGuiStorage, // Map ID -> ImGuiDockNode*: Active nodes
    pub Requests: Vec<ImGuiDockRequest>,
    pub NodesSettings: Vec<ImGuiDockNodeSettings>,
    pub WantFullRebuild: bool,
    // ImGuiDockContext()              { memset(this, 0, sizeof(*this)); }
}

impl ImGuiDockContext {
    pub fn find_node_by_id_mut(&mut self, id: ImguiHandle) -> Option<&mut ImGuiDockNode> {
        self.dock_nodes.get_mut(&id)
    }
}
