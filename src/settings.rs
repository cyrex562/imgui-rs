use crate::Context;
use crate::orig_imgui_single_file::{buf, buf_end, ImGuiID, ImGuiWindow};
use crate::text_buffer::TextBuffer;
use crate::types::Id32;
use crate::window::{Window, WindowFlags};
use crate::window::settings::{apply_window_settings, WindowSettings};

//-----------------------------------------------------------------------------
// [SECTION] Settings support
//-----------------------------------------------------------------------------
#[derive(Default,Debug,Clone)]
pub struct SettingsHandler
{
    // const char* TypeName;       // Short description stored in .ini file. Disallowed characters: '[' ']'
    pub type_name: String,
    // ImGuiID     type_hash;       // == ImHashStr(TypeName)
    pub type_hash: Id32,
    // void        (*clear_all_fn)(ImGuiContext* ctx, ImGuiSettingsHandler* handler);                                // clear all settings data
    pub clear_all_fn: Option<fn(.g: &mut DimgContext, handler: &mut SettingsHandler)>,
    // void        (*ReadInitFn)(ImGuiContext* ctx, ImGuiSettingsHandler* handler);                                // Read: Called before reading (in registration order)
    pub read_init_fn: Option<fn(.g: &mut DimgContext, handler: &mut SettingsHandler)>,
    // void*       (*read_open_fn)(ImGuiContext* ctx, ImGuiSettingsHandler* handler, const char* name);              // Read: Called when entering into a new ini entry e.g. "[window][name]"
    pub read_open_fn: Option<fn(.g: &mut DimgContext, handler: &mut SettingsHandler, name: &String)>,
    // void        (*read_line_fn)(ImGuiContext* ctx, ImGuiSettingsHandler* handler, void* entry, const char* line); // Read: Called for every line of text within an ini entry
    pub read_line_fn: Option<fn(.g: &mut DimgContext, handler: &mut SettingsHandler, entry: &mut Vec<u8>, line: &String)>,
    // void        (*apply_all_fn)(ImGuiContext* ctx, ImGuiSettingsHandler* handler);                                // Read: Called after reading (in registration order)
    pub apply_all_fn: Option<fn(.g: &mut DimgContext, handler: &mut SettingsHandler)>,
    // void        (*write_all_fn)(ImGuiContext* ctx, ImGuiSettingsHandler* handler, ImGuiTextBuffer* out_buf);      // Write: Output every entries into 'out_buf'
    pub write_all_fn: Option<fn(.g: &mut DimgContext, handler: SettingsHandler, out_buf: &mut DimgTextBuffer)>,
    // void*       user_data;
    pub user_data: Vec<u8>,
    //ImGuiSettingsHandler() { memset(this, 0, sizeof(*this)); }
}

// Called by NewFrame()
// void update_settings()
pub fn update_settings(g: &mut Context)
{
    // Load settings on first frame (if not explicitly loaded manually before)
    // ImGuiContext& g = *GImGui;
    if (!g.settings_loaded)
    {
        // IM_ASSERT(g.settings_windows.empty());
        if (g.io.ini_file_name)
            LoadIniSettingsFromDisk(g.io.ini_file_name);
        g.settings_loaded = true;
    }

    // Save settings (with a delay after the last modification, so we don't spam disk too much)
    if (g.SettingsDirtyTimer > 0.0)
    {
        g.SettingsDirtyTimer -= g.io.delta_time;
        if (g.SettingsDirtyTimer <= 0.0)
        {
            if (g.io.ini_file_name != NULL)
                save_ini_settings_to_disk(g.io.ini_file_name);
            else
                g.io.WantSaveIniSettings = true;  // Let user know they can call SaveIniSettingsToMemory(). user will need to clear io.want_save_ini_settings themselves.
            g.SettingsDirtyTimer = 0.0;
        }
    }
}

