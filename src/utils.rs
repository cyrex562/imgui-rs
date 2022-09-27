#![allow(non_snake_case)]

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
