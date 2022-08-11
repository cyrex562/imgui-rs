use crate::axis::Axis;
use crate::dock::context::dock_context_add_node;
use crate::dock::defines::DOCKING_SPLITTER_SIZE;
use crate::dock::node;
use crate::dock::node::window::{dock_node_apply_pos_size_to_windows, dock_node_move_windows};
use crate::dock::node::dock_node_get_root_node;
use crate::dock::settings::dock_settings_rename_node_references;
use crate::types::{DataAuthority, Id32};
use crate::utils::add_hash_set;
use crate::vectors::vector_2d::Vector2D;
use crate::{Context, INVALID_ID};
use std::ops::BitOr;
use crate::dock::node::dock_node::DockNode;
use crate::dock::node::dock_node_flags::{DOCK_NODE_FLAGS_LOCAL_FLAGS_TRANSFER_MASK, DockNodeFlags};

// void DockNodeTreeSplit(ImGuiContext* ctx, ImGuiDockNode* parent_node, ImGuiAxis split_axis, int split_inheritor_child_idx, float split_ratio, ImGuiDockNode* new_node)
pub fn dock_node_tree_split(
    g: &mut Context,
    parent_node: &mut DockNode,
    split_axis: Axis,
    split_inheritor_child_idx: i32,
    split_ratio: f32,
    new_node: &mut DockNode,
) {
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(split_axis != ImGuiAxis_None);

    let mut child_0 = if new_node.id != INVALID_ID && split_inheritor_child_idx != 0 {
        new_node
    } else {
        dock_context_add_node(g, 0)
    };
    child_0.parent_node_id = parent_node.id;

    let mut child_1 = if new_node.id != INVALID_ID && split_inheritor_child_idx != 1 {
        new_node
    } else {
        dock_context_add_node(g, 0)
    };
    child_1.parent_node = parent_node;

    let mut child_inheritor = if split_inheritor_child_idx == 0 {
        child_0
    } else {
        child_1
    };
    node::dock_node_move_child_nodes(g, child_inheritor, parent_node);
    parent_node.child_nodes[0] = child_0.id;
    parent_node.child_nodes[1] = child_1.id;
    parent_node.child_nodes[split_inheritor_child_idx].visible_window =
        parent_node.visible_window_id;
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

    dock_node_move_windows(
        g,
        parent_node.child_nodes[split_inheritor_child_idx],
        parent_node,
    );
    dock_settings_rename_node_references(
        g,
        parent_node.id,
        parent_node.child_nodes[split_inheritor_child_idx].id,
    );
    node::dock_node_update_has_central_node_child(g, dock_node_get_root_node(g, parent_node).unwrap());
    dock_node_tree_update_pos_size(g, parent_node, &parent_node.pos, &parent_node.size, None);

    // flags transfer (e.g. this is where we transfer the ImGuiDockNodeFlags_CentralNode property)
    child_0.shared_flags = parent_node.shared_flags.clone();
    child_0
        .sahred_flags
        .insert(DockNodeFlags::SharedFlagsInheritMask);
    child_1.shared_flags = parent_node.shared_flags.clone();
    child_1
        .shared_flags
        .insert(DockNodeFlags::SharedFlagsInheritMask);
    child_inheritor.local_flags = parent_node.local_flags.clone();
    child_inheritor
        .local_flags
        .insert(DockNodeFlags::LocalFlagsTransferMask);
    parent_node
        .local_flags
        .insert(DockNodeFlags::LocalFlagsTransferMask);
    child_0.update_merged_flags();
    child_1.update_merged_flags();
    parent_node.update_merged_flags();
    if child_inheritor.is_central_node() {
        dock_node_get_root_node(g, parent_node).central_node_id = child_inheritor.id;
    }
}

