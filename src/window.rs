use std::borrow::Borrow;
use std::cell::RefCell;
use std::os::raw::c_char;
use std::ptr::{null, null_mut};
use std::rc::Rc;
use std::collections::HashSet;
use crate::column::OldColumns;
use crate::condition::Cond;
use crate::config::ConfigFlags;
use crate::context::Context;
use crate::defines::ImGuiSizeCallback;
use crate::direction::Direction;
use crate::dock::DockNodeFlags;
use crate::dock_node::{dock_node_get_root_node, DockNode};
use crate::drag_drop::DragDropFlags;
use crate::draw_list::{add_draw_list_to_draw_data, DrawList};
use crate::globals::GImGui;
use crate::hash::{hash_string, ImHashData};
use crate::id::set_active_id;
use crate::input::NavLayer;
use crate::item::{ItemStatusFlags, LastItemData};
use crate::kv_store::Storage;
use crate::layout::LayoutType;
use crate::menu::ImGuiMenuColumns;
use crate::rect::Rect;
use crate::size_callback_data::SizeCallbackData;
use crate::stack::ImGuiStackSizes;
use crate::tab_bar::DimgTabItemFlags;
use crate::vectors::{Vector1D, Vector2D};
use crate::types::{Id32, ImGuiDataType, INVALID_ID, WindowHandle};
use crate::utils::remove_hash_set_val;
use crate::viewport::{Viewport, ViewportFlags};


// Transient per-window data, reset at the beginning of the frame. This used to be called ImGuiDrawContext, hence the dc variable name in ImGuiWindow.
// (That's theory, in practice the delimitation between ImGuiWindow and ImGuiWindowTempData is quite tenuous and could be reconsidered..)
// (This doesn't need a constructor because we zero-clear it as part of ImGuiWindow and all frame-temporary data are setup on Begin)
pub struct WindowTempData {
    // Layout
    // Vector2D                  CursorPos;              // Current emitting position, in absolute coordinates.
    pub cursor_pos: Vector2D,
    // Vector2D                  CursorPosPrevLine;
    pub cursor_pos_prev_line: Vector2D,
    // Vector2D                  CursorStartPos;         // Initial position after Begin(), generally ~ window position + window_padding.
    pub cursor_start_pos: Vector2D,
    // Vector2D                  CursorMaxPos;           // Used to implicitly calculate content_size at the beginning of next frame, for scrolling range and auto-resize. Always growing during the frame.
    pub cursor_max_pos: Vector2D,
    // Vector2D                  IdealMaxPos;            // Used to implicitly calculate content_size_ideal at the beginning of next frame, for auto-resize only. Always growing during the frame.
    pub ideal_max_pos: Vector2D,
    // Vector2D                  CurrLineSize;
    pub curr_line_size: Vector2D,
    // Vector2D                  PrevLineSize;
    pub prev_line_size: Vector2D,
    // float                   CurrLineTextBaseOffset; // Baseline offset (0.0 by default on a new line, generally == style.FramePadding.y when a framed item has been added).
    pub curr_line_text_base_offset: f32,
    // float                   PrevLineTextBaseOffset;
    pub prev_line_text_base_offset: f32,
    // bool                    IsSameLine;
    pub is_same_line: bool,
    // ImVec1                  Indent;                 // Indentation / start position from left of window (increased by TreePush/TreePop, etc.)
    pub indent: Vector1D,
    // ImVec1                  ColumnsOffset;          // Offset to the current column (if ColumnsCurrent > 0). FIXME: This and the above should be a stack to allow use cases like Tree->column->Tree. Need revamp columns API.
    pub columns_offset: Vector1D,
    // ImVec1                  GroupOffset;
    pub group_offset: Vector1D,
    // Vector2D                  CursorStartPosLossyness;// Record the loss of precision of CursorStartPos due to really large scrolling amount. This is used by clipper to compensentate and fix the most common use case of large scroll area.
    pub cursort_start_pos_lossyness: Vector2D,
    // Keyboard/Gamepad navigation
    // ImGuiNavLayer           NavLayerCurrent;        // Current layer, 0..31 (we currently only use 0..1)
    pub nav_layer_current: NavLayer,
    // short                   nav_layers_active_mask;    // Which layers have been written to (result from previous frame)
    pub nav_layers_active_mask: i16,
    // short                   nav_layers_active_mask_next;// Which layers have been written to (accumulator for current frame)
    pub nav_layers_active_mask_next: i16,
    // ImGuiID                 nav_focus_scope_id_current; // Current focus scope id while appending
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
    pub menu_columns: ImGuiMenuColumns,
    // int                     tree_depth;              // Current tree depth.
    pub tree_depth: i32,
    // ImU32                   tree_jump_to_parent_on_pop_mask; // Store a copy of !g.nav_id_is_alive for tree_depth 0..31.. Could be turned into a ImU64 if necessary.
    pub tree_jump_to_parent_on_pop_mask: u32,
    // ImVector<ImGuiWindow*>  ChildWindows;
    pub child_windows: Vec<Id32>,
    // ImGuiStorage*           state_storage;           // Current persistent per-window storage (store e.g. tree node open/close state)
    pub state_storage: Vec<u8>,
    // ImGuiOldColumns*        current_columns;         // Current columns set
    pub current_columns: OldColumns,
    // int                     current_table_idx;        // Current table index (into g.tables)
    pub current_table_idx: usize,
    // ImGuiLayoutType         layout_type;
    pub layout_type: LayoutType,
    // ImGuiLayoutType         parent_layout_type;       // Layout type of parent window at the time of Begin()
    pub parent_layout_type: LayoutType,
    // Local parameters stacks
    // We store the current settings outside of the vectors to increase memory locality (reduce cache misses). The vectors are rarely modified. Also it allows us to not heap allocate for short-lived windows which are not using those settings.
    // float                   item_width;              // Current item width (>0.0: width in pixels, <0.0: align xx pixels to the right of window).
    pub item_width: f32,
    // float                   text_wrap_pos;            // Current text wrap pos.
    pub text_wrap_pos: f32,
    // ImVector<float>         item_width_stack;         // Store item widths to restore (attention: .back() is not == item_width)
    pub item_width_stack: Vec<f32>,
    // ImVector<float>         text_wrap_pos_stack;       // Store text wrap pos to restore (attention: .back() is not == text_wrap_pos)
    pub text_wrap_pos_stack: Vec<f32>,
}


