use std::collections::HashSet;
use crate::axis::Axis;
use crate::context::Context;
use crate::types::{DataAuthority, Direction};
use crate::types::Id32;
use crate::{dock, window};
use crate::color::StyleColor;
use crate::dock::context::dock_context_remove_node;
use crate::dock::defines::DOCKING_SPLITTER_SIZE;
use crate::dock::{int, node, settings};
use crate::dock::preview::DockPreviewData;
use crate::draw::draw_list::get_foreground_draw_list;
use crate::frame::get_frame_height;
use crate::input::mouse::{start_mouse_moving_window, start_mouse_moving_window_or_node};
use crate::input::NavLayer;
use crate::item::ItemFlags;
use crate::rect::Rect;
use crate::settings::mark_ini_settings_dirty;
use crate::style::{get_color_u32, pop_style_color, push_style_color};
use crate::tab_bar::TabBar;
use crate::types::INVALID_ID;
use crate::utils::{add_hash_set, extend_hash_set};
use crate::vectors::ImLengthSqr;
use crate::vectors::two_d::Vector2D;
use crate::window::class::WindowClass;
use crate::window::{Window, WindowFlags, WINDOWS_HOVER_PADDING, WINDOWS_RESIZE_FROM_EDGES_FEEDBACK_TIMER};
use crate::window::lifecycle::update_window_parent_and_root_links;
use crate::window::pos::set_window_pos;
use crate::window::size::set_window_size;


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
    pub visible_window_id: Id32, //*mut ImGuiWindow,
    // ImGuiDockNode*          central_node;                // [Root node only] Pointer to central node.
    pub central_node_id: Id32, // *mut ImGuiDockNode,
    // ImGuiDockNode*          only_node_with_windows;        // [Root node only] Set when there is a single visible node within the hierarchy.
    pub only_node_with_window: Id32, // *mut ImGuiDockNode,
    // int                     count_node_with_windows;       // [Root node only]
    pub count_node_with_windows: i32,
    // int                     last_frame_alive;             // Last frame number the node was updated or kept alive explicitly with DockSpace() + ImGuiDockNodeFlags_KeepAliveOnly
    pub last_frame_alive: usize,
    // int                     last_frame_active;            // Last frame number the node was updated.
    pub last_frame_active: usize,
    // int                     LastFrameFocused;           // Last frame number the node was focused.
    pub last_frame_focused: usize,
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
//     State = DockNodeState::Unknown;
//     LastBgColor = IM_COL32_WHITE;
//     HostWindow = VisibleWindow = NULL;
//     CentralNode = only_node_with_windows = NULL;
//     count_node_with_windows = 0;
//     LastFrameAlive = LastFrameActive = LastFrameFocused = -1;
//     last_focused_node_id = 0;
//     SelectedTabId = 0;
//     WantCloseTabId = 0;
//     AuthorityForPos = AuthorityForSize = ImGuiDataAuthority_DockNode;
//     AuthorityForViewport = ImGuiDataAuthority_Auto;
//     is_visible = true;
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
    //     bool                    is_empty() const         { return child_nodes[0] == NULL && windows.len() == 0; }
    pub fn is_empty(&self) -> bool {
        // self.child_nodes[0].is_null() && self.windows.is_empty()
        self.child_nodes[0] == INVALID_ID && self.child_nodes[1] == INVALID_ID && self.windows.is_empty()
    }
    //     ImRect                  rect() const            { return ImRect(pos.x, pos.y, pos.x + size.x, pos.y + size.y); }
    pub fn rect(&self) -> Rect {
        Rect::new4(self.pos.x, self.pos.y, self.pos.x + self.size.x, self.pos.y + self.size.y)
    }
    //
    //     void                    set_local_flags(ImGuiDockNodeFlags flags) { local_flags = flags; update_merged_flags(); }
    pub fn set_local_flags(&mut self, flags: &HashSet<DockNodeFlags>) {
        // self.local_flags = flags;
        for flag in flags {
            self.local_flags.insert(flag.clone());
        }
        self.update_merged_flags();
    }
    //     void                    update_merged_flags()     { merged_flags = shared_flags | local_flags | local_flags_in_windows; }
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

// static void DockNodeAddWindow(ImGuiDockNode* node, ImGuiWindow* window, bool add_to_tab_bar)
pub fn dock_node_add_window(g: &mut Context, node: &mut DockNode, window: &mut Window, add_to_tab_bar: bool)
{
    // ImGuiContext& g = *GImGui; (void)g;
    if window.dock_node_id != INVALID_ID
    {
        // Can overwrite an existing window->dock_node (e.g. pointing to a disabled DockSpace node)
        // IM_ASSERT(window.dock_node.ID != node.ID);
        let dock_node_a = g.get_dock_node(window.dock_node_id);
        dock_node_remove_window(g, dock_node_a.unwrap(), window, 0);
    }
    // IM_ASSERT(window.dock_node == NULL || window.DockNodeAsHost == NULL);
    // IMGUI_DEBUG_LOG_DOCKING("[docking] DockNodeAddWindow node 0x%08X window '%s'\n", node.ID, window.Name);

    // If more than 2 windows appeared on the same frame leading to the creation of a new hosting window,
    // we'll hide windows until the host window is ready. Hide the 1st window after its been output (so it is not visible for one frame).
    // We will call dock_node_hide_window_during_host_window_creation() on ourselves in Begin()
    if node.host_window_id == INVALID_ID && node.windows.len() == 1 && g.get_window(node.windows[0]).unwrap().was_active == false {
        dock_node_hide_window_during_host_window_creation(g, g.get_window(node.windows[0]).unwrap());
    }

    node.windows.push_back(window);
    node.want_hiddent_tab_bar_update = true;
    window.dock_node_id = node.id;
    window.dock_id = node.id;
    window.dock_is_active = (node.windows.len() > 1);
    window.dock_tab_want_close = false;

    // When reactivating a node with one or two loose window, the window pos/size/viewport are authoritative over the node storage.
    // In particular it is important we init the viewport from the first window so we don't create two viewports and drop one.
    if node.host_window_id == INVALID_ID && node.is_floating_node()
    {
        if node.authority_for_pos == DataAuthority::Auto {
            node.authority_for_pos = DataAuthority::Window;
        }
        if node.authority_for_size == DataAuthority::Auto {
            node.authority_for_size = DataAuthority::Window;
        }
        if node.authority_for_viewport == DataAuthority::Auto {
            node.authority_for_viewport = DataAuthority::Window;
        }
    }

    // Add to tab bar if requested
    if add_to_tab_bar
    {
        if node.tab_bar == None
        {
            dock_node_add_tab_bar(g, node);
            node.tab_bar.selected_tab_id = node.selected_tab_id;
            node.tab_bar.next_selected_tab_id = node.selected_tab_id;

            // Add existing windows
            // for (int n = 0; n < node.windows.len() - 1; n += 1){
            for win_id in node.windows.iter() {
                let win_a = g.get_window(win_id)
                tab_bar_add_tab(g, &mut node.tab_bar, TabItemFlags::None, win_a);
            }
        }
        tab_bar_add_tab(&mut node.tab_bar, TabItemFlags::Unsorted, window);
    }

    dock_node_update_visible_flag(g, node);

    // Update this without waiting for the next time we Begin() in the window, so our host window will have the proper title bar color on its first frame.
    if node.host_window_id != INVALID_ID {
        let mut flags = window.flags.clone();
        flags.insert(WindowFlags::ChildWIndow);
        let mut parent_win = g.get_window(node.host_window_id);
        update_window_parent_and_root_links(g, window, &mut flags, Some(parent_win));
    }
}

// static void DockNodeRemoveWindow(ImGuiDockNode* node, ImGuiWindow* window, ImGuiID save_dock_id)
pub fn dock_node_remove_window(g: &mut Context, node: &mut DockNode, window: &mut window::Window, save_dock_id: Id32)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(window.dock_node == node);
    //IM_ASSERT(window->root_window_dock_tree == node->host_window);
    //IM_ASSERT(window->last_frame_active < g.frame_count);    // We may call this from Begin()
    // IM_ASSERT(save_dock_id == 0 || save_dock_id == node.ID);
    // IMGUI_DEBUG_LOG_DOCKING("[docking] DockNodeRemoveWindow node 0x%08X window '%s'\n", node.ID, window.Name);

    window.dock_node_id = INVALID_ID;
    window.dock_is_active = false;
    window.dock_tab_want_close = false;
    window.dock_id = save_dock_id;
    window.flags.remove(&WindowFlags::ChildWindow); // &= ~WindowFlags::ChildWindow;
    if window.parent_window_id != INVALID_ID {
        let mut parent_win = g.get_window(window.parent_window_id).unwrap();
        // window.parent_window.DC.ChildWindows.find_erase(window);
        parent_win.dc.child_windows.find_erase(window);
    }
    update_window_parent_and_root_links(g, window, &mut window.flags, None); // Update immediately

    // Remove window
    // bool erased = false;
    let mut erased = false;
    // for (int n = 0; n < node.windows.len(); n += 1){
    for win_id in node.windows.iter() {
        if win_id == window.id {
        // if (node.windows[n] == window) {
            node.windows.erase(node.windows.data + n);
            erased = true;
            break;
        }
    }
    if !erased {}
        // IM_ASSERT(erased);
    if node.visible_window_id == window.id {
        node.visible_window_id = INVALID_ID;
    }

    // Remove tab and possibly tab bar
    node.want_hiddent_tab_bar_update = true;
    if node.tab_bar.is_some()
    {
        tab_bar_remove_tab(&node.tab_bar, window.tab_id);
        // const int tab_count_threshold_for_tab_bar = node.is_central_node() ? 1 : 2;
        let tab_count_threshold_for_tab_bar: i32 = if node.is_central_node() {1} else {2};
        if node.windows.len() < tab_count_threshold_for_tab_bar {
            dock_node_remove_tab_bar(g, node);
        }
    }

    if node.windows.len() == 0 && !node.is_central_node() && !node.is_dock_space() && window.dock_id != node.id
    {
        // Automatic dock node delete themselves if they are not holding at least one tab
        dock_context_remove_node(g, node, true);
        return;
    }

    if node.windows.len() == 1 && !node.is_central_node() && node.host_window_id != INVALID_ID
    {
        // ImGuiWindow* remaining_window = node.windows[0];
        let remaining_window = g.get_window(node.windows[0]);
        if node.host_window_id.viewport_owned && node.is_root_node()
        {
            // Transfer viewport back to the remaining loose window
            // IMGUI_DEBUG_LOG_VIEWPORT("[viewport] Node %08X transfer viewport %08X=>%08X for window '%s'\n", node.id, node.host_window_id.Viewport.id, remaining_window.id, remaining_window.Name);
            // IM_ASSERT(node.host_window.Viewport.Window == node.host_window);
            // node.host_window_id.Viewport.Window = remaining_window;
            // node.host_window_id.Viewport.id = remaining_window.id;
            let host_win = g.get_window(node.host_window_id);
            let vp_a = g.get_viewport(host_win.viewport_id).unwrap();
            vp_a.window_id = remaining_window.id;
            vp_a.id = remaining_window.id;
        }
        remaining_window.collapsed = node.host_window_id.collapsed;
    }

    // Update visibility immediately is required so the DockNodeUpdateRemoveInactiveChilds() processing can reflect changes up the tree
    dock_node_update_visible_flag(g, node);
}

// static void dock_node_move_child_nodes(ImGuiDockNode* dst_node, ImGuiDockNode* src_node)
pub fn dock_node_move_child_nodes(g: &mut Context, dst_node: &mut DockNode, src_node: &DockNode)
{
    // IM_ASSERT(dst_node.Windows.size == 0);
    dst_node.child_nodes[0] = src_node.child_nodes[0];
    dst_node.child_nodes[1] = src_node.child_nodes[1];
    if dst_node.child_nodes[0] != INVALID_ID {
        // dst_node.child_nodes[0].parent_node = dst_node;
        let node = g.get_dock_node(dst_node.child_nodes[0]).unwrap();
        node.parent_node_id = dst_node.id;
    }
    if dst_node.child_nodes[1] {
        // dst_node.child_nodes[1].parent_node = dst_node;
        let node = g.get_dock_node(dst_node.child_nodes[1]).unwrap();
        node.parent_node_id = dst_node.id;
    }
    dst_node.split_axis = src_node.split_axis.clone();
    dst_node.size_ref = src_node.size_ref.clone();
    src_node.child_nodes[0] = INVALID_ID;
    src_node.child_nodes[1] = INVALID_ID;
}

