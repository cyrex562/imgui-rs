use crate::color::{
    ImGuiCol_CheckMark, ImGuiCol_FrameBg, ImGuiCol_FrameBgActive, ImGuiCol_FrameBgHovered,
};
use crate::frame_ops::GetFrameHeight;
use crate::item_flags::{ImGuiItemFlags, ImGuiItemFlags_MixedValue};
use crate::item_ops::{ItemAdd, ItemSize, MarkItemEdited};
use crate::item_status_flags::{ImGuiItemStatusFlags_Checkable, ImGuiItemStatusFlags_Checked};
use crate::math_ops::ImMax;
use crate::rect::ImRect;
use crate::render_ops::{RenderCheckMark, RenderFrame, RenderNavHighlight, RenderText};
use crate::style_ops::GetColorU32;
use crate::text_ops::CalcTextSize;
use crate::type_defs::ImguiHandle;
use crate::vec2::ImVec2;
use crate::window::ops::GetCurrentWindow;
use crate::window::ImguiWindow;
use crate::{button_ops, GImGui};
use libc::{c_float, c_int, c_uint};
use std::ptr::null;

pub unsafe fn Checkbox(label: &String, v: &mut bool) -> bool {
    let mut window = g.current_window_mut().unwrap();
    if window.skip_items {
        return false;
    }

    let g = GImGui; // ImGuiContext& g = *GImGui;
    let setyle = &mut g.style;
    let mut id: ImguiHandle = window.id_from_str(label, );
    let label_size: ImVec2 = CalcTextSize(, label, true, 0.0);

    let square_sz: c_float = GetFrameHeight();
    let pos: ImVec2 = window.dc.cursor_pos;
    let mut total_bb: ImRect = ImRect::new(
        pos,
        pos + ImVec2::from_floats(
            square_sz
                + (if label_size.x > 0.0 {
                    style.ItemInnerSpacing.x + label_size.x
                } else {
                    0.0
                }),
            label_size.y + style.FramePadding.y * 2.0,
        ),
    );
    ItemSize(g, &total_bb.GetSize(), style.FramePadding.y);
    if !ItemAdd(g, &mut total_bb, id, None, 0) {
        IMGUI_TEST_ENGINE_ITEM_INFO(
            id,
            label,
            g.LastItemData.StatusFlags
                | ImGuiItemStatusFlags_Checkable
                | (if *v { ImGuiItemStatusFlags_Checked } else { 0 }),
        );
        return false;
    }

    // hovered: bool, held;
    let mut hovered = false;
    let mut held = false;
    let mut pressed: bool = button_ops::ButtonBehavior(g, &total_bb, id, &mut hovered, &mut held, 0);
    if pressed {
        *v = !(*v);
        MarkItemEdited(g, id);
    }

    let mut check_bb: ImRect = ImRect::new(pos, pos + ImVec2::from_floats(square_sz, square_sz));
    RenderNavHighlight(, &total_bb, id, 0);
    RenderFrame(
        check_bb.min,
        check_bb.max,
        GetColorU32(
            if held && hovered {
                ImGuiCol_FrameBgActive
            } else {
                if hovered {
                    ImGuiCol_FrameBgHovered
                } else {
                    ImGuiCol_FrameBg
                }
            },
            0.0,
        ),
        true,
        style.FrameRounding,
    );
    check_col: u32 = GetColorU32(ImGuiCol_CheckMark, 0.0);
    let mut mixed_value: bool = (g.LastItemData.InFlags & ImGuiItemFlags_MixedValue) != 0;
    if mixed_value {
        // Undocumented tristate/mixed/indeterminate checkbox (#2644)
        // This may seem awkwardly designed because the aim is to make ImGuiItemFlags_MixedValue supported by all widgets (not just checkbox)
        let pad = ImVec2::from_floats(
            ImMax(1.0, IM_FLOOR(square_sz / 3.60)),
            ImMax(1.0, IM_FLOOR(square_sz / 3.60)),
        );
        window.DrawList.AddRectFilled(
            check_bb.min + pad,
            check_bb.max - pad,
            check_col,
            style.FrameRounding,
            0,
        );
    } else if (*v) {
        let pad: c_float = ImMax(1.0, IM_FLOOR(square_sz / 6.0));
        RenderCheckMark(
            &mut window.DrawList,
            check_bb.min + ImVec2::from_floats(pad, pad),
            check_col,
            square_sz - pad * 2.0,
        );
    }

    let label_pos: ImVec2 = ImVec2::from_floats(
        check_bb.max.x + style.ItemInnerSpacing.x,
        check_bb.min.y + style.FramePadding.y,
    );
    if g.LogEnabled {
        // LogRenderedText(&label_pos, mixed_value? "[~]": * v? "[x]": "[ ]");
    }
    if label_size.x > 0.0 {
        RenderText(label_pos, label, false, g);
    }

    IMGUI_TEST_ENGINE_ITEM_INFO(
        id,
        label,
        g.LastItemData.StatusFlags
            | ImGuiItemStatusFlags_Checkable
            | (if *v { ImGuiItemStatusFlags_Checked } else { 0 }),
    );
    return pressed;
}

// template<typename T>
pub unsafe fn CheckboxFlagsT<T>(label: &String, flags: &mut T, flags_value: &T) -> bool {
    // let mut all_on: bool =  (*flags.clone() & flags_value) == flags_value;
    let mut all_on = (flags[0] & flags_value) == flags_value;
    // let mut any_on: bool =  (*flags.clone() & flags_value) != 0;
    let mut any_on = (flags[0] & flags_value.clone()) != 0;
    pressed: bool;
    if !all_on && any_on {
        let g = GImGui; // ImGuiContext& g = *GImGui;
        let mut backup_item_flags: ImGuiItemFlags = g.CurrentItemFlags;
        g.CurrentItemFlags |= ImGuiItemFlags_MixedValue;
        pressed = Checkbox(label, &mut all_on);
        g.CurrentItemFlags = backup_item_flags;
    } else {
        pressed = Checkbox(label, &mut all_on);
    }
    if pressed {
        if (all_on) {
            *flags |= flags_value.clone();
        } else {
            *flags &= !flags_value.clone();
        }
    }
    return pressed;
}

pub unsafe fn CheckboxFlags(label: &String, flags: &mut [c_int], flags_value: &[c_int]) -> bool {
    return CheckboxFlagsT(label, flags, flags_value);
}

pub unsafe fn CheckboxFlags2(label: &String, flags: &mut [c_uint], flags_value: &[c_uint]) -> bool {
    return CheckboxFlagsT(label, flags, flags_value);
}

pub unsafe fn CheckboxFlags3(label: &String, flags: &mut [i64], flags_value: &[i64]) -> bool {
    return CheckboxFlagsT(label, flags, flags_value);
}

pub unsafe fn CheckboxFlags4(label: &String, flags: &mut [u64], flags_value: &[u64]) -> bool {
    return CheckboxFlagsT(label, flags, flags_value);
}
