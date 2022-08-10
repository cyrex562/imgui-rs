use std::collections::HashSet;
use crate::Context;
use crate::math::lerp_f32;
use crate::nav::{scroll_flags_contains_mask_x, scroll_flags_contains_mask_y, scroll_flags_remove_mask_x, scroll_flags_remove_mask_y, ScrollFlags};
use crate::rect::Rect;
use crate::vectors::vector_2d::Vector2D;
use crate::window::{Window, WindowFlags};

// Helper to snap on edges when aiming at an item very close to the edge,
// So the difference between window_padding and item_spacing will be in the visible area after scrolling.
// When we refactor the scrolling API this may be configurable with a flag?
// Note that the effect for this won't be visible on x axis with default style settings as window_padding.x == item_spacing.x by default.
// static float calc_scroll_edge_snap(float target, float snap_min, float snap_max, float snap_threshold, float center_ratio)
pub fn calc_scroll_edge_snap(g: &mut Context, target: f32, snap_min: f32, snap_max: f32, snap_threshold: f32, center_ratio: f32) -> f32
{
    if target <= snap_min + snap_threshold {
        return lerp_f32(snap_min, target, center_ratio);
    }
    if target >= snap_max - snap_threshold {
        return lerp_f32(target, snap_max, center_ratio);
    }
    return target;
}

// static Vector2D calc_next_scroll_from_scroll_target_and_clamp(Window* window)
pub fn calc_next_scroll_from_scroll_target_and_clamp(g: &mut Context, window: &mut Window) -> Vector2D
{
    let scroll = &mut window.scroll;
    if window.scroll_target.x < f32::MAX
    {
        let decoration_total_width =  window.scrollbar_sizes.x;
        let center_x_ratio =  window.scroll_target_center_ratio.x;
        let mut scroll_target_x =  window.scroll_target.x;
        if window.scroll_target_edge_snap_dist.x > 0.0
        {
            let snap_x_min =  0.0;
            let snap_x_max =  window.scroll_max.x + window.size_full.x - decoration_total_width;
            scroll_target_x = calc_scroll_edge_snap(g, scroll_target_x, snap_x_min, snap_x_max, window.scroll_target_edge_snap_dist.x, center_x_ratio);
        }
        scroll.x = scroll_target_x - center_x_ratio * (window.size_full.x - decoration_total_width);
    }
    if window.scroll_target.y < f32::MAX
    {
        let decoration_total_height =  window.title_bar_height() + window.MenuBarHeight() + window.scrollbar_sizes.y;
        let center_y_ratio =  window.scroll_target_center_ratio.y;
        let mut scroll_target_y =  window.scroll_target.y;
        if window.scroll_target_edge_snap_dist.y > 0.0
        {
            let snap_y_min =  0.0;
            let snap_y_max =  window.scroll_max.y + window.size_full.y - decoration_total_height;
            scroll_target_y = calc_scroll_edge_snap(g, scroll_target_y, snap_y_min, snap_y_max, window.scroll_target_edge_snap_dist.y, center_y_ratio);
        }
        scroll.y = scroll_target_y - center_y_ratio * (window.size_full.y - decoration_total_height);
    }
    scroll.x = f32::floor(ImMax(scroll.x, 0.0));
    scroll.y = f32::floor(ImMax(scroll.y, 0.0));
    if !window.collapsed && !window.skip_items
    {
        scroll.x = ImMin(scroll.x, window.scroll_max.x);
        scroll.y = ImMin(scroll.y, window.scroll_max.y);
    }
    return scroll.clone();
}

// void ScrollToItem(ImGuiScrollFlags flags)
pub fn scroll_to_item(g: &mut Context, flags: &HashSet<ScrollFlags>)
{
    // ImGuiContext& g = *GImGui;
    let window = g.current_window_mut();
    scroll_to_rect_ex(g, window, &g.last_item_data.nav_rect, flags);
}

// void ScrollToRect(Window* window, const Rect& item_rect, ImGuiScrollFlags flags)
pub fn scroll_to_rect(g: &mut Context, window: &mut Window, item_rect: &Rect, flags: &HashSet<ScrollFlags>)
{

    scroll_to_rect_ex(g, window, item_rect, flags);
}

