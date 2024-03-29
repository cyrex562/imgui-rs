// dear imgui: Renderer for WebGPU
// This needs to be used along with a Platform Binding (e.g. GLFW)
// (Please note that WebGPU is currently experimental, will not run on non-beta browsers, and may break.)

// Implemented features:
//  [X] Renderer: User texture binding. Use 'WGPUTextureView' as ImTextureID. Read the FAQ about ImTextureID!
//  [X] Renderer: Large meshes support (64k+ vertices) with 16-bit indices.

// You can use unmodified imgui_impl_* files in your project. See examples/ folder for examples of using this.
// Prefer including the entire imgui/ repository into your project (either as a copy or as a submodule), and only build the backends you need.
// If you are new to Dear ImGui, read documentation from the docs/ folder + read the top of imgui.cpp.
// Read online: https://github.com/ocornut/imgui/tree/master/docs

// CHANGELOG
// (minor and older changes stripped away, please see git history for details)
//  2021-11-29: Passing explicit buffer sizes to wgpuRenderPassEncoderSetVertexBuffer()/wgpuRenderPassEncoderSetIndexBuffer().
//  2021-08-24: Fix for latest specs.
//  2021-05-24: Add support for draw_data.FramebufferScale.
//  2021-05-19: Replaced direct access to ImDrawCmd::TextureId with a call to ImDrawCmd::GetTexID(). (will become a requirement)
//  2021-05-16: Update to latest WebGPU specs (compatible with Emscripten 2.0.20 and Chrome Canary 92).
//  2021-02-18: Change blending equation to preserve alpha in output buffer.
//  2021-01-28: Initial version.

// #include "imgui.h"
// #include "imgui_impl_wgpu.h"
// #include <limits.h>
// #include <webgpu/webgpu.h>

// #define HAS_EMSCRIPTEN_VERSION(major, minor, tiny) (__EMSCRIPTEN_major__ > (major) || (__EMSCRIPTEN_major__ == (major) && __EMSCRIPTEN_minor__ > (minor)) || (__EMSCRIPTEN_major__ == (major) && __EMSCRIPTEN_minor__ == (minor) && __EMSCRIPTEN_tiny__ >= (tiny)))

// #if defined(__EMSCRIPTEN__) && !HAS_EMSCRIPTEN_VERSION(2, 0, 20)
// #error "Requires at least emscripten 2.0.20"
// #endif

// Dear ImGui prototypes from imgui_internal.h
// extern ImGuiID ImHashData(const void* data_p, size_t data_size, ImU32 seed = 0);

// WebGPU data
// static WGPUDevice               g_wgpuDevice = NULL;
// static WGPUQueue                g_defaultQueue = NULL;
// static WGPUTextureFormat        g_renderTargetFormat = WGPUTextureFormat_Undefined;
// static WGPURenderPipeline       g_pipelineState = NULL;

use std::borrow::Cow;
use std::ptr::null_mut;
use wgpu;
use crate::core::storage::ImGuiStorage;
use libc::c_void;
use wgpu::{BindGroup, BindGroupDescriptor, ShaderModuleDescriptorSpirV};
use crate::drawing::draw_data::ImDrawData;
use crate::drawing::draw_vert::ImguiDrawVertex;

#[derive(Default,Debug,Clone)]
pub struct RenderResources
{
    // WGPUTexture         FontTexture;            // Font texture
    pub FontTexture: wgpu::Texture,
    // WGPUTextureView     FontTextureView;        // Texture view for font texture
    pub FontTextureView: wgpu::TextureView,
    // WGPUSampler         Sampler;                // Sampler for the font texture
    pub Sampler: wgpu::Sampler,
    // WGPUBuffer          Uniforms;               // Shader uniforms
    pub Uniforms: wgpu::Buffer,
    // WGPUBindGroup       CommonBindGroup;        // Resources bind-group to bind the common resources to pipeline
    pub CommonBindGroup: wgpu::BindGroup,
    // ImGuiStorage        ImageBindGroups;        // Resources bind-group to bind the font/image resources to pipeline (this is a key.value map)
    pub ImageBindGroups: ImGuiStorage,
    // WGPUBindGroup       ImageBindGroup;         // Default font-resource of Dear ImGui
    pub ImageBindGroup: wgpu::BindGroup,
    // WGPUBindGroupLayout ImageBindGroupLayout;   // Cache layout used for the image bind group. Avoids allocating unnecessary JS objects when working with WebASM
    pub ImageBindGroupLayout: wgpu::BindGroupLayout,
}
// static RenderResources  g_resources;

#[derive(Default,Debug,Clone)]
pub struct FrameResources
{
    // WGPUBuffer  IndexBuffer;
    pub IndexBuffer: wgpu::Buffer,
    // WGPUBuffer  VertexBuffer;
    pub VertexBuffer: wgpu::Buffer,
    // ImDrawIdx*  IndexBufferHost;
    pub IndexBufferHost: *mut ImDrawIdx,
    // ImDrawVert* VertexBufferHost;
    pub VertexBufferHost: *mut ImguiDrawVertex,
    // int         IndexBufferSize;
    pub IndexBufferSize: i32,
    // int         VertexBufferSize;
    pub VertexBufferSize: i32,
}
// static FrameResources*  g_pFrameResources = NULL;
// static unsigned int     g_numFramesInFlight = 0;
// static unsigned int     g_frameIndex = UINT_MAX;
#[derive(Default,Debug,Clone)]
pub struct Uniforms
{
    // float MVP[4][4];
    pub MVP: [[f32;4];4]
}