// void DockNodeTreeMerge(ImGuiContext* ctx, ImGuiDockNode* parent_node, ImGuiDockNode* merge_lead_child)
pub fn dock_node_tree_merge(
    g: &mut Context,
    parent_node: &mut DockNode,
    merge_lead_child: Option<&mut DockNode>,
) {
    // When called from DockContextProcessUndockNode() it is possible that one of the child is None.
    // ImGuiContext& g = *GImGui;
    let child_0 = g.dock_node_mut(parent_node.child_nodes[0]);
    let child_1 = g.dock_node_mut(parent_node.child_nodes[1]);
    // IM_ASSERT(child_0 || child_1);
    // IM_ASSERT(merge_lead_child == child_0 || merge_lead_child == child_1);
    if (child_0.is_some() && child_0.unwrap().windows.len() > 0)
        || (child_1.is_some() && child_1.unwrap().windows.len() > 0)
    {
        // IM_ASSERT(parent_node.TabBar == None);
        // IM_ASSERT(parent_node.Windows.size == 0);
    }
    // IMGUI_DEBUG_LOG_DOCKING("[docking] DockNodeTreeMerge: 0x%08X + 0x%08X back into parent 0x%08X\n", child_0 ? child_0.id : 0, child_1 ? child_1.id : 0, parent_node.id);

    let backup_last_explicit_size = parent_node.size_ref.clone();
    node::dock_node_move_child_nodes(g, parent_node, merge_lead_child.unwrap());
    if child_0.is_some() {
        dock_node_move_windows(g, parent_node, child_0.unwrap()); // Generally only 1 of the 2 child node will have windows
        dock_settings_rename_node_references(g, child_0.unwrap().id, parent_node.id);
    }
    if child_1.is_some() {
        dock_node_move_windows(g, parent_node, child_1.unwrap());
        dock_settings_rename_node_references(g, child_1.id, parent_node.id);
    }
    dock_node_apply_pos_size_to_windows(g, parent_node);
    parent_node.authority_for_pos = DataAuthority::Auto;
    parent_node.authority_for_size = DataAuthority::Auto;
    parent_node.authority_for_viewport = DataAuthority::Auto;
    parent_node.visible_window_id = merge_lead_child.visible_window;
    parent_node.size_ref = backup_last_explicit_size;

    // flags transfer
    parent_node
        .local_flags
        .remove(DockNodeFlags::LocalFlagsTransferMask); // Preserve Dockspace flag
                                                        // parent_node.local_flags |= (child_0 ? child_0.local_flags : 0) & DockNodeFlags::LocalFlagsTransferMask_;
    if child_0.is_some() {
        parent_node.local_flags =
            add_hash_set(&parent_node.local_flags, &child_0.unwrap().local_flags);
    } else {
        parent_node
            .local_flags
            .insert(DockNodeFlags::LocalFlagsTransferMask);
    }
    // parent_node.local_flags |= (child_1 ? child_1.local_flags : 0) & DockNodeFlags::LocalFlagsTransferMask_;
    if child_1.is_some() {
        parent_node.local_flags = add_hash_set(&parent_node.local_flags, &child_1.unwrap().local_flags);
    }
    for flag in DOCK_NODE_FLAGS_LOCAL_FLAGS_TRANSFER_MASK {
        parent_node.local_flags.insert(flag);
    }
    // parent_node.local_flags.insert(DOCK_NODE_FLAGS_LOCAL_FLAGS_TRANSFER_MASK);
    // parent_node.local_flags_in_windows = (child_0 ? child_0.local_flags_in_windows : 0) | (child_1 ? child_1.local_flags_in_windows : 0); // FIXME: Would be more consistent to update from actual windows
    if child_0.is_some() {
        parent_node.local_flags_in_windows = parent_node
            .local_flags_in_windows
            .bitor(&child_0.unwrap().local_flags_in_windows);
    } else {
        parent_node.local_flags_in_windows.clear();
    }

    if child_1.is_some() {
        parent_node.local_flags_in_windows = parent_node
            .local_flags_in_windows
            .bitor(&child_1.unwrap().local_flags_in_windows);
    } else {
        parent_node.local_flags_in_windows.clear();
    }

    parent_node.update_merged_flags();

    if child_0.is_some() {
        // g.dock_context.Nodes.SetVoidPtr(child_0.id, None);
        // IM_DELETE(child_0);
    }
    if child_1.is_some() {
        // g.dock_context.Nodes.SetVoidPtr(child_1.id, None);
        // IM_DELETE(child_1);
    }
}

