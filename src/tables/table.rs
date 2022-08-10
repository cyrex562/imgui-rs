use std::collections::HashSet;
use imgui_rs::draw::list_splitter::DrawListSplitter;
use imgui_rs::types::Id32;
use imgui_rs::types::SortDirection;
use crate::imgui_color::ImColor;
use crate::imgui_h::{ImDrawListSplitter, Id32, SortDirection, TableColumnFlags, ImGuiTableColumnSortSpecs, ImGuiTableFlags, ImGuiTableRowFlags, ImGuiTableSortSpecs};
use crate::imgui_rect::Rect;
use crate::imgui_text_buffer::ImGuiTextBuffer;
use crate::imgui_vec::{Vector1D, Vector2D};
use crate::imgui_window::Window;
use imgui_rs::rect::Rect;
use imgui_rs::text_buffer::TextBuffer;
use imgui_rs::vectors::{Vector1D, Vector2D};

// #define IM_COL32_DISABLE                IM_COL32(0,0,0,1)   // Special sentinel code which cannot be used as a regular color.
pub const IM_COL_32_DISABLE: ImColor = ImColor::new4(0,0,0,1);
// #define IMGUI_TABLE_MAX_COLUMNS         64                  // sizeof * 8. This is solely because we frequently encode columns set in a ImU64.
pub const IMGUI_TABLE_MAX_COLUMNS: usize = 64;
// #define IMGUI_TABLE_MAX_DRAW_CHANNELS   (4 + 64 * 2)        // See TableSetupDrawChannels()
pub const IMGUI_TABLE_MAX_DRAW_CHANNELS: u32 = 4 + 64 *2;

// Our current column maximum is 64 but we may raise that in the future.
// typedef ImS8 ImGuiTableColumnIdx;
pub type TableColumnIdx = i8;
// typedef ImU8 ImGuiTableDrawChannelIdx;
pub type TableDrawChannelIdx = i8;

// [Internal] sizeof() ~ 104
// We use the terminology "Enabled" to refer to a column that is not hidden by user/api.
// We use the terminology "Clipped" to refer to a column that is out of sight because of scrolling/clipping.
// This is in contrast with some user-facing api such as IsItemVisible() / IsRectVisible() which use "visible" to mean "not clipped".
#[derive(Default,Debug,Clone)]
pub struct TableColumn
{
    // ImGuiTableColumnFlags   flags;                          // flags after some patching (not directly same as provided by user). See ImGuiTableColumnFlags_
    pub flags: HashSet<TableColumnFlags>,
    // float                   width_given;                     // Final/actual width visible == (max_x - min_x), locked in TableUpdateLayout(). May be > width_request to honor minimum width, may be < width_request to honor shrinking columns down in tight space.
    pub width_given: f32,
    // float                   min_x;                           // Absolute positions
    pub min_x: f32,
    // float                   max_x;
    pub max_x: f32,
    // float                   width_request;                   // Master width absolute value when !(flags & _WidthStretch). When Stretch this is derived every frame from stretch_weight in TableUpdateLayout()
    pub width_request: f32,
    // float                   width_auto;                      // Automatic width
    pub width_auto: f32,
    // float                   stretch_weight;                  // Master width weight when (flags & _WidthStretch). Often around ~1.0 initially.
    pub stretch_weight: f32,
    // float                   init_stretch_weight_or_width;       // value passed to TableSetupColumn(). For width it is a content width (_without padding_).
    pub init_stretch_weight_or_width: f32,
    // ImRect                  clip_rect;                       // Clipping rectangle for the column
    pub clip_rect: Rect,
    // Id32                 user_id;                         // Optional, value passed to TableSetupColumn()
    pub user_id: Id32,
    // float                   work_min_x;                       // Contents region min ~(min_x + cell_padding_x + cell_spacing_x1) == cursor start position when entering column
    pub work_min_x: f32,
    // float                   work_max_x;                       // Contents region max ~(max_x - cell_padding_x - cell_spacing_x2)
    pub work_max_x: f32,
    // float                   item_width;                      // current item width for the column, preserved across rows
    pub item_width: f32,
    // float                   ContentMaxXFrozen;              // Contents maximum position for frozen rows (apart from headers), from which we can infer content width.
    pub context_max_xfrozen: f32,
    // float                   content_max_xunfrozen;
    pub content_max_xunfrozen: f32,
    // float                   content_max_xheaders_used;         // Contents maximum position for headers rows (regardless of freezing). TableHeader() automatically softclip itself + report ideal desired size, to avoid creating extraneous draw calls
    pub content_max_xheaders_used: f32,
    // float                   content_max_xheaders_ideal;
    pub content_max_xheaders_ideal: f32,
    // ImS16                   name_offset;                     // Offset into parent columns_names[]
    pub name_offset: i16,
    // ImGuiTableColumnIdx     display_order;                   // index within Table's IndexToDisplayOrder[] (column may be reordered by users)
    pub display_order: TableColumnIdx,
    // ImGuiTableColumnIdx     index_within_enabled_set;          // index within enabled/visible set (<= IndexToDisplayOrder)
    pub index_within_enabled_set: TableColumnIdx,
    // ImGuiTableColumnIdx     prev_enabled_column;              // index of prev enabled/visible column within columns[], -1 if first enabled/visible column
    pub prev_enabled_column: TableColumnIdx,
    // ImGuiTableColumnIdx     next_enabled_column;              // index of next enabled/visible column within columns[], -1 if last enabled/visible column
    pub next_enabled_column: TableColumnIdx,
    // ImGuiTableColumnIdx     sort_order;                      // index of this column within sort specs, -1 if not sorting on this column, 0 for single-sort, may be >0 on multi-sort
    pub sort_order: TableColumnIdx,
    // ImGuiTableDrawChannelIdx draw_channel_current;            // index within draw_splitter.Channels[]
    pub draw_channel_current: TableDrawChannelIdx,
    // ImGuiTableDrawChannelIdx draw_channel_frozen;             // Draw channels for frozen rows (often headers)
    pub draw_channel_frozen: TableDrawChannelIdx,
    // ImGuiTableDrawChannelIdx DrawChannelUnfrozen;           // Draw channels for unfrozen rows
    pub draw_channel_un_frozen: TableDrawChannelIdx,
    // bool                    is_enabled;                      // is_user_enabled && (flags & ImGuiTableColumnFlags_Disabled) == 0
    pub is_enabled: bool,
    // bool                    is_user_enabled;                  // Is the column not marked hidden by the user? (unrelated to being off view, e.g. clipped by scrolling).
    pub is_user_enabled: bool,
    // bool                    is_user_enabled_next_frame;
    pub is_user_enabled_next_frame: bool,
    // bool                    is_visible_x;                     // Is actually in view (e.g. overlapping the host window clipping rectangle, not scrolled).
    pub is_visible_x: bool,
    // bool                    is_visible_y;
    pub is_visible_y: bool,
    // bool                    is_request_output;                // Return value for TableSetColumnIndex() / TableNextColumn(): whether we request user to output contents or not.
    pub is_request_output: bool,
    // bool                    is_skip_items;                    // Do we want item submissions to this column to be completely ignored (no layout will happen).
    pub is_skip_items: bool,
    // bool                    IsPreserveWidthAuto;
    pub is_preserve_width_autio: bool,
    // ImS8                    nav_layer_current;                // ImGuiNavLayer in 1 byte
    pub nav_layer_current: i8,
    // ImU8                    auto_fit_queue;                   // Queue of 8 values for the next 8 frames to request auto-fit
    pub auto_fit_queue: i8,
    // ImU8                    CannotSkipItemsQueue;           // Queue of 8 values for the next 8 frames to disable Clipped/SkipItem
    pub cannot_skip_items_queue: i8,
    // ImU8                    sort_direction : 2;              // ImGuiSortDirection_Ascending or ImGuiSortDirection_Descending
    pub sort_direction: SortDirection,
    // ImU8                    sort_directions_avail_count : 2;   // Number of available sort directions (0 to 3)
    pub sort_directions_avail_count: i8,
    // ImU8                    sort_directions_avail_mask : 4;    // Mask of available sort directions (1-bit each)
    pub sort_directions_avail_mask: i8,
    // ImU8                    sort_directions_avail_list;        // Ordered of available sort directions (2-bits each)
    pub sort_directions_avail_list: i8,
}

