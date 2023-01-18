// dear imgui: Platform Backend for SDL2
#![allow(non_snake_case)]
// This needs to be used along with a Renderer (e.g. DirectX11, OpenGL3, Vulkan..)
// (Info: SDL2 is a cross-platform general purpose library for handling windows, inputs, graphics context creation, etc.)
// (Prefer SDL 2.0.5+ for full feature support.)

// Implemented features:
//  [X] Platform: Clipboard support.
//  [X] Platform: Keyboard support. Since 1.87 we are using the io.AddKeyEvent() function. Pass ImGuiKey values to all key functions e.g. IsKeyPressed(ImGuiKey_Space). [Legacy SDL_SCANCODE_* values will also be supported unless IMGUI_DISABLE_OBSOLETE_KEYIO is set]
//  [X] Platform: Gamepad support. Enabled with 'io.ConfigFlags |= ImGuiConfigFlags_NavEnableGamepad'.
//  [X] Platform: Mouse cursor shape and visibility. Disable with 'io.ConfigFlags |= ImGuiConfigFlags_NoMouseCursorChange'.
//  [X] Platform: Multi-viewport support (multiple windows). Enable with 'io.ConfigFlags |= ImGuiConfigFlags_ViewportsEnable'.
// Missing features:
//  [ ] Platform: SDL2 handling of IME under Windows appears to be broken and it explicitly disable the regular Windows IME. You can restore Windows IME by compiling SDL with SDL_DISABLE_WINDOWS_IME.
//  [ ] Platform: Multi-viewport + Minimized windows seems to break mouse wheel events (at least under Windows).

// You can use unmodified imgui_impl_* files in your project. See examples/ folder for examples of using this.
// Prefer including the entire imgui/ repository into your project (either as a copy or as a submodule), and only build the backends you need.
// If you are new to Dear ImGui, read documentation from the docs/ folder + read the top of imgui.cpp.
// Read online: https://github.com/ocornut/imgui/tree/master/docs

// CHANGELOG
// (minor and older changes stripped away, please see git history for details)
//  2022-XX-XX: Platform: Added support for multiple windows via the ImGuiPlatformIO interface.
//  2022-03-22: Inputs: Fix mouse position issues when dragging outside of boundaries. SDL_CaptureMouse() erroneously still gives out LEAVE events when hovering OS decorations.
//  2022-03-22: Inputs: Added support for extra mouse buttons (SDL_BUTTON_X1/SDL_BUTTON_X2).
//  2022-02-04: Added SDL_Renderer* parameter to ImGui_ImplSDL2_InitForSDLRenderer(), so we can use SDL_GetRendererOutputSize() instead of SDL_GL_GetDrawableSize() when bound to a SDL_Renderer.
//  2022-01-26: Inputs: replaced short-lived io.AddKeyModsEvent() (added two weeks ago) with io.AddKeyEvent() using ImGuiKey_ModXXX flags. Sorry for the confusion.
//  2021-01-20: Inputs: calling new io.AddKeyAnalogEvent() for gamepad support, instead of writing directly to io.NavInputs[].
//  2022-01-17: Inputs: calling new io.AddMousePosEvent(), io.AddMouseButtonEvent(), io.AddMouseWheelEvent() API (1.87+).
//  2022-01-17: Inputs: always update key mods next and before key event (not in NewFrame) to fix input queue with very low framerates.
//  2022-01-12: Update mouse inputs using SDL_MOUSEMOTION/SDL_WINDOWEVENT_LEAVE + fallback to provide it when focused but not hovered/captured. More standard and will allow us to pass it to future input queue API.
//  2022-01-12: Maintain our own copy of MouseButtonsDown mask instead of using IsAnyMouseDown() which will be obsoleted.
//  2022-01-10: Inputs: calling new io.AddKeyEvent(), io.AddKeyModsEvent() + io.SetKeyEventNativeData() API (1.87+). Support for full ImGuiKey range.
//  2021-08-17: Calling io.AddFocusEvent() on SDL_WINDOWEVENT_FOCUS_GAINED/SDL_WINDOWEVENT_FOCUS_LOST.
//  2021-07-29: Inputs: MousePos is correctly reported when the host platform window is hovered but not focused (using SDL_GetMouseFocus() + SDL_HINT_MOUSE_FOCUS_CLICKTHROUGH, requires SDL 2.0.5+)
//  2021-06:29: *BREAKING CHANGE* Removed 'window: *mut SDL_Window' parameter to ImGui_ImplSDL2_NewFrame() which was unnecessary.
//  2021-06-29: Reorganized backend to pull data from a single structure to facilitate usage with multiple-contexts (all g_XXXX access changed to bd.XXXX).
//  2021-03-22: Rework global mouse pos availability check listing supported platforms explicitly, effectively fixing mouse access on Raspberry Pi. (#2837, #3950)
//  2020-05-25: Misc: Report a zero display-size when window is minimized, to be consistent with other backends.
//  2020-02-20: Inputs: Fixed mapping for ImGuiKey_KeyPadEnter (using SDL_SCANCODE_KP_ENTER instead of SDL_SCANCODE_RETURN2).
//  2019-12-17: Inputs: On Wayland, use SDL_GetMouseState (because there is no global mouse state).
//  2019-12-05: Inputs: Added support for ImGuiMouseCursor_NotAllowed mouse cursor.
//  2019-07-21: Inputs: Added mapping for ImGuiKey_KeyPadEnter.
//  2019-04-23: Inputs: Added support for SDL_GameController (if ImGuiConfigFlags_NavEnableGamepad is set by user application).
//  2019-03-12: Misc: Preserve DisplayFramebufferScale when main window is minimized.
//  2018-12-21: Inputs: Workaround for Android/iOS which don't seem to handle focus related calls.
//  2018-11-30: Misc: Setting up io.BackendPlatformName so it can be displayed in the About Window.
//  2018-11-14: Changed the signature of ImGui_ImplSDL2_ProcessEvent() to take a 'const SDL_Event*'.
//  2018-08-01: Inputs: Workaround for Emscripten which doesn't seem to handle focus related calls.
//  2018-06-29: Inputs: Added support for the ImGuiMouseCursor_Hand cursor.
//  2018-06-08: Misc: Extracted imgui_impl_sdl.cpp/.h away from the old combined SDL2+OpenGL/Vulkan examples.
//  2018-06-08: Misc: ImGui_ImplSDL2_InitForOpenGL() now takes a SDL_GLContext parameter.
//  2018-05-09: Misc: Fixed clipboard paste memory leak (we didn't call SDL_FreeMemory on the data returned by SDL_GetClipboardText).
//  2018-03-20: Misc: Setup io.BackendFlags IM_GUI_BACKEND_FLAGS_HAS_MOUSE_CURSORS flag + honor ImGuiConfigFlags_NoMouseCursorChange flag.
//  2018-02-16: Inputs: Added support for mouse cursors, honoring GetMouseCursor() value.
//  2018-02-06: Misc: Removed call to shutdown() which is not available from 1.60 WIP, user needs to call CreateContext/DestroyContext themselves.
//  2018-02-06: Inputs: Added mapping for ImGuiKey_Space.
//  2018-02-05: Misc: Using SDL_GetPerformanceCounter() instead of SDL_GetTicks() to be able to handle very high framerate (1000+ FPS).
//  2018-02-05: Inputs: Keyboard mapping is using scancodes everywhere instead of a confusing mixture of keycodes and scancodes.
//  2018-01-20: Inputs: Added Horizontal Mouse Wheel support.
//  2018-01-19: Inputs: When available (SDL 2.0.4+) using SDL_CaptureMouse() to retrieve coordinates outside of client area when dragging. Otherwise (SDL 2.0.3 and before) testing for SDL_WINDOW_INPUT_FOCUS instead of SDL_WINDOW_MOUSE_FOCUS.
//  2018-01-18: Inputs: Added mapping for ImGuiKey_Insert.
//  2017-08-25: Inputs: MousePos set to -FLT_MAX,-FLT_MAX when mouse is unavailable/missing (instead of -1,-1).
//  2016-10-15: Misc: Added a void* user_data parameter to Clipboard function handlers.

// #include "imgui.h"
// #include "imgui_impl_sdl.h"

// SDL
// (the multi-viewports feature requires SDL features supported from SDL 2.0.4+. SDL 2.0.5+ is highly recommended)
// #include <SDL.h>
// #include <SDL_syswm.h>
// #if defined(__APPLE__)
// #include <TargetConditionals.h>
// #endif

