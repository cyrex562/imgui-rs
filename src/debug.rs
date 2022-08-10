use std::collections::HashSet;
use crate::color::StyleColor;
use crate::{Context, Viewport, ViewportFlags, window};
use crate::axis::Axis;
use crate::column::OldColumns;
use crate::types::DataAuthority::Window;
use crate::dock::node::{DockNode, DockNodeFlags};
use crate::draw::command::DrawCommand;
use crate::draw::draw_defines::DrawFlags;
use crate::draw::list::{DrawList, DrawListFlags, foreground_draw_list};
use crate::font::Font;
use crate::font::font_atlas::FontAtlas;
use crate::globals::GImGui;
use crate::input::NavLayer;
use crate::orig_imgui_single_file::int;
use crate::rect::Rect;
use crate::stack::StackTool;
use crate::style::{color_u32_from_style_color, pop_style_color, push_style_color};
use crate::tab_bar::TabBar;
use crate::types::{Id32, DataType};
use crate::vectors::vector_2d::Vector2D;
use crate::window::next_window::NextWindowDataFlags;
use crate::window::settings::WindowSettings;
use crate::window::WindowFlags;

// void DebugRenderViewportThumbnail(ImDrawList* draw_list, ImGuiViewportP* viewport, const Rect& bb)
pub fn debug_render_viewport_thumbnail(g: &mut Context, draw_list: &mut DrawList, viewport: &mut Viewport, bb: &Rect)
{
    // ImGuiContext& g = *GImGui;
    let window = g.current_window_mut();

    Vector2D scale = bb.GetSize() / viewport.size;
    Vector2D off = bb.min - viewport.pos * scale;
    let alpha_mul =  if(viewport.flags & ViewportFlags::Minimized) { 0.30 }else{ 1.00};
    window.draw_list->AddRectFilled(bb.min, bb.max, color_u32_from_style_color(StyleColor::Border, alpha_mul * 0.40));
    for (int i = 0; i != g.windows.len(); i += 1)
    {
        Window* thumb_window = g.windows[i];
        if (!thumb_window.was_active || (thumb_window.flags & WindowFlags::ChildWindow))
            continue;
        if (thumb_window.viewport != viewport)
            continue;

        Rect thumb_r = thumb_window.rect();
        Rect title_r = thumb_window.title_bar_rect();
        thumb_r = Rect(f32::floor(off + thumb_r.min * scale), f32::floor(off +  thumb_r.max * scale));
        title_r = Rect(f32::floor(off + title_r.min * scale), f32::floor(off +  Vector2D::new(title_r.max.x, title_r.min.y) * scale) + Vector2D::new(0,5)); // Exaggerate title bar height
        thumb_r.ClipWithFull(bb);
        title_r.ClipWithFull(bb);
        let window_is_focused = (g.nav_window && thumb_window.root_window_for_title_bar_highlight == g.nav_window->root_window_for_title_bar_highlight);
        window.draw_list->AddRectFilled(thumb_r.min, thumb_r.max, color_u32_from_style_color(StyleColor::WindowBg, alpha_mul));
        window.draw_list->AddRectFilled(title_r.min, title_r.max, color_u32_from_style_color(window_is_focused ? StyleColor::TitleBgActive : StyleColor::TitleBg, alpha_mul));
        window.draw_list->AddRect(thumb_r.min, thumb_r.max, color_u32_from_style_color(StyleColor::Border, alpha_mul));
        window.draw_list->AddText(g.font, g.font_size * 1.0, title_r.min, color_u32_from_style_color(StyleColor::Text, alpha_mul), thumb_window.name, find_rendered_text_end(thumb_window.name));
    }
    draw_list->AddRect(bb.min, bb.max, color_u32_from_style_color(StyleColor::Border, alpha_mul));
}

// static void RenderViewportsThumbnails()
pub fn render_viewports_thumbnails(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    let window = g.current_window_mut();

    // We don't display full monitor bounds (we could, but it often looks awkward), instead we display just enough to cover all of our viewports.
    let SCALE =  1.0 / 8.0;
    Rect bb_full(f32::MAX, f32::MAX, -f32::MAX, -f32::MAX);
    for (int n = 0; n < g.viewports.size; n += 1)
        bb_full.Add(g.viewports[n]->get_main_rect());
    Vector2D p = window.dc.cursor_pos;
    Vector2D off = p - bb_full.min * SCALE;
    for (int n = 0; n < g.viewports.size; n += 1)
    {
        ViewportP* viewport = g.viewports[n];
        Rect viewport_draw_bb(off + (viewport.pos) * SCALE, off + (viewport.pos + viewport.size) * SCALE);
        DebugRenderViewportThumbnail(window.draw_list, viewport, viewport_draw_bb);
    }
    Dummy(bb_full.GetSize() * SCALE);
}

// static int  ViewportComparerByFrontMostStampCount(const void* lhs, const void* rhs)
pub fn viewport_comparer_by_front_most_stamp_count(g: &mut Context, lhs: &Vec<u8>, rhs: &Vec<u8>) -> i32
{
    const ViewportP* a = *(const ViewportP* const*)lhs;
    const ViewportP* b = *(const ViewportP* const*)rhs;
    return b->last_frontmost_stamp_count - a->last_frontmost_stamp_count;
}

// Helper tool to diagnose between text encoding issues and font loading issues. Pass your UTF-8 string and verify that there are correct.
// void DebugTextEncoding(const char* str)
pub fn debug_text_encoding(g: &mut Context, text: &str)
{
    text("Text: \"%s\"", str);
    if (!BeginTable("list", 4, ImGuiTableFlags_Borders | ImGuiTableFlags_RowBg | ImGuiTableFlags_SizingFixedFit))
        return;
    TableSetupColumn("Offset");
    TableSetupColumn("UTF-8");
    TableSetupColumn("Glyph");
    TableSetupColumn("codepoint");
    TableHeadersRow();
    for (const char* p = str; *p != 0; )
    {
        unsigned int c;
        let c_utf8_len = text_char_from_utf8(&c, p, None);
        TableNextColumn();
        text("%d", (p - str));
        TableNextColumn();
        for (int byte_index = 0; byte_index < c_utf8_len; byte_index += 1)
        {
            if (byte_index > 0)
                same_line();
            text("0x%02X", (unsigned char)p[byte_index]);
        }
        TableNextColumn();
        if (GetFont()->FindGlyphNoFallback((ImWchar)c))
            TextUnformatted(p, p + c_utf8_len);
        else
            TextUnformatted((c == IM_UNICODE_CODEPOINT_INVALID) ? "[invalid]" : "[missing]");
        TableNextColumn();
        text("U+%04X", c);
        p += c_utf8_len;
    }
    EndTable();
}

// Avoid naming collision with imgui_demo.cpp's HelpMarker() for unity builds.
// static void MetricsHelpMarker(const char* desc)
pub fn metrics_help_marker(g: &mut Context, desc: &str)
{
    TextDisabled("(?)");
    if (is_item_hovered())
    {
        BeginTooltip();
        PushTextWrapPos(GetFontSize() * 35.0);
        TextUnformatted(desc);
        PopTextWrapPos();
        EndTooltip();
    }
}

// [DEBUG] List fonts in a font atlas and display its texture
// void ShowFontAtlas(ImFontAtlas* atlas)
pub fn show_font_atlas(g: &mut Context, atlas: &mut FontAtlas)
{
    for (int i = 0; i < atlas->Fonts.size; i += 1)
    {
        ImFont* font = atlas->Fonts[i];
        push_id(font);
        DebugNodeFont(font);
        pop_id();
    }
    if (TreeNode("Atlas texture", "Atlas texture (%dx%d pixels)", atlas->TexWidth, atlas->TexHeight))
    {
        Vector4D tint_col = Vector4D(1.0, 1.0, 1.0, 1.0);
        Vector4D border_col = Vector4D(1.0, 1.0, 1.0, 0.5);
        Image(atlas->TexID, Vector2D::new(atlas->TexWidth, atlas->TexHeight), Vector2D::new(0.0, 0.0), Vector2D::new(1.0, 1.0), tint_col, border_col);
        TreePop();
    }
}

