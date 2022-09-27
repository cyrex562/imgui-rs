extern crate freetype;
extern crate core;

use std::collections::HashSet;
use std::io::stdout;


mod imgui_cpp;
mod style;
mod io;
mod storage;
mod text_filter;
mod imgui_h;
mod type_defs;
mod text_buffer;
mod list_clipping;
mod context;
mod imgui_internal_h;
mod window;
mod platform_io;
mod vec2;
mod window_class;
mod viewport;
mod drawlist;
mod vec4;
mod rect;
mod win_dock_style;
mod old_columns;
mod dock_node;
mod draw_data;
mod input_event;
mod font;
mod font_glyph;
mod font_atlas;
mod font_config;
mod draw_list_shared_data;
mod window_stack_data;
mod input_source;
mod next_item_data;
mod last_item_data;
mod next_window_data;
mod color_mod;
mod style_mod;
mod group_data;
mod popup_data;
mod platform_monitor;
mod nav_item_data;
mod payload;
mod list_clipper_data;
mod table;
mod table_temp_data;
mod pool;
mod draw_channel;
mod tab_bar;
mod tab_item;
mod nav_layer;
mod tab_bar_flags;
mod ptr_or_index;
mod shrink_width_item;
mod input_text_state;
mod stb_textedit;
mod stb_text_edit_state;
mod stb_undo_state;
mod stb_undo_record;
mod combo_preview_data;
mod platform_ime_data;
mod dock_context;
mod dock_request_type;
mod dock_request;
mod direction;
mod dock_node_settings;
mod axis;
mod settings_handler;
mod window_settings;
mod chunk_stream;
mod table_settings;
mod table_flags;
mod table_column_settings;
mod sort_direction;
mod context_hook;
mod log_type;
mod color;
mod condition;
mod data_type;
mod key;
mod mouse_button;
mod mouse_cursor;
mod style_var;
mod table_bg_target;
mod draw_flags;
mod draw_list_flags;
mod font_atlas_flags;
mod backend_flags;
mod button_flags;
mod color_edit_flags;
mod config_flags;
mod combo_flags;
mod dock_node_flags;
mod drag_drop_flags;
mod focused_flags;
mod hoevered_flags;
mod mod_flags;
mod popup_flags;
mod selectable_flagss;
mod slider_flags;
mod tab_bat_flags;
mod input_text_flags;
mod tab_item_flags;
mod table_column_flags;
mod table_row_flags;
mod tree_node_flags;
mod viewport_flags;
mod window_flags;
mod data_authority;
mod layout_type;
mod activate_flags;
mod debug_log_flags;
mod input_flags;
mod item_flags;
mod item_status_flags;
mod old_column_flags;
mod nav_highlight_flags;
mod nav_move_flags;
mod next_item_data_flags;
mod next_window_data_flags;
mod scroll_flags;
mod separator_flags;
mod text_flags;
mod tooltip_flags;
mod metrics_config;
mod stack_tool;
mod stack_level_info;

/// void ImGui::Initialize()
// pub fn initialize(g: &mut Context) {
//     // let g = GImGui; // ImGuiContext& g = *GImGui;
//     // IM_ASSERT(!g.Initialized && !g.SettingsLoaded);
//
//     // Add .ini handle for Window type
//     {
//         // ImGuiSettingsHandler ini_handler;
//         let mut ini_handler = SettingsHandler::default();
//         ini_handler.type_name = String::from("window");
//         ini_handler.type_hash = hash_string(&ini_handler.type_name, 0);
//         ini_handler.clear_all_fn = WindowSettingsHandler_ClearAll;
//         ini_handler.read_open_fn = WindowSettingsHandler_ReadOpen;
//         ini_handler.read_line_fn = WindowSettingsHandler_ReadLine;
//         ini_handler.apply_all_fn = WindowSettingsHandler_ApplyAll;
//         ini_handler.write_all_fn = WindowSettingsHandler_WriteAll;
//         add_settings_handler(g, &ini_handler);
//     }
//
//     // Add .ini handle for ImGuiTable type
//     table_settings_add_settings_handler();
//
//     // Create default viewport
//     // ImGuiViewportP* viewport = IM_NEW(ImGuiViewportP)();
//     let mut viewport = Viewport::default();
//     viewport.id = IMGUI_VIEWPORT_DEFAULT_ID;
//     viewport.idx = 0;
//     viewport.platform_window_created = true;
//     viewport.flags = HashSet::from([ViewportFlags::OwnedByApp]);
//     g.viewports.push(viewport);
//     g.temp_buffer.resize(1024 * 3 + 1, 0);
//     g.platform_io.viewports.push(&g.viewports[0]);
//
//     // Initialize Docking
//     dock_context_initialize(&g);
//
//     g.initialized = true;
// }

