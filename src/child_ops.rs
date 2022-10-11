use std::io::SeekFrom::End;
use std::ptr::{null, null_mut};
use libc::{c_char, c_float, c_int};
use crate::axis::{ImGuiAxis_X, ImGuiAxis_Y};
use crate::color::{ImGuiCol_ChildBg, ImGuiCol_FrameBg};
use crate::condition::ImGuiCond_None;
use crate::content_ops::GetContentRegionAvail;
use crate::GImGui;
use crate::id_ops::SetActiveID;
use crate::input_source::ImGuiInputSource_Nav;
use crate::item_ops::{ItemAdd, ItemSize};
use crate::item_status_flags::ImGuiItemStatusFlags_HoveredWindow;
use crate::math_ops::ImMax;
use crate::nav_highlight_flags::ImGuiNavHighlightFlags_TypeThin;
use crate::rect::ImRect;
use crate::render_ops::RenderNavHighlight;
use crate::string_ops::ImFormatStringToTempBuffer;
use crate::style_ops::{PopStyleColor, PushStyleColor};
use crate::style_var_ops::{PopStyleVar, PopStyleVarInt, PushStyleVar, PushStyleVarFloat, PushStyleVarVec2};
use crate::type_defs::ImGuiID;
use crate::utils::flag_clear;
use crate::vec2::ImVec2;
use crate::window::ImGuiWindow;
use crate::window_flags::{ImGuiWindowFlags, ImGuiWindowFlags_AlwaysUseWindowPadding, ImGuiWindowFlags_ChildWindow, ImGuiWindowFlags_NavFlattened, ImGuiWindowFlags_NoDocking, ImGuiWindowFlags_NoMove, ImGuiWindowFlags_NoResize, ImGuiWindowFlags_NoSavedSettings, ImGuiWindowFlags_NoTitleBar};
use crate::window_ops::SetNextWindowSize;

// BeginChildEx: bool(name: *const c_char, ImGuiID id, const ImVec2& size_arg, border: bool, ImGuiWindowFlags flags)
pub unsafe fn BeginChildEx(name: *const c_char, id: ImGuiID, size_arg: &ImVec2, border: bool, mut flags: ImGuiWindowFlags) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut parent_window: *mut ImGuiWindow = g.CurrentWindow;

    flags |= ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoResize | ImGuiWindowFlags_NoSavedSettings | ImGuiWindowFlags_ChildWindow | ImGuiWindowFlags_NoDocking;
    flags |= (parent_window.Flags & ImGuiWindowFlags_NoMove);  // Inherit the NoMove flag

    // Size
    let content_avail: ImVec2 = GetContentRegionAvail();
    let mut size: ImVec2 = ImFloor(size_arg);
    let auto_fit_axises: c_int = (if size.x == 0f32 { (1 << ImGuiAxis_X) } else { 0x00 }) | (if size.y == 0f32 { (1 << ImGuiAxis_Y) } else { 0x00 });
    if size.x <= 0f32 {
        size.x = ImMax(content_avail.x + size.x, 4.00f32);
    }// Arbitrary minimum child size (0f32 causing too much issues)
    if size.y <= 0f32 {
        size.y = ImMax(content_avail.y + size.y, 4.00f32);
    }
    SetNextWindowSize(&size, ImGuiCond_None);

    // Build up name. If you need to append to a same child from multiple location in the ID stack, use BeginChild(ImGuiID id) with a stable value.
    let temp_window_name: *const c_char = null_mut();
    if name {
        // TODO:
        // ImFormatStringToTempBuffer(&mut temp_window_name, null_mut(), "%s/%s_%08X", parent_window.Name, name, id);
    } else {
        // TODO:
        // ImFormatStringToTempBuffer(&mut temp_window_name, null_mut(), "%s/%08X", parent_window.Name, id);
    }

    let backup_border_size: c_float = g.Style.ChildBorderSize;
    if !border {
        g.Style.ChildBorderSize = 0f32;
    }
    let mut ret: bool = Begin(temp_window_name, null_mut(), flags);
    g.Style.ChildBorderSize = backup_border_size;

    let mut child_window: *mut ImGuiWindow = g.CurrentWindow;
    child_window.ChildId = id;
    child_window.AutoFitChildAxises = auto_fit_axises;

    // Set the cursor to handle case where the user called SetNextWindowPos()+BeginChild() manually.
    // While this is not really documented/defined, it seems that the expected thing to do.
    if child_window.BeginCount == 1 {
        parent_window.DC.CursorPos = child_window.Pos;
    }

    // Process navigation-in immediately so NavInit can run on first frame
    if g.NavActivateId == id && flag_clear(flags, ImGuiWindowFlags_NavFlattened) && (child_window.DC.NavLayersActiveMask != 0 || child_window.DC.NavHasScroll) {
        FocusWindow(child_window);
        NavInitWindow(child_window, false);
        SetActiveID(id + 1, child_window); // Steal ActiveId with another arbitrary id so that key-press won't activate child item
        g.ActiveIdSource = ImGuiInputSource_Nav;
    }
    return ret;
}

