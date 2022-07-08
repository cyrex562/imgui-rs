use crate::imgui_h::{ImGuiAxis, ImGuiDataAuthority, ImGuiID, ImGuiWindowClass};
use crate::imgui_rect::ImRect;
use crate::imgui_tab_bar::ImGuiTabBar;
use crate::imgui_vec::ImVec2;
use crate::imgui_window::ImGuiWindow;


#[allow(non_camel_case_types)]// flags for ImGui::DockSpace(), shared/inherited by child nodes.
// (Some flags can be applied to individual nodes directly)
// FIXME-DOCK: Also see ImGuiDockNodeFlagsPrivate_ which may involve using the WIP and internal DockBuilder api.
pub enum DimgDockNodeFlags
{
    None                         = 0,
    KeepAliveOnly                = 1 << 0,   // Shared       // Don't display the dockspace node but keep it alive. windows docked into this dockspace node won't be undocked.
    //NoCentralNode              = 1 << 1,   // Shared       // Disable Central Node (the node which can stay empty)
    NoDockingInCentralNode       = 1 << 2,   // Shared       // Disable docking inside the Central Node, which will be always kept empty.
    PassthruCentralNode          = 1 << 3,   // Shared       // Enable passthru dockspace: 1) DockSpace() will render a ImGuiCol_WindowBg background covering everything excepted the Central Node when empty. Meaning the host window should probably use SetNextWindowBgAlpha(0.0) prior to Begin() when using this. 2) When Central Node is empty: let inputs pass-through + won't display a DockingEmptyBg background. See demo for details.
    NoSplit                      = 1 << 4,   // Shared/Local // Disable splitting the node into smaller nodes. Useful e.g. when embedding dockspaces into a main root one (the root one may have splitting disabled to reduce confusion). Note: when turned off, existing splits will be preserved.
    NoResize                     = 1 << 5,   // Shared/Local // Disable resizing node using the splitter/separators. Useful with programmatically setup dockspaces.
    AutoHideTabBar               = 1 << 6    // Shared/Local // Tab bar will automatically hide when there is a single window in the dock node.
}

pub struct  ImGuiDockNode
{
    // ImGuiID                 ID;
    pub ID: ImGuiID,
    // ImGuiDockNodeFlags      SharedFlags;                // (Write) flags shared by all nodes of a same dockspace hierarchy (inherited from the root node)
    pub SharedFlags: DimgDockNodeFlags,
    // ImGuiDockNodeFlags      LocalFlags;                 // (Write) flags specific to this node
    pub LocalFlags: DimgDockNodeFlags,
    // ImGuiDockNodeFlags      LocalFlagsInWindows;        // (Write) flags specific to this node, applied from windows
    pub LocalFlagsInWindows: DimgDockNodeFlags,
    // ImGuiDockNodeFlags      MergedFlags;                // (Read)  Effective flags (== SharedFlags | LocalFlagsInNode | LocalFlagsInWindows)
    pub MergedFlags: DimgDockNodeFlags,
    // ImGuiDockNodeState      State;
    pub State: ImGuiDockNodeState,
    // ImGuiDockNode*          ParentNode;
    pub ParentNode: *mut ImGuiDockNode,
    // ImGuiDockNode*          ChildNodes[2];              // [split node only] Child nodes (left/right or top/bottom). Consider switching to an array.
    pub ChildNodes: [*mut ImGuiDockNode;2],
    // ImVector<ImGuiWindow*>  windows;                    // Note: unordered list! Iterate TabBar->Tabs for user-order.
    pub Windows: Vec<ImGuiWindow>,
    // ImGuiTabBar*            TabBar;
    pub TabBar: *mut ImGuiTabBar,
    // ImVec2                  pos;                        // Current position
    pub Pos: ImVec2,
    // ImVec2                  size;                       // Current size
    pub Size: ImVec2,
    // ImVec2                  SizeRef;                    // [split node only] Last explicitly written-to size (overridden when using a splitter affecting the node), used to calculate size.
    pub SizeRef: ImVec2,
    // ImGuiAxis               SplitAxis;                  // [split node only] split axis (X or Y)
    pub SplitAxis: ImGuiAxis,
    // ImGuiWindowClass        window_class;                // [Root node only]
    pub WindowClass: ImGuiWindowClass,
    // ImU32                   LastBgColor;
    pub LastBgColor: u32,
    // ImGuiWindow*            HostWindow;
    pub HostWindow: *mut ImGuiWindow,
    // ImGuiWindow*            VisibleWindow;              // Generally point to window which is ID is == SelectedTabID, but when CTRL+Tabbing this can be a different window.
    pub VisibleWindow: *mut ImGuiWindow,
    // ImGuiDockNode*          CentralNode;                // [Root node only] Pointer to central node.
    pub CentralNode: *mut ImGuiDockNode,
    // ImGuiDockNode*          OnlyNodeWithWindows;        // [Root node only] Set when there is a single visible node within the hierarchy.
    pub OnlyNodeWithWindow: *mut ImGuiDockNode,
    // int                     CountNodeWithWindows;       // [Root node only]
    pub CountNodeWithWindows: i32,
    // int                     LastFrameAlive;             // Last frame number the node was updated or kept alive explicitly with DockSpace() + ImGuiDockNodeFlags_KeepAliveOnly
    pub LastFrameAlive: i32,
    // int                     last_frame_active;            // Last frame number the node was updated.
    pub LastFrameActive: i32,
    // int                     LastFrameFocused;           // Last frame number the node was focused.
    pub LastGrameFocused: i32,
    // ImGuiID                 LastFocusedNodeId;          // [Root node only] Which of our child docking node (any ancestor in the hierarchy) was last focused.
    pub LastFocusedNodeId: ImGuiID,
    // ImGuiID                 SelectedTabId;              // [Leaf node only] Which of our tab/window is selected.
    pub SelectedTabId: ImGuiID,
    // ImGuiID                 WantCloseTabId;             // [Leaf node only] Set when closing a specific tab/window.
    pub WantCloseTabId: ImGuiID,
    // ImGuiDataAuthority      AuthorityForPos         :3;
    pub AuthorityForPos: ImGuiDataAuthority,
    // ImGuiDataAuthority      AuthorityForSize        :3;
    pub AuthorityForSize: ImGuiDataAuthority,
    // ImGuiDataAuthority      AuthorityForViewport    :3;
    pub AuthorityForViewport: ImGuiDataAuthority,
    // bool                    IsVisible               :1; // Set to false when the node is hidden (usually disabled as it has no active window)
    pub IsVisible: bool,
    // bool                    IsFocused               :1;
    pub IsFocused: bool,
    // bool                    IsBgDrawnThisFrame      :1;
    pub IsBgDrawnThisFrame: bool,
    // bool                    has_close_button          :1; // Provide space for a close button (if any of the docked window has one). Note that button may be hidden on window without one.
    pub HasCloseButton: bool,
    // bool                    HasWindowMenuButton     :1;
    pub HasWindowMenuButton: bool,
    // bool                    HasCentralNodeChild     :1;
    pub HasCentralNodeChild: bool,
    // bool                    WantCloseAll            :1; // Set when closing all tabs at once.
    pub WantCloseAll: bool,
    // bool                    WantLockSizeOnce        :1;
    pub WanLockSizeOnce: bool,
    // bool                    WantMouseMove           :1; // After a node extraction we need to transition toward moving the newly created host window
    pub WantMouseMOve: bool,
    // bool                    WantHiddenTabBarUpdate  :1;
    pub WantHiddenTabBarUpdate: bool,
    // bool                    WantHiddenTabBarToggle  :1;
    pub WantHiddenTabBarToggle: bool,
}