// void MarkIniSettingsDirty()
pub fn mark_ini_settings_dirty(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    if (g.SettingsDirtyTimer <= 0.0)
        g.SettingsDirtyTimer = g.io.IniSavingRate;
}

// void MarkIniSettingsDirty(ImGuiWindow* window)
pub fn mark_ini_settings_dirty2(g: &mut Context, window: &mut Window)
{
    // ImGuiContext& g = *GImGui;
    if (!(window.flags & WindowFlags::NoSavedSettings))
        if (g.SettingsDirtyTimer <= 0.0)
            g.SettingsDirtyTimer = g.io.IniSavingRate;
}

// ImGuiWindowSettings* CreateNewWindowSettings(const char* name)
pub fn create_new_window_settings(g: &mut Context, name: &String) -> &mut WindowSettings
{
    // ImGuiContext& g = *GImGui;

// #if!IMGUI_DEBUG_INI_SETTINGS
    // Skip to the "###" marker if any. We don't skip past to match the behavior of GetID()
    // Preserve the full string when IMGUI_DEBUG_INI_SETTINGS is set to make .ini inspection easier.
    if (const char* p = strstr(name, "###"))
        name = p;

    const size_t name_len = strlen(name);

    // Allocate chunk
    const size_t chunk_size = sizeof(ImGuiWindowSettings) + name_len + 1;
    ImGuiWindowSettings* settings = g.settings_windows.alloc_chunk(chunk_size);
    IM_PLACEMENT_NEW(settings) ImGuiWindowSettings();
    settings.id = ImHashStr(name, name_len);
    memcpy(settings.GetName(), name, name_len + 1);   // Store with zero terminator

    return settings;
}

// ImGuiWindowSettings* FindWindowSettings(ImGuiID id)
pub fn find_window_settings(g: &mut Context, id: Id32) -> &mut WindowSettings
{
    // ImGuiContext& g = *GImGui;
    for (ImGuiWindowSettings* settings = g.settings_windows.begin(); settings != NULL; settings = g.settings_windows.next_chunk(settings))
        if (settings.id == id)
            return settings;
    return NULL;
}

// ImGuiWindowSettings* FindOrCreateWindowSettings(const char* name)
pub fn find_or_create_window_settings(g: &mut Context, name: &str) -> &mut WindowSettings
{
    if (ImGuiWindowSettings* settings = FindWindowSettings(ImHashStr(name)))
        return settings;
    return CreateNewWindowSettings(name);
}

// void add_settings_handler(const ImGuiSettingsHandler* handler)
pub fn add_settings_handler(g: &mut Context, handler: &SettingsHandler)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(FindSettingsHandler(handler.TypeName) == NULL);
    g.settings_handlers.push_back(*handler);
}

// void RemoveSettingsHandler(const char* type_name)
pub fn remove_settings_handler(g: &mut Context, type_name: &str)
{
    // ImGuiContext& g = *GImGui;
    if (ImGuiSettingsHandler* handler = FindSettingsHandler(type_name))
        g.settings_handlers.erase(handler);
}

// ImGuiSettingsHandler* FindSettingsHandler(const char* type_name)
pub fn find_settings_handler(g: &mut Context, type_name: &str) -> &mut SettingsHandler
{
    // ImGuiContext& g = *GImGui;
    const ImGuiID type_hash = ImHashStr(type_name);
    for (int handler_n = 0; handler_n < g.settings_handlers.size; handler_n += 1)
        if (g.settings_handlers[handler_n].type_hash == type_hash)
            return &g.settings_handlers[handler_n];
    return NULL;
}

// void ClearIniSettings()
pub fn clear_ini_settings(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    g.SettingsIniData.clear();
    for (int handler_n = 0; handler_n < g.settings_handlers.size; handler_n += 1)
        if (g.settings_handlers[handler_n].clear_all_fn)
            g.settings_handlers[handler_n].clear_all_fn(&g, &g.settings_handlers[handler_n]);
}

