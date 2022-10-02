
// FIXME: All the Contents Region function are messy or misleading. WE WILL AIM TO OBSOLETE ALL OF THEM WITH A NEW "WORK RECT" API. Thanks for your patience!

// FIXME: This is in window space (not screen space!).
ImVec2 GetContentRegionMax()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    let mx: ImVec2 = window.ContentRegionRect.Max - window.Pos;
    if (window.DC.CurrentColumns || g.CurrentTable)
        mx.x = window.WorkRect.Max.x - window.Pos.x;
    return mx;
}

// [Internal] Absolute coordinate. Saner. This is not exposed until we finishing refactoring work rect features.
ImVec2 GetContentRegionMaxAbs()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    let mx: ImVec2 = window.ContentRegionRect.Max;
    if (window.DC.CurrentColumns || g.CurrentTable)
        mx.x = window.WorkRect.Max.x;
    return mx;
}

ImVec2 GetContentRegionAvail()
{
    let mut window: *mut ImGuiWindow =  GimGui.CurrentWindow;
    return GetContentRegionMaxAbs() - window.DC.CursorPos;
}

// In window space (not screen space!)
ImVec2 GetWindowContentRegionMin()
{
    let mut window: *mut ImGuiWindow =  GimGui.CurrentWindow;
    return window.ContentRegionRect.Min - window.Pos;
}

ImVec2 GetWindowContentRegionMax()
{
    let mut window: *mut ImGuiWindow =  GimGui.CurrentWindow;
    return window.ContentRegionRect.Max - window.Pos;
}
