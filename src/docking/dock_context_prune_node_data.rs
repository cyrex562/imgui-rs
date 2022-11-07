#[derive(Default, Debug, Clone, Copy)]
pub struct ImGuiDockContextPruneNodeData {
    // c_int         CountWindows, CountChildWindows, CountChildNodes;
    pub CountWindows: c_int,
    pub CountChildWIndows: c_int,
    pub CountChildNodes: c_int,
    // ImguiHandle     RootId;
    pub RootId: ImguiHandle,
}

impl ImGuiDockContextPruneNodeData {
    // ImGuiDockContextPruneNodeData() { CountWindows = CountChildWindows = CountChildNodes = 0; RootId = 0; }
    pub fn new() -> Self {
        Self::default()
    }
}
