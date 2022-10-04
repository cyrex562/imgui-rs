
//-----------------------------------------------------------------------------
// [SECTION] SETTINGS
//-----------------------------------------------------------------------------
// - UpdateSettings() [Internal]
// - MarkIniSettingsDirty() [Internal]
// - CreateNewWindowSettings() [Internal]
// - FindWindowSettings() [Internal]
// - FindOrCreateWindowSettings() [Internal]
// - FindSettingsHandler() [Internal]
// - ClearIniSettings() [Internal]
// - LoadIniSettingsFromDisk()
// - LoadIniSettingsFromMemory()
// - SaveIniSettingsToDisk()
// - SaveIniSettingsToMemory()
// - WindowSettingsHandler_***() [Internal]
//-----------------------------------------------------------------------------

// Called by NewFrame()
c_void UpdateSettings()
{
    // Load settings on first frame (if not explicitly loaded manually before)
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (!g.SettingsLoaded)
    {
        // IM_ASSERT(g.SettingsWindows.empty());
        if (g.IO.IniFilename)
            LoadIniSettingsFromDisk(g.IO.IniFilename);
        g.SettingsLoaded = true;
    }

    // Save settings (with a delay after the last modification, so we don't spam disk too much)
    if (g.SettingsDirtyTimer > 0f32)
    {
        g.SettingsDirtyTimer -= g.IO.DeltaTime;
        if (g.SettingsDirtyTimer <= 0f32)
        {
            if (g.IO.IniFilename != null_mut())
                SaveIniSettingsToDisk(g.IO.IniFilename);
            else
                g.IO.WantSaveIniSettings = true;  // Let user know they can call SaveIniSettingsToMemory(). user will need to clear io.WantSaveIniSettings themselves.
            g.SettingsDirtyTimer = 0f32;
        }
    }
}

c_void MarkIniSettingsDirty()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (g.SettingsDirtyTimer <= 0f32)
        g.SettingsDirtyTimer = g.IO.IniSavingRate;
}

c_void MarkIniSettingsDirty(window: *mut ImGuiWindow)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (!(window.Flags & ImGuiWindowFlags_NoSavedSettings))
        if (g.SettingsDirtyTimer <= 0f32)
            g.SettingsDirtyTimer = g.IO.IniSavingRate;
}

ImGuiWindowSettings* CreateNewWindowSettings(name: *const c_char)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

// #if !IMGUI_DEBUG_INI_SETTINGS
    // Skip to the "###" marker if any. We don't skip past to match the behavior of GetID()
    // Preserve the full string when IMGUI_DEBUG_INI_SETTINGS is set to make .ini inspection easier.
    if (*const char p = strstr(name, "###"))
        name = p;
// #endif
    const size_t name_len = strlen(name);

    // Allocate chunk
    const size_t chunk_size = sizeof(ImGuiWindowSettings) + name_len + 1;
    let mut settings: *mut ImGuiWindowSettings = g.SettingsWindows.alloc_chunk(chunk_size);
    IM_PLACEMENT_NEW(settings) ImGuiWindowSettings();
    settings.ID = ImHashStr(name, name_len);
    memcpy(settings.GetName(), name, name_len + 1);   // Store with zero terminator

    return settings;
}

ImGuiWindowSettings* FindWindowSettings(id: ImGuiID)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    for (let mut settings: *mut ImGuiWindowSettings = g.SettingsWindows.begin(); settings != null_mut(); settings = g.SettingsWindows.next_chunk(settings))
        if (settings.ID == id)
            return settings;
    return null_mut();
}

ImGuiWindowSettings* FindOrCreateWindowSettings(name: *const c_char)
{
    if (let mut settings: *mut ImGuiWindowSettings = FindWindowSettings(ImHashStr(name)))
        return settings;
    return CreateNewWindowSettings(name);
}

c_void AddSettingsHandler(*const ImGuiSettingsHandler handler)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(FindSettingsHandler(handler.TypeName) == NULL);
    g.SettingsHandlers.push(*handler);
}

