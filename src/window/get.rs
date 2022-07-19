// static ImGuiWindow* GetCombinedRootWindow(ImGuiWindow* window, bool popup_hierarchy, bool dock_hierarchy)
pub fn get_combined_root_window(g: &mut Context, window: &mut Window, popup_hierarchy: bool, dock_hierarchy: bool) -> &mut Window
{
    ImGuiWindow* last_window = NULL;
    while (last_window != window)
    {
        last_window = window;
        window = window.root_window;
        if (popup_hierarchy)
            window = window.root_window_popup_tree;
		if (dock_hierarchy)
			window = window.root_window_dock_tree;
	}
    return window;
}



// bool ImGui::IsWindowChildOf(ImGuiWindow* window, ImGuiWindow* potential_parent, bool popup_hierarchy, bool dock_hierarchy)
pub fn is_window_child_of(g: &mut Context, window: &mut Window, potential_parent: &mut Window, popup_hierarchy: bool, dock_hierarchy: bool) -> bool
{
    ImGuiWindow* window_root = GetCombinedRootWindow(window, popup_hierarchy, dock_hierarchy);
    if (window_root == potential_parent)
        return true;
    while (window != NULL)
    {
        if (window == potential_parent)
            return true;
        if (window == window_root) // end of chain
            return false;
        window = window.parent_window;
    }
    return false;
}

// bool ImGui::is_window_within_begin_stack_of(ImGuiWindow* window, ImGuiWindow* potential_parent)
pub fn is_window_within_begin_stack_of(g: &mut Context, window: &mut Window, potential_parent: &mut Window) -> bool
{
    if (window.root_window == potential_parent)
        return true;
    while (window != NULL)
    {
        if (window == potential_parent)
            return true;
        window = window.ParentWindowInBeginStack;
    }
    return false;
}

// ImGuiID ImGui::GetWindowDockID()
pub fn get_window_dock_id(g: &mut Context) -> Id32
{
    ImGuiContext& g = *GImGui;
    return g.current_window.dock_id;
}

// float ImGui::GetWindowWidth()
pub fn get_window_width(g: &mut Context) -> f32
{
    ImGuiWindow* window = GImGui.CurrentWindow;
    return window.size.x;
}

// float ImGui::GetWindowHeight()
pub fn get_window_height(g: &mut Context) -> f32
{
    ImGuiWindow* window = GImGui.CurrentWindow;
    return window.size.y;
}