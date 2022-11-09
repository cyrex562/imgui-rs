use std::ffi::CStr;
use std::mem;
use std::ptr::null_mut;
use libc::{c_int, c_uchar, c_ulong, c_void, size_t};
use windows::Win32::Foundation::{BOOL, RECT};

struct IDirect3DDevice9;

use windows::Win32::Graphics::Direct3D9::{D3DBACKBUFFER_TYPE_MONO, D3DBLEND_INVSRCALPHA, D3DBLEND_ONE, D3DBLEND_SRCALPHA, D3DBLENDOP_ADD, D3DCULL_NONE, D3DDECLTYPE_D3DCOLOR, D3DFILL_SOLID, D3DFMT_A8R8G8B8, D3DFMT_D16, D3DFMT_INDEX16, D3DFMT_INDEX32, D3DFMT_UNKNOWN, D3DLOCK_DISCARD, D3DLOCKED_RECT, D3DPOOL_DEFAULT, D3DPRESENT_INTERVAL_IMMEDIATE, D3DPRESENT_PARAMETERS, D3DPT_TRIANGLELIST, D3DRS_ALPHABLENDENABLE, D3DRS_ALPHATESTENABLE, D3DRS_BLENDOP, D3DRS_CLIPPING, D3DRS_CULLMODE, D3DRS_DESTBLEND, D3DRS_DESTBLENDALPHA, D3DRS_FILLMODE, D3DRS_FOGENABLE, D3DRS_LIGHTING, D3DRS_RANGEFOGENABLE, D3DRS_SCISSORTESTENABLE, D3DRS_SEPARATEALPHABLENDENABLE, D3DRS_SHADEMODE, D3DRS_SPECULARENABLE, D3DRS_SRCBLEND, D3DRS_SRCBLENDALPHA, D3DRS_STENCILENABLE, D3DRS_ZENABLE, D3DRS_ZWRITEENABLE, D3DSAMP_MAGFILTER, D3DSAMP_MINFILTER, D3DSBT_ALL, D3DSHADE_GOURAUD, D3DSWAPEFFECT_DISCARD, D3DTEXF_LINEAR, D3DTOP_DISABLE, D3DTOP_MODULATE, D3DTS_PROJECTION, D3DTS_VIEW, D3DTS_WORLD, D3DTSS_ALPHAARG1, D3DTSS_ALPHAARG2, D3DTSS_ALPHAOP, D3DTSS_COLORARG1, D3DTSS_COLORARG2, D3DTSS_COLOROP, D3DUSAGE_DYNAMIC, D3DUSAGE_WRITEONLY, D3DVIEWPORT9, IDirect3DIndexBuffer9, IDirect3DStateBlock9, IDirect3DSurface9, IDirect3DSwapChain9, IDirect3DTexture9, IDirect3DVertexBuffer9};
use windows::Win32::Graphics::Direct3D::{D3DMATRIX, D3DMATRIX_0, D3DMATRIX_0_0};
use windows::Win32::System::SystemServices::{D3DCLEAR_TARGET, D3DFVF_DIFFUSE, D3DFVF_TEX1, D3DFVF_XYZ, D3DTA_DIFFUSE, D3DTA_TEXTURE};
use crate::backends::backend_flags::{IM_GUI_BACKEND_FLAGS_RENDERER_HAS_VIEWPORTS, IM_GUI_BACKEND_FLAGS_RENDERER_HAS_VTX_OFFSET};
use crate::backends::dx9_viewport_data::ImGui_ImplDX9_ViewportData;
use crate::core::config_flags::ImGuiConfigFlags_ViewportsEnable;
use crate::core::context::ImguiContext;
use crate::core::type_defs::ImDrawIdx;
use crate::core::utils::{flag_clear, flag_set};
use crate::core::vec2::ImVec2;
use crate::core::vec4::ImVec4;
use crate::drawing::draw_data::ImDrawData;
use crate::font::font_atlas::ImFontAtlas;
use crate::io::io_ops::GetIO;
use crate::viewport::ImguiViewport;
use crate::viewport::viewport_flags::ImGuiViewportFlags_NoRendererClear;
use crate::viewport::viewport_renderer_user_data::ViewportRendererUserData;

pub const D3D_OK: u32 = 0;


// DirectX data
#[derive(Default,Debug,Copy,Clone)]
pub struct ImGui_ImplDX9_Data
{
    // pub     pd3dDevice: LPDIRECT3DDEVICE9,
    pub pd3dDevice: *mut IDirect3DDevice9,
    pub pVB: *mut IDirect3DVertexBuffer9, //LPDIRECT3DVERTEXBUFFER9,
    pub pIB: *mut IDirect3DIndexBuffer9, //LPDIRECT3DINDEXBUFFER9,
    pub FontTexture: *mut IDirect3DTexture9, // LPDIRECT3DTEXTURE9,
    pub VertexBufferSize: i32,
    pub IndexBufferSize: i32,

}

impl ImGui_ImplDX9_Data {
    //     ImGui_ImplDX9_Data()        { memset((void*)this, 0, sizeof(*this)); VertexBufferSize = 5000; IndexBufferSize = 10000;}
    pub fn new() -> Self {
        Self {
            pd3dDevice: null_mut(),
            pVB: null_mut(),
            pIB: null_mut(),
            FontTexture: null_mut(),
            VertexBufferSize: 5000,
            IndexBufferSize: 10000,
        }
    }
}

