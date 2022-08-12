use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
//-----------------------------------------------------------------------------
// [SECTION] ImGuiContext (main Dear ImGui context)
//-----------------------------------------------------------------------------

use crate::color::{ColorEditFlags, StackedColorModifier, COLOR_EDIT_FLAGS_DFLT_OPTS};
use crate::combo::ComboPreviewData;
use crate::config::ConfigFlags;
use crate::dock::context::DockContext;
use crate::dock::node::dock_node::DockNode;
use crate::drag_drop::DragDropFlags;
use crate::draw::cmd::DrawCmd;
use crate::draw::command::DrawCommand;
use crate::draw::list::DrawList;
use crate::draw_channel::DrawChannel;
use crate::list_clipper::ListClipperData;
use crate::types::Direction;
use crate::window::ShrinkWidthItem;

use crate::draw::list_shared_data::DrawListSharedData;
use crate::font::font_atlas::FontAtlas;
use crate::font::font::Font;
use crate::group::GroupData;
use crate::input::{Key, InputSource, ModFlags, MouseButton, MouseCursor, NavLayer};
use crate::input::input_event::InputEvent;
use crate::input::io::{Io, PlatformIo};
use crate::item::{pop_item_flag, ItemFlags, LastItemData, NextItemData};

use crate::metrics::MetricsConfig;
use crate::nav::{nav_move_request_cancel, ActivateFlags, NavItemData, NavMoveFlags, ScrollFlags};
use crate::payload::Payload;
use crate::platform::{PlatformImeData, PlatformDisplay};

use crate::popup::PopupData;
use crate::rect::Rect;
use crate::settings::SettingsHandler;
use crate::stack::StackTool;
use crate::style::{Style, StyleMod};
use crate::tab_bar::TabBar;
use crate::table::{Table, TableSettings, TableTempData};

use crate::text_input_state::InputTextState;
use crate::types::{Id32, PtrOrIndex, WindowHandle, INVALID_ID};
use crate::vectors::vector_2d::Vector2D;
use crate::vectors::Vector4D;
use crate::viewport::Viewport;
use crate::window::next_window::NextWindowData;
use crate::window::settings::WindowSettings;
use crate::window::{Window, WindowStackData};

