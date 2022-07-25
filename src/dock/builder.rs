use std::collections::HashSet;
use crate::{Context, dock, INVALID_ID};
use crate::dock::context::{dock_context_add_node, dock_context_find_node_by_id, dock_context_remove_node};
use crate::dock::{context, node, settings};
use crate::dock::node::{dock_node_get_root_node, DockNode, DockNodeFlags};
use crate::dock::request::DockRequestType;
use crate::globals::GImGui;
use crate::types::{DataAuthority, Direction, Id32};
use crate::vectors::two_d::Vector2D;
use crate::window::get::find_window_by_name;

// void DockBuilderDockWindow(const char* window_name, ImGuiID node_id)
pub fn dock_builder_dock_window(g: &mut Context, window_name: &str, node_id: Id32)
{
    // We don't preserve relative order of multiple docked windows (by clearing dock_order back to -1)
    ImGuiID window_id = ImHashStr(window_name);
    if (ImGuiWindow* window = FindWindowByID(window_id))
    {
        // Apply to created window
        SetWindowDock(window, node_id, Cond::Always);
        window.DockOrder = -1;
    }
    else
    {
        // Apply to settings
        ImGuiWindowSettings* settings = FindWindowSettings(window_id);
        if (settings == NULL)
            settings = CreateNewWindowSettings(window_name);
        settings.dock_id = node_id;
        settings.dock_order = -1;
    }
}

// ImGuiDockNode* DockBuilderGetNode(ImGuiID node_id)
pub fn dock_builder_get_node(g: &mut Context, node_id: Id32) -> &mut DockNode
{
    ImGuiContext* .g = GImGui;
    return dock_context_find_node_by_id(.g, node_id);
}

// void DockBuilderSetNodePos(ImGuiID node_id, Vector2D pos)
pub fn dock_builder_set_node_pos(g: &mut Context, node_id: Id32, pos: Vector2D)
{
    ImGuiContext* .g = GImGui;
    ImGuiDockNode* node = dock_context_find_node_by_id(.g, node_id);
    if (node == NULL)
        return;
    node.pos = pos;
    node.authority_for_pos = DataAuthority::DockNode;
}

// void DockBuilderSetNodeSize(ImGuiID node_id, Vector2D size)
pub fn dock_builder_set_node_size(g: &mut Context, node_id: Id32, size: Vector2D)
{
    ImGuiContext* .g = GImGui;
    ImGuiDockNode* node = dock_context_find_node_by_id(.g, node_id);
    if (node == NULL)
        return;
    // IM_ASSERT(size.x > 0.0 && size.y > 0.0);
    node.size = node.size_ref = size;
    node.authority_for_size = DataAuthority::DockNode;
}

// Make sure to use the ImGuiDockNodeFlags_DockSpace flag to create a dockspace node! Otherwise this will create a floating node!
// - Floating node: you can then call DockBuilderSetNodePos()/DockBuilderSetNodeSize() to position and size the floating node.
// - Dockspace node: calling DockBuilderSetNodePos() is unnecessary.
// - If you intend to split a node immediately after creation using DockBuilderSplitNode(), make sure to call DockBuilderSetNodeSize() beforehand!
//   For various reason, the splitting code currently needs a base size otherwise space may not be allocated as precisely as you would expect.
// - Use (id == 0) to let the system allocate a node identifier.
// - Existing node with a same id will be removed.
// ImGuiID DockBuilderAddNode(ImGuiID id, ImGuiDockNodeFlags flags)
pub fn dock_builder_add_node(g: &mut Context, id: Id32, flags: &HashSet<DockNodeFlags>) -> Id32
{
    ImGuiContext* .g = GImGui;

    if (id != 0)
        DockBuilderRemoveNode(id);

    ImGuiDockNode* node = NULL;
    if (flags & DockNodeFlags::DockSpace)
    {
        DockSpace(id, Vector2D::new(0, 0), (flags & ~DockNodeFlags::DockSpace) | DockNodeFlags::KeepAliveOnly);
        node = dock_context_find_node_by_id(.g, id);
    }
    else
    {
        node = dock_context_add_node(.g, id);
        node.set_local_flags(flags);
    }
    node.LastFrameAlive = .g.frame_count;   // Set this otherwise BeginDocked will undock during the same frame.
    return node.id;
}

