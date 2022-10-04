use crate::draw_list_ops::{GetForegroundDrawList, GetForegroundDrawList2, GetForegroundDrawList3};
use crate::GImGui;

// inline c_void             DebugDrawItemRect(let mut col: u32 = IM_COL32(255, 0, 0, 255))
pub unsafe fn DebugDrawItemRect(col: u32) {
    let g = GImGui; // ImGuiContext& g = *GImGui; 
    let window = g.CurrentWindow;
    GetForegroundDrawList3(window).AddRect(&g.LastItemData.Rect.Min, &g.LastItemData.Rect.Max, col, 0.0, 0, 0.0);
}
