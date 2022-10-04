#![allow(non_snake_case)]

use libc::c_float;

// static inline bool   ImIsFloatAboveGuaranteedIntegerPrecision(0f32: c_float)
pub fn ImIsFloatAboveGuaranteedIntegerPrecision(f: c_loat) -> bool {
    return f <= -16777216 || f >= 16777216;
}

static inline c_float  ImPow(x: c_float, y: c_float)    { return powf(x, y); }          // DragBehaviorT/SliderBehaviorT uses ImPow with either float/double and need the precision
static inline double ImPow(double x, double y)  { return pow(x, y); }
static inline c_float  ImLog(x: c_float)             { return logf(x); }             // DragBehaviorT/SliderBehaviorT uses ImLog with either float/double and need the precision
static inline double ImLog(double x)            { return log(x); }
static inline c_int    ImAbs(x: c_int)               { return x < 0 ? -x : x; }
static inline c_float  ImAbs(x: c_float)             { return fabsf(x); }
static inline double ImAbs(double x)            { return fabs(x); }
static inline c_float  ImSign(x: c_float)            { return (x < 0f32) ? -1f32 : (x > 0f32) ? 1f32 : 0f32; } // Sign operator - returns -1, 0 or 1 based on sign of argument
static inline double ImSign(double x)           { return (x < 0.0) ? -1.0 : (x > 0.0) ? 1.0 : 0.0; }
// #ifdef IMGUI_ENABLE_SSE
static inline c_float  ImRsqrt(x: c_float)           { return _mm_cvtss_f32(_mm_rsqrt_ss(_mm_set_ss(x))); }
// #else
static inline c_float  ImRsqrt(x: c_float)           { return 1f32 / sqrtf(x); }
// #endif
static inline double ImRsqrt(double x)          { return 1.0 / sqrt(x); }

