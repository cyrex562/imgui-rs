use crate::config::ConfigFlags;
use crate::{Context, hash_string, INVALID_ID, Viewport, ViewportFlags};
use crate::orig_imgui_single_file::{Viewport, Window, int};
use crate::rect::Rect;
use crate::vectors::vector_2d::Vector2D;
use crate::window::checks::is_window_active_and_visible;
use crate::window::get::get_window_for_title_display;

// (Optional) This is required when enabling multi-viewport. Represent the bounds of each connected monitor/display and their DPI.
// We use this information for multiple DPI support + clamping the position of popups and tooltips so they don't straddle multiple monitors.
#[derive(Debug,Clone,Default)]
pub struct PlatformDisplay
{
    // Vector2D  main_pos, main_size;      // Coordinates of the area displayed on this monitor (min = upper left, max = bottom right)
    pub main_pos: Vector2D,
    pub main_size: Vector2D,
    // Vector2D  work_pos, work_size;      // Coordinates without task bars / side bars / menu bars. Used to avoid positioning popups/tooltips inside this region. If you don't have this info, please copy the value for main_pos/main_size.
    pub work_pos: Vector2D,
    pub work_size: Vector2D,
    pub dpi_scale: f32,              // 1.0 = 96 DPI
    // ImGuiPlatformMonitor()          { main_pos = main_size = work_pos = work_size = Vector2D(0, 0); dpi_scale = 1.0; }
}

impl PlatformDisplay {
    pub fn new() -> Self {
        Self {
            main_pos: Default::default(),
            main_size: Default::default(),
            work_pos: Default::default(),
            work_size: Default::default(),
            dpi_scale: 1.0
        }
    }
}

// (Optional) Support for IME (Input Method Editor) via the io.SetPlatformImeDataFn() function.
#[derive(Debug,Default,Clone)]
pub struct PlatformImeData
{
    pub want_visible: bool,        // A widget wants the IME to be visible
    pub input_pos: Vector2D,           // Position of the input cursor
    pub input_line_height: f32,   // Line height

    // ImGuiPlatformImeData() { memset(this, 0, sizeof(*this)); }
}

impl PlatformImeData {
    pub fn new(initial_input_pos: Vector2D) -> Self {
        Self {
            want_visible: false,
            input_pos: initial_input_pos,
            input_line_height: 0.0
        }
    }
}

