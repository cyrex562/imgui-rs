#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiModFlags;          // -> enum ImGuiModFlags_        // Flags: for io.KeyMods (Ctrl/Shift/Alt/Super)
pub type ImGuiModFlags = c_int;


// Helper "flags" version of key-mods to store and compare multiple key-mods easily. Sometimes used for storage (e.g. io.KeyMods) but otherwise not much used in public API.
// enum ImGuiModFlags_
// {
pub const ImGuiModFlags_None: ImGuiModFlags = 0;
pub const ImGuiModFlags_Ctrl: ImGuiModFlags = 1 << 0;
pub const ImGuiModFlags_Shift: ImGuiModFlags = 1 << 1;
pub const ImGuiModFlags_Alt: ImGuiModFlags = 1 << 2;
// Option/Menu key
pub const ImGuiModFlags_Super: ImGuiModFlags = 1 << 3;
// Cmd/Super/Windows key
pub const ImGuiModFlags_All: ImGuiModFlags = 0x0F;
// };