#[derive()]
pub struct Context {
    pub active_id: Id32,
    pub active_id_allow_overlap: bool,
    pub active_id_click_offset: Vector2D,
    pub active_id_has_been_edited_this_frame: bool,
    pub active_id_has_been_pressed_before: bool,
    pub active_id_hass_been_edited_before: bool,
    pub active_id_is_alive: Id32,
    pub active_id_is_just_activated: bool,
    pub active_id_mouse_button: MouseButton,
    pub active_id_no_clear_on_focus_loss: bool,
    pub active_id_previous_frame: Id32,
    pub active_id_previous_frame_has_been_edited_before: bool,
    pub active_id_previous_frame_is_alive: bool,
    pub active_id_previous_frame_window_id: Id32,
    pub active_id_source: InputSource,
    pub active_id_timer: f32,
    pub active_id_using_key_input_mask: Vec<Key>,
    pub active_id_using_mouse_wheel: bool,
    pub active_id_using_nav_dir_mask: u32,
    pub active_id_using_nav_input_mask: u32,
    pub active_id_window_id: Id32,
    pub begin_menu_count: i32,
    pub begin_popup_stack: Vec<PopupData>,
    pub clipboard_handler_data: Vec<u8>,
    pub clipper_temp_data: Vec<ListClipperData>,
    pub clipper_temp_data_stacked: i32,
    pub color_edit_last_color: u32,
    pub color_edit_last_hue: f32,
    pub color_edit_last_sat: f32,
    pub color_edit_options: HashSet<ColorEditFlags>,
    pub color_picker_ref: Vector4D,
    pub color_stack: Vec<StackedColorModifier>,
    pub combo_preview_data: ComboPreviewData,
    pub config_flags_curr_frame: HashSet<ConfigFlags>,
    pub config_flags_last_frame: HashSet<ConfigFlags>,
    pub current_dpi_scale: f32,
    pub current_item_flags: HashSet<ItemFlags>,
    pub current_tab_bar: Id32,
    pub current_tab_bar_stack: Vec<PtrOrIndex>,
    pub current_table: Id32,
    pub current_viewport_id: Id32,
    pub current_window_id: Id32,
    pub current_window_stack: Vec<WindowStackData>,
    pub debug_hook_id_info: Id32,
    pub debug_item_picker_active: bool,
    pub debug_item_picker_break_id: Id32,
    pub debug_metrics_config: MetricsConfig,
    pub debug_stack_tool: StackTool,
    pub dim_bg_ratio: f32,
    pub disabled_alpha_backup: f32,
    pub disabled_stack_size: usize,
    pub dock_context: DockContext,
    pub dock_nodes: HashMap<Id32, DockNode>,
    pub drag_current_accum: f32,
    pub drag_current_accum_dirty: bool,
    pub drag_drop_accept_flags: HashSet<DragDropFlags>,
    pub drag_drop_accept_frame_count: usize,
    pub drag_drop_accept_id_curr: Id32,
    pub drag_drop_accept_id_curr_rect_surface: f32,
    pub drag_drop_accept_id_prev: Id32,
    pub drag_drop_active: bool,
    pub drag_drop_hold_just_pressed_id: Id32,
    pub drag_drop_mouse_button: MouseButton,
    pub drag_drop_payload: Payload,
    pub drag_drop_payload_buf_heap: Vec<u8>,
    pub drag_drop_payload_buf_local: [u8; 16],
    pub drag_drop_source_flags: HashSet<DragDropFlags>,
    pub drag_drop_source_frame_count: usize,
    pub drag_drop_target_id: Id32,
    pub drag_drop_target_rect: Rect,
    pub drag_drop_within_source: bool,
    pub drag_drop_within_target: bool,
    pub drag_speed_default_ratio: f32,
    pub draw_channels_temp_merge_buffer: Vec<DrawChannel>,
    pub draw_commands: Vec<DrawCommand>,
    pub draw_list_shared_data: DrawListSharedData,
    pub draw_lists: HashMap<Id32, DrawList>,
    pub fallback_monitor: PlatformDisplay,
    pub focus_scope_stack: Vec<Id32>,
    pub font: Font,
    pub font_atlas_owned_by_context: bool,
    pub font_base_size: f32,
    pub font_size: f32,
    pub font_stack: Vec<Font>,
    pub frame_count: usize,
    pub frame_count_ended: usize,
    pub frame_count_platform_ended: usize,
    pub frame_count_rendered: usize,
    pub framerate_sec_per_frame: [f32; 128],
    pub framerate_sec_per_frame_accum: f32,
    pub framerate_sec_per_frame_count: i32,
    pub framerate_sec_per_frame_idx: i32,
    pub gc_compact_all: bool,
    pub group_stack: Vec<GroupData>,
    pub hook_id_next: Id32,
    pub hooks: Vec<ContextHook>,
    pub hovered_dock_node_id: Id32,
    pub hovered_id: Id32,
    pub hovered_id_allow_overlap: bool,
    pub hovered_id_disabled: bool,
    pub hovered_id_not_active_timer: f32,
    pub hovered_id_previous_frame: Id32,
    pub hovered_id_previous_frame_using_mouse_wheel: bool,
    pub hovered_id_timer: f32,
    pub hovered_id_using_mouse_wheel: bool,
    pub hovered_window_id: Id32,
    pub hovered_window_under_moving_window_id: Id32,
    pub initialized: bool,
    pub input_events_queue: Vec<InputEvent>,
    pub input_events_trail: Vec<InputEvent>,
    pub input_text_password_font: Font,
    pub input_text_state: InputTextState,
    pub io: Io,
    pub item_flags_stack: Vec<ItemFlags>,
    pub last_active_id: Id32,
    pub last_active_id_timer: f32,
    pub last_item_data: LastItemData,
    pub log_file: String,
    pub menus_id_submitted_this_frame: Vec<Id32>,
    pub mouse_cursor: MouseCursor,
    pub mouse_last_hovered_viewport_id: Id32,
    pub mouse_last_valid_pos: Vector2D,
    pub mouse_viewport_id: Id32,
    pub moving_window_id: Id32,
    pub nav_activate_down_id: Id32,
    pub nav_activate_flags: HashSet<ActivateFlags>,
    pub nav_activate_id: Id32,
    pub nav_activate_input_id: Id32,
    pub nav_activate_pressed_id: Id32,
    pub nav_any_request: bool,
    pub nav_disable_highlight: bool,
    pub nav_disable_mouse_hover: bool,
    pub nav_focus_scope_id: Id32,
    pub nav_id: Id32,
    pub nav_id_is_alive: bool,
    pub nav_init_request: bool,
    pub nav_init_request_from_move: bool,
    pub nav_init_result_id: Id32,
    pub nav_init_result_rect_rel: Rect,
    pub nav_input_source: InputSource,
    pub nav_just_moved_to_focus_scope_id: Id32,
    pub nav_just_moved_to_id: Id32,
    pub nav_just_moved_to_key_mods: ModFlags,
    pub nav_layer: NavLayer,
    pub nav_mouse_pos_dirty: bool,
    pub nav_move_clip_dir: Direction,
    pub nav_move_dir: Direction,
    pub nav_move_dir_for_debug: Direction,
    pub nav_move_flags: HashSet<NavMoveFlags>,
    pub nav_move_forward_to_next_frame: bool,
    pub nav_move_key_mods: HashSet<ModFlags>,
    pub nav_move_result_local: NavItemData,
    pub nav_move_result_local_visible: NavItemData,
    pub nav_move_result_other: NavItemData,
    pub nav_move_scoring_items: bool,
    pub nav_move_scroll_flags: HashSet<ScrollFlags>,
    pub nav_move_submitted: bool,
    pub nav_next_activate_flags: HashSet<ActivateFlags>,
    pub nav_next_activate_id: Id32,
    pub nav_scoring_debug_count: i32,
    pub nav_scoring_no_clip_rect: Rect,
    pub nav_scoring_rect: Rect,
    pub nav_tabbing_counter: i32,
    pub nav_tabbing_dir: i32,
    pub nav_tabbing_result_first: NavItemData,
    pub nav_window_id: Id32,
    pub nav_windowing_highlight_alpha: f32,
    pub nav_windowing_list_window_id: Id32,
    pub nav_windowing_target_anim: Id32,
    pub nav_windowing_target_id: Id32,
    pub nav_windowing_timer: f32,
    pub nav_windowing_toggle_layer: bool,
    pub next_item_data: NextItemData,
    pub next_window_data: NextWindowData,
    pub open_popup_stack: Vec<PopupData>,
    pub platform_ime_data: PlatformImeData,
    pub platform_ime_data_prev: PlatformImeData,
    pub platform_ime_viewport: Id32,
    pub platform_io: PlatformIo,
    pub platform_last_focused_viewport_id: Id32,
    pub platform_local_decimal_point: char,
    pub scrollbar_click_delta_to_grab_center: f32,
    pub settings_dirty_timer: f32,
    pub settings_handlers: Vec<SettingsHandler>,
    pub settings_ini_data: Vec<u8>,
    pub settings_loaded: bool,
    pub settings_tabls: Vec<TableSettings>,
    pub settings_windows: Vec<WindowSettings>,
    pub shrink_width_buffer: Vec<ShrinkWidthItem>,
    pub slider_current_accum: f32,
    pub slider_current_accum_dirty: bool,
    pub slider_grab_click_offset: f32,
    pub style: Style,
    pub style_var_stack: Vec<StyleMod>,
    pub tab_bars: HashMap<Id32, TabBar>,
    pub tables: HashMap<Id32, Table>,
    pub tables_last_time_active: Vec<f32>,
    pub tables_temp_data: Vec<TableTempData>,
    pub tables_temp_data_stacked: i32,
    pub temp_buffer: Vec<u8>,
    pub temp_input_id: Id32,
    pub test_engine: Vec<u8>,
    pub test_engine_hook_items: bool,
    pub time: f32,
    pub tooltip_override_count: i16,
    pub tooltip_slow_delay: f32,
    pub viewport_front_most_stamp_count: i32,
    pub viewports: Vec<Viewport>,
    pub want_capture_keyboard_next_frame: i32,
    pub want_capture_mouse_next_frame: i32,
    pub want_input_next_frame: i32,
    pub wheeling_window_id: Id32,
    pub wheeling_window_ref_mouse_pos: Vector2D,
    pub wheeling_window_timer: f32,
    pub windows: HashMap<Id32, Window>,
    pub windows_active_count: i32,
    pub windows_focus_order: Vec<Id32>,
    pub windows_hover_padding: Vector2D,
    pub windows_temp_sort_buffer: Vec<Id32>,
    pub within_end_child: bool,
    pub within_frame_scope: bool,
    pub within_frame_scope_with_implicit_window: bool,
}

