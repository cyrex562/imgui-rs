use crate::axis::{ImGuiAxis_X, ImGuiAxis_Y};
use crate::color::{ImGuiCol_ChildBg, ImGuiCol_FrameBg};
use crate::condition::ImGuiCond_None;
use crate::content_ops::content_region_avail;
use crate::id_ops::SetActiveID;
use crate::input_source::ImGuiInputSource_Nav;
use crate::item_ops::{ItemAdd, ItemSize};
use crate::item_status_flags::ImGuiItemStatusFlags_HoveredWindow;
use crate::math_ops::ImMax;
use crate::nav_highlight_flags::ImGuiNavHighlightFlags_TypeThin;
use crate::nav_ops::NavInitWindow;
use crate::rect::ImRect;
use crate::render_ops::RenderNavHighlight;
use crate::string_ops::ImFormatStringToTempBuffer;
use crate::style_ops::{PopStyleColor, PushStyleColor};
use crate::style_var::{
    ImGuiStyleVar_ChildBorderSize, ImGuiStyleVar_ChildRounding, ImGuiStyleVar_WindowPadding,
};
use crate::style_var_ops::{
    PopStyleVar, PopStyleVarInt, PushStyleVar, PushStyleVarFloat, PushStyleVarVec2,
};
use crate::type_defs::ImguiHandle;
use crate::utils::flag_clear;
use crate::vec2::ImVec2;
use crate::window::focus::FocusWindow;
use crate::window::ops::{Begin, GetCurrentWindow, SetNextWindowSize};
use crate::window::window_flags::{
    ImGuiWindowFlags, ImGuiWindowFlags_AlwaysUseWindowPadding, ImGuiWindowFlags_ChildWindow,
    ImGuiWindowFlags_NavFlattened, ImGuiWindowFlags_NoDocking, ImGuiWindowFlags_NoMove,
    ImGuiWindowFlags_NoResize, ImGuiWindowFlags_NoSavedSettings, ImGuiWindowFlags_NoTitleBar,
};
use crate::window::ImguiWindow;
use crate::window_flags::{
    ImGuiWindowFlags, ImGuiWindowFlags_AlwaysUseWindowPadding, ImGuiWindowFlags_ChildWindow,
    ImGuiWindowFlags_NavFlattened, ImGuiWindowFlags_NoDocking, ImGuiWindowFlags_NoMove,
    ImGuiWindowFlags_NoResize, ImGuiWindowFlags_NoSavedSettings, ImGuiWindowFlags_NoTitleBar,
};
use crate::window_ops::SetNextWindowSize;
use crate::GImGui;
use libc::{c_char, c_float, c_int};
use std::borrow::BorrowMut;
use std::io::SeekFrom::End;
use std::ptr::{null, null_mut};

// BeginChildEx: bool(name: *const c_char, ImguiHandle id, const size_arg: &mut ImVec2, border: bool, ImGuiWindowFlags flags)
pub unsafe fn BeginChildEx(
    name: String,
    id: ImguiHandle,
    size_arg: ImVec2,
    border: bool,
    mut flags: ImGuiWindowFlags,
) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut parent_window = g.current_window_mut().unwrap();

    flags |= ImGuiWindowFlags_NoTitleBar
        | ImGuiWindowFlags_NoResize
        | ImGuiWindowFlags_NoSavedSettings
        | ImGuiWindowFlags_ChildWindow
        | ImGuiWindowFlags_NoDocking;
    flags |= (parent_window.Flags & ImGuiWindowFlags_NoMove); // Inherit the NoMove flag

    // Size
    let content_avail: ImVec2 = content_region_avail(g);
    let mut size: ImVec2 = ImFloor(size_arg);
    let auto_fit_axises: c_int = (if size.x == 0.0 {
        (1 << ImGuiAxis_X)
    } else {
        0x00
    }) | (if size.y == 0.0 {
        (1 << ImGuiAxis_Y)
    } else {
        0x00
    });
    if size.x <= 0.0 {
        size.x = ImMax(content_avail.x + size.x, 4.0);
    } // Arbitrary minimum child size (0.0 causing too much issues)
    if size.y <= 0.0 {
        size.y = ImMax(content_avail.y + size.y, 4.0);
    }
    SetNextWindowSize(size, ImGuiCond_None);

    // Build up name. If you need to append to a same child from multiple location in the ID stack, use BeginChild(ImguiHandle id) with a stable value.
    let mut temp_window_name: String = String::default();
    if name {
        // TODO:
        // ImFormatStringToTempBuffer(&mut temp_window_name, None, "{}/{}_{}", parent_window.Name, name, id);
    } else {
        // TODO:
        // ImFormatStringToTempBuffer(&mut temp_window_name, None, "{}/{}", parent_window.Name, id);
    }

    let backup_border_size: c_float = g.style.ChildBorderSize;
    if !border {
        g.style.ChildBorderSize = 0.0;
    }
    let mut ret: bool = Begin(g, temp_window_name.as_str(), None);
    g.style.ChildBorderSize = backup_border_size;

    let mut child_window = &mut g.CurrentWindow;
    child_window.ChildId = id;
    child_window.AutoFitChildAxises = auto_fit_axises;

    // Set the cursor to handle case where the user called SetNextWindowPos()+BeginChild() manually.
    // While this is not really documented/defined, it seems that the expected thing to do.
    if child_window.BeginCount == 1 {
        parent_window.dc.cursor_pos = child_window.Pos;
    }

    // Process navigation-in immediately so NavInit can run on first frame
    if g.NavActivateId == id
        && flag_clear(flags, ImGuiWindowFlags_NavFlattened)
        && (child_window.DC.NavLayersActiveMask != 0 || child_window.DC.NavHasScroll)
    {
        FocusWindow(child_window.unwrap().borrow_mut());
        NavInitWindow(child_window.unwrap().borrow_mut(), false);
        SetActiveID(g, id + 1, child_window.unwrap().borrow_mut()); // Steal ActiveId with another arbitrary id so that key-press won't activate child item
        g.ActiveIdSource = ImGuiInputSource_Nav;
    }
    return ret;
}