c_void RemoveSettingsHandler(type_name: *const c_char)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (ImGuiSettingsHandler* handler = FindSettingsHandler(type_name))
        g.SettingsHandlers.erase(handler);
}

ImGuiSettingsHandler* FindSettingsHandler(type_name: *const c_char)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut type_hash: ImGuiID =  ImHashStr(type_name);
    for (let handler_n: c_int = 0; handler_n < g.SettingsHandlers.Size; handler_n++)
        if (g.SettingsHandlers[handler_n].TypeHash == type_hash)
            return &g.SettingsHandlers[handler_n];
    return null_mut();
}

c_void ClearIniSettings()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.SettingsIniData.clear();
    for (let handler_n: c_int = 0; handler_n < g.SettingsHandlers.Size; handler_n++)
        if (g.SettingsHandlers[handler_n].ClearAllFn)
            g.SettingsHandlers[handler_n].ClearAllFn(&g, &g.SettingsHandlers[handler_n]);
}

c_void LoadIniSettingsFromDisk(ini_filename: *const c_char)
{
    size_t file_data_size = 0;
    char* file_data = ImFileLoadToMemory(ini_filename, "rb", &file_data_size);
    if (!file_data)
        return;
    if (file_data_size > 0)
        LoadIniSettingsFromMemory(file_data, file_data_size);
    IM_FREE(file_data);
}

// Zero-tolerance, no error reporting, cheap .ini parsing
c_void LoadIniSettingsFromMemory(ini_data: *const c_char, size_t ini_size)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.Initialized);
    //IM_ASSERT(!g.WithinFrameScope && "Cannot be called between NewFrame() and EndFrame()");
    //IM_ASSERT(g.SettingsLoaded == false && g.FrameCount == 0);

    // For user convenience, we allow passing a non zero-terminated string (hence the ini_size parameter).
    // For our convenience and to make the code simpler, we'll also write zero-terminators within the buffer. So let's create a writable copy..
    if (ini_size == 0)
        ini_size = strlen(ini_data);
    g.SettingsIniData.Buf.resize(ini_size + 1);
    char* const buf = g.SettingsIniData.Buf.Data;
    char* const buf_end = buf + ini_size;
    memcpy(buf, ini_data, ini_size);
    buf_end[0] = 0;

    // Call pre-read handlers
    // Some types will clear their data (e.g. dock information) some types will allow merge/override (window)
    for (let handler_n: c_int = 0; handler_n < g.SettingsHandlers.Size; handler_n++)
        if (g.SettingsHandlers[handler_n].ReadInitFn)
            g.SettingsHandlers[handler_n].ReadInitFn(&g, &g.SettingsHandlers[handler_n]);

    entry_data: *mut c_void= null_mut();
    ImGuiSettingsHandler* entry_handler= null_mut();

    char* line_end= null_mut();
    for (char* line = buf; line < buf_end; line = line_end + 1)
    {
        // Skip new lines markers, then find end of the line
        while (*line == '\n' || *line == '\r')
            line+= 1;
        line_end = line;
        while (line_end < buf_end && *line_end != '\n' && *line_end != '\r')
            line_end+= 1;
        line_end[0] = 0;
        if (line[0] == ';')
            continue;
        if (line[0] == '[' && line_end > line && line_end[-1] == ']')
        {
            // Parse "[Type][Name]". Note that 'Name' can itself contains [] characters, which is acceptable with the current format and parsing code.
            line_end[-1] = 0;
            let mut  name_end: *const c_char = line_end - 1;
            let mut  type_start: *const c_char = line + 1;
            char* type_end = ImStrchrRange(type_start, name_end, ']');
            let mut  name_start: *const c_char = type_end ? ImStrchrRange(type_end + 1, name_end, '[') : null_mut();
            if (!type_end || !name_start)
                continue;
            *type_end = 0; // Overwrite first ']'
            name_start+= 1;  // Skip second '['
            entry_handler = FindSettingsHandler(type_start);
            entry_data = entry_handler ? entry_handler.ReadOpenFn(&g, entry_handler, name_start) : null_mut();
        }
        else if (entry_handler != null_mut() && entry_data != null_mut())
        {
            // Let type handler parse the line
            entry_handler.ReadLineFn(&g, entry_handler, entry_data, line);
        }
    }
    g.SettingsLoaded = true;

    // [DEBUG] Restore untouched copy so it can be browsed in Metrics (not strictly necessary)
    memcpy(buf, ini_data, ini_size);

    // Call post-read handlers
    for (let handler_n: c_int = 0; handler_n < g.SettingsHandlers.Size; handler_n++)
        if (g.SettingsHandlers[handler_n].ApplyAllFn)
            g.SettingsHandlers[handler_n].ApplyAllFn(&g, &g.SettingsHandlers[handler_n]);
}

