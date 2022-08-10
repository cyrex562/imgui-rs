use std::collections::HashSet;
use crate::config::ConfigFlags;
use crate::{Context, INVALID_ID};
use crate::draw::data::{DrawData, DrawDataBuilder};
use crate::draw::draw_data_builder::DrawDataBuilder;
use crate::draw::list::DrawList;
use crate::orig_imgui_single_file::{Id32, Window};
use crate::rect::Rect;
use crate::types::Id32;
use crate::vectors::vector_2d::Vector2D;
use crate::window::{Window, WindowFlags};
use crate::window::checks::is_window_active_and_visible;
use crate::window::layer::bring_window_to_display_front;
use crate::window::next_window::NextWindowDataFlags;
use crate::window::pos::translate_window;
use crate::window::size::scale_window;

/// ImGuiViewport Private/Internals fields (cardinal sin: we are using inheritance!)
/// Every instance of ImGuiViewport is in fact a ImGuiViewportP.
// #[derive(Debug,Clone,Default)]
// pub struct ViewportP
// {
//     /
// }
//
// impl ViewportP {
//     // // ImGuiViewportP()                    { Idx = -1; last_frame_active = DrawListsLastFrame[0] = DrawListsLastFrame[1] = LastFrontMostStampCount = -1; last_name_hash = 0; alpha = LastAlpha = 1.0; platform_monitor = -1; PlatformWindowCreated = false; window = None; DrawLists[0] = DrawLists[1] = None; last_platform_pos = last_platform_size = last_renderer_size = Vector2D(FLT_MAX, FLT_MAX); }
//     pub fn new() -> Self {
//         Self {
//             idx: -1,
//             last_frame_active: -1,
//             draw_lists_last_frame: [-1;2],
//             last_front_most_stamp_count: -1,
//             last_name_hash: 0,
//             alpha: 1.0,
//             last_alpha: 1.0,
//             platform_monitor: -1,
//             platform_window_created: false,
//             window: Id32::MAX,
//             draw_lists: [0;2],
//             last_platform_pos: Vector2D::new(f32::MAX, f32::MAX),
//             last_platform_size: Vector2D::new(f32::MAX, y: f32::MAX),
//             last_renderer_size: Vector2D::new(f32::MAX,f32::MAX),
//
//         }
//     }
//
//     //     // ~ImGuiViewportP()                   { if (DrawLists[0]) IM_DELETE(DrawLists[0]); if (DrawLists[1]) IM_DELETE(DrawLists[1]); }
//     //     // void    clear_request_flags()         { platform_request_close = platform_request_move = platform_request_resize = false; }
//     //
//
// }

// flags stored in ImGuiViewport::flags, giving indications to the platform backends.
#[derive(Debug,Clone)]
pub enum ViewportFlags
{
    None                     = 0,
    IsPlatformWindow        ,   // Represent a Platform window
    Isplatform_monitor       ,   // Represent a Platform Monitor (unused yet)
    OwnedByApp              ,   // Platform window: is created/managed by the application (rather than a dear imgui backend)
    NoDecoration            ,   // Platform window: Disable platform decorations: title bar, borders, etc. (generally set all windows, but if ImGuiConfigFlags_ViewportsDecoration is set we only set this on popups/tooltips)
    NoTaskBarIcon           ,   // Platform window: Disable platform task bar icon (generally set on popups/tooltips, or all windows if ImGuiConfigFlags_ViewportsNoTaskBarIcon is set)
    NoFocusOnAppearing      ,   // Platform window: Don't take focus when created.
    NoFocusOnClick          ,   // Platform window: Don't take focus when clicked on.
    NoInputs                ,   // Platform window: Make mouse pass through so we can drag this window while peaking behind it.
    NoRendererClear         ,   // Platform window: Renderer doesn't need to clear the framebuffer ahead (because we will fill it entirely).
    TopMost                 ,   // Platform window: Display on top (for tooltips only).
    Minimized               ,  // Platform window: window is minimized, can skip render. When minimized we tend to avoid using the viewport pos/size for clipping window or testing if they are contained in the viewport.
    NoAutoMerge             ,  // Platform window: Avoid merging this window into another host window. This can only be set via window_class viewport flags override (because we need to now ahead if we are going to create a viewport in the first place!).
    CanHostOtherWindows      = 1 << 12   // Main viewport: can host multiple imgui windows (secondary viewports are associated to a single window).
}

impl Default for ViewportFlags {
    fn default() -> Self {
        Self::None
    }
}

// - Currently represents the Platform window created by the application which is hosting our Dear ImGui windows.
// - With multi-viewport enabled, we extend this concept to have multiple active viewports.
// - In the future we will extend this concept further to also represent Platform Monitor and support a "no main platform window" operation mode.
// - About Main Area vs Work Area:
//   - Main Area = entire viewport.
//   - Work Area = entire viewport minus sections used by main menu bars (for platform windows), or by task bar (for platform monitor).
//   - windows are generally trying to stay within the Work Area of their host viewport.
#[derive(Debug,Clone,Default)]
pub struct Viewport
{



