pub mod preview;
pub mod rect;
pub mod tab_bar;
pub mod title_bar;
pub mod tree;
pub mod window;
pub mod dock_node;
pub mod dock_node_flags;
pub mod dock_node_state;
pub mod dock_node_settings;

use crate::axis::Axis;
use crate::button::ButtonFlags;
use crate::color::StyleColor;
use crate::condition::Condition;
use crate::context::Context;
use crate::dock::context::{dock_context_add_node, dock_context_remove_node};
use crate::dock::dock_style_color::{DOCKING_SPLITTER_SIZE, WindowDockStyleColor};
use crate::dock::node::rect::{
    dock_node_calc_drop_rects_and_test_mouse_pos, dock_node_calc_split_rects,
};
use crate::dock::node::window::{dock_node_apply_pos_size_to_windows, dock_node_move_windows, dock_node_remove_window};
use crate::dock::ops::begin_dockable_drag_drop_target;
use crate::dock::preview::DockPreviewData;
use crate::dock::settings::dock_settings_rename_node_references;
use crate::dock::{int, node, settings};
use crate::drag_drop::get_drag_drop_payload;
use crate::draw::list::foreground_draw_list;
use crate::frame::get_frame_height;
use crate::input::mouse::{
    is_mouse_clicked, start_mouse_moving_window, start_mouse_moving_window_or_node,
};
use crate::input::{MouseButton, NavLayer};
use crate::item::{is_item_active, ItemFlags, pop_item_flag, push_item_flag};
use crate::layout::same_line;
use crate::math::saturate_f32;
use crate::nav::nav_init_window;
use crate::nodes::{pop_style_var, push_style_float, push_style_vector2d};
use crate::payload::PayloadDataType;
use crate::popup::{begin_popup, end_popup, is_popup_open, open_popup};
use crate::rect::Rect;
use crate::settings::mark_ini_settings_dirty;
use crate::style::{color_convert_u32_to_float4, color_u32_from_style_color, get_color_u32_no_alpha, pop_style_color, push_style_color, StyleVar, WINDOW_DOCK_STYLE_COLORS};
use crate::tab_bar::{TabBar, TabBarFlags, TabItem, TabItemFlags};
use crate::types::Id32;
use crate::types::INVALID_ID;
use crate::types::{DataAuthority, Direction, DIRECTIONS};
use crate::utils::{add_hash_set, extend_hash_set};
use crate::vectors::vector_2d::Vector2D;
use crate::vectors::{vec_length_sqr, Vector4D};
use crate::window::class::WindowClass;
use crate::window::current::{get_id, pop_id, push_id, push_id2, push_override_id};
use crate::window::get::is_window_within_begin_stack_of;
use crate::window::layer::{bring_window_to_display_front, focus_window};
use crate::window::lifecycle::{begin, end, update_window_parent_and_root_links};
use crate::window::next_window::{
    set_next_window_class, set_next_window_collapsed, set_next_window_pos, set_next_window_size,
    set_next_window_viewport,
};
use crate::window::pos::set_window_pos;
use crate::window::render::render_window_outer_borders;
use crate::window::size::set_window_size;
use crate::window::state::set_window_hit_test_hole;
use crate::window::{
    Window, WindowDockStyle, WindowFlags, WINDOWS_HOVER_PADDING,
    WINDOWS_RESIZE_FROM_EDGES_FEEDBACK_TIMER,
};
use crate::{dock, hash_string};
use std::borrow::{Borrow, BorrowMut};
use std::collections::HashSet;
use std::ops::BitOr;
pub use dock_node::DockNode;
pub use dock_node_flags::DockNodeFlags;
pub use dock_node_state::DockNodeState;
use crate::dock::node::tree::dock_node_tree_update_pos_size;
use crate::render::render_rect_filled_with_hole;

pub fn dock_node_get_root_node(
    g: &mut Context,
    node: &mut DockNode,
) -> Option<&mut DockNode> {
    if node.parent_node_id == INVALID_ID {
        return None;
    }

    let mut out_node: Option<&mut DockNode> = None;
    let mut parent_node_id = node.parent_node_id;
    while parent_node_id != INVALID_ID {
        let next_node = g.dock_node_mut(parent_node_id).unwrap();
        parent_node_id = next_node.parent_node_id;
        out_node = Some(next_node);
    }
    return out_node;
}

pub fn dock_node_comparer_depth_most_first(
    g: &mut Context,
    lhs: &dock_node,
    rhs: &dock_node,
) -> i32 {
    // return dock_node_get_depth(b) - dock_node_get_depth(a);
    todo!()
}

// int dock_node_get_tab_order(Window* window)
pub fn dock_node_get_tab_order(g: &mut Context, window: &mut Window) -> i32 {
    // ImGuiTabBar* tab_bar = window.dock_node_id.tab_bar;
    let tab_bar = &mut g.dock_node_mut(window.dock_node_id).unwrap().tab_bar;
    // if (tab_bar == None)
    //     return -1
    if tab_bar == INVALID_ID {
        return -1;
    }
    // ImGuiTabItem* tab = TabBarFindTabByID(tab_bar, window.tab_id);
    let tab = tab_bar_find_tab_by_id(g, tab_bar, window.tab_id);
    // return tab ? tab_bar.GetTabOrder(tab) : -1;
    return if tab != INVALID_ID {
        tab_bar.get_tab_order(g, tab)
    } else {
        -1
    };
}

