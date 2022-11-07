use crate::data_type::{
    ImGuiDataType, IM_GUI_DATA_TYPE_COUNT, IM_GUI_DATA_TYPE_DOUBLE, IM_GUI_DATA_TYPE_FLOAT,
    IM_GUI_DATA_TYPE_S16, IM_GUI_DATA_TYPE_S32, IM_GUI_DATA_TYPE_S64, IM_GUI_DATA_TYPE_S8,
    IM_GUI_DATA_TYPE_U16, IM_GUI_DATA_TYPE_U32, IM_GUI_DATA_TYPE_U64, IM_GUI_DATA_TYPE_U8,
};
use crate::data_type_info::{ImGuiDataTypeInfo, GDATA_TYPE_INFO};
use crate::core::math_ops::{ImAddClampOverflow, ImSubClampOverflow};
use libc::{c_float, c_int};

pub fn data_type_info(data_type: ImGuiDataType) -> ImGuiDataTypeInfo {
    // IM_ASSERT(data_type >= 0 && data_type < IM_GUI_DATA_TYPE_COUNT);
    return GDATA_TYPE_INFO[data_type].clone();
}

pub fn DataTypeFormatString(
    buf: &mut String,
    buf_size: usize,
    data_type: ImGuiDataType,
    p_data: c_float,
    format: &String,
) -> usize {
    todo!();
    // Signedness doesn't matter when pushing integer arguments
    if data_type == IM_GUI_DATA_TYPE_S32 || data_type == IM_GUI_DATA_TYPE_U32 {
        // return ImFormatString(buf, buf_size, format, *(*;const u32)p_data);
    }
    if data_type == IM_GUI_DATA_TYPE_S64 || data_type == IM_GUI_DATA_TYPE_U64 {
        // return ImFormatString(buf, buf_size, format, *(*;const u64)p_data);
    }
    if (data_type == IM_GUI_DATA_TYPE_FLOAT) {
        // return ImFormatString(buf, buf_size, format, *(*;const c_float)p_data);
    }
    if (data_type == IM_GUI_DATA_TYPE_DOUBLE) {
        // return ImFormatString(buf, buf_size, format, *(*;const double)p_data);
    }
    if (data_type == IM_GUI_DATA_TYPE_S8) {
        // return ImFormatString(buf, buf_size, format, *(*;const i8)p_data);
    }
    if (data_type == IM_GUI_DATA_TYPE_U8) {
        // return ImFormatString(buf, buf_size, format, *(*;const u8)p_data);
    }
    if (data_type == IM_GUI_DATA_TYPE_S16) {
        // return ImFormatString(buf, buf_size, format, *(*;const i16)p_data);
    }
    if (data_type == IM_GUI_DATA_TYPE_U16) {
        // return ImFormatString(buf, buf_size, format, *(*;const ImU16)p_data);
    }
    // IM_ASSERT(0);
    return 0;
}

pub type DataTypeOperation = c_int;

pub const DATA_TYPE_OPERATION_ADD: DataTypeOperation = 0;
pub const DATA_TYPE_OPERATION_SUB: DataTypeOperation = 1;

