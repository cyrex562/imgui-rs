use std::collections::HashSet;
use crate::axis::Axis;
use crate::dock::node::dock_node_state::DockNodeState;
use crate::dock::node::dock_node_flags::DockNodeFlags;
use crate::INVALID_ID;
use crate::rect::Rect;
use crate::tab_bar::TabBar;
use crate::types::{DataAuthority, Id32};
use crate::utils::extend_hash_set;
use crate::vectors::Vector2D;
use crate::window::class::WindowClass;

#[derive(Default, Debug, Clone)]
pub struct DockNode {
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
    // pub parent_node: &'a mut DockNode,
    // ImGuiDockNode*          child_nodes[2];              // [split node only] Child nodes (left/right or top/bottom). Consider switching to an array.
    pub child_nodes: Vec<Id32>, //[*mut ImGuiDockNode;2],
    // ImVector<Window*>  windows;                    // Note: unordered list! Iterate tab_bar->Tabs for user-order.
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
    // window_class        window_class;                // [Root node only]
    pub window_class: WindowClass,
    // ImU32                   last_bg_color;
    pub last_bg_color: u32,
    // Window*            host_window;
    pub host_window_id: Id32, //*mut Window,
    // Window*            visible_window;              // Generally point to window which is id is == SelectedTabID, but when CTRL+Tabbing this can be a different window.
    pub visible_window_id: Id32, //*mut Window,
    // ImGuiDockNode*          central_node;                // [Root node only] Pointer to central node.
    pub central_node_id: Id32, // *mut ImGuiDockNode,
    // ImGuiDockNode*          only_node_with_windows;        // [Root node only] Set when there is a single visible node within the hierarchy.
    pub only_node_with_window_id: Id32, // *mut ImGuiDockNode,
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
    // bool                    want_lock_size_once        :1;
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
    // ImGuiDockNode::ImGuiDockNode(Id32 id)
    // {
    //     id = id;
    //     SharedFlags = LocalFlags = LocalFlagsInWindows = MergedFlags = ImGuiDockNodeFlags_None;
    //     ParentNode = ChildNodes[0] = ChildNodes[1] = None;
    //     TabBar = None;
    //     SplitAxis = ImGuiAxis_None;
    //
    //     State = DockNodeState::Unknown;
    //     LastBgColor = IM_COL32_WHITE;
    //     HostWindow = VisibleWindow = None;
    //     CentralNode = only_node_with_windows = None;
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
    //     WantCloseAll = want_lock_size_once = WantMouseMove = WantHiddenTabBarUpdate = WantHiddenTabBarToggle = false;
    // }
    //
    // ImGuiDockNode::~ImGuiDockNode()
    // {
    //     IM_DELETE(TabBar);
    //     TabBar = None;
    //     ChildNodes[0] = ChildNodes[1] = None;
    // }

    // ImGuiDockNode(DimgId id);
    pub fn new(id: Id32) -> Self {
        todo!()
    }
    //     ~ImGuiDockNode();
    //     bool                    is_root_node() const      { return parent_node == None; }
    pub fn is_root_node(&self) -> bool {
        self.parent_node_id > 0 && self.parent_node_id < Id32::MAX
    }
    //     bool                    is_dock_space() const     { return (merged_flags & ImGuiDockNodeFlags_DockSpace) != 0; }
    pub fn is_dock_space(&self) -> bool {
        // (&self.merged_flags & DimgDockNodeFlags::DockSpace) != 0
        self.merged_flags.contains(&DockNodeFlags::DockSpace) == false
    }
    //     bool                    is_floating_node() const  { return parent_node == None && (merged_flags & ImGuiDockNodeFlags_DockSpace) == 0; }
    pub fn is_floating_node(&self) -> bool {
        // self.parent_node.is_null() && &self.merged_flags & DimgDockNodeFlags::DockSpace == 0
        self.is_root_node() == false
            && self.merged_flags.contains(&DockNodeFlags::DockSpace) == false
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
    //     bool                    is_split_node() const     { return child_nodes[0] != None; }
    pub fn is_split_node(&self) -> bool {
        self.child_nodes[0] != INVALID_ID
    }
    //     bool                    is_leaf_node() const      { return child_nodes[0] == None; }
    pub fn is_leaf_node(&self) -> bool {
        self.child_nodes[0] == INVALID_ID
    }
    //     bool                    is_empty() const         { return child_nodes[0] == None && windows.len() == 0; }
    pub fn is_empty(&self) -> bool {
        // self.child_nodes[0].is_null() && self.windows.is_empty()
        self.child_nodes[0] == INVALID_ID
            && self.child_nodes[1] == INVALID_ID
            && self.windows.is_empty()
    }
    //     ImRect                  rect() const            { return ImRect(pos.x, pos.y, pos.x + size.x, pos.y + size.y); }
    pub fn rect(&self) -> Rect {
        Rect::new4(
            self.pos.x,
            self.pos.y,
            self.pos.x + self.size.x,
            self.pos.y + self.size.y,
        )
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
