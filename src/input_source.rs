#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

// pub enum ImGuiInputSource {
//     ImGuiInputSource_None = 0,
//     ImGuiInputSource_Mouse,
//     ImGuiInputSource_Keyboard,
//     ImGuiInputSource_Gamepad,
//     ImGuiInputSource_Clipboard,
//     // Currently only used by InputText()
//     ImGuiInputSource_Nav,
//     // Stored in g.ActiveIdSource only
//     ImGuiInputSource_COUNT,
// }

pub const ImGuiInputSource_None: i32 = 0;
pub const ImGuiInputSource_Mouse: i32 = 1;
pub const ImGuiInputSource_Keyboard: i32 = 2;
pub const ImGuiInputSource_Gamepad: i32 = 3;
pub const ImGuiInputSource_Clipboard: i32 = 4;
// Currently only used by InputText()
pub const ImGuiInputSource_Nav: i32 = 5;
// Stored in g.ActiveIdSource only
pub const ImGuiInputSource_COUNT: i32 = 6;
