use std::os::raw::c_char;
use crate::imgui_column::ImGuiOldColumns;
use crate::imgui_dock::ImGuiDockNode;
use crate::imgui_h::{ImDrawList, ImGuiCond, ImGuiDir, ImGuiID, ImGuiLayoutType, ImGuiMenuColumns, ImGuiNavLayer, ImGuiSizeCallback, ImGuiStackSizes, ImGuiViewport, ImGuiWindowClass, ImGuiWindowFlags};
use crate::imgui_item::{ImGuiItemStatusFlags, ImGuiLastItemData};
use crate::imgui_kv_store::ImGuiStorage;
use crate::imgui_rect::ImRect;
use crate::imgui_vec::{ImVec1, ImVec2};


// Transient per-window data, reset at the beginning of the frame. This used to be called ImGuiDrawContext, hence the DC variable name in ImGuiWindow.
// (That's theory, in practice the delimitation between ImGuiWindow and ImGuiWindowTempData is quite tenuous and could be reconsidered..)
// (This doesn't need a constructor because we zero-clear it as part of ImGuiWindow and all frame-temporary data are setup on Begin)
pub struct  ImGuiWindowTempData
{
    // Layout
    // ImVec2                  CursorPos;              // Current emitting position, in absolute coordinates.
    pub CursorPos: ImVec2,
    // ImVec2                  CursorPosPrevLine;
    pub CursorPosPrevLine: ImVec2,
    // ImVec2                  CursorStartPos;         // Initial position after Begin(), generally ~ window position + WindowPadding.
    pub CursorStartPos: ImVec2,
    // ImVec2                  CursorMaxPos;           // Used to implicitly calculate ContentSize at the beginning of next frame, for scrolling range and auto-resize. Always growing during the frame.
    pub CursorMaxPos: ImVec2,
    // ImVec2                  IdealMaxPos;            // Used to implicitly calculate ContentSizeIdeal at the beginning of next frame, for auto-resize only. Always growing during the frame.
    pub IdealMaxPos: ImVec2,
    // ImVec2                  CurrLineSize;
    pub CurrLineSize: ImVec2,
    // ImVec2                  PrevLineSize;
    pub PrevLineSize: ImVec2,
    // float                   CurrLineTextBaseOffset; // Baseline offset (0.0 by default on a new line, generally == style.FramePadding.y when a framed item has been added).
    pub CurrLineTextBaseOffset: f32,
    // float                   PrevLineTextBaseOffset;
    pub PrevLineTextBaseOffset: f32,
    // bool                    IsSameLine;
    pub IsSameLine: bool,
    // ImVec1                  Indent;                 // Indentation / start position from left of window (increased by TreePush/TreePop, etc.)
    pub Indent: ImVec1,
    // ImVec1                  ColumnsOffset;          // Offset to the current column (if ColumnsCurrent > 0). FIXME: This and the above should be a stack to allow use cases like Tree->Column->Tree. Need revamp columns API.
    pub ColumnsOffset: ImVec1,
    // ImVec1                  GroupOffset;
    pub GroupOffset: ImVec1,
    // ImVec2                  CursorStartPosLossyness;// Record the loss of precision of CursorStartPos due to really large scrolling amount. This is used by clipper to compensentate and fix the most common use case of large scroll area.
    pub CursortStartPosLossyness: ImVec2,
    // Keyboard/Gamepad navigation
    // ImGuiNavLayer           NavLayerCurrent;        // Current layer, 0..31 (we currently only use 0..1)
    pub NavLayerCurrent: ImGuiNavLayer,
    // short                   NavLayersActiveMask;    // Which layers have been written to (result from previous frame)
    pub NavLayersActiveMask: i16,
    // short                   NavLayersActiveMaskNext;// Which layers have been written to (accumulator for current frame)
    pub NavLayersActiveMaskNext: i16,
    // ImGuiID                 NavFocusScopeIdCurrent; // Current focus scope ID while appending
    pub NavFocusScopeIdCurrent: ImGuiID,
    // bool                    NavHideHighlightOneFrame;
    pub NavHideHiglightOneFrame: bool,
    // bool                    NavHasScroll;           // Set when scrolling can be used (ScrollMax > 0.0)
    pub NavHasScroll: bool,
    // Miscellaneous
    // bool                    MenuBarAppending;       // FIXME: Remove this
    pub MenuBarAppending: bool,
    // ImVec2                  MenuBarOffset;          // MenuBarOffset.x is sort of equivalent of a per-layer CursorPos.x, saved/restored as we switch to the menu bar. The only situation when MenuBarOffset.y is > 0 if when (SafeAreaPadding.y > FramePadding.y), often used on TVs.
    pub MenuBarOffset: ImVec2,
    // ImGuiMenuColumns        MenuColumns;            // Simplified columns storage for menu items measurement
    pub MenuColumns: ImGuiMenuColumns,
    // int                     TreeDepth;              // Current tree depth.
    pub TreeDepth: i32,
    // ImU32                   TreeJumpToParentOnPopMask; // Store a copy of !g.NavIdIsAlive for TreeDepth 0..31.. Could be turned into a ImU64 if necessary.
    pub TreeJumpToParentOnPopMask: u32,
    // ImVector<ImGuiWindow*>  ChildWindows;
    pub ChildWindows: Vec<ImGuiWindow>,
    // ImGuiStorage*           StateStorage;           // Current persistent per-window storage (store e.g. tree node open/close state)
    pub StateStorage: *mut ImGuiStorage,
    // ImGuiOldColumns*        CurrentColumns;         // Current columns set
    pub CurrentColumns: *mut ImGuiOldColumns,
    // int                     CurrentTableIdx;        // Current table index (into g.Tables)
    pub CurrentTableIdx: i32,
    // ImGuiLayoutType         LayoutType;
    pub LayoutType: ImGuiLayoutType,
    // ImGuiLayoutType         ParentLayoutType;       // Layout type of parent window at the time of Begin()
    pub ParentLayoutType: ImGuiLayoutType,
    // Local parameters stacks
    // We store the current settings outside of the vectors to increase memory locality (reduce cache misses). The vectors are rarely modified. Also it allows us to not heap allocate for short-lived windows which are not using those settings.
    // float                   ItemWidth;              // Current item width (>0.0: width in pixels, <0.0: align xx pixels to the right of window).
    pub ItemWidth: f32,
    // float                   TextWrapPos;            // Current text wrap pos.
    pub TextWrapPos: f32,
    // ImVector<float>         ItemWidthStack;         // Store item widths to restore (attention: .back() is not == ItemWidth)
    pub ItemWidthStack: Vec<f32>,
    // ImVector<float>         TextWrapPosStack;       // Store text wrap pos to restore (attention: .back() is not == TextWrapPos)
    pub TextWrapPosStack: Vec<f32>,
}