#[derive(Default,Debug,Copy,Clone)]
pub struct CUSTOMVERTEX
{
    // float    pos[3];
    pub pos: [f32;3],
    // D3DCOLOR col;
    pub col: D3DDECLTYPE_D3DCOLOR,
    // float    uv[2];
    pub uv: [f32;2]
}

pub const D3DFVF_CUSTOMVERTEX: u32  = (D3DFVF_XYZ|D3DFVF_DIFFUSE|D3DFVF_TEX1);

// TODO
// #ifdef IMGUI_USE_BGRA_PACKED_COLOR
// #define IMGUI_COL_TO_DX9_ARGB(_COL)     (_COL)
// #else
// #define IMGUI_COL_TO_DX9_ARGB(_COL)     (((_COL) & 0xFF00FF00) | (((_COL) & 0xFF0000) >> 16) | (((_COL) & 0xFF) << 16))
// #endif


// Backend data stored in io.BackendRendererUserData to allow support for multiple Dear ImGui contexts
// It is STRONGLY preferred that you use docking branch with multi-viewports (== single Dear ImGui context + multiple windows) instead of multiple Dear ImGui contexts.
pub fn ImGui_ImplDX9_GetBackendData(g: &mut ImguiContext) -> *mut ImGui_ImplDX9_Data
{
    // return ImGui::GetCurrentContext() ? (ImGui_ImplDX9_Data*)ImGui::GetIO().BackendRendererUserData : NULL;
    // GetIO().BackendRendererUserData as *mut ImGui_ImplDX9_Data
    let mut io = GetIO();
    let berud = &mut io.BackendRendererUserData;
}

// Forward Declarations
// pub unsafe fn ImGui_ImplDX9_InitPlatformInterface();
// pub unsafe fn ImGui_ImplDX9_ShutdownPlatformInterface();
// pub unsafe fn ImGui_ImplDX9_CreateDeviceObjectsForPlatformWindows();
// pub unsafe fn ImGui_ImplDX9_InvalidateDeviceObjectsForPlatformWindows();