impl Context {
    // ImGuiContext(ImFontAtlas* shared_font_atlas)
    pub fn new(shared_font_atlas: &mut FontAtlas) -> Self {
        Self {
            active_id: 0,
            active_id_allow_overlap: false,
            active_id_click_offset: Default::default(),
            active_id_has_been_edited_this_frame: false,
            active_id_has_been_pressed_before: false,
            active_id_hass_been_edited_before: false,
            active_id_is_alive: 0,
            active_id_is_just_activated: false,
            active_id_mouse_button: MouseButton::None,
            active_id_no_clear_on_focus_loss: false,
            active_id_previous_frame: 0,
            active_id_previous_frame_has_been_edited_before: false,
            active_id_previous_frame_is_alive: false,
            active_id_previous_frame_window_id: u32::MAX,
            active_id_source: InputSource::None,
            active_id_timer: 0.0,
            active_id_using_key_input_mask: vec![],
            active_id_using_mouse_wheel: false,
            active_id_using_nav_dir_mask: 0x00,
            active_id_using_nav_input_mask: 0x00,
            active_id_window_id: u32::MAX,
            begin_menu_count: 0,
            begin_popup_stack: vec![],
            clipboard_handler_data: vec![],
            clipper_temp_data: vec![],
            clipper_temp_data_stacked: 0,
            color_edit_last_color: 0,
            color_edit_last_hue: 0.0,
            color_edit_last_sat: 0.0,
            color_edit_options: COLOR_EDIT_FLAGS_DFLT_OPTS.clone(),
            color_picker_ref: Default::default(),
            color_stack: vec![],
            combo_preview_data: ComboPreviewData::default(),
            config_flags_curr_frame: HashSet::new(),
            config_flags_last_frame: HashSet::new(),
            current_dpi_scale: 0.0,
            current_item_flags: HashSet::new(),
            current_tab_bar: Id32::MAX,
            current_tab_bar_stack: vec![],
            current_table: Id32::MAX,
            current_viewport_id: INVALID_ID,
            current_window_id: INVALID_ID,
            current_window_stack: vec![],
            debug_hook_id_info: 0,
            debug_item_picker_active: false,
            debug_item_picker_break_id: 0,
            debug_metrics_config: MetricsConfig::default(),
            debug_stack_tool: StackTool::default(),
            dim_bg_ratio: 0.0,
            disabled_alpha_backup: 0.0,
            disabled_stack_size: 0,
            dock_context: DockContext::default(),
            dock_nodes: Default::default(),
            drag_current_accum: 0.0,
            drag_current_accum_dirty: false,
            drag_drop_accept_flags: HashSet::new(),
            drag_drop_accept_frame_count: -1,
            drag_drop_accept_id_curr: 0,
            drag_drop_accept_id_curr_rect_surface: 0.0,
            drag_drop_accept_id_prev: 0,
            drag_drop_active: false,
            drag_drop_hold_just_pressed_id: 0,
            drag_drop_mouse_button: MouseButton::None,
            drag_drop_payload: Default::default(),
            drag_drop_payload_buf_heap: vec![],
            drag_drop_payload_buf_local: [0; 16],
            drag_drop_source_flags: HashSet::new(),
            drag_drop_source_frame_count: -1,
            drag_drop_target_id: 0,
            drag_drop_target_rect: Rect::default(),
            drag_drop_within_source: false,
            drag_drop_within_target: false,
            drag_speed_default_ratio: 1.0 / 100.0,
            draw_channels_temp_merge_buffer: vec![],
            draw_commands: vec![],
            draw_list_shared_data: DrawListSharedData::default(),
            draw_lists: Default::default(),
            fallback_monitor: PlatformDisplay::default(),
            focus_scope_stack: vec![],
            font: Default::default(),
            font_atlas_owned_by_context: true,
            font_base_size: 0.0,
            font_size: 0.0,
            font_stack: vec![],
            frame_count: 0,
            frame_count_ended: -1,
            frame_count_platform_ended: -1,
            frame_count_rendered: -1,
            framerate_sec_per_frame: [0.0; 128],
            framerate_sec_per_frame_accum: 0.0,
            framerate_sec_per_frame_count: 0,
            framerate_sec_per_frame_idx: 0,
            gc_compact_all: false,
            group_stack: vec![],
            hook_id_next: 0,
            hooks: vec![],
            hovered_dock_node_id: INVALID_ID,
            hovered_id: 0,
            hovered_id_allow_overlap: false,
            hovered_id_disabled: false,
            hovered_id_not_active_timer: 0.0,
            hovered_id_previous_frame: 0,
            hovered_id_previous_frame_using_mouse_wheel: false,
            hovered_id_timer: 0.0,
            hovered_id_using_mouse_wheel: false,
            hovered_window_id: INVALID_ID,
            hovered_window_under_moving_window_id: INVALID_ID,
            initialized: false,
            input_events_queue: vec![],
            input_events_trail: vec![],
            input_text_password_font: Default::default(),
            input_text_state: InputTextState::default(),
            io: Io::new(),
            item_flags_stack: vec![],
            last_active_id: 0,
            last_active_id_timer: 0.0,
            last_item_data: LastItemData::default(),
            log_file: String::from(""),
            menus_id_submitted_this_frame: vec![],
            mouse_cursor: MouseCursor::Arrow,
            mouse_last_hovered_viewport_id: INVALID_ID,
            mouse_last_valid_pos: Default::default(),
            mouse_viewport_id: INVALID_ID,
            moving_window_id: INVALID_ID,
            nav_activate_down_id: 0,
            nav_activate_flags: HashSet::new(),
            nav_activate_id: 0,
            nav_activate_input_id: 0,
            nav_activate_pressed_id: 0,
            nav_any_request: false,
            nav_disable_highlight: false,
            nav_disable_mouse_hover: false,
            nav_focus_scope_id: 0,
            nav_id: 0,
            nav_id_is_alive: false,
            nav_init_request: false,
            nav_init_request_from_move: false,
            nav_init_result_id: 0,
            nav_init_result_rect_rel: Rect::default(),
            nav_input_source: InputSource::None,
            nav_just_moved_to_focus_scope_id: 0,
            nav_just_moved_to_id: 0,
            nav_just_moved_to_key_mods: ModFlags::None,
            nav_layer: NavLayer::Main,
            nav_mouse_pos_dirty: false,
            nav_move_clip_dir: Direction::None,
            nav_move_dir: Direction::None,
            nav_move_dir_for_debug: Direction::None,
            nav_move_flags: HashSet::new(),
            nav_move_forward_to_next_frame: false,
            nav_move_key_mods: HashSet::new(),
            nav_move_result_local: NavItemData::default(),
            nav_move_result_local_visible: NavItemData::default(),
            nav_move_result_other: NavItemData::default(),
            nav_move_scoring_items: false,
            nav_move_scroll_flags: HashSet::new(),
            nav_move_submitted: false,
            nav_next_activate_flags: HashSet::new(),
            nav_next_activate_id: 0,
            nav_scoring_debug_count: 0,
            nav_scoring_no_clip_rect: Rect::default(),
            nav_scoring_rect: Rect::default(),
            nav_tabbing_counter: 0,
            nav_tabbing_dir: 0,
            nav_tabbing_result_first: NavItemData::default(),
            nav_window_id: u32::MAX,
            nav_windowing_highlight_alpha: 0.0,
            nav_windowing_list_window_id: u32::MAX,
            nav_windowing_target_anim: u32::MAX,
            nav_windowing_target_id: u32::MAX,
            nav_windowing_timer: 0.0,
            nav_windowing_toggle_layer: false,
            next_item_data: NextItemData::default(),
            next_window_data: NextWindowData::default(),
            open_popup_stack: vec![],
            platform_ime_data: PlatformImeData::new(Vector2D::new(0.0, 0.0)),
            platform_ime_data_prev: PlatformImeData::new(Vector2D::new(-1.0, -1.0)),
            platform_ime_viewport: 0,
            platform_io: PlatformIo::new(),
            platform_last_focused_viewport_id: 0,
            platform_local_decimal_point: '.',
            scrollbar_click_delta_to_grab_center: 0.0,
            settings_dirty_timer: 0.0,
            settings_handlers: vec![],
            settings_ini_data: vec![],
            settings_loaded: false,
            settings_tabls: vec![],
            settings_windows: vec![],
            shrink_width_buffer: vec![],
            slider_current_accum: 0.0,
            slider_current_accum_dirty: false,
            slider_grab_click_offset: 0.0,
            style: Style::new(),
            style_var_stack: vec![],
            tab_bars: HashMap::new(),
            tables: HashMap::new(),
            tables_last_time_active: vec![],
            tables_temp_data: vec![],
            tables_temp_data_stacked: 0,
            temp_buffer: vec![],
            temp_input_id: 0,
            test_engine: vec![],
            test_engine_hook_items: false,
            time: 0.0,
            tooltip_override_count: 0,
            tooltip_slow_delay: 0.50,
            viewport_front_most_stamp_count: 0,
            viewports: vec![],
            want_capture_keyboard_next_frame: -1,
            want_capture_mouse_next_frame: -1,
            want_input_next_frame: 0,
            wheeling_window_id: INVALID_ID,
            wheeling_window_ref_mouse_pos: Default::default(),
            wheeling_window_timer: 0.0,
            windows: HashMap::new(),
            windows_active_count: 0,
            windows_focus_order: vec![],
            windows_hover_padding: Default::default(),
            windows_temp_sort_buffer: vec![],
            within_end_child: false,
            within_frame_scope: false,
            within_frame_scope_with_implicit_window: false,
        }
    }

