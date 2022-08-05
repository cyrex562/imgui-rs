use crate::draw::bezier;
use crate::draw::bezier::bezier_cubic_calc;
use crate::imgui_h::Vector2D;
use crate::imgui_vec::{ImLengthSqr, Vector2D};
use crate::vectors::Vector2D;

// Vector2D ImLineClosestPoint(const Vector2D& a, const Vector2D& b, const Vector2D& p)
pub fn line_closest_point(a: &Vector2D, b: &Vector2D, p: &Vector2D) -> Vector2D {
    // Vector2D ap = p - a;
    let mut ap: Vector2D = p - a;
    // Vector2D ab_dir = b - a;
    let mut ab_dir: Vector2D = b - a;
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

// bool ImTriangleContainsPoint(const Vector2D& a, const Vector2D& b, const Vector2D& c, const Vector2D& p)
pub fn triangle_contains_point(a: &Vector2D, b: &Vector2D, c: &Vector2D, p: &Vector2D) -> bool {
    // bool b1 = ((p.x - b.x) * (a.y - b.y) - (p.y - b.y) * (a.x - b.x)) < 0.0;
    let b1 = ((p.x - b.x) * (a.y - b.y) - (p.y - b.y) * (a.x - b.x)) < 0.0;
    // bool b2 = ((p.x - c.x) * (b.y - c.y) - (p.y - c.y) * (b.x - c.x)) < 0.0;
    let b2 = ((p.x - c.x) * (b.y - c.y) - (p.y - c.y) * (b.x - c.x)) < 0.0;
    // bool b3 = ((p.x - a.x) * (c.y - a.y) - (p.y - a.y) * (c.x - a.x)) < 0.0;
    let b3 = ((p.x - a.x) * (c.y - a.y) - (p.y - a.y) * (c.x - a.x)) < 0.0;
    return (b1 == b2) && (b2 == b3);
}

// void ImTriangleBarycentricCoords(const Vector2D& a, const Vector2D& b, const Vector2D& c, const Vector2D& p, float& out_u, float& out_v, float& out_w)
pub fn triangle_barycentric_coords(
    a: &Vector2D,
    b: &Vector2D,
    c: &Vector2D,
    p: &Vector2D,
    out_u: &mut f32,
    out_v: &mut f32,
    out_w: &mut f32,
) {
    // Vector2D v0 = b - a;
    let mut v0: Vector2D = b - a;
    // Vector2D v1 = c - a;
    let mut v1: Vector2D = c - a;
    // Vector2D v2 = p - a;
    let mut v2 = p - a;
    // let denom = v0.x * v1.y - v1.x * v0.y;
    let denom = v0.x * v1.y - v1.x + v0.y;
    *out_v = (v2.x * v1.y - v1.x * v2.y) / denom;
    *out_w = (v0.x * v2.y - v2.x * v0.y) / denom;
    *out_u = 1.0 - out_v - out_w;
}

// Vector2D ImTriangleClosestPoint(const Vector2D& a, const Vector2D& b, const Vector2D& c, const Vector2D& p)
pub fn triangle_closest_point(a: &Vector2D, b: &Vector2D, c: &Vector2D, p: &Vector2D) -> Vector2D {
    // Vector2D proj_ab = ImLineClosestPoint(a, b, p);
    let mut proj_ab = line_closest_point(a, b, p);
    // Vector2D proj_bc = ImLineClosestPoint(b, c, p);
    let mut proj_bc = line_closest_point(b, c, p);
    // Vector2D proj_ca = ImLineClosestPoint(c, a, p);
    let mut proj_ca = line_closest_point(c, a, p);
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

// //  Vector2D     ImBezierCubicCalc(const Vector2D& p1, const Vector2D& p2, const Vector2D& p3, const Vector2D& p4, float t);
// //  Vector2D     ImBezierCubicClosestPoint(const Vector2D& p1, const Vector2D& p2, const Vector2D& p3, const Vector2D& p4, const Vector2D& p, int num_segments);       // For curves with explicit number of segments
// //  Vector2D     ImBezierCubicClosestPointCasteljau(const Vector2D& p1, const Vector2D& p2, const Vector2D& p3, const Vector2D& p4, const Vector2D& p, float tess_tol);// For auto-tessellated curves you can use tess_tol = style.CurveTessellationTol
// //  Vector2D     ImBezierQuadraticCalc(const Vector2D& p1, const Vector2D& p2, const Vector2D& p3, float t);
// //  Vector2D     ImLineClosestPoint(const Vector2D& a, const Vector2D& b, const Vector2D& p);
// //  bool       ImTriangleContainsPoint(const Vector2D& a, const Vector2D& b, const Vector2D& c, const Vector2D& p);
// //  Vector2D     ImTriangleClosestPoint(const Vector2D& a, const Vector2D& b, const Vector2D& c, const Vector2D& p);
// //  void       ImTriangleBarycentricCoords(const Vector2D& a, const Vector2D& b, const Vector2D& c, const Vector2D& p, float& out_u, float& out_v, float& out_w);
// // inline float         ImTriangleArea(const Vector2D& a, const Vector2D& b, const Vector2D& c) { return f32::abs((a.x * (b.y - c.y)) + (b.x * (c.y - a.y)) + (c.x * (a.y - b.y))) * 0.5; }
// //  ImGuiDir   get_dir_quadrant_from_delta(float dx, float dy);