// void LoadIniSettingsFromDisk(const char* ini_filename)
pub fn load_ini_settings(g: &mut Context, ini_filename: &str)
{
    size_t file_data_size = 0;
    char* file_data = (char*)ImFileLoadToMemory(ini_filename, "rb", &file_data_size);
    if (!file_data)
        return;
    if (file_data_size > 0)
        LoadIniSettingsFromMemory(file_data, file_data_size);
    IM_FREE(file_data);
}

// Zero-tolerance, no error reporting, cheap .ini parsing
// void LoadIniSettingsFromMemory(const char* ini_data, size_t ini_size)
pub fn load_ini_settings_from_memory(g: &mut Context, ini_data: &str, ini_size: usize)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.initialized);
    //IM_ASSERT(!g.within_frame_scope && "Cannot be called between NewFrame() and EndFrame()");
    //IM_ASSERT(g.settings_loaded == false && g.frame_count == 0);

    // For user convenience, we allow passing a non zero-terminated string (hence the ini_size parameter).
    // For our convenience and to make the code simpler, we'll also write zero-terminators within the buffer. So let's create a writable copy..
    if (ini_size == 0)
        ini_size = strlen(ini_data);
    g.SettingsIniData.Buf.resize(ini_size + 1);
    char* const buf = g.SettingsIniData.Buf.data;
    char* const buf_end = buf + ini_size;
    memcpy(buf, ini_data, ini_size);
    buf_end[0] = 0;

    // Call pre-read handlers
    // Some types will clear their data (e.g. dock information) some types will allow merge/override (window)
    for (int handler_n = 0; handler_n < g.settings_handlers.size; handler_n += 1)
        if (g.settings_handlers[handler_n].ReadInitFn)
            g.settings_handlers[handler_n].ReadInitFn(&g, &g.settings_handlers[handler_n]);

    void* entry_data = NULL;
    ImGuiSettingsHandler* entry_handler = NULL;

    char* line_end = NULL;
    for (char* line = buf; line < buf_end; line = line_end + 1)
    {
        // Skip new lines markers, then find end of the line
        while (*line == '\n' || *line == '\r')
            line += 1;
        line_end = line;
        while (line_end < buf_end && *line_end != '\n' && *line_end != '\r')
            line_end += 1;
        line_end[0] = 0;
        if (line[0] == ';')
            continue;
        if (line[0] == '[' && line_end > line && line_end[-1] == ']')
        {
            // Parse "[Type][name]". Note that 'name' can itself contains [] characters, which is acceptable with the current format and parsing code.
            line_end[-1] = 0;
            const char* name_end = line_end - 1;
            const char* type_start = line + 1;
            char* type_end = (char*)(void*)ImStrchrRange(type_start, name_end, ']');
            const char* name_start = type_end ? ImStrchrRange(type_end + 1, name_end, '[') : NULL;
            if (!type_end || !name_start)
                continue;
            *type_end = 0; // Overwrite first ']'
            name_start += 1;  // Skip second '['
            entry_handler = FindSettingsHandler(type_start);
            entry_data = entry_handler ? entry_handler.read_open_fn(&g, entry_handler, name_start) : NULL;
        }
        else if (entry_handler != NULL && entry_data != NULL)
        {
            // Let type handler parse the line
            entry_handler.read_line_fn(&g, entry_handler, entry_data, line);
        }
    }
    g.settings_loaded = true;

    // [DEBUG] Restore untouched copy so it can be browsed in Metrics (not strictly necessary)
    memcpy(buf, ini_data, ini_size);

    // Call post-read handlers
    for (int handler_n = 0; handler_n < g.settings_handlers.size; handler_n += 1)
        if (g.settings_handlers[handler_n].apply_all_fn)
            g.settings_handlers[handler_n].apply_all_fn(&g, &g.settings_handlers[handler_n]);
}

