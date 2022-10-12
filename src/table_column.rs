#![allow(non_snake_case)]

use libc::c_float;
use crate::rect::ImRect;
use crate::sort_direction::{ImGuiSortDirection, ImGuiSortDirection_None};
use crate::table_column_flags::ImGuiTableColumnFlags;
use crate::type_defs::{ImGuiID, ImGuiTableColumnIdx, ImGuiTableDrawChannelIdx};

// [Internal] sizeof() ~ 104
// We use the terminology "Enabled" to refer to a column that is not Hidden by user/api.
// We use the terminology "Clipped" to refer to a column that is out of sight because of scrolling/clipping.
// This is in contrast with some user-facing api such as IsItemVisible() / IsRectVisible() which use "Visible" to mean "not clipped".
#[derive(Default, Debug, Clone)]
pub struct ImGuiTableColumn {
    pub Flags: ImGuiTableColumnFlags,
    // Flags after some patching (not directly same as provided by user). See ImGuiTableColumnFlags_
    pub WidthGiven: c_float,
    // Final/actual width visible == (MaxX - MinX), locked in TableUpdateLayout(). May be > WidthRequest to honor minimum width, may be < WidthRequest to honor shrinking columns down in tight space.
    pub MinX: c_float,
    // Absolute positions
    pub MaxX: c_float,
    pub WidthRequest: c_float,
    // Master width absolute value when !(Flags & _WidthStretch). When Stretch this is derived every frame from StretchWeight in TableUpdateLayout()
    pub WidthAuto: c_float,
    // Automatic width
    pub StretchWeight: c_float,
    // Master width weight when (Flags & _WidthStretch). Often around ~1f32 initially.
    pub InitStretchWeightOrWidth: c_float,
    // Value passed to TableSetupColumn(). For Width it is a content width (_without padding_).
    pub ClipRect: ImRect,
    // Clipping rectangle for the column
    pub UserID: ImGuiID,
    // Optional, value passed to TableSetupColumn()
    pub WorkMinX: c_float,
    // Contents region min ~(MinX + CellPaddingX + CellSpacingX1) == cursor start position when entering column
    pub WorkMaxX: c_float,
    // Contents region max ~(MaxX - CellPaddingX - CellSpacingX2)
    pub ItemWidth: c_float,
    // Current item width for the column, preserved across rows
    pub ContentMaxXFrozen: c_float,
    // Contents maximum position for frozen rows (apart from headers), from which we can infer content width.
    pub ContentMaxXUnfrozen: c_float,
    pub ContentMaxXHeadersUsed: c_float,
    // Contents maximum position for headers rows (regardless of freezing). TableHeader() automatically softclip itself + report ideal desired size, to avoid creating extraneous draw calls
    pub ContentMaxXHeadersIdeal: c_float,
    pub NameOffset: i16,
    // Offset into parent ColumnsNames[]
    pub DisplayOrder: ImGuiTableColumnIdx,
    // Index within Table's IndexToDisplayOrder[] (column may be reordered by users)
    pub IndexWithinEnabledSet: ImGuiTableColumnIdx,
    // Index within enabled/visible set (<= IndexToDisplayOrder)
    pub PrevEnabledColumn: ImGuiTableColumnIdx,
    // Index of prev enabled/visible column within Columns[], -1 if first enabled/visible column
    pub NextEnabledColumn: ImGuiTableColumnIdx,
    // Index of next enabled/visible column within Columns[], -1 if last enabled/visible column
    pub SortOrder: ImGuiTableColumnIdx,
    // Index of this column within sort specs, -1 if not sorting on this column, 0 for single-sort, may be >0 on multi-sort
    pub DrawChannelCurrent: ImGuiTableDrawChannelIdx,
    // Index within DrawSplitter.Channels[]
    pub DrawChannelFrozen: ImGuiTableDrawChannelIdx,
    // Draw channels for frozen rows (often headers)
    pub DrawChannelUnfrozen: ImGuiTableDrawChannelIdx,
    // Draw channels for unfrozen rows
    pub IsEnabled: bool,
    // IsUserEnabled && (Flags & ImGuiTableColumnFlags_Disabled) == 0
    pub IsUserEnabled: bool,
    // Is the column not marked Hidden by the user? (unrelated to being off view, e.g. clipped by scrolling).
    pub IsUserEnabledNextFrame: bool,
    pub IsVisibleX: bool,
    // Is actually in view (e.g. overlapping the host window clipping rectangle, not scrolled).
    pub IsVisibleY: bool,
    pub IsRequestOutput: bool,
    // Return value for TableSetColumnIndex() / TableNextColumn(): whether we request user to output contents or not.
    pub IsSkipItems: bool,
    // Do we want item submissions to this column to be completely ignored (no layout will happen).
    pub IsPreserveWidthAuto: bool,
    pub NavLayerCurrent: i32,
    // ImGuiNavLayer in 1 byte
    pub AutoFitQueue: u8,
    // Queue of 8 values for the next 8 frames to request auto-fit
    pub CannotSkipItemsQueue: u8,
    // Queue of 8 values for the next 8 frames to disable Clipped/SkipItem
    pub SortDirection: ImGuiSortDirection,
    // ImGuiSortDirection_Ascending or ImGuiSortDirection_Descending
    pub SortDirectionsAvailCount: i8,
    // Number of available sort directions (0 to 3)
    pub SortDirectionsAvailMask: i8,
    // Mask of available sort directions (1-bit each)
    pub SortDirectionsAvailList: i8,        // Ordered of available sort directions (2-bits each)
}

impl ImGuiTableColumn {
    pub fn new() -> Self {
        Self
        {
            // memset(this, 0, sizeof(*this));
            StretchWeight: -1f32,
            WidthRequest : -1f32,
            NameOffset : -1,
            DisplayOrder : -1,
            IndexWithinEnabledSet : -1,
            PrevEnabledColumn : -1,
            NextEnabledColumn : -1,
            SortOrder : -1,
            SortDirection : ImGuiSortDirection_None,
            DrawChannelCurrent : -1,
            DrawChannelFrozen : -1,
            DrawChannelUnfrozen : -1,
            ..Default::default()
        }
    }
}
