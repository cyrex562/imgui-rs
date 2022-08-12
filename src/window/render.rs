use crate::axis::Axis;
use crate::border::get_resize_border_rect;
use crate::color::{StyleColor, COLOR32_A_MASK, IM_COL32_A_SHIFT, IM_COL32_WHITE};
use crate::config::ConfigFlags;
use crate::dock::dock_style_color::DOCKING_TRANSPARENT_PAYLOAD_ALPHA;
use crate::drag_drop::DragDropFlags;
use crate::draw::draw_defines::DrawFlags;
use crate::draw::list::{foreground_draw_list, DrawListFlags};
use crate::globals::GImGui;
use crate::id::set_active_id;
use crate::input::mouse::{start_mouse_moving_window, start_mouse_moving_window_or_node};
use crate::input::{InputSource, NavLayer};
use crate::item::{ItemFlags, ItemStatusFlags};
use crate::math::{f32_to_int8_sat, saturate_f32};
use crate::orig_imgui_single_file::{buf, buf_end, int};
use crate::rect::Rect;
use crate::style::{
    color_u32_from_style_color, get_color_u32_no_alpha, pop_style_color, push_style_color, Style,
};
use crate::text::calc_text_size;
use crate::types::DataAuthority;
use crate::types::Direction;
use crate::vectors::vector_2d::Vector2D;
use crate::vectors::vec_length_sqr;
use crate::window::checks::is_window_active_and_visible;
use crate::window::get::get_window_display_layer;
use crate::window::next_window::NextWindowDataFlags;
use crate::window::props::get_window_bg_color_idx;
use crate::window::settings::apply_window_settings;
use crate::window::size::calc_window_size_after_constraint;
use crate::window::state::set_window_condition_allow_flags;
use crate::window::{
    Window, WindowFlags, WINDOWS_HOVER_PADDING, WINDOWS_RESIZE_FROM_EDGES_FEEDBACK_TIMER,
};
use crate::{Context, ViewportFlags};
use std::collections::HashSet;

// static void ImGui::render_window_outer_borders(Window* window)
pub fn render_window_outer_borders(g: &mut Context, window: &mut Window) {
    // ImGuiContext& g = *GImGui;
    // float rounding = window.window_rounding;
    let mut rounding = window.window_rounding;
    // float border_size = window.window_border_size;
    let mut border_size = window.window_border_size;
    // if border_size > 0.0 && !(window.flags & WindowFlags::NoBackground)
    if border_size > 0.0 && window.flags.contains(&WindowFlags::NoBackground) == false {
        window.draw_list.add_rect(
            &window.pos,
            &window.pos + &window.size,
            color_u32_from_style_color(StyleColor::Border, 0.0),
            rounding,
            0,
            border_size,
        );
    }

    // int border_held = window.ResizeBorderHeld;
    let mut border_held = window.resize_border_held;
    if border_held != -1 {
        // const ImGuiResizeBorderDef& def = resize_border_def[border_held];
        let def = resize_border_def[border_held];
        // Rect border_r = GetResizeBorderRect(window, border_held, rounding, 0.0);
        let mut border_r = get_resize_border_rect(window, border_held as i32, rounding, 0.0);
        window.draw_listpath_arc_to(
            Vector2D::lerp2(&border_r.min, &border_r.max, def.segment_n1)
                + Vector2D::new(0.5, 0.5)
                + def.inner_dir * rounding,
            rounding,
            def.outer_angle - f32::PI * 0.25,
            def.outer_angle,
        );
        window.draw_listpath_arc_to(
            Vector2D::lerp2(&border_r.min, &border_r.max, def.segment_n2)
                + Vector2D::new(0.5, 0.5)
                + def.inner_dir * rounding,
            rounding,
            def.outer_angle,
            def.outer_angle + f32::PI * 0.25,
        );
        window.draw_list.path_stroke(
            get_color_u32_no_alpha(StyleColor::SeparatorActive),
            0,
            ImMax(2.0, border_size),
        ); // Thicker than usual
    }
    // if (g.style.frame_border_size > 0 && !(window.flags & WindowFlags::NoTitleBar) && !window.dock_is_active)
    if g.style.frame_border_size > 0
        && window.flags.contains(&WindowFlags::NoTitleBar) == false
        && window.dock_is_active == false
    {
        let mut y = window.pos.y + window.title_bar_height() - 1;
        window.draw_list.add_line(
            Vector2D::new(window.pos.x + border_size, y),
            Vector2D::new(window.pos.x + window.size.x - border_size, y),
            color_u32_from_style_color(StyleColor::Border, 0.0),
            g.style.frame_border_size,
        );
    }
}

