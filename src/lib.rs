extern crate windows;

use crate::context_ops::CallContextHooks;
use crate::dock_context_ops::{init_dock_context, shutdown_dock_context};
use crate::docking::dock_context_ops::{init_dock_context, shutdown_dock_context};
use core::file_ops::close_file;
use core::hash_ops::hash_string;
use crate::imgui::GImGui;
use core::settings_handler::SettingsHandler;
use crate::settings_ops::{
    AddSettingsHandler, save_ini_settings_to_disk, WindowSettingsHandler_ApplyAll,
    WindowSettingsHandler_ClearAll, WindowSettingsHandler_ReadLine, WindowSettingsHandler_WriteAll,
};
use crate::tables::TableSettingsAddSettingsHandler;
use core::type_defs::INVALID_IMGUI_HANDLE;
use crate::viewport::Viewport;
use viewport::viewport_flags::ImGuiViewportFlags_OwnedByApp;
use viewport::viewport_ops::DestroyPlatformWindows;
use core::context::AppContext;
use core::context_hook::IM_GUI_CONTEXT_HOOK_TYPE_SHUTDOWN;
use std::collections::HashSet;
use std::io::stdout;
use std::ptr::null_mut;
use crate::core::settings_ops::{save_ini_settings_to_disk, WindowSettingsHandler_ApplyAll, WindowSettingsHandler_ClearAll, WindowSettingsHandler_ReadLine, WindowSettingsHandler_WriteAll};
use crate::table::tables::TableSettingsAddSettingsHandler;

mod backends;
mod color;
mod core;
mod data_type;
mod debugging;
mod docking;
mod drag_drop;
mod drawing;
mod font;
mod io;

mod stb;
mod style;
mod table;
mod viewport;
mod widgets;
mod window;
mod item;
mod layout;
mod platform;
mod text;

// c_void initialize()
pub fn initialize(g: &mut AppContext) {
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
    let mut viewport: *mut Viewport = IM_NEW(ImGuiViewportP)();
    viewport.ID = IMGUI_VIEWPORT_DEFAULT_ID;
    viewport.Idx = 0;
    viewport.PlatformWindowCreated = true;
    viewport.Flags = ImGuiViewportFlags_OwnedByApp;
    g.Viewports.push(viewport);
    g.TempBuffer.resize(1024 * 3 + 1, 0);
    g.PlatformIO.Viewports.push(g.Viewports[0]);

    // #ifdef IMGUI_HAS_DOCK
    // initialize Docking
    init_dock_context(g);
    // #endif

    g.initialized = true;
}

// This function is merely here to free heap allocations.
// c_void shutdown()
pub fn shutdown(g: &mut AppContext) {
    // The fonts atlas can be used prior to calling NewFrame(), so we clear it even if g.Initialized is FALSE (which would happen if we never called NewFrame)
    if g.IO.Fonts.is_some() && g.FontAtlasOwnedByContext {
        g.IO.Fonts.Locked = false;
        // IM_DELETE(g.IO.Fonts);
    }
    g.IO.Fonts = None;

    // Cleanup of other data are conditional on actually having initialized Dear ImGui.
    if !g.initialized {
        return;
    }

    // Save settings (unless we haven't attempted to load them: CreateContext/DestroyContext without a call to NewFrame shouldn't save an empty file)
    if g.SettingsLoaded && g.IO.IniFilename.is_empty() == false {
        save_ini_settings_to_disk(g, &g.IO.IniFilename);
    }

    // Destroy platform windows
    DestroyPlatformWindows(g);

    // shutdown extensions
    shutdown_dock_context(g);

    // CallContextHooks(g, IM_GUI_CONTEXT_HOOK_TYPE_SHUTDOWN);
    g.call_context_hooks(IM_GUI_CONTEXT_HOOK_TYPE_SHUTDOWN);

    // Clear everything else
    // g.Windows.clear_delete();
    g.WindowsFocusOrder.clear();
    g.WindowsTempSortBuffer.clear();
    g.CurrentWindow = INVALID_IMGUI_HANDLE;
    g.CurrentWindowStack.clear();
    g.windows.Clear();
    g.NavWindow = INVALID_IMGUI_HANDLE;
    g.HoveredWindow = INVALID_IMGUI_HANDLE;
    g.HoveredWindowUnderMovingWindow = INVALID_IMGUI_HANDLE;
    g.ActiveIdWindow = INVALID_IMGUI_HANDLE;
    g.ActiveIdPreviousFrameWindow = INVALID_IMGUI_HANDLE;
    g.MovingWindow = INVALID_IMGUI_HANDLE;
    g.ColorStack.clear();
    g.style_var_stack.clear();
    g.FontStack.clear();
    g.OpenPopupStack.clear();
    g.BeginPopupStack.clear();

    g.CurrentViewport = INVALID_IMGUI_HANDLE;
    g.MouseViewport = INVALID_IMGUI_HANDLE;
    g.MouseLastHoveredViewport = INVALID_IMGUI_HANDLE;
    g.Viewports.clear_delete();

    g.TabBars.Clear();
    g.CurrentTabBarStack.clear();
    g.shrink_width_buffer.clear();

    g.ClipperTempData.clear_destruct();

    g.Tables.Clear();
    g.TablesTempData.clear_destruct();
    g.DrawChannelsTempMergeBuffer.clear();

    g.ClipboardHandlerData.clear();
    g.MenusIdSubmittedThisFrame.clear();
    g.InputTextState.ClearFreeMemory();

    g.settings_windows.clear();
    g.settings_handlers.clear();

    if g.log_file_handle.is_some() {
        // #ifndef IMGUI_DISABLE_TTY_FUNCTIONS
        if g.log_file_handle.unwrap() != libc::stdout {
            // #endif
            close_file(&g.log_file_handle.unwrap());
        }
        g.log_file_handle = None;
    }
    g.log_buffer.clear();
    g.debug_log_buffer.clear();

    g.initialized = false;
}
