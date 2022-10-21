#![allow(non_upper_case_globals)]


use libc::c_int;
use crate::vec2::ImVec2;

// Data for resizing from corner
#[derive(Default,Debug,Copy,Clone)]
pub struct ImGuiResizeGripDef
{
    // ImVec2  CornerPosN;
    pub CornerPosN: ImVec2,
    // ImVec2  InnerDir;
    pub InnerDir: ImVec2,
    // c_int     AngleMin12, AngleMax12;
    pub AngleMin12: c_int,
    pub AngleMax12: c_int
}


pub const resize_grip_def: [ImGuiResizeGripDef; 4] = [
    ImGuiResizeGripDef { CornerPosN: ImVec2::from_floats(1.0, 1.0), InnerDir: ImVec2::from_floats(-1, -1), AngleMin12: 0, AngleMax12: 3 },  // Lower-right
    ImGuiResizeGripDef { CornerPosN: ImVec2::from_floats(0.0, 1.0), InnerDir: ImVec2::from_floats(1, -1), AngleMin12: 3, AngleMax12: 6 },  // Lower-left
    ImGuiResizeGripDef { CornerPosN: ImVec2::from_floats(0.0, 0.0), InnerDir: ImVec2::from_floats(1, 1), AngleMin12: 6, AngleMax12: 9 },  // Upper-left (Unused)
    ImGuiResizeGripDef { CornerPosN: ImVec2::from_floats(1.0, 0.0), InnerDir: ImVec2::from_floats(-1, 1), AngleMin12: 9, AngleMax12: 12 }  // Upper-right (Unused)
];
