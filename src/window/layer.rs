use crate::Context;
use crate::globals::GImGui;
use crate::window::{Window, WindowFlags};

// void ImGui::BringWindowToFocusFront(ImGuiWindow* window)
pub fn bring_window_to_focus_front(g: &mut Context, window: &mut Window)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(window == window.root_window);

    const int cur_order = window.focus_order;
    // IM_ASSERT(g.windows_focus_order[cur_order] == window);
    if (g.windows_focus_order.back() == window)
        return;

    const int new_order = g.windows_focus_order.size - 1;
    for (int n = cur_order; n < new_order; n += 1)
    {
        g.windows_focus_order[n] = g.windows_focus_order[n + 1];
        g.windows_focus_order[n].FocusOrder--;
        // IM_ASSERT(g.windows_focus_order[n].FocusOrder == n);
    }
    g.windows_focus_order[new_order] = window;
    window.focus_order = new_order;
}


// void ImGui::bring_window_to_display_front(ImGuiWindow* window)
pub fn bring_window_to_display_front(g: &mut Context, window: &mut Window)
{
    // ImGuiContext& g = *GImGui;
    ImGuiWindow* current_front_window = g.windows.back();
    if (current_front_window == window || current_front_window.root_window_dock_tree == window) // Cheap early out (could be better)
        return;
    for (int i = g.windows.len() - 2; i >= 0; i--) // We can ignore the top-most window
        if (g.windows[i] == window)
        {
            memmove(&g.windows[i], &g.windows[i + 1], (g.windows.len() - i - 1) * sizeof(ImGuiWindow*));
            g.windows[g.windows.len() - 1] = window;
            break;
        }
}

// void ImGui::BringWindowToDisplayBack(ImGuiWindow* window)
pub fn bring_window_to_display_back(g: &mut Context, window: &mut Window)
{
    // ImGuiContext& g = *GImGui;
    if (g.windows[0] == window)
        return;
    for (int i = 0; i < g.windows.len(); i += 1)
        if (g.windows[i] == window)
        {
            memmove(&g.windows[1], &g.windows[0], i * sizeof(ImGuiWindow*));
            g.windows[0] = window;
            break;
        }
}

// void ImGui::BringWindowToDisplayBehind(ImGuiWindow* window, ImGuiWindow* behind_window)
pub fn bring_window_to_display_behind(g: &mut Context, window: &mut Window, behind_window: &mut Window)
{
    // IM_ASSERT(window != NULL && behind_window != NULL);
    // ImGuiContext& g = *GImGui;
    window = window.root_window;
    behind_window = behind_window.root_window;
    int pos_wnd = FindWindowDisplayIndex(window);
    int pos_beh = FindWindowDisplayIndex(behind_window);
    if (pos_wnd < pos_beh)
    {
        size_t copy_bytes = (pos_beh - pos_wnd - 1) * sizeof(ImGuiWindow*);
        memmove(&g.windows.data[pos_wnd], &g.windows.data[pos_wnd + 1], copy_bytes);
        g.windows[pos_beh - 1] = window;
    }
    else
    {
        size_t copy_bytes = (pos_wnd - pos_beh) * sizeof(ImGuiWindow*);
        memmove(&g.windows.data[pos_beh + 1], &g.windows.data[pos_beh], copy_bytes);
        g.windows[pos_beh] = window;
    }
}


// Moving window to front of display and set focus (which happens to be back of our sorted list)
// void ImGui::focus_window(ImGuiWindow* window)
pub fn focus_window(g: &mut Context, window: &mut Window)
{
    // ImGuiContext& g = *GImGui;

    if (g.nav_window != window)
    {
        SetNavWindow(window);
        if (window && g.nav_disable_mouse_hover)
            g.NavMousePosDirty = true;
        g.nav_id = window ? window.NavLastIds[0] : 0; // Restore nav_id
        g.NavLayer = NavLayer::Main;
        g.NavFocusScopeId = 0;
        g.NavIdIsAlive = false;
    }

    // Close popups if any
    close_popups_over_window(window, false);

    // Move the root window to the top of the pile
    // IM_ASSERT(window == NULL || window.root_window_dock_tree != NULL);
    ImGuiWindow* focus_front_window = window ? window.root_window : NULL;
    ImGuiWindow* display_front_window = window ? window.root_window_dock_tree : NULL;
    ImGuiDockNode* dock_node = window ? window.dock_node_id: NULL;
    bool active_id_window_is_dock_node_host = (g.active_id_window && dock_node && dock_node.host_window == g.active_id_window);

    // Steal active widgets. Some of the cases it triggers includes:
    // - Focus a window while an InputText in another window is active, if focus happens before the old InputText can run.
    // - When using Nav to activate menu items (due to timing of activating on press->new window appears->losing active_id)
    // - Using dock host items (tab, collapse button) can trigger this before we redirect the active_id_window toward the child window.
    if (g.active_id != 0 && g.active_id_window && g.active_id_window.root_window != focus_front_window)
        if (!g.ActiveIdNoClearOnFocusLoss && !active_id_window_is_dock_node_host)
            clear_active_id();

    // Passing NULL allow to disable keyboard focus
    if (!window)
        return;
    window.LastFrameJustFocused = g.frame_count;

    // Select in dock node
    if (dock_node && dock_node.tab_bar)
        dock_node.tab_bar.selected_tab_id = dock_node.tab_bar.next_selected_tab_id = window.tab_id;

    // Bring to front
    BringWindowToFocusFront(focus_front_window);
    if (((window.flags | focus_front_window.flags | display_front_window.flags) & WindowFlags::NoBringToFrontOnFocus) == 0)
        bring_window_to_display_front(display_front_window);
}


// void ImGui::FocusTopMostWindowUnderOne(ImGuiWindow* under_this_window, ImGuiWindow* ignore_window)
pub fn focus_top_most_window_under_one(g: &mut Context, window: &mut under_this_window, ignore_window: &mut Window)
{
    // ImGuiContext& g = *GImGui;
    int start_idx = g.windows_focus_order.size - 1;
    if (under_this_window != NULL)
    {
        // Aim at root window behind us, if we are in a child window that's our own root (see #4640)
        int offset = -1;
        while (under_this_window.flags & WindowFlags::ChildWindow)
        {
            under_this_window = under_this_window.parent_window;
            offset = 0;
        }
        start_idx = FindWindowFocusIndex(under_this_window) + offset;
    }
    for (int i = start_idx; i >= 0; i--)
    {
        // We may later decide to test for different NoXXXInputs based on the active navigation input (mouse vs nav) but that may feel more confusing to the user.
        ImGuiWindow* window = g.windows_focus_order[i];
        // IM_ASSERT(window == window.root_window);
        if (window != ignore_window && window.was_active)
            if ((window.flags & (WindowFlags::NoMouseInputs | WindowFlags::NoNavInputs)) != (WindowFlags::NoMouseInputs | WindowFlags::NoNavInputs))
            {
                // FIXME-DOCK: This is failing (lagging by one frame) for docked windows.
                // If A and B are docked into window and B disappear, at the NewFrame() call site window->nav_last_child_nav_window will still point to B.
                // We might leverage the tab order implicitly stored in window->dock_node_as_host->tab_bar (essentially the 'most_recently_selected_tab' code in tab bar will do that but on next update)
                // to tell which is the "previous" window. Or we may leverage 'LastFrameFocused/last_frame_just_focused' and have this function handle child window itself?
                ImGuiWindow* focus_window = NavRestoreLastChildNavWindow(window);
                focus_window(focus_window);
                return;
            }
    }
    focus_window(NULL);
}
