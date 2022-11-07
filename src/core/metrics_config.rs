#![allow(non_snake_case)]

use libc::c_int;

#[derive(Default, Debug, Clone)]
pub struct ImGuiMetricsConfig {
    pub ShowDebugLog: bool,
    pub ShowStackTool: bool,
    pub ShowWindowsRects: bool,
    pub ShowWindowsBeginOrder: bool,
    pub ShowTablesRects: bool,
    pub ShowDrawCmdMesh: bool,
    pub ShowDrawCmdBoundingBoxes: bool,
    pub ShowDockingNodes: bool,
    pub ShowWindowsRectsType: i32,
    pub ShowTablesRectsType: i32,

}

impl ImGuiMetricsConfig {
    pub fn new() -> Self {
        Self {
            ShowDebugLog: true,
            ShowStackTool: true,
            ShowWindowsRects: true,
            ShowWindowsBeginOrder: true,
            ShowTablesRects: false,
            ShowDrawCmdMesh: true,
            ShowDrawCmdBoundingBoxes: true,
            ShowDockingNodes: false,
            ShowWindowsRectsType: -1,
            ShowTablesRectsType: -1,
        }
    }
}
