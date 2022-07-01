use crate::imgui_color::ImColor;
use crate::imgui_h::{ImDrawListSplitter, ImGuiID, ImGuiSortDirection, ImGuiTableColumnFlags, ImGuiTableColumnSortSpecs, ImGuiTableFlags, ImGuiTableRowFlags, ImGuiTableSortSpecs, ImGuiTextBuffer};
use crate::imgui_rect::ImRect;
use crate::imgui_vec::{ImVec1, ImVec2};
use crate::imgui_window::ImGuiWindow;

// #define IM_COL32_DISABLE                IM_COL32(0,0,0,1)   // Special sentinel code which cannot be used as a regular color.
pub const IM_COL_32_DISABLE: ImColor = ImColor::new4(0,0,0,1);
// #define IMGUI_TABLE_MAX_COLUMNS         64                  // sizeof(ImU64) * 8. This is solely because we frequently encode columns set in a ImU64.
pub const IMGUI_TABLE_MAX_COLUMNS: usize = 64;
// #define IMGUI_TABLE_MAX_DRAW_CHANNELS   (4 + 64 * 2)        // See TableSetupDrawChannels()
pub const IMGUI_TABLE_MAX_DRAW_CHANNELS: u32 = 4 + 64 *2;

// Our current column maximum is 64 but we may raise that in the future.
// typedef ImS8 ImGuiTableColumnIdx;
pub type ImGuiTableColumnIdx = i8;
// typedef ImU8 ImGuiTableDrawChannelIdx;
pub type ImGuiTableDrawChannelIdx = i8;