impl ImGuiDockNode {
    // ImGuiDockNode(ImGuiID id);
    pub fn new(id: ImGuiID) -> Self {
        todo!()
    }
    //     ~ImGuiDockNode();
    //     bool                    IsRootNode() const      { return ParentNode == NULL; }
    pub fn IsRootNode(&self) -> bool {
        self.ParentNode.is_null()
    }
    //     bool                    IsDockSpace() const     { return (MergedFlags & ImGuiDockNodeFlags_DockSpace) != 0; }
    pub fn IsDockSpace(&self) -> bool {
        (&self.MergedFlags & DimgDockNodeFlags::DockSpace) != 0
    }
    //     bool                    IsFloatingNode() const  { return ParentNode == NULL && (MergedFlags & ImGuiDockNodeFlags_DockSpace) == 0; }
    pub fn IsFloatingNode(&self) -> bool {
        self.ParentNode.is_null() && &self.MergedFlags & DimgDockNodeFlags::DockSpace == 0
    }
    //     bool                    IsCentralNode() const   { return (MergedFlags & ImGuiDockNodeFlags_CentralNode) != 0; }
    pub fn IsCentralNode(&self) -> bool {
        &self.MergedFlags & DimgDockNodeFlags::CentralNode != 0
    }
    //     bool                    IsHiddenTabBar() const  { return (MergedFlags & ImGuiDockNodeFlags_HiddenTabBar) != 0; } // hidden tab bar can be shown back by clicking the small triangle
    pub fn IsHiddenTabBar(&self) -> bool {
        &self.MergedFlags & DimgDockNodeFlags::HiddenTabBar != 0
    }
    //     bool                    IsNoTabBar() const      { return (MergedFlags & ImGuiDockNodeFlags_NoTabBar) != 0; }     // Never show a tab bar
    pub fn IsNoTabBar(&self) -> bool {
        &self.MergedFlags & DimgDockNodeFlags::NoTabBar
    }
    //     bool                    IsSplitNode() const     { return ChildNodes[0] != NULL; }
    pub fn IsSplitNode(&self) -> bool {
        self.ChildNodes[0].is_null() == false
    }
    //     bool                    IsLeafNode() const      { return ChildNodes[0] == NULL; }
    pub fn IsLeafNode(&self) -> bool {
        self.ChildNodes[0].is_null()
    }
    //     bool                    IsEmpty() const         { return ChildNodes[0] == NULL && windows.size == 0; }
    pub fn IsEmpty(&self) -> bool {
        self.ChildNodes[0].is_null() && self.Windows.is_empty()
    }
    //     ImRect                  Rect() const            { return ImRect(pos.x, pos.y, pos.x + size.x, pos.y + size.y); }
    pub fn Rect(&self) -> ImRect {
        ImRect::new4(self.Pos.x, self.Pos.y, self.Pos.x + self.Size.x, self.Pox.y + self.Size.y)
    }
    //
    //     void                    SetLocalFlags(ImGuiDockNodeFlags flags) { LocalFlags = flags; UpdateMergedFlags(); }
    pub fn SetLocalFlags(&mut self, flags: DimgDockNodeFlags) {
        self.LocalFlags = flags;
        self.UpdatemergedFlags();
    }
    //     void                    UpdateMergedFlags()     { MergedFlags = SharedFlags | LocalFlags | LocalFlagsInWindows; }
    pub fn UpdateMergedFlags(&mut self) {
        self.MergedFlags = &self.SharedFlags | &self.LocalFlags | &self.LocalFlagsInWindows;
    }
}

pub enum ImGuiDockNodeState
{
    Unknown,
    HostWindowHiddenBecauseSingleWindow,
    HostWindowHiddenBecauseWindowsAreResizing,
    HostWindowVisible
}

