#![allow(non_snake_case)]

use std::str::FromStr;
use libc::{c_double, c_float, c_int};
use crate::vec2::ImVec2;
use crate::vec4::ImVec4;

// static inline bool   ImIsFloatAboveGuaranteedIntegerPrecision(c_float 0.0)
pub fn ImIsFloatAboveGuaranteedIntegerPrecision(f: c_loat) -> bool {
    return f <= -16777216 || f >= 16777216;
}


// - Wrapper for standard libs functions. (Note that imgui_demo.cpp does _not_ use them to keep the code easy to copy)
// #ifndef IMGUI_DISABLE_DEFAULT_MATH_FUNCTIONS
// #define ImFabs(X)           fabsf(X)
pub fn ImFabs(x: c_float) -> c_float {
    x.abs()
}

// #define ImSqrt(X)           sqrtf(X)
pub fn ImSqrt(x: c_float) -> c_float {
    x.sqrt()
}

// #define ImFmod(X, Y)        fmodf((X), (Y))
pub fn ImFmod(x: c_float, y: c_float) -> c_float {
    x % y
}

// #define ImCos(X)            cosf(X)
pub fn ImCos(x: c_float) -> c_float {
    x.cos()
}

// #define ImSin(X)            sinf(X)
pub fn ImSin(x: c_float) -> c_float {
    x.sin()
}

// #define ImAcos(X)           acosf(X)
pub fn ImAcos(x: c_float) -> c_float {
    x.acos()
}

// #define ImAtan2(Y, X)       atan2f((Y), (X))
pub fn ImAtan2(y: c_float, x: c_float) -> c_float {
    y.atan2(x)
}


// #define ImAtof(STR)         atof(STR)
pub fn ImAtof(input: &str) -> f32 {
    f32::from_str(input).unwrap()
}

//#define ImFloorStd(X)     floorf(X)           // We use our own, see ImFloor() and ImFloorSigned()
pub fn ImFloorStd(x: c_float) -> c_float {
    x.floor()
}

// #define ImCeil(X)           ceilf(X)
pub fn ImCeil(x: c_float) -> c_float {
    x.ceil()
}

// static inline c_float  ImPow(c_float x, c_float y)    { return powf(x, y); }          // DragBehaviorT/SliderBehaviorT uses ImPow with either float/double and need the precision
pub fn ImPowFloat(x: c_float, y: c_float) -> f32 {
    x.powf(y)
}


// static inline double ImPow(double x, double y)  { return pow(x, y); }
pub fn ImPowDouble(x: c_double, y: c_double) -> c_double {
    x.pow(y)
}


// static inline c_float  ImLog(c_float x)             { return logf(x); }             // DragBehaviorT/SliderBehaviorT uses ImLog with either float/double and need the precision
pub fn ImLogFloat(x: c_float) -> c_float {
    x.log2()
}

// static inline double ImLog(double x)            { return log(x); }
pub fn ImLogDouble(x: c_double) -> c_double {
    x.log2()
}


// static inline c_int    ImAbs(x: c_int)               { return x < 0 ? -x : x; }
pub fn ImAbsInt(x: c_int) -> c_int {
    x.abs()
}

// static inline c_float  ImAbs(c_float x)             { return fabsf(x); }
pub fn ImAbsFloat(x: c_float) -> c_float {
    x.abs()
}

// static inline double ImAbs(double x)            { return fabs(x); }
pub fn ImAbsDouble(x: c_double) -> c_double {
    x.abs()
}

// static inline c_float  ImSign(c_float x)            { return (x < 0.0) ? -1.0 : (x > 0.0) ? 1.0 : 0.0; } // Sign operator - returns -1, 0 or 1 based on sign of argument
pub fn ImSignFloat(x: c_float) -> c_float {
    x.signum()
}


// static inline double ImSign(double x)           { return (x < 0.0) ? -1.0 : (x > 0.0) ? 1.0 : 0.0; }
pub fn ImSignDouble(x: c_double) -> c_double {
    x.signum()
}