impl TableColumn {
    // ImGuiTableColumn()
    //     {
    //         memset(this, 0, sizeof(*this));
    //         stretch_weight = width_request = -1.0;
    //         name_offset = -1;
    //         display_order = index_within_enabled_set = -1;
    //         prev_enabled_column = next_enabled_column = -1;
    //         sort_order = -1;
    //         sort_direction = ImGuiSortDirection_None;
    //         draw_channel_current = draw_channel_frozen = DrawChannelUnfrozen = (ImU8)-1;
    //     }
    pub fn new() -> Self {
        Self {
            flags: HashSet::new(),
            width_given: 0.0,
            min_x: 0.0,
            stretch_weight: -1.0,
            init_stretch_weight_or_width: 0.0,
            clip_rect: Rect::default(),
            user_id: 0,
            work_min_x: 0.0,
            work_max_x: 0.0,
            item_width: 0.0,
            context_max_xfrozen: 0.0,
            content_max_xunfrozen: 0.0,
            content_max_xheaders_used: 0.0,
            width_request: -1.0,
            name_offset: -1,
            display_order: -1,
            index_within_enabled_set: -1,
            prev_enabled_column: -1,
            next_enabled_column: -1,
            sort_order: -1,
            sort_direction: SortDirection::None,
            sort_directions_avail_count: 0,
            sort_directions_avail_mask: 0,
            draw_channel_current: -1,
            draw_channel_frozen: -1,
            draw_channel_un_frozen: -1,
            is_enabled: false,
            is_user_enabled: false,
            is_user_enabled_next_frame: false,
            is_visible_x: false,
            is_visible_y: false,
            is_request_output: false,
            is_skip_items: false,
            is_preserve_width_autio: false,
            nav_layer_current: 0,
            auto_fit_queue: 0,
            max_x: 0.0,
            width_auto: 0.0,
            content_max_xheaders_ideal: 0.0,
            cannot_skip_items_queue: 0,
            sort_directions_avail_list: 0
        }
    }
}

// Transient cell data stored per row.
// sizeof() ~ 6
#[derive(Debug,Default,Clone)]
pub struct TableCellData
{
    // ImU32                       bg_color;    // Actual color
    pub bg_color: u32,
    // ImGuiTableColumnIdx         column;     // column number
    pub column: TableColumnIdx,
}

// Per-instance data that needs preserving across frames (seemingly most others do not need to be preserved aside from debug needs, does that needs they could be moved to ImGuiTableTempData ?)
#[derive(Debug,Default,Clone)]
pub struct TableInstanceData
{
    // float                       last_outer_height;            // Outer height from last frame // FIXME: multi-instance issue (#3955)
    pub last_outer_height: f32,
    // float                       last_first_row_height;         // height of first row from last frame // FIXME: possible multi-instance issue?
    pub last_first_row_height: f32,
}

