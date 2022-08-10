pub mod checks;
pub mod class;
pub mod clip;
pub mod current;
pub mod focus;
pub mod get;
pub mod layer;
pub mod lifecycle;
pub mod next_window;
pub mod pos;
pub mod props;
pub mod render;
pub mod settings;
pub mod size;
pub mod state;
pub mod temp_data;
pub mod visibility;

use std::ptr::null_mut;

use crate::color::COLOR32_A_MASK;
use crate::column::OldColumns;
use crate::condition::Condition;
use crate::config::ConfigFlags;
use crate::context::{set_active_id_using_nav_and_keys, Context};
use class::WindowClass;
use next_window::NextWindowDataFlags;
use settings::WindowSettings;
use std::collections::HashSet;
use temp_data::WindowTempData;

use crate::dock::node::{dock_node_get_root_node, DockNode, DockNodeFlags};
use crate::drag_drop::DragDropFlags;
use crate::draw::list::add_draw_list_to_draw_data;
use crate::types::Direction;

use crate::draw::data;
use crate::hash::{hash_data, hash_string};
use crate::id::set_active_id;
use crate::input::{mouse, NavLayer};
use crate::item::{ItemStatusFlags, LastItemData};
use crate::kv_store::Storage;
use crate::layout::LayoutType;
use crate::menu_columns::MenuColumns;
use crate::rect::Rect;
use crate::size_callback_data::SizeCallbackData;
use crate::stack::ImGuiStackSizes;
use crate::tab_bar::TabItemFlags;
use crate::types::{Id32, WindowHandle, INVALID_ID};
use crate::utils::{add_hash_set, remove_hash_set_val, sub_hash_set};
use crate::vectors::vector_2d::Vector2D;
use crate::vectors::Vector1D;
use crate::viewport::{Viewport, ViewportFlags};

// Storage for one window
#[derive(Default, Debug, Clone)]
pub struct Window {