// Functions
pub unsafe fn ImGui_ImplDX9_SetupRenderState(g: &mut ImguiContext,  draw_data: &mut ImDrawData)
{
    let bd = ImGui_ImplDX9_GetBackendData(g);

    // Setup viewport
    let mut vp: D3DVIEWPORT9 = D3DVIEWPORT9::default();
    vp.X = 0;
    vp.Y = 0;
    vp.Width = draw_data.DisplaySize.x as u32;
    vp.Height = draw_data.DisplaySize.y as u32;
    vp.MinZ = 0.0;
    vp.MaxZ = 1.0;
    let result = bd.pd3dDevice.SetViewport(&vp);
    if result.is_err() {
        panic!()
    }

    // Setup render state: fixed-pipeline, alpha-blending, no face culling, no depth testing, shade mode (for gradient), bilinear sampling.
    bd.pd3dDevice.SetPixelShader(NULL);
    bd.pd3dDevice.SetVertexShader(NULL);
    bd.pd3dDevice.SetRenderState(D3DRS_FILLMODE, D3DFILL_SOLID);
    bd.pd3dDevice.SetRenderState(D3DRS_SHADEMODE, D3DSHADE_GOURAUD);
    bd.pd3dDevice.SetRenderState(D3DRS_ZWRITEENABLE, 0);
    bd.pd3dDevice.SetRenderState(D3DRS_ALPHATESTENABLE, 0);
    bd.pd3dDevice.SetRenderState(D3DRS_CULLMODE, D3DCULL_NONE);
    bd.pd3dDevice.SetRenderState(D3DRS_ZENABLE, 0);
    bd.pd3dDevice.SetRenderState(D3DRS_ALPHABLENDENABLE, 1);
    bd.pd3dDevice.SetRenderState(D3DRS_BLENDOP, D3DBLENDOP_ADD);
    bd.pd3dDevice.SetRenderState(D3DRS_SRCBLEND, D3DBLEND_SRCALPHA);
    bd.pd3dDevice.SetRenderState(D3DRS_DESTBLEND, D3DBLEND_INVSRCALPHA);
    bd.pd3dDevice.SetRenderState(D3DRS_SEPARATEALPHABLENDENABLE, 1);
    bd.pd3dDevice.SetRenderState(D3DRS_SRCBLENDALPHA, D3DBLEND_ONE);
    bd.pd3dDevice.SetRenderState(D3DRS_DESTBLENDALPHA, D3DBLEND_INVSRCALPHA);
    bd.pd3dDevice.SetRenderState(D3DRS_SCISSORTESTENABLE, 1);
    bd.pd3dDevice.SetRenderState(D3DRS_FOGENABLE, 0);
    bd.pd3dDevice.SetRenderState(D3DRS_RANGEFOGENABLE, 0);
    bd.pd3dDevice.SetRenderState(D3DRS_SPECULARENABLE, 0);
    bd.pd3dDevice.SetRenderState(D3DRS_STENCILENABLE, 0);
    bd.pd3dDevice.SetRenderState(D3DRS_CLIPPING, 1);
    bd.pd3dDevice.SetRenderState(D3DRS_LIGHTING, 0);
    bd.pd3dDevice.SetTextureStageState(0, D3DTSS_COLOROP, D3DTOP_MODULATE);
    bd.pd3dDevice.SetTextureStageState(0, D3DTSS_COLORARG1, D3DTA_TEXTURE);
    bd.pd3dDevice.SetTextureStageState(0, D3DTSS_COLORARG2, D3DTA_DIFFUSE);
    bd.pd3dDevice.SetTextureStageState(0, D3DTSS_ALPHAOP, D3DTOP_MODULATE);
    bd.pd3dDevice.SetTextureStageState(0, D3DTSS_ALPHAARG1, D3DTA_TEXTURE);
    bd.pd3dDevice.SetTextureStageState(0, D3DTSS_ALPHAARG2, D3DTA_DIFFUSE);
    bd.pd3dDevice.SetTextureStageState(1, D3DTSS_COLOROP, D3DTOP_DISABLE);
    bd.pd3dDevice.SetTextureStageState(1, D3DTSS_ALPHAOP, D3DTOP_DISABLE);
    bd.pd3dDevice.SetSamplerState(0, D3DSAMP_MINFILTER, D3DTEXF_LINEAR);
    bd.pd3dDevice.SetSamplerState(0, D3DSAMP_MAGFILTER, D3DTEXF_LINEAR);

    // Setup orthographic projection matrix
    // Our visible imgui space lies from draw_data.DisplayPos (top left) to draw_data.DisplayPos+data_data.DisplaySize (bottom right). DisplayPos is (0,0) for single viewport apps.
    // Being agnostic of whether <d3dx9.h> or <DirectXMath.h> can be used, we aren't relying on D3DXMatrixIdentity()/D3DXMatrixOrthoOffCenterLH() or DirectX::XMMatrixIdentity()/DirectX::XMMatrixOrthographicOffCenterLH()
    {
        let mut L: f32 = draw_data.DisplayPos.x + 0.5;
        let mut R: f32 = draw_data.DisplayPos.x + draw_data.DisplaySize.x + 0.5;
        let mut T: f32 = draw_data.DisplayPos.y + 0.5;
        let mut B: f32 = draw_data.DisplayPos.y + draw_data.DisplaySize.y + 0.5;
        let mat_identity = D3DMATRIX {
            Anonymous: D3DMATRIX_0 {
                Anonymous: D3DMATRIX_0_0 {
                    _11: 1.0,
                    _12: 0.0,
                    _13: 0.0,
                    _14: 0.0,
                    _21: 0.0,
                    _22: 1.0,
                    _23: 0.0,
                    _24: 0.0,
                    _31: 0.0,
                    _32: 0.0,
                    _33: 1.0,
                    _34: 0.0,
                    _41: 0.0,
                    _42: 0.0,
                    _43: 0.0,
                    _44: 1.0
                }
            }
        };
        let  mat_projection = D3DMATRIX {
            Anonymous: D3DMATRIX_0{
                Anonymous: D3DMATRIX_0_0{
                    _11: 2.0/(R-L),
                    _12: 0.0,
                    _13: 0.0,
                    _14: 0.0,
                    _21: 0.0,
                    _22: 2.0/(T-B),
                    _23: 0.0,
                    _24: 0.0,
                    _31: 0.0,
                    _32: 0.0,
                    _33: 0.5,
                    _34: 0.0,
                    _41: (L+R)/(L-R),
                    _42: (T+B)/(B-T),
                    _43: 0.5,
                    _44: 1.0
                }
            }
        };

        bd.pd3dDevice.SetTransform(D3DTS_WORLD, &mat_identity);
        bd.pd3dDevice.SetTransform(D3DTS_VIEW, &mat_identity);
        bd.pd3dDevice.SetTransform(D3DTS_PROJECTION, &mat_projection);
    }
}

