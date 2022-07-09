use std::borrow::Borrow;
use std::cell::RefCell;
use std::os::raw::c_char;
use std::ptr::{null, null_mut};
use std::rc::Rc;
use std::collections::HashSet;
use crate::column::DimgOldColumns;
use crate::condition::DimgCond;
use crate::context::DimgContext;
use crate::defines::ImGuiSizeCallback;
use crate::direction::DimgDirection;
use crate::dock::DimgDockNodeFlags;
use crate::dock_node::DimgDockNode;
use crate::draw_list::DimgDrawList;
use crate::globals::GImGui;
use crate::hash::{ImHashData, ImHashStr};
use crate::input::DimgNavLayer;
use crate::item::{DimgItemStatusFlags, DimgLastItemData};
use crate::kv_store::DimgStorage;
use crate::layout::ImGuiLayoutType;
use crate::menu::ImGuiMenuColumns;
use crate::rect::DimgRect;
use crate::stack::ImGuiStackSizes;
use crate::tab_bar::DimgTabItemFlags;
use crate::vec_nd::{DimgVec2D, ImVec1};
use crate::types::{DimgId, DimgWindowHandle, ImGuiDataType};
use crate::viewport::{DimgViewport, DimgViewportFlags};


// Transient per-window data, reset at the beginning of the frame. This used to be called ImGuiDrawContext, hence the dc variable name in ImGuiWindow.
// (That's theory, in practice the delimitation between ImGuiWindow and ImGuiWindowTempData is quite tenuous and could be reconsidered..)
// (This doesn't need a constructor because we zero-clear it as part of ImGuiWindow and all frame-temporary data are setup on Begin)
pub struct DimgWindowTempData {
    // Layout
    // ImVec2                  CursorPos;              // Current emitting position, in absolute coordinates.
    pub CursorPos: DimgVec2D,
    // ImVec2                  CursorPosPrevLine;
    pub CursorPosPrevLine: DimgVec2D,
    // ImVec2                  CursorStartPos;         // Initial position after Begin(), generally ~ window position + window_padding.
    pub CursorStartPos: DimgVec2D,
    // ImVec2                  CursorMaxPos;           // Used to implicitly calculate content_size at the beginning of next frame, for scrolling range and auto-resize. Always growing during the frame.
    pub CursorMaxPos: DimgVec2D,
    // ImVec2                  IdealMaxPos;            // Used to implicitly calculate content_size_ideal at the beginning of next frame, for auto-resize only. Always growing during the frame.
    pub IdealMaxPos: DimgVec2D,
    // ImVec2                  CurrLineSize;
    pub CurrLineSize: DimgVec2D,
    // ImVec2                  PrevLineSize;
    pub PrevLineSize: DimgVec2D,
    // float                   CurrLineTextBaseOffset; // Baseline offset (0.0 by default on a new line, generally == style.FramePadding.y when a framed item has been added).
    pub CurrLineTextBaseOffset: f32,
    // float                   PrevLineTextBaseOffset;
    pub PrevLineTextBaseOffset: f32,
    // bool                    IsSameLine;
    pub IsSameLine: bool,
    // ImVec1                  Indent;                 // Indentation / start position from left of window (increased by TreePush/TreePop, etc.)
    pub Indent: ImVec1,
    // ImVec1                  ColumnsOffset;          // Offset to the current column (if ColumnsCurrent > 0). FIXME: This and the above should be a stack to allow use cases like Tree->column->Tree. Need revamp columns API.
    pub ColumnsOffset: ImVec1,
    // ImVec1                  GroupOffset;
    pub GroupOffset: ImVec1,
    // ImVec2                  CursorStartPosLossyness;// Record the loss of precision of CursorStartPos due to really large scrolling amount. This is used by clipper to compensentate and fix the most common use case of large scroll area.
    pub CursortStartPosLossyness: DimgVec2D,
    // Keyboard/Gamepad navigation
    // ImGuiNavLayer           NavLayerCurrent;        // Current layer, 0..31 (we currently only use 0..1)
    pub NavLayerCurrent: DimgNavLayer,
    // short                   NavLayersActiveMask;    // Which layers have been written to (result from previous frame)
    pub NavLayersActiveMask: i16,
    // short                   NavLayersActiveMaskNext;// Which layers have been written to (accumulator for current frame)
    pub NavLayersActiveMaskNext: i16,
    // ImGuiID                 NavFocusScopeIdCurrent; // Current focus scope id while appending
    pub NavFocusScopeIdCurrent: DimgId,
    // bool                    NavHideHighlightOneFrame;
    pub NavHideHiglightOneFrame: bool,
    // bool                    NavHasScroll;           // Set when scrolling can be used (scroll_max > 0.0)
    pub NavHasScroll: bool,
    // Miscellaneous
    // bool                    MenuBarAppending;       // FIXME: Remove this
    pub MenuBarAppending: bool,
    // ImVec2                  MenuBarOffset;          // MenuBarOffset.x is sort of equivalent of a per-layer CursorPos.x, saved/restored as we switch to the menu bar. The only situation when MenuBarOffset.y is > 0 if when (SafeAreaPadding.y > FramePadding.y), often used on TVs.
    pub MenuBarOffset: DimgVec2D,
    // ImGuiMenuColumns        MenuColumns;            // Simplified columns storage for menu items measurement
    pub MenuColumns: ImGuiMenuColumns,
    // int                     TreeDepth;              // Current tree depth.
    pub TreeDepth: i32,
    // ImU32                   TreeJumpToParentOnPopMask; // Store a copy of !g.nav_id_is_alive for TreeDepth 0..31.. Could be turned into a ImU64 if necessary.
    pub TreeJumpToParentOnPopMask: u32,
    // ImVector<ImGuiWindow*>  ChildWindows;
    pub ChildWindows: Vec<DimgWindow>,
    // ImGuiStorage*           state_storage;           // Current persistent per-window storage (store e.g. tree node open/close state)
    pub StateStorage: Vec<u8>,
    // ImGuiOldColumns*        CurrentColumns;         // Current columns set
    pub CurrentColumns: DimgOldColumns,
    // int                     CurrentTableIdx;        // Current table index (into g.tables)
    pub CurrentTableIdx: usize,
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
#[derive(Default,Debug,Clone)]
pub struct DimgWindow {
    // char*                   name;                               // Window name, owned by the window.
    pub name: String,
    //*mut c_char,
    // ImGuiID                 id;                                 // == ImHashStr(name)
    pub id: DimgId,
    // ImGuiWindowFlags        flags, flags_previous_frame;          // See enum ImGuiWindowFlags_
    pub flags: DimgWindowFlags,
    pub flags_previous_frame: DimgWindowFlags,
    // ImGuiWindowClass        window_class;                        // Advanced users only. Set with SetNextWindowClass()
    pub window_class: DimgWindowClass,
    // ImGuiViewportP*         viewport;                           // Always set in Begin(). Inactive windows may have a NULL value here if their viewport was discarded.
    pub viewport: DimgId,
    // ImGuiID                 viewport_id;                         // We backup the viewport id (since the viewport may disappear or never be created if the window is inactive)
    pub viewport_id: DimgId,
    // ImVec2                  viewport_pos;                        // We backup the viewport position (since the viewport may disappear or never be created if the window is inactive)
    pub viewport_pos: DimgVec2D,
    // int                     viewport_allow_platform_monitor_extend; // Reset to -1 every frame (index is guaranteed to be valid between NewFrame..EndFrame), only used in the appearing frame of a tooltip/popup to enforce clamping to a given monitor
    pub viewport_allow_platform_monitor_extend: i32,
    // ImVec2                  pos;                                // Position (always rounded-up to nearest pixel)
    pub pos: DimgVec2D,
    // ImVec2                  size;                               // Current size (==size_full or collapsed title bar size)
    pub size: DimgVec2D,
    // ImVec2                  size_full;                           // size when non collapsed
    pub size_full: DimgVec2D,
    // ImVec2                  content_size;                        // size of contents/scrollable client area (calculated from the extents reach of the cursor) from previous frame. Does not include window decoration or window padding.
    pub content_size: DimgVec2D,
    // ImVec2                  content_size_ideal;
    pub content_size_ideal: DimgVec2D,
    // ImVec2                  content_size_explicit;                // size of contents/scrollable client area explicitly request by the user via SetNextWindowContentSize().
    pub content_size_explicit: DimgVec2D,
    // ImVec2                  window_padding;                      // Window padding at the time of Begin().
    pub window_padding: DimgVec2D,
    // float                   window_rounding;                     // Window rounding at the time of Begin(). May be clamped lower to avoid rendering artifacts with title bar, menu bar etc.
    pub window_rounding: f32,
    // float                   WindowBorderSize;                   // Window border size at the time of Begin().
    // int                     NameBufLen;                         // size of buffer storing name. May be larger than strlen(name)!
    // ImGuiID                 move_id;                             // == window->GetID("#MOVE")
    pub move_id: DimgId,
    // ImGuiID                 tab_id;                              // == window->GetID("#TAB")
    pub tab_id: DimgId,
    // ImGuiID                 child_id;                            // id of corresponding item in parent window (for navigation to return from child window to parent window)
    pub child_id: DimgId,
    // ImVec2                  scroll;
    pub scroll: DimgVec2D,
    // ImVec2                  scroll_max;
    pub scroll_max: DimgVec2D,
    // ImVec2                  scroll_target;                       // target scroll position. stored as cursor position with scrolling canceled out, so the highest point is always 0.0. (FLT_MAX for no change)
    pub scroll_target: DimgVec2D,
    // ImVec2                  scroll_target_center_ratio;            // 0.0 = scroll so that target position is at top, 0.5 = scroll so that target position is centered
    pub scroll_target_center_ratio: DimgVec2D,
    // ImVec2                  scroll_target_edge_snap_dist;           // 0.0 = no snapping, >0.0 snapping threshold
    pub scroll_target_edge_snap_dist: DimgVec2D,
    // ImVec2                  scrollbar_sizes;                     // size taken by each scrollbars on their smaller axis. Pay attention! scrollbar_sizes.x == width of the vertical scrollbar, scrollbar_sizes.y = height of the horizontal scrollbar.
    pub scrollbar_sizes: DimgVec2D,
    // bool                    scrollbar_x, scrollbar_y;             // Are scrollbars visible?
    pub scrollbar_x: bool,
    pub scrollbar_y: bool,
    // bool                    viewport_owned;
    pub viewport_owned: bool,
    // bool                    active;                             // Set to true on Begin(), unless collapsed
    pub active: bool,
    // bool                    was_active;
    pub was_active: bool,
    // bool                    write_accessed;                      // Set to true when any widget access the current window
    pub write_accessed: bool,
    // bool                    collapsed;                          // Set when collapsing window to become only title-bar
    pub collapsed: bool,
    // bool                    want_collapse_toggle;
    pub want_collapse_toggle: bool,
    // bool                    skip_items;                          // Set when items can safely be all clipped (e.g. window not visible or collapsed)
    pub skip_items: bool,
    // bool                    appearing;                          // Set during the frame where the window is appearing (or re-appearing)
    pub appearing: bool,
    // bool                    hidden;                             // Do not display (== HiddenFrames*** > 0)
    pub hidden: bool,
    // bool                    is_fallback_window;                   // Set on the "Debug##Default" window.
    pub is_fallback_window: bool,
    // bool                    is_explicit_child;                    // Set when passed _ChildWindow, left to false by BeginDocked()
    pub is_explicit_child: bool,
    // bool                    has_close_button;                     // Set when the window has a close button (p_open != NULL)
    pub has_close_button: bool,
    // signed char             resize_border_held;                   // Current border being held for resize (-1: none, otherwise 0-3)
    pub resize_border_held: i8,
    // short                   begin_count;                         // Number of Begin() during the current frame (generally 0 or 1, 1+ if appending via multiple Begin/End pairs)
    pub begin_count: i16,
    // short                   begin_order_within_parent;             // Begin() order within immediate parent window, if we are a child window. Otherwise 0.
    pub begin_order_within_parent: i16,
    // short                   begin_order_within_context;            // Begin() order within entire imgui context. This is mostly used for debugging submission order related issues.
    pub begin_order_within_context: i16,
    // short                   focus_order;                         // Order within windows_focus_order[], altered when windows are focused.
    pub focus_order: i16,
    // ImGuiID                 popup_id;                            // id in the popup stack when this window is used as a popup/menu (because we use generic name/id for recycling)
    pub popup_id: DimgId,
    // ImS8                    auto_fit_frames_x, auto_fit_frames_y;
    pub auto_fit_frames_x: i8,
    pub auto_fit_frames_y: i8,
    // ImS8                    auto_fit_child_axises;
    pub auto_fit_child_axises: i8,
    // bool                    auto_fit_only_grows;
    pub auto_fit_only_grows: bool,
    // ImGuiDir                auto_pos_last_direction;
    pub auto_pos_last_direction: DimgDirection,
    // ImS8                    hidden_frames_can_skip_items;           // Hide the window for N frames
    pub hidden_frames_can_skip_items: i8,
    // ImS8                    hidden_frames_cannot_skip_items;        // Hide the window for N frames while allowing items to be submitted so we can measure their size
    pub hidden_frames_cannot_skip_items: i8,
    // ImS8                    hidden_frames_for_render_only;          // Hide the window until frame N at Render() time only
    pub hidden_frames_for_render_only: i8,
    // ImS8                    disable_inputs_frames;                // Disable window interactions for N frames
    pub disable_inputs_frames: i8,
    // ImGuiCond               set_window_pos_allow_flags : 8;         // store acceptable condition flags for SetNextWindowPos() use.
    pub set_window_pos_allow_flags: DimgCond,
    // ImGuiCond               set_window_size_allow_flags : 8;        // store acceptable condition flags for SetNextWindowSize() use.
    pub set_window_size_allow_flags: DimgCond,
    // ImGuiCond               set_window_collapsed_allow_flags : 8;   // store acceptable condition flags for SetNextWindowCollapsed() use.
    pub set_window_collapsed_allow_flags: DimgCond,
    // ImGuiCond               SetWindowDockAllowFlags : 8;        // store acceptable condition flags for SetNextWindowDock() use.
    // ImVec2                  set_window_pos_val;                    // store window position when using a non-zero Pivot (position set needs to be processed when we know the window size)
    pub set_window_pos_val: DimgVec2D,
    // ImVec2                  set_window_pos_pivot;                  // store window pivot for positioning. ImVec2(0, 0) when positioning from top-left corner; ImVec2(0.5, 0.5) for centering; ImVec2(1, 1) for bottom right.
    pub set_window_pos_pivot: DimgVec2D,

