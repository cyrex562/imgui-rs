use std::ptr::null_mut;
use crate::dock_node::ImGuiDockNode;
use crate::dock_node_ops::DockNodeGetRootNode;
use crate::type_defs::ImGuiID;

// inline * mut ImGuiDockNode   DockBuilderGetCentralNode(ImGuiID node_id)
pub fn DockBuilderGetCentralNode(node_id: ImGuiID) -> *mut ImGuiDockNode {
    let mut node = DockBuilderGetNode(node_id);
    if !node { return null_mut(); }
    return DockNodeGetRootNode(node).CentralNode;
}