use std::ffi::CString;
use std::mem;
use std::os::raw::c_void;
use std::ptr::null_mut;
use libc::{c_char, c_int};
use sdl2::sys::{SDL_BUTTON_LEFT, SDL_BUTTON_MIDDLE, SDL_BUTTON_RIGHT, SDL_BUTTON_X1, SDL_BUTTON_X2, SDL_CaptureMouse, SDL_CreateSystemCursor, SDL_CreateWindow, SDL_Cursor, SDL_DestroyWindow, SDL_Event, SDL_free, SDL_FreeCursor, SDL_GameController, SDL_GameControllerButton, SDL_GameControllerGetAxis, SDL_GameControllerGetButton, SDL_GameControllerOpen, SDL_GetClipboardText, SDL_GetCurrentVideoDriver, SDL_GetDisplayBounds, SDL_GetDisplayDPI, SDL_GetDisplayUsableBounds, SDL_GetGlobalMouseState, SDL_GetKeyboardFocus, SDL_GetNumVideoDisplays, SDL_GetPerformanceCounter, SDL_GetPerformanceFrequency, SDL_GetRendererOutputSize, SDL_GetWindowFlags, SDL_GetWindowFromID, SDL_GetWindowID, SDL_GetWindowPosition, SDL_GetWindowSize, SDL_GetWindowWMInfo, SDL_GL_CreateContext, SDL_GL_DeleteContext, SDL_GL_GetCurrentContext, SDL_GL_GetDrawableSize, SDL_GL_MakeCurrent, SDL_GL_SetAttribute, SDL_GL_SetSwapInterval, SDL_GL_SwapWindow, SDL_GLContext, SDL_HINT_MOUSE_FOCUS_CLICKTHROUGH, SDL_Keymod, SDL_RaiseWindow, SDL_Rect, SDL_Renderer, SDL_SetClipboardText, SDL_SetCursor, SDL_SetHint, SDL_SetWindowOpacity, SDL_SetWindowPosition, SDL_SetWindowSize, SDL_SetWindowTitle, SDL_ShowCursor, SDL_ShowWindow, SDL_SYSWM_TYPE, SDL_SysWMinfo, SDL_SysWMinfo__bindgen_ty_1, SDL_version, SDL_Vulkan_CreateSurface, SDL_WarpMouseGlobal, SDL_WarpMouseInWindow, SDL_Window, VkInstance};
use sdl2::sys::SDL_bool::{SDL_FALSE, SDL_TRUE};
use sdl2::sys::SDL_EventType::{SDL_KEYDOWN, SDL_MOUSEBUTTONDOWN, SDL_MOUSEWHEEL, SDL_TEXTINPUT};
use sdl2::sys::SDL_GameControllerAxis::{SDL_CONTROLLER_AXIS_LEFTX, SDL_CONTROLLER_AXIS_LEFTY, SDL_CONTROLLER_AXIS_RIGHTX, SDL_CONTROLLER_AXIS_RIGHTY, SDL_CONTROLLER_AXIS_TRIGGERLEFT, SDL_CONTROLLER_AXIS_TRIGGERRIGHT};
use sdl2::sys::SDL_GameControllerButton::{SDL_CONTROLLER_BUTTON_A, SDL_CONTROLLER_BUTTON_B, SDL_CONTROLLER_BUTTON_BACK, SDL_CONTROLLER_BUTTON_DPAD_DOWN, SDL_CONTROLLER_BUTTON_DPAD_LEFT, SDL_CONTROLLER_BUTTON_DPAD_RIGHT, SDL_CONTROLLER_BUTTON_DPAD_UP, SDL_CONTROLLER_BUTTON_LEFTSHOULDER, SDL_CONTROLLER_BUTTON_LEFTSTICK, SDL_CONTROLLER_BUTTON_RIGHTSHOULDER, SDL_CONTROLLER_BUTTON_RIGHTSTICK, SDL_CONTROLLER_BUTTON_START, SDL_CONTROLLER_BUTTON_X, SDL_CONTROLLER_BUTTON_Y};
use sdl2::sys::SDL_GLattr::SDL_GL_SHARE_WITH_CURRENT_CONTEXT;
use sdl2::sys::SDL_Keymod::{KMOD_ALT, KMOD_CTRL, KMOD_GUI, KMOD_SHIFT};
use sdl2::sys::SDL_SystemCursor::{SDL_SYSTEM_CURSOR_ARROW, SDL_SYSTEM_CURSOR_HAND, SDL_SYSTEM_CURSOR_IBEAM, SDL_SYSTEM_CURSOR_NO, SDL_SYSTEM_CURSOR_SIZEALL, SDL_SYSTEM_CURSOR_SIZENESW, SDL_SYSTEM_CURSOR_SIZENS, SDL_SYSTEM_CURSOR_SIZENWSE, SDL_SYSTEM_CURSOR_SIZEWE};
use sdl2::sys::SDL_SYSWM_TYPE::SDL_SYSWM_UNKNOWN;
use sdl2::sys::SDL_WindowEventID::{SDL_WINDOWEVENT_CLOSE, SDL_WINDOWEVENT_ENTER, SDL_WINDOWEVENT_FOCUS_GAINED, SDL_WINDOWEVENT_FOCUS_LOST, SDL_WINDOWEVENT_LEAVE, SDL_WINDOWEVENT_MOVED, SDL_WINDOWEVENT_RESIZED};
use sdl2::sys::SDL_WindowFlags::{SDL_WINDOW_ALLOW_HIGHDPI, SDL_WINDOW_ALWAYS_ON_TOP, SDL_WINDOW_BORDERLESS, SDL_WINDOW_HIDDEN, SDL_WINDOW_INPUT_FOCUS, SDL_WINDOW_MINIMIZED, SDL_WINDOW_OPENGL, SDL_WINDOW_RESIZABLE, SDL_WINDOW_SKIP_TASKBAR, SDL_WINDOW_VULKAN};
use windows::Win32::Foundation::HWND;
use crate::core::config_flags::{ImGuiConfigFlags_NavEnableGamepad, ImGuiConfigFlags_NoMouseCursorChange, ImGuiConfigFlags_ViewportsEnable};
use crate::core::context::AppContext;
use crate::core::math_ops::ImSaturateFloat;
use crate::io::input_ops::GetMouseCursor;
use crate::io::io_ops::GetIO;
use crate::io::IoContext;
use crate::io::key::{ImGuiKey, ImGuiKey_0, ImGuiKey_1, ImGuiKey_2, ImGuiKey_3, ImGuiKey_4, ImGuiKey_5, ImGuiKey_6, ImGuiKey_7, ImGuiKey_8, ImGuiKey_9, ImGuiKey_A, ImGuiKey_Apostrophe, ImGuiKey_B, ImGuiKey_Backslash, ImGuiKey_Backspace, ImGuiKey_C, ImGuiKey_CapsLock, ImGuiKey_Comma, ImGuiKey_D, ImGuiKey_Delete, ImGuiKey_DownArrow, ImGuiKey_E, ImGuiKey_End, ImGuiKey_Enter, ImGuiKey_Equal, ImGuiKey_Escape, ImGuiKey_F, ImGuiKey_F1, ImGuiKey_F10, ImGuiKey_F11, ImGuiKey_F12, ImGuiKey_F2, ImGuiKey_F3, ImGuiKey_F4, ImGuiKey_F5, ImGuiKey_F6, ImGuiKey_F7, ImGuiKey_F8, ImGuiKey_F9, ImGuiKey_G, ImGuiKey_GamepadBack, ImGuiKey_GamepadDpadDown, ImGuiKey_GamepadDpadLeft, ImGuiKey_GamepadDpadRight, ImGuiKey_GamepadDpadUp, ImGuiKey_GamepadFaceDown, ImGuiKey_GamepadFaceLeft, ImGuiKey_GamepadFaceRight, ImGuiKey_GamepadFaceUp, ImGuiKey_GamepadL1, ImGuiKey_GamepadL2, ImGuiKey_GamepadL3, ImGuiKey_GamepadLStickDown, ImGuiKey_GamepadLStickLeft, ImGuiKey_GamepadLStickRight, ImGuiKey_GamepadLStickUp, ImGuiKey_GamepadR1, ImGuiKey_GamepadR2, ImGuiKey_GamepadR3, ImGuiKey_GamepadRStickDown, ImGuiKey_GamepadRStickLeft, ImGuiKey_GamepadRStickRight, ImGuiKey_GamepadRStickUp, ImGuiKey_GamepadStart, ImGuiKey_GraveAccent, ImGuiKey_H, ImGuiKey_Home, ImGuiKey_I, ImGuiKey_Insert, ImGuiKey_J, ImGuiKey_K, ImGuiKey_Keypad0, ImGuiKey_Keypad1, ImGuiKey_Keypad2, ImGuiKey_Keypad3, ImGuiKey_Keypad4, ImGuiKey_Keypad5, ImGuiKey_Keypad6, ImGuiKey_Keypad7, ImGuiKey_Keypad8, ImGuiKey_Keypad9, ImGuiKey_KeypadAdd, ImGuiKey_KeypadDecimal, ImGuiKey_KeypadDivide, ImGuiKey_KeypadEnter, ImGuiKey_KeypadEqual, ImGuiKey_KeypadMultiply, ImGuiKey_KeypadSubtract, ImGuiKey_L, ImGuiKey_LeftAlt, ImGuiKey_LeftArrow, ImGuiKey_LeftBracket, ImGuiKey_LeftCtrl, ImGuiKey_LeftShift, ImGuiKey_LeftSuper, ImGuiKey_M, ImGuiKey_Menu, ImGuiKey_Minus, ImGuiKey_ModAlt, ImGuiKey_ModCtrl, ImGuiKey_ModShift, ImGuiKey_ModSuper, ImGuiKey_N, ImGuiKey_None, ImGuiKey_NumLock, ImGuiKey_O, ImGuiKey_P, ImGuiKey_PageDown, ImGuiKey_PageUp, ImGuiKey_Pause, ImGuiKey_Period, ImGuiKey_PrintScreen, ImGuiKey_Q, ImGuiKey_R, ImGuiKey_RightAlt, ImGuiKey_RightArrow, ImGuiKey_RightBracket, ImGuiKey_RightCtrl, ImGuiKey_RightShift, ImGuiKey_RightSuper, ImGuiKey_S, ImGuiKey_ScrollLock, ImGuiKey_Semicolon, ImGuiKey_Slash, ImGuiKey_Space, ImGuiKey_T, ImGuiKey_Tab, ImGuiKey_U, ImGuiKey_UpArrow, ImGuiKey_V, ImGuiKey_W, ImGuiKey_X, ImGuiKey_Y, ImGuiKey_Z};
use crate::io::mouse_cursor::{ImGuiMouseCursor, ImGuiMouseCursor_Arrow, ImGuiMouseCursor_COUNT, ImGuiMouseCursor_Hand, ImGuiMouseCursor_None, ImGuiMouseCursor_NotAllowed, ImGuiMouseCursor_ResizeAll, ImGuiMouseCursor_ResizeEW, ImGuiMouseCursor_ResizeNESW, ImGuiMouseCursor_ResizeNS, ImGuiMouseCursor_ResizeNWSE, ImGuiMouseCursor_TextInput};
use crate::platform::platform_monitor::PlatformMonitor;
use crate::viewport::viewport_flags::{ImGuiViewportFlags_NoDecoration, ImGuiViewportFlags_NoFocusOnAppearing, ImGuiViewportFlags_NoTaskBarIcon, ImGuiViewportFlags_TopMost};
use crate::viewport::viewport_ops::{DestroyPlatformWindows, FindViewportByPlatformHandle, GetMainViewport};

