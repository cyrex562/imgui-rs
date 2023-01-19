// dear imgui: Renderer Backend for SDL_Renderer
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
// (Requires: SDL 2.0.17+)

// Important to understand: SDL_Renderer is an _optional_ component of SDL.
// For a multi-platform app consider using e.g. SDL+DirectX on Windows and SDL+OpenGL on Linux/OSX.
// If your application will want to render any non trivial amount of graphics other than UI,
// please be aware that SDL_Renderer offers a limited graphic API to the end-user and it might
// be difficult to step out of those boundaries.
// However, we understand it is a convenient choice to get an app started easily.

// Implemented features:
//  [X] Renderer: User texture binding. Use 'SDL_Texture*' as ImTextureID. Read the FAQ about ImTextureID!
//  [X] Renderer: Large meshes support (64k+ vertices) with 16-bit indices.
// Missing features:
//  [ ] Renderer: Multi-viewport support (multiple windows).

// You can copy and use unmodified imgui_impl_* files in your project. See examples/ folder for examples of using this.
// If you are new to Dear ImGui, read documentation from the docs/ folder + read the top of imgui.cpp.
// Read online: https://github.com/ocornut/imgui/tree/master/docs

// CHANGELOG
//  2021-12-21: Update SDL_RenderGeometryRaw() format to work with SDL 2.0.19.
//  2021-12-03: Added support for large mesh (64K+ vertices), enable IM_GUI_BACKEND_FLAGS_RENDERER_HAS_VTX_OFFSET flag.
//  2021-10-06: Backup and restore modified ClipRect/Viewport.
//  2021-09-21: Initial version.

// #include "imgui.h"
// #include "imgui_impl_sdlrenderer.h"
// #if defined(_MSC_VER) && _MSC_VER <= 1500 // MSVC 2008 or earlier
// #include <stddef.h>     // intptr_t
// #else
// #include <stdint.h>     // intptr_t
// #endif

// SDL
// #include <SDL.h>
// #if !SDL_VERSION_ATLEAST(2,0,17)
// #error This backend requires SDL 2.0.17+ because of SDL_RenderGeometry() function
// #endif

use std::ffi::CString;
use std::os::raw::c_void;
use std::ptr::null_mut;
use libc::c_int;
use sdl2::sys::{SDL_Color, SDL_CreateTexture, SDL_DestroyTexture, SDL_Log, SDL_Rect, SDL_Renderer, SDL_RenderGetClipRect, SDL_RenderGetScale, SDL_RenderGetViewport, SDL_RenderIsClipEnabled, SDL_RenderSetClipRect, SDL_RenderSetViewport, SDL_SetTextureBlendMode, SDL_SetTextureScaleMode, SDL_Texture, SDL_UpdateTexture};
use sdl2::sys::SDL_BlendMode::SDL_BLENDMODE_BLEND;
use sdl2::sys::SDL_bool::SDL_TRUE;
use sdl2::sys::SDL_PixelFormatEnum::SDL_PIXELFORMAT_ABGR8888;
use sdl2::sys::SDL_ScaleMode::SDL_ScaleModeLinear;
use sdl2::sys::SDL_TextureAccess::SDL_TEXTUREACCESS_STATIC;
use crate::backends::backend_flags::IM_GUI_BACKEND_FLAGS_RENDERER_HAS_VTX_OFFSET;
use crate::drawing::draw_data::ImDrawData;
use crate::io::io_ops::GetIO;

// SDL_Renderer data
#[derive(Default,Debug,Clone)]
pub struct ImGui_ImplSDLRenderer_Data
{
    // SDL_Renderer*   SDLRenderer;
    pub SDLRenderer: *mut SDL_Renderer,
    // SDL_Texture*    FontTexture;
    pub FontTexture: *mut SDL_Texture,
    // ImGui_ImplSDLRenderer_Data() { memset(this, 0, sizeof(*this)); }
}

