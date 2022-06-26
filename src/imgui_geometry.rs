use crate::imgui_h::ImVec2;
use crate::imgui_vec2::{ImLengthSqr, ImVec2};

// pub fn ImBezierCubicClosestPoint(const ImVec2& p1, const ImVec2& p2, const ImVec2& p3, const ImVec2& p4, const ImVec2& p, int num_segments) -> ImVec2
pub fn ImBezierCubicClosestPoint(p1: &ImVec2, p2: &ImVec2, p3: &ImVec2, p4: &ImVec2, p: &ImVec2, num_segments: usize) -> ImVec2 {
    // IM_ASSERT(num_segments > 0); // Use ImBezierCubicClosestPointCasteljau()
    // ImVec2 p_last = p1;
    let mut p_last = p1.clone();
    // ImVec2 p_closest;
    let mut p_closest = ImVec2::new2();
    // float p_closest_dist2 = FLT_MAX;
    let mut p_closest_dist2: f32 = f32::MAX;
    // float t_step = 1.0f / (float)num_segments;
    let mut t_step = 1.0 / num_segments as f32;
    // for (int i_step = 1; i_step <= num_segments; i_step++)
    let mut i_step = 1;
    while i_step <= num_segments {
        // ImVec2 p_current = ImBezierCubicCalc(p1, p2, p3, p4, t_step * i_step);
        let mut p_current = ImBezierCubicCalc(p1, p2, p3, p4, t_step * i_step);
        // ImVec2 p_line = ImLineClosestPoint(p_last, p_current, p);
        let mut p_line = ImLineClosestPoint(&p_last, p_current, p);
        // float dist2 = ImLengthSqr(p - p_line);
        let mut dist2 = ImLengthSqr(p - p_line);
        if dist2 < p_closest_dist2 {
            p_closest = p_line.clone();
            p_closest_dist2 = dist2;
        }
        p_last = p_current;
        i_step += 1;
    }
    return p_closest;
}

// Closely mimics PathBezierToCasteljau() in imgui_draw.cpp
// static void ImBezierCubicClosestPointCasteljauStep(const ImVec2& p, ImVec2& p_closest, ImVec2& p_last, float& p_closest_dist2, float x1, float y1, float x2, float y2, float x3, float y3, float x4, float y4, float tess_tol, int level)
pub fn ImBezierCubicClosestPointCasteljauStep(p: &ImVec2, p_closest: &mut ImVec2, p_last: &mut ImVec2, p_closest_dist2: &mut f32, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, x4: f32, y4: f32, tess_tol: f32, level: i32) {
    // float dx = x4 - x1;
    let mut dx = x4 - x1;
    // float dy = y4 - y1;
    let mut dy = y4 - y1;
    // float d2 = ((x2 - x4) * dy - (y2 - y4) * dx);
    let mut d2 = (x2 - x4) * dy - (y2 - y4) * dx;
    // float d3 = ((x3 - x4) * dy - (y3 - y4) * dx);
    let mut d3 = (x3 - x4) * dy - (y3 - y4) * dx;
    // d2 = (d2 >= 0) ? d2 : -d2;
    d2 = if d2 >= 0.0 { d2 } else { d2 * -1 };
    // d3 = (d3 >= 0) ? d3 : -d3;
    if (d2 + d3) * (d2 + d3) < tess_tol * (dx * dx + dy * dy) {
        // ImVec2 p_current(x4, y4);
        let mut p_current = ImVec2::new(x4, y4);
        // ImVec2 p_line = ImLineClosestPoint(p_last, p_current, p);
        let mut p_line = ImLineClosestPoint(p_last, &p_current, p);
        // float dist2 = ImLengthSqr(p - p_line);
        let mut dist2 = ImLengthSqr(p - p_line);
        if dist2 < *p_closest_dist2 {
            *p_closest = p_line.clone();
            *p_closest_dist2 = dist2;
        }
        *p_last = p_current.clone();
    } else if level < 10 {
        // float x12 = (x1 + x2)*0.5f,       y12 = (y1 + y2)*0.5f;
        let mut x12 = (x1 + x2) * 0.5;
        let mut y12 = (y1 + y2) * 0.5;
        // float x23 = (x2 + x3)*0.5f,       y23 = (y2 + y3)*0.5f;
        let mut x23 = (x2 + x3) * 0.5;
        let mut y23 = (y2 + y3) * 0.5;
        // float x34 = (x3 + x4)*0.5f,       y34 = (y3 + y4)*0.5f;
        let mut x34 = (x3 + x4) * 0.5;
        let mut y34 = (y3 + y4) * 0.5;
        // float x123 = (x12 + x23)*0.5f,    y123 = (y12 + y23)*0.5f;
        let mut x123 = (x12 + x23) * 0.5;
        let mut y123 = (y12 + y23) * 0.5;
        // float x234 = (x23 + x34)*0.5f,    y234 = (y23 + y34)*0.5f;
        let mut x234 = (x23 + x34) * 0.5;
        let mut y234 = (y23 + y34) * 0.5;
        // float x1234 = (x123 + x234)*0.5f, y1234 = (y123 + y234)*0.5f;
        let mut x1234 = (x123 + x234) * 0.5;
        let mut y1234 = (y123 + y234) * 0.5;
        ImBezierCubicClosestPointCasteljauStep(p, p_closest, p_last, p_closest_dist2, x1, y1, x12, y12, x123, y123, x1234, y1234, tess_tol, level + 1);
        ImBezierCubicClosestPointCasteljauStep(p, p_closest, p_last, p_closest_dist2, x1234, y1234, x234, y234, x34, y34, x4, y4, tess_tol, level + 1);
    }
}