impl TableInstanceData {
    // ImGuiTableInstanceData()    { last_outer_height = last_first_row_height = 0.0; }
    pub fn new() -> Self {
        Self {
            last_outer_height: 0.0,
            last_first_row_height: 0.0,
        }
    }
}

// FIXME-TABLE: more transient data could be stored in a per-stacked table structure: draw_splitter, sort_specs, incoming RowData
#[derive(Debug,Clone,Default)]
pub struct Table
{
    // Id32                     id;
    pub id: Id32,
    // ImGuiTableFlags             flags;
    pub flags: HashSet<TableFlags>,
    // void*                       raw_data;                    // Single allocation to hold columns[], display_order_to_index[] and row_cell_data[]
    pub raw_data: Vec<u8>,
    // ImGuiTableTempData*         temp_data;                   // Transient data while table is active. Point within g.CurrentTableStack[]
    pub temp_data: TableTempData,
    // ImSpan<ImGuiTableColumn>    columns;                    // Point within raw_data[]
    pub columns: Vec<TableColumn>,
    // ImSpan<ImGuiTableColumnIdx> display_order_to_index;        // Point within raw_data[]. Store display order of columns (when not reordered, the values are 0...count-1)
    pub display_order_to_index: Vec<TableColumnIdx>,
    // ImSpan<ImGuiTableCellData>  row_cell_data;                // Point within raw_data[]. Store cells background requests for current row.
    pub row_cell_data: Vec<TableCellData>,
    // ImU64                       enabled_mask_by_display_order;  // column display_order -> is_enabled map
    pub enabled_mask_by_display_order: u64,
    // ImU64                       enabled_mask_by_index;         // column index -> is_enabled map (== not hidden by user/api) in a format adequate for iterating column without touching cold data
    pub enabled_mask_by_index: u64,
    // ImU64                       visible_mask_by_index;         // column index -> is_visible_x|is_visible_y map (== not hidden by user/api && not hidden by scrolling/cliprect)
    pub visible_mask_by_index: u64,
    // ImU64                       request_output_mask_by_index;   // column index -> is_visible || AutoFit (== expect user to submit items)
    pub request_output_mask_by_index: u64,
    // ImGuiTableFlags             settings_loaded_flags;        // Which data were loaded from the .ini file (e.g. when order is not altered we won't save order)
    pub settings_loaded_flags: HashSet<TableFlags>,
    // int                         settings_offset;             // Offset in g.SettingsTables
    pub settings_offset: i32,
    // int                         last_frame_active;
    pub last_frame_active: i32,
    // int                         columns_count;               // Number of columns declared in BeginTable()
    pub columns_count: i32,
    // int                         current_row;
    pub current_row: i32,
    // int                         current_column;
    pub current_column: i32,
    // ImS16                       instance_current;            // count of BeginTable() calls with same id in the same frame (generally 0). This is a little bit similar to begin_count for a window, but multiple table with same id look are multiple tables, they are just synched.
    pub instance_current: i16,
    // ImS16                       instance_interacted;         // Mark which instance (generally 0) of the same id is being interacted with
    pub instance_interacted: i16,
    // float                       row_pos_y1;
    pub row_pos_y1: f32,
    // float                       row_pos_y2;
    pub row_pos_y2: f32,
    // float                       row_min_height;               // height submitted to TableNextRow()
    pub row_min_height: f32,
    // float                       RowTextBaseline;
    pub row_text_base_line: f32,
    // float                       row_indent_offset_x;
    pub row_indent_offset_x: f32,
    // ImGuiTableRowFlags          row_flags : 16;              // current row flags, see ImGuiTableRowFlags_
    pub row_flags: HashSet<TableRowFlags>,
    // ImGuiTableRowFlags          last_row_flags : 16;
    pub last_row_flags: HashSet<TableRowFlags>,
    // int                         row_bg_color_counter;          // Counter for alternating background colors (can be fast-forwarded by e.g clipper), not same as current_row because header rows typically don't increase this.
    pub row_bg_color_counter: i32,
    // ImU32                       row_bg_color[2];              // Background color override for current row.
    pub row_bg_color: [u32;2],
    // ImU32                       border_color_strong;
    pub border_color_strong: u32,
    // ImU32                       border_color_light;
    pub border_color_light: u32,
    // float                       border_x1;
    pub border_x1: f32,
    // float                       border_x2;
    pub border_x2: f32,
    // float                       host_indent_x;
    pub host_indent_x: f32,
    // float                       min_column_width;
    pub min_column_width: f32,
    // float                       outer_padding_x;
    pub outer_padding_x: f32,
    // float                       cell_padding_x;               // Padding from each borders
    pub cell_padding_x: f32,
    // float                       cell_padding_y;
    pub cell_padding_y: f32,
    // float                       cell_spacing_x1;              // spacing between non-bordered cells
    pub cell_spacing_x1: f32,
    // float                       cell_spacing_x2;
    pub cell_spacing_x2: f32,
    // float                       inner_width;                 // User value passed to BeginTable(), see comments at the top of BeginTable() for details.
    pub inner_width: f32,
    // float                       columns_given_width;          // Sum of current column width
    pub columns_given_width: f32,
    // float                       columns_auto_fit_width;        // Sum of ideal column width in order nothing to be clipped, used for auto-fitting and content width submission in outer window
    pub columns_auto_fit_width: f32,
    // float                       columns_stretch_sum_weights;   // Sum of weight of all enabled stretching columns
    pub columns_stretch_sum_weights: f32,
    // float                       ResizedColumnNextWidth;
    pub resize_column_next_width: f32,
    // float                       resize_lock_min_contents_x2;    // Lock minimum contents width while resizing down in order to not create feedback loops. But we allow growing the table.
    pub resize_lock_min_contents_x2: f32,
    // float                       ref_scale;                   // Reference scale to be able to rescale columns on font/dpi changes.
    pub ref_scale: f32,
    // ImRect                      outer_rect;                  // Note: for non-scrolling table, outer_rect.max.y is often FLT_MAX until EndTable(), unless a height has been specified in BeginTable().
    pub outer_rect: Rect,
    // ImRect                      inner_rect;                  // inner_rect but without decoration. As with outer_rect, for non-scrolling tables, inner_rect.max.y is
    pub inner_rect: Rect,
    // ImRect                      work_rect;
    pub work_rect: Rect,
    // ImRect                      inner_clip_rect;
    pub inner_clip_rect: Rect,
    // ImRect                      bg_clip_rect;                 // We use this to cpu-clip cell background color fill, evolve during the frame as we cross frozen rows boundaries
    pub bg_clip_rect: Rect,
    // ImRect                      Bg0ClipRectForDrawCmd;      // Actual ImDrawCmd clip rect for BG0/1 channel. This tends to be == outer_window->clip_rect at BeginTable() because output in BG0/BG1 is cpu-clipped
    pub bg_clip_rect_for_draw_cmd: Rect,
    // ImRect                      bg2clip_rect_for_draw_cmd;      // Actual ImDrawCmd clip rect for BG2 channel. This tends to be a correct, tight-fit, because output to BG2 are done by widgets relying on regular clip_rect.
    pub bg2clip_rect_for_draw_cmd: Rect,
    // ImRect                      host_clip_rect;               // This is used to check if we can eventually merge our columns draw calls into the current draw call of the current window.
    pub host_clip_rect: Rect,
    // ImRect                      host_backup_inner_clip_rect;    // Backup of inner_window->clip_rect during PushTableBackground()/PopTableBackground()
    pub host_backup_inner_clip_rect: Rect,
    // Window*                outer_window;                // Parent window for the table
    pub outer_window: Id32, // *mut Window,
    // Window*                inner_window;                // window holding the table data (== outer_window or a child window)
    pub inner_window: Id32, // *mut Window,
    // ImGuiTextBuffer             columns_names;               // Contiguous buffer holding columns names
    pub columns_names: TextBuffer,
    // ImDrawListSplitter*         draw_splitter;               // Shortcut to temp_data->draw_splitter while in table. Isolate draw commands per columns to avoid switching clip rect constantly
    pub draw_splitter: Id32, // *mut ImDrawListSplitter,
    // ImGuiTableInstanceData      instance_data_first;
    pub instance_data_first: TableInstanceData,
    // ImVector<ImGuiTableInstanceData>    instance_data_extra;  // FIXME-OPT: Using a small-vector pattern would be good.
    pub instance_data_extra: Vec<TableInstanceData>,
    // ImGuiTableColumnSortSpecs   sort_specs_single;
    pub sort_specs_single: TableColumnSortSpecs,
    // ImVector<ImGuiTableColumnSortSpecs> sort_specs_multi;     // FIXME-OPT: Using a small-vector pattern would be good.
    pub sort_specs_multi: Vec<TableColumnSortSpecs>,
    // ImGuiTableSortSpecs         sort_specs;                  // Public facing sorts specs, this is what we return in TableGetSortSpecs()
    pub sort_specs: TableSortSpecs,
    // ImGuiTableColumnIdx         sort_specs_count;
    pub sort_specs_count: TableColumnIdx,
    // ImGuiTableColumnIdx         columns_enabled_count;        // Number of enabled columns (<= columns_count)
    pub columns_enabled_count: TableColumnIdx,
    // ImGuiTableColumnIdx         columns_enabled_fixed_count;   // Number of enabled columns (<= columns_count)
    pub columns_enabled_fixed_count: TableColumnIdx,
    // ImGuiTableColumnIdx         decl_columns_count;           // count calls to TableSetupColumn()
    pub decl_columns_count: TableColumnIdx,
    // ImGuiTableColumnIdx         hovered_column_body;          // index of column whose visible region is being hovered. Important: == columns_count when hovering empty region after the right-most column!
    pub hovered_column_body: TableColumnIdx,
    // ImGuiTableColumnIdx         hovered_column_border;        // index of column whose right-border is being hovered (for resizing).
    pub hovered_column_border: TableColumnIdx,
    // ImGuiTableColumnIdx         auto_fit_single_column;        // index of single column requesting auto-fit.
    pub auto_fit_single_column: TableColumnIdx,
    // ImGuiTableColumnIdx         resized_column;              // index of column being resized. Reset when instance_current==0.
    pub resized_column: TableColumnIdx,
    // ImGuiTableColumnIdx         last_resized_column;          // index of column being resized from previous frame.
    pub last_resized_column: TableColumnIdx,
    // ImGuiTableColumnIdx         held_header_column;           // index of column header being held.
    pub held_header_column: TableColumnIdx,
    // ImGuiTableColumnIdx         reorder_column;              // index of column being reordered. (not cleared)
    pub reorder_column: TableColumnIdx,
    // ImGuiTableColumnIdx         reorder_column_dir;           // -1 or +1
    pub reorder_column_dir: TableColumnIdx,
    // ImGuiTableColumnIdx         left_most_enabled_column;      // index of left-most non-hidden column.
    pub left_most_enabled_column: TableColumnIdx,
    // ImGuiTableColumnIdx         RightMostEnabledColumn;     // index of right-most non-hidden column.
    pub right_mosst_enabled_column: TableColumnIdx,
    // ImGuiTableColumnIdx         left_most_stretched_column;    // index of left-most stretched column.
    pub left_most_stretched_column: TableColumnIdx,
    // ImGuiTableColumnIdx         right_most_stretched_column;   // index of right-most stretched column.
    pub right_most_stretched_column: TableColumnIdx,
    // ImGuiTableColumnIdx         context_popup_column;         // column right-clicked on, of -1 if opening context menu from a neutral/empty spot
    pub context_popup_column: TableColumnIdx,
    // ImGuiTableColumnIdx         freeze_rows_request;          // Requested frozen rows count
    pub freeze_rows_request: TableColumnIdx,
    // ImGuiTableColumnIdx         freeze_rows_count;            // Actual frozen row count (== freeze_rows_request, or == 0 when no scrolling offset)
    pub freeze_rows_count: TableColumnIdx,
    // ImGuiTableColumnIdx         freeze_columns_request;       // Requested frozen columns count
    pub freeze_columns_request: TableColumnIdx,
    // ImGuiTableColumnIdx         freeze_columns_count;         // Actual frozen columns count (== freeze_columns_request, or == 0 when no scrolling offset)
    pub freeze_columns_count: TableColumnIdx,
    // ImGuiTableColumnIdx         row_cell_data_current;         // index of current row_cell_data[] entry in current row
    pub row_cell_data_current: TableColumnIdx,
    // ImGuiTableDrawChannelIdx    dummy_draw_channel;           // Redirect non-visible columns here.
    pub dummy_draw_channel: TableDrawChannelIdx,
    // ImGuiTableDrawChannelIdx    bg2draw_channel_current;      // For selectable() and other widgets drawing across columns after the freezing line. index within draw_splitter.Channels[]
    pub bg2draw_channel_current: TableColumnIdx,
    // ImGuiTableDrawChannelIdx    bg2draw_channel_unfrozen;
    pub bg2draw_channel_unfrozen: TableColumnIdx,
    // bool                        is_layout_locked;             // Set by TableUpdateLayout() which is called when beginning the first row.
    pub is_layout_locked: bool,
    // bool                        is_inside_row;                // Set when inside TableBeginRow()/table_end_row().
    pub is_inside_row: bool,
    // bool                        is_initializing;
    pub is_initializing: bool,
    // bool                        is_sort_specs_dirty;
    pub is_sort_specs_dirty: bool,
    // bool                        is_using_headers;             // Set when the first row had the ImGuiTableRowFlags_Headers flag.
    pub is_using_headers: bool,
    // bool                        is_context_popup_open;         // Set when default context menu is open (also see: context_popup_column, instance_interacted).
    pub is_context_popup_open: bool,
    // bool                        is_settings_request_load;
    pub is_settings_request_load: bool,
    // bool                        is_settings_dirty;            // Set when table settings have changed and needs to be reported into ImGuiTableSetttings data.
    pub is_settings_dirty: bool,
    // bool                        is_default_display_order;      // Set when display order is unchanged from default (display_order contains 0...count-1)
    pub is_default_display_order: bool,
    // bool                        is_reset_all_request;
    pub is_reset_all_request: bool,
    // bool                        is_reset_display_order_request;
    pub is_reset_display_order_request: bool,
    // bool                        is_unfrozen_rows;             // Set when we got past the frozen row.
    pub is_unfrozen_rows: bool,
    // bool                        is_default_sizing_policy;      // Set if user didn't explicitly set a sizing policy in BeginTable()
    pub is_default_sizing_policy: bool,
    // bool                        memory_compacted;
    pub memory_compacted: bool,
    // bool                        host_skip_items;              // Backup of inner_window->SkipItem at the end of BeginTable(), because we will overwrite inner_window->SkipItem on a per-column basis
    pub host_skip_items: bool,
}

