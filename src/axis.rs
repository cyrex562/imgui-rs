// // X/Y enums are fixed to 0/1 so they may be used to index ImVec2
#![allow(non_upper_case_globals)]
// enum ImGuiAxis
// {
//     ImGuiAxis_None = -1,
//     ImGuiAxis_X = 0,
//     ImGuiAxis_Y = 1
// };

pub type ImGuiAxis = i32;
pub const ImGuiAxis_None: i32 = -1;
pub const ImGuiAxis_X: i32 = 0;
pub const ImGuiAxis_Y: i32 = 1;
