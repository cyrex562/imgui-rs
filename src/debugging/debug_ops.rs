use std::env::args;
use std::ffi::CStr;
use std::ptr::null_mut;
use libc::{c_char, c_float, c_int, c_uint, c_void, open, size_t};
use crate::core::axis::{IM_GUI_AXIS_X, IM_GUI_AXIS_Y};
use crate::button_ops::SmallButton;
use crate::widgets::checkbox_ops::{Checkbox, CheckboxFlags};
use crate::child_ops::{BeginChild, EndChild};
use crate::clipboard_ops::SetClipboardText;
use crate::color::{color_u32_from_rgba, ImGuiCol_Border, ImGuiCol_Header, ImGuiCol_Text, ImGuiCol_TextDisabled, ImGuiCol_TitleBg, ImGuiCol_TitleBgActive, ImGuiCol_WindowBg};
use crate::combo_box::{Combo, Combo2};
use crate::core::condition::{ImGuiCond_FirstUseEver, ImGuiCond_Once};
use crate::core::context::AppContext;
use crate::context_ops::GetFrameCount;
use crate::cursor_ops::{cursor_screen_pos, indent, unindent};
use crate::data_type::{ImGuiDataType, IM_GUI_DATA_TYPE_STRING};
use crate::debugging::debug_log_flags::{IM_GUI_DEBUG_LOG_FLAGS_EVENT_ACTIVE_ID, IM_GUI_DEBUG_LOG_FLAGS_EVENT_CLIPPER, IM_GUI_DEBUG_LOG_FLAGS_EVENT_DOCKING, IM_GUI_DEBUG_LOG_FLAGS_EVENT_FOCUS, IM_GUI_DEBUG_LOG_FLAGS_EVENT_IO, IM_GUI_DEBUG_LOG_FLAGS_EVENT_MASK, IM_GUI_DEBUG_LOG_FLAGS_EVENT_NAV, IM_GUI_DEBUG_LOG_FLAGS_EVENT_POPUP, IM_GUI_DEBUG_LOG_FLAGS_EVENT_VIEWPORT, IM_GUI_DEBUG_LOG_FLAGS_OUTPUT_TO_TTY};
use crate::dock_context_ops::clear_dock_context_nodes;
use crate::docking::dock_node::ImGuiDockNode;
use crate::docking::dock_node_flags::{ImGuiDockNodeFlags, ImGuiDockNodeFlags_HiddenTabBar, ImGuiDockNodeFlags_NoCloseButton, ImGuiDockNodeFlags_NoDocking, ImGuiDockNodeFlags_NoDockingOverEmpty, ImGuiDockNodeFlags_NoDockingOverMe, ImGuiDockNodeFlags_NoDockingOverOther, ImGuiDockNodeFlags_NoDockingSplitMe, ImGuiDockNodeFlags_NoDockingSplitOther, ImGuiDockNodeFlags_NoResize, ImGuiDockNodeFlags_NoResizeX, ImGuiDockNodeFlags_NoResizeY, ImGuiDockNodeFlags_NoSplit, ImGuiDockNodeFlags_NoTabBar, ImGuiDockNodeFlags_NoWindowMenuButton};
use crate::drag::DragFloat;
use crate::drawing::draw_cmd::ImDrawCmd;
use crate::drawing::draw_flags::ImDrawFlags_Closed;
use crate::drawing::draw_list::ImDrawList;
use crate::drawing::draw_list_flags::ImDrawListFlags_AntiAliasedLines;
use crate::draw_list_ops::{GetForegroundDrawList, GetForegroundDrawList2};
use crate::drawing::draw_vert::ImguiDrawVertex;
use crate::font::ImFont;
use crate::font_atlas::ImFontAtlas;
use crate::font::font_glyph::ImFontGlyph;
use crate::font::font_ops::{PopFont, PushFont};
use crate::widgets::hovered_flags::ImGuiHoveredFlags_DelayShort;
use crate::core::id_ops::pop_win_id_from_stack;
use crate::image_ops::Image;
use crate::imgui::GImGui;
use crate::ImguiViewport;
use crate::input_num_ops::InputText;
use crate::input_ops::{GetInputSourceName, IsKeyDown, IsKeyPressed, IsMouseClicked, IsMouseHoveringRect, SetMouseCursor};
use crate::input_text::InputTextMultiline;
use crate::input_text_flags::ImGuiInputTextFlags_ReadOnly;
use crate::io::IoContext;
use crate::io::io_ops::GetIO;
use crate::item::item_ops::{IsItemHovered, SetNextItemWidth};
use crate::io::key::{ImGuiKey_C, ImGuiKey_Escape, ImGuiKey_ModCtrl, ImGuiKey_NamedKey_BEGIN, ImGuiKey_NamedKey_END};
use crate::layout::layout_ops::{AlignTextToFramePadding, Dummy, same_line};
use crate::list_clipper::ImGuiListClipper;
use crate::core::math_ops::{ImFmod, ImMin, ImSqrt};
use crate::io::mod_flags::{ImGuiModFlags_Ctrl, ImGuiModFlags_Shift};
use crate::io::mouse_cursor::ImGuiMouseCursor_Hand;
use crate::nav_layer::ImGuiNavLayer_COUNT;
use crate::window::next_window_data_flags::ImGuiNextWindowDataFlags_HasSize;
use crate::table::old_columns::ImGuiOldColumns;
use crate::rect::ImRect;
use crate::drawing::render_ops::FindRenderedTextEnd;
use crate::widgets::scrolling_ops::{GetScrollMaxY, GetScrollY, SetScrollHereY};
use crate::widgets::separator::Separator;
use crate::settings_ops::{ClearIniSettings, FindWindowSettings, save_ini_settings_to_disk};
use crate::core::stack_level_info::ImGuiStackLevelInfo;
use crate::core::stack_tool::ImGuiStackTool;
use crate::core::storage::ImGuiStorage;
use crate::core::string_ops::{ImFormatString, ImTextCharFromUtf8, ImTextCharToUtf8};
use crate::style::ImguiStyle;
use crate::style_ops::{GetColorU32, GetStyle, GetStyleColorVec4, PopStyleColor, PushStyleColor};
use crate::style_var::ImGuiStyleVar_FramePadding;
use crate::widgets::tab_bar::ImGuiTabBar;
use crate::widgets::tab_item::ImGuiTabItem;
use crate::table::ImGuiTable;
use crate::table_bg_target::ImGuiTableBgTarget_CellBg;
use crate::table_column::ImGuiTableColumn;
use crate::table_column_flags::{ImGuiTableColumnFlags_WidthFixed, ImGuiTableColumnFlags_WidthStretch};
use crate::table_flags::{ImGuiTableFlags_Borders, ImGuiTableFlags_RowBg, ImGuiTableFlags_SizingFixedFit};
use crate::table_ops::TableGetInstanceData;
use crate::tables::{BeginTable, DebugNodeTable, EndTable, GetColumnOffsetFromNorm, TableHeadersRow, TableNextColumn, TableSetBgColor, TableSetupColumn};
use crate::text_ops::{BulletText, CalcTextSize, GetTextLineHeight, Text, TextColored, TextDisabled, TextUnformatted};
use crate::widgets::tooltip_ops::{BeginTooltip, EndTooltip};
use crate::widgets::tree_node_flags::{ImGuiTreeNodeFlags_None, ImGuiTreeNodeFlags_Selected};
use crate::core::type_defs::{ImguiHandle, ImGuiTableColumnIdx};
use crate::core::utils::{flag_clear, flag_set, GetVersion};
use crate::core::vec2::Vector2;
use crate::core::vec4::ImVec4;
use crate::viewport::viewport_flags::{ImguiViewportFlags_CanHostOtherWindows, ImguiViewportFlags_IsPlatformMonitor, ImguiViewportFlags_Minimized, ImguiViewportFlags_NoAutoMerge, ImguiViewportFlags_NoDecoration, ImguiViewportFlags_NoFocusOnAppearing, ImguiViewportFlags_NoFocusOnClick, ImguiViewportFlags_NoInputs, ImguiViewportFlags_NoRendererClear, ImguiViewportFlags_NoTaskBarIcon, ImguiViewportFlags_OwnedByApp, ImguiViewportFlags_TopMost};
use crate::viewport::viewport_ops::GetMainViewport;
use crate::viewport::widget_ops::{PopTextWrapPos, PushTextWrapPos};
use crate::widgets::{GetTreeNodeToLabelSpacing, Selectable, SetNextItemOpen, TabBarQueueReorder, TreeNode, TreeNodeEx, TreeNodeEx2, TreePop};
use crate::window::find::FindWindowByID;
use crate::window::ImguiWindow;
use crate::window::ops::{Begin, BeginDisabled, End, EndDisabled, GetCurrentWindow, SetNextWindowSize};
use crate::window::props::{GetFont, GetFontSize, GetWindowDrawList, SetNextWindowBgAlpha};
use crate::window::window_flags::{ImGuiWindowFlags, ImGuiWindowFlags_AlwaysAutoResize, ImGuiWindowFlags_AlwaysHorizontalScrollbar, ImGuiWindowFlags_AlwaysVerticalScrollbar, ImGuiWindowFlags_ChildMenu, ImGuiWindowFlags_ChildWindow, ImGuiWindowFlags_Modal, ImGuiWindowFlags_NoMouseInputs, ImGuiWindowFlags_NoNavInputs, ImGuiWindowFlags_NoSavedSettings, ImGuiWindowFlags_Popup, ImGuiWindowFlags_Tooltip};
use crate::window::window_settings::ImGuiWindowSettings;

// [DEBUG] Stack tool: hooks called by GetID() family functions
// c_void DebugHookIdInfo(ImguiHandle id, data_type: ImGuiDataType, data_id: *const c_void, data_id_end: *const c_void)
pub fn DebugHookIdInfo(g: &mut AppContext, id: ImguiHandle, data_type: ImGuiDataType, data_id: Option<&[u8]>) {
    // let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = &mut g.CurrentWindow;
    let mut tool = &mut g.DebugStackTool;

    // Step 0: stack query
    // This assume that the ID was computed with the current ID stack, which tends to be the case for our widget.
    if tool.StackLevel == -1 {
        tool.StackLevel += 1;
        tool.Results.resize(window.id_stack.len() + 1, ImGuiStackLevelInfo::default());
        // for (let n: c_int = 0; n < window.id_stack.Size + 1; n++)
        for n in 0..window.id_stack.len() + 1 {
            tool.Results[n].ID = if n < window.id_stack.len() {
                window.id_stack[n]
            } else { id };
        }
        return;
    }

    // Step 1+: query for individual level
    // IM_ASSERT(tool.StackLevel >= 0);
    if tool.StackLevel != window.id_stack.len() as c_int {
        return;
    }
    let mut info: *mut ImGuiStackLevelInfo = &mut tool.Results[tool.StackLevel];
    // IM_ASSERT(info.ID == id && info.QueryFrameCount > 0);

    match data_type {
        ImGuiDataType_S32 => {
            // let fmt_1 = format!("{}", data_id);
            // let cstr_fmt_1 = CStr::from_bytes_with_nul_unchecked(fmt_1.as_bytes());
            // ImFormatString(info.Desc.as_mut_ptr(), IM_ARRAYSIZE(info.Desc), cstr_fmt_1.as_ptr());
            todo!()
        },
        IM_GUI_DATA_TYPE_STRING => {
            // let raw_str_1 = if data_id_end.is_null() == false { dat_id_end - data_id } else {
            //     libc::strlen(data_id);
            // };
            // let data_id_cstr: CStr = Cstr::from_ptr(data_id);
            // let data_id_str = data_id_cstr.to_str().unwrap();
            //
            // ImFormatString(info.Desc.as_mut_ptr(), IM_ARRAYSIZE(info.Desc), data_id);
            todo!()
        },
        ImGuiDataType_Pointer => {
            // ImFormatString(info.Desc.as_mut_ptr(), IM_ARRAYSIZE(info.Desc), "0x{}", data_id);
            todo!()
        },

        ImGuiDataType_ID => {
            if (info.Desc[0] != 0) { // PushOverrideID() is often used to avoid hashing twice, which would lead to 2 calls to DebugHookIdInfo(). We prioritize the first one.
                return;
            }
            // ImFormatString(info.Desc, IM_ARRAYSIZE(info.Desc), "0x{} [override]", id);
            todo!()
        },

        _ => {
            todo!()
        }
    };
    info.QuerySuccess = true;
    info.DataType = data_type;
}



// Helper function to verify ABI compatibility between caller code and compiled version of Dear ImGui.
// Verify that the type sizes are matching between the calling file's compilation unit and imgui.cpp's compilation unit
// If this triggers you have an issue:
// - Most commonly: mismatched headers and compiled code version.
// - Or: mismatched configuration #define, compilation settings, packing pragma etc.
//   The configuration settings mentioned in imconfig.h must be set for all compilation units involved with Dear ImGui,
//   which is way it is required you put them in your imconfig file (and not just before including imgui.h).
//   Otherwise it is possible that different compilation units would see different structure layout
pub unsafe fn DebugCheckVersionAndDataLayout(version: *const c_char, sz_io: size_t, sz_style: size_t, sz_vec2: size_t, sz_vec4: size_t, sz_vert: size_t, sz_idx: size_t) -> bool
{
    // let mut error: bool =  false;
    // if (libc::strcmp(version, IMGUI_VERSION) != 0) { error = true; IM_ASSERT(libc::strcmp(version, IMGUI_VERSION) == 0 && "Mismatched version string!"); }
    // if (sz_io != sizeof(ImGuiIO)) { error = true; IM_ASSERT(sz_io == sizeof(ImGuiIO) && "Mismatched struct layout!"); }
    // if (sz_style != sizeof(ImGuiStyle)) { error = true; IM_ASSERT(sz_style == sizeof(ImGuiStyle) && "Mismatched struct layout!"); }
    // if (sz_vec2 != sizeof(ImVec2)) { error = true; IM_ASSERT(sz_vec2 == sizeof(ImVec2) && "Mismatched struct layout!"); }
    // if (sz_vec4 != sizeof(ImVec4)) { error = true; IM_ASSERT(sz_vec4 == sizeof(ImVec4) && "Mismatched struct layout!"); }
    // if (sz_vert != sizeof(ImDrawVert)) { error = true; IM_ASSERT(sz_vert == sizeof(ImDrawVert) && "Mismatched struct layout!"); }
    // if (sz_idx != sizeof) { error = true; IM_ASSERT(sz_idx == sizeof && "Mismatched struct layout!"); }
    // return !error;
    todo!()
}