//-----------------------------------------------------------------------------
// SHADERS
//-----------------------------------------------------------------------------

// glsl_shader.vert, compiled with:
// # glslangValidator -V -x -o glsl_shader.vert.u32 glsl_shader.vert
// /*
// #version 450 core
// layout(location = 0) in vec2 aPos;
// layout(location = 1) in vec2 aUV;
// layout(location = 2) in vec4 aColor;
// layout(set=0, binding = 0) uniform transform { mat4 mvp; };
//
// out gl_PerVertex { vec4 gl_Position; };
// layout(location = 0) out struct { vec4 Color; vec2 UV; } Out;
//
// void main()
// {
//     Out.Color = aColor;
//     Out.UV = aUV;
//     gl_Position = mvp * vec4(aPos, 0, 1);
// }
// */

pub const __glsl_shader_vert_spv: [u32;317] =
[
    0x07230203,0x00010000,0x00080007,0x0000002c,0x00000000,0x00020011,0x00000001,0x0006000b,
    0x00000001,0x4c534c47,0x6474732e,0x3035342e,0x00000000,0x0003000e,0x00000000,0x00000001,
    0x000a000f,0x00000000,0x00000004,0x6e69616d,0x00000000,0x0000000b,0x0000000f,0x00000015,
    0x0000001b,0x00000023,0x00030003,0x00000002,0x000001c2,0x00040005,0x00000004,0x6e69616d,
    0x00000000,0x00030005,0x00000009,0x00000000,0x00050006,0x00000009,0x00000000,0x6f6c6f43,
    0x00000072,0x00040006,0x00000009,0x00000001,0x00005655,0x00030005,0x0000000b,0x0074754f,
    0x00040005,0x0000000f,0x6c6f4361,0x0000726f,0x00030005,0x00000015,0x00565561,0x00060005,
    0x00000019,0x505f6c67,0x65567265,0x78657472,0x00000000,0x00060006,0x00000019,0x00000000,
    0x505f6c67,0x7469736f,0x006e6f69,0x00030005,0x0000001b,0x00000000,0x00050005,0x0000001d,
    0x6e617274,0x726f6673,0x0000006d,0x00040006,0x0000001d,0x00000000,0x0070766d,0x00030005,
    0x0000001f,0x00000000,0x00040005,0x00000023,0x736f5061,0x00000000,0x00040047,0x0000000b,
    0x0000001e,0x00000000,0x00040047,0x0000000f,0x0000001e,0x00000002,0x00040047,0x00000015,
    0x0000001e,0x00000001,0x00050048,0x00000019,0x00000000,0x0000000b,0x00000000,0x00030047,
    0x00000019,0x00000002,0x00040048,0x0000001d,0x00000000,0x00000005,0x00050048,0x0000001d,
    0x00000000,0x00000023,0x00000000,0x00050048,0x0000001d,0x00000000,0x00000007,0x00000010,
    0x00030047,0x0000001d,0x00000002,0x00040047,0x0000001f,0x00000022,0x00000000,0x00040047,
    0x0000001f,0x00000021,0x00000000,0x00040047,0x00000023,0x0000001e,0x00000000,0x00020013,
    0x00000002,0x00030021,0x00000003,0x00000002,0x00030016,0x00000006,0x00000020,0x00040017,
    0x00000007,0x00000006,0x00000004,0x00040017,0x00000008,0x00000006,0x00000002,0x0004001e,
    0x00000009,0x00000007,0x00000008,0x00040020,0x0000000a,0x00000003,0x00000009,0x0004003b,
    0x0000000a,0x0000000b,0x00000003,0x00040015,0x0000000c,0x00000020,0x00000001,0x0004002b,
    0x0000000c,0x0000000d,0x00000000,0x00040020,0x0000000e,0x00000001,0x00000007,0x0004003b,
    0x0000000e,0x0000000f,0x00000001,0x00040020,0x00000011,0x00000003,0x00000007,0x0004002b,
    0x0000000c,0x00000013,0x00000001,0x00040020,0x00000014,0x00000001,0x00000008,0x0004003b,
    0x00000014,0x00000015,0x00000001,0x00040020,0x00000017,0x00000003,0x00000008,0x0003001e,
    0x00000019,0x00000007,0x00040020,0x0000001a,0x00000003,0x00000019,0x0004003b,0x0000001a,
    0x0000001b,0x00000003,0x00040018,0x0000001c,0x00000007,0x00000004,0x0003001e,0x0000001d,
    0x0000001c,0x00040020,0x0000001e,0x00000002,0x0000001d,0x0004003b,0x0000001e,0x0000001f,
    0x00000002,0x00040020,0x00000020,0x00000002,0x0000001c,0x0004003b,0x00000014,0x00000023,
    0x00000001,0x0004002b,0x00000006,0x00000025,0x00000000,0x0004002b,0x00000006,0x00000026,
    0x3f800000,0x00050036,0x00000002,0x00000004,0x00000000,0x00000003,0x000200f8,0x00000005,
    0x0004003d,0x00000007,0x00000010,0x0000000f,0x00050041,0x00000011,0x00000012,0x0000000b,
    0x0000000d,0x0003003e,0x00000012,0x00000010,0x0004003d,0x00000008,0x00000016,0x00000015,
    0x00050041,0x00000017,0x00000018,0x0000000b,0x00000013,0x0003003e,0x00000018,0x00000016,
    0x00050041,0x00000020,0x00000021,0x0000001f,0x0000000d,0x0004003d,0x0000001c,0x00000022,
    0x00000021,0x0004003d,0x00000008,0x00000024,0x00000023,0x00050051,0x00000006,0x00000027,
    0x00000024,0x00000000,0x00050051,0x00000006,0x00000028,0x00000024,0x00000001,0x00070050,
    0x00000007,0x00000029,0x00000027,0x00000028,0x00000025,0x00000026,0x00050091,0x00000007,
    0x0000002a,0x00000022,0x00000029,0x00050041,0x00000011,0x0000002b,0x0000001b,0x0000000d,
    0x0003003e,0x0000002b,0x0000002a,0x000100fd,0x00010038
];

