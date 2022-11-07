use crate::color::{ImGuiCol_Button, ImGuiCol_ButtonActive, ImGuiCol_ButtonHovered};
use crate::core::id_ops::pop_win_id_from_stack;
use crate::item::item_ops::{ItemAdd, ItemSize};
use crate::core::math_ops::{ImClamp, ImMin};
use crate::rect::ImRect;
use crate::drawing::render_ops::{RenderFrame, RenderNavHighlight};
use crate::style_ops::{GetColorU32, GetColorU32FromImVec4};
use crate::style_var::ImGuiStyleVar_FramePadding;
use crate::core::type_defs::{ImguiHandle, ImTextureID};
use crate::core::vec2::ImVec2;
use crate::core::vec4::ImVec4;
use crate::window::ops::GetCurrentWindow;
use crate::window::ImguiWindow;
use crate::{button_ops, GImGui};
use core::ptr::null;
use imgui_rs::button_ops;
use imgui_rs::color::{ImGuiCol_Button, ImGuiCol_ButtonActive, ImGuiCol_ButtonHovered};
use imgui_rs::id_ops::PopID;
use imgui_rs::imgui::GImGui;
use imgui_rs::item_ops::{ItemAdd, ItemSize};
use imgui_rs::math_ops::{ImClamp, ImMin};
use imgui_rs::rect::ImRect;
use imgui_rs::render_ops::{RenderFrame, RenderNavHighlight};
use imgui_rs::style_ops::{GetColorU32, GetColorU32FromImVec4};
use imgui_rs::style_var::ImGuiStyleVar_FramePadding;
use imgui_rs::type_defs::{ImguiHandle, ImTextureID};
use imgui_rs::vec2::ImVec2;
use imgui_rs::vec4::ImVec4;
use imgui_rs::window::ops::GetCurrentWindow;
use imgui_rs::window::ImGuiWindow;
use libc::windows::{c_float, c_int};

pub unsafe fn Image(
    user_texture_id: ImTextureID,
    size: &ImVec2,
    uv0: &ImVec2,
    uv1: &ImVec2,
    tint_col: &ImVec4,
    border_col: &ImVec4,
) {
    let mut window = g.current_window_mut().unwrap();
    if window.skip_items {
        return;
    }

    let mut bb: ImRect = ImRect::new(window.dc.cursor_pos, window.dc.cursor_pos + size);
    if border_col.w > 0.0 {
        bb.max += ImVec2::from_floats(2.0, 2.0);
    }
    ItemSize(g, &bb.GetSize(), 0.0);
    if !ItemAdd(g, &mut bb, 0, None, 0) {
        return;
    }

    if (border_col.w > 0.0) {
        window
            .DrawList
            .AddRect(&bb.min, &bb.max, GetColorU32FromImVec4(border_col), 0.0);
        window.DrawList.AddImage(
            user_texture_id,
            bb.min + ImVec2::from_ints(1, 1),
            bb.max - ImVec2::from_floats(1.0, 1.0),
            uv0,
            uv1,
            GetColorU32FromImVec4(tint_col),
        );
    } else {
        window.DrawList.AddImage(
            user_texture_id,
            &bb.min,
            &bb.max,
            uv0,
            uv1,
            GetColorU32FromImVec4(tint_col),
        );
    }
}

// ImageButton() is flawed as 'id' is always derived from 'texture_id' (see #2464 #1390)
// We provide this internal helper to write your own variant while we figure out how to redesign the public ImageButton() API.
pub unsafe fn ImageButtonEx(
    id: ImguiHandle,
    texture_id: ImTextureID,
    size: &ImVec2,
    uv0: &ImVec2,
    uv1: &ImVec2,
    bg_col: &ImVec4,
    tint_col: &ImVec4,
) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.current_window_mut().unwrap();
    if window.skip_items {
        return false;
    }

    let padding: ImVec2 = g.style.FramePadding;
    let mut bb: ImRect = ImRect::new(
        window.dc.cursor_pos,
        window.dc.cursor_pos + size + padding * 2.0,
    );
    ItemSize(g, &bb.GetSize(), 0.0);
    if !ItemAdd(g, &mut bb, id, None, 0) {
        return false;
    }

    // hovered: bool, held;
    let mut hovered = false;
    let mut held = false;
    let mut pressed: bool = button_ops::ButtonBehavior(g, &bb, id, &mut hovered, &mut held, 0);

    // Render
    col: u32 = GetColorU32(
        if (held && hovered) {
            ImGuiCol_ButtonActive
        } else {
            if hovered {
                ImGuiCol_ButtonHovered
            } else {
                ImGuiCol_Button
            }
        },
        0.0,
    );
    RenderNavHighlight(, &bb, id, 0);
    RenderFrame(
        bb.min,
        bb.max,
        col,
        true,
        ImClamp(
            ImMin(padding.x as c_int, padding.y as c_int),
            0.0,
            g.style.FrameRounding,
        ),
    );
    if (bg_col.w > 0.0) {
        window.DrawList.AddRectFilled(
            bb.min + padding,
            bb.max - padding,
            GetColorU32FromImVec4(bg_col),
            0.0,
            0,
        );
    }
    window.DrawList.AddImage(
        texture_id,
        bb.min + padding,
        bb.max - padding,
        uv0,
        uv1,
        GetColorU32FromImVec4(tint_col),
    );

    return pressed;
}

pub unsafe fn ImageButton(
    str_id: &str,
    user_texture_id: ImTextureID,
    size: &ImVec2,
    uv0: &ImVec2,
    uv1: &ImVec2,
    bg_col: &ImVec4,
    tint_col: &ImVec4,
) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.current_window_mut().unwrap();
    if window.skip_items {
        return false;
    }

    return ImageButtonEx(
        window.GetID(str_id),
        user_texture_id,
        size,
        uv0,
        uv1,
        bg_col,
        tint_col,
    );
}

// #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
// Legacy API obsoleted in 1.89. Two differences with new ImageButton()
// - new ImageButton() requires an explicit 'const char* str_id'    Old ImageButton() used opaque imTextureId (created issue with: multiple buttons with same image, transient texture id values, opaque computation of ID)
// - new ImageButton() always use style.FramePadding                Old ImageButton() had an override argument.
// If you need to change padding with new ImageButton() you can use PushStyleVar(ImGuiStyleVar_FramePadding, value), consistent with other Button functions.
pub unsafe fn ImageButton2(
    user_texture_id: ImTextureID,
    size: &ImVec2,
    uv0: &ImVec2,
    uv1: &ImVec2,
    frame_padding: c_int,
    bg_col: &ImVec4,
    tint_col: &ImVec4,
) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window = g.current_window_mut().unwrap();
    if window.skip_items {
        return false;
    }

    // Default to using texture ID as ID. User can still push string/integer prefixes.
    PushID(user_texture_id);
    let mut id: ImguiHandle = window.GetID("#image");
    pop_win_id_from_stack(g);

    if frame_padding >= 0 {
        PushStyleVar(
            ImGuiStyleVar_FramePadding,
            ImVec2::from_floats(frame_padding as c_float, frame_padding as c_float),
        );
    }
    let mut ret: bool = ImageButtonEx(id, user_texture_id, size, uv0, uv1, bg_col, tint_col);
    if frame_padding >= 0 {
        PopStyleVar()();
    }
    return ret;
}
