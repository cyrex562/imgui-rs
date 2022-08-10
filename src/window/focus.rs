use crate::Context;
use crate::globals::GImGui;

// void ImGui::SetWindowFocus()
pub fn set_window_focus(g: &mut Context)
{
    focus_window(g.current_window_id);
}

// void ImGui::SetWindowFocus(const char* name)
pub fn set_window_focus2(g: &mut Context, name: &str)
{
    if (name)
    {
        if (Window* window = find_window_by_name(name))
            focus_window(window);
    }
    else
    {
        focus_window(None);
    }
}

// void PopFocusScope()
pub fn pop_focus_scope(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    let window = g.current_window_mut();
    // IM_ASSERT(g.FocusScopeStack.size > 0); // Too many PopFocusScope() ?
    window.dc.NavFocusScopeIdCurrent = g.FocusScopeStack.back();
    g.FocusScopeStack.pop_back();
}

// void SetItemDefaultFocus()
pub fn set_item_default_focus(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    let window = g.current_window_mut();
    if (!window.Appearing)
        return;
    if (g.nav_window != window.root_window_for_nav || (!g.nav_init_request && g.NavInitResultId == 0) || g.nav_layer != window.dcnav_layer_current)
        return;

    g.nav_init_request = false;
    g.NavInitResultId = g.last_item_data.id;
    g.NavInitResultRectRel = window_rect_abs_to_rel(window, g.last_item_data.rect);
    nav_update_any_request_flag();

    // scroll could be done in NavInitRequestApplyResult() via a opt-in flag (we however don't want regular init requests to scroll)
    if (!IsItemVisible())
        scroll_to_rect_ex(window, g.last_item_data.rect, ScrollFlags::None);
}
