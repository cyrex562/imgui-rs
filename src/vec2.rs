use std::ops::Index;
use libc::{c_float, c_short};

// ImVec2: 2D vector used to store positions, sizes etc. [Compile-time configurable type]
// This is a frequently used type in the API. Consider using IM_VEC2_CLASS_EXTRA to create implicit cast from/to our preferred type.
// IM_MSVC_RUNTIME_CHECKS_OFF
#[derive(Debug,Default,Clone)]
pub struct ImVec2
{
    // float                                   x, y;
    pub x: c_float,
    pub y: c_float

// #ifdef IM_VEC2_CLASS_EXTRA
// IM_VEC2_CLASS_EXTRA     // Define additional constructors and implicit cast operators in imconfig.h to convert back and forth between your math types and ImVec2.
// #endif
}

impl ImVec2 {
    // constexpr ImVec2()                      : x(0f32), y(0f32) { }
    pub fn new() -> Self {
        Self {
            x: 0f32,
            y: 0f32
        }
    }

    // constexpr ImVec2(float _x, float _y)    : x(_x), y(_y) { }
    pub fn new2(x: c_float, y: c_float) -> Self {
        Self {
            x,
            y
        }
    }

    // float  operator[] (size_t idx) const    { IM_ASSERT(idx <= 1); return (&x)[idx]; }    // We very rarely use this [] operator, the assert overhead is fine.
// float& operator[] (size_t idx)          { IM_ASSERT(idx <= 1); return (&x)[idx]; }    // We very rarely use this [] operator, the assert overhead is fine.
}


// Helper: ImVec2ih (2D vector, half-size integer, for long-term packed storage)
#[derive(Default,Debug,Clone)]
pub struct ImVec2ih
{
    // c_short   x, y;
    pub x: c_short,
    pub y: c_short,
    
}

impl ImVec2ih {
    // constexpr ImVec2ih()                           : x(0), y(0) {}
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }    
    
    // constexpr ImVec2ih(c_short _x, c_short _y)         : x(_x), y(_y) {}
    pub fn new2(x: c_short, y: c_short) -> Self {
        Self {
            x,
            y
        }
    }
    
    
    // constexpr explicit ImVec2ih(const ImVec2& rhs) : x((c_short)rhs.x), y((c_short)rhs.y) {}
    pub fn new3(rhs: &ImVec2) -> Self {
        Self {
            x: rhs.x.clone() as c_short,
            y: rhs.y.clone() as c_short
        }
    }


}