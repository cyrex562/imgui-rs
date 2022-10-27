#![allow(non_snake_case)]
extern crate freetype;
extern crate core;

use std::collections::HashSet;
use std::io::stdout;
use std::ptr::null_mut;
use crate::tables::TableSettingsAddSettingsHandler;
use crate::context_hook::ImGuiContextHookType_Shutdown;
use crate::context_ops::CallContextHooks;
use crate::dock_context_ops::{DockContextInitialize, DockContextShutdown};
use crate::file_ops::ImFileClose;
use crate::hash_ops::ImHashStr;
use crate::imgui::GImGui;
use crate::settings_handler::ImGuiSettingsHandler;
use crate::settings_ops::{AddSettingsHandler, SaveIniSettingsToDisk, WindowSettingsHandler_ApplyAll, WindowSettingsHandler_ClearAll, WindowSettingsHandler_ReadLine, WindowSettingsHandler_WriteAll};
use crate::viewport::ImGuiViewport;
use crate::viewport_flags::ImGuiViewportFlags_OwnedByApp;
use crate::viewport_ops::DestroyPlatformWindows;

mod style;
mod io;
mod storage;
mod text_filter;
mod type_defs;
mod text_buffer;
mod context;
mod window;
mod platform_io;
mod vec2;
mod viewport;
mod draw_list;
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
mod combo_preview_data;
mod platform_ime_data;
mod dock_context;
mod dock_request_type;
mod dock_request;
mod direction;
mod dock_node_settings;
mod axis;
mod settings_handler;
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
mod hovered_flags;
mod mod_flags;
mod popup_flags;
mod selectable_flags;
mod slider_flags;
mod input_text_flags;
mod tab_item_flags;
mod table_column_flags;
mod table_row_flags;
mod tree_node_flags;
mod viewport_flags;
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
mod list_clipper;
mod old_column_data;
mod draw_list_splitter;
mod table_ops;
mod logging_ops;
mod span;
mod table_column;
mod table_cell_data;
mod table_instance_data;
mod table_column_sort_specs;
mod draw_cmd;
mod draw;
mod draw_vert;
mod draw_cmd_header;
mod cursor_ops;
mod config;
mod constants;
mod imgui;
mod utils;
mod geometry_ops;
mod string_ops;
mod hash_ops;
mod file_ops;
mod color_ops;
mod list_clipper_range;
mod list_clipper_ops;
mod math_ops;
mod style_ops;
mod style_var_info;
mod g_style_var_info;
mod style_var_ops;
mod render_ops;
mod stack_sizes;
mod imvec1;
mod debug_ops;
mod garbage_collection;
mod id_ops;
mod item_ops;
mod input_ops;
mod memory_management;
mod key_data;
mod input_event_type;
mod io_ops;
mod clipboard_ops;
mod context_ops;
mod draw_data_ops;
mod draw_list_ops;
mod mouse_ops;
mod keyboard_ops;
mod frame_ops;
mod viewport_ops;
mod text_ops;
mod child_ops;
mod content_ops;
mod nav_ops;
mod layout_ops;
mod shade_verts_ops;
mod font_atlas_default_tex_data;
mod font_atlas_custom_rect;
mod font_builder_io;
mod font_build_src_data;
mod font_build_dst_data;
mod font_atlas_ops;
mod font_glyph_ranges_builder;
mod font_ops;
mod fallback_font_data;
mod size_callback_data;
mod resize_grip_def;
mod resize_border_def;
mod widget_ops;
mod state_storage_ops;
mod error_ops;
mod group_ops;
mod scrolling_ops;
mod tooltip_ops;
mod popup_ops;
mod popup_position_policy;
mod drag_drop_ops;
mod settings_ops;
mod dock_preview_data;
mod dock_context_ops;
mod dock_context_prune_node_data;
mod dock_node_ops;
mod dock_node_state;
mod bit_array;
mod bit_vector;
mod bit_array_ops;
mod span_allocator;
mod plot_type;
mod data_type_temp_storage;
mod data_type_info;
mod menu_columns;
mod input_text_callback_data;
mod table_sort_specs;
mod once_upon_a_frame;
mod dock_node_tree_info;
mod docking_ops;
mod platform_support;
mod stb;
mod tables;
mod merge_group;
mod a_widgets;
mod slider_ops;
mod button_ops;
mod image_ops;
mod checkbox_ops;
mod radio_button;
mod progress_bar;
mod bullet;
mod separator;
mod splitter;
mod combo_box;
mod data_type_ops;
mod drag;

