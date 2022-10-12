//-----------------------------------------------------------------------------
// Docking: Settings
//-----------------------------------------------------------------------------
// - DockSettingsRenameNodeReferences()
// - DockSettingsRemoveNodeReferences()
// - DockSettingsFindNodeSettings()
// - DockSettingsHandler_ApplyAll()
// - DockSettingsHandler_ReadOpen()
// - DockSettingsHandler_ReadLine()
// - DockSettingsHandler_DockNodeToSettings()
// - DockSettingsHandler_WriteAll()
//-----------------------------------------------------------------------------

use std::ptr::null_mut;
use libc::{c_char, c_int};
use crate::axis::{ImGuiAxis_None, ImGuiAxis_X};
use crate::context::ImGuiContext;
use crate::dock_context::ImGuiDockContext;
use crate::dock_context_ops::{DockContextBuildAddWindowsToNodes, DockContextBuildNodesFromSettings, DockContextClearNodes};
use crate::dock_node::ImGuiDockNode;
use crate::dock_node_flags::{ImGuiDockNodeFlags_CentralNode, ImGuiDockNodeFlags_DockSpace, ImGuiDockNodeFlags_HiddenTabBar, ImGuiDockNodeFlags_NoCloseButton, ImGuiDockNodeFlags_NoResize, ImGuiDockNodeFlags_NoTabBar, ImGuiDockNodeFlags_NoWindowMenuButton, ImGuiDockNodeFlags_SavedFlagsMask_};
use crate::dock_node_settings::ImGuiDockNodeSettings;
use crate::{GImGui, ImGuiSettingsHandler};
use crate::config_flags::ImGuiConfigFlags_DockingEnable;
use crate::string_ops::{ImStrSkipBlank, str_to_const_c_char_ptr};
use crate::text_buffer::ImGuiTextBuffer;
use crate::type_defs::ImGuiID;
use crate::utils::{flag_set, is_not_null};
use crate::vec2::ImVec2ih;
use crate::window::ImGuiWindow;

// static c_void DockSettingsRenameNodeReferences(old_node_id: ImGuiID, new_node_id: ImGuiID)
pub unsafe fn DockSettingsRenameNodeReferences(old_node_id: ImGuiID, new_node_id: ImGuiID) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    IMGUI_DEBUG_LOG_DOCKING("[docking] DockSettingsRenameNodeReferences: from 0x%08X -> to 0x%08X\n", old_node_id, new_node_id);
    // for (let window_n: c_int = 0; window_n < g.Windows.len(); window_n++)
    for window_n in 0..g.Windows.len() {
        let mut window: *mut ImGuiWindow = g.Windows[window_n];
        if window.DockId == old_node_id && window.DockNode == null_mut() {
            window.DockId = new_node_id;
        }
    }
    //// FIXME-OPT: We could remove this loop by storing the index in the map
    // for (let mut settings: *mut ImGuiWindowSettings = g.SettingsWindows.begin(); settings != null_mut(); settings = g.SettingsWindows.next_chunk(settings))
    for settings in g.SettingsWindow.iter_mut() {
        if settings.DockId == old_node_id {
            settings.DockId = new_node_id;
        }
    }
}

// Remove references stored in ImGuiWindowSettings to the given ImGuiDockNodeSettings
// static c_void DockSettingsRemoveNodeReferences(node_ids: *mut ImGuiID, node_ids_count: c_int)
pub unsafe fn DockSettingsRemoveNodeReferences(node_ids: *mut ImGuiID, node_ids_count: c_int) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut found: c_int = 0;
    //// FIXME-OPT: We could remove this loop by storing the index in the map
    // for (let mut settings: *mut ImGuiWindowSettings = g.SettingsWindows.begin(); settings != null_mut(); settings = g.SettingsWindows.next_chunk(settings))
    for settings in g.SettingsWindow.iter_mut() {
        // for (let node_n: c_int = 0; node_n < node_ids_count; node_n+ +)
        for node_n in 0..node_ids_count {
            if settings.DockId == node_ids[node_n] {
                settings.DockId = 0;
                settings.DockOrder = -1;
                found += 1;
                if found < node_ids_count {
                    break;
                }
                return;
            }
        }
    }
}

