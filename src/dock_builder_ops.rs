// inline * mut ImGuiDockNode   DockBuilderGetCentralNode(ImGuiID node_id)              
pub fn DockBuilderGetCentralNode(node_id: ImGuiID) -> *mut ImGuiDockNode
{ 
    let mut node = DockBuilderGetNode(node_id); 
    if !node {return null_mut();} 
    return DockNodeGetRootNode(node).CentralNode; 
}