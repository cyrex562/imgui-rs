use crate::{Context, INVALID_ID, SettingsHandler};
use crate::axis::Axis;
use crate::dock::context::{dock_context_find_node_by_id, DockContext};
use crate::dock::node::{DockNode, DockNodeFlags, DockNodeSettings};
use crate::text_buffer::TextBuffer;
use crate::types::Id32;

// static void DockSettingsRenameNodeReferences(ImGuiID old_node_id, ImGuiID new_node_id)
pub fn dock_settings_rename_node_references(g: &mut Context, old_node_id: Id32, new_node_id: Id32)
{
    // ImGuiContext& g = *GImGui;
    // IMGUI_DEBUG_LOG_DOCKING("[docking] DockSettingsRenameNodeReferences: from 0x%08X -> to 0x%08X\n", old_node_id, new_node_id);
    for (int window_n = 0; window_n < g.windows.len(); window_n += 1)
    {
        ImGuiWindow* window = g.windows[window_n];
        if (window.dock_id == old_node_id && window.dock_node == NULL)
            window.dock_id = new_node_id;
    }
    //// FIXME-OPT: We could remove this loop by storing the index in the map
    for (ImGuiWindowSettings* settings = g.settings_windows.begin(); settings != NULL; settings = g.settings_windows.next_chunk(settings))
        if (settings.dock_id == old_node_id)
            settings.dock_id = new_node_id;
}

// Remove references stored in ImGuiWindowSettings to the given ImGuiDockNodeSettings
// static void DockSettingsRemoveNodeReferences(ImGuiID* node_ids, int node_ids_count)
pub fn dock_settings_remove_node_references(g: &mut Context, node_ids: &mut Id32, node_ids_count: i32)
{
    // ImGuiContext& g = *GImGui;
    int found = 0;
    //// FIXME-OPT: We could remove this loop by storing the index in the map
    for (ImGuiWindowSettings* settings = g.settings_windows.begin(); settings != NULL; settings = g.settings_windows.next_chunk(settings))
        for (int node_n = 0; node_n < node_ids_count; node_n += 1)
            if (settings.dock_id == node_ids[node_n])
            {
                settings.dock_id = INVALID_ID;
                settings.dock_order = -1;
                if (found += 1 < node_ids_count)
                    break;
                return;
            }
}

// static ImGuiDockNodeSettings* DockSettingsFindNodeSettings(ImGuiContext* ctx, ImGuiID id)
pub fn dock_settings_find_node_settings(g: &mut Context, id: Id32) -> &mut DockNodeSettings
{
    // FIXME-OPT
    ImGuiDockContext* dc  = &g.dock_context;
    for (int n = 0; n < dc.NodesSettings.size; n += 1)
        if (dc.NodesSettings[n].id == id)
            return &dc.NodesSettings[n];
    return NULL;
}

// clear settings data
// static void DockSettingsHandler_ClearAll(ImGuiContext* ctx, ImGuiSettingsHandler*)
pub fn dock_settings_handler_clear_all(g: &mut Context, handler: &mut SettingsHandler)
{
    ImGuiDockContext* dc  = &g.dock_context;
    dc.NodesSettings.clear();
    DockContextClearNodes(g, 0, true);
}

// Recreate nodes based on settings data
// static void DockSettingsHandler_ApplyAll(ImGuiContext* ctx, ImGuiSettingsHandler*)
pub fn dock_settings_handler_apply_all(g: &mut Context, handler: &mut SettingsHandler)
{
    // Prune settings at boot time only
    ImGuiDockContext* dc  = &g.dock_context;
    if (g.windows.len() == 0)
        DockContextPruneUnusedSettingsNodes(g);
    DockContextBuildNodesFromSettings(g, dc.NodesSettings.data, dc.NodesSettings.size);
    DockContextBuildAddWindowsToNodes(g, 0);
}

// static void* DockSettingsHandler_ReadOpen(ImGuiContext*, ImGuiSettingsHandler*, const char* name)
pub fn dock_settings_handler_read_open(g: &mut Context, handler: &mut SettingsHandler, name: &str) -> bool
{
    if (strcmp(name, "data") != 0)
        return NULL;
    return (void*)1;
}