// tess_tol is generally the same value you would find in ImGui::GetStyle().CurveTessellationTol
// Because those ImXXX functions are lower-level than ImGui:: we cannot access this value automatically.
// ImVec2 ImBezierCubicClosestPointCasteljau(const ImVec2& p1, const ImVec2& p2, const ImVec2& p3, const ImVec2& p4, const ImVec2& p, float tess_tol)
pub fn ImBezierCubicClosestPointCasteljau(p1: &ImVec2, p2: &ImVec2, p3: &ImVec2, p4: &ImVec2, p: &ImVec2, tess_tol: f32) -> ImVec2 {
    // IM_ASSERT(tess_tol > 0.0f);
    // ImVec2 p_last = p1;
    let mut p_last = p1.clone();
    // ImVec2 p_closest;
    let mut p_closest = ImVec2::new2();
    // float p_closest_dist2 = FLT_MAX;
    let mut p_closest_dist2: f32 = f32::MAX;
    ImBezierCubicClosestPointCasteljauStep(p, &mut p_closest, &mut p_last, &mut p_closest_dist2, p1.x, p1.y, p2.x, p2.y, p3.x, p3.y, p4.x, p4.y, tess_tol, 0);
    return p_closest;
}

// ImVec2 ImLineClosestPoint(const ImVec2& a, const ImVec2& b, const ImVec2& p)
pub fn ImLineClosestPoint(a: &ImVec2, b: &ImVec2, p: &ImVec2) -> ImVec2 {
    // ImVec2 ap = p - a;
    let mut ap: ImVec2 = p - a;
    // ImVec2 ab_dir = b - a;
    let mut ab_dir: ImVec2 = b - a;
    // float dot = ap.x * ab_dir.x + ap.y * ab_dir.y;
    let mut dot = ap.x * ab_dir.x + ap.y * ab_dir.y;
    if dot < 0.0 {
        return a.clone();
    }
    // float ab_len_sqr = ab_dir.x * ab_dir.x + ab_dir.y * ab_dir.y;
    let mut ab_len_sqr = ab_dir.x * ab_dir.x + ab_dir.y * ab_dir.y;
    if (dot > ab_len_sqr) {
        return b.clone();
    }
    return a + ab_dir * dot / ab_len_sqr;
}

