use crate::imgui_h::{IM_COL32, IM_COL32_A_SHIFT, IM_COL32_B_SHIFT, IM_COL32_G_SHIFT, IM_COL32_R_SHIFT, ImVec4};
use crate::imgui_math::{IM_F32_TO_INT8_SAT, ImFabs, ImFmod, ImLerpU32};

// IMGUI_API ImU32 ImAlphaBlendColors(ImU32 col_a, ImU32 col_b)
pub fn ImAlphaBlendColors(col_a: u32, col_b: u32) -> u32
{
    // float t = ((col_b >> IM_COL32_A_SHIFT) & 0xFF) / 255.f;
    let t = ((col_b >> IM_COL32_A_SHIFT) & 0xff) / 255.0;
    // int r = ImLerp((int)(col_a >> IM_COL32_R_SHIFT) & 0xFF, (int)(col_b >> IM_COL32_R_SHIFT) & 0xFF, t);
    let r = ImLerpU32((col_a >> IM_COL32_R_SHIFT) & 0xff, (col_b >> IM_COL32_R_SHIFT) & 0xff, t);
    // int g = ImLerp((int)(col_a >> IM_COL32_G_SHIFT) & 0xFF, (int)(col_b >> IM_COL32_G_SHIFT) & 0xFF, t);
    let g = ImLerpU32((col_a >> IM_COL32_G_SHIFT) & 0xff, (col_b >> IM_COL32_G_SHIFT) & 0xff, t);
    // int b = ImLerp((int)(col_a >> IM_COL32_B_SHIFT) & 0xFF, (int)(col_b >> IM_COL32_B_SHIFT) & 0xFF, t);
    let b = ImLerpU32((col_a >> IM_COL32_B_SHIFT) & 0xff, (col_b >> IM_COL32_B_SHIFT) & 0xff, t);
    IM_COL32(r, g, b, 0xFF)
}

// ImVec4 ImGui::ColorConvertU32ToFloat4(ImU32 in)
pub fn ColorCovertU32ToFloat(in_u32: u32) -> ImVec4
{
    // float s = 1.0 / 255.0;
    let mut s: f32 = 1.0 / 255.0;
    return ImVec4{
        x:((in_u32 >> IM_COL32_R_SHIFT) & 0xFF) * s,
        y:((in_u32 >> IM_COL32_G_SHIFT) & 0xFF) * s,
        z:((in_u32 >> IM_COL32_B_SHIFT) & 0xFF) * s,
        w:((in_u32 >> IM_COL32_A_SHIFT) & 0xFF) * s};
}

// ImU32 ImGui::ColorConvertFloat4ToU32(const ImVec4& in)
pub fn ColorConvertFloat4ToU32(in_vec: &ImVec4) -> u32
{
    // ImU32 out;

    let mut out: u32  = ((IM_F32_TO_INT8_SAT(in_vec.x)) << IM_COL32_R_SHIFT )as u32;
    out |= ((IM_F32_TO_INT8_SAT(in_vec.y)) << IM_COL32_G_SHIFT) as u32;
    out |= ((IM_F32_TO_INT8_SAT(in_vec.z)) << IM_COL32_B_SHIFT) as u32;
    out |= ((IM_F32_TO_INT8_SAT(in_vec.w)) << IM_COL32_A_SHIFT) as u32;
    out
}

// Convert rgb floats ([0-1],[0-1],[0-1]) to hsv floats ([0-1],[0-1],[0-1]), from Foley & van Dam p592
// Optimized http://lolengine.net/blog/2013/01/13/fast-rgb-to-hsv
// void ImGui::ColorConvertRGBtoHSV(float r, float g, float b, float& out_h, float& out_s, float& out_v)
pub fn ColorConvertRGBtoHSV(r: f32, g: f32, b: f32, out_h: &mut f32, out_s: &mut f32, out_v: &mut f32)
{
    // float K = 0.f;
    let mut K: f32 = 0.0;
    if (g < b)
    {
        ImSwap(g, b);
        K = -1.0;
    }
    if (r < g)
    {
        ImSwap(r, g);
        K = -2.0 / 6.0 - K;
    }

    let mut chroma: f32 = r - (if g < b { g } else { b });
    *out_h = ImFabs(K + (g - b) / (6.0 * chroma + 1e-20));
    *out_s = chroma / (r + 1e-20);
    *out_v = r;
}

// Convert hsv floats ([0-1],[0-1],[0-1]) to rgb floats ([0-1],[0-1],[0-1]), from Foley & van Dam p593
// also http://en.wikipedia.org/wiki/HSL_and_HSV
// void ImGui::ColorConvertHSVtoRGB(float h, float s, float v, float& out_r, float& out_g, float& out_b)
pub fn ColorConvertHSVtoRGB(mut h: f32, s: f32, v: f32, out_r: &mut f32, out_g: &mut f32, out_b: &mut f32)
{
    if s == 0.0
    {
        // gray

        *out_r = v;
        *out_g = v;
        *out_b = v;
        return;
    }

    h = ImFmod(h, 1.0) / (60.0 / 360.0);
    // int   i = (int)h;
    let mut i: i32 = h as i32;
    // float f = h - (float)i;
    let mut f: f32 = h - i as f32;
    // float p = v * (1.0 - s);
    let mut p: f32 = v * (1.0 - s);
    // float q = v * (1.0 - s * f);
    let mut q: f32 = v * (1.0 - s * f);
    // float t = v * (1.0 - s * (1.0 - f));
    let mut t: f32 = v * (1.0 - s * (1.0 - f));

    // switch (i)
    // {
    match i {
        // case 0: out_r = v; out_g = t; out_b = p; break;
        0 => { *out_r = v; *out_g = t; *out_b = p}
        // case 1: out_r = q; out_g = v; out_b = p; break;
        1 => { *out_r = q; *out_g = v; *out_b = p}
        // case 2: out_r = p; out_g = v; out_b = t; break;
        2 => {*out_r = p; *out_g = v; *out_b = t}
        // case 3: out_r = p; out_g = q; out_b = v; break;
        3 => {*out_r = p; *out_g = q; *out_b = v}
        // case 4: out_r = t; out_g = p; out_b = v; break;
        4 => {*out_r = t; *out_g = p; *out_b = v}
        // case 5: default: out_r = v; out_g = p; out_b = q; break;
        _ => {*out_r = v; *out_g = p; *out_b = q}
        // }
    }
}
