// dear imgui, v1.89 WIP
// (tables and columns code)

/*

Index of this file:

// [SECTION] Commentary
// [SECTION] Header mess
// [SECTION] Tables: Main code
// [SECTION] Tables: Simple accessors
// [SECTION] Tables: Row changes
// [SECTION] Tables: Columns changes
// [SECTION] Tables: Columns width management
// [SECTION] Tables: Drawing
// [SECTION] Tables: Sorting
// [SECTION] Tables: Headers
// [SECTION] Tables: Context Menu
// [SECTION] Tables: Settings (.ini data)
// [SECTION] Tables: Garbage Collection
// [SECTION] Tables: Debugging
// [SECTION] Columns, BeginColumns, EndColumns, etc.

*/

// Navigating this file:
// - In Visual Studio IDE: CTRL+comma ("Edit.GoToAll") can follow symbols in comments, whereas CTRL+F12 ("Edit.GoToImplementation") cannot.
// - With Visual Assist installed: ALT+G ("VAssistX.GoToImplementation") can also follow symbols in comments.

//-----------------------------------------------------------------------------
// [SECTION] Commentary
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// Typical tables call flow: (root level is generally public API):
//-----------------------------------------------------------------------------
// - BeginTable()                               user begin into a table
//    | BeginChild()                            - (if ScrollX/ScrollY is set)
//    | TableBeginInitMemory()                  - first time table is used
//    | TableResetSettings()                    - on settings reset
//    | TableLoadSettings()                     - on settings load
//    | TableBeginApplyRequests()               - apply queued resizing/reordering/hiding requests
//    | - TableSetColumnWidth()                 - apply resizing width (for mouse resize, often requested by previous frame)
//    |    - TableUpdateColumnsWeightFromWidth()- recompute columns weights (of stretch columns) from their respective width
// - TableSetupColumn()                         user submit columns details (optional)
// - TableSetupScrollFreeze()                   user submit scroll freeze information (optional)
//-----------------------------------------------------------------------------
// - TableUpdateLayout() [Internal]             followup to BeginTable(): setup everything: widths, columns positions, clipping rectangles. Automatically called by the FIRST call to TableNextRow() or TableHeadersRow().
//    | TableSetupDrawChannels()                - setup ImDrawList channels
//    | TableUpdateBorders()                    - detect hovering columns for resize, ahead of contents submission
//    | TableDrawContextMenu()                  - draw right-click context menu
//-----------------------------------------------------------------------------
// - TableHeadersRow() or TableHeader()         user submit a headers row (optional)
//    | TableSortSpecsClickColumn()             - when left-clicked: alter sort order and sort direction
//    | TableOpenContextMenu()                  - when right-clicked: trigger opening of the default context menu
// - TableGetSortSpecs()                        user queries updated sort specs (optional, generally after submitting headers)
// - TableNextRow()                             user begin into a new row (also automatically called by TableHeadersRow())
//    | TableEndRow()                           - finish existing row
//    | TableBeginRow()                         - add a new row
// - TableSetColumnIndex() / TableNextColumn()  user begin into a cell
//    | TableEndCell()                          - close existing column/cell
//    | TableBeginCell()                        - enter into current column/cell
// - [...]                                      user emit contents
//-----------------------------------------------------------------------------
// - EndTable()                                 user ends the table
//    | TableDrawBorders()                      - draw outer borders, inner vertical borders
//    | TableMergeDrawChannels()                - merge draw channels if clipping isn't required
//    | EndChild()                              - (if ScrollX/ScrollY is set)
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// TABLE SIZING
//-----------------------------------------------------------------------------
// (Read carefully because this is subtle but it does make sense!)
//-----------------------------------------------------------------------------
// About 'outer_size':
// Its meaning needs to differ slightly depending on if we are using ScrollX/ScrollY flags.
// Default value is ImVec2::new(0.0, 0.0).
//   X
//   - outer_size.x <= 0.0  ->  Right-align from window/work-rect right-most edge. With -FLT_MIN or 0.0 will align exactly on right-most edge.
//   - outer_size.x  > 0.0  ->  Set Fixed width.
//   Y with ScrollX/ScrollY disabled: we output table directly in current window
//   - outer_size.y  < 0.0  ->  Bottom-align (but will auto extend, unless _NoHostExtendY is set). Not meaningful is parent window can vertically scroll.
//   - outer_size.y  = 0.0  ->  No minimum height (but will auto extend, unless _NoHostExtendY is set)
//   - outer_size.y  > 0.0  ->  Set Minimum height (but will auto extend, unless _NoHostExtenY is set)
//   Y with ScrollX/ScrollY enabled: using a child window for scrolling
//   - outer_size.y  < 0.0  ->  Bottom-align. Not meaningful is parent window can vertically scroll.
//   - outer_size.y  = 0.0  ->  Bottom-align, consistent with BeginChild(). Not recommended unless table is last item in parent window.
//   - outer_size.y  > 0.0  ->  Set Exact height. Recommended when using Scrolling on any axis.
//-----------------------------------------------------------------------------
// Outer size is also affected by the NoHostExtendX/NoHostExtendY flags.
// Important to that note how the two flags have slightly different behaviors!
//   - ImGuiTableFlags_NoHostExtendX -> Make outer width auto-fit to columns (overriding outer_size.x value). Only available when ScrollX/ScrollY are disabled and Stretch columns are not used.
//   - ImGuiTableFlags_NoHostExtendY -> Make outer height stop exactly at outer_size.y (prevent auto-extending table past the limit). Only available when ScrollX/ScrollY is disabled. Data below the limit will be clipped and not visible.
// In theory ImGuiTableFlags_NoHostExtendY could be the default and any non-scrolling tables with outer_size.y != 0.0 would use exact height.
// This would be consistent but perhaps less useful and more confusing (as vertically clipped items are not easily noticeable)
//-----------------------------------------------------------------------------
// About 'inner_width':
//   With ScrollX disabled:
//   - inner_width          ->  *ignored*
//   With ScrollX enabled:
//   - inner_width  < 0.0  ->  *illegal* fit in known width (right align from outer_size.x) <-- weird
//   - inner_width  = 0.0  ->  fit in outer_width: Fixed size columns will take space they need (if avail, otherwise shrink down), Stretch columns becomes Fixed columns.
//   - inner_width  > 0.0  ->  override scrolling width, generally to be larger than outer_size.x. Fixed column take space they need (if avail, otherwise shrink down), Stretch columns share remaining space!
//-----------------------------------------------------------------------------
// Details:
// - If you want to use Stretch columns with ScrollX, you generally need to specify 'inner_width' otherwise the concept
//   of "available space" doesn't make sense.
// - Even if not really useful, we allow 'inner_width < outer_size.x' for consistency and to facilitate understanding
//   of what the value does.
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// COLUMNS SIZING POLICIES
//-----------------------------------------------------------------------------
// About overriding column sizing policy and width/weight with TableSetupColumn():
// We use a default parameter of 'init_width_or_weight == -1'.
//   - with ImGuiTableColumnFlags_WidthFixed,    init_width  <= 0 (default)  --> width is automatic
//   - with ImGuiTableColumnFlags_WidthFixed,    init_width  >  0 (explicit) --> width is custom
//   - with ImGuiTableColumnFlags_WidthStretch,  init_weight <= 0 (default)  --> weight is 1.0f
//   - with ImGuiTableColumnFlags_WidthStretch,  init_weight >  0 (explicit) --> weight is custom
// Widths are specified _without_ CellPadding. If you specify a width of 100, the column will be cover (100 + Padding * 2.0)
// and you can fit a 100 wide item in it without clipping and with full padding.
//-----------------------------------------------------------------------------
// About default sizing policy (if you don't specify a ImGuiTableColumnFlags_WidthXXXX flag)
//   - with Table policy ImGuiTableFlags_SizingFixedFit      --> default Column policy is ImGuiTableColumnFlags_WidthFixed, default Width is equal to contents width
//   - with Table policy ImGuiTableFlags_SizingFixedSame     --> default Column policy is ImGuiTableColumnFlags_WidthFixed, default Width is max of all contents width
//   - with Table policy ImGuiTableFlags_SizingStretchSame   --> default Column policy is ImGuiTableColumnFlags_WidthStretch, default Weight is 1.0f
//   - with Table policy ImGuiTableFlags_SizingStretchWeight --> default Column policy is ImGuiTableColumnFlags_WidthStretch, default Weight is proportional to contents
// Default Width and default Weight can be overridden when calling TableSetupColumn().
//-----------------------------------------------------------------------------
// About mixing Fixed/Auto and Stretch columns together:
//   - the typical use of mixing sizing policies is: any number of LEADING Fixed columns, followed by one or two TRAILING Stretch columns.
//   - using mixed policies with ScrollX does not make much sense, as using Stretch columns with ScrollX does not make much sense in the first place!
//     that is, unless 'inner_width' is passed to BeginTable() to explicitly provide a total width to layout columns in.
//   - when using ImGuiTableFlags_SizingFixedSame with mixed columns, only the Fixed/Auto columns will match their widths to the width of the maximum contents.
//   - when using ImGuiTableFlags_SizingStretchSame with mixed columns, only the Stretch columns will match their weight/widths.
//-----------------------------------------------------------------------------
// About using column width:
// If a column is manual resizable or has a width specified with TableSetupColumn():
//   - you may use GetContentRegionAvail().x to query the width available in a given column.
//   - right-side alignment features such as SetNextItemWidth(-x) or PushItemWidth(-x) will rely on this width.
// If the column is not resizable and has no width specified with TableSetupColumn():
//   - its width will be automatic and be set to the max of items submitted.
//   - therefore you generally cannot have ALL items of the columns use e.g. SetNextItemWidth(-FLT_MIN).
//   - but if the column has one or more items of known/fixed size, this will become the reference width used by SetNextItemWidth(-FLT_MIN).
//-----------------------------------------------------------------------------


//-----------------------------------------------------------------------------
// TABLES CLIPPING/CULLING
//-----------------------------------------------------------------------------
// About clipping/culling of Rows in Tables:
// - For large numbers of rows, it is recommended you use ImGuiListClipper to only submit visible rows.
//   ImGuiListClipper is reliant on the fact that rows are of equal height.
//   See 'Demo->Tables->Vertical Scrolling' or 'Demo->Tables->Advanced' for a demo of using the clipper.
// - Note that auto-resizing columns don't play well with using the clipper.
//   By default a table with _ScrollX but without _Resizable will have column auto-resize.
//   So, if you want to use the clipper, make sure to either enable _Resizable, either setup columns width explicitly with _WidthFixed.
//-----------------------------------------------------------------------------
// About clipping/culling of Columns in Tables:
// - Both TableSetColumnIndex() and TableNextColumn() return true when the column is visible or performing
//   width measurements. Otherwise, you may skip submitting the contents of a cell/column, BUT ONLY if you know
//   it is not going to contribute to row height.
//   In many situations, you may skip submitting contents for every column but one (e.g. the first one).
// - Case A: column is not hidden by user, and at least partially in sight (most common case).
// - Case B: column is clipped / out of sight (because of scrolling or parent ClipRect): TableNextColumn() return false as a hint but we still allow layout output.
// - Case C: column is hidden explicitly by the user (e.g. via the context menu, or _DefaultHide column flag, etc.).
//
//                        [A]         [B]          [C]
//  TableNextColumn():    true        false        false       -> [userland] when TableNextColumn() / TableSetColumnIndex() return false, user can skip submitting items but only if the column doesn't contribute to row height.
//          SkipItems:    false       false        true        -> [internal] when SkipItems is true, most widgets will early out if submitted, resulting is no layout output.
//           ClipRect:    normal      zero-width   zero-width  -> [internal] when ClipRect is zero, ItemAdd() will return false and most widgets will early out mid-way.
//  ImDrawList output:    normal      dummy        dummy       -> [internal] when using the dummy channel, ImDrawList submissions (if any) will be wasted (because cliprect is zero-width anyway).
//
// - We need to distinguish those cases because non-hidden columns that are clipped outside of scrolling bounds should still contribute their height to the row.
//   However, in the majority of cases, the contribution to row height is the same for all columns, or the tallest cells are known by the programmer.
//-----------------------------------------------------------------------------
// About clipping/culling of whole Tables:
// - Scrolling tables with a known outer size can be clipped earlier as BeginTable() will return false.
//-----------------------------------------------------------------------------

//-----------------------------------------------------------------------------
// [SECTION] Header mess
//-----------------------------------------------------------------------------

// #if defined(_MSC_VER) && !defined(_CRT_SECURE_NO_WARNINGS)
// #define _CRT_SECURE_NO_WARNINGS
// #endif

// #include "imgui.h"
// #ifndef IMGUI_DISABLE

// #ifndef IMGUI_DEFINE_MATH_OPERATORS
// #define IMGUI_DEFINE_MATH_OPERATORS
// #endif
// #include "imgui_internal.h"

// System includes
// #if defined(_MSC_VER) && _MSC_VER <= 1500 // MSVC 2008 or earlier
// #include <stddef.h>     // intptr_t
// #else
// #include <stdint.h>     // intptr_t
// #endif

// Visual Studio warnings
// #ifdef _MSC_VER
// #pragma warning (disable: 4127)     // condition expression is constant
// #pragma warning (disable: 4996)     // 'This function or variable may be unsafe': strcpy, strdup, sprintf, vsnprintf, sscanf, fopen
// #if defined(_MSC_VER) && _MSC_VER >= 1922 // MSVC 2019 16.2 or later
// #pragma warning (disable: 5054)     // operator '|': deprecated between enumerations of different types
// #endif
// #pragma warning (disable: 26451)    // [Static Analyzer] Arithmetic overflow : Using operator 'xxx' on a 4 byte value and then casting the result to a 8 byte value. Cast the value to the wider type before calling operator 'xxx' to avoid overflow(io.2).
// #pragma warning (disable: 26812)    // [Static Analyzer] The enum type 'xxx' is unscoped. Prefer 'enum class' over 'enum' (Enum.3).
// #endif

// Clang/GCC warnings with -Weverything
// #if defined(__clang__)
// #if __has_warning("-Wunknown-warning-option")
// #pragma clang diagnostic ignored "-Wunknown-warning-option"         // warning: unknown warning group 'xxx'                      // not all warnings are known by all Clang versions and they tend to be rename-happy.. so ignoring warnings triggers new warnings on some configuration. Great!
// #endif
// #pragma clang diagnostic ignored "-Wunknown-pragmas"                // warning: unknown warning group 'xxx'
// #pragma clang diagnostic ignored "-Wold-style-cast"                 // warning: use of old-style cast                            // yes, they are more terse.
// #pragma clang diagnostic ignored "-Wfloat-equal"                    // warning: comparing floating point with == or != is unsafe // storing and comparing against same constants (typically 0.0) is ok.
// #pragma clang diagnostic ignored "-Wformat-nonliteral"              // warning: format string is not a string literal            // passing non-literal to vsnformat(). yes, user passing incorrect format strings can crash the code.
// #pragma clang diagnostic ignored "-Wsign-conversion"                // warning: implicit conversion changes signedness
// #pragma clang diagnostic ignored "-Wzero-as-null-pointer-constant"  // warning: zero as null pointer constant                    // some standard header variations use #define NULL 0
// #pragma clang diagnostic ignored "-Wdouble-promotion"               // warning: implicit conversion from 'float' to 'double' when passing argument to function  // using printf() is a misery with this as C++ va_arg ellipsis changes float to double.
// #pragma clang diagnostic ignored "-Wenum-enum-conversion"           // warning: bitwise operation between different enumeration types ('XXXFlags_' and 'XXXFlagsPrivate_')
// #pragma clang diagnostic ignored "-Wdeprecated-enum-enum-conversion"// warning: bitwise operation between different enumeration types ('XXXFlags_' and 'XXXFlagsPrivate_') is deprecated
// #pragma clang diagnostic ignored "-Wimplicit-int-float-conversion"  // warning: implicit conversion from 'xxx' to 'float' may lose precision
// #elif defined(__GNUC__)
// #pragma GCC diagnostic ignored "-Wpragmas"                          // warning: unknown option after '#pragma GCC diagnostic' kind
// #pragma GCC diagnostic ignored "-Wformat-nonliteral"                // warning: format not a string literal, format string not checked
// #pragma GCC diagnostic ignored "-Wclass-memaccess"                  // [__GNUC__ >= 8] warning: 'memset/memcpy' clearing/writing an object of type 'xxxx' with no trivial copy-assignment; use assignment or value-initialization instead
// #endif

//-----------------------------------------------------------------------------
// [SECTION] Tables: Main code
//-----------------------------------------------------------------------------
// - TableFixFlags() [Internal]
// - TableFindByID() [Internal]
// - BeginTable()
// - BeginTableEx() [Internal]
// - TableBeginInitMemory() [Internal]
// - TableBeginApplyRequests() [Internal]
// - TableSetupColumnFlags() [Internal]
// - TableUpdateLayout() [Internal]
// - TableUpdateBorders() [Internal]
// - EndTable()
// - TableSetupColumn()
// - TableSetupScrollFreeze()
//-----------------------------------------------------------------------------

use std::borrow::BorrowMut;
use std::io::SeekFrom::Current;
use std::ptr::{null, null_mut};
use std::sync::mpsc::channel;
use libc::{c_char, c_float, c_int, c_void, memcpy, memset, size_t, strlen};
use crate::chunk_stream::ImChunkStream;
use crate::color::{color_u32_from_rgba, IM_COL32_DISABLE, ImGuiCol_Header, ImGuiCol_HeaderActive, ImGuiCol_HeaderHovered, ImGuiCol_Separator, ImGuiCol_SeparatorActive, ImGuiCol_SeparatorHovered, ImGuiCol_TableBorderLight, ImGuiCol_TableBorderStrong, ImGuiCol_TableHeaderBg, ImGuiCol_Text, ImGuiCol_TextDisabled};
use crate::context_ops::GetFrameCount;
use crate::draw_list_ops::GetForegroundDrawList;
use crate::{AddSettingsHandler, GImGui, SettingsHandler, hash_string};
use crate::bit_array::ImBitArray;
use crate::widgets::button_flags::{ImGuiButtonFlags_AllowItemOverlap, ImGuiButtonFlags_FlattenChildren, ImGuiButtonFlags_NoNavFocus, ImGuiButtonFlags_PressedOnClick, ImGuiButtonFlags_PressedOnDoubleClick};
use crate::child_ops::{BeginChildEx, EndChild};
use crate::content_ops::content_region_avail;
use crate::core::context::AppContext;
use crate::cursor_ops::cursor_screen_pos;
use crate::core::direction::{ImGuiDir_Down, ImGuiDir_Up};
use crate::drawing::draw_channel::ImDrawChannel;
use crate::drawing::draw_list::ImDrawList;
use crate::drawing::draw_list_splitter::ImDrawListSplitter;
use crate::widgets::hovered_flags::ImGuiHoveredFlags_DelayNormal;
use crate::core::id_ops::{ClearActiveID, id_from_str, KeepAliveID, pop_win_id_from_stack, PushID, push_int_id, PushOverrideID};
use crate::input_ops::{GetMousePos, IsMouseDoubleClicked, IsMouseDragging, SetMouseCursor};
use crate::item::item_flags::ImGuiItemFlags_SelectableDontClosePopup;
use crate::item::item_ops::{CalcItemSize, GetItemRectMax, GetItemRectMin, IsAnyItemHovered, IsClippedEx, IsItemHovered, ItemAdd, ItemHoverable, ItemSize, PopItemFlag, PopItemWidth, PushItemFlag, PushItemWidth};
use crate::logging_ops::LogRenderedText;
use crate::core::math_ops::{ImClamp, ImLerp, ImMax, ImMin, ImSwap};
use crate::widgets::merge_group::MergeGroup;
use crate::io::mouse_button::ImGuiMouseButton_Right;
use crate::io::mouse_cursor::ImGuiMouseCursor_ResizeEW;
use crate::nav_highlight_flags::{ImGuiNavHighlightFlags_NoRounding, ImGuiNavHighlightFlags_TypeThin};
use crate::nav_layer::{ImGuiNavLayer_Main, ImGuiNavLayer_Menu};
use crate::table::old_column_data::ImGuiOldColumnData;
use crate::table::old_column_flags::{ImGuiOldColumnFlags, ImGuiOldColumnFlags_GrowParentContentsSize, ImGuiOldColumnFlags_NoBorder, ImGuiOldColumnFlags_NoForceWithinWindow, ImGuiOldColumnFlags_NoPreserveWidths, ImGuiOldColumnFlags_NoResize};
use crate::table::old_columns::ImGuiOldColumns;
use crate::popup_flags::ImGuiPopupFlags_None;
use crate::popup_ops::{BeginPopupEx, EndPopup, OpenPopupEx};
use crate::rect::ImRect;
use crate::drawing::render_ops::{FindRenderedTextEnd, RenderArrow, RenderNavHighlight, RenderText, RenderTextEllipsis};
use crate::widgets::scrolling_ops::SetScrollFromPosX;
use crate::settings_ops::MarkIniSettingsDirty;
use crate::layout::sort_direction::{ImGuiSortDirection, ImGuiSortDirection_Ascending, ImGuiSortDirection_Descending, ImGuiSortDirection_None};
use crate::widgets::span::ImSpan;
use crate::widgets::span_allocator::ImSpanAllocator;
use crate::core::string_ops::{ImFormatString, ImStrSkipBlank, str_to_const_c_char_ptr};
use crate::style_ops::{GetColorU32, GetStyle, GetStyleColorVec4, PopStyleColor, PushStyleColor};
use crate::table::ImGuiTable;
use crate::table_bg_target::{ImGuiTableBgTarget, ImGuiTableBgTarget_CellBg, ImGuiTableBgTarget_RowBg0, ImGuiTableBgTarget_RowBg1};
use crate::table_cell_data::ImGuiTableCellData;
use crate::table_column::ImGuiTableColumn;
use crate::table_column_flags::{ImGuiTableColumnFlags, ImGuiTableColumnFlags_DefaultHide, ImGuiTableColumnFlags_DefaultSort, ImGuiTableColumnFlags_Disabled, ImGuiTableColumnFlags_IndentDisable, ImGuiTableColumnFlags_IndentEnable, ImGuiTableColumnFlags_IndentMask_, ImGuiTableColumnFlags_IsEnabled, ImGuiTableColumnFlags_IsHovered, ImGuiTableColumnFlags_IsSorted, ImGuiTableColumnFlags_IsVisible, ImGuiTableColumnFlags_NoClip, ImGuiTableColumnFlags_NoDirectResize_, ImGuiTableColumnFlags_NoHeaderLabel, ImGuiTableColumnFlags_NoHeaderWidth, ImGuiTableColumnFlags_NoHide, ImGuiTableColumnFlags_None, ImGuiTableColumnFlags_NoReorder, ImGuiTableColumnFlags_NoResize, ImGuiTableColumnFlags_NoSort, ImGuiTableColumnFlags_NoSortAscending, ImGuiTableColumnFlags_NoSortDescending, ImGuiTableColumnFlags_PreferSortAscending, ImGuiTableColumnFlags_PreferSortDescending, ImGuiTableColumnFlags_StatusMask_, ImGuiTableColumnFlags_WidthFixed, ImGuiTableColumnFlags_WidthMask_, ImGuiTableColumnFlags_WidthStretch};
use crate::table_column_settings::ImGuiTableColumnSettings;
use crate::table_column_sort_specs::ImGuiTableColumnSortSpecs;
use crate::table_flags::{ImGuiTableFlags, ImGuiTableFlags_Borders, ImGuiTableFlags_BordersInnerH, ImGuiTableFlags_BordersInnerV, ImGuiTableFlags_BordersOuter, ImGuiTableFlags_BordersOuterH, ImGuiTableFlags_BordersOuterV, ImGuiTableFlags_ContextMenuInBody, ImGuiTableFlags_Hideable, ImGuiTableFlags_NoBordersInBody, ImGuiTableFlags_NoBordersInBodyUntilResize, ImGuiTableFlags_NoClip, ImGuiTableFlags_NoHostExtendX, ImGuiTableFlags_NoHostExtendY, ImGuiTableFlags_NoKeepColumnsVisible, ImGuiTableFlags_None, ImGuiTableFlags_NoPadInnerX, ImGuiTableFlags_NoPadOuterX, ImGuiTableFlags_NoSavedSettings, ImGuiTableFlags_PadOuterX, ImGuiTableFlags_PreciseWidths, ImGuiTableFlags_Reorderable, ImGuiTableFlags_Resizable, ImGuiTableFlags_ScrollX, ImGuiTableFlags_ScrollY, ImGuiTableFlags_SizingFixedFit, ImGuiTableFlags_SizingFixedSame, ImGuiTableFlags_SizingMask_, ImGuiTableFlags_SizingStretchProp, ImGuiTableFlags_SizingStretchSame, ImGuiTableFlags_Sortable, ImGuiTableFlags_SortMulti, ImGuiTableFlags_SortTristate};
use crate::table_instance_data::ImGuiTableInstanceData;
use crate::table_ops::{TableEndCell, TableEndRow, TableGetCellBgRect, TableGetInstanceData};
use crate::table_row_flags::{ImGuiTableRowFlags, ImGuiTableRowFlags_Headers, ImGuiTableRowFlags_None};
use crate::table_settings::ImGuiTableSettings;
use crate::table_sort_specs::ImGuiTableSortSpecs;
use crate::table_temp_data::ImGuiTableTempData;
use crate::text_buffer::ImGuiTextBuffer;
use crate::text_ops::{CalcTextSize, GetTextLineHeight};
use crate::core::type_defs::{ImguiHandle, ImGuiTableColumnIdx, ImGuiTableDrawChannelIdx};
use crate::core::utils::{flag_clear, flag_set};
use crate::core::vec2::Vector2;
use crate::window::ImguiWindow;
use crate::window::ops::GetCurrentWindow;
use crate::window::props::{SetNextWindowContentSize, SetNextWindowScroll};
use crate::window::rect::{PopClipRect, PushClipRect, SetWindowClipRectBeforeSetChannel};
use crate::window::window_flags::{ImGuiWindowFlags, ImGuiWindowFlags_AlwaysAutoResize, ImGuiWindowFlags_HorizontalScrollbar, ImGuiWindowFlags_None, ImGuiWindowFlags_NoSavedSettings, ImGuiWindowFlags_NoTitleBar};

// Configuration
pub const TABLE_DRAW_CHANNEL_BG0: c_int = 0;
pub const TABLE_DRAW_CHANNEL_BG2_FROZEN: c_int = 1;
pub const TABLE_DRAW_CHANNEL_NOCLIP: c_int = 2;                     // When using ImGuiTableFlags_NoClip (this becomes the last visible channel)
pub const TABLE_BORDER_SIZE: c_float                     = 1.0;    // FIXME-TABLE: Currently hard-coded because of clipping assumptions with outer borders rendering.
pub const TABLE_RESIZE_SEPARATOR_HALF_THICKNESS: c_float =  4.0;    // Extend outside inner borders.
pub const TABLE_RESIZE_SEPARATOR_FEEDBACK_TIMER: c_float =  0.06;   // Delay/timer before making the hover feedback (color+cursor) visible because tables/columns tends to be more cramped.

// Helper
pub fn TableFixFlags(mut flags: ImGuiTableFlags, outer_window: &mut ImguiWindow) -> ImGuiTableFlags
{
    // Adjust flags: set default sizing policy
    if (flag_clear(flags, ImGuiTableFlags_SizingMask_)) {
        flags |= if flag_set(flags, ImGuiTableFlags_ScrollX) || flag_set(outer_window.Flags, ImGuiWindowFlags_AlwaysAutoResize) { ImGuiTableFlags_SizingFixedFit } else { ImGuiTableFlags_SizingStretchSame };
    }

    // Adjust flags: enable NoKeepColumnsVisible when using ImGuiTableFlags_SizingFixedSame
    if (flags & ImGuiTableFlags_SizingMask_) == ImGuiTableFlags_SizingFixedSame {
        flags |= ImGuiTableFlags_NoKeepColumnsVisible;
    }

    // Adjust flags: enforce borders when resizable
    if flag_set(flags, ImGuiTableFlags_Resizable) {
        flags |= ImGuiTableFlags_BordersInnerV;
    }

    // Adjust flags: disable NoHostExtendX/NoHostExtendY if we have any scrolling going on
    if flags & (ImGuiTableFlags_ScrollX | ImGuiTableFlags_ScrollY) {
        flags &= !(ImGuiTableFlags_NoHostExtendX | ImGuiTableFlags_NoHostExtendY);
    }

    // Adjust flags: NoBordersInBodyUntilResize takes priority over NoBordersInBody
    if flag_set(flags, ImGuiTableFlags_NoBordersInBodyUntilResize) {
        flags &= !ImGuiTableFlags_NoBordersInBody;
    }

    // Adjust flags: disable saved settings if there's nothing to save
    if ((flags & (ImGuiTableFlags_Resizable | ImGuiTableFlags_Hideable | ImGuiTableFlags_Reorderable | ImGuiTableFlags_Sortable)) == 0) {
        flags |= ImGuiTableFlags_NoSavedSettings;
    }

    // Inherit _NoSavedSettings from top-level window (child windows always have _NoSavedSettings set)
    if (outer_window.Rootwindow.Flags & ImGuiWindowFlags_NoSavedSettings) {
        flags |= ImGuiTableFlags_NoSavedSettings;
    }

    return flags;
}

pub unsafe fn TableFindByID(id: ImguiHandle) -> *mut ImGuiTable
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return g.Tables.GetByKey(id);
}

// Read about "TABLE SIZING" at the top of this file.
pub fn BeginTable(str_id: &str, columns_count: usize, flags: ImGuiTableFlags, outer_size: Option<&mut Vector2>, inner_width: c_float) -> bool {
    let mut id: ImguiHandle = id_from_str(str_id);
    return BeginTableEx(g, str_id, id, columns_count, flags, outer_size, inner_width);
}

