// void ImGui::PushTextWrapPos(float wrap_pos_x)
pub fn push_text_wrap_pos(g: &mut Context, wrap_pos_x: f32)
{
    ImGuiWindow* window = GetCurrentWindow();
    window.dc.TextWrapPosStack.push_back(window.dc.TextWrapPos);
    window.dc.TextWrapPos = wrap_pos_x;
}

// void ImGui::PopTextWrapPos()
pub fn pop_text_wrap_pos(g: &mut Context)
{
    ImGuiWindow* window = GetCurrentWindow();
    window.dc.TextWrapPos = window.dc.TextWrapPosStack.back();
    window.dc.TextWrapPosStack.pop_back();
}