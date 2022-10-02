#![allow(non_snake_case)]

use crate::window::ImGuiWindow;

// c_void GcCompactTransientMiscBuffers()
pub fn GcCompatTransientMiscBuffers()
{
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.ItemFlagsStack.clear();
    g.GroupStack.clear();
    TableGcCompactSettings();
}

// Free up/compact internal window buffers, we can use this when a window becomes unused.
// Not freed:
// - ImGuiWindow, ImGuiWindowSettings, Name, StateStorage, ColumnsStorage (may hold useful data)
// This should have no noticeable visual effect. When the window reappear however, expect new allocation/buffer growth/copy cost.
// c_void GcCompactTransientWindowBuffers(ImGuiWindow* window)
pub fn GcCompactTransientWindowBuffers(window: *mut ImGuiWindow)
{
    window.MemoryCompacted = true;
    window.MemoryDrawListIdxCapacity = window.DrawList.IdxBuffer.Capacity;
    window.MemoryDrawListVtxCapacity = window.DrawList.VtxBuffer.Capacity;
    window.IDStack.clear();
    window.DrawList._ClearFreeMemory();
    window.DC.ChildWindows.clear();
    window.DC.ItemWidthStack.clear();
    window.DC.TextWrapPosStack.clear();
}

// c_void GcAwakeTransientWindowBuffers(ImGuiWindow* window)
pub fn GcAwakeTransientWindowBuffers(window: *mut ImGuiWindow)
{
    // We stored capacity of the ImDrawList buffer to reduce growth-caused allocation/copy when awakening.
    // The other buffers tends to amortize much faster.
    window.MemoryCompacted = false;
    window.DrawList.IdxBuffer.reserve(window.MemoryDrawListIdxCapacity as usize);
    window.DrawList.VtxBuffer.reserve(window.MemoryDrawListVtxCapacity as usize);
    window.MemoryDrawListIdxCapacity = 0;
    window.MemoryDrawListVtxCapacity = 0;
}
