use crate::Context;

// void ImGui::PushTextWrapPos(float wrap_pos_x)
pub fn push_text_wrap_pos(g: &mut Context, wrap_pos_x: f32)
{
    // Window* window = GetCurrentWindow();
    let window = g.current_window_mut();
    window.dc.text_wrap_pos_stack.push_back(window.dc.text_wrap_pos);
    window.dc.text_wrap_pos = wrap_pos_x;
}

// void ImGui::PopTextWrapPos()
pub fn pop_text_wrap_pos(g: &mut Context)
{
    // Window* window = GetCurrentWindow();
    let window = g.current_window_mut();
    window.dc.TextWrapPos = window.dc.TextWrapPosStack.back();
    window.dc.TextWrapPosStack.pop_back();
}
