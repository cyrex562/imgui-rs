use std::collections::HashSet;
use crate::draw_data::DrawData;
use crate::draw_data_builder::DrawDataBuilder;
use crate::draw_list::DrawList;
use crate::types::Id32;
use crate::vectors::Vector2D;

/// ImGuiViewport Private/Internals fields (cardinal sin: we are using inheritance!)
/// Every instance of ImGuiViewport is in fact a ImGuiViewportP.
// #[derive(Debug,Clone,Default)]
// pub struct ViewportP
// {
//     /
// }
//
// impl ViewportP {
//     // // ImGuiViewportP()                    { Idx = -1; last_frame_active = DrawListsLastFrame[0] = DrawListsLastFrame[1] = LastFrontMostStampCount = -1; LastNameHash = 0; Alpha = LastAlpha = 1.0; PlatformMonitor = -1; PlatformWindowCreated = false; Window = NULL; DrawLists[0] = DrawLists[1] = NULL; LastPlatformPos = LastPlatformSize = LastRendererSize = Vector2D(FLT_MAX, FLT_MAX); }
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
//     //     // void    ClearRequestFlags()         { platform_request_close = platform_request_move = platform_request_resize = false; }
//     //
//
// }

// flags stored in ImGuiViewport::flags, giving indications to the platform backends.
#[derive(Debug,Clone)]
pub enum ViewportFlags
{
    None                     = 0,
    IsPlatformWindow         = 1 << 0,   // Represent a Platform Window
    IsPlatformMonitor        = 1 << 1,   // Represent a Platform Monitor (unused yet)
    OwnedByApp               = 1 << 2,   // Platform Window: is created/managed by the application (rather than a dear imgui backend)
    NoDecoration             = 1 << 3,   // Platform Window: Disable platform decorations: title bar, borders, etc. (generally set all windows, but if ImGuiConfigFlags_ViewportsDecoration is set we only set this on popups/tooltips)
    NoTaskBarIcon            = 1 << 4,   // Platform Window: Disable platform task bar icon (generally set on popups/tooltips, or all windows if ImGuiConfigFlags_ViewportsNoTaskBarIcon is set)
    NoFocusOnAppearing       = 1 << 5,   // Platform Window: Don't take focus when created.
    NoFocusOnClick           = 1 << 6,   // Platform Window: Don't take focus when clicked on.
    NoInputs                 = 1 << 7,   // Platform Window: Make mouse pass through so we can drag this window while peaking behind it.
    NoRendererClear          = 1 << 8,   // Platform Window: Renderer doesn't need to clear the framebuffer ahead (because we will fill it entirely).
    TopMost                  = 1 << 9,   // Platform Window: Display on top (for tooltips only).
    Minimized                = 1 << 10,  // Platform Window: Window is minimized, can skip render. When minimized we tend to avoid using the viewport pos/size for clipping window or testing if they are contained in the viewport.
    NoAutoMerge              = 1 << 11,  // Platform Window: Avoid merging this window into another host window. This can only be set via ImGuiWindowClass viewport flags override (because we need to now ahead if we are going to create a viewport in the first place!).
    CanHostOtherWindows      = 1 << 12   // Main viewport: can host multiple imgui windows (secondary viewports are associated to a single window).
}

impl Default for ViewportFlags {
    fn default() -> Self {
        Self::None
    }
}

// - Currently represents the Platform Window created by the application which is hosting our Dear ImGui windows.
// - With multi-viewport enabled, we extend this concept to have multiple active viewports.
// - In the future we will extend this concept further to also represent Platform Monitor and support a "no main platform window" operation mode.
// - About Main Area vs Work Area:
//   - Main Area = entire viewport.
//   - Work Area = entire viewport minus sections used by main menu bars (for platform windows), or by task bar (for platform monitor).
//   - windows are generally trying to stay within the Work Area of their host viewport.
#[derive(Debug,Clone,Default)]
pub struct Viewport
{
    pub id: Id32,                   // Unique identifier for the viewport
    pub flags: HashSet<ViewportFlags>, //ImGuiViewportFlags  flags;                  // See
    pub pos: Vector2D,                    // Main Area: Position of the viewport (Dear ImGui coordinates are the same as OS desktop/native coordinates)
    pub size: Vector2D,                   // Main Area: size of the viewport.
    pub work_pos: Vector2D,                // Work Area: Position of the viewport minus task bars, menus bars, status bars (>= pos)
    pub work_size: Vector2D,               // Work Area: size of the viewport minus task bars, menu bars, status bars (<= size)
    pub dpi_scale: f32,              // 1.0 = 96 DPI = No extra scale.
    pub parent_viewport_id: Id32,     // (Advanced) 0: no parent. Instruct the platform backend to setup a parent/child relationship between platform windows.
    // ImDrawData*         draw_data;               // The ImDrawData corresponding to this viewport. valid after Render() and until the next call to NewFrame().
    pub draw_data: DrawData,

