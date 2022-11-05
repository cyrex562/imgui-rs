// GetContentRegionAvail: ImVec2()

use crate::utils::is_not_null;
use crate::vec2::ImVec2;
use crate::window::ImGuiWindow;
use crate::GImGui;

pub unsafe fn GetContentRegionAvail() -> ImVec2 {
    let g = GImGui;
    let mut window: &mut ImGuiWindow = g.CurrentWindow;
    return GetContentRegionMaxAbs() - window.DC.CursorPos;
}

// [Internal] Absolute coordinate. Saner. This is not exposed until we finishing refactoring work rect features.
pub unsafe fn GetContentRegionMaxAbs() -> ImVec2 {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    let mut mx: ImVec2 = window.ContentRegionRect.Max;
    if is_not_null(window.DC.CurrentColumns) || is_not_null(g.CurrentTable) {
        mx.x = window.WorkRect.Max.x;
    }
    return mx;
}


// FIXME: This is in window space (not screen space!).
// GetContentRegionMax: ImVec2()
pub unsafe fn GetContentRegionMax() -> ImVec2 {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    let mut mx: ImVec2 = window.ContentRegionRect.Max - window.Pos;
    if is_not_null(window.DC.CurrentColumns) || is_not_null(g.CurrentTable) {
        mx.x = window.WorkRect.Max.x - window.Pos.x;
    }
    return mx;
}


// In window space (not screen space!)
pub unsafe fn GetWindowContentRegionMin() -> ImVec2 {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window: &mut ImGuiWindow = g.CurrentWindow;
    return window.ContentRegionRect.Min - window.Pos;
}

pub unsafe fn GetWindowContentRegionMax() -> ImVec2 {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window: &mut ImGuiWindow = g.CurrentWindow;
    return window.ContentRegionRect.Max - window.Pos;
}
