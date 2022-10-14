#![allow(non_upper_case_globals)]

pub type ImGuiNavLayer = i32;

// pub enum ImGuiNavLayer {
//     ImGuiNavLayer_Main = 0,
//     // Main scrolling layer
//     ImGuiNavLayer_Menu = 1,
//     // Menu layer (access with Alt)
//     ImGuiNavLayer_COUNT,
// }
pub const ImGuiNavLayer_Main: ImGuiNavLayer = 0;
// Main scrolling layer
pub const ImGuiNavLayer_Menu: ImGuiNavLayer = 1;
// Menu layer (access with Alt)
pub const ImGuiNavLayer_COUNT: ImGuiNavLayer = 2;