// [Internal] sizeof() ~ 104
// We use the terminology "Enabled" to refer to a column that is not Hidden by user/api.
// We use the terminology "Clipped" to refer to a column that is out of sight because of scrolling/clipping.
// This is in contrast with some user-facing api such as IsItemVisible() / IsRectVisible() which use "Visible" to mean "not clipped".
#[derive(Default,Debug,Clone)]
pub struct ImGuiTableColumn
{
    // ImGuiTableColumnFlags   Flags;                          // Flags after some patching (not directly same as provided by user). See ImGuiTableColumnFlags_
    pub Flags: ImGuiTableColumnFlags,
    // float                   WidthGiven;                     // Final/actual width visible == (MaxX - MinX), locked in TableUpdateLayout(). May be > WidthRequest to honor minimum width, may be < WidthRequest to honor shrinking columns down in tight space.
    pub WidthGiven: f32,
    // float                   MinX;                           // Absolute positions
    pub MinX: f32,
    // float                   MaxX;
    pub MaxX: f32,
    // float                   WidthRequest;                   // Master width absolute value when !(Flags & _WidthStretch). When Stretch this is derived every frame from StretchWeight in TableUpdateLayout()
    pub WidthRequest: f32,
    // float                   WidthAuto;                      // Automatic width
    pub WidthAuto: f32,
    // float                   StretchWeight;                  // Master width weight when (Flags & _WidthStretch). Often around ~1.0 initially.
    pub StretchWeight: f32,
    // float                   InitStretchWeightOrWidth;       // Value passed to TableSetupColumn(). For Width it is a content width (_without padding_).
    pub InitStretchWeightOrWidth: f32,
    // ImRect                  ClipRect;                       // Clipping rectangle for the column
    pub ClipRect: ImRect,
    // ImGuiID                 UserID;                         // Optional, value passed to TableSetupColumn()
    pub UserID: ImGuiID,
    // float                   WorkMinX;                       // Contents region min ~(MinX + CellPaddingX + CellSpacingX1) == cursor start position when entering column
    pub WorkMinX: f32,
    // float                   WorkMaxX;                       // Contents region max ~(MaxX - CellPaddingX - CellSpacingX2)
    pub WorkMaxX: f32,
    // float                   ItemWidth;                      // Current item width for the column, preserved across rows
    pub ItemWidth: f32,
    // float                   ContentMaxXFrozen;              // Contents maximum position for frozen rows (apart from headers), from which we can infer content width.
    pub ContextMaxXFrozen: f32,
    // float                   ContentMaxXUnfrozen;
    pub ContentMaxXUnfrozen: f32,
    // float                   ContentMaxXHeadersUsed;         // Contents maximum position for headers rows (regardless of freezing). TableHeader() automatically softclip itself + report ideal desired size, to avoid creating extraneous draw calls
    pub ContentMaxXHeadersUsed: f32,
    // float                   ContentMaxXHeadersIdeal;
    pub ContentMaxXHeadersIdeal: f32,
    // ImS16                   NameOffset;                     // Offset into parent ColumnsNames[]
    pub NameOffset: i16,
    // ImGuiTableColumnIdx     DisplayOrder;                   // Index within Table's IndexToDisplayOrder[] (column may be reordered by users)
    pub DisplayOrder: ImGuiTableColumnIdx,
    // ImGuiTableColumnIdx     IndexWithinEnabledSet;          // Index within enabled/visible set (<= IndexToDisplayOrder)
    pub IndexWithinEnabledSet: ImGuiTableColumnIdx,
    // ImGuiTableColumnIdx     PrevEnabledColumn;              // Index of prev enabled/visible column within Columns[], -1 if first enabled/visible column
    pub PrevEnabledColumn: ImGuiTableColumnIdx,
    // ImGuiTableColumnIdx     NextEnabledColumn;              // Index of next enabled/visible column within Columns[], -1 if last enabled/visible column
    pub NextEnabledColumn: ImGuiTableColumnIdx,
    // ImGuiTableColumnIdx     SortOrder;                      // Index of this column within sort specs, -1 if not sorting on this column, 0 for single-sort, may be >0 on multi-sort
    pub SortOrder: ImGuiTableColumnIdx,
    // ImGuiTableDrawChannelIdx DrawChannelCurrent;            // Index within DrawSplitter.Channels[]
    pub DrawChannelCurrent: ImGuiTableDrawChannelIdx,
    // ImGuiTableDrawChannelIdx DrawChannelFrozen;             // Draw channels for frozen rows (often headers)
    pub DrawChannelFrozen: ImGuiTableDrawChannelIdx,
    // ImGuiTableDrawChannelIdx DrawChannelUnfrozen;           // Draw channels for unfrozen rows
    pub DrawChannelUnFrozen: ImGuiTableDrawChannelIdx,
    // bool                    IsEnabled;                      // IsUserEnabled && (Flags & ImGuiTableColumnFlags_Disabled) == 0
    pub IsEnabled: bool,
    // bool                    IsUserEnabled;                  // Is the column not marked Hidden by the user? (unrelated to being off view, e.g. clipped by scrolling).
    pub IsUserEnabled: bool,
    // bool                    IsUserEnabledNextFrame;
    pub IsUserEnabledNextFrame: bool,
    // bool                    IsVisibleX;                     // Is actually in view (e.g. overlapping the host window clipping rectangle, not scrolled).
    pub IsVisibleX: bool,
    // bool                    IsVisibleY;
    pub IsVisibleY: bool,
    // bool                    IsRequestOutput;                // Return value for TableSetColumnIndex() / TableNextColumn(): whether we request user to output contents or not.
    pub IsRequestOutput: bool,
    // bool                    IsSkipItems;                    // Do we want item submissions to this column to be completely ignored (no layout will happen).
    pub IsSkipItems: bool,
    // bool                    IsPreserveWidthAuto;
    pub IsPreserveWidthAutio: bool,
    // ImS8                    NavLayerCurrent;                // ImGuiNavLayer in 1 byte
    pub NavLayerCurrent: i8,
    // ImU8                    AutoFitQueue;                   // Queue of 8 values for the next 8 frames to request auto-fit
    pub AutoFitQueue: i8,
    // ImU8                    CannotSkipItemsQueue;           // Queue of 8 values for the next 8 frames to disable Clipped/SkipItem
    pub CannotSKipItemsQueue: i8,
    // ImU8                    SortDirection : 2;              // ImGuiSortDirection_Ascending or ImGuiSortDirection_Descending
    pub SortDIrection: ImGuiSortDirection,
    // ImU8                    SortDirectionsAvailCount : 2;   // Number of available sort directions (0 to 3)
    pub SortDirectionsAvailCount: i8,
    // ImU8                    SortDirectionsAvailMask : 4;    // Mask of available sort directions (1-bit each)
    pub SortDirectionsAvailMask: i8,
    // ImU8                    SortDirectionsAvailList;        // Ordered of available sort directions (2-bits each)
    pub SortDirectionsAvailList: i8,
}