    // ImVector<ImGuiID>       IDStack;                            // id stack. id are hashes seeded with the value at the top of the stack. (In theory this should be in the TempData structure)
    pub id_stack: Vec<DimgId>,
    // ImGuiWindowTempData     dc;                                 // Temporary per-window data, reset at the beginning of the frame. This used to be called ImGuiDrawContext, hence the "dc" variable name.
    pub dc: DimgWindowTempData,

    // The best way to understand what those rectangles are is to use the 'Metrics->Tools->Show windows Rectangles' viewer.
    // The main 'OuterRect', omitted as a field, is window->rect().
    // ImRect                  outer_rect_clipped;                   // == Window->rect() just after setup in Begin(). == window->rect() for root window.
    pub outer_rect_clipped: DimgRect,
    // ImRect                  inner_rect;                          // Inner rectangle (omit title bar, menu bar, scroll bar)
    pub inner_rect: DimgRect,
    // ImRect                  inner_clip_rect;                      // == inner_rect shrunk by window_padding*0.5 on each side, clipped within viewport or parent clip rect.
    pub inner_clip_rect: DimgRect,
    // ImRect                  work_rect;                           // Initially covers the whole scrolling region. Reduced by containers e.g columns/tables when active. Shrunk by window_padding*1.0 on each side. This is meant to replace content_region_rect over time (from 1.71+ onward).
    pub work_rect: DimgRect,
    // ImRect                  parent_work_rect;                     // Backup of work_rect before entering a container such as columns/tables. Used by e.g. SpanAllColumns functions to easily access. Stacked containers are responsible for maintaining this. // FIXME-WORKRECT: Could be a stack?
    pub parent_work_rect: DimgRect,
    // ImRect                  clip_rect;                           // Current clipping/scissoring rectangle, evolve as we are using PushClipRect(), etc. == draw_list->clip_rect_stack.back().
    pub clip_rect: DimgRect,
    // ImRect                  content_region_rect;                  // FIXME: This is currently confusing/misleading. It is essentially work_rect but not handling of scrolling. We currently rely on it as right/bottom aligned sizing operation need some size to rely on.
    pub content_region_rect: DimgRect,
    // ImVec2ih                hit_test_hole_size;                    // Define an optional rectangular hole where mouse will pass-through the window.
    pub hit_test_hole_size: DimgVec2D,
    // ImVec2ih                hit_test_hole_offset;
    pub hit_test_hole_offset: DimgVec2D,
    // int                     last_frame_active;                    // Last frame number the window was active.
    pub last_frame_active: i32,
    // int                     last_frame_just_focused;               // Last frame number the window was made Focused.
    pub last_frame_just_focused: i32,
    // float                   last_time_active;                     // Last timestamp the window was active (using float as we don't need high precision there)
    pub last_time_active: f32,
    // float                   item_width_default;
    pub item_width_default: f32,
    // ImGuiStorage            state_storage;
    pub state_storage: DimgStorage,
    // ImVector<ImGuiOldColumns> ColumnsStorage;
    pub column: Vec<DimgOldColumns>,
    // float                   font_window_scale;                    // User scale multiplier per-window, via SetWindowFontScale()
    pub font_window_scale: f32,
    // float                   font_dpi_scale;
    pub font_dpi_scale: f32,
    // int                     settings_offset;                     // Offset into settings_windows[] (offsets are always valid as we only grow the array from the back)
    pub settings_offset: i32,
    // ImDrawList*             draw_list;                           // == &DrawListInst (for backward compatibility reason with code using imgui_internal.h we keep this a pointer)
    pub draw_list: DimgDrawList,
    // ImDrawList              DrawListInst;
    pub draw_list_inst: DimgDrawList,
    // ImGuiWindow*            ParentWindow;                       // If we are a child _or_ popup _or_ docked window, this is pointing to our parent. Otherwise NULL.
    pub parent_window: DimgWindowHandle,
    // ImGuiWindow*            parent_window_in_begin_stack;
    pub parent_window_in_begin_stack: DimgWindowHandle,
    // ImGuiWindow*            root_window;                         // Point to ourself or first ancestor that is not a child window. Doesn't cross through popups/dock nodes.
    pub root_window: DimgWindowHandle,
    // ImGuiWindow*            root_window_popup_tree;                // Point to ourself or first ancestor that is not a child window. Cross through popups parent<>child.
    pub root_window_popup_tree: DimgWindowHandle,
    // ImGuiWindow*            root_window_dock_tree;                 // Point to ourself or first ancestor that is not a child window. Cross through dock nodes.
    pub root_window_dock_tree: DimgWindowHandle,
    // ImGuiWindow*            root_window_for_title_bar_highlight;     // Point to ourself or first ancestor which will display TitleBgActive color when this window is active.
    pub root_window_for_title_bar_highlight: DimgWindowHandle,
    // ImGuiWindow*            root_window_for_nav;                   // Point to ourself or first ancestor which doesn't have the NavFlattened flag.
    pub root_window_for_nav: DimgWindowHandle,
    // ImGuiWindow*            nav_last_child_nav_window;              // When going to the menu bar, we remember the child window we came from. (This could probably be made implicit if we kept g.windows sorted by last focused including child window.)
    pub nav_last_child_nav_window: DimgWindowHandle,
    // ImGuiID                 nav_last_ids[ImGuiNavLayer_COUNT];    // Last known nav_id for this window, per layer (0/1)
    pub nav_last_ids: Vec<DimgId>,
    // ImRect                  nav_rect_rel[ImGuiNavLayer_COUNT];    // Reference rectangle, in window relative space
    pub nav_rect_rel: Vec<DimgRect>,
    // int                     memory_draw_list_idx_capacity;          // Backup of last idx/vtx count, so when waking up the window we can preallocate and avoid iterative alloc/copy
    pub memory_draw_list_idx_capacity: usize,
    // int                     memory_draw_list_vtx_capacity;
    pub memory_draw_list_vtx_capacity: usize,
    // bool                    memory_compacted;                    // Set when window extraneous data have been garbage collected
    pub memory_compacted: bool,
    // Docking
    // bool                    dock_is_active        :1;             // When docking artifacts are actually visible. When this is set, dock_node is guaranteed to be != NULL. ~~ (dock_node != NULL) && (dock_node->windows.size > 1).
    pub dock_is_active: bool,
    // bool                    DockNodeIsVisible   :1;
    pub doc_node_is_visible: bool,
    // bool                    dock_tab_is_visible    :1;             // Is our window visible this frame? ~~ is the corresponding tab selected?
    pub dock_tab_is_visible: bool,
    // bool                    dock_tab_want_close    :1;
    pub dock_tab_want_close: bool,
    // short                   dock_order;                          // Order of the last time the window was visible within its dock_node. This is used to reorder windows that are reappearing on the same frame. Same value between windows that were active and windows that were none are possible.
    pub dock_order: i16,
    // ImGuiWindowDockStyle    dock_style;
    pub dock_style: DimgWindowDockStyle,
    // ImGuiDockNode*          dock_node;                           // Which node are we docked into. Important: Prefer testing dock_is_active in many cases as this will still be set when the dock node is hidden.
    pub dock_node: DimgId, // *mut ImGuiDockNode,
    // ImGuiDockNode*          dock_node_as_host;                     // Which node are we owning (for parent windows)
    pub dock_node_as_host: DimgId, // *mut ImGuiDockNode,
    // ImGuiID                 dock_id;                             // Backup of last valid dock_node->id, so single window remember their dock node id even when they are not bound any more
    pub dock_id: DimgId,
    // ImGuiItemStatusFlags    dock_tab_item_status_flags;
    pub dock_tab_item_status_flags: DimgItemStatusFlags,
    // ImRect                  DockTabItemRect;
    pub DockTabItemRect: DimgRect,
    pub SetWindowDockAllowFlags: DimgCond,

}

impl DimgWindow {
    // // ImGuiWindow is mostly a dumb struct. It merely has a constructor and a few helper methods
    // ImGuiWindow::ImGuiWindow(ImGuiContext* context, const char* name) : DrawListInst(NULL)
    pub unsafe fn new(context: *mut DimgContext, name: &mut String) -> Self {
        let mut out = Self {
            //     name = ImStrdup(name);
            //     NameBufLen = strlen(name) + 1;
            name: name.clone(),
            //     id = ImHashStr(name);
            id: ImHashStr(name.as_vec(), 0),
            //     IDStack.push_back(id);
            id_stack: Vec::new(),
            //     viewport_allow_platform_monitor_extend = -1;
            viewport_allow_platform_monitor_extend: -1,
            //     viewport_pos = ImVec2(FLT_MAX, FLT_MAX);
            viewport_pos: DimgVec2D::new(f32::MAX, f32::MAX),
            //     move_id = GetID("#MOVE");
            move_id: GetID("#MOVE"),
            //     tab_id = GetID("#TAB");
            tab_id: GetID("#TAB"),
            //     scroll_target = ImVec2(FLT_MAX, FLT_MAX);
            scroll_target: DimgVec2D::new(f32::MAX, f32::MAX),
            //     scroll_target_center_ratio = ImVec2(0.5, 0.5);
            scroll_target_center_ratio: DimgVec2D::new(0.5, 0.5),
            //     auto_fit_frames_x = auto_fit_frames_y = -1;
            auto_fit_frames_x: -1,
            auto_fit_frames_y: -1,
            //     auto_pos_last_direction = ImGuiDir_None;
            auto_pos_last_direction: DimgDirection::None,
            //     set_window_pos_allow_flags = set_window_size_allow_flags = set_window_collapsed_allow_flags = SetWindowDockAllowFlags = ImGuiCond_Always | ImGuiCond_Once | ImGuiCond_FirstUseEver | ImGuiCond_Appearing;
            set_window_pos_allow_flags: DimgCond::Always | DimgCond::Once | ImGuiCond::FirstUserEver | DimgCond::Appearing,
            set_window_size_allow_flags: DimgCond::Always | DimgCond::Once | ImGuiCond::FirstUserEver | DimgCond::Appearing,
            set_window_collapsed_allow_flags: DimgCond::Always | DimgCond::Once | ImGuiCond::FirstUserEver | DimgCond::Appearing,
            SetWindowDockAllowFlags: DimgCond::ImGuiCondAlways | DimgCond::Once | ImGuiCond:: FirstUserEver | DimgCond::Appearing,
            //     set_window_pos_val = set_window_pos_pivot = ImVec2(FLT_MAX, FLT_MAX);
            set_window_pos_val: DimgVec2D::new(f32::MAX, f32::MAX),
            set_window_pos_pivot: DimgVec2D::new(f32::MAX, f32::MAX),
            //     last_frame_active = -1;
            last_frame_active: -1,
            //     last_frame_just_focused = -1;
            last_frame_just_focused: -1,
            //     last_time_active = -1.0;
            last_time_active: -1.0,
            //     font_window_scale = font_dpi_scale = 1.0;
            font_window_scale: 1.0,
            font_dpi_scale: 1.0,
            //     settings_offset = -1;
            settings_offset: -1,
            //     dock_order = -1;
            dock_order: -1,
            //     draw_list = &DrawListInst;
            draw_list: DimgDrawList::default(),
            ..Default::default()
        };
        //     memset(this, 0, sizeof(*this));
        &out.id_stack.push(out.id);
        //     draw_list->_Data = &context->draw_list_shared_data;
        &out.draw_list.data = context.draw_list_shared_data.clone();
        //     draw_list->_OwnerName = name;
        &out.draw_list.owner_name = &out.name;
        //     IM_PLACEMENT_NEW(&window_class) ImGuiWindowClass();
        // TODO
        out
    }