    // Docking
    // Id32                 child_id;                            // id of corresponding item in parent window (for navigation to return from child window to parent window)
    // Id32                 dock_id;                             // Backup of last valid dock_node->id, so single window remember their dock node id even when they are not bound any more
    // Id32                 id;                                 // == hash_string(name)
    // Id32                 move_id;                             // == window->GetID("#MOVE")
    // Id32                 nav_last_ids[ImGuiNavLayer_COUNT];    // Last known nav_id for this window, per layer (0/1)
    // Id32                 popup_id;                            // id in the popup stack when this window is used as a popup/menu (because we use generic name/id for recycling)
    // Id32                 tab_id;                              // == window->GetID("#TAB")
    // Id32                 viewport_id;                         // We backup the viewport id (since the viewport may disappear or never be created if the window is inactive)
    // ImDrawList              DrawListInst;
    // ImDrawList*             draw_list;                           // == &DrawListInst (for backward compatibility reason with code using imgui_internal.h we keep this a pointer)
    // ImGuiCond               set_window_collapsed_allow_flags : 8;   // store acceptable condition flags for set_next_window_collapsed() use.
    // ImGuiCond               set_window_dock_allow_flags : 8;        // store acceptable condition flags for SetNextWindowDock() use.
    // ImGuiCond               set_window_pos_allow_flags : 8;         // store acceptable condition flags for set_next_window_pos() use.
    // ImGuiCond               set_window_size_allow_flags : 8;        // store acceptable condition flags for SetNextWindowSize() use.
    // ImGuiDir                auto_pos_last_direction;
    // ImGuiDockNode*          dock_node;                           // Which node are we docked into. Important: Prefer testing dock_is_active in many cases as this will still be set when the dock node is hidden.
    // ImGuiDockNode*          dock_node_as_host;                     // Which node are we owning (for parent windows)
    // ImGuiItemStatusFlags    dock_tab_item_status_flags;
    // ImGuiStorage            state_storage;
    // ImGuiViewportP*         viewport;                           // Always set in Begin(). Inactive windows may have a None value here if their viewport was discarded.
    // ImRect                  clip_rect;                           // current clipping/scissoring rectangle, evolve as we are using push_clip_rect(), etc. == draw_list->clip_rect_stack.back().
    // ImRect                  content_region_rect;                  // FIXME: This is currently confusing/misleading. It is essentially work_rect but not handling of scrolling. We currently rely on it as right/bottom aligned sizing operation need some size to rely on.
    // ImRect                  dock_tab_item_rect;
    // ImRect                  inner_clip_rect;                      // == inner_rect shrunk by window_padding*0.5 on each side, clipped within viewport or parent clip rect.
    // ImRect                  inner_rect;                          // Inner rectangle (omit title bar, menu bar, scroll bar)
    // ImRect                  nav_rect_rel[ImGuiNavLayer_COUNT];    // Reference rectangle, in window relative space
    // ImRect                  outer_rect_clipped;                   // == window->rect() just after setup in Begin(). == window->rect() for root window.
    // ImRect                  parent_work_rect;                     // Backup of work_rect before entering a container such as columns/tables. Used by e.g. SpanAllColumns functions to easily access. Stacked containers are responsible for maintaining this. // FIXME-WORKRECT: Could be a stack?
    // ImRect                  work_rect;                           // Initially covers the whole scrolling region. Reduced by containers e.g columns/tables when active. Shrunk by window_padding*1.0 on each side. This is meant to replace content_region_rect over time (from 1.71+ onward).
    // ImS8                    auto_fit_child_axises;
    // ImS8                    auto_fit_frames_x, auto_fit_frames_y;
    // ImS8                    disable_inputs_frames;                // Disable window interactions for N frames
    // ImS8                    hidden_frames_can_skip_items;           // Hide the window for N frames
    // ImS8                    hidden_frames_cannot_skip_items;        // Hide the window for N frames while allowing items to be submitted so we can measure their size
    // ImS8                    hidden_frames_for_render_only;          // Hide the window until frame N at Render() time only
    // ImVector<Id32>       IDStack;                            // id stack. id are hashes seeded with the value at the top of the stack. (In theory this should be in the temp_data structure)
    // ImVector<ImGuiOldColumns> ColumnsStorage;
    // The best way to understand what those rectangles are is to use the 'Metrics->Tools->Show windows Rectangles' viewer.
    // The main 'outer_rect', omitted as a field, is window->rect().
    // Vector2D                  content_size;                        // size of contents/scrollable client area (calculated from the extents reach of the cursor) from previous frame. Does not include window decoration or window padding.
    // Vector2D                  content_size_explicit;                // size of contents/scrollable client area explicitly request by the user via SetNextWindowContentSize().
    // Vector2D                  content_size_ideal;
    // Vector2D                  pos;                                // Position (always rounded-up to nearest pixel)
    // Vector2D                  scroll;
    // Vector2D                  scroll_max;
    // Vector2D                  scroll_target;                       // target scroll position. stored as cursor position with scrolling canceled out, so the highest point is always 0.0. (FLT_MAX for no change)
    // Vector2D                  scroll_target_center_ratio;            // 0.0 = scroll so that target position is at top, 0.5 = scroll so that target position is centered
    // Vector2D                  scroll_target_edge_snap_dist;           // 0.0 = no snapping, >0.0 snapping threshold
    // Vector2D                  scrollbar_sizes;                     // size taken by each scrollbars on their smaller axis. Pay attention! scrollbar_sizes.x == width of the vertical scrollbar, scrollbar_sizes.y = height of the horizontal scrollbar.
    // Vector2D                  set_window_pos_pivot;                  // store window pivot for positioning. Vector2D(0, 0) when positioning from top-left corner; Vector2D(0.5, 0.5) for centering; Vector2D(1, 1) for bottom right.
    // Vector2D                  set_window_pos_val;                    // store window position when using a non-zero Pivot (position set needs to be processed when we know the window size)
    // Vector2D                  size;                               // current size (==size_full or collapsed title bar size)
    // Vector2D                  size_full;                           // size when non collapsed
    // Vector2D                  viewport_pos;                        // We backup the viewport position (since the viewport may disappear or never be created if the window is inactive)
    // Vector2D                  window_padding;                      // window padding at the time of Begin().
    // Vector2D                hit_test_hole_offset;
    // Vector2D                hit_test_hole_size;                    // Define an optional rectangular hole where mouse will pass-through the window.
    // Window*            ParentWindow;                       // If we are a child _or_ popup _or_ docked window, this is pointing to our parent. Otherwise None.
    // Window*            RootWindow;                         // Point to ourself or first ancestor that is not a child window. Doesn't cross through popups/dock nodes.
    // Window*            nav_last_child_nav_window;              // When going to the menu bar, we remember the child window we came from. (This could probably be made implicit if we kept g.windows sorted by last focused including child window.)
    // Window*            parent_window_in_begin_stack;
    // Window*            root_window_dock_tree;                 // Point to ourself or first ancestor that is not a child window. Cross through dock nodes.
    // Window*            root_window_for_nav;                   // Point to ourself or first ancestor which doesn't have the NavFlattened flag.
    // Window*            root_window_for_title_bar_highlight;     // Point to ourself or first ancestor which will display TitleBgActive color when this window is active.
    // Window*            root_window_popup_tree;                // Point to ourself or first ancestor that is not a child window. Cross through popups parent<>child.
    // window_class        window_class;                        // Advanced users only. Set with set_next_window_class()
    // WindowDockStyle    dock_style;
    // WindowFlags        flags, flags_previous_frame;          // See enum WindowFlags_
    // WindowTempData     dc;                                 // Temporary per-window data, reset at the beginning of the frame. This used to be called ImGuiDrawContext, hence the "dc" variable name.
    // bool                    DockNodeis_visible   :1;
    // bool                    active;                             // Set to true on Begin(), unless collapsed
    // bool                    appearing;                          // Set during the frame where the window is appearing (or re-appearing)
    // bool                    auto_fit_only_grows;
    // bool                    collapsed;                          // Set when collapsing window to become only title-bar
    // bool                    dock_is_active        :1;             // When docking artifacts are actually visible. When this is set, dock_node is guaranteed to be != None. ~~ (dock_node != None) && (dock_node->windows.len() > 1).
    // bool                    dock_tab_is_visible    :1;             // Is our window visible this frame? ~~ is the corresponding tab selected?
    // bool                    dock_tab_want_close    :1;
    // bool                    has_close_button;                     // Set when the window has a close button (p_open != None)
    // bool                    hidden;                             // Do not display (== HiddenFrames*** > 0)
    // bool                    is_explicit_child;                    // Set when passed _ChildWindow, left to false by BeginDocked()
    // bool                    is_fallback_window;                   // Set on the "Debug##Default" window.
    // bool                    memory_compacted;                    // Set when window extraneous data have been garbage collected
    // bool                    scrollbar_x, scrollbar_y;             // Are scrollbars visible?
    // bool                    skip_items;                          // Set when items can safely be all clipped (e.g. window not visible or collapsed)
    // bool                    viewport_owned;
    // bool                    want_collapse_toggle;
    // bool                    was_active;
    // bool                    write_accessed;                      // Set to true when any widget access the current window
    // char*                   name;                               // window name, owned by the window.
    // float                   window_border_size;                   // window border size at the time of Begin().
    // float                   font_dpi_scale;
    // float                   font_window_scale;                    // User scale multiplier per-window, via SetWindowFontScale()
    // float                   item_width_default;
    // float                   last_time_active;                     // Last timestamp the window was active (using float as we don't need high precision there)
    // float                   window_rounding;                     // window rounding at the time of Begin(). May be clamped lower to avoid rendering artifacts with title bar, menu bar etc.
    // int                     NameBufLen;                         // size of buffer storing name. May be larger than strlen(name)!
    // int                     last_frame_active;                    // Last frame number the window was active.
    // int                     last_frame_just_focused;               // Last frame number the window was made Focused.
    // int                     memory_draw_list_idx_capacity;          // Backup of last idx/vtx count, so when waking up the window we can preallocate and avoid iterative alloc/copy
    // int                     memory_draw_list_vtx_capacity;
    // int                     settings_offset;                     // Offset into settings_windows[] (offsets are always valid as we only grow the array from the back)
    // int                     viewport_allow_platform_monitor_extend; // Reset to -1 every frame (index is guaranteed to be valid between NewFrame..EndFrame), only used in the appearing frame of a tooltip/popup to enforce clamping to a given monitor
    // pub viewport_id: Id32,
    // short                   begin_count;                         // Number of Begin() during the current frame (generally 0 or 1, 1+ if appending via multiple Begin/End pairs)
    // short                   begin_order_within_context;            // Begin() order within entire imgui context. This is mostly used for debugging submission order related issues.
    // short                   begin_order_within_parent;             // Begin() order within immediate parent window, if we are a child window. Otherwise 0.
    // short                   dock_order;                          // Order of the last time the window was visible within its dock_node. This is used to reorder windows that are reappearing on the same frame. Same value between windows that were active and windows that were none are possible.
    // short                   focus_order;                         // Order within windows_focus_order[], altered when windows are focused.
    // signed char             resize_border_held;                   // current border being held for resize (-1: none, otherwise 0-3)
    //*mut c_char,
    pub active: bool,
    pub appearing: bool,
    pub auto_fit_child_axises: i8,
    pub auto_fit_frames_x: i8,
    pub auto_fit_frames_y: i8,
    pub auto_fit_only_grows: bool,
    pub auto_pos_last_direction: Direction,
    pub begin_count: i16,
    pub begin_order_within_context: i16,
    pub begin_order_within_parent: i16,
    pub child_id: Id32,
    pub clip_rect: Rect,
    pub collapsed: bool,
    pub column: Vec<OldColumns>,
    pub content_region_rect: Rect,
    pub content_size: Vector2D,
    pub content_size_explicit: Vector2D,
    pub content_size_ideal: Vector2D,
    pub dc: WindowTempData,
    pub disable_inputs_frames: i8,
    pub doc_node_is_visible: bool,
    pub dock_id: Id32,
    pub dock_is_active: bool,
    pub dock_node_as_host_id: Id32, // Id32, // *mut ImGuiDockNode,
    pub dock_node_id: Id32, //Id32, // *mut ImGuiDockNode,
    pub dock_order: i16,
    pub dock_style: WindowDockStyle,
    pub dock_tab_is_visible: bool,
    pub dock_tab_item_rect: Rect,
    pub dock_tab_item_status_flags: HashSet<ItemStatusFlags>,
    pub dock_tab_want_close: bool,
    pub draw_list_id: Id32,
    pub draw_list_inst: Id32,
    pub flags: HashSet<WindowFlags>,
    pub flags_previous_frame: HashSet<WindowFlags>,
    pub focus_order: i16,
    pub font_dpi_scale: f32,
    pub font_window_scale: f32,
    pub has_close_button: bool,
    pub hidden: bool,
    pub hidden_frames_can_skip_items: i8,
    pub hidden_frames_cannot_skip_items: i8,
    pub hidden_frames_for_render_only: i8,
    pub hit_test_hole_offset: Vector2D,
    pub hit_test_hole_size: Vector2D,
    pub id: Id32,
    pub id_stack: Vec<Id32>,
    pub inner_clip_rect: Rect,
    pub inner_rect: Rect,
    pub is_explicit_child: bool,
    pub is_fallback_window: bool,
    pub item_width_default: f32,
    pub last_frame_active: usize,
    pub last_frame_just_focused: i32,
    pub last_time_active: f32,
    pub memory_compacted: bool,
    pub memory_draw_list_idx_capacity: usize,
    pub memory_draw_list_vtx_capacity: usize,
    pub move_id: Id32,
    pub name: String,
    pub nav_last_child_nav_window_id: WindowHandle,
    pub nav_last_ids: Vec<Id32>,
    pub nav_rect_rel: Vec<Rect>,
    pub outer_rect_clipped: Rect,
    pub parent_window_id: WindowHandle,
    pub parent_window_in_begin_stack_id: WindowHandle,
    pub parent_work_rect: Rect,
    pub popup_id: Id32,
    pub pos: Vector2D,
    pub resize_border_held: i8,
    pub root_window_dock_tree_id: WindowHandle,
    pub root_window_for_nav_id: WindowHandle,
    pub root_window_for_title_bar_highlight_id: WindowHandle,
    pub root_window_id: WindowHandle,
    pub root_window_popup_tree_id: WindowHandle,
    pub scroll: Vector2D,
    pub scroll_max: Vector2D,
    pub scroll_target: Vector2D,
    pub scroll_target_center_ratio: Vector2D,
    pub scroll_target_edge_snap_dist: Vector2D,
    pub scrollbar_sizes: Vector2D,
    pub scrollbar_x: bool,
    pub scrollbar_y: bool,
    pub set_window_collapsed_allow_flags: HashSet<Condition>,
    pub set_window_dock_allow_flags: HashSet<Condition>,
    pub set_window_pos_allow_flags: HashSet<Condition>,
    pub set_window_pos_pivot: Vector2D,
    pub set_window_pos_val: Vector2D,
    pub set_window_size_allow_flags: HashSet<Condition>,
    pub settings_offset: i32,
    pub size: Vector2D,
    pub size_full: Vector2D,
    pub skip_items: bool,
    pub state_storage: Storage,
    pub tab_id: Id32,
    pub viewport_allow_platform_monitor_extend: i32,
    pub viewport_id: Id32,
    pub viewport_owned: bool,
    pub viewport_pos: Vector2D,
    pub want_collapse_toggle: bool,
    pub was_active: bool,
    pub window_class: WindowClass,
    pub window_padding: Vector2D,
    pub window_rounding: f32,
    pub work_rect: Rect,
    pub write_accessed: bool,
}