    // Helpers
    // Id32             last_name_hash;
    // ImDrawData          DrawDataP;
    // ImDrawData*         draw_data;               // The ImDrawData corresponding to this viewport. valid after Render() and until the next call to NewFrame().
    // ImDrawDataBuilder   DrawDataBuilder;
    // ImDrawList*         DrawLists[2];           // Convenience background (0) and foreground (1) draw lists. We use them to draw software mouser cursor when io.mouse_draw_cursor is set and to draw most debug overlays.
    // ImGuiViewport()     { memset(this, 0, sizeof(*this)); }
    // Our design separate the Renderer and Platform backends to facilitate combining default backends with each others.
    // Platform/Backend Dependent data
    // The library never uses those fields, they are merely storage to facilitate backend implementation.
    // Vector2D              BuildWorkOffsetMax;     // Work Area: Offset being built during current frame. Generally <= 0.0.
    // Vector2D              BuildWorkOffsetMin;     // Work Area: Offset being built during current frame. Generally >= 0.0.
    // Vector2D              LastPos;
    // Vector2D              WorkOffsetMax;          // Work Area: Offset from pos+size to bottom-right corner of Work Area. Generally (0,0) or (0,-status_bar_height).
    // Vector2D              WorkOffsetMin;          // Work Area: Offset from pos to top-left corner of Work Area. Generally (0,0) or (0,+main_menu_bar_height). Work Area is Full Area but without menu-bars/status-bars (so WorkArea always fit inside pos/size!)
    // Vector2D              last_platform_pos;
    // Vector2D              last_platform_size;
    // Vector2D              last_renderer_size;
    // When our create your own backend for a custom engine, it is possible that both Renderer and Platform will be handled
    // Window*        window;                 // Set when the viewport is owned by a window (and ImGuiViewportFlags_CanHostOtherWindows is NOT set)
    // bool                PlatformWindowCreated;
    // by the same system and you may not need to use all the user_data/Handle fields.
    // float               alpha;                  // window opacity (when dragging dockable windows/viewports we make them transparent)
    // float               LastAlpha;
    // int                 DrawListsLastFrame[2];  // Last frame number the background (0) and foreground (1) draw lists were used
    // int                 Idx;
    // pub draw_data_p: DrawData,
    // short               platform_monitor;
    // void*               platform_handle;         // void* for FindViewportByplatform_handle(). (e.g. suggested to use natural platform handle such as HWND, GLFWWindow*, SDL_Window*)
    // void*               platform_handle_raw;      // void* to hold lower-level, platform-native window handle (under Win32 this is expected to be a HWND, unused for other platforms), when using an abstraction layer like GLFW or SDL (where platform_handle would be a SDL_Window*)
    // void*               platform_user_data;       // void* to hold custom data structure for the OS / platform (e.g. windowing info, render context). generally set by your platform_create_window function.
    // void*               renderer_user_data;       // void* to hold custom data structure for the renderer (e.g. swap chain, framebuffers etc.). generally set by your renderer_create_window function.
    // ~ImGuiViewport()    { IM_ASSERT(platform_user_data == None && renderer_user_data == None); }
    //int                 LastFrontMostStampCount;// Last stamp number from when a window hosted by this viewport was made front-most (by comparing this value between two viewport we have an implicit viewport z-order
    //int                 last_frame_active;        // Last frame number this viewport was activated by a window
    pub alpha: f32,
    pub build_work_offset_max: Vector2D,
    pub build_work_offset_min: Vector2D,
    pub dpi_scale: f32,              // 1.0 = 96 DPI = No extra scale.
    pub draw_data: DrawData,
    pub draw_data_builder: DrawDataBuilder,
    pub draw_list_ids: [Id32;2],
    pub draw_lists_last_frame: [usize;2],
    pub flags: HashSet<ViewportFlags>, //ImGuiViewportFlags  flags;                  // See
    pub id: Id32,                   // Unique identifier for the viewport
    pub idx: i32,
    pub last_apha: f32,
    pub last_frame_active: usize,
    pub last_front_most_stamp_count: i32,
    pub last_name_hash: Id32,
    pub last_plaform_pos: Vector2D,
    pub last_platform_size: Vector2D,
    pub last_pos: Vector2D,
    pub last_renderer_size: Vector2D,
    pub parent_viewport_id: Id32,     // (Advanced) 0: no parent. Instruct the platform backend to setup a parent/child relationship between platform windows.
    pub platform_handle: Vec<u8>,
    pub platform_handle_raw: Vec<u8>,
    pub platform_monitor: usize,
    pub platform_request_close: bool,   // Platform window requested closure (e.g. window was moved by the OS / host window manager, e.g. pressing ALT-F4)
    pub platform_request_move: bool,    // Platform window requested move (e.g. window was moved by the OS / host window manager, authoritative position will be OS window position)
    pub platform_request_resize: bool,  // Platform window requested resize (e.g. window was resized by the OS / host window manager, authoritative size will be OS window size)
    pub platform_window_created: bool,
    pub platform_user_data: Vec<u8>,
    pub pos: Vector2D,                    // Main Area: Position of the viewport (Dear ImGui coordinates are the same as OS desktop/native coordinates)
    pub renderer_user_data: Vec<u8>,
    pub size: Vector2D,                   // Main Area: size of the viewport.
    pub window_id: Id32,
    pub work_offset_max: Vector2D,
    pub work_offset_min: Vector2D,
    pub work_pos: Vector2D,                // Work Area: Position of the viewport minus task bars, menus bars, status bars (>= pos)
    pub work_size: Vector2D,               // Work Area: size of the viewport minus task bars, menu bars, status bars (<= size)
}

impl Viewport {
    // Vector2D              get_center() const       { return Vector2D(pos.x + size.x * 0.5, pos.y + size.y * 0.5); }
    // Vector2D              get_work_center() const   { return Vector2D(work_pos.x + work_size.x * 0.5, work_pos.y + work_size.y * 0.5); }
    pub fn get_center(&self) -> Vector2D {
        Vector2D {
            x: self.pos.x + self.size.x * 0.5,
            y: self.pos.y + self.size.y * 0.5
        }
    }

    pub fn get_work_center(&self) -> Vector2D {
        Vector2D {
            x: self.work_pos.x + self.work_size.x * 0.5,
            y: self.work_pos.y + self.work_size.y * 0.5
        }
    }

        //     // Calculate work rect pos/size given a set of offset (we have 1 pair of offset for rect locked from last frame data, and 1 pair for currently building rect)
    //     Vector2D  CalcWorkRectPos(const Vector2D& off_min) const                            { return Vector2D(pos.x + off_min.x, pos.y + off_min.y); }
    //     Vector2D  CalcWorkRectSize(const Vector2D& off_min, const Vector2D& off_max) const    { return Vector2D(ImMax(0.0, size.x - off_min.x + off_max.x), ImMax(0.0, size.y - off_min.y + off_max.y)); }
    //     void    UpdateWorkRect()            { work_pos = CalcWorkRectPos(WorkOffsetMin); work_size = CalcWorkRectSize(WorkOffsetMin, WorkOffsetMax); } // update public fields
    //
    //     // Helpers to retrieve ImRect (we don't need to store BuildWorkRect as every access tend to change it, hence the code asymmetry)
    //     ImRect  GetMainRect() const         { return ImRect(pos.x, pos.y, pos.x + size.x, pos.y + size.y); }
    //     ImRect  GetWorkRect() const         { return ImRect(work_pos.x, work_pos.y, work_pos.x + work_size.x, work_pos.y + work_size.y); }
    //     ImRect  GetBuildWorkRect() const    { Vector2D pos = CalcWorkRectPos(BuildWorkOffsetMin); Vector2D size = CalcWorkRectSize(BuildWorkOffsetMin, BuildWorkOffsetMax); return ImRect(pos.x, pos.y, pos.x + size.x, pos.y + size.y); }
}


