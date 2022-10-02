#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiComboFlags;        // -> enum ImGuiComboFlags_      // Flags: for BeginCombo()
pub type ImGuiComboFlags = c_int;



// Extend ImGuiComboFlags_
// enum ImGuiComboFlagsPrivate_
// {
pub const    ImGuiComboFlags_CustomPreview: ImGuiComboFlags           = 1 << 20;  // enable BeginComboPreview()
// };