impl Window {
    // // Window is mostly a dumb struct. It merely has a constructor and a few helper methods
    // Window::Window(ImGuiContext* context, const char* name) : DrawListInst(None)
    pub fn new(g: &mut Context, name: &str) -> Self {
        let mut out = Self {
            //     name = ImStrdup(name);
            //     NameBufLen = strlen(name) + 1;
            name: String::from(name),
            //     id = hash_string(name);
            id: hash_string(name.as_vec(), 0),
            //     IDStack.push_back(id);
            id_stack: Vec::new(),
            //     viewport_allow_platform_monitor_extend = -1;
            viewport_allow_platform_monitor_extend: -1,
            //     viewport_pos = Vector2D(FLT_MAX, FLT_MAX);
            viewport_pos: Vector2D::new(f32::MAX, f32::MAX),
            //     move_id = GetID("#MOVE");
            //     scroll_target = Vector2D(FLT_MAX, FLT_MAX);
            scroll_target: Vector2D::new(f32::MAX, f32::MAX),
            //     scroll_target_center_ratio = Vector2D(0.5, 0.5);
            scroll_target_center_ratio: Vector2D::new(0.5, 0.5),
            //     auto_fit_frames_x = auto_fit_frames_y = -1;
            auto_fit_frames_x: -1,
            auto_fit_frames_y: -1,
            //     auto_pos_last_direction = ImGuiDir_None;
            auto_pos_last_direction: Direction::None,
            //     set_window_pos_allow_flags = set_window_size_allow_flags = set_window_collapsed_allow_flags = set_window_dock_allow_flags = Condition::Always | ImGuiCond_Once | ImGuiCond_FirstUseEver | ImGuiCond_Appearing;
            set_window_pos_allow_flags: Condition::Always
                | Condition::Once
                | Condition::FirstUserEver
                | Condition::Appearing,
            set_window_size_allow_flags: Condition::Always
                | Condition::Once
                | Condition::FirstUserEver
                | Condition::Appearing,
            set_window_collapsed_allow_flags: Condition::Always
                | Condition::Once
                | Condition::FirstUserEver
                | Condition::Appearing,
            set_window_dock_allow_flags: Condition::ImGuiCondAlways
                | Condition::Once
                | Condition::FirstUserEver
                | Condition::Appearing,
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
            draw_list_id: INVALID_ID,
            ..Default::default()
        };

        // move_id: Self::get_id(g, "#MOVE"),
        //             //     tab_id = GetID("#TAB");
        //             tab_id: Self::get_id(g, "#TAB"),
        out.move_id = out.get_id(g, "#MOVE");
        out.tab_id = out.get_id(g, "#TAB");

        //     memset(this, 0, sizeof(*this));
        &out.id_stack.push(out.id);
        //     draw_list->_Data = &context->draw_list_shared_data;
        out.draw_list_id.data = g.draw_list_shared_data.clone();
        //     draw_list->_OwnerName = name;
        &out.draw_list_id.owner_name = &out.name;
        //     IM_PLACEMENT_NEW(&window_class) window_class();
        // TODO
        out
    }

