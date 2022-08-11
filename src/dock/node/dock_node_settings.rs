use std::collections::HashSet;
use crate::dock::node::DockNodeFlags;
use crate::types::Id32;
use crate::vectors::Vector2D;

// Persistent Settings data, stored contiguously in SettingsNodes (sizeof() ~32 bytes)
#[derive(Debug, Clone, Default)]
pub struct DockNodeSettings {
    pub id: Id32,
    pub parent_node_id: Id32,
    pub parent_window_id: Id32,
    pub selected_tab_id: Id32,
    pub split_axis: i8,
    pub depth: i8,
    pub flags: HashSet<DockNodeFlags>,
    pub pos: Vector2D,
    pub size: Vector2D,
    pub size_ref: Vector2D,
}