/// static void SetupViewportDrawData(ImGuiViewportP* viewport, ImVector<ImDrawList*>* draw_lists)
pub fn setup_viewport_draw_data(g: &mut Context, viewport: &mut Viewport, draw_lists: &Vec<Id32>)
{
    // When minimized, we report draw_data->display_size as zero to be consistent with non-viewport mode,
    // and to allow applications/backends to easily skip rendering.
    // FIXME: Note that we however do NOT attempt to report "zero drawlist / vertices" into the ImDrawData structure.
    // This is because the work has been done already, and its wasted! We should fix that and add optimizations for
    // it earlier in the pipeline, rather than pretend to hide the data at the end of the pipeline.
    // const bool is_minimized = (viewport.flags & ImGuiViewportFlags_Minimized) != 0;
    let is_minimized = viewport.flags.contains(&ViewportFlags::Minimized);

    // ImGuiIO& io = ImGui::GetIO();
    let io = get_io();
    // ImDrawData* draw_data = &viewport->DrawDataP;
    let draw_data: &mut DrawData = &mut viewport.draw_data;
    // viewport.draw_data = draw_data; // Make publicly accessible
    draw_data.valid = true;
    draw_data.draw_lists = if draw_lists.len() > 0 { draw_lists.data } else { vec![] };
    draw_data.cmd_lists_count = draw_lists.size;
    draw_data.total_vtx_count = 0;
    draw_data.total_idx_count = 0;
    draw_data.display_pos = viewport.pos.clone();
    draw_data.display_size = if is_minimized { Vector2D::new(0.0, 0.0) }else { viewport.size.clone() };
    draw_data.FramebufferScale = io.display_frame_buffer_scale; // FIXME-VIEWPORT: This may vary on a per-monitor/viewport basis?
    draw_data.OwnerViewport = viewport;
    // for (int n = 0; n < draw_lists.Size; n += 1)
    for ele in draw_lists {
        // {
        //     ImDrawList* draw_list = draw_lists.Data[n];
        let draw_list = g.draw_list_mut(*ele).unwrap();
        //     draw_list->_PopUnusedDrawCmd();
        draw_list.pop_unused_draw_cmd();
        //     draw_data.total_vtx_count += draw_list->VtxBuffer.Size;
        draw_data.total_vtx_count += draw_list.vtx_buffer.len();
        //     draw_data.total_idx_count += draw_list->idx_buffer.Size;
        draw_data.total_idx_count += draw_list.idx_buffer.len();
        // }
    }
}


// ImGuiViewport* get_main_viewport()
pub fn get_main_viewport(g: &mut Context) -> &mut Viewport
{
    // ImGuiContext& g = *GImGui;
    return &mut g.viewports[0];
}

// FIXME: This leaks access to viewports not listed in platform_io.viewports[]. Problematic? (#4236)
// ImGuiViewport* FindViewportByID(Id32 id)
pub fn find_viewport_by_id(g: &mut Context, id: Id32) -> Option<&mut Viewport>
{
    // ImGuiContext& g = *GImGui;
    // for (int n = 0; n < g.viewports.size; n += 1)
    //     if (g.viewports[n].id == id)
    //         return g.viewports[n];
    // return None;
    g.viewports.iter_mut().find(|x| x.id == id)
}

// ImGuiViewport* FindViewportByplatform_handle(void* platform_handle)
pub fn find_viewport_by_platform_handle(g: &mut Context, platform_handle: Id32) -> Option<&mut Viewport>
{
    // ImGuiContext& g = *GImGui;
    // for (int i = 0; i != g.viewports.size; i += 1)
    //     if (g.viewports[i].platform_handle == platform_handle)
    //         return g.viewports[i];
    // return None;
    g.viewports.iter_mut().find(|x|x.platform_handle == platform_handle)
}

// void SetCurrentViewport(Window* current_window, ImGuiViewportP* viewport)
pub fn set_current_viewport(g: &mut Context, current_window: Option<&mut Window>, viewport: Option<&mut Viewport>)
{
    // ImGuiContext& g = *GImGui;
    // current_window;

    if (viewport.is_some()) {
        viewport.unwrap().last_frame_active = g.frame_count;
    }
    if (g.current_viewport_id == viewport.unwrap().id) {
        return;
    }
    g.current_dpi_scale = if viewport.is_some() { viewport.dpi_scale } else { 1.0 };
    g.current_viewport_id = viewport.unwrap().id;
    //IMGUI_DEBUG_LOG_VIEWPORT("[viewport] SetCurrentViewport changed '%s' 0x%08X\n", current_window ? current_window->name : None, viewport ? viewport->id : 0);

    // Notify platform layer of viewport changes
    // FIXME-DPI: This is only currently used for experimenting with handling of multiple DPI
    if g.current_viewport_id != INVALID_ID && g.platform_io.platform_on_changed_viewport {
        g.platform_io.platform_on_changed_viewport(g.viewport_mut(g.current_viewport_id).unwrap());
    }
}

// void SetWindowViewport(Window* window, ImGuiViewportP* viewport)
pub fn set_window_viewport(g: &mut Context, window: &mut Window, viewport: &mut Viewport)
{
    // Abandon viewport
    let window_viewport = g.viewport_mut(window.viewport_id).unwrap();
    // let viewport_window = g.window_mut(viewport.window_id);
    if window.viewport_owned && window_viewport.window_id == window.id {
        window.viewport.size = Vector2D::new(0.0, 0.0);
    }

    // window.viewport_id = viewport.id;
    window.viewport_id = viewport.id;
    window.viewport_owned = (viewport.window_id == window.id);
}

// static bool GetWindowAlwaysWantOwnViewport(Window* window)
pub fn get_window_always_want_own_viewport(g: &mut Context, window: &mut Window) -> bool
{
    // Tooltips and menus are not automatically forced into their own viewport when the NoMerge flag is set, however the multiplication of viewports makes them more likely to protrude and create their own.
    // ImGuiContext& g = *GImGui;
    if g.io.config_viewports_not_auto_merge || window.window_class.viewport_flags_override_set.contains(&ViewportFlags::NoAutoMerge) {
        if g.config_flags_curr_frame.contains(&ConfigFlags::ViewportsEnable) {
            if !window.dock_is_active {
                // if window.flags & (WindowFlags::ChildWindow | WindowFlags::ChildMenu | WindowFlags::Tooltip) == 0
                if !window.flags.contains(&WindowFlags::ChildWindow) && !window.flags.contains(&WindowFlags::ChildMenu) && !window.flags.contains(&WindowFlags::Tooltip) {
                    if !window.flags.contains(&WindowFlags::Popup) || window.flags.contains(&WindowFlags::Modal) {
                        return true;
                    }
                }
            }
        }
    }
    return false;
}

