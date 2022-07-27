use std::collections::HashSet;
use crate::{Context, INVALID_ID, window};
use crate::axis::Axis;
use crate::color::StyleColor;
use crate::dock::defines::{DOCKING_SPLITTER_SIZE, WindowDockStyleColor};
use crate::dock::node;
use crate::dock::node::{DockNode, DockNodeFlags, int, tab_bar};
use crate::dock::node::rect::{dock_node_calc_drop_rects_and_test_mouse_pos, dock_node_calc_split_rects};
use crate::dock::preview::DockPreviewData;
use crate::draw::draw_list::get_foreground_draw_list;
use crate::frame::get_frame_height;
use crate::math::saturate_f32;
use crate::style::{get_color_u32, pop_style_color, push_style_color};
use crate::tab_bar::TabItemFlags;
use crate::types::{Direction, DIRECTIONS};
use crate::vectors::two_d::Vector2D;
use crate::window::{Window, WindowFlags};

// host_node may be NULL if the window doesn't have a dock_node already.
// FIXME-DOCK: This is misnamed since it's also doing the filtering.
// static void dock_node_preview_dock_setup(ImGuiWindow* host_window, ImGuiDockNode* host_node, ImGuiWindow* root_payload, ImGuiDockPreviewData* data, bool is_explicit_target, bool is_outer_docking)
pub fn dock_node_preview_dock_setup(g: &mut Context, host_window: &mut Window, host_node: Option<&mut DockNode>, root_payload: &mut Window, data: &mut DockPreviewData, is_explicit_target: bool, is_outer_docking: bool)
{
    // ImGuiContext& g = *GImGui;

    // There is an edge case when docking into a dockspace which only has inactive nodes.
    // In this case DockNodeTreeFindNodeByPos() will have selected a leaf node which is inactive.
    // Because the inactive leaf node doesn't have proper pos/size yet, we'll use the root node as reference.
    let root_payload_as_host = g.get_dock_node(root_payload.dock_node_as_host_id);
    let ref_node_for_rect = if host_node.is_some() && !host_node.unwrap().is_visible { node::dock_node_get_root_node(g, host_node.unwrap()) } else { host_node.unwrap() };
    if ref_node_for_rect {}
        // IM_ASSERT(ref_node_for_rect.is_visible == true);

    // Filter, figure out where we are allowed to dock
    // ImGuiDockNodeFlags src_node_flags = root_payload_as_host ? root_payload_as_host.merged_flags : root_payload.window_class.dock_node_flags_override_set;
    let src_node_flags: HashSet<DockNodeFlags> = if root_payload_as_host.is_some() {
        root_payload_as_host.unwrap().merged_flags.clone()
    } else {
        root_payload.window_class.dock_node_flags_override_set.clone();
    };
    let dst_node_flags: HashSet<DockNodeFlags> = if host_node.is_some()
    { host_node.unwrap().merged_flags.clone() } else { host_window.unwrap().window_class.dock_node_flags_override_set.clone() };
    data.is_center_available = true;
    if is_outer_docking {
        data.is_center_available = false;
    }
    else if dst_node_flags.contains(&DockNodeFlags::NoDocking) {
        data.is_center_available = false;
    }
    else if host_node.is_some() && (dst_node_flags.contains(&DockNodeFlags::NoDockingInCentralNode)) && host_node.is_central_node() {
        data.is_center_available = false;
    }
    else if !host_node.is_some()  && root_payload_as_host.is_some() && root_payload_as_host.unwrap().is_split_node() && (root_payload_as_host.unwrap().only_node_with_window_id == INVALID_ID) { // Is _visibly_ split?
        data.is_center_available = false;
    }
    else if dst_node_flags.contains(&DockNodeFlags::NoDockingOverMe) {
        data.is_center_available = false;
    }
    else if (src_node_flags.contains(&DockNodeFlags::NoDockingOverOther)) && (!host_node.is_some()) {
        data.is_center_available = false;
    }
    else if (src_node_flags.contains(&DockNodeFlags::NoDockingOverEmpty)) && host_node.is_some() {
        data.is_center_available = false;
    }

    data.is_sides_available = true;
    if (dst_node_flags.contains(&DockNodeFlags::NoSplit)) || g.io.config_docking_no_split {
        data.is_sides_available = false;
    }
    else if !is_outer_docking && host_node.is_some() && host_node.unwrap().parent_node_id == INVALID_ID && host_node.unwrap().is_central_node() {
        data.is_sides_available = false;
    }
    else if (dst_node_flags & DockNodeFlags::NoDockingSplitMe) || (src_node_flags & DockNodeFlags::NoDockingSplitOther) {
        data.is_sides_available = false;
    }

    // build a tentative future node (reuse same structure because it is practical. Shape will be readjusted when previewing a split)
    let data_future_node: &mut DockNode = g.get_dock_node(data.future_node).unwrwap();
    data_future_node.has_close_button = (if host_node.is_some() { host_node.unwrap().has_close_button } else { host_window.unwrap().has_close_button }) || (root_payload.has_close_button);
     data_future_node.hash_window_menu_button = if host_node.is_some() { true } else { ((host_window.flags.contains(&WindowFlags::NoCollapse)) == false) };
     data_future_node.pos = if ref_node_for_rect.id != INVALID_ID { ref_node_for_rect.pos.clone() } else { host_window.pos.clone()};
     data_future_node.size = if ref_node_for_rect.id != INVALID_ID { ref_node_for_rect.size.clone() } else { host_window.size.clone() };

    // Calculate drop shapes geometry for allowed splitting directions
    // IM_ASSERT(Dir::None == -1);
    data.split_node = host_node.unwrap().id;
    data.split_dir = Direction::None;
    data.is_split_dir_explicit = false;
    if !host_window.collapsed {
        for dir in DIRECTIONS.iter()
        {
            if dir == Direction::None && !data.is_center_available {
                continue;
            }
            if dir != Direction::None && !data.is_sides_available {
                continue;
            }
            if dock_node_calc_drop_rects_and_test_mouse_pos(g, &data_future_node.rect(), dir, data.drop_rects_draw[dir + 1], is_outer_docking, Some(&mut g.io.mouse_pos)) {
                data.split_dir = dir.clone();
                data.is_split_dir_explicit = true;
            }
        }
    }

    // When docking without holding Shift, we only allow and preview docking when hovering over a drop rect or over the title bar
    data.is_drop_allowed = (data.split_dir != Direction::None) || (data.is_center_available);
    if !is_explicit_target && !data.is_split_dir_explicit && !g.io.config_docking_with_shift {
        data.is_drop_allowed = false;
    }

    // Calculate split area
    data.split_ratio = 0.0;
    if data.split_dir != Direction::None
    {
        let split_dir = data.split_dir.clone();
        let split_axis = if split_dir == Direction::Left || split_dir == Direction::Right { Axis::X } else { Axis::Y };
        // Vector2D pos_new, pos_old = data.future_node.pos;
        let pos_new = data_future_node.pos.clone();
        let pos_old = data_future_node.pos.clone();
        // Vector2D size_new, size_old = data.future_node.size;
        let size_new = data_future_node.size.clone();
        let size_old = data_future_node.size.clone();

        dock_node_calc_split_rects(g, &pos_old, &size_old, &pos_new, &size_new, split_dir, root_payload.size.clone());

        // Calculate split ratio so we can pass it down the docking request
        let split_ratio = saturate_f32(size_new[&split_axis] / data_future_node.size[&split_axis]);
        data_future_node.pos = pos_new;
        data_future_node.size = size_new;
        data_split_ratio = if split_dir == Direction::Right || split_dir == Direction::Down
        { (1.0 - split_ratio) } else { (split_ratio) };
    }
}