// static ImGuiDockNodeSettings* DockSettingsFindNodeSettings(ctx: *mut ImGuiContext, id: ImGuiID)
pub fn DockSettingsFindNodeSettings(ctx: *mut ImGuiContext, id: ImGuiID) -> *mut ImGuiDockNodeSettings {

    // FIXME-OPT
    let dc = &mut ctx.DockContext;
    // for (let n: c_int = 0; n < dc.NodesSettings.Size; n++)
    for n in 0..dc.NodesSettings.len() {
        if dc.NodesSettings[n].ID == id {
            return &mut dc.NodesSettings[n];
        }
    }
    return null_mut();
}

// Clear settings data
// static c_void DockSettingsHandler_ClearAll(ctx: *mut ImGuiContext, ImGuiSettingsHandler*)
pub fn DockSettingsHandler_ClearAll(ctx: *mut ImGuiContext) {
    let dc = &mut ctx.DockContext;
    dc.NodesSettings.clear();
    DockContextClearNodes(ctx, 0, true);
}

// Recreate nodes based on settings data
// static c_void DockSettingsHandler_ApplyAll(ctx: *mut ImGuiContext, ImGuiSettingsHandler*)
pub unsafe fn DockSettingsHandler_ApplyAll(ctx: *mut ImGuiContext) {
    // Prune settings at boot time only
    let dc = &mut ctx.DockContext;
    if ctx.Windows.len() == 0 {
        DockContextPruneUnusedSettingsNodes(ctx);
    }
    DockContextBuildNodesFromSettings(ctx, dc.NodesSettings.Data, dc.NodesSettings.Size);
    DockContextBuildAddWindowsToNodes(ctx, 0);
}

// static *mut c_void DockSettingsHandler_ReadOpen(ImGuiContext*, ImGuiSettingsHandler*, name: *const c_char)
pub unsafe fn DockSettingsHandler_ReadOpen(name: *const c_char) -> c_int {
    // if (strcmp(name, "Data") != 0)
    //     return null_mut();
    // return 1;
    libc::strcmp(name, str_to_const_c_char_ptr("Data"))
}

// static c_void DockSettingsHandler_ReadLine(ctx: *mut ImGuiContext, ImGuiSettingsHandler*, *mut c_void, line: *const c_char)
pub unsafe fn DockSettingsHandler_ReadLine(ctx: *mut ImGuiContext, mut line: *const c_char) {
    let mut c: c_char = 0;
    let mut x: c_int = 0;
    let mut y = 0;
    let mut r: c_int = 0;

    // Parsing, e.g.
    // " DockNode   ID=0x00000001 Pos=383,193 Size=201,322 Split=Y,0.506 "
    // "   DockNode ID=0x00000002 Parent=0x00000001 "
    // Important: this code expect currently fields in a fixed order.
    // ImGuiDockNodeSettings node;
    let mut node = ImGuiDockNodeSettings::new();
    line = ImStrSkipBlank(line);
    if libc::strncmp(line, str_to_const_c_char_ptr("DockNode"), 8) == 0 { line = ImStrSkipBlank(line + libc::strlen(str_to_const_c_char_ptr("DockNode"))); } else if libc::strncmp(line, str_to_const_c_char_ptr("DockSpace"), 9) == 0 {
        line = ImStrSkipBlank(line + libc::strlen(str_to_const_c_char_ptr("DockSpace")));
        node.Flags |= ImGuiDockNodeFlags_DockSpace;
    } else { return; }
    // if (sscanf(line, "ID=0x%08X%n",      &node.ID, &r) == 1)            { line += r; } else return;
    // if (sscanf(line, " Parent=0x%08X%n", &node.ParentNodeId, &r) == 1)  { line += r; if (node.ParentNodeId == 0) return; }
    // if (sscanf(line, " Window=0x%08X%n", &node.ParentWindowId, &r) ==1) { line += r; if (node.ParentWindowId == 0) return; }
    // if (node.ParentNodeId == 0)
    // {
    //     if (sscanf(line, " Pos=%i,%i%n",  &x, &y, &r) == 2)         { line += r; node.Pos = ImVec2ih((c_short)x, y); } else return;
    //     if (sscanf(line, " Size=%i,%i%n", &x, &y, &r) == 2)         { line += r; node.Size = ImVec2ih((c_short)x, y); } else return;
    // }
    // else
    // {
    //     if (sscanf(line, " SizeRef=%i,%i%n", &x, &y, &r) == 2)      { line += r; node.SizeRef = ImVec2ih((c_short)x, y); }
    // }
    // if (sscanf(line, " Split=%c%n", &c, &r) == 1)                   { line += r; if (c == 'X') node.SplitAxis = ImGuiAxis_X; else if (c == 'Y') node.SplitAxis = ImGuiAxis_Y; }
    // if (sscanf(line, " NoResize=%d%n", &x, &r) == 1)                { line += r; if (x != 0) node.Flags |= ImGuiDockNodeFlags_NoResize; }
    // if (sscanf(line, " CentralNode=%d%n", &x, &r) == 1)             { line += r; if (x != 0) node.Flags |= ImGuiDockNodeFlags_CentralNode; }
    // if (sscanf(line, " NoTabBar=%d%n", &x, &r) == 1)                { line += r; if (x != 0) node.Flags |= ImGuiDockNodeFlags_NoTabBar; }
    // if (sscanf(line, " HiddenTabBar=%d%n", &x, &r) == 1)            { line += r; if (x != 0) node.Flags |= ImGuiDockNodeFlags_HiddenTabBar; }
    // if (sscanf(line, " NoWindowMenuButton=%d%n", &x, &r) == 1)      { line += r; if (x != 0) node.Flags |= ImGuiDockNodeFlags_NoWindowMenuButton; }
    // if (sscanf(line, " NoCloseButton=%d%n", &x, &r) == 1)           { line += r; if (x != 0) node.Flags |= ImGuiDockNodeFlags_NoCloseButton; }
    // if (sscanf(line, " Selected=0x%08X%n", &node.SelectedTabId,&r) == 1) { line += r; }
    // if (node.ParentNodeId != 0)
    //     if (ImGuiDockNodeSettings* parent_settings = DockSettingsFindNodeSettings(ctx, node.ParentNodeId))
    //         node.Depth = parent_settings.Depth + 1;
    // ctx.DockContext.NodesSettings.push(node);
    todo!()
}

