#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]


use std::env::args;
use std::ffi::CStr;
use std::mem;
use std::ptr::{null, null_mut};
use libc::{c_char, c_float, c_int, c_void, open, size_t, uintptr_t};
use crate::axis::{ImGuiAxis_X, ImGuiAxis_Y};
use crate::backend_flags::{ImGuiBackendFlags_HasMouseCursors, ImGuiBackendFlags_PlatformHasViewports, ImGuiBackendFlags_RendererHasViewports};
use crate::child_ops::{BeginChild, EndChild};
use crate::clipboard_ops::SetClipboardText;
use crate::color::{IM_COL32, ImGuiCol_Border, ImGuiCol_Header, ImGuiCol_Text, ImGuiCol_TextDisabled, ImGuiCol_TitleBg, ImGuiCol_TitleBgActive, ImGuiCol_WindowBg};
use crate::condition::{ImGuiCond_FirstUseEver, ImGuiCond_Once};
use crate::config_flags::{ImGuiConfigFlags_DockingEnable, ImGuiConfigFlags_ViewportsEnable};
use crate::context_ops::GetFrameCount;
use crate::cursor_ops::GetCursorScreenPos;
use crate::data_type::{ImGuiDataType, ImGuiDataType_String};
use crate::debug_log_flags::{ImGuiDebugLogFlags_EventActiveId, ImGuiDebugLogFlags_EventClipper, ImGuiDebugLogFlags_EventDocking, ImGuiDebugLogFlags_EventFocus, ImGuiDebugLogFlags_EventIO, ImGuiDebugLogFlags_EventMask_, ImGuiDebugLogFlags_EventNav, ImGuiDebugLogFlags_EventPopup, ImGuiDebugLogFlags_EventViewport, ImGuiDebugLogFlags_OutputToTTY};
use crate::dock_node::ImGuiDockNode;
use crate::dock_node_flags::{ImGuiDockNodeFlags, ImGuiDockNodeFlags_HiddenTabBar, ImGuiDockNodeFlags_NoCloseButton, ImGuiDockNodeFlags_NoDocking, ImGuiDockNodeFlags_NoDockingOverEmpty, ImGuiDockNodeFlags_NoDockingOverMe, ImGuiDockNodeFlags_NoDockingOverOther, ImGuiDockNodeFlags_NoDockingSplitMe, ImGuiDockNodeFlags_NoDockingSplitOther, ImGuiDockNodeFlags_NoResize, ImGuiDockNodeFlags_NoResizeX, ImGuiDockNodeFlags_NoResizeY, ImGuiDockNodeFlags_NoSplit, ImGuiDockNodeFlags_NoTabBar, ImGuiDockNodeFlags_NoWindowMenuButton};
use crate::dock_node_ops::DockNodeGetDepth;
use crate::draw_cmd::ImDrawCmd;
use crate::draw_flags::{ImDrawFlags, ImDrawFlags_Closed, ImDrawFlags_None};
use crate::draw_list::ImDrawList;
use crate::draw_list_flags::{ImDrawListFlags, ImDrawListFlags_AntiAliasedLines};
use crate::draw_list_ops::{GetForegroundDrawList, GetForegroundDrawList3};
use crate::draw_vert::ImDrawVert;
use crate::font::ImFont;
use crate::font_atlas::ImFontAtlas;
use crate::font_config::ImFontConfig;
use crate::font_glyph::ImFontGlyph;
use crate::hovered_flags::{ImGuiHoveredFlags_DelayShort, ImGuiHoveredFlags_None};
use crate::imgui::GImGui;
use crate::ImGuiViewport;
use crate::input_ops::{GetInputSourceName, IsKeyDown, IsKeyPressed, IsMouseClicked, IsMouseHoveringRect, SetMouseCursor};
use crate::input_text_flags::ImGuiInputTextFlags_ReadOnly;
use crate::io::ImGuiIO;
use crate::io_ops::GetIO;
use crate::item_ops::IsItemHovered;
use crate::item_picker_ops::DebugStartItemPicker;
use crate::key::{ImGuiKey_C, ImGuiKey_Escape, ImGuiKey_ModCtrl, ImGuiKey_NamedKey_BEGIN, ImGuiKey_NamedKey_END};
use crate::keyboard_ops::GetMergedModFlags;
use crate::list_clipper::ImGuiListClipper;
use crate::metrics_config::ImGuiMetricsConfig;
use crate::mod_flags::{ImGuiModFlags_Ctrl, ImGuiModFlags_Shift};
use crate::mouse_cursor::ImGuiMouseCursor_Hand;
use crate::nav_layer::ImGuiNavLayer_COUNT;
use crate::next_window_data_flags::ImGuiNextWindowDataFlags_HasSize;
use crate::old_columns::ImGuiOldColumns;
use crate::rect::ImRect;
use crate::render_ops::FindRenderedTextEnd;
use crate::stack_level_info::ImGuiStackLevelInfo;
use crate::stack_sizes::ImGuiStackSizes;
use crate::stack_tool::ImGuiStackTool;
use crate::state_ops::{Begin, End};
use crate::storage::{ImGuiStorage, ImGuiStoragePair};
use crate::string_ops::{ImFormatString, ImTextCharFromUtf8, ImTextCharToUtf8, str_to_const_c_char_ptr};
use crate::style::ImGuiStyle;
use crate::style_ops::{GetColorU32, GetStyle, GetStyleColorU32, GetStyleColorVec4, PopStyleColor, PushStyleColor};
use crate::style_var::ImGuiStyleVar_FramePadding;
use crate::style_var_ops::{PopStyleVar, PushStyleVar};
use crate::tab_bar::ImGuiTabBar;
use crate::tab_item::ImGuiTabItem;
use crate::table::ImGuiTable;
use crate::table_bg_target::ImGuiTableBgTarget_CellBg;
use crate::table_column::ImGuiTableColumn;
use crate::table_column_flags::{ImGuiTableColumnFlags_WidthFixed, ImGuiTableColumnFlags_WidthStretch};
use crate::table_flags::{ImGuiTableFlags_Borders, ImGuiTableFlags_RowBg, ImGuiTableFlags_SizingFixedFit};
use crate::table_ops::TableGetInstanceData;
use crate::text_ops::CalcTextSize;
use crate::tree_node_flags::{ImGuiTreeNodeFlags, ImGuiTreeNodeFlags_None, ImGuiTreeNodeFlags_Selected};
use crate::type_defs::{ImDrawIdx, ImGuiErrorLogCallback, ImGuiID};
use crate::utils::{flag_clear, flag_set, GetVersion, ImQsort};
use crate::vec2::ImVec2;
use crate::vec4::ImVec4;
use crate::viewport_flags::{ImGuiViewportFlags_CanHostOtherWindows, ImGuiViewportFlags_IsPlatformMonitor, ImGuiViewportFlags_Minimized, ImGuiViewportFlags_NoAutoMerge, ImGuiViewportFlags_NoDecoration, ImGuiViewportFlags_NoFocusOnAppearing, ImGuiViewportFlags_NoFocusOnClick, ImGuiViewportFlags_NoInputs, ImGuiViewportFlags_NoRendererClear, ImGuiViewportFlags_NoTaskBarIcon, ImGuiViewportFlags_OwnedByApp, ImGuiViewportFlags_TopMost};
use crate::window::ImGuiWindow;
use crate::window_flags::{ImGuiWindowFlags, ImGuiWindowFlags_AlwaysAutoResize, ImGuiWindowFlags_AlwaysHorizontalScrollbar, ImGuiWindowFlags_AlwaysVerticalScrollbar, ImGuiWindowFlags_ChildMenu, ImGuiWindowFlags_ChildWindow, ImGuiWindowFlags_Modal, ImGuiWindowFlags_NoMouseInputs, ImGuiWindowFlags_NoNavInputs, ImGuiWindowFlags_None, ImGuiWindowFlags_NoSavedSettings, ImGuiWindowFlags_Popup, ImGuiWindowFlags_Tooltip};
use crate::window_ops::{FindWindowByID, GetCurrentWindow};
use crate::window_settings::ImGuiWindowSettings;

// [DEBUG] Stack tool: hooks called by GetID() family functions
// c_void DebugHookIdInfo(ImGuiID id, ImGuiDataType data_type, *const c_void data_id, *const c_void data_id_end)
pub unsafe fn DebugHookIdInfo(id: ImGuiID, data_type: ImGuiDataType, data_id: *const c_void, data_id_ned: *const c_void) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;
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
            } else { id.clone() };
        }
        return;
    }

    // Step 1+: query for individual level
    // IM_ASSERT(tool.StackLevel >= 0);
    if tool.StackLevel != window.IDStack.len() as c_int {
        return;
    }
    let mut info: *mut ImGuiStackLevelInfo = &mut tool.Results[&tool.StackLevel];
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
            // ImFormatString(info.Desc, IM_ARRAYSIZE(info.Desc), "0x%08X [override]", id);
            todo!()
        },

        _ => {
            todo!()
        }
    };
    info.QuerySuccess = true;
    info.DataType = data_type;
}

struct Func {

}

impl Func {
    // static c_int IMGUI_CDECL WindowComparerByBeginOrder(*const c_void lhs, *const c_void rhs)
    pub fn WindowComparerByBeginOrder(lhs: *const ImGuiWindow, rhs: *const ImGuiWindow) -> c_int
    {
    // return ((*(*const ImGuiWindow const *)lhs).BeginOrderWithinContext - (*(*const ImGuiWindow const*)rhs).BeginOrderWithinContext);
        (lhs.BeginOrderWithinContext - rhs.BeginOrderWithinContext) as c_int
    }
}

// Helper function to verify ABI compatibility between caller code and compiled version of Dear ImGui.
// Verify that the type sizes are matching between the calling file's compilation unit and imgui.cpp's compilation unit
// If this triggers you have an issue:
// - Most commonly: mismatched headers and compiled code version.
// - Or: mismatched configuration #define, compilation settings, packing pragma etc.
//   The configuration settings mentioned in imconfig.h must be set for all compilation units involved with Dear ImGui,
//   which is way it is required you put them in your imconfig file (and not just before including imgui.h).
//   Otherwise it is possible that different compilation units would see different structure layout
// bool DebugCheckVersionAndDataLayout(*const char version, size_t sz_io, size_t sz_style, size_t sz_vec2, size_t sz_vec4, size_t sz_vert, size_t sz_idx)
pub unsafe fn DebugCheckVersionAndDataLayout(version: *const c_char, sz_io: size_t, sz_tyle: size_t, sz_vec2: size_t, sz_vec4: size_t, sz_vert: size_t, sz_idx: size_t) -> bool
{
    let mut error: bool =  false;
    if libc::strcmp(version, IMGUI_VERSION) != 0 {
        error = true;
        // IM_ASSERT(libc::strcmp(version, IMGUI_VERSION) == 0 && "Mismatched version string!");
    }
    if sz_io != mem::size_of::<ImGuiIO>() {
        error = true;
        // IM_ASSERT(sz_io == sizeof(ImGuiIO) && "Mismatched struct layout!");
    }
    if sz_style != mem::size_of::<ImGuiStyle>() {
        error = true;
        // IM_ASSERT(sz_style == sizeof(ImGuiStyle) && "Mismatched struct layout!");
    }
    if (sz_vec2 != mem::size_of::<ImVec2>()) {
        error = true;
        // IM_ASSERT(sz_vec2 == sizeof(ImVec2) && "Mismatched struct layout!");
    }
    if (sz_vec4 != mem::size_of::<ImVec4>()) {
        error = true;
        // IM_ASSERT(sz_vec4 == sizeof(ImVec4) && "Mismatched struct layout!");
    }
    if (sz_vert != mem::size_of::<ImDrawVert>()) {
        error = true;
        // IM_ASSERT(sz_vert == sizeof(ImDrawVert) && "Mismatched struct layout!");
    }
    if (sz_idx != mem::size_of::<ImDrawIdx>()) {
        error = true;
        // IM_ASSERT(sz_idx == sizeof(ImDrawIdx) && "Mismatched struct layout!");
    }
    return !error;
}



// static c_void ErrorCheckNewFrameSanityChecks()
pub unsafe fn ErrorCheckNewFrameSanityChecks()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    // Check user IM_ASSERT macro
    // (IF YOU GET A WARNING OR COMPILE ERROR HERE: it means your assert macro is incorrectly defined!
    //  If your macro uses multiple statements, it NEEDS to be surrounded by a 'do { ... } while (0)' block.
    //  This is a common C/C++ idiom to allow multiple statements macros to be used in control flow blocks.)
    // #define IM_ASSERT(EXPR)   if (SomeCode(EXPR)) SomeMoreCode();                    // Wrong!
    // #define IM_ASSERT(EXPR)   do { if (SomeCode(EXPR)) SomeMoreCode(); } while (0)   // Correct!
    // if (true) IM_ASSERT(1); else IM_ASSERT(0);

    // Check user data
    // (We pass an error message in the assert expression to make it visible to programmers who are not using a debugger, as most assert handlers display their argument)
    // IM_ASSERT(g.Initialized);
    // IM_ASSERT((g.IO.DeltaTime > 0f32 || g.FrameCount == 0)              && "Need a positive DeltaTime!");
    // IM_ASSERT((g.FrameCount == 0 || g.FrameCountEnded == g.FrameCount)  && "Forgot to call Render() or EndFrame() at the end of the previous frame?");
    // IM_ASSERT(g.IO.DisplaySize.x >= 0f32 && g.IO.DisplaySize.y >= 0f32  && "Invalid DisplaySize value!");
    // IM_ASSERT(g.IO.Fonts.IsBuilt()                                     && "Font Atlas not built! Make sure you called ImGui_ImplXXXX_NewFrame() function for renderer backend, which should call io.Fonts.GetTexDataAsRGBA32() / GetTexDataAsAlpha8()");
    // IM_ASSERT(g.Style.CurveTessellationTol > 0f32                       && "Invalid style setting!");
    // IM_ASSERT(g.Style.CircleTessellationMaxError > 0f32                 && "Invalid style setting!");
    // IM_ASSERT(g.Style.Alpha >= 0f32 && g.Style.Alpha <= 1f32            && "Invalid style setting!"); // Allows us to avoid a few clamps in color computations
    // IM_ASSERT(g.Style.WindowMinSize.x >= 1f32 && g.Style.WindowMinSize.y >= 1f32 && "Invalid style setting.");
    // IM_ASSERT(g.Style.WindowMenuButtonPosition == ImGuiDir_None || g.Style.WindowMenuButtonPosition == ImGuiDir_Left || g.Style.WindowMenuButtonPosition == ImGuiDir_Right);
    // IM_ASSERT(g.Style.ColorButtonPosition == ImGuiDir_Left || g.Style.ColorButtonPosition == ImGuiDir_Right);
// #ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
//     for (let n: c_int = ImGuiKey_NamedKey_BEGIN; n < ImGuiKey_COUNT; n++)
//     {}
        // IM_ASSERT(g.IO.KeyMap[n] >= -1 && g.IO.KeyMap[n] < ImGuiKey_LegacyNativeKey_END && "io.KeyMap[] contains an out of bound value (need to be 0..511, or -1 for unmapped key)");

    // Check: required key mapping (we intentionally do NOT check all keys to not pressure user into setting up everything, but Space is required and was only added in 1.60 WIP)
    // if ((g.IO.ConfigFlags & ImGuiConfigFlags_NavEnableKeyboard) && g.IO.BackendUsingLegacyKeyArrays == 1)
        // IM_ASSERT(g.IO.KeyMap[ImGuiKey_Space] != -1 && "ImGuiKey_Space is not mapped, required for keyboard navigation.");