// void ShowMetricsWindow(bool* p_open)
pub fn show_metrics_window(g: &mut Context, p_open: &mut bool)
{
    // ImGuiContext& g = *GImGui;
    ImGuiIO& io = g.io;
    ImGuiMetricsConfig* cfg = &g.DebugMetricsConfig;
    if (cfg->ShowDebugLog)
        ShowDebugLogWindow(&cfg->ShowDebugLog);
    if (cfg->ShowStackTool)
        ShowStackToolWindow(&cfg->ShowStackTool);

    if (!begin("Dear ImGui Metrics/Debugger", p_open) || GetCurrentWindow()->BeginCount > 1)
    {
        end();
        return;
    }

    // Basic info
    text("Dear ImGui %s", GetVersion());
    text("Application average %.3 ms/frame (%.1 FPS)", 1000.0 / io.frame_rate, io.frame_rate);
    text("%d vertices, %d indices (%d triangles)", io.metrics_render_vertices, io.metrics_render_indices, io.metrics_render_indices / 3);
    text("%d visible windows, %d active allocations", io.metrics_render_windows, io.MetricsActiveAllocations);
    //same_line(); if (SmallButton("GC")) { g.gc_compact_all = true; }

    Separator();

    // Debugging enums
    enum { WRT_OuterRect, WRT_OuterRectClipped, WRT_InnerRect, WRT_InnerClipRect, WRT_WorkRect, WRT_Content, WRT_ContentIdeal, WRT_ContentRegionRect, WRT_Count }; // windows rect Type
    const char* wrt_rects_names[WRT_Count] = { "outer_rect", "outer_rect_clipped", "inner_rect", "inner_clip_rect", "work_rect", "Content", "ContentIdeal", "content_region_rect" };
    enum { TRT_OuterRect, TRT_InnerRect, TRT_WorkRect, TRT_HostClipRect, TRT_InnerClipRect, TRT_BackgroundClipRect, TRT_ColumnsRect, TRT_ColumnsWorkRect, TRT_ColumnsClipRect, TRT_ColumnsContentHeadersUsed, TRT_ColumnsContentHeadersIdeal, TRT_ColumnsContentFrozen, TRT_ColumnsContentUnfrozen, TRT_Count }; // tables rect Type
    const char* trt_rects_names[TRT_Count] = { "outer_rect", "inner_rect", "work_rect", "host_clip_rect", "inner_clip_rect", "BackgroundClipRect", "ColumnsRect", "ColumnsWorkRect", "ColumnsClipRect", "ColumnsContentHeadersUsed", "ColumnsContentHeadersIdeal", "ColumnsContentFrozen", "ColumnsContentUnfrozen" };
    if (cfg->ShowWindowsRectsType < 0)
        cfg->ShowWindowsRectsType = WRT_WorkRect;
    if (cfg->ShowTablesRectsType < 0)
        cfg->ShowTablesRectsType = TRT_WorkRect;

    struct Funcs
    {
        static Rect GetTableRect(ImGuiTable* table, int rect_type, int n)
        {
            ImGuiTableInstanceData* table_instance = TableGetInstanceData(table, table->InstanceCurrent); // Always using last submitted instance
            if (rect_type == TRT_OuterRect)                     { return table->OuterRect; }
            else if (rect_type == TRT_InnerRect)                { return table->inner_rect; }
            else if (rect_type == TRT_WorkRect)                 { return table->WorkRect; }
            else if (rect_type == TRT_HostClipRect)             { return table->HostClipRect; }
            else if (rect_type == TRT_InnerClipRect)            { return table->InnerClipRect; }
            else if (rect_type == TRT_BackgroundClipRect)       { return table->BgClipRect; }
            else if (rect_type == TRT_ColumnsRect)              { ImGuiTableColumn* c = &table->Columns[n]; return Rect(c->MinX, table->InnerClipRect.min.y, c->MaxX, table->InnerClipRect.min.y + table_instance->LastOuterHeight); }
            else if (rect_type == TRT_ColumnsWorkRect)          { ImGuiTableColumn* c = &table->Columns[n]; return Rect(c->WorkMinX, table->WorkRect.min.y, c->WorkMaxX, table->WorkRect.max.y); }
            else if (rect_type == TRT_ColumnsClipRect)          { ImGuiTableColumn* c = &table->Columns[n]; return c->ClipRect; }
            else if (rect_type == TRT_ColumnsContentHeadersUsed){ ImGuiTableColumn* c = &table->Columns[n]; return Rect(c->WorkMinX, table->InnerClipRect.min.y, c->ContentMaxXHeadersUsed, table->InnerClipRect.min.y + table_instance->LastFirstRowHeight); } // Note: y1/y2 not always accurate
            else if (rect_type == TRT_ColumnsContentHeadersIdeal){ImGuiTableColumn* c = &table->Columns[n]; return Rect(c->WorkMinX, table->InnerClipRect.min.y, c->ContentMaxXHeadersIdeal, table->InnerClipRect.min.y + table_instance->LastFirstRowHeight); }
            else if (rect_type == TRT_ColumnsContentFrozen)     { ImGuiTableColumn* c = &table->Columns[n]; return Rect(c->WorkMinX, table->InnerClipRect.min.y, c->ContentMaxXFrozen, table->InnerClipRect.min.y + table_instance->LastFirstRowHeight); }
            else if (rect_type == TRT_ColumnsContentUnfrozen)   { ImGuiTableColumn* c = &table->Columns[n]; return Rect(c->WorkMinX, table->InnerClipRect.min.y + table_instance->LastFirstRowHeight, c->ContentMaxXUnfrozen, table->InnerClipRect.max.y); }
            // IM_ASSERT(0);
            return Rect();
        }

        static Rect GetWindowRect(Window* window, int rect_type)
        {
            if (rect_type == WRT_OuterRect)                 { return window.rect(); }
            else if (rect_type == WRT_OuterRectClipped)     { return window.OuterRectClipped; }
            else if (rect_type == WRT_InnerRect)            { return window.inner_rect; }
            else if (rect_type == WRT_InnerClipRect)        { return window.InnerClipRect; }
            else if (rect_type == WRT_WorkRect)             { return window.work_rect; }
            else if (rect_type == WRT_Content)       { Vector2D min = window.inner_rect.min - window.scroll + window.window_padding; return Rect(min, min + window.ContentSize); }
            else if (rect_type == WRT_ContentIdeal)         { Vector2D min = window.inner_rect.min - window.scroll + window.window_padding; return Rect(min, min + window.ContentSizeIdeal); }
            else if (rect_type == WRT_ContentRegionRect)    { return window.content_region_rect; }
            // IM_ASSERT(0);
            return Rect();
        }
    };

    // Tools
    if (TreeNode("Tools"))
    {
        bool show_encoding_viewer = TreeNode("UTF-8 Encoding viewer");
        same_line();
        MetricsHelpMarker("You can also call DebugTextEncoding() from your code with a given string to test that your UTF-8 encoding settings are correct.");
        if (show_encoding_viewer)
        {
            static char buf[100] = "";
            SetNextItemWidth(-FLT_MIN);
            InputText("##Text", buf, IM_ARRAYSIZE(buf));
            if (buf[0] != 0)
                DebugTextEncoding(buf);
            TreePop();
        }

        // The Item Picker tool is super useful to visually select an item and break into the call-stack of where it was submitted.
        if (Checkbox("Show Item Picker", &g.debug_item_picker_active) && g.debug_item_picker_active)
            DebugStartItemPicker();
        same_line();
        MetricsHelpMarker("Will call the IM_DEBUG_BREAK() macro to break in debugger.\nWarning: If you don't have a debugger attached, this will probably crash.");

        // Stack Tool is your best friend!
        Checkbox("Show Debug Log", &cfg->ShowDebugLog);
        same_line();
        MetricsHelpMarker("You can also call ShowDebugLogWindow() from your code.");

        // Stack Tool is your best friend!
        Checkbox("Show Stack Tool", &cfg->ShowStackTool);
        same_line();
        MetricsHelpMarker("You can also call ShowStackToolWindow() from your code.");

        Checkbox("Show windows begin order", &cfg->ShowWindowsBeginOrder);
        Checkbox("Show windows rectangles", &cfg->ShowWindowsRects);
        same_line();
        SetNextItemWidth(GetFontSize() * 12);
        cfg->ShowWindowsRects |= Combo("##show_windows_rect_type", &cfg->ShowWindowsRectsType, wrt_rects_names, WRT_Count, WRT_Count);
        if (cfg->ShowWindowsRects && g.nav_window != None)
        {
            BulletText("'%s':", g.nav_window->Name);
            Indent();
            for (int rect_n = 0; rect_n < WRT_Count; rect_n += 1)
            {
                Rect r = Funcs::GetWindowRect(g.nav_window, rect_n);
                text("(%6.1,%6.1) (%6.1,%6.1) size (%6.1,%6.1) %s", r.min.x, r.min.y, r.max.x, r.max.y, r.get_width(), r.get_height(), wrt_rects_names[rect_n]);
            }
            Unindent();
        }

        Checkbox("Show tables rectangles", &cfg->ShowTablesRects);
        same_line();
        SetNextItemWidth(GetFontSize() * 12);
        cfg->ShowTablesRects |= Combo("##show_table_rects_type", &cfg->ShowTablesRectsType, trt_rects_names, TRT_Count, TRT_Count);
        if (cfg->ShowTablesRects && g.nav_window != None)
        {
            for (int table_n = 0; table_n < g.tables.GetMapSize(); table_n += 1)
            {
                ImGuiTable* table = g.tables.TryGetMapData(table_n);
                if (table == None || table->last_frame_active < g.frame_count - 1 || (table->OuterWindow != g.nav_window && table->InnerWindow != g.nav_window))
                    continue;

                BulletText("Table 0x%08X (%d columns, in '%s')", table->ID, table->ColumnsCount, table->OuterWindow->Name);
                if (is_item_hovered())
                    foreground_draw_list()->AddRect(table->OuterRect.min - Vector2D::new(1, 1), table->OuterRect.max + Vector2D::new(1, 1), IM_COL32(255, 255, 0, 255), 0.0, 0, 2.0);
                Indent();
                char buf[128];
                for (int rect_n = 0; rect_n < TRT_Count; rect_n += 1)
                {
                    if (rect_n >= TRT_ColumnsRect)
                    {
                        if (rect_n != TRT_ColumnsRect && rect_n != TRT_ColumnsClipRect)
                            continue;
                        for (int column_n = 0; column_n < table->ColumnsCount; column_n += 1)
                        {
                            Rect r = Funcs::GetTableRect(table, rect_n, column_n);
                            ImFormatString(buf, IM_ARRAYSIZE(buf), "(%6.1,%6.1) (%6.1,%6.1) size (%6.1,%6.1) col %d %s", r.min.x, r.min.y, r.max.x, r.max.y, r.get_width(), r.get_height(), column_n, trt_rects_names[rect_n]);
                            selectable(buf);
                            if (is_item_hovered())
                                foreground_draw_list()->AddRect(r.min - Vector2D::new(1, 1), r.max + Vector2D::new(1, 1), IM_COL32(255, 255, 0, 255), 0.0, 0, 2.0);
                        }
                    }
                    else
                    {
                        Rect r = Funcs::GetTableRect(table, rect_n, -1);
                        ImFormatString(buf, IM_ARRAYSIZE(buf), "(%6.1,%6.1) (%6.1,%6.1) size (%6.1,%6.1) %s", r.min.x, r.min.y, r.max.x, r.max.y, r.get_width(), r.get_height(), trt_rects_names[rect_n]);
                        selectable(buf);
                        if (is_item_hovered())
                            foreground_draw_list()->AddRect(r.min - Vector2D::new(1, 1), r.max + Vector2D::new(1, 1), IM_COL32(255, 255, 0, 255), 0.0, 0, 2.0);
                    }
                }
                Unindent();
            }
        }

        TreePop();
    }

    // windows
    if (TreeNode("windows", "windows (%d)", g.windows.len()))
    {
        //SetNextItemOpen(true, ImGuiCond_Once);
        DebugNodeWindowsList(&g.windows, "By display order");
        DebugNodeWindowsList(&g.windows_focus_order, "By focus order (root windows)");
        if (TreeNode("By submission order (begin stack)"))
        {
            // Here we display windows in their submitted order/hierarchy, however note that the Begin stack doesn't constitute a Parent<>Child relationship!
            ImVector<Window*>& temp_buffer = g.windows_temp_sort_buffer;
            temp_buffer.resize(0);
            for (int i = 0; i < g.windows.len(); i += 1)
                if (g.windows[i]->last_frame_active + 1 >= g.frame_count)
                    temp_buffer.push_back(g.windows[i]);
            struct Func { static int  WindowComparerByBeginOrder(const void* lhs, const void* rhs) { return ((*(const Window* const *)lhs)->begin_order_within_context - (*(const Window* const*)rhs)->begin_order_within_context); } };
            ImQsort(temp_buffer.data, temp_buffer.size, sizeof(Window*), Func::WindowComparerByBeginOrder);
            DebugNodeWindowsListByBeginStackParent(temp_buffer.data, temp_buffer.size, None);
            TreePop();
        }

        TreePop();
    }

    // DrawLists
    int drawlist_count = 0;
    for (int viewport_i = 0; viewport_i < g.viewports.size; viewport_i += 1)
        drawlist_count += g.viewports[viewport_i].draw_data_builder.GetDrawListCount();
    if (TreeNode("DrawLists", "DrawLists (%d)", drawlist_count))
    {
        Checkbox("Show ImDrawCmd mesh when hovering", &cfg->ShowDrawCmdMesh);
        Checkbox("Show ImDrawCmd bounding boxes when hovering", &cfg->ShowDrawCmdBoundingBoxes);
        for (int viewport_i = 0; viewport_i < g.viewports.size; viewport_i += 1)
        {
            ViewportP* viewport = g.viewports[viewport_i];
            bool viewport_has_drawlist = false;
            for (int layer_i = 0; layer_i < IM_ARRAYSIZE(viewport.draw_data_builder.layers); layer_i += 1)
                for (int draw_list_i = 0; draw_list_i < viewport.draw_data_builder.layers[layer_i].size; draw_list_i += 1)
                {
                    if (!viewport_has_drawlist)
                        text("active DrawLists in viewport #%d, id: 0x%08X", viewport->Idx, viewport->ID);
                    viewport_has_drawlist = true;
                    DebugNodeDrawList(None, viewport, viewport.draw_data_builder.layers[layer_i][draw_list_i], "draw_list");
                }
        }
        TreePop();
    }

    // viewports
    if (TreeNode("viewports", "viewports (%d)", g.viewports.size))
    {
        Indent(GetTreeNodeToLabelSpacing());
        RenderViewportsThumbnails();
        Unindent(GetTreeNodeToLabelSpacing());

        bool open = TreeNode("Monitors", "Monitors (%d)", g.platform_io.monitors.size);
        same_line();
        MetricsHelpMarker("Dear ImGui uses monitor data:\n- to query DPI settings on a per monitor basis\n- to position popup/tooltips so they don't straddle monitors.");
        if (open)
        {
            for (int i = 0; i < g.platform_io.monitors.size; i += 1)
            {
                const platform_monitor& mon = g.platform_io.monitors[i];
                BulletText("Monitor #%d: DPI %.0%%\n MainMin (%.0,%.0), MainMax (%.0,%.0), main_size (%.0,%.0)\n WorkMin (%.0,%.0), WorkMax (%.0,%.0), work_size (%.0,%.0)",
                    i, mon.DpiScale * 100.0,
                    mon.MainPos.x, mon.MainPos.y, mon.MainPos.x + mon.MainSize.x, mon.MainPos.y + mon.MainSize.y, mon.MainSize.x, mon.MainSize.y,
                    mon.WorkPos.x, mon.WorkPos.y, mon.WorkPos.x + mon.work_size.x, mon.WorkPos.y + mon.work_size.y, mon.work_size.x, mon.work_size.y);
            }
            TreePop();
        }

        BulletText("mouse_viewport: 0x%08X (UserHovered 0x%08X, LastHovered 0x%08X)", g.mouse_viewport ? g.mouse_viewport->ID : 0, g.io.MouseHoveredViewport, g.mouse_last_hovered_viewport ? g.mouse_last_hovered_viewport->ID : 0);
        if (TreeNode("Inferred Z order (front-to-back)"))
        {
            static ImVector<ViewportP*> viewports;
            viewports.resize(g.viewports.size);
            memcpy(viewports.data, g.viewports.data, g.viewports.size_in_bytes());
            if (viewports.size > 1)
                ImQsort(viewports.data, viewports.size, sizeof(Viewport*), ViewportComparerByFrontMostStampCount);
            for (int i = 0; i < viewports.size; i += 1)
                BulletText("viewport #%d, id: 0x%08X, FrontMostStampCount = %08d, window: \"%s\"", viewports[i]->Idx, viewports[i]->ID, viewports[i]->last_frontmost_stamp_count, viewports[i]->Window ? viewports[i]->Window->Name : "N/A");
            TreePop();
        }

        for (int i = 0; i < g.viewports.size; i += 1)
            DebugNodeViewport(g.viewports[i]);
        TreePop();
    }

    // Details for Popups
    if (TreeNode("Popups", "Popups (%d)", g.open_popup_stack.size))
    {
        for (int i = 0; i < g.open_popup_stack.size; i += 1)
        {
            Window* window = g.open_popup_stack[i].Window;
            BulletText("PopupID: %08x, window: '%s'%s%s", g.open_popup_stack[i].popup_id, window ? window.name : "None", window && (window.flags & WindowFlags::ChildWindow) ? " ChildWindow" : "", window && (window.flags & WindowFlags::ChildMenu) ? " ChildMenu" : "");
        }
        TreePop();
    }

    // Details for tab_bars
    if (TreeNode("tab_bars", "Tab Bars (%d)", g.tab_bars.GetAliveCount()))
    {
        for (int n = 0; n < g.tab_bars.GetMapSize(); n += 1)
            if (ImGuiTabBar* tab_bar = g.tab_bars.TryGetMapData(n))
            {
                push_id(tab_bar);
                DebugNodeTabBar(tab_bar, "tab_bar");
                pop_id();
            }
        TreePop();
    }

    // Details for tables
    if (TreeNode("tables", "tables (%d)", g.tables.GetAliveCount()))
    {
        for (int n = 0; n < g.tables.GetMapSize(); n += 1)
            if (ImGuiTable* table = g.tables.TryGetMapData(n))
                DebugNodeTable(table);
        TreePop();
    }

    // Details for fonts
    ImFontAtlas* atlas = g.io.fonts;
    if (TreeNode("fonts", "fonts (%d)", atlas->Fonts.size))
    {
        ShowFontAtlas(atlas);
        TreePop();
    }

    // Details for InputText
    if (TreeNode("InputText"))
    {
        DebugNodeInputTextState(&g.input_text_state);
        TreePop();
    }

    // Details for Docking

    if (TreeNode("Docking"))
    {
        static bool root_nodes_only = true;
        ImGuiDockContext* dc = &g.dock_context;
        Checkbox("List root nodes", &root_nodes_only);
        Checkbox("Ctrl shows window dock info", &cfg->ShowDockingNodes);
        if (SmallButton("clear nodes")) { DockContextClearNodes(&g, 0, true); }
        same_line();
        if (SmallButton("Rebuild all")) { dc->WantFullRebuild = true; }
        for (int n = 0; n < dc->Nodes.data.size; n += 1)
            if (ImGuiDockNode* node = (ImGuiDockNode*)dc->Nodes.data[n].val_p)
                if (!root_nodes_only || node->IsRootNode())
                    DebugNodeDockNode(node, "Node");
        TreePop();
    }

        if (TreeNode("SettingsDocking", "Settings packed data: Docking"))
        {
            ImGuiDockContext* dc = &g.dock_context;
            text("In settings_windows:");
            for (WindowSettings* settings = g.settings_windows.begin(); settings != None; settings = g.settings_windows.next_chunk(settings))
                if (settings.dock_id != 0)
                    BulletText("window '%s' -> dock_id %08X", settings->GetName(), settings.dock_id);
            text("In SettingsNodes:");
            for (int n = 0; n < dc->nodes_settings.size; n += 1)
            {
                ImGuiDockNodeSettings* settings = &dc->nodes_settings[n];
                const char* selected_tab_name = None;
                if (settings->SelectedTabId)
                {
                    if (Window* window = find_window_by_id(settings->SelectedTabId))
                        selected_tab_name = window.name;
                    else if (WindowSettings* window_settings = find_window_settings(settings->SelectedTabId))
                        selected_tab_name = window_settings->GetName();
                }
                BulletText("Node %08X, Parent %08X, SelectedTab %08X ('%s')", settings->ID, settings->parent_node_id, settings->SelectedTabId, selected_tab_name ? selected_tab_name : settings->SelectedTabId ? "N/A" : "");
            }
            TreePop();
        }


        if (TreeNode("settings_ini_data", "Settings unpacked data (.ini): %d bytes", g.SettingsIniData.size()))
        {
            InputTextMultiline("##Ini", (char*)(void*)g.SettingsIniData.c_str(), g.SettingsIniData.Buf.size, Vector2D::new(-FLT_MIN, GetTextLineHeight() * 20), ImGuiInputTextFlags_ReadOnly);
            TreePop();
        }
        TreePop();
    }

