#![allow(non_snake_case)]

use crate::axis::{ImGuiAxis, ImGuiAxis_None};
use crate::color::IM_COL32_WHITE;
use crate::data_authority::{
    ImGuiDataAuthority, IM_GUI_DATA_AUTHORITY_AUTO, IM_GUI_DATA_AUTHORITY_DOCK_NODE,
};
use crate::dock_node_flags::{
    ImGuiDockNodeFlags, ImGuiDockNodeFlags_CentralNode, ImGuiDockNodeFlags_DockSpace,
    ImGuiDockNodeFlags_HiddenTabBar, ImGuiDockNodeFlags_NoTabBar, ImGuiDockNodeFlags_None,
};
use crate::dock_node_state::{ImGuiDockNodeState, ImGuiDockNodeState_Unknown};
use crate::tab_bar::ImGuiTabBar;
use crate::type_defs::ImguiHandle;
use crate::vec2::ImVec2;
use crate::window::window_class::ImGuiWindowClass;
use crate::window::ImguiWindow;
use crate::window_class::ImGuiWindowClass;
use libc::{c_int, size_t};
use std::ptr::{null, null_mut};

// sizeof() 156~192
#[derive(Default, Debug, Clone, Copy)]
pub struct ImGuiDockNode {
    pub ID: ImguiHandle,
    pub SharedFlags: ImGuiDockNodeFlags, // (Write) Flags shared by all nodes of a same dockspace hierarchy (inherited from the root node)
    pub LocalFlags: ImGuiDockNodeFlags,  // (Write) Flags specific to this node
    pub LocalFlagsInWindows: ImGuiDockNodeFlags, // (Write) Flags specific to this node, applied from windows
    pub MergedFlags: ImGuiDockNodeFlags, // (Read)  Effective flags (== SharedFlags | LocalFlagsInNode | LocalFlagsInWindows)
    pub State: ImGuiDockNodeState,
    pub ParentNode: ImGuiDockNode,
    // ImGuiDockNode*          ChildNodes[2];              // [Split node only] Child nodes (left/right or top/bottom). Consider switching to an array.
    pub ChildNodes: [ImGuiDockNode; 2],
    pub Windows: Vec<ImguiWindow>, // Note: unordered list! Iterate TabBar.Tabs for user-order.
    pub TabBar: ImGuiTabBar,
    pub Pos: ImVec2,                   // Current position
    pub Size: ImVec2,                  // Current size
    pub SizeRef: ImVec2, // [Split node only] Last explicitly written-to size (overridden when using a splitter affecting the node), used to calculate Size.
    pub SplitAxis: ImGuiAxis, // [Split node only] Split axis (X or Y)
    pub WindowClass: ImGuiWindowClass, // [Root node only]
    pub LastBgColor: u32,
    pub HostWindow: Option<ImguiWindow>,
    pub VisibleWindow: Option<ImguiWindow>, // Generally point to window which is ID is == SelectedTabID, but when CTRL+Tabbing this can be a different window.
    pub CentralNode: ImGuiDockNode,         // [Root node only] Pointer to central node.
    pub OnlyNodeWithWindows: ImGuiDockNode, // [Root node only] Set when there is a single visible node within the hierarchy.
    pub CountNodeWithWindows: size_t,       // [Root node only]
    pub LastFrameAlive: c_int, // Last frame number the node was updated or kept alive explicitly with DockSpace() + ImGuiDockNodeFlags_KeepAliveOnly
    pub LastFrameActive: c_int, // Last frame number the node was updated.
    pub LastFrameFocused: c_int, // Last frame number the node was focused.
    pub LastFocusedNodeId: ImguiHandle, // [Root node only] Which of our child docking node (any ancestor in the hierarchy) was last focused.
    pub SelectedTabId: ImguiHandle,     // [Leaf node only] Which of our tab/window is selected.
    pub WantCloseTabId: ImguiHandle,    // [Leaf node only] Set when closing a specific tab/window.
    pub AuthorityForPos: ImGuiDataAuthority,
    pub AuthorityForSize: ImGuiDataAuthority,
    pub AuthorityForViewport: ImGuiDataAuthority,
    pub IsVisible: bool, // Set to false when the node is hidden (usually disabled as it has no active window)
    pub IsFocused: bool,
    pub IsBgDrawnThisFrame: bool,
    pub HasCloseButton: bool, // Provide space for a close button (if any of the docked window has one). Note that button may be hidden on window without one.
    pub HasWindowMenuButton: bool,
    pub HasCentralNodeChild: bool,
    pub WantCloseAll: bool, // Set when closing all tabs at once.
    pub WantLockSizeOnce: bool,
    pub WantMouseMove: bool, // After a node extraction we need to transition toward moving the newly created host window
    pub WantHiddenTabBarUpdate: bool,
    pub WantHiddenTabBarToggle: bool,
}