pub fn  BeginTableEx(g: &mut AppContext, name: &String, id: ImguiHandle, columns_count: usize, mut flags: ImGuiTableFlags, outer_size: &mut Vector2, inner_width: c_float) -> bool
{
    let outer_window = g.current_window_mut().unwrap();
    if outer_window.skip_items {// Consistent with other tables + beneficial side effect that assert on miscalling EndTable() will be more visible.
        return false;
    }

    // Sanity checks
    // IM_ASSERT(columns_count > 0 && columns_count <= IMGUI_TABLE_MAX_COLUMNS && "Only 1..64 columns allowed!");
    if flag_set(flags , ImGuiTableFlags_ScrollX) {
        // IM_ASSERT(inner_width >= 0.0);
    }

    // If an outer size is specified ahead we will be able to early out when not visible. Exact clipping rules may evolve.
    let use_child_window: bool = (flags & (ImGuiTableFlags_ScrollX | ImGuiTableFlags_ScrollY)) != 0;
    let avail_size: Vector2 = content_region_avail(g);
    let actual_outer_size: Vector2 = CalcItemSize(g, outer_size, avail_size.x.max(1.0), if use_child_window { avail_size.y.max(1.0) } else { 0.0 });
    let mut outer_rect: ImRect = ImRect::new(outer_window.dc.cursor_pos, outer_window.dc.cursor_pos + actual_outer_size);
    if use_child_window && IsClippedEx(&mut outer_rect, 0)
    {
        ItemSize(g, &outer_rect.into(), 0.0);
        return false;
    }

    // Acquire storage for the table
    let mut table: *mut ImGuiTable = g.Tables.GetOrAddByKey(id);
    let instance_no: c_int = if table.LastFrameActive != g.FrameCount { 0} else { table.InstanceCurrent + 1};
    let mut instance_id: ImguiHandle =  id + instance_no;
    let table_last_flags: ImGuiTableFlags = table.Flags;
    if instance_no > 0 {}
        // IM_ASSERT(table.ColumnsCount == columns_count && "BeginTable(): Cannot change columns count mid-frame while preserving same ID");

    // Acquire temporary buffers
    let table_idx: c_int = g.Tables.GetIndex(table);
        g.TablesTempDataStacked += 1;
    if g.TablesTempDataStacked > g.TablesTempData.Size {
        g.TablesTempData.resize(g.TablesTempDataStacked, ImGuiTableTempData());
    }
        table.TempData = &mut g.TablesTempData[g.TablesTempDataStacked - 1];
   let mut temp_data: *mut ImGuiTableTempData = table.TempData;
    temp_data.TableIndex = table_idx;
    table.DrawSplitter = &mut table.TempData.DrawSplitter;
    table.DrawSplitter.Clear();

    // Fix flags
    table.IsDefaultSizingPolicy = flag_clear(flags, ImGuiTableFlags_SizingMask_);
    flags = TableFixFlags(flags, outer_window);

    // initialize
    table.ID = id;
    table.Flags = flags;
    table.InstanceCurrent = instance_no;
    table.LastFrameActive = g.FrameCount;
    table.OuterWindow = outer_window;
    table.InnerWindow = outer_window;
    table.ColumnsCount = columns_count;
    table.IsLayoutLocked = false;
    table.InnerWidth = inner_width;
    temp_data.UserOuterSize = outer_size.clone();
    if instance_no > 0 && table.InstanceDataExtra.Size < instance_no{
        table.InstanceDataExtra.push(ImGuiTableInstanceData());}

    // When not using a child window, WorkRect.Max will grow as we append contents.
    if use_child_window
    {
        // Ensure no vertical scrollbar appears if we only want horizontal one, to make flag consistent
        // (we have no other way to disable vertical scrollbar of a window while keeping the horizontal one showing)
        override_content_size: Vector2(f32::MAX, f32::MAX);
        if flag_set(flags, ImGuiTableFlags_ScrollX) && flag_clear(flags, ImGuiTableFlags_ScrollY) {
            override_content_size.y = FLT_MIN;
        }

        // Ensure specified width (when not specified, Stretched columns will act as if the width == OuterWidth and
        // never lead to any scrolling). We don't handle inner_width < 0.0, we could potentially use it to right-align
        // based on the right side of the child window work rect, which would require knowing ahead if we are going to
        // have decoration taking horizontal spaces (typically a vertical scrollbar).
        if flag_set(flags , ImGuiTableFlags_ScrollX) && inner_width > 0.0{
            override_content_size.x = inner_width;}

        if override_content_size.x != f32::MAX || override_content_size.y != f32::MAX {
            SetNextWindowContentSize(&Vector2::from_floats(if override_content_size.x != f32::MAX { override_content_size.x } else { 0.0 }, if override_content_size.y != f32::MAX { override_content_size.y }else { 0.0 }));
        }

        // Reset scroll if we are reactivating it
        if (table_last_flags & (ImGuiTableFlags_ScrollX | ImGuiTableFlags_ScrollY)) == 0 {
            SetNextWindowScroll(&Vector2::from_floats(0.0, 0.0));
        }

        // Create scrolling region (without border and zero window padding)
        let mut child_flags: ImGuiWindowFlags = if flags & ImGuiTableFlags_ScrollX { ImGuiWindowFlags_HorizontalScrollbar} else { ImGuiWindowFlags_None};
        BeginChildEx(name, instance_id, &outer_rect.GetSize(), false, child_flags);
        table.InnerWindow = g.CurrentWindow;
        table.WorkRect = table.Innerwindow.work_rect;
        table.OuterRect = table.Innerwindow.Rect();
        table.InnerRect = table.Innerwindow.InnerRect;
        // IM_ASSERT(table.Innerwindow.WindowPadding.x == 0.0 && table.Innerwindow.WindowPadding.y == 0.0 && table.Innerwindow.WindowBorderSize == 0.0);
    }
    else
    {
        // For non-scrolling tables, WorkRect == OuterRect == InnerRect.
        // But at this point we do NOT have a correct value for .Max.y (unless a height has been explicitly passed in). It will only be updated in EndTable().
        table.WorkRect =  outer_rect;table.OuterRect =  outer_rect;table.InnerRect = outer_rect;
    }

    // Push a standardized ID for both child-using and not-child-using tables
    PushOverrideID(g, instance_id);

    // Backup a copy of host window members we will modify
    inner_window: &mut ImguiWindow = table.InnerWindow;
    table.HostIndentX = inner_window.dc.indent.x;
    table.HostClipRect = inner_window.ClipRect;
    table.HostSkipItems = inner_window.skip_items;
    temp_data.HostBackupWorkRect = inner_window.work_rect;
    temp_data.HostBackupParentWorkRect = inner_window.ParentWorkRect;
    temp_data.HostBackupColumnsOffset = outer_window.DC.ColumnsOffset;
    temp_data.HostBackupPrevLineSize = inner_window.DC.PrevLineSize;
    temp_data.HostBackupCurrLineSize = inner_window.DC.CurrLineSize;
    temp_data.HostBackupCursorMaxPos = inner_window.DC.CursorMaxPos;
    temp_data.HostBackupItemWidth = outer_window.DC.ItemWidth;
    temp_data.HostBackupItemWidthStackSize = outer_window.DC.ItemWidthStack.Size;
    inner_window.DC.PrevLineSize = inner_window.DC.CurrLineSize = Vector2::from_floats(0.0, 0.0);

    // Padding and Spacing
    // - None               ........Content..... Pad .....Content........
    // - PadOuter           | Pad ..Content..... Pad .....Content.. Pad |
    // - PadInner           ........Content.. Pad | Pad ..Content........
    // - PadOuter+PadInner  | Pad ..Content.. Pad | Pad ..Content.. Pad |
    let pad_outer_x: bool = if flag_set(flags , ImGuiTableFlags_NoPadOuterX) { false } else{ if flags & ImGuiTableFlags_PadOuterX { true } else { flag_set(flags, ImGuiTableFlags_BordersOuterV) } };
    let pad_inner_x: bool = if flags & ImGuiTableFlags_NoPadInnerX { false} else { true};
    let inner_spacing_for_border: c_float =  if flag_set(flags, ImGuiTableFlags_BordersInnerV) { TABLE_BORDER_SIZE } else { 0.0 };
    let inner_spacing_explicit: c_float =  if (pad_inner_x && flag_clear(flags, ImGuiTableFlags_BordersInnerV)) { g.style.CellPadding.x } else { 0.0 };
    let inner_padding_explicit: c_float =  if (pad_inner_x && flag_set(flags, ImGuiTableFlags_BordersInnerV)) { g.style.CellPadding.x } else { 0.0 };
    table.CellSpacingX1 = inner_spacing_explicit + inner_spacing_for_border;
    table.CellSpacingX2 = inner_spacing_explicit;
    table.CellPaddingX = inner_padding_explicit;
    table.CellPaddingY = g.style.CellPadding.y;

    let outer_padding_for_border: c_float =  if flag_set(flags, ImGuiTableFlags_BordersOuterV) { TABLE_BORDER_SIZE } else { 0.0 };
    let outer_padding_explicit: c_float =  if pad_outer_x { g.style.CellPadding.x } else { 0.0 };
    table.OuterPaddingX = (outer_padding_for_border + outer_padding_explicit) - table.CellPaddingX;

    table.CurrentColumn = -1;
    table.CurrentRow = -1;
    table.RowBgColorCounter = 0;
    table.LastRowFlags = ImGuiTableRowFlags_None;
    table.InnerClipRect = if inner_window == outer_window { table.WorkRect} else { inner_window.ClipRect};
    table.InnerClipRect.ClipWith(table.WorkRect);     // We need this to honor inner_width
    table.InnerClipRect.ClipWithFull(&table.HostClipRect);
    table.InnerClipRect.max.y = if flags & ImGuiTableFlags_NoHostExtendY { ImMin(table.InnerClipRect.max.y as c_int, inner_window.work_rect.Max.y)} else { inner_window.ClipRect.Max.y};

    table.RowPosY1 = table.WorkRect.min.y;table.RowPosY2 = table.WorkRect.min.y; // This is needed somehow
    table.RowTextBaseline = 0.0; // This will be cleared again by TableBeginRow()
    table.FreezeRowsRequest = 0;
    table.FreezeRowsCount = 0; // This will be setup by TableSetupScrollFreeze(), if any
    table.FreezeColumnsRequest = 0;
    table.FreezeColumnsCount = 0;
    table.IsUnfrozenRows = true;
    table.DeclColumnsCount = 0;

    // Using opaque colors facilitate overlapping elements of the grid
    table.BorderColorStrong = GetColorU32(ImGuiCol_TableBorderStrong, 0.0);
    table.BorderColorLight = GetColorU32(ImGuiCol_TableBorderLight, 0.0);

    // Make table current
    g.CurrentTable = table;
    outer_window.DC.CurrentTableIdx = table_idx;
    if (inner_window != outer_window) { // So EndChild() within the inner window can restore the table properly.
        inner_window.DC.CurrentTableIdx = table_idx;
    }

    if (flag_set(table_last_flags , ImGuiTableFlags_Reorderable) && flag_clear(flags, ImGuiTableFlags_Reorderable)) {
        table.IsResetDisplayOrderRequest = true;
    }

    // Mark as used
    if (table_idx >= g.TablesLastTimeActive.Size) {
        g.TablesLastTimeActive.resize((table_idx + 1) as usize, -1.0);
    }
    g.TablesLastTimeActive[table_idx] = g.Time;
    temp_data.LastTimeActive = g.Time as c_float;
    table.MemoryCompacted = false;

    // Setup memory buffer (clear data if columns count changed)
    let mut old_columns_to_preserve: *mut ImGuiTableColumn= None;
    let mut old_columns_raw_data: *mut c_void= None;
    let old_columns_count = table.Columns;
    if old_columns_count != 0 && old_columns_count != columns_count
    {
        // Attempt to preserve width on column count change (#4046)
        old_columns_to_preserve = table.Columns.Data;
        old_columns_raw_data = table.RawData;
        table.RawData= None;
    }
    if (table.RawData == null_mut())
    {
        TableBeginInitMemory(table, columns_count);
        table.IsInitializing = true;
        table.IsSettingsRequestLoad = true;
    }
    if table.IsResetAllRequest{
        TableResetSettings(table);}
    if (table.IsInitializing)
    {
        // initialize
        table.SettingsOffset = -1;
        table.IsSortSpecsDirty = true;
        table.InstanceInteracted = -1;
        table.ContextPopupColumn = -1;
        table.ReorderColumn = -1;table.ResizedColumn = -1; table.LastResizedColumn = -1;
        table.AutoFitSingleColumn = -1;
        table.HoveredColumnBody = -1;table.HoveredColumnBorder = -1;
        // for (let n: c_int = 0; n < columns_count; n++)
        for n in 0 .. columns_count
        {
            let mut column: *mut ImGuiTableColumn = &mut table.Columns[n];
            if old_columns_to_preserve.is_null() == false && n < old_columns_count
            {
                // FIXME: We don't attempt to preserve column order in this path.
                *column = old_columns_to_preserve[n];
            }
            else
            {
                let width_auto: c_float =  column.WidthAuto;
                *column = ImGuiTableColumn();
                column.WidthAuto = width_auto;
                column.IsPreserveWidthAuto = true; // Preserve WidthAuto when reinitializing a live table: not technically necessary but remove a visible flicker
                column.IsEnabled = true;column.IsUserEnabled = true; column.IsUserEnabledNextFrame = true;
            }
            column.DisplayOrder = n as ImGuiTableColumnIdx;
            table.DisplayOrderToIndex[n] = n;
        }
    }
    if old_columns_raw_data {
        IM_FREE(old_columns_raw_data)(); }

    // Load settings
    if table.IsSettingsRequestLoad{
        TableLoadSettings(table);}

    // Handle DPI/font resize
    // This is designed to facilitate DPI changes with the assumption that e.g. style.CellPadding has been scaled as well.
    // It will also react to changing fonts with mixed results. It doesn't need to be perfect but merely provide a decent transition.
    // FIXME-DPI: Provide consistent standards for reference size. Perhaps using g.CurrentDpiScale would be more self explanatory.
    // This is will lead us to non-rounded WidthRequest in columns, which should work but is a poorly tested path.
    let new_ref_scale_unit: c_float =  g.FontSize; // g.Font->GetCharAdvance('A') ?
    if (table.RefScale != 0.0 && table.RefScale != new_ref_scale_unit)
    {
        let scale_factor: c_float =  new_ref_scale_unit / table.RefScale;
        //IMGUI_DEBUG_PRINT("[table] {} RefScaleUnit {} -> {}, scaling width by {}\n", table.ID, table.RefScaleUnit, new_ref_scale_unit, scale_factor);
        // for (let n: c_int = 0; n < columns_count; n++)
        for n in 0 .. columns_count
        {
            table.Columns[n].WidthRequest = table.Columns[n].WidthRequest * scale_factor;
        }
    }
    table.RefScale = new_ref_scale_unit;

    // Disable output until user calls TableNextRow() or TableNextColumn() leading to the TableUpdateLayout() call..
    // This is not strictly necessary but will reduce cases were "out of table" output will be misleading to the user.
    // Because we cannot safely assert in EndTable() when no rows have been created, this seems like our best option.
    inner_window.skip_items = true;

    // Clear names
    // At this point the .NameOffset field of each column will be invalid until TableUpdateLayout() or the first call to TableSetupColumn()
    if table.ColumnsNames.Buf.Size > 0{
        table.ColumnsNames.Buf.clear();}

    // Apply queued resizing/reordering/hiding requests
    TableBeginApplyRequests(table);

    return true;
}

// For reference, the average total _allocation count_ for a table is:
// + 0 (for ImGuiTable instance, we are pooling allocations in g.Tables)
// + 1 (for table.RawData allocated below)
// + 1 (for table.ColumnsNames, if names are used)
// Shared allocations per number of nested tables
// + 1 (for table.Splitter._Channels)
// + 2 * active_channels_count (for ImDrawCmd and ImDrawIdx buffers inside channels)
// Where active_channels_count is variable but often == columns_count or columns_count + 1, see TableSetupDrawChannels() for details.
// Unused channels don't perform their +2 allocations.
pub unsafe fn TableBeginInitMemory(table: *mut ImGuiTable, columns_count: c_int)
{
    // Allocate single buffer for our arrays
    // ImSpanAllocator<3> span_allocator;
    let mut span_allocator: ImSpanAllocator = ImSpanAllocator::default();
    span_allocator.Reserve(0, columns_count * sizeof(ImGuiTableColumn));
    span_allocator.Reserve(1, columns_count * sizeof(ImGuiTableColumnIdx));
    span_allocator.Reserve(2, columns_count * sizeof(ImGuiTableCellData));
    table.RawData = IM_ALLOC(span_allocator.GetArenaSizeInBytes());
    memset(table.RawData, 0, span_allocator.GetArenaSizeInBytes());
    span_allocator.SetArenaBasePtr(table.RawData);
    span_allocator.GetSpan(0, &mut table.Columns);
    span_allocator.GetSpan(1, &mut table.DisplayOrderToIndex);
    span_allocator.GetSpan(2, &mut table.RowCellData);
}

// Apply queued resizing/reordering/hiding requests
pub unsafe fn TableBeginApplyRequests(table: *mut ImGuiTable)
{
    // Handle resizing request
    // (We process this at the first TableBegin of the frame)
    // FIXME-TABLE: Contains columns if our work area doesn't allow for scrolling?
    if table.InstanceCurrent == 0
    {
        if table.ResizedColumn != -1 && table.ResizedColumnNextWidth != f32::MAX {
            TableSetColumnWidth(table.ResizedColumn as c_int, table.ResizedColumnNextWidth);
        }
        table.LastResizedColumn = table.ResizedColumn;
        table.ResizedColumnNextWidth = f32::MAX;
        table.ResizedColumn = -1;

        // Process auto-fit for single column, which is a special case for stretch columns and fixed columns with FixedSame policy.
        // FIXME-TABLE: Would be nice to redistribute available stretch space accordingly to other weights, instead of giving it all to siblings.
        if table.AutoFitSingleColumn != -1
        {
            TableSetColumnWidth(table.AutoFitSingleColumn as c_int, table.Columns[table.AutoFitSingleColumn].WidthAuto);
            table.AutoFitSingleColumn = -1;
        }
    }

    // Handle reordering request
    // Note: we don't clear ReorderColumn after handling the request.
    if table.InstanceCurrent == 0
    {
        if table.HeldHeaderColumn == -1 && table.ReorderColumn != -1 {
            table.ReorderColumn = -1;
        }
        table.HeldHeaderColumn = -1;
        if table.ReorderColumn != -1 && table.ReorderColumnDir != 0
        {
            // We need to handle reordering across hidden columns.
            // In the configuration below, moving C to the right of E will lead to:
            //    ... C [D] E  --->  ... [D] E  C   (Column name/index)
            //    ... 2  3  4        ...  2  3  4   (Display order)
            let reorder_dir = table.ReorderColumnDir;
            // IM_ASSERT(reorder_dir == -1 || reorder_dir == +1);
            // IM_ASSERT(table.Flags & ImGuiTableFlags_Reorderable);
            let mut src_column: *mut ImGuiTableColumn = &mut table.Columns[table.ReorderColumn];
            let mut dst_column: *mut ImGuiTableColumn = &mut table.Columns[if reorder_dir == -1 { src_column.PrevEnabledColumn } else { src_column.NextEnabledColumn}] ;
            IM_UNUSED(dst_column);
            let src_order = src_column.DisplayOrder;
            let dst_order = dst_column.DisplayOrder;
            src_column.DisplayOrder = dst_order;
            // for (let order_n: c_int = src_order + reorder_dir; order_n != dst_order + reorder_dir; order_n += reorder_dir)
            let mut order_n = src_order + reorder_dir;
            while order_n != dst_order + reorder_dir
            {
                table.Columns[table.DisplayOrderToIndex[order_n]].DisplayOrder -= reorder_dir;
                order_n += redorder_dir;

            }
            // IM_ASSERT(dst_column.DisplayOrder == dst_order - reorder_dir);

            // Display order is stored in both columns.IndexDisplayOrder and table.DisplayOrder[],
            // rebuild the later from the former.
            // for (let column_n: c_int = 0; column_n < table.ColumnsCount; column_n++)
            for column_n in 0 .. table.ColumnsCount
            {
                table.DisplayOrderToIndex[table.Columns[column_n].DisplayOrder] = column_n;
            }
            table.ReorderColumnDir = 0;
            table.IsSettingsDirty = true;
        }
    }

    // Handle display order reset request
    if table.IsResetDisplayOrderRequest
    {
        // for (let n: c_int = 0; n < table.ColumnsCount; n++)
        for n in 0 .. table.ColumnsCount
        {
            table.DisplayOrderToIndex[n] = table.Columns[n].DisplayOrder = n;
        }
        table.IsResetDisplayOrderRequest = false;
        table.IsSettingsDirty = true;
    }
}

// Adjust flags: default width mode + stretch columns are not allowed when auto extending
pub unsafe fn TableSetupColumnFlags(table: *mut ImGuiTable, column: *mut ImGuiTableColumn, flags_in: ImGuiTableColumnFlags)
{
    let mut flags: ImGuiTableColumnFlags = flags_in;

    // Sizing Policy
    if flag_clear(flags, ImGuiTableColumnFlags_WidthMask_)
    {
        let table_sizing_policy: ImGuiTableFlags = (table.Flags & ImGuiTableFlags_SizingMask_);
        if table_sizing_policy == ImGuiTableFlags_SizingFixedFit || table_sizing_policy == ImGuiTableFlags_SizingFixedSame {
            flags |= ImGuiTableColumnFlags_WidthFixed;
        }
        else {
            flags |= ImGuiTableColumnFlags_WidthStretch;
        }
    }
    else
    {
        // IM_ASSERT(ImIsPowerOfTwo(flags & ImGuiTableColumnFlags_WidthMask_)); // Check that only 1 of each set is used.
    }

    // Resize
    if flag_set(table.Flags, ImGuiTableFlags_Resizable) == false {
        flags |= ImGuiTableColumnFlags_NoResize;
    }

    // Sorting
    if (flag_set(flags, ImGuiTableColumnFlags_NoSortAscending) && (flags & ImGuiTableColumnFlags_NoSortDescending)) {
        flags |= ImGuiTableColumnFlags_NoSort;
    }

    // Indentation
    if (flag_clear(flags, ImGuiTableColumnFlags_IndentMask_)) {
        flags |= if table.Columns.index_from_ptr(column) == 0 { ImGuiTableColumnFlags_IndentEnable } else { ImGuiTableColumnFlags_IndentDisable };
    }

    // Alignment
    //if (flag_set(flags, ImGuiTableColumnFlags_AlignMask_) == 0)
    //    flags |= ImGuiTableColumnFlags_AlignCenter;
    //IM_ASSERT(ImIsPowerOfTwo(flags & ImGuiTableColumnFlags_AlignMask_)); // Check that only 1 of each set is used.

    // Preserve status flags
    column.Flags = flags | (column.Flags & ImGuiTableColumnFlags_StatusMask_);

    // Build an ordered list of available sort directions
    column.SortDirectionsAvailCount = 0;
    column.SortDirectionsAvailMask = 0;
    column.SortDirectionsAvailList = 0;
    if flag_set(table.Flags, ImGuiTableFlags_Sortable)
    {
        // let count: c_int = 0, mask = 0, list = 0;
        let mut count: c_int = 0;
        let mut mask: c_int = 0;
        let mut list: c_int = 0;
        if flag_set(flags, ImGuiTableColumnFlags_PreferSortAscending) && flag_set(flags, ImGuiTableColumnFlags_NoSortAscending)  == false { mask |= 1 << ImGuiSortDirection_Ascending;  list |= ImGuiSortDirection_Ascending  << (count << 1); count+= 1; }
        if flag_set(flags, ImGuiTableColumnFlags_PreferSortDescending) && flag_clear(flags, ImGuiTableColumnFlags_NoSortDescending) { mask |= 1 << ImGuiSortDirection_Descending; list |= ImGuiSortDirection_Descending << (count << 1); count+= 1; }
        if flag_set(flags, ImGuiTableColumnFlags_PreferSortAscending)  == false && flag_set(flags, ImGuiTableColumnFlags_NoSortAscending)  == false { mask |= 1 << ImGuiSortDirection_Ascending;  list |= ImGuiSortDirection_Ascending  << (count << 1); count+= 1; }
        if flag_clear(flags, ImGuiTableColumnFlags_PreferSortDescending) && flag_clear(flags, ImGuiTableColumnFlags_NoSortDescending) { mask |= 1 << ImGuiSortDirection_Descending; list |= ImGuiSortDirection_Descending << (count << 1); count+= 1; }
        if flag_set(table.Flags , ImGuiTableFlags_SortTristate) || count == 0 { mask |= 1 << ImGuiSortDirection_None; count+= 1; }
        column.SortDirectionsAvailList = list as i8;
        column.SortDirectionsAvailMask = mask as i8;
        column.SortDirectionsAvailCount = count as i8;
        TableFixColumnSortDirection(table, column);
    }
}