//-----------------------------------------------------------------------------
// [SECTION] METRICS/DEBUGGER WINDOW
//-----------------------------------------------------------------------------
// - RenderViewportThumbnail() [Internal]
// - RenderViewportsThumbnails() [Internal]
// - DebugTextEncoding()
// - MetricsHelpMarker() [Internal]
// - ShowFontAtlas() [Internal]
// - ShowMetricsWindow()
// - DebugNodeColumns() [Internal]
// - DebugNodeDockNode() [Internal]
// - DebugNodeDrawList() [Internal]
// - DebugNodeDrawCmdShowMeshAndBoundingBox() [Internal]
// - DebugNodeFont() [Internal]
// - DebugNodeFontGlyph() [Internal]
// - DebugNodeStorage() [Internal]
// - DebugNodeTabBar() [Internal]
// - DebugNodeViewport() [Internal]
// - DebugNodeWindow() [Internal]
// - DebugNodeWindowSettings() [Internal]
// - DebugNodeWindowsList() [Internal]
// - DebugNodeWindowsListByBeginStackParent() [Internal]
//-----------------------------------------------------------------------------

// #ifndef IMGUI_DISABLE_DEBUG_TOOLS

pub fn DebugRenderViewportThumbnail(g: &mut AppContext, draw_list: &mut ImDrawList, viewport: &mut ImguiViewport, bb: &mut ImRect)
{
    // let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = &mut g.CurrentWindow;

    let mut scale = bb.GetSize() / viewport.Size;
    let off = bb.min - viewport.Pos * scale;
    let alpha_mul: c_float =  if flag_set(viewport.Flags, ImguiViewportFlags_Minimized) { 0.3 } else { 1.0 };
    window.DrawList.AddRectFilled(&bb.min, &bb.max, GetColorU32(ImGuiCol_Border, alpha_mul * 0.4), 0.0, 0);
    // for (let i: c_int = 0; i != g.Windows.len(); i++)
    // for i in 0 .. g.Windows.len()
    for win in g.Windows.iter()
    {
        // let mut thumb_window =  g.Windows[i];
        if !win.WasActive || flag_set(win.Flags, ImGuiWindowFlags_ChildWindow) {
            continue;
        }
        if win.Viewport != viewport {
            continue;
        }

        let mut thumb_r: ImRect =  win.Rect();
        let mut title_r: ImRect =  win.TitleBarRect();
        thumb_r = ImRect(ImFloor(off + thumb_r.min * scale), ImFloor(off +  thumb_r.max * scale));
        title_r = ImRect(ImFloor(off + title_r.min * scale), ImFloor(off +  Vector2::from_floats(title_r.max.x, title_r.min.y) * scale) + Vector2::from_ints(0, 5)); // Exaggerate title bar height
        thumb_r.ClipWithFull(bb);
        title_r.ClipWithFull(bb);
        let window_is_focused: bool = (g.NavWindow.is_null() == fallse && thumb_window.RootWindowForTitleBarHighlight == g.NavWindow.RootWindowForTitleBarHighlight);
        window.DrawList.AddRectFilled(&thumb_r.min, &thumb_r.max, GetColorU32(ImGuiCol_WindowBg, alpha_mul), 0.0, 0);
        window.DrawList.AddRectFilled(&title_r.min, &title_r.max, GetColorU32(if window_is_focused { ImGuiCol_TitleBgActive } else { ImGuiCol_TitleBg }, alpha_mul), f, 0);
        window.DrawList.AddRect(&thumb_r.min, &thumb_r.max, GetColorU32(ImGuiCol_Border, alpha_mul), 0.0);
        //         window.DrawList.AddText(g.Font, g.FontSize * 1.0, title_r.Min, GetColorU32(ImGuiCol_Text, alpha_mul), thumb_window.Name, FindRenderedTextEnd(thumb_window.Name));
        // window.DrawList.AddText2(g.Font, GetColorU32(ImGuiCol_Text, alpha_mul), thumb_window.Name.as_str());
        window.DrawList.AddText2(Some(&g.Font), g.FontSize, &title_r.min, GetColorU32(ImGuiCol_Text, alpha_mul), thumb_window.Name, 0.0, None)
    }
    draw_list.AddRect(bb.min, bb.max, GetColorU32(ImGuiCol_Border, alpha_mul), 0.0);
}

pub fn RenderViewportsThumbnails(g: &mut AppContext)
{
    let mut window = &mut g.CurrentWindow;
    // We don't display full monitor bounds (we could, but it often looks awkward), instead we display just enough to cover all of our viewports.
    let SCALE: c_float =  1.0 / 8.0;
    let mut bb_full: ImRect = ImRect::new(f32::MAX, f32::MAX, -f32::MAX, -f32::MAX);
    // for (let n: c_int = 0; n < g.Viewports.len(); n++)
    for n in 0 .. g.Viewports.len()
    {
        bb_full.Add(&g.Viewports[n].GetMainRect().GetSize());
    }
    let p: Vector2 = window.dc.cursor_pos;
    let off: Vector2 = p - bb_full.min * SCALE;
    // for (let n: c_int = 0; n < g.Viewports.len(); n++)
    for n in 0 .. g.Viewports.len()
    {
        let mut viewport =  &mut g.Viewports[n];
        let mut viewport_draw_bb: ImRect = ImRect::new(off + (viewport.Pos) * SCALE, off + (viewport.Pos + viewport.Size) * SCALE);
        DebugRenderViewportThumbnail(g, &mut window.DrawList, viewport, &mut viewport_draw_bb);
    }
    Dummy(g, bb_full.GetSize() * SCALE);
}

pub fn ViewportComparerByFrontMostStampCount(lhs: &ImguiViewport, rhs: &ImguiViewport) -> c_int
{
    rhs.last_front_most_stamp_count - lhs.last_front_most_stamp_count
}

// Helper tool to diagnose between text encoding issues and font loading issues. Pass your UTF-8 string and verify that there are correct.
pub fn debug_text_encoding(txt: &String)
{
    Text(format!("Text: \"{}\"",txt));
    if !BeginTable("list", 4, ImGuiTableFlags_Borders | ImGuiTableFlags_RowBg | ImGuiTableFlags_SizingFixedFit, None, 0.0) { return ; }
    TableSetupColumn(String::from("Offset"), 0, 0.0, 0);
    TableSetupColumn(String::from("UTF-8"), 0, 0.0, 0);
    TableSetupColumn(String::from("Glyph"), 0, 0.0, 0);
    TableSetupColumn(String::from("Codepoint"), 0, 0.0, 0);
    TableHeadersRow();
    // for (p: *const c_char = str; *p != 0; )
    // {
    //     c: c_uint;
    //     let c_utf8_len: c_int = ImTextCharFromUtf8(&c, p, null_mut());
    //     TableNextColumn();
    //     Text("{}", (p - str));
    //     TableNextColumn();
    //     for (let byte_index: c_int = 0; byte_index < c_utf8_len; byte_index++)
    //     {
    //         if byte_index > 0 {
    //             SameLine(); }
    //         Text("0x{:02X}", p[byte_index]);
    //     }
    //     TableNextColumn();
    //     if (GetFont()->FindGlyphNoFallback(c)){
    //     TextUnformatted(p, p + c_utf8_len);
    // }
    //     else{
    //     TextUnformatted((c == IM_UNICODE_CODEPOINT_INVALID)? "[invalid]": "[missing]");
    // }
    //     TableNextColumn();
    //     Text("U+{}", c);
    //     p += c_utf8_len;
    // }
    EndTable();
}

// Avoid naming collision with imgui_demo.cpp's HelpMarker() for unity builds.
pub unsafe fn MetricsHelpMarker(desc: String)
{
    TextDisabled(String::from("(?)"));
    if IsItemHovered(ImGuiHoveredFlags_DelayShort)
    {
        BeginTooltip();
        PushTextWrapPos(GetFontSize() * 35.0);
        TextUnformatted(desc);
        PopTextWrapPos();
        EndTooltip();
    }
}

// [DEBUG] List fonts in a font atlas and display its texture
pub unsafe fn ShowFontAtlas(atlas: &mut ImFontAtlas) {
    // for (let i: c_int = 0; i < atlas->Fonts.Size; i++)
    for i in 0..atlas.Fonts.len() {
        let mut font = atlas.Fonts[i].clone();
        PushID(font);
        DebugNodeFont(&mut font);
        pop_win_id_from_stack(g);
    }
    if TreeNode(String::from("Atlas texture"), format!("Atlas texture ({}x{} pixels)", atlas.TexWidth, atlas.TexHeight)) {
        let tint_col = ImVec4::from_floats(1.0, 1.0, 1.0, 1.0);
        let border_col = ImVec4::from_floats(1.0, 1.0, 1.0, 0.5);
        Image(atlas.TexID, &Vector2::from_usizes(atlas.TexWidth, atlas.TexHeight), &Vector2::from_floats(0.0, 0.0), &Vector2::from_floats(1.0, 1.0), tint_col, border_col);
        TreePop();
    }
}

// enum { WRT_OuterRect, WRT_OuterRectClipped, WRT_InnerRect, WRT_InnerClipRect, WRT_WorkRect, WRT_Content, WRT_ContentIdeal, WRT_ContentRegionRect, WRT_Count };
pub type WRT = i32;
pub const WRT_OuterRect: WRT = 0;
pub const WRT_OuterRectClipped: WRT = 1;
pub const WRT_InnterRect: WRT = 2;
pub const WRT_InnerClipRect: WRT = 3;
pub const WRT_WorkRect: WRT = 4;
pub const WRT_Content: WRT = 5;
pub const WRT_ContentIdeal: WRT = 6;
pub const WRT_ContentRegionRect: WRT = 7;
pub const WRT_Count: WRT = 8;


// enum { TRT_OuterRect, TRT_InnerRect, TRT_WorkRect, TRT_HostClipRect, TRT_InnerClipRect, TRT_BackgroundClipRect, TRT_ColumnsRect, TRT_ColumnsWorkRect, TRT_ColumnsClipRect, TRT_ColumnsContentHeadersUsed, TRT_ColumnsContentHeadersIdeal, TRT_ColumnsContentFrozen, TRT_ColumnsContentUnfrozen, TRT_Count };
pub type TRT = i32;
pub const TRT_OuterRect: TRT = 0;
pub const TRT_InnerRect: TRT = 1;
pub const TRT_WorkRect: TRT = 2;
pub const TRT_HostClipRect: TRT = 3;
pub const TRT_InnerClipRect: TRT = 4;
pub const TRT_BackgroundClipRect: TRT = 5;
pub const TRT_ColumnsRect: TRT = 6;
pub const TRT_ColumnsWorkRect: TRT = 7;
pub const TRT_ColumnsClipRect: TRT = 8;
pub const TRT_ColumnsContentHeadersUsed: TRT = 9;
pub const TRT_ColumnsContentHeadersIdeal: TRT = 10;
pub const TRT_ColumnsContentFrozen: TRT = 11;
pub const TRT_ColumnsContentUnfrozen: TRT = 12;
pub const TRT_Count: TRT = 13;



