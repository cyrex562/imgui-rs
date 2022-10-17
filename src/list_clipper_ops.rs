#![allow(non_snake_case)]
//-----------------------------------------------------------------------------
// [SECTION] ImGuiListClipper
// This is currently not as flexible/powerful as it should be and really confusing/spaghetti, mostly because we changed
// the API mid-way through development and support two ways to using the clipper, needs some rework (see TODO)
//-----------------------------------------------------------------------------

use std::ptr::null_mut;
use libc::{c_float, c_int, c_void};
use crate::direction::{ImGuiDir_Down, ImGuiDir_Up};
use crate::imgui::GImGui;
use crate::imgui_cpp::GImGui;
use crate::list_clipper::{ImGuiListClipper, ImGuiListClipper_SeekCursorForItem};
use crate::list_clipper_data::ImGuiListClipperData;
use crate::list_clipper_range::ImGuiListClipperRange;
use crate::math_ops::{ImClamp, ImIsFloatAboveGuaranteedIntegerPrecision, ImMax, ImMin, ImSwap};
use crate::nav_move_flags::ImGuiNavMoveFlags_Tabbing;
use crate::table_ops::TableEndRow;
use crate::window::rect::WindowRectRelToAbs;
use crate::window_ops::WindowRectRelToAbs;

// FIXME-TABLE: This prevents us from using ImGuiListClipper _inside_ a table cell.
// The problem we have is that without a Begin/End scheme for rows using the clipper is ambiguous.
// static GetSkipItemForListClipping: bool()
pub unsafe fn GetSkipItemForListClipping() -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    //return (g.CurrentTable ? g.Currenttable.HostSkipItems : g.Currentwindow.SkipItems);
    return if g.CurrentTable.is_null() == false {
        g.CurrentTable.HostSkipItems
    } else {
        g.CurrentWindow.SkipItems
    };
}

// #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
// Legacy helper to calculate coarse clipping of large list of evenly sized items.
// This legacy API is not ideal because it assume we will return a single contiguous rectangle.
// Prefer using ImGuiListClipper which can returns non-contiguous ranges.
// void CalcListClipping(int items_count, float items_height, int* out_items_display_start, int* out_items_display_end)
pub unsafe fn CalcListClipping(items_count: usize, items_height: c_float, out_items_display_start: *mut c_int, out_items_display_end: *mut c_int) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let window = g.CurrentWindow;
    if g.LogEnabled {
        // If logging is active, do not perform any clipping
        *out_items_display_start = 0;
        *out_items_display_end = items_count as c_int;
        return;
    }
    if GetSkipItemForListClipping() {
        *out_items_display_start = 0;
        *out_items_display_end = 0;
        return;
    }

    // We create the union of the ClipRect and the scoring rect which at worst should be 1 page away from ClipRect
    // We don't include g.NavId's rectangle in there (unless g.NavJustMovedToId is set) because the rectangle enlargement can get costly.
    let mut rect = window.ClipRect.clone();
    if g.NavMoveScoringItems {
        rect.Add2(&g.NavScoringNoClipRect);
    }
    if g.NavJustMovedToId != 0 && (window.NavLastIds[0] == g.NavJustMovedToId) {
        rect.Add2(&WindowRectRelToAbs(window, &window.NavRectRel[0])); // Could store and use NavJustMovedToRectRel
    }
    let pos = window.DC.CursorPos.clone();
    let mut start = ((rect.Min.y - pos.y) / items_height);
    let mut end = ((rect.Max.y - pos.y) / items_height);

    // When performing a navigation request, ensure we have one item extra in the direction we are moving to
    // FIXME: Verify this works with tabbing
    let is_nav_request = (g.NavMoveScoringItems && g.NavWindow.is_null() == false && g.NavWindow.RootWindowForNav == window.RootWindowForNav);
    if is_nav_request && g.NavMoveClipDir == ImGuiDir_Up {
        start -= 1;
    }
    if is_nav_request && g.NavMoveClipDir == ImGuiDir_Down {
        end += 1;
    }

    start = ImClamp(start, 0, items_count);
    end = ImClamp(end + 1, start, items_count);
    *out_items_display_start = start as c_int;
    *out_items_display_end = end as c_int;
}
// #endif

