#![allow(non_snake_case)]

use crate::core::condition::ImGuiCond;
use crate::next_window_data_flags::{ImGuiNextWindowDataFlags, ImGuiNextWindowDataFlags_None};
use crate::rect::ImRect;
use crate::type_defs::ImguiHandle;
use crate::vec2::ImVec2;
use crate::window::window_class::ImGuiWindowClass;
use crate::window_class::ImGuiWindowClass;
use libc::{c_float, c_void};

// Storage for SetNexWindow** functions
#[derive(Default, Debug, Clone)]
pub struct ImGuiNextWindowData {
    pub Flags: ImGuiNextWindowDataFlags,
    pub PosCond: ImGuiCond,
    pub SizeCond: ImGuiCond,
    pub CollapsedCond: ImGuiCond,
    pub DockCond: ImGuiCond,
    pub PosVal: ImVec2,
    pub PosPivotVal: Option<ImVec2>,
    pub SizeVal: ImVec2,
    pub ContentSizeVal: ImVec2,
    pub ScrollVal: ImVec2,
    pub PosUndock: bool,
    pub CollapsedVal: bool,
    pub SizeConstraintRect: ImRect,
    pub SizeCallback: ImGuiSizeCallback,
    pub SizeCallbackUserData: Option<Vec<u8>>,
    pub BgAlphaVal: c_float,
    // Override background alpha
    pub ViewportId: ImguiHandle,
    pub DockId: ImguiHandle,
    pub WindowClass: ImGuiWindowClass,
    pub MenuBarOffsetMinVal: ImVec2, // (Always on) This is not exposed publicly, so we don't clear it and it doesn't have a corresponding flag (could we? for consistency?)
}

impl ImGuiNextWindowData {
    // ImGuiNextWindowData()       { memset(this, 0, sizeof(*this)); }

    // inline void ClearFlags()    { Flags = ImGuiNextWindowDataFlags_None; }
    pub fn ClearFlags(&mut self) {
        self.Flags = ImGuiNextWindowDataFlags_None;
    }
}
