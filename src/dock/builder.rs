use crate::dock::context::{
    dock_context_add_node, dock_context_find_node_by_id, dock_context_process_dock,
    dock_context_process_undock_window, dock_context_remove_node,
};
use crate::dock::node::window::dock_node_find_window_by_id;
use crate::dock::node::{dock_node_get_root_node, window, DockNode, DockNodeFlags};
use crate::dock::ops::set_window_dock;
use crate::dock::request::{DockRequest, DockRequestType};
use crate::dock::{context, node, settings};
use crate::globals::GImGui;
use crate::math::saturate_f32;
use crate::settings::{
    create_new_window_settings, find_or_create_window_settings, find_window_settings,
};
use crate::types::{DataAuthority, Direction, Id32};
use crate::vectors::two_d::Vector2D;
use crate::window::get::{find_window_by_name, find_window_id};
use crate::window::settings::WindowSettings;
use crate::{dock, hash_string, Context, INVALID_ID};
use std::collections::HashSet;

// void dock_builder_dock_window(const char* window_name, ImGuiID node_id)
pub fn dock_builder_dock_window(g: &mut Context, window_name: &str, node_id: Id32) {
    // We don't preserve relative order of multiple docked windows (by clearing dock_order back to -1)
    // ImGuiID window_id = ImHashStr(window_name);
    let window_id = hash_string(window_name, 0);
    let window = find_window_by_id(g, window_id);
    // if (ImGuiWindow* window = find_window_by_id(window_id))
    if window.is_some() {
        // Apply to created window
        set_window_dock(g, window, node_id, Cond::Always);
        window.dock_order = -1;
    } else {
        // Apply to settings
        // ImGuiWindowSettings* settings = FindWindowSettings(window_id);
        let mut settings_opt = find_window_settings(g, window_id);
        let mut settings: &mut WindowSettings;
        if settings_opt.is_none() {
            settings = create_new_window_settings(g, window_name);
        } else {
            settings = settings_opt.some();
        }
        settings.dock_id = node_id;
        settings.dock_order = -1;
    }
}

// ImGuiDockNode* dock_builder_get_node(ImGuiID node_id)
pub fn dock_builder_get_node(g: &mut Context, node_id: Id32) -> Option<&mut DockNode> {
    // ImGuiContext* .g = GImGui;
    return dock_context_find_node_by_id(g, node_id);
}

// void DockBuilderSetNodePos(ImGuiID node_id, Vector2D pos)
pub fn dock_builder_set_node_pos(g: &mut Context, node_id: Id32, pos: Vector2D) {
    // ImGuiContext* .g = GImGui;
    // ImGuiDockNode* node = dock_context_find_node_by_id(.g, node_id);
    let node_opt = dock_cotext_find_node_by_id(g, node_id);
    // if (node == None)
    //     return;
    if node_opt.is_none() {
        return;
    }
    let node = node.some();
    node.pos = pos;
    node.authority_for_pos = DataAuthority::DockNode;
}

// void DockBuilderSetNodeSize(ImGuiID node_id, Vector2D size)
pub fn dock_builder_set_node_size(g: &mut Context, node_id: Id32, size: Vector2D) {
    // ImGuiContext* .g = GImGui;
    // ImGuiDockNode* node = dock_context_find_node_by_id(.g, node_id);
    let node = dock_context_find_node_by_id(g, node_id);
    // if (node == None)
    //     return;
    if node.is_none() {
        return;
    }
    // IM_ASSERT(size.x > 0.0 && size.y > 0.0);
    node.unwrap().size = size;
    node.unwrap().size_ref = size.clones();
    node.unwrap().authority_for_size = DataAuthority::DockNode;
}