// static bool update_try_merge_window_into_host_viewport(Window* window, ImGuiViewportP* viewport)
pub fn update_try_merge_window_into_host_viewport(g: &mut Context, window: &mut Window, viewport: &mut Viewport) -> bool {
    // ImGuiContext& g = *GImGui;
    if window.viewport_id == viewport.id {
        return false;
    }
    if !viewport.flags.contains(&ViewportFlags::CanHostOtherWindows) {
        return false;
    }
    if !viewport.flags.contains(&ViewportFlags::Minimized) {
        return false;
    }
    if !viewport.get_main_rect().contains(window.rect()) {
        return false;
    }
    if get_window_always_want_own_viewport(g, window) {
        return false;
    }

    // FIXME: Can't use g.windows_focus_order[] for root windows only as we care about Z order. If we maintained a display_order along with focus_order we could..
    // for (int n = 0; n < g.windows.len(); n += 1)
    for (_, window_behind) in g.windows.iter_mut() {
        // Window* window_behind = g.windows[n];
        if window_behind.id == window.id {
            break;
        }
        if window_behind.was_active && window_behind.viewport_owned && !(window_behind.flags.contains(&WindowFlags::ChildWindow)) {
            if window_behind.viewport.get_main_rect().overlaps(window.rect()) {
                return false;
            }
        }
    }

    // Move to the existing viewport, Move child/hosted windows as well (FIXME-OPT: iterate child)
    // ViewportP* old_viewport = window.viewport;
    let mut old_viewport = g.viewport_mut(window.viewport_id).unwrap();
    if window.viewport_owned {
        // for (int n = 0; n < g.windows.len(); n += 1)
        for (_, win) in g.windows.iter_mut() {
            if win.viewport_id == old_viewport.id {
                set_window_viewport(g, win, viewport);
            }
        }
    }
    set_window_viewport(g, window, viewport);
    bring_window_to_display_front(g, window);
    return true;
}

// FIXME: handle 0 to N host viewports
// static bool UpdateTryMergeWindowIntoHostViewports(Window* window)
pub fn update_try_merge_window_into_host_viewports(g: &mut Context, window: &mut Window) -> bool
{
    // ImGuiContext& g = *GImGui;
    return update_try_merge_window_into_host_viewport(g, window, &mut g.viewports[0]);
}

// Translate Dear ImGui windows when a Host viewport has been moved
// (This additionally keeps windows at the same place when ConfigFlags::ViewportsEnable is toggled!)
// void TranslateWindowsInViewport(ImGuiViewportP* viewport, const Vector2D& old_pos, const Vector2D& new_pos)
pub fn translate_windows_in_viewport(g: &mut Context, viewport: &mut Viewport, old_pos: &Vector2D, new_pos: &Vector2D) {
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(viewport.Window == None && (viewport.flags & ImGuiViewportFlags_CanHostOtherWindows));

    // 1) We test if ConfigFlags::ViewportsEnable was just toggled, which allows us to conveniently
    // translate imgui windows from OS-window-local to absolute coordinates or vice-versa.
    // 2) If it's not going to fit into the new size, keep it at same absolute position.
    // One problem with this is that most Win32 applications doesn't update their render while dragging,
    // and so the window will appear to teleport when releasing the mouse.
    let translate_all_windows = g.config_flags_curr_frame.contains(&ConfigFlags::ViewportsEnable) != g.config_flags_last_frame.contains(&ConfigFlags::ViewportsEnable);
    let mut test_still_fit_rect = Rect::new(old_pos, old_pos + viewport.size);
    let delta_pos = new_pos - old_pos;
    // for (int window_n = 0; window_n < g.windows.len(); window_n += 1)
    for (_, window_n) in g.windows.iter_mut() {
        if translate_all_windows || (window_n.viewport_id == viewport.id && test_still_fit_rect.contains(window_n.rect())) {
            translate_window(window_n, delta_pos);
        }
    }
}

// scale all windows (position, size). Use when e.g. changing DPI. (This is a lossy operation!)
// void ScaleWindowsInViewport(ImGuiViewportP* viewport, float scale)
pub fn scale_windows_in_viewport(g: &mut Context, viewport: &mut Viewport, scale: f32)
{
    // ImGuiContext& g = *GImGui;
    if viewport.window_id
    {
        scale_window(viewport.Window, scale);
    }
    else
    {
        // for (int i = 0; i != g.windows.len(); i += 1)
        for (_, w) in g.windows.iter_mut()
        {
            if w.viewport_id == viewport.id {
                scale_window(w, scale);
            }
        }
    }
}

// If the backend doesn't set mouse_last_hovered_viewport or doesn't honor ViewportFlags::NoInputs, we do a search ourselves.
// A) It won't take account of the possibility that non-imgui windows may be in-between our dragged window and our target window.
// B) It requires platform_get_window_focus to be implemented by backend.
// ImGuiViewportP* FindHoveredViewportFromPlatformWindowStack(const Vector2D& mouse_platform_pos)
pub fn find_hovered_viewport_from_platform_window_stack(g: &mut Context, mouse_platform_pos: &Vector2D) -> &mut Viewport
{
    // ImGuiContext& g = *GImGui;
    // ViewportP* best_candidate = None;
    let mut best_candidate: &mut Viewport = &mut Viewport::default();
    // for (int n = 0; n < g.viewports.size; n += 1)
    for viewport in g.viewports.iter_mut()
    {
        // ViewportP* viewport = g.viewports[n];
        // if (!(viewport.flags & (ViewportFlags::NoInputs | ViewportFlags::Minimized)) && viewport.get_main_rect().contains(mouse_platform_pos))
        if !(viewport.flags.contains(&ViewportFlags::NoInputs) && !viewport.flags.contains(&ViewportFlags::Minimized)) && viewport.get_main_rect().contains(mouse_platform_pos)
        {
            if best_candidate.id == INVALID_ID || best_candidate.last_frontmost_stamp_count < viewport.last_frontmost_stamp_count {
                best_candidate = viewport;
            }
        }
    }
    return best_candidate;
}