// static void DockNodeMoveWindows(ImGuiDockNode* dst_node, ImGuiDockNode* src_node)
pub fn dock_node_move_windows(g: &mut Context, dst_node: &mut DockNode, src_node: &mut DockNode)
{
    // Insert tabs in the same orders as currently ordered (node->windows isn't ordered)
    // IM_ASSERT(src_node && dst_node && dst_node != src_node);
    // ImGuiTabBar* src_tab_bar = src_node.tab_bar;
    let src_tab_bar = &mut src_node.tab_bar;
    if src_tab_bar.is_some() {}
        // IM_ASSERT(src_node.Windows.size <= src_node.TabBar.Tabs.size);

    // If the dst_node is empty we can just move the entire tab bar (to preserve selection, scrolling, etc.)
    let move_tab_bar = (src_tab_bar.is_some()) && (dst_node.tab_bar.is_none());
    if move_tab_bar
    {
        dst_node.tab_bar = src_node.tab_bar.clone();
        src_node.tab_bar = None;
    }

    // for (int n = 0; n < src_node.windows.len(); n += 1)
    let mut n = 0;
    for win_id in src_node.windows.iter()
    {
        // dock_node's tab_bar may have non-window Tabs manually appended by user
        let win = if src_tab_bar.is_some() {
            g.get_window(src_tab_bar.unwrap().tab[n].window_id)
        } else {
            g.get_window(src_node.windows[n])
        };
        // if (ImGuiWindow* window = src_tab_bar ? src_tab_bar.tabs[n].Window : src_node.windows[n])
        // {
        //     window.dock_node = NULL;
        //     window.dock_is_active = false;
        //     node::dock_node_add_window(dst_node, window, move_tab_bar ? false: true);
        // }
        win.dock_node_id = INVALID_ID;
        win.dock_is_active = false;
        dock_node_add_window(g, dst_node, window, if move_tab_bar { false} else { true});
        n += 1;
    }
    src_node.windows.clear();

    if !move_tab_bar && src_node.tab_bar.is_some()
    {
        if dst_node.tab_bar {
            dst_node.tab_bar.selected_tab_id = src_node.tab_bar.selected_tab_id;
        }
        dock_node_remove_tab_bar(g, src_node);
    }
}

// static void dock_node_hide_host_window(ImGuiDockNode* node)
pub fn dock_node_hide_host_window(g: &mut Context, node: &mut DockNode)
{
    if node.host_window_id != INVALID_ID
    {
        let host_win = g.get_window(node.host_window_id);

        if host_win.dock_node_as_host_id == node.id {
            // node.host_window_id.dock_node_as_host = NULL;
            host_win.dock_node_as_host_id = INVALID_ID;
        }
        node.host_window_id = INVALID_ID;
    }

    if node.windows.len() == 1
    {
        node.visible_window_id = node.windows[0];
        // node.windows[0].dock_is_active = false;
        g.get_window(node.windows[0]).dock_is_active = false;
    }

    if node.tab_bar.is_some() {
        dock_node_remove_tab_bar(g, node);
    }
}

// static void DockNodeApplyPosSizeToWindows(ImGuiDockNode* node)
pub fn dock_node_apply_pos_size_to_windows(g: &mut Context, node: &mut DockNode)
{
    // for (int n = 0; n < node.windows.len(); n += 1)
    for win_id in node.windows.iter()
    {
        let node_win = g.get_window(*win_id);
        set_window_pos(g, node_win, &node.pos, Cond::Always); // We don't assign directly to pos because it can break the calculation of SizeContents on next frame
        set_window_size(g, node_win, &node.size, Cond::Always);
    }
}

// Search function called once by root node in DockNodeUpdate()
#[derive(Default,Debug,Clone)]
pub struct DockNodeTreeInfo
{
    // ImGuiDockNode*      CentralNode;
    pub central_node: Id32,
    // ImGuiDockNode*      first_node_with_windows;
    pub first_node_with_windows: Id32,
    // int                 count_nodes_with_windows;
    pub count_nodes_with_windows: i32,
    //ImGuiWindowClass  WindowClassForMerges;
    pub window_class_for_merges: WindowClass,

    // ImGuiDockNodeTreeInfo() { memset(this, 0, sizeof(*this)); }
}

// static void dock_node_find_info(ImGuiDockNode* node, ImGuiDockNodeTreeInfo* info)
pub fn dock_node_find_info(g: &mut Context, node: &mut DockNode, info: &mut DockNodeTreeInfo)
{
    if node.windows.len() > 0
    {
        if info.first_node_with_windows == INVALID_ID {
            info.first_node_with_windows = node.id;
        }
        info.count_nodes_with_windows += 1;
    }
    if node.is_central_node()
    {
        // IM_ASSERT(info.central_node == NULL); // Should be only one
        // IM_ASSERT(node.IsLeafNode() && "If you get this assert: please submit .ini file + repro of actions leading to this.");
        info.central_node = node.id;
    }
    if info.count_nodes_with_windows > 1 && info.central_node != INVALID_ID {
        return;
    }
    if node.child_nodes[0] != INVALID_ID {
        dock_node_find_info(node.child_nodes[0], info);
    }
    if node.child_nodes[1] != INVALID_ID {
        dock_node_find_info(node.child_nodes[1], info);
    }
}

// static ImGuiWindow* DockNodeFindWindowByID(ImGuiDockNode* node, ImGuiID id)
pub fn dock_node_find_window_by_id(g: &mut Context, node: &mut DockNode, id: Id32) -> Option<&mut Window>
{
    // IM_ASSERT(id != 0);
    // for (int n = 0; n < node.windows.len(); n += 1){
    for win_id in node.windows.iter() {
        let win = g.get_window(*win_id);
        // if (node.windows[n].id == id)
        if win.id == id
        {
            return Some(win);
        }
    }
    return None;
}

// - Remove inactive windows/nodes.
// - Update visibility flag.
// static void dock_node_update_flags_and_collapse(ImGuiDockNode* node)
pub fn dock_node_update_flags_and_collapse(g: &mut Context, node: &mut DockNode)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(node.ParentNode == NULL || node.ParentNode.ChildNodes[0] == node || node.ParentNode.ChildNodes[1] == node);

    // Inherit most flags
    if node.parent_node_id != INVALID_ID {
        let parent_node = g.get_dock_node(node.parent_node_id).unwrap();
        let mut flags_to_set = parent_node.shared_flags.clone();
        flags_to_set.insert(DockNodeFlags::SharedFlagsInheritMask);
        node.shared_flags = flags_to_set;
    }

    // Recurse into children
    // There is the possibility that one of our child becoming empty will delete itself and moving its sibling contents into 'node'.
    // If 'node->ChildNode[0]' delete itself, then 'node->ChildNode[1]->windows' will be moved into 'node'
    // If 'node->ChildNode[1]' delete itself, then 'node->ChildNode[0]->windows' will be moved into 'node' and the "remove inactive windows" loop will have run twice on those windows (harmless)
    node.has_central_node_child = false;
    if node.child_nodes[0] != INVALID_ID {
        dock_node_update_flags_and_collapse(g, g.get_dock_node(node.child_nodes[0]).unwrap());
    }
    if node.child_nodes[1] != INVALID_ID {
        dock_node_update_flags_and_collapse(g, g.get_dock_node(node.child_nodes[1]).unwrap());
    }

    // Remove inactive windows, collapse nodes
    // merge node flags overrides stored in windows
    // node.local_flags_in_windows = DockNodeFlags::None;
    node.local_flags_in_windows.clear();
    // for (int window_n = 0; window_n < node.windows.len(); window_n += 1)
    for win_id in node.windows.iter()
    {
        // ImGuiWindow* window = node.windows[window_n];
        let window = g.get_window(*win_id);
        // IM_ASSERT(window.dock_node == node);

        let node_was_active = (node.last_frame_active + 1 == g.frame_count);
        let mut remove = false;
        remove |= node_was_active && (window.last_frame_active + 1 < g.frame_count);
        remove |= node_was_active && (node.want_close_all || node.want_close_tab_id == window.tab_id) && window.has_close_button && !(window.flags.contains(&WindowFlags::UnsavedDocument));  // Submit all _expected_ closure from last frame
        remove |= (window.dock_tab_want_close);
        if remove
        {
            window.dock_tab_want_close = false;
            if node.windows.len() == 1 && !node.is_central_node()
            {
                dock_node_hide_host_window(g, node);
                node.State = DockNodeState::HostWindowHiddenBecauseSingleWindow;
                dock_node_remove_window(g, node, window, node.id); // Will delete the node so it'll be invalid on return
                return;
            }
            dock_node_remove_window(g, node, window, node.id);
            window_n -= 1;
            continue;
        }

        // FIXME-DOCKING: Missing policies for conflict resolution, hence the "Experimental" tag on this.
        //node->LocalFlagsInWindow &= ~window->window_class.DockNodeFlagsOverrideClear;
        node.local_flags_in_windows |= window.window_class.dock_node_flags_override_set;
        node.local_flags_in_windows = add_hash_set(&node.local_flags_in_windows, &window.window_class.dock_node_flags_override_set)
    }
    node.update_merged_flags();

    // Auto-hide tab bar option
    // ImGuiDockNodeFlags node_flags = node.MergedFlags;
    let node_flags = node.merged_flags.clone();
    if node.want_hiddent_tab_bar_update && node.windows.len() == 1 && (node_flags.contains(&DockNodeFlags::AutoHideTabBar)) && !node.is_hidden_tab_bar() {
        node.want_hidden_tab_bar_toggle = true;
    }
    node.want_hiddent_tab_bar_update = false;

    // Cancel toggling if we know our tab bar is enforced to be hidden at all times
    if node.want_hidden_tab_bar_toggle && node.visible_window_id != INVALID_ID && (node.visible_window_id.window_class.dock_node_flags_override_set.contains(&DockNodeFlags::HiddenTabBar)) {
        node.want_hidden_tab_bar_toggle = false;
    }

    // Apply toggles at a single point of the frame (here!)
    if node.windows.len() > 1 {
        let mut flags_to_set = node.local_flags.clone();
        flags_to_set.remove(&DockNodeFlags::HiddenTabBar);
        node.set_local_flags(&flags_to_set);
    }
    else if node.want_hidden_tab_bar_toggle {
        // node.set_local_flags(node.LocalFlags ^ DockNodeFlags::HiddenTabBar);
        let mut flags_to_set = node.local_flags.clone();
        flags_to_set.insert(&DockNodeFlags::HiddenTabBar);
        node.set_local_flags(&flags_to_set);
    }
    node.want_hidden_tab_bar_toggle = false;

    dock_node_update_visible_flag(g, node);
}

// This is rarely called as DockNodeUpdateForRootNode() generally does it most frames.
// static void dock_node_update_has_central_node_child(ImGuiDockNode* node)
pub fn dock_node_update_has_central_node_child(g: &mut Context, node: &mut DockNode)
{
    node.has_central_node_child = false;
    if node.child_nodes[0] != INVALID_ID {
        dock_node_update_has_central_node_child(g, g.get_dock_node(node.child_nodes[0]).unwrap());
    }
    if node.child_nodes[1] != INVALID_ID {
        dock_node_update_has_central_node_child(g, g.get_dock_node(node.child_nodes[1]).unwrap());
    }
    if node.is_root_node()
    {
        // ImGuiDockNode* mark_node = node.central_node;
        let mut mark_node_id = node.central_node_id;
        while mark_node_id != INVALID_ID
        {
            let mut mark_node = g.get_dock_node(mark_node_id).unwrap();
            mark_node.has_central_node_child = true;
            // mark_node = mark_node.parent_node;
            mark_node_id = mark_node.parent_node_id;
        }
    }
}

// static void dock_node_update_visible_flag(ImGuiDockNode* node)
pub fn dock_node_update_visible_flag(g: &mut Context, node: &mut DockNode)
{
    // Update visibility flag
    let mut is_visible = if node.parent_node_id == INVALID { node.is_dock_space() } else { node.is_central_node() };
    is_visible |= (node.windows.len() > 0);
    is_visible |= (node.child_nodes[0] !=INVALID_ID && g.get_dock_node(node.child_nodes[0]).unwrap().is_visible);
    is_visible |= (node.child_nodes[1] != INVALID_ID && g.get_dock_node(node.child_nodes[1]).unwrap().is_visible);
    node.is_visible = is_visible;
}

// static void DockNodeStartMouseMovingWindow(ImGuiDockNode* node, ImGuiWindow* window)
pub fn dock_node_start_mouse_moving_window(g: &mut Context, node: &mut DockNode, window: &mut Window)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(node.WantMouseMove == true);
    start_mouse_moving_window(g, window);
    g.active_id_click_offset = &g.io.mouse_clicked_pos[0] - &node.pos;
    g.moving_window = window; // If we are docked into a non moveable root window, start_mouse_moving_window() won't set g.moving_window. Override that decision.
    node.want_mouse_move = false;
}

