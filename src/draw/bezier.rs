use crate::vectors::two_d::Vector2D;

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
// static void PathBezierCubicCurveToCasteljau(ImVector<Vector2D>* path, float x1, float y1, float x2, float y2, float x3, float y3, float x4, float y4, float tess_tol, int level)
pub fn path_bezier_cubic_curve_to_casteljau(path: &Vec<Vector2D>, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, x4: f32, y4: f32, tess_tol: f32, level: i32)
{
    let dx = x4 - x1;
    let dy = y4 - y1;
    let mut d2 = (x2 - x4) * dy - (y2 - y4) * dx;
    let mut d3 = (x3 - x4) * dy - (y3 - y4) * dx;
    d2 = if d2 >= 0 { d2 }
    else { -d2 };
    d3 = if d3 >= 0 { d3 } else { -d3 };
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

static void PathBezierQuadraticCurveToCasteljau(ImVector<Vector2D>* path, float x1, float y1, float x2, float y2, float x3, float y3, float tess_tol, int level)
{
    let dx =  x3 - x1, dy = y3 - y1;
    let det =  (x2 - x3) * dy - (y2 - y3) * dx;
    if (det * det * 4.0 < tess_tol * (dx * dx + dy * dy))
    {
        path.push_back(Vector2D::new(x3, y3));
    }
    else if (level < 10)
    {
        let x12 =  (x1 + x2) * 0.5, y12 = (y1 + y2) * 0.5;
        let x23 =  (x2 + x3) * 0.5, y23 = (y2 + y3) * 0.5;
        let x123 =  (x12 + x23) * 0.5, y123 = (y12 + y23) * 0.5;
        PathBezierQuadraticCurveToCasteljau(path, x1, y1, x12, y12, x123, y123, tess_tol, level + 1);
        PathBezierQuadraticCurveToCasteljau(path, x123, y123, x23, y23, x3, y3, tess_tol, level + 1);
    }
}