    // ImGuiWindow::~ImGuiWindow()
    // {
    //     IM_ASSERT(draw_list == &DrawListInst);
    //     IM_DELETE(name);
    //     ColumnsStorage.clear_destruct();
    // }

    // ImGuiID ImGuiWindow::GetID(const char* str, const char* str_end)
    pub unsafe fn GetID(&mut self, g: &mut DimgContext, in_str: &mut String) -> DimgId {

        // ImGuiID seed = IDStack.back();
        let mut seed = self.id_stack.back();
        // ImGuiID id = ImHashStr(str, str_end ? (str_end - str) : 0, seed);
        let id = ImHashStr(in_str.as_mut_vec(), 0);
        // ImGuiContext& g = *GImGui;
        if g.debug_hook_id_info == id {
            ImGui::DebugHookIdInfo(id, ImGuiDataType_String, str, str_end);
        }
        return id;
    }

    // ImGuiID ImGuiWindow::GetID(const void* ptr)
    pub unsafe fn GetID2(&mut self, g: &mut DimgContext, ptr: &mut Vec<u8>) -> DimgId {
        // ImGuiID seed = IDStack.back();
        let mut seed = self.id_stack.back();
        // ImGuiID id = ImHashData(&ptr, sizeof(void*), seed);
        let mut id = ImHashData(ptr, seed);
        // ImGuiContext& g = *GImGui;
        if (g.debug_hook_id_info == id) {
            ImGui::DebugHookIdInfo(id, ImGuiDataType_Pointer, ptr, NULL);
        }
        return id;
    }

