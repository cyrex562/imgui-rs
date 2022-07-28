use crate::Context;
use crate::imgui_h::ImGuiID;
use crate::imgui_vec::{ImVec1, Vector2D};
use crate::item::ItemStatusFlags;
use crate::vectors::two_d::Vector2D;

// Stacked storage data for BeginGroup()/EndGroup()
#[derive(Default,Debug,Clone)]
pub struct GroupData
{
    // ImGuiID     WindowID;
    pub WindowID: ImGuiID,
    // Vector2D      BackupCursorPos;
    pub BackupCursorPos: Vector2D,
    // Vector2D      BackupCursorMaxPos;
    pub BackupCursorMaxPos: Vector2D,
    // ImVec1      BackupIndent;
    pub BackupIndent: ImVec1,
    // ImVec1      BackupGroupOffset;
    pub BackupGroupOffset: ImVec1,
    // Vector2D      BackupCurrLineSize;
    pub BackupCurrLineSize: Vector2D,
    // float       BackupCurrLineTextBaseOffset;
    pub BackupCurrLineTextBaseOffset: f32,
    // ImGuiID     BackupActiveIdIsAlive;
    pub BackupActiveIdIsAlive: ImGuiID,
    // bool        BackupActiveIdPreviousFrameIsAlive;
    pub BackupActiveIdPreviousFrameIsAlive: bool,
    // bool        BackupHoveredIdIsAlive;
    pub BackupHoveredIdIsAlive: bool,
    // bool        EmitItem;
    pub EmitItem: bool,
}

// Lock horizontal starting position + capture group bounding box into one "item" (so you can use IsItemHovered() or layout primitives such as same_line() on whole group, etc.)
// Groups are currently a mishmash of functionalities which should perhaps be clarified and separated.
// FIXME-OPT: Could we safely early out on ->skip_items?
// void BeginGroup()
pub fn begin_group(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;

    g.group_stack.resize(g.group_stack.size + 1);
    ImGuiGroupData& group_data = g.group_stack.back();
    group_data.WindowID = window.id;
    group_data.BackupCursorPos = window.dc.cursor_pos;
    group_data.BackupCursorMaxPos = window.dc.cursor_max_pos;
    group_data.BackupIndent = window.dc.indent;
    group_data.BackupGroupOffset = window.dc.GroupOffset;
    group_data.BackupCurrLineSize = window.dc.CurrLineSize;
    group_data.BackupCurrLineTextBaseOffset = window.dc.CurrLineTextBaseOffset;
    group_data.BackupActiveIdIsAlive = g.active_id_is_alive;
    group_data.BackupHoveredIdIsAlive = g.hovered_id != 0;
    group_data.BackupActiveIdPreviousFrameIsAlive = g.active_id_previous_frame_is_alive;
    group_data.EmitItem = true;

    window.dc.GroupOffset.x = window.dc.cursor_pos.x - window.pos.x - window.dc.columns_offset.x;
    window.dc.indent = window.dc.GroupOffset;
    window.dc.cursor_max_pos = window.dc.cursor_pos;
    window.dc.CurrLineSize = Vector2D::new(0.0, 0.0);
    if (g.LogEnabled)
        g.log_line_pos_y = -f32::MAX; // To enforce a carriage return
}

// void EndGroup()
pub fn end_group(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    // IM_ASSERT(g.group_stack.size > 0); // Mismatched BeginGroup()/EndGroup() calls

    ImGuiGroupData& group_data = g.group_stack.back();
    // IM_ASSERT(group_data.WindowID == window.id); // EndGroup() in wrong window?

    Rect group_bb(group_data.BackupCursorPos, ImMax(window.dc.cursor_max_pos, group_data.BackupCursorPos));

    window.dc.cursor_pos = group_data.BackupCursorPos;
    window.dc.cursor_max_pos = ImMax(group_data.BackupCursorMaxPos, window.dc.cursor_max_pos);
    window.dc.indent = group_data.BackupIndent;
    window.dc.GroupOffset = group_data.BackupGroupOffset;
    window.dc.CurrLineSize = group_data.BackupCurrLineSize;
    window.dc.CurrLineTextBaseOffset = group_data.BackupCurrLineTextBaseOffset;
    if (g.LogEnabled)
        g.log_line_pos_y = -f32::MAX; // To enforce a carriage return

    if (!group_data.EmitItem)
    {
        g.group_stack.pop_back();
        return;
    }

    window.dc.CurrLineTextBaseOffset = ImMax(window.dc.PrevLineTextBaseOffset, group_data.BackupCurrLineTextBaseOffset);      // FIXME: Incorrect, we should grab the base offset from the *first line* of the group but it is hard to obtain now.
    item_size(group_bb.GetSize());
    item_add(group_bb, 0, None, ItemFlags::NoTabStop);

    // If the current active_id was declared within the boundary of our group, we copy it to LastItemId so IsItemActive(), is_item_deactivated() etc. will be functional on the entire group.
    // It would be be neater if we replaced window.dc.LastItemId by e.g. 'bool LastItemIsActive', but would put a little more burden on individual widgets.
    // Also if you grep for LastItemId you'll notice it is only used in that context.
    // (The two tests not the same because active_id_is_alive is an id itself, in order to be able to handle active_id being overwritten during the frame.)
    const bool group_contains_curr_active_id = (group_data.BackupActiveIdIsAlive != g.active_id) && (g.active_id_is_alive == g.active_id) && g.active_id;
    const bool group_contains_prev_active_id = (group_data.BackupActiveIdPreviousFrameIsAlive == false) && (g.active_id_previous_frame_is_alive == true);
    if (group_contains_curr_active_id)
        g.last_item_data.id = g.active_id;
    else if (group_contains_prev_active_id)
        g.last_item_data.id = g.active_id_previous_frame;
    g.last_item_data.Rect = group_bb;

    // Forward Hovered flag
    const bool group_contains_curr_hovered_id = (group_data.BackupHoveredIdIsAlive == false) && g.hovered_id != 0;
    if (group_contains_curr_hovered_id)
        g.last_item_data.status_flags |= ItemStatusFlags::HoveredWindow;

    // Forward Edited flag
    if (group_contains_curr_active_id && g.active_id_has_been_edited_this_frame)
        g.last_item_data.status_flags |= ImGuiItemStatusFlags_Edited;

    // Forward Deactivated flag
    g.last_item_data.status_flags |= ImGuiItemStatusFlags_HasDeactivated;
    if (group_contains_prev_active_id && g.active_id != g.active_id_previous_frame)
        g.last_item_data.status_flags |= ImGuiItemStatusFlags_Deactivated;

    g.group_stack.pop_back();
    //window->draw_list->add_rect(group_bb.min, group_bb.max, IM_COL32(255,0,255,255));   // [Debug]
}