// Layout columns for the frame. This is in essence the followup to BeginTable().
// Runs on the first call to TableNextRow(), to give a chance for TableSetupColumn() to be called first.
// FIXME-TABLE: Our width (and therefore our WorkRect) will be minimal in the first frame for _WidthAuto columns.
// Increase feedback side-effect with widgets relying on WorkRect.Max.x... Maybe provide a default distribution for _WidthAuto columns?
pub unsafe fn TableUpdateLayout(table: *mut ImGuiTable)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(table.IsLayoutLocked == false);

    let table_sizing_policy: ImGuiTableFlags = (table.Flags & ImGuiTableFlags_SizingMask_);
    table.IsDefaultDisplayOrder = true;
    table.ColumnsEnabledCount = 0;
    table.EnabledMaskByIndex = 0x00;
    table.EnabledMaskByDisplayOrder = 0x00;
    table.LeftMostEnabledColumn = -1;
    table.MinColumnWidth = ImMax(1.0, g.style.FramePadding.x * 1.0); // g.style.ColumnsMinSpacing; // FIXME-TABLE

    // [Part 1] Apply/lock Enabled and Order states. Calculate auto/ideal width for columns. Count fixed/stretch columns.
    // Process columns in their visible orders as we are building the Prev/Next indices.
    let mut count_fixed: c_int = 0;                // Number of columns that have fixed sizing policies
    let mut count_stretch: c_int = 0;              // Number of columns that have stretch sizing policies
    let mut prev_visible_column_idx: c_int = -1;
    let mut has_auto_fit_request: bool =  false;
    let mut has_resizable: bool =  false;
    let mut stretch_sum_width_auto: c_float =  0.0;
    let mut fixed_max_width_auto: c_float =  0.0;
    // for (let order_n: c_int = 0; order_n < table.ColumnsCount; order_n++)
    for order_n in 0 .. table.ColumnsCount
    {
        let column_n: c_int = table.DisplayOrderToIndex[order_n];
        if (column_n != order_n) {
            table.IsDefaultDisplayOrder = false;
        }
        let column: *mut ImGuiTableColumn = &mut table.Columns[column_n];

        // Clear column setup if not submitted by user. Currently we make it mandatory to call TableSetupColumn() every frame.
        // It would easily work without but we're not ready to guarantee it since e.g. names need resubmission anyway.
        // We take a slight shortcut but in theory we could be calling TableSetupColumn() here with dummy values, it should yield the same effect.
        if table.DeclColumnsCount <= column_n as ImGuiTableColumnIdx
        {
            TableSetupColumnFlags(table, column, ImGuiTableColumnFlags_None);
            column.NameOffset = -1;
            column.UserID = 0;
            column.InitStretchWeightOrWidth = -1.0;
        }

        // Update Enabled state, mark settings and sort specs dirty
        if flag_clear(table.Flags, ImGuiTableFlags_Hideable) || flag_set(column.Flags, ImGuiTableColumnFlags_NoHide) {
            column.IsUserEnabledNextFrame = true;
        }
        if column.IsUserEnabled != column.IsUserEnabledNextFrame
        {
            column.IsUserEnabled = column.IsUserEnabledNextFrame;
            table.IsSettingsDirty = true;
        }
        column.IsEnabled = column.IsUserEnabled && flag_set(column.Flags, ImGuiTableColumnFlags_Disabled) == false;

        if column.SortOrder != -1 && !column.IsEnabled {
            table.IsSortSpecsDirty = true;
        }
        if column.SortOrder > 0 && flag_clear(table.Flags, ImGuiTableFlags_SortMulti) {
            table.IsSortSpecsDirty = true;
        }

        // Auto-fit unsized columns
        let start_auto_fit: bool = if column.Flags & ImGuiTableColumnFlags_WidthFixed { (column.WidthRequest < 0.0)} else { (column.StretchWeight < 0.0)};
        if start_auto_fit {
            column.AutoFitQueue = (1 << 3) - 1;column.CannotSkipItemsQueue = (1 << 3) - 1;
        } // Fit for three frames

        if !column.IsEnabled
        {
            column.IndexWithinEnabledSet = -1;
            continue;
        }

        // Mark as enabled and link to previous/next enabled column
        column.PrevEnabledColumn = prev_visible_column_idx as ImGuiTableColumnIdx;
        column.NextEnabledColumn = -1;
        if prev_visible_column_idx != -1 {
            table.Columns[prev_visible_column_idx].NextEnabledColumn = column_n;
        }
        else {
            table.LeftMostEnabledColumn = column_n as ImGuiTableColumnIdx;
        }
        table.ColumnsEnabledCount+= 1;
        column.IndexWithinEnabledSet = table.ColumnsEnabledCount;
        table.EnabledMaskByIndex |= 1 << column_n;
        table.EnabledMaskByDisplayOrder |= 1 << column.DisplayOrder;
        prev_visible_column_idx = column_n;
        // IM_ASSERT(column.IndexWithinEnabledSet <= column.DisplayOrder);

        // Calculate ideal/auto column width (that's the width required for all contents to be visible without clipping)
        // Combine width from regular rows + width from headers unless requested not to.
        if !column.IsPreserveWidthAuto {
            column.WidthAuto = TableGetColumnWidthAuto(table, column);
        }

        // Non-resizable columns keep their requested width (apply user value regardless of IsPreserveWidthAuto)
        let column_is_resizable: bool = flag_clear(column.Flags, ImGuiTableColumnFlags_NoResize);
        if column_is_resizable {
            has_resizable = true;}
        if flag_set(column.Flags, ImGuiTableColumnFlags_WidthFixed) && column.InitStretchWeightOrWidth > 0.0 && !column_is_resizable {
            column.WidthAuto = column.InitStretchWeightOrWidth;
        }

        if column.AutoFitQueue != 0x00 {
            has_auto_fit_request = true;}
        if flag_set(column.Flags, ImGuiTableColumnFlags_WidthStretch)
        {
            stretch_sum_width_auto += column.WidthAuto;
            count_stretch+= 1;
        }
        else
        {
            fixed_max_width_auto = ImMax(fixed_max_width_auto, column.WidthAuto);
            count_fixed+= 1;
        }
    }
    if flag_set(table.Flags, ImGuiTableFlags_Sortable) && table.SortSpecsCount == 0 && flag_clear(table.Flags, ImGuiTableFlags_SortTristate) {
        table.IsSortSpecsDirty = true;
    }
    table.RightMostEnabledColumn = prev_visible_column_idx as ImGuiTableColumnIdx;
    // IM_ASSERT(table.LeftMostEnabledColumn >= 0 && table.RightMostEnabledColumn >= 0);

    // [Part 2] Disable child window clipping while fitting columns. This is not strictly necessary but makes it possible
    // to avoid the column fitting having to wait until the first visible frame of the child container (may or not be a good thing).
    // FIXME-TABLE: for always auto-resizing columns may not want to do that all the time.
    if (has_auto_fit_request && table.OuterWindow != table.InnerWindow) {
        table.Innerwindow.skip_items = false;
    }
    if has_auto_fit_request{
        table.IsSettingsDirty = true;}

    // [Part 3] Fix column flags and record a few extra information.
    let mut sum_width_requests: c_float =  0.0;        // Sum of all width for fixed and auto-resize columns, excluding width contributed by Stretch columns but including spacing/padding.
    let mut stretch_sum_weights: c_float =  0.0;       // Sum of all weights for stretch columns.
    table.LeftMostStretchedColumn = -1;
    table.RightMostStretchedColumn = -1;
    // for (let column_n: c_int = 0; column_n < table.ColumnsCount; column_n++)
    for column_n in 0 .. table.ColumnsCount
    {
        if !(table.EnabledMaskByIndex & (1 << column_n)) {
            continue;
        }
        let mut column: *mut ImGuiTableColumn = &mut table.Columns[column_n];

        let column_is_resizable: bool = flag_set(column.Flags, ImGuiTableColumnFlags_NoResize) == false;
        if column.Flags & ImGuiTableColumnFlags_WidthFixed
        {
            // Apply same widths policy
            let mut width_auto: c_float =  column.WidthAuto;
            if table_sizing_policy == ImGuiTableFlags_SizingFixedSame && (column.AutoFitQueue != 0x00 || !column_is_resizable) {
                width_auto = fixed_max_width_auto;}

            // Apply automatic width
            // Latch initial size for fixed columns and update it constantly for auto-resizing column (unless clipped!)
            if (column.AutoFitQueue != 0x00) {
                column.WidthRequest = width_auto;
            }
            else if flag_set(column.Flags, ImGuiTableColumnFlags_WidthFixed) && !column_is_resizable && (table.RequestOutputMaskByIndex & (1 << column_n)) != 0 {
                column.WidthRequest = width_auto;
            }

            // FIXME-TABLE: Increase minimum size during init frame to avoid biasing auto-fitting widgets
            // (e.g. TextWrapped) too much. Otherwise what tends to happen is that TextWrapped would output a very
            // large height (= first frame scrollbar display very off + clipper would skip lots of items).
            // This is merely making the side-effect less extreme, but doesn't properly fixes it.
            // FIXME: Move this to ->WidthGiven to avoid temporary lossyless?
            // FIXME: This break IsPreserveWidthAuto from not flickering if the stored WidthAuto was smaller.
            if column.AutoFitQueue > 0x01 && table.IsInitializing && !column.IsPreserveWidthAuto {
                column.WidthRequest = ImMax(column.WidthRequest, table.MinColumnWidth * 4.0);
            }// FIXME-TABLE: Another constant/scale?
            sum_width_requests += column.WidthRequest;
        }
        else
        {
            // initialize stretch weight
            if (column.AutoFitQueue != 0x00 || column.StretchWeight < 0.0 || !column_is_resizable)
            {
                if column.InitStretchWeightOrWidth > 0.0{
                    column.StretchWeight = column.InitStretchWeightOrWidth;}
                else if (table_sizing_policy == ImGuiTableFlags_SizingStretchProp) {
                    column.StretchWeight = (column.WidthAuto / stretch_sum_width_auto) * count_stretch;
                }
                else {
                    column.StretchWeight = 1.0;
                }
            }

            stretch_sum_weights += column.StretchWeight;
            if table.LeftMostStretchedColumn == -1 || table.Columns[table.LeftMostStretchedColumn].DisplayOrder > column.DisplayOrder {
                table.LeftMostStretchedColumn = column_n as ImGuiTableColumnIdx;
            }
            if table.RightMostStretchedColumn == -1 || table.Columns[table.RightMostStretchedColumn].DisplayOrder < column.DisplayOrder {
                table.RightMostStretchedColumn = column_n as ImGuiTableColumnIdx;
            }
        }
        column.IsPreserveWidthAuto = false;
        sum_width_requests += table.CellPaddingX * 2.0;
    }
    table.ColumnsEnabledFixedCount = count_fixed as ImGuiTableColumnIdx;
    table.ColumnsStretchSumWeights = stretch_sum_weights;

    // [Part 4] Apply final widths based on requested widths
    let mut work_rect: ImRect =  table.WorkRect;
    let width_spacings: c_float =  (table.OuterPaddingX * 2.0) + (table.CellSpacingX1 + table.CellSpacingX2) * (table.ColumnsEnabledCount - 1);
    let width_avail: c_float =  if flag_set(table.Flags , ImGuiTableFlags_ScrollX) && table.InnerWidth == 0.0 { table.InnerClipRect.GetWidth() } else { work_rect.GetWidth() };
    let width_avail_for_stretched_columns: c_float =  width_avail - width_spacings - sum_width_requests;
    let mut width_remaining_for_stretched_columns: c_float =  width_avail_for_stretched_columns;
    table.ColumnsGivenWidth = width_spacings + (table.CellPaddingX * 2.0) * table.ColumnsEnabledCount;
    // for (let column_n: c_int = 0; column_n < table.ColumnsCount; column_n++)
    for column_n in 0 .. table.ColumnsCount
    {
        if (!(table.EnabledMaskByIndex & (1 << column_n))) {
            continue;
        }
        let mut column: *mut ImGuiTableColumn = &mut table.Columns[column_n];

        // Allocate width for stretched/weighted columns (StretchWeight gets converted into WidthRequest)
        if flag_set(column.Flags , ImGuiTableColumnFlags_WidthStretch)
        {
            let weight_ratio: c_float =  column.StretchWeight / stretch_sum_weights;
            column.WidthRequest = IM_FLOOR(ImMax(width_avail_for_stretched_columns * weight_ratio, table.MinColumnWidth) + 0.010f32);
            width_remaining_for_stretched_columns -= column.WidthRequest;
        }

        // [Resize Rule 1] The right-most Visible column is not resizable if there is at least one Stretch column
        // See additional comments in TableSetColumnWidth().
        if (column.NextEnabledColumn == -1 && table.LeftMostStretchedColumn != -1) {
            column.Flags |= ImGuiTableColumnFlags_NoDirectResize_;
        }

        // Assign final width, record width in case we will need to shrink
        column.WidthGiven = ImFloor(ImMax(column.WidthRequest, table.MinColumnWidth));
        table.ColumnsGivenWidth += column.WidthGiven;
    }

    // [Part 5] Redistribute stretch remainder width due to rounding (remainder width is < 1.0 * number of Stretch column).
    // Using right-to-left distribution (more likely to match resizing cursor).
    if (width_remaining_for_stretched_columns >= 1.0 && flag_clear(table.Flags , ImGuiTableFlags_PreciseWidths))
    {
        // for (let order_n: c_int = table.ColumnsCount -1; stretch_sum_weights > 0.0 &&width_remaining_for_stretched_columns > = 1.0 &&order_n > = 0; order_n - -)
        let mut order_n = table.ColumnsCount - 1;
        while stretch_sum_weights > 0.0 && width_remaining_for_stretched_columns >= 1.0 && order_n >= 0
        {
            if (!(table.EnabledMaskByDisplayOrder & (1 << order_n))) {
                continue;
            }
            let column: *mut ImGuiTableColumn = &mut table.Columns[table.DisplayOrderToIndex[order_n]];
            if (flag_clear(column.Flags, ImGuiTableColumnFlags_WidthStretch)) {
                continue;
            }
            column.WidthRequest += 1.0;
            column.WidthGiven += 1.0;
            width_remaining_for_stretched_columns -= 1.0;
            order_n -= 1;
        }
    }

    let mut table_instance: *mut ImGuiTableInstanceData = TableGetInstanceData(table, table.InstanceCurrent);
    table.HoveredColumnBody = -1;
    table.HoveredColumnBorder = -1;
    let mut mouse_hit_rect: ImRect = ImRect::new(table.OuterRect.min.x, table.OuterRect.min.y, table.OuterRect.max.x, ImMax(table.OuterRect.max.y, table.OuterRect.min.y + table_instance.LastOuterHeight));
    let is_hovering_table: bool = ItemHoverable(&mouse_hit_rect, 0);

    // [Part 6] Setup final position, offset, skip/clip states and clipping rectangles, detect hovered column
    // Process columns in their visible orders as we are comparing the visible order and adjusting host_clip_rect while looping.
    let mut visible_n: c_int = 0;
    let mut offset_x_frozen: bool =  (table.FreezeColumnsCount > 0);
    let mut offset_x: c_float =  (if table.FreezeColumnsCount > 0 { table.OuterRect.min.x } else { work_rect.min.x }) + table.OuterPaddingX - table.CellSpacingX1;
    let mut host_clip_rect: ImRect =  table.InnerClipRect;
    //host_clip_rect.Max.x += table.CellPaddingX + table.CellSpacingX2;
    table.VisibleMaskByIndex = 0x00;
    table.RequestOutputMaskByIndex = 0x00;
    // for (let order_n: c_int = 0; order_n < table.ColumnsCount; order_n++)
    for order_n in 0 .. table.ColumnsCount
    {
        let column_n: c_int = table.DisplayOrderToIndex[order_n];
        let mut column: *mut ImGuiTableColumn = &mut table.Columns[column_n];

        column.NavLayerCurrent = if table.FreezeRowsCount > 0 || column_n < table.FreezeColumnsCount as c_int { ImGuiNavLayer_Menu} else { ImGuiNavLayer_Main};

        if offset_x_frozen && table.FreezeColumnsCount == visible_n as ImGuiTableColumnIdx
        {
            offset_x += work_rect.min.x - table.OuterRect.min.x;
            offset_x_frozen = false;
        }

        // Clear status flags
        column.Flags &= !ImGuiTableColumnFlags_StatusMask_;

        if (table.EnabledMaskByDisplayOrder & (1 << order_n)) == 0
        {
            // Hidden column: clear a few fields and we are done with it for the remainder of the function.
            // We set a zero-width clip rect but set Min.y/Max.y properly to not interfere with the clipper.
            column.MinX = offset_x;column.MaxX = offset_x;column.WorkMinX = offset_x;column.ClipRect.min.x = offset_x;column.ClipRect.max.x = offset_x;
            column.WidthGiven = 0.0;
            column.ClipRect.min.y = work_rect.min.y;
            column.ClipRect.max.y = f32::MAX;
            column.ClipRect.ClipWithFull(&host_clip_rect);
            column.IsVisibleX = false; column.IsVisibleY = false; column.IsRequestOutput = false;
            column.IsSkipItems = true;
            column.ItemWidth = 1.0;
            continue;
        }

        // Detect hovered column
        if is_hovering_table && g.IO.MousePos.x >= column.ClipRect.min.x && g.IO.MousePos.x < column.ClipRect.max.x{
            table.HoveredColumnBody = column_n as ImGuiTableColumnIdx;}

        // Lock start position
        column.MinX = offset_x;

        // Lock width based on start position and minimum/maximum width for this position
        let max_width: c_float =  TableGetMaxColumnWidth(table, column_n);
        column.WidthGiven = column.WidthGiven.min(max_width);
        column.WidthGiven = ImMax(column.WidthGiven, column.WidthRequest.min(table.MinColumnWidth));
        column.MaxX = offset_x + column.WidthGiven + table.CellSpacingX1 + table.CellSpacingX2 + table.CellPaddingX * 2.0;

        // Lock other positions
        // - ClipRect.Min.x: Because merging draw commands doesn't compare min boundaries, we make ClipRect.Min.x match left bounds to be consistent regardless of merging.
        // - ClipRect.Max.x: using WorkMaxX instead of MaxX (aka including padding) makes things more consistent when resizing down, tho slightly detrimental to visibility in very-small column.
        // - ClipRect.Max.x: using MaxX makes it easier for header to receive hover highlight with no discontinuity and display sorting arrow.
        // - FIXME-TABLE: We want equal width columns to have equal (ClipRect.Max.x - WorkMinX) width, which means ClipRect.max.x cannot stray off host_clip_rect.Max.x else right-most column may appear shorter.
        column.WorkMinX = column.MinX + table.CellPaddingX + table.CellSpacingX1;
        column.WorkMaxX = column.MaxX - table.CellPaddingX - table.CellSpacingX2; // Expected max
        column.ItemWidth = ImFloor(column.WidthGiven * 0.650f32);
        column.ClipRect.min.x = column.MinX;
        column.ClipRect.min.y = work_rect.min.y;
        column.ClipRect.max.x = column.MaxX; //column.WorkMaxX;
        column.ClipRect.max.y = f32::MAX;
        column.ClipRect.ClipWithFull(&host_clip_rect);

        // Mark column as Clipped (not in sight)
        // Note that scrolling tables (where inner_window != outer_window) handle Y clipped earlier in BeginTable() so IsVisibleY really only applies to non-scrolling tables.
        // FIXME-TABLE: Because InnerClipRect.Max.y is conservatively ==outer_window.ClipRect.Max.y, we never can mark columns _Above_ the scroll line as not IsVisibleY.
        // Taking advantage of LastOuterHeight would yield good results there...
        // FIXME-TABLE: Y clipping is disabled because it effectively means not submitting will reduce contents width which is fed to outer_window.DC.CursorMaxPos.x,
        // and this may be used (e.g. typically by outer_window using AlwaysAutoResize or outer_window's horizontal scrollbar, but could be something else).
        // Possible solution to preserve last known content width for clipped column. Test 'table_reported_size' fails when enabling Y clipping and window is resized small.
        column.IsVisibleX = (column.ClipRect.max.x > column.ClipRect.min.x);
        column.IsVisibleY = true; // (column.ClipRect.Max.y > column.ClipRect.Min.y);
        let is_visible: bool = column.IsVisibleX; //&& column.IsVisibleY;
        if is_visible {
            table.VisibleMaskByIndex |= (1 << column_n);
        }

        // Mark column as requesting output from user. Note that fixed + non-resizable sets are auto-fitting at all times and therefore always request output.
        column.IsRequestOutput = is_visible || column.AutoFitQueue != 0 || column.CannotSkipItemsQueue != 0;
        if column.IsRequestOutput {
            table.RequestOutputMaskByIndex |= (1 << column_n);
        }

        // Mark column as SkipItems (ignoring all items/layout)
        column.IsSkipItems = !column.IsEnabled || table.HostSkipItems;
        if column.IsSkipItems {}
            // IM_ASSERT(!is_visible);

        // Update status flags
        column.Flags |= ImGuiTableColumnFlags_IsEnabled;
        if is_visible {
            column.Flags |= ImGuiTableColumnFlags_IsVisible;
        }
        if column.SortOrder != -1 {
            column.Flags |= ImGuiTableColumnFlags_IsSorted;
        }
        if table.HoveredColumnBody == column_n as ImGuiTableColumnIdx {
            column.Flags |= ImGuiTableColumnFlags_IsHovered;
        }

        // Alignment
        // FIXME-TABLE: This align based on the whole column width, not per-cell, and therefore isn't useful in
        // many cases (to be able to honor this we might be able to store a log of cells width, per row, for
        // visible rows, but nav/programmatic scroll would have visible artifacts.)
        //if flag_set(column.Flags, ImGuiTableColumnFlags_AlignRight)
        //    column.WorkMinX = ImMax(column.WorkMinX, column.MaxX - column.ContentWidthRowsUnfrozen);
        //else if flag_set(column.Flags, ImGuiTableColumnFlags_AlignCenter)
        //    column.WorkMinX = ImLerp(column.WorkMinX, ImMax(column.StartX, column.MaxX - column.ContentWidthRowsUnfrozen), 0.5);

        // Reset content width variables
        column.ContentMaxXFrozen = column.WorkMinX;
        column.ContentMaxXUnfrozen = column.WorkMinX;
        column.ContentMaxXHeadersUsed = column.WorkMinX;
        column.ContentMaxXHeadersIdeal = column.WorkMinX;

        // Don't decrement auto-fit counters until container window got a chance to submit its items
        if table.HostSkipItems == false
        {
            column.AutoFitQueue >>= 1;
            column.CannotSkipItemsQueue >>= 1;
        }

        if (visible_n < table.FreezeColumnsCount as c_int) {
            host_clip_rect.min.x = ImClamp(column.MaxX + TABLE_BORDER_SIZE, host_clip_rect.min.x, host_clip_rect.max.x);
        }

        offset_x += column.WidthGiven + table.CellSpacingX1 + table.CellSpacingX2 + table.CellPaddingX * 2.0;
        visible_n+= 1;
    }

    // [Part 7] Detect/store when we are hovering the unused space after the right-most column (so e.g. context menus can react on it)
    // Clear Resizable flag if none of our column are actually resizable (either via an explicit _NoResize flag, either
    // because of using _WidthAuto/_WidthStretch). This will hide the resizing option from the context menu.
    let unused_x1: c_float =  ImMax(table.WorkRect.min.x, table.Columns[table.RightMostEnabledColumn].ClipRect.Max.x);
    if (is_hovering_table && table.HoveredColumnBody == -1)
    {
        if g.IO.MousePos.x >= unused_x1{
            table.HoveredColumnBody = table.ColumnsCount as ImGuiTableColumnIdx;}
    }
    if has_resizable == false && flag_set(table.Flags , ImGuiTableFlags_Resizable) {
        table.Flags &= !ImGuiTableFlags_Resizable;
    }

    // [Part 8] Lock actual OuterRect/WorkRect right-most position.
    // This is done late to handle the case of fixed-columns tables not claiming more widths that they need.
    // Because of this we are careful with uses of WorkRect and InnerClipRect before this point.
    if table.RightMostStretchedColumn != -1 {
        table.Flags &= !ImGuiTableFlags_NoHostExtendX;
    }
    if table.Flags & ImGuiTableFlags_NoHostExtendX
    {
        table.OuterRect.max.x = unused_x1;
        table.WorkRect.max.x = unused_x1;
        table.InnerClipRect.max.x = table.InnerClipRect.max.x.min( unused_x1);
    }
    table.Innerwindow.ParentWorkRect = table.WorkRect;
    table.BorderX1 = table.InnerClipRect.min.x;// +((table.Flags & ImGuiTableFlags_BordersOuter) ? 0.0 : -1.0);
    table.BorderX2 = table.InnerClipRect.max.x;// +((table.Flags & ImGuiTableFlags_BordersOuter) ? 0.0 : +1.0);

    // [Part 9] Allocate draw channels and setup background cliprect
    TableSetupDrawChannels(table);

    // [Part 10] Hit testing on borders
    if table.Flags & ImGuiTableFlags_Resizable{
        TableUpdateBorders(table);}
    table_instance.LastFirstRowHeight = 0.0;
    table.IsLayoutLocked = true;
    table.IsUsingHeaders = false;

    // [Part 11] Context menu
    if TableBeginContextMenuPopup(table)
    {
        TableDrawContextMenu(table);
        EndPopup(g);
    }

    // [Part 13] Sanitize and build sort specs before we have a change to use them for display.
    // This path will only be exercised when sort specs are modified before header rows (e.g. init or visibility change)
    if table.IsSortSpecsDirty && flag_set(table.Flags , ImGuiTableFlags_Sortable){
        TableSortSpecsBuild(table);}

    // Initial state
    inner_window: &mut ImguiWindow = table.InnerWindow;
    if table.Flags & ImGuiTableFlags_NoClip {
        table.DrawSplitter.SetCurrentChannel(inner_window.DrawList, TABLE_DRAW_CHANNEL_NOCLIP);
    }
    else {
        inner_window.DrawList.PushClipRect(inner_window.ClipRect.Min, inner_window.ClipRect.Max, false);
    }
}

// Process hit-testing on resizing borders. Actual size change will be applied in EndTable()
// - Set table.HoveredColumnBorder with a short delay/timer to reduce feedback noise
// - Submit ahead of table contents and header, use ImGuiButtonFlags_AllowItemOverlap to prioritize widgets
//   overlapping the same area.
pub unsafe fn TableUpdateBorders(table: *mut ImGuiTable)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(table.Flags & ImGuiTableFlags_Resizable);

    // At this point OuterRect height may be zero or under actual final height, so we rely on temporal coherency and
    // use the final height from last frame. Because this is only affecting _interaction_ with columns, it is not
    // really problematic (whereas the actual visual will be displayed in EndTable() and using the current frame height).
    // Actual columns highlight/render will be performed in EndTable() and not be affected.
    table_instance: *mut ImGuiTableInstanceData = TableGetInstanceData(table, table.InstanceCurrent);
    let hit_half_width: c_float =  TABLE_RESIZE_SEPARATOR_HALF_THICKNESS;
    let hit_y1: c_float =  table.OuterRect.min.y;
    let hit_y2_body: c_float =  ImMax(table.OuterRect.max.y, hit_y1 + table_instance.LastOuterHeight);
    let hit_y2_head: c_float =  hit_y1 + table_instance.LastFirstRowHeight;

    // for (let order_n: c_int = 0; order_n < table.ColumnsCount; order_n++)
   for order_n in 0 .. table.ColumnsCount
    {
        if !(table.EnabledMaskByDisplayOrder & (1 << order_n)) {
            continue;
        }

        let column_n: c_int = table.DisplayOrderToIndex[order_n];
        let mut column: *mut ImGuiTableColumn = &mut table.Columns[column_n];
        if column.Flags & (ImGuiTableColumnFlags_NoResize | ImGuiTableColumnFlags_NoDirectResize_){
            continue;}

        // ImGuiTableFlags_NoBordersInBodyUntilResize will be honored in TableDrawBorders()
        let border_y2_hit: c_float =  if table.Flags & ImGuiTableFlags_NoBordersInBody { hit_y2_head }else { hit_y2_body };
        if flag_set(table.Flags ,ImGuiTableFlags_NoBordersInBody) && table.IsUsingHeaders == false{
            continue;}

        if !column.IsVisibleX && table.LastResizedColumn != column_n as ImGuiTableColumnIdx {
            continue;
        }

        let mut column_id: ImguiHandle =  TableGetColumnResizeID(table, column_n, table.InstanceCurrent);
        let mut hit_rect: ImRect = ImRect::new(column.MaxX - hit_half_width, hit_y1, column.MaxX + hit_half_width, border_y2_hit);
        //GetForegroundDrawList().AddRect(hit_rect.Min, hit_rect.Max, IM_COL32(255, 0, 0, 100));
        KeepAliveID(g, column_id);

        let mut hovered: bool =  false;
        let mut held = false;
        let mut pressed: bool =  ButtonBehavior(hit_rect, column_id, &hovered, &held, ImGuiButtonFlags_FlattenChildren | ImGuiButtonFlags_AllowItemOverlap | ImGuiButtonFlags_PressedOnClick | ImGuiButtonFlags_PressedOnDoubleClick | ImGuiButtonFlags_NoNavFocus);
        if pressed && IsMouseDoubleClicked(0)
        {
            TableSetColumnWidthAutoSingle(table, column_n);
            ClearActiveID(g);
            held = false;
            hovered = false;
        }
        if (held)
        {
            if (table.LastResizedColumn == -1) {
                table.ResizeLockMinContentsX2 = if table.RightMostEnabledColumn != -1 { table.Columns[table.RightMostEnabledColumn].MaxX } else { -f32::MAX };
            }
            table.ResizedColumn = column_n as ImGuiTableColumnIdx;
            table.InstanceInteracted = table.InstanceCurrent as i16;
        }
        if (hovered && g.HoveredIdTimer > TABLE_RESIZE_SEPARATOR_FEEDBACK_TIMER) || held
        {
            table.HoveredColumnBorder = column_n as ImGuiTableColumnIdx;
            SetMouseCursor(ImGuiMouseCursor_ResizeEW);
        }
    }
}

pub unsafe fn  EndTable()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut table: *mut ImGuiTable = g.CurrentTable;
    // IM_ASSERT(table != NULL && "Only call EndTable() if BeginTable() returns true!");

    // This assert would be very useful to catch a common error... unfortunately it would probably trigger in some
    // cases, and for consistency user may sometimes output empty tables (and still benefit from e.g. outer border)
    //IM_ASSERT(table.IsLayoutLocked && "Table unused: never called TableNextRow(), is that the intent?");

    // If the user never got to call TableNextRow() or TableNextColumn(), we call layout ourselves to ensure all our
    // code paths are consistent (instead of just hoping that TableBegin/TableEnd will work), get borders drawn, etc.
    if (!table.IsLayoutLocked) {
        TableUpdateLayout(table);
    }

    let flags: ImGuiTableFlags = table.Flags;
    let mut inner_window: &mut ImguiWindow = table.InnerWindow;
    let mut outer_window: &mut ImguiWindow = table.OuterWindow;
   let mut temp_data: *mut ImGuiTableTempData = table.TempData;
    // IM_ASSERT(inner_window == g.CurrentWindow);
    // IM_ASSERT(outer_window == inner_window || outer_window == inner_window.ParentWindow);

    if table.IsInsideRow{
        TableEndRow(table);}

    // Context menu in columns body
    if flag_set(flags , ImGuiTableFlags_ContextMenuInBody) {
        if (table.HoveredColumnBody != -1 && !IsAnyItemHovered() && IsMouseReleased(ImGuiMouseButton_Right)) {
            TableOpenContextMenu(table.HoveredColumnBody as c_int);
        }
    }

    // Finalize table height
    table_instance: *mut ImGuiTableInstanceData = TableGetInstanceData(table, table.InstanceCurrent);
    inner_window.dc.prev_line_size = temp_data.HostBackupPrevLineSize;
    inner_window.dc.curr_line_size = temp_data.HostBackupCurrLineSize;
    inner_window.dc.CursorMaxPos = temp_data.HostBackupCursorMaxPos;
    let inner_content_max_y: c_float =  table.RowPosY2;
    // IM_ASSERT(table.RowPosY2 == inner_window.dc.cursor_pos.y);
    if inner_window != outer_window {
        inner_window.dc.CursorMaxPos.y = inner_content_max_y;
    }
    else if flag_clear(flags, ImGuiTableFlags_NoHostExtendY) {
        table.OuterRect.max.y = ImMax(table.OuterRect.max.y, inner_content_max_y);
        table.InnerRect.max.y = ImMax(table.OuterRect.max.y, inner_content_max_y);
    }// Patch OuterRect/InnerRect height
    table.WorkRect.max.y = ImMax(table.WorkRect.max.y, table.OuterRect.max.y);
    table_instance.LastOuterHeight = table.OuterRect.GetHeight();

    // Setup inner scrolling range
    // FIXME: This ideally should be done earlier, in BeginTable() SetNextWindowContentSize call, just like writing to inner_window.DC.CursorMaxPos.y,
    // but since the later is likely to be impossible to do we'd rather update both axises together.
    if flag_set(table.Flags , ImGuiTableFlags_ScrollX)
    {
        let outer_padding_for_border: c_float =  if flag_set(table.Flags , ImGuiTableFlags_BordersOuterV) { TABLE_BORDER_SIZE } else{ 0.0};
        let mut max_pos_x: c_float =  table.Innerwindow.DC.CursorMaxPos.x;
        if table.RightMostEnabledColumn != -1 {
            max_pos_x = ImMax(max_pos_x, table.Columns[table.RightMostEnabledColumn].WorkMaxX + table.CellPaddingX + table.OuterPaddingX - outer_padding_for_border);
        }
        if table.ResizedColumn != -1 {
            max_pos_x = ImMax(max_pos_x, table.ResizeLockMinContentsX2);
        }
        table.Innerwindow.DC.CursorMaxPos.x = max_pos_x;
    }

    // Pop clipping rect
    if flag_clear(flags, ImGuiTableFlags_NoClip) {
        inner_window.DrawList.PopClipRect();
    }
    inner_window.ClipRect = inner_window.DrawList._ClipRectStack.last().unwrap().clone();

    // Draw borders
    if flag_set(flags, ImGuiTableFlags_Borders) {
        TableDrawBorders(table);
    }

