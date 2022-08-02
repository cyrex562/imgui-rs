use crate::config::ConfigFlags;
use crate::Context;
use crate::globals::GImGui;
use crate::orig_imgui_single_file::ImGuiPlatformMonitor;
use crate::style::pop_style_color;
use crate::window::WindowFlags;

// Helper function to verify ABI compatibility between caller code and compiled version of Dear ImGui.
// Verify that the type sizes are matching between the calling file's compilation unit and imgui.cpp's compilation unit
// If this triggers you have an issue:
// - Most commonly: mismatched headers and compiled code version.
// - Or: mismatched configuration #define, compilation settings, packing pragma etc.
//   The configuration settings mentioned in imconfig.h must be set for all compilation units involved with Dear ImGui,
//   which is way it is required you put them in your imconfig file (and not just before including imgui.h).
//   Otherwise it is possible that different compilation units would see different structure layout
// bool DebugCheckVersionAndDataLayout(const char* version, size_t sz_io, size_t sz_style, size_t sz_vec2, size_t sz_vec4, size_t sz_vert, size_t sz_idx)
pub fn debug_check_version_and_data_layout(g: &mut Context, version: &str, sz_io:usize, sz_style: usize, sz_vec2: usize, sz_vec4: usize, sz_vert: usize, sz_idx: usize) -> bool
{
    bool error = false;
    if (strcmp(version, IMGUI_VERSION) != 0) { error = true; // IM_ASSERT(strcmp(version, IMGUI_VERSION) == 0 && "Mismatched version string!"); }
    if (sz_io != sizeof(ImGuiIO)) { error = true; // IM_ASSERT(sz_io == sizeof(ImGuiIO) && "Mismatched struct layout!"); }
    if (sz_style != sizeof(ImGuiStyle)) { error = true; // IM_ASSERT(sz_style == sizeof(ImGuiStyle) && "Mismatched struct layout!"); }
    if (sz_vec2 != sizeof(Vector2D)) { error = true; // IM_ASSERT(sz_vec2 == sizeof(Vector2D) && "Mismatched struct layout!"); }
    if (sz_vec4 != sizeof(Vector4D)) { error = true; // IM_ASSERT(sz_vec4 == sizeof(Vector4D) && "Mismatched struct layout!"); }
    if (sz_vert != sizeof(ImDrawVert)) { error = true; // IM_ASSERT(sz_vert == sizeof(ImDrawVert) && "Mismatched struct layout!"); }
    if (sz_idx != sizeof) { error = true; // IM_ASSERT(sz_idx == sizeof(ImDrawIdx) && "Mismatched struct layout!"); }
    return !error;
}

// static void error_check_new_frame_sanity_checks()
pub fn error_check_new_frame_sanity_checks(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;

    // Check user IM_ASSERT macro
    // (IF YOU GET A WARNING OR COMPILE ERROR HERE: it means your assert macro is incorrectly defined!
    //  If your macro uses multiple statements, it NEEDS to be surrounded by a 'do { ... } while (0)' block.
    //  This is a common C/C++ idiom to allow multiple statements macros to be used in control flow blocks.)
    // #define IM_ASSERT(EXPR)   if (SomeCode(EXPR)) SomeMoreCode();                    // Wrong!
    // #define IM_ASSERT(EXPR)   do { if (SomeCode(EXPR)) SomeMoreCode(); } while (0)   // Correct!
    if (true) // IM_ASSERT(1); else IM_ASSERT(0);

    // Check user data
    // (We pass an error message in the assert expression to make it visible to programmers who are not using a debugger, as most assert handlers display their argument)
    // IM_ASSERT(g.initialized);
    // IM_ASSERT((g.io.delta_time > 0.0 || g.frame_count == 0)              && "Need a positive delta_time!");
    // IM_ASSERT((g.frame_count == 0 || g.frame_count_ended == g.frame_count)  && "Forgot to call Render() or EndFrame() at the end of the previous frame?");
    // IM_ASSERT(g.io.display_size.x >= 0.0 && g.io.display_size.y >= 0.0  && "Invalid display_size value!");
    // IM_ASSERT(g.io.fonts.IsBuilt()                                     && "font Atlas not built! Make sure you called ImGui_ImplXXXX_NewFrame() function for renderer backend, which should call io.fonts->GetTexDataAsRGBA32() / GetTexDataAsAlpha8()");
    // IM_ASSERT(g.style.curve_tessellation_tol > 0.0                       && "Invalid style setting!");
    // IM_ASSERT(g.style.circle_tessellation_max_error > 0.0                 && "Invalid style setting!");
    // IM_ASSERT(g.style.alpha >= 0.0 && g.style.alpha <= 1.0            && "Invalid style setting!"); // Allows us to avoid a few clamps in color computations
    // IM_ASSERT(g.style.window_min_size.x >= 1.0 && g.style.window_min_size.y >= 1.0 && "Invalid style setting.");
    // IM_ASSERT(g.style.window_menu_button_position == Dir::None || g.style.window_menu_button_position == Dir::Left || g.style.window_menu_button_position == Dir::Right);
    // IM_ASSERT(g.style.ColorButtonPosition == Dir::Left || g.style.ColorButtonPosition == Dir::Right);
#ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
    for (int n = ImGuiKey_NamedKey_BEGIN; n < ImGuiKey_COUNT; n += 1)
        // IM_ASSERT(g.io.key_map[n] >= -1 && g.io.key_map[n] < ImGuiKey_LegacyNativeKey_END && "io.key_map[] contains an out of bound value (need to be 0..511, or -1 for unmapped key)");

    // Check: required key mapping (we intentionally do NOT check all keys to not pressure user into setting up everything, but Space is required and was only added in 1.60 WIP)
    if ((g.io.config_flags & ConfigFlags::NavEnableKeyboard) && g.io.backend_using_legacy_key_arrays == 1)
        // IM_ASSERT(g.io.key_map[ImGuiKey_Space] != -1 && "ImGuiKey_Space is not mapped, required for keyboard navigation.");


    // Check: the io.config_windows_resize_from_edges option requires backend to honor mouse cursor changes and set the ImGuiBackendFlags_HasMouseCursors flag accordingly.
    if (g.io.ConfigWindowsResizeFromEdges && !(g.io.backend_flags & ImGuiBackendFlags_HasMouseCursors))
        g.io.ConfigWindowsResizeFromEdges = false;

    // Perform simple check: error if Docking or viewport are enabled _exactly_ on frame 1 (instead of frame 0 or later), which is a common error leading to loss of .ini data.
    if (g.frame_count == 1 && (g.io.config_flags & ImGuiConfigFlags_DockingEnable) && (g.config_flags_last_frame & ImGuiConfigFlags_DockingEnable) == 0)
        // IM_ASSERT(0 && "Please set DockingEnable before the first call to NewFrame()! Otherwise you will lose your .ini settings!");
    if (g.frame_count == 1 && (g.io.config_flags & ConfigFlags::ViewportsEnable) && (g.config_flags_last_frame & ConfigFlags::ViewportsEnable) == 0)
        // IM_ASSERT(0 && "Please set ViewportsEnable before the first call to NewFrame()! Otherwise you will lose your .ini settings!");

    // Perform simple checks: multi-viewport and platform windows support
    if (g.io.config_flags & ConfigFlags::ViewportsEnable)
    {
        if ((g.io.backend_flags & ImGuiBackendFlags_PlatformHasViewports) && (g.io.backend_flags & ImGuiBackendFlags_RendererHasViewports))
        {
            // IM_ASSERT((g.frame_count == 0 || g.frame_count == g.FrameCountPlatformEnded) && "Forgot to call UpdatePlatformWindows() in main loop after EndFrame()? Check examples/ applications for reference.");
            // IM_ASSERT(g.platform_io.Platform_CreateWindow  != None && "Platform init didn't install handlers?");
            // IM_ASSERT(g.platform_io.Platform_DestroyWindow != None && "Platform init didn't install handlers?");
            // IM_ASSERT(g.platform_io.Platform_GetWindowPos  != None && "Platform init didn't install handlers?");
            // IM_ASSERT(g.platform_io.Platform_SetWindowPos  != None && "Platform init didn't install handlers?");
            // IM_ASSERT(g.platform_io.Platform_GetWindowSize != None && "Platform init didn't install handlers?");
            // IM_ASSERT(g.platform_io.Platform_set_window_size != None && "Platform init didn't install handlers?");
            // IM_ASSERT(g.platform_io.monitors.size > 0 && "Platform init didn't setup Monitors list?");
            // IM_ASSERT((g.viewports[0].PlatformUserData != None || g.viewports[0].PlatformHandle != None) && "Platform init didn't setup main viewport.");
            if (g.io.config_docking_transparent_payload && (g.io.config_flags & ImGuiConfigFlags_DockingEnable))
                // IM_ASSERT(g.platform_io.Platform_SetWindowAlpha != None && "Platform_SetWindowAlpha handler is required to use io.ConfigDockingTransparent!");
        }
        else
        {
            // Disable feature, our backends do not support it
            g.io.config_flags &= ~ConfigFlags::ViewportsEnable;
        }

        // Perform simple checks on platform monitor data + compute a total bounding box for quick early outs
        for (int monitor_n = 0; monitor_n < g.platform_io.monitors.size; monitor_n += 1)
        {
            ImGuiPlatformMonitor& mon = g.platform_io.monitors[monitor_n];
            IM_UNUSED(mon);
            // IM_ASSERT(mon.MainSize.x > 0.0 && mon.MainSize.y > 0.0 && "Monitor main bounds not setup properly.");
            // IM_ASSERT(Rect(mon.MainPos, mon.MainPos + mon.MainSize).contains(Rect(mon.WorkPos, mon.WorkPos + mon.work_size)) && "Monitor work bounds not setup properly. If you don't have work area information, just copy MainPos/MainSize into them.");
            // IM_ASSERT(mon.DpiScale != 0.0);
        }
    }
}

// static void error_check_end_frame_sanity_checks()
pub fn error_check_end_frame_sanity_checks(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;

    // Verify that io.KeyXXX fields haven't been tampered with. Key mods should not be modified between NewFrame() and EndFrame()
    // One possible reason leading to this assert is that your backends update inputs _AFTER_ NewFrame().
    // It is known that when some modal native windows called mid-frame takes focus away, some backends such as GLFW will
    // send key release events mid-frame. This would normally trigger this assertion and lead to sheared inputs.
    // We silently accommodate for this case by ignoring/ the case where all io.KeyXXX modifiers were released (aka key_mod_flags == 0),
    // while still correctly asserting on mid-frame key press events.
    const ImGuiModFlags key_mods = get_merged_mod_flags();
    // IM_ASSERT((key_mods == 0 || g.io.key_mods == key_mods) && "Mismatching io.key_ctrl/io.key_shift/io.key_alt/io.key_super vs io.key_mods");
    IM_UNUSED(key_mods);

    // [EXPERIMENTAL] Recover from errors: You may call this yourself before EndFrame().
    //ErrorCheckEndFrameRecover();

    // Report when there is a mismatch of Begin/BeginChild vs End/EndChild calls. Important: Remember that the Begin/BeginChild API requires you
    // to always call End/EndChild even if Begin/BeginChild returns false! (this is unfortunately inconsistent with most other Begin* API).
    if (g.current_window_stack.size != 1)
    {
        if (g.current_window_stack.size > 1)
        {
            // IM_ASSERT_USER_ERROR(g.current_window_stack.size == 1, "Mismatched Begin/BeginChild vs End/EndChild calls: did you forget to call End/EndChild?");
            while (g.current_window_stack.size > 1)
                end();
        }
        else
        {
            // IM_ASSERT_USER_ERROR(g.current_window_stack.size == 1, "Mismatched Begin/BeginChild vs End/EndChild calls: did you call End/EndChild too much?");
        }
    }

    // IM_ASSERT_USER_ERROR(g.group_stack.size == 0, "Missing EndGroup call!");
}

// Experimental recovery from incorrect usage of BeginXXX/EndXXX/PushXXX/PopXXX calls.
// Must be called during or before EndFrame().
// This is generally flawed as we are not necessarily End/Popping things in the right order.
// FIXME: Can't recover from inside BeginTabItem/EndTabItem yet.
// FIXME: Can't recover from interleaved BeginTabBar/Begin
// void    ErrorCheckEndFrameRecover(ImGuiErrorLogCallback log_callback, void* user_data)
pub fn error_check_end_frame_recover(g: &mut Context, log_callback: ErrorLogCallback, user_data: &Vec<u8>)
{
    // PVS-Studio V1044 is "Loop break conditions do not depend on the number of iterations"
    // ImGuiContext& g = *GImGui;
    while (g.current_window_stack.size > 0) //-V1044
    {
        ErrorCheckEndWindowRecover(log_callback, user_data);
        ImGuiWindow* window = g.current_window;
        if (g.current_window_stack.size == 1)
        {
            // IM_ASSERT(window.is_fallback_window);
            break;
        }
        if (window.flags & WindowFlags::ChildWindow)
        {
            if (log_callback) log_callback(user_data, "Recovered from missing EndChild() for '%s'", window.name);
            end_child();
        }
        else
        {
            if (log_callback) log_callback(user_data, "Recovered from missing End() for '%s'", window.name);
            end();
        }
    }
}

// Must be called before End()/EndChild()
// void    ErrorCheckEndWindowRecover(ImGuiErrorLogCallback log_callback, void* user_data)
pub fn error_check_end_window_recover(g: &mut Context, log_callback: ErrorLogCallback, user_data: &Vec<u8>)
{
    // ImGuiContext& g = *GImGui;
    while (g.current_table && (g.current_table.OuterWindow == g.current_window || g.current_table.InnerWindow == g.current_window))
    {
        if (log_callback) log_callback(user_data, "Recovered from missing EndTable() in '%s'", g.current_table.OuterWindow.name);
        EndTable();
    }

    ImGuiWindow* window = g.current_window;
    ImGuiStackSizes* stack_sizes = &g.current_window_stack.back().StackSizesOnBegin;
    // IM_ASSERT(window != None);
    while (g.CurrentTabBar != None) //-V1044
    {
        if (log_callback) log_callback(user_data, "Recovered from missing EndTabBar() in '%s'", window.name);
        end_tab_bar();
    }
    while (window.dc.TreeDepth > 0)
    {
        if (log_callback) log_callback(user_data, "Recovered from missing TreePop() in '%s'", window.name);
        TreePop();
    }
    while (g.group_stack.size > stack_sizes.sizeOfGroupStack) //-V1044
    {
        if (log_callback) log_callback(user_data, "Recovered from missing EndGroup() in '%s'", window.name);
        EndGroup();
    }
    while (window.idStack.size > 1)
    {
        if (log_callback) log_callback(user_data, "Recovered from missing PopID() in '%s'", window.name);
        pop_id();
    }
    while (g.disabled_stack_size > stack_sizes.sizeOfDisabledStack) //-V1044
    {
        if (log_callback) log_callback(user_data, "Recovered from missing EndDisabled() in '%s'", window.name);
        EndDisabled();
    }
    while (g.color_stack.size > stack_sizes.sizeOfColorStack)
    {
        if (log_callback) log_callback(user_data, "Recovered from missing PopStyleColor() in '%s' for ImGuiCol_%s", window.name, GetStyleColorName(g.color_stack.back().Col));
        pop_style_color();
    }
    while (g.item_flags_stack.size > stack_sizes.sizeOfItemFlagsStack) //-V1044
    {
        if (log_callback) log_callback(user_data, "Recovered from missing PopItemFlag() in '%s'", window.name);
        pop_item_flag();
    }
    while (g.style_var_stack.size > stack_sizes.sizeOfStyleVarStack) //-V1044
    {
        if (log_callback) log_callback(user_data, "Recovered from missing PopStyleVar() in '%s'", window.name);
        pop_style_var();
    }
    while (g.FocusScopeStack.size > stack_sizes.sizeOfFocusScopeStack) //-V1044
    {
        if (log_callback) log_callback(user_data, "Recovered from missing PopFocusScope() in '%s'", window.name);
        PopFocusScope();
    }
}
