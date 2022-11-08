use crate::core::context::ImguiContext;
use crate::frame_ops::GetFrameHeight;
use crate::math_ops::ImClamp;
use crate::rect::ImRect;
use crate::utils::{flag_clear, is_not_null};
use crate::vec2::ImVec2;
use crate::window::ops::GetCurrentWindow;
use crate::window::window_flags::ImGuiWindowFlags_NoTitleBar;
use crate::window::ImguiWindow;
use crate::GImGui;

// [Internal] Small optimization to avoid calls to PopClipRect/SetCurrentChannel/PushClipRect in sequences,
// they would meddle many times with the underlying ImDrawCmd.
// Instead, we do a preemptive overwrite of clipping rectangle _without_ altering the command-buffer and let
// the subsequent single call to SetCurrentChannel() does it things once.
// c_void SetWindowClipRectBeforeSetChannel(window: &mut ImGuiWindow, const ImRect& clip_rect)
pub fn SetWindowClipRectBeforeSetChannel(window: &mut ImguiWindow, clip_rect: &ImRect) {
    let mut clip_rect_vec4 = clip_rect.ToVec4();
    window.ClipRect = clip_rect.ToVec4();
    window.DrawList._CmdHeader.ClipRect = clip_rect_vec4;
    window.DrawList._ClipRectStack[window.DrawList._ClipRectStack.len() - 1] =
        clip_rect_vec4.clone();
}

// inline ImRect           WindowRectRelToAbs(window: &mut ImGuiWindow, const ImRect& r)
pub fn WindowRectRelToAbs(window: &mut ImguiWindow, r: &ImRect) -> ImRect {
    let off = window.dc.cursor_start_pos.clone();
    ImRect::from_floats(
        r.min.x + off.x,
        r.min.y + off.y,
        r.max.x + off.x,
        r.max.y + off.y,
    )
}

// inline ImRect           WindowRectAbsToRel(window: &mut ImGuiWindow, const ImRect& r)
pub fn window_rect_abs_to_rel(window: &ImguiWindow, r: &ImRect) -> ImRect {
    let mut off = window.dc.cursor_start_pos;
    ImRect::from_floats(
        r.min.x - off.x,
        r.min.y - off.y,
        r.max.x - off.x,
        r.max.y - off.y,
    )
}

// Push a clipping rectangle for both ImGui logic (hit-testing etc.) and low-level ImDrawList rendering.
// - When using this function it is sane to ensure that float are perfectly rounded to integer values,
//   so that e.g. (max.x-min.x) in user's render produce correct result.
// - If the code here changes, may need to update code of functions like NextColumn() and PushColumnClipRect():
//   some frequently called functions which to modify both channels and clipping simultaneously tend to use the
//   more specialized SetWindowClipRectBeforeSetChannel() to avoid extraneous updates of underlying ImDrawCmds.
// c_void PushClipRect(const clip_rect_min: &mut ImVec2, const clip_rect_max: &mut ImVec2, intersect_with_current_clip_rect: bool)
pub fn PushClipRect(
    g: &mut ImguiContext,
    clip_rect_min: &ImVec2,
    clip_rect_max: &ImVec2,
    intersect_with_current_clip_rect: bool,
) {
    let mut window = g.current_window_mut();
    window.DrawList.PushClipRect(
        clip_rect_min,
        clip_rect_max,
        intersect_with_current_clip_rect,
    );
    window.ClipRect = window.DrawList._ClipRectStack.last().unwrap().clone();
}

// c_void PopClipRect()
pub fn PopClipRect(g: &mut ImguiContext) {
    let mut window = g.current_window_mut().unwrap();
    window.DrawList.PopClipRect();
    window.ClipRect = window.DrawList._ClipRectStack.last().unwrap().clone();
}

pub unsafe fn ClampWindowRect(window: &mut ImguiWindow, visibility_rect: &ImRect) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut size_for_clamping = window.Size;
    if g.IO.ConfigWindowsMoveFromTitleBarOnly
        && (flag_clear(window.Flags, ImGuiWindowFlags_NoTitleBar)
            || window.DockNodeAsHost.is_some())
    {
        size_for_clamping.y = GetFrameHeight();
    } // Not using window.TitleBarHeight() as DockNodeAsHost will report 0.0 here.
    window.position = ImClamp(
        window.position,
        visibility_rect.min - size_for_clamping,
        visibility_rect.max,
    );
}
