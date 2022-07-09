use crate::imgui_vec::ImVec4;
use crate::imgui_vec::ImVec2;

// Helpers: Maths
// IM_MSVC_RUNTIME_CHECKS_OFF
// - Wrapper for standard libs functions. (Note that imgui_demo.cpp does _not_ use them to keep the code easy to copy)
// #ifndef IMGUI_DISABLE_DEFAULT_MATH_FUNCTIONS
// #define ImFabs(x)           fabsf(x)
pub fn ImFabs(x: f32) -> f32 {
    f32::abs(x)
}
// #define ImSqrt(x)           sqrtf(x)
pub fn ImSqrt(x: f32) -> f32 {
    f32::sqrt(x)
}
// #define ImFmod(x, Y)        fmodf((x), (Y))
pub fn ImFmod(x: f32, y: f32) -> f32 {
    x % y
}
// #define ImCos(x)            cosf(x)
pub fn ImCos(x: f32) -> f32 {
    f32::cos(x)
}
// #define ImSin(x)            sinf(x)
pub fn ImSin(x: f32) -> f32 {
    f32::sin(x)
}
// #define ImAcos(x)           acosf(x)
pub fn ImAcos(x: f32) -> f32 {
    f32::acos(x)
}
// #define ImAtan2(Y, x)       atan2f((Y), (x))
pub fn ImAtan2(y: f32, x: f32) -> f32{
    f32::atan2(y,x)
}
// #define ImAtof(STR)         atof(STR)
pub fn ImAtof(x: &String) -> f32 {
    f32::try_from(x).unwrap()
}
//#define ImFloorStd(x)     floorf(x)           // We use our own, see ImFloor() and ImFloorSigned()
pub fn ImFloorStd(x: f32) -> f32 {
    f32::floor(x)
}
// #define ImCeil(x)           ceilf(x)
pub fn ImCeil(x: f32) -> f32 {
    f32::ceil(x)
}
// static inline float  ImPow(float x, float y)    { return powf(x, y); }          // DragBehaviorT/SliderBehaviorT uses ImPow with either float/double and need the precision
pub fn ImPow(x: f32, y: f32) -> f32 {
    f32::powf(x,y)
}

// static inline double ImPow(double x, double y)  { return pow(x, y); }
// static inline float  ImLog(float x)             { return logf(x); }             // DragBehaviorT/SliderBehaviorT uses ImLog with either float/double and need the precision
pub fn ImLog(x: f32) -> f32 {
    f32::log10(x)
}
// static inline double ImLog(double x)            { return log(x); }
// static inline int    ImAbs(int x)               { return x < 0 ? -x : x; }
pub fn ImAbsInt(x: i32) -> i32 {
    i32::abs(x)
}
// static inline float  ImAbs(float x)             { return fabsf(x); }
pub fn ImAbsFloat(x: f32) -> f32 {
    f32::abs(x)
}
// static inline double ImAbs(double x)            { return fabs(x); }
// static inline float  ImSign(float x)            { return (x < 0.0) ? -1.0 : (x > 0.0) ? 1.0 : 0.0; } // Sign operator - returns -1, 0 or 1 based on sign of argument
pub fn ImSignFloat(x: f32) -> f32 {
    if x == 0.0 {
        0.0
    } else if f32::is_sign_negative(x) {
        -1.0
    } else {
        1.0
    }
}

// static inline double ImSign(double x)           { return (x < 0.0) ? -1.0 : (x > 0.0) ? 1.0 : 0.0; }
// #ifdef IMGUI_ENABLE_SSE
// static inline float  ImRsqrt(float x)           { return _mm_cvtss_f32(_mm_rsqrt_ss(_mm_set_ss(x))); }
// #else
//static inline float  ImRsqrt(float x)           { return 1.0 / sqrtf(x); }
pub fn ImRsqrt(x: f32) -> f32 {
    1.0 / f32::sqrt(x)
}
// #endif
// static inline double ImRsqrt(double x)          { return 1.0 / sqrt(x); }
// #endif
// - ImMin/ImMax/ImClamp/ImLerp/ImSwap are used by widgets which support variety of types: signed/unsigned int/long long float/double
// (Exceptionally using templates here but we could also redefine them for those types)
// template<typename T> static inline T ImMin(T lhs, T rhs)                        { return lhs < rhs ? lhs : rhs; }

