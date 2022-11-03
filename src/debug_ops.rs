#![allow(non_snake_case)]


use std::env::args;
use std::ffi::CStr;
use std::ptr::null_mut;
use libc::{c_char, c_float, c_int, c_uint, c_void, open, size_t};
use crate::axis::{ImGuiAxis_X, ImGuiAxis_Y};
use crate::button_ops::SmallButton;
use crate::checkbox_ops::Checkbox;
use crate::child_ops::{BeginChild, EndChild};
use crate::clipboard_ops::SetClipboardText;
use crate::color::{IM_COL32, ImGuiCol_Border, ImGuiCol_Header, ImGuiCol_Text, ImGuiCol_TextDisabled, ImGuiCol_TitleBg, ImGuiCol_TitleBgActive, ImGuiCol_WindowBg};
use crate::combo_box::{Combo, Combo2};
use crate::condition::{ImGuiCond_FirstUseEver, ImGuiCond_Once};
use crate::context_ops::GetFrameCount;
use crate::cursor_ops::{GetCursorScreenPos, Indent, Unindent};
use crate::data_type::ImGuiDataType;
use crate::debug_log_flags::{ImGuiDebugLogFlags_EventActiveId, ImGuiDebugLogFlags_EventClipper, ImGuiDebugLogFlags_EventDocking, ImGuiDebugLogFlags_EventFocus, ImGuiDebugLogFlags_EventIO, ImGuiDebugLogFlags_EventMask_, ImGuiDebugLogFlags_EventNav, ImGuiDebugLogFlags_EventPopup, ImGuiDebugLogFlags_EventViewport, ImGuiDebugLogFlags_OutputToTTY};
use crate::dock_context_ops::DockContextClearNodes;
use crate::dock_node::ImGuiDockNode;
use crate::draw_list::ImDrawList;
use crate::draw_list_ops::{GetForegroundDrawList, GetForegroundDrawList2};
use crate::draw_vert::ImDrawVert;
use crate::font::ImFont;
use crate::font_atlas::ImFontAtlas;
use crate::font_glyph::ImFontGlyph;
use crate::font_ops::{PopFont, PushFont};
use crate::hovered_flags::ImGuiHoveredFlags_DelayShort;
use crate::id_ops::PopID;
use crate::image_ops::Image;
use crate::imgui::GImGui;
use crate::ImGuiViewport;
use crate::input_num_ops::InputText;
use crate::input_ops::{GetInputSourceName, IsKeyDown, IsKeyPressed, IsMouseClicked, IsMouseHoveringRect, SetMouseCursor};
use crate::input_text::InputTextMultiline;
use crate::input_text_flags::ImGuiInputTextFlags_ReadOnly;
use crate::io::ImGuiIO;
use crate::io_ops::GetIO;
use crate::item_ops::{IsItemHovered, SetNextItemWidth};
use crate::key::{ImGuiKey_C, ImGuiKey_Escape, ImGuiKey_ModCtrl, ImGuiKey_NamedKey_BEGIN, ImGuiKey_NamedKey_END};
use crate::layout_ops::{Dummy, SameLine};
use crate::math_ops::{ImFmod, ImMin, ImSqrt};
use crate::mod_flags::{ImGuiModFlags_Ctrl, ImGuiModFlags_Shift};
use crate::mouse_cursor::ImGuiMouseCursor_Hand;
use crate::next_window_data_flags::ImGuiNextWindowDataFlags_HasSize;
use crate::old_columns::ImGuiOldColumns;
use crate::rect::ImRect;
use crate::render_ops::FindRenderedTextEnd;
use crate::scrolling_ops::{GetScrollMaxY, GetScrollY, SetScrollHereY};
use crate::separator::Separator;
use crate::settings_ops::{ClearIniSettings, FindWindowSettings, SaveIniSettingsToDisk};
use crate::stack_level_info::ImGuiStackLevelInfo;
use crate::stack_tool::ImGuiStackTool;
use crate::storage::ImGuiStorage;
use crate::string_ops::{ImFormatString, ImTextCharFromUtf8, ImTextCharToUtf8};
use crate::style::ImGuiStyle;
use crate::style_ops::{GetColorU32, GetStyle, GetStyleColorVec4, PopStyleColor, PushStyleColor};
use crate::tab_bar::ImGuiTabBar;
use crate::tab_item::ImGuiTabItem;
use crate::table::ImGuiTable;
use crate::table_column::ImGuiTableColumn;
use crate::table_flags::{ImGuiTableFlags_Borders, ImGuiTableFlags_RowBg, ImGuiTableFlags_SizingFixedFit};
use crate::table_ops::TableGetInstanceData;
use crate::tables::{BeginTable, DebugNodeTable, EndTable, TableHeadersRow, TableNextColumn, TableSetupColumn};
use crate::text_ops::{BulletText, CalcTextSize, GetTextLineHeight, Text, TextDisabled, TextUnformatted};
use crate::tooltip_ops::{BeginTooltip, EndTooltip};
use crate::type_defs::{ImGuiID, ImGuiTableColumnIdx};
use crate::utils::{flag_clear, flag_set, GetVersion};
use crate::vec2::ImVec2;
use crate::vec4::ImVec4;
use crate::viewport_flags::ImGuiViewportFlags_Minimized;
use crate::viewport_ops::GetMainViewport;
use crate::widget_ops::{PopTextWrapPos, PushTextWrapPos};
use crate::widgets::{GetTreeNodeToLabelSpacing, Selectable, TreeNode, TreePop};
use crate::window::find::FindWindowByID;
use crate::window::ImGuiWindow;
use crate::window::ops::{Begin, End, GetCurrentWindow, SetNextWindowSize};
use crate::window::props::{GetFont, GetFontSize, GetWindowDrawList, SetNextWindowBgAlpha};
use crate::window::window_flags::{ImGuiWindowFlags, ImGuiWindowFlags_AlwaysAutoResize, ImGuiWindowFlags_AlwaysHorizontalScrollbar, ImGuiWindowFlags_AlwaysVerticalScrollbar, ImGuiWindowFlags_ChildMenu, ImGuiWindowFlags_ChildWindow, ImGuiWindowFlags_Modal, ImGuiWindowFlags_NoMouseInputs, ImGuiWindowFlags_NoNavInputs, ImGuiWindowFlags_NoSavedSettings, ImGuiWindowFlags_Popup, ImGuiWindowFlags_Tooltip};
use crate::window::window_settings::ImGuiWindowSettings;

// [DEBUG] Stack tool: hooks called by GetID() family functions
// c_void DebugHookIdInfo(ImGuiID id, data_type: ImGuiDataType, data_id: *const c_void, data_id_end: *const c_void)
pub unsafe fn DebugHookIdInfo(id: ImGuiID, data_type: ImGuiDataType, data_id: *const c_void, data_id_ned: *const c_void) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = &mut g.CurrentWindow;
    let mut tool: *mut ImGuiStackTool = &mut g.DebugStackTool;

    // Step 0: stack query
    // This assume that the ID was computed with the current ID stack, which tends to be the case for our widget.
    if tool.StackLevel == -1 {
        tool.StackLevel += 1;
        tool.Results.resize(window.IDStack.len() + 1, ImGuiStackLevelInfo::default());
        // for (let n: c_int = 0; n < window.IDStack.Size + 1; n++)
        for n in 0..window.IDStack.len() + 1 {
            tool.Results[n].ID = if n < window.IDStack.len() {
                window.IDStack[n]
            } else { id };
        }
        return;
    }

    // Step 1+: query for individual level
    // IM_ASSERT(tool.StackLevel >= 0);
    if tool.StackLevel != window.IDStack.len() as c_int {
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
        ImGuiDataType_String => {
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
            // ImFormatString(info.Desc.as_mut_ptr(), IM_ARRAYSIZE(info.Desc), "(void*)0x%p", data_id);
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

pub unsafe fn DebugRenderViewportThumbnail(draw_list: &mut ImDrawList, viewport: &mut ImGuiViewport, bb: &mut ImRect)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = &mut g.CurrentWindow;

    let mut scale = bb.GetSize() / viewport.Size;
    let off = bb.Min - viewport.Pos * scale;
    let alpha_mul: c_float =  if flag_set(viewport.Flags, ImGuiViewportFlags_Minimized) { 0.3 } else { 1.0 };
    window.DrawList.AddRectFilled(&bb.Min, &bb.Max, GetColorU32(ImGuiCol_Border, alpha_mul * 0.4), 0.0, 0);
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
        thumb_r = ImRect(ImFloor(off + thumb_r.Min * scale), ImFloor(off +  thumb_r.Max * scale));
        title_r = ImRect(ImFloor(off + title_r.Min * scale), ImFloor(off +  ImVec2::from_floats(title_r.Max.x, title_r.Min.y) * scale) + ImVec2::from_ints(0, 5)); // Exaggerate title bar height
        thumb_r.ClipWithFull(bb);
        title_r.ClipWithFull(bb);
        let window_is_focused: bool = (g.NavWindow.is_null() == fallse && thumb_window.RootWindowForTitleBarHighlight == g.NavWindow.RootWindowForTitleBarHighlight);
        window.DrawList.AddRectFilled(&thumb_r.Min, &thumb_r.Max, GetColorU32(ImGuiCol_WindowBg, alpha_mul), 0.0, 0);
        window.DrawList.AddRectFilled(&title_r.Min, &title_r.Max, GetColorU32(if window_is_focused { ImGuiCol_TitleBgActive } else { ImGuiCol_TitleBg }, alpha_mul), f, 0);
        window.DrawList.AddRect(&thumb_r.Min, &thumb_r.Max, GetColorU32(ImGuiCol_Border, alpha_mul), 0.0);
        //         window.DrawList.AddText(g.Font, g.FontSize * 1.0, title_r.Min, GetColorU32(ImGuiCol_Text, alpha_mul), thumb_window.Name, FindRenderedTextEnd(thumb_window.Name));
        // window.DrawList.AddText2(g.Font, GetColorU32(ImGuiCol_Text, alpha_mul), thumb_window.Name.as_str());
        window.DrawList.AddText2(Some(&g.Font), g.FontSize, &title_r.Min, GetColorU32(ImGuiCol_Text, alpha_mul), thumb_window.Name.as_str(), 0.0, &Default::default())
    }
    draw_list.AddRect(&bb.Min, &bb.Max, GetColorU32(ImGuiCol_Border, alpha_mul), 0.0);
}

pub unsafe fn RenderViewportsThumbnails()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = &mut g.CurrentWindow;

    // We don't display full monitor bounds (we could, but it often looks awkward), instead we display just enough to cover all of our viewports.
    let SCALE: c_float =  1.0 / 8.0;
    let mut bb_full: ImRect = ImRect::new(f32::MAX, f32::MAX, -f32::MAX, -f32::MAX);
    // for (let n: c_int = 0; n < g.Viewports.len(); n++)
    for n in 0 .. g.Viewports.len()
    {
        bb_full.Add(&g.Viewports[n].GetMainRect().GetSize());
    }
    let p: ImVec2 = window.DC.CursorPos;
    let off: ImVec2 = p - bb_full.Min * SCALE;
    // for (let n: c_int = 0; n < g.Viewports.len(); n++)
    for n in 0 .. g.Viewports.len()
    {
        let mut viewport =  &mut g.Viewports[n];
        let mut viewport_draw_bb: ImRect = ImRect::new(off + (viewport.Pos) * SCALE, off + (viewport.Pos + viewport.Size) * SCALE);
        DebugRenderViewportThumbnail(&mut window.DrawList, viewport, &mut viewport_draw_bb);
    }
    Dummy(bb_full.GetSize() * SCALE);
}

