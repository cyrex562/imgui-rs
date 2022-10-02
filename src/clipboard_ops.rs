#![allow(non_snake_case)]

// *const char GetClipboardText()
pub fn GetClipboardText() -> *const c_char
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return if g.IO.GetClipboardTextFn { g.IO.GetClipboardTextFn(g.IO.ClipboardUserData) }else { "" };
}

// c_void SetClipboardText(*const char text)
pub fn SetClipboardText(text: *const c_char)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if g.IO.SetClipboardTextFn {
        g.IO.SetClipboardTextFn(g.IO.ClipboardUserData, text);
    }
}
