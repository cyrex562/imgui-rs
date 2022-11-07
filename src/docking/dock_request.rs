#![allow(non_snake_case)]

use crate::core::direction::{ImGuiDir, ImGuiDir_None};
use crate::docking::dock_node::ImGuiDockNode;
use crate::docking::dock_request_type::{ImGuiDockRequestType, ImGuiDockRequestType_None};
use crate::window::ImguiWindow;
use libc::c_float;
use std::ptr::null_mut;

#[derive(Default, Debug, Clone, Copy)]
pub struct ImGuiDockRequest {
    pub Type: ImGuiDockRequestType,
    pub DockTargetWindow: *mut ImguiWindow, // Destination/Target Window to dock into (may be a loose window or a DockNode, might be NULL in which case DockTargetNode cannot be NULL)
    pub DockTargetNode: *mut ImGuiDockNode, // Destination/Target Node to dock into
    pub DockPayload: *mut ImguiWindow, // Source/Payload window to dock (may be a loose window or a DockNode), [Optional]
    pub DockSplitDir: ImGuiDir,
    pub DockSplitRatio: c_float,
    pub DockSplitOuter: bool,
    pub UndockTargetWindow: *mut ImguiWindow,
    pub UndockTargetNode: *mut ImGuiDockNode,
}

impl ImGuiDockRequest {
    pub fn new() -> Self {
        Self {
            Type: ImGuiDockRequestType_None,
            DockTargetWindow: None,
            DockPayload: None,
            UndockTargetWindow: None,
            DockTargetNode: None,
            UndockTargetNode: None,
            DockSplitDir: ImGuiDir_None,
            DockSplitRatio: 0.5,
            DockSplitOuter: false,
        }
    }
}
