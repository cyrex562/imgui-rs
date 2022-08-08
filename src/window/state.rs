use std::collections::HashSet;
use crate::condition::Condition;
use crate::Context;
use crate::utils::{add_hash_set, sub_hash_set};
use crate::vectors::vector_2d::Vector2D;
use crate::window::Window;

// static void SetWindowConditionAllowFlags(Window* window, ImGuiCond flags, bool enabled)
pub fn set_window_condition_allow_flags(
    window: &mut Window,
    flags: &mut HashSet<Condition>,
    enabled: bool,
) {
    window.set_window_pos_allow_flags = if enabled {
        // (window.set_window_pos_allow_flags + flags)
        add_hash_set(&window.set_window_collapsed_allow_flags, flags)
    } else {
        // (window.set_window_pos_allow_flags & ~flags)
        sub_hash_set(&window.set_window_pos_allow_flags, flags)
    };
    window.set_window_size_allow_flags = if enabled {
        // (window.set_window_size_allow_flags | flags)
        add_hash_set(&window.set_window_size_allow_flags, flags)
    } else {
        // window.set_window_size_allow_flags & ~flags
        sub_hash_set(&window.set_window_size_allow_flags, flags)
    };
    window.set_window_collapsed_allow_flags = if enabled {
        // (window.set_window_collapsed_allow_flags | flags)
        add_hash_set(&window.set_window_collapsed_allow_flags, flags)
    } else {
        // window.set_window_collapsed_allow_flags & ~flags
        sub_hash_set(&window.set_window_collapsed_allow_flags, flags)
    };
    window.set_window_dock_allow_flags = if enabled {
        // (window.set_window_dock_allow_flags | flags)
        add_hash_set(&window.set_window_dock_allow_flags, flags)
    } else {
        // (window.set_window_dock_allow_flags & ~flags)
        sub_hash_set(&window.set_window_dock_allow_flags, flags)
    };
}

// void ImGui::set_window_hit_test_hole(Window* window, const Vector2D& pos, const Vector2D& size)
pub fn set_window_hit_test_hole(g: &mut Context, window: &mut Window, pos: &Vector2D, size: &Vector2D)
{
    // IM_ASSERT(window.hit_test_hole_size.x == 0);     // We don't support multiple holes/hit test filters
    window.hit_test_hole_size = Vector2D(size);
    window.HitTestHoleOffset = Vector2D(pos - window.pos);
}