pub fn ViewportComparerByFrontMostStampCount(lhs: &ImGuiViewport, rhs: &ImGuiViewport) -> c_int
{
    rhs.LastFrontMostStampCount - lhs.LastFrontMostStampCount
}

// Helper tool to diagnose between text encoding issues and font loading issues. Pass your UTF-8 string and verify that there are correct.
pub unsafe fn DebugTextEncoding(txt: &str)
{
    Text(format!("Text: \"{}\"",txt).as_str());
    if !BeginTable("list", 4, ImGuiTableFlags_Borders | ImGuiTableFlags_RowBg | ImGuiTableFlags_SizingFixedFit, None, 0.0) { return ; }
    TableSetupColumn("Offset", 0, 0.0, 0);
    TableSetupColumn("UTF-8", 0, 0.0, 0);
    TableSetupColumn("Glyph", 0, 0.0, 0);
    TableSetupColumn("Codepoint", 0, 0.0, 0);
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
    //     Text("U+%04X", c);
    //     p += c_utf8_len;
    // }
    EndTable();
}

// Avoid naming collision with imgui_demo.cpp's HelpMarker() for unity builds.
pub unsafe fn MetricsHelpMarker(desc: &str)
{
    TextDisabled("(?)");
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
        let font = atlas.Fonts[i];
        PushID(font);
        DebugNodeFont(font);
        PopID();
    }
    if TreeNode("Atlas texture", format!("Atlas texture ({}x{} pixels)", atlas.TexWidth, atlas.TexHeight).as_str()) {
        let tint_col = ImVec4::from_floats(1.0, 1.0, 1.0, 1.0);
        let border_col = ImVec4::from_floats(1.0, 1.0, 1.0, 0.5);
        Image(atlas.TexID, &ImVec2::from_usizes(atlas.TexWidth, atlas.TexHeight), &ImVec2::from_floats(0.0, 0.0), &ImVec2::from_floats(1.0, 1.0), tint_col, border_col);
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
                return ImRect::from_floats(c.MinX, table.InnerClipRect.Min.y, c.MaxX, table.InnerClipRect.Min.y + table_instance.LastOuterHeight);
            } else if rect_type == TRT_ColumnsWorkRect {
                let c = &table.Columns[n];
                return ImRect::from_floats(c.WorkMinX, table.WorkRect.Min.y, c.WorkMaxX, table.WorkRect.Max.y);
            } else if rect_type == TRT_ColumnsClipRect {
                let c = &table.Columns[n];
                return c.ClipRect;
            } else if rect_type == TRT_ColumnsContentHeadersUsed {
                let c = &table.Columns[n];
                return ImRect::from_floats(c.WorkMinX, table.InnerClipRect.Min.y, c.ContentMaxXHeadersUsed, table.InnerClipRect.Min.y + table_instance.LastFirstRowHeight);
            } // Note: y1/y2 not always accurate
            else if rect_type == TRT_ColumnsContentHeadersIdeal {
                let c = &table.Columns[n];
                return ImRect::from_floats(c.WorkMinX, table.InnerClipRect.Min.y, c.ContentMaxXHeadersIdeal, table.InnerClipRect.Min.y + table_instance.LastFirstRowHeight);
            } else if rect_type == TRT_ColumnsContentFrozen {
                let c = &table.Columns[n];
                return ImRect::from_floats(c.WorkMinX, table.InnerClipRect.Min.y, c.ContentMaxXFrozen, table.InnerClipRect.Min.y + table_instance.LastFirstRowHeight);
            } else if rect_type == TRT_ColumnsContentUnfrozen {
                let c = &table.Columns[n];
                return ImRect::from_floats(c.WorkMinX, table.InnerClipRect.Min.y + table_instance.LastFirstRowHeight, c.ContentMaxXUnfrozen, table.InnerClipRect.Max.y);
            }
            // IM_ASSERT(0);
            return ImRect::default();
        }

        pub unsafe fn GetWindowRect(window: &mut ImGuiWindow, rect_type: i32) -> ImRect
        {
            if rect_type == WRT_OuterRect { return window.Rect(); }
            else if (rect_type == WRT_OuterRectClipped)     { return window.OuterRectClipped; }
            else if (rect_type == WRT_InnerRect)            { return window.InnerRect; }
            else if (rect_type == WRT_InnerClipRect)        { return window.InnerClipRect; }
            else if (rect_type == WRT_WorkRect)             { return window.WorkRect; }
            else if (rect_type == WRT_Content)       { let mut min =  window.InnerRect.Min - window.Scroll + window.WindowPadding; return ImRect::from_vec2(min, min + window.ContentSize); }
            else if (rect_type == WRT_ContentIdeal)         { let mut min =  window.InnerRect.Min - window.Scroll + window.WindowPadding; return ImRect::from_vec2(min, min + window.ContentSizeIdeal); }
            else if (rect_type == WRT_ContentRegionRect)    { return window.ContentRegionRect; }
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

    if !Begin("Dear ImGui Metrics/Debugger", Some(p_open)) || GetCurrentWindow().BeginCount > 1
    {
        End();
        return;
    }

    // Basic info
    Text(format!("Dear ImGui {}", GetVersion()).as_str());
    Text(format!("Application average {} ms/frame ({} FPS)", 1000 / io.Framerate, io.Framerate).as_str());
    Text(format!("{} vertices, {} indices ({} triangles)", io.MetricsRenderVertices, io.MetricsRenderIndices, io.MetricsRenderIndices / 3).as_str());
    Text(format!("{} visible windows, {} active allocations", io.MetricsRenderWindows, io.MetricsActiveAllocations).as_str());
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
    if TreeNode("Tools", "")
    {
        let mut show_encoding_viewer: bool =  TreeNode("UTF-8 Encoding viewer", "");
        SameLine(0.0, 0.0);
        MetricsHelpMarker("You can also call DebugTextEncoding() from your code with a given string to test that your UTF-8 encoding settings are correct.");
        if show_encoding_viewer
        {
            let mut buf = String::with_capacity(100);
            SetNextItemWidth(-FLT_MIN);
            InputText("##Text", &mut buf, buf.len(), 0, None, None);
            if buf[0] != 0 {
                DebugTextEncoding(buf.as_str());
            }
            TreePop();
        }

        // The Item Picker tool is super useful to visually select an item and break into the call-stack of where it was submitted.
        if Checkbox("Show Item Picker", &mut g.DebugItemPickerActive) && g.DebugItemPickerActive {
            DebugStartItemPicker();
        }
        SameLine(0.0, 0.0);
        MetricsHelpMarker("Will call the IM_DEBUG_BREAK() macro to break in debugger.\nWarning: If you don't have a debugger attached, this will probably crash.");

        // Stack Tool is your best friend!
        Checkbox("Show Debug Log", &mut cfg.ShowDebugLog);
        SameLine(0.0, 0.0);
        MetricsHelpMarker("You can also call ShowDebugLogWindow() from your code.");

        // Stack Tool is your best friend!
        Checkbox("Show Stack Tool", &mut cfg.ShowStackTool);
        SameLine(0.0, 0.0);
        MetricsHelpMarker("You can also call ShowStackToolWindow() from your code.");

        Checkbox("Show windows begin order", &mut cfg.ShowWindowsBeginOrder);
        Checkbox("Show windows rectangles", &mut cfg.ShowWindowsRects);
        SameLine(0.0, 0.0);
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
        cfg.ShowWindowsRects |= Combo2("##show_windows_rect_type", &mut cfg.ShowWindowsRectsType, &data, WRT_Count as usize, WRT_Count);
        if cfg.ShowWindowsRects && g.NavWindow.is_some()
        {
            BulletText(format!("'{}':", g.NavWindow.Name).as_str());
            Indent(0.0);
            // for (let rect_n: c_int = 0; rect_n < WRT_Count; rect_n++)
            for rect_n in 0 .. WRT_Count
            {
                let mut r: ImRect =  Funcs::GetWindowRect(&mut g.NavWindow.unwrap(), rect_n);
                Text(format!("({},{}) ({},{}) Size ({},{}) {}", r.Min.x, r.Min.y, r.Max.x, r.Max.y, r.GetWidth(), r.GetHeight(), wrt_rects_names[rect_n]).as_str());
            }
            Unindent(0.0);
        }

        Checkbox("Show tables rectangles", &mut cfg.ShowTablesRects);
        SameLine(0.0, 0.0);
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
        cfg.ShowTablesRects |= Combo2("##show_table_rects_type", &mut cfg.ShowTablesRectsType, &trt_data, TRT_Count as usize, TRT_Count);
        if cfg.ShowTablesRects && g.NavWindow != null_mut()
        {
            // for (let table_n: c_int = 0; table_n < g.Tables.GetMapSize(); table_n++)
            for (_, table) in g.Tables.iter_mut()
            {
                // let table = g.Tables.get_key_value(table_n);
                if table.LastFrameActive < g.FrameCount - 1 || (table.OuterWindow != g.NavWindow.unwrap() && table.InnerWindow != g.NavWindow) {
                    continue;
                }

                BulletText(format!("Table {} ({} columns, in '{}')", table.ID, table.ColumnsCount, table.Outerwindow.Name).as_str());
                if IsItemHovered(0) {
                    GetForegroundDrawList2().AddRect(table.OuterRect.Min - ImVec2::new(1, 1), table.OuterRect.Max + ImVec2::new(1, 1), IM_COL32(255, 255, 0, 255), 0.0);
                }
                Indent(0.0);
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
                            Selectable(buf.as_str(), false, 0, None);
                            if IsItemHovered(0) {
                                GetForegroundDrawList2().AddRect(r.Min - ImVec2::new(1, 1), r.Max + ImVec2::new(1, 1), IM_COL32(255, 255, 0, 255), 0.0);
                            }
                        }
                    }
                    else
                    {
                        let r: ImRect =  Funcs::GetTableRect(table, rect_n, -1);
                        // ImFormatString(buf, buf.len(), "(%6.1f,%6.10.0) (%6.1f,%6.10.0) Size (%6.1f,%6.10.0) {}", r.Min.x, r.Min.y, r.Max.x, r.Max.y, r.GetWidth(), r.GetHeight(), trt_rects_names[rect_n]);
                        Selectable(buf.as_str(), false, 0, None);
                        if (IsItemHovered(0)) {
                            GetForegroundDrawList2().AddRect(r.Min - ImVec2::new(1, 1), r.Max + ImVec2::new(1, 1), IM_COL32(255, 255, 0, 255), 0.0);
                        }
                    }
                }
                Unindent(0.0);
            }
        }

        TreePop();
    }

    pub fn WindowComparerByBeginOrder(lhs: &ImGuiWindow, rhs: &ImGuiWindow) ->c_int {
        // return ((*(*const ImGuiWindow const *)lhs).BeginOrderWithinContext - (*(*const ImGuiWindow const*)rhs).BeginOrderWithinContext);
        lhs.BeginOrderWithinContext - rhs.BeginOrderWithinContext
    }

    // Windows
    if TreeNode("Windows", format!("Windows ({})", g.Windows.len()).as_str())
    {
        //SetNextItemOpen(true, ImGuiCond_Once);
        DebugNodeWindowsList(&g.Windows, "By display order");
        DebugNodeWindowsList(&g.WindowsFocusOrder, "By focus order (root windows)");
        if TreeNode("By submission order (begin stack)", "")
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
    if TreeNode("DrawLists", format!("DrawLists ({})", drawlist_count).as_str())
    {
        Checkbox("Show ImDrawCmd mesh when hovering", &mut cfg.ShowDrawCmdMesh);
        Checkbox("Show ImDrawCmd bounding boxes when hovering", &mut cfg.ShowDrawCmdBoundingBoxes);
        // for (let viewport_i: c_int = 0; viewport_i < g.Viewports.len(); viewport_i++)
        for viewport in g.Viewports.iter_mut()
        {
            // let mut viewport: *mut ImGuiViewport =  g.Viewports[viewport_i];
            let mut viewport_has_drawlist: bool =  false;
            // for (let layer_i: c_int = 0; layer_i < IM_ARRAYSIZE(viewport.DrawDataBuilder.Layers); layer_i++)
            for layer in viewport.DrawDataBuilder.Layers.iter()
            {
                // for (let draw_list_i: c_int = 0; draw_list_i < viewport.DrawDataBuilder.Layers[layer_i].Size; draw_list_i+ +)
                for draw_list in layer.iter()
                {
                    if !viewport_has_drawlist {
                        Text(format!("Active DrawLists in Viewport #{}, ID: {}", viewport.Idx, viewport.ID).as_str());
                    }
                    viewport_has_drawlist = true;
                    DebugNodeDrawList(None, viewport, draw_list, "DrawList");
                }
            }
        }
        TreePop();
    }

    // Viewports
    if TreeNode("Viewports", format!("Viewports ({})", g.Viewports.len()).as_str())
    {
        Indent(GetTreeNodeToLabelSpacing());
        RenderViewportsThumbnails();
        Unindent(GetTreeNodeToLabelSpacing());

        let mut open: bool =  TreeNode("Monitors", format!("Monitors ({})", g.PlatformIO.Monitors.Size).as_str());
        SameLine(0.0, 0.0);
        MetricsHelpMarker("Dear ImGui uses monitor data:\n- to query DPI settings on a per monitor basis\n- to position popup/tooltips so they don't straddle monitors.");
        if open
        {
            // for (let i: c_int = 0; i < g.PlatformIO.Monitors.Size; i++)
            for mon in g.PlatformIO.Monitors.iter()
            {
                // const ImGuiPlatformMonitor& mon = g.PlatformIO.Monitors[i];
                BulletText(format!("Monitor #{}: DPI {}%\n MainMin ({}.0,{}.0), MainMax ({}.0,{}.0), MainSize ({}.0,{}.0)\n WorkMin ({}.0,{}.0), WorkMax ({}.0,{}.0), WorkSize ({}.0,{}.0)",
                                   i, mon.DpiScale * 100,
                                   mon.MainPos.x, mon.MainPos.y, mon.MainPos.x + mon.MainSize.x, mon.MainPos.y + mon.MainSize.y, mon.MainSize.x, mon.MainSize.y,
                                   mon.WorkPos.x, mon.WorkPos.y, mon.WorkPos.x + mon.WorkSize.x, mon.WorkPos.y + mon.WorkSize.y, mon.WorkSize.x, mon.WorkSize.y).as_str());
            }
            TreePop();
        }

        BulletText(format!("MouseViewport: {} (UserHovered {}, LastHovered {})", if g.MouseViewport { g.MouseViewport.ID }else{ 0}, g.IO.MouseHoveredViewport, if g.MouseLastHoveredViewport { g.MouseLastHoveredViewport.ID} else {0}).as_str());
        if TreeNode("Inferred Z order (front-to-back)", "")
        {
            // static Vec<*mut ImGuiViewportP> viewports;
            let mut viewports: Vec<ImGuiViewport> = vec![];
            viewports.reserve(g.Viewports.len());
            // memcpy(viewports.Data, g.Viewports.Data, g.Viewports.size_in_bytes());
            viewports.clone_from_slice(&g.Viewports);
            if viewports.Size > 1 {
                // ImQsort(viewports.Data, viewports.Size, sizeof(ImGuiViewport *), ViewportComparerByFrontMostStampCount);
                viewports.sort_by(ViewportComparerByFrontMostStampCount);
            }
            // for (let i: c_int = 0; i < viewports.Size; i++)
            for viewport in viewports.iter()
            {
                BulletText(format!("Viewport #{}, ID: 0x{}, FrontMostStampCount = %08d, Window: \"{}\"", viewports[i]->Idx, viewports[i].ID, viewports[i]->LastFrontMostStampCount, if viewports[i].Window { viewports[i] -> window.Name } else { "N/A" }).as_str());
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
    if TreeNode("Popups", format!("Popups ({})", g.OpenPopupStack.len()).as_str())
    {
        // for (let i: c_int = 0; i < g.OpenPopupStack.len(); i++)
        for popup_data in g.OpenPopupStack.iter_mut()
        {
            // As it's difficult to interact with tree nodes while popups are open, we display everything inline.
            // let popup_data: *const ImGuiPopupData = &g.OpenPopupStack[i];
            let window =  &mut popup_data.Window;
            BulletText(format!("PopupID: {}, Window: '{}' ({}{}), BackupNavWindow '{}', ParentWindow '{}'",
                popup_data.PopupId, window ? window.Name : "NULL", window && flag_set(window.Flags, ImGuiWindowFlags_ChildWindow) ? "Child;" : "", window && flag_set(window.Flags, ImGuiWindowFlags_ChildMenu) ? "Menu;" : "",
                popup_data.BackupNavWindow ? popup_data.BackupNavwindow.Name : "NULL", window && window.ParentWindow ? window.Parentwindow.Name : "NULL").as_str());
        }
        TreePop();
    }

    // Details for TabBars
    if TreeNode("TabBars", format!("Tab Bars ({})", g.TabBars.GetAliveCount()).as_str()) {
        // for (let n: c_int = 0; n < g.TabBars.GetMapSize(); n++)
        for tab_bar in g.TabBars.values_mut() {
            // if (tab_bar: &mut ImGuiTabBar = g.TabBars.TryGetMapData(n)) {
            PushID(tab_bar);
            DebugNodeTabBar(tab_bar, "TabBar");
            PopID();
            // }
        }
        TreePop();
    }

    // Details for Tables
    if TreeNode("Tables", format!("Tables ({})", g.Tables.GetAliveCount()).as_str())
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
    if TreeNode("Fonts", format!("Fonts ({})", atlas.Fonts.Size).as_str())
    {
        ShowFontAtlas(atlas);
        TreePop();
    }

    // Details for InputText
    if TreeNode("InputText", "")
    {
        DebugNodeInputTextState(&g.InputTextState);
        TreePop();
    }

    // Details for Docking
// #ifdef IMGUI_HAS_DOCK
    if TreeNode("Docking", "")
    {
        let mut root_nodes_only =  true;
        let dc = &mut g.DockContext;
        Checkbox("List root nodes", &mut root_nodes_only);
        Checkbox("Ctrl shows window dock info", &mut cfg.ShowDockingNodes);
        if SmallButton("Clear nodes") { DockContextClearNodes(g, 0, true); }
        SameLine(0.0, 0.0);
        if SmallButton("Rebuild all") { dc.WantFullRebuild = true; }
        // for (let n: c_int = 0; n < dc.Nodes.Data.Size; n++)
        for node in dc.Nodes.iter_mut()
        {
            // if node: *mut ImGuiDockNode = dc.Nodes.Data[n].val_p {
                if !root_nodes_only || node.IsRootNode() {
                    DebugNodeDockNode(node, "Node");
                }
            // }
        }
        TreePop();
    }
// #endif // #ifdef IMGUI_HAS_DOCK

    // Settings
    if TreeNode("Settings", "")
    {
        if SmallButton("Clear") {
            ClearIniSettings();
        }
        SameLine(0.0, 0.0);
        if SmallButton("Save to memory") {
            SaveIniSettingsToMemory();
        }
        SameLine(0.0, 0.0);
        if SmallButton("Save to disk") {
            SaveIniSettingsToDisk(g.IO.IniFilename);
        }
        SameLine(0.0, 0.0);
        if g.IO.IniFilename {
            Text(format!("\"{}\"", g.IO.IniFilename).as_str());
        }
        else {
            TextUnformatted("<NULL>");
        }
        Text(format!("SettingsDirtyTimer {}", g.SettingsDirtyTimer).as_str());
        if TreeNode("SettingsHandlers", format!("Settings handlers: ({})", g.SettingsHandlers.Size).as_str())
        {
            // for (let n: c_int = 0; n < g.SettingsHandlers.Size; n++)
            for handler in g.SettingsHandlers.iter()
            {
                BulletText(format!("{}", handler.TypeName).as_str());
            }
            TreePop();
        }
        if TreeNode("SettingsWindows", format!("Settings packed data: Windows: {} bytes", g.SettingsWindows.size()).as_str())
        {
            // for (settings: *mut ImGuiWindowSettings = g.SettingsWindows.begin(); settings != null_mut(); settings = g.SettingsWindows.next_chunk(settings))
            for settings in g.SettingsWindow.iter_mut()
            {
                DebugNodeWindowSettings(settings);
            }
            TreePop();
        }

        if TreeNode("SettingsTables", format!("Settings packed data: Tables: {} bytes", g.SettingsTables.size()).as_str())
        {
            // for (ImGuiTableSettings* settings = g.SettingsTables.begin(); settings != null_mut(); settings = g.SettingsTables.next_chunk(settings))
            for settings in g.SettingsTables.iter_mut()
            {
                DebugNodeTableSettings(settings);
            }
            TreePop();
        }

// #ifdef IMGUI_HAS_DOCK
        if TreeNode("SettingsDocking", "Settings packed data: Docking")
        {
            let dc = &mut g.DockContext;
            Text("In SettingsWindows:");
            // for (settings: *mut ImGuiWindowSettings = g.SettingsWindows.begin(); settings != null_mut(); settings = g.SettingsWindows.next_chunk(settings))
            for settings in g.SettingsWIndows.iter_mut()
            {
                if settings.DockId != 0 {
                    BulletText(format!("Window '{}' -> DockId {}", settings.GetName(), settings.DockId).as_str());
                }
            }
            Text("In SettingsNodes:");
            // for (let n: c_int = 0; n < dc.NodesSettings.Size; n++)
            for settings in dc.NodeSettings.iter_mut()
            {
                // settings: *mut ImGuiDockNodeSettings = &dc.NodesSettings[n];
                let mut  selected_tab_name = String::default();
                if settings.SelectedTabId
                {
                    let window =  FindWindowByID(settings.SelectedTabId);
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
                }).as_str());
            }
            TreePop();
        }
// #endif // #ifdef IMGUI_HAS_DOCK

        if TreeNode("SettingsIniData", format!("Settings unpacked data (.ini): {} bytes", g.SettingsIniData.size()).as_str())
        {
            InputTextMultiline("##Ini", &mut g.SettingsIniData, g.SettingsIniData.Buf.len(), &mut ImVec2::from_floats(f32::MIN, GetTextLineHeight() * 20), ImGuiInputTextFlags_ReadOnly, None, None);
            TreePop();
        }
        TreePop();
    }

    // Misc Details
    if (TreeNode("Internal state", ""))
    {
        Text("WINDOWING");
        Indent(0.0);
        Text(format!("HoveredWindow: '{}'", if g.HoveredWindow.is_some() { g.Hoveredwindow.unwrap().Name }else{ "NULL"}).as_str());
        Text(format!("Hoveredwindow.Root: '{}'", if g.HoveredWindow.is_some() { g.Hoveredwindow.unwrap().RootWindowDockTree.Name }else {"NULL"}).as_str());
        Text(format!("HoveredWindowUnderMovingWindow: '{}'", if g.HoveredWindowUnderMovingWindow.is_some() { g.HoveredWindowUnderMovingwindow.unwrap().Name} else {"NULL"}).as_str());
        Text(format!("HoveredDockNode: {}", if g.DebugHoveredDockNode.is_some() { g.DebugHoveredDockNode.unwrap().ID} else {0}).as_str());
        Text(format!("MovingWindow: '{}'", if g.MovingWindow.is_some() { g.Movingwindow.unwrap().Name }else {"NULL"}).as_str());
        Text(format!("MouseViewport: {} (UserHovered {}, LastHovered {})", g.MouseViewport.ID, g.IO.MouseHoveredViewport, if g.MouseLastHoveredViewport { g.MouseLastHoveredViewport.ID }else {0}).as_str());
        Unindent(0.0);

        Text("ITEMS");
        Indent(0.0);
        Text(format!("ActiveId: {}/{} ({} sec), AllowOverlap: {}, Source: {}", g.ActiveId, g.ActiveIdPreviousFrame, g.ActiveIdTimer, g.ActiveIdAllowOverlap, GetInputSourceName(g.ActiveIdSource)).as_str());
        Text(format!("ActiveIdWindow: '{}'", if g.ActiveIdWindow { g.ActiveIdwindow.Name} else {"NULL"}).as_str());

        let mut active_id_using_key_input_count: c_int = 0;
        // for (let n: c_int = ImGuiKey_NamedKey_BEGIN; n < ImGuiKey_NamedKey_END; n++)
        for n in ImGuiKey_NamedKey_BEGIN .. ImGuiKey_NamedKey_END
        {
            active_id_using_key_input_count += if g.ActiveIdUsingKeyInputMask[n] { 1 } else { 0 };
        }
        Text(format!("ActiveIdUsing: NavDirMask: {}, KeyInputMask: {} key(s)", g.ActiveIdUsingNavDirMask, active_id_using_key_input_count).as_str());
        Text(format!("HoveredId: {} ({} sec), AllowOverlap: {}", g.HoveredIdPreviousFrame, g.HoveredIdTimer, g.HoveredIdAllowOverlap).as_str()); // Not displaying g.HoveredId as it is update mid-frame
        Text(format!("HoverDelayId: {}, Timer: {}, ClearTimer: {}", g.HoverDelayId, g.HoverDelayTimer, g.HoverDelayClearTimer).as_str());
        Text(format!("DragDrop: {}, SourceId = {}, Payload \"{}\" ({} bytes)", g.DragDropActive, g.DragDropPayload.SourceId, g.DragDropPayload.DataType, g.DragDropPayload.DataSize).as_str());
        Unindent(0.0);

        Text("NAV,FOCUS");
        Indent(0.0);
        Text(format!("NavWindow: '{}'", if g.NavWindow.is_some() { g.NavWindow.unwrap().Name} else {"NULL"}).as_str());
        Text(format!("NavId: 0x{}, NavLayer: {}", g.NavId, g.NavLayer).as_str());
        Text(format!("NavInputSource: {}", GetInputSourceName(g.NavInputSource)).as_str());
        Text(format!("NavActive: {}, NavVisible: {}", g.IO.NavActive, g.IO.NavVisible).as_str());
        Text(format!("NavActivateId/DownId/PressedId/InputId: {}/{}/{}/{}", g.NavActivateId, g.NavActivateDownId, g.NavActivatePressedId, g.NavActivateInputId).as_str());
        Text(format!("NavActivateFlags: {}", g.NavActivateFlags).as_str());
        Text(format!("NavDisableHighlight: {}, NavDisableMouseHover: {}", g.NavDisableHighlight, g.NavDisableMouseHover).as_str());
        Text(format!("NavFocusScopeId = 0x{}", g.NavFocusScopeId).as_str());
        Text(format!("NavWindowingTarget: '{}'", if g.NavWindowingTarget { g.NavWindowingTarget.Name }else {"NULL"}).as_str());
        Unindent(0.0);

        TreePop();
    }

    // Overlay: Display windows Rectangles and Begin Order
    if cfg.ShowWindowsRects || cfg.ShowWindowsBeginOrder
    {
        // for (let n: c_int = 0; n < g.Windows.len(); n++)
        for window in g.Windows.iter_mut()
        {
            // let mut window: *mut ImGuiWindow =  g.Windows[n];
            if !window.WasActive {
                continue;
            }
            let mut  draw_list =  GetForegroundDrawList(Some(window));
            if cfg.ShowWindowsRects
            {
                let r: ImRect =  Funcs::GetWindowRect(window, cfg.ShowWindowsRectsType);
                draw_list.AddRect(&r.Min, &r.Max, IM_COL32(255, 0, 128, 255), 0.0);
            }
            if cfg.ShowWindowsBeginOrder && flag_clear(window.Flags, ImGuiWindowFlags_ChildWindow)
            {
                // buf: [c_char;32];
                let mut buf = String::with_capacity(32);
                // ImFormatString(buf, buf.len(), "{}", window.BeginOrderWithinContext);
                let font_size: c_float =  GetFontSize();
                draw_list.AddRectFilled(&window.Pos, window.Pos + ImVec2::from_floats(font_size, font_size), IM_COL32(200, 100, 100, 255), 0.0, 0);
                draw_list.AddText(&window.Pos, IM_COL32(255, 255, 255, 255), buf.as_str());
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
            // if (table == null_mut() || table.LastFrameActive < g.FrameCount - 1)
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
                    let col = if table.HoveredColumnBody == column_n as ImGuiTableColumnIdx { IM_COL32(255, 255, 128, 255)} else { IM_COL32(255, 0, 128, 255)};
                    let thickness: c_float =  if table.HoveredColumnBody == column_n as ImGuiTableColumnIdx { 3.0} else {1.0};
                    draw_list.AddRect(&r.Min, &r.Max, col, 0.0);
                }
            }
            else
            {
                let r: ImRect =  Funcs::GetTableRect(table, cfg.ShowTablesRectsType, -1);
                draw_list.AddRect(&r.Min, &r.Max, IM_COL32(255, 0, 128, 255), 0.0);
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
        // p += ImFormatString(p, buf + buf.len() - p, "DockId: %X{}\n", node.ID, if node.IsCentralNode() { " *CentralNode*"}else{ ""});
        // p += ImFormatString(p, buf + buf.len() - p, "WindowClass: {}\n", node.WindowClass.ClassId);
        // p += ImFormatString(p, buf + buf.len() - p, "Size: (%.0, %.0)\n", node.Size.x, node.Size.y);
        // p += ImFormatString(p, buf + buf.len() - p, "SizeRef: (%.0, %.0)\n", node.SizeRef.x, node.SizeRef.y);
        let depth: c_int = DockNodeGetDepth(node);
        overlay_draw_list.AddRect(node.Pos + ImVec2::new(3, 3) * depth, node.Pos + node.Size - ImVec2::new(3, 3) * depth, IM_COL32(200, 100, 100, 255), 0.0);
        let pos: ImVec2 = node.Pos + ImVec2::new(3, 3) * depth;
        overlay_draw_list.AddRectFilled(pos - ImVec2::new(1, 1), pos + CalcTextSize(buf.as_str(), false, 0.0) + ImVec2::from_ints(1, 1), IM_COL32(200, 100, 100, 255), 0.0, 0);
        overlay_draw_list.AddText2(None, 0.0, &pos, IM_COL32(255, 255, 255, 255), buf.as_str(), 0.0, None);
    }
// #endif // #ifdef IMGUI_HAS_DOCK

    End();
}

// [DEBUG] Display contents of Columns
pub unsafe fn DebugNodeColumns(ImGuiOldColumns* columns)
{
    if !TreeNode((uintptr_t)columns.ID, "Columns Id: 0x{}, Count: {}, Flags: 0x%04X", columns.ID, columns->Count, columns.Flags) { return ; }
    BulletText("Width: %.1f (MinX: %.1f, MaxX: %.10.0)", columns->OffMaxX - columns->OffMinX, columns->OffMinX, columns->OffMaxX);
    for (let column_n: c_int = 0; column_n < columns->Columns.Size; column_n++)
        BulletText("Column %02d: OffsetNorm {} (= %.1f px)", column_n, columns->Columns[column_n].OffsetNorm, GetColumnOffsetFromNorm(columns, columns->Columns[column_n].OffsetNorm));
    TreePop();
}

pub unsafe fn DebugNodeDockNodeFlags(ImGuiDockNodeFlags* p_flags, label: *const c_char, enabled: bool)
{
    using namespace ImGui;
    PushID(label);
    PushStyleVar(ImGuiStyleVar_FramePadding, ImVec2::from_floats(0.0, 0.0));
    Text("{}:", label);
    if (!enabled)
        BeginDisabled();
    CheckboxFlags("NoSplit", p_flags, ImGuiDockNodeFlags_NoSplit);
    CheckboxFlags("NoResize", p_flags, ImGuiDockNodeFlags_NoResize);
    CheckboxFlags("NoResizeX", p_flags, ImGuiDockNodeFlags_NoResizeX);
    CheckboxFlags("NoResizeY",p_flags, ImGuiDockNodeFlags_NoResizeY);
    CheckboxFlags("NoTabBar", p_flags, ImGuiDockNodeFlags_NoTabBar);
    CheckboxFlags("HiddenTabBar", p_flags, ImGuiDockNodeFlags_HiddenTabBar);
    CheckboxFlags("NoWindowMenuButton", p_flags, ImGuiDockNodeFlags_NoWindowMenuButton);
    CheckboxFlags("NoCloseButton", p_flags, ImGuiDockNodeFlags_NoCloseButton);
    CheckboxFlags("NoDocking", p_flags, ImGuiDockNodeFlags_NoDocking);
    CheckboxFlags("NoDockingSplitMe", p_flags, ImGuiDockNodeFlags_NoDockingSplitMe);
    CheckboxFlags("NoDockingSplitOther", p_flags, ImGuiDockNodeFlags_NoDockingSplitOther);
    CheckboxFlags("NoDockingOverMe", p_flags, ImGuiDockNodeFlags_NoDockingOverMe);
    CheckboxFlags("NoDockingOverOther", p_flags, ImGuiDockNodeFlags_NoDockingOverOther);
    CheckboxFlags("NoDockingOverEmpty", p_flags, ImGuiDockNodeFlags_NoDockingOverEmpty);
    if (!enabled)
        EndDisabled();
    PopStyleVar();
    PopID();
}

// [DEBUG] Display contents of ImDockNode
pub unsafe fn DebugNodeDockNode(node:&mut ImGuiDockNode, label: &str)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let is_alive: bool = (g.FrameCount - node.LastFrameAlive < 2);    // Submitted with ImGuiDockNodeFlags_KeepAliveOnly
    let is_active: bool = (g.FrameCount - node.LastFrameActive < 2);  // Submitted
    if (!is_alive) { PushStyleColor(ImGuiCol_Text, GetStyleColorVec4(ImGuiCol_TextDisabled)); }
    open: bool;
    tree_node_flags: ImGuiTreeNodeFlags = if node.IsFocused { ImGuiTreeNodeFlags_Selected }else { ImGuiTreeNodeFlags_None };
    if (node.Windows.len() > 0)
        open = TreeNodeEx(node.ID, tree_node_flags, "{} 0x%04X{}: {} windows (vis: '{}')", label, node.ID, node.IsVisible ? "" : " (hidden)", node.Windows.len(), if node.VisibleWindow { node.Visiblewindow.Name } else { "NULL" });
    else
        open = TreeNodeEx(node.ID, tree_node_flags, "{} 0x%04X{}: {} split (vis: '{}')", label, node.ID, node.IsVisible ? "" : " (hidden)", if (node.SplitAxis == ImGuiAxis_X) { "horizontal" } else {
            if (node.SplitAxis == ImGuiAxis_Y) {
                "vertical"
            }else { "n/a" }
        }, if node.VisibleWindow { node.Visiblewindow.Name }else { "NULL" });
    if (!is_alive) { PopStyleColor(); }
    if (is_active && IsItemHovered())
        if (let mut window: *mut ImGuiWindow = if node.HostWindow { node.HostWindow } else { node.VisibleWindow })
            GetForegroundDrawList(window).AddRect(node.Pos, node.Pos + node.Size, IM_COL32(255, 255, 0, 255));
    if (open)
    {
        // IM_ASSERT(node->ChildNodes[0] == NULL || node->ChildNodes[0].ParentNode == node);
        // IM_ASSERT(node->ChildNodes[1] == NULL || node->ChildNodes[1].ParentNode == node);
        BulletText("Pos (%.0,%.0), Size (%.0, %.0) Ref (%.0, %.0)",
            node.Pos.x, node.Pos.y, node.Size.x, node.Size.y, node.SizeRef.x, node.SizeRef.y);
        DebugNodeWindow(node.HostWindow, "HostWindow");
        DebugNodeWindow(node.VisibleWindow, "VisibleWindow");
        BulletText("SelectedTabID: 0x{}, LastFocusedNodeID: 0x{}", node.SelectedTabId, node.LastFocusedNodeId);
        BulletText("Misc:{}{}{}{}{}{}{}",
            if node.IsDockSpace() { " IsDockSpace" } else { "" },
            if node.IsCentralNode() { " IsCentralNode" }else { "" },
           if  is_alive { " IsAlive" } else { "" }, if is_active { " IsActive" } else { "" },if  node.IsFocused { " IsFocused" } else { "" },
            if node.WantLockSizeOnce { " WantLockSizeOnce" } else { "" },
            if node.HasCentralNodeChild { " HasCentralNodeChild" } else { "" });
        if (TreeNode("flags", "Flags Merged: 0x%04X, Local: 0x%04X, InWindows: 0x%04X, Shared: 0x%04X", node.MergedFlags, node.LocalFlags, node.LocalFlagsInWindows, node.SharedFlags))
        {
            if (BeginTable("flags", 4))
            {
                TableNextColumn(); DebugNodeDockNodeFlags(&node.MergedFlags, "MergedFlags", false);
                TableNextColumn(); DebugNodeDockNodeFlags(&node.LocalFlags, "LocalFlags", true);
                TableNextColumn(); DebugNodeDockNodeFlags(&node.LocalFlagsInWindows, "LocalFlagsInWindows", false);
                TableNextColumn(); DebugNodeDockNodeFlags(&node.SharedFlags, "SharedFlags", true);
                EndTable();
            }
            TreePop();
        }
        if (node.ParentNode)
            DebugNodeDockNode(node.ParentNode, "ParentNode");
        if (node.ChildNodes[0])
            DebugNodeDockNode(node.ChildNodes[0], "Child[0]");
        if (node.ChildNodes[1])
            DebugNodeDockNode(node.ChildNodes[1], "Child[1]");
        if (node.TabBar)
            DebugNodeTabBar(node.TabBar, "TabBar");
        DebugNodeWindowsList(&node.Windows, "Windows");

        TreePop();
    }
}

// [DEBUG] Display contents of ImDrawList
// Note that both 'window' and 'viewport' may be NULL here. Viewport is generally null of destroyed popups which previously owned a viewport.
pub unsafe fn DebugNodeDrawList(window: Option<&mut ImGuiWindow>,
                                viewport: &mut ImGuiViewport,
                                draw_list: &ImDrawList,
                                label: &str)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    ImGuiMetricsConfig* cfg = &g.DebugMetricsConfig;
    let cmd_count: c_int = draw_list.CmdBuffer.len();
    if (cmd_count > 0 && draw_list.CmdBuffer.last().unwrap().ElemCount == 0 && draw_list.CmdBuffer.last().unwrap().UserCallback == null_mut())
        cmd_count-= 1;
    let mut node_open: bool =  TreeNode(draw_list, "{}: '{}' {} vtx, {} indices, {} cmds", label, if draw_list._OwnerName { draw_list._OwnerName} else {""}, draw_list.VtxBuffer.len(), draw_list.IdxBuffer.len(), cmd_count);
    if (draw_list == GetWindowDrawList())
    {
        SameLine();
        TextColored(ImVec4(1.0, 0.4f, 0.4f, 1.0), "CURRENTLY APPENDING"); // Can't display stats for active draw list! (we don't have the data double-buffered)
        if node_open {
            TreePop(); }
        return;
    }

    let mut  fg_draw_list: *mut ImDrawList =  if viewport { GetForegroundDrawList(viewport)} else {null_mut()}; // Render additional visuals into the top-most draw list
    if (is_not_null(window) && IsItemHovered() && fg_draw_list)
        fg_draw_list.AddRect(window.Pos, window.Pos + window.Size, IM_COL32(255, 255, 0, 255));
    if !node_open { return ; }

    if (is_not_null(window) && !window.WasActive)
        TextDisabled("Warning: owning Window is inactive. This DrawList is not being rendered!");

    for (*const ImDrawCmd pcmd = draw_list.CmdBuffer; pcmd < draw_list.CmdBuffer + cmd_count; pcmd++)
    {
        if (pcmd->UserCallback)
        {
            BulletText("Callback %p, user_data %p", pcmd->UserCallback, pcmd->UserCallbackData);
            continue;
        }

        buf: [c_char;300];
        ImFormatString(buf, buf.len(), "DrawCmd:%5d tris, Tex 0x%p, ClipRect (%4.0,%4.0)-(%4.0,%4.0)",
            pcmd->ElemCount / 3, pcmd.TextureId,
            pcmd->ClipRect.x, pcmd->ClipRect.y, pcmd->ClipRect.z, pcmd->ClipRect.w);
        let mut pcmd_node_open: bool =  TreeNode((pcmd - draw_list.CmdBuffer.begin()), "{}", buf);
        if (IsItemHovered() && (cfg.ShowDrawCmdMesh || cfg.ShowDrawCmdBoundingBoxes) && fg_draw_list)
            DebugNodeDrawCmdShowMeshAndBoundingBox(fg_draw_list, draw_list, pcmd, cfg.ShowDrawCmdMesh, cfg.ShowDrawCmdBoundingBoxes);
        if (!pcmd_node_open)
            continue;

        // Calculate approximate coverage area (touched pixel count)
        // This will be in pixels squared as long there's no post-scaling happening to the renderer output.
        let idx_buffer: *const ImDrawIdx = if draw_list.IdxBuffer.len() > 0 { draw_list.IdxBuffer.Data} else { null_mut()};
        let vtx_buffer: *const ImDrawVert = draw_list.VtxBuffer.Data + pcmd->VtxOffset;
        let total_area: c_float =  0.0;
        for (let mut idx_n: c_uint =  pcmd->IdxOffset; idx_n < pcmd->IdxOffset + pcmd->ElemCount; )
        {
            triangle: ImVec2[3];
            for (let n: c_int = 0; n < 3; n++, idx_n++)
                triangle[n] = vtx_buffer[if idx_buffer { idx_buffer[idx_n]} else {idx_n}].pos;
            total_area += ImTriangleArea(triangle[0], triangle[1], triangle[2]);
        }

        // Display vertex information summary. Hover to get all triangles drawn in wire-frame
        ImFormatString(buf, buf.len(), "Mesh: ElemCount: {}, VtxOffset: +{}, IdxOffset: +{}, Area: ~%0.f px", pcmd->ElemCount, pcmd->VtxOffset, pcmd->IdxOffset, total_area);
        Selectable(buf);
        if (IsItemHovered() && fg_draw_list)
            DebugNodeDrawCmdShowMeshAndBoundingBox(fg_draw_list, draw_list, pcmd, true, false);

        // Display individual triangles/vertices. Hover on to get the corresponding triangle highlighted.
        ImGuiListClipper clipper;
        clipper.Begin(pcmd->ElemCount / 3); // Manually coarse clip our print out of individual vertices to save CPU, only items that may be visible.
        while (clipper.Step())
            for (let prim: c_int = clipper.DisplayStart, idx_i = pcmd->IdxOffset + clipper.DisplayStart * 3; prim < clipper.DisplayEnd; prim++)
            {
                char* buf_p = buf, * buf_end = buf + buf.len();
                triangle: ImVec2[3];
                for (let n: c_int = 0; n < 3; n++, idx_i++)
                {
                    const ImDrawVert& v = vtx_buffer[if idx_buffer { idx_buffer[idx_i]} else {idx_i}];
                    triangle[n] = v.pos;
                    buf_p += ImFormatString(buf_p, buf_end - buf_p, "{} %04d: pos (%8.2f,%8.20), uv (%.6f,%.60), col {}\n",
                        (n == 0) ? "Vert:" : "     ", idx_i, v.pos.x, v.pos.y, v.uv.x, v.uv.y, v.col);
                }

                Selectable(buf, false);
                if (fg_draw_list && IsItemHovered())
                {
                    ImDrawListFlags backup_flags = fg_draw_list.Flags;
                    fg_draw_list.Flags &= !ImDrawListFlags_AntiAliasedLines; // Disable AA on triangle outlines is more readable for very large and thin triangles.
                    fg_draw_list.AddPolyline(triangle, 3, IM_COL32(255, 255, 0, 255), ImDrawFlags_Closed, 1.0);
                    fg_draw_list.Flags = backup_flags;
                }
            }
        TreePop();
    }
    TreePop();
}