pub enum ImgMathTypes {
    Integer(i32),
    Unsigned(u32),
    Float(f32)
}

// template<typename T> static inline T ImMin(T lhs, T rhs)                        { return lhs < rhs ? lhs : rhs; }

pub fn ImMinI32(lhs: i32, rhs: i32) -> i32 {
    i32::min(lhs,rhs)
}
pub fn ImMinU32(lhs: u32, rhs: u32) -> u32 {
    u32::min(lhs, rhs)
}
pub fn ImMinF32(lhs: f32, rhs: f32) -> f32 {
    f32::min(lhs,rhs)
}

// template<typename T> static inline T ImMax(T lhs, T rhs)                        { return lhs >= rhs ? lhs : rhs; }
pub fn ImMaxI32(lhs: i32, rhs: i32) -> i32 {
    i32::max(lhs,rhs)
}
pub fn ImMaxU32(lhs: u32, rhs: u32) -> u32 {
    u32::max(lhs,rhs)
}
pub fn ImMaxF32(lhs: f32, rhs: f32) -> f32 {
    f32::max(lhs,rhs)
}

// template<typename T> static inline T ImClamp(T v, T mn, T mx)                   { return (v < mn) ? mn : (v > mx) ? mx : v; }
pub fn ImClampI32(v: i32, min_v: i32, max_v: i32) -> i32 {
    i32::clamp(v, min_v, max_v)
}
pub fn ImClampU32(v: u32, min_v: u32, max_v: u32) -> u32 {
    u32::clamp(v, min_v, max_v)
}
pub fn ImClampF32(v: f32, min_v: f32, max_v: f32)  -> f32 {
    f32::clamp(v,min_v,max_v)
}


// template<typename T> static inline T ImLerp(T a, T b, float t)                  { return (T)(a + (b - a) * t); }
pub fn ImLerpI32(a: i32, b: i32, t: f32) -> i32 {
    (a + (b-a) * t)
}
pub fn ImLerpU32(a: u32, b: u32, t: f32) -> u32 {
    (a + (b-a) * t)
}
pub fn ImLerpF32(a: f32, b: f32, t: f32) -> f32 {
    (a + (b-a) * t)
}

// template<typename T> static inline void ImSwap(T& a, T& b)                      { T tmp = a; a = b; b = tmp; }
pub fn ImSwapI32(a: &mut i32, b: &mut i32) {
    let mut tmp = *a;
    *a = *b;
    *b = tmp;
}

pub fn ImSwapU32(a: &mut u32, b: &mut u32) {
    let mut tmp = *a;
    *a = *b;
    *b = tmp;
}

pub fn ImSwapF32(a: &mut f32, b: &mut f32) {
    let mut tmp = *a;
    *a = *b;
    *b = tmp;
}

// template<typename T> static inline T ImAddClampOverflow(T a, T b, T mn, T mx)   { if (b < 0 && (a < mn - b)) return mn; if (b > 0 && (a > mx - b)) return mx; return a + b; }
pub fn ImAddClampOverflowI32(a: i32, b: i32, min_v: i32, max_v: i32) -> i32 {
    if b < 0 && a < min_v - b {
        min_v
    }
    else if b > 0 && a > max_v - b {
        max_v
    } else {
        a + b
    }
}

pub fn ImAddClampOverflowU32(a: u32, b: u32, min_v: u32, max_v: u32) -> u32 {
    if b < 0 && a < min_v - b {
        min_v
    }
    else if b > 0 && a > max_v - b {
        max_v
    } else {
        a + b
    }
}

pub fn ImAddClampOverflowF32(a: f32, b: f32, min_v: f32, max_v: f32) -> f32 {
    if b < 0.0 && a < min_v - b {
        min_v
    }
    else if b > 0.0 && a > max_v - b {
        max_v
    } else {
        a + b
    }
}


// template<typename T> static inline T ImSubClampOverflow(T a, T b, T mn, T mx)   { if (b > 0 && (a < mn + b)) return mn; if (b < 0 && (a > mx + b)) return mx; return a - b; }
pub fn ImSubClampOverflowI32(a: i32, b: i32, c: i32) -> i32 {
    if b > 0 && a < min_v + b {
        min_v
    } else if b < 0 && a > max_v + b {
        max_v
    } else {
        a - b
    }
}

