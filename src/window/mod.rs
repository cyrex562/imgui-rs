#![allow(non_snake_case)]

use crate::core::context::AppContext;
use crate::core::condition::{
    ImGuiCond, ImGuiCond_Always, ImGuiCond_Appearing, ImGuiCond_FirstUseEver, ImGuiCond_Once,
};
use crate::data_type::{IM_GUI_DATA_TYPE_POINTER, IM_GUI_DATA_TYPE_S32, IM_GUI_DATA_TYPE_STRING};
use crate::debug_ops::DebugHookIdInfo;
use crate::core::direction::{ImGuiDir, ImGuiDir_None};
use crate::docking::dock_node::ImGuiDockNode;
use crate::drawing::draw_list::ImDrawList;
use crate::core::hash_ops::{hash_data, hash_string};
use crate::core::id_ops::id_from_str;
use crate::imgui::GImGui;
use crate::item::item_status_flags::ImGuiItemStatusFlags;
use crate::layout::layout_type::ImGuiLayoutType;
use crate::nav_layer::ImGuiNavLayer_COUNT;
use crate::table::old_columns::ImGuiOldColumns;
use crate::rect::ImRect;
use crate::core::stack_sizes::ImGuiStackSizes;
use crate::core::storage::ImGuiStorage;
use crate::core::string_ops::ImStrdup;
use crate::core::type_defs::{ImGuiDir, ImguiHandle};
use crate::core::vec2::{Vector2, ImVec2ih};
use crate::core::vec4::ImVec4;
use crate::viewport::ImguiViewport;
use crate::docking::win_dock_style::ImGuiWindowDockStyle;
use crate::window::window_class::ImGuiWindowClass;
use crate::window::window_flags::ImGuiWindowFlags;
use crate::window::window_temp_data::ImGuiWindowTempData;
use crate::window_class::ImGuiWindowClass;
use crate::window_flags::ImGuiWindowFlags;
use crate::window_ops::WindowRectAbsToRel;
use crate::window_temp_data::ImGuiWindowTempData;
use libc::{c_char, c_float, c_int, c_short, c_void};
use rect::window_rect_abs_to_rel;
use std::borrow::Borrow;
use std::mem;
use std::ptr::null_mut;

pub mod find;
pub mod focus;
pub mod ops;
pub mod props;
pub mod rect;
pub mod render;
pub mod window_class;
pub mod window_dock_style_color;
pub mod window_dock_style_colors;
pub mod window_flags;
pub mod window_settings;
pub mod window_stack_data;
pub mod window_temp_data;
pub mod input_text;
pub mod input_text_callback_data;
pub mod input_text_flags;
pub mod input_text_state;
mod menu_columns;
pub mod next_window_data;
pub mod next_window_data_flags;

