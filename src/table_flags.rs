#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiTableFlags;        // -> enum ImGuiTableFlags_      // Flags: For BeginTable()
pub type ImGuiTableFlags = c_int;

// Flags for ImGui::BeginTable()
// - Important! Sizing policies have complex and subtle side effects, much more so than you would expect.
//   Read comments/demos carefully + experiment with live demos to get acquainted with them.
// - The DEFAULT sizing policies are:
//    - Default to ImGuiTableFlags_SizingFixedFit    if ScrollX is on, or if host window has ImGuiWindowFlags_AlwaysAutoResize.
//    - Default to ImGuiTableFlags_SizingStretchSame if ScrollX is off.
// - When ScrollX is off:
//    - Table defaults to ImGuiTableFlags_SizingStretchSame -> all Columns defaults to ImGuiTableColumnFlags_WidthStretch with same weight.
//    - Columns sizing policy allowed: Stretch (default), Fixed/Auto.
//    - Fixed Columns (if any) will generally obtain their requested width (unless the table cannot fit them all).
//    - Stretch Columns will share the remaining width according to their respective weight.
//    - Mixed Fixed/Stretch columns is possible but has various side-effects on resizing behaviors.
//      The typical use of mixing sizing policies is: any number of LEADING Fixed columns, followed by one or two TRAILING Stretch columns.
//      (this is because the visible order of columns have subtle but necessary effects on how they react to manual resizing).
// - When ScrollX is on:
//    - Table defaults to ImGuiTableFlags_SizingFixedFit -> all Columns defaults to ImGuiTableColumnFlags_WidthFixed
//    - Columns sizing policy allowed: Fixed/Auto mostly.
//    - Fixed Columns can be enlarged as needed. Table will show an horizontal scrollbar if needed.
//    - When using auto-resizing (non-resizable) fixed columns, querying the content width to use item right-alignment e.g. SetNextItemWidth(-FLT_MIN) doesn't make sense, would create a feedback loop.
//    - Using Stretch columns OFTEN DOES NOT MAKE SENSE if ScrollX is on, UNLESS you have specified a value for 'inner_width' in BeginTable().
//      If you specify a value for 'inner_width' then effectively the scrolling space is known and Stretch or mixed Fixed/Stretch columns become meaningful again.
// - Read on documentation at the top of imgui_tables.cpp for details.
// enum ImGuiTableFlags_
// {
// Features
pub const ImGuiTableFlags_None: ImGuiTableFlags = 0;
pub const ImGuiTableFlags_Resizable: ImGuiTableFlags = 1 << 0;
// Enable resizing columns.
pub const ImGuiTableFlags_Reorderable: ImGuiTableFlags = 1 << 1;
// Enable reordering columns in header row (need calling TableSetupColumn() + TableHeadersRow() to display headers)
pub const ImGuiTableFlags_Hideable: ImGuiTableFlags = 1 << 2;
// Enable hiding/disabling columns in context menu.
pub const ImGuiTableFlags_Sortable: ImGuiTableFlags = 1 << 3;
// Enable sorting. Call TableGetSortSpecs() to obtain sort specs. Also see ImGuiTableFlags_SortMulti and ImGuiTableFlags_SortTristate.
pub const ImGuiTableFlags_NoSavedSettings: ImGuiTableFlags = 1 << 4;
// Disable persisting columns order, width and sort settings in the .ini file.
pub const ImGuiTableFlags_ContextMenuInBody: ImGuiTableFlags = 1 << 5;
// Right-click on columns body/contents will display table context menu. By default it is available in TableHeadersRow().
// Decorations
pub const ImGuiTableFlags_RowBg: ImGuiTableFlags = 1 << 6;
// Set each RowBg color with ImGuiCol_TableRowBg or ImGuiCol_TableRowBgAlt (equivalent of calling TableSetBgColor with ImGuiTableBgFlags_RowBg0 on each row manually)
pub const ImGuiTableFlags_BordersInnerH: ImGuiTableFlags = 1 << 7;
// Draw horizontal borders between rows.
pub const ImGuiTableFlags_BordersOuterH: ImGuiTableFlags = 1 << 8;
// Draw horizontal borders at the top and bottom.
pub const ImGuiTableFlags_BordersInnerV: ImGuiTableFlags = 1 << 9;
// Draw vertical borders between columns.
pub const ImGuiTableFlags_BordersOuterV: ImGuiTableFlags = 1 << 10;
// Draw vertical borders on the left and right sides.
pub const ImGuiTableFlags_BordersH: ImGuiTableFlags = ImGuiTableFlags_BordersInnerH | ImGuiTableFlags_BordersOuterH;
// Draw horizontal borders.
pub const ImGuiTableFlags_BordersV: ImGuiTableFlags = ImGuiTableFlags_BordersInnerV | ImGuiTableFlags_BordersOuterV;
// Draw vertical borders.
pub const ImGuiTableFlags_BordersInner: ImGuiTableFlags = ImGuiTableFlags_BordersInnerV | ImGuiTableFlags_BordersInnerH;
// Draw inner borders.
pub const ImGuiTableFlags_BordersOuter: ImGuiTableFlags = ImGuiTableFlags_BordersOuterV | ImGuiTableFlags_BordersOuterH;
// Draw outer borders.
pub const ImGuiTableFlags_Borders: ImGuiTableFlags = ImGuiTableFlags_BordersInner | ImGuiTableFlags_BordersOuter;
// Draw all borders.
pub const ImGuiTableFlags_NoBordersInBody: ImGuiTableFlags = 1 << 11;
// [ALPHA] Disable vertical borders in columns Body (borders will always appears in Headers). -> May move to style
pub const ImGuiTableFlags_NoBordersInBodyUntilResize: ImGuiTableFlags = 1 << 12;
// [ALPHA] Disable vertical borders in columns Body until hovered for resize (borders will always appears in Headers). -> May move to style
// Sizing Policy (read above for defaults)
pub const ImGuiTableFlags_SizingFixedFit: ImGuiTableFlags = 1 << 13;
// Columns default to _WidthFixed or _WidthAuto (if resizable or not resizable), matching contents width.
pub const ImGuiTableFlags_SizingFixedSame: ImGuiTableFlags = 2 << 13;
// Columns default to _WidthFixed or _WidthAuto (if resizable or not resizable), matching the maximum contents width of all columns. Implicitly enable ImGuiTableFlags_NoKeepColumnsVisible.
pub const ImGuiTableFlags_SizingStretchProp: ImGuiTableFlags = 3 << 13;
// Columns default to _WidthStretch with default weights proportional to each columns contents widths.
pub const ImGuiTableFlags_SizingStretchSame: ImGuiTableFlags = 4 << 13;
// Columns default to _WidthStretch with default weights all equal, unless overridden by TableSetupColumn().
// Sizing Extra Options
pub const ImGuiTableFlags_NoHostExtendX: ImGuiTableFlags = 1 << 16;
// Make outer width auto-fit to columns, overriding outer_size.x value. Only available when ScrollX/ScrollY are disabled and Stretch columns are not used.
pub const ImGuiTableFlags_NoHostExtendY: ImGuiTableFlags = 1 << 17;
// Make outer height stop exactly at outer_size.y (prevent auto-extending table past the limit). Only available when ScrollX/ScrollY are disabled. Data below the limit will be clipped and not visible.
pub const ImGuiTableFlags_NoKeepColumnsVisible: ImGuiTableFlags = 1 << 18;
// Disable keeping column always minimally visible when ScrollX is off and table gets too small. Not recommended if columns are resizable.
pub const ImGuiTableFlags_PreciseWidths: ImGuiTableFlags = 1 << 19;
// Disable distributing remainder width to stretched columns (width allocation on a 100-wide table with 3 columns: Without this flag: 33,33,34. With this flag: 33,33,33). With larger number of columns, resizing will appear to be less smooth.
// Clipping
pub const ImGuiTableFlags_NoClip: ImGuiTableFlags = 1 << 20;
// Disable clipping rectangle for every individual columns (reduce draw command count, items will be able to overflow into other columns). Generally incompatible with TableSetupScrollFreeze().
// Padding
pub const ImGuiTableFlags_PadOuterX: ImGuiTableFlags = 1 << 21;
// Default if BordersOuterV is on. Enable outer-most padding. Generally desirable if you have headers.
pub const ImGuiTableFlags_NoPadOuterX: ImGuiTableFlags = 1 << 22;
// Default if BordersOuterV is off. Disable outer-most padding.
pub const ImGuiTableFlags_NoPadInnerX: ImGuiTableFlags = 1 << 23;
// Disable inner padding between columns (double inner padding if BordersOuterV is on, single inner padding if BordersOuterV is of0f32).
// Scrolling
pub const ImGuiTableFlags_ScrollX: ImGuiTableFlags = 1 << 24;
// Enable horizontal scrolling. Require 'outer_size' parameter of BeginTable() to specify the container size. Changes default sizing policy. Because this create a child window, ScrollY is currently generally recommended when using ScrollX.
pub const ImGuiTableFlags_ScrollY: ImGuiTableFlags = 1 << 25;
// Enable vertical scrolling. Require 'outer_size' parameter of BeginTable() to specify the container size.
// Sorting
pub const ImGuiTableFlags_SortMulti: ImGuiTableFlags = 1 << 26;
// Hold shift when clicking headers to sort on multiple column. TableGetSortSpecs() may return specs where (SpecsCount > 1).
pub const ImGuiTableFlags_SortTristate: ImGuiTableFlags = 1 << 27;  // Allow no sorting, disable default sorting. TableGetSortSpecs() may return specs where (SpecsCount == 0).

// [Internal] Combinations and masks
pub const ImGuiTableFlags_SizingMask_: ImGuiTableFlags = ImGuiTableFlags_SizingFixedFit | ImGuiTableFlags_SizingFixedSame | ImGuiTableFlags_SizingStretchProp | ImGuiTableFlags_SizingStretchSame;

// Obsolete names (will be removed soon)
// #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
//, ImGuiTableFlags_ColumnsWidthFixed = ImGuiTableFlags_SizingFixedFit, ImGuiTableFlags_ColumnsWidthStretch = ImGuiTableFlags_SizingStretchSame   // WIP Tables 2020/12
//, ImGuiTableFlags_SizingPolicyFixed = ImGuiTableFlags_SizingFixedFit, ImGuiTableFlags_SizingPolicyStretch = ImGuiTableFlags_SizingStretchSame   // WIP Tables 2021/01
// #endif
// };
