// dear imgui: Renderer Backend for Vulkan
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
// This needs to be used along with a Platform Backend (e.g. GLFW, SDL, Win32, custom..)

// Implemented features:
//  [X] Renderer: Large meshes support (64k+ vertices) with 16-bit indices.
//  [x] Renderer: Multi-viewport / platform windows. With issues (flickering when creating a new viewport).
//  [!] Renderer: User texture binding. Use 'VkDescriptorSet' as ImTextureID. Read the FAQ about ImTextureID! See https://github.com/ocornut/imgui/pull/914 for discussions.

// Important: on 32-bit systems, user texture binding is only supported if your imconfig file has '#define ImTextureID ImU64'.
// See imgui_impl_vulkan.cpp file for details.

// You can use unmodified imgui_impl_* files in your project. See examples/ folder for examples of using this.
// Prefer including the entire imgui/ repository into your project (either as a copy or as a submodule), and only build the backends you need.
// If you are new to Dear ImGui, read documentation from the docs/ folder + read the top of imgui.cpp.
// Read online: https://github.com/ocornut/imgui/tree/master/docs

// The aim of imgui_impl_vulkan.h/.cpp is to be usable in your engine without any modification.
// IF YOU FEEL YOU NEED TO MAKE ANY CHANGE TO THIS CODE, please share them and your feedback at https://github.com/ocornut/imgui/

// Important note to the reader who wish to integrate imgui_impl_vulkan.cpp/.h in their own engine/app.
// - Common ImGui_ImplVulkan_XXX functions and structures are used to interface with imgui_impl_vulkan.cpp/.h.
//   You will use those if you want to use this rendering backend in your engine/app.
// - Helper ImGui_ImplVulkanH_XXX functions and structures are only used by this example (main.cpp) and by
//   the backend itself (imgui_impl_vulkan.cpp), but should PROBABLY NOT be used by your own engine/app code.
// Read comments in imgui_impl_vulkan.h.

// #pragma once
// #include "imgui.h"      // IMGUI_IMPL_API


// #ifdef VK_NO_PROTOTYPES
// static bool g_FunctionsLoaded = false;
// #else
// static bool g_FunctionsLoaded = true;
// #endif
// #ifdef VK_NO_PROTOTYPES
// #define IMGUI_VULKAN_FUNC_MAP(IMGUI_VULKAN_FUNC_MAP_MACRO) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkAllocateCommandBuffers) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkAllocateDescriptorSets) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkAllocateMemory) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkBindBufferMemory) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkBindImageMemory) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkCmdBindDescriptorSets) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkCmdBindIndexBuffer) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkCmdBindPipeline) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkCmdBindVertexBuffers) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkCmdCopyBufferToImage) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkCmdDrawIndexed) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkCmdPipelineBarrier) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkCmdPushConstants) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkCmdSetScissor) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkCmdSetViewport) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkCreateBuffer) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkCreateCommandPool) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkCreateDescriptorSetLayout) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkCreateFence) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkCreateFramebuffer) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkCreateGraphicsPipelines) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkCreateImage) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkCreateImageView) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkCreatePipelineLayout) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkCreateRenderPass) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkCreateSampler) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkCreateSemaphore) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkCreateShaderModule) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkCreateSwapchainKHR) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkDestroyBuffer) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkDestroyCommandPool) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkDestroyDescriptorSetLayout) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkDestroyFence) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkDestroyFramebuffer) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkDestroyImage) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkDestroyImageView) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkDestroyPipeline) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkDestroyPipelineLayout) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkDestroyRenderPass) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkDestroySampler) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkDestroySemaphore) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkDestroyShaderModule) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkDestroySurfaceKHR) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkDestroySwapchainKHR) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkDeviceWaitIdle) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkFlushMappedMemoryRanges) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkFreeCommandBuffers) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkFreeMemory) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkGetBufferMemoryRequirements) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkGetImageMemoryRequirements) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkGetPhysicalDeviceMemoryProperties) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkGetPhysicalDeviceSurfaceCapabilitiesKHR) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkGetPhysicalDeviceSurfaceFormatsKHR) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkGetPhysicalDeviceSurfacePresentModesKHR) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkGetSwapchainImagesKHR) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkMapMemory) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkUnmapMemory) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkUpdateDescriptorSets) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkGetPhysicalDeviceSurfaceSupportKHR) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkWaitForFences) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkCmdBeginRenderPass) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkCmdEndRenderPass) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkQueuePresentKHR) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkBeginCommandBuffer) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkEndCommandBuffer) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkResetFences) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkQueueSubmit) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkResetCommandPool) \
//     IMGUI_VULKAN_FUNC_MAP_MACRO(vkAcquireNextImageKHR)

use std::ptr::null_mut;
use ash::prelude::VkResult;
use ash::vk;
use sdl2::sys::Font;
use crate::drawing::draw_data::ImDrawData;
use crate::io::io_ops::GetIO;
use libc::{c_char, c_void};
use crate::backends::backend_flags::{IM_GUI_BACKEND_FLAGS_RENDERER_HAS_VIEWPORTS, IM_GUI_BACKEND_FLAGS_RENDERER_HAS_VTX_OFFSET};
use crate::core::config_flags::ImGuiConfigFlags_ViewportsEnable;
use crate::core::context::AppContext;
use crate::core::type_defs::ImTextureID;
use crate::core::vec4::ImVec4;
use crate::viewport::viewport_flags::ImGuiViewportFlags_NoRendererClear;
use crate::viewport::viewport_ops::{DestroyPlatformWindows, GetMainViewport};

pub const __glsl_shader_vert_spv: [u32;324] =
[
    0x07230203,0x00010000,0x00080001,0x0000002e,0x00000000,0x00020011,0x00000001,0x0006000b,
    0x00000001,0x4c534c47,0x6474732e,0x3035342e,0x00000000,0x0003000e,0x00000000,0x00000001,
    0x000a000f,0x00000000,0x00000004,0x6e69616d,0x00000000,0x0000000b,0x0000000f,0x00000015,
    0x0000001b,0x0000001c,0x00030003,0x00000002,0x000001c2,0x00040005,0x00000004,0x6e69616d,
    0x00000000,0x00030005,0x00000009,0x00000000,0x00050006,0x00000009,0x00000000,0x6f6c6f43,
    0x00000072,0x00040006,0x00000009,0x00000001,0x00005655,0x00030005,0x0000000b,0x0074754f,
    0x00040005,0x0000000f,0x6c6f4361,0x0000726f,0x00030005,0x00000015,0x00565561,0x00060005,
    0x00000019,0x505f6c67,0x65567265,0x78657472,0x00000000,0x00060006,0x00000019,0x00000000,
    0x505f6c67,0x7469736f,0x006e6f69,0x00030005,0x0000001b,0x00000000,0x00040005,0x0000001c,
    0x736f5061,0x00000000,0x00060005,0x0000001e,0x73755075,0x6e6f4368,0x6e617473,0x00000074,
    0x00050006,0x0000001e,0x00000000,0x61635375,0x0000656c,0x00060006,0x0000001e,0x00000001,
    0x61725475,0x616c736e,0x00006574,0x00030005,0x00000020,0x00006370,0x00040047,0x0000000b,
    0x0000001e,0x00000000,0x00040047,0x0000000f,0x0000001e,0x00000002,0x00040047,0x00000015,
    0x0000001e,0x00000001,0x00050048,0x00000019,0x00000000,0x0000000b,0x00000000,0x00030047,
    0x00000019,0x00000002,0x00040047,0x0000001c,0x0000001e,0x00000000,0x00050048,0x0000001e,
    0x00000000,0x00000023,0x00000000,0x00050048,0x0000001e,0x00000001,0x00000023,0x00000008,
    0x00030047,0x0000001e,0x00000002,0x00020013,0x00000002,0x00030021,0x00000003,0x00000002,
    0x00030016,0x00000006,0x00000020,0x00040017,0x00000007,0x00000006,0x00000004,0x00040017,
    0x00000008,0x00000006,0x00000002,0x0004001e,0x00000009,0x00000007,0x00000008,0x00040020,
    0x0000000a,0x00000003,0x00000009,0x0004003b,0x0000000a,0x0000000b,0x00000003,0x00040015,
    0x0000000c,0x00000020,0x00000001,0x0004002b,0x0000000c,0x0000000d,0x00000000,0x00040020,
    0x0000000e,0x00000001,0x00000007,0x0004003b,0x0000000e,0x0000000f,0x00000001,0x00040020,
    0x00000011,0x00000003,0x00000007,0x0004002b,0x0000000c,0x00000013,0x00000001,0x00040020,
    0x00000014,0x00000001,0x00000008,0x0004003b,0x00000014,0x00000015,0x00000001,0x00040020,
    0x00000017,0x00000003,0x00000008,0x0003001e,0x00000019,0x00000007,0x00040020,0x0000001a,
    0x00000003,0x00000019,0x0004003b,0x0000001a,0x0000001b,0x00000003,0x0004003b,0x00000014,
    0x0000001c,0x00000001,0x0004001e,0x0000001e,0x00000008,0x00000008,0x00040020,0x0000001f,
    0x00000009,0x0000001e,0x0004003b,0x0000001f,0x00000020,0x00000009,0x00040020,0x00000021,
    0x00000009,0x00000008,0x0004002b,0x00000006,0x00000028,0x00000000,0x0004002b,0x00000006,
    0x00000029,0x3f800000,0x00050036,0x00000002,0x00000004,0x00000000,0x00000003,0x000200f8,
    0x00000005,0x0004003d,0x00000007,0x00000010,0x0000000f,0x00050041,0x00000011,0x00000012,
    0x0000000b,0x0000000d,0x0003003e,0x00000012,0x00000010,0x0004003d,0x00000008,0x00000016,
    0x00000015,0x00050041,0x00000017,0x00000018,0x0000000b,0x00000013,0x0003003e,0x00000018,
    0x00000016,0x0004003d,0x00000008,0x0000001d,0x0000001c,0x00050041,0x00000021,0x00000022,
    0x00000020,0x0000000d,0x0004003d,0x00000008,0x00000023,0x00000022,0x00050085,0x00000008,
    0x00000024,0x0000001d,0x00000023,0x00050041,0x00000021,0x00000025,0x00000020,0x00000013,
    0x0004003d,0x00000008,0x00000026,0x00000025,0x00050081,0x00000008,0x00000027,0x00000024,
    0x00000026,0x00050051,0x00000006,0x0000002a,0x00000027,0x00000000,0x00050051,0x00000006,
    0x0000002b,0x00000027,0x00000001,0x00070050,0x00000007,0x0000002c,0x0000002a,0x0000002b,
    0x00000028,0x00000029,0x00050041,0x00000011,0x0000002d,0x0000001b,0x0000000d,0x0003003e,
    0x0000002d,0x0000002c,0x000100fd,0x00010038
];


// Reusable buffers used for rendering 1 current in-flight frame, for ImGui_ImplVulkan_RenderDrawData()
// [Please zero-clear before use!]
#[derive(Default,Debug,Clone)]
struct ImGui_ImplVulkanH_FrameRenderBuffers
{
    // VkDeviceMemory      VertexBufferMemory;
    pub VertexBufferMemory: VkDeviceMemory,
    // VkDeviceMemory      IndexBufferMemory;
    pub IndexBufferMemory: VkDeviceMemory,
    // VkDeviceSize        VertexBufferSize;
    pub VertexBufferSize: VkDeviceSize,
    // VkDeviceSize        IndexBufferSize;
    pub IndexBufferSize: VkDeviceSize,
    // VkBuffer            VertexBuffer;
    pub VertexBuffer: VkBuffer,
    // VkBuffer            IndexBuffer;

}

// Each viewport will hold 1 ImGui_ImplVulkanH_WindowRenderBuffers
// [Please zero-clear before use!]
#[derive(Default,Debug,Clone)]
struct ImGui_ImplVulkanH_WindowRenderBuffers
{
    // uint32_t            Index;
    pub Index: u32,
    // uint32_t            Count;
    pub Count: u32,
    // ImGui_ImplVulkanH_FrameRenderBuffers*   FrameRenderBuffers;
    pub FrameRenderBuffers: *mut ImGui_ImplVulkanH_FrameRenderBuffers
}

// For multi-viewport support:
// Helper structure we store in the void* RenderUserData field of each ImGuiViewport to easily retrieve our backend data.
#[derive(Default,Debug,Clone)]
struct ImGui_ImplVulkan_ViewportData
{
    // bool                                    WindowOwned;
    pub WindowOwned: bool,
    // ImGui_ImplVulkanH_Window                Window;             // Used by secondary viewports only
    pub Window: ImGui_ImplVulkanH_Window,
    // ImGui_ImplVulkanH_WindowRenderBuffers   RenderBuffers;      // Used by all viewports
    pub RenderBuffers: ImGui_ImplVulkanH_WindowRenderBuffers,
    // ImGui_ImplVulkan_ViewportData()         { WindowOwned = false; memset(&RenderBuffers, 0, sizeof(RenderBuffers)); }
    // ~ImGui_ImplVulkan_ViewportData()        { }
}

impl ImGui_ImplVulkan_ViewportData {
    pub fn new() -> Self {
        Self {
            WindowOwned: false,
            ..Default::default()
        }
    }
}