// Render function.
pub unsafe fn ImGui_ImplDX9_RenderDrawData(draw_data: &mut ImDrawData)
{
    // Avoid rendering when minimized
    if draw_data.DisplaySize.x <= 0.0 || draw_data.DisplaySize.y <= 0.0 {
        return;
    }

    // Create and grow buffers if needed
    let mut bd = ImGui_ImplDX9_GetBackendData(g);
    if bd.pVB.is_null() || bd.VertexBufferSize < draw_data.TotalVtxCount as i32
    {
        if bd.pVB.is_null() == false {
            bd.pVB.Release();
            bd.pVB = null_mut();
        }
        bd.VertexBufferSize = (draw_data.TotalVtxCount + 5000) as i32;
        if bd.pd3dDevice.CreateVertexBuffer(bd.VertexBufferSize * mem::size_of::<CUSTOMVERTEX>(), D3DUSAGE_DYNAMIC | D3DUSAGE_WRITEONLY, D3DFVF_CUSTOMVERTEX, D3DPOOL_DEFAULT, &bd.pVB, NULL) < 0 {
            return;
        }
    }
    if bd.pIB.is_null() || bd.IndexBufferSize < draw_data.TotalIdxCount as i32
    {
        if bd.pIB {
            bd.pIB.Release();
            bd.pIB = null_mut();
        }
        bd.IndexBufferSize = (draw_data.TotalIdxCount + 10000) as i32;
        if bd.pd3dDevice.CreateIndexBuffer(bd.IndexBufferSize * mem::size_of::<ImDrawIdx>(), D3DUSAGE_DYNAMIC | D3DUSAGE_WRITEONLY, if mem::size_of::<ImDrawIdx>() == 2 { D3DFMT_INDEX16 } else { D3DFMT_INDEX32 }, D3DPOOL_DEFAULT, &bd.pIB, null_mut()) < 0 {
            return;
        }
    }

    // Backup the DX9 state
    let mut d3d9_state_block: *mut IDirect3DStateBlock9 = null_mut();
    if bd.pd3dDevice.CreateStateBlock(D3DSBT_ALL, &d3d9_state_block) < 0 {
        return;
    }
    if d3d9_state_block.Capture() < 0
    {
        d3d9_state_block.Release();
        return;
    }

    // Backup the DX9 transform (DX9 documentation suggests that it is included in the StateBlock but it doesn't appear to)
    // D3DMATRIX last_world, last_view, last_projection;
    let mut last_world: D3DMATRIX = D3DMATRIX{ Anonymous: D3DMATRIX_0 {Anonymous: D3DMATRIX_0_0{
        _11: 0.0,
        _12: 0.0,
        _13: 0.0,
        _14: 0.0,
        _21: 0.0,
        _22: 0.0,
        _23: 0.0,
        _24: 0.0,
        _31: 0.0,
        _32: 0.0,
        _33: 0.0,
        _34: 0.0,
        _41: 0.0,
        _42: 0.0,
        _43: 0.0,
        _44: 0.0
    }} };

    let mut last_view: D3DMATRIX = D3DMATRIX{ Anonymous: D3DMATRIX_0 {Anonymous: D3DMATRIX_0_0{
        _11: 0.0,
        _12: 0.0,
        _13: 0.0,
        _14: 0.0,
        _21: 0.0,
        _22: 0.0,
        _23: 0.0,
        _24: 0.0,
        _31: 0.0,
        _32: 0.0,
        _33: 0.0,
        _34: 0.0,
        _41: 0.0,
        _42: 0.0,
        _43: 0.0,
        _44: 0.0
    }} };

    let mut last_projection: D3DMATRIX = D3DMATRIX{ Anonymous: D3DMATRIX_0 {Anonymous: D3DMATRIX_0_0{
        _11: 0.0,
        _12: 0.0,
        _13: 0.0,
        _14: 0.0,
        _21: 0.0,
        _22: 0.0,
        _23: 0.0,
        _24: 0.0,
        _31: 0.0,
        _32: 0.0,
        _33: 0.0,
        _34: 0.0,
        _41: 0.0,
        _42: 0.0,
        _43: 0.0,
        _44: 0.0
    }} };

    bd.pd3dDevice.GetTransform(D3DTS_WORLD, &last_world);
    bd.pd3dDevice.GetTransform(D3DTS_VIEW, &last_view);
    bd.pd3dDevice.GetTransform(D3DTS_PROJECTION, &last_projection);

    // Allocate buffers
    // CUSTOMVERTEX* vtx_dst;
    let mut vtx_dst: *mut CUSTOMVERTEX = null_mut();
    // ImDrawIdx* idx_dst;
    let mut idx_dst: *mut ImDrawIdx = null_mut();
    if bd.pVB.Lock(0, (draw_data.TotalVtxCount * mem::size_of::<CUSTOMVERTEX>()) as u32, (&mut vtx_dst) as *mut *mut c_void, D3DLOCK_DISCARD as u32) < 0
    {
        d3d9_state_block.Release();
        return;
    }
    if bd.pIB.Lock(0, (draw_data.TotalIdxCount * mem::size_of::<ImDrawIdx>()) as u32, (&mut idx_dst) as *mut *mut c_void, D3DLOCK_DISCARD as u32) < 0
    {
        bd.pVB.Unlock();
        d3d9_state_block.Release();
        return;
    }

    // Copy and convert all vertices into a single contiguous buffer, convert colors to DX9 default format.
    // FIXME-OPT: This is a minor waste of resource, the ideal is to use imconfig.h and
    //  1) to avoid repacking colors:   #define IMGUI_USE_BGRA_PACKED_COLOR
    //  2) to avoid repacking vertices: #define IMGUI_OVERRIDE_DRAWVERT_STRUCT_LAYOUT struct ImDrawVert { ImVec2 pos; float z; ImU32 col; ImVec2 uv; }
    // for (int n = 0; n < draw_data.CmdListsCount; n++)
    for n in 0 .. draw_data.CmdListsCount
    {
        let cmd_list = draw_data.CmdLists[n];
        let vtx_src = cmd_list.VtxBuffer.Data;
        // for (int i = 0; i < cmd_list.VtxBuffer.Size; i++)
        for i in 0 .. cmd_list.VtxBuffer.len()
        {
            vtx_dst.pos[0] = vtx_src.pos.x;
            vtx_dst.pos[1] = vtx_src.pos.y;
            vtx_dst.pos[2] = 0.0;
            vtx_dst.col = IMGUI_COL_TO_DX9_ARGB(vtx_src.col);
            vtx_dst.uv[0] = vtx_src.uv.x;
            vtx_dst.uv[1] = vtx_src.uv.y;
            vtx_dst += 1;
            vtx_src += 1;
        }
        libc::memcpy(idx_dst as *mut c_void, cmd_list.IdxBuffer.Data, cmd_list.IdxBuffer.Size * mem::size_of::<ImDrawIdx>());
        idx_dst += cmd_list.IdxBuffer.Size;
    }
    bd.pVB.Unlock();
    bd.pIB.Unlock();
    bd.pd3dDevice.SetStreamSource(0, bd.pVB, 0, mem::size_of::<CUSTOMVERTEX>());
    bd.pd3dDevice.SetIndices(bd.pIB);
    bd.pd3dDevice.SetFVF(D3DFVF_CUSTOMVERTEX);

    // Setup desired DX state
    ImGui_ImplDX9_SetupRenderState(g, draw_data);

    // Render command lists
    // (Because we merged all buffers into a single one, we maintain our own offset into them)
    let mut global_vtx_offset = 0;
    let mut global_idx_offset = 0;
    let mut clip_off = draw_data.DisplayPos;
    // for (int n = 0; n < draw_data.CmdListsCount; n++)
    for n in 0 .. draw_data.CmdListsCount
    {
        let cmd_list = draw_data.CmdLists[n];
        // for (int cmd_i = 0; cmd_i < cmd_list.CmdBuffer.Size; cmd_i++)
        for cmd_i in 0 .. cmd_list.CmdBuffer.len()
        {
            let pcmd = &cmd_list.CmdBuffer[cmd_i];
            if pcmd.UserCallback !=null_mut()
            {
                // User callback, registered via ImDrawList::AddCallback()
                // (ImDrawCallback_ResetRenderState is a special callback value used by the user to request the renderer to reset render state.)
                if pcmd.UserCallback == ImDrawCallback_ResetRenderState {
                    ImGui_ImplDX9_SetupRenderState(g, draw_data);
                }
                else {
                    pcmd.UserCallback(cmd_list, pcmd);
                }
            }
            else
            {
                // Project scissor/clipping rectangles into framebuffer space
                let clip_min = ImVec2::from_floats(pcmd.ClipRect.x - clip_off.x, pcmd.ClipRect.y - clip_off.y);
                let clip_max = ImVec2::from_floats(pcmd.ClipRect.z - clip_off.x, pcmd.ClipRect.w - clip_off.y);
                if clip_max.x <= clip_min.x || clip_max.y <= clip_min.y {
                    continue;
                }

                // Apply Scissor/clipping rectangle, Bind texture, Draw
                let r = RECT{ left: clip_min.x as i32, top: clip_min.y as i32, right: clip_max.x as i32, bottom: clip_max.y as i32 };
                let texture = pcmd.GetTexID() as *mut IDirect3DTexture9 ; // (LPDIRECT3DTEXTURE9)
                bd.pd3dDevice.SetTexture(0, texture);
                bd.pd3dDevice.SetScissorRect(&r);
                bd.pd3dDevice.DrawIndexedPrimitive(D3DPT_TRIANGLELIST, pcmd.VtxOffset + global_vtx_offset, 0, cmd_list.VtxBuffer.Size, pcmd.IdxOffset + global_idx_offset, pcmd.ElemCount / 3);
            }
        }
        global_idx_offset += cmd_list.IdxBuffer.Size;
        global_vtx_offset += cmd_list.VtxBuffer.Size;
    }

    // When using multi-viewports, it appears that there's an odd logic in DirectX9 which prevent subsequent windows
    // from rendering until the first window submits at least one draw call, even once. That's our workaround. (see #2560)
    if global_vtx_offset == 0 {
        bd.pd3dDevice.DrawIndexedPrimitive(D3DPT_TRIANGLELIST, 0, 0, 0, 0, 0);
    }

    // Restore the DX9 transform
    bd.pd3dDevice.SetTransform(D3DTS_WORLD, &last_world);
    bd.pd3dDevice.SetTransform(D3DTS_VIEW, &last_view);
    bd.pd3dDevice.SetTransform(D3DTS_PROJECTION, &last_projection);

    // Restore the DX9 state
    d3d9_state_block.Apply();
    d3d9_state_block.Release();
}

