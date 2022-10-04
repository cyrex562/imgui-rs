// struct ImGuiViewportP : public ImGuiViewport
#![allow(non_snake_case)]

// - Currently represents the Platform Window created by the application which is hosting our Dear ImGui windows.
// - With multi-viewport enabled, we extend this concept to have multiple active viewports.
// - In the future we will extend this concept further to also represent Platform Monitor and support a "no main platform window" operation mode.
// - About Main Area vs Work Area:
//   - Main Area = entire viewport.
//   - Work Area = entire viewport minus sections used by main menu bars (for platform windows), or by task bar (for platform monitor).
//   - Windows are generally trying to stay within the Work Area of their host viewport.

use std::ptr::null_mut;
use libc::{c_float, c_int, c_short, c_void};
use crate::draw_data::{ImDrawData, ImDrawDataBuilder};
use crate::draw_list::ImDrawList;
use crate::rect::ImRect;
use crate::vec2::ImVec2;
use crate::window::ImGuiWindow;
use crate::type_defs::ImGuiID;
use crate::viewport_flags::ImGuiViewportFlags;

pub struct ImGuiViewport {
    pub Idx: c_int,
    pub LastFrameActive: c_int,
    // Last frame number this viewport was activated by a window
    pub LastFrontMostStampCount: c_int,
    // Last stamp number from when a window hosted by this viewport was made front-most (by comparing this value between two viewport we have an implicit viewport z-order
    pub LastNameHash: ImGuiID,
    pub LastPos: ImVec2,
    pub Alpha: c_float,
    // Window opacity (when dragging dockable windows/viewports we make them transparent)
    pub LastAlpha: c_float,
    pub PlatformMonitor: c_short,
    pub PlatformWindowCreated: bool,
    pub Window: *mut ImGuiWindow,
    // Set when the viewport is owned by a window (and ImGuiViewportFlags_CanHostOtherWindows is NOT set)
// c_int                 DrawListsLastFrame[2];  // Last frame number the background (0) and foreground (1) draw lists were used
    pub DrawListsLastFrame: [c_int; 2],
    // ImDrawList*         DrawLists[2];           // Convenience background (0) and foreground (1) draw lists. We use them to draw software mouser cursor when io.MouseDrawCursor is set and to draw most debug overlays.
    pub DrawLists: [*mut ImDrawList; 2],
    pub DrawDataP: ImDrawData,
    pub DrawDataBuilder: ImDrawDataBuilder,
    pub LastPlatformPos: ImVec2,
    pub LastPlatformSize: ImVec2,
    pub LastRendererSize: ImVec2,
    pub WorkOffsetMin: ImVec2,
    // Work Area: Offset from Pos to top-left corner of Work Area. Generally (0,0) or (0,+main_menu_bar_height). Work Area is Full Area but without menu-bars/status-bars (so WorkArea always fit inside Pos/Size!)
    pub WorkOffsetMax: ImVec2,
    // Work Area: Offset from Pos+Size to bottom-right corner of Work Area. Generally (0,0) or (0,-status_bar_height).
    pub BuildWorkOffsetMin: ImVec2,
    // Work Area: Offset being built during current frame. Generally >= 0f32.
    pub BuildWorkOffsetMax: ImVec2,     // Work Area: Offset being built during current frame. Generally <= 0f32.

    pub ID: ImGuiID,
    // Unique identifier for the viewport
    pub Flags: ImGuiViewportFlags,
    // See ImGuiViewportFlags_
    pub Pos: ImVec2,
    // Main Area: Position of the viewport (Dear ImGui coordinates are the same as OS desktop/native coordinates)
    pub Size: ImVec2,
    // Main Area: Size of the viewport.
    pub WorkPos: ImVec2,
    // Work Area: Position of the viewport minus task bars, menus bars, status bars (>= Pos)
    pub WorkSize: ImVec2,
    // Work Area: Size of the viewport minus task bars, menu bars, status bars (<= Size)
    pub DpiScale: c_float,
    // 1f32 = 96 DPI = No extra scale.
    pub ParentViewportId: ImGuiID,
    // (Advanced) 0: no parent. Instruct the platform backend to setup a parent/child relationship between platform windows.
    pub DrawData: *mut ImDrawData,               // The ImDrawData corresponding to this viewport. Valid after Render() and until the next call to NewFrame().