// void DockBuilderRemoveNode(ImGuiID node_id)
pub fn dock_builder_remove_node(g: &mut Context, node_id: Id32)
{
    ImGuiContext* .g = GImGui;
    ImGuiDockNode* node = dock_context_find_node_by_id(.g, node_id);
    if (node == NULL)
        return;
    DockBuilderRemoveNodeDockedWindows(node_id, true);
    DockBuilderRemoveNodeChildNodes(node_id);
    // Node may have moved or deleted if e.g. any merge happened
    node = dock_context_find_node_by_id(.g, node_id);
    if (node == NULL)
        return;
    if (node.is_central_node() && node.parent_node)
        node.parent_node.set_local_flags(node.parent_node.LocalFlags | DockNodeFlags::CentralNode);
    dock_context_remove_node(.g, node, true);
}

// root_id = 0 to remove all, root_id != 0 to remove child of given node.
// void DockBuilderRemoveNodeChildNodes(ImGuiID root_id)
pub fn dock_builder_remove_node_child_nodes(g: &mut Context, root_id: Id32)
{
    ImGuiContext* .g = GImGui;
    ImGuiDockContext* dc  = &.g.dock_context;

    ImGuiDockNode* root_node = root_id ? dock_context_find_node_by_id(.g, root_id) : NULL;
    if (root_id && root_node == NULL)
        return;
    bool has_central_node = false;

    ImGuiDataAuthority backup_root_node_authority_for_pos = root_node ? root_node.authority_for_pos : DataAuthority::Auto;
    ImGuiDataAuthority backup_root_node_authority_for_size = root_node ? root_node.authority_for_size : DataAuthority::Auto;

    // Process active windows
    ImVector<ImGuiDockNode*> nodes_to_remove;
    for (int n = 0; n < dc.Nodes.data.size; n += 1)
        if (ImGuiDockNode* node = (ImGuiDockNode*)dc.Nodes.data[n].val_p)
        {
            bool want_removal = (root_id == 0) || (node.id != root_id && dock_node_get_root_node(node).id == root_id);
            if (want_removal)
            {
                if (node.is_central_node())
                    has_central_node = true;
                if (root_id != 0)
                    DockContextQueueNotifyRemovedNode(.g, node);
                if (root_node)
                {
                    node::dock_node_move_windows(root_node, node);
                    settings::dock_settings_rename_node_references(node.id, root_node.id);
                }
                nodes_to_remove.push_back(node);
            }
        }

    // DockNodeMoveWindows->DockNodeAddWindow will normally set those when reaching two windows (which is only adequate during interactive merge)
    // Make sure we don't lose our current pos/size. (FIXME-DOCK: Consider tidying up that code in DockNodeAddWindow instead)
    if (root_node)
    {
        root_node.authority_for_pos = backup_root_node_authority_for_pos;
        root_node.authority_for_size = backup_root_node_authority_for_size;
    }

    // Apply to settings
    for (ImGuiWindowSettings* settings = .g.SettingsWindows.begin(); settings != NULL; settings = .g.SettingsWindows.next_chunk(settings))
        if (ImGuiID window_settings_dock_id = settings.dock_id)
            for (int n = 0; n < nodes_to_remove.size; n += 1)
                if (nodes_to_remove[n].id == window_settings_dock_id)
                {
                    settings.dock_id = root_id;
                    break;
                }

    // Not really efficient, but easier to destroy a whole hierarchy considering dock_context_remove_node is attempting to merge nodes
    if (nodes_to_remove.size > 1)
        ImQsort(nodes_to_remove.data, nodes_to_remove.size, sizeof(ImGuiDockNode*), DockNodeComparerDepthMostFirst);
    for (int n = 0; n < nodes_to_remove.size; n += 1)
        dock_context_remove_node(.g, nodes_to_remove[n], false);

    if (root_id == 0)
    {
        dc.Nodes.Clear();
        dc.requests.clear();
    }
    else if (has_central_node)
    {
        root_node.central_node = root_node;
        root_node.set_local_flags(root_node.LocalFlags | DockNodeFlags::CentralNode);
    }
}