// scroll to keep newly navigated item fully into view
// Vector2D scroll_to_rect_ex(Window* window, const Rect& item_rect, ImGuiScrollFlags flags)
pub fn scroll_to_rect_ex(g: &mut Context, window: &mut Window, item_rect: &Rect, flags: &HashSet<ScrollFlags>) -> Vector2D {
    // ImGuiContext& g = *GImGui;
    let mut window_rect = Rect::new(&(window.inner_rect.min - Vector2D::new(1f32, 1f32)), &(window.inner_rect.max + Vector2D::new(1f32, 1f32)));
    //GetForegroundDrawList(window)->add_rect(window_rect.min, window_rect.max, IM_COL32_WHITE); // [DEBUG]

    // Check that only one behavior is selected per axis
    // IM_ASSERT((flags & ScrollFlags::MaskX_) == 0 || ImIsPowerOfTwo(flags & ScrollFlags::MaskX_));
    // IM_ASSERT((flags & ScrollFlags::MaskY_) == 0 || ImIsPowerOfTwo(flags & ScrollFlags::MaskY_));

    // Defaults
    // ImGuiScrollFlags in_flags = flags;
    let mut in_flags: HashSet<ScrollFlags> = HashSet::new();
    HashSet::clone_from(&mut in_flags, flags);

    // if ((flags & ScrollFlags::MaskX_) == 0 && window.scrollbar_x)
    //     flags |= ScrollFlags::KeepVisibleEdgeX;
    if !scroll_flags_contains_mask_x(&in_flags) {
        in_flags.insert(ScrollFlags::KeepVisibleEdgeX);
    }
    // if ((flags & ScrollFlags::MaskY_) == 0)
    //     flags |= window.Appearing ? ScrollFlags::AlwaysCenterY : ScrollFlags::KeepVisibleEdgeY;
    if !scroll_flags_contains_mask_y(&in_flags) {
        if window.appearing {
            in_flags.insert(ScrollFlags::AlwaysCenterY)
        } else {
            in_flags.insert(ScrollFlags::KeepVisibleEdgeY);
        }
    }

    let fully_visible_x = item_rect.min.x >= window_rect.min.x && item_rect.max.x <= window_rect.max.x;
    let fully_visible_y = item_rect.min.y >= window_rect.min.y && item_rect.max.y <= window_rect.max.y;
    let can_be_fully_visible_x = (item_rect.width() + g.style.item_spacing.x * 2.0) <= window_rect.get_width();
    let can_be_fully_visible_y = (item_rect.height() + g.style.item_spacing.y * 2.0) <= window_rect.get_height();

    // if ((flags & ScrollFlags::KeepVisibleEdgeX) && !fully_visible_x)
    if in_flags.contains(&ScrollFlags::KeepVisibleEdgeX) {
        if item_rect.min.x < window_rect.min.x || !can_be_fully_visible_x {
            set_scroll_from_pos_x(g, window, item_rect.min.x - g.style.item_spacing.x - window.pos.x, 0.0);
        } else if item_rect.max.x >= window_rect.max.x {
            set_scroll_from_pos_x(g, window, item_rect.max.x + g.style.item_spacing.x - window.pos.x, 1.0);
        }
    }
    // else if (((flags & ScrollFlags::KeepVisibleCenterX) && !fully_visible_x) || (flags & ScrollFlags::AlwaysCenterX))
    if (in_flags.contains(&ScrollFlags::KeepVisibleCenterX) && !fully_visible_x) || in_flags.contains(&ScrollFlags::AlwaysCenterX) {
        let target_x = if can_be_fully_visible_x { f32::floor((item_rect.min.x + item_rect.max.x - window.inner_rect.width()) * 0.5) } else { item_rect.min.x };
        set_scroll_from_pos_x(g, window, target_x - window.pos.x, 0.0);
    }

    // if ((flags & ScrollFlags::KeepVisibleEdgeY) && !fully_visible_y)
    if in_flags.contains(&ScrollFlags::KeepVisibleEdgeY) && !fully_visible_y {
        if item_rect.min.y < window_rect.min.y || !can_be_fully_visible_y {
            set_scroll_from_pos_y(g, window, item_rect.min.y - g.style.item_spacing.y - window.pos.y, 0.0);
        } else if item_rect.max.y >= window_rect.max.y {
            set_scroll_from_pos_y(g, window, item_rect.max.y + g.style.item_spacing.y - window.pos.y, 1.0);
        }
    }
    // else if (((flags & ScrollFlags::KeepVisibleCenterY) && !fully_visible_y) || (flags & ScrollFlags::AlwaysCenterY))
    else if (in_flags.contains(&ScrollFlags::KeepVisibleCenterY) && !fully_visible_y) || in_flags.contains(&ScrollFlags::AlwaysCenterY) {
        let target_y = if can_be_fully_visible_y { f32::floor((item_rect.min.y + item_rect.max.y - window.inner_rect.height()) * 0.5) } else { item_rect.min.y };
        set_scroll_from_pos_y(g, window, target_y - window.pos.y, 0.0);
    }

    let next_scroll = calc_next_scroll_from_scroll_target_and_clamp(g, window);
    let mut delta_scroll = next_scroll - window.scroll;

    // Also scroll parent window to keep us into view if necessary
    // if (!(flags & ScrollFlags::NoScrollParent) && (window.flags & WindowFlags::ChildWindow))
    if !in_flags.contains(&ScrollFlags::NoScrollParent) && window.flags.contains(&WindowFlags::ChildWindow) {
        // FIXME-SCROLL: May be an option?
        // if (in_flags & (ScrollFlags::AlwaysCenterX | ScrollFlags::KeepVisibleCenterX)) != 0
        if in_flags.contains(&ScrollFlags::AlwaysCenterX) && in_flags.contains(&ScrollFlags::KeepVisibleCenterX) {
            // in_flags = (in_flags & ~ScrollFlags::MaskX_) | ScrollFlags::KeepVisibleEdgeX;
            scroll_flags_remove_mask_x(&mut in_flags);
            scroll_flags.insert(ScrollFlags::KeepVisibleEdgeX);
        }
        // if (in_flags & (ScrollFlags::AlwaysCenterY | ScrollFlags::KeepVisibleCenterY)) != 0
        if in_flags.contains(&ScrollFlags::AlwaysCenterY) && in_flags.contains(&ScrollFlags::KeepVisibleCenterY) {
            scroll_flags_remove_mask_y(&mut in_flags);
            // in_flags = (in_flags & ~ScrollFlags::MaskY_) | ScrollFlags::KeepVisibleEdgeY;
            in_flags.remove(&ScrollFlags::KeepVisibleEdgeY);
        }
        delta_scroll += scroll_to_rect_ex(g, window.parent_window, &Rect::new(&(item_rect.min - delta_scroll), &(item_rect.max - delta_scroll)), &in_flags);
    }

    return delta_scroll;
}

