use crate::dock_node_settings::ImGuiDockNodeSettings;
use crate::dock_request::ImGuiDockRequest;
use crate::storage::ImGuiStorage;

#[derive(Default,Debug,Clone)]
pub struct ImGuiDockContext
{
pub Nodes:  ImGuiStorage,          // Map ID -> ImGuiDockNode*: Active nodes
pub Requests:  Vec<ImGuiDockRequest>,
pub NodesSettings:  Vec<ImGuiDockNodeSettings>,
pub WantFullRebuild:  bool,
    // ImGuiDockContext()              { memset(this, 0, sizeof(*this)); }
}
