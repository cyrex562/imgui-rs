// FIXME: All the Contents Region function are messy or misleading. WE WILL AIM TO OBSOLETE ALL OF THEM WITH A NEW "WORK RECT" API. Thanks for your patience!

use crate::GImGui;
use crate::vec2::ImVec2;
use crate::window::ImGuiWindow;

// FIXME: This is in window space (not screen space!).
// ImVec2 GetContentRegionMax()
pub unsafe fn GetContentRegionMax() -> ImVec2 {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    let mut mx: ImVec2 = window.ContentRegionRect.Max - window.Pos;
    if window.DC.CurrentColumns.is_null() == false || g.CurrentTable.is_null() == false {
        mx.x = window.WorkRect.Max.x - window.Pos.x;
    }
    return mx;
}

// [Internal] Absolute coordinate. Saner. This is not exposed until we finishing refactoring work rect features.
// ImVec2 GetContentRegionMaxAbs()
pub unsafe fn GetContentRegionMaxAbs() -> ImVec2 {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    let mut mx: ImVec2 = window.ContentRegionRect.Max;
    if window.DC.CurrentColumns.is_null() == false || g.CurrentTable.is_null() == false {
        mx.x = window.WorkRect.Max.x;
    }
    return mx;
}

// ImVec2 GetContentRegionAvail()
pub unsafe fn GetContentRegionAvail() -> ImVec2 {
    let mut window: *mut ImGuiWindow = GimGui.CurrentWindow;
    return GetContentRegionMaxAbs() - window.DC.CursorPos;
}

// In window space (not screen space!)
// ImVec2 GetWindowContentRegionMin()
pub unsafe fn GetWindowContentRegionMin() -> ImVec2 {
    let mut window: *mut ImGuiWindow = GimGui.CurrentWindow;
    return window.ContentRegionRect.Min - window.Pos;
}

// ImVec2 GetWindowContentRegionMax()
pub unsafe fn GetWindowContentRegionMax() -> ImVec2 {
    let mut window: *mut ImGuiWindow = GimGui.CurrentWindow;
    return window.ContentRegionRect.Max - window.Pos;
}