// [DEBUG] Display mesh/aabb of a ImDrawCmd
pub unsafe fn DebugNodeDrawCmdShowMeshAndBoundingBox(ImDrawList* out_draw_list, *const ImDrawList draw_list, *const ImDrawCmd draw_cmd, show_mesh: bool, show_aabb: bool)
{
    // IM_ASSERT(show_mesh || show_aabb);

    // Draw wire-frame version of all triangles
    let clip_rect: ImRect =  draw_cmd->ClipRect;
    let mut vtxs_rect: ImRect = ImRect::new(f32::MAX, f32::MAX, -f32::MAX, -f32::MAX);
    ImDrawListFlags backup_flags = out_draw_list.Flags;
    out_draw_list.Flags &= !ImDrawListFlags_AntiAliasedLines; // Disable AA on triangle outlines is more readable for very large and thin triangles.
    for (let mut idx_n: c_uint =  draw_cmd->IdxOffset, idx_end = draw_cmd->IdxOffset + draw_cmd->ElemCount; idx_n < idx_end; )
    {
        ImDrawIdx* idx_buffer = if draw_list.IdxBuffer.len() > 0 { draw_list.IdxBuffer.Data} else { null_mut()}; // We don't hold on those pointers past iterations as ->AddPolyline() may invalidate them if out_draw_list==draw_list
        vtx_buffer: *mut ImDrawVert = draw_list.VtxBuffer.Data + draw_cmd->VtxOffset;

        triangle: ImVec2[3];
        for (let n: c_int = 0; n < 3; n++, idx_n++)
            vtxs_rect.Add((triangle[n] = vtx_buffer[if idx_buffer { idx_buffer[idx_n]} else {idx_n}].pos));
        if (show_mesh)
            out_draw_list.AddPolyline(triangle, 3, IM_COL32(255, 255, 0, 255), ImDrawFlags_Closed, 1.0); // In yellow: mesh triangles
    }
    // Draw bounding boxes
    if (show_aabb)
    {
        out_draw_list.AddRect(ImFloor(clip_rect.Min), ImFloor(clip_rect.Max), IM_COL32(255, 0, 255, 255)); // In pink: clipping rectangle submitted to GPU
        out_draw_list.AddRect(ImFloor(vtxs_rect.Min), ImFloor(vtxs_rect.Max), IM_COL32(0, 255, 255, 255)); // In cyan: bounding box of triangles
    }
    out_draw_list.Flags = backup_flags;
}

