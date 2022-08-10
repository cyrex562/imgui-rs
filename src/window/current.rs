use crate::Context;
use crate::globals::GImGui;
use crate::kv_store::Storage;
use crate::orig_imgui_single_file::Id32;
use crate::types::Id32;
use crate::vectors::vector_2d::Vector2D;

// void SetStateStorage(ImGuiStorage* tree)
pub fn set_state_storage(g: &mut Context, tree: &Storage)
{
    let window = g.current_window_mut();;
    window.dc.StateStorage = tree ? tree : &window.StateStorage;
}

// ImGuiStorage* GetStateStorage()
pub fn get_state_storage(g: &mut Context) -> &mut Storage
{
    let window = g.current_window_mut();;
    return window.dc.StateStorage;
}

// void push_id(const char* str_id)
pub fn push_id(g: &mut Context, str_id: &str)
{
    // ImGuiContext& g = *GImGui;
    let window = g.current_window_mut();
    Id32 id = window.get_id(str_id);
    window.id_stack.push_back(id);
}

// void push_id(const char* str_id_begin, const char* str_id_end)
pub fn push_id2(g: &mut Context, str_id_begin: &str, )
{
    // ImGuiContext& g = *GImGui;
    let window = g.current_window_mut();
    Id32 id = window.get_id(str_id_begin, str_id_end);
    window.id_stack.push_back(id);
}

// void push_id(const void* ptr_id)
 fn push_id3(g: &mut Context, ptr_id: Id32)
{
    // ImGuiContext& g = *GImGui;
    let window = g.current_window_mut();
    Id32 id = window.get_id(ptr_id);
    window.id_stack.push_back(id);
}

// void push_id(int int_id)
pub fn push_id4(g: &mut Context, int_id: Id32)
{
    // ImGuiContext& g = *GImGui;
    let window = g.current_window_mut();
    Id32 id = window.get_id(int_id);
    window.id_stack.push_back(id);
}

// Push a given id value ignoring the id stack as a seed.
// void PushOverrideID(Id32 id)
pub fn push_override_id(g: &mut Context, id: Id32)
{
    // ImGuiContext& g = *GImGui;
    let window = g.current_window_mut();
    if (g.debug_hook_id_info == id)
        debug_hook_id_info(id, DataType::ID, None, None);
    window.id_stack.push_back(id);
}

// void PopID()
pub fn pop_id(g: &mut Context)
{
    let window = g.current_window_mut();;
    // IM_ASSERT(window.IDStack.size > 1); // Too many PopID(), or could be popping in a wrong/different window?
    window.id_stack.pop_back();
}

// Id32 GetID(const char* str_id)
pub fn get_id(g: &mut Context, str_id: &str) -> Id32
{
    let window = g.current_window_mut();;
    return window.get_id(str_id);
}

// Id32 GetID(const char* str_id_begin, const char* str_id_end)
pub fn get_id2(g: &mut Context, str_id: &str) -> Id32
{
    let window = g.current_window_mut();;
    return window.get_id(str_id_begin, str_id_end);
}

// Id32 GetID(const void* ptr_id)
pub fn get_id3(g: &mut Context, ptr_id: Id32)->Id32
{
    let window = g.current_window_mut();;
    return window.get_id(ptr_id);
}

// bool IsRectVisible(const Vector2D& size)
pub fn is_rect_visible(g: &mut Context, size: &Vector2D) -> bool
{
    let window = g.current_window_mut();;
    return window.clip_rect.Overlaps(Rect(window.dc.cursor_pos, window.dc.cursor_pos + size));
}

// bool IsRectVisible(const Vector2D& rect_min, const Vector2D& rect_max)
pub fn is_rect_visible2(g: &mut Context, rect_min: &Vector2D, rect_max: &Vector2D) -> bool
{
    let window = g.current_window_mut();;
    return window.clip_rect.Overlaps(Rect(rect_min, rect_max));
}
