#![allow(non_snake_case)]


use std::mem;
use std::ptr::null_mut;
use libc::{c_char, c_float, c_int, c_short, c_void};
use crate::condition::{ImGuiCond, ImGuiCond_Always, ImGuiCond_Appearing, ImGuiCond_FirstUseEver, ImGuiCond_Once};
use crate::context::ImGuiContext;
use crate::data_type::{ImGuiDataType_Pointer, ImGuiDataType_S32, ImGuiDataType_String};
use crate::debug_ops::DebugHookIdInfo;
use crate::direction::{ImGuiDir, ImGuiDir_None};
use crate::dock_node::ImGuiDockNode;
use crate::draw_list::ImDrawList;
use crate::hash_ops::{ImHashData, ImHashStr};
use crate::imgui::GImGui;
use crate::item_status_flags::ImGuiItemStatusFlags;
use crate::layout_type::ImGuiLayoutType;
use crate::nav_layer::ImGuiNavLayer_COUNT;
use crate::old_columns::ImGuiOldColumns;
use crate::rect::ImRect;
use crate::stack_sizes::ImGuiStackSizes;
use crate::storage::ImGuiStorage;
use crate::string_ops::ImStrdup;
use crate::vec2::{ImVec2, ImVec2ih};
use crate::viewport::ImGuiViewport;
use crate::win_dock_style::ImGuiWindowDockStyle;
use crate::window_class::ImGuiWindowClass;
use crate::type_defs::{ImGuiDir, ImGuiID};
use crate::vec4::ImVec4;
use crate::window::window_class::ImGuiWindowClass;
use crate::window::window_flags::ImGuiWindowFlags;
use rect::WindowRectAbsToRel;
use crate::window::window_temp_data::ImGuiWindowTempData;
use crate::window_flags::ImGuiWindowFlags;
use crate::window_ops::WindowRectAbsToRel;
use crate::window_temp_data::ImGuiWindowTempData;

pub mod window_class;
pub mod window_stack_data;
pub mod window_settings;
pub mod window_flags;
pub mod ops;
pub mod window_temp_data;
pub mod window_dock_style_colors;
pub mod window_dock_style_color;
pub mod render;
pub mod rect;
pub mod find;
pub mod focus;
pub mod props;
mod stb_tt_point;