// Make sure to use the ImGuiDockNodeFlags_DockSpace flag to create a dockspace node! Otherwise this will create a floating node!
// - Floating node: you can then call DockBuilderSetNodePos()/DockBuilderSetNodeSize() to position and size the floating node.
// - Dockspace node: calling DockBuilderSetNodePos() is unnecessary.
// - If you intend to split a node immediately after creation using DockBuilderSplitNode(), make sure to call DockBuilderSetNodeSize() beforehand!
//   For various reason, the splitting code currently needs a base size otherwise space may not be allocated as precisely as you would expect.
// - Use (id == 0) to let the system allocate a node identifier.
// - Existing node with a same id will be removed.
// ImGuiID DockBuilderAddNode(ImGuiID id, ImGuiDockNodeFlags flags)
pub fn dock_builder_add_node(g: &mut Context, id: Id32, flags: &HashSet<DockNodeFlags>) -> Id32 {
    // ImGuiContext* .g = GImGui;

    if id != INVALID_ID {
        dock_builder_remove_node(g, id);
    }

    // ImGuiDockNode* node = None;
    let node: Option<&mut DockNode>;
    // if (flags & DockNodeFlags::DockSpace)
    if flags.contains(&DockNodeFlags::DockSpace) {
        let mut flags_b: HashSet<DockNodeFlags> = flags.clone();
        flags_b.remove(&DockNodeFlags::DockSpace);
        flags_b.insert(DockNodeFlags::KeepAliveOnly);
        let dock_space = DockSpace::new(id, Vector2D::new(0.0, 0.0), &flags_b);
        node = dock_context_find_node_by_id(g, id);
    } else {
        node = Some(dock_context_add_node(g, id));
        node.set_local_flags(flags);
    }
    node.last_frame_alive = g.frame_count; // Set this otherwise BeginDocked will undock during the same frame.
    return node.id;
}

// void DockBuilderRemoveNode(ImGuiID node_id)
pub fn dock_builder_remove_node(g: &mut Context, node_id: Id32) {
    // ImGuiContext* g = GImGui;
    let mut node = dock_context_find_node_by_id(g, node_id);
    if node.is_none() {
        return;
    }
    dock_builder_remove_nodeDockedWindows(node_id, true);
    dock_builder_remove_nodeChildNodes(node_id);
    // Node may have moved or deleted if e.g. any merge happened
    node = dock_context_find_node_by_id(g, node_id);
    if node.is_none() {
        return;
    }
    if node.is_central_node() && node.parent_node {
        node.parent_node
            .set_local_flags(node.parent_node.local_flags | DockNodeFlags::CentralNode);
    }
    dock_context_remove_node(g, node.some(), true);
}

// root_id = 0 to remove all, root_id != 0 to remove child of given node.
// void DockBuilderRemoveNodeChildNodes(ImGuiID root_id)
pub fn dock_builder_remove_node_child_nodes(g: &mut Context, root_id: Id32) {
    // ImGuiContext* g = GImGui;
    // ImGuiDockContext* dc  = &g.dock_context;
    let dc = &mut g.dock_context;

    // ImGuiDockNode* root_node = root_id ? dock_context_find_node_by_id(g, root_id) : None;
    let root_node = if root_id != INVALID_ID {
        dock_context_find_node_by_id(g, root_id)
    } else {
        None
    };

    if root_id != INVALID_ID && root_node.is_none() {
        return;
    }
    let mut has_central_node = false;

    let backup_root_node_authority_for_pos = if root_node.is_some() {
        root_node.authority_for_pos
    } else {
        DataAuthority::Auto
    };
    let backup_root_node_authority_for_size = if root_node.is_some() {
        root_node.authority_for_size
    } else {
        DataAuthority::Auto
    };

    // Process active windows
    // ImVector<ImGuiDockNode*> nodes_to_remove;
    let mut nodes_to_remove: Vec<&mut DockNonde>;
    // for (int n = 0; n < dc.Nodes.data.size; n += 1)
    for node_id in dc.nodes.iter() {
        let node_opt = g.get_dock_node(*node_id);
        // if (ImGuiDockNode * node = dc.Nodes.data[n].val_p)
        if node_opt.is_some() {
            let node = node_opt.unwrap();
            let want_removal = (root_id == 0)
                || (node.id != root_id && dock_node_get_root_node(g, node).id == root_id);
            if want_removal {
                if node.is_central_node() {
                    has_central_node = true;
                }
                if root_id != INVALID_ID {
                    dock_context_queue_notify_removed_node(g, node);
                }
                if root_node {
                    window::dock_node_move_windows(g, root_node.unwrap(), node);
                    settings::dock_settings_rename_node_references(g, node.id, root_node.id);
                }
                nodes_to_remove.push_back(node);
            }
        }
    }

    // DockNodeMoveWindows->DockNodeAddWindow will normally set those when reaching two windows (which is only adequate during interactive merge)
    // Make sure we don't lose our current pos/size. (FIXME-DOCK: Consider tidying up that code in DockNodeAddWindow instead)
    if root_node.unwrap() {
        root_node.unwrap().authority_for_pos = backup_root_node_authority_for_pos;
        root_node.unwrap().authority_for_size = backup_root_node_authority_for_size;
    }

    // Apply to settings
    // for (ImGuiWindowSettings* settings = g.SettingsWindows.begin(); settings != None; settings = g.SettingsWindows.next_chunk(settings))
    for settings in g.settings_windows.iter_mut() {
        // if (ImGuiID
        // window_settings_dock_id = settings.dock_id)
        // window_settings_dock_id
        let windows_settings_dock_id = settings.dock_id;
        if windows_settings_dock_id != INVALID_ID {
            // for (int n = 0; n < nodes_to_remove.size; n += 1)
            for n in nodes_to_remove {
                // if (nodes_to_remove[n].id == window_settings_dock_id)
                if n.id == window_settings_dock_id {
                    settings.dock_id = root_id;
                    break;
                }
            }
        }
    }

    // Not really efficient, but easier to destroy a whole hierarchy considering dock_context_remove_node is attempting to merge nodes
    if nodes_to_remove.len() > 1 {
        // ImQsort(nodes_to_remove.data, nodes_to_remove.size, sizeof(ImGuiDockNode *), DockNodeComparerDepthMostFirst);
        nodes_to_remove.sort();
    }
    // for (int n = 0; n < nodes_to_remove.size; n += 1)
    for n in nodes_to_remove {
        dock_context_remove_node(g, n, false);
    }

    if root_id == INVALID_ID {
        dc.nodes.clear();
        dc.requests.clear();
    } else if has_central_node {
        root_node.central_node = root_node;
        let mut flags_to_set = root_node.local_flags.clone();
        flags_to_set.insert(DockNodeFlags::CentralNode);
        // root_node.set_local_flags(root_node.LocalFlags | DockNodeFlags::CentralNode);
        root_node.set_local_flags(flags_to_set);
    }
}