// #if 0
    // Strip out dummy channel draw calls
    // We have no way to prevent user submitting direct ImDrawList calls into a hidden column (but  calls will be clipped out)
    // Pros: remove draw calls which will have no effect. since they'll have zero-size cliprect they may be early out anyway.
    // Cons: making it harder for users watching metrics/debugger to spot the wasted vertices.
    // if table.DummyDrawChannel != -1
    // {
    //     dummy_channel: *mut ImDrawChannel = &table.DrawSplitter._Channels[table.DummyDrawChannel];
    //     dummy_channel._CmdBuffer.clear();
    //     dummy_channel._IdxBuffer.clear();
    // }
// #endif

    // Flatten channels and merge draw calls
    splitter: *mut ImDrawListSplitter = table.DrawSplitter;
    splitter.SetCurrentChannel(inner_window.DrawList, 0);
    if flag_clear(table.Flags , ImGuiTableFlags_NoClip) {
        TableMergeDrawChannels(table);}
    splitter.Merge(inner_window.DrawList);

    // Update ColumnsAutoFitWidth to get us ahead for host using our size to auto-resize without waiting for next BeginTable()
    let mut auto_fit_width_for_fixed: c_float =  0.0;
    let mut auto_fit_width_for_stretched: c_float =  0.0;
    let mut auto_fit_width_for_stretched_min: c_float =  0.0;
    // for (let column_n: c_int = 0; column_n < table.ColumnsCount; column_n++)
    for column_n in 0 .. table.ColumnsCount
    {
        if table.EnabledMaskByIndex & (1 << column_n) {
            let mut column: *mut ImGuiTableColumn = &mut table.Columns[column_n];
            let column_width_request: c_float = if flag_set(column.Flags, ImGuiTableColumnFlags_WidthFixed) && flag_clear(column.Flags, ImGuiTableColumnFlags_NoResize) { column.WidthRequest } else { TableGetColumnWidthAuto(table, column) };
            if column.Flags & ImGuiTableColumnFlags_WidthFixed {
                auto_fit_width_for_fixed += column_width_request;
            }
            else {
                auto_fit_width_for_stretched += column_width_request;
            }
            if flag_set(column.Flags, ImGuiTableColumnFlags_WidthStretch) && flag_set(column.Flags, ImGuiTableColumnFlags_NoResize) {
                auto_fit_width_for_stretched_min = ImMax(auto_fit_width_for_stretched_min, column_width_request / (column.StretchWeight / table.ColumnsStretchSumWeights));
            }
        }
    }
    let width_spacings: c_float =  (table.OuterPaddingX * 2.0) + (table.CellSpacingX1 + table.CellSpacingX2) * (table.ColumnsEnabledCount - 1);
    table.ColumnsAutoFitWidth = width_spacings + (table.CellPaddingX * 2.0) * table.ColumnsEnabledCount + auto_fit_width_for_fixed + ImMax(auto_fit_width_for_stretched, auto_fit_width_for_stretched_min);

    // Update scroll
    if flag_clear(table.Flags , ImGuiTableFlags_ScrollX) && inner_window != outer_window
    {
        inner_window.scroll.x = 0.0;
    }
    else if table.LastResizedColumn != -1 && table.ResizedColumn == -1 && inner_window.scrollbarX && table.InstanceInteracted == table.InstanceCurrent as i16
    {
        // When releasing a column being resized, scroll to keep the resulting column in sight
        let neighbor_width_to_keep_visible: c_float =  table.MinColumnWidth + table.CellPaddingX * 2.0;
        let mut column: *mut ImGuiTableColumn = &mut table.Columns[table.LastResizedColumn];
        if column.MaxX < table.InnerClipRect.min.x {
            SetScrollFromPosX(inner_window, column.MaxX - inner_window.position.x - neighbor_width_to_keep_visible, 1.0);
        }
        else if column.MaxX > table.InnerClipRect.max.x {
            SetScrollFromPosX(inner_window, column.MaxX - inner_window.position.x + neighbor_width_to_keep_visible, 1.0);
        }
    }

    // Apply resizing/dragging at the end of the frame
    if table.ResizedColumn != -1 && table.InstanceCurrent == table.InstanceInteracted as c_int
    {
        let column: *mut ImGuiTableColumn = &mut table.Columns[table.ResizedColumn];
        let new_x2: c_float =  (g.IO.MousePos.x - g.ActiveIdClickOffset.x + TABLE_RESIZE_SEPARATOR_HALF_THICKNESS);
        let new_width: c_float =  ImFloor(new_x2 - column.MinX - table.CellSpacingX1 - table.CellPaddingX * 2.0);
        table.ResizedColumnNextWidth = new_width;
    }

    // Pop from id stack
    // IM_ASSERT_USER_ERROR(inner_window.id_stack.back() == table.ID + table.InstanceCurrent, "Mismatching PushID/PopID!");
    // IM_ASSERT_USER_ERROR(outer_window.DC.ItemWidthStack.Size >= temp_Data.HostBackupItemWidthStackSize, "Too many PopItemWidth!");
    pop_win_id_from_stack(g);

    // Restore window data that we modified
    let backup_outer_max_pos: Vector2 = outer_window.dc.CursorMaxPos;
    inner_window.work_rect = temp_data.HostBackupWorkRect;
    inner_window.ParentWorkRect = temp_data.HostBackupParentWorkRect;
    inner_window.skip_items = table.HostSkipItems;
    outer_window.dc.cursor_pos = table.OuterRect.min;
    outer_window.dc.item_width = temp_data.HostBackupItemWidth;
    outer_window.dc.ItemWidthStack.Size = temp_data.HostBackupItemWidthStackSize;
    outer_window.dc.columns_offset = temp_data.HostBackupColumnsOffset.clone();

    // Layout in outer window
    // (FIXME: To allow auto-fit and allow desirable effect of SameLine() we dissociate 'used' vs 'ideal' size by overriding
    // CursorPosPrevLine and CursorMaxPos manually. That should be a more general layout feature, see same problem e.g. #3414)
    if inner_window != outer_window
    {
        EndChild();
    }
    else
    {
        ItemSize(g, &table.OuterRect.GetSize(), 0.0);
        ItemAdd(g, &mut table.OuterRect, 0, None, 0);
    }

    // Override declared contents width/height to enable auto-resize while not needlessly adding a scrollbar
    if flag_set(table.Flags , ImGuiTableFlags_NoHostExtendX)
    {
        // FIXME-TABLE: Could we remove this section?
        // ColumnsAutoFitWidth may be one frame ahead here since for Fixed+NoResize is calculated from latest contents
        // IM_ASSERT((table.Flags & ImGuiTableFlags_ScrollX) == 0);
        outer_window.dc.CursorMaxPos.x = ImMax(backup_outer_max_pos.x, table.OuterRect.min.x + table.ColumnsAutoFitWidth);
    }
    else if temp_data.UserOuterSize.x <= 0.0
    {
        let decoration_size: c_float =  if flag_set(table.Flags , ImGuiTableFlags_ScrollX) { inner_window.scrollbarSizes.x} else {0.0};
        outer_window.dc.IdealMaxPos.x = ImMax(outer_window.dc.IdealMaxPos.x, table.OuterRect.min.x + table.ColumnsAutoFitWidth + decoration_size - temp_data.UserOuterSize.x);
        outer_window.dc.CursorMaxPos.x = ImMax(backup_outer_max_pos.x, table.OuterRect.max.x.min( table.OuterRect.min.x + table.ColumnsAutoFitWidth));
    }
    else
    {
        outer_window.dc.CursorMaxPos.x = ImMax(backup_outer_max_pos.x, table.OuterRect.max.x);
    }
    if temp_data.UserOuterSize.y <= 0.0
    {
        let decoration_size: c_float =  if flag_set(table.Flags, ImGuiTableFlags_ScrollY) { inner_window.scrollbarSizes.y} else{ 0.0};
        outer_window.dc.IdealMaxPos.y = ImMax(outer_window.dc.IdealMaxPos.y, inner_content_max_y + decoration_size - temp_data.UserOuterSize.y);
        outer_window.dc.CursorMaxPos.y = ImMax(backup_outer_max_pos.y, table.OuterRect.max.y.min( inner_content_max_y));
    }
    else
    {
        // OuterRect.Max.y may already have been pushed downward from the initial value (unless ImGuiTableFlags_NoHostExtendY is set)
        outer_window.dc.CursorMaxPos.y = ImMax(backup_outer_max_pos.y, table.OuterRect.max.y);
    }

    // Save settings
    if table.IsSettingsDirty{
        TableSaveSettings(table);}
    table.IsInitializing = false;

    // Clear or restore current table, if any
    // IM_ASSERT(g.CurrentWindow == outer_window && g.CurrentTable == table);
    // IM_ASSERT(g.TablesTempDataStacked > 0);
    g.TablesTempDataStacked -= 1;
    temp_data = if g.TablesTempDataStacked > 0 { &mut g.TablesTempData[g.TablesTempDataStacked - 1]} else { None};
    g.CurrentTable = if temp_data { g.Tables.GetByIndex(temp_data.TableIndex) }else {None};
    if g.CurrentTable
    {
        g.Currenttable.TempData = temp_data;
        g.Currenttable.DrawSplitter = &temp_data.DrawSplitter;
    }
    outer_window.dc.CurrentTableIdx = if g.CurrentTable { g.Tables.GetIndex(g.CurrentTable)} else {- 1};
}

// See "COLUMN SIZING POLICIES" comments at the top of this file
// If (init_width_or_weight <= 0.0) it is ignored
pub unsafe fn TableSetupColumn(label: String, mut flags: ImGuiTableColumnFlags, init_width_or_weight: c_float, user_id: ImguiHandle)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut table: *mut ImGuiTable = g.CurrentTable;
    // IM_ASSERT(table != NULL && "Need to call TableSetupColumn() after BeginTable()!");
    // IM_ASSERT(table.IsLayoutLocked == false && "Need to call call TableSetupColumn() before first row!");
    // IM_ASSERT(flag_set(flags, ImGuiTableColumnFlags_StatusMask_) == 0 && "Illegal to pass StatusMask values to TableSetupColumn()");
    if (table.DeclColumnsCount >= table.ColumnsCount as ImGuiTableColumnIdx)
    {
        // IM_ASSERT_USER_ERROR(table.DeclColumnsCount < table.ColumnsCount, "Called TableSetupColumn() too many times!");
        return;
    }

    let column: *mut ImGuiTableColumn = &mut table.Columns[table.DeclColumnsCount];
    table.DeclColumnsCount+= 1;

    // Assert when passing a width or weight if policy is entirely left to default, to avoid storing width into weight and vice-versa.
    // Give a grace to users of ImGuiTableFlags_ScrollX.
    if table.IsDefaultSizingPolicy && flag_clear(flags, ImGuiTableColumnFlags_WidthMask_) && flag_clear(flags, ImGuiTableFlags_ScrollX) {}
        // IM_ASSERT(init_width_or_weight <= 0.0 && "Can only specify width/weight if sizing policy is set explicitly in either Table or Column.");

    // When passing a width automatically enforce WidthFixed policy
    // (whereas TableSetupColumnFlags would default to WidthAuto if table is not Resizable)
    if flag_clear(flags, ImGuiTableColumnFlags_WidthMask_) && init_width_or_weight > 0.0 {
        if (table.Flags & ImGuiTableFlags_SizingMask_) == ImGuiTableFlags_SizingFixedFit || (table.Flags & ImGuiTableFlags_SizingMask_) == ImGuiTableFlags_SizingFixedSame {
            flags |= ImGuiTableColumnFlags_WidthFixed;
        }
    }

    TableSetupColumnFlags(table, column, flags);
    column.UserID = user_id;
    flags = column.Flags;

    // initialize defaults
    column.InitStretchWeightOrWidth = init_width_or_weight;
    if table.IsInitializing
    {
        // Init width or weight
        if column.WidthRequest < 0.0 && column.StretchWeight < 0.0
        {
            if flag_set(flags , ImGuiTableColumnFlags_WidthFixed) && init_width_or_weight > 0.0{
                column.WidthRequest = init_width_or_weight;}
            if flag_set(flags , ImGuiTableColumnFlags_WidthStretch) {
                column.StretchWeight = if init_width_or_weight > 0.0 { init_width_or_weight } else { -1.0 };
            }

            // Disable auto-fit if an explicit width/weight has been specified
            if init_width_or_weight > 0.0{
                column.AutoFitQueue = 0x00;}
        }

        // Init default visibility/sort state
        if flag_set(flags , ImGuiTableColumnFlags_DefaultHide) && flag_clear(table.SettingsLoadedFlags , ImGuiTableFlags_Hideable){
            column.IsUserEnabled = false;column.IsUserEnabledNextFrame = false;}
        if flag_set(flags , ImGuiTableColumnFlags_DefaultSort) && flag_clear(table.SettingsLoadedFlags , ImGuiTableFlags_Sortable)
        {
            column.SortOrder = 0; // Multiple columns using _DefaultSort will be reassigned unique SortOrder values when building the sort specs.
            column.SortDirection = if column.Flags & ImGuiTableColumnFlags_PreferSortDescending { ImGuiSortDirection_Descending} else { (ImGuiSortDirection_Ascending)};
        }
    }

    // Store name (append with zero-terminator in contiguous buffer)
    column.NameOffset = -1;
    if label != None && label[0] != 0
    {
        column.NameOffset = table.ColumnsNames.size() as i16;
        table.ColumnsNames.append(label, label + strlen(label) + 1);
    }
}

// [Public]
pub unsafe fn TableSetupScrollFreeze(columns: c_int, rows: c_int)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
     let table: *mut ImGuiTable = g.CurrentTable;
    // IM_ASSERT(table != NULL && "Need to call TableSetupColumn() after BeginTable()!");
    // IM_ASSERT(table.IsLayoutLocked == false && "Need to call TableSetupColumn() before first row!");
    // IM_ASSERT(columns >= 0 && columns < IMGUI_TABLE_MAX_COLUMNS);
    // IM_ASSERT(rows >= 0 && rows < 128); // Arbitrary limit

    table.FreezeColumnsRequest = if table.Flags & ImGuiTableFlags_ScrollX { ImMin(columns, table.ColumnsCount)} else { 0};
    table.FreezeColumnsCount = if table.Innerwindow.scroll.x != 0.0 { table.FreezeColumnsRequest} else { 0};
    table.FreezeRowsRequest = if table.Flags & ImGuiTableFlags_ScrollY { rows } else { 0 } as ImGuiTableColumnIdx;
    table.FreezeRowsCount = if table.Innerwindow.scroll.y != 0.0 { table.FreezeRowsRequest} else { 0};
    table.IsUnfrozenRows = (table.FreezeRowsCount == 0); // Make sure this is set before TableUpdateLayout() so ImGuiListClipper can benefit from it.b

    // Ensure frozen columns are ordered in their section. We still allow multiple frozen columns to be reordered.
    // FIXME-TABLE: This work for preserving 2143 into 21|43. How about 4321 turning into 21|43? (preserve relative order in each section)
    // for (let column_n: c_int = 0; column_n < table.FreezeColumnsRequest; column_n++)
    for column_n in 0 .. table.FreezeColumnsRequest
    {
        let order_n: c_int = table.DisplayOrderToIndex[column_n];
        if order_n != column_n as c_int && order_n >= table.FreezeColumnsRequest as c_int
        {
            ImSwap(table.Columns[table.DisplayOrderToIndex[order_n]].DisplayOrder, table.Columns[table.DisplayOrderToIndex[column_n]].DisplayOrder);
            ImSwap(table.DisplayOrderToIndex[order_n], table.DisplayOrderToIndex[column_n]);
        }
    }
}

//-----------------------------------------------------------------------------
// [SECTION] Tables: Simple accessors
//-----------------------------------------------------------------------------
// - TableGetColumnCount()
// - TableGetColumnName()
// - TableGetColumnName() [Internal]
// - TableSetColumnEnabled()
// - TableGetColumnFlags()
// - TableGetCellBgRect() [Internal]
// - TableGetColumnResizeID() [Internal]
// - TableGetHoveredColumn() [Internal]
// - TableSetBgColor()
//-----------------------------------------------------------------------------

pub unsafe fn TableGetColumnCount() -> c_int
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut table: *mut ImGuiTable = g.CurrentTable;
    return if table { table.ColumnsCount} else {0};
}

pub unsafe fn TableGetColumnName(mut column_n: c_int) -> *const c_char
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut table: *mut ImGuiTable = g.CurrentTable;
    if (!table) {
        return None;
    }
    if column_n < 0{
        column_n = table.CurrentColumn;}
    return TableGetColumnName2(table, column_n);
}

pub unsafe fn TableGetColumnName2(table: *const ImGuiTable, column_n: c_int) -> *const c_char
{
    if table.IsLayoutLocked == false && column_n >= table.DeclColumnsCount as c_int {
        return str_to_const_c_char_ptr("");} // NameOffset is invalid at this point
    let column: *const ImGuiTableColumn = &table.Columns[column_n];
    if (column.NameOffset == -1) {
        return str_to_const_c_char_ptr("");
    }
    return &table.ColumnsNames.Buf[column.NameOffset];
}

// Change user accessible enabled/disabled state of a column (often perceived as "showing/hiding" from users point of view)
// Note that end-user can use the context menu to change this themselves (right-click in headers, or right-click in columns body with ImGuiTableFlags_ContextMenuInBody)
// - Require table to have the ImGuiTableFlags_Hideable flag because we are manipulating user accessible state.
// - Request will be applied during next layout, which happens on the first call to TableNextRow() after BeginTable().
// - For the getter you can test (TableGetColumnFlags() & ImGuiTableColumnFlags_IsEnabled) != 0.
// - Alternative: the ImGuiTableColumnFlags_Disabled is an overriding/master disable flag which will also hide the column from context menu.
pub unsafe fn TableSetColumnEnabled(mut column_n: c_int, enabled: bool)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let table: *mut ImGuiTable = g.CurrentTable;
    // IM_ASSERT(table != NULL);
    if !table { return ; }
    // IM_ASSERT(table.Flags & ImGuiTableFlags_Hideable); // See comments above
    if column_n < 0{
        column_n = table.CurrentColumn;}
    // IM_ASSERT(column_n >= 0 && column_n < table.ColumnsCount);
    let column: *mut ImGuiTableColumn = &mut table.Columns[column_n];
    column.IsUserEnabledNextFrame = enabled;
}

// We allow querying for an extra column in order to poll the IsHovered state of the right-most section
pub unsafe fn TableGetColumnFlags(mut column_n: c_int) -> ImGuiTableColumnFlags
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
     let table: *mut ImGuiTable = g.CurrentTable;
    if !table { return  ImGuiTableColumnFlags_None; }
    if column_n < 0{
        column_n = table.CurrentColumn;}
    if column_n == table.ColumnsCount {
        return if table.HoveredColumnBody == column_n as ImGuiTableColumnIdx { ImGuiTableColumnFlags_IsHovered } else { ImGuiTableColumnFlags_None };
    }
    return table.Columns[column_n].Flags;
}



// Return the resizing ID for the right-side of the given column.
pub unsafe fn TableGetColumnResizeID(table: *const ImGuiTable, column_n: c_int, instance_no: c_int) -> ImguiHandle
{
    // IM_ASSERT(column_n >= 0 && column_n < table.ColumnsCount);
    let mut id: ImguiHandle =  table.ID + 1 + (instance_no * table.ColumnsCount) + column_n;
    return id;
}

// Return -1 when table is not hovered. return columns_count if the unused space at the right of visible columns is hovered.
pub unsafe fn TableGetHoveredColumn() -> c_int
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
     let table: *mut ImGuiTable = g.CurrentTable;
    if (!table) {
        return -1;
    }
    return table.HoveredColumnBody as c_int;
}

pub unsafe fn TableSetBgColor(target: ImGuiTableBgTarget, mut color: u32, mut column_n: c_int)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
     let table: *mut ImGuiTable = g.CurrentTable;
    // IM_ASSERT(target != ImGuiTableBgTarget_None);

    if color == IM_COL32_DISABLE {
        color = 0;}

    // We cannot draw neither the cell or row background immediately as we don't know the row height at this point in time.
    match (target)
    {
    ImGuiTableBgTarget_CellBg =>
    {
        if (table.RowPosY1 > table.InnerClipRect.max.y) { // Discard
            return;
        }
        if (column_n == -1) {
            column_n = table.CurrentColumn;
        }
        if (table.VisibleMaskByIndex & (1 << column_n)) == 0 { return ; }
        if (table.RowCellDataCurrent < 0 || table.RowCellData[table.RowCellDataCurrent].Column != column_n) {
            table.RowCellDataCurrent += 1;
        }
        let cell_data: *mut ImGuiTableCellData = &mut table.RowCellData[table.RowCellDataCurrent];
        cell_data.BgColor = color;
        cell_data.Column = column_n as ImGuiTableColumnIdx;
        // break;
    }
    ImGuiTableBgTarget_RowBg0 |
    ImGuiTableBgTarget_RowBg1 =>
    {
        if table.RowPosY1 > table.InnerClipRect.max.y { // Discard
            return;
        }
        // IM_ASSERT(column_n == -1);
        let bg_idx: c_int = if target == ImGuiTableBgTarget_RowBg1 { 1} else { 0};
        table.RowBgColor[bg_idx] = color;
        // break;
    }
    _ => {}
        // IM_ASSERT(0);
    }
}

//-------------------------------------------------------------------------
// [SECTION] Tables: Row changes
//-------------------------------------------------------------------------
// - TableGetRowIndex()
// - TableNextRow()
// - TableBeginRow() [Internal]
// - TableEndRow() [Internal]
//-------------------------------------------------------------------------

// [Public] Note: for row coloring we use ->RowBgColorCounter which is the same value without counting header rows
pub unsafe fn TableGetRowIndex() -> c_int
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
     let table: *mut ImGuiTable = g.CurrentTable;
    if !table { return  0; }
    return table.CurrentRow;
}

// [Public] Starts into the first cell of a new row
pub unsafe fn TableNextRow(row_flags: ImGuiTableRowFlags,row_min_height: c_float)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
     let table: *mut ImGuiTable = g.CurrentTable;

    if !table.IsLayoutLocked {
        TableUpdateLayout(table);
    }
    if table.IsInsideRow{
        TableEndRow(table);}

    table.LastRowFlags = table.RowFlags;
    table.RowFlags = row_flags;
    table.RowMinHeight = row_min_height;
    TableBeginRow(table);

    // We honor min_row_height requested by user, but cannot guarantee per-row maximum height,
    // because that would essentially require a unique clipping rectangle per-cell.
    table.RowPosY2 += table.CellPaddingY * 2.0;
    table.RowPosY2 = ImMax(table.RowPosY2, table.RowPosY1 + row_min_height);

    // Disable output until user calls TableNextColumn()
    table.Innerwindow.skip_items = true;
}

// [Internal] Called by TableNextRow()
pub unsafe fn TableBeginRow(table: *mut ImGuiTable)
{
    let mut window: &mut ImguiWindow = table.InnerWindow;
    // IM_ASSERT(!table.IsInsideRow);

    // New row
    table.CurrentRow+= 1;
    table.CurrentColumn = -1;
    table.RowBgColor[0] =  IM_COL32_DISABLE;
    table.RowBgColor[1] = IM_COL32_DISABLE;
    table.RowCellDataCurrent = -1;
    table.IsInsideRow = true;

    // Begin frozen rows
    let mut next_y1: c_float =  table.RowPosY2;
    if table.CurrentRow == 0 && table.FreezeRowsCount > 0{
        next_y1 =  table.OuterRect.min.y;window.dc.cursor_pos.y = table.OuterRect.min.y;}

    table.RowPosY1 =next_y1; table.RowPosY2 = next_y1;
    table.RowTextBaseline = 0.0;
    table.RowIndentOffsetX = window.dc.indent.x - table.HostIndentX; // Lock indent
    window.dc.prev_line_text_base_offset = 0.0;
    window.dc.curr_line_size = Vector2::from_floats(0.0, 0.0);
    window.dc.is_same_line = window.dc.is_set_pos = false;
    window.dc.CursorMaxPos.y = next_y1;

    // Making the header BG color non-transparent will allow us to overlay it multiple times when handling smooth dragging.
    if flag_set(table.RowFlags , ImGuiTableRowFlags_Headers)
    {
        TableSetBgColor(ImGuiTableBgTarget_RowBg0, GetColorU32(ImGuiCol_TableHeaderBg, 0.0), 0);
        if table.CurrentRow == 0{
            table.IsUsingHeaders = true;}
    }
}

//-------------------------------------------------------------------------
// [SECTION] Tables: Columns changes
//-------------------------------------------------------------------------
// - TableGetColumnIndex()
// - TableSetColumnIndex()
// - TableNextColumn()
// - TableBeginCell() [Internal]
// - TableEndCell() [Internal]
//-------------------------------------------------------------------------

pub unsafe fn TableGetColumnIndex() -> c_int
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
     let table: *mut ImGuiTable = g.CurrentTable;
    if !table { return  0; }
    return table.CurrentColumn;
}

// [Public] Append into a specific column
pub unsafe fn TableSetColumnIndex(column_n: c_int) -> bool
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
     let table: *mut ImGuiTable = g.CurrentTable;
    if !table { return  false; }

    if (table.CurrentColumn != column_n)
    {
        if (table.CurrentColumn != -1) {
            TableEndCell(table);
        }
        // IM_ASSERT(column_n >= 0 && table.ColumnsCount);
        TableBeginCell(table, column_n);
    }

    // Return whether the column is visible. User may choose to skip submitting items based on this return value,
    // however they shouldn't skip submitting for columns that may have the tallest contribution to row height.
    return (table.RequestOutputMaskByIndex & (1 << column_n)) != 0;
}

// [Public] Append into the next column, wrap and create a new row when already on last column
pub unsafe fn TableNextColumn() -> bool
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
     let table: *mut ImGuiTable = g.CurrentTable;
    if !table { return  false; }

    if (table.IsInsideRow && table.CurrentColumn + 1 < table.ColumnsCount)
    {
        if (table.CurrentColumn != -1) {
            TableEndCell(table);
        }
        TableBeginCell(table, table.CurrentColumn + 1);
    }
    else
    {
        TableNextRow(0, 0.0);
        TableBeginCell(table, 0);
    }

    // Return whether the column is visible. User may choose to skip submitting items based on this return value,
    // however they shouldn't skip submitting for columns that may have the tallest contribution to row height.
    let column_n: c_int = table.CurrentColumn;
    return (table.RequestOutputMaskByIndex & (1 << column_n)) != 0;
}


// [Internal] Called by TableSetColumnIndex()/TableNextColumn()
// This is called very frequently, so we need to be mindful of unnecessary overhead.
// FIXME-TABLE FIXME-OPT: Could probably shortcut some things for non-active or clipped columns.
pub unsafe fn TableBeginCell(table: *mut ImGuiTable, column_n: c_int)
{
    let mut column: *mut ImGuiTableColumn = &mut table.Columns[column_n];
    let mut window: &mut ImguiWindow = table.InnerWindow;
    table.CurrentColumn = column_n;

    // Start position is roughly ~~ CellRect.Min + CellPadding + Indent
    let mut start_x: c_float =  column.WorkMinX;
    if flag_set(column.Flags, ImGuiTableColumnFlags_IndentEnable) {
        start_x += table.RowIndentOffsetX;
    } // ~~ += window.dc.indent.x - table.HostIndentX, except we locked it for the row.

    window.dc.cursor_pos.x = start_x;
    window.dc.cursor_pos.y = table.RowPosY1 + table.CellPaddingY;
    window.dc.CursorMaxPos.x = window.dc.cursor_pos.x;
    window.dc.columns_offset.x = start_x - window.position.x - window.dc.indent.x; // FIXME-WORKRECT
    window.dc.curr_line_text_base_offset = table.RowTextBaseline;
    window.dc.NavLayerCurrent = column.NavLayerCurrent;

    window.work_rect.min.y = window.dc.cursor_pos.y;
    window.work_rect.min.x = column.WorkMinX;
    window.work_rect.max.x = column.WorkMaxX;
    window.dc.item_width = column.ItemWidth;

    // To allow ImGuiListClipper to function we propagate our row height
    if (!column.IsEnabled) {
        window.dc.cursor_pos.y = ImMax(window.dc.cursor_pos.y, table.RowPosY2);
    }

    window.skip_items = column.IsSkipItems;
    if (column.IsSkipItems)
    {
        let g = GImGui; // ImGuiContext& g = *GImGui;
        g.last_item_data.ID = 0;
        g.last_item_data.StatusFlags = 0;
    }

    if flag_set(table.Flags, ImGuiTableFlags_NoClip)
    {
        // FIXME: if we end up drawing all borders/bg in EndTable, could remove this and just assert that channel hasn't changed.
        table.DrawSplitter.SetCurrentChannel(&mut *window.DrawList, TABLE_DRAW_CHANNEL_NOCLIP);
        //IM_ASSERT(table.DrawSplitter._Current == TABLE_DRAW_CHANNEL_NOCLIP);
    }
    else
    {
        // FIXME-TABLE: Could avoid this if draw channel is dummy channel?
        SetWindowClipRectBeforeSetChannel(window, &column.ClipRect);
        table.DrawSplitter.SetCurrentChannel(&mut *window.DrawList, column.DrawChannelCurrent as c_int);
    }

    // Logging
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if g.LogEnabled && !column.IsSkipItems
    {
        LogRenderedText(g, &window.dc.cursor_pos, str_to_const_c_char_ptr("|"));
        g.LogLinePosY = f32::MAX;
    }
}



