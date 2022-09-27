#![allow(non_snake_case)]

use std::ptr::null_mut;
use libc::c_float;
use crate::direction::{ImGuiDir, ImGuiDir_None};
use crate::dock_node::ImGuiDockNode;
use crate::dock_request_type::{ImGuiDockRequestType, ImGuiDockRequestType_None};
use crate::window::ImGuiWindow;

#[derive(Default,Debug,Clone)]
pub struct ImGuiDockRequest
{
pub Type:  ImGuiDockRequestType,
pub DockTargetWindow:  *mut ImGuiWindow,   // Destination/Target Window to dock into (may be a loose window or a DockNode, might be NULL in which case DockTargetNode cannot be NULL)
pub DockTargetNode:  *mut ImGuiDockNode,     // Destination/Target Node to dock into
pub DockPayload:  *mut ImGuiWindow,        // Source/Payload window to dock (may be a loose window or a DockNode), [Optional]
pub DockSplitDir:  ImGuiDir,
pub DockSplitRatio:  c_float,
pub DockSplitOuter:  bool,
pub UndockTargetWindow:  *mut ImGuiWindow,
pub UndockTargetNode:  *mut ImGuiDockNode,


}

impl ImGuiDockRequest {
    pub fn new() -> Self {
        Self {
            Type: ImGuiDockRequestType_None,
            DockTargetWindow: null_mut(),
            DockPayload: null_mut(),
            UndockTargetWindow: null_mut(),
            DockTargetNode: null_mut(),
            UndockTargetNode: null_mut(),
            DockSplitDir: ImGuiDir_None,
            DockSplitRatio: 0.5f32,
            DockSplitOuter: false,
        }
    }
}