impl Table {
    // ImGuiTable()                { memset(this, 0, sizeof(*this)); last_frame_active = -1; }
    pub fn new() -> Self {
        Self {
            last_frame_active: -1,
            ..Default::default()
        }
    }
    //     ~ImGuiTable()               { IM_FREE(raw_data); }
}

// Transient data that are only needed between BeginTable() and EndTable(), those buffers are shared (1 per level of stacked table).
// - Accessing those requires chasing an extra pointer so for very frequently used data we leave them in the main table structure.
// - We also leave out of this structure data that tend to be particularly useful for debugging/metrics.
#[derive(Default,Debug,Clone)]
pub struct TableTempData
{
    // int                         table_index;                 // index in g.tables.Buf[] pool
    pub table_index: i32,
    // float                       last_time_active;             // Last timestamp this structure was used
    pub last_time_active: f32,

    // Vector2D                      user_outer_size;              // outer_size.x passed to BeginTable()
    pub user_outer_size: Vector2D,
    // ImDrawListSplitter          draw_splitter;
    pub draw_splitter: DrawListSplitter,

    // ImRect                      host_backup_work_rect;         // Backup of inner_window->work_rect at the end of BeginTable()
    pub host_backup_work_rect: Rect,
    // ImRect                      host_backup_parent_work_rect;   // Backup of inner_window->parent_work_rect at the end of BeginTable()
    pub host_backup_parent_work_rect: Rect,
    // Vector2D                      host_backupprev_line_size;     // Backup of inner_window->dc.prev_line_size at the end of BeginTable()
    pub host_backupprev_line_size: Vector2D,
    // Vector2D                      hostbackup_curr_line_size;     // Backup of inner_window->dc.curr_line_size at the end of BeginTable()
    pub hostbackup_curr_line_size: Vector2D,
    // Vector2D                      Hostbackup_cursor_max_pos;     // Backup of inner_window->dc.CursorMaxPos at the end of BeginTable()
    pub host_backup_cursor_max_pos: Vector2D,
    // Vector1D                      HostBackupColumnsOffset;    // Backup of outer_window->dc.columns_offset at the end of BeginTable()
    pub host_backup_column_offset: Vector1D,
    // float                       host_backup_item_width;        // Backup of outer_window->dc.item_width at the end of BeginTable()
    pub host_backup_item_width: f32,
    // int                         host_backup_item_width_stack_size;//Backup of outer_window->dc.item_width_stack.size at the end of BeginTable()
    pub host_backup_item_width_stack_size: i32,
}

