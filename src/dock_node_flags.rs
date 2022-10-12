#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiDockNodeFlags;     // -> enum ImGuiDockNodeFlags_   // Flags: for DockSpace()
pub type ImGuiDockNodeFlags = c_int;


// Flags for DockSpace(), shared/inherited by child nodes.
// (Some flags can be applied to individual nodes directly)
// FIXME-DOCK: Also see ImGuiDockNodeFlagsPrivate_ which may involve using the WIP and internal DockBuilder api.
// enum ImGuiDockNodeFlags_
// {
pub const ImGuiDockNodeFlags_None: ImGuiDockNodeFlags = 0;
pub const ImGuiDockNodeFlags_KeepAliveOnly: ImGuiDockNodeFlags = 1 << 0;
// Shared       // Don't display the dockspace node but keep it alive. Windows docked into this dockspace node won't be undocked.
//ImGuiDockNodeFlags_NoCentralNode              = 1 << 1,   // Shared       // Disable Central Node (the node which can stay empty)
pub const ImGuiDockNodeFlags_NoDockingInCentralNode: ImGuiDockNodeFlags = 1 << 2;
// Shared       // Disable docking inside the Central Node, which will be always kept empty.
pub const ImGuiDockNodeFlags_PassthruCentralNode: ImGuiDockNodeFlags = 1 << 3;
// Shared       // Enable passthru dockspace: 1) DockSpace() will render a ImGuiCol_WindowBg background covering everything excepted the Central Node when empty. Meaning the host window should probably use SetNextWindowBgAlpha(0f32) prior to Begin() when using this. 2) When Central Node is empty: let inputs pass-through + won't display a DockingEmptyBg background. See demo for details.
pub const ImGuiDockNodeFlags_NoSplit: ImGuiDockNodeFlags = 1 << 4;
// Shared/Local // Disable splitting the node into smaller nodes. Useful e.g. when embedding dockspaces into a main root one (the root one may have splitting disabled to reduce confusion). Note: when turned off, existing splits will be preserved.
pub const ImGuiDockNodeFlags_NoResize: ImGuiDockNodeFlags = 1 << 5;
// Shared/Local // Disable resizing node using the splitter/separators. Useful with programmatically setup dockspaces.
pub const ImGuiDockNodeFlags_AutoHideTabBar: ImGuiDockNodeFlags = 1 << 6;   // Shared/Local // Tab bar will automatically hide when there is a single window in the dock node.
// };


