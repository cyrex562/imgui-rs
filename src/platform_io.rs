#![allow(non_snake_case)]

use libc::{c_char, c_float, c_void};

// (Optional) Access via GetPlatformIO()
#[derive(Default,Debug,Clone)]
pub struct ImGuiPlatformIO
{
    //------------------------------------------------------------------
    // Input - Backend interface/functions + Monitor List
    //------------------------------------------------------------------

    // (Optional) Platform functions (e.g. Win32, GLFW, SDL2)
    // For reference, the second column shows which function are generally calling the Platform Functions:
    //   N = NewFrame()                        ~ beginning of the dear imgui frame: read info from platform/OS windows (latest size/position)
    //   F = Begin(), EndFrame()        ~ during the dear imgui frame
    //   U = UpdatePlatformWindows()           ~ after the dear imgui frame: create and update all platform/OS windows
    //   R = RenderPlatformWindowsDefault()    ~ render
    //   D = DestroyPlatformWindows()          ~ shutdown
    // The general idea is that NewFrame() we will read the current Platform/OS state, and UpdatePlatformWindows() will write to it.
    //
    // The functions are designed so we can mix and match 2 imgui_impl_xxxx files, one for the Platform (~window/input handling), one for Renderer.
    // Custom engine backends will often provide both Platform and Renderer interfaces and so may not need to use all functions.
    // Platform functions are typically called before their Renderer counterpart, apart from Destroy which are called the other way.

    // Platform function --------------------------------------------------- Called by -----
    // void    (*Platform_CreateWindow)(ImGuiViewport* vp);                    // . . U . .  // Create a new platform window for the given viewport
    pub Platform_CreateWindow: fn(vp: *mut ImGuiViewport),

    // void    (*Platform_DestroyWindow)(ImGuiViewport* vp);                   // N . U . D  //
    pub Platform_DestroyWindow: fn(vp: *mut ImGuiViewport),

    // void    (*Platform_ShowWindow)(ImGuiViewport* vp);                      // . . U . .  // Newly created windows are initially hidden so SetWindowPos/Size/Title can be called on them before showing the window
    pub Platform_ShowWindow: fn(vp: *mut ImGuiViewport),

    // void    (*Platform_SetWindowPos)(ImGuiViewport* vp, ImVec2 pos);        // . . U . .  // Set platform window position (given the upper-left corner of client area)
    pub Platform_SetWindowPos: fn(vp: *mut ImGuiViewport, pos: ImVec2),

    // ImVec2  (*Platform_GetWindowPos)(ImGuiViewport* vp);                    // N . . . .  //
    pub Platform_GetWindowPos: fn(vp: *mut ImGuiViewport) -> ImVec2,

    // void    (*Platform_SetWindowSize)(ImGuiViewport* vp, ImVec2 size);      // . . U . .  // Set platform window client area size (ignoring OS decorations such as OS title bar etc.)
    pub Platform_SetWindowSize: fn(vp: *mut ImGuiViewport, size: ImVec2),

    // ImVec2  (*Platform_GetWindowSize)(ImGuiViewport* vp);                   // N . . . .  // Get platform window client area size
    pub Platform_GetWindowSize: fn(vp: *mut ImGuiViewport) -> ImVec2,

    // void    (*Platform_SetWindowFocus)(ImGuiViewport* vp);                  // N . . . .  // Move window to front and set input focus
    pub Platform_SetWindowFocus: fn(vp: *mut ImGuiViewport),

    // bool    (*Platform_GetWindowFocus)(ImGuiViewport* vp);                  // . . U . .  //
    pub Platform_GetWindowFocus: fn(vp: *mut ImGuiViewport) -> bool,

    // bool    (*Platform_GetWindowMinimized)(ImGuiViewport* vp);              // N . . . .  // Get platform window minimized state. When minimized, we generally won't attempt to get/set size and contents will be culled more easily
    pub Platform_GetWindowMinimized: fn(vp: *mut ImGuiViewport) -> bool,

    // void    (*Platform_SetWindowTitle)(ImGuiViewport* vp, const char* str); // . . U . .  // Set platform window title (given an UTF-8 string)
    pub Platform_SetWindowTitle: fn(vp: *mut ImGuiViewport, title: *const c_char),

