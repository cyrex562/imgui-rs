#![allow(non_snake_case)]

use libc::{c_char, c_void};
use crate::axis::ImGuiAxis;
use crate::context::ImGuiContext;
use crate::text_buffer::ImGuiTextBuffer;
use crate::type_defs::ImGuiID;

#[derive(Default, Debug, Clone)]
pub struct ImGuiSettingsHandler {
    pub TypeName: *const char,
    // Short description stored in .ini file. Disallowed characters: '[' ']'
    pub TypeHash: ImGuiID,
    // == ImHashStr(TypeName)
    pub UserData: *mut c_void,

    // ImGuiSettingsHandler() { memset(this, 0, sizeof(*this)); }
    // c_void        (*ClearAllFn)(*mut ImGuiContext ctx, *mut ImGuiSettingsHandler handler);                                // Clear all settings data
    pub ClearAllFn: fn(ctx: *mut ImGuiContext, handler: *mut Self),

    // c_void        (*ReadInitFn)(*mut ImGuiContext ctx, *mut ImGuiSettingsHandler handler);                                // Read: Called before reading (in registration order)
    pub ReadInitFn: fn(ctx: *mut ImGuiContext, handler: *mut Self),

    // *mut c_void       (*ReadOpenFn)(*mut ImGuiContext ctx, *mut ImGuiSettingsHandler handler, name: *const c_char);              // Read: Called when entering into a new ini entry e.g. "[Window][Name]"
    pub ReadOpenFn: fn(ctx: *mut ImGuiContext, handler: *mut Self, name: *const c_char),

    // c_void        (*ReadLineFn)(*mut ImGuiContext ctx, *mut ImGuiSettingsHandler handler, *mut c_void entry, line: *const c_char); // Read: Called for every line of text within an ini entry
    pub ReadLineFn: fn(ctx: *mut ImGuiContext, handler: *mut Self, entry: *mut c_void, line: *const c_char),

    // c_void        (*ApplyAllFn)(*mut ImGuiContext ctx, *mut ImGuiSettingsHandler handler);                                // Read: Called after reading (in registration order)
    pub ApplyAllFn: fn(ctx: *mut ImGuiContext, handler: *mut Self)

    // c_void        (*WriteAllFn)(*mut ImGuiContext ctx, *mut ImGuiSettingsHandler handler, *mut ImGuiTextBuffer out_bu0f32);      // Write: Output every entries into 'out_buf'
    pub WriteAllFn: fn(ctx: *mut ImGuiContext, handler: *mut Self, out_buf: *mut ImGuiTextBuffer),
}

impl ImGuiSettingsHandler {}
