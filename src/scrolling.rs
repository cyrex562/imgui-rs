use std::collections::HashSet;
use crate::Context;
use crate::nav::ScrollFlags;
use crate::rect::Rect;
use crate::vectors::vector_2d::Vector2D;
use crate::window::{Window, WindowFlags};

// Helper to snap on edges when aiming at an item very close to the edge,
// So the difference between window_padding and ItemSpacing will be in the visible area after scrolling.
// When we refactor the scrolling API this may be configurable with a flag?
// Note that the effect for this won't be visible on x axis with default style settings as window_padding.x == ItemSpacing.x by default.
// static float CalcScrollEdgeSnap(float target, float snap_min, float snap_max, float snap_threshold, float center_ratio)
pub fn calc_scroll_edge_snap(g: &mut Context, target: f32, snap_min: f32, snap_max: f32, snap_threshold: f32, center_ratio: f32) -> f32
{
    if (target <= snap_min + snap_threshold)
        return ImLerp(snap_min, target, center_ratio);
    if (target >= snap_max - snap_threshold)
        return ImLerp(target, snap_max, center_ratio);
    return target;
}

// static Vector2D CalcNextScrollFromScrollTargetAndClamp(Window* window)
pub fn calc_next_scroll_from_scroll_target_and_clamp(g: &mut Context, window: &mut Window) -> Vector2D
{
    Vector2D scroll = window.scroll;
    if (window.ScrollTarget.x < f32::MAX)
    {
        let decoration_total_width =  window.scrollbar_sizes.x;
        let center_x_ratio =  window.ScrollTargetCenterRatio.x;
        let scroll_target_x =  window.ScrollTarget.x;
        if (window.ScrollTargetEdgeSnapDist.x > 0.0)
        {
            let snap_x_min =  0.0;
            let snap_x_max =  window.scroll_max.x + window.size_full.x - decoration_total_width;
            scroll_target_x = CalcScrollEdgeSnap(scroll_target_x, snap_x_min, snap_x_max, window.ScrollTargetEdgeSnapDist.x, center_x_ratio);
        }
        scroll.x = scroll_target_x - center_x_ratio * (window.size_full.x - decoration_total_width);
    }
    if (window.ScrollTarget.y < f32::MAX)
    {
        let decoration_total_height =  window.title_bar_height() + window.MenuBarHeight() + window.scrollbar_sizes.y;
        let center_y_ratio =  window.ScrollTargetCenterRatio.y;
        let scroll_target_y =  window.ScrollTarget.y;
        if (window.ScrollTargetEdgeSnapDist.y > 0.0)
        {
            let snap_y_min =  0.0;
            let snap_y_max =  window.scroll_max.y + window.size_full.y - decoration_total_height;
            scroll_target_y = CalcScrollEdgeSnap(scroll_target_y, snap_y_min, snap_y_max, window.ScrollTargetEdgeSnapDist.y, center_y_ratio);
        }
        scroll.y = scroll_target_y - center_y_ratio * (window.size_full.y - decoration_total_height);
    }
    scroll.x = f32::floor(ImMax(scroll.x, 0.0));
    scroll.y = f32::floor(ImMax(scroll.y, 0.0));
    if (!window.collapsed && !window.skip_items)
    {
        scroll.x = ImMin(scroll.x, window.scroll_max.x);
        scroll.y = ImMin(scroll.y, window.scroll_max.y);
    }
    return scroll;
}

// void ScrollToItem(ImGuiScrollFlags flags)
pub fn scroll_to_item(g: &mut Context, flags: &HashSet<ScrollFlags>)
{
    // ImGuiContext& g = *GImGui;
    Window* window = g.current_window;
    ScrollToRectEx(window, g.last_item_data.nav_rect, flags);
}

// void ScrollToRect(Window* window, const Rect& item_rect, ImGuiScrollFlags flags)
pub fn scroll_to_rect(g: &mut Context, window: &mut Window, item_rect: &Rect, flags: &HashSet<ScrollFlags>)
{
    ScrollToRectEx(window, item_rect, flags);
}

