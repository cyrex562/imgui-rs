// ImVec2 GetContentRegionAvail()


use crate::GImGui;
use crate::vec2::ImVec2;
use crate::window::ImGuiWindow;

pub unsafe fn GetContentRegionAvail() -> ImVec2
{
    let mut window: *mut ImGuiWindow =  GimGui.CurrentWindow;
    return GetContentRegionMaxAbs() - window.DC.CursorPos;
}

// [Internal] Absolute coordinate. Saner. This is not exposed until we finishing refactoring work rect features.
pub unsafe fn GetContentRegionMaxAbs() -> ImVec2
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    let mut mx: ImVec2 = window.ContentRegionRect.Max;
    if window.DC.CurrentColumns || g.CurrentTable {
        mx.x = window.WorkRect.Max.x;
    }
    return mx;
}