// glsl_shader.frag, compiled with:
// # glslangValidator -V -x -o glsl_shader.frag.u32 glsl_shader.frag
/*
#version 450 core
layout(location = 0) out vec4 fColor;
layout(set=0, binding=1) uniform sampler s;
layout(set=1, binding=0) uniform texture2D t;
layout(location = 0) in struct { vec4 Color; vec2 UV; } In;
void main()
{
    fColor = In.Color * texture(sampler2D(t, s), In.UV.st);
}
*/
pub const __glsl_shader_frag_spv: [u32;221] =
[
    0x07230203,0x00010000,0x00080007,0x00000023,0x00000000,0x00020011,0x00000001,0x0006000b,
    0x00000001,0x4c534c47,0x6474732e,0x3035342e,0x00000000,0x0003000e,0x00000000,0x00000001,
    0x0007000f,0x00000004,0x00000004,0x6e69616d,0x00000000,0x00000009,0x0000000d,0x00030010,
    0x00000004,0x00000007,0x00030003,0x00000002,0x000001c2,0x00040005,0x00000004,0x6e69616d,
    0x00000000,0x00040005,0x00000009,0x6c6f4366,0x0000726f,0x00030005,0x0000000b,0x00000000,
    0x00050006,0x0000000b,0x00000000,0x6f6c6f43,0x00000072,0x00040006,0x0000000b,0x00000001,
    0x00005655,0x00030005,0x0000000d,0x00006e49,0x00030005,0x00000015,0x00000074,0x00030005,
    0x00000019,0x00000073,0x00040047,0x00000009,0x0000001e,0x00000000,0x00040047,0x0000000d,
    0x0000001e,0x00000000,0x00040047,0x00000015,0x00000022,0x00000001,0x00040047,0x00000015,
    0x00000021,0x00000000,0x00040047,0x00000019,0x00000022,0x00000000,0x00040047,0x00000019,
    0x00000021,0x00000001,0x00020013,0x00000002,0x00030021,0x00000003,0x00000002,0x00030016,
    0x00000006,0x00000020,0x00040017,0x00000007,0x00000006,0x00000004,0x00040020,0x00000008,
    0x00000003,0x00000007,0x0004003b,0x00000008,0x00000009,0x00000003,0x00040017,0x0000000a,
    0x00000006,0x00000002,0x0004001e,0x0000000b,0x00000007,0x0000000a,0x00040020,0x0000000c,
    0x00000001,0x0000000b,0x0004003b,0x0000000c,0x0000000d,0x00000001,0x00040015,0x0000000e,
    0x00000020,0x00000001,0x0004002b,0x0000000e,0x0000000f,0x00000000,0x00040020,0x00000010,
    0x00000001,0x00000007,0x00090019,0x00000013,0x00000006,0x00000001,0x00000000,0x00000000,
    0x00000000,0x00000001,0x00000000,0x00040020,0x00000014,0x00000000,0x00000013,0x0004003b,
    0x00000014,0x00000015,0x00000000,0x0002001a,0x00000017,0x00040020,0x00000018,0x00000000,
    0x00000017,0x0004003b,0x00000018,0x00000019,0x00000000,0x0003001b,0x0000001b,0x00000013,
    0x0004002b,0x0000000e,0x0000001d,0x00000001,0x00040020,0x0000001e,0x00000001,0x0000000a,
    0x00050036,0x00000002,0x00000004,0x00000000,0x00000003,0x000200f8,0x00000005,0x00050041,
    0x00000010,0x00000011,0x0000000d,0x0000000f,0x0004003d,0x00000007,0x00000012,0x00000011,
    0x0004003d,0x00000013,0x00000016,0x00000015,0x0004003d,0x00000017,0x0000001a,0x00000019,
    0x00050056,0x0000001b,0x0000001c,0x00000016,0x0000001a,0x00050041,0x0000001e,0x0000001f,
    0x0000000d,0x0000001d,0x0004003d,0x0000000a,0x00000020,0x0000001f,0x00050057,0x00000007,
    0x00000021,0x0000001c,0x00000020,0x00050085,0x00000007,0x00000022,0x00000012,0x00000021,
    0x0003003e,0x00000009,0x00000022,0x000100fd,0x00010038
];

pub fn safe_release_imgui_draw_idx(res: *mut ImDrawIdx)
{
    if res.is_null() == false {
        unsafe { libc::free(res); }
    }
    // res = null_mut();
}

pub fn safe_rlease_imgui_draw_vert(res: *mut ImguiDrawVertex)
{
    if res.is_null() == false {
        unsafe { libc::free(res as *mut c_void); }
    }
    // res = null_mut();
}

