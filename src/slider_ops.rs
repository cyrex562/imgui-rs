use crate::axis::{ImGuiAxis, ImGuiAxis_X, ImGuiAxis_Y};
use crate::color::{
    ImGuiCol_FrameBg, ImGuiCol_FrameBgActive, ImGuiCol_FrameBgHovered, ImGuiCol_SliderGrab,
    ImGuiCol_SliderGrabActive,
};
use crate::data_type::{
    ImGuiDataType, ImGuiDataType_Double, ImGuiDataType_Float, ImGuiDataType_S32,
};
use crate::data_type_info::GDataTypeInfo;
use crate::direction::{ImGuiDir_Down, ImGuiDir_Left, ImGuiDir_Right, ImGuiDir_Up};
use crate::group_ops::{BeginGroup, EndGroup};
use crate::id_ops::{ClearActiveID, PopID, SetActiveID};
use crate::imgui::GImGui;
use crate::input_ops::IsKeyDown;
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
use crate::layout_ops::SameLine;
use crate::logging_ops::LogSetNextTextDecoration;
use crate::math_ops::{ImLerp, ImMax, ImSwap};
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
use crate::style_ops::GetColorU32;
use crate::text_ops::CalcTextSize;
use crate::type_defs::ImGuiID;
use crate::utils::{flag_clear, flag_set};
use crate::vec2::ImVec2;
use crate::window::focus::FocusWindow;
use crate::window::ops::GetCurrentWindow;
use crate::window::ImGuiWindow;
use crate::{data_type_ops, input_num_ops, text_ops, widgets};
use libc::{c_char, c_float, c_int, size_t};
use std::borrow::Borrow;
use std::ptr::{null, null_mut};

