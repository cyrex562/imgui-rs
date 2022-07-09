
use std::collections::HashMap;
//-----------------------------------------------------------------------------
// [SECTION] ImGuiContext (main Dear ImGui context)
//-----------------------------------------------------------------------------


use std::ptr::null_mut;

use crate::clipper::DimgListClipperData;
use crate::color::{DimgColorEditFlags, DimgColorMod};
use crate::combo::DimgComboPreviewData;
use crate::config::DimgConfigFlags;
use crate::window::DimgShrinkWidthItem;
use crate::direction::DimgDirection;
use crate::dock_context::DimgDockContext;
use crate::drag_drop::DimgDragDropFlags;
use crate::draw_channel::DimgDrawChannel;

use crate::draw_list::DimgDrawListSharedData;
use crate::font::DimgFont;
use crate::font_atlas::DimgFontAtlas;
use crate::group::DimgGroupData;
use crate::input::{DimgInputSource, DimgKey, DimgModFlags, DimgMouseCursor, DimgNavLayer};
use crate::input_event::DimgInputEvent;
use crate::io::{DimgIo, DimgPlatformIo};
use crate::item::{DimgLastItemData, DimgNextItemData};

use crate::log::ImGuiLogType;
use crate::nav::{DimgActivateFlags, DimgNavItemData, DimgNavMoveFlags, DimgScrollFlags};
use crate::payload::DimgPayload;
use crate::platform::{DimgPlatformImeData, DimgPlatformMonitor};
use crate::pool::ImGuiPool;
use crate::popup::DimgPopupData;
use crate::rect::DimgRect;
use crate::settings::DimgSettingsHandler;
use crate::style::{DimgStyle, DimgStyleMod};
use crate::tab_bar::DimgTabBar;
use crate::table::{DimgTable, DimgTableTempData, ImGuiTableSettings};

use crate::text_input_state::DimgInputTextState;
use crate::types::{DIMG_ID_INVALID, DimgId, DimgPtrOrIndex};
use crate::vec_nd::{DimgVec2D, DimgVec4};
use crate::viewport::DimgViewport;
use crate::window::{DimgItemFlags, DimgNextWindowData, DimgWindow, DimgWindowSettings, DimgWindowStackData};

