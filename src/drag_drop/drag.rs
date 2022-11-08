use crate::activate_flags::IM_GUI_ACTIVATE_FLAGS_PREFER_INPUT;
use crate::core::axis::{ImGuiAxis, IM_GUI_AXIS_X, IM_GUI_AXIS_Y};
use crate::color::{ImGuiCol_FrameBg, ImGuiCol_FrameBgActive, ImGuiCol_FrameBgHovered};
use crate::data_type::{
    ImGuiDataType, IM_GUI_DATA_TYPE_DOUBLE, IM_GUI_DATA_TYPE_FLOAT, IM_GUI_DATA_TYPE_S32,
};
use crate::data_type_info::GDATA_TYPE_INFO;
use crate::data_type_ops::DataTypeCompare;
use crate::direction::{ImGuiDir_Left, ImGuiDir_Right};
use crate::group_ops::{BeginGroup, EndGroup};
use crate::id_ops::{push_int_id, push_str_id, ClearActiveID, pop_win_id_from_stack, SetActiveID};
use crate::input_ops::{IsKeyDown, IsMouseDragPastThreshold, IsMousePosValid};
use crate::input_source::{ImGuiInputSource_Gamepad, ImGuiInputSource_Mouse, ImGuiInputSource_Nav};
use crate::item_flags::{ImGuiItemFlags_Inputable, ImGuiItemFlags_ReadOnly};
use crate::item_ops::{
    CalcItemWidth, ItemAdd, ItemHoverable, ItemSize, MarkItemEdited, PopItemWidth,
    PushMultiItemsWidths,
};
use crate::item_status_flags::ImGuiItemStatusFlags_FocusedByTabbing;
use crate::key::{
    ImGuiKey_NavGamepadTweakFast, ImGuiKey_NavGamepadTweakSlow, ImGuiKey_NavKeyboardTweakFast,
    ImGuiKey_NavKeyboardTweakSlow,
};
use crate::layout_ops::same_line;
use crate::math_ops::{ImMax, ImMin};
use crate::nav_ops::{GetNavTweakPressedAmount, SetFocusID};
use crate::rect::ImRect;
use crate::render_ops::{
    FindRenderedTextEnd, RenderFrame, RenderNavHighlight, RenderText, RenderTextClipped,
};
use crate::slider_flags::{
    ImGuiSliderFlags, ImGuiSliderFlags_AlwaysClamp, ImGuiSliderFlags_Logarithmic,
    ImGuiSliderFlags_NoInput, ImGuiSliderFlags_NoRoundToFormat, ImGuiSliderFlags_ReadOnly,
    ImGuiSliderFlags_Vertical,
};
use crate::slider_ops::ScaleRatioFromValueT;
use crate::style_ops::GetColorU32;
use crate::text_flags::ImGuiTextFlags_None;
use crate::text_ops::CalcTextSize;
use crate::type_defs::ImguiHandle;
use crate::utils::{flag_clear, flag_set};
use crate::vec2::ImVec2;
use crate::window::focus::FocusWindow;
use crate::window::ops::GetCurrentWindow;
use crate::window::ImguiWindow;
use crate::{data_type_ops, input_num_ops, slider_ops, text_ops, widgets, GImGui};
use libc::{c_char, c_float, c_int, size_t, INT_MAX, INT_MIN};
use std::ptr::{null, null_mut};

// Widgets
static DRAGDROP_HOLD_TO_OPEN_TIMER: c_float = 0.70f32;

