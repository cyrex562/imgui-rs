use crate::button_flags::{
    ImGuiButtonFlags, ImGuiButtonFlags_DontClosePopups, ImGuiButtonFlags_Repeat,
};
use crate::button_ops::ButtonEx;
use crate::data_type::{
    ImGuiDataType, ImGuiDataType_Double, ImGuiDataType_Float, ImGuiDataType_S32,
};
use crate::data_type_info::GDataTypeInfo;
use crate::data_type_ops::{
    DataTypeApplyFromText, DataTypeApplyOp, DataTypeFormatString, DATA_TYPE_OPERATION_ADD,
    DATA_TYPE_OPERATION_SUB,
};
use crate::data_type_temp_storage::ImGuiDataTypeTempStorage;
use crate::frame_ops::GetFrameHeight;
use crate::group_ops::{BeginGroup, EndGroup};
use crate::id_ops::PopID;
use crate::input_text_flags::{
    ImGuiInputTextFlags, ImGuiInputTextFlags_AutoSelectAll, ImGuiInputTextFlags_CharsDecimal,
    ImGuiInputTextFlags_CharsHexadecimal, ImGuiInputTextFlags_CharsScientific,
    ImGuiInputTextFlags_NoMarkEdited, ImGuiInputTextFlags_ReadOnly,
};
use crate::item_ops::{
    CalcItemWidth, MarkItemEdited, PopItemWidth, PushMultiItemsWidths, SetNextItemWidth,
};
use crate::layout_ops::SameLine;
use crate::math_ops::ImMax;
use crate::rect::ImRect;
use crate::render_ops::FindRenderedTextEnd;
use crate::string_ops::ImStrTrimBlanks;
use crate::type_defs::{ImGuiID, ImGuiInputTextCallback};
use crate::utils::flag_set;
use crate::vec2::ImVec2;
use crate::window::ops::{BeginDisabled, EndDisabled, GetCurrentWindow};
use crate::window::ImGuiWindow;
use crate::{data_type_ops, input_text, text_ops, widgets, GImGui};
use libc::{c_char, c_double, c_float, c_int, size_t, strlen};

pub unsafe fn InputScalar_DefaultCharsFilter(
    data_type: ImGuiDataType,
    format: &str,
) -> ImGuiInputTextFlags {
    if data_type == ImGuiDataType_Float || data_type == ImGuiDataType_Double {
        return ImGuiInputTextFlags_CharsScientific;
    }
    const format_last_char: c_char = if format[0] {
        format[strlen(format) - 1]
    } else {
        0
    };
    return if format_last_char == 'x' as c_char || format_last_char == 'X' as c_char {
        ImGuiInputTextFlags_CharsHexadecimal
    } else {
        ImGuiInputTextFlags_CharsDecimal
    };
}