pub unsafe fn DataTypeApplyOp<T>(
    data_type: ImGuiDataType,
    op: DataTypeOperation,
    output: &mut T,
    arg1: &T,
    arg2: &T,
) {
    match data_type {
        IM_GUI_DATA_TYPE_S8 => {
            if op == DATA_TYPE_OPERATION_ADD {
                *output = ImAddClampOverflow(arg1, arg2, IM_S8_MIN, IM_S8_MAX);
            }
            if op == DATA_TYPE_OPERATION_SUB {
                *output = ImSubClampOverflow(arg1, arg2, IM_S8_MIN, IM_S8_MAX);
            }
            return;
        }
        IM_GUI_DATA_TYPE_U8 => {
            if op == DATA_TYPE_OPERATION_ADD {
                *output = ImAddClampOverflow(arg1, arg2, IM_U8_MIN, IM_U8_MAX);
            }
            if op == DATA_TYPE_OPERATION_SUB {
                *output = ImSubClampOverflow(arg1, arg2, IM_U8_MIN, IM_U8_MAX);
            }
            return;
        }
        IM_GUI_DATA_TYPE_S16 => {
            if op == DATA_TYPE_OPERATION_ADD {
                *output = ImAddClampOverflow(arg1, arg2, IM_S16_MIN, IM_S16_MAX);
            }
            if op == DATA_TYPE_OPERATION_SUB {
                *output = ImSubClampOverflow(arg1, arg2, IM_S16_MIN, IM_S16_MAX);
            }
            return;
        }
        IM_GUI_DATA_TYPE_U16 => {
            if op == DATA_TYPE_OPERATION_ADD {
                *output = ImAddClampOverflow(arg1, arg2, IM_U16_MIN, IM_U16_MAX);
            }
            if op == DATA_TYPE_OPERATION_SUB {
                *output = ImSubClampOverflow(arg1, arg2, IM_U16_MIN, IM_U16_MAX);
            }
            return;
        }
        IM_GUI_DATA_TYPE_S32 => {
            if op == DATA_TYPE_OPERATION_ADD {
                *output = ImAddClampOverflow(arg1, arg2, IM_S32_MIN, IM_S32_MAX);
            }
            if op == DATA_TYPE_OPERATION_SUB {
                *output = ImSubClampOverflow(arg1, arg2, IM_S32_MIN, IM_S32_MAX);
            }
            return;
        }
        IM_GUI_DATA_TYPE_U32 => {
            if op == DATA_TYPE_OPERATION_ADD {
                *output = ImAddClampOverflow(arg1, arg2, IM_U32_MIN, IM_U32_MAX);
            }
            if op == DATA_TYPE_OPERATION_SUB {
                *output = ImSubClampOverflow(arg1, arg2, IM_U32_MIN, IM_U32_MAX);
            }
            return;
        }
        IM_GUI_DATA_TYPE_S64 => {
            if op == DATA_TYPE_OPERATION_ADD {
                *output = ImAddClampOverflow(arg1, arg2, IM_S64_MIN, IM_S64_MAX);
            }
            if op == DATA_TYPE_OPERATION_SUB {
                *output = ImSubClampOverflow(arg1, arg2, IM_S64_MIN, IM_S64_MAX);
            }
            return;
        }
        IM_GUI_DATA_TYPE_U64 => {
            if op == DATA_TYPE_OPERATION_ADD {
                *output = ImAddClampOverflow(arg1, arg2, IM_U64_MIN, IM_U64_MAX);
            }
            if op == DATA_TYPE_OPERATION_SUB {
                *output = ImSubClampOverflow(arg1, arg2, IM_U64_MIN, IM_U64_MAX);
            }
            return;
        }
        IM_GUI_DATA_TYPE_FLOAT => {
            if op == DATA_TYPE_OPERATION_ADD {
                *output = arg1 + arg2;
            }
            if op == DATA_TYPE_OPERATION_SUB {
                *output = arg1 - arg2;
            }
            return;
        }
        IM_GUI_DATA_TYPE_DOUBLE => {
            if op == DATA_TYPE_OPERATION_ADD {
                *output = arg1 + arg2;
            }
            if op == DATA_TYPE_OPERATION_SUB {
                *output = arg1 - arg2;
            }
            return;
        }
        IM_GUI_DATA_TYPE_COUNT => {}
        _ => {}
    }
    // IM_ASSERT(0);
}