impl ImGuiTableColumn {
    // ImGuiTableColumn()
    //     {
    //         memset(this, 0, sizeof(*this));
    //         StretchWeight = WidthRequest = -1.0;
    //         NameOffset = -1;
    //         DisplayOrder = IndexWithinEnabledSet = -1;
    //         PrevEnabledColumn = NextEnabledColumn = -1;
    //         SortOrder = -1;
    //         SortDirection = ImGuiSortDirection_None;
    //         DrawChannelCurrent = DrawChannelFrozen = DrawChannelUnfrozen = (ImU8)-1;
    //     }
    pub fn new() -> Self {
        Self {
            Flags: ImGuiTableColumnFlags::ImGuiTableColumnFlags_None,
            WidthGiven: 0.0,
            MinX: 0.0,
            StretchWeight: -1.0,
            InitStretchWeightOrWidth: 0.0,
            ClipRect: ImRect::new(),
            UserID: 0,
            WorkMinX: 0.0,
            WorkMaxX: 0.0,
            ItemWidth: 0.0,
            ContextMaxXFrozen: 0.0,
            ContentMaxXUnfrozen: 0.0,
            ContentMaxXHeadersUsed: 0.0,
            WidthRequest: -1.0,
            NameOffset: -1,
            DisplayOrder: -1,
            IndexWithinEnabledSet: -1,
            PrevEnabledColumn: -1,
            NextEnabledColumn: -1,
            SortOrder: -1,
            SortDIrection: ImGuiSortDirection::None,
            SortDirectionsAvailCount: 0,
            SortDirectionsAvailMask: 0,
            DrawChannelCurrent: -1,
            DrawChannelFrozen: -1,
            DrawChannelUnFrozen: -1,
            IsEnabled: false,
            IsUserEnabled: false,
            IsUserEnabledNextFrame: false,
            IsVisibleX: false,
            IsVisibleY: false,
            IsRequestOutput: false,
            IsSkipItems: false,
            IsPreserveWidthAutio: false,
            NavLayerCurrent: 0,
            AutoFitQueue: 0,
            MaxX: 0.0,
            WidthAuto: 0.0,
            ContentMaxXHeadersIdeal: 0.0,
            CannotSKipItemsQueue: 0,
            SortDirectionsAvailList: 0
        }
    }
}

// Transient cell data stored per row.
// sizeof() ~ 6
#[derive(Debug,Default,Clone)]
pub struct ImGuiTableCellData
{
    // ImU32                       BgColor;    // Actual color
    pub BgColor: u32,
    // ImGuiTableColumnIdx         Column;     // Column number
    pub Column: ImGuiTableColumnIdx,
}

// Per-instance data that needs preserving across frames (seemingly most others do not need to be preserved aside from debug needs, does that needs they could be moved to ImGuiTableTempData ?)
#[derive(Debug,Default,Clone)]
pub struct ImGuiTableInstanceData
{
    // float                       LastOuterHeight;            // Outer height from last frame // FIXME: multi-instance issue (#3955)
    pub LastOuterHeight: f32,
    // float                       LastFirstRowHeight;         // Height of first row from last frame // FIXME: possible multi-instance issue?
    pub LastFirstRowHeight: f32,
}

impl ImGuiTableInstanceData {
    // ImGuiTableInstanceData()    { LastOuterHeight = LastFirstRowHeight = 0.0; }
    pub fn new() -> Self {
        Self {
            LastOuterHeight: 0.0,
            LastFirstRowHeight: 0.0,
        }
    }
}

