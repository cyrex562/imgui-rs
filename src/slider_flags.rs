#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiSliderFlags;       // -> enum ImGuiSliderFlags_     // Flags: for DragFloat(), DragInt(), SliderFloat(), SliderInt() etc.
pub type ImGuiSliderFlags = c_int;

// Flags for DragFloat(), DragInt(), SliderFloat(), SliderInt() etc.
// We use the same sets of flags for DragXXX() and SliderXXX() functions as the features are the same and it makes it easier to swap them.
// enum ImGuiSliderFlags_
// {
pub const ImGuiSliderFlags_None: ImGuiSliderFlags = 0;
pub const ImGuiSliderFlags_AlwaysClamp: ImGuiSliderFlags = 1 << 4; // Clamp value to min/max bounds when input manually with CTRL+Click. By default CTRL+Click allows going out of bounds.
pub const ImGuiSliderFlags_Logarithmic: ImGuiSliderFlags = 1 << 5; // Make the widget logarithmic (linear otherwise). Consider using ImGuiSliderFlags_NoRoundToFormat with this if using a format-string with small amount of digits.
pub const ImGuiSliderFlags_NoRoundToFormat: ImGuiSliderFlags = 1 << 6; // Disable rounding underlying value to match precision of the display format string (e.g. %.3f values are rounded to those 3 digits)
pub const ImGuiSliderFlags_NoInput: ImGuiSliderFlags = 1 << 7; // Disable CTRL+Click or Enter key allowing to input text directly into the widget
pub const ImGuiSliderFlags_InvalidMask_: ImGuiSliderFlags = 0x7000000; // [Internal] We treat using those bits as being potentially a 'float power' argument from the previous API that has got miscast to this enum; and will trigger an assert if needed.

// Obsolete names (will be removed)
// #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
pub const ImGuiSliderFlags_ClampOnInput: ImGuiSliderFlags = ImGuiSliderFlags_AlwaysClamp; // [renamed in 1.79]
                                                                                          // #endif
                                                                                          // };

// Extend ImGuiSliderFlags_
// enum ImGuiSliderFlagsPrivate_
// {
pub const ImGuiSliderFlags_Vertical: ImGuiSliderFlags = 1 << 20; // Should this slider be orientated vertically?
pub const ImGuiSliderFlags_ReadOnly: ImGuiSliderFlags = 1 << 21;
// };
