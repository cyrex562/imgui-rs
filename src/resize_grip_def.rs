#![allow(non_upper_case_globals)]


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
    ImGuiResizeGripDef { CornerPosN: ImVec2::new(1.0, 1.0), InnderDir: ImVec2::new(-1, -1), AngleMin12: 0, AngleMax12: 3 },  // Lower-right
    ImGuiResizeGripDef { CornerPosN: ImVec2::new(0.0, 1.0), InnderDir: ImVec2::new(1, -1), AngleMin12: 3, AngleMax12: 6 },  // Lower-left
    ImGuiResizeGripDef { CornerPosN: ImVec2::new(0.0, 0.0), InnderDir: ImVec2::new(1, 1), AngleMin12: 6, AngleMax12: 9 },  // Upper-left (Unused)
    ImGuiResizeGripDef { CornerPosN: ImVec2::new(1.0, 0.0), InnderDir: ImVec2::new(-1, 1), AngleMin12: 9, AngleMax12: 12 }  // Upper-right (Unused)
];