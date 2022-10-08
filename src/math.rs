#![allow(non_snake_case)]

use libc::{c_double, c_float, c_int};
use crate::direction::ImGuiDir;
use crate::vec2::ImVec2;
use crate::vec4::ImVec4;

// Helpers: Bit manipulation
// static inline bool      ImIsPowerOfTwo(v: c_int)
pub fn ImIsPowerOfTwo(y: c_int) -> bool {
    return v != 0 && (v & (v - 1)) == 0;
}


// static inline bool      ImIsPowerOfTwo(u64 v)
pub fn ImIsPowerOfTwo2(v: u64) -> bool {
    return v != 0 && (v & (v - 1)) == 0;
}


// static inline c_int       ImUpperPowerOfTwo(v: c_int)
pub fn ImUpperPowerOfTwo(mut v: c_int) -> c_int {
    v -= 1;
    v |= v >> 1;
    v |= v >> 2;
    v |= v >> 4;
    v |= v >> 8;
    v |= v >> 16;
    v += 1;
    return v;
}

// static inline bool   ImIsFloatAboveGuaranteedIntegerPrecision(0f32: c_float)
pub fn ImIsFloatAboveGuaranteedIntegerPrecision(f: c_loat) -> bool {
    return f <= -16777216 || f >= 16777216;
}

// static inline c_float  ImPow(x: c_float, y: c_float)    { return powf(x, y); }          // DragBehaviorT/SliderBehaviorT uses ImPow with either float/double and need the precision
pub fn ImPow(x: c_float, y: c_float) -> c_float {
    // return f32::pow(x, y);
    let x1: f32 = x as f32;
    x1.powf(y)
}


// static inline double ImPow(double x, double y)  { return pow(x, y); }
pub fn ImPow2(x: c_double, y: c_double) -> c_double {
    x.pow(y)
}

// static inline c_float  ImLog(x: c_float)             { return logf(x); }             // DragBehaviorT/SliderBehaviorT uses ImLog with either float/double and need the precision
pub fn ImLog(x: c_float) -> c_float {
    // x.logf()
    let x1 = x as f32;
    x1.log10()
}

// static inline double ImLog(double x)            { return log(x); }
pub fn ImLog2(x: c_double) -> c_double {
    let x1 = x as f32;
    x1.log10() as c_double
}

// static inline c_int    ImAbs(x: c_int)               { return x < 0 ? -x : x; }
pub fn ImAbs(x: c_int) -> c_int {
    let x1 = x as i32;
    x1.abs()
}

// static inline c_float  ImAbs(x: c_float)             { return fabsf(x); }
pub fn ImAbs2(x: c_float) -> c_float {
    let x1 = x as f32;
    x1.abs()
}

// static inline double ImAbs(double x)            { return fabs(x); }
pub fn ImAbs3(x: c_double) -> c_double {
    let x1 = x as f32;
    x1.abs() as c_double
}


// static inline c_float  ImSign(x: c_float)            { return (x < 0f32) ? -1f32 : (x > 0f32) ? 1f32 : 0f32; } // Sign operator - returns -1, 0 or 1 based on sign of argument
pub fn ImSign(x: c_float) -> c_float {
    return if x < 0.0 {
        -1.0
    } else {
        if x > 0.0 {
            1
        } else {
            0.0
        }
    };
}


// static inline double ImSign(double x)           { return (x < 0.0) ? -1.0 : (x > 0.0) ? 1.0 : 0.0; }
pub fn ImSign2(x: c_double) -> c_double {
    return if x < 0.0 {
        -1.0
    } else {
        if x > 0.0 {
            1.0
        } else {
            0.0
        }
    };
}

// #ifdef IMGUI_ENABLE_SSE
// static inline c_float  ImRsqrt(x: c_float)           { return _mm_cvtss_f32(_mm_rsqrt_ss(_mm_set_ss(x))); }
// #else
// static inline c_float  ImRsqrt(x: c_float)           { return 1f32 / sqrtf(x); }
pub fn ImRsqrt(x: c_float) -> c_float {
    let x1 = x as f32;
    1.0 / x1.sqrt()
}

// #endif
// static inline double ImRsqrt(double x)          { return 1.0 / sqrt(x); }
pub fn ImRsqrt2(x: c_double) -> c_double {
    let x1 = x as f32;
    (1.0 / x1.sqrt()) as c_double
}

// template<typename T> static inline T ImMin(T lhs, T rhs)                        { return lhs < rhs ? lhs : rhs; }
pub fn ImMin<T>(lhs: T, rhs: T) -> T {
    return if lhs < rhs {
        lhs
    } else {
        rhs
    };
}