// #endif

    // Check: the io.ConfigWindowsResizeFromEdges option requires backend to honor mouse cursor changes and set the ImGuiBackendFlags_HasMouseCursors flag accordingly.
    if g.IO.ConfigWindowsResizeFromEdges == true && flag_clear(&g.IO.BackendFlags, &ImGuiBackendFlags_HasMouseCursors) {
        g.IO.ConfigWindowsResizeFromEdges = false;
    }

    // Perform simple check: error if Docking or Viewport are enabled _exactly_ on frame 1 (instead of frame 0 or later), which is a common error leading to loss of .ini data.
    if g.FrameCount == 1 && flag_set(&g.IO.ConfigFlags, &ImGuiConfigFlags_DockingEnable) && flag_clear(&g.ConfigFlagsLastFrame, &ImGuiConfigFlags_DockingEnable){} else {}
        // IM_ASSERT(0 && "Please set DockingEnable before the first call to NewFrame()! Otherwise you will lose your .ini settings!");
    if g.FrameCount == 1 && flag_set(&g.IO.ConfigFlags, & ImGuiConfigFlags_ViewportsEnable) && flag_clear(&g.ConfigFlagsLastFrame, & ImGuiConfigFlags_ViewportsEnable) {}
        // IM_ASSERT(0 && "Please set ViewportsEnable before the first call to NewFrame()! Otherwise you will lose your .ini settings!");

    // Perform simple checks: multi-viewport and platform windows support
    if flag_set(&g.IO.ConfigFlags, & ImGuiConfigFlags_ViewportsEnable)
    {
        if flag_set(&g.IO.BackendFlags, & ImGuiBackendFlags_PlatformHasViewports) && flag_set(&g.IO.BackendFlags, & ImGuiBackendFlags_RendererHasViewports)
        {
            // IM_ASSERT((g.FrameCount == 0 || g.FrameCount == g.FrameCountPlatformEnded) && "Forgot to call UpdatePlatformWindows() in main loop after EndFrame()? Check examples/ applications for reference.");
            // IM_ASSERT(g.PlatformIO.Platform_CreateWindow  != NULL && "Platform init didn't install handlers?");
            // IM_ASSERT(g.PlatformIO.Platform_DestroyWindow != NULL && "Platform init didn't install handlers?");
            // IM_ASSERT(g.PlatformIO.Platform_GetWindowPos  != NULL && "Platform init didn't install handlers?");
            // IM_ASSERT(g.PlatformIO.Platform_SetWindowPos  != NULL && "Platform init didn't install handlers?");
            // IM_ASSERT(g.PlatformIO.Platform_GetWindowSize != NULL && "Platform init didn't install handlers?");
            // IM_ASSERT(g.PlatformIO.Platform_SetWindowSize != NULL && "Platform init didn't install handlers?");
            // IM_ASSERT(g.PlatformIO.Monitors.len() > 0 && "Platform init didn't setup Monitors list?");
            // IM_ASSERT((g.Viewports[0]->PlatformUserData != NULL || g.Viewports[0]->PlatformHandle != NULL) && "Platform init didn't setup main viewport.");
            if g.IO.ConfigDockingTransparentPayload == true && flag_set(&g.IO.ConfigFlags, &ImGuiConfigFlags_DockingEnable) {}
                // IM_ASSERT(g.PlatformIO.Platform_SetWindowAlpha != NULL && "Platform_SetWindowAlpha handler is required to use io.ConfigDockingTransparent!");
        }
        else
        {
            // Disable feature, our backends do not support it
            g.IO.ConfigFlags &= !ImGuiConfigFlags_ViewportsEnable;
        }

        // Perform simple checks on platform monitor data + compute a total bounding box for quick early outs
        // for (let monitor_n: c_int = 0; monitor_n < g.PlatformIO.Monitors.len(); monitor_n++)
        for monitor_n in 0 .. g.PlatformIO.Monitors.len()
        {
            let mon = g.PlatformIO.Monitors[monitor_n];
            // IM_UNUSED(mon);
            // IM_ASSERT(mon.MainSize.x > 0f32 && mon.MainSize.y > 0f32 && "Monitor main bounds not setup properly.");
            // IM_ASSERT(ImRect(mon.MainPos, mon.MainPos + mon.MainSize).Contains(ImRect(mon.WorkPos, mon.WorkPos + mon.WorkSize)) && "Monitor work bounds not setup properly. If you don't have work area information, just copy MainPos/MainSize into them.");
            // IM_ASSERT(mon.DpiScale != 0f32);
        }
    }
}

// static c_void ErrorCheckEndFrameSanityChecks()
pub unsafe fn ErrorCheckEndFrameSanityChecks()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;

    // Verify that io.KeyXXX fields haven't been tampered with. Key mods should not be modified between NewFrame() and EndFrame()
    // One possible reason leading to this assert is that your backends update inputs _AFTER_ NewFrame().
    // It is known that when some modal native windows called mid-frame takes focus away, some backends such as GLFW will
    // send key release events mid-frame. This would normally trigger this assertion and lead to sheared inputs.
    // We silently accommodate for this case by ignoring/ the case where all io.KeyXXX modifiers were released (aka key_mod_flags == 0),
    // while still correctly asserting on mid-frame key press events.
    let key_mods = GetMergedModFlags();
    // IM_ASSERT((key_mods == 0 || g.IO.KeyMods == key_mods) && "Mismatching io.KeyCtrl/io.KeyShift/io.KeyAlt/io.KeySuper vs io.KeyMods");
    // IM_UNUSED(key_mods);

    // [EXPERIMENTAL] Recover from errors: You may call this yourself before EndFrame().
    //ErrorCheckEndFrameRecover();

    // Report when there is a mismatch of Begin/BeginChild vs End/EndChild calls. Important: Remember that the Begin/BeginChild API requires you
    // to always call End/EndChild even if Begin/BeginChild returns false! (this is unfortunately inconsistent with most other Begin* API).
    if g.CurrentWindowStack.Size != 1
    {
        if g.CurrentWindowStack.Size > 1
        {
            // IM_ASSERT_USER_ERROR(g.CurrentWindowStack.Size == 1, "Mismatched Begin/BeginChild vs End/EndChild calls: did you forget to call End/EndChild?");
            while g.CurrentWindowStack.Size > 1 {
                End();
            }
        }
        else
        {
            // IM_ASSERT_USER_ERROR(g.CurrentWindowStack.Size == 1, "Mismatched Begin/BeginChild vs End/EndChild calls: did you call End/EndChild too much?");
        }
    }

    // IM_ASSERT_USER_ERROR(g.GroupStack.Size == 0, "Missing EndGroup call!");
}

// Experimental recovery from incorrect usage of BeginXXX/EndXXX/PushXXX/PopXXX calls.
// Must be called during or before EndFrame().
// This is generally flawed as we are not necessarily End/Popping things in the right order.
// FIXME: Can't recover from inside BeginTabItem/EndTabItem yet.
// FIXME: Can't recover from interleaved BeginTabBar/Begin
// c_void    ErrorCheckEndFrameRecover(ImGuiErrorLogCallback log_callback, user_data: *mut c_void)
pub unsafe fn ErrorCheckEndFrameRecover(log_callback: ImGuiErrorLogCallback, user_data: *mut c_void)
{
    // PVS-Studio V1044 is "Loop break conditions do not depend on the number of iterations"
    let g = GImGui; // ImGuiContext& g = *GImGui;
    while g.CurrentWindowStack.Size > 0 //-V1044
    {
        ErrorCheckEndWindowRecover(log_callback, user_data);
        let mut window = g.CurrentWindow;
        if g.CurrentWindowStack.Size == 1
        {
            // IM_ASSERT(window.IsFallbackWindow);
            break;
        }
        if flag_set(window.Flags.clone(), ImGuiWindowFlags_ChildWindow)
        {
            if log_callback {
                // log_callback(user_data, "Recovered from missing EndChild() for '%s'", window.Name);
            }
            EndChild();
        }
        else
        {
            if log_callback {
                // log_callback(user_data, "Recovered from missing End() for '%s'", window.Name);
            }
            End();
        }
    }
}

// Must be called before End()/EndChild()
// c_void    ErrorCheckEndWindowRecover(ImGuiErrorLogCallback log_callback, user_data: *mut c_void)
pub unsafe fn ErrocCheckEndWindowRecover(log_callback: ImGuiErrorLogCallback, user_data: *mut c_void)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    while g.CurrentTable.is_null() == false && (g.Currenttable.OuterWindow == g.CurrentWindow || g.Currenttable.InnerWindow == g.CurrentWindow)
    {
        if log_callback {
            // log_callback(user_data, "Recovered from missing EndTable() in '%s'", g.Currenttable.Outerwindow.Name);
        }
        EndTable();
    }

    let mut window = g.CurrentWindow;
    let stack_sizes = &g.CurrentWindowStack.last().unwrap().StackSizesOnBegin;
    // IM_ASSERT(window != NULL);
    while g.CurrentTabBar != null_mut() //-V1044
    {
        if log_callback {
            // log_callback(user_data, "Recovered from missing EndTabBar() in '%s'", window.Name);
        }
        EndTabBar();
    }
    while window.DC.TreeDepth > 0
    {
        // if (log_callback) {
        //     log_callback(user_data, "Recovered from missing TreePop() in '%s'", window.Name);
        // }
        TreePop();
    }
    while g.GroupStack.Size > stack_sizes.SizeOfGroupStack //-V1044
    {
        if log_callback {
            // log_callback(user_data, "Recovered from missing EndGroup() in '%s'", window.Name);
        }
        EndGroup();
    }
    while window.IDStack.Size > 1
    {
        if log_callback {
            // log_callback(user_data, "Recovered from missing PopID() in '%s'", window.Name);
        }
        PopID();
    }
    while g.DisabledStackSize > stack_sizes.SizeOfDisabledStack //-V1044
    {
        if log_callback {
            // log_callback(user_data, "Recovered from missing EndDisabled() in '%s'", window.Name);
        }
        EndDisabled();
    }
    while g.ColorStack.Size > stack_sizes.SizeOfColorStack
    {
        if log_callback {
            // log_callback(user_data, "Recovered from missing PopStyleColor() in '%s' for ImGuiCol_%s", window.Name, GetStyleColorName(g.ColorStack.last().unwrap().Col));
        }
        PopStyleColor(0);
    }
    while g.ItemFlagsStack.Size > stack_sizes.SizeOfItemFlagsStack //-V1044
    {
        if log_callback {
            // log_callback(user_data, "Recovered from missing PopItemFlag() in '%s'", window.Name);
        }
        PopItemFlag();
    }
    while g.StyleVarStack.Size > stack_sizes.SizeOfStyleVarStack //-V1044
    {
        if log_callback {
            // log_callback(user_data, "Recovered from missing PopStyleVar() in '%s'", window.Name);
        }
        PopStyleVar(0);
    }
    while g.FocusScopeStack.Size > stack_sizes.SizeOfFocusScopeStack //-V1044
    {
        if log_callback {
            // log_callback(user_data, "Recovered from missing PopFocusScope() in '%s'", window.Name);
        }
        PopFocusScope();
    }
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

// c_void DebugRenderViewportThumbnail(ImDrawList* draw_list, *mut ImGuiViewportP viewport, const ImRect& bb)
pub unsafe fn DebugRenderViewportThumbnaiL(draw_list: &mut ImDrawList, viewport: *mut ImGuiViewport, bb: &mut ImRect)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;

    let scale: ImVec2 = bb.GetSize() / viewport.Size;
    let off: ImVec2 = bb.Min - viewport.Pos * scale;
    let alpha_mul: c_float =  if flag_set(viewport.Flags, ImGuiViewportFlags_Minimized) { 0.3f32 } else { 1f32 };
    window.DrawList.AddRectFilled(&bb.Min, &bb.Max, GetColorU32(ImGuiCol_Border, alpha_mul * 0.400f32), 0f32, ImDrawFlags_None);
    // for (let i: c_int = 0; i != g.Windows.len(); i++)
    for i in 0 .. g.Windows.len()
    {
        let mut thumb_window: *mut ImGuiWindow =  g.Windows[i];
        if !thumb_window.WasActive || flag_set(thumb_window.Flags, ImGuiWindowFlags_ChildWindow) {
            continue;
        }
        if thumb_window.Viewport != viewport {
            continue;
        }

        let mut thumb_r: ImRect =  thumb_window.Rect();
        let mut title_r: ImRect =  thumb_window.TitleBarRect();
        thumb_r = ImRect(ImFloor(off + thumb_r.Min * scale), ImFloor(off +  thumb_r.Max * scale));
        title_r = ImRect(ImFloor(off + title_r.Min * scale), ImFloor(off +  ImVec2(title_r.Max.x, title_r.Min.y) * scale) + ImVec2(0,5)); // Exaggerate title bar height
        thumb_r.ClipWithFull(bb);
        title_r.ClipWithFull(bb);
        let window_is_focused: bool = (g.NavWindow.is_null() == false && thumb_window.RootWindowForTitleBarHighlight == g.NavWindow.RootWindowForTitleBarHighlight);
        window.DrawList.AddRectFilled(&thumb_r.Min, &thumb_r.Max, GetColorU32(ImGuiCol_WindowBg, alpha_mul), 0f32, ImDrawFlags_None);
        window.DrawList.AddRectFilled(&title_r.Min, &title_r.Max, GetColorU32(if window_is_focused { ImGuiCol_TitleBgActive }else { ImGuiCol_TitleBg }, alpha_mul), 0f32, ImDrawFlags_None);
        window.DrawList.AddRect(&thumb_r.Min, &thumb_r.Max, GetColorU32(ImGuiCol_Border, alpha_mul), 0f32, ImDrawFlags_None, 0f32);
        window.DrawList.AddText2(g.Font, g.FontSize * 1f32, &title_r.Min, GetColorU32(ImGuiCol_Text, alpha_mul), thumb_window.Name, FindRenderedTextEnd(thumb_window.Name, null()), 0f32, null());
    }
    draw_list.AddRect(&bb.Min, &bb.Max, GetColorU32(ImGuiCol_Border, alpha_mul), 0f32, ImDrawFlags_None, 0f32) ;
}

// static c_void RenderViewportsThumbnails()
pub unsafe fn RenderViewportsThumbnail()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;

    // We don't display full monitor bounds (we could, but it often looks awkward), instead we display just enough to cover all of our viewports.
    let SCALE: c_float =  1f32 / 8.0f32;
    let mut bb_full: ImRect = ImRect::new4(f32::MAX, f32::MAX, f32::MIN, f32::MIN);
    // for (let n: c_int = 0; n < g.Viewports.len(); n++)
    for n in 0 .. g.Viewports.len()
    {
        bb_full.Add2(&g.Viewports[n].GetMainRect());
    }
    let p: ImVec2 = window.DC.CursorPos;
    let off: ImVec2 = p - bb_full.Min * SCALE;
    // for (let n: c_int = 0; n < g.Viewports.len(); n++)
    for n in 0 .. g.Viewports.len()
    {
        let mut viewport: *mut ImGuiViewport =  g.Viewports[n];
        let mut viewport_draw_bb: ImRect = ImRect::new2(off + (viewport.Pos) * SCALE, off + (viewport.Pos + viewport.Size) * SCALE);
        DebugRenderViewportThumbnail(window.DrawList, viewport, viewport_draw_bb);
    }
    Dummy(bb_full.GetSize() * SCALE);
}

// static c_int IMGUI_CDECL ViewportComparerByFrontMostStampCount(*const c_void lhs, *const c_void rhs)
pub fn ViewportComparerByFrontMostStampCount(lhs: *const ImGuiViewport, rhs: *const ImGuiViewport) -> c_int
{
    // const let mut a: *mut ImGuiViewport =  *(const *mut ImGuiViewportP const*)lhs;
    // const let mut b: *mut ImGuiViewport =  *(const *mut ImGuiViewportP const*)rhs;
    return lhs.LastFrontMostStampCount - rhs.LastFrontMostStampCount;
}