    // Window::~Window()
    // {
    //     IM_ASSERT(draw_list == &DrawListInst);
    //     IM_DELETE(name);
    //     ColumnsStorage.clear_destruct();
    // }

    // Id32 Window::GetID(const char* str, const char* str_end)
    pub fn get_id(&mut self, g: &mut Context, in_str: &str) -> Id32 {
        // Id32 seed = IDStack.back();
        let mut seed = self.id_stack.back();
        // Id32 id = hash_string(str, str_end ? (str_end - str) : 0, seed);
        let id = hash_string(in_str.as_mut_vec(), 0);
        // ImGuiContext& g = *GImGui;
        if g.debug_hook_id_info == id {
            // debug_hook_id_info(id, DataType::String, str, str_end);
        }
        return id;
    }

    // Id32 Window::GetID(const void* ptr)
    pub fn get_id2(&mut self, g: &mut Context, ptr: &mut Vec<u8>) -> Id32 {
        // Id32 seed = IDStack.back();
        let mut seed = self.id_stack.back();
        // Id32 id = ImHashData(&ptr, sizeof(void*), seed);
        let mut id = hash_data(ptr, seed);
        // ImGuiContext& g = *GImGui;
        if g.debug_hook_id_info == id {
            // debug_hook_id_info(id, DataType::Pointer, ptr, None);
        }
        return id;
    }