// FIXME-TABLE: more transient data could be stored in a per-stacked table structure: DrawSplitter, SortSpecs, incoming RowData
pub struct  ImGuiTable
{
    // ImGuiID                     ID;
    pub ID: ImGuiID,
    // ImGuiTableFlags             Flags;
    pub Flags: ImGuiTableFlags,
    // void*                       RawData;                    // Single allocation to hold Columns[], DisplayOrderToIndex[] and RowCellData[]
    pub RawData: Vec<u8>,
    // ImGuiTableTempData*         TempData;                   // Transient data while table is active. Point within g.CurrentTableStack[]
    pub TempData: *mut ImGuiTableTempData,
    // ImSpan<ImGuiTableColumn>    Columns;                    // Point within RawData[]
    pub Columns: Vec<ImGuiTableColumn>,
    // ImSpan<ImGuiTableColumnIdx> DisplayOrderToIndex;        // Point within RawData[]. Store display order of columns (when not reordered, the values are 0...Count-1)
    pub DisplayOrderToIndex: Vec<ImGuiTableColumnIdx>,
    // ImSpan<ImGuiTableCellData>  RowCellData;                // Point within RawData[]. Store cells background requests for current row.
    pub RowCellData: Vec<ImGuiTableCellData>,
    // ImU64                       EnabledMaskByDisplayOrder;  // Column DisplayOrder -> IsEnabled map
    pub EnabledMaskByDisplayOrder: u64,
    // ImU64                       EnabledMaskByIndex;         // Column Index -> IsEnabled map (== not hidden by user/api) in a format adequate for iterating column without touching cold data
    pub EnabledMaskByIndex: u64,
    // ImU64                       VisibleMaskByIndex;         // Column Index -> IsVisibleX|IsVisibleY map (== not hidden by user/api && not hidden by scrolling/cliprect)
    pub VisibleMaskByIndex: u64,
    // ImU64                       RequestOutputMaskByIndex;   // Column Index -> IsVisible || AutoFit (== expect user to submit items)
    pub RequestOutputMaskByIndex: u64,
    // ImGuiTableFlags             SettingsLoadedFlags;        // Which data were loaded from the .ini file (e.g. when order is not altered we won't save order)
    pub SettingsLoadedFlags: ImGuiTableFlags,
    // int                         SettingsOffset;             // Offset in g.SettingsTables
    pub SettingsOffset: i32,
    // int                         LastFrameActive;
    pub LastFrameActive: i32,
    // int                         ColumnsCount;               // Number of columns declared in BeginTable()
    pub ColumnsCount: i32,
    // int                         CurrentRow;
    pub CurrentRow: i32,
    // int                         CurrentColumn;
    pub CurrentColumn: i32,
    // ImS16                       InstanceCurrent;            // Count of BeginTable() calls with same ID in the same frame (generally 0). This is a little bit similar to BeginCount for a window, but multiple table with same ID look are multiple tables, they are just synched.
    pub InstanceCurrent: i16,
    // ImS16                       InstanceInteracted;         // Mark which instance (generally 0) of the same ID is being interacted with
    pub InstanceInteracted: i16,
    // float                       RowPosY1;
    pub RowPosY1: f32,
    // float                       RowPosY2;
    pub RowPosY2: f32,
    // float                       RowMinHeight;               // Height submitted to TableNextRow()
    pub RowMinHeight: f32,
    // float                       RowTextBaseline;
    pub RowTextBaseLine: f32,
    // float                       RowIndentOffsetX;
    pub RowIndentOffsetX: f32,
    // ImGuiTableRowFlags          RowFlags : 16;              // Current row flags, see ImGuiTableRowFlags_
    pub RowFlags: ImGuiTableRowFlags,
    // ImGuiTableRowFlags          LastRowFlags : 16;
    pub LastRowFlags: ImGuiTableRowFlags,
    // int                         RowBgColorCounter;          // Counter for alternating background colors (can be fast-forwarded by e.g clipper), not same as CurrentRow because header rows typically don't increase this.
    pub RowBgColorCounter: i32,
    // ImU32                       RowBgColor[2];              // Background color override for current row.
    pub RowBgColor: [u32;2],
    // ImU32                       BorderColorStrong;
    pub BorderColorStrong: u32,
    // ImU32                       BorderColorLight;
    pub BorderColorLight: u32,
    // float                       BorderX1;
    pub BorderX1: f32,
    // float                       BorderX2;
    pub BorderX2: f32,
    // float                       HostIndentX;
    pub HostIndentX: f32,
    // float                       MinColumnWidth;
    pub MinColumnWidth: f32,
    // float                       OuterPaddingX;
    pub OuterPaddingX: f32,
    // float                       CellPaddingX;               // Padding from each borders
    pub CellPaddingX: f32,
    // float                       CellPaddingY;
    pub CellPaddingY: f32,
    // float                       CellSpacingX1;              // Spacing between non-bordered cells
    pub CellSpacingX1: f32,
    // float                       CellSpacingX2;
    pub CellSpacingX2: f32,
    // float                       InnerWidth;                 // User value passed to BeginTable(), see comments at the top of BeginTable() for details.
    pub InnerWidth: f32,
    // float                       ColumnsGivenWidth;          // Sum of current column width
    pub ColumnsGivenWidth: f32,
    // float                       ColumnsAutoFitWidth;        // Sum of ideal column width in order nothing to be clipped, used for auto-fitting and content width submission in outer window
    pub ColumnsAutoFitWidth: f32,
    // float                       ColumnsStretchSumWeights;   // Sum of weight of all enabled stretching columns
    pub ColumnsStretchSumWeights: f32,
    // float                       ResizedColumnNextWidth;
    pub ResizeColumnNextWidth: f32,
    // float                       ResizeLockMinContentsX2;    // Lock minimum contents width while resizing down in order to not create feedback loops. But we allow growing the table.
    pub ResizeLockMinContentsX2: f32,
    // float                       RefScale;                   // Reference scale to be able to rescale columns on font/dpi changes.
    pub RefScale: f32,
    // ImRect                      OuterRect;                  // Note: for non-scrolling table, OuterRect.Max.y is often FLT_MAX until EndTable(), unless a height has been specified in BeginTable().
    pub OuterRect: ImRect,
    // ImRect                      InnerRect;                  // InnerRect but without decoration. As with OuterRect, for non-scrolling tables, InnerRect.Max.y is
    pub InnerRect: ImRect,
    // ImRect                      WorkRect;
    pub WorkRect: ImRect,
    // ImRect                      InnerClipRect;
    pub InnerClipRect: ImRect,
    // ImRect                      BgClipRect;                 // We use this to cpu-clip cell background color fill, evolve during the frame as we cross frozen rows boundaries
    pub BgClipRect: ImRect,
    // ImRect                      Bg0ClipRectForDrawCmd;      // Actual ImDrawCmd clip rect for BG0/1 channel. This tends to be == OuterWindow->ClipRect at BeginTable() because output in BG0/BG1 is cpu-clipped
    pub BgClipRectForDrawCmd: ImRect,
    // ImRect                      Bg2ClipRectForDrawCmd;      // Actual ImDrawCmd clip rect for BG2 channel. This tends to be a correct, tight-fit, because output to BG2 are done by widgets relying on regular ClipRect.
    pub Bg2ClipRectForDrawCmd: ImRect,
    // ImRect                      HostClipRect;               // This is used to check if we can eventually merge our columns draw calls into the current draw call of the current window.
    pub HostClipRect: ImRect,
    // ImRect                      HostBackupInnerClipRect;    // Backup of InnerWindow->ClipRect during PushTableBackground()/PopTableBackground()
    pub HostBackupInnerClipRect: ImRect,
    // ImGuiWindow*                OuterWindow;                // Parent window for the table
    pub OuterWindow: *mut ImGuiWindow,
    // ImGuiWindow*                InnerWindow;                // Window holding the table data (== OuterWindow or a child window)
    pub InnerWindow: *mut ImGuiWindow,
    // ImGuiTextBuffer             ColumnsNames;               // Contiguous buffer holding columns names
    pub ColumnsNames: ImGuiTextBuffer,
    // ImDrawListSplitter*         DrawSplitter;               // Shortcut to TempData->DrawSplitter while in table. Isolate draw commands per columns to avoid switching clip rect constantly
    pub DrawSplitter: *mut ImDrawListSplitter,
    // ImGuiTableInstanceData      InstanceDataFirst;
    pub InstanceDataFirst: ImGuiTableInstanceData,
    // ImVector<ImGuiTableInstanceData>    InstanceDataExtra;  // FIXME-OPT: Using a small-vector pattern would be good.
    pub InstanceDataExtra: Vec<ImGuiTableInstanceData>,
    // ImGuiTableColumnSortSpecs   SortSpecsSingle;
    pub SortSpecsSingle: ImGuiTableColumnSortSpecs,
    // ImVector<ImGuiTableColumnSortSpecs> SortSpecsMulti;     // FIXME-OPT: Using a small-vector pattern would be good.
    pub SortSpecsMulti: Vec<ImGuiTableColumnSortSpecs>,
    // ImGuiTableSortSpecs         SortSpecs;                  // Public facing sorts specs, this is what we return in TableGetSortSpecs()
    pub SortSpecs: ImGuiTableSortSpecs,
    // ImGuiTableColumnIdx         SortSpecsCount;
    pub SortSpecsCount: ImGuiTableColumnIdx,
    // ImGuiTableColumnIdx         ColumnsEnabledCount;        // Number of enabled columns (<= ColumnsCount)
    pub ColumnsEnabledCount: ImGuiTableColumnIdx,
    // ImGuiTableColumnIdx         ColumnsEnabledFixedCount;   // Number of enabled columns (<= ColumnsCount)
    pub ColumnsEnabledFixedCount: ImGuiTableColumnIdx,
    // ImGuiTableColumnIdx         DeclColumnsCount;           // Count calls to TableSetupColumn()
    pub DeclColumnsCount: ImGuiTableColumnIdx,
    // ImGuiTableColumnIdx         HoveredColumnBody;          // Index of column whose visible region is being hovered. Important: == ColumnsCount when hovering empty region after the right-most column!
    pub HoveredColumnBody: ImGuiTableColumnIdx,
    // ImGuiTableColumnIdx         HoveredColumnBorder;        // Index of column whose right-border is being hovered (for resizing).
    pub HoveredColumnBorder: ImGuiTableColumnIdx,
    // ImGuiTableColumnIdx         AutoFitSingleColumn;        // Index of single column requesting auto-fit.
    pub AutoFitSingleColumn: ImGuiTableColumnIdx,
    // ImGuiTableColumnIdx         ResizedColumn;              // Index of column being resized. Reset when InstanceCurrent==0.
    pub ResizedColumn: ImGuiTableColumnIdx,
    // ImGuiTableColumnIdx         LastResizedColumn;          // Index of column being resized from previous frame.
    pub LastResizedColumn: ImGuiTableColumnIdx,
    // ImGuiTableColumnIdx         HeldHeaderColumn;           // Index of column header being held.
    pub HeldHeaderColumn: ImGuiTableColumnIdx,
    // ImGuiTableColumnIdx         ReorderColumn;              // Index of column being reordered. (not cleared)
    pub ReorderColumn: ImGuiTableColumnIdx,
    // ImGuiTableColumnIdx         ReorderColumnDir;           // -1 or +1
    pub ReorderColumnDir: ImGuiTableColumnIdx,
    // ImGuiTableColumnIdx         LeftMostEnabledColumn;      // Index of left-most non-hidden column.
    pub LeftMostEnabledColumn: ImGuiTableColumnIdx,
    // ImGuiTableColumnIdx         RightMostEnabledColumn;     // Index of right-most non-hidden column.
    pub RightMosstEnabledColumn: ImGuiTableColumnIdx,
    // ImGuiTableColumnIdx         LeftMostStretchedColumn;    // Index of left-most stretched column.
    pub LeftMostStretchedColumn: ImGuiTableColumnIdx,
    // ImGuiTableColumnIdx         RightMostStretchedColumn;   // Index of right-most stretched column.
    pub RightMostStretchedColumn: ImGuiTableColumnIdx,
    // ImGuiTableColumnIdx         ContextPopupColumn;         // Column right-clicked on, of -1 if opening context menu from a neutral/empty spot
    pub ContextPopupColumn: ImGuiTableColumnIdx,
    // ImGuiTableColumnIdx         FreezeRowsRequest;          // Requested frozen rows count
    pub FreezeRowsRequest: ImGuiTableColumnIdx,
    // ImGuiTableColumnIdx         FreezeRowsCount;            // Actual frozen row count (== FreezeRowsRequest, or == 0 when no scrolling offset)
    pub FreezeRowsCount: ImGuiTableColumnIdx,
    // ImGuiTableColumnIdx         FreezeColumnsRequest;       // Requested frozen columns count
    pub FreezeColumnsRequest: ImGuiTableColumnIdx,
    // ImGuiTableColumnIdx         FreezeColumnsCount;         // Actual frozen columns count (== FreezeColumnsRequest, or == 0 when no scrolling offset)
    pub FreezeColumnsCount: ImGuiTableColumnIdx,
    // ImGuiTableColumnIdx         RowCellDataCurrent;         // Index of current RowCellData[] entry in current row
    pub RowCellDataCurrent: ImGuiTableColumnIdx,
    // ImGuiTableDrawChannelIdx    DummyDrawChannel;           // Redirect non-visible columns here.
    pub DummyDrawChannel: ImGuiTableDrawChannelIdx,
    // ImGuiTableDrawChannelIdx    Bg2DrawChannelCurrent;      // For Selectable() and other widgets drawing across columns after the freezing line. Index within DrawSplitter.Channels[]
    pub Bg2DrawChannelCurrent: ImGuiTableColumnIdx,
    // ImGuiTableDrawChannelIdx    Bg2DrawChannelUnfrozen;
    pub Bg2DrawChannelUnfrozen: ImGuiTableColumnIdx,
    // bool                        IsLayoutLocked;             // Set by TableUpdateLayout() which is called when beginning the first row.
    pub IsLayoutLocked: bool,
    // bool                        IsInsideRow;                // Set when inside TableBeginRow()/TableEndRow().
    pub IsInsideRow: bool,
    // bool                        IsInitializing;
    pub IsInitializing: bool,
    // bool                        IsSortSpecsDirty;
    pub IsSortSpecsDirty: bool,
    // bool                        IsUsingHeaders;             // Set when the first row had the ImGuiTableRowFlags_Headers flag.
    pub IsUsingHeaders: bool,
    // bool                        IsContextPopupOpen;         // Set when default context menu is open (also see: ContextPopupColumn, InstanceInteracted).
    pub IsContextPopupOpen: bool,
    // bool                        IsSettingsRequestLoad;
    pub IsSettingsRequestLoad: bool,
    // bool                        IsSettingsDirty;            // Set when table settings have changed and needs to be reported into ImGuiTableSetttings data.
    pub IsSettingsDirty: bool,
    // bool                        IsDefaultDisplayOrder;      // Set when display order is unchanged from default (DisplayOrder contains 0...Count-1)
    pub IsDefaultDisplayOrder: bool,
    // bool                        IsResetAllRequest;
    pub IsResetAllRequest: bool,
    // bool                        IsResetDisplayOrderRequest;
    pub IsResetDisplayOrderRequest: bool,
    // bool                        IsUnfrozenRows;             // Set when we got past the frozen row.
    pub IsUnfrozenRows: bool,
    // bool                        IsDefaultSizingPolicy;      // Set if user didn't explicitly set a sizing policy in BeginTable()
    pub IsDefaultSizingPolicy: bool,
    // bool                        MemoryCompacted;
    pub MemoryCompacted: bool,
    // bool                        HostSkipItems;              // Backup of InnerWindow->SkipItem at the end of BeginTable(), because we will overwrite InnerWindow->SkipItem on a per-column basis
    pub HostSkipItems: bool,
}