// Helper tool to diagnose between text encoding issues and font loading issues. Pass your UTF-8 string and verify that there are correct.
// c_void DebugTextEncoding(*const char str)
pub unsafe fn DebugTextEncoding(text: *const c_char)
{
    Text("Text: \"%s\"", text);
    if !BeginTable("list", 4, ImGuiTableFlags_Borders | ImGuiTableFlags_RowBg | ImGuiTableFlags_SizingFixedFit) {
        return;
    }
    TableSetupColumn("Offset");
    TableSetupColumn("UTF-8");
    TableSetupColumn("Glyph");
    TableSetupColumn("Codepoint");
    TableHeadersRow();
    // for (*const char p = text; *p != 0; )
    let mut p = text;
    while *p != 0
    {
        let mut c = 0u32;
        let c_utf8_len: c_int = ImTextCharFromUtf8(&mut c, p, null_mut());
        TableNextColumn();
        Text("%d", (p - text));
        TableNextColumn();
        // for (let byte_index: c_int = 0; byte_index < c_utf8_len; byte_index++)
        for byte_index in 0 .. c_utf8_len
        {
            if byte_index > 0 {
                SameLine();
            }
            Text("0x%02X",p[byte_index]);
        }
        TableNextColumn();
        if GetFont().FindGlyphNoFallback(c) {
            TextUnformatted(p, p + c_utf8_len);
        }
        else {
            TextUnformatted(if c == IM_UNICODE_CODEPOINT_INVALID { "[invalid]" } else { "[missing]" });
        }
        TableNextColumn();
        Text("U+%04X", c);
        p += c_utf8_len;
    }
    EndTable();
}

// Avoid naming collision with imgui_demo.cpp's HelpMarker() for unity builds.
// static c_void MetricsHelpMarker(*const char desc)
pub unsafe fn MetricsHelpMarker(desc: *const c_char)
{
    TextDisabled("(?)");
    if (IsItemHovered(ImGuiHoveredFlags_DelayShort))
    {
        BeginTooltip();
        PushTextWrapPos(GetFontSize() * 35.00f32);
        TextUnformatted(desc);
        PopTextWrapPos();
        EndTooltip();
    }
}

// [DEBUG] List fonts in a font atlas and display its texture
// c_void ShowFontAtlas(ImFontAtlas* atlas)
pub fn ShowFontAtlas(atlas: *mut ImFontAtlas)
{
    // for (let i: c_int = 0; i < atlas.Fonts.Size; i++)
    for i in 0 .. atlas.Fonts.Size
    {
        let font = atlas.Fonts[i];
        PushID(font);
        DebugNodeFont(font);
        PopID();
    }
    if TreeNode("Atlas texture", "Atlas texture (%dx%d pixels)", atlas.TexWidth, atlas.TexHeight)
    {
        let tint_col = ImVec4::new2(1f32, 1f32, 1f32, 1f32);
        let border_col = ImVec4::new2(1f32, 1f32, 1f32, 0.5f32);
        Image(atlas.TexID, ImVec2(atlas.TexWidth, atlas.TexHeight), ImVec2(0f32, 0f32), ImVec2(1f32, 1f32), tint_col, border_col);
        TreePop();
    }
}

// Debugging enums
//     enum { WRT_OuterRect, WRT_OuterRectClipped, WRT_InnerRect, WRT_InnerClipRect, WRT_WorkRect, WRT_Content, WRT_ContentIdeal, WRT_ContentRegionRect, WRT_Count }; // Windows Rect Type
pub const WRT_OuterRect: i32 = 0;
pub const WRT_OuterRectClipped: i32 = 1;
pub const WRT_InnerRect: i32 = 2;
pub const WRT_InnerClipRect: i32 = 3;
pub const WRT_WorkRect: i32 = 4;
pub const WRT_Content: i32 = 5;
pub const WRT_ContentIdeal: i32 = 6;
pub const WRT_ContentRegionDict: i32 = 7;
pub const WRT_Count: i32 = 8;

// *const char wrt_rects_names[WRT_Count] = { "OuterRect", "OuterRectClipped", "InnerRect", "InnerClipRect", "WorkRect", "Content", "ContentIdeal", "ContentRegionRect" };
pub const wrt_rects_names: [&'static str;WRT_Count as usize] = [
    "OuterRect", "OuterRectClipped", "InnerRect", "InnerClipRect", "WorkRect", "Content", "ContentIdeal", "ContentRegionRect"
];


// enum { TRT_OuterRect, TRT_InnerRect, TRT_WorkRect, TRT_HostClipRect, TRT_InnerClipRect, TRT_BackgroundClipRect, TRT_ColumnsRect, TRT_ColumnsWorkRect, TRT_ColumnsClipRect, TRT_ColumnsContentHeadersUsed, TRT_ColumnsContentHeadersIdeal, TRT_ColumnsContentFrozen, TRT_ColumnsContentUnfrozen, TRT_Count };
pub const TRT_OuterRect: i32 = 0;
pub const TRT_InnerRect: i32 = 1;
pub const TRT_WorkRect: i32 = 2;
pub const TRT_HostClipRect: i32 = 3;
pub const TRT_InnerClipRect: i32 = 4;
pub const TRT_BackgroundClipRect: i32 = 5;
pub const TRT_ColumnsRect: i32 = 6;
pub const TRT_ColumnsWorkRect: i32 = 7;
pub const TRT_ColumnsClipRect: i32 = 8;
pub const TRT_ColumnsContentHeadersUsed: i32 = 9;
pub const TRT_ColumnsContentHeadersIdeal: i32 = 10;
pub const TRT_ColumnsContentFrozen: i32 = 11;
pub const TRT_ColumnsContentUnfrozen: i32 = 12;
pub const TRT_Count: i32 = 13;

// Tables Rect Type
//     *const char trt_rects_names[TRT_Count] = { "OuterRect", "InnerRect", "WorkRect", "HostClipRect", "InnerClipRect", "BackgroundClipRect", "ColumnsRect", "ColumnsWorkRect", "ColumnsClipRect", "ColumnsContentHeadersUsed", "ColumnsContentHeadersIdeal", "ColumnsContentFrozen", "ColumnsContentUnfrozen" };
pub const trt_rects_names: [&'static str; TRT_Count as usize] = [
    "OuterRect", "InnerRect", "WorkRect", "HostClipRect", "InnerClipRect", "BackgroundClipRect", "ColumnsRect", "ColumnsWorkRect", "ColumnsClipRect", "ColumnsContentHeadersUsed", "ColumnsContentHeadersIndeal", "ColumnsContentFrozen", "ColumnsContentUnfrozen"
];


pub struct Funcs
    {
  
    }

impl Funcs {
          // static ImRect GetTableRect(ImGuiTable* table, c_int rect_type, c_int n)
          pub fn GetTableRect(table: *mut ImGuiTable, rect_type: c_int, n: c_int) -> ImRect {
              let table_instance = TableGetInstanceData(table, table.InstanceCurrent); // Always using last submitted instance
              if rect_type == TRT_OuterRect { return table.OuterRect.clone(); } else if rect_type == TRT_InnerRect { return table.InnerRect.clone(); } else if rect_type == TRT_WorkRect { return table.WorkRect.clone(); } else if rect_type == TRT_HostClipRect { return table.HostClipRect.clone(); } else if rect_type == TRT_InnerClipRect { return table.InnerClipRect.clone(); } else if rect_type == TRT_BackgroundClipRect { return table.BgClipRect.clone(); } else if rect_type == TRT_ColumnsRect {
                  let mut c: &mut ImGuiTableColumn = &mut table.Columns[n];
                  return ImRect::new4(c.MinX, table.InnerClipRect.Min.y, c.MaxX, table.InnerClipRect.Min.y + table_instance.LastOuterHeight);
              } else if rect_type == TRT_ColumnsWorkRect {
                  let mut c = &table.Columns[n];
                  return ImRect::new4(c.WorkMinX, table.WorkRect.Min.y, c.WorkMaxX, table.WorkRect.Max.y);
              } else if rect_type == TRT_ColumnsClipRect {
                  let mut c = &table.Columns[n];
                  return c.ClipRect;
              } else if rect_type == TRT_ColumnsContentHeadersUsed {
                  let mut c = &table.Columns[n];
                  return ImRect::new4(c.WorkMinX, table.InnerClipRect.Min.y, c.ContentMaxXHeadersUsed, table.InnerClipRect.Min.y + table_instance.LastFirstRowHeight);
              } // Note: y1/y2 not always accurate
              else if rect_type == TRT_ColumnsContentHeadersIdeal {
                  let mut c = &table.Columns[n];
                  return ImRect::new4(c.WorkMinX, table.InnerClipRect.Min.y, c.ContentMaxXHeadersIdeal, table.InnerClipRect.Min.y + table_instance.LastFirstRowHeight);
              } else if rect_type == TRT_ColumnsContentFrozen {
                  let mut c = &table.Columns[n];
                  return ImRect::new4(c.WorkMinX, table.InnerClipRect.Min.y, c.ContentMaxXFrozen, table.InnerClipRect.Min.y + table_instance.LastFirstRowHeight);
              } else if rect_type == TRT_ColumnsContentUnfrozen {
                  let mut c = &table.Columns[n];
                  return ImRect::new4(c.WorkMinX, table.InnerClipRect.Min.y + table_instance.LastFirstRowHeight, c.ContentMaxXUnfrozen, table.InnerClipRect.Max.y);
              }
              // IM_ASSERT(0);
              return ImRect::new();
          }

    // static ImRect GetWindowRect(ImGuiWindow* window, c_int rect_type)
    pub fn GetWindowRect(window: *mut ImGuiWindow, rect_type: c_int) -> ImRect
    {
        if rect_type == WRT_OuterRect { return window.Rect(); }
        else if rect_type == WRT_OuterRectClipped { return window.OuterRectClipped.clone(); }
        else if rect_type == WRT_InnerRect { return window.InnerRect.clone(); }
        else if rect_type == WRT_InnerClipRect { return window.InnerClipRect.clone(); }
        else if rect_type == WRT_WorkRect { return window.WorkRect.clone(); }
        else if rect_type == WRT_Content { let mut min: ImVec2 =  window.InnerRect.Min - window.Scroll + window.WindowPadding; return ImRect::new2(&min, min + window.ContentSize); }
        else if rect_type == WRT_ContentIdeal { let mut min: ImVec2 =  window.InnerRect.Min - window.Scroll + window.WindowPadding; return ImRect::new2(&min, min + window.ContentSizeIdeal); }
        else if rect_type == WRT_ContentRegionRect { return window.ContentRegionRect.clone(); }
        // IM_ASSERT(0);
        return ImRect();
    }
}

