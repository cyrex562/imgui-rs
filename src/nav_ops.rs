

// We get there when either NavId == id, or when g.NavAnyRequest is set (which is updated by NavUpdateAnyRequestFlag above)
// This is called after LastItemData is set.
pub unsafe fn NavProcessItem()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    const let mut id: ImGuiID =  g.LastItemData.ID;
    const let nav_bb: ImRect =  g.LastItemData.NavRect;
    const let mut item_flags: ImGuiItemFlags =  g.LastItemData.InFlags;

    // Process Init Request
    if g.NavInitRequest && g.NavLayer == window.DC.NavLayerCurrent && (item_flags & ImGuiItemFlags_Disabled) == 0
    {
        // Even if 'ImGuiItemFlags_NoNavDefaultFocus' is on (typically collapse/close button) we record the first ResultId so they can be used as a fallback
        let candidate_for_nav_default_focus: bool = (item_flags & ImGuiItemFlags_NoNavDefaultFocus) == 0;
        if candidate_for_nav_default_focus || g.NavInitResultId == 0
        {
            g.NavInitResultId = id;
            g.NavInitResultRectRel = WindowRectAbsToRel(window, nav_bb);
        }
        if candidate_for_nav_default_focus
        {
            g.NavInitRequest = false; // Found a match, clear request
            NavUpdateAnyRequestFlag();
        }
    }

    // Process Move Request (scoring for navigation)
    // FIXME-NAV: Consider policy for double scoring (scoring from NavScoringRect + scoring from a rect wrapped according to current wrapping policy)
    if (g.NavMoveScoringItems)
    {
        let is_tab_stop: bool = (item_flags & ImGuiItemFlags_Inputable) && (item_flags & (ImGuiItemFlags_NoTabStop | ImGuiItemFlags_Disabled)) == 0;
        let is_tabbing: bool = (g.NavMoveFlags & ImGuiNavMoveFlags_Tabbing) != 0;
        if (is_tabbing)
        {
            if (is_tab_stop || (g.NavMoveFlags & ImGuiNavMoveFlags_FocusApi))
                NavProcessItemForTabbingRequest(id);
        }
        else if ((g.NavId != id || (g.NavMoveFlags & ImGuiNavMoveFlags_AllowCurrentNavId)) && !(item_flags & (ImGuiItemFlags_Disabled | ImGuiItemFlags_NoNav)))
        {
            ImGuiNavItemData* result = (window == g.NavWindow) ? &g.NavMoveResultLocal : &g.NavMoveResultOther;
            if (!is_tabbing)
            {
                if (NavScoreItem(result))
                    NavApplyItemToResult(result);

                // Features like PageUp/PageDown need to maintain a separate score for the visible set of items.
                let VISIBLE_RATIO: c_float =  0.70f32;
                if ((g.NavMoveFlags & ImGuiNavMoveFlags_AlsoScoreVisibleSet) && window.ClipRect.Overlaps(nav_bb))
                    if (ImClamp(nav_bb.Max.y, window.ClipRect.Min.y, window.ClipRect.Max.y) - ImClamp(nav_bb.Min.y, window.ClipRect.Min.y, window.ClipRect.Max.y) >= (nav_bb.Max.y - nav_bb.Min.y) * VISIBLE_RATIO)
                        if (NavScoreItem(&g.NavMoveResultLocalVisible))
                            NavApplyItemToResult(&g.NavMoveResultLocalVisible);
            }
        }
    }

    // Update window-relative bounding box of navigated item
    if (g.NavId == id)
    {
        if (g.NavWindow != window)
            SetNavWindow(window); // Always refresh g.NavWindow, because some operations such as FocusItem() may not have a window.
        g.NavLayer = window.DC.NavLayerCurrent;
        g.NavFocusScopeId = window.DC.NavFocusScopeIdCurrent;
        g.NavIdIsAlive = true;
        window.NavRectRel[window.DC.NavLayerCurrent] = WindowRectAbsToRel(window, nav_bb);    // Store item bounding box (relative to window position)
    }
}
