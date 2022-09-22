#![allow(non_snake_case)]


use libc::{c_char, c_short};
use crate::imgui_context::ImGuiContext;
use crate::imgui_storage::ImGuiStorage;
use crate::type_defs::{ImGuiCond, ImGuiDir, ImGuiID, ImGuiItemStatusFlags, ImGuiWindowFlags};

// Storage for one window
pub struct  ImGuiWindow
{
pub Name: *mut c_char,                               // Window name, owned by the window.
pub ID: ImGuiID,                                 // == ImHashStr(Name)
// ImGuiWindowFlags        Flags, FlagsPreviousFrame;          // See enum ImGuiWindowFlags_
pub Flags: ImGuiWindowFlags,
    pub FlagsPreviousFrame: ImGuiWindowFlags,
    pub WindowClass: ImGuiWindowClass,                        // Advanced users only. Set with SetNextWindowClass()
pub Viewport: *mut ImGuiViewportP,                           // Always set in Begin(). Inactive windows may have a NULL value here if their viewport was discarded.
pub ViewportId: ImGuiID,                         // We backup the viewport id (since the viewport may disappear or never be created if the window is inactive)
pub ViewportPos: ImVec2,                        // We backup the viewport position (since the viewport may disappear or never be created if the window is inactive)
pub ViewportAllowPlatformMonitorExtend: int, // Reset to -1 every frame (index is guaranteed to be valid between NewFrame..EndFrame), only used in the Appearing frame of a tooltip/popup to enforce clamping to a given monitor
pub Pos: ImVec2,                                // Position (always rounded-up to nearest pixel)
pub Size: ImVec2,                               // Current size (==SizeFull or collapsed title bar size)
pub SizeFull: ImVec2,                           // Size when non collapsed
pub ContentSize: ImVec2,                        // Size of contents/scrollable client area (calculated from the extents reach of the cursor) from previous frame. Does not include window decoration or window padding.
pub ContentSizeIdeal: ImVec2,
pub ContentSizeExplicit: ImVec2,                // Size of contents/scrollable client area explicitly request by the user via SetNextWindowContentSize().
pub WindowPadding: ImVec2,                      // Window padding at the time of Begin().
pub WindowRounding: float,                     // Window rounding at the time of Begin(). May be clamped lower to avoid rendering artifacts with title bar, menu bar etc.
pub WindowBorderSize: float,                   // Window border size at the time of Begin().
pub NameBufLen: int,                         // Size of buffer storing Name. May be larger than strlen(Name)!
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

pub LastFrameActive: int,                    // Last frame number the window was Active.
pub LastFrameJustFocused: int,               // Last frame number the window was made Focused.
pub LastTimeActive: float,                     // Last timestamp the window was Active (using float as we don't need high precision there)
pub ItemWidthDefault: float,
pub StateStorage: ImGuiStorage,
// ImVector<ImGuiOldColumns> ColumnsStorage;
pub ColumnsStorage: Vec<ImGuiOldColumns>
    pub FontWindowScale: float,                    // User scale multiplier per-window, via SetWindowFontScale()
pub FontDpiScale: float,
pub SettingsOffset: int,                     // Offset into SettingsWindows[] (offsets are always valid as we only grow the array from the back)

pub DrawList: ImDrawList*,                           // == &DrawListInst (for backward compatibility reason with code using imgui_internal.h we keep this a pointer)
pub DrawListInst: ImDrawList,
pub ParentWindow: ImGuiWindow*,                       // If we are a child _or_ popup _or_ docked window, this is pointing to our parent. Otherwise NULL.
pub ParentWindowInBeginStack: ImGuiWindow*,
pub RootWindow: ImGuiWindow*,                         // Point to ourself or first ancestor that is not a child window. Doesn't cross through popups/dock nodes.
pub RootWindowPopupTree: ImGuiWindow*,                // Point to ourself or first ancestor that is not a child window. Cross through popups parent<>child.
pub RootWindowDockTree: ImGuiWindow*,                 // Point to ourself or first ancestor that is not a child window. Cross through dock nodes.
pub RootWindowForTitleBarHighlight: ImGuiWindow*,     // Point to ourself or first ancestor which will display TitleBgActive color when this window is active.
pub RootWindowForNav: ImGuiWindow*,                   // Point to ourself or first ancestor which doesn't have the NavFlattened flag.

pub NavLastChildNavWindow: ImGuiWindow*,              // When going to the menu bar, we remember the child window we came from. (This could probably be made implicit if we kept g.Windows sorted by last focused including child window.)
ImGuiID                 NavLastIds[ImGuiNavLayer_COUNT];    // Last known NavId for this window, per layer (0/1)
ImRect                  NavRectRel[ImGuiNavLayer_COUNT];    // Reference rectangle, in window relative space

pub MemoryDrawListIdxCapacity: int,          // Backup of last idx/vtx count, so when waking up the window we can preallocate and avoid iterative alloc/copy
pub MemoryDrawListVtxCapacity: int,
pub MemoryCompacted: bool,                    // Set when window extraneous data have been garbage collected

// Docking
bool                    DockIsActive        :1;             // When docking artifacts are actually visible. When this is set, DockNode is guaranteed to be != NULL. ~~ (DockNode != NULL) && (DockNode->Windows.Size > 1).
bool                    DockNodeIsVisible   :1;
bool                    DockTabIsVisible    :1;             // Is our window visible this frame? ~~ is the corresponding tab selected?
bool                    DockTabWantClose    :1;
pub DockOrder: c_short,                          // Order of the last time the window was visible within its DockNode. This is used to reorder windows that are reappearing on the same frame. Same value between windows that were active and windows that were none are possible.
pub DockStyle: ImGuiWindowDockStyle,
pub DockNode: ImGuiDockNode*,                           // Which node are we docked into. Important: Prefer testing DockIsActive in many cases as this will still be set when the dock node is hidden.
pub DockNodeAsHost: ImGuiDockNode*,                     // Which node are we owning (for parent windows)
pub DockId: ImGuiID,                             // Backup of last valid DockNode->ID, so single window remember their dock node id even when they are not bound any more
pub DockTabItemStatusFlags: ImGuiItemStatusFlags,
pub DockTabItemRect: ImRect,

public:
ImGuiWindow(ImGuiContext* context, *const c_char name);
~ImGuiWindow();

ImGuiID     GetID(*const c_char str, *const c_char str_end = NULL);
ImGuiID     GetID(const void* ptr);
ImGuiID     GetID(int n);
ImGuiID     GetIDFromRectangle(const ImRect& r_abs);

// We don't use g.FontSize because the window may be != g.CurrentWindow.
ImRect      Rect() const            { return ImRect(Pos.x, Pos.y, Pos.x + Size.x, Pos.y + Size.y); }
float       CalcFontSize() const    { let g = GImGui; // ImGuiContext& g = *GImGui; float scale = g.FontBaseSize * FontWindowScale * FontDpiScale; if (ParentWindow) scale *= Parentwindow.FontWindowScale; return scale; }
float       TitleBarHeight() const  { let g = GImGui; // ImGuiContext& g = *GImGui; return (Flags & ImGuiWindowFlags_NoTitleBar) ? 0f32 : CalcFontSize() + g.Style.FramePadding.y * 2.0f32; }
ImRect      TitleBarRect() const    { return ImRect(Pos, ImVec2(Pos.x + SizeFull.x, Pos.y + TitleBarHeight())); }
float       MenuBarHeight() const   { let g = GImGui; // ImGuiContext& g = *GImGui; return (Flags & ImGuiWindowFlags_MenuBar) ? DC.MenuBarOffset.y + CalcFontSize() + g.Style.FramePadding.y * 2.0f32 : 0f32; }
ImRect      MenuBarRect() const     { float y1 = Pos.y + TitleBarHeight(); return ImRect(Pos.x, y1, Pos.x + SizeFull.x, y1 + MenuBarHeight()); }
};