// update viewports and monitor infos
// Note that this is running even if 'ConfigFlags::ViewportsEnable' is not set, in order to clear unused viewports (if any) and update monitor info.
// static void update_viewports_new_frame()
pub fn update_viewports_new_frame(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.platform_io.viewports.size <= g.viewports.size);

    // update Minimized status (we need it first in order to decide if we'll apply pos/size of the main viewport)
    let viewports_enabled = ( g.config_flags_curr_frame & ConfigFlags::ViewportsEnable) != 0;
    if (viewports_enabled)
    {
        for (int n = 0; n < g.viewports.size; n += 1)
        {
            ViewportP* viewport = g.viewports[n];
            let platform_funcs_available = viewport.platform_window_created;
            if (g.platform_io.platform_get_window_minimized && platform_funcs_available)
            {
                bool minimized = g.platform_io.platform_get_window_minimized(viewport);
                if (minimized)
                    viewport.flags |= ViewportFlags::Minimized;
                else
                    viewport.flags &= ~ViewportFlags::Minimized;
            }
        }
    }

    // Create/update main viewport with current platform position.
    // FIXME-VIEWPORT: size is driven by backend/user code for backward-compatibility but we should aim to make this more consistent.
    ViewportP* main_viewport = g.viewports[0];
    // IM_ASSERT(main_viewport.id == IMGUI_VIEWPORT_DEFAULT_ID);
    // IM_ASSERT(main_viewport.Window == None);
    Vector2D main_viewport_pos = viewports_enabled ? g.platform_io.platform_get_window_pos(main_viewport) : Vector2D::new(0.0, 0.0);
    Vector2D main_viewport_size = g.io.display_size;
    if (viewports_enabled && (main_viewport.flags & ViewportFlags::Minimized))
    {
        main_viewport_pos = main_viewport.pos;    // Preserve last pos/size when minimized (FIXME: We don't do the same for size outside of the viewport path)
        main_viewport_size = main_viewport.size;
    }
    AddUpdateViewport(None, IMGUI_VIEWPORT_DEFAULT_ID, main_viewport_pos, main_viewport_size, ViewportFlags::OwnedByApp | ViewportFlags::CanHostOtherWindows);

    g.current_dpi_scale = 0.0;
    g.current_viewport = None;
    g.mouse_viewport = None;
    for (int n = 0; n < g.viewports.size; n += 1)
    {
        ViewportP* viewport = g.viewports[n];
        viewport.Idx = n;

        // Erase unused viewports
        if (n > 0 && viewport.last_frame_active < g.frame_count - 2)
        {
            DestroyViewport(viewport);
            n -= 1 ;
            continue;
        }

        let platform_funcs_available = viewport.platform_window_created;
        if (viewports_enabled)
        {
            // update Position and size (from Platform window to ImGui) if requested.
            // We do it early in the frame instead of waiting for UpdatePlatformWindows() to avoid a frame of lag when moving/resizing using OS facilities.
            if (!(viewport.flags & ViewportFlags::Minimized) && platform_funcs_available)
            {
                // viewport->work_pos and work_size will be updated below
                if (viewport.platform_request_move)
                    viewport.pos = viewport.last_platform_pos = g.platform_io.platform_get_window_pos(viewport);
                if (viewport.platform_requsest_resize)
                    viewport.size = viewport.last_platform_size = g.platform_io.platform_get_window_size(viewport);
            }
        }

        // update/copy monitor info
        UpdateViewportplatform_monitor(viewport);

        // Lock down space taken by menu bars and status bars, reset the offset for functions like BeginMainMenuBar() to alter them again.
        viewport.WorkOffsetMin = viewport.BuildWorkOffsetMin;
        viewport.WorkOffsetMax = viewport.BuildWorkOffsetMax;
        viewport.BuildWorkOffsetMin = viewport.BuildWorkOffsetMax = Vector2D::new(0.0, 0.0);
        viewport.update_work_rect();

        // Reset alpha every frame. Users of transparency (docking) needs to request a lower alpha back.
        viewport.alpha = 1.0;

        // Translate Dear ImGui windows when a Host viewport has been moved
        // (This additionally keeps windows at the same place when ConfigFlags::ViewportsEnable is toggled!)
        const Vector2D viewport_delta_pos = viewport.pos - viewport.LastPos;
        if ((viewport.flags & ViewportFlags::CanHostOtherWindows) && (viewport_delta_pos.x != 0.0 || viewport_delta_pos.y != 0.0))
            TranslateWindowsInViewport(viewport, viewport.LastPos, viewport.pos);

        // update DPI scale
        float new_dpi_scale;
        if (g.platform_io.platform_get_window_dpi_scale && platform_funcs_available)
            new_dpi_scale = g.platform_io.platform_get_window_dpi_scale(viewport);
        else if (viewport.platform_monitor != -1)
            new_dpi_scale = g.platform_io.monitors[viewport.platform_monitor].DpiScale;
        else
            new_dpi_scale = (viewport.DpiScale != 0.0) ? viewport.DpiScale : 1.0;
        if (viewport.DpiScale != 0.0 && new_dpi_scale != viewport.DpiScale)
        {
            let scale_factor =  new_dpi_scale / viewport.DpiScale;
            if (g.io.config_flags & ConfigFlags::DpiEnableScaleViewports)
                ScaleWindowsInViewport(viewport, scale_factor);
            //if (viewport == get_main_viewport())
            //    g.PlatformInterface.set_window_size(viewport, viewport->size * scale_factor);

            // scale our window moving pivot so that the window will rescale roughly around the mouse position.
            // FIXME-VIEWPORT: This currently creates a resizing feedback loop when a window is straddling a DPI transition border.
            // (Minor: since our sizes do not perfectly linearly scale, deferring the click offset scale until we know the actual window scale ratio may get us slightly more precise mouse positioning.)
            //if (g.moving_window != None && g.moving_window->viewport == viewport)
            //    g.active_id_click_offset = f32::floor(g.active_id_click_offset * scale_factor);
        }
        viewport.DpiScale = new_dpi_scale;
    }

    // update fallback monitor
    if (g.platform_io.monitors.size == 0)
    {
        platform_monitor* monitor = &g.FallbackMonitor;
        monitor.MainPos = main_viewport.pos;
        monitor.MainSize = main_viewport.size;
        monitor.WorkPos = main_viewport.WorkPos;
        monitor.work_size = main_viewport.work_size;
        monitor.DpiScale = main_viewport.DpiScale;
    }

    if (!viewports_enabled)
    {
        g.mouse_viewport = main_viewport;
        return;
    }

    // Mouse handling: decide on the actual mouse viewport for this frame between the active/focused viewport and the hovered viewport.
    // Note that 'viewport_hovered' should skip over any viewport that has the ViewportFlags::NoInputs flags set.
    ViewportP* viewport_hovered = None;
    if (g.io.backend_flags & ImGuiBackendFlags_HasMouseHoveredViewport)
    {
        viewport_hovered = g.io.MouseHoveredViewport ? (ViewportP*)FindViewportByID(g.io.MouseHoveredViewport) : None;
        if (viewport_hovered && (viewport_hovered.flags & ViewportFlags::NoInputs))
            viewport_hovered = FindHoveredViewportFromPlatformWindowStack(g.io.mouse_pos); // Backend failed to handle _NoInputs viewport: revert to our fallback.
    }
    else
    {
        // If the backend doesn't know how to honor ViewportFlags::NoInputs, we do a search ourselves. Note that this search:
        // A) won't take account of the possibility that non-imgui windows may be in-between our dragged window and our target window.
        // B) won't take account of how the backend apply parent<>child relationship to secondary viewports, which affects their Z order.
        // C) uses LastFrameAsRefViewport as a flawed replacement for the last time a window was focused (we could/should fix that by introducing Focus functions in platform_io)
        viewport_hovered = FindHoveredViewportFromPlatformWindowStack(g.io.mouse_pos);
    }
    if (viewport_hovered != None)
        g.mouse_last_hovered_viewport = viewport_hovered;
    else if (g.mouse_last_hovered_viewport == None)
        g.mouse_last_hovered_viewport = g.viewports[0];

    // update mouse reference viewport
    // (when moving a window we aim at its viewport, but this will be overwritten below if we go in drag and drop mode)
    // (MovingViewport->viewport will be None in the rare situation where the window disappared while moving, set UpdateMouseMovingWindowNewFrame() for details)
    if (g.moving_window && g.moving_window.viewport)
        g.mouse_viewport = g.moving_window.viewport;
    else
        g.mouse_viewport = g.mouse_last_hovered_viewport;

    // When dragging something, always refer to the last hovered viewport.
    // - when releasing a moving window we will revert to aiming behind (at viewport_hovered)
    // - when we are between viewports, our dragged preview will tend to show in the last viewport _even_ if we don't have tooltips in their viewports (when lacking monitor info)
    // - consider the case of holding on a menu item to browse child menus: even thou a mouse button is held, there's no active id because menu items only react on mouse release.
    // FIXME-VIEWPORT: This is essentially broken, when ImGuiBackendFlags_HasMouseHoveredViewport is set we want to trust when viewport_hovered==None and use that.
    let is_mouse_dragging_with_an_expected_destination = g.drag_drop_active;
    if (is_mouse_dragging_with_an_expected_destination && viewport_hovered == None)
        viewport_hovered = g.mouse_last_hovered_viewport;
    if (is_mouse_dragging_with_an_expected_destination || g.active_id == 0 || !IsAnyMouseDown())
        if (viewport_hovered != None && viewport_hovered != g.mouse_viewport && !(viewport_hovered.flags & ViewportFlags::NoInputs))
            g.mouse_viewport = viewport_hovered;

    // IM_ASSERT(g.mouse_viewport != None);
}