template<typename T> static inline T ImMin(T lhs, T rhs)                        { return lhs < rhs ? lhs : rhs; }
template<typename T> static inline T ImMax(T lhs, T rhs)                        { return lhs >= rhs ? lhs : rhs; }
template<typename T> static inline T ImClamp(T v, T mn, T mx)                   { return (v < mn) ? mn : (v > mx) ? mx : v; }
template<typename T> static inline T ImLerp(T a, T b, t: c_float)                  { return (T)(a + (b - a) * t); }
template<typename T> static inline c_void ImSwap(T& a, T& b)                      { T tmp = a; a = b; b = tmp; }
template<typename T> static inline T ImAddClampOverflow(T a, T b, T mn, T mx)   { if (b < 0 && (a < mn - b)) return mn; if (b > 0 && (a > mx - b)) return mx; return a + b; }
template<typename T> static inline T ImSubClampOverflow(T a, T b, T mn, T mx)   { if (b > 0 && (a < mn + b)) return mn; if (b < 0 && (a > mx + b)) return mx; return a - b; }
// - Misc maths helpers
static inline ImVec2 ImMin(const lhs: &ImVec2, const rhs: &ImVec2)                { return ImVec2::new(lhs.x < rhs.x ? lhs.x : rhs.x, lhs.y < rhs.y ? lhs.y : rhs.y); }
static inline ImVec2 ImMax(const lhs: &ImVec2, const rhs: &ImVec2)                { return ImVec2::new(lhs.x >= rhs.x ? lhs.x : rhs.x, lhs.y >= rhs.y ? lhs.y : rhs.y); }
static inline ImVec2 ImClamp(const v: &ImVec2, const mn: &ImVec2, mx: ImVec2)      { return ImVec2::new((v.x < mn.x) ? mn.x : (v.x > mx.x) ? mx.x : v.x, (v.y < mn.y) ? mn.y : (v.y > mx.y) ? mx.y : v.y); }
static inline ImVec2 ImLerp(const a: &ImVec2, const b: &ImVec2, t: c_float)          { return ImVec2::new(a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t); }
static inline ImVec2 ImLerp(const a: &ImVec2, const b: &ImVec2, const t: &ImVec2)  { return ImVec2::new(a.x + (b.x - a.x) * t.x, a.y + (b.y - a.y) * t.y); }
static inline ImVec4 ImLerp(const ImVec4& a, const ImVec4& b, t: c_float)          { return ImVec4(a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t, a.z + (b.z - a.z) * t, a.w + (b.w - a.w) * t); }
static inline c_float  ImSaturate(0f32: c_float)                                        { return (f < 0f32) ? 0f32 : (f > 1f32) ? 1f32 : f; }
static inline c_float  ImLengthSqr(const lhs: &ImVec2)                             { return (lhs.x * lhs.x) + (lhs.y * lhs.y); }
static inline c_float  ImLengthSqr(const ImVec4& lhs)                             { return (lhs.x * lhs.x) + (lhs.y * lhs.y) + (lhs.z * lhs.z) + (lhs.w * lhs.w); }
static inline c_float  ImInvLength(const lhs: &ImVec2, fail_value: c_float)           { let d: c_float =  (lhs.x * lhs.x) + (lhs.y * lhs.y); if (d > 0f32) return ImRsqrt(d); return fail_value; }
static inline c_float  ImFloor(0f32: c_float)                                           { return (0f32); }
static inline c_float  ImFloorSigned(0f32: c_float)                                     { return ((f >= 0 || f == 0f32) ? f : f - 1); } // Decent replacement for floorf()
static inline ImVec2 ImFloor(const v: &ImVec2)                                   { return ImVec2::new((v.x), (v.y)); }
static inline ImVec2 ImFloorSigned(const v: &ImVec2)                             { return ImVec2::new(ImFloorSigned(v.x), ImFloorSigned(v.y)); }
static inline c_int    ImModPositive(a: c_int, b: c_int)                                { return (a + b) % b; }
static inline c_float  ImDot(const a: &ImVec2, const b: &ImVec2)                    { return a.x * b.x + a.y * b.y; }
static inline ImVec2 ImRotate(const v: &ImVec2, cos_a: c_float, sin_a: c_float)        { return ImVec2::new(v.x * cos_a - v.y * sin_a, v.x * sin_a + v.y * cos_a); }
static inline c_float  ImLinearSweep(current: c_float, target: c_float, speed: c_float)    { if (current < target) return ImMin(current + speed, target); if (current > target) return ImMax(current - speed, target); return current; }
static inline ImVec2 ImMul(const lhs: &ImVec2, const rhs: &ImVec2)                { return ImVec2::new(lhs.x * rhs.x, lhs.y * rhs.y); }
ImVec2     ImBezierCubicCalc(const p1: &ImVec2, const p2: &ImVec2, const p3: &ImVec2, const p4: &ImVec2, t: c_float);
 ImVec2     ImBezierCubicClosestPoint(const p1: &ImVec2, const p2: &ImVec2, const p3: &ImVec2, const p4: &ImVec2, const p: &ImVec2, num_segments: c_int);       // For curves with explicit number of segments
 ImVec2     ImBezierCubicClosestPointCasteljau(const p1: &ImVec2, const p2: &ImVec2, const p3: &ImVec2, const p4: &ImVec2, const p: &ImVec2, tess_tol: c_float);// For auto-tessellated curves you can use tess_tol = style.CurveTessellationTol
 ImVec2     ImBezierQuadraticCalc(const p1: &ImVec2, const p2: &ImVec2, const p3: &ImVec2, t: c_float);
 ImVec2     ImLineClosestPoint(const a: &ImVec2, const b: &ImVec2, const p: &ImVec2);
 bool       ImTriangleContainsPoint(const a: &ImVec2, const b: &ImVec2, const c: &ImVec2, const p: &ImVec2);
 ImVec2     ImTriangleClosestPoint(const a: &ImVec2, const b: &ImVec2, const c: &ImVec2, const p: &ImVec2);
 c_void       ImTriangleBarycentricCoords(const a: &ImVec2, const b: &ImVec2, const c: &ImVec2, const p: &ImVec2, c_float& out_u, c_float& out_v, c_float& out_w);
inline c_float         ImTriangleArea(const a: &ImVec2, const b: &ImVec2, const c: &ImVec2) { return ImFabs((a.x * (b.y - c.y)) + (b.x * (c.y - a.y)) + (c.x * (a.y - b.y))) * 0.5f32; }
 ImGuiDir   ImGetDirQuadrantFromDelta(dx: c_float, dy: c_float);