// Misc Details
    if (TreeNode("Internal state"))
    {
        text("WINDOWING");
        Indent();
        text("hovered_window: '%s'", g.hovered_window ? g.hovered_window->Name : "None");
        text("hovered_window->Root: '%s'", g.hovered_window ? g.hovered_window->root_window_dock_tree->Name : "None");
        text("hovered_window_under_moving_window: '%s'", g.hovered_window_under_moving_window ? g.hovered_window_under_moving_window->Name : "None");
        text("hovered_dock_node: 0x%08X", g.hovered_dock_node ? g.hovered_dock_node->ID : 0);
        text("moving_window: '%s'", g.moving_window ? g.moving_window->Name : "None");
        text("mouse_viewport: 0x%08X (UserHovered 0x%08X, LastHovered 0x%08X)", g.mouse_viewport->ID, g.io.MouseHoveredViewport, g.mouse_last_hovered_viewport ? g.mouse_last_hovered_viewport->ID : 0);
        Unindent();

        text("ITEMS");
        Indent();
        text("active_id: 0x%08X/0x%08X (%.2 sec), AllowOverlap: %d, Source: %s", g.active_id, g.active_id_previous_frame, g.active_id_timer, g.active_id_allow_overlap, GetInputSourceName(g.active_id_source));
        text("active_id_window: '%s'", g.active_id_window ? g.active_id_window->Name : "None");

        int active_id_using_key_input_count = 0;
        for (int n = ImGuiKey_NamedKey_BEGIN; n < ImGuiKey_NamedKey_END; n += 1)
            active_id_using_key_input_count += if g.active_id_using_key_input_mask[n] { 1 }else{ 0};
        text("ActiveIdUsing: Wheel: %d, NavDirMask: %x, NavInputMask: %x, KeyInputMask: %d key(s)", g.active_id_using_mouse_wheel, g.active_id_using_nav_dir_mask, g.active_id_using_nav_input_mask, active_id_using_key_input_count);
        text("hovered_id: 0x%08X (%.2 sec), AllowOverlap: %d", g.hovered_id_previous_frame, g.hovered_id_timer, g.hovered_id_allow_overlap); // Not displaying g.hovered_id as it is update mid-frame
        text("DragDrop: %d, source_id = 0x%08X, Payload \"%s\" (%d bytes)", g.drag_drop_active, g.drag_drop_payload.source_id, g.drag_drop_payload.dataType, g.drag_drop_payload.dataSize);
        Unindent();

        text("NAV,FOCUS");
        Indent();
        text("nav_window: '%s'", g.nav_window ? g.nav_window->Name : "None");
        text("nav_id: 0x%08X, nav_layer: %d", g.nav_id, g.nav_layer);
        text("nav_input_source: %s", GetInputSourceName(g.nav_input_source));
        text("nav_active: %d, nav_visible: %d", g.io.nav_active, g.io.NavVisible);
        text("nav_activate_id/DownId/PressedId/InputId: %08X/%08X/%08X/%08X", g.nav_activate_id, g.NavActivateDownId, g.NavActivatePressedId, g.NavActivateInputId);
        text("nav_activate_flags: %04X", g.NavActivateFlags);
        text("NavDisableHighlight: %d, nav_disable_mouse_hover: %d", g.nav_disable_highlight, g.nav_disable_mouse_hover);
        text("nav_focus_scope_id = 0x%08X", g.nav_focus_spope_id);
        text("nav_windowing_target: '%s'", g.nav_windowing_target ? g.nav_windowing_target->Name : "None");
        Unindent();

        TreePop();
    }

    // Overlay: Display windows Rectangles and Begin Order
    if (cfg->ShowWindowsRects || cfg->ShowWindowsBeginOrder)
    {
        for (int n = 0; n < g.windows.len(); n += 1)
        {
            Window* window = g.windows[n];
            if (!window.was_active)
                continue;
            ImDrawList* draw_list = get_foreground_draw_list(window);
            if (cfg->ShowWindowsRects)
            {
                Rect r = Funcs::GetWindowRect(window, cfg->ShowWindowsRectsType);
                draw_list->AddRect(r.min, r.max, IM_COL32(255, 0, 128, 255));
            }
            if (cfg->ShowWindowsBeginOrder && !(window.flags & WindowFlags::ChildWindow))
            {
                char buf[32];
                ImFormatString(buf, IM_ARRAYSIZE(buf), "%d", window.begin_order_within_context);
                let font_size =  GetFontSize();
                draw_list->AddRectFilled(window.pos, window.pos + Vector2D::new(font_size, font_size), IM_COL32(200, 100, 100, 255));
                draw_list->AddText(window.pos, IM_COL32(255, 255, 255, 255), buf);
            }
        }
    }

    // Overlay: Display tables Rectangles
    if (cfg->ShowTablesRects)
    {
        for (int table_n = 0; table_n < g.tables.GetMapSize(); table_n += 1)
        {
            ImGuiTable* table = g.tables.TryGetMapData(table_n);
            if (table == None || table->last_frame_active < g.frame_count - 1)
                continue;
            ImDrawList* draw_list = get_foreground_draw_list(table->OuterWindow);
            if (cfg->ShowTablesRectsType >= TRT_ColumnsRect)
            {
                for (int column_n = 0; column_n < table->ColumnsCount; column_n += 1)
                {
                    Rect r = Funcs::GetTableRect(table, cfg->ShowTablesRectsType, column_n);
                    ImU32 col = if (table->HoveredColumnBody == column_n) { IM_COL32(255, 255, 128, 255) }else{ IM_COL32(255, 0, 128, 255)};
                    let thickness =  if (table->HoveredColumnBody == column_n) { 3.0 }else{ 1.0};
                    draw_list->AddRect(r.min, r.max, col, 0.0, 0, thickness);
                }
            }
            else
            {
                Rect r = Funcs::GetTableRect(table, cfg->ShowTablesRectsType, -1);
                draw_list->AddRect(r.min, r.max, IM_COL32(255, 0, 128, 255));
            }
        }
    }