// [DEBUG] Display details for a single font, called by ShowStyleEditor().
pub unsafe fn DebugNodeFont(font: *mut ImFont)
{
    let mut opened: bool =  TreeNode(font, "Font: \"{}\"\n%.2f px, {} glyphs, {} file(s)",
        if font.ConfigData { font.ConfigData[0].Name } else { "" }, font.FontSize, font.Glyphs.Size, font.ConfigDataCount);
    SameLine();
    if (SmallButton("Set as default"))
        GetIO().FontDefault = font;
    if !opened { return ; }

    // Display preview text
    PushFont(font);
    Text("The quick brown fox jumps over the lazy dog");
    PopFont();

    // Display details
    SetNextItemWidth(GetFontSize() * 8);
    DragFloat("Font scale", &font.Scale, 0.005f, 0.3f, 2.0, "%.1f");
    SameLine(); MetricsHelpMarker(
        "Note than the default embedded font is NOT meant to be scaled.\n\n"
        "Font are currently rendered into bitmaps at a given size at the time of building the atlas. "
        "You may oversample them to get some flexibility with scaling. "
        "You can also render at multiple sizes and select which one to use at runtime.\n\n"
        "(Glimmer of hope: the atlas system will be rewritten in the future to make scaling more flexible.)");
    Text("Ascent: %f, Descent: %f, Height: %f", font.Ascent, font.Descent, font.Ascent - font.Descent);
    c_str: [c_char;5];
    Text("Fallback character: '{}' (U+%04X)", ImTextCharToUtf8(c_str, font.FallbackChar), font.FallbackChar);
    Text("Ellipsis character: '{}' (U+%04X)", ImTextCharToUtf8(c_str, font.EllipsisChar), font.EllipsisChar);
    let surface_sqrt: c_int = ImSqrt(font.MetricsTotalSurface);
    Text("Texture Area: about {} px ~{}x{} px", font.MetricsTotalSurface, surface_sqrt, surface_sqrt);
    for (let config_i: c_int = 0; config_i < font.ConfigDataCount; config_i++)
        if (font.ConfigData)
            if (*const ImFontConfig cfg = &font.ConfigData[config_i])
                BulletText("Input {}: \'{}\', Oversample: ({},{}), PixelSnapH: {}, Offset: (%.1f,%.10.0)",
                    config_i, cfg.Name, cfg.OversampleH, cfg.OversampleV, cfg.PixelSnapH, cfg.GlyphOffset.x, cfg.GlyphOffset.y);

    // Display all glyphs of the fonts in separate pages of 256 characters
    if (TreeNode("Glyphs", "Glyphs ({})", font.Glyphs.Size))
    {
        let mut  draw_list: *mut ImDrawList =  GetWindowDrawList();
        glyph_col: u32 = GetColorU32(ImGuiCol_Text, 0.0);
        let cell_size: c_float =  font.FontSize * 1;
        let cell_spacing: c_float =  GetStyle().ItemSpacing.y;
        for (let mut base: c_uint =  0; base <= IM_UNICODE_CODEPOINT_MAX; base += 256)
        {
            // Skip ahead if a large bunch of glyphs are not present in the font (test in chunks of 4k)
            // This is only a small optimization to reduce the number of iterations when IM_UNICODE_MAX_CODEPOINT
            // is large // (if ImWchar==ImWchar32 we will do at least about 272 queries here)
            if (!(base & 4095) && font.IsGlyphRangeUnused(base, base + 4095))
            {
                base += 4096 - 256;
                continue;
            }

            let count: c_int = 0;
            for (let mut n: c_uint =  0; n < 256; n++)
                if (font.FindGlyphNoFallback((base + n)))
                    count+= 1;
            if count <= 0 {
                continue(); }
            if (!TreeNode(base, "U+%04X..U+%04X ({} {})", base, base + 255, count, count > 1 ? "glyphs" : "glyph"))
                continue;

            // Draw a 16x16 grid of glyphs
            let base_pos: ImVec2 = GetCursorScreenPos();
            for (let mut n: c_uint =  0; n < 256; n++)
            {
                // We use ImFont::RenderChar as a shortcut because we don't have UTF-8 conversion functions
                // available here and thus cannot easily generate a zero-terminated UTF-8 encoded string.
                cell_p1: ImVec2(base_pos.x + (n % 16) * (cell_size + cell_spacing), base_pos.y + (n / 16) * (cell_size + cell_spacing));
                cell_p2: ImVec2(cell_p1.x + cell_size, cell_p1.y + cell_size);
                let glyph: *const ImFontGlyph = font.FindGlyphNoFallback((base + n));
                draw_list.AddRect(cell_p1, cell_p2, if glyph { IM_COL32(255, 255, 255, 100) } else { IM_COL32(255, 255, 255, 50) });
                if (!glyph)
                    continue;
                font.RenderChar(draw_list, cell_size, cell_p1, glyph_col, (base + n));
                if (IsMouseHoveringRect(cell_p1, cell_p2))
                {
                    BeginTooltip();
                    DebugNodeFontGlyph(font, glyph);
                    EndTooltip();
                }
            }
            Dummy(ImVec2::from_floats((cell_size + cell_spacing) * 16, (cell_size + cell_spacing) * 16));
            TreePop();
        }
        TreePop();
    }
    TreePop();
}