// Backend data stored in io.BackendRendererUserData to allow support for multiple Dear ImGui contexts
// It is STRONGLY preferred that you use docking branch with multi-viewports (== single Dear ImGui context + multiple windows) instead of multiple Dear ImGui contexts.
pub fn ImGui_ImplSDLRenderer_GetBackendData() -> *mut ImGui_ImplSDLRenderer_Data
{
    return if GetCurrentContext() {
        GetIO().BackendRendererUserData as *mut ImGui_ImplSDLRenderer_Data
    } else { null_mut() };
}

// Functions
pub fn ImGui_ImplSDLRenderer_Init(renderer: *mut SDL_Renderer) -> bool
{
    let mut io = GetIO();
    // IM_ASSERT(io.BackendRendererUserData == NULL && "Already initialized a renderer backend!");
    // IM_ASSERT(renderer != NULL && "SDL_Renderer not initialized!");

    // Setup backend capabilities flags
    ImGui_ImplSDLRenderer_Data* bd = IM_NEW(ImGui_ImplSDLRenderer_Data)();
    io.BackendRendererUserData = bd;
    io.BackendRendererName = "imgui_impl_sdlrenderer";
    io.BackendFlags |= IM_GUI_BACKEND_FLAGS_RENDERER_HAS_VTX_OFFSET;  // We can honor the ImDrawCmd::VtxOffset field, allowing for large meshes.

    bd.SDLRenderer = renderer;

    return true;
}

pub fn ImGui_ImplSDLRenderer_Shutdown()
{
    let mut bd = ImGui_ImplSDLRenderer_GetBackendData();
    // IM_ASSERT(bd != NULL && "No renderer backend to shutdown, or already shutdown?");
    let mut io = GetIO();

    ImGui_ImplSDLRenderer_DestroyDeviceObjects();

    io.BackendRendererName = null_mut();
    io.BackendRendererUserData = null_mut();
    IM_DELETE(bd);
}

pub fn ImGui_ImplSDLRenderer_SetupRenderState()
{
	ImGui_ImplSDLRenderer_Data* bd = ImGui_ImplSDLRenderer_GetBackendData();

	// Clear out any viewports and cliprect set by the user
    // FIXME: Technically speaking there are lots of other things we could backup/setup/restore during our render process.
    unsafe { SDL_RenderSetViewport(bd.SDLRenderer, null_mut()); }
    unsafe { SDL_RenderSetClipRect(bd.SDLRenderer, null_mut()); }
}

pub fn ImGui_ImplSDLRenderer_NewFrame()
{
    ImGui_ImplSDLRenderer_Data* bd = ImGui_ImplSDLRenderer_GetBackendData();
    // IM_ASSERT(bd != NULL && "Did you call ImGui_ImplSDLRenderer_Init()?");

    if (!bd.FontTexture){
    ImGui_ImplSDLRenderer_CreateDeviceObjects();
}
}

