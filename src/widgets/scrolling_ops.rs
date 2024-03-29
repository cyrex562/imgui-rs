// Helper to snap on edges when aiming at an item very close to the edge,
// So the difference between WindowPadding and ItemSpacing will be in the visible area after scrolling.
// When we refactor the scrolling API this may be configurable with a flag?
// Note that the effect for this won't be visible on X axis with default Style settings as WindowPadding.x == ItemSpacing.x by default.
pub unsafe fn staticCalcScrollEdgeSnap(
    target: c_float,
    snap_min: c_float,
    snap_max: c_float,
    snap_threshold: c_float,
    center_ratio: c_float,
) -> f32 {
    if target <= snap_min + snap_threshold {
        return ImLerp(snap_min, target, center_ratio);
    }
    if target >= snap_max - snap_threshold {
        return ImLerp(target, snap_max, center_ratio);
    }
    return target;
}

use crate::color::{
    ImGuiCol_ScrollbarBg, ImGuiCol_ScrollbarGrab, ImGuiCol_ScrollbarGrabActive,
    ImGuiCol_ScrollbarGrabHovered,
};
use crate::core::axis::{ImGuiAxis, IM_GUI_AXIS_X, IM_GUI_AXIS_Y};
use crate::drawing::draw_flags::{
    ImDrawFlags, ImDrawFlags_RoundCornersBottomLeft, ImDrawFlags_RoundCornersBottomRight,
    ImDrawFlags_RoundCornersNone, ImDrawFlags_RoundCornersTopRight,
};
use crate::core::id_ops::{KeepAliveID, SetHoveredID};
use crate::core::math_ops::{ImClamp, ImLerp, ImMax, ImMin};
use crate::rect::ImRect;
use crate::widgets::scroll_flags::{
    ImGuiScrollFlags, ImGuiScrollFlags_AlwaysCenterX, ImGuiScrollFlags_AlwaysCenterY,
    ImGuiScrollFlags_KeepVisibleCenterX, ImGuiScrollFlags_KeepVisibleCenterY,
    ImGuiScrollFlags_KeepVisibleEdgeX, ImGuiScrollFlags_KeepVisibleEdgeY, ImGuiScrollFlags_MaskX_,
    ImGuiScrollFlags_MaskY_, ImGuiScrollFlags_NoScrollParent,
};
use crate::style_ops::GetColorU32;
use crate::core::type_defs::ImguiHandle;
use crate::core::utils::{flag_clear, flag_set};
use crate::core::vec2::Vector2;
use crate::widgets::button_flags::ImGuiButtonFlags_NoNavFocus;
use crate::window::window_flags::{
    ImGuiWindowFlags_ChildWindow, ImGuiWindowFlags_MenuBar, ImGuiWindowFlags_NoTitleBar,
};
use crate::window::ImguiWindow;
use crate::{button_ops, GImGui};
use libc::c_float;