// static void dock_node_move_child_nodes(ImGuiDockNode* dst_node, ImGuiDockNode* src_node)
pub fn dock_node_move_child_nodes(g: &mut Context, dst_node: &mut DockNode, src_node: &DockNode) {
    // IM_ASSERT(dst_node.Windows.size == 0);
    dst_node.child_nodes[0] = src_node.child_nodes[0];
    dst_node.child_nodes[1] = src_node.child_nodes[1];
    if dst_node.child_nodes[0] != INVALID_ID {
        // dst_node.child_nodes[0].parent_node = dst_node;
        let node = g.dock_node_mut(dst_node.child_nodes[0]).unwrap();
        node.parent_node_id = dst_node.id;
    }
    if dst_node.child_nodes[1] {
        // dst_node.child_nodes[1].parent_node = dst_node;
        let node = g.dock_node_mut(dst_node.child_nodes[1]).unwrap();
        node.parent_node_id = dst_node.id;
    }
    dst_node.split_axis = src_node.split_axis.clone();
    dst_node.size_ref = src_node.size_ref.clone();
    src_node.child_nodes[0] = INVALID_ID;
    src_node.child_nodes[1] = INVALID_ID;
}

// Search function called once by root node in dock_node_update()
#[derive(Default, Debug, Clone)]
pub struct DockNodeTreeInfo {
    // ImGuiDockNode*      CentralNode;
    pub central_node: Id32,
    // ImGuiDockNode*      first_node_with_windows;
    pub first_node_with_windows: Id32,
    // int                 count_nodes_with_windows;
    pub count_nodes_with_windows: i32,
    //window_class  WindowClassForMerges;
    pub window_class_for_merges: WindowClass,
    // ImGuiDockNodeTreeInfo() { memset(this, 0, sizeof(*this)); }
}

// static void dock_node_find_info(ImGuiDockNode* node, ImGuiDockNodeTreeInfo* info)
pub fn dock_node_find_info(g: &mut Context, node: &mut DockNode, info: &mut DockNodeTreeInfo) {
    if node.windows.len() > 0 {
        if info.first_node_with_windows == INVALID_ID {
            info.first_node_with_windows = node.id;
        }
        info.count_nodes_with_windows += 1;
    }
    if node.is_central_node() {
        // IM_ASSERT(info.central_node == None); // Should be only one
        // IM_ASSERT(node.IsLeafNode() && "If you get this assert: please submit .ini file + repro of actions leading to this.");
        info.central_node = node.id;
    }
    if info.count_nodes_with_windows > 1 && info.central_node != INVALID_ID {
        return;
    }
    if node.child_nodes[0] != INVALID_ID {
        let node = g.dock_node_mut(node.child_nodes[0]).unwrap();
        dock_node_find_info(g, node, info);
    }
    if node.child_nodes[1] != INVALID_ID {
        let node = g.dock_node_mut(node.child_nodes[1]).unwrap();
        dock_node_find_info(g, node, info);
    }
}

// - Remove inactive windows/nodes.
// - update visibility flag.
// static void dock_node_update_flags_and_collapse(ImGuiDockNode* node)
pub fn dock_node_update_flags_and_collapse(g: &mut Context, node: &mut DockNode) {
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(node.ParentNode == None || node.ParentNode.ChildNodes[0] == node || node.ParentNode.ChildNodes[1] == node);

    // Inherit most flags
    if node.parent_node_id != INVALID_ID {
        let parent_node = g.dock_node_mut(node.parent_node_id).unwrap();
        let mut flags_to_set = parent_node.shared_flags.clone();
        flags_to_set.insert(DockNodeFlags::SharedFlagsInheritMask);
        node.shared_flags = flags_to_set;
    }

    // Recurse into children
    // There is the possibility that one of our child becoming empty will delete itself and moving its sibling contents into 'node'.
    // If 'node->ChildNode[0]' delete itself, then 'node->ChildNode[1]->windows' will be moved into 'node'
    // If 'node->ChildNode[1]' delete itself, then 'node->ChildNode[0]->windows' will be moved into 'node' and the "remove inactive windows" loop will have run twice on those windows (harmless)
    node.has_central_node_child = false;
    if node.child_nodes[0] != INVALID_ID {
        dock_node_update_flags_and_collapse(g, g.dock_node_mut(node.child_nodes[0]).unwrap());
    }
    if node.child_nodes[1] != INVALID_ID {
        dock_node_update_flags_and_collapse(g, g.dock_node_mut(node.child_nodes[1]).unwrap());
    }

    // Remove inactive windows, collapse nodes
    // merge node flags overrides stored in windows
    // node.local_flags_in_windows = DockNodeFlags::None;
    node.local_flags_in_windows.clear();
    // for (int window_n = 0; window_n < node.windows.len(); window_n += 1)
    for win_id in node.windows.iter() {
        // Window* window = node.windows[window_n];
        let window = g.window_mut(*win_id);
        // IM_ASSERT(window.dock_node == node);

        let node_was_active = (node.last_frame_active + 1 == g.frame_count);
        let mut remove = false;
        remove |= node_was_active && (window.last_frame_active + 1 < g.frame_count);
        remove |= node_was_active && (node.want_close_all || node.want_close_tab_id == window.tab_id) && window.has_close_button && !(window.flags.contains(&WindowFlags::UnsavedDocument)); // Submit all _expected_ closure from last frame
        remove |= (window.dock_tab_want_close);
        if remove {
            window.dock_tab_want_close = false;
            if node.windows.len() == 1 && !node.is_central_node() {
                window::dock_node_hide_host_window(g, node);
                node.State = DockNodeState::HostWindowHiddenBecauseSingleWindow;
                dock_node_remove_window(g, node, window, node.id); // Will delete the node so it'll be invalid on return
                return;
            }
            dock_node_remove_window(g, node, window, node.id);
            window_n -= 1;
            continue;
        }

        // FIXME-DOCKING: Missing policies for conflict resolution, hence the "Experimental" tag on this.
        //node->LocalFlagsInWindow &= ~window->window_class.DockNodeFlagsOverrideClear;
        // node.local_flags_in_windows |= window.window_class.dock_node_flags_override_set;
        node.local_flags_in_windows = add_hash_set(
            &node.local_flags_in_windows,
            &window.window_class.dock_node_flags_override_set,
        )
    }
    node.update_merged_flags();

    // Auto-hide tab bar option
    // ImGuiDockNodeFlags node_flags = node.merged_flags;
    let node_flags = node.merged_flags.clone();
    if node.want_hiddent_tab_bar_update && node.windows.len() == 1 && (node_flags.contains(&DockNodeFlags::AutoHideTabBar)) && !node.is_hidden_tab_bar() {
        node.want_hidden_tab_bar_toggle = true;
    }
    node.want_hiddent_tab_bar_update = false;

    // Cancel toggling if we know our tab bar is enforced to be hidden at all times
    if node.want_hidden_tab_bar_toggle && node.visible_window_id != INVALID_ID && (node.visible_window_id.window_class.dock_node_flags_override_set.contains(&DockNodeFlags::HiddenTabBar)) {
        node.want_hidden_tab_bar_toggle = false;
    }

    // Apply toggles at a single point of the frame (here!)
    if node.windows.len() > 1 {
        let mut flags_to_set = node.local_flags.clone();
        flags_to_set.remove(&DockNodeFlags::HiddenTabBar);
        node.set_local_flags(&flags_to_set);
    } else if node.want_hidden_tab_bar_toggle {
        // node.set_local_flags(node.LocalFlags ^ DockNodeFlags::HiddenTabBar);
        let mut flags_to_set = node.local_flags.clone();
        flags_to_set.insert(DockNodeFlags::HiddenTabBar);
        node.set_local_flags(&flags_to_set);
    }
    node.want_hidden_tab_bar_toggle = false;

    dock_node_update_visible_flag(g, node);
}

