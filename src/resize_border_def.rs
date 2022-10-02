#![allow(non_upper_case_globals)]

#[derive(Default, Debug, Copy, Clone)]
pub struct ImGuiResizeBorderDef {
    // ImVec2 InnerDir;
    pub InnerDir: ImVec2,
    // ImVec2 SegmentN1, SegmentN2;
    pub SegmentN1: ImVec2,
    pub SegmentN2: ImVec2,
    // c_float  OuterAngle;
    pub OuterAngle: c_float,
}

// static const ImGuiResizeBorderDef resize_border_def[4] =
pub const resize_border_def: [ImGuiResizeBorderDef;4] =
[
    ImGuiResizeBorderDef{ InnerDir: ImVec2::new2(1, 0), SegmentN1: ImVec2(0, 1), SegmentN2: ImVec2(0, 0), OuterAngle: IM_PI * 1f32 }, // Left
    ImGuiResizeBorderDef{ InnerDir: ImVec2(-1, 0), SegmentN1: ImVec2(1, 0), SegmentN2: ImVec2(1, 1), OuterAngle: IM_PI * 0.00f32 }, // Right
    ImGuiResizeBorderDef{ InnerDir: ImVec2(0, 1), SegmentN1: ImVec2(0, 0), SegmentN2: ImVec2(1, 0), OuterAngle: IM_PI * 1.50f32 }, // Up
    ImGuiResizeBorderDef{ InnerDir: mVec2(0, -1), SegmentN1: ImVec2(1, 1), SegmentN2: ImVec2(0, 1), OuterAngle: IM_PI * 0.50f32 }  // Down
];
