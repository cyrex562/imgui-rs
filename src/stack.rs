use crate::types::Id32;

#[derive(Default,Debug,Clone)]
struct ImGuiStackLevelInfo
{
    //ImGuiID                 id;
    pub id: Id32,
    //ImS8                    QueryFrameCount;            // >= 1: Query in progress
    pub query_frame_count: i8,
    // bool                    QuerySuccess;               // Obtained result from debug_hook_id_info()
    pub query_success: bool,
    // ImGuiDataType           DataType : 8;
    pub data_type: DimgDataType,
    // char                    Desc[57];                   // Arbitrarily sized buffer to hold a result (FIXME: could replace Results[] with a chunk stream?) FIXME: Now that we added CTRL+C this should be fixed.
    pub desc: String,
    // ImGuiStackLevelInfo()   { memset(this, 0, sizeof(*this)); }
}

// state for Stack tool queries
#[derive(Default,Debug,Clone)]
pub struct StackTool
{
    // int                     LastActiveFrame;
    pub last_active_frame: i32,
    // int                     StackLevel;                 // -1: query stack and resize Results, >= 0: individual stack level
    pub stack_level: i32,
    // ImGuiID                 QueryId;                    // id to query details for
    pub query_id: Id32,
    // ImVector<ImGuiStackLevelInfo> Results;
    pub results: Vec<DimgStackLevelInfo>,
    // bool                    CopyToClipboardOnCtrlC;
    pub copy_to_clopboard_on_ctrl_c: bool,
    // float                   CopyToClipboardLastTime;
    pub copy_to_clipboard_last_time: f32,
    // ImGuiStackTool()        { memset(this, 0, sizeof(*this)); CopyToClipboardLastTime = -FLT_MAX; }
}

#[derive(Debug,Default,Clone)]
pub struct  ImGuiStackSizes
{
    // short   SizeOfIDStack;
    pub SizeofIDStack: i16,
    // short   SizeOfColorStack;
    pub SizeOfColorStack: i16,
    // short   SizeOfStyleVarStack;
    pub SizeOfStyleVarStack: i16,
    // short   SizeOfFontStack;
    pub SizeOfFontStack: i16,
    // short   SizeOfFocusScopeStack;
    pub SizeOfFocusScopeStack: i16,
    // short   SizeOfGroupStack;
    pub SizeOfGroupStack: i16,
    // short   SizeOfItemFlagsStack;
    pub SizeOfItemFlagsStack: i16,
    // short   SizeOfBeginPopupStack;
    pub SizeOfBeginPopupStack: i16,
    // short   SizeOfDisabledStack;
    pub SizeOfDisabledStack: i16,
}

impl ImGuiStackSizes {
    // ImGuiStackSizes() { memset(this, 0, sizeof(*this)); }
    pub fn new()-> Self {
        Self {
            ..Default::default()
        }
    }
    //     void SetToCurrentState();
    pub fn SetToCurrentState(&mut self) {
        todo!()
    }
    //     void CompareWithCurrentState();
    pub fn CompareWithCurrentState(&self) {

    }
}