// Update central_node, only_node_with_windows, LastFocusedNodeID. Copy window class.
// static void DockNodeUpdateForRootNode(ImGuiDockNode* node)
pub fn dock_node_update_for_root_node(g: &mut Context, node: &mut DockNode)
{
    dock_node_update_flags_and_collapse(g, node);

    // - Setup central node pointers
    // - Find if there's only a single visible window in the hierarchy (in which case we need to display a regular title bar -> FIXME-DOCK: that last part is not done yet!)
    // Cannot merge this with dock_node_update_flags_and_collapse() because first_node_with_windows is found after window removal and child collapsing
    // ImGuiDockNodeTreeInfo info;
    let mut info: DockNodeTreeInfo = DockNodeTreeInfo::default();
    dock_node_find_info(g, node, &mut info);
    node.central_node_id = info.central_node;
    node.only_node_with_windows = if info.count_nodes_with_windows == 1 { info.first_node_with_windows } else  {INVALID_ID};
    node.count_node_with_windows = info.count_nodes_with_windows;
    if node.last_focused_node_id == INVALID_ID && info.first_node_with_windows != INVALID_ID {
        node.last_focused_node_id = info.first_node_with_windows.id;
    }

    // Copy the window class from of our first window so it can be used for proper dock filtering.
    // When node has mixed windows, prioritize the class with the most constraint (docking_allow_unclassed = false) as the reference to copy.
    // FIXME-DOCK: We don't recurse properly, this code could be reworked to work from DockNodeUpdateScanRec.
    // if ImGuiDockNode* first_node_with_windows = info.first_node_with_windows
    let first_node_with_windows = g.get_dock_node(info.first_node_with_windows);
    if first_node_with_windows.is_some()
    {
        let first_node: &mut DockNode = first_node_with_windows.some();
        let win0 = g.get_window(first_node.windows[0]);
        node.window_class = win0.window_class.clone();
        // for (int n = 1; n < first_node_with_windows.windows.len(); n += 1)
        for win_id in first_node.windows.iter()
        {
            let win = g.get_window(*win_id);
            if win.window_class.docking_allow_unclassed == false {
                node.window_class =win.window_class.clone();
                break;
            }
        }
    }

    // ImGuiDockNode* mark_node = node.central_node_id;
    let mut mark_node_id = node.central_node_id;
    while mark_node_id
    {
        let mark_node = g.get_dock_node(mark_node_id).unwrap();
        mark_node.has_central_node_child = true;
        mark_node_id = mark_node.parent_node_id;
    }
}

// static void DockNodeSetupHostWindow(ImGuiDockNode* node, ImGuiWindow* host_window)
pub fn dock_node_setup_host_window(g: &mut Context, node: &mut DockNode, host_window: &mut window::Window)
{
    // Remove ourselves from any previous different host window
    // This can happen if a user mistakenly does (see #4295 for details):
    //  - N+0: DockBuilderAddNode(id, 0)    // missing ImGuiDockNodeFlags_DockSpace
    //  - N+1: NewFrame()                   // will create floating host window for that node
    //  - N+1: DockSpace(id)                // requalify node as dockspace, moving host window
    if node.host_window_id != INVALID_ID && node.host_window_id != host_window.id && g.get_window(node.host_window_id).dock_node_as_host_id == node.id {
        g.get_window(node.host_window_id).dock_node_as_host_id = INVALID_ID;
    }

    host_window.dock_node_as_host_id = node.id;
    node.host_window_id = host_window.id;
}

// static void DockNodeUpdate(ImGuiDockNode* node)
pub fn dock_node_update(g: &mut Context, node: &mut DockNode)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(node.LastFrameActive != g.frame_count);
    node.last_frame_alive = g.frame_count;
    node.is_bg_drawn_this_frame = false;

    node.central_node_id = node.only_node_with_windows = NULL;
    if (node.is_root_node())
        DockNodeUpdateForRootNode(node);

    // Remove tab bar if not needed
    if (node.tab_bar && node.is_no_tab_bar())
        dock_node_remove_tab_bar(node);

    // Early out for hidden root dock nodes (when all dock_id references are in inactive windows, or there is only 1 floating window holding on the dock_id)
    bool want_to_hide_host_window = false;
    if (node.is_floating_node())
    {
        if (node.windows.len() <= 1 && node.is_leaf_node())
            if (!g.io.ConfigDockingAlwaysTabBar && (node.windows.len() == 0 || !node.windows[0].window_class.DockingAlwaysTabBar))
                want_to_hide_host_window = true;
        if (node.count_node_with_windows == 0)
            want_to_hide_host_window = true;
    }
    if (want_to_hide_host_window)
    {
        if (node.windows.len() == 1)
        {
            // Floating window pos/size is authoritative
            ImGuiWindow* single_window = node.windows[0];
            node.pos = single_window.pos;
            node.size = single_window.sizeFull;
            node.authority_for_pos = node.authority_for_size = node.authority_for_viewport = DataAuthority::Window;

            // Transfer focus immediately so when we revert to a regular window it is immediately selected
            if (node.host_window_id && g.nav_window == node.host_window_id)
                focus_window(single_window);
            if (node.host_window_id)
            {
                single_window.viewport = node.host_window_id.Viewport;
                single_window.viewport_id = node.host_window_id.viewport_id;
                if (node.host_window_id.viewport_owned)
                {
                    single_window.viewport.Window = single_window;
                    single_window.viewport_owned = true;
                }
            }
        }

        dock_node_hide_host_window(node);
        node.State = DockNodeState::HostWindowHiddenBecauseSingleWindow;
        node.want_close_all = false;
        node.want_close_tab_id = 0;
        node.has_close_button = node.HasWindowMenuButton = false;
        node.last_frame_active = g.frame_count;

        if (nodewant_mouse_move && node.windows.len() == 1)
            DockNodeStartMouseMovingWindow(node, node.windows[0]);
        return;
    }

    // In some circumstance we will defer creating the host window (so everything will be kept hidden),
    // while the expected visible window is resizing itself.
    // This is important for first-time (no ini settings restored) single window when io.config_docking_always_tab_bar is enabled,
    // otherwise the node ends up using the minimum window size. Effectively those windows will take an extra frame to show up:
    //   N+0: Begin(): window created (with no known size), node is created
    //   N+1: DockNodeUpdate(): node skip creating host window / Begin(): window size applied, not visible
    //   N+2: DockNodeUpdate(): node can create host window / Begin(): window becomes visible
    // We could remove this frame if we could reliably calculate the expected window size during node update, before the Begin() code.
    // It would require a generalization of CalcWindowExpectedSize(), probably extracting code away from Begin().
    // In reality it isn't very important as user quickly ends up with size data in .ini file.
    if (node.is_visible && node.host_window_id == NULL && node.is_floating_node() && node.is_leaf_node())
    {
        // IM_ASSERT(node.Windows.size > 0);
        ImGuiWindow* ref_window = NULL;
        if (node.selected_tab_id != 0) // Note that we prune single-window-node settings on .ini loading, so this is generally 0 for them!
            ref_window = DockNodefind_window_by_id(node, node.selected_tab_id);
        if (ref_window == NULL)
            ref_window = node.windows[0];
        if (ref_window.auto_fit_frames_x > 0 || ref_window.auto_fit_frames_y > 0)
        {
            node.State = DockNodeState::HostWindowHiddenBecauseWindowsAreResizing;
            return;
        }
    }

    const ImGuiDockNodeFlags node_flags = node.MergedFlags;

    // Decide if the node will have a close button and a window menu button
    node.HasWindowMenuButton = (node.windows.len() > 0) && (node_flags & DockNodeFlags::NoWindowMenuButton) == 0;
    node.has_close_button = false;
    for (int window_n = 0; window_n < node.windows.len(); window_n += 1)
    {
        // FIXME-DOCK: Setting dock_is_active here means that for single active window in a leaf node, dock_is_active will be cleared until the next Begin() call.
        ImGuiWindow* window = node.windows[window_n];
        node.has_close_button |= window.has_close_button;
        window.dock_is_active = (node.windows.len() > 1);
    }
    if (node_flags & DockNodeFlags::NoCloseButton)
        node.has_close_button = false;

    // Bind or create host window
    ImGuiWindow* host_window = NULL;
    bool beginned_into_host_window = false;
    if (node.is_dock_space())
    {
        // [Explicit root dockspace node]
        // IM_ASSERT(node.host_window);
        host_window = node.host_window_id;
    }
    else
    {
        // [Automatic root or child nodes]
        if (node.is_root_node() && node.is_visible)
        {
            ImGuiWindow* ref_window = (node.windows.len() > 0) ? node.windows[0] : NULL;

            // Sync pos
            if (node.authority_for_pos == DataAuthority::Window && ref_window)
                SetNextWindowPos(ref_window.pos);
            else if (node.authority_for_pos == DataAuthority::DockNode)
                SetNextWindowPos(node.pos);

            // Sync size
            if (node.authority_for_size == DataAuthority::Window && ref_window)
                set_next_window_size(ref_window.sizeFull);
            else if (node.authority_for_size == DataAuthority::DockNode)
                set_next_window_size(node.size);

            // Sync collapsed
            if (node.authority_for_size == DataAuthority::Window && ref_window)
                SetNextWindowCollapsed(ref_window.collapsed);

            // Sync viewport
            if (node.authority_for_viewport == DataAuthority::Window && ref_window)
                SetNextWindowViewport(ref_window.viewport_id);

            SetNextWindowClass(&node.window_class);

            // Begin into the host window
            char window_label[20];
            dock_node_get_host_window_title(node, window_label, IM_ARRAYSIZE(window_label));
            ImGuiWindowFlags window_flags = WindowFlags::NoScrollbar | WindowFlags::NoScrollWithMouse | WindowFlags::DockNodeHost;
            window_flags |= WindowFlags::NoFocusOnAppearing;
            window_flags |= WindowFlags::NoSavedSettings | WindowFlags::NoNavFocus | WindowFlags::NoCollapse;
            window_flags |= WindowFlags::NoTitleBar;

            SetNextWindowBgAlpha(0.0); // Don't set ImGuiWindowFlags_NoBackground because it disables borders
            push_style_var(StyleVar::WindowPadding, Vector2D::new(0, 0));
            begin(window_label, NULL, window_flags);
            pop_style_var();
            beginned_into_host_window = true;

            host_window = g.current_window;
            DockNodeSetupHostWindow(node, host_window);
            host_window.dc.cursor_pos = host_window.pos;
            node.pos = host_window.pos;
            node.size = host_window.size;

            // We set ImGuiWindowFlags_NoFocusOnAppearing because we don't want the host window to take full focus (e.g. steal nav_window)
            // But we still it bring it to the front of display. There's no way to choose this precise behavior via window flags.
            // One simple case to ponder if: window A has a toggle to create windows B/C/D. Dock B/C/D together, clear the toggle and enable it again.
            // When reappearing B/C/D will request focus and be moved to the top of the display pile, but they are not linked to the dock host window
            // during the frame they appear. The dock host window would keep its old display order, and the sorting in EndFrame would move B/C/D back
            // after the dock host window, losing their top-most status.
            if (node.host_window_id.appearing)
                BringWindowToDisplayFront(node.host_window_id);

            node.authority_for_pos = node.authority_for_size = node.authority_for_viewport = DataAuthority::Auto;
        }
        else if (node.parent_node)
        {
            node.host_window_id = host_window = node.parent_node.host_window;
            node.authority_for_pos = node.authority_for_size = node.authority_for_viewport = DataAuthority::Auto;
        }
        if (nodewant_mouse_move && node.host_window_id)
            DockNodeStartMouseMovingWindow(node, node.host_window_id);
    }

    // Update focused node (the one whose title bar is highlight) within a node tree
    if (node.is_split_node())
        // IM_ASSERT(node.TabBar == NULL);
    if (node.is_root_node())
        if (g.nav_window && g.nav_window.root_window.dock_node && g.nav_window.root_window.parent_window == host_window)
            node.last_focused_node_id = g.nav_window.root_window.dock_node.id;

    // Register a hit-test hole in the window unless we are currently dragging a window that is compatible with our dockspace
    ImGuiDockNode* central_node = node.central_node_id;
    const bool central_node_hole = node.is_root_node() && host_window && (node_flags & DockNodeFlags::PassthruCentralNode) != 0 && central_node != NULL && central_node.IsEmpty();
    bool central_node_hole_register_hit_test_hole = central_node_hole;
    if (central_node_hole)
        if (const ImGuiPayload* payload = GetDragDropPayload())
            if (payload.IsDataType(IMGUI_PAYLOAD_TYPE_WINDOW) && DockNodeIsDropAllowed(host_window, *(ImGuiWindow**)payload.Data))
                central_node_hole_register_hit_test_hole = false;
    if (central_node_hole_register_hit_test_hole)
    {
        // We add a little padding to match the "resize from edges" behavior and allow grabbing the splitter easily.
        // (But we only add it if there's something else on the other side of the hole, otherwise for e.g. fullscreen
        // covering passthru node we'd have a gap on the edge not covered by the hole)
        // IM_ASSERT(node.is_dock_space()); // We cannot pass this flag without the DockSpace() api. Testing this because we also setup the hole in host_window->parent_node
        ImGuiDockNode* root_node = dock_node_get_root_node(central_node);
        Rect root_rect(root_node.pos, root_node.pos + root_node.size);
        Rect hole_rect(central_node.pos, central_node.pos + central_node.size);
        if (hole_rect.min.x > root_rect.min.x) { hole_rect.min.x += WINDOWS_HOVER_PADDING; }
        if (hole_rect.max.x < root_rect.max.x) { hole_rect.max.x -= WINDOWS_HOVER_PADDING; }
        if (hole_rect.min.y > root_rect.min.y) { hole_rect.min.y += WINDOWS_HOVER_PADDING; }
        if (hole_rect.max.y < root_rect.max.y) { hole_rect.max.y -= WINDOWS_HOVER_PADDING; }
        //GetForegroundDrawList()->add_rect(hole_rect.min, hole_rect.max, IM_COL32(255, 0, 0, 255));
        if (central_node_hole && !hole_rect.is_inverted())
        {
            SetWindowHitTestHole(host_window, hole_rect.min, hole_rect.max - hole_rect.min);
            if (host_window.parent_window)
                SetWindowHitTestHole(host_window.parent_window, hole_rect.min, hole_rect.max - hole_rect.min);
        }
    }

    // Update position/size, process and draw resizing splitters
    if (node.is_root_node() && host_window)
    {
        host_window.draw_list.channels_set_current(1);
        DockNodeTreeUpdatePosSize(node, host_window.pos, host_window.size);
        DockNodeTreeUpdateSplitter(node);
    }

    // Draw empty node background (currently can only be the Central Node)
    if (host_window && node.IsEmpty() && node.is_visible)
    {
        host_window.draw_list.channels_set_current(0);
        node.last_bg_color = (node_flags & DockNodeFlags::PassthruCentralNode) ? 0 : get_color_u32(StyleColor::DockingEmptyBg);
        if (node.last_bg_color != 0)
            host_window.draw_list.add_rect_filled(node.pos, node.pos + node.size, node.last_bg_color);
        node.is_bg_drawn_this_frame = true;
    }

    // Draw whole dockspace background if ImGuiDockNodeFlags_PassthruCentralNode if set.
    // We need to draw a background at the root level if requested by ImGuiDockNodeFlags_PassthruCentralNode, but we will only know the correct pos/size
    // _after_ processing the resizing splitters. So we are using the draw_list channel splitting facility to submit drawing primitives out of order!
    const bool render_dockspace_bg = node.is_root_node() && host_window && (node_flags & DockNodeFlags::PassthruCentralNode) != 0;
    if (render_dockspace_bg && node.is_visible)
    {
        host_window.draw_list.channels_set_current(0);
        if (central_node_hole)
            render_rect_filled_with_hole(host_window.draw_list, node.rect(), central_node.rect(), get_color_u32(StyleColor::WindowBg), 0.0);
        else
            host_window.draw_list.add_rect_filled(node.pos, node.pos + node.size, get_color_u32(StyleColor::WindowBg), 0.0);
    }

    // Draw and populate Tab Bar
    if (host_window)
        host_window.draw_list.channels_set_current(1);
    if (host_window && node.windows.len() > 0)
    {
        DockNodeUpdateTabBar(node, host_window);
    }
    else
    {
        node.want_close_all = false;
        node.want_close_tab_id = 0;
        node.IsFocused = false;
    }
    if (node.tab_bar && node.tab_bar.selected_tab_id)
        node.selected_tab_id = node.tab_bar.selected_tab_id;
    else if (node.windows.len() > 0)
        node.selected_tab_id = node.windows[0].id;

    // Draw payload drop target
    if (host_window && node.is_visible)
        if (node.is_root_node() && (g.moving_window == NULL || g.moving_window.root_window_dock_tree != host_window))
            BeginDockableDragDropTarget(host_window);

    // We update this after DockNodeUpdateTabBar()
    node.last_frame_active = g.frame_count;

    // Recurse into children
    // FIXME-DOCK FIXME-OPT: Should not need to recurse into children
    if (host_window)
    {
        if (node.child_nodes[0])
            DockNodeUpdate(node.child_nodes[0]);
        if (node.child_nodes[1])
            DockNodeUpdate(node.child_nodes[1]);

        // Render outer borders last (after the tab bar)
        if (node.is_root_node())
        {
            host_window.draw_list.channels_set_current(1);
            RenderWindowOuterBorders(host_window);
        }

        // Further rendering (= hosted windows background) will be drawn on layer 0
        host_window.draw_list.channels_set_current(0);
    }

    // End host window
    if (beginned_into_host_window) //-V1020
        end();
}