//-------------------------------------------------------------------------
// [SECTION] Tables: Columns width management
//-------------------------------------------------------------------------
// - TableGetMaxColumnWidth() [Internal]
// - TableGetColumnWidthAuto() [Internal]
// - TableSetColumnWidth()
// - TableSetColumnWidthAutoSingle() [Internal]
// - TableSetColumnWidthAutoAll() [Internal]
// - TableUpdateColumnsWeightFromWidth() [Internal]
//-------------------------------------------------------------------------

// Maximum column content width given current layout. Use column.MinX so this value on a per-column basis.TableGetMaxColumnWidth: c_float(table: *const ImGuiTable, column_n: c_int)
pub unsafe fn TableGetMaxColumnWidth(table: *const ImGuiTable, column_n:c_int)->c_float
{
    let column: *const ImGuiTableColumn = &table.Columns[column_n];
    let mut max_width: c_float =  f32::MAX;
    let min_column_distance: c_float =  table.MinColumnWidth + table.CellPaddingX * 2.0 + table.CellSpacingX1 + table.CellSpacingX2;
    if flag_set(table.Flags, ImGuiTableFlags_ScrollX)
    {
        // Frozen columns can't reach beyond visible width else scrolling will naturally break.
        // (we use DisplayOrder as within a set of multiple frozen column reordering is possible)
        if (column.DisplayOrder < table.FreezeColumnsRequest)
        {
            max_width = (table.InnerClipRect.max.x - (table.FreezeColumnsRequest - column.DisplayOrder) * min_column_distance) - column.MinX;
            max_width = max_width - table.OuterPaddingX - table.CellPaddingX - table.CellSpacingX2;
        }
    }
    else if ((table.Flags & ImGuiTableFlags_NoKeepColumnsVisible) == 0)
    {
        // If horizontal scrolling if disabled, we apply a final lossless shrinking of columns in order to make
        // sure they are all visible. Because of this we also know that all of the columns will always fit in
        // table.WorkRect and therefore in table.InnerRect (because ScrollX is off)
        // FIXME-TABLE: This is solved incorrectly but also quite a difficult problem to fix as we also want ClipRect width to match.
        // See "table_width_distrib" and "table_width_keep_visible" tests
        max_width = table.WorkRect.max.x - (table.ColumnsEnabledCount - column.IndexWithinEnabledSet - 1) * min_column_distance - column.MinX;
        //max_width -= table.CellSpacingX1;
        max_width -= table.CellSpacingX2;
        max_width -= table.CellPaddingX * 2.0;
        max_width -= table.OuterPaddingX;
    }
    return max_width;
}

// Note this is meant to be stored in column.WidthAuto, please generally use the WidthAuto
pub unsafe fn fieldTableGetColumnWidthAuto(table: *mut ImGuiTable, column: *mut ImGuiTableColumn) -> f32
{
    let content_width_body: c_float =  ImMax(column.ContentMaxXFrozen, column.ContentMaxXUnfrozen) - column.WorkMinX;
    let content_width_headers: c_float =  column.ContentMaxXHeadersIdeal - column.WorkMinX;
    let mut width_auto: c_float =  content_width_body;
    if (flag_clear(column.Flags, ImGuiTableColumnFlags_NoHeaderWidth)) {
        width_auto = ImMax(width_auto, content_width_headers);
    }

    // Non-resizable fixed columns preserve their requested width
    if (flag_set(column.Flags , ImGuiTableColumnFlags_WidthFixed) && column.InitStretchWeightOrWidth > 0.0) {
        if (flag_clear(table.Flags , ImGuiTableFlags_Resizable) || flag_set(column.Flags , ImGuiTableColumnFlags_NoResize)) {
            width_auto = column.InitStretchWeightOrWidth;
        }
    }

    return ImMax(width_auto, table.MinColumnWidth);
}

// 'width' = inner column width, without padding
pub unsafe fn TableSetColumnWidth(column_n: c_int,width: c_float)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
     let table: *mut ImGuiTable = g.CurrentTable;
    // IM_ASSERT(table != NULL && table.IsLayoutLocked == false);
    // IM_ASSERT(column_n >= 0 && column_n < table.ColumnsCount);
    let mut column_0: *mut ImGuiTableColumn = &mut table.Columns[column_n];
    let column_0_width: c_float =  width;

    // Apply constraints early
    // Compare both requested and actual given width to avoid overwriting requested width when column is stuck (minimum size, bounded)
    // IM_ASSERT(table.MinColumnWidth > 0.0);
    let min_width: c_float =  table.MinColumnWidth;
    let max_width: c_float =  ImMax(min_width, TableGetMaxColumnWidth(table, column_n));
    let mut column_0_width = ImClamp(column_0_width, min_width, max_width);
    if column_0.WidthGiven == column_0_width || column_0.WidthRequest == column_0_width { return ; }

    //IMGUI_DEBUG_PRINT("TableSetColumnWidth({}, {}->{})\n", column_0_idx, column_0->WidthGiven, column_0_width);
    let mut column_1: *mut ImGuiTableColumn = if column_0.NextEnabledColumn != -1 { &mut table.Columns[column_0.NextEnabledColumn]} else { None};

    // In this surprisingly not simple because of how we support mixing Fixed and multiple Stretch columns.
    // - All fixed: easy.
    // - All stretch: easy.
    // - One or more fixed + one stretch: easy.
    // - One or more fixed + more than one stretch: tricky.
    // Qt when manual resize is enabled only support a single _trailing_ stretch column.

    // When forwarding resize from Wn| to Fn+1| we need to be considerate of the _NoResize flag on Fn+1.
    // FIXME-TABLE: Find a way to rewrite all of this so interactions feel more consistent for the user.
    // Scenarios:
    // - F1 F2 F3  resize from F1| or F2|   --> ok: alter ->WidthRequested of Fixed column. Subsequent columns will be offset.
    // - F1 F2 F3  resize from F3|          --> ok: alter ->WidthRequested of Fixed column. If active, ScrollX extent can be altered.
    // - F1 F2 W3  resize from F1| or F2|   --> ok: alter ->WidthRequested of Fixed column. If active, ScrollX extent can be altered, but it doesn't make much sense as the Stretch column will always be minimal size.
    // - F1 F2 W3  resize from W3|          --> ok: no-op (disabled by Resize Rule 1)
    // - W1 W2 W3  resize from W1| or W2|   --> ok
    // - W1 W2 W3  resize from W3|          --> ok: no-op (disabled by Resize Rule 1)
    // - W1 F2 F3  resize from F3|          --> ok: no-op (disabled by Resize Rule 1)
    // - W1 F2     resize from F2|          --> ok: no-op (disabled by Resize Rule 1)
    // - W1 W2 F3  resize from W1| or W2|   --> ok
    // - W1 F2 W3  resize from W1| or F2|   --> ok
    // - F1 W2 F3  resize from W2|          --> ok
    // - F1 W3 F2  resize from W3|          --> ok
    // - W1 F2 F3  resize from W1|          --> ok: equivalent to resizing |F2. F3 will not move.
    // - W1 F2 F3  resize from F2|          --> ok
    // All resizes from a Wx columns are locking other columns.

    // Possible improvements:
    // - W1 W2 W3  resize W1|               --> to not be stuck, both W2 and W3 would stretch down. Seems possible to fix. Would be most beneficial to simplify resize of all-weighted columns.
    // - W3 F1 F2  resize W3|               --> to not be stuck past F1|, both F1 and F2 would need to stretch down, which would be lossy or ambiguous. Seems hard to fix.

    // [Resize Rule 1] Can't resize from right of right-most visible column if there is any Stretch column. Implemented in TableUpdateLayout().

    // If we have all Fixed columns OR resizing a Fixed column that doesn't come after a Stretch one, we can do an offsetting resize.
    // This is the preferred resize path
    if flag_set(column_0.Flags , ImGuiTableColumnFlags_WidthFixed) {
        if column_1.is_null()
            || table.LeftMostStretchedColumn == -1
            || table.Columns[table.LeftMostStretchedColumn].DisplayOrder >= column_0.DisplayOrder {
            column_0.WidthRequest = column_0_width;
            table.IsSettingsDirty = true;
            return;
        }
    }

    // We can also use previous column if there's no next one (this is used when doing an auto-fit on the right-most stretch column)
    if column_1 == None {
        column_1 = if column_0.PrevEnabledColumn != -1 { &mut table.Columns[column_0.PrevEnabledColumn] } else { None };
    }
    if column_1 == None { return ; }

    // Resizing from right-side of a Stretch column before a Fixed column forward sizing to left-side of fixed column.
    // (old_a + old_b == new_a + new_b) --> (new_a == old_a + old_b - new_b)
    let column_1_width: c_float =  ImMax(column_1.WidthRequest - (column_0_width - column_0.WidthRequest), min_width);
    column_0_width = column_0.WidthRequest + column_1.WidthRequest - column_1_width;
    // IM_ASSERT(column_0_width > 0.0 && column_1_width > 0.0);
    column_0.WidthRequest = column_0_width;
    column_1.WidthRequest = column_1_width;
    if (column_0.Flags | column_1.Flags) & ImGuiTableColumnFlags_WidthStretch{
        TableUpdateColumnsWeightFromWidth(table);}
    table.IsSettingsDirty = true;
}

// Disable clipping then auto-fit, will take 2 frames
// (we don't take a shortcut for unclipped columns to reduce inconsistencies when e.g. resizing multiple columns)
pub unsafe fn TableSetColumnWidthAutoSingle(table: *mut ImGuiTable, column_n: c_int)
{
    // Single auto width uses auto-fit
    let column: *mut ImGuiTableColumn = &mut table.Columns[column_n];
    if !column.IsEnabled { return ; }
    column.CannotSkipItemsQueue = (1 << 0);
    table.AutoFitSingleColumn = column_n as ImGuiTableColumnIdx;
}

pub unsafe fn TableSetColumnWidthAutoAll(table: *mut ImGuiTable)
{
    // for (let column_n: c_int = 0; column_n < table.ColumnsCount; column_n++)
    for column_n in 0 .. table.ColumnsCount
    {
        let column: *mut ImGuiTableColumn = &mut table.Columns[column_n];
        if (!column.IsEnabled && flag_clear(column.Flags , ImGuiTableColumnFlags_WidthStretch)) { // Cannot reset weight of hidden stretch column
            continue;
        }
        column.CannotSkipItemsQueue = (1 << 0);
        column.AutoFitQueue = (1 << 1);
    }
}

pub unsafe fn TableUpdateColumnsWeightFromWidth(table: *mut ImGuiTable)
{
    // IM_ASSERT(table.LeftMostStretchedColumn != -1 && table.RightMostStretchedColumn != -1);

    // Measure existing quantity
    let mut visible_weight: c_float =  0.0;
    let mut visible_width: c_float =  0.0;
    // for (let column_n: c_int = 0; column_n < table.ColumnsCount; column_n++)
    for column_n in 0 .. table.ColumnsCount
    {
        let column: *mut ImGuiTableColumn = &mut table.Columns[column_n];
        if !column.IsEnabled || flag_clear(column.Flags, ImGuiTableColumnFlags_WidthStretch) {
            continue;
        }
        // IM_ASSERT(column.StretchWeight > 0.0);
        visible_weight += column.StretchWeight;
        visible_width += column.WidthRequest;
    }
    // IM_ASSERT(visible_weight > 0.0 && visible_width > 0.0);

    // Apply new weights
    // for (let column_n: c_int = 0; column_n < table.ColumnsCount; column_n++)
    for column_n in 0 .. table.ColumnsCount
    {
        let column: *mut ImGuiTableColumn = &mut table.Columns[column_n];
        if !column.IsEnabled || flag_clear(column.Flags, ImGuiTableColumnFlags_WidthStretch) {
            continue;
        }
        column.StretchWeight = (column.WidthRequest / visible_width) * visible_weight;
        // IM_ASSERT(column.StretchWeight > 0.0);
    }
}

//-------------------------------------------------------------------------
// [SECTION] Tables: Drawing
//-------------------------------------------------------------------------
// - TablePushBackgroundChannel() [Internal]
// - TablePopBackgroundChannel() [Internal]
// - TableSetupDrawChannels() [Internal]
// - TableMergeDrawChannels() [Internal]
// - TableDrawBorders() [Internal]
//-------------------------------------------------------------------------

// Bg2 is used by Selectable (and possibly other widgets) to render to the background.
// Unlike our Bg0/1 channel which we uses for RowBg/CellBg/Borders and where we guarantee all shapes to be CPU-clipped, the Bg2 channel being widgets-facing will rely on regular ClipRect.
pub unsafe fn TablePushBackgroundChannel()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window  = g.current_window_mut().unwrap();
     let table: *mut ImGuiTable = g.CurrentTable;

    // Optimization: avoid SetCurrentChannel() + PushClipRect()
    table.HostBackupInnerClipRect = window.ClipRect;
    SetWindowClipRectBeforeSetChannel(window, &table.Bg2ClipRectForDrawCmd);
    table.DrawSplitter.SetCurrentChannel(window.DrawList, table.Bg2DrawChannelCurrent as c_int);
}

pub unsafe fn TablePopBackgroundChannel()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window  = g.current_window_mut().unwrap();
     let table: *mut ImGuiTable = g.CurrentTable;
    let column: *mut ImGuiTableColumn = &mut table.Columns[table.CurrentColumn];

    // Optimization: avoid PopClipRect() + SetCurrentChannel()
    SetWindowClipRectBeforeSetChannel(window, &table.HostBackupInnerClipRect);
    table.DrawSplitter.SetCurrentChannel(window.DrawList, column.DrawChannelCurrent as c_int);
}

// Allocate draw channels. Called by TableUpdateLayout()
// - We allocate them following storage order instead of display order so reordering columns won't needlessly
//   increase overall dormant memory cost.
// - We isolate headers draw commands in their own channels instead of just altering clip rects.
//   This is in order to facilitate merging of draw commands.
// - After crossing FreezeRowsCount, all columns see their current draw channel changed to a second set of channels.
// - We only use the dummy draw channel so we can push a null clipping rectangle into it without affecting other
//   channels, while simplifying per-row/per-cell overhead. It will be empty and discarded when merged.
// - We allocate 1 or 2 background draw channels. This is because we know TablePushBackgroundChannel() is only used for
//   horizontal spanning. If we allowed vertical spanning we'd need one background draw channel per merge group (1-4).
// Draw channel allocation (before merging):
// - NoClip                       --> 2+D+1 channels: bg0/1 + bg2 + foreground (same clip rect == always 1 draw call)
// - Clip                         --> 2+D+N channels
// - FreezeRows                   --> 2+D+N*2 (unless scrolling value is zero)
// - FreezeRows || FreezeColunns  --> 3+D+N*2 (unless scrolling value is zero)
// Where D is 1 if any column is clipped or hidden (dummy channel) otherwise 0.
pub unsafe fn TableSetupDrawChannels(table: *mut ImGuiTable)
{
    let freeze_row_multiplier: c_int = if table.FreezeRowsCount > 0 { 2} else { 1};
    let channels_for_row: c_int = if table.Flags & ImGuiTableFlags_NoClip { 1} else { table.ColumnsEnabledCount};
    let channels_for_bg: c_int = 1 + 1 * freeze_row_multiplier;
    let channels_for_dummy: c_int = if table.ColumnsEnabledCount < table.ColumnsCount as ImGuiTableColumnIdx || table.VisibleMaskByIndex != table.EnabledMaskByIndex { 1} else { 0};
    let channels_total: c_int = channels_for_bg + (channels_for_row * freeze_row_multiplier) + channels_for_dummy;
    table.DrawSplitter.Split(table.Innerwindow.DrawList, channels_total as size_t);
    table.DummyDrawChannel = if channels_for_dummy > 0 { channels_total - 1 } else { -1 } as ImGuiTableDrawChannelIdx;
    table.Bg2DrawChannelCurrent = TABLE_DRAW_CHANNEL_BG2_FROZEN as ImGuiTableDrawChannelIdx;
    table.Bg2DrawChannelUnfrozen = if table.FreezeRowsCount > 0 { 2 + channels_for_row } else { TABLE_DRAW_CHANNEL_BG2_FROZEN } as ImGuiTableDrawChannelIdx;

    let mut draw_channel_current: c_int = 2;
    // for (let column_n: c_int = 0; column_n < table.ColumnsCount; column_n++)
    for column_n in 0 .. table.ColumnsCount
    {
        let column: *mut ImGuiTableColumn = &mut table.Columns[column_n];
        if column.IsVisibleX && column.IsVisibleY
        {
            column.DrawChannelFrozen = (draw_channel_current) as ImGuiTableDrawChannelIdx;
            column.DrawChannelUnfrozen = (draw_channel_current + (if table.FreezeRowsCount > 0 { channels_for_row + 1 } else { 0 })) as ImGuiTableDrawChannelIdx;
            if flag_clear(table.Flags, ImGuiTableFlags_NoClip) {
                draw_channel_current += 1;
            }
        }
        else
        {
            column.DrawChannelFrozen = table.DummyDrawChannel;
            column.DrawChannelUnfrozen = table.DummyDrawChannel;
        }
        column.DrawChannelCurrent = column.DrawChannelFrozen;
    }

    // Initial draw cmd starts with a BgClipRect that matches the one of its host, to facilitate merge draw commands by default.
    // All our cell highlight are manually clipped with BgClipRect. When unfreezing it will be made smaller to fit scrolling rect.
    // (This technically isn't part of setting up draw channels, but is reasonably related to be done here)
    table.BgClipRect = table.InnerClipRect;
    table.Bg0ClipRectForDrawCmd = table.Outerwindow.ClipRect;
    table.Bg2ClipRectForDrawCmd = table.HostClipRect;
    // IM_ASSERT(table.BgClipRect.Min.y <= table.BgClipRect.Max.y);
}

// This function reorder draw channels based on matching clip rectangle, to facilitate merging them. Called by EndTable().
// For simplicity we call it TableMergeDrawChannels() but in fact it only reorder channels + overwrite ClipRect,
// actual merging is done by table.DrawSplitter.Merge() which is called right after TableMergeDrawChannels().
//
// Columns where the contents didn't stray off their local clip rectangle can be merged. To achieve
// this we merge their clip rect and make them contiguous in the channel list, so they can be merged
// by the call to DrawSplitter.Merge() following to the call to this function.
// We reorder draw commands by arranging them into a maximum of 4 distinct groups:
//
//   1 group:               2 groups:              2 groups:              4 groups:
//   [ 0. ] no freeze       [ 0. ] row freeze      [ 01 ] col freeze      [ 01 ] row+col freeze
//   [ .. ]  or no scroll   [ 2. ]  and v-scroll   [ .. ]  and h-scroll   [ 23 ]  and v+h-scroll
//
// Each column itself can use 1 channel (row freeze disabled) or 2 channels (row freeze enabled).
// When the contents of a column didn't stray off its limit, we move its channels into the corresponding group
// based on its position (within frozen rows/columns groups or not).
// At the end of the operation our 1-4 groups will each have a ImDrawCmd using the same ClipRect.
// This function assume that each column are pointing to a distinct draw channel,
// otherwise merge_group->ChannelsCount will not match set bit count of merge_group->ChannelsMask.
//
// Column channels will not be merged into one of the 1-4 groups in the following cases:
// - The contents stray off its clipping rectangle (we only compare the MaxX value, not the MinX value).
//   Direct ImDrawList calls won't be taken into account by default, if you use them make sure the  bounds
//   matches, by e.g. calling SetCursorScreenPos().
// - The channel uses more than one draw command itself. We drop all our attempt at merging stuff here..
//   we could do better but it's going to be rare and probably not worth the hassle.
// Columns for which the draw channel(s) haven't been merged with other will use their own ImDrawCmd.
//
// This function is particularly tricky to understand.. take a breath.
pub unsafe fn TableMergeDrawChannels(table: *mut ImGuiTable)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    splitter: *mut ImDrawListSplitter = table.DrawSplitter;
    let has_freeze_v: bool = (table.FreezeRowsCount > 0);
    let has_freeze_h: bool = (table.FreezeColumnsCount > 0);
    // IM_ASSERT(splitter->_Current == 0);

    // Track which groups we are going to attempt to merge, and which channels goes into each group.

    let mut merge_group_mask: c_int = 0x00;
    // MergeGroup merge_groups[4];
    let mut merge_groups: [MergeGroup;4] = [MergeGroup::default();4];

    // 1. Scan channels and take note of those which can be merged
    // for (let column_n: c_int = 0; column_n < table.ColumnsCount; column_n++)
    for column_n in 0 .. table.ColumnsCount
    {
        if (table.VisibleMaskByIndex & (1 << column_n)) == 0{
            continue;}
        let column: *mut ImGuiTableColumn = &mut table.Columns[column_n];

        let merge_group_sub_count: c_int = if has_freeze_v {2} else { 1 };
        // for (let merge_group_sub_n: c_int = 0; merge_group_sub_n < merge_group_sub_count; merge_group_sub_n++)
        for merge_group_sub_n in 0 .. merge_group_sub_count
        {
            let channel_no: c_int = if merge_group_sub_n == 0 { column.DrawChannelFrozen } else { column.DrawChannelUnfrozen } as c_int;

            // Don't attempt to merge if there are multiple draw calls within the column
            let src_channel: *mut ImDrawChannel = &mut splitter._Channels[channel_no];
            if src_channel._CmdBuffer.len() > 0 && src_channel._CmdBuffer.last().unwrap().ElemCount == 0 && src_channel._CmdBuffer.last().unwrap().UserCallback == None { // Equivalent of PopUnusedDrawCmd()
                src_channel._CmdBuffer.pop_back();
            }
            if (src_channel._CmdBuffer.len() != 1) {
                continue;
            }

            // Find out the width of this merge group and check if it will fit in our column
            // (note that we assume that rendering didn't stray on the left direction. we should need a CursorMinPos to detect it)
            if (flag_clear(column.Flags, ImGuiTableColumnFlags_NoClip))
            {
                let mut content_max_x: c_float = 0.0;
                if (!has_freeze_v) {
                    content_max_x = ImMax(column.ContentMaxXUnfrozen, column.ContentMaxXHeadersUsed);
                } // No row freeze
                else if (merge_group_sub_n == 0) {
                    content_max_x = ImMax(column.ContentMaxXFrozen, column.ContentMaxXHeadersUsed);
                }  // Row freeze: use width before freeze
                else {
                    content_max_x = column.ContentMaxXUnfrozen;
                }                                // Row freeze: use width after freeze
                if content_max_x > column.ClipRect.max.x{
                    continue;}
            }

            let merge_group_n: c_int = (if has_freeze_h && column_n < table.FreezeColumnsCount as c_int { 0 } else { 1 }) + (if has_freeze_v && merge_group_sub_n == 0 { 0 } else { 2 });
            // IM_ASSERT(channel_no < IMGUI_TABLE_MAX_DRAW_CHANNELS);
            let merge_group: *mut MergeGroup = &mut merge_groups[merge_group_n];
            if merge_group.ChannelsCount == 0 {
                merge_group.ClipRect = ImRect::default();
            }
            merge_group.ChannelsMask.SetBit(channel_no);
            merge_group.ChannelsCount+= 1;
            merge_group.ClipRect.Add(src_channel._CmdBuffer[0].ClipRect.into());
            merge_group_mask |= (1 << merge_group_n);
        }

        // Invalidate current draw channel
        // (we don't clear DrawChannelFrozen/DrawChannelUnfrozen solely to facilitate debugging/later inspection of data)
        column.DrawChannelCurrent = -1;
    }

    // [DEBUG] Display merge groups
// #if 0
    if g.IO.KeyShift {
        // for (let merge_group_n: c_int = 0; merge_group_n < merge_groups.len(); merge_group_n+ +)
       for merge_group_n in 0 .. merge_groups.len()
        {
            let mut merge_group: *mut MergeGroup = &mut merge_groups[merge_group_n];
            if merge_group.ChannelsCount == 0 {
                continue;
            }
            buf: [c_char; 32];
            // ImFormatString(buf, 32, "MG{}:{}", merge_group_n, merge_group.ChannelsCount);
            let text_pos: Vector2 = merge_group.ClipRect.min + Vector2::from_floats(4.0, 4.0);
            let text_size: Vector2 = CalcTextSize(buf, None, false, 0.0);
            GetForegroundDrawList(null_mut()).AddRectFilled(&text_pos, text_pos + text_size, color_u32_from_rgba(0, 0, 0, 255), 0.0, 0);
            GetForegroundDrawList(null_mut()).AddText(&text_pos, color_u32_from_rgba(255, 255, 0, 255), buf);
            GetForegroundDrawList(null_mut()).AddRect(&merge_group.ClipRect.min, &merge_group.ClipRect.max, color_u32_from_rgba(255, 255, 0, 255), 0.0);
        }
    }
// #endif

    // 2. Rewrite channel list in our preferred order
    if merge_group_mask != 0
    {
        // We skip channel 0 (Bg0/Bg1) and 1 (Bg2 frozen) from the shuffling since they won't move - see channels allocation in TableSetupDrawChannels().
        let LEADING_DRAW_CHANNELS: c_int = 2;
        g.DrawChannelsTempMergeBuffer.resize_with(splitter._Count - LEADING_DRAW_CHANNELS, ImDrawChannel::default()); // Use shared temporary storage so the allocation gets amortized
        dst_tmp: *mut ImDrawChannel = g.DrawChannelsTempMergeBuffer.Data;
        // ImBitArray<IMGUI_TABLE_MAX_DRAW_CHANNELS> remaining_mask;                       // We need 132-bit of storage
        let mut remaining_mask = ImBitArray::default();
        remaining_mask.SetBitRange(LEADING_DRAW_CHANNELS, splitter._Count);
        remaining_mask.ClearBit(table.Bg2DrawChannelUnfrozen);
        // IM_ASSERT(has_freeze_v == false || table.Bg2DrawChannelUnfrozen != TABLE_DRAW_CHANNEL_BG2_FROZEN);
        let mut remaining_count: c_int = splitter._Count - (if has_freeze_v { LEADING_DRAW_CHANNELS + 1 } else { LEADING_DRAW_CHANNELS });
        //ImRect host_rect = if (table.InnerWindow == table.OuterWindow) { table.InnerClipRect} else {table.HostClipRect};
        let host_rect: ImRect =  table.HostClipRect;
        // for (let merge_group_n: c_int = 0; merge_group_n < merge_groups.len(); merge_group_n++)
        for merge_group_n in 0 .. merge_groups.len()
        {
            let mut merge_channels_count: c_int = merge_groups[merge_group_n].ChannelsCount;
            if merge_channels_count > 0
            {
                merge_group: *mut MergeGroup = &mut merge_groups[merge_group_n];
                let mut merge_clip_rect: ImRect =  merge_group.ClipRect;

                // Extend outer-most clip limits to match those of host, so draw calls can be merged even if
                // outer-most columns have some outer padding offsetting them from their parent ClipRect.
                // The principal cases this is dealing with are:
                // - On a same-window table (not scrolling = single group), all fitting columns ClipRect -> will extend and match host ClipRect -> will merge
                // - Columns can use padding and have left-most ClipRect.Min.x and right-most ClipRect.Max.x != from host ClipRect -> will extend and match host ClipRect -> will merge
                // FIXME-TABLE FIXME-WORKRECT: We are wasting a merge opportunity on tables without scrolling if column doesn't fit
                // within host clip rect, solely because of the half-padding difference between window.work_rect and window.InnerClipRect.
                if (merge_group_n & 1) == 0 || !has_freeze_h {
                    merge_clip_rect.min.x = merge_clip_rect.min.x.min( host_rect.min.x);
                }
                if (merge_group_n & 2) == 0 || !has_freeze_v {
                    merge_clip_rect.min.y = merge_clip_rect.min.y.min( host_rect.min.y);
                }
                if (merge_group_n & 1) != 0 {
                    merge_clip_rect.max.x = merge_clip_rect.max.x.max( host_rect.max.x);
                }
                if (merge_group_n & 2) != 0 && flag_clear(table.Flags, ImGuiTableFlags_NoHostExtendY) {
                    merge_clip_rect.max.y = ImMax(merge_clip_rect.max.y, host_rect.max.y);
                }
// #if 0
                GetOverlayDrawList().AddRect(merge_group.ClipRect.Min, merge_group.ClipRect.Max, color_u32_from_rgba(255, 0, 0, 200), 0.0, 0, 1.0);
                GetOverlayDrawList().AddLine(merge_group.ClipRect.Min, merge_clip_rect.min, color_u32_from_rgba(255, 100, 0, 200));
                GetOverlayDrawList().AddLine(merge_group.ClipRect.Max, merge_clip_rect.max, color_u32_from_rgba(255, 100, 0, 200));
// #endif
                remaining_count -= merge_group.ChannelsCount;
                // for (let n: c_int = 0; n < IM_ARRAYSIZE(remaining_mask.Storage); n++)
                for n in 0 .. remaining_mask.len()
                {
                    remaining_mask.Storage[n] &= !merge_group.ChannelsMask.Storage[n];
                }
                // for (let n: c_int = 0; n < splitter._Count && merge_channels_count != 0; n++)
                for n in 0 .. splitter._Count
                {
                    // Copy + overwrite new clip rect
                    if !merge_group.ChannelsMask.TestBit(n) {
                        continue;
                    }
                    merge_group.ChannelsMask.ClearBit(n);
                    merge_channels_count-= 1;

                    let channel: *mut ImDrawChannel = &mut splitter._Channels[n];
                    // IM_ASSERT(channel->_CmdBuffer.Size == 1 && merge_clip_rect.Contains(ImRect(channel->_CmdBuffer[0].ClipRect)));
                    channel._CmdBuffer[0].ClipRect = merge_clip_rect.ToVec4();
                    memcpy(dst_tmp, channel, sizeof(ImDrawChannel));
                    dst_tmp += 1;
                    if merge_channels_count == 0 { break;}
                }
            }

            // Make sure Bg2DrawChannelUnfrozen appears in the middle of our groups (whereas Bg0/Bg1 and Bg2 frozen are fixed to 0 and 1)
            if merge_group_n == 1 && has_freeze_v {
                memcpy(dst_tmp, &splitter._Channels[table.Bg2DrawChannelUnfrozen], sizeof(ImDrawChannel));
                dst_tmp += 1;
            }
        }

        // Append unmergeable channels that we didn't reorder at the end of the list
        // for (let n: c_int = 0; n < splitter._Count && remaining_count != 0; n++)
        for n in 0 .. splitter._Count
        {
            if !remaining_mask.TestBit(n) {
                continue;
            }
            let channel: *mut ImDrawChannel = &mut splitter._Channels[n];
            memcpy(dst_tmp, channel, sizeof(ImDrawChannel));
            dst_tmp += 1;
            remaining_count-= 1;
            if remaining_count == 0 {  break; }
        }
        // IM_ASSERT(dst_tmp == g.DrawChannelsTempMergeBuffer.Data + g.DrawChannelsTempMergeBuffer.Size);
        memcpy(splitter._Channels.Data + LEADING_DRAW_CHANNELS, g.DrawChannelsTempMergeBuffer.Data, (splitter._Count - LEADING_DRAW_CHANNELS) * sizeof(ImDrawChannel));
    }
}