/// Draw background and borders
/// Draw and handle scrollbars
/// void ImGui::RenderWindowDecorations(Window* window, const Rect& title_bar_rect, bool title_bar_is_highlight, bool handle_borders_and_resize_grips, int resize_grip_count, const ImU32 resize_grip_col[4], float resize_grip_draw_size)
pub fn render_window_decorations(
    g: &mut Context,
    window: &mut Window,
    title_bar_rect: &Rect,
    title_bar_is_highlight: bool,
    handle_borders_and_resize_grips: bool,
    resize_grip_count: i32,
    resize_grip_col: [u32; 4],
    resize_grip_draw_size: f32,
) {
    // ImGuiContext& g = *GImGui;
    // ImGuiStyle& style = g.style;
    let style = &g.style;
    // WindowFlags flags = window.flags;
    let flags = &window.flags;

    // Ensure that ScrollBar doesn't read last frame's skip_items
    // IM_ASSERT(window.begin_count == 0);
    window.skip_items = false;

    // Draw window + handle manual resize
    // As we highlight the title bar when want_focus is set, multiple reappearing windows will have have their title bar highlighted on their reappearing frame.
    // let window_rounding = window.window_rounding;
    let window_rounding = window.window_rounding;
    // let window_border_size = window.window_border_size;
    let window_border_size = window.window_border_size;
    if window.collapsed {
        // Title bar only
        // float backup_border_size = style.frame_border_size;
        let backup_border_size = style.frame_border_size;
        g.style.frame_border_size = window.WindowBorderSize;
        // ImU32 title_bar_col = get_color_u32((title_bar_is_highlight && !g.nav_disable_highlight) ? Color::TitleBgActive : Color::TitleBgCollapsed);
        let title_bar_col =
            get_color_u32_no_alpha(if title_bar_is_highlight && !g.nav_disable_highlight {
                StyleColor::TitleBgActive
            } else {
                StyleColor::TitleBgCollapsed
            });
        // RenderFrame(title_bar_rect.min, title_bar_rect.max, title_bar_col, true, window_rounding);
        render_Frame(
            &title_bar_rect.min,
            &title_bar_rect.max,
            title_bar_col,
            true,
            window_rounding,
        );
        g.style.frame_border_size = backup_border_size;
    } else {
        // window background
        if !(flags.contains(&WindowFlags::NoBackground)) {
            // bool is_docking_transparent_payload = false;
            let mut is_docking_transparent_payload = false;

            if g.drag_drop_active
                && (g.frame_count - g.drag_drop_accept_fraame_count) <= 1
                && g.io.config_docking_transparent_payload
            {
                if g.drag_drop_payload.is_data_type(PAYLOAD_TYPE_WINDOW)
                    && g.drag_drop_payload.data == window
                {
                    is_docking_transparent_payload = true;
                }
            }

            let mut bg_col = get_color_u32_no_alpha(get_window_bg_color_idx(window));
            if window.viewport_owned {
                // No alpha
                bg_col = (bg_col | COLOR32_A_MASK);
                if is_docking_transparent_payload {
                    window.viewport.alpha *= DOCKING_TRANSPARENT_PAYLOAD_ALPHA;
                }
            } else {
                // Adjust alpha. For docking
                let mut override_alpha = false;
                let mut alpha = 1.0;
                if g.next_window_data
                    .flags
                    .contains(&NextWindowDataFlags::HasBgAlpha)
                {
                    alpha = g.next_window_data.bg_alpha_val;
                    override_alpha = true;
                }
                if is_docking_transparent_payload {
                    alpha *= DOCKING_TRANSPARENT_PAYLOAD_ALPHA; // FIXME-DOCK: Should that be an override?
                    override_alpha = true;
                }
                if override_alpha {
                    bg_col = (bg_col & !COLOR32_A_MASK)
                        | (f32_to_int8_sat(alpha) << IM_COL32_A_SHIFT);
                }
            }

            // Render, for docked windows and host windows we ensure bg goes before decorations
            let bg_draw_list = if window.dock_is_active {
                // window.dock_node.host_window.draw_list
                let win = g.window_mut(window.dock_node_id.host_window_id).unwrap();
                g.draw_list_mut(win.draw_list_id).unwrap()
            } else {
                g.draw_list_mut(window.draw_list_id).unwrap()
            };

            if window.dock_is_active || (flags.contains(&WindowFlags::DockNodeHost)) {
                bg_draw_list.channels_set_current(0);
            }
            if window.dock_is_active {
                window.dock_node_id.last_bg_color = bg_col;
            }

            bg_draw_list.add_rect_filled(
                &window.pos + Vector2D::new(0.0, window.title_bar_height()),
                &window.pos + &window.size,
                bg_col,
                window_rounding,
                if flags.contains(&WindowFlags::NoTitleBar) {
                    &HashSet::from([])
                } else {
                    &HashSet::from([DrawFlags::RoundCornersBottom])
                },
            );

            // if (window.dock_is_active || (flags & WindowFlags::DockNodeHost)) {
            if window.dock_is_active || flags.contains(&WindowFlags::DockNodeHost) {
                bg_draw_list.channels_set_current(1);
            }
        }
        if window.dock_is_active {
            window.dock_node_id.is_bg_drawn_this_frame = true;
        }

        // Title bar
        // (when docked, dock_node are drawing their own title bar. Individual windows however do NOT set the _NoTitleBar flag,
        // in order for their pos/size to be matching their undocking state.)
        if !(flags.contains(&WindowFlags::NoTitleBar)) && !window.dock_is_active {
            let title_bar_col = get_color_u32_no_alpha(if title_bar_is_highlight {
                StyleColor::TitleBgActive
            } else {
                StyleColor::TitleBg
            });
            window.draw_list.add_rect_filled(
                &title_bar_rect.min,
                &title_bar_rect.max,
                title_bar_col,
                window_rounding,
                DrawFlags::RoundCornersTop,
            );
        }

        // Menu bar
        if flags.contains(&WindowFlags::MenuBar) {
            let menu_bar_rect = window.menu_bar_rect();
            menu_bar_rect.clip_width(window.rect()); // Soft clipping, in particular child window don't have minimum size covering the menu bar so this is useful for them.
            window.draw_list.add_rect_filled(
                menu_bar_rect.min + Vector2D::new(window_border_size, 0.0),
                menu_bar_rect.max - Vector2D::new(window_border_size, 0.0),
                get_color_u32_no_alpha(StyleColor::MenuBarBg),
                if flags.contains(&WindowFlags::NoTitleBar) {
                    window_rounding
                } else {
                    0.0
                },
                &HashSet::from([DrawFlags::RoundCornersTop]),
            );
            if style.frame_border_size > 0.0 && menu_bar_rect.max.y < window.pos.y + window.size.y {
                let x = window.draw_list.add_line(
                    menu_bar_rect.get_bl(),
                    menu_bar_rect.GetBR(),
                    get_color_u32_no_alpha(StyleColor::Border),
                    style.frame_border_size,
                );
            }
        }

        // Docking: Unhide tab bar (small triangle in the corner), drag from small triangle to quickly undock
        // ImGuiDockNode* node = window.dock_node;
        let node = &mut window.dock_node_id;
        if window.dock_is_active && node.is_hidden_tab_bar() && !node.is_no_tab_bar() {
            // float unhide_sz_draw = f32::floor(g.FontSize * 0.70);
            let unhide_sz_draw = f32::floor(g.font_size * 0.70);
            let unhide_sz_hit = f32::floor(g.font_size * 0.55);
            let p = &mut node.pos;
            // Rect r(p, p + Vector2D::new(unhide_sz_hit, unhide_sz_hit));
            let mut r = Rect::new2(p, p + Vector2D::new(unhide_sz_hit, unhide_sz_hit));
            let unhide_id = window.get_id(g, "#UNHIDE");
            keep_alive_id(unhide_id);
            // bool hovered, held;
            let mut hovered = false;
            let mut held = false;
            if button_behavior(
                r,
                unhide_id,
                &hovered,
                &held,
                &HashSet::from(ButtonFlags::FlattenChildren),
            ) {
                node.want_hidden_tab_bar_toggle = true;
            } else if held && is_mouse_dragging(0) {
                start_mouse_moving_window_or_node(g, window, node, true);
            }

            // FIXME-DOCK: Ideally we'd use ImGuiCol_TitleBgActive/ImGuiCol_TitleBg here, but neither is guaranteed to be visible enough at this sort of size..
            let mut col =
                get_color_u32_no_alpha(if (held && hovered) || (node.is_focused && !hovered) {
                    StyleColor::ButtonActive
                } else {
                    if hovered {
                        StyleColor::ButtonHovered
                    } else {
                        StyleColor::Button
                    }
                });
            window.draw_list.add_triangle_filled(
                p,
                p + Vector2D::new(unhide_sz_draw, 0.0),
                p + Vector2D::new(0.0, unhide_sz_draw),
                col,
            );
        }

        // Scrollbars
        if window.scrollbar_x {
            Scrollbar(Axis::X);
        }
        if window.scrollbar_y {
            Scrollbar(Axis::Y);
        }

        // Render resize grips (after their input handling so we don't have a frame of latency)
        if handle_borders_and_resize_grips && !(flags.contains(&WindowFlags::NoResize)) {
            // for (int resize_grip_n = 0; resize_grip_n < resize_grip_count; resize_grip_n += 1)
            for resize_grip_n in 0..resize_grip_count {
                let grip = &resize_grip_def[resize_grip_n];
                let corner =
                    Vector2D::lerp2(&window.pos, &window.pos + &window.size, grip.corner_pos_n);
                window.draw_list.path_line_to(
                    &corner
                        + grip.inner_dir
                            * if resize_grip_n & 1 {
                                Vector2D::new(window_border_size, resize_grip_draw_size)
                            } else {
                                Vector2D::new(resize_grip_draw_size, window_border_size)
                            },
                );
                window.draw_list.path_line_to(
                    &corner
                        + grip.inner_dir
                            * if resize_grip_n & 1 {
                                Vector2D::new(resize_grip_draw_size, window_border_size)
                            } else {
                                Vector2D::new(window_border_size, resize_grip_draw_size)
                            },
                );
                window.draw_list.path_arc_to_fast(
                    Vector2D::new(
                        &corner.x + grip.inner_dir.x * (window_rounding + window_border_size),
                        &corner.y + grip.inner_dir.y * (window_rounding + window_border_size),
                    ),
                    window_rounding,
                    grip.angle_min_12,
                    grip.angle_max_12,
                );
                window
                    .draw_list
                    .path_fill_convex(resize_grip_col[resize_grip_n]);
            }
        }

        // Borders (for dock node host they will be rendered over after the tab bar)
        if handle_borders_and_resize_grips && !window.dock_node_as_host_id {
            render_window_outer_borders(window);
        }
    }
}

