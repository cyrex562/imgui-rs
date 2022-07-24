use crate::rect::Rect;
use crate::types::{Direction, Id32};

#[derive(Debug,Default,Clone)]
pub struct DockPreviewData
{
    // ImGuiDockNode   FutureNode;
    pub future_node: DimgDockNode,
    // bool            IsDropAllowed;
    pub is_drop_allowed: bool,
    // bool            IsCenterAvailable;
    pub is_center_available: bool,
    // bool            IsSidesAvailable;           // Hold your breath, grammar freaks..
    pub is_sides_available: bool,
    // bool            IsSplitDirExplicit;         // Set when hovered the drop rect (vs. implicit SplitDir==None when hovered the window)
    pub is_split_dir_explicit: bool,
    // ImGuiDockNode*  SplitNode;
    pub split_node: Id32,
    // ImGuiDir        SplitDir;
    pub split_dir: Direction,
    // float           SplitRatio;
    pub split_ratio: f32,
    // ImRect          drop_rects_draw[ImGuiDir_COUNT + 1];  // May be slightly different from hit-testing drop rects used in DockNodeCalcDropRects()
    pub drop_rects_draw: [Rect; 5 ],
}