// float GetScrollX()
pub fn get_scroll_x(g: &mut Context) -> f32
{
    let window = g.current_window_mut();;
    return window.scroll.x;
}

// float GetScrollY()
pub fn get_scroll_y(g: &mut Context) -> f32
{
    let window = g.current_window_mut();;
    return window.scroll.y;
}

// float GetScrollMaxX()
pub fn get_scroll_max_x(g: &mut Context) -> f32
{
    let window = g.current_window_mut();;
    return window.scroll_max.x;
}

// float GetScrollMaxY()
pub fn get_scroll_max_y(g: &mut Context) -> f32
{
    let window = g.current_window_mut();;
    return window.scroll_max.y;
}

// void set_scroll_x(Window* window, float scroll_x)
pub fn set_scroll_x(g: &mut Context, window: &mut Window, scroll_x: f32)
{
    window.scroll_target.x = scroll_x;
    window.scroll_target_center_ratio.x = 0.0;
    window.scroll_target_edge_snap_dist.x = 0.0;
}

// void set_scroll_y(Window* window, float scroll_y)
pub fn set_scroll_y(g: &mut Context, window: &mut Window, scroll_y: f32)
{
    window.scroll_target.y = scroll_y;
    window.scroll_target_center_ratio.y = 0.0;
    window.scroll_target_edge_snap_dist.y = 0.0;
}

// void set_scroll_x(float scroll_x)
pub fn set_scroll_x2(g: &mut Context, scroll_x: f32)
{
    // ImGuiContext& g = *GImGui;
    set_scroll_x(g, g.current_window, scroll_x);
}

// void set_scroll_y(float scroll_y)
pub fn set_scroll_y2(g: &mut Context, scroll_y:f32)
{
    // ImGuiContext& g = *GImGui;
    set_scroll_y(g, g.current_window, scroll_y);
}