    // Platform/Backend Dependent Data
    // Our design separate the Renderer and Platform backends to facilitate combining default backends with each others.
    // When our create your own backend for a custom engine, it is possible that both Renderer and Platform will be handled
    // by the same system and you may not need to use all the UserData/Handle fields.
    // The library never uses those fields, they are merely storage to facilitate backend implementation.
    pub RendererUserData: *mut c_void,
    // void* to hold custom data structure for the renderer (e.g. swap chain, framebuffers etc.). generally set by your Renderer_CreateWindow function.
    pub PlatformUserData: *mut c_void,
    // void* to hold custom data structure for the OS / platform (e.g. windowing info, render context). generally set by your Platform_CreateWindow function.
    pub PlatformHandle: *mut c_void,
    // void* for FindViewportByPlatformHandle(). (e.g. suggested to use natural platform handle such as HWND, GLFWWindow*, SDL_Window*)
    pub PlatformHandleRaw: *mut c_void,
    // void* to hold lower-level, platform-native window handle (under Win32 this is expected to be a HWND, unused for other platforms), when using an abstraction layer like GLFW or SDL (where PlatformHandle would be a SDL_Window*)
    pub PlatformRequestMove: bool,
    // Platform window requested move (e.g. window was moved by the OS / host window manager, authoritative position will be OS window position)
    pub PlatformRequestResize: bool,
    // Platform window requested resize (e.g. window was resized by the OS / host window manager, authoritative size will be OS window size)
    pub PlatformRequestClose: bool,   // Platform window requested closure (e.g. window was moved by the OS / host window manager, e.g. pressing ALT-F4)
}


impl ImGuiViewport {
    // ImGuiViewport()     { memset(this, 0, sizeof(*this)); }
    // ~ImGuiViewport()    { IM_ASSERT(PlatformUserData == NULL && RendererUserData == NULL); }

    // Helpers

    // ImVec2              GetCenter() const       { return ImVec2::new(Pos.x + Size.x * 0.5f32, Pos.y + Size.y * 0.5f32); }
    pub fn GetCenter(&self) -> ImVec2 {
        ImVec2::new2(self.Pos.x + self.Size.x * 0.5f32, self.Pos.y + self.Size.y * 0.5f32)
    }

    // ImVec2              GetWorkCenter() const   { return ImVec2::new(WorkPos.x + WorkSize.x * 0.5f32, WorkPos.y + WorkSize.y * 0.5f32); }
    pub fn GetWorkCenter(&self) -> ImVec2 {
        ImVec2::new2(
            self.WorkPos.x + self.WorkSize.x * 0.5f32,
            self.WorkPos.y + self.WorkSize.y * 0.5f32,
        )
    }

    // ImGuiViewportP()                    { Idx = -1; LastFrameActive = DrawListsLastFrame[0] = DrawListsLastFrame[1] = LastFrontMostStampCount = -1; LastNameHash = 0; Alpha = LastAlpha = 1f32; PlatformMonitor = -1; PlatformWindowCreated = false; Window = None; DrawLists[0] = DrawLists[1] = None; LastPlatformPos = LastPlatformSize = LastRendererSize = ImVec2::new(f32::MAX, f32::MAX); }
    pub fn new() -> Self {
        Self {
            Idx: -1,
            LastFrameActive: -1,
            DrawListsLastFrame: [-1; 2],
            LastFrontMostStampCount: -1,
            Alpha: 1f32,
            LastAlpha: 1f32,
            PlatformMonitor: -1,
            PlatformWindowCreated: false,
            Window: null_mut(),
            DrawLists: [null_mut(); 2],
            LastPlatformPos: ImVec2::new2(f32::MAX, f32::MAX),
            LastPlatformSize: ImVec2::new2(f32::MAX, f32::MAX),
            LastRendererSize: ImVec2::new2(f32::MAX, f32::MAX),
            ..Default::default()
        }
    }