// update pos/size for a node hierarchy (don't affect child windows yet)
// (Depth-first, Pre-Order)
// void dock_node_tree_update_pos_size(ImGuiDockNode* node, Vector2D pos, Vector2D size, ImGuiDockNode* only_write_to_single_node)
pub fn dock_node_tree_update_pos_size(
    g: &mut Context,
    node: &mut DockNode,
    pos: &Vector2D,
    size: &Vector2D,
    only_write_to_single_node: Option<&mut DockNode>,
) {
    // During the regular dock node update we write to all nodes.
    // 'only_write_to_single_node' is only set when turning a node visible mid-frame and we need its size right-away.
    let write_to_node =
        only_write_to_single_node.is_none() || only_write_to_single_node.unwrap().id == node.id;
    if write_to_node {
        node.pos = pos.clone();
        node.size = size.clone();
    }

    if node.is_leaf_node() {
        return;
    }

    // ImGuiDockNode* child_0 = node.child_nodes[0];
    let child_0 = g.dock_node_mut(node.child_nodes[0]);
    // ImGuiDockNode* child_1 = node.child_nodes[1];
    let child_1 = g.dock_node_mut(node.child_nodes[1]);
    // Vector2D child_0_pos = pos, child_1_pos = pos;
    let child_0_pos = child_0.unwrap().pos.clone();
    let child_1_pos = child_1.unwrap().pos.clone();
    // Vector2D child_0_size = size, child_1_size = size;
    let child_0_size = child_0.unwrap().size.clone();
    let child_1_size = child_1.unwrap().size_clone();

    let child_0_is_toward_single_node = (only_write_to_single_node.is_some()
        && dock_node_is_in_hierarchy_of(&only_write_to_single_node, child_0));
    let child_1_is_toward_single_node = (only_write_to_single_node.is_some()
        && dock_node_is_in_hierarchy_of(&only_write_to_single_node, child_1));
    let child_0_is_or_will_be_visible =
        child_0.unwrap().is_visible || child_0_is_toward_single_node;
    let child_1_is_or_will_be_visible =
        child_1.unwrap().is_visible || child_1_is_toward_single_node;

    if child_0_is_or_will_be_visible && child_1_is_or_will_be_visible {
        // ImGuiContext& g = *GImGui;
        let spacing = DOCKING_SPLITTER_SIZE;
        let axis = node.split_axis.clone();
        let size_avail = f32::max(size[axis] - spacing, 0.0);

        // size allocation policy
        // 1) The first 0..window_min_size[axis]*2 are allocated evenly to both windows.
        // let size_min_each = f32::floor(ImMin(size_avail, g.style.window_min_size[axis] * 2.0) * 0.5);
        let size_min_each =
            f32::floor(f32::min(size_avail, g.style.window_min_size[&axis] * 2.0) * 0.5);

        // FIXME: Blocks 2) and 3) are essentially doing nearly the same thing.
        // Difference are: write-back to size_ref; application of a minimum size; rounding before f32::floor()
        // Clarify and rework differences between size & size_ref and purpose of want_lock_size_once

        // 2) Process locked absolute size (during a splitter resize we preserve the child of nodes not touching the splitter edge)
        if child_0.unwrap().want_lock_size_once && !child_1.unwrap().want_lock_size_once {
            child_0_size[&axis] = child_0.unwrap().size_ref[&axis] =
                f32::min(size_avail - 1.0, child_0.unwrap().size[&axis]);
            child_1_size[&axis] =
                child_1.unwrap().size_ref[&axis] = (size_avail - child_0_size[&axis]);
            // IM_ASSERT(child_0.sizeRef[axis] > 0.0 && child_1.sizeRef[axis] > 0.0);
        } else if child_1.unwrap().want_lock_size_once && !child_0.unwrap().want_lock_size_once {
            child_1_size[&axis] = child_1.unwrap().size_ref[&axis] =
                f32::min(size_avail - 1.0, child_1.unwrap().size[&axis]);
            child_0_size[&axis] =
                child_0.unwrap().size_ref[&axis] = (size_avail - child_1_size[&axis]);
            // IM_ASSERT(child_0.sizeRef[axis] > 0.0 && child_1.sizeRef[axis] > 0.0);
        } else if child_0.unwrap().want_lock_size_once && child_1.unwrap().want_lock_size_once {
            // FIXME-DOCK: We cannot honor the requested size, so apply ratio.
            // Currently this path will only be taken if code programmatically sets want_lock_size_once
            let mut split_ratio = child_0_size[&axis] / (child_0_size[&axis] + child_1_size[&axis]);
            child_0_size[&axis] =
                child_0.unwrap().size_ref[&axis] = f32::floor(size_avail * split_ratio);
            child_1_size[&axis] =
                child_1.unwrap().size_ref[&axis] = (size_avail - child_0_size[&axis]);
            // IM_ASSERT(child_0.sizeRef[axis] > 0.0 && child_1.sizeRef[axis] > 0.0);
        }
        // 3) If one window is the central node (~ use remaining space, should be made explicit!), use explicit size from the other, and remainder for the central node
        else if child_0.unwrap().size_ref[&axis] != 0.0 && child_1.unwrap().has_central_node_child
        {
            child_0_size[&axis] =
                f32::min(size_avail - size_min_each, child_0.unwrap().size_ref[&axis]);
            child_1_size[&axis] = (size_avail - child_0_size[&axis]);
        } else if child_1.unwrap().size_ref[&axis] != 0.0 && child_0.unwrap().has_central_node_child
        {
            child_1_size[&axis] =
                f32::min(size_avail - size_min_each, child_1.unwrap().size_ref[&axis]);
            child_0_size[&axis] = (size_avail - child_1_size[&axis]);
        } else {
            // 4) Otherwise distribute according to the relative ratio of each size_ref value
            let split_ratio = child_0.unwrap().size_ref[&axis]
                / (child_0.unwrap().size_ref[&axis] + child_1.unwrap().size_ref[&axis]);
            child_0_size[&axis] =
                f32::max(size_min_each, f32::floor(size_avail * split_ratio + 0.5));
            child_1_size[&axis] = (size_avail - child_0_size[&axis]);
        }

        child_1_pos[&axis] += spacing + child_0_size[&axis];
    }

    if only_write_to_single_node.is_none() {
        child_0.unwrap().want_lock_size_once = false;
        child_1.unwrap().want_lock_size_once = false;
    }

    let child_0_recurse = if only_write_to_single_node {
        child_0_is_toward_single_node
    } else {
        child_0.unwrap().is_visible
    };
    let child_1_recurse = if only_write_to_single_node {
        child_1_is_toward_single_node
    } else {
        child_1.unwrap().is_visible
    };
    if child_0_recurse {
        dock_node_tree_update_pos_size(g, child_0.unwrap(), &child_0_pos, &child_0_size, None);
    }
    if child_1_recurse {
        dock_node_tree_update_pos_size(g, child_1.unwrap(), &child_1_pos, &child_1_size, None);
    }
}

// static void dock_node_tree_update_splitterFindTouchingNode(ImGuiDockNode* node, ImGuiAxis axis, int side, ImVector<ImGuiDockNode*>* touching_nodes)
pub fn dock_node_tree_update_splitter_find_touching_node(
    g: &mut Context,
    node: &mut DockNode,
    axis: Axis,
    side: i32,
    touching_nodes: &mut Vec<Id32>,
) {
    if node.is_leaf_node() {
        touching_nodes.push_back(node);
        return;
    }
    if node.child_nodes[0].is_visible {
        if node.split_axis != axis || side == 0 || !node.child_nodes[1].is_visible {
            dock_node_tree_update_splitterFindTouchingNode(
                node.child_nodes[0],
                axis,
                side,
                touching_nodes,
            );
        }
    }
    if node.child_nodes[1].is_visible {
        if node.split_axis != axis || side == 1 || !node.child_nodes[0].is_visible {
            dock_node_tree_update_splitterFindTouchingNode(
                node.child_nodes[1],
                axis,
                side,
                touching_nodes,
            );
        }
    }
}