c_void SaveIniSettingsToDisk(ini_filename: *const c_char)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.SettingsDirtyTimer = 0f32;
    if (!ini_filename)
        return;

    size_t ini_data_size = 0;
    let mut  ini_data: *const c_char = SaveIniSettingsToMemory(&ini_data_size);
    ImFileHandle f = ImFileOpen(ini_filename, "wt");
    if (!0f32)
        return;
    ImFileWrite(ini_data, sizeof, ini_data_size, 0f32);
    ImFileClose(0f32);
}

// Call registered handlers (e.g. SettingsHandlerWindow_WriteAll() + custom handlers) to write their stuff into a text buffer
*const char SaveIniSettingsToMemory(size_t* out_size)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.SettingsDirtyTimer = 0f32;
    g.SettingsIniData.Buf.clear();
    g.SettingsIniData.Buf.push(0);
    for (let handler_n: c_int = 0; handler_n < g.SettingsHandlers.Size; handler_n++)
    {
        ImGuiSettingsHandler* handler = &g.SettingsHandlers[handler_n];
        handler.WriteAllFn(&g, handler, &g.SettingsIniData);
    }
    if (out_size)
        *out_size = g.SettingsIniData.size();
    return g.SettingsIniData.c_str();
}

static c_void WindowSettingsHandler_ClearAll(ctx: *mut ImGuiContext, ImGuiSettingsHandler*)
{
    let g = ctx;
    for (let i: c_int = 0; i != g.Windows.len(); i++)
        g.Windows[i].SettingsOffset = -1;
    g.SettingsWindows.clear();
}

static *mut c_void WindowSettingsHandler_ReadOpen(ImGuiContext*, ImGuiSettingsHandler*, name: *const c_char)
{
    let mut settings: *mut ImGuiWindowSettings = FindOrCreateWindowSettings(name);
    let mut id: ImGuiID =  settings.ID;
    *settings = ImGuiWindowSettings(); // Clear existing if recycling previous entry
    settings.ID = id;
    settings.WantApply = true;
    return settings;
}

static c_void WindowSettingsHandler_ReadLine(ImGuiContext*, ImGuiSettingsHandler*, entry: *mut c_void, line: *const c_char)
{
    let mut settings: *mut ImGuiWindowSettings = (ImGuiWindowSettings*)entry;
    x: c_int, y;
    let mut i: c_int = 0;
    u32 u1;
    if (sscanf(line, "Pos=%i,%i", &x, &y) == 2)             { settings.Pos = ImVec2ih((c_short)x, y); }
    else if (sscanf(line, "Size=%i,%i", &x, &y) == 2)       { settings.Size = ImVec2ih((c_short)x, y); }
    else if (sscanf(line, "ViewportId=0x%08X", &u1) == 1)   { settings.ViewportId = u1; }
    else if (sscanf(line, "ViewportPos=%i,%i", &x, &y) == 2){ settings.ViewportPos = ImVec2ih((c_short)x, y); }
    else if (sscanf(line, "Collapsed=%d", &i) == 1)         { settings.Collapsed = (i != 0); }
    else if (sscanf(line, "DockId=0x%X,%d", &u1, &i) == 2)  { settings.DockId = u1; settings.DockOrder = i; }
    else if (sscanf(line, "DockId=0x%X", &u1) == 1)         { settings.DockId = u1; settings.DockOrder = -1; }
    else if (sscanf(line, "ClassId=0x%X", &u1) == 1)        { settings.ClassId = u1; }
}