// FIXME-TABLE: This is a mess, need to redesign how we render borders (as some are also done in TableEndRow)
pub unsafe fn TableDrawBorders(table: *mut ImGuiTable)
{
    inner_window: &mut ImguiWindow = table.InnerWindow;
    if !table.Outerwindow.ClipRect.Overlaps(table.OuterRect) { return ; }

    inner_drawlist: *mut ImDrawList = inner_window.DrawList;
    table.DrawSplitter.SetCurrentChannel(inner_drawlist, TABLE_DRAW_CHANNEL_BG0);
    inner_drawlist.PushClipRect(table.Bg0ClipRectForDrawCmd.min, table.Bg0ClipRectForDrawCmd.max, false);

    // Draw inner border and resizing feedback
    table_instance: *mut ImGuiTableInstanceData = TableGetInstanceData(table, table.InstanceCurrent);
    let border_size: c_float =  TABLE_BORDER_SIZE;
    let draw_y1: c_float =  table.InnerRect.min.y;
    let draw_y2_body: c_float =  table.InnerRect.max.y;
    let draw_y2_head: c_float =  if table.IsUsingHeaders { table.InnerRect.max.y.min( (if table.FreezeRowsCount >= 1{ table.InnerRect.min.y }else { table.WorkRect.min.y }) + table_instance.LastFirstRowHeight) } else { draw_y1 };
    if flag_set(table.Flags, ImGuiTableFlags_BordersInnerV)
    {
        // for (let order_n: c_int = 0; order_n < table.ColumnsCount; order_n++)
        for order_n in 0 .. table.ColumnsCount
        {
            if (!(table.EnabledMaskByDisplayOrder & (1 << order_n))) {
                continue;
            }

            let column_n: c_int = table.DisplayOrderToIndex[order_n];
            let column: *mut ImGuiTableColumn = &mut table.Columns[column_n];
            let is_hovered: bool = (table.HoveredColumnBorder == column_n as ImGuiTableColumnIdx);
            let is_resized: bool = (table.ResizedColumn == column_n as ImGuiTableColumnIdx) && (table.InstanceInteracted == table.InstanceCurrent as i16);
            let is_resizable: bool = (column.Flags & (ImGuiTableColumnFlags_NoResize | ImGuiTableColumnFlags_NoDirectResize_)) == 0;
            let is_frozen_separator: bool = (table.FreezeColumnsCount == (order_n + 1) as ImGuiTableColumnIdx);
            if (column.MaxX > table.InnerClipRect.max.x && !is_resized) {
                continue;
            }

            // Decide whether right-most column is visible
            if (column.NextEnabledColumn == -1 && !is_resizable) {
                if ((table.Flags & ImGuiTableFlags_SizingMask_) != ImGuiTableFlags_SizingFixedSame || flag_set(table.Flags , ImGuiTableFlags_NoHostExtendX)) {
                    continue;
                }
            }
            if (column.MaxX <= column.ClipRect.min.x) { // FIXME-TABLE FIXME-STYLE: Assume BorderSize==1, this is problematic if we want to increase the border size..
                continue;
            }

            // Draw in outer window so right-most column won't be clipped
            // Always draw full height border when being resized/hovered, or on the delimitation of frozen column scrolling.
            col: u32;
            let mut draw_y2: c_float = 0.0;
            if is_hovered || is_resized || is_frozen_separator
            {
                draw_y2 = draw_y2_body;
                col = if is_resized { GetColorU32(ImGuiCol_SeparatorActive, 0.0) } else {
                    if is_hovered {
                        GetColorU32(ImGuiCol_SeparatorHovered, 0.0)
                    }else {
                        table.BorderColorStrong
                    }};
            }
            else
            {
                draw_y2 = if table.Flags & (ImGuiTableFlags_NoBordersInBody | ImGuiTableFlags_NoBordersInBodyUntilResize) { draw_y2_head} else { draw_y2_body};
                col = if table.Flags & (ImGuiTableFlags_NoBordersInBody | ImGuiTableFlags_NoBordersInBodyUntilResize) { table.BorderColorStrong} else { table.BorderColorLight};
            }

            if (draw_y2 > draw_y1) {
                inner_drawlist.AddLine(Vector2::from_floats(column.MaxX, draw_y1), Vector2::from_floats(column.MaxX, draw_y2), col, border_size);
            }
        }
    }

    // Draw outer border
    // FIXME: could use AddRect or explicit VLine/HLine helper?
    if flag_set(table.Flags, ImGuiTableFlags_BordersOuter)
    {
        // Display outer border offset by 1 which is a simple way to display it without adding an extra draw call
        // (Without the offset, in outer_window it would be rendered behind cells, because child windows are above their
        // parent. In inner_window, it won't reach out over scrollbars. Another weird solution would be to display part
        // of it in inner window, and the part that's over scrollbars in the outer window..)
        // Either solution currently won't allow us to use a larger border size: the border would clipped.
        let outer_border: ImRect =  table.OuterRect;
        outer_col: u32 = table.BorderColorStrong;
        if ((table.Flags & ImGuiTableFlags_BordersOuter) == ImGuiTableFlags_BordersOuter)
        {
            inner_drawlist.AddRect(outer_border.min, outer_border.max, outer_col, 0.0, 0, border_size);
        }
        else if flag_set(table.Flags, ImGuiTableFlags_BordersOuterV)
        {
            inner_drawlist.AddLine(outer_border.min, Vector2::from_floats(outer_border.min.x, outer_border.max.y), outer_col, border_size);
            inner_drawlist.AddLine(Vector2::from_floats(outer_border.max.x, outer_border.min.y), outer_border.max, outer_col, border_size);
        }
        else if flag_set(table.Flags, ImGuiTableFlags_BordersOuterH)
        {
            inner_drawlist.AddLine(outer_border.min, Vector2::from_floats(outer_border.max.x, outer_border.min.y), outer_col, border_size);
            inner_drawlist.AddLine(Vector2::from_floats(outer_border.min.x, outer_border.max.y), outer_border.max, outer_col, border_size);
        }
    }
    if flag_set(table.Flags, ImGuiTableFlags_BordersInnerH) && table.RowPosY2 < table.OuterRect.max.y
    {
        // Draw bottom-most row border
        let border_y: c_float =  table.RowPosY2;
        if border_y >= table.BgClipRect.min.y && border_y < table.BgClipRect.max.y {
            inner_drawlist.AddLine(Vector2::from_floats(table.BorderX1, border_y),
                                   Vector2::from_floats(table.BorderX2, border_y),
                                   table.BorderColorLight,
                                   border_size);
        }
    }

    inner_drawlist.PopClipRect();
}

//-------------------------------------------------------------------------
// [SECTION] Tables: Sorting
//-------------------------------------------------------------------------
// - TableGetSortSpecs()
// - TableFixColumnSortDirection() [Internal]
// - TableGetColumnNextSortDirection() [Internal]
// - TableSetColumnSortDirection() [Internal]
// - TableSortSpecsSanitize() [Internal]
// - TableSortSpecsBuild() [Internal]
//-------------------------------------------------------------------------

// Return NULL if no sort specs (most often when ImGuiTableFlags_Sortable is not set)
// You can sort your data again when 'SpecsChanged == true'. It will be true with sorting specs have changed since
// last call, or the first time.
// Lifetime: don't hold on this pointer over multiple frames or past any subsequent call to BeginTable()!
pub unsafe fn TableGetSortSpecs() -> *mut ImGuiTableSortSpecs
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
     let table: *mut ImGuiTable = g.CurrentTable;
    // IM_ASSERT(table != NULL);

    if (flag_clear(table.Flags, ImGuiTableFlags_Sortable)) {
        return None;
    }

    // Require layout (in case TableHeadersRow() hasn't been called) as it may alter IsSortSpecsDirty in some paths.
    if (!table.IsLayoutLocked) {
        TableUpdateLayout(table);
    }

    TableSortSpecsBuild(table);

    return &mut table.SortSpecs;
}

pub fn  TableGetColumnAvailSortDirection(column: *mut ImGuiTableColumn, n: c_int) -> ImGuiSortDirection
{
    // IM_ASSERT(n < column.SortDirectionsAvailCount);
    return ((column.SortDirectionsAvailList >> (n << 1)) & 0x03) as ImGuiSortDirection;
}

// Fix sort direction if currently set on a value which is unavailable (e.g. activating NoSortAscending/NoSortDescending)
pub unsafe fn TableFixColumnSortDirection(table: *mut ImGuiTable, column: *mut ImGuiTableColumn)
{
    if column.SortOrder == -1 || (column.SortDirectionsAvailMask & (1 << column.SortDirection)) != 0 { return ; }
    column.SortDirection = TableGetColumnAvailSortDirection(column, 0);
    table.IsSortSpecsDirty = true;
}

// Calculate next sort direction that would be set after clicking the column
// - If the PreferSortDescending flag is set, we will default to a Descending direction on the first click.
// - Note that the PreferSortAscending flag is never checked, it is essentially the default and therefore a no-op.
// IM_STATIC_ASSERT(ImGuiSortDirection_None == 0 && ImGuiSortDirection_Ascending == 1 && ImGuiSortDirection_Descending == 2);
pub unsafe fn TableGetColumnNextSortDirection(column: *mut ImGuiTableColumn) -> ImGuiSortDirection
{
    // IM_ASSERT(column.SortDirectionsAvailCount > 0);
    if column.SortOrder == -1 {
        return TableGetColumnAvailSortDirection(column, 0);
    }
    // for (let n: c_int = 0; n < 3; n++)
    for n in 0 .. 3
    {
        if column.SortDirection == TableGetColumnAvailSortDirection(column, n) {
            return TableGetColumnAvailSortDirection(column, (n + 1) % column.SortDirectionsAvailCount);
        }
    }
    // IM_ASSERT(0);
    return ImGuiSortDirection_None;
}

// Note that the NoSortAscending/NoSortDescending flags are processed in TableSortSpecsSanitize(), and they may change/revert
// the value of SortDirection. We could technically also do it here but it would be unnecessary and duplicate code.
pub unsafe fn TableSetColumnSortDirection(column_n: c_int, sort_direction: ImGuiSortDirection, mut append_to_sort_specs: bool)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
     let table: *mut ImGuiTable = g.CurrentTable;

    if flag_clear(table.Flags, ImGuiTableFlags_SortMulti) {
        append_to_sort_specs = false;
    }
    if (flag_clear(table.Flags, ImGuiTableFlags_SortTristate)) {}
        // IM_ASSERT(sort_direction != ImGuiSortDirection_None);

    let mut sort_order_max: ImGuiTableColumnIdx = 0;
    if (append_to_sort_specs) {
        // for (let other_column_n: c_int = 0; other_column_n < table.ColumnsCount; other_column_n+ +)
        for other_column_n in 0 .. table.ColumnsCount
        {
            sort_order_max = ImMax(sort_order_max, table.Columns[other_column_n].SortOrder);
        }
    }

    let column: *mut ImGuiTableColumn = &mut table.Columns[column_n];
    column.SortDirection = sort_direction;
    if (column.SortDirection == ImGuiSortDirection_None) {
        column.SortOrder = -1;
    }
    else if (column.SortOrder == -1 || !append_to_sort_specs) {
        column.SortOrder = if append_to_sort_specs { sort_order_max + 1 } else { 0 };
    }

    // for (let other_column_n: c_int = 0; other_column_n < table.ColumnsCount; other_column_n++)
    for other_column_n in 0 .. table.ColumnsCount
    {
        let other_column: *mut ImGuiTableColumn = &mut table.Columns[other_column_n];
        if (other_column != column && !append_to_sort_specs) {
            other_column.SortOrder = -1;
        }
        TableFixColumnSortDirection(table, other_column);
    }
    table.IsSettingsDirty = true;
    table.IsSortSpecsDirty = true;
}

pub unsafe fn TableSortSpecsSanitize(table: *mut ImGuiTable)
{
    // IM_ASSERT(table.Flags & ImGuiTableFlags_Sortable);

    // Clear SortOrder from hidden column and verify that there's no gap or duplicate.
    let mut sort_order_count: c_int = 0;
    let mut sort_order_mask = 0x00;
    // for (let column_n: c_int = 0; column_n < table.ColumnsCount; column_n++)
    for column_n in 0 .. table.ColumnsCount
    {
        let column: *mut ImGuiTableColumn = &mut table.Columns[column_n];
        if (column.SortOrder != -1 && !column.IsEnabled) {
            column.SortOrder = -1;
        }
        if (column.SortOrder == -1) {
            continue;
        }
        sort_order_count+= 1;
        sort_order_mask |= (1 << column.SortOrder);
        // IM_ASSERT(sort_order_count < sizeof(sort_order_mask) * 8);
    }

    let need_fix_linearize: bool = (1 << sort_order_count) != (sort_order_mask + 1);
    let need_fix_single_sort_order: bool = (sort_order_count > 1) && flag_clear(table.Flags , ImGuiTableFlags_SortMulti);
    if need_fix_linearize || need_fix_single_sort_order
    {
        let mut fixed_mask = 0x00;
        // for (let sort_n: c_int = 0; sort_n < sort_order_count; sort_n++)
        for sort_n in 0 .. sort_order_count
        {
            // Fix: Rewrite sort order fields if needed so they have no gap or duplicate.
            // (e.g. SortOrder 0 disappeared, SortOrder 1..2 exists --> rewrite then as SortOrder 0..1)
            let mut column_with_smallest_sort_order: c_int = -1;
            // for (let column_n: c_int = 0; column_n < table.ColumnsCount; column_n++)
            for column_n in 0 .. table.ColumnsCount
            {
                if (fixed_mask & (1 << column_n)) == 0 && table.Columns[column_n].SortOrder != -1 {
                    if column_with_smallest_sort_order == -1 || table.Columns[column_n].SortOrder < table.Columns[column_with_smallest_sort_order].SortOrder {
                        column_with_smallest_sort_order = column_n;
                    }
                }
            }
            // IM_ASSERT(column_with_smallest_sort_order != -1);
            fixed_mask |= (1 << column_with_smallest_sort_order);
            table.Columns[column_with_smallest_sort_order].SortOrder = sort_n;

            // Fix: Make sure only one column has a SortOrder if ImGuiTableFlags_MultiSortable is not set.
            if need_fix_single_sort_order
            {
                sort_order_count = 1;
                // for (let column_n: c_int = 0; column_n < table.ColumnsCount; column_n++)
                for column_n in 0 .. table.ColumnsCount
                {
                    if column_n != column_with_smallest_sort_order {
                        table.Columns[column_n].SortOrder = -1;
                    }
                }
                break;
            }
        }
    }

    // Fallback default sort order (if no column had the ImGuiTableColumnFlags_DefaultSort flag)
    if (sort_order_count == 0 && flag_clear(table.Flags , ImGuiTableFlags_SortTristate))
    {
        // for (let column_n: c_int = 0; column_n < table.ColumnsCount; column_n+ +)
        for column_n in 0 .. table.ColumnsCount
        {
            let column: *mut ImGuiTableColumn = &mut table.Columns[column_n];
            if column.IsEnabled && flag_clear(column.Flags, ImGuiTableColumnFlags_NoSort) {
                sort_order_count = 1;
                column.SortOrder = 0;
                column.SortDirection = TableGetColumnAvailSortDirection(column, 0);
                break;
            }
        }
    }

    table.SortSpecsCount = sort_order_count as ImGuiTableColumnIdx;
}

pub unsafe fn TableSortSpecsBuild(table: *mut ImGuiTable)
{
    let mut dirty: bool =  table.IsSortSpecsDirty;
    if (dirty)
    {
        TableSortSpecsSanitize(table);
        table.SortSpecsMulti.resize_with(if table.SortSpecsCount <= 1 { 0 } else { table.SortSpecsCount }, ImGuiTableSortSpecs::default());
        table.SortSpecs.SpecsDirty = true; // Mark as dirty for user
        table.IsSortSpecsDirty = false; // Mark as not dirty for us
    }

    // Write output
    let sort_specs: *mut ImGuiTableColumnSortSpecs = if table.SortSpecsCount == 0 { None } else{ if table.SortSpecsCount == 1 { &mut table.SortSpecsSingle} else { table.SortSpecsMulti.Data}};
    if (dirty && sort_specs != null_mut()) {
        // for (let column_n: c_int = 0; column_n < table.ColumnsCount; column_n+ +)
        for column_n in 0 .. table.ColumnsCount
        {
            let column: *mut ImGuiTableColumn = &mut table.Columns[column_n];
            if (column.SortOrder == -1) {
                continue;
            }
            // IM_ASSERT(column.SortOrder < table.SortSpecsCount);
            let sort_spec = &mut sort_specs[column.SortOrder];
            sort_spec.ColumnUserID = column.UserID;
            sort_spec.ColumnIndex = column_n;
            sort_spec.SortOrder = column.SortOrder;
            sort_spec.SortDirection = column.SortDirection;
        }
    }

    table.SortSpecs.Specs = sort_specs;
    table.SortSpecs.SpecsCount = table.SortSpecsCount as size_t;
}

//-------------------------------------------------------------------------
// [SECTION] Tables: Headers
//-------------------------------------------------------------------------
// - TableGetHeaderRowHeight() [Internal]
// - TableHeadersRow()
// - TableHeader()
//-------------------------------------------------------------------------
pub unsafe fn TableGetHeaderRowHeight() -> f32
{
    // Caring for a minor edge case:
    // Calculate row height, for the unlikely case that some labels may be taller than others.
    // If we didn't do that, uneven header height would highlight but smaller one before the tallest wouldn't catch input for all height.
    // In your custom header row you may omit this all together and just call TableNextRow() without a height...
    let mut row_height: c_float =  GetTextLineHeight();
    let columns_count: c_int = TableGetColumnCount();
    // for (let column_n: c_int = 0; column_n < columns_count; column_n++)
    for column_n in 0 .. columns_count
    {
        let flags: ImGuiTableColumnFlags = TableGetColumnFlags(column_n);
        if flag_set(flags, ImGuiTableColumnFlags_IsEnabled) && flag_clear(flags, ImGuiTableColumnFlags_NoHeaderLabel) {
            row_height = ImMax(row_height, CalcTextSize(TableGetColumnName(column_n), None, false, 0.0).y);
        }
    }
    row_height += GetStyle().CellPadding.y * 2.0;
    return row_height;
}

// [Public] This is a helper to output TableHeader() calls based on the column names declared in TableSetupColumn().
// The intent is that advanced users willing to create customized headers would not need to use this helper
// and can create their own! For example: TableHeader() may be preceeded by Checkbox() or other custom widgets.
// See 'Demo->Tables->Custom headers' for a demonstration of implementing a custom version of this.
// This code is constructed to not make much use of internal functions, as it is intended to be a template to copy.
// FIXME-TABLE: TableOpenContextMenu() and TableGetHeaderRowHeight() are not public.
pub unsafe fn TableHeadersRow()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
     let table: *mut ImGuiTable = g.CurrentTable;
    // IM_ASSERT(table != NULL && "Need to call TableHeadersRow() after BeginTable()!");

    // Layout if not already done (this is automatically done by TableNextRow, we do it here solely to facilitate stepping in debugger as it is frequent to step in TableUpdateLayout)
    if (!table.IsLayoutLocked) {
        TableUpdateLayout(table);
    }

    // Open row
    let row_y1: c_float =  cursor_screen_pos(g).y;
    let row_height: c_float =  TableGetHeaderRowHeight();
    TableNextRow(ImGuiTableRowFlags_Headers, row_height);
    if (table.HostSkipItems) {// Merely an optimization, you may skip in your own code.
        return;
    }

    let columns_count: c_int = TableGetColumnCount();
    // for (let column_n: c_int = 0; column_n < columns_count; column_n++)
    for column_n in 0 .. columns_count
    {
        if (!TableSetColumnIndex(column_n)) {
            continue;
        }

        // Push an id to allow unnamed labels (generally accidental, but let's behave nicely with them)
        // - in your own code you may omit the PushID/PopID all-together, provided you know they won't collide
        // - table.InstanceCurrent is only >0 when we use multiple BeginTable/EndTable calls with same identifier.
        let mut  name: *const c_char = if TableGetColumnFlags(column_n) & ImGuiTableColumnFlags_NoHeaderLabel { str_to_const_c_char_ptr("")} else { TableGetColumnName(column_n)};
        push_int_id(g, table.InstanceCurrent * table.ColumnsCount + column_n);
        TableHeader(name);
        pop_win_id_from_stack(g);
    }

    // Allow opening popup from the right-most section after the last column.
    let mouse_pos: Vector2 = GetMousePos();
    if (IsMouseReleased(1) && TableGetHoveredColumn() == columns_count) {
        if (mouse_pos.y >= row_y1 && mouse_pos.y < row_y1 + row_height) {
            TableOpenContextMenu(-1);
        }
    }// Will open a non-column-specific popup.
}

// Emit a column header (text + optional sort order)
// We cpu-clip text here so that all columns headers can be merged into a same draw call.
// Note that because of how we cpu-clip and display sorting indicators, you _cannot_ use SameLine() after a TableHeader()
pub unsafe fn TableHeader(mut label: *const c_char)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window  = g.current_window_mut().unwrap();
    if window.skip_items { return ; }

     let table: *mut ImGuiTable = g.CurrentTable;
    // IM_ASSERT(table != NULL && "Need to call TableHeader() after BeginTable()!");
    // IM_ASSERT(table.CurrentColumn != -1);
    let column_n: c_int = table.CurrentColumn;
    let column: *mut ImGuiTableColumn = &mut table.Columns[column_n];

    // Label
    if label == None {
        label = str_to_const_c_char_ptr(""); }
    let mut  label_end: *const c_char = FindRenderedTextEnd(label, null());
    let label_size: Vector2 = CalcTextSize(label, label_end, true, 0.0);
    let label_pos: Vector2 = window.dc.cursor_pos;

    // If we already got a row height, there's use that.
    // FIXME-TABLE: Padding problem if the correct outer-padding CellBgRect strays off our ClipRect?
    let cell_r: ImRect =  TableGetCellBgRect(table, column_n);
    let label_height: c_float =  ImMax(label_size.y, table.RowMinHeight - table.CellPaddingY * 2.0);

    // Calculate ideal size for sort order arrow
    let mut w_arrow: c_float =  0.0;
    let mut w_sort_text: c_float =  0.0;
    sort_order_suf: [c_char;4] = [0;4];
    let ARROW_SCALE: c_float =  0.65f32;
    if flag_set(table.Flags, ImGuiTableFlags_Sortable) && flag_clear(column.Flags, ImGuiTableColumnFlags_NoSort)
    {
        w_arrow = ImFloor(g.FontSize * ARROW_SCALE + g.style.FramePadding.x);
        if column.SortOrder > 0
        {
            // ImFormatString(sort_order_suf, sort_order_su0f32.len(), "{}", column.SortOrder + 1);
            w_sort_text = g.style.ItemInnerSpacing.x + CalcTextSize(sort_order_suf, None, false, 0.0).x;
        }
    }

    // We feed our unclipped width to the column without writing on CursorMaxPos, so that column is still considering for merging.
    let max_pos_x: c_float =  label_pos.x + label_size.x + w_sort_text + w_arrow;
    column.ContentMaxXHeadersUsed = ImMax(column.ContentMaxXHeadersUsed, column.WorkMaxX);
    column.ContentMaxXHeadersIdeal = ImMax(column.ContentMaxXHeadersIdeal, max_pos_x);

    // Keep header highlighted when context menu is open.
    let selected: bool = (table.IsContextPopupOpen && table.ContextPopupColumn == column_n as ImGuiTableColumnIdx && table.InstanceInteracted == table.InstanceCurrent as i16);
    let mut id: ImguiHandle =  window.id_from_str(label);
    let mut bb: ImRect = ImRect::new(cell_r.min.x, cell_r.min.y, cell_r.max.x, ImMax(cell_r.max.y, cell_r.min.y + label_height + g.style.CellPadding.y * 2.0));
    ItemSize(g, &Vector2::from_floats(0.0, label_height), 0.0); // Don't declare unclipped width, it'll be fed ContentMaxPosHeadersIdeal
    if !ItemAdd(g, &mut bb, id, None, 0) { return ; }

    //GetForegroundDrawList().AddRect(cell_r.Min, cell_r.Max, IM_COL32(255, 0, 0, 255)); // [DEBUG]
    //GetForegroundDrawList().AddRect(bb.Min, bb.Max, IM_COL32(255, 0, 0, 255)); // [DEBUG]

    // Using AllowItemOverlap mode because we cover the whole cell, and we want user to be able to submit subsequent items.
    let mut hovered = false;
    let mut held = false;
    let mut pressed: bool =  ButtonBehavior(bb, id, &hovered, &held, ImGuiButtonFlags_AllowItemOverlap);
    if g.ActiveId != id {
        SetItemAllowOverlap();
    }
    if held || hovered || selected
    {
        let col: u32 = GetColorU32(if held { ImGuiCol_HeaderActive } else { if hovered { ImGuiCol_HeaderHovered } else { ImGuiCol_Header } }, 0.0);
        //RenderFrame(bb.Min, bb.Max, col, false, 0.0);
        TableSetBgColor(ImGuiTableBgTarget_CellBg, col, table.CurrentColumn);
    }
    else
    {
        // Submit single cell bg color in the case we didn't submit a full header row
        if (table.RowFlags & ImGuiTableRowFlags_Headers) == 0 {
            TableSetBgColor(ImGuiTableBgTarget_CellBg, GetColorU32(ImGuiCol_TableHeaderBg, 0.0), table.CurrentColumn);
        }
    }
    RenderNavHighlight(, &bb, id, ImGuiNavHighlightFlags_TypeThin | ImGuiNavHighlightFlags_NoRounding);
    if held{
        table.HeldHeaderColumn = column_n as ImGuiTableColumnIdx;}
    window.dc.cursor_pos.y -= g.style.ItemSpacing.y * 0.5;

    // Drag and drop to re-order columns.
    // FIXME-TABLE: Scroll request while reordering a column and it lands out of the scrolling zone.
    if (held && flag_set(table.Flags, ImGuiTableFlags_Reorderable) && IsMouseDragging(0, 0.0) && !g.DragDropActive)
    {
        // While moving a column it will jump on the other side of the mouse, so we also test for MouseDelta.x
        table.ReorderColumn = column_n as ImGuiTableColumnIdx;
        table.InstanceInteracted = table.InstanceCurrent as i16;

        // We don't reorder: through the frozen<>unfrozen line, or through a column that is marked with ImGuiTableColumnFlags_NoReorder.
        if (g.IO.MouseDelta.x < 0.0 && g.IO.MousePos.x < cell_r.min.x) {
            let prev_column: *mut ImGuiTableColumn = if (column.PrevEnabledColumn != -1) { &mut table.Columns[column.PrevEnabledColumn] } else { None };
            if prev_column.is_null() == false {
                if !((column.Flags | prev_column.Flags) & ImGuiTableColumnFlags_NoReorder) {
                    if ((column.IndexWithinEnabledSet < table.FreezeColumnsRequest) == (prev_column.IndexWithinEnabledSet < table.FreezeColumnsRequest)) {
                        table.ReorderColumnDir = -1;
                    }
                }
            }
        }
        if (g.IO.MouseDelta.x > 0.0 && g.IO.MousePos.x > cell_r.max.x) {
            let next_column: *mut ImGuiTableColumn = if (column.NextEnabledColumn != -1) { &mut table.Columns[column.NextEnabledColumn] } else { None };
            if next_column.is_null() == false {
                if (!((column.Flags | next_column.Flags) & ImGuiTableColumnFlags_NoReorder)) {
                    if (column.IndexWithinEnabledSet < table.FreezeColumnsRequest) == (next_column.IndexWithinEnabledSet < table.FreezeColumnsRequest) {
                        table.ReorderColumnDir = 1;
                    }
                }
            }
        }
    }

    // Sort order arrow
    let ellipsis_max: c_float =  cell_r.max.x - w_arrow - w_sort_text;
    if flag_set(table.Flags, ImGuiTableFlags_Sortable) && flag_clear(column.Flags, ImGuiTableColumnFlags_NoSort)
    {
        if column.SortOrder != -1
        {
            let mut x: c_float =  ImMax(cell_r.min.x, cell_r.max.x - w_arrow - w_sort_text);
            let y: c_float =  label_pos.y;
            if column.SortOrder > 0
            {
                PushStyleColor(ImGuiCol_Text, GetColorU32(ImGuiCol_Text, 0.70));
                RenderText(Vector2::from_floats(x + g.style.ItemInnerSpacing.x, y), sort_order_su0f32, None, false);
                PopStyleColor(0);
                x += w_sort_text;
            }
            RenderArrow(window.DrawList, Vector2::from_floats(x, y), GetColorU32(ImGuiCol_Text, 0.0), if column.SortDirection == ImGuiSortDirection_Ascending { ImGuiDir_Up } else { ImGuiDir_Down }, ARROW_SCALE);
        }

        // Handle clicking on column header to adjust Sort Order
        if pressed && table.ReorderColumn != column_n as ImGuiTableColumnIdx
        {
            sort_direction: ImGuiSortDirection = TableGetColumnNextSortDirection(column);
            TableSetColumnSortDirection(column_n, sort_direction, g.IO.KeyShift);
        }
    }

    // Render clipped label. Clipping here ensure that in the majority of situations, all our header cells will
    // be merged into a single draw call.
    //window.DrawList.AddCircleFilled(ImVec2::new(ellipsis_max, label_pos.y), 40, IM_COL32_WHITE);
    RenderTextEllipsis(window.DrawList, &label_pos, &Vector2::from_floats(ellipsis_max, label_pos.y + label_height + g.style.FramePadding.y), ellipsis_max, ellipsis_max, label, label_end, &label_size);

    let text_clipped: bool = label_size.x > (ellipsis_max - label_pos.x);
    if text_clipped && hovered && g.ActiveId == 0 && IsItemHovered(ImGuiHoveredFlags_DelayNormal) {
        SetTooltip("%.*s", (label_end - label), label);
    }

    // We don't use BeginPopupContextItem() because we want the popup to stay up even after the column is hidden
    if IsMouseReleased(1) && IsItemHovered(0) {
        TableOpenContextMenu(column_n);}
}