// void save_ini_settings_to_disk(const char* ini_filename)
pub fn save_ini_settings_to_disk(g: &mut Context, ini_filename: &str)
{
    // ImGuiContext& g = *GImGui;
    g.SettingsDirtyTimer = 0.0;
    if (!ini_filename)
        return;

    size_t ini_data_size = 0;
    const char* ini_data = SaveIniSettingsToMemory(&ini_data_size);
    ImFileHandle f = ImFileOpen(ini_filename, "wt");
    if (!f)
        return;
    ImFileWrite(ini_data, sizeof(char), ini_data_size, f);
    ImFileClose(f);
}

// Call registered handlers (e.g. SettingsHandlerWindow_WriteAll() + custom handlers) to write their stuff into a text buffer
// const char* SaveIniSettingsToMemory(size_t* out_size)
pub fn save_init_settings(g: &mut Context, out_size: &mut usize) -> String
{
    // ImGuiContext& g = *GImGui;
    g.SettingsDirtyTimer = 0.0;
    g.SettingsIniData.Buf.resize(0);
    g.SettingsIniData.Buf.push_back(0);
    for (int handler_n = 0; handler_n < g.settings_handlers.size; handler_n += 1)
    {
        ImGuiSettingsHandler* handler = &g.settings_handlers[handler_n];
        handler.write_all_fn(&g, handler, &g.SettingsIniData);
    }
    if (out_size)
        *out_size = g.SettingsIniData.size();
    return g.SettingsIniData.c_str();
}

// static void WindowSettingsHandler_ClearAll(ImGuiContext* ctx, ImGuiSettingsHandler*)
pub fn window_settings_handler_clear_all(g: &mut Context, handler: &mut SettingsHandler)
{
    // ImGuiContext& g = *.g;
    for (int i = 0; i != g.windows.len(); i += 1)
        g.windows[i].SettingsOffset = -1;
    g.settings_windows.clear();
}

// static void* WindowSettingsHandler_ReadOpen(ImGuiContext*, ImGuiSettingsHandler*, const char* name)
pub fn window_settings_handler_read_open(g: &mut Context, handler: &mut SettingsHandler, name: &str) -> WindowSettings
{
    ImGuiWindowSettings* settings = FindOrCreateWindowSettings(name);
    ImGuiID id = settings.id;
    *settings = ImGuiWindowSettings(); // clear existing if recycling previous entry
    settings.id = id;
    settings.WantApply = true;
    return (void*)settings;
}

// static void WindowSettingsHandler_ReadLine(ImGuiContext*, ImGuiSettingsHandler*, void* entry, const char* line)
pub fn window_settings_handler_read_line(g: &mut Context, handler: &mut SettingsHandler, entry: &mut Vec<u8>, line: &str)
{
    ImGuiWindowSettings* settings = (ImGuiWindowSettings*)entry;
    int x, y;
    int i;
    ImU32 u1;
    if (sscanf(line, "pos=%i,%i", &x, &y) == 2)             { settings.pos = Vector2Dih(x, y); }
    else if (sscanf(line, "size=%i,%i", &x, &y) == 2)       { settings.size = Vector2Dih(x, y); }
    else if (sscanf(line, "viewport_id=0x%08X", &u1) == 1)   { settings.viewport_id = u1; }
    else if (sscanf(line, "viewport_pos=%i,%i", &x, &y) == 2){ settings.viewport_pos = Vector2Dih(x, y); }
    else if (sscanf(line, "collapsed=%d", &i) == 1)         { settings.collapsed = (i != 0); }
    else if (sscanf(line, "dock_id=0x%x,%d", &u1, &i) == 2)  { settings.dock_id = u1; settings.dock_order = i; }
    else if (sscanf(line, "dock_id=0x%x", &u1) == 1)         { settings.dock_id = u1; settings.dock_order = -1; }
    else if (sscanf(line, "class_id=0x%x", &u1) == 1)        { settings.ClassId = u1; }
}