// BeginChild: bool(str_id: *const c_char, const ImVec2& size_arg, border: bool, ImGuiWindowFlags extra_flags)
pub unsafe fn BeginChild(str_id: *const c_char, size_arg: &ImVec2, border: bool, extra_flags: ImGuiWindowFlags) -> bool {
    let mut window: *mut ImGuiWindow = GetCurrentWindow();
    return BeginChildEx(str_id, window.GetID(str_id, null()), size_arg, border, extra_flags);
}

// BeginChild: bool(ImGuiID id, const ImVec2& size_arg, border: bool, ImGuiWindowFlags extra_flags)
pub unsafe fn BeginChild2(id: ImGuiID, size_arg: &ImVec2, border: bool, extra_flags: ImGuiWindowFlags) -> bool {
    // IM_ASSERT(id != 0);
    return BeginChildEx(null_mut(), id, size_arg, border, extra_flags);
}

// c_void EndChild()
pub unsafe fn EndChild() {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.CurrentWindow;

    // IM_ASSERT(g.WithinEndChild == false);
    // IM_ASSERT(window.Flags & ImGuiWindowFlags_ChildWindow);   // Mismatched BeginChild()/EndChild() calls

    g.WithinEndChild = true;
    if window.BeginCount > 1 {
        End(0);
    } else {
        let mut sz: ImVec2 = window.Size;
        if window.AutoFitChildAxises & (1 << ImGuiAxis_X) {// Arbitrary minimum zero-ish child size of 4.0f32 causes less trouble than a 0f32
            sz.x = ImMax(4.0f32, sz.x);
        }
        if window.AutoFitChildAxises & (1 << ImGuiAxis_Y) {
            sz.y = ImMax(4.0f32, sz.y);
        }
        End(0);

        let mut parent_window: *mut ImGuiWindow = g.CurrentWindow;
        let mut bb: ImRect = ImRect::from_vec2(&parent_window.DC.CursorPos, parent_window.DC.CursorPos + sz);
        ItemSize(&sz, 0.0);
        if (window.DC.NavLayersActiveMask != 0 || window.DC.NavHasScroll) && flag_clear(window.Flags, ImGuiWindowFlags_NavFlattened) {
            ItemAdd(&mut bb, window.ChildId, null(), 0);
            RenderNavHighlight(&bb, window.ChildId, 0);

            // When browsing a window that has no activable items (scroll only) we keep a highlight on the child (pass g.NavId to trick into always displaying)
            if window.DC.NavLayersActiveMask == 0 && window == g.NavWindow {
                RenderNavHighlight(&ImRect::from_vec2(bb.Min.clone() - ImVec2::new(2.0, 2.0), bb.Max.clone() + ImVec2::new(2.0, 2.0)), g.NavId, ImGuiNavHighlightFlags_TypeThin);
            }
        } else {
            // Not navigable into
            ItemAdd(&mut bb, 0, null(), 0);
        }
        if g.HoveredWindow == window {
            g.LastItemData.StatusFlags |= ImGuiItemStatusFlags_HoveredWindow;
        }
    }
    g.WithinEndChild = false;
    g.LogLinePosY = f32::MIN; // To enforce a carriage return
}

// Helper to create a child window / scrolling region that looks like a normal widget frame.
// BeginChildFrame: bool(ImGuiID id, const ImVec2& size, ImGuiWindowFlags extra_flags)
pub unsafe fn BeginChildFrame(id: ImGuiID, size: &ImVec2, extra_flags: ImGuiWindowFlagss) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let style = &mut g.Style;
    PushStyleColor(ImGuiCol_ChildBg, style.Colors[ImGuiCol_FrameBg]);
    PushStyleVarFloat(ImGuiStyleVar_ChildRounding, style.FrameRounding);
    PushStyleVarFloat(ImGuiStyleVar_ChildBorderSize, style.FrameBorderSize);
    PushStyleVarVec2(ImGuiStyleVar_WindowPadding, &style.FramePadding);
    let mut ret: bool = BeginChild2(id, size, true, ImGuiWindowFlags_NoMove | ImGuiWindowFlags_AlwaysUseWindowPadding | extra_flags);
    PopStyleVarInt(3);
    PopStyleColor(0);
    return ret;
}

// c_void EndChildFrame()
pub unsafe fn EndChildFrame() {
    EndChild();
}
