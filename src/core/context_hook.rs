use crate::core::context::ImguiContext;
use crate::type_defs::ImguiHandle;
use libc::{c_int, c_void};

pub type ImguiContextHookType = c_int;

pub const IM_GUI_CONTEXT_HOOK_TYPE_NEW_FRAME_PRE: ImguiContextHookType = 0;
pub const IM_GUI_CONTEXT_HOOK_TYPE_NEW_FRAME_POST: ImguiContextHookType = 1;
pub const IM_GUI_CONTEXT_HOOK_TYPE_END_FRAME_PRE: ImguiContextHookType = 2;
pub const IM_GUI_CONTEXT_HOOK_TYPE_END_FRAME_POST: ImguiContextHookType = 3;
pub const IM_GUI_CONTEXT_HOOK_TYPE_RENDER_PRE: ImguiContextHookType = 4;
pub const IM_GUI_CONTEXT_HOOK_TYPE_RENDER_POST: ImguiContextHookType = 5;
pub const IM_GUI_CONTEXT_HOOK_TYPE_SHUTDOWN: ImguiContextHookType = 6;
pub const IM_GUI_CONTEXT_HOOK_TYPE_PENDING_REMOVAL: ImguiContextHookType = 7;

pub type ImGuiContextHookCallback = fn(g: &mut ImguiContext, hook: &mut ImGuiContextHook);

#[derive(Default, Debug, Clone)]
pub struct ImGuiContextHook {
    pub HookId: ImguiHandle,
    // A unique ID assigned by AddContextHook()
    pub hook_type: ImguiContextHookType,
    pub Owner: ImguiHandle,
    pub Callback: ImGuiContextHookCallback,
    pub UserData: Vec<u8>,
}