    // void    (*Platform_SetWindowAlpha)(ImGuiViewport* vp, float alpha);     // . . U . .  // (Optional) Setup global transparency (not per-pixel transparency)
    pub Platform_SetWindowAlpha: fn(vp: *mut ImGuiViewport, alpha: c_float),

    // void    (*Platform_UpdateWindow)(ImGuiViewport* vp);                    // . . U . .  // (Optional) Called by UpdatePlatformWindows(). Optional hook to allow the platform backend from doing general book-keeping every frame.
    pub Platform_UpdateWindow: fn(vp: *mut ImGuiViewport),

    // void    (*Platform_RenderWindow)(ImGuiViewport* vp, void* render_arg);  // . . . R .  // (Optional) Main rendering (platform side! This is often unused, or just setting a "current" context for OpenGL bindings). 'render_arg' is the value passed to RenderPlatformWindowsDefault().
    pub Platform_RenderWindow: fn(vp: *mut ImGuiViewport, render_arg: *mut c_void),

    // void    (*Platform_SwapBuffers)(ImGuiViewport* vp, void* render_arg);   // . . . R .  // (Optional) Call Present/SwapBuffers (platform side! This is often unused!). 'render_arg' is the value passed to RenderPlatformWindowsDefault().
    pub Platform_SwapBuffers: fn(vp: *mut ImGuiViewport, render_arg: *mut c_void),

    // float   (*Platform_GetWindowDpiScale)(ImGuiViewport* vp);               // N . . . .  // (Optional) [BETA] FIXME-DPI: DPI handling: Return DPI scale for this viewport. 1f32 = 96 DPI.

    // void    (*Platform_OnChangedViewport)(ImGuiViewport* vp);               // . F . . .  // (Optional) [BETA] FIXME-DPI: DPI handling: Called during Begin() every time the viewport we are outputting into changes, so backend has a chance to swap fonts to adjust style.

    // int     (*Platform_CreateVkSurface)(ImGuiViewport* vp, u64 vk_inst, const void* vk_allocators, u64* out_vk_surface); // (Optional) For a Vulkan Renderer to call into Platform code (since the surface creation needs to tie them both).

    // (Optional) Renderer functions (e.g. DirectX, OpenGL, Vulkan)
    // void    (*Renderer_CreateWindow)(ImGuiViewport* vp);                    // . . U . .  // Create swap chain, frame buffers etc. (called after Platform_CreateWindow)

    // void    (*Renderer_DestroyWindow)(ImGuiViewport* vp);                   // N . U . D  // Destroy swap chain, frame buffers etc. (called before Platform_DestroyWindow)

    // void    (*Renderer_SetWindowSize)(ImGuiViewport* vp, ImVec2 size);      // . . U . .  // Resize swap chain, frame buffers etc. (called after Platform_SetWindowSize)

    // void    (*Renderer_RenderWindow)(ImGuiViewport* vp, void* render_arg);  // . . . R .  // (Optional) Clear framebuffer, setup render target, then render the viewport.DrawData. 'render_arg' is the value passed to RenderPlatformWindowsDefault().

    // void    (*Renderer_SwapBuffers)(ImGuiViewport* vp, void* render_arg);   // . . . R .  // (Optional) Call Present/SwapBuffers. 'render_arg' is the value passed to RenderPlatformWindowsDefault().

    // (Optional) Monitor list
    // - Updated by: app/backend. Update every frame to dynamically support changing monitor or DPI configuration.
    // - Used by: dear imgui to query DPI info, clamp popups/tooltips within same monitor and not have them straddle monitors.
    // ImVector<ImGuiPlatformMonitor>  Monitors;
    pub Monitors: Vec<ImGuiPlatformMonitor>,

    //------------------------------------------------------------------
    // Output - List of viewports to render into platform windows
    //------------------------------------------------------------------

    // Viewports list (the list is updated by calling EndFrame or Render)
    // (in the future we will attempt to organize this feature to remove the need for a "main viewport")
    // ImVector<ImGuiViewport*>        Viewports;                              // Main viewports, followed by all secondary viewports.
    pub Viewports: Vec<ImGUiViewport>,

    // ImGuiPlatformIO()               { memset(this, 0, sizeof(*this)); }     // Zero clear
}