// Storage for one window
pub struct  ImGuiWindow
{
    // char*                   Name;                               // Window name, owned by the window.
    pub Name: *mut c_char,
    // ImGuiID                 ID;                                 // == ImHashStr(Name)
    pub ID: ImGuiID,
    // ImGuiWindowFlags        Flags, FlagsPreviousFrame;          // See enum ImGuiWindowFlags_
    pub Flags: ImGuiWindowFlags,
    pub FlagsPreviousFrame: ImGuiWindowFlags,
    // ImGuiWindowClass        WindowClass;                        // Advanced users only. Set with SetNextWindowClass()
    pub WindowClass: ImGuiWindowClass,
    // ImGuiViewportP*         Viewport;                           // Always set in Begin(). Inactive windows may have a NULL value here if their viewport was discarded.
    pub Viewport: *mut ImGuiViewport,
    // ImGuiID                 ViewportId;                         // We backup the viewport id (since the viewport may disappear or never be created if the window is inactive)
    pub ViewportId: ImGuiID,
    // ImVec2                  ViewportPos;                        // We backup the viewport position (since the viewport may disappear or never be created if the window is inactive)
    pub ViewportPos: ImVec2,
    // int                     ViewportAllowPlatformMonitorExtend; // Reset to -1 every frame (index is guaranteed to be valid between NewFrame..EndFrame), only used in the Appearing frame of a tooltip/popup to enforce clamping to a given monitor
    pub ViewportAllowPlatformMonitorExtend: i32,
    // ImVec2                  Pos;                                // Position (always rounded-up to nearest pixel)
    pub Pos: ImVec2,
    // ImVec2                  Size;                               // Current size (==SizeFull or collapsed title bar size)
    pub Size: ImVec2,
    // ImVec2                  SizeFull;                           // Size when non collapsed
    pub SizeFull: ImVec2,
    // ImVec2                  ContentSize;                        // Size of contents/scrollable client area (calculated from the extents reach of the cursor) from previous frame. Does not include window decoration or window padding.
    pub ContentSize: ImVec2,
    // ImVec2                  ContentSizeIdeal;
    pub ContentSizeIdeal: ImVec2,
    // ImVec2                  ContentSizeExplicit;                // Size of contents/scrollable client area explicitly request by the user via SetNextWindowContentSize().
    pub ContentSizeExplicit: ImVec2,
    // ImVec2                  WindowPadding;                      // Window padding at the time of Begin().
    pub WindowPadding: ImVec2,
    // float                   WindowRounding;                     // Window rounding at the time of Begin(). May be clamped lower to avoid rendering artifacts with title bar, menu bar etc.
    pub WindowRounding: f32,
    // float                   WindowBorderSize;                   // Window border size at the time of Begin().
    // int                     NameBufLen;                         // Size of buffer storing Name. May be larger than strlen(Name)!
    // ImGuiID                 MoveId;                             // == window->GetID("#MOVE")
    pub MoveId: ImGuiID,
    // ImGuiID                 TabId;                              // == window->GetID("#TAB")
    pub TabId: ImGuiID,
    // ImGuiID                 ChildId;                            // ID of corresponding item in parent window (for navigation to return from child window to parent window)
    pub ChildId: ImGuiID,
    // ImVec2                  Scroll;
    pub Scroll: ImVec2,
    // ImVec2                  ScrollMax;
    pub ScrollMax: ImVec2,
    // ImVec2                  ScrollTarget;                       // target scroll position. stored as cursor position with scrolling canceled out, so the highest point is always 0.0. (FLT_MAX for no change)
    pub ScrollTarget: ImVec2,
    // ImVec2                  ScrollTargetCenterRatio;            // 0.0 = scroll so that target position is at top, 0.5 = scroll so that target position is centered
    pub ScrollTargetCenterRatio: ImVec2,
    // ImVec2                  ScrollTargetEdgeSnapDist;           // 0.0 = no snapping, >0.0 snapping threshold
    pub ScrollTargetEdgeSnapDist: ImVec2,
    // ImVec2                  ScrollbarSizes;                     // Size taken by each scrollbars on their smaller axis. Pay attention! ScrollbarSizes.x == width of the vertical scrollbar, ScrollbarSizes.y = height of the horizontal scrollbar.
    pub ScrollbarSizes: ImVec2,
    // bool                    ScrollbarX, ScrollbarY;             // Are scrollbars visible?
    pub ScrollbarX: bool,
    pub ScrollbarY: bool,
    // bool                    ViewportOwned;
    pub ViewportOwned: bool,
    // bool                    Active;                             // Set to true on Begin(), unless Collapsed
    pub Active: bool,
    // bool                    WasActive;
    pub WasActive: bool,
    // bool                    WriteAccessed;                      // Set to true when any widget access the current window
    pub WriteAccessed: bool,
    // bool                    Collapsed;                          // Set when collapsing window to become only title-bar
    pub Collapsed: bool,
    // bool                    WantCollapseToggle;
    pub WantCollapseToggle: bool,
    // bool                    SkipItems;                          // Set when items can safely be all clipped (e.g. window not visible or collapsed)
    pub SkipItems: bool,
    // bool                    Appearing;                          // Set during the frame where the window is appearing (or re-appearing)
    pub Appearing: bool,
    // bool                    Hidden;                             // Do not display (== HiddenFrames*** > 0)
    pub Hidden: bool,
    // bool                    IsFallbackWindow;                   // Set on the "Debug##Default" window.
    pub IsFallbackWindow: bool,
    // bool                    IsExplicitChild;                    // Set when passed _ChildWindow, left to false by BeginDocked()
    pub IsExplicitChild: bool,
    // bool                    HasCloseButton;                     // Set when the window has a close button (p_open != NULL)
    pub HasCloseButton: bool,
    // signed char             ResizeBorderHeld;                   // Current border being held for resize (-1: none, otherwise 0-3)
    pub ResizeBorderHeld: i8,
    // short                   BeginCount;                         // Number of Begin() during the current frame (generally 0 or 1, 1+ if appending via multiple Begin/End pairs)
    pub BeginCount: i16,
    // short                   BeginOrderWithinParent;             // Begin() order within immediate parent window, if we are a child window. Otherwise 0.
    pub BeginOrderWithinParent: i16,
    // short                   BeginOrderWithinContext;            // Begin() order within entire imgui context. This is mostly used for debugging submission order related issues.
    pub BeginOrderWithinContext: i16,
    // short                   FocusOrder;                         // Order within WindowsFocusOrder[], altered when windows are focused.
    pub FocusOrder: i16,
    // ImGuiID                 PopupId;                            // ID in the popup stack when this window is used as a popup/menu (because we use generic Name/ID for recycling)
    pub PopupId: ImGuiID,
    // ImS8                    AutoFitFramesX, AutoFitFramesY;
    pub AutoFitFramesX: i8,
    pub AutoFitFramesY: i8,
    // ImS8                    AutoFitChildAxises;
    pub AutoFitChildAxises: i8,
    // bool                    AutoFitOnlyGrows;
    pub AutoFitOnlyGrows: bool,
    // ImGuiDir                AutoPosLastDirection;
    pub AutoPosLastDirection: ImGuiDir,
    // ImS8                    HiddenFramesCanSkipItems;           // Hide the window for N frames
    pub HiddenFramesCanSkipItems: i8,
    // ImS8                    HiddenFramesCannotSkipItems;        // Hide the window for N frames while allowing items to be submitted so we can measure their size
    pub HiddenFramesCannotSkipItems: i8,
    // ImS8                    HiddenFramesForRenderOnly;          // Hide the window until frame N at Render() time only
    pub HiddenFramesForRenderOnly: i8,
    // ImS8                    DisableInputsFrames;                // Disable window interactions for N frames
    pub DisableInputsFrames: i8,
    // ImGuiCond               SetWindowPosAllowFlags : 8;         // store acceptable condition flags for SetNextWindowPos() use.
    pub SetWindowPosAllowFlags: ImGuiCond,
    // ImGuiCond               SetWindowSizeAllowFlags : 8;        // store acceptable condition flags for SetNextWindowSize() use.
    pub SetWindowSizeAllowFlags: ImGuiCond,
    // ImGuiCond               SetWindowCollapsedAllowFlags : 8;   // store acceptable condition flags for SetNextWindowCollapsed() use.
    pub SetWindowCollapsedAllowFlags: ImGuiCond,
    // ImGuiCond               SetWindowDockAllowFlags : 8;        // store acceptable condition flags for SetNextWindowDock() use.
    // ImVec2                  SetWindowPosVal;                    // store window position when using a non-zero Pivot (position set needs to be processed when we know the window size)
    pub SetWindowPosVal: ImVec2,
    // ImVec2                  SetWindowPosPivot;                  // store window pivot for positioning. ImVec2(0, 0) when positioning from top-left corner; ImVec2(0.5, 0.5) for centering; ImVec2(1, 1) for bottom right.
    pub SetWindowPosPivot: ImVec2,