// #ifdef IMGUI_ENABLE_SSE
// static inline c_float  ImRsqrt(c_float x)           { return _mm_cvtss_f32(_mm_rsqrt_ss(_mm_set_ss(x))); }
pub fn ImRsqrtFloat(x: c_float) -> c_float {
    1.0 / x.sqrt()
}

// #else
// static inline c_float  ImRsqrt(c_float x)           { return 1.0 / sqrtf(x); }
// #endif
// static inline double ImRsqrt(double x)          { return 1.0 / sqrt(x); }
pub fn ImRsqrtDouble(x: c_double) -> c_double {
    1.0 / x.sqrt()
}


// #endif
// - ImMin/ImMax/ImClamp/ImLerp/ImSwap are used by widgets which support variety of types: signed/unsigned int/long long float/double
// (Exceptionally using templates here but we could also redefine them for those types)
// template<typename T> static inline T ImMin(T lhs, T rhs)                        { return lhs < rhs ? lhs : rhs; }
pub fn ImMin<T>(lhs: T, rhs: T) -> T {
    T::min(lhs, rhs)
}


// template<typename T> static inline T ImMax(T lhs, T rhs)                        { return lhs >= rhs ? lhs : rhs; }
pub fn ImMax<T>(lhs: T, rhs: T) -> T {
    T::max(lhs, rhs)
}


// template<typename T> static inline T ImClamp(T v, T mn, T mx)                   { return (v < mn) ? mn : (v > mx) ? mx : v; }
pub fn ImClamp<T>(v: T, mn: T, mx: T) -> T {
    if v < mn { mn }
    if v > mx { mx }
    v
}


// template<typename T> static inline T ImLerp(T a, T b, c_float t)                  { return (T)(a + (b - a) * t); }
pub fn ImLerp<T>(a: T, b: T, t: c_float) -> T {
    (a + (b - 1) * t)
}


// template<typename T> static inline c_void ImSwap(T& a, T& b)                      { T tmp = a; a = b; b = tmp; }
pub fn ImSwap<T>(a: &mut T, b: &mut T) {
    let mut tmp = *a.clone();
    *a = *b.clone();
    *b = tmp.clone();
}


// template<typename T> static inline T ImAddClampOverflow(T a, T b, T mn, T mx)   { if (b < 0 && (a < mn - b)) return mn; if (b > 0 && (a > mx - b)) return mx; return a + b; }
pub fn ImAddClampOverflow(a: T, b: T, mn: T, mx: T) {
    if (b < 0) && (a < mn - b) {
        mn
    }
    if (b > 0) && (a > mx - b) {
        mx
    }
    a + b
}


// template<typename T> static inline T ImSubClampOverflow(T a, T b, T mn, T mx)   { if (b > 0 && (a < mn + b)) return mn; if (b < 0 && (a > mx + b)) return mx; return a - b; }
pub fn ImSubClampOverflow(a: T, b: T, mn: T, mx: T) -> T {
    if b > 0  && (a < mn + b) {
        mn
    }
    if b < 0 && (a > mx + b) {
        mx
    }
    a - b
}


// - Misc maths helpers
// static inline ImMin: ImVec2(lhs: &ImVec2, rhs: &ImVec2)                { return ImVec2::new(lhs.x < rhs.x ? lhs.x : rhs.x, lhs.y < rhs.y ? lhs.y : rhs.y); }
pub fn ImMinVec2(lhs: &mut ImVec2, rhs: &mut ImVec2) -> ImVec2 {
    let x = if lhs.x < rhs.x {
        lhs.x
    } else {
        rhs.x
    };
    let y = if lhs.y < rhs.y {
        lhs.y
    } else {
        rhs.y
    };
    ImVec2::new(x, y)
}


