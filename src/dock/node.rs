use std::collections::HashSet;
use crate::axis::Axis;
use crate::context::Context;
use crate::types::DataAuthority;
use crate::types::Id32;
use crate::dock;
use crate::rect::Rect;
use crate::tab_bar::TabBar;
use crate::types::INVALID_ID;
use crate::utils::extend_hash_set;
use crate::vectors::two_d::Vector2D;
use crate::window::class::WindowClass;
use crate::window::Window;


#[derive(Clone,Debug,Eq, PartialEq,Hash)]
pub enum DockNodeFlags
{
    None                         = 0,
    KeepAliveOnly               ,   // Shared       // Don't display the dockspace node but keep it alive. windows docked into this dockspace node won't be undocked.
    //NoCentralNode              = 1 << 1,   // Shared       // Disable Central Node (the node which can stay empty)
    NoDockingInCentralNode      ,   // Shared       // Disable docking inside the Central Node, which will be always kept empty.
    PassthruCentralNode         ,   // Shared       // Enable passthru dockspace: 1) DockSpace() will render a ImGuiCol_WindowBg background covering everything excepted the Central Node when empty. Meaning the host window should probably use SetNextWindowBgAlpha(0.0) prior to Begin() when using this. 2) When Central Node is empty: let inputs pass-through + won't display a DockingEmptyBg background. See demo for details.
    NoSplit                     ,   // Shared/Local // Disable splitting the node into smaller nodes. Useful e.g. when embedding dockspaces into a main root one (the root one may have splitting disabled to reduce confusion). Note: when turned off, existing splits will be preserved.
    NoResize                    ,   // Shared/Local // Disable resizing node using the splitter/separators. Useful with programmatically setup dockspaces.
    AutoHideTabBar              ,    // Shared/Local // Tab bar will automatically hide when there is a single window in the dock node.
    // [Internal]
    DockSpace               ,  // Local, Saved  // A dockspace is a node that occupy space within an existing user window. Otherwise the node is floating and create its own window.
    CentralNode             ,  // Local, Saved  // The central node has 2 main properties: stay visible when empty, only use "remaining" spaces from its neighbor.
    NoTabBar                ,  // Local, Saved  // Tab bar is completely unavailable. No triangle in the corner to enable it back.
    HiddenTabBar            ,  // Local, Saved  // Tab bar is hidden, with a triangle in the corner to show it again (NB: actual tab-bar instance may be destroyed as this is only used for single-window tab bar)
    NoWindowMenuButton      ,  // Local, Saved  // Disable window/docking menu (that one that appears instead of the collapse button)
    NoCloseButton           ,  // Local, Saved  //
    NoDocking               ,  // Local, Saved  // Disable any form of docking in this dockspace or individual node. (On a whole dockspace, this pretty much defeat the purpose of using a dockspace at all). Note: when turned on, existing docked nodes will be preserved.
    NoDockingSplitMe        ,  // [EXPERIMENTAL] Prevent another window/node from splitting this node.
    NoDockingSplitOther     ,  // [EXPERIMENTAL] Prevent this node from splitting another window/node.
    NoDockingOverMe         ,  // [EXPERIMENTAL] Prevent another window/node to be docked over this node.
    NoDockingOverOther      ,  // [EXPERIMENTAL] Prevent this node to be docked over another window or non-empty node.
    NoDockingOverEmpty      ,  // [EXPERIMENTAL] Prevent this node to be docked over an empty node (e.g. DockSpace with no other windows)
    NoResizeX               ,  // [EXPERIMENTAL]
    NoResizeY               ,  // [EXPERIMENTAL]
    SharedFlagsInheritMask,
}

impl Default for DockNodeFlags {
    fn default() -> Self {
        Self::None
    }
}

pub const NO_RESIZE_FLAGS_MASK: HashSet<DockNodeFlags>       = HashSet::from([DockNodeFlags::NoResize, DockNodeFlags::NoResizeX, DockNodeFlags::NoResizeY]);

