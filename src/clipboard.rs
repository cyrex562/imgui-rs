use std::os::raw::c_char;
use crate::imgui_context::ImGuiContext;

// void ImGui::SetClipboardText(const char* text)
pub fn SetClipboardText(g: *mut ImGuiContext, text: *const c_char)
{
    // ImGuiContext& g = *GImGui;
    if g.IO.SetClipboardTextFn {
        g.IO.SetClipboardTextFn(g.IO.ClipboardUserData, text);
    }
}