// update user-facing viewport list (g.viewports -> g.platform_io.viewports after filtering out some)
// static void update_viewports_end_frame()
pub fn update_viewports_ends_frame(g: &mut Context)
{
    // ImGuiContext& g = *GImGui;
    g.platform_io.viewports.resize(0);
    for (int i = 0; i < g.viewports.size; i += 1)
    {
        ViewportP* viewport = g.viewports[i];
        viewport.LastPos = viewport.pos;
        if (viewport.last_frame_active < g.frame_count || viewport.size.x <= 0.0 || viewport.size.y <= 0.0)
            if (i > 0) // Always include main viewport in the list
                continue;
        if (viewport.Window && !is_window_active_and_visible(viewport.Window))
            continue;
        if (i > 0)
            // IM_ASSERT(viewport.Window != None);
        g.platform_io.viewports.push_back(viewport);
    }
    g.viewports[0].clear_request_flags(); // clear main viewport flags because UpdatePlatformWindows() won't do it and may not even be called
}

// FIXME: We should ideally refactor the system to call this every frame (we currently don't)
// ImGuiViewportP* AddUpdateViewport(Window* window, Id32 id, const Vector2D& pos, const Vector2D& size, ImGuiViewportFlags flags)
pub fn add_update_viewport(g: &mut Context, window: &mut Window, id: Id32, pos: &Vector2D, size: &Vector2D, flags: &HashSet<ViewportFlags>)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(id != 0);

    flags |= ViewportFlags::IsPlatformWindow;
    if (window != None)
    {
        if (g.moving_window && g.moving_window.root_window_dock_tree == window)
            flags |= ViewportFlags::NoInputs | ViewportFlags::NoFocusOnAppearing;
        if ((window.flags & WindowFlags::NoMouseInputs) && (window.flags & WindowFlags::NoNavInputs))
            flags |= ViewportFlags::NoInputs;
        if (window.flags & WindowFlags::NoFocusOnAppearing)
            flags |= ViewportFlags::NoFocusOnAppearing;
    }

    ViewportP* viewport = (ViewportP*)FindViewportByID(id);
    if (viewport)
    {
        // Always update for main viewport as we are already pulling correct platform pos/size (see #4900)
        if (!viewport.platform_request_move || viewport.id == IMGUI_VIEWPORT_DEFAULT_ID)
            viewport.pos = pos;
        if (!viewport.platform_requsest_resize || viewport.id == IMGUI_VIEWPORT_DEFAULT_ID)
            viewport.size = size;
        viewport.flags = flags | (viewport.flags & ViewportFlags::Minimized); // Preserve existing flags
    }
    else
    {
        // New viewport
        viewport = IM_NEW(ViewportP)();
        viewport.id = id;
        viewport.Idx = g.viewports.size;
        viewport.pos = viewport.LastPos = pos;
        viewport.size = size;
        viewport.flags = flags;
        UpdateViewportplatform_monitor(viewport);
        g.viewports.push_back(viewport);
        IMGUI_DEBUG_LOG_VIEWPORT("[viewport] Add viewport %08X '%s'\n", id, window ? window.name : "<None>");

        // We normally setup for all viewports in NewFrame() but here need to handle the mid-frame creation of a new viewport.
        // We need to extend the fullscreen clip rect so the OverlayDrawList clip is correct for that the first frame
        g.draw_list_shared_data.clip_rect_full_screen.x = ImMin(g.draw_list_shared_data.clip_rect_full_screen.x, viewport.pos.x);
        g.draw_list_shared_data.clip_rect_full_screen.y = ImMin(g.draw_list_shared_data.clip_rect_full_screen.y, viewport.pos.y);
        g.draw_list_shared_data.clip_rect_full_screen.z = ImMax(g.draw_list_shared_data.clip_rect_full_screen.z, viewport.pos.x + viewport.size.x);
        g.draw_list_shared_data.clip_rect_full_screen.w = ImMax(g.draw_list_shared_data.clip_rect_full_screen.w, viewport.pos.y + viewport.size.y);

        // Store initial dpi_scale before the OS platform window creation, based on expected monitor data.
        // This is so we can select an appropriate font size on the first frame of our window lifetime
        if (viewport.platform_monitor != -1)
            viewport.DpiScale = g.platform_io.monitors[viewport.platform_monitor].DpiScale;
    }

    viewport.Window = window;
    viewport.last_frame_active = g.frame_count;
    viewport.update_work_rect();
    // IM_ASSERT(window == None || viewport.id == window.id);

    if (window != None)
        window.viewport_owned = true;

    return viewport;
}

