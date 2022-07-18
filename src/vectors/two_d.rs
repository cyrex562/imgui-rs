use std::ops::{Add, Div, Mul, Sub};
use crate::math;

// Vector2D: 2D vector used to store positions, sizes etc. [Compile-time configurable type]
// This is a frequently used type in the API. Consider using IM_VEC2_CLASS_EXTRA to create implicit cast from/to our preferred type.
// IM_MSVC_RUNTIME_CHECKS_OFF
#[derive(Default,Debug,Clone)]
pub struct Vector2D
{
    // float                                   x, y;
    pub x: f32,
    pub y: f32,

    // constexpr Vector2D()                      : x(0.0), y(0.0) { }
    // constexpr Vector2D(float _x, float _y)    : x(_x), y(_y) { }
    // float  operator[] (size_t idx) const    { IM_ASSERT(idx <= 1); return (&x)[idx]; }    // We very rarely use this [] operator, the assert overhead is fine.
    // float& operator[] (size_t idx)          { IM_ASSERT(idx <= 1); return (&x)[idx]; }    // We very rarely use this [] operator, the assert overhead is fine.
// #ifdef IM_VEC2_CLASS_EXTRA
//     IM_VEC2_CLASS_EXTRA     // Define additional constructors and implicit cast operators in imconfig.h to convert back and forth between your math types and Vector2D.
// #endif
}

impl Vector2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y
        }
    }

    pub fn new2() -> Self {
        Self {
            x: 0.0,
            y: 0.0
        }
    }

    pub fn clear(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
    }

    pub fn floor(item: Self) -> Self {
        Self {
            x: f32::floor(item.x),
            y: f32::floor(item.y)
        }
    }

    pub fn min(&self, rhs: &Self) -> Self {
        Self {
            x: if self.x < rhs.x { self.x} else { rhs.x },
            y: if self.y < rhs.y { self.y} else {rhs.y}
        }
    }

    pub fn clamp(&self, min_v: &Self, max_v: &Self) -> Self {
        Self {
            x: if self.x < min_v.x { min_v.x} else if  v.x > max_v.x { max_v.x} else {self.x},
            y: if self.y < min_v.y { min_v.y} else if v.y > max_v.y { max_v.y} else {self.y}
        }
    }

    pub fn lerp(&self, rhs: &Self, t: f32) -> Self {
        Self {
            x: self.x + (rhs.x - self.x) * t,
            y: self.y + (rhs.y - self.y) & t,
        }
    }

    pub fn lerp2(&self, b: &Self, t: &Self) -> Self {
        Self {
            x: self.x + (b.x - self.x) * t.x,
            y: self.y + (b.y - self.y) * t.y,
        }
    }

    pub fn max(&self, rhs: &Self) -> Self {
        Self {
            x: if self.x > rhs.x { self.x} else {rhs.x},
            y: if self.y > rhs.y { self.y} else {rhs.y}
        }
    }
}


impl Sub for Vector2D {
    type Output = Vector2D;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl Add for Vector2D {
    type Output = Vector2D;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Mul<f32> for Vector2D {
    type Output = Vector2D;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl Div<f32> for Vector2D {
    type Output = Vector2D;
    // static inline Vector2D operator/(const Vector2D& lhs, const float rhs)              { return Vector2D(lhs.x / rhs, lhs.y / rhs); }
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs
        }
    }
}

impl Mul for Vector2D {
    type Output = Vector2D;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
}

// static inline Vector2D operator/(const Vector2D& lhs, const Vector2D& rhs)            { return Vector2D(lhs.x / rhs.x, lhs.y / rhs.y); }
impl Div for Vector2D {
    type Output = Vector2D;
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y
        }
    }
}

// static inline float  ImInvLength(const Vector2D& lhs, float fail_value)           { float d = (lhs.x * lhs.x) + (lhs.y * lhs.y); if (d > 0.0) return ImRsqrt(d); return fail_value; }
pub fn inv_length(lhs: &Vector2D, fail_value: f32) -> f32 {
    let mut d = (lhs.x * lhs.x) +  (lhs.y * lhs.y);
    if d > 0.0 {
        return math::r_sqrt(d)
    }
    fail_value
}

// static inline float  ImDot(const Vector2D& a, const Vector2D& b)                    { return a.x * b.x + a.y * b.y; }
pub fn dot_vector_2d(a: &Vector2D, b: &Vector2D) -> f32 {
    a.x * b.x + a.y * b.y
}

// static inline Vector2D ImRotate(const Vector2D& v, float cos_a, float sin_a)        { return Vector2D(v.x * cos_a - v.y * sin_a, v.x * sin_a + v.y * cos_a); }
pub fn rorate_vector2d(v: &Vector2D, cos_a: f32, sin_a: f32) -> Vector2D {
    Vector2D{
        x: v.x * cos_a - v.y * sin_a,
        y: v.x * sin_a + v.y * cos_a
    }
}