// static ImGuiID DockNodeUpdateWindowMenu(ImGuiDockNode* node, ImGuiTabBar* tab_bar)
pub fn dock_node_update_window_menu(g: &mut Context, node: &mut DockNode, tab_bar: &mut TabBar) -> Id32
{
    // Try to position the menu so it is more likely to stays within the same viewport
    // ImGuiContext& g = *GImGui;
    ImGuiID ret_tab_id = INVALID_ID;
    if (g.style.window_menu_button_position == Direction::Left)
        SetNextWindowPos(Vector2D::new(node.pos.x, node.pos.y + get_frame_height()), Cond::Always, Vector2D::new(0.0, 0.0));
    else
        SetNextWindowPos(Vector2D::new(node.pos.x + node.size.x, node.pos.y + get_frame_height()), Cond::Always, Vector2D::new(1.0, 0.0));
    if (BeginPopup("#WindowMenu"))
    {
        node.IsFocused = true;
        if (tab_bar.tabs.size == 1)
        {
            if (MenuItem("Hide tab bar", NULL, node.is_hidden_tab_bar()))
                node.want_hidden_tab_bar_toggle = true;
        }
        else
        {
            for (int tab_n = 0; tab_n < tab_bar.tabs.size; tab_n += 1)
            {
                ImGuiTabItem* tab = &tab_bar.tabs[tab_n];
                if (tab.flags & TabItemFlags::Button)
                    continue;
                if (Selectable(tab_bar.GetTabName(tab), tab.id == tab_bar.selected_tab_id))
                    ret_tab_id = tab.id;
                SameLine();
                Text("   ");
            }
        }
        EndPopup();
    }
    return ret_tab_id;
}

// User helper to append/amend into a dock node tab bar. Most commonly used to add e.g. a "+" button.
// bool DockNodeBeginAmendTabBar(ImGuiDockNode* node)
pub fn dock_node_begin_amend_tab_bar(g: &mut Context, node: &mut DockNode) -> bool
{
    if (node.tab_bar == NULL || node.host_window_id == NULL)
        return false;
    if (node.MergedFlags & DockNodeFlags::KeepAliveOnly)
        return false;
    begin(node.host_window_id.Name);
    PushOverrideID(node.id);
    bool ret = BeginTabBarEx(node.tab_bar, node.tab_bar.BarRect, node.tab_bar.flags, node);
    IM_UNUSED(ret);
    // IM_ASSERT(ret);
    return true;
}

// void DockNodeEndAmendTabBar()
pub fn dock_node_end_amend_tab_bar(g: &mut Context)
{
    EndTabBar();
    PopID();
    end();
}

// static bool IsDockNodeTitleBarHighlighted(ImGuiDockNode* node, ImGuiDockNode* root_node, ImGuiWindow* host_window)
pub fn is_dock_node_title_bar_hihglighted(g: &mut Context, node: &mut DockNode, root_node: &mut DockNode, host_window: &mut window::Window) -> bool
{
    // CTRL+Tab highlight (only highlighting leaf node, not whole hierarchy)
    // ImGuiContext& g = *GImGui;
    if (g.nav_windowing_target)
        return (g.nav_windowing_target.dock_node == node);

    // FIXME-DOCKING: May want alternative to treat central node void differently? e.g. if (g.nav_window == host_window)
    if (g.nav_window && g.nav_window.root_window_for_title_bar_highlight == host_window.root_window_dock_tree && root_node.last_focused_node_id == node.id)
        for (ImGuiDockNode* parent_node = g.nav_window.root_window.dock_node; parent_node != NULL; parent_node = parent_node.host_window ? parent_node.host_window.root_window.dock_node : NULL)
            if ((parent_node = dock_node_get_root_node(parent_node)) == root_node)
                return true;
    return false;
}