// BeginChild: bool(str_id: *const c_char, const size_arg: &mut ImVec2, border: bool, ImGuiWindowFlags extra_flags)
pub unsafe fn BeginChild(
    str_id: String,
    size_arg: ImVec2,
    border: bool,
    extra_flags: ImGuiWindowFlags,
) -> bool {
    let mut window = g.current_window_mut().unwrap();
    return BeginChildEx(
        str_id,
        window.id_from_str(&str_id.clone()),
        size_arg,
        border,
        extra_flags,
    );
}

// BeginChild: bool(ImguiHandle id, const size_arg: &mut ImVec2, border: bool, ImGuiWindowFlags extra_flags)
pub unsafe fn BeginChild2(
    id: ImguiHandle,
    size_arg: ImVec2,
    border: bool,
    extra_flags: ImGuiWindowFlags,
) -> bool {
    // IM_ASSERT(id != 0);
    return BeginChildEx(String::from(""), id, size_arg, border, extra_flags);
}

// c_void EndChild()
pub unsafe fn EndChild() {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.current_window_mut().unwrap();

    // IM_ASSERT(g.WithinEndChild == false);
    // IM_ASSERT(window.Flags & ImGuiWindowFlags_ChildWindow);   // Mismatched BeginChild()/EndChild() calls

    g.WithinEndChild = true;
    if window.BeginCount > 1 {
        End(0);
    } else {
        let mut sz: ImVec2 = window.Size;
        if window.AutoFitChildAxises & (1 << ImGuiAxis_X) {
            // Arbitrary minimum zero-ish child size of 4.0 causes less trouble than a 0.0
            sz.x = ImMax(4.0, sz.x);
        }
        if window.AutoFitChildAxises & (1 << ImGuiAxis_Y) {
            sz.y = ImMax(4.0, sz.y);
        }
        End(0);

        let mut parent_window = g.current_window_mut().unwrap();
        let mut bb: ImRect =
            ImRect::from_vec2(&parent_window.dc.cursor_pos, parent_window.dc.cursor_pos + sz);
        ItemSize(g, &sz, 0.0);
        if (window.dc.NavLayersActiveMask != 0 || window.dc.NavHasScroll)
            && flag_clear(window.Flags, ImGuiWindowFlags_NavFlattened)
        {
            ItemAdd(g, &mut bb, window.ChildId, None, 0);
            RenderNavHighlight(, &bb, window.ChildId, 0);

            // When browsing a window that has no activable items (scroll only) we keep a highlight on the child (pass g.NavId to trick into always displaying)
            if window.dc.NavLayersActiveMask == 0 && window == g.NavWindow {
                RenderNavHighlight(,
                                   &ImRect::from_vec2(
                                       bb.min.clone() - ImVec2::from_floats(2.0, 2.0),
                                       bb.max.clone() + ImVec2::from_floats(2.0, 2.0),
                                   ),
                                   g.NavId,
                                   ImGuiNavHighlightFlags_TypeThin,
                );
            }
        } else {
            // Not navigable into
            ItemAdd(g, &mut bb, 0, None, 0);
        }
        if g.HoveredWindow == window {
            g.LastItemData.StatusFlags |= ImGuiItemStatusFlags_HoveredWindow;
        }
    }
    g.WithinEndChild = false;
    g.LogLinePosY = f32::MIN; // To enforce a carriage return
}

// Helper to create a child window / scrolling region that looks like a normal widget frame.
// BeginChildFrame: bool(ImguiHandle id, const size: &mut ImVec2, ImGuiWindowFlags extra_flags)
pub unsafe fn BeginChildFrame(
    id: ImguiHandle,
    size: ImVec2,
    extra_flags: ImGuiWindowFlagss,
) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let style = &mut g.style;
    PushStyleColor(ImGuiCol_ChildBg, style.Colors[ImGuiCol_FrameBg]);
    PushStyleVarFloat(ImGuiStyleVar_ChildRounding, style.FrameRounding);
    PushStyleVarFloat(ImGuiStyleVar_ChildBorderSize, style.FrameBorderSize);
    PushStyleVarVec2(ImGuiStyleVar_WindowPadding, &style.FramePadding);
    let mut ret: bool = BeginChild2(
        id,
        size,
        true,
        ImGuiWindowFlags_NoMove | ImGuiWindowFlags_AlwaysUseWindowPadding | extra_flags,
    );
    PopStyleVarInt(3);
    PopStyleColor(0);
    return ret;
}

// c_void EndChildFrame()
pub unsafe fn EndChildFrame() {
    EndChild();
}