pub unsafe fn DebugNodeFontGlyph(ImFont*, *const ImFontGlyph glyph)
{
    Text("Codepoint: U+%04X", glyph->Codepoint);
    Separator();
    Text("Visible: {}", glyph->Visible);
    Text("AdvanceX: %.1f", glyph->AdvanceX);
    Text("Pos: (%.2f,%.20)->(%.2f,%.20)", glyph->X0, glyph->Y0, glyph->X1, glyph->Y1);
    Text("UV: ({},{})->({},{})", glyph->U0, glyph->V0, glyph->U1, glyph->V1);
}

// [DEBUG] Display contents of ImGuiStorage
pub unsafe fn DebugNodeStorage(ImGuiStorage* storage, label: *const c_char)
{
    if !TreeNode(label, "{}: {} entries, {} bytes", label, storage.Data.Size, storage.Data.size_in_bytes()) { return ; }
    for (let n: c_int = 0; n < storage.Data.Size; n++)
    {
        const ImGuiStorage::ImGuiStoragePair& p = storage.Data[n];
        BulletText("Key 0x{} Value { i: {} }", p.key, p.val_i); // Important: we currently don't store a type, real value may not be integer.
    }
    TreePop();
}

// [DEBUG] Display contents of ImGuiTabBar
pub unsafe fn DebugNodeTabBar(tab_bar: &mut ImGuiTabBar, label: &str)
{
    // Standalone tab bars (not associated to docking/windows functionality) currently hold no discernible strings.
    buf: [c_char;256];
    char* p = buf;
    let mut  buf_end: *const c_char = buf + buf.len();
    let is_active: bool = (tab_bar->PrevFrameVisible >= GetFrameCount() - 2);
    p += ImFormatString(p, buf_end - p, "{} 0x{} ({} tabs){}", label, tab_bar.ID, tab_bar.Tabs.Size, is_active ? "" : " *Inactive*");
    p += ImFormatString(p, buf_end - p, "  { ");
    for (let tab_n: c_int = 0; tab_n < ImMin(tab_bar.Tabs.Size, 3); tab_n++)
    {
        ImGuiTabItem* tab = &tab_bar.Tabs[tab_n];
        p += ImFormatString(p, buf_end - p, "{}'{}'",
            tab_n > 0 ? ", " : "", (tab.Window || tab.NameOffset != -1) ? tab_bar.GetTabNametab) : "???");
    }
    p += ImFormatString(p, buf_end - p, (tab_bar.Tabs.Size > 3) ? " ... }" : " } ");
    if (!is_active) { PushStyleColor(ImGuiCol_Text, GetStyleColorVec4(ImGuiCol_TextDisabled)); }
    let mut open: bool =  TreeNode(label, "{}", buf);
    if (!is_active) { PopStyleColor(); }
    if (is_active && IsItemHovered())
    {
        let mut  draw_list: *mut ImDrawList =  GetForegroundDrawList();
        draw_list.AddRect(tab_bar.BarRect.Min, tab_bar.BarRect.Max, IM_COL32(255, 255, 0, 255));
        draw_list.AddLine(ImVec2::from_floats(tab_bar->ScrollingRectMinX, tab_bar.BarRect.Min.y), ImVec2::from_floats(tab_bar->ScrollingRectMinX, tab_bar.BarRect.Max.y), IM_COL32(0, 255, 0, 255));
        draw_list.AddLine(ImVec2::from_floats(tab_bar->ScrollingRectMaxX, tab_bar.BarRect.Min.y), ImVec2::from_floats(tab_bar->ScrollingRectMaxX, tab_bar.BarRect.Max.y), IM_COL32(0, 255, 0, 255));
    }
    if (open)
    {
        for (let tab_n: c_int = 0; tab_n < tab_bar.Tabs.Size; tab_n++)
        {
            let tab: *const ImGuiTabItem = &tab_bar.Tabs[tab_n];
            PushID(tab);
            if (SmallButton("<")) { TabBarQueueReorder(tab_bar, tab, -1); } SameLine(0, 2);
            if (SmallButton(">")) { TabBarQueueReorder(tab_bar, tab, 1); } SameLine();
            Text("%02d%c Tab 0x{} '{}' Offset: %.2f, Width: %.2f/%.2f",
                tab_n, (tab.ID == tab_bar.SelectedTabId) ? '*' : ' ', tab.ID, (tab.Window || tab.NameOffset != -1) ? tab_bar.GetTabNametab) : "???", tab->Offset, tab->Width, tab->ContentWidth);
            PopID();
        }
        TreePop();
    }
}