// This function is merely here to free heap allocations.
// void ImGui::Shutdown()
// pub fn shutdown(g: &mut Context) {
//     // The fonts atlas can be used prior to calling NewFrame(), so we clear it even if g.Initialized is FALSE (which would happen if we never called NewFrame)
//     // let g = GImGui; // ImGuiContext& g = *GImGui;
//     if g.io.fonts.is_empty() == false && g.font_atlas_owned_by_context {
//         g.io.fonts.locked = false;
//         // IM_DELETE(g.io.fonts);
//         g.io.fonts.clear();
//     }
//     // g.io.fonts = None;
//
//     // Cleanup of other data are conditional on actually having initialized Dear ImGui.
//     if !g.initialized {
//         return;
//     }
//
//     // Save settings (unless we haven't attempted to load them: CreateContext/DestroyContext without a call to NewFrame shouldn't save an empty file)
//     if g.settings_loaded && g.io.ini_filename.is_empty() == false {
//         save_ini_settings_to_disk(g, g.io.ini_file_name);
//     }
//
//     // Destroy platform windows
//     destroy_platform_windows(g);
//
//     // Shutdown extensions
//     dock_context_shutdown(g);
//
//     call_context_hooks(g, ContextHookType::Shutdown);
//
//     // clear everything else
//     g.windows.clear_delete();
//     g.windows_focus_order.clear();
//     g.windows_temp_sort_buffer.clear();
//     g.current_window_id = INVALID_ID;
//     g.current_window_stack.clear();
//     g.windows_by_id.Clear();
//     g.nav_window_id = INVALID_ID;
//     g.hovered_window_id = INVALID_ID;
//     g.hovered_window_under_moving_window_id = INVALID_ID;
//     g.active_id_window_id = INVALID_ID;
//     g.active_id_previous_frame_window_id = INVALID_ID;
//     g.moving_window_id = INVALID_ID;
//     g.color_stack.clear();
//     g.style_var_stack.clear();
//     g.font_stack.clear();
//     g.open_popup_stack.clear();
//     g.begin_popup_stack.clear();
//
//     g.current_viewport_id = INVALID_ID;
//     g.mouse_viewport_id = INVALID_ID;
//     g.mouse_last_hovered_viewport_id = INVALID_ID;
//     g.viewports.clear_delete();
//
//     g.tab_bars.Clear();
//     g.current_tab_bar_stack.clear();
//     g.shrink_width_buffer.clear();
//
//     g.clipper_temp_data.clear_destruct();
//
//     g.tables.Clear();
//     g.tables_temp_data.clear_destruct();
//     g.draw_channels_temp_merge_buffer.clear();
//
//     g.clipboard_handler_data.clear();
//     g.menus_id_submitted_this_frame.clear();
//     g.input_text_state.clear_free_memory();
//
//     g.settings_windows.clear();
//     g.settings_handlers.clear();
//
//     // if g.log_file
//     // {
//     //
//     //     if g.log_file != stdout {
//     //         ImFileClose(g.log_file);
//     //     }
//     //     g.log_file.clear();
//     // }
//     // g.LogBuffer.clear();
//     // g.DebugLogBuf.clear();
//
//     g.initialized = false;
// }