// c_void ShowMetricsWindow(bool* p_open)
pub unsafe fn ShowMetricsWindow(p_open: *mut bool)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let io = &mut g.IO;
    let mut cfg = &mut g.DebugMetricsConfig;
    if cfg.ShowDebugLog {
    ShowDebugLogWindow(&mut cfg.ShowDebugLog);
}
    if cfg.ShowStackTool {
    ShowStackToolWindow(&cfg.ShowStackTool);
}

    if !Begin(str_to_const_c_char_ptr("Dear ImGui Metrics/Debugger"), p_open, ImGuiWindowFlags_None) || GetCurrentWindow().BeginCount > 1
    {
        End();
        return;
    }

    // Basic info
    Text("Dear ImGui %s", GetVersion());
    Text("Application average %.3f ms/frame (%.1f FPS)", 1000f32 / io.Framerate, io.Framerate);
    Text("%d vertices, %d indices (%d triangles)", io.MetricsRenderVertices, io.MetricsRenderIndices, io.MetricsRenderIndices / 3);
    Text("%d visible windows, %d active allocations", io.MetricsRenderWindows, io.MetricsActiveAllocations);
    //SameLine(); if (SmallButton("GC")) { g.GcCompactAll = true; }

    Separator();


    if cfg.ShowWindowsRectsType < 0 {
        cfg.ShowWindowsRectsType = WRT_WorkRect;
    }
    if cfg.ShowTablesRectsType < 0 {
        cfg.ShowTablesRectsType = TRT_WorkRect;
    }

    

    // Tools
    if TreeNode("Tools")
    {
        let mut show_encoding_viewer: bool =  TreeNode("UTF-8 Encoding viewer");
        SameLine();
        MetricsHelpMarker(str_to_const_c_char_ptr("You can also call DebugTextEncoding() from your code with a given string to test that your UTF-8 encoding settings are correct."));
        if show_encoding_viewer
        {
            static buf: [c_char;100] = [0;100];
            SetNextItemWidth(-FLT_MIN);
            InputText("##Text", buf, IM_ARRAYSIZE(buf));
            if buf[0] != 0 {
                DebugTextEncoding(buf.as_ptr());
            }
            TreePop();
        }

        // The Item Picker tool is super useful to visually select an item and break into the call-stack of where it was submitted.
        if Checkbox("Show Item Picker", &g.DebugItemPickerActive) && g.DebugItemPickerActive {
            DebugStartItemPicker();
        }
        SameLine();
        MetricsHelpMarker(str_to_const_c_char_ptr("Will call the IM_DEBUG_BREAK() macro to break in debugger.\nWarning: If you don't have a debugger attached, this will probably crash."));

        // Stack Tool is your best friend!
        Checkbox("Show Debug Log", &cfg.ShowDebugLog);
        SameLine();
        MetricsHelpMarker(str_to_const_c_char_ptr("You can also call ShowDebugLogWindow() from your code."));

        // Stack Tool is your best friend!
        Checkbox("Show Stack Tool", &cfg.ShowStackTool);
        SameLine();
        MetricsHelpMarker(str_to_const_c_char_ptr("You can also call ShowStackToolWindow() from your code."));

        Checkbox("Show windows begin order", &cfg.ShowWindowsBeginOrder);
        Checkbox("Show windows rectangles", &cfg.ShowWindowsRects);
        SameLine();
        SetNextItemWidth(GetFontSize() * 12);
        cfg.ShowWindowsRects |= Combo("##show_windows_rect_type", &cfg.ShowWindowsRectsType, wrt_rects_names, WRT_Count, WRT_Count);
        if cfg.ShowWindowsRects && g.NavWindow != null_mut()
        {
            BulletText("'%s':", g.NavWindow.Name);
            Indent();
            // for (let rect_n: c_int = 0; rect_n < WRT_Count; rect_n++)
            for rect_n in 0 .. WRT_Count
            {
                let mut r: ImRect =  Funcs::GetWindowRect(g.NavWindow, rect_n);
                Text("(%6.1f,%6.10f32) (%6.1f,%6.10f32) Size (%6.1f,%6.10f32) %s", r.Min.x, r.Min.y, r.Max.x, r.Max.y, r.GetWidth(), r.GetHeight(), wrt_rects_names[rect_n]);
            }
            Unindent();
        }

        Checkbox("Show tables rectangles", &cfg.ShowTablesRects);
        SameLine();
        SetNextItemWidth(GetFontSize() * 12);
        cfg.ShowTablesRects |= Combo("##show_table_rects_type", &cfg.ShowTablesRectsType, trt_rects_names, TRT_Count, TRT_Count);
        if cfg.ShowTablesRects && g.NavWindow != null_mut()
        {
            // for (let table_n: c_int = 0; table_n < g.Tables.GetMapSize(); table_n++)
            for table_n in 0 .. g.Tables.GetMapSize()
            {
                ImGuiTable* table = g.Tables.TryGetMapData(table_n);
                if table == null_mut() || table.LastFrameActive < g.FrameCount - 1 || (table.OuterWindow != g.NavWindow && table.InnerWindow != g.NavWindow) {
                    continue;
                }

                BulletText("Table 0x%08X (%d columns, in '%s')", table.ID, table.ColumnsCount, table.Outerwindow.Name);
                if IsItemHovered(ImGuiHoveredFlags_None) {
                    GetForegroundDrawList(null_mut()).AddRect(table.OuterRect.Min - ImVec2(1, 1), table.OuterRect.Max + ImVec2(1, 1), IM_COL32(255, 255, 0, 255), 0f32, 0, 2.00f32);
                }
                Indent();
                buf: [c_char;128];
                // for (let rect_n: c_int = 0; rect_n < TRT_Count; rect_n++)
                for rect_n in 0 .. TRT_Count
                {
                    if rect_n >= TRT_ColumnsRect
                    {
                        if rect_n != TRT_ColumnsRect && rect_n != TRT_ColumnsClipRect {
                            continue;
                        }
                        // for (let column_n: c_int = 0; column_n < table.ColumnsCount; column_n++)
                        for column_n in 0 .. table.ColumnsCount
                        {
                            let mut r: ImRect =  Funcs::GetTableRect(table, rect_n, column_n);
                            // ImFormatString(buf, buf.len(), str_to_const_c_char_ptr("(%6.1f,%6.10f32) (%6.1f,%6.10f32) Size (%6.1f,%6.10f32) Col %d %s"), r.Min.x, r.Min.y, r.Max.x, r.Max.y, r.GetWidth(), r.GetHeight(), column_n, trt_rects_names[rect_n]);
                            Selectable(buf);
                            if IsItemHovered(ImGuiHoveredFlags_None) {
                                GetForegroundDrawList(null_mut()).AddRect(r.Min - ImVec2(1, 1), r.Max + ImVec2(1, 1), IM_COL32(255, 255, 0, 255), 0f32, 0, 2.00f32);
                            }
                        }
                    }
                    else
                    {
                        let r: ImRect =  Funcs::GetTableRect(table, rect_n, -1);
                        // ImFormatString(buf, IM_ARRAYSIZE(buf), "(%6.1f,%6.10f32) (%6.1f,%6.10f32) Size (%6.1f,%6.10f32) %s", r.Min.x, r.Min.y, r.Max.x, r.Max.y, r.GetWidth(), r.GetHeight(), trt_rects_names[rect_n]);
                        Selectable(buf);
                        if IsItemHovered(ImGuiHoveredFlags_None) {
                            GetForegroundDrawList(null_mut()).AddRect(r.Min - ImVec2(1, 1), r.Max + ImVec2(1, 1), IM_COL32(255, 255, 0, 255), 0f32, 0, 2.00f32);
                        }
                    }
                }
                Unindent();
            }
        }

        TreePop();
    }

    // Windows
    if TreeNode("Windows", "Windows (%d)", g.Windows.len())
    {
        //SetNextItemOpen(true, ImGuiCond_Once);
        DebugNodeWindowsList(&mut g.Windows, str_to_const_c_char_ptr("By display order"));
        DebugNodeWindowsList(&mut g.WindowsFocusOrder, str_to_const_c_char_ptr("By focus order (root windows)"));
        if TreeNode("By submission order (begin stack)")
        {
            // Here we display windows in their submitted order/hierarchy, however note that the Begin stack doesn't constitute a Parent<>Child relationship!
            let temp_buffer = &mut g.WindowsTempSortBuffer;
            temp_buffer.clear();
            // for (let i: c_int = 0; i < g.Windows.len(); i++)
            for i in 0 .. g.Windows.len()
            {
                if g.Windows[i].LastFrameActive + 1 >= g.FrameCount {
                    temp_buffer.push(g.Windows[i]);
                }
            }


            ImQsort(temp_buffer.Data, temp_buffer.Size, mem::size_of::<*mut ImGuiWindow>(), Func::WindowComparerByBeginOrder);
            DebugNodeWindowsListByBeginStackParent(temp_buffer.Data, temp_buffer.Size, null_mut());
            TreePop();
        }

        TreePop();
    }

    // DrawLists
    let mut drawlist_count: c_int = 0;
    // for (let viewport_i: c_int = 0; viewport_i < g.Viewports.len(); viewport_i++)
    for viewport_i in 0.. g.Viewports.len()
    {
        drawlist_count += g.Viewports[viewport_i].DrawDataBuilder.GetDrawListCount();
    }
    if TreeNode("DrawLists", "DrawLists (%d)", drawlist_count)
    {
        Checkbox("Show ImDrawCmd mesh when hovering", &cfg.ShowDrawCmdMesh);
        Checkbox("Show ImDrawCmd bounding boxes when hovering", &cfg.ShowDrawCmdBoundingBoxes);
        // for (let viewport_i: c_int = 0; viewport_i < g.Viewports.len(); viewport_i++)
        for viewport_i in 0 .. g.Viewports.len()
        {
            let mut viewport: *mut ImGuiViewport =  g.Viewports[viewport_i];
            let mut viewport_has_drawlist: bool =  false;
            // for (let layer_i: c_int = 0; layer_i < IM_ARRAYSIZE(viewport.DrawDataBuilder.Layers); layer_i++)
            for layer_i in 0 .. IM_ARRAYSIZE(&viewport.DrawDataBuilder.Layers)
            {
                // for (let draw_list_i: c_int = 0; draw_list_i < viewport.DrawDataBuilder.Layers[layer_i].Size; draw_list_i+ +)
                for draw_list_i in 0 .. viewport.DrawDataBuilder.Layers[layer_i].len()
                {
                    if !viewport_has_drawlist {
                        Text("Active DrawLists in Viewport #%d, ID: 0x%08X", viewport.Idx, viewport.ID);
                    }
                    viewport_has_drawlist = true;
                    DebugNodeDrawList(null_mut(), viewport, viewport.DrawDataBuilder.Layers[layer_i][draw_list_i], str_to_const_c_char_ptr("DrawList"));
                }
            }
        }
        TreePop();
    }

    // Viewports
    if TreeNode("Viewports", "Viewports (%d)", g.Viewports.len())
    {
        Indent(GetTreeNodeToLabelSpacing());
        RenderViewportsThumbnails();
        Unindent(GetTreeNodeToLabelSpacing());

        let mut open: bool =  TreeNode("Monitors", "Monitors (%d)", g.PlatformIO.Monitors.len());
        SameLine();
        MetricsHelpMarker(str_to_const_c_char_ptr("Dear ImGui uses monitor data:\n- to query DPI settings on a per monitor basis\n- to position popup/tooltips so they don't straddle monitors."));
        if open
        {
            // for (let i: c_int = 0; i < g.PlatformIO.Monitors.len(); i++)
            for i in 0 .. g.PlatformIO.Monitors.len()
            {
                // const ImGuiPlatformMonitor& mon = g.PlatformIO.Monitors[i];
                let mon = g.PlatformIO.Monitors[i];
                BulletText("Monitor #%d: DPI %.0f%%\n MainMin (%.0f32,%.00f32), MainMax (%.0f32,%.00f32), MainSize (%.0f32,%.00f32)\n WorkMin (%.0f32,%.00f32), WorkMax (%.0f32,%.00f32), WorkSize (%.0f32,%.00f32)",
                    i, mon.DpiScale * 100f32,
                    mon.MainPos.x, mon.MainPos.y, mon.MainPos.x + mon.MainSize.x, mon.MainPos.y + mon.MainSize.y, mon.MainSize.x, mon.MainSize.y,
                    mon.WorkPos.x, mon.WorkPos.y, mon.WorkPos.x + mon.WorkSize.x, mon.WorkPos.y + mon.WorkSize.y, mon.WorkSize.x, mon.WorkSize.y);
            }
            TreePop();
        }

        BulletText("MouseViewport: 0x%08X (UserHovered 0x%08X, LastHovered 0x%08X)", if g.MouseViewport.is_null == false { g.MouseViewport.ID } else { 0 }, g.IO.MouseHoveredViewport, if g.MouseLastHoveredViewport.is_null == false { g.MouseLastHoveredViewport.ID } else { 0 });
        if TreeNode("Inferred Z order (front-to-back)")
        {
            // static Vec<*mut ImGuiViewportP> viewports;
            let mut Viewports: Vec<*mutImGuiViewport> = vec![];
            // viewports.resize(g.Viewports.len());
            // memcpy(viewports.Data, g.Viewports.Data, g.Viewports.size_in_bytes());
            for vp in g.Viewports {
                Viewports.push(vp);
            }
            if viewports.Size > 1 {
                ImQsort(viewports.Data, viewports.Size, mem::size_of::<*mut ImGuiViewport>(), ViewportComparerByFrontMostStampCount);
            }
            // for (let i: c_int = 0; i < viewports.Size; i++)
            for i in 0 .. viewports.len()
            {
                BulletText("Viewport #%d, ID: 0x%08X, FrontMostStampCount = %08d, Window: \"%s\"", viewports[i].Idx, viewports[i].ID, viewports[i].LastFrontMostStampCount, if viewports[i].Window { viewports[i].window.Name } else { "N/A" });
            }
            TreePop();
        }

        // for (let i: c_int = 0; i < g.Viewports.len(); i++)
        for i in 0 .. g.Viewports.len()
        {
            DebugNodeViewport(&mut g.Viewports[i]);
        }
        TreePop();
    }

    // Details for Popups
    if TreeNode("Popups", "Popups (%d)", g.OpenPopupStack.Size)
    {
        // for (let i: c_int = 0; i < g.OpenPopupStack.Size; i++)
        for i in 0 .. g.OpenPopupStack.len()
        {
            // As it's difficult to interact with tree nodes while popups are open, we display everything inline.
            let popup_data = &g.OpenPopupStack[i];
            let mut window: *mut ImGuiWindow =  popup_Data.Window;
            BulletText("PopupID: %08x, Window: '%s' (%s%s), BackupNavWindow '%s', ParentWindow '%s'",
                popup_Data.PopupId, if window { window.Name } else { "NULL" }, if window.is_null() == false && flag_set(window.Flags, ImGuiWindowFlags_ChildWindow) { "Child;" } else { "" }, if window.is_null() == false &&  flag_set(window.Flags, ImGuiWindowFlags_ChildMenu) { "Menu;" } else { "" },
                if popup_Data.BackupNavWindow { popup_Data.BackupNavwindow.Name } else { "NULL" }, if window && window.ParentWindow { window.Parentwindow.Name } else { "NULL" });
        }
        TreePop();
    }

    // Details for TabBars
    if TreeNode("TabBars", "Tab Bars (%d)", g.TabBars.GetAliveCount())
    {
        // for (let n: c_int = 0; n < g.TabBars.GetMapSize(); n++)
        for n in 0 .. g.TabBars.GetMapSize()
        {
            let tab_bar = g.TabBars.TryGetMapData(n);
            if tab_bar.is_null() == false {
                PushID(tab_bar);
                DebugNodeTabBar(tab_bar, str_to_const_c_char_ptr("TabBar"));
                PopID();
            }
        }
        TreePop();
    }

    // Details for Tables
    if TreeNode("Tables", "Tables (%d)", g.Tables.GetAliveCount())
    {
        // for (let n: c_int = 0; n < g.Tables.GetMapSize(); n++)
        for n in 0 .. g.Tables.GetMapSize()
        {
            let table = g.Tables.TryGetMapData(n);
            if table.is_null() == false {
                DebugNodeTable(table);
            }
        }
        TreePop();
    }

    // Details for Fonts
    ImFontAtlas* atlas = g.IO.Fonts;
    if TreeNode("Fonts", "Fonts (%d)", atlas.Fonts.Size)
    {
        ShowFontAtlas(atlas);
        TreePop();
    }

    // Details for InputText
    if TreeNode("InputText")
    {
        DebugNodeInputTextState(&g.InputTextState);
        TreePop();
    }

    // Details for Docking
// #ifdef IMGUI_HAS_DOCK
    if TreeNode("Docking") {
        let mut root_nodes_only: bool = true;
        let mut dc = &mut g.DockContext;
        Checkbox("List root nodes", &root_nodes_only);
        Checkbox("Ctrl shows window dock info", &cfg.ShowDockingNodes);
        if SmallButton("Clear nodes") { DockContextClearNodes(&g, 0, true); }
        SameLine();
        if SmallButton("Rebuild all") { dc.WantFullRebuild = true; }
        // for (let n: c_int = 0; n < dc->Nodes.Data.Size; n++)
        for n in 0..dc.Nodes.len() {
            let node = dc.Nodes[n].val_p;
            if node.is_null() == false {
                if !root_nodes_only || node.IsRootNode() {
                    DebugNodeDockNode(node, str_to_const_c_char_ptr("Node"));
                }
            }
        }
        TreePop();
    }
