use std::ops::{Add, Div, Mul, Sub};

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

// static inline Vector2D operator+(const Vector2D& lhs, const Vector2D& rhs)            { return Vector2D(lhs.x + rhs.x, lhs.y + rhs.y); }
// static inline Vector2D operator-(const Vector2D& lhs, const Vector2D& rhs)            { return Vector2D(lhs.x - rhs.x, lhs.y - rhs.y); }
// static inline Vector2D operator*(const Vector2D& lhs, const Vector2D& rhs)            { return Vector2D(lhs.x * rhs.x, lhs.y * rhs.y); }

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
