#![allow(non_camel_case_types)]

pub enum ImGuiInputSource {
    ImGuiInputSource_None = 0,
    ImGuiInputSource_Mouse,
    ImGuiInputSource_Keyboard,
    ImGuiInputSource_Gamepad,
    ImGuiInputSource_Clipboard,
    // Currently only used by InputText()
    ImGuiInputSource_Nav,
    // Stored in g.ActiveIdSource only
    ImGuiInputSource_COUNT,
}


pub enum ImGuiNavLayer {
    ImGuiNavLayer_Main = 0,
    // Main scrolling layer
    ImGuiNavLayer_Menu = 1,
    // Menu layer (access with Alt)
    ImGuiNavLayer_COUNT,
}