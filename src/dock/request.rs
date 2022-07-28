use crate::types::{Direction, Id32};
use crate::INVALID_ID;

#[derive(Debug, Clone, Default)]
pub struct DockRequest {
    // ImGuiDockRequestType    Type;
    pub request_type: DockRequestType,
    // ImGuiWindow*            DockTargetWindow;   // Destination/Target window to dock into (may be a loose window or a dock_node, might be None in which case DockTargetNode cannot be None)
    pub dock_target_window_id: Id32,
    // ImGuiDockNode*          DockTargetNode;     // Destination/Target Node to dock into
    pub dock_target_node_id: Id32,
    // ImGuiWindow*            DockPayload;        // Source/Payload window to dock (may be a loose window or a dock_node), [Optional]
    pub dock_payload_id: Id32,
    // ImGuiDir                DockSplitDir;
    pub dock_split_dir: Direction,
    // float                   DockSplitRatio;
    pub dock_split_ratio: f32,
    // bool                    DockSplitOuter;
    pub dock_split_outer: bool,
    // ImGuiWindow*            UndockTargetWindow;
    pub undock_target_window_id: Id32,
    // ImGuiDockNode*          UndockTargetNode;
    pub undock_target_node_id: Id32,
}

impl DockRequest {
    //ImGuiDockRequest()
    pub fn new() -> Self {
        // Type = None;
        // DockTargetWindow = DockPayload = UndockTargetWindow = None;
        // DockTargetNode = UndockTargetNode = None;
        // DockSplitDir = ImGuiDir_None;
        // DockSplitRatio = 0.5;
        // DockSplitOuter = false;
        Self {
            request_type: DockRequestType::None,
            dock_target_window_id: INVALID_ID,
            dock_payload_id: INVALID_ID,
            undock_target_window_id: INVALID_ID,
            dock_target_node_id: INVALID_ID,
            undock_target_node_id: INVALID_ID,
            dock_split_dir: Direction::None,
            dock_split_ratio: 0.5,
            dock_split_outer: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum DockRequestType {
    None,
    Dock,
    Undock,
    Split, // split is the same as Dock but without a DockPayload
}

impl Default for DockRequestType {
    fn default() -> Self {
        Self::None
    }
}
