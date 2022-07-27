use crate::axis::Axis;
use crate::{Context, INVALID_ID};
use crate::dock::context::dock_context_add_node;
use crate::dock::defines::DOCKING_SPLITTER_SIZE;
use crate::dock::node;
use crate::dock::node::{DockNode, DockNodeFlags};
use crate::dock::node::window::dock_node_move_windows;
use crate::dock::settings::dock_settings_rename_node_references;
use crate::types::DataAuthority;
use crate::vectors::two_d::Vector2D;

// void DockNodeTreeSplit(ImGuiContext* ctx, ImGuiDockNode* parent_node, ImGuiAxis split_axis, int split_inheritor_child_idx, float split_ratio, ImGuiDockNode* new_node)
pub fn dock_node_tree_split(g: &mut Context,
                            parent_node: &mut DockNode,
                            split_axis: Axis,
                            split_inheritor_child_idx: i32,
                            split_ratio: f32,
                            new_node: &mut DockNode)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(split_axis != ImGuiAxis_None);

    let mut child_0 = if new_node.id != INVALID_ID && split_inheritor_child_idx != 0 { new_node } else { dock_context_add_node(g, 0) };
    child_0.parent_node_id = parent_node.id;

    let mut child_1 = if new_node.id != INVALID_ID && split_inheritor_child_idx != 1 { new_node } else { dock_context_add_node(g, 0) };
    child_1.parent_node = parent_node;

    let mut child_inheritor = if split_inheritor_child_idx == 0 { child_0 } else { child_1 };
    node::dock_node_move_child_nodes(g, child_inheritor, parent_node);
    parent_node.child_nodes[0] = child_0.id;
    parent_node.child_nodes[1] = child_1.id;
    parent_node.child_nodes[split_inheritor_child_idx].visible_window = parent_node.visible_window_id;
    parent_node.split_axis = split_axis;
    parent_node.visible_window_id = INVALID_ID;
    parent_node.authority_for_pos = DataAuthority::DockNode;
    parent_node.authority_for_size = DataAuthority::DockNode;

    let mut size_avail = (parent_node.size[&split_axis] - DOCKING_SPLITTER_SIZE);
    size_avail = Vector2D::max(size_avail, g.style.window_min_size[&split_axis] * 2.0);
    // IM_ASSERT(size_avail > 0.0); // If you created a node manually with DockBuilderAddNode(), you need to also call DockBuilderSetNodeSize() before splitting.
    child_0.size_ref = parent_node.size.clone();
    child_1.size_ref = parent_node.size.clone();
    child_0.size_ref[&split_axis] = f32::floor(size_avail * split_ratio);
    child_1.size_ref[&split_axis] = f32::floor(size_avail - child_0.size_ref[&split_axis]);

    dock_node_move_windows(g, parent_node.child_nodes[split_inheritor_child_idx], parent_node);
    dock_settings_rename_node_references(g, parent_node.id, parent_node.child_nodes[split_inheritor_child_idx].id);
    node::dock_node_update_has_central_node_child(g, node::dock_node_get_root_node(g, parent_node));
    node::dock_node_tree_update_pos_size(g, parent_node, &parent_node.pos, &parent_node.size, None);

    // flags transfer (e.g. this is where we transfer the ImGuiDockNodeFlags_CentralNode property)
    child_0.shared_flags = parent_node.shared_flags.clone();
    child_0.sahred_flags.insert(DockNodeFlags::SharedFlagsInheritMask);
    child_1.shared_flags = parent_node.shared_flags.clone();
    child_1.shared_flags.insert(DockNodeFlags::SharedFlagsInheritMask);
    child_inheritor.local_flags = parent_node.local_flags.clone();
    child_inheritor.local_flags.insert(DockNodeFlags::LocalFlagsTransferMask);
    parent_node.local_flags.insert(DockNodeFlags::LocalFlagsTransferMask);
    child_0.update_merged_flags();
    child_1.update_merged_flags();
    parent_node.update_merged_flags();
    if child_inheritor.is_central_node() {
        node::dock_node_get_root_node(g, parent_node).central_node_id = child_inheritor.id;
    }
}
