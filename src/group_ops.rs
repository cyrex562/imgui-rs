
// Lock horizontal starting position + capture group bounding box into one "item" (so you can use IsItemHovered() or layout primitives such as SameLine() on whole group, etc.)
// Groups are currently a mishmash of functionalities which should perhaps be clarified and separated.
// FIXME-OPT: Could we safely early out on .SkipItems?
c_void BeginGroup()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;

    g.GroupStack.resize(g.GroupStack.Size + 1);
    ImGuiGroupData& group_data = g.GroupStack.last().unwrap();
    group_data.WindowID = window.ID;
    group_data.BackupCursorPos = window.DC.CursorPos;
    group_data.BackupCursorMaxPos = window.DC.CursorMaxPos;
    group_data.BackupIndent = window.DC.Indent;
    group_data.BackupGroupOffset = window.DC.GroupOffset;
    group_data.BackupCurrLineSize = window.DC.CurrLineSize;
    group_data.BackupCurrLineTextBaseOffset = window.DC.CurrLineTextBaseOffset;
    group_data.BackupActiveIdIsAlive = g.ActiveIdIsAlive;
    group_data.BackupHoveredIdIsAlive = g.HoveredId != 0;
    group_data.BackupActiveIdPreviousFrameIsAlive = g.ActiveIdPreviousFrameIsAlive;
    group_data.EmitItem = true;

    window.DC.GroupOffset.x = window.DC.CursorPos.x - window.Pos.x - window.DC.ColumnsOffset.x;
    window.DC.Indent = window.DC.GroupOffset;
    window.DC.CursorMaxPos = window.DC.CursorPos;
    window.DC.CurrLineSize = ImVec2::new2(0f32, 0f32);
    if (g.LogEnabled)
        g.LogLinePosY = -f32::MAX; // To enforce a carriage return
}

c_void EndGroup()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    // IM_ASSERT(g.GroupStack.Size > 0); // Mismatched BeginGroup()/EndGroup() calls

    ImGuiGroupData& group_data = g.GroupStack.last().unwrap();
    // IM_ASSERT(group_data.WindowID == window.ID); // EndGroup() in wrong window?

    if (window.DC.IsSetPos)
        ErrorCheckUsingSetCursorPosToExtendParentBoundaries();

    let mut group_bb: ImRect = ImRect::new(group_data.BackupCursorPos, ImMax(window.DC.CursorMaxPos, group_data.BackupCursorPos));

    window.DC.CursorPos = group_data.BackupCursorPos;
    window.DC.CursorMaxPos = ImMax(group_data.BackupCursorMaxPos, window.DC.CursorMaxPos);
    window.DC.Indent = group_data.BackupIndent;
    window.DC.GroupOffset = group_data.BackupGroupOffset;
    window.DC.CurrLineSize = group_data.BackupCurrLineSize;
    window.DC.CurrLineTextBaseOffset = group_data.BackupCurrLineTextBaseOffset;
    if (g.LogEnabled)
        g.LogLinePosY = -f32::MAX; // To enforce a carriage return

    if (!group_data.EmitItem)
    {
        g.GroupStack.pop_back();
        return;
    }

    window.DC.CurrLineTextBaseOffset = ImMax(window.DC.PrevLineTextBaseOffset, group_data.BackupCurrLineTextBaseOffset);      // FIXME: Incorrect, we should grab the base offset from the *first line* of the group but it is hard to obtain now.
    ItemSize(group_bb.GetSize());
    ItemAdd(group_bb, 0, null_mut(), ImGuiItemFlags_NoTabStop);

    // If the current ActiveId was declared within the boundary of our group, we copy it to LastItemId so IsItemActive(), IsItemDeactivated() etc. will be functional on the entire group.
    // It would be be neater if we replaced window.DC.LastItemId by e.g. 'bool LastItemIsActive', but would put a little more burden on individual widgets.
    // Also if you grep for LastItemId you'll notice it is only used in that context.
    // (The two tests not the same because ActiveIdIsAlive is an ID itself, in order to be able to handle ActiveId being overwritten during the frame.)
    let group_contains_curr_active_id: bool = (group_data.BackupActiveIdIsAlive != g.ActiveId) && (g.ActiveIdIsAlive == g.ActiveId) && g.ActiveId;
    let group_contains_prev_active_id: bool = (group_data.BackupActiveIdPreviousFrameIsAlive == false) && (g.ActiveIdPreviousFrameIsAlive == true);
    if (group_contains_curr_active_id)
        g.LastItemData.ID = g.ActiveId;
    else if (group_contains_prev_active_id)
        g.LastItemData.ID = g.ActiveIdPreviousFrame;
    g.LastItemData.Rect = group_bb;

    // Forward Hovered flag
    let group_contains_curr_hovered_id: bool = (group_data.BackupHoveredIdIsAlive == false) && g.HoveredId != 0;
    if (group_contains_curr_hovered_id)
        g.LastItemData.StatusFlags |= ImGuiItemStatusFlags_HoveredWindow;

    // Forward Edited flag
    if (group_contains_curr_active_id && g.ActiveIdHasBeenEditedThisFrame)
        g.LastItemData.StatusFlags |= ImGuiItemStatusFlags_Edited;

    // Forward Deactivated flag
    g.LastItemData.StatusFlags |= ImGuiItemStatusFlags_HasDeactivated;
    if (group_contains_prev_active_id && g.ActiveId != g.ActiveIdPreviousFrame)
        g.LastItemData.StatusFlags |= ImGuiItemStatusFlags_Deactivated;

    g.GroupStack.pop_back();
    //window.DrawList.AddRect(group_bb.Min, group_bb.Max, IM_COL32(255,0,255,255));   // [Debug]
}