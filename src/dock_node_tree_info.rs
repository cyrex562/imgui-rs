use crate::dock_node::ImGuiDockNode;
use crate::window::window_class::ImGuiWindowClass;
use libc::size_t;

// Search function called once by root node in DockNodeUpdate()
#[derive(Default, Debug, Copy, Clone)]
pub struct ImGuiDockNodeTreeInfo {
    // ImGuiDockNode*      CentralNode;
    pub CentralNode: *mut ImGuiDockNode,
    // ImGuiDockNode*      FirstNodeWithWindows;
    pub FirstNodeWithWindows: *mut ImGuiDockNode,
    // c_int                 CountNodesWithWindows;
    pub CountNodesWithWindows: size_t,
    //ImGuiWindowClass  WindowClassForMerges;
    pub WindowClassForMerges: ImGuiWindowClass,
    // ImGuiDockNodeTreeInfo() { memset(this, 0, sizeof(*this)); }
}