// Submit the tab bar corresponding to a dock node and various housekeeping details.
// static void DockNodeUpdateTabBar(ImGuiDockNode* node, ImGuiWindow* host_window)
pub fn dock_node_update_tab_bar(g: &mut Context, node: &mut DockNode, host_window: &mut window::Window)
{
    // ImGuiContext& g = *GImGui;
    ImGuiStyle& style = g.style;

    const bool node_was_active = (node.last_frame_active + 1 == g.frame_count);
    const bool closed_all = node.want_close_all && node_was_active;
    const ImGuiID closed_one = node.want_close_tab_id && node_was_active;
    node.want_close_all = false;
    node.want_close_tab_id = 0;

    // Decide if we should use a focused title bar color
    bool is_focused = false;
    ImGuiDockNode* root_node = dock_node_get_root_node(node);
    if (IsDockNodeTitleBarHighlighted(node, root_node, host_window))
        is_focused = true;

    // hidden tab bar will show a triangle on the upper-left (in Begin)
    if (node.is_hidden_tab_bar() || node.is_no_tab_bar())
    {
        node.visible_window_id = (node.windows.len() > 0) ? node.windows[0] : NULL;
        node.IsFocused = is_focused;
        if (is_focused)
            node.LastFrameFocused = g.frame_count;
        if (node.visible_window_id)
        {
            // Notify root of visible window (used to display title in OS task bar)
            if (is_focused || root_node.visible_window == NULL)
                root_node.visible_window = node.visible_window_id;
            if (node.tab_bar)
                node.tab_bar.VisibleTabId = node.visible_window_id.tab_id;
        }
        return;
    }

    // Move ourselves to the Menu layer (so we can be accessed by tapping Alt) + undo skip_items flag in order to draw over the title bar even if the window is collapsed
    bool backup_skip_item = host_window.skip_items;
    if (!node.is_dock_space())
    {
        host_window.skip_items = false;
        host_window.dcnav_layer_current = NavLayer::Menu;
    }

    // Use PushOverrideID() instead of PushID() to use the node id _without_ the host window id.
    // This is to facilitate computing those id from the outside, and will affect more or less only the id of the collapse button, popup and tabs,
    // as docked windows themselves will override the stack with their own root id.
    PushOverrideID(node.id);
    ImGuiTabBar* tab_bar = node.tab_bar;
    bool tab_bar_is_recreated = (tab_bar == NULL); // Tab bar are automatically destroyed when a node gets hidden
    if (tab_bar == NULL)
    {
        dock_node_add_tab_bar(node);
        tab_bar = node.tab_bar;
    }

    ImGuiID focus_tab_id = INVALID_ID;
    node.IsFocused = is_focused;

    const ImGuiDockNodeFlags node_flags = node.MergedFlags;
    const bool has_window_menu_button = (node_flags & DockNodeFlags::NoWindowMenuButton) == 0 && (style.window_menu_button_position != Direction::None);

    // In a dock node, the Collapse Button turns into the window Menu button.
    // FIXME-DOCK FIXME-OPT: Could we recycle popups id across multiple dock nodes?
    if (has_window_menu_button && IsPopupOpen("#WindowMenu"))
    {
        if (ImGuiID tab_id = DockNodeUpdateWindowMenu(node, tab_bar))
            focus_tab_id = tab_bar.next_selected_tab_id = tab_id;
        is_focused |= node.IsFocused;
    }

    // Layout
    Rect title_bar_rect, tab_bar_rect;
    Vector2D window_menu_button_pos;
    Vector2D close_button_pos;
    DockNodeCalcTabBarLayout(node, &title_bar_rect, &tab_bar_rect, &window_menu_button_pos, &close_button_pos);

    // Submit new tabs, they will be added as Unsorted and sorted below based on relative dock_order value.
    const int tabs_count_old = tab_bar.tabs.size;
    for (int window_n = 0; window_n < node.windows.len(); window_n += 1)
    {
        ImGuiWindow* window = node.windows[window_n];
        if (TabBarFindTabByID(tab_bar, window.tab_id) == NULL)
            tab_bar_add_tab(tab_bar, TabItemFlags::Unsorted, window);
    }

    // Title bar
    if (is_focused)
        node.LastFrameFocused = g.frame_count;
    ImU32 title_bar_col = get_color_u32(host_window.collapsed ? StyleColor::TitleBgCollapsed : is_focused ? StyleColor::TitleBgActive : StyleColor::TitleBg);
    ImDrawFlags rounding_flags = calc_rounding_flags_for_rect_in_rect(title_bar_rect, host_window.Rect(), DOCKING_SPLITTER_SIZE);
    host_window.draw_list.add_rect_filled(title_bar_rect.min, title_bar_rect.max, title_bar_col, host_window.WindowRounding, rounding_flags);

    // Docking/Collapse button
    if (has_window_menu_button)
    {
        if (collapse_button(host_window.get_id("#COLLAPSE"), window_menu_button_pos, node)) // == DockNodeGetWindowMenuButtonId(node)
            OpenPopup("#WindowMenu");
        if (IsItemActive())
            focus_tab_id = tab_bar.selected_tab_id;
    }

    // If multiple tabs are appearing on the same frame, sort them based on their persistent dock_order value
    int tabs_unsorted_start = tab_bar.tabs.size;
    for (int tab_n = tab_bar.tabs.size - 1; tab_n >= 0 && (tab_bar.tabs[tab_n].flags & TabItemFlags::Unsorted); tab_n--)
    {
        // FIXME-DOCK: Consider only clearing the flag after the tab has been alive for a few consecutive frames, allowing late comers to not break sorting?
        tab_bar.tabs[tab_n].flags &= ~TabItemFlags::Unsorted;
        tabs_unsorted_start = tab_n;
    }
    if (tab_bar.tabs.size > tabs_unsorted_start)
    {
        // IMGUI_DEBUG_LOG_DOCKING("[docking] In node 0x%08X: %d new appearing tabs:%s\n", node.ID, tab_bar.Tabs.size - tabs_unsorted_start, (tab_bar.Tabs.size > tabs_unsorted_start + 1) ? " (will sort)" : "");
        for (int tab_n = tabs_unsorted_start; tab_n < tab_bar.tabs.size; tab_n += 1)
            // IMGUI_DEBUG_LOG_DOCKING("[docking] - Tab '%s' Order %d\n", tab_bar.Tabs[tab_n].Window.Name, tab_bar.Tabs[tab_n].Window.dock_order);
        if (tab_bar.tabs.size > tabs_unsorted_start + 1)
            ImQsort(tab_bar.tabs.data + tabs_unsorted_start, tab_bar.tabs.size - tabs_unsorted_start, sizeof(ImGuiTabItem), TabItemComparerByDockOrder);
    }

    // Apply nav_window focus back to the tab bar
    if (g.nav_window && g.nav_window.root_window.dock_node == node)
        tab_bar.selected_tab_id = g.nav_window.root_window.id;

    // Selected newly added tabs, or persistent tab id if the tab bar was just recreated
    if (tab_bar_is_recreated && TabBarFindTabByID(tab_bar, node.selected_tab_id) != NULL)
        tab_bar.selected_tab_id = tab_bar.next_selected_tab_id = node.selected_tab_id;
    else if (tab_bar.tabs.size > tabs_count_old)
        tab_bar.selected_tab_id = tab_bar.next_selected_tab_id = tab_bar.tabs.back().Window.tab_id;

    // Begin tab bar
    ImGuiTabBarFlags tab_bar_flags = ImGuiTabBarFlags_Reorderable | ImGuiTabBarFlags_AutoSelectNewTabs; // | ImGuiTabBarFlags_NoTabListScrollingButtons);
    tab_bar_flags |= ImGuiTabBarFlags_SaveSettings | ImGuiTabBarFlags_DockNode;
    if (!host_window.collapsed && is_focused)
        tab_bar_flags |= ImGuiTabBarFlags_IsFocused;
    BeginTabBarEx(tab_bar, tab_bar_rect, tab_bar_flags, node);
    //host_window->draw_list->add_rect(tab_bar_rect.min, tab_bar_rect.max, IM_COL32(255,0,255,255));

    // Backup style colors
    Vector4D backup_style_cols[ImGuiWindowDockStyleCol_COUNT];
    for (int color_n = 0; color_n < ImGuiWindowDockStyleCol_COUNT; color_n += 1)
        backup_style_cols[color_n] = g.style.colors[GWindowDockStyleColors[color_n]];

    // Submit actual tabs
    node.visible_window_id = NULL;
    for (int window_n = 0; window_n < node.windows.len(); window_n += 1)
    {
        ImGuiWindow* window = node.windows[window_n];
        if ((closed_all || closed_one == window.tab_id) && window.has_close_button && !(window.flags & WindowFlags::UnsavedDocument))
            continue;
        if (window.last_frame_active + 1 >= g.frame_count || !node_was_active)
        {
            ImGuiTabItemFlags tab_item_flags = 0;
            tab_item_flags |= window.window_class.TabItemFlagsOverrideSet;
            if (window.flags & WindowFlags::UnsavedDocument)
                tab_item_flags |= TabItemFlags::UnsavedDocument;
            if (tab_bar.flags & ImGuiTabBarFlags_NoCloseWithMiddleMouseButton)
                tab_item_flags |= TabItemFlags::NoCloseWithMiddleMouseButton;

            // Apply stored style overrides for the window
            for (int color_n = 0; color_n < ImGuiWindowDockStyleCol_COUNT; color_n += 1)
                g.style.colors[GWindowDockStyleColors[color_n]] = ColorConvertU32ToFloat4(window.DockStyle.colors[color_n]);

            // Note that TabItemEx() calls TabBarCalcTabID() so our tab item id will ignore the current id stack (rightly so)
            bool tab_open = true;
            TabItemEx(tab_bar, window.Name, window.has_close_button ? &tab_open : NULL, tab_item_flags, window);
            if (!tab_open)
                node.want_close_tab_id = window.tab_id;
            if (tab_bar.VisibleTabId == window.tab_id)
                node.visible_window_id = window;

            // Store last item data so it can be queried with IsItemXXX functions after the user Begin() call
            window.DockTabItemStatusFlags = g.last_item_data.status_flags;
            window.DockTabItemRect = g.last_item_data.Rect;

            // Update navigation id on menu layer
            if (g.nav_window && g.nav_window.root_window == window && (window.dc.nav_layers_active_mask & (1 << NavLayer::Menu)) == 0)
                host_window.NavLastIds[1] = window.tab_id;
        }
    }

    // Restore style colors
    for (int color_n = 0; color_n < ImGuiWindowDockStyleCol_COUNT; color_n += 1)
        g.style.colors[GWindowDockStyleColors[color_n]] = backup_style_cols[color_n];

    // Notify root of visible window (used to display title in OS task bar)
    if (node.visible_window_id)
        if (is_focused || root_node.visible_window == NULL)
            root_node.visible_window = node.visible_window_id;

    // Close button (after visible_window was updated)
    // Note that visible_window may have been overrided by CTRL+Tabbing, so visible_window->tab_id may be != from tab_bar->selected_tab_id
    const bool close_button_is_enabled = node.has_close_button && node.visible_window_id && node.visible_window_id.has_close_button;
    const bool close_button_is_visible = node.has_close_button;
    //const bool close_button_is_visible = close_button_is_enabled; // Most people would expect this behavior of not even showing the button (leaving a hole since we can't claim that space as other windows in the tba bar have one)
    if (close_button_is_visible)
    {
        if (!close_button_is_enabled)
        {
            push_item_flag(ItemFlags::Disabled, true);
            push_style_color(, StyleColor::Text, style.colors[StyleColor::Text] * Vector4D(1.0, 1.0, 1.0, 0.4));
        }
        if (close_button(host_window.get_id("#CLOSE"), close_button_pos))
        {
            node.want_close_all = true;
            for (int n = 0; n < tab_bar.tabs.size; n += 1)
                TabBarCloseTab(tab_bar, &tab_bar.tabs[n]);
        }
        //if (IsItemActive())
        //    focus_tab_id = tab_bar->selected_tab_id;
        if (!close_button_is_enabled)
        {
            pop_style_color();
            pop_item_flag();
        }
    }

    // When clicking on the title bar outside of tabs, we still focus the selected tab for that node
    // FIXME: TabItem use AllowItemOverlap so we manually perform a more specific test for now (hovered || held)
    ImGuiID title_bar_id = host_window.get_id("#TITLEBAR");
    if (g.hovered_id == 0 || g.hovered_id == title_bar_id || g.active_id == title_bar_id)
    {
        bool held;
        button_behavior(title_bar_rect, title_bar_id, NULL, &held, ButtonFlags::AllowItemOverlap);
        if (g.hovered_id == title_bar_id)
        {
            // ImGuiButtonFlags_AllowItemOverlap + SetItemAllowOverlap() required for appending into dock node tab bar,
            // otherwise dragging window will steal hovered_id and amended tabs cannot get them.
            g.last_item_data.id = title_bar_id;
            SetItemAllowOverlap();
        }
        if (held)
        {
            if (IsMouseClicked(0))
                focus_tab_id = tab_bar.selected_tab_id;

            // Forward moving request to selected window
            if (ImGuiTabItem* tab = TabBarFindTabByID(tab_bar, tab_bar.selected_tab_id))
                start_mouse_moving_window_or_node(tab.Window ? tab.Window : node.host_window, node, false);
        }
    }

    // Forward focus from host node to selected window
    //if (is_focused && g.nav_window == host_window && !g.nav_windowing_target)
    //    focus_tab_id = tab_bar->selected_tab_id;

    // When clicked on a tab we requested focus to the docked child
    // This overrides the value set by "forward focus from host node to selected window".
    if (tab_bar.next_selected_tab_id)
        focus_tab_id = tab_bar.next_selected_tab_id;

    // Apply navigation focus
    if (focus_tab_id != 0)
        if (ImGuiTabItem* tab = TabBarFindTabByID(tab_bar, focus_tab_id))
            if (tab.Window)
            {
                focus_window(tab.Window);
                nav_init_window(tab.Window, false);
            }

    EndTabBar();
    PopID();

    // Restore skip_items flag
    if (!node.is_dock_space())
    {
        host_window.dcnav_layer_current = NavLayer::Main;
        host_window.skip_items = backup_skip_item;
    }
}

// static void DockNodeAddTabBar(ImGuiDockNode* node)
pub fn dock_node_add_tab_bar(g: &mut Context, node: &mut DockNode)
{
    // IM_ASSERT(node.TabBar == NULL);
    node.tab_bar = IM_NEW(ImGuiTabBar);
}

// static void dock_node_remove_tab_bar(ImGuiDockNode* node)
pub fn dock_node_remove_tab_bar(g: &mut Context, node: &mut DockNode)
{
    if (node.tab_bar == NULL)
        return;
    IM_DELETE(node.tab_bar);
    node.tab_bar = NULL;
}

// static bool DockNodeIsDropAllowedOne(ImGuiWindow* payload, ImGuiWindow* host_window)
pub fn dock_node_is_drop_allowed_one(g: &mut Context, payload: &mut window::Window, host_window: &mut window::Window) -> bool
{
    if (host_window.dock_node_as_host_id && host_window.dock_node_as_host_id.is_dock_space() && payload.BeginOrderWithinContext < host_window.BeginOrderWithinContext)
        return false;

    ImGuiWindowClass* host_class = host_window.dock_node_as_host_id? &host_window.dock_node_as_host_id.window_class : &host_window.window_class;
    ImGuiWindowClass* payload_class = &payload.window_class;
    if (host_class.ClassId != payload_class.ClassId)
    {
        if (host_class.ClassId != 0 && host_class.docking_allow_unclassed && payload_class.ClassId == 0)
            return true;
        if (payload_class.ClassId != 0 && payload_class.docking_allow_unclassed && host_class.ClassId == 0)
            return true;
        return false;
    }

    // Prevent docking any window created above a popup
    // Technically we should support it (e.g. in the case of a long-lived modal window that had fancy docking features),
    // by e.g. adding a 'if (!IsWindowWithinBeginStackOf(host_window, popup_window))' test.
    // But it would requires more work on our end because the dock host windows is technically created in NewFrame()
    // and our ->ParentXXX and ->RootXXX pointers inside windows are currently mislading or lacking.
    // ImGuiContext& g = *GImGui;
    for (int i = g.open_popup_stack.size - 1; i >= 0; i--)
        if (ImGuiWindow* popup_window = g.open_popup_stack[i].Window)
            if (is_window_within_begin_stack_of(payload, popup_window))   // Payload is created from within a popup begin stack.
                return false;

    return true;
}

// static bool DockNodeIsDropAllowed(ImGuiWindow* host_window, ImGuiWindow* root_payload)
pub fn dock_node_is_drop_allowed(g: &mut Context, host_window: &mut window::Window, root_payload: &mut window::Window) -> bool
{
    if (root_payload.dock_node_as_host_id && root_payload.dock_node_as_host_id.is_split_node()) // FIXME-DOCK: Missing filtering
        return true;

    const int payload_count = root_payload.dock_node_as_host_id? root_payload.dock_node_as_host_id.windows.len() : 1;
    for (int payload_n = 0; payload_n < payload_count; payload_n += 1)
    {
        ImGuiWindow* payload = root_payload.dock_node_as_host_id? root_payload.dock_node_as_host_id.windows[payload_n] : root_payload;
        if (DockNodeIsDropAllowedOne(payload, host_window))
            return true;
    }
    return false;
}

// window menu button == collapse button when not in a dock node.
// FIXME: This is similar to RenderWindowTitleBarContents(), may want to share code.
// static void DockNodeCalcTabBarLayout(const ImGuiDockNode* node, Rect* out_title_rect, Rect* out_tab_bar_rect, Vector2D* out_window_menu_button_pos, Vector2D* out_close_button_pos)
pub fn dock_node_calc_tab_bar_layout(g: &mut Context, node: &mut DockNode, out_title_rect: &mut Rect, out_tab_bar_rect: &mut Rect, out_window_menu_button_pos: &mut Vector2D, out_close_button_pos: &mut Vector2D)
{
    // ImGuiContext& g = *GImGui;
    ImGuiStyle& style = g.style;

    Rect r = Rect(node.pos.x, node.pos.y, node.pos.x + node.size.x, node.pos.y + g.font_size + g.style.frame_padding.y * 2.0);
    if (out_title_rect) { *out_title_rect = r; }

    r.min.x += style.WindowBorderSize;
    r.max.x -= style.WindowBorderSize;

    float button_sz = g.font_size;

    Vector2D window_menu_button_pos = r.min;
    r.min.x += style.frame_padding.x;
    r.max.x -= style.frame_padding.x;
    if (node.has_close_button)
    {
        r.max.x -= button_sz;
        if (out_close_button_pos) *out_close_button_pos = Vector2D::new(r.max.x - style.frame_padding.x, r.min.y);
    }
    if (node.HasWindowMenuButton && style.window_menu_button_position == Direction::Left)
    {
        r.min.x += button_sz + style.item_inner_spacing.x;
    }
    else if (node.HasWindowMenuButton && style.window_menu_button_position == Direction::Right)
    {
        r.max.x -= button_sz + style.frame_padding.x;
        window_menu_button_pos = Vector2D::new(r.max.x, r.min.y);
    }
    if (out_tab_bar_rect) { *out_tab_bar_rect = r; }
    if (out_window_menu_button_pos) { *out_window_menu_button_pos = window_menu_button_pos; }
}

