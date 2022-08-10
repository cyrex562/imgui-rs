use crate::imgui_h::{Id32, ImGuiTabBarFlags};
use crate::imgui_rect::Rect;
use crate::imgui_text_buffer::ImGuiTextBuffer;
use crate::imgui_vec::Vector2D;
use crate::imgui_window::Window;
use crate::rect::Rect;
use crate::types::Id32;
use crate::vectors::vector_2d::Vector2D;
use std::collections::HashSet;

#[allow(non_camel_case_types)] // flags for ImGui::BeginTabItem()
pub enum TabItemFlags {
    None,
    UnsavedDocument,
    // Display a dot next to the title + tab is selected when clicking the x + closure is not assumed (will wait for user to stop submitting the tab). Otherwise closure is assumed when pressing the x, so if you keep submitting the tab may reappear at end of tab bar.
    SetSelected,
    // Trigger flag to programmatically make the tab selected when calling BeginTabItem()
    NoCloseWithMiddleMouseButton,
    // Disable behavior of closing tabs (that are submitted with p_open != None) with middle mouse button. You can still repro this behavior on user's side with if (IsItemHovered() && IsMouseClicked(2)) *p_open = false.
    NoPushId,
    // Don't call push_id(tab->id)/PopID() on BeginTabItem()/EndTabItem()
    NoTooltip,
    // Disable tooltip for the given tab
    NoReorder,
    // Disable reordering this tab or having another tab cross over this tab
    Leading,
    // Enforce the tab position to the left of the tab bar (after the tab list popup button)
    Trailing, // Enforce the tab position to the right of the tab bar (before the scrolling buttons)
     NoCloseButton            ,  // Track whether p_open was set or not (we'll need this info on the next frame to recompute ContentWidth during layout)
    Button                   ,  // Used by TabItemButton, change the tab item behavior to mimic a button
    Unsorted                 ,  // [Docking] Trailing tabs with the _Unsorted flag will be sorted based on the dock_order of their window.
    Preview                 // [Docking] Display tab shape for docking preview (height is adjusted slightly to compensate for the yet missing tab bar)
}

// Storage for one active tab item (sizeof() 48 bytes)
#[derive(Clone, Debug, Default)]
pub struct TabItem {
    // Id32             id;
    pub id: Id32,
    // ImGuiTabItemFlags   flags;
    pub flags: HashSet<TabItemFlags>,
    // Window*        window;                 // When TabItem is part of a dock_node's tab_bar, we hold on to a window.
    pub window_id: Id32,
    // int                 LastFrameVisible;
    pub last_frame_visible: i32,
    // int                 LastFrameSelected;      // This allows us to infer an ordered list of the last activated tabs with little maintenance
    pub last_frame_selected: i32,
    // float               Offset;                 // Position relative to beginning of tab
    pub offset: f32,
    // float               width;                  // width currently displayed
    pub width: f32,
    // float               ContentWidth;           // width of label, stored during BeginTabItem() call
    pub content_width: f32,
    // float               RequestedWidth;         // width optionally requested by caller, -1.0 is unused
    pub requested_width: f32,
    // ImS32               name_offset;             // When window==None, offset to name within parent ImGuiTabBar::TabsNames
    pub name_offset: i32,
    // ImS16               BeginOrder;             // BeginTabItem() order, used to re-order tabs after toggling ImGuiTabBarFlags_Reorderable
    pub begin_order: i16,
    // ImS16               IndexDuringLayout;      // index only used during TabBarLayout()
    pub index_during_layout: i16,
    // bool                WantClose;              // Marked as closed by SetTabItemClosed()
    pub want_close: bool,
}

