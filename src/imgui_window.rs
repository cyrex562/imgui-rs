#![allow(non_snake_case)]


use libc::{c_char, c_float, c_int, c_short};
use crate::imgui_context::ImGuiContext;
use crate::imgui_dock_node::ImGuiDockNode;
use crate::imgui_drawlist::ImDrawList;
use crate::imgui_old_columns::ImGuiOldColumns;
use crate::imgui_rect::ImRect;
use crate::imgui_storage::ImGuiStorage;
use crate::imgui_vec2::{ImVec2, ImVec2ih};
use crate::imgui_viewport::ImGuiViewport;
use crate::imgui_win_dock_style::ImGuiWindowDockStyle;
use crate::imgui_window_class::ImGuiWindowClass;
use crate::type_defs::{ImGuiCond, ImGuiDir, ImGuiID, ImGuiItemStatusFlags, ImGuiLayoutType, ImGuiWindowFlags};

// Storage for one window
pub struct  ImGuiWindow
{
pub Name: *mut c_char,                               // Window name, owned by the window.
pub ID: ImGuiID,                                 // == ImHashStr(Name)
// ImGuiWindowFlags        Flags, FlagsPreviousFrame;          // See enum ImGuiWindowFlags_
pub Flags: ImGuiWindowFlags,
    pub FlagsPreviousFrame: ImGuiWindowFlags,
    pub WindowClass: ImGuiWindowClass,                        // Advanced users only. Set with SetNextWindowClass()
pub Viewport: *mut ImGuiViewport,                           // Always set in Begin(). Inactive windows may have a NULL value here if their viewport was discarded.
pub ViewportId: ImGuiID,                         // We backup the viewport id (since the viewport may disappear or never be created if the window is inactive)
pub ViewportPos: ImVec2,                        // We backup the viewport position (since the viewport may disappear or never be created if the window is inactive)
pub ViewportAllowPlatformMonitorExtend: c_int, // Reset to -1 every frame (index is guaranteed to be valid between NewFrame..EndFrame), only used in the Appearing frame of a tooltip/popup to enforce clamping to a given monitor
pub Pos: ImVec2,                                // Position (always rounded-up to nearest pixel)
pub Size: ImVec2,                               // Current size (==SizeFull or collapsed title bar size)
pub SizeFull: ImVec2,                           // Size when non collapsed
pub ContentSize: ImVec2,                        // Size of contents/scrollable client area (calculated from the extents reach of the cursor) from previous frame. Does not include window decoration or window padding.
pub ContentSizeIdeal: ImVec2,
pub ContentSizeExplicit: ImVec2,                // Size of contents/scrollable client area explicitly request by the user via SetNextWindowContentSize().
pub WindowPadding: ImVec2,                      // Window padding at the time of Begin().
pub WindowRounding: c_float,                     // Window rounding at the time of Begin(). May be clamped lower to avoid rendering artifacts with title bar, menu bar etc.
pub WindowBorderSize: c_float,                   // Window border size at the time of Begin().
pub NameBufLen: c_int,                         // Size of buffer storing Name. May be larger than strlen(Name)!
pub MoveId: ImGuiID,                             // == window.GetID("#MOVE")
pub TabId: ImGuiID,                              // == window.GetID("#TAB")
pub ChildId: ImGuiID,                            // ID of corresponding item in parent window (for navigation to return from child window to parent window)
pub Scroll: ImVec2,
pub ScrollMax: ImVec2,
pub ScrollTarget: ImVec2,                       // target scroll position. stored as cursor position with scrolling canceled out, so the highest point is always 0f32. (f32::MAX for no change)
pub ScrollTargetCenterRatio: ImVec2,            // 0f32 = scroll so that target position is at top, 0.5f32 = scroll so that target position is centered
pub ScrollTargetEdgeSnapDist: ImVec2,           // 0f32 = no snapping, >0f32 snapping threshold
pub ScrollbarSizes: ImVec2,                     // Size taken by each scrollbars on their smaller axis. Pay attention! ScrollbarSizes.x == width of the vertical scrollbar, ScrollbarSizes.y = height of the horizontal scrollbar.
// bool                    ScrollbarX, ScrollbarY;             // Are scrollbars visible?
pub ScrollbarX: bool,
    pub ScrollbarY: bool,
    pub ViewportOwned: bool,
pub Active: bool,                             // Set to true on Begin(), unless Collapsed
pub WasActive: bool,
pub WriteAccessed: bool,                      // Set to true when any widget access the current window
pub Collapsed: bool,                          // Set when collapsing window to become only title-bar
pub WantCollapseToggle: bool,
pub SkipItems: bool,                          // Set when items can safely be all clipped (e.g. window not visible or collapsed)
pub Appearing: bool,                          // Set during the frame where the window is appearing (or re-appearing)
pub Hidden: bool,                             // Do not display (== HiddenFrames*** > 0)
pub IsFallbackWindow: bool,                   // Set on the "Debug##Default" window.
pub IsExplicitChild: bool,                    // Set when passed _ChildWindow, left to false by BeginDocked()
pub HasCloseButton: bool,                     // Set when the window has a close button (p_open != NULL)
// signed char             ResizeBorderHeld;                   // Current border being held for resize (-1: none, otherwise 0-3)
pub ResizeBorderHeld: i8,
    pub BeginCount: c_short,                         // Number of Begin() during the current frame (generally 0 or 1, 1+ if appending via multiple Begin/End pairs)
pub BeginOrderWithinParent: c_short,             // Begin() order within immediate parent window, if we are a child window. Otherwise 0.
pub BeginOrderWithinContext: c_short,            // Begin() order within entire imgui context. This is mostly used for debugging submission order related issues.
pub FocusOrder: c_short,                         // Order within WindowsFocusOrder[], altered when windows are focused.
pub PopupId: ImGuiID,                            // ID in the popup stack when this window is used as a popup/menu (because we use generic Name/ID for recycling)
// i8                    AutoFitFramesX, AutoFitFramesY;
pub AutoFitFramesX: i8,
    pub AutoFitFramesY: i8,
    pub AutoFitChildAxises: i8,
pub AutoFitOnlyGrows: bool,
pub AutoPosLastDirection: ImGuiDir,
pub HiddenFramesCanSkipItems: i8,           // Hide the window for N frames
pub HiddenFramesCannotSkipItems: i8,        // Hide the window for N frames while allowing items to be submitted so we can measure their size
pub HiddenFramesForRenderOnly: i8,          // Hide the window until frame N at Render() time only
pub DisableInputsFrames: i8,                // Disable window interactions for N frames
// ImGuiCond               SetWindowPosAllowFlags : 8;         // store acceptable condition flags for SetNextWindowPos() use.
pub SetWindowPosAllowFlags: ImGuiCond,
    // ImGuiCond               SetWindowSizeAllowFlags : 8;        // store acceptable condition flags for SetNextWindowSize() use.
pub SetWindowSizeAllowFlags: ImGuiCond,
    // ImGuiCond               SetWindowCollapsedAllowFlags : 8;   // store acceptable condition flags for SetNextWindowCollapsed() use.
pub SetWindowCollapseAllowFlags: ImGuiCond,
    // ImGuiCond               SetWindowDockAllowFlags : 8;        // store acceptable condition flags for SetNextWindowDock() use.
pub SetWindowDockAllowFlags: ImGuiCond,
pub SetWindowPosVal: ImVec2,                    // store window position when using a non-zero Pivot (position set needs to be processed when we know the window size)
pub SetWindowPosPivot: ImVec2,                  // store window pivot for positioning. ImVec2(0, 0) when positioning from top-left corner; ImVec2(0.5f32, 0.5f32) for centering; ImVec2(1, 1) for bottom right.

// ImVector<ImGuiID>       IDStack;                            // ID stack. ID are hashes seeded with the value at the top of the stack. (In theory this should be in the TempData structure)
pub IDStack: Vec<ImGuiID>,
    pub DC: ImGuiWindowTempData,                                 // Temporary per-window data, reset at the beginning of the frame. This used to be called ImGuiDrawContext, hence the "DC" variable name.

// The best way to understand what those rectangles are is to use the 'Metrics->Tools->Show Windows Rectangles' viewer.
// The main 'OuterRect', omitted as a field, is window.Rect().
pub OuterRectClipped: ImRect,                   // == window.Rect() just after setup in Begin(). == window.Rect() for root window.
pub InnerRect: ImRect,                          // Inner rectangle (omit title bar, menu bar, scroll bar)
pub InnerClipRect: ImRect,                      // == InnerRect shrunk by WindowPadding*0.5f32 on each side, clipped within viewport or parent clip rect.
pub WorkRect: ImRect,                           // Initially covers the whole scrolling region. Reduced by containers e.g columns/tables when active. Shrunk by WindowPadding*1f32 on each side. This is meant to replace ContentRegionRect over time (from 1.71+ onward).
pub ParentWorkRect: ImRect,                     // Backup of WorkRect before entering a container such as columns/tables. Used by e.g. SpanAllColumns functions to easily access. Stacked containers are responsible for maintaining this. // FIXME-WORKRECT: Could be a stack?
pub ClipRect: ImRect,                           // Current clipping/scissoring rectangle, evolve as we are using PushClipRect(), etc. == DrawList->clip_rect_stack.back().
pub ContentRegionRect: ImRect,                  // FIXME: This is currently confusing/misleading. It is essentially WorkRect but not handling of scrolling. We currently rely on it as right/bottom aligned sizing operation need some size to rely on.
pub HitTestHoleSize: ImVec2ih,                    // Define an optional rectangular hole where mouse will pass-through the window.
pub HitTestHoleOffset: ImVec2ih,

pub LastFrameActive: c_int,                    // Last frame number the window was Active.
pub LastFrameJustFocused: c_int,               // Last frame number the window was made Focused.
pub LastTimeActive: c_float,                     // Last timestamp the window was Active (using float as we don't need high precision there)
pub ItemWidthDefault: c_float,
pub StateStorage: ImGuiStorage,
// ImVector<ImGuiOldColumns> ColumnsStorage;
pub ColumnsStorage: Vec<ImGuiOldColumns>,
    pub FontWindowScale: c_float,                    // User scale multiplier per-window, via SetWindowFontScale()
pub FontDpiScale: c_float,
pub SettingsOffset: c_int,                     // Offset into SettingsWindows[] (offsets are always valid as we only grow the array from the back)

pub DrawList: *mut ImDrawList,                           // == &DrawListInst (for backward compatibility reason with code using imgui_internal.h we keep this a pointer)
pub DrawListInst: ImDrawList,
pub ParentWindow: *mut ImGuiWindow,                       // If we are a child _or_ popup _or_ docked window, this is pointing to our parent. Otherwise NULL.
pub ParentWindowInBeginStack: *mut ImGuiWindow,
pub RootWindow: *mut ImGuiWindow,                         // Point to ourself or first ancestor that is not a child window. Doesn't cross through popups/dock nodes.
pub RootWindowPopupTree: *mut ImGuiWindow,                // Point to ourself or first ancestor that is not a child window. Cross through popups parent<>child.
pub RootWindowDockTree: *mut ImGuiWindow,                 // Point to ourself or first ancestor that is not a child window. Cross through dock nodes.
pub RootWindowForTitleBarHighlight: *mut ImGuiWindow,     // Point to ourself or first ancestor which will display TitleBgActive color when this window is active.
pub RootWindowForNav: *mut ImGuiWindow,                   // Point to ourself or first ancestor which doesn't have the NavFlattened flag.

pub NavLastChildNavWindow: *mut ImGuiWindow,              // When going to the menu bar, we remember the child window we came from. (This could probably be made implicit if we kept g.Windows sorted by last focused including child window.)
// ImGuiID                 NavLastIds[ImGuiNavLayer_COUNT];    // Last known NavId for this window, per layer (0/1)
pub NavLastIds: [ImGuiID;ImGuiNavLayer_COUNT],
    // ImRect                  NavRectRel[ImGuiNavLayer_COUNT];    // Reference rectangle, in window relative space
pub NavRectRel: [ImRect;ImGuiNavLayer_COUNT],

pub MemoryDrawListIdxCapacity: c_int,          // Backup of last idx/vtx count, so when waking up the window we can preallocate and avoid iterative alloc/copy
pub MemoryDrawListVtxCapacity: c_int,
pub MemoryCompacted: bool,                    // Set when window extraneous data have been garbage collected

// Docking
// bool                    DockIsActive        :1;             // When docking artifacts are actually visible. When this is set, DockNode is guaranteed to be != NULL. ~~ (DockNode != NULL) && (DockNode->Windows.Size > 1).
pub DockIsActive: bool,
    // bool                    DockNodeIsVisible   :1;
pub DockNodeIsVisible: bool,
    // bool                    DockTabIsVisible    :1;             // Is our window visible this frame? ~~ is the corresponding tab selected?
pub DockTabIsVisible: bool,
    // bool                    DockTabWantClose    :1;
pub DockTabWantClose: bool,
    pub DockOrder: c_short,                          // Order of the last time the window was visible within its DockNode. This is used to reorder windows that are reappearing on the same frame. Same value between windows that were active and windows that were none are possible.
pub DockStyle: ImGuiWindowDockStyle,
pub DockNode: * mut ImGuiDockNode,                           // Which node are we docked into. Important: Prefer testing DockIsActive in many cases as this will still be set when the dock node is hidden.
pub DockNodeAsHost: * mut ImGuiDockNode,                     // Which node are we owning (for parent windows)
pub DockId: ImGuiID,                             // Backup of last valid DockNode->ID, so single window remember their dock node id even when they are not bound any more
pub DockTabItemStatusFlags: ImGuiItemStatusFlags,
pub DockTabItemRect: ImRect,

}