    // Id32 Window::GetID(int n)
    pub fn get_id3(&mut self, g: &mut Context, n: i32) -> Id32 {
        // Id32 seed = IDStack.back();
        let mut seed = self.id_stack.back();
        // Id32 id = ImHashData(&n, sizeof(n), seed);
        let mut n_bytes: [u8; 4] = [0; 4];
        let n_bytes_raw = n.to_le_bytes();
        n_bytes[0] = n_bytes_raw[0];
        n_bytes[1] = n_bytes_raw[1];
        n_bytes[2] = n_bytes_raw[2];
        n_bytes[3] = n_bytes_raw[3];
        let mut id = hash_data(&mut n_bytes.into_vec(), seed);
        // TODO
        // ImGuiContext& g = *GImGui;
        if g.debug_hook_id_info == id {
            // debug_hook_id_info(id, DataType::S32, n, null());
        }
        return id;
    }

    // This is only used in rare/specific situations to manufacture an id out of nowhere.
    // Id32 Window::get_id_from_rectangle(const ImRect& r_abs)
    pub fn get_id_from_rect(&mut self, g: &mut Context, r_abs: &Rect) -> Id32 {
        // Id32 seed = IDStack.back();
        let seed = self.id_stack.back();
        // ImRect r_rel = ImGui::WindowRectAbsToRel(this, r_abs);
        let r_rel = window_rect_abs_to_rel(self, r_abs);
        // Id32 id = ImHashData(&r_rel, sizeof(r_rel), seed);
        let id = hash_data(&r_rel, seed);
        return id;
    }

