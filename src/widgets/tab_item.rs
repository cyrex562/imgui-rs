#![allow(non_snake_case)]

use crate::widgets::tab_item_flags::ImGuiTabItemFlags;
use crate::core::type_defs::ImguiHandle;
use crate::window::ImguiWindow;
use libc::{c_float, c_int};

// Storage for one active tab item (sizeof() 48 bytes)
#[derive(Default, Debug, Clone)]
pub struct ImGuiTabItem {
    pub ID: ImguiHandle,
    pub Flags: ImGuiTabItemFlags,
    pub window: &mut ImguiWindow,
    // When TabItem is part of a DockNode's TabBar, we hold on to a window.
    pub LastFrameVisible: c_int,
    pub LastFrameSelected: c_int,
    // This allows us to infer an ordered list of the last activated tabs with little maintenance
    pub Offset: c_float,
    // Position relative to beginning of tab
    pub Width: c_float,
    // Width currently displayed
    pub ContentWidth: c_float,
    // Width of label, stored during BeginTabItem() call
    pub RequestedWidth: c_float,
    // Width optionally requested by caller, -1.0 is unused
    pub NameOffset: i32,
    // When Window==NULL, offset to name within parent ImGuiTabBar::TabsNames
    pub BeginOrder: i16,
    // BeginTabItem() order, used to re-order tabs after toggling ImGuiTabBarFlags_Reorderable
    pub IndexDuringLayout: i16,
    // Index only used during TabBarLayout()
    pub WantClose: bool, // Marked as closed by SetTabItemClosed()
}

impl ImGuiTabItem {
    // ImGuiTabItem()      {
    pub fn new() -> Self {
        // memset(this, 0, sizeof(*this));
        //     LastFrameVisible = LastFrameSelected = -1;
        //     RequestedWidth = -1.0;
        //     NameOffset = -1;
        //     BeginOrder = IndexDuringLayout = -1;
        Self {
            LastFrameVisible: -1,
            LastFrameSelected: -1,
            RequestedWidth: -1.0,
            NameOffset: -1,
            BeginOrder: -1,
            IndexDuringLayout: -1,
            ..Default::default()
        }
    }
}