// User can input math operators (e.g. +100) to edit a numerical values.
// NB: This is _not_ a full expression evaluator. We should probably add one and replace this dumb mess..
pub unsafe fn DataTypeApplyFromText(
    buf: &str,
    data_type: ImGuiDataType,
    p_data: &mut c_float,
    format: &str,
) -> bool {
    // while (ImCharIsBlankA(*buf))
    //     buf+= 1;
    // if !buf[0] { return  false; }
    //
    // // Copy the value in an opaque buffer so we can compare at the end of the function if it changed at all.
    // let type_info: *const ImGuiDataTypeInfo = DataTypeGetInfo(data_type);
    // data_backup: ImGuiDataTypeTempStorage;
    // memcpy(&data_backup, p_data, type_info.Size);
    //
    // // Sanitize format
    // // For float/double we have to ignore format with precision (e.g. "{}") because sscanf doesn't take them in, so force them into {} and %lf
    // format_sanitized: [c_char;32];
    // if (data_type == IM_GUI_DATA_TYPE_FLOAT || data_type == IM_GUI_DATA_TYPE_DOUBLE)
    //     format = type_info->ScanFmt;
    // else
    //     format = ImParseFormatSanitizeForScanning(format, format_sanitized, format_sanitized.len());
    //
    // // Small types need a 32-bit buffer to receive the result from scanf()
    // let v32: c_int = 0;
    // if sscanf(buf, format, if type_info.Size >= 4 { p_data } else { &v32 }) < 1 { return  false; }
    // if (type_info.Size < 4)
    // {
    //     if (data_type == IM_GUI_DATA_TYPE_S8)
    //         p_data = ImClamp(v32, IM_S8_MIN, IM_S8_MAX);
    //     else if (data_type == IM_GUI_DATA_TYPE_U8)
    //         *(*mut u8)p_data = ImClamp(v32, IM_U8_MIN, IM_U8_MAX);
    //     else if (data_type == IM_GUI_DATA_TYPE_S16)
    //         *(*mut i16)p_data = ImClamp(v32, IM_S16_MIN, IM_S16_MAX);
    //     else if (data_type == IM_GUI_DATA_TYPE_U16)
    //         *(*mut ImU16)p_data = (ImU16)ImClamp(v32, IM_U16_MIN, IM_U16_MAX);
    //     else
    //         // IM_ASSERT(0);
    // }
    //
    // return memcmp(&data_backup, p_data, type_info.Size) != 0;
    todo!()
}

// template<typename T>
// pub fn DataTypeCompareT(*const T lhs, *const T rhs) -> c_int
pub fn DataTypeCompareT<T>(lhs: &T, rhs: &T) -> c_int {
    // if (*lhs < *rhs) return -1;
    // if *lhs > *rhs { return  1; }
    // return 0;
    if lhs < rhs {
        return -1;
    } else if lhs > rhs {
        return 1;
    }
    return 0;
}

pub fn DataTypeCompare<T>(data_type: ImGuiDataType, arg_1: &T, arg_2: &T) -> c_int {
    // switch (data_type)
    // {
    // IM_GUI_DATA_TYPE_S8 =>     return DataTypeCompareT<i8  >((*const i8  )arg_1, (*const i8  )arg_2);
    // IM_GUI_DATA_TYPE_U8 =>     return DataTypeCompareT<u8  >((*const u8  )arg_1, (*const u8  )arg_2);
    // IM_GUI_DATA_TYPE_S16 =>    return DataTypeCompareT<i16 >((*const i16 )arg_1, (*const i16 )arg_2);
    // IM_GUI_DATA_TYPE_U16 =>    return DataTypeCompareT<ImU16 >((*const ImU16 )arg_1, (*const ImU16 )arg_2);
    // IM_GUI_DATA_TYPE_S32 =>    return DataTypeCompareT<i32 >((*const i32 )arg_1, (*const i32 )arg_2);
    // IM_GUI_DATA_TYPE_U32 =>    return DataTypeCompareT<u32 >((*const u32 )arg_1, (*const u32 )arg_2);
    // IM_GUI_DATA_TYPE_S64 =>    return DataTypeCompareT<i64 >((*const i64 )arg_1, (*const i64 )arg_2);
    // IM_GUI_DATA_TYPE_U64 =>    return DataTypeCompareT<u64 >((*const u64 )arg_1, (*const u64 )arg_2);
    // IM_GUI_DATA_TYPE_FLOAT =>  return DataTypeCompareT<c_float >((*const c_float )arg_1, (*const c_float )arg_2);
    // IM_GUI_DATA_TYPE_DOUBLE => return DataTypeCompareT<double>((*const double)arg_1, (*const double)arg_2);
    // IM_GUI_DATA_TYPE_COUNT =>  break;
    // }
    // // IM_ASSERT(0);
    // return 0;
    DataTypeCompareT(arg_1, arg_2)
}

// template<typename T>
pub unsafe fn DataTypeClampT<T>(v: &mut T, v_min: &T, v_max: &T) -> bool {
    // Clamp, both sides are optional, return true if modified
    // if (v_min && *v < *v_min) { *v = *v_min; return true; }
    if *v < v_min {
        *v = v_min;
        return true;
    }
    if *v > v_max {
        *v = v_max;
        return true;
    }
    // if (v_max && *v > *v_max) { *v = *v_max; return true; }

    return false;
}

