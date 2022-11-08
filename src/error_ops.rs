use crate::backend_flags::{
    IM_GUI_BACKEND_FLAGS_HAS_MOUSE_CURSORS, IM_GUI_BACKEND_FLAGS_PLATFORM_HAS_VIEWPORTS,
    IM_GUI_BACKEND_FLAGS_RENDERER_HAS_VIEWPORTS,
};
use crate::child_ops::EndChild;
use crate::core::config_flags::{
    ImGuiConfigFlags_DockingEnable, ImGuiConfigFlags_NavEnableKeyboard,
    ImGuiConfigFlags_ViewportsEnable,
};
use crate::group_ops::EndGroup;
use crate::id_ops::pop_win_id_from_stack;
use crate::item_ops::PopItemFlag;
use crate::key::{ImGuiKey_COUNT, ImGuiKey_NamedKey_BEGIN};
use crate::keyboard_ops::GetMergedModFlags;
use crate::platform_monitor::ImGuiPlatformMonitor;
use crate::style_ops::PopStyleColor;
use crate::type_defs::ImGuiErrorLogCallback;
use crate::utils::{flag_clear, flag_set, is_not_null};
use crate::window::focus::PopFocusScope;
use crate::window::ops::{End, EndDisabled};
use crate::window::window_flags::ImGuiWindowFlags_ChildWindow;
use crate::GImGui;
use libc::c_void;
use std::ptr::null_mut;

