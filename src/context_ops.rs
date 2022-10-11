#![allow(non_snake_case)]

use crate::context::ImGuiContext;
use crate::context_hook::{
    ImGuiContextHook, ImGuiContextHookType, ImGuiContextHookType_PendingRemoval_,
};
use crate::font_atlas::ImFontAtlas;
use crate::imgui::GImGui;
use crate::platform_io::ImGuiPlatformIO;
use crate::type_defs::ImGuiID;
use crate::{Initialize, Shutdown};
use libc::{c_double, c_int};
use std::ptr::null_mut;

// Internal state access - if you want to share Dear ImGui state between modules (e.g. DLL) or allocate it yourself
// Note that we still point to some static data and members (such as GFontAtlas), so the state instance you end up using will point to the static data within its module
// ImGuiContext* GetCurrentContext()
pub unsafe fn GetCurrentContext() -> *mut ImGuiContext {
    return GImGui;
}

// c_void SetCurrentContext(ImGuiContext* ctx)
pub fn SetCurrentContext(ctx: *mut ImGuiContext) {
    // #ifdef IMGUI_SET_CURRENT_CONTEXT_FUNC
    IMGUI_SET_CURRENT_CONTEXT_FUNC(ctx); // For custom thread-based hackery you may want to have control over this.
                                         // #else
                                         //     GImGui = ctx;
                                         // #endif
}

// ImGuiContext* CreateContext(shared_font_atlas: *mut ImFontAtlas)
pub unsafe fn CreateContext(shared_font_atlas: *mut ImFontAtlas) -> *mut ImGuiContext {
    // ImGuiContext* prev_ctx = GetCurrentContext();
    let mut prev_ctx = GetCurrentContext();
    let mut ctx = ImGuiContext::new(shared_font_atlas);
    SetCurrentContext(&mut ctx);
    Initialize();
    if prev_ctx != null_mut() {
        SetCurrentContext(prev_ctx);
    } // Restore previous context if any, else keep new one.
    return &mut ctx;
}

// c_void DestroyContext(ImGuiContext* ctx)
pub unsafe fn DestroyContext(mut ctx: *mut ImGuiContext) {
    let mut prev_ctx = GetCurrentContext();
    if ctx == null_mut() {
        //-V1051
        ctx = prev_ctx;
    }
    SetCurrentContext(ctx);
    Shutdown();
    SetCurrentContext(if prev_ctx != ctx {
        prev_ctx
    } else {
        null_mut()
    });
    IM_DELETE(ctx);
}

// No specific ordering/dependency support, will see as needed
// ImGuiID AddContextHook(ImGuiContext* ctx, *const ImGuiContextHook hook)
pub unsafe fn AddContextHook(ctx: *mut ImGuiContext, hook: *const ImGuiContextHook) -> ImGuiID {
    // ImGuiContext& g = *ctx;
    let g = ctx;
    // IM_ASSERT(hook->Callback != NULL && hook->HookId == 0 && hook->Type != ImGuiContextHookType_PendingRemoval_);
    g.Hooks.push((*hook).clone());
    g.HookIdNext += 1;
    g.Hooks.last_mut().unwrap().HookId = g.HookIdNext.last().unwrap().clone();
    return g.HookIdNext.last().unwrap().clone();
}

// Deferred removal, avoiding issue with changing vector while iterating it
// c_void RemoveContextHook(ImGuiContext* ctx, ImGuiID hook_id)
pub fn RemoveContextHook(ctx: *mut ImGuiContext, hook_id: ImGuiID) {
    // ImGuiContext& g = *ctx;
    let g = ctx;
    // IM_ASSERT(hook_id != 0);
    // for (let n: c_int = 0; n < g.Hooks.Size; n++)
    for n in 0..g.Hooks.len() {
        if g.Hooks[n].HookId == hook_id {
            g.Hooks[n].Type = ImGuiContextHookType_PendingRemoval_;
        }
    }
}

// Call context hooks (used by e.g. test engine)
// We assume a small number of hooks so all stored in same array
// c_void CallContextHooks(ImGuiContext* ctx, ImGuiContextHookType hook_type)
pub fn CallContextHooks(ctx: *mut ImGuiContext, hook_type: ImGuiContextHookType) {
    // ImGuiContext& g = *ctx;
    let g = ctx;
    // for (let n: c_int = 0; n < g.Hooks.Size; n++)
    for n in 0..g.Hooks.len() {
        if g.Hooks[n].Type == hook_type {
            g.Hooks[n].Callback(&g, &g.Hooks[n]);
        }
    }
}

// ImGuiPlatformIO& GetPlatformIO()
pub fn GetPlatformIO() -> &mut ImGuiPlatformIO {
    // IM_ASSERT(GImGui != NULL && "No current context. Did you call CreateContext() or SetCurrentContext()?");
    return GimGui.PlatformIO;
}

// double GetTime()
fn GetTime() -> c_double {
    return GimGui.Time;
}

// GetFrameCount: c_int()
pub fn GetFrameCount() -> c_int {
    return GimGui.FrameCount;
}
