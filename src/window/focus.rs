use crate::Context;
use crate::globals::GImGui;

// void ImGui::SetWindowFocus()
pub fn set_window_focus(g: &mut Context)
{
    focus_window(GImGui.CurrentWindow);
}

// void ImGui::SetWindowFocus(const char* name)
pub fn set_window_focus2(g: &mut Context, name: &str)
{
    if (name)
    {
        if (ImGuiWindow* window = FindWindowByName(name))
            focus_window(window);
    }
    else
    {
        focus_window(NULL);
    }
}

// void PopFocusScope()
pub fn pop_focus_scope(g: &mut Context)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    IM_ASSERT(g.FocusScopeStack.size > 0); // Too many PopFocusScope() ?
    window.dc.NavFocusScopeIdCurrent = g.FocusScopeStack.back();
    g.FocusScopeStack.pop_back();
}

// void SetItemDefaultFocus()
pub fn set_item_default_focus(g: &mut Context)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    if (!window.Appearing)
        return;
    if (g.nav_window != window.root_window_for_nav || (!g.NavInitRequest && g.NavInitResultId == 0) || g.NavLayer != window.dcnav_layer_current)
        return;

    g.NavInitRequest = false;
    g.NavInitResultId = g.last_item_data.id;
    g.NavInitResultRectRel = window_rect_abs_to_rel(window, g.last_item_data.Rect);
    NavUpdateAnyRequestFlag();

    // scroll could be done in NavInitRequestApplyResult() via a opt-in flag (we however don't want regular init requests to scroll)
    if (!IsItemVisible())
        ScrollToRectEx(window, g.last_item_data.Rect, ImGuiScrollFlags_None);
}
