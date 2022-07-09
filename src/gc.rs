use crate::context::DimgContext;
use crate::window::DimgWindow;

// void ImGui::GcCompactTransientMiscBuffers()
pub fn GcCompactTransientMiscBuffers(g: &mut DimgContext)
{
    // ImGuiContext& g = *GImGui;
    g.item_flags_stack.clear();
    g.group_stack.clear();
    TableGcCompactSettings();
}


// Free up/compact internal window buffers, we can use this when a window becomes unused.
// Not freed:
// - ImGuiWindow, ImGuiWindowSettings, name, state_storage, ColumnsStorage (may hold useful data)
// This should have no noticeable visual effect. When the window reappear however, expect new allocation/buffer growth/copy cost.
// void ImGui::GcCompactTransientWindowBuffers(ImGuiWindow* window)
pub fn GcCompactTransientWindowBufufers(window: &mut DimgWindow)
{
    window.memory_compacted = true;
    window.memory_draw_list_idx_capacity = window.draw_list.IdxBuffer.Capacity;
    window.memory_draw_list_vtx_capacity = window.draw_list.VtxBuffer.Capacity;
    window.id_stack.clear();
    window.draw_list._ClearFreeMemory();
    window.dc.ChildWindows.clear();
    window.dc.ItemWidthStack.clear();
    window.dc.TextWrapPosStack.clear();
}

// void ImGui::GcAwakeTransientWindowBuffers(ImGuiWindow* window)
pub fn GcAwakeTransientWindowBuffers(window: &mut DimgWindow)
{
    // We stored capacity of the ImDrawList buffer to reduce growth-caused allocation/copy when awakening.
    // The other buffers tends to amortize much faster.
    window.memory_compacted = false;
    window.draw_list.IdxBuffer.reserve(window.memory_draw_list_idx_capacity);
    window.draw_list.VtxBuffer.reserve(window.memory_draw_list_vtx_capacity);
    window.memory_draw_list_idx_capacity = 0;
    window.memory_draw_list_vtx_capacity = 0;
}