// void DockNodeCalcSplitRects(Vector2D& pos_old, Vector2D& size_old, Vector2D& pos_new, Vector2D& size_new, ImGuiDir dir, Vector2D size_new_desired)
pub fn dock_node_calc_split_rects(g: &mut Context, pos_old: &mut Vector2D, size_old: &mut Vector2D, size_new: &mut Vector2D, dir: Direction, size_new_desired: Vector2D)
{
    // ImGuiContext& g = *GImGui;
    const float dock_spacing = g.style.item_inner_spacing.x;
    const ImGuiAxis axis = (dir == Direction::Left || dir == Direction::Right) ? Axis::X : Axis::Y;
    pos_new[axis ^ 1] = pos_old[axis ^ 1];
    size_new[axis ^ 1] = size_old[axis ^ 1];

    // Distribute size on given axis (with a desired size or equally)
    const float w_avail = size_old[axis] - dock_spacing;
    if (size_new_desired[axis] > 0.0 && size_new_desired[axis] <= w_avail * 0.5)
    {
        size_new[axis] = size_new_desired[axis];
        size_old[axis] = f32::floor(w_avail - size_new[axis]);
    }
    else
    {
        size_new[axis] = f32::floor(w_avail * 0.5);
        size_old[axis] = f32::floor(w_avail - size_new[axis]);
    }

    // Position each node
    if (dir == Direction::Right || dir == Direction::Down)
    {
        pos_new[axis] = pos_old[axis] + size_old[axis] + dock_spacing;
    }
    else if (dir == Direction::Left || dir == Direction::Up)
    {
        pos_new[axis] = pos_old[axis];
        pos_old[axis] = pos_new[axis] + size_new[axis] + dock_spacing;
    }
}

// Retrieve the drop rectangles for a given direction or for the center + perform hit testing.
// bool DockNodeCalcDropRectsAndTestMousePos(const Rect& parent, ImGuiDir dir, Rect& out_r, bool outer_docking, Vector2D* test_mouse_pos)
pub fn dock_node_calc_drop_rects_and_test_mouse_pos(g: &mut Context, parent: &Rect, dir: &mut Direction, out_r: &mut Rect, outer_docking: bool, test_mouse_pos: &mut Vector2D) -> bool
{
    // ImGuiContext& g = *GImGui;

    const float parent_smaller_axis = ImMin(parent.get_width(), parent.get_height());
    const float hs_for_central_nodes = ImMin(g.font_size * 1.5, ImMax(g.font_size * 0.5, parent_smaller_axis / 8.0));
    float hs_w; // Half-size, longer axis
    float hs_h; // Half-size, smaller axis
    Vector2D off; // Distance from edge or center
    if (outer_docking)
    {
        //hs_w = f32::floor(ImClamp(parent_smaller_axis - hs_for_central_nodes * 4.0, g.font_size * 0.5, g.font_size * 8.0));
        //hs_h = f32::floor(hs_w * 0.15);
        //off = Vector2D(f32::floor(parent.get_width() * 0.5 - GetFrameHeightWithSpacing() * 1.4 - hs_h), f32::floor(parent.get_height() * 0.5 - GetFrameHeightWithSpacing() * 1.4 - hs_h));
        hs_w = f32::floor(hs_for_central_nodes * 1.50);
        hs_h = f32::floor(hs_for_central_nodes * 0.80);
        off = Vector2D::new(f32::floor(parent.get_width() * 0.5 - hs_h), f32::floor(parent.get_height() * 0.5 - hs_h));
    }
    else
    {
        hs_w = f32::floor(hs_for_central_nodes);
        hs_h = f32::floor(hs_for_central_nodes * 0.90);
        off = Vector2D::new(f32::floor(hs_w * 2.40), f32::floor(hs_w * 2.40));
    }

    Vector2D c = f32::floor(parent.get_center());
    if      (dir == Direction::None)  { out_r = Rect(c.x - hs_w, c.y - hs_w,         c.x + hs_w, c.y + hs_w);         }
    else if (dir == Direction::Up)    { out_r = Rect(c.x - hs_w, c.y - off.y - hs_h, c.x + hs_w, c.y - off.y + hs_h); }
    else if (dir == Direction::Down)  { out_r = Rect(c.x - hs_w, c.y + off.y - hs_h, c.x + hs_w, c.y + off.y + hs_h); }
    else if (dir == Direction::Left)  { out_r = Rect(c.x - off.x - hs_h, c.y - hs_w, c.x - off.x + hs_h, c.y + hs_w); }
    else if (dir == Direction::Right) { out_r = Rect(c.x + off.x - hs_h, c.y - hs_w, c.x + off.x + hs_h, c.y + hs_w); }

    if (test_mouse_pos == NULL)
        return false;

    Rect hit_r = out_r;
    if (!outer_docking)
    {
        // Custom hit testing for the 5-way selection, designed to reduce flickering when moving diagonally between sides
        hit_r.Expand(f32::floor(hs_w * 0.30));
        Vector2D mouse_delta = (*test_mouse_pos - c);
        float mouse_delta_len2 = ImLengthSqr(mouse_delta);
        float r_threshold_center = hs_w * 1.4;
        float r_threshold_sides = hs_w * (1.4 + 1.2);
        if (mouse_delta_len2 < r_threshold_center * r_threshold_center)
            return (dir == Direction::None);
        if (mouse_delta_len2 < r_threshold_sides * r_threshold_sides)
            return (dir == ImGetDirQuadrantFromDelta(mouse_delta.x, mouse_delta.y));
    }
    return hit_r.Contains(*test_mouse_pos);
}

// host_node may be NULL if the window doesn't have a dock_node already.
// FIXME-DOCK: This is misnamed since it's also doing the filtering.
// static void dock_node_preview_dock_setup(ImGuiWindow* host_window, ImGuiDockNode* host_node, ImGuiWindow* root_payload, ImGuiDockPreviewData* data, bool is_explicit_target, bool is_outer_docking)
pub fn dock_node_preview_dock_setup(g: &mut Context, host_window: &mut window::Window, host_node: &mut DockNode, root_payload: &mut window::Window, data: &mut DockPreviewData, is_explicit_target: bool, is_outer_docking: bool)
{
    // ImGuiContext& g = *GImGui;

    // There is an edge case when docking into a dockspace which only has inactive nodes.
    // In this case DockNodeTreeFindNodeByPos() will have selected a leaf node which is inactive.
    // Because the inactive leaf node doesn't have proper pos/size yet, we'll use the root node as reference.
    ImGuiDockNode* root_payload_as_host = root_payload.dock_node_as_host_id;
    ImGuiDockNode* ref_node_for_rect = (host_node && !host_node.is_visible) ? dock_node_get_root_node(host_node) : host_node;
    if (ref_node_for_rect)
        // IM_ASSERT(ref_node_for_rect.is_visible == true);

    // Filter, figure out where we are allowed to dock
    ImGuiDockNodeFlags src_node_flags = root_payload_as_host ? root_payload_as_host.MergedFlags : root_payload.window_class.dock_node_flags_override_set;
    ImGuiDockNodeFlags dst_node_flags = host_node ? host_node.MergedFlags : host_window.window_class.dock_node_flags_override_set;
    data.IsCenterAvailable = true;
    if (is_outer_docking)
        data.IsCenterAvailable = false;
    else if (dst_node_flags & DockNodeFlags::NoDocking)
        data.IsCenterAvailable = false;
    else if (host_node && (dst_node_flags & DockNodeFlags::NoDockingInCentralNode) && host_node.is_central_node())
        data.IsCenterAvailable = false;
    else if ((!host_node || !host_node.IsEmpty()) && root_payload_as_host && root_payload_as_host.is_split_node() && (root_payload_as_host.only_node_with_windows == NULL)) // Is _visibly_ split?
        data.IsCenterAvailable = false;
    else if (dst_node_flags & DockNodeFlags::NoDockingOverMe)
        data.IsCenterAvailable = false;
    else if ((src_node_flags & DockNodeFlags::NoDockingOverOther) && (!host_node || !host_node.IsEmpty()))
        data.IsCenterAvailable = false;
    else if ((src_node_flags & DockNodeFlags::NoDockingOverEmpty) && host_node && host_node.IsEmpty())
        data.IsCenterAvailable = false;

    data.IsSidesAvailable = true;
    if ((dst_node_flags & DockNodeFlags::NoSplit) || g.io.ConfigDockingNoSplit)
        data.IsSidesAvailable = false;
    else if (!is_outer_docking && host_node && host_node.parent_node == NULL && host_node.is_central_node())
        data.IsSidesAvailable = false;
    else if ((dst_node_flags & DockNodeFlags::NoDockingSplitMe) || (src_node_flags & DockNodeFlags::NoDockingSplitOther))
        data.IsSidesAvailable = false;

    // build a tentative future node (reuse same structure because it is practical. Shape will be readjusted when previewing a split)
    data.FutureNode.has_close_button = (host_node ? host_node.has_close_button : host_window.has_close_button) || (root_payload.has_close_button);
    data.FutureNode.HasWindowMenuButton = host_node ? true : ((host_window.flags & WindowFlags::NoCollapse) == 0);
    data.FutureNode.pos = ref_node_for_rect ? ref_node_for_rect.pos : host_window.pos;
    data.FutureNode.size = ref_node_for_rect ? ref_node_for_rect.size : host_window.size;

    // Calculate drop shapes geometry for allowed splitting directions
    // IM_ASSERT(Dir::None == -1);
    data.SplitNode = host_node;
    data.SplitDir = Direction::None;
    data.IsSplitDirExplicit = false;
    if (!host_window.collapsed)
        for (int dir = Direction::None; dir < Direction::COUNT; dir += 1)
        {
            if (dir == Direction::None && !data.IsCenterAvailable)
                continue;
            if (dir != Direction::None && !data.IsSidesAvailable)
                continue;
            if (DockNodeCalcDropRectsAndTestMousePos(data.FutureNode.Rect(), (ImGuiDir)dir, data.drop_rects_draw[dir+1], is_outer_docking, &g.io.mouse_pos))
            {
                data.SplitDir = (ImGuiDir)dir;
                data.IsSplitDirExplicit = true;
            }
        }

    // When docking without holding Shift, we only allow and preview docking when hovering over a drop rect or over the title bar
    data.IsDropAllowed = (data.SplitDir != Direction::None) || (data.IsCenterAvailable);
    if (!is_explicit_target && !data.IsSplitDirExplicit && !g.io.ConfigDockingWithShift)
        data.IsDropAllowed = false;

    // Calculate split area
    data.SplitRatio = 0.0;
    if (data.SplitDir != Direction::None)
    {
        ImGuiDir split_dir = data.SplitDir;
        ImGuiAxis split_axis = (split_dir == Direction::Left || split_dir == Direction::Right) ? Axis::X : Axis::Y;
        Vector2D pos_new, pos_old = data.FutureNode.pos;
        Vector2D size_new, size_old = data.FutureNode.size;
        DockNodeCalcSplitRects(pos_old, size_old, pos_new, size_new, split_dir, root_payload.size);

        // Calculate split ratio so we can pass it down the docking request
        float split_ratio = ImSaturate(size_new[split_axis] / data.FutureNode.size[split_axis]);
        data.FutureNode.pos = pos_new;
        data.FutureNode.size = size_new;
        data.SplitRatio = (split_dir == Direction::Right || split_dir == Direction::Down) ? (1.0 - split_ratio) : (split_ratio);
    }
}