pub fn ImGui_ImplSDLRenderer_RenderDrawData(draw_data: *mut ImDrawData)
{
	ImGui_ImplSDLRenderer_Data* bd = ImGui_ImplSDLRenderer_GetBackendData();

	// If there's a scale factor set by the user, use that instead
    // If the user has specified a scale factor to SDL_Renderer already via SDL_RenderSetScale(), SDL will scale whatever we pass
    // to SDL_RenderGeometryRaw() by that scale factor. In that case we don't want to be also scaling it ourselves here.
    let mut rsx = 1.0f32;
	let mut rsy = 1.0f32;
    unsafe { SDL_RenderGetScale(bd.SDLRenderer, &mut rsx, &mut rsy); }
    render_scale: ImVec2;
	render_scale.x = if rsx == 1.0f32 { draw_data.FramebufferScale.x} else {1.0f32 };
	render_scale.y = if rsy == 1.0f32 { draw_data.FramebufferScale.y } else { 1.0f32 };

	// Avoid rendering when minimized, scale coordinates for retina displays (screen coordinates != framebuffer coordinates)
	let mut fb_width = (draw_data.DisplaySize.x * render_scale.x);
	let mut fb_height = (draw_data.DisplaySize.y * render_scale.y);
	if fb_width == 0 || fb_height == 0 {
        return;
    }

    // Backup SDL_Renderer state that will be modified to restore it afterwards
    #[derive(Default,Debug,Clone)]
    struct BackupSDLRendererState
    {
        // SDL_Rect    Viewport;
        pub Viewport: SDL_Rect,
        // bool        ClipEnabled;
        pub ClipEnabled: bool,
        // SDL_Rect    ClipRect;
        pub ClipRect: SDL_Rect,
    }
    let mut old: BackupSDLRendererState = BackupSDLRendererState::default();
    unsafe { old.ClipEnabled = SDL_RenderIsClipEnabled(bd.SDLRenderer) == SDL_TRUE; }
    unsafe { SDL_RenderGetViewport(bd.SDLRenderer, &mut old.Viewport); }
    unsafe { SDL_RenderGetClipRect(bd.SDLRenderer, &mut old.ClipRect); }

	// Will project scissor/clipping rectangles into framebuffer space
	clip_off: ImVec2 = draw_data.DisplayPos;         // (0,0) unless using multi-viewports
	clip_scale: ImVec2 = render_scale;

    // Render command lists
    ImGui_ImplSDLRenderer_SetupRenderState();
    // for (int n = 0; n < draw_data.CmdListsCount; n++)
    for n in 0 .. draw_data.CmdListsCount
    {
        let cmd_list = draw_data.CmdLists[n];
        let vtx_buffer = cmd_list.VtxBuffer.Data;
        let idx_buffer = cmd_list.IdxBuffer.Data;

        // for (int cmd_i = 0; cmd_i < cmd_list.CmdBuffer.Size; cmd_i++)
        for cmd_i in 0 .. cmd_list.CmdBuffer.len()
        {
            let pcmd = &mut cmd_list.CmdBuffer[cmd_i];
            if pcmd.UserCallback
            {
                // User callback, registered via ImDrawList::AddCallback()
                // (ImDrawCallback_ResetRenderState is a special callback value used by the user to request the renderer to reset render state.)
                if pcmd.UserCallback == ImDrawCallback_ResetRenderState {
                    ImGui_ImplSDLRenderer_SetupRenderState();
                }
                else {
                    pcmd.UserCallback(cmd_list, pcmd);
                }
            }
            else
            {
                // Project scissor/clipping rectangles into framebuffer space
                let mut clip_min =  ImVec2::new((pcmd.ClipRect.x - clip_off.x) * clip_scale.x, (pcmd.ClipRect.y - clip_off.y) * clip_scale.y);
                let mut clip_max = ImVec2((pcmd.ClipRect.z - clip_off.x) * clip_scale.x, (pcmd.ClipRect.w - clip_off.y) * clip_scale.y);
                if clip_min.x < 0.0f32 { clip_min.x = 0.0f32; }
                if clip_min.y < 0.0f32 { clip_min.y = 0.0f32; }
                if clip_max.x > fb_width { clip_max.x = fb_width; }
                if clip_max.y > fb_height { clip_max.y = fb_height; }
                if clip_max.x <= clip_min.x || clip_max.y <= clip_min.y {
                    continue;
                }

                let mut  r: SDL_Rect = SDL_Rect{ x: (clip_min.x), y: (clip_min.y), w: (clip_max.x - clip_min.x), h: (clip_max.y - clip_min.y) };
                unsafe { SDL_RenderSetClipRect(bd.SDLRenderer, &r); }

                let mut xy = ((vtx_buffer + pcmd.VtxOffset) + IM_OFFSETOF(ImDrawVert, pos));
                let mut uv = ((vtx_buffer + pcmd.VtxOffset) + IM_OFFSETOF(ImDrawVert, uv));
// #if SDL_VERSION_ATLEAST(2,0,19)
                let mut color: *mut SDL_Color = ((vtx_buffer + pcmd.VtxOffset) + IM_OFFSETOF(ImDrawVert, col)); // SDL 2.0.19+
// #else
//                 const int* color = (const int*)((vtx_buffer + pcmd.VtxOffset) + IM_OFFSETOF(ImDrawVert, col)); // SDL 2.0.17 and 2.0.18
// #endif

                // Bind texture, Draw
				let tex: *mut SDL_Texture = pcmd.GetTexID();
                SDL_RenderGeometryRaw(bd.SDLRenderer, tex,
                    xy, sizeof(ImDrawVert),
                    color, sizeof(ImDrawVert),
                    uv, sizeof(ImDrawVert),
                    cmd_list.VtxBuffer.Size - pcmd.VtxOffset,
                    idx_buffer + pcmd.IdxOffset, pcmd.ElemCount, sizeof(ImDrawIdx));
            }
        }
    }

    // Restore modified SDL_Renderer state
    unsafe { SDL_RenderSetViewport(bd.SDLRenderer, &old.Viewport); }
    unsafe { SDL_RenderSetClipRect(bd.SDLRenderer, old.ClipEnabled? & old.ClipRect: null_mut()); }
}

