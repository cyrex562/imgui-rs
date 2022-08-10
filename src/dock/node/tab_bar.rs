use crate::button::ButtonFlags;
use crate::color::StyleColor;
use crate::dock::defines::{WindowDockStyleColor, DOCKING_SPLITTER_SIZE};
use crate::dock::node;
use crate::dock::node::{title_bar, window, DockNode, DockNodeFlags};
use crate::input::mouse::{is_mouse_clicked, start_mouse_moving_window_or_node};
use crate::input::{MouseButton, NavLayer};
use crate::item::{is_item_active, pop_item_flag, push_item_flag, ItemFlags};
use crate::nav::nav_init_window;
use crate::popup::{is_popup_open, open_popup};
use crate::rect::Rect;
use crate::style::{
    color_convert_u32_to_float4, get_color_u32_no_alpha, pop_style_color, push_style_color,
    WINDOW_DOCK_STYLE_COLORS,
};
use crate::tab_bar::{TabBar, TabBarFlags, TabItem, TabItemFlags};
use crate::types::{Direction, Id32};
use crate::utils::add_hash_set;
use crate::vectors::vector_2d::Vector2D;
use crate::vectors::Vector4D;
use crate::window::current::{pop_id, push_override_id};
use crate::window::layer::focus_window;
use crate::window::lifecycle::{begin, end};
use crate::window::{Window, WindowFlags};
use crate::{hash_string, Context, INVALID_ID};
use std::collections::HashSet;

// User helper to append/amend into a dock node tab bar. Most commonly used to add e.g. a "+" button.
// bool DockNodeBeginAmendTabBar(ImGuiDockNode* node)
pub fn dock_node_begin_amend_tab_bar(g: &mut Context, node: &mut DockNode) -> bool {
    if node.tab_bar.is_none() || node.host_window_id == INVALID_ID {
        return false;
    }
    if node.merged_flags.contains(&DockNodeFlags::KeepAliveOnly) {
        return false;
    }
    begin(g, node.host_window_id.name, None, None);
    push_override_id(g, node.id);
    let ret = begin_tab_bar_ex(
        &node.tab_bar,
        node.tab_bar.BarRect,
        node.tab_bar.flags,
        node,
    );
    // IM_UNUSED(ret);
    // IM_ASSERT(ret);
    return true;
}

// void DockNodeEndAmendTabBar()
pub fn dock_node_end_amend_tab_bar(g: &mut Context) {
    end_tab_bar(g);
    pop_id(g);
    end(g);
}

