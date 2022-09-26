#![allow(non_snake_case)]
//-----------------------------------------------------------------------------
// [SECTION] ImGuiListClipper
// This is currently not as flexible/powerful as it should be and really confusing/spaghetti, mostly because we changed
// the API mid-way through development and support two ways to using the clipper, needs some rework (see TODO)
//-----------------------------------------------------------------------------

use libc::{c_float, c_int, c_void};
use crate::imgui_cpp::GImGui;

// FIXME-TABLE: This prevents us from using ImGuiListClipper _inside_ a table cell.
// The problem we have is that without a Begin/End scheme for rows using the clipper is ambiguous.
// static bool GetSkipItemForListClipping()
pub fn GetSkipItemForListClipping()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    //return (g.CurrentTable ? g.CurrentTable->HostSkipItems : g.Currentwindow.SkipItems);
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
// void ImGui::CalcListClipping(int items_count, float items_height, int* out_items_display_start, int* out_items_display_end)
pub unsafe fn CalcListClipping(items_count: usize, items_height: c_float, out_items_display_start: *mut c_int, out_items_display_end: *mut c_int)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let window = g.CurrentWindow;
    if g.LogEnabled
    {
        // If logging is active, do not perform any clipping
        *out_items_display_start = 0;
        *out_items_display_end = items_count as c_int;
        return;
    }
    if GetSkipItemForListClipping()
    {
        *out_items_display_start = 0;
        *out_items_display_end = 0;
        return;
    }

    // We create the union of the ClipRect and the scoring rect which at worst should be 1 page away from ClipRect
    // We don't include g.NavId's rectangle in there (unless g.NavJustMovedToId is set) because the rectangle enlargement can get costly.
    let rect = window.ClipRect;
    if g.NavMoveScoringItems {
        rect.Add(g.NavScoringNoClipRect);
    }
    if g.NavJustMovedToId != 0 && (window.NavLastIds[0] == g.NavJustMovedToId) {
        rect.Add(WindowRectRelToAbs(window, window.NavRectRel[0])); // Could store and use NavJustMovedToRectRel
    }
    let pos = window.DC.CursorPos;
    let start = ((rect.Min.y - pos.y) / items_height);
    let end = ((rect.Max.y - pos.y) / items_height);

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
    *out_items_display_start = start;
    *out_items_display_end = end;
}
// #endif

