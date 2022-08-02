
// Generic linear color gradient, write to RGB fields, leave A untouched.
void ImGui::ShadeVertsLinearColorGradientKeepAlpha(ImDrawList* draw_list, int vert_start_idx, int vert_end_idx, Vector2D gradient_p0, Vector2D gradient_p1, ImU32 col0, ImU32 col1)
{
    Vector2D gradient_extent = gradient_p1 - gradient_p0;
    let gradient_inv_length2 =  1.0 / ImLengthSqr(gradient_extent);
    ImDrawVert* vert_start = draw_list.vtx_buffer.data + vert_start_idx;
    ImDrawVert* vert_end = draw_list.vtx_buffer.data + vert_end_idx;
    let col0_r = (col0 >> IM_COL32_R_SHIFT) & 0xFF;
    let col0_g = (col0 >> IM_COL32_G_SHIFT) & 0xFF;
    let col0_b = (col0 >> IM_COL32_B_SHIFT) & 0xFF;
    let col_delta_r = ((col1 >> IM_COL32_R_SHIFT) & 0xFF) - col0_r;
    let col_delta_g = ((col1 >> IM_COL32_G_SHIFT) & 0xFF) - col0_g;
    let col_delta_b = ((col1 >> IM_COL32_B_SHIFT) & 0xFF) - col0_b;
    for (ImDrawVert* vert = vert_start; vert < vert_end; vert += 1)
    {
        let d =  ImDot(vert.pos - gradient_p0, gradient_extent);
        let t =  ImClamp(d * gradient_inv_length2, 0.0, 1.0);
        int r = (col0_r + col_delta_r * t);
        int g = (col0_g + col_delta_g * t);
        int b = (col0_b + col_delta_b * t);
        vert.col = (r << IM_COL32_R_SHIFT) | (g << IM_COL32_G_SHIFT) | (b << IM_COL32_B_SHIFT) | (vert.col & COLOR32_A_MASK);
    }
}

// Distribute UV over (a, b) rectangle
void ImGui::ShadeVertsLinearUV(ImDrawList* draw_list, int vert_start_idx, int vert_end_idx, const Vector2D& a, const Vector2D& b, const Vector2D& uv_a, const Vector2D& uv_b, bool clamp)
{
    const Vector2D size = b - a;
    const Vector2D uv_size = uv_b - uv_a;
    const Vector2D scale = Vector2D::new(
        size.x != 0.0 ? (uv_size.x / size.x) : 0.0,
        size.y != 0.0 ? (uv_size.y / size.y) : 0.0);

    ImDrawVert* vert_start = draw_list.vtx_buffer.data + vert_start_idx;
    ImDrawVert* vert_end = draw_list.vtx_buffer.data + vert_end_idx;
    if (clamp)
    {
        const Vector2D min = ImMin(uv_a, uv_b);
        const Vector2D max = ImMax(uv_a, uv_b);
        for (ImDrawVert* vertex = vert_start; vertex < vert_end;  += 1vertex)
            vertex.uv = ImClamp(uv_a + ImMul(Vector2D::new(vertex.pos.x, vertex.pos.y) - a, scale), min, max);
    }
    else
    {
        for (ImDrawVert* vertex = vert_start; vertex < vert_end;  += 1vertex)
            vertex.uv = uv_a + ImMul(Vector2D::new(vertex.pos.x, vertex.pos.y) - a, scale);
    }
}