pub fn safe_release_wgpu_bind_grp_layout(res: *mut wgpu::BindGroupLayout) {
    if res.is_null() == false {
        unsafe { libc::free(res as *mut c_void); }
    }
    // res = null_mut();
}

pub fn safe_release_wgpu_bind_group(res: *mut wgpu::BindGroup) {
    if res.is_null() == false {
        unsafe { libc::free(res as *mut c_void); }
    }
    // res = null_mut();
}

pub fn safe_release_wgpu_buffer(res: *mut wgpu::Buffer) {
    if res.is_null() == false {
        unsafe { libc::free(res as *mut c_void); }
    }
    // res = null_mut();
}

pub fn safe_release_wgpu_render_pipeline(res: *mut wgpu::RenderPipeline) {
    if res.is_null() == false {
        unsafe { libc::free(res as *mut c_void); }
    }
    // res = null_mut();
}

pub fn safe_release_wgpu_sample(res: *mut wgpu::Sampler) {
    if res.is_null() == false {
        unsafe { libc::free(res as *mut c_void); }
    }
    // res = null_mut();
}

pub fn safe_release_wgpu_shader_module(res: *mut wgpu::ShaderModule) {
    if res.is_null() == false {
        unsafe { libc::free(res as *mut c_void); }
    }
    // res = null_mut();
}

pub fn safe_release_wgpu_texture_view(res: *mut wgpu::TextureView) {
    if res.is_null() == false {
        unsafe { libc::free(res as *mut c_void); }
    }
}

pub fn safe_release_wgpu_texture(res: *mut wgpu::Texture) {
    if res.is_null() == false {
        unsafe { libc::free(res as *mut c_void); }
    }
}

pub fn safe_release_render_resources(res: *mut RenderResources) {
    // SafeRelease(res.FontTexture);
    //     SafeRelease(res.FontTextureView);
    //     SafeRelease(res.Sampler);
    //     SafeRelease(res.Uniforms);
    //     SafeRelease(res.CommonBindGroup);
    //     SafeRelease(res.ImageBindGroup);
    //     SafeRelease(res.ImageBindGroupLayout);
}

pub fn safe_release_frame_resources(res: *mut FrameResources) {
    // SafeRelease(res.IndexBuffer);
    //     SafeRelease(res.VertexBuffer);
    //     SafeRelease(res.IndexBufferHost);
    //     SafeRelease(res.VertexBufferHost);
}

pub fn ImGui_ImplWGPU_CreateShaderModule(device: &mut wgpu::Device, binary_data: *mut u32, binary_data_size: u32) -> wgpu::ShaderModule
{
    // let mut spirv_desc: wgpu::ShaderModuleDescriptorSpirV = wgpu::ShaderModuleDescriptorSpirV{ label: None, source: Default::default() };
    // spirv_desc.chain.sType = WGPUSType_ShaderModuleSPIRVDescriptor;
    // spirv_desc.codeSize = binary_data_size;
    // spirv_desc.code = binary_data;
    //
    // WGPUShaderModuleDescriptor desc = {};
    // desc.nextInChain = reinterpret_cast<WGPUChainedStruct*>(&spirv_desc);
    //
    // WGPUProgrammableStageDescriptor stage_desc = {};
    // stage_desc.module = wgpu::Device::create_shader_module(g_wgpuDevice, &desc);
    // stage_desc.entryPoint = "main";
    // return stage_desc;
    let mut desc: wgpu::ShaderModuleDescriptorSpirV = wgpu::ShaderModuleDescriptorSpirV {
        label: None,
        source: Cow::from(binary_data),
    };
    let out = unsafe{device::create_shader_module_spirv(desc)};
    out
}

pub fn ImGui_ImplWGPU_CreateImageBindGroup(device: &mut wgpu::Device, layout: wgpu::BindGroupLayout, texture: wgpu::TextureView) -> wgpu::BindGroup
{
     let mut entries: [wgpu::BindGroupEntry;1] = [ wgpu::BindGroupEntry{ binding: 0, resource: wgpu::BindingResource::TextureView(&texture) } ];
    let descriptor: wgpu::BindGroupDescriptor = wgpu::BindGroupDescriptor{
        label: None,
        layout: &layout,
        entries: &entries,
    };
    return device::create_bind_group(&descriptor);
}