// Storage for one window
#[derive(Default,Debug,Clone)]
pub struct Window {
    // char*                   name;                               // Window name, owned by the window.
    pub name: String,
    //*mut c_char,
    // ImGuiID                 id;                                 // == ImHashStr(name)
    pub id: Id32,
    // ImGuiWindowFlags        flags, flags_previous_frame;          // See enum ImGuiWindowFlags_
    pub flags: HashSet<WindowFlags>,
    pub flags_previous_frame: HashSet<WindowFlags>,
    // ImGuiWindowClass        window_class;                        // Advanced users only. Set with SetNextWindowClass()
    pub window_class: WindowClass,
    // ImGuiViewportP*         viewport;                           // Always set in Begin(). Inactive windows may have a NULL value here if their viewport was discarded.
    pub viewport: Id32,
    // ImGuiID                 viewport_id;                         // We backup the viewport id (since the viewport may disappear or never be created if the window is inactive)
    pub viewport_id: Id32,
    // Vector2D                  viewport_pos;                        // We backup the viewport position (since the viewport may disappear or never be created if the window is inactive)
    pub viewport_pos: Vector2D,
    // int                     viewport_allow_platform_monitor_extend; // Reset to -1 every frame (index is guaranteed to be valid between NewFrame..EndFrame), only used in the appearing frame of a tooltip/popup to enforce clamping to a given monitor
    pub viewport_allow_platform_monitor_extend: i32,
    // Vector2D                  pos;                                // Position (always rounded-up to nearest pixel)
    pub pos: Vector2D,
    // Vector2D                  size;                               // Current size (==size_full or collapsed title bar size)
    pub size: Vector2D,
    // Vector2D                  size_full;                           // size when non collapsed
    pub size_full: Vector2D,
    // Vector2D                  content_size;                        // size of contents/scrollable client area (calculated from the extents reach of the cursor) from previous frame. Does not include window decoration or window padding.
    pub content_size: Vector2D,
    // Vector2D                  content_size_ideal;
    pub content_size_ideal: Vector2D,
    // Vector2D                  content_size_explicit;                // size of contents/scrollable client area explicitly request by the user via SetNextWindowContentSize().
    pub content_size_explicit: Vector2D,
    // Vector2D                  window_padding;                      // Window padding at the time of Begin().
    pub window_padding: Vector2D,
    // float                   window_rounding;                     // Window rounding at the time of Begin(). May be clamped lower to avoid rendering artifacts with title bar, menu bar etc.
    pub window_rounding: f32,
    // float                   WindowBorderSize;                   // Window border size at the time of Begin().
    // int                     NameBufLen;                         // size of buffer storing name. May be larger than strlen(name)!
    // ImGuiID                 move_id;                             // == window->GetID("#MOVE")
    pub move_id: Id32,
    // ImGuiID                 tab_id;                              // == window->GetID("#TAB")
    pub tab_id: Id32,
    // ImGuiID                 child_id;                            // id of corresponding item in parent window (for navigation to return from child window to parent window)
    pub child_id: Id32,
    // Vector2D                  scroll;
    pub scroll: Vector2D,
    // Vector2D                  scroll_max;
    pub scroll_max: Vector2D,
    // Vector2D                  scroll_target;                       // target scroll position. stored as cursor position with scrolling canceled out, so the highest point is always 0.0. (FLT_MAX for no change)
    pub scroll_target: Vector2D,
    // Vector2D                  scroll_target_center_ratio;            // 0.0 = scroll so that target position is at top, 0.5 = scroll so that target position is centered
    pub scroll_target_center_ratio: Vector2D,
    // Vector2D                  scroll_target_edge_snap_dist;           // 0.0 = no snapping, >0.0 snapping threshold
    pub scroll_target_edge_snap_dist: Vector2D,
    // Vector2D                  scrollbar_sizes;                     // size taken by each scrollbars on their smaller axis. Pay attention! scrollbar_sizes.x == width of the vertical scrollbar, scrollbar_sizes.y = height of the horizontal scrollbar.
    pub scrollbar_sizes: Vector2D,
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
    pub popup_id: Id32,
    // ImS8                    auto_fit_frames_x, auto_fit_frames_y;
    pub auto_fit_frames_x: i8,
    pub auto_fit_frames_y: i8,
    // ImS8                    auto_fit_child_axises;
    pub auto_fit_child_axises: i8,
    // bool                    auto_fit_only_grows;
    pub auto_fit_only_grows: bool,
    // ImGuiDir                auto_pos_last_direction;
    pub auto_pos_last_direction: Direction,
    // ImS8                    hidden_frames_can_skip_items;           // Hide the window for N frames
    pub hidden_frames_can_skip_items: i8,
    // ImS8                    hidden_frames_cannot_skip_items;        // Hide the window for N frames while allowing items to be submitted so we can measure their size
    pub hidden_frames_cannot_skip_items: i8,
    // ImS8                    hidden_frames_for_render_only;          // Hide the window until frame N at Render() time only
    pub hidden_frames_for_render_only: i8,
    // ImS8                    disable_inputs_frames;                // Disable window interactions for N frames
    pub disable_inputs_frames: i8,
    // ImGuiCond               set_window_pos_allow_flags : 8;         // store acceptable condition flags for SetNextWindowPos() use.
    pub set_window_pos_allow_flags: Cond,
    // ImGuiCond               set_window_size_allow_flags : 8;        // store acceptable condition flags for SetNextWindowSize() use.
    pub set_window_size_allow_flags: Cond,
    // ImGuiCond               set_window_collapsed_allow_flags : 8;   // store acceptable condition flags for SetNextWindowCollapsed() use.
    pub set_window_collapsed_allow_flags: Cond,
    // ImGuiCond               set_window_dock_allow_flags : 8;        // store acceptable condition flags for SetNextWindowDock() use.
    // Vector2D                  set_window_pos_val;                    // store window position when using a non-zero Pivot (position set needs to be processed when we know the window size)
    pub set_window_pos_val: Vector2D,
    // Vector2D                  set_window_pos_pivot;                  // store window pivot for positioning. Vector2D(0, 0) when positioning from top-left corner; Vector2D(0.5, 0.5) for centering; Vector2D(1, 1) for bottom right.
    pub set_window_pos_pivot: Vector2D,

    // ImVector<ImGuiID>       IDStack;                            // id stack. id are hashes seeded with the value at the top of the stack. (In theory this should be in the TempData structure)
    pub id_stack: Vec<Id32>,
    // ImGuiWindowTempData     dc;                                 // Temporary per-window data, reset at the beginning of the frame. This used to be called ImGuiDrawContext, hence the "dc" variable name.
    pub dc: WindowTempData,