pub unsafe fn ErrorCheckNewFrameSanityChecks() {
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
    // IM_ASSERT((g.IO.DeltaTime > 0.0 || g.FrameCount == 0)              && "Need a positive DeltaTime!");
    // IM_ASSERT((g.FrameCount == 0 || g.FrameCountEnded == g.FrameCount)  && "Forgot to call Render() or EndFrame() at the end of the previous frame?");
    // IM_ASSERT(g.IO.DisplaySize.x >= 0.0 && g.IO.DisplaySize.y >= 0.0  && "Invalid DisplaySize value!");
    // IM_ASSERT(g.IO.Fonts.IsBuilt()                                     && "Font Atlas not built! Make sure you called ImGui_ImplXXXX_NewFrame() function for renderer backend, which should call io.Fonts.GetTexDataAsRGBA32() / GetTexDataAsAlpha8()");
    // IM_ASSERT(g.style.CurveTessellationTol > 0.0                       && "Invalid style setting!");
    // IM_ASSERT(g.style.CircleTessellationMaxError > 0.0                 && "Invalid style setting!");
    // IM_ASSERT(g.style.Alpha >= 0.0 && g.style.Alpha <= 1.0            && "Invalid style setting!"); // Allows us to avoid a few clamps in color computations
    // IM_ASSERT(g.style.WindowMinSize.x >= 1.0 && g.style.WindowMinSize.y >= 1.0 && "Invalid style setting.");
    // IM_ASSERT(g.style.WindowMenuButtonPosition == ImGuiDir_None || g.style.WindowMenuButtonPosition == ImGuiDir_Left || g.style.WindowMenuButtonPosition == ImGuiDir_Right);
    // IM_ASSERT(g.style.ColorButtonPosition == ImGuiDir_Left || g.style.ColorButtonPosition == ImGuiDir_Right);
    // #ifndef IMGUI_DISABLE_OBSOLETE_KEYIO
    //     for (let n: c_int = ImGuiKey_NamedKey_BEGIN; n < ImGuiKey_COUNT; n++)
    for n in ImGuiKey_NamedKey_BEGIN..ImGuiKey_COUNT {}
    // IM_ASSERT(g.IO.KeyMap[n] >= -1 && g.IO.KeyMap[n] < ImGuiKey_LegacyNativeKey_END && "io.KeyMap[] contains an out of bound value (need to be 0..511, or -1 for unmapped key)");

    // Check: required key mapping (we intentionally do NOT check all keys to not pressure user into setting up everything, but Space is required and was only added in 1.60 WIP)
    if (flg_set(g.IO.ConfigFlags, ImGuiConfigFlags_NavEnableKeyboard)
        && g.IO.BackendUsingLegacyKeyArrays == 1)
    {}
    // IM_ASSERT(g.IO.KeyMap[ImGuiKey_Space] != -1 && "ImGuiKey_Space is not mapped, required for keyboard navigation.");
    // #endif

    // Check: the io.ConfigWindowsResizeFromEdges option requires backend to honor mouse cursor changes and set the IM_GUI_BACKEND_FLAGS_HAS_MOUSE_CURSORS flag accordingly.
    if (g.IO.ConfigWindowsResizeFromEdges
        && flag_clear(g.IO.BackendFlags, IM_GUI_BACKEND_FLAGS_HAS_MOUSE_CURSORS))
    {
        g.IO.ConfigWindowsResizeFromEdges = false;
    }

    // Perform simple check: error if Docking or Viewport are enabled _exactly_ on frame 1 (instead of frame 0 or later), which is a common error leading to loss of .ini data.
    if (g.FrameCount == 1
        && flag_set(g.IO.ConfigFlags, ImGuiConfigFlags_DockingEnable)
        && (g.ConfigFlagsLastFrame & ImGuiConfigFlags_DockingEnable) == 0)
    {}
    // IM_ASSERT(0 && "Please set DockingEnable before the first call to NewFrame()! Otherwise you will lose your .ini settings!");
    if (g.FrameCount == 1
        && flag_set(g.IO.ConfigFlags, ImGuiConfigFlags_ViewportsEnable)
        && (g.ConfigFlagsLastFrame & ImGuiConfigFlags_ViewportsEnable) == 0)
    {}
    // IM_ASSERT(0 && "Please set ViewportsEnable before the first call to NewFrame()! Otherwise you will lose your .ini settings!");

    // Perform simple checks: multi-viewport and platform windows support
    if (g.IO.ConfigFlags & ImGuiConfigFlags_ViewportsEnable) {
        if ((g.IO.BackendFlags & IM_GUI_BACKEND_FLAGS_PLATFORM_HAS_VIEWPORTS)
            && (g.IO.BackendFlags & IM_GUI_BACKEND_FLAGS_RENDERER_HAS_VIEWPORTS))
        {
            // IM_ASSERT((g.FrameCount == 0 || g.FrameCount == g.FrameCountPlatformEnded) && "Forgot to call UpdatePlatformWindows() in main loop after EndFrame()? Check examples/ applications for reference.");
            // IM_ASSERT(g.PlatformIO.Platform_CreateWindow  != NULL && "Platform init didn't install handlers?");
            // IM_ASSERT(g.PlatformIO.Platform_DestroyWindow != NULL && "Platform init didn't install handlers?");
            // IM_ASSERT(g.PlatformIO.Platform_GetWindowPos  != NULL && "Platform init didn't install handlers?");
            // IM_ASSERT(g.PlatformIO.Platform_SetWindowPos  != NULL && "Platform init didn't install handlers?");
            // IM_ASSERT(g.PlatformIO.Platform_GetWindowSize != NULL && "Platform init didn't install handlers?");
            // IM_ASSERT(g.PlatformIO.Platform_SetWindowSize != NULL && "Platform init didn't install handlers?");
            // IM_ASSERT(g.PlatformIO.Monitors.Size > 0 && "Platform init didn't setup Monitors list?");
            // IM_ASSERT((g.Viewports[0]->PlatformUserData != NULL || g.Viewports[0]->PlatformHandle != NULL) && "Platform init didn't setup main viewport.");
            if (g.IO.ConfigDockingTransparentPayload
                && flag_set(g.IO.ConfigFlags, ImGuiConfigFlags_DockingEnable))
            {}
            // IM_ASSERT(g.PlatformIO.Platform_SetWindowAlpha != NULL && "Platform_SetWindowAlpha handler is required to use io.ConfigDockingTransparent!");
        } else {
            // Disable feature, our backends do not support it
            g.IO.ConfigFlags &= !ImGuiConfigFlags_ViewportsEnable;
        }

        // Perform simple checks on platform monitor data + compute a total bounding box for quick early outs
        // for (let monitor_n: c_int = 0; monitor_n < g.PlatformIO.Monitors.Size; monitor_n++)
        for monitor_n in 0..g.PlatformIO.Monitors.len() {
            let mon = &g.PlatformIO.Monitors[monitor_n];
            // IM_UNUSED(mon);
            // IM_ASSERT(mon.MainSize.x > 0.0 && mon.MainSize.y > 0.0 && "Monitor main bounds not setup properly.");
            // IM_ASSERT(ImRect(mon.MainPos, mon.MainPos + mon.MainSize).Contains(ImRect(mon.WorkPos, mon.WorkPos + mon.WorkSize)) && "Monitor work bounds not setup properly. If you don't have work area information, just copy MainPos/MainSize into them.");
            // IM_ASSERT(mon.DpiScale != 0.0);
        }
    }
}

