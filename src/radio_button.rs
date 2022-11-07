use libc::{c_float, c_int};
use std::ptr::null;
use crate::frame_ops::GetFrameHeight;
use crate::{button_ops, GImGui};
use crate::color::{ImGuiCol_Border, ImGuiCol_BorderShadow, ImGuiCol_CheckMark, ImGuiCol_FrameBg, ImGuiCol_FrameBgActive, ImGuiCol_FrameBgHovered};
use crate::item_ops::{ItemAdd, ItemSize, MarkItemEdited};
use crate::math_ops::ImMax;
use crate::rect::ImRect;
use crate::render_ops::{RenderNavHighlight, RenderText};
use crate::style_ops::GetColorU32;
use crate::text_ops::CalcTextSize;
use crate::type_defs::ImguiHandle;
use crate::vec2::ImVec2;
use crate::window::ImguiWindow;
use crate::window::ops::GetCurrentWindow;

pub unsafe fn RadioButton(label: String, active: bool) -> bool
{
    let mut window = g.current_window_mut().unwrap();
    if window.skip_items { return  false; }

    let g = GImGui; // ImGuiContext& g = *GImGui;
    let setyle = &mut g.style;
    let mut id: ImguiHandle =  window.id_from_str(label, );
    let label_size: ImVec2 = CalcTextSize(, label, true, 0.0);

    let square_sz: c_float =  GetFrameHeight();
    let pos: ImVec2 = window.dc.cursor_pos;
    let mut check_bb: ImRect = ImRect::new(pos, pos + ImVec2::from_floats(square_sz, square_sz));
    let mut total_bb: ImRect = ImRect::new(pos, pos + ImVec2::from_floats(square_sz + (if label_size.x > 0.0 { style.ItemInnerSpacing.x + label_size.x } else { 0.0 }), label_size.y + style.FramePadding.y * 2.0));
    ItemSize(g, &total_bb.GetSize(), style.FramePadding.y);
    if !ItemAdd(g, &mut total_bb, id, None, 0) { return  false; }

    let mut center: ImVec2 = check_bb.GetCenter();
    center.x = IM_ROUND(center.x);
    center.y = IM_ROUND(center.y);
    let radius: c_float =  (square_sz - 1.0) * 0.5;

    let mut hovered = false;
    let mut held = false;
    let mut pressed: bool =  button_ops::ButtonBehavior(g, &total_bb, id, &mut hovered, &mut held, 0);
    if pressed {
        MarkItemEdited(g, id); }

    RenderNavHighlight(, &total_bb, id, 0);
    window.DrawList.AddCircleFilled(&center, radius, GetColorU32(if (held && hovered) { ImGuiCol_FrameBgActive } else { if hovered { ImGuiCol_FrameBgHovered } else { ImGuiCol_FrameBg } }, 0.0), 16);
    if (active)
    {
        let pad: c_float =  ImMax(1.0, IM_FLOOR(square_sz / 6.0));
        window.DrawList.AddCircleFilled(&center, radius - pad, GetColorU32(ImGuiCol_CheckMark, 0.0), 16);
    }

    if style.FrameBorderSize > 0.0
    {
        window.DrawList.AddCircle(center + ImVec2::from_ints(1, 1), radius, GetColorU32(ImGuiCol_BorderShadow, 0.0), 16, style.FrameBorderSize);
        window.DrawList.AddCircle(&center, radius, GetColorU32(ImGuiCol_Border, 0.0), 16, style.FrameBorderSize);
    }

    let label_pos: ImVec2 = ImVec2::from_floats(check_bb.max.x + style.ItemInnerSpacing.x, check_bb.min.y + style.FramePadding.y);
    if g.LogEnabled {
        // LogRenderedText(&label_pos, active? "(x)": "( )");
    }
    if label_size.x > 0.0 {
        RenderText(label_pos, label, false, g);
    }

    IMGUI_TEST_ENGINE_ITEM_INFO(id, label, g.last_item_data.StatusFlags);
    return pressed;
}

// FIXME: This would work nicely if it was a public template, e.g. 'template<T> RadioButton(const char* label, T* v, T v_button)', but I'm not sure how we would expose it..
pub unsafe fn RadioButton2(label: String, v: *mut c_int, v_button: c_int) -> bool
{
    let pressed: bool = RadioButton(label, *v == v_button);
    if pressed {
        *v = v_button;
    }
    return pressed;
}