// Vulkan data
#[derive(Default,Debug,Clone)]
pub struct ImGui_ImplVulkan_Data
{
    // ImGui_ImplVulkan_InitInfo   VulkanInitInfo;
    pub VulkanInitInfo: ImGui_ImplVulkan_InitInfo,
    // VkRenderPass                RenderPass;
    pub RenderPass: vk::RenderPass,
    // VkDeviceSize                BufferMemoryAlignment;
    pub BufferMemoryAlignment: vd::DeviceSize,
    // VkPipelineCreateFlags       PipelineCreateFlags;
    pub PipelineCreateFlags: vd::PipelineCreateFlags,
    // VkDescriptorSetLayout       DescriptorSetLayout;
    pub DescriptorSetLayout: vd::DescriptorSetLayout,
    // VkPipelineLayout            PipelineLayout;
    pub PipelineLayout: vd::PipelineLayout,
    // VkPipeline                  Pipeline;
    pub Pipeline: vd::Pipeline,
    // uint32_t                    Subpass;
    pub Subpass: u32,
    // VkShaderModule              ShaderModuleVert;
    pub ShaderModuleVert: vk::ShaderModule,
    // VkShaderModule              ShaderModuleFrag;
    pub ShaderModuleFrag: vk::ShaderModule,
    // Font data
    pub data: Font,
    // VkSampler                   FontSampler;
    pub FontSampler: vk::Sampler,
    // VkDeviceMemory              FontMemory;
    pub FontMemory: vk::DeviceMemory,
    // VkImage                     FontImage;
    pub FontImage: vk::Image,
    // VkImageView                 FontView;
    pub FontView: vk::ImageView,
    // VkDescriptorSet             FontDescriptorSet;
    pub FontDescriptorSet: vk::DescriptorSet,
    // VkDeviceMemory              UploadBufferMemory;
    pub UploadBufferMemory: vk::DeviceMemory,
    // VkBuffer                    UploadBuffer;
    pub UploadBuffer: vk::Buffer,
    // Render buffers for main window
    // ImGui_ImplVulkanH_WindowRenderBuffers MainWindowRenderBuffers;
    pub MainWindowRenderBuffers: ImGui_ImplVulkanH_WindowRenderBuffers,
    // ImGui_ImplVulkan_Data()
    // {
    //     memset(this, 0, sizeof(*this));
    //     BufferMemoryAlignment = 256;
    // }
}


// [Configuration] in order to use a custom Vulkan function loader:
// (1) You'll need to disable default Vulkan function prototypes.
//     We provide a '#define IMGUI_IMPL_VULKAN_NO_PROTOTYPES' convenience configuration flag.
//     In order to make sure this is visible from the imgui_impl_vulkan.cpp compilation unit:
//     - Add '#define IMGUI_IMPL_VULKAN_NO_PROTOTYPES' in your imconfig.h file
//     - Or as a compilation flag in your build system
//     - Or uncomment here (not recommended because you'd be modifying imgui sources!)
//     - Do not simply add it in a .cpp file!
// (2) Call ImGui_ImplVulkan_LoadFunctions() before ImGui_ImplVulkan_Init() with your custom function.
// If you have no idea what this is, leave it alone!
//#define IMGUI_IMPL_VULKAN_NO_PROTOTYPES

// Vulkan includes
// #if defined(IMGUI_IMPL_VULKAN_NO_PROTOTYPES) && !defined(VK_NO_PROTOTYPES)
// #define VK_NO_PROTOTYPES
// #endif
// #include <vulkan/vulkan.h>

// Initialization data, for ImGui_ImplVulkan_Init()
// [Please zero-clear before use!]
#[derive(Default,Debug,Clone)]
pub struct ImGui_ImplVulkan_InitInfo
{
    // VkInstance                      Instance;
    pub Instance: vk::Instance,
    // VkPhysicalDevice                PhysicalDevice;
    pub PhysicalDevice: vk::PhysicalDevice,
    // VkDevice                        Device;
    pub Device: vk::Device,
    // uint32_t                        QueueFamily;
    pub QueueFamily: u32,
    // VkQueue                         Queue;
    pub Queue: vk::Queue,
    // VkPipelineCache                 PipelineCache;
    pub PipelineCache: vk::PipelineCache,
    // VkDescriptorPool                DescriptorPool;
    pub DescriptorPool: vk::DescriptorPool,
    // uint32_t                        Subpass;
    pub Subpass: u32,
    // uint32_t                        MinImageCount;          // >= 2
    pub MinImageCount: u32,
    // uint32_t                        ImageCount;             // >= MinImageCount
    pub ImageCount: u32,
    // VkSampleCountFlagBits           MSAASamples;            // >= VK_SAMPLE_COUNT_1_BIT (0 -> default to VK_SAMPLE_COUNT_1_BIT)
    pub MSAASamples: vk::SampleCountFlagBits,
    // const vk::AllocationCallbacks*    Allocator;
    pub Allocator: *const vk::AllocationCallbacks,
    // void                            (*CheckVkResultFn)(VkResult err);
    pub CheckVkResultFn: fn(err: vk::Result),
}

// Called by user code
// IMGUI_IMPL_API bool         ImGui_ImplVulkan_Init(ImGui_ImplVulkan_InitInfo* info, VkRenderPass render_pass);
// IMGUI_IMPL_API void         ImGui_ImplVulkan_Shutdown();
// IMGUI_IMPL_API void         ImGui_ImplVulkan_NewFrame();
// IMGUI_IMPL_API void         ImGui_ImplVulkan_RenderDrawData(ImDrawData* draw_data, VkCommandBuffer command_buffer, VkPipeline pipeline = VK_NULL_HANDLE);
// IMGUI_IMPL_API bool         ImGui_ImplVulkan_CreateFontsTexture(VkCommandBuffer command_buffer);
// IMGUI_IMPL_API void         ImGui_ImplVulkan_DestroyFontUploadObjects();
// IMGUI_IMPL_API void         ImGui_ImplVulkan_SetMinImageCount(uint32_t min_image_count); // To override MinImageCount after initialization (e.g. if swap chain is recreated)

// Register a texture (VkDescriptorSet == ImTextureID)
// FIXME: This is experimental in the sense that we are unsure how to best design/tackle this problem, please post to https://github.com/ocornut/imgui/pull/914 if you have suggestions.
// IMGUI_IMPL_API VkDescriptorSet ImGui_ImplVulkan_AddTexture(VkSampler sampler, VkImageView image_view, VkImageLayout image_layout);

// Optional: load Vulkan functions with a custom function loader
// This is only useful with IMGUI_IMPL_VULKAN_NO_PROTOTYPES / VK_NO_PROTOTYPES
// IMGUI_IMPL_API bool         ImGui_ImplVulkan_LoadFunctions(PFN_vkVoidFunction(*loader_func)(function_name: *const c_char, void* user_data), void* user_data = NULL);

//-------------------------------------------------------------------------
// Internal / Miscellaneous Vulkan Helpers
// (Used by example's main.cpp. Used by multi-viewport features. PROBABLY NOT used by your own engine/app.)
//-------------------------------------------------------------------------
// You probably do NOT need to use or care about those functions.
// Those functions only exist because:
//   1) they facilitate the readability and maintenance of the multiple main.cpp examples files.
//   2) the multi-viewport / platform window implementation needs them internally.
// Generally we avoid exposing any kind of superfluous high-level helpers in the bindings,
// but it is too much code to duplicate everywhere so we exceptionally expose them.
//
// Your engine/app will likely _already_ have code to setup all that stuff (swap chain, render pass, frame buffers, etc.).
// You may read this code to learn about Vulkan, but it is recommended you use you own custom tailored code to do equivalent work.
// (The ImGui_ImplVulkanH_XXX functions do not interact with any of the state used by the regular ImGui_ImplVulkan_XXX functions)
//-------------------------------------------------------------------------

// struct ImGui_ImplVulkanH_Frame;
// struct ImGui_ImplVulkanH_Window;

// Helpers
// IMGUI_IMPL_API void                 ImGui_ImplVulkanH_CreateOrResizeWindow(VkInstance instance, VkPhysicalDevice physical_device, VkDevice device, ImGui_ImplVulkanH_Window* wnd, uint32_t queue_family, const vk::AllocationCallbacks* allocator, int w, int h, uint32_t min_image_count);
// IMGUI_IMPL_API void                 ImGui_ImplVulkanH_DestroyWindow(VkInstance instance, VkDevice device, ImGui_ImplVulkanH_Window* wnd, const vk::AllocationCallbacks* allocator);
// IMGUI_IMPL_API VkSurfaceFormatKHR   ImGui_ImplVulkanH_SelectSurfaceFormat(VkPhysicalDevice physical_device, VkSurfaceKHR surface, const VkFormat* request_formats, int request_formats_count, VkColorSpaceKHR request_color_space);
// IMGUI_IMPL_API VkPresentModeKHR     ImGui_ImplVulkanH_SelectPresentMode(VkPhysicalDevice physical_device, VkSurfaceKHR surface, const VkPresentModeKHR* request_modes, int request_modes_count);
// IMGUI_IMPL_API int                  ImGui_ImplVulkanH_GetMinImageCountFromPresentMode(VkPresentModeKHR present_mode);

// Helper structure to hold the data needed by one rendering frame
// (Used by example's main.cpp. Used by multi-viewport features. Probably NOT used by your own engine/app.)
// [Please zero-clear before use!]
#[derive(Default,Debug,Clone)]
pub struct ImGui_ImplVulkanH_Frame
{
    // VkCommandPool       CommandPool;
    pub CommandPool: vk::CommandPool,
    // VkCommandBuffer     CommandBuffer;
    pub CommandBuffer: vk::CommandBuffer,
    // VkFence             Fence;
    pub Fence: vk::Fence,
    // VkImage             Backbuffer;
    pub Backbuffer: vk::Image,
    // VkImageView         BackbufferView;
    pub BackbufferView: vk::ImageView,
    // VkFramebuffer       Framebuffer;
    pub Framebuffer: vk::Framebuffer
}

#[derive(Default,Debug,Clone)]
pub struct ImGui_ImplVulkanH_FrameSemaphores
{
    // VkSemaphore         ImageAcquiredSemaphore;
    pub ImageAcquireSemaphore: vk::Semaphore,
    // VkSemaphore         RenderCompleteSemaphore;
    pub RenderCompleteSemaphore: vk::Semaphore,
}

// Helper structure to hold the data needed by one rendering context into one OS window
// (Used by example's main.cpp. Used by multi-viewport features. Probably NOT used by your own engine/app.)
#[derive(Default,Debug,Clone)]
pub struct ImGui_ImplVulkanH_Window
{
    // int                 Width;
    pub Width: i32,
    // int                 Height;
    pub Height: i32,
    // VkSwapchainKHR      Swapchain;
    pub Swapchain: vk::SwapchainKHR,
    // VkSurfaceKHR        Surface;
    pub Surface: vk::SurfaceKHR,
    // VkSurfaceFormatKHR  SurfaceFormat;
    pub SurfaceFormat: vk::SurfaceFormatKHR,
    // VkPresentModeKHR    PresentMode;
    pub PresentMode: vk::PresentModeKHR,
    // VkRenderPass        RenderPass;
    pub RenderPass: vk::RenderPass,
    // VkPipeline          Pipeline;               // The window pipeline may uses a different VkRenderPass than the one passed in ImGui_ImplVulkan_InitInfo
    pub Pipeline: vk::Pipeline,
    // bool                ClearEnable;
    pub ClearEnable: bool,
    // VkClearValue        ClearValue;
    pub ClearValue: vk::ClearValue,
    // uint32_t            FrameIndex;             // Current frame being rendered to (0 <= FrameIndex < FrameInFlightCount)
    pub FrameIndex: u32,
    // uint32_t            ImageCount;             // Number of simultaneous in-flight frames (returned by vkGetSwapchainImagesKHR, usually derived from min_image_count)
    pub ImageCount: u32,
    // uint32_t            SemaphoreIndex;         // Current set of swapchain wait semaphores we're using (needs to be distinct from per frame data)
    pub SemaphoreIndex: u32,
    // ImGui_ImplVulkanH_Frame*            Frames;
    pub Frames: *mut ImGui_ImplVulkanH_Frame,
    // ImGui_ImplVulkanH_FrameSemaphores*  FrameSemaphores;
    pub FrameSemaphores: *mut ImGui_ImplVulkanH_FrameSemaphores,
    // ImGui_ImplVulkanH_Window()
    // {
    //     memset(this, 0, sizeof(*this));
    //     PresentMode = (VkPresentModeKHR)~0;     // Ensure we get an error if user doesn't set this.
    //     ClearEnable = true;
    // }
}


// Backend data stored in io.BackendRendererUserData to allow support for multiple Dear ImGui contexts
// It is STRONGLY preferred that you use docking branch with multi-viewports (== single Dear ImGui context + multiple windows) instead of multiple Dear ImGui contexts.
// FIXME: multi-context support is not tested and probably dysfunctional in this backend.
pub fn ImGui_ImplVulkan_GetBackendData() -> *mut ImGui_ImplVulkan_Data
{
    return if GetCurrentContext() {
        GetIO().BackendRendererUserData as *mut ImGui_ImplVulkan_Data
    } else { null_mut() };
}

pub fn ImGui_ImplVulkan_MemoryType(properties: vk::MemoryPropertyFlags, type_bits: u32) -> u32
{
    let mut bd = ImGui_ImplVulkan_GetBackendData();
    let mut v = &mut bd.VulkanInitInfo;
    let mut prop: vk::PhysicalDeviceMemoryProperties = vk::PhysicalDeviceMemoryProperties{};
    vk::PFN_vkGetPhysicalDeviceMemoryProperties(v.PhysicalDevice, &mut prop);
    // for (uint32_t i = 0; i < prop.memoryTypeCount; i++)
    for i in 0 .. prop.memoryTypeCount
    {
        if (prop.memoryTypes[i].propertyFlags & properties) == properties && type_bits & (1 << i) != 0 {
            return i;
        }
    }
    return 0xFFFFFFFF; // Unable to find memoryType
}

pub fn check_vk_result(VkResult: err)
{
    let mut bd = ImGui_ImplVulkan_GetBackendData();
    if bd.is_null() {
        return;
    }
    let mut v = &mut bd.VulkanInitInfo;
    if v.CheckVkResultFn {
        v.CheckVkResultFn(err)
    };
}

