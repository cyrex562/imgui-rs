#[derive(Default,Debug,Clone,Copy)]
pub struct ImGuiDockContextPruneNodeData
{
    // c_int         CountWindows, CountChildWindows, CountChildNodes;
    pub CountWindows: c_int,
    pub CountChildWIndows: c_int,
    pub CountChildNodes: c_int,
    // ImGuiID     RootId;
    pub RootId: ImGuiID
    
}

impl ImGuiDockContextPruneNodeData {
    // ImGuiDockContextPruneNodeData() { CountWindows = CountChildWindows = CountChildNodes = 0; RootId = 0; }
    pub fn new() -> Self {
        Self::default()
    }
}