// - Misc maths helpers
// static inline ImVec2 ImMin(const ImVec2& lhs, const ImVec2& rhs)                { return ImVec2(lhs.x < rhs.x ? lhs.x : rhs.x, lhs.y < rhs.y ? lhs.y : rhs.y); }
pub fn ImMinVec2(lhs: &ImVec2, rhs: &ImVec2) -> ImVec2 {
    ImVec2 {
        x: if lhs.x < rhs.x { lhs.x } else {rhs.x},
        y: if lhs.y < rhs.y {lhs.y} else { rhs.y}
    }
}

// static inline ImVec2 ImMax(const ImVec2& lhs, const ImVec2& rhs)                { return ImVec2(lhs.x >= rhs.x ? lhs.x : rhs.x, lhs.y >= rhs.y ? lhs.y : rhs.y); }
pub fn ImMaxVec2(lhs: &ImVec2, rhs: &ImVec2) -> ImVec2 {
    ImVec2 {
        x: if lhs.x >rhs.x {lhs.x} else {rhs.x},
        y: if lhs.y > rhs.y {lhs.y} else {rhs.y}
    }
}

// static inline ImVec2 ImClamp(const ImVec2& v, const ImVec2& mn, ImVec2 mx)      { return ImVec2((v.x < mn.x) ? mn.x : (v.x > mx.x) ? mx.x : v.x, (v.y < mn.y) ? mn.y : (v.y > mx.y) ? mx.y : v.y); }
pub fn ImClampVec2(v: &ImVec2, min_v: &ImVec2, max_v: &ImVec2) -> ImVec2 {
    ImVec2 {
        x: if v.x < min_v.x { min_v.x} else if v.x > max_v.x { max_v.x} else {v.x},
        y: if v.y < min_v.y { min_v.y} else if v.y > max_v.y { max_v.y} else {v.y}
    }
}

// static inline ImVec2 ImLerp(const ImVec2& a, const ImVec2& b, float t)          { return ImVec2(a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t); }
pub fn ImLerpVec2(a: &ImVec2, b: &ImVec2, t: f32) -> ImVec2 {
    Imvec2 {
        x: a.x + (b.x - a.x) * t,
        y: a.y + (b.y - a.y) & t
    }
}

// static inline ImVec2 ImLerp(const ImVec2& a, const ImVec2& b, const ImVec2& t)  { return ImVec2(a.x + (b.x - a.x) * t.x, a.y + (b.y - a.y) * t.y); }
pub fn ImLerpVec22(a: &ImVec2, b: &ImVec2, t: &ImVec2) -> ImVec2 {
    ImVec2 {
        x: a.x + (b.x - a.x) * t.x,
        y: a.y + (b.y - a.y) * t.y,
    }
}

// static inline ImVec4 ImLerp(const ImVec4& a, const ImVec4& b, float t)          { return ImVec4(a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t, a.z + (b.z - a.z) * t, a.w + (b.w - a.w) * t); }
pub fn ImLerpVec4(a: &ImVec4, b: &ImVec4, t: f32) -> ImVec4 {
    ImVec4 {
        x: a.x + (b.x - a.x) * t,
        y: a.y + (b.y - a.y) * t,
        z: a.z + (b.z - a.z) * t,
        w: a.w + (b.w - a.w) * t
    }
}

