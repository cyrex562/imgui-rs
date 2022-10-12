#![allow(non_upper_case_globals)]

use std::ptr::null_mut;
use libc::c_void;
use crate::context::ImGuiContext;

// ImGuiContext*   GImGui = None;
pub static mut GImGui: *mut ImGuiContext = null_mut();

// static void*                GImAllocatorUserData = None;
pub static mut GImAllocatorUserData: *mut c_void = null_mut();