pub fn CreateOrResizeBuffer(buffer: &mut vk::Buffer, buffer_memory: &mut vk::DeviceMemory, p_buffer_size: &mut vk::DeviceSize, new_size: usize, usage: vk::BufferUsageFlagBits)
{
    let mut bd = ImGui_ImplVulkan_GetBackendData();
    let mut v = &mut bd.VulkanInitInfo;
    let mut err: vk::Result;
    if buffer != VK_NULL_HANDLE {
    vkDestroyBuffer(v.Device, buffer, v.Allocator);
}
    if buffer_memory != VK_NULL_HANDLE {
    vkFreeMemory(v.Device, buffer_memory, v.Allocator);
}

    let mut vertex_buffer_size_aligned: vk::DeviceSize = ((new_size - 1) / bd.BufferMemoryAlignment + 1) * bd.BufferMemoryAlignment;
    let mut buffer_info: vK::BufferCreateInfo = vk::BufferCreateInfo{};
    buffer_info.sType = VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO;
    buffer_info.size = vertex_buffer_size_aligned;
    buffer_info.usage = usage;
    buffer_info.sharingMode = VK_SHARING_MODE_EXCLUSIVE;
    err = vkCreateBuffer(v.Device, &buffer_info, v.Allocator, &buffer);
    check_vk_result(err);

    let mut req: vk::MemoryRequirements = vk::MemoryRequirements{};
    vk::PFN_vkGetBufferMemoryRequirements(v.Device, buffer, &mut req);
    bd.BufferMemoryAlignment = if bd.BufferMemoryAlignment > req.alignment { bd.BufferMemoryAlignment } else { req.alignment };
    let mut alloc_info: vk::MemoryAllocateInfo = vk::MemoryAllocateInfo{};
    alloc_info.sType = VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
    alloc_info.allocationSize = req.size;
    alloc_info.memoryTypeIndex = ImGui_ImplVulkan_MemoryType(VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT, req.memoryTypeBits);
    err = vk::PFN_vkAllocateMemory(v.Device, &alloc_info, v.Allocator, &buffer_memory);
    check_vk_result(err);

    err = vk::PFN_vkBindBufferMemory(v.Device, buffer, buffer_memory, 0);
    check_vk_result(err);
    *p_buffer_size = req.size;
}

pub fn ImGui_ImplVulkan_SetupRenderState(draw_data: *mut ImDrawData, pipeline: vk::Pipeline, command_buffer: vk::CommandBuffer, rb: *mut ImGui_ImplVulkanH_FrameRenderBuffers, fb_width: i32, fb_height: i32)
{
    let mut bd = ImGui_ImplVulkan_GetBackendData();

    // Bind pipeline:
    {
        vk::PFN_vkCmdBindPipeline(command_buffer, VK_PIPELINE_BIND_POINT_GRAPHICS, pipeline);
    }

    // Bind Vertex And Index Buffer:
    if draw_data.TotalVtxCount > 0
    {
        let mut vertex_buffers: [vk::Buffer;1] = [rb.VertexBuffer ];
        let mut vertex_offset: [vk::DeviceSize;1] = [ 0 ];
        vk::PFN_vkCmdBindVertexBuffers(command_buffer, 0, 1, vertex_buffers, vertex_offset);
        vk::PFN_vkCmdBindIndexBuffer(command_buffer, rb.IndexBuffer, 0, if std::mem::sizeof::<ImDrawIdx>() == 2 { VK_INDEX_TYPE_UINT16 } else { VK_INDEX_TYPE_UINT32 });
    }

    // Setup viewport:
    {
        let mut viewport: vk::Viewport = vk::Viewport{};
        viewport.x = 0;
        viewport.y = 0;
        viewport.width = fb_width;
        viewport.height = fb_height;
        viewport.minDepth = 0.0f32;
        viewport.maxDepth = 1.0f32;
        vkCmdSetViewport(command_buffer, 0, 1, &viewport);
    }

    // Setup scale and translation:
    // Our visible imgui space lies from draw_data.DisplayPps (top left) to draw_data.DisplayPos+data_data->DisplaySize (bottom right). DisplayPos is (0,0) for single viewport apps.
    {
        let mut scale: [f32;2] = [0f32;2];
        scale[0] = 2.0f32 / draw_data.DisplaySize.x;
        scale[1] = 2.0f32 / draw_data.DisplaySize.y;
        let mut translate: [f32;2] = [0f32;2];
        translate[0] = -1.0f32 - draw_data.DisplayPos.x * scale[0];
        translate[1] = -1.0f32 - draw_data.DisplayPos.y * scale[1];
        vk::PFN_vkCmdPushConstants(command_buffer, bd.PipelineLayout, VK_SHADER_STAGE_VERTEX_BIT, sizeof * 0, sizeof * 2, scale);
        vk::PFN_vkCmdPushConstants(command_buffer, bd.PipelineLayout, VK_SHADER_STAGE_VERTEX_BIT, sizeof * 2, sizeof * 2, translate);
    }
}

// Render function
pub fn ImGui_ImplVulkan_RenderDrawData(draw_data: *mut ImDrawData, command_buffer: vk::CommandBuffer, pipeline: vk::Pipeline)
{
    // Avoid rendering when minimized, scale coordinates for retina displays (screen coordinates != framebuffer coordinates)
    let mut fb_width: i32 = (draw_data.DisplaySize.x * draw_data.FramebufferScale.x) as i32;
    let mut fb_height: i32 = (draw_data.DisplaySize.y * draw_data.FramebufferScale.y) as i32;
    if fb_width <= 0 || fb_height <= 0 {
        return;
    }

    let mut bd = ImGui_ImplVulkan_GetBackendData();
    let mut v = &mut bd.VulkanInitInfo;
    if pipeline == VK_NULL_HANDLE {
    pipeline = bd.Pipeline;
}

    // Allocate array to store enough vertex/index buffers. Each unique viewport gets its own storage.
    let mut viewport_renderer_data = draw_data.OwnerViewport.RendererUserData as *mut ImGui_ImplVulkan_ViewportData;
    // IM_ASSERT(viewport_renderer_data != null_mut());
    let mut wrb = &mut viewport_renderer_data.RenderBuffers;
    if wrb.FrameRenderBuffers == null_mut()
    {
        wrb.Index = 0;
        wrb.Count = v.ImageCount;
        unsafe { wrb.FrameRenderBuffers = libc::malloc(std::mem::sizeof::<ImGui_ImplVulkanH_FrameRenderBuffers>() * wrb.Count) as *mut ImGui_ImplVulkanH_FrameRenderBuffers; }
        unsafe { libc::memset(wrb.FrameRenderBuffers as *mut c_void, 0, sizeof(ImGui_ImplVulkanH_FrameRenderBuffers) * wrb.Count); }
    }
    // IM_ASSERT(wrb.Count == v.ImageCount);
    wrb.Index = (wrb.Index + 1) % wrb.Count;
    let mut rb: &mut ImGui_ImplVulkanH_FrameRenderBuffers = &mut wrb.FrameRenderBuffers[wrb.Index];

    if draw_data.TotalVtxCount > 0
    {
        // Create or resize the vertex/index buffers
        let mut vertex_size = draw_data.TotalVtxCount * std::mem::sizeof::<ImDrawVert>();
        let mut index_size = draw_data.TotalIdxCount * std::mem::sizeof::<ImDrawIdx>();
        if rb.VertexBuffer == VK_NULL_HANDLE || rb.VertexBufferSize < vertex_size {
            CreateOrResizeBuffer(rb.VertexBuffer, rb.VertexBufferMemory, rb.VertexBufferSize, vertex_size, VK_BUFFER_USAGE_VERTEX_BUFFER_BIT);
        }
        if rb.IndexBuffer == VK_NULL_HANDLE || rb.IndexBufferSize < index_size {
            CreateOrResizeBuffer(rb.IndexBuffer, rb.IndexBufferMemory, rb.IndexBufferSize, index_size, VK_BUFFER_USAGE_INDEX_BUFFER_BIT);
        }

        // Upload vertex/index data into a single contiguous GPU buffer
        let mut vtx_dst: *mut ImDrawVert = null_mut();
        let mut idx_dst: *mut ImDrawIdx = null_mut();
        let mut err = vk::PFN_vkMapMemory(v.Device, rb.VertexBufferMemory, 0, rb.VertexBufferSize, 0, (&mut vtx_dst));
        check_vk_result(err);
        err = vk::PFN_vkMapMemory(v.Device, rb.IndexBufferMemory, 0, rb.IndexBufferSize, 0, (&mut idx_dst));
        check_vk_result(err);
        // for (int n = 0; n < draw_data.CmdListsCount; n++)
        for n in 0 .. draw_data.CmdListsCount
        {
            let cmd_list = draw_data.CmdLists[n];
            unsafe { libc::memcpy(vtx_dst, cmd_list.VtxBuffer.Data, cmd_list.VtxBuffer.Size * std::mem::sizeof::<ImDrawVert>()); }
            unsafe { libc::memcpy(idx_dst, cmd_list.IdxBuffer.Data, cmd_list.IdxBuffer.Size * std::mem::sizeof::<ImDrawIdx>()); }
            vtx_dst += cmd_list.VtxBuffer.Size;
            idx_dst += cmd_list.IdxBuffer.Size;
        }
        let mut range: [vk::MappedMemoryRange;2] = [vk::MappedMemoryRange{};2];
        range[0].sType = VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE;
        range[0].memory = rb.VertexBufferMemory;
        range[0].size = VK_WHOLE_SIZE;
        range[1].sType = VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE;
        range[1].memory = rb.IndexBufferMemory;
        range[1].size = VK_WHOLE_SIZE;
        err = vkFlushMappedMemoryRanges(v.Device, 2, range);
        check_vk_result(err);
        vkUnmapMemory(v.Device, rb.VertexBufferMemory);
        vkUnmapMemory(v.Device, rb.IndexBufferMemory);
    }

    // Setup desired Vulkan state
    ImGui_ImplVulkan_SetupRenderState(draw_data, pipeline, command_buffer, rb, fb_width, fb_height);

    // Will project scissor/clipping rectangles into framebuffer space
    let mut clip_off: ImVec2 = draw_data.DisplayPos;         // (0,0) unless using multi-viewports
    let mut clip_scale: ImVec2 = draw_data.FramebufferScale; // (1,1) unless using retina display which are often (2,2)

    // Render command lists
    // (Because we merged all buffers into a single one, we maintain our own offset into them)
    let mut  global_vtx_offset = 0;
    let mut  global_idx_offset = 0;
    // for (int n = 0; n < draw_data.CmdListsCount; n++)
    for n in 0 .. draw_data.CmdListsCount
    {
        let mut cmd_list = draw_data.CmdLists[n];
        // for (int cmd_i = 0; cmd_i < cmd_list.CmdBuffer.Size; cmd_i++)
        for cmd_i in 0 .. cmd_list.CmdBuffer.len()
        {
            let mut pcmd = &mut cmd_list.CmdBuffer[cmd_i];
            if pcmd.UserCallback != null_mut()
            {
                // User callback, registered via ImDrawList::AddCallback()
                // (ImDrawCallback_ResetRenderState is a special callback value used by the user to request the renderer to reset render state.)
                if pcmd.UserCallback == ImDrawCallback_ResetRenderState {
                    ImGui_ImplVulkan_SetupRenderState(draw_data, pipeline, command_buffer, rb, fb_width, fb_height);
                }
                else {
                    pcmd.UserCallback(cmd_list, pcmd);
                }
            }
            else
            {
                // Project scissor/clipping rectangles into framebuffer space
                let mut clip_min = ImVec2::from((pcmd.ClipRect.x - clip_off.x) * clip_scale.x, (pcmd.ClipRect.y - clip_off.y) * clip_scale.y);
                let mut clip_max = ImVec2::from((pcmd.ClipRect.z - clip_off.x) * clip_scale.x, (pcmd.ClipRect.w - clip_off.y) * clip_scale.y);

                // Clamp to viewport as vkCmdSetScissor() won't accept values that are off bounds
                if clip_min.x < 0.0f32 { clip_min.x = 0.0f32; }
                if clip_min.y < 0.0f32 { clip_min.y = 0.0f32; }
                if clip_max.x > fb_width { clip_max.x = fb_width; }
                if clip_max.y > fb_height { clip_max.y = fb_height; }
                if clip_max.x <= clip_min.x || clip_max.y <= clip_min.y {
                    continue;
                }

                // Apply scissor/clipping rectangle
                let mut scissor: vk::Rect2D;
                scissor.offset.x = (clip_min.x);
                scissor.offset.y = (clip_min.y);
                scissor.extent.width = (clip_max.x - clip_min.x);
                scissor.extent.height = (clip_max.y - clip_min.y);
                vkCmdSetScissor(command_buffer, 0, 1, &scissor);

                // Bind DescriptorSet with font or user texture
                let mut desc_set: [vk::DescriptorSet;1] = [ pcmd.TextureId ];
                if std::mem::sizeof::<ImTextureID>() < std::mem::sizeof::<u64>()
                {
                    // We don't support texture switches if ImTextureID hasn't been redefined to be 64-bit. Do a flaky check that other textures haven't been used.
                    // IM_ASSERT(pcmd.TextureId == bd.FontDescriptorSet);
                    desc_set[0] = bd.FontDescriptorSet;
                }
                vkCmdBindDescriptorSets(command_buffer, VK_PIPELINE_BIND_POINT_GRAPHICS, bd.PipelineLayout, 0, 1, desc_set, 0, null_mut());

                // Draw
                vkCmdDrawIndexed(command_buffer, pcmd.ElemCount, 1, pcmd.IdxOffset + global_idx_offset, pcmd.VtxOffset + global_vtx_offset, 0);
            }
        }
        global_idx_offset += cmd_list.IdxBuffer.Size;
        global_vtx_offset += cmd_list.VtxBuffer.Size;
    }

    // Note: at this point both vkCmdSetViewport() and vkCmdSetScissor() have been called.
    // Our last values will leak into user/application rendering IF:
    // - Your app uses a pipeline with VK_DYNAMIC_STATE_VIEWPORT or VK_DYNAMIC_STATE_SCISSOR dynamic state
    // - And you forgot to call vkCmdSetViewport() and vkCmdSetScissor() yourself to explicitely set that state.
    // If you use VK_DYNAMIC_STATE_VIEWPORT or VK_DYNAMIC_STATE_SCISSOR you are responsible for setting the values before rendering.
    // In theory we should aim to backup/restore those values but I am not sure this is possible.
    // We perform a call to vkCmdSetScissor() to set back a full viewport which is likely to fix things for 99% users but technically this is not perfect. (See github #4644)
    let scissor: vk::Rect2D = vk::Rect2D{ offset:vk::Offset2D{ x: 0, y: 0 }, extent:vk::Extent2D{ width: fb_width, height: fb_height } };
    vk::PFN_vkCmdSetScissor(command_buffer, 0, 1, &scissor);
}