// This is called by DragBehavior() when the widget is active (held by mouse or being manipulated with Nav controls)
// template<typename TYPE, typename SIGNEDTYPE, typename FLOATTYPE>
pub unsafe fn DragBehaviorT<T, U>(
    data_type: ImGuiDataType,
    v: &mut c_float,
    mut v_speed: c_float,
    v_min: c_float,
    v_max: c_float,
    format: &str,
    flags: ImGuiSliderFlags,
) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    const axis: ImGuiAxis = if flag_set(flags, ImGuiSliderFlags_Vertical) {
            IM_GUI_AXIS_Y
    } else {
            IM_GUI_AXIS_X
    };
    let is_clamped: bool = (v_min < v_max);
    let is_logarithmic: bool = flag_set(flags, ImGuiSliderFlags_Logarithmic);
    let is_floating_point: bool =
        (data_type == IM_GUI_DATA_TYPE_FLOAT) || (data_type == IM_GUI_DATA_TYPE_DOUBLE);

    // Default tweak speed
    if v_speed == 0.0 && is_clamped && (v_max - v_min < f32::MAX) {
        v_speed = ((v_max - v_min) * g.DragSpeedDefaultRatio);
    }

    // Inputs accumulates into g.DragCurrentAccum, which is flushed into the current value as soon as it makes a difference with our precision settings
    let mut adjust_delta: c_float = 0.0;
    if g.ActiveIdSource == ImGuiInputSource_Mouse
        && IsMousePosValid(null())
        && IsMouseDragPastThreshold(0, g.IO.MouseDragThreshold * DRAG_MOUSE_THRESHOLD_FACTOR)
    {
        adjust_delta = g.IO.MouseDelta[axis];
        if g.IO.KeyAlt {
            adjust_delta *= 1.0 / 100;
        }
        if g.IO.KeyShift {
            adjust_delta *= 10.0;
        }
    } else if g.ActiveIdSource == ImGuiInputSource_Nav {
        let decimal_precision: usize = if is_floating_point {
            ImParseFormatPrecision(format, 3)
        } else {
            0
        };
        let tweak_slow: bool = IsKeyDown(if g.NavInputSource == ImGuiInputSource_Gamepad {
            ImGuiKey_NavGamepadTweakSlow
        } else {
            ImGuiKey_NavKeyboardTweakSlow
        });
        let tweak_fast: bool = IsKeyDown(if g.NavInputSource == ImGuiInputSource_Gamepad {
            ImGuiKey_NavGamepadTweakFast
        } else {
            ImGuiKey_NavKeyboardTweakFast
        });
        let tweak_factor: c_float = if tweak_slow {
            1.0 / 1.0
        } else {
            if tweak_fast {
                10.0
            } else {
                1.0
            }
        };
        adjust_delta = GetNavTweakPressedAmount(axis) * tweak_factor;
        v_speed = ImMax(
            v_speed,
            data_type_ops::GetMinimumStepAtDecimalPrecision(decimal_precision),
        );
    }
    adjust_delta *= v_speed;

    // For vertical drag we currently assume that Up=higher value (like we do with vertical sliders). This may become a parameter.
    if axis == IM_GUI_AXIS_Y {
        adjust_delta = -adjust_delta;
    }

    // For logarithmic use our range is effectively 0..1 so scale the delta into that range
    if is_logarithmic && (v_max - v_min < f32::MAX) && ((v_max - v_min) > 0.00010f32) {
        // Epsilon to avoid /0
        adjust_delta /= (v_max - v_min);
    }

    // Clear current value on activation
    // Avoid altering values and clamping when we are _already_ past the limits and heading in the same direction, so e.g. if range is 0..255, current value is 300 and we are pushing to the right side, keep the 300.
    let mut is_just_activated: bool = g.ActiveIdIsJustActivated;
    let mut is_already_past_limits_and_pushing_outward: bool =
        is_clamped && ((*v >= v_max && adjust_delta > 0.0) || (*v <= v_min && adjust_delta < 0.0));
    if is_just_activated || is_already_past_limits_and_pushing_outward {
        g.DragCurrentAccum = 0.0;
        g.DragCurrentAccumDirty = false;
    } else if adjust_delta != 0.0 {
        g.DragCurrentAccum += adjust_delta;
        g.DragCurrentAccumDirty = true;
    }

    if !g.DragCurrentAccumDirty {
        return false;
    }

    let mut v_cur: T = (*v).clone();
    let mut v_old_ref_for_accum_remainder = 0.0;

    let mut logarithmic_zero_epsilon: c_float = 0.0; // Only valid when is_logarithmic is true
    let zero_deadzone_halfsize: c_float = 0.0; // Drag widgets have no deadzone (as it doesn't make sense)
    if is_logarithmic {
        // When using logarithmic sliders, we need to clamp to avoid hitting zero, but our choice of clamp value greatly affects slider precision. We attempt to use the specified precision to estimate a good lower bound.
        let decimal_precision: f32 = if is_floating_point {
            ImParseFormatPrecision(format, 3)
        } else {
            1.0
        };
        logarithmic_zero_epsilon = (0.1 as c_float).powf(decimal_precision);

        // Convert to parametric space, apply delta, convert back
        let v_old_parametric: c_float = ScaleRatioFromValueT(
            data_type,
            v_cur,
            v_min,
            v_max,
            is_logarithmic,
            logarithmic_zero_epsilon,
            zero_deadzone_halfsize,
        );
        let v_new_parametric: c_float = v_old_parametric + g.DragCurrentAccum;
        v_cur = slider_ops::ScaleValueFromRatioT(
            data_type,
            v_new_parametric,
            v_min,
            v_max,
            is_logarithmic,
            logarithmic_zero_epsilon,
            zero_deadzone_halfsize,
        );
        v_old_ref_for_accum_remainder = v_old_parametric;
    } else {
        v_cur += g.DragCurrentAccum;
    }

    // Round to user desired precision based on format string
    if is_floating_point && flag_clear(flags, ImGuiSliderFlags_NoRoundToFormat) {
        v_cur = data_type_ops::RoundScalarWithFormatT(format, data_type, v_cur);
    }

    // Preserve remainder after rounding has been applied. This also allow slow tweaking of values.
    g.DragCurrentAccumDirty = false;
    if is_logarithmic {
        // Convert to parametric space, apply delta, convert back
        let v_new_parametric: c_float = slider_ops::ScaleRatioFromValueT(
            data_type,
            v_cur,
            v_min,
            v_max,
            is_logarithmic,
            logarithmic_zero_epsilon,
            zero_deadzone_halfsize,
        );
        g.DragCurrentAccum -= (v_new_parametric - v_old_ref_for_accum_remainder);
    } else {
        g.DragCurrentAccum -= (v_cur - v.clone());
    }

    // Lose zero sign for float/double
    if (v_cur == 0) {
        v_cur = 0;
    }

    // Clamp values (+ handle overflow/wrap-around for integer types)
    if *v != v_cur && is_clamped {
        if v_cur < v_min || (v_cur > *v && adjust_delta < 0.0 && !is_floating_point) {
            v_cur = v_min;
        }
        if v_cur > v_max || (v_cur < *v && adjust_delta > 0.0 && !is_floating_point) {
            v_cur = v_max;
        }
    }

    // Apply result
    if *v == v_cur {
        return false;
    }
    *v = v_cur.clone();
    return true;
}

