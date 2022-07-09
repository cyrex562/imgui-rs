use std::collections::HashSet;
use crate::axis::DimgAxis;
use crate::data_authority::DimgDataAuthority;
use crate::types::DimgId;
use crate::dock;
use crate::rect::DimgRect;
use crate::tab_bar::DimgTabBar;
use crate::types::DIMG_ID_INVALID;
use crate::utils::extend_hash_set;
use crate::vec_nd::DimgVec2D;
use crate::window::DimgWindowClass;



#[derive(Clone,Debug,Eq, PartialEq,Hash)]
pub enum DimgDockNodeFlags
{
    None                         = 0,
    KeepAliveOnly                = 1 << 0,   // Shared       // Don't display the dockspace node but keep it alive. windows docked into this dockspace node won't be undocked.
    //NoCentralNode              = 1 << 1,   // Shared       // Disable Central Node (the node which can stay empty)
    NoDockingInCentralNode       = 1 << 2,   // Shared       // Disable docking inside the Central Node, which will be always kept empty.
    PassthruCentralNode          = 1 << 3,   // Shared       // Enable passthru dockspace: 1) DockSpace() will render a ImGuiCol_WindowBg background covering everything excepted the Central Node when empty. Meaning the host window should probably use SetNextWindowBgAlpha(0.0) prior to Begin() when using this. 2) When Central Node is empty: let inputs pass-through + won't display a DockingEmptyBg background. See demo for details.
    NoSplit                      = 1 << 4,   // Shared/Local // Disable splitting the node into smaller nodes. Useful e.g. when embedding dockspaces into a main root one (the root one may have splitting disabled to reduce confusion). Note: when turned off, existing splits will be preserved.
    NoResize                     = 1 << 5,   // Shared/Local // Disable resizing node using the splitter/separators. Useful with programmatically setup dockspaces.
    AutoHideTabBar               = 1 << 6,    // Shared/Local // Tab bar will automatically hide when there is a single window in the dock node.
    // [Internal]
    DockSpace                = 1 << 10,  // Local, Saved  // A dockspace is a node that occupy space within an existing user window. Otherwise the node is floating and create its own window.
    CentralNode              = 1 << 11,  // Local, Saved  // The central node has 2 main properties: stay visible when empty, only use "remaining" spaces from its neighbor.
    NoTabBar                 = 1 << 12,  // Local, Saved  // Tab bar is completely unavailable. No triangle in the corner to enable it back.
    HiddenTabBar             = 1 << 13,  // Local, Saved  // Tab bar is hidden, with a triangle in the corner to show it again (NB: actual tab-bar instance may be destroyed as this is only used for single-window tab bar)
    NoWindowMenuButton       = 1 << 14,  // Local, Saved  // Disable window/docking menu (that one that appears instead of the collapse button)
    NoCloseButton            = 1 << 15,  // Local, Saved  //
    NoDocking                = 1 << 16,  // Local, Saved  // Disable any form of docking in this dockspace or individual node. (On a whole dockspace, this pretty much defeat the purpose of using a dockspace at all). Note: when turned on, existing docked nodes will be preserved.
    NoDockingSplitMe         = 1 << 17,  // [EXPERIMENTAL] Prevent another window/node from splitting this node.
    NoDockingSplitOther      = 1 << 18,  // [EXPERIMENTAL] Prevent this node from splitting another window/node.
    NoDockingOverMe          = 1 << 19,  // [EXPERIMENTAL] Prevent another window/node to be docked over this node.
    NoDockingOverOther       = 1 << 20,  // [EXPERIMENTAL] Prevent this node to be docked over another window or non-empty node.
    NoDockingOverEmpty       = 1 << 21,  // [EXPERIMENTAL] Prevent this node to be docked over an empty node (e.g. DockSpace with no other windows)
    NoResizeX                = 1 << 22,  // [EXPERIMENTAL]
    NoResizeY                = 1 << 23,  // [EXPERIMENTAL]
    SharedFlagsInheritMask,
}

impl Default for DimgDockNodeFlags {
    fn default() -> Self {
        Self::None
    }
}

pub const NO_RESIZE_FLAGS_MASK: HashSet<DimgDockNodeFlags>       = HashSet::from([DimgDockNodeFlags::NoResize, DimgDockNodeFlags::NoResizeX, DimgDockNodeFlags::NoResizeY]);