// void DockBuilderRemoveNodeDockedWindows(ImGuiID root_id, bool clear_settings_refs)
pub fn dock_builder_remove_node_docked_windows(
    g: &mut Context,
    root_id: Id32,
    clear_settings_refs: bool,
) {
    // clear references in settings
    // ImGuiContext* g = GImGui;
    // ImGuiContext& g = *.g;
    if clear_settings_refs {
        // for (ImGuiWindowSettings* settings = g.settings_windows.begin(); settings != None; settings = g.settings_windows.next_chunk(settings))
        for settings in g.settings_handlers.iter_mut() {
            let mut want_removal = (root_id == INVALID_ID) || (settings.dock_id == root_id);
            if !want_removal && settings.dock_id != INVALID_ID {
                // if ImGuiDockNode * node = dock_context_find_node_by_id(g, settings.dock_id)
                let node = dock_context_find_node_by_id(g, settings.dock_id);
                {
                    if dock_node_get_root_node(g, node.unwrap()).id == root_id {
                        want_removal = true;
                    }
                }
            }
            if want_removal {
                settings.dock_id = INVALID_ID;
            }
        }
    }

    // clear references in windows
    // for (int n = 0; n < g.windows.len(); n += 1)
    for (_, window) in g.windows.iter_mut() {
        // ImGuiWindow* window = g.windows[n];
        let win_dock_node = g.get_dock_node(window.dock_node_id).unwrap();
        let want_removal = (root_id == 0)
            || (window.dock_node_id != INVALID_ID
                && dock_node_get_root_node(g, win_dock_node).id == root_id)
            || (window.dock_node_as_host_id != INVALID_ID
                && window.dock_node_as_host_id == root_id);
        if want_removal {
            // const ImGuiID backup_dock_id = window.dock_id;
            let backup_dock_id = window.dock_id;
            // IM_UNUSED(backup_dock_id);
            dock_context_process_undock_window(g, window, clear_settings_refs);
            if !clear_settings_refs {}
            // IM_ASSERT(window.DockId == backup_dock_id);
        }
    }
}