// Apply to existing windows (if any)
// static void WindowSettingsHandler_ApplyAll(ImGuiContext* ctx, ImGuiSettingsHandler*)
pub fn window_handler_apply_all(g: &mut Context, handler: &mut SettingsHandler)
{
    // ImGuiContext& g = *.g;
    for (ImGuiWindowSettings* settings = g.settings_windows.begin(); settings != NULL; settings = g.settings_windows.next_chunk(settings))
        if (settings.WantApply)
        {
            if (ImGuiWindow* window = FindWindowByID(settings.id))
                apply_window_settings(window, settings);
            settings.WantApply = false;
        }
}

// static void WindowSettingsHandler_WriteAll(ImGuiContext* ctx, ImGuiSettingsHandler* handler, ImGuiTextBuffer* buf)
pub fn window_settings_handler_write_all(g: &mut Context, handler: &mut SettingsHandler, buf: &mut TextBuffer)
{
    // Gather data from windows that were active during this session
    // (if a window wasn't opened in this session we preserve its settings)
    // ImGuiContext& g = *.g;
    for (int i = 0; i != g.windows.len(); i += 1)
    {
        ImGuiWindow* window = g.windows[i];
        if (window.flags & WindowFlags::NoSavedSettings)
            continue;

        ImGuiWindowSettings* settings = (window.settings_offset != -1) ? g.settings_windows.ptr_from_offset(window.settings_offset) : FindWindowSettings(window.id);
        if (!settings)
        {
            settings = CreateNewWindowSettings(window.Name);
            window.settings_offset = g.settings_windows.offset_from_ptr(settings);
        }
        // IM_ASSERT(settings.ID == window.id);
        settings.pos = Vector2Dih(window.pos - window.viewport_pos);
        settings.size = Vector2Dih(window.size_full);
        settings.viewport_id = window.viewport_id;
        settings.viewport_pos = Vector2Dih(window.viewport_pos);
        // IM_ASSERT(window.dock_node == NULL || window.dock_node.ID == window.DockId);
        settings.dock_id = window.dock_id;
        settings.ClassId = window.window_class.ClassId;
        settings.dock_order = window.DockOrder;
        settings.collapsed = window.collapsed;
    }

    // Write to text buffer
    buf.reserve(buf->size() + g.settings_windows.len()() * 6); // ballpark reserve
    for (ImGuiWindowSettings* settings = g.settings_windows.begin(); settings != NULL; settings = g.settings_windows.next_chunk(settings))
    {
        const char* settings_name = settings.GetName();
        buf.appendf("[%s][%s]\n", handler.TypeName, settings_name);
        if (settings.viewport_id != 0 && settings.viewport_id != IMGUI_VIEWPORT_DEFAULT_ID)
        {
            buf.appendf("viewport_pos=%d,%d\n", settings.viewport_pos.x, settings.viewport_pos.y);
            buf.appendf("viewport_id=0x%08X\n", settings.viewport_id);
        }
        if (settings.pos.x != 0 || settings.pos.y != 0 || settings.viewport_id == IMGUI_VIEWPORT_DEFAULT_ID)
            buf.appendf("pos=%d,%d\n", settings.pos.x, settings.pos.y);
        if (settings.size.x != 0 || settings.size.y != 0)
            buf.appendf("size=%d,%d\n", settings.size.x, settings.size.y);
        buf.appendf("collapsed=%d\n", settings.collapsed);
        if (settings.dock_id != 0)
        {
            //buf->appendf("tab_id=0x%08X\n", ImHashStr("#TAB", 4, settings->id)); // window->tab_id: this is not read back but writing it makes "debugging" the .ini data easier.
            if (settings.dock_order == -1)
                buf.appendf("dock_id=0x%08X\n", settings.dock_id);
            else
                buf.appendf("dock_id=0x%08X,%d\n", settings.dock_id, settings.dock_order);
            if (settings.ClassId != 0)
                buf.appendf("class_id=0x%08X\n", settings.ClassId);
        }
        buf.append("\n");
    }
}