// #ifdef IMGUI_HAS_DOCK
    // Overlay: Display Docking info
    if (cfg->ShowDockingNodes && g.io.key_ctrl && g.hovered_dock_node)
    {
        char buf[64] = "";
        char* p = buf;
        ImGuiDockNode* node = g.hovered_dock_node;
        ImDrawList* overlay_draw_list = if node->HostWindow { get_foreground_draw_list(node->HostWindow) }else{ get_foreground_draw_list(get_main_viewport())};
        p += ImFormatString(p, buf + IM_ARRAYSIZE(buf) - p, "dock_id: %x%s\n", node->ID, node->is_central_node() ? " *central_node*" : "");
        p += ImFormatString(p, buf + IM_ARRAYSIZE(buf) - p, "window_class: %08X\n", node->WindowClassclass_id);
        p += ImFormatString(p, buf + IM_ARRAYSIZE(buf) - p, "size: (%.0, %.0)\n", node.size.x, node.size.y);
        p += ImFormatString(p, buf + IM_ARRAYSIZE(buf) - p, "size_ref: (%.0, %.0)\n", node.size_ref.x, node.size_ref.y);
        int depth = dock_node_get_depth(node);
        overlay_draw_list->AddRect(node.pos + Vector2D::new(3, 3) * depth, node.pos + node.size - Vector2D::new(3, 3) * depth, IM_COL32(200, 100, 100, 255));
        Vector2D pos = node.pos + Vector2D::new(3, 3) * depth;
        overlay_draw_list->AddRectFilled(pos - Vector2D::new(1, 1), pos + calc_text_size(buf) + Vector2D::new(1, 1), IM_COL32(200, 100, 100, 255));
        overlay_draw_list->AddText(None, 0.0, pos, IM_COL32(255, 255, 255, 255), buf);
    }
 // #ifdef IMGUI_HAS_DOCK

    end();
}

// [DEBUG] Display contents of columns
// void DebugNodeColumns(ImGuiOldColumns* columns)
pub fn debug_node_columns(g: &mut Context, columns: &mut OldColumns)
{
    if (!TreeNode((void*)(uintptr_t)columns->ID, "columns Id: 0x%08X, Count: %d, flags: 0x%04X", columns->ID, columns->Count, columns.flags))
        return;
    BulletText("width: %.1 (min_x: %.1, max_x: %.1)", columns->OffMaxX - columns->OffMinX, columns->OffMinX, columns->OffMaxX);
    for (int column_n = 0; column_n < columns->Columns.size; column_n += 1)
        BulletText("column %02d: offset_norm %.3 (= %.1 px)", column_n, columns->Columns[column_n].OffsetNorm, GetColumnOffsetFromNorm(columns, columns->Columns[column_n].OffsetNorm));
    TreePop();
}

// static void DebugNodeDockNodeFlags(ImGuiDockNodeFlags* p_flags, const char* label, bool enabled)
pub fn debug_node_dock_node_flags(g: &mut Context, p_flags: &HashSet<DockNodeFlags>, label: &str, enabled: bool)
{
    using namespace ImGui;
    push_id(label);
    push_style_var(StyleVar::frame_padding, Vector2D::new(0.0, 0.0));
    text("%s:", label);
    if (!enabled)
        BeginDisabled();
    CheckboxFlags("NoSplit", p_flags, DockNodeFlags::NoSplit);
    CheckboxFlags("NoResize", p_flags, DockNodeFlags::NoResize);
    CheckboxFlags("NoResizeX", p_flags, DockNodeFlags::NoResizeX);
    CheckboxFlags("NoResizeY",p_flags, DockNodeFlags::NoResizeY);
    CheckboxFlags("NoTabBar", p_flags, DockNodeFlags::NoTabBar);
    CheckboxFlags("HiddenTabBar", p_flags, DockNodeFlags::HiddenTabBar);
    CheckboxFlags("NoWindowMenuButton", p_flags, DockNodeFlags::NoWindowMenuButton);
    CheckboxFlags("NoCloseButton", p_flags, DockNodeFlags::NoCloseButton);
    CheckboxFlags("NoDocking", p_flags, DockNodeFlags::NoDocking);
    CheckboxFlags("NoDockingSplitMe", p_flags, DockNodeFlags::NoDockingSplitMe);
    CheckboxFlags("NoDockingSplitOther", p_flags, DockNodeFlags::NoDockingSplitOther);
    CheckboxFlags("NoDockingOverMe", p_flags, DockNodeFlags::NoDockingOverMe);
    CheckboxFlags("NoDockingOverOther", p_flags, DockNodeFlags::NoDockingOverOther);
    CheckboxFlags("NoDockingOverEmpty", p_flags, DockNodeFlags::NoDockingOverEmpty);
    if (!enabled)
        EndDisabled();
    pop_style_var();
    pop_id();
}

