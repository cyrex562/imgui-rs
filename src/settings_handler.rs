#![allow(non_snake_case)]

use crate::axis::ImGuiAxis;
use crate::context::ImguiContext;
use crate::text_buffer::ImGuiTextBuffer;
use crate::type_defs::ImguiHandle;
use libc::{c_char, c_void};

#[derive(Default, Debug, Clone)]
pub struct SettingsHandler {
    pub TypeName: String,
    // Short description stored in .ini file. Disallowed characters: '[' ']'
    pub TypeHash: ImguiHandle,
    // == ImHashStr(TypeName)
    pub UserData: Vec<u8>,

    // ImGuiSettingsHandler() { memset(this, 0, sizeof(*this)); }
    // c_void        (*ClearAllFn)(g: &mut ImguiContext, *mut handler: ImGuiSettingsHandler);                                // Clear all settings data
    pub ClearAllFn: fn(g: &mut ImguiContext, handler: *mut Self),

    // c_void        (*ReadInitFn)(g: &mut ImguiContext, *mut handler: ImGuiSettingsHandler);                                // Read: Called before reading (in registration order)
    pub ReadInitFn: fn(g: &mut ImguiContext, handler: *mut Self),

    // *mut c_void       (*ReadOpenFn)(g: &mut ImguiContext, *mut handler: ImGuiSettingsHandler, name: *const c_char);              // Read: Called when entering into a new ini entry e.g. "[Window][Name]"
    pub ReadOpenFn: fn(g: &mut ImguiContext, handler: *mut Self, name: *const c_char),

    // c_void        (*ReadLineFn)(g: &mut ImguiContext, *mut handler: ImGuiSettingsHandler, entry: *mut c_void, line: *const c_char); // Read: Called for every line of text within an ini entry
    pub ReadLineFn:
        fn(g: &mut ImguiContext, handler: *mut Self, entry: *mut c_void, line: *const c_char),

    // c_void        (*ApplyAllFn)(g: &mut ImguiContext, *mut handler: ImGuiSettingsHandler);                                // Read: Called after reading (in registration order)
    pub ApplyAllFn: fn(g: &mut ImguiContext, handler: *mut Self),

    // c_void        (*WriteAllFn)(g: &mut ImguiContext, *mut handler: ImGuiSettingsHandler, *mut ImGuiTextBuffer out_bu0f32);      // Write: Output every entries into 'out_buf'
    pub WriteAllFn: fn(g: &mut ImguiContext, handler: *mut Self, out_buf: *mut ImGuiTextBuffer),
}

impl SettingsHandler {}