// Storage for one window
pub struct ImGuiWindow {
    pub Name: *mut c_char,
    // Window name, owned by the window.
    pub ID: ImGuiID,
    // == ImHashStr(Name)
// ImGuiWindowFlags        Flags, FlagsPreviousFrame;          // See enum ImGuiWindowFlags_
    pub Flags: ImGuiWindowFlags,
    pub FlagsPreviousFrame: ImGuiWindowFlags,
    pub WindowClass: ImGuiWindowClass,
    // Advanced users only. Set with SetNextWindowClass()
    pub Viewport: *mut ImGuiViewport,
    // Always set in Begin(). Inactive windows may have a NULL value here if their viewport was discarded.
    pub ViewportId: ImGuiID,
    // We backup the viewport id (since the viewport may disappear or never be created if the window is inactive)
    pub ViewportPos: ImVec2,
    // We backup the viewport position (since the viewport may disappear or never be created if the window is inactive)
    pub ViewportAllowPlatformMonitorExtend: c_int,
    // Reset to -1 every frame (index is guaranteed to be valid between NewFrame..EndFrame), only used in the Appearing frame of a tooltip/popup to enforce clamping to a given monitor
    pub Pos: ImVec2,
    // Position (always rounded-up to nearest pixel)
    pub Size: ImVec2,
    // Current size (==SizeFull or collapsed title bar size)
    pub SizeFull: ImVec2,
    // Size when non collapsed
    pub ContentSize: ImVec2,
    // Size of contents/scrollable client area (calculated from the extents reach of the cursor) from previous frame. Does not include window decoration or window padding.
    pub ContentSizeIdeal: ImVec2,
    pub ContentSizeExplicit: ImVec2,
    // Size of contents/scrollable client area explicitly request by the user via SetNextWindowContentSize().
    pub WindowPadding: ImVec2,
    // Window padding at the time of Begin().
    pub WindowRounding: c_float,
    // Window rounding at the time of Begin(). May be clamped lower to avoid rendering artifacts with title bar, menu bar etc.
    pub WindowBorderSize: c_float,
    // Window border size at the time of Begin().
    pub NameBufLen: usize,
    // Size of buffer storing Name. May be larger than strlen(Name)!
    pub MoveId: ImGuiID,
    // == window.GetID("#MOVE")
    pub TabId: ImGuiID,
    // == window.GetID("#TAB")
    pub ChildId: ImGuiID,
    // ID of corresponding item in parent window (for navigation to return from child window to parent window)
    pub Scroll: ImVec2,
    pub ScrollMax: ImVec2,
    pub ScrollTarget: ImVec2,
    // target scroll position. stored as cursor position with scrolling canceled out, so the highest point is always 0.0. (f32::MAX for no change)
    pub ScrollTargetCenterRatio: ImVec2,
    // 0.0 = scroll so that target position is at top, 0.5 = scroll so that target position is centered
    pub ScrollTargetEdgeSnapDist: ImVec2,
    // 0.0 = no snapping, >0.0 snapping threshold
    pub ScrollbarSizes: ImVec2,
    // Size taken by each scrollbars on their smaller axis. Pay attention! ScrollbarSizes.x == width of the vertical scrollbar, ScrollbarSizes.y = height of the horizontal scrollbar.
// bool                    ScrollbarX, ScrollbarY;             // Are scrollbars visible?
    pub ScrollbarX: bool,
    pub ScrollbarY: bool,
    pub ViewportOwned: bool,
    pub Active: bool,
    // Set to true on Begin(), unless Collapsed
    pub WasActive: bool,
    pub WriteAccessed: bool,
    // Set to true when any widget access the current window
    pub Collapsed: bool,
    // Set when collapsing window to become only title-bar
    pub WantCollapseToggle: bool,
    pub SkipItems: bool,
    // Set when items can safely be all clipped (e.g. window not visible or collapsed)
    pub Appearing: bool,
    // Set during the frame where the window is appearing (or re-appearing)
    pub Hidden: bool,
    // Do not display (== HiddenFrames*** > 0)
    pub IsFallbackWindow: bool,
    // Set on the "Debug##Default" window.
    pub IsExplicitChild: bool,
    // Set when passed _ChildWindow, left to false by BeginDocked()
    pub HasCloseButton: bool,
    // Set when the window has a close button (p_open != NULL)
// signed char             ResizeBorderHeld;                   // Current border being held for resize (-1: none, otherwise 0-3)
    pub ResizeBorderHeld: i8,
    pub BeginCount: i32,
    // Number of Begin() during the current frame (generally 0 or 1, 1+ if appending via multiple Begin/End pairs)
    pub BeginOrderWithinParent: i32,
    // Begin() order within immediate parent window, if we are a child window. Otherwise 0.
    pub BeginOrderWithinContext: i32,
    // Begin() order within entire imgui context. This is mostly used for debugging submission order related issues.
    pub FocusOrder: i32,
    // Order within WindowsFocusOrder[], altered when windows are focused.
    pub PopupId: ImGuiID,
    // ID in the popup stack when this window is used as a popup/menu (because we use generic Name/ID for recycling)
// i8                    AutoFitFramesX, AutoFitFramesY;
    pub AutoFitFramesX: i8,
    pub AutoFitFramesY: i8,
    pub AutoFitChildAxises: c_int,
    pub AutoFitOnlyGrows: bool,
    pub AutoPosLastDirection: ImGuiDir,
    pub HiddenFramesCanSkipItems: i8,
    // Hide the window for N frames
    pub HiddenFramesCannotSkipItems: i8,
    // Hide the window for N frames while allowing items to be submitted so we can measure their size
    pub HiddenFramesForRenderOnly: i8,
    // Hide the window until frame N at Render() time only
    pub DisableInputsFrames: i8,
    // Disable window interactions for N frames
// ImGuiCond               SetWindowPosAllowFlags : 8;         // store acceptable condition flags for SetNextWindowPos() use.
    pub SetWindowPosAllowFlags: ImGuiCond,
    // ImGuiCond               SetWindowSizeAllowFlags : 8;        // store acceptable condition flags for SetNextWindowSize() use.
    pub SetWindowSizeAllowFlags: ImGuiCond,
    // ImGuiCond               SetWindowCollapsedAllowFlags : 8;   // store acceptable condition flags for SetNextWindowCollapsed() use.
    pub SetWindowCollapsedAllowFlags: ImGuiCond,
    // ImGuiCond               SetWindowDockAllowFlags : 8;        // store acceptable condition flags for SetNextWindowDock() use.
    pub SetWindowDockAllowFlags: ImGuiCond,
    pub SetWindowPosVal: ImVec2,
    // store window position when using a non-zero Pivot (position set needs to be processed when we know the window size)
    pub SetWindowPosPivot: ImVec2,                  // store window pivot for positioning. ImVec2::new(0, 0) when positioning from top-left corner; ImVec2::new(0.5, 0.5) for centering; ImVec2::new(1, 1) for bottom right.