// scroll to keep newly navigated item fully into view
// Vector2D ScrollToRectEx(Window* window, const Rect& item_rect, ImGuiScrollFlags flags)
pub fn scroll_to_rect_ex(g: &mut Context, window: &mut Window, item_rect: &Rect, flags: &HashSet<ScrollFlags>)
{
    // ImGuiContext& g = *GImGui;
    Rect window_rect(window.inner_rect.min - Vector2D::new(1, 1), window.inner_rect.max + Vector2D::new(1, 1));
    //GetForegroundDrawList(window)->add_rect(window_rect.min, window_rect.max, IM_COL32_WHITE); // [DEBUG]

    // Check that only one behavior is selected per axis
    // IM_ASSERT((flags & ImGuiScrollFlags_MaskX_) == 0 || ImIsPowerOfTwo(flags & ImGuiScrollFlags_MaskX_));
    // IM_ASSERT((flags & ImGuiScrollFlags_MaskY_) == 0 || ImIsPowerOfTwo(flags & ImGuiScrollFlags_MaskY_));

    // Defaults
    ImGuiScrollFlags in_flags = flags;
    if ((flags & ImGuiScrollFlags_MaskX_) == 0 && window.scrollbar_x)
        flags |= ImGuiScrollFlags_KeepVisibleEdgeX;
    if ((flags & ImGuiScrollFlags_MaskY_) == 0)
        flags |= window.Appearing ? ImGuiScrollFlags_AlwaysCenterY : ImGuiScrollFlags_KeepVisibleEdgeY;

    const bool fully_visible_x = item_rect.min.x >= window_rect.min.x && item_rect.max.x <= window_rect.max.x;
    const bool fully_visible_y = item_rect.min.y >= window_rect.min.y && item_rect.max.y <= window_rect.max.y;
    const bool can_be_fully_visible_x = (item_rect.get_width() + g.style.item_spacing.x * 2.0) <= window_rect.get_width();
    const bool can_be_fully_visible_y = (item_rect.get_height() + g.style.item_spacing.y * 2.0) <= window_rect.get_height();

    if ((flags & ImGuiScrollFlags_KeepVisibleEdgeX) && !fully_visible_x)
    {
        if (item_rect.min.x < window_rect.min.x || !can_be_fully_visible_x)
            SetScrollFromPosX(window, item_rect.min.x - g.style.item_spacing.x - window.pos.x, 0.0);
        else if (item_rect.max.x >= window_rect.max.x)
            SetScrollFromPosX(window, item_rect.max.x + g.style.item_spacing.x - window.pos.x, 1.0);
    }
    else if (((flags & ImGuiScrollFlags_KeepVisibleCenterX) && !fully_visible_x) || (flags & ImGuiScrollFlags_AlwaysCenterX))
    {
        let target_x =  can_be_fully_visible_x ? f32::floor((item_rect.min.x + item_rect.max.x - window.inner_rect.get_width()) * 0.5) : item_rect.min.x;
        SetScrollFromPosX(window, target_x - window.pos.x, 0.0);
    }

    if ((flags & ImGuiScrollFlags_KeepVisibleEdgeY) && !fully_visible_y)
    {
        if (item_rect.min.y < window_rect.min.y || !can_be_fully_visible_y)
            SetScrollFromPosY(window, item_rect.min.y - g.style.item_spacing.y - window.pos.y, 0.0);
        else if (item_rect.max.y >= window_rect.max.y)
            SetScrollFromPosY(window, item_rect.max.y + g.style.item_spacing.y - window.pos.y, 1.0);
    }
    else if (((flags & ImGuiScrollFlags_KeepVisibleCenterY) && !fully_visible_y) || (flags & ImGuiScrollFlags_AlwaysCenterY))
    {
        let target_y =  can_be_fully_visible_y ? f32::floor((item_rect.min.y + item_rect.max.y - window.inner_rect.get_height()) * 0.5) : item_rect.min.y;
        SetScrollFromPosY(window, target_y - window.pos.y, 0.0);
    }

    Vector2D next_scroll = CalcNextScrollFromScrollTargetAndClamp(window);
    Vector2D delta_scroll = next_scroll - window.scroll;

    // Also scroll parent window to keep us into view if necessary
    if (!(flags & ImGuiScrollFlags_NoScrollParent) && (window.flags & WindowFlags::ChildWindow))
    {
        // FIXME-SCROLL: May be an option?
        if ((in_flags & (ImGuiScrollFlags_AlwaysCenterX | ImGuiScrollFlags_KeepVisibleCenterX)) != 0)
            in_flags = (in_flags & ~ImGuiScrollFlags_MaskX_) | ImGuiScrollFlags_KeepVisibleEdgeX;
        if ((in_flags & (ImGuiScrollFlags_AlwaysCenterY | ImGuiScrollFlags_KeepVisibleCenterY)) != 0)
            in_flags = (in_flags & ~ImGuiScrollFlags_MaskY_) | ImGuiScrollFlags_KeepVisibleEdgeY;
        delta_scroll += ScrollToRectEx(window.parent_window, Rect(item_rect.min - delta_scroll, item_rect.max - delta_scroll), in_flags);
    }

    return delta_scroll;
}

