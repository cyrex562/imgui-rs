use libc::{c_float, c_int};
use crate::stack_level_info::ImGuiStackLevelInfo;
use crate::type_defs::ImGuiID;

// State for Stack tool queries
#[derive(Default,Debug,Clone)]
pub struct ImGuiStackTool
{
pub LastActiveFrame:  c_int,
pub StackLevel:  c_int,                 // -1: query stack and resize Results, >= 0: individual stack level
pub QueryId:  ImGuiID,                    // ID to query details for
pub Results:  Vec<ImGuiStackLevelInfo>,
pub CopyToClipboardOnCtrlC:  bool,
pub CopyToClipboardLastTime:  c_float,

    
}

impl ImGuiStackTool {
//     ImGuiStackTool()        { memset(this, 0, sizeof(*this)); CopyToClipboardLastTime = -f32::MAX; }
    pub fn new() -> Self {
    Self {
        CopyToClipboardLastTime: f32::MIN,
        ..Default::default()
    }
}
}