// Submit the tab bar corresponding to a dock node and various housekeeping details.
// static void dock_node_updateTabBar(ImGuiDockNode* node, Window* host_window)
pub fn dock_node_update_tab_bar(g: &mut Context, node: &mut DockNode, host_window: &mut Window) {
    // ImGuiContext& g = *GImGui;
    // ImGuiStyle& style = g.style;
    let style = &mut g.style;

    let node_was_active = (node.last_frame_active + 1 == g.frame_count);
    let closed_all = node.want_close_all && node_was_active;
    let closed_one = node.want_close_tab_id != INVALID_ID && node_was_active;
    node.want_close_all = false;
    node.want_close_tab_id = 0;

    // Decide if we should use a focused title bar color
    let mut is_focused = false;
    ImGuiDockNode * root_node = node::dock_node_get_root_node(g, node);
    if title_bar::is_dock_node_title_bar_highlighted(g, node, root_node, host_window) {
        is_focused = true;
    }

    // hidden tab bar will show a triangle on the upper-left (in Begin)
    if node.is_hidden_tab_bar() || node.is_no_tab_bar() {
        node.visible_window_id = if node.windows.len() > 0 {
            node.windows[0]
        } else {
            INVALID_ID
        };
        node.is_focused = is_focused;
        if is_focused {
            node.last_frame_focused = g.frame_count;
        }
        if node.visible_window_id {
            // Notify root of visible window (used to display title in OS task bar)
            if is_focused || root_node.visible_window == INVALID_ID {
                root_node.visible_window = node.visible_window_id;
            }
            if node.tab_bar {
                node.tab_bar.visible_tab_id = node.visible_window_id.tab_id;
            }
        }
        return;
    }

    // Move ourselves to the Menu layer (so we can be accessed by tapping Alt) + undo skip_items flag in order to draw over the title bar even if the window is collapsed
    let backup_skip_item = host_window.skip_items;
    if !node.is_dock_space() {
        host_window.skip_items = false;
        host_window.dcnav_layer_current = NavLayer::Menu;
    }

    // Use PushOverrideID() instead of push_id() to use the node id _without_ the host window id.
    // This is to facilitate computing those id from the outside, and will affect more or less only the id of the collapse button, popup and tabs,
    // as docked windows themselves will override the stack with their own root id.
    push_override_id(g, node.id);
    let mut tab_bar = &mut node.tab_bar;
    let tab_bar_is_recreated = (tab_bar.is_none()); // Tab bar are automatically destroyed when a node gets hidden
    if tab_bar.is_none() {
        dock_node_add_tab_bar(g, node);
        tab_bar = &mut node.tab_bar;
    }

    let mut focus_tab_id: Id32 = INVALID_ID;
    node.is_focused = is_focused;

    let node_flags = node.merged_flags.clone();
    let has_window_menu_button = node_flags.contains(&DockNodeFlags::NoWindowMenuButton) == false
        && (style.window_menu_button_position != Direction::None);

    // In a dock node, the Collapse Button turns into the window Menu button.
    // FIXME-DOCK FIXME-OPT: Could we recycle popups id across multiple dock nodes?
    if has_window_menu_button && is_popup_open(g, hash_string("#WindowMenu", 0), None) {
        let tab_id = window::dock_node_update_window_menu(g, node, &mut tab_bar.unwrap());
        if tab_id != INVALID_ID {
            tab_bar.unwrap().next_selected_tab_id = tab_id;
            focus_tab_id = tab_id;
        }
        is_focused |= node.is_focused;
    }

    // Layout
    // Rect title_bar_rect, tab_bar_rect;
    let mut title_bar_rect = Rect::default();
    let mut tab_bar_rect = Rect::default();
    // Vector2D window_menu_button_pos;
    let mut window_menu_button_pos = Vector2D::default();
    // Vector2D close_button_pos = V;
    let mut close_button_pos = Vector2D::default();

    dock_node_calc_tab_bar_layout(
        g,
        Some(node),
        Some(&mut title_bar_rect),
        &mut tab_bar_rect,
        Some(&mut window_menu_button_pos),
        Some(&mut close_button_pos),
    );

    // Submit new tabs, they will be added as Unsorted and sorted below based on relative dock_order value.
    let tabs_count_old = tab_bar.unwrap().tabs.size;
    // for (int window_n = 0; window_n < node.windows.len(); window_n += 1)
    for window_n in 0..node.windows.len() {
        let window = g.window_mut(node.windows[window_n]);
        if tab_bar_find_tab_by_id(tab_bar, window.tab_id).is_none() {
            tab_bar_add_tab(tab_bar, TabItemFlags::Unsorted, window);
        }
    }

    // Title bar
    if is_focused {
        node.last_frame_focused = g.frame_count;
    }
    // ImU32 title_bar_col = get_color_u32(host_window.collapsed ? StyleColor::TitleBgCollapsed : is_focused ? StyleColor::TitleBgActive : StyleColor::TitleBg);
    let title_bar_col = get_color_u32_no_alpha(if host_window.collapsed {
        StyleColor::TitleBgCollapsed
    } else {
        if is_focused {
            StyleColor::TitleBgActive
        } else {
            StyleColor::TitleBg
        }
    });
    let rounding_flags = calc_rounding_flags_for_rect_in_rect(
        title_bar_rect,
        host_window.rect(),
        DOCKING_SPLITTER_SIZE,
    );
    g.draw_list_mut(host_window.draw_list_id).add_rect_filled(
        &title_bar_rect.min,
        &title_bar_rect.max,
        title_bar_col,
        host_window.WindowRounding,
        rounding_flags,
    );

    // Docking/Collapse button
    if has_window_menu_button {
        if collapse_button(
            host_window.get_id(g, "#COLLAPSE"),
            window_menu_button_pos,
            node,
        ) {
            // == DockNodeGetWindowMenuButtonId(node)
            open_popup(g, "#WindowMenu", None);
        }
        if is_item_active(g) {
            focus_tab_id = tab_bar.selected_tab_id;
        }
    }

    // If multiple tabs are appearing on the same frame, sort them based on their persistent dock_order value
    let tabs_unsorted_start = tab_bar.unwrap().tabs.len();
    // for (int tab_n = tab_bar.tabs.size - 1; tab_n >= 0 && (tab_bar.tabs[tab_n].flags & TabItemFlags::Unsorted); tab_n--)
    // for tab in tab_bar.unwrap().tabs.iter_mut().filter(|x| x.flags.contains(TabItemFlags::Unsorted))
    // {
    //     // FIXME-DOCK: Consider only clearing the flag after the tab has been alive for a few consecutive frames, allowing late comers to not break sorting?
    //     // tab_bar.tabs[tab_n].flags &= ~TabItemFlags::Unsorted;
    //     // tabs_unsorted_start = tab_n;
    //     tabs_unsorted_start =
    // }
    // if (tab_bar.tabs.size > tabs_unsorted_start)
    // {
    //     // IMGUI_DEBUG_LOG_DOCKING("[docking] In node 0x%08X: %d new appearing tabs:%s\n", node.id, tab_bar.Tabs.size - tabs_unsorted_start, (tab_bar.Tabs.size > tabs_unsorted_start + 1) ? " (will sort)" : "");
    //     for (int tab_n = tabs_unsorted_start; tab_n < tab_bar.tabs.size; tab_n += 1)
    //         // IMGUI_DEBUG_LOG_DOCKING("[docking] - Tab '%s' Order %d\n", tab_bar.Tabs[tab_n].Window.Name, tab_bar.Tabs[tab_n].Window.dock_order);
    //     if (tab_bar.tabs.size > tabs_unsorted_start + 1)
    //         ImQsort(tab_bar.tabs.data + tabs_unsorted_start, tab_bar.tabs.size - tabs_unsorted_start, sizeof(ImGuiTabItem), TabItemComparerByDockOrder);
    // }
    tab_bar.unwrap().tabs.sort_by(|a, b| {
        let a_win = g.window_mut(a.window_id);
        let b_win = g.window_mut(b.window_id);
        a_win.dock_order.cmp(&b_win.dock_order)
    });

    // Apply nav_window focus back to the tab bar
    if g.nav_window && g.nav_window.root_window.dock_node == node {
        tab_bar.selected_tab_id = g.nav_window.root_window.id;
    }

    // Selected newly added tabs, or persistent tab id if the tab bar was just recreated
    if tab_bar_is_recreated && tab_bar_find_tab_by_id(tab_bar, node.selected_tab_id) != None {
        tab_bar.selected_tab_id = tab_bar.next_selected_tab_id = node.selected_tab_id;
    } else if tab_bar.tabs.size > tabs_count_old {
        tab_bar.selected_tab_id = tab_bar.next_selected_tab_id = tab_bar.tabs.back().Window.tab_id;
    }

    // Begin tab bar
    let mut tab_bar_flags: HashSet<TabBarFlags> = HashSet::from([
        TabBarFlags::Reorderable,
        TabBarFlags::AutoSelectNewTabs,
        TabBarFlags::SaveSettings,
        TabBarFlags::DockNode,
    ]); // | ImGuiTabBarFlags_NoTabListScrollingButtons);
        // tab_bar_flags |= TabBarFlags::SaveSettings | TabBarFlags::DockNode;
    if !host_window.collapsed && is_focused {
        tab_bar_flags.insert(TabBarFlags::IsFocused); // |= TabBarFlags::IsFocused;
    }
    begin_tab_bar_ex(tab_bar, tab_bar_rect, tab_bar_flags, node);
    //host_window->draw_list->add_rect(tab_bar_rect.min, tab_bar_rect.max, IM_COL32(255,0,255,255));

    // Backup style colors
    let mut backup_style_colors: [Vector4D; WindowDockStyleColor.len()] =
        [Vector4D::default(); WindowsDockStyleColor.len()];
    // for (int color_n = 0; color_n < WindowDockStyleColor.; color_n += 1){
    //     backup_style_colors[color_n] = g.style.colors[WINDOW_DOCK_STYLE_COLORS[color_n]];
    // }
    for i in 0..backup_style_colors.len() {
        backup_style_colors[i] = g.style.colors[&WINDOW_DOCK_STYLE_COLORS[i]];
    }

    // Submit actual tabs
    node.visible_window_id = INVALID_ID;
    // for (int window_n = 0; window_n < node.windows.len(); window_n += 1)s
    for win_id in node.windows {
        // Window* window = node.windows[window_n];
        let window = g.window_mut(win_id);
        if (closed_all || closed_one) == (window.tab_id != INVALID_ID && window.has_close_button)
            && !(window.flags.contains(&WindowFlags::UnsavedDocument)) {
            continue;
        }
        if window.last_frame_active + 1 >= g.frame_count || !node_was_active {
            // ImGuiTabItemFlags tab_item_flags = 0;
            let mut tab_item_flags: HashSet<TabItemFlags> = HashSet::new();
            // tab_item_flags |= window.window_class.TabItemFlagsOverrideSet;
            tab_item_flags = add_hash_set(
                &tab_item_flags,
                &window.window_class.tab_item_flags_override_set,
            );
            if window.flags.contains(&WindowFlags::UnsavedDocument) {
                tab_item_flags.insert(TabItemFlags::UnsavedDocument);
            }
            if tab_bar
                .flags
                .contains(&TabBarFlags::NoCloseWithMiddleMouseButton)
            {
                tab_item_flags.insert(TabItemFlags::NoCloseWithMiddleMouseButton);
            }

            // Apply stored style overrides for the window
            // for (int color_n = 0; color_n < WindowDockStyleColor::LastItem; color_n += 1){
            for color_n in 0..WindowDockStyleColor::LastItem {
                g.style.colors[WINDOW_DOCK_STYLE_COLORS[color_n].clone()] =
                    color_convert_u32_to_float4(window.dock_style.colors[color_n]);
            }

            // Note that TabItemEx() calls TabBarCalcTabID() so our tab item id will ignore the current id stack (rightly so)
            let mut tab_open = true;
            tab_item_ex(
                tab_bar,
                &window.name,
                if window.has_close_button {
                    Some(&tab_open)
                } else {
                    None
                },
                tab_item_flags,
                window,
            );
            if !tab_open {
                node.want_close_tab_id = window.tab_id;
            }
            if tab_bar.visible_tab_id == window.tab_id {
                node.visible_window_id = window.id;
            }

            // Store last item data so it can be queried with IsItemXXX functions after the user Begin() call
            window.dock_tab_item_status_flags = g.last_item_data.status_flags.clone();
            window.dock_tab_item_rect = g.last_item_data.rect;

            // update navigation id on menu layer
            if g.nav_window
                && g.nav_window.root_window == window
                && (window.dc.nav_layers_active_mask & (1 << NavLayer::Menu)) == 0
            {
                host_window.nav_last_ids[1] = window.tab_id;
            }
        }
    }

    // Restore style colors
    // for (int color_n = 0; color_n < WindowDockStyleColor::COUNT; color_n += 1)
    for color_n in 0..WindowDockStyleColor::LastItem {
        g.style.colors[WINDOW_DOCK_STYLE_COLORS[color_n].clone()] =
            backup_style_colors[color_n].clone();
    }

    // Notify root of visible window (used to display title in OS task bar)
    if node.visible_window_id {
        if is_focused || root_node.visible_window_id == INVALID_ID {
            root_node.visible_window = node.visible_window_id;
        }
    }

    // Close button (after visible_window was updated)
    // Note that visible_window may have been overrided by CTRL+Tabbing, so visible_window->tab_id may be != from tab_bar->selected_tab_id
    let close_button_is_enabled = node.has_close_button
        && node.visible_window_id != INVALID_ID
        && node.visible_window_id.has_close_button;
    let close_button_is_visible = node.has_close_button;
    //const bool close_button_is_visible = close_button_is_enabled; // Most people would expect this behavior of not even showing the button (leaving a hole since we can't claim that space as other windows in the tba bar have one)
    if close_button_is_visible {
        if !close_button_is_enabled {
            push_item_flag(g, &ItemFlags::Disabled, true);
            push_style_color(
                g,
                StyleColor::Text,
                style.colors[StyleColor::Text] * Vector4D(1.0, 1.0, 1.0, 0.4),
            );
        }
        if close_button(host_window.get_id(g, "#CLOSE"), close_button_pos) {
            node.want_close_all = true;
            // for (int n = 0; n < tab_bar.tabs.size; n += 1){
            for tab in tab_bar.unwrap().tabs.iter_mut() {
                tab_bar_close_tab(tab_bar, tab);
            }
        }
        //if (IsItemActive())
        //    focus_tab_id = tab_bar->selected_tab_id;
        if !close_button_is_enabled {
            pop_style_color(0);
            pop_item_flag(g);
        }
    }

    // When clicking on the title bar outside of tabs, we still focus the selected tab for that node
    // FIXME: TabItem use AllowItemOverlap so we manually perform a more specific test for now (hovered || held)
    let title_bar_id = host_window.get_id(g, "#TITLEBAR");
    if g.hovered_id == 0 || g.hovered_id == title_bar_id || g.active_id == title_bar_id {
        let mut held = false;
        button_behavior(
            &title_bar_rect,
            title_bar_id,
            None,
            &held,
            ButtonFlags::AllowItemOverlap,
        );
        if g.hovered_id == title_bar_id {
            // ImGuiButtonFlags_AllowItemOverlap + SetItemAllowOverlap() required for appending into dock node tab bar,
            // otherwise dragging window will steal hovered_id and amended tabs cannot get them.
            g.last_item_data.id = title_bar_id;
            SetItemAllowOverlap();
        }
        if held {
            if is_mouse_clicked(g, MouseButton::Left, false) {
                focus_tab_id = tab_bar.selected_tab_id;
            }

            // Forward moving request to selected window
            let tab = tab_bar_find_tab_by_id(tab_bar, tab_bar.unwrap().selected_tab_id);
            // if ImGuiTabItem* tab = tab_bar_find_tab_by_id(tab_bar, tab_bar.selected_tab_id) {
            if tab.is_some() {
                start_mouse_moving_window_or_node(
                    g,
                    if tab.unwrap().wwindow_id != INVALID_ID {
                        g.window_mut(tab.window_id)
                    } else {
                        g.window_mut(node.host_window_id)
                    },
                    node,
                    false,
                );
            }
        }
    }

    // Forward focus from host node to selected window
    //if (is_focused && g.nav_window == host_window && !g.nav_windowing_target)
    //    focus_tab_id = tab_bar->selected_tab_id;

    // When clicked on a tab we requested focus to the docked child
    // This overrides the value set by "forward focus from host node to selected window".
    if tab_bar.unwrap().next_selected_tab_id {
        focus_tab_id = tab_bar.unwrap().next_selected_tab_id;
    }

    // Apply navigation focus
    if focus_tab_id != 0 {
        let tab: Option<&mut TabItem> = tab_bar_find_tab_by_id(tab_bar.unwrap(), focus_tab_id);
        if tab.is_some() {
            if tab.unwrap().window_id != INVALID_ID {
                focus_window(g, g.window_mut(tab.unwrap().window_id));
                nav_init_window(g, g.window_mut(tab.unwrap().window_id), false);
            }
        }
    }

    end_tab_bar();
    pop_id(g);

    // Restore skip_items flag
    if !node.is_dock_space() {
        host_window.dc.nav_layer_current = NavLayer::Main;
        host_window.skip_items = backup_skip_item;
    }
}

