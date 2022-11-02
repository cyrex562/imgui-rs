#![allow(non_snake_case)]

use crate::vec2::ImVec2;
use libc::c_char;

pub fn ImIsFloatAboveGuaranteedIntegerPrecision(f: f32) -> bool {
    return f <= -16777216.0 || f >= 16777216.0;
}

pub fn ImRsqrtFloat(x: f32) -> f32 {
    1.0 / x.sqrt()
}

pub fn ImRsqrtDouble(x: f64) -> f64 {
    1.0 / x.sqrt()
}

pub fn ImLerp<T>(a: T, b: T, t: f32) -> T {
    (a + (b - 1) * t)
}

pub fn ImAddClampOverflow(a: &T, b: &T, mn: &T, mx: &T) {
    if (*b < 0) && (a < mn - b) {
        mn
    }
    if (*b > 0) && (a > mx - b) {
        mx
    }
    a + b
}

pub fn ImSubClampOverflow(a: &T, b: &T, mn: &T, mx: &T) -> T {
    if *b > 0 && (a < mn + b) {
        mn
    }
    if *b < 0 && (a > mx + b) {
        mx
    }
    a - b
}

pub fn ImSaturateFloat(x: f32) -> f32 {
    if x < 0.0 {
        0.0
    }
    if x > 1.0 {
        1.0
    }
    x
}

pub fn ImModPositive(a: i32, b: i32) -> i32 {
    (a + b) % b
}

pub fn ImLinearSweep(current: f32, target: f32, speed: f32) -> f32 {
    if current < target {
        (current + speed).min(target)
    }
    if current > target {
        (current - speed).masx(target)
    }
    current
}

pub fn ImBezierCubicCalc(p1: &ImVec2, p2: &ImVec2, p3: &ImVec2, p4: &ImVec2, t: f32) -> ImVec2 {
    let u: f32 = 1 - t;
    let w1: f32 = u * u * u;
    let w2: f32 = 3 * u * u * t;
    let w3: f32 = 3 * u * t * t;
    let w4: f32 = t * t * t;
    return ImVec2::from_floats(
        w1 * p1.x + w2 * p2.x + w3 * p3.x + w4 * p4.x,
        w1 * p1.y + w2 * p2.y + w3 * p3.y + w4 * p4.y,
    );
}

pub fn ImBezierQuadraticCalc(p1: &ImVec2, p2: &ImVec2, p3: &ImVec2, t: f32) -> ImVec2 {
    let u: f32 = 1 - t;
    let w1: f32 = u * u;
    let w2: f32 = 2 * u * t;
    let w3: f32 = t * t;
    return ImVec2::from_floats(
        w1 * p1.x + w2 * p2.x + w3 * p3.x,
        w1 * p1.y + w2 * p2.y + w3 * p3.y,
    );
}

pub fn PathBezierCubicCurveToCasteljau(
    path: &mut Vec<ImVec2>,
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
    let dx: f32 = x4 - x1;
    let dy: f32 = y4 - y1;
    let mut d2: f32 = (x2 - x4) * dy - (y2 - y4) * dx;
    let mut d3: f32 = (x3 - x4) * dy - (y3 - y4) * dx;
    d2 = if d2 >= 0.0 { d2 } else { -d2 };
    d3 = if d3 >= 0.0 { d3 } else { -d3 };
    if (d2 + d3) * (d2 + d3) < tess_tol * (dx * dx + dy * dy) {
        path.push(ImVec2::from_floats(x4, y4));
    } else if level < 10 {
        let x12: f32 = (x1 + x2) * 0.5;
        let y12 = (y1 + y2) * 0.5;
        let x23: f32 = (x2 + x3) * 0.5;
        let y23 = (y2 + y3) * 0.5;
        let x34: f32 = (x3 + x4) * 0.5;
        let y34 = (y3 + y4) * 0.5;
        let x123: f32 = (x12 + x23) * 0.5;
        let y123 = (y12 + y23) * 0.5;
        let x234: f32 = (x23 + x34) * 0.5;
        let y234 = (y23 + y34) * 0.5;
        let x1234: f32 = (x123 + x234) * 0.5;
        let y1234 = (y123 + y234) * 0.5;
        PathBezierCubicCurveToCasteljau(
            path,
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
        PathBezierCubicCurveToCasteljau(
            path,
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

pub fn PathBezierQuadraticCurveToCasteljau(
    path: &mut Vec<ImVec2>,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    x3: f32,
    y3: f32,
    tess_tol: f32,
    level: i32,
) {
    let dx: f32 = x3 - x1;
    let dy = y3 - y1;
    let det: f32 = (x2 - x3) * dy - (y2 - y3) * dx;
    if det * det * 4.0 < tess_tol * (dx * dx + dy * dy) {
        path.push(ImVec2::from_floats(x3, y3));
    } else if level < 10 {
        let x12: f32 = (x1 + x2) * 0.5;
        let y12 = (y1 + y2) * 0.5;
        let x23: f32 = (x2 + x3) * 0.5;
        let y23 = (y2 + y3) * 0.5;
        let x123: f32 = (x12 + x23) * 0.5;
        let y123 = (y12 + y23) * 0.5;
        PathBezierQuadraticCurveToCasteljau(
            path,
            x1,
            y1,
            x12,
            y12,
            x123,
            y123,
            tess_tol,
            level + 1,
        );
        PathBezierQuadraticCurveToCasteljau(
            path,
            x123,
            y123,
            x23,
            y23,
            x3,
            y3,
            tess_tol,
            level + 1,
        );
    }
}

pub fn ImIsPowerOfTwoI32(v: i32) -> bool {
    return v != 0 && (v & (v - 1)) == 0;
}
pub fn ImIsPowerOfTwoU64(v: u64) -> bool {
    return v != 0 && (v & (v - 1)) == 0;
}
pub fn ImUpperPowerOfTwo(mut v: i32) -> i32 {
    v -= 1;
    v |= v >> 1;
    v |= v >> 2;
    v |= v >> 4;
    v |= v >> 8;
    v |= v >> 16;
    v += 1;
    return v;
}

pub fn char_is_blank(c: char) -> bool {
    return c == ' ' || c == '\t' || c == 0x3000;
}