// static inline ImMax: ImVec2(lhs: &ImVec2, rhs: &ImVec2)                { return ImVec2::new(lhs.x >= rhs.x ? lhs.x : rhs.x, lhs.y >= rhs.y ? lhs.y : rhs.y); }
pub fn ImMaxVec2(lhs: &mut ImVec2, rhs: &mut ImVec2) -> ImVec2 {
    let x = if lhs.x >= rhs.x {
        lhs.x
    } else {
        rhs.x
    };
    let y = if lhs.y >= rhs.y {
        lhs.y
    } else {
        rhs.y
    };
    ImVec2::new(x, y)
}


// static inline ImClamp: ImVec2(v: &ImVec2, mn: &ImVec2, mx: ImVec2)      { return ImVec2::new((v.x < mn.x) ? mn.x : (v.x > mx.x) ? mx.x : v.x, (v.y < mn.y) ? mn.y : (v.y > mx.y) ? mx.y : v.y); }
pub fn ImClampVec2(v: &ImVec2, mn: &ImVec2, mx: &ImVec2) -> ImVec2 {
    let x = if v.x < mn.x {
        mn.x
    } else if v.x > mx.x {
        mx.x
    } else {
        v.x
    };
    let y = if v.y < mn.y {
        mn.y
    } else if v.y > mx.y {
        mn.y
    } else {
        v.y
    };
    ImVec2::new(x, y)
}


// static inline ImLerp: ImVec2(a: &ImVec2, b: &ImVec2, c_float t)          { return ImVec2::new(a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t); }
pub fn ImLerpVec2(a: &ImVec2, b: &ImVec2, t: c_float) -> ImVec2 {
    let x = a.x + (b.x - a.x) * t;
    let y = a.y + (b.y - a.y) * t;
    ImVec2::new(x, y)
}


// static inline ImLerp: ImVec2(a: &ImVec2, b: &ImVec2, t: &ImVec2)  { return ImVec2::new(a.x + (b.x - a.x) * t.x, a.y + (b.y - a.y) * t.y); }
pub fn ImLerpVec22(a: &ImVec2, b: &ImVec2, t: &ImVec2) -> ImVec2 {
    let x = a.x  + (b.x - a.x) * t.x;
    let y = a.y + (b.y - a.y) * t.y;
    ImVec2::new(x, y)
}


// static inline ImVec4 ImLerp(const ImVec4& a, const ImVec4& b, c_float t)          { return ImVec4(a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t, a.z + (b.z - a.z) * t, a.w + (b.w - a.w) * t); }
pub fn ImLerpVec4(a: &ImVec4, b: &ImVec4, t: c_float) -> ImVec4 {
    let x = a.x + (b.x - a.x) * t;
    let y = a.y + (b.y - a.y) * t;
    let z = a.z + (b.z - a.z) * t;
    let w = a.w + (b.w - a.w) * t;
    ImVec4::new2(x,y,z,w)
}

// static inline c_float  ImSaturate(c_float 0.0)                                        { return (f < 0.0) ? 0.0 : (f > 1.0) ? 1.0 : f; }
pub fn ImSaturateFloat(x: c_float) -> c_float {
    if x < 0.0 {
        0.0
    }
    if x > 1.0 {
        1.0
    }
    x
}

// static inline c_float  ImLengthSqr(lhs: &ImVec2)                             { return (lhs.x * lhs.x) + (lhs.y * lhs.y); }
pub fn ImLengthSqrVec2(lhs: &ImVec2) -> c_float {
    (lhs.x * lhs.x) + (lhs.y * lhs.y)
}


// static inline c_float  ImLengthSqr(const ImVec4& lhs)                             { return (lhs.x * lhs.x) + (lhs.y * lhs.y) + (lhs.z * lhs.z) + (lhs.w * lhs.w); }
pub fn ImLengthSqrVec4(lhs: &ImVec4) -> c_float{
    (lhs.x * lhs.x) + (lhs.y * lhs.y) + (lhs.z * lhs.z) + (lhs.w * lhs.w)
}


