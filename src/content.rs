use crate::Context;
use crate::globals::GImGui;
use crate::vectors::two_d::Vector2D;

// In window space (not screen space!)
// Vector2D GetWindowContentRegionMin()
pub fn get_window_content_region_min(g: &mut Context) -> Vector2D
{
    ImGuiWindow* window = g.CurrentWindow;
    return window.ContentRegionRect.min - window.pos;
}

// FIXME: This is in window space (not screen space!).
// Vector2D GetContentRegionMax()
pub fn get_content_region_max(g: &mut Context) -> Vector2D
{
    // ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    Vector2D mx = window.ContentRegionRect.max - window.pos;
    if (window.dc.CurrentColumns || g.CurrentTable)
        mx.x = window.WorkRect.max.x - window.pos.x;
    return mx;
}

// [Internal] Absolute coordinate. Saner. This is not exposed until we finishing refactoring work rect features.
// Vector2D GetContentRegionMaxAbs()
pub fn get_content_region_max_abs(g: &mut Context) -> Vector2D
{
    // ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    Vector2D mx = window.ContentRegionRect.max;
    if (window.dc.CurrentColumns || g.CurrentTable)
        mx.x = window.WorkRect.max.x;
    return mx;
}

// Vector2D GetContentRegionAvail()
pub fn get_content_region_avail(g: &mut Context) -> Vector2D
{
    ImGuiWindow* window = g.CurrentWindow;
    return GetContentRegionMaxAbs() - window.dc.cursor_pos;
}

// Vector2D GetWindowContentRegionMax()
pub fn get_window_content_region_max(g: &mut Context) -> Vector2D
{
    ImGuiWindow* window = g.CurrentWindow;
    return window.ContentRegionRect.max - window.pos;
}