pub const LOCAL_FLAGS_MASK: HashSet<DimgDockNodeFlags> = HashSet::from([DimgDockNodeFlags::NoSplit, DimgDockNodeFlags::AutoHideTabBar, DimgDockNodeFlags::DockSpace, DimgDockNodeFlags::CentralNode, DimgDockNodeFlags::NoTabBar, DimgDockNodeFlags::HiddenTabBar, DimgDockNodeFlags::NoWindowMenuButton, DimgDockNodeFlags::NoCloseButton, DimgDockNodeFlags::NoDocking, DimgDockNodeFlags::NoResize, DimgDockNodeFlags::NoResizeX, DimgDockNodeFlags::NoResizeY]);

pub const LOCAL_FLAGS_TRANSFER_MASK: HashSet<DimgDockNodeFlags>  = LOCAL_FLAGS_MASK;

// When splitting those flags are moved to the inheriting child, never duplicated
pub const SAVED_FLAGS_MASK: HashSet<DimgDockNodeFlags>          = HashSet::from([DimgDockNodeFlags::NoResize, DimgDockNodeFlags::NoResizeX, DimgDockNodeFlags::NoResizeY, DimgDockNodeFlags::DockSpace, DimgDockNodeFlags::CentralNode, DimgDockNodeFlags::NoTabBar, DimgDockNodeFlags::HiddenTabBar, DimgDockNodeFlags::NoWindowMenuButton, DimgDockNodeFlags::NoCloseButton, DimgDockNodeFlags::NoDocking]);