    // ImVector<ImGuiID>       IDStack;                            // ID stack. ID are hashes seeded with the value at the top of the stack. (In theory this should be in the TempData structure)
    pub IDStack: Vec<ImGuiID>,
    pub DC: ImGuiWindowTempData,                                 // Temporary per-window data, reset at the beginning of the frame. This used to be called ImGuiDrawContext, hence the "DC" variable name.

    // The best way to understand what those rectangles are is to use the 'Metrics->Tools->Show Windows Rectangles' viewer.
// The main 'OuterRect', omitted as a field, is window.Rect().
    pub OuterRectClipped: ImRect,
    // == window.Rect() just after setup in Begin(). == window.Rect() for root window.
    pub InnerRect: ImRect,
    // Inner rectangle (omit title bar, menu bar, scroll bar)
    pub InnerClipRect: ImRect,
    // == InnerRect shrunk by WindowPadding*0.5 on each side, clipped within viewport or parent clip rect.
    pub WorkRect: ImRect,
    // Initially covers the whole scrolling region. Reduced by containers e.g columns/tables when active. Shrunk by WindowPadding*1.0 on each side. This is meant to replace ContentRegionRect over time (from 1.71+ onward).
    pub ParentWorkRect: ImRect,
    // Backup of WorkRect before entering a container such as columns/tables. Used by e.g. SpanAllColumns functions to easily access. Stacked containers are responsible for maintaining this. // FIXME-WORKRECT: Could be a stack?
    pub ClipRect: ImVec4,
    // Current clipping/scissoring rectangle, evolve as we are using PushClipRect(), etc. == DrawList.clip_rect_stack.back().
    pub ContentRegionRect: ImRect,
    // FIXME: This is currently confusing/misleading. It is essentially WorkRect but not handling of scrolling. We currently rely on it as right/bottom aligned sizing operation need some size to rely on.
    pub HitTestHoleSize: ImVec2ih,
    // Define an optional rectangular hole where mouse will pass-through the window.
    pub HitTestHoleOffset: ImVec2ih,

    pub LastFrameActive: c_int,
    // Last frame number the window was Active.
    pub LastFrameJustFocused: c_int,
    // Last frame number the window was made Focused.
    pub LastTimeActive: c_float,
    // Last timestamp the window was Active (using float as we don't need high precision there)
    pub ItemWidthDefault: c_float,
    pub StateStorage: ImGuiStorage,
    // ImVector<ImGuiOldColumns> ColumnsStorage;
    pub ColumnsStorage: Vec<ImGuiOldColumns>,
    pub FontWindowScale: c_float,
    // User scale multiplier per-window, via SetWindowFontScale()
    pub FontDpiScale: c_float,
    pub SettingsOffset: c_int,                     // Offset into SettingsWindows[] (offsets are always valid as we only grow the array from the back)

    pub DrawList: *mut ImDrawList,
    // == &DrawListInst (for backward compatibility reason with code using imgui_internal.h we keep this a pointer)
    pub DrawListInst: *mut ImDrawList,
    pub ParentWindow: *mut ImGuiWindow,
    // If we are a child _or_ popup _or_ docked window, this is pointing to our parent. Otherwise NULL.
    pub ParentWindowInBeginStack: *mut ImGuiWindow,
    pub RootWindow: *mut ImGuiWindow,
    // Point to ourself or first ancestor that is not a child window. Doesn't cross through popups/dock nodes.
    pub RootWindowPopupTree: *mut ImGuiWindow,
    // Point to ourself or first ancestor that is not a child window. Cross through popups parent<>child.
    pub RootWindowDockTree: *mut ImGuiWindow,
    // Point to ourself or first ancestor that is not a child window. Cross through dock nodes.
    pub RootWindowForTitleBarHighlight: *mut ImGuiWindow,
    // Point to ourself or first ancestor which will display TitleBgActive color when this window is active.
    pub RootWindowForNav: *mut ImGuiWindow,                   // Point to ourself or first ancestor which doesn't have the NavFlattened flag.