// static inline c_float  ImInvLength(lhs: &ImVec2, c_float fail_value)           { let d: c_float =  (lhs.x * lhs.x) + (lhs.y * lhs.y); if (d > 0.0) return ImRsqrt(d); return fail_value; }
pub fn ImInvLength(lhs: &ImVec2, fail_value: c_float) -> c_float {
    let d = (lhs.x * lhs.x) + (lhs.y * lhs.y);
    if d > 0.0 {
        ImRsqrtFloat(d)
    } else {
        fail_value
    }
}

// static inline c_float  ImFloor(c_float 0.0)                                           { return (0.0); }
pub fn ImFloorFloat(x : c_float) -> c_float {
    x.floor()
}

// static inline c_float  ImFloorSigned(c_float 0.0)                                     { return ((f >= 0 || f == 0.0) ? f : f - 1); } // Decent replacement for floorf()

// static inline ImFloor: ImVec2(v: &ImVec2)                                   { return ImVec2::new((v.x), (v.y)); }
pub fn ImFloorVec2(v: &ImVec2) -> ImVec2 {
    ImVec2::new(v.x.floor(), v.y.floor())
}


// static inline ImFloorSigned: ImVec2(v: &ImVec2)                             { return ImVec2::new(ImFloorSigned(v.x), ImFloorSigned(v.y)); }


// static inline c_int    ImModPositive(a: c_int, b: c_int)                                { return (a + b) % b; }
pub fn ImModPositive(a: c_int, b: c_int) -> c_int {
    (a + b) % b
}


// static inline c_float  ImDot(a: &ImVec2, b: &ImVec2)                    { return a.x * b.x + a.y * b.y; }
pub fn ImDotVec2(a: &ImVec2, b: &ImVec2) -> c_float {
    a.x * b.x + a.y * b.y
}


// static inline ImRotate: ImVec2(v: &ImVec2, c_float cos_a, c_float sin_a)        { return ImVec2::new(v.x * cos_a - v.y * sin_a, v.x * sin_a + v.y * cos_a); }
pub fn ImRotate(v: &ImVec2, cos_a: c_float, sin_a: c_float) -> ImVec2 {
    ImVec2::new(
        v.x * cos_a - v.y * sin_a,
        v.x * sin_a - v.y * cos_a
    )
}


// static inline c_float  ImLinearSweep(c_float current, c_float target, c_float speed)    { if (current < target) return ImMin(current + speed, target); if (current > target) return ImMax(current - speed, target); return current; }
pub fn ImLinearSweep(current: c_float, target: c_float, speed: c_float) -> c_float {
    if current < target {
        ImMin(current + speed, target)
    }
    if current > target {
        ImMax(current - pseed, target)
    }
    current
}


// static inline ImMul: ImVec2(lhs: &ImVec2, rhs: &ImVec2)                { return ImVec2::new(lhs.x * rhs.x, lhs.y * rhs.y); }
pub fn ImMul(lhs: &ImVec2, rhs: &ImVec2) -> ImVec2 {
    ImVec2::new(lhs.x * rhs.x, lhs.y * rhs.y)
}


pub fn ImBezierCubicCalc(p1: &ImVec2, p2: &ImVec2, p3: &ImVec2, p4: &ImVec2,t: c_float) -> ImVec2
{
    let u: c_float =  1 - t;
    let w1: c_float =  u * u * u;
    let w2: c_float =  3 * u * u * t;
    let w3: c_float =  3 * u * t * t;
    let w4: c_float =  t * t * t;
    return ImVec2::new(w1 * p1.x + w2 * p2.x + w3 * p3.x + w4 * p4.x, w1 * p1.y + w2 * p2.y + w3 * p3.y + w4 * p4.y);
}