#[derive()]
pub struct DimgContext {
    // bool                    Initialized;
    pub initialized: bool,
    // bool                    font_atlas_owned_by_context;            // io.fonts-> is owned by the ImGuiContext and will be destructed along with it.
    pub font_atlas_owned_by_context: bool,
    // ImGuiIO                 io;
    pub io: DimgIo,
    // ImGuiPlatformIO         platform_io;
    pub platform_io: DimgPlatformIo,
    // ImVector<ImGuiInputEvent> input_events_queue;                 // Input events which will be tricked/written into io structure.
    pub input_events_queue: Vec<DimgInputEvent>,
    // ImVector<ImGuiInputEvent> input_events_trail;                 // Past input events processed in NewFrame(). This is to allow domain-specific application to access e.g mouse/pen trail.
    pub input_events_trail: Vec<DimgInputEvent>,
    // ImGuiStyle              style;
    pub style: DimgStyle,
    // ImGuiConfigFlags        config_flags_curr_frame;               // = g.io.config_flags at the time of NewFrame()
    pub config_flags_curr_frame: DimgConfigFlags,
    // ImGuiConfigFlags        config_flags_last_frame;
    pub config_flags_last_frame: DimgConfigFlags,
    // ImFont*                 font;                               // (Shortcut) == font_stack.empty() ? io.font : font_stack.back()
    pub font: DimgFont,
    // float                   font_size;                           // (Shortcut) == font_base_size * g.current_window->font_window_scale == window->font_size(). Text height for current window.
    pub font_size: f32,
    // float                   font_base_size;                       // (Shortcut) == io.font_global_scale * font->scale * font->font_size. Base text height.
    pub font_base_size: f32,
    // ImDrawListSharedData    draw_list_shared_data;
    pub draw_list_shared_data: DimgDrawListSharedData,
    // double                  time;
    pub time: f32,
    // int                     frame_count;
    pub frame_count: i32,
    //int                     frame_count_ended;
    pub frame_count_ended: i32,
    // int                     frame_count_platform_ended;
    pub frame_count_platform_ended: i32,
    // int                     frame_count_rendered;
    pub frame_count_rendered: i32,
    // bool                    within_frame_scope;                   // Set by NewFrame(), cleared by EndFrame()
    pub within_frame_scope: bool,
    // bool                    within_frame_scope_with_implicit_window; // Set by NewFrame(), cleared by EndFrame() when the implicit debug window has been pushed
    pub within_frame_scope_with_implicit_window: bool,
    // bool                    within_end_child;                     // Set within EndChild()
    pub within_end_child: bool,
    // bool                    gc_compact_all;                       // Request full GC
    pub gc_compact_all: bool,
    // bool                    TestEngineHookItems;                // Will call test engine hooks: ImGuiTestEngineHook_ItemAdd(), ImGuiTestEngineHook_ItemInfo(), ImGuiTestEngineHook_Log()
    pub test_engine_hook_items: bool,
    // void*                   test_engine;                         // Test engine user data
    pub test_engine: Vec<u8>,
    // windows state
    // ImVector<ImGuiWindow*>  windows;                            // windows, sorted in display order, back to front
    pub windows: HashMap<DimgId, DimgWindow>,
    //Vec<ImGuiWindow>,
    // ImVector<ImGuiWindow*>  windows_focus_order;                  // Root windows, sorted in focus order, back to front.
    pub windows_focus_order: Vec<DimgId>,
    // ImVector<ImGuiWindow*>  windows_temp_sort_buffer;              // Temporary buffer used in EndFrame() to reorder windows so parents are kept before their child
    pub windows_temp_sort_buffer: Vec<DimgId>,
    // ImVector<ImGuiWindowStackData> current_window_stack;
    pub current_window_stack: Vec<DimgWindowStackData>,
    // ImGuiStorage            WindowsById;                        // Map window's ImGuiID to ImGuiWindow*
    // pub WindowsById: ImGuiStorage,
    // int                     windows_active_count;                 // Number of unique windows submitted by frame
    pub windows_active_count: i32,
    // ImVec2                  windows_hover_padding;                // Padding around resizable windows for which hovering on counts as hovering the window == ImMax(style.TouchExtraPadding, WINDOWS_HOVER_PADDING)
    pub windows_hover_padding: DimgVec2D,
    // ImGuiWindow*            current_window;                      // Window being drawn into
    pub current_window: DimgId,
    //*mut ImGuiWindow,
    // ImGuiWindow*            hovered_window;                      // Window the mouse is hovering. Will typically catch mouse inputs.
    pub hovered_window: DimgId,
    //*mut ImGuiWindow,
    // ImGuiWindow*            hovered_window_under_moving_window;     // Hovered window ignoring moving_window. Only set if moving_window is set.
    pub hovered_window_under_moving_window: DimgId,
    //*mut ImGuiWindow,
    // ImGuiDockNode*          hovered_dock_node;                    // [Debug] Hovered dock node.
    pub hovered_dock_node: DimgId,
    // ImGuiWindow*            moving_window;                       // Track the window we clicked on (in order to preserve focus). The actual window that is moved is generally moving_window->root_window_dock_tree.
    pub moving_window: DimgId,
    // ImGuiWindow*            wheeling_window;                     // Track the window we started mouse-wheeling on. Until a timer elapse or mouse has moved, generally keep scrolling the same window even if during the course of scrolling the mouse ends up hovering a child window.
    pub wheeling_window: DimgId,
    //*mut ImGuiWindow,
    // ImVec2                  wheeling_window_ref_mouse_pos;
    pub wheeling_window_ref_mouse_pos: DimgVec2D,
    // float                   wheeling_window_timer;
    pub wheeling_window_timer: f32,
    // Item/widgets state and tracking information
    // ImGuiID                 debug_hook_id_info;                    // Will call core hooks: debug_hook_id_info() from GetID functions, used by Stack Tool [next hovered_id/active_id to not pull in an extra cache-line]
    pub debug_hook_id_info: DimgId,
    // ImGuiID                 hovered_id;                          // Hovered widget, filled during the frame
    pub hovered_id: DimgId,
    // ImGuiID                 hovered_id_previous_frame;
    pub hovered_id_previous_frame: DimgId,
    // bool                    hovered_id_allow_overlap;
    pub hovered_id_allow_overlap: bool,
    // bool                    hovered_id_using_mouse_wheel;           // Hovered widget will use mouse wheel. Blocks scrolling the underlying window.
    pub hovered_id_using_mouse_wheel: bool,
    // bool                    hovered_id_previous_frame_using_mouse_wheel;
    pub hovered_id_previous_frame_using_mouse_wheel: bool,
    // bool                    hovered_id_disabled;                  // At least one widget passed the rect test, but has been discarded by disabled flag or popup inhibit. May be true even if hovered_id == 0.
    pub hovered_id_disabled: bool,
    // float                   hovered_id_timer;                     // Measure contiguous hovering time
    pub hovered_id_timer: f32,
    // float                   hovered_id_not_active_timer;            // Measure contiguous hovering time where the item has not been active
    pub hovered_id_not_active_timer: f32,
    // ImGuiID                 active_id;                           // active widget
    pub active_id: DimgId,
    // ImGuiID                 active_id_is_alive;                    // active widget has been seen this frame (we can't use a bool as the active_id may change within the frame)
    pub active_id_is_alive: DimgId,
    // float                   active_id_timer;
    pub active_id_timer: f32,
    // bool                    active_id_is_just_activated;            // Set at the time of activation for one frame
    pub active_id_is_just_activated: bool,
    // bool                    active_id_allow_overlap;               // active widget allows another widget to steal active id (generally for overlapping widgets, but not always)
    pub active_id_allow_overlap: bool,
    // bool                    active_id_no_clear_on_focus_loss;         // Disable losing active id if the active id window gets unfocused.
    pub active_id_no_clear_on_focus_loss: bool,
    // bool                    active_id_has_been_pressed_before;       // Track whether the active id led to a press (this is to allow changing between PressOnClick and PressOnRelease without pressing twice). Used by range_select branch.
    pub active_id_has_been_pressed_before: bool,
    // bool                    ActiveIdHasBeenEditedBefore;        // Was the value associated to the widget Edited over the course of the active state.
    pub active_id_hass_been_edited_before: bool,
    // bool                    active_id_has_been_edited_this_frame;
    pub active_id_has_been_edited_this_frame: bool,
    // ImVec2                  ActiveIdClickOffset;                // Clicked offset from upper-left corner, if applicable (currently only set by ButtonBehavior)
    pub active_id_clock_offset: DimgVec2D,
    // ImGuiWindow*            active_id_window;
    pub active_id_window: DimgId,
    // ImGuiInputSource        active_id_source;                     // Activating with mouse or nav (gamepad/keyboard)
    pub active_id_source: DimgInputSource,
    // int                     active_id_mouse_button;
    pub active_id_mouse_button: i32,
    // ImGuiID                 active_id_previous_frame;
    pub active_id_previous_frame: DimgId,
    //bool                    active_id_previous_frame_is_alive;
    pub active_id_previous_frame_is_alive: bool,
    // bool                    active_id_previous_frame_has_been_edited_before;
    pub active_id_previous_frame_has_been_edited_before: bool,
    // ImGuiWindow*            active_id_previous_frame_window;
    pub active_id_previous_frame_window: DimgId,
    // ImGuiID                 last_active_id;                       // Store the last non-zero active_id, useful for animation.
    pub last_active_id: DimgId,
    // float                   last_active_id_timer;                  // Store the last non-zero active_id timer since the beginning of activation, useful for animation.
    pub last_active_id_timer: f32,
    // Input Ownership
    // bool                    active_id_using_mouse_wheel;            // active widget will want to read mouse wheel. Blocks scrolling the underlying window.
    pub active_id_using_mouse_wheel: bool,
    // ImU32                   active_id_using_nav_dir_mask;            // active widget will want to read those nav move requests (e.g. can activate a button and move away from it)
    pub active_id_using_nav_dir_mask: u32,
    // ImU32                   active_id_using_nav_input_mask;          // active widget will want to read those nav inputs.
    pub active_id_using_nav_input_mask: u32,
    // ImBitArrayForNamedKeys  active_id_using_key_input_mask;          // active widget will want to read those key inputs. When we grow the ImGuiKey enum we'll need to either to order the enum to make useful keys come first, either redesign this into e.g. a small array.
    pub active_id_using_key_input_mask: Vec<DimgKey>,
    // Next window/item data
    // ImGuiItemFlags          current_item_flags;                      // == g.item_flags_stack.back()
    pub current_item_flags: DimgItemFlags,
    // ImGuiNextItemData       next_item_data;                       // Storage for SetNextItem** functions
    pub next_item_data: DimgNextItemData,
    // ImGuiLastItemData       last_item_data;                       // Storage for last submitted item (setup by ItemAdd)
    pub last_item_data: DimgLastItemData,
    // ImGuiNextWindowData     next_window_data;                     // Storage for SetNextWindow** functions
    pub next_window_data: DimgNextWindowData,

