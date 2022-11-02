use libc::c_float;
use std::mem;

#[derive(Default, Debug, Clone, Copy)]
pub struct ImVec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl ImVec4 {
    pub fn to_vec(&self) -> Vec<u8> {
        let mut out_vec: Vec<u8> = Vec::with_capacity(mem::size_of::<ImVec4>());
        let mut x_bytes: [u8; 4] = self.x.to_le_bytes();
        let mut y_bytes: [u8; 4] = self.y.to_le_bytes();
        let mut z_bytes: [u8; 4] = self.z.to_le_bytes();
        let mut w_bytes: [u8; 4] = self.w.to_le_bytes();
        out_vec.extend(x_bytes.iter());
        out_vec.extend(y_bytes.iter());
        out_vec.extend(z_bytes.iter());
        out_vec.extend(w_bytes.iter());

        out_vec
    }

    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn from_floats(x: c_float, y: c_float, z: c_float, w: c_float) -> Sef {
        Self { x, y, z, w }
    }
}

// static inline ImVec4 ImLerp(a: &ImVec4, b: &ImVec4, c_float t)          { return ImVec4(a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t, a.z + (b.z - a.z) * t, a.w + (b.w - a.w) * t); }
pub fn ImLerpVec4(a: &ImVec4, b: &ImVec4, t: c_float) -> ImVec4 {
    let x = a.x + (b.x - a.x) * t;
    let y = a.y + (b.y - a.y) * t;
    let z = a.z + (b.z - a.z) * t;
    let w = a.w + (b.w - a.w) * t;
    ImVec4::from_floats(x, y, z, w)
}

// static inline c_float  ImLengthSqr(lhs: &ImVec4)                             { return (lhs.x * lhs.x) + (lhs.y * lhs.y) + (lhs.z * lhs.z) + (lhs.w * lhs.w); }
pub fn ImLengthSqrVec4(lhs: &ImVec4) -> c_float {
    (lhs.x * lhs.x) + (lhs.y * lhs.y) + (lhs.z * lhs.z) + (lhs.w * lhs.w)
}
