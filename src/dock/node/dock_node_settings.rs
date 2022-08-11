use std::collections::HashSet;
use crate::dock::node::DockNodeFlags;
use crate::types::Id32;
use crate::vectors::Vector2D;

// Persistent Settings data, stored contiguously in SettingsNodes (sizeof() ~32 bytes)
#[derive(Debug, Clone, Default)]
pub struct DockNodeSettings {
    // Id32             id;
    pub id: Id32,
    // Id32             parent_node_id;
    pub parent_node_id: Id32,
    // Id32             ParentWindowId;
    pub parent_window_id: Id32,
    // Id32             SelectedTabId;
    pub selected_tab_id: Id32,
    // signed char         SplitAxis;
    pub split_axis: i8,
    // char                Depth;
    pub depth: i8,
    // ImGuiDockNodeFlags  flags;                  // NB: We save individual flags one by one in ascii format (ImGuiDockNodeFlags_SavedFlagsMask_)
    pub flags: HashSet<DockNodeFlags>,
    // Vector2D            pos;
    pub pos: Vector2D,
    // Vector2D            size;
    pub size: Vector2D,
    // Vector2D            SizeRef;
    pub size_ref: Vector2D,
    // ImGuiDockNodeSettings() { memset(this, 0, sizeof(*this)); SplitAxis = ImGuiAxis_None; }
}