    pub fn get_node(&mut self, node_id: Id32) -> &mut DockNode {
        todo!()
    }
} // end of Window Impl

#[derive(Debug, Clone, Default)]
pub struct WindowDockStyle {
    // ImU32 colors[WindowDockStyleCol_COUNT];
    pub colors: Vec<u32>,
}

// data saved for each window pushed into the stack
#[derive(Debug, Clone, Default)]
pub struct WindowStackData {
    // Window*            window;
    pub window: WindowHandle,
    // ImGuiLastItemData       parent_last_item_data_backup;
    pub parent_last_item_data_backup: LastItemData,
    // ImGuiStackSizes         stack_sizes_on_begin;      // Store size of various stacks for asserting
    pub stack_sizes_on_begin: ImGuiStackSizes,
}

//     WindowFlags_NoDecoration           = WindowFlags::NoTitleBar | WindowFlags_NoResize | WindowFlags_NoScrollbar | WindowFlags_NoCollapse,
// pub const NoDecoration: i32 = DimgWindowFlags::NoTitleBar | DimgWindowFlags::NoResize | DimgWindowFlags::NoScrollbar | DimgWindowFlags::NoCollapse;
pub const WIN_FLAGS_NO_DECORATION: HashSet<WindowFlags> = HashSet::from([
    WindowFlags::NoTitleBar,
    WindowFlags::NoResize,
    WindowFlags::NoScrollbar,
    WindowFlags::NoCollapse,
]);

// WindowFlags_NoNav                  = WindowFlags_NoNavInputs | WindowFlags_NoNavFocus,
// pub const WindowFlags_NoNav: i32 = DimgWindowFlags::NoNavInputs | DimgWindowFlags::NoNavFocus;
pub const DIMG_WIN_FLAGS_NO_NAV: HashSet<WindowFlags> =
    HashSet::from([WindowFlags::NoNavInputs, WindowFlags::NoNavFocus]);

//     WindowFlags_NoInputs               = WindowFlags_NoMouseInputs | WindowFlags_NoNavInputs | WindowFlags_NoNavFocus,
// pub const NoInputs: i32 = DimgWindowFlags::NoMouseInputs | DimgWindowFlags::NoNavInputs | DimgWindowFlags::NoNavFocus;
pub const DIMG_WIN_FLAGS_NO_INPUTS: HashSet<WindowFlags> = HashSet::from([
    WindowFlags::NoMouseInputs,
    WindowFlags::NoNavInputs,
    WindowFlags::NoNavFocus,
]);

// When using CTRL+TAB (or Gamepad Square+L/R) we delay the visual a little in order to reduce visual noise doing a fast switch.
// static let NAV_WINDOWING_HIGHLIGHT_DELAY            = 0.20;    // time before the highlight and screen dimming starts fading in
pub const NAV_WINDOWING_HIGHLIGHT_DELAY: f32 = 0.20;

