use crate::rect::ImRect;
use crate::vec2::ImVec2;
use crate::window::ImGuiWindow;

pub fn IsRectVisible(size: &ImVec2) -> bool {
    let mut window: *mut ImGuiWindow = GimGui.CurrentWindow;
    return window.ClipRect.Overlaps(&ImRect::new2(&window.DC.CursorPos, window.DC.CursorPos + size));
}

pub fn IsRectVisible2(rect_min: &ImVec2, rect_max: &ImVec2) -> bool {
    let mut window: *mut ImGuiWindow = GimGui.CurrentWindow;
    return window.ClipRect.Overlaps(&ImRect::new2(rect_min, rect_max));
}
