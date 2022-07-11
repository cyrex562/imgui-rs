use crate::vectors::Vector2D;

// Vertex layout
// #ifndef IMGUI_OVERRIDE_DRAWVERT_STRUCT_LAYOUT
#[derive(Debug,Clone,Default)]
pub struct DrawVertex
{
    pub pos: Vector2D,
    pub uv: Vector2D,
    pub col: u32,
}