impl TableTempData {
    // ImGuiTableTempData()        { memset(this, 0, sizeof(*this)); last_time_active = -1.0; }
    pub fn new() -> Self {
        Self {
            last_time_active: -1.0,
                ..Default::default()
        }
    }
}

// sizeof() ~ 12
#[derive(Debug,Default,Clone)]
pub struct TableColumnSettings
{
    // float                   width_or_weight;
    width_or_weight: f32,
    // Id32                 user_id;
    pub user_id: Id32,
    // ImGuiTableColumnIdx     index;
    pub index: TableColumnIdx,
    // ImGuiTableColumnIdx     display_order;
    pub display_order: TableColumnIdx,
    // ImGuiTableColumnIdx     sort_order;
    pub sort_order: TableColumnIdx,
    // ImU8                    sort_direction : 2;
    pub sort_direction: SortDirection,
    // ImU8                    is_enabled : 1; // "visible" in ini file
    pub is_enabled: u8,
    // ImU8                    is_stretch : 1;
    pub is_stretch: u8,
}

impl TableColumnSettings {
    // pub fn ImGuiTableColumnSettings() -> Self
    pub fn new() -> Self
    {
        Self {
            width_or_weight: 0.0,
            user_id: 0,
            index: -1,
            display_order: -1,
            sort_order: -1,
            sort_direction: SortDirection::None,
            is_enabled: 1,
            is_stretch: 0,
        }
    }
}

