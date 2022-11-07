use crate::axis::{ImGuiAxis, ImGuiAxis_None};
use crate::dock_node_flags::ImGuiDockNodeFlags;
use crate::type_defs::ImguiHandle;
use crate::vec2::ImVec2ih;
use libc::c_char;

// Persistent Settings data, stored contiguously in SettingsNodes (sizeof() ~32 bytes)
#[derive(Default, Debug, Clone, Copy)]
pub struct ImGuiDockNodeSettings {
    pub ID: ImguiHandle,
    pub ParentNodeId: ImguiHandle,
    pub ParentWindowId: ImguiHandle,
    pub SelectedTabId: ImguiHandle,
    pub SplitAxis: ImGuiAxis,
    pub Depth: c_char,
    pub Flags: ImGuiDockNodeFlags, // NB: We save individual flags one by one in ascii format (ImGuiDockNodeFlags_SavedFlagsMask_)
    pub Pos: ImVec2ih,
    pub Size: ImVec2ih,
    pub SizeRef: ImVec2ih,
}

impl ImGuiDockNodeSettings {
    pub fn new() -> Self {
        // ImGuiDockNodeSettings() { memset(this, 0, sizeof(*this)); SplitAxis = ImGuiAxis_None; }
        Self {
            SplitAxis: ImGuiAxis_None,
            ..Default::default()
        }
    }
}
