#![allow(non_snake_case)]

use crate::table_column_settings::ImGuiTableColumnSettings;
use crate::table_flags::ImGuiTableFlags;
use crate::type_defs::{ImGuiTableColumnIdx, ImguiHandle};
use libc::c_float;

// This is designed to be stored in a single ImChunkStream (1 header followed by N ImGuiTableColumnSettings, etc.)
#[derive(Default, Debug, Clone)]
pub struct ImGuiTableSettings {
    pub ID: ImguiHandle,            // Set to 0 to invalidate/delete the setting
    pub SaveFlags: ImGuiTableFlags, // Indicate data we want to save using the Resizable/Reorderable/Sortable/Hideable flags (could be using its own flags..)
    pub RefScale: c_float, // Reference scale to be able to rescale columns on font/dpi changes.
    pub ColumnsCount: ImGuiTableColumnIdx,
    pub ColumnsCountMax: ImGuiTableColumnIdx, // Maximum number of columns this settings instance can store, we can recycle a settings instance with lower number of columns but not higher
    pub WantApply: bool, // Set when loaded from .ini data (to enable merging/loading .ini data into an already running context)
}

impl ImGuiTableSettings {
    // ImGuiTableSettings()        { memset(this, 0, sizeof(*this)); }

    // *mut ImGuiTableColumnSettings   GetColumnSettings()     { return (*mut ImGuiTableColumnSettings)(this + 1); }
    pub fn GetColumnSettings(&mut self) -> *mut ImGuiTableColumnSettings {
        todo!()
    }
}
