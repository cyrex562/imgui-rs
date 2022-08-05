use crate::Context;
use crate::vectors::vector_2d::Vector2D;

/// Push a clipping rectangle for both ImGui logic (hit-testing etc.) and low-level ImDrawList rendering.
/// - When using this function it is sane to ensure that float are perfectly rounded to integer values,
///   so that e.g. (max.x-min.x) in user's render produce correct result.
/// - If the code here changes, may need to update code of functions like NextColumn() and PushColumnClipRect():
///   some frequently called functions which to modify both channels and clipping simultaneously tend to use the
///   more specialized SetWindowClipRectBeforeSetChannel() to avoid extraneous updates of underlying ImDrawCmds.
// void ImGui::PushClipRect(const Vector2D& clip_rect_min, const Vector2D& clip_rect_max, bool intersect_with_current_clip_rect)
pub fn push_clip_rect(
    g: &mut Context,
    clip_rect_min: &Vector2D,
    clip_rect_max: &Vector2D,
    intersect_with_current_clip_rect: bool,
) {
    // Window* window = GetCurrentWindow();
    let window = g.current_window_mut().unwrap();
    // window.draw_list->PushClipRect(clip_rect_min, clip_rect_max, intersect_with_current_clip_rect);
    let draw_list = g.draw_list_mut(window.draw_list_id).unwrap();
    draw_list.push_clip_rect(
        clip_rect_min,
        clip_rect_max,
        intersect_with_current_clip_rect,
    );
    // window.ClipRect = window.draw_list->_ClipRectStack.back();
    window.clip_rect = draw_list.clip_rect_stack.last().unwrap().clone()
}

// void ImGui::PopClipRect()
pub fn pop_clip_rect(g: &mut Context) {
    // Window* window = GetCurrentWindow();
    let window = g.current_window_mut().unwrap();
    // window.draw_list->PopClipRect();
    let draw_list = g.draw_list_mut(window.draw_list_id).unwrap();
    draw_list.pop_clip_rect();
    // window.ClipRect = window.draw_list->_ClipRectStack.back();
    window.clip_rect = draw_list.clip_rect_stack.last().unwrap().clone();
}