// static inline float  ImSaturate(float f)                                        { return (f < 0.0) ? 0.0 : (f > 1.0) ? 1.0 : f; }
pub fn ImSaturate(f: f32) -> f32 {
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

// static inline float  ImLengthSqr(const ImVec4& lhs)                             { return (lhs.x * lhs.x) + (lhs.y * lhs.y) + (lhs.z * lhs.z) + (lhs.w * lhs.w); }
pub fn ImLengthSqr(lhs: &ImVec4) -> f32 {
    (lhs.x * lhs.x) + (lhs.y * lhs.y) + (lhs.z * lhs.z) + (lhs.w * lhs.w)
}

// static inline float  ImInvLength(const ImVec2& lhs, float fail_value)           { float d = (lhs.x * lhs.x) + (lhs.y * lhs.y); if (d > 0.0) return ImRsqrt(d); return fail_value; }
pub fn ImInvLength(lhs: &ImVec2, fail_value: f32) -> f32 {
    let mut d = (lhs.x * lhs.x) +  (lhs.y * lhs.y);
    if d > 0.0 {
        return ImRsqrt(d)
    }
    fail_value
}

// static inline float  ImFloor(float f)                                           { return (float)(f); }
pub fn ImFloor(f: f32) -> f32 {
    f32::floor(f)
}

// static inline float  ImFloorSigned(float f)                                     { return (float)((f >= 0 || (float)f == f) ? f : f - 1); } // Decent replacement for floorf()
// pub fn ImFloorSigned(f: f32) -> f32 {
//     f32::floor
// }

// static inline ImVec2 ImFloor(const ImVec2& v)                                   { return ImVec2((float)(v.x), (float)(v.y)); }
pub fn ImFloorVec2(v: &ImVec2) -> ImVec2 {
    ImVec2 {
        x: f32::floor(v.x),
        y: f32::floor(v.y)
    }
}


// static inline ImVec2 ImFloorSigned(const ImVec2& v)                             { return ImVec2(ImFloorSigned(v.x), ImFloorSigned(v.y)); }

// static inline int    ImModPositive(int a, int b)                                { return (a + b) % b; }
pub fn ImModPositive(a: i32, b: i32) -> i32 {
    (a + b) % b
}

// static inline float  ImDot(const ImVec2& a, const ImVec2& b)                    { return a.x * b.x + a.y * b.y; }
pub fn ImDot(a: &ImVec2, b: &ImVec2) -> f32 {
    a.x * b.x + a.y * b.y
}

// static inline ImVec2 ImRotate(const ImVec2& v, float cos_a, float sin_a)        { return ImVec2(v.x * cos_a - v.y * sin_a, v.x * sin_a + v.y * cos_a); }
pub fn ImRotate(v: &ImVec2, cos_a: f32, sin_a: f32) -> ImVec2 {
    ImVec2{
        x: v.x * cos_a - v.y * sin_a,
        y: v.x * sin_a + v.y * cos_a
    }
}

// static inline float  ImLinearSweep(float current, float target, float speed)    { if (current < target) return ImMin(current + speed, target); if (current > target) return ImMax(current - speed, target); return current; }
pub fn ImLinearSweep(current: f32, target: f32, speed: f32) -> f32 {
    if current < target {
        ImMin(current + speed, target)
    } else if current > target {
        ImMax(current - speed, target)
    } else {
        current
    }
}

// static inline ImVec2 ImMul(const ImVec2& lhs, const ImVec2& rhs)                { return ImVec2(lhs.x * rhs.x, lhs.y * rhs.y); }
pub fn ImMulVec2(lhs: &ImVec2, rhs: &ImVec2) -> ImVec2 {
    ImVec2 {
        x: lhs.x * rhs.x,
        y: lhs.y * rhs.y
    }
}

// static inline bool   ImIsFloatAboveGuaranteedIntegerPrecision(float f)          { return f <= -16777216 || f >= 16777216; }
pub fn ImIsFloatAboveGuranteedIntegerPrecision(f: f32) -> bool {
    f <= -16777216.0 || f >= 16777216.0
}
// IM_MSVC_RUNTIME_CHECKS_RESTORE


// #define IM_F32_TO_INT8_UNBOUND(_VAL)    (((_VAL) * 255.0 + ((_VAL)>=0 ? 0.5 : -0.5)))   // Unsaturated, for display purpose
pub fn IM_F32_TO_INT8_UNBOUND(x: f32) -> i8 {
    x * 255.0 + if x >= 0.0 { 0.5} else { -0.5 } as i8
}

// #define IM_F32_TO_INT8_SAT(_VAL)        ((ImSaturate(_VAL) * 255.0 + 0.5))               // Saturated, always output 0..255
pub fn IM_F32_TO_INT8_SAT(x: f32) -> i8 {
    ImSaturate(x) * 255.0 + 0.5 as i8
}

// #define IM_FLOOR(_VAL)                  ((float)(_VAL))                                    // ImFloor() is not inlined in MSVC debug builds
// #define IM_ROUND(_VAL)                  ((float)((_VAL) + 0.5))                           //
pub fn IM_ROUND(x: f32) -> f32{
    f32::round(x)
}