pub const LOCAL_FLAGS_MASK: HashSet<DockNodeFlags> = HashSet::from([DockNodeFlags::NoSplit, DockNodeFlags::AutoHideTabBar, DockNodeFlags::DockSpace, DockNodeFlags::CentralNode, DockNodeFlags::NoTabBar, DockNodeFlags::HiddenTabBar, DockNodeFlags::NoWindowMenuButton, DockNodeFlags::NoCloseButton, DockNodeFlags::NoDocking, DockNodeFlags::NoResize, DockNodeFlags::NoResizeX, DockNodeFlags::NoResizeY]);

pub const LOCAL_FLAGS_TRANSFER_MASK: HashSet<DockNodeFlags>  = LOCAL_FLAGS_MASK;

// When splitting those flags are moved to the inheriting child, never duplicated
pub const SAVED_FLAGS_MASK: HashSet<DockNodeFlags>          = HashSet::from([DockNodeFlags::NoResize, DockNodeFlags::NoResizeX, DockNodeFlags::NoResizeY, DockNodeFlags::DockSpace, DockNodeFlags::CentralNode, DockNodeFlags::NoTabBar, DockNodeFlags::HiddenTabBar, DockNodeFlags::NoWindowMenuButton, DockNodeFlags::NoCloseButton, DockNodeFlags::NoDocking]);

