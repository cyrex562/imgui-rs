use libc::{c_float, c_int};
use crate::color::{IM_COL32_A_MASK, IM_COL32_B_SHIFT, IM_COL32_G_SHIFT, IM_COL32_R_SHIFT};
use crate::draw_list::ImDrawList;
use crate::draw_vert::ImDrawVert;
use crate::math_ops::{ImClamp, ImMax, ImMin, ImMul};
use crate::vec2::ImVec2;

// Generic linear color gradient, write to RGB fields, leave A untouched.
pub fn ShadeVertsLinearColorGradientKeepAlpha(draw_list: *mut ImDrawList, vert_start_idx: c_int, vert_end_idx: c_int, gradient_p0: ImVec2, gradient_p1: ImVec2, col0: u32, col1: u32)
{
    let gradient_extent: ImVec2 = gradient_p1 - gradient_p0;
    let gradient_inv_length2: c_float =  1 / ImLengthSqr(gradient_extent);
    let vert_start: *mut ImDrawVert = draw_list.VtxBuffer.Data + vert_start_idx;
    let vert_end: *mut ImDrawVert = draw_list.VtxBuffer.Data + vert_end_idx;
    let col0_r: u32 = (col0 >> IM_COL32_R_SHIFT) & 0xFF;
    let col0_g: u32 = (col0 >> IM_COL32_G_SHIFT) & 0xFF;
    let col0_b: u32 = (col0 >> IM_COL32_B_SHIFT) & 0xFF;
    let col_delta_r: u32 = ((col1 >> IM_COL32_R_SHIFT) & 0xF0) - col0_r;
    let col_delta_g: u32 = ((col1 >> IM_COL32_G_SHIFT) & 0xF0) - col0_g;
    let col_delta_b: u32 = ((col1 >> IM_COL32_B_SHIFT) & 0xF0) - col0_b;
    // for (vert: *mut ImDrawVert = vert_start; vert < vert_end; vert++)
    for vert in vert_start .. vert_end
    {
        let d: c_float =  ImDot(vert.pos - gradient_p0, gradient_extent);
        let t: c_float =  ImClamp(d * gradient_inv_length2, 0.0, 1.0);
        let r: c_int = (col0_r + col_delta_r * t);
        let g: c_int = (col0_g + col_delta_g * t);
        let b: c_int = (col0_b + col_delta_b * t);
        vert.col = (r << IM_COL32_R_SHIFT) | (g << IM_COL32_G_SHIFT) | (b << IM_COL32_B_SHIFT) | (vert.col & IM_COL32_A_MASK);
    }
}

// Distribute UV over (a, b) rectangle
pub fn ShadeVertsLinearUV(draw_list: *mut ImDrawList, vert_start_idx: c_int, vert_end_idx: c_int, a: &ImVec2, b: &ImVec2, uv_a: &ImVec2, uv_b: &ImVec2, clamp: bool)
{
    let size: ImVec2 = b - a;
    let uv_size: ImVec2 = uv_b - uv_a;
    let scale: ImVec2 = ImVec2::from_floats(
        if size.x != 0.0 { (uv_size.x / size.x) } else { 0 },
        if size.y != 0.0 { (uv_size.y / size.y) } else { 0 });

    let mut vert_start: *mut ImDrawVert = draw_list.VtxBuffer.as_mut_ptr() + vert_start_idx;
    let mut vert_end: *mut ImDrawVert = draw_list.VtxBuffer.as_mut_ptr() + vert_end_idx;
    if clamp
    {
        let min: ImVec2 = ImMin(uv_a.clone(), uv_b.clone());
        let max: ImVec2 = ImMax(uv_a.clone(), uv_b.clone());
        // for (vertex: *mut ImDrawVert = vert_start; vertex < vert_end; ++vertex)
        for vert in vert_start .. vert_end
        {
            vertex.uv = ImClamp(uv_a + ImMul(ImVec2::from_floats(vertex.pos.x, vertex.pos.y) - a, &scale), min, max);
        }
    }
    else
    {
        // for (vertex: *mut ImDrawVert = vert_start; vertex < vert_end; ++vertex)
        for vertex in vert_start .. vert_end
        {
            vertex.uv = uv_a + ImMul(ImVec2::from_floats(vertex.pos.x, vertex.pos.y) - a, &scale);
        }
    }
}