impl ImGuiWindow {
    //ImGuiWindow(ImGuiContext* context, *const c_char name);


    //~ImGuiWindow();

    // ImGuiID     GetID(*const c_char str, *const c_char str_end = NULL);


    // ImGuiID     GetID(const void* ptr);


    // ImGuiID     GetID(int n);


    // ImGuiID     GetIDFromRectangle(const ImRect& r_abs);

// We don't use g.FontSize because the window may be != g.CurrentWindow.
//     ImRect      Rect() const            { return ImRect(Pos.x, Pos.y, Pos.x + Size.x, Pos.y + Size.y); }


    // float       CalcFontSize() const    { let g = GImGui; // ImGuiContext& g = *GImGui; float scale = g.FontBaseSize * FontWindowScale * FontDpiScale; if (ParentWindow) scale *= Parentwindow.FontWindowScale; return scale; }


    // float       TitleBarHeight() const  { let g = GImGui; // ImGuiContext& g = *GImGui; return (Flags & ImGuiWindowFlags_NoTitleBar) ? 0f32 : CalcFontSize() + g.Style.FramePadding.y * 2.0f32; }


    // ImRect      TitleBarRect() const    { return ImRect(Pos, ImVec2(Pos.x + SizeFull.x, Pos.y + TitleBarHeight())); }


    // float       MenuBarHeight() const   { let g = GImGui; // ImGuiContext& g = *GImGui; return (Flags & ImGuiWindowFlags_MenuBar) ? DC.MenuBarOffset.y + CalcFontSize() + g.Style.FramePadding.y * 2.0f32 : 0f32; }


    // ImRect      MenuBarRect() const     { float y1 = Pos.y + TitleBarHeight(); return ImRect(Pos.x, y1, Pos.x + SizeFull.x, y1 + MenuBarHeight()); }
}



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
    // Offset to the current column (if ColumnsCurrent > 0). FIXME: This and the above should be a stack to allow use cases like Tree->Column->Tree. Need revamp columns API.
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
    // Store a copy of !g.NavIdIsAlive for TreeDepth 0..31.. Could be turned into a ImU64 if necessary.
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