// float GetScrollX()
pub fn get_Scroll_x(g: &mut Context) -> f32
{
    Window* window = g.current_window_id;
    return window.scroll.x;
}

// float GetScrollY()
pub fn get_scroll_y(g: &mut Context) -> f32
{
    Window* window = g.current_window_id;
    return window.scroll.y;
}

// float GetScrollMaxX()
pub fn get_scroll_max_x(g: &mut Context) -> f32
{
    Window* window = g.current_window_id;
    return window.scroll_max.x;
}

// float GetScrollMaxY()
pub fn get_scroll_max_y(g: &mut Context) -> f32
{
    Window* window = g.current_window_id;
    return window.scroll_max.y;
}

// void set_scroll_x(Window* window, float scroll_x)
pub fn set_scroll_x(g: &mut Context, window: &mut Window, scroll_x: f32)
{
    window.ScrollTarget.x = scroll_x;
    window.ScrollTargetCenterRatio.x = 0.0;
    window.ScrollTargetEdgeSnapDist.x = 0.0;
}

// void set_scroll_y(Window* window, float scroll_y)
pub fn set_scroll_y(g: &mut Context, window: &mut Window, scroll_y: f32)
{
    window.ScrollTarget.y = scroll_y;
    window.ScrollTargetCenterRatio.y = 0.0;
    window.ScrollTargetEdgeSnapDist.y = 0.0;
}

// void set_scroll_x(float scroll_x)
pub fn set_scroll_x2(g: &mut Context, scroll_x: f32)
{
    // ImGuiContext& g = *GImGui;
    set_scroll_x(g.current_window, scroll_x);
}

// void set_scroll_y(float scroll_y)
pub fn set_scroll_y2(g: &mut Context, scroll_y:f32)
{
    // ImGuiContext& g = *GImGui;
    set_scroll_y(g.current_window, scroll_y);
}

// Note that a local position will vary depending on initial scroll value,
// This is a little bit confusing so bear with us:
//  - local_pos = (absolution_pos - window->pos)
//  - So local_x/local_y are 0.0 for a position at the upper-left corner of a window,
//    and generally local_x/local_y are >(padding+decoration) && <(size-padding-decoration) when in the visible area.
//  - They mostly exists because of legacy API.
// Following the rules above, when trying to work with scrolling code, consider that:
//  - SetScrollFromPosY(0.0) == SetScrollY(0.0 + scroll.y) == has no effect!
//  - SetScrollFromPosY(-scroll.y) == SetScrollY(-scroll.y + scroll.y) == SetScrollY(0.0) == reset scroll. Of course writing SetScrollY(0.0) directly then makes more sense
// We store a target position so centering and clamping can occur on the next frame when we are guaranteed to have a known window size
// void SetScrollFromPosX(Window* window, float local_x, float center_x_ratio)
pub fn set_scroll_from_pos_x(g: &mut Context, window: &mut Window, local_x: f32, center_x_ratio: f32)
{
    // IM_ASSERT(center_x_ratio >= 0.0 && center_x_ratio <= 1.0);
    window.ScrollTarget.x = f32::floor(local_x + window.scroll.x); // Convert local position to scroll offset
    window.ScrollTargetCenterRatio.x = center_x_ratio;
    window.ScrollTargetEdgeSnapDist.x = 0.0;
}

