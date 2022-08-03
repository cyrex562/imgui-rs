use crate::{Context, INVALID_ID};
use crate::globals::GImGui;
use crate::vectors::vector_2d::Vector2D;

/// In window space (not screen space!)
/// Vector2D GetWindowContentRegionMin()
pub fn get_window_content_region_min(g: &mut Context) -> Vector2D {
    let window = g.current_window_mut();
    return &window.content_region_rect.min - &window.pos;
}

pub fn get_content_region_max(g: &mut Context) -> Vector2D {
    // FIXME: This is in window space (not screen space!).
    // ImGuiContext& g = *GImGui;
    // ImGuiWindow* window = g.current_window;
    let window = g.current_window_mut();
    let mut mx: Vector2D = &window.content_region_rect.max - &window.pos;
    if window.dc.current_columns.is_some() || g.current_table != INVALID_ID {
        mx.x = window.work_rect.max.x - window.pos.x;
    }
    return mx;
}

/// [Internal] Absolute coordinate. Saner. This is not exposed until we finishing refactoring work rect features.
pub fn get_content_region_max_abs(g: &mut Context) -> Vector2D {
    // ImGuiContext& g = *GImGui;
    // ImGuiWindow* window = g.current_window;
    let window = g.current_window_mut();
    let mut mx: Vector2D = window.content_region_rect.max.clone();
    if window.dc.current_columns.is_some() || g.current_table != INVALID_ID {
        mx.x = window.work_rect.max.x;
    }
    return mx;
}


pub fn get_content_region_avail(g: &mut Context) -> Vector2D {
    // ImGuiWindow* window = g.current_window_id;
    let window = g.current_window_mut();
    return get_content_region_max_abs(g) - window.dc.cursor_pos.clone();
}

pub fn get_window_content_region_max(g: &mut Context) -> Vector2D {
    // ImGuiWindow* window = g.current_window_id;
    let window = g.current_window_mut();
    return &window.content_region_rect.max - &window.pos;
}