// bool ImTriangleContainsPoint(const ImVec2& a, const ImVec2& b, const ImVec2& c, const ImVec2& p)
pub fn ImTriangleContainsPoint(a: &ImVec2, b: &ImVec2, c: &ImVec2, p: &ImVec2) -> bool {
    // bool b1 = ((p.x - b.x) * (a.y - b.y) - (p.y - b.y) * (a.x - b.x)) < 0.0f;
    let b1 = ((p.x - b.x) * (a.y - b.y) - (p.y - b.y) * (a.x - b.x)) < 0.0;
    // bool b2 = ((p.x - c.x) * (b.y - c.y) - (p.y - c.y) * (b.x - c.x)) < 0.0f;
    let b2 = ((p.x - c.x) * (b.y - c.y) - (p.y - c.y) * (b.x - c.x)) < 0.0;
    // bool b3 = ((p.x - a.x) * (c.y - a.y) - (p.y - a.y) * (c.x - a.x)) < 0.0f;
    let b3 = ((p.x - a.x) * (c.y - a.y) - (p.y - a.y) * (c.x - a.x)) < 0.0;
    return (b1 == b2) && (b2 == b3);
}

// void ImTriangleBarycentricCoords(const ImVec2& a, const ImVec2& b, const ImVec2& c, const ImVec2& p, float& out_u, float& out_v, float& out_w)
pub fn ImTriangleBarycentricCoords(a: &ImVec2, b: &ImVec2, c: &ImVec2, p: &ImVec2, out_u: &mut f32, out_v: &mut f32, out_w: &mut f32) {
    // ImVec2 v0 = b - a;
    let mut v0: ImVec2 = b - a;
    // ImVec2 v1 = c - a;
    let mut v1: ImVec2 = c - a;
    // ImVec2 v2 = p - a;
    let mut v2 = p - a;
    // const float denom = v0.x * v1.y - v1.x * v0.y;
    let denom = v0.x * v1.y - v1.x + v0.y;
    *out_v = (v2.x * v1.y - v1.x * v2.y) / denom;
    *out_w = (v0.x * v2.y - v2.x * v0.y) / denom;
    *out_u = 1.0 - out_v - out_w;
}

// ImVec2 ImTriangleClosestPoint(const ImVec2& a, const ImVec2& b, const ImVec2& c, const ImVec2& p)
pub fn ImTriangleClosestPoint(a: &ImVec2, b: &ImVec2, c: &ImVec2, p: &ImVec2) -> ImVec2 {
    // ImVec2 proj_ab = ImLineClosestPoint(a, b, p);
    let mut proj_ab = ImLineClosestPoint(a, b, p);
    // ImVec2 proj_bc = ImLineClosestPoint(b, c, p);
    let mut proj_bc = ImLineClosestPoint(b, c, p);
    // ImVec2 proj_ca = ImLineClosestPoint(c, a, p);
    let mut proj_ca = ImLineClosestPoint(c, a, p);
    // float dist2_ab = ImLengthSqr(p - proj_ab);
    let mut dist2_ab = ImLengthSqr(p - proj_ab);
    // float dist2_bc = ImLengthSqr(p - proj_bc);
    let mut dist2_bc = ImLengthSqr(p - proj_bc);
    // float dist2_ca = ImLengthSqr(p - proj_ca);
    let mut dist2_ca = ImLengthSqr(p - proj_ca);
    // float m = ImMin(dist2_ab, ImMin(dist2_bc, dist2_ca));
    let mut m = f32::min(dist2_ab, f32::min(dist2_bc, dist2_ca));
    if m == dist2_ab {
        return proj_ab.clone();
    }
    if m == dist2_bc {
        return proj_bc.clone();
    }
    return proj_ca.clone();
}