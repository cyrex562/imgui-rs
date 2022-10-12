#![allow(non_snake_case)]

use std::ptr::null_mut;
use libc::{c_int, size_t};
use crate::draw_cmd::ImDrawCmd;
use crate::draw_list::ImDrawList;
use crate::draw_vert::ImDrawVert;
use crate::vec2::ImVec2;
use crate::vec4::ImVec4;
use crate::viewport::ImGuiViewport;

// All draw data to render a Dear ImGui frame
// (NB: the style and the naming convention here is a little inconsistent, we currently preserve them for backward compatibility purpose,
// as this is one of the oldest structure exposed by the library! Basically, ImDrawList == CmdList)
#[derive(Default,Debug,Clone)]
pub struct ImDrawData {
    pub Valid: bool,
    // Only valid after Render() is called and before the next NewFrame() is called.
    pub CmdListsCount: size_t,
    // Number of ImDrawList* to render
    pub TotalIdxCount: size_t,
    // For convenience, sum of all ImDrawList's IdxBuffer.Size
    pub TotalVtxCount: size_t,
    // For convenience, sum of all ImDrawList's VtxBuffer.Size
    pub CmdLists: *mut *mut ImDrawList,
    // Array of ImDrawList* to render. The ImDrawList are owned by ImGuiContext and only pointed to from here.
    pub DisplayPos: ImVec2,
    // Top-left position of the viewport to render (== top-left of the orthogonal projection matrix to use) (== GetMainViewport()->Pos for the main viewport, == (0.0) in most single-viewport applications)
    pub DisplaySize: ImVec2,
    // Size of the viewport to render (== GetMainViewport()->Size for the main viewport, == io.DisplaySize in most single-viewport applications)
    pub FramebufferScale: ImVec2,
    // Amount of pixels for each unit of DisplaySize. Based on io.DisplayFramebufferScale. Generally (1,1) on normal display, (2,2) on OSX with Retina display.
    pub OwnerViewport: *mut ImGuiViewport,          // Viewport carrying the ImDrawData instance, might be of use to the renderer (generally not).
}

impl ImDrawData {

    // Functions
    // ImDrawData()    { Clear(); }


    // void Clear()    { memset(this, 0, sizeof(*this)); }     // The ImDrawList are owned by ImGuiContext!
    pub fn Clear(&mut self) {
        self.Valid = false;
        self.CmdListsCount = 0;
        self.TotalIdxCount = 0;
        self.TotalVtxCount = 0;
        self.CmdLists = null_mut();
        self.DisplayPos = ImVec2::default();
        self.DisplaySize = ImVec2::default();
        self.FramebufferScale = ImVec2::default();
        self.OwnerViewport = null_mut();
    }

    // void  DeIndexAllBuffers();                    // Helper to convert all buffers from indexed to non-indexed, in case you cannot render indexed. Note: this is slow and most likely a waste of resources. Always prefer indexed rendering!
    pub fn DeIndexAllBuffers(&mut self) {
        
        // Vec<ImDrawVert> new_vtx_buffer;
        let mut new_vtx_buffer: Vec<ImDrawVert> = vec![];
        self.TotalVtxCount = 0;
        self.TotalIdxCount = 0;
    // for (let i: c_int = 0; i < CmdListsCount; i++)
    for i in 0 .. self.CmdListsCount
        {
        let mut  cmd_list: *mut ImDrawList =  self.CmdLists[i];
        if cmd_list.IdxBuffer.empty() {
            continue;
        }
        new_vtx_buffer.resize_with(cmd_list.IdxBuffer.len(), ImDrawCmd::default());
        // for (let j: c_int = 0; j < cmd_list.IdxBuffer.len(); j++)
        for j in 0 .. cmd_list.IdxBuffer.len()
            {
            new_vtx_buffer[j] = cmd_list.VtxBuffer[cmd_list.IdxBuffer[j]];
        }
        // cmd_list.VtxBuffer.swap(new_vtx_buffer, 0);
        cmd_list.IdxBuffer.clear();
        self.TotalVtxCount += cmd_list.VtxBuffer.len();
    }
    }

    // void  ScaleClipRects(const ImVec2& fb_scale); // Helper to scale the ClipRect field of each ImDrawCmd. Use if your final output buffer is at a different scale than Dear ImGui expects, or if there is a difference between your window resolution and framebuffer resolution.
    pub fn ScaleClipRects(&mut self, fb_scale: &ImVec2) {
        // for (let i: c_int = 0; i < CmdListsCount; i++)
        for i in 0 .. self.CmdListsCount
        {
            let mut  cmd_list: *mut ImDrawList =  self.CmdLists[i];
            // for (let cmd_i: c_int = 0; cmd_i < cmd_list.CmdBuffer.len(); cmd_i++)
            for i in 0 .. cmd_list.CmdBuffer.len()
            {
                let cmd: *mut ImDrawCmd = &mut cmd_list.CmdBuffer[cmd_i];
                cmd.ClipRect = ImVec4::new2(cmd.ClipRect.x * fb_scale.x, cmd.ClipRect.y * fb_scale.y, cmd.ClipRect.z * fb_scale.x, cmd.ClipRect.w * fb_scale.y);
            }
        }
    }
}



#[derive(Default,Debug,Clone)]
pub struct ImDrawDataBuilder
{
    // ImVector<ImDrawList*>   Layers[2];           // Global layers for: regular, tooltip
    pub Layers: [Vec<*mut ImDrawList>;2],

}

impl ImDrawDataBuilder {
    // void Clear()                    { for (n: c_int = 0; n < IM_ARRAYSIZE(Layers); n++) Layers[n].resize(0); }
    pub fn Clear(&mut self) {
        self.Layers[0].clear();
        self.Layers[1].clear();
    }

    // void ClearFreeMemory()          { for (n: c_int = 0; n < IM_ARRAYSIZE(Layers); n++) Layers[n].clear(); }


    // c_int  GetDrawListCount() const   { count: c_int = 0; for (n: c_int = 0; n < IM_ARRAYSIZE(Layers); n++) count += Layers[n].Size; return count; }
    pub fn GetDrawListCount(&self) -> c_int {
        (self.Layers[0].len() + self.Layers[1].len()) as c_int
    }

    // void FlattenIntoSingleLayer();
    // pub fn FlattenIntoSingleLayer(&mut self) {
    //     todo!()
    // }
    // c_void ImDrawDataBuilder::FlattenIntoSingleLayer()
    pub unsafe fn FlattenIntoSingleLayer(&mut self)
    {
        let mut n: c_int = self.Layers[0].Size;
        let mut size: c_int = n;
        // for (let i: c_int = 1; i < IM_ARRAYSIZE(Layers); i++)
        for i in 1 .. self.Layers.len()
        {
            size += self.Layers[i].Size;
        }
        // self.Layers[0].resize(size);
        // for (let layer_n: c_int = 1; layer_n < IM_ARRAYSIZE(Layers); layer_n++)
        for layer_n in 1 .. self.Layers.len()
        {
            let mut layer = self.Layers[layer_n].clone();
            if layer.empty() {
                continue;
            }
            libc::memcpy(&mut self.Layers[0][n], &layer[0], layer.Size * libc::sizeof(ImDrawList));
            n += layer.Size;
            // layer.resize(0);
        }
    }
}