// Storage for a tab bar (sizeof() 152 bytes)
#[derive(Clone, Debug, Default)]
pub struct TabBar {
    // ImVector<ImGuiTabItem> Tabs;
    pub tabs: Vec<TabItem>,
    // ImGuiTabBarFlags    flags;
    pub flags: HashSet<TabBarFlags>,
    // Id32             id;                     // Zero for tab-bars used by docking
    pub id: Id32,
    // Id32             selected_tab_id;          // Selected tab/window
    pub selected_tab_id: Id32,
    // Id32             NextSelectedTabId;      // Next selected tab/window. Will also trigger a scrolling animation
    pub next_selected_tab_id: Id32,
    // Id32             VisibleTabId;           // Can occasionally be != selected_tab_id (e.g. when previewing contents for CTRL+TAB preview)
    pub visible_tab_id: Id32,
    // int                 CurrFrameVisible;
    pub curr_frame_visible: i32,
    // int                 PrevFrameVisible;
    pub prev_frame_visible: i32,
    // ImRect              BarRect;
    pub bar_rect: Rect,
    // float               CurrTabsContentsHeight;
    pub curr_tabs_contents_height: f32,
    // float               PrevTabsContentsHeight; // Record the height of contents submitted below the tab bar
    pub prev_tabs_contents_height: f32,
    // float               WidthAllTabs;           // Actual width of all tabs (locked during layout)
    pub width_all_tabs: f32,
    // float               WidthAllTabsIdeal;      // Ideal width if all tabs were visible and not clipped
    pub width_all_tabs_ideal: f32,
    // float               ScrollingAnim;
    pub scrolling_anim: f32,
    // float               ScrollingTarget;
    pub scrolling_target: f32,
    // float               ScrollingTargetDistToVisibility;
    pub scrolling_target_dist_to_visibility: f32,
    // float               ScrollingSpeed;
    pub scrolling_speed: f32,
    // float               ScrollingRectMinX;
    pub scrolling_rect_min_x: f32,
    // float               ScrollingRectMaxX;
    pub scrolling_rect_max_x: f32,
    // Id32             ReorderRequestTabId;
    pub reorder_request_tab_id: Id32,
    // ImS16               ReorderRequestOffset;
    pub reorder_request_offset: i16,
    // ImS8                begin_count;
    pub begin_count: i8,
    // bool                WantLayout;
    pub want_layout: bool,
    // bool                VisibleTabWasSubmitted;
    pub visible_tab_was_submitted: bool,
    // bool                TabsAddedNew;           // Set to true when a new tab item or button has been added to the tab bar during last frame
    pub tabs_add_new: bool,
    // ImS16               TabsActiveCount;        // Number of tabs submitted this frame.
    pub tabs_active_count: i16,
    // ImS16               LastTabItemIdx;         // index of last BeginTabItem() tab for use by EndTabItem()
    pub last_tab_item_idx: i16,
    // float               ItemSpacingY;
    pub item_spacing_y: f32,
    // Vector2D              FramePadding;           // style.FramePadding locked at the time of BeginTabBar()
    pub frame_padding: Vector2D,
    // Vector2D              backup_cursor_pos;
    pub backup_cursor_pos: Vector2D,
    // ImGuiTextBuffer     TabsNames;              // For non-docking tab bar we re-append names in a contiguous buffer.
    pub tabs_names: String,
}

// pub const     FittingPolicyDefault_: i32          = tab_bar_flags::FittingPolicyResizeDown as i32;
pub const FITTING_POLICY_DFLT: TabBarFlags = TabBarFlags::FittingPolicyResizeDown;

// pub const FittingPolicyMask_ : i32            = tab_bar_flags::FittingPolicyResizeDown | tab_bar_flags::FittingPolicyScroll;
pub const FITTING_POLICY_MASK: HashSet<TabBarFlags> = HashSet::from([
    TabBarFlags::FittingPolicyResizeDown,
    TabBarFlags::FittingPolicyScroll,
]);

// flags for ImGui::BeginTabBar()
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TabBarFlags {
    None = 0,
    Reorderable,
    // Allow manually dragging tabs to re-order them + New tabs are appended at the end of list
    AutoSelectNewTabs,
    // Automatically select new tabs when they appear
    TabListPopupButton,
    // Disable buttons to open the tab list popup
    NoCloseWithMiddleMouseButton,
    // Disable behavior of closing tabs (that are submitted with p_open != None) with middle mouse button. You can still repro this behavior on user's side with if (IsItemHovered() && IsMouseClicked(2)) *p_open = false.
    NoTabListScrollingButtons,
    // Disable scrolling buttons (apply when fitting policy is ImGuiTabBarFlags_FittingPolicyScroll)
    NoTooltip,
    // Disable tooltips when hovering a tab
    FittingPolicyResizeDown,
    // Resize tabs when they don't fit
    FittingPolicyScroll,
    // Add scroll buttons when tabs don't fit
    // ImGuiTabBarFlags_FittingPolicyMask_             = ImGuiTabBarFlags_FittingPolicyResizeDown | ImGuiTabBarFlags_FittingPolicyScroll,
    // ImGuiTabBarFlags_FittingPolicyDefault_          = ImGuiTabBarFlags_FittingPolicyResizeDown
    DockNode,
    // Part of a dock node [we don't use this in the master branch but it facilitate branch syncing to keep this around]
    IsFocused,
    SaveSettings, // FIXME: Settings are handled by the docking system, this only request the tab bar to mark settings dirty when reordering tabs
}
