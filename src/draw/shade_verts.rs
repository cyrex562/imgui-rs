use crate::color::{COLOR32_A_MASK, IM_COL32_B_SHIFT, IM_COL32_G_SHIFT, IM_COL32_R_SHIFT};
use crate::Context;
use crate::draw::DrawList;
use crate::vectors::vec_length_sqr;
use crate::vectors::Vector2D;

// Generic linear color gradient, write to RGB fields, leave A untouched.
// void ImGui::ShadeVertsLinearColorGradientKeepAlpha(ImDrawList* draw_list, int vert_start_idx, int vert_end_idx, Vector2D gradient_p0, Vector2D gradient_p1, ImU32 col0, ImU32 col1)
pub fn shade_verts_linear_color_gradient_keep_alpha(draw_list: &mut DrawList, vert_start_idx: usize, vert_end_idx: usize, gradient_p0: Vector2D, gradient_p1: Vector2D, col0: u32, col1: u32) {
    let gradient_extent = gradient_p1 - gradient_p0;
    let gradient_inv_length2 = 1.0 / vec_length_sqr(&gradient_extent);
    let vert_start = draw_list.vtx_buffer.data + vert_start_idx;
    let vert_end = draw_list.vtx_buffer.data + vert_end_idx;
    let col0_r = (col0 >> IM_COL32_R_SHIFT) & 0xFF;
    let col0_g = (col0 >> IM_COL32_G_SHIFT) & 0xFF;
    let col0_b = (col0 >> IM_COL32_B_SHIFT) & 0xFF;
    let col_delta_r = ((col1 >> IM_COL32_R_SHIFT) & 0xFF) - col0_r;
    let col_delta_g = ((col1 >> IM_COL32_G_SHIFT) & 0xFF) - col0_g;
    let col_delta_b = ((col1 >> IM_COL32_B_SHIFT) & 0xFF) - col0_b;
    // for (ImDrawVert* vert = vert_start; vert < vert_end; vert += 1)
    for vert in vert_start..vert_end {
        let d = Vector2D::dot(vert.pos - &gradient_p0, &gradient_extent);
        let t = ImClamp(d * gradient_inv_length2, 0.0, 1.0);
        let r = (col0_r + col_delta_r * t);
        let g = (col0_g + col_delta_g * t);
        let b = (col0_b + col_delta_b * t);
        vert.col = (r << IM_COL32_R_SHIFT) | (g << IM_COL32_G_SHIFT) | (b << IM_COL32_B_SHIFT) | (vert.col & COLOR32_A_MASK);
    }
}

// Distribute UV over (a, b) rectangle
// void shade_verts_linear_uv(ImDrawList* draw_list, int vert_start_idx, int vert_end_idx, const Vector2D& a, const Vector2D& b, const Vector2D& uv_a, const Vector2D& uv_b, bool clamp)
pub fn shade_verts_linear_uv(draw_list: &mut DrawList, vert_start_idx: i32, vert_end_idx: i32, a: &Vector2D, b: &Vector2D, uv_a: &Vector2D, uv_b: &Vector2D, clamp: bool) {
    let size = b - a;
    let uv_size = uv_b - uv_a;
    let scale = Vector2D::new(
        if size.x != 0.0 { (uv_size.x / size.x) } else { 0.0 },
        if size.y != 0.0 { (uv_size.y / size.y) } else { 0.0 });

    let vert_start = draw_list.vtx_buffer.data + vert_start_idx;
    let vert_end = draw_list.vtx_buffer.data + vert_end_idx;
    if clamp {
        let min = Vector2D::min(uv_a, uv_b);
        let max = Vector2D::max(uv_a, uv_b);
        // for (ImDrawVert* vertex = vert_start; vertex < vert_end;  += 1vertex)
        for vertex in vert_start..vert_end {
            vertex.uv = Vector2D::clamp(uv_a + ((Vector2D::new(vertex.pos.x, vertex.pos.y) - a.clone()) * &scale), &min, &max);
        }
    } else {
        // for (ImDrawVert* vertex = vert_start; vertex < vert_end;  += 1vertex)
        for vertex in vert_start..vert_end {
            // vertex.uv = uv_a + ImMul(Vector2D::new(vertex.pos.x, vertex.pos.y) - a, scale);
            vertex.uv = uv_a + ((Vector2D::new(vertex.pos.x, vertex.pos.y) - a.clone()) * &scale)
        }
    }
}