//-------------------------------------------------------------------------
// [SECTION] Tables: Context Menu
//-------------------------------------------------------------------------
// - TableOpenContextMenu() [Internal]
// - TableDrawContextMenu() [Internal]
//-------------------------------------------------------------------------

// Use -1 to open menu not specific to a given column.
pub unsafe fn TableOpenContextMenu(mut column_n: c_int)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
     let table: *mut ImGuiTable = g.CurrentTable;
    if (column_n == -1 && table.CurrentColumn != -1) {  // When called within a column automatically use this one (for consistency)
        column_n = table.CurrentColumn;
    }
    if (column_n == table.ColumnsCount) {             // To facilitate using with TableGetHoveredColumn()
        column_n = -1;
    }
    // IM_ASSERT(column_n >= -1 && column_n < table.ColumnsCount);
    if (table.Flags & (ImGuiTableFlags_Resizable | ImGuiTableFlags_Reorderable | ImGuiTableFlags_Hideable))
    {
        table.IsContextPopupOpen = true;
        table.ContextPopupColumn = column_n as ImGuiTableColumnIdx;
        table.InstanceInteracted = table.InstanceCurrent as i16;
        let mut context_menu_id: ImguiHandle =  hash_string(str_to_const_c_char_ptr("##ContextMenu"), table.ID as u32);
        OpenPopupEx(g, context_menu_id, ImGuiPopupFlags_None);
    }
}

pub unsafe fn TableBeginContextMenuPopup(table: *mut ImGuiTable) -> bool
{
    if !table.IsContextPopupOpen || table.InstanceCurrent != table.InstanceInteracted as c_int { return  false; }
    let mut context_menu_id: ImguiHandle =  hash_string(str_to_const_c_char_ptr("##ContextMenu"), table.ID as u32);
    if BeginPopupEx(context_menu_id, ImGuiWindowFlags_AlwaysAutoResize | ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoSavedSettings) { return  true; }
    table.IsContextPopupOpen = false;
    return false;
}

// Output context menu into current window (generally a popup)
// FIXME-TABLE: Ideally this should be writable by the user. Full programmatic access to that data?
pub unsafe fn TableDrawContextMenu(table: *mut ImGuiTable)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window  = g.current_window_mut().unwrap();
    if window.skip_items { return ; }

    let mut want_separator: bool =  false;
    let column_n: c_int = if table.ContextPopupColumn >= 0 && table.ContextPopupColumn < table.ColumnsCount as ImGuiTableColumnIdx { table.ContextPopupColumn } else { -1 } as c_int;
    let column: *mut ImGuiTableColumn = if column_n != -1 { &mut table.Columns[column_n]} else { None};

    // Sizing
    if flag_set(table.Flags, ImGuiTableFlags_Resizable)
    {
        if (column != null_mut())
        {
            let can_resize: bool = flag_clear(column.Flags, ImGuiTableColumnFlags_NoResize) && column.IsEnabled;
            if (MenuItem("Size column to fit###SizeOne", None, false, can_resize)) {
                TableSetColumnWidthAutoSingle(table, column_n);
            }
        }
let size_all_desc: *const c_char;
        if table.ColumnsEnabledFixedCount == table.ColumnsEnabledCount && (table.Flags & ImGuiTableFlags_SizingMask_) != ImGuiTableFlags_SizingFixedSame {
            size_all_desc = str_to_const_c_char_ptr("Size all columns to fit###SizeAll");
        }     // All fixed
        else {
            size_all_desc = str_to_const_c_char_ptr("Size all columns to default###SizeAll");
        }   // All stretch or mixed
        if (MenuItem(size_all_desc, null_mut())) {
            TableSetColumnWidthAutoAll(table);
        }
        want_separator = true;
    }

    // Ordering
    if flag_set(table.Flags, ImGuiTableFlags_Reorderable)
    {
        if (MenuItem("Reset order", None, false, !table.IsDefaultDisplayOrder)) {
            table.IsResetDisplayOrderRequest = true;
        }
        want_separator = true;
    }

    // Reset all (should work but seems unnecessary/noisy to expose?)
    //if (MenuItem("Reset all"))
    //    table.IsResetAllRequest = true;

    // Sorting
    // (modify TableOpenContextMenu() to add _Sortable flag if enabling this)
// #if 0
    if (flag_set(table.Flags , ImGuiTableFlags_Sortable) && column != None && flag_clear(column.Flags, ImGuiTableColumnFlags_NoSort))
    {
        if want_separator {
            Separator(); }
        want_separator = true;

        let mut append_to_sort_specs: bool =  g.IO.KeyShift;
        if (MenuItem("Sort in Ascending Order", None, column.SortOrder != -1 && column.SortDirection == ImGuiSortDirection_Ascending, flag_clear(column.Flags, ImGuiTableColumnFlags_NoSortAscending))) {
            TableSetColumnSortDirection( column_n, ImGuiSortDirection_Ascending, append_to_sort_specs);
        }
        if (MenuItem("Sort in Descending Order", None, column.SortOrder != -1 && column.SortDirection == ImGuiSortDirection_Descending, flag_clear(column.Flags, ImGuiTableColumnFlags_NoSortDescending))) {
            TableSetColumnSortDirection(column_n, ImGuiSortDirection_Descending, append_to_sort_specs);
        }
    }
// #endif

    // Hiding / Visibility
    if flag_set(table.Flags, ImGuiTableFlags_Hideable)
    {
        if want_separator {
            Separator(); }
        want_separator = true;

        PushItemFlag(ImGuiItemFlags_SelectableDontClosePopup, true);
        // for (let other_column_n: c_int = 0; other_column_n < table.ColumnsCount; other_column_n++)
        for other_column_n in 0 .. table.ColumnsCount
        {
            other_column: *mut ImGuiTableColumn = &mut table.Columns[other_column_n];
            if other_column.Flags & ImGuiTableColumnFlags_Disabled{
                continue;}

            let mut  name: *const c_char = TableGetColumnName(other_column_n);
            if name == None || name[0] == 0 {
                name = str_to_const_c_char_ptr("<Unknown>");
            }

            // Make sure we can't hide the last active column
            let mut menu_item_active: bool =  if flag_set(other_column.Flags, ImGuiTableColumnFlags_NoHide) { false } else { true };
            if other_column.IsUserEnabled && table.ColumnsEnabledCount <= 1 {
                menu_item_active = false;}
            if (MenuItem(name, None, other_column.IsUserEnabled, menu_item_active)) {
                other_column.IsUserEnabledNextFrame = !other_column.IsUserEnabled;
            }
        }
        PopItemFlag();
    }
}

//-------------------------------------------------------------------------
// [SECTION] Tables: Settings (.ini data)
//-------------------------------------------------------------------------
// FIXME: The binding/finding/creating flow are too confusing.
//-------------------------------------------------------------------------
// - TableSettingsInit() [Internal]
// - TableSettingsCalcChunkSize() [Internal]
// - TableSettingsCreate() [Internal]
// - TableSettingsFindByID() [Internal]
// - TableGetBoundSettings() [Internal]
// - TableResetSettings()
// - TableSaveSettings() [Internal]
// - TableLoadSettings() [Internal]
// - TableSettingsHandler_ClearAll() [Internal]
// - TableSettingsHandler_ApplyAll() [Internal]
// - TableSettingsHandler_ReadOpen() [Internal]
// - TableSettingsHandler_ReadLine() [Internal]
// - TableSettingsHandler_WriteAll() [Internal]
// - TableSettingsInstallHandler() [Internal]
//-------------------------------------------------------------------------
// [Init] 1: TableSettingsHandler_ReadXXXX()   Load and parse .ini file into TableSettings.
// [Main] 2: TableLoadSettings()               When table is created, bind Table to TableSettings, serialize TableSettings data into Table.
// [Main] 3: TableSaveSettings()               When table properties are modified, serialize Table data into bound or new TableSettings, mark .ini as dirty.
// [Main] 4: TableSettingsHandler_WriteAll()   When .ini file is dirty (which can come from other source), save TableSettings into .ini file.
//-------------------------------------------------------------------------

// Clear and initialize empty settings instance
pub unsafe fn TableSettingsInit(settings: *mut ImGuiTableSettings, id: ImguiHandle, columns_count: c_int, columns_count_max: c_int)
{
    // IM_PLACEMENT_NEW(settings) ImGuiTableSettings();
    let mut settings = ImGuiTableSettings::default();
    let mut settings_column = settings.GetColumnSettings();
    // for (let n: c_int = 0; n < columns_count_max; n++, settings_column++)
    for n in 0 .. columns_count_max
    {
        // IM_PLACEMENT_NEW(settings_column)

        // ImGuiTableColumnSettings();
        settings_column = ImGuiTableColumnSettings::default().borrow_mut();
        settings_column += 1;
    }
    settings.ID = id;
    settings.ColumnsCount = columns_count as ImGuiTableColumnIdx;
    settings.ColumnsCountMax = columns_count_max as ImGuiTableColumnIdx;
    settings.WantApply = true;
}

pub unsafe fn TableSettingsCalcChunkSize(columns_count: c_int) -> size_t
{
    return sizeof(ImGuiTableSettings) + columns_count * sizeof(ImGuiTableColumnSettings);
}

pub unsafe fn TableSettingsCreate(id: ImguiHandle, columns_count: c_int) -> *mut ImGuiTableSettings
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let settings: *mut ImGuiTableSettings = g.SettingsTables.alloc_chunk(TableSettingsCalcChunkSize(columns_count));
    TableSettingsInit(settings, id, columns_count, columns_count);
    return settings;
}

// Find existing settings
pub unsafe fn TableSettingsFindByID(id: ImguiHandle) ->  *mut ImGuiTableSettings
{
    // FIXME-OPT: Might want to store a lookup map for this?
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // for (settings: *mut ImGuiTableSettings = g.SettingsTables.begin(); settings != None; settings = g.SettingsTables.next_chunk(settings))
    for settings in g.SettingsTables
    {
        if settings.ID == id { return settings; }
    }
    return None;
}

// Get settings for a given table, NULL if none
pub unsafe fn TableGetBoundSettings(table: *mut ImGuiTable) -> *mut ImGuiTableSettings
{
    if table.SettingsOffset != -1
    {
        let g = GImGui; // ImGuiContext& g = *GImGui;
        let settings: *mut ImGuiTableSettings = g.SettingsTables.ptr_from_offset(table.SettingsOffset);
        // IM_ASSERT(settings.ID == table.ID);
        if settings.ColumnsCountMax >= table.ColumnsCount as ImGuiTableColumnIdx { return  settings; } // OK
        settings.ID = 0; // Invalidate storage, we won't fit because of a count change
    }
    return None;
}

// Restore initial state of table (with or without saved settings)
pub unsafe fn TableResetSettings(table: *mut ImGuiTable)
{
    table.IsInitializing = true;
    table.IsSettingsDirty = true;
    table.IsResetAllRequest = false;
    table.IsSettingsRequestLoad = false;                   // Don't reload from ini
    table.SettingsLoadedFlags = ImGuiTableFlags_None;      // Mark as nothing loaded so our initialized data becomes authoritative
}

pub unsafe fn TableSaveSettings(table: *mut ImGuiTable)
{
    table.IsSettingsDirty = false;
    if table.Flags & ImGuiTableFlags_NoSavedSettings { return ; }

    // Bind or create settings data
    let g = GImGui; // ImGuiContext& g = *GImGui;
    settings: *mut ImGuiTableSettings = TableGetBoundSettings(table);
    if (settings == null_mut())
    {
        settings = TableSettingsCreate(table.ID, table.ColumnsCount);
        table.SettingsOffset = g.SettingsTables.offset_from_ptr(settings);
    }
    settings.ColumnsCount = table.ColumnsCount;

    // Serialize ImGuiTable/ImGuiTableColumn into ImGuiTableSettings/ImGuiTableColumnSettings
    // IM_ASSERT(settings.ID == table.ID);
    // IM_ASSERT(settings.ColumnsCount == table.ColumnsCount && settings.ColumnsCountMax >= settings.ColumnsCount);
    let mut column: *mut ImGuiTableColumn = table.Columns.Data;
    let mut column_settings = settings.GetColumnSettings();

    let mut save_ref_scale: bool =  false;
    settings.SaveFlags = ImGuiTableFlags_None;
    // for (let n: c_int = 0; n < table.ColumnsCount; n++, column++, column_settings++)
   for n in 0 .. table.ColumnsCount
    {
        let width_or_weight: c_float =  if flag_set(column.Flags, ImGuiTableColumnFlags_WidthStretch) { column.StretchWeight } else { column.WidthRequest };
        column_settings.WidthOrWeight = width_or_weight;
        column_settings.Index = n;
        column_settings.DisplayOrder = column.DisplayOrder;
        column_settings.SortOrder = column.SortOrder;
        column_settings.SortDirection = column.SortDirection;
        column_settings.IsEnabled = column.IsUserEnabled;
        column_settings.IsStretch = if column.Flags & ImGuiTableColumnFlags_WidthStretch { 1} else { 0};
        if flag_clear(column.Flags, ImGuiTableColumnFlags_WidthStretch) {
            save_ref_scale = true;}

        // We skip saving some data in the .ini file when they are unnecessary to restore our state.
        // Note that fixed width where initial width was derived from auto-fit will always be saved as InitStretchWeightOrWidth will be 0.0.
        // FIXME-TABLE: We don't have logic to easily compare SortOrder to DefaultSortOrder yet so it's always saved when present.
        if (width_or_weight != column.InitStretchWeightOrWidth) {
            settings.SaveFlags |= ImGuiTableFlags_Resizable;
        }
        if (column.DisplayOrder != n as ImGuiTableColumnIdx) {
            settings.SaveFlags |= ImGuiTableFlags_Reorderable;
        }
        if (column.SortOrder != -1) {
            settings.SaveFlags |= ImGuiTableFlags_Sortable;
        }
        if (column.IsUserEnabled != ((column.Flags & ImGuiTableColumnFlags_DefaultHide) == 0)) {
            settings.SaveFlags |= ImGuiTableFlags_Hideable;
        }
        column += 1;
        column_settings += 1;
    }
    settings.SaveFlags &= table.Flags;
    settings.RefScale = if save_ref_scale { table.RefScale } else { 0.0 };

    MarkIniSettingsDirty();
}

pub unsafe fn TableLoadSettings(table: *mut ImGuiTable)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    table.IsSettingsRequestLoad = false;
    if table.Flags & ImGuiTableFlags_NoSavedSettings { return ; }

    // Bind settings
    settings: *mut ImGuiTableSettings;
    if (table.SettingsOffset == -1)
    {
        settings = TableSettingsFindByID(table.ID);
        if settings == None { return ; }
        if (settings.ColumnsCount != table.ColumnsCount) { // Allow settings if columns count changed. We could otherwise decide to return...
            table.IsSettingsDirty = true;
        }
        table.SettingsOffset = g.SettingsTables.offset_from_ptr(settings);
    }
    else
    {
        settings = TableGetBoundSettings(table);
    }

    table.SettingsLoadedFlags = settings.SaveFlags;
    table.RefScale = settings.RefScale;

    // Serialize ImGuiTableSettings/ImGuiTableColumnSettings into ImGuiTable/ImGuiTableColumn
    let mut column_settings: *mut ImGuiTableColumnSettings = settings.GetColumnSettings();
    let mut  display_order_mask = 0;
    // for (let data_n: c_int = 0; data_n < settings.ColumnsCount; data_n++, column_settings++)
    for data_n in 0 .. settings.ColumnsCount
    {
        let column_n: c_int = column_settings.Index as c_int;
        if column_n < 0 || column_n >= table.ColumnsCount{
            continue;}

        let column: *mut ImGuiTableColumn = &mut table.Columns[column_n];
        if settings.SaveFlags & ImGuiTableFlags_Resizable
        {
            if (column_settings.IsStretch){
            column.StretchWeight = column_settings.WidthOrWeight;
        }
            else{
            column.WidthRequest = column_settings.WidthOrWeight;
        }
            column.AutoFitQueue = 0x00;
        }
        if (settings.SaveFlags & ImGuiTableFlags_Reorderable) {
            column.DisplayOrder = column_settings.DisplayOrder;
        }
        else{
        column.DisplayOrder = column_n as ImGuiTableColumnIdx;
    }
        display_order_mask |= 1 << column.DisplayOrder;
        column.IsUserEnabled = column_settings.IsEnabled;
        column.IsUserEnabledNextFrame = column_settings.IsEnabled;
        column.SortOrder = column_settings.SortOrder;
        column.SortDirection = column_settings.SortDirection;
        column_settings += 1;
    }

    // Validate and fix invalid display order data
    let expected_display_order_mask = if settings.ColumnsCount == 64 { !0} else { (1 << settings.ColumnsCount) - 1};
    if (display_order_mask != expected_display_order_mask) {
        // for (let column_n: c_int = 0; column_n < table.ColumnsCount; column_n+ +)
        for column_n in 0 .. table.ColumnsCount
        {
            table.Columns[column_n].DisplayOrder = column_n;
        }
    }

    // Rebuild index
    // for (let column_n: c_int = 0; column_n < table.ColumnsCount; column_n++)
    for column_n in 0 .. table.ColumnsCount
    {
        table.DisplayOrderToIndex[table.Columns[column_n].DisplayOrder] = column_n;
    }
}

pub unsafe fn TableSettingsHandler_ClearAll(g: &mut AppContext, handler: *mut SettingsHandler)
{
    let g =  ctx;
    // for (let i: c_int = 0; i != g.Tables.GetMapSize(); i++)
    for i in 0 .. g.Tables.GetMapSize()
    {
        let mut table: *mut ImGuiTable = g.Tables.TryGetMapData(i);
        if table.is_null() == false {
            table.SettingsOffset = -1;
        }
    }
    g.SettingsTables.clear();
}

// Apply to existing windows (if any)
pub unsafe fn TableSettingsHandler_ApplyAll(g: &mut AppContext, handler: *mut SettingsHandler)
{
    let g =  ctx;
    // for (let i: c_int = 0; i != g.Tables.GetMapSize(); i++)
    for i in 0 .. g.Tables.GetMapSize()
    {
        let table: *mut ImGuiTable = g.Tables.TryGetMapData(i);
        if table.is_null() == false {
            table.IsSettingsRequestLoad = true;
            table.SettingsOffset = -1;
        }
    }
}

pub unsafe fn TableSettingsHandler_ReadOpen(g: &mut AppContext, handler: *mut SettingsHandler, name: *const c_char) -> *mut c_void
{
    let mut id: ImguiHandle =  0;
    let columns_count: c_int = 0;
    if (libc::sscanf(name, str_to_const_c_char_ptr("0x{},{}"), &id, &columns_count) < 2) {
        return None;
    }

    let settings: *mut ImGuiTableSettings = TableSettingsFindByID(id);
    if settings.is_null() == false
    {
        if (settings.ColumnsCountMax >= columns_count as ImGuiTableColumnIdx)
        {
            TableSettingsInit(settings, id, columns_count, settings.ColumnsCountMax as c_int); // Recycle
            return settings;
        }
        settings.ID = 0; // Invalidate storage, we won't fit because of a count change
    }
    return TableSettingsCreate(id, columns_count);
}

pub unsafe fn TableSettingsHandler_ReadLine(g: &mut AppContext, handler: *mut SettingsHandler, entry: *mut c_void, line: &mut c_char)
{
    // // "Column 0  UserID=0x42AD2D21 Width=100 Visible=1 Order=0 Sort=0v"
    // let settings: *mut ImGuiTableSettings = entry;
    // let mut f: c_float =  0.0;
    // let mut column_n: c_int = 0;
    // let mut r: c_int = 0;
    // let mut n: c_int = 0;
    //
    // if (libc::sscanf(line, str_to_const_c_char_ptr("RefScale={}"), &0.0) == 1) { settings.RefScale = f; return; }
    //
    // if (libc::sscanf(line, str_to_const_c_char_ptr("Column {}%n"), &column_n, &r) == 1)
    // {
    //     if column_n < 0 || column_n >= settings.ColumnsCount as c_int { return ; }
    //     line = ImStrSkipBlank(line + r);
    //      c: c_char = 0;
    //     let  column: *mut ImGuiTableColumnSettings = settings.GetColumnSettings() + column_n;
    //     column.Index = column_n;
    //     if (libc::sscanf(line, "UserID=0x{}%n", (*mut u32)&n, &r)==1) { line = ImStrSkipBlank(line + r); column.UserID = n; }
    //     if (sscanf(line, "Width={}%n", &n, &r) == 1)            { line = ImStrSkipBlank(line + r); column.WidthOrWeight = n; column.IsStretch = 0; settings.SaveFlags |= ImGuiTableFlags_Resizable; }
    //     if (sscanf(line, "Weight={}%n", &f, &r) == 1)           { line = ImStrSkipBlank(line + r); column.WidthOrWeight = f; column.IsStretch = 1; settings.SaveFlags |= ImGuiTableFlags_Resizable; }
    //     if (sscanf(line, "Visible={}%n", &n, &r) == 1)          { line = ImStrSkipBlank(line + r); column.IsEnabled = n; settings.SaveFlags |= ImGuiTableFlags_Hideable; }
    //     if (sscanf(line, "Order={}%n", &n, &r) == 1)            { line = ImStrSkipBlank(line + r); column.DisplayOrder = n; settings.SaveFlags |= ImGuiTableFlags_Reorderable; }
    //     if (sscanf(line, "Sort={}%c%n", &n, &c, &r) == 2)       { line = ImStrSkipBlank(line + r); column.SortOrder = n; column.SortDirection = if (c == '^') { ImGuiSortDirection_Descending } else { ImGuiSortDirection_Ascending }; settings.SaveFlags |= ImGuiTableFlags_Sortable; }
    // }
}

pub unsafe fn TableSettingsHandler_WriteAll(g: &mut AppContext, handler: *mut SettingsHandler, buf: *mut ImGuiTextBuffer)
{
    // let g =  ctx;
    // for (settings: *mut ImGuiTableSettings = g.SettingsTables.begin(); settings != None; settings = g.SettingsTables.next_chunk(settings))
    // {
    //     if (settings.ID == 0) // Skip ditched settings
    //         continue;
    //
    //     // TableSaveSettings() may clear some of those flags when we establish that the data can be stripped
    //     // (e.g. Order was unchanged)
    //     let save_size: bool = (settings.SaveFlags & ImGuiTableFlags_Resizable) != 0;
    //     let save_visible: bool = (settings.SaveFlags & ImGuiTableFlags_Hideable) != 0;
    //     let save_order: bool = (settings.SaveFlags & ImGuiTableFlags_Reorderable) != 0;
    //     let save_sort: bool = (settings.SaveFlags & ImGuiTableFlags_Sortable) != 0;
    //     if (!save_size && !save_visible && !save_order && !save_sort)
    //         continue;
    //
    //     buf->reserve(buf->size() + 30 + settings.ColumnsCount * 50); // ballpark reserve
    //     buf->appendf("[{}][0x{},{}]\n", handler.TypeName, settings.ID, settings.ColumnsCount);
    //     if (settings.RefScale != 0.0)
    //         buf->appendf("RefScale=%g\n", settings.RefScale);
    //     *mut ImGuiTableColumnSettings column = settings.GetColumnSettings();
    //     for (let column_n: c_int = 0; column_n < settings.ColumnsCount; column_n++, column++)
    //     {
    //         // "Column 0  UserID=0x42AD2D21 Width=100 Visible=1 Order=0 Sort=0v"
    //         let mut save_column: bool =  column.UserID != 0 || save_size || save_visible || save_order || (save_sort && column.SortOrder != -1);
    //         if (!save_column)
    //             continue;
    //         buf->appendf("Column %-2d", column_n);
    //         if (column.UserID != 0)                    buf->appendf(" UserID={}", column.UserID);
    //         if (save_size && column.IsStretch)         buf->appendf(" Weight=%.4f", column.WidthOrWeight);
    //         if (save_size && !column.IsStretch)        buf->appendf(" Width={}", column.WidthOrWeight);
    //         if (save_visible)                           buf->appendf(" Visible={}", column.IsEnabled);
    //         if (save_order)                             buf->appendf(" Order={}", column.DisplayOrder);
    //         if (save_sort && column.SortOrder != -1)   buf->appendf(" Sort={}%c", column.SortOrder, (column.SortDirection == ImGuiSortDirection_Ascending) ? 'v' : '^');
    //         buf->append("\n");
    //     }
    //     buf->append("\n");
    // }
}

pub fn TableSettingsAddSettingsHandler(g: &mut AppContext)
{
    let mut ini_handler: SettingsHandler::default();
    ini_handler.TypeName = "Table";
    ini_handler.TypeHash = hash_string(str_to_const_c_char_ptr("Table"), 0);
    ini_handler.ClearAllFn = TableSettingsHandler_ClearAll;
    ini_handler.ReadOpenFn = TableSettingsHandler_ReadOpen;
    ini_handler.ReadLineFn = TableSettingsHandler_ReadLine;
    ini_handler.ApplyAllFn = TableSettingsHandler_ApplyAll;
    ini_handler.WriteAllFn = TableSettingsHandler_WriteAll;
    // AddSettingsHandler(g, &ini_handler);
    g.add_settings_handler(&ini_handler);
}

//-------------------------------------------------------------------------
// [SECTION] Tables: Garbage Collection
//-------------------------------------------------------------------------
// - TableRemove() [Internal]
// - TableGcCompactTransientBuffers() [Internal]
// - TableGcCompactSettings() [Internal]
//-------------------------------------------------------------------------

// Remove Table (currently only used by TestEngine)
pub unsafe fn TableRemove(table: *mut ImGuiTable)
{
    //IMGUI_DEBUG_PRINT("TableRemove() id=0x{}\n", table.ID);
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let table_idx: c_int = g.Tables.GetIndex(table);
    //memset(table.RawData.Data, 0, table.RawData.size_in_bytes());
    //memset(table, 0, sizeof(ImGuiTable));
    g.Tables.Remove(table.ID, table);
    g.TablesLastTimeActive[table_idx] = -1.0;
}

// Free up/compact internal Table buffers for when it gets unused
pub unsafe fn TableGcCompactTransientBuffers(table: *mut ImGuiTable)
{
    //IMGUI_DEBUG_PRINT("TableGcCompactTransientBuffers() id=0x{}\n", table.ID);
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(table.MemoryCompacted == false);
    table.SortSpecs.Specs= None;
    table.SortSpecsMulti.clear();
    table.IsSortSpecsDirty = true; // FIXME: shouldn't have to leak into user performing a sort
    table.ColumnsNames.clear();
    table.MemoryCompacted = true;
    // for (let n: c_int = 0; n < table.ColumnsCount; n++)
    for n in 0 .. table.ColumnsCount
    {
        table.Columns[n].NameOffset = -1;
    }
    g.TablesLastTimeActive[g.Tables.GetIndex(table)] = -1.0;
}

pub unsafe fn TableGcCompactTransientBuffers2(temp_data: *mut ImGuiTableTempData )
{
    temp_Data.DrawSplitter.ClearFreeMemory();
    temp_Data.LastTimeActive = -1.0;
}

// Compact and remove unused settings data (currently only used by TestEngine)
pub unsafe fn TableGcCompactSettings()
{
    // let g = GImGui; // ImGuiContext& g = *GImGui;
    // let required_memory: c_int = 0;
    // for (settings: *mut ImGuiTableSettings = g.SettingsTables.begin(); settings != None; settings = g.SettingsTables.next_chunk(settings))
    //     if (settings.ID != 0)
    //         required_memory += TableSettingsCalcChunkSize(settings.ColumnsCount);
    // if required_memory == g.SettingsTables.Buf.Size { return ; }
    // ImChunkStream<ImGuiTableSettings> new_chunk_stream;
    // new_chunk_stream.Buf.reserve(required_memory);
    // for (settings: *mut ImGuiTableSettings = g.SettingsTables.begin(); settings != None; settings = g.SettingsTables.next_chunk(settings))
    //     if (settings.ID != 0)
    //         memcpy(new_chunk_stream.alloc_chunk(TableSettingsCalcChunkSize(settings.ColumnsCount)), settings, TableSettingsCalcChunkSize(settings.ColumnsCount));
    // g.SettingsTables.swap(new_chunk_stream);
}


