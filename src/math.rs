use crate::imgui_vec::Vector4D;
use crate::imgui_vec::Vector2D;
use crate::vectors::two_d::Vector2D;
use crate::vectors::Vector4D;

// Helpers: Maths
// IM_MSVC_RUNTIME_CHECKS_OFF
// - Wrapper for standard libs functions. (Note that imgui_demo.cpp does _not_ use them to keep the code easy to copy)
// #ifndef IMGUI_DISABLE_DEFAULT_MATH_FUNCTIONS
// #define f32::abs(x)           fabsf(x)
// pub fn f32::abs(x: f32) -> f32 {
//     f32::abs(x)
// }
// #define ImSqrt(x)           sqrtf(x)
// pub fn ImSqrt(x: f32) -> f32 {
//     f32::sqrt(x)
// }
// #define f32::mod(x, Y)        fmodf((x), (Y))
pub fn f32_mod(x: f32, y: f32) -> f32 {
    x % y
}
// #define ImCos(x)            cosf(x)
// pub fn ImCos(x: f32) -> f32 {
//     f32::cos(x)
// }
// #define ImSin(x)            sinf(x)
// pub fn ImSin(x: f32) -> f32 {
//     f32::sin(x)
// }
// #define ImAcos(x)           acosf(x)
// pub fn ImAcos(x: f32) -> f32 {
//     f32::acos(x)
// }
// #define ImAtan2(Y, x)       atan2f((Y), (x))
// pub fn ImAtan2(y: f32, x: f32) -> f32{
//     f32::atan2(y,x)
// }
// #define ImAtof(STR)         atof(STR)
pub fn atof(x: &String) -> f32 {
    f32::try_from(x).unwrap()
}
//#define ImFloorStd(x)     floorf(x)           // We use our own, see f32::floor() and f32::floor()
// pub fn ImFloorStd(x: f32) -> f32 {
//     f32::floor(x)
// }
// #define ImCeil(x)           ceilf(x)
// pub fn ImCeil(x: f32) -> f32 {
//     f32::ceil(x)
// }
// static inline float  ImPow(float x, float y)    { return powf(x, y); }          // DragBehaviorT/SliderBehaviorT uses ImPow with either float/double and need the precision
// pub fn ImPow(x: f32, y: f32) -> f32 {
//     f32::powf(x,y)
// }