// #if SDL_VERSION_ATLEAST(2,0,4) && !defined(__EMSCRIPTEN__) && !defined(__ANDROID__) && !(defined(__APPLE__) && TARGET_OS_IOS) && !defined(__amigaos4__)
// #define SDL_HAS_CAPTURE_AND_GLOBAL_MOUSE    1
// #else
// #define SDL_HAS_CAPTURE_AND_GLOBAL_MOUSE    0
// #endif
// TODO: determine using compile flags like above
pub const SDL_HAS_CAPTURE_AND_GLOBAL_MOUSE: u32 = 1;

// #define SDL_HAS_MOUSE_FOCUS_CLICKTHROUGH    SDL_VERSION_ATLEAST(2,0,5)
pub const SDL_HAS_MOUSE_FOCUS_CLICKTHROUGH: u32 = 1;
// #define SDL_HAS_WINDOW_ALPHA                SDL_VERSION_ATLEAST(2,0,5)
pub const SDL_HAS_WINDOW_ALPHA: u32 = 1;
// #define SDL_HAS_ALWAYS_ON_TOP               SDL_VERSION_ATLEAST(2,0,5)
pub const SDL_HAS_ALWAYS_ON_TOP: u32 = 1;
// #define SDL_HAS_USABLE_DISPLAY_BOUNDS       SDL_VERSION_ATLEAST(2,0,5)
pub const SDL_HAS_USABLE_DISPLAY_BOUNDS: u32 = 1;
// #define SDL_HAS_PER_MONITOR_DPI             SDL_VERSION_ATLEAST(2,0,4)
pub const SDL_HAS_PER_MONITOR_DPI: u32 = 1;
// #define SDL_HAS_VULKAN                      SDL_VERSION_ATLEAST(2,0,6)
pub const SDL_HAS_VULKAN: u32 = 1;

// #if !SDL_HAS_VULKAN
// static const Uint32 SDL_WINDOW_VULKAN = 0x10000000;
// #endif

// SDL Data
pub struct ImGui_ImplSDL2_Data
{
    // SDL_Window*     Window;
    pub Window: *mut SDL_Window,
    // SDL_Renderer*   Renderer;
    pub Renderer: *mut SDL_Renderer,
    // Uint64          Time;
    pub Time: u64,
    // Uint32          MouseWindowID;
    pub MouseWindowID: u32,
    // int             MouseButtonsDown;
    pub MouseButtonsDown: i32,
    // SDL_Cursor*     MouseCursors[ImGuiMouseCursor_COUNT];
    pub MouseCursors: [*mut SDL_Cursor;ImGuiMouseCursor_COUNT as usize],
    // int             PendingMouseLeaveFrame;
    pub PendingMouseLeaveFrame: i32,
    // char*           ClipboardTextData;
    pub ClipboardTextData: *mut c_char,
    // bool            MouseCanUseGlobalState;
    pub MouseCanUseGlobalState: bool,
    // bool            UseVulkan;
    pub UseVulkan: bool,

    // ImGui_ImplSDL2_Data()   { memset((void*)this, 0, sizeof(*this)); }
}

// Backend data stored in io.BackendPlatformUserData to allow support for multiple Dear ImGui contexts
// It is STRONGLY preferred that you use docking branch with multi-viewports (== single Dear ImGui context + multiple windows) instead of multiple Dear ImGui contexts.
// FIXME: multi-context support is not well tested and probably dysfunctional in this backend.
// FIXME: some shared resources (mouse cursor shape, gamepad) are mishandled when using multi-context.
pub fn ImGui_ImplSDL2_GetBackendData() -> *mut ImGui_ImplSDL2_Data
{
    // return GetCurrentContext() ? (ImGui_ImplSDL2_Data*)GetIO().BackendPlatformUserData : NULL;
    GetIO().backend_platform_user_data as *mut ImGui_ImplSDL2_Data
}

// Forward Declarations
// static void ImGui_ImplSDL2_UpdateMonitors();
// static void ImGui_ImplSDL2_InitPlatformInterface(window: *mut SDL_Window, void* sdl_gl_context);
// static void ImGui_ImplSDL2_ShutdownPlatformInterface();

// Functions
pub fn ImGui_ImplSDL2_GetClipboardText() -> *mut c_char
{
    // ImGui_ImplSDL2_Data* bd = ImGui_ImplSDL2_GetBackendData();
    let mut bd = ImGui_ImplSDL2_GetBackendData();
    if bd.ClipboardTextData.is_null() == false {
        unsafe { SDL_free(bd.ClipboardTextData as *mut c_void) }
    }
    // if (bd.ClipboardTextData)
    //     SDL_free(bd.ClipboardTextData);
    // bd.ClipboardTextData = SDL_GetClipboardText();
    unsafe { bd.ClipboardTextData = SDL_GetClipboardText(); }

    // return bd.ClipboardTextData;
    bd.ClipboardTextData
}

pub fn ImGui_ImplSDL2_SetClipboardText(text: *const c_char)
{
    unsafe { SDL_SetClipboardText(text); }
}

pub fn ImGui_ImplSDL2_KeycodeToImGuiKey(keycode: i32) -> ImGuiKey
{
    return match keycode {
        SDLK_TAB => { ImGuiKey_Tab },
        SDLK_LEFT => { ImGuiKey_LeftArrow },
        SDLK_RIGHT => { ImGuiKey_RightArrow },
        SDLK_UP => { ImGuiKey_UpArrow },
        SDLK_DOWN => { ImGuiKey_DownArrow },
        SDLK_PAGEUP => { ImGuiKey_PageUp },
        SDLK_PAGEDOWN => { ImGuiKey_PageDown },
        SDLK_HOME => { ImGuiKey_Home },
        SDLK_END => { ImGuiKey_End },
        SDLK_INSERT => { ImGuiKey_Insert },
        SDLK_DELETE => { ImGuiKey_Delete },
        SDLK_BACKSPACE => { ImGuiKey_Backspace },
        SDLK_SPACE => { ImGuiKey_Space },
        SDLK_RETURN => { ImGuiKey_Enter },
        SDLK_ESCAPE => { ImGuiKey_Escape },
        SDLK_QUOTE => { ImGuiKey_Apostrophe },
        SDLK_COMMA => { ImGuiKey_Comma },
        SDLK_MINUS => { ImGuiKey_Minus },
        SDLK_PERIOD => { ImGuiKey_Period },
        SDLK_SLASH => { ImGuiKey_Slash },
        SDLK_SEMICOLON => { ImGuiKey_Semicolon },
        SDLK_EQUALS => { ImGuiKey_Equal },
        SDLK_LEFTBRACKET => { ImGuiKey_LeftBracket },
        SDLK_BACKSLASH => { ImGuiKey_Backslash },
        SDLK_RIGHTBRACKET => { ImGuiKey_RightBracket },
        SDLK_BACKQUOTE => { ImGuiKey_GraveAccent },
        SDLK_CAPSLOCK => { ImGuiKey_CapsLock },
        SDLK_SCROLLLOCK => { ImGuiKey_ScrollLock },
        SDLK_NUMLOCKCLEAR => { ImGuiKey_NumLock },
        SDLK_PRINTSCREEN => { ImGuiKey_PrintScreen },
        SDLK_PAUSE => { ImGuiKey_Pause },
        SDLK_KP_0 => { ImGuiKey_Keypad0 },
        SDLK_KP_1 => { ImGuiKey_Keypad1 },
        SDLK_KP_2 => { ImGuiKey_Keypad2 },
        SDLK_KP_3 => { ImGuiKey_Keypad3 },
        SDLK_KP_4 => { ImGuiKey_Keypad4 },
        SDLK_KP_5 => { ImGuiKey_Keypad5 },
        SDLK_KP_6 => { ImGuiKey_Keypad6 },
        SDLK_KP_7 => { ImGuiKey_Keypad7 },
        SDLK_KP_8 => { ImGuiKey_Keypad8 },
        SDLK_KP_9 => { ImGuiKey_Keypad9 },
        SDLK_KP_PERIOD => { ImGuiKey_KeypadDecimal },
        SDLK_KP_DIVIDE => { ImGuiKey_KeypadDivide },
        SDLK_KP_MULTIPLY => { ImGuiKey_KeypadMultiply },
        SDLK_KP_MINUS => { ImGuiKey_KeypadSubtract },
        SDLK_KP_PLUS => { ImGuiKey_KeypadAdd },
        SDLK_KP_ENTER => { ImGuiKey_KeypadEnter },
        SDLK_KP_EQUALS => { ImGuiKey_KeypadEqual },
        SDLK_LCTRL => { ImGuiKey_LeftCtrl },
        SDLK_LSHIFT => { ImGuiKey_LeftShift },
        SDLK_LALT => { ImGuiKey_LeftAlt },
        SDLK_LGUI => { ImGuiKey_LeftSuper },
        SDLK_RCTRL => { ImGuiKey_RightCtrl },
        SDLK_RSHIFT => { ImGuiKey_RightShift },
        SDLK_RALT => { ImGuiKey_RightAlt },
        SDLK_RGUI => { ImGuiKey_RightSuper },
        SDLK_APPLICATION => { ImGuiKey_Menu },
        SDLK_0 => { ImGuiKey_0 },
        SDLK_1 => { ImGuiKey_1 },
        SDLK_2 => { ImGuiKey_2 },
        SDLK_3 => { ImGuiKey_3 },
        SDLK_4 => { ImGuiKey_4 },
        SDLK_5 => { ImGuiKey_5 },
        SDLK_6 => { ImGuiKey_6 },
        SDLK_7 => { ImGuiKey_7 },
        SDLK_8 => { ImGuiKey_8 },
        SDLK_9 => { ImGuiKey_9 },
        SDLK_a => { ImGuiKey_A },
        SDLK_b => { ImGuiKey_B },
        SDLK_c => { ImGuiKey_C },
        SDLK_d => { ImGuiKey_D },
        SDLK_e => { ImGuiKey_E },
        SDLK_f => { ImGuiKey_F },
        SDLK_g => { ImGuiKey_G },
        SDLK_h => { ImGuiKey_H },
        SDLK_i => { ImGuiKey_I },
        SDLK_j => { ImGuiKey_J },
        SDLK_k => { ImGuiKey_K },
        SDLK_l => { ImGuiKey_L },
        SDLK_m => { ImGuiKey_M },
        SDLK_n => { ImGuiKey_N },
        SDLK_o => { ImGuiKey_O },
        SDLK_p => { ImGuiKey_P },
        SDLK_q => { ImGuiKey_Q },
        SDLK_r => { ImGuiKey_R },
        SDLK_s => { ImGuiKey_S },
        SDLK_t => { ImGuiKey_T },
        SDLK_u => { ImGuiKey_U },
        SDLK_v => { ImGuiKey_V },
        SDLK_w => { ImGuiKey_W },
        SDLK_x => { ImGuiKey_X },
        SDLK_y => { ImGuiKey_Y },
        SDLK_z => { ImGuiKey_Z },
        SDLK_F1 => { ImGuiKey_F1 },
        SDLK_F2 => { ImGuiKey_F2 },
        SDLK_F3 => { ImGuiKey_F3 },
        SDLK_F4 => { ImGuiKey_F4 },
        SDLK_F5 => { ImGuiKey_F5 },
        SDLK_F6 => { ImGuiKey_F6 },
        SDLK_F7 => { ImGuiKey_F7 },
        SDLK_F8 => { ImGuiKey_F8 },
        SDLK_F9 => { ImGuiKey_F9 },
        SDLK_F10 => { ImGuiKey_F10 },
        SDLK_F11 => { ImGuiKey_F11 },
        SDLK_F12 => { ImGuiKey_F12 },
        _ => { ImGuiKey_None }
    }
}