pub unsafe fn ImGui_ImplDX9_Init(device: *mut IDirect3DDevice9) -> bool
{
    // ImGuiIO& io = ImGui::GetIO();
    let io = GetIO();
    // IM_ASSERT(io.BackendRendererUserData == NULL && "Already initialized a renderer backend!");

    // Setup backend capabilities flags
    // ImGui_ImplDX9_Data* bd = IM_NEW(ImGui_ImplDX9_Data)();
    let mut bd: *mut ImGui_ImplDX9_Data = libc::malloc(mem::size_of::<ImGui_ImplDX9_Data>()) as *mut ImGui_ImplDX9_Data;
    io.BackendRendererUserData = bd as *mut c_void;
    io.BackendRendererName = String::from("imgui_impl_dx9");
    io.BackendFlags |= IM_GUI_BACKEND_FLAGS_RENDERER_HAS_VTX_OFFSET;  // We can honor the ImDrawCmd::VtxOffset field, allowing for large meshes.
    io.BackendFlags |= IM_GUI_BACKEND_FLAGS_RENDERER_HAS_VIEWPORTS;  // We can create multi-viewports on the Renderer side (optional)
    bd.pd3dDevice = device;
    bd.pd3dDevice.AddRef();

    if flag_set(io.ConfigFlags , ImGuiConfigFlags_ViewportsEnable) {
        ImGui_ImplDX9_InitPlatformInterface();
    }

    return true;
}