    pub NavLastChildNavWindow: *mut ImGuiWindow,
    // When going to the menu bar, we remember the child window we came from. (This could probably be made implicit if we kept g.Windows sorted by last focused including child window.)
// ImGuiID                 NavLastIds[ImGuiNavLayer_COUNT];    // Last known NavId for this window, per layer (0/1)
    pub NavLastIds: [ImGuiID; ImGuiNavLayer_COUNT as usize],
    // ImRect                  NavRectRel[ImGuiNavLayer_COUNT];    // Reference rectangle, in window relative space
    pub NavRectRel: [ImRect; ImGuiNavLayer_COUNT as usize],

    pub MemoryDrawListIdxCapacity: c_int,
    // Backup of last idx/vtx count, so when waking up the window we can preallocate and avoid iterative alloc/copy
    pub MemoryDrawListVtxCapacity: c_int,
    pub MemoryCompacted: bool,                    // Set when window extraneous data have been garbage collected

    // Docking
// bool                    DockIsActive        :1;             // When docking artifacts are actually visible. When this is set, DockNode is guaranteed to be != NULL. ~~ (DockNode != NULL) && (DockNode.Windows.Size > 1).
    pub DockIsActive: bool,
    // bool                    DockNodeIsVisible   :1;
    pub DockNodeIsVisible: bool,
    // bool                    DockTabIsVisible    :1;             // Is our window visible this frame? ~~ is the corresponding tab selected?
    pub DockTabIsVisible: bool,
    // bool                    DockTabWantClose    :1;
    pub DockTabWantClose: bool,
    pub DockOrder: i32,
    // Order of the last time the window was visible within its DockNode. This is used to reorder windows that are reappearing on the same frame. Same value between windows that were active and windows that were none are possible.
    pub DockStyle: ImGuiWindowDockStyle,
    pub DockNode: *mut ImGuiDockNode,
    // Which node are we docked into. Important: Prefer testing DockIsActive in many cases as this will still be set when the dock node is hidden.
    pub DockNodeAsHost: *mut ImGuiDockNode,
    // Which node are we owning (for parent windows)
    pub DockId: ImGuiID,
    // Backup of last valid DockNode.ID, so single window remember their dock node id even when they are not bound any more
    pub DockTabItemStatusFlags: ImGuiItemStatusFlags,
    pub DockTabItemRect: ImRect,

}

impl ImGuiWindow {
    //ImGuiWindow(context: *mut ImGuiContext, *const c_char name);
    pub unsafe fn new(context: *mut ImGuiContext, name: *const c_char) {
        let mut out = Self {
            Name: ImStrdup(name),
            NameBufLen: libc::strlen(name) + 1,
            ID: ImHashStr(name, 0, 0),
            ViewportAllowPlatformMonitorExtend: -1,
            ViewportPos: ImVec2::new(f32::MAX, f32::MAX),
            MoveId: GetID("#MOVE"),
            TabId: GetID("#TAB"),
            ScrollTarget: ImVec2::new(f32::MAX, f32::MAX),
            ScrollTargetCenterRatio: ImVec2::new(0.5, 0.5),
            AutoFitFramesX: -1,
            AutoFitFramesY: -1,
            AutoPosLastDirection: ImGuiDir_None,
            SetWindowPosAllowFlags: ImGuiCond_Always | ImGuiCond_Once | ImGuiCond_FirstUseEver | ImGuiCond_Appearing,
            SetWindowSizeAllowFlags: ImGuiCond_Always | ImGuiCond_Once | ImGuiCond_FirstUseEver | ImGuiCond_Appearing,
            SetWindowCollapsedAllowFlags: ImGuiCond_Always | ImGuiCond_Once | ImGuiCond_FirstUseEver | ImGuiCond_Appearing,
            SetWindowDockAllowFlags: ImGuiCond_Always | ImGuiCond_Once | ImGuiCond_FirstUseEver | ImGuiCond_Appearing,
            SetWindowPosVal: ImVec2::new(f32::MAX, f32::MAX),
            SetWindowPosPivot: ImVec2::new(f32::MAX, f32::MAX),
            LastFrameActive: -1,
            LastFrameJustFocused: -1,
            LastTimeActive: -1.0,
            FontWindowScale: 1.0,
            FontDpiScale: 1.0,
            SettingsOffset: -1,
            DockOrder: -1,
            DrawList: null_mut(),
            DrawListInst: null_mut(),
            ..Default::default()
        };

        out.DrawList._Data = &context.DrawListSharedData;
        out.DrawList._OwnerName = Name;
        out.IDStack.push(ID);
    }


