#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiItemFlags;             // -> enum ImGuiItemFlags_          // Flags: for PushItemFlag()
pub type ImGuiItemFlags = c_int;

// Transient per-window flags, reset at the beginning of the frame. For child window, inherited from parent on first Begin().
// This is going to be exposed in imgui.h when stabilized enough.
// enum ImGuiItemFlags_
// {
// Controlled by user
pub const ImGuiItemFlags_None: ImGuiItemFlags = 0;
pub const ImGuiItemFlags_NoTabStop: ImGuiItemFlags = 1 << 0;
// false     // Disable keyboard tabbing (FIXME: should merge with _NoNav)
pub const ImGuiItemFlags_ButtonRepeat: ImGuiItemFlags = 1 << 1;
// false     // Button() will return true multiple times based on io.KeyRepeatDelay and io.KeyRepeatRate settings.
pub const ImGuiItemFlags_Disabled: ImGuiItemFlags = 1 << 2;
// false     // Disable interactions but doesn't affect visuals. See BeginDisabled()/EndDisabled(). See github.com/ocornut/imgui/issues/211
pub const ImGuiItemFlags_NoNav: ImGuiItemFlags = 1 << 3;
// false     // Disable keyboard/gamepad directional navigation (FIXME: should merge with _NoTabStop)
pub const ImGuiItemFlags_NoNavDefaultFocus: ImGuiItemFlags = 1 << 4;
// false     // Disable item being a candidate for default focus (e.g. used by title bar items)
pub const ImGuiItemFlags_SelectableDontClosePopup: ImGuiItemFlags = 1 << 5;
// false     // Disable MenuItem/Selectable() automatically closing their popup window
pub const ImGuiItemFlags_MixedValue: ImGuiItemFlags = 1 << 6;
// false     // [BETA] Represent a mixed/indeterminate value; generally multi-selection where values differ. Currently only supported by Checkbox() (later should support all sorts of widgets)
pub const ImGuiItemFlags_ReadOnly: ImGuiItemFlags = 1 << 7;  // false     // [ALPHA] Allow hovering interactions but underlying value is not changed.

// Controlled by widget code
pub const ImGuiItemFlags_Inputable: ImGuiItemFlags = 1 << 8;  // false     // [WIP] Auto-activate input mode when tab focused. Currently only used and supported by a few items before it becomes a generic feature.
// };
