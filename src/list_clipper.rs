use std::ffi::c_void;
use crate::imgui_clipper::ImGuiListClipperData;
use crate::imgui_globals::GImGui;
use crate::imgui_math::ImMaxF32;

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
#[derive(Default,Debug,Clone)]
pub struct ImGuiListClipper
{
    pub DisplayStart: i32,     // First item to display, updated by each call to Step()
    pub DisplayEnd: i32,       // End of items to display (exclusive)
    pub ItemsCount: i32,       // [Internal] Number of items
    pub ItemsHeight: f32,       // [Internal] height of item after a first step and item submission can calculate it
    pub StartPosY: f32,         // [Internal] Cursor position at the time of Begin() or after table frozen rows are all processed
    pub TempData: *mut c_void, // void*           TempData;           // [Internal] Internal data



// #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
//     inline ImGuiListClipper(int items_count, float items_height = -1.0) { memset(this, 0, sizeof(*this)); ItemsCount = -1; Begin(items_count, items_height); } // [removed in 1.79]
// #endif
}

impl ImGuiListClipper {
    pub fn new() -> Self {
        // memset(this, 0, sizeof(*this));
        //     ItemsCount = -1;
        Self {
            ItemsCount: -1,
            ..Default::default()
        }

    }
    // items_count: Use INT_MAX if you don't know how many items you have (in which case the cursor won't be advanced in the final step)
    // items_height: Use -1.0 to be calculated automatically on first step. Otherwise pass in the distance between your items, typically GetTextLineHeightWithSpacing() or GetFrameHeightWithSpacing().
    //  ImGuiListClipper();
    //  ~ImGuiListClipper();
    //  void  Begin(int items_count, float items_height = -1.0);
    pub fn begin(&mut self, items_count: i32, items_height: f32) {

        // ImGuiContext& g = *GImGui;
        let g = GImGui;
        // ImGuiWindow* window = g.current_window;
        let window = g.current_window;

        let table = g.CurrentTable;
    if table.is_null() == false {
        if (table.IsInsideRow){
            TableEndRow(table);
        }
    }
    self.StartPosY = window.dc.cursor_pos.y;
    self.ItemsHeight = items_height;
    self.ItemsCount = items_count;
    self.DisplayStart = -1;
    self.DisplayEnd = 0;

    // Acquire temporary buffer
    if (g.ClipperTempDataStacked += 1 > g.clipper_temp_data.size) {
        g.clipper_temp_data.resize(g.ClipperTempDataStacked as usize, ImGuiListClipperData());
    }
    // ImGuiListClipperData* data = &g.clipper_temp_data[g.clipper_temp_data_stacked - 1];
    let data = &g.clipper_temp_data[g.ClipperTempDataStacked - 1];
        data.Reset(this);
    data.LossynessOffset = window.dc.cursor_start_posLossyness.y;
    self.TempData = data as *mut c_void;
    }
    //  void  End();             // Automatically called on the last call of Step() that returns false.
    pub fn end(&mut self) {

        ImGuiContext& g = *GImGui;
    if (ImGuiListClipperData* data = (ImGuiListClipperData*)TempData)
    {
        // In theory here we should assert that we are already at the right position, but it seems saner to just seek at the end and not assert/crash the user.
        if (ItemsCount >= 0 && ItemsCount < INT_MAX && DisplayStart >= 0)
            ImGuiListClipper_SeekCursorForItem(this, ItemsCount);

        // Restore temporary buffer and fix back pointers which may be invalidated when nesting
        IM_ASSERT(data.ListClipper == this);
        data.StepNo = data.Ranges.size;
        if (g.ClipperTempDataStacked -= 1 > 0)
        {
            data = &g.clipper_temp_data[g.ClipperTempDataStacked - 1];
            data.ListClipper.TempData = data;
        }
        TempData = NULL;
    }
    ItemsCount = -1;


    }
    //  bool  Step();            // Call until it returns false. The DisplayStart/DisplayEnd fields will be set and you can process/draw those items.
    pub fn Step(&mut self) -> bool {
        todo!()
    }
    // Call ForceDisplayRangeByIndices() before first call to Step() if you need a range of items to be displayed regardless of visibility.
    //  void  ForceDisplayRangeByIndices(int item_min, int item_max); // item_max is exclusive e.g. use (42, 42+1) to make item 42 always visible BUT due to alignment/padding of certain items it is likely that an extra item may be included on either end of the display range.
    pub fn ForceDisplayRangeByIndices(&mut self, item_min: i32, item_max: i32) {
        todo!()
    }
}