// Note that Drag/Slider functions are only forwarding the min/max values clamping values if the ImGuiSliderFlags_AlwaysClamp flag is set!
// This is intended: this way we allow CTRL+Click manual input to set a value out of bounds, for maximum flexibility.
// However this may not be ideal for all uses, as some user code may break on out of bound values.
pub unsafe fn TempInputScalar(
    bb: &mut ImRect,
    id: ImGuiID,
    label: String,
    data_type: ImGuiDataType,
    p_data: &mut c_float,
    format: &mut String,
    mut p_clamp_min: c_float,
    mut p_clamp_max: c_float,
) -> bool {
    let mut fmt_buf = String::with_capacity(32);
    // data_buf: [c_char;32];
    let mut data_buf = String::with_capacity(32);
    *format = ImParseFormatTrimDecorations(format, fmt_buf, fmt_buf.len());
    data_type_ops::DataTypeFormatString(
        &mut data_buf,
        data_buf.len(),
        data_type,
        p_data.to_owned(),
        format,
    );
    ImStrTrimBlanks(&mut data_buf);

    let mut flags: ImGuiInputTextFlags =
        ImGuiInputTextFlags_AutoSelectAll | ImGuiInputTextFlags_NoMarkEdited;
    flags |= InputScalar_DefaultCharsFilter(data_type, format);

    let mut value_changed: bool = false;
    if input_text::TempInputText(bb, id, label, &mut data_buf, data_buf.len(), flags) {
        // Backup old value
        data_type_size: size_t = data_type_ops::DataTypeGetInfo(data_type).Size;
        let mut data_backup: ImGuiDataTypeTempStorage = ImGuiDataTypeTempStorage::default();
        // memcpy(&data_backup, p_data, data_type_size);

        // Apply new value (or operations) then clamp
        data_type_ops::DataTypeApplyFromText(&data_buf, data_type, p_data, format);
        if p_clamp_min || p_clamp_max {
            if p_clamp_min
                && p_clamp_max
                && data_type_ops::DataTypeCompare(data_type, &p_clamp_min, &p_clamp_max) > 0
            {
                // ImSwap(p_clamp_min, p_clamp_max);
                // let mut temp = p_clamp_min.clone();
                let mut temp = p_clamp_min;
                p_clamp_min = p_clamp_max;
                p_clamp_max = p_clamp_min;
            }
            data_type_ops::DataTypeClamp(data_type, p_data, &p_clamp_min, &p_clamp_max);
        }

        // Only mark as edited if new value is different
        // TODO
        // value_changed = memcmp(&data_backup, p_data, data_type_size) != 0;
        // if value_changed {
        //     MarkItemEdited(id); }
    }
    return value_changed;
}

// Note: p_data, p_step, p_step_fast are _pointers_ to a memory address holding the data. For an Input widget, p_step and p_step_fast are optional.
// Read code of e.g. InputFloat(), InputInt() etc. or examples in 'Demo->Widgets->Data Types' to understand how to use this function directly.
pub unsafe fn InputScalar(
    label: String,
    data_type: ImGuiDataType,
    p_data: &mut c_float,
    p_step: Option<c_float>,
    p_step_fast: Option<c_float>,
    format: &mut String,
    mut flags: ImGuiInputTextFlags,
) -> bool {
    let mut window = GetCurrentWindow();
    if window.SkipItems {
        return false;
    }

    let g = GImGui; // ImGuiContext& g = *GImGui;
    let style = &mut g.Style;

    if format.is_empty() {
        *format = data_type_ops::DataTypeGetInfo(data_type).PrintFmt;
    }

    // buf: [c_char;64];
    let mut buf = String::default();
    DataTypeFormatString(&mut buf, buf.len(), data_type, *p_data, format);

    // Testing ActiveId as a minor optimization as filtering is not needed until active
    if g.ActiveId == 0
        && (flags
            & (ImGuiInputTextFlags_CharsDecimal
                | ImGuiInputTextFlags_CharsHexadecimal
                | ImGuiInputTextFlags_CharsScientific))
            == 0
    {
        flags |= InputScalar_DefaultCharsFilter(data_type, format);
    }
    flags |= ImGuiInputTextFlags_AutoSelectAll | ImGuiInputTextFlags_NoMarkEdited; // We call MarkItemEdited() ourselves by comparing the actual data rather than the string.

    let mut value_changed: bool = false;
    if p_step.is_some() {
        let button_size: c_float = GetFrameHeight();

        BeginGroup(); // The only purpose of the group here is to allow the caller to query item data e.g. IsItemActive()
        PushID(label);
        SetNextItemWidth(ImMax(
            1.0,
            CalcItemWidth() - (button_size + style.ItemInnerSpacing.x) * 2,
        ));
        if InputText("", &mut buf, buf.len(), flags, None, None) {
            // PushId(label) + "" gives us the expected ID from outside point of view
            value_changed = DataTypeApplyFromText(buf.as_str(), data_type, p_data, format);
        }
        IMGUI_TEST_ENGINE_ITEM_INFO(g.LastItemData.ID, label, g.LastItemData.StatusFlags);

        // Step buttons
        let backup_frame_padding: ImVec2 = style.FramePadding;
        style.FramePadding.x = style.FramePadding.y;
        button_flags: ImGuiButtonFlags = ImGuiButtonFlags_Repeat | ImGuiButtonFlags_DontClosePopups;
        if flags & ImGuiInputTextFlags_ReadOnly {
            BeginDisabled(false);
        }
        SameLine(0.0, style.ItemInnerSpacing.x);
        if ButtonEx("-", ImVec2::new(button_size, button_size), button_flags) {
            DataTypeApplyOp(
                data_type,
                DATA_TYPE_OPERATION_SUB,
                p_data,
                *p_data,
                if g.IO.KeyCtrl && p_step_fast.is_some() {
                    p_step_fast.unwrap()
                } else {
                    p_step.unwrap()
                },
            );
            value_changed = true;
        }
        SameLine(0.0, style.ItemInnerSpacing.x);
        if ButtonEx("+", ImVec2::new(button_size, button_size), button_flags) {
            data_type_ops::DataTypeApplyOp(
                data_type,
                DATA_TYPE_OPERATION_ADD,
                p_data,
                *p_data,
                if g.IO.KeyCtrl && p_step_fast.is_some() {
                    p_step_fast.unwrap()
                } else {
                    p_step.unwrap()
                },
            );
            value_changed = true;
        }
        if flags & ImGuiInputTextFlags_ReadOnly {
            EndDisabled();
        }

        let mut label_end = FindRenderedTextEnd(label);
        if label != label_end {
            SameLine(0.0, style.ItemInnerSpacing.x);
            text_ops::TextEx(label, 0);
        }
        style.FramePadding = backup_frame_padding;

        PopID();
        EndGroup();
    } else {
        if InputText(label, &mut buf, buf.len(), flags, None, None) {
            value_changed = DataTypeApplyFromText(buf.as_str(), data_type, p_data, format);
        }
    }
    if value_changed {
        MarkItemEdited(g.LastItemData.ID);
    }

    return value_changed;
}