// If 'out_id_at_dir' or 'out_id_at_opposite_dir' are non None, the function will write out the id of the two new nodes created.
// Return value is id of the node at the specified direction, so same as (*out_id_at_dir) if that pointer is set.
// FIXME-DOCK: We are not exposing nor using split_outer.
// ImGuiID DockBuilderSplitNode(ImGuiID id, ImGuiDir split_dir, float size_ratio_for_node_at_dir, ImGuiID* out_id_at_dir, ImGuiID* out_id_at_opposite_dir)
pub fn dock_builder_split_node(
    g: &mut Context,
    id: Id32,
    split_dir: Direction,
    size_ratio_for_node_at_dir: f32,
    out_id_at_dir: &mut Id32,
    out_id_at_opposite_dir: &mut Id32,
) -> Id32 {
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(split_dir != Dir::None);
    // IMGUI_DEBUG_LOG_DOCKING("[docking] DockBuilderSplitNode: node 0x%08X, split_dir %d\n", id, split_dir);

    let node = dock_context_find_node_by_id(g, id);
    if node.is_none() {
        // IM_ASSERT(node != None);
        return 0;
    }

    // IM_ASSERT(!node.IsSplitNode()); // Assert if already split

    // ImGuiDockRequest req;
    let mut req: DockRequest = DockRequest::default();
    req.request_type = DockRequestType::Split;
    req.dock_target_window_id = INVALID_ID;
    req.dock_target_node_id = INVALID_ID;
    req.dock_payload_id = INVALID_ID;
    req.dock_split_dir = split_dir;
    req.dock_split_ratio = saturate_f32(
        if split_dir == Direction::Left || split_dir == Direction::Up {
            size_ratio_for_node_at_dir
        } else {
            1.0 - size_ratio_for_node_at_dir
        },
    );
    req.dock_split_outer = false;
    dock_context_process_dock(g, &mut req);

    let id_at_dir =
        node.child_nodes[if split_dir == Direction::Left || split_dir == Direction::Up {
            0
        } else {
            1
        }]
        .id;
    let id_at_opposite_dir =
        node.child_nodes[if split_dir == Direction::Left || split_dir == Direction::Up {
            1
        } else {
            0
        }]
        .id;
    if out_id_at_dir {
        *out_id_at_dir = id_at_dir;
    }
    if out_id_at_opposite_dir {
        *out_id_at_opposite_dir = id_at_opposite_dir;
    }
    return id_at_dir;
}

// static ImGuiDockNode* dock_builder_copy_node_rec(ImGuiDockNode* src_node, ImGuiID dst_node_id_if_known, ImVector<ImGuiID>* out_node_remap_pairs)
pub fn dock_builder_copy_node_rec(
    g: &mut Context,
    src_node: &mut DockNode,
    dst_node_id_if_known: Id32,
    out_node_remap_pairs: &mut Vec<Id32>,
) -> &mut DockNode {
    // ImGuiContext& g = *GImGui;
    let mut dst_node = dock_context_add_node(g, dst_node_id_if_known);
    dst_node.shared_flags = src_node.shared_flags.clone();
    dst_node.local_flags = src_node.local_flags.clone();
    dst_node.local_flags_in_windows.clear(); //= DockNodeFlags::None;
    dst_node.pos = src_node.pos.clone();
    dst_node.size = src_node.size.clone();
    dst_node.size_ref = src_node.size_ref.clone();
    dst_node.split_axis = src_node.split_axis.clone();
    dst_node.update_merged_flags();

    out_node_remap_pairs.push_back(src_node.id);
    out_node_remap_pairs.push_back(dst_node.id);

    // for (int child_n = 0; child_n < IM_ARRAYSIZE(src_node.child_nodes); child_n += 1)
    // for child_node_id in src_node.child_nodes
    for child_n in 0..src_node.child_nodes.len() {
        let child_node_id = src_node.child_nodes[child_n];
        if child_node_id != INVALID_ID {
            let child_node = g.get_dock_node(child_node_id).unwrap();
            let x = dock_builder_copy_node_rec(g, child_node, 0, out_node_remap_pairs);
            // dst_node.child_nodes[child_node_id] = dock_builder_copy_node_rec(src_node.child_nodes[child_node_id], 0, out_node_remap_pairs);
            //[src_node.child_nodes.index].parent_node = dst_node;
            let child_node_id = dst_node.child_nodes[child_node_id];
            g.get_dock_node(child_node_id).unwrap().parent_node_id = dst_node.id;
        }
    }

    // IMGUI_DEBUG_LOG_DOCKING("[docking] Fork node %08X -> %08X (%d childs)\n", src_node.ID, dst_node.ID, dst_node.IsSplitNode() ? 2 : 0);
    return dst_node;
}

