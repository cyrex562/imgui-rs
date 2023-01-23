#![allow(non_snake_case)]

use crate::drawing::draw_data::ImDrawData;
use crate::drawing::draw_list::ImDrawList;
use crate::drawing::draw_list_flags::ImDrawListFlags_AllowVtxOffset;
use crate::core::type_defs::DrawIndex;
use crate::window::ops::{GetWindowDisplayLayer, IsWindowActiveAndVisible};
use crate::window::window_flags::ImGuiWindowFlags_DockNodeHost;
use crate::window::ImguiWindow;
use crate::window_flags::ImGuiWindowFlags_DockNodeHost;
use crate::window_ops::{GetWindowDisplayLayer, IsWindowActiveAndVisible};
use crate::{GImGui, ImguiViewport};
use libc::c_int;
use std::ptr::null_mut;

// Pass this to your backend rendering function! Valid after Render() and until the next call to NewFrame()
// ImDrawData* GetDrawData()
pub unsafe fn GetDrawData() -> *mut ImDrawData {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut viewport: *mut ImguiViewport = g.Viewports[0];
    return if viewport.DrawDataP.Valid {
        &mut viewport.DrawDataP
    } else {
        None
    };
}

// static c_void AddDrawListToDrawData(Vec<ImDrawList*>* out_list, draw_list: *mut ImDrawList)
pub fn AddDrawListToDrawData(out_list: &mut Vec<*mut ImDrawList>, draw_list: *mut ImDrawList) {
    if draw_list.CmdBuffer.len() == 0 {
        return;
    }
    if draw_list.CmdBuffer.len() == 1
        && draw_list.CmdBuffer[0].ElemCount == 0
        && draw_list.CmdBuffer[0].UserCallback == None
    {
        return;
    }

    // Draw list sanity check. Detect mismatch between PrimReserve() calls and incrementing _VtxCurrentIdx, _VtxWritePtr etc.
    // May trigger for you if you are using PrimXXX functions incorrectly.
    // IM_ASSERT(draw_list.VtxBuffer.Size == 0 || draw_list._VtxWritePtr == draw_list.VtxBuffer.Data + draw_list.VtxBuffer.Size);
    // IM_ASSERT(draw_list.IdxBuffer.Size == 0 || draw_list._IdxWritePtr == draw_list.IdxBuffer.Data + draw_list.IdxBuffer.Size);
    if flag_clear(draw_list.Flags, ImDrawListFlags_AllowVtxOffset) {}
    // IM_ASSERT(draw_list._VtxCurrentIdx == draw_list.VtxBuffer.Size);

    // Check that draw_list doesn't use more vertices than indexable (default ImDrawIdx = unsigned short = 2 bytes = 64K vertices per ImDrawList = per window)
    // If this assert triggers because you are drawing lots of stuff manually:
    // - First, make sure you are coarse clipping yourself and not trying to draw many things outside visible bounds.
    //   Be mindful that the ImDrawList API doesn't filter vertices. Use the Metrics/Debugger window to inspect draw list contents.
    // - If you want large meshes with more than 64K vertices, you can either:
    //   (A) Handle the ImDrawCmd::VtxOffset value in your renderer backend, and set 'io.BackendFlags |= IM_GUI_BACKEND_FLAGS_RENDERER_HAS_VTX_OFFSET'.
    //       Most example backends already support this from 1.71. Pre-1.71 backends won't.
    //       Some graphics API such as GL ES 1/2 don't have a way to offset the starting vertex so it is not supported for them.
    //   (B) Or handle 32-bit indices in your renderer backend, and uncomment '#define ImDrawIdx unsigned int' line in imconfig.h.
    //       Most example backends already support this. For example, the OpenGL example code detect index size at compile-time:
    //         glDrawElements(GL_TRIANGLES, (GLsizei)pcmd->ElemCount, sizeof(ImDrawIdx) == 2 ? GL_UNSIGNED_SHORT : GL_UNSIGNED_INT, idx_buffer_offset);
    //       Your own engine or render API may use different parameters or function calls to specify index sizes.
    //       2 and 4 bytes indices are generally supported by most graphics API.
    // - If for some reason neither of those solutions works for you, a workaround is to call BeginChild()/EndChild() before reaching
    //   the 64K limit to split your draw commands in multiple draw lists.
    if libc::sizeof == 2 {}
    // IM_ASSERT(draw_list._VtxCurrentIdx < (1 << 16) && "Too many vertices in ImDrawList using 16-bit indices. Read comment above");

    out_list.push(draw_list);
}

// static c_void AddWindowToDrawData(window: &mut ImGuiWindow, layer: c_int)
pub unsafe fn AddWindowToDrawData(window: &mut ImguiWindow, layer: c_int) {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut viewport: *mut ImguiViewport = window.Viewport;
    g.IO.MetricsRenderWindows += 1;
    if window.Flags & ImGuiWindowFlags_DockNodeHost {
        window.DrawList.ChannelsMerge();
    }
    AddDrawListToDrawData(&mut viewport.DrawDataBuilder.Layers[layer], window.DrawList);
    // for (let i: c_int = 0; i < window.dc.ChildWindows.Size; i++)
    for i in 0..window.dc.ChildWindows.len() {
        let mut child: *mut ImguiWindow = window.dc.ChildWindows[i];
        if IsWindowActiveAndVisible(child) {
            // Clipped children may have been marked not active
            AddWindowToDrawData(child, layer);
        }
    }
}

// Layer is locked for the root window, however child windows may use a different viewport (e.g. extruding menu)
// pub unsafe fn AddRootWindowToDrawData(window: &mut ImGuiWindow)
pub unsafe fn AddRootWindowToDrawData(window: &mut ImguiWindow) {
    AddWindowToDrawData(window, GetWindowDisplayLayer(window));
}
