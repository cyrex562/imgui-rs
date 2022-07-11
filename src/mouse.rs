use crate::context::Context;
use crate::rect::Rect;

/// Test if mouse cursor is hovering given rectangle
/// NB- Rectangle is clipped by our current clip setting
/// NB- Expand the rectangle to be generous on imprecise inputs systems (g.style.TouchExtraPadding)
/// bool ImGui::IsMouseHoveringRect(const Vector2D& r_min, const Vector2D& r_max, bool clip)
pub fn is_mouse_hovering_rect(g: &mut Context, r_min: &Vector2D, r_max: &Vector2D, clip: bool) -> bool {
    // ImGuiContext& g = *GImGui;

    // Clip
    // ImRect rect_clipped(r_min, r_max);
    let mut rect_clipped = Rect {
        min: r_min.clone(),
        max: r_max.clone(),
    };

    if clip {
        let curr_win = g.get_current_window()?;
        rect_clipped.ClipWith(&curr_win.clip_rect);
    }

    // Expand for touch input
    let min_1 = rect_clipped.min - g.style.TouchExtraPadding;
    let max_1 = rect_clipped.max - g.style.TouchExtraPadding;
    let rect_for_touch = Rect::new2(&min_1, &max_1);
    if !rect_for_touch.Contains(g.IO.MousePos) {
        return false;
    }
    if !g.MouseViewport.GetMainRect().Overlaps(&rect_clipped){
        return false;
    }
    return true;
}
