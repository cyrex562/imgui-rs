#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiButtonFlags;       // -> enum ImGuiButtonFlags_     // Flags: for InvisibleButton()
pub type ImGuiButtonFlags = c_int;


// Flags for InvisibleButton() [extended in imgui_internal.h]
// enum ImGuiButtonFlags_
// {
pub const ImGuiButtonFlags_None: ImGuiButtonFlags = 0;
pub const ImGuiButtonFlags_MouseButtonLeft: ImGuiButtonFlags = 1 << 0;
// React on left mouse button (default)
pub const ImGuiButtonFlags_MouseButtonRight: ImGuiButtonFlags = 1 << 1;
// React on right mouse button
pub const ImGuiButtonFlags_MouseButtonMiddle: ImGuiButtonFlags = 1 << 2;   // React on center mouse button

// [Internal]
pub const ImGuiButtonFlags_MouseButtonMask_: ImGuiButtonFlags = ImGuiButtonFlags_MouseButtonLeft | ImGuiButtonFlags_MouseButtonRight | ImGuiButtonFlags_MouseButtonMiddle;
pub const ImGuiButtonFlags_MouseButtonDefault_: ImGuiButtonFlags = ImGuiButtonFlags_MouseButtonLeft;
// };

// Extend ImGuiButtonFlags_
// enum ImGuiButtonFlagsPrivate_
// {
pub const ImGuiButtonFlags_PressedOnClick: ImGuiButtonFlags = 1 << 4;
// return true on click (mouse down event)
pub const ImGuiButtonFlags_PressedOnClickRelease: ImGuiButtonFlags = 1 << 5;
// [Default] return true on click + release on same item <-- this is what the majority of Button are using
pub const ImGuiButtonFlags_PressedOnClickReleaseAnywhere: ImGuiButtonFlags = 1 << 6;
// return true on click + release even if the release event is not done while hovering the item
pub const ImGuiButtonFlags_PressedOnRelease: ImGuiButtonFlags = 1 << 7;
// return true on release (default requires click+release)
pub const ImGuiButtonFlags_PressedOnDoubleClick: ImGuiButtonFlags = 1 << 8;
// return true on double-click (default requires click+release)
pub const ImGuiButtonFlags_PressedOnDragDropHold: ImGuiButtonFlags = 1 << 9;
// return true when held into while we are drag and dropping another item (used by e.g. tree nodes, collapsing headers)
pub const ImGuiButtonFlags_Repeat: ImGuiButtonFlags = 1 << 10;
// hold to repeat
pub const ImGuiButtonFlags_FlattenChildren: ImGuiButtonFlags = 1 << 11;
// allow interactions even if a child window is overlapping
pub const ImGuiButtonFlags_AllowItemOverlap: ImGuiButtonFlags = 1 << 12;
// require previous frame HoveredId to either match id or be null before being usable, use along with SetItemAllowOverlap()
pub const ImGuiButtonFlags_DontClosePopups: ImGuiButtonFlags = 1 << 13;
// disable automatically closing parent popup on press // [UNUSED]
//ImGuiButtonFlags_Disabled             = 1 << 14,  // disable interactions -> use BeginDisabled() or ImGuiItemFlags_Disabled
pub const ImGuiButtonFlags_AlignTextBaseLine: ImGuiButtonFlags = 1 << 15;
// vertically align button to match text baseline - ButtonEx() only // FIXME: Should be removed and handled by SmallButton(), not possible currently because of DC.CursorPosPrevLine
pub const ImGuiButtonFlags_NoKeyModifiers: ImGuiButtonFlags = 1 << 16;
// disable mouse interaction if a key modifier is held
pub const ImGuiButtonFlags_NoHoldingActiveId: ImGuiButtonFlags = 1 << 17;
// don't set ActiveId while holding the mouse (ImGuiButtonFlags_PressedOnClick only)
pub const ImGuiButtonFlags_NoNavFocus: ImGuiButtonFlags = 1 << 18;
// don't override navigation focus when activated
pub const ImGuiButtonFlags_NoHoveredOnFocus: ImGuiButtonFlags = 1 << 19;
// don't report as hovered when nav focus is on this item
pub const ImGuiButtonFlags_PressedOnMask_: ImGuiButtonFlags = ImGuiButtonFlags_PressedOnClick | ImGuiButtonFlags_PressedOnClickRelease | ImGuiButtonFlags_PressedOnClickReleaseAnywhere | ImGuiButtonFlags_PressedOnRelease | ImGuiButtonFlags_PressedOnDoubleClick | ImGuiButtonFlags_PressedOnDragDropHold;
pub const ImGuiButtonFlags_PressedOnDefault_: ImGuiButtonFlags = ImGuiButtonFlags_PressedOnClickRelease;
// };