    // Shared stacks
    // ImVector<ImGuiColorMod> color_stack;                         // Stack for PushStyleColor()/PopStyleColor() - inherited by Begin()
    pub color_stack: Vec<DimgColorMod>,
    // ImVector<ImGuiStyleMod> style_var_stack;                      // Stack for PushStyleVar()/PopStyleVar() - inherited by Begin()
    pub style_var_stack: Vec<DimgStyleMod>,
    // ImVector<ImFont*>       font_stack;                          // Stack for PushFont()/PopFont() - inherited by Begin()
    pub font_stack: Vec<DimgFont>,
    // ImVector<ImGuiID>       focus_scope_stack;                    // Stack for PushFocusScope()/PopFocusScope() - not inherited by Begin(), unless child window
    pub focus_scope_stack: Vec<DimgId>,
    // ImVector<ImGuiItemFlags>item_flags_stack;                     // Stack for PushItemFlag()/PopItemFlag() - inherited by Begin()
    pub item_flags_stack: Vec<DimgItemFlags>,
    // ImVector<ImGuiGroupData>group_stack;                         // Stack for BeginGroup()/EndGroup() - not inherited by Begin()
    pub group_stack: Vec<DimgGroupData>,
    // ImVector<ImGuiPopupData>open_popup_stack;                     // Which popups are open (persistent)
    pub open_popup_stack: Vec<DimgPopupData>,
    // ImVector<ImGuiPopupData>begin_popup_stack;                    // Which level of BeginPopup() we are in (reset every frame)
    pub begin_popup_stack: Vec<DimgPopupData>,
    // int                     begin_menu_count;
    pub begin_menu_count: i32,