impl ImGuiTable {
    // ImGuiTable()                { memset(this, 0, sizeof(*this)); LastFrameActive = -1; }
    pub fn new() -> Self {
        Self {
            LastFrameActive: -1,
            ..Default::default()
        }
    }
    //     ~ImGuiTable()               { IM_FREE(RawData); }
}

// Transient data that are only needed between BeginTable() and EndTable(), those buffers are shared (1 per level of stacked table).
// - Accessing those requires chasing an extra pointer so for very frequently used data we leave them in the main table structure.
// - We also leave out of this structure data that tend to be particularly useful for debugging/metrics.
#[derive(Default,Debug,Clone)]
pub struct  ImGuiTableTempData
{
    // int                         TableIndex;                 // Index in g.Tables.Buf[] pool
    pub TableIndex: i32,
    // float                       LastTimeActive;             // Last timestamp this structure was used
    pub LastTimeActive: f32,

    // ImVec2                      UserOuterSize;              // outer_size.x passed to BeginTable()
    pub UserOuterSize: ImVec2,
    // ImDrawListSplitter          DrawSplitter;
    pub DrawSplitter: ImDrawListSplitter,

    // ImRect                      HostBackupWorkRect;         // Backup of InnerWindow->WorkRect at the end of BeginTable()
    pub HostBackupWorkRect: ImRect,
    // ImRect                      HostBackupParentWorkRect;   // Backup of InnerWindow->ParentWorkRect at the end of BeginTable()
    pub HostBackupParentWorkRect: ImRect,
    // ImVec2                      HostBackupPrevLineSize;     // Backup of InnerWindow->DC.PrevLineSize at the end of BeginTable()
    pub HostBackupPrevLineSize: ImVec2,
    // ImVec2                      HostBackupCurrLineSize;     // Backup of InnerWindow->DC.CurrLineSize at the end of BeginTable()
    pub HostBackupCurrLineSize: ImVec2,
    // ImVec2                      HostBackupCursorMaxPos;     // Backup of InnerWindow->DC.CursorMaxPos at the end of BeginTable()
    pub HostBackupCursorMaxPOs: ImVec2,
    // ImVec1                      HostBackupColumnsOffset;    // Backup of OuterWindow->DC.ColumnsOffset at the end of BeginTable()
    pub HostBackupColumnOffset: ImVec1,
    // float                       HostBackupItemWidth;        // Backup of OuterWindow->DC.ItemWidth at the end of BeginTable()
    pub HostBackupItemWidth: f32,
    // int                         HostBackupItemWidthStackSize;//Backup of OuterWindow->DC.ItemWidthStack.Size at the end of BeginTable()
    pub HostBackupItemWidthStackSize: i32,
}

