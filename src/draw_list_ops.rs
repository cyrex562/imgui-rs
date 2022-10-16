#![allow(non_snake_case)]

use std::ptr::null_mut;
use libc::c_char;
use crate::draw_list::ImDrawList;
use crate::draw_list_shared_data::ImDrawListSharedData;
use crate::imgui::GImGui;
use crate::string_ops::str_to_const_c_char_ptr;
use crate::viewport::ImGuiViewport;

// static ImDrawList* GetViewportDrawList(viewport: *mut ImGuiViewport, drawlist_no: size_t, drawlist_name: *const c_char)
pub unsafe fn GetViewportDrawList(viewport: *mut ImGuiViewport, drawlist_no: c_sizet, drawlist_name: *const c_char) -> *mut ImDrawList
{
    // Create the draw list on demand, because they are not frequently used for all viewports
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT(drawlist_no < IM_ARRAYSIZE(viewport.DrawLists));
    let mut  draw_list: *mut ImDrawList =  viewport.DrawLists[drawlist_no];
    if draw_list == null_mut()
    {
        draw_list = IM_NEW(ImDrawList)(&g.DrawListSharedData);
        draw_list._OwnerName = drawlist_name;
        viewport.DrawLists[drawlist_no] = draw_list;
    }

    // Our ImDrawList system requires that there is always a command
    if viewport.DrawListsLastFrame[drawlist_no] != g.FrameCount
    {
        draw_list._ResetForNewFrame();
        draw_list.PushTextureID(g.IO.Fonts.TexID);
        draw_list.PushClipRect(&viewport.Pos.clone(), &viewport.Pos.clone() + &viewport.Size.clone(), false);
        viewport.DrawListsLastFrame[drawlist_no] = g.FrameCount;
    }
    return draw_list;
}

// ImDrawList* GetBackgroundDrawList(viewport: *mut ImGuiViewport)
pub unsafe fn GetBackgroundDrawList(viewport: *mut ImGuiViewport) -> *mut ImDrawList
{
    return GetViewportDrawList(viewport, 0, str_to_const_c_char_ptr("##Background"));
}

// ImDrawList* GetBackgroundDrawList()
pub unsafe fn GetBackgroundDrawList2() -> *mut ImDrawList
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return GetBackgroundDrawList(g.Currentwindow.Viewport);
}

// ImDrawList* GetForegroundDrawList(viewport: *mut ImGuiViewport)
pub unsafe fn GetForegroundDrawList(viewport: *mut ImGuiViewport) -> *mut ImDrawList
{
    return GetViewportDrawList(viewport, 1, str_to_const_c_char_ptr("##Foreground"));
}

// ImDrawList* GetForegroundDrawList()
pub unsafe fn GetForegroundDrawList2() -> *mut ImDrawList
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    return GetForegroundDrawList(g.Currentwindow.Viewport);
}

// ImDrawListSharedData* GetDrawListSharedData()
pub unsafe fn GetDrawListSharedData() -> *mut ImDrawListSharedData
{
    return &mut GimGui.DrawListSharedData;
}