// void DockBuilderRemoveNodeDockedWindows(ImGuiID root_id, bool clear_settings_refs)
pub fn dock_builder_remove_node_docked_windows(g: &mut Context, root_id: Id32, clear_settings_refs: bool)
{
    // clear references in settings
    ImGuiContext* .g = GImGui;
    // ImGuiContext& g = *.g;
    if (clear_settings_refs)
    {
        for (ImGuiWindowSettings* settings = g.settings_windows.begin(); settings != NULL; settings = g.settings_windows.next_chunk(settings))
        {
            bool want_removal = (root_id == 0) || (settings.dock_id == root_id);
            if (!want_removal && settings.dock_id != 0)
                if (ImGuiDockNode* node = dock_context_find_node_by_id(.g, settings.dock_id))
                    if (dock_node_get_root_node(node).id == root_id)
                        want_removal = true;
            if (want_removal)
                settings.dock_id = INVALID_ID;
        }
    }

    // clear references in windows
    for (int n = 0; n < g.windows.len(); n += 1)
    {
        ImGuiWindow* window = g.windows[n];
        bool want_removal = (root_id == 0) || (window.dock_node && dock_node_get_root_node(window.dock_node).id == root_id) || (window.dock_node_as_host && window.dock_node_as_host.id == root_id);
        if (want_removal)
        {
            const ImGuiID backup_dock_id = window.dock_id;
            IM_UNUSED(backup_dock_id);
            DockContextProcessUndockWindow(.g, window, clear_settings_refs);
            if (!clear_settings_refs)
                // IM_ASSERT(window.DockId == backup_dock_id);
        }
    }
}

// If 'out_id_at_dir' or 'out_id_at_opposite_dir' are non NULL, the function will write out the id of the two new nodes created.
// Return value is id of the node at the specified direction, so same as (*out_id_at_dir) if that pointer is set.
// FIXME-DOCK: We are not exposing nor using split_outer.
// ImGuiID DockBuilderSplitNode(ImGuiID id, ImGuiDir split_dir, float size_ratio_for_node_at_dir, ImGuiID* out_id_at_dir, ImGuiID* out_id_at_opposite_dir)
pub fn dock_builder_split_node(g: &mut Context, id: Id32, split_dir: Direction, size_ratio_for_node_at_dir: f32, out_id_at_dir: &mut Id32, out_id_at_opposite_dir: &mut Id32) -> Id32
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(split_dir != Dir::None);
    // IMGUI_DEBUG_LOG_DOCKING("[docking] DockBuilderSplitNode: node 0x%08X, split_dir %d\n", id, split_dir);

    ImGuiDockNode* node = dock_context_find_node_by_id(&g, id);
    if (node == NULL)
    {
        // IM_ASSERT(node != NULL);
        return 0;
    }

    // IM_ASSERT(!node.IsSplitNode()); // Assert if already split

    ImGuiDockRequest req;
    req.Type = DockRequestType::Split;
    req.dock_target_window = NULL;
    req.dock_target_node = node;
    req.dock_payload = NULL;
    req.dock_split_dir = split_dir;
    req.dock_split_ratio = ImSaturate((split_dir == Direction::Left || split_dir == Direction::Up) ? size_ratio_for_node_at_dir : 1.0 - size_ratio_for_node_at_dir);
    req.dock_split_outer = false;
    context::dock_context_process_dock(&g, &req);

    ImGuiID id_at_dir = node.child_nodes[(split_dir == Direction::Left || split_dir == Direction::Up) ? 0 : 1].id;
    ImGuiID id_at_opposite_dir = node.child_nodes[(split_dir == Direction::Left || split_dir == Direction::Up) ? 1 : 0].id;
    if (out_id_at_dir)
        *out_id_at_dir = id_at_dir;
    if (out_id_at_opposite_dir)
        *out_id_at_opposite_dir = id_at_opposite_dir;
    return id_at_dir;
}

