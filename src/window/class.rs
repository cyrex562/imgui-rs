use crate::dock::node::DockNodeFlags;
use crate::tab_bar::DimgTabItemFlags;
use crate::types::Id32;
use crate::ViewportFlags;

/// [ALPHA] Rarely used / very advanced uses only. Use with SetNextWindowClass() and DockSpace() functions.
/// Important: the content of this class is still highly WIP and likely to change and be refactored
/// before we stabilize Docking features. Please be mindful if using this.
/// Provide hints:
/// - To the platform backend via altered viewport flags (enable/disable OS decoration, OS task bar icons, etc.)
/// - To the platform backend for OS level parent/child relationships of viewport.
/// - To the docking system for various options and filtering.
#[derive(Default,Debug,Clone)]
pub struct WindowClass
{
    pub class_id: Id32,                  // User data. 0 = Default class (unclassed). windows of different classes cannot be docked with each others.
    pub parent_viewport_id: Id32,         // Hint for the platform backend. -1: use default. 0: request platform backend to not parent the platform. != 0: request platform backend to create a parent<>child relationship between the platform windows. Not conforming backends are free to e.g. parent every viewport to the main viewport or not.
    pub viewport_flags_override_set: ViewportFlags,   // viewport flags to set when a window of this class owns a viewport. This allows you to enforce OS decoration or task bar icon, override the defaults on a per-window basis.
    pub viewport_flags_override_clear: ViewportFlags, // viewport flags to clear when a window of this class owns a viewport. This allows you to enforce OS decoration or task bar icon, override the defaults on a per-window basis.
    pub tab_item_flags_override_set: DimgTabItemFlags,    // [EXPERIMENTAL] TabItem flags to set when a window of this class gets submitted into a dock node tab bar. May use with ImGuiTabItemFlags_Leading or ImGuiTabItemFlags_Trailing.
    pub dock_node_flags_override_set: DockNodeFlags,   // [EXPERIMENTAL] Dock node flags to set when a window of this class is hosted by a dock node (it doesn't have to be selected!)
    pub docking_always_tab_bar: bool,        // Set to true to enforce single floating windows of this class always having their own docking node (equivalent of setting the global io.config_docking_always_tab_bar)
    pub docking_allow_unclassed: bool,      // Set to true to allow windows of this class to be docked/merged with an unclassed window. // FIXME-DOCK: Move to DockNodeFlags override?

}

impl WindowClass {
    // ImGuiWindowClass() { memset(this, 0, sizeof(*this)); parent_viewport_id = (ImGuiID)-1; docking_allow_unclassed = true;
    pub fn new() -> Self {
        Self {
            parent_viewport_id: Id32::MAX,
            docking_allow_unclassed: true,
            ..Default::default()
        }
    }
}
