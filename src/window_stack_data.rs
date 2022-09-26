#![allow(non_snake_case)]

use crate::imgui_window::ImGuiWindow;
use crate::last_item_data::ImGuiLastItemData;

// Data saved for each window pushed into the stack
#[derive(Default, Debug, Clone)]
pub struct ImGuiWindowStackData {
    pub Window: *mut ImGuiWindow,
    pub ParentLastItemDataBackup: ImGuiLastItemData,
    pub StackSizesOnBegin: ImGuiStackSizes,      // Store size of various stacks for asserting
}
