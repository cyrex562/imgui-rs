#![allow(non_upper_case_globals)]


use libc::c_int;
use crate::core::vec2::Vector2;

// Data for resizing from corner
#[derive(Default,Debug,Copy,Clone)]
pub struct ImGuiResizeGripDef
{
    // ImVec2  CornerPosN;
    pub CornerPosN: Vector2,
    // ImVec2  InnerDir;
    pub InnerDir: Vector2,
    // c_int     AngleMin12, AngleMax12;
    pub AngleMin12: c_int,
    pub AngleMax12: c_int
}


pub const resize_grip_def: [ImGuiResizeGripDef; 4] = [
    ImGuiResizeGripDef { CornerPosN: Vector2::from_floats(1.0, 1.0), InnerDir: Vector2::from_floats(-1, -1), AngleMin12: 0, AngleMax12: 3 },  // Lower-right
    ImGuiResizeGripDef { CornerPosN: Vector2::from_floats(0.0, 1.0), InnerDir: Vector2::from_floats(1, -1), AngleMin12: 3, AngleMax12: 6 },  // Lower-left
    ImGuiResizeGripDef { CornerPosN: Vector2::from_floats(0.0, 0.0), InnerDir: Vector2::from_floats(1, 1), AngleMin12: 6, AngleMax12: 9 },  // Upper-left (Unused)
    ImGuiResizeGripDef { CornerPosN: Vector2::from_floats(1.0, 0.0), InnerDir: Vector2::from_floats(-1, 1), AngleMin12: 9, AngleMax12: 12 }  // Upper-right (Unused)
];
