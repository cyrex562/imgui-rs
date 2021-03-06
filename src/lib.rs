extern crate core;

use crate::context::{call_context_hooks, Context, ContextHookType};
use crate::hash::hash_string;
use crate::settings::SettingsHandler;
use crate::types::INVALID_ID;
use crate::viewport::{Viewport, ViewportFlags};
use dock::context::dock_context_shutdown;
use std::collections::HashSet;
use std::io::stdout;

mod axis;
mod border;
mod button;
mod child;
mod clipboard;
mod clipper;
mod color;
mod column;
mod combo;
pub mod components;
mod condition;
pub mod config;
mod content;
mod context;
mod contexty;
mod cursor;
mod data_authority;
mod data_type;
mod debug;
mod direction;
mod dock;
mod drag_drop;
mod draw;
mod draw_channel;
pub mod error_handling;
mod file;
mod font;
mod frame;
mod gc;
mod geometry;
mod globals;
mod group;
mod hash;
mod helpers;
mod id;
mod ime;
pub mod input;
mod input_event;
mod input_text;
mod internal_h;
mod io;
mod item;
mod kv_store;
mod layout;
mod list_clipper;
mod log;
mod math;
mod menu;
mod metrics;
mod modal;
mod nav;
mod nodes;
mod nodes_h;
pub mod orig_imgui_single_file;
mod payload;
mod platform;
mod plot;
mod pool;
mod popup;
mod rect;
mod render;
mod resize;
mod scrolling;
mod select;
mod selectable;
mod separator;
mod settings;
mod size_callback_data;
mod slider;
mod sort;
mod stack;
mod stb_text_edit_state;
mod stb_textedit_h;
mod string;
mod style;
mod tab_bar;
mod tab_item;
mod table;
mod table_column;
mod table_row;
mod text;
mod text_buffer;
mod text_filter;
mod text_input_state;
mod text_range;
pub mod text_wrap;
mod texture;
mod tooltip;
mod tree_node;
mod types;
mod utils;
mod vectors;
mod viewport;
mod widgets;
mod window;

/// void ImGui::Initialize()
pub fn initialize(g: &mut Context) {
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(!g.Initialized && !g.SettingsLoaded);

    // Add .ini handle for ImGuiWindow type
    {
        // ImGuiSettingsHandler ini_handler;
        let mut ini_handler = SettingsHandler::default();
        ini_handler.type_name = String::from("window");
        ini_handler.type_hash = hash_string(&ini_handler.type_name.into_bytes());
        ini_handler.clear_all_fn = WindowSettingsHandler_ClearAll;
        ini_handler.read_open_fn = WindowSettingsHandler_ReadOpen;
        ini_handler.read_line_fn = WindowSettingsHandler_ReadLine;
        ini_handler.apply_all_fn = WindowSettingsHandler_ApplyAll;
        ini_handler.write_all_fn = WindowSettingsHandler_WriteAll;
        add_settings_handler(&ini_handler);
    }

    // Add .ini handle for ImGuiTable type
    table_settings_add_settings_handler();

    // Create default viewport
    // ImGuiViewportP* viewport = IM_NEW(ImGuiViewportP)();
    let mut viewport = Viewport::default();
    viewport.id = IMGUI_VIEWPORT_DEFAULT_ID;
    viewport.idx = 0;
    viewport.platform_window_created = true;
    viewport.flags = HashSet::from([ViewportFlags::OwnedByApp]);
    g.viewports.push_back(viewport);
    g.temp_buffer.resize(1024 * 3 + 1, 0);
    g.platform_io.viewports.push_back(&g.viewports[0]);

    // Initialize Docking
    dock_context_initialize(&g);

    g.initialized = true;
}

// This function is merely here to free heap allocations.
// void ImGui::Shutdown()
pub fn shutdown(g: &mut Context) {
    // The fonts atlas can be used prior to calling NewFrame(), so we clear it even if g.Initialized is FALSE (which would happen if we never called NewFrame)
    // ImGuiContext& g = *GImGui;
    if g.io.fonts.is_empty() == false && g.font_atlas_owned_by_context {
        g.io.fonts.locked = false;
        // IM_DELETE(g.io.fonts);
        g.io.fonts.clear();
    }
    // g.io.fonts = None;

    // Cleanup of other data are conditional on actually having initialized Dear ImGui.
    if !g.initialized {
        return;
    }

    // Save settings (unless we haven't attempted to load them: CreateContext/DestroyContext without a call to NewFrame shouldn't save an empty file)
    if (g.settings_loaded && g.io.ini_filename.is_empty() == false) {
        save_ini_settings_to_disk(g.io.ini_file_name);
    }

    // Destroy platform windows
    destroy_platform_windows();

    // Shutdown extensions
    dock_context_shutdown(g);

    call_context_hooks(g, ContextHookType::Shutdown);

    // clear everything else
    g.windows.clear_delete();
    g.windows_focus_order.clear();
    g.windows_temp_sort_buffer.clear();
    g.current_window_id = INVALID_ID;
    g.current_window_stack.clear();
    g.windows_by_id.Clear();
    g.nav_window_id = INVALID_ID;
    g.hovered_window_id = INVALID_ID;
    g.hovered_window_under_moving_window_id = INVALID_ID;
    g.active_id_window_id = INVALID_ID;
    g.active_id_previous_frame_window_id = INVALID_ID;
    g.moving_window_id = INVALID_ID;
    g.color_stack.clear();
    g.style_var_stack.clear();
    g.font_stack.clear();
    g.open_popup_stack.clear();
    g.begin_popup_stack.clear();

    g.current_viewport_id = INVALID_ID;
    g.mouse_viewport_id = INVALID_ID;
    g.mouse_last_hovered_viewport_id = INVALID_ID;
    g.viewports.clear_delete();

    g.tab_bars.Clear();
    g.current_tab_bar_stack.clear();
    g.shrink_width_buffer.clear();

    g.clipper_temp_data.clear_destruct();

    g.tables.Clear();
    g.tables_temp_data.clear_destruct();
    g.draw_channels_temp_merge_buffer.clear();

    g.clipboard_handler_data.clear();
    g.menus_id_submitted_this_frame.clear();
    g.input_text_state.ClearFreeMemory();

    g.settings_windows.clear();
    g.settings_handlers.clear();

    // if g.log_file
    // {
    //
    //     if g.log_file != stdout {
    //         ImFileClose(g.log_file);
    //     }
    //     g.log_file.clear();
    // }
    // g.LogBuffer.clear();
    // g.DebugLogBuf.clear();

    g.initialized = false;
}
