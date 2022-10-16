#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiComboFlags;        // -> enum ImGuiComboFlags_      // Flags: for BeginCombo()
pub type ImGuiComboFlags = c_int;

pub const ImGuiComboFlags_None: ImGuiComboFlags = 0;
pub const ImGuiComboFlags_PopupAlignLeft: ImGuiComboFlags = 1 << 0; // Align the popup toward the left by default
pub const ImGuiComboFlags_HeightSmall: ImGuiComboFlags = 1 << 1; // Max ~4 items visible. Tip: If you want your combo popup to be a specific size you can use SetNextWindowSizeConstraints() prior to calling BeginCombo()
pub const ImGuiComboFlags_HeightRegular: ImGuiComboFlags = 1 << 2; // Max ~8 items visible (default)
pub const ImGuiComboFlags_HeightLarge: ImGuiComboFlags = 1 << 3; // Max ~20 items visible
pub const ImGuiComboFlags_HeightLargest: ImGuiComboFlags = 1 << 4; // As many fitting items as possible
pub const ImGuiComboFlags_NoArrowButton: ImGuiComboFlags = 1 << 5; // Display on the preview box without the square arrow button
pub const ImGuiComboFlags_NoPreview: ImGuiComboFlags = 1 << 6; // Display only a square arrow button
pub const ImGuiComboFlags_HeightMask_: ImGuiComboFlags = ImGuiComboFlags_HeightSmall
    | ImGuiComboFlags_HeightRegular
    | ImGuiComboFlags_HeightLarge
    | ImGuiComboFlags_HeightLargest;

// Extend ImGuiComboFlags_
// enum ImGuiComboFlagsPrivate_
// {
pub const ImGuiComboFlags_CustomPreview: ImGuiComboFlags = 1 << 20; // enable BeginComboPreview()
                                                                    // };