pub fn ImGui_ImplSDL2_UpdateKeyModifiers(sdl_key_mods: SDL_Keymod)
{
    // ImGuiIO& io = GetIO();
    let io = GetIO();
    io.AddKeyEvent(ImGuiKey_ModCtrl, (sdl_key_mods & KMOD_CTRL) != 0);
    io.AddKeyEvent(ImGuiKey_ModShift, (sdl_key_mods & KMOD_SHIFT) != 0);
    io.AddKeyEvent(ImGuiKey_ModAlt, (sdl_key_mods & KMOD_ALT) != 0);
    io.AddKeyEvent(ImGuiKey_ModSuper, (sdl_key_mods & KMOD_GUI) != 0);
}

// You can read the io.WantCaptureMouse, io.WantCaptureKeyboard flags to tell if dear imgui wants to use your inputs.
// - When io.WantCaptureMouse is true, do not dispatch mouse input data to your main application, or clear/overwrite your copy of the mouse data.
// - When io.WantCaptureKeyboard is true, do not dispatch keyboard input data to your main application, or clear/overwrite your copy of the keyboard data.
// Generally you may always pass all inputs to dear imgui, and hide them from your application based on those two flags.
// If you have multiple SDL events and some of them are not meant to be used by dear imgui, you may need to filter events based on their windowID field.
pub fn ImGui_ImplSDL2_ProcessEvent(event: *const SDL_Event) -> bool
{
    let io = GetIO();
    let bd = ImGui_ImplSDL2_GetBackendData();

    match event.type_
    {
         SDL_MOUSEMOTION =>
        {
            let mut mouse_pos = ImVec2::new(event.motion.x, event.motion.y);
            if io.ConfigFlags & ImGuiConfigFlags_ViewportsEnable
            {
                // int window_x, window_y;
                let mut window_x = 0i32;
                let mut window_y = 0i32;
                unsafe { SDL_GetWindowPosition(SDL_GetWindowFromID(event.motion.windowID), &mut window_x, &mut window_y); }
                mouse_pos.x += window_x;
                mouse_pos.y += window_y;
            }
            io.AddMousePosEvent(mouse_pos.x, mouse_pos.y);
            return true;
        },
        SDL_MOUSEWHEEL =>
        {
            let mut wheel_x = if (event.wheel.x > 0) { 1.0f32 } else { if (event.wheel.x < 0) { -1.0f32 }else { 0.0f32 } };
            let mut wheel_y = if (event.wheel.y > 0) { 1.0f32 } else { if (event.wheel.y < 0) { -1.0f32 } else { 0.0f32 } };
            io.AddMouseWheelEvent(wheel_x, wheel_y);
            return true;
        }
        SDL_MOUSEBUTTONDOWN |
        SDL_MOUSEBUTTONUP=>
        {
            let mut mouse_button = -1;
            if event.button.button == SDL_BUTTON_LEFT as u8 { mouse_button = 0; }
            if event.button.button == SDL_BUTTON_RIGHT as u8 { mouse_button = 1; }
            if event.button.button == SDL_BUTTON_MIDDLE as u8 { mouse_button = 2; }
            if event.button.button == SDL_BUTTON_X1 as u8 { mouse_button = 3; }
            if event.button.button == SDL_BUTTON_X2 as u8 { mouse_button = 4; }
            if mouse_button == -1 {
                
            } else {
                io.AddMouseButtonEvent(mouse_button, (event.type_ == SDL_MOUSEBUTTONDOWN as u32));
                bd.MouseButtonsDown = if event.type_ == SDL_MOUSEBUTTONDOWN as u32 { (bd.MouseButtonsDown | (1 << mouse_button)) } else { (bd.MouseButtonsDown & !(1 << mouse_button)) };
                return true;
            }
        }
        SDL_TEXTINPUT =>
        {
            unsafe { io.AddInputCharactersUTF8(event.text.text.as_ptr()); }
            return true;
        }
         SDL_KEYDOWN |
         SDL_KEYUP=>
        {
            ImGui_ImplSDL2_UpdateKeyModifiers(event.key.keysym.mod_ as SDL_Keymod);
            let key = ImGui_ImplSDL2_KeycodeToImGuiKey(event.key.keysym.sym);
            io.AddKeyEvent(key, (event.type_ == SDL_KEYDOWN as u32));
            io.SetKeyEventNativeData(key, event.key.keysym.sym, event.key.keysym.scancode as c_int, event.key.keysym.scancode as c_int); // To support legacy indexing (<1.87 user code). Legacy backend uses SDLK_*** as indices to IsKeyXXX() functions.
            return true;
        }
         SDL_WINDOWEVENT=>
        {
            // - When capturing mouse, SDL will send a bunch of conflicting LEAVE/ENTER event on every mouse move, but the final ENTER tends to be right.
            // - However we won't get a correct LEAVE event for a captured window.
            // - In some cases, when detaching a window from main viewport SDL may send SDL_WINDOWEVENT_ENTER one frame too late,
            //   causing SDL_WINDOWEVENT_LEAVE on previous frame to interrupt drag operation by clear mouse position. This is why
            //   we delay process the SDL_WINDOWEVENT_LEAVE events by one frame. See issue #5012 for details.
            let mut  window_event = event.window.event;
            if window_event == SDL_WINDOWEVENT_ENTER as u8
            {
                bd.MouseWindowID = event.window.windowID;
                bd.PendingMouseLeaveFrame = 0;
            }
            if window_event == SDL_WINDOWEVENT_LEAVE as u8 {
                bd.PendingMouseLeaveFrame = GetFrameCount() + 1;
            }
            if window_event == SDL_WINDOWEVENT_FOCUS_GAINED as u8 {
                io.AddFocusEvent(true);
            }
            else if window_event == SDL_WINDOWEVENT_FOCUS_LOST as u8 {
                io.AddFocusEvent(false);
            }
            unsafe {
                if window_event == SDL_WINDOWEVENT_CLOSE as u8 || window_event == SDL_WINDOWEVENT_MOVED as u8 || window_event == SDL_WINDOWEVENT_RESIZED as u8 {
                    let viewport = FindViewportByPlatformHandle(SDL_GetWindowFromID(event.window.windowID) as *mut c_void);
                    if viewport.is_null() == false {
                        if (window_event == SDL_WINDOWEVENT_CLOSE as u8) {
                            viewport.PlatformRequestClose = true;
                        }
                        if (window_event == SDL_WINDOWEVENT_MOVED as u8) {
                            viewport.PlatformRequestMove = true;
                        }
                        if (window_event == SDL_WINDOWEVENT_RESIZED as u8) {
                            viewport.PlatformRequestResize = true;
                        }
                        return true;
                    }
                }
            }
            return true;
        }
    }
    return false;
}