// This is rarely called as dock_node_update_for_root_node() generally does it most frames.
// static void dock_node_update_has_central_node_child(ImGuiDockNode* node)
pub fn dock_node_update_has_central_node_child(g: &mut Context, node: &mut DockNode) {
    node.has_central_node_child = false;
    if node.child_nodes[0] != INVALID_ID {
        dock_node_update_has_central_node_child(g, g.dock_node_mut(node.child_nodes[0]).unwrap());
    }
    if node.child_nodes[1] != INVALID_ID {
        dock_node_update_has_central_node_child(g, g.dock_node_mut(node.child_nodes[1]).unwrap());
    }
    if node.is_root_node() {
        // ImGuiDockNode* mark_node = node.central_node;
        let mut mark_node_id = node.central_node_id;
        while mark_node_id != INVALID_ID {
            let mut mark_node = g.dock_node_mut(mark_node_id).unwrap();
            mark_node.has_central_node_child = true;
            // mark_node = mark_node.parent_node;
            mark_node_id = mark_node.parent_node_id;
        }
    }
}

// static void dock_node_update_visible_flag(ImGuiDockNode* node)
pub fn dock_node_update_visible_flag(g: &mut Context, node: &mut DockNode) {
    // update visibility flag
    let mut is_visible = if node.parent_node_id == INVALID {
        node.is_dock_space()
    } else {
        node.is_central_node()
    };
    is_visible |= (node.windows.len() > 0);
    is_visible |= (node.child_nodes[0] != INVALID_ID && g.dock_node_mut(node.child_nodes[0]).unwrap().is_visible);
    is_visible |= (node.child_nodes[1] != INVALID_ID && g.dock_node_mut(node.child_nodes[1]).unwrap().is_visible);
    node.is_visible = is_visible;
}

// update central_node, only_node_with_windows, LastFocusedNodeID. Copy window class.
// static void dock_node_update_for_root_node(ImGuiDockNode* node)
pub fn dock_node_update_for_root_node(g: &mut Context, node: &mut DockNode) {
    dock_node_update_flags_and_collapse(g, node);

    // - Setup central node pointers
    // - Find if there's only a single visible window in the hierarchy (in which case we need to display a regular title bar -> FIXME-DOCK: that last part is not done yet!)
    // Cannot merge this with dock_node_update_flags_and_collapse() because first_node_with_windows is found after window removal and child collapsing
    // ImGuiDockNodeTreeInfo info;
    let mut info: DockNodeTreeInfo = DockNodeTreeInfo::default();
    dock_node_find_info(g, node, &mut info);
    node.central_node_id = info.central_node;
    node.only_node_with_windows = if info.count_nodes_with_windows == 1 {
        info.first_node_with_windows
    } else {
        INVALID_ID
    };
    node.count_node_with_windows = info.count_nodes_with_windows;
    if node.last_focused_node_id == INVALID_ID && info.first_node_with_windows != INVALID_ID {
        node.last_focused_node_id = info.first_node_with_windows.id;
    }

    // Copy the window class from of our first window so it can be used for proper dock filtering.
    // When node has mixed windows, prioritize the class with the most constraint (docking_allow_unclassed = false) as the reference to copy.
    // FIXME-DOCK: We don't recurse properly, this code could be reworked to work from dock_node_updateScanRec.
    // if ImGuiDockNode* first_node_with_windows = info.first_node_with_windows
    let first_node_with_windows = g.dock_node_mut(info.first_node_with_windows);
    if first_node_with_windows.is_some() {
        let first_node: &mut DockNode = first_node_with_windows.some();
        let win0 = g.window_mut(first_node.windows[0]);
        node.window_class = win0.window_class.clone();
        // for (int n = 1; n < first_node_with_windows.windows.len(); n += 1)
        for win_id in first_node.windows.iter() {
            let win = g.window_mut(*win_id);
            if win.window_class.docking_allow_unclassed == false {
                node.window_class = win.window_class.clone();
                break;
            }
        }
    }

    // ImGuiDockNode* mark_node = node.central_node_id;
    let mut mark_node_id = node.central_node_id;
    while mark_node_id {
        let mark_node = g.dock_node_mut(mark_node_id).unwrap();
        mark_node.has_central_node_child = true;
        mark_node_id = mark_node.parent_node_id;
    }
}