// static CalcNextScrollFromScrollTargetAndClamp: ImVec2(window: &mut ImGuiWindow)
pub fn CalcNextScrollFromScrollTargetAndClamp(window: &mut ImguiWindow) -> Vector2 {
    let mut scroll: Vector2 = window.scroll;
    if window.scrollTarget.x < f32::MAX {
        let decoration_total_width: c_float = window.scrollbarSizes.x;
        let center_x_ratio: c_float = window.scrollTargetCenterRatio.x;
        let mut scroll_target_x: c_float = window.scrollTarget.x;
        if window.scrollTargetEdgeSnapDist.x > 0.0 {
            let snap_x_min: c_float = 0.0;
            let snap_x_max: c_float =
                window.scrollMax.x + window.SizeFull.x - decoration_total_width;
            scroll_target_x = CalcScrollEdgeSnap(
                scroll_target_x,
                snap_x_min,
                snap_x_max,
                window.scrollTargetEdgeSnapDist.x,
                center_x_ratio,
            );
        }
        scroll.x = scroll_target_x - center_x_ratio * (window.SizeFull.x - decoration_total_width);
    }
    if window.scrollTarget.y < f32::MAX {
        let decoration_total_height: c_float =
            window.TitleBarHeight() + window.MenuBarHeight() + window.scrollbarSizes.y;
        let center_y_ratio: c_float = window.scrollTargetCenterRatio.y;
        let mut scroll_target_y: c_float = window.scrollTarget.y;
        if window.scrollTargetEdgeSnapDist.y > 0.0 {
            let snap_y_min: c_float = 0.0;
            let snap_y_max: c_float =
                window.scrollMax.y + window.SizeFull.y - decoration_total_height;
            scroll_target_y = CalcScrollEdgeSnap(
                scroll_target_y,
                snap_y_min,
                snap_y_max,
                window.scrollTargetEdgeSnapDist.y,
                center_y_ratio,
            );
        }
        scroll.y = scroll_target_y - center_y_ratio * (window.SizeFull.y - decoration_total_height);
    }
    scroll.x = IM_FLOOR(ImMax(scroll.x, 0.0));
    scroll.y = IM_FLOOR(ImMax(scroll.y, 0.0));
    if !window.Collapsed && !window.skip_items {
        scroll.x = ImMin(scroll.x, window.scrollMax.x);
        scroll.y = ImMin(scroll.y, window.scrollMax.y);
    }
    return scroll;
}

pub unsafe fn ScrollToItem(flags: ImGuiScrollFlags) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.current_window_mut().unwrap();
    ScrollToRectEx(window, &mut g.last_item_data.NavRect, flags);
}

pub unsafe fn ScrollToRect(
    window: &mut ImguiWindow,
    item_rect: &mut ImRect,
    flags: ImGuiScrollFlags,
) {
    ScrollToRectEx(window, item_rect, flags);
}