pub fn ImGui_ImplVulkan_CreateFontsTexture(command_buffer: vk::CommandBuffer) -> bool
{
    let mut io = GetIO();
    let mut bd = ImGui_ImplVulkan_GetBackendData();
    let mut v = &mut bd.VulkanInitInfo;

    let mut pixels: *mut u8 = null_mut();
    let mut width = 0i32;
    // int width, height;
    let mut height = 0i32;
    io.Fonts.GetTexDataAsRGBA32(&pixels, &width, &height);
    let mut upload_size = width * height * 4 * std::mem::sizeof::<c_char>();

    let mut err: vk::Result;

    // Create the Image:
    {
        let mut info = vk::ImageCreateInfo = vk::ImageCreateInfo{};
        info.sType = VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO;
        info.imageType = VK_IMAGE_TYPE_2D;
        info.format = VK_FORMAT_R8G8B8A8_UNORM;
        info.extent.width = width;
        info.extent.height = height;
        info.extent.depth = 1;
        info.mipLevels = 1;
        info.arrayLayers = 1;
        info.samples = VK_SAMPLE_COUNT_1_BIT;
        info.tiling = VK_IMAGE_TILING_OPTIMAL;
        info.usage = VK_IMAGE_USAGE_SAMPLED_BIT | VK_IMAGE_USAGE_TRANSFER_DST_BIT;
        info.sharingMode = VK_SHARING_MODE_EXCLUSIVE;
        info.initialLayout = VK_IMAGE_LAYOUT_UNDEFINED;
        err = vkCreateImage(v.Device, &info, v.Allocator, &bd.FontImage);
        check_vk_result(err);
        let mut req: vk::MemoryRequirements = vk::MemoryRequirements{};
        vk::PFN_vkGetImageMemoryRequirements(v.Device, bd.FontImage, &req);
        let mut alloc_info: vk::MemoryAllocateInfo = vk::MemoryAllocateInfo{};
        alloc_info.sType = VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
        alloc_info.allocationSize = req.size;
        alloc_info.memoryTypeIndex = ImGui_ImplVulkan_MemoryType(VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT, req.memoryTypeBits);
        err = vkAllocateMemory(v.Device, &alloc_info, v.Allocator, &bd.FontMemory);
        check_vk_result(err);
        err = vkBindImageMemory(v.Device, bd.FontImage, bd.FontMemory, 0);
        check_vk_result(err);
    }

    // Create the Image View:
    {
        let mut info: vk::ImageViewCreateInfo = vk::ImageViewCreateInfo{};
        info.sType = VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO;
        info.image = bd.FontImage;
        info.viewType = VK_IMAGE_VIEW_TYPE_2D;
        info.format = VK_FORMAT_R8G8B8A8_UNORM;
        info.subresourceRange.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT;
        info.subresourceRange.levelCount = 1;
        info.subresourceRange.layerCount = 1;
        err = vkCreateImageView(v.Device, &info, v.Allocator, &bd.FontView);
        check_vk_result(err);
    }

    // Create the Descriptor Set:
    bd.FontDescriptorSet = ImGui_ImplVulkan_AddTexture(bd.FontSampler, bd.FontView, VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL);

    // Create the Upload Buffer:
    {
        let mut buffer_info: vk::BufferCreateInfo = vk::BufferCreateInfo{};
        buffer_info.sType = VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO;
        buffer_info.size = upload_size;
        buffer_info.usage = VK_BUFFER_USAGE_TRANSFER_SRC_BIT;
        buffer_info.sharingMode = VK_SHARING_MODE_EXCLUSIVE;
        err = vkCreateBuffer(v.Device, &buffer_info, v.Allocator, &bd.UploadBuffer);
        check_vk_result(err);
        let mut req: vk::MemoryRequirements = vk::MemoryRequirements{};
        vk::PFN_vkGetBufferMemoryRequirements(v.Device, bd.UploadBuffer, &req);
        bd.BufferMemoryAlignment = if bd.BufferMemoryAlignment > req.alignment { bd.BufferMemoryAlignment } else { req.alignment };
        let mut alloc_info: vk::MemoryAllocateInfo = vk::MemoryAllocateInfo{};
        alloc_info.sType = VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
        alloc_info.allocationSize = req.size;
        alloc_info.memoryTypeIndex = ImGui_ImplVulkan_MemoryType(VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT, req.memoryTypeBits);
        err = vkAllocateMemory(v.Device, &alloc_info, v.Allocator, &bd.UploadBufferMemory);
        check_vk_result(err);
        err = vkBindBufferMemory(v.Device, bd.UploadBuffer, bd.UploadBufferMemory, 0);
        check_vk_result(err);
    }

    // Upload to Buffer:
    {
        let mut map: *mut c_char = null_mut();
        err = vk::PFN_vkMapMemory(v.Device, bd.UploadBufferMemory, 0, upload_size, 0, (&mut map));
        check_vk_result(err);
        unsafe { libc::memcpy(map as *mut c_void, pixels as *const c_void, upload_size); }
        let mut range: [vk::MappedMemoryRange;1] = [vk::MappedMemoryRange{}];
        range[0].sType = VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE;
        range[0].memory = bd.UploadBufferMemory;
        range[0].size = upload_size;
        err = vkFlushMappedMemoryRanges(v.Device, 1, range);
        check_vk_result(err);
        vkUnmapMemory(v.Device, bd.UploadBufferMemory);
    }

    // Copy to Image:
    {
        let mut copy_barrier: [vk::ImageMemoryBarrier;1] = [vk::ImageMemoryBarrier{}];
        copy_barrier[0].sType = VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER;
        copy_barrier[0].dstAccessMask = VK_ACCESS_TRANSFER_WRITE_BIT;
        copy_barrier[0].oldLayout = VK_IMAGE_LAYOUT_UNDEFINED;
        copy_barrier[0].newLayout = VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL;
        copy_barrier[0].srcQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED;
        copy_barrier[0].dstQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED;
        copy_barrier[0].image = bd.FontImage;
        copy_barrier[0].subresourceRange.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT;
        copy_barrier[0].subresourceRange.levelCount = 1;
        copy_barrier[0].subresourceRange.layerCount = 1;
        vkCmdPipelineBarrier(command_buffer, VK_PIPELINE_STAGE_HOST_BIT, VK_PIPELINE_STAGE_TRANSFER_BIT, 0, 0, null_mut(), 0, null_mut(), 1, copy_barrier);

        let mut region: vk::BufferImageCopy = vk::BufferImageCopy{};
        region.imageSubresource.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT;
        region.imageSubresource.layerCount = 1;
        region.imageExtent.width = width;
        region.imageExtent.height = height;
        region.imageExtent.depth = 1;
        vk::PFN_vkCmdCopyBufferToImage(command_buffer, bd.UploadBuffer, bd.FontImage, VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL, 1, &region);

        let mut use_barrier: [vk::ImageMemoryBarrier;1] = [vk::ImageMemoryBarrier{}];
        use_barrier[0].sType = VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER;
        use_barrier[0].srcAccessMask = VK_ACCESS_TRANSFER_WRITE_BIT;
        use_barrier[0].dstAccessMask = VK_ACCESS_SHADER_READ_BIT;
        use_barrier[0].oldLayout = VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL;
        use_barrier[0].newLayout = VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL;
        use_barrier[0].srcQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED;
        use_barrier[0].dstQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED;
        use_barrier[0].image = bd.FontImage;
        use_barrier[0].subresourceRange.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT;
        use_barrier[0].subresourceRange.levelCount = 1;
        use_barrier[0].subresourceRange.layerCount = 1;
        vk::PFN_vkCmdPipelineBarrier(command_buffer, VK_PIPELINE_STAGE_TRANSFER_BIT, VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT, 0, 0, null_mut(), 0, null_mut(), 1, use_barrier);
    }

    // Store our identifier
    io.Fonts.SetTexID(bd.FontDescriptorSet);

    return true;
}

pub fn ImGui_ImplVulkan_CreateShaderModules(device: vk::Device, allocator: *mut vk::AllocationCallbacks)
{
    // Create the shader modules
    let mut bd = ImGui_ImplVulkan_GetBackendData();
    if bd.ShaderModuleVert == VK_NULL_HANDLE
    {
        let mut vert_info: vk::ShaderModuleCreateInfo = vk::ShaderModuleCreateInfo{};
        vert_info.sType = VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO;
        vert_info.codeSize = std::mem::sizeof::<__glsl_shader_vert_spv>();
        vert_info.pCode = __glsl_shader_vert_spv;
        let mut err = vk::PFN_vkCreateShaderModule(device, &vert_info, allocator, &bd.ShaderModuleVert);
        check_vk_result(err);
    }
    if bd.ShaderModuleFrag == VK_NULL_HANDLE
    {
        let mut frag_info: vk::ShaderModuleCreateInfo = vk::ShaderModuleCreateInfo{};
        frag_info.sType = VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO;
        frag_info.codeSize = sizeof(__glsl_shader_frag_spv);
        frag_info.pCode = __glsl_shader_frag_spv;
        let mut err = vk::CreateShaderModule(device, &frag_info, allocator, &bd.ShaderModuleFrag);
        check_vk_result(err);
    }
}

pub fn ImGui_ImplVulkan_CreateFontSampler(device: vk::Device, allocator: *mut vk::AllocationCallbacks)
{
    let mut bd = ImGui_ImplVulkan_GetBackendData();
    if bd.FontSampler {
        return;
    }

    // Bilinear sampling is required by default. Set 'io.Fonts->Flags |= ImFontAtlasFlags_NoBakedLines' or 'style.AntiAliasedLinesUseTex = false' to allow point/nearest sampling.
    let mut info: vk::SamplerCreateInfo = vk::SamplerCreateInfo{};
    info.sType = VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO;
    info.magFilter = VK_FILTER_LINEAR;
    info.minFilter = VK_FILTER_LINEAR;
    info.mipmapMode = VK_SAMPLER_MIPMAP_MODE_LINEAR;
    info.addressModeU = VK_SAMPLER_ADDRESS_MODE_REPEAT;
    info.addressModeV = VK_SAMPLER_ADDRESS_MODE_REPEAT;
    info.addressModeW = VK_SAMPLER_ADDRESS_MODE_REPEAT;
    info.minLod = -1000;
    info.maxLod = 1000;
    info.maxAnisotropy = 1.0f32;
    let mut err = vk::PFN_vkCreateSampler(device, &info, allocator, &bd.FontSampler);
    check_vk_result(err);
}

pub fn ImGui_ImplVulkan_CreateDescriptorSetLayout(device: vk::Device, allocator: *mut vk::AllocationCallbacks)
{
    let mut bd = ImGui_ImplVulkan_GetBackendData();
    if bd.DescriptorSetLayout {
        return;
    }

    ImGui_ImplVulkan_CreateFontSampler(device, allocator);
    let mut sampler: [vk::Sampler ;1] = [ bd.FontSampler ];
    let mut binding: [vk::DescriptorSetLayoutBinding; 1] = [vk::DescriptorSetLayoutBinding{}];
    binding[0].descriptorType = VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER;
    binding[0].descriptorCount = 1;
    binding[0].stageFlags = VK_SHADER_STAGE_FRAGMENT_BIT;
    binding[0].pImmutableSamplers = sampler;
    let mut info: vk::DescriptorSetLayoutCreateInfo = vk::DescriptorSetLayoutCreateInfo{};
    info.sType = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO;
    info.bindingCount = 1;
    info.pBindings = binding;
    let mut err: vk::Result = vk::PFN_vkCreateDescriptorSetLayout(device, &info, allocator, &bd.DescriptorSetLayout);
    check_vk_result(err);
}

pub fn ImGui_ImplVulkan_CreatePipelineLayout(device: vk::Device, allocator: *mut vk::AllocationCallbacks)
{
    let mut bd = ImGui_ImplVulkan_GetBackendData();
    if bd.PipelineLayout {
        return;
    }

    // Constants: we are using 'vec2 offset' and 'vec2 scale' instead of a full 3d projection matrix
    ImGui_ImplVulkan_CreateDescriptorSetLayout(device, allocator);
    let mut push_constants: [vk::PushConstantRange;1] = [vk::PushConstantRange{}];
    push_constants[0].stageFlags = VK_SHADER_STAGE_VERTEX_BIT;
    push_constants[0].offset = sizeof * 0;
    push_constants[0].size = sizeof * 4;
    let mut set_layout: [vk::DescriptorSetLayout;1] = [ bd.DescriptorSetLayout ];
    let mut layout_info: vk::PipelineLayoutCreateInfo = vk::PipelineLayoutCreateInfo{};
    layout_info.sType = VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO;
    layout_info.setLayoutCount = 1;
    layout_info.pSetLayouts = set_layout;
    layout_info.pushConstantRangeCount = 1;
    layout_info.pPushConstantRanges = push_constants;
    let mut err = vk::PFN_vkCreatePipelineLayout(device, &layout_info, allocator, &bd.PipelineLayout);
    check_vk_result(err);
}