// This is designed to be stored in a single ImChunkStream (1 header followed by N ImGuiTableColumnSettings, etc.)
#[derive(Debug,Clone,Default)]
pub struct TableSettings
{
    // Id32                     id;                     // Set to 0 to invalidate/delete the setting
    pub id: Id32,
    // ImGuiTableFlags             save_flags;              // Indicate data we want to save using the Resizable/Reorderable/Sortable/Hideable flags (could be using its own flags..)
    pub save_flags: HashSet<TableFlags>,
    // float                       ref_scale;               // Reference scale to be able to rescale columns on font/dpi changes.
    pub ref_scale: f32,
    // ImGuiTableColumnIdx         columns_count;
    pub columns_count: TableColumnIdx,
    // ImGuiTableColumnIdx         columns_count_max;        // Maximum number of columns this settings instance can store, we can recycle a settings instance with lower number of columns but not higher
    pub columns_count_max: TableColumnIdx,
    // bool                        want_apply;              // Set when loaded from .ini data (to enable merging/loading .ini data into an already running context)
    pub want_apply: bool,
}

impl TableSettings {
    // ImGuiTableSettings()        { memset(this, 0, sizeof(*this)); }
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    //     ImGuiTableColumnSettings*   get_column_settings()     { return (ImGuiTableColumnSettings*)(this + 1); }
    pub fn get_column_settings(&self) -> *mut Self {
        todo!()
    }
}

/// Sorting specification for one column of a table (sizeof == 12 bytes)
#[derive(Default,Debug,Clone)]
pub struct TableColumnSortSpecs
{
    pub column_user_id: Id32,     // User id of the column (if specified by a TableSetupColumn() call)
    pub column_index: i16,        // index of the column
    pub sort_order: i16,          // index within parent ImGuiTableSortSpecs (always stored in order starting from 0, tables sorted on a single criteria will always have a 0 here)
    pub sort_direction: SortDirection,  // ImGuiSortDirection_Ascending or ImGuiSortDirection_Descending (you can use this or SortSign, whichever is more convenient for your sort function)

    // ImGuiTableColumnSortSpecs() { memset(this, 0, sizeof(*this)); }
}

/// Sorting specifications for a table (often handling sort specs for a single column, occasionally more)
/// Obtained by calling TableGetSortSpecs().
/// When 'specs_dirty == true' you can sort your data. It will be true with sorting specs have changed since last call, or the first time.
/// Make sure to set 'specs_dirty = false' after sorting, else you may wastefully sort your data every frame!
#[derive(Default,Debug,Clone)]
pub struct TableSortSpecs
{
    pub specs: TableColumnSortSpecs, // const ImGuiTableColumnSortSpecs* specs;     // Pointer to sort spec array.
    pub specs_count: i32,   // Sort spec count. Most often 1. May be > 1 when ImGuiTableFlags_SortMulti is enabled. May be == 0 when ImGuiTableFlags_SortTristate is enabled.
    pub specs_dirty: bool,     // Set to true when specs have changed since last time! Use this to sort again, then clear the flag.