// Scroll to keep newly navigated item fully into view
pub unsafe fn ScrollToRectEx(
    window: &mut ImguiWindow,
    item_rect: &mut ImRect,
    mut flags: ImGuiScrollFlags,
) -> Vector2 {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window_rect: ImRect = ImRect::new(
        window.InnerRect.min - Vector2::from_floats(1.0, 1.0),
        window.InnerRect.max + Vector2::from_floats(1.0, 1.0),
    );
    //GetForegroundDrawList(window).AddRect(window_rect.Min, window_rect.Max, IM_COL32_WHITE); // [DEBUG]

    // Check that only one behavior is selected per axis
    // IM_ASSERT(flag_set(flags, ImGuiScrollFlags_MaskX_) == 0 || ImIsPowerOfTwo(flags & ImGuiScrollFlags_MaskX_));
    // IM_ASSERT(flag_set(flags, ImGuiScrollFlags_MaskY_) == 0 || ImIsPowerOfTwo(flags & ImGuiScrollFlags_MaskY_));

    // Defaults
    let mut in_flags = flags;
    if (flag_clear(flags, ImGuiScrollFlags_MaskX_) && window.scrollbarX) {
        flags |= ImGuiScrollFlags_KeepVisibleEdgeX;
    }
    if (flag_clear(flags, ImGuiScrollFlags_MaskY_)) {
        flags |= if window.Appearing {
            ImGuiScrollFlags_AlwaysCenterY
        } else {
            ImGuiScrollFlags_KeepVisibleEdgeY
        };
    }

    let fully_visible_x: bool =
        item_rect.min.x >= window_rect.min.x && item_rect.max.x <= window_rect.max.x;
    let fully_visible_y: bool =
        item_rect.min.y >= window_rect.min.y && item_rect.max.y <= window_rect.max.y;
    let can_be_fully_visible_x: bool =
        (item_rect.GetWidth() + g.style.ItemSpacing.x * 2.0) <= window_rect.GetWidth();
    let can_be_fully_visible_y: bool =
        (item_rect.GetHeight() + g.style.ItemSpacing.y * 2.0) <= window_rect.GetHeight();

    if flag_set(flags, ImGuiScrollFlags_KeepVisibleEdgeX) && !fully_visible_x {
        if item_rect.min.x < window_rect.min.x || !can_be_fully_visible_x {
            SetScrollFromPosX(
                window,
                item_rect.min.x - g.style.ItemSpacing.x - window.position.x,
                0.0,
            );
        } else if item_rect.max.x >= window_rect.max.x {
            SetScrollFromPosX(
                window,
                item_rect.max.x + g.style.ItemSpacing.x - window.position.x,
                1.0,
            );
        }
    } else if (flag_set(flags, ImGuiScrollFlags_KeepVisibleCenterX) && !fully_visible_x)
        || flag_set(flags, ImGuiScrollFlags_AlwaysCenterX)
    {
        let target_x: c_float = if can_be_fully_visible_x {
            ImFloor((item_rect.min.x + item_rect.max.x - window.InnerRect.GetWidth()) * 0.5)
        } else {
            item_rect.min.x
        };
        SetScrollFromPosX(window, target_x - window.position.x, 0.0);
    }

    if (flag_set(flags, ImGuiScrollFlags_KeepVisibleEdgeY) && !fully_visible_y) {
        if (item_rect.min.y < window_rect.min.y || !can_be_fully_visible_y) {
            SetScrollFromPosY(
                window,
                item_rect.min.y - g.style.ItemSpacing.y - window.position.y,
                0.0,
            );
        } else if (item_rect.max.y >= window_rect.max.y) {
            SetScrollFromPosY(
                window,
                item_rect.max.y + g.style.ItemSpacing.y - window.position.y,
                1.0,
            );
        }
    } else if ((flag_set(flags, ImGuiScrollFlags_KeepVisibleCenterY) && !fully_visible_y)
        || flag_set(flags, ImGuiScrollFlags_AlwaysCenterY))
    {
        let target_y: c_float = if can_be_fully_visible_y {
            ImFloor((item_rect.min.y + item_rect.max.y - window.InnerRect.GetHeight()) * 0.5)
        } else {
            item_rect.min.y
        };
        SetScrollFromPosY(window, target_y - window.position.y, 0.0);
    }

    let next_scroll: Vector2 = CalcNextScrollFromScrollTargetAndClamp(window);
    let mut delta_scroll: Vector2 = next_scroll - window.scroll;

    // Also scroll parent window to keep us into view if necessary
    if flag_clear(flags, ImGuiScrollFlags_NoScrollParent)
        && flag_set(window.Flags, ImGuiWindowFlags_ChildWindow)
    {
        // FIXME-SCROLL: May be an option?
        if (in_flags & (ImGuiScrollFlags_AlwaysCenterX | ImGuiScrollFlags_KeepVisibleCenterX)) != 0
        {
            in_flags =
                flag_set(in_flags, !ImGuiScrollFlags_MaskX_) | ImGuiScrollFlags_KeepVisibleEdgeX;
        }
        if (in_flags & (ImGuiScrollFlags_AlwaysCenterY | ImGuiScrollFlags_KeepVisibleCenterY)) != 0
        {
            in_flags = (in_flags & !ImGuiScrollFlags_MaskY_) | ImGuiScrollFlags_KeepVisibleEdgeY;
        }
        delta_scroll += ScrollToRectEx(
            window.ParentWindow,
            ImRect(item_rect.min - delta_scroll, item_rect.max - delta_scroll),
            in_flags,
        );
    }

    return delta_scroll;
}

// GetScrollX: c_float()
pub unsafe fn GetScrollX() -> f32 {
    let mut window = g.current_window_mut().unwrap();
    return window.scroll.x;
}

// GetScrollY: c_float()
pub unsafe fn GetScrollY() -> f32 {
    let mut window = g.current_window_mut().unwrap();
    return window.scroll.y;
}

