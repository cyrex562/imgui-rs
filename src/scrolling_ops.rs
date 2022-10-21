
// Helper to snap on edges when aiming at an item very close to the edge,
// So the difference between WindowPadding and ItemSpacing will be in the visible area after scrolling.
// When we refactor the scrolling API this may be configurable with a flag?
// Note that the effect for this won't be visible on X axis with default Style settings as WindowPadding.x == ItemSpacing.x by default.
pub unsafe fn staticCalcScrollEdgeSnap(target: c_float,snap_min: c_float,snap_max: c_float,snap_threshold: c_float,center_ratio: c_float) -> c_float
{
    if target <= snap_min + snap_threshold {
        return ImLerp(snap_min, target, center_ratio);
    }
    if target >= snap_max - snap_threshold {
        return ImLerp(target, snap_max, center_ratio);
    }
    return target;
}

use libc::c_float;
use crate::GImGui;
use crate::math_ops::{ImLerp, ImMax, ImMin};
use crate::rect::ImRect;
use crate::scroll_flags::{ImGuiScrollFlags, ImGuiScrollFlags_AlwaysCenterX, ImGuiScrollFlags_AlwaysCenterY, ImGuiScrollFlags_KeepVisibleCenterX, ImGuiScrollFlags_KeepVisibleCenterY, ImGuiScrollFlags_KeepVisibleEdgeX, ImGuiScrollFlags_KeepVisibleEdgeY, ImGuiScrollFlags_MaskX_, ImGuiScrollFlags_MaskY_, ImGuiScrollFlags_NoScrollParent};
use crate::utils::{flag_clear, flag_set};
use crate::vec2::ImVec2;
use crate::window::ImGuiWindow;
use crate::window::window_flags::ImGuiWindowFlags_ChildWindow;

// static CalcNextScrollFromScrollTargetAndClamp: ImVec2(window: *mut ImGuiWindow)
pub unsafe fn CalcNextScrollFromScrollTargetAndClamp(window: *mut ImGuiWindow) -> ImVec2
{
    let mut scroll: ImVec2 = window.Scroll;
    if window.ScrollTarget.x < f32::MAX
    {
        let decoration_total_width: c_float =  window.ScrollbarSizes.x;
        let center_x_ratio: c_float =  window.ScrollTargetCenterRatio.x;
        let mut scroll_target_x: c_float =  window.ScrollTarget.x;
        if window.ScrollTargetEdgeSnapDist.x > 0.0
        {
            let snap_x_min: c_float =  0.0;
            let snap_x_max: c_float =  window.ScrollMax.x + window.SizeFull.x - decoration_total_width;
            scroll_target_x = CalcScrollEdgeSnap(scroll_target_x, snap_x_min, snap_x_max, window.ScrollTargetEdgeSnapDist.x, center_x_ratio);
        }
        scroll.x = scroll_target_x - center_x_ratio * (window.SizeFull.x - decoration_total_width);
    }
    if window.ScrollTarget.y < f32::MAX
    {
        let decoration_total_height: c_float =  window.TitleBarHeight() + window.MenuBarHeight() + window.ScrollbarSizes.y;
        let center_y_ratio: c_float =  window.ScrollTargetCenterRatio.y;
        let mut scroll_target_y: c_float =  window.ScrollTarget.y;
        if window.ScrollTargetEdgeSnapDist.y > 0.0
        {
            let snap_y_min: c_float =  0.0;
            let snap_y_max: c_float =  window.ScrollMax.y + window.SizeFull.y - decoration_total_height;
            scroll_target_y = CalcScrollEdgeSnap(scroll_target_y, snap_y_min, snap_y_max, window.ScrollTargetEdgeSnapDist.y, center_y_ratio);
        }
        scroll.y = scroll_target_y - center_y_ratio * (window.SizeFull.y - decoration_total_height);
    }
    scroll.x = IM_FLOOR(ImMax(scroll.x, 0.0));
    scroll.y = IM_FLOOR(ImMax(scroll.y, 0.0));
    if !window.Collapsed && !window.SkipItems
    {
        scroll.x = ImMin(scroll.x, window.ScrollMax.x);
        scroll.y = ImMin(scroll.y, window.ScrollMax.y);
    }
    return scroll;
}