// FIXME: Try to move more of the code into shared SliderBehavior()
// template<typename TYPE, typename SIGNEDTYPE, typename FLOATTYPE>
pub unsafe fn SliderBehaviorT(
    bb: &ImRect,
    id: ImGuiID,
    data_type: ImGuiDataType,
    v: &mut c_float,
    v_min: c_float,
    v_max: c_float,
    format: &str,
    flags: ImGuiSliderFlags,
    out_grab_bb: &mut ImRect,
) -> bool {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    let style = &mut g.Style;

    const axis: ImGuiAxis = if flags & ImGuiSliderFlags_Vertical {
        ImGuiAxis_Y
    } else {
        ImGuiAxis_X
    };
    let is_logarithmic: bool = flag_set(flags, ImGuiSliderFlags_Logarithmic);
    let is_floating_point: bool =
        (data_type == ImGuiDataType_Float) || (data_type == ImGuiDataType_Double);
    let v_range = (if v_min < v_max {
        v_max - v_min
    } else {
        v_min - v_max
    });

    // Calculate bounds
    let grab_padding: c_float = 2.0; // FIXME: Should be part of style.
    let slider_sz: c_float = (bb.Max[axis] - bb.Min[axis]) - grab_padding * 2.0;
    let mut grab_sz: c_float = style.GrabMinSize;
    if !is_floating_point && v_range >= 0.0 as c_float {
        // v_range < 0 may happen on integer overflows
        grab_sz = ImMax((slider_sz / (v_range + 1)), style.GrabMinSize);
    } // For integer sliders: if possible have the grab size represent 1 unit
    grab_sz = grab_sz.min(slider_sz);
    let slider_usable_sz: c_float = slider_sz - grab_sz;
    let slider_usable_pos_min: c_float = bb.Min[axis] + grab_padding + grab_sz * 0.5;
    let slider_usable_pos_max: c_float = bb.Max[axis] - grab_padding - grab_sz * 0.5;

    let mut logarithmic_zero_epsilon: c_float = 0.0; // Only valid when is_logarithmic is true
    let mut zero_deadzone_halfsize: c_float = 0.0; // Only valid when is_logarithmic is true
    if is_logarithmic {
        // When using logarithmic sliders, we need to clamp to avoid hitting zero, but our choice of clamp value greatly affects slider precision. We attempt to use the specified precision to estimate a good lower bound.
        let decimal_precision: c_int = if is_floating_point {
            ImParseFormatPrecision(format, 3)
        } else {
            1
        };
        logarithmic_zero_epsilon = (0.1 as c_float).powf(decimal_precision as f32);
        zero_deadzone_halfsize = (style.LogSliderDeadzone * 0.5) / ImMax(slider_usable_sz, 1.0);
    }

    // Process interacting with the slider
    let mut value_changed: bool = false;
    if g.ActiveId == id {
        let mut set_new_value: bool = false;
        let mut clicked_t: c_float = 0.0;
        if g.ActiveIdSource == ImGuiInputSource_Mouse {
            if !g.IO.MouseDown[0] {
                ClearActiveID();
            } else {
                let mouse_abs_pos: c_float = g.IO.MousePos[axis];
                if g.ActiveIdIsJustActivated {
                    let mut grab_t: c_float = ScaleRatioFromValueT(
                        data_type,
                        v,
                        v_min,
                        v_max,
                        is_logarithmic,
                        logarithmic_zero_epsilon,
                        zero_deadzone_halfsize,
                    );
                    if axis == ImGuiAxis_Y {
                        grab_t = 1.0 - grab_t;
                    }
                    let grab_pos: c_float =
                        ImLerp(slider_usable_pos_min, slider_usable_pos_max, grab_t);
                    let clicked_around_grab: bool = (mouse_abs_pos
                        >= grab_pos - grab_sz * 0.5 - 1.0)
                        && (mouse_abs_pos <= grab_pos + grab_sz * 0.5 + 1.0); // No harm being extra generous here.
                    g.SliderGrabClickOffset = if clicked_around_grab && is_floating_point {
                        mouse_abs_pos - grab_pos
                    } else {
                        0.0
                    };
                }
                if slider_usable_sz > 0.0 {
                    clicked_t = ImSaturate(
                        (mouse_abs_pos - g.SliderGrabClickOffset - slider_usable_pos_min)
                            / slider_usable_sz,
                    );
                }
                if axis == ImGuiAxis_Y {
                    clicked_t = 1.0 - clicked_t;
                }
                set_new_value = true;
            }
        } else if g.ActiveIdSource == ImGuiInputSource_Nav {
            if g.ActiveIdIsJustActivated {
                g.SliderCurrentAccum = 0.0; // Reset any stored nav delta upon activation
                g.SliderCurrentAccumDirty = false;
            }

            let mut input_delta: c_float = if axis == ImGuiAxis_X {
                GetNavTweakPressedAmount(axis) as c_float
            } else {
                -GetNavTweakPressedAmount(axis) as c_float
            };
            if input_delta != 0.0 {
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
                let decimal_precision: c_int = if is_floating_point {
                    ImParseFormatPrecision(format, 3)
                } else {
                    0
                };
                if decimal_precision > 0 {
                    input_delta /= 100; // Gamepad/keyboard tweak speeds in % of slider bounds
                    if tweak_slow {
                        input_delta /= 10.0;
                    }
                } else {
                    if (v_range >= -100.0 && v_range <= 100.0) || tweak_slow {
                        input_delta = if input_delta < 0.0 {
                            -1.0
                        } else {
                            1.0 / v_range
                        };
                    }
                    // Gamepad/keyboard tweak speeds in integer steps
                    else {
                        input_delta /= 100;
                    }
                }
                if tweak_fast {
                    input_delta *= 10.0;
                }
                g.SliderCurrentAccum += input_delta;
                g.SliderCurrentAccumDirty = true;
            }

            let delta: c_float = g.SliderCurrentAccum;
            if g.NavActivatePressedId == id && !g.ActiveIdIsJustActivated {
                ClearActiveID();
            } else if g.SliderCurrentAccumDirty {
                clicked_t = ScaleRatioFromValueT(
                    data_type,
                    v,
                    v_min,
                    v_max,
                    is_logarithmic,
                    logarithmic_zero_epsilon,
                    zero_deadzone_halfsize,
                );

                if (clicked_t >= 1.0 && delta > 0.0) || (clicked_t <= 0.0 && delta < 0.0)
                // This is to avoid applying the saturation when already past the limits
                {
                    set_new_value = false;
                    g.SliderCurrentAccum = 0.0; // If pushing up against the limits, don't continue to accumulate
                } else {
                    set_new_value = true;
                    let old_clicked_t: c_float = clicked_t;
                    clicked_t = ImSaturate(clicked_t + delta);

                    // Calculate what our "new" clicked_t will be, and thus how far we actually moved the slider, and subtract this from the accumulator
                    let mut v_new = ScaleValueFromRatioT(
                        data_type,
                        clicked_t,
                        v_min,
                        v_max,
                        is_logarithmic,
                        logarithmic_zero_epsilon,
                        zero_deadzone_halfsize,
                    );
                    if is_floating_point && flag_clear(flags, ImGuiSliderFlags_NoRoundToFormat) {
                        v_new = data_type_ops::RoundScalarWithFormatT(format, data_type, v_new);
                    }
                    let new_clicked_t: c_float = ScaleRatioFromValueT(
                        data_type,
                        &mut v_new,
                        v_min,
                        v_max,
                        is_logarithmic,
                        logarithmic_zero_epsilon,
                        zero_deadzone_halfsize,
                    );

                    if delta > 0.0 as c_float {
                        g.SliderCurrentAccum -= (new_clicked_t - old_clicked_t).min(delta);
                    } else {
                        g.SliderCurrentAccum -= ImMax(new_clicked_t - old_clicked_t, delta);
                    }
                }

                g.SliderCurrentAccumDirty = false;
            }
        }

        if set_new_value {
            let mut v_new = ScaleValueFromRatioT(
                data_type,
                clicked_t,
                v_min,
                v_max,
                is_logarithmic,
                logarithmic_zero_epsilon,
                zero_deadzone_halfsize,
            );

            // Round to user desired precision based on format string
            if is_floating_point && flag_clear(flags, ImGuiSliderFlags_NoRoundToFormat) {
                v_new = data_type_ops::RoundScalarWithFormatT(format, data_type, v_new);
            }

            // Apply result
            if *v != v_new {
                *v = v_new;
                value_changed = true;
            }
        }
    }

    if slider_sz < 1.0 {
        *out_grab_bb = ImRect(bb.Min, bb.Min);
    } else {
        // Output grab position so it can be displayed by the caller
        let mut grab_t: c_float = ScaleRatioFromValueT(
            data_type,
            v,
            v_min,
            v_max,
            is_logarithmic,
            logarithmic_zero_epsilon,
            zero_deadzone_halfsize,
        );
        if axis == ImGuiAxis_Y {
            grab_t = 1.0 - grab_t;
        }
        let grab_pos: c_float = ImLerp(slider_usable_pos_min, slider_usable_pos_max, grab_t);
        if axis == ImGuiAxis_X {
            *out_grab_bb = ImRect::from_floats(
                grab_pos - grab_sz * 0.5,
                bb.Min.y + grab_padding,
                grab_pos + grab_sz * 0.5,
                bb.Max.y - grab_padding,
            );
        } else {
            *out_grab_bb = ImRect::from_floats(
                bb.Min.x + grab_padding,
                grab_pos - grab_sz * 0.5,
                bb.Max.x - grab_padding,
                grab_pos + grab_sz * 0.5,
            );
        }
    }

    return value_changed;
}

