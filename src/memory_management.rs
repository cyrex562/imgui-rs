

// IM_ALLOC() == MemAlloc()
*mut c_void MemAlloc(size: size_t)
{
    if (ImGuiContext* ctx = GImGui)
        ctx.IO.MetricsActiveAllocations+= 1;
    return (*GImAllocatorAllocFunc)(size, GImAllocatorUserData);
}

// IM_FREE() == MemFree()
pub unsafe fn MemFree(ptr: *mut c_void)
{
    if (ptr)
        if (ImGuiContext* ctx = GImGui)
            ctx->IO.MetricsActiveAllocations-= 1;
    return (*GImAllocatorFreeFunc)(ptr, GImAllocatorUserData);
}