pub unsafe fn ScrollToItem(flags: ImGuiScrollFlags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    ScrollToRectEx(window, &mut g.LastItemData.NavRect, flags);
}

pub unsafe fn ScrollToRect(window: *mut ImGuiWindow, item_rect: &mut ImRect, flags: ImGuiScrollFlags)
{
    ScrollToRectEx(window, item_rect, flags);
}

// Scroll to keep newly navigated item fully into view
pub unsafe fn ScrollToRectEx(window: *mut ImGuiWindow, item_rect: &mut ImRect, mut flags: ImGuiScrollFlags) -> ImVec2
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window_rect: ImRect = ImRect::new(window.InnerRect.Min - ImVec2::from_floats(1.0, 1.0), window.InnerRect.Max + ImVec2::from_floats(1.0, 1.0));
    //GetForegroundDrawList(window).AddRect(window_rect.Min, window_rect.Max, IM_COL32_WHITE); // [DEBUG]

    // Check that only one behavior is selected per axis
    // IM_ASSERT(flag_set(flags, ImGuiScrollFlags_MaskX_) == 0 || ImIsPowerOfTwo(flags & ImGuiScrollFlags_MaskX_));
    // IM_ASSERT(flag_set(flags, ImGuiScrollFlags_MaskY_) == 0 || ImIsPowerOfTwo(flags & ImGuiScrollFlags_MaskY_));

    // Defaults
    let mut in_flags = flags;
    if (flag_clear(flags, ImGuiScrollFlags_MaskX_) && window.ScrollbarX) {
        flags |= ImGuiScrollFlags_KeepVisibleEdgeX;
    }
    if (flag_clear(flags, ImGuiScrollFlags_MaskY_)) {
        flags |= if window.Appearing {
            ImGuiScrollFlags_AlwaysCenterY
        } else { ImGuiScrollFlags_KeepVisibleEdgeY };
    }

    let fully_visible_x: bool = item_rect.Min.x >= window_rect.Min.x && item_rect.Max.x <= window_rect.Max.x;
    let fully_visible_y: bool = item_rect.Min.y >= window_rect.Min.y && item_rect.Max.y <= window_rect.Max.y;
    let can_be_fully_visible_x: bool = (item_rect.GetWidth() + g.Style.ItemSpacing.x * 2.0) <= window_rect.GetWidth();
    let can_be_fully_visible_y: bool = (item_rect.GetHeight() + g.Style.ItemSpacing.y * 2.0) <= window_rect.GetHeight();

    if flag_set(flags, ImGuiScrollFlags_KeepVisibleEdgeX) && !fully_visible_x
    {
        if item_rect.Min.x < window_rect.Min.x || !can_be_fully_visible_x {
            SetScrollFromPosX(window, item_rect.Min.x - g.Style.ItemSpacing.x - window.Pos.x, 0.0);
        }
        else if item_rect.Max.x >= window_rect.Max.x {
            SetScrollFromPosX(window, item_rect.Max.x + g.Style.ItemSpacing.x - window.Pos.x, 1.0);
        }
    }
    else if (flag_set(flags, ImGuiScrollFlags_KeepVisibleCenterX) && !fully_visible_x) || flag_set(flags, ImGuiScrollFlags_AlwaysCenterX)
    {
        let target_x: c_float =  if can_be_fully_visible_x { ImFloor((item_rect.Min.x + item_rect.Max.x - window.InnerRect.GetWidth()) * 0.5) } else { item_rect.Min.x };
        SetScrollFromPosX(window, target_x - window.Pos.x, 0.0);
    }

    if (flag_set(flags, ImGuiScrollFlags_KeepVisibleEdgeY) && !fully_visible_y)
    {
        if (item_rect.Min.y < window_rect.Min.y || !can_be_fully_visible_y) {
            SetScrollFromPosY(window, item_rect.Min.y - g.Style.ItemSpacing.y - window.Pos.y, 0.0);
        }
        else if (item_rect.Max.y >= window_rect.Max.y) {
            SetScrollFromPosY(window, item_rect.Max.y + g.Style.ItemSpacing.y - window.Pos.y, 1.0);
        }
    }
    else if ((flag_set(flags, ImGuiScrollFlags_KeepVisibleCenterY) && !fully_visible_y) || flag_set(flags, ImGuiScrollFlags_AlwaysCenterY))
    {
        let target_y: c_float =  if can_be_fully_visible_y { ImFloor((item_rect.Min.y + item_rect.Max.y - window.InnerRect.GetHeight()) * 0.5) } else { item_rect.Min.y };
        SetScrollFromPosY(window, target_y - window.Pos.y, 0.0);
    }

    let next_scroll: ImVec2 = CalcNextScrollFromScrollTargetAndClamp(window);
    let mut delta_scroll: ImVec2 = next_scroll - window.Scroll;

    // Also scroll parent window to keep us into view if necessary
    if flag_clear(flags, ImGuiScrollFlags_NoScrollParent) && flag_set(window.Flags, ImGuiWindowFlags_ChildWindow)
    {
        // FIXME-SCROLL: May be an option?
        if (in_flags & (ImGuiScrollFlags_AlwaysCenterX | ImGuiScrollFlags_KeepVisibleCenterX)) != 0 {
            in_flags = flag_set(in_flags, !ImGuiScrollFlags_MaskX_) | ImGuiScrollFlags_KeepVisibleEdgeX;
        }
        if (in_flags & (ImGuiScrollFlags_AlwaysCenterY | ImGuiScrollFlags_KeepVisibleCenterY)) != 0 {
            in_flags = (in_flags & !ImGuiScrollFlags_MaskY_) | ImGuiScrollFlags_KeepVisibleEdgeY;
        }
        delta_scroll += ScrollToRectEx(window.ParentWindow, ImRect(item_rect.Min - delta_scroll, item_rect.Max - delta_scroll), in_flags);
    }

    return delta_scroll;
}