// [DEBUG] Display contents of ImDockNode
// void DebugNodeDockNode(ImGuiDockNode* node, const char* label)
pub fn debug_node_dock_node(g: &mut Context, node: &mut DockNode, label: &str)
{
    // ImGuiContext& g = *GImGui;
    let is_alive = (g.frame_count - node->last_frame_alive < 2);    // Submitted with ImGuiDockNodeFlags_KeepAliveOnly
    let is_active = (g.frame_count - node->last_frame_active < 2);  // Submitted
    if (!is_alive) { push_style_color(, StyleColor::Text, GetStyleColorVec4(StyleColor::TextDisabled)); }
    bool open;
    ImGuiTreeNodeFlags tree_node_flags = if node->IsFocused { ImGuiTreeNodeFlags_Selected }else{ ImGuiTreeNodeFlags_None};
    if (node->Windows.size > 0)
        open = TreeNodeEx((void*)(intptr_t)node->ID, tree_node_flags, "%s 0x%04X%s: %d windows (vis: '%s')", label, node->ID, node->is_visible ? "" : " (hidden)", node->Windows.size, node->VisibleWindow ? node->VisibleWindow->Name : "None");
    else
        open = TreeNodeEx((void*)(intptr_t)node->ID, tree_node_flags, "%s 0x%04X%s: %s split (vis: '%s')", label, node->ID, node->is_visible ? "" : " (hidden)", (node->SplitAxis == Axis::X) ? "horizontal" : (node->SplitAxis == Axis::Y) ? "vertical" : "n/a", node->VisibleWindow ? node->VisibleWindow->Name : "None");
    if (!is_alive) { pop_style_color(); }
    if (is_active && is_item_hovered())
        if (Window* window = node->HostWindow ? node->HostWindow : node->VisibleWindow)
            foreground_draw_list(window)->AddRect(node.pos, node.pos + node.size, IM_COL32(255, 255, 0, 255));
    if (open)
    {
        // IM_ASSERT(node->ChildNodes[0] == None || node->ChildNodes[0]->ParentNode == node);
        // IM_ASSERT(node->ChildNodes[1] == None || node->ChildNodes[1]->ParentNode == node);
        BulletText("pos (%.0,%.0), size (%.0, %.0) Ref (%.0, %.0)",
            node.pos.x, node.pos.y, node.size.x, node.size.y, node.size_ref.x, node.size_ref.y);
        DebugNodeWindow(node->HostWindow, "host_window");
        DebugNodeWindow(node->VisibleWindow, "visible_window");
        BulletText("SelectedTabID: 0x%08X, LastFocusedNodeID: 0x%08X", node->SelectedTabId, node->last_focused_node_id);
        BulletText("Misc:%s%s%s%s%s%s%s",
            node->is_dock_space() ? " is_dock_space" : "",
            node->is_central_node() ? " is_central_node" : "",
            is_alive ? " IsAlive" : "", is_active ? " IsActive" : "", node->IsFocused ? " is_focused" : "",
            node->want_lock_size_once ? " want_lock_size_once" : "",
            node->HasCentralNodeChild ? " has_central_node_child" : "");
        if (TreeNode("flags", "flags Merged: 0x%04X, Local: 0x%04X, InWindows: 0x%04X, Shared: 0x%04X", node->MergedFlags, node->LocalFlags, node->LocalFlagsInWindows, node->SharedFlags))
        {
            if (BeginTable("flags", 4))
            {
                TableNextColumn(); DebugNodeDockNodeFlags(&node->MergedFlags, "merged_flags", false);
                TableNextColumn(); DebugNodeDockNodeFlags(&node->LocalFlags, "local_flags", true);
                TableNextColumn(); DebugNodeDockNodeFlags(&node->LocalFlagsInWindows, "local_flags_in_windows", false);
                TableNextColumn(); DebugNodeDockNodeFlags(&node->SharedFlags, "shared_flags", true);
                EndTable();
            }
            TreePop();
        }
        if (node->ParentNode)
            DebugNodeDockNode(node->ParentNode, "parent_node");
        if (node->ChildNodes[0])
            DebugNodeDockNode(node->ChildNodes[0], "Child[0]");
        if (node->ChildNodes[1])
            DebugNodeDockNode(node->ChildNodes[1], "Child[1]");
        if (node->TabBar)
            DebugNodeTabBar(node->TabBar, "tab_bar");
        TreePop();
    }
}

// [DEBUG] Display contents of ImDrawList
// Note that both 'window' and 'viewport' may be None here. viewport is generally null of destroyed popups which previously owned a viewport.
// void DebugNodeDrawList(Window* window, ImGuiViewportP* viewport, const ImDrawList* draw_list, const char* label)
pub fn debug_node_draw_list(g: &mut Context, window: &mut window::Window, viewport: &mut Viewport, draw_list: &DrawList, label: &str)
{
    // ImGuiContext& g = *GImGui;
    ImGuiMetricsConfig* cfg = &g.DebugMetricsConfig;
    int cmd_count = draw_list.cmd_buffer.size;
    if (cmd_count > 0 && draw_list.cmd_buffer.back().elem_count == 0 && draw_list.cmd_buffer.back().user_callback == None)
        cmd_count -= 1 ;
    bool node_open = TreeNode(draw_list, "%s: '%s' %d vtx, %d indices, %d cmds", label, draw_list->_OwnerName ? draw_list->_OwnerName : "", draw_list->vtx_buffer.size, draw_list->idx_buffer.size, cmd_count);
    if (draw_list == GetWindowDrawList())
    {
        same_line();
        TextColored(Vector4D(1.0, 0.4, 0.4, 1.0), "CURRENTLY APPENDING"); // Can't display stats for active draw list! (we don't have the data double-buffered)
        if (node_open)
            TreePop();
        return;
    }

    ImDrawList* fg_draw_list = if viewport { foreground_draw_list(viewport) }else{ None}; // Render additional visuals into the top-most draw list
    if (window && is_item_hovered() && fg_draw_list)
        fg_draw_list->AddRect(window.pos, window.pos + window.size, IM_COL32(255, 255, 0, 255));
    if (!node_open)
        return;

    if (window && !window.was_active)
        TextDisabled("Warning: owning window is inactive. This draw_list is not being rendered!");

    for (const ImDrawCmd* pcmd = draw_list.cmd_buffer.data; pcmd < draw_list.cmd_buffer.data + cmd_count; pcmd += 1)
    {
        if (pcmd->UserCallback)
        {
            BulletText("Callback %p, user_data %p", pcmd->UserCallback, pcmd->UserCallbackData);
            continue;
        }

        char buf[300];
        ImFormatString(buf, IM_ARRAYSIZE(buf), "DrawCmd:%5d tris, Tex 0x%p, clip_rect (%4.0,%4.0)-(%4.0,%4.0)",
            pcmd->ElemCount / 3, (void*)(intptr_t)pcmd->TextureId,
            pcmd->ClipRect.x, pcmd->ClipRect.y, pcmd->ClipRect.z, pcmd->ClipRect.w);
        bool pcmd_node_open = TreeNode((void*)(pcmd - draw_list.cmd_buffer.begin()), "%s", buf);
        if (is_item_hovered() && (cfg->ShowDrawCmdMesh || cfg->ShowDrawCmdBoundingBoxes) && fg_draw_list)
            DebugNodeDrawCmdShowMeshAndBoundingBox(fg_draw_list, draw_list, pcmd, cfg->ShowDrawCmdMesh, cfg->ShowDrawCmdBoundingBoxes);
        if (!pcmd_node_open)
            continue;

        // Calculate approximate coverage area (touched pixel count)
        // This will be in pixels squared as long there's no post-scaling happening to the renderer output.
        const ImDrawIdx* idx_buffer = if (draw_list->idx_buffer.size > 0) { draw_list->idx_buffer.data }else{ None};
        const ImDrawVert* vtx_buffer = draw_list->vtx_buffer.data + pcmd->vtx_offset;
        let total_area =  0.0;
        for (unsigned int idx_n = pcmd->IdxOffset; idx_n < pcmd->IdxOffset + pcmd->ElemCount; )
        {
            Vector2D triangle[3];
            for (int n = 0; n < 3; n += 1, idx_n += 1)
                triangle[n] = if vtx_buffer[idx_buffer { idx_buffer[idx_n] }else{ idx_n].pos};
            total_area += ImTriangleArea(triangle[0], triangle[1], triangle[2]);
        }

        // Display vertex information summary. Hover to get all triangles drawn in wire-frame
        ImFormatString(buf, IM_ARRAYSIZE(buf), "Mesh: elem_count: %d, vtx_offset: +%d, idx_offset: +%d, Area: ~%0.f px", pcmd->ElemCount, pcmd->vtx_offset, pcmd->IdxOffset, total_area);
        selectable(buf);
        if (is_item_hovered() && fg_draw_list)
            DebugNodeDrawCmdShowMeshAndBoundingBox(fg_draw_list, draw_list, pcmd, true, false);

        // Display individual triangles/vertices. Hover on to get the corresponding triangle highlighted.
        ImGuiListClipper clipper;
        clipper.begin(pcmd->ElemCount / 3); // Manually coarse clip our print out of individual vertices to save CPU, only items that may be visible.
        while (clipper.Step())
            for (int prim = clipper.DisplayStart, idx_i = pcmd->IdxOffset + clipper.DisplayStart * 3; prim < clipper.DisplayEnd; prim += 1)
            {
                char* buf_p = buf, * buf_end = buf + IM_ARRAYSIZE(buf);
                Vector2D triangle[3];
                for (int n = 0; n < 3; n += 1, idx_i += 1)
                {
                    const ImDrawVert& v = vtx_buffer[idx_buffer ? idx_buffer[idx_i] : idx_i];
                    triangle[n] = v.pos;
                    buf_p += ImFormatString(buf_p, buf_end - buf_p, "%s %04d: pos (%8.2,%8.2), uv (%.6,%.6), col %08X\n",
                        (n == 0) ? "Vert:" : "     ", idx_i, v.pos.x, v.pos.y, v.uv.x, v.uv.y, v.col);
                }

                selectable(buf, false);
                if (fg_draw_list && is_item_hovered())
                {
                    ImDrawListFlags backup_flags = fg_draw_list.flags;
                    fg_draw_list.flags &= ~DrawListFlags::AntiAliasedLines; // Disable AA on triangle outlines is more readable for very large and thin triangles.
                    fg_draw_list->AddPolyline(triangle, 3, IM_COL32(255, 255, 0, 255), DrawFlags::Closed, 1.0);
                    fg_draw_list.flags = backup_flags;
                }
            }
        TreePop();
    }
    TreePop();
}

// [DEBUG] Display mesh/aabb of a ImDrawCmd
// void DebugNodeDrawCmdShowMeshAndBoundingBox(ImDrawList* out_draw_list, const ImDrawList* draw_list, const ImDrawCmd* draw_cmd, bool show_mesh, bool show_aabb)
pub fn debug_node_draw_cmd_show_mesh_and_bounding_box(g: &mut Context, out_draw_list: &mut DrawList, draw_list: &DrawList, draw_cmd: &DrawCommand, show_mesh: bool, show_aabb: bool)
{
    // IM_ASSERT(show_mesh || show_aabb);

    // Draw wire-frame version of all triangles
    Rect clip_rect = draw_cmd->ClipRect;
    Rect vtxs_rect(f32::MAX, f32::MAX, -f32::MAX, -f32::MAX);
    ImDrawListFlags backup_flags = out_draw_list.flags;
    out_draw_list.flags &= ~DrawListFlags::AntiAliasedLines; // Disable AA on triangle outlines is more readable for very large and thin triangles.
    for (unsigned int idx_n = draw_cmd->IdxOffset, idx_end = draw_cmd->IdxOffset + draw_cmd->ElemCount; idx_n < idx_end; )
    {
        ImDrawIdx* idx_buffer = if (draw_list->idx_buffer.size > 0) { draw_list->idx_buffer.data }else{ None}; // We don't hold on those pointers past iterations as ->add_polyline() may invalidate them if out_draw_list==draw_list
        ImDrawVert* vtx_buffer = draw_list->vtx_buffer.data + draw_cmd->vtx_offset;

        Vector2D triangle[3];
        for (int n = 0; n < 3; n += 1, idx_n += 1)
            vtxs_rect.Add((triangle[n] = vtx_buffer[idx_buffer ? idx_buffer[idx_n] : idx_n].pos));
        if (show_mesh)
            out_draw_list->AddPolyline(triangle, 3, IM_COL32(255, 255, 0, 255), DrawFlags::Closed, 1.0); // In yellow: mesh triangles
    }
    // Draw bounding boxes
    if (show_aabb)
    {
        out_draw_list->AddRect(f32::floor(clip_rect.min), f32::floor(clip_rect.max), IM_COL32(255, 0, 255, 255)); // In pink: clipping rectangle submitted to GPU
        out_draw_list->AddRect(f32::floor(vtxs_rect.min), f32::floor(vtxs_rect.max), IM_COL32(0, 255, 255, 255)); // In cyan: bounding box of triangles
    }
    out_draw_list.flags = backup_flags;
}

