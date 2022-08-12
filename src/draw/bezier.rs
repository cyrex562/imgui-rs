use freetype::Vector;
use crate::geometry;
use crate::vectors::vector_2d::Vector2D;

// Vector2D ImBezierCubicCalc(const Vector2D& p1, const Vector2D& p2, const Vector2D& p3, const Vector2D& p4, float t)
pub fn bezier_cubic_calc(p1: &Vector2D, p2: &Vector2D, p3: &Vector2D, p4: &Vector2D, t: f32) -> Vector2D
{
    let u = 1.0 - t;
    let w1 = u * u * u;
    let w2 = 3 * u * u * t;
    let w3 = 3 * u * t * t;
    let w4 = t * t * t;
    return Vector2D::new(w1 * p1.x + w2 * p2.x + w3 * p3.x + w4 * p4.x, w1 * p1.y + w2 * p2.y + w3 * p3.y + w4 * p4.y);
}

// Vector2D ImBezierQuadraticCalc(const Vector2D& p1, const Vector2D& p2, const Vector2D& p3, float t)
pub fn bezier_quadratic_calc(p1: &Vector2D, p2: &Vector2D, p3: &Vector2D, t: f32) -> Vector2D
{
    let u = 1.0 - t;
    let w1 = u * u;
    let w2 = 2 * u * t;
    let w3 = t * t;
    return Vector2D::new(w1 * p1.x + w2 * p2.x + w3 * p3.x, w1 * p1.y + w2 * p2.y + w3 * p3.y);
}

// Closely mimics ImBezierCubicClosestPointCasteljau() in imgui.cpp
// static void path_bezier_cubic_curve_toCasteljau(ImVector<Vector2D>* path, float x1, float y1, float x2, float y2, float x3, float y3, float x4, float y4, float tess_tol, int level)
pub fn path_bezier_cubic_curve_to_casteljau(path: &Vec<Vector2D>, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, x4: f32, y4: f32, tess_tol: f32, level: i32)
{
    let dx = x4 - x1;
    let dy = y4 - y1;
    let mut d2 = (x2 - x4) * dy - (y2 - y4) * dx;
    let mut d3 = (x3 - x4) * dy - (y3 - y4) * dx;
    d2 = if d2 >= 0f32 { d2 }
    else { -d2 };
    d3 = if d3 >= 0f32 { d3 } else { -d3 };
    if (d2 + d3) * (d2 + d3) < tess_tol * (dx * dx + dy * dy)
    {
        path.push_back(Vector2D::new(x4, y4));
    }
    else if level < 10
    {
        let x12 = (x1 + x2) * 0.5;
        let y12 = (y1 + y2) * 0.5;
        let x23 =  (x2 + x3) * 0.5;
        let y23 = (y2 + y3) * 0.5;
        let x34 =  (x3 + x4) * 0.5;
        let y34 = (y3 + y4) * 0.5;
        let x123 =  (x12 + x23) * 0.5;
        let y123 = (y12 + y23) * 0.5;
        let x234 =  (x23 + x34) * 0.5;
        let y234 = (y23 + y34) * 0.5;
        let x1234 =  (x123 + x234) * 0.5;
        let y1234 = (y123 + y234) * 0.5;
        path_bezier_cubic_curve_to_casteljau(path, x1, y1, x12, y12, x123, y123, x1234, y1234, tess_tol, level + 1);
        path_bezier_cubic_curve_to_casteljau(path, x1234, y1234, x234, y234, x34, y34, x4, y4, tess_tol, level + 1);
    }
}

// static void path_bezier_quadratic_curve_to_casteljau(ImVector<Vector2D>* path, float x1, float y1, float x2, float y2, float x3, float y3, float tess_tol, int level)
pub fn path_bezier_quadratic_curve_to_casteljau(path: &Vec<Vector2D>, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, tess_tol: f32, level: i32)
{
    let dx =  x3 - x1;
    let dy = y3 - y1;
    let det =  (x2 - x3) * dy - (y2 - y3) * dx;
    if det * det * 4.0 < tess_tol * (dx * dx + dy * dy)
    {
        path.push_back(Vector2D::new(x3, y3));
    }
    else if level < 10
    {
        let x12 =  (x1 + x2) * 0.5;
        let y12 = (y1 + y2) * 0.5;
        let x23 =  (x2 + x3) * 0.5;
        let y23 = (y2 + y3) * 0.5;
        let x123 =  (x12 + x23) * 0.5;
        let y123 = (y12 + y23) * 0.5;
        path_bezier_quadratic_curve_to_casteljau(path, x1, y1, x12, y12, x123, y123, tess_tol, level + 1);
        path_bezier_quadratic_curve_to_casteljau(path, x123, y123, x23, y23, x3, y3, tess_tol, level + 1);
    }
}

