use crate::dock::context::dock_context_remove_node;
use crate::dock::node;
use crate::dock::node::{tab_bar, DockNode};
use crate::frame::get_frame_height;
use crate::input::mouse::start_mouse_moving_window;
use crate::layout::same_line;
use crate::popup::{begin_popup, end_popup};
use crate::tab_bar::{TabBar, TabItemFlags};
use crate::types::{DataAuthority, Direction, Id32};
use crate::vectors::vector_2d::Vector2D;
use crate::window::lifecycle::update_window_parent_and_root_links;
use crate::window::next_window::set_next_window_pos;
use crate::window::pos::set_window_pos;
use crate::window::size::set_window_size;
use crate::window::{Window, WindowFlags};
use crate::{Context, INVALID_ID};

// static void dock_node_hide_window_during_host_window_creation(ImGuiWindow* window)
pub fn dock_node_hide_window_during_host_window_creation(g: &mut Context, window: &mut Window) {
    window.hidden = true;
    window.hidden_frames_can_skip_items = if window.active { 1 } else { 2 };
}

// static void DockNodeAddWindow(ImGuiDockNode* node, ImGuiWindow* window, bool add_to_tab_bar)
pub fn dock_node_add_window(
    g: &mut Context,
    node: &mut DockNode,
    window: &mut Window,
    add_to_tab_bar: bool,
) {
    // ImGuiContext& g = *GImGui; (void)g;
    if window.dock_node_id != INVALID_ID {
        // Can overwrite an existing window->dock_node (e.g. pointing to a disabled DockSpace node)
        // IM_ASSERT(window.dock_node.ID != node.ID);
        let dock_node_a = g.get_dock_node(window.dock_node_id);
        dock_node_remove_window(g, dock_node_a.unwrap(), window, 0);
    }
    // IM_ASSERT(window.dock_node == None || window.DockNodeAsHost == None);
    // IMGUI_DEBUG_LOG_DOCKING("[docking] DockNodeAddWindow node 0x%08X window '%s'\n", node.ID, window.Name);

    // If more than 2 windows appeared on the same frame leading to the creation of a new hosting window,
    // we'll hide windows until the host window is ready. Hide the 1st window after its been output (so it is not visible for one frame).
    // We will call dock_node_hide_window_during_host_window_creation() on ourselves in Begin()
    if node.host_window_id == INVALID_ID
        && node.windows.len() == 1
        && g.get_window(node.windows[0]).unwrap().was_active == false
    {
        dock_node_hide_window_during_host_window_creation(
            g,
            g.get_window(node.windows[0]).unwrap(),
        );
    }

    node.windows.push_back(window);
    node.want_hiddent_tab_bar_update = true;
    window.dock_node_id = node.id;
    window.dock_id = node.id;
    window.dock_is_active = (node.windows.len() > 1);
    window.dock_tab_want_close = false;

    // When reactivating a node with one or two loose window, the window pos/size/viewport are authoritative over the node storage.
    // In particular it is important we init the viewport from the first window so we don't create two viewports and drop one.
    if node.host_window_id == INVALID_ID && node.is_floating_node() {
        if node.authority_for_pos == DataAuthority::Auto {
            node.authority_for_pos = DataAuthority::Window;
        }
        if node.authority_for_size == DataAuthority::Auto {
            node.authority_for_size = DataAuthority::Window;
        }
        if node.authority_for_viewport == DataAuthority::Auto {
            node.authority_for_viewport = DataAuthority::Window;
        }
    }

    // Add to tab bar if requested
    if add_to_tab_bar {
        if node.tab_bar == None {
            tab_bar::dock_node_add_tab_bar(g, node);
            node.tab_bar.selected_tab_id = node.selected_tab_id;
            node.tab_bar.next_selected_tab_id = node.selected_tab_id;

            // Add existing windows
            // for (int n = 0; n < node.windows.len() - 1; n += 1){
            for win_id in node.windows.iter() {
                let win_a = g.get_window(*win_id);
                tab_bar_add_tab(g, &mut node.tab_bar, TabItemFlags::None, win_a);
            }
        }
        tab_bar_add_tab(&mut node.tab_bar, TabItemFlags::Unsorted, window);
    }

    node::dock_node_update_visible_flag(g, node);

    // Update this without waiting for the next time we Begin() in the window, so our host window will have the proper title bar color on its first frame.
    if node.host_window_id != INVALID_ID {
        let mut flags = window.flags.clone();
        flags.insert(WindowFlags::ChildWIndow);
        let mut parent_win = g.get_window(node.host_window_id);
        update_window_parent_and_root_links(g, window, &mut flags, Some(parent_win));
    }
}

