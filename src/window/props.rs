use crate::{Context, Viewport};
use crate::globals::GImGui;
use crate::types::Id32;
use crate::window::{Window, WindowFlags};

// static ImGuiColor get_window_bg_color_idx(ImGuiWindow* window)
pub fn get_window_bg_color_idx(window: &mut Window)
{
    // if (window.flags & (WindowFlags::Tooltip | WindowFlags::Popup))
    if window.flags.contains(&WindowFlags::Tooltip) && window.flags.contains(&WindowFlags::Popup)
    {
        return StyleColor::PopupBg;
    }
    // if ((window.flags & WindowFlags::ChildWindow) && !window.dock_is_active)
   if window.flags.contains(WindowFlags::ChildWindow) && window.dock_is_active == false
    {
        return StyleColor::ChildBg;
    }
    return StyleColor::WindowBg;
}

// ImDrawList* ImGui::GetWindowDrawList()
pub fn get_window_draw_list(g: &mut Context)
{
    ImGuiWindow* window = GetCurrentWindow();
    return window.draw_list;
}

// float ImGui::GetWindowDpiScale()
pub fn get_window_dpi_scale(g: &mut Context) -> f32
{
    // ImGuiContext& g = *GImGui;
    return g.CurrentDpiScale;
}

// ImGuiViewport* ImGui::GetWindowViewport()
pub fn get_window_viewport(g: &mut Context) -> &mut Viewport
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.current_viewport != NULL && g.current_viewport == g.current_window.Viewport);
    return g.current_viewport;
}

// void PushFocusScope(ImGuiID id)
pub fn push_focus_scope(g: &mut Context, id: Id32)
{
    // ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    g.FocusScopeStack.push_back(window.dc.NavFocusScopeIdCurrent);
    window.dc.NavFocusScopeIdCurrent = id;
}
