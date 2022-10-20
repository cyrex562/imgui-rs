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
    // c_void        (*ClearAllFn)(ctx: *mut ImGuiContext, *mut handler: ImGuiSettingsHandler);                                // Clear all settings data
    pub ClearAllFn: fn(ctx: *mut ImGuiContext, handler: *mut Self),

    // c_void        (*ReadInitFn)(ctx: *mut ImGuiContext, *mut handler: ImGuiSettingsHandler);                                // Read: Called before reading (in registration order)
    pub ReadInitFn: fn(ctx: *mut ImGuiContext, handler: *mut Self),

    // *mut c_void       (*ReadOpenFn)(ctx: *mut ImGuiContext, *mut handler: ImGuiSettingsHandler, name: *const c_char);              // Read: Called when entering into a new ini entry e.g. "[Window][Name]"
    pub ReadOpenFn: fn(ctx: *mut ImGuiContext, handler: *mut Self, name: *const c_char),

    // c_void        (*ReadLineFn)(ctx: *mut ImGuiContext, *mut handler: ImGuiSettingsHandler, entry: *mut c_void, line: *const c_char); // Read: Called for every line of text within an ini entry
    pub ReadLineFn: fn(ctx: *mut ImGuiContext, handler: *mut Self, entry: *mut c_void, line: *const c_char),

    // c_void        (*ApplyAllFn)(ctx: *mut ImGuiContext, *mut handler: ImGuiSettingsHandler);                                // Read: Called after reading (in registration order)
    pub ApplyAllFn: fn(ctx: *mut ImGuiContext, handler: *mut Self),

    // c_void        (*WriteAllFn)(ctx: *mut ImGuiContext, *mut handler: ImGuiSettingsHandler, *mut ImGuiTextBuffer out_bu0f32);      // Write: Output every entries into 'out_buf'
    pub WriteAllFn: fn(ctx: *mut ImGuiContext, handler: *mut Self, out_buf: *mut ImGuiTextBuffer),
}

impl ImGuiSettingsHandler {}