pub fn ImBezierQuadraticCalc(p1: &ImVec2, p2: &ImVec2, p3: &ImVec2,t: c_float) -> ImVec2
{
    let u: c_float =  1 - t;
    let w1: c_float =  u * u;
    let w2: c_float =  2 * u * t;
    let w3: c_float =  t * t;
    return ImVec2::new(w1 * p1.x + w2 * p2.x + w3 * p3.x, w1 * p1.y + w2 * p2.y + w3 * p3.y);
}

// Closely mimics ImBezierCubicClosestPointCasteljau() in imgui.cpp
pub fn PathBezierCubicCurveToCasteljau(path: &mut Vec<ImVec2>, x1: c_float, y1: c_float, x2: c_float, y2: c_float, x3: c_float, y3: c_float, x4: c_float, y4: c_float, tess_tol: c_float, level: c_int) {
    let dx: c_float = x4 - x1;
    let dy: c_float = y4 - y1;
    let mut d2: c_float = (x2 - x4) * dy - (y2 - y4) * dx;
    let mut d3: c_float = (x3 - x4) * dy - (y3 - y4) * dx;
    d2 = if d2 >= 0.0 { d2 } else { -d2 };
    d3 = if d3 >= 0.0 { d3 } else { -d3 };
    if (d2 + d3) * (d2 + d3) < tess_tol * (dx * dx + dy * dy) {
        path.push(ImVec2::new(x4, y4));
    } else if level < 10 {
        let x12: c_float = (x1 + x2) * 0.5;
        let y12 = (y1 + y2) * 0.5;
        let x23: c_float = (x2 + x3) * 0.5;
        let y23 = (y2 + y3) * 0.5;
        let x34: c_float = (x3 + x4) * 0.5;
        let y34 = (y3 + y4) * 0.5;
        let x123: c_float = (x12 + x23) * 0.5;
        let y123 = (y12 + y23) * 0.5;
        let x234: c_float = (x23 + x34) * 0.5;
        let y234 = (y23 + y34) * 0.5;
        let x1234: c_float = (x123 + x234) * 0.5;
        let y1234 = (y123 + y234) * 0.5;
        PathBezierCubicCurveToCasteljau(path, x1, y1, x12, y12, x123, y123, x1234, y1234, tess_tol, level + 1);
        PathBezierCubicCurveToCasteljau(path, x1234, y1234, x234, y234, x34, y34, x4, y4, tess_tol, level + 1);
    }
}

pub fn PathBezierQuadraticCurveToCasteljau(path: &mut Vec<ImVec2> ,x1: c_float,y1: c_float,x2: c_float,y2: c_float,x3: c_float,y3: c_float,tess_tol: c_float, level: c_int)
{
    let dx: c_float =  x3 - x1;
    let dy = y3 - y1;
    let det: c_float =  (x2 - x3) * dy - (y2 - y3) * dx;
    if det * det * 4.0 < tess_tol * (dx * dx + dy * dy)
    {
        path.push(ImVec2::new(x3, y3));
    }
    else if level < 10
    {
        let x12: c_float =  (x1 + x2) * 0.5;
        let y12 = (y1 + y2) * 0.5;
        let x23: c_float =  (x2 + x3) * 0.5;
        let y23 = (y2 + y3) * 0.5;
        let x123: c_float =  (x12 + x23) * 0.5;
        let y123 = (y12 + y23) * 0.5;
        PathBezierQuadraticCurveToCasteljau(path, x1, y1, x12, y12, x123, y123, tess_tol, level + 1);
        PathBezierQuadraticCurveToCasteljau(path, x123, y123, x23, y23, x3, y3, tess_tol, level + 1);
    }
}

pub fn ImAcosX(x: c_float) -> c_float {
    if x <= 0 { return IM_PI * 0.5; };
    if x >= 1 { return 0.0; };
    return ImAcos(x);
    //return (-0.69813170079773212 * x * x - 0.872664625997164770) * x + 1.5707963267948966; // Cheap approximation, may be enough for what we do.
}
