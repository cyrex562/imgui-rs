#![allow(non_snake_case)]

use crate::rect::ImRect;
use crate::tab_bar_flags::ImGuiTabBarFlags;
use crate::tab_item::ImGuiTabItem;
use crate::text_buffer::ImGuiTextBuffer;
use crate::type_defs::ImGuiID;
use crate::vec2::ImVec2;
use libc::{c_char, c_float, c_int};
use std::borrow::{Borrow, BorrowMut};

// Storage for a tab bar (sizeof() 152 bytes)
#[derive(Default, Debug, Clone)]
pub struct ImGuiTabBar {
    pub Tabs: Vec<ImGuiTabItem>,
    pub Flags: ImGuiTabBarFlags,
    pub ID: ImGuiID,
    // Zero for tab-bars used by docking
    pub SelectedTabId: ImGuiID,
    // Selected tab/window
    pub NextSelectedTabId: ImGuiID,
    // Next selected tab/window. Will also trigger a scrolling animation
    pub VisibleTabId: ImGuiID,
    // Can occasionally be != SelectedTabId (e.g. when previewing contents for CTRL+TAB preview)
    pub CurrFrameVisible: c_int,
    pub PrevFrameVisible: c_int,
    pub BarRect: ImRect,
    pub CurrTabsContentsHeight: c_float,
    pub PrevTabsContentsHeight: c_float,
    // Record the height of contents submitted below the tab bar
    pub WidthAllTabs: c_float,
    // Actual width of all tabs (locked during layout)
    pub WidthAllTabsIdeal: c_float,
    // Ideal width if all tabs were visible and not clipped
    pub ScrollingAnim: c_float,
    pub ScrollingTarget: c_float,
    pub ScrollingTargetDistToVisibility: c_float,
    pub ScrollingSpeed: c_float,
    pub ScrollingRectMinX: c_float,
    pub ScrollingRectMaxX: c_float,
    pub ReorderRequestTabId: ImGuiID,
    pub ReorderRequestOffset: i16,
    pub BeginCount: i8,
    pub WantLayout: bool,
    pub VisibleTabWasSubmitted: bool,
    pub TabsAddedNew: bool,
    // Set to true when a new tab item or button has been added to the tab bar during last frame
    pub TabsActiveCount: i16,
    // Number of tabs submitted this frame.
    pub LastTabItemIdx: i16,
    // Index of last BeginTabItem() tab for use by EndTabItem()
    pub ItemSpacingY: c_float,
    pub FramePadding: ImVec2,
    // style.FramePadding locked at the time of BeginTabBar()
    pub BackupCursorPos: ImVec2,
    pub TabsNames: ImGuiTextBuffer, // For non-docking tab bar we re-append names in a contiguous buffer.
}

impl ImGuiTabBar {
    // ImGuiTabBar();
    pub fn new() -> Self {
        let mut out = Self {
            CurrFrameVisible: -1,
            PrevFrameVisible: -1,
            LastTabItemIdx: -1,
            ..Default()
        };
        out
    }

    // c_int                 GetTabOrder(*const ImGuiTabItem tab) const  { return Tabs.index_from_ptr(tab); }
    pub fn GetTabOrder(&self, tab: *const ImGuiTabItem) -> c_int {
        order_result = self.Tabs.binary_search(tab.borrow()).is_ok();
        return if order_result.is_ok() {
            order_result.unwrap()
        } else {
            -1
        };
    }

    // *const char         GetTabName(*const ImGuiTabItem tab) const
    pub fn GetTabname(&self, tab: *const ImGuiTabItem) -> *const c_char {
        if tab.Window.is_null() == false {
            return tab.Window.Name;
        }
        // IM_ASSERT(tab.NameOffset != -1 && tab.NameOffset < TabsNames.Buf.Size);
        return self.TabsNames.Buf.as_ptr() + tab.NameOffset;
    }
}