// Storage for one window
pub struct ImguiWindow {
    pub Name: String,
    // Window name, owned by the window.
    pub ID: ImguiHandle,
    // == ImHashStr(Name)
    // ImGuiWindowFlags        Flags, FlagsPreviousFrame;          // See enum ImGuiWindowFlags_
    pub Flags: ImGuiWindowFlags,
    pub FlagsPreviousFrame: ImGuiWindowFlags,
    pub WindowClass: ImGuiWindowClass,
    // Advanced users only. Set with SetNextWindowClass()
    pub Viewport: ImguiViewport,
    // Always set in Begin(). Inactive windows may have a NULL value here if their viewport was discarded.
    pub ViewportId: ImguiHandle,
    // We backup the viewport id (since the viewport may disappear or never be created if the window is inactive)
    pub ViewportPos: Vector2,
    // We backup the viewport position (since the viewport may disappear or never be created if the window is inactive)
    pub ViewportAllowPlatformMonitorExtend: c_int,
    // Reset to -1 every frame (index is guaranteed to be valid between NewFrame..EndFrame), only used in the Appearing frame of a tooltip/popup to enforce clamping to a given monitor
    pub position: Vector2,
    // Position (always rounded-up to nearest pixel)
    pub Size: Vector2,
    // Current size (==SizeFull or collapsed title bar size)
    pub SizeFull: Vector2,
    // Size when non collapsed
    pub ContentSize: Vector2,
    // Size of contents/scrollable client area (calculated from the extents reach of the cursor) from previous frame. Does not include window decoration or window padding.
    pub ContentSizeIdeal: Vector2,
    pub ContentSizeExplicit: Vector2,
    // Size of contents/scrollable client area explicitly request by the user via SetNextWindowContentSize().
    pub WindowPadding: Vector2,
    // Window padding at the time of Begin().
    pub WindowRounding: c_float,
    // Window rounding at the time of Begin(). May be clamped lower to avoid rendering artifacts with title bar, menu bar etc.
    pub WindowBorderSize: c_float,
    // Window border size at the time of Begin().
    pub NameBufLen: usize,
    // Size of buffer storing Name. May be larger than strlen(Name)!
    pub MoveId: ImguiHandle,
    // == window.GetID("#MOVE")
    pub TabId: ImguiHandle,
    // == window.GetID("#TAB")
    pub ChildId: ImguiHandle,
    // ID of corresponding item in parent window (for navigation to return from child window to parent window)
    pub scroll: Vector2,
    pub ScrollMax: Vector2,
    pub ScrollTarget: Vector2,
    // target scroll position. stored as cursor position with scrolling canceled out, so the highest point is always 0.0. (f32::MAX for no change)
    pub ScrollTargetCenterRatio: Vector2,
    // 0.0 = scroll so that target position is at top, 0.5 = scroll so that target position is centered
    pub ScrollTargetEdgeSnapDist: Vector2,
    // 0.0 = no snapping, >0.0 snapping threshold
    pub ScrollbarSizes: Vector2,
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
    pub skip_items: bool,
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
    pub PopupId: ImguiHandle,
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
    pub SetWindowPosVal: Vector2,
    // store window position when using a non-zero Pivot (position set needs to be processed when we know the window size)
    pub SetWindowPosPivot: Vector2, // store window pivot for positioning. ImVec2::new(0, 0) when positioning from top-left corner; ImVec2::new(0.5, 0.5) for centering; ImVec2::new(1, 1) for bottom right.

    // ImVector<ImguiHandle>       IDStack;                            // ID stack. ID are hashes seeded with the value at the top of the stack. (In theory this should be in the TempData structure)
    pub id_stack: Vec<ImguiHandle>,
    pub dc: ImGuiWindowTempData, // Temporary per-window data, reset at the beginning of the frame. This used to be called ImGuiDrawContext, hence the "DC" variable name.

    // The best way to understand what those rectangles are is to use the 'Metrics->Tools->Show Windows Rectangles' viewer.
    // The main 'OuterRect', omitted as a field, is window.Rect().
    pub OuterRectClipped: ImRect,
    // == window.Rect() just after setup in Begin(). == window.Rect() for root window.
    pub InnerRect: ImRect,
    // Inner rectangle (omit title bar, menu bar, scroll bar)
    pub InnerClipRect: ImRect,
    // == InnerRect shrunk by WindowPadding*0.5 on each side, clipped within viewport or parent clip rect.
    pub work_rect: ImRect,
    // Initially covers the whole scrolling region. Reduced by containers e.g columns/tables when active. Shrunk by WindowPadding*1.0 on each side. This is meant to replace ContentRegionRect over time (from 1.71+ onward).
    pub ParentWorkRect: ImRect,
    // Backup of WorkRect before entering a container such as columns/tables. Used by e.g. SpanAllColumns functions to easily access. Stacked containers are responsible for maintaining this. // FIXME-WORKRECT: Could be a stack?
    pub ClipRect: ImVec4,
    // Current clipping/scissoring rectangle, evolve as we are using PushClipRect(), etc. == DrawList.clip_rect_stack.back().
    pub content_region_rect: ImRect,
    // FIXME: This is currently confusing/misleading. It is essentially WorkRect but not handling of scrolling. We currently rely on it as right/bottom aligned sizing operation need some size to rely on.
    pub HitTestHoleSize: ImVec2ih,
    // Define an optional rectangular hole where mouse will pass-through the window.
    pub HitTestHoleOffset: ImVec2ih,

