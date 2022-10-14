#![allow(non_snake_case)]

use crate::dock_node_flags::ImGuiDockNodeFlags;
use crate::GImGui;
use crate::next_window_data_flags::ImGuiNextWindowDataFlags_HasWindowClass;
use crate::tab_item_flags::ImGuiTabItemFlags;
use crate::type_defs::ImGuiID;
use crate::viewport_flags::ImGuiViewportFlags;

// [ALPHA] Rarely used / very advanced uses only. Use with SetNextWindowClass() and DockSpace() functions.
// Important: the content of this class is still highly WIP and likely to change and be refactored
// before we stabilize Docking features. Please be mindful if using this.
// Provide hints:
// - To the platform backend via altered viewport flags (enable/disable OS decoration, OS task bar icons, etc.)
// - To the platform backend for OS level parent/child relationships of viewport.
// - To the docking system for various options and filtering.
#[derive(Debug, Default, Clone)]
pub struct ImGuiWindowClass {
    pub ClassId: ImGuiID,
    // User data. 0 = Default class (unclassed). Windows of different classes cannot be docked with each others.
    pub ParentViewportId: ImGuiID,
    // Hint for the platform backend. -1: use default. 0: request platform backend to not parent the platform. != 0: request platform backend to create a parent<>child relationship between the platform windows. Not conforming backends are free to e.g. parent every viewport to the main viewport or not.
    pub ViewportFlagsOverrideSet: ImGuiViewportFlags,
    // Viewport flags to set when a window of this class owns a viewport. This allows you to enforce OS decoration or task bar icon, override the defaults on a per-window basis.
    pub ViewportFlagsOverrideClear: ImGuiViewportFlags,
    // Viewport flags to clear when a window of this class owns a viewport. This allows you to enforce OS decoration or task bar icon, override the defaults on a per-window basis.
    pub TabItemFlagsOverrideSet: ImGuiTabItemFlags,
    // [EXPERIMENTAL] TabItem flags to set when a window of this class gets submitted into a dock node tab bar. May use with ImGuiTabItemFlags_Leading or ImGuiTabItemFlags_Trailing.
    pub DockNodeFlagsOverrideSet: ImGuiDockNodeFlags,
    // [EXPERIMENTAL] Dock node flags to set when a window of this class is hosted by a dock node (it doesn't have to be selected!)
    pub DockingAlwaysTabBar: bool,
    // Set to true to enforce single floating windows of this class always having their own docking node (equivalent of setting the global io.ConfigDockingAlwaysTabBar)
    pub DockingAllowUnclassed: bool,      // Set to true to allow windows of this class to be docked/merged with an unclassed window. // FIXME-DOCK: Move to DockNodeFlags override?

    // ImGuiWindowClass() { memset(this, 0, sizeof(*this)); ParentViewportId = (ImGuiID)-1; DockingAllowUnclassed = true; }
}

impl ImGuiWindowClass {
    pub fn new() -> Self {
        Self {
            ParentViewportId: -1,
            DockingAllowUnclassed: true,
            ..Default::default()
        }
    }
}

pub unsafe fn SetNextWindowClass(mut window_class: *const ImGuiWindowClass)
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    // IM_ASSERT((window_class->ViewportFlagsOverrideSet & window_class->ViewportFlagsOverrideClear) == 0); // Cannot set both set and clear for the same bit
    g.NextWindowData.Flags |= ImGuiNextWindowDataFlags_HasWindowClass;
    g.NextWindowData.WindowClass = (*window_class).clone();
}