impl ImGuiTableTempData {
    // ImGuiTableTempData()        { memset(this, 0, sizeof(*this)); LastTimeActive = -1.0; }
    pub fn new() -> Self {
        Self {
            LastTimeActive: -1.0,
                ..Default::default()
        }
    }
}

// sizeof() ~ 12
#[derive(Debug,Default,Clone)]
pub struct ImGuiTableColumnSettings
{
    // float                   WidthOrWeight;
    WidthOrWeight: f32,
    // ImGuiID                 UserID;
    pub UserID: ImGuiID,
    // ImGuiTableColumnIdx     Index;
    pub Index: ImGuiTableColumnIdx,
    // ImGuiTableColumnIdx     DisplayOrder;
    pub DisplayOrder: ImGuiTableColumnIdx,
    // ImGuiTableColumnIdx     SortOrder;
    pub SortOrder: ImGuiTableColumnIdx,
    // ImU8                    SortDirection : 2;
    pub SortDirection: ImGuiSortDirection,
    // ImU8                    IsEnabled : 1; // "Visible" in ini file
    pub IsEnabled: u8,
    // ImU8                    IsStretch : 1;
    pub IsStretch: u8,
}

impl ImGuiTableColumnSettings {
    // pub fn ImGuiTableColumnSettings() -> Self
    pub fn new() -> Self
    {
        Self {
            WidthOrWeight: 0.0,
            UserID: 0,
            Index: -1,
            DisplayOrder: -1,
            SortOrder: -1,
            SortDirection: ImGuiSortDirection::None,
            IsEnabled: 1,
            IsStretch: 0,
        }
    }
}

