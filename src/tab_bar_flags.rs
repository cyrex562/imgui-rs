#![allow(non_upper_case_globals)]

pub const ImGuiTabBarFlags_None: i32                           = 0;
pub const ImGuiTabBarFlags_Reorderable: i32                    = 1 << 0;
// Allow manually dragging tabs to re-order them + New tabs are appended at the end of list
pub const ImGuiTabBarFlags_AutoSelectNewTabs: i32              = 1 << 1;
// Automatically select new tabs when they appear
pub const ImGuiTabBarFlags_TabListPopupButton: i32             = 1 << 2;
// Disable buttons to open the tab list popup
pub const ImGuiTabBarFlags_NoCloseWithMiddleMouseButton: i32   = 1 << 3;
// Disable behavior of closing tabs (that are submitted with p_open != NULL) with middle mouse button. You can still repro this behavior on user's side with if (IsItemHovered() && IsMouseClicked(2)) *p_open = false.
pub const ImGuiTabBarFlags_NoTabListScrollingButtons: i32      = 1 << 4;
// Disable scrolling buttons (apply when fitting policy is ImGuiTabBarFlags_FittingPolicyScroll)
pub const ImGuiTabBarFlags_NoTooltip: i32                      = 1 << 5;
// Disable tooltips when hovering a tab
pub const ImGuiTabBarFlags_FittingPolicyResizeDown: i32        = 1 << 6;
// Resize tabs when they don't fit
pub const ImGuiTabBarFlags_FittingPolicyScroll: i32            = 1 << 7;

pub const ImGuiTabBarFlags_FittingPolicyMask_: i32 = ImGuiTabBarFlags_FittingPolicyResizeDown | ImGuiTabBarFlags_FittingPolicyScroll;

pub type ImGuiTabBarFlags = i32;