    /// Panics if a window in the window hash set for the global context does not contain a window matching the id set in the current_window_id field
    pub fn current_window_mut(&mut self) -> &mut Window {
        self.windows.get_mut(&self.current_window_id).expect(
            format!(
                "failed to get current window (id={})",
                self.current_window_id
            ).as_str(),
        )
    }

    pub fn current_table_mut(&mut self) -> &mut Table {
        self.tables.get_mut(&self.current_table).unwrap()
    }

    pub fn nav_window_mut(&mut self) -> &mut Window {
        self.windows.get_mut(&self.nav_window_id).unwrap()
    }

    pub fn hovered_window_under_moving_window_mut(&mut self) -> &mut Window {
        self.window_mut(self.hovered_window_under_moving_window_id)
    }

    pub fn viewport_mut(&mut self, vp_id: Id32) -> Option<&mut Viewport> {
        for vp in self.viewports.iter_mut() {
            if vp.id == vp_id {
                return Some(vp);
            }
        }

        return None;
    }

    pub fn draw_command_mut(&mut self, draw_cmd_id: Id32) -> Option<&mut DrawCommand> {
        for dc in self.draw_commands.iter_mut() {
            if dc.id == draw_cmd_id {
                return Some(dc);
            }
        }
        return None;
    }

    pub fn window_mut(&mut self, win_id: Id32) -> &mut Window {
        self.windows.get_mut(&win_id).expect(format!("window not found in window stack for id={}", win_id).as_str())
    }