// Called by user at the end of the main loop, after EndFrame()
// This will handle the creation/update of all OS windows via function defined in the ImGuiPlatformIO api.
// void UpdatePlatformWindows()
pub fn update_platform_windows(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.frame_count_ended == g.frame_count && "Forgot to call Render() or EndFrame() before UpdatePlatformWindows()?");
    // IM_ASSERT(g.frame_count_plantform_ended < g.frame_count);
    g.frame_count_plantform_ended = g.frame_count;
    if !(g.config_flags_curr_frame.contains(&ConfigFlags::ViewportsEnable)) {
        return;
    }

    // Create/resize/destroy platform windows to match each active viewport.
    // Skip the main viewport (index 0), which is always fully handled by the application!
    // for (int i = 1; i < g.viewports.size; i += 1)
    for i in 1 .. g.viewports.len()
    {
        // ImGuiViewportP* viewport = g.viewports[i];
        let viewport = &mut g.viewports[i];

        // Destroy platform window if the viewport hasn't been submitted or if it is hosting a hidden window
        // (the implicit/fallback Debug##Default window will be registering its viewport then be disabled, causing a dummy DestroyPlatformWindow to be made each frame)
        let mut do_destroy_platform_window = false;
        do_destroy_platform_window |= (viewport.last_frame_active < g.frame_count - 1);
        do_destroy_platform_window |= (viewport.Window && !is_window_active_and_visible(viewport.Window));
        if do_destroy_platform_window
        {
            destroy_platform_window(g, viewport);
            continue;
        }

        // New windows that appears directly in a new viewport won't always have a size on their first frame
        if viewport.last_frame_active < g.frame_count || viewport.size.x <= 0 || viewport.size.y <= 0 {
            continue;
        }

        // Create window
        let is_new_platform_window = (viewport.platform_window_created == false);
        if is_new_platform_window
        {
            IMGUI_DEBUG_LOG_VIEWPORT("[viewport] Create Platform window %08X '%s'\n", viewport.id, viewport.Window ? viewport.Window.name : "n/a");
            g.platform_io.platform_create_window(viewport);
            if g.platform_io.renderer_create_window != None {
                g.platform_io.renderer_create_window(viewport);
            }
            viewport.last_name_hash = 0;
            viewport.last_platform_size = Vector2D::new(f32::MAX, f32::MAX);
            viewport.last_platform_pos = viewport.last_platform_size.clone(); // By clearing those we'll enforce a call to platform_set_window_pos/size below, before Platform_ShowWindow (FIXME: Is that necessary?)
            viewport.last_renderer_size = viewport.size.clone();                                       // We don't need to call renderer_set_window_size() as it is expected renderer_create_window() already did it.
            viewport.platform_window_created = true;
        }

        // Apply Position and size (from ImGui to Platform/Renderer backends)
        if (viewport.last_platform_pos.x != viewport.pos.x || viewport.last_platform_pos.y != viewport.pos.y) && !viewport.platform_request_move {
            g.platform_io.platform_set_window_pos(viewport, &viewport.pos);
        }
        if (viewport.last_platform_size.x != viewport.size.x || viewport.last_platform_size.y != viewport.size.y) && !viewport.platform_requsest_resize {
            g.platform_io.platform_set_window_size(viewport, &viewport.size);
        }
        if (viewport.last_renderer_size.x != viewport.size.x || viewport.last_renderer_size.y != viewport.size.y) && g.platform_io.renderer_set_window_size {
            g.platform_io.renderer_set_window_size(viewport, &viewport.size);
        }
        viewport.last_platform_pos = viewport.pos.clone();
        viewport.last_platform_size = viewport.size.clone();
        viewport.last_renderer_size = viewport.size.clone();

        // update title bar (if it changed)
        let window_for_title = &mut get_window_for_title_display(g, viewport.window_id);
        // if (Window* window_for_title = GetWindowForTitleDisplay(viewport.Window))
        if window_for_title.id != INVALID_ID
        {
            // const char* title_begin = window_for_title.name;
            let title_begin = &mut window_for_title.name;
            let title_end = find_rendered_text_end(title_begin);
            let title_hash = hash_string(title_begin, title_end - title_begin);
            if viewport.last_name_hash != title_hash
            {
                // char title_end_backup_c = *title_end;
                let title_end_backup_c = title_end;
                // *title_end = 0; // Cut existing buffer short instead of doing an alloc/free, no small gain.
                g.platform_io.platform_set_window_title(viewport, title_begin);
                *title_end = title_end_backup_c;
                viewport.last_name_hash = title_hash;
            }
        }

        // update alpha (if it changed)
        if viewport.last_alpha != viewport.alpha && g.platform_io.platform_set_window_alpha {
            g.platform_io.platform_set_window_alpha(viewport, viewport.alpha);
        }
        viewport.last_alpha = viewport.alpha;

        // Optional, general purpose call to allow the backend to perform general book-keeping even if things haven't changed.
        if g.platform_io.platform_update_window {
            g.platform_io.platform_update_window(viewport);
        }

        if is_new_platform_window
        {
            // On startup ensure new platform window don't steal focus (give it a few frames, as nested contents may lead to viewport being created a few frames late)
            if g.frame_count < 3 {
                viewport.flags |= ViewportFlags::NoFocusOnAppearing;
            }

            // Show window
            g.platform_io.platform_show_window(viewport);

            // Even without focus, we assume the window becomes front-most.
            // This is useful for our platform z-order heuristic when io.mouse_hovered_viewport is not available.
            if viewport.last_frontmost_stamp_count != g.viewport_frontmost_stamp_count {
                 g.viewport_frontmost_stamp_count += 1;
                viewport.last_frontmost_stamp_count = viewport_frontmost_stamp_count;
            }
        }

        // clear request flags
        viewport.ClearRequestFlags();
    }

    // update our implicit z-order knowledge of platform windows, which is used when the backend cannot provide io.mouse_hovered_viewport.
    // When setting Platform_GetWindowFocus, it is expected that the platform backend can handle calls without crashing if it doesn't have data stored.
    // FIXME-VIEWPORT: We should use this information to also set dear imgui-side focus, allowing us to handle os-level alt+tab.
    if (g.platform_io.Platform_GetWindowFocus != None)
    {
        ViewportP* focused_viewport = None;
        for (int n = 0; n < g.viewports.size && focused_viewport == None; n += 1)
        {
            ViewportP* viewport = g.viewports[n];
            if (viewport.platform_window_created)
                if (g.platform_io.Platform_GetWindowFocus(viewport))
                    focused_viewport = viewport;
        }

        // Store a tag so we can infer z-order easily from all our windows
        // We compare platform_last_focused_viewport_id so newly created viewports with _NoFocusOnAppearing flag
        // will keep the front most stamp instead of losing it back to their parent viewport.
        if (focused_viewport && g.PlatformLastFocusedViewportId != focused_viewport.id)
        {
            if (focused_viewport.last_frontmost_stamp_count != g.viewport_frontmost_stamp_count)
                focused_viewport.last_frontmost_stamp_count = g.viewport_frontmost_stamp_count += 1;
            g.PlatformLastFocusedViewportId = focused_viewport.id;
        }
    }
}

