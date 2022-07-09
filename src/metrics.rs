impl DimgMetricsConfig {
    // ImGuiMetricsConfig()
    pub fn new() -> Self
    {
        // ShowDebugLog = ShowStackTool = ShowWindowsRects = ShowWindowsBeginOrder = ShowTablesRects = false;
        // ShowDrawCmdMesh = true;
        // ShowDrawCmdBoundingBoxes = true;
        // ShowDockingNodes = false;
        // ShowWindowsRectsType = ShowTablesRectsType = -1;
        Self {
            show_debug_log: false,
            show_stack_tool: false,
            show_windows_rects: false,
            show_windows_begin_order: false,
            show_tables_rects: false,
            show_draw_cmd_mesh: true,
            show_draw_cmd_bounding_boxes: true,
            show_docking_nodes: false,
            show_windows_rects_type: -1,
            show_tables_rects_type: -1
        }
    }
}


#[derive(Default,Debug,Clone)]
pub struct DimgMetricsConfig
{
    //bool        ShowDebugLog;
    pub show_debug_log: bool,
    // bool        ShowStackTool;
    pub show_stack_tool: bool,
    //bool        ShowWindowsRects;
    pub show_windows_rects: bool,
    //bool        ShowWindowsBeginOrder;
    pub show_windows_begin_order: bool,
    // bool        ShowTablesRects;
    pub show_tables_rects: bool,
    // bool        ShowDrawCmdMesh;
    pub show_draw_cmd_mesh: bool,
    // bool        ShowDrawCmdBoundingBoxes;
    pub show_draw_cmd_bounding_boxes: bool,
    // bool        ShowDockingNodes;
    pub show_docking_nodes: bool,
    // int         ShowWindowsRectsType;
    pub show_windows_rects_type: i32,
    // int         ShowTablesRectsType;
    pub show_tables_rects_type: i32,
}
