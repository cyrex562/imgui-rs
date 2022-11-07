use crate::math_ops::ImRsqrtFloat;
use libc::{c_float, c_short};
use std::mem;
use std::ops::Index;

// ImVec2: 2D vector used to store positions, sizes etc. [Compile-time configurable type]
// This is a frequently used type in the API. Consider using IM_VEC2_CLASS_EXTRA to create implicit cast from/to our preferred type.
// IM_MSVC_RUNTIME_CHECKS_OFF
#[derive(Debug, Default, Clone, Copy)]
pub struct ImVec2 {
    pub x: f32,
    pub y: f32,
}

impl ImVec2 {
    pub fn from_floats(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn from_usizes(x: usize, y: usize) -> Self {
        Self {
            x: x as f32,
            y: y as f32,
        }
    }

    pub fn from_ints(x: i32, y: i32) -> Self {
        Self {
            x: x as f32,
            y: y as f32,
        }
    }
    pub fn to_vec(&mut self) -> Vec<u8> {
        let mut out = vec![];
        let elem_len = mem::size_of::<f32>();
        let x_bytes = self.x.to_le_bytes();
        let y_bytes = self.y.to_le_bytes();
        out.copy_from_slice()
    }
}

// Helper: ImVec2ih (2D vector, half-size integer, for long-term packed storage)
#[derive(Default, Debug, Clone)]
pub struct ImVec2ih {
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
        Self { x, y }
    }

    // constexpr explicit ImVec2ih(const rhs: &mut ImVec2) : x((c_short)rhs.x), y((c_short)rhs.y) {}
    pub fn new3(rhs: &ImVec2) -> Self {
        Self {
            x: rhs.x.clone() as c_short,
            y: rhs.y.clone() as c_short,
        }
    }
}

pub fn ImMinVec2(lhs: &mut ImVec2, rhs: &mut ImVec2) -> ImVec2 {
    let x = if lhs.x < rhs.x { lhs.x } else { rhs.x };
    let y = if lhs.y < rhs.y { lhs.y } else { rhs.y };
    ImVec2::from_floats(x, y)
}

pub fn ImMaxVec2(lhs: &mut ImVec2, rhs: &mut ImVec2) -> ImVec2 {
    let x = if lhs.x >= rhs.x { lhs.x } else { rhs.x };
    let y = if lhs.y >= rhs.y { lhs.y } else { rhs.y };
    ImVec2::from_floats(x, y)
}

// static inline ImClamp: ImVec2(v: &ImVec2, mn: &ImVec2, mx: ImVec2)      { return ImVec2::new((v.x < mn.x) ? mn.x : (v.x > mx.x) ? mx.x : v.x, (v.y < mn.y) ? mn.y : (v.y > mx.y) ? mx.y : v.y); }
pub fn ImClampVec2(v: &ImVec2, mn: &ImVec2, mx: &ImVec2) -> ImVec2 {
    let x = if v.x < mn.x {
        mn.x
    } else if v.x > mx.x {
        mx.x
    } else {
        v.x
    };
    let y = if v.y < mn.y {
        mn.y
    } else if v.y > mx.y {
        mn.y
    } else {
        v.y
    };
    ImVec2::from_floats(x, y)
}

// static inline ImLerp: ImVec2(a: &ImVec2, b: &ImVec2, f32 t)          { return ImVec2::new(a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t); }
pub fn ImLerpVec2(a: &ImVec2, b: &ImVec2, t: f32) -> ImVec2 {
    let x = a.x + (b.x - a.x) * t;
    let y = a.y + (b.y - a.y) * t;
    ImVec2::from_floats(x, y)
}

// static inline ImLerp: ImVec2(a: &ImVec2, b: &ImVec2, t: &ImVec2)  { return ImVec2::new(a.x + (b.x - a.x) * t.x, a.y + (b.y - a.y) * t.y); }
pub fn ImLerpVec22(a: &ImVec2, b: &ImVec2, t: &ImVec2) -> ImVec2 {
    let x = a.x + (b.x - a.x) * t.x;
    let y = a.y + (b.y - a.y) * t.y;
    ImVec2::from_floats(x, y)
}

// static inline f32  ImLengthSqr(lhs: &ImVec2)                             { return (lhs.x * lhs.x) + (lhs.y * lhs.y); }
pub fn ImLengthSqrVec2(lhs: &ImVec2) -> f32 {
    (lhs.x * lhs.x) + (lhs.y * lhs.y)
}

pub fn ImInvLength(lhs: &ImVec2, fail_value: f32) -> f32 {
    let d = (lhs.x * lhs.x) + (lhs.y * lhs.y);
    if d > 0.0 {
        ImRsqrtFloat(d)
    } else {
        fail_value
    }
}

pub fn ImFloorVec2(v: &ImVec2) -> ImVec2 {
    ImVec2::from_floats(v.x.floor(), v.y.floor())
}

pub fn ImDotVec2(a: &ImVec2, b: &ImVec2) -> f32 {
    a.x * b.x + a.y * b.y
}

pub fn ImRotate(v: &ImVec2, cos_a: f32, sin_a: f32) -> ImVec2 {
    ImVec2::from_floats(v.x * cos_a - v.y * sin_a, v.x * sin_a - v.y * cos_a)
}

pub fn ImMul(lhs: &ImVec2, rhs: &ImVec2) -> ImVec2 {
    ImVec2::from_floats(lhs.x * rhs.x, lhs.y * rhs.y)
}
