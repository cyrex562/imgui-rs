use crate::imgui_h::{ImGuiID, ImGuiTabBarFlags};
use crate::imgui_rect::ImRect;
use crate::imgui_text_buffer::ImGuiTextBuffer;
use crate::imgui_vec::ImVec2;
use crate::imgui_window::ImGuiWindow;


#[allow(non_camel_case_types)]// Flags for ImGui::BeginTabItem()
pub enum ImGuiTabItemFlags {
    None = 0,
    UnsavedDocument = 1 << 0,
    // Display a dot next to the title + tab is selected when clicking the X + closure is not assumed (will wait for user to stop submitting the tab). Otherwise closure is assumed when pressing the X, so if you keep submitting the tab may reappear at end of tab bar.
    SetSelected = 1 << 1,
    // Trigger flag to programmatically make the tab selected when calling BeginTabItem()
    NoCloseWithMiddleMouseButton = 1 << 2,
    // Disable behavior of closing tabs (that are submitted with p_open != NULL) with middle mouse button. You can still repro this behavior on user's side with if (IsItemHovered() && IsMouseClicked(2)) *p_open = false.
    NoPushId = 1 << 3,
    // Don't call PushID(tab->ID)/PopID() on BeginTabItem()/EndTabItem()
    NoTooltip = 1 << 4,
    // Disable tooltip for the given tab
    NoReorder = 1 << 5,
    // Disable reordering this tab or having another tab cross over this tab
    Leading = 1 << 6,
    // Enforce the tab position to the left of the tab bar (after the tab list popup button)
    Trailing = 1 << 7,    // Enforce the tab position to the right of the tab bar (before the scrolling buttons)
}


// Storage for one active tab item (sizeof() 48 bytes)
#[derive(Clone, Debug, Default)]
pub struct ImGuiTabItem {
    // ImGuiID             ID;
    pub ID: ImGuiID,
    // ImGuiTabItemFlags   Flags;
    pub Flags: ImGuiTabItemFlags,
    // ImGuiWindow*        Window;                 // When TabItem is part of a DockNode's TabBar, we hold on to a window.
    pub Window: *mut ImGuiWindow,
    // int                 LastFrameVisible;
    pub LastFrameVisible: i32,
    // int                 LastFrameSelected;      // This allows us to infer an ordered list of the last activated tabs with little maintenance
    pub LastFrameSelected: i32,
    // float               Offset;                 // Position relative to beginning of tab
    pub Offset: f32,
    // float               Width;                  // Width currently displayed
    pub Width: f32,
    // float               ContentWidth;           // Width of label, stored during BeginTabItem() call
    pub ContentWidth: f32,
    // float               RequestedWidth;         // Width optionally requested by caller, -1.0 is unused
    pub RequestedWidth: f32,
    // ImS32               NameOffset;             // When Window==NULL, offset to name within parent ImGuiTabBar::TabsNames
    pub NameOffset: i32,
    // ImS16               BeginOrder;             // BeginTabItem() order, used to re-order tabs after toggling ImGuiTabBarFlags_Reorderable
    pub BeginOrder: i16,
    // ImS16               IndexDuringLayout;      // Index only used during TabBarLayout()
    pub IndexDuringLayout: i16,
    // bool                WantClose;              // Marked as closed by SetTabItemClosed()
    pub WantClose: bool,
}

impl ImGuiTabItem {
    //     ImGuiTabItem()      { memset(this, 0, sizeof(*this)); LastFrameVisible = LastFrameSelected = -1; NameOffset = -1; BeginOrder = IndexDuringLayout = -1; }
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}


// Storage for a tab bar (sizeof() 152 bytes)
#[derive(Clone, Debug, Default)]
pub struct ImGuiTabBar {
    // ImVector<ImGuiTabItem> Tabs;
    pub Tabs: Vec<ImGuiTabItem>,
    // ImGuiTabBarFlags    Flags;
    pub Flags: ImGuiTabBarFlags,
    // ImGuiID             ID;                     // Zero for tab-bars used by docking
    pub ID: ImGuiID,
    // ImGuiID             SelectedTabId;          // Selected tab/window
    pub SelectedTabId: ImGuiID,
    // ImGuiID             NextSelectedTabId;      // Next selected tab/window. Will also trigger a scrolling animation
    pub NextSelectedTabId: ImGuiID,
    // ImGuiID             VisibleTabId;           // Can occasionally be != SelectedTabId (e.g. when previewing contents for CTRL+TAB preview)
    pub VisibleTabId: ImGuiID,
    // int                 CurrFrameVisible;
    pub CurrFrameVisible: i32,
    // int                 PrevFrameVisible;
    pub PrevFrameVisible: i32,
    // ImRect              BarRect;
    pub BarRect: ImRect,
    // float               CurrTabsContentsHeight;
    pub CurrTabsContentsHeight: f32,
    // float               PrevTabsContentsHeight; // Record the height of contents submitted below the tab bar
    pub PrevTabsContentsHeight: f32,
    // float               WidthAllTabs;           // Actual width of all tabs (locked during layout)
    pub WidthAllTabs: f32,
    // float               WidthAllTabsIdeal;      // Ideal width if all tabs were visible and not clipped
    pub WidthAllTabsIdeal: f32,
    // float               ScrollingAnim;
    pub ScrollingAnim: f32,
    // float               ScrollingTarget;
    pub ScrollingTarget: f32,
    // float               ScrollingTargetDistToVisibility;
    pub ScrollingTargetDistToVisibility: f32,
    // float               ScrollingSpeed;
    pub ScrollingSpeed: f32,
    // float               ScrollingRectMinX;
    pub ScrollingRectMinX: f32,
    // float               ScrollingRectMaxX;
    pub ScrollingRectMaxX: f32,
    // ImGuiID             ReorderRequestTabId;
    pub ReorderRequestTabId: ImGuiID,
    // ImS16               ReorderRequestOffset;
    pub ReorderRequestOffset: i16,
    // ImS8                BeginCount;
    pub BeginCount: i8,
    // bool                WantLayout;
    pub WantLayout: bool,
    // bool                VisibleTabWasSubmitted;
    pub VisibleTabWasSubmitted: bool,
    // bool                TabsAddedNew;           // Set to true when a new tab item or button has been added to the tab bar during last frame
    pub TabsAddedNew: bool,
    // ImS16               TabsActiveCount;        // Number of tabs submitted this frame.
    pub TabsActiveCount: i16,
    // ImS16               LastTabItemIdx;         // Index of last BeginTabItem() tab for use by EndTabItem()
    pub LastTabItemIdx: i16,
    // float               ItemSpacingY;
    pub ItemSpacingY: f32,
    // ImVec2              FramePadding;           // style.FramePadding locked at the time of BeginTabBar()
    pub FramePadding: ImVec2,
    // ImVec2              BackupCursorPos;
    pub BackupCursorPos: ImVec2,
    // ImGuiTextBuffer     TabsNames;              // For non-docking tab bar we re-append names in a contiguous buffer.
    pub TabsNames: ImGuiTextBuffer,

}
