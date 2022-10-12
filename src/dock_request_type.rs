// enum ImGuiDockRequestType
#![allow(non_upper_case_globals)]
// {
//     ImGuiDockRequestType_None = 0,
//     ImGuiDockRequestType_Dock,
//     ImGuiDockRequestType_Undock,
//     ImGuiDockRequestType_Split                  // Split is the same as Dock but without a DockPayload
// };

pub type ImGuiDockRequestType = i32;

pub const ImGuiDockRequestType_None: i32 = 0;
pub const ImGuiDockRequestType_Dock: i32 = 1;
pub const ImGuiDockRequestType_Undock: i32 = 2;
pub const ImGuiDockRequestType_Split: i32 = 3;                  // Split is the same as Dock but without a DockPayload
