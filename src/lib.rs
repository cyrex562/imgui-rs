#![allow(non_snake_case)]
extern crate core;
extern crate freetype;

use crate::context::ImguiContext;
use crate::context_hook::IM_GUI_CONTEXT_HOOK_TYPE_SHUTDOWN;
use crate::context_ops::CallContextHooks;
use crate::dock_context_ops::{init_dock_context, shutdown_dock_context};
use crate::file_ops::ImFileClose;
use crate::hash_ops::hash_string;
use crate::imgui::GImGui;
use crate::settings_handler::SettingsHandler;
use crate::settings_ops::{
    AddSettingsHandler, SaveIniSettingsToDisk, WindowSettingsHandler_ApplyAll,
    WindowSettingsHandler_ClearAll, WindowSettingsHandler_ReadLine, WindowSettingsHandler_WriteAll,
};
use crate::tables::TableSettingsAddSettingsHandler;
use crate::viewport::ImguiViewport;
use crate::viewport_flags::ImGuiViewportFlags_OwnedByApp;
use crate::viewport_ops::DestroyPlatformWindows;
use std::collections::HashSet;
use std::io::stdout;
use std::ptr::null_mut;

mod activate_flags;
mod axis;
mod backend_flags;
mod child_ops;
mod clipboard_ops;
mod color;
mod color_edit_flags;
mod color_mod;
mod color_ops;
mod combo_flags;
mod combo_preview_data;
mod condition;
mod config;
mod config_flags;
mod constants;
mod content_ops;
mod context;
mod context_hook;
mod context_ops;
mod cursor_ops;
mod data_authority;
mod data_type;
mod data_type_info;
mod data_type_ops;
mod data_type_temp_storage;
mod debug_log_flags;
mod debug_ops;
mod direction;
mod draw;
mod draw_channel;
mod draw_cmd;
mod draw_cmd_header;
mod draw_data;
mod draw_data_ops;
mod draw_flags;
mod draw_list;
mod draw_list_flags;
mod draw_list_ops;
mod draw_list_shared_data;
mod draw_list_splitter;
mod draw_vert;
mod error_ops;
mod file_ops;
mod focused_flags;
mod font;
mod font_atlas;
mod font_atlas_custom_rect;
mod font_atlas_default_tex_data;
mod font_atlas_flags;
mod font_atlas_ops;
mod font_build_dst_data;
mod font_build_src_data;
mod font_builder_io;
mod font_config;
mod font_glyph;
mod font_glyph_ranges_builder;
mod font_ops;
mod frame_ops;
mod g_style_var_info;
mod garbage_collection;
mod geometry_ops;
mod group_data;
mod group_ops;
mod hash_ops;
mod hovered_flags;
mod id_ops;
mod image_ops;
mod imgui;
mod imvec1;
mod input_event;
mod input_event_type;
mod input_flags;
mod input_num_ops;
mod input_ops;
mod input_source;
mod input_text;
mod input_text_callback_data;
mod input_text_flags;
mod input_text_state;
mod io;
mod io_ops;
mod item_flags;
mod item_ops;
mod item_status_flags;
mod key;
mod key_data;
mod keyboard_ops;
mod last_item_data;
mod layout_ops;
mod layout_type;
mod list_clipper;
mod list_clipper_data;
mod list_clipper_ops;
mod list_clipper_range;
mod log_type;
mod logging_ops;
mod math_ops;
mod memory_management;
mod menu_columns;
mod merge_group;
mod metrics_config;
mod mod_flags;
mod mouse_button;
mod mouse_cursor;
mod mouse_ops;
mod nav_highlight_flags;
mod nav_item_data;
mod nav_layer;
mod nav_move_flags;
mod nav_ops;
mod next_item_data;
mod next_item_data_flags;
mod next_window_data;
mod next_window_data_flags;
mod old_column_data;
mod old_column_flags;
mod old_columns;
mod once_upon_a_frame;
mod payload;
mod platform_ime_data;
mod platform_io;
mod platform_monitor;
mod platform_support;
mod plot_array_getter_data;
mod plot_type;
mod pool;
mod popup_data;
mod popup_flags;
mod popup_ops;
mod popup_position_policy;
mod progress_bar;
mod ptr_or_index;
mod radio_button;
mod rect;
mod render_ops;
mod resize_border_def;
mod resize_grip_def;
mod scroll_flags;
mod scrolling_ops;
mod selectable_flags;
mod separator;
mod separator_flags;
mod settings_handler;
mod settings_ops;
mod shade_verts_ops;
mod shrink_width_item;
mod size_callback_data;
mod slider_flags;
mod slider_ops;
mod sort_direction;
mod span;
mod span_allocator;
mod splitter;
mod stack_level_info;
mod stack_sizes;
mod stack_tool;
mod state_storage_ops;
mod stb;
mod storage;
mod string_ops;
mod style;
mod style_mod;
mod style_ops;
mod style_var;
mod style_var_info;
mod style_var_ops;
mod tab_bar;
mod tab_bar_flags;
mod tab_bar_section;
mod tab_item;
mod tab_item_flags;
mod table;
mod table_bg_target;
mod table_cell_data;
mod table_column;
mod table_column_flags;
mod table_column_settings;
mod table_column_sort_specs;
mod table_flags;
mod table_instance_data;
mod table_ops;
mod table_row_flags;
mod table_settings;
mod table_sort_specs;
mod table_temp_data;
mod tables;
mod text_filter;
mod text_flags;
mod text_ops;
mod tooltip_flags;
mod tooltip_ops;
mod tree_node_flags;
mod type_defs;
mod utils;
mod vec2;
mod vec4;
mod viewport;
mod viewport_flags;
mod viewport_ops;
mod widget_ops;
mod a_widgets;
mod win_dock_style;
mod window;
mod widgets;
mod docking;
mod drag_drop;
mod core;