// static void DockNodePreviewDockRender(ImGuiWindow* host_window, ImGuiDockNode* host_node, ImGuiWindow* root_payload, const ImGuiDockPreviewData* data)
pub fn dock_node_preview_dock_render(g: &mut Context, host_window: &mut host_window, host_node: &mut DockNode, root_payload: &mut window::Window, data: &mut DockPreviewData)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(g.current_window == host_window);   // Because we rely on font size to calculate tab sizes

    // With this option, we only display the preview on the target viewport, and the payload viewport is made transparent.
    // To compensate for the single layer obstructed by the payload, we'll increase the alpha of the preview nodes.
    const bool is_transparent_payload = g.io.config_docking_transparent_payload;

    // In case the two windows involved are on different viewports, we will draw the overlay on each of them.
    int overlay_draw_lists_count = 0;
    ImDrawList* overlay_draw_lists[2];
    overlay_draw_lists[overlay_draw_lists_count += 1] = get_foreground_draw_list(host_window.viewport);
    if (host_window.viewport != root_payload.Viewport && !is_transparent_payload)
        overlay_draw_lists[overlay_draw_lists_count += 1] = get_foreground_draw_list(root_payload.Viewport);

    // Draw main preview rectangle
    const ImU32 overlay_col_main = get_color_u32(StyleColor::DockingPreview, is_transparent_payload ? 0.60 : 0.40);
    const ImU32 overlay_col_drop = get_color_u32(StyleColor::DockingPreview, is_transparent_payload ? 0.90 : 0.70);
    const ImU32 overlay_col_drop_hovered = get_color_u32(StyleColor::DockingPreview, is_transparent_payload ? 1.20 : 1.00);
    const ImU32 overlay_col_lines = get_color_u32(StyleColor::NavWindowingHighlight, is_transparent_payload ? 0.80 : 0.60);

    // Display area preview
    const bool can_preview_tabs = (root_payload.dock_node_as_host_id == NULL || root_payload.dock_node_as_host_id.windows.len() > 0);
    if (data.IsDropAllowed)
    {
        Rect overlay_rect = data.FutureNode.Rect();
        if (data.SplitDir == Direction::None && can_preview_tabs)
            overlay_rect.min.y += get_frame_height();
        if (data.SplitDir != Direction::None || data.IsCenterAvailable)
            for (int overlay_n = 0; overlay_n < overlay_draw_lists_count; overlay_n += 1)
                overlay_draw_lists[overlay_n].add_rect_filled(overlay_rect.min, overlay_rect.max, overlay_col_main, host_window.WindowRounding, calc_rounding_flags_for_rect_in_rect(overlay_rect, host_window.Rect(), DOCKING_SPLITTER_SIZE));
    }

    // Display tab shape/label preview unless we are splitting node (it generally makes the situation harder to read)
    if (data.IsDropAllowed && can_preview_tabs && data.SplitDir == Direction::None && data.IsCenterAvailable)
    {
        // Compute target tab bar geometry so we can locate our preview tabs
        Rect tab_bar_rect;
        DockNodeCalcTabBarLayout(&data.FutureNode, NULL, &tab_bar_rect, NULL, NULL);
        Vector2D tab_pos = tab_bar_rect.min;
        if (host_node && host_node.tab_bar)
        {
            if (!host_node.is_hidden_tab_bar() && !host_node.is_no_tab_bar())
                tab_pos.x += host_node.tab_bar.WidthAllTabs + g.style.item_inner_spacing.x; // We don't use OffsetNewTab because when using non-persistent-order tab bar it is incremented with each Tab submission.
            else
                tab_pos.x += g.style.item_inner_spacing.x + TabItemCalcSize(host_node.windows[0].Name, host_node.windows[0].has_close_button).x;
        }
        else if (!(host_window.flags & WindowFlags::DockNodeHost))
        {
            tab_pos.x += g.style.item_inner_spacing.x + TabItemCalcSize(host_window.Name, host_window.has_close_button).x; // Account for slight offset which will be added when changing from title bar to tab bar
        }

        // Draw tab shape/label preview (payload may be a loose window or a host window carrying multiple tabbed windows)
        if (root_payload.dock_node_as_host_id)
            // IM_ASSERT(root_payload.DockNodeAsHost.Windows.size <= root_payload.DockNodeAsHost.TabBar.Tabs.size);
        ImGuiTabBar* tab_bar_with_payload = root_payload.dock_node_as_host_id? root_payload.dock_node_as_host_id.tab_bar : NULL;
        const int payload_count = tab_bar_with_payload ? tab_bar_with_payload.tabs.size : 1;
        for (int payload_n = 0; payload_n < payload_count; payload_n += 1)
        {
            // dock_node's tab_bar may have non-window Tabs manually appended by user
            ImGuiWindow* payload_window = tab_bar_with_payload ? tab_bar_with_payload.tabs[payload_n].Window : root_payload;
            if (tab_bar_with_payload && payload_window == NULL)
                continue;
            if (!DockNodeIsDropAllowedOne(payload_window, host_window))
                continue;

            // Calculate the tab bounding box for each payload window
            Vector2D tab_size = TabItemCalcSize(payload_window.Name, payload_window.has_close_button);
            Rect tab_bb(tab_pos.x, tab_pos.y, tab_pos.x + tab_size.x, tab_pos.y + tab_size.y);
            tab_pos.x += tab_size.x + g.style.item_inner_spacing.x;
            const ImU32 overlay_col_text = get_color_u32(payload_window.DockStyle.colors[ImGuiWindowDockStyleCol_Text]);
            const ImU32 overlay_col_tabs = get_color_u32(payload_window.DockStyle.colors[ImGuiWindowDockStyleCol_TabActive]);
            push_style_color(, StyleColor::Text, overlay_col_text);
            for (int overlay_n = 0; overlay_n < overlay_draw_lists_count; overlay_n += 1)
            {
                ImGuiTabItemFlags tab_flags = TabItemFlags::Preview | ((payload_window.flags & WindowFlags::UnsavedDocument) ? TabItemFlags::UnsavedDocument : 0);
                if (!tab_bar_rect.Contains(tab_bb))
                    overlay_draw_lists[overlay_n].push_clip_rect(tab_bar_rect.min, tab_bar_rect.max);
                TabItemBackground(overlay_draw_lists[overlay_n], tab_bb, tab_flags, overlay_col_tabs);
                TabItemLabelAndCloseButton(overlay_draw_lists[overlay_n], tab_bb, tab_flags, g.style.frame_padding, payload_window.Name, 0, 0, false, NULL, NULL);
                if (!tab_bar_rect.Contains(tab_bb))
                    overlay_draw_lists[overlay_n].pop_clip_rect();
            }
            pop_style_color();
        }
    }

    // Display drop boxes
    const float overlay_rounding = ImMax(3.0, g.style.FrameRounding);
    for (int dir = Direction::None; dir < Direction::COUNT; dir += 1)
    {
        if (!data.drop_rects_draw[dir + 1].is_inverted())
        {
            Rect draw_r = data.drop_rects_draw[dir + 1];
            Rect draw_r_in = draw_r;
            draw_r_in.Expand(-2.0);
            ImU32 overlay_col = (data.SplitDir == (ImGuiDir)dir && data.IsSplitDirExplicit) ? overlay_col_drop_hovered : overlay_col_drop;
            for (int overlay_n = 0; overlay_n < overlay_draw_lists_count; overlay_n += 1)
            {
                Vector2D center = f32::floor(draw_r_in.get_center());
                overlay_draw_lists[overlay_n].add_rect_filled(draw_r.min, draw_r.max, overlay_col, overlay_rounding);
                overlay_draw_lists[overlay_n].AddRect(draw_r_in.min, draw_r_in.max, overlay_col_lines, overlay_rounding);
                if (dir == Direction::Left || dir == Direction::Right)
                    overlay_draw_lists[overlay_n].add_line(Vector2D::new(center.x, draw_r_in.min.y), Vector2D::new(center.x, draw_r_in.max.y), overlay_col_lines);
                if (dir == Direction::Up || dir == Direction::Down)
                    overlay_draw_lists[overlay_n].add_line(Vector2D::new(draw_r_in.min.x, center.y), Vector2D::new(draw_r_in.max.x, center.y), overlay_col_lines);
            }
        }

        // Stop after ImGuiDir_None
        if ((host_node && (host_node.MergedFlags & DockNodeFlags::NoSplit)) || g.io.ConfigDockingNoSplit)
            return;
    }
}

// void DockNodeTreeSplit(ImGuiContext* ctx, ImGuiDockNode* parent_node, ImGuiAxis split_axis, int split_inheritor_child_idx, float split_ratio, ImGuiDockNode* new_node)
pub fn dock_node_tree_split(g: &mut Context, parent_node: &mut DockNode, split_axis: Axis, split_inheritor_child_idx: i32, split_ratio: f32, new_node: &mut DockNode)
{
    // ImGuiContext& g = *GImGui;
    // IM_ASSERT(split_axis != ImGuiAxis_None);

    ImGuiDockNode* child_0 = (new_node && split_inheritor_child_idx != 0) ? new_node : dock_context_add_node(g, 0);
    child_0parent_node = parent_node;

    ImGuiDockNode* child_1 = (new_node && split_inheritor_child_idx != 1) ? new_node : dock_context_add_node(g, 0);
    child_1parent_node = parent_node;

    ImGuiDockNode* child_inheritor = (split_inheritor_child_idx == 0) ? child_0 : child_1;
    dock_node_move_child_nodes(child_inheritor, parent_node);
    parent_node.child_nodes[0] = child_0;
    parent_node.child_nodes[1] = child_1;
    parent_node.child_nodes[split_inheritor_child_idx].visible_window = parent_node.visible_window_id;
    parent_node.split_axis = split_axis;
    parent_node.visible_window_id = NULL;
    parent_node.authority_for_pos = parent_node.authority_for_size = DataAuthority::DockNode;

    float size_avail = (parent_node.size[split_axis] - DOCKING_SPLITTER_SIZE);
    size_avail = ImMax(size_avail, g.style.window_min_size[split_axis] * 2.0);
    // IM_ASSERT(size_avail > 0.0); // If you created a node manually with DockBuilderAddNode(), you need to also call DockBuilderSetNodeSize() before splitting.
    child_0.size_ref = child_1.size_ref = parent_node.size;
    child_0.size_ref[split_axis] = f32::floor(size_avail * split_ratio);
    child_1.size_ref[split_axis] = f32::floor(size_avail - child_0.size_ref[split_axis]);

    dock_node_move_windows(parent_node.child_nodes[split_inheritor_child_idx], parent_node);
    settings::dock_settings_rename_node_references(parent_node.id, parent_node.child_nodes[split_inheritor_child_idx].id);
    dock_node_update_has_central_node_child(dock_node_get_root_node(parent_node));
    DockNodeTreeUpdatePosSize(parent_node, parent_node.pos, parent_node.size);

    // flags transfer (e.g. this is where we transfer the ImGuiDockNodeFlags_CentralNode property)
    child_0.shared_flags = parent_node.shared_flags & DockNodeFlags::SharedFlagsInheritMask_;
    child_1.shared_flags = parent_node.shared_flags & DockNodeFlags::SharedFlagsInheritMask_;
    child_inheritor.local_flags = parent_node.local_flags & DockNodeFlags::LocalFlagsTransferMask_;
    parent_node.local_flags &= ~DockNodeFlags::LocalFlagsTransferMask_;
    child_0.update_merged_flags();
    child_1.update_merged_flags();
    parent_node.update_merged_flags();
    if (child_inheritor.is_central_node())
        dock_node_get_root_node(parent_node).central_node_id = child_inheritor;
}

// void DockNodeTreeMerge(ImGuiContext* ctx, ImGuiDockNode* parent_node, ImGuiDockNode* merge_lead_child)
pub fn dock_node_tree_merge(g: &mut Context, parent_node: &mut DockNode, merge_lead_child: Option<&mut DockNode>)
{
    // When called from DockContextProcessUndockNode() it is possible that one of the child is NULL.
    // ImGuiContext& g = *GImGui;
    ImGuiDockNode* child_0 = parent_node.child_nodes[0];
    ImGuiDockNode* child_1 = parent_node.child_nodes[1];
    // IM_ASSERT(child_0 || child_1);
    // IM_ASSERT(merge_lead_child == child_0 || merge_lead_child == child_1);
    if ((child_0 && child_0.windows.len() > 0) || (child_1 && child_1.windows.len() > 0))
    {
        // IM_ASSERT(parent_node.TabBar == NULL);
        // IM_ASSERT(parent_node.Windows.size == 0);
    }
    // IMGUI_DEBUG_LOG_DOCKING("[docking] DockNodeTreeMerge: 0x%08X + 0x%08X back into parent 0x%08X\n", child_0 ? child_0.ID : 0, child_1 ? child_1.ID : 0, parent_node.ID);

    Vector2D backup_last_explicit_size = parent_node.size_ref;
    dock_node_move_child_nodes(parent_node, merge_lead_child);
    if (child_0)
    {
        dock_node_move_windows(parent_node, child_0); // Generally only 1 of the 2 child node will have windows
        settings::dock_settings_rename_node_references(child_0.id, parent_node.id);
    }
    if (child_1)
    {
        dock_node_move_windows(parent_node, child_1);
        settings::dock_settings_rename_node_references(child_1.id, parent_node.id);
    }
    DockNodeApplyPosSizeToWindows(parent_node);
    parent_node.authority_for_pos = parent_node.authority_for_size = parent_node.authority_for_viewport = DataAuthority::Auto;
    parent_node.visible_window_id = merge_lead_child.visible_window;
    parent_node.size_ref = backup_last_explicit_size;

    // flags transfer
    parent_node.local_flags &= ~DockNodeFlags::LocalFlagsTransferMask_; // Preserve Dockspace flag
    parent_node.local_flags |= (child_0 ? child_0.local_flags : 0) & DockNodeFlags::LocalFlagsTransferMask_;
    parent_node.local_flags |= (child_1 ? child_1.local_flags : 0) & DockNodeFlags::LocalFlagsTransferMask_;
    parent_node.local_flags_in_windows = (child_0 ? child_0.local_flags_in_windows : 0) | (child_1 ? child_1.local_flags_in_windows : 0); // FIXME: Would be more consistent to update from actual windows
    parent_node.update_merged_flags();

    if (child_0)
    {
        g.dock_context.Nodes.SetVoidPtr(child_0.id, NULL);
        IM_DELETE(child_0);
    }
    if (child_1)
    {
        g.dock_context.Nodes.SetVoidPtr(child_1.id, NULL);
        IM_DELETE(child_1);
    }
}