// static void ImGuiListClipper_SortAndFuseRanges(ImVector<ImGuiListClipperRange>& ranges, int offset = 0)
pub fn ImGuiListClipper_SortAndFuseRanges(ranges: &mut Vec<ImGuiListClipperRange>, offset: usize) {
    if ranges.Size - offset <= 1 {
        return;
    }

// Helper to order ranges and fuse them together if possible (bubble sort is fine as we are only sorting 2-3 entries)
//     for (int sort_end = ranges.Size - offset -1; sort_end > 0; - - sort_end)
    for sort_end in ranges.len() - offset - 1.. 0
    {
        // for (int i = offset; i < sort_end + offset; + + i)
        for i in offset .. sort_end + offset
        {
            if ranges[i].Min > ranges[i + 1].Min {
                ImSwap(ranges[i], ranges[i + 1]);
            }
        }
    }

// Now fuse ranges together as much as possible.
//     for (int i = 1 + offset; i < ranges.Size; i+ +)
    for mut i in 1 + offset .. ranges.len()
    {

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

// static void ImGuiListClipper_SeekCursorAndSetupPrevLine(float pos_y, float line_height)
pub fn ImGuiListClipper_SeekCursor
{
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
    
    let columns = window.DC.CurrentColumns
    // if (ImGuiOldColumns* columns = window.DC.CurrentColumns) 
    
    {
        columns -> LineMinY = window.DC.CursorPos.y;
    }        // Setting this so that cell Y position are set properly
    if (*mut ImGuiTable table = g.CurrentTable)
    {
    if (table->IsInsideRow){
        ImGui::TableEndRow(table);
    }
    table->RowPosY2 = window.DC.CursorPos.y;
    const c_int row_increase = ((off_y / line_height) + 0.5f32);
    //table->CurrentRow += row_increase; // Can't do without fixing TableEndRow()
    table->RowBgColorCounter += row_increase;
    }
}

static c_void ImGuiListClipper_SeekCursorForItem(*mut ImGuiListClipper clipper, c_int item_n)
{
// StartPosY starts from ItemsFrozen hence the subtraction
// Perform the add and multiply with double to allow seeking through larger ranges
*mut ImGuiListClipperData data = (*mut ImGuiListClipperData)clipper->TempData;
c_float pos_y = ((double)clipper->StartPosY + data->LossynessOffset + (double)(item_n - data->ItemsFrozen) * clipper->ItemsHeight);
ImGuiListClipper_SeekCursorAndSetupPrevLine(pos_y, clipper->ItemsHeight);
}



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
//           ImGui::Text("line number %d", i);
// Generally what happens is:
// - Clipper lets you process the first element (DisplayStart = 0, DisplayEnd = 1) regardless of it being visible or not.
// - User code submit that one element.
// - Clipper can measure the height of the first element
// - Clipper calculate the actual range of elements to display based on the current clipping rectangle, position the cursor before the first visible element.
// - User code submit visible elements.
// - The clipper also handles various subtleties related to keyboard/gamepad navigation, wrapping etc.
#[derive(Debug,Clone,Default)]
pub struct ImGuiListClipper
{
    // int             DisplayStart;       // First item to display, updated by each call to Step()
    pub DisplayStart: i32,

    // int             DisplayEnd;         // End of items to display (exclusive)
    pub DisplayEnd: i32,

    // int             ItemsCount;         // [Internal] Number of items
    pub ItemsCount: i32,

    // float           ItemsHeight;        // [Internal] Height of item after a first step and item submission can calculate it
    pub ItemsHeight: f32,

    // float           StartPosY;          // [Internal] Cursor position at the time of Begin() or after table frozen rows are all processed
    pub StartPosY: f32,

    // void*           TempData;           // [Internal] Internal data
    pub TempData: *const c_void,

    // items_count: Use INT_MAX if you don't know how many items you have (in which case the cursor won't be advanced in the final step)
    // items_height: Use -1f32 to be calculated automatically on first step. Otherwise pass in the distance between your items, typically GetTextLineHeightWithSpacing() or GetFrameHeightWithSpacing().

}

impl ImGuiListClipper {
    // IMGUI_API ImGuiListClipper();
    pub fn new () -> Self {
        Self {
            ..Default::default()
        }
    }

    // IMGUI_API ~ImGuiListClipper();

    // IMGUI_API void  Begin(int items_count, float items_height = -1f32);
    pub fn Begin(&mut self, items_count: i32, items_height: f32) {
        let g = GImGui; // ImGuiContext& g = *GImGui;
        let mut window = g.CurrentWindow;
        // IMGUI_DEBUG_LOG_CLIPPER("Clipper: Begin(%d,%.20f32) in '%s'\n", items_count, items_height, window.Name);

        let table = g.CurrentTable;
        if table.is_null() == false {
            if table.IsInsideRow {
                TableEndRow(table);
            }
        }

        StartPosY = window.DC.CursorPos.y;
        ItemsHeight = items_height;
        ItemsCount = items_count;
        DisplayStart = -1;
        DisplayEnd = 0;

        // Acquire temporary buffer
        if g.ClipperTempDataStacked > g.ClipperTempData.len() {
            g.ClipperTempData.resize(g.ClipperTempDataStacked, ImGuiListClipperData());
        }
        g.ClipperTempDataStacked += 1;
        let data = &g.ClipperTempData[g.ClipperTempDataStacked - 1];
        data.Reset(this);
        data.LossynessOffset = window.DC.CursorStartPosLossyness.y;
        TempData = data;
    }


    // IMGUI_API void  End();             // Automatically called on the last call of Step() that returns false.

    // IMGUI_API bool  Step();            // Call until it returns false. The DisplayStart/DisplayEnd fields will be set and you can process/draw those items.

    // Call ForceDisplayRangeByIndices() before first call to Step() if you need a range of items to be displayed regardless of visibility.
    // IMGUI_API void  ForceDisplayRangeByIndices(int item_min, int item_max); // item_max is exclusive e.g. use (42, 42+1) to make item 42 always visible BUT due to alignment/padding of certain items it is likely that an extra item may be included on either end of the display range.

//     // #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
//     inline ImGuiListClipper(int items_count, float items_height = -1f32) { memset(this, 0, sizeof(*this)); ItemsCount = -1; Begin(items_count, items_height); } // [removed in 1.79]
// // #endif
}