// static void DestroyViewport(ImGuiViewportP* viewport)
pub fn destroy_viewport(g: &mut Context, viewport: &mut Viewport)
{
    // clear references to this viewport in windows (window->viewport_id becomes the master data)
    // ImGuiContext& g = *GImGui;
    for (int window_n = 0; window_n < g.windows.len(); window_n += 1)
    {
        Window* window = g.windows[window_n];
        if (window.viewport != viewport)
            continue;
        window.viewport = None;
        window.viewport_owned = false;
    }
    if (viewport == g.mouse_last_hovered_viewport)
        g.mouse_last_hovered_viewport = None;

    // Destroy
    IMGUI_DEBUG_LOG_VIEWPORT("[viewport] Delete viewport %08X '%s'\n", viewport.id, viewport.Window ? viewport.Window.name : "n/a");
    destroy_platform_window(viewport); // In most circumstances the platform window will already be destroyed here.
    // IM_ASSERT(g.platform_io.viewports.contains(viewport) == false);
    // IM_ASSERT(g.viewports[viewport.Idx] == viewport);
    g.viewports.erase(g.viewports.data + viewport.Idx);
    IM_DELETE(viewport);
}

// FIXME-VIEWPORT: This is all super messy and ought to be clarified or rewritten.
// static void WindowSelectViewport(Window* window)
pub fn WindowSelectViewport(g: &mut Context, window: &mut Window)
{
    // ImGuiContext& g = *GImGui;
    WindowFlags flags = window.flags;
    window.viewport_allow_platform_monitor_extend = -1;

    // Restore main viewport if multi-viewport is not supported by the backend
    ViewportP* main_viewport = (ViewportP*)(void*)get_main_viewport();
    if (!(g.config_flags_curr_frame & ConfigFlags::ViewportsEnable))
    {
        set_window_viewport(window, main_viewport);
        return;
    }
    window.viewport_owned = false;

    // appearing popups reset their viewport so they can inherit again
    if ((flags & (WindowFlags::Popup | WindowFlags::Tooltip)) && window.Appearing)
    {
        window.viewport = None;
        window.viewport_id = INVALID_ID;
    }

    if ((g.next_window_data.flags & NextWindowDataFlags::HasViewport) == 0)
    {
        // By default inherit from parent window
        if (window.viewport == None && window.parent_window && (!window.parent_window.is_fallback_window || window.parent_window.WasActive))
            window.viewport = window.parent_window.viewport;

        // Attempt to restore saved viewport id (= window that hasn't been activated yet), try to restore the viewport based on saved 'window->viewport_pos' restored from .ini file
        if (window.viewport == None && window.viewport_id != 0)
        {
            window.viewport = (ViewportP*)FindViewportByID(window.viewport_id);
            if (window.viewport == None && window.viewport_pos.x != f32::MAX && window.viewport_pos.y != f32::MAX)
                window.viewport = AddUpdateViewport(window, window.id, window.viewport_pos, window.size, ViewportFlags::None);
        }
    }

    bool lock_viewport = false;
    if (g.next_window_data.flags & NextWindowDataFlags::HasViewport)
    {
        // Code explicitly request a viewport
        window.viewport = (ViewportP*)FindViewportByID(g.next_window_data.viewport_id);
        window.viewport_id = g.next_window_data.viewport_id; // Store id even if viewport isn't resolved yet.
        lock_viewport = true;
    }
    else if ((flags & WindowFlags::ChildWindow) || (flags & WindowFlags::ChildMenu))
    {
        // Always inherit viewport from parent window
        if (window.dock_node_id && window.dock_node_id.host_window_id)
            // IM_ASSERT(window.dock_node.host_window.viewport == window.parent_window.viewport);
        window.viewport = window.parent_window.viewport;
    }
    else if (window.dock_node_id && window.dock_node_id.host_window_id)
    {
        // This covers the "always inherit viewport from parent window" case for when a window reattach to a node that was just created mid-frame
        window.viewport = window.dock_node_id.host_window_id.viewport;
    }
    else if (flags & WindowFlags::Tooltip)
    {
        window.viewport = g.mouse_viewport;
    }
    else if (get_window_always_want_own_viewport(window))
    {
        window.viewport = AddUpdateViewport(window, window.id, window.pos, window.size, ViewportFlags::None);
    }
    else if (g.moving_window && g.moving_window.root_window_dock_tree == window && is_mouse_pos_valid())
    {
        if (window.viewport != None && window.viewport.Window == window)
            window.viewport = AddUpdateViewport(window, window.id, window.pos, window.size, ViewportFlags::None);
    }
    else
    {
        // merge into host viewport?
        // We cannot test window->viewport_owned as it set lower in the function.
        // Testing (g.active_id == 0 || g.active_id_allow_overlap) to avoid merging during a short-term widget interaction. Main intent was to avoid during resize (see #4212)
        bool try_to_merge_into_host_viewport = (window.viewport && window == window.viewport.Window && (g.active_id == 0 || g.active_id_allow_overlap));
        if (try_to_merge_into_host_viewport)
            UpdateTryMergeWindowIntoHostViewports(window);
    }

    // Fallback: merge in default viewport if z-order matches, otherwise create a new viewport
    if (window.viewport == None)
        if (!update_try_merge_window_into_host_viewport(window, main_viewport))
            window.viewport = AddUpdateViewport(window, window.id, window.pos, window.size, ViewportFlags::None);

    // Mark window as allowed to protrude outside of its viewport and into the current monitor
    if (!lock_viewport)
    {
        if (flags & (WindowFlags::Tooltip | WindowFlags::Popup))
        {
            // We need to take account of the possibility that mouse may become invalid.
            // Popups/Tooltip always set viewport_allow_platform_monitor_extend so GetWindowAllowedExtentRect() will return full monitor bounds.
            Vector2D mouse_ref = (flags & WindowFlags::Tooltip) ? g.io.mouse_pos : g.begin_popup_stack.back().OpenMousePos;
            bool use_mouse_ref = (g.nav_disable_highlight || !g.nav_disable_mouse_hover || !g.nav_window);
            bool mouse_valid = is_mouse_pos_valid(&mouse_ref);
            if ((window.Appearing || (flags & (WindowFlags::Tooltip | WindowFlags::ChildMenu))) && (!use_mouse_ref || mouse_valid))
                window.viewport_allow_platform_monitor_extend = Findplatform_monitorForPos((use_mouse_ref && mouse_valid) ? mouse_ref : nav_calc_preferred_ref_pos());
            else
                window.viewport_allow_platform_monitor_extend = window.viewport.platform_monitor;
        }
        else if (window.viewport && window != window.viewport.Window && window.viewport.Window && !(flags & WindowFlags::ChildWindow) && window.dock_node_id == None)
        {
            // When called from Begin() we don't have access to a proper version of the hidden flag yet, so we replicate this code.
            let will_be_visible = (window.dock_is_active && !window.dock_tab_is_visible) ? false : true;
            if ((window.flags & WindowFlags::DockNodeHost) && window.viewport.last_frame_active < g.frame_count && will_be_visible)
            {
                // Steal/transfer ownership
                IMGUI_DEBUG_LOG_VIEWPORT("[viewport] window '%s' steal viewport %08X from window '%s'\n", window.name, window.viewport.id, window.viewport.Window.name);
                window.viewport.Window = window;
                window.viewport.id = window.id;
                window.viewport.last_name_hash = 0;
            }
            else if (!UpdateTryMergeWindowIntoHostViewports(window)) // merge?
            {
                // New viewport
                window.viewport = AddUpdateViewport(window, window.id, window.pos, window.size, ViewportFlags::NoFocusOnAppearing);
            }
        }
        else if (window.viewport_allow_platform_monitor_extend < 0 && (flags & WindowFlags::ChildWindow) == 0)
        {
            // Regular (non-child, non-popup) windows by default are also allowed to protrude
            // Child windows are kept contained within their parent.
            window.viewport_allow_platform_monitor_extend = window.viewport.platform_monitor;
        }
    }

    // update flags
    window.viewport_owned = (window == window.viewport.Window);
    window.viewport_id = window.viewport.id;

    // If the OS window has a title bar, hide our imgui title bar
    //if (window->viewport_owned && !(window->viewport->flags & ImGuiViewportFlags_NoDecoration))
    //    window->flags |= WindowFlags::NoTitleBar;
}