    // ImVector<ImGuiID>       IDStack;                            // ID stack. ID are hashes seeded with the value at the top of the stack. (In theory this should be in the TempData structure)
    pub IDStack: Vec<ImGuiID>,
    // ImGuiWindowTempData     DC;                                 // Temporary per-window data, reset at the beginning of the frame. This used to be called ImGuiDrawContext, hence the "DC" variable name.
    pub DC: ImGuiWindowTempData,

    // The best way to understand what those rectangles are is to use the 'Metrics->Tools->Show Windows Rectangles' viewer.
    // The main 'OuterRect', omitted as a field, is window->Rect().
    // ImRect                  OuterRectClipped;                   // == Window->Rect() just after setup in Begin(). == window->Rect() for root window.
    pub OuterRectClipped: ImRect,
    // ImRect                  InnerRect;                          // Inner rectangle (omit title bar, menu bar, scroll bar)
    pub InnerRect: ImRect,
    // ImRect                  InnerClipRect;                      // == InnerRect shrunk by WindowPadding*0.5 on each side, clipped within viewport or parent clip rect.
    pub InnerClipRect: ImRect,
    // ImRect                  WorkRect;                           // Initially covers the whole scrolling region. Reduced by containers e.g columns/tables when active. Shrunk by WindowPadding*1.0 on each side. This is meant to replace ContentRegionRect over time (from 1.71+ onward).
    pub WorkRect: ImRect,
    // ImRect                  ParentWorkRect;                     // Backup of WorkRect before entering a container such as columns/tables. Used by e.g. SpanAllColumns functions to easily access. Stacked containers are responsible for maintaining this. // FIXME-WORKRECT: Could be a stack?
    pub ParentWorkRect: ImRect,
    // ImRect                  ClipRect;                           // Current clipping/scissoring rectangle, evolve as we are using PushClipRect(), etc. == DrawList->clip_rect_stack.back().
    pub ClipRect: ImRect,
    // ImRect                  ContentRegionRect;                  // FIXME: This is currently confusing/misleading. It is essentially WorkRect but not handling of scrolling. We currently rely on it as right/bottom aligned sizing operation need some size to rely on.
    pub ContentRegionRect: ImRect,
    // ImVec2ih                HitTestHoleSize;                    // Define an optional rectangular hole where mouse will pass-through the window.
    pub HitTestHoleSize: ImVec2,
    // ImVec2ih                HitTestHoleOffset;
    pub HitTestHoleOffset: ImVec2,
    // int                     LastFrameActive;                    // Last frame number the window was Active.
    pub LastFrameActive: i32,
    // int                     LastFrameJustFocused;               // Last frame number the window was made Focused.
    pub LastFrameJustFocused: i32,
    // float                   LastTimeActive;                     // Last timestamp the window was Active (using float as we don't need high precision there)
    pub LastTimeActive: f32,
    // float                   ItemWidthDefault;
    pub ItemWidthDefault: f32,
    // ImGuiStorage            StateStorage;
    pub StateStorage: ImGuiStorage,
    // ImVector<ImGuiOldColumns> ColumnsStorage;
    pub Column: Vec<ImGuiOldColumns>,
    // float                   FontWindowScale;                    // User scale multiplier per-window, via SetWindowFontScale()
    pub FontWindowScale: f32,
    // float                   FontDpiScale;
    pub FontDpiScale: f32,
    // int                     SettingsOffset;                     // Offset into SettingsWindows[] (offsets are always valid as we only grow the array from the back)
    pub SettingsOffset: i32,
    // ImDrawList*             DrawList;                           // == &DrawListInst (for backward compatibility reason with code using imgui_internal.h we keep this a pointer)
    pub DrawList: *mut ImDrawList,
    // ImDrawList              DrawListInst;
    pub DrawListInst: ImDrawList,
    // ImGuiWindow*            ParentWindow;                       // If we are a child _or_ popup _or_ docked window, this is pointing to our parent. Otherwise NULL.
    pub ParentWindow: *mut ImGuiWindow,
    // ImGuiWindow*            ParentWindowInBeginStack;
    pub ParentWindowInBeginStack: *mut ImGuiWindow,
    // ImGuiWindow*            RootWindow;                         // Point to ourself or first ancestor that is not a child window. Doesn't cross through popups/dock nodes.
    pub RootWindow: *mut ImGuiWindow,
    // ImGuiWindow*            RootWindowPopupTree;                // Point to ourself or first ancestor that is not a child window. Cross through popups parent<>child.
    pub RootWindowPopupTree: *mut ImGuiWindow,
    // ImGuiWindow*            RootWindowDockTree;                 // Point to ourself or first ancestor that is not a child window. Cross through dock nodes.
    pub RootWindowDockTree: *mut ImGuiWindow,
    // ImGuiWindow*            RootWindowForTitleBarHighlight;     // Point to ourself or first ancestor which will display TitleBgActive color when this window is active.
    pub RootWindowForTitleBarHighlight: *mut ImGuiWindow,
    // ImGuiWindow*            RootWindowForNav;                   // Point to ourself or first ancestor which doesn't have the NavFlattened flag.
    pub RootWindowForNav: *mut ImGuiWindow,
    // ImGuiWindow*            NavLastChildNavWindow;              // When going to the menu bar, we remember the child window we came from. (This could probably be made implicit if we kept g.Windows sorted by last focused including child window.)
    pub NavLastChildNavWindow: *mut ImGuiWindow,
    // ImGuiID                 NavLastIds[ImGuiNavLayer_COUNT];    // Last known NavId for this window, per layer (0/1)
    pub NavLastIds: Vec<ImGuiID>,
    // ImRect                  NavRectRel[ImGuiNavLayer_COUNT];    // Reference rectangle, in window relative space
    pub NavRectRel: Vec<ImRect>,
    // int                     MemoryDrawListIdxCapacity;          // Backup of last idx/vtx count, so when waking up the window we can preallocate and avoid iterative alloc/copy
    pub MemoryDrawListIdxCapacity: i32,
    // int                     MemoryDrawListVtxCapacity;
    pub MemoryDrawListVtxCapacity: i32,
    // bool                    MemoryCompacted;                    // Set when window extraneous data have been garbage collected
    pub MemoryCompacted: bool,
    // Docking
    // bool                    DockIsActive        :1;             // When docking artifacts are actually visible. When this is set, DockNode is guaranteed to be != NULL. ~~ (DockNode != NULL) && (DockNode->Windows.Size > 1).
    pub DockIsActive: bool,
    // bool                    DockNodeIsVisible   :1;
    pub DocNodeIsVisible: bool,
    // bool                    DockTabIsVisible    :1;             // Is our window visible this frame? ~~ is the corresponding tab selected?
    pub DockTabIsVisible: bool,
    // bool                    DockTabWantClose    :1;
    pub DockTabWantClose: bool,
    // short                   DockOrder;                          // Order of the last time the window was visible within its DockNode. This is used to reorder windows that are reappearing on the same frame. Same value between windows that were active and windows that were none are possible.
    pub DockOrder: i16,
    // ImGuiWindowDockStyle    DockStyle;
    pub DockStyle: ImGuiWindowDockStyle,
    // ImGuiDockNode*          DockNode;                           // Which node are we docked into. Important: Prefer testing DockIsActive in many cases as this will still be set when the dock node is hidden.
    pub DockNode: *mut ImGuiDockNode,
    // ImGuiDockNode*          DockNodeAsHost;                     // Which node are we owning (for parent windows)
    pub DockNodeAsHost: *mut ImGuiDockNode,
    // ImGuiID                 DockId;                             // Backup of last valid DockNode->ID, so single window remember their dock node id even when they are not bound any more
    pub DockId: ImGuiID,
    // ImGuiItemStatusFlags    DockTabItemStatusFlags;
    pub DockTabItemStatusFlags: ImGuiItemStatusFlags,
    // ImRect                  DockTabItemRect;
    pub DockTabItemRect: ImRect,

}