pub const wrt_rects_names: [&'static str;8] = [ "OuterRect", "OuterRectClipped", "InnerRect", "InnerClipRect", "WorkRect", "Content", "ContentIdeal", "ContentRegionRect" ];
     // Tables Rect Type
    pub const trt_rects_names: [&'startic str;13] = [ "OuterRect", "InnerRect", "WorkRect", "HostClipRect", "InnerClipRect", "BackgroundClipRect", "ColumnsRect", "ColumnsWorkRect", "ColumnsClipRect", "ColumnsContentHeadersUsed", "ColumnsContentHeadersIdeal", "ColumnsContentFrozen", "ColumnsContentUnfrozen" ];


#[derive(Default,Debug,Clone)]
    struct Funcs {

    }

    impl Funcs
    {
        pub unsafe fn GetTableRect(table: &mut ImGuiTable, rect_type: i32, n: i32) -> ImRect {
            let table_instance = TableGetInstanceData(table, table.InstanceCurrent); // Always using last submitted instance
            if rect_type == TRT_OuterRect { return table.OuterRect; } else if rect_type == TRT_InnerRect { return table.InnerRect; } else if rect_type == TRT_WorkRect { return table.WorkRect; } else if rect_type == TRT_HostClipRect { return table.HostClipRect; } else if rect_type == TRT_InnerClipRect { return table.InnerClipRect; } else if rect_type == TRT_BackgroundClipRect { return table.BgClipRect; } else if rect_type == TRT_ColumnsRect {
                let c = &table.Columns[n];
                return ImRect::from_floats(c.MinX, table.InnerClipRect.min.y, c.MaxX, table.InnerClipRect.min.y + table_instance.LastOuterHeight);
            } else if rect_type == TRT_ColumnsWorkRect {
                let c = &table.Columns[n];
                return ImRect::from_floats(c.WorkMinX, table.WorkRect.min.y, c.WorkMaxX, table.WorkRect.max.y);
            } else if rect_type == TRT_ColumnsClipRect {
                let c = &table.Columns[n];
                return c.ClipRect;
            } else if rect_type == TRT_ColumnsContentHeadersUsed {
                let c = &table.Columns[n];
                return ImRect::from_floats(c.WorkMinX, table.InnerClipRect.min.y, c.ContentMaxXHeadersUsed, table.InnerClipRect.min.y + table_instance.LastFirstRowHeight);
            } // Note: y1/y2 not always accurate
            else if rect_type == TRT_ColumnsContentHeadersIdeal {
                let c = &table.Columns[n];
                return ImRect::from_floats(c.WorkMinX, table.InnerClipRect.min.y, c.ContentMaxXHeadersIdeal, table.InnerClipRect.min.y + table_instance.LastFirstRowHeight);
            } else if rect_type == TRT_ColumnsContentFrozen {
                let c = &table.Columns[n];
                return ImRect::from_floats(c.WorkMinX, table.InnerClipRect.min.y, c.ContentMaxXFrozen, table.InnerClipRect.min.y + table_instance.LastFirstRowHeight);
            } else if rect_type == TRT_ColumnsContentUnfrozen {
                let c = &table.Columns[n];
                return ImRect::from_floats(c.WorkMinX, table.InnerClipRect.min.y + table_instance.LastFirstRowHeight, c.ContentMaxXUnfrozen, table.InnerClipRect.max.y);
            }
            // IM_ASSERT(0);
            return ImRect::default();
        }

        pub unsafe fn GetWindowRect(window: &mut ImguiWindow, rect_type: i32) -> ImRect
        {
            if rect_type == WRT_OuterRect { return window.Rect(); }
            else if (rect_type == WRT_OuterRectClipped)     { return window.OuterRectClipped; }
            else if (rect_type == WRT_InnerRect)            { return window.InnerRect; }
            else if (rect_type == WRT_InnerClipRect)        { return window.InnerClipRect; }
            else if (rect_type == WRT_WorkRect)             { return window.work_rect; }
            else if (rect_type == WRT_Content)       { let mut min =  window.InnerRect.min - window.scroll + window.WindowPadding; return ImRect::from_vec2(min, min + window.ContentSize); }
            else if (rect_type == WRT_ContentIdeal)         { let mut min =  window.InnerRect.min - window.scroll + window.WindowPadding; return ImRect::from_vec2(min, min + window.ContentSizeIdeal); }
            else if (rect_type == WRT_ContentRegionRect)    { return window.content_region_rect; }
            // IM_ASSERT(0);
            return ImRect::default();
        }
    }

pub unsafe fn ShowMetricsWindow(p_open: &mut bool)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let io = &g.IO;
    let cfg = &mut g.DebugMetricsConfig;
    if cfg.ShowDebugLog {
        ShowDebugLogWindow(&mut cfg.ShowDebugLog);
    }
    if cfg.ShowStackTool {
        ShowStackToolWindow(&mut cfg.ShowStackTool);
    }

    if !Begin(g, "Dear ImGui Metrics/Debugger", Some(p_open)) || GetCurrentWindow().BeginCount > 1
    {
        End();
        return;
    }

    // Basic info
    Text(format!("Dear ImGui {}", GetVersion()));
    Text(format!("Application average {} ms/frame ({} FPS)", 1000 / io.Framerate, io.Framerate));
    Text(format!("{} vertices, {} indices ({} triangles)", io.MetricsRenderVertices, io.MetricsRenderIndices, io.MetricsRenderIndices / 3));
    Text(format!("{} visible windows, {} active allocations", io.MetricsRenderWindows, io.MetricsActiveAllocations));
    //SameLine(); if (SmallButton("GC")) { g.GcCompactAll = true; }

    Separator();

    // Debugging enums
    // Windows Rect Type

    if cfg.ShowWindowsRectsType < 0 {
        cfg.ShowWindowsRectsType = WRT_WorkRect;
    }
    if cfg.ShowTablesRectsType < 0 {
        cfg.ShowTablesRectsType = TRT_WorkRect;
    }



    // Tools
    if TreeNode(String::from("Tools"), String::from(""))
    {
        let mut show_encoding_viewer: bool =  TreeNode(String::from("UTF-8 Encoding viewer"), String::from(""));
        same_line(g, 0.0, 0.0);
        MetricsHelpMarker(String::from("You can also call DebugTextEncoding() from your code with a given string to test that your UTF-8 encoding settings are correct."));
        if show_encoding_viewer
        {
            let mut buf = String::with_capacity(100);
            SetNextItemWidth(-FLT_MIN);
            InputText(String::from("##Text"), &mut buf, buf.len(), 0, None, None);
            if buf[0] != 0 {
                debug_text_encoding(buf.as_str());
            }
            TreePop();
        }

        // The Item Picker tool is super useful to visually select an item and break into the call-stack of where it was submitted.
        if Checkbox(String::from("Show Item Picker"), &mut g.DebugItemPickerActive) && g.DebugItemPickerActive {
            DebugStartItemPicker();
        }
        same_line(g, 0.0, 0.0);
        MetricsHelpMarker(String::from("Will call the IM_DEBUG_BREAK() macro to break in debugger.\nWarning: If you don't have a debugger attached, this will probably crash."));

        // Stack Tool is your best friend!
        Checkbox(String::from("Show Debug Log"), &mut cfg.ShowDebugLog);
        same_line(g, 0.0, 0.0);
        MetricsHelpMarker(String::from("You can also call ShowDebugLogWindow() from your code."));

        // Stack Tool is your best friend!
        Checkbox(String::from("Show Stack Tool"), &mut cfg.ShowStackTool);
        same_line(g, 0.0, 0.0);
        MetricsHelpMarker(String::from("You can also call ShowStackToolWindow() from your code."));

        Checkbox(String::from("Show windows begin order"), &mut cfg.ShowWindowsBeginOrder);
        Checkbox(String::from("Show windows rectangles"), &mut cfg.ShowWindowsRects);
        same_line(g, 0.0, 0.0);
        SetNextItemWidth(GetFontSize() * 12);
        let data: [String;8] = [
            String::from(wrt_rects_names[0]),
            String::from(wrt_rects_names[1]),
            String::from(wrt_rects_names[2]),
            String::from(wrt_rects_names[3]),
            String::from(wrt_rects_names[4]),
            String::from(wrt_rects_names[5]),
            String::from(wrt_rects_names[6]),
            String::from(wrt_rects_names[7])];
        cfg.ShowWindowsRects |= Combo2(String::from("##show_windows_rect_type"), &mut cfg.ShowWindowsRectsType, &data, WRT_Count as usize, WRT_Count);
        if cfg.ShowWindowsRects && g.NavWindow.is_some()
        {
            BulletText(format!("'{}':", g.NavWindow.Name));
            indent(0.0, g);
            // for (let rect_n: c_int = 0; rect_n < WRT_Count; rect_n++)
            for rect_n in 0 .. WRT_Count
            {
                let mut r: ImRect =  Funcs::GetWindowRect(&mut g.NavWindow.unwrap(), rect_n);
                Text(format!("({},{}) ({},{}) Size ({},{}) {}", r.min.x, r.min.y, r.max.x, r.max.y, r.GetWidth(), r.GetHeight(), wrt_rects_names[rect_n]));
            }
            unindent(g, 0.0);
        }

        Checkbox(String::from("Show tables rectangles"), &mut cfg.ShowTablesRects);
        same_line(g, 0.0, 0.0);
        SetNextItemWidth(GetFontSize() * 12);
        let trt_data: [String;13] = [
            String::from(trt_rects_names[0]),
            String::from(trt_rects_names[1]),
            String::from(trt_rects_names[2]),
            String::from(trt_rects_names[3]),
            String::from(trt_rects_names[4]),
            String::from(trt_rects_names[5]),
            String::from(trt_rects_names[6]),
            String::from(trt_rects_names[7]),
            String::from(trt_rects_names[8]),
            String::from(trt_rects_names[9]),
            String::from(trt_rects_names[10]),
            String::from(trt_rects_names[11]),
            String::from(trt_rects_names[12])
        ];
        cfg.ShowTablesRects |= Combo2(String::from("##show_table_rects_type"), &mut cfg.ShowTablesRectsType, &trt_data, TRT_Count as usize, TRT_Count);
        if cfg.ShowTablesRects && g.NavWindow != None
        {
            // for (let table_n: c_int = 0; table_n < g.Tables.GetMapSize(); table_n++)
            for (_, table) in g.Tables.iter_mut()
            {
                // let table = g.Tables.get_key_value(table_n);
                if table.LastFrameActive < g.FrameCount - 1 || (table.OuterWindow != g.NavWindow.unwrap() && table.InnerWindow != g.NavWindow) {
                    continue;
                }

                BulletText(format!("Table {} ({} columns, in '{}')", table.ID, table.ColumnsCount, table.Outerwindow.Name));
                if IsItemHovered(0) {
                    GetForegroundDrawList2().AddRect(table.OuterRect.Min - Vector2::new(1, 1), table.OuterRect.Max + Vector2::new(1, 1), color_u32_from_rgba(255, 255, 0, 255), 0.0);
                }
                indent(0.0, g);
                // buf: [c_char;128];
                let mut buf = String::with_capacity(128);
                // for (let rect_n: c_int = 0; rect_n < TRT_Count; rect_n++)
                for rect_n in 0 .. TRT_COUNT
                {
                    if rect_n >= TRT_ColumnsRect
                    {
                        if rect_n != TRT_ColumnsRect && rect_n != TRT_ColumnsClipRect {
                            continue;
                        }
                        // for (let column_n: c_int = 0; column_n < table.ColumnsCount; column_n++)
                        for column_n in 0 .. table.ColumnsCount
                        {
                            let r =  Funcs::GetTableRect(table, rect_n, column_n as i32);
                            // ImFormatString(buf, buf.len(), "(%6.1f,%6.10.0) (%6.1f,%6.10.0) Size (%6.1f,%6.10.0) Col {} {}", r.Min.x, r.Min.y, r.Max.x, r.Max.y, r.GetWidth(), r.GetHeight(), column_n, trt_rects_names[rect_n]);
                            Selectable(buf.clone(), false, 0, None);
                            if IsItemHovered(0) {
                                GetForegroundDrawList2().AddRect(r.min - Vector2::new(1, 1), r.max + Vector2::new(1, 1), color_u32_from_rgba(255, 255, 0, 255), 0.0);
                            }
                        }
                    }
                    else
                    {
                        let r: ImRect =  Funcs::GetTableRect(table, rect_n, -1);
                        // ImFormatString(buf, buf.len(), "(%6.1f,%6.10.0) (%6.1f,%6.10.0) Size (%6.1f,%6.10.0) {}", r.Min.x, r.Min.y, r.Max.x, r.Max.y, r.GetWidth(), r.GetHeight(), trt_rects_names[rect_n]);
                        Selectable(buf.clone(), false, 0, None);
                        if IsItemHovered(0) {
                            GetForegroundDrawList2().AddRect(r.min - Vector2::new(1, 1), r.max + Vector2::new(1, 1), color_u32_from_rgba(255, 255, 0, 255), 0.0);
                        }
                    }
                }
                unindent(g, 0.0);
            }
        }

        TreePop();
    }

    pub fn WindowComparerByBeginOrder(lhs: &ImguiWindow, rhs: &ImguiWindow) ->c_int {
        // return ((*(*const ImGuiWindow const *)lhs).BeginOrderWithinContext - (*(*const ImGuiWindow const*)rhs).BeginOrderWithinContext);
        lhs.BeginOrderWithinContext - rhs.BeginOrderWithinContext
    }

    // Windows
    if TreeNode(String::from("Windows"), format!("Windows ({})", g.Windows.len()))
    {
        //SetNextItemOpen(true, ImGuiCond_Once);
        DebugNodeWindowsList(&mut g.Windows, String::from("By display order"));
        DebugNodeWindowsList(&mut g.WindowsFocusOrder, String::from("By focus order (root windows)"));
        if TreeNode(String::from("By submission order (begin stack)"), String::from(""))
        {
            // Here we display windows in their submitted order/hierarchy, however note that the Begin stack doesn't constitute a Parent<>Child relationship!
            let mut temp_buffer = &mut g.WindowsTempSortBuffer;
            temp_buffer.clear();
            // for (let i: c_int = 0; i < g.Windows.len(); i++)
            for win in g.Windows.iter()
            {
                if win.LastFrameActive + 1 >= g.FrameCount {
                    temp_buffer.push(g.Windows[i]);
                }
            }
            // struct Func { :  };
            // ImQsort(temp_buffer.Data, temp_buffer.Size, sizeof, Func::WindowComparerByBeginOrder);
            temp_buffer.sort_by(WindowComparerByBeginOrder);
            DebugNodeWindowsListByBeginStackParent(temp_buffer.Data, temp_buffer.Size);
            TreePop();
        }

        TreePop();
    }

    // DrawLists
    let mut drawlist_count: usize = 0;
    // for (let viewport_i: c_int = 0; viewport_i < g.Viewports.len(); viewport_i++){
    for viewport in g.Viewports.iter() {
        drawlist_count += veiwport.DrawDataBuilder.GetDrawListCount();
    }
    if TreeNode(String::from("DrawLists"), format!("DrawLists ({})", drawlist_count))
    {
        Checkbox(String::from("Show ImDrawCmd mesh when hovering"), &mut cfg.ShowDrawCmdMesh);
        Checkbox(String::from("Show ImDrawCmd bounding boxes when hovering"), &mut cfg.ShowDrawCmdBoundingBoxes);
        // for (let viewport_i: c_int = 0; viewport_i < g.Viewports.len(); viewport_i++)
        for viewport in g.Viewports.iter_mut()
        {
            // let mut viewport: *mut ImguiViewport =  g.Viewports[viewport_i];
            let mut viewport_has_drawlist: bool =  false;
            // for (let layer_i: c_int = 0; layer_i < IM_ARRAYSIZE(viewport.DrawDataBuilder.Layers); layer_i++)
            for layer in viewport.DrawDataBuilder.Layers.iter()
            {
                // for (let draw_list_i: c_int = 0; draw_list_i < viewport.DrawDataBuilder.Layers[layer_i].Size; draw_list_i+ +)
                for draw_list in layer.iter()
                {
                    if !viewport_has_drawlist {
                        Text(format!("Active DrawLists in Viewport #{}, ID: {}", viewport.Idx, viewport.ID));
                    }
                    viewport_has_drawlist = true;
                    DebugNodeDrawList(None, Some(viewport), draw_list, String::from("DrawList"));
                }
            }
        }
        TreePop();
    }

    // Viewports
    if TreeNode(String::from("Viewports"), format!("Viewports ({})", g.Viewports.len()))
    {
        indent(GetTreeNodeToLabelSpacing(), g);
        RenderViewportsThumbnails(g);
        unindent(g, GetTreeNodeToLabelSpacing());

        let mut open: bool =  TreeNode(String::from("Monitors"), format!("Monitors ({})", g.PlatformIO.Monitors.Size));
        same_line(g, 0.0, 0.0);
        MetricsHelpMarker(String::from("Dear ImGui uses monitor data:\n- to query DPI settings on a per monitor basis\n- to position popup/tooltips so they don't straddle monitors."));
        if open
        {
            // for (let i: c_int = 0; i < g.PlatformIO.Monitors.Size; i++)
            for mon in g.PlatformIO.Monitors.iter()
            {
                // const ImGuiPlatformMonitor& mon = g.PlatformIO.Monitors[i];
                BulletText(format!("Monitor #{}: DPI {}%\n MainMin ({}.0,{}.0), MainMax ({}.0,{}.0), MainSize ({}.0,{}.0)\n WorkMin ({}.0,{}.0), WorkMax ({}.0,{}.0), WorkSize ({}.0,{}.0)",
                                   i, mon.DpiScale * 100,
                                   mon.MainPos.x, mon.MainPos.y, mon.MainPos.x + mon.MainSize.x, mon.MainPos.y + mon.MainSize.y, mon.MainSize.x, mon.MainSize.y,
                                   mon.WorkPos.x, mon.WorkPos.y, mon.WorkPos.x + mon.WorkSize.x, mon.WorkPos.y + mon.WorkSize.y, mon.WorkSize.x, mon.WorkSize.y));
            }
            TreePop();
        }

        BulletText(format!("MouseViewport: {} (UserHovered {}, LastHovered {})", if g.MouseViewport { g.MouseViewport.ID }else{ 0}, g.IO.MouseHoveredViewport, if g.MouseLastHoveredViewport { g.MouseLastHoveredViewport.ID} else {0}));
        if TreeNode(String::from("Inferred Z order (front-to-back)"), String::from(""))
        {
            // static Vec<*mut ImguiViewportP> viewports;
            let mut viewports: Vec<ImguiViewport> = vec![];
            viewports.reserve(g.Viewports.len());
            // memcpy(viewports.Data, g.Viewports.Data, g.Viewports.size_in_bytes());
            viewports.clone_from_slice(&g.Viewports);
            if viewports.Size > 1 {
                // ImQsort(viewports.Data, viewports.Size, sizeof(ImguiViewport *), ViewportComparerByFrontMostStampCount);
                viewports.sort_by(ViewportComparerByFrontMostStampCount);
            }
            // for (let i: c_int = 0; i < viewports.Size; i++)
            for viewport in viewports.iter()
            {
                BulletText(format!("Viewport #{}, ID: 0x{}, FrontMostStampCount = %08d, Window: \"{}\"", viewports[i]->Idx, viewports[i].ID, viewports[i]->LastFrontMostStampCount, if viewports[i].Window { viewports[i] -> window.Name } else { "N/A" }));
            }
            TreePop();
        }

        // for (let i: c_int = 0; i < g.Viewports.len(); i++)
        for viewport in g.Viewports.iter_mut()
        {
            DebugNodeViewport(viewport);
        }
        TreePop();
    }

    // Details for Popups
    if TreeNode(String::from("Popups"), format!("Popups ({})", g.OpenPopupStack.len()))
    {
        // for (let i: c_int = 0; i < g.OpenPopupStack.len(); i++)
        for popup_data in g.OpenPopupStack.iter_mut()
        {
            // As it's difficult to interact with tree nodes while popups are open, we display everything inline.
            // let popup_data: *const ImGuiPopupData = &g.OpenPopupStack[i];
            let window =  &mut popup_data.Window;
            BulletText(format!("PopupID: {}, Window: '{}' ({}{}), BackupNavWindow '{}', ParentWindow '{}'",
                popup_data.PopupId, window ? window.Name : "NULL", window && flag_set(window.Flags, ImGuiWindowFlags_ChildWindow) ? "Child;" : "", window && flag_set(window.Flags, ImGuiWindowFlags_ChildMenu) ? "Menu;" : "",
                popup_data.BackupNavWindow ? popup_data.BackupNavwindow.Name : "NULL", window && window.ParentWindow ? window.Parentwindow.Name : "NULL"));
        }
        TreePop();
    }

    // Details for TabBars
    if TreeNode(String::from("TabBars"), format!("Tab Bars ({})", g.TabBars.GetAliveCount())) {
        // for (let n: c_int = 0; n < g.TabBars.GetMapSize(); n++)
        for tab_bar in g.TabBars.values_mut() {
            // if (tab_bar: &mut ImGuiTabBar = g.TabBars.TryGetMapData(n)) {
            PushID(tab_bar);
            DebugNodeTabBar(tab_bar, String::from("TabBar"));
            pop_win_id_from_stack(g);
            // }
        }
        TreePop();
    }

    // Details for Tables
    if TreeNode(String::from("Tables"), format!("Tables ({})", g.Tables.GetAliveCount()))
    {
        // for (let n: c_int = 0; n < g.Tables.GetMapSize(); n++)
        for table in g.Tables.values_mut()
        {
            // if (ImGuiTable * table = g.Tables.TryGetMapData(n)) {
                DebugNodeTable(table);
            // }
        }
        TreePop();
    }

    // Details for Fonts
    let atlas = &mut g.IO.Fonts;
    if TreeNode(String::from("Fonts"), format!("Fonts ({})", atlas.Fonts.Size))
    {
        ShowFontAtlas(atlas);
        TreePop();
    }

    // Details for InputText
    if TreeNode(String::from("InputText"), String::from(""))
    {
        DebugNodeInputTextState(&g.InputTextState);
        TreePop();
    }

    // Details for Docking
// #ifdef IMGUI_HAS_DOCK
    if TreeNode(String::from("Docking"), String::from(""))
    {
        let mut root_nodes_only =  true;
        let dc = &mut g.DockContext;
        Checkbox(String::from("List root nodes"), &mut root_nodes_only);
        Checkbox(String::from("Ctrl shows window dock info"), &mut cfg.ShowDockingNodes);
        if SmallButton(String::from("Clear nodes")) { clear_dock_context_nodes(g, 0, true); }
        same_line(g, 0.0, 0.0);
        if SmallButton(String::from("Rebuild all")) { dc.WantFullRebuild = true; }
        // for (let n: c_int = 0; n < dc.Nodes.Data.Size; n++)
        for node in dc.Nodes.iter_mut()
        {
            // if node: *mut ImGuiDockNode = dc.Nodes.Data[n].val_p {
                if !root_nodes_only || node.IsRootNode() {
                    DebugNodeDockNode(node, String::from("Node"));
                }
            // }
        }
        TreePop();
    }
// #endif // #ifdef IMGUI_HAS_DOCK

    // Settings
    if TreeNode(String::from("Settings"), String::from(""))
    {
        if SmallButton(String::from("Clear")) {
            ClearIniSettings();
        }
        same_line(g, 0.0, 0.0);
        if SmallButton(String::from("Save to memory")) {
            SaveIniSettingsToMemory();
        }
        same_line(g, 0.0, 0.0);
        if SmallButton(String::from("Save to disk")) {
            save_ini_settings_to_disk(g, g.IO.IniFilename);
        }
        same_line(g, 0.0, 0.0);
        if g.IO.IniFilename {
            Text(format!("\"{}\"", g.IO.IniFilename));
        }
        else {
            TextUnformatted(String::from("<NULL>"));
        }
        Text(format!("SettingsDirtyTimer {}", g.SettingsDirtyTimer));
        if TreeNode(String::from("SettingsHandlers"), format!("Settings handlers: ({})", g.SettingsHandlers.Size))
        {
            // for (let n: c_int = 0; n < g.SettingsHandlers.Size; n++)
            for handler in g.SettingsHandlers.iter()
            {
                BulletText(format!("{}", handler.TypeName));
            }
            TreePop();
        }
        if TreeNode(String::from("SettingsWindows"), format!("Settings packed data: Windows: {} bytes", g.SettingsWindows.size()))
        {
            // for (settings: *mut ImGuiWindowSettings = g.SettingsWindows.begin(); settings != None; settings = g.SettingsWindows.next_chunk(settings))
            for settings in g.SettingsWindow.iter_mut()
            {
                DebugNodeWindowSettings(settings);
            }
            TreePop();
        }

        if TreeNode(String::from("SettingsTables"), format!("Settings packed data: Tables: {} bytes", g.SettingsTables.size()))
        {
            // for (ImGuiTableSettings* settings = g.SettingsTables.begin(); settings != None; settings = g.SettingsTables.next_chunk(settings))
            for settings in g.SettingsTables.iter_mut()
            {
                DebugNodeTableSettings(settings);
            }
            TreePop();
        }

// #ifdef IMGUI_HAS_DOCK
        if TreeNode(String::from("SettingsDocking"), String::from("Settings packed data: Docking"))
        {
            let dc = &mut g.DockContext;
            Text(String::from("In SettingsWindows:"));
            // for (settings: *mut ImGuiWindowSettings = g.SettingsWindows.begin(); settings != None; settings = g.SettingsWindows.next_chunk(settings))
            for settings in g.SettingsWIndows.iter_mut()
            {
                if settings.DockId != 0 {
                    BulletText(format!("Window '{}' -> DockId {}", settings.GetName(), settings.DockId));
                }
            }
            Text(String::from("In SettingsNodes:"));
            // for (let n: c_int = 0; n < dc.NodesSettings.Size; n++)
            for settings in dc.NodeSettings.iter_mut()
            {
                // settings: *mut ImGuiDockNodeSettings = &dc.NodesSettings[n];
                let mut  selected_tab_name = String::default();
                if settings.SelectedTabId
                {
                    let window =  FindWindowByID(, settings.SelectedTabId);
                    if window.is_null() == false
                    {
                    selected_tab_name = window.Name.clone();
                    }
                    else {
                        let mut window_settings = FindWindowSettings(settings.SelectedTabId);
                        if window_settings.is_null() == false {
                            selected_tab_name = window_settings.GetName();
                        }
                    }
                }
                BulletText(format!("Node {}, Parent {}, SelectedTab {} ('{}')", settings.ID, settings.ParentNodeId, settings.SelectedTabId, if selected_tab_name.is_empty() == false { selected_tab_name} else {
                    if settings.SelectedTabId {
                        "N/A"
                    }else { "" }
                }));
            }
            TreePop();
        }
// #endif // #ifdef IMGUI_HAS_DOCK

        if TreeNode(String::from("SettingsIniData"), format!("Settings unpacked data (.ini): {} bytes", g.SettingsIniData.size()))
        {
            InputTextMultiline(String::from("##Ini"), &mut g.SettingsIniData, g.SettingsIniData.Buf.len(), &mut Vector2::from_floats(f32::MIN, GetTextLineHeight() * 20), ImGuiInputTextFlags_ReadOnly, None, None);
            TreePop();
        }
        TreePop();
    }

    // Misc Details
    if TreeNode(String::from("Internal state"), String::from(""))
    {
        Text(String::from("WINDOWING"));
        indent(0.0, g);
        Text(format!("HoveredWindow: '{}'", if g.HoveredWindow.is_some() { g.Hoveredwindow.unwrap().Name }else{ "NULL"}));
        Text(format!("Hoveredwindow.Root: '{}'", if g.HoveredWindow.is_some() { g.Hoveredwindow.unwrap().RootWindowDockTree.Name }else {"NULL"}));
        Text(format!("HoveredWindowUnderMovingWindow: '{}'", if g.HoveredWindowUnderMovingWindow.is_some() { g.HoveredWindowUnderMovingwindow.unwrap().Name} else {"NULL"}));
        Text(format!("HoveredDockNode: {}", if g.DebugHoveredDockNode.is_some() { g.DebugHoveredDockNode.unwrap().ID} else {0}));
        Text(format!("MovingWindow: '{}'", if g.MovingWindow.is_some() { g.Movingwindow.unwrap().Name }else {"NULL"}));
        Text(format!("MouseViewport: {} (UserHovered {}, LastHovered {})", g.MouseViewport.ID, g.IO.MouseHoveredViewport, if g.MouseLastHoveredViewport { g.MouseLastHoveredViewport.ID }else {0}));
        unindent(g, 0.0);

        Text(String::from("ITEMS"));
        indent(0.0, g);
        Text(format!("ActiveId: {}/{} ({} sec), AllowOverlap: {}, Source: {}", g.ActiveId, g.ActiveIdPreviousFrame, g.ActiveIdTimer, g.ActiveIdAllowOverlap, GetInputSourceName(g.ActiveIdSource)));
        Text(format!("ActiveIdWindow: '{}'", if g.ActiveIdWindow { g.ActiveIdwindow.Name} else {"NULL"}));

        let mut active_id_using_key_input_count: c_int = 0;
        // for (let n: c_int = ImGuiKey_NamedKey_BEGIN; n < ImGuiKey_NamedKey_END; n++)
        for n in ImGuiKey_NamedKey_BEGIN .. ImGuiKey_NamedKey_END
        {
            active_id_using_key_input_count += if g.ActiveIdUsingKeyInputMask[n] { 1 } else { 0 };
        }
        Text(format!("ActiveIdUsing: NavDirMask: {}, KeyInputMask: {} key(s)", g.ActiveIdUsingNavDirMask, active_id_using_key_input_count));
        Text(format!("HoveredId: {} ({} sec), AllowOverlap: {}", g.HoveredIdPreviousFrame, g.HoveredIdTimer, g.HoveredIdAllowOverlap)); // Not displaying g.HoveredId as it is update mid-frame
        Text(format!("HoverDelayId: {}, Timer: {}, ClearTimer: {}", g.HoverDelayId, g.HoverDelayTimer, g.HoverDelayClearTimer));
        Text(format!("DragDrop: {}, SourceId = {}, Payload \"{}\" ({} bytes)", g.DragDropActive, g.DragDropPayload.SourceId, g.DragDropPayload.DataType, g.DragDropPayload.DataSize));
        unindent(g, 0.0);

        Text(String::from("NAV,FOCUS"));
        indent(0.0, g);
        Text(format!("NavWindow: '{}'", if g.NavWindow.is_some() { g.NavWindow.unwrap().Name} else {"NULL"}));
        Text(format!("NavId: 0x{}, NavLayer: {}", g.NavId, g.NavLayer));
        Text(format!("NavInputSource: {}", GetInputSourceName(g.NavInputSource)));
        Text(format!("NavActive: {}, NavVisible: {}", g.IO.NavActive, g.IO.NavVisible));
        Text(format!("NavActivateId/DownId/PressedId/InputId: {}/{}/{}/{}", g.NavActivateId, g.NavActivateDownId, g.NavActivatePressedId, g.NavActivateInputId));
        Text(format!("NavActivateFlags: {}", g.NavActivateFlags));
        Text(format!("NavDisableHighlight: {}, NavDisableMouseHover: {}", g.NavDisableHighlight, g.NavDisableMouseHover));
        Text(format!("NavFocusScopeId = 0x{}", g.NavFocusScopeId));
        Text(format!("NavWindowingTarget: '{}'", if g.NavWindowingTarget { g.NavWindowingTarget.Name }else {"NULL"}));
        unindent(g, 0.0);

        TreePop();
    }

    // Overlay: Display windows Rectangles and Begin Order
    if cfg.ShowWindowsRects || cfg.ShowWindowsBeginOrder
    {
        // for (let n: c_int = 0; n < g.Windows.len(); n++)
        for window in g.Windows.iter_mut()
        {
            // let mut window: &mut ImGuiWindow =  g.Windows[n];
            if !window.WasActive {
                continue;
            }
            let mut  draw_list =  GetForegroundDrawList(Some(window));
            if cfg.ShowWindowsRects
            {
                let r: ImRect =  Funcs::GetWindowRect(window, cfg.ShowWindowsRectsType);
                draw_list.AddRect(r.min, r.max, color_u32_from_rgba(255, 0, 128, 255), 0.0);
            }
            if cfg.ShowWindowsBeginOrder && flag_clear(window.Flags, ImGuiWindowFlags_ChildWindow)
            {
                // buf: [c_char;32];
                let mut buf = String::with_capacity(32);
                // ImFormatString(buf, buf.len(), "{}", window.BeginOrderWithinContext);
                let font_size: c_float =  GetFontSize();
                draw_list.AddRectFilled(&window.position, window.position + Vector2::from_floats(font_size, font_size), color_u32_from_rgba(200, 100, 100, 255), 0.0, 0);
                draw_list.AddText(window.position, color_u32_from_rgba(255, 255, 255, 255), buf);
            }
        }
    }

    // Overlay: Display Tables Rectangles
    if cfg.ShowTablesRects
    {
        // for (let table_n: c_int = 0; table_n < g.Tables.GetMapSize(); table_n++)
        for table in g.Tables.values_mut()
        {
            // ImGuiTable* table = g.Tables.TryGetMapData(table_n);
            // if (table == None || table.LastFrameActive < g.FrameCount - 1)
            if table.LastFrameActive < g.FrameCount - 1
            {
                continue;
            }
            let mut  draw_list =  GetForegroundDrawList2();
            if cfg.ShowTablesRectsType >= TRT_ColumnsRect
            {
                // for (let column_n: c_int = 0; column_n < table.ColumnsCount; column_n++)
                for column_n in 0 .. table.ColumnsCount
                {
                    let r =  Funcs::GetTableRect(table, cfg.ShowTablesRectsType, column_n as i32);
                    let column = if table.HoveredColumnBody == column_n as ImGuiTableColumnIdx { color_u32_from_rgba(255, 255, 128, 255)} else { color_u32_from_rgba(255, 0, 128, 255)};
                    let thickness: c_float =  if table.HoveredColumnBody == column_n as ImGuiTableColumnIdx { 3.0} else {1.0};
                    draw_list.AddRect(r.min, r.max, column, 0.0);
                }
            }
            else
            {
                let r: ImRect =  Funcs::GetTableRect(table, cfg.ShowTablesRectsType, -1);
                draw_list.AddRect(r.min, r.max, color_u32_from_rgba(255, 0, 128, 255), 0.0);
            }
        }
    }

// #ifdef IMGUI_HAS_DOCK
    // Overlay: Display Docking info
    if cfg.ShowDockingNodes && g.IO.KeyCtrl && g.DebugHoveredDockNode.is_some()
    {
        // buf: [c_char;64] = "";
        let buf = String::with_capacity(64);
        // char* p = buf;
        let node = &mut g.DebugHoveredDockNode;
        let mut  overlay_draw_list =  if node.HostWindow { GetForegroundDrawList(node.HostWindow)} else{ GetForegroundDrawList(Some(GetMainViewport()))};
        // p += ImFormatString(p, buf + buf.len() - p, "DockId: {}{}\n", node.ID, if node.IsCentralNode() { " *CentralNode*"}else{ ""});
        // p += ImFormatString(p, buf + buf.len() - p, "WindowClass: {}\n", node.WindowClass.ClassId);
        // p += ImFormatString(p, buf + buf.len() - p, "Size: ({}, {})\n", node.Size.x, node.Size.y);
        // p += ImFormatString(p, buf + buf.len() - p, "SizeRef: ({}, {})\n", node.SizeRef.x, node.SizeRef.y);
        let depth: c_int = DockNodeGetDepth(node);
        overlay_draw_list.AddRect(node.Pos + Vector2::new(3, 3) * depth, node.Pos + node.Size - Vector2::new(3, 3) * depth, color_u32_from_rgba(200, 100, 100, 255), 0.0);
        let pos: Vector2 = node.Pos + Vector2::new(3, 3) * depth;
        overlay_draw_list.AddRectFilled(pos - Vector2::new(1, 1), pos + CalcTextSize(, buf, false, 0.0) + Vector2::from_ints(1, 1), color_u32_from_rgba(200, 100, 100, 255), 0.0, 0);
        overlay_draw_list.AddText2(None, 0.0, pos, color_u32_from_rgba(255, 255, 255, 255), buf.clone(), 0.0, None);
    }
// #endif // #ifdef IMGUI_HAS_DOCK

    End();
}

// [DEBUG] Display contents of Columns
pub unsafe fn DebugNodeColumns(columns: &ImGuiOldColumns) {
    if !TreeNode(columns.ID.to_string(), format!("Columns Id: 0x{}, Count: {}, Flags: {}", columns.ID, columns.Count, columns.Flags)) { return; }
    BulletText(format!("Width: {} (MinX: {}, MaxX: {})", columns.OffMaxX - columns.OffMinX, columns.OffMinX, columns.OffMaxX));
    // for (let column_n: c_int = 0; column_n < columns.Columns.Size; column_n++)
    for column_n in 0..columns.Columns.len() {
        BulletText(format!("Column {}: OffsetNorm {} (= {} px)", column_n, columns.Columns[column_n].OffsetNorm, GetColumnOffsetFromNorm(columns, columns.Columns[column_n].OffsetNorm)));
    }
    TreePop();
}

pub unsafe fn DebugNodeDockNodeFlags(p_flags: &mut ImGuiDockNodeFlags, label: String, enabled: bool) {
    // using namespace ImGui;
    PushID(label);
    PushStyleVar(ImGuiStyleVar_FramePadding, Vector2::from_floats(0.0, 0.0));
    Text(format!("{}:", label));
    if !enabled {
        BeginDisabled(false);
    }
    CheckboxFlags(String::from("NoSplit"), p_flags, ImGuiDockNodeFlags_NoSplit);
    CheckboxFlags(String::from("NoResize"), p_flags, ImGuiDockNodeFlags_NoResize);
    CheckboxFlags(String::from("NoResizeX"), p_flags, ImGuiDockNodeFlags_NoResizeX);
    CheckboxFlags(String::from("NoResizeY"), p_flags, ImGuiDockNodeFlags_NoResizeY);
    CheckboxFlags(String::from("NoTabBar"), p_flags, ImGuiDockNodeFlags_NoTabBar);
    CheckboxFlags(String::from("HiddenTabBar"), p_flags, ImGuiDockNodeFlags_HiddenTabBar);
    CheckboxFlags(String::from("NoWindowMenuButton"), p_flags, ImGuiDockNodeFlags_NoWindowMenuButton);
    CheckboxFlags(String::from("NoCloseButton"), p_flags, ImGuiDockNodeFlags_NoCloseButton);
    CheckboxFlags(String::from("NoDocking"), p_flags, ImGuiDockNodeFlags_NoDocking);
    CheckboxFlags(String::from("NoDockingSplitMe"), p_flags, ImGuiDockNodeFlags_NoDockingSplitMe);
    CheckboxFlags(String::from("NoDockingSplitOther"), p_flags, ImGuiDockNodeFlags_NoDockingSplitOther);
    CheckboxFlags(String::from("NoDockingOverMe"), p_flags, ImGuiDockNodeFlags_NoDockingOverMe);
    CheckboxFlags(String::from("NoDockingOverOther"), p_flags, ImGuiDockNodeFlags_NoDockingOverOther);
    CheckboxFlags(String::from("NoDockingOverEmpty"), p_flags, ImGuiDockNodeFlags_NoDockingOverEmpty);
    if !enabled {
        EndDisabled();
    }
    PopStyleVar();
    pop_win_id_from_stack(g);
}

// [DEBUG] Display contents of ImDockNode
pub unsafe fn DebugNodeDockNode(node:&mut ImGuiDockNode, label: String)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let is_alive: bool = (g.FrameCount - node.LastFrameAlive < 2);    // Submitted with ImGuiDockNodeFlags_KeepAliveOnly
    let is_active: bool = (g.FrameCount - node.LastFrameActive < 2);  // Submitted
    if !is_alive { PushStyleColor(ImGuiCol_Text, ImGuiCol_TextDisabled); }
    let mut open = false;
    let tree_node_flags = if node.IsFocused { ImGuiTreeNodeFlags_Selected }else { ImGuiTreeNodeFlags_None };
    if node.Windows.len() > 0 {
        open = TreeNodeEx2(node.ID.to_string(), tree_node_flags, format!("{} {}{}: {} windows (vis: '{}')", label, node.ID, if node.IsVisible {""}else { " (hidden)" }, node.Windows.len(), if node.VisibleWindow { node.Visiblewindow.Name } else { "NULL" }));
    }
    else {
        open = TreeNodeEx2(node.ID.to_string(), tree_node_flags, format!("{} {}{}: {} split (vis: '{}')", label, node.ID, if node.IsVisible? {""}else { " (hidden)" }, if node.SplitAxis == IM_GUI_AXIS_X { "horizontal" } else {
            if node.SplitAxis == IM_GUI_AXIS_Y {
                "vertical"
            } else { "n/a" }
        }, if node.VisibleWindow { node.Visiblewindow.Name } else { "NULL" }));
    }
    if !is_alive { PopStyleColor(0); }
    if is_active && IsItemHovered(0) {
        let mut window = &mut node.HostWindow.unwrap_or(node.VisibleWindow.unwrap());
        GetForegroundDrawList(Some(&mut window.Viewport)).AddRect(node.Pos, node.Pos + node.Size, color_u32_from_rgba(255, 255, 0, 255), 0.0);
    }
    if (open)
    {
        // IM_ASSERT(node->ChildNodes[0] == NULL || node->ChildNodes[0].ParentNode == node);
        // IM_ASSERT(node->ChildNodes[1] == NULL || node->ChildNodes[1].ParentNode == node);
        BulletText(format!("Pos ({},{}), Size ({}, {}) Ref ({}, {})",
            node.Pos.x, node.Pos.y, node.Size.x, node.Size.y, node.SizeRef.x, node.SizeRef.y));
        let mut window = match node.HostWindow.clone() {
          Some(mut x) => Some(&mut x),
            None => None
        };
        DebugNodeWindow(window, String::from("HostWindow"));
        window = match node.VisibleWindow.clone() {
            Some(mut x) => Some(&mut x),
            None => None
        };
        DebugNodeWindow(window, String::from("VisibleWindow"));
        BulletText(format!("SelectedTabID: 0x{}, LastFocusedNodeID: 0x{}", node.SelectedTabId, node.LastFocusedNodeId));
        BulletText(format!("Misc:{}{}{}{}{}{}{}",
            if node.IsDockSpace() { " IsDockSpace" } else { "" },
            if node.IsCentralNode() { " IsCentralNode" }else { "" },
           if  is_alive { " IsAlive" } else { "" }, if is_active { " IsActive" } else { "" },if  node.IsFocused { " IsFocused" } else { "" },
            if node.WantLockSizeOnce { " WantLockSizeOnce" } else { "" },
            if node.HasCentralNodeChild { " HasCentralNodeChild" } else { "" }));
        if TreeNode(String::from("flags"), format!("Flags Merged: {}, Local: {}, InWindows: {}, Shared: {}", node.MergedFlags, node.LocalFlags, node.LocalFlagsInWindows, node.SharedFlags))
        {
            if BeginTable("flags", 4, 0, None, 0.0)
            {
                TableNextColumn(); DebugNodeDockNodeFlags(&mut node.MergedFlags, String::from("MergedFlags"), false);
                TableNextColumn(); DebugNodeDockNodeFlags(&mut node.LocalFlags, String::from("LocalFlags"), true);
                TableNextColumn(); DebugNodeDockNodeFlags(&mut node.LocalFlagsInWindows, String::from("LocalFlagsInWindows"), false);
                TableNextColumn(); DebugNodeDockNodeFlags(&mut node.SharedFlags, String::from("SharedFlags"), true);
                EndTable();
            }
            TreePop();
        }
        if node.ParentNode {
            DebugNodeDockNode(&mut node.ParentNode, String::from("ParentNode"));
        }
        if node.ChildNodes[0] {
            DebugNodeDockNode(&mut node.ChildNodes[0], String::from("Child[0]"));
        }
        if (node.ChildNodes[1]) {
            DebugNodeDockNode(&mut node.ChildNodes[1], String::from("Child[1]"));
        }
        if (node.TabBar) {
            DebugNodeTabBar(&mut node.TabBar, String::from("TabBar"));
        }
        DebugNodeWindowsList(&mut node.Windows, String::from("Windows"));

        TreePop();
    }
}

