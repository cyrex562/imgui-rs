#![allow(non_upper_case_globals)]

use libc::c_float;
use crate::core::vec2::Vector2;

// Data for resizing from borders
#[derive(Default,Debug,Copy, Clone)]
pub struct ImGuiResizeBorderDef
{
    // InnerDir: ImVec2;
    pub InnerDir: Vector2,
    // SegmentN1: ImVec2, SegmentN2;OuterAngle: c_float;
    pub SegmentN1: Vector2,
    pub SegmentN2: Vector2,
    pub OuterAngle: c_float
}

pub const resize_border_def: [ImGuiResizeBorderDef;4] =
[
    ImGuiResizeBorderDef{ InnerDir: Vector2::from_floats(1.0, 0.0), SegmentN1: Vector2::from_floats(0.0, 1.0), SegmentN2: Vector2::from_floats(0.0, 0.0), OuterAngle:IM_PI * 1.0 }, // Left
    ImGuiResizeBorderDef{ InnerDir: Vector2::from_floats(-1.0, 0.0), SegmentN1: Vector2::from_floats(1.0, 0.0), SegmentN2: Vector2::from_floats(1.0, 1.0), OuterAngle:IM_PI * 0.0 }, // Right
    ImGuiResizeBorderDef{ InnerDir: Vector2::from_floats(0.0, 1.0), SegmentN1: Vector2::from_floats(0.0, 0.0), SegmentN2: Vector2::from_floats(1.0, 0.0), OuterAngle:IM_PI * 1.5 }, // Up
    ImGuiResizeBorderDef{ InnerDir: Vector2::from_floats(0.0, -1.0), SegmentN1: Vector2::from_floats(1.0, 1.0), SegmentN2: Vector2::from_floats(0.0, 1.0), OuterAngle:IM_PI * 0.5 }  // Down
];
