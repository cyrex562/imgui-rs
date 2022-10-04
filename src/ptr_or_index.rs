#![allow(non_snake_case)]

use std::ptr::null_mut;
use libc::{c_int, c_void};

#[derive(Default,Debug,Clone)]
pub struct ImGuiPtrOrIndex
{
pub Ptr: *mut c_void,            // Either field can be set, not both. e.g. Dock node tab bars are loose while BeginTabBar() ones are in a pool.
pub Index:  c_int,          // Usually index in a main pool.


}

impl ImGuiPtrOrIndex {
    // ImGuiPtrOrIndex(*mut void ptr)  { Ptr = ptr; Index = -1; }
    pub fn new(ptr: *mut c_void) -> Self {
        Self {
            Ptr: ptr,
            Index: -1
        }
    }

    // ImGuiPtrOrIndex(index: c_int)  { Ptr = None; Index = index; }
    pub fn new2(index:c_int)  -> Self {
        Self {
            Ptr: null_mut(),
            Index: index
        }
    }
}