pub unsafe fn DragBehavior(
    id: ImguiHandle,
    data_type: ImGuiDataType,
    p_v: &mut c_float,
    v_speed: c_float,
    p_min: c_float,
    p_max: c_float,
    format: &str,
    flags: ImGuiSliderFlags,
) -> bool {
    // Read imgui.cpp "API BREAKING CHANGES" section for 1.78 if you hit this assert.
    // IM_ASSERT((flags == 1 || flag_set(flags, ImGuiSliderFlags_InvalidMask_) == 0) && "Invalid flags: ImGuiSliderFlags! Has the 'float power' argument been mistakenly cast to flags? Call function with ImGuiSliderFlags_Logarithmic flags instead.");

    let g = GImGui; // ImGuiContext& g = *GImGui;
    if g.ActiveId == id {
        // Those are the things we can do easily outside the DragBehaviorT<> template, saves code generation.
        if g.ActiveIdSource == ImGuiInputSource_Mouse && !g.IO.MouseDown[0] {
            ClearActiveID(g);
        } else if g.ActiveIdSource == ImGuiInputSource_Nav
            && g.NavActivatePressedId == id
            && !g.ActiveIdIsJustActivated
        {
            ClearActiveID(g);
        }
    }
    if g.ActiveId != id {
        return false;
    }
    if flag_set(g.last_item_data.in_flags, ImGuiItemFlags_ReadOnly)
        || flag_set(flags, ImGuiSliderFlags_ReadOnly)
    {
        return false;
    }

    //     match data_type
    //     {
    //     IM_GUI_DATA_TYPE_S8 =>     {
    //         let v32 = p_v;
    //         let mut r: bool =  DragBehaviorT(IM_GUI_DATA_TYPE_S32, &v32, v_speed, if p_min {  p_min }else { IM_S8_MIN }, if { p_max } { p_max }  else { IM_S8_MAX },  format, flags); if r) *(*mut i8 { p_v = v32;} return r; }
    //     IM_GUI_DATA_TYPE_U8 =>     { v32: u32 = *(*mut u8)p_v;  let mut r: bool =  DragBehaviorT<u32, i32, c_float>(IM_GUI_DATA_TYPE_U32, &v32, v_speed, if p_min { *(*const u8) p_min }else { IM_U8_MIN }, if { p_max } { *(*const u8)p_max } else { IM_U8_MAX },  format, flags); if r) *(*mut u8 { p_v = v32;} return r; }
    //     IM_GUI_DATA_TYPE_S16 =>    { i32 v32 = *(*mut i16)p_v; let mut r: bool =  DragBehaviorT<i32, i32, c_float>(IM_GUI_DATA_TYPE_S32, &v32, v_speed, if p_min { p_min }else { IM_S16_MIN }, if p_max { p_max }else { IM_S16_MAX }, format, flags); if (r) *(*mut i16)p_v = v32; return r; }
    // IM_GUI_DATA_TYPE_U16 =>    { v32: u32 = *(*mut ImU16)p_v; let mut r: bool =  DragBehaviorT<u32, i32, c_float>(IM_GUI_DATA_TYPE_U32, &v32, v_speed, if p_min { p_min } else { IM_U16_MIN }, if  p_max { p_max } else { IM_U16_MAX }, format, flags); if (r) *(*mut ImU16)p_v = (ImU16)v32; return r; }
    //     IM_GUI_DATA_TYPE_S32 =>    return DragBehaviorT<i32, i32, c_float >(data_type, (*mut i32)p_v,  v_speed, if p_min { *(*const i32 )p_min } else { IM_S32_MIN }, if p_max { *(*const i32 )p_max } else { IM_S32_MAX }, format, flags);
    //     IM_GUI_DATA_TYPE_U32 =>    return DragBehaviorT<u32, i32, c_float >(data_type, (*mut u32)p_v,  v_speed, if p_min { *(*const u32 )p_min } else { IM_U32_MIN }, if p_max { *(*const u32 )p_max } else { IM_U32_MAX }, format, flags);
    //     IM_GUI_DATA_TYPE_S64 =>    return DragBehaviorT<i64, i64, double>(data_type, (*mut i64)p_v,  v_speed, if p_min { *(*const i64 )p_min } else { IM_S64_MIN }, if p_max { *(*const i64 )p_max } else { IM_S64_MAX, format, flags) };
    //     IM_GUI_DATA_TYPE_U64 =>    return DragBehaviorT<u64, i64, double>(data_type, (*mut u64)p_v,  v_speed, if p_min { *(*const u64 )p_min } else { IM_U64_MIN }, if p_max { *(*const u64 )p_max } else { IM_U64_MAX, format, flags) };
    //     IM_GUI_DATA_TYPE_FLOAT =>  return DragBehaviorT<c_float, c_float, c_float >(data_type, (&mut c_float)p_v,  v_speed, if p_min { *(*const c_float )p_min } else { -f32::MAX },  if  p_max { *(*const c_float )p_max } else { f32::MAX },    format, flags);
    //     IM_GUI_DATA_TYPE_DOUBLE => return DragBehaviorT<double,double,double>(data_type, (*mut double)p_v, v_speed, if p_min { p_min } else { -DBL_MAX },   if p_max { p_max }else { DBL_MAX },    format, flags);
    //     IM_GUI_DATA_TYPE_COUNT =>  break;
    //     }
    //     // IM_ASSERT(0);
    DragBehaviorT(data_type, p_v, v_speed, p_min, p_max, format, flags)
    // return false;
}

