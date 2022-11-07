use libc::c_float;

// Helper: ImVec1 (1D vector)
// (this odd construct is used to facilitate the transition between 1D and 2D, and the maintenance of some branches/patches)
// IM_MSVC_RUNTIME_CHECKS_OFF
#[derive(Default, Debug, Clone, Copy)]
pub struct ImVec1 {
    pub x: c_float,
}

impl ImVec1 {
    // constexpr ImVec1()         : x(0.0) { }
    pub fn new() -> Self {
        Self { x: 0.0 }
    }
    // constexpr ImVec1(c_float _x) : x(_x) { }
    pub fn new2(x: c_float) -> Self {
        Self { x }
    }
}