// [DEBUG] Display contents of ImDrawList
// Note that both 'window' and 'viewport' may be NULL here. Viewport is generally null of destroyed popups which previously owned a viewport.
pub unsafe fn DebugNodeDrawList(window: Option<&mut ImguiWindow>,
                                viewport: Option<&mut ImguiViewport>,
                                draw_list: &ImDrawList,
                                label: String)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let cfg = &g.DebugMetricsConfig;
    let mut cmd_count = draw_list.CmdBuffer.len();
    if cmd_count > 0 && draw_list.CmdBuffer.last().unwrap().ElemCount == 0 && draw_list.CmdBuffer.last().unwrap().UserCallback == None {
        cmd_count -= 1;
    }
    let mut node_open: bool =  TreeNode(draw_list.id.to_string(), format!("{}: '{}' {} vtx, {} indices, {} cmds", label, if draw_list._OwnerName { draw_list._OwnerName} else {""}, draw_list.VtxBuffer.len(), draw_list.IdxBuffer.len(), cmd_count));
    if draw_list == GetWindowDrawList()
    {
        same_line(g, 0.0, 0.0);
        TextColored(&mut ImVec4::from_floats(1.0, 0.4, 0.4, 1.0), String::from("CURRENTLY APPENDING")); // Can't display stats for active draw list! (we don't have the data double-buffered)
        if node_open {
            TreePop(); }
        return;
    }

    let mut  fg_draw_list =  if viewport.is_some() { Some(GetForegroundDrawList(viewport))} else {None}; // Render additional visuals into the top-most draw list
    if window.is_some() && IsItemHovered(0) && fg_draw_list.is_some() {
        fg_draw_list.unwrap().AddRect(window.position, window.position + window.Size, color_u32_from_rgba(255, 255, 0, 255), 0.0);
    }
    if !node_open { return ; }

    if window.is_some() && !window.unwrap().WasActive {
        TextDisabled(String::from("Warning: owning Window is inactive. This DrawList is not being rendered!"));
    }


    // for (*const ImDrawCmd pcmd = draw_list.CmdBuffer; pcmd < draw_list.CmdBuffer + cmd_count; pcmd++)
    for pcmd in draw_list.CmdBuffer.iter()
    {
        if pcmd.UserCallback.is_some()
        {
            // BulletText(format!("Callback {:?}, user_data {:?}", pcmd.UserCallback, pcmd.UserCallbackData).as_str());
            continue;
        }

        // buf: [c_char;300];
        let mut buf = String::with_capacity(300);
        // ImFormatString(buf, buf.len(), format!("DrawCmd:%5d tris, Tex 0x{}, ClipRect (%4.0,%4.0)-(%4.0,%4.0)",
        //     pcmd.ElemCount / 3, pcmd.TextureId,
        //     pcmd.ClipRect.x, pcmd.ClipRect.y, pcmd.ClipRect.z, pcmd.ClipRect.w));
        let mut pcmd_node_open: bool =  TreeNode((pcmd - draw_list.CmdBuffer.begin()), format!("{}", buf));
        if IsItemHovered(0) && (cfg.ShowDrawCmdMesh || cfg.ShowDrawCmdBoundingBoxes) && fg_draw_list.is_some() {
            // DebugNodeDrawCmdShowMeshAndBoundingBox(fg_draw_list, draw_list, pcmd, cfg.ShowDrawCmdMesh, cfg.ShowDrawCmdBoundingBoxes);
        }
        if !pcmd_node_open {
            continue;
        }

        // Calculate approximate coverage area (touched pixel count)
        // This will be in pixels squared as long there's no post-scaling happening to the renderer output.
        let idx_buffer = if draw_list.IdxBuffer.len() > 0 { draw_list.IdxBuffer.Data} else { None};
        let vtx_buffer = draw_list.VtxBuffer.Data + pcmd.VtxOffset;
        let mut total_area: c_float =  0.0;
        // for (let mut idx_n: c_uint =  pcmd.IdxOffset; idx_n < pcmd.IdxOffset + pcmd.ElemCount; )
        for idx_n in pcmd.IdxOffset .. pcmd.IdxOffset + pcmd.ElemCount
        {
            // triangle: ImVec2[3];
            let mut triangle: [Vector2;3] = [Vector2::default();3];
            // for (let n: c_int = 0; n < 3; n++, idx_n++)
            for n in 0 .. 3
            {
                triangle[n] = vtx_buffer[if idx_buffer { idx_buffer[idx_n] } else { idx_n }].pos;
            }
            total_area += ImTriangleArea(triangle[0], triangle[1], triangle[2]);
        }

        // Display vertex information summary. Hover to get all triangles drawn in wire-frame
        // ImFormatString(buf, buf.len(), "Mesh: ElemCount: {}, VtxOffset: +{}, IdxOffset: +{}, Area: ~%0.f px", pcmd.ElemCount, pcmd.VtxOffset, pcmd.IdxOffset, total_area);
        Selectable(buf, false, 0, None);
        if IsItemHovered(0) && fg_draw_list.is_some() {
            // DebugNodeDrawCmdShowMeshAndBoundingBox(fg_draw_list, draw_list, pcmd, true, false);
        }

        // Display individual triangles/vertices. Hover on to get the corresponding triangle highlighted.
        let mut clipper = ImGuiListClipper::default();
        clipper.Begin(pcmd.ElemCount / 3, 0.0); // Manually coarse clip our print out of individual vertices to save CPU, only items that may be visible.
        while clipper.Step() {
            // for (let prim: c_int = clipper.DisplayStart, idx_i = pcmd.IdxOffset + clipper.DisplayStart * 3; prim < clipper.DisplayEnd; prim+ +)
            let mut idx_i = pcmd.IdxOffset + clipper.DisplayStart * 3;
            for prim in clipper.DisplayStart .. clipper.DisplayEnd
            {
                // let buf_p = buf;
                // let buf_end = buf + buf.len();
                // triangle: ImVec2[3];
                let mut triangle: [Vector2;3] = [Vector2::default();3];
                // for (let n: c_int = 0; n < 3; n+ +, idx_i+ +)
                for n in 0 .. 3
                {

                    let v = vtx_buffer[if idx_buffer { idx_buffer[idx_i] } else { idx_i }];
                    triangle[n] = v.pos;
                    // buf_p += ImFormatString(buf_p, buf_end - buf_p, "{} %04d: pos (%8.2f,%8.20), uv (%.6f,%.60), col {}\n",
                    //                         (n == 0)? "Vert:": "     ", idx_i, v.pos.x, v.pos.y, v.uv.x, v.uv.y, v.col);
                    idx_i += 1;
                }

                Selectable(buf.clone(), false, 0, None);
                if fg_draw_list.is_some() && IsItemHovered(0) {
                   let backup_flags = fg_draw_list.unwrap().Flags;
                    let dl = gd_draw_list.unwrap();
                    dl.Flags &= !ImDrawListFlags_AntiAliasedLines; // Disable AA on triangle outlines is more readable for very large and thin triangles.
                    dl.AddPolyline(triangle, 3, color_u32_from_rgba(255, 255, 0, 255), ImDrawFlags_Closed, 1.0);
                    dl.Flags = backup_flags;
                    fg_draw_list.replace(dl);
                }
            }
        }
        TreePop();
    }
    TreePop();
}