// Note: p_data, p_min and p_max are _pointers_ to a memory address holding the data. For a Drag widget, p_min and p_max are optional.
// Read code of e.g. DragFloat(), DragInt() etc. or examples in 'Demo->Widgets->Data Types' to understand how to use this function directly.
pub unsafe fn DragScalar<T>(
    label: String,
    data_type: ImGuiDataType,
    p_data: &mut c_float,
    v_speed: c_float,
    p_min: Option<c_float>,
    p_max: Option<c_float>,
    format: &mut String,
    flags: ImGuiSliderFlags,
) -> bool {
    let mut window = g.current_window_mut().unwrap();
    if window.skip_items {
        return false;
    }

    let g = GImGui; // ImGuiContext& g = *GImGui;
    let setyle = &mut g.style;
    let mut id: ImguiHandle = window.id_from_str(label, );
    let w: c_float = CalcItemWidth(g);

    let label_size: ImVec2 = CalcTextSize(, label, true, 0.0);
    let mut frame_bb: ImRect = ImRect::new(
        window.dc.cursor_pos,
        window.dc.cursor_pos + ImVec2::from_floats(w, label_size.y + style.FramePadding.y * 2.0),
    );
    let mut total_bb: ImRect = ImRect::new(
        frame_bb.min,
        frame_bb.max
            + ImVec2::from_floats(
                if label_size.x > 0.0 {
                    style.ItemInnerSpacing.x + label_size.x
                } else {
                    0.0
                },
                0.0,
            ),
    );

    let temp_input_allowed: bool = flag_clear(flags, ImGuiSliderFlags_NoInput);
    ItemSize(g, &total_bb.GetSize(), style.FramePadding.y);
    if !ItemAdd(
        g,
        &mut total_bb,
        id,
        &frame_bb,
        if temp_input_allowed {
            ImGuiItemFlags_Inputable
        } else {
            0
        },
    ) {
        return false;
    }

    // Default format string when passing NULL
    if format.is_empty() {
        *format = data_type_ops::data_type_info(data_type).PrintFmt;
    } else if data_type == IM_GUI_DATA_TYPE_S32 && format != String::format("{}") {
        // (FIXME-LEGACY: Patch old "{}f" format string to use "{}", read function more details.)
        *format = data_type_ops::PatchFormatStringFloatToInt(format);
    }

    let hovered: bool = ItemHoverable(&frame_bb, id);
    let mut temp_input_is_active: bool = temp_input_allowed && TempInputIsActive(id);
    if !temp_input_is_active {
        // Tabbing or CTRL-clicking on Drag turns it into an InputText
        let input_requested_by_tabbing: bool = temp_input_allowed
            && (g.last_item_data.StatusFlags & ImGuiItemStatusFlags_FocusedByTabbing) != 0;
        let clicked: bool = (hovered && g.IO.MouseClicked[0]);
        let double_clicked: bool = (hovered && g.IO.MouseClickedCount[0] == 2);
        let make_active: bool = (input_requested_by_tabbing
            || clicked
            || double_clicked
            || g.NavActivateId == id
            || g.NavActivateInputId == id);
        if make_active && temp_input_allowed {
            if input_requested_by_tabbing
                || (clicked && g.IO.KeyCtrl)
                || double_clicked
                || g.NavActivateInputId == id
            {
                temp_input_is_active = true;
            }
        }

        // (Optional) simple click (without moving) turns Drag into an InputText
        if g.IO.ConfigDragClickToInputText && temp_input_allowed && !temp_input_is_active {
            if g.ActiveId == id
                && hovered
                && g.IO.MouseReleased[0]
                && !IsMouseDragPastThreshold(
                    0,
                    g.IO.MouseDragThreshold * DRAG_MOUSE_THRESHOLD_FACTOR,
                )
            {
                g.NavActivateId = id;
                g.NavActivateInputId = id;
                g.NavActivateFlags = IM_GUI_ACTIVATE_FLAGS_PREFER_INPUT;
                temp_input_is_active = true;
            }
        }

        if make_active && !temp_input_is_active {
            SetActiveID(g, id, window);
            SetFocusID(id, window);
            FocusWindow(window);
            g.ActiveIdUsingNavDirMask = (1 << ImGuiDir_Left) | (1 << ImGuiDir_Right);
        }
    }

    if temp_input_is_active {
        // Only clamp CTRL+Click input when ImGuiSliderFlags_AlwaysClamp is set
        let is_clamp_input: bool = flag_set(flags, ImGuiSliderFlags_AlwaysClamp)
            && (p_min.is_none()
                || p_max.is_none()
                || DataTypeCompare(data_type, &p_min.unwrap_or(0.0), &p_max.unwrap_or(0.0)) < 0);
        return input_num_ops::TempInputScalar(
            &mut frame_bb,
            id,
            label,
            data_type,
            p_data,
            format,
            if is_clamp_input {
                p_min.unwrap_or(0.0)
            } else {
                c_float::MIN
            },
            if is_clamp_input {
                p_max.unwrap_or(0.0)
            } else {
                c_float::MIN
            },
        );
    }

    // Draw frame
    frame_col: u32 = GetColorU32(
        if g.ActiveId == id {
            ImGuiCol_FrameBgActive
        } else {
            if hovered {
                ImGuiCol_FrameBgHovered
            } else {
                ImGuiCol_FrameBg
            }
        },
        0.0,
    );
    RenderNavHighlight(, &frame_bb, id, 0);
    RenderFrame(
        frame_bb.min,
        frame_bb.max,
        frame_col,
        true,
        style.FrameRounding,
    );

    // Drag behavior
    let value_changed: bool = DragBehavior(
        id,
        data_type,
        p_data,
        v_speed,
        p_min.unwrap_or(c_float::MIN),
        p_max.unwrap_or(c_float::MIN),
        format,
        flags,
    );
    if value_changed {
        MarkItemEdited(g, id);
    }

    // Display value using user-provided display format so user can add prefix/suffix/decorations to the value.
    value_buf: [c_char; 64];
    let mut value_buf_end: &str = value_buf
        + data_type_ops::DataTypeFormatString(
            value_buf,
            value_buf.len(),
            data_type,
            *p_data,
            format,
        );
    if g.LogEnabled {
        // LogSetNextTextDecoration("{", "}");
    }
    let clip_rect = ImRect::from_vec2(
        &ImVec2::from_floats(0.5, 0.5),
        &ImVec2::from_floats(0.5, 0.5),
    );
    RenderTextClipped(
        &frame_bb.min,
        &frame_bb.max,
        value_buf,
        value_buf.len(),
        None,
        &clip_rect,
    );

    if label_size.x > 0.0 {
        RenderText(
            ImVec2::from_floats(
                frame_bb.max.x + style.ItemInnerSpacing.x,
                frame_bb.min.y + style.FramePadding.y,
            ),
            label,
            false,
            g,
        );
    }

    // IMGUI_TEST_ENGINE_ITEM_INFO(id, label, g.last_item_data.StatusFlags);
    return value_changed;
}

