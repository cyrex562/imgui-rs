use libc::c_float;
use crate::direction::{ImGuiDir, ImGuiDir_COUNT};
use crate::dock_node::ImGuiDockNode;
use crate::rect::ImRect;

#[derive(Default, Debug, Copy, Clone)]
pub struct ImGuiDockPreviewData {
    pub FutureNode: ImGuiDockNode,
    pub IsDropAllowed: bool,
    pub IsCenterAvailable: bool,
    pub IsSidesAvailable: bool,
    // Hold your breath, grammar freaks..
    pub IsSplitDirExplicit: bool,
    // Set when hovered the drop rect (vs. implicit SplitDir==None when hovered the window)
    pub SplitNode: *mut ImGuiDockNode,
    pub SplitDir: ImGuiDir,
    pub SplitRatio: c_float,
    // ImRect          DropRectsDraw[ImGuiDir_COUNT + 1];  // May be slightly different from hit-testing drop rects used in DockNodeCalcDropRects()
    pub DropRectsDraw: [ImRect; (ImGuiDir_COUNT + 1) as usize],

}

impl ImGuiDockPreviewData {
    // ImGuiDockPreviewData() : FutureNode(0) { IsDropAllowed = IsCenterAvailable = IsSidesAvailable = IsSplitDirExplicit = false; SplitNode= null_mut(); SplitDir = ImGuiDir_None; SplitRatio = 0.f; for (let n: c_int = 0; n < IM_ARRAYSIZE(DropRectsDraw); n++) DropRectsDraw[n] = ImRect::new(+f32::MAX, +f32::MAX, -f32::MAX, -f32::MAX); }
}
