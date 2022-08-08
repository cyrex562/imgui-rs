use crate::column::OldColumns;
use crate::input::NavLayer;
use crate::layout::LayoutType;
use crate::menu_columns::MenuColumns;
use crate::types::Id32;
use crate::vectors::Vector1D;
use crate::vectors::vector_2d::Vector2D;

// Transient per-window data, reset at the beginning of the frame. This used to be called ImGuiDrawContext, hence the dc variable name in Window.
// (That's theory, in practice the delimitation between Window and WindowTempData is quite tenuous and could be reconsidered..)
// (This doesn't need a constructor because we zero-clear it as part of Window and all frame-temporary data are setup on Begin)
#[derive(Default,Debug,Clone)]
pub struct WindowTempData {
    // Layout
    // Vector2D                  CursorPos;              // current emitting position, in absolute coordinates.
    pub cursor_pos: Vector2D,
    // Vector2D                  CursorPosPrevLine;
    pub cursor_pos_prev_line: Vector2D,
    // Vector2D                  CursorStartPos;         // Initial position after Begin(), generally ~ window position + window_padding.
    pub cursor_start_pos: Vector2D,
    // Vector2D                  CursorMaxPos;           // Used to implicitly calculate content_size at the beginning of next frame, for scrolling range and auto-resize. Always growing during the frame.
    pub cursor_max_pos: Vector2D,
    // Vector2D                  IdealMaxPos;            // Used to implicitly calculate content_size_ideal at the beginning of next frame, for auto-resize only. Always growing during the frame.
    pub ideal_max_pos: Vector2D,
    // Vector2D                  curr_line_size;
    pub curr_line_size: Vector2D,
    // Vector2D                  prev_line_size;
    pub prev_line_size: Vector2D,
    // float                   curr_line_text_base_offset; // Baseline offset (0.0 by default on a new line, generally == style.FramePadding.y when a framed item has been added).
    pub curr_line_text_base_offset: f32,
    // float                   PrevLineTextBaseOffset;
    pub prev_line_text_base_offset: f32,
    // bool                    Issame_line;
    pub is_same_line: bool,
    // Vector1D                  Indent;                 // Indentation / start position from left of window (increased by TreePush/TreePop, etc.)
    pub indent: Vector1D,
    // Vector1D                  columns_offset;          // Offset to the current column (if ColumnsCurrent > 0). FIXME: This and the above should be a stack to allow use cases like Tree->column->Tree. Need revamp columns API.
    pub columns_offset: Vector1D,
    // Vector1D                  GroupOffset;
    pub group_offset: Vector1D,
    // Vector2D                  CursorStartPosLossyness;// Record the loss of precision of CursorStartPos due to really large scrolling amount. This is used by clipper to compensentate and fix the most common use case of large scroll area.
    pub cursort_start_pos_lossyness: Vector2D,
    // Keyboard/Gamepad navigation
    // ImGuiNavLayer           nav_layer_current;        // current layer, 0..31 (we currently only use 0..1)
    pub nav_layer_current: NavLayer,
    // short                   nav_layers_active_mask;    // Which layers have been written to (result from previous frame)
    pub nav_layers_active_mask: i16,
    // short                   nav_layers_active_mask_next;// Which layers have been written to (accumulator for current frame)
    pub nav_layers_active_mask_next: i16,
    // Id32                 nav_focus_scope_id_current; // current focus scope id while appending
    pub nav_focus_scope_id_current: Id32,
    // bool                    NavHideHighlightOneFrame;
    pub nav_hide_higlight_one_frame: bool,
    // bool                    nav_has_scroll;           // Set when scrolling can be used (scroll_max > 0.0)
    pub nav_has_scroll: bool,
    // Miscellaneous
    // bool                    menu_bar_appending;       // FIXME: Remove this
    pub menu_bar_appending: bool,
    // Vector2D                  menu_bar_offset;          // menu_bar_offset.x is sort of equivalent of a per-layer CursorPos.x, saved/restored as we switch to the menu bar. The only situation when menu_bar_offset.y is > 0 if when (SafeAreaPadding.y > FramePadding.y), often used on TVs.
    pub menu_bar_offset: Vector2D,
    // ImGuiMenuColumns        menu_columns;            // Simplified columns storage for menu items measurement
    pub menu_columns: MenuColumns,
    // int                     tree_depth;              // current tree depth.
    pub tree_depth: i32,
    // ImU32                   tree_jump_to_parent_on_pop_mask; // Store a copy of !g.nav_id_is_alive for tree_depth 0..31.. Could be turned into a ImU64 if necessary.
    pub tree_jump_to_parent_on_pop_mask: u32,
    // ImVector<Window*>  ChildWindows;
    pub child_windows: Vec<Id32>,
    // ImGuiStorage*           state_storage;           // current persistent per-window storage (store e.g. tree node open/close state)
    pub state_storage: Vec<u8>,
    // ImGuiOldColumns*        current_columns;         // current columns set
    pub current_columns: Option<OldColumns>,
    // int                     current_table_idx;        // current table index (into g.tables)
    pub current_table_idx: usize,
    // ImGuiLayoutType         layout_type;
    pub layout_type: LayoutType,
    // ImGuiLayoutType         parent_layout_type;       // Layout type of parent window at the time of Begin()
    pub parent_layout_type: LayoutType,
    // Local parameters stacks
    // We store the current settings outside of the vectors to increase memory locality (reduce cache misses). The vectors are rarely modified. Also it allows us to not heap allocate for short-lived windows which are not using those settings.
    // float                   item_width;              // current item width (>0.0: width in pixels, <0.0: align xx pixels to the right of window).
    pub item_width: f32,
    // float                   text_wrap_pos;            // current text wrap pos.
    pub text_wrap_pos: f32,
    // ImVector<float>         item_width_stack;         // Store item widths to restore (attention: .back() is not == item_width)
    pub item_width_stack: Vec<f32>,
    // ImVector<float>         text_wrap_pos_stack;       // Store text wrap pos to restore (attention: .back() is not == text_wrap_pos)
    pub text_wrap_pos_stack: Vec<f32>,
}