#[derive(Default,Debug,Clone)]
pub struct DockNode
{
    // DimgId                 id;
    pub id: Id32,
    // ImGuiDockNodeFlags      shared_flags;                // (Write) flags shared by all nodes of a same dockspace hierarchy (inherited from the root node)
    pub shared_flags: HashSet<DockNodeFlags>,
    // ImGuiDockNodeFlags      local_flags;                 // (Write) flags specific to this node
    pub local_flags: HashSet<DockNodeFlags>,
    // ImGuiDockNodeFlags      local_flags_in_windows;        // (Write) flags specific to this node, applied from windows
    pub local_flags_in_windows: HashSet<DockNodeFlags>,
    // ImGuiDockNodeFlags      merged_flags;                // (Read)  Effective flags (== shared_flags | LocalFlagsInNode | local_flags_in_windows)
    pub merged_flags: HashSet<DockNodeFlags>,
    // ImGuiDockNodeState      state;
    pub state: DockNodeState,
    // ImGuiDockNode*          parent_node;
    pub parent_node_id: Id32, //*mut ImGuiDockNode,
    // ImGuiDockNode*          child_nodes[2];              // [split node only] Child nodes (left/right or top/bottom). Consider switching to an array.
    pub child_nodes: Vec<Id32>, //[*mut ImGuiDockNode;2],
    // ImVector<ImGuiWindow*>  windows;                    // Note: unordered list! Iterate tab_bar->Tabs for user-order.
    pub windows: Vec<Id32>,
    // ImGuiTabBar*            tab_bar;
    pub tab_bar: Option<TabBar>, //*mut ImGuiTabBar,
    // DimgVec2D                  pos;                        // current position
    // pub pos: DimgVec2D,
    pub pos: Vector2D,
// DimgVec2D                  size;                       // current size
    pub size: Vector2D,
    // DimgVec2D                  size_ref;                    // [split node only] Last explicitly written-to size (overridden when using a splitter affecting the node), used to calculate size.
    pub size_ref: Vector2D,
    // ImGuiAxis               split_axis;                  // [split node only] split axis (x or Y)
    pub split_axis: Axis,
    // ImGuiWindowClass        window_class;                // [Root node only]
    pub window_class: WindowClass,
    // ImU32                   last_bg_color;
    pub last_bg_color: u32,
    // ImGuiWindow*            host_window;
    pub host_window_id: Id32, //*mut ImGuiWindow,
    // ImGuiWindow*            visible_window;              // Generally point to window which is id is == SelectedTabID, but when CTRL+Tabbing this can be a different window.
    pub visible_window: Id32, //*mut ImGuiWindow,
    // ImGuiDockNode*          central_node;                // [Root node only] Pointer to central node.
    pub central_node: Id32, // *mut ImGuiDockNode,
    // ImGuiDockNode*          OnlyNodeWithWindows;        // [Root node only] Set when there is a single visible node within the hierarchy.
    pub only_node_with_window: Id32, // *mut ImGuiDockNode,
    // int                     count_node_with_windows;       // [Root node only]
    pub count_node_with_windows: i32,
    // int                     last_frame_alive;             // Last frame number the node was updated or kept alive explicitly with DockSpace() + ImGuiDockNodeFlags_KeepAliveOnly
    pub last_frame_alive: i32,
    // int                     last_frame_active;            // Last frame number the node was updated.
    pub last_frame_active: i32,
    // int                     LastFrameFocused;           // Last frame number the node was focused.
    pub last_grame_focused: i32,
    // DimgId                 last_focused_node_id;          // [Root node only] Which of our child docking node (any ancestor in the hierarchy) was last focused.
    pub last_focused_node_id: Id32,
    // DimgId                 selected_tab_id;              // [Leaf node only] Which of our tab/window is selected.
    pub selected_tab_id: Id32,
    // DimgId                 want_close_tab_id;             // [Leaf node only] Set when closing a specific tab/window.
    pub want_close_tab_id: Id32,
    // ImGuiDataAuthority      authority_for_pos         :3;
    pub authority_for_pos: DataAuthority,
    // ImGuiDataAuthority      authority_for_size        :3;
    pub authority_for_size: DataAuthority,
    // ImGuiDataAuthority      authority_for_viewport    :3;
    pub authority_for_viewport: DataAuthority,
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

impl DockNode {
    //
// ImGuiDockNode::ImGuiDockNode(ImGuiID id)
// {
//     ID = id;
//     SharedFlags = LocalFlags = LocalFlagsInWindows = MergedFlags = ImGuiDockNodeFlags_None;
//     ParentNode = ChildNodes[0] = ChildNodes[1] = NULL;
//     TabBar = NULL;
//     SplitAxis = ImGuiAxis_None;
//
//     State = ImGuiDockNodeState_Unknown;
//     LastBgColor = IM_COL32_WHITE;
//     HostWindow = VisibleWindow = NULL;
//     CentralNode = OnlyNodeWithWindows = NULL;
//     CountNodeWithWindows = 0;
//     LastFrameAlive = LastFrameActive = LastFrameFocused = -1;
//     LastFocusedNodeId = 0;
//     SelectedTabId = 0;
//     WantCloseTabId = 0;
//     AuthorityForPos = AuthorityForSize = ImGuiDataAuthority_DockNode;
//     AuthorityForViewport = ImGuiDataAuthority_Auto;
//     IsVisible = true;
//     IsFocused = HasCloseButton = HasWindowMenuButton = HasCentralNodeChild = false;
//     IsBgDrawnThisFrame = false;
//     WantCloseAll = WantLockSizeOnce = WantMouseMove = WantHiddenTabBarUpdate = WantHiddenTabBarToggle = false;
// }
//
// ImGuiDockNode::~ImGuiDockNode()
// {
//     IM_DELETE(TabBar);
//     TabBar = NULL;
//     ChildNodes[0] = ChildNodes[1] = NULL;
// }


