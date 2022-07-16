extern crate core;

use std::collections::HashSet;
use std::io::stdout;
use crate::context::{call_context_hooks, Context, ContextHookType};
use crate::dock_context::dock_context_shutdown;
use crate::hash::hash_string;
use crate::settings::SettingsHandler;
use crate::types::INVALID_ID;
use crate::viewport::{Viewport, ViewportFlags};

pub mod config;
pub mod imgui;
mod style;
mod io;
mod input_event;
mod internal_h;
mod geometry;
mod vectors;
mod hash;
mod file;
mod text;
mod color;
mod math;
mod kv_store;
mod sort;
mod context;
mod log;
mod draw_list;
mod window;
mod column;
mod rect;
mod dock;
mod tab_bar;
mod input;
mod item;
mod group;
mod popup;
mod nav;
mod clipper;
mod table;
mod pool;
mod stb_textedit_h;
mod string;
mod text_input_state;
mod stb_text_edit_state;
mod globals;
mod text_filter;
mod text_range;
mod text_buffer;
mod list_clipper;
mod render;
mod clipboard;
mod defines;
mod gc;
mod id;
mod types;
mod payload;
mod combo;
mod helpers;
mod slider;
mod select;
mod button;
mod tree_node;
mod separator;
mod tooltip;
mod plot;
mod data_type;
mod dock_node;
mod viewport;
mod settings;
mod metrics;
mod stack;
mod contexty;
mod tab_item;
mod font;
mod axis;
mod data_authority;
mod utils;
mod font_atlas;
mod font_glyph;
mod draw_cmd;
mod draw_vert;
mod draw_channel;
mod drag_drop;
mod table_column;
mod table_row;
mod condition;
mod direction;
mod selectable;
mod input_text;
mod texture;
mod platform;
mod menu;
mod layout;
mod dock_context;
mod mouse;
mod draw;
mod draw_list_shared_data;
mod draw_data_builder;
mod draw_list_splitter;
mod draw_data;
mod frame;
mod draw_defines;
mod size_callback_data;
mod keyboard;
mod child;
mod resize;
mod border;

/// void ImGui::Initialize()
pub fn initialize(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(!g.Initialized && !g.SettingsLoaded);

    // Add .ini handle for ImGuiWindow type
    {
        // ImGuiSettingsHandler ini_handler;
        let mut ini_handler = SettingsHandler::default();
        ini_handler.type_name = String::from("window");
        ini_handler.type_hash = hash_string(&ini_handler.type_name.into_bytes(), 0);
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
pub fn shutdown(g: &mut Context)
{
    // The fonts atlas can be used prior to calling NewFrame(), so we clear it even if g.Initialized is FALSE (which would happen if we never called NewFrame)
    // ImGuiContext& g = *GImGui;
    if (g.io.fonts.is_empty() == false && g.font_atlas_owned_by_context)
    {
        g.io.fonts.locked = false;
        // IM_DELETE(g.io.fonts);
        g.io.fonts.clear();
    }
    // g.io.fonts = NULL;

    // Cleanup of other data are conditional on actually having initialized Dear ImGui.
    if (!g.initialized) {
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

    if g.log_file
    {

        if g.log_file != stdout {
            ImFileClose(g.log_file);
        }
        g.log_file.clear();
    }
    // g.LogBuffer.clear();
    // g.DebugLogBuf.clear();

    g.initialized = false;
}