impl ImGuiDockNode {
    // ImGuiDockNode(ImguiHandle id);
    pub fn new(id: ImguiHandle) -> Self {
        let mut out = Self::default();
        out.ID = id;
        // SharedFlags = LocalFlags = LocalFlagsInWindows = MergedFlags = ImGuiDockNodeFlags_None;
        out.SharedFlags = ImGuiDockNodeFlags_None;
        out.LocalFlags = ImGuiDockNodeFlags_None;
        out.LocalFlagsInWindows = ImGuiDockNodeFlags_None;
        out.MergedFlags = ImGuiDockNodeFlags_None;
        // ParentNode = ChildNodes[0] = ChildNodes[1]= None;
        out.ParentNode = None;
        out.ChildNodes = [None; 2];
        out.TabBar = None;
        out.SplitAxis = ImGuiAxis_None;
        out.State = ImGuiDockNodeState_Unknown;
        out.LastBgColor = IM_COL32_WHITE;
        out.HostWindow = None;
        out.VisibleWindow = None;
        out.CentralNode = None;
        out.OnlyNodeWithWindows = None;
        out.CountNodeWithWindows = 0;
        out.LastFrameAlive = -1;
        out.LastFrameActive = -1;
        out.LastFrameFocused = -1;
        out.LastFocusedNodeId = 0;
        out.SelectedTabId = 0;
        out.WantCloseTabId = 0;
        out.AuthorityForPos = IM_GUI_DATA_AUTHORITY_DOCK_NODE;
        out.AuthorityForSize = IM_GUI_DATA_AUTHORITY_DOCK_NODE;
        out.AuthorityForViewport = IM_GUI_DATA_AUTHORITY_AUTO;
        out.IsVisible = true;
        out.IsFocused = false;
        out.HasCloseButton = false;
        out.HasWindowMenuButton = false;
        out.HasCentralNodeChild = false;
        out.IsBgDrawnThisFrame = false;
        out.WantCloseAll = false;
        out.WantLockSizeOnce = false;
        out.WantMouseMove = false;
        out.WantHiddenTabBarUpdate = false;
        out.WantHiddenTabBarToggle = false;
        out
    }

    // ~ImGuiDockNode();

    // bool                    IsRootNode() const      { return ParentNode == None; }
    pub fn IsRootNode(&self) -> bool {
        self.ParentNode.is_null()
    }

    // bool                    IsDockSpace() const     { return (MergedFlags & ImGuiDockNodeFlags_DockSpace) != 0; }
    pub fn IsDockSpace(&self) -> bool {
        self.MergedFlags & ImGuiDockNodeFlags_DockSpace != 0
    }

    // bool                    IsFloatingNode() const  { return ParentNode == NULL && (MergedFlags & ImGuiDockNodeFlags_DockSpace) == 0; }
    pub fn IsFloatingNode(&self) -> bool {
        self.ParentNode.is_null() && self.MergedFlags & ImGuiDockNodeFlags_DockSpace == 0
    }

    // bool                    IsCentralNode() const   { return (MergedFlags & ImGuiDockNodeFlags_CentralNode) != 0; }
    pub fn IsCentralNode(&self) -> bool {
        self.MergedFlags & ImGuiDockNodeFlags_CentralNode != 0
    }

    // bool                    IsHiddenTabBar() const  { return (MergedFlags & ImGuiDockNodeFlags_HiddenTabBar) != 0; } // Hidden tab bar can be shown back by clicking the small triangle
    pub fn IsHiddenTabBar(&self) -> bool {
        self.MergedFlags & ImGuiDockNodeFlags_HiddenTabBar != 0
    }

    // bool                    IsNoTabBar() const      { return (MergedFlags & ImGuiDockNodeFlags_NoTabBar) != 0; }     // Never show a tab bar
    pub fn IsNoTabBar(&self) -> bool {
        self.MergedFlags & ImGuiDockNodeFlags_NoTabBar != 0
    }

    // bool                    IsSplitNode() const     { return ChildNodes[0] != None; }
    pub fn IsSplitNode(&self) -> bool {
        self.ChildNodes[0].is_null() == false
    }

    // bool                    IsLeafNode() const      { return ChildNodes[0] == None; }
    pub fn IsLeafNode(&self) -> bool {
        self.ChildNodes[0].is_null()
    }

    // bool                    IsEmpty() const         { return ChildNodes[0] == NULL && Windows.Size == 0; }
    pub fn IsEmpty(&self) -> bool {
        self.ChildNodes[0].is_null() && self.Windows.len() == 0
    }

    // ImRect                  Rect() const            { return ImRect(Pos.x, Pos.y, Pos.x + Size.x, Pos.y + Size.y); }

    // void                    SetLocalFlags(ImGuiDockNodeFlags flags) { LocalFlags = flags; UpdateMergedFlags(); }

    // void                    UpdateMergedFlags()     { MergedFlags = SharedFlags | LocalFlags | LocalFlagsInWindows; }
}
