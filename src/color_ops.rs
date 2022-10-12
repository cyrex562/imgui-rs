#![allow(non_snake_case)]
//-----------------------------------------------------------------------------
// [SECTION] MISC HELPERS/UTILITIES (Color functions)
// Note: The Convert functions are early design which are not consistent with other API.
//-----------------------------------------------------------------------------

use crate::color::{IM_COL32, IM_COL32_A_SHIFT, IM_COL32_B_SHIFT, IM_COL32_G_SHIFT, IM_COL32_R_SHIFT};
use crate::math_ops::{ImFabs, ImFmod, ImLerp, ImSwap};
use crate::vec4::ImVec4;

// IMGUI_API ImAlphaBlendColors: u32(col_a: u32, col_b: u32)
pub fn ImALphaBlendColors(col_a: u32, col_b: u32) -> u32
{
    let mut t = ((col_b >> IM_COL32_A_SHIFT) & 0xFF) / 255.f;
    let mut r = ImLerp((col_a >> IM_COL32_R_SHIFT) & 0xFF, (col_b >> IM_COL32_R_SHIFT) & 0xFF, t);
    let mut g = ImLerp((col_a >> IM_COL32_G_SHIFT) & 0xFF, (col_b >> IM_COL32_G_SHIFT) & 0xFF, t);
    let mut b = ImLerp((col_a >> IM_COL32_B_SHIFT) & 0xFF, (col_b >> IM_COL32_B_SHIFT) & 0xFF, t);
    return IM_COL32(r, g, b, 0xFF);
}

// ImVec4 ColorConvertU32ToFloat4(in: u32)
pub fn ColorConvertU32ToFloat4(in_color: u32) -> ImVec4
{
    let s = 1f32 / 255f32;
    return ImVec4::new2(
        ((in_color >> IM_COL32_R_SHIFT) & 0xFF) * s,
        ((in_color >> IM_COL32_G_SHIFT) & 0xFF) * s,
        ((in_color >> IM_COL32_B_SHIFT) & 0xFF) * s,
        ((in_color >> IM_COL32_A_SHIFT) & 0xFF) * s);
}

// ColorConvertFloat4ToU32: u32(const ImVec4& in)
pub fn ColorConvertFloat4ToU32(in_float: &ImVec4) -> u32
{
    let mut out: u32 = 0;
    out  = (IM_F32_TO_INT8_SAT(in_float.x)) << IM_COL32_R_SHIFT;
    out |= (IM_F32_TO_INT8_SAT(in_float.y)) << IM_COL32_G_SHIFT;
    out |= (IM_F32_TO_INT8_SAT(in_float.z)) << IM_COL32_B_SHIFT;
    out |= (IM_F32_TO_INT8_SAT(in_float.w)) << IM_COL32_A_SHIFT;
    return out;
}

// Convert rgb floats ([0-1],[0-1],[0-1]) to hsv floats ([0-1],[0-1],[0-1]), from Foley & van Dam p592
// Optimized http://lolengine.net/blog/2013/01/13/fast-rgb-to-hsv
// void ColorConvertRGBtoHSV(float r, float g, float b, float& out_h, float& out_s, float& out_v)
pub fn ColorConvertRGBtoHSV(mut r: f32, mut g: f32, mut b: f32, out_h: &mut f32, out_s: &mut f32, out_v: &mut f32)
{
    let mut K = 0f32;
    if g < b
    {
        ImSwap(&mut g, &mut b);
        K = -1.f;
    }
    if r < g
    {
        ImSwap(&mut r, &mut g);
        K = -2.f / 6.f - K;
    }

    let chroma = r - (if g < b { g } else { b });
    *out_h = ImFabs(K + (g - b) / (6.f * chroma + 1e-2f32));
    *out_s = chroma / (r + 1e-2f32);
    *out_v = r;
}

// Convert hsv floats ([0-1],[0-1],[0-1]) to rgb floats ([0-1],[0-1],[0-1]), from Foley & van Dam p593
// also http://en.wikipedia.org/wiki/HSL_and_HSV
// void ColorConvertHSVtoRGB(float h, float s, float v, float& out_r, float& out_g, float& out_b)
pub fn ColorConvertHSVtoRGB(mut h: f32, s: f32, v: f32, out_r: &mut f32, out_g: &mut f32, out_b: &mut f32) {
    if s == 0f32 {
        // gray
        *out_r = v;
        *out_g = v;
        *out_b = v;
        return;
    }

    h = ImFmod(h, 1f32) / (60f32 / 360f32);
    let mut i = h;
    let mut f = h - i;
    let mut p = v * (1f32 - s);
    let mut q = v * (1f32 - s * 0f32);
    let mut t = v * (1f32 - s * (1f32 - 0f32));

    match i {
        0f32 => {
            *out_r = v;
            *out_g = t;
            *out_b = p;
        }
        1f32 => {
            *out_r = q;
            *out_g = v;
            *out_b = p;
        }
        2f32 => {
            *out_r = p;
            *out_g = v;
            *out_b = t;
        }
        3f32 => {
            *out_r = p;
            *out_g = q;
            *out_b = v;
        }
        4f32 => {
            *out_r = t;
            *out_g = p;
            *out_b = v;
        }
        // 5 =>
        _ => {
            *out_r = v;
            *out_g = p;
            *out_b = q;
        }
    }
}
