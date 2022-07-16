pub mod two_d;

use std::ops::{Add, Div, Mul, Sub};
use two_d::Vector2D;

// static inline Vector2D operator+(const Vector2D& lhs, const Vector2D& rhs)            { return Vector2D(lhs.x + rhs.x, lhs.y + rhs.y); }
// static inline Vector2D operator-(const Vector2D& lhs, const Vector2D& rhs)            { return Vector2D(lhs.x - rhs.x, lhs.y - rhs.y); }
// static inline Vector2D operator*(const Vector2D& lhs, const Vector2D& rhs)            { return Vector2D(lhs.x * rhs.x, lhs.y * rhs.y); }


// static inline Vector2D& operator*=(Vector2D& lhs, const float rhs)                  { lhs.x *= rhs; lhs.y *= rhs; return lhs; }
// static inline Vector2D& operator/=(Vector2D& lhs, const float rhs)                  { lhs.x /= rhs; lhs.y /= rhs; return lhs; }
// static inline Vector2D& operator+=(Vector2D& lhs, const Vector2D& rhs)                { lhs.x += rhs.x; lhs.y += rhs.y; return lhs; }
// static inline Vector2D& operator-=(Vector2D& lhs, const Vector2D& rhs)                { lhs.x -= rhs.x; lhs.y -= rhs.y; return lhs; }
// static inline Vector2D& operator*=(Vector2D& lhs, const Vector2D& rhs)                { lhs.x *= rhs.x; lhs.y *= rhs.y; return lhs; }
// static inline Vector2D& operator/=(Vector2D& lhs, const Vector2D& rhs)                { lhs.x /= rhs.x; lhs.y /= rhs.y; return lhs; }




pub fn ImLengthSqr(lhs: &Vector2D) -> f32 { return (lhs.x * lhs.x) + (lhs.y * lhs.y); }

#[derive(Default,Debug,Clone)]
pub struct Vector1D
{
    // float   x;
    pub x: f32,
    // constexpr ImVec1()         : x(0.0) { }

    // constexpr ImVec1(float _x) : x(_x) { }
}

impl Vector1D {
    pub fn new() -> Self {
        Self {
            x: 0.0
        }
    }

    pub fn new2(x: f32) -> Self {
        Self {
            x
        }
    }
}

/// Vector4D: 4D vector used to store clipping rectangles, colors etc. [Compile-time configurable type]
#[derive(Default,Debug,Clone)]
pub struct Vector4D
{
    // float                                                     x, y, z, w;
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
    // constexpr Vector4D()                                        : x(0.0), y(0.0), z(0.0), w(0.0) { }
    // constexpr Vector4D(float _x, float _y, float _z, float _w)  : x(_x), y(_y), z(_z), w(_w) { }
// #ifdef IM_VEC4_CLASS_EXTRA
//     IM_VEC4_CLASS_EXTRA     // Define additional constructors and implicit cast operators in imconfig.h to convert back and forth between your math types and Vector4D.
// #endif
}

impl Vector4D {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {
            x, y, z, w
        }
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
