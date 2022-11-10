// struct ImGuiViewportP : public ImGuiViewport
#![allow(non_snake_case)]

// - Currently represents the Platform Window created by the application which is hosting our Dear ImGui windows.
// - With multi-viewport enabled, we extend this concept to have multiple active viewports.
// - In the future we will extend this concept further to also represent Platform Monitor and support a "no main platform window" operation mode.
// - About Main Area vs Work Area:
//   - Main Area = entire viewport.
//   - Work Area = entire viewport minus sections used by main menu bars (for platform windows), or by task bar (for platform monitor).
//   - Windows are generally trying to stay within the Work Area of their host viewport.

use crate::drawing::draw_data::{ImDrawData, ImDrawDataBuilder};
use crate::drawing::draw_list::ImDrawList;
use crate::core::math_ops::ImMax;
use crate::rect::ImRect;
use crate::core::type_defs::ImguiHandle;
use crate::core::vec2::Vector2;
use viewport_flags::ImGuiViewportFlags;
use crate::window::ImguiWindow;
use crate::INVALID_IMGUI_HANDLE;
use libc::{c_float, c_int, c_short, c_void};
use std::ptr::null_mut;
use viewport_renderer_user_data::ViewportRendererUserData;
use crate::viewport::viewport_platform_handle::ViewportPlatformHandle;

pub mod widget_ops;
pub mod viewport_ops;
pub mod viewport_flags;
pub mod viewport_renderer_user_data;
mod viewport_platform_handle;


pub struct Viewport {
    pub Idx: c_int,
    pub LastFrameActive: c_int,
    // Last frame number this viewport was activated by a window
    pub last_front_most_stamp_count: c_int,
    // Last stamp number from when a window hosted by this viewport was made front-most (by comparing this value between two viewport we have an implicit viewport z-order
    pub LastNameHash: ImguiHandle,
    pub LastPos: Vector2,
    pub Alpha: c_float,
    // Window opacity (when dragging dockable windows/viewports we make them transparent)
    pub LastAlpha: c_float,
    pub PlatformMonitor: c_short,
    pub PlatformWindowCreated: bool,
    pub window: ImguiHandle,
    // Set when the viewport is owned by a window (and ImGuiViewportFlags_CanHostOtherWindows is NOT set)
    // Last frame number the background (0) and foreground (1) draw lists were used
    pub DrawListsLastFrame: [c_int; 2],
    // Convenience background (0) and foreground (1) draw lists. We use them to draw software mouser cursor when io.MouseDrawCursor is set and to draw most debug overlays.
    pub DrawLists: [ImguiHandle; 2],
    // pub DrawDataP: ImDrawData,
    // pub DrawDataBuilder: ImDrawDataBuilder,
    pub LastPlatformPos: Vector2,
    pub LastPlatformSize: Vector2,
    pub LastRendererSize: Vector2,
    pub WorkOffsetMin: Vector2,
    // Work Area: Offset from Pos to top-left corner of Work Area. Generally (0,0) or (0,+main_menu_bar_height). Work Area is Full Area but without menu-bars/status-bars (so WorkArea always fit inside Pos/Size!)
    pub WorkOffsetMax: Vector2,
    // Work Area: Offset from Pos+Size to bottom-right corner of Work Area. Generally (0,0) or (0,-status_bar_height).
    pub BuildWorkOffsetMin: Vector2,
    // Work Area: Offset being built during current frame. Generally >= 0.0.
    pub BuildWorkOffsetMax: Vector2, // Work Area: Offset being built during current frame. Generally <= 0.0.
    pub ID: ImguiHandle,
    // Unique identifier for the viewport
    pub Flags: ImGuiViewportFlags,
    // See ImGuiViewportFlags_
    pub Pos: Vector2,
    // Main Area: Position of the viewport (Dear ImGui coordinates are the same as OS desktop/native coordinates)
    pub Size: Vector2,
    // Main Area: Size of the viewport.
    pub WorkPos: Vector2,
    // Work Area: Position of the viewport minus task bars, menus bars, status bars (>= Pos)
    pub WorkSize: Vector2,
    // Work Area: Size of the viewport minus task bars, menu bars, status bars (<= Size)
    pub DpiScale: c_float,
    // 1.0 = 96 DPI = No extra scale.
    pub ParentViewportId: ImguiHandle,
    // (Advanced) 0: no parent. Instruct the platform backend to setup a parent/child relationship between platform windows.
    pub DrawData: ImDrawData, // The ImDrawData corresponding to this viewport. Valid after Render() and until the next call to NewFrame().
    // Platform/Backend Dependent Data
    // Our design separate the Renderer and Platform backends to facilitate combining default backends with each others.
    // When our create your own backend for a custom engine, it is possible that both Renderer and Platform will be handled
    // by the same system and you may not need to use all the UserData/Handle fields.
    // The library never uses those fields, they are merely storage to facilitate backend implementation.
    pub RendererUserData:ViewportRendererUserData,
    // void* to hold custom data structure for the renderer (e.g. swap chain, framebuffers etc.). generally set by your Renderer_CreateWindow function.
    pub PlatformUserData: Vec<u8>,
    // void* to hold custom data structure for the OS / platform (e.g. windowing info, render context). generally set by your Platform_CreateWindow function.
    pub PlatformHandle:ViewportPlatformHandle,
    // void* for FindViewportByPlatformHandle(). (e.g. suggested to use natural platform handle such as HWND, GLFWWindow*, SDL_Window*)
    pub PlatformHandleRaw: ViewportPlatformHandle,
    // void* to hold lower-level, platform-native window handle (under Win32 this is expected to be a HWND, unused for other platforms), when using an abstraction layer like GLFW or SDL (where PlatformHandle would be a SDL_Window*)
    pub PlatformRequestMove: bool,
    // Platform window requested move (e.g. window was moved by the OS / host window manager, authoritative position will be OS window position)
    pub PlatformRequestResize: bool,
    // Platform window requested resize (e.g. window was resized by the OS / host window manager, authoritative size will be OS window size)
    pub PlatformRequestClose: bool, // Platform window requested closure (e.g. window was moved by the OS / host window manager, e.g. pressing ALT-F4)
}