    pub LastFrameActive: usize,
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
    pub SettingsOffset: c_int, // Offset into SettingsWindows[] (offsets are always valid as we only grow the array from the back)

    pub DrawList: ImDrawList,
    // == &DrawListInst (for backward compatibility reason with code using imgui_internal.h we keep this a pointer)
    pub DrawListInst: Option<ImDrawList>,
    pub Parentwindow: Option<ImguiHandle>,
    // If we are a child _or_ popup _or_ docked window, this is pointing to our parent. Otherwise NULL.
    pub ParentWindowInBeginStack: ImguiHandle,
    pub Rootwindow: ImguiHandle,
    // Point to ourself or first ancestor that is not a child window. Doesn't cross through popups/dock nodes.
    pub RootWindowPopupTree: ImguiHandle,
    // Point to ourself or first ancestor that is not a child window. Cross through popups parent<>child.
    pub RootWindowDockTree: ImguiHandle,
    // Point to ourself or first ancestor that is not a child window. Cross through dock nodes.
    pub RootWindowForTitleBarHighlight: ImguiHandle,
    // Point to ourself or first ancestor which will display TitleBgActive color when this window is active.
    pub RootWindowForNav: ImguiHandle, // Point to ourself or first ancestor which doesn't have the NavFlattened flag.

    pub NavLastChildNavwindow: ImguiHandle,
    // When going to the menu bar, we remember the child window we came from. (This could probably be made implicit if we kept g.Windows sorted by last focused including child window.)
    // ImguiHandle                 NavLastIds[ImGuiNavLayer_COUNT];    // Last known NavId for this window, per layer (0/1)
    pub NavLastIds: [ImguiHandle; ImGuiNavLayer_COUNT as usize],
    // ImRect                  NavRectRel[ImGuiNavLayer_COUNT];    // Reference rectangle, in window relative space
    pub NavRectRel: [ImRect; ImGuiNavLayer_COUNT as usize],

    pub MemoryDrawListIdxCapacity: c_int,
    // Backup of last idx/vtx count, so when waking up the window we can preallocate and avoid iterative alloc/copy
    pub MemoryDrawListVtxCapacity: c_int,
    pub MemoryCompacted: bool, // Set when window extraneous data have been garbage collected

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
    pub DockNode: Option<ImGuiDockNode>,
    // Which node are we docked into. Important: Prefer testing DockIsActive in many cases as this will still be set when the dock node is hidden.
    pub DockNodeAsHost: Option<ImGuiDockNode>,
    // Which node are we owning (for parent windows)
    pub DockId: ImguiHandle,
    // Backup of last valid DockNode.ID, so single window remember their dock node id even when they are not bound any more
    pub DockTabItemStatusFlags: ImGuiItemStatusFlags,
    pub DockTabItemRect: ImRect,
}

impl ImguiWindow {
    //ImGuiWindow(context: *mut ImGuiContext, *const c_char name);
    pub fn new(g: &mut AppContext, name: &String) -> Self {
        let mut out = Self {
            Name: String::from(name),
            // NameBufLen: name.len(),
            ID: hash_string(name, 0),
            ViewportAllowPlatformMonitorExtend: -1,
            ViewportPos: Vector2::from_floats(f32::MAX, f32::MAX),
            MoveId: id_from_str("#MOVE"),
            TabId: id_from_str("#TAB"),
            ScrollTarget: Vector2::from_floats(f32::MAX, f32::MAX),
            ScrollTargetCenterRatio: Vector2::from_floats(0.5, 0.5),
            AutoFitFramesX: -1,
            AutoFitFramesY: -1,
            AutoPosLastDirection: ImGuiDir_None,
            SetWindowPosAllowFlags: ImGuiCond_Always
                | ImGuiCond_Once
                | ImGuiCond_FirstUseEver
                | ImGuiCond_Appearing,
            SetWindowSizeAllowFlags: ImGuiCond_Always
                | ImGuiCond_Once
                | ImGuiCond_FirstUseEver
                | ImGuiCond_Appearing,
            SetWindowCollapsedAllowFlags: ImGuiCond_Always
                | ImGuiCond_Once
                | ImGuiCond_FirstUseEver
                | ImGuiCond_Appearing,
            SetWindowDockAllowFlags: ImGuiCond_Always
                | ImGuiCond_Once
                | ImGuiCond_FirstUseEver
                | ImGuiCond_Appearing,
            SetWindowPosVal: Vector2::from_floats(f32::MAX, f32::MAX),
            SetWindowPosPivot: Vector2::from_floats(f32::MAX, f32::MAX),
            LastFrameActive: -1,
            LastFrameJustFocused: -1,
            LastTimeActive: -1.0,
            FontWindowScale: 1.0,
            FontDpiScale: 1.0,
            SettingsOffset: -1,
            DockOrder: -1,
            DrawList: None,
            DrawListInst: None,
            ..Default::default()
        };

        out.DrawList._Data = &g.DrawListSharedData;
        out.DrawList._OwnerName = Name;
        out.id_stack.push(ID);
    }