pub fn ImGui_ImplVulkan_CreatePipeline(device: vk::Device, allocator: *mut vk::AllocationCallbacks, pipelineCache: vk::PipelineCache, renderPass: vk::RenderPass, MSAASamples: vk::SampleCountFlagBits, pipelines: *mut vk::Pipeline, subpass: u32)
{
    let mut bd: *mut ImGui_ImplVulkan_Data =  ImGui_ImplVulkan_GetBackendData();
    ImGui_ImplVulkan_CreateShaderModules(device, allocator);

    let mut stage: [vk::PipelineShaderStageCreateInfo;2] = [vk::PipelineShaderStageCreateInfo{};2];
    stage[0].sType = VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO;
    stage[0].stage = VK_SHADER_STAGE_VERTEX_BIT;
    stage[0].module = bd.ShaderModuleVert;
    stage[0].pName = "main";
    stage[1].sType = VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO;
    stage[1].stage = VK_SHADER_STAGE_FRAGMENT_BIT;
    stage[1].module = bd.ShaderModuleFrag;
    stage[1].pName = "main";

    let mut binding_desc: [vk::VertexInputBindingDescription;1] = [vk::VertexInputBindingDescription];
    binding_desc[0].stride = std::mem::sizeof::<ImDrawVert>();
    binding_desc[0].inputRate = VK_VERTEX_INPUT_RATE_VERTEX;

    let mut attribute_desc: [vk::VertexInputAttributeDescription;3] = [vk::VertexInputAttributeDescription{};3];
    attribute_desc[0].location = 0;
    attribute_desc[0].binding = binding_desc[0].binding;
    attribute_desc[0].format = VK_FORMAT_R32G32_SFLOAT;
    attribute_desc[0].offset = IM_OFFSETOF(ImDrawVert, pos);
    attribute_desc[1].location = 1;
    attribute_desc[1].binding = binding_desc[0].binding;
    attribute_desc[1].format = VK_FORMAT_R32G32_SFLOAT;
    attribute_desc[1].offset = IM_OFFSETOF(ImDrawVert, uv);
    attribute_desc[2].location = 2;
    attribute_desc[2].binding = binding_desc[0].binding;
    attribute_desc[2].format = VK_FORMAT_R8G8B8A8_UNORM;
    attribute_desc[2].offset = IM_OFFSETOF(ImDrawVert, col);

    let mut vertex_info: vk::PipelineVertexInputStateCreateInfo = vk::PipelineVertexInputStateCreateInfo{};
    vertex_info.sType = VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO;
    vertex_info.vertexBindingDescriptionCount = 1;
    vertex_info.pVertexBindingDescriptions = binding_desc;
    vertex_info.vertexAttributeDescriptionCount = 3;
    vertex_info.pVertexAttributeDescriptions = attribute_desc;

    let mut ia_info: vk::PipelineInputAssemblyStateCreateInfo = vk::PipelineInputAssemblyStateCreateInfo{};
    ia_info.sType = VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO;
    ia_info.topology = VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST;

    let mut viewport_info: vk::PipelineViewportStateCreateInfo = vk::PipelineViewportStateCreateInfo{};
    viewport_info.sType = VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO;
    viewport_info.viewportCount = 1;
    viewport_info.scissorCount = 1;

    let mut raster_info: vk::PipelineRasterizationStateCreateInfo = vk::PipelineRasterizationStateCreateInfo{};
    raster_info.sType = VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO;
    raster_info.polygonMode = VK_POLYGON_MODE_FILL;
    raster_info.cullMode = VK_CULL_MODE_NONE;
    raster_info.frontFace = VK_FRONT_FACE_COUNTER_CLOCKWISE;
    raster_info.lineWidth = 1.0f32;

    let mut ms_info: vk::PipelineMultisampleStateCreateInfo = vk::PipelineMultisampleStateCreateInfo{};
    ms_info.sType = VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO;
    ms_info.rasterizationSamples = if MSAASamples != 0 { MSAASamples } else { VK_SAMPLE_COUNT_1_BIT };

    let mut color_attachment: [vk::PipelineColorBlendAttachmentState;1] = [vk::PipelineColorBlendAttachmentState{}];
    color_attachment[0].blendEnable = VK_TRUE;
    color_attachment[0].srcColorBlendFactor = VK_BLEND_FACTOR_SRC_ALPHA;
    color_attachment[0].dstColorBlendFactor = VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA;
    color_attachment[0].colorBlendOp = VK_BLEND_OP_ADD;
    color_attachment[0].srcAlphaBlendFactor = VK_BLEND_FACTOR_ONE;
    color_attachment[0].dstAlphaBlendFactor = VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA;
    color_attachment[0].alphaBlendOp = VK_BLEND_OP_ADD;
    color_attachment[0].colorWriteMask = VK_COLOR_COMPONENT_R_BIT | VK_COLOR_COMPONENT_G_BIT | VK_COLOR_COMPONENT_B_BIT | VK_COLOR_COMPONENT_A_BIT;

    let mut depth_info: vk::PipelineDepthStencilStateCreateInfo = vk::PipelineDepthStencilStateCreateInfo{};
    depth_info.sType = VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO;

    let mut blend_info: vk::PipelineColorBlendStateCreateInfo = vk::PipelineColorBlendStateCreateInfo{};
    blend_info.sType = VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO;
    blend_info.attachmentCount = 1;
    blend_info.pAttachments = color_attachment;

    let mut dynamic_states: [vk::DynamicState;2] = [ VK_DYNAMIC_STATE_VIEWPORT, VK_DYNAMIC_STATE_SCISSOR ];
    let mut dynamic_state: vk::PipelineDynamicStateCreateInfo = vk::PipelineDynamicStateCreateInfo{};
    dynamic_state.sType = VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO;
    dynamic_state.dynamicStateCount = IM_ARRAYSIZE(dynamic_states);
    dynamic_state.pDynamicStates = dynamic_states;

    ImGui_ImplVulkan_CreatePipelineLayout(device, allocator);

    let mut info: vk::GraphicsPipelineCreateInfo = vk::GraphicsPipelineCreateInfo{};
    info.sType = VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO;
    info.flags = bd.PipelineCreateFlags;
    info.stageCount = 2;
    info.pStages = stage;
    info.pVertexInputState = &vertex_info;
    info.pInputAssemblyState = &ia_info;
    info.pViewportState = &viewport_info;
    info.pRasterizationState = &raster_info;
    info.pMultisampleState = &ms_info;
    info.pDepthStencilState = &depth_info;
    info.pColorBlendState = &blend_info;
    info.pDynamicState = &dynamic_state;
    info.layout = bd.PipelineLayout;
    info.renderPass = renderPass;
    info.subpass = subpass;
    let mut err = vk::PFN_vkCreateGraphicsPipelines(device, pipelineCache, 1, &info, allocator, pipeline);
    check_vk_result(err);
}

pub fn ImGui_ImplVulkan_CreateDeviceObjects() -> bool
{
    let mut bd: *mut ImGui_ImplVulkan_Data =  ImGui_ImplVulkan_GetBackendData();
    ImGui_ImplVulkan_InitInfo* v = &bd.VulkanInitInfo;
    let mut err: vk::Result;

    if !bd.FontSampler
    {
        let mut info: vk::SamplerCreateInfo = vk::SamplerCreateInfo{};
        info.sType = VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO;
        info.magFilter = VK_FILTER_LINEAR;
        info.minFilter = VK_FILTER_LINEAR;
        info.mipmapMode = VK_SAMPLER_MIPMAP_MODE_LINEAR;
        info.addressModeU = VK_SAMPLER_ADDRESS_MODE_REPEAT;
        info.addressModeV = VK_SAMPLER_ADDRESS_MODE_REPEAT;
        info.addressModeW = VK_SAMPLER_ADDRESS_MODE_REPEAT;
        info.minLod = -1000;
        info.maxLod = 1000;
        info.maxAnisotropy = 1.0f32;
        err = vkCreateSampler(v.Device, &info, v.Allocator, &bd.FontSampler);
        check_vk_result(err);
    }

    if !bd.DescriptorSetLayout
    {
        let mut sampler: [vk::Sampler;1] = [bd.FontSampler];
        let mut binding: [vk::DescriptorSetLayoutBinding;1] = [vk::DescriptorSetLayoutBinding{}];
        binding[0].descriptorType = VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER;
        binding[0].descriptorCount = 1;
        binding[0].stageFlags = VK_SHADER_STAGE_FRAGMENT_BIT;
        binding[0].pImmutableSamplers = sampler;
        let mut info: vk::DescriptorSetLayoutCreateInfo = vk::DescriptorSetLayoutCreateInfo{};
        info.sType = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO;
        info.bindingCount = 1;
        info.pBindings = binding;
        err = vkCreateDescriptorSetLayout(v.Device, &info, v.Allocator, &bd.DescriptorSetLayout);
        check_vk_result(err);
    }

    if !bd.PipelineLayout
    {
        // Constants: we are using 'vec2 offset' and 'vec2 scale' instead of a full 3d projection matrix
        let mut push_constants: [vk::PushConstantRange;1] = vk::PushConstantRange{};
        push_constants[0].stageFlags = VK_SHADER_STAGE_VERTEX_BIT;
        push_constants[0].offset = sizeof * 0;
        push_constants[0].size = sizeof * 4;
        let mut set_layout: [vk::DescriptorSetLayout;1] = [ bd.DescriptorSetLayout ];
        let mut layout_info: vk::PipelineLayoutCreateInfo = vk::PipelineLayoutCreateInfo{};
        layout_info.sType = VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO;
        layout_info.setLayoutCount = 1;
        layout_info.pSetLayouts = set_layout;
        layout_info.pushConstantRangeCount = 1;
        layout_info.pPushConstantRanges = push_constants;
        err = vk::PFN_vkCreatePipelineLayout(v.Device, &layout_info, v.Allocator, &bd.PipelineLayout);
        check_vk_result(err);
    }

    ImGui_ImplVulkan_CreatePipeline(v.Device, v.Allocator, v.PipelineCache, bd.RenderPass, v.MSAASamples, &mut bd.Pipeline, bd.Subpass);

    return true;
}

pub fn    ImGui_ImplVulkan_DestroyFontUploadObjects()
{
    let mut bd = ImGui_ImplVulkan_GetBackendData();
    let mut v = &bd.VulkanInitInfo;
    if bd.UploadBuffer
    {
        vkDestroyBuffer(v.Device, bd.UploadBuffer, v.Allocator);
        bd.UploadBuffer = VK_NULL_HANDLE;
    }
    if bd.UploadBufferMemory
    {
        vkFreeMemory(v.Device, bd.UploadBufferMemory, v.Allocator);
        bd.UploadBufferMemory = VK_NULL_HANDLE;
    }
}

pub fn    ImGui_ImplVulkan_DestroyDeviceObjects()
{
    let mut bd = ImGui_ImplVulkan_GetBackendData();
    let mut v = &bd.VulkanInitInfo;
    ImGui_ImplVulkanH_DestroyAllViewportsRenderBuffers(v.Device, v.Allocator);
    ImGui_ImplVulkan_DestroyFontUploadObjects();

    if (bd.ShaderModuleVert)     { vk::PFN_vkDestroyShaderModule(v.Device, bd.ShaderModuleVert, v.Allocator); bd.ShaderModuleVert = VK_NULL_HANDLE; }
    if (bd.ShaderModuleFrag)     { vk::PFN_vkDestroyShaderModule(v.Device, bd.ShaderModuleFrag, v.Allocator); bd.ShaderModuleFrag = VK_NULL_HANDLE; }
    if (bd.FontView)             { vk::PFN_vkDestroyImageView(v.Device, bd.FontView, v.Allocator); bd.FontView = VK_NULL_HANDLE; }
    if (bd.FontImage)            { vk::PFN_vkDestroyImage(v.Device, bd.FontImage, v.Allocator); bd.FontImage = VK_NULL_HANDLE; }
    if (bd.FontMemory)           { vk::PFN_vkFreeMemory(v.Device, bd.FontMemory, v.Allocator); bd.FontMemory = VK_NULL_HANDLE; }
    if (bd.FontSampler)          { vk::PFN_vkDestroySampler(v.Device, bd.FontSampler, v.Allocator); bd.FontSampler = VK_NULL_HANDLE; }
    if (bd.DescriptorSetLayout)  { vk::PFN_vkDestroyDescriptorSetLayout(v.Device, bd.DescriptorSetLayout, v.Allocator); bd.DescriptorSetLayout = VK_NULL_HANDLE; }
    if (bd.PipelineLayout)       { vk::PFN_vkDestroyPipelineLayout(v.Device, bd.PipelineLayout, v.Allocator); bd.PipelineLayout = VK_NULL_HANDLE; }
    if (bd.Pipeline)             { vk::PFN_vkDestroyPipeline(v.Device, bd.Pipeline, v.Allocator); bd.Pipeline = VK_NULL_HANDLE; }
}

pub fn    ImGui_ImplVulkan_LoadFunctions(loader_func: fn(function_name: *const c_char, user_data: *mut c_void), user_data: *mut c_void)
{
    // Load function pointers
    // You can use the default Vulkan loader using:
    //      ImGui_ImplVulkan_LoadFunctions([](const char* function_name, void*) { return vkGetInstanceProcAddr(your_vk_isntance, function_name); });
    // But this would be equivalent to not setting VK_NO_PROTOTYPES.
// #ifdef VK_NO_PROTOTYPES
// #define IMGUI_VULKAN_FUNC_LOAD(func) \
//     func = reinterpret_cast<decltype(func)>(loader_func(#func, user_data)); \
//     if (func == null_mut())   \
//         return false;
//     IMGUI_VULKAN_FUNC_MAP(IMGUI_VULKAN_FUNC_LOAD)
// #undef IMGUI_VULKAN_FUNC_LOAD
// #else
//     IM_UNUSED(loader_func);
//     IM_UNUSED(user_data);
// #endif
//     g_FunctionsLoaded = true;
//     return true;
    // TODO:
}