pub unsafe fn ErrorCheckEndFrameSanityChecks() {
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
    if g.CurrentWindowStack.Size != 1 {
        if g.CurrentWindowStack.Size > 1 {
            // IM_ASSERT_USER_ERROR(g.CurrentWindowStack.Size == 1, "Mismatched Begin/BeginChild vs End/EndChild calls: did you forget to call End/EndChild?");
            while g.CurrentWindowStack.Size > 1 {
                End();
            }
        } else {
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
pub unsafe fn ErrorCheckEndFrameRecover(
    log_callback: ImGuiErrorLogCallback,
    user_data: *mut c_void,
) {
    // PVS-Studio V1044 is "Loop break conditions do not depend on the number of iterations"
    let g = GImGui; // ImGuiContext& g = *GImGui;
    while g.CurrentWindowStack.Size > 0
    //-V1044
    {
        ErrorCheckEndWindowRecover(log_callback, user_data);
        let mut window  = g.current_window_mut().unwrap();
        if g.CurrentWindowStack.Size == 1 {
            // IM_ASSERT(window.IsFallbackWindow);
            break;
        }
        if window.Flags & ImGuiWindowFlags_ChildWindow {
            // if (log_callback) log_callback(user_data, "Recovered from missing EndChild() for '{}'", window.Name);
            // EndChild();
        } else {
            // if (log_callback) log_callback(user_data, "Recovered from missing End() for '{}'", window.Name);
            // End();
        }
    }
}

// Must be called before End()/EndChild()
pub unsafe fn ErrorCheckEndWindowRecover(
    log_callback: ImGuiErrorLogCallback,
    user_data: *mut c_void,
) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    while is_not_null(g.CurrentTable)
        && (g.Currenttable.OuterWindow == g.CurrentWindow
            || g.Currenttable.InnerWindow == g.CurrentWindow)
    {
        // if log_callback { log_callback(user_data, "Recovered from missing EndTable() in '{}'", g.Currenttable.Outerwindow.Name); }
        EndTable();
    }

    let mut window  = g.current_window_mut().unwrap();
    let stack_sizes = g.current_window_mut().unwrap()Stack.last().unwrap().StackSizesOnBegin;
    // IM_ASSERT(window != NULL);
    while (g.CurrentTabBar != null_mut())
    //-V1044
    {
        // if (log_callback) { log_callback(user_data, "Recovered from missing EndTabBar() in '{}'", window.Name); }
        EndTabBar();
    }
    while (window.dc.TreeDepth > 0) {
        // if (log_callback) { log_callback(user_data, "Recovered from missing TreePop() in '{}'", window.Name); }
        TreePop();
    }
    while g.GroupStack.Size > stack_sizes.SizeOfGroupStack
    //-V1044
    {
        // if (log_callback) { log_callback(user_data, "Recovered from missing EndGroup() in '{}'", window.Name); }
        EndGroup();
    }
    while window.id_stack.Size > 1 {
        // if (log_callback) { log_callback(user_data, "Recovered from missing PopID() in '{}'", window.Name); }
        pop_win_id_from_stack(g);
    }
    while g.DisabledStackSize > stack_sizes.SizeOfDisabledStack
    //-V1044
    {
        // if (log_callback) { log_callback(user_data, "Recovered from missing EndDisabled() in '{}'", window.Name); }
        EndDisabled();
    }
    while g.ColorStack.Size > stack_sizes.SizeOfColorStack {
        // if (log_callback) { log_callback(user_data, "Recovered from missing PopStyleColor() in '{}' for ImGuiCol_{}", window.Name, GetStyleColorName(g.ColorStack.last().unwrap().Col)); }
        PopStyleColor(0);
    }
    while g.ItemFlagsStack.Size > stack_sizes.SizeOfItemFlagsStack
    //-V1044
    {
        // if (log_callback) log_callback(user_data, "Recovered from missing PopItemFlag() in '{}'", window.Name);
        PopItemFlag();
    }
    while g.styleVarStack.Size > stack_sizes.SizeOfStyleVarStack
    //-V1044
    {
        // if (log_callback) log_callback(user_data, "Recovered from missing PopStyleVar() in '{}'", window.Name);
        PopStyleVar();
    }
    while (g.FocusScopeStack.Size > stack_sizes.SizeOfFocusScopeStack)
    //-V1044
    {
        // if (log_callback) log_callback(user_data, "Recovered from missing PopFocusScope() in '{}'", window.Name);
        PopFocusScope();
    }
}