pub fn ImGui_ImplWGPU_SetupRenderState(
    draw_data: *mut ImDrawData,
    ctx: wgpu::util::RenderEncoder,
    fr: *mut FrameResources)
{
    // Setup orthographic projection matrix into our constant buffer
    // Our visible imgui space lies from draw_data.DisplayPos (top left) to draw_data.DisplayPos+data_data.DisplaySize (bottom right).
    {
        let mut L = draw_data.DisplayPos.x;
        let mut R = draw_data.DisplayPos.x + draw_data.DisplaySize.x;
        let mut T = draw_data.DisplayPos.y;
        let mut B = draw_data.DisplayPos.y + draw_data.DisplaySize.y;
        let mut mvp: [[f32;4];4] =
        [
            [ 2.0f32/(R-L),   0.0f32,           0.0f32,       0.0f32 ],
            [ 0.0f32,         2.0f32/(T-B),     0.0f32,       0.0f32 ],
            [ 0.0f32,         0.0f32,           0.5f32,       0.0f32 ],
            [ (R+L)/(L-R),  (T+B)/(B-T),    0.5f32,       1.0f32 ],
        ];
        wgpu::QueueWriteBuffer(g_defaultQueue, g_resources.Uniforms, 0, mvp, sizeof(mvp));
    }

    // Setup viewport
    wgpu::RenderPassEncoderSetViewport(ctx, 0, 0, draw_data.FramebufferScale.x * draw_data.DisplaySize.x, draw_data.FramebufferScale.y * draw_data.DisplaySize.y, 0, 1);

    // Bind shader and vertex buffers
    wgpu::util::RenderEncoder::set_vertex_buffer(0, fr.VertexBuffer, 0, fr.VertexBufferSize * sizeof(ImguiDrawVertex));
    wgpu::RenderPassEncoderSetIndexBuffer(ctx, fr.IndexBuffer, sizeof(ImDrawIdx) == 2 ? WGPUIndexFormat_Uint16 : WGPUIndexFormat_Uint32, 0, fr.IndexBufferSize * sizeof(ImDrawIdx));
    wgpu::RenderPassEncoderSetPipeline(ctx, g_pipelineState);
    wgpu::RenderPassEncoderSetBindGroup(ctx, 0, g_resources.CommonBindGroup, 0, NULL);

    // Setup blend factor
    WGPUColor blend_color = { 0.f, 0.f, 0.f, 0.f };
    wgpuRenderPassEncoderSetBlendConstant(ctx, &blend_color);
}

// Render function
// (this used to be set in io.RenderDrawListsFn and called by ImGui::Render(), but you can now call this directly from your main loop)
void ImGui_ImplWGPU_RenderDrawData(ImDrawData* draw_data, WGPURenderPassEncoder pass_encoder)
{
    // Avoid rendering when minimized
    if (draw_data.DisplaySize.x <= 0.0f || draw_data.DisplaySize.y <= 0.0f)
        return;

    // FIXME: Assuming that this only gets called once per frame!
    // If not, we can't just re-allocate the IB or VB, we'll have to do a proper allocator.
    g_frameIndex = g_frameIndex + 1;
    FrameResources* fr = &g_pFrameResources[g_frameIndex % g_numFramesInFlight];

    // Create and grow vertex/index buffers if needed
    if (fr.VertexBuffer == NULL || fr.VertexBufferSize < draw_data.TotalVtxCount)
    {
        if (fr.VertexBuffer)
        {
            wgpuBufferDestroy(fr.VertexBuffer);
            wgpuBufferRelease(fr.VertexBuffer);
        }
        SafeRelease(fr.VertexBufferHost);
        fr.VertexBufferSize = draw_data.TotalVtxCount + 5000;

        WGPUBufferDescriptor vb_desc =
        {
            NULL,
            "Dear ImGui Vertex buffer",
            WGPUBufferUsage_CopyDst | WGPUBufferUsage_Vertex,
            fr.VertexBufferSize * sizeof(ImguiDrawVertex),
            false
        };
        fr.VertexBuffer = wgpuDeviceCreateBuffer(g_wgpuDevice, &vb_desc);
        if (!fr.VertexBuffer)
            return;

        fr.VertexBufferHost = new ImguiDrawVertex[fr.VertexBufferSize];
    }
    if (fr.IndexBuffer == NULL || fr.IndexBufferSize < draw_data.TotalIdxCount)
    {
        if (fr.IndexBuffer)
        {
            wgpuBufferDestroy(fr.IndexBuffer);
            wgpuBufferRelease(fr.IndexBuffer);
        }
        SafeRelease(fr.IndexBufferHost);
        fr.IndexBufferSize = draw_data.TotalIdxCount + 10000;

        WGPUBufferDescriptor ib_desc =
        {
            NULL,
            "Dear ImGui Index buffer",
            WGPUBufferUsage_CopyDst | WGPUBufferUsage_Index,
            fr.IndexBufferSize * sizeof(ImDrawIdx),
            false
        };
        fr.IndexBuffer = wgpuDeviceCreateBuffer(g_wgpuDevice, &ib_desc);
        if (!fr.IndexBuffer)
            return;

        fr.IndexBufferHost = new ImDrawIdx[fr.IndexBufferSize];
    }

    // Upload vertex/index data into a single contiguous GPU buffer
    ImguiDrawVertex* vtx_dst = (ImguiDrawVertex*)fr.VertexBufferHost;
    ImDrawIdx* idx_dst = (ImDrawIdx*)fr.IndexBufferHost;
    for (int n = 0; n < draw_data.CmdListsCount; n++)
    {
        const ImDrawList* cmd_list = draw_data.CmdLists[n];
        memcpy(vtx_dst, cmd_list.VtxBuffer.Data, cmd_list.VtxBuffer.Size * sizeof(ImguiDrawVertex));
        memcpy(idx_dst, cmd_list.IdxBuffer.Data, cmd_list.IdxBuffer.Size * sizeof(ImDrawIdx));
        vtx_dst += cmd_list.VtxBuffer.Size;
        idx_dst += cmd_list.IdxBuffer.Size;
    }
    int64_t vb_write_size = ((char*)vtx_dst - (char*)fr.VertexBufferHost + 3) & ~3;
    int64_t ib_write_size = ((char*)idx_dst - (char*)fr.IndexBufferHost  + 3) & ~3;
    wgpuQueueWriteBuffer(g_defaultQueue, fr.VertexBuffer, 0, fr.VertexBufferHost, vb_write_size);
    wgpuQueueWriteBuffer(g_defaultQueue, fr.IndexBuffer,  0, fr.IndexBufferHost,  ib_write_size);

    // Setup desired render state
    ImGui_ImplWGPU_SetupRenderState(draw_data, pass_encoder, fr);

    // Render command lists
    // (Because we merged all buffers into a single one, we maintain our own offset into them)
    int global_vtx_offset = 0;
    int global_idx_offset = 0;
    clip_scale: ImVec2 = draw_data.FramebufferScale;
    clip_off: ImVec2 = draw_data.DisplayPos;
    for (int n = 0; n < draw_data.CmdListsCount; n++)
    {
        const ImDrawList* cmd_list = draw_data.CmdLists[n];
        for (int cmd_i = 0; cmd_i < cmd_list.CmdBuffer.Size; cmd_i++)
        {
            const ImDrawCmd* pcmd = &cmd_list.CmdBuffer[cmd_i];
            if (pcmd.UserCallback != NULL)
            {
                // User callback, registered via ImDrawList::AddCallback()
                // (ImDrawCallback_ResetRenderState is a special callback value used by the user to request the renderer to reset render state.)
                if (pcmd.UserCallback == ImDrawCallback_ResetRenderState)
                    ImGui_ImplWGPU_SetupRenderState(draw_data, pass_encoder, fr);
                else
                    pcmd.UserCallback(cmd_list, pcmd);
            }
            else
            {
                // Bind custom texture
                ImTextureID tex_id = pcmd.GetTexID();
                ImGuiID tex_id_hash = ImHashData(&tex_id, sizeof(tex_id));
                auto bind_group = g_resources.ImageBindGroups.GetVoidPtr(tex_id_hash);
                if (bind_group)
                {
                    wgpuRenderPassEncoderSetBindGroup(pass_encoder, 1, (WGPUBindGroup)bind_group, 0, NULL);
                }
                else
                {
                    WGPUBindGroup image_bind_group = ImGui_ImplWGPU_CreateImageBindGroup(g_resources.ImageBindGroupLayout, (WGPUTextureView)tex_id);
                    g_resources.ImageBindGroups.SetVoidPtr(tex_id_hash, image_bind_group);
                    wgpuRenderPassEncoderSetBindGroup(pass_encoder, 1, image_bind_group, 0, NULL);
                }

                // Project scissor/clipping rectangles into framebuffer space
                clip_min: ImVec2((pcmd.ClipRect.x - clip_off.x) * clip_scale.x, (pcmd.ClipRect.y - clip_off.y) * clip_scale.y);
                clip_max: ImVec2((pcmd.ClipRect.z - clip_off.x) * clip_scale.x, (pcmd.ClipRect.w - clip_off.y) * clip_scale.y);
                if (clip_max.x <= clip_min.x || clip_max.y <= clip_min.y)
                    continue;

                // Apply scissor/clipping rectangle, Draw
                wgpuRenderPassEncoderSetScissorRect(pass_encoder, clip_min.x, clip_min.y, (clip_max.x - clip_min.x), (clip_max.y - clip_min.y));
                wgpuRenderPassEncoderDrawIndexed(pass_encoder, pcmd.ElemCount, 1, pcmd.IdxOffset + global_idx_offset, pcmd.VtxOffset + global_vtx_offset, 0);
            }
        }
        global_idx_offset += cmd_list.IdxBuffer.Size;
        global_vtx_offset += cmd_list.VtxBuffer.Size;
    }
}

