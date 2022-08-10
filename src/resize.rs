use crate::vectors::vector_2d::Vector2D;

// data for resizing from corner
pub struct ResizeGripDef {
    // Vector2D  CornerPosN;
    pub corner_pos_n: Vector2D,
    // Vector2D  InnerDir;
    pub inner_dir: Vector2D,
    // int     AngleMin12, AngleMax12;
    pub angle_min12: i32,
    pub angle_max12: i32,
}

impl ResizeGripDef {
    pub fn new(corner_pos_n: Vector2D, inner_dir: Vector2D, angle_min12: i32, angle_max12: i32) -> Self {
        Self {
            corner_pos_n,
            inner_dir,
            angle_min12,
            angle_max12,
        }
    }
}

pub const RESIZE_GRIP_DEF: [ResizeGripDef; 4] = [
    ResizeGripDef::new(Vector2D::new(1f32, 1f32), Vector2D::new(-1f32, -1f32), 0, 3),  // Lower-right
    ResizeGripDef::new(Vector2D::new(0f32, 1f32), Vector2D::new(1f32, -1f32), 3, 6),  // Lower-left
    ResizeGripDef::new(Vector2D::new(0f32, 0f32), Vector2D::new(1f32, 1f32), 6, 9),  // Upper-left (Unused)
    ResizeGripDef::new(Vector2D::new(1f32, 0f32), Vector2D::new(-1f32, 1f32), 9, 12)  // Upper-right (Unused)
];