    // viewports
    // ImVector<ImGuiViewportP*> viewports;                        // active viewports (always 1+, and generally 1 unless multi-viewports are enabled). Each viewports hold their copy of ImDrawData.
    pub viewports: Vec<DimgViewport>,
    // float                   current_dpi_scale;                    // == current_viewport->dpi_scale
    pub current_dpi_scale: f32,
    // ImGuiViewportP*         current_viewport;                    // We track changes of viewport (happening in Begin) so we can call Platform_OnChangedViewport()
    pub current_viewport: DimgId,
    // ImGuiViewportP*         mouse_viewport;
    pub mouse_viewport: DimgId,
    // ImGuiViewportP*         mouse_last_hovered_viewport;           // Last known viewport that was hovered by mouse (even if we are not hovering any viewport any more) + honoring the _NoInputs flag.
    pub mouse_last_hovered_viewport: DimgId,
    // ImGuiID                 platform_last_focused_viewport_id;
    pub platform_last_focused_viewport_id: DimgId,
    // ImGuiPlatformMonitor    fallback_monitor;                    // Virtual monitor used as fallback if backend doesn't provide monitor information.
    pub fallback_monitor: DimgPlatformMonitor,
    // int                     viewport_front_most_stamp_count;        // Every time the front-most window changes, we stamp its viewport with an incrementing counter
    pub viewport_front_most_stamp_count: i32,
    // Gamepad/keyboard Navigation
    // ImGuiWindow*            nav_window;                          // Focused window for navigation. Could be called 'FocusedWindow'
    pub nav_window: DimgId,
    // ImGuiID                 nav_id;                              // Focused item for navigation
    pub nav_id: DimgId,
    // ImGuiID                 nav_focus_scope_id;                    // Identify a selection scope (selection code often wants to "clear other items" when landing on an item of the selection set)
    pub nav_focus_scope_id: DimgId,
    // ImGuiID                 nav_activate_id;                      // ~~ (g.active_id == 0) && IsNavInputPressed(ImGuiNavInput_Activate) ? nav_id : 0, also set when calling ActivateItem()
    pub nav_activate_id: DimgId,
    // ImGuiID                 nav_activate_down_id;                  // ~~ IsNavInputDown(ImGuiNavInput_Activate) ? nav_id : 0
    pub nav_activate_down_id: DimgId,
    // ImGuiID                 nav_activate_pressed_id;               // ~~ IsNavInputPressed(ImGuiNavInput_Activate) ? nav_id : 0
    pub nav_activate_pressed_id: DimgId,
    // ImGuiID                 nav_activate_input_id;                 // ~~ IsNavInputPressed(ImGuiNavInput_Input) ? nav_id : 0; ImGuiActivateFlags_PreferInput will be set and nav_activate_id will be 0.
    pub nav_activate_input_id: DimgId,
    // ImGuiActivateFlags      nav_activate_flags;
    pub nav_activate_flags: DimgActivateFlags,
    // ImGuiID                 nav_just_moved_to_id;                   // Just navigated to this id (result of a successfully MoveRequest).
    pub nav_just_moved_to_id: DimgId,
    // ImGuiID                 nav_just_moved_to_focus_scope_id;         // Just navigated to this focus scope id (result of a successfully MoveRequest).
    pub nav_just_moved_to_focus_scope_id: DimgId,
    // ImGuiModFlags           nav_just_moved_to_key_mods;
    pub nav_just_moved_to_key_mods: DimgModFlags,
    // ImGuiID                 nav_next_activate_id;                  // Set by ActivateItem(), queued until next frame.
    pub nav_next_activate_id: DimgId,
    // ImGuiActivateFlags      nav_next_activate_flags;
    pub nav_next_activate_flags: DimgActivateFlags,
    // ImGuiInputSource        nav_input_source;                     // Keyboard or Gamepad mode? THIS WILL ONLY BE None or NavGamepad or NavKeyboard.
    pub nav_input_source: DimgInputSource,
    // ImGuiNavLayer           nav_layer;                           // Layer we are navigating on. For now the system is hard-coded for 0=main contents and 1=menu/title bar, may expose layers later.
    pub nav_layer: DimgNavLayer,
    // bool                    nav_id_is_alive;                       // Nav widget has been seen this frame ~~ nav_rect_rel is valid
    pub nav_id_is_alive: bool,
    // bool                    nav_mouse_pos_dirty;                   // When set we will update mouse position if (io.config_flags & ImGuiConfigFlags_NavEnableSetMousePos) if set (NB: this not enabled by default)
    pub nav_mouse_pos_dirty: bool,
    // bool                    NavDisableHighlight;                // When user starts using mouse, we hide gamepad/keyboard highlight (NB: but they are still available, which is why NavDisableHighlight isn't always != nav_disable_mouse_hover)
    pub nav_disable_high_light: bool,
    // bool                    nav_disable_mouse_hover;               // When user starts using gamepad/keyboard, we hide mouse hovering highlight until mouse is touched again.
    pub nav_disable_mouse_hover: bool,
    // Navigation: Init & Move Requests
    // bool                    nav_any_request;                      // ~~ NavMoveRequest || nav_init_request this is to perform early out in ItemAdd()
    pub nav_any_request: bool,
    // bool                    nav_init_request;                     // Init request for appearing window to select first item
    pub nav_init_request: bool,
    // bool                    nav_init_request_from_move;
    pub nav_init_request_from_move: bool,
    // ImGuiID                 nav_init_result_id;                    // Init request result (first item of the window, or one for which SetItemDefaultFocus() was called)
    pub nav_init_result_id: DimgId,
    // ImRect                  nav_init_result_rect_rel;               // Init request result rectangle (relative to parent window)
    pub nav_init_result_rect_rel: DimgRect,
    // bool                    nav_move_submitted;                   // Move request submitted, will process result on next NewFrame()
    pub nav_move_submitted: bool,
    // bool                    nav_move_scoring_items;                // Move request submitted, still scoring incoming items
    pub nav_move_scoring_items: bool,
    // bool                    nav_move_forward_to_next_frame;
    pub nav_move_forward_to_next_frame: bool,
    // ImGuiNavMoveFlags       nav_move_flags;
    pub nav_move_flags: DimgNavMoveFlags,
    // ImGuiScrollFlags        nav_move_scroll_flags;
    pub nav_move_scroll_flags: DimgScrollFlags,
    // ImGuiModFlags           nav_move_key_mods;
    pub nav_move_key_mods: DimgModFlags,
    // ImGuiDir                nav_move_dir;                         // Direction of the move request (left/right/up/down)
    pub nav_move_dir: DimgDirection,
    // ImGuiDir                NavMoveDirForDebug;
    pub nav_move_dir_for_debug: DimgDirection,
    // ImGuiDir                nav_move_clip_dir;                     // FIXME-NAV: Describe the purpose of this better. Might want to rename?
    pub nav_move_clip_dir: DimgDirection,
    // ImRect                  nav_scoring_rect;                     // Rectangle used for scoring, in screen space. Based of window->nav_rect_rel[], modified for directional navigation scoring.
    pub nav_scoring_rect: DimgRect,
    // ImRect                  nav_scoring_no_clip_rect;               // Some nav operations (such as PageUp/PageDown) enforce a region which clipper will attempt to always keep submitted
    pub nav_scoring_no_clip_rect: DimgRect,
    // int                     nav_scoring_debug_count;               // Metrics for debugging
    pub nav_scoring_debug_count: i32,
    // int                     nav_tabbing_dir;                      // Generally -1 or +1, 0 when tabbing without a nav id
    pub nav_tabbing_dir: i32,
    // int                     nav_tabbing_counter;                  // >0 when counting items for tabbing
    pub nav_tabbing_counter: i32,
    // ImGuiNavItemData        nav_move_result_local;                 // Best move request candidate within nav_window
    pub nav_move_result_local: DimgNavItemData,
    // ImGuiNavItemData        nav_move_result_local_visible;          // Best move request candidate within nav_window that are mostly visible (when using ImGuiNavMoveFlags_AlsoScoreVisibleSet flag)
    pub nav_move_result_local_visible: DimgNavItemData,
    // ImGuiNavItemData        nav_move_result_other;                 // Best move request candidate within nav_window's flattened hierarchy (when using ImGuiWindowFlags_NavFlattened flag)
    pub nav_move_result_other: DimgNavItemData,
    // ImGuiNavItemData        nav_tabbing_result_first;              // First tabbing request candidate within nav_window and flattened hierarchy
    pub nav_tabbing_result_first: DimgNavItemData,
    // Navigation: Windowing (CTRL+TAB for list, or Menu button + keys or directional pads to move/resize)
    // ImGuiWindow*            nav_windowing_target;                 // Target window when doing CTRL+Tab (or Pad Menu + FocusPrev/Next), this window is temporarily displayed top-most!
    pub nav_windowing_target: DimgId,
    // ImGuiWindow*            nav_windowing_target_anim;             // Record of last valid nav_windowing_target until DimBgRatio and nav_windowing_highlight_alpha becomes 0.0, so the fade-out can stay on it.
    pub nav_windowing_target_anim: DimgId,
    // ImGuiWindow*            nav_windowing_list_window;             // Internal window actually listing the CTRL+Tab contents
    pub nav_windowing_list_window: DimgId,
    // float                   nav_windowing_timer;
    pub nav_windowing_timer: f32,
    // float                   nav_windowing_highlight_alpha;
    pub nav_windowing_highlight_alpha: f32,
    // bool                    nav_windowing_toggle_layer;
    pub nav_windowing_toggle_layer: bool,
    // Render
    // float                   DimBgRatio;                         // 0.0..1.0 animation when fading in a dimming background (for modal window and CTRL+TAB list)
    pub dim_bg_ration: f32,
    // ImGuiMouseCursor        mouse_cursor;
    pub mouse_cursor: DimgMouseCursor,
    // Drag and Drop
    // bool                    drag_drop_active;
    pub drag_drop_active: bool,
    // bool                    drag_drop_within_source;               // Set when within a BeginDragDropXXX/EndDragDropXXX block for a drag source.
    pub drag_drop_within_source: bool,
    // bool                    drag_drop_within_target;               // Set when within a BeginDragDropXXX/EndDragDropXXX block for a drag target.
    pub drag_drop_within_target: bool,
    // ImGuiDragDropFlags      drag_drop_source_flags;
    pub drag_drop_source_flags: DimgDragDropFlags,
    // int                     drag_drop_source_frame_count;
    pub drag_drop_source_frame_count: i32,
    // int                     drag_drop_mouse_button;
    pub drag_drop_mouse_button: i32,
    // ImGuiPayload            drag_drop_payload;
    pub drag_drop_payload: DimgPayload,
    // ImRect                  drag_drop_target_rect;                 // Store rectangle of current target candidate (we favor small targets when overlapping)
    pub drag_drop_target_rect: DimgRect,
    // ImGuiID                 drag_drop_target_id;
    pub drag_drop_target_id: DimgId,
    // ImGuiDragDropFlags      drag_drop_accept_flags;
    pub drag_drop_accept_flags: DimgDragDropFlags,
    // float                   drag_drop_accept_id_curr_rect_surface;    // Target item surface (we resolve overlapping targets by prioritizing the smaller surface)
    pub drag_drop_accept_id_curr_rect_surface: f32,
    // ImGuiID                 drag_drop_accept_id_curr;               // Target item id (set at the time of accepting the payload)
    pub drag_drop_accept_id_curr: DimgId,
    // ImGuiID                 drag_drop_accept_id_prev;               // Target item id from previous frame (we need to store this to allow for overlapping drag and drop targets)
    pub drag_drop_accept_id_prev: DimgId,
    // int                     drag_drop_accept_frame_count;           // Last time a target expressed a desire to accept the source
    pub drag_drop_accept_frame_count: i32,
    // ImGuiID                 drag_drop_hold_just_pressed_id;          // Set when holding a payload just made ButtonBehavior() return a press.
    pub drag_drop_hold_just_pressed_id: DimgId,
    // ImVector<unsigned char> drag_drop_payload_buf_heap;             // We don't expose the ImVector<> directly, ImGuiPayload only holds pointer+size
    pub drag_drop_payload_buf_heap: Vec<u8>,
    // unsigned char           drag_drop_payload_buf_local[16];        // Local buffer for small payloads
    pub drag_drop_payload_buf_local: [u8; 16],
    // Clipper
    // int                             clipper_temp_data_stacked;
    pub clipper_temp_data_stacked: i32,
    // ImVector<ImGuiListClipperData>  clipper_temp_data;
    pub clipper_temp_data: Vec<DimgListClipperData>,
    // tables
    // ImGuiTable*                     current_table;
    pub current_table: DimgTable,
    // int                             tables_temp_data_stacked;      // Temporary table data size (because we leave previous instances undestructed, we generally don't use tables_temp_data.size)
    pub tables_temp_data_stacked: i32,
    // ImVector<ImGuiTableTempData>    tables_temp_data;             // Temporary table data (buffers reused/shared across instances, support nesting)
    pub tables_temp_data: Vec<DimgTableTempData>,
    // ImGuiPool<ImGuiTable>              tables;                     // Persistent table data
    pub tables: HashMap<DimgId, DimgTable>,
    // ImVector<float>                 tables_last_time_active;       // Last used timestamp of each tables (SOA, for efficient GC)
    pub tables_last_time_active: Vec<f32>,
    // ImVector<ImDrawChannel>         draw_channels_temp_merge_buffer;
    pub draw_channels_temp_merge_buffer: Vec<DimgDrawChannel>,
    // Tab bars
    // ImGuiTabBar*                    current_tab_bar;
    pub current_tab_bar: DimgId,
    // ImGuiPool<ImGuiTabBar>             tab_bars;
    pub tab_bars: Vec<DimgTabBar>,
    // ImVector<ImGuiPtrOrIndex>       current_tab_bar_stack;
    pub current_tab_bar_stack: Vec<DimgPtrOrIndex>,
    // ImVector<ImGuiShrinkWidthItem>  shrink_width_buffer;
    pub shrink_width_buffer: Vec<DimgShrinkWidthItem>,
    // Widget state
    // ImVec2                  mouse_last_valid_pos;
    pub mouse_last_valid_pos: DimgVec2D,
    // ImGuiInputTextState     input_text_state;
    pub input_text_state: DimgInputTextState,
    // ImFont                  input_text_password_font;
    pub input_text_password_font: DimgFont,
    // ImGuiID                 temp_input_id;                        // Temporary text input when CTRL+clicking on a slider, etc.
    pub temp_input_id: DimgId,
    // ImGuiColorEditFlags     color_edit_options;                   // Store user options for color edit widgets
    pub color_edit_options: DimgColorEditFlags,
    // float                   color_edit_last_hue;                   // Backup of last Hue associated to LastColor, so we can restore Hue in lossy RGB<>HSV round trips
    pub color_edit_last_hue: f32,
    // float                   color_edit_last_sat;                   // Backup of last Saturation associated to LastColor, so we can restore Saturation in lossy RGB<>HSV round trips
    pub color_edit_last_sat: f32,
    // ImU32                   color_edit_last_color;                 // RGB value with alpha set to 0.
    pub color_edit_last_color: u32,
    // ImVec4                  color_picker_ref;                     // Initial/reference color at the time of opening the color picker.
    pub color_picker_ref: DimgVec4,
    // ImGuiComboPreviewData   combo_preview_data;
    pub combo_preview_data: DimgComboPreviewData,
    // float                   slider_grab_click_offset;
    pub slider_grab_click_offset: f32,
    // float                   slider_current_accum;                 // Accumulated slider delta when using navigation controls.
    pub slider_current_accum: f32,
    // bool                    slider_current_accum_dirty;            // Has the accumulated slider delta changed since last time we tried to apply it?
    pub slider_current_accum_dirty: bool,
    // bool                    drag_current_accum_dirty;
    pub drag_current_accum_dirty: bool,
    // float                   drag_current_accum;                   // Accumulator for dragging modification. Always high-precision, not rounded by end-user precision settings
    pub drag_current_accum: f32,
    // float                   drag_speed_default_ratio;              // If speed == 0.0, uses (max-min) * drag_speed_default_ratio
    pub drag_speed_default_ratio: f32,
    // float                   scrollbar_click_delta_to_grab_center;    // Distance between mouse and center of grab box, normalized in parent space. Use storage?
    pub scrollbar_click_delta_to_grab_center: f32,
    // float                   disabled_alpha_backup;                // Backup for style.Alpha for BeginDisabled()
    pub disabled_alpha_backup: f32,
    // short                   disabled_stack_size;
    pub disabled_stack_size: i16,
    // short                   tooltip_override_count;
    pub tooltip_override_count: i16,
    // float                   tooltip_slow_delay;                   // time before slow tooltips appears (FIXME: This is temporary until we merge in tooltip timer+priority work)
    pub tooltip_slow_delay: f32,
    // ImVector<char>          clipboard_handler_data;               // If no custom clipboard handler is defined
    pub clipboard_handler_data: Vec<u8>,
    // ImVector<ImGuiID>       menus_id_submitted_this_frame;          // A list of menu IDs that were rendered at least once
    pub menus_id_submitted_this_frame: Vec<DimgId>,
    // Platform support
    // ImGuiPlatformImeData    platform_ime_data;                    // data updated by current frame
    pub platform_ime_data: DimgPlatformImeData,
    // ImGuiPlatformImeData    platform_ime_data_prev;                // Previous frame data (when changing we will call io.SetPlatformImeDataFn
    pub platform_ime_data_prev: DimgPlatformImeData,
    // ImGuiID                 platform_ime_viewport;
    pub platform_ime_viewport: DimgId,
    // char                    PlatformLocaleDecimalPoint;         // '.' or *localeconv()->decimal_point
    pub platform_local_decimal_point: char,
    // Extensions
    // FIXME: We could provide an API to register one slot in an array held in ImGuiContext?
    // ImGuiDockContext        dock_context;
    pub dock_context: DimgDockContext,
    // Settings
    // bool                    settings_loaded;
    pub settings_loaded: bool,
    // float                   settings_dirty_timer;                 // Save .ini Settings to memory when time reaches zero
    pub settings_dirty_timer: f32,
    // ImGuiTextBuffer         settings_ini_data;                    // In memory .ini settings
    pub settings_ini_data: Vec<u8>,
    // ImVector<ImGuiSettingsHandler>      settings_handlers;       // List of .ini settings handlers
    pub settings_handlers: Vec<DimgSettingsHandler>,
    // ImChunkStream<ImGuiWindowSettings>  settings_windows;        // ImGuiWindow .ini settings entries
    pub settings_windows: Vec<DimgWindowSettings>,
    // ImChunkStream<ImGuiTableSettings>   SettingsTables;         // ImGuiTable .ini settings entries
    pub settings_tabls: Vec<DimgTableSettings>,
    // ImVector<ImGuiContextHook>          hooks;                  // hooks for extensions (e.g. test engine)
    pub hooks: Vec<DimgContextHook>,
    // ImGuiID                             hook_id_next;             // Next available HookId
    pub hook_id_next: DimgId,
    // Capture/Logging
    // bool                    log_enabled;                         // Currently capturing
    // pub log_enabled: bool,
    // ImGuiLogType            log_type;                            // Capture target
    // pub log_type: ImGuiLogType,
    // ImFileHandle            log_file;                            // If != NULL log to stdout/ file
    pub log_file: String,
    // ImGuiTextBuffer         LogBuffer;                          // Accumulation buffer when log to clipboard. This is pointer so our GImGui static constructor doesn't call heap allocators.
    // pub LogBuffer: ImGuiTextBuffer,
    // const char*             LogNextPrefix;
    // pub LogNextPrefix: *const c_char,
    // const char*             LogNextSuffix;
    // pub LogNextSuffix: *const c_char,
    // float                   LogLinePosY;
    // pub LogLinePosY: f32,
    // bool                    LogLineFirstItem;
    // pub LogLineFirstLine: bool,
    // int                     LogDepthRef;
    // pub LogDepthRef: i32,
    // int                     LogDepthToExpand;
    // pub LogDepthToExpand: i32,
    // int                     LogDepthToExpandDefault;            // Default/stored value for LogDepthMaxExpand if not specified in the LogXXX function call.
    // pub LogDepthToExpandDefault: i32,
    // Debug Tools
    // ImGuiDebugLogFlags      DebugLogFlags;
    // pub DebugLogFlags: ImGuiDebugLogFlags,
    // ImGuiTextBuffer         DebugLogBuf;
    // pub DebugLogBuf: ImGuiTextBuffer,
    // bool                    debug_item_picker_active;              // Item picker is active (started with DebugStartItemPicker())
    pub debug_item_picker_active: bool,
    // ImGuiID                 debug_item_picker_break_id;             // Will call IM_DEBUG_BREAK() when encountering this id
    pub debug_item_picker_break_id: DimgId,
    // ImGuiMetricsConfig      debug_metrics_config;
    pub debug_metrics_config: ImGuiMetricsConfig,
    // ImGuiStackTool          debug_stack_tool;
    pub debug_stack_tool: ImGuiStackTool,