impl Viewport {
    pub fn get_center(&self) -> Vector2 {
        Vector2::from_floats(
            self.Pos.x + self.Size.x * 0.5,
            self.Pos.y + self.Size.y * 0.5,
        )
    }

    pub fn get_work_center(&self) -> Vector2 {
        Vector2::from_floats(
            self.WorkPos.x + self.WorkSize.x * 0.5,
            self.WorkPos.y + self.WorkSize.y * 0.5,
        )
    }

    pub fn new() -> Self {
        Self {
            Idx: -1,
            LastFrameActive: -1,
            DrawListsLastFrame: [-1; 2],
            last_front_most_stamp_count: -1,
            Alpha: 1.0,
            LastAlpha: 1.0,
            PlatformMonitor: -1,
            PlatformWindowCreated: false,
            window: INVALID_IMGUI_HANDLE,
            DrawLists: [INVALID_IMGUI_HANDLE; 2],
            LastPlatformPos: Vector2::from_floats(f32::MAX, f32::MAX),
            LastPlatformSize: Vector2::from_floats(f32::MAX, f32::MAX),
            LastRendererSize: Vector2::from_floats(f32::MAX, f32::MAX),
            ..Default::default()
        }
    }

    pub fn clear_request_flags(&mut self) {
        self.PlatformRequestClose = false;
        self.PlatformRequestMove = false;
        self.PlatformRequestResize = false;
    }

    // Calculate work rect pos/size given a set of offset (we have 1 pair of offset for rect locked from last frame data, and 1 pair for currently building rect)
    pub fn calc_work_rect_pos(&self, off_min: &Vector2) -> Vector2 {
        Vector2::from_floats(self.Pos.x + off_min.x, self.Pos.y + off_min.y)
    }

    pub fn calc_work_rect_size(&self, off_min: &Vector2, off_max: &Vector2) -> Vector2 {
        Vector2::from_floats(
            ImMax(0.0, self.Size.x - off_min.x + off_max.x),
            ImMax(0.0, self.Size.y - off_min.y + off_max.y),
        )
    }

    pub fn update_work_rect(&mut self) {
        self.WorkPos = self.calc_work_rect_pos(&self.WorkOffsetMin);
        self.WorkSize = self.calc_work_rect_size(&self.WorkOffsetMin, &self.WorkOffsetMax)
    }

    // Helpers to retrieve ImRect (we don't need to store BuildWorkRect as every access tend to change it, hence the code asymmetry)
    pub fn get_main_rect(&self) -> ImRect {
        ImRect::from_floats(
            self.Pos.x,
            self.Pos.y,
            self.Pos.x + self.Size.x,
            self.Pos.y + self.Size.y,
        )
    }

    pub fn get_work_rect(&self) -> ImRect {
        ImRect::from_floats(
            self.WorkPos.x,
            self.WorkPos.y,
            self.WorkPos.x + self.WorkSize.x,
            self.WorkPos.y + self.WorkSize.y,
        )
    }

    pub fn get_build_work_rect(&self) -> ImRect {
        let pos = self.calc_work_rect_pos(&self.BuildWorkOffsetMin);
        let size = self.calc_work_rect_size(&self.BuildWorkOffsetMin, &self.BuildWorkOffsetMax);
        ImRect::from_floats(pos.x, pos.y, pos.x + size.x, pos.y + size.y)
    }
}