    pub fn dock_node_mut(&mut self, dock_node_id: Id32) -> Option<&mut DockNode> {
        self.dock_nodes.get_mut(&dock_node_id)
    }

    pub fn draw_list_mut(&mut self, draw_list_id: Id32) -> &mut DrawList {
        self.draw_lists.get_mut(&draw_list_id).expect(format!("draw list not found in collection for id={}", draw_list_id).as_str())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ContextHookType {
    None,
    NewFramePre,
    NewFramePost,
    EndFramePre,
    EndFramePost,
    RenderPre,
    RenderPost,
    Shutdown,
    PendingRemoval,
}

impl Default for ContextHookType {
    fn default() -> Self {
        Self::None
    }
}

pub type ContextHookCallback = fn(g: &mut Context, hook: &mut ContextHook);

#[derive(Default, Clone)]
pub struct ContextHook {
    // Id32                     HookId;     // A unique id assigned by AddContextHook()
    pub hook_id: Id32,
    // ImGuiContextHookType        Type;
    pub hook_type: ContextHookType,
    // Id32                     Owner;
    pub owner: Id32,
    // ImGuiContextHookCallback    Callback;
    pub callback: Option<ContextHookCallback>,
    // void*                       user_data;
    pub user_data: Vec<u8>,
    // ImGuiContextHook()          { memset(this, 0, sizeof(*this)); }
}

impl Debug for ContextHook {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ContextHook").field("hook_id", &self.hook_id).field("hook_type", &self.hook_type).field("owner", &self.owner).field(
            "callback",
            &format!("is_some: {}", &self.callback.is_some()),
        ).field(
            "user_data",
            &format!(
                "{:x} {:x} {:x} {:x} {:x} {:x} {:x} {:x}",
                &self.user_data[0],
                &self.user_data[1],
                &self.user_data[2],
                &self.user_data[3],
                &self.user_data[4],
                &self.user_data[5],
                &self.user_data[6],
                &self.user_data[7]
            ),
        ).finish()
    }
}