pub fn  ImGui_ImplSDL2_Init(window: *mut SDL_Window, renderer: *mut SDL_Renderer, sdl_gl_context: *mut c_void) -> bool
{
    let io= GetIO();
    // IM_ASSERT(io.BackendPlatformUserData == NULL && "Already initialized a platform backend!");

    // Check and store if we are on a SDL backend that supports global mouse position
    // ("wayland" and "rpi" don't support it, but we chose to use a white-list instead of a black-list)
    let mut mouse_can_use_global_state = false;

    let sdl_backend = unsafe { SDL_GetCurrentVideoDriver() };
    let sdl_backend_str = sdl_backend.to_string();
    let global_mouse_whitelist: [String;5] = [ String::from("windows"), String::from("cocoa"), String::from("x11"), String::from("DIVE"), String::from("VMAN") ];

    for x in global_mouse_whitelist.iter() {
        if sdl_backend_str == x {
            mouse_can_use_global_state = true;
        }
    }

    // Setup backend capabilities flags
    // ImGui_ImplSDL2_Data* bd = IM_NEW(ImGui_ImplSDL2_Data)();
    let bd: *mut ImGui_ImplSDL2_Data = unsafe { libc::malloc(std::mem::size_of::<ImGui_ImplSDL2_Data>()) } as *mut ImGui_ImplSDL2_Data;

    io.BackendPlatformUserData = bd;
    io.BackendPlatformName = "imgui_impl_sdl";
    io.BackendFlags |= ImGuiBackendFlags_HasMouseCursors;           // We can honor GetMouseCursor() values (optional)
    io.BackendFlags |= ImGuiBackendFlags_HasSetMousePos;            // We can honor io.WantSetMousePos requests (optional, rarely used)
    if (mouse_can_use_global_state) {
        io.BackendFlags |= ImGuiBackendFlags_PlatformHasViewports;
    } // We can create multi-viewports on the Platform side (optional)

    // SDL on Linux/OSX doesn't report events for unfocused windows (see https://github.com/ocornut/imgui/issues/4960)
// #ifndef __APPLE__
    if (mouse_can_use_global_state) {
        io.BackendFlags |= ImGuiBackendFlags_HasMouseHoveredViewport;
    }// We can call io.AddMouseViewportEvent() with correct data (optional)
// #endif

    bd.Window = window;
    bd.Renderer = renderer;
    bd.MouseCanUseGlobalState = mouse_can_use_global_state;

    io.SetClipboardTextFn = ImGui_ImplSDL2_SetClipboardText;
    io.GetClipboardTextFn = ImGui_ImplSDL2_GetClipboardText;
    io.ClipboardUserData = NULL;

    // Load mouse cursors
    unsafe { bd.MouseCursors[ImGuiMouseCursor_Arrow] = SDL_CreateSystemCursor(SDL_SYSTEM_CURSOR_ARROW); }
    unsafe { bd.MouseCursors[ImGuiMouseCursor_TextInput] = SDL_CreateSystemCursor(SDL_SYSTEM_CURSOR_IBEAM); }
    unsafe { bd.MouseCursors[ImGuiMouseCursor_ResizeAll] = SDL_CreateSystemCursor(SDL_SYSTEM_CURSOR_SIZEALL); }
    unsafe { bd.MouseCursors[ImGuiMouseCursor_ResizeNS] = SDL_CreateSystemCursor(SDL_SYSTEM_CURSOR_SIZENS); }
    unsafe { bd.MouseCursors[ImGuiMouseCursor_ResizeEW] = SDL_CreateSystemCursor(SDL_SYSTEM_CURSOR_SIZEWE); }
    unsafe { bd.MouseCursors[ImGuiMouseCursor_ResizeNESW] = SDL_CreateSystemCursor(SDL_SYSTEM_CURSOR_SIZENESW); }
    unsafe { bd.MouseCursors[ImGuiMouseCursor_ResizeNWSE] = SDL_CreateSystemCursor(SDL_SYSTEM_CURSOR_SIZENWSE); }
    unsafe { bd.MouseCursors[ImGuiMouseCursor_Hand] = SDL_CreateSystemCursor(SDL_SYSTEM_CURSOR_HAND); }
    unsafe { bd.MouseCursors[ImGuiMouseCursor_NotAllowed] = SDL_CreateSystemCursor(SDL_SYSTEM_CURSOR_NO); }

    // Set platform dependent data in viewport
    // Our mouse update function expect PlatformHandle to be filled for the main viewport
    let main_viewport = unsafe { GetMainViewport() };
    main_viewport.PlatformHandle = ViewportPlatformHandle::VoidPointer(window);
    main_viewport.PlatformHandleRaw = ViewportPlatformHandle::Unknown;
    let mut info: SDL_SysWMinfo = SDL_SysWMInfo{
    version: SDL_version{
        major: 0,
        minor: 0,
        patch: 0,
    },
    subsystem: SDL_SYSWM_UNKNOWN,
};
    SDL_VERSION(&info.version);
    unsafe {
        if SDL_GetWindowWMInfo(window, &mut info) {
            // # ifdef
            // _WIN32
            main_viewport.PlatformHandleRaw = info.info.win.window;
            // # elif defined(__APPLE__) && defined(SDL_VIDEO_DRIVER_COCOA)
            // main_viewport.PlatformHandleRaw = (void *) info.info.cocoa.window;
            // # endif
        }
    }

    // Set SDL hint to receive mouse click events on window focus, otherwise SDL doesn't emit the event.
    // Without this, when clicking to gain focus, our widgets wouldn't activate even though they showed as hovered.
    // (This is unfortunately a global SDL setting, so enabling it might have a side-effect on your application.
    // It is unlikely to make a difference, but if your app absolutely needs to ignore the initial on-focus click:
    // you can ignore SDL_MOUSEBUTTONDOWN events coming right after a SDL_WINDOWEVENT_FOCUS_GAINED)
// #if SDL_HAS_MOUSE_FOCUS_CLICKTHROUGH
    unsafe { SDL_SetHint(CString::from(SDL_HINT_MOUSE_FOCUS_CLICKTHROUGH).as_ptr(), CString::from("1").as_ptr()); }
// #endif

    // Update monitors
    ImGui_ImplSDL2_UpdateMonitors();

    // We need SDL_CaptureMouse(), SDL_GetGlobalMouseState() from SDL 2.0.4+ to support multiple viewports.
    // We left the call to ImGui_ImplSDL2_InitPlatformInterface() outside of #ifdef to avoid unused-function warnings.
    if (io.ConfigFlags & ImGuiConfigFlags_ViewportsEnable) && (io.BackendFlags & ImGuiBackendFlags_PlatformHasViewports) {
        ImGui_ImplSDL2_InitPlatformInterface(window, sdl_gl_context);
    }

    return true;
}

pub fn ImGui_ImplSDL2_InitForOpenGL(window: *mut SDL_Window, sdl_gl_context: *mut c_void) -> bool
{
    return ImGui_ImplSDL2_Init(window, null_mut(), sdl_gl_context);
}

pub fn ImGui_ImplSDL2_InitForVulkan(window: *mut SDL_Window) -> bool
{
// #if !SDL_HAS_VULKAN
//     IM_ASSERT(0 && "Unsupported");
// #endif
    if !ImGui_ImplSDL2_Init(window, null_mut(), null_mut()) {
        return false;
    }
    let mut bd = ImGui_ImplSDL2_GetBackendData();
    bd.UseVulkan = true;
    return true;
}

pub fn ImGui_ImplSDL2_InitForD3D(window: *mut SDL_Window) -> bool
{
// #if !defined(_WIN32)
//     IM_ASSERT(0 && "Unsupported");
// #endif
    return ImGui_ImplSDL2_Init(window, null_mut(), null_mut());
}

pub fn ImGui_ImplSDL2_InitForMetal(window: *mut SDL_Window) -> bool
{
    return ImGui_ImplSDL2_Init(window, null_mut(), null_mut());
}

pub fn ImGui_ImplSDL2_InitForSDLRenderer(window: *mut SDL_Window, renderer: *mut SDL_Renderer) -> bool
{
    return ImGui_ImplSDL2_Init(window, renderer, null_mut());
}

pub fn ImGui_ImplSDL2_Shutdown(app_ctx: &mut AppContext)
{
    let mut bd = ImGui_ImplSDL2_GetBackendData();
    // IM_ASSERT(bd != null_mut() && "No platform backend to shutdown, or already shutdown?");
    let mut io = GetIO();

    ImGui_ImplSDL2_ShutdownPlatformInterface(app_ctx);

    if (bd.ClipboardTextData) {
        unsafe { SDL_free(bd.ClipboardTextData as *mut c_void); }
    }
    // for (ImGuiMouseCursor cursor_n = 0; cursor_n < ImGuiMouseCursor_COUNT; cursor_n++)
    for cursor_n in 0 .. ImGuiMouseCursor_COUNT
    {
        unsafe { SDL_FreeCursor(bd.MouseCursors[cursor_n]); }
    }

    io.BackendPlatformName = null_mut();
    io.BackendPlatformUserData = null_mut();
    unsafe { libc::free(bd as *mut c_void); }
}