// Apply to existing windows (if any)
static c_void WindowSettingsHandler_ApplyAll(ctx: *mut ImGuiContext, ImGuiSettingsHandler*)
{
    let g = ctx;
    for (let mut settings: *mut ImGuiWindowSettings = g.SettingsWindows.begin(); settings != null_mut(); settings = g.SettingsWindows.next_chunk(settings))
        if (settings.WantApply)
        {
            if (let mut window: *mut ImGuiWindow =  FindWindowByID(settings.ID))
                ApplyWindowSettings(window, settings);
            settings.WantApply = false;
        }
}

static c_void WindowSettingsHandler_WriteAll(ctx: *mut ImGuiContext, handler: *mut ImGuiSettingsHandler, buf: *mut ImGuiTextBuffer)
{
    // Gather data from windows that were active during this session
    // (if a window wasn't opened in this session we preserve its settings)
    let g = ctx;
    for (let i: c_int = 0; i != g.Windows.len(); i++)
    {
        let mut window: *mut ImGuiWindow =  g.Windows[i];
        if (window.Flags & ImGuiWindowFlags_NoSavedSettings)
            continue;

        let mut settings: *mut ImGuiWindowSettings = (window.SettingsOffset != -1) ? g.SettingsWindows.ptr_from_offset(window.SettingsOffset) : FindWindowSettings(window.ID);
        if (!settings)
        {
            settings = CreateNewWindowSettings(window.Name);
            window.SettingsOffset = g.SettingsWindows.offset_from_ptr(settings);
        }
        // IM_ASSERT(settings.ID == window.ID);
        settings.Pos = ImVec2ih(window.Pos - window.ViewportPos);
        settings.Size = ImVec2ih(window.SizeFull);
        settings.ViewportId = window.ViewportId;
        settings.ViewportPos = ImVec2ih(window.ViewportPos);
        // IM_ASSERT(window.DockNode == NULL || window.DockNode.ID == window.DockId);
        settings.DockId = window.DockId;
        settings.ClassId = window.WindowClass.ClassId;
        settings.DockOrder = window.DockOrder;
        settings.Collapsed = window.Collapsed;
    }

    // Write to text buffer
    buf.reserve(buf.size() + g.SettingsWindows.size() * 6); // ballpark reserve
    for (let mut settings: *mut ImGuiWindowSettings = g.SettingsWindows.begin(); settings != null_mut(); settings = g.SettingsWindows.next_chunk(settings))
    {
        let mut  settings_name: *const c_char = settings.GetName();
        buf.appendf("[%s][%s]\n", handler.TypeName, settings_name);
        if (settings.ViewportId != 0 && settings.ViewportId != IMGUI_VIEWPORT_DEFAULT_ID)
        {
            buf.appendf("ViewportPos=%d,%d\n", settings.ViewportPos.x, settings.ViewportPos.y);
            buf.appendf("ViewportId=0x%08X\n", settings.ViewportId);
        }
        if (settings.Pos.x != 0 || settings.Pos.y != 0 || settings.ViewportId == IMGUI_VIEWPORT_DEFAULT_ID)
            buf.appendf("Pos=%d,%d\n", settings.Pos.x, settings.Pos.y);
        if (settings.Size.x != 0 || settings.Size.y != 0)
            buf.appendf("Size=%d,%d\n", settings.Size.x, settings.Size.y);
        buf.appendf("Collapsed=%d\n", settings.Collapsed);
        if (settings.DockId != 0)
        {
            //buf.appendf("TabId=0x%08X\n", ImHashStr("#TAB", 4, settings.ID)); // window.TabId: this is not read back but writing it makes "debugging" the .ini data easier.
            if (settings.DockOrder == -1)
                buf.appendf("DockId=0x%08X\n", settings.DockId);
            else
                buf.appendf("DockId=0x%08X,%d\n", settings.DockId, settings.DockOrder);
            if (settings.ClassId != 0)
                buf.appendf("ClassId=0x%08X\n", settings.ClassId);
        }
        buf.append("\n");
    }
}