// void dock_builder_copy_node(ImGuiID src_node_id, ImGuiID dst_node_id, ImVector<ImGuiID>* out_node_remap_pairs)
pub fn dock_builder_copy_node(
    g: &mut Context,
    src_node_id: Id32,
    dst_node_id: Id32,
    out_node_remap_pairs: &mut Vec<Id32>,
) {
    // ImGuiContext* g = GImGui;
    // IM_ASSERT(src_node_id != 0);
    // IM_ASSERT(dst_node_id != 0);
    // IM_ASSERT(out_node_remap_pairs != None);

    dock_builder_remove_node(g, dst_node_id);

    let src_node = dock_context_find_node_by_id(g, src_node_id);
    // IM_ASSERT(src_node != None);

    out_node_remap_pairs.clear();
    dock_builder_copy_node_rec(g, src_node.unwrap(), dst_node_id, out_node_remap_pairs);

    // IM_ASSERT((out_node_remap_pairs.size % 2) == 0);
}

// void dock_builder_copy_window_settings(const char* src_name, const char* dst_name)
pub fn dock_builder_copy_window_settings(g: &mut Context, src_name: &str, dst_name: &str) {
    let src_window = find_window_by_name(g, src_name);
    if src_window.is_none() {
        return;
    }
    let mut dst_window = find_window_by_name(g, dst_name);
    // if (ImGuiWindow* dst_window = find_window_by_name(dst_name))
    if dst_window.is_some() {
        dst_window.unwrap().pos = src_window.unwrap().pos.clone();
        dst_window.unwrap().size = src_window.unwrap().size.clone();
        dst_window.unwrap().size_full = src_window.unwrap().size_full.clone();
        dst_window.unwrap().collapsed = src_window.unwrap().collapsed;
    }
    // else if (ImGuiWindowSettings* dst_settings = FindOrCreateWindowSettings(dst_name))
    let dst_settings = find_or_create_window_settings(g, dst_name);
    let window_pos_2ih = Vector2D::new(src_window.pos.x, src_window.pos.y);
    if src_window.viewport_id != 0 && src_window.viewport_id != IMGUI_VIEWPORT_DEFAULT_ID {
        dst_settings.viewport_pos = window_pos_2ih;
        dst_settings.viewport_id = src_window.viewport_id;
        dst_settings.pos = Vector2D::default();
    } else {
        dst_settings.pos = window_pos_2ih;
    }
    dst_settings.size = Vector2D(src_window.size_full);
    dst_settings.collapsed = src_window.collapsed;
}