// template<typename T> static inline T ImMax(T lhs, T rhs)                        { return lhs >= rhs ? lhs : rhs; }
pub fn ImMax<T>(lhs: T, rhs: T) -> T {
    return if lhs >= rhs {
        lhs
    } else {
        rhs
    };
}

// template<typename T> static inline T ImClamp(T v, T mn, T mx)                   { return (v < mn) ? mn : (v > mx) ? mx : v; }
pub fn ImClamp<T>(v: T, mn: T, mx: T) -> T {
    return if v < mn {
        mn
    } else {
        if v > mx {
            ex
        } else {
            v
        }
    };
}


// template<typename T> static inline T ImLerp(T a, T b, t: c_float)                  { return (T)(a + (b - a) * t); }
pub fn ImLerp<T: Clone>(a: T, b: T, t: c_float) -> T {
    return a + (b - a.clone()) * t;
}

// template<typename T> static inline c_void ImSwap(T& a, T& b)                      { T tmp = a; a = b; b = tmp; }
pub fn ImSwap<T>(a: &mut T, b: &mut T) {
    let tmp = a.clone();
    *a = b.clone();
    *b = tmp.clone();
}

// template<typename T> static inline T ImAddClampOverflow(T a, T b, T mn, T mx)   { if (b < 0 && (a < mn - b)) return mn; if (b > 0 && (a > mx - b)) return mx; return a + b; }
pub fn ImAddClampOverflow(a: T, b: T, mn: T, mx: T) -> T {
    if b < 0 && (a < mn - b) {
        mn
    }
    if b > 0 && (a > mx - b) {
        mx
    }
    a + b
}

// template<typename T> static inline T ImSubClampOverflow(T a, T b, T mn, T mx)   { if (b > 0 && (a < mn + b)) return mn; if (b < 0 && (a > mx + b)) return mx; return a - b; }
pub fn ImSubClampOverflow(a: T, b: T, mn: T, mx: T) -> T {
    if b > 0 && (a < mn + b) {
        mn
    }
    if b < 0 && (a > mx + b) {
        mx
    }
    a - b
}

// - Misc maths helpers
// static inline ImVec2 ImMin(const lhs: &ImVec2, const rhs: &ImVec2)                { return ImVec2::new(lhs.x < rhs.x ? lhs.x : rhs.x, lhs.y < rhs.y ? lhs.y : rhs.y); }

pub fn ImMin2(lhs: &ImVec2, rhs: &ImVec2) -> ImVec2 {
    ImVec2::new2(
        if lhs.x < rhs.x {
            lhs.x
        } else {
            rhs.x
        },
        if lhs.y < rhs.y {
            lhs.y
        } else {
            rhs.y
        },
    )
}

// static inline ImVec2 ImMax(const lhs: &ImVec2, const rhs: &ImVec2)                { return ImVec2::new(lhs.x >= rhs.x ? lhs.x : rhs.x, lhs.y >= rhs.y ? lhs.y : rhs.y); }
pub fn ImMax2(lhs: &ImVec2, rhs: &ImVec2) -> ImVec2 {
    ImVec2::new2(
        if lhs.x >= rhs.x {
            lhs.x
        } else {
            rhs.x
        },
        if lhs.y >= rhs.y {
            lhs.y
        } else {
            rhs.y
        },
    )
}


// static inline ImVec2 ImClamp(const v: &ImVec2, const mn: &ImVec2, mx: ImVec2)      { return ImVec2::new((v.x < mn.x) ? mn.x : (v.x > mx.x) ? mx.x : v.x, (v.y < mn.y) ? mn.y : (v.y > mx.y) ? mx.y : v.y); }
pub fn ImClamp2(v: &ImVec2, mn: &ImVec2, mx: ImVec2) -> ImVec2 {
    ImVec2::new2(if v.x < mn.x {
        mn.x
    } else {
        if v.x > mx.x {
            mx.x
        } else {
            v.x
        }
    },
                 if v.y < mn.y {
                     mn.y
                 } else {
                     if v.y > mx.y {
                         mx.y
                     } else {
                         v.y
                     }
                 })
}


// static inline ImVec2 ImLerp(const a: &ImVec2, const b: &ImVec2, t: c_float)          { return ImVec2::new(a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t); }
pub fn ImLerp2(a: &ImVec2, b: &ImVec2, t: c_float) -> ImVec2 {
    ImVec2::new2(
        a.x + (b.x - a.x) * t,
        a.y + (b.y - a.y) * t,
    )
}