pub unsafe fn InputScalarN(
    label: String,
    data_type: ImGuiDataType,
    p_data: &mut [c_float],
    components: usize,
    p_step: Option<&[c_float]>,
    p_step_fast: Option<&[c_float]>,
    format: &mut String,
    flags: ImGuiInputTextFlags,
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
        value_changed |= InputScalar(
            "",
            data_type,
            &mut p_data[i],
            Some(p_step[i]),
            Some(p_step_fast[i]),
            format,
            flags,
        );
        PopID();
        PopItemWidth();
        // p_data = (p_data + type_size);
    }
    PopID();

    let mut label_end = FindRenderedTextEnd(label);
    if label != label_end {
        SameLine(0.0, g.Style.ItemInnerSpacing.x);
        text_ops::TextEx(label, 0);
    }

    EndGroup();
    return value_changed;
}

pub unsafe fn InputFloat(
    label: String,
    v: &mut c_float,
    step: c_float,
    step_fast: c_float,
    format: &mut String,
    mut flags: ImGuiInputTextFlags,
) -> bool {
    flags |= ImGuiInputTextFlags_CharsScientific;
    return InputScalar(
        label,
        ImGuiDataType_Float,
        v,
        (if step > 0.0 { Some(step) } else { None }),
        (if step_fast > 0.0 {
            Some(step_fast)
        } else {
            None
        }),
        format,
        flags,
    );
}

pub unsafe fn InputFloat2(
    label: String,
    v: &mut [c_float; 2],
    format: &mut String,
    flags: ImGuiInputTextFlags,
) -> bool {
    return InputScalarN(label, ImGuiDataType_Float, v, 2, None, None, format, flags);
}

pub unsafe fn InputFloat3(
    label: String,
    v: &mut [c_float; 3],
    format: &mut String,
    flags: ImGuiInputTextFlags,
) -> bool {
    return InputScalarN(label, ImGuiDataType_Float, v, 3, None, None, format, flags);
}