// [DEBUG] Display mesh/aabb of a ImDrawCmd
pub unsafe fn DebugNodeDrawCmdShowMeshAndBoundingBox(out_draw_list: &mut ImDrawList, draw_list: &ImDrawList, draw_cmd: &ImDrawCmd, show_mesh: bool, show_aabb: bool)
{
    // IM_ASSERT(show_mesh || show_aabb);

    // Draw wire-frame version of all triangles
    let clip_rect =  draw_cmd.ClipRect;
    let mut vtxs_rect = ImRect::from_floats(f32::MAX, f32::MAX, -f32::MAX, -f32::MAX);
    let backup_flags = out_draw_list.Flags;
    out_draw_list.Flags &= !ImDrawListFlags_AntiAliasedLines; // Disable AA on triangle outlines is more readable for very large and thin triangles.
    // for (let mut idx_n: c_uint =  draw_cmd.IdxOffset, idx_end = draw_cmd.IdxOffset + draw_cmd.ElemCount; idx_n < idx_end; )
    let idx_end = draw_cmd.IdxOffset + draw_cmd.ElemCount;
    let mut idx_n = draw_cmd.IdxOffset;
    while idx_n < idx_end
    {

        let idx_buffer = if draw_list.IdxBuffer.len() > 0 { draw_list.IdxBuffer.Data} else { None}; // We don't hold on those pointers past iterations as ->AddPolyline() may invalidate them if out_draw_list==draw_list
        let vtx_buffer = draw_list.VtxBuffer.Data + draw_cmd.VtxOffset;

        // let mut triangle: [ImVec2;3] = [ImVec2::default();3];
        let mut triangle: Vec<Vector2> = vec![];
        // for (let n: c_int = 0; n < 3; n++, idx_n++)
        for n in 0 .. 3
        {
            triangle[n] = vtx_buffer[if idx_buffer { idx_buffer[idx_n] } else { idx_n }].pos;
            vtxs_rect.Add(&triangle[n]);
            idx_n += 1;
        }
        if show_mesh {
            out_draw_list.AddPolyline(&triangle, color_u32_from_rgba(255, 255, 0, 255), ImDrawFlags_Closed, 1.0);
        } // In yellow: mesh triangles
    }
    // Draw bounding boxes
    if (show_aabb)
    {
        out_draw_list.AddRect((clip_rect.Min).floor(), (clip_rect.Max).floor(), color_u32_from_rgba(255, 0, 255, 255), 0.0); // In pink: clipping rectangle submitted to GPU
        out_draw_list.AddRect((vtxs_rect.min).floor(), (vtxs_rect.max).floor(), color_u32_from_rgba(0, 255, 255, 255), 0.0); // In cyan: bounding box of triangles
    }
    out_draw_list.Flags = backup_flags;
}