    // ImGuiDockNode(DimgId id);
    pub fn new(id: Id32) -> Self {
        todo!()
    }
    //     ~ImGuiDockNode();
    //     bool                    is_root_node() const      { return parent_node == NULL; }
    pub fn is_root_node(&self) -> bool {
        self.parent_node_id > 0 && self.parent_node_id < Id32::MAX
    }
    //     bool                    is_dock_space() const     { return (merged_flags & ImGuiDockNodeFlags_DockSpace) != 0; }
    pub fn is_dock_space(&self) -> bool {
        // (&self.merged_flags & DimgDockNodeFlags::DockSpace) != 0
        self.merged_flags.contains(&DockNodeFlags::DockSpace) == false
    }
    //     bool                    is_floating_node() const  { return parent_node == NULL && (merged_flags & ImGuiDockNodeFlags_DockSpace) == 0; }
    pub fn is_floating_node(&self) -> bool {
        // self.parent_node.is_null() && &self.merged_flags & DimgDockNodeFlags::DockSpace == 0
        self.is_root_node() == false && self.merged_flags.contains(&DockNodeFlags::DockSpace) == false
    }
    //     bool                    is_central_node() const   { return (merged_flags & ImGuiDockNodeFlags_CentralNode) != 0; }
    pub fn is_central_node(&self) -> bool {
        self.merged_flags.contains(&DockNodeFlags::CentralNode) == false
    }
    //     bool                    is_hidden_tab_bar() const  { return (merged_flags & ImGuiDockNodeFlags_HiddenTabBar) != 0; } // hidden tab bar can be shown back by clicking the small triangle
    pub fn is_hidden_tab_bar(&self) -> bool {
        self.merged_flags.contains(&DockNodeFlags::HiddenTabBar) == false
    }
    //     bool                    is_no_tab_bar() const      { return (merged_flags & ImGuiDockNodeFlags_NoTabBar) != 0; }     // Never show a tab bar
    pub fn is_no_tab_bar(&self) -> bool {
        self.merged_flags.contains(&DockNodeFlags::NoTabBar)
    }
    //     bool                    is_split_node() const     { return child_nodes[0] != NULL; }
    pub fn is_split_node(&self) -> bool {
        self.child_nodes[0] != INVALID_ID
    }
    //     bool                    is_leaf_node() const      { return child_nodes[0] == NULL; }
    pub fn is_leaf_node(&self) -> bool {
        self.child_nodes[0] == INVALID_ID
    }
    //     bool                    is_empty() const         { return child_nodes[0] == NULL && windows.size == 0; }
    pub fn is_empty(&self) -> bool {
        // self.child_nodes[0].is_null() && self.windows.is_empty()
        self.child_nodes[0] == INVALID_ID && self.child_nodes[1] == INVALID_ID && self.windows.is_empty()
    }
    //     ImRect                  rect() const            { return ImRect(pos.x, pos.y, pos.x + size.x, pos.y + size.y); }
    pub fn rect(&self) -> Rect {
        Rect::new4(self.pos.x, self.pos.y, self.pos.x + self.size.x, self.pos.y + self.size.y)
    }
    //
    //     void                    set_local_flags(ImGuiDockNodeFlags flags) { local_flags = flags; UpdateMergedFlags(); }
    pub fn set_local_flags(&mut self, flags: &HashSet<DockNodeFlags>) {
        // self.local_flags = flags;
        for flag in flags {
            self.local_flags.insert(flag.clone());
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
pub enum DockNodeState
{
    Unknown,
    HostWindowHiddenBecauseSingleWindow,
    HostWindowHiddenBecauseWindowsAreResizing,
    HostWindowVisible
}

impl Default for DockNodeState {
    fn default() -> Self {
        Self::Unknown
    }
}

// Persistent Settings data, stored contiguously in SettingsNodes (sizeof() ~32 bytes)
#[derive(Debug,Clone, Default)]
pub struct DockNodeSettings
{
    // ImGuiID             id;
    pub id: Id32,
    // ImGuiID             parent_node_id;
    pub parent_node_id: Id32,
    // ImGuiID             ParentWindowId;
    pub parent_window_id: Id32,
    // ImGuiID             SelectedTabId;
    pub selected_tab_id: Id32,
    // signed char         SplitAxis;
    pub split_axis: i8,
    // char                Depth;
    pub depth: i8,
    // ImGuiDockNodeFlags  flags;                  // NB: We save individual flags one by one in ascii format (ImGuiDockNodeFlags_SavedFlagsMask_)
    pub flags: DockNodeFlags,
    // Vector2Dih            pos;
    pub pos: Vector2D,
    // Vector2Dih            size;
    pub size: Vector2D,
    // Vector2Dih            SizeRef;
    pub size_ref: Vector2D,
    // ImGuiDockNodeSettings() { memset(this, 0, sizeof(*this)); SplitAxis = ImGuiAxis_None; }
}

// Docking
// (some functions are only declared in imgui.cpp, see Docking section)
//  void          DockContextInitialize(ImGuiContext* ctx);
//  void          DockContextShutdown(ImGuiContext* ctx);
//  void          DockContextClearNodes(ImGuiContext* ctx, ImGuiID root_id, bool clear_settings_refs); // Use root_id==0 to clear all
//  void          DockContextRebuildNodes(ImGuiContext* ctx);
//  void          DockContextNewFrameUpdateUndocking(ImGuiContext* ctx);
//  void          DockContextNewFrameUpdateDocking(ImGuiContext* ctx);
//  void          DockContextEndFrame(ImGuiContext* ctx);
//  ImGuiID       dock_context_gen_node_id(ImGuiContext* ctx);
//  void          DockContextQueueDock(ImGuiContext* ctx, ImGuiWindow* target, ImGuiDockNode* target_node, ImGuiWindow* payload, ImGuiDir split_dir, float split_ratio, bool split_outer);
//  void          DockContextQueueUndockWindow(ImGuiContext* ctx, ImGuiWindow* window);
//  void          DockContextQueueUndockNode(ImGuiContext* ctx, ImGuiDockNode* node);
//  bool          DockContextCalcDropPosForDocking(ImGuiWindow* target, ImGuiDockNode* target_node, ImGuiWindow* payload, ImGuiDir split_dir, bool split_outer, Vector2D* out_pos);
//  bool          DockNodeBeginAmendTabBar(ImGuiDockNode* node);
//  void          DockNodeEndAmendTabBar();
/// inline ImGuiDockNode*   DockNodeGetRootNode(ImGuiDockNode* node)                 { while (node->parent_node) node = node->parent_node; return node; }
pub fn dock_node_get_root_node<'context>(g: &'context mut Context, node: &mut DockNode) -> &'context mut DockNode {
    let mut out_node: &mut DockNode = node;
    while out_node.parent_node_id != INVALID_ID {
        out_node = g.get_dock_node(out_node.parent_node_id).unwrap()
    }
    node
}

// static int  DockNodeComparerDepthMostFirst(const void* lhs, const void* rhs)
pub fn dock_node_comparer_depth_most_first(g: &mut Context, lhs: &Vec<u8>, rhs: &Vec<u8>) -> i32
{
    const ImGuiDockNode* a = *(const ImGuiDockNode* const*)lhs;
    const ImGuiDockNode* b = *(const ImGuiDockNode* const*)rhs;
    return DockNodeGetDepth(b) - DockNodeGetDepth(a);
}

// int DockNodeGetTabOrder(ImGuiWindow* window)
pub fn dock_node_get_tab_order(g: &mut Context, window: &mut Window) -> i32
{
    // ImGuiTabBar* tab_bar = window.dock_node_id.tab_bar;
    let tab_bar = &mut g.get_dock_node(window.dock_node_id).unwrap().tab_bar;
    // if (tab_bar == NULL)
    //     return -1
    if tab_bar == INVALID_ID {
        return -1;
    }
    // ImGuiTabItem* tab = TabBarFindTabByID(tab_bar, window.tab_id);
    let tab = tab_bar_find_tab_by_id(g, tab_bar, window.tab_id);
    // return tab ? tab_bar.GetTabOrder(tab) : -1;
    return if tab != INVALID_ID {
        tab_bar.get_tab_order(g, tab)
    } else {
        -1
    }
}

// static void dock_node_hide_window_during_host_window_creation(ImGuiWindow* window)
pub fn dock_node_hide_window_during_host_window_creation(g: &mut Context, window: &mut Window)
{
    window.hidden = true;
    window.hidden_frames_can_skip_items = if window.active { 1 } else { 2 };
}