// This code is incredibly messy because some of the functions we need for full viewport support are not available in SDL < 2.0.4.
pub fn ImGui_ImplSDL2_UpdateMouseData()
{
    let mut bd = ImGui_ImplSDL2_GetBackendData();
    let mut io = GetIO();

    // We forward mouse input when hovered or captured (via SDL_MOUSEMOTION) or when focused (below)
// #if SDL_HAS_CAPTURE_AND_GLOBAL_MOUSE
    // SDL_CaptureMouse() let the OS know e.g. that our imgui drag outside the SDL window boundaries shouldn't e.g. trigger other operations outside
    unsafe { SDL_CaptureMouse(if bd.MouseButtonsDown != 0 { SDL_TRUE } else { SDL_FALSE }); }
    let mut focused_window = unsafe { SDL_GetKeyboardFocus() };
    let is_app_focused = (focused_window.is_null() == false && (bd.Window == focused_window || unsafe { FindViewportByPlatformHandle(focused_window as *mut c_void).is_null() == false }));
// #else
//     focused_window: *mut SDL_Window = bd.Window;
//     const bool is_app_focused = (SDL_GetWindowFlags(bd.Window) & SDL_WINDOW_INPUT_FOCUS) != 0; // SDL 2.0.3 and non-windowed systems: single-viewport only
// #endif

    if is_app_focused
    {
        // (Optional) Set OS mouse position from Dear ImGui if requested (rarely used, only when ImGuiConfigFlags_NavEnableSetMousePos is enabled by user)
        if io.WantSetMousePos
        {
// #if SDL_HAS_CAPTURE_AND_GLOBAL_MOUSE
            if io.ConfigFlags & ImGuiConfigFlags_ViewportsEnable {
                unsafe { SDL_WarpMouseGlobal(io.MousePos.x as i32, io.MousePos.y as i32); }
            }
            else {
// #endif
                unsafe { SDL_WarpMouseInWindow(bd.Window, io.MousePos.x as i32, io.MousePos.y as i32); }
            }
        }

        // (Optional) Fallback to provide mouse position when focused (SDL_MOUSEMOTION already provides this when hovered or captured)
        if bd.MouseCanUseGlobalState && bd.MouseButtonsDown == 0
        {
            // Single-viewport mode: mouse position in client window coordinates (io.MousePos is (0,0) when the mouse is on the upper-left corner of the app window)
            // Multi-viewport mode: mouse position in OS absolute coordinates (io.MousePos is (0,0) when the mouse is on the upper-left of the primary monitor)
            // int mouse_x, mouse_y, window_x, window_y;
            let mut mouse_x = 0i32;
            let mut mouse_y = 0i32;
            let mut window_x = 0i32;
            let mut window_y = 0i32;
            unsafe { SDL_GetGlobalMouseState(&mut mouse_x, &mut mouse_y); }
            if !(io.ConfigFlags & ImGuiConfigFlags_ViewportsEnable)
            {
                unsafe { SDL_GetWindowPosition(focused_window, &mut window_x, &mut window_y); }
                mouse_x -= window_x;
                mouse_y -= window_y;
            }
            io.AddMousePosEvent(mouse_x as f32, mouse_y as f32);
        }
    }

    // (Optional) When using multiple viewports: call io.AddMouseViewportEvent() with the viewport the OS mouse cursor is hovering.
    // If IM_GUI_BACKEND_FLAGS_HAS_MOUSE_HOVERED_VIEWPORT is not set by the backend, Dear imGui will ignore this field and infer the information using its flawed heuristic.
    // - [!] SDL backend does NOT correctly ignore viewports with the _NoInputs flag.
    //       Some backend are not able to handle that correctly. If a backend report an hovered viewport that has the _NoInputs flag (e.g. when dragging a window
    //       for docking, the viewport has the _NoInputs flag in order to allow us to find the viewport under), then Dear ImGui is forced to ignore the value reported
    //       by the backend, and use its flawed heuristic to guess the viewport behind.
    // - [X] SDL backend correctly reports this regardless of another viewport behind focused and dragged from (we need this to find a useful drag and drop target).
    unsafe {
        if io.BackendFlags & ImGuiBackendFlags_HasMouseHoveredViewport {
            let mut mouse_viewport_id = 0;
            let mut sdl_mouse_window = SDL_GetWindowFromID(bd.MouseWindowID);
            if sdl_mouse_window.is_null() == false {
                let mut mouse_viewport = FindViewportByPlatformHandle(sdl_mouse_window as *mut c_void);
                if mouse_viewport.is_null() == false
                {
                    mouse_viewport_id = mouse_viewport.ID;
                }
                io.AddMouseViewportEvent(mouse_viewport_id);
            }
        }
    }
}

pub fn ImGui_ImplSDL2_UpdateMouseCursor()
{
    let mut io = GetIO();
    if (io.ConfigFlags & ImGuiConfigFlags_NoMouseCursorChange) {
        return;
    }
    let mut bd = ImGui_ImplSDL2_GetBackendData();

    let mut imgui_cursor = GetMouseCursor();
    if io.MouseDrawCursor || imgui_cursor == ImGuiMouseCursor_None
    {
        // Hide OS mouse cursor if imgui is drawing it or if it wants no cursor
        unsafe { SDL_ShowCursor(SDL_FALSE as c_int); }
    }
    else
    {
        // Show OS mouse cursor
        unsafe { SDL_SetCursor(if bd.MouseCursors[imgui_cursor] { bd.MouseCursors[imgui_cursor] } else { bd.MouseCursors[ImGuiMouseCursor_Arrow] }); }
        unsafe { SDL_ShowCursor(SDL_TRUE as c_int); }
    }
}



pub fn ImGui_ImplSDL2_UpdateGamepads()
{
    fn map_button(io: &mut IoContext, game_controller: *mut SDL_GameController, key: ImGuiKey, sdl_btn: SDL_GameControllerButton) {
        unsafe { io.AddKeyEvent(KEY_NO, SDL_GameControllerGetButton(game_controller, BUTTON_NO) != 0); }
    }
    let mut io = GetIO();
    // FIXME: Technically feeding gamepad shouldn't depend on this now that they are regular inputs.
    if (io.ConfigFlags & ImGuiConfigFlags_NavEnableGamepad) == 0 {
        return;
    }

    // Get gamepad
    io.BackendFlags &= !mGuiBackendFlags_HasGamepad;
    let mut game_controller = unsafe { SDL_GameControllerOpen(0) };
    if !game_controller {
        return;
    }
    io.BackendFlags |= ImGuiBackendFlags_HasGamepad;

    // Update gamepad inputs
    // #define IM_SATURATE(V)                      (V < 0.0f ? 0.0f : V > 1.0f ? 1.0f : V)
    // #define MAP_BUTTON(key_no, BUTTON_NO)       { io.AddKeyEvent(key_no, SDL_GameControllerGetButton(game_controller, BUTTON_NO) != 0); }
    fn map_analog(io: &mut IoContext, game_controller: *mut SDL_GameController, key_no: ImGuiKey, axis_no: SDL_ControllerAxis, v0: f32, v1: f32) {
        let vn = (unsafe { SDL_GameControllerGetAxis(game_controller, axis_no) } - v0) / (v1 - v0);
        vn = ImSaturateFloat(vn);
        io.AddKeyAnalogEvent(key_no, vn > 0.1f32, vn); }

    let thumb_dead_zone = 8000f32;           // SDL_gamecontroller.h suggests using this value.
    map_button(io, game_controller, ImGuiKey_GamepadStart,           SDL_CONTROLLER_BUTTON_START);
    map_button(io, game_controller, ImGuiKey_GamepadBack,            SDL_CONTROLLER_BUTTON_BACK);
    map_button(io, game_controller, ImGuiKey_GamepadFaceLeft,        SDL_CONTROLLER_BUTTON_X);              // Xbox X, PS Square
    map_button(io, game_controller, ImGuiKey_GamepadFaceRight,       SDL_CONTROLLER_BUTTON_B);              // Xbox B, PS Circle
    map_button(io, game_controller, ImGuiKey_GamepadFaceUp,          SDL_CONTROLLER_BUTTON_Y);              // Xbox Y, PS Triangle
    map_button(io, game_controller, ImGuiKey_GamepadFaceDown,        SDL_CONTROLLER_BUTTON_A);              // Xbox A, PS Cross
    map_button(io, game_controller, ImGuiKey_GamepadDpadLeft,        SDL_CONTROLLER_BUTTON_DPAD_LEFT);
    map_button(io, game_controller, ImGuiKey_GamepadDpadRight,       SDL_CONTROLLER_BUTTON_DPAD_RIGHT);
    map_button(io, game_controller, ImGuiKey_GamepadDpadUp,          SDL_CONTROLLER_BUTTON_DPAD_UP);
    map_button(io, game_controller, ImGuiKey_GamepadDpadDown,        SDL_CONTROLLER_BUTTON_DPAD_DOWN);
    map_button(io, game_controller, ImGuiKey_GamepadL1,              SDL_CONTROLLER_BUTTON_LEFTSHOULDER);
    map_button(io, game_controller, ImGuiKey_GamepadR1,              SDL_CONTROLLER_BUTTON_RIGHTSHOULDER);
    map_analog(io, game_controller, ImGuiKey_GamepadL2,              SDL_CONTROLLER_AXIS_TRIGGERLEFT,  0.0f32, 32767f32);
    map_analog(io, game_controller, ImGuiKey_GamepadR2,              SDL_CONTROLLER_AXIS_TRIGGERRIGHT, 0.0f32, 32767f32);
    map_button(io, game_controller, ImGuiKey_GamepadL3,              SDL_CONTROLLER_BUTTON_LEFTSTICK);
    map_button(io, game_controller, ImGuiKey_GamepadR3,              SDL_CONTROLLER_BUTTON_RIGHTSTICK);
    map_analog(io, game_controller, ImGuiKey_GamepadLStickLeft,      SDL_CONTROLLER_AXIS_LEFTX,  thumb_dead_zone * -1f32, 32768f32 * -1f32);
    map_analog(io, game_controller, ImGuiKey_GamepadLStickRight,     SDL_CONTROLLER_AXIS_LEFTX,  thumb_dead_zone, 32767f32);
    map_analog(io, game_controller, ImGuiKey_GamepadLStickUp,        SDL_CONTROLLER_AXIS_LEFTY,  thumb_dead_zone * -1f32, 32768f32 * -1f32);
    map_analog(io, game_controller, ImGuiKey_GamepadLStickDown,      SDL_CONTROLLER_AXIS_LEFTY,  thumb_dead_zone, 32767f32);
    map_analog(io, game_controller, ImGuiKey_GamepadRStickLeft,      SDL_CONTROLLER_AXIS_RIGHTX, thumb_dead_zone * -1f32, 32768f32 * -1f32);
    map_analog(io, game_controller, ImGuiKey_GamepadRStickRight,     SDL_CONTROLLER_AXIS_RIGHTX, thumb_dead_zone, 32767f32);
    map_analog(io, game_controller, ImGuiKey_GamepadRStickUp,        SDL_CONTROLLER_AXIS_RIGHTY, thumb_dead_zone * -1f32, 32768f32 * -1f32);
    map_analog(io, game_controller, ImGuiKey_GamepadRStickDown,      SDL_CONTROLLER_AXIS_RIGHTY, thumb_dead_zone, 32767f32);
}