// [DEBUG] Display details for a single font, called by ShowStyleEditor().
// void DebugNodeFont(ImFont* font)
pub fn debug_node_font(g: &mut Context, font: &mut Font)
{
    bool opened = TreeNode(font, "font: \"%s\"\n%.2 px, %d glyphs, %d file(s)",
        font->ConfigData ? font->ConfigData[0].name : "", font->FontSize, font->Glyphs.size, font->ConfigDataCount);
    same_line();
    if (SmallButton("Set as default"))
        GetIO().FontDefault = font;
    if (!opened)
        return;

    // Display preview text
    PushFont(font);
    text("The quick brown fox jumps over the lazy dog");
    PopFont();

    // Display details
    SetNextItemWidth(GetFontSize() * 8);
    DragFloat("font scale", &font->Scale, 0.005, 0.3, 2.0, "%.1");
    same_line(); MetricsHelpMarker(
        "Note than the default embedded font is NOT meant to be scaled.\n\n"
        "font are currently rendered into bitmaps at a given size at the time of building the atlas. "
        "You may oversample them to get some flexibility with scaling. "
        "You can also render at multiple sizes and select which one to use at runtime.\n\n"
        "(Glimmer of hope: the atlas system will be rewritten in the future to make scaling more flexible.)");
    text("ascent: %f, descent: %f, height: %f", font->Ascent, font->Descent, font->Ascent - font->Descent);
    char c_str[5];
    text("Fallback character: '%s' (U+%04X)", ImTextCharToUtf8(c_str, font->FallbackChar), font->FallbackChar);
    text("Ellipsis character: '%s' (U+%04X)", ImTextCharToUtf8(c_str, font->EllipsisChar), font->EllipsisChar);
    let surface_sqrt = ImSqrt(font->MetricsTotalSurface);
    text("Texture Area: about %d px ~%dx%d px", font->MetricsTotalSurface, surface_sqrt, surface_sqrt);
    for (int config_i = 0; config_i < font->ConfigDataCount; config_i += 1)
        if (font->ConfigData)
            if (const ImFontConfig* cfg = &font->ConfigData[config_i])
                BulletText("Input %d: \'%s\', Oversample: (%d,%d), pixel_snap_h: %d, Offset: (%.1,%.1)",
                    config_i, cfg->Name, cfg->OversampleH, cfg->OversampleV, cfg->PixelSnapH, cfg->GlyphOffset.x, cfg->GlyphOffset.y);

    // Display all glyphs of the fonts in separate pages of 256 characters
    if (TreeNode("glyphs", "glyphs (%d)", font->Glyphs.size))
    {
        ImDrawList* draw_list = GetWindowDrawList();
        const ImU32 glyph_col = color_u32_from_style_color(StyleColor::Text);
        let cell_size = font->FontSize * 1;
        let cell_spacing = GetStyle().item_spacing.y;
        for (unsigned int base = 0; base <= IM_UNICODE_CODEPOINT_MAX; base += 256)
        {
            // Skip ahead if a large bunch of glyphs are not present in the font (test in chunks of 4k)
            // This is only a small optimization to reduce the number of iterations when IM_UNICODE_MAX_CODEPOINT
            // is large // (if ImWchar==ImWchar32 we will do at least about 272 queries here)
            if (!(base & 4095) && font->IsGlyphRangeUnused(base, base + 4095))
            {
                base += 4096 - 256;
                continue;
            }

            int count = 0;
            for (unsigned int n = 0; n < 256; n += 1)
                if (font->FindGlyphNoFallback((ImWchar)(base + n)))
                    count += 1;
            if (count <= 0)
                continue;
            if (!TreeNode((void*)(intptr_t)base, "U+%04X..U+%04X (%d %s)", base, base + 255, count, count > 1 ? "glyphs" : "glyph"))
                continue;

            // Draw a 16x16 grid of glyphs
            Vector2D base_pos = GetCursorScreenPos();
            for (unsigned int n = 0; n < 256; n += 1)
            {
                // We use ImFont::render_char as a shortcut because we don't have UTF-8 conversion functions
                // available here and thus cannot easily generate a zero-terminated UTF-8 encoded string.
                Vector2D cell_p1(base_pos.x + (n % 16) * (cell_size + cell_spacing), base_pos.y + (n / 16) * (cell_size + cell_spacing));
                Vector2D cell_p2(cell_p1.x + cell_size, cell_p1.y + cell_size);
                const ImFontGlyph* glyph = font->FindGlyphNoFallback((ImWchar)(base + n));
                draw_list->AddRect(cell_p1, cell_p2, glyph ? IM_COL32(255, 255, 255, 100) : IM_COL32(255, 255, 255, 50));
                if (!glyph)
                    continue;
                font->RenderChar(draw_list, cell_size, cell_p1, glyph_col, (ImWchar)(base + n));
                if (is_mouse_hovering_rect(cell_p1, cell_p2))
                {
                    BeginTooltip();
                    DebugNodeFontGlyph(font, glyph);
                    EndTooltip();
                }
            }
            Dummy(Vector2D::new((cell_size + cell_spacing) * 16, (cell_size + cell_spacing) * 16));
            TreePop();
        }
        TreePop();
    }
    TreePop();
}

// void DebugNodeFontGlyph(ImFont*, const ImFontGlyph* glyph)
pub fn debug_node_font_glyph(g: &mut Context, font: &mut Font, glyph: &FontGlyph)
{
    text("codepoint: U+%04X", glyph->Codepoint);
    Separator();
    text("visible: %d", glyph->Visible);
    text("advance_x: %.1", glyph->AdvanceX);
    text("pos: (%.2,%.2)->(%.2,%.2)", glyph->X0, glyph->Y0, glyph->X1, glyph->Y1);
    text("UV: (%.3,%.3)->(%.3,%.3)", glyph->U0, glyph->V0, glyph->U1, glyph->V1);
}

// [DEBUG] Display contents of ImGuiStorage
// void DebugNodeStorage(ImGuiStorage* storage, const char* label)
pub fn debug_node_storage(g: &mut Context, )
{
    if (!TreeNode(label, "%s: %d entries, %d bytes", label, storage->Data.size, storage->Data.size_in_bytes()))
        return;
    for (int n = 0; n < storage->Data.size; n += 1)
    {
        const ImGuiStorage::ImGuiStoragePair& p = storage->Data[n];
        BulletText("Key 0x%08X value { i: %d }", p.key, p.val_i); // Important: we currently don't store a type, real value may not be integer.
    }
    TreePop();
}

// [DEBUG] Display contents of ImGuiTabBar
// void DebugNodeTabBar(ImGuiTabBar* tab_bar, const char* label)
pub fn debug_node_tab_bar(g: &mut Context, tab_bar: &mut TabBar, label: &str)
{
    // Standalone tab bars (not associated to docking/windows functionality) currently hold no discernible strings.
    char buf[256];
    char* p = buf;
    const char* buf_end = buf + IM_ARRAYSIZE(buf);
    let is_active = (tab_bar->PrevFrameVisible >= GetFrameCount() - 2);
    p += ImFormatString(p, buf_end - p, "%s 0x%08X (%d tabs)%s", label, tab_bar->ID, tab_bar->Tabs.size, is_active ? "" : " *Inactive*");
    p += ImFormatString(p, buf_end - p, "  { ");
    for (int tab_n = 0; tab_n < ImMin(tab_bar->Tabs.size, 3); tab_n += 1)
    {
        ImGuiTabItem* tab = &tab_bar->Tabs[tab_n];
        p += ImFormatString(p, buf_end - p, "%s'%s'",
            tab_n > 0 ? ", " : "", (tab->Window || tab->NameOffset != -1) ? tab_bar->get_tab_name(tab) : "???");
    }
    p += ImFormatString(p, buf_end - p, (tab_bar->Tabs.size > 3) ? " ... }" : " } ");
    if (!is_active) { push_style_color(, StyleColor::Text, GetStyleColorVec4(StyleColor::TextDisabled)); }
    bool open = TreeNode(label, "%s", buf);
    if (!is_active) { pop_style_color(); }
    if (is_active && is_item_hovered())
    {
        ImDrawList* draw_list = foreground_draw_list();
        draw_list->AddRect(tab_bar->BarRect.min, tab_bar->BarRect.max, IM_COL32(255, 255, 0, 255));
        draw_list->AddLine(Vector2D::new(tab_bar->ScrollingRectMinX, tab_bar->BarRect.min.y), Vector2D::new(tab_bar->ScrollingRectMinX, tab_bar->BarRect.max.y), IM_COL32(0, 255, 0, 255));
        draw_list->AddLine(Vector2D::new(tab_bar->ScrollingRectMaxX, tab_bar->BarRect.min.y), Vector2D::new(tab_bar->ScrollingRectMaxX, tab_bar->BarRect.max.y), IM_COL32(0, 255, 0, 255));
    }
    if (open)
    {
        for (int tab_n = 0; tab_n < tab_bar->Tabs.size; tab_n += 1)
        {
            const ImGuiTabItem* tab = &tab_bar->Tabs[tab_n];
            push_id(tab);
            if (SmallButton("<")) { TabBarQueueReorder(tab_bar, tab, -1); } same_line(0, 2);
            if (SmallButton(">")) { TabBarQueueReorder(tab_bar, tab, +1); } same_line();
            text("%02d%c Tab 0x%08X '%s' Offset: %.1, width: %.1/%.1",
                tab_n, (tab->ID == tab_bar->SelectedTabId) ? '*' : ' ', tab->ID, (tab->Window || tab->NameOffset != -1) ? tab_bar->get_tab_name(tab) : "???", tab->Offset, tab->Width, tab->ContentWidth);
            pop_id();
        }
        TreePop();
    }
}