pub unsafe fn InputFloat4(
    label: String,
    v: &mut [c_float; 4],
    format: &mut String,
    flags: ImGuiInputTextFlags,
) -> bool {
    return InputScalarN(label, ImGuiDataType_Float, v, 4, None, None, format, flags);
}

pub unsafe fn InputInt(
    label: String,
    v: &mut c_int,
    step: c_int,
    step_fast: c_int,
    flags: ImGuiInputTextFlags,
) -> bool {
    // Hexadecimal input provided as a convenience but the flag name is awkward. Typically you'd use InputText() to parse your own data, if you want to handle prefixes.
    let mut v_float: c_float = c_flaot::from(*v);
    let mut format: String =
        String::from(if flag_set(flags, ImGuiInputTextFlags_CharsHexadecimal) {
            "{}"
        } else {
            "{}"
        });
    let step_float: c_float = c_float::from(step);
    let step_fast_float: c_float = c_float::from(step_fast);
    return InputScalar(
        label,
        ImGuiDataType_S32,
        &mut v_float,
        (if step > 0 { Some(step_float) } else { None }),
        (if step_fast > 0 {
            Some(step_fast_float)
        } else {
            None
        }),
        &mut format,
        flags,
    );
}

pub unsafe fn InputInt2(label: String, v: &mut [c_int; 2], flags: ImGuiInputTextFlags) -> bool {
    let mut v_float: [c_float; 2] = [c_float::from(v[0]), c_float::from(v[1])];
    let mut format = String::from("{}");
    return InputScalarN(
        label,
        ImGuiDataType_S32,
        &mut v_float,
        2,
        None,
        None,
        &mut format,
        flags,
    );
}

pub unsafe fn InputInt3(label: String, v: [c_int; 3], flags: ImGuiInputTextFlags) -> bool {
    let mut v_float: [c_float; 3] = [
        c_float::from(v[0]),
        c_float::from(v[1]),
        c_float::from(v[2]),
    ];
    let mut format = String::from("{}");
    return InputScalarN(
        label,
        ImGuiDataType_S32,
        &mut v_float,
        3,
        None,
        None,
        &mut format,
        flags,
    );
}

pub unsafe fn InputInt4(label: String, v: [c_int; 4], flags: ImGuiInputTextFlags) -> bool {
    let mut v_float: [c_float; 4] = [
        c_float::from(v[0]),
        c_float::from(v[1]),
        c_float::from(v[2]),
        c_float::from(v[3]),
    ];
    let mut format = String::from("{}");
    return InputScalarN(
        label,
        ImGuiDataType_S32,
        &mut v_float,
        4,
        None,
        None,
        &mut format,
        flags,
    );
}

pub unsafe fn InputDouble(
    label: String,
    v: &mut c_double,
    step: c_double,
    step_fast: c_double,
    format: &mut String,
    mut flags: ImGuiInputTextFlags,
) -> bool {
    flags |= ImGuiInputTextFlags_CharsScientific;
    let mut v_float = c_float::from(*v);
    let mut step_float = c_float::from(step);
    let mut step_fast_float = c_float::from(step_fast);
    return InputScalar(
        label,
        ImGuiDataType_Double,
        &mut v_float,
        (if step > 0.0 { Some(step_float) } else { None }),
        (if step_fast > 0.0 {
            Some(step_fast_float)
        } else {
            None
        }),
        format,
        flags,
    );
}

pub unsafe fn InputText(
    label: String,
    buf: &mut String,
    buf_size: size_t,
    flags: ImGuiInputTextFlags,
    callback: Option<ImGuiInputTextCallback>,
    user_data: Option<&Vec<u8>>,
) -> bool {
    // IM_ASSERT(flag_clear(flags, ImGuiInputTextFlags_Multiline)); // call InputTextMultiline()
    let mut size_arg = ImVec2::default();
    return input_text::InputTextEx(
        label,
        "",
        buf,
        buf_size,
        &mut size_arg,
        flags,
        callback,
        user_data,
    );
}
