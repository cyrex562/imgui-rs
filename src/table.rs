#![allow(non_snake_case)]

use libc::{c_float, c_int, c_void};
use crate::draw_list_splitter::ImDrawListSplitter;
use crate::rect::ImRect;
use crate::span::ImSpan;
use crate::table_cell_data::ImGuiTableCellData;
use crate::table_column::ImGuiTableColumn;
use crate::table_column_sort_specs::ImGuiTableColumnSortSpecs;
use crate::table_flags::ImGuiTableFlags;
use crate::table_instance_data::ImGuiTableInstanceData;
use crate::table_row_flags::ImGuiTableRowFlags;
use crate::table_temp_data::ImGuiTableTempData;
use crate::text_buffer::ImGuiTextBuffer;
use crate::window::ImGuiWindow;
use crate::type_defs::{ImGuiID, ImGuiTableColumnIdx, ImGuiTableDrawChannelIdx};

// FIXME-TABLE: more transient data could be stored in a per-stacked table structure: DrawSplitter, SortSpecs, incoming RowData 
#[derive(Default,Debug,Clone)]
pub struct ImGuiTable {
    pub ID: ImGuiID,
    pub Flags: ImGuiTableFlags,
    pub RawData: *mut c_void,
    // Single allocation to hold Columns[], DisplayOrderToIndex[] and RowCellData[]
    pub TempData: *mut ImGuiTableTempData,
    // Transient data while table is active. Point within g.CurrentTableStack[]
    pub Columns: ImSpan<ImGuiTableColumn>,
    // Point within RawData[]
    pub DisplayOrderToIndex: ImSpan<ImGuiTableColumnIdx>,
    // Point within RawData[]. Store display order of columns (when not reordered, the values are 0...Count-1)
    pub RowCellData: ImSpan<ImGuiTableCellData>,
    // Point within RawData[]. Store cells background requests for current row.
    pub EnabledMaskByDisplayOrder: u64,
    // Column DisplayOrder -> IsEnabled map
    pub EnabledMaskByIndex: u64,
    // Column Index -> IsEnabled map (== not hidden by user/api) in a format adequate for iterating column without touching cold data
    pub VisibleMaskByIndex: u64,
    // Column Index -> IsVisibleX|IsVisibleY map (== not hidden by user/api && not hidden by scrolling/cliprect)
    pub RequestOutputMaskByIndex: u64,
    // Column Index -> IsVisible || AutoFit (== expect user to submit items)
    pub SettingsLoadedFlags: ImGuiTableFlags,
    // Which data were loaded from the .ini file (e.g. when order is not altered we won't save order)
    pub SettingsOffset: c_int,
    // Offset in g.SettingsTables
    pub LastFrameActive: c_int,
    pub ColumnsCount: c_int,
    // Number of columns declared in BeginTable()
    pub CurrentRow: c_int,
    pub CurrentColumn: c_int,
    pub InstanceCurrent: c_int,
    // Count of BeginTable() calls with same ID in the same frame (generally 0). This is a little bit similar to BeginCount for a window, but multiple table with same ID look are multiple tables, they are just synched.
    pub InstanceInteracted: i16,
    // Mark which instance (generally 0) of the same ID is being interacted with
    pub RowPosY1: c_float,
    pub RowPosY2: c_float,
    pub RowMinHeight: c_float,
    // Height submitted to TableNextRow()
    pub RowTextBaseline: c_float,
    pub RowIndentOffsetX: c_float,
    pub RowFlags: ImGuiTableRowFlags,
    // Current row flags, see ImGuiTableRowFlags_
    pub LastRowFlags: ImGuiTableRowFlags,
    pub RowBgColorCounter: c_int,
    // Counter for alternating background colors (can be fast-forwarded by e.g clipper), not same as CurrentRow because header rows typically don't increase this.
    pub RowBgColor: [u32; 2],
    // Background color override for current row.
    pub BorderColorStrong: u32,
    pub BorderColorLight: u32,
    pub BorderX1: c_float,
    pub BorderX2: c_float,
    pub HostIndentX: c_float,
    pub MinColumnWidth: c_float,
    pub OuterPaddingX: c_float,
    pub CellPaddingX: c_float,
    // Padding from each borders
    pub CellPaddingY: c_float,
    pub CellSpacingX1: c_float,
    // Spacing between non-bordered cells
    pub CellSpacingX2: c_float,
    pub InnerWidth: c_float,
    // User value passed to BeginTable(), see comments at the top of BeginTable() for details.
    pub ColumnsGivenWidth: c_float,
    // Sum of current column width
    pub ColumnsAutoFitWidth: c_float,
    // Sum of ideal column width in order nothing to be clipped, used for auto-fitting and content width submission in outer window
    pub ColumnsStretchSumWeights: c_float,
    // Sum of weight of all enabled stretching columns
    pub ResizedColumnNextWidth: c_float,
    pub ResizeLockMinContentsX2: c_float,
    // Lock minimum contents width while resizing down in order to not create feedback loops. But we allow growing the table.
    pub RefScale: c_float,
    // Reference scale to be able to rescale columns on font/dpi changes.
    pub OuterRect: ImRect,
    // Note: for non-scrolling table, OuterRect.Max.y is often f32::MAX until EndTable(), unless a height has been specified in BeginTable().
    pub InnerRect: ImRect,
    // InnerRect but without decoration. As with OuterRect, for non-scrolling tables, InnerRect.Max.y is
    pub WorkRect: ImRect,
    pub InnerClipRect: ImRect,
    pub BgClipRect: ImRect,
    // We use this to cpu-clip cell background color fill, evolve during the frame as we cross frozen rows boundaries
    pub Bg0ClipRectForDrawCmd: ImRect,
    // Actual ImDrawCmd clip rect for BG0/1 channel. This tends to be == Outerwindow.ClipRect at BeginTable() because output in BG0/BG1 is cpu-clipped
    pub Bg2ClipRectForDrawCmd: ImRect,
    // Actual ImDrawCmd clip rect for BG2 channel. This tends to be a correct, tight-fit, because output to BG2 are done by widgets relying on regular ClipRect.
    pub HostClipRect: ImRect,
    // This is used to check if we can eventually merge our columns draw calls into the current draw call of the current window.
    pub HostBackupInnerClipRect: ImRect,
    // Backup of Innerwindow.ClipRect during PushTableBackground()/PopTableBackground()
    pub OuterWindow: *mut ImGuiWindow,
    // Parent window for the table
    pub InnerWindow: *mut ImGuiWindow,
    // Window holding the table data (== OuterWindow or a child window)
    pub ColumnsNames: ImGuiTextBuffer,
    // Contiguous buffer holding columns names
    pub DrawSplitter: *mut ImDrawListSplitter,
    // Shortcut to TempData->DrawSplitter while in table. Isolate draw commands per columns to avoid switching clip rect constantly
    pub InstanceDataFirst: ImGuiTableInstanceData,
    pub InstanceDataExtra: Vec<ImGuiTableInstanceData>,
    // FIXME-OPT: Using a small-vector pattern would be good.
    pub SortSpecsSingle: ImGuiTableColumnSortSpecs,
    pub SortSpecsMulti: Vec<ImGuiTableColumnSortSpecs>,
    // FIXME-OPT: Using a small-vector pattern would be good.
    pub SortSpecs: ImGuiTableSortSpecs,
    // Public facing sorts specs, this is what we return in TableGetSortSpecs()
    pub SortSpecsCount: ImGuiTableColumnIdx,
    pub ColumnsEnabledCount: ImGuiTableColumnIdx,
    // Number of enabled columns (<= ColumnsCount)
    pub ColumnsEnabledFixedCount: ImGuiTableColumnIdx,
    // Number of enabled columns (<= ColumnsCount)
    pub DeclColumnsCount: ImGuiTableColumnIdx,
    // Count calls to TableSetupColumn()
    pub HoveredColumnBody: ImGuiTableColumnIdx,
    // Index of column whose visible region is being hovered. Important: == ColumnsCount when hovering empty region after the right-most column!
    pub HoveredColumnBorder: ImGuiTableColumnIdx,
    // Index of column whose right-border is being hovered (for resizing).
    pub AutoFitSingleColumn: ImGuiTableColumnIdx,
    // Index of single column requesting auto-fit.
    pub ResizedColumn: ImGuiTableColumnIdx,
    // Index of column being resized. Reset when InstanceCurrent==0.
    pub LastResizedColumn: ImGuiTableColumnIdx,
    // Index of column being resized from previous frame.
    pub HeldHeaderColumn: ImGuiTableColumnIdx,
    // Index of column header being held.
    pub ReorderColumn: ImGuiTableColumnIdx,
    // Index of column being reordered. (not cleared)
    pub ReorderColumnDir: ImGuiTableColumnIdx,
    // -1 or +1
    pub LeftMostEnabledColumn: ImGuiTableColumnIdx,
    // Index of left-most non-hidden column.
    pub RightMostEnabledColumn: ImGuiTableColumnIdx,
    // Index of right-most non-hidden column.
    pub LeftMostStretchedColumn: ImGuiTableColumnIdx,
    // Index of left-most stretched column.
    pub RightMostStretchedColumn: ImGuiTableColumnIdx,
    // Index of right-most stretched column.
    pub ContextPopupColumn: ImGuiTableColumnIdx,
    // Column right-clicked on, of -1 if opening context menu from a neutral/empty spot
    pub FreezeRowsRequest: ImGuiTableColumnIdx,
    // Requested frozen rows count
    pub FreezeRowsCount: ImGuiTableColumnIdx,
    // Actual frozen row count (== FreezeRowsRequest, or == 0 when no scrolling offset)
    pub FreezeColumnsRequest: ImGuiTableColumnIdx,
    // Requested frozen columns count
    pub FreezeColumnsCount: ImGuiTableColumnIdx,
    // Actual frozen columns count (== FreezeColumnsRequest, or == 0 when no scrolling offset)
    pub RowCellDataCurrent: ImGuiTableColumnIdx,
    // Index of current RowCellData[] entry in current row
    pub DummyDrawChannel: ImGuiTableDrawChannelIdx,
    // Redirect non-visible columns here.
    pub Bg2DrawChannelCurrent: ImGuiTableDrawChannelIdx,
    // For Selectable() and other widgets drawing across columns after the freezing line. Index within DrawSplitter.Channels[]
    pub Bg2DrawChannelUnfrozen: ImGuiTableDrawChannelIdx,
    pub IsLayoutLocked: bool,
    // Set by TableUpdateLayout() which is called when beginning the first row.
    pub IsInsideRow: bool,
    // Set when inside TableBeginRow()/TableEndRow().
    pub IsInitializing: bool,
    pub IsSortSpecsDirty: bool,
    pub IsUsingHeaders: bool,
    // Set when the first row had the ImGuiTableRowFlags_Headers flag.
    pub IsContextPopupOpen: bool,
    // Set when default context menu is open (also see: ContextPopupColumn, InstanceInteracted).
    pub IsSettingsRequestLoad: bool,
    pub IsSettingsDirty: bool,
    // Set when table settings have changed and needs to be reported into ImGuiTableSetttings data.
    pub IsDefaultDisplayOrder: bool,
    // Set when display order is unchanged from default (DisplayOrder contains 0...Count-1)
    pub IsResetAllRequest: bool,
    pub IsResetDisplayOrderRequest: bool,
    pub IsUnfrozenRows: bool,
    // Set when we got past the frozen row.
    pub IsDefaultSizingPolicy: bool,
    // Set if user didn't explicitly set a sizing policy in BeginTable()
    pub MemoryCompacted: bool,
    pub HostSkipItems: bool,              // Backup of Innerwindow.SkipItem at the end of BeginTable(), because we will overwrite Innerwindow.SkipItem on a per-column basis
}

impl ImGuiTable {
    // ImGuiTable()                { memset(this, 0, sizeof(*this)); LastFrameActive = -1; }
    
    
    // ~ImGuiTable()               { IM_FREE(RawData); }
}