// void SetScrollFromPosY(Window* window, float local_y, float center_y_ratio)
pub fn set_scroll_from_pos_y(g: &mut Context, window: &mut Window, local_y: f32, center_y_ratio: f32)
{
    // IM_ASSERT(center_y_ratio >= 0.0 && center_y_ratio <= 1.0);
    let decoration_up_height = window.title_bar_height() + window.MenuBarHeight(); // FIXME: Would be nice to have a more standardized access to our scrollable/client rect;
    local_y -= decoration_up_height;
    window.ScrollTarget.y = f32::floor(local_y + window.scroll.y); // Convert local position to scroll offset
    window.ScrollTargetCenterRatio.y = center_y_ratio;
    window.ScrollTargetEdgeSnapDist.y = 0.0;
}

fn set_scroll_from_pos_x2(g: &mut Context, local_x: f32, center_x_ratio: f32)
{
    // ImGuiContext& g = *GImGui;
    SetScrollFromPosX(g.current_window, local_x, center_x_ratio);
}

// void SetScrollFromPosY(float local_y, float center_y_ratio)
pub fn set_scroll_from_pos_y2(g: &mut Context, local_y: f32, center_y_ratio: f32)
{
    // ImGuiContext& g = *GImGui;
    SetScrollFromPosY(g.current_window, local_y, center_y_ratio);
}

// center_x_ratio: 0.0 left of last item, 0.5 horizontal center of last item, 1.0 right of last item.
// void SetScrollHereX(float center_x_ratio)
pub fn set_scroll_here_x(g: &mut Context, center_x_ratio: f32)
{
    // ImGuiContext& g = *GImGui;
    Window* window = g.current_window;
    let spacing_x =  ImMax(window.WindowPadding.x, g.style.item_spacing.x);
    let target_pos_x =  ImLerp(g.last_item_data.Rect.min.x - spacing_x, g.last_item_data.Rect.max.x + spacing_x, center_x_ratio);
    SetScrollFromPosX(window, target_pos_x - window.pos.x, center_x_ratio); // Convert from absolute to local pos

    // Tweak: snap on edges when aiming at an item very close to the edge
    window.ScrollTargetEdgeSnapDist.x = ImMax(0.0, window.WindowPadding.x - spacing_x);
}

// center_y_ratio: 0.0 top of last item, 0.5 vertical center of last item, 1.0 bottom of last item.
// void SetScrollHereY(float center_y_ratio)
pub fn set_scroll_here_y(g: &mut Context, center_y_ratio: f32)
{
    // ImGuiContext& g = *GImGui;
    Window* window = g.current_window;
    let spacing_y =  ImMax(window.WindowPadding.y, g.style.item_spacing.y);
    let target_pos_y =  ImLerp(window.dc.cursor_pos_prev_line.y - spacing_y, window.dc.cursor_pos_prev_line.y + window.dc.prev_line_size.y + spacing_y, center_y_ratio);
    SetScrollFromPosY(window, target_pos_y - window.pos.y, center_y_ratio); // Convert from absolute to local pos

    // Tweak: snap on edges when aiming at an item very close to the edge
    window.ScrollTargetEdgeSnapDist.y = ImMax(0.0, window.WindowPadding.y - spacing_y);
}