// static inline ImVec2 ImLerp(const a: &ImVec2, const b: &ImVec2, const t: &ImVec2)  { return ImVec2::new(a.x + (b.x - a.x) * t.x, a.y + (b.y - a.y) * t.y); }
pub fn ImLerp3(a: &ImVec2, b: &ImVec2, t: &ImVec2) -> ImVec2 {
    ImVec2::new2(
        a.x + (b.x - a.x) * t.x,
        a.y + (b.y - a.y) * t.y,
    )
}


// static inline ImVec4 ImLerp(const ImVec4& a, const ImVec4& b, t: c_float)          { return ImVec4(a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t, a.z + (b.z - a.z) * t, a.w + (b.w - a.w) * t); }
pub fn ImLerp4(a: &ImVec4, b: &ImVec4, t: c_float) -> ImVec4 {
    ImVec4::new2(
        a.x + (b.x - a.x) * t,
        a.y + (b.y - a.y) * t,
        a.z + (b.z - a.z) * t,
        a.w + (b.w - a.w) * t,
    )
}


// static inline c_float  ImSaturate(0f32: c_float)                                        { return (f < 0f32) ? 0f32 : (f > 1f32) ? 1f32 : f; }
pub fn ImSaturate(f: c_float) -> c_float {
    if f < 0.0 {
        0.0
    } else {
        if f > 1.0 {
            1.0
        } else {
            f
        }
    }
}

// static inline c_float  ImLengthSqr(const lhs: &ImVec2)                             { return (lhs.x * lhs.x) + (lhs.y * lhs.y); }
pub fn ImLengthSqr(lhs: &ImVec2) -> c_float {
    (lhs.x * lhs.x) + (lhs.y * lhs.y)
}

// static inline c_float  ImLengthSqr(const ImVec4& lhs)                             { return (lhs.x * lhs.x) + (lhs.y * lhs.y) + (lhs.z * lhs.z) + (lhs.w * lhs.w); }
pub fn ImLengthSqr2(lhs: &ImVec4) -> c_float {
    (lhs.x * lhs.x) + (lhs.y * lhs.y) + (lhs.z * lhs.z) + (lhs.w * lhs.w)
}

// static inline c_float  ImInvLength(const lhs: &ImVec2, fail_value: c_float)           { let d: c_float =  (lhs.x * lhs.x) + (lhs.y * lhs.y); if (d > 0f32) return ImRsqrt(d); return fail_value; }
pub fn ImInvLength(lhs: &ImVec2, fail_value: c_float) -> c_float {
    let d: c_float = (lhs.x * lhs.x) + (lhs.y * lhs.y);
    if d > 0f32 { return ImRsqrt(d); }
    return fail_value;
}

// static inline c_float  ImFloor(0f32: c_float)                                           { return (0f32); }
pub fn ImFloor(f: c_float) -> c_float {
    f
}

// static inline c_float  ImFloorSigned(0f32: c_float)                                     { return ((f >= 0 || f == 0f32) ? f : f - 1); } // Decent replacement for floorf()
pub fn ImFloorSigned(f: c_float) -> c_float {
    if f >= 0.0 || f == 0.0 { f } else { f - 1 }
}

// static inline ImVec2 ImFloor(const v: &ImVec2)                                   { return ImVec2::new((v.x), (v.y)); }
pub fn ImFloor2(v: &ImVec2) -> ImVec2 {
    ImVec2::new2(v.x, v.y)
}

// static inline ImVec2 ImFloorSigned(const v: &ImVec2)                             { return ImVec2::new(ImFloorSigned(v.x), ImFloorSigned(v.y)); }
pub fn ImFloorSigned2(v: &ImVec2) -> ImVec2 {
    ImVec2::new2(ImFloorSigned(v.x), ImFloorSigned(v.y))
}

// static inline c_int    ImModPositive(a: c_int, b: c_int)                                { return (a + b) % b; }
pub fn ImModPositive(a: c_int, b: c_int) -> c_int {
    (a + b) % b
}

// static inline c_float  ImDot(const a: &ImVec2, const b: &ImVec2)                    { return a.x * b.x + a.y * b.y; }
pub fn ImDot(a: &ImVec2, b: &ImVec2) -> c_float {
    a.x * b.x + a.y * b.y
}

