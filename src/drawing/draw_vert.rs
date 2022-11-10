use crate::core::vec2::Vector2;

// Vertex layout
// #ifndef IMGUI_OVERRIDE_DRAWVERT_STRUCT_LAYOUT
#[derive(Default, Debug, Clone, Copy)]
pub struct DrawVertex {
    pub pos: Vector2,
    pub uv: Vector2,
    pub col: u32,
}
