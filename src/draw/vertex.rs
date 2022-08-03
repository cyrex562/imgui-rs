use crate::vectors::vector_2d::Vector2D;

// Vertex layout
// #ifndef IMGUI_OVERRIDE_DRAWVERT_STRUCT_LAYOUT
#[derive(Debug,Clone,Default)]
pub struct DrawVertex
{
    pub position: Vector2D,
    pub uv: Vector2D,
    pub color: u32,
}