/// Deferred removal, avoiding issue with changing vector while iterating it
pub fn remove_context_hook(g: &mut Context, hook_id: Id32) {
    // ImGuiContext& g = *ctx;
    // IM_ASSERT(hook_id != 0);
    // for (int n = 0; n < g.Hooks.Size; n += 1){
    for n in 0..g.hooks.len() {
        if g.hooks[n].hook_id == hook_id {
            g.hooks[n].hook_type = ContextHookType::PendingRemoval;
        }
    }
}

/// No specific ordering/dependency support, will see as needed
pub fn add_context_hook(g: &mut Context, hook: &ContextHook) -> Id32 {
    // ImGuiContext& g = *ctx;
    // IM_ASSERT(hook->Callback != None && hook->HookId == 0 && hook->Type != ImGuiContextHookType_PendingRemoval_);
    g.hooks.push(hook.clone());
    g.hook_id_next += 1;
    // g.hooks.last().hook_id = g.hook_id_next;
    g.hooks[g.hooks.len() - 1].hook_id = g.hook_id_next;
    return g.hook_id_next;
}

/// Call context hooks (used by e.g. test engine)
/// We assume a small number of hooks so all stored in same array
pub fn call_context_hooks(g: &mut Context, hook_type: ContextHookType) {
    // ImGuiContext& g = *ctx;
    // for (int n = 0; n < g.Hooks.Size; n += 1){
    for n in 0..g.hooks.len() {
        if g.hooks[n].hook_type == hook_type && g.hooks[n].callback.is_some() {
            g.hooks[n].callback.unwrap()(g, &mut g.hooks[n]);
        }
    }
}

