use libc::{c_float, c_int, c_short};
use crate::layout_type::ImGuiLayoutType;
use crate::old_columns::ImGuiOldColumns;
use crate::storage::ImGuiStorage;
use crate::type_defs::ImGuiID;
use crate::vec2::ImVec2;
use crate::window::ImGuiWindow;

// Transient per-window data, reset at the beginning of the frame. This used to be called ImGuiDrawContext, hence the DC variable name in ImGuiWindow.
// (That's theory, in practice the delimitation between ImGuiWindow and ImGuiWindowTempData is quite tenuous and could be reconsidered..)
// (This doesn't need a constructor because we zero-clear it as part of ImGuiWindow and all frame-temporary data are setup on Begin)
#[derive(Default,Debug,Clone)]
pub struct ImGuiWindowTempData {
    // Layout
    pub CursorPos: ImVec2,
    // Current emitting position, in absolute coordinates.
    pub CursorPosPrevLine: ImVec2,
    pub CursorStartPos: ImVec2,
    // Initial position after Begin(), generally ~ window position + WindowPadding.
    pub CursorMaxPos: ImVec2,
    // Used to implicitly calculate ContentSize at the beginning of next frame, for scrolling range and auto-resize. Always growing during the frame.
    pub IdealMaxPos: ImVec2,
    // Used to implicitly calculate ContentSizeIdeal at the beginning of next frame, for auto-resize only. Always growing during the frame.
    pub CurrLineSize: ImVec2,
    pub PrevLineSize: ImVec2,
    pub CurrLineTextBaseOffset: c_float,
    // Baseline offset (0f32 by default on a new line, generally == style.FramePadding.y when a framed item has been added).
    pub PrevLineTextBaseOffset: c_float,
    pub IsSameLine: bool,
    pub IsSetPos: bool,
    pub Indent: ImVec1,
    // Indentation / start position from left of window (increased by TreePush/TreePop, etc.)
    pub ColumnsOffset: ImVec1,
    // Offset to the current column (if ColumnsCurrent > 0). FIXME: This and the above should be a stack to allow use cases like Tree->column.Tree. Need revamp columns API.
    pub GroupOffset: ImVec1,
    pub CursorStartPosLossyness: ImVec2,// Record the loss of precision of CursorStartPos due to really large scrolling amount. This is used by clipper to compensentate and fix the most common use case of large scroll area.

    // Keyboard/Gamepad navigation
    pub NavLayerCurrent: ImGuiNavLayer,
    // Current layer, 0..31 (we currently only use 0..1)
    pub NavLayersActiveMask: c_short,
    // Which layers have been written to (result from previous frame)
    pub NavLayersActiveMaskNext: c_short,
    // Which layers have been written to (accumulator for current frame)
    pub NavFocusScopeIdCurrent: ImGuiID,
    // Current focus scope ID while appending
    pub NavHideHighlightOneFrame: bool,
    pub NavHasScroll: bool,           // Set when scrolling can be used (ScrollMax > 0f32)

    // Miscellaneous
    pub MenuBarAppending: bool,
    // FIXME: Remove this
    pub MenuBarOffset: ImVec2,
    // MenuBarOffset.x is sort of equivalent of a per-layer CursorPos.x, saved/restored as we switch to the menu bar. The only situation when MenuBarOffset.y is > 0 if when (SafeAreaPadding.y > FramePadding.y), often used on TVs.
    pub MenuColumns: ImGuiMenuColumns,
    // Simplified columns storage for menu items measurement
    pub TreeDepth: c_int,
    // Current tree depth.
    pub TreeJumpToParentOnPopMask: u32,
    // Store a copy of !g.NavIdIsAlive for TreeDepth 0..31.. Could be turned into a u64 if necessary.
    pub ChildWindows: Vec<*mut ImGuiWindow>,
    pub StateStorage: *mut ImGuiStorage,
    // Current persistent per-window storage (store e.g. tree node open/close state)
    pub CurrentColumns: *mut ImGuiOldColumns,
    // Current columns set
    pub CurrentTableIdx: c_int,
    // Current table index (into g.Tables)
    pub LayoutType: ImGuiLayoutType,
    pub ParentLayoutType: ImGuiLayoutType,       // Layout type of parent window at the time of Begin()

    // Local parameters stacks
    // We store the current settings outside of the vectors to increase memory locality (reduce cache misses). The vectors are rarely modified. Also it allows us to not heap allocate for short-lived windows which are not using those settings.
    pub ItemWidth: c_float,
    // Current item width (>0.0: width in pixels, <0.0: align xx pixels to the right of window).
    pub TextWrapPos: c_float,
    // Current text wrap pos.
    pub ItemWidthStack: Vec<c_float>,
    // Store item widths to restore (attention: .back() is not == ItemWidth)
    pub TextWrapPosStack: Vec<c_float>,       // Store text wrap pos to restore (attention: .back() is not == TextWrapPos)
}
