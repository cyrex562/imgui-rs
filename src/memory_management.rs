use std::os::raw::c_void;
use libc::size_t;
use crate::context::ImGuiContext;
use crate::GImGui;
use crate::imgui::GImAllocatorUserData;
use crate::utils::{GImAllocatorAllocFunc, GImAllocatorFreeFunc};

// IM_ALLOC() == MemAlloc()
// *mut c_void MemAlloc(size_t size)
pub unsafe fn MemAlloc(size: size_t) -> *mut c_void {
    let ctx = GImGui;
    if ctx.is_null() == false {
        ctx.IO.MetricsActiveAllocations += 1;
    }
    return GImAllocatorAllocFunc(size, GImAllocatorUserData);
}

// IM_FREE() == MemFree()
// c_void MemFree(ptr: *mut c_void)
pub unsafe fn MemFree(ptr: *mut c_void) {
    if ptr {
        if ctx = GImGui {
            ctx.IO.MetricsActiveAllocations -= 1;
        }
    }
    return GImAllocatorFreeFunc(ptr, GImAllocatorUserData);
}


// struct ImNewWrapper {};
// inline operator: *mut c_void new(size_t, ImNewWrapper, ptr: *mut c_void) { return ptr; }
// inline c_void  operator delete(*mut c_void, ImNewWrapper, *mut c_void)   {} // This is only required so we can use the symmetrical new()
// // #define IM_ALLOC(_SIZE)                     MemAlloc(_SIZE)
// // #define IM_FREE(_PTR)                       MemFree(_PTR)
// // #define IM_PLACEMENT_NEW(_PTR)              new(ImNewWrapper(), _PTR)
// // #define IM_NEW(_TYPE)                       new(ImNewWrapper(), MemAlloc(sizeof(_TYPE))) _TYPE
// template<typename T> c_void IM_DELETE(T* p)   { if (p) { p->~T(); MemFree(p); } }