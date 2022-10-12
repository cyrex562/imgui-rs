#![allow(non_upper_case_globals)]

use libc::c_int;

pub type ImGuiActivateFlags = c_int;

pub const ImGuiActivateFlags_None: ImGuiActivateFlags = 0;
// Favor activation that requires keyboard text input (e.g. for Slider/Drag). Default if keyboard is available.
pub const ImGuiActivateFlags_PreferInput: ImGuiActivateFlags = 1;
// Favor activation for tweaking with arrows or gamepad (e.g. for Slider/Drag). Default if keyboard is not available.
pub const ImGuiActivateFlags_PreferTweak: ImGuiActivateFlags = 2;
// Request widget to preserve state if it can (e.g. InputText will try to preserve cursor/selection)
pub const ImGuiActivateFlags_TryToPreserveState: ImGuiActivateFlags = 3;
