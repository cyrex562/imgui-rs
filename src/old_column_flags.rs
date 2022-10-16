#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiOldColumnFlags;        // -> enum ImGuiOldColumnFlags_     // Flags: for BeginColumns()
pub type ImGuiOldColumnFlags = c_int;

// Flags for internal's BeginColumns(). Prefix using BeginTable() nowadays!
// enum ImGuiOldColumnFlags_
// {
    pub const ImGuiOldColumnFlags_None: ImGuiOldColumnFlags = 0;
    pub const ImGuiOldColumnFlags_NoBorder: ImGuiOldColumnFlags = 1 << 0;   // Disable column dividers
    pub const ImGuiOldColumnFlags_NoResize: ImGuiOldColumnFlags = 1 << 1;   // Disable resizing columns when clicking on the dividers
    pub const ImGuiOldColumnFlags_NoPreserveWidths: ImGuiOldColumnFlags = 1 << 2;   // Disable column width preservation when adjusting columns
    pub const ImGuiOldColumnFlags_NoForceWithinWindow: ImGuiOldColumnFlags = 1 << 3;   // Disable forcing columns to fit within window
    pub const ImGuiOldColumnFlags_GrowParentContentsSize: ImGuiOldColumnFlags = 1 << 4;   // (WIP) Restore pre-1.51 behavior of extending the parent window contents size but _without affecting the columns width at all_. Will eventually remove.

    // Obsolete names (will be removed)
// #ifndef IMGUI_DISABLE_OBSOLETE_FUNCTIONS
//     ImGuiColumnsFlags_None                      = ImGuiOldColumnFlags_None;
//     ImGuiColumnsFlags_NoBorder                  = ImGuiOldColumnFlags_NoBorder;
//     ImGuiColumnsFlags_NoResize                  = ImGuiOldColumnFlags_NoResize;
//     ImGuiColumnsFlags_NoPreserveWidths          = ImGuiOldColumnFlags_NoPreserveWidths;
//     ImGuiColumnsFlags_NoForceWithinWindow       = ImGuiOldColumnFlags_NoForceWithinWindow;
//     ImGuiColumnsFlags_GrowParentContentsSize    = ImGuiOldColumnFlags_GrowParentContentsSize;
// #endif
// };