pub unsafe fn DragScalarN(
    label: String,
    data_type: ImGuiDataType,
    p_data: &mut [c_float],
    components: usize,
    v_speed: c_float,
    p_min: &mut [c_float],
    p_max: &mut [c_float],
    format: &str,
    flags: ImGuiSliderFlags,
) -> bool {
    let mut window = g.current_window_mut().unwrap();
    if window.skip_items {
        return false;
    }

    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut value_changed: bool = false;
    BeginGroup();
    push_str_id(g, label);
    PushMultiItemsWidths(components, CalcItemWidth(g));
    let type_size: size_t = GDATA_TYPE_INFO[data_type].Size;
    // for (let i: c_int = 0; i < components; i++)
    for i in 0..components {
        push_int_id(g, i as c_int);
        if i > 0 {
            same_line(g, 0.0, g.style.ItemInnerSpacing.x);
        }
        value_changed |= DragScalar(
            "",
            data_type,
            &mut p_data[i],
            v_speed,
            Some(p_min[i]),
            Some(p_max[i]),
            &mut String::from(format),
            flags,
        );
        pop_win_id_from_stack(g);
        PopItemWidth();
        p_data[i] = (p_data.clone() + type_size);
    }
    pop_win_id_from_stack(g);

    let mut label_end = FindRenderedTextEnd(label);
    if label.is_empty() == false {
        same_line(g, 0.0, g.style.ItemInnerSpacing.x);
        text_ops::TextEx(g, label, ImGuiTextFlags_None);
    }

    EndGroup();
    return value_changed;
}

