use std::collections::HashSet;
use crate::{Context, INVALID_ID};
use crate::imgui_h::Id32;
use crate::imgui_vec::{Vector1D, Vector2D};
use crate::item::{item_add, item_size, ItemFlags, ItemStatusFlags};
use crate::rect::Rect;
use crate::types::Id32;
use crate::vectors::Vector1D;
use crate::vectors::vector_2d::Vector2D;

// Stacked storage data for BeginGroup()/EndGroup()
#[derive(Default,Debug,Clone)]
pub struct GroupData
{
    // Id32     window_id;
    pub window_id: Id32,
    // Vector2D      backup_cursor_pos;
    pub backup_cursor_pos: Vector2D,
    // Vector2D      backup_cursor_max_pos;
    pub backup_cursor_max_pos: Vector2D,
    // Vector1D      backup_indent;
    pub backup_indent: Vector1D,
    // Vector1D      backup_group_offset;
    pub backup_group_offset: Vector1D,
    // Vector2D      backup_curr_line_size;
    pub backup_curr_line_size: Vector2D,
    // float       backup_curr_line_text_base_offset;
    pub backup_curr_line_text_base_offset: f32,
    // Id32     backup_active_id_is_alive;
    pub backup_active_id_is_alive: Id32,
    // bool        backup_active_id_previous_frame_is_alive;
    pub backup_active_id_previous_frame_is_alive: bool,
    // bool        backup_hovered_id_is_alive;
    pub backup_hovered_id_is_alive: bool,
    // bool        emit_item;
    pub emit_item: bool,
}

// Lock horizontal starting position + capture group bounding box into one "item" (so you can use IsItemHovered() or layout primitives such as same_line() on whole group, etc.)
// Groups are currently a mishmash of functionalities which should perhaps be clarified and separated.
// FIXME-OPT: Could we safely early out on ->skip_items?
// void BeginGroup()
pub fn begin_group(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    // Window* window = g.current_window;
    let window = g.current_window_mut();

    g.group_stack.resize(g.group_stack.size + 1, GroupData::default());
    // GroupData& group_data = g.group_stack.back();
    let group_data = g.group_stack.last_mut().unwrap();
    group_data.window_id = window.id;
    group_data.backup_cursor_pos = window.dc.cursor_pos.clone();
    group_data.backup_cursor_max_pos = window.dc.cursor_max_pos.clone();
    group_data.backup_indent = window.dc.indent.clone();
    group_data.backup_grioup_offset = window.dc.group_offset;
    group_data.backup_curr_line_size = window.dc.curr_line_size.clone();
    group_data.backup_curr_line_text_base_offset = window.dc.curr_line_text_base_offset;
    group_data.backup_active_id_is_alive = g.active_id_is_alive;
    group_data.BackupHoveredIdIsAlive = g.hovered_id != 0;
    group_data.backup_active_id_previous_frame_is_alive = g.active_id_previous_frame_is_alive;
    group_data.emit_item = true;

    window.dc.GroupOffset.x = window.dc.cursor_pos.x - window.pos.x - window.dc.columns_offset.x;
    window.dc.indent = window.dc.GroupOffset;
    window.dc.cursor_max_pos = window.dc.cursor_pos.clone();
    window.dc.curr_line_size = Vector2D::new(0.0, 0.0);
    if g.log_enabled {
        g.log_line_pos_y = -f32::MAX; // To enforce a carriage return
        }
}

// void EndGroup()
pub fn end_group(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    // Window* window = g.current_window;
    let window = g.current_winow_mut();
    // IM_ASSERT(g.group_stack.size > 0); // Mismatched BeginGroup()/EndGroup() calls

    // GroupData& group_data = g.group_stack.back();
    let group_data = g.group_stack.last_mut().unwrap();
    // IM_ASSERT(group_data.window_id == window.id); // EndGroup() in wrong window?

    let mut group_bb = Rect::new(&group_data.backup_cursor_pos, Vector2D::max(&window.dc.cursor_max_pos, &group_data.backup_cursor_pos));

    window.dc.cursor_pos = group_data.backup_cursor_pos.clone();
    window.dc.cursor_max_pos = Vector2D::max(&group_data.backup_cursor_max_pos, window.dc.cursor_max_pos);
    window.dc.indent = group_data.backup_indent;
    window.dc.GroupOffset = group_data.backup_group_offset;
    window.dc.curr_line_size = group_data.backup_curr_line_size.clone();
    window.dc.curr_line_text_base_offset = group_data.backup_curr_line_text_base_offset;
    if g.log_enabled {
        g.log_line_pos_y = -f32::MAX;
    }// To enforce a carriage return

    if !group_data.emit_item
    {
        g.group_stack.pop_back();
        return;
    }

    window.dc.curr_line_text_base_offset = ImMax(window.dc.PrevLineTextBaseOffset, group_data.backup_curr_line_text_base_offset);      // FIXME: Incorrect, we should grab the base offset from the *first line* of the group but it is hard to obtain now.
    item_size(g, &group_bb.size(), 0.0);
    let mut flags = HashSet::from([ItemFlags::NoTabStop]);
    item_add(g, &mut group_bb, 0, None, Some(&mut flags));

    // If the current active_id was declared within the boundary of our group, we copy it to LastItemId so IsItemActive(), is_item_deactivated() etc. will be functional on the entire group.
    // It would be be neater if we replaced window.dc.LastItemId by e.g. 'bool LastItemIsActive', but would put a little more burden on individual widgets.
    // Also if you grep for LastItemId you'll notice it is only used in that context.
    // (The two tests not the same because active_id_is_alive is an id itself, in order to be able to handle active_id being overwritten during the frame.)
    let group_contains_curr_active_id = (group_data.backup_active_id_is_alive != g.active_id) && (g.active_id_is_alive == g.active_id) && g.active_id != INVALID_ID;
    let group_contains_prev_active_id = (group_data.backup_active_id_previous_frame_is_alive == false) && (g.active_id_previous_frame_is_alive == true);
    if group_contains_curr_active_id {
        g.last_item_data.id = g.active_id;
    }
    else if group_contains_prev_active_id {
        g.last_item_data.id = g.active_id_previous_frame;
    }
    g.last_item_data.rect = group_bb;

    // Forward Hovered flag
    let group_contains_curr_hovered_id = (group_data.BackupHoveredIdIsAlive == false) && g.hovered_id != 0;
    if group_contains_curr_hovered_id {
        g.last_item_data.status_flags |= ItemStatusFlags::HoveredWindow;
    }

    // Forward edited flag
    if group_contains_curr_active_id && g.active_id_has_been_edited_this_frame {
        g.last_item_data.status_flags |= ItemStatusFlags::Edited;
    }

    // Forward Deactivated flag
    g.last_item_data.status_flags |= ItemStatusFlags::HasDeactivated;
    if group_contains_prev_active_id && g.active_id != g.active_id_previous_frame {
        g.last_item_data.status_flags |= ItemStatusFlags::Deactivated;
    }

    g.group_stack.pop_back();
    //window->draw_list->add_rect(group_bb.min, group_bb.max, IM_COL32(255,0,255,255));   // [Debug]
}
