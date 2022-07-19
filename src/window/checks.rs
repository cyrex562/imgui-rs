

// bool ImGui::is_window_above(ImGuiWindow* potential_above, ImGuiWindow* potential_below)
pub fn is_window_above(g: &mut Context, potential_above: &mut Window, potential_below: &mut Window)
{
    ImGuiContext& g = *GImGui;

    // It would be saner to ensure that display layer is always reflected in the g.windows[] order, which would likely requires altering all manipulations of that array
    const int display_layer_delta = get_window_display_layer(potential_above) - get_window_display_layer(potential_below);
    if (display_layer_delta != 0)
        return display_layer_delta > 0;

    for (int i = g.windows.size - 1; i >= 0; i--)
    {
        ImGuiWindow* candidate_window = g.windows[i];
        if (candidate_window == potential_above)
            return true;
        if (candidate_window == potential_below)
            return false;
    }
    return false;
}


// bool ImGui::IsWindowHovered(ImGuiHoveredFlags flags)
pub fn is_window_hovered(g: &mut Context, flags: &mut HashSet<HoveredFlags>) -> bool
{
    IM_ASSERT((flags & (ImGuiHoveredFlags_AllowWhenOverlapped | ImGuiHoveredFlags_AllowWhenDisabled)) == 0);   // flags not supported by this function
    ImGuiContext& g = *GImGui;
    ImGuiWindow* ref_window = g.hovered_window;
    ImGuiWindow* cur_window = g.current_window;
    if (ref_window == NULL)
        return false;

    if ((flags & ImGuiHoveredFlags_AnyWindow) == 0)
    {
        IM_ASSERT(cur_window); // Not inside a Begin()/End()
        const bool popup_hierarchy = (flags & ImGuiHoveredFlags_NoPopupHierarchy) == 0;
        const bool dock_hierarchy = (flags & ImGuiHoveredFlags_DockHierarchy) != 0;
        if (flags & ImGuiHoveredFlags_RootWindow)
            cur_window = GetCombinedRootWindow(cur_window, popup_hierarchy, dock_hierarchy);

        bool result;
        if (flags & ImGuiHoveredFlags_ChildWindows)
            result = IsWindowChildOf(ref_window, cur_window, popup_hierarchy, dock_hierarchy);
        else
            result = (ref_window == cur_window);
        if (!result)
            return false;
    }

    if (!IsWindowContentHoverable(ref_window, flags))
        return false;
    if (!(flags & ImGuiHoveredFlags_AllowWhenBlockedByActiveItem))
        if (g.active_id != 0 && !g.ActiveIdAllowOverlap && g.active_id != ref_window.move_id)
            return false;
    return true;
}



// bool ImGui::IsWindowFocused(ImGuiFocusedFlags flags)
pub fn is_window_focused(g: &mut Context, flags: &mut HashSet<FocusedFlags>)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* ref_window = g.nav_window;
    ImGuiWindow* cur_window = g.current_window;

    if (ref_window == NULL)
        return false;
    if (flags & ImGuiFocusedFlags_AnyWindow)
        return true;

    IM_ASSERT(cur_window); // Not inside a Begin()/End()
    const bool popup_hierarchy = (flags & ImGuiFocusedFlags_NoPopupHierarchy) == 0;
    const bool dock_hierarchy = (flags & ImGuiFocusedFlags_DockHierarchy) != 0;
    if (flags & ImGuiHoveredFlags_RootWindow)
        cur_window = GetCombinedRootWindow(cur_window, popup_hierarchy, dock_hierarchy);

    if (flags & ImGuiHoveredFlags_ChildWindows)
        return IsWindowChildOf(ref_window, cur_window, popup_hierarchy, dock_hierarchy);
    else
        return (ref_window == cur_window);
}

// bool ImGui::IsWindowDocked()
pub fn is_window_docked(g: &mut Context) -> bool
{
    ImGuiContext& g = *GImGui;
    return g.current_window.dock_is_active;
}

// Can we focus this window with CTRL+TAB (or PadMenu + PadFocusPrev/PadFocusNext)
// Note that NoNavFocus makes the window not reachable with CTRL+TAB but it can still be focused with mouse or programmatically.
// If you want a window to never be focused, you may use the e.g. NoInputs flag.
// bool ImGui::IsWindowNavFocusable(ImGuiWindow* window)
pub fn is_window_nav_focusable(g: &mut Context, window: &mut Window) -> bool
{
    return window.was_active && window == window.root_window && !(window.flags & WindowFlags::NoNavFocus);
}