use crate::{Context, dock, INVALID_ID};
use crate::dock::node::DockNode;

// static void ImGui::dock_context_remove_node(ImGuiContext* ctx, ImGuiDockNode* node, bool merge_sibling_into_parent_node)
pub fn dock_context_remove_node(g: &mut Context, node: &mut DockNode, merge_sibling_into_parent_node: bool)
{
    // ImGuiContext& g = *ctx;
    // ImGuiDockContext* dc  = &ctx.DockContext;
    let dc = &mut g.dock_context;

    // IMGUI_DEBUG_LOG_DOCKING("[docking] dock_context_remove_node 0x%08X\n", node.ID);
    // IM_ASSERT(DockContextFindNodeByID(ctx, node.id) == node);
    // IM_ASSERT(node.ChildNodes[0] == NULL && node.ChildNodes[1] == NULL);
    // IM_ASSERT(node.Windows.size == 0);

    if node.host_window_id != INVALID_ID {
        let win = g.get_window(node.host_window_id);
        // node.host_window.dock_node_as_host = NULL;
        win.dock_node_as_host_id = None;
    }

    // ImGuiDockNode* parent_node = node.ParentNode;
    let parent_node = g.get_dock_node(node.parent_node_id);

    // const bool merge = (merge_sibling_into_parent_node && parent_node != NULL);
    let merge = merge_sibling_into_parent_node && parent_node.is_some();
    let parent_node_obj = parent_node.unwrap();
    if parent_node.is_some() {
        if merge {

            // IM_ASSERT(parent_node.ChildNodes[0] == node || parent_node.ChildNodes[1] == node);
            // ImGuiDockNode* sibling_node = (parent_node.ChildNodes[0] == node ? parent_node.ChildNodes[1] : parent_node.ChildNodes[0]);
            let sibling_node_id = if parent_node_obj.child_nodes[0] == node.id { parent_node_obj.child_nodes[1] } else { parent_node.child_nodes[0] };
            let sibling_node = g.get_dock_node(sibling_node_id);
            dock::dock_node_tree_merge(g, parent_node_obj, sibling_node);
        } else {

            // for (int n = 0; parent_node && n < IM_ARRAYSIZE(parent_node.ChildNodes); n += 1)
                // if (parent_node.ChildNodes[n] == node) {
                //    parent_node_obj.child_nodes.remove()

            // }

            parent_node_obj.child_nodes.retain( |child_node| child_node != node.id);

            // dc.Nodes.SetVoidPtr(node.ID, NULL);
            dc.nodes.retain(|x| x != node.id);
            // IM_DELETE(node);
            g.dock_nodes.retain(|dn| dn != node.id);
        }
    }
}