/// FIXME-TABLE: This prevents us from using ImGuiListClipper _inside_ a table cell.
/// The problem we have is that without a Begin/End scheme for rows using the clipper is ambiguous.
// static bool GetSkipItemForListClipping()
pub fn GetSkipItemForListClipping() -> bool
{
    // ImGuiContext& g = *GImGui;
    // return (g.current_table ? g.current_table->HostSkipItems : g.current_window->skip_items);
    if GImGui.CurrentTable {
        GImGui.CurrentTable.HostSkipItems
    } else {
        GImGui.current_window.skip_items
    }

}



//static void ImGuiListClipper_SortAndFuseRanges(ImVector<ImGuiListClipperRange>& ranges, int offset = 0)
pub fn sort_and_fuse_ranges(ranges: &mut Vec<ImGuiListClipper>, offset: usize)
{
    if (ranges.size - offset <= 1) {
        return;
    }

    // Helper to order ranges and fuse them together if possible (bubble sort is fine as we are only sorting 2-3 entries)
    // for (int sort_end = ranges.size - offset - 1; sort_end > 0; sort_end -= 1){
    let mut sort_end = ranges.len() - offset -1;
   while (sort_end > 0) {
        // for (int i = offset; i < sort_end + offset; i += 1){
        for i in offset .. (sort_end + offset) {
            if (ranges[i].min > ranges[i + 1].min) {
                ImSwap(ranges[i], ranges[i + 1]);
            }
        }
        sort_end -= 1;
    }

    // Now fuse ranges together as much as possible.
    // for (int i = 1 + offset; i < ranges.size; i += 1)
    for i in (1 + offset) .. ranges.size
    {
        // IM_ASSERT(!ranges[i].PosToIndexConvert && !ranges[i - 1].PosToIndexConvert);
        if ranges[i - 1].max < ranges[i].min {
            continue;
        }
        ranges[i - 1].min = ImMin(ranges[i - 1].min, ranges[i].min);
        ranges[i - 1].max = ImMax(ranges[i - 1].max, ranges[i].max);
        ranges.erase(ranges.data + i);
        // i -= 1;
    }
}

// static void ImGuiListClipper_SeekCursorAndSetupPrevLine(float pos_y, float line_height)
pub fn seek_cursor_and_setup_prev_line(pos_y: f32, line_height: f32)
{
    // Set cursor position and a few other things so that SetScrollHereY() and Columns() can work when seeking cursor.
    // FIXME: It is problematic that we have to do that here, because custom/equivalent end-user code would stumble on the same issue.
    // The clipper should probably have a final step to display the last item in a regular manner, maybe with an opt-out flag for data sets which may have costly seek?
    // ImGuiContext& g = *GImGui;
    // ImGuiWindow* window = g.current_window;
    let window = GImGui.current_window;
    // float off_y = pos_y - window->dc.cursor_pos.y;
    let off_y = pos_y - window.dc.cursor_pos.y;
    // window->dc.cursor_pos.y = pos_y;
    *window.dc.cursor_pos.y = pos_y;
    // window->dc.CursorMaxPos.y = ImMax(window->dc.CursorMaxPos.y, pos_y - g.style.ItemSpacing.y);
    *window.dc.cursor_max_pos.y = ImMaxF32(window.dc.cursor_max_pos.y, pos_y - GImGui.style.ItemSpacing.y);
    // window->dc.CursorPosPrevLine.y = window->dc.cursor_pos.y - line_height;  // Setting those fields so that SetScrollHereY() can properly function after the end of our clipper usage.
    *window.dc.CursorPosPrevLine.y = window.dc.cursor_pos.y - line_height;
    // window->dc.PrevLineSize.y = (line_height - g.style.ItemSpacing.y);      // If we end up needing more accurate data (to e.g. use SameLine) we may as well make the clipper have a fourth step to let user process and display the last item in their list.
    *window.dc.PrevLineSize.y = (line_height - GImGui.style.ItemSpacing.y);
    let columns = window.dc.CurrentColumns;
    if (columns.is_null() == false) {
        columns.LineMinY = window.dc.cursor_pos.y;
    }                         // Setting this so that cell Y position are set properly
    // if (ImGuiTable* table = g.current_table)
    let table = GImGui.CurrentTable;
    if table.is_null() == false
    {
        if (table.IsInsideRow){
            TableEndRow(table);
    }
        table.RowPosY2 = window.dc.cursor_pos.y;
        // const int row_increase = ((off_y / line_height) + 0.5);
        let row_increate = (off_y / line_height) + 0.5;
        //table->CurrentRow += row_increase; // Can't do without fixing TableEndRow()
        table.RowBgColorCounter += row_increase;
    }
}