// c_void Initialize()
pub fn Initialize(g: &mut ImguiContext) {
    // IM_ASSERT(!g.Initialized && !g.SettingsLoaded);
    // Add .ini handle for ImGuiWindow type
    let mut ini_handler = SettingsHandler::new();
    ini_handler.TypeName = "Window";
    ini_handler.TypeHash = ImHashStr2("Window");
    ini_handler.ClearAllFn = WindowSettingsHandler_ClearAll;
    ini_handler.ReadOpenFn = WindowSettingsHandler_ReadOpen;
    ini_handler.ReadLineFn = WindowSettingsHandler_ReadLine;
    ini_handler.ApplyAllFn = WindowSettingsHandler_ApplyAll;
    ini_handler.WriteAllFn = WindowSettingsHandler_WriteAll;
    // AddSettingsHandler(g, &ini_handler);
    g.add_settings_handler(&ini_handler);

    // Add .ini handle for ImGuiTable type
    TableSettingsAddSettingsHandler(g);

    // Create default viewport
    let mut viewport: *mut ImguiViewport = IM_NEW(ImGuiViewportP)();
    viewport.ID = IMGUI_VIEWPORT_DEFAULT_ID;
    viewport.Idx = 0;
    viewport.PlatformWindowCreated = true;
    viewport.Flags = ImGuiViewportFlags_OwnedByApp;
    g.Viewports.push(viewport);
    g.TempBuffer.resize(1024 * 3 + 1, 0);
    g.PlatformIO.Viewports.push(g.Viewports[0]);

    // #ifdef IMGUI_HAS_DOCK
    // Initialize Docking
    init_dock_context(g);
    // #endif

    g.Initialized = true;
}

// This function is merely here to free heap allocations.
// c_void Shutdown()
pub fn Shutdown(g: &mut ImguiContext) {
    // The fonts atlas can be used prior to calling NewFrame(), so we clear it even if g.Initialized is FALSE (which would happen if we never called NewFrame)
    if g.IO.Fonts.is_some() && g.FontAtlasOwnedByContext {
        g.IO.Fonts.Locked = false;
        // IM_DELETE(g.IO.Fonts);
    }
    g.IO.Fonts = None;

    // Cleanup of other data are conditional on actually having initialized Dear ImGui.
    if !g.Initialized {
        return;
    }

    // Save settings (unless we haven't attempted to load them: CreateContext/DestroyContext without a call to NewFrame shouldn't save an empty file)
    if g.SettingsLoaded && g.IO.IniFilename != None {
        SaveIniSettingsToDisk(g.IO.IniFilename);
    }

    // Destroy platform windows
    DestroyPlatformWindows();

    // Shutdown extensions
    shutdown_dock_context(g);

    // CallContextHooks(g, IM_GUI_CONTEXT_HOOK_TYPE_SHUTDOWN);
    g.call_context_hooks(IM_GUI_CONTEXT_HOOK_TYPE_SHUTDOWN);

    // Clear everything else
    g.Windows.clear_delete();
    g.WindowsFocusOrder.clear();
    g.WindowsTempSortBuffer.clear();
    g.CurrentWindow = None;
    g.CurrentWindowStack.clear();
    g.WindowsById.Clear();
    g.NavWindow = None;
    g.HoveredWindow = null_Mut();
    g.HoveredWindowUnderMovingWindow = None;
    g.ActiveIdWindow = None;
    g.ActiveIdPreviousFrameWindow = None;
    g.MovingWindow = None;
    g.ColorStack.clear();
    g.styleVarStack.clear();
    g.FontStack.clear();
    g.OpenPopupStack.clear();
    g.BeginPopupStack.clear();

    g.CurrentViewport = None;
    g.MouseViewport = None;
    g.MouseLastHoveredViewport = None;
    g.Viewports.clear_delete();

    g.TabBars.Clear();
    g.CurrentTabBarStack.clear();
    g.ShrinkWidthBuffer.clear();

    g.ClipperTempData.clear_destruct();

    g.Tables.Clear();
    g.TablesTempData.clear_destruct();
    g.DrawChannelsTempMergeBuffer.clear();

    g.ClipboardHandlerData.clear();
    g.MenusIdSubmittedThisFrame.clear();
    g.InputTextState.ClearFreeMemory();

    g.SettingsWindows.clear();
    g.SettingsHandlers.clear();

    if g.LogFile {
        // #ifndef IMGUI_DISABLE_TTY_FUNCTIONS
        if g.LogFile != libc::stdout {
            // #endif
            ImFileClose(g.LogFile);
        }
        g.LogFile = None;
    }
    g.LogBuffer.clear();
    g.DebugLogBuf.clear();

    g.Initialized = false;
}