    //~ImGuiWindow();

    // ImguiHandle     GetID(*const c_char str, *const c_char str_end = NULL);
    pub fn id_by_string(&self, g: &mut AppContext, begin: &String) -> ImguiHandle {
        let mut seed: ImguiHandle = self.id_stack.last().unwrap().clone();
        let mut id: ImguiHandle = hash_string(begin, seed as u32);
        // let g = GImGui; // ImGuiContext& g = *GImGui;
        if g.DebugHookIdInfo == id {
            DebugHookIdInfo(g, id, IM_GUI_DATA_TYPE_STRING, begin.into_bytes().borrow());
        }
        return id;
    }

    pub fn id_by_int(&self, g: &mut AppContext, n: c_int) -> ImguiHandle {
        let mut seed = self.id_stack.last().unwrap().clone();
        let mut id = hash_data(&n.to_le_bytes(), seed as u32);
        if g.DebugHookIdInfo == id {
            // DebugHookIdInfo(id, IM_GUI_DATA_TYPE_S32, n, NULL);
        }
        return id;
    }

    // ImguiHandle     GetIDFromRectangle(const ImRect& r_abs);
    // This is only used in rare/specific situations to manufacture an ID out of nowhere.
    // ImguiHandle ImGuiWindow::GetIDFromRectangle(const ImRect& r_abs)
    pub unsafe fn id_by_rectangle(&self, r_abs: &ImRect) -> ImguiHandle {
        let mut seed: ImguiHandle = self.id_stack.last().unwrap().clone();
        let r_rel: ImRect = window_rect_abs_to_rel(self, r_abs);

        let mut id: ImguiHandle = hash_data(&r_rel, seed as u32);
        return id;
    }

    // We don't use g.FontSize because the window may be != g.CurrentWindow.
    //     ImRect      Rect() const            { return ImRect(Pos.x, Pos.y, Pos.x + Size.x, Pos.y + Size.y); }

    // float       CalcFontSize() const    { let g = GImGui; // ImGuiContext& g = *GImGui; float scale = g.FontBaseSize * FontWindowScale * FontDpiScale; if (ParentWindow) scale *= Parentwindow.FontWindowScale; return scale; }

    // float       TitleBarHeight() const  { let g = GImGui; // ImGuiContext& g = *GImGui; return (Flags & ImGuiWindowFlags_NoTitleBar) ? 0.0 : CalcFontSize() + g.style.FramePadding.y * 2.0; }

    // ImRect      TitleBarRect() const    { return ImRect(Pos, ImVec2::new(Pos.x + SizeFull.x, Pos.y + TitleBarHeight())); }

    // float       MenuBarHeight() const   { let g = GImGui; // ImGuiContext& g = *GImGui; return (Flags & ImGuiWindowFlags_MenuBar) ? DC.MenuBarOffset.y + CalcFontSize() + g.style.FramePadding.y * 2.0 : 0.0; }

    // ImRect      MenuBarRect() const     { float y1 = Pos.y + TitleBarHeight(); return ImRect(Pos.x, y1, Pos.x + SizeFull.x, y1 + MenuBarHeight()); }
}