// #endif // #ifdef IMGUI_HAS_DOCK

    // Settings
    if TreeNode("Settings")
    {
        if SmallButton("Clear") {
            ClearIniSettings();
        }
        SameLine();
        if SmallButton("Save to memory") {
            SaveIniSettingsToMemory();
        }
        SameLine();
        if SmallButton("Save to disk") {
            SaveIniSettingsToDisk(g.IO.IniFilename);
        }
        SameLine();
        if g.IO.IniFilename {
            Text("\"%s\"", g.IO.IniFilename);
        }
        else {
            TextUnformatted("<NULL>");
        }
        Text("SettingsDirtyTimer %.2f", g.SettingsDirtyTimer);
        if TreeNode("SettingsHandlers", "Settings handlers: (%d)", g.SettingsHandlers.Size)
        {
            // for (let n: c_int = 0; n < g.SettingsHandlers.Size; n++)
            for n in 0 .. g.SettingsHandlers.len()
            {
                BulletText("%s", g.SettingsHandlers[n].TypeName);
            }
            TreePop();
        }
        if TreeNode("SettingsWindows", "Settings packed data: Windows: %d bytes", g.SettingsWindows.size())
        {
            // for (ImGuiWindowSettings* settings = g.SettingsWindows.begin(); settings != null_mut(); settings = g.SettingsWindows.next_chunk(settings))
            for settings in g.SettingsWindow.iter_mut()
            {
                DebugNodeWindowSettings(settings);
            }
            TreePop();
        }

        if TreeNode("SettingsTables", "Settings packed data: Tables: %d bytes", g.SettingsTables.size())
        {
            // for (ImGuiTableSettings* settings = g.SettingsTables.begin(); settings != null_mut(); settings = g.SettingsTables.next_chunk(settings))
            for settings in g.SettingsTables.iter()
            {
                DebugNodeTableSettings(settings);
            }
            TreePop();
        }

// #ifdef IMGUI_HAS_DOCK
        if TreeNode("SettingsDocking", "Settings packed data: Docking")
        {
            // ImGuiDockContext* dc = &g.DockContext;
            let mut dc = &mut g.DockContext;
            Text("In SettingsWindows:");
            // for (ImGuiWindowSettings* settings = g.SettingsWindows.begin(); settings != null_mut(); settings = g.SettingsWindows.next_chunk(settings))
            for settings in g.SettingsWindow.iter_mut()
            {
                if settings.DockId != 0 {
                    BulletText("Window '%s' -> DockId %08X", settings.GetName(), settings.DockId);
                }
            }
            Text("In SettingsNodes:");
            // for (let n: c_int = 0; n < dc->NodesSettings.Size; n++)
            for n in 0 .. dc.NodesSettings.len()
            {
                // ImGuiDockNodeSettings* settings = &dc->NodesSettings[n];
                let mut settings = &mut dc.NodesSettings[n];
                let mut  selected_tab_name: *const c_char= null_mut();
                if settings.SelectedTabId
                {
                    let window = FindWindowByID(settings.SelectedTabId);
                    if window.is_null() == false{
                    selected_tab_name = window.Name;
                }
                let window_settings = FindWindowSettings(settings.SelectedTabId);
                // else if (ImGuiWindowSettings* window_settings = FindWindowSettings(settings.SelectedTabId))
               if window_settings.is_null() == false
                {
                    selected_tab_name = window_settings.GetName();
                }
                // }
                BulletText("Node %08X, Parent %08X, SelectedTab %08X ('%s')", settings.ID, settings.ParentNodeId, settings.SelectedTabId, if selected_tab_name { selected_tab_name } else {
                    if settings.SelectedTabId {
                        "N/A"
                    }else { "" }
                });
            }
            TreePop();
        }
// #endif // #ifdef IMGUI_HAS_DOCK

        if TreeNode("SettingsIniData", "Settings unpacked data (.ini): %d bytes", g.SettingsIniData.size())
        {
            InputTextMultiline("##Ini", g.SettingsIniData.c_str(), g.SettingsIniData.Buf.Size, ImVec2::new2(-FLT_MIN, GetTextLineHeight() * 20), ImGuiInputTextFlags_ReadOnly);
            TreePop();
        }
        TreePop();
    }

    // Misc Details
    if TreeNode("Internal state")
    {
        Text("WINDOWING");
        Indent();
        Text("HoveredWindow: '%s'", if g.HoveredWindow { g.Hoveredwindow.Name }else { "NULL" });
        Text("Hoveredwindow.Root: '%s'", if g.HoveredWindow { g.Hoveredwindow.RootWindowDockTree.Name } else { "NULL" });
        Text("HoveredWindowUnderMovingWindow: '%s'", if g.HoveredWindowUnderMovingWindow { g.HoveredWindowUnderMovingwindow.Name } else { "NULL" });
        Text("HoveredDockNode: 0x%08X", if g.DebugHoveredDockNode { g.DebugHoveredDockNode.ID } else { 0 });
        Text("MovingWindow: '%s'", if g.MovingWindow { g.Movingwindow.Name } else { "NULL" });
        Text("MouseViewport: 0x%08X (UserHovered 0x%08X, LastHovered 0x%08X)", g.MouseViewport.ID, g.IO.MouseHoveredViewport, if g.MouseLastHoveredViewport { g.MouseLastHoveredViewport.ID }else { 0 });
        Unindent();

        Text("ITEMS");
        Indent();
        Text("ActiveId: 0x%08X/0x%08X (%.2f sec), AllowOverlap: %d, Source: %s", g.ActiveId, g.ActiveIdPreviousFrame, g.ActiveIdTimer, g.ActiveIdAllowOverlap, GetInputSourceName(g.ActiveIdSource));
        Text("ActiveIdWindow: '%s'", if g.ActiveIdWindow { g.ActiveIdwindow.Name } else { "NULL" });

        let mut active_id_using_key_input_count: c_int = 0;
        //for (let n: c_int = ImGuiKey_NamedKey_BEGIN; n < ImGuiKey_NamedKey_END; n++)
        for n in ImGuiKey_NamedKey_BEGIN..ImGuiKey_NamedKey_END {
            active_id_using_key_input_count += if g.ActiveIdUsingKeyInputMask[n] {
                1
            } else { 0 };
        }
        Text("ActiveIdUsing: NavDirMask: %X, KeyInputMask: %d key(s)", g.ActiveIdUsingNavDirMask, active_id_using_key_input_count);
        Text("HoveredId: 0x%08X (%.2f sec), AllowOverlap: %d", g.HoveredIdPreviousFrame, g.HoveredIdTimer, g.HoveredIdAllowOverlap); // Not displaying g.HoveredId as it is update mid-frame
        Text("HoverDelayId: 0x%08X, Timer: %.2f, ClearTimer: %.2f", g.HoverDelayId, g.HoverDelayTimer, g.HoverDelayClearTimer);
        Text("DragDrop: %d, SourceId = 0x%08X, Payload \"%s\" (%d bytes)", g.DragDropActive, g.DragDropPayload.SourceId, g.DragDropPayload.DataType, g.DragDropPayload.DataSize);
        Unindent();

        Text("NAV,FOCUS");
        Indent();
        Text("NavWindow: '%s'", if g.NavWindow { g.NavWindow.Name }else { "NULL" });
        Text("NavId: 0x%08X, NavLayer: %d", g.NavId, g.NavLayer);
        Text("NavInputSource: %s", GetInputSourceName(g.NavInputSource));
        Text("NavActive: %d, NavVisible: %d", g.IO.NavActive, g.IO.NavVisible);
        Text("NavActivateId/DownId/PressedId/InputId: %08X/%08X/%08X/%08X", g.NavActivateId, g.NavActivateDownId, g.NavActivatePressedId, g.NavActivateInputId);
        Text("NavActivateFlags: %04X", g.NavActivateFlags);
        Text("NavDisableHighlight: %d, NavDisableMouseHover: %d", g.NavDisableHighlight, g.NavDisableMouseHover);
        Text("NavFocusScopeId = 0x%08X", g.NavFocusScopeId);
        Text("NavWindowingTarget: '%s'", if g.NavWindowingTarget { g.NavWindowingTarget.Name } else { "NULL" });
        Unindent();

        TreePop();
    }

    // Overlay: Display windows Rectangles and Begin Order
    if cfg.ShowWindowsRects || cfg.ShowWindowsBeginOrder
    {
        // for (let n: c_int = 0; n < g.Windows.len(); n++)
        for n in 0 .. g.Windows.len()
        {
            let mut window: *mut ImGuiWindow =  g.Windows[n];
            if (!window.WasActive) {
                continue;
            }
            let mut  draw_list: *mut ImDrawList =  GetForegroundDrawList3(window);
            if cfg.ShowWindowsRects
            {
                let r: ImRect =  Funcs::GetWindowRect(window, cfg.ShowWindowsRectsType);
                draw_list.AddRect(&r.Min, &r.Max, IM_COL32(255, 0, 128, 255), 0f32, ImDrawFlags_None, 0f32 );
            }
            if cfg.ShowWindowsBeginOrder && flag_clear(window.Flags, ImGuiWindowFlags_ChildWindow)
            {
                buf: [c_char;32];
                // ImFormatString(buf, IM_ARRAYSIZE(buf), "%d", window.BeginOrderWithinContext);
                let font_size: c_float =  GetFontSize();
                draw_list.AddRectFilled(&window.Pos, window.Pos + ImVec2(font_size, font_size), IM_COL32(200, 100, 100, 255), 0f32, ImDrawFlags_None);
                draw_list.AddText(&window.Pos, IM_COL32(255, 255, 255, 255), buf, null());
            }
        }
    }

    // Overlay: Display Tables Rectangles
    if cfg.ShowTablesRects
    {
        // for (let table_n: c_int = 0; table_n < g.Tables.GetMapSize(); table_n++)
        for table_n in 0 .. g.Tables.GetMapSize()
        {
            ImGuiTable* table = g.Tables.TryGetMapData(table_n);
            if table == null_mut() || table.LastFrameActive < g.FrameCount - 1 {
                continue;
            }
            let mut  draw_list: *mut ImDrawList =  GetForegroundDrawList(table.OuterWindow);
            if cfg.ShowTablesRectsType >= TRT_ColumnsRect
            {
                // for (let column_n: c_int = 0; column_n < table.ColumnsCount; column_n++)
                for column_n in 0 .. table.ColumnsCount
                {
                    let r: ImRect =  Funcs::GetTableRect(table, cfg.ShowTablesRectsType, column_n);
                    let col = if table.HoveredColumnBody == column_n { IM_COL32(255, 255, 128, 255) } else { IM_COL32(255, 0, 128, 255) };
                    let thickness: c_float =  if table.HoveredColumnBody == column_n { 3.0f32 } else { 1f32 };
                    draw_list.AddRect(&r.Min, &r.Max, col, 0f32, 0, thickness );
                }
            }
            else
            {
                let r: ImRect =  Funcs::GetTableRect(table, cfg.ShowTablesRectsType, -1);
                draw_list.AddRect(&r.Min, &r.Max, IM_COL32(255, 0, 128, 255), 0f32, ImDrawFlags_None, 0f32 );
            }
        }
    }

// #ifdef IMGUI_HAS_DOCK
    // Overlay: Display Docking info
    if cfg.ShowDockingNodes && g.IO.KeyCtrl && g.DebugHoveredDockNode.is_null() == false
    {
        let buf: [c_char;64] = [0;64];
        let mut p: *const c_char = buf.as_ptr();
        // ImGuiDockNode* node = g.DebugHoveredDockNode;
        let node = &mut g.DebugHoveredDockNode;
        let mut  overlay_draw_list: *mut ImDrawList =  if node.HostWindow { GetForegroundDrawList3(node.HostWindow) } else { GetForegroundDrawList(GetMainViewport()) };
        // p += ImFormatString(p, buf + IM_ARRAYSIZE(buf) - p, "DockId: %X%s\n", node.ID, node.IsCentralNode() ? " *CentralNode*" : "");
        // p += ImFormatString(p, buf + IM_ARRAYSIZE(buf) - p, "WindowClass: %08X\n", node.WindowClass.ClassId);
        // p += ImFormatString(p, buf + IM_ARRAYSIZE(buf) - p, "Size: (%.0f32, %.00f32)\n", node.Size.x, node.Size.y);
        // p += ImFormatString(p, buf + IM_ARRAYSIZE(buf) - p, "SizeRef: (%.0f32, %.00f32)\n", node.SizeRef.x, node.SizeRef.y);
        let depth: c_int = DockNodeGetDepth(node);
        overlay_draw_list.AddRect(node.Pos + ImVec2::new2(3f32, 3f32) * depth, node.Pos + node.Size - ImVec2(3, 3) * depth, IM_COL32(200, 100, 100, 255), 0f32, ImDrawFlags_None, 0f32);
        let pos: ImVec2 = node.Pos + ImVec2(3, 3) * depth;
        overlay_draw_list.AddRectFilled(pos - ImVec2::new2(1f32, 1f32), pos + CalcTextSize(buf.as_ptr(), null(), false, 0f32) + ImVec2(1, 1), IM_COL32(200, 100, 100, 255), 0f32, ImDrawFlags_None);
        overlay_draw_list.AddText2(null_mut(), 0f32, &pos, IM_COL32(255, 255, 255, 255), buf.as_ptr(), null(), 0f32, null());
    }
// #endif // #ifdef IMGUI_HAS_DOCK

    End();
}

// [DEBUG] Display contents of Columns
// c_void DebugNodeColumns(ImGuiOldColumns* columns)
pub fn DebugNodeColumns(columns: *mut ImGuiOldColumns) {
    if !TreeNode(columns.ID, "Columns Id: 0x%08X, Count: %d, Flags: 0x%04X", columns.ID, columns.Count, columns.Flags) {
        return;
    }
    BulletText("Width: %.1f (MinX: %.1f, MaxX: %.10f32)", columns.OffMaxX - columns.OffMinX, columns.OffMinX, columns.OffMaxX);
    // for (let column_n: c_int = 0; column_n < columns.Columns.Size; column_n++)
    for column_n in 0..columns.Columns.len() {
        BulletText("Column %02d: OffsetNorm %.3f (= %.1f px)", column_n, columns.Columns[column_n].OffsetNorm, GetColumnOffsetFromNorm(columns, columns.Columns[column_n].OffsetNorm));
    }
    TreePop();
}

// static c_void DebugNodeDockNodeFlags(ImGuiDockNodeFlags* p_flags, *const char label, bool enabled)
pub unsafe fn DebugNodeDockNodeFlags(p_flags: *mut ImGuiDockNodeFlags, label: *const c_char, enabled: bool) {
    // using namespace ImGui;
    PushID(label);
    PushStyleVar(ImGuiStyleVar_FramePadding, ImVec2(0f32, 0f32));
    Text("%s:", label);
    if !enabled {
        BeginDisabled();
    }
    CheckboxFlags("NoSplit", p_flags, ImGuiDockNodeFlags_NoSplit);
    CheckboxFlags("NoResize", p_flags, ImGuiDockNodeFlags_NoResize);
    CheckboxFlags("NoResizeX", p_flags, ImGuiDockNodeFlags_NoResizeX);
    CheckboxFlags("NoResizeY", p_flags, ImGuiDockNodeFlags_NoResizeY);
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
    if !enabled {
        EndDisabled();
    }
    PopStyleVar(0);
    PopID();
}

// [DEBUG] Display contents of ImDockNode
// c_void DebugNodeDockNode(ImGuiDockNode* node, *const char label)
pub unsafe fn DebugNodeDockNode(node: *mut ImGuiDockNode, label: *const c_char)
    {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let is_alive: bool = (g.FrameCount - node.LastFrameAlive < 2);    // Submitted with ImGuiDockNodeFlags_KeepAliveOnly
    let is_active: bool = (g.FrameCount - node.LastFrameActive < 2);  // Submitted
    if !is_alive { PushStyleColor(ImGuiCol_Text, GetStyleColorU32(ImGuiCol_TextDisabled)); }
    let mut open = false;
    let tree_node_flags = if node.IsFocused { ImGuiTreeNodeFlags_Selected } else { ImGuiTreeNodeFlags_None };
    if node.Windows.len() > 0 {
        open = TreeNodeEx(node.ID, tree_node_flags, "%s 0x%04X%s: %d windows (vis: '%s')", label, node.ID, if node.IsVisible { "" } else { " (hidden)" }, node.Windows.len(), if node.VisibleWindow { node.Visiblewindow.Name } else { "NULL" });
    }
    else {
        open = TreeNodeEx(node.ID, tree_node_flags, "%s 0x%04X%s: %s split (vis: '%s')", label, node.ID, if node.IsVisible { "" } else { " (hidden)" }, if node.SplitAxis == ImGuiAxis_X { "horizontal" } else {
            if node.SplitAxis == ImGuiAxis_Y
            {
                "vertical"
            } else{ "n/a" }
        },if  node.VisibleWindow {
            node.Visiblewindow.Name
        } else { "NULL" });
    }
    if !is_alive { PopStyleColor(0); }
    if is_active && IsItemHovered(ImGuiHoveredFlags_None) {
        // let mut window: *mut ImGuiWindow = if node.HostWindow.is_null() == false {
        //     node.HostWindow
        // }  else node.VisibleWindow {
        //     GetForegroundDrawList(window) -> AddRect(node.Pos, node.Pos + node.Size, IM_COL32(255, 255, 0, 255));
        // }
    }
    if open
    {
        // IM_ASSERT(node->ChildNodes[0] == NULL || node->ChildNodes[0]->ParentNode == node);
        // IM_ASSERT(node->ChildNodes[1] == NULL || node->ChildNodes[1]->ParentNode == node);
        BulletText("Pos (%.0f32,%.00f32), Size (%.0f32, %.00f32) Ref (%.0f32, %.00f32)",
            node.Pos.x, node.Pos.y, node.Size.x, node.Size.y, node.SizeRef.x, node.SizeRef.y);
        DebugNodeWindow(node.HostWindow, str_to_const_c_char_ptr("HostWindow"));
        DebugNodeWindow(node.VisibleWindow, str_to_const_c_char_ptr("VisibleWindow"));
        BulletText("SelectedTabID: 0x%08X, LastFocusedNodeID: 0x%08X", node.SelectedTabId, node.LastFocusedNodeId);
        BulletText("Misc:%s%s%s%s%s%s%s",
            if node.IsDockSpace() { " IsDockSpace" } else { "" },
            if node.IsCentralNode() { " IsCentralNode" }else { "" },
            if is_alive { " IsAlive" } else { "" }, if is_active { " IsActive" }else { "" }, if node.IsFocused { " IsFocused" } else { "" },
            if node.WantLockSizeOnce { " WantLockSizeOnce" }else { "" },
            if node.HasCentralNodeChild { " HasCentralNodeChild" } else { "" });
        if TreeNode("flags", "Flags Merged: 0x%04X, Local: 0x%04X, InWindows: 0x%04X, Shared: 0x%04X", node.MergedFlags, node.LocalFlags, node.LocalFlagsInWindows, node.SharedFlags)
        {
            if BeginTable("flags", 4)
            {
                TableNextColumn(); DebugNodeDockNodeFlags(&mut node.MergedFlags, str_to_const_c_char_ptr("MergedFlags"), false);
                TableNextColumn(); DebugNodeDockNodeFlags(&mut node.LocalFlags, str_to_const_c_char_ptr("LocalFlags"), true);
                TableNextColumn(); DebugNodeDockNodeFlags(&mut node.LocalFlagsInWindows, str_to_const_c_char_ptr("LocalFlagsInWindows"), false);
                TableNextColumn(); DebugNodeDockNodeFlags(&mut node.SharedFlags, str_to_const_c_char_ptr("SharedFlags"), true);
                EndTable();
            }
            TreePop();
        }
        if node.ParentNode {
            DebugNodeDockNode(node.ParentNode, str_to_const_c_char_ptr("ParentNode"));
        }
        if node.ChildNodes[0] {
            DebugNodeDockNode(node.ChildNodes[0], str_to_const_c_char_ptr("Child[0]"));
        }
        if node.ChildNodes[1] {
            DebugNodeDockNode(node.ChildNodes[1], str_to_const_c_char_ptr("Child[1]"));
        }
        if node.TabBar {
            DebugNodeTabBar(node.TabBar, str_to_const_c_char_ptr("TabBar"));
        }
        DebugNodeWindowsList(&mut node.Windows, str_to_const_c_char_ptr("Windows"));

        TreePop();
    }
}

