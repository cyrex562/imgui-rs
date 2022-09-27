#![allow(non_upper_case_globals)]

use std::ptr::null_mut;
use crate::context::ImGuiContext;

// ImGuiContext*   GImGui = None;
pub static GImGui: *mut ImGuiContext = null_mut();

// static void*                GImAllocatorUserData = None;
pub static GImAllocatorUserData: *mut u8 = null_mut();