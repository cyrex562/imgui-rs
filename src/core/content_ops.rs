// GetContentRegionAvail: ImVec2()

use crate::core::context::AppContext;
use crate::core::utils::is_not_null;
use crate::core::vec2::Vector2;
use crate::window::ImguiWindow;
use crate::GImGui;

pub fn content_region_avail(g: &mut AppContext) -> Vector2 {
    let mut window = g.current_window_mut().unwrap();
    return content_region_max_abs(g) - window.dc.cursor_pos;
}

// [Internal] Absolute coordinate. Saner. This is not exposed until we finishing refactoring work rect features.
pub fn content_region_max_abs(g: &mut AppContext) -> Vector2 {
    let mut window = g.current_window_mut().unwrap();
    let mut mx: Vector2 = window.content_region_rect.max;
    if is_not_null(window.dc.current_columns) || g.current_table.is_some() {
        mx.x = window.work_rect.max.x;
    }
    return mx;
}

// FIXME: This is in window space (not screen space!).
pub fn content_region_max(g: &mut AppContext) -> Vector2 {
    let mut window = g.current_window_mut().unwrap();
    let mut mx: Vector2 = window.content_region_rect.max - window.position;
    if is_not_null(window.dc.current_columns) || g.current_table.is_some() {
        mx.x = window.work_rect.max.x - window.position.x;
    }
    return mx;
}

// In window space (not screen space!)
pub unsafe fn GetWindowContentRegionMin(g: &mut AppContext) -> Vector2 {
    let mut window = g.current_window_mut().unwrap();
    return window.content_region_rect.min - window.position;
}

pub fn GetWindowContentRegionMax(g: &mut AppContext) -> Vector2 {
    // let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.current_window_mut().unwrap();
    return window.content_region_rect.Max - window.position;
}