pub unsafe fn DragFloat(
    label: String,
    v: &mut c_float,
    v_speed: c_float,
    v_min: c_float,
    v_max: c_float,
    format: &mut String,
    flags: ImGuiSliderFlags,
) -> bool {
    return DragScalar(
        label,
        IM_GUI_DATA_TYPE_FLOAT,
        v,
        v_speed,
        Some(v_min),
        Some(v_max),
        format,
        flags,
    );
}

pub unsafe fn DragFloat2(
    label: String,
    v: &mut [c_float; 2],
    v_speed: c_float,
    v_min: &mut [c_float; 2],
    v_max: &mut [c_float; 2],
    format: &str,
    flags: ImGuiSliderFlags,
) -> bool {
    return DragScalarN(
        label,
        IM_GUI_DATA_TYPE_FLOAT,
        v,
        2,
        v_speed,
        v_min,
        v_max,
        format,
        flags,
    );
}

pub unsafe fn DragFloat3(
    label: String,
    v: &mut [c_float; 3],
    v_speed: c_float,
    v_min: &mut [c_float; 3],
    v_max: &mut [c_float; 3],
    format: &str,
    flags: ImGuiSliderFlags,
) -> bool {
    return DragScalarN(
        label,
        IM_GUI_DATA_TYPE_FLOAT,
        v,
        3,
        v_speed,
        v_min,
        v_max,
        format,
        flags,
    );
}