// [DEBUG] Display details for a single font, called by ShowStyleEditor().
pub unsafe fn DebugNodeFont(font: &mut ImFont)
{
    let mut opened: bool =  TreeNode(font.to_string(), format!("Font: \"{}\"\n{} px, {} glyphs, {} file(s)",
        if font.ConfigData { font.ConfigData[0].Name } else { "" }, font.FontSize, font.Glyphs.Size, font.ConfigDataCount));
    same_line(g, 0.0, 0.0);
    if SmallButton(String::from("Set as default")) {
        GetIO().FontDefault = font;
    }
    if !opened { return ; }

    // Display preview text
    PushFont(Some(font.clone()));
    Text(String::from("The quick brown fox jumps over the lazy dog"));
    PopFont();

    // Display details
    SetNextItemWidth(GetFontSize() * 8);
    DragFloat(String::from("Font scale"), &mut font.Scale, 0.005, 0.3, 2.0, &mut String::from("{}"), 0);
    same_line(g, 0.0, 0.0); MetricsHelpMarker(
        "Note than the default embedded font is NOT meant to be scaled.\n\n" +
        "Font are currently rendered into bitmaps at a given size at the time of building the atlas. " +
        "You may oversample them to get some flexibility with scaling. " +
        "You can also render at multiple sizes and select which one to use at runtime.\n\n" +
        "(Glimmer of hope: the atlas system will be rewritten in the future to make scaling more flexible.)");
    Text(format!("Ascent: {}, Descent: {}, Height: {}", font.Ascent, font.Descent, font.Ascent - font.Descent));
    // c_str: [c_char;5];
    let mut c_str = String::with_capacity(5);
    // Text(format!("Fallback character: '{}' (U+{})", c_str);
    // Text(format!("Ellipsis character: '{}' (U+{})", ImTextCharToUtf8(c_str, font.EllipsisChar), font.EllipsisChar).as_str());
    let surface_sqrt = f32::from(font.MetricsTotalSurface).sqrt();
    Text(format!("Texture Area: about {} px ~{}x{} px", font.MetricsTotalSurface, surface_sqrt, surface_sqrt));
    // for (let config_i: c_int = 0; config_i < font.ConfigDataCount; config_i++)
    for config_i in 0 .. font.ConfigDataCount
    {
        if font.ConfigData.is_null() == false {
            let cfg = &font.ConfigData[config_i];
            BulletText(format!("Input {}: \'{}\', Oversample: ({},{}), PixelSnapH: {}, Offset: ({},{})",
                               config_i, cfg.Name, cfg.OversampleH, cfg.OversampleV, cfg.PixelSnapH, cfg.GlyphOffset.x, cfg.GlyphOffset.y));
        }
    }

    // Display all glyphs of the fonts in separate pages of 256 characters
    if TreeNode(String::from("Glyphs"), String::from("Glyphs ({})"))
    {
        let mut  draw_list =  GetWindowDrawList();
        let glyph_col: u32 = GetColorU32(ImGuiCol_Text, 0.0);
        let cell_size: c_float =  font.FontSize * 1;
        let cell_spacing: c_float =  GetStyle().item_spacing.y;
        // for (let mut base: c_uint =  0; base <= IM_UNICODE_CODEPOINT_MAX; base += 256)
        for mut base in (0 .. IM_UNICODE_CODEPOINT_MAX).step_by(256)
        {
            // Skip ahead if a large bunch of glyphs are not present in the font (test in chunks of 4k)
            // This is only a small optimization to reduce the number of iterations when IM_UNICODE_MAX_CODEPOINT
            // is large // (if ImWchar==ImWchar32 we will do at least about 272 queries here)
            if !(base & 4095) != 0 && font.IsGlyphRangeUnused(base, base + 4095)
            {
                base += 4096 - 256;
                continue;
            }

            let mut count: c_int = 0;
            // for (let mut n: c_uint =  0; n < 256; n++)
            for n in 0 .. 256
            {
                if font.FindGlyphNoFallback(char::from_u32(base + n).unwrap()) {
                    count += 1;
                }
            }
            if count <= 0 {
                continue;
            }
            if !TreeNode(base.to_string(), format!("U+{}..U+{} ({} {})", base, base + 255, count, count > 1 ? "glyphs" : "glyph")) {
                continue;
            }

            // Draw a 16x16 grid of glyphs
            let base_pos: Vector2 = cursor_screen_pos(g);
            // for (let mut n: c_uint =  0; n < 256; n++)
            {
                // We use ImFont::RenderChar as a shortcut because we don't have UTF-8 conversion functions
                // available here and thus cannot easily generate a zero-terminated UTF-8 encoded string.
                let cell_p1 = Vector2::from_floats(base_pos.x + (n % 16) * (cell_size + cell_spacing), base_pos.y + (n / 16) * (cell_size + cell_spacing));
                let cell_p2 = Vector2::from_floats(cell_p1.x + cell_size, cell_p1.y + cell_size);
                let glyph = font.FindGlyphNoFallback((base + n));
                draw_list.AddRect(cell_p1, cell_p2, if glyph.is_some() { color_u32_from_rgba(255, 255, 255, 100) } else { color_u32_from_rgba(255, 255, 255, 50) }, 0.0);
                if !glyph {
                    continue;
                }
                font.RenderChar(&mut draw_list, cell_size, &cell_p1, glyph_col, (base + n));
                if IsMouseHoveringRect(&cell_p1, &cell_p2, false)
                {
                    BeginTooltip();
                    // DebugNodeFontGlyph(font.clone());
                    EndTooltip();
                }
            }
            Dummy(g, &Vector2::from_floats((cell_size + cell_spacing) * 16, (cell_size + cell_spacing) * 16));
            TreePop();
        }
        TreePop();
    }
    TreePop();
}

pub unsafe fn DebugNodeFontGlyph(glyph: &ImFontGlyph)
{
    Text(format!("Codepoint: U+{}", glyph.Codepoint));
    Separator();
    Text(format!("Visible: {}", glyph.Visible));
    Text(format!("AdvanceX: {}", glyph.AdvanceX));
    Text(format!("Pos: ({},{})->({},{})", glyph.X0, glyph.Y0, glyph.X1, glyph.Y1));
    Text(format!("UV: ({},{})->({},{})", glyph.U0, glyph.V0, glyph.U1, glyph.V1));
}

// [DEBUG] Display contents of ImGuiStorage
pub unsafe fn DebugNodeStorage(storage: &ImGuiStorage, label: String)
{
    if !TreeNode(label, format!("{}: {} entries, {} bytes", label, storage.Data.Size, storage.Data.size_in_bytes())) { return ; }
    // for (let n: c_int = 0; n < storage.Data.Size; n++)
    for n in 0 .. storage.Data.len()
    {
        let p = &storage.Data[n];
        BulletText(format!("Key {} Value  i: {}", p.key, p.val_i)); // Important: we currently don't store a type, real value may not be integer.
    }
    TreePop();
}

