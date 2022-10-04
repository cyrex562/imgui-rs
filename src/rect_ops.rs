
bool IsRectVisible(const size: &ImVec2)
{
    let mut window: *mut ImGuiWindow =  GimGui.CurrentWindow;
    return window.ClipRect.Overlaps(ImRect::new(window.DC.CursorPos, window.DC.CursorPos + size));
}

bool IsRectVisible(const rect_min: &ImVec2, const rect_max: &ImVec2)
{
    let mut window: *mut ImGuiWindow =  GimGui.CurrentWindow;
    return window.ClipRect.Overlaps(ImRect::new(rect_min, rect_max));
}
