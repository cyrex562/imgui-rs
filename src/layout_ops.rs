#![allow(non_snake_case)]

use libc::{c_float, c_int};
use std::ptr::null;
use crate::GImGui;
use crate::item_ops::{ItemAdd, ItemSize};
use crate::layout_type::{ImGuiLayoutType, ImGuiLayoutType_Vertical};
use crate::math_ops::ImMax;
use crate::rect::ImRect;
use crate::shrink_width_item::ImGuiShrinkWidthItem;
use crate::vec2::ImVec2;
use crate::window::ImGuiWindow;
use crate::window::ops::GetCurrentWindow;

// Gets back to previous line and continue with horizontal layout
//      offset_from_start_x == 0 : follow right after previous item
//      offset_from_start_x != 0 : align to specified x position (relative to window/group left)
//      spacing_w < 0            : use default spacing if pos_x == 0, no spacing if pos_x != 0
//      spacing_w >= 0           : enforce spacing amount
pub unsafe fn SameLine(offset_from_start_x: c_float, spacing_w: c_float) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    if window.SkipItems {
        return;
    }

    if offset_from_start_x != 0.0 {
        if spacing_w < 0.0 {
            spacing_w = 0.0;
        }
        window.DC.CursorPos.x = window.Pos.x - window.Scroll.x + offset_from_start_x + spacing_w + window.DC.GroupOffset.x + window.DC.ColumnsOffset.x;
        window.DC.CursorPos.y = window.DC.CursorPosPrevLine.y;
    } else {
        if spacing_w < 0.0 {
            spacing_w = g.Style.ItemSpacing.x;
        }
        window.DC.CursorPos.x = window.DC.CursorPosPrevLine.x + spacing_w;
        window.DC.CursorPos.y = window.DC.CursorPosPrevLine.y;
    }
    window.DC.CurrLineSize = window.DC.PrevLineSize;
    window.DC.CurrLineTextBaseOffset = window.DC.PrevLineTextBaseOffset;
    window.DC.IsSameLine = true;
}

pub unsafe fn Spacing()
{
    let mut window: *mut ImGuiWindow = GetCurrentWindow();
    if window.SkipItems { return ; }
    ItemSize(&ImVec2::from_ints(0, 0), 0.0);
}

pub unsafe fn Dummy(size: &ImVec2)
{
    let mut window: *mut ImGuiWindow = GetCurrentWindow();
    if window.SkipItems { return ; }

    let mut bb: ImRect = ImRect::new(window.DC.CursorPos, window.DC.CursorPos + size);
    ItemSize(size, 0.0);
    ItemAdd(&mut bb, 0, None, 0);
}

pub unsafe fn NewLine()
{
    let mut window: *mut ImGuiWindow = GetCurrentWindow();
    if window.SkipItems { return ; }

    let g = GImGui; // ImGuiContext& g = *GImGui;
    let backup_layout_type: ImGuiLayoutType = window.DC.LayoutType;
    window.DC.LayoutType = ImGuiLayoutType_Vertical;
    window.DC.IsSameLine = false;
    if window.DC.CurrLineSize.y > 0.0 {    // In the event that we are on a line with items that is smaller that FontSize high, we will preserve its height.
        ItemSize(&ImVec2::from_ints(0, 0), 0.0);
    }
    else {
        ItemSize(&ImVec2::from_floats(0.0, g.FontSize), 0.0);
    }
    window.DC.LayoutType = backup_layout_type;
}

pub unsafe fn AlignTextToFramePadding()
{
    let mut window: *mut ImGuiWindow = GetCurrentWindow();
    if window.SkipItems { return ; }

    let g = GImGui; // ImGuiContext& g = *GImGui;
    window.DC.CurrLineSize.y = ImMax(window.DC.CurrLineSize.y, g.FontSize + g.Style.FramePadding.y * 2);
    window.DC.CurrLineTextBaseOffset = ImMax(window.DC.CurrLineTextBaseOffset, g.Style.FramePadding.y);
}

pub fn ShrinkWidthItemComparer(lhs: *const ImGuiShrinkWidthItem, rhs: *const ImGuiShrinkWidthItem) -> c_int
{
    let a: *const ImGuiShrinkWidthItem = lhs;
    let b: *const ImGuiShrinkWidthItem = rhs;
    let d: c_int = (b.Width - a.Width) as c_int;
    if d { return  d; }
    return b.Index - a.Index;
}

// Shrink excess width from a set of item, by removing width from the larger items first.
// Set items Width to -1.0 to disable shrinking this item.
pub unsafe fn ShrinkWidths(items: *mut ImGuiShrinkWidthItem, count: c_int, mut width_excess: c_float)
{
    if count == 1
    {
        if items[0].Width >= 0.0 {
            items[0].Width = ImMax(items[0].Width - width_excess, 1.0);
        }
        return;
    }
    // TODO:
    // ImQsort(items, count as size_t, sizeof(ImGuiShrinkWidthItem), ShrinkWidthItemComparer);
    let mut count_same_width: c_int = 1;
    while width_excess > 0.0 && count_same_width < count
    {
        while count_same_width < count && items[0].Width <= items[count_same_width].Width {
            count_same_width += 1;
        }
        let max_width_to_remove_per_item: c_float =  if count_same_width < count && items[count_same_width].Width >= 0.0 { (items[0].Width - items[count_same_width].Width) } else { items[0].Width - 1.0 };
        if max_width_to_remove_per_item <= 0.0{
            break;}
        let width_to_remove_per_item: c_float =  (width_excess / count_same_width).min( max_width_to_remove_per_item);
        // for (let item_n: c_int = 0; item_n < count_same_width; item_n++)
        for item_n in 0 .. count_same_width
        {
            items[item_n].Width -= width_to_remove_per_item;
        }
        width_excess -= width_to_remove_per_item * count_same_width;
    }

    // Round width and redistribute remainder
    // Ensure that e.g. the right-most tab of a shrunk tab-bar always reaches exactly at the same distance from the right-most edge of the tab bar separator.
    width_excess = 0.0;
    // for (let n: c_int = 0; n < count; n++)
    for n in 0 .. count
    {
        let width_rounded: c_float =  ImFloor(items[n].Width);
        width_excess += items[n].Width - width_rounded;
        items[n].Width = width_rounded;
    }
    while (width_excess > 0.0) {
        // for (let n: c_int = 0; n < count &&width_excess > 0.0; n+ +)
        for n in 0 .. count
        {
            let width_to_add: c_float = (items[n].InitialWidth - items[n].Width).min( 1.0);
            items[n].Width += width_to_add;
            width_excess -= width_to_add;
            if width_excess <= 0.0 {
                break;
            }
        }
    }
}