// c_void Initialize()
pub unsafe fn Initialize()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(!g.Initialized && !g.SettingsLoaded);

    // Add .ini handle for ImGuiWindow type
    {
        let mut ini_handler = ImGuiSettingsHandler::new();
        ini_handler.TypeName = "Window";
        ini_handler.TypeHash = ImHashStr2("Window");
        ini_handler.ClearAllFn = WindowSettingsHandler_ClearAll;
        ini_handler.ReadOpenFn = WindowSettingsHandler_ReadOpen;
        ini_handler.ReadLineFn = WindowSettingsHandler_ReadLine;
        ini_handler.ApplyAllFn = WindowSettingsHandler_ApplyAll;
        ini_handler.WriteAllFn = WindowSettingsHandler_WriteAll;
        AddSettingsHandler(&ini_handler);
    }

    // Add .ini handle for ImGuiTable type
    TableSettingsAddSettingsHandler();

    // Create default viewport
    let mut viewport: *mut ImGuiViewport =  IM_NEW(ImGuiViewportP)();
    viewport.ID = IMGUI_VIEWPORT_DEFAULT_ID;
    viewport.Idx = 0;
    viewport.PlatformWindowCreated = true;
    viewport.Flags = ImGuiViewportFlags_OwnedByApp;
    g.Viewports.push(viewport);
    g.TempBuffer.resize(1024 * 3 + 1, 0);
    g.PlatformIO.Viewports.push(g.Viewports[0]);

// #ifdef IMGUI_HAS_DOCK
    // Initialize Docking
    DockContextInitialize(g);
// #endif


    g.Initialized = true;
}

// This function is merely here to free heap allocations.
// c_void Shutdown()
pub unsafe fn Shutdown()
{
    // The fonts atlas can be used prior to calling NewFrame(), so we clear it even if g.Initialized is FALSE (which would happen if we never called NewFrame)
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if g.IO.Fonts.is_null() == false && g.FontAtlasOwnedByContext
    {
        g.IO.Fonts.Locked = false;
        IM_DELETE(g.IO.Fonts);
    }
    g.IO.Fonts= null_mut();

    // Cleanup of other data are conditional on actually having initialized Dear ImGui.
    if !g.Initialized {
        return;
    }

    // Save settings (unless we haven't attempted to load them: CreateContext/DestroyContext without a call to NewFrame shouldn't save an empty file)
    if g.SettingsLoaded && g.IO.IniFilename != null_mut() {
        SaveIniSettingsToDisk(g.IO.IniFilename);
    }

    // Destroy platform windows
    DestroyPlatformWindows();

    // Shutdown extensions
    DockContextShutdown(g);

    CallContextHooks(g, ImGuiContextHookType_Shutdown);

    // Clear everything else
    g.Windows.clear_delete();
    g.WindowsFocusOrder.clear();
    g.WindowsTempSortBuffer.clear();
    g.CurrentWindow= null_mut();
    g.CurrentWindowStack.clear();
    g.WindowsById.Clear();
    g.NavWindow= null_mut();
    g.HoveredWindow = null_Mut();
    g.HoveredWindowUnderMovingWindow= null_mut();
    g.ActiveIdWindow = null_mut();
    g.ActiveIdPreviousFrameWindow= null_mut();
    g.MovingWindow= null_mut();
    g.ColorStack.clear();
    g.StyleVarStack.clear();
    g.FontStack.clear();
    g.OpenPopupStack.clear();
    g.BeginPopupStack.clear();

    g.CurrentViewport = null_mut();
    g.MouseViewport = null_mut();
    g.MouseLastHoveredViewport= null_mut();
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

    if g.LogFile
    {
// #ifndef IMGUI_DISABLE_TTY_FUNCTIONS
        if g.LogFile != libc::stdout {
// #endif
            ImFileClose(g.LogFile);
        }
        g.LogFile= null_mut();
    }
    g.LogBuffer.clear();
    g.DebugLogBuf.clear();

    g.Initialized = false;
}