static void ImGui_ImplWGPU_CreateFontsTexture()
{
    // Build texture atlas
    ImGuiIO& io = Imgui::GetIO();
    unsigned char* pixels;
    int width, height, size_pp;
    io.Fonts.GetTexDataAsRGBA32(&pixels, &width, &height, &size_pp);

    // Upload texture to graphics system
    {
        WGPUTextureDescriptor tex_desc = {};
        tex_desc.label = "Dear ImGui Font Texture";
        tex_desc.dimension = WGPUTextureDimension_2D;
        tex_desc.size.width = width;
        tex_desc.size.height = height;
        tex_desc.size.depthOrArrayLayers = 1;
        tex_desc.sampleCount = 1;
        tex_desc.format = WGPUTextureFormat_RGBA8Unorm;
        tex_desc.mipLevelCount = 1;
        tex_desc.usage = WGPUTextureUsage_CopyDst | WGPUTextureUsage_TextureBinding;
        g_resources.FontTexture = wgpuDeviceCreateTexture(g_wgpuDevice, &tex_desc);

        WGPUTextureViewDescriptor tex_view_desc = {};
        tex_view_desc.format = WGPUTextureFormat_RGBA8Unorm;
        tex_view_desc.dimension = WGPUTextureViewDimension_2D;
        tex_view_desc.baseMipLevel = 0;
        tex_view_desc.mipLevelCount = 1;
        tex_view_desc.baseArrayLayer = 0;
        tex_view_desc.arrayLayerCount = 1;
        tex_view_desc.aspect = WGPUTextureAspect_All;
        g_resources.FontTextureView = wgpuTextureCreateView(g_resources.FontTexture, &tex_view_desc);
    }

    // Upload texture data
    {
        WGPUImageCopyTexture dst_view = {};
        dst_view.texture = g_resources.FontTexture;
        dst_view.mipLevel = 0;
        dst_view.origin = { 0, 0, 0 };
        dst_view.aspect = WGPUTextureAspect_All;
        WGPUTextureDataLayout layout = {};
        layout.offset = 0;
        layout.bytesPerRow = width * size_pp;
        layout.rowsPerImage = height;
        WGPUExtent3D size = { width, height, 1 };
        wgpuQueueWriteTexture(g_defaultQueue, &dst_view, pixels, (width * size_pp * height), &layout, &size);
    }

    // Create the associated sampler
    // (Bilinear sampling is required by default. Set 'io.Fonts.Flags |= ImFontAtlasFlags_NoBakedLines' or 'style.AntiAliasedLinesUseTex = false' to allow point/nearest sampling)
    {
        WGPUSamplerDescriptor sampler_desc = {};
        sampler_desc.minFilter = WGPUFilterMode_Linear;
        sampler_desc.magFilter = WGPUFilterMode_Linear;
        sampler_desc.mipmapFilter = WGPUFilterMode_Linear;
        sampler_desc.addressModeU = WGPUAddressMode_Repeat;
        sampler_desc.addressModeV = WGPUAddressMode_Repeat;
        sampler_desc.addressModeW = WGPUAddressMode_Repeat;
        sampler_desc.maxAnisotropy = 1;
        g_resources.Sampler = wgpuDeviceCreateSampler(g_wgpuDevice, &sampler_desc);
    }

    // Store our identifier
    static_assert(sizeof(ImTextureID) >= sizeof(g_resources.FontTexture), "Can't pack descriptor handle into TexID, 32-bit not supported yet.");
    io.Fonts.SetTexID((ImTextureID)g_resources.FontTextureView);
}