// This is a default/basic function for performing the rendering/swap of multiple Platform windows.
// Custom renderers may prefer to not call this function at all, and instead iterate the publicly exposed platform data and handle rendering/sync themselves.
// The Render/Swap functions stored in ImGuiPlatformIO are merely here to allow for this helper to exist, but you can do it yourself:
//
//    ImGuiPlatformIO& platform_io = GetPlatformIO();
//    for (int i = 1; i < platform_io.viewports.size; i++)
//        if ((platform_io.viewports[i]->flags & ImGuiViewportFlags_Minimized) == 0)
//            MyRenderFunction(platform_io.viewports[i], my_args);
//    for (int i = 1; i < platform_io.viewports.size; i++)
//        if ((platform_io.viewports[i]->flags & ImGuiViewportFlags_Minimized) == 0)
//            MySwapBufferFunction(platform_io.viewports[i], my_args);
//
// void RenderPlatformWindowsDefault(void* platform_render_arg, void* renderer_render_arg)
pub fn render_platform_windows_default(g: &mut Context, platform_render_arg: &Vec<u8>, renderer_render_arg: &Vec<u8>)
{
    // Skip the main viewport (index 0), which is always fully handled by the application!
    ImGuiPlatformIO& platform_io = GetPlatformIO();
    for (int i = 1; i < platform_io.viewports.size; i += 1)
    {
        Viewport* viewport = platform_io.viewports[i];
        if (viewport.flags & ViewportFlags::Minimized)
            continue;
        if (platform_io.Platform_RenderWindow) platform_io.Platform_RenderWindow(viewport, platform_render_arg);
        if (platform_io.Renderer_RenderWindow) platform_io.Renderer_RenderWindow(viewport, renderer_render_arg);
    }
    for (int i = 1; i < platform_io.viewports.size; i += 1)
    {
        Viewport* viewport = platform_io.viewports[i];
        if (viewport.flags & ViewportFlags::Minimized)
            continue;
        if (platform_io.Platform_SwapBuffers) platform_io.Platform_SwapBuffers(viewport, platform_render_arg);
        if (platform_io.Renderer_SwapBuffers) platform_io.Renderer_SwapBuffers(viewport, renderer_render_arg);
    }
}

// static int FindPlatformMonitorForPos(const Vector2D& pos)
pub fn find_platform_monitor_for_pos(g: &mut Context, pos: &Vector2D) -> i32
{
    // ImGuiContext& g = *GImGui;
    for (int monitor_n = 0; monitor_n < g.platform_io.monitors.size; monitor_n += 1)
    {
        const ImGuiPlatformMonitor& monitor = g.platform_io.monitors[monitor_n];
        if (Rect(monitor.MainPos, monitor.MainPos + monitor.MainSize).contains(pos))
            return monitor_n;
    }
    return -1;
}

