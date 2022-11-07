#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiScrollFlags;           // -> enum ImGuiScrollFlags_        // Flags: for ScrollToItem() and navigation requests
pub type ImGuiScrollFlags = c_int;


// Early work-in-progress API for ScrollToItem()
// enum ImGuiScrollFlags_
// {
pub const ImGuiScrollFlags_None: ImGuiScrollFlags = 0;
pub const ImGuiScrollFlags_KeepVisibleEdgeX: ImGuiScrollFlags = 1 << 0;
// If item is not visible: scroll as little as possible on X axis to bring item back into view [default for X axis]
pub const ImGuiScrollFlags_KeepVisibleEdgeY: ImGuiScrollFlags = 1 << 1;
// If item is not visible: scroll as little as possible on Y axis to bring item back into view [default for Y axis for windows that are already visible]
pub const ImGuiScrollFlags_KeepVisibleCenterX: ImGuiScrollFlags = 1 << 2;
// If item is not visible: scroll to make the item centered on X axis [rarely used]
pub const ImGuiScrollFlags_KeepVisibleCenterY: ImGuiScrollFlags = 1 << 3;
// If item is not visible: scroll to make the item centered on Y axis
pub const ImGuiScrollFlags_AlwaysCenterX: ImGuiScrollFlags = 1 << 4;
// Always center the result item on X axis [rarely used]
pub const ImGuiScrollFlags_AlwaysCenterY: ImGuiScrollFlags = 1 << 5;
// Always center the result item on Y axis [default for Y axis for appearing window)
pub const ImGuiScrollFlags_NoScrollParent: ImGuiScrollFlags = 1 << 6;
// Disable forwarding scrolling to parent window if required to keep item/rect visible (only scroll window the function was applied to).
pub const ImGuiScrollFlags_MaskX_: ImGuiScrollFlags = ImGuiScrollFlags_KeepVisibleEdgeX | ImGuiScrollFlags_KeepVisibleCenterX | ImGuiScrollFlags_AlwaysCenterX;
pub const ImGuiScrollFlags_MaskY_: ImGuiScrollFlags = ImGuiScrollFlags_KeepVisibleEdgeY | ImGuiScrollFlags_KeepVisibleCenterY | ImGuiScrollFlags_AlwaysCenterY;
// };