static void ImGui_ImplWGPU_CreateUniformBuffer()
{
    WGPUBufferDescriptor ub_desc =
    {
        NULL,
        "Dear ImGui Uniform buffer",
        WGPUBufferUsage_CopyDst | WGPUBufferUsage_Uniform,
        sizeof(Uniforms),
        false
    };
    g_resources.Uniforms = wgpuDeviceCreateBuffer(g_wgpuDevice, &ub_desc);
}

bool ImGui_ImplWGPU_CreateDeviceObjects()
{
    if (!g_wgpuDevice)
        return false;
    if (g_pipelineState)
        ImGui_ImplWGPU_InvalidateDeviceObjects();

    // Create render pipeline
    WGPURenderPipelineDescriptor graphics_pipeline_desc = {};
    graphics_pipeline_desc.primitive.topology = WGPUPrimitiveTopology_TriangleList;
    graphics_pipeline_desc.primitive.stripIndexFormat = WGPUIndexFormat_Undefined;
    graphics_pipeline_desc.primitive.frontFace = WGPUFrontFace_CW;
    graphics_pipeline_desc.primitive.cullMode = WGPUCullMode_None;
    graphics_pipeline_desc.multisample.count = 1;
    graphics_pipeline_desc.multisample.mask = UINT_MAX;
    graphics_pipeline_desc.multisample.alphaToCoverageEnabled = false;
    graphics_pipeline_desc.layout = nullptr; // Use automatic layout generation

    // Create the vertex shader
    WGPUProgrammableStageDescriptor vertex_shader_desc = ImGui_ImplWGPU_CreateShaderModule(__glsl_shader_vert_spv, sizeof(__glsl_shader_vert_spv) / sizeof);
    graphics_pipeline_desc.vertex.module = vertex_shader_desc.module;
    graphics_pipeline_desc.vertex.entryPoint = vertex_shader_desc.entryPoint;

    // Vertex input configuration
    WGPUVertexAttribute attribute_desc[] =
    {
        { WGPUVertexFormat_Float32x2, (uint64_t)IM_OFFSETOF(ImguiDrawVertex, pos), 0 },
        { WGPUVertexFormat_Float32x2, (uint64_t)IM_OFFSETOF(ImguiDrawVertex, uv),  1 },
        { WGPUVertexFormat_Unorm8x4,  (uint64_t)IM_OFFSETOF(ImguiDrawVertex, col), 2 },
    };

    WGPUVertexBufferLayout buffer_layouts[1];
    buffer_layouts[0].arrayStride = sizeof(ImguiDrawVertex);
    buffer_layouts[0].stepMode = WGPUVertexStepMode_Vertex;
    buffer_layouts[0].attributeCount = 3;
    buffer_layouts[0].attributes = attribute_desc;

    graphics_pipeline_desc.vertex.bufferCount = 1;
    graphics_pipeline_desc.vertex.buffers = buffer_layouts;

    // Create the pixel shader
    WGPUProgrammableStageDescriptor pixel_shader_desc = ImGui_ImplWGPU_CreateShaderModule(__glsl_shader_frag_spv, sizeof(__glsl_shader_frag_spv) / sizeof);

    // Create the blending setup
    WGPUBlendState blend_state = {};
    blend_state.alpha.operation = WGPUBlendOperation_Add;
    blend_state.alpha.srcFactor = WGPUBlendFactor_One;
    blend_state.alpha.dstFactor = WGPUBlendFactor_OneMinusSrcAlpha;
    blend_state.color.operation = WGPUBlendOperation_Add;
    blend_state.color.srcFactor = WGPUBlendFactor_SrcAlpha;
    blend_state.color.dstFactor = WGPUBlendFactor_OneMinusSrcAlpha;

    WGPUColorTargetState color_state = {};
    color_state.format = g_renderTargetFormat;
    color_state.blend = &blend_state;
    color_state.writeMask = WGPUColorWriteMask_All;

    WGPUFragmentState fragment_state = {};
    fragment_state.module = pixel_shader_desc.module;
    fragment_state.entryPoint = pixel_shader_desc.entryPoint;
    fragment_state.targetCount = 1;
    fragment_state.targets = &color_state;

    graphics_pipeline_desc.fragment = &fragment_state;

    // Create depth-stencil State
    WGPUDepthStencilState depth_stencil_state = {};
    depth_stencil_state.depthBias = 0;
    depth_stencil_state.depthBiasClamp = 0;
    depth_stencil_state.depthBiasSlopeScale = 0;

    // Configure disabled depth-stencil state
    graphics_pipeline_desc.depthStencil = nullptr;

    g_pipelineState = wgpuDeviceCreateRenderPipeline(g_wgpuDevice, &graphics_pipeline_desc);

    ImGui_ImplWGPU_CreateFontsTexture();
    ImGui_ImplWGPU_CreateUniformBuffer();

    // Create resource bind group
    WGPUBindGroupLayout bg_layouts[2];
    bg_layouts[0] = wgpuRenderPipelineGetBindGroupLayout(g_pipelineState, 0);
    bg_layouts[1] = wgpuRenderPipelineGetBindGroupLayout(g_pipelineState, 1);

    WGPUBindGroupEntry common_bg_entries[] =
    {
        { nullptr, 0, g_resources.Uniforms, 0, sizeof(Uniforms), 0, 0 },
        { nullptr, 1, 0, 0, 0, g_resources.Sampler, 0 },
    };

    WGPUBindGroupDescriptor common_bg_descriptor = {};
    common_bg_descriptor.layout = bg_layouts[0];
    common_bg_descriptor.entryCount = sizeof(common_bg_entries) / sizeof(WGPUBindGroupEntry);
    common_bg_descriptor.entries = common_bg_entries;
    g_resources.CommonBindGroup = wgpuDeviceCreateBindGroup(g_wgpuDevice, &common_bg_descriptor);

    WGPUBindGroup image_bind_group = ImGui_ImplWGPU_CreateImageBindGroup(bg_layouts[1], g_resources.FontTextureView);
    g_resources.ImageBindGroup = image_bind_group;
    g_resources.ImageBindGroupLayout = bg_layouts[1];
    g_resources.ImageBindGroups.SetVoidPtr(ImHashData(&g_resources.FontTextureView, sizeof(ImTextureID)), image_bind_group);

    SafeRelease(vertex_shader_desc.module);
    SafeRelease(pixel_shader_desc.module);
    SafeRelease(bg_layouts[0]);

    return true;
}