// static void DockSettingsHandler_ReadLine(ImGuiContext* ctx, ImGuiSettingsHandler*, void*, const char* line)
pub fn dock_settings_handler_read_line(g: &mut Context, handler: &mut SettingsHandler, data: &Vec<u8>, line: &str)
{
    char c = 0;
    int x = 0, y = 0;
    int r = 0;

    // Parsing, e.g.
    // " dock_node   id=0x00000001 pos=383,193 size=201,322 split=Y,0.506 "
    // "   dock_node id=0x00000002 Parent=0x00000001 "
    // Important: this code expect currently fields in a fixed order.
    ImGuiDockNodeSettings node;
    line = ImStrSkipBlank(line);
    if      (strncmp(line, "dock_node", 8) == 0)  { line = ImStrSkipBlank(line + strlen("dock_node")); }
    else if (strncmp(line, "DockSpace", 9) == 0) { line = ImStrSkipBlank(line + strlen("DockSpace")); node.flags |= DockNodeFlags::DockSpace; }
    else return;
    if (sscanf(line, "id=0x%08X%n",      &node.id, &r) == 1)            { line += r; } else return;
    if (sscanf(line, " Parent=0x%08X%n", &node.parent_node_id, &r) == 1)  { line += r; if (node.parent_node_id == 0) return; }
    if (sscanf(line, " window=0x%08X%n", &node.ParentWindowId, &r) ==1) { line += r; if (node.ParentWindowId == 0) return; }
    if (node.parent_node_id == 0)
    {
        if (sscanf(line, " pos=%i,%i%n",  &x, &y, &r) == 2)         { line += r; node.pos = Vector2D(x, y); } else return;
        if (sscanf(line, " size=%i,%i%n", &x, &y, &r) == 2)         { line += r; node.size = Vector2D(x, y); } else return;
    }
    else
    {
        if (sscanf(line, " size_ref=%i,%i%n", &x, &y, &r) == 2)      { line += r; node.size_ref = Vector2D(x, y); }
    }
    if (sscanf(line, " split=%c%n", &c, &r) == 1)                   { line += r; if (c == 'X') node.split_axis = Axis::X; else if (c == 'Y') node.split_axis = Axis::Y; }
    if (sscanf(line, " NoResize=%d%n", &x, &r) == 1)                { line += r; if (x != 0) node.flags |= DockNodeFlags::NoResize; }
    if (sscanf(line, " central_node=%d%n", &x, &r) == 1)             { line += r; if (x != 0) node.flags |= DockNodeFlags::CentralNode; }
    if (sscanf(line, " NoTabBar=%d%n", &x, &r) == 1)                { line += r; if (x != 0) node.flags |= DockNodeFlags::NoTabBar; }
    if (sscanf(line, " HiddenTabBar=%d%n", &x, &r) == 1)            { line += r; if (x != 0) node.flags |= DockNodeFlags::HiddenTabBar; }
    if (sscanf(line, " NoWindowMenuButton=%d%n", &x, &r) == 1)      { line += r; if (x != 0) node.flags |= DockNodeFlags::NoWindowMenuButton; }
    if (sscanf(line, " NoCloseButton=%d%n", &x, &r) == 1)           { line += r; if (x != 0) node.flags |= DockNodeFlags::NoCloseButton; }
    if (sscanf(line, " Selected=0x%08X%n", &node.selected_tab_id,&r) == 1) { line += r; }
    if (node.parent_node_id != 0)
        if (ImGuiDockNodeSettings* parent_settings = DockSettingsFindNodeSettings(g, node.parent_node_id))
            node.Depth = parent_settings.Depth + 1;
    g.dock_context.NodesSettings.push_back(node);
}

// static void DockSettingsHandler_DockNodeToSettings(ImGuiDockContext* dc, ImGuiDockNode* node, int depth)
pub fn dock_settings_handler_dock_node_to_settings(g: &mut Context, dc: &mut DockContext, node: &mut DockNode, depth: i32)
{
    ImGuiDockNodeSettings node_settings;
    // IM_ASSERT(depth < (1 << (sizeof(node_settings.Depth) << 3)));
    node_settings.id = node.id;
    node_settings.parent_node_id = node.parent_node ? node.parent_node.id : 0;
    node_settings.ParentWindowId = (node.is_dock_space() && node.host_window_id && node.host_window_id.parent_window) ? node.host_window_id.parent_window.id : 0;
    node_settings.selected_tab_id = node.selected_tab_id;
    node_settings.split_axis = (signed char)(node.is_split_node() ? node.split_axis : ImGuiAxis_None);
    node_settings.Depth = (char)depth;
    node_settings.flags = (node.local_flags & DockNodeFlags::SavedFlagsMask_);
    node_settings.pos = Vector2D(node.pos);
    node_settings.size = Vector2D(node.size);
    node_settings.size_ref = Vector2D(node.size_ref);
    dc.NodesSettings.push_back(node_settings);
    if (node.child_nodes[0])
        DockSettingsHandler::DockNodeToSettings(dc, node.child_nodes[0], depth + 1);
    if (node.child_nodes[1])
        DockSettingsHandler::DockNodeToSettings(dc, node.child_nodes[1], depth + 1);
}