    // ImGuiID ImGuiWindow::GetID(int n)
    pub unsafe fn GetID3(&mut self, g: &mut DimgContext, n: i32) -> DimgId {
        // ImGuiID seed = IDStack.back();
        let mut seed = self.id_stack.back();
        // ImGuiID id = ImHashData(&n, sizeof(n), seed);
        let mut n_bytes: [u8; 4] = [0; 4];
        let n_bytes_raw = n.to_le_bytes();
        n_bytes[0] = n_bytes_raw[0];
        n_bytes[1] = n_bytes_raw[1];
        n_bytes[2] = n_bytes_raw[2];
        n_bytes[3] = n_bytes_raw[3];
        let mut id = ImHashData(&mut n_bytes.into_vec(), seed);
        // TODO
        // ImGuiContext& g = *GImGui;
        if g.debug_hook_id_info == id {
            DebugHookIdInfo(id, ImGuiDataType::S32, n, null());
        }
        return id;
    }

    // This is only used in rare/specific situations to manufacture an id out of nowhere.
    // ImGuiID ImGuiWindow::GetIDFromRectangle(const ImRect& r_abs)
    pub unsafe fn GetIDFromRectangle(&mut self, g: &mut DimgContext, r_abs: &DimgRect) -> DimgId {
        // ImGuiID seed = IDStack.back();
        let seed = self.id_stack.back();
        // ImRect r_rel = ImGui::WindowRectAbsToRel(this, r_abs);
        let r_rel = WindowRectAbsToRel(self, r_abs);
        // ImGuiID id = ImHashData(&r_rel, sizeof(r_rel), seed);
        let id = ImHashData(&r_rel, seed);
        return id;
    }
}

// static void SetCurrentWindow(ImGuiWindow* window)
pub fn SetCurrentWindow(g: &mut DimgContext, window_handle: DimgWindowHandle) {
    // ImGuiContext& g = *GImGui;
    g.current_window = window_handle;
    // if window
    g.current_table = if window_handle.DC.CurrentTableIdx != -1 { g.tables.GetByIndex(window_handle.DC.CurrentTableIdx) } else { null_mut() };
    g.font_size = window_handle.CalcFontSize();
    g.draw_list_shared_data.FontSize = window_handle.CalcFontSize();
}

#[derive(Debug, Clone, Default)]
pub struct DimgWindowDockStyle {
    // ImU32 Colors[ImGuiWindowDockStyleCol_COUNT];
    pub Colors: Vec<u32>,
}

// data saved for each window pushed into the stack
#[derive(Debug, Clone, Default)]
pub struct DimgWindowStackData {
    // ImGuiWindow*            Window;
    pub Window: *mut DimgWindow,
    // ImGuiLastItemData       ParentLastItemDataBackup;
    pub ParentLastItemDataBackup: DimgLastItemData,
    // ImGuiStackSizes         StackSizesOnBegin;      // Store size of various stacks for asserting
    pub StackSizesOnBegin: ImGuiStackSizes,
}


// Transient per-window flags, reset at the beginning of the frame. For child window, inherited from parent on first Begin().
// This is going to be exposed in imgui.h when stabilized enough.
pub enum DimgItemFlags {
    None = 0,
    NoTabStop = 1 << 0,
    // false     // Disable keyboard tabbing (FIXME: should merge with _NoNav)
    ButtonRepeat = 1 << 1,
    // false     // Button() will return true multiple times based on io.key_repeat_delay and io.key_repeat_rate settings.
    Disabled = 1 << 2,
    // false     // Disable interactions but doesn't affect visuals. See BeginDisabled()/EndDisabled(). See github.com/ocornut/imgui/issues/211
    NoNav = 1 << 3,
    // false     // Disable keyboard/gamepad directional navigation (FIXME: should merge with _NoTabStop)
    NoNavDefaultFocus = 1 << 4,
    // false     // Disable item being a candidate for default focus (e.g. used by title bar items)
    SelectableDontClosePopup = 1 << 5,
    // false     // Disable MenuItem/Selectable() automatically closing their popup window
    MixedValue = 1 << 6,
    // false     // [BETA] Represent a mixed/indeterminate value, generally multi-selection where values differ. Currently only supported by Checkbox() (later should support all sorts of widgets)
    ReadOnly = 1 << 7,
    // false     // [ALPHA] Allow hovering interactions but underlying value is not changed.
    Inputable = 1 << 8,   // false     // [WIP] Auto-activate input mode when tab focused. Currently only used and supported by a few items before it becomes a generic feature.
}


// Storage for SetNexWindow** functions
#[derive(Debug, Clone, Default)]
pub struct DimgNextWindowData {
    // ImGuiNextWindowDataFlags    flags;
    pub Flags: ImGuiNextWindowDataFlags,
    // ImGuiCond                   PosCond;
    pub PosCond: DimgCond,
    // ImGuiCond                   SizeCond;
    pub SizeCond: DimgCond,
    // ImGuiCond                   CollapsedCond;
    pub CollapseCond: DimgCond,
    // ImGuiCond                   DockCond;
    pub DockCond: DimgCond,
    // ImVec2                      PosVal;
    pub PosVal: DimgVec2D,
    // ImVec2                      PosPivotVal;
    pub PosPivotVal: DimgVec2D,
    // ImVec2                      SizeVal;
    pub SizeVal: DimgVec2D,
    // ImVec2                      ContentSizeVal;
    pub ContentSizeVal: DimgVec2D,
    // ImVec2                      ScrollVal;
    pub ScrollVal: DimgVec2D,
    // bool                        PosUndock;
    pub PosUndock: bool,
    // bool                        CollapsedVal;
    pub CollapsedVal: bool,
    // ImRect                      SizeConstraintRect;
    pub SizeConstraintRect: DimgRect,
    // ImGuiSizeCallback           SizeCallback;
    pub SizeCallback: ImGuiSizeCallback,
    // void*                       SizeCallbackUserData;
    pub SizeCallbackUserData: Vec<u8>,
    // float                       BgAlphaVal;             // Override background alpha
    pub BgAlphaVal: f32,
    // ImGuiID                     viewport_id;
    pub ViewportId: DimgId,
    // ImGuiID                     dock_id;
    pub DockId: DimgId,
    // ImGuiWindowClass            window_class;
    pub WindowClass: DimgWindowClass,
    // ImVec2                      MenuBarOffsetMinVal;    // (Always on) This is not exposed publicly, so we don't clear it and it doesn't have a corresponding flag (could we? for consistency?)
    pub MenuBarOffsetMinVal: DimgVec2D,

}

impl DimgNextWindowData {
    // ImGuiNextWindowData()       { memset(this, 0, sizeof(*this)); }
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    //     inline void ClearFlags()    { flags = ImGuiNextWindowDataFlags_None; }
    pub fn ClearFlags(&mut self) {
        self.Flags = ImGuiNextWindowDataFlags::None
    }
}


pub enum ImGuiNextWindowDataFlags {
    None = 0,
    HasPos = 1 << 0,
    HasSize = 1 << 1,
    HasContentSize = 1 << 2,
    HasCollapsed = 1 << 3,
    HasSizeConstraint = 1 << 4,
    HasFocus = 1 << 5,
    HasBgAlpha = 1 << 6,
    HasScroll = 1 << 7,
    HasViewport = 1 << 8,
    HasDock = 1 << 9,
    HasWindowClass = 1 << 10,
}


// static inline bool IsWindowContentHoverable(ImGuiWindow* window, ImGuiHoveredFlags flags)
pub fn IsWindowContentHoverable(g: &mut DimgContext, window: &mut DimgWindow, flags: DimgHoveredFlags) -> bool {
    // An active popup disable hovering on other windows (apart from its own children)
    // FIXME-OPT: This could be cached/stored within the window.
    // ImGuiContext& g = *GImGui;
    if g.nav_window {
        if ImGuiWindow * focused_root_window = g.nav_window.RootWindowDockTree {
            if focused_root_window.WasActive && focused_root_window != window.root_window_dock_tree {
                // For the purpose of those flags we differentiate "standard popup" from "modal popup"
                // NB: The order of those two tests is important because Modal windows are also Popups.
                if focused_root_window.Flags & DimgWindowFlags::Modal {
                    return false;
                }
                if (focused_root_window.Flags & DimgWindowFlags::Popup) && !(flags & DimgHoveredFlags::AllowWhenBlockedByPopup) {
                    return false;
                }
            }
        }
    }
    // Filter by viewport
    if window.viewport != g.mouse_viewport {
        if g.moving_window == NULL || window.root_window_dock_tree != g.moving_window.RootWindowDockTree {
            return false;
        }
    }

    return true;
}


// This is roughly matching the behavior of internal-facing ItemHoverable()
// - we allow hovering to be true when active_id==window->MoveID, so that clicking on non-interactive items such as a Text() item still returns true with IsItemHovered()
// - this should work even for non-interactive items that have no id, so we cannot use LastItemId
// bool ImGui::IsItemHovered(ImGuiHoveredFlags flags)
pub fn IsItemHovered(g: &mut DimgContext, flags: &DimgHoveredFlags) -> bool
{
    // ImGuiContext& g = *GImGui;
    let window = &mut g.current_window;
    if g.nav_disable_mouse_hover && !g.NavDisableHighlight && !(flags & DimgHoveredFlags::NoNavOverride)
    {
        if (g.last_item_data.InFlags & ImGuiItemFlags_Disabled) && !(flags & DimgHoveredFlags::AllowWhenDisabled) {
            return false;
        }
        if (!IsItemFocused()) {
            return false;
        }
    }
    else
    {
        // Test for bounding box overlap, as updated as ItemAdd()
        let status_flags = g.last_item_data.StatusFlags;
        if (!(status_flags & DimgItemStatusFlags::HoveredRect)) {
            return false;
        }
        // IM_ASSERT((flags & (ImGuiHoveredFlags_AnyWindow | ImGuiHoveredFlags_RootWindow | ImGuiHoveredFlags_ChildWindows | ImGuiHoveredFlags_NoPopupHierarchy | ImGuiHoveredFlags_DockHierarchy)) == 0);   // flags not supported by this function

        // Test if we are hovering the right window (our window could be behind another window)
        // [2021/03/02] Reworked / reverted the revert, finally. Note we want e.g. BeginGroup/ItemAdd/EndGroup to work as well. (#3851)
        // [2017/10/16] Reverted commit 344d48be3 and testing root_window instead. I believe it is correct to NOT test for root_window but this leaves us unable
        // to use IsItemHovered() after EndChild() itself. Until a solution is found I believe reverting to the test from 2017/09/27 is safe since this was
        // the test that has been running for a long while.
        if (g.hovered_window != window && (status_flags & ImGuiItemStatusFlags_HoveredWindow) == 0) {
            if ((flags & DimgHoveredFlags::AllowWhenOverlapped) == 0) {
                return false;
            }
        }

        // Test if another item is active (e.g. being dragged)
        if ((flags & DimgHoveredFlags::AllowWhenBlockedByActiveItem) == 0) {
            if (g.active_id != 0 && g.active_id != g.last_item_data.ID && !g.active_id_allow_overlap) {
                if (g.active_id != window.MoveId && g.active_id != window.TabId) {
                    return false;
                }
            }
        }

        // Test if interactions on this window are blocked by an active popup or modal.
        // The ImGuiHoveredFlags_AllowWhenBlockedByPopup flag will be tested here.
        if (!IsWindowContentHoverable(g, window, flags)) {
            return false;
        }

        // Test if the item is disabled
        if ((g.last_item_data.InFlags & ImGuiItemFlags_Disabled) && !(flags & DimgHoveredFlags::AllowWhenDisabled)) {
            return false;
        }

        // Special handling for calling after Begin() which represent the title bar or tab.
        // When the window is skipped/collapsed (skip_items==true) that last item (always ->move_id submitted by Begin)
        // will never be overwritten so we need to detect the case.
        if (g.last_item_data.ID == window.MoveId && window.WriteAccessed)
            return false;
    }

    return true;
}

/// [ALPHA] Rarely used / very advanced uses only. Use with SetNextWindowClass() and DockSpace() functions.
/// Important: the content of this class is still highly WIP and likely to change and be refactored
/// before we stabilize Docking features. Please be mindful if using this.
/// Provide hints:
/// - To the platform backend via altered viewport flags (enable/disable OS decoration, OS task bar icons, etc.)
/// - To the platform backend for OS level parent/child relationships of viewport.
/// - To the docking system for various options and filtering.
#[derive(Default,Debug,Clone)]
pub struct DimgWindowClass
{
    pub class_id: DimgId,                  // User data. 0 = Default class (unclassed). windows of different classes cannot be docked with each others.
    pub parent_viewport_id: DimgId,         // Hint for the platform backend. -1: use default. 0: request platform backend to not parent the platform. != 0: request platform backend to create a parent<>child relationship between the platform windows. Not conforming backends are free to e.g. parent every viewport to the main viewport or not.
    pub viewport_flags_override_set: DimgViewportFlags,   // viewport flags to set when a window of this class owns a viewport. This allows you to enforce OS decoration or task bar icon, override the defaults on a per-window basis.
    pub viewport_flags_override_clear: DimgViewportFlags, // viewport flags to clear when a window of this class owns a viewport. This allows you to enforce OS decoration or task bar icon, override the defaults on a per-window basis.
    pub tab_item_flags_override_set: DimgTabItemFlags,    // [EXPERIMENTAL] TabItem flags to set when a window of this class gets submitted into a dock node tab bar. May use with ImGuiTabItemFlags_Leading or ImGuiTabItemFlags_Trailing.
    pub dock_node_flags_override_set: DimgDockNodeFlags,   // [EXPERIMENTAL] Dock node flags to set when a window of this class is hosted by a dock node (it doesn't have to be selected!)
    pub docking_always_tab_bar: bool,        // Set to true to enforce single floating windows of this class always having their own docking node (equivalent of setting the global io.config_docking_always_tab_bar)
    pub docking_allow_unclassed: bool,      // Set to true to allow windows of this class to be docked/merged with an unclassed window. // FIXME-DOCK: Move to DockNodeFlags override?

}

impl DimgWindowClass {
    // ImGuiWindowClass() { memset(this, 0, sizeof(*this)); parent_viewport_id = (ImGuiID)-1; docking_allow_unclassed = true;
    pub fn new() -> Self {
        Self {
            parent_viewport_id: DimgId::MAX,
            docking_allow_unclassed: true,
            ..Default::default()
        }
    }
}

/// windows data saved in imgui.ini file
/// Because we never destroy or rename ImGuiWindowSettings, we can store the names in a separate buffer easily.
/// (this is designed to be stored in a ImChunkStream buffer, with the variable-length name following our structure)
#[derive(Default,Debug,Clone)]
pub struct DimgWindowSettings
{
    //ImGuiID     id;
    pub id: DimgId,
    // ImVec2ih    pos;            // NB: Settings position are stored RELATIVE to the viewport! Whereas runtime ones are absolute positions.
    pub pos: DimgVec2D,
    // ImVec2ih    size;
    pub size: DimgVec2D,
    // ImVec2ih    ViewportPos;
    pub viewport_pos: DimgVec2D,
    // ImGuiID     ViewportId;
    pub viewport_id: DimgId,
    // ImGuiID     DockId;         // id of last known dock_node (even if the dock_node is invisible because it has only 1 active window), or 0 if none.
    pub dock_id: DimgId,
    // ImGuiID     ClassId;        // id of window class if specified
    pub class_id: DimgId,
    // short       DockOrder;      // Order of the last time the window was visible within its dock_node. This is used to reorder windows that are reappearing on the same frame. Same value between windows that were active and windows that were none are possible.
    pub dock_order: i16,
    // bool        Collapsed;
    pub collapsed: bool,
    // bool        WantApply;      // Set when loaded from .ini data (to enable merging/loading .ini data into an already running context)
    pub want_apply: bool,
    // ImGuiWindowSettings()       { memset(this, 0, sizeof(*this)); DockOrder = -1; }
    // char* GetName()             { return (char*)(this + 1); }
}

//     ImGuiWindowFlags_NoDecoration           = ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoResize | ImGuiWindowFlags_NoScrollbar | ImGuiWindowFlags_NoCollapse,
// pub const NoDecoration: i32 = DimgWindowFlags::NoTitleBar | DimgWindowFlags::NoResize | DimgWindowFlags::NoScrollbar | DimgWindowFlags::NoCollapse;
pub const DIMG_WIN_FLAGS_NO_DECORATION: HashSet<DimgWindowFlags> = HashSet::from([
    DimgWindowFlags::NoTitleBar, DimgWindowFlags::NoResize, DimgWindowFlags::NoScrollbar, DimgWindowFlags::NoCollapse
]);

// ImGuiWindowFlags_NoNav                  = ImGuiWindowFlags_NoNavInputs | ImGuiWindowFlags_NoNavFocus,
// pub const ImGuiWindowFlags_NoNav: i32 = DimgWindowFlags::NoNavInputs | DimgWindowFlags::NoNavFocus;
pub const DIMG_WIN_FLAGS_NO_NAV: HashSet<DimgWindowFlags> = HashSet::from([DimgWindowFlags::NoNavInputs, DimgWindowFlags::NoNavFocus]);

//     ImGuiWindowFlags_NoInputs               = ImGuiWindowFlags_NoMouseInputs | ImGuiWindowFlags_NoNavInputs | ImGuiWindowFlags_NoNavFocus,
// pub const NoInputs: i32 = DimgWindowFlags::NoMouseInputs | DimgWindowFlags::NoNavInputs | DimgWindowFlags::NoNavFocus;
pub const DIMG_WIN_FLAGS_NO_INPUTS: HashSet<DimgWindowFlags> = HashSet::from([
    DimgWindowFlags::NoMouseInputs, DimgWindowFlags::NoNavInputs, DimgWindowFlags::NoNavFocus
]);

// When using CTRL+TAB (or Gamepad Square+L/R) we delay the visual a little in order to reduce visual noise doing a fast switch.
// static const float NAV_WINDOWING_HIGHLIGHT_DELAY            = 0.20;    // time before the highlight and screen dimming starts fading in
pub const NAV_WINDOWING_HIGHLIGHT_DELAY: f32 = 0.20;

// static const float NAV_WINDOWING_LIST_APPEAR_DELAY          = 0.15;    // time before the window list starts to appear
pub const NAV_WINDOWING_LIST_APPEAR_DELAY: f32 = 0.15;

// Window resizing from edges (when io.config_windows_resize_from_edges = true and ImGuiBackendFlags_HasMouseCursors is set in io.backend_flags by backend)
// static const float WINDOWS_HOVER_PADDING                    = 4.0;     // Extend outside window for hovering/resizing (maxxed with TouchPadding) and inside windows for borders. Affect FindHoveredWindow().
pub const WINDOWS_HOVER_PADDING: f32 = 4.0;

// static const float WINDOWS_RESIZE_FROM_EDGES_FEEDBACK_TIMER = 0.04;    // Reduce visual noise by only highlighting the border after a certain time.
pub const WINDOWS_RESIZE_FROM_EDGES_FEEDBACK_TIMER: f32 = 0.04;

// typedef void    (*ImGuiSizeCallback)(ImGuiSizeCallbackData* data);              // Callback function for ImGui::SetNextWindowSizeConstraints()
pub type ImGuiSizeCallback = fn(*mut DimgSizeCallbackData);

// flags for ImGui::Begin()
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgWindowFlags
{
    None                   = 0,
    NoTitleBar             = 1 << 0,   // Disable title-bar
    NoResize               = 1 << 1,   // Disable user resizing with the lower-right grip
    NoMove                 = 1 << 2,   // Disable user moving the window
    NoScrollbar            = 1 << 3,   // Disable scrollbars (window can still scroll with mouse or programmatically)
    NoScrollWithMouse      = 1 << 4,   // Disable user vertically scrolling with mouse wheel. On child window, mouse wheel will be forwarded to the parent unless NoScrollbar is also set.
    NoCollapse             = 1 << 5,   // Disable user collapsing window by double-clicking on it. Also referred to as Window Menu Button (e.g. within a docking node).
    AlwaysAutoResize       = 1 << 6,   // Resize every window to its content every frame
    NoBackground           = 1 << 7,   // Disable drawing background color (WindowBg, etc.) and outside border. Similar as using SetNextWindowBgAlpha(0.0).
    NoSavedSettings        = 1 << 8,   // Never load/save settings in .ini file
    NoMouseInputs          = 1 << 9,   // Disable catching mouse, hovering test with pass through.
    MenuBar                = 1 << 10,  // Has a menu-bar
    HorizontalScrollbar    = 1 << 11,  // Allow horizontal scrollbar to appear (off by default). You may use SetNextWindowContentSize(ImVec2(width,0.0)); prior to calling Begin() to specify width. Read code in imgui_demo in the "Horizontal Scrolling" section.
    NoFocusOnAppearing     = 1 << 12,  // Disable taking focus when transitioning from hidden to visible state
    NoBringToFrontOnFocus  = 1 << 13,  // Disable bringing window to front when taking focus (e.g. clicking on it or programmatically giving it focus)
    AlwaysVerticalScrollbar= 1 << 14,  // Always show vertical scrollbar (even if content_size.y < size.y)
    AlwaysHorizontalScrollbar=1<< 15,  // Always show horizontal scrollbar (even if content_size.x < size.x)
    AlwaysUseWindowPadding = 1 << 16,  // Ensure child windows without border uses style.window_padding (ignored by default for non-bordered child windows, because more convenient)
    NoNavInputs            = 1 << 18,  // No gamepad/keyboard navigation within the window
    NoNavFocus             = 1 << 19,  // No focusing toward this window with gamepad/keyboard navigation (e.g. skipped by CTRL+TAB)
    UnsavedDocument        = 1 << 20,  // Display a dot next to the title. When used in a tab/docking context, tab is selected when clicking the x + closure is not assumed (will wait for user to stop submitting the tab). Otherwise closure is assumed when pressing the x, so if you keep submitting the tab may reappear at end of tab bar.
    NoDocking              = 1 << 21,  // Disable docking of this window
    // [Internal]
    NavFlattened           = 1 << 23,  // [BETA] On child window: allow gamepad/keyboard navigation to cross over parent border to this child or between sibling child windows.
    ChildWindow            = 1 << 24,  // Don't use! For internal use by BeginChild()
    Tooltip                = 1 << 25,  // Don't use! For internal use by BeginTooltip()
    Popup                  = 1 << 26,  // Don't use! For internal use by BeginPopup()
    Modal                  = 1 << 27,  // Don't use! For internal use by BeginPopupModal()
    ChildMenu              = 1 << 28,  // Don't use! For internal use by BeginMenu()
    DockNodeHost           = 1 << 29   // Don't use! For internal use by Begin()/NewFrame()
    // [Obsolete]
    //ImGuiWindowFlags_ResizeFromAnySide    = 1 << 17,  // [Obsolete] --> Set io.config_windows_resize_from_edges=true and make sure mouse cursors are supported by backend (io.backend_flags & ImGuiBackendFlags_HasMouseCursors)
}

// flags for ImGui::IsWindowFocused()
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgFocusedFlags
{
    None                          = 0,
    ChildWindows                  = 1 << 0,   // Return true if any children of the window is focused
    RootWindow                    = 1 << 1,   // Test from root window (top most parent of the current hierarchy)
    AnyWindow                     = 1 << 2,   // Return true if any window is focused. Important: If you are trying to tell how to dispatch your low-level inputs, do NOT use this. Use 'io.want_capture_mouse' instead! Please read the FAQ!
    NoPopupHierarchy              = 1 << 3,   // Do not consider popup hierarchy (do not treat popup emitter as parent of popup) (when used with _ChildWindows or _RootWindow)
    DockHierarchy                 = 1 << 4,   // Consider docking hierarchy (treat dockspace host as parent of docked window) (when used with _ChildWindows or _RootWindow)
    // ImGuiFocusedFlags_RootAndChildWindows           = ImGuiFocusedFlags_RootWindow | ImGuiFocusedFlags_ChildWindows
}


// flags for ImGui::IsItemHovered(), ImGui::IsWindowHovered()
// Note: if you are trying to check whether your mouse should be dispatched to Dear ImGui or to your app, you should use 'io.want_capture_mouse' instead! Please read the FAQ!
// Note: windows with the ImGuiWindowFlags_NoInputs flag are ignored by IsWindowHovered() calls.
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum DimgHoveredFlags
{
    None                          = 0,        // Return true if directly over the item/window, not obstructed by another window, not obstructed by an active popup or modal blocking inputs under them.
    ChildWindows                  = 1 << 0,   // IsWindowHovered() only: Return true if any children of the window is hovered
    RootWindow                    = 1 << 1,   // IsWindowHovered() only: Test from root window (top most parent of the current hierarchy)
    AnyWindow                     = 1 << 2,   // IsWindowHovered() only: Return true if any window is hovered
    NoPopupHierarchy              = 1 << 3,   // IsWindowHovered() only: Do not consider popup hierarchy (do not treat popup emitter as parent of popup) (when used with _ChildWindows or _RootWindow)
    DockHierarchy                 = 1 << 4,   // IsWindowHovered() only: Consider docking hierarchy (treat dockspace host as parent of docked window) (when used with _ChildWindows or _RootWindow)
    AllowWhenBlockedByPopup       = 1 << 5,   // Return true even if a popup window is normally blocking access to this item/window
    //ImGuiHoveredFlags_AllowWhenBlockedByModal     = 1 << 6,   // Return true even if a modal popup window is normally blocking access to this item/window. FIXME-TODO: Unavailable yet.
    AllowWhenBlockedByActiveItem  = 1 << 7,   // Return true even if an active item is blocking access to this item/window. Useful for Drag and Drop patterns.
    AllowWhenOverlapped           = 1 << 8,   // IsItemHovered() only: Return true even if the position is obstructed or overlapped by another window
    AllowWhenDisabled             = 1 << 9,   // IsItemHovered() only: Return true even if the item is disabled
    NoNavOverride                 = 1 << 10,  // Disable using gamepad/keyboard navigation state when active, always query mouse.
    // ImGuiHoveredFlags_RectOnly                      = ImGuiHoveredFlags_AllowWhenBlockedByPopup | ImGuiHoveredFlags_AllowWhenBlockedByActiveItem | ImGuiHoveredFlags_AllowWhenOverlapped,
    // ImGuiHoveredFlags_RootAndChildWindows           = ImGuiHoveredFlags_RootWindow | ImGuiHoveredFlags_ChildWindows
}

// pub const RootAndChildWindows: i32           = DimgHoveredFlags::RootWindow | DimgHoveredFlags::ChildWindows;
pub const ROOT_AND_CHILD_WINDOWS: HashSet<DimgHoveredFlags> = HashSet::from([
    DimgHoveredFlags::RootWindow, DimgHoveredFlags::ChildWindows
]);


// pub const RectOnly : i32                     = DimgHoveredFlags::AllowWhenBlockedByPopup | DimgHoveredFlags::AllowWhenBlockedByActiveItem | DimgHoveredFlags::AllowWhenOverlapped;
 pub const RECT_ONLY: HashSet<DimgHoveredFlags> = HashSet::from([
     DimgHoveredFlags::AllowWhenBlockedByPopup, DimgHoveredFlags::AllowWhenBlockedByActiveItem, DimgHoveredFlags::AllowWhenOverlapped
 ]);

// Resizing callback data to apply custom constraint. As enabled by SetNextWindowSizeConstraints(). Callback is called during the next Begin().
// NB: For basic min/max size constraint on each axis you don't need to use the callback! The SetNextWindowSizeConstraints() parameters are enough.
pub struct DimgSizeCallbackData
{
    // void*   user_data;       // Read-only.   What user passed to SetNextWindowSizeConstraints()
    pub user_data: Vec<u8>,
    // pub pos: ImVec2,            // Read-only.   Window position, for reference.
    pub pos: DimgVec2D,
    // pub current_size: ImVec2,    // Read-only.   Current window size.
    pub current_size: DimgVec2D,
    // pub desired_size: ImVec2,    // Read-write.  Desired size, based on user's mouse position. Write to this field to restrain resizing.
    pub desired_size: DimgVec2D,
}

#[derive(Debug,Clone,Default)]
pub struct DimgShrinkWidthItem
{
    // int         Index;
    pub Index: i32,
    // float       width;
    pub Width: f32,
    // float       InitialWidth;
    pub InitialWidth: f32,
}
