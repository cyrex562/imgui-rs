#![allow(non_snake_case)]

use std::ptr::null_mut;
use libc::{c_float, c_int, c_void};
use crate::imgui::GImGui;
use crate::imgui_cpp::GImGui;
use crate::list_clipper_data::ImGuiListClipperData;
use crate::list_clipper_ops::ImGuiListClipper_StepInternal;
use crate::list_clipper_range::ImGuiListClipperRange;
use crate::table_ops::TableEndRow;

// Helper: Manually clip large list of items.
// If you have lots evenly spaced items and you have a random access to the list, you can perform coarse
// clipping based on visibility to only submit items that are in view.
// The clipper calculates the range of visible items and advance the cursor to compensate for the non-visible items we have skipped.
// (Dear ImGui already clip items based on their bounds but: it needs to first layout the item to do so, and generally
//  fetching/submitting your own data incurs additional cost. Coarse clipping using ImGuiListClipper allows you to easily
//  scale using lists with tens of thousands of items without a problem)
// Usage:
//   ImGuiListClipper clipper;
//   clipper.Begin(1000);         // We have 1000 elements, evenly spaced.
//   while (clipper.Step())
//       for (int i = clipper.DisplayStart; i < clipper.DisplayEnd; i++)
//           Text("line number %d", i);
// Generally what happens is:
// - Clipper lets you process the first element (DisplayStart = 0, DisplayEnd = 1) regardless of it being visible or not.
// - User code submit that one element.
// - Clipper can measure the height of the first element
// - Clipper calculate the actual range of elements to display based on the current clipping rectangle, position the cursor before the first visible element.
// - User code submit visible elements.
// - The clipper also handles various subtleties related to keyboard/gamepad navigation, wrapping etc.
#[derive(Default, Debug, Clone)]
pub struct ImGuiListClipper {
    pub DisplayStart: c_int,
    // First item to display, updated by each call to Step()
    pub DisplayEnd: c_int,
    // End of items to display (exclusive)
    pub ItemsCount: c_int,
    // [Internal] Number of items
    pub ItemsHeight: c_float,
    // [Internal] Height of item after a first step and item submission can calculate it
    pub StartPosY: c_float,
    // [Internal] Cursor position at the time of Begin() or after table frozen rows are all processed
    pub TempData: *mut c_void,           // [Internal] Internal data
}

impl ImGuiListClipper {
    // items_count: Use INT_MAX if you don't know how many items you have (in which case the cursor won't be advanced in the final step)
    // items_height: Use -1f32 to be calculated automatically on first step. Otherwise pass in the distance between your items, typically GetTextLineHeightWithSpacing() or GetFrameHeightWithSpacing().
    // ImGuiListClipper();

    // ~ImGuiListClipper();


    // c_void  Begin(c_int items_count, c_float items_height = -1f32);
    // IMGUI_API void  Begin(int items_count, float items_height = -1f32);
    pub unsafe fn Begin(&mut self, items_count: i32, items_height: f32) {
        let g = GImGui; // ImGuiContext& g = *GImGui;
        let mut window = g.CurrentWindow;
        // IMGUI_DEBUG_LOG_CLIPPER("Clipper: Begin(%d,%.20f32) in '%s'\n", items_count, items_height, window.Name);

        let table = g.CurrentTable;
        if table.is_null() == false {
            if table.IsInsideRow {
                TableEndRow(table);
            }
        }

        self.StartPosY = window.DC.CursorPos.y;
        self.ItemsHeight = items_height;
        self.ItemsCount = items_count;
        self.DisplayStart = -1;
        self.DisplayEnd = 0;

        // Acquire temporary buffer
        if g.ClipperTempDataStacked > g.ClipperTempData.len() {
            g.ClipperTempData.resize(g.ClipperTempDataStacked, ImGuiListClipperData::default());
        }
        g.ClipperTempDataStacked += 1;
        let mut data = &mut g.ClipperTempData[g.ClipperTempDataStacked - 1];
        data.Reset(this);
        data.LossynessOffset = window.DC.CursorStartPosLossyness.y;
        self.TempData = data;
    }

    // c_void  End();             // Automatically called on the last call of Step() that returns false.
    pub unsafe fn End(&mut self) {
        let g = GImGui; // ImGuiContext& g = *GImGui;
        let mut data = self.TempData as *mut ImGuiListClipperData;
        if data.is_null() == false {
            // In theory here we should assert that we are already at the right position, but it seems saner to just seek at the end and not assert/crash the user.
            // IMGUI_DEBUG_LOG_CLIPPER("Clipper: End() in '%s'\n", g.Currentwindow.Name);
            if self.ItemsCount >= 0 && self.ItemsCount < i32::MAX && self.DisplayStart >= 0 {
                ImGuiListClipper_SeekCursorForItem(self, self.ItemsCount);
            }

            // Restore temporary buffer and fix back pointers which may be invalidated when nesting
            // IM_ASSERT(data->ListClipper == this);
            data.StepNo = data.Ranges.len();
            g.ClipperTempDataStacked -= 1;
            if g.ClipperTempDataStacked > 0 {
                data = &mut g.ClipperTempData[g.ClipperTempDataStacked - 1];
                data.ListClipper.TempData = data;
            }

            self.TempData = null_mut();
        }
        self.ItemsCount = -1;
    }