// Called by Init/NewFrame/shutdown
pub fn ImGui_ImplSDLRenderer_CreateFontsTexture() -> bool
{
    let mut io = GetIO();
    ImGui_ImplSDLRenderer_Data* bd = ImGui_ImplSDLRenderer_GetBackendData();

    // Build texture atlas
    // unsigned char* pixels;
    let mut pixels: *mut u8 = null_mut();
    // int width, height;
    let mut width = 0i32;
    let mut height = 0i32;
    io.Fonts.GetTexDataAsRGBA32(&mut pixels, &mut width, &mut height);   // Load as RGBA 32-bit (75% of the memory is wasted, but default font is so small) because it is more likely to be compatible with user's existing shaders. If your ImTextureId represent a higher-level concept than just a GL texture id, consider calling GetTexDataAsAlpha8() instead to save on GPU memory.

    // Upload texture to graphics system
    // (Bilinear sampling is required by default. Set 'io.Fonts.Flags |= ImFontAtlasFlags_NoBakedLines' or 'style.AntiAliasedLinesUseTex = false' to allow point/nearest sampling)
    unsafe { bd.FontTexture = SDL_CreateTexture(bd.SDLRenderer, SDL_PIXELFORMAT_ABGR8888 as u32, SDL_TEXTUREACCESS_STATIC as c_int, width, height); }
    if bd.FontTexture == null_mut()
    {
        unsafe { SDL_Log(CString::from("error creating texture").as_ptr()); }
        return false;
    }
    unsafe { SDL_UpdateTexture(bd.FontTexture, null_mut(), pixels as *const c_void, 4 * width); }
    unsafe { SDL_SetTextureBlendMode(bd.FontTexture, SDL_BLENDMODE_BLEND); }
    unsafe { SDL_SetTextureScaleMode(bd.FontTexture, SDL_ScaleModeLinear); }

    // Store our identifier
    io.Fonts.SetTexID(bd.FontTexture);

    return true;
}

pub fn ImGui_ImplSDLRenderer_DestroyFontsTexture()
{
    let mut io = GetIO();
    ImGui_ImplSDLRenderer_Data* bd = ImGui_ImplSDLRenderer_GetBackendData();
    if bd.FontTexture
    {
        io.Fonts.SetTexID(0);
        unsafe { SDL_DestroyTexture(bd.FontTexture); }
        bd.FontTexture = null_mut();
    }
}

pub fn ImGui_ImplSDLRenderer_CreateDeviceObjects() -> bool
{
    return ImGui_ImplSDLRenderer_CreateFontsTexture();
}

pub fn ImGui_ImplSDLRenderer_DestroyDeviceObjects()
{
    ImGui_ImplSDLRenderer_DestroyFontsTexture();
}
