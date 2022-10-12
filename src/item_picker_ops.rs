use crate::GImGui;

// inline c_void             DebugStartItemPicker()
pub unsafe fn DebugStartItemPicker() {
    let g = GImGui; // ImGuiContext& g = *GImGui;
    g.DebugItemPickerActive = true;
}