pub fn    ImGui_ImplVulkan_Init(info: *mut ImGui_ImplVulkan_InitInfo, render_pass: vk::RenderPass) -> bool
{
    // IM_ASSERT(g_FunctionsLoaded && "Need to call ImGui_ImplVulkan_LoadFunctions() if IMGUI_IMPL_VULKAN_NO_PROTOTYPES or VK_NO_PROTOTYPES are set!");

    ImGuiIO& io = Imgui::GetIO();
    // IM_ASSERT(io.BackendRendererUserData == null_mut() && "Already initialized a renderer backend!");

    // Setup backend capabilities flags
    let mut bd: *mut ImGui_ImplVulkan_Data =  IM_NEW(ImGui_ImplVulkan_Data)();
    io.BackendRendererUserData = bd;
    io.BackendRendererName = "imgui_impl_vulkan";
    io.BackendFlags |= IM_GUI_BACKEND_FLAGS_RENDERER_HAS_VTX_OFFSET;  // We can honor the ImDrawCmd::VtxOffset field, allowing for large meshes.
    io.BackendFlags |= IM_GUI_BACKEND_FLAGS_RENDERER_HAS_VIEWPORTS;  // We can create multi-viewports on the Renderer side (optional)

    IM_ASSERT(info.Instance != VK_NULL_HANDLE);
    IM_ASSERT(info.PhysicalDevice != VK_NULL_HANDLE);
    IM_ASSERT(info.Device != VK_NULL_HANDLE);
    IM_ASSERT(info.Queue != VK_NULL_HANDLE);
    IM_ASSERT(info.DescriptorPool != VK_NULL_HANDLE);
    IM_ASSERT(info.MinImageCount >= 2);
    IM_ASSERT(info.ImageCount >= info.MinImageCount);
    IM_ASSERT(render_pass != VK_NULL_HANDLE);

    unsafe { bd.VulkanInitInfo = (*info).clone(); }
    bd.RenderPass = render_pass;
    bd.Subpass = info.Subpass;

    ImGui_ImplVulkan_CreateDeviceObjects();

    // Our render function expect RendererUserData to be storing the window render buffer we need (for the main viewport we won't use ->Window)
    let mut main_viewport = unsafe { GetMainViewport() };
    unsafe { main_viewport.RendererUserData = libc::malloc(std::mem::size_of::<ImGui_ImplVulkan_ViewportData>()); } // IM_NEW(ImGui_ImplVulkan_ViewportData)();

    if io.ConfigFlags & ImGuiConfigFlags_ViewportsEnable {
        ImGui_ImplVulkan_InitPlatformInterface();
    }

    return true;
}

pub fn ImGui_ImplVulkan_Shutdown()
{
    let mut bd = ImGui_ImplVulkan_GetBackendData();
    // IM_ASSERT(bd != null_mut() && "No renderer backend to shutdown, or already shutdown?");
    let mut io = Imgui::GetIO();

    // First destroy objects in all viewports
    ImGui_ImplVulkan_DestroyDeviceObjects();

    // Manually delete main viewport render data in-case we haven't initialized for viewports
    let mut main_viewport = unsafe { GetMainViewport() };
    let mut vd = main_viewport.RendererUserData;
    if vd
    {
        unsafe { libc::free(vd as *mut c_void); }
    }
    main_viewport.RendererUserData = null_mut();

    // Clean up windows
    ImGui_ImplVulkan_ShutdownPlatformInterface();

    io.BackendRendererName = null_mut();
    io.BackendRendererUserData = null_mut();
    IM_DELETE(bd);
}

pub fn ImGui_ImplVulkan_NewFrame()
{
    let mut bd = ImGui_ImplVulkan_GetBackendData();
    // IM_ASSERT(bd != null_mut() && "Did you call ImGui_ImplVulkan_Init()?");
    // IM_UNUSED(bd);
}

pub fn ImGui_ImplVulkan_SetMinImageCount(min_image_count: u32)
{
   let mut bd = ImGui_ImplVulkan_GetBackendData();
    // IM_ASSERT(min_image_count >= 2);
    if bd.VulkanInitInfo.MinImageCount == min_image_count {
        return;
    }

    // IM_ASSERT(0); // FIXME-VIEWPORT: Unsupported. Need to recreate all swap chains!
    let mut v = &mut bd.VulkanInitInfo;
    let mut err = vkDeviceWaitIdle(v.Device);
    check_vk_result(err);
    ImGui_ImplVulkanH_DestroyAllViewportsRenderBuffers(v.Device, v.Allocator);

    bd.VulkanInitInfo.MinImageCount = min_image_count;
}

// Register a texture
// FIXME: This is experimental in the sense that we are unsure how to best design/tackle this problem, please post to https://github.com/ocornut/imgui/pull/914 if you have suggestions.
pub fn ImGui_ImplVulkan_AddTexture(sampler: vk::Sample, image_view: vk::ImageView, image_layout: vk::ImageLayout) -> vk::DescriptorSet
{
    let mut bd = ImGui_ImplVulkan_GetBackendData();
    let mut v = &mut bd.VulkanInitInfo;

    // Create Descriptor Set:
    let mut descriptor_set: vk::DescriptorSet = vk::DescriptorSet{};
    {
        let mut alloc_info: vk::DescriptorSetAllocateInfo = vk::DescriptorSetAllocateInfo{};
        alloc_info.sType = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO;
        alloc_info.descriptorPool = v.DescriptorPool;
        alloc_info.descriptorSetCount = 1;
        alloc_info.pSetLayouts = &bd.DescriptorSetLayout;
        let mut err = vk::PFN_vkAllocateDescriptorSets(v.Device, &alloc_info, &descriptor_set);
        check_vk_result(err);
    }

    // Update the Descriptor Set:
    {
        let mut desc_image: [vk::DescriptorImageInfo;1] = [vk::DescriptorImageInfo{}];
        desc_image[0].sampler = sampler;
        desc_image[0].imageView = image_view;
        desc_image[0].imageLayout = image_layout;
        let mut write_desc: [vk::WriteDescriptorSet;1] = vk::WriteDescriptorSet{};
        write_desc[0].sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET;
        write_desc[0].dstSet = descriptor_set;
        write_desc[0].descriptorCount = 1;
        write_desc[0].descriptorType = VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER;
        write_desc[0].pImageInfo = desc_image;
        vkUpdateDescriptorSets(v.Device, 1, write_desc, 0, null_mut());
    }
    return descriptor_set;
}

//-------------------------------------------------------------------------
// Internal / Miscellaneous Vulkan Helpers
// (Used by example's main.cpp. Used by multi-viewport features. PROBABLY NOT used by your own app.)
//-------------------------------------------------------------------------
// You probably do NOT need to use or care about those functions.
// Those functions only exist because:
//   1) they facilitate the readability and maintenance of the multiple main.cpp examples files.
//   2) the upcoming multi-viewport feature will need them internally.
// Generally we avoid exposing any kind of superfluous high-level helpers in the backends,
// but it is too much code to duplicate everywhere so we exceptionally expose them.
//
// Your engine/app will likely _already_ have code to setup all that stuff (swap chain, render pass, frame buffers, etc.).
// You may read this code to learn about Vulkan, but it is recommended you use you own custom tailored code to do equivalent work.
// (The ImGui_ImplVulkanH_XXX functions do not interact with any of the state used by the regular ImGui_ImplVulkan_XXX functions)
//-------------------------------------------------------------------------

pub fn ImGui_ImplVulkanH_SelectSurfaceFormat(physical_device: vk::PhysicalDevice, surface: vk::SurfaceKHR, request_formats: *const vk::Format, request_formats_count: i32, request_color_space: vk::ColorSpaceKHR) -> vk::SurfaceFormatKHR
{
    // IM_ASSERT(g_FunctionsLoaded && "Need to call ImGui_ImplVulkan_LoadFunctions() if IMGUI_IMPL_VULKAN_NO_PROTOTYPES or VK_NO_PROTOTYPES are set!");
    // IM_ASSERT(request_formats != null_mut());
    // IM_ASSERT(request_formats_count > 0);

    // Per Spec Format and View Format are expected to be the same unless VK_IMAGE_CREATE_MUTABLE_BIT was set at image creation
    // Assuming that the default behavior is without setting this bit, there is no need for separate Swapchain image and image view format
    // Additionally several new color spaces were introduced with Vulkan Spec v1.0.40,
    // hence we must make sure that a format with the mostly available color space, VK_COLOR_SPACE_SRGB_NONLINEAR_KHR, is found and used.
    let mut avail_count = 0u32;
    vk::PFN_vkGetPhysicalDeviceSurfaceFormatsKHR(physical_device, surface, &avail_count, null_mut());
    let mut avail_format: Vec<VkSurfaceFormatKHR> = vec![];
    // avail_format.resize(avail_count);
    vk::PFN_vkGetPhysicalDeviceSurfaceFormatsKHR(physical_device, surface, &avail_count, avail_format.Data);

    // First check if only one format, VK_FORMAT_UNDEFINED, is available, which would imply that any format is available
    if avail_count == 1
    {
        if avail_format[0].format == VK_FORMAT_UNDEFINED
        {
            let mut ret: vk::SurfaceFormatKHR = vk::SurfaceFormatKHR{};
            ret.format = request_formats[0];
            ret.colorSpace = request_color_space;
            return ret;
        }
        else
        {
            // No point in searching another format
            return avail_format[0];
        }
    }
    else
    {
        // Request several formats, the first found will be used
        // for (int request_i = 0; request_i < request_formats_count; request_i++)
        for request_i in 0 .. request_formats_count
        {
            // for (uint32_t avail_i = 0; avail_i < avail_count; avail_i+ +)
            for avail_i  in 0 .. avail_count
            {
                if avail_format[avail_i].format == request_formats[request_i] && avail_format[avail_i].colorSpace == request_color_space {
                    return avail_format[avail_i];
                }
            }
        }

        // If none of the requested image formats could be found, use the first available
        return avail_format[0];
    }
}

pub fn ImGui_ImplVulkanH_SelectPresentMode(physical_device: vk::PhysicalDevice, surface: vk::SurfaceKHR, request_modes: *const vk::PresentModeKHR, request_modes_count: i32) -> vk::PresentModeKHR
{
    // IM_ASSERT(g_FunctionsLoaded && "Need to call ImGui_ImplVulkan_LoadFunctions() if IMGUI_IMPL_VULKAN_NO_PROTOTYPES or VK_NO_PROTOTYPES are set!");
    // IM_ASSERT(request_modes != null_mut());
    // IM_ASSERT(request_modes_count > 0);

    // Request a certain mode and confirm that it is available. If not use VK_PRESENT_MODE_FIFO_KHR which is mandatory
    let mut avail_count = 0u32;
    vk::PFN_vkGetPhysicalDeviceSurfacePresentModesKHR(physical_device, surface, &avail_count, null_mut());
    let mut avail_modes: Vec<VkPresentModeKHR> = vec![];
    // avail_modes.resize(avail_count);
    vk::PFN_vkGetPhysicalDeviceSurfacePresentModesKHR(physical_device, surface, &avail_count, avail_modes.Data);
    //for (uint32_t avail_i = 0; avail_i < avail_count; avail_i++)
    //    printf("[vulkan] avail_modes[%d] = %d\n", avail_i, avail_modes[avail_i]);

    // for (int request_i = 0; request_i < request_modes_count; request_i++)
    for request_i in 0 .. request_modes_count
    {
        // for (uint32_t avail_i = 0; avail_i < avail_count; avail_i+ +)
        for avail_i in 0 .. avail_count
        {
            if request_modes[request_i] == avail_modes[avail_i] {
                return request_modes[request_i];
            }
        }
    }

    return VK_PRESENT_MODE_FIFO_KHR; // Always available
}

pub fn ImGui_ImplVulkanH_CreateWindowCommandBuffers(physical_device: vk::PhysicalDevice , device: vk::Device, wd: *mut ImGui_ImplVulkanH_Window, queue_family: u32, allocator: * const vk::AllocationCallbacks)
{
    // IM_ASSERT(physical_device != VK_NULL_HANDLE && device != VK_NULL_HANDLE);
    // (void)physical_device;
    // (void)allocator;

    // Create Command Buffers
    let mut err: vk::Result;
    // for (uint32_t i = 0; i < wd.ImageCount; i++)
    for i in 0 .. wd.ImageCount
    {
        let mut fd: * mut ImGui_ImplVulkanH_Frame = &mut wd.Frames[i];
        let mut fsd: * mut ImGui_ImplVulkanH_FrameSemaphores  = &mut wd.FrameSemaphores[i];
        {
            let mut info: vk::CommandPoolCreateInfo = vk::CommandPoolCreateInfo{};
            info.sType = VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO;
            info.flags = VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT;
            info.queueFamilyIndex = queue_family;
            err = vkCreateCommandPool(device, &info, allocator, &fd.CommandPool);
            check_vk_result(err);
        }
        {
            let mut info: vk::CommandBufferAllocateInfo = vk::CommandBufferAllocateInfo{};
            info.sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO;
            info.commandPool = fd.CommandPool;
            info.level = VK_COMMAND_BUFFER_LEVEL_PRIMARY;
            info.commandBufferCount = 1;
            err = vkAllocateCommandBuffers(device, &info, &fd.CommandBuffer);
            check_vk_result(err);
        }
        {
            let mut info: vk::FenceCreateInfo = vk::FenceCreateInfo{};
            info.sType = VK_STRUCTURE_TYPE_FENCE_CREATE_INFO;
            info.flags = VK_FENCE_CREATE_SIGNALED_BIT;
            err = vkCreateFence(device, &info, allocator, &mut fd.Fence);
            check_vk_result(err);
        }
        {
            let mut info: vk::SemaphoreCreateInfo = vk::SemaphoreCreateInfo{};
            info.sType = VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO;
            err = vkCreateSemaphore(device, &info, allocator, &mut fsd.ImageAcquiredSemaphore);
            check_vk_result(err);
            err = vkCreateSemaphore(device, &info, allocator, &fsd.RenderCompleteSemaphore);
            check_vk_result(err);
        }
    }
}

pub fn ImGui_ImplVulkanH_GetMinImageCountFromPresentMode(present_mode: vk::PresentModeKHR) -> i32
{
    if present_mode == VK_PRESENT_MODE_MAILBOX_KHR {
        return 3;
    }
    if present_mode == VK_PRESENT_MODE_FIFO_KHR || present_mode == VK_PRESENT_MODE_FIFO_RELAXED_KHR {
        return 2;
    }
    if present_mode == VK_PRESENT_MODE_IMMEDIATE_KHR {
        return 1;
    }
    // IM_ASSERT(0);
    return 1;
}