// static void dock_node_update(ImGuiDockNode* node)
pub fn dock_node_update(g: &mut Context, node: &mut DockNode) {
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(node.LastFrameActive != g.frame_count);
    node.last_frame_alive = g.frame_count;
    node.is_bg_drawn_this_frame = false;

    node.central_node_id = INVALID_ID;
    node.only_node_with_windows = INVALID_ID;
    if node.is_root_node() {
        dock_node_update_for_root_node(g, node);
    }

    // Remove tab bar if not needed
    if node.tab_bar.is_some() && node.is_no_tab_bar() {
        tab_bar::dock_node_remove_tab_bar(g, node);
    }

    // Early out for hidden root dock nodes (when all dock_id references are in inactive windows, or there is only 1 floating window holding on the dock_id)
    let mut want_to_hide_host_window = false;
    if node.is_floating_node() {
        if node.windows.len() <= 1 && node.is_leaf_node() {
            if !g.io.config_docking_always_tab_bar && (node.windows.len() == 0 || !node.windows[0].window_class.docking_always_tab_bar) {
                want_to_hide_host_window = true;
            }
        }
        if node.count_node_with_windows == 0 {
            want_to_hide_host_window = true;
        }
    }
    if want_to_hide_host_window {
        if node.windows.len() == 1 {
            // Floating window pos/size is authoritative
            let single_window = g.window_mut(node.windows[0]);
            node.pos = single_window.pos.clone();
            node.size = single_window.size_full.clone();
            node.authority_for_pos = DataAuthority::Window;
            node.authority_for_size = DataAuthority::Window;
            node.authority_for_viewport = DataAuthority::Window;

            // Transfer focus immediately so when we revert to a regular window it is immediately selected
            if node.host_window_id != INVALID_ID && g.nav_window.id == node.host_window_id {
                focus_window(g, single_window);
            }
            if node.host_window_id != INVALID_ID {
                single_window.viewport = node.host_window_id.viewport;
                single_window.viewport_id = node.host_window_id.viewport_id;
                if node.host_window_id.viewport_owned {
                    single_window.viewport.Window = single_window;
                    single_window.viewport_owned = true;
                }
            }
        }

        window::dock_node_hide_host_window(g, node);
        node.State = DockNodeState::HostWindowHiddenBecauseSingleWindow;
        node.want_close_all = false;
        node.want_close_tab_id = 0;
        node.has_close_button = false;
        node.hash_window_menu_button = false;
        node.last_frame_active = g.frame_count;

        if nodewant_mouse_move && node.windows.len() == 1 {
            window::dock_node_start_mouse_moving_window(g, node, g.window_mut(node.windows[0]));
        }
        return;
    }

    // In some circumstance we will defer creating the host window (so everything will be kept hidden),
    // while the expected visible window is resizing itself.
    // This is important for first-time (no ini settings restored) single window when io.config_docking_always_tab_bar is enabled,
    // otherwise the node ends up using the minimum window size. Effectively those windows will take an extra frame to show up:
    //   N+0: Begin(): window created (with no known size), node is created
    //   N+1: dock_node_update(): node skip creating host window / Begin(): window size applied, not visible
    //   N+2: dock_node_update(): node can create host window / Begin(): window becomes visible
    // We could remove this frame if we could reliably calculate the expected window size during node update, before the Begin() code.
    // It would require a generalization of CalcWindowExpectedSize(), probably extracting code away from Begin().
    // In reality it isn't very important as user quickly ends up with size data in .ini file.
    if node.is_visible && node.host_window_id == INVALID_ID && node.is_floating_node() && node.is_leaf_node() {
        // IM_ASSERT(node.Windows.size > 0);
        // Window* ref_window = None;

        let mut ref_window: Option<&mut Window> = None;
        if node.selected_tab_id != INVALID_ID {
            // Note that we prune single-window-node settings on .ini loading, so this is generally 0 for them!
            ref_window = window::dock_node_find_window_by_id(g, node, node.selected_tab_id);
        }
        if ref_window.is_none() {
            ref_window = Some(g.window_mut(node.windows[0]));
        }
        if ref_window.unwrap().auto_fit_frames_x > 0 || ref_window.unwrap().auto_fit_frames_y > 0 {
            node.State = DockNodeState::HostWindowHiddenBecauseWindowsAreResizing;
            return;
        }
    }

    let mut node_flags = node.merged_flags.clone();

    // Decide if the node will have a close button and a window menu button
    node.hash_window_menu_button = (node.windows.len() > 0) && node_flags.contains(&DockNodeFlags::NoWindowMenuButton) == false;
    node.has_close_button = false;
    // for (int window_n = 0; window_n < node.windows.len(); window_n += 1)
    for window_n in 0..node.windows.len() {
        // FIXME-DOCK: Setting dock_is_active here means that for single active window in a leaf node, dock_is_active will be cleared until the next Begin() call.
        // Window* window = node.windows[window_n];
        let window = g.window_mut(node.windows[window_n]);
        node.has_close_button = window.has_close_button;
        window.dock_is_active = (node.windows.len() > 1);
    }
    if node_flags.contains(&DockNodeFlags::NoCloseButton) {
        node.has_close_button = false;
    }

    // Bind or create host window
    // Window* host_window = None;
    let mut host_window: Option<&mut Window> = None;
    let mut beginned_into_host_window = false;
    if node.is_dock_space() {
        // [Explicit root dockspace node]
        // IM_ASSERT(node.host_window);
        host_window = Some(g.window_mut(node.host_window_id));
    } else {
        // [Automatic root or child nodes]
        if node.is_root_node() && node.is_visible {
            let mut ref_window = if node.windows.len() > 0 {
                Some(g.window_mut(node.windows[0]))
            } else {
                None
            };

            // Sync pos
            if node.authority_for_pos == DataAuthority::Window && ref_window.is_some() {
                set_next_window_pos(g, &ref_window.unwrap().pos, Condition::None, None);
            } else if node.authority_for_pos == DataAuthority::DockNode {
                set_next_window_pos(g, &node.pos, Condition::None, None);
            }

            // Sync size
            if node.authority_for_size == DataAuthority::Window && ref_window.is_some() {
                set_next_window_size(g, &ref_window.unwrap().size_full, Condition::None);
            } else if node.authority_for_size == DataAuthority::DockNode {
                set_next_window_size(g, &node.size, Condition::None);
            }

            // Sync collapsed
            if node.authority_for_size == DataAuthority::Window && ref_window.is_some() {
                set_next_window_collapsed(g, ref_window.unwrap().collapsed, Condition::None);
            }

            // Sync viewport
            if node.authority_for_viewport == DataAuthority::Window && ref_window.is_some() {
                set_next_window_viewport(g, ref_window.viewport_id);
            }

            set_next_window_class(g, &mut node.window_class);

            // Begin into the host window
            // char window_label[20];
            let mut window_label = String::new();
            dock_node_get_host_window_title(node, window_label, window_label.len());
            // WindowFlags window_flags = WindowFlags::NoScrollbar | WindowFlags::NoScrollWithMouse | WindowFlags::DockNodeHost;
            let mut window_flags = HashSet::from([
                WindowFlags::NoScrollbar,
                WindowFlags::NoScrollWithMouse,
                WindowFlags::DockNodeHost,
                WindowFlags::NoFocusOnAppearing,
                WindowFlags::NoSavedSettings,
                WindowFlags::NoNavFocus,
                WindowFlags::NoCollapse,
                WindowFlags::NoTitleBar,
            ]);
            // window_flags |= WindowFlags::NoFocusOnAppearing;
            // window_flags |= WindowFlags::NoSavedSettings | WindowFlags::NoNavFocus | WindowFlags::NoCollapse;
            // window_flags |= WindowFlags::NoTitleBar;

            set_netxt_window_bg_alpha(0.0); // Don't set WindowFlags_NoBackground because it disables borders
            push_style_vector2d(g, StyleVar::WindowPadding, &Vector2D::default());
            begin(g, window_label.as_str(), None, Some(&mut window_flags));
            pop_style_var(g, 0);
            beginned_into_host_window = true;

            host_window = Some(g.window_mut(g.current_window_id));
            window::dock_node_setup_host_window(g, node, host_window.unwrap());
            host_window.dc.cursor_pos = host_window.pos;
            node.pos = host_window.pos;
            node.size = host_window.size;

            // We set WindowFlags_NoFocusOnAppearing because we don't want the host window to take full focus (e.g. steal nav_window)
            // But we still it bring it to the front of display. There's no way to choose this precise behavior via window flags.
            // One simple case to ponder if: window A has a toggle to create windows B/C/D. Dock B/C/D together, clear the toggle and enable it again.
            // When reappearing B/C/D will request focus and be moved to the top of the display pile, but they are not linked to the dock host window
            // during the frame they appear. The dock host window would keep its old display order, and the sorting in EndFrame would move B/C/D back
            // after the dock host window, losing their top-most status.
            if host_window.unwrap().appearing {
                bring_window_to_display_front(g, host_window.unwrap());
            }

            node.authority_for_pos = DataAuthority::Auto;
            node.authority_for_size = DataAuthority::Auto;
            node.authority_for_viewport = DataAuthority::Auto;
        } else if node.parent_node_id != INVALID_ID {
            let parent_node = g.dock_node_mut(node.parent_node_id);
            host_window = Some(g.window_mut(parent_node.unwrap().host_window_id));
            node.host_window_id = host_window.unwrap().id;
            //host_window = node.parent_node.host_window;
            node.authority_for_pos = DataAuthority::Auto;
            node.authority_for_size = DataAuthority::Auto;
            node.authority_for_viewport = DataAuthority::Auto;
        }
        if nodewant_mouse_move && node.host_window_id != INVALID_ID {
            let win = g.window_mut(node.host_window_id);
            window::dock_node_start_mouse_moving_window(g, node, win);
        }
    }

    // update focused node (the one whose title bar is highlight) within a node tree
    if node.is_split_node() {}
    // IM_ASSERT(node.TabBar == None);
    if node.is_root_node() {
        // if g.nav_window_id != INVALID_ID && nav_win.root_window.dock_node && g.nav_window.root_window.parent_window == host_window {
        //     node.last_focused_node_id = g.nav_window.root_window.dock_node.id;
        // }
        if g.nav_window_id != INVALID_ID {
            let nav_win = g.window_mut(g.nav_window_id);
            if nav_win.root_window != INVALID_ID {
                let nav_win_root_win = g.window_mut(nav_win.root_window_id);
                if nav_win_root_win.dock_node_id != INVALID_ID && nav_win_root_win.parent_window_id != INVALID_ID {
                    let nav_win_root_win_dock_node = g.dock_node_mut(nav_win_root_win.dock_node_id);
                    // let nav_win_root_win_parent_win =
                    //     g.window_mut(nav_win_root_win.parent_window_id);
                    node.last_focused_node_id = nav_win_root_win_dock_node.id;
                }
            }
        }
    }

    // Register a hit-test hole in the window unless we are currently dragging a window that is compatible with our dockspace
    // ImGuiDockNode* central_node = node.central_node_id;
    let central_node = g.dock_node_mut(node.central_node_id);
    let central_node_hole = node.is_root_node() && host_window.is_some() && node_flags.contains(&DockNodeFlags::PassthruCentralNode) && central_node.is_some() && central_node.unwrap().is_empty();
    let mut central_node_hole_register_hit_test_hole = central_node_hole;
    if central_node_hole {
        let payload = get_drag_drop_payload(g);
        // if (const ImGuiPayload* payload = GetDragDropPayload()) {
        if payload.is_some() {
            if payload.data_type == PayloadDataType::Window && dock_node_is_drop_allowed(g, host_window.unwrap(), &mut payload.data.win) {
                central_node_hole_register_hit_test_hole = false;
            }
        }
    }
    if central_node_hole_register_hit_test_hole {
        // We add a little padding to match the "resize from edges" behavior and allow grabbing the splitter easily.
        // (But we only add it if there's something else on the other side of the hole, otherwise for e.g. fullscreen
        // covering passthru node we'd have a gap on the edge not covered by the hole)
        // IM_ASSERT(node.is_dock_space()); // We cannot pass this flag without the DockSpace() api. Testing this because we also setup the hole in host_window->parent_node
        // ImGuiDockNode* root_node = dock_node_get_root_node(central_node);
        let root_node = dock_node_get_root_node(g, central_node.unwrap());
        // Rect root_rect(root_node.pos, root_node.pos + root_node.size);
        let mut root_rect = Rect::from((&root_node.pos, &(&root_node.pos + &root_node.size)));
        // Rect hole_rect(central_node.pos, central_node.pos + central_node.size);
        let mut hole_rect = Rect::from((&central_node.pos, &(&central_node.pos + &central_node.size)));
        if hole_rect.min.x > root_rect.min.x {
            hole_rect.min.x += WINDOWS_HOVER_PADDING;
        }
        if hole_rect.max.x < root_rect.max.x {
            hole_rect.max.x -= WINDOWS_HOVER_PADDING;
        }
        if hole_rect.min.y > root_rect.min.y {
            hole_rect.min.y += WINDOWS_HOVER_PADDING;
        }
        if hole_rect.max.y < root_rect.max.y {
            hole_rect.max.y -= WINDOWS_HOVER_PADDING;
        }
        //GetForegroundDrawList()->add_rect(hole_rect.min, hole_rect.max, IM_COL32(255, 0, 0, 255));
        if central_node_hole && !hole_rect.is_inverted() {
            set_window_hit_test_hole(
                g,
                host_window.unwrap(),
                &hole_rect.min,
                &(&hole_rect.max - &hole_rect.min),
            );
            if host_window.unwrap().parent_window_id != INVALID_ID {
                set_window_hit_test_hole(
                    g,
                    g.window_mut(host_window.unwrap().parent_window_id),
                    &hole_rect.min,
                    &(&hole_rect.max - &hole_rect.min),
                );
            }
        }
    }

    // update position/size, process and draw resizing splitters
    if node.is_root_node() && host_window.is_some() {
        host_window.unwrap().draw_list.channels_set_current(1);
        dock_node_tree_update_pos_size(
            g,
            node,
            &host_window.unwrap().pos,
            &host_window.unwrap().size,
            None,
        );
        dock_node_tree_update_splitter(g, node);
    }

    // Draw empty node background (currently can only be the Central Node)
    if host_window.is_some() && node.is_empty() && node.is_visible {
        host_window.unwrap().draw_list.channels_set_current(0);
        node.last_bg_color = if node_flags.contains(&DockNodeFlags::PassthruCentralNode) {
            0
        } else {
            color_u32_from_style_color(g, StyleColor::DockingEmptyBg);
        };
        if node.last_bg_color != 0 {
            host_window.unwrap().draw_list.add_rect_filled(
                &node.pos,
                &(&node.pos + &node.size),
                &node.last_bg_color,
            );
        }
        node.is_bg_drawn_this_frame = true;
    }

    // Draw whole dockspace background if ImGuiDockNodeFlags_PassthruCentralNode if set.
    // We need to draw a background at the root level if requested by ImGuiDockNodeFlags_PassthruCentralNode, but we will only know the correct pos/size
    // _after_ processing the resizing splitters. So we are using the draw_list channel splitting facility to submit drawing primitives out of order!
    let render_dockspace_bg = node.is_root_node() && host_window.is_some() && (node_flags.contains(&DockNodeFlags::PassthruCentralNode));
    if render_dockspace_bg && node.is_visible {
        host_window.draw_list.channels_set_current(0);
        let draw_list = g.draw_list_mut(host_window.unwrap().draw_list_id);
        if central_node_hole {
            render_rect_filled_with_hole(
                draw_list,
                &node.rect(),
                central_node.rect(),
                get_color_u32_no_alpha(StyleColor::WindowBg),
                0.0,
            );
        } else {
            host_window.draw_list.add_rect_filled(
                &node.pos,
                &(&node.pos + &node.size),
                get_color_u32_no_alpha(StyleColor::WindowBg),
                0.0,
            );
        }
    }

    // Draw and populate Tab Bar
    if host_window.is_some() {
        host_window.unwrap().draw_list.channels_set_current(1);
    }
    if host_window.is_some() && node.windows.len() > 0 {
        dock_node_updateTabBar(node, host_window);
    } else {
        node.want_close_all = false;
        node.want_close_tab_id = 0;
        node.is_focused = false;
    }
    if node.tab_bar.is_some() && node.tab_bar.unwrap().selected_tab_id != INVALID_ID {
        node.selected_tab_id = node.tab_bar.unwrap().selected_tab_id;
    } else if node.windows.len() > 0 {
        node.selected_tab_id = node.windows[0].id;
    }

    // Draw payload drop target
    if host_window.is_some() && node.is_visible {
        if node.is_root_node() && (g.moving_window_id == INVALID_ID || g.window_mut(g.moving_window_id).root_window_dock_tree_id != host_window.unwrap().id) {
            begin_dockable_drag_drop_target(g, host_window.unwrap());
        }
    }

    // We update this after dock_node_updateTabBar()
    node.last_frame_active = g.frame_count;

    // Recurse into children
    // FIXME-DOCK FIXME-OPT: Should not need to recurse into children
    if host_window.is_some() {
        if node.child_nodes[0] != INVALID_ID {
            let cn = g.dock_node_mut(node.child_nodes[0]).unwrap();
            dock_node_update(g, cn);
        }
        if node.child_nodes[1] != INVALID_ID {
            let cn = g.dock_node_mut(node.child_nodes[1]).unwrap();
            dock_node_update(g, cn);
        }

        // Render outer borders last (after the tab bar)
        if node.is_root_node() {
            host_window.unwrap().draw_list.channels_set_current(1);
            render_window_outer_borders(g, host_window.unwrap());
        }

        // Further rendering (= hosted windows background) will be drawn on layer 0
        host_window.draw_list.channels_set_current(0);
    }

    // End host window
    if beginned_into_host_window {
        //-V1020
        end(g);
    }
}

