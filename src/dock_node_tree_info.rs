use libc::c_int;
use crate::dock_node::ImGuiDockNode;

// Search function called once by root node in DockNodeUpdate()
#[derive(Default,Debug,Copy,Clone)]
pub struct ImGuiDockNodeTreeInfo
{
    // ImGuiDockNode*      CentralNode;
    pub CentralNode: *mut ImGuiDockNode,

    // ImGuiDockNode*      FirstNodeWithWindows;
    pub FirstNodeWithWindows: *mut ImGuiDockNode,

    // c_int                 CountNodesWithWindows;
    pub CountNodesWithWindows: c_int,

    //ImGuiWindowClass  WindowClassForMerges;

    // ImGuiDockNodeTreeInfo() { memset(this, 0, sizeof(*this)); }
}