// FIXME: Will probably want to change this signature, in particular how the window remapping pairs are passed.
// void DockBuilderCopyDockSpace(ImGuiID src_dockspace_id, ImGuiID dst_dockspace_id, ImVector<const char*>* in_window_remap_pairs)
pub fn dock_builder_copy_dock_space(
    g: &mut Context,
    src_dockspace_id: Id32,
    dst_dockspace_id: Id32,
    in_window_remap_pairs: &mut Vec<String>,
) {
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(src_dockspace_id != 0);
    // IM_ASSERT(dst_dockspace_id != 0);
    // IM_ASSERT(in_window_remap_pairs != None);
    // IM_ASSERT((in_window_remap_pairs.size % 2) == 0);

    // Duplicate entire dock
    // FIXME: When overwriting dst_dockspace_id, windows that aren't part of our dockspace window class but that are docked in a same node will be split apart,
    // whereas we could attempt to at least keep them together in a new, same floating node.
    // ImVector<ImGuiID> node_remap_pairs;
    let mut node_remap_pairs: Vec<Id32> = vec![];
    dock_builder_copy_node(g, src_dockspace_id, dst_dockspace_id, &mut node_remap_pairs);

    // Attempt to transition all the upcoming windows associated to dst_dockspace_id into the newly created hierarchy of dock nodes
    // (The windows associated to src_dockspace_id are staying in place)
    // ImVector<ImGuiID> src_windows;
    let mut src_windows: Vec<Id32> = vec![];
    // for (int remap_window_n = 0; remap_window_n < in_window_remap_pairs.size; remap_window_n += 2)
    for remap_window_n in (0..in_window_remap_pairs.len()).step_by(2) {
        let src_window_name = in_window_remap_pairs[remap_window_n].clone();
        let dst_window_name = in_window_remap_pairs[remap_window_n + 1].clone();
        let src_window_id = hash_string(src_window_name.as_str(), 0);
        src_windows.push(src_window_id);

        // Search in the remapping tables
        let mut src_dock_id = INVALID_ID;
        // if (ImGuiWindow* src_window = find_window_by_id(src_window_id)) {
        let mut src_window = find_window_by_id(g, src_window_id);
        if src_window.is_some() {
            src_dock_id = src_window.dock_id;
        }
        let src_window_settings = find_window_settings(g, src_window_id);
        // else if (ImGuiWindowSettings* src_window_settings = FindWindowSettings(src_window_id)) {
        if src_window_settings.is_some() {
            src_dock_id = src_window_settings.dock_id;
        }
        // ImGuiID dst_dock_id = INVALID_ID;
        let mut dst_dock_id = INVALID_ID;
        // for (int dock_remap_n = 0; dock_remap_n < node_remap_pairs.size; dock_remap_n += 2)
        for dock_remap_n in (0..node_remap_pairs.len()).step_by(2) {
            if node_remap_pairs[dock_remap_n] == src_dock_id {
                dst_dock_id = node_remap_pairs[dock_remap_n + 1];
                //node_remap_pairs[dock_remap_n] = node_remap_pairs[dock_remap_n + 1] = 0; // clear
                break;
            }
        }

        if dst_dock_id != 0 {
            // Docked windows gets redocked into the new node hierarchy.
            // IMGUI_DEBUG_LOG_DOCKING("[docking] Remap live window '%s' 0x%08X -> '%s' 0x%08X\n", src_window_name, src_dock_id, dst_window_name, dst_dock_id);
            dock_builder_dock_window(g, dst_window_name.as_str(), dst_dock_id);
        } else {
            // Floating windows gets their settings transferred (regardless of whether the new window already exist or not)
            // When this is leading to a Copy and not a Move, we would get two overlapping floating windows. Could we possibly dock them together?
            // IMGUI_DEBUG_LOG_DOCKING("[docking] Remap window settings '%s' -> '%s'\n", src_window_name, dst_window_name);
            dock_builder_copy_window_settings(
                g,
                src_window_name.as_str(),
                dst_window_name.as_str(),
            );
        }
    }

    // Anything else in the source nodes of 'node_remap_pairs' are windows that were docked in src_dockspace_id but are not owned by it (unaffiliated windows, e.g. "ImGui Demo")
    // Find those windows and move to them to the cloned dock node. This may be optional?
    // for (int dock_remap_n = 0; dock_remap_n < node_remap_pairs.size; dock_remap_n += 2)
    for dock_remap_n in (0..node_remap_pairs.len()).step_by(2) {
        // if (ImGuiID
        // src_dock_id = node_remap_pairs[dock_remap_n])
        let src_dock_id = node_remap_pairs[dock_remap_n];
        if src_doc_id != INVALID_ID {
            // ImGuiID
            let dst_dock_id = node_remap_pairs[dock_remap_n + 1];
            let node = dock_builder_get_node(g, src_dock_id);
            // for (int window_n = 0; window_n < node.windows.len(); window_n += 1)
            for window_n in 0..node.windows.len() {
                // ImGuiWindow * window = node.windows[window_n];
                let window = g.get_window(window_n);
                if src_windows.contains(&window.id) {
                    continue;
                }

                // Docked windows gets redocked into the new node hierarchy.
                // IMGUI_DEBUG_LOG_DOCKING("[docking] Remap window '%s' %08X -> %08X\n", window.Name, src_dock_id, dst_dock_id);
                dock_builder_dock_window(g, &window.name, dst_dock_id);
            }
        }
    }
}

// FIXME-DOCK: This is awkward because in series of split user is likely to loose access to its root node.
// void DockBuilderFinish(ImGuiID root_id)
pub fn dock_builder_finish(g: &mut Context, root_id: Id32) {
    ImGuiContext * g = GImGui;
    //DockContextRebuild(ctx);
    dock_context_build_add_windows_to_nodes(g, root_id);
}