// static void ImGuiListClipper_SeekCursorForItem(ImGuiListClipper* clipper, int item_n)
pub fn seek_cursor_for_item(clipper: *mut ImGuiListClipper, item_n: usize)
{
    // StartPosY starts from ItemsFrozen hence the subtraction
    // Perform the add and multiply with double to allow seeking through larger ranges
    // ImGuiListClipperData* data = (ImGuiListClipperData*)clipper->TempData;
    let mut data = clipper.TempData as *mut ImGuiListClipperData;
    // float pos_y = (float)((double)clipper->StartPosY + data->LossynessOffset + (double)(item_n - data->ItemsFrozen) * clipper->ItemsHeight);
    let pos_y = clipper.StartPosY + data.LossynessOffset + (item_n - data.ItemsFrozen) * clipper.ItemsHeight;
    // ImGuiListClipper_SeekCursorAndSetupPrevLine(pos_y, clipper->ItemsHeight);
    seek_cursor_and_setup_prev_line(pos_y, clipper.ItemsHeight);
}

// ImGuiListClipper::ImGuiListClipper()
// {
//
// }

// ImGuiListClipper::~ImGuiListClipper()
// {
//     End();
// }

// Use case A: Begin() called from constructor with items_height<0, then called again from Step() in StepNo 1
// Use case B: Begin() called from constructor with items_height>0
// FIXME-LEGACY: Ideally we should remove the Begin/End functions but they are part of the legacy API we still support. This is why some of the code in Step() calling Begin() and reassign some fields, spaghetti style.
// void ImGuiListClipper::Begin(int items_count, float items_height)
// {
//
// }

// void ImGuiListClipper::End()
// {
//
// }

void ImGuiListClipper::ForceDisplayRangeByIndices(int item_min, int item_max)
{
    ImGuiListClipperData* data = (ImGuiListClipperData*)TempData;
    IM_ASSERT(DisplayStart < 0); // Only allowed after Begin() and if there has not been a specified range yet.
    IM_ASSERT(item_min <= item_max);
    if (item_min < item_max)
        data.Ranges.push_back(ImGuiListClipperRange::FromIndices(item_min, item_max));
}

