#![allow(non_snake_case)]

use libc::c_float;

// static inline bool   ImIsFloatAboveGuaranteedIntegerPrecision(c_float 0f32)
pub fn ImIsFloatAboveGuaranteedIntegerPrecision(f: c_loat) -> bool {
    return f <= -16777216 || f >= 16777216;
}