// static ImGuiDockNode* DockBuilderCopyNodeRec(ImGuiDockNode* src_node, ImGuiID dst_node_id_if_known, ImVector<ImGuiID>* out_node_remap_pairs)
pub fn dock_builder_copy_node_rec(g: &mut Context, src_node: &mut DockNode, dst_node_id_if_known: Id32, out_node_remap_pairs: &mut Vec<Id32>) -> &mut DockNode
{
    // ImGuiContext& g = *GImGui;
    ImGuiDockNode* dst_node = dock_context_add_node(&g, dst_node_id_if_known);
    dst_node.shared_flags = src_node.shared_flags;
    dst_node.LocalFlags = src_node.LocalFlags;
    dst_node.local_flags_in_windows = DockNodeFlags::None;
    dst_node.pos = src_node.pos;
    dst_node.size = src_node.size;
    dst_node.size_ref = src_node.size_ref;
    dst_node.split_axis = src_node.split_axis;
    dst_node.update_merged_flags();

    out_node_remap_pairs.push_back(src_node.id);
    out_node_remap_pairs.push_back(dst_node.id);

    for (int child_n = 0; child_n < IM_ARRAYSIZE(src_node.child_nodes); child_n += 1)
        if (src_node.child_nodes[child_n])
        {
            dst_node.child_nodes[child_n] = DockBuilderCopyNodeRec(src_node.child_nodes[child_n], 0, out_node_remap_pairs);
            dst_node.child_nodes[child_n]parent_node = dst_node;
        }

    // IMGUI_DEBUG_LOG_DOCKING("[docking] Fork node %08X -> %08X (%d childs)\n", src_node.ID, dst_node.ID, dst_node.IsSplitNode() ? 2 : 0);
    return dst_node;
}

// void DockBuilderCopyNode(ImGuiID src_node_id, ImGuiID dst_node_id, ImVector<ImGuiID>* out_node_remap_pairs)
pub fn dock_builder_copy_node(g: &mut Context, src_node_id: Id32, dst_node_id: Id32, out_node_remap_pairs: &mut Vec<Id32>)
{
    ImGuiContext* .g = GImGui;
    // IM_ASSERT(src_node_id != 0);
    // IM_ASSERT(dst_node_id != 0);
    // IM_ASSERT(out_node_remap_pairs != NULL);

    DockBuilderRemoveNode(dst_node_id);

    ImGuiDockNode* src_node = dock_context_find_node_by_id(.g, src_node_id);
    // IM_ASSERT(src_node != NULL);

    out_node_remap_pairs.clear();
    DockBuilderCopyNodeRec(src_node, dst_node_id, out_node_remap_pairs);

    // IM_ASSERT((out_node_remap_pairs.size % 2) == 0);
}

// void DockBuilderCopyWindowSettings(const char* src_name, const char* dst_name)
pub fn dock_builder_copy_window_settings(g: &mut Context, src_name: &str, dst_name: &str)
{
    ImGuiWindow* src_window = find_window_by_name(src_name);
    if (src_window == NULL)
        return;
    if (ImGuiWindow* dst_window = find_window_by_name(dst_name))
    {
        dst_window.pos = src_window.pos;
        dst_window.size = src_window.size;
        dst_window.sizeFull = src_window.sizeFull;
        dst_window.collapsed = src_window.collapsed;
    }
    else if (ImGuiWindowSettings* dst_settings = FindOrCreateWindowSettings(dst_name))
    {
        Vector2Dih window_pos_2ih = Vector2Dih(src_window.pos);
        if (src_window.viewport_id != 0 && src_window.viewport_id != IMGUI_VIEWPORT_DEFAULT_ID)
        {
            dst_settings.viewport_pos = window_pos_2ih;
            dst_settings.viewport_id = src_window.viewport_id;
            dst_settings.pos = Vector2Dih(0, 0);
        }
        else
        {
            dst_settings.pos = window_pos_2ih;
        }
        dst_settings.size = Vector2Dih(src_window.sizeFull);
        dst_settings.collapsed = src_window.collapsed;
    }
}