pub unsafe fn ImGui_ImplDX9_Shutdown(g: &mut ImguiContext)
{
    let bd = ImGui_ImplDX9_GetBackendData(g);
    // IM_ASSERT(bd != NULL && "No renderer backend to shutdown, or already shutdown?");
    // ImGuiIO& io = ImGui::GetIO();
    let io = GetIO();

    ImGui_ImplDX9_ShutdownPlatformInterface();
    ImGui_ImplDX9_InvalidateDeviceObjects(g);
    if bd.pd3dDevice { bd.pd3dDevice.Release(); }
    io.BackendRendererName = null_mut();
    io.BackendRendererUserData = null_mut();
    libc::free(bd as *mut c_void);
}

pub unsafe fn ImGui_ImplDX9_CreateFontsTexture(g: &mut ImguiContext) -> bool
{
    // Build texture atlas
    // ImGuiIO& io = ImGui::GetIO();
    let io = GetIO();
    let bd = ImGui_ImplDX9_GetBackendData(g);
    // unsigned char* pixels;
    let mut pixels: *mut c_uchar = null_mut();
    // int width, height, bytes_per_pixel;
    let mut width: c_int = 0;
    let mut height: c_int = 0;
    let mut bytes_per_pixel: c_int = 0;
    io.Fonts.GetTexDataAsRGBA32(&pixels, &width, &height, &bytes_per_pixel);

    // Convert RGBA32 to BGRA32 (because RGBA32 is not well supported by DX9 devices)
// #ifndef IMGUI_USE_BGRA_PACKED_COLOR
//     if (io.Fonts.TexPixelsUseColors)
//     {
//         ImU32* dst_start = (ImU32*)ImGui::MemAlloc((size_t)width * height * bytes_per_pixel);
//         for (ImU32* src = (ImU32*)pixels, *dst = dst_start, *dst_end = dst_start + (size_t)width * height; dst < dst_end; src++, dst++)
//             *dst = IMGUI_COL_TO_DX9_ARGB(*src);
//         pixels = (unsigned char*)dst_start;
//     }
// #endif

    // Upload texture to graphics system
    bd.FontTexture = null_mut();
    if bd.pd3dDevice.CreateTexture(width, height, 1, D3DUSAGE_DYNAMIC, D3DFMT_A8R8G8B8, D3DPOOL_DEFAULT, &bd.FontTexture, NULL) < 0 {
        return false;
    }
    let mut tex_locked_rect: D3DLOCKED_RECT = D3DLOCKED_RECT{ Pitch: 0, pBits: null_mut() };
    let mut empty_rect: RECT = RECT::default();
    if bd.FontTexture.LockRect(0, &mut tex_locked_rect, &empty_rect, 0) != D3D_OK {
        return false;
    }
    // for (int y = 0; y < height; y++)
    for y in 0 .. height
    {
        libc::memcpy(
        tex_locked_rect.pBits + tex_locked_rect.Pitch * y,
        pixels + width * bytes_per_pixel * y,
        (width * bytes_per_pixel) as size_t);
    }
    bd.FontTexture.UnlockRect(0);

    // Store our identifier
    io.Fonts.SetTexID(bd.FontTexture);

// #ifndef IMGUI_USE_BGRA_PACKED_COLOR
//     if (io.Fonts.TexPixelsUseColors)
//         ImGui::MemFree(pixels);
// #endif

    return true;
}

pub unsafe fn ImGui_ImplDX9_CreateDeviceObjects(g: &mut ImguiContext) -> bool
{
    let bd = ImGui_ImplDX9_GetBackendData(g);
    if bd.is_null() || bd.pd3dDevice.is_null() {
        return false;
    }
    if !ImGui_ImplDX9_CreateFontsTexture(g) {
        return false;
    }
    ImGui_ImplDX9_CreateDeviceObjectsForPlatformWindows();
    return true;
}

pub unsafe fn ImGui_ImplDX9_InvalidateDeviceObjects(g: &mut ImguiContext)
{
    let bd = ImGui_ImplDX9_GetBackendData(g);
    if bd.is_null() || bd.pd3dDevice.is_null() {
        return;
    }
    if bd.pVB { bd.pVB.Release(); bd.pVB = NULL; }
    if bd.pIB { bd.pIB.Release(); bd.pIB = NULL; }
    if bd.FontTexture.is_null() == false {
        bd.FontTexture.Release();
        bd.FontTexture = null_mut();
        let aio = GetIO();
        let fonts: &mut ImFontAtlas = &mut aio.Fonts.unwrap();
        fonts.SetTexID(tex_id);
        // GetIO().Fonts.unwrap()(NULL);
    } // We copied bd.pFontTextureView to io.Fonts.TexID so let's clear that as well.
    ImGui_ImplDX9_InvalidateDeviceObjectsForPlatformWindows();
}

pub unsafe fn ImGui_ImplDX9_NewFrame(g: &mut ImguiContext)
{
    let bd = ImGui_ImplDX9_GetBackendData(g);
    // IM_ASSERT(bd != NULL && "Did you call ImGui_ImplDX9_Init()?");

    if bd.FontTexture.is_null() {
        ImGui_ImplDX9_CreateDeviceObjects(g);
    }
}