// [DEBUG] Display contents of ImGuiTabBar
pub unsafe fn DebugNodeTabBar(tab_bar: &mut ImGuiTabBar, label: String)
{
    // Standalone tab bars (not associated to docking/windows functionality) currently hold no discernible strings.
    buf: [c_char;256];
    char* p = buf;
    let mut  buf_end: *const c_char = buf + buf.len();
    let is_active: bool = (PrevFrameVisible >= GetFrameCount() - 2);
    // p += ImFormatString(p, buf_end - p, format!("{} 0x{} ({} tabs){}", label, tab_bar.ID, tab_bar.Tabs.Size, if is_active { ""} else { " *Inactive*" }));
    // p += ImFormatString(p, buf_end - p, "  { ");
    // for (let tab_n: c_int = 0; tab_n < ImMin(tab_bar.Tabs.Size, 3); tab_n++)
    for tab_n in 0 .. tab_bar.Tabs.len().min(3)
    {
        let tab = &tab_bar.Tabs[tab_n];
        // p += ImFormatString(p, buf_end - p, "{}'{}'",
        //     tab_n > 0 ? ", " : "", (tab.Window || tab.NameOffset != -1) ? tab_bar.GetTabNametab) : "???");
    }
    // p += ImFormatString(p, buf_end - p, (tab_bar.Tabs.Size > 3) ? " ... }" : " } ");
    if !is_active { PushStyleColor(ImGuiCol_Text, ImGuiCol_TextDisabled); }
    let mut open: bool =  TreeNode(label, formt!("{}", buf));
    if !is_active { PopStyleColor(0); }
    if is_active && IsItemHovered(0)
    {
        let mut  draw_list: *mut ImDrawList =  GetForegroundDrawList(None);
        draw_list.AddRect(tab_bar.BarRect.min,
                          tab_bar.BarRect.max,
                          color_u32_from_rgba(255, 255, 0, 255),
                          0.0);
        draw_list.AddLine(Vector2::from_floats(ScrollingRectMinX, tab_bar.BarRect.min.y), Vector2::from_floats(ScrollingRectMinX, tab_bar.BarRect.max.y), color_u32_from_rgba(0, 255, 0, 255), 0.0);
        draw_list.AddLine(Vector2::from_floats(ScrollingRectMaxX, tab_bar.BarRect.min.y), Vector2::from_floats(ScrollingRectMaxX, tab_bar.BarRect.max.y), color_u32_from_rgba(0, 255, 0, 255), 0.0);
    }
    if open
    {
        // for (let tab_n: c_int = 0; tab_n < tab_bar.Tabs.Size; tab_n++)
        for tab_n in 0 .. tab_bar.Tabs.len()
        {
            let tab = &tab_bar.Tabs[tab_n];
            PushID(tab);
            if SmallButton(String::from("<")) { TabBarQueueReorder(tab_bar, tab, -1); } same_line(g, 0.0, 2.0);
            if SmallButton(String::from(">")) { TabBarQueueReorder(tab_bar, tab, 1); } same_line(g, 0.0, 0.0);
            Text(format!("{}{} Tab 0x{} '{}' Offset: {}, Width: {}/{}",
                         tab_n, if tab.ID == tab_bar.SelectedTabId { '*'} else {' '}, tab.ID, if tab.Window.is_some() || tab.NameOffset != -1 { tab_bar.GetTabNametab
                } else { "???" }, tab.Offset, tab.Width, tab.ContentWidth));
            pop_win_id_from_stack(g);
        }
        TreePop();
    }
}

pub unsafe fn DebugNodeViewport(viewport: &mut ImguiViewport)
{
    SetNextItemOpen(true, ImGuiCond_Once);
    if TreeNode(viewport.ID.to_string(), format!("Viewport #{}, ID: 0x{}, Parent: 0x{}, Window: \"{}\"", viewport.Idx, viewport.ID, viewport.ParentViewportId, if viewport.Window { viewport.window.Name } else { "N/A" }))
    {
        let flags = viewport.Flags;
        BulletText(format!("Main Pos: ({},{}), Size: ({},{})\nWorkArea Offset Left: {} Top: {}, Right: {}, Bottom: {}f\nMonitor: {}, DpiScale: {}f%%",
            viewport.Pos.x, viewport.Pos.y, viewport.Size.x, viewport.Size.y,
            viewport.WorkOffsetMin.x, viewport.WorkOffsetMin.y, viewport.WorkOffsetMax.x, viewport.WorkOffsetMax.y,
            viewport.PlatformMonitor, viewport.DpiScale * 100));
        if viewport.Idx > 0 { same_line(g, 0.0, 0.0); if SmallButton(String::from("Reset Pos")) { viewport.Pos = Vector2::from_ints(200, 200); viewport.UpdateWorkRect(); if viewport.Window{ viewport.window.position = viewport.Pos;} } }
        BulletText(format!("Flags: {} ={}{}{}{}{}{}{}{}{}{}{}{}", viewport.Flags,
            //(flags & ImguiViewportFlags_IsPlatformWindow) ? " IsPlatformWindow" : "", // Omitting because it is the standard
            if flag_set(flags, ImguiViewportFlags_IsPlatformMonitor) { " IsPlatformMonitor"} else {""},
            if flag_set(flags, ImguiViewportFlags_OwnedByApp) { " OwnedByApp" } else {""},
            if flag_set(flags, ImguiViewportFlags_NoDecoration) { " NoDecoration"} else {""},
            if flag_set(flags, ImguiViewportFlags_NoTaskBarIcon) { " NoTaskBarIcon" }else{""},
            if flag_set(flags, ImguiViewportFlags_NoFocusOnAppearing) { " NoFocusOnAppearing" }else {""},
            if flag_set(flags, ImguiViewportFlags_NoFocusOnClick) { " NoFocusOnClick"} else {""},
            if flag_set(flags, ImguiViewportFlags_NoInputs) { " NoInputs"} else {""},
            if flag_set(flags, ImguiViewportFlags_NoRendererClear) { " NoRendererClear"} else {""},
            if flag_set(flags, ImguiViewportFlags_TopMost) { " TopMost"} else {""},
            if flag_set(flags, ImguiViewportFlags_Minimized) { " Minimized"} else {""},
            if flag_set(flags, ImguiViewportFlags_NoAutoMerge) { " NoAutoMerge"} else {""},
            if flag_set(flags, ImguiViewportFlags_CanHostOtherWindows) { " CanHostOtherWindows" }else { "" }));
        // for (let layer_i: c_int = 0; layer_i < IM_ARRAYSIZE(viewport.DrawDataBuilder.Layers); layer_i++)
        for layer_i in 0 .. viewport.DrawDataBuilder.Layers.len()
        {
            // for (let draw_list_i: c_int = 0; draw_list_i < viewport.DrawDataBuilder.Layers[layer_i].Size; draw_list_i+ +)
            for draw_list_i in 0 .. viewport.DrawDataBuilder.Layers[layer_i].len()
            {
                DebugNodeDrawList(None, Some(viewport), &viewport.DrawDataBuilder.Layers[layer_i][draw_list_i], String::from("DrawList"));
            }
        }
        TreePop();
    }
}

pub unsafe fn DebugNodeWindow(window: Option<&mut ImguiWindow>, label: String)
{
    if window.is_none()
    {
        BulletText(format!("{}: NULL", label));
        return;
    }

    let g = GImGui; // ImGuiContext& g = *GImGui;
    let is_active: bool = window.WasActive;
    let tree_node_flags = if window == g.NavWindow { ImGuiTreeNodeFlags_Selected} else { ImGuiTreeNodeFlags_None};
    if !is_active { PushStyleColor(ImGuiCol_Text, ImGuiCol_TextDisabled); }
    let open: bool = TreeNodeEx2(label, tree_node_flags, format!("{} '{}'{}", label, window.Name, if is_active { ""} else { " *Inactive*" }));
    if !is_active { PopStyleColor(0); }
    if IsItemHovered(0) && is_active {
        GetForegroundDrawList(window).AddRect(window.position.clone(), window.position.clone() + window.Size.clone(), color_u32_from_rgba(255, 255, 0, 255), 0.0);
    }
    if !open { return ; }

    if window.MemoryCompacted {
        TextDisabled(String::from("Note: some memory buffers have been compacted/freed."));
    }

    let flags = window.Flags.clone();
    DebugNodeDrawList(window.clone(), window.Viewport.clone(), window.DrawList.clone(), String::from("DrawList"));
    BulletText(format!("Pos: ({},{}), Size: ({},{}), ContentSize ({},{}) Ideal ({},{})", window.position.x, window.position.y, window.Size.x, window.Size.y, window.ContentSize.x, window.ContentSize.y, window.ContentSizeIdeal.x, window.ContentSizeIdeal.y));
    BulletText(format!("Flags: 0x{} ({}{}{}{}{}{}{}{}{}..)", flags,
        if flag_set(flags, ImGuiWindowFlags_ChildWindow)  { "Child "} else {""},     if  flag_set(flags, ImGuiWindowFlags_Tooltip)    { "Tooltip "}   else {""},  if flag_set(flags, ImGuiWindowFlags_Popup) { "Popup "} else {""},
        if flag_set(flags, ImGuiWindowFlags_Modal)       { "Modal "} else {""},    if  flag_set(flags, ImGuiWindowFlags_ChildMenu)   { "ChildMenu "} else {""}, if flag_set(flags, ImGuiWindowFlags_NoSavedSettings) { "NoSavedSettings "} else {""},
        if flag_set(flags , ImGuiWindowFlags_NoMouseInputs){ "NoMouseInputs"}else{""}, if flag_set(flags, ImGuiWindowFlags_NoNavInputs) { "NoNavInputs"} else {""}, if flag_set(flags, ImGuiWindowFlags_AlwaysAutoResize) { "AlwaysAutoResize"} else { "" }));
    BulletText(format!("WindowClassId: 0x{}", window.WindowClass.ClassId));
    BulletText(format!("Scroll: ({}/{},{}/{}) Scrollbar:{}{}", window.scroll.x, window.scrollMax.x, window.scroll.y, window.scrollMax.y, if window.scrollbarX { "X"} else{""}, if window.scrollbarY { "Y" }else { "" }));
    BulletText(format!("Active: {}/{}, WriteAccessed: {}, BeginOrderWithinContext: {}", window.Active, window.WasActive, window.WriteAccessed, if window.Active || window.WasActive { window.BeginOrderWithinContext } else { -1 }));
    BulletText(format!("Appearing: {}, Hidden: {} (CanSkip {} Cannot {}), SkipItems: {}", window.Appearing, window.Hidden, window.HiddenFramesCanSkipItems, window.HiddenFramesCannotSkipItems, window.skip_items));
    // for (let layer: c_int = 0; layer < ImGuiNavLayer_COUNT; layer++)
    for layer in 0 .. ImGuiNavLayer_COUNT
    {
        let r =  window.NavRectRel[layer].clone();
        if r.Min.x >= r.Max.y && r.Min.y >= r.Max.y
        {
            BulletText(format!("NavLastIds[{}]: 0x{}", layer, window.NavLastIds[layer]));
            continue;
        }
        BulletText(format!("NavLastIds[{}]: 0x{} at +({},{})({},{})", layer, window.NavLastIds[layer], r.Min.x, r.Min.y, r.Max.x, r.Max.y));
        if IsItemHovered(0) {
            GetForegroundDrawList(window.clone()).AddRect(r.Min + window.position.clone(), r.Max + window.position.clone(), color_u32_from_rgba(255, 255, 0, 255), 0.0);
        }
    }
    BulletText(format!("NavLayersActiveMask: {}, NavLastChildNavWindow: {}", window.dc.NavLayersActiveMask, if window.NavLastChildNavWindow { window.NavLastChildNavwindow.Name } else { "NULL" }));

    BulletText(format!("Viewport: {}{}, ViewportId: 0x{}, ViewportPos: ({},{})", if window.Viewport { window.Viewport.Idx } else { -1 }, if window.ViewportOwned { " (Owned)" } else { "" }, window.ViewportId, window.ViewportPos.x, window.ViewportPos.y));
    BulletText(format!("ViewportMonitor: {}", if window.Viewport { window.Viewport.PlatformMonitor } else { -1 }));
    BulletText(format!("DockId: {}, DockOrder: {}, Act: {}, Vis: {}", window.DockId, window.DockOrder, window.DockIsActive, window.DockTabIsVisible));
    if window.unwrap().DockNode.is_some() || window.unwrap().DockNodeAsHost.is_some() {
        DebugNodeDockNode(&mut if window.unwrap().DockNodeAsHost.is_some() { window.unwrap().DockNodeAsHost.unwrap() } else { window.unwrap().DockNode.unwrap() }, String::from(if window.unwrap().DockNodeAsHost.is_some() { "DockNodeAsHost" } else { "DockNode" }));
    }

    if window.RootWindow != window { DebugNodeWindow(window.RootWindow.clone(), String::from("RootWindow")); }
    if window.RootWindowDockTree != window.RootWindow { DebugNodeWindow(window.RootWindowDockTree.clone(), String::from("RootWindowDockTree")); }
    if window.ParentWindow != null_mut() { DebugNodeWindow(window.ParentWindow.clone(), String::from("ParentWindow")); }
    if window.dc.ChildWindows.Size > 0 { DebugNodeWindowsList(&mut window.dc.ChildWindows, String::from("ChildWindows")); }
    if window.ColumnsStorage.Size > 0 && TreeNode(String::from("Columns"), format!("Columns sets ({})", window.ColumnsStorage.Size))
    {
        // for (let n: c_int = 0; n < window.ColumnsStorage.Size; n++)
        for n in 0 .. window.unwrap().ColumnsStorage.len()
        {
            DebugNodeColumns(&window.unwrap().ColumnsStorage[n]);
        }
        TreePop();
    }
    DebugNodeStorage(&window.StateStorage, String::from("Storage"));
    TreePop();
}

pub unsafe fn DebugNodeWindowSettings(settings: &mut ImGuiWindowSettings)
{
    Text(format!("{} \"{}\" Pos ({},{}) Size ({},{}) Collapsed={}",
        settings.ID, settings.GetName(), settings.Pos.x, settings.Pos.y, settings.Size.x, settings.Size.y, settings.Collapsed));
}

pub unsafe fn DebugNodeWindowsList(windows: &mut Vec<ImguiWindow>, label: String)
{
    if !TreeNode(label, format!("{} ({})", label, windows.len())) { return ; }
    // for (let i: c_int = windows.Size - 1; i >= 0; i--) // Iterate front to back
    for win in windows.iter_mut()
    {
        PushID(win);
        DebugNodeWindow(Some(win), String::from("Window"));
        pop_win_id_from_stack(g);
    }
    TreePop();
}

