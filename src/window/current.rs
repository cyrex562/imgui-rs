use crate::Context;
use crate::globals::GImGui;
use crate::kv_store::Storage;
use crate::orig_imgui_single_file::ImGuiID;
use crate::types::Id32;
use crate::vectors::two_d::Vector2D;

// void SetStateStorage(ImGuiStorage* tree)
pub fn set_state_storage(g: &mut Context, tree: &Storage)
{
    ImGuiWindow* window = GImGui.CurrentWindow;
    window.dc.StateStorage = tree ? tree : &window.StateStorage;
}

// ImGuiStorage* GetStateStorage()
pub fn get_state_storage(g: &mut Context) -> &mut Storage
{
    ImGuiWindow* window = GImGui.CurrentWindow;
    return window.dc.StateStorage;
}

// void PushID(const char* str_id)
pub fn push_id(g: &mut Context, str_id: &str)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    ImGuiID id = window.get_id(str_id);
    window.IDStack.push_back(id);
}

// void PushID(const char* str_id_begin, const char* str_id_end)
pub fn push_id2(g: &mut Context, str_id_begin: &str, )
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    ImGuiID id = window.get_id(str_id_begin, str_id_end);
    window.IDStack.push_back(id);
}

// void PushID(const void* ptr_id)
 fn push_id3(g: &mut Context, ptr_id: Id32)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    ImGuiID id = window.get_id(ptr_id);
    window.IDStack.push_back(id);
}

// void PushID(int int_id)
pub fn push_id4(g: &mut Context, int_id: Id32)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    ImGuiID id = window.get_id(int_id);
    window.IDStack.push_back(id);
}

// Push a given id value ignoring the id stack as a seed.
// void PushOverrideID(ImGuiID id)
pub fn push_override_id(g: &mut Context, id: Id32)
{
    ImGuiContext& g = *GImGui;
    ImGuiWindow* window = g.current_window;
    if (g.debug_hook_id_info == id)
        debug_hook_id_info(id, ImGuiDataType_ID, NULL, NULL);
    window.IDStack.push_back(id);
}

// void PopID()
pub fn pop_id(g: &mut Context)
{
    ImGuiWindow* window = GImGui.CurrentWindow;
    IM_ASSERT(window.IDStack.size > 1); // Too many PopID(), or could be popping in a wrong/different window?
    window.IDStack.pop_back();
}

// ImGuiID GetID(const char* str_id)
pub fn get_id(g: &mut Context, str_id: &str) -> Id32
{
    ImGuiWindow* window = GImGui.CurrentWindow;
    return window.get_id(str_id);
}

// ImGuiID GetID(const char* str_id_begin, const char* str_id_end)
pub fn get_id2(g: &mut Context, str_id: &str) -> Id32
{
    ImGuiWindow* window = GImGui.CurrentWindow;
    return window.get_id(str_id_begin, str_id_end);
}

// ImGuiID GetID(const void* ptr_id)
pub fn get_id3(g: &mut Context, ptr_id: Id32)->Id32
{
    ImGuiWindow* window = GImGui.CurrentWindow;
    return window.get_id(ptr_id);
}

// bool IsRectVisible(const Vector2D& size)
pub fn is_rect_visible(g: &mut Context, size: &Vector2D) -> bool
{
    ImGuiWindow* window = GImGui.CurrentWindow;
    return window.clip_rect.Overlaps(Rect(window.dc.cursor_pos, window.dc.cursor_pos + size));
}

// bool IsRectVisible(const Vector2D& rect_min, const Vector2D& rect_max)
pub fn is_rect_visible2(g: &mut Context, rect_min: &Vector2D, rect_max: &Vector2D) -> bool
{
    ImGuiWindow* window = GImGui.CurrentWindow;
    return window.clip_rect.Overlaps(Rect(rect_min, rect_max));
}