//--------------------------------------------------------------------------------------------------------
// MULTI-VIEWPORT / PLATFORM INTERFACE SUPPORT
// This is an _advanced_ and _optional_ feature, allowing the backend to create and handle multiple viewports simultaneously.
// If you are new to dear imgui or creating a new binding for dear imgui, it is recommended that you completely ignore this section first..
//--------------------------------------------------------------------------------------------------------

pub unsafe fn ImGui_ImplDX9_CreateWindow(g: &mut ImguiContext, viewport: &mut ImguiViewport)
{
    let bd = ImGui_ImplDX9_GetBackendData(g);
    // let vd = libc::malloc(mem::size_of::<ImGui_ImplDX9_ViewportData>()) as *mut ImGui_ImplDX9_ViewportData;//IM_NEW(ImGui_ImplDX9_ViewportData)();
     let mut vd: &mut ImGui_ImplDX9_ViewportData = &mut ImGui_ImplDX9_ViewportData::default();
    // let vd: &mut ImGui_ImplDX9_Data = viewport.RendererUserData.into();
    if let ViewportRendererUserData::Dx9ViewportData(&mut x) = viewport.RendererUserData {
        vd = x;
    }
    
    // PlatformHandleRaw should always be a HWND, whereas PlatformHandle might be a higher-level handle (e.g. GLFWWindow*, SDL_Window*).
    // Some backends will leave PlatformHandleRaw NULL, in which case we assume PlatformHandle will contain the HWND.
    let hwnd = if viewport.PlatformHandleRaw != ViewportPlatformHandle::Unset { &mut viewport.PlatformHandleRaw } else { &mut viewport.PlatformHandle };
    // IM_ASSERT(hwnd != 0);

    // ZeroMemory(&vd.d3dpp, sizeof(D3DPRESENT_PARAMETERS));
    vd.d3dpp.Windowed = BOOL::from(true);
    vd.d3dpp.SwapEffect = D3DSWAPEFFECT_DISCARD;
    vd.d3dpp.BackBufferWidth = viewport.Size.x as u32;
    vd.d3dpp.BackBufferHeight = viewport.Size.y as u32;
    vd.d3dpp.BackBufferFormat = D3DFMT_UNKNOWN;
    vd.d3dpp.hDeviceWindow = hwnd.into();
    vd.d3dpp.EnableAutoDepthStencil = BOOL::from(false);
    vd.d3dpp.AutoDepthStencilFormat = D3DFMT_D16;
    vd.d3dpp.PresentationInterval = D3DPRESENT_INTERVAL_IMMEDIATE as u32;   // Present without vsync

    let hr = bd.pd3dDevice.CreateAdditionalSwapChain(&vd.d3dpp, &vd.swap_chain);
    // IM_UNUSED(hr);
    // IM_ASSERT(hr == D3D_OK);
    // IM_ASSERT(vd.swap_chain != NULL);
     viewport.RendererUserData = ViewportRendererUserData::Dx9ViewportData(vd);
}

pub unsafe fn ImGui_ImplDX9_DestroyWindow(viewport: &mut ImguiViewport)
{
    // The main viewport (owned by the application) will always have RendererUserData == NULL since we didn't create the data for it.
     let mut vd: &mut ImGui_ImplDX9_ViewportData = &mut ImGui_ImplDX9_ViewportData::default();
    // let vd: &mut ImGui_ImplDX9_Data = viewport.RendererUserData.into();
    if let ViewportRendererUserData::Dx9ViewportData(&mut x) = viewport.RendererUserData {
        vd = x;
    }
    // if (ImGui_ImplDX9_ViewportData* vd = (ImGui_ImplDX9_ViewportData*)viewport.RendererUserData)
    // {
    //     
    // }
    if vd.swap_chain.is_null() == false {
        vd.swap_chain.Release();
    }
        vd.swap_chain = null_mut();
        // ZeroMemory(&vd.d3dpp, sizeof(D3DPRESENT_PARAMETERS));
        // IM_DELETE(vd);
    viewport.RendererUserData = ViewportRendererUserData::Unset;
}

pub fn ImGui_ImplDX9_SetWindowSize(g: &mut ImguiContext, viewport: &mut ImguiViewport, size: ImVec2)
{
   let bd = ImGui_ImplDX9_GetBackendData(g);
    let mut vd: &mut ImGui_ImplDX9_ViewportData = &mut ImGui_ImplDX9_ViewportData::default();
    // let vd: &mut ImGui_ImplDX9_Data = viewport.RendererUserData.into();
    if let ViewportRendererUserData::Dx9ViewportData(&mut x) = viewport.RendererUserData {
        vd = x;
    }
    if vd.swap_chain != null_mut()
    {
        vd.swap_chain.Release();
        vd.swap_chain = null_mut();
        vd.d3dpp.BackBufferWidth = size.x as u32;
        vd.d3dpp.BackBufferHeight = size.y as u32;
        let hr = bd.pd3dDevice.CreateAdditionalSwapChain(&vd.d3dpp, &vd.swap_chain); IM_UNUSED(hr);
        // IM_ASSERT(hr == D3D_OK);
    }
}