    // ImGuiTableSortSpecs()       { memset(this, 0, sizeof(*this)); }
}


// flags for ImGui::BeginTable()
// - Important! Sizing policies have complex and subtle side effects, much more so than you would expect.
//   Read comments/demos carefully + experiment with live demos to get acquainted with them.
// - The DEFAULT sizing policies are:
//    - Default to ImGuiTableFlags_SizingFixedFit    if scroll_x is on, or if host window has WindowFlags_AlwaysAutoResize.
//    - Default to ImGuiTableFlags_SizingStretchSame if scroll_x is off.
// - When scroll_x is off:
//    - Table defaults to ImGuiTableFlags_SizingStretchSame -> all columns defaults to ImGuiTableColumnFlags_WidthStretch with same weight.
//    - columns sizing policy allowed: Stretch (default), Fixed/Auto.
//    - Fixed columns (if any) will generally obtain their requested width (unless the table cannot fit them all).
//    - Stretch columns will share the remaining width according to their respective weight.
//    - Mixed Fixed/Stretch columns is possible but has various side-effects on resizing behaviors.
//      The typical use of mixing sizing policies is: any number of LEADING Fixed columns, followed by one or two TRAILING Stretch columns.
//      (this is because the visible order of columns have subtle but necessary effects on how they react to manual resizing).
// - When scroll_x is on:
//    - Table defaults to ImGuiTableFlags_SizingFixedFit -> all columns defaults to ImGuiTableColumnFlags_WidthFixed
//    - columns sizing policy allowed: Fixed/Auto mostly.
//    - Fixed columns can be enlarged as needed. Table will show an horizontal scrollbar if needed.
//    - When using auto-resizing (non-resizable) fixed columns, querying the content width to use item right-alignment e.g. SetNextItemWidth(-FLT_MIN) doesn't make sense, would create a feedback loop.
//    - Using Stretch columns OFTEN DOES NOT MAKE SENSE if scroll_x is on, UNLESS you have specified a value for 'inner_width' in BeginTable().
//      If you specify a value for 'inner_width' then effectively the scrolling space is known and Stretch or mixed Fixed/Stretch columns become meaningful again.
// - Read on documentation at the top of imgui_tables.cpp for details.
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum TableFlags
{
    // Features
    None                       = 0,
    Resizable                 ,   // Enable resizing columns.
    Reorderable               ,   // Enable reordering columns in header row (need calling TableSetupColumn() + TableHeadersRow() to display headers)
    Hideable                  ,   // Enable hiding/disabling columns in context menu.
    Sortable                  ,   // Enable sorting. Call TableGetSortSpecs() to obtain sort specs. Also see ImGuiTableFlags_SortMulti and ImGuiTableFlags_SortTristate.
    NoSavedSettings           ,   // Disable persisting columns order, width and sort settings in the .ini file.
    ContextMenuInBody         ,   // Right-click on columns body/contents will display table context menu. By default it is available in TableHeadersRow().
    // Decorations
    RowBg                     ,   // Set each RowBg color with ImGuiCol_TableRowBg or ImGuiCol_TableRowBgAlt (equivalent of calling TableSetBgColor with ImGuiTableBgFlags_RowBg0 on each row manually)
    BordersInnerH             ,   // Draw horizontal borders between rows.
    BordersOuterH             ,   // Draw horizontal borders at the top and bottom.
    BordersInnerV             ,   // Draw vertical borders between columns.
    BordersOuterV             ,  // Draw vertical borders on the left and right sides.
    // ImGuiTableFlags_BordersH                   = ImGuiTableFlags_BordersInnerH | ImGuiTableFlags_BordersOuterH, // Draw horizontal borders.
    // ImGuiTableFlags_BordersV                   = ImGuiTableFlags_BordersInnerV | ImGuiTableFlags_BordersOuterV, // Draw vertical borders.
    // ImGuiTableFlags_BordersInner               = ImGuiTableFlags_BordersInnerV | ImGuiTableFlags_BordersInnerH, // Draw inner borders.
    // ImGuiTableFlags_BordersOuter               = ImGuiTableFlags_BordersOuterV | ImGuiTableFlags_BordersOuterH, // Draw outer borders.
    // ImGuiTableFlags_Borders                    = ImGuiTableFlags_BordersInner | ImGuiTableFlags_BordersOuter,   // Draw all borders.
    NoBordersInBody           ,  // [ALPHA] Disable vertical borders in columns Body (borders will always appears in Headers). -> May move to style
    NoBordersInBodyUntilResize,  // [ALPHA] Disable vertical borders in columns Body until hovered for resize (borders will always appears in Headers). -> May move to style
    // Sizing Policy (read above for defaults)
    SizingFixedFit            ,  // columns default to _WidthFixed or _WidthAuto (if resizable or not resizable), matching contents width.
    SizingFixedSame            = 2 << 13,  // columns default to _WidthFixed or _WidthAuto (if resizable or not resizable), matching the maximum contents width of all columns. Implicitly enable ImGuiTableFlags_NoKeepColumnsVisible.
    SizingStretchProp          = 3 << 13,  // columns default to _WidthStretch with default weights proportional to each columns contents widths.
    SizingStretchSame          = 4 << 13,  // columns default to _WidthStretch with default weights all equal, unless overridden by TableSetupColumn().
    // Sizing Extra Options
    NoHostExtendX             ,  // Make outer width auto-fit to columns, overriding outer_size.x value. Only available when scroll_x/ScrollY are disabled and Stretch columns are not used.
    NoHostExtendY             ,  // Make outer height stop exactly at outer_size.y (prevent auto-extending table past the limit). Only available when scroll_x/ScrollY are disabled. data below the limit will be clipped and not visible.
    NoKeepColumnsVisible      ,  // Disable keeping column always minimally visible when scroll_x is off and table gets too small. Not recommended if columns are resizable.
    PreciseWidths             ,  // Disable distributing remainder width to stretched columns (width allocation on a 100-wide table with 3 columns: Without this flag: 33,33,34. With this flag: 33,33,33). With larger number of columns, resizing will appear to be less smooth.
    // Clipping
    NoClip                    ,  // Disable clipping rectangle for every individual columns (reduce draw command count, items will be able to overflow into other columns). Generally incompatible with TableSetupScrollFreeze().
    // Padding
    PadOuterX                 ,  // Default if BordersOuterV is on. Enable outer-most padding. Generally desirable if you have headers.
    NoPadOuterX               ,  // Default if BordersOuterV is off. Disable outer-most padding.
    NoPadInnerX               ,  // Disable inner padding between columns (double inner padding if BordersOuterV is on, single inner padding if BordersOuterV is off).
    // Scrolling
    ScrollX                   ,  // Enable horizontal scrolling. Require 'outer_size' parameter of BeginTable() to specify the container size. Changes default sizing policy. Because this create a child window, ScrollY is currently generally recommended when using scroll_x.
    ScrollY                   ,  // Enable vertical scrolling. Require 'outer_size' parameter of BeginTable() to specify the container size.
    // Sorting
    SortMulti                 ,  // Hold shift when clicking headers to sort on multiple column. TableGetSortSpecs() may return specs where (specs_count > 1).
    SortTristate              ,  // Allow no sorting, disable default sorting. TableGetSortSpecs() may return specs where (specs_count == 0).

    // [Internal] Combinations and masks
    // ImGuiTableFlags_SizingMask_                = ImGuiTableFlags_SizingFixedFit | ImGuiTableFlags_SizingFixedSame | ImGuiTableFlags_SizingStretchProp | ImGuiTableFlags_SizingStretchSame

    // Obsolete names (will be removed soon)
// #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
//     //, ImGuiTableFlags_ColumnsWidthFixed = ImGuiTableFlags_SizingFixedFit, ImGuiTableFlags_ColumnsWidthStretch = ImGuiTableFlags_SizingStretchSame   // WIP tables 2020/12
//     //, ImGuiTableFlags_SizingPolicyFixed = ImGuiTableFlags_SizingFixedFit, ImGuiTableFlags_SizingPolicyStretch = ImGuiTableFlags_SizingStretchSame   // WIP tables 2021/01
// #endif
}