bool ImGuiListClipper::Step()
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    ImGuiListClipperData* data = (ImGuiListClipperData*)TempData;
    IM_ASSERT(data != NULL && "Called ImGuiListClipper::Step() too many times, or before ImGuiListClipper::Begin() ?");

    ImGuiTable* table = g.CurrentTable;
    if (table && table.IsInsideRow)
        ImGui::TableEndRow(table);

    // No items
    if (ItemsCount == 0 || GetSkipItemForListClipping())
        return (void)end(), false;

    // While we are in frozen row state, keep displaying items one by one, unclipped
    // FIXME: Could be stored as a table-agnostic state.
    if (data.StepNo == 0 && table != NULL && !table.IsUnfrozenRows)
    {
        DisplayStart = data.ItemsFrozen;
        DisplayEnd = data.ItemsFrozen + 1;
        if (DisplayStart >= ItemsCount)
            return (void)end(), false;
        data.ItemsFrozen += 1;
        return true;
    }

    // Step 0: Let you process the first element (regardless of it being visible or not, so we can measure the element height)
    bool calc_clipping = false;
    if (data.StepNo == 0)
    {
        StartPosY = window.dc.cursor_pos.y;
        if (ItemsHeight <= 0.0)
        {
            // Submit the first item (or range) so we can measure its height (generally the first range is 0..1)
            data.Ranges.push_front(ImGuiListClipperRange::FromIndices(data.ItemsFrozen, data.ItemsFrozen + 1));
            DisplayStart = ImMax(data.Ranges[0].min, data.ItemsFrozen);
            DisplayEnd = ImMin(data.Ranges[0].max, ItemsCount);
            if (DisplayStart == DisplayEnd)
                return (void)end(), false;
            data.StepNo = 1;
            return true;
        }
        calc_clipping = true;   // If on the first step with known item height, calculate clipping.
    }

    // Step 1: Let the clipper infer height from first range
    if (ItemsHeight <= 0.0)
    {
        IM_ASSERT(data.StepNo == 1);
        if (table)
            IM_ASSERT(table.RowPosY1 == StartPosY && table.RowPosY2 == window.dc.cursor_pos.y);

        ItemsHeight = (window.dc.cursor_pos.y - StartPosY) / (DisplayEnd - DisplayStart);
        bool affected_by_floating_point_precision = ImIsFloatAboveGuaranteedIntegerPrecision(StartPosY) || ImIsFloatAboveGuaranteedIntegerPrecision(window.dc.cursor_pos.y);
        if (affected_by_floating_point_precision)
            ItemsHeight = window.dc.PrevLineSize.y + g.style.ItemSpacing.y; // FIXME: Technically wouldn't allow multi-line entries.

        IM_ASSERT(ItemsHeight > 0.0 && "Unable to calculate item height! First item hasn't moved the cursor vertically!");
        calc_clipping = true;   // If item height had to be calculated, calculate clipping afterwards.
    }

    // Step 0 or 1: Calculate the actual ranges of visible elements.
    const int already_submitted = DisplayEnd;
    if (calc_clipping)
    {
        if (g.LogEnabled)
        {
            // If logging is active, do not perform any clipping
            data.Ranges.push_back(ImGuiListClipperRange::FromIndices(0, ItemsCount));
        }
        else
        {
            // Add range selected to be included for navigation
            const bool is_nav_request = (g.NavMoveScoringItems && g.nav_window && g.nav_window.root_window_for_nav == window.root_window_for_nav);
            if (is_nav_request)
                data.Ranges.push_back(ImGuiListClipperRange::FromPositions(g.NavScoringNoClipRect.min.y, g.NavScoringNoClipRect.max.y, 0, 0));
            if (is_nav_request && (g.NavMoveFlags & ImGuiNavMoveFlags_Tabbing) && g.NavTabbingDir == -1)
                data.Ranges.push_back(ImGuiListClipperRange::FromIndices(ItemsCount - 1, ItemsCount));

            // Add focused/active item
            Rect nav_rect_abs = ImGui::WindowRectRelToAbs(window, window.NavRectRel[0]);
            if (g.nav_id != 0 && window.NavLastIds[0] == g.nav_id)
                data.Ranges.push_back(ImGuiListClipperRange::FromPositions(nav_rect_abs.min.y, nav_rect_abs.max.y, 0, 0));

            // Add visible range
            const int off_min = (is_nav_request && g.NavMoveClipDir == Dir::Up) ? -1 : 0;
            const int off_max = (is_nav_request && g.NavMoveClipDir == Dir::Down) ? 1 : 0;
            data.Ranges.push_back(ImGuiListClipperRange::FromPositions(window.clip_rect.min.y, window.clip_rect.max.y, off_min, off_max));
        }

        // Convert position ranges to item index ranges
        // - Very important: when a starting position is after our maximum item, we set min to (ItemsCount - 1). This allows us to handle most forms of wrapping.
        // - Due to how Selectable extra padding they tend to be "unaligned" with exact unit in the item list,
        //   which with the flooring/ceiling tend to lead to 2 items instead of one being submitted.
        for (int i = 0; i < data.Ranges.size; i += 1)
            if (data.Ranges[i].PosToIndexConvert)
            {
                int m1 = (((double)data.Ranges[i].min - window.dc.cursor_pos.y - data.LossynessOffset) / ItemsHeight);
                int m2 = ((((double)data.Ranges[i].max - window.dc.cursor_pos.y - data.LossynessOffset) / ItemsHeight) + 0.999999);
                data.Ranges[i].min = ImClamp(already_submitted + m1 + data.Ranges[i].PosToIndexOffsetMin, already_submitted, ItemsCount - 1);
                data.Ranges[i].max = ImClamp(already_submitted + m2 + data.Ranges[i].PosToIndexOffsetMax, data.Ranges[i].min + 1, ItemsCount);
                data.Ranges[i].PosToIndexConvert = false;
            }
        ImGuiListClipper_SortAndFuseRanges(data.Ranges, data.StepNo);
    }

    // Step 0+ (if item height is given in advance) or 1+: Display the next range in line.
    if (data.StepNo < data.Ranges.size)
    {
        DisplayStart = ImMax(data.Ranges[data.StepNo].min, already_submitted);
        DisplayEnd = ImMin(data.Ranges[data.StepNo].max, ItemsCount);
        if (DisplayStart > already_submitted) //-V1051
            ImGuiListClipper_SeekCursorForItem(this, DisplayStart);
        data.StepNo += 1;
        return true;
    }

    // After the last step: Let the clipper validate that we have reached the expected Y position (corresponding to element DisplayEnd),
    // Advance the cursor to the end of the list and then returns 'false' to end the loop.
    if (ItemsCount < INT_MAX)
        ImGuiListClipper_SeekCursorForItem(this, ItemsCount);

    end();
    return false;
}
