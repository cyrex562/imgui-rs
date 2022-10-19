use crate::GImGui;
use crate::math_ops::ImClamp;
use crate::rect::ImRect;
use crate::utils::{flag_clear, is_not_null};
use crate::vec2::ImVec2;
use crate::window::ImGuiWindow;
use crate::window::window_flags::ImGuiWindowFlags_NoTitleBar;

// [Internal] Small optimization to avoid calls to PopClipRect/SetCurrentChannel/PushClipRect in sequences,
// they would meddle many times with the underlying ImDrawCmd.
// Instead, we do a preemptive overwrite of clipping rectangle _without_ altering the command-buffer and let
// the subsequent single call to SetCurrentChannel() does it things once.
// c_void SetWindowClipRectBeforeSetChannel(window: *mut ImGuiWindow, const ImRect& clip_rect)
pub fn SetWindowClipRectBeforeSetChannel(window: *mut ImGuiWindow, clip_rect: &ImRect) {
    let mut clip_rect_vec4 = clip_rect.ToVec4();
    window.ClipRect = clip_rect.ToVec4();
    window.DrawList._CmdHeader.ClipRect = clip_rect_vec4;
    window.DrawList._ClipRectStack[window.DrawList._ClipRectStack.len() - 1] =
        clip_rect_vec4.clone();
}

// inline ImRect           WindowRectRelToAbs(window: *mut ImGuiWindow, const ImRect& r)
pub fn WindowRectRelToAbs(window: *mut ImGuiWindow, r: &ImRect) -> ImRect {
    let off = window.DC.CursorStartPos.clone();
    ImRect::from_floats(
        r.Min.x + off.x,
        r.Min.y + off.y,
        r.Max.x + off.x,
        r.Max.y + off.y,
    )
}

// inline ImRect           WindowRectAbsToRel(window: *mut ImGuiWindow, const ImRect& r)
pub fn WindowRectAbsToRel(window: *mut ImGuiWindow, r: &ImRect) -> ImRect {
    let mut off: ImVec2 = window.DC.CursorStartPos.clone();
    return ImRect::from_floats(
        r.Min.x - off.x,
        r.Min.y - off.y,
        r.Max.x - off.x,
        r.Max.y - off.y,
    );
}

// Push a clipping rectangle for both ImGui logic (hit-testing etc.) and low-level ImDrawList rendering.
// - When using this function it is sane to ensure that float are perfectly rounded to integer values,
//   so that e.g. (max.x-min.x) in user's render produce correct result.
// - If the code here changes, may need to update code of functions like NextColumn() and PushColumnClipRect():
//   some frequently called functions which to modify both channels and clipping simultaneously tend to use the
//   more specialized SetWindowClipRectBeforeSetChannel() to avoid extraneous updates of underlying ImDrawCmds.
// c_void PushClipRect(const clip_rect_min: &mut ImVec2, const clip_rect_max: &mut ImVec2, intersect_with_current_clip_rect: bool)
pub unsafe fn PushClipRect(
    clip_rect_min: &ImVec2,
    clip_rect_max: &ImVec2,
    intersect_with_current_clip_rect: bool,
) {
    let mut window: *mut ImGuiWindow = GetCurrentWindow();
    window.DrawList.PushClipRect(
        clip_rect_min,
        clip_rect_max,
        intersect_with_current_clip_rect,
    );
    window.ClipRect = window.DrawList._ClipRectStack.last().unwrap().clone();
}

// c_void PopClipRect()
pub unsafe fn PopClipRect() {
    let mut window: *mut ImGuiWindow = GetCurrentWindow();
    window.DrawList.PopClipRect();
    window.ClipRect = window.DrawList._ClipRectStack.last().unwrap().clone();
}


pub unsafe fn ClampWindowRect(window: *mut ImGuiWindow, visibility_rect: &ImRect)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut size_for_clamping: ImVec2 = window.Size;
    if g.IO.ConfigWindowsMoveFromTitleBarOnly && (flag_clear(window.Flags, ImGuiWindowFlags_NoTitleBar) || is_not_null(window.DockNodeAsHost)) {
        size_for_clamping.y = GetFrameHeight();
    } // Not using window.TitleBarHeight() as DockNodeAsHost will report 0.0 here.
    window.Pos = ImClamp(window.Pos, visibility_rect.Min - size_for_clamping, visibility_rect.Max);
}