pub unsafe fn DragFloat4(
    label: String,
    v: &mut [c_float; 4],
    v_speed: c_float,
    v_min: &mut [c_float; 4],
    v_max: &mut [c_float; 4],
    format: &str,
    flags: ImGuiSliderFlags,
) -> bool {
    return DragScalarN(
        label,
        IM_GUI_DATA_TYPE_FLOAT,
        v,
        4,
        v_speed,
        v_min,
        v_max,
        format,
        flags,
    );
}

// NB: You likely want to specify the ImGuiSliderFlags_AlwaysClamp when using this.
pub unsafe fn DragFloatRange2(
    label: String,
    v_current_min: &mut c_float,
    v_current_max: &mut c_float,
    v_speed: c_float,
    v_min: c_float,
    v_max: c_float,
    format: &mut String,
    format_max: &str,
    flags: ImGuiSliderFlags,
) -> bool {
    let mut window = g.current_window_mut().unwrap();
    if window.skip_items {
        return false;
    }

    let g = GImGui; // ImGuiContext& g = *GImGui;
    PushID(label);
    BeginGroup();
    PushMultiItemsWidths(2, CalcItemWidth(g));

    let mut min_min: c_float = if v_min >= v_max { -f32::MAX } else { v_min };
    let mut min_max: c_float = if v_min >= v_max {
        *v_current_max
    } else {
        v_max.min(*v_current_max)
    };
    let min_flags: ImGuiSliderFlags = flags
        | (if min_min == min_max {
            ImGuiSliderFlags_ReadOnly
        } else {
            0
        });
    let mut value_changed: bool = DragScalar(
        "##min",
        IM_GUI_DATA_TYPE_FLOAT,
        v_current_min,
        v_speed,
        Some(min_min),
        Some(min_max),
        format,
        min_flags,
    );
    PopItemWidth();
    same_line(g, 0.0, g.style.ItemInnerSpacing.x);

    let mut max_min: c_float = if v_min >= v_max {
        *v_current_min
    } else {
        ImMax(v_min, *v_current_min)
    };
    let mut max_max: c_float = if v_min >= v_max { f32::MAX } else { v_max };
    max_flags: ImGuiSliderFlags = flags
        | (if max_min == max_max {
            ImGuiSliderFlags_ReadOnly
        } else {
            0
        });
    value_changed |= DragScalar(
        "##max",
        IM_GUI_DATA_TYPE_FLOAT,
        v_current_max,
        v_speed,
        Some(max_min),
        Some(max_max),
        if format_max.is_empty() == false {
            &mut String::from(format_max)
        } else {
            &mut String::from(format)
        },
        max_flags,
    );
    PopItemWidth();
    same_line(g, 0.0, g.style.ItemInnerSpacing.x);

    text_ops::TextEx(g, label, ImGuiTextFlags_None);
    EndGroup();
    pop_win_id_from_stack(g);

    return value_changed;
}

// NB: v_speed is float to allow adjusting the drag speed with more precision
pub unsafe fn DragInt(
    label: String,
    v: &mut c_int,
    v_speed: c_float,
    v_min: c_int,
    v_max: c_int,
    format: &str,
    flags: ImGuiSliderFlags,
) -> bool {
    let mut v_float = c_float::from(*v);
    let mut v_min_float = c_float::from(v_min);
    let mut v_max_float = c_float::from(v_max);
    return DragScalar(
        label,
        IM_GUI_DATA_TYPE_S32,
        &mut v_float,
        v_speed,
        Some(v_min_float),
        Some(v_max_float),
        &mut String::from(format),
        flags,
    );
}

pub unsafe fn DragInt2(
    label: String,
    v: &mut [c_int; 2],
    v_speed: c_float,
    v_min: &[c_int; 2],
    v_max: &[c_int; 2],
    format: &str,
    flags: ImGuiSliderFlags,
) -> bool {
    let mut v_array: [c_float; 2] = [c_float::from(v[0]), c_float::from(v[1])];
    let mut v_min_array: [c_float; 2] = [c_float::from(v_min[0]), c_float::from(v_min[1])];
    let mut v_max_array: [c_float; 2] = [c_float::from(v_max[0]), c_float::from(v_max[1])];

    return DragScalarN(
        label,
        IM_GUI_DATA_TYPE_S32,
        &mut v_array,
        2,
        v_speed,
        &mut v_min_array,
        &mut v_max_array,
        format,
        flags,
    );
}