pub unsafe fn DebugNodeViewport(viewport: *mut ImGuiViewport)
{
    SetNextItemOpen(true, ImGuiCond_Once);
    if (TreeNode(viewport.ID, "Viewport #{}, ID: 0x{}, Parent: 0x{}, Window: \"{}\"", viewport.Idx, viewport.ID, viewport.ParentViewportId, if viewport.Window { viewport.window.Name } else { "N/A" }))
    {
        flags: ImGuiWindowFlags = viewport.Flags;
        BulletText("Main Pos: (%.0,%.0), Size: (%.0,%.0)\nWorkArea Offset Left: %.0 Top: %.0, Right: %.0, Bottom: %.0f\nMonitor: {}, DpiScale: %.0f%%",
            viewport.Pos.x, viewport.Pos.y, viewport.Size.x, viewport.Size.y,
            viewport.WorkOffsetMin.x, viewport.WorkOffsetMin.y, viewport.WorkOffsetMax.x, viewport.WorkOffsetMax.y,
            viewport.PlatformMonitor, viewport.DpiScale * 100);
        if (viewport.Idx > 0) { SameLine(); if (SmallButton("Reset Pos")) { viewport.Pos = ImVec2::from_floats(200, 200); viewport.UpdateWorkRect(); if viewport.Window{ viewport.window.Pos = viewport.Pos;} } }
        BulletText("Flags: 0x%04X ={}{}{}{}{}{}{}{}{}{}{}{}", viewport.Flags,
            //(flags & ImGuiViewportFlags_IsPlatformWindow) ? " IsPlatformWindow" : "", // Omitting because it is the standard
            flag_set(flags, ImGuiViewportFlags_IsPlatformMonitor) ? " IsPlatformMonitor" : "",
            flag_set(flags, ImGuiViewportFlags_OwnedByApp) ? " OwnedByApp" : "",
            flag_set(flags, ImGuiViewportFlags_NoDecoration) ? " NoDecoration" : "",
            flag_set(flags, ImGuiViewportFlags_NoTaskBarIcon) ? " NoTaskBarIcon" : "",
            flag_set(flags, ImGuiViewportFlags_NoFocusOnAppearing) ? " NoFocusOnAppearing" : "",
            flag_set(flags, ImGuiViewportFlags_NoFocusOnClick) ? " NoFocusOnClick" : "",
            flag_set(flags, ImGuiViewportFlags_NoInputs) ? " NoInputs" : "",
            flag_set(flags, ImGuiViewportFlags_NoRendererClear) ? " NoRendererClear" : "",
            flag_set(flags, ImGuiViewportFlags_TopMost) ? " TopMost" : "",
            flag_set(flags, ImGuiViewportFlags_Minimized) ? " Minimized" : "",
            flag_set(flags, ImGuiViewportFlags_NoAutoMerge) ? " NoAutoMerge" : "",
            flag_set(flags, ImGuiViewportFlags_CanHostOtherWindows) ? " CanHostOtherWindows" : "");
        for (let layer_i: c_int = 0; layer_i < IM_ARRAYSIZE(viewport.DrawDataBuilder.Layers); layer_i++)
            for (let draw_list_i: c_int = 0; draw_list_i < viewport.DrawDataBuilder.Layers[layer_i].Size; draw_list_i++)
                DebugNodeDrawList(null_mut(), viewport, viewport.DrawDataBuilder.Layers[layer_i][draw_list_i], "DrawList");
        TreePop();
    }
}

