#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiTabItemFlags;      // -> enum ImGuiTabItemFlags_    // Flags: for BeginTabItem()
pub type ImGuiTabItemFlags = c_int;

// Flags for BeginTabItem()
// enum ImGuiTabItemFlags_
// {
    pub const ImGuiTabItemFlags_None: ImGuiTabItemFlags =  0;
    pub const ImGuiTabItemFlags_UnsavedDocument: ImGuiTabItemFlags =  1 << 0;   // Display a dot next to the title + tab is selected when clicking the X + closure is not assumed (will wait for user to stop submitting the tab). Otherwise closure is assumed when pressing the X, so if you keep submitting the tab may reappear at end of tab bar.
    pub const ImGuiTabItemFlags_SetSelected: ImGuiTabItemFlags =  1 << 1;   // Trigger flag to programmatically make the tab selected when calling BeginTabItem()
    pub const ImGuiTabItemFlags_NoCloseWithMiddleMouseButton: ImGuiTabItemFlags =  1 << 2;   // Disable behavior of closing tabs (that are submitted with p_open != NULL) with middle mouse button. You can still repro this behavior on user's side with if (IsItemHovered() && IsMouseClicked(2)) *p_open = false.
    pub const ImGuiTabItemFlags_NoPushId: ImGuiTabItemFlags =  1 << 3;   // Don't call PushID(tab.ID)/PopID() on BeginTabItem()/EndTabItem()
    pub const ImGuiTabItemFlags_NoTooltip: ImGuiTabItemFlags =  1 << 4;   // Disable tooltip for the given tab
    pub const ImGuiTabItemFlags_NoReorder: ImGuiTabItemFlags =  1 << 5;   // Disable reordering this tab or having another tab cross over this tab
    pub const ImGuiTabItemFlags_Leading: ImGuiTabItemFlags =  1 << 6;   // Enforce the tab position to the left of the tab bar (after the tab list popup button)
    pub const ImGuiTabItemFlags_Trailing: ImGuiTabItemFlags =  1 << 7;   // Enforce the tab position to the right of the tab bar (before the scrolling buttons)
// };


// Extend ImGuiTabItemFlags_
// enum ImGuiTabItemFlagsPrivate_
// {
    pub const ImGuiTabItemFlags_SectionMask_: ImGuiTabItemFlags              = ImGuiTabItemFlags_Leading | ImGuiTabItemFlags_Trailing;
    pub const ImGuiTabItemFlags_NoCloseButton   : ImGuiTabItemFlags          = 1 << 20;  // Track whether p_open was set or not (we'll need this info on the next frame to recompute ContentWidth during layout)
    pub const ImGuiTabItemFlags_Button  : ImGuiTabItemFlags                  = 1 << 21;  // Used by TabItemButton, change the tab item behavior to mimic a button
    pub const ImGuiTabItemFlags_Unsorted : ImGuiTabItemFlags                 = 1 << 22;  // [Docking] Trailing tabs with the _Unsorted flag will be sorted based on the DockOrder of their Window.
    pub const ImGuiTabItemFlags_Preview    : ImGuiTabItemFlags               = 1 << 23;  // [Docking] Display tab shape for docking preview (height is adjusted slightly to compensate for the yet missing tab bar)
// };
