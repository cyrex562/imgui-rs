

// Data for resizing from corner
#[derive(Default,Debug,Clone)]
struct ImGuiResizeGripDef
{
    // ImVec2  CornerPosN;
    pub CornerPosN: ImVec2,
    // ImVec2  InnerDir;
    pub InnerDir: ImVec2,
    // c_int     AngleMin12, AngleMax12;
    pub AngleMin12: c_int,
    pub AngleMax12: c_int
}


pub const  resize_grip_def:[ImGuiResizeGripDef;4] =
[
    ImGuiResizeGripDef{ ImVec2::New2(1f, 1f), ImVec2::New2(-1f, -1f), 0, 3 },  // Lower-right
    ImGuiResizeGripDef{ ImVec2::New2(0f, 1f), ImVec2::New2(1f, -1f), 3, 6 },  // Lower-left
    ImGuiResizeGripDef{ ImVec2::New2(0f, 0f), ImVec2::New2(+1, +1), 6, 9 },  // Upper-left (Unused)
    ImGuiResizeGripDef{ ImVec2(1, 0), ImVec2(-1, +1), 9, 12 }  // Upper-right (Unused)
];
