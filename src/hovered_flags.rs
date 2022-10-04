#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiHoveredFlags;      // -> enum ImGuiHoveredFlags_    // Flags: for IsItemHovered(), IsWindowHovered() etc.
pub type ImGuiHoveredFlags = c_int;

// enum ImGuiHoveredFlags_
// {
pub const ImGuiHoveredFlags_None: ImGuiHoveredFlags = 0;
// Return true if directly over the item/window; not obstructed by another window; not obstructed by an active popup or modal blocking inputs under them.
pub const ImGuiHoveredFlags_ChildWindows: ImGuiHoveredFlags = 1 << 0;
// IsWindowHovered() only: Return true if any children of the window is hovered
pub const ImGuiHoveredFlags_RootWindow: ImGuiHoveredFlags = 1 << 1;
// IsWindowHovered() only: Test from root window (top most parent of the current hierarchy)
pub const ImGuiHoveredFlags_AnyWindow: ImGuiHoveredFlags = 1 << 2;
// IsWindowHovered() only: Return true if any window is hovered
pub const ImGuiHoveredFlags_NoPopupHierarchy: ImGuiHoveredFlags = 1 << 3;
// IsWindowHovered() only: Do not consider popup hierarchy (do not treat popup emitter as parent of popup) (when used with _ChildWindows or _RootWindow)
pub const ImGuiHoveredFlags_DockHierarchy: ImGuiHoveredFlags = 1 << 4;
// IsWindowHovered() only: Consider docking hierarchy (treat dockspace host as parent of docked window) (when used with _ChildWindows or _RootWindow)
pub const ImGuiHoveredFlags_AllowWhenBlockedByPopup: ImGuiHoveredFlags = 1 << 5;
// Return true even if a popup window is normally blocking access to this item/window
//pub const ImGuiHoveredFlags_AllowWhenBlockedByModal: ImGuiHoveredFlags =  1 << 6;   // Return true even if a modal popup window is normally blocking access to this item/window. FIXME-TODO: Unavailable yet.
pub const ImGuiHoveredFlags_AllowWhenBlockedByActiveItem: ImGuiHoveredFlags = 1 << 7;
// Return true even if an active item is blocking access to this item/window. Useful for Drag and Drop patterns.
pub const ImGuiHoveredFlags_AllowWhenOverlapped: ImGuiHoveredFlags = 1 << 8;
// IsItemHovered() only: Return true even if the position is obstructed or overlapped by another window
pub const ImGuiHoveredFlags_AllowWhenDisabled: ImGuiHoveredFlags = 1 << 9;
// IsItemHovered() only: Return true even if the item is disabled
pub const ImGuiHoveredFlags_NoNavOverride: ImGuiHoveredFlags = 1 << 10;
// Disable using gamepad/keyboard navigation state when active; always query mouse.
pub const ImGuiHoveredFlags_RectOnly: ImGuiHoveredFlags = ImGuiHoveredFlags_AllowWhenBlockedByPopup | ImGuiHoveredFlags_AllowWhenBlockedByActiveItem | ImGuiHoveredFlags_AllowWhenOverlapped;
pub const ImGuiHoveredFlags_RootAndChildWindows: ImGuiHoveredFlags = ImGuiHoveredFlags_RootWindow | ImGuiHoveredFlags_ChildWindows;

// Hovering delays (for tooltips)
pub const ImGuiHoveredFlags_DelayNormal: ImGuiHoveredFlags = 1 << 11;
// Return true after io.HoverDelayNormal elapsed (~0.30 sec)
pub const ImGuiHoveredFlags_DelayShort: ImGuiHoveredFlags = 1 << 12;
// Return true after io.HoverDelayShort elapsed (~0.10 sec)
pub const ImGuiHoveredFlags_NoSharedDelay: ImGuiHoveredFlags = 1 << 13;  // Disable shared delay system where moving from one item to the next keeps the previous timer for a short time (standard for tooltips with long delays)
// };