pub unsafe fn DragInt3(
    label: String,
    v: &mut [c_int; 3],
    v_speed: c_float,
    v_min: &[c_int; 3],
    v_max: &[c_int; 3],
    format: &str,
    flags: ImGuiSliderFlags,
) -> bool {
    let mut v_array: [c_float; 3] = [
        c_float::from(v[0]),
        c_float::from(v[1]),
        c_float::from(v[2]),
    ];
    let mut v_min_array: [c_float; 3] = [
        c_float::from(v_min[0]),
        c_float::from(v_min[1]),
        c_float::from(v_min[2]),
    ];
    let mut v_max_array: [c_float; 3] = [
        c_float::from(v_max[0]),
        c_float::from(v_max[1]),
        c_float::from(v_min[3]),
    ];
    return DragScalarN(
        label,
        IM_GUI_DATA_TYPE_S32,
        &mut v_array,
        3,
        v_speed,
        &mut v_min_array,
        &mut v_max_array,
        format,
        flags,
    );
}

pub unsafe fn DragInt4(
    label: String,
    v: &mut [c_int; 4],
    v_speed: c_float,
    v_min: &[c_int; 4],
    v_max: &[c_int; 4],
    format: &str,
    flags: ImGuiSliderFlags,
) -> bool {
    let mut v_array: [c_float; 4] = [
        c_float::from(v[0]),
        c_float::from(v[1]),
        c_float::from(v[2]),
        c_float::from(v[3]),
    ];
    let mut v_min_array: [c_float; 4] = [
        c_float::from(v_min[0]),
        c_float::from(v_min[1]),
        c_float::from(v_min[2]),
        c_float::from(v_min[3]),
    ];
    let mut v_max_array: [c_float; 4] = [
        c_float::from(v_max[0]),
        c_float::from(v_max[1]),
        c_float::from(v_min[3]),
        c_float::from(v_min[4]),
    ];
    return DragScalarN(
        label,
        IM_GUI_DATA_TYPE_S32,
        &mut v_array,
        4,
        v_speed,
        &mut v_min_array,
        &mut v_max_array,
        format,
        flags,
    );
}

// NB: You likely want to specify the ImGuiSliderFlags_AlwaysClamp when using this.
pub unsafe fn DragIntRange2(
    label: String,
    v_current_min: &mut c_int,
    v_current_max: &mut c_int,
    v_speed: c_float,
    v_min: c_int,
    v_max: c_int,
    format: &str,
    format_max: &str,
    flags: ImGuiSliderFlags,
) -> bool {
    let mut window = g.current_window_mut().unwrap();
    if window.skip_items {
        return false;
    }

    let g = GImGui; // ImGuiContext& g = *GImGui;
    PushID(label);
    BeginGroup();
    PushMultiItemsWidths(2, CalcItemWidth(g));

    let min_min: c_int = if v_min >= v_max { INT_MIN } else { v_min };
    let min_max: c_int = if v_min >= v_max {
        *v_current_max
    } else {
        ImMin(v_max, *v_current_max)
    };
    min_flags: ImGuiSliderFlags = flags
        | (if min_min == min_max {
            ImGuiSliderFlags_ReadOnly
        } else {
            0
        });
    let mut value_changed: bool = DragInt(
        "##min",
        v_current_min,
        v_speed,
        min_min,
        min_max,
        format,
        min_flags,
    );
    PopItemWidth();
    same_line(g, 0.0, g.style.ItemInnerSpacing.x);

    let max_min: c_int = if v_min >= v_max {
        *v_current_min
    } else {
        ImMax(v_min, *v_current_min)
    };
    let max_max: c_int = if v_min >= v_max { INT_MAX } else { v_max };
    max_flags: ImGuiSliderFlags = flags
        | (if max_min == max_max {
            ImGuiSliderFlags_ReadOnly
        } else {
            0
        });
    value_changed |= DragInt(
        "##max",
        v_current_max,
        v_speed,
        max_min,
        max_max,
        if format_max { format_max } else { format },
        max_flags,
    );
    PopItemWidth();
    same_line(g, 0.0, g.style.ItemInnerSpacing.x);

    text_ops::TextEx(g, label, ImGuiTextFlags_None);
    EndGroup();
    pop_win_id_from_stack(g);

    return value_changed;
}
