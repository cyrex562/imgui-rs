use crate::{rect::ImRect, direction::{ImGuiDir_COUNT, ImGuiDir, ImGuiDir_None}, dock_node::ImGuiDockNode};

#[derive(Default,Debug,Clone, Copy)]
pub struct ImGuiDockPreviewData
{
    // ImGuiDockNode   FutureNode;
    pub FutureNode: ImGuiDockNode,
    // bool            IsDropAllowed;
    pub IsDropAllowed: bool,
    // bool            IsCenterAvailable;
    pub IsCenterAvailable: bool,
    // bool            IsSidesAvailable;           // Hold your breath, grammar freaks..
    pub IsSidesAvailable: bool,
    // bool            IsSplitDirExplicit;         // Set when hovered the drop rect (vs. implicit SplitDir==None when hovered the window)
    pub IsSplitDirExplicit: bool,
    // ImGuiDockNode*  SplitNode;
    pub SplitNode: *mut ImGuiDockNode,
    // ImGuiDir        SplitDir;SplitRatio: c_float;
    pub SplitDir: ImGuiDir,
    pub SplitRatio: c_float,
    // ImRect          DropRectsDraw[ImGuiDir_COUNT + 1];  // May be slightly different from hit-testing drop rects used in DockNodeCalcDropRects()
    pub DropRectsDraw: [ImRect; ImGuiDir_COUNT + 1],
    
}

impl ImGuiDockPreviewData {
    // ImGuiDockPreviewData() : FutureNode(0) 
    pub fn new() -> Self
    { 
        let mut out = Self::default();
        // IsDropAllowed = IsCenterAvailable = IsSidesAvailable = IsSplitDirExplicit = false; 
        out.IsDropAllowed = false;
        out.IsCenterAvailable = false;
        out.IsSidesAvailable = false;
        out.IsSplitDirExplicit = false;
        out.SplitNode= null_mut(); 
        out.SplitDir = ImGuiDir_None; 
        out.SplitRatio = 0.0; 
        // for (let n: c_int = 0; n < DropRectsDraw.len(); n++) DropRectsDraw[n] = ImRect(f32::MAX, f32::MAX, -f32::MAX, -f32::MAX);
        for n in 0 .. out.DropRectsDraw.len() {
            out.DropRectsDraw[n] = ImRect::default()
        }
        out 
    }
}