// void WindowSyncOwnedViewport(Window* window, Window* parent_window_in_stack)
pub fn window_sync_owned_viewport(g: &mut Context, window: &mut Window, parent_window_in_stack: &mut Window)
{
    // ImGuiContext& g = *GImGui;

    bool viewport_rect_changed = false;

    // Synchronize window --> viewport in most situations
    // Synchronize viewport -> window in case the platform window has been moved or resized from the OS/WM
    if (window.viewport.platform_request_move)
    {
        window.pos = window.viewport.pos;
        mark_ini_settings_dirty(window);
    }
    else if (memcmp(&window.viewport.pos, &window.pos, sizeof(window.pos)) != 0)
    {
        viewport_rect_changed = true;
        window.viewport.pos = window.pos;
    }

    if (window.viewport.platform_requsest_resize)
    {
        window.size = window.size_full = window.viewport.size;
        mark_ini_settings_dirty(window);
    }
    else if (memcmp(&window.viewport.size, &window.size, sizeof(window.size)) != 0)
    {
        viewport_rect_changed = true;
        window.viewport.size = window.size;
    }
    window.viewport.update_work_rect();

    // The viewport may have changed monitor since the global update in UpdateViewportsNewFrame()
    // Either a set_next_window_pos() call in the current frame or a set_window_pos() call in the previous frame may have this effect.
    if (viewport_rect_changed)
        UpdateViewportplatform_monitor(window.viewport);

    // update common viewport flags
    const ViewportFlags viewport_flags_to_clear = ViewportFlags::TopMost | ViewportFlags::NoTaskBarIcon | ViewportFlags::NoDecoration | ViewportFlags::NoRendererClear;
    ViewportFlags viewport_flags = window.viewport.flags & ~viewport_flags_to_clear;
    WindowFlags window_flags = window.flags;
    let is_modal = (window_flags & WindowFlags::Modal) != 0;
    let is_short_lived_floating_window = (window_flags & (WindowFlags::ChildMenu | WindowFlags::Tooltip | WindowFlags::Popup)) != 0;
    if (window_flags & WindowFlags::Tooltip)
        viewport_flags |= ViewportFlags::TopMost;
    if ((g.io.ConfigViewportsNoTaskBarIcon || is_short_lived_floating_window) && !is_modal)
        viewport_flags |= ViewportFlags::NoTaskBarIcon;
    if (g.io.ConfigViewportsNoDecoration || is_short_lived_floating_window)
        viewport_flags |= ViewportFlags::NoDecoration;

    // Not correct to set modal as topmost because:
    // - Because other popups can be stacked above a modal (e.g. combo box in a modal)
    // - ImGuiViewportFlags_TopMost is currently handled different in backends: in Win32 it is "appear top most" whereas in GLFW and SDL it is "stay topmost"
    //if (flags & WindowFlags_Modal)
    //    viewport_flags |= ImGuiViewportFlags_TopMost;

    // For popups and menus that may be protruding out of their parent viewport, we enable _NoFocusOnClick so that clicking on them
    // won't steal the OS focus away from their parent window (which may be reflected in OS the title bar decoration).
    // Setting _NoFocusOnClick would technically prevent us from bringing back to front in case they are being covered by an OS window from a different app,
    // but it shouldn't be much of a problem considering those are already popups that are closed when clicking elsewhere.
    if (is_short_lived_floating_window && !is_modal)
        viewport_flags |= ViewportFlags::NoFocusOnAppearing | ViewportFlags::NoFocusOnClick;

    // We can overwrite viewport flags using window_class (advanced users)
    if (window.window_class.viewport_flags_override_set)
        viewport_flags |= window.window_class.viewport_flags_override_set;
    if (window.window_class.viewportFlagsOverrideClear)
        viewport_flags &= ~window.window_class.viewportFlagsOverrideClear;

    // We can also tell the backend that clearing the platform window won't be necessary,
    // as our window background is filling the viewport and we have disabled BgAlpha.
    // FIXME: Work on support for per-viewport transparency (#2766)
    if (!(window_flags & WindowFlags::NoBackground))
        viewport_flags |= ViewportFlags::NoRendererClear;

    window.viewport.flags = viewport_flags;

    // update parent viewport id
    // (the !is_fallback_window test mimic the one done in WindowSelectViewport())
    if (window.window_class.ParentViewportId != (Id32)-1)
        window.viewport.ParentViewportId = window.window_class.ParentViewportId;
    else if ((window_flags & (WindowFlags::Popup | WindowFlags::Tooltip)) && parent_window_in_stack && (!parent_window_in_stack.is_fallback_window || parent_window_in_stack.WasActive))
        window.viewport.ParentViewportId = parent_window_in_stack.viewport.id;
    else
        window.viewport.ParentViewportId = g.io.ConfigViewportsNoDefaultParent ? 0 : IMGUI_VIEWPORT_DEFAULT_ID;
}