// GetScrollX: c_float()
pub unsafe fn GetScrollX() -> c_float
{
    let mut window: *mut ImGuiWindow =  GimGui.CurrentWindow;
    return window.Scroll.x;
}

// GetScrollY: c_float()
pub unsafe fn GetScrollY() -> c_float
{
    let mut window: *mut ImGuiWindow =  GimGui.CurrentWindow;
    return window.Scroll.y;
}

// GetScrollMaxX: c_float()
pub unsafe fn GetScrollMax() -> c_float
{
    let mut window: *mut ImGuiWindow =  GimGui.CurrentWindow;
    return window.ScrollMax.x;
}

// GetScrollMaxY: c_float()
pub unsafe fn GetScrollMaxY() -> c_float
{
    let mut window: *mut ImGuiWindow =  GimGui.CurrentWindow;
    return window.ScrollMax.y;
}

pub unsafe fn SetScrollX(window: *mut ImGuiWindow,scroll_x: c_float)
{
    window.ScrollTarget.x = scroll_x;
    window.ScrollTargetCenterRatio.x = 0.0;
    window.ScrollTargetEdgeSnapDist.x = 0.0;
}

pub unsafe fn SetScrollY(window: *mut ImGuiWindow,scroll_y: c_float)
{
    window.ScrollTarget.y = scroll_y;
    window.ScrollTargetCenterRatio.y = 0.0;
    window.ScrollTargetEdgeSnapDist.y = 0.0;
}

pub unsafe fn SetScrollX2(scroll_x: c_float)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    SetScrollX(g.CurrentWindow, scroll_x);
}

pub unsafe fn SetScrollY2(scroll_y: c_float)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    SetScrollY(g.CurrentWindow, scroll_y);
}