// static void DockNodeRemoveWindow(ImGuiDockNode* node, ImGuiWindow* window, ImGuiID save_dock_id)
pub fn dock_node_remove_window(
    g: &mut Context,
    node: &mut DockNode,
    window: &mut Window,
    save_dock_id: Id32,
) {
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(window.dock_node == node);
    //IM_ASSERT(window->root_window_dock_tree == node->host_window);
    //IM_ASSERT(window->last_frame_active < g.frame_count);    // We may call this from Begin()
    // IM_ASSERT(save_dock_id == 0 || save_dock_id == node.ID);
    // IMGUI_DEBUG_LOG_DOCKING("[docking] DockNodeRemoveWindow node 0x%08X window '%s'\n", node.ID, window.Name);

    window.dock_node_id = INVALID_ID;
    window.dock_is_active = false;
    window.dock_tab_want_close = false;
    window.dock_id = save_dock_id;
    window.flags.remove(&WindowFlags::ChildWindow); // &= ~WindowFlags::ChildWindow;
    if window.parent_window_id != INVALID_ID {
        let mut parent_win = g.get_window(window.parent_window_id).unwrap();
        // window.parent_window.DC.ChildWindows.find_erase(window);
        parent_win.dc.child_windows.find_erase(window);
    }
    update_window_parent_and_root_links(g, window, &mut window.flags, None); // Update immediately

    // Remove window
    // bool erased = false;
    let mut erased = false;
    // for (int n = 0; n < node.windows.len(); n += 1){
    for win_id in node.windows.iter() {
        if win_id == window.id {
            // if (node.windows[n] == window) {
            node.windows.erase(node.windows.data + n);
            erased = true;
            break;
        }
    }
    if !erased {}
    // IM_ASSERT(erased);
    if node.visible_window_id == window.id {
        node.visible_window_id = INVALID_ID;
    }

    // Remove tab and possibly tab bar
    node.want_hiddent_tab_bar_update = true;
    if node.tab_bar.is_some() {
        tab_bar_remove_tab(&node.tab_bar, window.tab_id);
        // let tab_count_threshold_for_tab_bar = node.is_central_node() ? 1 : 2;
        let tab_count_threshold_for_tab_bar: i32 = if node.is_central_node() { 1 } else { 2 };
        if node.windows.len() < tab_count_threshold_for_tab_bar as usize {
            tab_bar::dock_node_remove_tab_bar(g, node);
        }
    }

    if node.windows.len() == 0
        && !node.is_central_node()
        && !node.is_dock_space()
        && window.dock_id != node.id
    {
        // Automatic dock node delete themselves if they are not holding at least one tab
        dock_context_remove_node(g, node, true);
        return;
    }

    if node.windows.len() == 1 && !node.is_central_node() && node.host_window_id != INVALID_ID {
        // ImGuiWindow* remaining_window = node.windows[0];
        let remaining_window = g.get_window(node.windows[0]);
        if node.host_window_id.viewport_owned && node.is_root_node() {
            // Transfer viewport back to the remaining loose window
            // IMGUI_DEBUG_LOG_VIEWPORT("[viewport] Node %08X transfer viewport %08X=>%08X for window '%s'\n", node.id, node.host_window_id.viewport.id, remaining_window.id, remaining_window.Name);
            // IM_ASSERT(node.host_window.viewport.Window == node.host_window);
            // node.host_window_id.viewport.Window = remaining_window;
            // node.host_window_id.viewport.id = remaining_window.id;
            let host_win = g.get_window(node.host_window_id);
            let vp_a = g.get_viewport(host_win.viewport_id).unwrap();
            vp_a.window_id = remaining_window.id;
            vp_a.id = remaining_window.id;
        }
        remaining_window.collapsed = node.host_window_id.collapsed;
    }

    // Update visibility immediately is required so the dock_node_updateRemoveInactiveChilds() processing can reflect changes up the tree
    node::dock_node_update_visible_flag(g, node);
}