// GetScrollMaxX: c_float()
pub unsafe fn GetScrollMax() -> f32 {
    let mut window = g.current_window_mut().unwrap();
    return window.scrollMax.x;
}

// GetScrollMaxY: c_float()
pub unsafe fn GetScrollMaxY() -> f32 {
    let mut window = g.current_window_mut().unwrap();
    return window.scrollMax.y;
}

pub unsafe fn SetScrollX(window: &mut ImguiWindow, scroll_x: c_float) {
    window.scrollTarget.x = scroll_x;
    window.scrollTargetCenterRatio.x = 0.0;
    window.scrollTargetEdgeSnapDist.x = 0.0;
}

pub unsafe fn SetScrollY(window: &mut ImguiWindow, scroll_y: c_float) {
    window.scrollTarget.y = scroll_y;
    window.scrollTargetCenterRatio.y = 0.0;
    window.scrollTargetEdgeSnapDist.y = 0.0;
}

pub unsafe fn SetScrollX2(scroll_x: c_float) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    SetScrollX(g.CurrentWindow, scroll_x);
}

pub unsafe fn SetScrollY2(scroll_y: c_float) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    SetScrollY(g.CurrentWindow, scroll_y);
}

// Note that a local position will vary depending on initial scroll value,
// This is a little bit confusing so bear with us:
//  - local_pos = (absolution_pos - window.position)
//  - So local_x/local_y are 0.0 for a position at the upper-left corner of a window,
//    and generally local_x/local_y are >(padding+decoration) && <(size-padding-decoration) when in the visible area.
//  - They mostly exists because of legacy API.
// Following the rules above, when trying to work with scrolling code, consider that:
//  - SetScrollFromPosY(0.0) == SetScrollY(0.0 + scroll.y) == has no effect!
//  - SetScrollFromPosY(-scroll.y) == SetScrollY(-scroll.y + scroll.y) == SetScrollY(0.0) == reset scroll. Of course writing SetScrollY(0.0) directly then makes more sense
// We store a target position so centering and clamping can occur on the next frame when we are guaranteed to have a known window size
pub unsafe fn SetScrollFromPosX(
    window: &mut ImguiWindow,
    local_x: c_float,
    center_x_ratio: c_float,
) {
    // IM_ASSERT(center_x_ratio >= 0.0 && center_x_ratio <= 1.0);
    window.scrollTarget.x = IM_FLOOR(local_x + window.scroll.x); // Convert local position to scroll offset
    window.scrollTargetCenterRatio.x = center_x_ratio;
    window.scrollTargetEdgeSnapDist.x = 0.0;
}

pub unsafe fn SetScrollFromPosY(
    window: &mut ImguiWindow,
    mut local_y: c_float,
    center_y_ratio: c_float,
) {
    // IM_ASSERT(center_y_ratio >= 0.0 && center_y_ratio <= 1.0);
    let decoration_up_height: c_float = window.TitleBarHeight() + window.MenuBarHeight(); // FIXME: Would be nice to have a more standardized access to our scrollable/client rect;
    local_y -= decoration_up_height;
    window.scrollTarget.y = IM_FLOOR(local_y + window.scroll.y); // Convert local position to scroll offset
    window.scrollTargetCenterRatio.y = center_y_ratio;
    window.scrollTargetEdgeSnapDist.y = 0.0;
}

pub unsafe fn SetScrollFromPosX2(local_x: c_float, center_x_ratio: c_float) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    SetScrollFromPosX(g.CurrentWindow, local_x, center_x_ratio);
}

pub unsafe fn SetScrollFromPosY2(local_y: c_float, center_y_ratio: c_float) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    SetScrollFromPosY(g.CurrentWindow, local_y, center_y_ratio);
}