// Note that a local position will vary depending on initial scroll value,
// This is a little bit confusing so bear with us:
//  - local_pos = (absolution_pos - window.Pos)
//  - So local_x/local_y are 0.0 for a position at the upper-left corner of a window,
//    and generally local_x/local_y are >(padding+decoration) && <(size-padding-decoration) when in the visible area.
//  - They mostly exists because of legacy API.
// Following the rules above, when trying to work with scrolling code, consider that:
//  - SetScrollFromPosY(0.0) == SetScrollY(0.0 + scroll.y) == has no effect!
//  - SetScrollFromPosY(-scroll.y) == SetScrollY(-scroll.y + scroll.y) == SetScrollY(0.0) == reset scroll. Of course writing SetScrollY(0.0) directly then makes more sense
// We store a target position so centering and clamping can occur on the next frame when we are guaranteed to have a known window size
pub unsafe fn SetScrollFromPosX(window: *mut ImGuiWindow,local_x: c_float,center_x_ratio: c_float)
{
    // IM_ASSERT(center_x_ratio >= 0.0 && center_x_ratio <= 1.0);
    window.ScrollTarget.x = IM_FLOOR(local_x + window.Scroll.x); // Convert local position to scroll offset
    window.ScrollTargetCenterRatio.x = center_x_ratio;
    window.ScrollTargetEdgeSnapDist.x = 0.0;
}

pub unsafe fn SetScrollFromPosY(window: *mut ImGuiWindow, mut local_y: c_float,center_y_ratio: c_float)
{
    // IM_ASSERT(center_y_ratio >= 0.0 && center_y_ratio <= 1.0);
    let decoration_up_height: c_float =  window.TitleBarHeight() + window.MenuBarHeight(); // FIXME: Would be nice to have a more standardized access to our scrollable/client rect;
    local_y -= decoration_up_height;
    window.ScrollTarget.y = IM_FLOOR(local_y + window.Scroll.y); // Convert local position to scroll offset
    window.ScrollTargetCenterRatio.y = center_y_ratio;
    window.ScrollTargetEdgeSnapDist.y = 0.0;
}

pub unsafe fn SetScrollFromPosX2(local_x: c_float,center_x_ratio: c_float)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    SetScrollFromPosX(g.CurrentWindow, local_x, center_x_ratio);
}

pub unsafe fn SetScrollFromPosY2(local_y: c_float,center_y_ratio: c_float)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    SetScrollFromPosY(g.CurrentWindow, local_y, center_y_ratio);
}

// center_x_ratio: 0.0 left of last item, 0.5 horizontal center of last item, 1.0 right of last item.
pub unsafe fn SetScrollHereX(center_x_ratio: c_float)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    let spacing_x: c_float =  ImMax(window.WindowPadding.x, g.Style.ItemSpacing.x);
    let target_pos_x: c_float =  ImLerp(g.LastItemData.Rect.Min.x - spacing_x, g.LastItemData.Rect.Max.x + spacing_x, center_x_ratio);
    SetScrollFromPosX(window, target_pos_x - window.Pos.x, center_x_ratio); // Convert from absolute to local pos

    // Tweak: snap on edges when aiming at an item very close to the edge
    window.ScrollTargetEdgeSnapDist.x = ImMax(0.0, window.WindowPadding.x - spacing_x);
}

// center_y_ratio: 0.0 top of last item, 0.5 vertical center of last item, 1.0 bottom of last item.
pub unsafe fn SetScrollHereY(center_y_ratio: c_float)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    let spacing_y: c_float =  ImMax(window.WindowPadding.y, g.Style.ItemSpacing.y);
    let target_pos_y: c_float =  ImLerp(window.DC.CursorPosPrevLine.y - spacing_y, window.DC.CursorPosPrevLine.y + window.DC.PrevLineSize.y + spacing_y, center_y_ratio);
    SetScrollFromPosY(window, target_pos_y - window.Pos.y, center_y_ratio); // Convert from absolute to local pos

    // Tweak: snap on edges when aiming at an item very close to the edge
    window.ScrollTargetEdgeSnapDist.y = ImMax(0.0, window.WindowPadding.y - spacing_y);
}