// [DEBUG] Display contents of ImDrawList
// Note that both 'window' and 'viewport' may be NULL here. Viewport is generally null of destroyed popups which previously owned a viewport.
// c_void DebugNodeDrawList(ImGuiWindow* window, *mut ImGuiViewportP viewport, *const ImDrawList draw_list, *const char label)
pub unsafe fn DebugNodeDrawList(window: *mut ImGuiWindow, viewport: *mut ImGuiViewport, draw_list: *const ImDrawList, label: *const c_char)
    {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let cfg = &g.DebugMetricsConfig;
    let mut cmd_count = draw_list.CmdBuffer.len();
    if cmd_count > 0 && draw_list.CmdBuffer.last().unwrap().ElemCount == 0 && draw_list.CmdBuffer.last().unwrap().UserCallback == null_mut() {
        cmd_count -= 1;
    }
    let mut node_open: bool =  TreeNode(draw_list, "%s: '%s' %d vtx, %d indices, %d cmds", label, if draw_list._OwnerName { draw_list._OwnerName } else { "" }, draw_list.VtxBuffer.Size, draw_list.IdxBuffer.Size, cmd_count);
    if draw_list == GetWindowDrawList()
    {
        SameLine();
        TextColored(ImVec4(1f32, 0.4f32, 0.4f32, 1f32), "CURRENTLY APPENDING"); // Can't display stats for active draw list! (we don't have the data double-buffered)
        if node_open {
            TreePop();
        }
        return;
    }

    let mut  fg_draw_list: *mut ImDrawList =  if viewport { GetForegroundDrawList(viewport) } else { null_mut() }; // Render additional visuals into the top-most draw list
    if window.is_null() == false && IsItemHovered(ImGuiHoveredFlags_None) && fg_draw_list.is_null() == false {
        fg_draw_list.AddRect(&window.Pos, window.Pos + window.Size, IM_COL32(255, 255, 0, 255), 0f32, ImDrawFlags_None, 0f32 );
    }
    if !node_open {
        return;
    }

    if window.is_null() == false && !window.WasActive {
        TextDisabled("Warning: owning Window is inactive. This DrawList is not being rendered!");
    }

    // for (*const ImDrawCmd pcmd = draw_list.CmdBuffer.Data; pcmd < draw_list.CmdBuffer.Data + cmd_count; pcmd++)
    for pcmd in draw_list.CmdBuffer.iter()
        {
        if pcmd.UserCallback
        {
            BulletText("Callback %p, user_data %p", pcmd.UserCallback, pcmd.UserCallbackData);
            continue;
        }

        let mut buf: [c_char;300] = [0;300];
        // ImFormatString(buf, IM_ARRAYSIZE(buf), "DrawCmd:%5d tris, Tex 0x%p, ClipRect (%4.0f32,%4.00f32)-(%4.0f32,%4.00f32)",
        //     pcmd->ElemCount / 3, pcmd.TextureId,
        //     pcmd->ClipRect.x, pcmd->ClipRect.y, pcmd->ClipRect.z, pcmd->ClipRect.w);
        let mut pcmd_node_open: bool =  TreeNode((pcmd - draw_list.CmdBuffer.begin()), "%s", buf);
        if IsItemHovered(ImGuiHoveredFlags_None) && (cfg.ShowDrawCmdMesh || cfg.ShowDrawCmdBoundingBoxes) && fg_draw_list.is_null() == false {
            DebugNodeDrawCmdShowMeshAndBoundingBox(&mut *fg_draw_list, draw_list, pcmd, cfg.ShowDrawCmdMesh, cfg.ShowDrawCmdBoundingBoxes);
        }
        if !pcmd_node_open {
            continue;
        }

        // Calculate approximate coverage area (touched pixel count)
        // This will be in pixels squared as long there's no post-scaling happening to the renderer output.
        let idx_buffer: *const ImDrawIdx = if draw_list.IdxBuffer.Size > 0 { draw_list.IdxBuffer.Data } else { null_mut() };
        let vtx_buffer: *const ImDrawVert = draw_list.VtxBuffer.Data + pcmd.VtxOffset;
        let mut total_area: c_float =  0f32;
        // for (let mut idx_n: c_uint =  pcmd->IdxOffset; idx_n < pcmd->IdxOffset + pcmd->ElemCount; )
        for idx_n in pcmd.IdxOffset .. pcmd.IdxOffset + pcmd.ElemCount
            {
            let mut triangle: [Imvec2;3] = [ImVec2::default();3];
            // for (let n: c_int = 0; n < 3; n++, idx_n++)
            for n in 0 .. 3
                {
                triangle[n] = vtx_buffer[if idx_buffer { idx_buffer[idx_n] }else { idx_n }].pos;
            }
            total_area += ImTriangleArea(triangle[0], triangle[1], triangle[2]);
        }

        // Display vertex information summary. Hover to get all triangles drawn in wire-frame
        // ImFormatString(buf, IM_ARRAYSIZE(buf), "Mesh: ElemCount: %d, VtxOffset: +%d, IdxOffset: +%d, Area: ~%0.f px", pcmd->ElemCount, pcmd->VtxOffset, pcmd->IdxOffset, total_area);
        Selectable(buf);
        if IsItemHovered(ImGuiHoveredFlags_None) && fg_draw_list.is_null() == false {
            DebugNodeDrawCmdShowMeshAndBoundingBox(&mut *fg_draw_list, draw_list, pcmd, true, false);
        }

        // Display individual triangles/vertices. Hover on to get the corresponding triangle highlighted.
        let mut clipper: ImGuiListClipper = ImGuiListClipper::default();
        clipper.Begin((pcmd.ElemCount / 3) as i32, 0f32); // Manually coarse clip our print out of individual vertices to save CPU, only items that may be visible.
        while clipper.Step() {
            let mut idx_i = pcmd.IdxOffset + clipper.DisplayStart;

            // for (let prim: c_int = clipper.DisplayStart, idx_i = pcmd->IdxOffset + clipper.DisplayStart * 3; prim < clipper.DisplayEnd; prim+ +)
            for prim in clipper.DisplayStart .. clipper.DisplayEnd
            {

                // char * buf_p = buf, *buf_end = buf + IM_ARRAYSIZE(buf);
                let buf_p: *mut c_char = buf.as_mut_ptr();
                let bu_end: *mut c_char = buf + IM_ARRAYSIZE(buf);
                // ImVec2
                // triangle[3];
                let mut triangle: [ImVec2;3] = [ImVec2::default();3];
                // for (let n: c_int = 0; n < 3; n+ +, idx_i+ +)
                for n in 0 .. 3
                {
                    let v = vtx_buffer[if idx_buffer { idx_buffer[idx_i] }else { idx_i }];
                    triangle[n] = v.pos;
                    // buf_p += ImFormatString(buf_p, buf_end - buf_p, "%s %04d: pos (%8.2f,%8.20f32), uv (%.6f,%.60f32), col %08X\n",
                    //                         (n == 0)? "Vert:": "     ", idx_i, v.pos.x, v.pos.y, v.uv.x, v.uv.y, v.col);
                }

                Selectable(buf, false);
                if fg_draw_list.is_null() == false && IsItemHovered(ImGuiHoveredFlags_None) {
                    let backup_flags = fg_draw_list.Flags;
                    fg_draw_list.Flags &= !ImDrawListFlags_AntiAliasedLines; // Disable AA on triangle outlines is more readable for very large and thin triangles.
                    fg_draw_list.AddPolyline(triangle.as_ptr(), 3, IM_COL32(255, 255, 0, 255), ImDrawFlags_Closed, 1f32);
                    fg_draw_list.Flags = backup_flags;
                }
            }
        }
        TreePop();
    }
    TreePop();
}

// [DEBUG] Display mesh/aabb of a ImDrawCmd
// c_void DebugNodeDrawCmdShowMeshAndBoundingBox(ImDrawList* out_draw_list, *const ImDrawList draw_list, *const ImDrawCmd draw_cmd, bool show_mesh, bool show_aabb)
pub fn DebugNodeDrawCmdShowMeshAndBoundingBox(out_draw_list: &mut ImDrawList, darw_list: *const ImDrawList, draw_cmd: *const ImDrawCmd, show_mesh: bool, show_aabb: bool)
    {
    // IM_ASSERT(show_mesh || show_aabb);

    // Draw wire-frame version of all triangles
    let clip_rect: ImRect =  draw_cmd.ClipRect.clone();
    let mut vtxs_rect: ImRect = ImRect::new4(f32::MAX, f32::MAX, f32::MIN, f32::MIN);
    let backup_flags = out_draw_list.Flags;
    out_draw_list.Flags &= !ImDrawListFlags_AntiAliasedLines; // Disable AA on triangle outlines is more readable for very large and thin triangles.
    // for (let mut idx_n: c_uint =  draw_cmd->IdxOffset, idx_end = draw_cmd->IdxOffset + draw_cmd->ElemCount; idx_n < idx_end; )
        let idx_end = draw_cmd.IdxOffset + draw_cmd.ElemCount;
    for idx_n in draw_cmd.IdxOffset .. idx_end
        {
        let idx_buffer = if draw_list.IdxBuffer.Size > 0 { draw_list.IdxBuffer.Data } else { null_mut() }; // We don't hold on those pointers past iterations as ->AddPolyline() may invalidate them if out_draw_list==draw_list
        let vtx_buffer = draw_list.VtxBuffer.Data + draw_cmd.VtxOffset;

        let mut triangle: [ImVec2; 3] = [ImVec2::default();3];
        // for (let n: c_int = 0; n < 3; n++, idx_n++)
        for n in 0 .. 3
            {
                triangle[n] = vtx_buffer[if idx_buffer { idx_buffer[idx_n] } else { idx_n }].pos;
            vtxs_rect.Add(&triangle[n]);
        }
        if show_mesh {
            out_draw_list.AddPolyline(triangle.as_ptr(), 3, IM_COL32(255, 255, 0, 255), ImDrawFlags_Closed, 1f32);
        }// In yellow: mesh triangles
    }
    // Draw bounding boxes
    if show_aabb
    {
        out_draw_list.AddRect(ImFloor(clip_rect.Min), ImFloor(clip_rect.Max), IM_COL32(255, 0, 255, 255), 0f32, ImDrawFlags_None, 0f32); // In pink: clipping rectangle submitted to GPU
        out_draw_list.AddRect(ImFloor(vtxs_rect.Min), ImFloor(vtxs_rect.Max), IM_COL32(0, 255, 255, 255), 0f32, ImDrawFlags_None, 0f32); // In cyan: bounding box of triangles
    }
    out_draw_list.Flags = backup_flags;
}

// [DEBUG] Display details for a single font, called by ShowStyleEditor().
// c_void DebugNodeFont(ImFont* font)
pub unsafe fn DebugNodeFont(font: &mut ImFont)
    {
    let mut opened: bool =  TreeNode(font, "Font: \"%s\"\n%.2f px, %d glyphs, %d file(s)",
        if font.ConfigData { font.ConfigData[0].Name } else { "" }, font.FontSize, font.Glyphs.Size, font.ConfigDataCount);
    SameLine();
    if SmallButton("Set as default") {
        GetIO().FontDefault = font;
    }
    if !opened {
        return;
    }

    // Display preview text
    PushFont(font);
    Text("The quick brown fox jumps over the lazy dog");
    PopFont();

    // Display details
    SetNextItemWidth(GetFontSize() * 8);
    DragFloat("Font scale", &font.Scale, 0.005f32, 0.3f32, 2.0f32, "%.1f");
    SameLine(); MetricsHelpMarker(
        str_to_const_c_char_ptr("Note than the default embedded font is NOT meant to be scaled.\n\n" +
        "Font are currently rendered into bitmaps at a given size at the time of building the atlas. " +
        "You may oversample them to get some flexibility with scaling. " +
        "You can also render at multiple sizes and select which one to use at runtime.\n\n" +
        "(Glimmer of hope: the atlas system will be rewritten in the future to make scaling more flexible.)"));
    Text("Ascent: %f, Descent: %f, Height: %f", font.Ascent, font.Descent, font.Ascent - font.Descent);
    let mut c_str: [c_char;5] = [0;5];
    // Text("Fallback character: '%s' (U+%04X)", ImTextCharToUtf8(c_str, font.FallbackChar), font.FallbackChar);
    // Text("Ellipsis character: '%s' (U+%04X)", ImTextCharToUtf8(c_str, font.EllipsisChar), font.EllipsisChar);
    let surface_sqrt: c_int = ImSqrt(font.MetricsTotalSurface);
    Text("Texture Area: about %d px ~%dx%d px", font.MetricsTotalSurface, surface_sqrt, surface_sqrt);
    // for (let config_i: c_int = 0; config_i < font.ConfigDataCount; config_i++)
    for config_i in 0 .. font.ConfigDataCount {
        if font.ConfigData {
            let mut cfg: &ImFontConfig = &font.ConfigData[config_i];

            if cfg {
                BulletText("Input %d: \'%s\', Oversample: (%d,%d), PixelSnapH: %d, Offset: (%.1f,%.10f32)",
                           config_i, cfg.Name, cfg.OversampleH, cfg.OversampleV, cfg.PixelSnapH, cfg.GlyphOffset.x, cfg.GlyphOffset.y);
            }
        }
    }
    // Display all glyphs of the fonts in separate pages of 256 characters
    if TreeNode("Glyphs", "Glyphs (%d)", font.Glyphs.Size)
    {
        let mut  draw_list: *mut ImDrawList =  GetWindowDrawList();
        let glyph_col = GetColorU32(ImGuiCol_Text, 0.0);
        let cell_size: c_float =  font.FontSize * 1;
        let cell_spacing: c_float =  GetStyle().ItemSpacing.y;
        // for (let mut base: c_uint =  0; base <= IM_UNICODE_CODEPOINT_MAX; base += 256)
        for mut base in (0 .. IM_UNICODE_CODEPOINT_MAX).step_by(256)
        {
            // Skip ahead if a large bunch of glyphs are not present in the font (test in chunks of 4k)
            // This is only a small optimization to reduce the number of iterations when IM_UNICODE_MAX_CODEPOINT
            // is large // (if ImWchar==ImWchar32 we will do at least about 272 queries here)
            if (base & 4095) == 0 && font.IsGlyphRangeUnused(base, base + 4095)
            {
                base += 4096 - 256;
                continue;
            }

            let mut count: c_int = 0;
            // for (let mut n: c_uint =  0; n < 256; n++)
            for n in 0 .. 256
            {
                if font.FindGlyphNoFallback((base + n)) {
                    count += 1;
                }
            }
            if count <= 0 {
                continue;
            }
            if !TreeNode(base, "U+%04X..U+%04X (%d %s)", base, base + 255, count, if count > 1 { "glyphs" }else { "glyph" }) {
                continue;
            }

            // Draw a 16x16 grid of glyphs
            let base_pos: ImVec2 = GetCursorScreenPos();
            // for (let mut n: c_uint =  0; n < 256; n++)
            for n in 0 .. 256
            {
                // We use ImFont::RenderChar as a shortcut because we don't have UTF-8 conversion functions
                // available here and thus cannot easily generate a zero-terminated UTF-8 encoded string.
                let mut cell_p1 = ImVec2::new2(base_pos.x + (n % 16) * (cell_size + cell_spacing), base_pos.y + (n / 16) * (cell_size + cell_spacing));
                let cell_p2 = ImVec2::new2(cell_p1.x + cell_size, cell_p1.y + cell_size);
                let glyph = font.FindGlyphNoFallback((base + n));
                draw_list.AddRect(&cell_p1, &cell_p2, if glyph { IM_COL32(255, 255, 255, 100) } else { IM_COL32(255, 255, 255, 50) }, 0f32, ImDrawFlags_None, 0f32);
                if !glyph {
                    continue;
                }
                font.RenderChar(draw_list, cell_size, &cell_p1, glyph_col, (base + n));
                if IsMouseHoveringRect(&cell_p1, &cell_p2, false)
                {
                    BeginTooltip();
                    DebugNodeFontGlyph(font, glyph);
                    EndTooltip();
                }
            }
            Dummy(ImVec2((cell_size + cell_spacing) * 16, (cell_size + cell_spacing) * 16));
            TreePop();
        }
        TreePop();
    }
    TreePop();
}

// c_void DebugNodeFontGlyph(ImFont*, *const ImFontGlyph glyph)
pub fn DebugNodeFontGlypn(glyph: *const ImFontGlyph) {
    Text("Codepoint: U+%04X", glyph.Codepoint);
    Separator();
    Text("Visible: %d", glyph.Visible);
    Text("AdvanceX: %.1f", glyph.AdvanceX);
    Text("Pos: (%.2f,%.20f32)->(%.2f,%.20f32)", glyph.X0, glyph.Y0, glyph.X1, glyph.Y1);
    Text("UV: (%.3f,%.30f32)->(%.3f,%.30f32)", glyph.U0, glyph.V0, glyph.U1, glyph.V1);
}