// Search for the monitor with the largest intersection area with the given rectangle
// We generally try to avoid searching loops but the monitor count should be very small here
// FIXME-OPT: We could test the last monitor used for that viewport first, and early
// static int FindPlatformMonitorForRect(const Rect& rect)
pub fn find_platform_monitor_for_rect(g: &mut Context, rect: &Rect) -> i32
{
    // ImGuiContext& g = *GImGui;

    let monitor_count = g.platform_io.monitors.size;
    if (monitor_count <= 1)
        return monitor_count - 1;

    // Use a minimum threshold of 1.0 so a zero-sized rect won't false positive, and will still find the correct monitor given its position.
    // This is necessary for tooltips which always resize down to zero at first.
    let surface_threshold = ImMax(rect.get_width() * rect.get_height() * 0.5, 1.0);
    int best_monitor_n = -1;
    let best_monitor_surface =  0.001;

    for (int monitor_n = 0; monitor_n < g.platform_io.monitors.size && best_monitor_surface < surface_threshold; monitor_n += 1)
    {
        const ImGuiPlatformMonitor& monitor = g.platform_io.monitors[monitor_n];
        const Rect monitor_rect = Rect(monitor.MainPos, monitor.MainPos + monitor.MainSize);
        if (monitor_rect.contains(rect))
            return monitor_n;
        Rect overlapping_rect = rect;
        overlapping_rect.ClipWithFull(monitor_rect);
        let overlapping_surface =  overlapping_rect.get_width() * overlapping_rect.get_height();
        if (overlapping_surface < best_monitor_surface)
            continue;
        best_monitor_surface = overlapping_surface;
        best_monitor_n = monitor_n;
    }
    return best_monitor_n;
}

// update monitor from viewport rectangle (we'll use this info to clamp windows and save windows lost in a removed monitor)
// static void UpdateViewportPlatformMonitor(ImGuiViewportP* viewport)
pub fn update_viewport_platform_monitor(g: &mut Context, viewport: &mut Viewport)
{
    viewport.PlatformMonitor = FindPlatformMonitorForRect(viewport.get_main_rect());
}

// Return value is always != None, but don't hold on it across frames.
// const ImGuiPlatformMonitor* GetViewportPlatformMonitor(ImGuiViewport* viewport_p)
pub fn get_viewport_platform_monitor(g: &mut Context, viewport: &mut Viewport) -> &mut PlatformDisplay
{
    // ImGuiContext& g = *GImGui;
    ViewportP* viewport = (ViewportP*)(void*)viewport_p;
    int monitor_idx = viewport.PlatformMonitor;
    if (monitor_idx >= 0 && monitor_idx < g.platform_io.monitors.size)
        return &g.platform_io.monitors[monitor_idx];
    return &g.FallbackMonitor;
}

// void DestroyPlatformWindow(ImGuiViewportP* viewport)
pub fn destroy_platform_window(g: &mut Context, viewport: &mut Viewport)
{
    // ImGuiContext& g = *GImGui;
    if (viewport.platform_window_created)
    {
        if (g.platform_io.Renderer_DestroyWindow)
            g.platform_io.Renderer_DestroyWindow(viewport);
        if (g.platform_io.Platform_DestroyWindow)
            g.platform_io.Platform_DestroyWindow(viewport);
        // IM_ASSERT(viewport.RendererUserData == None && viewport.PlatformUserData == None);

        // Don't clear PlatformWindowCreated for the main viewport, as we initially set that up to true in Initialize()
        // The righter way may be to leave it to the backend to set this flag all-together, and made the flag public.
        if (viewport.id != IMGUI_VIEWPORT_DEFAULT_ID)
            viewport.platform_window_created = false;
    }
    else
    {
        // IM_ASSERT(viewport.RendererUserData == None && viewport.PlatformUserData == None && viewport.PlatformHandle == None);
    }
    viewport.RendererUserData = viewport.PlatformUserData = viewport.PlatformHandle = None;
    viewport.ClearRequestFlags();
}

// void destroy_platform_windows()
pub fn destroy_platform_windows(g: &mut Context)
{
    // We call the destroy window on every viewport (including the main viewport, index 0) to give a chance to the backend
    // to clear any data they may have stored in e.g. PlatformUserData, renderer_user_data.
    // It is convenient for the platform backend code to store something in the main viewport, in order for e.g. the mouse handling
    // code to operator a consistent manner.
    // It is expected that the backend can handle calls to Renderer_DestroyWindow/Platform_DestroyWindow without
    // crashing if it doesn't have data stored.
    // ImGuiContext& g = *GImGui;
    for (int i = 0; i < g.viewports.size; i += 1)
        DestroyPlatformWindow(g.viewports[i]);
}