pub unsafe fn DataTypeClamp<T>(
    data_type: ImGuiDataType,
    p_data: &mut T,
    p_min: &T,
    p_max: &T,
) -> bool {
    // switch (data_type)
    // {
    // IM_GUI_DATA_TYPE_S8 =>     return DataTypeClampT<i8  >((*mut i8  )p_data, (*const i8  )p_min, (*const i8  )p_max);
    // IM_GUI_DATA_TYPE_U8 =>     return DataTypeClampT<u8  >((*mut u8  )p_data, (*const u8  )p_min, (*const u8  )p_max);
    // IM_GUI_DATA_TYPE_S16 =>    return DataTypeClampT<i16 >((*mut i16 )p_data, (*const i16 )p_min, (*const i16 )p_max);
    // IM_GUI_DATA_TYPE_U16 =>    return DataTypeClampT<ImU16 >((*mut ImU16 )p_data, (*const ImU16 )p_min, (*const ImU16 )p_max);
    // IM_GUI_DATA_TYPE_S32 =>    return DataTypeClampT<i32 >((*mut i32 )p_data, (*const i32 )p_min, (*const i32 )p_max);
    // IM_GUI_DATA_TYPE_U32 =>    return DataTypeClampT<u32 >((*mut u32 )p_data, (*const u32 )p_min, (*const u32 )p_max);
    // IM_GUI_DATA_TYPE_S64 =>    return DataTypeClampT<i64 >((*mut i64 )p_data, (*const i64 )p_min, (*const i64 )p_max);
    // IM_GUI_DATA_TYPE_U64 =>    return DataTypeClampT<u64 >((*mut u64 )p_data, (*const u64 )p_min, (*const u64 )p_max);
    // IM_GUI_DATA_TYPE_FLOAT =>  return DataTypeClampT<c_float >((&mut c_float )p_data, (*const c_float )p_min, (*const c_float )p_max);
    // IM_GUI_DATA_TYPE_DOUBLE => return DataTypeClampT<double>((*mut double)p_data, (*const double)p_min, (*const double)p_max);
    // IM_GUI_DATA_TYPE_COUNT =>  break;
    // }
    // // IM_ASSERT(0);
    // return false;
    DataTypeClampT(p_data, p_min, p_max)
}

pub fn GetMinimumStepAtDecimalPrecision(decimal_precision: usize) -> f32 {
    let min_steps: [c_float; 10] = [
        1.0, 0.1, 0.01, 0.001, 0.01, 0.001, 0.0001, 0.00001, 0.000001, 0.0000001,
    ];
    if decimal_precision < 0 {
        return f32::MIN;
    }
    return if decimal_precision < min_steps.len() {
        min_steps[decimal_precision]
    } else {
        // ImPow(10.0, - decimal_precision)
        let mut x = 10.0;
        x.powf(-decimal_precision as f64)
    };
}

// template<typename TYPE>
pub fn RoundScalarWithFormatT(format: &str, data_type: ImGuiDataType, v: T) -> T {
    // // IM_UNUSED(data_type);
    // // IM_ASSERT(data_type == IM_GUI_DATA_TYPE_FLOAT || data_type == IM_GUI_DATA_TYPE_DOUBLE);
    // let mut  fmt_start: &str = ImParseFormatFindStart(format);
    // if (fmt_start[0] != '%' || fmt_start[1] == '%') { // Don't apply if the value is not visible in the format string
    //     return v;
    // }
    //
    // // Sanitize format
    // fmt_sanitized: [c_char;32];
    // ImParseFormatSanitizeForPrinting(fmt_start, fmt_sanitized, fmt_sanitized.len());
    // fmt_start = fmt_sanitized;
    //
    // // Format value with our rounding, and read back
    // v_str: [c_char;64];
    // ImFormatString(v_str, v_str.len(), fmt_start, v);
    // let mut  p: &str = v_str;
    // while (*p == ' ')
    //     p+= 1;
    // v = ImAtof(p);
    //
    // return v;
    todo!()
}