// This is designed to be stored in a single ImChunkStream (1 header followed by N ImGuiTableColumnSettings, etc.)
#[derive(Debug,Clone,Default)]
pub struct ImGuiTableSettings
{
    // ImGuiID                     ID;                     // Set to 0 to invalidate/delete the setting
    pub ID: ImGuiID,
    // ImGuiTableFlags             SaveFlags;              // Indicate data we want to save using the Resizable/Reorderable/Sortable/Hideable flags (could be using its own flags..)
    pub SaveFlags: ImGuiTableFlags,
    // float                       RefScale;               // Reference scale to be able to rescale columns on font/dpi changes.
    pub RefScale: f32,
    // ImGuiTableColumnIdx         ColumnsCount;
    pub ColumnsCount: ImGuiTableColumnIdx,
    // ImGuiTableColumnIdx         ColumnsCountMax;        // Maximum number of columns this settings instance can store, we can recycle a settings instance with lower number of columns but not higher
    pub ColumnsCountMax: ImGuiTableColumnIdx,
    // bool                        WantApply;              // Set when loaded from .ini data (to enable merging/loading .ini data into an already running context)
    pub WantApply: bool,
}

impl ImGuiTableSettings {
    // ImGuiTableSettings()        { memset(this, 0, sizeof(*this)); }
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    //     ImGuiTableColumnSettings*   GetColumnSettings()     { return (ImGuiTableColumnSettings*)(this + 1); }
    pub fn GetColumnSettings(&self) -> *mut Self {
        todo!()
    }
}