// pub const     BordersOuter  : i32             = DimgTableFlags::BordersOuterV | DimgTableFlags::BordersOuterH;
pub const BORDERS_OUTER: HashSet<TableFlags> = HashSet::from([
    TableFlags::BordersOuterV, TableFlags::BordersOuterH
]);

// pub const     Borders    : i32                = BordersInner | BordersOuter;
pub const BORDERS: HashSet<TableFlags> = BORDERS_INNER.union(&BORDERS_OUTER).cloned().collect();

// pub const     BordersInner     : i32          = DimgTableFlags::BordersInnerV | DimgTableFlags::BordersInnerH;
pub const BORDERS_INNER: HashSet<TableFlags> = HashSet::from(
    [
        TableFlags::BordersInnerV, TableFlags::BordersInnerH
    ]
);

// pub const     BordersV : i32                  = DimgTableFlags::BordersInnerV | DimgTableFlags::BordersOuterV;
pub const BORDERS_V: HashSet<TableFlags> = HashSet::from([
    TableFlags::BordersInnerV, TableFlags::BordersOuterV
]);


// pub const BordersH: i32                   = DimgTableFlags::BordersInnerH | DimgTableFlags::BordersOuterH;
pub const BORDERS_H: HashSet<TableFlags> = HashSet::from([
    TableFlags::BordersInnerH, TableFlags::BordersOuterH
]) ;

// pub const SizingMask_: i32                 = DimgTableFlags::SizingFixedFit | DimgTableFlags::SizingFixedSame | DimgTableFlags::SizingStretchProp | DimgTableFlags::SizingStretchSame;
pub const SIZING_MASK: HashSet<TableFlags> = HashSet::from([
    TableFlags::SizingFixedFit, TableFlags::SizingFixedSame, TableFlags::SizingStretchProp, TableFlags::SizingStretchSame
]);

// Enum for ImGui::TableSetBgColor()
// Background colors are rendering in 3 layers:
//  - Layer 0: draw with RowBg0 color if set, otherwise draw with ColumnBg0 if set.
//  - Layer 1: draw with RowBg1 color if set, otherwise draw with ColumnBg1 if set.
//  - Layer 2: draw with CellBg color if set.
// The purpose of the two row/columns layers is to let you decide if a background color changes should override or blend with the existing color.
// When using ImGuiTableFlags_RowBg on the table, each row has the RowBg0 color automatically set for odd/even rows.
// If you set the color of RowBg0 target, your color will override the existing RowBg0 color.
// If you set the color of RowBg1 or ColumnBg1 target, your color will blend over the RowBg0 color.
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum TableBgTarget
{
    None                         = 0,
    RowBg0                       = 1,        // Set row background color 0 (generally used for background, automatically set when ImGuiTableFlags_RowBg is used)
    RowBg1                       = 2,        // Set row background color 1 (generally used for selection marking)
    CellBg                       = 3         // Set cell background color (top-most color)
}
