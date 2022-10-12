#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiActivateFlags;         // -> enum ImGuiActivateFlags_      // Flags: for navigation/focus function (will be for ActivateItem() later)
pub type ImGuiActivateFlags = c_int;

// enum ImGuiActivateFlags_
// {
pub const ImGuiActivateFlags_None: ImGuiActivateFlags = 0;
pub const ImGuiActivateFlags_PreferInput: ImGuiActivateFlags = 1 << 0;
// Favor activation that requires keyboard text input (e.g. for Slider/Drag). Default if keyboard is available.
pub const ImGuiActivateFlags_PreferTweak: ImGuiActivateFlags = 1 << 1;
// Favor activation for tweaking with arrows or gamepad (e.g. for Slider/Drag). Default if keyboard is not available.
pub const ImGuiActivateFlags_TryToPreserveState: ImGuiActivateFlags = 1 << 2;       // Request widget to preserve state if it can (e.g. InputText will try to preserve cursor/selection)
// };