// static inline double ImPow(double x, double y)  { return pow(x, y); }
// static inline float  ImLog(float x)             { return logf(x); }             // DragBehaviorT/SliderBehaviorT uses ImLog with either float/double and need the precision
// pub fn ImLog(x: f32) -> f32 {
//     f32::log10(x)
// }
// static inline double ImLog(double x)            { return log(x); }
// static inline int    ImAbs(int x)               { return x < 0 ? -x : x; }
// pub fn ImAbsInt(x: i32) -> i32 {
//     i32::abs(x)
// }
// static inline float  ImAbs(float x)             { return fabsf(x); }
// pub fn ImAbsFloat(x: f32) -> f32 {
//     f32::abs(x)
// }
// static inline double ImAbs(double x)            { return fabs(x); }
// static inline float  ImSign(float x)            { return (x < 0.0) ? -1.0 : (x > 0.0) ? 1.0 : 0.0; } // Sign operator - returns -1, 0 or 1 based on sign of argument
pub fn f32_sign_float(x: f32) -> f32 {
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
pub fn r_sqrt(x: f32) -> f32 {
    1.0 / f32::sqrt(x)
}
// #endif
// static inline double ImRsqrt(double x)          { return 1.0 / sqrt(x); }
// #endif
// - ImMin/ImMax/ImClamp/ImLerp/ImSwap are used by widgets which support variety of types: signed/unsigned int/long long float/double
// (Exceptionally using templates here but we could also redefine them for those types)
// template<typename T> static inline T ImMin(T lhs, T rhs)                        { return lhs < rhs ? lhs : rhs; }

// pub enum ImgMathTypes {
//     Integer(i32),
//     Unsigned(u32),
//     Float(f32)
// }

// template<typename T> static inline T ImMin(T lhs, T rhs)                        { return lhs < rhs ? lhs : rhs; }

// pub fn ImMinI32(lhs: i32, rhs: i32) -> i32 {
//     i32::min(lhs,rhs)
// }
// pub fn ImMinU32(lhs: u32, rhs: u32) -> u32 {
//     u32::min(lhs, rhs)
// }
// pub fn ImMinF32(lhs: f32, rhs: f32) -> f32 {
//     f32::min(lhs,rhs)
// }

// template<typename T> static inline T ImMax(T lhs, T rhs)                        { return lhs >= rhs ? lhs : rhs; }
// pub fn ImMaxI32(lhs: i32, rhs: i32) -> i32 {
//     i32::max(lhs,rhs)
// }
// pub fn ImMaxU32(lhs: u32, rhs: u32) -> u32 {
//     u32::max(lhs,rhs)
// }
// pub fn ImMaxF32(lhs: f32, rhs: f32) -> f32 {
//     f32::max(lhs,rhs)
// }

// template<typename T> static inline T ImClamp(T v, T mn, T mx)                   { return (v < mn) ? mn : (v > mx) ? mx : v; }
// pub fn ImClampI32(v: i32, min_v: i32, max_v: i32) -> i32 {
//     i32::clamp(v, min_v, max_v)
// }
// pub fn ImClampU32(v: u32, min_v: u32, max_v: u32) -> u32 {
//     u32::clamp(v, min_v, max_v)
// }
// pub fn ImClampF32(v: f32, min_v: f32, max_v: f32)  -> f32 {
//     f32::clamp(v,min_v,max_v)
// }


// template<typename T> static inline T ImLerp(T a, T b, float t)                  { return (T)(a + (b - a) * t); }
pub fn lerp_i32(a: i32, b: i32, t: f32) -> i32 {
    (a + (b-a) * t)
}
pub fn lerp_u32(a: u32, b: u32, t: f32) -> u32 {
    (a + (b-a) * t)
}
pub fn lerp_f32(a: f32, b: f32, t: f32) -> f32 {
    (a + (b-a) * t)
}

// template<typename T> static inline void ImSwap(T& a, T& b)                      { T tmp = a; a = b; b = tmp; }
pub fn swap_i32(a: &mut i32, b: &mut i32) {
    let mut tmp = *a;
    *a = *b;
    *b = tmp;
}

pub fn swap_u32(a: &mut u32, b: &mut u32) {
    let mut tmp = *a;
    *a = *b;
    *b = tmp;
}

pub fn swap_f32(a: &mut f32, b: &mut f32) {
    let mut tmp = *a;
    *a = *b;
    *b = tmp;
}

// template<typename T> static inline T ImAddClampOverflow(T a, T b, T mn, T mx)   { if (b < 0 && (a < mn - b)) return mn; if (b > 0 && (a > mx - b)) return mx; return a + b; }
pub fn add_clamp_overflow_i32(a: i32, b: i32, min_v: i32, max_v: i32) -> i32 {
    if b < 0 && a < min_v - b {
        min_v
    }
    else if b > 0 && a > max_v - b {
        max_v
    } else {
        a + b
    }
}

pub fn add_clamp_overflow_u32(a: u32, b: u32, min_v: u32, max_v: u32) -> u32 {
    if b < 0 && a < min_v - b {
        min_v
    }
    else if b > 0 && a > max_v - b {
        max_v
    } else {
        a + b
    }
}

pub fn add_clamp_overflow_f32(a: f32, b: f32, min_v: f32, max_v: f32) -> f32 {
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
pub fn sub_clamp_overflow_i32(a: i32, b: i32, c: i32) -> i32 {
    if b > 0 && a < min_v + b {
        min_v
    } else if b < 0 && a > max_v + b {
        max_v
    } else {
        a - b
    }
}

// - Misc maths helpers
// static inline Vector2D ImMin(const Vector2D& lhs, const Vector2D& rhs)                { return Vector2D(lhs.x < rhs.x ? lhs.x : rhs.x, lhs.y < rhs.y ? lhs.y : rhs.y); }
// pub fn ImMinVec2(lhs: &Vector2D, rhs: &Vector2D) -> Vector2D {
//     Vector2D {
//         x: if lhs.x < rhs.x { lhs.x } else {rhs.x},
//         y: if lhs.y < rhs.y {lhs.y} else { rhs.y}
//     }
// }

// static inline Vector2D ImMax(const Vector2D& lhs, const Vector2D& rhs)                { return Vector2D(lhs.x >= rhs.x ? lhs.x : rhs.x, lhs.y >= rhs.y ? lhs.y : rhs.y); }
// pub fn ImMaxVec2(lhs: &Vector2D, rhs: &Vector2D) -> Vector2D {
//     Vector2D {
//         x: if lhs.x >rhs.x {lhs.x} else {rhs.x},
//         y: if lhs.y > rhs.y {lhs.y} else {rhs.y}
//     }
// }

// static inline Vector2D ImClamp(const Vector2D& v, const Vector2D& mn, Vector2D mx)      { return Vector2D((v.x < mn.x) ? mn.x : (v.x > mx.x) ? mx.x : v.x, (v.y < mn.y) ? mn.y : (v.y > mx.y) ? mx.y : v.y); }
// pub fn ImClampVec2(v: &Vector2D, min_v: &Vector2D, max_v: &Vector2D) -> Vector2D {
//     Vector2D {
//         x: if v.x < min_v.x { min_v.x} else if v.x > max_v.x { max_v.x} else {v.x},
//         y: if v.y < min_v.y { min_v.y} else if v.y > max_v.y { max_v.y} else {v.y}
//     }
// }

// static inline Vector2D ImLerp(const Vector2D& a, const Vector2D& b, float t)          { return Vector2D(a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t); }
// pub fn ImLerpVec2(a: &Vector2D, b: &Vector2D, t: f32) -> Vector2D {
//     Imvec2 {
//         x: a.x + (b.x - a.x) * t,
//         y: a.y + (b.y - a.y) & t
//     }
// }

// static inline Vector2D ImLerp(const Vector2D& a, const Vector2D& b, const Vector2D& t)  { return Vector2D(a.x + (b.x - a.x) * t.x, a.y + (b.y - a.y) * t.y); }
// pub fn ImLerpVec22(a: &Vector2D, b: &Vector2D, t: &Vector2D) -> Vector2D {
//     Vector2D {
//         x: a.x + (b.x - a.x) * t.x,
//         y: a.y + (b.y - a.y) * t.y,
//     }
// }

// static inline float  ImSaturate(float f)                                        { return (f < 0.0) ? 0.0 : (f > 1.0) ? 1.0 : f; }
pub fn saturate_f32(f: f32) -> f32 {
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

// static inline int    ImModPositive(int a, int b)                                { return (a + b) % b; }
pub fn mod_positive(a: i32, b: i32) -> i32 {
    (a + b) % b
}

// static inline float  ImLinearSweep(float current, float target, float speed)    { if (current < target) return ImMin(current + speed, target); if (current > target) return ImMax(current - speed, target); return current; }
pub fn linear_sweep(current: f32, target: f32, speed: f32) -> f32 {
    if current < target {
        ImMin(current + speed, target)
    } else if current > target {
        ImMax(current - speed, target)
    } else {
        current
    }
}

// static inline Vector2D ImMul(const Vector2D& lhs, const Vector2D& rhs)                { return Vector2D(lhs.x * rhs.x, lhs.y * rhs.y); }
// pub fn ImMulVec2(lhs: &Vector2D, rhs: &Vector2D) -> Vector2D {
//     Vector2D {
//         x: lhs.x * rhs.x,
//         y: lhs.y * rhs.y
//     }
// }

// static inline bool   ImIsFloatAboveGuaranteedIntegerPrecision(float f)          { return f <= -16777216 || f >= 16777216; }
pub fn is_float_above_guranteed_integer_precision(f: f32) -> bool {
    f <= -16777216.0 || f >= 16777216.0
}
// IM_MSVC_RUNTIME_CHECKS_RESTORE


// #define IM_F32_TO_INT8_UNBOUND(_VAL)    (((_VAL) * 255.0 + ((_VAL)>=0 ? 0.5 : -0.5)))   // Unsaturated, for display purpose
pub fn f32_to_i8_unbound(x: f32) -> i8 {
    x * 255.0 + if x >= 0.0 { 0.5} else { -0.5 } as i8
}

// #define im_f32_to_int8_sat(_VAL)        ((ImSaturate(_VAL) * 255.0 + 0.5))               // Saturated, always output 0..255
pub fn im_f32_to_int8_sat(x: f32) -> i8 {
    saturate_f32(x) * 255.0 + 0.5 as i8
}

// #define IM_FLOOR(_VAL)                  ((float)(_VAL))                                    // f32::floor() is not inlined in MSVC debug builds
// #define IM_ROUND(_VAL)                  ((float)((_VAL) + 0.5))                           //
// pub fn IM_ROUND(x: f32) -> f32{
//     f32::round(x)
// }