// static void ImGuiListClipper_SortAndFuseRanges(ImVector<ImGuiListClipperRange>& ranges, int offset = 0)
pub fn ImGuiListClipper_SortAndFuseRanges(ranges: &mut Vec<ImGuiListClipperRange>, offset: usize) {
    if ranges.Size - offset <= 1 {
        return;
    }

// Helper to order ranges and fuse them together if possible (bubble sort is fine as we are only sorting 2-3 entries)
//     for (int sort_end = ranges.Size - offset -1; sort_end > 0; - - sort_end)
    for sort_end in ranges.len() - offset - 1..0 {
        // for (int i = offset; i < sort_end + offset; + + i)
        for i in offset..sort_end + offset {
            if ranges[i].Min > ranges[i + 1].Min {
                ImSwap(&ranges[i], &ranges[i + 1]);
            }
        }
    }

// Now fuse ranges together as much as possible.
//     for (int i = 1 + offset; i < ranges.Size; i+ +)
    for mut i in 1 + offset..ranges.len() {

        // IM_ASSERT(!ranges[i].PosToIndexConvert && !ranges[i - 1].PosToIndexConvert);
        if ranges[i - 1].Max < ranges[i].Min {
            continue;
        }
        ranges[i - 1].Min = ImMin(ranges[i - 1].Min, ranges[i].Min);
        ranges[i - 1].Max = ImMax(ranges[i - 1].Max, ranges[i].Max);
        ranges.erase(ranges.Data + i);
        i -= 1;
    }
}


// static ImGuiListClipper_StepInternal: bool(ImGuiListClipper* clipper)
pub unsafe fn ImGuiListClipper_StepInternal(clipper: *mut ImGuiListClipper) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    let mut data: *mut ImGuiListClipperData = clipper.TempData;
// IM_ASSERT(data != NULL && "Called ImGuiListClipper::Step() too many times, or before ImGuiListClipper::Begin() ?");

    let mut table = g.CurrentTable;
    if table.is_null() == false && table.IsInsideRow {
        TableEndRow(table);
    }

// No items
    if clipper.ItemsCount == 0 || GetSkipItemForListClipping() {
        return false;
    }

// While we are in frozen row state, keep displaying items one by one, unclipped
// FIXME: Could be stored as a table-agnostic state.
    if data.StepNo == 0 && table != null_mut() && !table.IsUnfrozenRows {
        clipper.DisplayStart = data.ItemsFrozen;
        clipper.DisplayEnd = ImMin(data.ItemsFrozen + 1, clipper.ItemsCount);
        if clipper.DisplayStart < clipper.DisplayEnd {
            data.ItemsFrozen += 1;
        }
        return true;
    }

// Step 0: Let you process the first element (regardless of it being visible or not, so we can measure the element height)
    let mut calc_clipping: bool = false;
    if data.StepNo == 0 {
        clipper.StartPosY = window.DC.CursorPos.y;
        if clipper.ItemsHeight <= 0.0 {
// Submit the first item (or range) so we can measure its height (generally the first range is 0..1)
            data.Ranges.push_front(ImGuiListClipperRange::FromIndices(data.ItemsFrozen, data.ItemsFrozen + 1));
            clipper.DisplayStart = ImMax(data.Ranges[0].Min, data.ItemsFrozen);
            clipper.DisplayEnd = ImMin(data.Ranges[0].Max, clipper.ItemsCount);
            data.StepNo = 1;
            return true;
        }
        calc_clipping = true;   // If on the first step with known item height, calculate clipping.
    }

// Step 1: Let the clipper infer height from first range
    if clipper.ItemsHeight <= 0.0 {
// IM_ASSERT(data->StepNo == 1);
// if (table) {}
// IM_ASSERT(table.RowPosY1 == clipper->StartPosY && table.RowPosY2 == window.DC.CursorPos.y);

        clipper.ItemsHeight = (window.DC.CursorPos.y - clipper.StartPosY) / (clipper.DisplayEnd - clipper.DisplayStart);
        let mut affected_by_floating_point_precision: bool = ImIsFloatAboveGuaranteedIntegerPrecision(clipper.StartPosY) || ImIsFloatAboveGuaranteedIntegerPrecision(window.DC.CursorPos.y);
        if affected_by_floating_point_precision {
            clipper.ItemsHeight = window.DC.PrevLineSize.y + g.Style.ItemSpacing.y; // FIXME: Technically wouldn't allow multi-line entries.
        }
// IM_ASSERT(clipper->ItemsHeight > 0.0 && "Unable to calculate item height! First item hasn't moved the cursor vertically!");
        calc_clipping = true;   // If item height had to be calculated, calculate clipping afterwards.
    }

