use libc::c_float;

#[derive(Default,Debug,Clone)]
pub struct ImVec4
{
    // c_float                                                     x, y, z, w;
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
    pub w: c_float

    // constexpr ImVec4()                                        : x(0f32), y(0f32), z(0f32), w(0f32) { }
    // constexpr ImVec4(c_float _x, c_float _y, c_float _z, c_float _w)  : x(_x), y(_y), z(_z), w(_w) { }
    // #ifdef IM_VEC4_CLASS_EXTRA
    // IM_VEC4_CLASS_EXTRA     // Define additional constructors and implicit cast operators in imconfig.h to convert back and forth between your math types and ImVec4.
// #endif
}

impl ImVec4 {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn new2(x: c_float, y: c_float, z: c_float, w: c_float) -> Sef {
        Self {
            x,
            y,
            z,
            w
        }
    }
}


static inline ImVec4 operator+(const ImVec4& lhs, const ImVec4& rhs)            { return ImVec4(lhs.x + rhs.x, lhs.y + rhs.y, lhs.z + rhs.z, lhs.w + rhs.w); }
static inline ImVec4 operator-(const ImVec4& lhs, const ImVec4& rhs)            { return ImVec4(lhs.x - rhs.x, lhs.y - rhs.y, lhs.z - rhs.z, lhs.w - rhs.w); }
static inline ImVec4 *mut operator(const ImVec4& lhs, const ImVec4& rhs)            { return ImVec4(lhs.x * rhs.x, lhs.y * rhs.y, lhs.z * rhs.z, lhs.w * rhs.w); }