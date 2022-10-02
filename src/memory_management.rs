

// IM_ALLOC() == MemAlloc()
*mut c_void MemAlloc(size_t size)
{
    if (ImGuiContext* ctx = GImGui)
        ctx.IO.MetricsActiveAllocations+= 1;
    return (*GImAllocatorAllocFunc)(size, GImAllocatorUserData);
}

// IM_FREE() == MemFree()
c_void MemFree(ptr: *mut c_void)
{
    if (ptr)
        if (ImGuiContext* ctx = GImGui)
            ctx->IO.MetricsActiveAllocations-= 1;
    return (*GImAllocatorFreeFunc)(ptr, GImAllocatorUserData);
}