// static void DockSettingsHandler_WriteAll(ImGuiContext* ctx, ImGuiSettingsHandler* handler, ImGuiTextBuffer* buf)
pub fn dock_settings_handler_write_all(g: &mut Context, handler: &mut SettingsHandler, bug: &mut TextBuffer)
{
    // ImGuiContext& g = *.g;
    ImGuiDockContext* dc = &g.dock_context;
    if (!(g.io.config_flags & ImGuiConfigFlags_DockingEnable))
        return;

    // Gather settings data
    // (unlike our windows settings, because nodes are always built we can do a full rewrite of the SettingsNode buffer)
    dc.NodesSettings.resize(0);
    dc.NodesSettings.reserve(dc.Nodes.data.size);
    for (int n = 0; n < dc.Nodes.data.size; n += 1)
        if (ImGuiDockNode* node = (ImGuiDockNode*)dc.Nodes.data[n].val_p)
            if (node.is_root_node())
                DockSettingsHandler::DockNodeToSettings(dc, node, 0);

    int max_depth = 0;
    for (int node_n = 0; node_n < dc.NodesSettings.size; node_n += 1)
        max_depth = ImMax(dc.NodesSettings[node_n].Depth, max_depth);

    // Write to text buffer
    buf.appendf("[%s][data]\n", handler.TypeName);
    for (int node_n = 0; node_n < dc.NodesSettings.size; node_n += 1)
    {
        const int line_start_pos = buf->size(); (void)line_start_pos;
        const ImGuiDockNodeSettings* node_settings = &dc.NodesSettings[node_n];
        buf.appendf("%*s%s%*s", node_settings.Depth * 2, "", (node_settings.flags & DockNodeFlags::DockSpace) ? "DockSpace" : "dock_node ", (max_depth - node_settings.Depth) * 2, "");  // Text align nodes to facilitate looking at .ini file
        buf.appendf(" id=0x%08X", node_settings->ID);
        if (node_settings->parent_node_id)
        {
            buf->appendf(" Parent=0x%08X size_ref=%d,%d", node_settings->parent_node_id, node_settings.size_ref.x, node_settings.size_ref.y);
        }
        else
        {
            if (node_settings->ParentWindowId)
                buf->appendf(" window=0x%08X", node_settings->ParentWindowId);
            buf->appendf(" pos=%d,%d size=%d,%d", node_settings.pos.x, node_settings.pos.y, node_settings.size.x, node_settings.size.y);
        }
        if (node_settings->SplitAxis != ImGuiAxis_None)
            buf->appendf(" split=%c", (node_settings->SplitAxis == Axis::X) ? 'X' : 'Y');
        if (node_settings.flags & DockNodeFlags::NoResize)
            buf->appendf(" NoResize=1");
        if (node_settings.flags & DockNodeFlags::CentralNode)
            buf->appendf(" central_node=1");
        if (node_settings.flags & DockNodeFlags::NoTabBar)
            buf->appendf(" NoTabBar=1");
        if (node_settings.flags & DockNodeFlags::HiddenTabBar)
            buf->appendf(" HiddenTabBar=1");
        if (node_settings.flags & DockNodeFlags::NoWindowMenuButton)
            buf->appendf(" NoWindowMenuButton=1");
        if (node_settings.flags & DockNodeFlags::NoCloseButton)
            buf->appendf(" NoCloseButton=1");
        if (node_settings->SelectedTabId)
            buf->appendf(" Selected=0x%08X", node_settings->SelectedTabId);

// #ifIMGUI_DEBUG_INI_SETTINGS
        // [DEBUG] Include comments in the .ini file to ease debugging
        if (ImGuiDockNode* node = dock_context_find_node_by_id(g, node_settings->ID))
        {
            buf->appendf("%*s", ImMax(2, (line_start_pos + 92) - buf->size()), "");     // Align everything
            if (node->is_dock_space() && node->HostWindow && node->HostWindow->parent_window)
                buf->appendf(" ; in '%s'", node->HostWindow->parent_window->Name);
            // Iterate settings so we can give info about windows that didn't exist during the session.
            int contains_window = 0;
            for (ImGuiWindowSettings* settings = g.settings_windows.begin(); settings != NULL; settings = g.settings_windows.next_chunk(settings))
                if (settings.dock_id == node_settings->ID)
                {
                    if (contains_window += 1 == 0)
                        buf->appendf(" ; contains ");
                    buf->appendf("'%s' ", settings->GetName());
                }
        }

        buf->appendf("\n");
    }
    buf->appendf("\n");
}