pub unsafe fn DebugNodeWindow(window: &mut ImGuiWindow, label: &str)
{
    if (window == null_mut())
    {
        BulletText("{}: NULL", label);
        return;
    }

    let g = GImGui; // ImGuiContext& g = *GImGui;
    let is_active: bool = window.WasActive;
    tree_node_flags: ImGuiTreeNodeFlags = if window == g.NavWindow { ImGuiTreeNodeFlags_Selected} else { ImGuiTreeNodeFlags_None};
    if (!is_active) { PushStyleColor(ImGuiCol_Text, GetStyleColorVec4(ImGuiCol_TextDisabled)); }
    let open: bool = TreeNodeEx(label, tree_node_flags, "{} '{}'{}", label, window.Name, is_active ? "" : " *Inactive*");
    if (!is_active) { PopStyleColor(); }
    if (IsItemHovered() && is_active)
        GetForegroundDrawList(window).AddRect(window.Pos, window.Pos + window.Size, IM_COL32(255, 255, 0, 255));
    if !open { return ; }

    if (window.MemoryCompacted)
        TextDisabled("Note: some memory buffers have been compacted/freed.");

    flags: ImGuiWindowFlags = window.Flags;
    DebugNodeDrawList(window, window.Viewport, window.DrawList, "DrawList");
    BulletText("Pos: (%.1f,%.10.0), Size: (%.1f,%.10.0), ContentSize (%.1f,%.10.0) Ideal (%.1f,%.10.0)", window.Pos.x, window.Pos.y, window.Size.x, window.Size.y, window.ContentSize.x, window.ContentSize.y, window.ContentSizeIdeal.x, window.ContentSizeIdeal.y);
    BulletText("Flags: 0x{} ({}{}{}{}{}{}{}{}{}..)", flags,
        flag_set(flags, ImGuiWindowFlags_ChildWindow)  ? "Child " : "",      flag_set(flags, ImGuiWindowFlags_Tooltip)     ? "Tooltip "   : "",  flag_set(flags, ImGuiWindowFlags_Popup) ? "Popup " : "",
        flag_set(flags, ImGuiWindowFlags_Modal)        ? "Modal " : "",      flag_set(flags, ImGuiWindowFlags_ChildMenu)   ? "ChildMenu " : "",  flag_set(flags, ImGuiWindowFlags_NoSavedSettings) ? "NoSavedSettings " : "",
        (flags & ImGuiWindowFlags_NoMouseInputs)? "NoMouseInputs":"", flag_set(flags, ImGuiWindowFlags_NoNavInputs) ? "NoNavInputs" : "", flag_set(flags, ImGuiWindowFlags_AlwaysAutoResize) ? "AlwaysAutoResize" : "");
    BulletText("WindowClassId: 0x{}", window.WindowClass.ClassId);
    BulletText("Scroll: (%.2f/%.2f,%.2f/%.20) Scrollbar:{}{}", window.Scroll.x, window.ScrollMax.x, window.Scroll.y, window.ScrollMax.y, window.ScrollbarX ? "X" : "", window.ScrollbarY ? "Y" : "");
    BulletText("Active: {}/{}, WriteAccessed: {}, BeginOrderWithinContext: {}", window.Active, window.WasActive, window.WriteAccessed, if (window.Active || window.WasActive) { window.BeginOrderWithinContext } else { -1 });
    BulletText("Appearing: {}, Hidden: {} (CanSkip {} Cannot {}), SkipItems: {}", window.Appearing, window.Hidden, window.HiddenFramesCanSkipItems, window.HiddenFramesCannotSkipItems, window.SkipItems);
    for (let layer: c_int = 0; layer < ImGuiNavLayer_COUNT; layer++)
    {
        let r: ImRect =  window.NavRectRel[layer];
        if (r.Min.x >= r.Max.y && r.Min.y >= r.Max.y)
        {
            BulletText("NavLastIds[{}]: 0x{}", layer, window.NavLastIds[layer]);
            continue;
        }
        BulletText("NavLastIds[{}]: 0x{} at +(%.1f,%.10.0)(%.1f,%.10.0)", layer, window.NavLastIds[layer], r.Min.x, r.Min.y, r.Max.x, r.Max.y);
        if (IsItemHovered())
            GetForegroundDrawList(window).AddRect(r.Min + window.Pos, r.Max + window.Pos, IM_COL32(255, 255, 0, 255));
    }
    BulletText("NavLayersActiveMask: %X, NavLastChildNavWindow: {}", window.DC.NavLayersActiveMask, if window.NavLastChildNavWindow { window.NavLastChildNavwindow.Name } else { "NULL" });

    BulletText("Viewport: {}{}, ViewportId: 0x{}, ViewportPos: (%.1f,%.10.0)", if window.Viewport { window.Viewport.Idx } else { -1 }, if window.ViewportOwned { " (Owned)" } else { "" }, window.ViewportId, window.ViewportPos.x, window.ViewportPos.y);
    BulletText("ViewportMonitor: {}", if window.Viewport { window.Viewport.PlatformMonitor } else { -1 });
    BulletText("DockId: 0x%04X, DockOrder: {}, Act: {}, Vis: {}", window.DockId, window.DockOrder, window.DockIsActive, window.DockTabIsVisible);
    if (window.DockNode || window.DockNodeAsHost)
        DebugNodeDockNode(if window.DockNodeAsHost { window.DockNodeAsHost } else { window.DockNode }, if window.DockNodeAsHost { "DockNodeAsHost" } else { "DockNode" });

    if (window.RootWindow != window)       { DebugNodeWindow(window.RootWindow, "RootWindow"); }
    if (window.RootWindowDockTree != window.RootWindow) { DebugNodeWindow(window.RootWindowDockTree, "RootWindowDockTree"); }
    if (window.ParentWindow != null_mut())       { DebugNodeWindow(window.ParentWindow, "ParentWindow"); }
    if (window.DC.ChildWindows.Size > 0)   { DebugNodeWindowsList(&window.DC.ChildWindows, "ChildWindows"); }
    if (window.ColumnsStorage.Size > 0 && TreeNode("Columns", "Columns sets ({})", window.ColumnsStorage.Size))
    {
        for (let n: c_int = 0; n < window.ColumnsStorage.Size; n++)
            DebugNodeColumns(&window.ColumnsStorage[n]);
        TreePop();
    }
    DebugNodeStorage(&window.StateStorage, "Storage");
    TreePop();
}

pub unsafe fn DebugNodeWindowSettings(settings: &mut ImGuiWindowSettings)
{
    Text(format!("{} \"{}\" Pos ({},{}) Size ({},{}) Collapsed={}",
        settings.ID, settings.GetName(), settings.Pos.x, settings.Pos.y, settings.Size.x, settings.Size.y, settings.Collapsed).as_str());
}

pub unsafe fn DebugNodeWindowsList(windows: &Vec<ImGuiWindow>, label: &str)
{
    if !TreeNode(label, format!("{} ({})", label, windows.len()).as_str()) { return ; }
    // for (let i: c_int = windows.Size - 1; i >= 0; i--) // Iterate front to back
    for win in windows
    {
        PushID(win);
        DebugNodeWindow(win, "Window");
        PopID();
    }
    TreePop();
}

// FIXME-OPT: This is technically suboptimal, but it is simpler this way.
pub unsafe fn DebugNodeWindowsListByBeginStackParent(windows: &mut Vec<ImGuiWIndow>, parent_in_begin_stack: &mut ImGuiWindow)
{
    // for (let i: c_int = 0; i < windows_size; i++)
    for win in windows
    {
        // let mut window: *mut ImGuiWindow =  windows[i];
        if win.ParentWindowInBeginStack != parent_in_begin_stack {
            continue;
        }
        // buf: [c_char;20];
        let mut buf = String::with_capacity(20);
        // ImFormatString(buf, buf.len(), "[%04d] Window", window.BeginOrderWithinContext);
        //BulletText("[%04d] Window '{}'", window.BeginOrderWithinContext, window.Name);
        DebugNodeWindow(window, buf.as_str());
        Indent(0.0);
        // DebugNodeWindowsListByBeginStackParent(windows + i + 1, windows_size - i - 1, window);
        Unindent(0.0);
    }
}

//-----------------------------------------------------------------------------
// [SECTION] DEBUG LOG
//-----------------------------------------------------------------------------

pub unsafe fn DebugLog(fmt: *const c_char, ...)
{
    va_list args;
    va_start(args, fmt);
    DebugLogV(fmt, args);
    va_end(args);
}