// center_x_ratio: 0.0 left of last item, 0.5 horizontal center of last item, 1.0 right of last item.
pub unsafe fn SetScrollHereX(center_x_ratio: c_float) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.current_window_mut().unwrap();
    let spacing_x: c_float = ImMax(window.WindowPadding.x, g.style.ItemSpacing.x);
    let target_pos_x: c_float = ImLerp(
        g.last_item_data.Rect.Min.x - spacing_x,
        g.last_item_data.Rect.Max.x + spacing_x,
        center_x_ratio,
    );
    SetScrollFromPosX(window, target_pos_x - window.position.x, center_x_ratio); // Convert from absolute to local pos

    // Tweak: snap on edges when aiming at an item very close to the edge
    window.scrollTargetEdgeSnapDist.x = ImMax(0.0, window.WindowPadding.x - spacing_x);
}

// center_y_ratio: 0.0 top of last item, 0.5 vertical center of last item, 1.0 bottom of last item.
pub unsafe fn SetScrollHereY(center_y_ratio: c_float) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.current_window_mut().unwrap();
    let spacing_y: c_float = ImMax(window.WindowPadding.y, g.style.ItemSpacing.y);
    let target_pos_y: c_float = ImLerp(
        window.dc.cursor_pos_prev_line.y - spacing_y,
        window.dc.cursor_pos_prev_line.y + window.dc.PrevLineSize.y + spacing_y,
        center_y_ratio,
    );
    SetScrollFromPosY(window, target_pos_y - window.position.y, center_y_ratio); // Convert from absolute to local pos

    // Tweak: snap on edges when aiming at an item very close to the edge
    window.scrollTargetEdgeSnapDist.y = ImMax(0.0, window.WindowPadding.y - spacing_y);
}

pub unsafe fn GetWindowScrollbarID(window: &mut ImguiWindow, axis: ImGuiAxis) -> ImguiHandle {
    return window.id_by_string(if axis == IM_GUI_AXIS_X {
        "#SCROLLX"
    } else {
        "#SCROLLY"
    });
}

// Return scrollbar rectangle, must only be called for corresponding axis if window.scrollbarX/Y is set.
pub unsafe fn GetWindowScrollbarRect(window: &mut ImguiWindow, axis: ImGuiAxis) -> ImRect {
    let outer_rect: ImRect = window.Rect();
    let inner_rect: ImRect = window.InnerRect;
    let border_size: c_float = window.WindowBorderSize;
    let scrollbar_size: c_float = window.scrollbarSizes[axis ^ 1]; // (ScrollbarSizes.x = width of Y scrollbar; ScrollbarSizes.y = height of X scrollbar)
                                                                   // IM_ASSERT(scrollbar_size > 0.0);
    if (axis == IM_GUI_AXIS_X) {
        return ImRect(
            inner_rect.min.x,
            ImMax(
                outer_rect.min.y,
                outer_rect.max.y - border_size - scrollbar_size,
            ),
            inner_rect.max.x,
            outer_rect.max.y,
        );
    } else {
        return ImRect(
            ImMax(
                outer_rect.min.x,
                outer_rect.max.x - border_size - scrollbar_size,
            ),
            inner_rect.min.y,
            outer_rect.max.x,
            inner_rect.max.y,
        );
    }
}

pub unsafe fn Scrollbar(axis: ImGuiAxis) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.current_window_mut().unwrap();
    let mut id: ImguiHandle = GetWindowScrollbarID(window, axis);

    // Calculate scrollbar bounding box
    let mut bb: ImRect = GetWindowScrollbarRect(window, axis);
    rounding_corners: ImDrawFlags = ImDrawFlags_RoundCornersNone;
    if axis == IM_GUI_AXIS_X {
        rounding_corners |= ImDrawFlags_RoundCornersBottomLeft;
        if !window.scrollbarY {
            rounding_corners |= ImDrawFlags_RoundCornersBottomRight;
        }
    } else {
        if flag_set(window.Flags, ImGuiWindowFlags_NoTitleBar)
            && flag_clear(window.Flags, ImGuiWindowFlags_MenuBar)
        {
            rounding_corners |= ImDrawFlags_RoundCornersTopRight;
        }
        if !window.scrollbarX {
            rounding_corners |= ImDrawFlags_RoundCornersBottomRight;
        }
    }
    let size_avail: c_float = window.InnerRect.Max[axis] - window.InnerRect.Min[axis];
    let size_contents: c_float = window.ContentSize[axis] + window.WindowPadding[axis] * 2.0;
    let scroll = window.scroll[axis];
    ScrollbarEx(
        &mut bb,
        id,
        axis,
        &mut scroll,
        size_avail as i64,
        size_contents as i64,
        rounding_corners,
    );
    window.scroll[axis] = scroll;
}