// [DEBUG] Display contents of ImGuiStorage
// c_void DebugNodeStorage(ImGuiStorage* storage, *const char label)
pub fn DebugNodeStorage(storage: *mut ImGuiStorage, label: *const c_char)
    {
    if !TreeNode(label, "%s: %d entries, %d bytes", label, storage.Data.Size, storage.Data.size_in_bytes()) {
        return;
    }
    // for (let n: c_int = 0; n < storage.Data.Size; n++)
    for n in 0 .. storage.len()
        {
        let p = storage[n];
        BulletText("Key 0x%08X Value { i: %d }", p.key, p.val_i); // Important: we currently don't store a type, real value may not be integer.
    }
    TreePop();
}

// [DEBUG] Display contents of ImGuiTabBar
// c_void DebugNodeTabBar(ImGuiTabBar* tab_bar, *const char label)
pub unsafe fn DebugNodeTabBar(tab_bar: *mut ImGuiTabBar, label: *const c_char)
    {
    // Standalone tab bars (not associated to docking/windows functionality) currently hold no discernible strings.
     let mut buf: [c_char;256] = [0;256];
    char* p = buf;
    let mut  buf_end: *const c_char = buf + IM_ARRAYSIZE(buf);
    let is_active: bool = (tab_bar.PrevFrameVisible >= GetFrameCount() - 2);
    // p += ImFormatString(p, buf_end - p, "%s 0x%08X (%d tabs)%s", label, tab_bar.ID, tab_bar.Tabs.Size, if is_active { "" } else { " *Inactive*" });
    // p += ImFormatString(p, buf_end - p, "  { ");
    // for (let tab_n: c_int = 0; tab_n < ImMin(tab_bar.Tabs.Size, 3); tab_n++)
    for tab_n in 0 .. ImMin(tab_bar.Tabs.len())
        {
        let tab = &tab_bar.Tabs[tab_n];
        // p += ImFormatString(p, buf_end - p, "%s'%s'",
        //     if tab_n > 0 { ", " } else { "" }, if (tab.Window || tab.NameOffset != -1) { tab_bar.GetTabName(tab) } else { "???" });
    }
    // p += ImFormatString(p, buf_end - p, (tab_bar.Tabs.Size > 3) ? " ... }" : " } ");
    if (!is_active) { PushStyleColor(ImGuiCol_Text, GetStyleColorU32(ImGuiCol_TextDisabled)); }
    let mut open: bool =  TreeNode(label, "%s", buf);
    if (!is_active) { PopStyleColor(0); }
    if (is_active && IsItemHovered(ImGuiHoveredFlags_None))
    {
        let mut  draw_list: *mut ImDrawList =  GetForegroundDrawList(null_mut());
        draw_list.AddRect(&tab_bar.BarRect.Min, &tab_bar.BarRect.Max, IM_COL32(255, 255, 0, 255), 0f32, ImDrawFlags_None, 0f32);
        draw_list.AddLine(ImVec2(tab_bar.ScrollingRectMinX, tab_bar.BarRect.Min.y), ImVec2(tab_bar.ScrollingRectMinX, tab_bar.BarRect.Max.y), IM_COL32(0, 255, 0, 255), 0f32);
        draw_list.AddLine(ImVec2(tab_bar.ScrollingRectMaxX, tab_bar.BarRect.Min.y), ImVec2(tab_bar.ScrollingRectMaxX, tab_bar.BarRect.Max.y), IM_COL32(0, 255, 0, 255), 0f32);
    }
    if open
    {
        // for (let tab_n: c_int = 0; tab_n < tab_bar.Tabs.Size; tab_n++)
        for tab_n in 0 .. tab_bar.Tabs.len()
        {
            let tab: *const ImGuiTabItem = &tab_bar.Tabs[tab_n];
            PushID(tab);
            if SmallButton("<") { TabBarQueueReorder(tab_bar, tab, -1); } SameLine(0, 2);
            if SmallButton(">") { TabBarQueueReorder(tab_bar, tab, 1); } SameLine();
            Text("%02d%c Tab 0x%08X '%s' Offset: %.2f, Width: %.2f/%.2f",
                 tab_n, if tab.ID == tab_bar.SelectedTabId { '*' } else { ' ' }, tab.ID, if tab.Window.is_null() == false || tab.NameOffset != -1 { tab_bar.GetTabName(tab) } else { "???" }, tab.Offset, tab.Width, tab.ContentWidth);
            PopID();
        }
        TreePop();
    }
}

// c_void DebugNodeViewport(*mut ImGuiViewportP viewport)
pub unsafe fn DebugNodeViewport(viewport: &mut ImGuiViewport)
    {
    SetNextItemOpen(true, ImGuiCond_Once);
    if TreeNode(viewport.ID, "Viewport #%d, ID: 0x%08X, Parent: 0x%08X, Window: \"%s\"", viewport.Idx, viewport.ID, viewport.ParentViewportId, if viewport.Window { viewport.window.Name }else { "N/A" })
    {
        let flags = viewport.Flags;
        BulletText("Main Pos: (%.0f32,%.00f32), Size: (%.0f32,%.00f32)\nWorkArea Offset Left: %.0f32 Top: %.0f32, Right: %.0f32, Bottom: %.0f\nMonitor: %d, DpiScale: %.0f%%",
            viewport.Pos.x, viewport.Pos.y, viewport.Size.x, viewport.Size.y,
            viewport.WorkOffsetMin.x, viewport.WorkOffsetMin.y, viewport.WorkOffsetMax.x, viewport.WorkOffsetMax.y,
            viewport.PlatformMonitor, viewport.DpiScale * 100f32);
        if viewport.Idx > 0 {
            SameLine();
            if SmallButton("Reset Pos") {
                viewport.Pos = ImVec2::new2(200f32, 200f32);
                viewport.UpdateWorkRect();
                if viewport.Window { viewport.window.Pos = viewport.Pos; }
            }
        }
        BulletText("Flags: 0x%04X =%s%s%s%s%s%s%s%s%s%s%s%s", viewport.Flags,
                   //(flags & ImGuiViewportFlags_IsPlatformWindow) ? " IsPlatformWindow" : "", // Omitting because it is the standard
            if flags & ImGuiViewportFlags_IsPlatformMonitor { " IsPlatformMonitor" } else { "" },
                   if flags & ImGuiViewportFlags_OwnedByApp { " OwnedByApp" } else { "" },
                   if flags & ImGuiViewportFlags_NoDecoration { " NoDecoration" } else {
                ""},
                   if flags & ImGuiViewportFlags_NoTaskBarIcon { " NoTaskBarIcon" } else { "" },
                   if flags & ImGuiViewportFlags_NoFocusOnAppearing { " NoFocusOnAppearing" }else { "" },
                   if flags & ImGuiViewportFlags_NoFocusOnClick { " NoFocusOnClick" } else { "" },
                   if flags & ImGuiViewportFlags_NoInputs { " NoInputs" } else { "" },
                   if flags & ImGuiViewportFlags_NoRendererClear { " NoRendererClear" } else { "" },
                   if flags & ImGuiViewportFlags_TopMost { " TopMost" } else { "" },
                   if flags & ImGuiViewportFlags_Minimized { " Minimized" } else { "" },
                   if flags & ImGuiViewportFlags_NoAutoMerge { " NoAutoMerge" } else { "" },
                   if flags & ImGuiViewportFlags_CanHostOtherWindows { " CanHostOtherWindows" }else { "" });
        // for (let layer_i: c_int = 0; layer_i < IM_ARRAYSIZE(viewport.DrawDataBuilder.Layers); layer_i++)
        for layer_i in 0 .. viewport.DrawDataBuilder.Layers.len()
        {
            // for (let draw_list_i: c_int = 0; draw_list_i < viewport.DrawDataBuilder.Layers[layer_i].Size; draw_list_i+ +)
            for draw_list_i in 0 .. viewport.DrawDataBuilder.Layers[layer_i].len()
            {
                DebugNodeDrawList(null_mut(), viewport, viewport.DrawDataBuilder.Layers[layer_i][draw_list_i], str_to_const_c_char_ptr("DrawList"));
            }
        }
        TreePop();
    }
}

// c_void DebugNodeWindow(ImGuiWindow* window, *const char label)
pub unsafe fn DebugNodeWindow(window: *mut ImGuiWindow, label: *const c_char)
    {
    if window == null_mut()
    {
        BulletText("%s: NULL", label);
        return;
    }

    let g = GImGui; // ImGuiContext& g = *GImGui;
    let is_active: bool = window.WasActive;
    let tree_node_flags = if window == g.NavWindow { ImGuiTreeNodeFlags_Selected } else { ImGuiTreeNodeFlags_None };
    if !is_active { PushStyleColor(ImGuiCol_Text, GetStyleColorU32(ImGuiCol_TextDisabled)); }
    let open: bool = TreeNodeEx(label, tree_node_flags, "%s '%s'%s", label, window.Name, if is_active { "" } else { " *Inactive*" });
    if !is_active { PopStyleColor(0); }
    if IsItemHovered(ImGuiHoveredFlags_None) && is_active {
        GetForegroundDrawList3(window).AddRect(&window.Pos, window.Pos + window.Size, IM_COL32(255, 255, 0, 255), 0f32, ImDrawFlags_None, 0f32);
    }
    if !open {
        return;
    }

    if (window.MemoryCompacted) {
        TextDisabled("Note: some memory buffers have been compacted/freed.");
    }

    let flags = window.Flags;
    DebugNodeDrawList(window, window.Viewport, window.DrawList, str_to_const_c_char_ptr("DrawList"));
    BulletText("Pos: (%.1f,%.10f32), Size: (%.1f,%.10f32), ContentSize (%.1f,%.10f32) Ideal (%.1f,%.10f32)", window.Pos.x, window.Pos.y, window.Size.x, window.Size.y, window.ContentSize.x, window.ContentSize.y, window.ContentSizeIdeal.x, window.ContentSizeIdeal.y);
    BulletText("Flags: 0x%08X (%s%s%s%s%s%s%s%s%s..)", flags,
               if flags & ImGuiWindowFlags_ChildWindow { "Child " }else { "" }, if flags & ImGuiWindowFlags_Tooltip { "Tooltip " }  else { "" }, if flags & ImGuiWindowFlags_Popup { "Popup " } else { "" },
               if flags & ImGuiWindowFlags_Modal { "Modal " }else { "" }, if flags & ImGuiWindowFlags_ChildMenu { "ChildMenu " }else { "" }, if flags & ImGuiWindowFlags_NoSavedSettings { "NoSavedSettings " }else { "" },
               if flags & ImGuiWindowFlags_NoMouseInputs { "NoMouseInputs" }else { "" }, if flags & ImGuiWindowFlags_NoNavInputs { "NoNavInputs" }else { "" }, if flags & ImGuiWindowFlags_AlwaysAutoResize { "AlwaysAutoResize" }else { "" });
    BulletText("WindowClassId: 0x%08X", window.WindowClass.ClassId);
    BulletText("Scroll: (%.2f/%.2f,%.2f/%.20f32) Scrollbar:%s%s", window.Scroll.x, window.ScrollMax.x, window.Scroll.y, window.ScrollMax.y, if window.ScrollbarX { "X" } else { "" }, if window.ScrollbarY { "Y" }else { "" });
    BulletText("Active: %d/%d, WriteAccessed: %d, BeginOrderWithinContext: %d", window.Active, window.WasActive, window.WriteAccessed, if (window.Active || window.WasActive) { window.BeginOrderWithinContext } else { -1 });
    BulletText("Appearing: %d, Hidden: %d (CanSkip %d Cannot %d), SkipItems: %d", window.Appearing, window.Hidden, window.HiddenFramesCanSkipItems, window.HiddenFramesCannotSkipItems, window.SkipItems);
    // for (let layer: c_int = 0; layer < ImGuiNavLayer_COUNT; layer++)
    for layer in 0 .. ImGuiNavLayer_COUNT
        {
        let r: ImRect =  window.NavRectRel[layer];
        if r.Min.x >= r.Max.y && r.Min.y >= r.Max.y
        {
            BulletText("NavLastIds[%d]: 0x%08X", layer, window.NavLastIds[layer]);
            continue;
        }
        BulletText("NavLastIds[%d]: 0x%08X at +(%.1f,%.10f32)(%.1f,%.10f32)", layer, window.NavLastIds[layer], r.Min.x, r.Min.y, r.Max.x, r.Max.y);
        if IsItemHovered(ImGuiHoveredFlags_None) {
            GetForegroundDrawList3(window).AddRect(r.Min + window.Pos, r.Max + window.Pos, IM_COL32(255, 255, 0, 255), 0f32, ImDrawFlags_None, 0f32);
        }
    }
    BulletText("NavLayersActiveMask: %X, NavLastChildNavWindow: %s", window.DC.NavLayersActiveMask, if window.NavLastChildNavWindow { window.NavLastChildNavwindow.Name } else { "NULL" });

    BulletText("Viewport: %d%s, ViewportId: 0x%08X, ViewportPos: (%.1f,%.10f32)", if window.Viewport { window.Viewport.Idx } else { -1 }, if window.ViewportOwned { " (Owned)" }else { "" }, window.ViewportId, window.ViewportPos.x, window.ViewportPos.y);
    BulletText("ViewportMonitor: %d", if window.Viewport { window.Viewport.PlatformMonitor } else { -1 });
    BulletText("DockId: 0x%04X, DockOrder: %d, Act: %d, Vis: %d", window.DockId, window.DockOrder, window.DockIsActive, window.DockTabIsVisible);
    if window.DockNode || window.DockNodeAsHost {
        DebugNodeDockNode(if window.DockNodeAsHost { window.DockNodeAsHost }else { window.DockNode }, if window.DockNodeAsHost { str_to_const_c_char_ptr("DockNodeAsHost") }else { str_to_const_c_char_ptr("DockNode") });
    }

    if window.RootWindow != window { DebugNodeWindow(window.RootWindow, str_to_const_c_char_ptr("RootWindow")); }
    if window.RootWindowDockTree != window.RootWindow { DebugNodeWindow(window.RootWindowDockTree, str_to_const_c_char_ptr("RootWindowDockTree")); }
    if window.ParentWindow != null_mut() { DebugNodeWindow(window.ParentWindow, str_to_const_c_char_ptr("ParentWindow")); }
    if window.DC.ChildWindows.Size > 0 { DebugNodeWindowsList(&mut window.DC.ChildWindows, str_to_const_c_char_ptr("ChildWindows")); }
    if window.ColumnsStorage.Size > 0 && TreeNode("Columns", "Columns sets (%d)", window.ColumnsStorage.Size)
    {
        // for (let n: c_int = 0; n < window.ColumnsStorage.Size; n++)
        for n in 0 .. window.ColumnsStorage.len()
        {
            DebugNodeColumns(&mut window.ColumnsStorage[n]);
        }
        TreePop();
    }
    DebugNodeStorage(&mut window.StateStorage, str_to_const_c_char_ptr("Storage"));
    TreePop();
}

// c_void DebugNodeWindowSettings(ImGuiWindowSettings* settings)
pub fn DebugNodeWindowSettings(settings: &mut ImGuiWindowSettings)
    {
    Text("0x%08X \"%s\" Pos (%d,%d) Size (%d,%d) Collapsed=%d",
        settings.ID, settings.GetName(), settings.Pos.x, settings.Pos.y, settings.Size.x, settings.Size.y, settings.Collapsed);
}

// c_void DebugNodeWindowsList(Vec<ImGuiWindow*>* windows, *const char label)
pub unsafe fn DebugNodeWindowsList(windows: *mut Vec<*mut ImGuiWindow>, label: *const c_char)
    {
    if !TreeNode(label, "%s (%d)", label, windows.Size) {
        return;
    }
    // for (let i: c_int = windows.Size - 1; i >= 0; i--) // Iterate front to back
    for i in windows.len() - 1 .. 0
        {
        PushID((*windows)[i]);
        DebugNodeWindow((*windows)[i], str_to_const_c_char_ptr("Window"));
        PopID();
    }
    TreePop();
}

