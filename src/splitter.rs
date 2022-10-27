use libc::c_float;
use std::ptr::null;
use crate::axis::{ImGuiAxis, ImGuiAxis_X, ImGuiAxis_Y};
use crate::{button_ops, GImGui};
use crate::button_flags::{ImGuiButtonFlags_AllowItemOverlap, ImGuiButtonFlags_FlattenChildren};
use crate::color::{IM_COL32_A_MASK, ImGuiCol_Separator, ImGuiCol_SeparatorActive, ImGuiCol_SeparatorHovered};
use crate::input_ops::SetMouseCursor;
use crate::item_flags::{ImGuiItemFlags, ImGuiItemFlags_NoNav, ImGuiItemFlags_NoNavDefaultFocus};
use crate::item_ops::{ItemAdd, MarkItemEdited};
use crate::item_status_flags::ImGuiItemStatusFlags_HoveredRect;
use crate::math_ops::ImMax;
use crate::mouse_cursor::{ImGuiMouseCursor_ResizeEW, ImGuiMouseCursor_ResizeNS};
use crate::rect::ImRect;
use crate::style_ops::GetColorU32;
use crate::type_defs::ImGuiID;
use crate::vec2::ImVec2;
use crate::window::ImGuiWindow;

// Using 'hover_visibility_delay' allows us to hide the highlight and mouse cursor for a short time, which can be convenient to reduce visual noise.
pub unsafe fn SplitterBehavior(bb: &mut ImRect, id: ImGuiID, axis: ImGuiAxis, size1: &mut c_float, size2: &mut c_float,min_size1: c_float,min_size2: c_float,hover_extend: c_float,hover_visibility_delay: c_float, bg_col: u32) -> bool
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut window: *mut ImGuiWindow = g.CurrentWindow;

    let mut item_flags_backup: ImGuiItemFlags =  g.CurrentItemFlags;
    g.CurrentItemFlags |= ImGuiItemFlags_NoNav | ImGuiItemFlags_NoNavDefaultFocus;
    let mut item_add: bool =  ItemAdd(bb, id, null(), 0);
    g.CurrentItemFlags = item_flags_backup;
    if !item_add { return  false; }

    let mut hovered = false; let mut held = false;
    let mut bb_interact: ImRect =  bb.clone();
    bb_interact.expand_from_vec(if axis == ImGuiAxis_Y { &mVec2::from_floats(0.0, hover_extend) } else { &ImVec2::from_floats(hover_extend, 0.0) });
    button_ops::ButtonBehavior(&bb_interact, id, &mut hovered, &mut held, ImGuiButtonFlags_FlattenChildren | ImGuiButtonFlags_AllowItemOverlap);
    if (hovered) {
        g.LastItemData.StatusFlags |= ImGuiItemStatusFlags_HoveredRect;
    } // for IsItemHovered(), because bb_interact is larger than bb
    if (g.ActiveId != id) {
        SetItemAllowOverlap();
    }

    if held || (hovered && g.HoveredIdPreviousFrame == id && g.HoveredIdTimer >= hover_visibility_delay) {
        SetMouseCursor(if axis == ImGuiAxis_Y { ImGuiMouseCursor_ResizeNS } else { ImGuiMouseCursor_ResizeEW });
    }

    let mut bb_render: ImRect =  bb.clone();
    if (held)
    {
        let mouse_delta_2d: ImVec2 = g.IO.MousePos - g.ActiveIdClickOffset - bb_interact.Min;
        let mut mouse_delta: c_float =  if (axis == ImGuiAxis_Y) { mouse_delta_2d.y } else { mouse_delta_2d.x };

        // Minimum pane size
        let size_1_maximum_delta: c_float =  ImMax(0.0, *size1 - min_size1);
        let size_2_maximum_delta: c_float =  ImMax(0.0, *size2 - min_size2);
        if (mouse_delta < -size_1_maximum_delta) {
            mouse_delta = -size_1_maximum_delta;
        }
        if mouse_delta > size_2_maximum_delta {
            mouse_delta = size_2_maximum_delta;}

        // Apply resize
        if (mouse_delta != 0.0)
        {
            if (mouse_delta < 0.0) {}
                // IM_ASSERT(*size1 + mouse_delta >= min_size1);
            if (mouse_delta > 0.0) {}
                // IM_ASSERT(*size2 - mouse_delta >= min_size2);
            *size1 += mouse_delta;
            *size2 -= mouse_delta;
            bb_render.Translate(if axis == ImGuiAxis_X { &ImVec2::from_floats(mouse_delta, 0.0) } else { &ImVec2::from_floats(0.0, mouse_delta) });
            MarkItemEdited(id);
        }
    }

    // Render at new position
    if bg_col & IM_COL32_A_MASK {
        window.DrawList.AddRectFilled(&bb_render.Min, &bb_render.Max, bg_col, 0.0, 0);
    }
    col: u32 = GetColorU32(if held { ImGuiCol_SeparatorActive } else {
        if hovered && g.HoveredIdTimer >= hover_visibility_delay {
            ImGuiCol_SeparatorHovered
        } else { ImGuiCol_Separator }
    }, 0.0);
    window.DrawList.AddRectFilled(&bb_render.Min, &bb_render.Max, col, 0.0, 0);

    return held;
}
