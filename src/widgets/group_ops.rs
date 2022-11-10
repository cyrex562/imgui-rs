use crate::cursor_ops::ErrorCheckUsingSetCursorPosToExtendParentBoundaries;
use crate::widgets::group_data::ImGuiGroupData;
use crate::item::item_flags::ImGuiItemFlags_NoTabStop;
use crate::item::item_ops::{ItemAdd, ItemSize};
use crate::item::item_status_flags::{
    ImGuiItemStatusFlags_Deactivated, ImGuiItemStatusFlags_Edited,
    ImGuiItemStatusFlags_HasDeactivated, ImGuiItemStatusFlags_HoveredWindow,
};
use crate::core::math_ops::ImMax;
use crate::rect::ImRect;
use crate::core::vec2::Vector2;
use crate::GImGui;
use std::ptr::null_mut;

// Lock horizontal starting position + capture group bounding box into one "item" (so you can use IsItemHovered() or layout primitives such as SameLine() on whole group, etc.)
// Groups are currently a mishmash of functionalities which should perhaps be clarified and separated.
// FIXME-OPT: Could we safely early out on ->SkipItems?
pub unsafe fn BeginGroup() {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.current_window_mut().unwrap();

    g.GroupStack
        .resize_with(g.GroupStack.Size + 1, ImGuiGroupData::default());
    let mut group_data = g.GroupStack.last_mut().unwrap();
    group_data.WindowID = window.ID;
    group_data.BackupCursorPos = window.dc.cursor_pos;
    group_data.BackupCursorMaxPos = window.dc.CursorMaxPos;
    group_data.BackupIndent = window.dc.indent.clone();
    group_data.BackupGroupOffset = window.dc.group_offset.clone();
    group_data.BackupCurrLineSize = window.dc.CurrLineSize;
    group_data.BackupCurrLineTextBaseOffset = window.dc.CurrLineTextBaseOffset;
    group_data.BackupActiveIdIsAlive = g.ActiveIdIsAlive;
    group_data.BackupHoveredIdIsAlive = g.HoveredId != 0;
    group_data.BackupActiveIdPreviousFrameIsAlive = g.ActiveIdPreviousFrameIsAlive;
    group_data.EmitItem = true;

    window.dc.group_offset.x =
        window.dc.cursor_pos.x - window.position.x - window.dc.ColumnsOffset.x;
    window.dc.indent = window.dc.group_offset.clone();
    window.dc.CursorMaxPos = window.dc.cursor_pos;
    window.dc.CurrLineSize = Vector2::from_floats(0.0, 0.0);
    if g.LogEnabled {
        g.LogLinePosY = -f32::MAX;
    } // To enforce a carriage return
}

pub unsafe fn EndGroup() {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.current_window_mut().unwrap();
    // IM_ASSERT(g.GroupStack.Size > 0); // Mismatched BeginGroup()/EndGroup() calls

    ImGuiGroupData & group_data = g.GroupStack.last().unwrap();
    // IM_ASSERT(group_data.WindowID == window.ID); // EndGroup() in wrong window?

    if (window.dc.is_set_pos) {
        ErrorCheckUsingSetCursorPosToExtendParentBoundaries(g);
    }

    let mut group_bb: ImRect = ImRect::new(
        group_data.BackupCursorPos,
        ImMax(window.dc.CursorMaxPos, group_data.BackupCursorPos),
    );

    window.dc.cursor_pos = group_data.BackupCursorPos;
    window.dc.CursorMaxPos = ImMax(group_data.BackupCursorMaxPos, window.dc.CursorMaxPos);
    window.dc.indent = group_data.BackupIndent;
    window.dc.group_offset = group_data.BackupGroupOffset;
    window.dc.CurrLineSize = group_data.BackupCurrLineSize;
    window.dc.CurrLineTextBaseOffset = group_data.BackupCurrLineTextBaseOffset;
    if g.LogEnabled {
        g.LogLinePosY = -f32::MAX;
    } // To enforce a carriage return

    if !group_data.EmitItem {
        g.GroupStack.pop_back();
        return;
    }

    window.dc.CurrLineTextBaseOffset = ImMax(
        window.dc.PrevLineTextBaseOffset,
        group_data.BackupCurrLineTextBaseOffset,
    ); // FIXME: Incorrect, we should grab the base offset from the *first line* of the group but it is hard to obtain now.
    ItemSize(g, &group_bb.GetSize(), 0.0);
    ItemAdd(g, &mut group_bb, 0, None, ImGuiItemFlags_NoTabStop);

    // If the current ActiveId was declared within the boundary of our group, we copy it to LastItemId so IsItemActive(), IsItemDeactivated() etc. will be functional on the entire group.
    // It would be be neater if we replaced window.dc.LastItemId by e.g. 'LastItemIsActive: bool', but would put a little more burden on individual widgets.
    // Also if you grep for LastItemId you'll notice it is only used in that context.
    // (The two tests not the same because ActiveIdIsAlive is an ID itself, in order to be able to handle ActiveId being overwritten during the frame.)
    let group_contains_curr_active_id: bool = (group_data.BackupActiveIdIsAlive != g.ActiveId)
        && (g.ActiveIdIsAlive == g.ActiveId)
        && g.ActiveId != 0;
    let group_contains_prev_active_id: bool = (group_data.BackupActiveIdPreviousFrameIsAlive
        == false)
        && (g.ActiveIdPreviousFrameIsAlive == true);
    if group_contains_curr_active_id {
        g.last_item_data.ID = g.ActiveId;
    } else if group_contains_prev_active_id {
        g.last_item_data.ID = g.ActiveIdPreviousFrame;
    }
    g.last_item_data.Rect = group_bb;

    // Forward Hovered flag
    let group_contains_curr_hovered_id: bool =
        (group_data.BackupHoveredIdIsAlive == false) && g.HoveredId != 0;
    if group_contains_curr_hovered_id {
        g.last_item_data.StatusFlags |= ImGuiItemStatusFlags_HoveredWindow;
    }

    // Forward Edited flag
    if group_contains_curr_active_id && g.ActiveIdHasBeenEditedThisFrame {
        g.last_item_data.StatusFlags |= ImGuiItemStatusFlags_Edited;
    }

    // Forward Deactivated flag
    g.last_item_data.StatusFlags |= ImGuiItemStatusFlags_HasDeactivated;
    if group_contains_prev_active_id && g.ActiveId != g.ActiveIdPreviousFrame {
        g.last_item_data.StatusFlags |= ImGuiItemStatusFlags_Deactivated;
    }

    g.GroupStack.pop_back();
    //window.DrawList.AddRect(group_bb.Min, group_bb.Max, IM_COL32(255,0,255,255));   // [Debug]
}