// Update pos/size for a node hierarchy (don't affect child windows yet)
// (Depth-first, Pre-Order)
// void DockNodeTreeUpdatePosSize(ImGuiDockNode* node, Vector2D pos, Vector2D size, ImGuiDockNode* only_write_to_single_node)
pub fn dock_node_tree_update_pos_size(g: &mut Context, node: &mut DockNode, pos: Vector2D, size: Vector2D, only_write_to_single_node: &mut DockNode)
{
    // During the regular dock node update we write to all nodes.
    // 'only_write_to_single_node' is only set when turning a node visible mid-frame and we need its size right-away.
    const bool write_to_node = only_write_to_single_node == NULL || only_write_to_single_node == node;
    if (write_to_node)
    {
        node.pos = pos;
        node.size = size;
    }

    if (node.is_leaf_node())
        return;

    ImGuiDockNode* child_0 = node.child_nodes[0];
    ImGuiDockNode* child_1 = node.child_nodes[1];
    Vector2D child_0_pos = pos, child_1_pos = pos;
    Vector2D child_0_size = size, child_1_size = size;

    const bool child_0_is_toward_single_node = (only_write_to_single_node != NULL && DockNodeIsInHierarchyOf(only_write_to_single_node, child_0));
    const bool child_1_is_toward_single_node = (only_write_to_single_node != NULL && DockNodeIsInHierarchyOf(only_write_to_single_node, child_1));
    const bool child_0_is_or_will_be_visible = child_0.is_visible || child_0_is_toward_single_node;
    const bool child_1_is_or_will_be_visible = child_1.is_visible || child_1_is_toward_single_node;

    if (child_0_is_or_will_be_visible && child_1_is_or_will_be_visible)
    {
        // ImGuiContext& g = *GImGui;
        const float spacing = DOCKING_SPLITTER_SIZE;
        const ImGuiAxis axis = node.split_axis;
        const float size_avail = ImMax(size[axis] - spacing, 0.0);

        // size allocation policy
        // 1) The first 0..WindowMinSize[axis]*2 are allocated evenly to both windows.
        const float size_min_each = f32::floor(ImMin(size_avail, g.style.window_min_size[axis] * 2.0) * 0.5);

        // FIXME: Blocks 2) and 3) are essentially doing nearly the same thing.
        // Difference are: write-back to size_ref; application of a minimum size; rounding before f32::floor()
        // Clarify and rework differences between size & size_ref and purpose of WantLockSizeOnce

        // 2) Process locked absolute size (during a splitter resize we preserve the child of nodes not touching the splitter edge)
        if (child_0.WantLockSizeOnce && !child_1.WantLockSizeOnce)
        {
            child_0_size[axis] = child_0.size_ref[axis] = ImMin(size_avail - 1.0, child_0.size[axis]);
            child_1_size[axis] = child_1.size_ref[axis] = (size_avail - child_0_size[axis]);
            // IM_ASSERT(child_0.sizeRef[axis] > 0.0 && child_1.sizeRef[axis] > 0.0);
        }
        else if (child_1.WantLockSizeOnce && !child_0.WantLockSizeOnce)
        {
            child_1_size[axis] = child_1.size_ref[axis] = ImMin(size_avail - 1.0, child_1.size[axis]);
            child_0_size[axis] = child_0.size_ref[axis] = (size_avail - child_1_size[axis]);
            // IM_ASSERT(child_0.sizeRef[axis] > 0.0 && child_1.sizeRef[axis] > 0.0);
        }
        else if (child_0.WantLockSizeOnce && child_1.WantLockSizeOnce)
        {
            // FIXME-DOCK: We cannot honor the requested size, so apply ratio.
            // Currently this path will only be taken if code programmatically sets WantLockSizeOnce
            float split_ratio = child_0_size[axis] / (child_0_size[axis] + child_1_size[axis]);
            child_0_size[axis] = child_0.size_ref[axis] = f32::floor(size_avail * split_ratio);
            child_1_size[axis] = child_1.size_ref[axis] = (size_avail - child_0_size[axis]);
            // IM_ASSERT(child_0.sizeRef[axis] > 0.0 && child_1.sizeRef[axis] > 0.0);
        }

        // 3) If one window is the central node (~ use remaining space, should be made explicit!), use explicit size from the other, and remainder for the central node
        else if (child_0.size_ref[axis] != 0.0 && child_1.has_central_node_child)
        {
            child_0_size[axis] = ImMin(size_avail - size_min_each, child_0.size_ref[axis]);
            child_1_size[axis] = (size_avail - child_0_size[axis]);
        }
        else if (child_1.size_ref[axis] != 0.0 && child_0.has_central_node_child)
        {
            child_1_size[axis] = ImMin(size_avail - size_min_each, child_1.size_ref[axis]);
            child_0_size[axis] = (size_avail - child_1_size[axis]);
        }
        else
        {
            // 4) Otherwise distribute according to the relative ratio of each size_ref value
            float split_ratio = child_0.size_ref[axis] / (child_0.size_ref[axis] + child_1.size_ref[axis]);
            child_0_size[axis] = ImMax(size_min_each, f32::floor(size_avail * split_ratio + 0.5));
            child_1_size[axis] = (size_avail - child_0_size[axis]);
        }

        child_1_pos[axis] += spacing + child_0_size[axis];
    }

    if (only_write_to_single_node == NULL)
        child_0.WantLockSizeOnce = child_1.WantLockSizeOnce = false;

    const bool child_0_recurse = only_write_to_single_node ? child_0_is_toward_single_node : child_0.is_visible;
    const bool child_1_recurse = only_write_to_single_node ? child_1_is_toward_single_node : child_1.is_visible;
    if (child_0_recurse)
        DockNodeTreeUpdatePosSize(child_0, child_0_pos, child_0_size);
    if (child_1_recurse)
        DockNodeTreeUpdatePosSize(child_1, child_1_pos, child_1_size);
}

// static void DockNodeTreeUpdateSplitterFindTouchingNode(ImGuiDockNode* node, ImGuiAxis axis, int side, ImVector<ImGuiDockNode*>* touching_nodes)
pub fn dock_node_tree_update_splitter_find_touching_node(g: &mut Context, node: &mut DockNode, axis: Axis, side: i32, touching_nodes: &mut Vec<Id32>)
{
    if (node.is_leaf_node())
    {
        touching_nodes.push_back(node);
        return;
    }
    if (node.child_nodes[0].is_visible)
        if (node.split_axis != axis || side == 0 || !node.child_nodes[1].is_visible)
            DockNodeTreeUpdateSplitterFindTouchingNode(node.child_nodes[0], axis, side, touching_nodes);
    if (node.child_nodes[1].is_visible)
        if (node.split_axis != axis || side == 1 || !node.child_nodes[0].is_visible)
            DockNodeTreeUpdateSplitterFindTouchingNode(node.child_nodes[1], axis, side, touching_nodes);
}

// (Depth-First, Pre-Order)
// void DockNodeTreeUpdateSplitter(ImGuiDockNode* node)
pub fn dock_node_tree_update_splitter(g: &mut Context, node: &mut DockNode)
{
    if (node.is_leaf_node())
        return;

    // ImGuiContext& g = *GImGui;

    ImGuiDockNode* child_0 = node.child_nodes[0];
    ImGuiDockNode* child_1 = node.child_nodes[1];
    if (child_0.is_visible && child_1.is_visible)
    {
        // Bounding box of the splitter cover the space between both nodes (w = Spacing, h = size[xy^1] for when splitting horizontally)
        const ImGuiAxis axis = node.split_axis;
        // IM_ASSERT(axis != ImGuiAxis_None);
        Rect bb;
        bb.min = child_0.pos;
        bb.max = child_1.pos;
        bb.min[axis] += child_0.size[axis];
        bb.max[axis ^ 1] += child_1.size[axis ^ 1];
        //if (g.io.key_ctrl) GetForegroundDrawList(g.current_window->viewport)->add_rect(bb.min, bb.max, IM_COL32(255,0,255,255));

        const ImGuiDockNodeFlags merged_flags = child_0.MergedFlags | child_1.MergedFlags; // Merged flags for BOTH childs
        const ImGuiDockNodeFlags no_resize_axis_flag = (axis == Axis::X) ? DockNodeFlags::NoResizeX : DockNodeFlags::NoResizeY;
        if ((merged_flags & DockNodeFlags::NoResize) || (merged_flags & no_resize_axis_flag))
        {
            ImGuiWindow* window = g.current_window;
            window.draw_list.add_rect_filled(bb.min, bb.max, get_color_u32(StyleColor::Separator), g.style.FrameRounding);
        }
        else
        {
            //bb.min[axis] += 1; // Display a little inward so highlight doesn't connect with nearby tabs on the neighbor node.
            //bb.max[axis] -= 1;
            PushID(node.id);

            // Find resizing limits by gathering list of nodes that are touching the splitter line.
            ImVector<ImGuiDockNode*> touching_nodes[2];
            float min_size = g.style.window_min_size[axis];
            float resize_limits[2];
            resize_limits[0] = node.child_nodes[0].pos[axis] + min_size;
            resize_limits[1] = node.child_nodes[1].pos[axis] + node.child_nodes[1].size[axis] - min_size;

            ImGuiID splitter_id = GetID("##splitter");
            if (g.active_id == splitter_id) // Only process when splitter is active
            {
                DockNodeTreeUpdateSplitterFindTouchingNode(child_0, axis, 1, &touching_nodes[0]);
                DockNodeTreeUpdateSplitterFindTouchingNode(child_1, axis, 0, &touching_nodes[1]);
                for (int touching_node_n = 0; touching_node_n < touching_nodes[0].size; touching_node_n += 1)
                    resize_limits[0] = ImMax(resize_limits[0], touching_nodes[0][touching_node_n].rect().min[axis] + min_size);
                for (int touching_node_n = 0; touching_node_n < touching_nodes[1].size; touching_node_n += 1)
                    resize_limits[1] = ImMin(resize_limits[1], touching_nodes[1][touching_node_n].rect().max[axis] - min_size);

                // [DEBUG] Render touching nodes & limits
                /*
                ImDrawList* draw_list = node->host_window ? GetForegroundDrawList(node->host_window) : GetForegroundDrawList(GetMainViewport());
                for (int n = 0; n < 2; n++)
                {
                    for (int touching_node_n = 0; touching_node_n < touching_nodes[n].size; touching_node_n++)
                        draw_list->add_rect(touching_nodes[n][touching_node_n]->pos, touching_nodes[n][touching_node_n]->pos + touching_nodes[n][touching_node_n]->size, IM_COL32(0, 255, 0, 255));
                    if (axis == ImGuiAxis_X)
                        draw_list->add_line(Vector2D(resize_limits[n], node->child_nodes[n]->pos.y), Vector2D(resize_limits[n], node->child_nodes[n]->pos.y + node->child_nodes[n]->size.y), IM_COL32(255, 0, 255, 255), 3.0);
                    else
                        draw_list->add_line(Vector2D(node->child_nodes[n]->pos.x, resize_limits[n]), Vector2D(node->child_nodes[n]->pos.x + node->child_nodes[n]->size.x, resize_limits[n]), IM_COL32(255, 0, 255, 255), 3.0);
                }
                */
            }

            // Use a short delay before highlighting the splitter (and changing the mouse cursor) in order for regular mouse movement to not highlight many splitters
            float cur_size_0 = child_0.size[axis];
            float cur_size_1 = child_1.size[axis];
            float min_size_0 = resize_limits[0] - child_0.pos[axis];
            float min_size_1 = child_1.pos[axis] + child_1.size[axis] - resize_limits[1];
            ImU32 bg_col = get_color_u32(StyleColor::WindowBg);
            if (SplitterBehavior(bb, GetID("##splitter"), axis, &cur_size_0, &cur_size_1, min_size_0, min_size_1, WINDOWS_HOVER_PADDING, WINDOWS_RESIZE_FROM_EDGES_FEEDBACK_TIMER, bg_col))
            {
                if (touching_nodes[0].size > 0 && touching_nodes[1].size > 0)
                {
                    child_0.size[axis] = child_0.size_ref[axis] = cur_size_0;
                    child_1.pos[axis] -= cur_size_1 - child_1.size[axis];
                    child_1.size[axis] = child_1.size_ref[axis] = cur_size_1;

                    // Lock the size of every node that is a sibling of the node we are touching
                    // This might be less desirable if we can merge sibling of a same axis into the same parental level.
                    for (int side_n = 0; side_n < 2; side_n += 1)
                        for (int touching_node_n = 0; touching_node_n < touching_nodes[side_n].size; touching_node_n += 1)
                        {
                            ImGuiDockNode* touching_node = touching_nodes[side_n][touching_node_n];
                            //ImDrawList* draw_list = node->host_window ? GetForegroundDrawList(node->host_window) : GetForegroundDrawList(GetMainViewport());
                            //draw_list->add_rect(touching_node->pos, touching_node->pos + touching_node->size, IM_COL32(255, 128, 0, 255));
                            while (touching_node.parent_node != node)
                            {
                                if (touching_node.parent_node.split_axis == axis)
                                {
                                    // Mark other node so its size will be preserved during the upcoming call to DockNodeTreeUpdatePosSize().
                                    ImGuiDockNode* node_to_preserve = touching_node.parent_node.child_nodes[side_n];
                                    node_to_preserve.WantLockSizeOnce = true;
                                    //draw_list->add_rect(touching_node->pos, touching_node->rect().max, IM_COL32(255, 0, 0, 255));
                                    //draw_list->add_rect_filled(node_to_preserve->pos, node_to_preserve->rect().max, IM_COL32(0, 255, 0, 100));
                                }
                                touching_node = touching_node.parent_node;
                            }
                        }

                    DockNodeTreeUpdatePosSize(child_0, child_0.pos, child_0.size);
                    DockNodeTreeUpdatePosSize(child_1, child_1.pos, child_1.size);
                    mark_ini_settings_dirty();
                }
            }
            PopID();
        }
    }

    if (child_0.is_visible)
        DockNodeTreeUpdateSplitter(child_0);
    if (child_1.is_visible)
        DockNodeTreeUpdateSplitter(child_1);
}
