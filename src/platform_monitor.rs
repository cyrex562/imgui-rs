#![allow(non_snake_case)]

use libc::c_float;
use crate::vec2::ImVec2;

// (Optional) This is required when enabling multi-viewport. Represent the bounds of each connected monitor/display and their DPI.
// We use this information for multiple DPI support + clamping the position of popups and tooltips so they don't straddle multiple monitors.
#[derive(Default, Debug, Clone)]
pub struct ImGuiPlatformMonitor {
    // ImVec2  MainPos, MainSize;      // Coordinates of the area displayed on this monitor (Min = upper left, Max = bottom right)
    pub MainPos: ImVec2,
    pub MainSize: ImVec2,

    // ImVec2  WorkPos, WorkSize;      // Coordinates without task bars / side bars / menu bars. Used to avoid positioning popups/tooltips inside this region. If you don't have this info, please copy the value for MainPos/MainSize.
    pub WorkPos: ImVec2,
    pub WorkSize: ImVec2,

    // c_float   DpiScale;               // 1f32 = 96 DPI
    pub DpiScale: c_float,

}

impl ImGuiPlatformMonitor {
    // ImGuiPlatformMonitor()          { MainPos = MainSize = WorkPos = WorkSize = ImVec2::new2(0, 0); DpiScale = 1f32; }
    pub fn new() -> Self {
        Self {
            MainPos: ImVec2::new(),
            MainSize: ImVec2::new(),
            WorkPos: ImVec2::new(),
            WorkSize: ImVec2::new(),
            DpiScale: 1f32,
        }
    }
}
