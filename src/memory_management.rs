// // IM_ALLOC() == MemAlloc()
// MemAlloc: *mut c_void(size: size_t)
// {
//     if (g: &mut ImguiContext = GImGui)
//         ctx.IO.MetricsActiveAllocations+= 1;
//     return (*GImAllocatorAllocFunc)(size, GImAllocatorUserData);
// }
//
// // IM_FREE() == MemFree()
// pub unsafe fn MemFree(ptr: *mut c_void)
// {
//     if (ptr)
//         if (g: &mut ImguiContext = GImGui)
//             ctx->IO.MetricsActiveAllocations-= 1;
//     return (*GImAllocatorFreeFunc)(ptr, GImAllocatorUserData);
// }
