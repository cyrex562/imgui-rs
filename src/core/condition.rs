#![allow(non_upper_case_globals)]

use libc::c_int;

// typedef int ImGuiCond;              // -> enum ImGuiCond_            // Enum: A condition for many Set*() functions
pub type ImGuiCond = c_int;

// Enumeration for SetWindow***(), SetNextWindow***(), SetNextItem***() functions
// Represent a condition.
// Important: Treat as a regular enum! Do NOT combine multiple values using binary operators! All the functions above treat 0 as a shortcut to ImGuiCond_Always.
// enum ImGuiCond_
// {
pub const ImGuiCond_None: ImGuiCond = 0; // No condition (always set the variable); same as _Always
pub const ImGuiCond_Always: ImGuiCond = 1 << 0; // No condition (always set the variable); same as _None
pub const ImGuiCond_Once: ImGuiCond = 1 << 1; // Set the variable once per runtime session (only the first call will succeed)
pub const ImGuiCond_FirstUseEver: ImGuiCond = 1 << 2; // Set the variable if the object/window has no persistently saved data (no entry in .ini file)
pub const ImGuiCond_Appearing: ImGuiCond = 1 << 3; // Set the variable if the object/window is appearing after being hidden/inactive (or the first time)
                                                   // };