// For 32-bit and larger types, slider bounds are limited to half the natural type range.
// So e.g. an integer Slider between INT_MAX-10 and INT_MAX will fail, but an integer Slider between INT_MAX/2-10 and INT_MAX/2 will be ok.
// It would be possible to lift that limitation with some work but it doesn't seem to be worth it for sliders.
pub unsafe fn SliderBehavior(
    bb: &ImRect,
    id: ImGuiID,
    data_type: ImGuiDataType,
    p_v: &mut c_float,
    p_min: c_float,
    p_max: c_float,
    format: &str,
    flags: ImGuiSliderFlags,
    out_grab_bb: &mut ImRect,
) -> bool {
    // Read imgui.cpp "API BREAKING CHANGES" section for 1.78 if you hit this assert.
    // IM_ASSERT((flags == 1 || flag_set(flags, ImGuiSliderFlags_InvalidMask_) == 0) && "Invalid flag: ImGuiSliderFlags!  Has the 'float power' argument been mistakenly cast to flags? Call function with ImGuiSliderFlags_Logarithmic flags instead.");

    // Those are the things we can do easily outside the SliderBehaviorT<> template, saves code generation.
    let g = GImGui; // ImGuiContext& g = *GImGui;
    if flag_set(g.LastItemData.InFlags, ImGuiItemFlags_ReadOnly)
        || flag_set(flags, ImGuiSliderFlags_ReadOnly)
    {
        return false;
    }

    SliderBehaviorT(
        bb,
        id,
        ImGuiDataType_S32,
        p_v,
        p_min,
        p_max,
        format,
        flags,
        out_grab_bb,
    )

    // match data_type
    // {
    // ImGuiDataType_S8 =>  {
    //     let v32 = p_v;
    //     let mut r: bool =  SliderBehaviorT(bb, id, ImGuiDataType_S32, &v32, p_min,  p_max,  format, flags, out_grab_bb);
    //     if r { p_v = v32;}
    //     return r; }
    // ImGuiDataType_U8 =>  { v32: u32 = *(*mut u8)p_v;  let mut r: bool =  SliderBehaviorT<u32, i32, c_float>(bb, id, ImGuiDataType_U32, &v32, *(*const u8)p_min,  *(*const u8)p_max,  format, flags, out_grab_bb); if r) *(*mut u8 { p_v = v32;}  return r; }
    // ImGuiDataType_S16 => { i32 v32 = *(*mut i16)p_v; let mut r: bool =  SliderBehaviorT<i32, i32, c_float>(bb, id, ImGuiDataType_S32, &v32, p_min, p_max, format, flags, out_grab_bb); if (r) *(*mut i16)p_v = v32; return r; }
    // ImGuiDataType_U16 => { v32: u32 = *(*mut ImU16)p_v; let mut r: bool =  SliderBehaviorT<u32, i32, c_float>(bb, id, ImGuiDataType_U32, &v32, p_min, p_max, format, flags, out_grab_bb); if (r) *(*mut ImU16)p_v = (ImU16)v32; return r; }
    // ImGuiDataType_S32 =>
    //     // IM_ASSERT(p_min >= IM_S32_MIN / 2 && p_max <= IM_S32_MAX / 2);
    //     return SliderBehaviorT<i32, i32, c_float >(bb, id, data_type, (*mut i32)p_v,  p_min,  p_max,  format, flags, out_grab_bb);
    // ImGuiDataType_U32 =>
    //     // IM_ASSERT(p_max <= IM_U32_MAX / 2);
    //     return SliderBehaviorT<u32, i32, c_float >(bb, id, data_type, (*mut u32)p_v,  p_min,  p_max,  format, flags, out_grab_bb);
    // ImGuiDataType_S64 =>
    //     // IM_ASSERT(*(*const ImS64)p_min >= IM_S64_MIN / 2 && *(*const ImS64)p_max <= IM_S64_MAX / 2);
    //     return SliderBehaviorT<i64, i64, double>(bb, id, data_type, (*mut i64)p_v,  p_min,  p_max,  format, flags, out_grab_bb);
    // ImGuiDataType_U64 =>
    //     // IM_ASSERT(p_max <= IM_U64_MAX / 2);
    //     return SliderBehaviorT<u64, i64, double>(bb, id, data_type, (*mut u64)p_v,  p_min,  p_max,  format, flags, out_grab_bb);
    // ImGuiDataType_Float =>
    //     // IM_ASSERT(p_min >= -f32::MAX / 2.0 && p_max <= f32::MAX / 2.0);
    //     return SliderBehaviorT<c_float, c_float, c_float >(bb, id, data_type, (&mut c_float)p_v,  p_min,  p_max,  format, flags, out_grab_bb);
    // ImGuiDataType_Double =>
    //     // IM_ASSERT(p_min >= -DBL_MAX / 2.0 && p_max <= DBL_MAX / 2.0);
    //     return SliderBehaviorT<double, double, double>(bb, id, data_type, (*mut double)p_v, p_min, p_max, format, flags, out_grab_bb);
    // ImGuiDataType_COUNT => break;
    // }
    // // IM_ASSERT(0);
    // return false;
}