#[derive(Default,Debug,Clone)]
pub struct DimgDockNode
{
    // DimgId                 id;
    pub id: DimgId,
    // ImGuiDockNodeFlags      shared_flags;                // (Write) flags shared by all nodes of a same dockspace hierarchy (inherited from the root node)
    pub shared_flags: HashSet<DimgDockNodeFlags>,
    // ImGuiDockNodeFlags      local_flags;                 // (Write) flags specific to this node
    pub local_flags: HashSet<DimgDockNodeFlags>,
    // ImGuiDockNodeFlags      local_flags_in_windows;        // (Write) flags specific to this node, applied from windows
    pub local_flags_in_windows: HashSet<DimgDockNodeFlags>,
    // ImGuiDockNodeFlags      merged_flags;                // (Read)  Effective flags (== shared_flags | LocalFlagsInNode | local_flags_in_windows)
    pub merged_flags: HashSet<DimgDockNodeFlags>,
    // ImGuiDockNodeState      state;
    pub state: DimgDockNodeState,
    // ImGuiDockNode*          parent_node;
    pub parent_node: DimgId, //*mut ImGuiDockNode,
    // ImGuiDockNode*          child_nodes[2];              // [split node only] Child nodes (left/right or top/bottom). Consider switching to an array.
    pub child_nodes: Vec<DimgId>, //[*mut ImGuiDockNode;2],
    // ImVector<ImGuiWindow*>  windows;                    // Note: unordered list! Iterate tab_bar->Tabs for user-order.
    pub windows: Vec<DimgId>,
    // ImGuiTabBar*            tab_bar;
    pub tab_bar: DimgTabBar, //*mut ImGuiTabBar,
    // DimgVec2D                  pos;                        // Current position
    // pub pos: DimgVec2D,
    pub pos: DimgVec2D,
// DimgVec2D                  size;                       // Current size
    pub size: DimgVec2D,
    // DimgVec2D                  size_ref;                    // [split node only] Last explicitly written-to size (overridden when using a splitter affecting the node), used to calculate size.
    pub size_ref: DimgVec2D,
    // ImGuiAxis               split_axis;                  // [split node only] split axis (x or Y)
    pub split_axis: DimgAxis,
    // ImGuiWindowClass        window_class;                // [Root node only]
    pub window_class: DimgWindowClass,
    // ImU32                   last_bg_color;
    pub last_bg_color: u32,
    // ImGuiWindow*            host_window;
    pub host_window: DimgId, //*mut ImGuiWindow,
    // ImGuiWindow*            visible_window;              // Generally point to window which is id is == SelectedTabID, but when CTRL+Tabbing this can be a different window.
    pub visible_window: DimgId, //*mut ImGuiWindow,
    // ImGuiDockNode*          central_node;                // [Root node only] Pointer to central node.
    pub central_node: DimgId, // *mut ImGuiDockNode,
    // ImGuiDockNode*          OnlyNodeWithWindows;        // [Root node only] Set when there is a single visible node within the hierarchy.
    pub only_node_with_window: DimgId, // *mut ImGuiDockNode,
    // int                     count_node_with_windows;       // [Root node only]
    pub count_node_with_windows: i32,
    // int                     last_frame_alive;             // Last frame number the node was updated or kept alive explicitly with DockSpace() + ImGuiDockNodeFlags_KeepAliveOnly
    pub last_frame_alive: i32,
    // int                     last_frame_active;            // Last frame number the node was updated.
    pub last_frame_active: i32,
    // int                     LastFrameFocused;           // Last frame number the node was focused.
    pub last_grame_focused: i32,
    // DimgId                 last_focused_node_id;          // [Root node only] Which of our child docking node (any ancestor in the hierarchy) was last focused.
    pub last_focused_node_id: DimgId,
    // DimgId                 selected_tab_id;              // [Leaf node only] Which of our tab/window is selected.
    pub selected_tab_id: DimgId,
    // DimgId                 want_close_tab_id;             // [Leaf node only] Set when closing a specific tab/window.
    pub want_close_tab_id: DimgId,
    // ImGuiDataAuthority      authority_for_pos         :3;
    pub authority_for_pos: DimgDataAuthority,
    // ImGuiDataAuthority      authority_for_size        :3;
    pub authority_for_size: DimgDataAuthority,
    // ImGuiDataAuthority      authority_for_viewport    :3;
    pub authority_for_viewport: DimgDataAuthority,
    // bool                    is_visible               :1; // Set to false when the node is hidden (usually disabled as it has no active window)
    pub is_visible: bool,
    // bool                    is_focused               :1;
    pub is_focused: bool,
    // bool                    is_bg_drawn_this_frame      :1;
    pub is_bg_drawn_this_frame: bool,
    // bool                    has_close_button          :1; // Provide space for a close button (if any of the docked window has one). Note that button may be hidden on window without one.
    pub has_close_button: bool,
    // bool                    has_window_menu_button     :1;
    pub has_window_menu_button: bool,
    // bool                    has_central_node_child     :1;
    pub has_central_node_child: bool,
    // bool                    want_close_all            :1; // Set when closing all tabs at once.
    pub want_close_all: bool,
    // bool                    WantLockSizeOnce        :1;
    pub wan_lock_size_once: bool,
    // bool                    WantMouseMove           :1; // After a node extraction we need to transition toward moving the newly created host window
    pub want_mouse_move: bool,
    // bool                    want_hidden_tab_bar_update  :1;
    pub want_hidden_tab_bar_update: bool,
    // bool                    want_hidden_tab_bar_toggle  :1;
    pub want_hidden_tab_bar_toggle: bool,
}