// static bool dock_node_is_drop_allowedOne(Window* payload, Window* host_window)
pub fn dock_node_is_drop_allowed_one(
    g: &mut Context,
    payload: &mut Window,
    host_window: &mut Window,
) -> bool {
    if host_window.dock_node_as_host_id != INVALID_ID && g.dock_node_mut(host_window.dock_node_as_host_id).is_dock_space() && payload.begin_order_within_context < host_window.begin_order_within_context {
        return false;
    }

    let host_class = if host_window.dock_node_as_host_id != INVALID_ID {
        g.dock_node_mut(host_window.dock_node_as_host_id).unwrap().window_class.borrow()
    } else {
        host_window.window_class.borrow()
    };
    let payload_class = payload.window_class.borrow();
    if host_class.class_id != payload_class.class_id {
        if host_class.class_id != 0 && host_class.docking_allow_unclassed && payload_class.class_id == 0 {
            return true;
        }
        if payload_class.class_id != 0 && payload_class.docking_allow_unclassed && host_class.class_id == 0 {
            return true;
        }
        return false;
    }

    // Prevent docking any window created above a popup
    // Technically we should support it (e.g. in the case of a long-lived modal window that had fancy docking features),
    // by e.g. adding a 'if (!IsWindowWithinBeginStackOf(host_window, popup_window))' test.
    // But it would requires more work on our end because the dock host windows is technically created in NewFrame()
    // and our ->ParentXXX and ->RootXXX pointers inside windows are currently mislading or lacking.
    // ImGuiContext& g = *GImGui;
    // for (int i = g.open_popup_stack.size - 1; i >= 0; i--)
    for i in g.open_popup_stack.len() - 1..0 {
        // if (Window * popup_window = g.open_popup_stack[i].Window)
        let popup_window = g.window_mut(g.open_popup_stack[i].window_id);
        {
            if is_window_within_begin_stack_of(g, payload, popup_window) {
                // Payload is created from within a popup begin stack.
                return false;
            }
        }
    }

    return true;
}

