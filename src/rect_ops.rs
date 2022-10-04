
bool IsRectVisible(const ImVec2& size)
{
    let mut window: *mut ImGuiWindow =  GimGui.CurrentWindow;
    return window.ClipRect.Overlaps(ImRect::new(window.DC.CursorPos, window.DC.CursorPos + size));
}

bool IsRectVisible(const ImVec2& rect_min, const ImVec2& rect_max)
{
    let mut window: *mut ImGuiWindow =  GimGui.CurrentWindow;
    return window.ClipRect.Overlaps(ImRect::new(rect_min, rect_max));
}
