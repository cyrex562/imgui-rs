use crate::vec_nd::DimgVec2D;

// Vertex layout
// #ifndef IMGUI_OVERRIDE_DRAWVERT_STRUCT_LAYOUT
#[derive(Debug,Clone,Default)]
pub struct DimgDrawVert
{
    pub pos: DimgVec2D,
    pub uv: DimgVec2D,
    pub col: u32,
}
