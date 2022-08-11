use std::collections::HashSet;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum DockNodeFlags {
    None = 0,
    KeepAliveOnly, // Shared       // Don't display the dockspace node but keep it alive. windows docked into this dockspace node won't be undocked.
    //NoCentralNode              = 1 << 1,   // Shared       // Disable Central Node (the node which can stay empty)
    NoDockingInCentralNode, // Shared       // Disable docking inside the Central Node, which will be always kept empty.
    PassthruCentralNode, // Shared       // Enable passthru dockspace: 1) DockSpace() will render a ImGuiCol_WindowBg background covering everything excepted the Central Node when empty. Meaning the host window should probably use set_netxt_window_bg_alpha(0.0) prior to Begin() when using this. 2) When Central Node is empty: let inputs pass-through + won't display a DockingEmptyBg background. See demo for details.
    NoSplit, // Shared/Local // Disable splitting the node into smaller nodes. Useful e.g. when embedding dockspaces into a main root one (the root one may have splitting disabled to reduce confusion). Note: when turned off, existing splits will be preserved.
    NoResize, // Shared/Local // Disable resizing node using the splitter/separators. Useful with programmatically setup dockspaces.
    AutoHideTabBar, // Shared/Local // Tab bar will automatically hide when there is a single window in the dock node.
    // [Internal]
    DockSpace, // Local, Saved  // A dockspace is a node that occupy space within an existing user window. Otherwise the node is floating and create its own window.
    CentralNode, // Local, Saved  // The central node has 2 main properties: stay visible when empty, only use "remaining" spaces from its neighbor.
    NoTabBar, // Local, Saved  // Tab bar is completely unavailable. No triangle in the corner to enable it back.
    HiddenTabBar, // Local, Saved  // Tab bar is hidden, with a triangle in the corner to show it again (NB: actual tab-bar instance may be destroyed as this is only used for single-window tab bar)
    NoWindowMenuButton, // Local, Saved  // Disable window/docking menu (that one that appears instead of the collapse button)
    NoCloseButton,      // Local, Saved  //
    NoDocking, // Local, Saved  // Disable any form of docking in this dockspace or individual node. (On a whole dockspace, this pretty much defeat the purpose of using a dockspace at all). Note: when turned on, existing docked nodes will be preserved.
    NoDockingSplitMe, // [EXPERIMENTAL] Prevent another window/node from splitting this node.
    NoDockingSplitOther, // [EXPERIMENTAL] Prevent this node from splitting another window/node.
    NoDockingOverMe, // [EXPERIMENTAL] Prevent another window/node to be docked over this node.
    NoDockingOverOther, // [EXPERIMENTAL] Prevent this node to be docked over another window or non-empty node.
    NoDockingOverEmpty, // [EXPERIMENTAL] Prevent this node to be docked over an empty node (e.g. DockSpace with no other windows)
    NoResizeX,          // [EXPERIMENTAL]
    NoResizeY,          // [EXPERIMENTAL]
    SharedFlagsInheritMask,
}

impl Default for DockNodeFlags {
    fn default() -> Self {
        Self::None
    }
}

pub const NO_RESIZE_FLAGS_MASK: HashSet<DockNodeFlags> = HashSet::from([
    DockNodeFlags::NoResize,
    DockNodeFlags::NoResizeX,
    DockNodeFlags::NoResizeY,
]);

pub const LOCAL_FLAGS_MASK: HashSet<DockNodeFlags> = HashSet::from([
    DockNodeFlags::NoSplit,
    DockNodeFlags::AutoHideTabBar,
    DockNodeFlags::DockSpace,
    DockNodeFlags::CentralNode,
    DockNodeFlags::NoTabBar,
    DockNodeFlags::HiddenTabBar,
    DockNodeFlags::NoWindowMenuButton,
    DockNodeFlags::NoCloseButton,
    DockNodeFlags::NoDocking,
    DockNodeFlags::NoResize,
    DockNodeFlags::NoResizeX,
    DockNodeFlags::NoResizeY,
]);

pub const DOCK_NODE_FLAGS_LOCAL_FLAGS_TRANSFER_MASK: HashSet<DockNodeFlags> = LOCAL_FLAGS_MASK;

// When splitting those flags are moved to the inheriting child, never duplicated
pub const SAVED_FLAGS_MASK: HashSet<DockNodeFlags> = HashSet::from([
    DockNodeFlags::NoResize,
    DockNodeFlags::NoResizeX,
    DockNodeFlags::NoResizeY,
    DockNodeFlags::DockSpace,
    DockNodeFlags::CentralNode,
    DockNodeFlags::NoTabBar,
    DockNodeFlags::HiddenTabBar,
    DockNodeFlags::NoWindowMenuButton,
    DockNodeFlags::NoCloseButton,
    DockNodeFlags::NoDocking,
]);
