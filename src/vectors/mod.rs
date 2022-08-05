pub mod vector_2d;

use std::ops::{Add, Div, Mul, Sub};
pub use vector_2d::Vector2D;

// static inline Vector2D operator+(const Vector2D& lhs, const Vector2D& rhs)            { return Vector2D(lhs.x + rhs.x, lhs.y + rhs.y); }
// static inline Vector2D operator-(const Vector2D& lhs, const Vector2D& rhs)            { return Vector2D(lhs.x - rhs.x, lhs.y - rhs.y); }
// static inline Vector2D operator*(const Vector2D& lhs, const Vector2D& rhs)            { return Vector2D(lhs.x * rhs.x, lhs.y * rhs.y); }

// static inline Vector2D& operator*=(Vector2D& lhs, let rhs)                  { lhs.x *= rhs; lhs.y *= rhs; return lhs; }
// static inline Vector2D& operator/=(Vector2D& lhs, let rhs)                  { lhs.x /= rhs; lhs.y /= rhs; return lhs; }
// static inline Vector2D& operator+=(Vector2D& lhs, const Vector2D& rhs)                { lhs.x += rhs.x; lhs.y += rhs.y; return lhs; }
// static inline Vector2D& operator-=(Vector2D& lhs, const Vector2D& rhs)                { lhs.x -= rhs.x; lhs.y -= rhs.y; return lhs; }
// static inline Vector2D& operator*=(Vector2D& lhs, const Vector2D& rhs)                { lhs.x *= rhs.x; lhs.y *= rhs.y; return lhs; }
// static inline Vector2D& operator/=(Vector2D& lhs, const Vector2D& rhs)                { lhs.x /= rhs.x; lhs.y /= rhs.y; return lhs; }

pub fn vec_length_sqr(lhs: &Vector2D) -> f32 {
    return (lhs.x * lhs.x) + (lhs.y * lhs.y);
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Vector1D {
    // float   x;
    pub x: f32,
    // constexpr Vector1D()         : x(0.0) { }

    // constexpr Vector1D(float _x) : x(_x) { }
}

impl Vector1D {
    pub fn new() -> Self {
        Self { x: 0.0 }
    }

    pub fn new2(x: f32) -> Self {
        Self { x }
    }
}

/// Vector4D: 4D vector used to store clipping rectangles, colors etc. [Compile-time configurable type]
#[derive(Default, Debug, Clone, Copy)]
pub struct Vector4D {
    // float                                                     x, y, z, w;
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32, // constexpr Vector4D()                                        : x(0.0), y(0.0), z(0.0), w(0.0) { }
                // constexpr Vector4D(float _x, float _y, float _z, float _w)  : x(_x), y(_y), z(_z), w(_w) { }
                // #ifdef IM_VEC4_CLASS_EXTRA
                //     IM_VEC4_CLASS_EXTRA     // Define additional constructors and implicit cast operators in imconfig.h to convert back and forth between your math types and Vector4D.
                // #endif
}

impl Vector4D {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn clear(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
        self.z = 0.0;
        self.w = 0.0;
    }
}

// static inline Vector4D operator+(const Vector4D& lhs, const Vector4D& rhs)            { return Vector4D(lhs.x + rhs.x, lhs.y + rhs.y, lhs.z + rhs.z, lhs.w + rhs.w); }
// static inline Vector4D operator-(const Vector4D& lhs, const Vector4D& rhs)            { return Vector4D(lhs.x - rhs.x, lhs.y - rhs.y, lhs.z - rhs.z, lhs.w - rhs.w); }
// static inline Vector4D operator*(const Vector4D& lhs, const Vector4D& rhs)            { return Vector4D(lhs.x * rhs.x, lhs.y * rhs.y, lhs.z * rhs.z, lhs.w * rhs.w); }

// static inline Vector4D ImLerp(const Vector4D& a, const Vector4D& b, float t)          { return Vector4D(a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t, a.z + (b.z - a.z) * t, a.w + (b.w - a.w) * t); }
pub fn lerp_vector4d(a: &Vector4D, b: &Vector4D, t: f32) -> Vector4D {
    Vector4D {
        x: a.x + (b.x - a.x) * t,
        y: a.y + (b.y - a.y) * t,
        z: a.z + (b.z - a.z) * t,
        w: a.w + (b.w - a.w) * t,
    }
}

// static inline float  ImLengthSqr(const Vector4D& lhs)                             { return (lhs.x * lhs.x) + (lhs.y * lhs.y) + (lhs.z * lhs.z) + (lhs.w * lhs.w); }
pub fn length_sqr_vector4d(lhs: &Vector4D) -> f32 {
    (lhs.x * lhs.x) + (lhs.y * lhs.y) + (lhs.z * lhs.z) + (lhs.w * lhs.w)
}