    // ~ImGuiViewportP()                   { if (DrawLists[0]) IM_DELETE(DrawLists[0]); if (DrawLists[1]) IM_DELETE(DrawLists[1]); }


    // void    ClearRequestFlags()         { PlatformRequestClose = PlatformRequestMove = PlatformRequestResize = false; }
    pub fn ClearRequestFlags(&mut self) {
        self.PlatformRequestClose = false;
        self.PlatformRequestMove = false;
        self.PlatformRequestResize = false;
    }

    // Calculate work rect pos/size given a set of offset (we have 1 pair of offset for rect locked from last frame data, and 1 pair for currently building rect)
//     ImVec2  CalcWorkRectPos(const ImVec2& off_min) const                            { return ImVec2::new(Pos.x + off_min.x, Pos.y + off_min.y); }
    pub fn CalcWorkRectPos(&self, off_min: &ImVec2) -> ImVec2 {
        ImVec2::new2(self.Pos.x + off_min.x, self.Pos.y + off_min.y)
    }


    // ImVec2  CalcWorkRectSize(const ImVec2& off_min, const ImVec2& off_max) const    { return ImVec2::new(ImMax(0f32, Size.x - off_min.x + off_max.x), ImMax(0f32, Size.y - off_min.y + off_max.y)); }
    pub fn CalcWorkRectSize(&self, off_min: &ImVec2, off_max: &ImVec2) -> ImVec2 {
        ImVec2::new2(
            ImMax(0f32, self.Size.x - off_min.x + off_max.x),
            ImMax(0f32, self.Size.y - off_min.y + off_max.y),
        )
    }


    // void    UpdateWorkRect()            { WorkPos = CalcWorkRectPos(WorkOffsetMin); WorkSize = CalcWorkRectSize(WorkOffsetMin, WorkOffsetMax); } // Update public fields
    pub fn UpdateWorkRect(&mut self) {
        self.WorkPos = self.CalcWorkRectPos(&self.WorkOffsetMin);
        self.WorkSize = self.CalcWorkRectSize(&self.WorkOffsetMin, &self.WorkOffsetMax)
    }

    // Helpers to retrieve ImRect (we don't need to store BuildWorkRect as every access tend to change it, hence the code asymmetry)
//     ImRect  GetMainRect() const         { return ImRect::new(Pos.x, Pos.y, Pos.x + Size.x, Pos.y + Size.y); }
    pub fn GetMainRect(&self) -> ImRect {
        ImRect::new4(self.Pos.x, self.Pos.y, self.Pos.x + self.Size.x, self.Pos.y + self.Size.y)
    }


    // ImRect  GetWorkRect() const         { return ImRect::new(WorkPos.x, WorkPos.y, WorkPos.x + WorkSize.x, WorkPos.y + WorkSize.y); }
    pub fn GetWorkRect(&self) -> ImRect {
        ImRect::new4(self.WorkPos.x, self.WorkPos.y, self.WorkPos.x + self.WorkSize.x, self.WorkPos.y + self.WorkSize.y)
    }


    // ImRect  GetBuildWorkRect() const    { let mut pos: ImVec2 =  CalcWorkRectPos(BuildWorkOffsetMin); let mut size: ImVec2 =  CalcWorkRectSize(BuildWorkOffsetMin, BuildWorkOffsetMax); return ImRect::new(pos.x, pos.y, pos.x + size.x, pos.y + size.y); }
    pub fn GetBuildWorkRect(&self) -> ImRect {
        let pos = self.CalcWorkRectPos(&self.BuildWorkOffsetMin);
        let size = self.CalcWorkRectSize(&self.BuildWorkOffsetMin, &self.BuildWorkOffsetMax);
        ImRect::new4(pos.x, pos.y, pos.x + size.x, pos.y + size.y)
    }
}