void ImGui_ImplWGPU_InvalidateDeviceObjects()
{
    if (!g_wgpuDevice)
        return;

    SafeRelease(g_pipelineState);
    SafeRelease(g_resources);

    ImGuiIO& io = Imgui::GetIO();
    io.Fonts.SetTexID(NULL); // We copied g_pFontTextureView to io.Fonts.TexID so let's clear that as well.

    for (unsigned int i = 0; i < g_numFramesInFlight; i++)
        SafeRelease(g_pFrameResources[i]);
}

bool ImGui_ImplWGPU_Init(WGPUDevice device, int num_frames_in_flight, WGPUTextureFormat rt_format)
{
    // Setup backend capabilities flags
    ImGuiIO& io = Imgui::GetIO();
    io.BackendRendererName = "imgui_impl_webgpu";
    io.BackendFlags |= IM_GUI_BACKEND_FLAGS_RENDERER_HAS_VTX_OFFSET;  // We can honor the ImDrawCmd::VtxOffset field, allowing for large meshes.

    g_wgpuDevice = device;
    g_defaultQueue = wgpuDeviceGetQueue(g_wgpuDevice);
    g_renderTargetFormat = rt_format;
    g_pFrameResources = new FrameResources[num_frames_in_flight];
    g_numFramesInFlight = num_frames_in_flight;
    g_frameIndex = UINT_MAX;

    g_resources.FontTexture = NULL;
    g_resources.FontTextureView = NULL;
    g_resources.Sampler = NULL;
    g_resources.Uniforms = NULL;
    g_resources.CommonBindGroup = NULL;
    g_resources.ImageBindGroups.Data.reserve(100);
    g_resources.ImageBindGroup = NULL;
    g_resources.ImageBindGroupLayout = NULL;

    // Create buffers with a default size (they will later be grown as needed)
    for (int i = 0; i < num_frames_in_flight; i++)
    {
        FrameResources* fr = &g_pFrameResources[i];
        fr.IndexBuffer = NULL;
        fr.VertexBuffer = NULL;
        fr.IndexBufferHost = NULL;
        fr.VertexBufferHost = NULL;
        fr.IndexBufferSize = 10000;
        fr.VertexBufferSize = 5000;
    }

    return true;
}

void ImGui_ImplWGPU_Shutdown()
{
    ImGui_ImplWGPU_InvalidateDeviceObjects();
    delete[] g_pFrameResources;
    g_pFrameResources = NULL;
    wgpuQueueRelease(g_defaultQueue);
    g_wgpuDevice = NULL;
    g_numFramesInFlight = 0;
    g_frameIndex = UINT_MAX;
}

void ImGui_ImplWGPU_NewFrame()
{
    if (!g_pipelineState)
        ImGui_ImplWGPU_CreateDeviceObjects();
}