    //~ImGuiWindow();

    // ImGuiID     GetID(*const c_char str, *const c_char str_end = NULL);
    pub unsafe fn GetID(&self, begin: *const c_char, end: *const c_char) -> ImGuiID {
        let mut seed: ImGuiID = self.IDStack.last().unwrap().clone();
        let mut id: ImGuiID = ImHashStr(begin, if end.is_null() == false { (end - begin) } else { 0 }, seed as u32);
        let g = GImGui; // ImGuiContext& g = *GImGui;
        if g.DebugHookIdInfo == id {
            DebugHookIdInfo(id, ImGuiDataType_String, begin, end);
        }
        return id;
    }


    // ImGuiID     GetID(const void* ptr);
    pub unsafe fn GetID2(&self, ptr: *const c_void) -> ImGuiID {
        let mut seed: ImGuiID = self.IDStack.last().unwrap().clone();
        let mut id: ImGuiID = ImHashData(&ptr, mem::size_of::<*mut c_void>(), seed as u32);
        let g = GImGui; // ImGuiContext& g = *GImGui;
        if g.DebugHookIdInfo == id {
            DebugHookIdInfo(id, ImGuiDataType_Pointer, ptr, null_mut());
        }
        return id;
    }


    // ImGuiID     GetID(int n);
    // ImGuiID ImGuiWindow::GetID(n: c_int)
    pub unsafe fn GetID3(&self, n: c_int) -> ImGuiID {
        let mut seed: ImGuiID = self.IDStack.last().unwrap().clone();
        let mut id: ImGuiID = ImHashData(&n, libc::sizeof(n), seed as u32);
        let g = GImGui; // ImGuiContext& g = *GImGui;
        if g.DebugHookIdInfo == id {
            // DebugHookIdInfo(id, ImGuiDataType_S32, n, NULL);
        }
        return id;
    }

    // ImGuiID     GetIDFromRectangle(const ImRect& r_abs);
    // This is only used in rare/specific situations to manufacture an ID out of nowhere.
    // ImGuiID ImGuiWindow::GetIDFromRectangle(const ImRect& r_abs)
    pub unsafe fn GetIDFromRectangle(&self, r_abs: &ImRect) -> ImGuiID
    {
        let mut seed: ImGuiID =  self.IDStack.last().unwrap().clone();
        let r_rel: ImRect =  WindowRectAbsToRel(this, r_abs);
        let mut id: ImGuiID =  ImHashData(&r_rel, libc::sizeof(r_rel), seed as u32);
        return id;
    }

// We don't use g.FontSize because the window may be != g.CurrentWindow.
//     ImRect      Rect() const            { return ImRect(Pos.x, Pos.y, Pos.x + Size.x, Pos.y + Size.y); }


    // float       CalcFontSize() const    { let g = GImGui; // ImGuiContext& g = *GImGui; float scale = g.FontBaseSize * FontWindowScale * FontDpiScale; if (ParentWindow) scale *= Parentwindow.FontWindowScale; return scale; }


    // float       TitleBarHeight() const  { let g = GImGui; // ImGuiContext& g = *GImGui; return (Flags & ImGuiWindowFlags_NoTitleBar) ? 0.0 : CalcFontSize() + g.Style.FramePadding.y * 2.0; }


    // ImRect      TitleBarRect() const    { return ImRect(Pos, ImVec2::new(Pos.x + SizeFull.x, Pos.y + TitleBarHeight())); }


    // float       MenuBarHeight() const   { let g = GImGui; // ImGuiContext& g = *GImGui; return (Flags & ImGuiWindowFlags_MenuBar) ? DC.MenuBarOffset.y + CalcFontSize() + g.Style.FramePadding.y * 2.0 : 0.0; }


    // ImRect      MenuBarRect() const     { float y1 = Pos.y + TitleBarHeight(); return ImRect(Pos.x, y1, Pos.x + SizeFull.x, y1 + MenuBarHeight()); }
}