pub fn set_active_id_using_nav_and_keys(g: &mut Context) {
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.active_id != 0);
    g.active_id_using_nav_dir_mask = !0;
    g.active_id_using_nav_input_mask = !0;
    g.active_id_using_key_input_mask.SetAllBits();
    nav_move_request_cancel(g);
}

/// BeginDisabled()/EndDisabled()
/// - Those can be nested but it cannot be used to enable an already disabled section (a single BeginDisabled(true) in the stack is enough to keep everything disabled)
/// - Visually this is currently altering alpha, but it is expected that in a future styling system this would work differently.
/// - Feedback welcome at https://github.com/ocornut/imgui/issues/211
/// - BeginDisabled(false) essentially does nothing useful but is provided to facilitate use of boolean expressions. If you can avoid calling BeginDisabled(False)/EndDisabled() best to avoid it.
/// - Optimized shortcuts instead of PushStyleVar() + push_item_flag()
pub fn begin_disabled(g: &mut Context, disabled: bool) {
    // ImGuiContext& g = *GImGui;

    let was_disabled = g.current_item_flags.contains(&ItemFlags::Disabled);
    if !was_disabled && disabled {
        g.disabled_alpha_backup = g.style.alpha;
        g.style.alpha *= g.style.disabled_alpha; // PushStyleVar(ImGuiStyleVar_Alpha, g.style.alpha * g.style.DisabledAlpha);
    }
    if was_disabled || disabled {
        g.current_item_flags.insert(ItemFlags::Disabled);
    }

    // g.item_flags_stack.push_back(g.current_item_flags);
    for f in g.current_item_flags.iter() {
        let push_flag: ItemFlag = f.clone();
        g.item_flags_stack.push(push_flag);
    }
    g.disabled_stack_size += 1;
}

// void ImGui::EndDisabled()
pub fn end_disabled(g: &mut Context) {
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.DisabledStackSize > 0);
    g.disabled_stack_size -= 1;
    let was_disabled = g.current_item_flags.contains(&ItemFlags::Disabled);
    //PopItemFlag();
    pop_item_flag(g);
    g.item_flags_stack.pop_back();
    g.current_item_flags = g.item_flags_stack.back();
    // if (was_disabled && (g.current_item_flags & ItemFlags::Disabled) == 0)
    if was_disabled {
        g.style.alpha = g.disabled_alpha_backup; //PopStyleVar();}
    }
}

// static void set_current_window(Window* window)
pub fn set_current_window(g: &mut Context, window_handle: WindowHandle) {
    // ImGuiContext& g = *GImGui;
    g.current_window_id = window_handle;
    // if window
    let current_window = g.window_mut(window_handle);
    g.current_table_id = if current_window.dc.current_table_idx != -1 {
        g.tables.get_by_index(window_handle.dc.current_table_idx)
    } else {
        INVALID_ID
    };
    g.font_size = current_window.calc_font_size();
    g.draw_list_shared_data.font_size = g.font_size;
}