// Note that a local position will vary depending on initial scroll value,
// This is a little bit confusing so bear with us:
//  - local_pos = (absolution_pos - window->pos)
//  - So local_x/local_y are 0.0 for a position at the upper-left corner of a window,
//    and generally local_x/local_y are >(padding+decoration) && <(size-padding-decoration) when in the visible area.
//  - They mostly exists because of legacy API.
// Following the rules above, when trying to work with scrolling code, consider that:
//  - set_scroll_from_pos_y(0.0) == SetScrollY(0.0 + scroll.y) == has no effect!
//  - set_scroll_from_pos_y(-scroll.y) == SetScrollY(-scroll.y + scroll.y) == SetScrollY(0.0) == reset scroll. Of course writing SetScrollY(0.0) directly then makes more sense
// We store a target position so centering and clamping can occur on the next frame when we are guaranteed to have a known window size
// void set_scroll_from_pos_x(Window* window, float local_x, float center_x_ratio)
pub fn set_scroll_from_pos_x(g: &mut Context, window: &mut Window, local_x: f32, center_x_ratio: f32)
{
    // IM_ASSERT(center_x_ratio >= 0.0 && center_x_ratio <= 1.0);
    window.scroll_target.x = f32::floor(local_x + window.scroll.x); // Convert local position to scroll offset
    window.scroll_target_center_ratio.x = center_x_ratio;
    window.scroll_target_edge_snap_dist.x = 0.0;
}

// void set_scroll_from_pos_y(Window* window, float local_y, float center_y_ratio)
pub fn set_scroll_from_pos_y(g: &mut Context, window: &mut Window, mut local_y: f32, center_y_ratio: f32)
{
    // IM_ASSERT(center_y_ratio >= 0.0 && center_y_ratio <= 1.0);
    let decoration_up_height = window.title_bar_height() + window.MenuBarHeight(); // FIXME: Would be nice to have a more standardized access to our scrollable/client rect;
    local_y -= decoration_up_height;
    window.scroll_target.y = f32::floor(local_y + window.scroll.y); // Convert local position to scroll offset
    window.scroll_target_center_ratio.y = center_y_ratio;
    window.scroll_target_edge_snap_dist.y = 0.0;
}

fn set_scroll_from_pos_x2(g: &mut Context, local_x: f32, center_x_ratio: f32)
{
    // ImGuiContext& g = *GImGui;
    set_scroll_from_pos_x(g, g.current_window, local_x, center_x_ratio);
}

// void set_scroll_from_pos_y(float local_y, float center_y_ratio)
pub fn set_scroll_from_pos_y2(g: &mut Context, local_y: f32, center_y_ratio: f32)
{
    // ImGuiContext& g = *GImGui;
    set_scroll_from_pos_y(g, g.current_window, local_y, center_y_ratio);
}

// center_x_ratio: 0.0 left of last item, 0.5 horizontal center of last item, 1.0 right of last item.
// void SetScrollHereX(float center_x_ratio)
pub fn set_scroll_here_x(g: &mut Context, center_x_ratio: f32)
{
    // ImGuiContext& g = *GImGui;
    let window = g.current_window_mut();
    let spacing_x =  f32::max(window.window_padding.x, g.style.item_spacing.x);
    let target_pos_x =  lerp_f32(g.last_item_data.rect.min.x - spacing_x, g.last_item_data.rect.max.x + spacing_x, center_x_ratio);
    set_scroll_from_pos_x(g, window, target_pos_x - window.pos.x, center_x_ratio); // Convert from absolute to local pos

    // Tweak: snap on edges when aiming at an item very close to the edge
    window.scroll_target_edge_snap_dist.x = ImMax(0.0, window.window_padding.x - spacing_x);
}

// center_y_ratio: 0.0 top of last item, 0.5 vertical center of last item, 1.0 bottom of last item.
// void SetScrollHereY(float center_y_ratio)
pub fn set_scroll_here_y(g: &mut Context, center_y_ratio: f32)
{
    // ImGuiContext& g = *GImGui;
    let window = g.current_window_mut();
    let spacing_y =  ImMax(window.window_padding.y, g.style.item_spacing.y);
    let target_pos_y =  ImLerp(window.dc.cursor_pos_prev_line.y - spacing_y, window.dc.cursor_pos_prev_line.y + window.dc.prev_line_size.y + spacing_y, center_y_ratio);
    set_scroll_from_pos_y(g, window, target_pos_y - window.pos.y, center_y_ratio); // Convert from absolute to local pos

    // Tweak: snap on edges when aiming at an item very close to the edge
    window.scroll_target_edge_snap_dist.y = ImMax(0.0, window.window_padding.y - spacing_y);
}