// static bool dock_node_is_drop_allowed(Window* host_window, Window* root_payload)
pub fn dock_node_is_drop_allowed(
    g: &mut Context,
    host_window: &mut Window,
    root_payload: &mut Window,
) -> bool {
    let root_payload_dock_node = g.dock_node_mut(root_payload.dock_node_as_host_id);
    if root_payload_dock_node.is_some() && root_payload_dock_node.unwrap().is_split_node() {
        // FIXME-DOCK: Missing filtering
        return true;
    }

    let payload_count = if root_payload_dock_node.is_some() {
        root_payload_dock_node.unwrap().windows.len()
    } else {
        1
    };
    // for (int payload_n = 0; payload_n < payload_count; payload_n += 1)
    for payload_n in 0..payload_count {
        // Window* payload = root_payload.dock_node_as_host_id? root_payload.dock_node_as_host_id.windows[payload_n] : root_payload;
        let payload = if root_payload_dock_node.is_some() {
            g.window_mut(root_payload_dock_node.unwrap().windows[payload_n])
        } else {
            root_payload
        };

        if dock_node_is_drop_allowed_one(g, payload, host_window) {
            return true;
        }
    }
    return false;
}

// (Depth-First, Pre-Order)
// void dock_node_tree_update_splitter(ImGuiDockNode* node)
pub fn dock_node_tree_update_splitter(g: &mut Context, node: &mut DockNode) {
    if node.is_leaf_node() {
        return;
    }

    // ImGuiContext& g = *GImGui;

    // ImGuiDockNode* child_0 = node.child_nodes[0];
    // ImGuiDockNode* child_1 = node.child_nodes[1];
    let child_0 = g.dock_node_mut(node.child_nodes[0]);
    let child_1 = g.dock_node_mut(node.child_nodes[1]);
    if child_0.unwrap().is_visible && child_1.unwrap().is_visible {
        // Bounding box of the splitter cover the space between both nodes (w = spacing, h = size[xy^1] for when splitting horizontally)
        // const ImGuiAxis axis = node.split_axis;
        let axis = node.split_axis.clone();
        // IM_ASSERT(axis != ImGuiAxis_None);
        // Rect bb;
        let mut bb = Rect::default();
        bb.min = child_0.unwrap().pos.clone();
        bb.max = child_1.unwrap().pos.clone();
        bb.min[&axis] += child_0.size[&axis];
        bb.max[&axis ^ 1] += child_1.size[&axis ^ 1];
        //if (g.io.key_ctrl) GetForegroundDrawList(g.current_window->viewport)->add_rect(bb.min, bb.max, IM_COL32(255,0,255,255));

        let merged_flags = &child_0.unwrap().merged_flags | &child_1.unwrap().merged_flags; // Merged flags for BOTH childs
        let no_resize_axis_flag = if axis == Axis::X {
            DockNodeFlags::NoResizeX
        } else {
            DockNodeFlags::NoResizeY
        };
        if (&merged_flags.contains(&DockNodeFlags::NoResize)) || (&merged_flags.contains(&no_resize_axis_flag)) {
            // Window* window = g.current_window;
            let window = g.current_window_mut();
            window.draw_list.add_rect_filled(
                bb.min,
                bb.max,
                get_color_u32_no_alpha(StyleColor::Separator),
                g.style.frame_rounding,
            );
        } else {
            //bb.min[axis] += 1; // Display a little inward so highlight doesn't connect with nearby tabs on the neighbor node.
            //bb.max[axis] -= 1;
            // push_id(g, node.id);

            // Find resizing limits by gathering list of nodes that are touching the splitter line.
            // ImVector<ImGuiDockNode*> touching_nodes[2];
            let touching_nodes: [Vec<&mut DockNode>; 2];
            let min_size = g.style.window_min_size[axis];
            let mut resize_limits: [f32; 2] = [0f32; 2];
            resize_limits[0] = node.child_nodes[0].pos[&axis] + min_size;
            resize_limits[1] = node.child_nodes[1].pos[&axis] + node.child_nodes[1].size[&axis] - min_size;

            let splitter_id = get_id(g, "##splitter");
            if g.active_id == splitter_id {
                // Only process when splitter is active {
                dock_node_tree_update_splitterFindTouchingNode(
                    child_0,
                    &axis,
                    1,
                    &touching_nodes[0],
                );
                dock_node_tree_update_splitterFindTouchingNode(
                    child_1,
                    &axis,
                    0,
                    &touching_nodes[1],
                );

                // for (int touching_node_n = 0; touching_node_n < touching_nodes[0].size; touching_node_n += 1)
                for touching_node_n in 0..touching_nodes[0].len() {
                    resize_limits[0] = ImMax(
                        resize_limits[0],
                        touching_nodes[0][touching_node_n].rect().min[&axis] + min_size,
                    );
                }
                // for (int touching_node_n = 0; touching_node_n < touching_nodes[1].size; touching_node_n += 1)
                for touching_node_n in 0..touching_nodes[1].len() {
                    resize_limits[1] = ImMin(
                        resize_limits[1],
                        touching_nodes[1][touching_node_n].rect().max[axis] - min_size,
                    );
                }

                // [DEBUG] Render touching nodes & limits
                /*
                ImDrawList* draw_list = node->host_window ? GetForegroundDrawList(node->host_window) : GetForegroundDrawList(get_main_viewport());
                for (int n = 0; n < 2; n++)
                {
                    for (int touching_node_n = 0; touching_node_n < touching_nodes[n].size; touching_node_n++)
                        draw_list->add_rect(touching_nodes[n][touching_node_n]->pos, touching_nodes[n][touching_node_n]->pos + touching_nodes[n][touching_node_n]->size, IM_COL32(0, 255, 0, 255));
                    if (axis == ImGuiAxis_X)
                        draw_list->add_line(Vector2D(resize_limits[n], node->child_nodes[n]->pos.y), Vector2D(resize_limits[n], node->child_nodes[n]->pos.y + node->child_nodes[n]->size.y), IM_COL32(255, 0, 255, 255), 3.0);
                    else
                        draw_list->add_line(Vector2D(node->child_nodes[n]->pos.x, resize_limits[n]), Vector2D(node->child_nodes[n]->pos.x + node->child_nodes[n]->size.x, resize_limits[n]), IM_COL32(255, 0, 255, 255), 3.0);
                }
                */
            }

            // Use a short delay before highlighting the splitter (and changing the mouse cursor) in order for regular mouse movement to not highlight many splitters
            let cur_size_0 = child_0.unwrap().size[axis].clone();
            let cur_size_1 = child_1.size[axis].clone();
            let min_size_0 = resize_limits[0] - child_0.pos[axis].clone();
            let min_size_1 = child_1.pos[axis].clone() + child_1.size[axis].clone() - resize_limits[1];
            let bg_col = get_color_u32_no_alpha(StyleColor::WindowBg);
            if splitter_behavior(
                bb,
                GetID("##splitter"),
                axis,
                &cur_size_0,
                &cur_size_1,
                min_size_0,
                min_size_1,
                WINDOWS_HOVER_PADDING,
                WINDOWS_RESIZE_FROM_EDGES_FEEDBACK_TIMER,
                bg_col,
            ) {
                if touching_nodes[0].size > 0 && touching_nodes[1].size > 0 {
                    child_0.unwrap().size[axis] = child_0.unwrap().size_ref[axis] = cur_size_0;
                    child_1.unwrap().pos[axis] -= cur_size_1 - child_1.unwrap().size[axis];
                    child_1.unwrap().size[axis] = child_1.unwrap().size_ref[axis] = cur_size_1;

                    // Lock the size of every node that is a sibling of the node we are touching
                    // This might be less desirable if we can merge sibling of a same axis into the same parental level.
                    // for (int side_n = 0; side_n < 2; side_n += 1)
                    for side_n in 0..2 {
                        // for (int touching_node_n = 0; touching_node_n < touching_nodes[side_n].size; touching_node_n += 1)
                        for touching_node_n in 0..touching_nodes[side_n].len() {
                            // ImGuiDockNode* touching_node = touching_nodes[side_n][touching_node_n];
                            let mut touching_node = touching_nodes[side_n][touching_node_n];
                            //ImDrawList* draw_list = node->host_window ? GetForegroundDrawList(node->host_window) : GetForegroundDrawList(get_main_viewport());
                            //draw_list->add_rect(touching_node->pos, touching_node->pos + touching_node->size, IM_COL32(255, 128, 0, 255));
                            while touching_node.parent_node != node {
                                if touching_node.parent_node.split_axis == axis {
                                    // Mark other node so its size will be preserved during the upcoming call to dock_node_tree_update_pos_size().
                                    // ImGuiDockNode* node_to_preserve = touching_node.parent_node.child_nodes[side_n];
                                    let parent_node = g.dock_node_mut(touching_node.parent_node_id).unwrap();
                                    let node_to_preserve = g.dock_node_mut(parent_node.child_nodes[side_n]).unwrap();
                                    node_to_preserve.want_lock_size_once = true;
                                    //draw_list->add_rect(touching_node->pos, touching_node->rect().max, IM_COL32(255, 0, 0, 255));
                                    //draw_list->add_rect_filled(node_to_preserve->pos, node_to_preserve->rect().max, IM_COL32(0, 255, 0, 100));
                                }
                                touching_node = touching_node.parent_node;
                            }
                        }
                    }
                    dock_node_tree_update_pos_size(g, child_0.unwrap(), &child_0.unwrap().pos, &child_0.unwrap().size, None);
                    dock_node_tree_update_pos_size(g, child_1.unwrap(), &child_1.unwrap().pos, &child_1.unwrap().size, None);
                    mark_ini_settings_dirty(g);
                }
            }
            pop_id(g);
        }
    }

    if child_0.is_visible {
        dock_node_tree_update_splitter(g, child_0.unwrap());
    }
    if child_1.is_visible {
        dock_node_tree_update_splitter(g, child_1.unwrap());
    }
}