// static void DockNodeMoveWindows(ImGuiDockNode* dst_node, ImGuiDockNode* src_node)
pub fn dock_node_move_windows(g: &mut Context, dst_node: &mut DockNode, src_node: &mut DockNode) {
    // Insert tabs in the same orders as currently ordered (node->windows isn't ordered)
    // IM_ASSERT(src_node && dst_node && dst_node != src_node);
    // ImGuiTabBar* src_tab_bar = src_node.tab_bar;
    let src_tab_bar = &mut src_node.tab_bar;
    if src_tab_bar.is_some() {}
    // IM_ASSERT(src_node.Windows.size <= src_node.TabBar.Tabs.size);

    // If the dst_node is empty we can just move the entire tab bar (to preserve selection, scrolling, etc.)
    let move_tab_bar = (src_tab_bar.is_some()) && (dst_node.tab_bar.is_none());
    if move_tab_bar {
        dst_node.tab_bar = src_node.tab_bar.clone();
        src_node.tab_bar = None;
    }

    // for (int n = 0; n < src_node.windows.len(); n += 1)
    let mut n = 0;
    for win_id in src_node.windows.iter() {
        // dock_node's tab_bar may have non-window Tabs manually appended by user
        let win = if src_tab_bar.is_some() {
            g.get_window(src_tab_bar.unwrap().tab[n].window_id)
        } else {
            g.get_window(src_node.windows[n])
        };
        // if (ImGuiWindow* window = src_tab_bar ? src_tab_bar.tabs[n].Window : src_node.windows[n])
        // {
        //     window.dock_node = None;
        //     window.dock_is_active = false;
        //     node::dock_node_add_window(dst_node, window, move_tab_bar ? false: true);
        // }
        win.dock_node_id = INVALID_ID;
        win.dock_is_active = false;
        dock_node_add_window(g, dst_node, window, if move_tab_bar { false } else { true });
        n += 1;
    }
    src_node.windows.clear();

    if !move_tab_bar && src_node.tab_bar.is_some() {
        if dst_node.tab_bar {
            dst_node.tab_bar.selected_tab_id = src_node.tab_bar.selected_tab_id;
        }
        tab_bar::dock_node_remove_tab_bar(g, src_node);
    }
}

// static void dock_node_hide_host_window(ImGuiDockNode* node)
pub fn dock_node_hide_host_window(g: &mut Context, node: &mut DockNode) {
    if node.host_window_id != INVALID_ID {
        let host_win = g.get_window(node.host_window_id);

        if host_win.dock_node_as_host_id == node.id {
            // node.host_window_id.dock_node_as_host = None;
            host_win.dock_node_as_host_id = INVALID_ID;
        }
        node.host_window_id = INVALID_ID;
    }

    if node.windows.len() == 1 {
        node.visible_window_id = node.windows[0];
        // node.windows[0].dock_is_active = false;
        g.get_window(node.windows[0]).dock_is_active = false;
    }

    if node.tab_bar.is_some() {
        tab_bar::dock_node_remove_tab_bar(g, node);
    }
}

// static void DockNodeApplyPosSizeToWindows(ImGuiDockNode* node)
pub fn dock_node_apply_pos_size_to_windows(g: &mut Context, node: &mut DockNode) {
    // for (int n = 0; n < node.windows.len(); n += 1)
    for win_id in node.windows.iter() {
        let node_win = g.get_window(*win_id);
        set_window_pos(g, node_win, &node.pos, Cond::Always); // We don't assign directly to pos because it can break the calculation of SizeContents on next frame
        set_window_size(g, node_win, &node.size, Cond::Always);
    }
}