    // The best way to understand what those rectangles are is to use the 'Metrics->Tools->Show windows Rectangles' viewer.
    // The main 'OuterRect', omitted as a field, is window->rect().
    // ImRect                  outer_rect_clipped;                   // == Window->rect() just after setup in Begin(). == window->rect() for root window.
    pub outer_rect_clipped: Rect,
    // ImRect                  inner_rect;                          // Inner rectangle (omit title bar, menu bar, scroll bar)
    pub inner_rect: Rect,
    // ImRect                  inner_clip_rect;                      // == inner_rect shrunk by window_padding*0.5 on each side, clipped within viewport or parent clip rect.
    pub inner_clip_rect: Rect,
    // ImRect                  work_rect;                           // Initially covers the whole scrolling region. Reduced by containers e.g columns/tables when active. Shrunk by window_padding*1.0 on each side. This is meant to replace content_region_rect over time (from 1.71+ onward).
    pub work_rect: Rect,
    // ImRect                  parent_work_rect;                     // Backup of work_rect before entering a container such as columns/tables. Used by e.g. SpanAllColumns functions to easily access. Stacked containers are responsible for maintaining this. // FIXME-WORKRECT: Could be a stack?
    pub parent_work_rect: Rect,
    // ImRect                  clip_rect;                           // Current clipping/scissoring rectangle, evolve as we are using push_clip_rect(), etc. == draw_list->clip_rect_stack.back().
    pub clip_rect: Rect,
    // ImRect                  content_region_rect;                  // FIXME: This is currently confusing/misleading. It is essentially work_rect but not handling of scrolling. We currently rely on it as right/bottom aligned sizing operation need some size to rely on.
    pub content_region_rect: Rect,
    // Vector2Dih                hit_test_hole_size;                    // Define an optional rectangular hole where mouse will pass-through the window.
    pub hit_test_hole_size: Vector2D,
    // Vector2Dih                hit_test_hole_offset;
    pub hit_test_hole_offset: Vector2D,
    // int                     last_frame_active;                    // Last frame number the window was active.
    pub last_frame_active: i32,
    // int                     last_frame_just_focused;               // Last frame number the window was made Focused.
    pub last_frame_just_focused: i32,
    // float                   last_time_active;                     // Last timestamp the window was active (using float as we don't need high precision there)
    pub last_time_active: f32,
    // float                   item_width_default;
    pub item_width_default: f32,
    // ImGuiStorage            state_storage;
    pub state_storage: Storage,
    // ImVector<ImGuiOldColumns> ColumnsStorage;
    pub column: Vec<OldColumns>,
    // float                   font_window_scale;                    // User scale multiplier per-window, via SetWindowFontScale()
    pub font_window_scale: f32,
    // float                   font_dpi_scale;
    pub font_dpi_scale: f32,
    // int                     settings_offset;                     // Offset into settings_windows[] (offsets are always valid as we only grow the array from the back)
    pub settings_offset: i32,
    // ImDrawList*             draw_list;                           // == &DrawListInst (for backward compatibility reason with code using imgui_internal.h we keep this a pointer)
    pub draw_list: Id32,
    // ImDrawList              DrawListInst;
    pub draw_list_inst: Id32,
    // ImGuiWindow*            ParentWindow;                       // If we are a child _or_ popup _or_ docked window, this is pointing to our parent. Otherwise NULL.
    pub parent_window_id: WindowHandle,
    // ImGuiWindow*            parent_window_in_begin_stack;
    pub parent_window_in_begin_stack: WindowHandle,
    // ImGuiWindow*            root_window;                         // Point to ourself or first ancestor that is not a child window. Doesn't cross through popups/dock nodes.
    pub root_window: WindowHandle,
    // ImGuiWindow*            root_window_popup_tree;                // Point to ourself or first ancestor that is not a child window. Cross through popups parent<>child.
    pub root_window_popup_tree: WindowHandle,
    // ImGuiWindow*            root_window_dock_tree;                 // Point to ourself or first ancestor that is not a child window. Cross through dock nodes.
    pub root_window_dock_tree: WindowHandle,
    // ImGuiWindow*            root_window_for_title_bar_highlight;     // Point to ourself or first ancestor which will display TitleBgActive color when this window is active.
    pub root_window_for_title_bar_highlight: WindowHandle,
    // ImGuiWindow*            root_window_for_nav;                   // Point to ourself or first ancestor which doesn't have the NavFlattened flag.
    pub root_window_for_nav: WindowHandle,
    // ImGuiWindow*            nav_last_child_nav_window;              // When going to the menu bar, we remember the child window we came from. (This could probably be made implicit if we kept g.windows sorted by last focused including child window.)
    pub nav_last_child_nav_window: WindowHandle,
    // ImGuiID                 nav_last_ids[ImGuiNavLayer_COUNT];    // Last known nav_id for this window, per layer (0/1)
    pub nav_last_ids: Vec<Id32>,
    // ImRect                  nav_rect_rel[ImGuiNavLayer_COUNT];    // Reference rectangle, in window relative space
    pub nav_rect_rel: Vec<Rect>,
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
    pub dock_style: WindowDockStyle,
    // ImGuiDockNode*          dock_node;                           // Which node are we docked into. Important: Prefer testing dock_is_active in many cases as this will still be set when the dock node is hidden.
    pub dock_node: DockNode, //Id32, // *mut ImGuiDockNode,
    // ImGuiDockNode*          dock_node_as_host;                     // Which node are we owning (for parent windows)
    pub dock_node_as_host: DockNode,// Id32, // *mut ImGuiDockNode,
    // ImGuiID                 dock_id;                             // Backup of last valid dock_node->id, so single window remember their dock node id even when they are not bound any more
    pub dock_id: Id32,
    // ImGuiItemStatusFlags    dock_tab_item_status_flags;
    pub dock_tab_item_status_flags: HashSet<ItemStatusFlags>,
    // ImRect                  dock_tab_item_rect;
    pub dock_tab_item_rect: Rect,
    pub set_window_dock_allow_flags: Cond,

}