// Vertical/Horizontal scrollbar
// The entire piece of code below is rather confusing because:
// - We handle absolute seeking (when first clicking outside the grab) and relative manipulation (afterward or when clicking inside the grab)
// - We store values as normalized ratio and in a form that allows the window content to change while we are holding on a scrollbar
// - We handle both horizontal and vertical scrollbars, which makes the terminology not ideal.
// Still, the code should probably be made simpler..
pub unsafe fn ScrollbarEx(
    bb_frame: &mut ImRect,
    id: ImguiHandle,
    axis: ImGuiAxis,
    p_scroll_v: *mut i64,
    size_avail_v: i64,
    size_contents_v: i64,
    flags: ImDrawFlags,
) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.current_window_mut().unwrap();
    if window.skip_items {
        return false;
    }

    KeepAliveID(g, id);

    let bb_frame_width: c_float = bb_frame.GetWidth();
    let bb_frame_height: c_float = bb_frame.GetHeight();
    if bb_frame_width <= 0.0 || bb_frame_height <= 0.0 {
        return false;
    }

    // When we are too small, start hiding and disabling the grab (this reduce visual noise on very small window and facilitate using the window resize grab)
    let mut alpha: c_float = 1.0;
    if (axis == IM_GUI_AXIS_Y) && bb_frame_height < g.FontSize + g.style.FramePadding.y * 2.0 {
        alpha = ImSaturate((bb_frame_height - g.FontSize) / (g.style.FramePadding.y * 2.0));
    }
    if alpha <= 0.0 {
        return false;
    }

    let setyle = &mut g.style;
    let allow_interaction: bool = (alpha >= 1.0);

    let bb = bb_frame;
    bb.expand_from_vec(&Vector2::from_floats(
        -ImClamp(IM_FLOOR((bb_frame_width - 2.0) * 0.5), 0.0, 3.0),
        -ImClamp(IM_FLOOR((bb_frame_height - 2.0) * 0.5), 0.0, 3.0),
    ));

    // V denote the main, longer axis of the scrollbar (= height for a vertical scrollbar)
    let scrollbar_size_v: c_float = if (axis == IM_GUI_AXIS_X) {
        bb.GetWidth()
    } else {
        bb.GetHeight()
    };

    // Calculate the height of our grabbable box. It generally represent the amount visible (vs the total scrollable amount)
    // But we maintain a minimum size in pixel to allow for the user to still aim inside.
    // IM_ASSERT(ImMax(size_contents_v, size_avail_v) > 0.0); // Adding this assert to check if the ImMax(XXX,1.0) is still needed. PLEASE CONTACT ME if this triggers.
    let win_size_v = ImMax(ImMax(size_contents_v, size_avail_v), 1);
    let grab_h_pixels: c_float = ImClamp(
        scrollbar_size_v * (size_avail_v / win_size_v),
        style.GrabMinSize,
        scrollbar_size_v,
    );
    let grab_h_norm: c_float = grab_h_pixels / scrollbar_size_v;

    // Handle input right away. None of the code of Begin() is relying on scrolling position before calling Scrollbar().
    let mut held: bool = false;
    let mut hovered: bool = false;
    button_ops::ButtonBehavior(
        g,
        bb,
        id,
        &mut hovered,
        &mut held,
        ImGuiButtonFlags_NoNavFocus,
    );

    let scroll_max = ImMax(1, size_contents_v - size_avail_v);
    let mut scroll_ratio: c_float = ImSaturate(*p_scroll_v / scroll_max);
    let mut grab_v_norm: c_float =
        scroll_ratio * (scrollbar_size_v - grab_h_pixels) / scrollbar_size_v; // Grab position in normalized space
    if (held && allow_interaction && grab_h_norm < 1.0) {
        let scrollbar_pos_v: c_float = bb.min[axis];
        let mouse_pos_v: c_float = g.IO.MousePos[axis];

        // Click position in scrollbar normalized space (0.0->1.0)
        let clicked_v_norm: c_float =
            ImSaturate((mouse_pos_v - scrollbar_pos_v) / scrollbar_size_v);
        SetHoveredID(id);

        let mut seek_absolute: bool = false;
        if (g.ActiveIdIsJustActivated) {
            // On initial click calculate the distance between mouse and the center of the grab
            seek_absolute =
                (clicked_v_norm < grab_v_norm || clicked_v_norm > grab_v_norm + grab_h_norm);
            if seek_absolute {
                g.ScrollbarClickDeltaToGrabCenter = 0.0;
            } else {
                g.ScrollbarClickDeltaToGrabCenter =
                    clicked_v_norm - grab_v_norm - grab_h_norm * 0.5;
            }
        }

        // Apply scroll (p_scroll_v will generally point on one member of window.scroll)
        // It is ok to modify Scroll here because we are being called in Begin() after the calculation of ContentSize and before setting up our starting position
        let scroll_v_norm: c_float = ImSaturate(
            (clicked_v_norm - g.ScrollbarClickDeltaToGrabCenter - grab_h_norm * 0.5)
                / (1.0 - grab_h_norm),
        );
        *p_scroll_v = (scroll_v_norm * scroll_max);

        // Update values for rendering
        scroll_ratio = ImSaturate(*p_scroll_v / scroll_max);
        grab_v_norm = scroll_ratio * (scrollbar_size_v - grab_h_pixels) / scrollbar_size_v;

        // Update distance to grab now that we have seeked and saturated
        if seek_absolute {
            g.ScrollbarClickDeltaToGrabCenter = clicked_v_norm - grab_v_norm - grab_h_norm * 0.5;
        }
    }

    // Render
    bg_col: u32 = GetColorU32(ImGuiCol_ScrollbarBg, 0.0);
    grab_col: u32 = GetColorU32(
        if held {
            ImGuiCol_ScrollbarGrabActive
        } else {
            if hovered {
                ImGuiCol_ScrollbarGrabHovered
            } else {
                ImGuiCol_ScrollbarGrab
            }
        },
        alpha,
    );
    window.DrawList.AddRectFilled(
        &bb_frame.min,
        &bb_frame.max,
        bg_col,
        window.WindowRounding,
        flags,
    );
    let mut grab_rect: ImRect = ImRect::default();
    if (axis == IM_GUI_AXIS_X) {
        grab_rect = ImRect(
            ImLerp(bb.min.x, bb.max.x, grab_v_norm),
            bb.min.y,
            ImLerp(bb.min.x, bb.max.x, grab_v_norm) + grab_h_pixels,
            bb.max.y,
        );
    } else {
        grab_rect = ImRect(
            bb.min.x,
            ImLerp(bb.min.y, bb.max.y, grab_v_norm),
            bb.max.x,
            ImLerp(bb.min.y, bb.max.y, grab_v_norm) + grab_h_pixels,
        );
    }
    window.DrawList.AddRectFilled(
        &grab_rect.min,
        &grab_rect.max,
        grab_col,
        style.ScrollbarRounding,
        0,
    );

    return held;
}
