#![allow(non_snake_case)]

use crate::rect::ImRect;
use crate::window::ImGuiWindow;

// [Internal] Small optimization to avoid calls to PopClipRect/SetCurrentChannel/PushClipRect in sequences,
// they would meddle many times with the underlying ImDrawCmd.
// Instead, we do a preemptive overwrite of clipping rectangle _without_ altering the command-buffer and let
// the subsequent single call to SetCurrentChannel() does it things once.
// c_void ImGui::SetWindowClipRectBeforeSetChannel(*mut ImGuiWindow window, const ImRect& clip_rect)
pub fn SetWindowClipRectBeforeSetChannel(window: *mut ImGuiWindow, clip_rect: &ImRect) {
    let mut clip_rect_vec4 = clip_rect.ToVec4();
    window.ClipRect = clip_rect.clone();
    window.DrawList._CmdHeader.ClipRect = clip_rect_vec4;
    window.DrawList._ClipRectStack[window.DrawList._ClipRectStack.Size - 1] = clip_rect_vec4.clone();
}


// inline ImRect           WindowRectRelToAbs(*mut ImGuiWindow window, const ImRect& r) 
pub fn WindowRectRelToAbs(window: *mut ImGuiWindow, r: &ImRect) -> ImRect {
    let off = window.DC.CursorStartPos.clone();
    ImRect::new4(r.Min.x + off.x, r.Min.y + off.y, r.Max.x + off.x, r.Max.y + off.y)
}