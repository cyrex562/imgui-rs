use std::ops::{Add, Sub};

// ImVec2: 2D vector used to store positions, sizes etc. [Compile-time configurable type]
// This is a frequently used type in the API. Consider using IM_VEC2_CLASS_EXTRA to create implicit cast from/to our preferred type.
// IM_MSVC_RUNTIME_CHECKS_OFF
#[derive(Default,Debug,Clone)]
pub struct ImVec2
{
    // float                                   x, y;
    pub x: f32,
    pub y: f32,

    // constexpr ImVec2()                      : x(0.0), y(0.0) { }
    // constexpr ImVec2(float _x, float _y)    : x(_x), y(_y) { }
    // float  operator[] (size_t idx) const    { IM_ASSERT(idx <= 1); return (&x)[idx]; }    // We very rarely use this [] operator, the assert overhead is fine.
    // float& operator[] (size_t idx)          { IM_ASSERT(idx <= 1); return (&x)[idx]; }    // We very rarely use this [] operator, the assert overhead is fine.
// #ifdef IM_VEC2_CLASS_EXTRA
//     IM_VEC2_CLASS_EXTRA     // Define additional constructors and implicit cast operators in imconfig.h to convert back and forth between your math types and ImVec2.
// #endif
}

impl ImVec2 {
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
}

impl Sub for ImVec2 {
    type Output = ImVec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl Add for ImVec2 {
    type Output = ImVec2;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}


pub fn ImLengthSqr(lhs: &ImVec2) -> f32 { return (lhs.x * lhs.x) + (lhs.y * lhs.y); }