impl DimgDockNode {
    // ImGuiDockNode(DimgId id);
    pub fn new(id: DimgId) -> Self {
        todo!()
    }
    //     ~ImGuiDockNode();
    //     bool                    is_root_node() const      { return parent_node == NULL; }
    pub fn is_root_node(&self) -> bool {
        self.parent_node > 0 && self.parent_node < DimgId::MAX
    }
    //     bool                    is_dock_space() const     { return (merged_flags & ImGuiDockNodeFlags_DockSpace) != 0; }
    pub fn is_dock_space(&self) -> bool {
        // (&self.merged_flags & DimgDockNodeFlags::DockSpace) != 0
        self.merged_flags.contains(&DimgDockNodeFlags::DockSpace) == false
    }
    //     bool                    is_floating_node() const  { return parent_node == NULL && (merged_flags & ImGuiDockNodeFlags_DockSpace) == 0; }
    pub fn is_floating_node(&self) -> bool {
        // self.parent_node.is_null() && &self.merged_flags & DimgDockNodeFlags::DockSpace == 0
        self.is_root_node() == false && self.merged_flags.contains(&DimgDockNodeFlags::DockSpace) == false
    }
    //     bool                    is_central_node() const   { return (merged_flags & ImGuiDockNodeFlags_CentralNode) != 0; }
    pub fn is_central_node(&self) -> bool {
        self.merged_flags.contains(&DimgDockNodeFlags::CentralNode) == false
    }
    //     bool                    is_hidden_tab_bar() const  { return (merged_flags & ImGuiDockNodeFlags_HiddenTabBar) != 0; } // hidden tab bar can be shown back by clicking the small triangle
    pub fn is_hidden_tab_bar(&self) -> bool {
        self.merged_flags.contains(&DimgDockNodeFlags::HiddenTabBar) == false
    }
    //     bool                    is_no_tab_bar() const      { return (merged_flags & ImGuiDockNodeFlags_NoTabBar) != 0; }     // Never show a tab bar
    pub fn is_no_tab_bar(&self) -> bool {
        self.merged_flags.contains(&DimgDockNodeFlags::NoTabBar)
    }
    //     bool                    is_split_node() const     { return child_nodes[0] != NULL; }
    pub fn is_split_node(&self) -> bool {
        self.child_nodes[0] != DIMG_ID_INVALID
    }
    //     bool                    is_leaf_node() const      { return child_nodes[0] == NULL; }
    pub fn is_leaf_node(&self) -> bool {
        self.child_nodes[0] == DIMG_ID_INVALID
    }
    //     bool                    is_empty() const         { return child_nodes[0] == NULL && windows.size == 0; }
    pub fn is_empty(&self) -> bool {
        // self.child_nodes[0].is_null() && self.windows.is_empty()
        self.child_nodes[0] == DIMG_ID_INVALID && self.child_nodes[1] == DIMG_ID_INVALID && self.windows.is_empty()
    }
    //     ImRect                  rect() const            { return ImRect(pos.x, pos.y, pos.x + size.x, pos.y + size.y); }
    pub fn rect(&self) -> DimgRect {
        DimgRect::new4(self.pos.x, self.pos.y, self.pos.x + self.size.x, self.pos.y + self.size.y)
    }
    //
    //     void                    set_local_flags(ImGuiDockNodeFlags flags) { local_flags = flags; UpdateMergedFlags(); }
    pub fn set_local_flags(&mut self, flags: HashSet<DimgDockNodeFlags>) {
        // self.local_flags = flags;
        for flag in flags {
            self.local_flags.insert(flag);
        }
        self.update_merged_flags();
    }
    //     void                    UpdateMergedFlags()     { merged_flags = shared_flags | local_flags | local_flags_in_windows; }
    pub fn update_merged_flags(&mut self) {
        // self.merged_flags = &self.shared_flags | &self.local_flags | &self.local_flags_in_windows;
        extend_hash_set(&mut self.merged_flags, &self.shared_flags);
        extend_hash_set(&mut self.merged_flags, &self.local_flags);
        extend_hash_set(&mut self.merged_flags, &self.local_flags_in_windows);
    }
}


#[derive(Debug,Clone)]
pub enum DimgDockNodeState
{
    Unknown,
    HostWindowHiddenBecauseSingleWindow,
    HostWindowHiddenBecauseWindowsAreResizing,
    HostWindowVisible
}

impl Default for DimgDockNodeState {
    fn default() -> Self {
        Self::Unknown
    }
}

// Persistent Settings data, stored contiguously in SettingsNodes (sizeof() ~32 bytes)
#[derive(Debug,Clone)]
pub struct DimgDockNodeSettings
{
    // ImGuiID             id;
    pub id: DimgId,
    // ImGuiID             ParentNodeId;
    pub parent_node_id: DimgId,
    // ImGuiID             ParentWindowId;
    pub parent_window_id: DimgId,
    // ImGuiID             SelectedTabId;
    pub selected_tab_id: DimgId,
    // signed char         SplitAxis;
    pub split_axis: i8,
    // char                Depth;
    pub depth: i8,
    // ImGuiDockNodeFlags  flags;                  // NB: We save individual flags one by one in ascii format (ImGuiDockNodeFlags_SavedFlagsMask_)
    pub flags: DimgDockNodeFlags,
    // ImVec2ih            pos;
    pub pos: DimgVec2D,
    // ImVec2ih            size;
    pub size: DimgVec2D,
    // ImVec2ih            SizeRef;
    pub size_ref: DimgVec2D,
    // ImGuiDockNodeSettings() { memset(this, 0, sizeof(*this)); SplitAxis = ImGuiAxis_None; }
}