// pub fn ImBezierCubicClosestPoint(const Vector2D& p1, const Vector2D& p2, const Vector2D& p3, const Vector2D& p4, const Vector2D& p, int num_segments) -> Vector2D
pub fn bezier_cubic_closest_point(
    p1: &Vector2D,
    p2: &Vector2D,
    p3: &Vector2D,
    p4: &Vector2D,
    p: &Vector2D,
    num_segments: usize,
) -> Vector2D {
    // IM_ASSERT(num_segments > 0); // Use ImBezierCubicClosestPointCasteljau()
    // Vector2D p_last = p1;
    let mut p_last = p1.clone();
    // Vector2D p_closest;
    let mut p_closest = Vector2D::new2();
    // float p_closest_dist2 = FLT_MAX;
    let mut p_closest_dist2: f32 = f32::MAX;
    // float t_step = 1.0 / (float)num_segments;
    let mut t_step = 1.0 / num_segments as f32;
    // for (int i_step = 1; i_step <= num_segments; i_step++)
    let mut i_step = 1;
    while i_step <= num_segments {
        // Vector2D p_current = ImBezierCubicCalc(p1, p2, p3, p4, t_step * i_step);
        let mut p_current = bezier_cubic_calc(p1, p2, p3, p4, t_step * i_step);
        // Vector2D p_line = ImLineClosestPoint(p_last, p_current, p);
        let mut p_line = geometry::line_closest_point(&p_last, &p_current, p);
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
// static void ImBezierCubicClosestPointCasteljauStep(const Vector2D& p, Vector2D& p_closest, Vector2D& p_last, float& p_closest_dist2, float x1, float y1, float x2, float y2, float x3, float y3, float x4, float y4, float tess_tol, int level)
pub fn bezier_cubic_closest_point_casteljau_step(
    p: &Vector2D,
    p_closest: &mut Vector2D,
    p_last: &mut Vector2D,
    p_closest_dist2: &mut f32,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    x3: f32,
    y3: f32,
    x4: f32,
    y4: f32,
    tess_tol: f32,
    level: i32,
) {
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
        // Vector2D p_current(x4, y4);
        let mut p_current = Vector2D::new(x4, y4);
        // Vector2D p_line = ImLineClosestPoint(p_last, p_current, p);
        let mut p_line = geometry::line_closest_point(p_last, &p_current, p);
        // float dist2 = ImLengthSqr(p - p_line);
        let mut dist2 = ImLengthSqr(p - p_line);
        if dist2 < *p_closest_dist2 {
            *p_closest = p_line.clone();
            *p_closest_dist2 = dist2;
        }
        *p_last = p_current.clone();
    } else if level < 10 {
        // float x12 = (x1 + x2)*0.5,       y12 = (y1 + y2)*0.5;
        let mut x12 = (x1 + x2) * 0.5;
        let mut y12 = (y1 + y2) * 0.5;
        // float x23 = (x2 + x3)*0.5,       y23 = (y2 + y3)*0.5;
        let mut x23 = (x2 + x3) * 0.5;
        let mut y23 = (y2 + y3) * 0.5;
        // float x34 = (x3 + x4)*0.5,       y34 = (y3 + y4)*0.5;
        let mut x34 = (x3 + x4) * 0.5;
        let mut y34 = (y3 + y4) * 0.5;
        // float x123 = (x12 + x23)*0.5,    y123 = (y12 + y23)*0.5;
        let mut x123 = (x12 + x23) * 0.5;
        let mut y123 = (y12 + y23) * 0.5;
        // float x234 = (x23 + x34)*0.5,    y234 = (y23 + y34)*0.5;
        let mut x234 = (x23 + x34) * 0.5;
        let mut y234 = (y23 + y34) * 0.5;
        // float x1234 = (x123 + x234)*0.5, y1234 = (y123 + y234)*0.5;
        let mut x1234 = (x123 + x234) * 0.5;
        let mut y1234 = (y123 + y234) * 0.5;
        bezier_cubic_closest_point_casteljau_step(
            p,
            p_closest,
            p_last,
            p_closest_dist2,
            x1,
            y1,
            x12,
            y12,
            x123,
            y123,
            x1234,
            y1234,
            tess_tol,
            level + 1,
        );
        bezier_cubic_closest_point_casteljau_step(
            p,
            p_closest,
            p_last,
            p_closest_dist2,
            x1234,
            y1234,
            x234,
            y234,
            x34,
            y34,
            x4,
            y4,
            tess_tol,
            level + 1,
        );
    }
}

// tess_tol is generally the same value you would find in ImGui::GetStyle().CurveTessellationTol
// Because those ImXXX functions are lower-level than ImGui:: we cannot access this value automatically.
// Vector2D ImBezierCubicClosestPointCasteljau(const Vector2D& p1, const Vector2D& p2, const Vector2D& p3, const Vector2D& p4, const Vector2D& p, float tess_tol)
pub fn bezier_cubic_closest_point_casteljau(
    p1: &Vector2D,
    p2: &Vector2D,
    p3: &Vector2D,
    p4: &Vector2D,
    p: &Vector2D,
    tess_tol: f32,
) -> Vector2D {
    // IM_ASSERT(tess_tol > 0.0);
    // Vector2D p_last = p1;
    let mut p_last = p1.clone();
    // Vector2D p_closest;
    let mut p_closest = Vector2D::new2();
    // float p_closest_dist2 = FLT_MAX;
    let mut p_closest_dist2: f32 = f32::MAX;
    bezier_cubic_closest_point_casteljau_step(
        p,
        &mut p_closest,
        &mut p_last,
        &mut p_closest_dist2,
        p1.x,
        p1.y,
        p2.x,
        p2.y,
        p3.x,
        p3.y,
        p4.x,
        p4.y,
        tess_tol,
        0,
    );
    return p_closest;
}