// static void DockNodePreviewDockRender(ImGuiWindow* host_window, ImGuiDockNode* host_node, ImGuiWindow* root_payload, const ImGuiDockPreviewData* data)
pub fn dock_node_preview_dock_render(g: &mut Context, host_window: &mut host_window, host_node: &mut DockNode, root_payload: &mut window::Window, data: &mut DockPreviewData)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.current_window == host_window);   // Because we rely on font size to calculate tab sizes

    // With this option, we only display the preview on the target viewport, and the payload viewport is made transparent.
    // To compensate for the single layer obstructed by the payload, we'll increase the alpha of the preview nodes.
    const bool is_transparent_payload = g.io.config_docking_transparent_payload;

    // In case the two windows involved are on different viewports, we will draw the overlay on each of them.
    int overlay_draw_lists_count = 0;
    ImDrawList* overlay_draw_lists[2];
    overlay_draw_lists[overlay_draw_lists_count += 1] = get_foreground_draw_list(host_window.viewport);
    if (host_window.viewport != root_payload.Viewport && !is_transparent_payload)
        overlay_draw_lists[overlay_draw_lists_count += 1] = get_foreground_draw_list(root_payload.Viewport);

    // Draw main preview rectangle
    const ImU32 overlay_col_main = get_color_u32(StyleColor::DockingPreview, is_transparent_payload ? 0.60 : 0.40);
    const ImU32 overlay_col_drop = get_color_u32(StyleColor::DockingPreview, is_transparent_payload ? 0.90 : 0.70);
    const ImU32 overlay_col_drop_hovered = get_color_u32(StyleColor::DockingPreview, is_transparent_payload ? 1.20 : 1.00);
    const ImU32 overlay_col_lines = get_color_u32(StyleColor::NavWindowingHighlight, is_transparent_payload ? 0.80 : 0.60);

    // Display area preview
    const bool can_preview_tabs = (root_payload.dock_node_as_host_id == NULL || root_payload.dock_node_as_host_id.windows.len() > 0);
    if (data.is_drop_allowed)
    {
        Rect overlay_rect = data.future_node.Rect();
        if (data.split_dir == Direction::None && can_preview_tabs)
            overlay_rect.min.y += get_frame_height();
        if (data.split_dir != Direction::None || data.is_center_available)
            for (int overlay_n = 0; overlay_n < overlay_draw_lists_count; overlay_n += 1)
                overlay_draw_lists[overlay_n].add_rect_filled(overlay_rect.min, overlay_rect.max, overlay_col_main, host_window.WindowRounding, calc_rounding_flags_for_rect_in_rect(overlay_rect, host_window.Rect(), DOCKING_SPLITTER_SIZE));
    }

    // Display tab shape/label preview unless we are splitting node (it generally makes the situation harder to read)
    if (data.is_drop_allowed && can_preview_tabs && data.split_dir == Direction::None && data.is_center_available)
    {
        // Compute target tab bar geometry so we can locate our preview tabs
        Rect tab_bar_rect;
        tab_bar::dock_node_calc_tab_bar_layout(&data.future_node, NULL, &tab_bar_rect, NULL, NULL);
        Vector2D tab_pos = tab_bar_rect.min;
        if (host_node && host_node.tab_bar)
        {
            if (!host_node.is_hidden_tab_bar() && !host_node.is_no_tab_bar())
                tab_pos.x += host_node.tab_bar.WidthAllTabs + g.style.item_inner_spacing.x; // We don't use OffsetNewTab because when using non-persistent-order tab bar it is incremented with each Tab submission.
            else
                tab_pos.x += g.style.item_inner_spacing.x + TabItemCalcSize(host_node.windows[0].name, host_node.windows[0].has_close_button).x;
        }
        else if (!(host_window.flags & WindowFlags::DockNodeHost))
        {
            tab_pos.x += g.style.item_inner_spacing.x + TabItemCalcSize(host_window.name, host_window.has_close_button).x; // Account for slight offset which will be added when changing from title bar to tab bar
        }

        // Draw tab shape/label preview (payload may be a loose window or a host window carrying multiple tabbed windows)
        if (root_payload.dock_node_as_host_id)
            // IM_ASSERT(root_payload.DockNodeAsHost.Windows.size <= root_payload.DockNodeAsHost.TabBar.Tabs.size);
        ImGuiTabBar* tab_bar_with_payload = root_payload.dock_node_as_host_id? root_payload.dock_node_as_host_id.tab_bar : NULL;
        const int payload_count = tab_bar_with_payload ? tab_bar_with_payload.tabs.size : 1;
        for (int payload_n = 0; payload_n < payload_count; payload_n += 1)
        {
            // dock_node's tab_bar may have non-window Tabs manually appended by user
            ImGuiWindow* payload_window = tab_bar_with_payload ? tab_bar_with_payload.tabs[payload_n].Window : root_payload;
            if (tab_bar_with_payload && payload_window == NULL)
                continue;
            if (!dock_node_is_drop_allowedOne(payload_window, host_window))
                continue;

            // Calculate the tab bounding box for each payload window
            Vector2D tab_size = TabItemCalcSize(payload_window.name, payload_window.has_close_button);
            Rect tab_bb(tab_pos.x, tab_pos.y, tab_pos.x + tab_size.x, tab_pos.y + tab_size.y);
            tab_pos.x += tab_size.x + g.style.item_inner_spacing.x;
            const ImU32 overlay_col_text = get_color_u32(payload_window.dock_style.colors[WindowDockStyleColor::Text]);
            const ImU32 overlay_col_tabs = get_color_u32(payload_window.dock_style.colors[WindowDockStyleColor::TabActive]);
            push_style_color(, StyleColor::Text, overlay_col_text);
            for (int overlay_n = 0; overlay_n < overlay_draw_lists_count; overlay_n += 1)
            {
                ImGuiTabItemFlags tab_flags = TabItemFlags::Preview | ((payload_window.flags & WindowFlags::UnsavedDocument) ? TabItemFlags::UnsavedDocument : 0);
                if (!tab_bar_rect.Contains(tab_bb))
                    overlay_draw_lists[overlay_n].push_clip_rect(tab_bar_rect.min, tab_bar_rect.max);
                TabItemBackground(overlay_draw_lists[overlay_n], tab_bb, tab_flags, overlay_col_tabs);
                TabItemLabelAndCloseButton(overlay_draw_lists[overlay_n], tab_bb, tab_flags, g.style.frame_padding, payload_window.name, 0, 0, false, NULL, NULL);
                if (!tab_bar_rect.Contains(tab_bb))
                    overlay_draw_lists[overlay_n].pop_clip_rect();
            }
            pop_style_color();
        }
    }

    // Display drop boxes
    const float overlay_rounding = ImMax(3.0, g.style.FrameRounding);
    for (int dir = Direction::None; dir < Direction::COUNT; dir += 1)
    {
        if (!data.drop_rects_draw[dir + 1].is_inverted())
        {
            Rect draw_r = data.drop_rects_draw[dir + 1];
            Rect draw_r_in = draw_r;
            draw_r_in.Expand(-2.0);
            ImU32 overlay_col = (data.split_dir == (ImGuiDir)dir && data.is_split_dir_explicit) ? overlay_col_drop_hovered : overlay_col_drop;
            for (int overlay_n = 0; overlay_n < overlay_draw_lists_count; overlay_n += 1)
            {
                Vector2D center = f32::floor(draw_r_in.get_center());
                overlay_draw_lists[overlay_n].add_rect_filled(draw_r.min, draw_r.max, overlay_col, overlay_rounding);
                overlay_draw_lists[overlay_n].AddRect(draw_r_in.min, draw_r_in.max, overlay_col_lines, overlay_rounding);
                if (dir == Direction::Left || dir == Direction::Right)
                    overlay_draw_lists[overlay_n].add_line(Vector2D::new(center.x, draw_r_in.min.y), Vector2D::new(center.x, draw_r_in.max.y), overlay_col_lines);
                if (dir == Direction::Up || dir == Direction::Down)
                    overlay_draw_lists[overlay_n].add_line(Vector2D::new(draw_r_in.min.x, center.y), Vector2D::new(draw_r_in.max.x, center.y), overlay_col_lines);
            }
        }

        // Stop after ImGuiDir_None
        if ((host_node && (host_node.merged_flags & DockNodeFlags::NoSplit)) || g.io.config_docking_no_split)
            return;
    }
}
