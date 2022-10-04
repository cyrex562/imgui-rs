#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

pub type ImGuiInputSource = i32;

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

pub const ImGuiInputSource_None: ImGuiInputSource = 0;
pub const ImGuiInputSource_Mouse: ImGuiInputSource = 1;
pub const ImGuiInputSource_Keyboard: ImGuiInputSource = 2;
pub const ImGuiInputSource_Gamepad: ImGuiInputSource = 3;
pub const ImGuiInputSource_Clipboard: ImGuiInputSource = 4;
// Currently only used by InputText()
pub const ImGuiInputSource_Nav: ImGuiInputSource = 5;
// Stored in g.ActiveIdSource only
pub const ImGuiInputSource_COUNT: ImGuiInputSource = 6;

pub const input_source_names: [&'static str; 6] = ["None", "Mouse", "Keyboard", "Gamepad", "Nav", "Clipboard"];