// FIXME-OPT: This is technically suboptimal, but it is simpler this way.
// c_void DebugNodeWindowsListByBeginStackParent(ImGuiWindow** windows, c_int windows_size, ImGuiWindow* parent_in_begin_stack)
pub unsafe fn DebugNodeWindowsListByBeginStackParent(winodws: *mut *mut ImGuiWindow, windows_size: c_int, parent_in_begin_stack: *mut ImGuiWindow)
    {
    // for (let i: c_int = 0; i < windows_size; i++)
    for i in 0 .. windows_size
        {
        let mut window: *mut ImGuiWindow =  windows[i];
        if window.ParentWindowInBeginStack != parent_in_begin_stack {
            continue;
        }
        let mut buf: [c_char;20] = [0;20];
        // ImFormatString(buf, IM_ARRAYSIZE(buf), "[%04d] Window", window.BeginOrderWithinContext);
        //BulletText("[%04d] Window '%s'", window.BeginOrderWithinContext, window.Name);
        DebugNodeWindow(window, buf.as_ptr());
        Indent();
        DebugNodeWindowsListByBeginStackParent(windows + i + 1, windows_size - i - 1, window);
        Unindent();
    }
}

//-----------------------------------------------------------------------------
// [SECTION] DEBUG LOG
//-----------------------------------------------------------------------------

// c_void DebugLog(*const char fmt, ...)
// pub fn DebugLog()
//     {
//     va_list args;
//     va_start(args, fmt);
//     DebugLogV(fmt, args);
//     va_end(args);
// }

// c_void DebugLogV(*const char fmt, va_list args)
// {
//     let g = GImGui; // ImGuiContext& g = *GImGui;
//     let old_size: c_int = g.DebugLogBuf.size();
//     g.DebugLogBuf.appendf("[%05d] ", g.FrameCount);
//     g.DebugLogBuf.appendfv(fmt, args);
//     if (g.DebugLogFlags & ImGuiDebugLogFlags_OutputToTTY)
//         IMGUI_DEBUG_PRINTF("%s", g.DebugLogBuf.begin() + old_size);
// }

// c_void ShowDebugLogWindow(bool* p_open)
pub unsafe fn ShowDebugLogWindow(p_open: *mut bool) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if !(g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasSize) {
        SetNextWindowSize(ImVec2(0f32, GetFontSize() * 12.00f32), ImGuiCond_FirstUseEver);
    }
    if !Begin(str_to_const_c_char_ptr("Dear ImGui Debug Log"), p_open, ImGuiWindowFlags_None) || GetCurrentWindow().BeginCount > 1 {
        End();
        return;
    }

    AlignTextToFramePadding();
    Text("Log events:");
    SameLine();
    CheckboxFlags("All", &g.DebugLogFlags, ImGuiDebugLogFlags_EventMask_);
    SameLine();
    CheckboxFlags("ActiveId", &g.DebugLogFlags, ImGuiDebugLogFlags_EventActiveId);
    SameLine();
    CheckboxFlags("Focus", &g.DebugLogFlags, ImGuiDebugLogFlags_EventFocus);
    SameLine();
    CheckboxFlags("Popup", &g.DebugLogFlags, ImGuiDebugLogFlags_EventPopup);
    SameLine();
    CheckboxFlags("Nav", &g.DebugLogFlags, ImGuiDebugLogFlags_EventNav);
    SameLine();
    CheckboxFlags("Clipper", &g.DebugLogFlags, ImGuiDebugLogFlags_EventClipper);
    SameLine();
    CheckboxFlags("IO", &g.DebugLogFlags, ImGuiDebugLogFlags_EventIO);
    SameLine();
    CheckboxFlags("Docking", &g.DebugLogFlags, ImGuiDebugLogFlags_EventDocking);
    SameLine();
    CheckboxFlags("Viewport", &g.DebugLogFlags, ImGuiDebugLogFlags_EventViewport);

    if (SmallButton("Clear")) {
        g.DebugLogBuf.clear();
    }
    SameLine();
    if (SmallButton("Copy")) {
        SetClipboardText(g.DebugLogBuf.c_str());
    }
    BeginChild(str_to_const_c_char_ptr("##log"), &ImVec2::new2(0f32, 0f32), true, ImGuiWindowFlags_AlwaysVerticalScrollbar | ImGuiWindowFlags_AlwaysHorizontalScrollbar);
    TextUnformatted(g.DebugLogBuf.begin(), g.DebugLogBuf.end()); // FIXME-OPT: Could use a line index, but TextUnformatted() has a semi-decent fast path for large text.
    if GetScrollY() >= GetScrollMaxY() {
        SetScrollHereY(1f32);
    }
    EndChild();

    End();
}

//-----------------------------------------------------------------------------
// [SECTION] OTHER DEBUG TOOLS (ITEM PICKER, STACK TOOL)
//-----------------------------------------------------------------------------

// [DEBUG] Item picker tool - start with DebugStartItemPicker() - useful to visually select an item and break into its call-stack.
// c_void UpdateDebugToolItemPicker()
pub unsafe fn UpdateDebugToolItemPicker()
    {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.DebugItemPickerBreakId = 0;
    if (!g.DebugItemPickerActive) {
        return;
    }

    let mut hovered_id =  g.HoveredIdPreviousFrame;
    SetMouseCursor(ImGuiMouseCursor_Hand);
    if IsKeyPressed(ImGuiKey_Escape, false) {
        g.DebugItemPickerActive = false;
    }
    let change_mapping: bool = g.IO.KeyMods == (ImGuiModFlags_Ctrl | ImGuiModFlags_Shift);
    if !change_mapping && IsMouseClicked(g.DebugItemPickerMouseButton, false) && hovered_id != 0
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
    SetNextWindowBgAlpha(0.700f32);
    BeginTooltip();
    Text("HoveredId: 0x%08X", hovered_id);
    Text("Press ESC to abort picking.");
    let mouse_button_names: [&'static str;3] = [ "Left", "Right", "Middle" ];
    if (change_mapping) {
        Text("Remap w/ Ctrl+Shift: click anywhere to select new mouse button.");
    }
    else {
        TextColored(GetStyleColorU32(if hovered_id? { ImGuiCol_Text }else{ ImGuiCol_TextDisabled }), "Click %s Button to break in debugger! (remap w/ Ctrl+Shift)", mouse_button_names[g.DebugItemPickerMouseButton]);
    }
    EndTooltip();
}

// [DEBUG] Stack Tool: update queries. Called by NewFrame()
// c_void UpdateDebugToolStackQueries()
pub unsafe fn UpdateDebugToolStackQueries()
    {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut tool =  &mut g.DebugStackTool;

    // Clear hook when stack tool is not visible
    g.DebugHookIdInfo = 0;
    if (g.FrameCount != tool.LastActiveFrame + 1) {
        return;
    }

    // Update queries. The steps are: -1: query Stack, >= 0: query each stack item
    // We can only perform 1 ID Info query every frame. This is designed so the GetID() tests are cheap and constant-time
    let mut query_id: ImGuiID =  if g.HoveredIdPreviousFrame { g.HoveredIdPreviousFrame } else { g.ActiveId };
    if tool.QueryId != query_id
    {
        tool.QueryId = query_id;
        tool.StackLevel = -1;
        tool.Results.clear();
    }
    if query_id == 0 {
        return;
    }

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

// static c_int StackToolFormatLevelInfo(ImGuiStackTool* tool, c_int n, bool format_for_ui, char* buf, size_t buf_size)
pub unsafe fn StackToolFormatLevelInfo(tool: *mut ImGuiStackTool, n: c_int, format_for_ui: bool, buf: *mut c_char, buf_size: size_t) -> c_int
    {
    let mut info: &mut ImGuiStackLevelInfo=  &mut tool.Results[n];
    let mut window: *mut ImGuiWindow =  if info.Desc[0] == 0 && n == 0 { FindWindowByID(info.ID) } else { null_mut() };
    if window {                                                              // Source: window name (because the root ID don't call GetID() and so doesn't get hooked)
        // return ImFormatString(buf, buf_size, format_for_ui? "\"%s\" [window]": "%s", window.Name);
        return 0;
    }
    if info.QuerySuccess {                                                    // Source: GetID() hooks (prioritize over ItemInfo() because we frequently use patterns like: PushID(str), Button("") where they both have same id)
        // return ImFormatString(buf, buf_size, (format_for_ui && info.DataType == ImGuiDataType_String)? "\"%s\"": "%s", info.Desc);
        return 0;
    }
    if tool.StackLevel < tool.Results.Size {                             // Only start using fallback below when all queries are done, so during queries we don't flickering ??? markers.
        // return (*buf = 0);
        return 0;
    }
// // #ifdef IMGUI_ENABLE_TEST_ENGINE
//         let label = ImGuiTestEngine_FindItemDebugLabel(GImGui, info.ID);
//     if label != null() {  // Source: ImGuiTestEngine's ItemInfo()
//         // return ImFormatString(buf, buf_size, format_for_ui? "??? \"%s\"": "%s", label);
//         return 0;
//     }
// // #endif
//     return ImFormatString(buf, buf_size, "???");
        return 0;
}

// Stack Tool: Display UI
// c_void ShowStackToolWindow(bool* p_open)
pub unsafe fn ShowStackToolWIndow(p_open: *mut bool)
    {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if (!(g.NextWindowData.Flags & ImGuiNextWindowDataFlags_HasSize)) {
        SetNextWindowSize(ImVec2(0f32, GetFontSize() * 8.00f32), ImGuiCond_FirstUseEver);
    }
    if !Begin(str_to_const_c_char_ptr("Dear ImGui Stack Tool"), p_open, 0) || GetCurrentWindow().BeginCount > 1
    {
        End();
        return;
    }

    // Display hovered/active status
    let mut tool: *mut ImGuiStackTool =  &mut g.DebugStackTool;
    let mut hovered_id: ImGuiID =  g.HoveredIdPreviousFrame;
    let mut active_id: ImGuiID =  g.ActiveId;
// #ifdef IMGUI_ENABLE_TEST_ENGINE
    Text("HoveredId: 0x%08X (\"%s\"), ActiveId:  0x%08X (\"%s\")", hovered_id, if hovered_id { ImGuiTestEngine_FindItemDebugLabel(&g, hovered_id) } else { "" }, active_id, if active_id { ImGuiTestEngine_FindItemDebugLabel(&g, active_id) } else { "" });
// #else
    Text("HoveredId: 0x%08X, ActiveId:  0x%08X", hovered_id, active_id);
// #endif
    SameLine();
    MetricsHelpMarker(str_to_const_c_char_ptr("Hover an item with the mouse to display elements of the ID Stack leading to the item's final ID.\nEach level of the stack correspond to a PushID() call.\nAll levels of the stack are hashed together to make the final ID of a widget (ID displayed at the bottom level of the stack).\nRead FAQ entry about the ID stack for details."));

    // CTRL+C to copy path
    let time_since_copy: c_float = (g.Time - tool.CopyToClipboardLastTime) as c_float;
    Checkbox("Ctrl+C: copy path to clipboard", &tool.CopyToClipboardOnCtrlC);
    SameLine();
    TextColored(if(time_since_copy >= 0f32 && time_since_copy < 0.75f32 && ImFmod(time_since_copy, 0.250f32) < 0.25f32 * 0.5f32) { ImVec4(1.f, 1.f, 0.3f32, 1f32) } else { ImVec4::default() }, "*COPIED*");
    if tool.CopyToClipboardOnCtrlC && IsKeyDown(ImGuiKey_ModCtrl) && IsKeyPressed(ImGuiKey_C, false)
    {
        tool.CopyToClipboardLastTime = g.Time as c_float;
        let mut p = g.TempBuffer.as_mut_ptr();
        let p_end = p + g.TempBuffer.len();
        // for (let stack_n: c_int = 0; stack_n < tool.Results.Size && p + 3 < p_end; stack_n++)
        for stack_n in 0 .. tool.Results.len()
        {
            *p = '/' as c_char;
            p += 1;
            let mut level_desc: [c_char;256] = [0;256];
            StackToolFormatLevelInfo(tool, stack_n as c_int, false, level_desc.as_mut_ptr(), IM_ARRAYSIZE(level_desc));
            // for (let n: c_int = 0; level_desc[n] && p + 2 < p_end; n++)
            for n in 0 .. level_desc[n]
            {
                if level_desc[n] == '/' as c_char {
                    *p = '\\' as c_char;
                    p +=1 ;
                }
                *p = level_desc[n];
                p += 1;

                if !(p + 2 < p_end) {
                    break;
                }
            }

            if !(p + 3 < p_end) {
            break;}
        }
        }
        *p = '\0';
        SetClipboardText(g.TempBuffer.Data);
    }

    // Display decorated stack
    tool.LastActiveFrame = g.FrameCount;
    if tool.Results.Size > 0 && BeginTable("##table", 3, ImGuiTableFlags_Borders)
    {
        let id_width: c_float =  CalcTextSize("0xDDDDDDDD", null(), false, 0f32).x;
        TableSetupColumn("Seed", ImGuiTableColumnFlags_WidthFixed, id_width);
        TableSetupColumn("PushID", ImGuiTableColumnFlags_WidthStretch);
        TableSetupColumn("Result", ImGuiTableColumnFlags_WidthFixed, id_width);
        TableHeadersRow();
        // for (let n: c_int = 0; n < tool.Results.Size; n++)
        for n in 0 .. tool.Results.len()
        {
            let mut info: *mut ImGuiStackLevelInfo =  &mut tool.Results[n];
            TableNextColumn();
            Text("0x%08X", if n > 0 { tool.Results[n - 1].ID } else { 0 });
            TableNextColumn();
            StackToolFormatLevelInfo(tool, n, true, g.TempBuffer.Data, g.TempBuffer.Size);
            TextUnformatted(g.TempBuffer.Data);
            TableNextColumn();
            Text("0x%08X", info.ID);
            if n == tool.Results.Size - 1 {
                TableSetBgColor(ImGuiTableBgTarget_CellBg, GetColorU32(ImGuiCol_Header, 0.0));
            }
        }
        EndTable();
    }
    End();
}

// #else
//
// c_void ShowMetricsWindow(bool*) {}
// c_void ShowFontAtlas(ImFontAtlas*) {}
// c_void DebugNodeColumns(ImGuiOldColumns*) {}
// c_void DebugNodeDrawList(ImGuiWindow*, *mut ImGuiViewportP, *const ImDrawList, *const char) {}
// c_void DebugNodeDrawCmdShowMeshAndBoundingBox(ImDrawList*, *const ImDrawList, *const ImDrawCmd, bool, bool) {}
// c_void DebugNodeFont(ImFont*) {}
// c_void DebugNodeStorage(ImGuiStorage*, *const char) {}
// c_void DebugNodeTabBar(ImGuiTabBar*, *const char) {}
// c_void DebugNodeWindow(ImGuiWindow*, *const char) {}
// c_void DebugNodeWindowSettings(ImGuiWindowSettings*) {}
// c_void DebugNodeWindowsList(Vec<ImGuiWindow*>*, *const char) {}
// c_void DebugNodeViewport {}
//
// c_void DebugLog(*const char, ...) {}
// c_void DebugLogV(*const char, va_list) {}
// c_void ShowDebugLogWindow(bool*) {}
// c_void ShowStackToolWindow(bool*) {}
// c_void DebugHookIdInfo(ImGuiID, ImGuiDataType, *const c_void, *const c_void) {}
// c_void UpdateDebugToolItemPicker() {}
// c_void UpdateDebugToolStackQueries() {}

// #endif // #ifndef IMGUI_DISABLE_DEBUG_TOOLS

//-----------------------------------------------------------------------------

// Include imgui_user.inl at the end of imgui.cpp to access private data/functions that aren't exposed.
// Prefer just including imgui_internal.h from your code rather than using this define. If a declaration is missing from imgui_internal.h add it or request it on the github.
// #ifdef IMGUI_INCLUDE_IMGUI_USER_INL
// #include "imgui_user.inl"
// #endif

//-----------------------------------------------------------------------------

// #endif // #ifndef IMGUI_DISABLE
