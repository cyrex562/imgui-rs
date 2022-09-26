#![allow(non_snake_case)]

use std::ptr::null_mut;
use libc::c_int;
use crate::drawlist::ImDrawList;
use crate::imgui_vec2::ImVec2;
use crate::imgui_viewport::ImGuiViewport;

// All draw data to render a Dear ImGui frame
// (NB: the style and the naming convention here is a little inconsistent, we currently preserve them for backward compatibility purpose,
// as this is one of the oldest structure exposed by the library! Basically, ImDrawList == CmdList)
#[derive(Default,Debug,Clone)]
pub struct ImDrawData
{
pub Valid: bool,                  // Only valid after Render() is called and before the next NewFrame() is called.
pub CmdListsCount: c_int,          // Number of ImDrawList* to render
pub TotalIdxCount: c_int,          // For convenience, sum of all ImDrawList's IdxBuffer.Size
pub TotalVtxCount: c_int,          // For convenience, sum of all ImDrawList's VtxBuffer.Size
pub CmdLists: *mut *mut ImDrawList,               // Array of ImDrawList* to render. The ImDrawList are owned by ImGuiContext and only pointed to from here.
pub DisplayPos: ImVec2,             // Top-left position of the viewport to render (== top-left of the orthogonal projection matrix to use) (== GetMainViewport()->Pos for the main viewport, == (0.0) in most single-viewport applications)
pub DisplaySize: ImVec2,            // Size of the viewport to render (== GetMainViewport()->Size for the main viewport, == io.DisplaySize in most single-viewport applications)
pub FramebufferScale: ImVec2,       // Amount of pixels for each unit of DisplaySize. Based on io.DisplayFramebufferScale. Generally (1,1) on normal display, (2,2) on OSX with Retina display.
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
        self.DisplayPos = ImVec2::new();
        self.DisplaySize = ImVec2::new();
        self.FramebufferScale = ImVec2::new();
        self.OwnerViewport = null_mut();
    }

    // void  DeIndexAllBuffers();                    // Helper to convert all buffers from indexed to non-indexed, in case you cannot render indexed. Note: this is slow and most likely a waste of resources. Always prefer indexed rendering!
    pub fn DeIndexAllBuffers(&mut self) {
        todo!()
    }

    // void  ScaleClipRects(const ImVec2& fb_scale); // Helper to scale the ClipRect field of each ImDrawCmd. Use if your final output buffer is at a different scale than Dear ImGui expects, or if there is a difference between your window resolution and framebuffer resolution.
    pub fn ScaleClipRects(&mut self, fb_scale: &ImVec2) {
        todo!()
    }
}



#[derive(Default,Debug,Clone)]
pub struct ImDrawDataBuilder
{
    // ImVector<ImDrawList*>   Layers[2];           // Global layers for: regular, tooltip
    pub Layers: [Vec<*mut ImDrawList>;2],

}

impl ImDrawDataBuilder {
    // void Clear()                    { for (c_int n = 0; n < IM_ARRAYSIZE(Layers); n++) Layers[n].resize(0); }
    pub fn Clear(&mut self) {
        self.Layers[0].clear();
        self.Layers[1].clear();
    }

    // void ClearFreeMemory()          { for (c_int n = 0; n < IM_ARRAYSIZE(Layers); n++) Layers[n].clear(); }


    // c_int  GetDrawListCount() const   { c_int count = 0; for (c_int n = 0; n < IM_ARRAYSIZE(Layers); n++) count += Layers[n].Size; return count; }
    pub fn GetDrawListCount(&self) -> c_int {
        (self.Layers[0].len() + self.Layers[1].len()) as c_int
    }

    // void FlattenIntoSingleLayer();
    pub fn FlattenIntoSingleLayer(&mut self) {
        todo!()
    }
}
