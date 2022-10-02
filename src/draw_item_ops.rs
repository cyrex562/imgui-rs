// inline c_void             DebugDrawItemRect(u32 col = IM_COL32(255, 0, 0, 255))    
pub unsafe fn DebugDrawItemRect(col: u32) {
    let g = GImGui; // ImGuiContext& g = *GImGui; 
    let window = g.CurrentWindow;
    GetForegroundDrawList(window).AddRect(g.LastItemData.Rect.Min, g.LastItemData.Rect.Max, col);
}