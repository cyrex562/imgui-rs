
pub type ImGuiActivateFlags = i32;

pub const IM_GUI_ACTIVATE_FLAGS_NONE: ImGuiActivateFlags = 0;
// Favor activation that requires keyboard text input (e.g. for Slider/Drag). Default if keyboard is available.
pub const IM_GUI_ACTIVATE_FLAGS_PREFER_INPUT: ImGuiActivateFlags = 1;
// Favor activation for tweaking with arrows or gamepad (e.g. for Slider/Drag). Default if keyboard is not available.
pub const IM_GUI_ACTIVATE_FLAGS_PREFER_TWEAK: ImGuiActivateFlags = 2;
// Request widget to preserve state if it can (e.g. InputText will try to preserve cursor/selection)
pub const IM_GUI_ACTIVATE_FLAGS_TRY_TO_PRESERVE_STATE: ImGuiActivateFlags = 3;