// static let NAV_WINDOWING_LIST_APPEAR_DELAY          = 0.15;    // time before the window list starts to appear
pub const NAV_WINDOWING_LIST_APPEAR_DELAY: f32 = 0.15;

// window resizing from edges (when io.config_windows_resize_from_edges = true and ImGuiBackendFlags_HasMouseCursors is set in io.backend_flags by backend)
// static let WINDOWS_HOVER_PADDING                    = 4.0;     // Extend outside window for hovering/resizing (maxxed with TouchPadding) and inside windows for borders. Affect FindHoveredWindow().
pub const WINDOWS_HOVER_PADDING: f32 = 4.0;

// static let WINDOWS_RESIZE_FROM_EDGES_FEEDBACK_TIMER = 0.04;    // Reduce visual noise by only highlighting the border after a certain time.
pub const WINDOWS_RESIZE_FROM_EDGES_FEEDBACK_TIMER: f32 = 0.04;

// typedef void    (*ImGuiSizeCallback)(ImGuiSizeCallbackData* data);              // Callback function for ImGui::SetNextWindowSizeConstraints()
pub type ImGuiSizeCallback = fn(*mut SizeCallbackData);

// flags for ImGui::Begin()
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum WindowFlags {
    None = 0,
    NoTitleBar,                // Disable title-bar
    NoResize,                  // Disable user resizing with the lower-right grip
    NoMove,                    // Disable user moving the window
    NoScrollbar, // Disable scrollbars (window can still scroll with mouse or programmatically)
    NoScrollWithMouse, // Disable user vertically scrolling with mouse wheel. On child window, mouse wheel will be forwarded to the parent unless NoScrollbar is also set.
    NoCollapse, // Disable user collapsing window by double-clicking on it. Also referred to as window Menu Button (e.g. within a docking node).
    AlwaysAutoResize, // Resize every window to its content every frame
    NoBackground, // Disable drawing background color (WindowBg, etc.) and outside border. Similar as using set_netxt_window_bg_alpha(0.0).
    NoSavedSettings, // Never load/save settings in .ini file
    NoMouseInputs, // Disable catching mouse, hovering test with pass through.
    MenuBar,      // Has a menu-bar
    HorizontalScrollbar, // Allow horizontal scrollbar to appear (off by default). You may use SetNextWindowContentSize(Vector2D(width,0.0)); prior to calling Begin() to specify width. Read code in imgui_demo in the "Horizontal Scrolling" section.
    NoFocusOnAppearing,  // Disable taking focus when transitioning from hidden to visible state
    NoBringToFrontOnFocus, // Disable bringing window to front when taking focus (e.g. clicking on it or programmatically giving it focus)
    AlwaysVerticalScrollbar, // Always show vertical scrollbar (even if content_size.y < size.y)
    AlwaysHorizontalScrollbar, // Always show horizontal scrollbar (even if content_size.x < size.x)
    AlwaysUseWindowPadding, // Ensure child windows without border uses style.window_padding (ignored by default for non-bordered child windows, because more convenient)
    NoNavInputs,            // No gamepad/keyboard navigation within the window
    NoNavFocus, // No focusing toward this window with gamepad/keyboard navigation (e.g. skipped by CTRL+TAB)
    UnsavedDocument, // Display a dot next to the title. When used in a tab/docking context, tab is selected when clicking the x + closure is not assumed (will wait for user to stop submitting the tab). Otherwise closure is assumed when pressing the x, so if you keep submitting the tab may reappear at end of tab bar.
    NoDocking,       // Disable docking of this window
    // [Internal]
    NavFlattened, // [BETA] On child window: allow gamepad/keyboard navigation to cross over parent border to this child or between sibling child windows.
    ChildWindow,  // Don't use! For internal use by BeginChild()
    Tooltip,      // Don't use! For internal use by BeginTooltip()
    Popup,        // Don't use! For internal use by begin_popup()
    Modal,        // Don't use! For internal use by begin_popupModal()
    ChildMenu,    // Don't use! For internal use by BeginMenu()
    DockNodeHost, // Don't use! For internal use by Begin()/NewFrame()
                  // [Obsolete]
                  //WindowFlags_ResizeFromAnySide    = 1 << 17,  // [Obsolete] --> Set io.config_windows_resize_from_edges=true and make sure mouse cursors are supported by backend (io.backend_flags & ImGuiBackendFlags_HasMouseCursors)
}