// FIXME-PLATFORM: SDL doesn't have an event to notify the application of display/monitor changes
pub fn ImGui_ImplSDL2_UpdateMonitors()
{
    let mut platform_io = GetPlatformIO();
    platform_io.Monitors.resize(0);
    let display_count = unsafe { SDL_GetNumVideoDisplays() };
    // for (int n = 0; n < display_count; n++)
    for n in 0 .. display_count
    {
        // Warning: the validity of monitor DPI information on Windows depends on the application DPI awareness settings, which generally needs to be set in the manifest or at runtime.
        // ImGuiPlatformMonitor monitor;
        let mut monitor: PlatformMonitor = PlatformMonitor::new();
        let mut r: SDL_Rect = SDL_Rect{
            x: 0,
            y: 0,
            w: 0,
            h: 0,
        };
        unsafe { SDL_GetDisplayBounds(n, &mut r); }
        monitor.WorkPos = ImVec2(r.x, r.y);
        monitor.MainPos = monitor.WorkPos;
        monitor.WorkSize = ImVec2(r.w, r.h);
        monitor.MainSize = monitor.WorkSize;
// #if SDL_HAS_USABLE_DISPLAY_BOUNDS
        unsafe { SDL_GetDisplayUsableBounds(n, &mut r); }
        monitor.WorkPos = ImVec2(r.x, r.y);
        monitor.WorkSize = ImVec2(r.w, r.h);
// #endif
// #if SDL_HAS_PER_MONITOR_DPI
        // FIXME-VIEWPORT: On MacOS SDL reports actual monitor DPI scale, ignoring OS configuration. We may want to set
        //  DpiScale to cocoa_window.backingScaleFactor here.
        let mut dpi = 0.0f32;
        unsafe {
            if !SDL_GetDisplayDPI(n, &mut dpi, null_mut(), null_mut()) {
                monitor.DpiScale = dpi / 96.0f32;
            }
        }
// #endif
        platform_io.Monitors.push_back(monitor);
    }
}

pub fn ImGui_ImplSDL2_NewFrame()
{
    let mut bd = ImGui_ImplSDL2_GetBackendData();
    // IM_ASSERT(bd != null_mut() && "Did you call ImGui_ImplSDL2_Init()?");
    let mut io = GetIO();

    // Setup display size (every frame to accommodate for window resizing)
    // int w, h;
    let mut w = 0i32;
    let mut h = 0i32;
    // int display_w, display_h;
    let mut display_w = 0i32;
    let mut display_h = 0i32;

    unsafe { SDL_GetWindowSize(bd.Window, &mut w, &mut h); }
    unsafe {
        if SDL_GetWindowFlags(bd.Window) & SDL_WINDOW_MINIMIZED {
            w = 0;
            h = 0;
        }
    }
    if bd.Renderer != null_mut() {
        unsafe { SDL_GetRendererOutputSize(bd.Renderer, &mut display_w, &mut display_h); }
    }
    else {
        unsafe { SDL_GL_GetDrawableSize(bd.Window, &mut display_w, &mut display_h); }
    }
    io.DisplaySize = ImVec2(w, h);
    if w > 0 && h > 0 {
        io.DisplayFramebufferScale = ImVec2(display_w / w, display_h / h);
    }

    // Setup time step (we don't use SDL_GetTicks() because it is using millisecond resolution)
    let mut frequency = unsafe { SDL_GetPerformanceFrequency() };
    let mut current_time = unsafe { SDL_GetPerformanceCounter() };
    io.DeltaTime = if bd.Time > 0 { ((double)(current_time - bd.Time) / frequency) } else { 1.0f32 / 60.0f32 };
    bd.Time = current_time;

    if bd.PendingMouseLeaveFrame != 0 && bd.PendingMouseLeaveFrame >= GetFrameCount() && bd.MouseButtonsDown == 0
    {
        bd.MouseWindowID = 0;
        bd.PendingMouseLeaveFrame = 0;
        io.AddMousePosEvent(-FLT_MAX, -FLT_MAX);
    }

    ImGui_ImplSDL2_UpdateMouseData();
    ImGui_ImplSDL2_UpdateMouseCursor();

    // Update game controllers (if enabled and available)
    ImGui_ImplSDL2_UpdateGamepads();
}

//--------------------------------------------------------------------------------------------------------
// MULTI-VIEWPORT / PLATFORM INTERFACE SUPPORT
// This is an _advanced_ and _optional_ feature, allowing the backend to create and handle multiple viewports simultaneously.
// If you are new to dear imgui or creating a new binding for dear imgui, it is recommended that you completely ignore this section first..
//--------------------------------------------------------------------------------------------------------

// Helper structure we store in the void* RenderUserData field of each ImGuiViewport to easily retrieve our backend data.
#[derive(Default,Debug,Clone)]
pub struct ImGui_ImplSDL2_ViewportData
{
    // SDL_Window*     Window;
    pub Window: *mut SDL_Window,
    // Uint32          WindowID;
    pub WindowID: u32,
    // bool            WindowOwned;
    pub WindowOwned: bool,
    // SDL_GLContext   GLContext;
    pub GLContext: SDL_GLContext,
    // ImGui_ImplSDL2_ViewportData() { Window = null_mut(); WindowID = 0; WindowOwned = false; GLContext = null_mut(); }
    // ~ImGui_ImplSDL2_ViewportData() { IM_ASSERT(Window == null_mut() && GLContext == null_mut()); }
}

impl ImGui_ImplSDL2_ViewportData {
    pub fn new() -> Self {
        Self {
            Window: null_mut(),
            WindowID: 0,
            WindowOwned: false,
            GLContext: null_mut(),
        }
    }
}

pub fn ImGui_ImplSDL2_CreateWindow(viewport: *mut ImGuiViewport)
{
    let mut bd = ImGui_ImplSDL2_GetBackendData();
    let mut vd: *mut ImGui_ImplSDL2_ViewportData = unsafe { libc::malloc(mem::size_of::<ImGui_ImplSDL2_ViewportData>()) } as *mut ImGui_ImplSDL2_ViewportData;
    viewport.PlatformUserData = vd;

    let mut main_viewport = unsafe { GetMainViewport() };
    let mut main_viewport_data: *mut ImGui_ImplSDL2_ViewportData = main_viewport.PlatformUserData as * mut ImGui_ImplSDL2_ViewportData;

    // Share GL resources with main context
    let use_opengl = (main_viewport_data.GLContext != null_mut());
    let mut backup_context: SDL_GLContext = SDL_GLContext{};
    if use_opengl
    {
        unsafe { backup_context = SDL_GL_GetCurrentContext(); }
        unsafe { SDL_GL_SetAttribute(SDL_GL_SHARE_WITH_CURRENT_CONTEXT, 1); }
        unsafe { SDL_GL_MakeCurrent(main_viewport_data.Window, main_viewport_data.GLContext); }
    }

    let mut sdl_flags = 0;
    sdl_flags |= if use_opengl { SDL_WINDOW_OPENGL } else {
        (if bd.UseVulkan {
            SDL_WINDOW_VULKAN
        } else { 0 })
    };
    unsafe { sdl_flags |= SDL_GetWindowFlags(bd.Window) & SDL_WINDOW_ALLOW_HIGHDPI; }
    sdl_flags |= SDL_WINDOW_HIDDEN;
    sdl_flags |= if viewport.Flags & ImGuiViewportFlags_NoDecoration != 0 { SDL_WINDOW_BORDERLESS } else { 0 };
    sdl_flags |= if (viewport.Flags & ImGuiViewportFlags_NoDecoration)!= 0 { 0 } else { SDL_WINDOW_RESIZABLE };
// #if !defined(_WIN32)
    // See SDL hack in ImGui_ImplSDL2_ShowWindow().
    sdl_flags |= if (viewport.Flags & ImGuiViewportFlags_NoTaskBarIcon) != 0 { SDL_WINDOW_SKIP_TASKBAR } else { 0 };
// #endif
// #if SDL_HAS_ALWAYS_ON_TOP
    sdl_flags |= if (viewport.Flags & ImGuiViewportFlags_TopMost) != 0 { SDL_WINDOW_ALWAYS_ON_TOP } else { 0 };
// #endif
    unsafe { vd.Window = SDL_CreateWindow(CString::from("No Title Yet").as_ptr(), viewport.Pos.x, viewport.Pos.y, viewport.Size.x, viewport.Size.y, sdl_flags); }
    vd.WindowOwned = true;
    if use_opengl
    {
        unsafe { vd.GLContext = SDL_GL_CreateContext(vd.Window); }
        unsafe { SDL_GL_SetSwapInterval(0); }
    }
    if use_opengl {
        unsafe { SDL_GL_MakeCurrent(vd.Window, backup_context); }
    }

    viewport.PlatformHandle = vd.Window;
    viewport.PlatformHandleRaw = null_mut();
    let mut info: SDL_SysWMinfo = SDL_SysWMinfo{
        version: SDL_version {
            major: 0,
            minor: 0,
            patch: 0,
        },
        subsystem: SDL_SYSWM_TYPE::SDL_SYSWM_UNKNOWN,
        info: SDL_SysWMinfo__bindgen_ty_1{},
    };
    SDL_VERSION(&info.version);
    unsafe {
        if SDL_GetWindowWMInfo(vd.Window, &mut info) {
            // # if defined(_WIN32)
            viewport.PlatformHandleRaw = info.info.win.window;
            // # elif
            // defined(__APPLE__) && defined(SDL_VIDEO_DRIVER_COCOA)
            // viewport.PlatformHandleRaw = info.info.cocoa.window;
            // # endif
        }
    }
}

