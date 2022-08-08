use crate::condition::Condition;
use crate::Context;
use crate::globals::GImGui;
use crate::window::Window;

// void ImGui::SetWindowCollapsed(Window* window, bool collapsed, ImGuiCond cond)
pub fn set_window_collapsed(g: &mut Context, window: &mut Window, collapsed: bool, cond: Condition)
{
    // Test condition (NB: bit 0 is always true) and clear flags for next time
    if (cond && (window.set_window_collapsed_allow_flags & cond) == 0)
        return;
    window.set_window_collapsed_allow_flags &= ~(ImGuiCond_Once | Condition::FirstUseEver | ImGuiCond_Appearing);

    // Set
    window.collapsed = collapsed;
}

// void ImGui::SetWindowCollapsed(bool collapsed, ImGuiCond cond)
pub fn set_window_collapsed2(g: &mut Context, collapsed: bool, cond: Condition)
{
    set_window_collapsed(g.current_window_id, collapsed, cond);
}

// bool ImGui::IsWindowCollapsed()
pub fn is_window_collapsed(g: &mut Context) -> bool
{
    Window* window = GetCurrentWindowRead();
    return window.collapsed;
}

// bool ImGui::IsWindowAppearing()
pub fn is_window_appearing(g: &mut Context) -> bool
{
    Window* window = GetCurrentWindowRead();
    return window.Appearing;
}

// void ImGui::SetWindowCollapsed(const char* name, bool collapsed, ImGuiCond cond)
pub fn set_window_collapsed3(g: &mut Context, name: &str, collapsed: bool, cond: Condition)
{
    if (Window* window = find_window_by_name(name))
        SetWindowCollapsed(window, collapsed, cond);
}