// void DebugNodeViewport(ImGuiViewportP* viewport)
pub fn debug_node_viewport(g: &mut Context, viewport: &mut Viewport)
{
    SetNextItemOpen(true, ImGuiCond_Once);
    if (TreeNode((void*)(intptr_t)viewport->ID, "viewport #%d, id: 0x%08X, Parent: 0x%08X, window: \"%s\"", viewport->Idx, viewport->ID, viewport->ParentViewportId, viewport->Window ? viewport->Window->Name : "N/A"))
    {
        WindowFlags flags = viewport.flags;
        BulletText("Main pos: (%.0,%.0), size: (%.0,%.0)\nWorkArea Offset Left: %.0 Top: %.0, Right: %.0, Bottom: %.0\nMonitor: %d, dpi_scale: %.0%%",
            viewport.pos.x, viewport.pos.y, viewport.size.x, viewport.size.y,
            viewport->WorkOffsetMin.x, viewport->WorkOffsetMin.y, viewport->WorkOffsetMax.x, viewport->WorkOffsetMax.y,
            viewport->platform_monitor, viewport->DpiScale * 100.0);
        if (viewport->Idx > 0) { same_line(); if (SmallButton("Reset pos")) { viewport.pos = Vector2D::new(200, 200); viewport.update_work_rect(); if (viewport->Window) viewport->Window.pos = viewport.pos; } }
        BulletText("flags: 0x%04X =%s%s%s%s%s%s%s%s%s%s%s%s", viewport.flags,
            //(flags & ImGuiViewportFlags_IsPlatformWindow) ? " IsPlatformWindow" : "", // Omitting because it is the standard
            (flags & ViewportFlags::Isplatform_monitor) ? " Isplatform_monitor" : "",
            (flags & ViewportFlags::OwnedByApp) ? " OwnedByApp" : "",
            (flags & ViewportFlags::NoDecoration) ? " NoDecoration" : "",
            (flags & ViewportFlags::NoTaskBarIcon) ? " NoTaskBarIcon" : "",
            (flags & ViewportFlags::NoFocusOnAppearing) ? " NoFocusOnAppearing" : "",
            (flags & ViewportFlags::NoFocusOnClick) ? " NoFocusOnClick" : "",
            (flags & ViewportFlags::NoInputs) ? " NoInputs" : "",
            (flags & ViewportFlags::NoRendererClear) ? " NoRendererClear" : "",
            (flags & ViewportFlags::TopMost) ? " TopMost" : "",
            (flags & ViewportFlags::Minimized) ? " Minimized" : "",
            (flags & ViewportFlags::NoAutoMerge) ? " NoAutoMerge" : "",
            (flags & ViewportFlags::CanHostOtherWindows) ? " CanHostOtherWindows" : "");
        for (int layer_i = 0; layer_i < IM_ARRAYSIZE(viewport.draw_data_builder.layers); layer_i += 1)
            for (int draw_list_i = 0; draw_list_i < viewport.draw_data_builder.layers[layer_i].size; draw_list_i += 1)
                DebugNodeDrawList(None, viewport, viewport.draw_data_builder.layers[layer_i][draw_list_i], "draw_list");
        TreePop();
    }
}

// void DebugNodeWindow(Window* window, const char* label)
pub fn debug_node_window(g: &mut Context, window: &mut window::Window, label: &str)
{
    if (window == None)
    {
        BulletText("%s: None", label);
        return;
    }

    // ImGuiContext& g = *GImGui;
    let is_active = window.was_active;
    ImGuiTreeNodeFlags tree_node_flags = if (window == g.nav_window) { ImGuiTreeNodeFlags_Selected }else{ ImGuiTreeNodeFlags_None};
    if (!is_active) { push_style_color(, StyleColor::Text, GetStyleColorVec4(StyleColor::TextDisabled)); }
    let open = TreeNodeEx(label, tree_node_flags, "%s '%s'%s", label, window.name, is_active ? "" : " *Inactive*");
    if (!is_active) { pop_style_color(); }
    if (is_item_hovered() && is_active)
        foreground_draw_list(window)->AddRect(window.pos, window.pos + window.size, IM_COL32(255, 255, 0, 255));
    if (!open)
        return;

    if (window.memory_compacted)
        TextDisabled("Note: some memory buffers have been compacted/freed.");

    WindowFlags flags = window.flags;
    DebugNodeDrawList(window, window.viewport, window.draw_list, "draw_list");
    BulletText("pos: (%.1,%.1), size: (%.1,%.1), content_size (%.1,%.1) Ideal (%.1,%.1)", window.pos.x, window.pos.y, window.size.x, window.size.y, window.ContentSize.x, window.ContentSize.y, window.ContentSizeIdeal.x, window.ContentSizeIdeal.y);
    BulletText("flags: 0x%08X (%s%s%s%s%s%s%s%s%s..)", flags,
        (flags & WindowFlags::ChildWindow)  ? "Child " : "",      (flags & WindowFlags::Tooltip)     ? "Tooltip "   : "",  (flags & WindowFlags::Popup) ? "Popup " : "",
        (flags & WindowFlags::Modal)        ? "Modal " : "",      (flags & WindowFlags::ChildMenu)   ? "ChildMenu " : "",  (flags & WindowFlags::NoSavedSettings) ? "NoSavedSettings " : "",
        (flags & WindowFlags::NoMouseInputs)? "NoMouseInputs":"", (flags & WindowFlags::NoNavInputs) ? "NoNavInputs" : "", (flags & WindowFlags::AlwaysAutoResize) ? "AlwaysAutoResize" : "");
    BulletText("WindowClassId: 0x%08X", window.window_class.class_id);
    BulletText("scroll: (%.2/%.2,%.2/%.2) Scrollbar:%s%s", window.scroll.x, window.scroll_max.x, window.scroll.y, window.scroll_max.y, window.scrollbar_x ? "x" : "", window.scrollbar_y ? "Y" : "");
    BulletText("active: %d/%d, write_accessed: %d, begin_order_within_context: %d", window.active, window.was_active, window.write_accessed, (window.active || window.was_active) ? window.begin_order_within_context : -1);
    BulletText("appearing: %d, hidden: %d (CanSkip %d Cannot %d), skip_items: %d", window.Appearing, window.hidden, window..hidden_frames_can_skip_items, window.hidden_frames_cannot_skip_items, window.skip_items);
    for (int layer = 0; layer < NavLayer::COUNT; layer += 1)
    {
        Rect r = window.nav_rect_rel[layer];
        if (r.min.x >= r.max.y && r.min.y >= r.max.y)
        {
            BulletText("nav_last_ids[%d]: 0x%08X", layer, window.nav_last_ids[layer]);
            continue;
        }
        BulletText("nav_last_ids[%d]: 0x%08X at +(%.1,%.1)(%.1,%.1)", layer, window.nav_last_ids[layer], r.min.x, r.min.y, r.max.x, r.max.y);
        if (is_item_hovered())
            foreground_draw_list(window)->AddRect(r.min + window.pos, r.max + window.pos, IM_COL32(255, 255, 0, 255));
    }
    BulletText("nav_layers_active_mask: %x, nav_last_child_nav_window: %s", window.dc.nav_layers_active_mask, window.nav_last_child_nav_window_id ? window.nav_last_child_nav_window_id->Name : "None");

    BulletText("viewport: %d%s, viewport_id: 0x%08X, viewport_pos: (%.1,%.1)", window.viewport ? window.viewport->Idx : -1, window.viewport_owned ? " (Owned)" : "", window.viewport_id, window.viewport_pos.x, window.viewport_pos.y);
    BulletText("ViewportMonitor: %d", window.viewport ? window.viewport->platform_monitor : -1);
    BulletText("dock_id: 0x%04X, dock_order: %d, Act: %d, Vis: %d", window.dock_id, window.dock_order, window.dock_is_active, window.dock_tab_is_visible);
    if (window.dock_node_id || window.dock_node_as_host_id)
        DebugNodeDockNode(window.dock_node_as_host_id? window.dock_node_as_host_id: window.dock_node, window.dock_node_as_host_id? "dock_node_as_host": "dock_node");

    if (window.root_window != window)       { DebugNodeWindow(window.root_window, "RootWindow"); }
    if (window.root_window_dock_tree != window.root_window) { DebugNodeWindow(window.root_window_dock_tree, "root_window_dock_tree"); }
    if (window.parent_window != None)       { DebugNodeWindow(window.parent_window, "ParentWindow"); }
    if (window.dc.ChildWindows.size > 0)   { DebugNodeWindowsList(&window.dc.ChildWindows, "ChildWindows"); }
    if (window.ColumnsStorage.size > 0 && TreeNode("columns", "columns sets (%d)", window.ColumnsStorage.size))
    {
        for (int n = 0; n < window.ColumnsStorage.size; n += 1)
            DebugNodeColumns(&window.ColumnsStorage[n]);
        TreePop();
    }
    DebugNodeStorage(&window.StateStorage, "Storage");
    TreePop();
}

// void DebugNodeWindowSettings(WindowSettings* settings)
pub fn debug_node_window_Settings(g: &mut Context, settings: &mut WindowSettings)
{
    text("0x%08X \"%s\" pos (%d,%d) size (%d,%d) collapsed=%d",
        settings->ID, settings->GetName(), settings.pos.x, settings.pos.y, settings.size.x, settings.size.y, settings.collapsed);
}

// void DebugNodeWindowsList(ImVector<Window*>* windows, const char* label)
pub fn debug_node_windows_list(g: &mut Context, windows: &mut Vec<Id32>, label: &str)
{
    if (!TreeNode(label, "%s (%d)", label, windows.len()))
        return;
    for (int i = windows.len() - 1; i >= 0; i -= 1 ) // Iterate front to back
    {
        push_id((*windows)[i]);
        DebugNodeWindow((*windows)[i], "window");
        pop_id();
    }
    TreePop();
}

// FIXME-OPT: This is technically suboptimal, but it is simpler this way.
// void DebugNodeWindowsListByBeginStackParent(Window** windows, int windows_size, Window* parent_in_begin_stack)
pub fn debug_node_windows_list_by_begin_stack_parent(g: &mut Context, windows: &mut Vec<id32>, windows_size: i32, parent_in_begin_stack: &mut window::Window)
{
    for (int i = 0; i < windows_size; i += 1)
    {
        Window* window = windows[i];
        if (window.ParentWindowInBeginStack != parent_in_begin_stack)
            continue;
        char buf[20];
        ImFormatString(buf, IM_ARRAYSIZE(buf), "[%04d] window", window.begin_order_within_context);
        //BulletText("[%04d] window '%s'", window->begin_order_within_context, window->name);
        DebugNodeWindow(window, buf);
        Indent();
        DebugNodeWindowsListByBeginStackParent(windows + i + 1, windows_size - i - 1, window);
        Unindent();
    }
}