pub fn ImGui_ImplSDL2_DestroyWindow(viewport: *mut ImGuiViewport)
{
    let mut vd = viewport.PlatformUserData as *mut ImGui_ImplSDL2_ViewportData;
    if vd.is_null() == false
    {
        if vd.GLContext.is_null() == false && vd.WindowOwned {
            unsafe { SDL_GL_DeleteContext(vd.GLContext); }
        }
        if vd.Window.is_null() == false && vd.WindowOwned {
            unsafe { SDL_DestroyWindow(vd.Window); }
        }
        vd.GLContext = null_mut();
        vd.Window = null_mut();
        unsafe { libc::free(vd as *mut c_void); }
    }
    viewport.PlatformUserData = viewport.PlatformHandle = null_mut();
}

pub fn ImGui_ImplSDL2_ShowWindow(viewport: *mut ImGuiViewport)
{
    let mut vd = viewport.PlatformUserData as *mut ImGui_ImplSDL2_ViewportData;
// #if defined(_WIN32)
    let hwnd = viewport.PlatformHandleRaw as HWND;

    // SDL hack: Hide icon from task bar
    // Note: SDL 2.0.6+ has a SDL_WINDOW_SKIP_TASKBAR flag which is supported under Windows but the way it create the window breaks our seamless transition.
    if viewport.Flags & ImGuiViewportFlags_NoTaskBarIcon
    {
        let ex_style = GetWindowLong(hwnd, GWL_EXSTYLE);
        ex_style &= !WS_EX_APPWINDOW;
        ex_style |= WS_EX_TOOLWINDOW;
        SetWindowLong(hwnd, GWL_EXSTYLE, ex_style);
    }

    // SDL hack: SDL always activate/focus windows :/
    if viewport.Flags & ImGuiViewportFlags_NoFocusOnAppearing
    {
        ShowWindow(hwnd, SW_SHOWNA);
        return;
    }
// #endif

    unsafe { SDL_ShowWindow(vd.Window); }
}

pub fn ImGui_ImplSDL2_GetWindowPos(viewport: *mut ImGuiViewport) -> ImVec2
{
    let mut vd = viewport.PlatformUserData as *mut ImGui_ImplSDL2_ViewportData;
    // int x = 0, y = 0;
    let mut x = 0i32;
    let mut y = 0i32;
    unsafe { SDL_GetWindowPosition(vd.Window, &mut x, &mut y); }
    return ImVec2(x, y);
}

pub fn ImGui_ImplSDL2_SetWindowPos(viewport: *mut ImGuiViewport, pos: ImVec2)
{
    let mut vd = viewport.PlatformUserData as *mut ImGui_ImplSDL2_ViewportData;
    unsafe { SDL_SetWindowPosition(vd.Window, pos.x, pos.y); }
}

pub fn ImGui_ImplSDL2_GetWindowSize(viewport: *mut ImGuiViewport) -> ImVec2
{
    let mut vd = viewport.PlatformUserData as *mut ImGui_ImplSDL2_ViewportData;
    // int w = 0, h = 0;
    let mut w = 0i32;
    let mut h = 0i32;
    unsafe { SDL_GetWindowSize(vd.Window, &mut w, &mut h); }
    return ImVec2(w, h);
}

pub fn ImGui_ImplSDL2_SetWindowSize(viewport: *mut ImGuiViewport, size: ImVec2)
{
    let mut vd = viewport.PlatformUserData as *mut ImGui_ImplSDL2_ViewportData;
    unsafe { SDL_SetWindowSize(vd.Window, size.x, size.y); }
}

pub fn ImGui_ImplSDL2_SetWindowTitle(viewport: *mut ImGuiViewport, title: *const c_char)
{
    let mut vd = viewport.PlatformUserData as *mut ImGui_ImplSDL2_ViewportData;
    unsafe { SDL_SetWindowTitle(vd.Window, title); }
}

// #if SDL_HAS_WINDOW_ALPHA
pub fn ImGui_ImplSDL2_SetWindowAlpha(viewport: *mut ImGuiViewport, alpha: f32)
{
    let mut vd = viewport.PlatformUserData as *mut ImGui_ImplSDL2_ViewportData;
    unsafe { SDL_SetWindowOpacity(vd.Window, alpha); }
}
// #endif


pub fn ImGui_ImplSDL2_SetWindowFocus(viewport: *mut ImGuiViewport)
{
    let mut vd = viewport.PlatformUserData as *mut ImGui_ImplSDL2_ViewportData;
    unsafe { SDL_RaiseWindow(vd.Window); }
}

pub fn ImGui_ImplSDL2_GetWindowFocus(viewport: *mut ImGuiViewport) -> bool
{
    let mut vd = viewport.PlatformUserData as *mut ImGui_ImplSDL2_ViewportData;
    unsafe { return (SDL_GetWindowFlags(vd.Window) & SDL_WINDOW_INPUT_FOCUS) != 0; }
}

pub fn ImGui_ImplSDL2_GetWindowMinimized(viewport: *mut ImGuiViewport) -> bool
{
    let mut vd = viewport.PlatformUserData as *mut ImGui_ImplSDL2_ViewportData;
    unsafe { return (SDL_GetWindowFlags(vd.Window) & SDL_WINDOW_MINIMIZED) != 0; }
}

pub fn ImGui_ImplSDL2_RenderWindow(viewport: *mut ImGuiViewport)
{
    let mut vd = viewport.PlatformUserData as *mut ImGui_ImplSDL2_ViewportData;
    if (vd.GLContext) {
        unsafe { SDL_GL_MakeCurrent(vd.Window, vd.GLContext); }
    }
}

pub fn ImGui_ImplSDL2_SwapBuffers(viewport: *mut ImGuiViewport)
{
    let mut vd = viewport.PlatformUserData as *mut ImGui_ImplSDL2_ViewportData;
    if vd.GLContext
    {
        unsafe { SDL_GL_MakeCurrent(vd.Window, vd.GLContext); }
        unsafe { SDL_GL_SwapWindow(vd.Window); }
    }
}

// Vulkan support (the Vulkan renderer needs to call a platform-side support function to create the surface)
// SDL is graceful enough to _not_ need <vulkan/vulkan.h> so we can safely include this.
// #if SDL_HAS_VULKAN
// #include <SDL_vulkan.h>
pub fn ImGui_ImplSDL2_CreateVkSurface(viewport: *mut ImGuiViewport, vk_instance: u64, vk_allocator: *mut c_void, out_vk_surface: *mut u64) -> i32
{
    let mut vd = viewport.PlatformUserData as *mut ImGui_ImplSDL2_ViewportData;
    // (void)vk_allocator;
    let ret = unsafe { SDL_Vulkan_CreateSurface(vd.Window, vk_instance as VkInstance, out_vk_surface) };
    return if ret { 0 } else { 1 }; // ret ? VK_SUCCESS : VK_NOT_READY
}
// #endif // SDL_HAS_VULKAN

pub fn ImGui_ImplSDL2_InitPlatformInterface(window: *mut SDL_Window, sdl_gl_context: *mut c_void)
{
    // Register platform interface (will be coupled with a renderer interface)
    ImGuiPlatformIO& platform_io = GetPlatformIO();
    platform_io.Platform_CreateWindow = ImGui_ImplSDL2_CreateWindow;
    platform_io.Platform_DestroyWindow = ImGui_ImplSDL2_DestroyWindow;
    platform_io.Platform_ShowWindow = ImGui_ImplSDL2_ShowWindow;
    platform_io.Platform_SetWindowPos = ImGui_ImplSDL2_SetWindowPos;
    platform_io.Platform_GetWindowPos = ImGui_ImplSDL2_GetWindowPos;
    platform_io.Platform_SetWindowSize = ImGui_ImplSDL2_SetWindowSize;
    platform_io.Platform_GetWindowSize = ImGui_ImplSDL2_GetWindowSize;
    platform_io.Platform_SetWindowFocus = ImGui_ImplSDL2_SetWindowFocus;
    platform_io.Platform_GetWindowFocus = ImGui_ImplSDL2_GetWindowFocus;
    platform_io.Platform_GetWindowMinimized = ImGui_ImplSDL2_GetWindowMinimized;
    platform_io.Platform_SetWindowTitle = ImGui_ImplSDL2_SetWindowTitle;
    platform_io.Platform_RenderWindow = ImGui_ImplSDL2_RenderWindow;
    platform_io.Platform_SwapBuffers = ImGui_ImplSDL2_SwapBuffers;
// #if SDL_HAS_WINDOW_ALPHA
    platform_io.Platform_SetWindowAlpha = ImGui_ImplSDL2_SetWindowAlpha;
// #endif
// #if SDL_HAS_VULKAN
    platform_io.Platform_CreateVkSurface = ImGui_ImplSDL2_CreateVkSurface;
// #endif

    // Register main window handle (which is owned by the main application, not by us)
    // This is mostly for simplicity and consistency, so that our code (e.g. mouse handling etc.) can use same logic for main and secondary viewports.
    unsafe { let mut main_viewport = GetMainViewport(); }
    let mut vd = IM_NEW(ImGui_ImplSDL2_ViewportData)();
    vd.Window = window;
    unsafe { vd.WindowID = SDL_GetWindowID(window); }
    vd.WindowOwned = false;
    vd.GLContext = sdl_gl_context;
    main_viewport.PlatformUserData = vd;
    main_viewport.PlatformHandle = vd.Window;
}

pub fn ImGui_ImplSDL2_ShutdownPlatformInterface(app_ctx: &mut AppContext)
{
    DestroyPlatformWindows(app_ctx);
}
