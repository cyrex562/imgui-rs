#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use crate::context::ImGuiContext;
use crate::type_defs::ImGuiID;
use libc::{c_int, c_void};

pub type ImGuiContextHookType = c_int;

// enum ImGuiContextHookType {
pub const ImGuiContextHookType_NewFramePre: ImGuiContextHookType = 0;
pub const ImGuiContextHookType_NewFramePost: ImGuiContextHookType = 1;
pub const ImGuiContextHookType_EndFramePre: ImGuiContextHookType = 2;
pub const ImGuiContextHookType_EndFramePost: ImGuiContextHookType = 3;
pub const ImGuiContextHookType_RenderPre: ImGuiContextHookType = 4;
pub const ImGuiContextHookType_RenderPost: ImGuiContextHookType = 5;
pub const ImGuiContextHookType_Shutdown: ImGuiContextHookType = 6;
pub const ImGuiContextHookType_PendingRemoval_: ImGuiContextHookType = 7;
// };

// typedef c_void (*ImGuiContextHookCallback)(*mut ImGuiContext ctx, *mut ImGuiContextHook hook);
pub type ImGuiContextHookCallback = fn(ctx: *mut ImGuiContext, hook: *mut ImGuiContextHook);

#[derive(Default, Debug, Clone)]
pub struct ImGuiContextHook {
    pub HookId: ImGuiID,
    // A unique ID assigned by AddContextHook()
    pub Type: ImGuiContextHookType,
    pub Owner: ImGuiID,
    pub Callback: ImGuiContextHookCallback,
    pub UserData: *mut c_void,
    // ImGuiContextHook()          { memset(this, 0, sizeof(*this)); }
}