// [DEBUG] Item picker tool - start with DebugStartItemPicker() - useful to visually select an item and break into its call-stack.
// void update_debug_tool_item_picker()
pub fn update_debug_tool_item_picker(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    g.debug_item_picker_break_id = 0;
    if (!g.debug_item_picker_active)
        return;

    const Id32 hovered_id = g.hovered_id_previous_frame;
    SetMouseCursor(ImGuiMouseCursor_Hand);
    if (IsKeyPressed(ImGuiKey_Escape))
        g.debug_item_picker_active = false;
    if (is_mouse_clicked(0) && hovered_id)
    {
        g.debug_item_picker_break_id = hovered_id;
        g.debug_item_picker_active = false;
    }
    set_netxt_window_bg_alpha(0.60);
    BeginTooltip();
    text("hovered_id: 0x%08X", hovered_id);
    text("Press ESC to abort picking.");
    TextColored(GetStyleColorVec4(hovered_id ? StyleColor::Text : StyleColor::TextDisabled), "Click to break in debugger!");
    EndTooltip();
}

// [DEBUG] Stack Tool: update queries. Called by NewFrame()
// void update_debug_tool_stack_queries()
pub fn update_debug_tool_stack_queries(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    ImGuiStackTool* tool = &g.DebugStackTool;

    // clear hook when stack tool is not visible
    g.debug_hook_id_info = 0;
    if (g.frame_count != tool->LastActiveFrame + 1)
        return;

    // update queries. The steps are: -1: query Stack, >= 0: query each stack item
    // We can only perform 1 id Info query every frame. This is designed so the GetID() tests are cheap and constant-time
    const Id32 query_id = if g.hovered_id_previous_frame { g.hovered_id_previous_frame }else{ g.active_id};
    if (tool->QueryId != query_id)
    {
        tool->QueryId = query_id;
        tool->StackLevel = -1;
        tool->Results.resize(0);
    }
    if (query_id == 0)
        return;

    // Advance to next stack level when we got our result, or after 2 frames (in case we never get a result)
    int stack_level = tool->StackLevel;
    if (stack_level >= 0 && stack_level < tool->Results.size)
        if (tool->Results[stack_level].QuerySuccess || tool->Results[stack_level].QueryFrameCount > 2)
            tool->StackLevel += 1;

    // update hook
    stack_level = tool->StackLevel;
    if (stack_level == -1)
        g.debug_hook_id_info = query_id;
    if (stack_level >= 0 && stack_level < tool->Results.size)
    {
        g.debug_hook_id_info = tool->Results[stack_level].id;
        tool->Results[stack_level].QueryFrameCount += 1;
    }
}

// [DEBUG] Stack tool: hooks called by GetID() family functions
// void debug_hook_id_info(Id32 id, DataType data_type, const void* data_id, const void* data_id_end)
pub fn debug_hook_id_info(g: &mut Context, id: Id32, data_type: DataType, data_id: &Vec<u8>, data_id_end: &Vec<u8>)
{
    // ImGuiContext& g = *GImGui;
    let window = g.current_window_mut();
    ImGuiStackTool* tool = &g.DebugStackTool;

    // step 0: stack query
    // This assume that the id was computed with the current id stack, which tends to be the case for our widget.
    if (tool->StackLevel == -1)
    {
        tool->StackLevel += 1;
        tool->Results.resize(window.id_stack.size + 1, ImGuiStackLevelInfo());
        for (int n = 0; n < window.id_stack.size + 1; n += 1)
            tool->Results[n].id = if (n < window.id_stack.size) { window.id_stack[n] }else{ id};
        return;
    }

    // step 1+: query for individual level
    // IM_ASSERT(tool->StackLevel >= 0);
    if (tool->StackLevel != window.id_stack.size)
        return;
    ImGuiStackLevelInfo* info = &tool->Results[tool->StackLevel];
    // IM_ASSERT(info->id == id && info->QueryFrameCount > 0);

    switch (data_type)
    {
    case DataType::S32:
        ImFormatString(info->Desc, IM_ARRAYSIZE(info->Desc), "%d", (intptr_t)data_id);
        break;
    case DataType::String:
        ImFormatString(info->Desc, IM_ARRAYSIZE(info->Desc), "%.*s", data_id_end ? ((const char*)data_id_end - (const char*)data_id) : strlen((const char*)data_id), (const char*)data_id);
        break;
    case DataType::Pointer:
        ImFormatString(info->Desc, IM_ARRAYSIZE(info->Desc), "(void*)0x%p", data_id);
        break;
    case DataType::ID:
        if (info->Desc[0] != 0) // PushOverrideID() is often used to avoid hashing twice, which would lead to 2 calls to debug_hook_id_info(). We prioritize the first one.
            return;
        ImFormatString(info->Desc, IM_ARRAYSIZE(info->Desc), "0x%08X [override]", id);
        break;
    default:
        // IM_ASSERT(0);
    }
    info->QuerySuccess = true;
    info->DataType = data_type;
}

// static int StackToolFormatLevelInfo(ImGuiStackTool* tool, int n, bool format_for_ui, char* buf, size_t buf_size)
pub fn stack_tool_format_level(g: &mut Context, tool: &StackTool, n: i32, format_for_ui: bool, buf: &mut String, buf_size: usize) -> i32
{
    ImGuiStackLevelInfo* info = &tool->Results[n];
    Window* window = if (info->Desc[0] == 0 && n == 0) { find_window_by_id(info->ID) }else{ None};
    if (window)                                                                 // Source: window name (because the root id don't call GetID() and so doesn't get hooked)
        return ImFormatString(buf, buf_size, format_for_ui ? "\"%s\" [window]" : "%s", window.name);
    if (info->QuerySuccess)                                                     // Source: GetID() hooks (prioritize over ItemInfo() because we frequently use patterns like: push_id(str), Button("") where they both have same id)
        return ImFormatString(buf, buf_size, (format_for_ui && info->DataType == DataType::String) ? "\"%s\"" : "%s", info->Desc);
    if (tool->StackLevel < tool->Results.size)                                  // Only start using fallback below when all queries are done, so during queries we don't flickering ??? markers.
        return (*buf = 0);
#ifdef IMGUI_ENABLE_TEST_ENGINE
    if (const char* label = ImGuiTestEngine_FindItemDebugLabel(GImGui, info->ID))   // Source: ImGuiTestEngine's ItemInfo()
        return ImFormatString(buf, buf_size, format_for_ui ? "??? \"%s\"" : "%s", label);

    return ImFormatString(buf, buf_size, "???");
}

// Stack Tool: Display UI
// void ShowStackToolWindow(bool* p_open)
pub fn show_stack_tool_window(g: &mut Context, p_open: &mut bool)
{
    // ImGuiContext& g = *GImGui;
    if (!(g.next_window_data.flags & NextWindowDataFlags::HasSize))
        set_next_window_size(Vector2D::new(0.0, GetFontSize() * 8.0), Condition::FirstUseEver);
    if (!begin("Dear ImGui Stack Tool", p_open) || GetCurrentWindow()->BeginCount > 1)
    {
        end();
        return;
    }

    // Display hovered/active status
    ImGuiStackTool* tool = &g.DebugStackTool;
    const Id32 hovered_id = g.hovered_id_previous_frame;
    const Id32 active_id = g.active_id;
#ifdef IMGUI_ENABLE_TEST_ENGINE
    text("hovered_id: 0x%08X (\"%s\"), active_id:  0x%08X (\"%s\")", hovered_id, hovered_id ? ImGuiTestEngine_FindItemDebugLabel(&g, hovered_id) : "", active_id, active_id ? ImGuiTestEngine_FindItemDebugLabel(&g, active_id) : "");
#else
    text("hovered_id: 0x%08X, active_id:  0x%08X", hovered_id, active_id);

    same_line();
    MetricsHelpMarker("Hover an item with the mouse to display elements of the id Stack leading to the item's final id.\nEach level of the stack correspond to a push_id() call.\nAll levels of the stack are hashed together to make the final id of a widget (id displayed at the bottom level of the stack).\nRead FAQ entry about the id stack for details.");

    // CTRL+C to copy path
    let time_since_copy = g.time - tool->CopyToClipboardLastTime;
    Checkbox("Ctrl+C: copy path to clipboard", &tool->CopyToClipboardOnCtrlC);
    same_line();
    TextColored((time_since_copy >= 0.0 && time_since_copy < 0.75 && f32::mod(time_since_copy, 0.25) < 0.25 * 0.5) ? Vector4D(1.f, 1.f, 0.3, 1.f) : Vector4D(), "*COPIED*");
    if (tool->CopyToClipboardOnCtrlC && IsKeyDown(Key::ModCtrl) && IsKeyPressed(ImGuiKey_C))
    {
        tool->CopyToClipboardLastTime = g.time;
        char* p = g.TempBuffer.data;
        char* p_end = p + g.TempBuffer.size;
        for (int stack_n = 0; stack_n < tool->Results.size && p + 3 < p_end; stack_n += 1)
        {
            *p += 1 = '/';
            char level_desc[256];
            StackToolFormatLevelInfo(tool, stack_n, false, level_desc, IM_ARRAYSIZE(level_desc));
            for (int n = 0; level_desc[n] && p + 2 < p_end; n += 1)
            {
                if (level_desc[n] == '/')
                    *p += 1 = '\\';
                *p += 1 = level_desc[n];
            }
        }
        *p = '\0';
        SetClipboardText(g.TempBuffer.data);
    }

    // Display decorated stack
    tool->LastActiveFrame = g.frame_count;
    if (tool->Results.size > 0 && BeginTable("##table", 3, ImGuiTableFlags_Borders))
    {
        let id_width = calc_text_size("0xDDDDDDDD").x;
        TableSetupColumn("Seed", ImGuiTableColumnFlags_WidthFixed, id_width);
        TableSetupColumn("push_id", ImGuiTableColumnFlags_WidthStretch);
        TableSetupColumn("Result", ImGuiTableColumnFlags_WidthFixed, id_width);
        TableHeadersRow();
        for (int n = 0; n < tool->Results.size; n += 1)
        {
            ImGuiStackLevelInfo* info = &tool->Results[n];
            TableNextColumn();
            text("0x%08X", (n > 0) ? tool->Results[n - 1].id : 0);
            TableNextColumn();
            StackToolFormatLevelInfo(tool, n, true, g.TempBuffer.data, g.TempBuffer.size);
            TextUnformatted(g.TempBuffer.data);
            TableNextColumn();
            text("0x%08X", info->ID);
            if (n == tool->Results.size - 1)
                TableSetBgColor(ImGuiTableBgTarget_CellBg, color_u32_from_style_color(StyleColor::Header));
        }
        EndTable();
    }
    end();
}
