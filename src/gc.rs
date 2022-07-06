use crate::context::ImGuiContext;
use crate::window::ImGuiWindow;

// void ImGui::GcCompactTransientMiscBuffers()
pub fn GcCompactTransientMiscBuffers(g: &mut ImGuiContext)
{
    // ImGuiContext& g = *GImGui;
    g.ItemFlagsStack.clear();
    g.GroupStack.clear();
    TableGcCompactSettings();
}


// Free up/compact internal window buffers, we can use this when a window becomes unused.
// Not freed:
// - ImGuiWindow, ImGuiWindowSettings, Name, StateStorage, ColumnsStorage (may hold useful data)
// This should have no noticeable visual effect. When the window reappear however, expect new allocation/buffer growth/copy cost.
// void ImGui::GcCompactTransientWindowBuffers(ImGuiWindow* window)
pub fn GcCompactTransientWindowBufufers(window: &mut ImGuiWindow)
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

// void ImGui::GcAwakeTransientWindowBuffers(ImGuiWindow* window)
pub fn GcAwakeTransientWindowBuffers(window: &mut ImGuiWindow)
{
    // We stored capacity of the ImDrawList buffer to reduce growth-caused allocation/copy when awakening.
    // The other buffers tends to amortize much faster.
    window.MemoryCompacted = false;
    window.DrawList.IdxBuffer.reserve(window.MemoryDrawListIdxCapacity);
    window.DrawList.VtxBuffer.reserve(window.MemoryDrawListVtxCapacity);
    window.MemoryDrawListIdxCapacity = 0;
    window.MemoryDrawListVtxCapacity = 0;
}
