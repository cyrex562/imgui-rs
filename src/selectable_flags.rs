#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiSelectableFlags;   // -> enum ImGuiSelectableFlags_ // Flags: for Selectable()
pub type ImGuiSelectableFlags = c_int;
//
// // Flags for Selectable()
// enum ImGuiSelectableFlags_
// {
pub const ImGuiSelectableFlags_None: ImGuiSelectableFlags = 0;
pub const ImGuiSelectableFlags_DontClosePopups: ImGuiSelectableFlags = 1 << 0; // Clicking this don't close parent popup window
pub const ImGuiSelectableFlags_SpanAllColumns: ImGuiSelectableFlags = 1 << 1; // Selectable frame can span all columns (text will still fit in current column)
pub const ImGuiSelectableFlags_AllowDoubleClick: ImGuiSelectableFlags = 1 << 2; // Generate press events on double clicks too
pub const ImGuiSelectableFlags_Disabled: ImGuiSelectableFlags = 1 << 3; // Cannot be selected; display grayed out text
pub const ImGuiSelectableFlags_AllowItemOverlap: ImGuiSelectableFlags = 1 << 4; // (WIP) Hit testing to allow subsequent widgets to overlap this one
                                                                                // };
                                                                                //
                                                                                // // Extend ImGuiSelectableFlags_
                                                                                // enum ImGuiSelectableFlagsPrivate_
                                                                                // {
                                                                                // NB: need to be in sync with last value of ImGuiSelectableFlags_
pub const ImGuiSelectableFlags_NoHoldingActiveID: ImGuiSelectableFlags = 1 << 20;
pub const ImGuiSelectableFlags_SelectOnNav: ImGuiSelectableFlags = 1 << 21; // (WIP) Auto-select when moved into. This is not exposed in public API as to handle multi-select and modifiers we will need user to explicitly control focus scope. May be replaced with a BeginSelection() API.
pub const ImGuiSelectableFlags_SelectOnClick: ImGuiSelectableFlags = 1 << 22; // Override button behavior to react on Click (default is Click+Release)
pub const ImGuiSelectableFlags_SelectOnRelease: ImGuiSelectableFlags = 1 << 23; // Override button behavior to react on Release (default is Click+Release)
pub const ImGuiSelectableFlags_SpanAvailWidth: ImGuiSelectableFlags = 1 << 24; // Span all avail width even if we declared less for layout purpose. FIXME: We may be able to remove this (added in 6251d379; 2bcafc86 for menus)
pub const ImGuiSelectableFlags_DrawHoveredWhenHeld: ImGuiSelectableFlags = 1 << 25; // Always show active when held; even is not hovered. This concept could probably be renamed/formalized somehow.
pub const ImGuiSelectableFlags_SetNavIdOnHover: ImGuiSelectableFlags = 1 << 26; // Set Nav/Focus ID on mouse hover (used by MenuItem)
pub const ImGuiSelectableFlags_NoPadWithHalfSpacing: ImGuiSelectableFlags = 1 << 27; // Disable padding each side with ItemSpacing * 0.5
                                                                                     // };