#[derive(Debug,Clone,Default)]
pub struct ImGuiWindowDockStyle
{
    // ImU32 Colors[ImGuiWindowDockStyleCol_COUNT];
    pub Colors: Vec<u32>,
}

// Data saved for each window pushed into the stack
#[derive(Debug,Clone,Default)]
pub struct ImGuiWindowStackData
{
    // ImGuiWindow*            Window;
    pub Window: *mut ImGuiWindow,
    // ImGuiLastItemData       ParentLastItemDataBackup;
    pub ParentLastItemDataBackup: ImGuiLastItemData,
    // ImGuiStackSizes         StackSizesOnBegin;      // Store size of various stacks for asserting
    pub StackSizesOnBegin: ImGuiStackSizes,
}


// Transient per-window flags, reset at the beginning of the frame. For child window, inherited from parent on first Begin().
// This is going to be exposed in imgui.h when stabilized enough.
pub enum ImGuiItemFlags
{
    None                     = 0,
    NoTabStop                = 1 << 0,  // false     // Disable keyboard tabbing (FIXME: should merge with _NoNav)
    ButtonRepeat             = 1 << 1,  // false     // Button() will return true multiple times based on io.KeyRepeatDelay and io.KeyRepeatRate settings.
    Disabled                 = 1 << 2,  // false     // Disable interactions but doesn't affect visuals. See BeginDisabled()/EndDisabled(). See github.com/ocornut/imgui/issues/211
    NoNav                    = 1 << 3,  // false     // Disable keyboard/gamepad directional navigation (FIXME: should merge with _NoTabStop)
    NoNavDefaultFocus        = 1 << 4,  // false     // Disable item being a candidate for default focus (e.g. used by title bar items)
    SelectableDontClosePopup = 1 << 5,  // false     // Disable MenuItem/Selectable() automatically closing their popup window
    MixedValue               = 1 << 6,  // false     // [BETA] Represent a mixed/indeterminate value, generally multi-selection where values differ. Currently only supported by Checkbox() (later should support all sorts of widgets)
    ReadOnly                 = 1 << 7,  // false     // [ALPHA] Allow hovering interactions but underlying value is not changed.
    Inputable                = 1 << 8   // false     // [WIP] Auto-activate input mode when tab focused. Currently only used and supported by a few items before it becomes a generic feature.
}


