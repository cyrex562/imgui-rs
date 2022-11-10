use crate::core::imvec1::ImVec1;
use crate::layout::layout_type::ImGuiLayoutType;
use crate::nav_layer::ImGuiNavLayer;
use crate::table::old_columns::ImGuiOldColumns;
use crate::core::storage::ImGuiStorage;
use crate::core::type_defs::ImguiHandle;
use crate::core::vec2::Vector2;
use crate::window::ImguiWindow;
use libc::{c_float, c_int, c_short};

// Transient per-window data, reset at the beginning of the frame. This used to be called ImGuiDrawContext, hence the DC variable name in ImGuiWindow.
// (That's theory, in practice the delimitation between ImGuiWindow and ImGuiWindowTempData is quite tenuous and could be reconsidered..)
// (This doesn't need a constructor because we zero-clear it as part of ImGuiWindow and all frame-temporary data are setup on Begin)
#[derive(Default, Debug, Clone)]
pub struct ImGuiWindowTempData {
    // Layout
    pub cursor_pos: Vector2,
    // Current emitting position, in absolute coordinates.
    pub cursor_pos_prev_line: Vector2,
    pub cursor_start_pos: Vector2,
    // Initial position after Begin(), generally ~ window position + WindowPadding.
    pub CursorMaxPos: Vector2,
    // Used to implicitly calculate ContentSize at the beginning of next frame, for scrolling range and auto-resize. Always growing during the frame.
    pub IdealMaxPos: Vector2,
    // Used to implicitly calculate ContentSizeIdeal at the beginning of next frame, for auto-resize only. Always growing during the frame.
    pub curr_line_size: Vector2,
    pub prev_line_size: Vector2,
    pub curr_line_text_base_offset: f32,
    // Baseline offset (0.0 by default on a new line, generally == style.FramePadding.y when a framed item has been added).
    pub prev_line_text_base_offset: f32,
    pub is_same_line: bool,
    pub is_set_pos: bool,
    pub indent: ImVec1,
    // Indentation / start position from left of window (increased by TreePush/TreePop, etc.)
    pub columns_offset: ImVec1,
    // Offset to the current column (if ColumnsCurrent > 0). FIXME: This and the above should be a stack to allow use cases like Tree->column.Tree. Need revamp columns API.
    pub group_offset: ImVec1,
    pub CursorStartPosLossyness: Vector2, // Record the loss of precision of CursorStartPos due to really large scrolling amount. This is used by clipper to compensentate and fix the most common use case of large scroll area.
    // Keyboard/Gamepad navigation
    pub NavLayerCurrent: ImGuiNavLayer,
    // Current layer, 0..31 (we currently only use 0..1)
    pub NavLayersActiveMask: c_short,
    // Which layers have been written to (result from previous frame)
    pub NavLayersActiveMaskNext: c_short,
    // Which layers have been written to (accumulator for current frame)
    pub NavFocusScopeIdCurrent: ImguiHandle,
    // Current focus scope ID while appending
    pub NavHideHighlightOneFrame: bool,
    pub NavHasScroll: bool, // Set when scrolling can be used (ScrollMax > 0.0)
    // Miscellaneous
    pub MenuBarAppending: bool,
    // FIXME: Remove this
    pub MenuBarOffset: Vector2,
    // MenuBarOffset.x is sort of equivalent of a per-layer CursorPos.x, saved/restored as we switch to the menu bar. The only situation when MenuBarOffset.y is > 0 if when (SafeAreaPadding.y > FramePadding.y), often used on TVs.
    pub MenuColumns: ImGuiMenuColumns,
    // Simplified columns storage for menu items measurement
    pub TreeDepth: c_int,
    // Current tree depth.
    pub TreeJumpToParentOnPopMask: u32,
    // Store a copy of !g.NavIdIsAlive for TreeDepth 0..31.. Could be turned into a u64 if necessary.
    pub ChildWindows: Vec<*mut ImguiWindow>,
    pub StateStorage: *mut ImGuiStorage,
    // Current persistent per-window storage (store e.g. tree node open/close state)
    pub current_columns: *mut ImGuiOldColumns,
    // Current columns set
    pub CurrentTableIdx: c_int,
    // Current table index (into g.Tables)
    pub LayoutType: ImGuiLayoutType,
    pub ParentLayoutType: ImGuiLayoutType, // Layout type of parent window at the time of Begin()
    // Local parameters stacks
    // We store the current settings outside of the vectors to increase memory locality (reduce cache misses). The vectors are rarely modified. Also it allows us to not heap allocate for short-lived windows which are not using those settings.
    pub item_width: c_float,
    // Current item width (>0.0: width in pixels, <0.0: align xx pixels to the right of window).
    pub TextWrapPos: c_float,
    // Current text wrap pos.
    pub ItemWidthStack: Vec<c_float>,
    // Store item widths to restore (attention: .back() is not == ItemWidth)
    pub TextWrapPosStack: Vec<c_float>, // Store text wrap pos to restore (attention: .back() is not == TextWrapPos)
}