// Step 0 or 1: Calculate the actual ranges of visible elements.
    let already_submitted: c_int = clipper.DisplayEnd;
    if calc_clipping {
        if g.LogEnabled {
// If logging is active, do not perform any clipping
            data.Ranges.push(ImGuiListClipperRange::FromIndices(0, clipper.ItemsCount));
        } else {
// Add range selected to be included for navigation
            let is_nav_request: bool = (g.NavMoveScoringItems && g.NavWindow.is_null() == false && g.NavWindow.RootWindowForNav == window.RootWindowForNav);
            if is_nav_request {
                data.Ranges.push(ImGuiListClipperRange::FromPositions(g.NavScoringNoClipRect.Min.y, g.NavScoringNoClipRect.Max.y, 0, 0));
            }
            if is_nav_request && (g.NavMoveFlags & ImGuiNavMoveFlags_Tabbing) != 0 && g.NavTabbingDir == -1 {
                data.Ranges.push(ImGuiListClipperRange::FromIndices(clipper.ItemsCount - 1, clipper.ItemsCount));
            }

// Add focused/active item
            let mut nav_rect_abs = WindowRectRelToAbs(window, &window.NavRectRel[0]);
            if g.NavId != 0 && window.NavLastIds[0] == g.NavId {
                data.Ranges.push(ImGuiListClipperRange::FromPositions(nav_rect_abs.Min.y, nav_rect_abs.Max.y, 0, 0));
            }

// Add visible range
            let off_min: c_int = if is_nav_request && g.NavMoveClipDir == ImGuiDir_Up { -1 } else { 0 };
            let off_max: c_int = if is_nav_request && g.NavMoveClipDir == ImGuiDir_Down { 1 } else { 0 };
            data.Ranges.push(ImGuiListClipperRange::FromPositions(window.ClipRect.Min.y, window.ClipRect.Max.y, off_min, off_max));
        }

// Convert position ranges to item index ranges
// - Very important: when a starting position is after our maximum item, we set Min to (ItemsCount - 1). This allows us to handle most forms of wrapping.
// - Due to how Selectable extra padding they tend to be "unaligned" with exact unit in the item list,
//   which with the flooring/ceiling tend to lead to 2 items instead of one being submitted.
// for (i: c_int = 0; i < data.Ranges.Size; i++)
        for i in 0..data.Ranges.len() {
            if data.Ranges[i].PosToIndexConvert {
                let mut m1 = ((data.Ranges[i].Min - window.DC.CursorPos.y - data.LossynessOffset) / clipper.ItemsHeight);
                let mut m2 = (((data.Ranges[i].Max - window.DC.CursorPos.y - data.LossynessOffset) / clipper.ItemsHeight) + 0.9999990f32);
                data.Ranges[i].Min = ImClamp(already_submitted + m1 + data.Ranges[i].PosToIndexOffsetMin, already_submitted, clipper.ItemsCount - 1);
                data.Ranges[i].Max = ImClamp(already_submitted + m2 + data.Ranges[i].PosToIndexOffsetMax, data.Ranges[i].Min + 1, clipper.ItemsCount);
                data.Ranges[i].PosToIndexConvert = false;
            }
        }
        ImGuiListClipper_SortAndFuseRanges(&mut data.Ranges, data.StepNo);
    }

// Step 0+ (if item height is given in advance) or 1+: Display the next range in line.
    if data.StepNo < data.Ranges.Size {
        clipper.DisplayStart = ImMax(data.Ranges[data.StepNo].Min, already_submitted);
        clipper.DisplayEnd = ImMin(data.Ranges[data.StepNo].Max, clipper.ItemsCount);
        if clipper.DisplayStart > already_submitted { //-V1051
            ImGuiListClipper_SeekCursorForItem(clipper, clipper.DisplayStart);
        }
        data.StepNo += 1;
        return true;
    }

// After the last step: Let the clipper validate that we have reached the expected Y position (corresponding to element DisplayEnd),
// Advance the cursor to the end of the list and then returns 'false' to end the loop.
    if clipper.ItemsCount < i32::MAX {
        ImGuiListClipper_SeekCursorForItem(clipper, clipper.ItemsCount);
    }

    return false;
}