impl Window {
    // // ImGuiWindow is mostly a dumb struct. It merely has a constructor and a few helper methods
    // ImGuiWindow::ImGuiWindow(ImGuiContext* context, const char* name) : DrawListInst(NULL)
    pub unsafe fn new(context: *mut Context, name: &mut String) -> Self {
        let mut out = Self {
            //     name = ImStrdup(name);
            //     NameBufLen = strlen(name) + 1;
            name: name.clone(),
            //     id = ImHashStr(name);
            id: hash_string(name.as_vec(), 0),
            //     IDStack.push_back(id);
            id_stack: Vec::new(),
            //     viewport_allow_platform_monitor_extend = -1;
            viewport_allow_platform_monitor_extend: -1,
            //     viewport_pos = Vector2D(FLT_MAX, FLT_MAX);
            viewport_pos: Vector2D::new(f32::MAX, f32::MAX),
            //     move_id = GetID("#MOVE");
            move_id: GetID("#MOVE"),
            //     tab_id = GetID("#TAB");
            tab_id: GetID("#TAB"),
            //     scroll_target = Vector2D(FLT_MAX, FLT_MAX);
            scroll_target: Vector2D::new(f32::MAX, f32::MAX),
            //     scroll_target_center_ratio = Vector2D(0.5, 0.5);
            scroll_target_center_ratio: Vector2D::new(0.5, 0.5),
            //     auto_fit_frames_x = auto_fit_frames_y = -1;
            auto_fit_frames_x: -1,
            auto_fit_frames_y: -1,
            //     auto_pos_last_direction = ImGuiDir_None;
            auto_pos_last_direction: Direction::None,
            //     set_window_pos_allow_flags = set_window_size_allow_flags = set_window_collapsed_allow_flags = set_window_dock_allow_flags = Cond::Always | ImGuiCond_Once | ImGuiCond_FirstUseEver | ImGuiCond_Appearing;
            set_window_pos_allow_flags: Cond::Always | Cond::Once | Cond::FirstUserEver | Cond::Appearing,
            set_window_size_allow_flags: Cond::Always | Cond::Once | Cond::FirstUserEver | Cond::Appearing,
            set_window_collapsed_allow_flags: Cond::Always | Cond::Once | Cond::FirstUserEver | Cond::Appearing,
            set_window_dock_allow_flags: Cond::ImGuiCondAlways | Cond::Once | Cond:: FirstUserEver | Cond::Appearing,
            //     set_window_pos_val = set_window_pos_pivot = Vector2D(FLT_MAX, FLT_MAX);
            set_window_pos_val: Vector2D::new(f32::MAX, f32::MAX),
            set_window_pos_pivot: Vector2D::new(f32::MAX, f32::MAX),
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
            draw_list: DrawList::default(),
            ..Default::default()
        };
        //     memset(this, 0, sizeof(*this));
        &out.id_stack.push(out.id);
        //     draw_list->_Data = &context->draw_list_shared_data;
        out.draw_list.data = context.draw_list_shared_data.clone();
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
    pub unsafe fn GetID(&mut self, g: &mut Context, in_str: &mut String) -> Id32 {

        // ImGuiID seed = IDStack.back();
        let mut seed = self.id_stack.back();
        // ImGuiID id = ImHashStr(str, str_end ? (str_end - str) : 0, seed);
        let id = hash_string(in_str.as_mut_vec(), 0);
        // ImGuiContext& g = *GImGui;
        if g.debug_hook_id_info == id {
            ImGui::DebugHookIdInfo(id, ImGuiDataType_String, str, str_end);
        }
        return id;
    }

    // ImGuiID ImGuiWindow::GetID(const void* ptr)
    pub unsafe fn GetID2(&mut self, g: &mut Context, ptr: &mut Vec<u8>) -> Id32 {
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
    pub unsafe fn GetID3(&mut self, g: &mut Context, n: i32) -> Id32 {
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
    pub unsafe fn GetIDFromRectangle(&mut self, g: &mut Context, r_abs: &Rect) -> Id32 {
        // ImGuiID seed = IDStack.back();
        let seed = self.id_stack.back();
        // ImRect r_rel = ImGui::WindowRectAbsToRel(this, r_abs);
        let r_rel = WindowRectAbsToRel(self, r_abs);
        // ImGuiID id = ImHashData(&r_rel, sizeof(r_rel), seed);
        let id = ImHashData(&r_rel, seed);
        return id;
    }

    pub fn get_node(&mut self, node_id: Id32) -> &mut DockNode {
        for n in self.node
    }
}

// static void set_current_window(ImGuiWindow* window)
pub fn set_current_window(ctx: &mut Context, window_handle: WindowHandle) {
    // ImGuiContext& g = *GImGui;
    ctx.current_window_id = window_handle;
    // if window
    ctx.current_table = if window_handle.DC.CurrentTableIdx != -1 { ctx.tables.get_by_index(window_handle.DC.CurrentTableIdx) } else { null_mut() };
    ctx.font_size = window_handle.CalcFontSize();
    ctx.draw_list_shared_data.font_size = window_handle.CalcFontSize();
}

#[derive(Debug, Clone, Default)]
pub struct WindowDockStyle {
    // ImU32 Colors[ImGuiWindowDockStyleCol_COUNT];
    pub Colors: Vec<u32>,
}

// data saved for each window pushed into the stack
#[derive(Debug, Clone, Default)]
pub struct WindowStackData {
    // ImGuiWindow*            Window;
    pub Window: *mut Window,
    // ImGuiLastItemData       ParentLastItemDataBackup;
    pub ParentLastItemDataBackup: LastItemData,
    // ImGuiStackSizes         StackSizesOnBegin;      // Store size of various stacks for asserting
    pub StackSizesOnBegin: ImGuiStackSizes,
}


// Transient per-window flags, reset at the beginning of the frame. For child window, inherited from parent on first Begin().
// This is going to be exposed in imgui.h when stabilized enough.
pub enum ItemFlags {
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
pub struct NextWindowData {
    // ImGuiNextWindowDataFlags    flags;
    pub Flags: ImGuiNextWindowDataFlags,
    // ImGuiCond                   PosCond;
    pub PosCond: Cond,
    // ImGuiCond                   SizeCond;
    pub SizeCond: Cond,
    // ImGuiCond                   CollapsedCond;
    pub CollapseCond: Cond,
    // ImGuiCond                   DockCond;
    pub DockCond: Cond,
    // Vector2D                      PosVal;
    pub PosVal: Vector2D,
    // Vector2D                      PosPivotVal;
    pub PosPivotVal: Vector2D,
    // Vector2D                      SizeVal;
    pub SizeVal: Vector2D,
    // Vector2D                      ContentSizeVal;
    pub ContentSizeVal: Vector2D,
    // Vector2D                      ScrollVal;
    pub ScrollVal: Vector2D,
    // bool                        PosUndock;
    pub PosUndock: bool,
    // bool                        CollapsedVal;
    pub CollapsedVal: bool,
    // ImRect                      SizeConstraintRect;
    pub SizeConstraintRect: Rect,
    // ImGuiSizeCallback           SizeCallback;
    pub SizeCallback: ImGuiSizeCallback,
    // void*                       SizeCallbackUserData;
    pub SizeCallbackUserData: Vec<u8>,
    // float                       BgAlphaVal;             // Override background alpha
    pub BgAlphaVal: f32,
    // ImGuiID                     viewport_id;
    pub ViewportId: Id32,
    // ImGuiID                     dock_id;
    pub DockId: Id32,
    // ImGuiWindowClass            window_class;
    pub WindowClass: WindowClass,
    // Vector2D                      MenuBarOffsetMinVal;    // (Always on) This is not exposed publicly, so we don't clear it and it doesn't have a corresponding flag (could we? for consistency?)
    pub MenuBarOffsetMinVal: Vector2D,

}

impl NextWindowData {
    // ImGuiNextWindowData()       { memset(this, 0, sizeof(*this)); }
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    //     inline void ClearFlags()    { flags = ImGuiNextWindowDataFlags_None; }
    pub fn ClearFlags(&mut self) {
        self.flags = ImGuiNextWindowDataFlags::None
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
pub fn is_window_content_hoverable(g: &mut Context, window: &mut Window, flags: HoveredFlags) -> bool {
    // An active popup disable hovering on other windows (apart from its own children)
    // FIXME-OPT: This could be cached/stored within the window.
    // ImGuiContext& g = *GImGui;
    if g.nav_window_id {
        if ImGuiWindow * focused_root_window = g.nav_window_id.RootWindowDockTree {
            if focused_root_window.was_active && focused_root_window != window.root_window_dock_tree {
                // For the purpose of those flags we differentiate "standard popup" from "modal popup"
                // NB: The order of those two tests is important because Modal windows are also Popups.
                if focused_root_window.flags & WindowFlags::Modal {
                    return false;
                }
                if (focused_root_window.flags & WindowFlags::Popup) && !(flags & HoveredFlags::AllowWhenBlockedByPopup) {
                    return false;
                }
            }
        }
    }
    // Filter by viewport
    if window.viewport != g.mouse_viewport_id {
        if g.moving_window_id == NULL || window.root_window_dock_tree != g.moving_window_id.RootWindowDockTree {
            return false;
        }
    }

    return true;
}


// This is roughly matching the behavior of internal-facing ItemHoverable()
// - we allow hovering to be true when active_id==window->MoveID, so that clicking on non-interactive items such as a Text() item still returns true with IsItemHovered()
// - this should work even for non-interactive items that have no id, so we cannot use LastItemId
// bool ImGui::IsItemHovered(ImGuiHoveredFlags flags)
pub fn IsItemHovered(g: &mut Context, flags: &HoveredFlags) -> bool
{
    // ImGuiContext& g = *GImGui;
    let window = &mut g.current_window_id;
    if g.nav_disable_mouse_hover && !g.NavDisableHighlight && !(flags & HoveredFlags::NoNavOverride)
    {
        if (g.last_item_data.in_flags & ImGuiItemFlags_Disabled) && !(flags & HoveredFlags::AllowWhenDisabled) {
            return false;
        }
        if (!IsItemFocused()) {
            return false;
        }
    }
    else
    {
        // Test for bounding box overlap, as updated as ItemAdd()
        let status_flags = g.last_item_data.status_flags;
        if (!(status_flags & ItemStatusFlags::HoveredRect)) {
            return false;
        }
        // IM_ASSERT((flags & (ImGuiHoveredFlags_AnyWindow | ImGuiHoveredFlags_RootWindow | ImGuiHoveredFlags_ChildWindows | ImGuiHoveredFlags_NoPopupHierarchy | ImGuiHoveredFlags_DockHierarchy)) == 0);   // flags not supported by this function

        // Test if we are hovering the right window (our window could be behind another window)
        // [2021/03/02] Reworked / reverted the revert, finally. Note we want e.g. BeginGroup/ItemAdd/EndGroup to work as well. (#3851)
        // [2017/10/16] Reverted commit 344d48be3 and testing root_window instead. I believe it is correct to NOT test for root_window but this leaves us unable
        // to use IsItemHovered() after EndChild() itself. Until a solution is found I believe reverting to the test from 2017/09/27 is safe since this was
        // the test that has been running for a long while.
        if (g.hovered_window_id != window && (status_flags & ImGuiItemStatusFlags_HoveredWindow) == 0) {
            if ((flags & HoveredFlags::AllowWhenOverlapped) == 0) {
                return false;
            }
        }

        // Test if another item is active (e.g. being dragged)
        if ((flags & HoveredFlags::AllowWhenBlockedByActiveItem) == 0) {
            if (g.active_id != 0 && g.active_id != g.last_item_data.id && !g.active_id_allow_overlap) {
                if (g.active_id != window.MoveId && g.active_id != window.TabId) {
                    return false;
                }
            }
        }

        // Test if interactions on this window are blocked by an active popup or modal.
        // The ImGuiHoveredFlags_AllowWhenBlockedByPopup flag will be tested here.
        if (!is_window_content_hoverable(g, window, flags)) {
            return false;
        }

        // Test if the item is disabled
        if ((g.last_item_data.in_flags & ImGuiItemFlags_Disabled) && !(flags & HoveredFlags::AllowWhenDisabled)) {
            return false;
        }

        // Special handling for calling after Begin() which represent the title bar or tab.
        // When the window is skipped/collapsed (skip_items==true) that last item (always ->move_id submitted by Begin)
        // will never be overwritten so we need to detect the case.
        if (g.last_item_data.id == window.MoveId && window.WriteAccessed)
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
pub struct WindowClass
{
    pub class_id: Id32,                  // User data. 0 = Default class (unclassed). windows of different classes cannot be docked with each others.
    pub parent_viewport_id: Id32,         // Hint for the platform backend. -1: use default. 0: request platform backend to not parent the platform. != 0: request platform backend to create a parent<>child relationship between the platform windows. Not conforming backends are free to e.g. parent every viewport to the main viewport or not.
    pub viewport_flags_override_set: ViewportFlags,   // viewport flags to set when a window of this class owns a viewport. This allows you to enforce OS decoration or task bar icon, override the defaults on a per-window basis.
    pub viewport_flags_override_clear: ViewportFlags, // viewport flags to clear when a window of this class owns a viewport. This allows you to enforce OS decoration or task bar icon, override the defaults on a per-window basis.
    pub tab_item_flags_override_set: DimgTabItemFlags,    // [EXPERIMENTAL] TabItem flags to set when a window of this class gets submitted into a dock node tab bar. May use with ImGuiTabItemFlags_Leading or ImGuiTabItemFlags_Trailing.
    pub dock_node_flags_override_set: DockNodeFlags,   // [EXPERIMENTAL] Dock node flags to set when a window of this class is hosted by a dock node (it doesn't have to be selected!)
    pub docking_always_tab_bar: bool,        // Set to true to enforce single floating windows of this class always having their own docking node (equivalent of setting the global io.config_docking_always_tab_bar)
    pub docking_allow_unclassed: bool,      // Set to true to allow windows of this class to be docked/merged with an unclassed window. // FIXME-DOCK: Move to DockNodeFlags override?

}

impl WindowClass {
    // ImGuiWindowClass() { memset(this, 0, sizeof(*this)); parent_viewport_id = (ImGuiID)-1; docking_allow_unclassed = true;
    pub fn new() -> Self {
        Self {
            parent_viewport_id: Id32::MAX,
            docking_allow_unclassed: true,
            ..Default::default()
        }
    }
}

/// windows data saved in imgui.ini file
/// Because we never destroy or rename ImGuiWindowSettings, we can store the names in a separate buffer easily.
/// (this is designed to be stored in a ImChunkStream buffer, with the variable-length name following our structure)
#[derive(Default,Debug,Clone)]
pub struct WindowSettings
{
    //ImGuiID     id;
    pub id: Id32,
    // Vector2Dih    pos;            // NB: Settings position are stored RELATIVE to the viewport! Whereas runtime ones are absolute positions.
    pub pos: Vector2D,
    // Vector2Dih    size;
    pub size: Vector2D,
    // Vector2Dih    ViewportPos;
    pub viewport_pos: Vector2D,
    // ImGuiID     ViewportId;
    pub viewport_id: Id32,
    // ImGuiID     DockId;         // id of last known dock_node (even if the dock_node is invisible because it has only 1 active window), or 0 if none.
    pub dock_id: Id32,
    // ImGuiID     ClassId;        // id of window class if specified
    pub class_id: Id32,
    // short       DockOrder;      // Order of the last time the window was visible within its dock_node. This is used to reorder windows that are reappearing on the same frame. Same value between windows that were active and windows that were none are possible.
    pub dock_order: i16,
    // bool        Collapsed;
    pub collapsed: bool,
    // bool        WantApply;      // Set when loaded from .ini data (to enable merging/loading .ini data into an already running context)
    pub want_apply: bool,
    // ImGuiWindowSettings()       { memset(this, 0, sizeof(*this)); DockOrder = -1; }
    // char* GetName()             { return (char*)(this + 1); }
}

//     ImGuiWindowFlags_NoDecoration           = WindowFlags::NoTitleBar | ImGuiWindowFlags_NoResize | ImGuiWindowFlags_NoScrollbar | ImGuiWindowFlags_NoCollapse,
// pub const NoDecoration: i32 = DimgWindowFlags::NoTitleBar | DimgWindowFlags::NoResize | DimgWindowFlags::NoScrollbar | DimgWindowFlags::NoCollapse;
pub const DIMG_WIN_FLAGS_NO_DECORATION: HashSet<WindowFlags> = HashSet::from([
    WindowFlags::NoTitleBar, WindowFlags::NoResize, WindowFlags::NoScrollbar, WindowFlags::NoCollapse
]);

// ImGuiWindowFlags_NoNav                  = ImGuiWindowFlags_NoNavInputs | ImGuiWindowFlags_NoNavFocus,
// pub const ImGuiWindowFlags_NoNav: i32 = DimgWindowFlags::NoNavInputs | DimgWindowFlags::NoNavFocus;
pub const DIMG_WIN_FLAGS_NO_NAV: HashSet<WindowFlags> = HashSet::from([WindowFlags::NoNavInputs, WindowFlags::NoNavFocus]);

//     ImGuiWindowFlags_NoInputs               = ImGuiWindowFlags_NoMouseInputs | ImGuiWindowFlags_NoNavInputs | ImGuiWindowFlags_NoNavFocus,
// pub const NoInputs: i32 = DimgWindowFlags::NoMouseInputs | DimgWindowFlags::NoNavInputs | DimgWindowFlags::NoNavFocus;
pub const DIMG_WIN_FLAGS_NO_INPUTS: HashSet<WindowFlags> = HashSet::from([
    WindowFlags::NoMouseInputs, WindowFlags::NoNavInputs, WindowFlags::NoNavFocus
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
pub type ImGuiSizeCallback = fn(*mut SizeCallbackData);

// flags for ImGui::Begin()
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum WindowFlags
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
    HorizontalScrollbar    = 1 << 11,  // Allow horizontal scrollbar to appear (off by default). You may use SetNextWindowContentSize(Vector2D(width,0.0)); prior to calling Begin() to specify width. Read code in imgui_demo in the "Horizontal Scrolling" section.
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
    root_window                    = 1 << 1,   // Test from root window (top most parent of the current hierarchy)
    AnyWindow                     = 1 << 2,   // Return true if any window is focused. Important: If you are trying to tell how to dispatch your low-level inputs, do NOT use this. Use 'io.want_capture_mouse' instead! Please read the FAQ!
    NoPopupHierarchy              = 1 << 3,   // Do not consider popup hierarchy (do not treat popup emitter as parent of popup) (when used with _ChildWindows or _RootWindow)
    DockHierarchy                 = 1 << 4,   // Consider docking hierarchy (treat dockspace host as parent of docked window) (when used with _ChildWindows or _RootWindow)
    // ImGuiFocusedFlags_RootAndChildWindows           = ImGuiFocusedFlags_RootWindow | ImGuiFocusedFlags_ChildWindows
}


// flags for ImGui::IsItemHovered(), ImGui::IsWindowHovered()
// Note: if you are trying to check whether your mouse should be dispatched to Dear ImGui or to your app, you should use 'io.want_capture_mouse' instead! Please read the FAQ!
// Note: windows with the ImGuiWindowFlags_NoInputs flag are ignored by IsWindowHovered() calls.
#[derive(Debug,Clone,Eq, PartialEq,Hash)]
pub enum HoveredFlags
{
    None                          = 0,        // Return true if directly over the item/window, not obstructed by another window, not obstructed by an active popup or modal blocking inputs under them.
    ChildWindows                  = 1 << 0,   // IsWindowHovered() only: Return true if any children of the window is hovered
    root_window                    = 1 << 1,   // IsWindowHovered() only: Test from root window (top most parent of the current hierarchy)
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
pub const ROOT_AND_CHILD_WINDOWS: HashSet<HoveredFlags> = HashSet::from([
    HoveredFlags::root_window, HoveredFlags::ChildWindows
]);


// pub const RectOnly : i32                     = DimgHoveredFlags::AllowWhenBlockedByPopup | DimgHoveredFlags::AllowWhenBlockedByActiveItem | DimgHoveredFlags::AllowWhenOverlapped;
 pub const RECT_ONLY: HashSet<HoveredFlags> = HashSet::from([
     HoveredFlags::AllowWhenBlockedByPopup, HoveredFlags::AllowWhenBlockedByActiveItem, HoveredFlags::AllowWhenOverlapped
 ]);

#[derive(Debug,Clone,Default)]
pub struct ShrinkWidthItem
{
    // int         index;
    pub index: i32,
    // float       width;
    pub Width: f32,
    // float       InitialWidth;
    pub InitialWidth: f32,
}

// bool ImGui::IsClippedEx(const ImRect& bb, ImGuiID id)
pub fn is_clipped_ex(g: &mut Context, bb: &Rect, id: Id32) -> Result<bool, &'static str>
{
    // ImGuiContext& g = *GImGui;
    // ImGuiWindow* window = g.CurrentWindow;
    let window = g.get_current_window()?;
    if !bb.Overlaps(window.ClipRect) {
        if id == 0 || (id != g.active_id && id != g.NavId) {
            if !g.LogEnabled {
                return Ok(true);
            }
        }
    }
    return Ok(false);
}

// float ImGui::CalcWrapWidthForPos(const Vector2D& pos, float wrap_pos_x)
pub fn calc_wrap_width_for_pos(g: &mut Context, pos: &Vector2D, wrap_pox_x: f32) -> Result<f32, &'static str>
{
    if wrap_pos_x < 0.0 {
        return Ok(0.0);
    }

    // ImGuiContext& g = *GImGui;
    // ImGuiWindow* window = g.CurrentWindow;
    let window = g.get_current_window()?;
    if wrap_pos_x == 0.0
    {
        // We could decide to setup a default wrapping max point for auto-resizing windows,
        // or have auto-wrap (with unspecified wrapping pos) behave as a content_size extending function?
        //if (window->hidden && (window->flags & ImGuiWindowFlags_AlwaysAutoResize))
        //    wrap_pos_x = ImMax(window->work_rect.min.x + g.font_size * 10.0, window->work_rect.max.x);
        //else
        wrap_pos_x = window.work_rect.max.x;
    }
    else if wrap_pos_x > 0.0
    {
        wrap_pos_x += window.pos.x - window.scroll.x; // wrap_pos_x is provided is window local space
    }

    let out = f32::max(wrap_pos_x - pos.x, 1.0)
    Ok(out)
}

/// void ImGui::start_mouse_moving_window(ImGuiWindow* window)
pub fn start_mouse_moving_window(g: &mut Context, window: &mut Window)
{
    // Set active_id even if the _NoMove flag is set. Without it, dragging away from a window with _NoMove would activate hover on other windows.
    // We _also_ call this when clicking in a window empty space when io.config_windows_move_from_title_bar_only is set, but clear g.moving_window afterward.
    // This is because we want active_id to be set even when the window is not permitted to move.
    // ImGuiContext& g = *GImGui;
    // focus_window(window);
    focus_window(window);
    // SetActiveID(window.MoveId, window);
    set_active_id(ctx, window.id, window);
    g.nav_disable_highlight = true;
    g.active_id_click_offset = &g.io.mouse_clicked_pos[0] - window.root_window_dock_tree.pos;
    g.ActiveIdNoClearOnFocusLoss = true;
    // SetActiveIdUsingNavAndKeys();
    set_active_id_using_nav_and_keys();

    // bool can_move_window = true;
    let mut can_move_window= true;
    if window.flags.contains(WindowFlags::NoMove) || (window.root_window_dock_tree.flags.contains(WindowFlags::NoMove) {
        can_move_window = false;
    }
    let node = window.dock_node_as_host;
    if node != INVALID_ID {
        if node.visible_window && (node.visible_window.flags.contains(WindowFlags::NoMove)) {
        can_move_window = false;}
    }
    if can_move_window {
        g.moving_window_id = window;
    }
}

/// We use 'undock_floating_node == false' when dragging from title bar to allow moving groups of floating nodes without undocking them.
/// - undock_floating_node == true: when dragging from a floating node within a hierarchy, always undock the node.
/// - undock_floating_node == false: when dragging from a floating node within a hierarchy, move root window.
/// void ImGui::StartMouseMovingWindowOrNode(ImGuiWindow* window, ImGuiDockNode* node, bool undock_floating_node)
pub fn start_mouse_moving_window_or_node(g: &mut Context, window: &mut Window, node: &mut DockNode, undock_floating_node: bool)
{
    // ImGuiContext& g = *GImGui;
    // bool can_undock_node = false;
    let mut can_undock_node = false;
    // if (node != NULL && node->VisibleWindow && (node->VisibleWindow.flags & ImGuiWindowFlags_NoMove) == 0)
    if node.visible_window != INVALID_ID && node.visible_window.flags.contains(WindowFlags::NoMove) == false
    {
        // Can undock if:
        // - part of a floating node hierarchy with more than one visible node (if only one is visible, we'll just move the whole hierarchy)
        // - part of a dockspace node hierarchy (trivia: undocking from a fixed/central node will create a new node and copy windows)
        // ImGuiDockNode* root_node = DockNodeGetRootNode(node);
        let mut root_node = dock_node_get_root_node(g, node);
        //if (root_node->OnlyNodeWithWindows != node || root_node->CentralNode != NULL)
        if root_node.only_node_with_window != node.id || root_node.central_node != INVALID_ID
        {  // -V1051 PVS-Studio thinks node should be root_node and is wrong about that.
        // if (undock_floating_node || root_node -> IsDockSpace())
        if undock_floating_node || root_node.is_dock_space()
            {
            can_undock_node = true;
        }
    }
    }

    // const bool clicked = IsMouseClicked(0);
    let clicked = is_mouse_clicked(g, 0);
    // const bool dragging = IsMouseDragging(0, g.io.MouseDragThreshold * 1.70);
    let dragging = is_mouse_dragging(g, 0, g.io.mouse_drag_threshold * 1.70);
    if can_undock_node && dragging {
        dock_context_queue_undock_node(&g, node); // Will lead to DockNodeStartMouseMovingWindow() -> start_mouse_moving_window() being called next frame
    }
    else if !can_undock_node && (clicked || dragging) && g.moving_window_id != window.id {
        start_mouse_moving_window(g, window);
    }
}

/// Handle mouse moving window
/// Note: moving window with the navigation keys (Square + d-pad / CTRL+TAB + Arrows) are processed in NavUpdateWindowing()
/// FIXME: We don't have strong guarantee that g.moving_window stay synched with g.active_id == g.moving_window->move_id.
/// This is currently enforced by the fact that BeginDragDropSource() is setting all g.ActiveIdUsingXXXX flags to inhibit navigation inputs,
/// but if we should more thoroughly test cases where g.active_id or g.moving_window gets changed and not the other.
/// void ImGui::UpdateMouseMovingWindowNewFrame()
pub fn update_mouse_moving_window_new_frame(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    if (g.moving_window_id != INVALID_ID)
    {
        // We actually want to move the root window. g.moving_window == window we clicked on (could be a child window).
        // We track it to preserve Focus and so that generally active_id_window == moving_window and active_id == moving_window->move_id for consistency.
        keep_alive_id(g.active_id);
        // IM_ASSERT(g.moving_window && g.moving_window->RootWindowDockTree);
        // ImGuiWindow* moving_window = g.moving_window->RootWindowDockTree;
        let moving_window_id = g.get_window(g.moving_window_id).unwrap().root_window_dock_tree;
        let moving_window = g.get_window(moving_window_id).unwrap();

        // When a window stop being submitted while being dragged, it may will its viewport until next Begin()
        // const bool window_disappared = ((!moving_window.WasActive && !moving_window.Active) || moving_window.viewport == NULL);
        let window_disappeared = !moving
        if (g.io.mouse_down[0] && is_mouse_pos_valid(&g.io.mouse_pos) && !window_disappared)
        {
            // Vector2D pos = g.io.mouse_pos - g.ActiveIdClickOffset;
            let mut pos = g.io.mouse_pos.clone() - g.active_id_click_offset.clone();
            if (moving_window.pos.x != pos.x || moving_window.pos.y != pos.y)
            {
                set_window_pos(moving_window, pos, Cond::Always);
                if (moving_window.viewport_owned) // Synchronize viewport immediately because some overlays may relies on clipping rectangle before we Begin() into the window.
                {
                    moving_window.viewport.pos = pos.clone();
                    moving_window.viewport.update_work_rect();
                }
            }
            focus_window(g.moving_window_id);
        }
        else
        {
            if !window_disappared
            {
                // Try to merge the window back into the main viewport.
                // This works because mouse_viewport should be != moving_window->viewport on release (as per code in UpdateViewports)
                if g.config_flags_curr_frame.contains(ConfigFlags::ViewportsEnable) {
                    update_try_merge_window_into_host_viewport(moving_window, g.mouse_viewport_id);
                }

                // Restore the mouse viewport so that we don't hover the viewport _under_ the moved window during the frame we released the mouse button.
                if !is_drag_drop_payload_being_accepted() {
                    g.mouse_viewport_id = moving_window.viewport;
                }

                // clear the NoInput window flag set by the viewport system
                // moving_window.viewport.flags &= ~ViewportFlags::NoInputs; // FIXME-VIEWPORT: Test engine managed to crash here because viewport was NULL.
                let mut viewport: &mut Viewport = g.get_viewport(moving_window.viewport_id).unwrap();
                remove_hash_set_val(&mut viewport.flags, &ViewportFlags::NoInputs)
            }

            g.moving_window_id = INVALID_ID;
            clear_active_id();
        }
    }
    else
    {
        // When clicking/dragging from a window that has the _NoMove flag, we still set the active_id in order to prevent hovering others.
        if (g.active_id_window_id != INVALID_ID) // && g.active_id_window.move_id == g.active_id)
        {
            let active_id_win = g.get_window(g.active_id_window_id).unwrap();
            if active_id_win.move_id == g.active_id {
                keep_alive_id(g.active_id);
                if (!g.io.mouse_down[0]) {
                    clear_active_id();
                }
            }
        }
    }
}

/// Initiate moving window when clicking on empty space or title bar.
/// Handle left-click and right-click focus.
/// void ImGui::UpdateMouseMovingWindowEndFrame()
pub fn update_mouse_moving_window_end_frame(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    if g.active_id != INVALID_ID || g.hovered_id != INVALID_ID {
        return;
    }

    // Unless we just made a window/popup appear
    // if (g.nav_window && g.nav_window.appearing) {
    //     return;
    // }
    if g.nav_window_id != INVALID_ID {
        let win = g.get_window(g.nav_window_id).unwrap();
        if win.appearing {
            return;
        }
    }

    // Click on empty space to focus window and start moving
    // (after we're done with all our widgets, so e.g. clicking on docking tab-bar which have set hovered_id already and not get us here!)
    if g.io.mouse_clicked[0]
    {
        // Handle the edge case of a popup being closed while clicking in its empty space.
        // If we try to focus it, focus_window() > close_popups_over_window() will accidentally close any parent popups because they are not linked together any more.
        // ImGuiWindow* root_window = g.hovered_window ? g.hovered_window->RootWindow : NULL;
        let root_window = if g.hovered_window_id != INVALID_ID {
            let hov_win = g.get_window(g.hovered_window_id).unwrap();
            Some(g.get_window(hov_win.root_window).unwrap())
        } else {
            None
        };
        // const bool is_closed_popup = root_window && (root_window.Flags & ImGuiWindowFlags_Popup) && !IsPopupOpen(root_window.PopupId, ImGuiPopupFlags_AnyPopupLevel);
        let is_closed_popup: bool = if root_window.is_some() {
            let root_win = root_window.unwrap();
            if root_win.flags.contains(&WindowFlags::Popup) {
                if is_popup_open(root_win.popup_id, PopupFlags::AnyPopupLevel) {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        };

        // if (root_window != NULL && !is_closed_popup)

        if root_window != INVALID_ID && is_closed_popup == false
        {
            let root_win = root_window.unwrap();
            start_mouse_moving_window(g, g.hovered_window); //-V595

            // Cancel moving if clicked outside of title bar
            if g.io.config_windows_move_from_title_bar_only {
                if !(root_win.flags.contains(&WindowFlags::NoTitleBar)) || root_win.dock_is_active {
                    if !root_win.title_bar_rect().Contains(&g.io.mouse_clicked_pos[0]) {
                        g.moving_window_id = NULL;
                    }
                }
            }

            // Cancel moving if clicked over an item which was disabled or inhibited by popups (note that we know hovered_id == 0 already)
            if g.hovered_id_disabled {
                g.moving_window_id = NULL;
            }
        }
        else if root_window == INVALID_ID && g.nav_window_id != NULL && get_top_most_popup_modal() == NULL
        {
            // Clicking on void disable focus
            focus_window(NULL);
        }
    }

    // With right mouse button we close popups without changing focus based on where the mouse is aimed
    // Instead, focus will be restored to the window under the bottom-most closed popup.
    // (The left mouse button path calls focus_window on the hovered window, which will lead NewFrame->close_popups_over_window to trigger)
    if g.io.mouse_clicked[1]
    {
        // Find the top-most window between hovered_window and the top-most Modal Window.
        // This is where we can trim the popup stack.
        let modal = get_top_most_popup_modal();
        let hovered_window_above_modal = g.hovered_window && (modal == NULL || is_window_above(g.hovered_window, modal));
        close_popups_over_window(if hovered_window_above_modal { g.hovered_window } else { modal}, true);
    }
}

/// This is called during NewFrame()->UpdateViewportsNewFrame() only.
/// Need to keep in sync with set_window_pos()
/// static void TranslateWindow(ImGuiWindow* window, const Vector2D& delta)
pub fn translate_window(window: &mut Window, delta: &Vector2D)
{
    window.pos += delta;
    window.ClipRect.Translate(delta);
    window.OuterRectClipped.Translate(delta);
    window.inner_rect.Translate(delta);
    window.DC.CursorPos += delta;
    window.DC.CursorStartPos += delta;
    window.DC.CursorMaxPos += delta;
    window.DC.IdealMaxPos += delta;
}

/// static void ScaleWindow(ImGuiWindow* window, float scale)
pub fn scale_window(window: &mut Window, scale: f32)
{
    // Vector2D origin = window.viewport.pos;
    let mut origin = window.viewport.pos;
    window.pos = f32::floor((window.pos - origin) * scale + origin);
    window.size = f32::floor(window.size * scale);
    window.size_full = f32::floor(window.size_full * scale);
    window.ContentSize = f32::floor(window.ContentSize * scale);
}

// static bool IsWindowActiveAndVisible(ImGuiWindow* window)
pub fn is_window_active_and_visible(window: &mut Window)
{
    return (window.active) && (!window.hidden);
}

/// The reason this is exposed in imgui_internal.h is: on touch-based system that don't have hovering, we want to dispatch inputs to the right target (imgui vs imgui+app)
/// void ImGui::UpdateHoveredWindowAndCaptureFlags()
pub fn update_hovered_window_and_capture_flags(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    // ImGuiIO& io = g.io;
    let io = &mut g.io;
    g.windows_hover_padding = Vector2D::max(g.style.touch_extra_padding, Vector2D::new(WINDOWS_HOVER_PADDING, WINDOWS_HOVER_PADDING));

    // Find the window hovered by mouse:
    // - Child windows can extend beyond the limit of their parent so we need to derive HoveredRootWindow from hovered_window.
    // - When moving a window we can skip the search, which also conveniently bypasses the fact that window->WindowRectClipped is lagging as this point of the frame.
    // - We also support the moved window toggling the NoInputs flag after moving has started in order to be able to detect windows below it, which is useful for e.g. docking mechanisms.
    let mut clear_hovered_windows = false;
    find_hovered_window();
    // IM_ASSERT(g.hovered_window == NULL || g.hovered_window == g.moving_window || g.hovered_window->Viewport == g.mouse_viewport);

    // Modal windows prevents mouse from hovering behind them.
    // ImGuiWindow* modal_window = get_top_most_popup_modal();
    let modal_window = get_top_most_popup_modal();
    let hov_win = g.get_window(g.hovered_window_id).unwrap();
    if modal_window && hovered_window_id != INVALID_ID && !is_window_within_begin_stack_of(g.get_window(g.hovered_window).unwrap().root_window, modal_window) { // FIXME-MERGE: root_window_dock_tree ?
        clear_hovered_windows = true;
    }

    // Disabled mouse?
    if io.config_flags.contains(&ConfigFlags::NoMouse) {
        clear_hovered_windows = true;
    }

    // We track click ownership. When clicked outside of a window the click is owned by the application and
    // won't report hovering nor request capture even while dragging over our windows afterward.
    // const bool has_open_popup = (g.OpenPopupStack.Size > 0);
    let has_open_popup = g.open_popup_stack.size > 0;
    let has_open_modal = (modal_window != NULL);
    let mut mouse_earliest_down = -1;
    let mut mouse_any_down = false;
    // for (int i = 0; i < IM_ARRAYSIZE(io.mouse_down); i += 1)
    for i in 0 .. io.mouse_down.len()
    {
        if (io.mouse_clicked[i])
        {
            io.mouse_down_owned[i] = (g.hovered_window_id != INVALID_ID) || has_open_popup;
            io.mouse_down_owned_unless_popup_close[i] = (g.hovered_window_id != INVALID_ID) || has_open_modal;
        }
        mouse_any_down |= io.mouse_down[i];
        if (io.mouse_down[i]) {
            if (mouse_earliest_down == -1 || io.mouse_clicked_time[i] < io.mouse_clicked_time[mouse_earliest_down]) {
                mouse_earliest_down = i;
            }
        }
    }
    let mouse_avail = (mouse_earliest_down == -1) || io.mouse_down_owned[mouse_earliest_down];
    let mouse_avail_unless_popup_close = (mouse_earliest_down == -1) || io.mouse_down_owned_unless_popup_close[mouse_earliest_down];

    // If mouse was first clicked outside of ImGui bounds we also cancel out hovering.
    // FIXME: For patterns of drag and drop across OS windows, we may need to rework/remove this test (first committed 311c0ca9 on 2015/02)
    let mouse_dragging_extern_payload = g.drag_drop_active && (g.drag_drop_source_flags & DragDropFlags::SourceExtern) != 0;
    if (!mouse_avail && !mouse_dragging_extern_payload) {
        clear_hovered_windows = true;
    }

    if (clear_hovered_windows) {
        g.hovered_window = g.hovered_window_under_moving_window_id = NULL;
    }

    // Update io.want_capture_mouse for the user application (true = dispatch mouse info to Dear ImGui only, false = dispatch mouse to Dear ImGui + underlying app)
    // Update io.WantCaptureMouseAllowPopupClose (experimental) to give a chance for app to react to popup closure with a drag
    if (g.want_capture_mouse_next_frame != -1)
    {
         io.want_capture_mouse_unless_popup_close = (g.want_capture_mouse_next_frame != 0);
        io.want_capture_mouse = io.want_capture_mouse_unless_popup_close;
    }
    else
    {
        io.want_capture_mouse = (mouse_avail && (g.hovered_window_id != INVALID_ID || mouse_any_down)) || has_open_popup;
        io.want_capture_mouse_unless_popup_close = (mouse_avail_unless_popup_close && (g.hovered_window_id != INVALID_ID || mouse_any_down)) || has_open_modal;
    }

    // Update io.want_capture_keyboard for the user application (true = dispatch keyboard info to Dear ImGui only, false = dispatch keyboard info to Dear ImGui + underlying app)
    if (g.want_capture_keyboard_next_frame != -1) {
        io.want_capture_keyboard = (g.want_capture_keyboard_next_frame != 0);
    }
    else{
    io.want_capture_keyboard = (g.active_id != 0) || (modal_window != NULL);
}
    if (io.nav_active && (io.config_flags.contains(&ConfigFlags::NavEnableKeyboard)) && !(io.config_flags.contains(&ConfigFlags::NavNoCaptureKeyboard))) {
        io.want_capture_keyboard = true;
    }

    // Update io.want_text_input flag, this is to allow systems without a keyboard (e.g. mobile, hand-held) to show a software keyboard if possible
    io.want_text_input = if g.want_text_input_next_frame != -1 { (g.want_text_input_next_frame != 0)} else { false };
}


/// static int IMGUI_CDECL ChildWindowComparer(const void* lhs, const void* rhs)
pub fn child_window_comparer(lhs: &Window, rhs: &Window) -> i32
{
    // const ImGuiWindow* const a = *(const ImGuiWindow* const *)lhs;
    // const ImGuiWindow* const b = *(const ImGuiWindow* const *)rhs;
    if lhs.flags.contains(&WindowFlags::Popup) - rhs.flags.contains(& WindowFlags::Popup){
        return 1;}
    if lhs.flags.contains(&WindowFlags::Tooltip) - rhs.flags.contains(&WindowFlags::Tooltip) {
        return 1;
    }
    return (lhs.begin_order_within_parent - rhs.begin_order_within_parent) as i32
}

// static void AddWindowToSortBuffer(ImVector<ImGuiWindow*>* out_sorted_windows, ImGuiWindow* window)
pub fn add_window_to_sort_buffer(ctx: &mut Context, out_sorted_windows: &Vec<Id32>, window: Id32)
{
    out_sorted_windows.push_back(window);
    let win = ctx.get_window(window).unwrap();
    if window.active
    {
        // int count = window.DC.ChildWindows.Size;
        let count = win.dc.child_windows.len();
        // ImQsort(window.DC.ChildWindows.Data, count, sizeof(ImGuiWindow*), ChildWindowComparer);
        win.dc.child_windows.sort();
        for child_win_id in win.dc.child_windows.iter() {
            let child_win = ctx.get_window(*child_win_id).unwrap();
            if child_win.active {
                add_window_to_sort_buffer(ctx, out_sorted_windows, *child_win_id);
            }
        }

        // for (int i = 0; i < count; i += 1)
        // {
        //     ImGuiWindow* child = window.DC.ChildWindows[i];
        //     if (child->Active)
        //         AddWindowToSortBuffer(out_sorted_windows, child);
        // }
    }
}

/// static void AddWindowToDrawData(ImGuiWindow* window, int layer)
pub fn add_window_to_draw_data(ctx: &mut Context, window: &mut Window, layer: i32)
{
    // ImGuiContext& g = *GImGui;
    // ImGuiViewportP* viewport = window.viewport;
    let viewport_id = window.viewport_id;
    let viewport = ctx.get_viewport(viewport_id).unwrap();
    g.io.metrics_render_windows += 1;
    if window.flags.contains(&WindowFlags::DockNodeHost) {
        window.draw_list.channels_merge();
    }
    add_draw_list_to_draw_data(ctx, &mut viewport.draw_data_builder.layers[layer], window.draw_list);
    // for (int i = 0; i < window.DC.ChildWindows.Size; i += 1)
    // {
    //     ImGuiWindow* child = window.DC.ChildWindows[i];
    //     if (IsWindowActiveAndVisible(child)) // Clipped children may have been marked not active
    //         AddWindowToDrawData(child, layer);
    // }
    for child_id in window.dc.child_windows.iter() {
        let win_obj = ctx.get_window(*child_id).unwrap();
        if is_window_active_and_visible(win_obj) {
            add_window_to_draw_data(ctx, win_obj, layer);
        }
    }
}

/// static inline int GetWindowDisplayLayer(ImGuiWindow* window)
pub fn get_window_display_layer(window: &Window) -> i32
{
    // return (window.flags & WindowFlags::Tooltip) ? 1 : 0;
    if window.flags.contains(&WindowFlags::Tooltip) {
        1
    } else {
        0
    }
}

/// Layer is locked for the root window, however child windows may use a different viewport (e.g. extruding menu)
// static inline void AddRootWindowToDrawData(ImGuiWindow* window)
pub fn add_root_window_to_draw_data(ctx: &mut Context, window: &mut Window)
{
    // AddWindowToDrawData(window, GetWindowDisplayLayer(window));
    add_window_to_draw_data(ctx, window, get_window_display_layer(window))
}