// Also destroy old swap chain and in-flight frames data, if any.
pub fn ImGui_ImplVulkanH_CreateWindowSwapChain(physical_device: vk::PhysicalDevice, device: vk::Device, wd: *mut ImGui_ImplVulkanH_Window, allocator: *const vk::AllocationCallbacks, w: i32, h: i32, mut min_image_count: u32)
{
    let mut err: vk::Result;
    let mut old_swapchain: vk::SwapchainKHR = wd.Swapchain;
    wd.Swapchain = VK_NULL_HANDLE;
    err = vk::PFN_vkDeviceWaitIdle(device);
    check_vk_result(err);

    // We don't use ImGui_ImplVulkanH_DestroyWindow() because we want to preserve the old swapchain to create the new one.
    // Destroy old Framebuffer
    // for (uint32_t i = 0; i < wd.ImageCount; i++)
    for i in 0 .. wd.ImageCount
    {
        ImGui_ImplVulkanH_DestroyFrame(device, &mut wd.Frames[i], allocator);
        ImGui_ImplVulkanH_DestroyFrameSemaphores(device, &mut wd.FrameSemaphores[i], allocator);
    }
    unsafe { libc::free(wd.Frames as *mut c_void); }
    unsafe { libc::free(wd.FrameSemaphores as *mut c_void); }
    wd.Frames = null_mut();
    wd.FrameSemaphores = null_mut();
    wd.ImageCount = 0;
    if (wd.RenderPass) {
        vk::PFN_vkDestroyRenderPass(device, wd.RenderPass, allocator);
    }
    if (wd.Pipeline) {
        vk::PFN_vkDestroyPipeline(device, wd.Pipeline, allocator);
    }

    // If min image count was not specified, request different count of images dependent on selected present mode
    if (min_image_count == 0) {
        min_image_count = ImGui_ImplVulkanH_GetMinImageCountFromPresentMode(wd.PresentMode) as u32 as u32;
    }

    // Create Swapchain
    {
        let mut info: vk::SwapchainCreateInfoKHR = vk::SwapchainCreateInfoKHR{};
        info.sType = VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR;
        info.surface = wd.Surface;
        info.minImageCount = min_image_count;
        info.imageFormat = wd.SurfaceFormat.format;
        info.imageColorSpace = wd.SurfaceFormat.colorSpace;
        info.imageArrayLayers = 1;
        info.imageUsage = VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT;
        info.imageSharingMode = VK_SHARING_MODE_EXCLUSIVE;           // Assume that graphics family == present family
        info.preTransform = VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR;
        info.compositeAlpha = VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR;
        info.presentMode = wd.PresentMode;
        info.clipped = VK_TRUE;
        info.oldSwapchain = old_swapchain;
        let mut cap: vk::SurfaceCapabilitiesKHR = vk::SurfaceCapabilitiesKHR{};
        err = vk::PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR(physical_device, wd.Surface, &cap);
        check_vk_result(err);
        if (info.minImageCount < cap.minImageCount) {
            info.minImageCount = cap.minImageCount;
        }
        else if (cap.maxImageCount != 0 && info.minImageCount > cap.maxImageCount) {
            info.minImageCount = cap.maxImageCount;
        }

        if (cap.currentExtent.width == 0xffffffff)
        {
            info.imageExtent.width = wd.Width = w;
            info.imageExtent.height = wd.Height = h;
        }
        else
        {
            info.imageExtent.width = wd.Width = cap.currentExtent.width;
            info.imageExtent.height = wd.Height = cap.currentExtent.height;
        }
        err = vk::PFN_vkCreateSwapchainKHR(device, &info, allocator, &wd.Swapchain);
        check_vk_result(err);
        err = vk::PFN_vkGetSwapchainImagesKHR(device, wd.Swapchain, &wd.ImageCount, null_mut());
        check_vk_result(err);
        let mut backbuffers: [vk::Image;16] = [vk::Image{};16];
        IM_ASSERT(wd.ImageCount >= min_image_count);
        IM_ASSERT(wd.ImageCount < IM_ARRAYSIZE(backbuffers));
        err = vkGetSwapchainImagesKHR(device, wd.Swapchain, &wd.ImageCount, backbuffers);
        check_vk_result(err);

        IM_ASSERT(wd.Frames == null_mut());
        unsafe { wd.Frames = libc::malloc(std::mem::size_of::<ImGui_ImplVulkanH_Frame>() * wd.ImageCount) as *mut ImGui_ImplVulkanH_Frame; }
        unsafe { wd.FrameSemaphores = libc::malloc(std::mem::size_of::<ImGui_ImplVulkanH_FrameSemaphores>() * wd.ImageCount) as *mut ImGui_ImplVulkanH_FrameSemaphores; }
        unsafe { libc::memset(wd.Frames as *mut c_void, 0, std::mem::size_of::<ImGui_ImplVulkanH_Frame>() * wd.ImageCount); }
        unsafe { libc::memset(wd.FrameSemaphores as *mut c_void, 0, std::mem::size_of::<ImGui_ImplVulkanH_FrameSemaphores>() * wd.ImageCount); }
        // for (uint32_t i = 0; i < wd.ImageCount; i++)
        for i in 0 .. wd.ImageCount
        {
            wd.Frames[i].Backbuffer = backbuffers[i];
        }
    }
    if old_swapchain {
        vk::PFN_vkDestroySwapchainKHR(device, old_swapchain, allocator);
    }

    // Create the Render Pass
    {
        let mut attachment: vk::AttachmentDescription = vk::AttachmentDescription{};
        attachment.format = wd.SurfaceFormat.format;
        attachment.samples = VK_SAMPLE_COUNT_1_BIT;
        attachment.loadOp = if wd.ClearEnable { VK_ATTACHMENT_LOAD_OP_CLEAR } else { VK_ATTACHMENT_LOAD_OP_DONT_CARE };
        attachment.storeOp = VK_ATTACHMENT_STORE_OP_STORE;
        attachment.stencilLoadOp = VK_ATTACHMENT_LOAD_OP_DONT_CARE;
        attachment.stencilStoreOp = VK_ATTACHMENT_STORE_OP_DONT_CARE;
        attachment.initialLayout = VK_IMAGE_LAYOUT_UNDEFINED;
        attachment.finalLayout = VK_IMAGE_LAYOUT_PRESENT_SRC_KHR;
        let mut color_attachment: vk::AttachmentReference = vk::AttachmentReference{};
        color_attachment.attachment = 0;
        color_attachment.layout = VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL;
        let mut subpass: vk::SubpassDescription = vk::SubpassDescription{};
        subpass.pipelineBindPoint = VK_PIPELINE_BIND_POINT_GRAPHICS;
        subpass.colorAttachmentCount = 1;
        subpass.pColorAttachments = &color_attachment;
        let mut dependency: vk::SubpassDependency = vk::SubpassDependency{};
        dependency.srcSubpass = VK_SUBPASS_EXTERNAL;
        dependency.dstSubpass = 0;
        dependency.srcStageMask = VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT;
        dependency.dstStageMask = VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT;
        dependency.srcAccessMask = 0;
        dependency.dstAccessMask = VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT;
        let mut info: vk::RenderPassCreateInfo = vk::RenderPassCreateInfo{};
        info.sType = VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO;
        info.attachmentCount = 1;
        info.pAttachments = &attachment;
        info.subpassCount = 1;
        info.pSubpasses = &subpass;
        info.dependencyCount = 1;
        info.pDependencies = &dependency;
        err = vkCreateRenderPass(device, &info, allocator, &wd.RenderPass);
        check_vk_result(err);

        // We do not create a pipeline by default as this is also used by examples' main.cpp,
        // but secondary viewport in multi-viewport mode may want to create one with:
        //ImGui_ImplVulkan_CreatePipeline(device, allocator, VK_NULL_HANDLE, wd.RenderPass, VK_SAMPLE_COUNT_1_BIT, &wd.Pipeline, bd.Subpass);
    }

    // Create The Image Views
    {
        let mut info: vk::ImageViewCreateInfo = vk::ImageViewCreateInfo{};
        info.sType = VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO;
        info.viewType = VK_IMAGE_VIEW_TYPE_2D;
        info.format = wd.SurfaceFormat.format;
        info.components.r = VK_COMPONENT_SWIZZLE_R;
        info.components.g = VK_COMPONENT_SWIZZLE_G;
        info.components.b = VK_COMPONENT_SWIZZLE_B;
        info.components.a = VK_COMPONENT_SWIZZLE_A;
        let mut image_range: vk::ImageSubresourceRange = vk::ImageSubresourceRange{}; //{ VK_IMAGE_ASPECT_COLOR_BIT, 0, 1, 0, 1 };
        info.subresourceRange = image_range;
        // for (uint32_t i = 0; i < wd.ImageCount; i++)
        for i in 0 .. wd.ImageCount
        {
            ImGui_ImplVulkanH_Frame* fd = &wd.Frames[i];
            info.image = fd.Backbuffer;
            err = vkCreateImageView(device, &info, allocator, &fd.BackbufferView);
            check_vk_result(err);
        }
    }

    // Create Framebuffer
    {
        let mut attachment: [vk::ImageView;1] = [vk::ImageView];
        let mut info: vk::FramebufferCreateInfo = vk::FramebufferCreateInfo{};
        info.sType = VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO;
        info.renderPass = wd.RenderPass;
        info.attachmentCount = 1;
        info.pAttachments = attachment;
        info.width = wd.Width;
        info.height = wd.Height;
        info.layers = 1;
        // for (i = 0; i < wd.ImageCount; i++)
        for i in 0 .. wd.ImageCount
        {
            let fd: *mut ImGui_ImplVulkanH_Frame = &mut wd.Frames[i];
            attachment[0] = fd.BackbufferView;
            err = vk::PFN_vkCreateFramebuffer(device, &info, allocator, &fd.Framebuffer);
            check_vk_result(err);
        }
    }
}

// Create or resize window
pub fn ImGui_ImplVulkanH_CreateOrResizeWindow(instance: vk::Instance, physical_device: vk::PhysicalDevice, device: vk::Device, wd: *mut ImGui_ImplVulkanH_Window, queue_family: u32,  allocator: *const vk::AllocationCallbacks, width: i32, height: i32, min_image_count: u32)
{
    // IM_ASSERT(g_FunctionsLoaded && "Need to call ImGui_ImplVulkan_LoadFunctions() if IMGUI_IMPL_VULKAN_NO_PROTOTYPES or VK_NO_PROTOTYPES are set!");
    // (void)instance;
    ImGui_ImplVulkanH_CreateWindowSwapChain(physical_device, device, wd, allocator, width, height, min_image_count);
    //ImGui_ImplVulkan_CreatePipeline(device, allocator, VK_NULL_HANDLE, wd.RenderPass, VK_SAMPLE_COUNT_1_BIT, &wd.Pipeline, g_VulkanInitInfo.Subpass);
    ImGui_ImplVulkanH_CreateWindowCommandBuffers(physical_device, device, wd, queue_family, allocator);
}

pub fn ImGui_ImplVulkanH_DestroyWindow(instance: vk::Instance, device: vk::Device, wd: *mut ImGui_ImplVulkanH_Window, allocator: *const vk::AllocationCallbacks)
{
    vk::PFN_vkDeviceWaitIdle(device); // FIXME: We could wait on the Queue if we had the queue in wd. (otherwise VulkanH functions can't use globals)
    //vkQueueWaitIdle(bd.Queue);

    // for (uint32_t i = 0; i < wd.ImageCount; i++)
    for i in 0 .. wd.ImageCount
    {
        ImGui_ImplVulkanH_DestroyFrame(device, &mut wd.Frames[i], allocator);
        ImGui_ImplVulkanH_DestroyFrameSemaphores(device, &mut wd.FrameSemaphores[i], allocator);
    }
    unsafe { libc::free(wd.Frames as *mut c_void); }
    unsafe { libc::free(wd.FrameSemaphores as *mut c_void); }
    wd.Frames = null_mut();
    wd.FrameSemaphores = null_mut();
    vk::PFN_vkDestroyPipeline(device, wd.Pipeline, allocator);
    vk::PFN_vkDestroyRenderPass(device, wd.RenderPass, allocator);
    vk::PFN_vkDestroySwapchainKHR(device, wd.Swapchain, allocator);
    vk::PFN_vkDestroySurfaceKHR(instance, wd.Surface, allocator);

    unsafe { *wd = ImGui_ImplVulkanH_Window(); }
}

pub fn ImGui_ImplVulkanH_DestroyFrame(device: vk::Device, fd: *mut ImGui_ImplVulkanH_Frame, allocator: *const vk::AllocationCallbacks)
{
    vk::PFN_vkDestroyFence(device, fd.Fence, allocator);
    vk::PFN_vkFreeCommandBuffers(device, fd.CommandPool, 1, &fd.CommandBuffer);
    vk::PFN_vkDestroyCommandPool(device, fd.CommandPool, allocator);
    fd.Fence = VK_NULL_HANDLE;
    fd.CommandBuffer = VK_NULL_HANDLE;
    fd.CommandPool = VK_NULL_HANDLE;

    vk::PFN_vkDestroyImageView(device, fd.BackbufferView, allocator);
    vk::PFN_vkDestroyFramebuffer(device, fd.Framebuffer, allocator);
}

pub fn ImGui_ImplVulkanH_DestroyFrameSemaphores(device: vk::Device, fsd: *mut ImGui_ImplVulkanH_FrameSemaphores, allocator: *const vk::AllocationCallbacks)
{
    vk::PFN_vkDestroySemaphore(device, fsd.ImageAcquiredSemaphore, allocator);
    vk::PFN_vkDestroySemaphore(device, fsd.RenderCompleteSemaphore, allocator);
    fsd.ImageAcquiredSemaphore = fsd.RenderCompleteSemaphore = VK_NULL_HANDLE;
}