// Storage for SetNexWindow** functions
#[derive(Debug,Clone,Default)]
pub struct ImGuiNextWindowData
{
    // ImGuiNextWindowDataFlags    Flags;
    pub Flags: ImGuiNextWindowDataFlags,
    // ImGuiCond                   PosCond;
    pub PosCond: ImGuiCond,
    // ImGuiCond                   SizeCond;
    pub SizeCond: ImGuiCond,
    // ImGuiCond                   CollapsedCond;
    pub CollapseCond: ImGuiCond,
    // ImGuiCond                   DockCond;
    pub DockCond: ImGuiCond,
    // ImVec2                      PosVal;
    pub PosVal: ImVec2,
    // ImVec2                      PosPivotVal;
    pub PosPivotVal: ImVec2,
    // ImVec2                      SizeVal;
    pub SizeVal: ImVec2,
    // ImVec2                      ContentSizeVal;
    pub ContentSizeVal: ImVec2,
    // ImVec2                      ScrollVal;
    pub ScrollVal: ImVec2,
    // bool                        PosUndock;
    pub PosUndock: bool,
    // bool                        CollapsedVal;
    pub CollapsedVal: bool,
    // ImRect                      SizeConstraintRect;
    pub SizeConstraintRect: ImRect,
    // ImGuiSizeCallback           SizeCallback;
    pub SizeCallback: ImGuiSizeCallback,
    // void*                       SizeCallbackUserData;
    pub SizeCallbackUserData: Vec<u8>,
    // float                       BgAlphaVal;             // Override background alpha
    pub BgAlphaVal: f32,
    // ImGuiID                     ViewportId;
    pub ViewportId: ImGuiID,
    // ImGuiID                     DockId;
    pub DockId: ImGuiID,
    // ImGuiWindowClass            WindowClass;
    pub WindowClass: ImGuiWindowClass,
    // ImVec2                      MenuBarOffsetMinVal;    // (Always on) This is not exposed publicly, so we don't clear it and it doesn't have a corresponding flag (could we? for consistency?)
    pub MenuBarOffsetMinVal: ImVec2,

}

impl ImGuiNextWindowData{
    // ImGuiNextWindowData()       { memset(this, 0, sizeof(*this)); }
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    //     inline void ClearFlags()    { Flags = ImGuiNextWindowDataFlags_None; }
    pub fn ClearFlags(&mut self) {
        self.Flags = ImGuiNextWindowDataFlags::None
    }
}


pub enum ImGuiNextWindowDataFlags
{
    None               = 0,
    HasPos             = 1 << 0,
    HasSize            = 1 << 1,
    HasContentSize     = 1 << 2,
    HasCollapsed       = 1 << 3,
    HasSizeConstraint  = 1 << 4,
    HasFocus           = 1 << 5,
    HasBgAlpha         = 1 << 6,
    HasScroll          = 1 << 7,
    HasViewport        = 1 << 8,
    HasDock            = 1 << 9,
    HasWindowClass     = 1 << 10
}