// flags for ImGui::IsWindowFocused()
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum FocusedFlags {
    None = 0,
    ChildWindows,
    // Return true if any children of the window is focused
    RootWindow,
    // Test from root window (top most parent of the current hierarchy)
    AnyWindow,
    // Return true if any window is focused. Important: If you are trying to tell how to dispatch your low-level inputs, do NOT use this. Use 'io.want_capture_mouse' instead! Please read the FAQ!
    NoPopupHierarchy,
    // Do not consider popup hierarchy (do not treat popup emitter as parent of popup) (when used with _ChildWindows or _RootWindow)
    DockHierarchy, // Consider docking hierarchy (treat dockspace host as parent of docked window) (when used with _ChildWindows or _RootWindow)
                   // ImGuiFocusedFlags_RootAndChildWindows           = ImGuiFocusedFlags_RootWindow | ImGuiFocusedFlags_ChildWindows
}

// flags for ImGui::IsItemHovered(), ImGui::IsWindowHovered()
// Note: if you are trying to check whether your mouse should be dispatched to Dear ImGui or to your app, you should use 'io.want_capture_mouse' instead! Please read the FAQ!
// Note: windows with the WindowFlags_NoInputs flag are ignored by IsWindowHovered() calls.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum HoveredFlags {
    None = 0,
    // Return true if directly over the item/window, not obstructed by another window, not obstructed by an active popup or modal blocking inputs under them.
    ChildWindows,
    // IsWindowHovered() only: Return true if any children of the window is hovered
    RootWindow,
    // IsWindowHovered() only: Test from root window (top most parent of the current hierarchy)
    AnyWindow,
    // IsWindowHovered() only: Return true if any window is hovered
    NoPopupHierarchy,
    // IsWindowHovered() only: Do not consider popup hierarchy (do not treat popup emitter as parent of popup) (when used with _ChildWindows or _RootWindow)
    DockHierarchy,
    // IsWindowHovered() only: Consider docking hierarchy (treat dockspace host as parent of docked window) (when used with _ChildWindows or _RootWindow)
    AllowWhenBlockedByPopup,
    // Return true even if a popup window is normally blocking access to this item/window
    //ImGuiHoveredFlags_AllowWhenBlockedByModal     = 1 << 6,   // Return true even if a modal popup window is normally blocking access to this item/window. FIXME-TODO: Unavailable yet.
    AllowWhenBlockedByActiveItem,
    // Return true even if an active item is blocking access to this item/window. Useful for Drag and Drop patterns.
    AllowWhenOverlapped,
    // IsItemHovered() only: Return true even if the position is obstructed or overlapped by another window
    AllowWhenDisabled,
    // IsItemHovered() only: Return true even if the item is disabled
    NoNavOverride, // Disable using gamepad/keyboard navigation state when active, always query mouse.
                   // ImGuiHoveredFlags_RectOnly                      = ImGuiHoveredFlags_AllowWhenBlockedByPopup | ImGuiHoveredFlags_AllowWhenBlockedByActiveItem | ImGuiHoveredFlags_AllowWhenOverlapped,
                   // ImGuiHoveredFlags_RootAndChildWindows           = ImGuiHoveredFlags_RootWindow | ImGuiHoveredFlags_ChildWindows
}

// pub const RootAndChildWindows: i32           = DimgHoveredFlags::RootWindow | DimgHoveredFlags::ChildWindows;
pub const ROOT_AND_CHILD_WINDOWS: HashSet<HoveredFlags> =
    HashSet::from([HoveredFlags::RootWindow, HoveredFlags::ChildWindows]);

// pub const RectOnly : i32                     = DimgHoveredFlags::AllowWhenBlockedByPopup | DimgHoveredFlags::AllowWhenBlockedByActiveItem | DimgHoveredFlags::AllowWhenOverlapped;
pub const RECT_ONLY: HashSet<HoveredFlags> = HashSet::from([
    HoveredFlags::AllowWhenBlockedByPopup,
    HoveredFlags::AllowWhenBlockedByActiveItem,
    HoveredFlags::AllowWhenOverlapped,
]);

#[derive(Debug, Clone, Default)]
pub struct ShrinkWidthItem {
    // int         index;
    pub index: i32,
    // float       width;
    pub width: f32,
    // float       initial_width;
    pub initial_width: f32,
}