//-------------------------------------------------------------------------
// [SECTION] Tables: Debugging
//-------------------------------------------------------------------------
// - DebugNodeTable() [Internal]
//-------------------------------------------------------------------------

// #ifndef IMGUI_DISABLE_DEBUG_TOOLS

pub fn DebugNodeTableGetSizingPolicyDesc(sizing_policy: ImGuiTableFlags) -> *const c_char
{
    // sizing_policy &= ImGuiTableFlags_SizingMask_;
    // if (sizing_policy == ImGuiTableFlags_SizingFixedFit)    { return "FixedFit"; }
    // if (sizing_policy == ImGuiTableFlags_SizingFixedSame)   { return "FixedSame"; }
    // if (sizing_policy == ImGuiTableFlags_SizingStretchProp) { return "StretchProp"; }
    // if (sizing_policy == ImGuiTableFlags_SizingStretchSame) { return "StretchSame"; }
    // return "N/A";
    todo!()
}

pub unsafe fn DebugNodeTable(table: *mut ImGuiTable)
{
//     buf: [c_char;512];
//     p: *mut c_char = buf;
//     let mut  buf_end: *const c_char = buf + buf.len();
//     let is_active: bool = (table.LastFrameActive >= GetFrameCount() - 2); // Note that fully clipped early out scrolling tables will appear as inactive here.
//     ImFormatString(p, buf_end - p, "Table 0x{} ({} columns, in '{}'){}", table.ID, table.ColumnsCount, table.Outerwindow.Name, is_active ? "" : " *Inactive*");
//     if (!is_active) { PushStyleColor(ImGuiCol_Text, GetStyleColorVec4(ImGuiCol_TextDisabled)); }
//     let mut open: bool =  TreeNode(table, "{}", buf);
//     if (!is_active) { PopStyleColor(); }
//     if (IsItemHovered())
//         GetForegroundDrawList().AddRect(table.OuterRect.Min, table.OuterRect.Max, IM_COL32(255, 255, 0, 255));
//     if (IsItemVisible() && table.HoveredColumnBody != -1)
//         GetForegroundDrawList().AddRect(GetItemRectMin(), GetItemRectMax(), IM_COL32(255, 255, 0, 255));
//     if !open { return ; }
//     if (table.InstanceCurrent > 0)
//         Text("** {} instances of same table! Some data below will refer to last instance.", table.InstanceCurrent + 1);
//     let mut clear_settings: bool =  SmallButton("Clear settings");
//     BulletText("OuterRect: Pos: ({},{}) Size: ({},{}) Sizing: '{}'", table.OuterRect.Min.x, table.OuterRect.Min.y, table.OuterRect.GetWidth(), table.OuterRect.GetHeight(), DebugNodeTableGetSizingPolicyDesc(table.Flags));
//     BulletText("ColumnsGivenWidth: {}, ColumnsAutoFitWidth: {}, InnerWidth: {}{}", table.ColumnsGivenWidth, table.ColumnsAutoFitWidth, table.InnerWidth, table.InnerWidth == 0.0 ? " (auto)" : "");
//     BulletText("CellPaddingX: {}, CellSpacingX: {}/{}, OuterPaddingX: {}", table.CellPaddingX, table.CellSpacingX1, table.CellSpacingX2, table.OuterPaddingX);
//     BulletText("HoveredColumnBody: {}, HoveredColumnBorder: {}", table.HoveredColumnBody, table.HoveredColumnBorder);
//     BulletText("ResizedColumn: {}, ReorderColumn: {}, HeldHeaderColumn: {}", table.ResizedColumn, table.ReorderColumn, table.HeldHeaderColumn);
//     //BulletText("BgDrawChannels: {}/{}", 0, table.BgDrawChannelUnfrozen);
//     let sum_weights: c_float =  0.0;
//     for (let n: c_int = 0; n < table.ColumnsCount; n++)
//         if (table.Columns[n].Flags & ImGuiTableColumnFlags_WidthStretch)
//             sum_weights += table.Columns[n].StretchWeight;
//     for (let n: c_int = 0; n < table.ColumnsCount; n++)
//     {
//         column: *mut ImGuiTableColumn = &table.Columns[n];
//         let mut  name: *const c_char = TableGetColumnName(table, n);
//         ImFormatString(buf, buf.len(),
//             "Column {} order {} '{}': offset %+.2f to %+.2f{}\n"
//             "Enabled: {}, VisibleX/Y: {}/{}, RequestOutput: {}, SkipItems: {}, DrawChannels: {},{}\n"
//             "WidthGiven: {}, Request/Auto: {}/{}, StretchWeight: {} ({}%%)\n"
//             "MinX: {}, MaxX: {} (%+.10.0), ClipRect: {} to {} (+{})\n"
//             "ContentWidth: {},{}, HeadersUsed/Ideal {}/{}\n"
//             "Sort: {}{}, UserID: 0x{}, Flags: {}: {}{}{}..",
//             n, column.DisplayOrder, name, column.MinX - table.WorkRect.Min.x, column.MaxX - table.WorkRect.Min.x, (n < table.FreezeColumnsRequest) ? " (Frozen)" : "",
//             column.IsEnabled, column.IsVisibleX, column.IsVisibleY, column.IsRequestOutput, column.IsSkipItems, column.DrawChannelFrozen, column.DrawChannelUnfrozen,
//             column.WidthGiven, column.WidthRequest, column.WidthAuto, column.StretchWeight, if column.StretchWeight > 0.0 { (column.StretchWeight / sum_weights) * 100 } else { 0.0 },
//             column.MinX, column.MaxX, column.MaxX - column.MinX, column.ClipRect.Min.x, column.ClipRect.Max.x, column.ClipRect.Max.x - column.ClipRect.Min.x,
//             column.ContentMaxXFrozen - column.WorkMinX, column.ContentMaxXUnfrozen - column.WorkMinX, column.ContentMaxXHeadersUsed - column.WorkMinX, column.ContentMaxXHeadersIdeal - column.WorkMinX,
//             column.SortOrder, if (column.SortDirection == ImGuiSortDirection_Ascending) { " (Asc)" } else {
//                if (column.SortDirection == ImGuiSortDirection_Descending) {
//                    " (Des)"
//                } else { "" }
//             }, column.UserID, column.Flags,
//             flag_set(column.Flags, ImGuiTableColumnFlags_WidthStretch) ? "WidthStretch " : "",
//             flag_set(column.Flags, ImGuiTableColumnFlags_WidthFixed) ? "WidthFixed " : "",
//             flag_set(column.Flags, ImGuiTableColumnFlags_NoResize) ? "NoResize " : "");
//         Bullet();
//         Selectable(buf);
//         if (IsItemHovered())
//         {
//             let mut r: ImRect = ImRect::new(column.MinX, table.OuterRect.Min.y, column.MaxX, table.OuterRect.Max.y);
//             GetForegroundDrawList().AddRect(r.Min, r.Max, IM_COL32(255, 255, 0, 255));
//         }
//     }
//     if (settings: *mut ImGuiTableSettings = TableGetBoundSettings(table))
//         DebugNodeTableSettings(settings);
//     if clear_settings{
//         table.IsResetAllRequest = true;}
//     TreePop();
// }
//
// pub unsafe fn DebugNodeTableSettings(settings: *mut ImGuiTableSettings)
// {
//     if !TreeNode(settings.ID, "Settings 0x{} ({} columns)", settings.ID, settings.ColumnsCount) { return ; }
//     BulletText("SaveFlags: 0x{}", settings.SaveFlags);
//     BulletText("ColumnsCount: {} (max {})", settings.ColumnsCount, settings.ColumnsCountMax);
//     for (let n: c_int = 0; n < settings.ColumnsCount; n++)
//     {
//         *mut ImGuiTableColumnSettings column_settings = &settings.GetColumnSettings()[n];
//         sort_dir: ImGuiSortDirection = if column_settings.SortOrder != -1 { (ImGuiSortDirection)column_settings.SortDirection} else { ImGuiSortDirection_None};
//         BulletText("Column {} Order {} SortOrder {} {} Vis {} {} %7.3f UserID 0x{}",
//             n, column_settings.DisplayOrder, column_settings.SortOrder,
//             (sort_dir == ImGuiSortDirection_Ascending) ? "Asc" : (sort_dir == ImGuiSortDirection_Descending) ? "Des" : "---",
//             column_settings.IsEnabled, column_settings.IsStretch ? "Weight" : "Width ", column_settings.WidthOrWeight, column_settings.UserID);
//     }
//     TreePop();
    todo!()
}

// #else // #ifndef IMGUI_DISABLE_DEBUG_TOOLS

// pub unsafe fn DebugNodeTable(*mut ImGuiTable) {}
// pub unsafe fn DebugNodeTableSettings(*mut ImGuiTableSettings) {}

// #endif


//-------------------------------------------------------------------------
// [SECTION] Columns, BeginColumns, EndColumns, etc.
// (This is a legacy API, prefer using BeginTable/EndTable!)
//-------------------------------------------------------------------------
// FIXME: sizing is lossy when columns width is very small (default width may turn negative etc.)
//-------------------------------------------------------------------------
// - SetWindowClipRectBeforeSetChannel() [Internal]
// - GetColumnIndex()
// - GetColumnsCount()
// - GetColumnOffset()
// - GetColumnWidth()
// - SetColumnOffset()
// - SetColumnWidth()
// - PushColumnClipRect() [Internal]
// - PushColumnsBackground() [Internal]
// - PopColumnsBackground() [Internal]
// - FindOrCreateColumns() [Internal]
// - GetColumnsID() [Internal]
// - BeginColumns()
// - NextColumn()
// - EndColumns()
// - Columns()
//-------------------------------------------------------------------------



pub unsafe fn GetColumnIndex() -> c_int
{
    let window: &mut ImguiWindow = GetCurrentWindowRead();
    return if window.dc.current_columns { window.dc.current_columns.Current }else {0};
}

pub unsafe fn GetColumnsCount() -> c_int
{
    let window: &mut ImguiWindow = GetCurrentWindowRead();
    return if window.dc.current_columns { window.dc.current_columns.Count } else {1};
}

pub unsafe fn GetColumnOffsetFromNorm(columns: *const ImGuiOldColumns,offset_norm: c_float) -> f32
{
    return offset_norm * (columns.OffMaxX - columns.OffMinX);
}

pub unsafe fn GetColumnNormFromOffset(columns: *const ImGuiOldColumns,offset: c_float) -> f32
{
    return offset / (columns.OffMaxX - columns.OffMinX);
}

pub const COLUMNS_HIT_RECT_HALF_WIDTH: c_float =  4.0;

pub unsafe fn staticGetDraggedColumnOffset(columns: *mut ImGuiOldColumns, column_index: c_int) -> f32
{
    // Active (dragged) column always follow mouse. The reason we need this is that dragging a column to the right edge of an auto-resizing
    // window creates a feedback loop because we store normalized positions. So while dragging we enforce absolute positioning.
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let window: &mut ImguiWindow = g.CurrentWindow;
    // IM_ASSERT(column_index > 0); // We are not supposed to drag column 0.
    // IM_ASSERT(g.ActiveId == columns.ID + ImguiHandle(column_index));

    let mut x: c_float =  g.IO.MousePos.x - g.ActiveIdClickOffset.x + COLUMNS_HIT_RECT_HALF_WIDTH - window.position.x;
    x = ImMax(x, GetColumnOffset(column_index - 1) + g.style.ColumnsMinSpacing);
    if ((columns.Flags & ImGuiOldColumnFlags_NoPreserveWidths)) {
        x = ImMin(x as c_int, (GetColumnOffset(column_index + 1) - g.style.ColumnsMinSpacing) as c_int);
    }

    return x;
}

pub unsafe fn GetColumnOffset(mut column_index: c_int) -> f32 {
    let window: &mut ImguiWindow = GetCurrentWindowRead();
    let columns: *mut ImGuiOldColumns = window.dc.current_columns;
    if columns == None {
        return 0.0;
    }

    if (column_index < 0) {
        column_index = columns.Current;
    }
    // IM_ASSERT(column_index < columns.Columns.Size);

    let t: c_float = columns.Columns[column_index].OffsetNorm;
    let x_offset: c_float = ImLerp(columns.OffMinX, columns.OffMaxX, t);
    return x_offset;
}

pub unsafe fn GetColumnWidthEx(columns: *mut ImGuiOldColumns, mut column_index: c_int, before_resize: bool) -> f32
{
    if (column_index < 0) {
        column_index = columns.Current;
    }

    let mut offset_norm: c_float = 0.0;
    if (before_resize) {
        offset_norm = columns.Columns[column_index + 1].OffsetNormBeforeResize - columns.Columns[column_index].OffsetNormBeforeResize;
    }
    else {
        offset_norm = columns.Columns[column_index + 1].OffsetNorm - columns.Columns[column_index].OffsetNorm;
    }
    return GetColumnOffsetFromNorm(columns, offset_norm);
}

pub unsafe fn GetColumnWidth(mut column_index: c_int) -> f32
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window  = g.current_window_mut().unwrap();
    columns: *mut ImGuiOldColumns = window.dc.CurrentColumns;
    if columns == None{
        return content_region_avail(g).x;}

    if (column_index < 0) {
        column_index = columns.Current;
    }
    return GetColumnOffsetFromNorm(columns, columns.Columns[column_index + 1].OffsetNorm - columns.Columns[column_index].OffsetNorm);
}

pub unsafe fn SetColumnOffset(mut column_index: c_int,mut offset: c_float)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let window: &mut ImguiWindow = g.CurrentWindow;
    let columns: *mut ImGuiOldColumns = window.dc.current_columns;
    // IM_ASSERT(columns != NULL);

    if (column_index < 0) {
        column_index = columns.Current;
    }
    // IM_ASSERT(column_index < columns.Columns.Size);

    let preserve_width: bool = flag_clear(columns.Flags, ImGuiOldColumnFlags_NoPreserveWidths) && (column_index < columns.Count - 1);
    let width: c_float =  if preserve_width { GetColumnWidthEx(columns, column_index, columns.IsBeingResized) } else { 0.0 };

    if (flag_clear(columns.Flags, ImGuiOldColumnFlags_NoForceWithinWindow)) {
        offset = offset.min( columns.OffMaxX - g.style.ColumnsMinSpacing * (columns.Count - column_index));
    }
    columns.Columns[column_index].OffsetNorm = GetColumnNormFromOffset(columns, offset - columns.OffMinX);

    if (preserve_width) {
        SetColumnOffset(column_index + 1, offset + ImMax(g.style.ColumnsMinSpacing, width));
    }
}

pub unsafe fn SetColumnWidth(mut column_index: c_int,width: c_float)
{
    let window: &mut ImguiWindow = GetCurrentWindowRead();
    let columns: *mut ImGuiOldColumns = window.dc.current_columns;
    // IM_ASSERT(columns != NULL);

    if (column_index < 0) {
        column_index = columns.Current;
    }
    SetColumnOffset(column_index + 1, GetColumnOffset(column_index) + width);
}

pub unsafe fn PushColumnClipRect(mut column_index: c_int)
{
    let window: &mut ImguiWindow = GetCurrentWindowRead();
    let columns: *mut ImGuiOldColumns = window.dc.current_columns;
    if (column_index < 0) {
        column_index = columns.Current;
    }

    let column: *mut ImGuiOldColumnData = &mut columns.Columns[column_index];
    PushClipRect(g, &column.ClipRect.min, &column.ClipRect.max, false);
}

// Get into the columns background draw command (which is generally the same draw command as before we called BeginColumns)
pub unsafe fn PushColumnsBackground()
{
    let window: &mut ImguiWindow = GetCurrentWindowRead();
    let columns: *mut ImGuiOldColumns = window.dc.current_columns;
    if columns.Count == 1 { return ; }

    // Optimization: avoid SetCurrentChannel() + PushClipRect()
    columns.HostBackupClipRect = window.ClipRect.into();
    SetWindowClipRectBeforeSetChannel(window, &columns.HostInitialClipRect);
    columns.Splitter.SetCurrentChannel(&mut *window.DrawList, 0);
}

pub unsafe fn PopColumnsBackground()
{
    let window: &mut ImguiWindow = GetCurrentWindowRead();
    let columns: *mut ImGuiOldColumns = window.dc.current_columns;
    if columns.Count == 1 { return ; }

    // Optimization: avoid PopClipRect() + SetCurrentChannel()
    SetWindowClipRectBeforeSetChannel(window, &columns.HostBackupClipRect);
    columns.Splitter.SetCurrentChannel(&mut *window.DrawList, columns.Current + 1);
}

pub unsafe fn FindOrCreateColumns(window: &mut ImguiWindow, id: ImguiHandle) -> *mut ImGuiOldColumns
{
    // We have few columns per window so for now we don't need bother much with turning this into a faster lookup.
    // for (let n: c_int = 0; n < window.ColumnsStorage.Size; n++)
    for n in 0 .. window.ColumnsStorage
    {
        if window.ColumnsStorage[n].ID == id {
            return &mut window.ColumnsStorage[n];
        }
    }

    window.ColumnsStorage.push(ImGuiOldColumns::default());
    let columns: *mut ImGuiOldColumns = window.ColumnsStorage.last_mut().unwrap();
    columns.ID = id;
    return columns;
}

pub unsafe fn GetColumnsID(str_id: *const c_char, columns_count: c_int) -> ImguiHandle
{
    let mut window = g.current_window_mut().unwrap();

    // Differentiate column ID with an arbitrary prefix for cases where users name their columns set the same as another widget.
    // In addition, when an identifier isn't explicitly provided we include the number of columns in the hash to make it uniquer.
    push_int_id(g, 0x11223347 + (if str_id {0} else { columns_count }));
    let mut id: ImguiHandle =  window.id_from_str(if str_id { str_id }else {"columns"}, );
    pop_win_id_from_stack(g);

    return id;
}

pub unsafe fn BeginColumns(str_id: *const c_char, columns_count: c_int, flags: ImGuiOldColumnFlags)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let window: &mut ImguiWindow = GetCurrentWindow();

    // IM_ASSERT(columns_count >= 1);
    // IM_ASSERT(window.dc.CurrentColumns == NULL);   // Nested columns are currently not supported

    // Acquire storage for the columns set
    let mut id: ImguiHandle =  GetColumnsID(str_id, columns_count);
    let columns: *mut ImGuiOldColumns = FindOrCreateColumns(window, id);
    // IM_ASSERT(columns.ID == id);
    columns.Current = 0;
    columns.Count = columns_count;
    columns.Flags = flags;
    window.dc.current_columns = columns;

    columns.HostCursorPosY = window.dc.cursor_pos.y;
    columns.HostCursorMaxPosX = window.dc.CursorMaxPos.x;
    columns.HostInitialClipRect = window.ClipRect.into();
    columns.HostBackupParentWorkRect = window.ParentWorkRect;
    window.ParentWorkRect = window.work_rect;

    // Set state for first column
    // We aim so that the right-most column will have the same clipping width as other after being clipped by parent ClipRect
    let column_padding: c_float =  g.style.ItemSpacing.x;
    let half_clip_extend_x: c_float =  ImFloor(ImMax(window.WindowPadding.x * 0.5, window.WindowBorderSize));
    let max_1: c_float =  window.work_rect.max.x + column_padding - ImMax(column_padding - window.WindowPadding.x, 0.0);
    let max_2: c_float =  window.work_rect.max.x + half_clip_extend_x;
    columns.OffMinX = window.dc.indent.x - column_padding + ImMax(column_padding - window.WindowPadding.x, 0.0);
    columns.OffMaxX = ImMax(ImMin(max_1 as c_int, max_2 as c_int) - window.position.x, columns.OffMinX + 1.0);
    columns.LineMinY = window.dc.cursor_pos.y;columns.LineMaxY = window.dc.cursor_pos.y;

    // Clear data if columns count changed
    if (columns.Columns.Size != 0 && columns.Columns.Size != columns_count + 1) {
        columns.Columns.clear();
    }

    // initialize default widths
    columns.IsFirstFrame = (columns.Columns.Size == 0);
    if (columns.Columns.Size == 0)
    {
        columns.Columns.reserve((columns_count + 1) as usize);
        // for (let n: c_int = 0; n < columns_count + 1; n++)
        for n in 0 .. columns_count
        {
            let mut column = ImGuiOldColumnData::default();
            column.OffsetNorm = n / columns_count;
            columns.Columns.push(column);
        }
    }

    // for (let n: c_int = 0; n < columns_count; n++)
    for n in 0 .. columns_count
    {
        // Compute clipping rectangle
        let column: *mut ImGuiOldColumnData = &mut columns.Columns[n];
        let clip_x1: c_float =  IM_ROUND(window.position.x + GetColumnOffset(n));
        let clip_x2: c_float =  IM_ROUND(window.position.x + GetColumnOffset(n + 1) - 1.0);
        column.ClipRect = ImRect(clip_x1, -f32::MAX, clip_x2, f32::MAX);
        column.ClipRect.ClipWithFull(window.ClipRect.into());
    }

    if columns.Count > 1
    {
        columns.Splitter.Split(window.DrawList, (1 + columns.Count) as size_t);
        columns.Splitter.SetCurrentChannel(&mut *window.DrawList, 1);
        PushColumnClipRect(0);
    }

    // We don't generally store Indent.x inside ColumnsOffset because it may be manipulated by the user.
    let offset_0: c_float =  GetColumnOffset(columns.Current);
    let offset_1: c_float =  GetColumnOffset(columns.Current + 1);
    let width: c_float =  offset_1 - offset_0;
    PushItemWidth(width * 0.650f32);
    window.dc.columns_offset.x = ImMax(column_padding - window.WindowPadding.x, 0.0);
    window.dc.cursor_pos.x = IM_FLOOR(window.position.x + window.dc.indent.x + window.dc.columns_offset.x);
    window.work_rect.max.x = window.position.x + offset_1 - column_padding;
}

pub unsafe fn NextColumn()
{
    let window: &mut ImguiWindow = GetCurrentWindow();
    if window.skip_items || window.dc.current_columns == None { return ; }

    let g = GImGui; // ImGuiContext& g = *GImGui;
    let columns: *mut ImGuiOldColumns = window.dc.current_columns;

    if columns.Count == 1
    {
        window.dc.cursor_pos.x = IM_FLOOR(window.position.x + window.dc.indent.x + window.dc.columns_offset.x);
        // IM_ASSERT(columns.Current == 0);
        return;
    }

    // Next column
    columns.Current += 1;
    if columns.Current == columns.Count {
        columns.Current = 0;
    }

    PopItemWidth();

    // Optimization: avoid PopClipRect() + SetCurrentChannel() + PushClipRect()
    // (which would needlessly attempt to update commands in the wrong channel, then pop or overwrite them),
    let column: *mut ImGuiOldColumnData = &mut columns.Columns[columns.Current];
    SetWindowClipRectBeforeSetChannel(window, &column.ClipRect);
    columns.Splitter.SetCurrentChannel(&mut *window.DrawList, columns.Current + 1);

    let column_padding: c_float =  g.style.ItemSpacing.x;
    columns.LineMaxY = ImMax(columns.LineMaxY, window.dc.cursor_pos.y);
    if columns.Current > 0
    {
        // Columns 1+ ignore IndentX (by canceling it out)
        // FIXME-COLUMNS: Unnecessary, could be locked?
        window.dc.columns_offset.x = GetColumnOffset(columns.Current) - window.dc.indent.x + column_padding;
    }
    else
    {
        // New row/line: column 0 honor IndentX.
        window.dc.columns_offset.x = ImMax(column_padding - window.WindowPadding.x, 0.0);
        window.dc.is_same_line = false;
        columns.LineMinY = columns.LineMaxY;
    }
    window.dc.cursor_pos.x = IM_FLOOR(window.position.x + window.dc.indent.x + window.dc.columns_offset.x);
    window.dc.cursor_pos.y = columns.LineMinY;
    window.dc.curr_line_size = Vector2::from_floats(0.0, 0.0);
    window.dc.curr_line_text_base_offset = 0.0;

    // FIXME-COLUMNS: Share code with BeginColumns() - move code on columns setup.
    let offset_0: c_float =  GetColumnOffset(columns.Current);
    let offset_1: c_float =  GetColumnOffset(columns.Current + 1);
    let width: c_float =  offset_1 - offset_0;
    PushItemWidth(width * 0.650f32);
    window.work_rect.max.x = window.position.x + offset_1 - column_padding;
}

pub unsafe fn EndColumns()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let window: &mut ImguiWindow = GetCurrentWindow();
    let columns: *mut ImGuiOldColumns = window.dc.current_columns;
    // IM_ASSERT(columns != NULL);

    PopItemWidth();
    if (columns.Count > 1)
    {
        PopClipRect(g);
        columns.Splitter.Merge(window.DrawList);
    }

    let flags: ImGuiOldColumnFlags = columns.Flags;
    columns.LineMaxY = ImMax(columns.LineMaxY, window.dc.cursor_pos.y);
    window.dc.cursor_pos.y = columns.LineMaxY;
    if (flag_clear(flags, ImGuiOldColumnFlags_GrowParentContentsSize)) {
        window.dc.CursorMaxPos.x = columns.HostCursorMaxPosX;
    }  // Restore cursor max pos, as columns don't grow parent

    // Draw columns borders and handle resize
    // The IsBeingResized flag ensure we preserve pre-resize columns width so back-and-forth are not lossy
    let mut is_being_resized: bool =  false;
    if (flag_clear(flags, ImGuiOldColumnFlags_NoBorder) && !window.skip_items)
    {
        // We clip Y boundaries CPU side because very long triangles are mishandled by some GPU drivers.
        let y1: c_float =  ImMax(columns.HostCursorPosY, window.ClipRect.Min.y);
        let y2: c_float =  window.dc.cursor_pos.y.min( window.ClipRect.Max.y);
        let mut dragging_column: c_int = -1;
        // for (let n: c_int = 1; n < columns.Count; n++)
        for n in 1 .. columns.Count
        {
            let column: *mut ImGuiOldColumnData = &mut columns.Columns[n];
            let x: c_float =  window.position.x + GetColumnOffset(n);
            let mut column_id: ImguiHandle =  columns.ID + ImguiHandle(n);
            let column_hit_hw: c_float =  COLUMNS_HIT_RECT_HALF_WIDTH;
            let mut column_hit_rect: ImRect = ImRect::new(Vector2::from_floats(x - column_hit_hw, y1), Vector2::from_floats(x + column_hit_hw, y2));
            KeepAliveID(g, column_id);
            if (IsClippedEx(&mut column_hit_rect, column_id)) { // FIXME: Can be removed or replaced with a lower-level test
                continue;
            }

            let mut hovered: bool =  false;
            let mut held = false;
            if flag_clear(flags, ImGuiOldColumnFlags_NoResize)
            {
                ButtonBehavior(column_hit_rect, column_id, &hovered, &held);
                if hovered || held{
                    g.MouseCursor = ImGuiMouseCursor_ResizeEW;}
                if held && flag_clear(column.Flags, ImGuiOldColumnFlags_NoResize) {
                    dragging_column = n;}
            }

            // Draw column
            col: u32 = GetColorU32(if held { ImGuiCol_SeparatorActive } else {
                if hovered {
                    ImGuiCol_SeparatorHovered
                } else { ImGuiCol_Separator }
            }, 0.0);
            let xi: c_float =  IM_FLOOR(x);
            window.DrawList.AddLine(&Vector2::from_floats(xi, y1 + 1.0), &Vector2::from_floats(xi, y2), col, 0.0);
        }

        // Apply dragging after drawing the column lines, so our rendered lines are in sync with how items were displayed during the frame.
        if dragging_column != -1
        {
            if !columns.IsBeingResized {
                // for (let n: c_int = 0; n < columns.Count + 1; n++)
                for n in 0..columns.Count {
                    columns.Columns[n].OffsetNormBeforeResize = columns.Columns[n].OffsetNorm;
                }
            }
            columns.IsBeingResized = true;
            is_being_resized = true;
            let x: c_float =  GetDraggedColumnOffset(columns, dragging_column);
            SetColumnOffset(dragging_column, x);
        }
    }
    columns.IsBeingResized = is_being_resized;

    window.work_rect = window.ParentWorkRect;
    window.ParentWorkRect = columns.HostBackupParentWorkRect;
    window.dc.current_columns = None;
    window.dc.columns_offset.x = 0.0;
    window.dc.cursor_pos.x = IM_FLOOR(window.position.x + window.dc.indent.x + window.dc.columns_offset.x);
}

pub unsafe fn Columns(columns_count: c_int, id: *const c_char, border: bool)
{
    let mut window = g.current_window_mut().unwrap();
    // IM_ASSERT(columns_count >= 1);

    flags: ImGuiOldColumnFlags = (if border {0} else { ImGuiOldColumnFlags_NoBorder });
    //flags |= ImGuiOldColumnFlags_NoPreserveWidths; // NB: Legacy behavior
    columns: *mut ImGuiOldColumns = window.dc.CurrentColumns;
    if columns != None && columns.Count == columns_count && columns.Flags == flags { return ; }

    if (columns != null_mut()) {
        EndColumns();
    }

    if (columns_count != 1) {
        BeginColumns(id, columns_count, flags);
    }
}

//-------------------------------------------------------------------------

// #endif // #ifndef IMGUI_DISABLE