    // bool  Step();            // Call until it returns false. The DisplayStart/DisplayEnd fields will be set and you can process/draw those items.
    // bool ImGuiListClipper::Step()
    pub unsafe fn Step(&mut self) -> bool {
        let g = GImGui; // ImGuiContext& g = *GImGui;
        let mut need_items_height: bool = (self.ItemsHeight <= 0f32);
        let mut ret: bool = ImGuiListClipper_StepInternal(this);
        if ret && (self.DisplayStart == self.DisplayEnd) {
            ret = false;
        }
        if g.CurrentTable.is_null() == false && g.Currenttable.IsUnfrozenRows == false {
            IMGUI_DEBUG_LOG_CLIPPER("Clipper: Step(): inside frozen table row.\n");
        }
        if need_items_height && self.ItemsHeight > 0f32 {
            IMGUI_DEBUG_LOG_CLIPPER("Clipper: Step(): computed ItemsHeight: %.2f.\n", self.ItemsHeight);
        }
        if ret {
            IMGUI_DEBUG_LOG_CLIPPER("Clipper: Step(): display %d to %d.\n", self.DisplayStart, self.DisplayEnd);
        } else {
            IMGUI_DEBUG_LOG_CLIPPER("Clipper: Step(): End.\n");
        }
        if !ret {
            self.End();
        }
        return ret;
    }

    // Call ForceDisplayRangeByIndices() before first call to Step() if you need a range of items to be displayed regardless of visibility.
    // c_void  ForceDisplayRangeByIndices(c_int item_min, c_int item_max); // item_max is exclusive e.g. use (42, 42+1) to make item 42 always visible BUT due to alignment/padding of certain items it is likely that an extra item may be included on either end of the display range.
    pub fn ForceDisplayRangeByIndices(&mut self, item_min: c_int, item_max: c_int) {
        let mut data = self.TempData;
        // IM_ASSERT(DisplayStart < 0); // Only allowed after Begin() and if there has not been a specified range yet.
        // IM_ASSERT(item_min <= item_max);
        if item_min < item_max {
            data.Ranges.push(ImGuiListClipperRange::FromIndices(item_min, item_max));
        }
    }


    // #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
    // inline ImGuiListClipper(c_int items_count, c_float items_height = -1f32) { memset(this, 0, sizeof(*this)); ItemsCount = -1; Begin(items_count, items_height); } // [removed in 1.79]
// #endif
}

// static c_void ImGuiListClipper_SeekCursorForItem(*mut ImGuiListClipper clipper, c_int item_n)
pub unsafe fn ImGuiListClipper_SeekCursorForItem(clipper: *mut ImGuiListClipper, item_n: c_int) {
// StartPosY starts from ItemsFrozen hence the subtraction
// Perform the add and multiply with double to allow seeking through larger ranges
// *mut ImGuiListClipperData data = (*mut ImGuiListClipperData)clipper->TempData;
    let mut data = clipper.TempData as *mut ImGuiListClipperData;

    let mut pos_y = (clipper.StartPosY + data.LossynessOffset + (item_n - data.ItemsFrozen) * clipper.ItemsHeight);

    ImGuiListClipper_SeekCursorAndSetupPrevLine(pos_y, clipper.ItemsHeight);
}

// static void ImGuiListClipper_SeekCursorAndSetupPrevLine(float pos_y, float line_height)
pub unsafe fn ImGuiListClipper_SeekCursorAndSetupPrevLine(pos_y: c_float, line_height: c_float) {
    // Set cursor position and a few other things so that SetScrollHereY() and Columns() can work when seeking cursor.
    // FIXME: It is problematic that we have to do that here, because custom/equivalent end-user code would stumble on the same issue.
    // The clipper should probably have a final step to display the last item in a regular manner, maybe with an opt-out flag for data sets which may have costly seek?
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
    let off_y = pos_y - window.DC.CursorPos.y;
    window.DC.CursorPos.y = pos_y;
    window.DC.CursorMaxPos.y = ImMax(window.DC.CursorMaxPos.y, pos_y - g.Style.ItemSpacing.y);
    window.DC.CursorPosPrevLine.y = window.DC.CursorPos.y - line_height;  // Setting those fields so that SetScrollHereY() can properly function after the end of our clipper usage.
    window.DC.PrevLineSize.y = (line_height - g.Style.ItemSpacing.y);      // If we end up needing more accurate data (to e.g. use SameLine) we may as well make the clipper have a fourth step to let user process and display the last item in their list.

    let columns = window.DC.CurrentColumns;
    // if (ImGuiOldColumns* columns = window.DC.CurrentColumns)
    if columns.is_null() == false {
        columns.LineMinY = window.DC.CursorPos.y;
    }        // Setting this so that cell Y position are set properly
    let mut table = g.CurrentTable;
    if table.is_null() == false {
        if table.IsInsideRow() {
            TableEndRow(table);
        }
        table.RowPosY2 = window.DC.CursorPos.y;
        let row_increase = ((off_y / line_height) + 0.5f32);
        //table.CurrentRow += row_increase; // Can't do without fixing TableEndRow()
        table.RowBgColorCounter += row_increase;
    }
}