    // Misc
    // float                   framerate_sec_per_frame[120];          // Calculate estimate of framerate for user over the last 2 seconds.
    pub framerate_sec_per_frame: [f32; 128],
    // int                     framerate_sec_per_frame_idx;
    pub framerate_sec_per_frame_idx: i32,
    // int                     framerate_sec_per_frame_count;
    pub framerate_sec_per_frame_count: i32,
    // float                   framerate_sec_per_frame_accum;
    pub framerate_sec_per_frame_accum: f32,
    // int                     want_capture_mouse_next_frame;          // Explicit capture override via SetNextFrameWantCaptureMouse()/SetNextFrameWantCaptureKeyboard(). Default to -1.
    pub want_capture_mouse_next_frame: i32,
    // int                     want_capture_keyboard_next_frame;       // "
    pub want_capture_keyboard_next_frame: i32,
    // int                     WantTextInputNextFrame;
    pub want_input_next_frame: i32,
    // ImVector<char>          temp_buffer;                         // Temporary text buffer
    pub temp_buffer: Vec<u8>,

}

impl DimgContext {
    // ImGuiContext(ImFontAtlas* shared_font_atlas)
    pub fn new(shared_font_atlas: &mut DimgFontAtlas) -> Self
    {
        Self {
            initialized: false,
            config_flags_curr_frame: DimgConfigFlags::ImGuiConfigFlags_None,
            config_flags_last_frame: DimgConfigFlags::ImGuiConfigFlags_None,
            font_atlas_owned_by_context: true,
            font_size: 0.0,
            font_base_size: 0.0,
            io: DimgIo::new(),
            platform_io: DimgPlatformIo::new(),
            input_events_queue: vec![],
            input_events_trail: vec![],
            time: 0.0,
            frame_count: 0,
            frame_count_ended: -1,
            frame_count_platform_ended: -1,
            frame_count_rendered: -1,
            within_frame_scope: false,
            within_frame_scope_with_implicit_window: false,
            within_end_child: false,
            gc_compact_all: false,
            // TestEngineHookItems: false,
            test_engine: vec![],
            windows: HashMap::new(),
            windows_focus_order: vec![],
            windows_temp_sort_buffer: vec![],
            current_window_stack: vec![],
            // WindowsById: vec![],
            windows_active_count: 0,
            windows_hover_padding: Default::default(),
            current_window: DIMG_ID_INVALID,
            hovered_window: DIMG_ID_INVALID,
            hovered_window_under_moving_window: DIMG_ID_INVALID,
            hovered_dock_node: DIMG_ID_INVALID,
            moving_window: DIMG_ID_INVALID,
            wheeling_window: DIMG_ID_INVALID,
            wheeling_window_ref_mouse_pos: Default::default(),
            wheeling_window_timer: 0.0,

            debug_hook_id_info: 0,
            hovered_id: 0,
            hovered_id_previous_frame: 0,
            // hovered_id_previous_frame: (),
            hovered_id_allow_overlap: false,
            hovered_id_using_mouse_wheel: false,
            hovered_id_previous_frame_using_mouse_wheel: false,
            hovered_id_disabled: false,
            hovered_id_timer: 0.0,
            hovered_id_not_active_timer: 0.0,
            active_id: 0,
            active_id_is_alive: 0,
            active_id_timer: 0.0,
            active_id_is_just_activated: false,
            active_id_allow_overlap: false,
            active_id_no_clear_on_focus_loss: false,
            active_id_has_been_pressed_before: false,
            // ActiveIdHasBeenEditedBefore: false,
            active_id_has_been_edited_this_frame: false,
            // ActiveIdClickOffset: ImVec2::new( - 1, -1),
            active_id_window: u32::MAX,
            active_id_source: DimgInputSource::None,
            active_id_mouse_button: - 1,
            active_id_previous_frame: 0,
            active_id_previous_frame_is_alive: false,
            active_id_previous_frame_has_been_edited_before: false,
            active_id_previous_frame_window: u32::MAX,
            last_active_id: 0,
            last_active_id_timer: 0.0,
            active_id_using_mouse_wheel: false,
            active_id_using_nav_dir_mask: 0x00,
            active_id_using_nav_input_mask: 0x00,
            // active_id_using_key_input_mask.ClearAllBits(),
            active_id_using_key_input_mask: vec![],
            current_item_flags: DimgItemFlags::None,
            next_item_data: DimgNextItemData::default(),
            last_item_data: DimgLastItemData::default(),
            next_window_data: DimgNextWindowData::default(),
            color_stack: vec![],
            style_var_stack: vec![],
            font_stack: vec![],
            focus_scope_stack: vec![],
            item_flags_stack: vec![],
            group_stack: vec![],
            open_popup_stack: vec![],
            begin_popup_stack: vec![],
            begin_menu_count: 0,

            viewports: vec![],
            current_dpi_scale: 0.0,
            current_viewport: DIMG_ID_INVALID,
            mouse_viewport: DIMG_ID_INVALID,
            // mouse_last_hovered_viewport: NULL,
            mouse_last_hovered_viewport: DIMG_ID_INVALID,
            platform_last_focused_viewport_id: 0,
            fallback_monitor: DimgPlatformMonitor::default(),
            viewport_front_most_stamp_count: 0,
            nav_window: u32::MAX ,
            nav_id: 0,
            nav_focus_scope_id: 0,
            nav_activate_id: 0,
            nav_activate_down_id: 0,
            nav_activate_pressed_id: 0,
            nav_just_moved_to_id: 0,
            nav_activate_flags: DimgActivateFlags::None,
            nav_just_moved_to_key_mods: ImGuiModeFlags::None,
            nav_next_activate_id: 0,
            nav_next_activate_flags: DimgActivateFlags::None,
            nav_input_source: DimgInputSource::None,
            nav_layer: DimgNavLayer::Main,
            nav_id_is_alive: false,
            nav_mouse_pos_dirty: false,
            // NavDisableHighlight: true,
            nav_disable_mouse_hover: false,
            nav_any_request: false,
            nav_init_request: false,
            nav_init_request_from_move: false,
            nav_init_result_id: 0,
            nav_move_submitted: false,
            nav_move_scoring_items: false,
            nav_move_forward_to_next_frame: false,
            nav_move_flags: DimgNavMoveFlags::None,
            nav_move_scroll_flags: DimgScrollFlags::None,
            nav_move_key_mods: DimgModFlags::None,
            nav_move_dir: DimgDirection::None,
            // NavMoveDirForDebug: NavMoveClipDir: ImGuiDir_None,
            nav_move_dir_for_debug: DimgDirection::None,
            nav_move_clip_dir: DimgDirection::None,
            nav_scoring_rect: DimgRect::default(),
            nav_scoring_no_clip_rect: DimgRect::default(),
            nav_scoring_debug_count: 0,
            nav_tabbing_dir: 0,
            nav_tabbing_counter: 0,
            nav_move_result_local: DimgNavItemData::default(),
            nav_move_result_local_visible: DimgNavItemData::default(),
            nav_move_result_other: DimgNavItemData::default(),
            nav_tabbing_result_first: DimgNavItemData::default(),
            nav_windowing_target: u32::MAX,
            nav_windowing_target_anim: u32::MAX,
            nav_windowing_list_window: u32::MAX,
            // nav_windowing_target_anim: (),
            // nav_windowing_list_window: (),
            nav_windowing_timer: 0.0,
            nav_windowing_highlight_alpha: 0.0,
            nav_windowing_toggle_layer: false,
            // DimBgRatio: 0.0,
            mouse_cursor: DimgMouseCursor::Arrow,
            drag_drop_active: false,
            drag_drop_within_source: false,
            drag_drop_within_target: false,
            // drag_drop_within_source: false,
            // drag_drop_within_target: false,
            drag_drop_source_flags: DimgDragDropFlags::None,
            drag_drop_source_frame_count: - 1,
            drag_drop_mouse_button: - 1,
            drag_drop_payload: Default::default(),
            drag_drop_target_rect: DimgRect::default(),
            drag_drop_target_id: 0,
            drag_drop_accept_flags: DimgDragDropFlags::None,
            drag_drop_accept_id_curr_rect_surface: 0.0,
            drag_drop_accept_id_curr: 0,
            drag_drop_accept_id_prev: 0,
            // drag_drop_accept_id_curr: 0,
            drag_drop_accept_frame_count: - 1,
            drag_drop_hold_just_pressed_id: 0,
            drag_drop_payload_buf_local: [0;16],
            clipper_temp_data_stacked: 0,
            clipper_temp_data: vec![],
            current_table: DimgId,
            tables_temp_data_stacked: 0,
            tables_temp_data: vec![],
            tables: ImGuiPool::default(),
            tables_last_time_active: vec![],
            draw_channels_temp_merge_buffer: vec![],
            current_tab_bar: DimgId,
            tab_bars: ImGuiPool::default(),
            current_tab_bar_stack: vec![],
            shrink_width_buffer: vec![],
            mouse_last_valid_pos: Default::default(),
            input_text_state: DimgInputTextState::default(),
            input_text_password_font: Default::default(),
            temp_input_id: 0,
            color_edit_options: DimgColorEditFlags::DefaultOptions,
            color_edit_last_hue: 0.0,
            color_edit_last_sat: 0.0,
            color_edit_last_color: 0,
            color_picker_ref: Default::default(),
            combo_preview_data: (),
            slider_grab_click_offset: 0.0,
            slider_current_accum: 0.0,
            slider_current_accum_dirty: false, drag_current_accum_dirty: false, drag_current_accum: 0.0,
            drag_speed_default_ratio: 1.0 / 100.0,
            disabled_alpha_backup: 0.0,
            disabled_stack_size: 0,
            scrollbar_click_delta_to_grab_center: 0.0,
            tooltip_override_count: 0,
            tooltip_slow_delay: 0.50,

            clipboard_handler_data: vec![],
            menus_id_submitted_this_frame: vec![],
            platform_ime_data: DimgPlatformImeData::new(DimgVec2D::new(0.0, 0.0)),
            // platform_ime_data_prev.InputPos: ImVec2( - 1.0,
            // -1.0), // Different to ensure initial submission
            platform_ime_data_prev: DimgPlatformImeData::new(DimgVec2D::new(-1.0, -1.0)),
            platform_ime_viewport: 0,
            platform_local_decimal_point: '.',
            // PlatformLocaleDecimalPoint: '.',
            settings_loaded: false,
            settings_dirty_timer: 0.0,
            settings_ini_data: Default::default(),
            settings_handlers: vec![],
            settings_windows: (),
            settings_tabls: (),
            hooks: vec![],
            hook_id_next: 0,
            log_enabled: false,
            log_type: ImGuiLogType::None,
            // LogNextPrefix: null_mut(),
            // LogNextSuffix: null_mut(),
            log_file: null_mut(),
            // LogLinePosY: f32::MAX,
            // LogLineFirstItem: false,
            // LogDepthRef: 0,
            // LogDepthToExpand: 2,
            // LogDepthToExpandDefault: 2,
            // DebugLogFlags: ImGuiDebugLogFlags::OutputToTTY,
            // DebugLogBuf: Default::default(),
            debug_item_picker_active: false, debug_item_picker_break_id: 0,

            debug_metrics_config: (),
            framerate_sec_per_frame: [0.0;128],
            framerate_sec_per_frame_idx: 0,
            framerate_sec_per_frame_count: 0,
            framerate_sec_per_frame_accum: 0.0,
            want_capture_mouse_next_frame: -1,
            want_capture_keyboard_next_frame: -1,
            // WantTextInputNextFrame: - 1,
            // want_capture_keyboard_next_frame: 0,
            want_input_next_frame: 0,
            style: DimgStyle::new(),
            draw_list_shared_data: DimgDrawListSharedData::default(),
            test_engine_hook_items: false,
            active_id_hass_been_edited_before: false,
            active_id_clock_offset: Default::default(),
            nav_activate_input_id: 0,
            nav_just_moved_to_focus_scope_id: 0,
            nav_disable_high_light: false,
            nav_init_result_rect_rel: DimgRect::default(),
            // nav_windowing_highlight_alpha: 0.0,
            dim_bg_ration: 0.0,
            drag_drop_payload_buf_heap: vec![],
            // color_edit_last_sat: 0.0,
            dock_context: (),
            // LogBuffer: Default::default(),
            // LogNextSuffix: (),
            // LogLineFirstLine: false,
            // LogDepthToExpandDefault: 0,
            debug_stack_tool: (),
            // framerate_sec_per_frame_count: 0,
            temp_buffer: vec![]
        }
    }
}

pub enum DimgContextHookType { NewFramePre, NewFramePost, EndFramePre, EndFramePost, RenderPre, RenderPost, Shutdown, PendingRemoval_ }

//-----------------------------------------------------------------------------
// [SECTION] Generic context hooks
//-----------------------------------------------------------------------------
#[derive(Default,Debug,Clone)]
pub struct DimgContextHook
{
    // ImGuiID                     HookId;     // A unique id assigned by AddContextHook()
    pub hook_id: DimgId,
    // ImGuiContextHookType        Type;
    pub hook_type: DimgContextHookType,
    // ImGuiID                     Owner;
    pub owner: DimgId,
    // ImGuiContextHookCallback    Callback;
    pub callback: Option<DimgContextHookCallback>,
    // void*                       user_data;
    pub user_data: Vec<u8>,
    // ImGuiContextHook()          { memset(this, 0, sizeof(*this)); }
}
