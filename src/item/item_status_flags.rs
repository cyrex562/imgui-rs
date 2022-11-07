#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiItemStatusFlags;       // -> enum ImGuiItemStatusFlags_    // Flags: for DC.LastItemStatusFlags
pub type ImGuiItemStatusFlags = c_int;


// Storage for LastItem data
// enum ImGuiItemStatusFlags_
// {
    pub const ImGuiItemStatusFlags_None: ImGuiItemStatusFlags =  0;
    pub const ImGuiItemStatusFlags_HoveredRect: ImGuiItemStatusFlags =  1 << 0;   // Mouse position is within item rectangle (does NOT mean that the window is in correct z-order and can be hovered!; this is only one part of the most-common IsItemHovered test)
    pub const ImGuiItemStatusFlags_HasDisplayRect: ImGuiItemStatusFlags =  1 << 1;   // g.last_item_data.DisplayRect is valid
    pub const ImGuiItemStatusFlags_Edited: ImGuiItemStatusFlags =  1 << 2;   // Value exposed by item was edited in the current frame (should match the return: bool value of most widgets)
    pub const ImGuiItemStatusFlags_ToggledSelection: ImGuiItemStatusFlags =  1 << 3;   // Set when Selectable(); TreeNode() reports toggling a selection. We can't report "Selected"; only state changes; in order to easily handle clipping with less issues.
    pub const ImGuiItemStatusFlags_ToggledOpen: ImGuiItemStatusFlags =  1 << 4;   // Set when TreeNode() reports toggling their open state.
    pub const ImGuiItemStatusFlags_HasDeactivated: ImGuiItemStatusFlags =  1 << 5;   // Set if the widget/group is able to provide data for the ImGuiItemStatusFlags_Deactivated flag.
    pub const ImGuiItemStatusFlags_Deactivated: ImGuiItemStatusFlags =  1 << 6;   // Only valid if ImGuiItemStatusFlags_HasDeactivated is set.
    pub const ImGuiItemStatusFlags_HoveredWindow: ImGuiItemStatusFlags =  1 << 7;   // Override the HoveredWindow test to allow cross-window hover testing.
    pub const ImGuiItemStatusFlags_FocusedByTabbing: ImGuiItemStatusFlags =  1 << 8;   // Set when the Focusable item just got focused by Tabbing (FIXME: to be removed soon)

// #ifdef IMGUI_ENABLE_TEST_ENGINE
    pub const ImGuiItemStatusFlags_Openable: ImGuiItemStatusFlags =  1 << 20;  // Item is an openable (e.g. TreeNode)
    pub const ImGuiItemStatusFlags_Opened: ImGuiItemStatusFlags =  1 << 21;  //
    pub const ImGuiItemStatusFlags_Checkable: ImGuiItemStatusFlags =  1 << 22;  // Item is a checkable (e.g. CheckBox; MenuItem)
    pub const ImGuiItemStatusFlags_Checked: ImGuiItemStatusFlags =  1 << 23;  //
// #endif
// };