// static void DockNodeAddTabBar(ImGuiDockNode* node)
pub fn dock_node_add_tab_bar(g: &mut Context, node: &mut DockNode) {
    // IM_ASSERT(node.TabBar == None);
    node.tab_bar = Some(TabBar::default());
}

// static void dock_node_remove_tab_bar(ImGuiDockNode* node)
pub fn dock_node_remove_tab_bar(g: &mut Context, node: &mut DockNode) {
    if node.tab_bar.is_none() {
        return;
    }
    // IM_DELETE(node.tab_bar);
    node.tab_bar = None;
}

// window menu button == collapse button when not in a dock node.
// FIXME: This is similar to RenderWindowTitleBarContents(), may want to share code.
// static void DockNodeCalcTabBarLayout(const ImGuiDockNode* node, Rect* out_title_rect, Rect* out_tab_bar_rect, Vector2D* out_window_menu_button_pos, Vector2D* out_close_button_pos)
pub fn dock_node_calc_tab_bar_layout(
    g: &mut Context,
    node: Option<&mut DockNode>,
    mut out_title_rect: Option<&mut Rect>,
    out_tab_bar_rect: &mut Rect,
    out_window_menu_button_pos: Option<&mut Vector2D>,
    out_close_button_pos: Option<&mut Vector2D>,
) {
    // ImGuiContext& g = *GImGui;
    // ImGuiStyle& style = g.style;
    let style = &mut g.style;

    let mut r = Rect::from((
        node.pos.x,
        node.pos.y,
        node.pos.x + node.size.x,
        node.pos.y + g.font_size + g.style.frame_padding.y * 2.0,
    ));
    if out_title_rect.is_some() {
        out_title_rect.unwrap().min = r.min.clone();
        out_title_rect.unwrap().max = r.max.clone();
    }

    r.min.x += style.window_border_size;
    r.max.x -= style.window_border_size;

    let button_sz = g.font_size;

    let mut window_menu_button_pos = r.min;
    r.min.x += style.frame_padding.x;
    r.max.x -= style.frame_padding.x;
    if node.has_close_button {
        r.max.x -= button_sz;
        // if (out_close_button_pos) *out_close_button_pos = Vector2D::new(r.max.x.clone() - style.frame_padding.x, r.min.y.clone());
        if out_close_button_pos.is_some() {
            out_close_button_pos.unwrap().x = r.max.x.clone() - style.frame_padding.x;
            out_close_button_pos.unwrap().y = r.min.y.clone();
        }
    }
    if node.hash_window_menu_button && style.window_menu_button_position == Direction::Left {
        r.min.x += button_sz + style.item_inner_spacing.x;
    } else if node.hash_window_menu_button && style.window_menu_button_position == Direction::Right
    {
        r.max.x -= button_sz + style.frame_padding.x;
        window_menu_button_pos = Vector2D::new(r.max.x.clone(), r.min.y.clone());
    }
    if out_tab_bar_rect {
        out_tab_bar_rect.unwrap().min = r.min.clone();
        out_tab_bar_rect.unwrap().max = r.max.clone()
    }
    if out_window_menu_button_pos {
        *out_window_menu_button_pos = window_menu_button_pos;
    }
}