// FIXME-OPT: This is technically suboptimal, but it is simpler this way.
pub unsafe fn DebugNodeWindowsListByBeginStackParent(windows: &mut Vec<ImGuiWIndow>, parent_in_begin_stack: &mut ImguiWindow)
{
    // for (let i: c_int = 0; i < windows_size; i++)
    for win in windows
    {
        // let mut window: &mut ImGuiWindow =  windows[i];
        if win.ParentWindowInBeginStack != parent_in_begin_stack {
            continue;
        }
        // buf: [c_char;20];
        let mut buf = String::with_capacity(20);
        // ImFormatString(buf, buf.len(), "[%04d] Window", window.BeginOrderWithinContext);
        //BulletText("[%04d] Window '{}'", window.BeginOrderWithinContext, window.Name);
        DebugNodeWindow(window, buf);
        indent(0.0, g);
        // DebugNodeWindowsListByBeginStackParent(windows + i + 1, windows_size - i - 1, window);
        unindent(g, 0.0);
    }
}

//-----------------------------------------------------------------------------
// [SECTION] DEBUG LOG
//-----------------------------------------------------------------------------

pub unsafe fn DebugLog(fmt: String)
{
    // va_list args;
    // va_start(args, fmt);
    DebugLogV(fmt);
    // va_end(args);
}

pub unsafe fn DebugLogV(fmt: String)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let old_size: c_int = g.DebugLogBuf.size();
    g.DebugLogBuf.appendf("[%05d] ", g.FrameCount);
    g.DebugLogBuf.appendfv(fmt, args);
    if g.DebugLogFlags & IM_GUI_DEBUG_LOG_FLAGS_OUTPUT_TO_TTY {
        IMGUI_DEBUG_PRINTF("{}", g.DebugLogBuf.begin() + old_size);
    }
}

pub unsafe fn ShowDebugLogWindow(p_open: &mut bool)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if flag_clear(g.NextWindowData.Flags , ImGuiNextWindowDataFlags_HasSize) {
        SetNextWindowSize(Vector2::from_floats(0.0, GetFontSize() * 12.0), ImGuiCond_FirstUseEver);
    }
    if !Begin(g, "Dear ImGui Debug Log", Some(p_open)) || GetCurrentWindow().BeginCount > 1
    {
        End();
        return;
    }

    AlignTextToFramePadding();
    Text(String::from("Log events:"));
    same_line(g, 0.0, 0.0); CheckboxFlags(String::from("All"), &mut g.DebugLogFlags, IM_GUI_DEBUG_LOG_FLAGS_EVENT_MASK);
    same_line(g, 0.0, 0.0); CheckboxFlags(String::from("ActiveId"), &mut g.DebugLogFlags, IM_GUI_DEBUG_LOG_FLAGS_EVENT_ACTIVE_ID);
    same_line(g, 0.0, 0.0); CheckboxFlags(String::from("Focus"), &mut g.DebugLogFlags, IM_GUI_DEBUG_LOG_FLAGS_EVENT_FOCUS);
    same_line(g, 0.0, 0.0); CheckboxFlags(String::from("Popup"), &mut g.DebugLogFlags, IM_GUI_DEBUG_LOG_FLAGS_EVENT_POPUP);
    same_line(g, 0.0, 0.0); CheckboxFlags(String::from("Nav"), &mut g.DebugLogFlags, IM_GUI_DEBUG_LOG_FLAGS_EVENT_NAV);
    same_line(g, 0.0, 0.0); CheckboxFlags(String::from("Clipper"), &mut g.DebugLogFlags, IM_GUI_DEBUG_LOG_FLAGS_EVENT_CLIPPER);
    same_line(g, 0.0, 0.0); CheckboxFlags(String::from("IO"), &mut g.DebugLogFlags, IM_GUI_DEBUG_LOG_FLAGS_EVENT_IO);
    same_line(g, 0.0, 0.0); CheckboxFlags(String::from("Docking"), &mut g.DebugLogFlags, IM_GUI_DEBUG_LOG_FLAGS_EVENT_DOCKING);
    same_line(g, 0.0, 0.0); CheckboxFlags(String::from("Viewport"), &mut g.DebugLogFlags, IM_GUI_DEBUG_LOG_FLAGS_EVENT_VIEWPORT);

    if SmallButton(String::from("Clear")) {
        g.DebugLogBuf.clear();
    }
    same_line(g, 0.0, 0.0);
    if SmallButton(String::from("Copy")) {
        SetClipboardText(g.DebugLogBuf.c_str());
    }
    BeginChild(String::from("##log"), Vector2::from_floats(0.0, 0.0), true, ImGuiWindowFlags_AlwaysVerticalScrollbar | ImGuiWindowFlags_AlwaysHorizontalScrollbar);
    TextUnformatted(g.DebugLogBuf.begin()); // FIXME-OPT: Could use a line index, but TextUnformatted() has a semi-decent fast path for large text.
    if GetScrollY() >= GetScrollMaxY(){
        SetScrollHereY(1.0);}
    EndChild();

    End();
}

//-----------------------------------------------------------------------------
// [SECTION] OTHER DEBUG TOOLS (ITEM PICKER, STACK TOOL)
//-----------------------------------------------------------------------------

// [DEBUG] Item picker tool - start with DebugStartItemPicker() - useful to visually select an item and break into its call-stack.
pub unsafe fn UpdateDebugToolItemPicker()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.DebugItemPickerBreakId = 0;
    if !g.DebugItemPickerActive { return ; }

    let mut hovered_id: ImguiHandle =  g.HoveredIdPreviousFrame;
    SetMouseCursor(ImGuiMouseCursor_Hand);
    if IsKeyPressed(ImGuiKey_Escape, false) {
        g.DebugItemPickerActive = false;}
    let change_mapping: bool = g.IO.KeyMods == (ImGuiModFlags_Ctrl | ImGuiModFlags_Shift);
    if !change_mapping && IsMouseClicked(g.DebugItemPickerMouseButton, false) && hovered_id != -1
    {
        g.DebugItemPickerBreakId = hovered_id;
        g.DebugItemPickerActive = false;
    }
    // for (let mouse_button: c_int = 0; mouse_button < 3; mouse_button++)
    for mouse_button in 0 .. 3
    {
        if change_mapping && IsMouseClicked(mouse_button, false) {
            g.DebugItemPickerMouseButton = mouse_button;
        }
    }
    SetNextWindowBgAlpha(0.70);
    BeginTooltip();
    Text(format!("HoveredId: 0x{}", hovered_id));
    Text(format!("Press ESC to abort picking."));
    let mouse_button_names: [&'static str;3] = [ "Left", "Right", "Middle" ];
    if change_mapping {
        Text(String::from("Remap w/ Ctrl+Shift: click anywhere to select new mouse button."));
    }
    else {
        TextColored(GetStyleColorVec4(if hovered_id { ImGuiCol_Text } else { ImGuiCol_TextDisabled }), format!("Click {} Button to break in debugger! (remap w/ Ctrl+Shift)", mouse_button_names[g.DebugItemPickerMouseButton]));
    }
    EndTooltip();
}

// [DEBUG] Stack Tool: update queries. Called by NewFrame()
pub unsafe fn UpdateDebugToolStackQueries()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut tool =  &mut g.DebugStackTool;

    // Clear hook when stack tool is not visible
    g.DebugHookIdInfo = 0;
    if g.FrameCount != (tool.LastActiveFrame + 1) as usize { return ; }

    // Update queries. The steps are: -1: query Stack, >= 0: query each stack item
    // We can only perform 1 ID Info query every frame. This is designed so the GetID() tests are cheap and constant-time
    let mut query_id: ImguiHandle =  if g.HoveredIdPreviousFrame { g.HoveredIdPreviousFrame } else { g.ActiveId };
    if tool.QueryId != query_id
    {
        tool.QueryId = query_id;
        tool.StackLevel = -1;
        tool.Results.clear();
    }
    if query_id == 0 { return ; }

    // Advance to next stack level when we got our result, or after 2 frames (in case we never get a result)
    let mut stack_level: c_int = tool.StackLevel;
    if stack_level >= 0 && stack_level < tool.Results.Size {
        if tool.Results[stack_level].QuerySuccess || tool.Results[stack_level].QueryFrameCount > 2 {
            tool.StackLevel += 1;
        }
    }

    // Update hook
    stack_level = tool.StackLevel;
    if stack_level == -1 {
        g.DebugHookIdInfo = query_id;
    }
    if stack_level >= 0 && stack_level < tool.Results.Size
    {
        g.DebugHookIdInfo = tool.Results[stack_level].ID;
        tool.Results[stack_level].QueryFrameCount+= 1;
    }
}

pub unsafe fn StackToolFormatLevelInfo(tool: &ImGuiStackTool, n: c_int, format_for_ui: bool, buf: &String, buf_size: size_t) -> c_int
{
    let mut info =  &tool.Results[n];
    let mut window =  if info.Desc[0] == 0 && n == 0 { FindWindowByID(, info.ID) } else { None };
    // if (window) {                                                              // Source: window name (because the root ID don't call GetID() and so doesn't get hooked)
    //     return ImFormatString(buf, buf_size, format_for_ui? "\"{}\" [window]": "{}", window.Name);
    // }
    // if (info.QuerySuccess) {                                                  // Source: GetID() hooks (prioritize over ItemInfo() because we frequently use patterns like: PushID(str), Button("") where they both have same id)
    //     return ImFormatString(buf, buf_size, format!(if (format_for_ui && info.DataType == IM_GUI_DATA_TYPE_STRING) { "\"{}\"" }else { "{}" , info.Desc)}));
    // }
    // if (tool.StackLevel < tool.Results.Size) {                            // Only start using fallback below when all queries are done, so during queries we don't flickering ??? markers.
    //     return (*buf = 0);
    // }
// #ifdef IMGUI_ENABLE_TEST_ENGINE
//     if (label: *const c_char = ImGuiTestEngine_FindItemDebugLabel(GImGui, info.ID)) {   // Source: ImGuiTestEngine's ItemInfo()
//         return ImFormatString(buf, buf_size, format_for_ui? "??? \"{}\"": "{}", label);
//     }
// #endif
//     return ImFormatString(buf, buf_size, "???");
    todo!()
}

// Stack Tool: Display UI
pub unsafe fn ShowStackToolWindow(p_open: &mut bool)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if !(g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasSize) {
        SetNextWindowSize(Vector2::from_floats(0.0, GetFontSize() * 8.0), ImGuiCond_FirstUseEver);
    }
    if !Begin(g, "Dear ImGui Stack Tool", Some(p_open)) || GetCurrentWindow().BeginCount > 1
    {
        End();
        return;
    }

    // Display hovered/active status
    let mut tool =  &mut g.DebugStackTool;
    let mut hovered_id: ImguiHandle =  g.HoveredIdPreviousFrame;
    let mut active_id: ImguiHandle =  g.ActiveId;
// #ifdef IMGUI_ENABLE_TEST_ENGINE
    Text(format!("HoveredId: 0x{} (\"{}\"), ActiveId:  0x{} (\"{}\")", hovered_id, if hovered_id { ImGuiTestEngine_FindItemDebugLabel(&g, hovered_id)} else {""}, active_id, if active_id { ImGuiTestEngine_FindItemDebugLabel(&g, active_id)} else { "" }));
// #else
    Text(format!("HoveredId: 0x{}, ActiveId:  0x{}", hovered_id, active_id));
// #endif
    same_line(g, 0.0, 0.0);
    MetricsHelpMarker(String::from("Hover an item with the mouse to display elements of the ID Stack leading to the item's final ID.\nEach level of the stack correspond to a PushID() call.\nAll levels of the stack are hashed together to make the final ID of a widget (ID displayed at the bottom level of the stack).\nRead FAQ entry about the ID stack for details."));

    // CTRL+C to copy path
    let time_since_copy =  g.Time - tool.CopyToClipboardLastTime;
    Checkbox(String::from("Ctrl+C: copy path to clipboard"), &mut tool.CopyToClipboardOnCtrlC);
    same_line(g, 0.0, 0.0);
    TextColored(&if time_since_copy >= 0.0 && time_since_copy < 0.75 && ImFmod(time_since_copy, 0.25) < 0.25 * 0.5 { ImVec4::from_floats(1.0, 1.0, 0.3, 1.0) } else { ImVec4::default() }, String::from("*COPIED*"));
    if tool.CopyToClipboardOnCtrlC && IsKeyDown(ImGuiKey_ModCtrl) && IsKeyPressed(ImGuiKey_C, false)
    {
        tool.CopyToClipboardLastTime = g.Time as c_float;
        let p = g.TempBuffer.Data;
        let p_end = p + g.TempBuffer.Size;
        // for (let stack_n: c_int = 0; stack_n < tool.Results.Size && p + 3 < p_end; stack_n++)
        // for n in 0 .. tool.Results.len()
        // {
        //     *p++ = '/';
        //     level_desc: [c_char;256];
        //     StackToolFormatLevelInfo(tool, stack_n, false, level_desc, level_desc.len());
        //     for (let n: c_int = 0; level_desc[n] && p + 2 < p_end; n++)
        //     {
        //         if (level_desc[n] == '/')
        //             *p++ = '\\';
        //         *p++ = level_desc[n];
        //     }
        //     if (p+3 > = p_end) {
        //         break;
        //     }
        // }
        // *p = '\0';
        SetClipboardText(g.TempBuffer.Data);
    }

    // Display decorated stack
    tool.LastActiveFrame = g.FrameCount as c_int;
    if tool.Results.Size > 0 && BeginTable("##table", 3, ImGuiTableFlags_Borders, None, 0.0)
    {
        let id_width: c_float =  CalcTextSize(, String::from("0xDDDDDDDD"), false, 0.0).x;
        TableSetupColumn(String::from("Seed"), ImGuiTableColumnFlags_WidthFixed, id_width, 0);
        TableSetupColumn(String::from("PushID"), ImGuiTableColumnFlags_WidthStretch, 0.0, 0);
        TableSetupColumn(String::from("Result"), ImGuiTableColumnFlags_WidthFixed, id_width, 0);
        TableHeadersRow();
        // for (let n: c_int = 0; n < tool.Results.Size; n++)
        for n in 0 .. tool.Results.len()
        {
            let mut info =  &tool.Results[n];
            TableNextColumn();
            Text(format!("0x{}", if n > 0 { tool.Results[n - 1].ID } else { 0 }));
            TableNextColumn();
            StackToolFormatLevelInfo(tool, n as c_int, true, g.TempBuffer.Data, g.TempBuffer.Size);
            TextUnformatted(g.TempBuffer.Data);
            TableNextColumn();
            Text(format!("0x{}", info.ID));
            if n == tool.Results.Size - 1 {
                TableSetBgColor(ImGuiTableBgTarget_CellBg, GetColorU32(ImGuiCol_Header, 0.0), 0);
            }
        }
        EndTable();
    }
    End();
}