    // Platform/Backend Dependent data
    // Our design separate the Renderer and Platform backends to facilitate combining default backends with each others.
    // When our create your own backend for a custom engine, it is possible that both Renderer and Platform will be handled
    // by the same system and you may not need to use all the user_data/Handle fields.
    // The library never uses those fields, they are merely storage to facilitate backend implementation.
    // void*               renderer_user_data;       // void* to hold custom data structure for the renderer (e.g. swap chain, framebuffers etc.). generally set by your Renderer_CreateWindow function.
    pub renderer_user_data: Vec<u8>,
    // void*               PlatformUserData;       // void* to hold custom data structure for the OS / platform (e.g. windowing info, render context). generally set by your Platform_CreateWindow function.
    pub platformuser_data: Vec<u8>,
    // void*               platform_handle;         // void* for FindViewportByPlatformHandle(). (e.g. suggested to use natural platform handle such as HWND, GLFWWindow*, SDL_Window*)
    pub platform_handle: Vec<u8>,
    // void*               platform_handle_raw;      // void* to hold lower-level, platform-native window handle (under Win32 this is expected to be a HWND, unused for other platforms), when using an abstraction layer like GLFW or SDL (where platform_handle would be a SDL_Window*)
    pub platform_handle_raw: Vec<u8>,
    pub platform_request_move: bool,    // Platform window requested move (e.g. window was moved by the OS / host window manager, authoritative position will be OS window position)
    pub platform_request_resize: bool,  // Platform window requested resize (e.g. window was resized by the OS / host window manager, authoritative size will be OS window size)
    pub platform_request_close: bool,   // Platform window requested closure (e.g. window was moved by the OS / host window manager, e.g. pressing ALT-F4)

    // ImGuiViewport()     { memset(this, 0, sizeof(*this)); }
    // ~ImGuiViewport()    { IM_ASSERT(PlatformUserData == NULL && renderer_user_data == NULL); }

    // Helpers
    // int                 Idx;
    pub idx: i32,
    //int                 last_frame_active;        // Last frame number this viewport was activated by a window
    pub last_frame_active: i32,
    //int                 LastFrontMostStampCount;// Last stamp number from when a window hosted by this viewport was made front-most (by comparing this value between two viewport we have an implicit viewport z-order
    pub last_front_most_stamp_count: i32,
    // ImGuiID             LastNameHash;
    pub last_name_hash: Id32,
    // Vector2D              LastPos;
    pub last_pos: Vector2D,
    // float               Alpha;                  // Window opacity (when dragging dockable windows/viewports we make them transparent)
    pub alpha: f32,
    // float               LastAlpha;
    pub last_apha: f32,
    // short               PlatformMonitor;
    pub platform_monitor: i16,
    // bool                PlatformWindowCreated;
    pub platform_window_created: bool,
    // ImGuiWindow*        Window;                 // Set when the viewport is owned by a window (and ImGuiViewportFlags_CanHostOtherWindows is NOT set)
    pub window: Id32,
    // int                 DrawListsLastFrame[2];  // Last frame number the background (0) and foreground (1) draw lists were used
    pub draw_lists_last_frame: [i32;2],
    // ImDrawList*         DrawLists[2];           // Convenience background (0) and foreground (1) draw lists. We use them to draw software mouser cursor when io.mouse_draw_cursor is set and to draw most debug overlays.
    pub draw_lists: [Id32;2],
    // ImDrawData          DrawDataP;
    pub draw_data_p: DrawData,
    // ImDrawDataBuilder   DrawDataBuilder;
    pub draw_data_builder: DrawDataBuilder,
    // Vector2D              LastPlatformPos;
    pub last_plaform_pos: Vector2D,
    // Vector2D              LastPlatformSize;
    pub last_platform_size: Vector2D,
    // Vector2D              LastRendererSize;
    pub last_renderer_size: Vector2D,
    // Vector2D              WorkOffsetMin;          // Work Area: Offset from pos to top-left corner of Work Area. Generally (0,0) or (0,+main_menu_bar_height). Work Area is Full Area but without menu-bars/status-bars (so WorkArea always fit inside pos/size!)
    pub work_offset_min: Vector2D,
    // Vector2D              WorkOffsetMax;          // Work Area: Offset from pos+size to bottom-right corner of Work Area. Generally (0,0) or (0,-status_bar_height).
    pub work_offset_max: Vector2D,
    // Vector2D              BuildWorkOffsetMin;     // Work Area: Offset being built during current frame. Generally >= 0.0.
    pub build_work_offset_min: Vector2D,
    // Vector2D              BuildWorkOffsetMax;     // Work Area: Offset being built during current frame. Generally <= 0.0.
    pub build_work_offset_max: Vector2D,
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
    //     void    UpdateWorkRect()            { work_pos = CalcWorkRectPos(WorkOffsetMin); work_size = CalcWorkRectSize(WorkOffsetMin, WorkOffsetMax); } // Update public fields
    //
    //     // Helpers to retrieve ImRect (we don't need to store BuildWorkRect as every access tend to change it, hence the code asymmetry)
    //     ImRect  GetMainRect() const         { return ImRect(pos.x, pos.y, pos.x + size.x, pos.y + size.y); }
    //     ImRect  GetWorkRect() const         { return ImRect(work_pos.x, work_pos.y, work_pos.x + work_size.x, work_pos.y + work_size.y); }
    //     ImRect  GetBuildWorkRect() const    { Vector2D pos = CalcWorkRectPos(BuildWorkOffsetMin); Vector2D size = CalcWorkRectSize(BuildWorkOffsetMin, BuildWorkOffsetMax); return ImRect(pos.x, pos.y, pos.x + size.x, pos.y + size.y); }
}