pub type D3DCOLOR = c_ulong;

pub fn D3DCOLOR_RGBA(r: c_int, g: c_int, b: c_int, a: c_int) -> D3DCOLOR {

    let bytes: [u8;4] = [r as u8,g as u8 ,b as u8,a as u8];
    D3DCOLOR::from_le_bytes(bytes)
}

pub unsafe fn ImGui_ImplDX9_RenderWindow(g: &mut ImguiContext, viewport: &mut ImguiViewport)
{
    let bd = ImGui_ImplDX9_GetBackendData(g);
    let mut vd: &mut ImGui_ImplDX9_ViewportData = &mut ImGui_ImplDX9_ViewportData::default();
    if let ViewportRendererUserData::Dx9ViewportData(&mut x) = viewport.RendererUserData {
        vd = x;
    }
    // ImGui_ImplDX9_ViewportData* vd = (ImGui_ImplDX9_ViewportData*)viewport.RendererUserData;
    let clear_color = ImVec4::from_floats(0.0, 0.0, 0.0,1.0);
    // LPDIRECT3DSURFACE9 render_target = NULL;
    // let render_target: *mut IDirect3DSurface9 = null_mut();
    // LPDIRECT3DSURFACE9 last_render_target = NULL;
    let last_render_target: *mut IDirect3DSurface9 = null_mut();
    // LPDIRECT3DSURFACE9 last_depth_stencil = NULL;
    let last_depth_stencil: *mut IDirect3DSurface9 = null_mut();
    let render_target = vd.swap_chain.GetBackBuffer(0, D3DBACKBUFFER_TYPE_MONO).unwrap();
    bd.pd3dDevice.GetRenderTarget(0, &last_render_target);
    bd.pd3dDevice.GetDepthStencilSurface(&last_depth_stencil);
    bd.pd3dDevice.SetRenderTarget(0, render_target);
    bd.pd3dDevice.SetDepthStencilSurface(NULL);

    if flag_clear(viewport.Flags, ImGuiViewportFlags_NoRendererClear)
    {
        let clear_col_dx = D3DCOLOR_RGBA((clear_color.x*255.0), (clear_color.y*255.0), (clear_color.z*255.0), (clear_color.w*255.0));
        bd.pd3dDevice.Clear(0, null_mut(), D3DCLEAR_TARGET, clear_col_dx,1.0, 0);
    }

    ImGui_ImplDX9_RenderDrawData(&mut viewport.DrawData);

    // Restore render target
    bd.pd3dDevice.SetRenderTarget(0, last_render_target);
    bd.pd3dDevice.SetDepthStencilSurface(last_depth_stencil);
    render_target.Release();
    last_render_target.Release();
    if (last_depth_stencil) { last_depth_stencil.Release(); }
}

pub unsafe fn ImGui_ImplDX9_SwapBuffers(g: &mut ImguiContext, viewport: &mut ImguiViewport)
{
    // ImGui_ImplDX9_ViewportData* vd = (ImGui_ImplDX9_ViewportData*)viewport.RendererUserData;
    let mut vd: &mut ImGui_ImplDX9_ViewportData = &mut ImGui_ImplDX9_ViewportData::default();
    if let ViewportRendererUserData::Dx9ViewportData(&mut x) = viewport.RendererUserData {
        vd = x;
    }

    HRESULT hr = vd.swap_chain.Present(NULL, NULL, vd.d3dpp.hDeviceWindow, NULL, 0);
    // Let main application handle D3DERR_DEVICELOST by resetting the device.
    IM_ASSERT(hr == D3D_OK || hr == D3DERR_DEVICELOST);
}

pub unsafe fn ImGui_ImplDX9_InitPlatformInterface()
{
    ImGuiPlatformIO& platform_io = Imgui::GetPlatformIO();
    platform_io.Renderer_CreateWindow = ImGui_ImplDX9_CreateWindow;
    platform_io.Renderer_DestroyWindow = ImGui_ImplDX9_DestroyWindow;
    platform_io.Renderer_SetWindowSize = ImGui_ImplDX9_SetWindowSize;
    platform_io.Renderer_RenderWindow = ImGui_ImplDX9_RenderWindow;
    platform_io.Renderer_SwapBuffers = ImGui_ImplDX9_SwapBuffers;
}

pub unsafe fn ImGui_ImplDX9_ShutdownPlatformInterface()
{
    Imgui::DestroyPlatformWindows();
}

pub unsafe fn ImGui_ImplDX9_CreateDeviceObjectsForPlatformWindows()
{
    ImGuiPlatformIO& platform_io = Imgui::GetPlatformIO();
    for (int i = 1; i < platform_io.Viewports.Size; i++)
        if (!platform_io.Viewports[i]->RendererUserData)
            ImGui_ImplDX9_CreateWindow(platform_io.Viewports[i]);
}

pub unsafe fn ImGui_ImplDX9_InvalidateDeviceObjectsForPlatformWindows()
{
    ImGuiPlatformIO& platform_io = Imgui::GetPlatformIO();
    for (int i = 1; i < platform_io.Viewports.Size; i++)
        if (platform_io.Viewports[i]->RendererUserData)
            ImGui_ImplDX9_DestroyWindow(platform_io.Viewports[i]);
}
