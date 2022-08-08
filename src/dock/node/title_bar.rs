use crate::dock::node;
use crate::dock::node::{dock_node_get_root_node, DockNode};
use crate::{window, Context, INVALID_ID};

// static bool IsDockNodeTitleBarHighlighted(ImGuiDockNode* node, ImGuiDockNode* root_node, Window* host_window)
pub fn is_dock_node_title_bar_highlighted(
    g: &mut Context,
    node: &mut DockNode,
    root_node: &mut DockNode,
    host_window: &mut window::Window,
) -> bool {
    // CTRL+Tab highlight (only highlighting leaf node, not whole hierarchy)
    // ImGuiContext& g = *GImGui;
    if g.nav_windowing_target_id != INVALID_ID {
        let nav_win_target = g.window_mut(g.nav_windowing_target_id);
        return nav_win_target.dock_node_id == node.id;
    }

    // FIXME-DOCKING: May want alternative to treat central node void differently? e.g. if (g.nav_window == host_window)
    if g.nav_window_id != INVALID_ID && g.window_mut(g.nav_window_id).root_window_for_title_bar_highlight_id == host_window.root_window_dock_tree_id && root_node.last_focused_node_id == node.id {
        // for (ImGuiDockNode* parent_node = g.nav_window.root_window.dock_node; parent_node != None; parent_node = parent_node.host_window ? parent_node.host_window.root_window.dock_node : None){
        let nav_win = g.window_mut(g.nav_window_id);

        let mut parent_node: Option<&mut DockNode> = g.dock_node_mut(nav_win.dock_node_id);
        while parent_node.is_some() {
            parent_node = dock_node_get_root_node(g, parent_node.unwrap());
            if parent_node.unwrap().id == root_node.id {
                return true;
            }
            // if ((parent_node = dock_node_get_root_node(parent_node)) == root_node) {
            //     return true;
            // }

            let parent_node_host_win = g.window_mut(parent_node.unwrap().host_window_id);
            parent_node = if parent_node.unwrap().host_window_id != INVALID_ID {
                g.dock_node_mut(parent_node_host_win.dock_node_id)
            } else {
                let root_win = g.window_mut(parent_node_host_win.root_window_id);
                g.dock_node_mut(root_win.dock_node_id);
            };
        }
    }
    return false;
}