// static c_void DockSettingsHandler_DockNodeToSettings(ImGuiDockContext* dc, node: *mut ImGuiDockNode, depth: c_int)
pub fn DockSettingsHandler_DockNodeToSettings(dc: *mut ImGuiDockContext, node: *mut ImGuiDockNode, depth: c_int) {
    // ImGuiDockNodeSettings node_settings;
    let mut node_settings = ImGuiDockNodeSettings::default();
    // IM_ASSERT(depth < (1 << (sizeof(node_settings.Depth) << 3)));
    node_settings.ID = node.ID;
    node_settings.ParentNodeId = if node.ParentNode { node.ParentNode.ID } else { 0 };
    node_settings.ParentWindowId = if (node.IsDockSpace() && is_not_null(node.HostWindow) && node.Hostwindow.ParentWindow) { node.Hostwindow.Parentwindow.ID } else { 0 };
    node_settings.SelectedTabId = node.SelectedTabId;
    node_settings.SplitAxis = (if node.IsSplitNode() { node.SplitAxis } else { ImGuiAxis_None });
    node_settings.Depth = depth as c_char;
    node_settings.Flags = (node.LocalFlags & ImGuiDockNodeFlags_SavedFlagsMask_);
    node_settings.Pos = ImVec2ih::new3(&node.Pos);
    node_settings.Size = ImVec2ih::new3(&node.Size);
    node_settings.SizeRef = ImVec2ih::new3(node.SizeRe0f32);
    dc.NodesSettings.push(node_settings);
    if (node.ChildNodes[0]) {
        DockSettingsHandler_DockNodeToSettings(dc, node.ChildNodes[0], depth + 1);
    }
    if (node.ChildNodes[1]) {
        DockSettingsHandler_DockNodeToSettings(dc, node.ChildNodes[1], depth + 1);
    }
}

