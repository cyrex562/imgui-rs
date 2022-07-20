use std::collections::HashSet;
use crate::condition::Condition;
use crate::Context;
use crate::utils::{add_hash_set, sub_hash_set};
use crate::vectors::two_d::Vector2D;
use crate::window::Window;

// static void SetWindowConditionAllowFlags(ImGuiWindow* window, ImGuiCond flags, bool enabled)
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

// void ImGui::SetWindowHitTestHole(ImGuiWindow* window, const Vector2D& pos, const Vector2D& size)
pub fn set_window_hit_test_hole(g: &mut Context, window: &mut Window, pos: &Vector2D, size: &Vector2D)
{
    // IM_ASSERT(window.hit_test_hole_size.x == 0);     // We don't support multiple holes/hit test filters
    window.hit_test_hole_size = Vector2Dih(size);
    window.HitTestHoleOffset = Vector2Dih(pos - window.pos);
}
