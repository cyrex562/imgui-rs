#![allow(non_snake_case)]

use crate::data_type::ImGuiDataType;
use crate::type_defs::ImguiHandle;
use libc::c_char;

#[derive(Default, Debug, Clone)]
pub struct ImGuiStackLevelInfo {
    pub ID: ImguiHandle,
    pub QueryFrameCount: i8,
    // >= 1: Query in progress
    pub QuerySuccess: bool,
    // Obtained result from DebugHookIdInfo()
    pub DataType: ImGuiDataType,
    pub Desc: [c_char; 57], // Arbitrarily sized buffer to hold a result (FIXME: could replace Results[] with a chunk stream?) FIXME: Now that we added CTRL+C this should be fixed.

                            // ImGuiStackLevelInfo()   { memset(this, 0, sizeof(*this)); }
}