// static c_void DockSettingsHandler_WriteAll(ctx: *mut ImGuiContext, handler: *mut ImGuiSettingsHandler, buf: *mut ImGuiTextBuffer)
pub unsafe fn DockSettingsHandler_WriteAll(ctx: *mut ImGuiContext, handler: *mut ImGuiSettingsHandler, buf: *mut ImGuiTextBuffer) {
    let g = ctx;
    let dc = &mut ctx.DockContext;
    if !(g.IO.ConfigFlags & ImGuiConfigFlags_DockingEnable) {
        return;
    }

    // Gather settings data
    // (unlike our windows settings, because nodes are always built we can do a full rewrite of the SettingsNode buffer)
    dc.NodesSettings.clear();
    dc.NodesSettings.reserve(dc.Nodes.Data.Size);
    // for (let n: c_int = 0; n < dc.Nodes.Data.Size; n++)
    for n in 0..dc.Nodes.Data.len() {
        if node: *mut ImGuiDockNode = dc.Nodes.Data[n].val_p as *mut ImGuiDockNode {
            if node.IsRootNode() {
                DockSettingsHandler_DockNodeToSettings(dc, node, 0);
            }
        }
    }

    let mut max_depth: c_int = 0;
    // for (let node_n: c_int = 0; node_n < dc.NodesSettings.Size; node_n++)
    for node_n in 0..dc.NodesSettings.len() {
        max_depth = ImMax(dc.NodesSettings[node_n].Depth, max_depth);
    }

    // Write to text buffer
    buf.appendf("[%s][Data]\n", handler.TypeName);
    // for (let node_n: c_int = 0; node_n < dc.NodesSettings.Size; node_n++)
    for node_n in 0..dc.NodesSettings.len() {
        let line_start_pos: c_int = buf.size() as c_int;
        // let line_start_pos;
        let node_settings: *const ImGuiDockNodeSettings = &dc.NodesSettings[node_n];
        buf.appendf("%*s%s%*s", node_settings.Depth * 2, "", if flag_set(node_settings.Flags, ImGuiDockNodeFlags_DockSpace) { "DockSpace" } else { "DockNode " }, (max_depth - node_settings.Depth) * 2, "");  // Text align nodes to facilitate looking at .ini file
        buf.appendf(" ID=0x%08X", node_settings.ID);
        if node_settings.ParentNodeId {
            buf.appendf(" Parent=0x%08X SizeRef=%d,%d", node_settings.ParentNodeId, node_settings.SizeRef.x, node_settings.SizeRef.y);
        } else {
            if node_settings.ParentWindowId {
                buf.appendf(" Window=0x%08X", node_settings.ParentWindowId);
            }
            buf.appendf(" Pos=%d,%d Size=%d,%d", node_settings.Pos.x, node_settings.Pos.y, node_settings.Size.x, node_settings.Size.y);
        }
        if node_settings.SplitAxis != ImGuiAxis_None {
            buf.appendf(" Split=%c", if node_settings.SplitAxis == ImGuiAxis_X { 'X' } else { 'Y' });
        }
        if node_settings.Flags & ImGuiDockNodeFlags_NoResize {
            buf.appendf(" NoResize=1");
        }
        if node_settings.Flags & ImGuiDockNodeFlags_CentralNode {
            buf.appendf(" CentralNode=1");
        }
        if node_settings.Flags & ImGuiDockNodeFlags_NoTabBar {
            buf.appendf(" NoTabBar=1");
        }
        if node_settings.Flags & ImGuiDockNodeFlags_HiddenTabBar {
            buf.appendf(" HiddenTabBar=1");
        }
        if node_settings.Flags & ImGuiDockNodeFlags_NoWindowMenuButton {
            buf.appendf(" NoWindowMenuButton=1");
        }
        if node_settings.Flags & ImGuiDockNodeFlags_NoCloseButton {
            buf.appendf(" NoCloseButton=1");
        }
        if node_settings.SelectedTabId {
            buf.appendf(" Selected=0x%08X", node_settings.SelectedTabId);
        }

// #if IMGUI_DEBUG_INI_SETTINGS
        // [DEBUG] Include comments in the .ini file to ease debugging
        if node: *mut ImGuiDockNode = DockContextFindNodeByID(ctx, node_settings.ID) {
            buf.appendf("%*s", ImMax(2, (line_start_pos + 92) - buf.size()), "");     // Align everything
            if node.IsDockSpace() && node.HostWindow && node.Hostwindow.ParentWindow {
                buf.appendf(" ; in '%s'", node.Hostwindow.Parentwindow.Name);
            }
            // Iterate settings so we can give info about windows that didn't exist during the session.
            let mut contains_window: c_int = 0;
            // for (let mut settings: *mut ImGuiWindowSettings = g.SettingsWindows.begin(); settings != null_mut(); settings = g.SettingsWindows.next_chunk(settings))
            for settings in g.SettingsWindow.iter_mut() {
                if settings.DockId == node_settings.ID {
                    if contains_window == 0 {
                        buf.appendf(" ; contains ");
                    }
                    contains_window += 1;
                    buf.appendf("'%s' ", settings.GetName());
                }
            }
        }
// #endif
        buf.appendf("\n");
    }
    buf.appendf("\n");
}