// Extend ImGuiDockNodeFlags_
// enum ImGuiDockNodeFlagsPrivate_
// {
// [Internal]
pub const ImGuiDockNodeFlags_DockSpace: ImGuiDockNodeFlags = 1 << 10;
// Local, Saved  // A dockspace is a node that occupy space within an existing user window. Otherwise the node is floating and create its own window.
pub const ImGuiDockNodeFlags_CentralNode: ImGuiDockNodeFlags = 1 << 11;
// Local, Saved  // The central node has 2 main properties: stay visible when empty, only use "remaining" spaces from its neighbor.
pub const ImGuiDockNodeFlags_NoTabBar: ImGuiDockNodeFlags = 1 << 12;
// Local, Saved  // Tab bar is completely unavailable. No triangle in the corner to enable it back.
pub const ImGuiDockNodeFlags_HiddenTabBar: ImGuiDockNodeFlags = 1 << 13;
// Local, Saved  // Tab bar is hidden, with a triangle in the corner to show it again (NB: actual tab-bar instance may be destroyed as this is only used for single-window tab bar)
pub const ImGuiDockNodeFlags_NoWindowMenuButton: ImGuiDockNodeFlags = 1 << 14;
// Local, Saved  // Disable window/docking menu (that one that appears instead of the collapse button)
pub const ImGuiDockNodeFlags_NoCloseButton: ImGuiDockNodeFlags = 1 << 15;
// Local, Saved  //
pub const ImGuiDockNodeFlags_NoDocking: ImGuiDockNodeFlags = 1 << 16;
// Local, Saved  // Disable any form of docking in this dockspace or individual node. (On a whole dockspace, this pretty much defeat the purpose of using a dockspace at all). Note: when turned on, existing docked nodes will be preserved.
pub const ImGuiDockNodeFlags_NoDockingSplitMe: ImGuiDockNodeFlags = 1 << 17;
// [EXPERIMENTAL] Prevent another window/node from splitting this node.
pub const ImGuiDockNodeFlags_NoDockingSplitOther: ImGuiDockNodeFlags = 1 << 18;
// [EXPERIMENTAL] Prevent this node from splitting another window/node.
pub const ImGuiDockNodeFlags_NoDockingOverMe: ImGuiDockNodeFlags = 1 << 19;
// [EXPERIMENTAL] Prevent another window/node to be docked over this node.
pub const ImGuiDockNodeFlags_NoDockingOverOther: ImGuiDockNodeFlags = 1 << 20;
// [EXPERIMENTAL] Prevent this node to be docked over another window or non-empty node.
pub const ImGuiDockNodeFlags_NoDockingOverEmpty: ImGuiDockNodeFlags = 1 << 21;
// [EXPERIMENTAL] Prevent this node to be docked over an empty node (e.g. DockSpace with no other windows)
pub const ImGuiDockNodeFlags_NoResizeX: ImGuiDockNodeFlags = 1 << 22;
// [EXPERIMENTAL]
pub const ImGuiDockNodeFlags_NoResizeY: ImGuiDockNodeFlags = 1 << 23;
// [EXPERIMENTAL]
// pub const ImGuiDockNodeFlags_NoResize: ImGuiDockNodeFlags = 1 << 24;
// pub const ImGuiDockNodeFlags_NoSplit: ImGuiDockNodeFlags = 1 << 25;
pub const ImGuiDockNodeFlags_SharedFlagsInheritMask_: ImGuiDockNodeFlags = !0;
// pub const ImGuiDockNodeFlags_AutoHideTabBar: ImGuiDockNodeFlags = 1 << 26;
pub const ImGuiDockNodeFlags_NoResizeFlagsMask_: ImGuiDockNodeFlags = ImGuiDockNodeFlags_NoResize | ImGuiDockNodeFlags_NoResizeX | ImGuiDockNodeFlags_NoResizeY;
pub const ImGuiDockNodeFlags_LocalFlagsMask_: ImGuiDockNodeFlags = ImGuiDockNodeFlags_NoSplit | ImGuiDockNodeFlags_NoResizeFlagsMask_ | ImGuiDockNodeFlags_AutoHideTabBar | ImGuiDockNodeFlags_DockSpace | ImGuiDockNodeFlags_CentralNode | ImGuiDockNodeFlags_NoTabBar | ImGuiDockNodeFlags_HiddenTabBar | ImGuiDockNodeFlags_NoWindowMenuButton | ImGuiDockNodeFlags_NoCloseButton | ImGuiDockNodeFlags_NoDocking;
pub const ImGuiDockNodeFlags_LocalFlagsTransferMask_: ImGuiDockNodeFlags = ImGuiDockNodeFlags_LocalFlagsMask_ & !ImGuiDockNodeFlags_DockSpace;
// When splitting those flags are moved to the inheriting child, never duplicated
pub const ImGuiDockNodeFlags_SavedFlagsMask_: ImGuiDockNodeFlags = ImGuiDockNodeFlags_NoResizeFlagsMask_ | ImGuiDockNodeFlags_DockSpace | ImGuiDockNodeFlags_CentralNode | ImGuiDockNodeFlags_NoTabBar | ImGuiDockNodeFlags_HiddenTabBar | ImGuiDockNodeFlags_NoWindowMenuButton | ImGuiDockNodeFlags_NoCloseButton | ImGuiDockNodeFlags_NoDocking;
// };