// Render title text, collapse button, close button
// When inside a dock node, this is handled in DockNodeCalcTabBarLayout() instead.
// void ImGui::RenderWindowTitleBarContents(Window* window, const Rect& title_bar_rect, const char* name, bool* p_open)
pub fn render_window_title_bar_contents(
    g: &mut Context,
    window: &mut Window,
    title_bar_rect: &Rect,
    name: &str,
    p_open: Option<&mut bool>,
) {
    // ImGuiContext& g = *GImGui;
    // ImGuiStyle& style = g.style;
    let style: &mut Style = &mut g.style;
    // WindowFlags flags = window.flags;
    let flags: &mut HashSet<WindowFlags> = &mut window.flags;

    // const bool has_close_button = (p_open != None);
    let has_close_button = p_open.is_some();
    // const bool has_collapse_button = !(flags & WindowFlags::NoCollapse) && (style.WindowMenuButtonPosition != Dir::None);
    let has_collapse_button = flags.contains(&WindowFlags::NoCollapse) == false
        && style.window_menu_button_position != Direction::None;

    // Close & Collapse button are on the Menu nav_layer and don't default focus (unless there's nothing else on that layer)
    // FIXME-NAV: Might want (or not?) to set the equivalent of ImGuiButtonFlags_NoNavFocus so that mouse clicks on standard title bar items don't necessarily set nav/keyboard ref?
    // const ImGuiItemFlags item_flags_backup = g.CurrentItemFlags;
    let item_flags_backup = g.current_item_flags.clone();
    // g.CurrentItemFlags |= ImGuiItemFlags_NoNavDefaultFocus;
    g.current_item_flags.insert(ItemFlags::NoNavDefaultFocus);
    window.dcnav_layer_current = NavLayer::Menu;

    // Layout buttons
    // FIXME: Would be nice to generalize the subtleties expressed here into reusable code.
    // float pad_l = style.FramePadding.x;
    let pad_l = style.frame_padding.x;
    // float pad_r = style.FramePadding.x;
    let pad_r = style.frame_padding.x;
    // float button_sz = g.font_size;
    let button_sz = g.font_size.clone();
    // Vector2D close_button_pos;
    let mut close_button_pos = Vector2D::default();
    // Vector2D collapse_button_pos;
    let mut collpasse_button_pos = Vector2D::default();
    if has_close_button {
        pad_r += button_sz;
        close_button_pos = Vector2D::new(
            title_bar_rect.max.x - pad_r - style.frame_padding.x,
            title_bar_rect.min.y,
        );
    }
    if has_collapse_button && style.window_menu_button_position == Direction::Right {
        pad_r += button_sz;
        collapse_button_pos = Vector2D::new(
            title_bar_rect.max.x - pad_r - style.frame_padding.x,
            title_bar_rect.min.y,
        );
    }
    if has_collapse_button && style.window_menu_button_position == Direction::Left {
        collapse_button_pos = Vector2D::new(
            title_bar_rect.min.x + pad_l - style.frame_padding.x,
            title_bar_rect.min.y,
        );
        pad_l += button_sz;
    }

    // Collapse button (submitting first so it gets priority when choosing a navigation init fallback)
    if has_collapse_button {
        if collapse_button(window.get_id(g, "#COLLAPSE"), collapse_button_pos, None) {
            window.WantCollapseToggle = true;
        } // Defer actual collapsing to next frame as we are too far in the Begin() function
    }
    // Close button
    if has_close_button {
        if close_button(window.get_id(g, "#CLOSE"), close_button_pos) {
            *p_open = false;
        }
    }

    window.dcnav_layer_current = NavLayer::Main;
    g.current_item_flags = item_flags_backup;

    // Title bar text (with: horizontal alignment, avoiding collapse/close button, optional "unsaved document" marker)
    // FIXME: Refactor text alignment facilities along with render_text helpers, this is WAY too much messy code..
    // let marker_size_x = (flags & WindowFlags::UnsavedDocument) ? button_sz * 0.80 : 0.0;
    let marker_size_x = if flags.contains(&WindowFlags::UnsavedDocument) {
        button_sz * 0.80
    } else {
        0.0
    };
    // const Vector2D text_size = calc_text_size(name, None, true) + Vector2D::new(marker_size_x, 0.0);
    let text_size = calc_text_size(g, name, true, 0.0) + Vector2D::new(marker_size_x, 0.0);

    // As a nice touch we try to ensure that centered title text doesn't get affected by visibility of Close/Collapse button,
    // while uncentered title text will still reach edges correctly.
    if pad_l > style.frame_padding.x {
        pad_l += g.style.item_inner_spacing.x;
    }
    if pad_r > style.frame_padding.x {
        pad_r += g.style.item_inner_spacing.x;
    }
    if style.window_title_align.x > 0.0 && style.window_title_align.x < 1.0 {
        let centerness = saturate_f32(1.0 - f32::abs(style.window_title_align.x - 0.5) * 2.0); // 0.0 on either edges, 1.0 on center
        let pad_extend = f32::min(
            f32::max(pad_l, pad_r),
            title_bar_rect.width() - pad_l - pad_r - text_size.x,
        );
        pad_l = f32::max(pad_l, pad_extend * centerness);
        pad_r = f32::max(pad_r, pad_extend * centerness);
    }

    let layout_r = Rect::new4(
        title_bar_rect.min.x + pad_l,
        title_bar_rect.min.y,
        title_bar_rect.max.x - pad_r,
        title_bar_rect.max.y,
    );
    let mut clip_r = Rect::new4(
        layout_r.min.x,
        layout_r.min.y,
        f32::min(
            layout_r.max.x + g.style.item_inner_spacing.x,
            title_bar_rect.max.x,
        ),
        layout_r.max.y,
    );
    if flags.contains(&WindowFlags::UnsavedDocument) {
        // Vector2D marker_pos;
        let mut marker_pos = Vector2D::default();
        marker_pos.x = f32::clamp(
            layout_r.min.x
                + (layout_r.get_width() - text_size.x) * style.window_title_align.x
                + text_size.x,
            layout_r.min.x,
            layout_r.max.x,
        );
        marker_pos.y = (layout_r.min.y + layout_r.max.y) * 0.5;
        if marker_pos.x > layout_r.min.x {
            render_bullet(
                window.draw_list,
                marker_pos,
                get_color_u32_no_alpha(StyleColor::Text),
            );
            clip_r.max.x = f32::min(clip_r.max.x, &marker_pos.x - (marker_size_x * 0.5));
        }
    }
    //if (g.io.key_shift) window->draw_list->add_rect(layout_r.min, layout_r.max, IM_COL32(255, 128, 0, 255)); // [DEBUG]
    //if (g.io.key_ctrl) window->draw_list->add_rect(clip_r.min, clip_r.max, IM_COL32(255, 128, 0, 255)); // [DEBUG]
    render_text_clipped(
        layout_r.min,
        layout_r.max,
        name,
        &text_size,
        style.window_title_align,
        &clip_r,
    );
}
