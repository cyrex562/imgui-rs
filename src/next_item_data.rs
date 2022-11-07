#![allow(non_snake_case)]

use crate::condition::ImGuiCond;
use crate::next_item_data_flags::{ImGuiNextItemDataFlags, ImGuiNextItemDataFlags_None};
use crate::type_defs::ImguiHandle;
use libc::c_float;

#[derive(Default, Debug, Clone)]
pub struct ImGuiNextItemData {
    pub flags: ImGuiNextItemDataFlags,
    pub width: c_float,
    // Set by SetNextItemWidth()
    pub FocusScopeId: ImguiHandle,
    // Set by SetNextItemMultiSelectData() (!= 0 signify value has been set, so it's an alternate version of HasSelectionData, we don't use Flags for this because they are cleared too early. This is mostly used for debugging)
    pub OpenCond: ImGuiCond,
    pub OpenVal: bool, // Set by SetNextItemOpen()
}

impl ImGuiNextItemData {
    // ImGuiNextItemData()         { memset(this, 0, sizeof(*this)); }

    // inline void ClearFlags()    { Flags = ImGuiNextItemDataFlags_None; } // Also cleared manually by ItemAdd()!
    pub fn ClearFlags(&mut self) {
        self.flags = ImGuiNextItemDataFlags_None
    }
}
