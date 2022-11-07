// Windows data saved in imgui.ini file
#![allow(non_snake_case)]
// Because we never destroy or rename ImGuiWindowSettings, we can store the names in a separate buffer easily.
// (this is designed to be stored in a ImChunkStream buffer, with the variable-length Name following our structure)

use crate::type_defs::ImguiHandle;
use crate::vec2::ImVec2ih;
use libc::{c_char, c_short};

#[derive(Default, Debug, Clone)]
pub struct ImGuiWindowSettings {
    pub ID: ImguiHandle,
    pub Pos: ImVec2ih,
    // NB: Settings position are stored RELATIVE to the viewport! Whereas runtime ones are absolute positions.
    pub Size: ImVec2ih,
    pub ViewportPos: ImVec2ih,
    pub ViewportId: ImguiHandle,
    pub DockId: ImguiHandle,
    // ID of last known DockNode (even if the DockNode is invisible because it has only 1 active window), or 0 if none.
    pub ClassId: ImguiHandle,
    // ID of window class if specified
    pub DockOrder: c_short,
    // Order of the last time the window was visible within its DockNode. This is used to reorder windows that are reappearing on the same frame. Same value between windows that were active and windows that were none are possible.
    pub Collapsed: bool,
    pub WantApply: bool, // Set when loaded from .ini data (to enable merging/loading .ini data into an already running context)
}

impl ImGuiWindowSettings {
    // ImGuiWindowSettings()       { memset(this, 0, sizeof(*this)); DockOrder = -1; }
    pub fn new() -> Self {
        Self {
            DockOrder: -1,
            ..Default::default()
        }
    }

    // *mut char GetName()             { return (this + 1); }
    pub fn GetName(&mut self) -> String {
        todo!()
    }
}