pub unsafe fn DebugLogV(fmt: *const c_char, va_list args)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let old_size: c_int = g.DebugLogBuf.size();
    g.DebugLogBuf.appendf("[%05d] ", g.FrameCount);
    g.DebugLogBuf.appendfv(fmt, args);
    if (g.DebugLogFlags & ImGuiDebugLogFlags_OutputToTTY)
        IMGUI_DEBUG_PRINTF("{}", g.DebugLogBuf.begin() + old_size);
}

pub unsafe fn ShowDebugLogWindow(p_open: &mut bool)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (!(g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasSize))
        SetNextWindowSize(ImVec2::from_floats(0.0, GetFontSize() * 12.0), ImGuiCond_FirstUseEver);
    if (!Begin("Dear ImGui Debug Log", p_open) || GetCurrentWindow()->BeginCount > 1)
    {
        End();
        return;
    }

    AlignTextToFramePadding();
    Text("Log events:");
    SameLine(); CheckboxFlags("All", &g.DebugLogFlags, ImGuiDebugLogFlags_EventMask_);
    SameLine(); CheckboxFlags("ActiveId", &g.DebugLogFlags, ImGuiDebugLogFlags_EventActiveId);
    SameLine(); CheckboxFlags("Focus", &g.DebugLogFlags, ImGuiDebugLogFlags_EventFocus);
    SameLine(); CheckboxFlags("Popup", &g.DebugLogFlags, ImGuiDebugLogFlags_EventPopup);
    SameLine(); CheckboxFlags("Nav", &g.DebugLogFlags, ImGuiDebugLogFlags_EventNav);
    SameLine(); CheckboxFlags("Clipper", &g.DebugLogFlags, ImGuiDebugLogFlags_EventClipper);
    SameLine(); CheckboxFlags("IO", &g.DebugLogFlags, ImGuiDebugLogFlags_EventIO);
    SameLine(); CheckboxFlags("Docking", &g.DebugLogFlags, ImGuiDebugLogFlags_EventDocking);
    SameLine(); CheckboxFlags("Viewport", &g.DebugLogFlags, ImGuiDebugLogFlags_EventViewport);

    if (SmallButton("Clear"))
        g.DebugLogBuf.clear();
    SameLine();
    if (SmallButton("Copy"))
        SetClipboardText(g.DebugLogBuf.c_str());
    BeginChild("##log", ImVec2::from_floats(0.0, 0.0), true, ImGuiWindowFlags_AlwaysVerticalScrollbar | ImGuiWindowFlags_AlwaysHorizontalScrollbar);
    TextUnformatted(g.DebugLogBuf.begin(), g.DebugLogBuf.end()); // FIXME-OPT: Could use a line index, but TextUnformatted() has a semi-decent fast path for large text.
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

    let mut hovered_id: ImGuiID =  g.HoveredIdPreviousFrame;
    SetMouseCursor(ImGuiMouseCursor_Hand);
    if IsKeyPressed(ImGuiKey_Escape){
        g.DebugItemPickerActive = false;}
    let change_mapping: bool = g.IO.KeyMods == (ImGuiModFlags_Ctrl | ImGuiModFlags_Shift);
    if (!change_mapping && IsMouseClicked(g.DebugItemPickerMouseButton) && hovered_id)
    {
        g.DebugItemPickerBreakId = hovered_id;
        g.DebugItemPickerActive = false;
    }
    for (let mouse_button: c_int = 0; mouse_button < 3; mouse_button++)
        if change_mapping && IsMouseClicked(mouse_button){
            g.DebugItemPickerMouseButton = mouse_button;}
    SetNextWindowBgAlpha(0.70);
    BeginTooltip();
    Text("HoveredId: 0x{}", hovered_id);
    Text("Press ESC to abort picking.");
    mouse_button_names: *const c_char[] = { "Left", "Right", "Middle" };
    if (change_mapping)
        Text("Remap w/ Ctrl+Shift: click anywhere to select new mouse button.");
    else
        TextColored(GetStyleColorVec4(if hovered_id {ImGuiCol_Text} else { ImGuiCol_TextDisabled }), "Click {} Button to break in debugger! (remap w/ Ctrl+Shift)", mouse_button_names[g.DebugItemPickerMouseButton]);
    EndTooltip();
}

// [DEBUG] Stack Tool: update queries. Called by NewFrame()
pub unsafe fn UpdateDebugToolStackQueries()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut tool: *mut ImGuiStackTool =  &g.DebugStackTool;

    // Clear hook when stack tool is not visible
    g.DebugHookIdInfo = 0;
    if g.FrameCount != tool.LastActiveFrame + 1 { return ; }

    // Update queries. The steps are: -1: query Stack, >= 0: query each stack item
    // We can only perform 1 ID Info query every frame. This is designed so the GetID() tests are cheap and constant-time
    let mut query_id: ImGuiID =  if g.HoveredIdPreviousFrame { g.HoveredIdPreviousFrame } else { g.ActiveId };
    if (tool.QueryId != query_id)
    {
        tool.QueryId = query_id;
        tool.StackLevel = -1;
        tool.Results.clear();
    }
    if query_id == 0 { return ; }

    // Advance to next stack level when we got our result, or after 2 frames (in case we never get a result)
    let stack_level: c_int = tool.StackLevel;
    if (stack_level >= 0 && stack_level < tool.Results.Size)
        if (tool.Results[stack_level].QuerySuccess || tool.Results[stack_level].QueryFrameCount > 2)
            tool.StackLevel+= 1;

    // Update hook
    stack_level = tool.StackLevel;
    if (stack_level == -1)
        g.DebugHookIdInfo = query_id;
    if (stack_level >= 0 && stack_level < tool.Results.Size)
    {
        g.DebugHookIdInfo = tool.Results[stack_level].ID;
        tool.Results[stack_level].QueryFrameCount+= 1;
    }
}

pub fn StackToolFormatLevelInfo(ImGuiStackTool* tool, n: c_int, format_for_ui: bool, char* buf, buf_size: size_t) -> c_int
{
    let mut info: *mut ImGuiStackLevelInfo =  &tool.Results[n];
    let mut window: *mut ImGuiWindow =  if (info.Desc[0] == 0 && n == 0) { FindWindowByID(info.ID) } else { null_mut() };
    if (window)                                                                 // Source: window name (because the root ID don't call GetID() and so doesn't get hooked)
        return ImFormatString(buf, buf_size, format_for_ui ? "\"{}\" [window]" : "{}", window.Name);
    if (info.QuerySuccess)                                                     // Source: GetID() hooks (prioritize over ItemInfo() because we frequently use patterns like: PushID(str), Button("") where they both have same id)
        return ImFormatString(buf, buf_size, (format_for_ui && info.DataType == ImGuiDataType_String) ? "\"{}\"" : "{}", info.Desc);
    if (tool.StackLevel < tool.Results.Size)                                  // Only start using fallback below when all queries are done, so during queries we don't flickering ??? markers.
        return (*buf = 0);
// #ifdef IMGUI_ENABLE_TEST_ENGINE
    if (label: *const c_char = ImGuiTestEngine_FindItemDebugLabel(GImGui, info.ID))   // Source: ImGuiTestEngine's ItemInfo()
        return ImFormatString(buf, buf_size, format_for_ui ? "??? \"{}\"" : "{}", label);
// #endif
    return ImFormatString(buf, buf_size, "???");
}

// Stack Tool: Display UI
pub unsafe fn ShowStackToolWindow(p_open: &mut bool)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (!(g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasSize))
        SetNextWindowSize(ImVec2::from_floats(0.0, GetFontSize() * 8.0), ImGuiCond_FirstUseEver);
    if (!Begin("Dear ImGui Stack Tool", p_open) || GetCurrentWindow()->BeginCount > 1)
    {
        End();
        return;
    }

    // Display hovered/active status
    let mut tool: *mut ImGuiStackTool =  &g.DebugStackTool;
    let mut hovered_id: ImGuiID =  g.HoveredIdPreviousFrame;
    let mut active_id: ImGuiID =  g.ActiveId;
// #ifdef IMGUI_ENABLE_TEST_ENGINE
    Text("HoveredId: 0x{} (\"{}\"), ActiveId:  0x{} (\"{}\")", hovered_id, hovered_id ? ImGuiTestEngine_FindItemDebugLabel(&g, hovered_id) : "", active_id, active_id ? ImGuiTestEngine_FindItemDebugLabel(&g, active_id) : "");
// #else
    Text("HoveredId: 0x{}, ActiveId:  0x{}", hovered_id, active_id);
// #endif
    SameLine();
    MetricsHelpMarker("Hover an item with the mouse to display elements of the ID Stack leading to the item's final ID.\nEach level of the stack correspond to a PushID() call.\nAll levels of the stack are hashed together to make the final ID of a widget (ID displayed at the bottom level of the stack).\nRead FAQ entry about the ID stack for details.");

    // CTRL+C to copy path
    let time_since_copy: c_float =  g.Time - tool.CopyToClipboardLastTime;
    Checkbox("Ctrl+C: copy path to clipboard", &tool.CopyToClipboardOnCtrlC);
    SameLine();
    TextColored(if (time_since_copy >= 0.0 && time_since_copy < 0.75f && ImFmod(time_since_copy, 0.250f32) < 0.25f * 0.5) { ImVec4(1.f, 1.f, 0.3f, 1.0) } else { ImVec4() }, "*COPIED*");
    if (tool.CopyToClipboardOnCtrlC && IsKeyDown(ImGuiKey_ModCtrl) && IsKeyPressed(ImGuiKey_C))
    {
        tool.CopyToClipboardLastTime = g.Time;
        char* p = g.TempBuffer.Data;
        char* p_end = p + g.TempBuffer.Size;
        for (let stack_n: c_int = 0; stack_n < tool.Results.Size && p + 3 < p_end; stack_n++)
        {
            *p++ = '/';
            level_desc: [c_char;256];
            StackToolFormatLevelInfo(tool, stack_n, false, level_desc, level_desc.len());
            for (let n: c_int = 0; level_desc[n] && p + 2 < p_end; n++)
            {
                if (level_desc[n] == '/')
                    *p++ = '\\';
                *p++ = level_desc[n];
            }
        }
        *p = '\0';
        SetClipboardText(g.TempBuffer.Data);
    }

    // Display decorated stack
    tool.LastActiveFrame = g.FrameCount;
    if (tool.Results.Size > 0 && BeginTable("##table", 3, ImGuiTableFlags_Borders))
    {
        let id_width: c_float =  CalcTextSize("0xDDDDDDDD").x;
        TableSetupColumn("Seed", ImGuiTableColumnFlags_WidthFixed, id_width);
        TableSetupColumn("PushID", ImGuiTableColumnFlags_WidthStretch);
        TableSetupColumn("Result", ImGuiTableColumnFlags_WidthFixed, id_width);
        TableHeadersRow();
        for (let n: c_int = 0; n < tool.Results.Size; n++)
        {
            let mut info: *mut ImGuiStackLevelInfo =  &tool.Results[n];
            TableNextColumn();
            Text("0x{}", if (n > 0) { tool.Results[n - 1].ID } else { 0 });
            TableNextColumn();
            StackToolFormatLevelInfo(tool, n, true, g.TempBuffer.Data, g.TempBuffer.Size);
            TextUnformatted(g.TempBuffer.Data);
            TableNextColumn();
            Text("0x{}", info.ID);
            if (n == tool.Results.Size - 1)
                TableSetBgColor(ImGuiTableBgTarget_CellBg, GetColorU32(ImGuiCol_Header, 0.0));
        }
        EndTable();
    }
    End();
}
