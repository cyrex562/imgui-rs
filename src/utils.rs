#![allow(non_snake_case)]

use std::ptr::null_mut;
use libc::{c_char, c_int, c_void, size_t};
use crate::context::ImGuiContext;
use crate::context_hook::{ImGuiContextHook, ImGuiContextHookType, ImGuiContextHookType_PendingRemoval_};
use crate::font_atlas::ImFontAtlas;
use crate::imgui::{GImAllocatorUserData, GImGui};
use crate::type_defs::{ImGuiID, ImGuiMemAllocFunc, ImGuiMemFreeFunc};

// Memory Allocator functions. Use SetAllocatorFunctions() to change them.
// - You probably don't want to modify that mid-program, and if you use global/static e.g. ImVector<> instances you may need to keep them accessible during program destruction.
// - DLL users: read comments above.
// #ifndef IMGUI_DISABLE_DEFAULT_ALLOCATORS
// static void*   MallocWrapper(size_t size, void* user_data)    { IM_UNUSED(user_data); return malloc(size); }
pub unsafe fn MallocWrapper(size: &usize, mut user_data: *mut u8) -> *mut u8 {
    user_data = libc::malloc(size as libc::size_t) as *mut u8;
    return user_data;
}

// static void    FreeWrapper(void* ptr, void* user_data)        { IM_UNUSED(user_data); free(ptr); }
pub unsafe fn FreeWrapper<T>(ptr: T) {
    libc::free(ptr);
}

// static ImGuiMemAllocFunc    GImAllocatorAllocFunc = MallocWrapper;
pub type GImAllocatorAllocFunc = MallocWrapper;
// static ImGuiMemFreeFunc     GImAllocatorFreeFunc = FreeWrapper;
pub type GImAllocatorFreeFunc = FreeWrapper;



// *const char GetVersion()
pub fn GetVersion() -> *const c_char
{
    return IMGUI_VERSION;
}


// c_void SetAllocatorFunctions(ImGuiMemAllocFunc alloc_func, ImGuiMemFreeFunc free_func, user_data: *mut c_void)
pub unsafe fn SetAllocatorFunctions(alloc_func: ImGuiMemAllocFunc, free_func: ImGuiMemFreeFunc, user_data: *mut c_void) {
    GImAllocatorAllocFunc = alloc_func;
    GImAllocatorFreeFunc = free_func;
    GImAllocatorUserData = user_data;
}

// This is provided to facilitate copying allocators from one static/DLL boundary to another (e.g. retrieve default allocator of your executable address space)
// c_void GetAllocatorFunctions(ImGuiMemAllocFunc* p_alloc_func, ImGuiMemFreeFunc* p_free_func, c_void** p_user_data)
pub unsafe fn GetAllocatorFunctions(p_alloc_func: *mut ImGuiMemAllocFunc, p_free_func: *mut ImGuiMemFreeFunc, p_user_data: *mut *mut c_void)
{
    *p_alloc_func = GImAllocatorAllocFunc;
    *p_free_func = GImAllocatorFreeFunc;
    *p_user_data = GImAllocatorUserData;
}

pub fn flag_set<T>(flags: T, flag: T) -> bool {
    flags & flag == 1
}

pub fn flag_clear<T>(flags: T, flag: T) -> bool {
    flag_set(flags, flag) == false
}

// static inline c_void      ImQsort(*mut c_void base, size_t count, size_t size_of_element, c_int(IMGUI_CDECL *compare_func)(c_void *mut const, c_void *mut const)) 
pub fn ImQsort(
    base: *mut c_void, 
    count: size_t, size_of_element: size_t, 
    compare_fn: fn(a: *const c_void, b: *const c_void) -> c_int)
{ 
if count > 1 { libc::qsort(base, count, size_of_element, compare_func); }
}


pub fn is_null<T>(pointer: *T) -> bool
{
    pointer.is_null()
}

pub fn is_not_null<T>(pointer: *T) -> bool {
    pointer.is_null() == false
}