pub fn ImGui_ImplVulkanH_DestroyFrameRenderBuffers(device: VkDevice, buffers: * mut ImGui_ImplVulkanH_FrameRenderBuffers, allocator: *const vk::AllocationCallbacks)
{
    if (buffers.VertexBuffer) { vk::PFN_vkDestroyBuffer(device, buffers.VertexBuffer, allocator); buffers.VertexBuffer = VK_NULL_HANDLE; }
    if (buffers.VertexBufferMemory) { vk::PFN_vkFreeMemory(device, buffers.VertexBufferMemory, allocator); buffers.VertexBufferMemory = VK_NULL_HANDLE; }
    if (buffers.IndexBuffer) { vk::PFN_vkDestroyBuffer(device, buffers.IndexBuffer, allocator); buffers.IndexBuffer = VK_NULL_HANDLE; }
    if (buffers.IndexBufferMemory) { vk::PFN_vkFreeMemory(device, buffers.IndexBufferMemory, allocator); buffers.IndexBufferMemory = VK_NULL_HANDLE; }
    buffers.VertexBufferSize = 0;
    buffers.IndexBufferSize = 0;
}

pub fn ImGui_ImplVulkanH_DestroyWindowRenderBuffers(device: vk::Device, buffer: *mut ImGui_ImplVulkanH_WindowRenderBuffers, allocator: *const vk::AllocationCallbacks)
{
    // for (uint32_t n = 0; n < buffers.Count; n++)
    for n in 0 .. buffers.Count
    {
        ImGui_ImplVulkanH_DestroyFrameRenderBuffers(device, &mut buffers.FrameRenderBuffers[n], allocator);
    }
    unsafe { libc::free(buffers.FrameRenderBuffers); }
    buffers.FrameRenderBuffers = null_mut();
    buffers.Index = 0;
    buffers.Count = 0;
}

pub fn ImGui_ImplVulkanH_DestroyAllViewportsRenderBuffers(device: vk::Device, allocator: *const vk::AllocationCallbacks)
{
    let mut platform_io = GetPlatformIO();
    // for (int n = 0; n < platform_io.Viewports.Size; n++)
    for n in 0 .. platform_io.Viewports.len()
    {
        let mut vd = platform_io.Viewports[n].RendererUserData as *mut ImGui_ImplVulkan_ViewportData;
        if vd.is_null() == false
        {
            ImGui_ImplVulkanH_DestroyWindowRenderBuffers(device, &mut vd.RenderBuffers, allocator);
        }
    }
}

//--------------------------------------------------------------------------------------------------------
// MULTI-VIEWPORT / PLATFORM INTERFACE SUPPORT
// This is an _advanced_ and _optional_ feature, allowing the backend to create and handle multiple viewports simultaneously.
// If you are new to dear imgui or creating a new binding for dear imgui, it is recommended that you completely ignore this section first..
//--------------------------------------------------------------------------------------------------------

pub fn ImGui_ImplVulkan_CreateWindow(viewport: *mut ImGuiViewport)
{
    let mut bd = ImGui_ImplVulkan_GetBackendData();
    let mut vd = unsafe { libc::malloc(std::mem::size_of::<ImGui_ImplVulkan_ViewportData>()) } as *mut ImGui_ImplVulkan_ViewportData;
    viewport.RendererUserData = vd;
    let mut wd = &mut vd.Window;
    let mut v = &mut bd.VulkanInitInfo;

    // Create surface
    let mut platform_io = GetPlatformIO();
    let mut err = platform_io.Platform_CreateVkSurface(viewport, v.Instance, v.Allocator, &wd.Surface);
    check_vk_result(err);

    // Check for WSI support
    let mut res = 0i32;
    vkGetPhysicalDeviceSurfaceSupportKHR(v.PhysicalDevice, v.QueueFamily, wd.Surface, &res);
    if res != VK_TRUE
    {
        // IM_ASSERT(0); // Error: no WSI support on physical device
        return;
    }

    // Select Surface Format
    let mut requestSurfaceImageFormat: [vk::Format;4] = [ VK_FORMAT_B8G8R8A8_UNORM, VK_FORMAT_R8G8B8A8_UNORM, VK_FORMAT_B8G8R8_UNORM, VK_FORMAT_R8G8B8_UNORM ];
    let mut requestSurfaceColorSpace: vk::ColorSpaceKHR = VK_COLORSPACE_SRGB_NONLINEAR_KHR;
    wd.SurfaceFormat = ImGui_ImplVulkanH_SelectSurfaceFormat(v.PhysicalDevice, wd.Surface, requestSurfaceImageFormat.as_ptr(), requestSurfaceImageFormat.len() as i32, requestSurfaceColorSpace);

    // Select Present Mode
    // FIXME-VULKAN: Even thought mailbox seems to get us maximum framerate with a single window, it halves framerate with a second window etc. (w/ Nvidia and SDK 1.82.1)
    let mut present_modes: [VkPresentModeKHR;3] = [VK_PRESENT_MODE_MAILBOX_KHR, VK_PRESENT_MODE_IMMEDIATE_KHR, VK_PRESENT_MODE_FIFO_KHR ];
    wd.PresentMode = ImGui_ImplVulkanH_SelectPresentMode(v.PhysicalDevice, wd.Surface, &present_modes[0], (present_modes.len()) as i32);
    //printf("[vulkan] Secondary window selected PresentMode = %d\n", wd.PresentMode);

    // Create SwapChain, RenderPass, Framebuffer, etc.
    wd.ClearEnable = if viewport.Flags & ImGuiViewportFlags_NoRendererClear { false } else { true };
    ImGui_ImplVulkanH_CreateOrResizeWindow(v.Instance, v.PhysicalDevice, v.Device, wd, v.QueueFamily, v.Allocator, viewport.Size.x, viewport.Size.y, v.MinImageCount);
    vd.WindowOwned = true;
}

pub fn ImGui_ImplVulkan_DestroyWindow(viewport: *mut ImGuiViewport)
{
    // The main viewport (owned by the application) will always have RendererUserData == NULL since we didn't create the data for it.
    let mut bd = ImGui_ImplVulkan_GetBackendData();
    let mut vd = viewport.RendererUserData as *mut ImGui_ImplVulkan_ViewportData;
    if vd.is_null() == false
    {
        let v = &mut bd.VulkanInitInfo;
        if vd.WindowOwned {
            ImGui_ImplVulkanH_DestroyWindow(v.Instance, v.Device, &mut vd.Window, v.Allocator);
        }
        ImGui_ImplVulkanH_DestroyWindowRenderBuffers(v.Device, &mut vd.RenderBuffers, v.Allocator);
        unsafe { libc::free(vd); }
    }
    viewport.RendererUserData = null_mut();
}

pub fn ImGui_ImplVulkan_SetWindowSize(viewport: *mut ImGuiViewport, size: ImVec2)
{
    let mut bd: *mut ImGui_ImplVulkan_Data =  ImGui_ImplVulkan_GetBackendData();
    let mut vd = viewport.RendererUserData;
    // This is NULL for the main viewport (which is left to the user/app to handle)
    if (vd == null_mut()) { return; }
    ImGui_ImplVulkan_InitInfo* v = &bd.VulkanInitInfo;
    vd.Window.ClearEnable = if (viewport.Flags & ImGuiViewportFlags_NoRendererClear) { false } else { true; }
    ImGui_ImplVulkanH_CreateOrResizeWindow(v.Instance, v.PhysicalDevice, v.Device, &mut vd.Window, v.QueueFamily, v.Allocator, size.x, size.y, v.MinImageCount);
}

pub fn ImGui_ImplVulkan_RenderWindow(viewport: *mut ImGuiViewport)
{
    let mut bd: *mut ImGui_ImplVulkan_Data =  ImGui_ImplVulkan_GetBackendData();
    ImGui_ImplVulkan_ViewportData* vd = viewport.RendererUserData;
    ImGui_ImplVulkanH_Window* wd = &vd.Window;
    ImGui_ImplVulkan_InitInfo* v = &bd.VulkanInitInfo;
    let mut err: vk::Result = vk::Result{ 0: 0 };

    let mut fd = &mut wd.Frames[wd.FrameIndex];
    let mut fsd = &mut wd.FrameSemaphores[wd.SemaphoreIndex];
    if 1
    {
        if 1
        {
          err = vk::PFN_vkAcquireNextImageKHR(v.Device, wd.Swapchain, UINT64_MAX, fsd.ImageAcquiredSemaphore, VK_NULL_HANDLE, &wd.FrameIndex);
          check_vk_result(err);
          fd = &mut wd.Frames[wd.FrameIndex];
        }
        loop
        {
            err = vk::PFN_vkWaitForFences(v.Device, 1, &fd.Fence, VK_TRUE, 100);
            if (err == VK_SUCCESS) { break; }
            if (err == VK_TIMEOUT) { continue; }
            check_vk_result(err);
        }
        if 1
        {
            err = vk::PFN_vkResetCommandPool(v.Device, fd.CommandPool, 0);
            check_vk_result(err);
            let mut info: vk::CommandBufferBeginInfo = vk::CommandBufferBeginInfo{};
            info.sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO;
            info.flags |= VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT;
            err = vk::PFN_vkBeginCommandBuffer(fd.CommandBuffer, &info);
            check_vk_result(err);
        }
        if 1
        {
            let mut clear_color = ImVec4::new4(0.0f32, 0.0f32, 0.0f32, 1.0f32);
            // unsafe { libc::memcpy(&mut wd.ClearValue.color.float32[0], &mut clear_color as mut *c_void, 4 * sizeof); }

            let mut info: vk::RenderPassBeginInfo = vk::RenderPassBeginInfo{};
            info.sType = VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO;
            info.renderPass = wd.RenderPass;
            info.framebuffer = fd.Framebuffer;
            info.renderArea.extent.width = wd.Width;
            info.renderArea.extent.height = wd.Height;
            info.clearValueCount = if (viewport.Flags & ImGuiViewportFlags_NoRendererClear) { 0 } else { 1 };
            info.pClearValues = if (viewport.Flags & ImGuiViewportFlags_NoRendererClear) { null_mut() } else { &wd.ClearValue };
            vk::PFN_vkCmdBeginRenderPass(fd.CommandBuffer, &info, VK_SUBPASS_CONTENTS_INLINE);
        }
    }

    ImGui_ImplVulkan_RenderDrawData(viewport.DrawData, fd.CommandBuffer, wd.Pipeline);

    {
        vk::PFN_vkCmdEndRenderPass(fd.CommandBuffer);
        {
            let mut wait_stage = VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT;
            let mut info: vk::SubmitInfo = vk::SubmitInfo{};
            info.sType = VK_STRUCTURE_TYPE_SUBMIT_INFO;
            info.waitSemaphoreCount = 1;
            info.pWaitSemaphores = &fsd.ImageAcquiredSemaphore;
            info.pWaitDstStageMask = &wait_stage;
            info.commandBufferCount = 1;
            info.pCommandBuffers = &fd.CommandBuffer;
            info.signalSemaphoreCount = 1;
            info.pSignalSemaphores = &fsd.RenderCompleteSemaphore;

            err = vkEndCommandBuffer(fd.CommandBuffer);
            check_vk_result(err);
            err = vkResetFences(v.Device, 1, &fd.Fence);
            check_vk_result(err);
            err = vkQueueSubmit(v.Queue, 1, &info, fd.Fence);
            check_vk_result(err);
        }
    }
}

pub fn ImGui_ImplVulkan_SwapBuffers(viewport: *mut ImGuiViewport)
{
    let mut bd: *mut ImGui_ImplVulkan_Data =  ImGui_ImplVulkan_GetBackendData();
    let mut  vd = &mut viewport.RendererUserData;
   let mut  wd = &mut vd.Window;
    let mut  v = &mut bd.VulkanInitInfo;

    let mut err: vk::Result = vk::Result{ 0: 0 };
    let mut present_index: u32 = wd.FrameIndex;

    let mut fsd = &mut wd.FrameSemaphores[wd.SemaphoreIndex];
    let mut info: vk::PresentInfoKHR = vk::PresentInfoKHR{};
    info.sType = VK_STRUCTURE_TYPE_PRESENT_INFO_KHR;
    info.waitSemaphoreCount = 1;
    info.pWaitSemaphores = &fsd.RenderCompleteSemaphore;
    info.swapchainCount = 1;
    info.pSwapchains = &wd.Swapchain;
    info.pImageIndices = &present_index;
    err = vk::PFN_vkQueuePresentKHR(v.Queue, &info);
    if (err == VK_ERROR_OUT_OF_DATE_KHR || err == VK_SUBOPTIMAL_KHR) {
        ImGui_ImplVulkanH_CreateOrResizeWindow(v.Instance, v.PhysicalDevice, v.Device, &mut vd.Window, v.QueueFamily, v.Allocator, viewport.Size.x, viewport.Size.y, v.MinImageCount);
    }
    else {
        check_vk_result(err);
    }

    wd.FrameIndex = (wd.FrameIndex + 1) % wd.ImageCount;         // This is for the next vkWaitForFences()
    wd.SemaphoreIndex = (wd.SemaphoreIndex + 1) % wd.ImageCount; // Now we can use the next set of semaphores
}

pub fn  ImGui_ImplVulkan_InitPlatformInterface()
{
    ImGuiPlatformIO& platform_io = Imgui::GetPlatformIO();
    if (Imgui::GetIO().ConfigFlags & ImGuiConfigFlags_ViewportsEnable) {}
        // IM_ASSERT(platform_io.Platform_CreateVkSurface != null_mut() && "Platform needs to setup the CreateVkSurface handler.");
    platform_io.Renderer_CreateWindow = ImGui_ImplVulkan_CreateWindow;
    platform_io.Renderer_DestroyWindow = ImGui_ImplVulkan_DestroyWindow;
    platform_io.Renderer_SetWindowSize = ImGui_ImplVulkan_SetWindowSize;
    platform_io.Renderer_RenderWindow = ImGui_ImplVulkan_RenderWindow;
    platform_io.Renderer_SwapBuffers = ImGui_ImplVulkan_SwapBuffers;
}

pub fn  ImGui_ImplVulkan_ShutdownPlatformInterface(app_ctx: &mut AppContext)
{
    DestroyPlatformWindows(app_ctx);
}