// static inline ImVec2 ImRotate(const v: &ImVec2, cos_a: c_float, sin_a: c_float)        { return ImVec2::new(v.x * cos_a - v.y * sin_a, v.x * sin_a + v.y * cos_a); }
pub fn ImRotate(v: &ImVec2, cos_a: c_float, sin_a: c_float) -> ImVec2 {
    ImVec2::new2(v.x * cos_a - v.y * sin_a, v.x * sin_a + v.y * cos_a)
}

// static inline c_float  ImLinearSweep(current: c_float, target: c_float, speed: c_float)    { if (current < target) return ImMin(current + speed, target); if (current > target) return ImMax(current - speed, target); return current; }
pub fn ImLinearSweep(current: c_float, target: c_float, speed: c_float) -> c_float {
    if current < target {
        ImMin(current + speed, target)
    }
    if current > target {
        ImMax(current - speed, target)
    }
    current
}

// static inline ImVec2 ImMul(const lhs: &ImVec2, const rhs: &ImVec2)                { return ImVec2::new(lhs.x * rhs.x, lhs.y * rhs.y); }
pub fn ImMul(lhs: &ImVec2, rhs: &ImVec2) -> ImVec2 {
    ImVec2::new2(
        lhs.x * rhs.x,
        lhs.y * rhs.y,
    )
}

// ImVec2     ImBezierCubicCalc(const p1: &ImVec2, const p2: &ImVec2, const p3: &ImVec2, const p4: &ImVec2, t: c_float);
pub fn ImBezierCubicCalc(p1: &ImVec2, p2: &ImVec2, p3: &ImVec2, p4: &ImVec2, t: c_float) -> ImVec2
{
    let u: c_float =  1f32 - t;
    let w1: c_float =  u * u * u;
    let w2: c_float =  3 * u * u * t;
    let w3: c_float =  3 * u * t * t;
    let w4: c_float =  t * t * t;
    return ImVec2::new2(w1 * p1.x + w2 * p2.x + w3 * p3.x + w4 * p4.x, w1 * p1.y + w2 * p2.y + w3 * p3.y + w4 * p4.y);
}

// ImVec2     ImBezierCubicClosestPoint(const p1: &ImVec2, const p2: &ImVec2, const p3: &ImVec2, const p4: &ImVec2, const p: &ImVec2, num_segments: c_int);       // For curves with explicit number of segments

// ImVec2     ImBezierCubicClosestPointCasteljau(const p1: &ImVec2, const p2: &ImVec2, const p3: &ImVec2, const p4: &ImVec2, const p: &ImVec2, tess_tol: c_float);// For auto-tessellated curves you can use tess_tol = style.CurveTessellationTol

// ImVec2     ImBezierQuadraticCalc(const p1: &ImVec2, const p2: &ImVec2, const p3: &ImVec2, t: c_float);
pub fn ImBezierQuadraticCalc(p1: &ImVec2, p2: &ImVec2, p3: &ImVec2, t: c_float) -> ImVec2
{
    let u: c_float =  1f32 - t;
    let w1: c_float =  u * u;
    let w2: c_float =  2 * u * t;
    let w3: c_float =  t * t;
    return ImVec2::new2(w1 * p1.x + w2 * p2.x + w3 * p3.x, w1 * p1.y + w2 * p2.y + w3 * p3.y);
}


// ImVec2     ImLineClosestPoint(const a: &ImVec2, const b: &ImVec2, const p: &ImVec2);

// bool       ImTriangleContainsPoint(const a: &ImVec2, const b: &ImVec2, const c: &ImVec2, const p: &ImVec2);

// ImVec2     ImTriangleClosestPoint(const a: &ImVec2, const b: &ImVec2, const c: &ImVec2, const p: &ImVec2);

// c_void       ImTriangleBarycentricCoords(const a: &ImVec2, const b: &ImVec2, const c: &ImVec2, const p: &ImVec2, c_float& out_u, c_float& out_v, c_float& out_w);

// inline c_float         ImTriangleArea(const a: &ImVec2, const b: &ImVec2, const c: &ImVec2) { return ImFabs((a.x * (b.y - c.y)) + (b.x * (c.y - a.y)) + (c.x * (a.y - b.y))) * 0.5f32; }
pub fn ImTriangleArea(a: &ImVec2, b: &ImVec2, c: &ImVec2) -> c_float {
    ImFabs((a.x * (b.y - c.y)) + (b.x * (c.y - a.y)) + (c.x * (a.y - b.y))) * 0.5f32
}

// ImGuiDir   ImGetDirQuadrantFromDelta(dx: c_float, dy: c_float);
// pub fn ImGetDirQuadrantFromDelta(dx: c_float, dy: c_float) -> ImGuiDir