// Note: p_data, p_min and p_max are _pointers_ to a memory address holding the data. For a slider, they are all required.
// Read code of e.g. SliderFloat(), SliderInt() etc. or examples in 'Demo->Widgets->Data Types' to understand how to use this function directly.
pub unsafe fn SliderScalar(
    label: String,
    data_type: ImGuiDataType,
    p_data: &mut c_float,
    p_min: c_float,
    p_max: c_float,
    format: &mut String,
    flags: ImGuiSliderFlags,
) -> bool {
    let mut window = GetCurrentWindow();
    if window.SkipItems {
        return false;
    }

    let g = GImGui; // ImGuiContext& g = *GImGui;
    let setyle = &mut g.Style;
    let mut id: ImGuiID = window.GetID(label);
    let w: c_float = CalcItemWidth();

    let label_size: ImVec2 = CalcTextSize(label, true, 0.0);
    let mut frame_bb: ImRect = ImRect::new(
        window.DC.CursorPos,
        window.DC.CursorPos + ImVec2::new(w, label_size.y + style.FramePadding.y * 2.0),
    );
    let mut total_bb: ImRect = ImRect::new(
        frame_bb.Min,
        frame_bb.Max
            + ImVec2::new(
                if label_size.x > 0.0 {
                    style.ItemInnerSpacing.x + label_size.x
                } else {
                    0.0
                },
                0.0,
            ),
    );

    let temp_input_allowed: bool = flag_clear(flags, ImGuiSliderFlags_NoInput);
    ItemSize(total_bb.GetSize().borrow(), style.FramePadding.y);
    if !ItemAdd(
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
    if format == None {
        *format = data_type_ops::DataTypeGetInfo(data_type).PrintFmt;
    } else if data_type == ImGuiDataType_S32 && format != String::from("{}") {
        // (FIXME-LEGACY: Patch old "{}f" format string to use "{}", read function more details.)
        *format = data_type_ops::PatchFormatStringFloatToInt(format);
    }
    let hovered: bool = ItemHoverable(&frame_bb, id);
    let mut temp_input_is_active: bool = temp_input_allowed && TempInputIsActive(id);
    if !temp_input_is_active {
        // Tabbing or CTRL-clicking on Slider turns it into an input box
        let input_requested_by_tabbing: bool = temp_input_allowed
            && (g.LastItemData.StatusFlags & ImGuiItemStatusFlags_FocusedByTabbing) != 0;
        let clicked: bool = (hovered && g.IO.MouseClicked[0]);
        let make_active: bool = (input_requested_by_tabbing
            || clicked
            || g.NavActivateId == id
            || g.NavActivateInputId == id);
        if make_active && temp_input_allowed {
            if input_requested_by_tabbing || (clicked && g.IO.KeyCtrl) || g.NavActivateInputId == id
            {
                temp_input_is_active = true;
            }
        }

        if make_active && !temp_input_is_active {
            SetActiveID(id, window);
            SetFocusID(id, window);
            FocusWindow(window);
            g.ActiveIdUsingNavDirMask |= (1 << ImGuiDir_Left) | (1 << ImGuiDir_Right);
        }
    }

    if temp_input_is_active {
        // Only clamp CTRL+Click input when ImGuiSliderFlags_AlwaysClamp is set
        let is_clamp_input: bool = flag_set(flags, ImGuiSliderFlags_AlwaysClamp);
        return input_num_ops::TempInputScalar(
            &mut frame_bb,
            id,
            label,
            data_type,
            p_data,
            format,
            if is_clamp_input { p_min } else { None },
            if is_clamp_input { p_max } else { None },
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
    RenderNavHighlight(&frame_bb, id, 0);
    RenderFrame(
        frame_bb.Min,
        frame_bb.Max,
        frame_col,
        true,
        g.Style.FrameRounding,
    );

    // Slider behavior
    let mut grab_bb: ImRect = ImRect::default();
    let value_changed: bool = SliderBehavior(
        &frame_bb,
        id,
        data_type,
        p_data,
        p_min,
        p_max,
        format,
        flags,
        &mut grab_bb,
    );
    if value_changed {
        MarkItemEdited(id);
    }

    // Render grab
    if grab_bb.Max.x > grab_bb.Min.x {
        window.DrawList.AddRectFilled(
            &grab_bb.Min,
            &grab_bb.Max,
            GetColorU32(
                if g.ActiveId == id {
                    ImGuiCol_SliderGrabActive
                } else {
                    ImGuiCol_SliderGrab
                },
                0.0,
            ),
            style.GrabRounding,
            0,
        );
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
        LogSetNextTextDecoration("{", "}");
    }
    RenderTextClipped(
        &frame_bb.Min,
        &frame_bb.Max,
        value_buf,
        None,
        ImVec2::new(0.5, 0.5),
        None,
    );

    if label_size.x > 0.0 {
        RenderText(
            ImVec2::new(
                frame_bb.Max.x + style.ItemInnerSpacing.x,
                frame_bb.Min.y + style.FramePadding.y,
            ),
            label,
            false,
        );
    }

    IMGUI_TEST_ENGINE_ITEM_INFO(id, label, g.LastItemData.StatusFlags);
    return value_changed;
}

// Add multiple sliders on 1 line for compact edition of multiple components
pub unsafe fn SliderScalarN(
    label: String,
    data_type: ImGuiDataType,
    v: &mut [c_float],
    components: usize,
    v_min: &mut [c_float],
    v_max: &mut [c_float],
    format: &str,
    flags: ImGuiSliderFlags,
) -> bool {
    let mut window = GetCurrentWindow();
    if window.SkipItems {
        return false;
    }

    let g = GImGui; // ImGuiContext& g = *GImGui;
    let mut value_changed: bool = false;
    BeginGroup();
    PushID(label);
    PushMultiItemsWidths(components, CalcItemWidth());
    type_size: size_t = GDataTypeInfo[data_type].Size;
    // for (let i: c_int = 0; i < components; i++)
    for i in 0..components {
        PushID(i);
        if i > 0 {
            SameLine(0.0, g.Style.ItemInnerSpacing.x);
        }
        value_changed |= SliderScalar(
            "",
            data_type,
            &mut v[i],
            v_min[i],
            v_max[i],
            format.as_str(),
            flags,
        );
        PopID();
        PopItemWidth();
        *v = (*v + type_size);
    }
    PopID();

    let mut label_end = FindRenderedTextEnd(label);
    if label.len() > 0 {
        SameLine(0.0, g.Style.ItemInnerSpacing.x);
        text_ops::TextEx(label, 0);
    }

    EndGroup();
    return value_changed;
}

pub unsafe fn SliderFloat(
    label: String,
    v: &mut c_float,
    v_min: c_float,
    v_max: c_float,
    format: &mut String,
    flags: ImGuiSliderFlags,
) -> bool {
    return SliderScalar(label, ImGuiDataType_Float, v, v_min, v_max, format, flags);
}

pub unsafe fn SliderFloat2(
    label: String,
    v: &mut [c_float; 2],
    v_min: &mut [c_float; 2],
    v_max: &mut [c_float; 2],
    format: &str,
    flags: ImGuiSliderFlags,
) -> bool {
    return SliderScalarN(
        label,
        ImGuiDataType_Float,
        v,
        2,
        v_min,
        v_max,
        format,
        flags,
    );
}

pub unsafe fn SliderFloat3(
    label: String,
    v: &mut [c_float; 3],
    v_min: &mut [c_float; 3],
    v_max: &mut [c_float; 3],
    format: &str,
    flags: ImGuiSliderFlags,
) -> bool {
    return SliderScalarN(label, ImGuiDataType_Float, v, 3, v_min, _max, format, flags);
}

pub unsafe fn SliderFloat4(
    label: String,
    v: &mut [c_float; 4],
    v_min: &mut [c_float; 4],
    v_max: &mut [c_float; 4],
    format: &str,
    flags: ImGuiSliderFlags,
) -> bool {
    return SliderScalarN(
        label,
        ImGuiDataType_Float,
        v,
        4,
        v_min,
        v_max,
        format,
        flags,
    );
}

pub unsafe fn SliderAngle(
    label: String,
    v_rad: &mut c_float,
    v_degrees_min: c_float,
    v_degrees_max: c_float,
    format: &mut String,
    flags: ImGuiSliderFlags,
) -> bool {
    if format.is_empty() {
        *format = "{} deg".into_string();
    }
    let mut v_deg: c_float = (*v_rad) * 360f32 / (2 * IM_PI);
    let mut value_changed: bool = SliderFloat(
        label,
        &mut v_deg,
        v_degrees_min,
        v_degrees_max,
        format,
        flags,
    );
    *v_rad = v_deg * (2 * IM_PI) / 360f32;
    return value_changed;
}

pub unsafe fn SliderInt(
    label: String,
    v: &mut c_int,
    v_min: &mut c_int,
    v_max: &mut c_int,
    format: &mut String,
    flags: ImGuiSliderFlags,
) -> bool {
    let mut v_float = *v as c_float;
    let mut v_min_float = *v_min as c_float;
    let mut v_max_float = *v_max as c_float;
    return SliderScalar(
        label,
        ImGuiDataType_S32,
        &mut v_float,
        v_min_float,
        v_max_float,
        format,
        flags,
    );
}

pub unsafe fn SliderInt2(
    label: String,
    v: &mut [c_int; 2],
    v_min: &mut [c_int; 2],
    v_max: &mut [c_int; 2],
    format: &str,
    flags: ImGuiSliderFlags,
) -> bool {
    let mut v_float: [c_float; 2] = [v[0] as c_float, v[1] as c_float];
    let mut v_min_float: [c_float; 2] = [v_min[0] as c_float, v_min[1] as c_float];
    let mut v_max_float: [c_float; 2] = [v_max[0] as c_float, v_max[1] as c_float];
    return SliderScalarN(
        label,
        ImGuiDataType_S32,
        &mut v_float,
        2,
        &mut v_min_float,
        &mut v_max_float,
        format,
        flags,
    );
}

pub unsafe fn SliderInt3(
    label: String,
    v: [c_int; 3],
    v_min: c_int,
    v_max: c_int,
    format: &str,
    flags: ImGuiSliderFlags,
) -> bool {
    let mut v_float: [c_float; 3] = [v[0] as c_float, v[1] as c_float, v[2] as c_float];
    let mut v_min_float: [c_float; 3] = [
        v_min[0] as c_float,
        v_min[1] as c_float,
        v_min[2] as c_float,
    ];
    let mut v_max_float: [c_float; 3] = [
        v_max[0] as c_float,
        v_max[1] as c_float,
        v_max[2] as c_float,
    ];
    return SliderScalarN(
        label,
        ImGuiDataType_S32,
        &mut v_float,
        3,
        &mut v_min_float,
        &mut v_max_float,
        format,
        flags,
    );
}

pub unsafe fn SliderInt4(
    label: String,
    v: [c_int; 4],
    v_min: c_int,
    v_max: c_int,
    format: &str,
    flags: ImGuiSliderFlags,
) -> bool {
    let mut v_float: [c_float; 4] = [
        v[0] as c_float,
        v[1] as c_float,
        v[2] as c_float,
        v[3] as c_float,
    ];
    let mut v_min_float: [c_float; 4] = [
        v_min[0] as c_float,
        v_min[1] as c_float,
        v_min[2] as c_float,
        v_min[3] as c_float,
    ];
    let mut v_max_float: [c_float; 4] = [
        v_max[0] as c_float,
        v_max[1] as c_float,
        v_max[2] as c_float,
        v_max[3] as c_float,
    ];
    return SliderScalarN(
        label,
        ImGuiDataType_S32,
        &mut v_float,
        4,
        &mut v_min_float,
        &mut v_max_float,
        format,
        flags,
    );
}

pub unsafe fn VSliderScalar(
    label: String,
    size: &ImVec2,
    data_type: ImGuiDataType,
    p_data: &mut c_float,
    p_min: c_float,
    p_max: c_float,
    format: &mut String,
    flags: ImGuiSliderFlags,
) -> bool {
    let mut window = GetCurrentWindow();
    if window.SkipItems {
        return false;
    }

    let g = GImGui; // ImGuiContext& g = *GImGui;
    let setyle = &mut g.Style;
    let mut id: ImGuiID = window.GetID(label);

    let label_size: ImVec2 = CalcTextSize(label, true, 0.0);
    let mut frame_bb: ImRect = ImRect::new(window.DC.CursorPos, window.DC.CursorPos + size);
    let mut bb: ImRect = ImRect::new(
        frame_bb.Min,
        frame_bb.Max
            + ImVec2::new(
                if label_size.x > 0.0 {
                    style.ItemInnerSpacing.x + label_size.x
                } else {
                    0.0
                },
                0.0,
            ),
    );

    ItemSize(&bb.GetSize(), style.FramePadding.y);
    if !ItemAdd(&mut frame_bb, id, None, 0) {
        return false;
    }

    // Default format string when passing NULL
    if format.is_empty() {
        *format = data_type_ops::DataTypeGetInfo(data_type).PrintFmt;
    } else if data_type == ImGuiDataType_S32 && format != "{}".into_string() {
        // (FIXME-LEGACY: Patch old "{}f" format string to use "{}", read function more details.)
        *format = data_type_ops::PatchFormatStringFloatToInt(format);
    }

    let hovered: bool = ItemHoverable(&frame_bb, id);
    if (hovered && g.IO.MouseClicked[0]) || g.NavActivateId == id || g.NavActivateInputId == id {
        SetActiveID(id, window);
        SetFocusID(id, window);
        FocusWindow(window);
        g.ActiveIdUsingNavDirMask |= (1 << ImGuiDir_Up) | (1 << ImGuiDir_Down);
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
    RenderNavHighlight(&frame_bb, id, 0);
    RenderFrame(
        frame_bb.Min,
        frame_bb.Max,
        frame_col,
        true,
        g.Style.FrameRounding,
    );

    // Slider behavior
    let mut grab_bb: ImRect = ImRect::default();
    let value_changed: bool = SliderBehavior(
        &frame_bb,
        id,
        data_type,
        p_data,
        p_min,
        p_max,
        format,
        flags | ImGuiSliderFlags_Vertical,
        &mut grab_bb,
    );
    if value_changed {
        MarkItemEdited(id);
    }

    // Render grab
    if grab_bb.Max.y > grab_bb.Min.y {
        window.DrawList.AddRectFilled(
            &grab_bb.Min,
            &grab_bb.Max,
            GetColorU32(
                if g.ActiveId == id {
                    ImGuiCol_SliderGrabActive
                } else {
                    ImGuiCol_SliderGrab
                },
                0.0,
            ),
            style.GrabRounding,
            0,
        );
    }

    // Display value using user-provided display format so user can add prefix/suffix/decorations to the value.
    // For the vertical slider we allow centered text to overlap the frame padding
    value_buf: [c_char; 64];
    let mut value_buf_end: &str = value_buf
        + data_type_ops::DataTypeFormatString(
            value_buf,
            value_buf.len(),
            data_type,
            *p_data,
            format,
        );
    RenderTextClipped(
        ImVec2::new(frame_bb.Min.x, frame_bb.Min.y + style.FramePadding.y),
        &frame_bb.Max,
        value_buf,
        None,
        ImVec2::new(0.5, 0.0),
        None,
    );
    if label_size.x > 0.0 {
        RenderText(
            ImVec2::new(
                frame_bb.Max.x + style.ItemInnerSpacing.x,
                frame_bb.Min.y + style.FramePadding.y,
            ),
            label,
            false,
        );
    }

    return value_changed;
}

pub unsafe fn VSliderFloat(
    label: String,
    size: &ImVec2,
    v: &mut c_float,
    v_min: c_float,
    v_max: c_float,
    format: &mut String,
    flags: ImGuiSliderFlags,
) -> bool {
    return VSliderScalar(
        label,
        size,
        ImGuiDataType_Float,
        v,
        v_min,
        v_max,
        format,
        flags,
    );
}

pub unsafe fn VSliderInt(
    label: String,
    size: &ImVec2,
    v: &mut c_int,
    v_min: &mut c_int,
    v_max: &mut c_int,
    format: &mut String,
    flags: ImGuiSliderFlags,
) -> bool {
    let mut v_float = v as c_float;
    let mut v_min_float = v as c_float;
    let mut v_max_float = v as c_float;
    return VSliderScalar(
        label,
        size,
        ImGuiDataType_S32,
        &mut v_float,
        v_min_float,
        v_max_float,
        format,
        flags,
    );
}

// Convert a value v in the output space of a slider into a parametric position on the slider itself (the logical opposite of ScaleValueFromRatioT)
pub fn ScaleRatioFromValueT(
    data_type: ImGuiDataType,
    v: &mut c_float,
    mut v_min: c_float,
    mut v_max: c_float,
    is_logarithmic: bool,
    logarithmic_zero_epsilon: c_float,
    zero_deadzone_halfsize: c_float,
) -> c_float {
    if v_min == v_max {
        return 0.0;
    }
    // IM_UNUSED(data_type);

    let v_clamped = if v_min < v_max {
        v.clamp(v_min, v_max)
    } else {
        v.clamp(v_max, v_min)
    };
    return if is_logarithmic {
        let mut flipped: bool = v_max < v_min;

        if flipped {
            // Handle the case where the range is backwards
            ImSwap(&mut v_min, &mut v_max);
        }

        // Fudge min/max to avoid getting close to log(0)
        let mut v_min_fudged: c_float = if v_min.abs() < logarithmic_zero_epsilon {
            if v_min < 0.0 as c_float {
                logarithmic_zero_epsilon * -1
            } else {
                logarithmic_zero_epsilon
            }
        } else {
            v_min
        };
        let mut v_max_fudged = if v_max.abs() < logarithmic_zero_epsilon {
            if v_max < 0.0 as c_float {
                -logarithmic_zero_epsilon
            } else {
                logarithmic_zero_epsilon
            }
        } else {
            v_max
        };

        // Awkward special cases - we need ranges of the form (-100 .. 0) to convert to (-100 .. -epsilon), not (-100 .. epsilon)
        if (v_min == 0.0) && (v_max < 0.0) {
            v_min_fudged = logarithmic_zero_epsilon * -1;
        } else if (v_max == 0.0) && (v_min < 0.0) {
            v_max_fudged = logarithmic_zero_epsilon;
        }

        let mut result: c_float = 0.0;
        if v_clamped <= v_min_fudged {
            result = 0.0;
        }
        // Workaround for values that are in-range but below our fudge
        else if v_clamped >= v_max_fudged {
            result = 1.0;
        }
        // Workaround for values that are in-range but above our fudge
        // Range crosses zero, so split into two portions
        else if (v_min * v_max) < 0.0 {
            let zero_point_center: c_float = (-v_min) / (v_max - v_min); // The zero point in parametric space.  There's an argument we should take the logarithmic nature into account when calculating this, but for now this should do (and the most common case of a symmetrical range works fine)
            let zero_point_snap_L: c_float = zero_point_center - zero_deadzone_halfsize;
            let zero_point_snap_R: c_float = zero_point_center + zero_deadzone_halfsize;
            if v == 0.0 {
                result = zero_point_center;
            }
            // Special case for exactly zero
            else if *v < 0.0 {
                result = (1.0
                    - (ImLog(-v_clamped / logarithmic_zero_epsilon)
                        / ImLog(-v_min_fudged / logarithmic_zero_epsilon)))
                    * zero_point_snap_L;
            } else {
                result = zero_point_snap_R
                    + ((ImLog(v_clamped / logarithmic_zero_epsilon)
                        / ImLog(v_max_fudged / logarithmic_zero_epsilon))
                        * (1.0 - zero_point_snap_R));
            }
        } else if (v_min < 0.0) || (v_max < 0.0) {
            // Entirely negative slider
            result =
                1.0 - (ImLog(-v_clamped / -v_max_fudged) / ImLog(-v_min_fudged / -v_max_fudged));
        } else {
            result = (ImLog(v_clamped / v_min_fudged) / ImLog(v_max_fudged / v_min_fudged));
        }

        if flipped {
            (1.0 - result)
        } else {
            result
        }
    } else {
        // Linear slider
        (v_clamped - v_min) / (v_max - v_min)
    };
}

// Convert a parametric position on a slider into a value v in the output space (the logical opposite of ScaleRatioFromValueT)
// template<typename TYPE, typename SIGNEDTYPE, typename FLOATTYPE>
pub fn ScaleValueFromRatioT(
    data_type: ImGuiDataType,
    t: c_float,
    v_min: c_float,
    v_max: c_float,
    is_logarithmic: bool,
    logarithmic_zero_epsilon: c_float,
    zero_deadzone_halfsize: c_float,
) -> c_float {
    // We special-case the extents because otherwise our logarithmic fudging can lead to "mathematically correct"
    // but non-intuitive behaviors like a fully-left slider not actually reaching the minimum value. Also generally simpler.
    if t <= 0.0 || v_min == v_max {
        return v_min.clone();
    }
    if t >= 1.0 {
        return v_max.clone();
    }

    let mut result = 0.0 as c_float;
    if is_logarithmic {
        // Fudge min/max to avoid getting silly results close to zero
        let v_min_fudged = if v_min.abs() < logarithmic_zero_epsilon {
            if v_min < 0.0 as c_float {
                logarithmic_zero_epsilon * -1
            } else {
                logarithmic_zero_epsilon
            }
        } else {
            v_min
        };
        let v_max_fudged = if v_max.abs() < logarithmic_zero_epsilon {
            if v_max < 0.0 as c_float {
                logarithmic_zero_epsilon * -1
            } else {
                logarithmic_zero_epsilon
            }
        } else {
            v_max
        };

        let flipped: bool = v_max < v_min; // Check if range is "backwards"
        if flipped {
            ImSwap(v_min_fudged, v_max_fudged);
        }

        // Awkward special case - we need ranges of the form (-100 .. 0) to convert to (-100 .. -epsilon), not (-100 .. epsilon)
        if (v_max == 0.0) && (v_min < 0.0) {
            v_max_fudged = -logarithmic_zero_epsilon;
        }

        let t_with_flip: c_float = if flipped { (1.0 - t) } else { t }; // t, but flipped if necessary to account for us flipping the range

        if (v_min * v_max) < 0.0
        // Range crosses zero, so we have to do this in two parts
        {
            let zero_point_center: c_float = (v_min.min(v_max) * -1) / (v_max - v_min).abs(); // The zero point in parametric space
            let zero_point_snap_L: c_float = zero_point_center - zero_deadzone_halfsize;
            let zero_point_snap_R: c_float = zero_point_center + zero_deadzone_halfsize;
            if t_with_flip >= zero_point_snap_L && t_with_flip <= zero_point_snap_R {
                result = 0.0;
            }
            // Special case to make getting exactly zero possible (the epsilon prevents it otherwise)
            else if t_with_flip < zero_point_center {
                result = -(logarithmic_zero_epsilon
                    * ImPow(
                        -v_min_fudged / logarithmic_zero_epsilon,
                        (1.0 - (t_with_flip / zero_point_snap_L)),
                    ));
            } else {
                result = (logarithmic_zero_epsilon
                    * ImPow(
                        v_max_fudged / logarithmic_zero_epsilon,
                        ((t_with_flip - zero_point_snap_R) / (1.0 - zero_point_snap_R)),
                    ));
            }
        } else if (v_min < 0.0) || (v_max < 0.0) {
            // Entirely negative slider
            result = -(-v_max_fudged * ImPow(-v_min_fudged / -v_max_fudged, (1.0 - t_with_flip)));
        } else {
            result = (v_min_fudged * ImPow(v_max_fudged / v_min_fudged, t_with_flip));
        }
    } else {
        // Linear slider
        let is_floating_point: bool =
            (data_type == ImGuiDataType_Float) || (data_type == ImGuiDataType_Double);
        if is_floating_point {
            result = ImLerp(v_min, v_max, t);
        } else if t < 1.0 {
            // - For integer values we want the clicking position to match the grab box so we round above
            //   This code is carefully tuned to work with large values (e.g. high ranges of U64) while preserving this property..
            // - Not doing a *1.0 multiply at the end of a range as it tends to be lossy. While absolute aiming at a large s64/u64
            //   range is going to be imprecise anyway, with this check we at least make the edge values matches expected limits.
            let v_new_off_f = (v_max - v_min) * t;
            result = (v_min + (v_new_off_f + (if v_min > v_max { -0.5 } else { 0.5 })));
        }
    }

    return result;
}
