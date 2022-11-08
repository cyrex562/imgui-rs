#![allow(non_snake_case)]

use crate::core::context::ImguiContext;
use crate::item_ops::{ItemAdd, ItemSize};
use crate::layout_type::{ImGuiLayoutType, ImGuiLayoutType_Vertical};
use crate::math_ops::ImMax;
use crate::rect::ImRect;
use crate::shrink_width_item::ImGuiShrinkWidthItem;
use crate::vec2::ImVec2;
use crate::window::ops::GetCurrentWindow;
use crate::window::ImguiWindow;
use crate::GImGui;
use libc::{c_float, c_int};
use std::ptr::null;

// Gets back to previous line and continue with horizontal layout
//      offset_from_start_x == 0 : follow right after previous item
//      offset_from_start_x != 0 : align to specified x position (relative to window/group left)
//      spacing_w < 0            : use default spacing if pos_x == 0, no spacing if pos_x != 0
//      spacing_w >= 0           : enforce spacing amount
pub fn same_line(g: &mut ImguiContext, offset_from_start_x: c_float, mut spacing_w: c_float) {
    let mut window = g.current_window_mut().unwrap();
    if window.skip_items {
        return;
    }

    if offset_from_start_x != 0.0 {
        if spacing_w < 0.0 {
            spacing_w = 0.0;
        }
        window.dc.cursor_pos.x = window.position.x - window.scroll.x
            + offset_from_start_x
            + spacing_w
            + window.dc.group_offset.x
            + window.dc.columns_offset.x;
        window.dc.cursor_pos.y = window.dc.cursor_pos_prev_line.y;
    } else {
        if spacing_w < 0.0 {
            spacing_w = g.style.item_spacing.x;
        }
        window.dc.cursor_pos.x = window.dc.cursor_pos_prev_line.x + spacing_w;
        window.dc.cursor_pos.y = window.dc.cursor_pos_prev_line.y;
    }
    window.dc.curr_line_size = window.dc.prev_line_size;
    window.dc.curr_line_text_base_offset = window.dc.prev_line_text_base_offset;
    window.dc.is_same_line = true;
}

pub fn spacing(g: &mut ImguiContext) {
    let mut window = g.current_window_mut().unwrap();
    if window.skip_items {
        return;
    }
    ItemSize(g, &ImVec2::from_ints(0, 0), 0.0);
}

pub fn Dummy(g: &mut ImguiContext, size: &ImVec2) {
    let mut window = g.current_window_mut().unwrap();
    if window.skip_items {
        return;
    }

    let mut bb = ImRect::from_vec2(&window.dc.cursor_pos, window.dc.cursor_pos + size);
    ItemSize(g, size, 0.0);
    ItemAdd(g, &mut bb, 0, None, 0);
}

pub fn NewLine(g: &mut ImguiContext) {
    let mut window = g.current_window_mut().unwrap();
    if window.skip_items {
        return;
    }

    let backup_layout_type = window.dc.LayoutType;
    window.dc.LayoutType = ImGuiLayoutType_Vertical;
    window.dc.is_same_line = false;
    if window.dc.curr_line_size.y > 0.0 {
        // In the event that we are on a line with items that is smaller that FontSize high, we will preserve its height.
        ItemSize(g, &ImVec2::from_ints(0, 0), 0.0);
    } else {
        ItemSize(g, &ImVec2::from_floats(0.0, g.FontSize), 0.0);
    }
    window.dc.LayoutType = backup_layout_type;
}

pub unsafe fn AlignTextToFramePadding() {
    let mut window = g.current_window_mut().unwrap();
    if window.skip_items {
        return;
    }

    let g = GImGui; // ImGuiContext& g = *GImGui;
    window.dc.CurrLineSize.y = ImMax(
        window.dc.CurrLineSize.y,
        g.FontSize + g.style.FramePadding.y * 2,
    );
    window.dc.CurrLineTextBaseOffset =
        ImMax(window.dc.CurrLineTextBaseOffset, g.style.FramePadding.y);
}

pub fn ShrinkWidthItemComparer(
    lhs: *const ImGuiShrinkWidthItem,
    rhs: *const ImGuiShrinkWidthItem,
) -> c_int {
    let a: *const ImGuiShrinkWidthItem = lhs;
    let b: *const ImGuiShrinkWidthItem = rhs;
    let d: c_int = (b.Width - a.Width) as c_int;
    if d {
        return d;
    }
    return b.Index - a.Index;
}

// Shrink excess width from a set of item, by removing width from the larger items first.
// Set items Width to -1.0 to disable shrinking this item.
pub unsafe fn ShrinkWidths(
    items: *mut ImGuiShrinkWidthItem,
    count: c_int,
    mut width_excess: c_float,
) {
    if count == 1 {
        if items[0].Width >= 0.0 {
            items[0].Width = ImMax(items[0].Width - width_excess, 1.0);
        }
        return;
    }
    // TODO:
    // ImQsort(items, count as size_t, sizeof(ImGuiShrinkWidthItem), ShrinkWidthItemComparer);
    let mut count_same_width: c_int = 1;
    while width_excess > 0.0 && count_same_width < count {
        while count_same_width < count && items[0].Width <= items[count_same_width].Width {
            count_same_width += 1;
        }
        let max_width_to_remove_per_item: c_float =
            if count_same_width < count && items[count_same_width].Width >= 0.0 {
                (items[0].Width - items[count_same_width].Width)
            } else {
                items[0].Width - 1.0
            };
        if max_width_to_remove_per_item <= 0.0 {
            break;
        }
        let width_to_remove_per_item: c_float =
            (width_excess / count_same_width).min(max_width_to_remove_per_item);
        // for (let item_n: c_int = 0; item_n < count_same_width; item_n++)
        for item_n in 0..count_same_width {
            items[item_n].Width -= width_to_remove_per_item;
        }
        width_excess -= width_to_remove_per_item * count_same_width;
    }

    // Round width and redistribute remainder
    // Ensure that e.g. the right-most tab of a shrunk tab-bar always reaches exactly at the same distance from the right-most edge of the tab bar separator.
    width_excess = 0.0;
    // for (let n: c_int = 0; n < count; n++)
    for n in 0..count {
        let width_rounded: c_float = ImFloor(items[n].Width);
        width_excess += items[n].Width - width_rounded;
        items[n].Width = width_rounded;
    }
    while (width_excess > 0.0) {
        // for (let n: c_int = 0; n < count &&width_excess > 0.0; n+ +)
        for n in 0..count {
            let width_to_add: c_float = (items[n].InitialWidth - items[n].Width).min(1.0);
            items[n].Width += width_to_add;
            width_excess -= width_to_add;
            if width_excess <= 0.0 {
                break;
            }
        }
    }
}