// FIXME: Will probably want to change this signature, in particular how the window remapping pairs are passed.
// void DockBuilderCopyDockSpace(ImGuiID src_dockspace_id, ImGuiID dst_dockspace_id, ImVector<const char*>* in_window_remap_pairs)
pub fn dock_builder_copy_dock_space(g: &mut Context, src_dockspace_id: Id32, dst_dockspace_id: Id32, in_window_remap_pairs: &mut Vec<String>)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(src_dockspace_id != 0);
    // IM_ASSERT(dst_dockspace_id != 0);
    // IM_ASSERT(in_window_remap_pairs != NULL);
    // IM_ASSERT((in_window_remap_pairs.size % 2) == 0);

    // Duplicate entire dock
    // FIXME: When overwriting dst_dockspace_id, windows that aren't part of our dockspace window class but that are docked in a same node will be split apart,
    // whereas we could attempt to at least keep them together in a new, same floating node.
    ImVector<ImGuiID> node_remap_pairs;
    DockBuilderCopyNode(src_dockspace_id, dst_dockspace_id, &node_remap_pairs);

    // Attempt to transition all the upcoming windows associated to dst_dockspace_id into the newly created hierarchy of dock nodes
    // (The windows associated to src_dockspace_id are staying in place)
    ImVector<ImGuiID> src_windows;
    for (int remap_window_n = 0; remap_window_n < in_window_remap_pairs.size; remap_window_n += 2)
    {
        const char* src_window_name = (*in_window_remap_pairs)[remap_window_n];
        const char* dst_window_name = (*in_window_remap_pairs)[remap_window_n + 1];
        ImGuiID src_window_id = ImHashStr(src_window_name);
        src_windows.push_back(src_window_id);

        // Search in the remapping tables
        ImGuiID src_dock_id = INVALID_ID;
        if (ImGuiWindow* src_window = FindWindowByID(src_window_id))
            src_dock_id = src_window.dock_id;
        else if (ImGuiWindowSettings* src_window_settings = FindWindowSettings(src_window_id))
            src_dock_id = src_window_settings.dock_id;
        ImGuiID dst_dock_id = INVALID_ID;
        for (int dock_remap_n = 0; dock_remap_n < node_remap_pairs.size; dock_remap_n += 2)
            if (node_remap_pairs[dock_remap_n] == src_dock_id)
            {
                dst_dock_id = node_remap_pairs[dock_remap_n + 1];
                //node_remap_pairs[dock_remap_n] = node_remap_pairs[dock_remap_n + 1] = 0; // clear
                break;
            }

        if (dst_dock_id != 0)
        {
            // Docked windows gets redocked into the new node hierarchy.
            // IMGUI_DEBUG_LOG_DOCKING("[docking] Remap live window '%s' 0x%08X -> '%s' 0x%08X\n", src_window_name, src_dock_id, dst_window_name, dst_dock_id);
            DockBuilderDockWindow(dst_window_name, dst_dock_id);
        }
        else
        {
            // Floating windows gets their settings transferred (regardless of whether the new window already exist or not)
            // When this is leading to a Copy and not a Move, we would get two overlapping floating windows. Could we possibly dock them together?
            // IMGUI_DEBUG_LOG_DOCKING("[docking] Remap window settings '%s' -> '%s'\n", src_window_name, dst_window_name);
            DockBuilderCopyWindowSettings(src_window_name, dst_window_name);
        }
    }

    // Anything else in the source nodes of 'node_remap_pairs' are windows that were docked in src_dockspace_id but are not owned by it (unaffiliated windows, e.g. "ImGui Demo")
    // Find those windows and move to them to the cloned dock node. This may be optional?
    for (int dock_remap_n = 0; dock_remap_n < node_remap_pairs.size; dock_remap_n += 2)
        if (ImGuiID src_dock_id = node_remap_pairs[dock_remap_n])
        {
            ImGuiID dst_dock_id = node_remap_pairs[dock_remap_n + 1];
            ImGuiDockNode* node = DockBuilderGetNode(src_dock_id);
            for (int window_n = 0; window_n < node.windows.len(); window_n += 1)
            {
                ImGuiWindow* window = node.windows[window_n];
                if (src_windows.contains(window.id))
                    continue;

                // Docked windows gets redocked into the new node hierarchy.
                // IMGUI_DEBUG_LOG_DOCKING("[docking] Remap window '%s' %08X -> %08X\n", window.Name, src_dock_id, dst_dock_id);
                DockBuilderDockWindow(window.Name, dst_dock_id);
            }
        }
}

// FIXME-DOCK: This is awkward because in series of split user is likely to loose access to its root node.
// void DockBuilderFinish(ImGuiID root_id)
pub fn dock_builder_finish(g: &mut Context, root_id: Id32)
{
    ImGuiContext* .g = GImGui;
    //DockContextRebuild(ctx);
    DockContextBuildAddWindowsToNodes(.g, root_id);
}
