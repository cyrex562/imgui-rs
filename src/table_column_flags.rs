#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiTableColumnFlags;  // -> enum ImGuiTableColumnFlags_// Flags: For TableSetupColumn()
pub type ImGuiTableColumnFlags = c_int;


// Flags for TableSetupColumn()
// enum ImGuiTableColumnFlags_
// {
// Input configuration flags
pub const ImGuiTableColumnFlags_None: ImGuiTableColumnFlags = 0;
pub const ImGuiTableColumnFlags_Disabled: ImGuiTableColumnFlags = 1 << 0;
// Overriding/master disable flag: hide column, won't show in context menu (unlike calling TableSetColumnEnabled() which manipulates the user accessible state)
pub const ImGuiTableColumnFlags_DefaultHide: ImGuiTableColumnFlags = 1 << 1;
// Default as a hidden/disabled column.
pub const ImGuiTableColumnFlags_DefaultSort: ImGuiTableColumnFlags = 1 << 2;
// Default as a sorting column.
pub const ImGuiTableColumnFlags_WidthStretch: ImGuiTableColumnFlags = 1 << 3;
// Column will stretch. Preferable with horizontal scrolling disabled (default if table sizing policy is _SizingStretchSame or _SizingStretchProp).
pub const ImGuiTableColumnFlags_WidthFixed: ImGuiTableColumnFlags = 1 << 4;
// Column will not stretch. Preferable with horizontal scrolling enabled (default if table sizing policy is _SizingFixedFit and table is resizable).
pub const ImGuiTableColumnFlags_NoResize: ImGuiTableColumnFlags = 1 << 5;
// Disable manual resizing.
pub const ImGuiTableColumnFlags_NoReorder: ImGuiTableColumnFlags = 1 << 6;
// Disable manual reordering this column, this will also prevent other columns from crossing over this column.
pub const ImGuiTableColumnFlags_NoHide: ImGuiTableColumnFlags = 1 << 7;
// Disable ability to hide/disable this column.
pub const ImGuiTableColumnFlags_NoClip: ImGuiTableColumnFlags = 1 << 8;
// Disable clipping for this column (all NoClip columns will render in a same draw command).
pub const ImGuiTableColumnFlags_NoSort: ImGuiTableColumnFlags = 1 << 9;
// Disable ability to sort on this field (even if ImGuiTableFlags_Sortable is set on the table).
pub const ImGuiTableColumnFlags_NoSortAscending: ImGuiTableColumnFlags = 1 << 10;
// Disable ability to sort in the ascending direction.
pub const ImGuiTableColumnFlags_NoSortDescending: ImGuiTableColumnFlags = 1 << 11;
// Disable ability to sort in the descending direction.
pub const ImGuiTableColumnFlags_NoHeaderLabel: ImGuiTableColumnFlags = 1 << 12;
// TableHeadersRow() will not submit label for this column. Convenient for some small columns. Name will still appear in context menu.
pub const ImGuiTableColumnFlags_NoHeaderWidth: ImGuiTableColumnFlags = 1 << 13;
// Disable header text width contribution to automatic column width.
pub const ImGuiTableColumnFlags_PreferSortAscending: ImGuiTableColumnFlags = 1 << 14;
// Make the initial sort direction Ascending when first sorting on this column (default).
pub const ImGuiTableColumnFlags_PreferSortDescending: ImGuiTableColumnFlags = 1 << 15;
// Make the initial sort direction Descending when first sorting on this column.
pub const ImGuiTableColumnFlags_IndentEnable: ImGuiTableColumnFlags = 1 << 16;
// Use current Indent value when entering cell (default for column 0).
pub const ImGuiTableColumnFlags_IndentDisable: ImGuiTableColumnFlags = 1 << 17;  // Ignore current Indent value when entering cell (default for columns > 0). Indentation changes _within_ the cell will still be honored.

// Output status flags, read-only via TableGetColumnFlags()
pub const ImGuiTableColumnFlags_IsEnabled: ImGuiTableColumnFlags = 1 << 24;
// Status: is enabled == not hidden by user/api (referred to as "Hide" in _DefaultHide and _NoHide) flags.
pub const ImGuiTableColumnFlags_IsVisible: ImGuiTableColumnFlags = 1 << 25;
// Status: is visible == is enabled AND not clipped by scrolling.
pub const ImGuiTableColumnFlags_IsSorted: ImGuiTableColumnFlags = 1 << 26;
// Status: is currently part of the sort specs
pub const ImGuiTableColumnFlags_IsHovered: ImGuiTableColumnFlags = 1 << 27;  // Status: is hovered by mouse

// [Internal] Combinations and masks
pub const ImGuiTableColumnFlags_WidthMask_: ImGuiTableColumnFlags = ImGuiTableColumnFlags_WidthStretch | ImGuiTableColumnFlags_WidthFixed;
pub const ImGuiTableColumnFlags_IndentMask_: ImGuiTableColumnFlags = ImGuiTableColumnFlags_IndentEnable | ImGuiTableColumnFlags_IndentDisable;
pub const ImGuiTableColumnFlags_StatusMask_: ImGuiTableColumnFlags = ImGuiTableColumnFlags_IsEnabled | ImGuiTableColumnFlags_IsVisible | ImGuiTableColumnFlags_IsSorted | ImGuiTableColumnFlags_IsHovered;
pub const ImGuiTableColumnFlags_NoDirectResize_: ImGuiTableColumnFlags = 1 << 30;  // [Internal] Disable user resizing this column directly (it may however we resized indirectly from its left edge)

// Obsolete names (will be removed soon)
// #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
//ImGuiTableColumnFlags_WidthAuto           = ImGuiTableColumnFlags_WidthFixed | ImGuiTableColumnFlags_NoResize, // Column will not stretch and keep resizing based on submitted contents.
// #endif
// };