// static ImGuiWindow* DockNodeFindWindowByID(ImGuiDockNode* node, ImGuiID id)
pub fn dock_node_find_window_by_id(
    g: &mut Context,
    node: &mut DockNode,
    id: Id32,
) -> Option<&mut Window> {
    // IM_ASSERT(id != 0);
    // for (int n = 0; n < node.windows.len(); n += 1){
    for win_id in node.windows.iter() {
        let win = g.get_window(*win_id);
        // if (node.windows[n].id == id)
        if win.id == id {
            return Some(win);
        }
    }
    return None;
}

// static void dock_node_start_mouse_moving_window(ImGuiDockNode* node, ImGuiWindow* window)
pub fn dock_node_start_mouse_moving_window(
    g: &mut Context,
    node: &mut DockNode,
    window: &mut Window,
) {
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(node.WantMouseMove == true);
    start_mouse_moving_window(g, window);
    g.active_id_click_offset = &g.io.mouse_clicked_pos[0] - &node.pos;
    g.moving_window = window; // If we are docked into a non moveable root window, start_mouse_moving_window() won't set g.moving_window. Override that decision.
    node.want_mouse_move = false;
}

// static void dock_node_setup_host_window(ImGuiDockNode* node, ImGuiWindow* host_window)
pub fn dock_node_setup_host_window(g: &mut Context, node: &mut DockNode, host_window: &mut Window) {
    // Remove ourselves from any previous different host window
    // This can happen if a user mistakenly does (see #4295 for details):
    //  - N+0: DockBuilderAddNode(id, 0)    // missing ImGuiDockNodeFlags_DockSpace
    //  - N+1: NewFrame()                   // will create floating host window for that node
    //  - N+1: DockSpace(id)                // requalify node as dockspace, moving host window
    if node.host_window_id != INVALID_ID
        && node.host_window_id != host_window.id
        && g.get_window(node.host_window_id).dock_node_as_host_id == node.id
    {
        g.get_window(node.host_window_id).dock_node_as_host_id = INVALID_ID;
    }

    host_window.dock_node_as_host_id = node.id;
    node.host_window_id = host_window.id;
}

// static ImGuiID dock_node_updateWindowMenu(ImGuiDockNode* node, ImGuiTabBar* tab_bar)
pub fn dock_node_update_window_menu(
    g: &mut Context,
    node: &mut DockNode,
    tab_bar: &mut TabBar,
) -> Id32 {
    // Try to position the menu so it is more likely to stays within the same viewport
    // ImGuiContext& g = *GImGui;
    let mut ret_tab_id: Id32 = INVALID_ID;
    if g.style.window_menu_button_position == Direction::Left {
        set_next_window_pos(
            g,
            &Vector2D::new(node.pos.x, node.pos.y + get_frame_height(g)),
            Cond::Always,
            Some(Vector2D::new(0.0, 0.0)),
        );
    } else {
        set_next_window_pos(
            g,
            &Vector2D::new(node.pos.x + node.size.x, node.pos.y + get_frame_height(g)),
            Cond::Always,
            Some(Vector2D::new(1.0, 0.0)),
        );
    }
    if begin_popup(g, "#WindowMenu", None) {
        node.is_focused = true;
        if tab_bar.tabs.size == 1 {
            if menu_item("Hide tab bar", None, node.is_hidden_tab_bar()) {
                node.want_hidden_tab_bar_toggle = true;
            }
        } else {
            // for (int tab_n = 0; tab_n < tab_bar.tabs.size; tab_n += 1)
            for tab_n in 0..tab_bar.tabs.len() {
                let tab = &tab_bar.tabs[tab_n];
                if tab.flags.contains(&TabItemFlags::Button) {
                    continue;
                }
                if selectable(tab_bar.get_tab_name(tab), tab.id == tab_bar.selected_tab_id) {
                    ret_tab_id = tab.id;
                }
                same_line(g, 0f32, 0f32);
                text("   ");
            }
        }
        end_popup(g);
    }
    return ret_tab_id;
}
